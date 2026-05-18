//! Process-topology, host-lane, restart, and reattach inspection records.
//!
//! This module owns the runtime-side truth model that lets shell, run, debug,
//! notebook, preview, AI-tool, provider, log, diagnostic, and support surfaces
//! explain which host lane produced a visible result. The records keep
//! plain-language host families, fault-domain ownership, restart budget state,
//! crash-loop or quarantine posture, and reattach review decisions together so
//! consumers do not infer current truth from stale output.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Schema version shared by host-lane topology and fault-domain state records.
pub const HOST_TOPOLOGY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for one host-lane record.
pub const HOST_LANE_RECORD_KIND: &str = "host_lane_record";

/// Stable record-kind tag for one inline host-badge group.
pub const HOST_BADGE_GROUP_RECORD_KIND: &str = "host_badge_group_record";

/// Stable record-kind tag for one topology inspector packet.
pub const TOPOLOGY_INSPECTOR_RECORD_KIND: &str = "topology_inspector_record";

/// Stable record-kind tag for one fault-domain restart card.
pub const FAULT_DOMAIN_RESTART_CARD_RECORD_KIND: &str = "fault_domain_restart_card_record";

/// Stable record-kind tag for one reattach review sheet.
pub const REATTACH_REVIEW_SHEET_RECORD_KIND: &str = "reattach_review_sheet_record";

/// Stable record-kind tag for one crash-loop or quarantine banner.
pub const CRASH_LOOP_QUARANTINE_BANNER_RECORD_KIND: &str = "crash_loop_quarantine_banner_record";

/// Stable record-kind tag for one lane-filtered event viewer.
pub const LANE_FILTERED_EVENT_VIEWER_RECORD_KIND: &str = "lane_filtered_event_viewer_record";

/// Plain-language host family visible to users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostLaneFamily {
    /// Local shell service running on the user's device.
    LocalShellService,
    /// Language server, graph worker, or analyzer lane.
    LanguageAnalysisHost,
    /// Isolated extension runtime or sandbox host.
    ExtensionSandboxHost,
    /// Debug adapter, task adapter, or test adapter lane.
    DebugTaskAdapterHost,
    /// Notebook or REPL kernel lane.
    NotebookKernel,
    /// SSH, helper, or remote workspace agent lane.
    RemoteWorkspaceAgent,
    /// Managed control-plane or provider-owned service lane.
    ManagedServiceLane,
}

impl HostLaneFamily {
    /// All host families in canonical display order.
    pub const ALL: [Self; 7] = [
        Self::LocalShellService,
        Self::LanguageAnalysisHost,
        Self::ExtensionSandboxHost,
        Self::DebugTaskAdapterHost,
        Self::NotebookKernel,
        Self::RemoteWorkspaceAgent,
        Self::ManagedServiceLane,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalShellService => "local_shell_service",
            Self::LanguageAnalysisHost => "language_analysis_host",
            Self::ExtensionSandboxHost => "extension_sandbox_host",
            Self::DebugTaskAdapterHost => "debug_task_adapter_host",
            Self::NotebookKernel => "notebook_kernel",
            Self::RemoteWorkspaceAgent => "remote_workspace_agent",
            Self::ManagedServiceLane => "managed_service_lane",
        }
    }

    /// Plain-language label shown in host badges and topology cards.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalShellService => "Local shell service",
            Self::LanguageAnalysisHost => "Language/analysis host",
            Self::ExtensionSandboxHost => "Extension sandbox host",
            Self::DebugTaskAdapterHost => "Debug/task adapter host",
            Self::NotebookKernel => "Notebook kernel",
            Self::RemoteWorkspaceAgent => "Remote workspace agent",
            Self::ManagedServiceLane => "Managed service lane",
        }
    }

    /// True when the lane can launch or resume mutating work.
    pub const fn can_mutate_or_resume_execution(self) -> bool {
        matches!(
            self,
            Self::DebugTaskAdapterHost
                | Self::NotebookKernel
                | Self::RemoteWorkspaceAgent
                | Self::ManagedServiceLane
        )
    }
}

/// Boundary or locality badge attached to one host lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostBoundaryBadgeClass {
    /// Lane runs on the local device.
    Local,
    /// Lane is isolated from the shell process.
    Isolated,
    /// Lane is extension-owned.
    ExtensionOwned,
    /// Lane owns stateful kernel variables or outputs.
    KernelStateful,
    /// Lane can perform execution-facing work.
    ExecutionFacing,
    /// Lane crosses a remote target boundary.
    RemoteBoundary,
    /// Lane is managed or provider-backed.
    ManagedBoundary,
    /// Lane currently exposes stale or partial truth.
    PartialTruth,
}

impl HostBoundaryBadgeClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Isolated => "isolated",
            Self::ExtensionOwned => "extension_owned",
            Self::KernelStateful => "kernel_stateful",
            Self::ExecutionFacing => "execution_facing",
            Self::RemoteBoundary => "remote_boundary",
            Self::ManagedBoundary => "managed_boundary",
            Self::PartialTruth => "partial_truth",
        }
    }

    /// Short visible label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Local => "Local",
            Self::Isolated => "Isolated",
            Self::ExtensionOwned => "Extension-owned",
            Self::KernelStateful => "Kernel state",
            Self::ExecutionFacing => "Execution-facing",
            Self::RemoteBoundary => "Remote boundary",
            Self::ManagedBoundary => "Managed boundary",
            Self::PartialTruth => "Partial truth",
        }
    }
}

/// Current health of a host lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostLaneHealthClass {
    /// Lane is healthy and current.
    Healthy,
    /// Lane is starting or warming.
    Starting,
    /// Lane is restarting or reconnecting.
    Reconnecting,
    /// Lane is available with narrowed capability.
    Degraded,
    /// Lane is serving stale snapshots only.
    StaleSnapshot,
    /// Lane is disconnected.
    Disconnected,
    /// Lane exceeded budget or policy and is quarantined.
    Quarantined,
    /// Lane is in a crash loop.
    CrashLoop,
    /// Lane is disabled.
    Disabled,
}

impl HostLaneHealthClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Starting => "starting",
            Self::Reconnecting => "reconnecting",
            Self::Degraded => "degraded",
            Self::StaleSnapshot => "stale_snapshot",
            Self::Disconnected => "disconnected",
            Self::Quarantined => "quarantined",
            Self::CrashLoop => "crash_loop",
            Self::Disabled => "disabled",
        }
    }

    /// Visible label for cards and badges.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Starting => "starting",
            Self::Reconnecting => "reconnecting",
            Self::Degraded => "degraded",
            Self::StaleSnapshot => "stale snapshot",
            Self::Disconnected => "disconnected",
            Self::Quarantined => "quarantined",
            Self::CrashLoop => "crash loop",
            Self::Disabled => "disabled",
        }
    }

    /// True when surfaces must avoid presenting the lane as fully current.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Healthy)
    }

    /// True when hidden automatic recovery is not allowed.
    pub const fn blocks_healthy_claim(self) -> bool {
        matches!(
            self,
            Self::Disconnected | Self::Quarantined | Self::CrashLoop | Self::Disabled
        )
    }
}

/// Fault-domain class that owns restart and quarantine behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultDomainClass {
    /// Desktop shell interaction core.
    ShellInteractionCore,
    /// Workspace knowledge worker group.
    WorkspaceKnowledgeGroup,
    /// Task, test, debug, terminal, notebook, or provider-run session host.
    SessionExecutionHost,
    /// Extension or external tool host.
    ExtensionOrToolHost,
    /// AI or external-tool broker.
    AiToolBroker,
    /// Remote connector or attach path.
    RemoteConnector,
    /// Policy, entitlement, or verifier helper.
    PolicyVerifierHelper,
}

impl FaultDomainClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellInteractionCore => "shell_interaction_core",
            Self::WorkspaceKnowledgeGroup => "workspace_knowledge_group",
            Self::SessionExecutionHost => "session_execution_host",
            Self::ExtensionOrToolHost => "extension_or_tool_host",
            Self::AiToolBroker => "ai_tool_broker",
            Self::RemoteConnector => "remote_connector",
            Self::PolicyVerifierHelper => "policy_verifier_helper",
        }
    }

    /// Plain-language label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ShellInteractionCore => "Shell interaction core",
            Self::WorkspaceKnowledgeGroup => "Workspace knowledge group",
            Self::SessionExecutionHost => "Session execution host",
            Self::ExtensionOrToolHost => "Extension or tool host",
            Self::AiToolBroker => "AI/tool broker",
            Self::RemoteConnector => "Remote connector",
            Self::PolicyVerifierHelper => "Policy/verifier helper",
        }
    }
}

/// Restart-budget state derived for a fault-domain card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartBudgetStateClass {
    /// Lane is inside its automatic restart budget.
    WithinBudget,
    /// One more counted strike would exhaust the budget.
    BudgetWarning,
    /// Automatic restart budget is exhausted.
    BudgetExhausted,
    /// Lane is quarantined after budget or policy escalation.
    Quarantined,
    /// Lane has no hidden automatic restart budget.
    NoAutomaticRestart,
    /// Lane requires a reattach review before control is current.
    ReattachReviewRequired,
    /// Lane is disabled.
    Disabled,
}

impl RestartBudgetStateClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinBudget => "within_budget",
            Self::BudgetWarning => "budget_warning",
            Self::BudgetExhausted => "budget_exhausted",
            Self::Quarantined => "quarantined",
            Self::NoAutomaticRestart => "no_automatic_restart",
            Self::ReattachReviewRequired => "reattach_review_required",
            Self::Disabled => "disabled",
        }
    }

    /// Visible label for restart-budget cards.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WithinBudget => "within budget",
            Self::BudgetWarning => "budget warning",
            Self::BudgetExhausted => "budget exhausted",
            Self::Quarantined => "quarantined",
            Self::NoAutomaticRestart => "no automatic restart",
            Self::ReattachReviewRequired => "reattach review required",
            Self::Disabled => "disabled",
        }
    }
}

/// Freshness of a visible result bound to a host lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostResultFreshnessClass {
    /// Result is confirmed by the current host.
    Current,
    /// Result is preserved but stale.
    Stale,
    /// Result is intentionally partial.
    PartialTruth,
    /// Result is being rebuilt after restart.
    Rebuilding,
    /// Result is a disconnected snapshot.
    DisconnectedSnapshot,
    /// Result awaits a current host refresh.
    AwaitingRefresh,
}

impl HostResultFreshnessClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::PartialTruth => "partial_truth",
            Self::Rebuilding => "rebuilding",
            Self::DisconnectedSnapshot => "disconnected_snapshot",
            Self::AwaitingRefresh => "awaiting_refresh",
        }
    }

    /// Visible label for surface rows.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::PartialTruth => "partial truth",
            Self::Rebuilding => "rebuilding",
            Self::DisconnectedSnapshot => "disconnected snapshot",
            Self::AwaitingRefresh => "awaiting refresh",
        }
    }

    /// True when the result must not be treated as current.
    pub const fn needs_disclosure(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// Surface where a host badge group is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeSurfaceClass {
    /// Log or event output row.
    Logs,
    /// Run, task, build, or test row.
    Run,
    /// Debug view, frame, variable, or adapter-control surface.
    DebugView,
    /// Notebook output, variable, diagnostic, or kernel status row.
    NotebookOutput,
    /// Preview or browser-backed runtime strip.
    Preview,
    /// AI tool action row.
    AiToolAction,
    /// Provider-backed runtime summary row.
    ProviderRuntimeSummary,
    /// Runtime diagnostic row.
    RuntimeDiagnostic,
    /// Support/export row.
    SupportExport,
}

impl RuntimeSurfaceClass {
    /// Surfaces that must expose inline host badges when host identity matters.
    pub const REQUIRED_INLINE_BADGE_SURFACES: [Self; 7] = [
        Self::Logs,
        Self::Run,
        Self::DebugView,
        Self::NotebookOutput,
        Self::Preview,
        Self::AiToolAction,
        Self::ProviderRuntimeSummary,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Logs => "logs",
            Self::Run => "run",
            Self::DebugView => "debug_view",
            Self::NotebookOutput => "notebook_output",
            Self::Preview => "preview",
            Self::AiToolAction => "ai_tool_action",
            Self::ProviderRuntimeSummary => "provider_runtime_summary",
            Self::RuntimeDiagnostic => "runtime_diagnostic",
            Self::SupportExport => "support_export",
        }
    }

    /// Visible label for the consuming surface.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Logs => "Logs",
            Self::Run => "Run",
            Self::DebugView => "Debug view",
            Self::NotebookOutput => "Notebook output",
            Self::Preview => "Preview",
            Self::AiToolAction => "AI tool action",
            Self::ProviderRuntimeSummary => "Provider runtime summary",
            Self::RuntimeDiagnostic => "Runtime diagnostic",
            Self::SupportExport => "Support export",
        }
    }
}

/// Detail target opened from a host badge or topology card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostDetailOpenTarget {
    /// Open the topology inspector.
    TopologyInspector,
    /// Open the fault-domain restart card.
    FaultDomainCard,
    /// Open the reattach review sheet.
    ReattachReview,
    /// Open support/export detail.
    SupportExport,
}

impl HostDetailOpenTarget {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyInspector => "topology_inspector",
            Self::FaultDomainCard => "fault_domain_card",
            Self::ReattachReview => "reattach_review",
            Self::SupportExport => "support_export",
        }
    }
}

/// Boundary badge with both token and label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundaryBadge {
    /// Badge class.
    pub class: HostBoundaryBadgeClass,
    /// Stable badge token.
    pub token: String,
    /// Visible badge label.
    pub label: String,
}

impl HostBoundaryBadge {
    /// Builds a badge from the closed badge class.
    pub fn from_class(class: HostBoundaryBadgeClass) -> Self {
        Self {
            class,
            token: class.as_str().to_owned(),
            label: class.label().to_owned(),
        }
    }
}

/// Action that opens lane details from an inline badge or card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostDetailAction {
    /// Visible action label.
    pub label: String,
    /// Stable action reference.
    pub action_ref: String,
    /// Detail target opened by the action.
    pub open_target: HostDetailOpenTarget,
    /// Stable detail-target token.
    pub open_target_token: String,
}

impl HostDetailAction {
    /// Builds an action for the given lane and target.
    pub fn for_lane(lane_id: &str, open_target: HostDetailOpenTarget) -> Self {
        let label = match open_target {
            HostDetailOpenTarget::TopologyInspector => "Open topology",
            HostDetailOpenTarget::FaultDomainCard => "Open fault domain",
            HostDetailOpenTarget::ReattachReview => "Review reattach",
            HostDetailOpenTarget::SupportExport => "Open support export",
        };
        Self {
            label: label.to_owned(),
            action_ref: format!("action:host-detail:{lane_id}:{}", open_target.as_str()),
            open_target,
            open_target_token: open_target.as_str().to_owned(),
        }
    }
}

/// Input used to build one [`HostLaneRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostLaneSeed {
    /// Stable lane id.
    pub lane_id: String,
    /// Plain-language host family.
    pub family: HostLaneFamily,
    /// Role label shown in topology detail.
    pub role_label: String,
    /// Host instance label shown in topology detail.
    pub host_label: String,
    /// Opaque target reference when the lane is target-bound.
    pub target_ref: Option<String>,
    /// Locality summary for badge groups.
    pub locality_label: String,
    /// Boundary badges attached to the lane.
    pub boundary_badge_classes: Vec<HostBoundaryBadgeClass>,
    /// Current host health.
    pub health_class: HostLaneHealthClass,
    /// Fault-domain class that owns the lane.
    pub fault_domain_class: FaultDomainClass,
    /// Stable fault-domain id.
    pub fault_domain_id: String,
    /// Restart-budget reference.
    pub restart_budget_ref: String,
    /// Counted strikes in the current window.
    pub restart_strike_count: u32,
    /// Automatic restart budget in the current window.
    pub restart_budget_in_window: u32,
    /// User-visible capabilities affected by the current state.
    pub affected_capability_tokens: Vec<String>,
    /// Checkpoints preserved across restart or reattach.
    pub preserved_checkpoint_refs: Vec<String>,
    /// Visible results that are stale or partial because of this lane.
    pub stale_result_refs: Vec<String>,
    /// Quarantine or crash-loop evidence reference.
    pub quarantine_ref: Option<String>,
    /// Whether the current host has confirmed this lane's truth.
    pub current_host_confirmed: bool,
}

/// One host lane in the runtime topology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostLaneRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable lane id.
    pub lane_id: String,
    /// Plain-language host family.
    pub family: HostLaneFamily,
    /// Stable host-family token.
    pub family_token: String,
    /// Plain-language host-family label.
    pub family_label: String,
    /// Role label shown in topology detail.
    pub role_label: String,
    /// Host instance label shown in topology detail.
    pub host_label: String,
    /// Opaque target reference when the lane is target-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<String>,
    /// Locality summary for badge groups.
    pub locality_label: String,
    /// Boundary badges attached to the lane.
    pub boundary_badges: Vec<HostBoundaryBadge>,
    /// Current host health.
    pub health_class: HostLaneHealthClass,
    /// Stable health token.
    pub health_token: String,
    /// Visible health label.
    pub health_label: String,
    /// Fault-domain class that owns the lane.
    pub fault_domain_class: FaultDomainClass,
    /// Stable fault-domain class token.
    pub fault_domain_token: String,
    /// Plain-language fault-domain class label.
    pub fault_domain_label: String,
    /// Stable fault-domain id.
    pub fault_domain_id: String,
    /// Restart-budget reference.
    pub restart_budget_ref: String,
    /// Counted strikes in the current window.
    pub restart_strike_count: u32,
    /// Automatic restart budget in the current window.
    pub restart_budget_in_window: u32,
    /// User-visible capabilities affected by the current state.
    pub affected_capability_tokens: Vec<String>,
    /// Checkpoints preserved across restart or reattach.
    pub preserved_checkpoint_refs: Vec<String>,
    /// Visible results that are stale or partial because of this lane.
    pub stale_result_refs: Vec<String>,
    /// Quarantine or crash-loop evidence reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quarantine_ref: Option<String>,
    /// Whether the current host has confirmed this lane's truth.
    pub current_host_confirmed: bool,
    /// Detail action opened from this lane.
    pub detail_action: HostDetailAction,
}

impl HostLaneRecord {
    /// Builds one lane record from seed data.
    pub fn from_seed(seed: HostLaneSeed) -> Self {
        let mut boundary_badges = seed
            .boundary_badge_classes
            .into_iter()
            .map(HostBoundaryBadge::from_class)
            .collect::<Vec<_>>();
        if seed.health_class.requires_disclosure()
            && !boundary_badges
                .iter()
                .any(|badge| badge.class == HostBoundaryBadgeClass::PartialTruth)
        {
            boundary_badges.push(HostBoundaryBadge::from_class(
                HostBoundaryBadgeClass::PartialTruth,
            ));
        }
        Self {
            record_kind: HOST_LANE_RECORD_KIND.to_owned(),
            schema_version: HOST_TOPOLOGY_SCHEMA_VERSION,
            lane_id: seed.lane_id.clone(),
            family: seed.family,
            family_token: seed.family.as_str().to_owned(),
            family_label: seed.family.label().to_owned(),
            role_label: seed.role_label,
            host_label: seed.host_label,
            target_ref: seed.target_ref,
            locality_label: seed.locality_label,
            boundary_badges,
            health_class: seed.health_class,
            health_token: seed.health_class.as_str().to_owned(),
            health_label: seed.health_class.label().to_owned(),
            fault_domain_class: seed.fault_domain_class,
            fault_domain_token: seed.fault_domain_class.as_str().to_owned(),
            fault_domain_label: seed.fault_domain_class.label().to_owned(),
            fault_domain_id: seed.fault_domain_id,
            restart_budget_ref: seed.restart_budget_ref,
            restart_strike_count: seed.restart_strike_count,
            restart_budget_in_window: seed.restart_budget_in_window,
            affected_capability_tokens: seed.affected_capability_tokens,
            preserved_checkpoint_refs: seed.preserved_checkpoint_refs,
            stale_result_refs: seed.stale_result_refs,
            quarantine_ref: seed.quarantine_ref,
            current_host_confirmed: seed.current_host_confirmed,
            detail_action: HostDetailAction::for_lane(
                &seed.lane_id,
                HostDetailOpenTarget::TopologyInspector,
            ),
        }
    }

    /// Returns true when the lane state must be visible inline.
    pub fn requires_disclosure(&self) -> bool {
        self.health_class.requires_disclosure()
            || !self.current_host_confirmed
            || !self.stale_result_refs.is_empty()
    }

    /// Returns the host fingerprint token used by reattach reviews.
    pub fn host_fingerprint_token(&self) -> String {
        format!(
            "{}:{}:{}",
            self.family_token,
            self.lane_id,
            self.target_ref.as_deref().unwrap_or("no_target")
        )
    }

    /// Derives the current restart-budget state.
    pub fn restart_budget_state(&self) -> RestartBudgetStateClass {
        match self.health_class {
            HostLaneHealthClass::Disabled => RestartBudgetStateClass::Disabled,
            HostLaneHealthClass::Quarantined | HostLaneHealthClass::CrashLoop => {
                RestartBudgetStateClass::Quarantined
            }
            HostLaneHealthClass::Disconnected if self.family.can_mutate_or_resume_execution() => {
                RestartBudgetStateClass::ReattachReviewRequired
            }
            _ if self.restart_budget_in_window == 0 => RestartBudgetStateClass::NoAutomaticRestart,
            _ if self.restart_strike_count >= self.restart_budget_in_window => {
                RestartBudgetStateClass::BudgetExhausted
            }
            _ if self.restart_strike_count + 1 >= self.restart_budget_in_window => {
                RestartBudgetStateClass::BudgetWarning
            }
            _ => RestartBudgetStateClass::WithinBudget,
        }
    }
}

/// Input used to bind a visible result to one or more host lanes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeResultSeed {
    /// Stable result id.
    pub result_id: String,
    /// Surface rendering the result.
    pub surface: RuntimeSurfaceClass,
    /// Export-safe result summary.
    pub result_summary: String,
    /// Host lanes that produced or currently own the result.
    pub host_lane_ids: Vec<String>,
    /// Freshness state of the result.
    pub freshness_class: HostResultFreshnessClass,
    /// Optional provenance or run reference.
    pub provenance_ref: Option<String>,
    /// Whether reattach review is required before this result can be current.
    pub requires_reattach_review: bool,
}

/// Inline badge group rendered beside a user-visible result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBadgeGroup {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Result id this badge group annotates.
    pub result_ref: String,
    /// Surface rendering the result.
    pub surface: RuntimeSurfaceClass,
    /// Stable surface token.
    pub surface_token: String,
    /// Host lane id.
    pub host_lane_ref: String,
    /// Plain-language primary badge label.
    pub primary_badge_label: String,
    /// Current state badge label.
    pub state_badge_label: String,
    /// Optional restart or stale-truth note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_note: Option<String>,
    /// Boundary badges copied from the host lane.
    pub boundary_badges: Vec<HostBoundaryBadge>,
    /// Detail action opened from the badge group.
    pub detail_action: HostDetailAction,
}

impl HostBadgeGroup {
    /// Builds a badge group for one result and lane.
    pub fn from_lane(
        result_ref: &str,
        surface: RuntimeSurfaceClass,
        lane: &HostLaneRecord,
    ) -> Self {
        let restart_note = if lane.restart_strike_count > 0 || lane.requires_disclosure() {
            Some(format!(
                "{}; restart budget {}/{}",
                lane.health_label, lane.restart_strike_count, lane.restart_budget_in_window
            ))
        } else {
            None
        };
        let open_target = if lane.health_class.blocks_healthy_claim() {
            HostDetailOpenTarget::FaultDomainCard
        } else {
            HostDetailOpenTarget::TopologyInspector
        };
        Self {
            record_kind: HOST_BADGE_GROUP_RECORD_KIND.to_owned(),
            schema_version: HOST_TOPOLOGY_SCHEMA_VERSION,
            result_ref: result_ref.to_owned(),
            surface,
            surface_token: surface.as_str().to_owned(),
            host_lane_ref: lane.lane_id.clone(),
            primary_badge_label: lane.family_label.clone(),
            state_badge_label: lane.health_label.clone(),
            restart_note,
            boundary_badges: lane.boundary_badges.clone(),
            detail_action: HostDetailAction::for_lane(&lane.lane_id, open_target),
        }
    }
}

/// One visible result bound to host-lane truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeSurfaceResult {
    /// Stable result id.
    pub result_id: String,
    /// Surface rendering the result.
    pub surface: RuntimeSurfaceClass,
    /// Stable surface token.
    pub surface_token: String,
    /// Export-safe result summary.
    pub result_summary: String,
    /// Host lanes that produced or currently own the result.
    pub host_lane_ids: Vec<String>,
    /// Inline badge groups for the result.
    pub host_badge_groups: Vec<HostBadgeGroup>,
    /// Freshness state of the result.
    pub freshness_class: HostResultFreshnessClass,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Visible freshness label.
    pub freshness_label: String,
    /// Optional provenance or run reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance_ref: Option<String>,
    /// Whether a current host has confirmed this result.
    pub current_host_confirmed: bool,
    /// Whether reattach review is required before this result can be current.
    pub requires_reattach_review: bool,
}

impl RuntimeSurfaceResult {
    fn from_seed(seed: RuntimeResultSeed, lanes_by_id: &BTreeMap<String, HostLaneRecord>) -> Self {
        let host_badge_groups = seed
            .host_lane_ids
            .iter()
            .filter_map(|lane_id| lanes_by_id.get(lane_id))
            .map(|lane| HostBadgeGroup::from_lane(&seed.result_id, seed.surface, lane))
            .collect::<Vec<_>>();
        let current_host_confirmed = !seed.freshness_class.needs_disclosure()
            && !seed.requires_reattach_review
            && seed
                .host_lane_ids
                .iter()
                .filter_map(|lane_id| lanes_by_id.get(lane_id))
                .all(|lane| lane.current_host_confirmed && !lane.requires_disclosure());
        Self {
            result_id: seed.result_id,
            surface: seed.surface,
            surface_token: seed.surface.as_str().to_owned(),
            result_summary: seed.result_summary,
            host_lane_ids: seed.host_lane_ids,
            host_badge_groups,
            freshness_class: seed.freshness_class,
            freshness_token: seed.freshness_class.as_str().to_owned(),
            freshness_label: seed.freshness_class.label().to_owned(),
            provenance_ref: seed.provenance_ref,
            current_host_confirmed,
            requires_reattach_review: seed.requires_reattach_review,
        }
    }
}

/// Topology inspector mapping visible results back to host lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyInspectorRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable inspector id.
    pub inspector_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Host lanes in the workspace.
    pub lanes: Vec<HostLaneRecord>,
    /// Visible result rows bound to lanes.
    pub results: Vec<RuntimeSurfaceResult>,
    /// Count of healthy lanes.
    pub healthy_lane_count: u32,
    /// Count of lanes that need disclosure.
    pub degraded_lane_count: u32,
    /// Count of stale, partial, or awaiting-refresh results.
    pub partial_truth_result_count: u32,
    /// Export-safe summary.
    pub summary: String,
}

impl TopologyInspectorRecord {
    /// Builds an inspector from lane records and visible result seeds.
    pub fn from_lanes_and_results(
        inspector_id: impl Into<String>,
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        lanes: Vec<HostLaneRecord>,
        result_seeds: Vec<RuntimeResultSeed>,
    ) -> Self {
        let lanes_by_id = lanes
            .iter()
            .map(|lane| (lane.lane_id.clone(), lane.clone()))
            .collect::<BTreeMap<_, _>>();
        let results = result_seeds
            .into_iter()
            .map(|seed| RuntimeSurfaceResult::from_seed(seed, &lanes_by_id))
            .collect::<Vec<_>>();
        let healthy_lane_count = lanes
            .iter()
            .filter(|lane| lane.health_class == HostLaneHealthClass::Healthy)
            .count() as u32;
        let degraded_lane_count = lanes
            .iter()
            .filter(|lane| lane.requires_disclosure())
            .count() as u32;
        let partial_truth_result_count = results
            .iter()
            .filter(|result| result.freshness_class.needs_disclosure())
            .count() as u32;
        Self {
            record_kind: TOPOLOGY_INSPECTOR_RECORD_KIND.to_owned(),
            schema_version: HOST_TOPOLOGY_SCHEMA_VERSION,
            inspector_id: inspector_id.into(),
            workspace_id: workspace_id.into(),
            generated_at: generated_at.into(),
            lanes,
            results,
            healthy_lane_count,
            degraded_lane_count,
            partial_truth_result_count,
            summary: "Host topology keeps lane health, fault domains, and partial truth visible."
                .to_owned(),
        }
    }

    /// Returns the host lane with the given id.
    pub fn lane(&self, lane_id: &str) -> Option<&HostLaneRecord> {
        self.lanes.iter().find(|lane| lane.lane_id == lane_id)
    }

    /// Returns all result rows on a surface.
    pub fn results_for_surface(&self, surface: RuntimeSurfaceClass) -> Vec<&RuntimeSurfaceResult> {
        self.results
            .iter()
            .filter(|result| result.surface == surface)
            .collect()
    }

    /// Returns required inline-badge surface tokens absent from this inspector.
    pub fn missing_required_surface_tokens(&self) -> Vec<String> {
        RuntimeSurfaceClass::REQUIRED_INLINE_BADGE_SURFACES
            .into_iter()
            .filter(|surface| {
                !self.results.iter().any(|result| {
                    result.surface == *surface && !result.host_badge_groups.is_empty()
                })
            })
            .map(|surface| surface.as_str().to_owned())
            .collect()
    }

    /// Validates cross references and current-truth disclosure behavior.
    pub fn validate(&self) -> Vec<TopologyInspectorViolation> {
        let mut violations = Vec::new();
        if self.record_kind != TOPOLOGY_INSPECTOR_RECORD_KIND {
            push_topology_violation(
                &mut violations,
                "record_kind",
                &self.inspector_id,
                "record_kind must be topology_inspector_record",
            );
        }
        if self.schema_version != HOST_TOPOLOGY_SCHEMA_VERSION {
            push_topology_violation(
                &mut violations,
                "schema_version",
                &self.inspector_id,
                "schema_version must be 1",
            );
        }

        let lane_ids = self
            .lanes
            .iter()
            .map(|lane| lane.lane_id.as_str())
            .collect::<BTreeSet<_>>();
        for lane in &self.lanes {
            if lane.record_kind != HOST_LANE_RECORD_KIND {
                push_topology_violation(
                    &mut violations,
                    "lanes.record_kind",
                    &lane.lane_id,
                    "lane record_kind must be host_lane_record",
                );
            }
            if lane.family_label != lane.family.label() {
                push_topology_violation(
                    &mut violations,
                    "lanes.family_label",
                    &lane.lane_id,
                    "host family label must stay plain-language and canonical",
                );
            }
            if lane.fault_domain_id.is_empty() || lane.restart_budget_ref.is_empty() {
                push_topology_violation(
                    &mut violations,
                    "lanes.fault_domain",
                    &lane.lane_id,
                    "lane must name a fault domain and restart budget ref",
                );
            }
        }

        for result in &self.results {
            if result.host_lane_ids.is_empty() {
                push_topology_violation(
                    &mut violations,
                    "results.host_lane_ids",
                    &result.result_id,
                    "visible result must name at least one host lane",
                );
            }
            if result.host_badge_groups.is_empty() {
                push_topology_violation(
                    &mut violations,
                    "results.host_badge_groups",
                    &result.result_id,
                    "visible result must carry inline host badges",
                );
            }
            for lane_id in &result.host_lane_ids {
                if !lane_ids.contains(lane_id.as_str()) {
                    push_topology_violation(
                        &mut violations,
                        "results.host_lane_ids",
                        &result.result_id,
                        format!("result references unknown host lane {lane_id}"),
                    );
                }
            }
            if result.requires_reattach_review && result.current_host_confirmed {
                push_topology_violation(
                    &mut violations,
                    "results.current_host_confirmed",
                    &result.result_id,
                    "reattach-review results cannot claim current host confirmation",
                );
            }
        }

        for missing in self.missing_required_surface_tokens() {
            push_topology_violation(
                &mut violations,
                "results.surface_coverage",
                &self.inspector_id,
                format!("missing required host-badge surface {missing}"),
            );
        }
        violations
    }
}

/// Validation issue emitted by topology inspector checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyInspectorViolation {
    /// Field or path that failed validation.
    pub path: String,
    /// Subject record reference.
    pub subject_ref: String,
    /// Export-safe validation summary.
    pub summary: String,
}

fn push_topology_violation(
    violations: &mut Vec<TopologyInspectorViolation>,
    path: impl Into<String>,
    subject_ref: impl Into<String>,
    summary: impl Into<String>,
) {
    violations.push(TopologyInspectorViolation {
        path: path.into(),
        subject_ref: subject_ref.into(),
        summary: summary.into(),
    });
}

/// Safe action class shown on restart cards and crash banners.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultDomainNextSafeActionClass {
    /// Open topology detail.
    OpenTopology,
    /// Continue using the lane with limited capability.
    ContinueLimited,
    /// Refresh non-mutating analysis.
    RefreshAnalysis,
    /// Review reattach before resuming control.
    ReviewReattach,
    /// Explicitly rerun the captured action.
    ExplicitRerun,
    /// Restart the lane in isolated mode.
    RestartIsolated,
    /// Export evidence for support or incident review.
    ExportEvidence,
    /// Route to manual repair.
    ManualRepair,
}

impl FaultDomainNextSafeActionClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenTopology => "open_topology",
            Self::ContinueLimited => "continue_limited",
            Self::RefreshAnalysis => "refresh_analysis",
            Self::ReviewReattach => "review_reattach",
            Self::ExplicitRerun => "explicit_rerun",
            Self::RestartIsolated => "restart_isolated",
            Self::ExportEvidence => "export_evidence",
            Self::ManualRepair => "manual_repair",
        }
    }

    /// Visible action label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenTopology => "Open topology",
            Self::ContinueLimited => "Continue limited",
            Self::RefreshAnalysis => "Refresh analysis",
            Self::ReviewReattach => "Review reattach",
            Self::ExplicitRerun => "Rerun explicitly",
            Self::RestartIsolated => "Restart isolated",
            Self::ExportEvidence => "Export evidence",
            Self::ManualRepair => "Manual repair",
        }
    }
}

/// Fault-domain and restart-budget card for one host lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultDomainRestartCard {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable card id.
    pub card_id: String,
    /// Host lane id.
    pub host_lane_ref: String,
    /// Plain-language host family label.
    pub host_family_label: String,
    /// Fault-domain id.
    pub fault_domain_id: String,
    /// Fault-domain class token.
    pub fault_domain_token: String,
    /// Restart-budget ref.
    pub restart_budget_ref: String,
    /// Current strike count.
    pub restart_strike_count: u32,
    /// Restart budget in the active window.
    pub restart_budget_in_window: u32,
    /// Derived restart-budget state.
    pub restart_budget_state: RestartBudgetStateClass,
    /// Stable restart-budget state token.
    pub restart_budget_state_token: String,
    /// Visible restart-budget state label.
    pub restart_budget_state_label: String,
    /// Capabilities affected by the current state.
    pub affected_capability_tokens: Vec<String>,
    /// Preserved checkpoints.
    pub preserved_checkpoint_refs: Vec<String>,
    /// Safe next action classes.
    pub next_safe_actions: Vec<FaultDomainNextSafeActionClass>,
    /// Stable safe-action tokens.
    pub next_safe_action_tokens: Vec<String>,
}

impl FaultDomainRestartCard {
    /// Builds a fault-domain card from a host lane.
    pub fn from_lane(card_id: impl Into<String>, lane: &HostLaneRecord) -> Self {
        let restart_budget_state = lane.restart_budget_state();
        let next_safe_actions = derive_next_safe_actions(lane);
        let next_safe_action_tokens = next_safe_actions
            .iter()
            .map(|action| action.as_str().to_owned())
            .collect();
        Self {
            record_kind: FAULT_DOMAIN_RESTART_CARD_RECORD_KIND.to_owned(),
            schema_version: HOST_TOPOLOGY_SCHEMA_VERSION,
            card_id: card_id.into(),
            host_lane_ref: lane.lane_id.clone(),
            host_family_label: lane.family_label.clone(),
            fault_domain_id: lane.fault_domain_id.clone(),
            fault_domain_token: lane.fault_domain_token.clone(),
            restart_budget_ref: lane.restart_budget_ref.clone(),
            restart_strike_count: lane.restart_strike_count,
            restart_budget_in_window: lane.restart_budget_in_window,
            restart_budget_state,
            restart_budget_state_token: restart_budget_state.as_str().to_owned(),
            restart_budget_state_label: restart_budget_state.label().to_owned(),
            affected_capability_tokens: lane.affected_capability_tokens.clone(),
            preserved_checkpoint_refs: lane.preserved_checkpoint_refs.clone(),
            next_safe_actions,
            next_safe_action_tokens,
        }
    }

    /// True when the card blocks a hidden healthy claim.
    pub fn blocks_healthy_claim(&self) -> bool {
        matches!(
            self.restart_budget_state,
            RestartBudgetStateClass::BudgetExhausted
                | RestartBudgetStateClass::Quarantined
                | RestartBudgetStateClass::ReattachReviewRequired
                | RestartBudgetStateClass::Disabled
        )
    }
}

fn derive_next_safe_actions(lane: &HostLaneRecord) -> Vec<FaultDomainNextSafeActionClass> {
    match lane.health_class {
        HostLaneHealthClass::Healthy => vec![FaultDomainNextSafeActionClass::OpenTopology],
        HostLaneHealthClass::Starting
        | HostLaneHealthClass::Reconnecting
        | HostLaneHealthClass::Degraded
        | HostLaneHealthClass::StaleSnapshot
            if lane.family == HostLaneFamily::LanguageAnalysisHost =>
        {
            vec![
                FaultDomainNextSafeActionClass::ContinueLimited,
                FaultDomainNextSafeActionClass::RefreshAnalysis,
                FaultDomainNextSafeActionClass::OpenTopology,
            ]
        }
        HostLaneHealthClass::Quarantined
        | HostLaneHealthClass::CrashLoop
        | HostLaneHealthClass::Disabled => vec![
            FaultDomainNextSafeActionClass::ExportEvidence,
            FaultDomainNextSafeActionClass::RestartIsolated,
            FaultDomainNextSafeActionClass::ManualRepair,
        ],
        HostLaneHealthClass::Disconnected
        | HostLaneHealthClass::Reconnecting
        | HostLaneHealthClass::Degraded
        | HostLaneHealthClass::StaleSnapshot => vec![
            FaultDomainNextSafeActionClass::ReviewReattach,
            FaultDomainNextSafeActionClass::ExplicitRerun,
            FaultDomainNextSafeActionClass::ExportEvidence,
        ],
        HostLaneHealthClass::Starting => vec![FaultDomainNextSafeActionClass::OpenTopology],
    }
}

/// Replay risk carried by a reattach review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReattachReplayRiskClass {
    /// No replay is involved.
    NoReplay,
    /// Replay is read-only and idempotent.
    ReadOnly,
    /// Replay may mutate local state.
    Mutating,
    /// Replay crosses a privileged or external authority boundary.
    Privileged,
    /// Replay risk is unknown and requires review.
    UnknownRequiresReview,
}

impl ReattachReplayRiskClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoReplay => "no_replay",
            Self::ReadOnly => "read_only",
            Self::Mutating => "mutating",
            Self::Privileged => "privileged",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// True when the old work must not be silently rerun.
    pub const fn forbids_silent_rerun(self) -> bool {
        matches!(
            self,
            Self::Mutating | Self::Privileged | Self::UnknownRequiresReview
        )
    }
}

/// Rerun or review requirement derived during reattach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunRequirementClass {
    /// No rerun is required.
    NotRequired,
    /// Refresh is required but replay is not.
    RefreshRequired,
    /// Explicit rerun is required.
    ExplicitRerunRequired,
    /// Reapproval is required before rerun.
    ReapprovalRequired,
    /// Manual repair is required.
    ManualRepairRequired,
}

impl RerunRequirementClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::RefreshRequired => "refresh_required",
            Self::ExplicitRerunRequired => "explicit_rerun_required",
            Self::ReapprovalRequired => "reapproval_required",
            Self::ManualRepairRequired => "manual_repair_required",
        }
    }
}

/// Decision produced by a reattach review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReattachReviewDecisionClass {
    /// Current host may be treated as live.
    Current,
    /// Non-mutating lane auto-reattached but stale results must refresh.
    AutoReattachedStaleRefresh,
    /// Review is required before current truth can be claimed.
    ReviewRequired,
    /// Reapproval is required before current truth can be claimed.
    ReapprovalRequired,
    /// Explicit rerun is required.
    RerunRequired,
    /// Manual repair blocks reattach.
    BlockedManualRepair,
}

impl ReattachReviewDecisionClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::AutoReattachedStaleRefresh => "auto_reattached_stale_refresh",
            Self::ReviewRequired => "review_required",
            Self::ReapprovalRequired => "reapproval_required",
            Self::RerunRequired => "rerun_required",
            Self::BlockedManualRepair => "blocked_manual_repair",
        }
    }

    /// True when the surface may claim the lane is current.
    pub const fn allows_current_claim(self) -> bool {
        matches!(self, Self::Current | Self::AutoReattachedStaleRefresh)
    }
}

/// Field compared during reattach review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReattachDriftFieldClass {
    /// Host family changed.
    HostFamily,
    /// Host lane id changed.
    LaneId,
    /// Target reference changed.
    TargetRef,
    /// Host health changed.
    Health,
    /// Policy epoch changed.
    PolicyEpoch,
    /// Auth scope changed.
    AuthScope,
}

impl ReattachDriftFieldClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostFamily => "host_family",
            Self::LaneId => "lane_id",
            Self::TargetRef => "target_ref",
            Self::Health => "health",
            Self::PolicyEpoch => "policy_epoch",
            Self::AuthScope => "auth_scope",
        }
    }
}

/// One changed field in a reattach review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReattachDriftRow {
    /// Compared field.
    pub field: ReattachDriftFieldClass,
    /// Stable field token.
    pub field_token: String,
    /// Previous value token.
    pub previous_value_token: String,
    /// Current value token.
    pub current_value_token: String,
    /// Whether this field blocks silent continuation.
    pub requires_review: bool,
}

/// Input values for reattach review comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReattachReviewInput {
    /// Stable review id.
    pub review_id: String,
    /// Previous policy epoch.
    pub previous_policy_epoch: u64,
    /// Current policy epoch.
    pub current_policy_epoch: u64,
    /// Previous auth scope reference.
    pub previous_auth_scope_ref: String,
    /// Current auth scope reference.
    pub current_auth_scope_ref: String,
    /// Preserved state references.
    pub preserved_state_refs: Vec<String>,
    /// Lost state references.
    pub lost_state_refs: Vec<String>,
    /// Replay risk.
    pub replay_risk_class: ReattachReplayRiskClass,
    /// Rerun requirement.
    pub rerun_requirement_class: RerunRequirementClass,
    /// Review timestamp.
    pub reviewed_at: String,
}

/// Review sheet comparing previous and current host identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReattachReviewSheet {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Previous host lane id.
    pub previous_host_lane_ref: String,
    /// Current host lane id.
    pub current_host_lane_ref: String,
    /// Previous host family label.
    pub previous_host_family_label: String,
    /// Current host family label.
    pub current_host_family_label: String,
    /// Previous host fingerprint token.
    pub previous_host_fingerprint_token: String,
    /// Current host fingerprint token.
    pub current_host_fingerprint_token: String,
    /// Drift rows detected by the review.
    pub drift_rows: Vec<ReattachDriftRow>,
    /// Preserved state references.
    pub preserved_state_refs: Vec<String>,
    /// Lost state references.
    pub lost_state_refs: Vec<String>,
    /// Replay risk.
    pub replay_risk_class: ReattachReplayRiskClass,
    /// Stable replay-risk token.
    pub replay_risk_token: String,
    /// Rerun requirement.
    pub rerun_requirement_class: RerunRequirementClass,
    /// Stable rerun-requirement token.
    pub rerun_requirement_token: String,
    /// Whether policy epoch changed.
    pub policy_drift_present: bool,
    /// Whether auth scope changed.
    pub auth_drift_present: bool,
    /// Review decision.
    pub decision: ReattachReviewDecisionClass,
    /// Stable decision token.
    pub decision_token: String,
    /// Whether the current host may be claimed as current.
    pub current_lane_accepted: bool,
    /// Review timestamp.
    pub reviewed_at: String,
}

impl ReattachReviewSheet {
    /// Compares previous and current lane identity and derives a decision.
    pub fn compare(
        previous: &HostLaneRecord,
        current: &HostLaneRecord,
        input: ReattachReviewInput,
    ) -> Self {
        let mut drift_rows = Vec::new();
        push_reattach_drift(
            &mut drift_rows,
            ReattachDriftFieldClass::HostFamily,
            previous.family_token.clone(),
            current.family_token.clone(),
            true,
        );
        push_reattach_drift(
            &mut drift_rows,
            ReattachDriftFieldClass::LaneId,
            previous.lane_id.clone(),
            current.lane_id.clone(),
            true,
        );
        push_reattach_drift(
            &mut drift_rows,
            ReattachDriftFieldClass::TargetRef,
            previous.target_ref.clone().unwrap_or_else(|| "none".into()),
            current.target_ref.clone().unwrap_or_else(|| "none".into()),
            true,
        );
        push_reattach_drift(
            &mut drift_rows,
            ReattachDriftFieldClass::Health,
            previous.health_token.clone(),
            current.health_token.clone(),
            current.health_class.requires_disclosure(),
        );
        push_reattach_drift(
            &mut drift_rows,
            ReattachDriftFieldClass::PolicyEpoch,
            input.previous_policy_epoch.to_string(),
            input.current_policy_epoch.to_string(),
            true,
        );
        push_reattach_drift(
            &mut drift_rows,
            ReattachDriftFieldClass::AuthScope,
            input.previous_auth_scope_ref.clone(),
            input.current_auth_scope_ref.clone(),
            true,
        );
        drift_rows.retain(|row| row.previous_value_token != row.current_value_token);

        let policy_drift_present = drift_rows
            .iter()
            .any(|row| row.field == ReattachDriftFieldClass::PolicyEpoch);
        let auth_drift_present = drift_rows
            .iter()
            .any(|row| row.field == ReattachDriftFieldClass::AuthScope);

        let decision =
            if input.rerun_requirement_class == RerunRequirementClass::ManualRepairRequired {
                ReattachReviewDecisionClass::BlockedManualRepair
            } else if input.rerun_requirement_class == RerunRequirementClass::ReapprovalRequired
                || policy_drift_present
                || auth_drift_present
            {
                ReattachReviewDecisionClass::ReapprovalRequired
            } else if input.replay_risk_class.forbids_silent_rerun()
                || input.rerun_requirement_class == RerunRequirementClass::ExplicitRerunRequired
            {
                ReattachReviewDecisionClass::RerunRequired
            } else if previous.host_fingerprint_token() == current.host_fingerprint_token()
                && !current.health_class.requires_disclosure()
                && drift_rows.is_empty()
            {
                ReattachReviewDecisionClass::Current
            } else if current.family == HostLaneFamily::LanguageAnalysisHost
                && matches!(
                    current.health_class,
                    HostLaneHealthClass::Reconnecting
                        | HostLaneHealthClass::Degraded
                        | HostLaneHealthClass::StaleSnapshot
                )
            {
                ReattachReviewDecisionClass::AutoReattachedStaleRefresh
            } else {
                ReattachReviewDecisionClass::ReviewRequired
            };

        Self {
            record_kind: REATTACH_REVIEW_SHEET_RECORD_KIND.to_owned(),
            schema_version: HOST_TOPOLOGY_SCHEMA_VERSION,
            review_id: input.review_id,
            previous_host_lane_ref: previous.lane_id.clone(),
            current_host_lane_ref: current.lane_id.clone(),
            previous_host_family_label: previous.family_label.clone(),
            current_host_family_label: current.family_label.clone(),
            previous_host_fingerprint_token: previous.host_fingerprint_token(),
            current_host_fingerprint_token: current.host_fingerprint_token(),
            drift_rows,
            preserved_state_refs: input.preserved_state_refs,
            lost_state_refs: input.lost_state_refs,
            replay_risk_class: input.replay_risk_class,
            replay_risk_token: input.replay_risk_class.as_str().to_owned(),
            rerun_requirement_class: input.rerun_requirement_class,
            rerun_requirement_token: input.rerun_requirement_class.as_str().to_owned(),
            policy_drift_present,
            auth_drift_present,
            decision,
            decision_token: decision.as_str().to_owned(),
            current_lane_accepted: decision.allows_current_claim(),
            reviewed_at: input.reviewed_at,
        }
    }
}

fn push_reattach_drift(
    drift_rows: &mut Vec<ReattachDriftRow>,
    field: ReattachDriftFieldClass,
    previous_value_token: String,
    current_value_token: String,
    requires_review: bool,
) {
    drift_rows.push(ReattachDriftRow {
        field,
        field_token: field.as_str().to_owned(),
        previous_value_token,
        current_value_token,
        requires_review,
    });
}

/// Banner that prevents crash-loop or quarantine state from looking healthy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopQuarantineBanner {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable banner id.
    pub banner_id: String,
    /// Failing host lane id.
    pub failing_host_lane_ref: String,
    /// Plain-language failing host label.
    pub failing_host_label: String,
    /// Host state token.
    pub health_token: String,
    /// Fault-domain id.
    pub fault_domain_id: String,
    /// Capabilities affected by the failure.
    pub affected_capability_tokens: Vec<String>,
    /// Visible artifacts that must remain stale or partial.
    pub stale_visible_artifact_refs: Vec<String>,
    /// Safe fallback summary.
    pub safe_fallback_label: String,
    /// Evidence or export reference.
    pub evidence_ref: String,
    /// Safe action tokens.
    pub action_tokens: Vec<String>,
    /// Whether this banner blocks a healthy status claim.
    pub blocks_healthy_claim: bool,
}

impl CrashLoopQuarantineBanner {
    /// Builds a banner when the lane is in crash-loop, quarantine, disabled,
    /// or disconnected state.
    pub fn maybe_from_lane(
        banner_id: impl Into<String>,
        lane: &HostLaneRecord,
        stale_visible_artifact_refs: Vec<String>,
        evidence_ref: impl Into<String>,
    ) -> Option<Self> {
        if !lane.health_class.blocks_healthy_claim() {
            return None;
        }
        let actions = derive_next_safe_actions(lane)
            .into_iter()
            .map(|action| action.as_str().to_owned())
            .collect::<Vec<_>>();
        Some(Self {
            record_kind: CRASH_LOOP_QUARANTINE_BANNER_RECORD_KIND.to_owned(),
            schema_version: HOST_TOPOLOGY_SCHEMA_VERSION,
            banner_id: banner_id.into(),
            failing_host_lane_ref: lane.lane_id.clone(),
            failing_host_label: lane.family_label.clone(),
            health_token: lane.health_token.clone(),
            fault_domain_id: lane.fault_domain_id.clone(),
            affected_capability_tokens: lane.affected_capability_tokens.clone(),
            stale_visible_artifact_refs,
            safe_fallback_label: safe_fallback_label_for(lane).to_owned(),
            evidence_ref: evidence_ref.into(),
            action_tokens: actions,
            blocks_healthy_claim: true,
        })
    }
}

fn safe_fallback_label_for(lane: &HostLaneRecord) -> &'static str {
    match lane.family {
        HostLaneFamily::LanguageAnalysisHost => "Use cached diagnostics while analysis refreshes",
        HostLaneFamily::ExtensionSandboxHost => "Continue with the extension host isolated",
        HostLaneFamily::DebugTaskAdapterHost => {
            "Inspect stale debug snapshots without live control"
        }
        HostLaneFamily::NotebookKernel => "Edit notebook without current kernel state",
        HostLaneFamily::RemoteWorkspaceAgent => "Continue local editing until reattach review",
        HostLaneFamily::ManagedServiceLane => "Use local-only or cached provider evidence",
        HostLaneFamily::LocalShellService => "Open recovery or safe mode",
    }
}

/// Restart marker attached to a lane-filtered event row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartMarkerClass {
    /// Event has no restart relationship.
    None,
    /// Event happened before a host restart.
    BeforeHostRestart,
    /// Event scheduled a restart.
    RestartScheduled,
    /// Event happened after a host restart.
    AfterHostRestart,
    /// Event entered quarantine.
    QuarantineEntered,
    /// Event came from a reviewed reattach.
    ReattachedAfterReview,
}

impl RestartMarkerClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::BeforeHostRestart => "before_host_restart",
            Self::RestartScheduled => "restart_scheduled",
            Self::AfterHostRestart => "after_host_restart",
            Self::QuarantineEntered => "quarantine_entered",
            Self::ReattachedAfterReview => "reattached_after_review",
        }
    }

    /// True when the marker should be emphasized in logs and support exports.
    pub const fn is_restart_related(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// One event row in a lane-filtered log or event viewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneEventRow {
    /// Event id.
    pub event_id: String,
    /// Host lane id.
    pub host_lane_ref: String,
    /// Host family label.
    pub host_family_label: String,
    /// Event timestamp.
    pub timestamp: String,
    /// Event class token.
    pub event_class_token: String,
    /// Restart marker.
    pub restart_marker: RestartMarkerClass,
    /// Stable restart-marker token.
    pub restart_marker_token: String,
    /// Related run or session reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_run_ref: Option<String>,
    /// Diagnostic or provenance reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_provenance_ref: Option<String>,
    /// Redaction class token.
    pub redaction_class_token: String,
    /// Export-safe event summary.
    pub summary: String,
}

impl LaneEventRow {
    /// Builds one event row.
    pub fn new(
        event_id: impl Into<String>,
        lane: &HostLaneRecord,
        timestamp: impl Into<String>,
        event_class_token: impl Into<String>,
        restart_marker: RestartMarkerClass,
        related_run_ref: Option<String>,
        diagnostic_provenance_ref: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            event_id: event_id.into(),
            host_lane_ref: lane.lane_id.clone(),
            host_family_label: lane.family_label.clone(),
            timestamp: timestamp.into(),
            event_class_token: event_class_token.into(),
            restart_marker,
            restart_marker_token: restart_marker.as_str().to_owned(),
            related_run_ref,
            diagnostic_provenance_ref,
            redaction_class_token: "metadata_safe_default".to_owned(),
            summary: summary.into(),
        }
    }
}

/// Lane-filtered event viewer preserving restart markers and provenance links.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneFilteredEventViewer {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable viewer id.
    pub viewer_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Active lane filter, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_lane_filter: Option<String>,
    /// Event rows.
    pub rows: Vec<LaneEventRow>,
    /// Count of restart-related rows.
    pub restart_marker_count: u32,
}

impl LaneFilteredEventViewer {
    /// Builds a lane-filtered event viewer.
    pub fn from_rows(
        viewer_id: impl Into<String>,
        workspace_id: impl Into<String>,
        active_lane_filter: Option<String>,
        rows: Vec<LaneEventRow>,
    ) -> Self {
        let rows = if let Some(filter) = &active_lane_filter {
            rows.into_iter()
                .filter(|row| row.host_lane_ref == *filter)
                .collect()
        } else {
            rows
        };
        let restart_marker_count = rows
            .iter()
            .filter(|row| row.restart_marker.is_restart_related())
            .count() as u32;
        Self {
            record_kind: LANE_FILTERED_EVENT_VIEWER_RECORD_KIND.to_owned(),
            schema_version: HOST_TOPOLOGY_SCHEMA_VERSION,
            viewer_id: viewer_id.into(),
            workspace_id: workspace_id.into(),
            active_lane_filter,
            rows,
            restart_marker_count,
        }
    }

    /// Returns a new viewer filtered to one lane.
    pub fn filter_by_lane(&self, lane_id: impl Into<String>) -> Self {
        Self::from_rows(
            self.viewer_id.clone(),
            self.workspace_id.clone(),
            Some(lane_id.into()),
            self.rows.clone(),
        )
    }

    /// Returns true when every row carries host-lane and provenance identity.
    pub fn rows_preserve_provenance(&self) -> bool {
        self.rows.iter().all(|row| {
            !row.event_id.is_empty()
                && !row.host_lane_ref.is_empty()
                && (!row.event_class_token.is_empty())
                && (row.related_run_ref.is_some() || row.diagnostic_provenance_ref.is_some())
        })
    }
}

/// Returns the canonical seeded host-lane records used by fixtures and docs.
pub fn seeded_host_lanes() -> Vec<HostLaneRecord> {
    vec![
        HostLaneRecord::from_seed(HostLaneSeed {
            lane_id: "lane:local-shell".into(),
            family: HostLaneFamily::LocalShellService,
            role_label: "Command routing and local view model".into(),
            host_label: "desktop shell".into(),
            target_ref: Some("target:local-device".into()),
            locality_label: "on device".into(),
            boundary_badge_classes: vec![HostBoundaryBadgeClass::Local],
            health_class: HostLaneHealthClass::Healthy,
            fault_domain_class: FaultDomainClass::ShellInteractionCore,
            fault_domain_id: "fault-domain:shell-interaction-core".into(),
            restart_budget_ref: "restart-budget:shell:no-hidden-restart".into(),
            restart_strike_count: 0,
            restart_budget_in_window: 0,
            affected_capability_tokens: vec![],
            preserved_checkpoint_refs: vec!["checkpoint:session-shell-layout".into()],
            stale_result_refs: vec![],
            quarantine_ref: None,
            current_host_confirmed: true,
        }),
        HostLaneRecord::from_seed(HostLaneSeed {
            lane_id: "lane:language-analysis".into(),
            family: HostLaneFamily::LanguageAnalysisHost,
            role_label: "Language server and semantic analysis".into(),
            host_label: "typescript analyzer".into(),
            target_ref: Some("target:workspace-root".into()),
            locality_label: "local worker".into(),
            boundary_badge_classes: vec![
                HostBoundaryBadgeClass::Local,
                HostBoundaryBadgeClass::Isolated,
            ],
            health_class: HostLaneHealthClass::Reconnecting,
            fault_domain_class: FaultDomainClass::WorkspaceKnowledgeGroup,
            fault_domain_id: "fault-domain:workspace-knowledge".into(),
            restart_budget_ref: "restart-budget:workspace-knowledge:analysis".into(),
            restart_strike_count: 1,
            restart_budget_in_window: 3,
            affected_capability_tokens: vec![
                "diagnostics".into(),
                "semantic_navigation".into(),
                "code_actions".into(),
            ],
            preserved_checkpoint_refs: vec!["checkpoint:analysis-cache-epoch-41".into()],
            stale_result_refs: vec![
                "result:diagnostic:language".into(),
                "result:log:language-host-restart".into(),
            ],
            quarantine_ref: None,
            current_host_confirmed: false,
        }),
        HostLaneRecord::from_seed(HostLaneSeed {
            lane_id: "lane:extension-sandbox".into(),
            family: HostLaneFamily::ExtensionSandboxHost,
            role_label: "Extension host sandbox".into(),
            host_label: "publisher.python-tools".into(),
            target_ref: Some("extension:publisher.python-tools".into()),
            locality_label: "isolated extension runtime".into(),
            boundary_badge_classes: vec![
                HostBoundaryBadgeClass::Isolated,
                HostBoundaryBadgeClass::ExtensionOwned,
            ],
            health_class: HostLaneHealthClass::Quarantined,
            fault_domain_class: FaultDomainClass::ExtensionOrToolHost,
            fault_domain_id: "fault-domain:extension-tool-host".into(),
            restart_budget_ref: "restart-budget:extension-host:default".into(),
            restart_strike_count: 2,
            restart_budget_in_window: 2,
            affected_capability_tokens: vec!["extension_commands".into(), "extension_views".into()],
            preserved_checkpoint_refs: vec!["checkpoint:extension-disable-state".into()],
            stale_result_refs: vec!["result:provider:summary".into()],
            quarantine_ref: Some("quarantine:extension-host:publisher.python-tools".into()),
            current_host_confirmed: false,
        }),
        HostLaneRecord::from_seed(HostLaneSeed {
            lane_id: "lane:debug-task-adapter".into(),
            family: HostLaneFamily::DebugTaskAdapterHost,
            role_label: "Debug and test adapter session".into(),
            host_label: "debugpy adapter".into(),
            target_ref: Some("run:debug-session-17".into()),
            locality_label: "execution-facing local adapter".into(),
            boundary_badge_classes: vec![
                HostBoundaryBadgeClass::Local,
                HostBoundaryBadgeClass::ExecutionFacing,
            ],
            health_class: HostLaneHealthClass::Disconnected,
            fault_domain_class: FaultDomainClass::SessionExecutionHost,
            fault_domain_id: "fault-domain:session-execution".into(),
            restart_budget_ref: "restart-budget:session:debug".into(),
            restart_strike_count: 1,
            restart_budget_in_window: 2,
            affected_capability_tokens: vec!["step_continue".into(), "expression_eval".into()],
            preserved_checkpoint_refs: vec!["checkpoint:debug-log-buffer".into()],
            stale_result_refs: vec!["result:debug:stale-frame".into()],
            quarantine_ref: None,
            current_host_confirmed: false,
        }),
        HostLaneRecord::from_seed(HostLaneSeed {
            lane_id: "lane:notebook-kernel".into(),
            family: HostLaneFamily::NotebookKernel,
            role_label: "Notebook execution kernel".into(),
            host_label: "python kernel".into(),
            target_ref: Some("notebook:analysis.ipynb".into()),
            locality_label: "stateful kernel".into(),
            boundary_badge_classes: vec![
                HostBoundaryBadgeClass::KernelStateful,
                HostBoundaryBadgeClass::ExecutionFacing,
            ],
            health_class: HostLaneHealthClass::CrashLoop,
            fault_domain_class: FaultDomainClass::SessionExecutionHost,
            fault_domain_id: "fault-domain:notebook-kernel".into(),
            restart_budget_ref: "restart-budget:session:notebook".into(),
            restart_strike_count: 2,
            restart_budget_in_window: 2,
            affected_capability_tokens: vec!["cell_run".into(), "variable_explorer".into()],
            preserved_checkpoint_refs: vec!["checkpoint:notebook-output-snapshot".into()],
            stale_result_refs: vec!["result:notebook:output".into()],
            quarantine_ref: Some("crash:kern-112".into()),
            current_host_confirmed: false,
        }),
        HostLaneRecord::from_seed(HostLaneSeed {
            lane_id: "lane:remote-agent".into(),
            family: HostLaneFamily::RemoteWorkspaceAgent,
            role_label: "Remote filesystem and runtime authority".into(),
            host_label: "remote-agent 1.8.3".into(),
            target_ref: Some("target:remote:ssh-staging".into()),
            locality_label: "remote target".into(),
            boundary_badge_classes: vec![
                HostBoundaryBadgeClass::RemoteBoundary,
                HostBoundaryBadgeClass::ExecutionFacing,
            ],
            health_class: HostLaneHealthClass::Disconnected,
            fault_domain_class: FaultDomainClass::RemoteConnector,
            fault_domain_id: "fault-domain:remote-connector".into(),
            restart_budget_ref: "restart-budget:remote-connector:bounded-reconnect".into(),
            restart_strike_count: 1,
            restart_budget_in_window: 3,
            affected_capability_tokens: vec!["remote_terminal".into(), "forwarded_ports".into()],
            preserved_checkpoint_refs: vec!["checkpoint:remote-route-witness".into()],
            stale_result_refs: vec!["result:preview:remote-web".into()],
            quarantine_ref: None,
            current_host_confirmed: false,
        }),
        HostLaneRecord::from_seed(HostLaneSeed {
            lane_id: "lane:managed-service".into(),
            family: HostLaneFamily::ManagedServiceLane,
            role_label: "Provider-backed runtime summary".into(),
            host_label: "managed control plane".into(),
            target_ref: Some("provider:workspace-control".into()),
            locality_label: "managed service".into(),
            boundary_badge_classes: vec![HostBoundaryBadgeClass::ManagedBoundary],
            health_class: HostLaneHealthClass::Degraded,
            fault_domain_class: FaultDomainClass::AiToolBroker,
            fault_domain_id: "fault-domain:managed-service".into(),
            restart_budget_ref: "restart-budget:provider-broker:circuit-breaker".into(),
            restart_strike_count: 1,
            restart_budget_in_window: 1,
            affected_capability_tokens: vec!["provider_status".into(), "ai_tool_dispatch".into()],
            preserved_checkpoint_refs: vec!["checkpoint:provider-evidence-packet".into()],
            stale_result_refs: vec![
                "result:ai:tool-action".into(),
                "result:provider:summary".into(),
            ],
            quarantine_ref: None,
            current_host_confirmed: false,
        }),
    ]
}

/// Returns a seeded topology inspector covering every required beta surface.
pub fn seeded_host_topology_inspector() -> TopologyInspectorRecord {
    let lanes = seeded_host_lanes();
    TopologyInspectorRecord::from_lanes_and_results(
        "topology:host-lane:seed",
        "workspace:host-lane:seed",
        "2026-05-18T12:00:00Z",
        lanes,
        vec![
            RuntimeResultSeed {
                result_id: "result:log:language-host-restart".into(),
                surface: RuntimeSurfaceClass::Logs,
                result_summary: "Analysis host restart marker in event log".into(),
                host_lane_ids: vec!["lane:language-analysis".into()],
                freshness_class: HostResultFreshnessClass::Rebuilding,
                provenance_ref: Some("event:analysis:restart-scheduled".into()),
                requires_reattach_review: false,
            },
            RuntimeResultSeed {
                result_id: "result:run:pytest".into(),
                surface: RuntimeSurfaceClass::Run,
                result_summary: "Test run preserved with adapter identity".into(),
                host_lane_ids: vec!["lane:debug-task-adapter".into()],
                freshness_class: HostResultFreshnessClass::DisconnectedSnapshot,
                provenance_ref: Some("run:pytest:42".into()),
                requires_reattach_review: true,
            },
            RuntimeResultSeed {
                result_id: "result:debug:stale-frame".into(),
                surface: RuntimeSurfaceClass::DebugView,
                result_summary: "Last debug frame is a stale snapshot".into(),
                host_lane_ids: vec!["lane:debug-task-adapter".into()],
                freshness_class: HostResultFreshnessClass::DisconnectedSnapshot,
                provenance_ref: Some("debug-session:17".into()),
                requires_reattach_review: true,
            },
            RuntimeResultSeed {
                result_id: "result:notebook:output".into(),
                surface: RuntimeSurfaceClass::NotebookOutput,
                result_summary: "Notebook output predates kernel crash".into(),
                host_lane_ids: vec!["lane:notebook-kernel".into()],
                freshness_class: HostResultFreshnessClass::Stale,
                provenance_ref: Some("notebook-output:analysis:cell-8".into()),
                requires_reattach_review: true,
            },
            RuntimeResultSeed {
                result_id: "result:preview:remote-web".into(),
                surface: RuntimeSurfaceClass::Preview,
                result_summary: "Remote preview awaits agent reattach".into(),
                host_lane_ids: vec!["lane:remote-agent".into()],
                freshness_class: HostResultFreshnessClass::AwaitingRefresh,
                provenance_ref: Some("preview:remote-web".into()),
                requires_reattach_review: true,
            },
            RuntimeResultSeed {
                result_id: "result:ai:tool-action".into(),
                surface: RuntimeSurfaceClass::AiToolAction,
                result_summary: "AI tool action held behind provider lane degradation".into(),
                host_lane_ids: vec!["lane:managed-service".into()],
                freshness_class: HostResultFreshnessClass::PartialTruth,
                provenance_ref: Some("tool-action:plan:19".into()),
                requires_reattach_review: true,
            },
            RuntimeResultSeed {
                result_id: "result:provider:summary".into(),
                surface: RuntimeSurfaceClass::ProviderRuntimeSummary,
                result_summary: "Provider-backed runtime summary is partial".into(),
                host_lane_ids: vec![
                    "lane:managed-service".into(),
                    "lane:extension-sandbox".into(),
                ],
                freshness_class: HostResultFreshnessClass::PartialTruth,
                provenance_ref: Some("provider-summary:workspace-control".into()),
                requires_reattach_review: true,
            },
            RuntimeResultSeed {
                result_id: "result:diagnostic:language".into(),
                surface: RuntimeSurfaceClass::RuntimeDiagnostic,
                result_summary: "Semantic diagnostics are stale until analysis confirms".into(),
                host_lane_ids: vec!["lane:language-analysis".into()],
                freshness_class: HostResultFreshnessClass::Stale,
                provenance_ref: Some("diagnostic:semantic:ts".into()),
                requires_reattach_review: false,
            },
        ],
    )
}

/// Returns a seeded reattach review for remote-agent identity drift.
pub fn seeded_reattach_review_sheet() -> ReattachReviewSheet {
    let previous = HostLaneRecord::from_seed(HostLaneSeed {
        lane_id: "lane:remote-agent".into(),
        family: HostLaneFamily::RemoteWorkspaceAgent,
        role_label: "Remote filesystem and runtime authority".into(),
        host_label: "remote-agent 1.8.2".into(),
        target_ref: Some("target:remote:ssh-staging-old".into()),
        locality_label: "remote target".into(),
        boundary_badge_classes: vec![HostBoundaryBadgeClass::RemoteBoundary],
        health_class: HostLaneHealthClass::Healthy,
        fault_domain_class: FaultDomainClass::RemoteConnector,
        fault_domain_id: "fault-domain:remote-connector".into(),
        restart_budget_ref: "restart-budget:remote-connector:bounded-reconnect".into(),
        restart_strike_count: 0,
        restart_budget_in_window: 3,
        affected_capability_tokens: vec![],
        preserved_checkpoint_refs: vec!["checkpoint:remote-route-witness:old".into()],
        stale_result_refs: vec![],
        quarantine_ref: None,
        current_host_confirmed: true,
    });
    let current = seeded_host_lanes()
        .into_iter()
        .find(|lane| lane.lane_id == "lane:remote-agent")
        .expect("seeded remote lane exists");
    ReattachReviewSheet::compare(
        &previous,
        &current,
        ReattachReviewInput {
            review_id: "reattach-review:remote-agent:seed".into(),
            previous_policy_epoch: 14,
            current_policy_epoch: 15,
            previous_auth_scope_ref: "auth-scope:remote-agent:old".into(),
            current_auth_scope_ref: "auth-scope:remote-agent:current".into(),
            preserved_state_refs: vec![
                "checkpoint:remote-route-witness".into(),
                "log-buffer:remote-agent".into(),
            ],
            lost_state_refs: vec!["forwarded-port:3000".into(), "live-terminal-control".into()],
            replay_risk_class: ReattachReplayRiskClass::Privileged,
            rerun_requirement_class: RerunRequirementClass::ReapprovalRequired,
            reviewed_at: "2026-05-18T12:02:00Z".into(),
        },
    )
}

/// Returns a seeded lane-filtered event viewer with restart markers.
pub fn seeded_lane_filtered_event_viewer() -> LaneFilteredEventViewer {
    let lanes = seeded_host_lanes()
        .into_iter()
        .map(|lane| (lane.lane_id.clone(), lane))
        .collect::<BTreeMap<_, _>>();
    let analysis = lanes
        .get("lane:language-analysis")
        .expect("analysis lane exists");
    let debug = lanes
        .get("lane:debug-task-adapter")
        .expect("debug lane exists");
    let remote = lanes.get("lane:remote-agent").expect("remote lane exists");
    let notebook = lanes
        .get("lane:notebook-kernel")
        .expect("notebook lane exists");
    LaneFilteredEventViewer::from_rows(
        "event-viewer:host-lanes:seed",
        "workspace:host-lane:seed",
        None,
        vec![
            LaneEventRow::new(
                "event:analysis:heartbeat-lost",
                analysis,
                "2026-05-18T12:00:03Z",
                "heartbeat_lost",
                RestartMarkerClass::BeforeHostRestart,
                None,
                Some("ctx-prov:analysis".into()),
                "Analysis heartbeat was lost before restart.",
            ),
            LaneEventRow::new(
                "event:analysis:restart-scheduled",
                analysis,
                "2026-05-18T12:00:05Z",
                "restart_scheduled",
                RestartMarkerClass::RestartScheduled,
                None,
                Some("ctx-prov:analysis".into()),
                "Analysis host restart scheduled inside budget.",
            ),
            LaneEventRow::new(
                "event:debug:disconnect",
                debug,
                "2026-05-18T12:00:07Z",
                "session_disconnected",
                RestartMarkerClass::BeforeHostRestart,
                Some("run:pytest:42".into()),
                Some("ctx-prov:debug".into()),
                "Debug adapter disconnected; live control is stale.",
            ),
            LaneEventRow::new(
                "event:notebook:quarantine",
                notebook,
                "2026-05-18T12:00:09Z",
                "kernel_quarantined",
                RestartMarkerClass::QuarantineEntered,
                Some("notebook:analysis.ipynb".into()),
                Some("ctx-prov:notebook".into()),
                "Notebook kernel entered crash-loop quarantine.",
            ),
            LaneEventRow::new(
                "event:remote:reattach-review",
                remote,
                "2026-05-18T12:00:12Z",
                "reattach_review_required",
                RestartMarkerClass::ReattachedAfterReview,
                Some("preview:remote-web".into()),
                Some("ctx-prov:remote".into()),
                "Remote agent identity changed; review is required.",
            ),
        ],
    )
}
