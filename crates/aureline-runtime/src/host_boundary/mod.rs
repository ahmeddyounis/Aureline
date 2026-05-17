//! Host-boundary truth, wrong-target reapproval, and lifecycle export.
//!
//! This module composes the existing execution-context, target-discovery, and
//! managed-workspace lifecycle records into one redaction-safe packet that
//! task, terminal, debug, AI-tool, browser-handoff, transcript, CLI/headless,
//! and support/export surfaces can quote without rewording target truth. It
//! keeps discovery source, confidence, freshness, host-boundary cue stack,
//! route identity, lifecycle projection, and wrong-target reapproval posture on
//! one reusable object.

use serde::{Deserialize, Serialize};

use crate::execution_context::{
    ExecutionContext, ReachabilityState, SurfaceClass, TargetClass, TrustState,
};
use crate::managed_workspace_lifecycle_beta::{
    ManagedLifecycleStateClass, ManagedWorkspaceLifecycleBetaRecord,
};
use crate::target_discovery::{
    DiscoveryFreshnessClass, ProtectedActionClass, ProtectedActionDecisionClass,
    ProtectedActionDecisionRow, TargetDiscoveryBetaRow,
};
use crate::targets::{HostBoundaryCueClass, TargetConfidenceCard};

/// Schema version for host-boundary truth, projection, support export, and reapproval records.
pub const HOST_BOUNDARY_AND_LIFECYCLE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for one host-boundary truth record.
pub const HOST_BOUNDARY_TRUTH_RECORD_KIND: &str = "host_boundary_truth_record";

/// Stable record-kind tag for one host-boundary surface projection.
pub const HOST_BOUNDARY_SURFACE_PROJECTION_RECORD_KIND: &str =
    "host_boundary_surface_projection_record";

/// Stable record-kind tag for a review-vs-commit reapproval evaluation.
pub const HOST_BOUNDARY_REAPPROVAL_EVALUATION_RECORD_KIND: &str =
    "host_boundary_reapproval_evaluation_record";

/// Stable record-kind tag for a host-boundary support/export packet.
pub const HOST_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND: &str = "host_boundary_support_export_record";

/// Surface consuming a host-boundary truth projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostBoundarySurfaceClass {
    /// Task, run, test, or build row that can dispatch work.
    Task,
    /// Interactive terminal header or terminal command-boundary row.
    Terminal,
    /// Debug launch, attach, frame, or adapter-control surface.
    Debug,
    /// AI tool-call plane that can observe, propose, or dispatch tool work.
    AiTool,
    /// Browser handoff sheet, callback receiver, or return-path row.
    BrowserHandoff,
    /// Restored transcript or captured runtime output view.
    Transcript,
    /// CLI or headless output surface.
    CliHeadless,
    /// Support bundle, operator export, or evidence packet surface.
    SupportExport,
}

impl HostBoundarySurfaceClass {
    /// All surfaces protected by this beta contract.
    pub const ALL: [Self; 8] = [
        Self::Task,
        Self::Terminal,
        Self::Debug,
        Self::AiTool,
        Self::BrowserHandoff,
        Self::Transcript,
        Self::CliHeadless,
        Self::SupportExport,
    ];

    /// Stable token recorded in projections and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Task => "task",
            Self::Terminal => "terminal",
            Self::Debug => "debug",
            Self::AiTool => "ai_tool",
            Self::BrowserHandoff => "browser_handoff",
            Self::Transcript => "transcript",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Task => "Task",
            Self::Terminal => "Terminal",
            Self::Debug => "Debug",
            Self::AiTool => "AI tool",
            Self::BrowserHandoff => "Browser handoff",
            Self::Transcript => "Transcript",
            Self::CliHeadless => "CLI/headless",
            Self::SupportExport => "Support export",
        }
    }
}

impl From<SurfaceClass> for HostBoundarySurfaceClass {
    fn from(value: SurfaceClass) -> Self {
        match value {
            SurfaceClass::Terminal => Self::Terminal,
            SurfaceClass::Task | SurfaceClass::Test | SurfaceClass::Scaffolding => Self::Task,
            SurfaceClass::Debug => Self::Debug,
            SurfaceClass::AiToolCall => Self::AiTool,
            SurfaceClass::NotebookKernel
            | SurfaceClass::DoctorRepair
            | SurfaceClass::ImportProbe
            | SurfaceClass::ReplayProbe => Self::CliHeadless,
        }
    }
}

/// Origin token for the action or evidence row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionOriginClass {
    /// User action from the local desktop shell.
    UserKeystrokeLocal,
    /// CLI command invoked on the local desktop or headless runner.
    CliInvocationLocal,
    /// AI tool call initiated from the local client.
    AiToolCallLocal,
    /// AI tool call initiated from a managed execution lane.
    AiToolCallManaged,
    /// Remote helper or agent requested the action.
    RemoteAgentRequest,
    /// Managed control plane requested the action.
    ManagedControlPlaneRequest,
    /// Provider or browser return callback supplied the action.
    ProviderCallbackInbound,
    /// Imported session, restore packet, or transcript supplied evidence.
    ImportOrRestoreSession,
}

impl ActionOriginClass {
    /// Stable token recorded in route truth.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserKeystrokeLocal => "user_keystroke_local",
            Self::CliInvocationLocal => "cli_invocation_local",
            Self::AiToolCallLocal => "ai_tool_call_local",
            Self::AiToolCallManaged => "ai_tool_call_managed",
            Self::RemoteAgentRequest => "remote_agent_request",
            Self::ManagedControlPlaneRequest => "managed_control_plane_request",
            Self::ProviderCallbackInbound => "provider_callback_inbound",
            Self::ImportOrRestoreSession => "import_or_restore_session",
        }
    }
}

/// Target token from the origin/target/route vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionTargetClass {
    /// Local desktop host target.
    LocalHostTarget,
    /// Local container target.
    ContainerLocalTarget,
    /// Devcontainer target.
    DevcontainerTarget,
    /// SSH remote target.
    RemoteSshTarget,
    /// Remote agent target.
    RemoteAgentTarget,
    /// Managed workspace target.
    ManagedWorkspaceTarget,
    /// Local notebook kernel target.
    NotebookKernelLocalTarget,
    /// Remote notebook kernel target.
    NotebookKernelRemoteTarget,
    /// AI sandbox target.
    AiSandboxTarget,
    /// System browser target.
    SystemBrowserTarget,
    /// Unknown target requiring review.
    UnknownTargetClass,
}

impl ActionTargetClass {
    /// Stable token recorded in route truth.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHostTarget => "local_host_target",
            Self::ContainerLocalTarget => "container_local_target",
            Self::DevcontainerTarget => "devcontainer_target",
            Self::RemoteSshTarget => "remote_ssh_target",
            Self::RemoteAgentTarget => "remote_agent_target",
            Self::ManagedWorkspaceTarget => "managed_workspace_target",
            Self::NotebookKernelLocalTarget => "notebook_kernel_local_target",
            Self::NotebookKernelRemoteTarget => "notebook_kernel_remote_target",
            Self::AiSandboxTarget => "ai_sandbox_target",
            Self::SystemBrowserTarget => "system_browser_target",
            Self::UnknownTargetClass => "unknown_target_class",
        }
    }
}

/// Route token from the origin/target/route vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionRouteClass {
    /// Work stays inside the local process or host.
    InProcessRoute,
    /// Work uses a local RPC route.
    LocalRpcRoute,
    /// Work uses an SSH or remote RPC route.
    RemoteRpcRoute,
    /// Work uses a managed control-plane route.
    ManagedControlPlaneRoute,
    /// Work uses a remote-agent attach route.
    RemoteAgentAttachRoute,
    /// Work uses an approval-gated route.
    ApprovalGatedRoute,
    /// Work is handed to a browser surface.
    BrowserHandoffRoute,
    /// Work traverses a tunnel exposure route.
    TunnelExposedRoute,
    /// Work uses a bridged helper route.
    BridgedHelperRoute,
    /// Route truth is unknown and must be reviewed.
    HeuristicUnknownRoute,
}

impl ActionRouteClass {
    /// Stable token recorded in route truth.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProcessRoute => "in_process_route",
            Self::LocalRpcRoute => "local_rpc_route",
            Self::RemoteRpcRoute => "remote_rpc_route",
            Self::ManagedControlPlaneRoute => "managed_control_plane_route",
            Self::RemoteAgentAttachRoute => "remote_agent_attach_route",
            Self::ApprovalGatedRoute => "approval_gated_route",
            Self::BrowserHandoffRoute => "browser_handoff_route",
            Self::TunnelExposedRoute => "tunnel_exposed_route",
            Self::BridgedHelperRoute => "bridged_helper_route",
            Self::HeuristicUnknownRoute => "heuristic_unknown_route",
        }
    }

    /// Short reviewer-facing route label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::InProcessRoute => "In process",
            Self::LocalRpcRoute => "Local RPC",
            Self::RemoteRpcRoute => "Remote RPC",
            Self::ManagedControlPlaneRoute => "Managed control plane",
            Self::RemoteAgentAttachRoute => "Remote agent attach",
            Self::ApprovalGatedRoute => "Approval gated",
            Self::BrowserHandoffRoute => "Browser handoff",
            Self::TunnelExposedRoute => "Tunnel exposed",
            Self::BridgedHelperRoute => "Bridged helper",
            Self::HeuristicUnknownRoute => "Unknown route",
        }
    }
}

/// Exposure token from the origin/target/route vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionExposureClass {
    /// Read-only local action with no side effect.
    NoSideEffectLocalRead,
    /// Mutation remains local to the selected workspace.
    LocalOnlyMutation,
    /// Mutation is visible inside the current workspace.
    WorkspaceVisibleMutation,
    /// Browser session can observe the state.
    BrowserSessionVisible,
    /// Tunnel route is exposed publicly.
    TunnelExposedPublic,
    /// Exposure is unknown and requires review.
    ExposureUnknownRequiresReview,
}

impl ActionExposureClass {
    /// Stable token recorded in route truth.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSideEffectLocalRead => "no_side_effect_local_read",
            Self::LocalOnlyMutation => "local_only_mutation",
            Self::WorkspaceVisibleMutation => "workspace_visible_mutation",
            Self::BrowserSessionVisible => "browser_session_visible",
            Self::TunnelExposedPublic => "tunnel_exposed_public",
            Self::ExposureUnknownRequiresReview => "exposure_unknown_requires_review",
        }
    }
}

/// Reason token explaining why route truth changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteChangeReasonCode {
    /// No route change was observed.
    CanonicalNoRouteChange,
    /// Route changed because approval is now required.
    RouteEscalatedToApprovalRequired,
    /// Route changed to browser handoff.
    RouteEscalatedToBrowserHandoff,
    /// Route target became unreachable.
    RouteChangedTargetUnreachable,
    /// Route target was reassigned.
    RouteChangedTargetReassigned,
    /// Policy narrowed the route.
    RouteChangedPolicyNarrowed,
    /// Wrong-target detection changed the route.
    RouteChangedWrongTargetDetected,
    /// Approval was withdrawn.
    RouteChangedApprovalWithdrawn,
    /// Freshness floor was not met.
    RouteChangedFreshnessFloorUnmet,
    /// Host boundary changed.
    RouteChangedHostMismatch,
    /// Route is unknown and requires review.
    RouteUnknownRequiresReview,
}

impl RouteChangeReasonCode {
    /// Stable token recorded in route truth.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalNoRouteChange => "canonical_no_route_change",
            Self::RouteEscalatedToApprovalRequired => "route_escalated_to_approval_required",
            Self::RouteEscalatedToBrowserHandoff => "route_escalated_to_browser_handoff",
            Self::RouteChangedTargetUnreachable => "route_changed_target_unreachable",
            Self::RouteChangedTargetReassigned => "route_changed_target_reassigned",
            Self::RouteChangedPolicyNarrowed => "route_changed_policy_narrowed",
            Self::RouteChangedWrongTargetDetected => "route_changed_wrong_target_detected",
            Self::RouteChangedApprovalWithdrawn => "route_changed_approval_withdrawn",
            Self::RouteChangedFreshnessFloorUnmet => "route_changed_freshness_floor_unmet",
            Self::RouteChangedHostMismatch => "route_changed_host_mismatch",
            Self::RouteUnknownRequiresReview => "route_unknown_requires_review",
        }
    }
}

/// Authority linkage token for the selected route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityLinkageClass {
    /// No authority is required because the row is read-only evidence.
    NoAuthorityRequiredReadOnly,
    /// Local user action is the authority source.
    LocalUserKeystrokeAuthority,
    /// Approval ticket is linked.
    ApprovalTicketLinked,
    /// Browser handoff packet is linked.
    BrowserHandoffPacketLinked,
    /// Remote attach ticket is linked.
    RemoteAgentAttachTicketLinked,
    /// Managed control-plane token is linked.
    ManagedControlPlaneTokenLinked,
    /// Supervisor repair ticket is linked.
    SupervisorRepairTicketLinked,
    /// Required authority is missing and the action is denied.
    AuthorityMissingDenied,
}

impl AuthorityLinkageClass {
    /// Stable token recorded in route truth.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAuthorityRequiredReadOnly => "no_authority_required_read_only",
            Self::LocalUserKeystrokeAuthority => "local_user_keystroke_authority",
            Self::ApprovalTicketLinked => "approval_ticket_linked",
            Self::BrowserHandoffPacketLinked => "browser_handoff_packet_linked",
            Self::RemoteAgentAttachTicketLinked => "remote_agent_attach_ticket_linked",
            Self::ManagedControlPlaneTokenLinked => "managed_control_plane_token_linked",
            Self::SupervisorRepairTicketLinked => "supervisor_repair_ticket_linked",
            Self::AuthorityMissingDenied => "authority_missing_denied",
        }
    }
}

/// Freshness token projected with host-boundary truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryFreshnessClass {
    /// Current authoritative owner was contacted for this row.
    AuthoritativeLive,
    /// Cached or session-warm evidence is still useful but not live.
    WarmCached,
    /// Cached evidence is degraded and must stay labeled.
    DegradedCached,
    /// Evidence is stale.
    Stale,
    /// Freshness is unknown.
    Unknown,
}

impl BoundaryFreshnessClass {
    /// Stable token recorded in truth records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
        }
    }
}

/// Reachability token projected with host-boundary truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryReachabilityClass {
    /// Target is reachable.
    Reachable,
    /// Target is warming.
    Warming,
    /// Target is degraded but partially reachable.
    Degraded,
    /// Target is unreachable.
    Unreachable,
    /// Target is disabled by policy.
    DisabledByPolicy,
    /// Target is reachable only after reauthentication.
    ReachablePendingReauth,
}

impl BoundaryReachabilityClass {
    /// Stable token recorded in truth records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::Warming => "warming",
            Self::Degraded => "degraded",
            Self::Unreachable => "unreachable",
            Self::DisabledByPolicy => "disabled_by_policy",
            Self::ReachablePendingReauth => "reachable_pending_reauth",
        }
    }
}

/// Managed-workspace lifecycle token from the host-boundary verification matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryManagedLifecycleState {
    /// No managed workspace is declared.
    Undeclared,
    /// Managed control plane is provisioning the workspace.
    Provisioning,
    /// Workspace warmers are running.
    Warming,
    /// Workspace is ready.
    Ready,
    /// Workspace is idle-suspended.
    IdleSuspended,
    /// Workspace is snapshot-paused.
    SnapshotPaused,
    /// Workspace is hibernated.
    Hibernated,
    /// Workspace is recovering.
    Recovering,
    /// Workspace is quarantined.
    Quarantined,
    /// Workspace is retiring.
    Retiring,
    /// Workspace is retired.
    Retired,
}

impl BoundaryManagedLifecycleState {
    /// Stable token recorded in truth records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Undeclared => "undeclared",
            Self::Provisioning => "provisioning",
            Self::Warming => "warming",
            Self::Ready => "ready",
            Self::IdleSuspended => "idle_suspended",
            Self::SnapshotPaused => "snapshot_paused",
            Self::Hibernated => "hibernated",
            Self::Recovering => "recovering",
            Self::Quarantined => "quarantined",
            Self::Retiring => "retiring",
            Self::Retired => "retired",
        }
    }
}

/// Reviewer-facing managed-workspace lifecycle label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedWorkspaceReviewerLabel {
    /// Managed workspace is warming.
    Warming,
    /// Managed workspace is ready.
    Ready,
    /// Managed workspace is degraded.
    Degraded,
    /// Managed workspace is paused.
    Paused,
    /// Managed workspace is suspended.
    Suspended,
    /// Managed workspace is expired.
    Expired,
    /// Editor continues locally while managed authority is unavailable.
    LocalOnlyContinuation,
}

impl ManagedWorkspaceReviewerLabel {
    /// Stable token recorded in truth records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Warming => "warming",
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::Paused => "paused",
            Self::Suspended => "suspended",
            Self::Expired => "expired",
            Self::LocalOnlyContinuation => "local_only_continuation",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Warming => "Warming",
            Self::Ready => "Ready",
            Self::Degraded => "Degraded",
            Self::Paused => "Paused",
            Self::Suspended => "Suspended",
            Self::Expired => "Expired",
            Self::LocalOnlyContinuation => "Local-only continuation",
        }
    }
}

/// Expiry reason attached to an expired managed-workspace projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpiryReasonClass {
    /// Session ticket expired.
    SessionTicketExpired,
    /// Hibernation window elapsed.
    HibernationWindowElapsed,
    /// Retirement drain completed.
    RetirementDrainWindowCompleted,
    /// Policy epoch rolled.
    PolicyEpochRolled,
    /// Kill switch tripped.
    KillSwitchTripped,
    /// Successor image is available.
    SuccessorImageAvailable,
}

impl ExpiryReasonClass {
    /// Stable token recorded in truth records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionTicketExpired => "session_ticket_expired",
            Self::HibernationWindowElapsed => "hibernation_window_elapsed",
            Self::RetirementDrainWindowCompleted => "retirement_drain_window_completed",
            Self::PolicyEpochRolled => "policy_epoch_rolled",
            Self::KillSwitchTripped => "kill_switch_tripped",
            Self::SuccessorImageAvailable => "successor_image_available",
        }
    }
}

/// Reason attached to a local-only continuation projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalOnlyContinuationReasonClass {
    /// Route dependency is unreachable.
    RouteDependencyUnreachable,
    /// Managed control plane is unreachable.
    ManagedControlPlaneUnreachable,
    /// Remote-agent attach is unreachable.
    RemoteAgentAttachUnreachable,
    /// Browser handoff return is unavailable.
    BrowserHandoffReturnUnavailable,
    /// User requested local fallback.
    UserRequestedLocalFallback,
    /// Administrator requested local fallback.
    AdminRequestedLocalFallback,
}

impl LocalOnlyContinuationReasonClass {
    /// Stable token recorded in truth records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteDependencyUnreachable => "route_dependency_unreachable",
            Self::ManagedControlPlaneUnreachable => "managed_control_plane_unreachable",
            Self::RemoteAgentAttachUnreachable => "remote_agent_attach_unreachable",
            Self::BrowserHandoffReturnUnavailable => "browser_handoff_return_unavailable",
            Self::UserRequestedLocalFallback => "user_requested_local_fallback",
            Self::AdminRequestedLocalFallback => "admin_requested_local_fallback",
        }
    }
}

/// Wrong-target correction class for review and support surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WrongTargetCorrectionClass {
    /// No correction was needed.
    NoCorrectionNeeded,
    /// Wrong target was corrected before commit.
    CorrectedBeforeCommit,
    /// Wrong target was corrected after a partial effect.
    CorrectedAfterPartialEffect,
    /// User must choose between plausible targets.
    RequiresUserConfirmation,
    /// Action is blocked until reapproval completes.
    BlockedPendingReapproval,
}

impl WrongTargetCorrectionClass {
    /// Stable token recorded in truth records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCorrectionNeeded => "no_correction_needed",
            Self::CorrectedBeforeCommit => "corrected_before_commit",
            Self::CorrectedAfterPartialEffect => "corrected_after_partial_effect",
            Self::RequiresUserConfirmation => "requires_user_confirmation",
            Self::BlockedPendingReapproval => "blocked_pending_reapproval",
        }
    }
}

/// Reapproval requirement class for wrong-target and route-drift cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReapprovalRequirementClass {
    /// No reapproval is required.
    NoReapprovalRequired,
    /// Session ticket refresh is required.
    SessionTicketRefreshRequired,
    /// Approval ticket must be reissued.
    ApprovalTicketReissueRequired,
    /// Administrator confirmation is required.
    AdminConfirmationRequired,
    /// Policy narrowing review is required.
    PolicyNarrowingRequired,
    /// Workspace trust must be re-evaluated.
    TrustReevaluationRequired,
}

impl ReapprovalRequirementClass {
    /// Stable token recorded in truth records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoReapprovalRequired => "no_reapproval_required",
            Self::SessionTicketRefreshRequired => "session_ticket_refresh_required",
            Self::ApprovalTicketReissueRequired => "approval_ticket_reissue_required",
            Self::AdminConfirmationRequired => "admin_confirmation_required",
            Self::PolicyNarrowingRequired => "policy_narrowing_required",
            Self::TrustReevaluationRequired => "trust_reevaluation_required",
        }
    }

    /// True when the class requires a new review or approval before commit.
    pub const fn blocks_reuse(self) -> bool {
        !matches!(self, Self::NoReapprovalRequired)
    }
}

/// Redaction posture for host-boundary truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryRedactionClass {
    /// Metadata and hashes only.
    MetadataAndHashesOnly,
    /// Support-bundle scoped export.
    SupportBundleScoped,
    /// Broadened capture export.
    BroadenedCapture,
}

impl BoundaryRedactionClass {
    /// Stable token recorded in truth records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataAndHashesOnly => "metadata_and_hashes_only",
            Self::SupportBundleScoped => "support_bundle_scoped",
            Self::BroadenedCapture => "broadened_capture",
        }
    }
}

/// Export-inclusion posture for host-boundary truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportInclusionPosture {
    /// Safe to include by default as metadata.
    MetadataSafeDefault,
    /// Restricted to operator/support review.
    OperatorOnlyRestricted,
    /// Included only after broadened capture opt-in.
    BroadenedCaptureOptIn,
}

impl ExportInclusionPosture {
    /// Stable token recorded in truth records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::BroadenedCaptureOptIn => "broadened_capture_opt_in",
        }
    }
}

/// Adapter kind for adapter-side confidence evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterKind {
    /// Remote-agent attach adapter.
    RemoteAgentAttach,
    /// SSH adapter.
    RemoteSshAdapter,
    /// Managed-workspace adapter.
    ManagedWorkspaceAdapter,
    /// Notebook-kernel adapter.
    NotebookKernelAdapter,
    /// Bridged-helper adapter.
    BridgedHelperAdapter,
    /// AI sandbox adapter.
    AiSandboxAdapter,
}

impl AdapterKind {
    /// Stable token recorded in truth records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemoteAgentAttach => "remote_agent_attach",
            Self::RemoteSshAdapter => "remote_ssh_adapter",
            Self::ManagedWorkspaceAdapter => "managed_workspace_adapter",
            Self::NotebookKernelAdapter => "notebook_kernel_adapter",
            Self::BridgedHelperAdapter => "bridged_helper_adapter",
            Self::AiSandboxAdapter => "ai_sandbox_adapter",
        }
    }
}

/// Adapter-side confidence class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterConfidenceClass {
    /// Adapter reported an authoritative match.
    AdapterAuthoritativeMatch,
    /// Adapter probe was consistent.
    AdapterProbedConsistent,
    /// Adapter probe diverged.
    AdapterProbedDivergent,
    /// Adapter inferred the target from session context.
    AdapterInferredFromSession,
    /// Adapter was unreachable.
    AdapterUnreachable,
}

impl AdapterConfidenceClass {
    /// Stable token recorded in truth records and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterAuthoritativeMatch => "adapter_authoritative_match",
            Self::AdapterProbedConsistent => "adapter_probed_consistent",
            Self::AdapterProbedDivergent => "adapter_probed_divergent",
            Self::AdapterInferredFromSession => "adapter_inferred_from_session",
            Self::AdapterUnreachable => "adapter_unreachable",
        }
    }
}

/// Adapter confidence placeholder carried beside canonical discovery confidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterConfidencePlaceholder {
    /// Adapter kind.
    pub adapter_kind: AdapterKind,
    /// Stable adapter-kind token.
    pub adapter_kind_token: String,
    /// Adapter confidence class.
    pub adapter_confidence_class: AdapterConfidenceClass,
    /// Stable adapter-confidence token.
    pub adapter_confidence_token: String,
    /// Divergence or inference reason tokens.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adapter_divergence_or_inference_reasons: Vec<String>,
    /// Authority envelope tag carried by the adapter evidence.
    pub adapter_authority_envelope_tag: String,
    /// Adapter freshness class.
    pub adapter_freshness_class: BoundaryFreshnessClass,
    /// Stable adapter freshness token.
    pub adapter_freshness_token: String,
    /// Evidence refs backing the adapter claim.
    #[serde(default)]
    pub adapter_evidence_refs: Vec<String>,
}

impl AdapterConfidencePlaceholder {
    /// Builds a placeholder from typed adapter-side evidence.
    pub fn new(
        adapter_kind: AdapterKind,
        adapter_confidence_class: AdapterConfidenceClass,
        adapter_freshness_class: BoundaryFreshnessClass,
        adapter_authority_envelope_tag: impl Into<String>,
        adapter_evidence_refs: Vec<String>,
    ) -> Self {
        Self {
            adapter_kind,
            adapter_kind_token: adapter_kind.as_str().to_owned(),
            adapter_confidence_class,
            adapter_confidence_token: adapter_confidence_class.as_str().to_owned(),
            adapter_divergence_or_inference_reasons: Vec::new(),
            adapter_authority_envelope_tag: adapter_authority_envelope_tag.into(),
            adapter_freshness_class,
            adapter_freshness_token: adapter_freshness_class.as_str().to_owned(),
            adapter_evidence_refs,
        }
    }
}

/// Reusable discovery authority block shared by all protected surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveryAuthorityBlock {
    /// Discovery source token.
    pub discovery_source_token: String,
    /// Discovery freshness token.
    pub discovery_freshness_token: String,
    /// Target discovery confidence token from the alpha card.
    pub target_discovery_confidence_token: String,
    /// Resolver confidence token.
    pub resolver_confidence_token: String,
    /// Supported capability tokens advertised by the row.
    pub advertised_capability_tokens: Vec<String>,
    /// Capability tokens that remain authoritative after action decisions.
    pub authoritative_capability_subset_tokens: Vec<String>,
    /// Per-action decisions copied from the discovery row.
    pub protected_action_decisions: Vec<ProtectedActionDecisionRow>,
    /// True when any protected action is blocked.
    pub any_protected_action_blocked: bool,
    /// True when only a strict subset of advertised capabilities remains authoritative.
    pub narrowed_to_subset: bool,
}

impl DiscoveryAuthorityBlock {
    /// Projects the reusable discovery block from a beta target-discovery row.
    pub fn from_row(row: &TargetDiscoveryBetaRow) -> Self {
        let mut authoritative_capability_subset_tokens = Vec::new();
        for decision in &row.protected_action_decisions {
            if decision.decision.is_blocked() {
                continue;
            }
            if let Some(capability) = decision.action.required_capability() {
                let token = capability.as_str().to_owned();
                if row.supported_capabilities.contains(&capability)
                    && !authoritative_capability_subset_tokens.contains(&token)
                {
                    authoritative_capability_subset_tokens.push(token);
                }
            }
        }
        let any_protected_action_blocked = row
            .protected_action_decisions
            .iter()
            .any(|decision| decision.decision.is_blocked());
        let narrowed_to_subset =
            authoritative_capability_subset_tokens.len() < row.supported_capability_tokens.len();
        Self {
            discovery_source_token: row.discovery_source_token.clone(),
            discovery_freshness_token: row.discovery_freshness_token.clone(),
            target_discovery_confidence_token: row.alpha_discovery_confidence_token.clone(),
            resolver_confidence_token: row.target_confidence_level_token.clone(),
            advertised_capability_tokens: row.supported_capability_tokens.clone(),
            authoritative_capability_subset_tokens,
            protected_action_decisions: row.protected_action_decisions.clone(),
            any_protected_action_blocked,
            narrowed_to_subset,
        }
    }

    /// Returns the decision for one protected action.
    pub fn decision_for(
        &self,
        action: ProtectedActionClass,
    ) -> Option<&ProtectedActionDecisionRow> {
        self.protected_action_decisions
            .iter()
            .find(|row| row.action == action)
    }
}

/// Normalized host, user, container, workspace, and route chips.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundaryIdentityChips {
    /// Stable host chip token.
    pub host_identity_chip: String,
    /// Stable user or identity-mode chip token.
    pub user_identity_chip: String,
    /// Container chip when a container boundary exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_identity_chip: Option<String>,
    /// Workspace chip.
    pub workspace_identity_chip: String,
    /// Route chip.
    pub route_identity_chip: String,
}

impl HostBoundaryIdentityChips {
    /// Builds normalized identity chips from an execution context and route.
    pub fn from_context(context: &ExecutionContext, route: ActionRouteClass) -> Self {
        let container_identity_chip = match context.target_identity.target_class {
            TargetClass::ContainerLocal | TargetClass::Devcontainer => {
                Some(context.target_identity.canonical_target_id.clone())
            }
            _ => None,
        };
        Self {
            host_identity_chip: context.target_identity.canonical_target_id.clone(),
            user_identity_chip: identity_mode_token(context).to_owned(),
            container_identity_chip,
            workspace_identity_chip: context.invocation_subject.workspace_id.clone(),
            route_identity_chip: route.as_str().to_owned(),
        }
    }
}

/// Managed lifecycle projection embedded in a host-boundary truth record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedLifecycleTruth {
    /// Managed lifecycle state token from the verification matrix.
    pub managed_workspace_lifecycle_state: BoundaryManagedLifecycleState,
    /// Stable managed lifecycle state token.
    pub managed_workspace_lifecycle_state_token: String,
    /// Reviewer-facing lifecycle label.
    pub managed_workspace_reviewer_label: ManagedWorkspaceReviewerLabel,
    /// Stable reviewer-label token.
    pub managed_workspace_reviewer_label_token: String,
    /// Current beta lifecycle state token.
    pub beta_lifecycle_state_token: String,
    /// Local-editing continuity token.
    pub local_editing_continuity_token: String,
    /// Activation-budget summary ref.
    pub activation_budget_summary_ref: String,
    /// Optional expiry reason class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_reason_class: Option<ExpiryReasonClass>,
    /// Optional expiry reason token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_reason_token: Option<String>,
    /// Optional local-only-continuation reason class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only_continuation_reason_class: Option<LocalOnlyContinuationReasonClass>,
    /// Optional local-only-continuation reason token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only_continuation_reason_token: Option<String>,
    /// Ordered lineage tokens exported without privileged tooling.
    pub lifecycle_lineage_tokens: Vec<String>,
    /// Lifecycle record refs backing this projection.
    pub lifecycle_evidence_refs: Vec<String>,
}

impl ManagedLifecycleTruth {
    /// Projects managed lifecycle truth from the beta lifecycle record.
    pub fn from_record(
        record: &ManagedWorkspaceLifecycleBetaRecord,
        activation_budget_summary_ref: impl Into<String>,
    ) -> Self {
        let managed_workspace_lifecycle_state = lifecycle_state_from_beta(record.current_state);
        let managed_workspace_reviewer_label = reviewer_label_from_beta(record.current_state);
        let lifecycle_lineage_tokens = record
            .lineage
            .iter()
            .map(|entry| format!("{}->{}", entry.phase_token, entry.state_token))
            .collect::<Vec<_>>();
        let lifecycle_evidence_refs = vec![record.row_id.clone()];
        Self {
            managed_workspace_lifecycle_state,
            managed_workspace_lifecycle_state_token: managed_workspace_lifecycle_state
                .as_str()
                .to_owned(),
            managed_workspace_reviewer_label,
            managed_workspace_reviewer_label_token: managed_workspace_reviewer_label
                .as_str()
                .to_owned(),
            beta_lifecycle_state_token: record.current_state_token.clone(),
            local_editing_continuity_token: record.local_editing_continuity_token.clone(),
            activation_budget_summary_ref: activation_budget_summary_ref.into(),
            expiry_reason_class: None,
            expiry_reason_token: None,
            local_only_continuation_reason_class: None,
            local_only_continuation_reason_token: None,
            lifecycle_lineage_tokens,
            lifecycle_evidence_refs,
        }
    }

    /// Attaches an expiry reason token.
    pub fn with_expiry_reason(mut self, reason: ExpiryReasonClass) -> Self {
        self.expiry_reason_class = Some(reason);
        self.expiry_reason_token = Some(reason.as_str().to_owned());
        self
    }

    /// Attaches a local-only continuation reason token.
    pub fn with_local_only_continuation_reason(
        mut self,
        reason: LocalOnlyContinuationReasonClass,
    ) -> Self {
        self.local_only_continuation_reason_class = Some(reason);
        self.local_only_continuation_reason_token = Some(reason.as_str().to_owned());
        self.managed_workspace_reviewer_label =
            ManagedWorkspaceReviewerLabel::LocalOnlyContinuation;
        self.managed_workspace_reviewer_label_token =
            ManagedWorkspaceReviewerLabel::LocalOnlyContinuation
                .as_str()
                .to_owned();
        self
    }
}

/// Options used when minting a host-boundary truth record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostBoundaryTruthOptions {
    /// Stable truth record id.
    pub record_id: String,
    /// Invocation-session id shared with route truth.
    pub invocation_session_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Surface that first requested the record.
    pub surface: HostBoundarySurfaceClass,
    /// Optional origin override.
    pub action_origin_class: Option<ActionOriginClass>,
    /// Optional target override.
    pub action_target_class: Option<ActionTargetClass>,
    /// Optional route override.
    pub action_route_class: Option<ActionRouteClass>,
    /// Optional exposure override.
    pub action_exposure_class: Option<ActionExposureClass>,
    /// Authority linkage class.
    pub authority_linkage_class: AuthorityLinkageClass,
    /// Wrong-target correction class.
    pub wrong_target_correction_class: WrongTargetCorrectionClass,
    /// Reapproval requirement class.
    pub reapproval_requirement_class: ReapprovalRequirementClass,
    /// Prior target ref when correction or drift occurred.
    pub prior_target_ref: Option<String>,
    /// Prior route class when correction or drift occurred.
    pub prior_route_class: Option<ActionRouteClass>,
    /// Route-change reason.
    pub route_change_reason_code: RouteChangeReasonCode,
    /// Repair hook ref shown when reapproval is required.
    pub repair_hook_ref: Option<String>,
    /// Target evidence refs.
    pub evidence_refs: Vec<String>,
    /// Adapter confidence override.
    pub adapter_confidence_placeholder: Option<AdapterConfidencePlaceholder>,
    /// Redaction class.
    pub redaction_class: BoundaryRedactionClass,
    /// Export-inclusion posture.
    pub export_inclusion_posture: ExportInclusionPosture,
}

impl HostBoundaryTruthOptions {
    /// Builds default options for one truth record and surface.
    pub fn new(
        record_id: impl Into<String>,
        invocation_session_id: impl Into<String>,
        generated_at: impl Into<String>,
        surface: HostBoundarySurfaceClass,
    ) -> Self {
        Self {
            record_id: record_id.into(),
            invocation_session_id: invocation_session_id.into(),
            generated_at: generated_at.into(),
            surface,
            action_origin_class: None,
            action_target_class: None,
            action_route_class: None,
            action_exposure_class: None,
            authority_linkage_class: AuthorityLinkageClass::LocalUserKeystrokeAuthority,
            wrong_target_correction_class: WrongTargetCorrectionClass::NoCorrectionNeeded,
            reapproval_requirement_class: ReapprovalRequirementClass::NoReapprovalRequired,
            prior_target_ref: None,
            prior_route_class: None,
            route_change_reason_code: RouteChangeReasonCode::CanonicalNoRouteChange,
            repair_hook_ref: None,
            evidence_refs: Vec::new(),
            adapter_confidence_placeholder: None,
            redaction_class: BoundaryRedactionClass::MetadataAndHashesOnly,
            export_inclusion_posture: ExportInclusionPosture::MetadataSafeDefault,
        }
    }
}

/// One reusable host-boundary truth record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundaryTruthRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable truth record id.
    pub record_id: String,
    /// Invocation-session id shared with route truth.
    pub invocation_session_id: String,
    /// Source execution context id.
    pub execution_context_ref: String,
    /// Workspace id.
    pub workspace_ref: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Surface that first requested this truth record.
    pub source_surface: HostBoundarySurfaceClass,
    /// Stable source-surface token.
    pub source_surface_token: String,
    /// Resolved canonical target id.
    pub target_ref: String,
    /// Execution target class token.
    pub target_class_token: String,
    /// Action origin class.
    pub action_origin_class: ActionOriginClass,
    /// Stable action-origin token.
    pub action_origin_token: String,
    /// Action target class.
    pub action_target_class: ActionTargetClass,
    /// Stable action-target token.
    pub action_target_token: String,
    /// Action route class.
    pub action_route_class: ActionRouteClass,
    /// Stable action-route token.
    pub action_route_token: String,
    /// Action exposure class.
    pub action_exposure_class: ActionExposureClass,
    /// Stable action-exposure token.
    pub action_exposure_token: String,
    /// Route-change reason.
    pub route_change_reason_code: RouteChangeReasonCode,
    /// Stable route-change reason token.
    pub route_change_reason_token: String,
    /// Authority linkage class.
    pub authority_linkage_class: AuthorityLinkageClass,
    /// Stable authority-linkage token.
    pub authority_linkage_token: String,
    /// Normalized identity chips.
    pub identity_chips: HostBoundaryIdentityChips,
    /// Ordered host-boundary cue stack.
    pub host_boundary_cue_stack: Vec<HostBoundaryCueClass>,
    /// Stable host-boundary cue tokens.
    pub host_boundary_cue_stack_tokens: Vec<String>,
    /// True when boundary cues must be visible.
    pub host_boundary_visible: bool,
    /// Reusable discovery authority block.
    pub discovery: DiscoveryAuthorityBlock,
    /// Adapter confidence placeholder when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_confidence_placeholder: Option<AdapterConfidencePlaceholder>,
    /// Target reachability class.
    pub reachability_class: BoundaryReachabilityClass,
    /// Stable reachability token.
    pub reachability_token: String,
    /// Boundary freshness class.
    pub freshness_class: BoundaryFreshnessClass,
    /// Stable boundary freshness token.
    pub freshness_token: String,
    /// Managed lifecycle projection when the target is managed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_lifecycle: Option<ManagedLifecycleTruth>,
    /// Wrong-target correction class.
    pub wrong_target_correction_class: WrongTargetCorrectionClass,
    /// Stable wrong-target correction token.
    pub wrong_target_correction_token: String,
    /// Reapproval requirement class.
    pub reapproval_requirement_class: ReapprovalRequirementClass,
    /// Stable reapproval requirement token.
    pub reapproval_requirement_token: String,
    /// Prior target ref when correction or drift occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_target_ref: Option<String>,
    /// Prior route token when correction or drift occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_route_token: Option<String>,
    /// Repair hook ref shown when reapproval is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_hook_ref: Option<String>,
    /// Target evidence refs.
    #[serde(default)]
    pub target_evidence_refs: Vec<String>,
    /// Policy epoch captured for review-vs-commit drift checks.
    pub policy_epoch: u64,
    /// Trust state token captured for review-vs-commit drift checks.
    pub trust_state_token: String,
    /// Redaction class.
    pub redaction_class: BoundaryRedactionClass,
    /// Stable redaction token.
    pub redaction_token: String,
    /// Export-inclusion posture.
    pub export_inclusion_posture: ExportInclusionPosture,
    /// Stable export-inclusion token.
    pub export_inclusion_posture_token: String,
    /// True because raw hostnames, paths, command lines, env bodies, and secrets are excluded.
    pub redaction_safe: bool,
}

impl HostBoundaryTruthRecord {
    /// Builds a truth record from an execution context and optional lifecycle row.
    pub fn from_context(
        context: &ExecutionContext,
        lifecycle: Option<&ManagedWorkspaceLifecycleBetaRecord>,
        options: HostBoundaryTruthOptions,
    ) -> Self {
        let card = TargetConfidenceCard::from_context(context);
        let discovery_row = TargetDiscoveryBetaRow::from_card_and_context(&card, context);
        Self::from_context_and_discovery(context, &discovery_row, lifecycle, options)
    }

    /// Builds a truth record from an execution context and an already-minted discovery row.
    pub fn from_context_and_discovery(
        context: &ExecutionContext,
        discovery_row: &TargetDiscoveryBetaRow,
        lifecycle: Option<&ManagedWorkspaceLifecycleBetaRecord>,
        options: HostBoundaryTruthOptions,
    ) -> Self {
        let action_target_class = options
            .action_target_class
            .unwrap_or_else(|| action_target_for_context(context));
        let action_route_class = options
            .action_route_class
            .unwrap_or_else(|| action_route_for_context(context, options.surface));
        let action_exposure_class = options
            .action_exposure_class
            .unwrap_or_else(|| action_exposure_for_surface(options.surface));
        let action_origin_class = options
            .action_origin_class
            .unwrap_or_else(|| action_origin_for_surface(options.surface, context));
        let cue_stack = host_boundary_cue_stack_for_target(context.target_identity.target_class);
        let cue_tokens = cue_stack
            .iter()
            .map(|cue| cue.as_str().to_owned())
            .collect::<Vec<_>>();
        let discovery = DiscoveryAuthorityBlock::from_row(discovery_row);
        let freshness_class = boundary_freshness_from_discovery(discovery_row.discovery_freshness);
        let reachability_class =
            boundary_reachability_from_context(context.target_identity.reachability_state);
        let managed_lifecycle = lifecycle.map(|record| {
            ManagedLifecycleTruth::from_record(
                record,
                format!("activation-budget:{}", stable_token(&record.workspace_ref)),
            )
        });
        let mut target_evidence_refs = vec![
            context.execution_context_id.clone(),
            discovery_row.row_id.clone(),
        ];
        if let Some(lifecycle) = &managed_lifecycle {
            target_evidence_refs.extend(lifecycle.lifecycle_evidence_refs.clone());
        }
        target_evidence_refs.extend(options.evidence_refs);
        let host_boundary_visible = !matches!(
            context.target_identity.target_class,
            TargetClass::LocalHost | TargetClass::NotebookKernelLocal
        ) || options.surface != HostBoundarySurfaceClass::SupportExport;
        let prior_route_token = options
            .prior_route_class
            .map(|route| route.as_str().to_owned());
        Self {
            record_kind: HOST_BOUNDARY_TRUTH_RECORD_KIND.to_owned(),
            schema_version: HOST_BOUNDARY_AND_LIFECYCLE_SCHEMA_VERSION,
            record_id: options.record_id,
            invocation_session_id: options.invocation_session_id,
            execution_context_ref: context.execution_context_id.clone(),
            workspace_ref: context.invocation_subject.workspace_id.clone(),
            generated_at: options.generated_at,
            source_surface: options.surface,
            source_surface_token: options.surface.as_str().to_owned(),
            target_ref: context.target_identity.canonical_target_id.clone(),
            target_class_token: context.target_identity.target_class.as_str().to_owned(),
            action_origin_class,
            action_origin_token: action_origin_class.as_str().to_owned(),
            action_target_class,
            action_target_token: action_target_class.as_str().to_owned(),
            action_route_class,
            action_route_token: action_route_class.as_str().to_owned(),
            action_exposure_class,
            action_exposure_token: action_exposure_class.as_str().to_owned(),
            route_change_reason_code: options.route_change_reason_code,
            route_change_reason_token: options.route_change_reason_code.as_str().to_owned(),
            authority_linkage_class: options.authority_linkage_class,
            authority_linkage_token: options.authority_linkage_class.as_str().to_owned(),
            identity_chips: HostBoundaryIdentityChips::from_context(context, action_route_class),
            host_boundary_cue_stack: cue_stack,
            host_boundary_cue_stack_tokens: cue_tokens,
            host_boundary_visible,
            discovery,
            adapter_confidence_placeholder: options
                .adapter_confidence_placeholder
                .or_else(|| default_adapter_confidence(context, freshness_class)),
            reachability_class,
            reachability_token: reachability_class.as_str().to_owned(),
            freshness_class,
            freshness_token: freshness_class.as_str().to_owned(),
            managed_lifecycle,
            wrong_target_correction_class: options.wrong_target_correction_class,
            wrong_target_correction_token: options
                .wrong_target_correction_class
                .as_str()
                .to_owned(),
            reapproval_requirement_class: options.reapproval_requirement_class,
            reapproval_requirement_token: options.reapproval_requirement_class.as_str().to_owned(),
            prior_target_ref: options.prior_target_ref,
            prior_route_token,
            repair_hook_ref: options.repair_hook_ref,
            target_evidence_refs,
            policy_epoch: context.policy_and_trust.policy_epoch,
            trust_state_token: trust_state_token(context.policy_and_trust.trust_state).to_owned(),
            redaction_class: options.redaction_class,
            redaction_token: options.redaction_class.as_str().to_owned(),
            export_inclusion_posture: options.export_inclusion_posture,
            export_inclusion_posture_token: options.export_inclusion_posture.as_str().to_owned(),
            redaction_safe: true,
        }
    }

    /// Returns validation issues that would make this record overclaim truth.
    pub fn validate(&self) -> Vec<HostBoundaryTruthViolation> {
        let mut violations = Vec::new();
        if self.record_kind != HOST_BOUNDARY_TRUTH_RECORD_KIND {
            violations.push(HostBoundaryTruthViolation::new(
                "unexpected_record_kind",
                "record_kind",
                "host-boundary truth record kind must stay canonical",
            ));
        }
        if self.schema_version != HOST_BOUNDARY_AND_LIFECYCLE_SCHEMA_VERSION {
            violations.push(HostBoundaryTruthViolation::new(
                "unexpected_schema_version",
                "schema_version",
                "host-boundary truth schema version must match this crate",
            ));
        }
        if self.host_boundary_cue_stack.is_empty() {
            violations.push(HostBoundaryTruthViolation::new(
                "empty_host_boundary_cue_stack",
                "host_boundary_cue_stack",
                "every host-boundary truth record must carry at least one cue",
            ));
        }
        let expected_tokens = self
            .host_boundary_cue_stack
            .iter()
            .map(|cue| cue.as_str().to_owned())
            .collect::<Vec<_>>();
        if expected_tokens != self.host_boundary_cue_stack_tokens {
            violations.push(HostBoundaryTruthViolation::new(
                "host_boundary_cue_tokens_mismatch",
                "host_boundary_cue_stack_tokens",
                "cue tokens must mirror the typed cue stack",
            ));
        }
        if self.action_target_class == ActionTargetClass::ManagedWorkspaceTarget
            && self.managed_lifecycle.is_none()
        {
            violations.push(HostBoundaryTruthViolation::new(
                "managed_lifecycle_missing",
                "managed_lifecycle",
                "managed workspace target rows must carry lifecycle truth",
            ));
        }
        if self.wrong_target_correction_class != WrongTargetCorrectionClass::NoCorrectionNeeded {
            if self.prior_target_ref.is_none() {
                violations.push(HostBoundaryTruthViolation::new(
                    "prior_target_ref_missing",
                    "prior_target_ref",
                    "wrong-target correction rows must preserve the prior target ref",
                ));
            }
            if self.prior_route_token.is_none() {
                violations.push(HostBoundaryTruthViolation::new(
                    "prior_route_token_missing",
                    "prior_route_token",
                    "wrong-target correction rows must preserve the prior route token",
                ));
            }
            if self.route_change_reason_code == RouteChangeReasonCode::CanonicalNoRouteChange {
                violations.push(HostBoundaryTruthViolation::new(
                    "route_change_reason_missing",
                    "route_change_reason_code",
                    "wrong-target correction rows must carry a non-canonical route-change reason",
                ));
            }
        }
        if self.wrong_target_correction_class
            == WrongTargetCorrectionClass::BlockedPendingReapproval
            && self.reapproval_requirement_class == ReapprovalRequirementClass::NoReapprovalRequired
        {
            violations.push(HostBoundaryTruthViolation::new(
                "blocked_reapproval_without_requirement",
                "reapproval_requirement_class",
                "blocked-pending-reapproval rows must name a reapproval requirement",
            ));
        }
        if self.reapproval_requirement_class.blocks_reuse() && self.repair_hook_ref.is_none() {
            violations.push(HostBoundaryTruthViolation::new(
                "repair_hook_ref_missing",
                "repair_hook_ref",
                "rows that require reapproval must expose a repair or review hook",
            ));
        }
        if let Some(lifecycle) = &self.managed_lifecycle {
            if lifecycle.managed_workspace_reviewer_label == ManagedWorkspaceReviewerLabel::Expired
                && lifecycle.expiry_reason_class.is_none()
            {
                violations.push(HostBoundaryTruthViolation::new(
                    "expiry_reason_missing",
                    "managed_lifecycle.expiry_reason_class",
                    "expired managed-workspace rows must name an expiry reason",
                ));
            }
            if lifecycle.managed_workspace_reviewer_label
                == ManagedWorkspaceReviewerLabel::LocalOnlyContinuation
                && lifecycle.local_only_continuation_reason_class.is_none()
            {
                violations.push(HostBoundaryTruthViolation::new(
                    "local_only_continuation_reason_missing",
                    "managed_lifecycle.local_only_continuation_reason_class",
                    "local-only continuation rows must name the continuation reason",
                ));
            }
        }
        violations
    }

    /// True when a stored approval or preview cannot be reused.
    pub fn requires_reapproval_or_review(&self) -> bool {
        self.reapproval_requirement_class.blocks_reuse()
            || self.wrong_target_correction_class != WrongTargetCorrectionClass::NoCorrectionNeeded
            || self
                .discovery
                .protected_action_decisions
                .iter()
                .any(|decision| {
                    decision.decision == ProtectedActionDecisionClass::BlockedResolverUnavailable
                        || decision.decision == ProtectedActionDecisionClass::BlockedFreshnessStale
                })
    }

    /// Projects this truth record to one consumer surface.
    pub fn projection(&self, surface: HostBoundarySurfaceClass) -> HostBoundarySurfaceProjection {
        HostBoundarySurfaceProjection::from_record(self, surface)
    }

    /// Projects this truth record to every protected surface.
    pub fn all_surface_projections(&self) -> Vec<HostBoundarySurfaceProjection> {
        HostBoundarySurfaceClass::ALL
            .into_iter()
            .map(|surface| self.projection(surface))
            .collect()
    }
}

/// Validation issue emitted by host-boundary truth records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundaryTruthViolation {
    /// Stable violation code.
    pub code: String,
    /// Field path that failed validation.
    pub field_path: String,
    /// Review-safe validation message.
    pub message: String,
}

impl HostBoundaryTruthViolation {
    /// Builds one validation issue.
    pub fn new(
        code: impl Into<String>,
        field_path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            field_path: field_path.into(),
            message: message.into(),
        }
    }
}

/// Surface-specific projection that carries the same truth tokens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundarySurfaceProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Projection id.
    pub projection_id: String,
    /// Source truth record id.
    pub truth_record_ref: String,
    /// Target surface.
    pub surface: HostBoundarySurfaceClass,
    /// Stable surface token.
    pub surface_token: String,
    /// Target ref.
    pub target_ref: String,
    /// Action route token.
    pub action_route_token: String,
    /// Route label.
    pub route_label: String,
    /// Host-boundary cue tokens.
    pub host_boundary_cue_stack_tokens: Vec<String>,
    /// Identity chips.
    pub identity_chips: HostBoundaryIdentityChips,
    /// Discovery source token.
    pub discovery_source_token: String,
    /// Discovery freshness token.
    pub discovery_freshness_token: String,
    /// Resolver confidence token.
    pub resolver_confidence_token: String,
    /// Authoritative capability subset tokens.
    pub authoritative_capability_subset_tokens: Vec<String>,
    /// Optional lifecycle label token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_label_token: Option<String>,
    /// Optional lifecycle display label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_label: Option<String>,
    /// Wrong-target correction token.
    pub wrong_target_correction_token: String,
    /// Reapproval requirement token.
    pub reapproval_requirement_token: String,
    /// True when continue/commit controls may be enabled.
    pub continue_action_enabled: bool,
    /// Stable summary for UI, CLI, transcript, and export readers.
    pub summary: String,
    /// True because raw hostnames, paths, command lines, env bodies, and secrets are excluded.
    pub redaction_safe: bool,
}

impl HostBoundarySurfaceProjection {
    /// Builds one surface projection from a shared truth record.
    pub fn from_record(
        record: &HostBoundaryTruthRecord,
        surface: HostBoundarySurfaceClass,
    ) -> Self {
        let lifecycle_label_token = record
            .managed_lifecycle
            .as_ref()
            .map(|lifecycle| lifecycle.managed_workspace_reviewer_label_token.clone());
        let lifecycle_label = record.managed_lifecycle.as_ref().map(|lifecycle| {
            lifecycle
                .managed_workspace_reviewer_label
                .label()
                .to_owned()
        });
        let continue_action_enabled = !record.requires_reapproval_or_review()
            && !matches!(
                record.reachability_class,
                BoundaryReachabilityClass::Unreachable
                    | BoundaryReachabilityClass::DisabledByPolicy
                    | BoundaryReachabilityClass::ReachablePendingReauth
            )
            && record
                .managed_lifecycle
                .as_ref()
                .map(|lifecycle| {
                    lifecycle.managed_workspace_reviewer_label
                        == ManagedWorkspaceReviewerLabel::Ready
                })
                .unwrap_or(true);
        let summary = format!(
            "{} surface sees target {} via {} with boundary [{}], freshness {}, reapproval {}.",
            surface.label(),
            record.target_ref,
            record.action_route_token,
            record.host_boundary_cue_stack_tokens.join(" > "),
            record.freshness_token,
            record.reapproval_requirement_token,
        );
        Self {
            record_kind: HOST_BOUNDARY_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: HOST_BOUNDARY_AND_LIFECYCLE_SCHEMA_VERSION,
            projection_id: format!(
                "host-boundary-projection:{}:{}",
                stable_token(&record.record_id),
                surface.as_str()
            ),
            truth_record_ref: record.record_id.clone(),
            surface,
            surface_token: surface.as_str().to_owned(),
            target_ref: record.target_ref.clone(),
            action_route_token: record.action_route_token.clone(),
            route_label: record.action_route_class.label().to_owned(),
            host_boundary_cue_stack_tokens: record.host_boundary_cue_stack_tokens.clone(),
            identity_chips: record.identity_chips.clone(),
            discovery_source_token: record.discovery.discovery_source_token.clone(),
            discovery_freshness_token: record.discovery.discovery_freshness_token.clone(),
            resolver_confidence_token: record.discovery.resolver_confidence_token.clone(),
            authoritative_capability_subset_tokens: record
                .discovery
                .authoritative_capability_subset_tokens
                .clone(),
            lifecycle_label_token,
            lifecycle_label,
            wrong_target_correction_token: record.wrong_target_correction_token.clone(),
            reapproval_requirement_token: record.reapproval_requirement_token.clone(),
            continue_action_enabled,
            summary,
            redaction_safe: true,
        }
    }
}

/// Stored review binding compared against a commit-time truth record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundaryReviewBinding {
    /// Binding id.
    pub binding_id: String,
    /// Source truth record id.
    pub truth_record_ref: String,
    /// Target ref at review time.
    pub target_ref: String,
    /// Workspace ref at review time.
    pub workspace_ref: String,
    /// Action target class at review time.
    pub action_target_class: ActionTargetClass,
    /// Action route class at review time.
    pub action_route_class: ActionRouteClass,
    /// Host-boundary cue stack at review time.
    pub host_boundary_cue_stack: Vec<HostBoundaryCueClass>,
    /// Managed lifecycle state token at review time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_lifecycle_state_token: Option<String>,
    /// Policy epoch at review time.
    pub policy_epoch: u64,
    /// Authority linkage class at review time.
    pub authority_linkage_class: AuthorityLinkageClass,
}

impl HostBoundaryReviewBinding {
    /// Captures the review-time binding from a truth record.
    pub fn from_record(binding_id: impl Into<String>, record: &HostBoundaryTruthRecord) -> Self {
        Self {
            binding_id: binding_id.into(),
            truth_record_ref: record.record_id.clone(),
            target_ref: record.target_ref.clone(),
            workspace_ref: record.workspace_ref.clone(),
            action_target_class: record.action_target_class,
            action_route_class: record.action_route_class,
            host_boundary_cue_stack: record.host_boundary_cue_stack.clone(),
            managed_lifecycle_state_token: record
                .managed_lifecycle
                .as_ref()
                .map(|lifecycle| lifecycle.managed_workspace_lifecycle_state_token.clone()),
            policy_epoch: record.policy_epoch,
            authority_linkage_class: record.authority_linkage_class,
        }
    }
}

/// Field that drifted between review and commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostBoundaryDriftField {
    /// Target ref changed.
    Target,
    /// Workspace ref changed.
    Workspace,
    /// Action target class changed.
    ActionTargetClass,
    /// Action route class changed.
    ActionRouteClass,
    /// Host-boundary cue stack changed.
    HostBoundaryCueStack,
    /// Managed lifecycle state changed.
    ManagedLifecycleState,
    /// Policy epoch changed.
    PolicyEpoch,
    /// Authority linkage class changed.
    AuthorityLinkageClass,
}

impl HostBoundaryDriftField {
    /// Stable field token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Target => "target",
            Self::Workspace => "workspace",
            Self::ActionTargetClass => "action_target_class",
            Self::ActionRouteClass => "action_route_class",
            Self::HostBoundaryCueStack => "host_boundary_cue_stack",
            Self::ManagedLifecycleState => "managed_lifecycle_state",
            Self::PolicyEpoch => "policy_epoch",
            Self::AuthorityLinkageClass => "authority_linkage_class",
        }
    }
}

/// One drift row from review-vs-commit revalidation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundaryDriftRow {
    /// Drift field.
    pub field: HostBoundaryDriftField,
    /// Stable field token.
    pub field_token: String,
    /// Review-time token.
    pub review_value_token: String,
    /// Commit-time token.
    pub commit_value_token: String,
}

/// Outcome of host-boundary reapproval evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum HostBoundaryReapprovalOutcome {
    /// Commit-time truth still matches review-time truth.
    Fresh,
    /// Commit-time truth drifted and requires reapproval before commit.
    ReapprovalRequired {
        /// Drift rows explaining why reapproval is required.
        drift_rows: Vec<HostBoundaryDriftRow>,
    },
}

impl HostBoundaryReapprovalOutcome {
    /// True when the outcome requires reapproval.
    pub fn requires_reapproval(&self) -> bool {
        matches!(self, Self::ReapprovalRequired { .. })
    }

    /// Drift rows when reapproval is required; empty otherwise.
    pub fn drift_rows(&self) -> &[HostBoundaryDriftRow] {
        match self {
            Self::Fresh => &[],
            Self::ReapprovalRequired { drift_rows } => drift_rows,
        }
    }
}

/// Full review-vs-commit reapproval evaluation record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundaryReapprovalEvaluation {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Evaluation id.
    pub evaluation_id: String,
    /// Review binding.
    pub review_binding: HostBoundaryReviewBinding,
    /// Commit-time truth record ref.
    pub commit_truth_record_ref: String,
    /// Outcome.
    pub outcome: HostBoundaryReapprovalOutcome,
    /// Reapproval requirement selected for the drift.
    pub reapproval_requirement_class: ReapprovalRequirementClass,
    /// Stable reapproval requirement token.
    pub reapproval_requirement_token: String,
}

/// Evaluates whether review-time authority can be reused at commit time.
pub fn evaluate_host_boundary_reapproval(
    evaluation_id: impl Into<String>,
    review_binding: &HostBoundaryReviewBinding,
    commit_record: &HostBoundaryTruthRecord,
) -> HostBoundaryReapprovalEvaluation {
    let mut drift_rows = Vec::new();
    push_drift(
        &mut drift_rows,
        HostBoundaryDriftField::Target,
        review_binding.target_ref.clone(),
        commit_record.target_ref.clone(),
    );
    push_drift(
        &mut drift_rows,
        HostBoundaryDriftField::Workspace,
        review_binding.workspace_ref.clone(),
        commit_record.workspace_ref.clone(),
    );
    push_drift(
        &mut drift_rows,
        HostBoundaryDriftField::ActionTargetClass,
        review_binding.action_target_class.as_str().to_owned(),
        commit_record.action_target_class.as_str().to_owned(),
    );
    push_drift(
        &mut drift_rows,
        HostBoundaryDriftField::ActionRouteClass,
        review_binding.action_route_class.as_str().to_owned(),
        commit_record.action_route_class.as_str().to_owned(),
    );
    push_drift(
        &mut drift_rows,
        HostBoundaryDriftField::HostBoundaryCueStack,
        join_cue_tokens(&review_binding.host_boundary_cue_stack),
        commit_record.host_boundary_cue_stack_tokens.join("|"),
    );
    push_drift(
        &mut drift_rows,
        HostBoundaryDriftField::ManagedLifecycleState,
        review_binding
            .managed_lifecycle_state_token
            .clone()
            .unwrap_or_else(|| "<none>".to_owned()),
        commit_record
            .managed_lifecycle
            .as_ref()
            .map(|lifecycle| lifecycle.managed_workspace_lifecycle_state_token.clone())
            .unwrap_or_else(|| "<none>".to_owned()),
    );
    push_drift(
        &mut drift_rows,
        HostBoundaryDriftField::PolicyEpoch,
        review_binding.policy_epoch.to_string(),
        commit_record.policy_epoch.to_string(),
    );
    push_drift(
        &mut drift_rows,
        HostBoundaryDriftField::AuthorityLinkageClass,
        review_binding.authority_linkage_class.as_str().to_owned(),
        commit_record.authority_linkage_class.as_str().to_owned(),
    );
    let fallback = if drift_rows.is_empty() {
        ReapprovalRequirementClass::NoReapprovalRequired
    } else if commit_record.reapproval_requirement_class
        == ReapprovalRequirementClass::NoReapprovalRequired
    {
        ReapprovalRequirementClass::ApprovalTicketReissueRequired
    } else {
        commit_record.reapproval_requirement_class
    };
    let outcome = if drift_rows.is_empty() {
        HostBoundaryReapprovalOutcome::Fresh
    } else {
        HostBoundaryReapprovalOutcome::ReapprovalRequired { drift_rows }
    };
    HostBoundaryReapprovalEvaluation {
        record_kind: HOST_BOUNDARY_REAPPROVAL_EVALUATION_RECORD_KIND.to_owned(),
        schema_version: HOST_BOUNDARY_AND_LIFECYCLE_SCHEMA_VERSION,
        evaluation_id: evaluation_id.into(),
        review_binding: review_binding.clone(),
        commit_truth_record_ref: commit_record.record_id.clone(),
        outcome,
        reapproval_requirement_class: fallback,
        reapproval_requirement_token: fallback.as_str().to_owned(),
    }
}

/// Host-boundary support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBoundarySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Support export id.
    pub support_export_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Truth records included in the packet.
    pub records: Vec<HostBoundaryTruthRecord>,
    /// Support-export projections for each truth record.
    pub support_projections: Vec<HostBoundarySurfaceProjection>,
    /// Reapproval evaluations included in the packet.
    #[serde(default)]
    pub reapproval_evaluations: Vec<HostBoundaryReapprovalEvaluation>,
    /// True when any record blocks authority reuse.
    pub any_record_requires_reapproval_or_review: bool,
    /// True because raw hostnames, paths, command lines, env bodies, and secrets are excluded.
    pub redaction_safe: bool,
}

impl HostBoundarySupportExport {
    /// Builds a support export from truth records and reapproval evaluations.
    pub fn from_records(
        support_export_id: impl Into<String>,
        generated_at: impl Into<String>,
        records: Vec<HostBoundaryTruthRecord>,
        reapproval_evaluations: Vec<HostBoundaryReapprovalEvaluation>,
    ) -> Self {
        let support_projections = records
            .iter()
            .map(|record| record.projection(HostBoundarySurfaceClass::SupportExport))
            .collect::<Vec<_>>();
        let any_record_requires_reapproval_or_review = records
            .iter()
            .any(HostBoundaryTruthRecord::requires_reapproval_or_review)
            || reapproval_evaluations
                .iter()
                .any(|evaluation| evaluation.outcome.requires_reapproval());
        Self {
            record_kind: HOST_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: HOST_BOUNDARY_AND_LIFECYCLE_SCHEMA_VERSION,
            support_export_id: support_export_id.into(),
            generated_at: generated_at.into(),
            records,
            support_projections,
            reapproval_evaluations,
            any_record_requires_reapproval_or_review,
            redaction_safe: true,
        }
    }

    /// Deterministic plaintext rendering for CLI and support readers.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!("Host-boundary support export: {}\n", self.support_export_id);
        out.push_str(&format!("Generated: {}\n", self.generated_at));
        out.push_str(&format!(
            "Records: {} (reapproval-or-review: {})\n",
            self.records.len(),
            self.any_record_requires_reapproval_or_review
        ));
        for record in &self.records {
            out.push_str(&format!(
                "record={}; target={}; route={}; boundary={}; discovery={}/{}/{}; reapproval={}; correction={}\n",
                record.record_id,
                record.target_ref,
                record.action_route_token,
                record.host_boundary_cue_stack_tokens.join(">"),
                record.discovery.discovery_source_token,
                record.discovery.discovery_freshness_token,
                record.discovery.resolver_confidence_token,
                record.reapproval_requirement_token,
                record.wrong_target_correction_token,
            ));
            if let Some(lifecycle) = &record.managed_lifecycle {
                out.push_str(&format!(
                    "  lifecycle={}; reviewer_label={}; continuity={}; lineage={}\n",
                    lifecycle.managed_workspace_lifecycle_state_token,
                    lifecycle.managed_workspace_reviewer_label_token,
                    lifecycle.local_editing_continuity_token,
                    lifecycle.lifecycle_lineage_tokens.join(">"),
                ));
            }
        }
        for evaluation in &self.reapproval_evaluations {
            out.push_str(&format!(
                "evaluation={}; requirement={}; drift_count={}\n",
                evaluation.evaluation_id,
                evaluation.reapproval_requirement_token,
                evaluation.outcome.drift_rows().len(),
            ));
        }
        out
    }
}

fn action_origin_for_surface(
    surface: HostBoundarySurfaceClass,
    context: &ExecutionContext,
) -> ActionOriginClass {
    match surface {
        HostBoundarySurfaceClass::AiTool => {
            if context.target_identity.target_class == TargetClass::ManagedWorkspace {
                ActionOriginClass::AiToolCallManaged
            } else {
                ActionOriginClass::AiToolCallLocal
            }
        }
        HostBoundarySurfaceClass::BrowserHandoff => ActionOriginClass::ProviderCallbackInbound,
        HostBoundarySurfaceClass::CliHeadless => ActionOriginClass::CliInvocationLocal,
        HostBoundarySurfaceClass::Transcript => ActionOriginClass::ImportOrRestoreSession,
        _ => ActionOriginClass::UserKeystrokeLocal,
    }
}

fn action_target_for_context(context: &ExecutionContext) -> ActionTargetClass {
    match context.target_identity.target_class {
        TargetClass::LocalHost => ActionTargetClass::LocalHostTarget,
        TargetClass::SshRemote => ActionTargetClass::RemoteSshTarget,
        TargetClass::ContainerLocal => ActionTargetClass::ContainerLocalTarget,
        TargetClass::Devcontainer => ActionTargetClass::DevcontainerTarget,
        TargetClass::RemoteWorkspaceVm | TargetClass::PrebuildRuntime => {
            ActionTargetClass::RemoteAgentTarget
        }
        TargetClass::ManagedWorkspace => ActionTargetClass::ManagedWorkspaceTarget,
        TargetClass::NotebookKernelLocal => ActionTargetClass::NotebookKernelLocalTarget,
        TargetClass::NotebookKernelRemote => ActionTargetClass::NotebookKernelRemoteTarget,
        TargetClass::AiSandbox => ActionTargetClass::AiSandboxTarget,
    }
}

fn action_route_for_context(
    context: &ExecutionContext,
    surface: HostBoundarySurfaceClass,
) -> ActionRouteClass {
    if surface == HostBoundarySurfaceClass::BrowserHandoff {
        return ActionRouteClass::BrowserHandoffRoute;
    }
    match context.target_identity.target_class {
        TargetClass::LocalHost | TargetClass::NotebookKernelLocal => {
            ActionRouteClass::LocalRpcRoute
        }
        TargetClass::ContainerLocal | TargetClass::Devcontainer => ActionRouteClass::LocalRpcRoute,
        TargetClass::SshRemote => ActionRouteClass::RemoteRpcRoute,
        TargetClass::RemoteWorkspaceVm | TargetClass::NotebookKernelRemote => {
            ActionRouteClass::RemoteAgentAttachRoute
        }
        TargetClass::PrebuildRuntime | TargetClass::ManagedWorkspace | TargetClass::AiSandbox => {
            ActionRouteClass::ManagedControlPlaneRoute
        }
    }
}

fn action_exposure_for_surface(surface: HostBoundarySurfaceClass) -> ActionExposureClass {
    match surface {
        HostBoundarySurfaceClass::BrowserHandoff => ActionExposureClass::BrowserSessionVisible,
        HostBoundarySurfaceClass::Transcript | HostBoundarySurfaceClass::SupportExport => {
            ActionExposureClass::NoSideEffectLocalRead
        }
        _ => ActionExposureClass::WorkspaceVisibleMutation,
    }
}

fn host_boundary_cue_stack_for_target(target_class: TargetClass) -> Vec<HostBoundaryCueClass> {
    match target_class {
        TargetClass::LocalHost => vec![HostBoundaryCueClass::LocalHostBoundary],
        TargetClass::SshRemote => vec![
            HostBoundaryCueClass::LocalHostBoundary,
            HostBoundaryCueClass::RemoteSshBoundary,
        ],
        TargetClass::ContainerLocal => vec![
            HostBoundaryCueClass::LocalHostBoundary,
            HostBoundaryCueClass::ContainerKernelBoundary,
        ],
        TargetClass::Devcontainer => vec![
            HostBoundaryCueClass::LocalHostBoundary,
            HostBoundaryCueClass::DevcontainerBoundary,
        ],
        TargetClass::RemoteWorkspaceVm | TargetClass::PrebuildRuntime => vec![
            HostBoundaryCueClass::LocalHostBoundary,
            HostBoundaryCueClass::RemoteAgentBoundary,
        ],
        TargetClass::ManagedWorkspace => vec![
            HostBoundaryCueClass::LocalHostBoundary,
            HostBoundaryCueClass::ManagedWorkspaceBoundary,
        ],
        TargetClass::NotebookKernelLocal => vec![
            HostBoundaryCueClass::LocalHostBoundary,
            HostBoundaryCueClass::NotebookKernelBoundary,
        ],
        TargetClass::NotebookKernelRemote => vec![
            HostBoundaryCueClass::LocalHostBoundary,
            HostBoundaryCueClass::RemoteAgentBoundary,
            HostBoundaryCueClass::NotebookKernelBoundary,
        ],
        TargetClass::AiSandbox => vec![
            HostBoundaryCueClass::LocalHostBoundary,
            HostBoundaryCueClass::AiSandboxBoundary,
        ],
    }
}

fn boundary_freshness_from_discovery(discovery: DiscoveryFreshnessClass) -> BoundaryFreshnessClass {
    match discovery {
        DiscoveryFreshnessClass::FreshProbe => BoundaryFreshnessClass::AuthoritativeLive,
        DiscoveryFreshnessClass::RecentWithinSession
        | DiscoveryFreshnessClass::ImportedAuthoritative => BoundaryFreshnessClass::WarmCached,
        DiscoveryFreshnessClass::StaleImported => BoundaryFreshnessClass::Stale,
        DiscoveryFreshnessClass::Unknown => BoundaryFreshnessClass::Unknown,
    }
}

fn boundary_reachability_from_context(
    reachability: ReachabilityState,
) -> BoundaryReachabilityClass {
    match reachability {
        ReachabilityState::Reachable => BoundaryReachabilityClass::Reachable,
        ReachabilityState::Warming => BoundaryReachabilityClass::Warming,
        ReachabilityState::Degraded => BoundaryReachabilityClass::Degraded,
        ReachabilityState::Unreachable => BoundaryReachabilityClass::Unreachable,
        ReachabilityState::PolicyBlocked => BoundaryReachabilityClass::DisabledByPolicy,
    }
}

fn lifecycle_state_from_beta(state: ManagedLifecycleStateClass) -> BoundaryManagedLifecycleState {
    match state {
        ManagedLifecycleStateClass::Starting => BoundaryManagedLifecycleState::Provisioning,
        ManagedLifecycleStateClass::Live => BoundaryManagedLifecycleState::Ready,
        ManagedLifecycleStateClass::Suspended => BoundaryManagedLifecycleState::IdleSuspended,
        ManagedLifecycleStateClass::Resuming => BoundaryManagedLifecycleState::Warming,
        ManagedLifecycleStateClass::Degraded => BoundaryManagedLifecycleState::Recovering,
        ManagedLifecycleStateClass::ReconnectRequired => BoundaryManagedLifecycleState::Recovering,
        ManagedLifecycleStateClass::Retiring => BoundaryManagedLifecycleState::Retiring,
        ManagedLifecycleStateClass::Retired => BoundaryManagedLifecycleState::Retired,
    }
}

fn reviewer_label_from_beta(state: ManagedLifecycleStateClass) -> ManagedWorkspaceReviewerLabel {
    match state {
        ManagedLifecycleStateClass::Starting | ManagedLifecycleStateClass::Resuming => {
            ManagedWorkspaceReviewerLabel::Warming
        }
        ManagedLifecycleStateClass::Live => ManagedWorkspaceReviewerLabel::Ready,
        ManagedLifecycleStateClass::Suspended => ManagedWorkspaceReviewerLabel::Suspended,
        ManagedLifecycleStateClass::Degraded | ManagedLifecycleStateClass::ReconnectRequired => {
            ManagedWorkspaceReviewerLabel::Degraded
        }
        ManagedLifecycleStateClass::Retiring | ManagedLifecycleStateClass::Retired => {
            ManagedWorkspaceReviewerLabel::Expired
        }
    }
}

fn default_adapter_confidence(
    context: &ExecutionContext,
    freshness: BoundaryFreshnessClass,
) -> Option<AdapterConfidencePlaceholder> {
    let adapter_kind = match context.target_identity.target_class {
        TargetClass::SshRemote => AdapterKind::RemoteSshAdapter,
        TargetClass::RemoteWorkspaceVm | TargetClass::PrebuildRuntime => {
            AdapterKind::RemoteAgentAttach
        }
        TargetClass::ManagedWorkspace => AdapterKind::ManagedWorkspaceAdapter,
        TargetClass::NotebookKernelRemote => AdapterKind::NotebookKernelAdapter,
        TargetClass::AiSandbox => AdapterKind::AiSandboxAdapter,
        _ => return None,
    };
    let confidence = match freshness {
        BoundaryFreshnessClass::AuthoritativeLive => {
            AdapterConfidenceClass::AdapterAuthoritativeMatch
        }
        BoundaryFreshnessClass::WarmCached => AdapterConfidenceClass::AdapterProbedConsistent,
        BoundaryFreshnessClass::DegradedCached | BoundaryFreshnessClass::Stale => {
            AdapterConfidenceClass::AdapterProbedDivergent
        }
        BoundaryFreshnessClass::Unknown => AdapterConfidenceClass::AdapterInferredFromSession,
    };
    Some(AdapterConfidencePlaceholder::new(
        adapter_kind,
        confidence,
        freshness,
        "projected_from_execution",
        vec![context.execution_context_id.clone()],
    ))
}

fn identity_mode_token(context: &ExecutionContext) -> &'static str {
    context.policy_and_trust.identity_mode.as_str()
}

fn trust_state_token(state: TrustState) -> &'static str {
    match state {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
        TrustState::PendingEvaluation => "pending_evaluation",
    }
}

fn join_cue_tokens(cues: &[HostBoundaryCueClass]) -> String {
    cues.iter()
        .map(|cue| cue.as_str())
        .collect::<Vec<_>>()
        .join("|")
}

fn push_drift(
    rows: &mut Vec<HostBoundaryDriftRow>,
    field: HostBoundaryDriftField,
    review_value_token: String,
    commit_value_token: String,
) {
    if review_value_token != commit_value_token {
        rows.push(HostBoundaryDriftRow {
            field,
            field_token: field.as_str().to_owned(),
            review_value_token,
            commit_value_token,
        });
    }
}

fn stable_token(raw: &str) -> String {
    let mut token = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            token.push(ch.to_ascii_lowercase());
        } else if !token.ends_with('_') {
            token.push('_');
        }
    }
    let token = token.trim_matches('_').to_owned();
    if token.is_empty() {
        "unnamed".to_owned()
    } else {
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_context::{
        CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
        ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
        ToolchainClass,
    };

    fn resolver() -> ExecutionContextResolver {
        ExecutionContextResolver::new(ExecutionContextResolverConfig {
            workspace_id: "workspace:host-boundary".to_owned(),
            profile_id: Some("profile:default".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 1,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some("/workspace".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "capsule:host-boundary".to_owned(),
                capsule_hash: "sha256:host-boundary".to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "host-boundary-test".to_owned(),
        })
    }

    #[test]
    fn local_context_projects_one_truth_to_every_surface() {
        let mut resolver = resolver();
        let context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run",
            TrustState::Trusted,
            "2026-05-17T00:00:00Z",
        ));
        let record = HostBoundaryTruthRecord::from_context(
            &context,
            None,
            HostBoundaryTruthOptions::new(
                "host-boundary:local",
                "inv:local",
                "2026-05-17T00:00:01Z",
                HostBoundarySurfaceClass::Task,
            ),
        );
        assert!(record.validate().is_empty());
        let projections = record.all_surface_projections();
        assert_eq!(projections.len(), HostBoundarySurfaceClass::ALL.len());
        for projection in projections {
            assert_eq!(projection.truth_record_ref, record.record_id);
            assert_eq!(
                projection.host_boundary_cue_stack_tokens,
                record.host_boundary_cue_stack_tokens
            );
            assert_eq!(projection.action_route_token, record.action_route_token);
            assert_eq!(
                projection.discovery_source_token,
                record.discovery.discovery_source_token
            );
        }
    }

    #[test]
    fn review_binding_requires_reapproval_when_route_drifts() {
        let mut resolver = resolver();
        let review_context = resolver.resolve(ExecutionContextRequest::task_seed(
            "task.run",
            TrustState::Trusted,
            "2026-05-17T00:00:00Z",
        ));
        let review_record = HostBoundaryTruthRecord::from_context(
            &review_context,
            None,
            HostBoundaryTruthOptions::new(
                "host-boundary:review",
                "inv:review",
                "2026-05-17T00:00:01Z",
                HostBoundarySurfaceClass::Task,
            ),
        );
        let binding = HostBoundaryReviewBinding::from_record("binding:review", &review_record);

        let mut commit_request = ExecutionContextRequest::task_seed(
            "task.run",
            TrustState::Trusted,
            "2026-05-17T00:00:02Z",
        );
        commit_request.requested_target_class = Some(TargetClass::SshRemote);
        commit_request.requested_toolchain_class = Some(ToolchainClass::BuildDriverRuntime);
        let commit_context = resolver.resolve(commit_request);
        let mut options = HostBoundaryTruthOptions::new(
            "host-boundary:commit",
            "inv:review",
            "2026-05-17T00:00:03Z",
            HostBoundarySurfaceClass::Task,
        );
        options.reapproval_requirement_class =
            ReapprovalRequirementClass::ApprovalTicketReissueRequired;
        options.repair_hook_ref = Some("review.host_boundary.reapprove".to_owned());
        options.route_change_reason_code = RouteChangeReasonCode::RouteChangedWrongTargetDetected;
        options.wrong_target_correction_class =
            WrongTargetCorrectionClass::BlockedPendingReapproval;
        options.prior_target_ref = Some(review_record.target_ref.clone());
        options.prior_route_class = Some(review_record.action_route_class);
        let commit_record = HostBoundaryTruthRecord::from_context(&commit_context, None, options);
        let evaluation =
            evaluate_host_boundary_reapproval("eval:route-drift", &binding, &commit_record);
        assert!(evaluation.outcome.requires_reapproval());
        assert!(evaluation
            .outcome
            .drift_rows()
            .iter()
            .any(|row| row.field == HostBoundaryDriftField::Target));
    }
}
