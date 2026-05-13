use serde::{Deserialize, Serialize};

use aureline_language::{
    LanguageServerHostIdentity, LanguageServerHostStatus, RouterCompletenessClass,
    RouterFallbackClass, RouterFaultDomainId, RouterFreshnessClass, RouterHealthState,
    RouterLocalityClass, RouterScopeClaimClass, ScopeLimitClass,
};

/// Integer schema version for language-host supervision records.
pub const LANGUAGE_HOST_SUPERVISION_SCHEMA_VERSION: u32 = 1;

/// Runtime state of one supervised language host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguageHostRuntimeStateClass {
    /// Host launch has been requested.
    Starting,
    /// Host is ready to serve requests.
    Ready,
    /// Host is running with narrowed capability after failure.
    Degraded,
    /// Host is restarting or reconnecting after a non-clean exit.
    Reconnecting,
    /// Host cannot be used.
    Unavailable,
    /// Host exhausted its restart budget and is paused.
    Quarantined,
    /// Host was stopped intentionally.
    Shutdown,
}

impl LanguageHostRuntimeStateClass {
    /// Returns true when shell/status surfaces must show a degraded label.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Ready)
    }
}

/// Exit or failure reason observed by the supervisor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguageHostExitReasonClass {
    /// Host shut down cleanly.
    CleanShutdown,
    /// Supervisor requested stop.
    SupervisorRequestedStop,
    /// User requested stop.
    UserRequestedStop,
    /// Host crashed from an unhandled panic.
    CrashUnhandledPanic,
    /// Host crashed from a signal.
    CrashSignal,
    /// Host exceeded resource quota.
    CrashResourceQuotaExceeded,
    /// RPC contract mismatched the supervisor.
    RpcContractMismatch,
    /// Watchdog killed the host after a stall.
    WatchdogForcedKillAfterStall,
    /// Supervisor could not classify the non-clean exit.
    UnknownNoncleanExit,
}

impl LanguageHostExitReasonClass {
    /// Returns true when this reason should increment restart strikes.
    pub const fn counts_toward_restart_budget(self) -> bool {
        !matches!(
            self,
            Self::CleanShutdown | Self::SupervisorRequestedStop | Self::UserRequestedStop
        )
    }
}

/// Event class in the host lifecycle lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguageHostEventClass {
    /// Launch was requested.
    LaunchRequested,
    /// Host emitted a heartbeat.
    HeartbeatReceived,
    /// Host became ready.
    HostReady,
    /// Host exited.
    HostExited,
    /// Restart or reconnect was scheduled.
    RestartScheduled,
    /// Host entered quarantine.
    Quarantined,
    /// Host was marked unavailable.
    MarkedUnavailable,
    /// Host reconnected successfully.
    Reconnected,
    /// Host shut down intentionally.
    Shutdown,
}

/// Stable identity for one supervised language host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageHostIdentity {
    /// Host instance id.
    pub host_instance_id: String,
    /// Provider id served by this host.
    pub provider_id: String,
    /// Workspace that owns this host.
    pub workspace_id: String,
    /// Root that scoped this host.
    pub root_ref: String,
    /// Language id served by this host.
    pub language_id: String,
    /// Plain-language server label.
    pub server_label: String,
    /// Execution context anchoring target and toolchain identity.
    pub execution_context_id: String,
    /// Host locality.
    pub locality_class: RouterLocalityClass,
    /// Fault domain owning restart accounting.
    pub fault_domain_id: RouterFaultDomainId,
    /// Restart budget reference.
    pub restart_budget_ref: String,
}

impl LanguageHostIdentity {
    /// Projects runtime identity into the language-router identity shape.
    pub fn router_identity(&self) -> LanguageServerHostIdentity {
        LanguageServerHostIdentity {
            host_instance_id: self.host_instance_id.clone(),
            provider_id: self.provider_id.clone(),
            workspace_id: self.workspace_id.clone(),
            root_ref: self.root_ref.clone(),
            language_id: self.language_id.clone(),
            server_label: self.server_label.clone(),
            execution_context_id: self.execution_context_id.clone(),
            locality_class: self.locality_class,
            fault_domain_id: self.fault_domain_id,
            restart_budget_ref: self.restart_budget_ref.clone(),
        }
    }
}

/// One visible lifecycle event emitted by the supervisor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageHostSupervisorEvent {
    /// Event id.
    pub event_id: String,
    /// Host instance id.
    pub host_instance_id: String,
    /// Event class.
    pub event_class: LanguageHostEventClass,
    /// Runtime state after the event.
    pub state_after: LanguageHostRuntimeStateClass,
    /// Optional exit reason.
    pub exit_reason_class: Option<LanguageHostExitReasonClass>,
    /// Restart strike count after the event.
    pub restart_strike_count: u32,
    /// Event timestamp.
    pub observed_at: String,
    /// Export-safe event summary.
    pub summary: String,
}

/// Snapshot of one supervised language host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageHostSnapshot {
    /// Stable host identity.
    pub identity: LanguageHostIdentity,
    /// Current runtime state.
    pub runtime_state_class: LanguageHostRuntimeStateClass,
    /// Current health projected to language-provider surfaces.
    pub health_state: RouterHealthState,
    /// Current freshness projected to language-provider surfaces.
    pub freshness_class: RouterFreshnessClass,
    /// Scope this host claims to cover.
    pub scope_claim_class: RouterScopeClaimClass,
    /// Completeness for the claimed scope.
    pub completeness_class: RouterCompletenessClass,
    /// Concrete scope limits.
    pub scope_limit_classes: Vec<ScopeLimitClass>,
    /// Supported language-router capabilities.
    pub supported_capability_classes: Vec<aureline_language::RouterCapabilityClass>,
    /// Restart strikes observed in the current budget window.
    pub restart_strike_count: u32,
    /// Automatic restart budget in the window.
    pub restart_budget_in_window: u32,
    /// Reconnect attempts since launch.
    pub reconnect_attempt_count: u32,
    /// Quarantine reference, when active.
    pub quarantine_ref: Option<String>,
    /// Last observed heartbeat time.
    pub last_heartbeat_at: Option<String>,
    /// Last ready time.
    pub last_ready_at: Option<String>,
    /// Last exit reason.
    pub last_exit_reason_class: Option<LanguageHostExitReasonClass>,
    /// Lifecycle event lineage.
    pub event_lineage: Vec<LanguageHostSupervisorEvent>,
    /// Export-safe runtime summary.
    pub summary: String,
}

impl LanguageHostSnapshot {
    /// Returns true when shell/status surfaces must show this host state.
    pub fn requires_shell_disclosure(&self) -> bool {
        self.runtime_state_class.requires_disclosure()
            || self.health_state.requires_disclosure()
            || self.quarantine_ref.is_some()
    }

    /// Projects this snapshot into a language-router host status row.
    pub fn router_host_status(&self) -> LanguageServerHostStatus {
        LanguageServerHostStatus {
            identity: self.identity.router_identity(),
            health_state: self.health_state,
            freshness_class: self.freshness_class,
            scope_claim_class: self.scope_claim_class,
            completeness_class: self.completeness_class,
            scope_limit_classes: self.scope_limit_classes.clone(),
            supported_capability_classes: self.supported_capability_classes.clone(),
            restart_strike_count: self.restart_strike_count,
            quarantine_ref: self.quarantine_ref.clone(),
            fallback_class: RouterFallbackClass::ProtocolToText,
            health_summary: self.summary.clone(),
        }
    }
}

/// Support-export packet carrying language-host identity and restart lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageHostSupportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub language_host_supervision_schema_version: u32,
    /// Supervisor session id.
    pub supervisor_session_id: String,
    /// Workspace id covered by this packet.
    pub workspace_id: String,
    /// Host snapshots included in the packet.
    pub host_rows: Vec<LanguageHostSnapshot>,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe packet summary.
    pub export_safe_summary: String,
}

impl LanguageHostSupportPacket {
    /// Stable record-kind tag for language-host support packets.
    pub const RECORD_KIND: &'static str = "language_host_support_packet";
}
