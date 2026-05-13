use std::collections::BTreeMap;
use std::fmt;

use aureline_language::{
    RouterCapabilityClass, RouterCompletenessClass, RouterFaultDomainId, RouterFreshnessClass,
    RouterHealthState, RouterLocalityClass, RouterScopeClaimClass,
};

use super::records::{
    LanguageHostEventClass, LanguageHostExitReasonClass, LanguageHostIdentity,
    LanguageHostRuntimeStateClass, LanguageHostSnapshot, LanguageHostSupervisorEvent,
    LanguageHostSupportPacket, LANGUAGE_HOST_SUPERVISION_SCHEMA_VERSION,
};

/// Scope key used to ensure one host per workspace, root, and language.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LanguageHostScopeKey {
    /// Workspace id.
    pub workspace_id: String,
    /// Root reference.
    pub root_ref: String,
    /// Language id.
    pub language_id: String,
}

impl LanguageHostScopeKey {
    /// Builds a scope key.
    pub fn new(
        workspace_id: impl Into<String>,
        root_ref: impl Into<String>,
        language_id: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            root_ref: root_ref.into(),
            language_id: language_id.into(),
        }
    }
}

/// Launch request for a language-server host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageHostLaunchSpec {
    /// Workspace id.
    pub workspace_id: String,
    /// Root reference that scopes the host.
    pub root_ref: String,
    /// Language id.
    pub language_id: String,
    /// Plain-language server label.
    pub server_label: String,
    /// Execution context anchoring target and toolchain identity.
    pub execution_context_id: String,
    /// Provider id to expose to router decisions.
    pub provider_id: String,
    /// Locality where the host runs.
    pub locality_class: RouterLocalityClass,
    /// Capabilities advertised by the host.
    pub supported_capability_classes: Vec<RouterCapabilityClass>,
}

impl LanguageHostLaunchSpec {
    /// Builds a local sidecar launch spec for a launch-language LSP.
    pub fn local_sidecar(
        workspace_id: impl Into<String>,
        root_ref: impl Into<String>,
        language_id: impl Into<String>,
        server_label: impl Into<String>,
        execution_context_id: impl Into<String>,
        provider_id: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            root_ref: root_ref.into(),
            language_id: language_id.into(),
            server_label: server_label.into(),
            execution_context_id: execution_context_id.into(),
            provider_id: provider_id.into(),
            locality_class: RouterLocalityClass::LocalSidecar,
            supported_capability_classes: launch_lsp_capabilities(),
        }
    }

    fn scope_key(&self) -> LanguageHostScopeKey {
        LanguageHostScopeKey::new(
            self.workspace_id.clone(),
            self.root_ref.clone(),
            self.language_id.clone(),
        )
    }
}

/// Supervisor configuration for language hosts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageHostSupervisorConfig {
    /// Stable supervisor session id.
    pub supervisor_session_id: String,
    /// Automatic restart budget in the strike window.
    pub automatic_restarts_in_window: u32,
}

impl Default for LanguageHostSupervisorConfig {
    fn default() -> Self {
        Self {
            supervisor_session_id: "supervisor:language-hosts:local".into(),
            automatic_restarts_in_window: 3,
        }
    }
}

/// Error returned by the language-host supervisor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanguageHostSupervisorError {
    /// No host exists for the requested host id.
    UnknownHost(String),
}

impl fmt::Display for LanguageHostSupervisorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownHost(host_id) => write!(f, "unknown language host {host_id}"),
        }
    }
}

impl std::error::Error for LanguageHostSupervisorError {}

struct SupervisorEventInput {
    host_instance_id: String,
    event_class: LanguageHostEventClass,
    state_after: LanguageHostRuntimeStateClass,
    exit_reason_class: Option<LanguageHostExitReasonClass>,
    restart_strike_count: u32,
    observed_at: String,
    summary: String,
}

/// Supervises workspace-scoped language-server hosts.
#[derive(Debug, Clone)]
pub struct LanguageHostSupervisor {
    config: LanguageHostSupervisorConfig,
    hosts: BTreeMap<String, LanguageHostSnapshot>,
    by_scope: BTreeMap<LanguageHostScopeKey, String>,
    next_host_counter: u64,
    next_event_counter: u64,
}

impl LanguageHostSupervisor {
    /// Builds a supervisor with default restart budget settings.
    pub fn new() -> Self {
        Self::with_config(LanguageHostSupervisorConfig::default())
    }

    /// Builds a supervisor with explicit settings.
    pub fn with_config(config: LanguageHostSupervisorConfig) -> Self {
        Self {
            config,
            hosts: BTreeMap::new(),
            by_scope: BTreeMap::new(),
            next_host_counter: 1,
            next_event_counter: 1,
        }
    }

    /// Launches a host for the scope, or returns the existing scoped host id.
    pub fn launch_or_reuse_host(
        &mut self,
        spec: LanguageHostLaunchSpec,
        observed_at: impl Into<String>,
    ) -> String {
        let observed_at = observed_at.into();
        let key = spec.scope_key();
        if let Some(host_id) = self.by_scope.get(&key) {
            return host_id.clone();
        }

        let host_instance_id = format!(
            "host:lsp:{}:{}",
            sanitize_id(&spec.workspace_id),
            self.next_host_counter
        );
        self.next_host_counter += 1;

        let identity = LanguageHostIdentity {
            host_instance_id: host_instance_id.clone(),
            provider_id: spec.provider_id,
            workspace_id: spec.workspace_id,
            root_ref: spec.root_ref,
            language_id: spec.language_id,
            server_label: spec.server_label,
            execution_context_id: spec.execution_context_id,
            locality_class: spec.locality_class,
            fault_domain_id: RouterFaultDomainId::SessionScopedExecutionHosts,
            restart_budget_ref: "restart_budget:session_scoped_execution_hosts:language:01".into(),
        };

        let mut snapshot = LanguageHostSnapshot {
            identity,
            runtime_state_class: LanguageHostRuntimeStateClass::Starting,
            health_state: RouterHealthState::Warming,
            freshness_class: RouterFreshnessClass::Unverified,
            scope_claim_class: RouterScopeClaimClass::ActiveWorkset,
            completeness_class: RouterCompletenessClass::PartialForClaimedScope,
            scope_limit_classes: Vec::new(),
            supported_capability_classes: spec.supported_capability_classes,
            restart_strike_count: 0,
            restart_budget_in_window: self.config.automatic_restarts_in_window,
            reconnect_attempt_count: 0,
            quarantine_ref: None,
            last_heartbeat_at: None,
            last_ready_at: None,
            last_exit_reason_class: None,
            event_lineage: Vec::new(),
            summary: "Language server launch requested; host is warming.".into(),
        };
        let event = self.event(SupervisorEventInput {
            host_instance_id: host_instance_id.clone(),
            event_class: LanguageHostEventClass::LaunchRequested,
            state_after: LanguageHostRuntimeStateClass::Starting,
            exit_reason_class: None,
            restart_strike_count: 0,
            observed_at,
            summary: "Language server launch requested.".into(),
        });
        snapshot.event_lineage.push(event);

        self.by_scope
            .insert(snapshot_scope_key(&snapshot), host_instance_id.clone());
        self.hosts.insert(host_instance_id.clone(), snapshot);
        host_instance_id
    }

    /// Marks a host heartbeat as observed.
    ///
    /// # Errors
    ///
    /// Returns [`LanguageHostSupervisorError::UnknownHost`] when the host id is
    /// not supervised by this instance.
    pub fn record_heartbeat(
        &mut self,
        host_id: &str,
        observed_at: impl Into<String>,
    ) -> Result<&LanguageHostSnapshot, LanguageHostSupervisorError> {
        let observed_at = observed_at.into();
        let state_after = {
            let snapshot = self
                .hosts
                .get(host_id)
                .ok_or_else(|| LanguageHostSupervisorError::UnknownHost(host_id.to_owned()))?;
            snapshot.runtime_state_class
        };
        let strike_count = self.host(host_id)?.restart_strike_count;
        let event = self.event(SupervisorEventInput {
            host_instance_id: host_id.to_owned(),
            event_class: LanguageHostEventClass::HeartbeatReceived,
            state_after,
            exit_reason_class: None,
            restart_strike_count: strike_count,
            observed_at: observed_at.clone(),
            summary: "Language host heartbeat observed.".into(),
        });
        let snapshot = self.host_mut(host_id)?;
        snapshot.last_heartbeat_at = Some(observed_at);
        snapshot.event_lineage.push(event);
        self.host(host_id)
    }

    /// Marks a warming or reconnecting host as ready.
    ///
    /// # Errors
    ///
    /// Returns [`LanguageHostSupervisorError::UnknownHost`] when the host id is
    /// not supervised by this instance.
    pub fn mark_ready(
        &mut self,
        host_id: &str,
        observed_at: impl Into<String>,
    ) -> Result<&LanguageHostSnapshot, LanguageHostSupervisorError> {
        let observed_at = observed_at.into();
        let was_reconnecting = matches!(
            self.host(host_id)?.runtime_state_class,
            LanguageHostRuntimeStateClass::Reconnecting
                | LanguageHostRuntimeStateClass::Degraded
                | LanguageHostRuntimeStateClass::Starting
        );
        let event_class = if was_reconnecting {
            LanguageHostEventClass::Reconnected
        } else {
            LanguageHostEventClass::HostReady
        };
        let strike_count = self.host(host_id)?.restart_strike_count;
        let event = self.event(SupervisorEventInput {
            host_instance_id: host_id.to_owned(),
            event_class,
            state_after: LanguageHostRuntimeStateClass::Ready,
            exit_reason_class: None,
            restart_strike_count: strike_count,
            observed_at: observed_at.clone(),
            summary: "Language host is ready and can serve live LSP requests.".into(),
        });

        let snapshot = self.host_mut(host_id)?;
        snapshot.runtime_state_class = LanguageHostRuntimeStateClass::Ready;
        snapshot.health_state = RouterHealthState::Ready;
        snapshot.freshness_class = RouterFreshnessClass::AuthoritativeLive;
        snapshot.completeness_class = RouterCompletenessClass::CompleteForClaimedScope;
        snapshot.last_ready_at = Some(observed_at.clone());
        snapshot.last_heartbeat_at = Some(observed_at);
        snapshot.summary = format!(
            "{} is ready for {} in {}.",
            snapshot.identity.server_label,
            snapshot.identity.language_id,
            snapshot.identity.root_ref
        );
        snapshot.event_lineage.push(event);
        self.host(host_id)
    }

    /// Records host exit and applies restart or quarantine policy.
    ///
    /// # Errors
    ///
    /// Returns [`LanguageHostSupervisorError::UnknownHost`] when the host id is
    /// not supervised by this instance.
    pub fn record_exit(
        &mut self,
        host_id: &str,
        reason: LanguageHostExitReasonClass,
        observed_at: impl Into<String>,
    ) -> Result<&LanguageHostSnapshot, LanguageHostSupervisorError> {
        let observed_at = observed_at.into();
        let mut strike_count = self.host(host_id)?.restart_strike_count;
        if reason.counts_toward_restart_budget() {
            strike_count += 1;
        }

        let exit_event = self.event(SupervisorEventInput {
            host_instance_id: host_id.to_owned(),
            event_class: LanguageHostEventClass::HostExited,
            state_after: LanguageHostRuntimeStateClass::Degraded,
            exit_reason_class: Some(reason),
            restart_strike_count: strike_count,
            observed_at: observed_at.clone(),
            summary: "Language host exit observed by supervisor.".into(),
        });

        let over_budget = reason.counts_toward_restart_budget()
            && strike_count >= self.config.automatic_restarts_in_window;

        if over_budget {
            let quarantine_ref = format!("quarantine:{}:{}", sanitize_id(host_id), strike_count);
            let quarantine_event = self.event(SupervisorEventInput {
                host_instance_id: host_id.to_owned(),
                event_class: LanguageHostEventClass::Quarantined,
                state_after: LanguageHostRuntimeStateClass::Quarantined,
                exit_reason_class: Some(reason),
                restart_strike_count: strike_count,
                observed_at,
                summary: "Language host exceeded its restart budget and entered quarantine.".into(),
            });
            let snapshot = self.host_mut(host_id)?;
            snapshot.restart_strike_count = strike_count;
            snapshot.runtime_state_class = LanguageHostRuntimeStateClass::Quarantined;
            snapshot.health_state = RouterHealthState::CrashLoopQuarantined;
            snapshot.freshness_class = RouterFreshnessClass::Unverified;
            snapshot.completeness_class = RouterCompletenessClass::UnavailableForClaimedScope;
            snapshot.reconnect_attempt_count += 1;
            snapshot.quarantine_ref = Some(quarantine_ref);
            snapshot.last_exit_reason_class = Some(reason);
            snapshot.summary = format!(
                "{} is quarantined after {} failures in the restart window.",
                snapshot.identity.server_label, strike_count
            );
            snapshot.event_lineage.push(exit_event);
            snapshot.event_lineage.push(quarantine_event);
        } else if reason.counts_toward_restart_budget() {
            let restart_event = self.event(SupervisorEventInput {
                host_instance_id: host_id.to_owned(),
                event_class: LanguageHostEventClass::RestartScheduled,
                state_after: LanguageHostRuntimeStateClass::Reconnecting,
                exit_reason_class: Some(reason),
                restart_strike_count: strike_count,
                observed_at,
                summary: "Language host restart scheduled with visible reconnecting state.".into(),
            });
            let snapshot = self.host_mut(host_id)?;
            snapshot.restart_strike_count = strike_count;
            snapshot.runtime_state_class = LanguageHostRuntimeStateClass::Reconnecting;
            snapshot.health_state = RouterHealthState::Warming;
            snapshot.freshness_class = RouterFreshnessClass::DegradedCached;
            snapshot.completeness_class = RouterCompletenessClass::PartialForClaimedScope;
            snapshot.reconnect_attempt_count += 1;
            snapshot.last_exit_reason_class = Some(reason);
            snapshot.summary = format!(
                "{} is reconnecting after a non-clean exit; cached or syntax fallback must be labeled.",
                snapshot.identity.server_label
            );
            snapshot.event_lineage.push(exit_event);
            snapshot.event_lineage.push(restart_event);
        } else {
            let shutdown_event = self.event(SupervisorEventInput {
                host_instance_id: host_id.to_owned(),
                event_class: LanguageHostEventClass::Shutdown,
                state_after: LanguageHostRuntimeStateClass::Shutdown,
                exit_reason_class: Some(reason),
                restart_strike_count: strike_count,
                observed_at,
                summary: "Language host shut down cleanly.".into(),
            });
            let snapshot = self.host_mut(host_id)?;
            snapshot.runtime_state_class = LanguageHostRuntimeStateClass::Shutdown;
            snapshot.health_state = RouterHealthState::Unavailable;
            snapshot.freshness_class = RouterFreshnessClass::Unverified;
            snapshot.completeness_class = RouterCompletenessClass::UnavailableForClaimedScope;
            snapshot.last_exit_reason_class = Some(reason);
            snapshot.summary = format!("{} shut down cleanly.", snapshot.identity.server_label);
            snapshot.event_lineage.push(exit_event);
            snapshot.event_lineage.push(shutdown_event);
        }

        self.host(host_id)
    }

    /// Marks a host unavailable without consuming the crash-loop budget.
    ///
    /// # Errors
    ///
    /// Returns [`LanguageHostSupervisorError::UnknownHost`] when the host id is
    /// not supervised by this instance.
    pub fn mark_unavailable(
        &mut self,
        host_id: &str,
        observed_at: impl Into<String>,
        summary: impl Into<String>,
    ) -> Result<&LanguageHostSnapshot, LanguageHostSupervisorError> {
        let observed_at = observed_at.into();
        let summary = summary.into();
        let strike_count = self.host(host_id)?.restart_strike_count;
        let event = self.event(SupervisorEventInput {
            host_instance_id: host_id.to_owned(),
            event_class: LanguageHostEventClass::MarkedUnavailable,
            state_after: LanguageHostRuntimeStateClass::Unavailable,
            exit_reason_class: None,
            restart_strike_count: strike_count,
            observed_at,
            summary: summary.clone(),
        });
        let snapshot = self.host_mut(host_id)?;
        snapshot.runtime_state_class = LanguageHostRuntimeStateClass::Unavailable;
        snapshot.health_state = RouterHealthState::Unavailable;
        snapshot.freshness_class = RouterFreshnessClass::Unverified;
        snapshot.completeness_class = RouterCompletenessClass::UnavailableForClaimedScope;
        snapshot.summary = summary;
        snapshot.event_lineage.push(event);
        self.host(host_id)
    }

    /// Returns a supervised host snapshot.
    pub fn snapshot(&self, host_id: &str) -> Option<&LanguageHostSnapshot> {
        self.hosts.get(host_id)
    }

    /// Returns the host id for a workspace/root/language scope.
    pub fn host_for_scope(&self, key: &LanguageHostScopeKey) -> Option<&str> {
        self.by_scope.get(key).map(String::as_str)
    }

    /// Returns all host snapshots in stable order.
    pub fn snapshots(&self) -> impl Iterator<Item = &LanguageHostSnapshot> {
        self.hosts.values()
    }

    /// Returns router-readable status rows for all supervised language hosts.
    pub fn router_host_statuses(&self) -> Vec<aureline_language::LanguageServerHostStatus> {
        self.hosts
            .values()
            .map(LanguageHostSnapshot::router_host_status)
            .collect()
    }

    /// Builds an export-safe language-host support packet for a workspace.
    pub fn support_packet(
        &self,
        workspace_id: &str,
        captured_at: impl Into<String>,
    ) -> LanguageHostSupportPacket {
        let captured_at = captured_at.into();
        let host_rows = self
            .hosts
            .values()
            .filter(|snapshot| snapshot.identity.workspace_id == workspace_id)
            .cloned()
            .collect::<Vec<_>>();
        LanguageHostSupportPacket {
            record_kind: LanguageHostSupportPacket::RECORD_KIND.into(),
            language_host_supervision_schema_version: LANGUAGE_HOST_SUPERVISION_SCHEMA_VERSION,
            supervisor_session_id: self.config.supervisor_session_id.clone(),
            workspace_id: workspace_id.to_owned(),
            export_safe_summary: format!(
                "{} language-host rows captured for workspace {}.",
                host_rows.len(),
                workspace_id
            ),
            host_rows,
            captured_at,
        }
    }

    fn host(&self, host_id: &str) -> Result<&LanguageHostSnapshot, LanguageHostSupervisorError> {
        self.hosts
            .get(host_id)
            .ok_or_else(|| LanguageHostSupervisorError::UnknownHost(host_id.to_owned()))
    }

    fn host_mut(
        &mut self,
        host_id: &str,
    ) -> Result<&mut LanguageHostSnapshot, LanguageHostSupervisorError> {
        self.hosts
            .get_mut(host_id)
            .ok_or_else(|| LanguageHostSupervisorError::UnknownHost(host_id.to_owned()))
    }

    fn event(&mut self, input: SupervisorEventInput) -> LanguageHostSupervisorEvent {
        let event = LanguageHostSupervisorEvent {
            event_id: format!("event:language-host:{}", self.next_event_counter),
            host_instance_id: input.host_instance_id,
            event_class: input.event_class,
            state_after: input.state_after,
            exit_reason_class: input.exit_reason_class,
            restart_strike_count: input.restart_strike_count,
            observed_at: input.observed_at,
            summary: input.summary,
        };
        self.next_event_counter += 1;
        event
    }
}

impl Default for LanguageHostSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

fn launch_lsp_capabilities() -> Vec<RouterCapabilityClass> {
    vec![
        RouterCapabilityClass::Definition,
        RouterCapabilityClass::Reference,
        RouterCapabilityClass::Hover,
        RouterCapabilityClass::Rename,
        RouterCapabilityClass::Completion,
        RouterCapabilityClass::Formatting,
        RouterCapabilityClass::CodeAction,
        RouterCapabilityClass::Diagnostics,
        RouterCapabilityClass::SignatureHelp,
        RouterCapabilityClass::InlineHint,
    ]
}

fn snapshot_scope_key(snapshot: &LanguageHostSnapshot) -> LanguageHostScopeKey {
    LanguageHostScopeKey::new(
        snapshot.identity.workspace_id.clone(),
        snapshot.identity.root_ref.clone(),
        snapshot.identity.language_id.clone(),
    )
}

fn sanitize_id(value: &str) -> String {
    value
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}
