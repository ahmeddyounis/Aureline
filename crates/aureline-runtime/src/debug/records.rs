use serde::{Deserialize, Serialize};

/// Integer schema version for debug-session lifecycle records.
pub const DEBUG_SESSION_LIFECYCLE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for a debug-session snapshot record.
pub const DEBUG_SESSION_RECORD_KIND: &str = "debug_session_record";

/// Stable record-kind tag for a debug-session lifecycle event record.
pub const DEBUG_SESSION_EVENT_RECORD_KIND: &str = "debug_session_event_record";

/// Stable record-kind tag for the debug-session support-export packet.
pub const DEBUG_SESSION_SUPPORT_PACKET_RECORD_KIND: &str = "debug_session_support_packet_record";

/// Typed state for one supervised debug session.
///
/// The state machine is intentionally explicit: every transition has a
/// dedicated label so shell, support export, and release evidence consumers
/// never need to infer whether a session is still warming, actively serving,
/// or being rebuilt after a crash.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugSessionStateClass {
    /// Launch or attach was requested; adapter has not yet started.
    LaunchRequested,
    /// Adapter process started and the supervisor is negotiating capabilities.
    NegotiatingCapabilities,
    /// Negotiation succeeded and the session is preparing target attach/launch.
    HandshakeComplete,
    /// Session is attached to a running target and ready to serve requests.
    AttachedRunning,
    /// Session launched the target and the target is running under debug.
    LaunchedRunning,
    /// Target is paused (breakpoint, exception, manual pause).
    Paused,
    /// Adapter exited or stalled; reconnect is scheduled inside budget.
    Reconnecting,
    /// Session is running with narrowed capability after a recoverable fault.
    Degraded,
    /// Session exceeded its restart budget and is paused until repair.
    Quarantined,
    /// Session was terminated cleanly by the user, supervisor, or adapter.
    Terminated,
}

impl DebugSessionStateClass {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchRequested => "launch_requested",
            Self::NegotiatingCapabilities => "negotiating_capabilities",
            Self::HandshakeComplete => "handshake_complete",
            Self::AttachedRunning => "attached_running",
            Self::LaunchedRunning => "launched_running",
            Self::Paused => "paused",
            Self::Reconnecting => "reconnecting",
            Self::Degraded => "degraded",
            Self::Quarantined => "quarantined",
            Self::Terminated => "terminated",
        }
    }

    /// Returns true when shell / status surfaces MUST disclose the state to
    /// the user. Steady-state running, paused, and cleanly terminated
    /// sessions are silent; reconnect, degraded, and quarantine states must
    /// surface to the user so debug evidence cannot drift silently.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(
            self,
            Self::AttachedRunning
                | Self::LaunchedRunning
                | Self::Paused
                | Self::Terminated
        )
    }

    /// Returns true when the session is in a steady-state running posture.
    pub const fn is_running(self) -> bool {
        matches!(self, Self::AttachedRunning | Self::LaunchedRunning)
    }
}

/// Launch versus attach posture for a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugSessionMode {
    /// Adapter launches the debugged target.
    Launch,
    /// Adapter attaches to an already-running target.
    Attach,
    /// Supervisor is reconnecting a previously launched / attached session.
    Reconnect,
}

impl DebugSessionMode {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Launch => "launch",
            Self::Attach => "attach",
            Self::Reconnect => "reconnect",
        }
    }
}

/// Transport class used to reach the debug adapter process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugAdapterTransportClass {
    /// Adapter runs as a local sidecar speaking DAP over stdio.
    LocalSidecarStdio,
    /// Adapter runs as a local sidecar reachable over a loopback DAP socket.
    LocalSidecarLoopbackSocket,
    /// Adapter runs on a managed / remote host reached over the platform's
    /// connector.
    ManagedConnector,
}

impl DebugAdapterTransportClass {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSidecarStdio => "local_sidecar_stdio",
            Self::LocalSidecarLoopbackSocket => "local_sidecar_loopback_socket",
            Self::ManagedConnector => "managed_connector",
        }
    }
}

/// Capability class negotiated between supervisor and adapter.
///
/// The closed vocabulary lets capability downgrades survive into the support
/// export instead of becoming free-form strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugAdapterCapabilityClass {
    /// Set / clear function breakpoints.
    FunctionBreakpoints,
    /// Set / clear conditional breakpoints.
    ConditionalBreakpoints,
    /// Hit-count breakpoints.
    HitCountBreakpoints,
    /// Log-point (tracepoint) breakpoints.
    LogPoints,
    /// Data / watch breakpoints.
    DataBreakpoints,
    /// Exception breakpoints with filters.
    ExceptionBreakpointFilters,
    /// Restart of the running target without relaunching the adapter.
    TargetRestartRequest,
    /// Terminate request distinct from disconnect.
    TerminateRequest,
    /// Step-back and reverse-continue (replay-class debugging).
    ReverseExecution,
    /// Hot-reload code into the running target.
    HotReload,
    /// Inline breakpoint locations response.
    BreakpointLocations,
    /// Stepping granularity beyond statement (instruction / line).
    GranularityStepping,
    /// Modules-loaded events.
    ModulesEvents,
    /// Loaded-source events.
    LoadedSourcesEvents,
    /// Read-memory / write-memory requests.
    MemoryAccess,
}

impl DebugAdapterCapabilityClass {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FunctionBreakpoints => "function_breakpoints",
            Self::ConditionalBreakpoints => "conditional_breakpoints",
            Self::HitCountBreakpoints => "hit_count_breakpoints",
            Self::LogPoints => "log_points",
            Self::DataBreakpoints => "data_breakpoints",
            Self::ExceptionBreakpointFilters => "exception_breakpoint_filters",
            Self::TargetRestartRequest => "target_restart_request",
            Self::TerminateRequest => "terminate_request",
            Self::ReverseExecution => "reverse_execution",
            Self::HotReload => "hot_reload",
            Self::BreakpointLocations => "breakpoint_locations",
            Self::GranularityStepping => "granularity_stepping",
            Self::ModulesEvents => "modules_events",
            Self::LoadedSourcesEvents => "loaded_sources_events",
            Self::MemoryAccess => "memory_access",
        }
    }
}

/// Outcome of the supervisor / adapter capability negotiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugAdapterNegotiationOutcome {
    /// Adapter accepted every requested capability.
    AgreedFull,
    /// Adapter accepted the session but dropped at least one requested
    /// capability; the supervisor records the dropped set.
    AgreedWithCapabilityDowngrade,
    /// Adapter refused the session because the protocol version is
    /// incompatible.
    RefusedIncompatibleProtocol,
    /// Adapter refused the session because a required capability is missing.
    RefusedMissingRequiredCapability,
    /// Adapter refused the session because policy blocks the requested
    /// transport or target.
    RefusedPolicyBlocked,
    /// Adapter did not respond inside the initialize timeout.
    RefusedInitializeTimeout,
}

impl DebugAdapterNegotiationOutcome {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AgreedFull => "agreed_full",
            Self::AgreedWithCapabilityDowngrade => "agreed_with_capability_downgrade",
            Self::RefusedIncompatibleProtocol => "refused_incompatible_protocol",
            Self::RefusedMissingRequiredCapability => "refused_missing_required_capability",
            Self::RefusedPolicyBlocked => "refused_policy_blocked",
            Self::RefusedInitializeTimeout => "refused_initialize_timeout",
        }
    }

    /// Returns true when the negotiation outcome lets the session proceed.
    pub const fn permits_session(self) -> bool {
        matches!(self, Self::AgreedFull | Self::AgreedWithCapabilityDowngrade)
    }
}

/// Why a session exited or stalled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugSessionExitReasonClass {
    /// Session ended cleanly.
    CleanTermination,
    /// User requested termination from the shell.
    UserRequestedTermination,
    /// Supervisor requested termination (e.g. workspace shutdown).
    SupervisorRequestedTermination,
    /// Adapter process crashed with an unhandled panic / error.
    AdapterCrashUnhandled,
    /// Adapter process exited from a signal.
    AdapterCrashSignal,
    /// Adapter sent a frame that violated the DAP protocol.
    AdapterProtocolViolation,
    /// Adapter did not initialize inside the configured timeout.
    AdapterInitializeTimeout,
    /// Adapter exceeded the configured resource quota.
    AdapterResourceQuotaExceeded,
    /// Watchdog killed the adapter after a stall.
    WatchdogForcedKillAfterStall,
    /// The debugged target became unreachable.
    TargetLost,
    /// Adapter capabilities no longer match the supervisor's contract.
    ContractMismatch,
    /// Supervisor could not classify the non-clean exit.
    UnknownNoncleanExit,
}

impl DebugSessionExitReasonClass {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CleanTermination => "clean_termination",
            Self::UserRequestedTermination => "user_requested_termination",
            Self::SupervisorRequestedTermination => "supervisor_requested_termination",
            Self::AdapterCrashUnhandled => "adapter_crash_unhandled",
            Self::AdapterCrashSignal => "adapter_crash_signal",
            Self::AdapterProtocolViolation => "adapter_protocol_violation",
            Self::AdapterInitializeTimeout => "adapter_initialize_timeout",
            Self::AdapterResourceQuotaExceeded => "adapter_resource_quota_exceeded",
            Self::WatchdogForcedKillAfterStall => "watchdog_forced_kill_after_stall",
            Self::TargetLost => "target_lost",
            Self::ContractMismatch => "contract_mismatch",
            Self::UnknownNoncleanExit => "unknown_nonclean_exit",
        }
    }

    /// Returns true when the exit counts toward the crash-loop restart
    /// budget. Clean and user / supervisor-requested terminations do not.
    pub const fn counts_toward_restart_budget(self) -> bool {
        !matches!(
            self,
            Self::CleanTermination
                | Self::UserRequestedTermination
                | Self::SupervisorRequestedTermination
        )
    }

    /// Returns true when the exit reason indicates a session-isolated fault
    /// rather than a host-wide problem. Beta promise: a single adapter
    /// crash MUST NOT destabilize the shell or unrelated sessions.
    pub const fn is_session_isolated_fault(self) -> bool {
        matches!(
            self,
            Self::AdapterCrashUnhandled
                | Self::AdapterCrashSignal
                | Self::AdapterProtocolViolation
                | Self::AdapterInitializeTimeout
                | Self::AdapterResourceQuotaExceeded
                | Self::WatchdogForcedKillAfterStall
                | Self::TargetLost
                | Self::ContractMismatch
                | Self::UnknownNoncleanExit
        )
    }
}

/// Why a reconnect was attempted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugSessionRestartCause {
    /// Restart followed an adapter crash inside the budget.
    AdapterCrashInsideBudget,
    /// Restart followed a watchdog stall kill inside the budget.
    WatchdogStallInsideBudget,
    /// Restart followed a protocol violation inside the budget.
    ProtocolViolationInsideBudget,
    /// Restart was requested by the user through the shell.
    UserRequested,
}

impl DebugSessionRestartCause {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterCrashInsideBudget => "adapter_crash_inside_budget",
            Self::WatchdogStallInsideBudget => "watchdog_stall_inside_budget",
            Self::ProtocolViolationInsideBudget => "protocol_violation_inside_budget",
            Self::UserRequested => "user_requested",
        }
    }
}

/// Event class in the session lifecycle lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugSessionEventClass {
    /// Launch was requested by the shell.
    LaunchRequested,
    /// Attach was requested by the shell.
    AttachRequested,
    /// Capability negotiation started.
    NegotiationStarted,
    /// Capabilities were agreed (with or without downgrade).
    CapabilitiesAgreed,
    /// Capability negotiation was refused.
    NegotiationRefused,
    /// Session became ready to serve.
    SessionReady,
    /// Target paused at a breakpoint or exception.
    Paused,
    /// Target resumed.
    Resumed,
    /// Adapter exited (clean or otherwise).
    AdapterExited,
    /// Restart was scheduled inside the budget.
    RestartScheduled,
    /// Session reconnected successfully.
    SessionReconnected,
    /// Session was moved to quarantine.
    SessionQuarantined,
    /// Session was terminated.
    SessionTerminated,
    /// Support export was emitted from this session.
    SupportExportEmitted,
}

impl DebugSessionEventClass {
    /// Stable string token for export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchRequested => "launch_requested",
            Self::AttachRequested => "attach_requested",
            Self::NegotiationStarted => "negotiation_started",
            Self::CapabilitiesAgreed => "capabilities_agreed",
            Self::NegotiationRefused => "negotiation_refused",
            Self::SessionReady => "session_ready",
            Self::Paused => "paused",
            Self::Resumed => "resumed",
            Self::AdapterExited => "adapter_exited",
            Self::RestartScheduled => "restart_scheduled",
            Self::SessionReconnected => "session_reconnected",
            Self::SessionQuarantined => "session_quarantined",
            Self::SessionTerminated => "session_terminated",
            Self::SupportExportEmitted => "support_export_emitted",
        }
    }
}

/// Exact target identity carried with every session record so support
/// evidence cannot confuse one debugged target for another.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugSessionTargetIdentity {
    /// Canonical target id from the shared execution context.
    pub canonical_target_id: String,
    /// Target-class token (mirrors the shared `TargetClass` vocabulary).
    pub target_class_token: String,
    /// Plain-language target label suitable for support export.
    pub target_label: String,
    /// Resolved working-directory digest, if the resolver settled one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory_digest: Option<String>,
    /// Optional inferior process id when attached. Omitted for launches that
    /// have not produced a process yet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inferior_process_id: Option<u32>,
}

/// Stable identity for a debug adapter implementation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugAdapterIdentity {
    /// Stable adapter id (e.g. "adapter:python:debugpy").
    pub adapter_id: String,
    /// Plain-language adapter label.
    pub adapter_label: String,
    /// Adapter implementation version reported during negotiation.
    pub adapter_version: String,
    /// DAP protocol version requested by the supervisor.
    pub requested_dap_protocol_version: String,
    /// DAP protocol version agreed during negotiation, once settled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreed_dap_protocol_version: Option<String>,
    /// Transport class used to reach this adapter.
    pub transport_class: DebugAdapterTransportClass,
    /// Stable string token for the transport class.
    pub transport_class_token: String,
}

impl DebugAdapterIdentity {
    /// Builds a new adapter identity with the transport token derived from
    /// the [`DebugAdapterTransportClass`].
    pub fn new(
        adapter_id: impl Into<String>,
        adapter_label: impl Into<String>,
        adapter_version: impl Into<String>,
        requested_dap_protocol_version: impl Into<String>,
        transport_class: DebugAdapterTransportClass,
    ) -> Self {
        Self {
            adapter_id: adapter_id.into(),
            adapter_label: adapter_label.into(),
            adapter_version: adapter_version.into(),
            requested_dap_protocol_version: requested_dap_protocol_version.into(),
            agreed_dap_protocol_version: None,
            transport_class,
            transport_class_token: transport_class.as_str().to_owned(),
        }
    }
}

/// Stable identity for one supervised debug session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugSessionIdentity {
    /// Stable session id (opaque, scoped to the supervisor).
    pub session_id: String,
    /// Workspace that owns the session.
    pub workspace_id: String,
    /// Root reference that scopes the session.
    pub root_ref: String,
    /// Language id served by the debugged target.
    pub language_id: String,
    /// Execution context anchoring target and toolchain identity.
    pub execution_context_id: String,
    /// Session mode (launch / attach / reconnect).
    pub mode: DebugSessionMode,
    /// Stable string token for the session mode.
    pub mode_token: String,
    /// Adapter identity for this session.
    pub adapter: DebugAdapterIdentity,
    /// Target identity for this session.
    pub target: DebugSessionTargetIdentity,
    /// Fault-domain id; sessions live under the session-scoped execution
    /// hosts domain so a single crash is bounded.
    pub fault_domain_id: String,
    /// Restart-budget reference shared with other session-scoped hosts.
    pub restart_budget_ref: String,
}

/// One visible lifecycle event emitted by the supervisor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugSessionLifecycleEvent {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable event id.
    pub event_id: String,
    /// Session id this event belongs to.
    pub session_id: String,
    /// Event class.
    pub event_class: DebugSessionEventClass,
    /// Stable string token for the event class.
    pub event_class_token: String,
    /// Runtime state after the event.
    pub state_after: DebugSessionStateClass,
    /// Stable string token for the state.
    pub state_after_token: String,
    /// Optional exit reason associated with the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_reason_class: Option<DebugSessionExitReasonClass>,
    /// Stable string token for the exit reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_reason_token: Option<String>,
    /// Optional restart cause associated with the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_cause: Option<DebugSessionRestartCause>,
    /// Stable string token for the restart cause.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_cause_token: Option<String>,
    /// Restart-strike count after the event.
    pub restart_strike_count: u32,
    /// Wall-clock timestamp of the event.
    pub observed_at: String,
    /// Export-safe event summary.
    pub summary: String,
}

/// Snapshot of one supervised debug session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugSessionSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable session identity.
    pub identity: DebugSessionIdentity,
    /// Current runtime state.
    pub state_class: DebugSessionStateClass,
    /// Stable string token for the current state.
    pub state_class_token: String,
    /// Negotiation outcome, once settled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negotiation_outcome: Option<DebugAdapterNegotiationOutcome>,
    /// Stable string token for the negotiation outcome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negotiation_outcome_token: Option<String>,
    /// Requested adapter capabilities (closed vocabulary).
    pub requested_capabilities: Vec<DebugAdapterCapabilityClass>,
    /// Adapter-advertised capabilities (closed vocabulary).
    pub advertised_capabilities: Vec<DebugAdapterCapabilityClass>,
    /// Capabilities the session may rely on after negotiation.
    pub agreed_capabilities: Vec<DebugAdapterCapabilityClass>,
    /// Capabilities requested but dropped during negotiation.
    pub dropped_capabilities: Vec<DebugAdapterCapabilityClass>,
    /// Restart strikes observed in the current budget window.
    pub restart_strike_count: u32,
    /// Automatic restart budget configured for this session.
    pub restart_budget_in_window: u32,
    /// Reconnect attempts observed since launch.
    pub reconnect_attempt_count: u32,
    /// Quarantine reference, when active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quarantine_ref: Option<String>,
    /// Last exit reason recorded for this session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_exit_reason_class: Option<DebugSessionExitReasonClass>,
    /// Stable string token for the last exit reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_exit_reason_token: Option<String>,
    /// Wall-clock timestamp the session was first opened.
    pub opened_at: String,
    /// Wall-clock timestamp of the last state change.
    pub last_state_change_at: String,
    /// Lifecycle event lineage in observed order.
    pub event_lineage: Vec<DebugSessionLifecycleEvent>,
    /// Export-safe summary line.
    pub summary: String,
}

impl DebugSessionSnapshot {
    /// Returns true when shell / status surfaces MUST disclose the state.
    ///
    /// Disclosure fires when the current state is not steady (running /
    /// paused / cleanly terminated), when the session is quarantined, or
    /// when the last recorded exit was a session-isolated fault. The last
    /// case keeps an unrecovered termination honest after the live state
    /// settles into `terminated`.
    pub fn requires_shell_disclosure(&self) -> bool {
        if self.state_class.requires_disclosure() || self.quarantine_ref.is_some() {
            return true;
        }
        if let Some(reason) = self.last_exit_reason_class {
            return reason.is_session_isolated_fault();
        }
        false
    }
}

/// Support-export packet carrying a workspace's debug-session evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugSessionSupportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub debug_session_lifecycle_schema_version: u32,
    /// Stable supervisor session id.
    pub supervisor_session_id: String,
    /// Workspace id covered by this packet.
    pub workspace_id: String,
    /// Session snapshots in stable order.
    pub session_rows: Vec<DebugSessionSnapshot>,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe packet summary.
    pub export_safe_summary: String,
}
