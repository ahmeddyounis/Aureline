use std::collections::BTreeMap;
use std::fmt;

use super::records::{
    DebugAdapterCapabilityClass, DebugAdapterIdentity, DebugAdapterNegotiationOutcome,
    DebugAdapterTransportClass, DebugSessionEventClass, DebugSessionExitReasonClass,
    DebugSessionIdentity, DebugSessionLifecycleEvent, DebugSessionMode, DebugSessionRestartCause,
    DebugSessionSnapshot, DebugSessionStateClass, DebugSessionSupportPacket,
    DebugSessionTargetIdentity, DEBUG_SESSION_EVENT_RECORD_KIND,
    DEBUG_SESSION_LIFECYCLE_SCHEMA_VERSION, DEBUG_SESSION_RECORD_KIND,
    DEBUG_SESSION_SUPPORT_PACKET_RECORD_KIND,
};

/// Stable fault-domain id used for every session.
const SESSION_SCOPED_FAULT_DOMAIN_ID: &str = "session_scoped_execution_hosts";
/// Stable restart-budget reference used for every session.
const SESSION_RESTART_BUDGET_REF: &str =
    "restart_budget:session_scoped_execution_hosts:debug:01";

/// Launch request for a debug session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugSessionLaunchSpec {
    /// Workspace id.
    pub workspace_id: String,
    /// Root reference.
    pub root_ref: String,
    /// Language id served by the debugged target.
    pub language_id: String,
    /// Execution context anchoring target / toolchain identity.
    pub execution_context_id: String,
    /// Session mode (launch / attach).
    pub mode: DebugSessionMode,
    /// Adapter identity for this session.
    pub adapter: DebugAdapterIdentity,
    /// Exact target identity.
    pub target: DebugSessionTargetIdentity,
}

impl DebugSessionLaunchSpec {
    /// Builds a launch spec for a local stdio adapter sidecar.
    #[allow(clippy::too_many_arguments)]
    pub fn local_launch(
        workspace_id: impl Into<String>,
        root_ref: impl Into<String>,
        language_id: impl Into<String>,
        execution_context_id: impl Into<String>,
        adapter_id: impl Into<String>,
        adapter_label: impl Into<String>,
        adapter_version: impl Into<String>,
        requested_dap_protocol_version: impl Into<String>,
        target: DebugSessionTargetIdentity,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            root_ref: root_ref.into(),
            language_id: language_id.into(),
            execution_context_id: execution_context_id.into(),
            mode: DebugSessionMode::Launch,
            adapter: DebugAdapterIdentity::new(
                adapter_id,
                adapter_label,
                adapter_version,
                requested_dap_protocol_version,
                DebugAdapterTransportClass::LocalSidecarStdio,
            ),
            target,
        }
    }

    /// Builds an attach spec for a local stdio adapter sidecar.
    #[allow(clippy::too_many_arguments)]
    pub fn local_attach(
        workspace_id: impl Into<String>,
        root_ref: impl Into<String>,
        language_id: impl Into<String>,
        execution_context_id: impl Into<String>,
        adapter_id: impl Into<String>,
        adapter_label: impl Into<String>,
        adapter_version: impl Into<String>,
        requested_dap_protocol_version: impl Into<String>,
        target: DebugSessionTargetIdentity,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            root_ref: root_ref.into(),
            language_id: language_id.into(),
            execution_context_id: execution_context_id.into(),
            mode: DebugSessionMode::Attach,
            adapter: DebugAdapterIdentity::new(
                adapter_id,
                adapter_label,
                adapter_version,
                requested_dap_protocol_version,
                DebugAdapterTransportClass::LocalSidecarStdio,
            ),
            target,
        }
    }
}

/// Capability request: which capabilities the supervisor needs and which are
/// mandatory for the session to proceed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugAdapterCapabilityRequest {
    /// Capabilities the supervisor would prefer.
    pub requested: Vec<DebugAdapterCapabilityClass>,
    /// Capabilities the supervisor cannot operate without.
    pub required: Vec<DebugAdapterCapabilityClass>,
}

impl DebugAdapterCapabilityRequest {
    /// Builds a capability request from two slices.
    pub fn new(
        requested: impl IntoIterator<Item = DebugAdapterCapabilityClass>,
        required: impl IntoIterator<Item = DebugAdapterCapabilityClass>,
    ) -> Self {
        Self {
            requested: dedupe_sorted(requested),
            required: dedupe_sorted(required),
        }
    }
}

/// Capability response: what the adapter advertised.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugAdapterCapabilityResponse {
    /// Capabilities the adapter advertised.
    pub advertised: Vec<DebugAdapterCapabilityClass>,
    /// Adapter-reported DAP protocol version.
    pub agreed_dap_protocol_version: String,
}

impl DebugAdapterCapabilityResponse {
    /// Builds a capability response.
    pub fn new(
        advertised: impl IntoIterator<Item = DebugAdapterCapabilityClass>,
        agreed_dap_protocol_version: impl Into<String>,
    ) -> Self {
        Self {
            advertised: dedupe_sorted(advertised),
            agreed_dap_protocol_version: agreed_dap_protocol_version.into(),
        }
    }
}

/// Negotiation input combining request and response or a refusal reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebugAdapterNegotiationInput {
    /// Adapter responded inside the timeout.
    AdapterResponded {
        request: DebugAdapterCapabilityRequest,
        response: DebugAdapterCapabilityResponse,
    },
    /// Adapter refused with a typed reason class.
    AdapterRefused {
        request: DebugAdapterCapabilityRequest,
        outcome: DebugAdapterNegotiationOutcome,
    },
}

/// Supervisor configuration for debug-adapter hosts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DapHostSupervisorConfig {
    /// Stable supervisor session id.
    pub supervisor_session_id: String,
    /// Automatic restart budget in the strike window.
    pub automatic_restarts_in_window: u32,
}

impl Default for DapHostSupervisorConfig {
    fn default() -> Self {
        Self {
            supervisor_session_id: "supervisor:dap-host:local".into(),
            automatic_restarts_in_window: 3,
        }
    }
}

/// Error returned by the DAP host supervisor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DapHostSupervisorError {
    /// No session exists for the requested session id.
    UnknownSession(String),
    /// The session is in a state that does not allow the requested operation.
    InvalidTransition {
        session_id: String,
        from: DebugSessionStateClass,
        attempted: &'static str,
    },
    /// The negotiation refused a required capability.
    NegotiationRequiredCapabilityMissing {
        session_id: String,
        missing: Vec<DebugAdapterCapabilityClass>,
    },
}

impl fmt::Display for DapHostSupervisorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSession(id) => write!(f, "unknown debug session {id}"),
            Self::InvalidTransition {
                session_id,
                from,
                attempted,
            } => write!(
                f,
                "debug session {session_id} cannot {attempted} from state {state}",
                state = from.as_str()
            ),
            Self::NegotiationRequiredCapabilityMissing { session_id, missing } => {
                let tokens: Vec<&str> = missing.iter().map(|c| c.as_str()).collect();
                write!(
                    f,
                    "debug session {session_id} negotiation missing required capabilities: {}",
                    tokens.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for DapHostSupervisorError {}

struct EventInput {
    session_id: String,
    event_class: DebugSessionEventClass,
    state_after: DebugSessionStateClass,
    exit_reason_class: Option<DebugSessionExitReasonClass>,
    restart_cause: Option<DebugSessionRestartCause>,
    restart_strike_count: u32,
    observed_at: String,
    summary: String,
}

/// Supervises workspace-scoped DAP sessions.
#[derive(Debug, Clone)]
pub struct DapHostSupervisor {
    config: DapHostSupervisorConfig,
    sessions: BTreeMap<String, DebugSessionSnapshot>,
    next_session_counter: u64,
    next_event_counter: u64,
}

impl DapHostSupervisor {
    /// Builds a supervisor with default restart budget settings.
    pub fn new() -> Self {
        Self::with_config(DapHostSupervisorConfig::default())
    }

    /// Builds a supervisor with explicit settings.
    pub fn with_config(config: DapHostSupervisorConfig) -> Self {
        Self {
            config,
            sessions: BTreeMap::new(),
            next_session_counter: 1,
            next_event_counter: 1,
        }
    }

    /// Opens a new session in [`DebugSessionStateClass::LaunchRequested`].
    pub fn open_session(
        &mut self,
        spec: DebugSessionLaunchSpec,
        observed_at: impl Into<String>,
    ) -> String {
        let observed_at = observed_at.into();
        let session_id = format!(
            "session:dap:{}:{}",
            sanitize_id(&spec.workspace_id),
            self.next_session_counter
        );
        self.next_session_counter += 1;

        let mode_token = spec.mode.as_str().to_owned();
        let identity = DebugSessionIdentity {
            session_id: session_id.clone(),
            workspace_id: spec.workspace_id,
            root_ref: spec.root_ref,
            language_id: spec.language_id,
            execution_context_id: spec.execution_context_id,
            mode: spec.mode,
            mode_token,
            adapter: spec.adapter,
            target: spec.target,
            fault_domain_id: SESSION_SCOPED_FAULT_DOMAIN_ID.into(),
            restart_budget_ref: SESSION_RESTART_BUDGET_REF.into(),
        };

        let initial_event_class = match identity.mode {
            DebugSessionMode::Launch => DebugSessionEventClass::LaunchRequested,
            DebugSessionMode::Attach => DebugSessionEventClass::AttachRequested,
            DebugSessionMode::Reconnect => DebugSessionEventClass::RestartScheduled,
        };
        let initial_summary = match identity.mode {
            DebugSessionMode::Launch => {
                format!(
                    "Debug-session launch requested for target {} via {}.",
                    identity.target.canonical_target_id, identity.adapter.adapter_label
                )
            }
            DebugSessionMode::Attach => {
                format!(
                    "Debug-session attach requested for target {} via {}.",
                    identity.target.canonical_target_id, identity.adapter.adapter_label
                )
            }
            DebugSessionMode::Reconnect => {
                format!(
                    "Debug-session reconnect requested for target {} via {}.",
                    identity.target.canonical_target_id, identity.adapter.adapter_label
                )
            }
        };

        let event = self.event(EventInput {
            session_id: session_id.clone(),
            event_class: initial_event_class,
            state_after: DebugSessionStateClass::LaunchRequested,
            exit_reason_class: None,
            restart_cause: None,
            restart_strike_count: 0,
            observed_at: observed_at.clone(),
            summary: initial_summary.clone(),
        });

        let snapshot = DebugSessionSnapshot {
            record_kind: DEBUG_SESSION_RECORD_KIND.into(),
            schema_version: DEBUG_SESSION_LIFECYCLE_SCHEMA_VERSION,
            identity,
            state_class: DebugSessionStateClass::LaunchRequested,
            state_class_token: DebugSessionStateClass::LaunchRequested.as_str().to_owned(),
            negotiation_outcome: None,
            negotiation_outcome_token: None,
            requested_capabilities: Vec::new(),
            advertised_capabilities: Vec::new(),
            agreed_capabilities: Vec::new(),
            dropped_capabilities: Vec::new(),
            restart_strike_count: 0,
            restart_budget_in_window: self.config.automatic_restarts_in_window,
            reconnect_attempt_count: 0,
            quarantine_ref: None,
            last_exit_reason_class: None,
            last_exit_reason_token: None,
            opened_at: observed_at.clone(),
            last_state_change_at: observed_at,
            event_lineage: vec![event],
            summary: initial_summary,
        };
        self.sessions.insert(session_id.clone(), snapshot);
        session_id
    }

    /// Runs adapter capability negotiation and updates session state.
    ///
    /// # Errors
    ///
    /// Returns [`DapHostSupervisorError::UnknownSession`] when the session id
    /// is not supervised, [`DapHostSupervisorError::InvalidTransition`] when
    /// the session is past the negotiation phase, or
    /// [`DapHostSupervisorError::NegotiationRequiredCapabilityMissing`] when
    /// the adapter cannot satisfy a required capability.
    pub fn negotiate(
        &mut self,
        session_id: &str,
        input: DebugAdapterNegotiationInput,
        observed_at: impl Into<String>,
    ) -> Result<&DebugSessionSnapshot, DapHostSupervisorError> {
        let observed_at = observed_at.into();
        {
            let snapshot = self.session(session_id)?;
            if !matches!(snapshot.state_class, DebugSessionStateClass::LaunchRequested) {
                return Err(DapHostSupervisorError::InvalidTransition {
                    session_id: session_id.into(),
                    from: snapshot.state_class,
                    attempted: "negotiate",
                });
            }
        }

        let strike_count = self.session(session_id)?.restart_strike_count;
        let neg_start = self.event(EventInput {
            session_id: session_id.into(),
            event_class: DebugSessionEventClass::NegotiationStarted,
            state_after: DebugSessionStateClass::NegotiatingCapabilities,
            exit_reason_class: None,
            restart_cause: None,
            restart_strike_count: strike_count,
            observed_at: observed_at.clone(),
            summary: "Debug-adapter capability negotiation started.".into(),
        });
        {
            let snapshot = self.session_mut(session_id)?;
            snapshot.state_class = DebugSessionStateClass::NegotiatingCapabilities;
            snapshot.state_class_token = DebugSessionStateClass::NegotiatingCapabilities
                .as_str()
                .to_owned();
            snapshot.last_state_change_at = observed_at.clone();
            snapshot.event_lineage.push(neg_start);
        }

        match input {
            DebugAdapterNegotiationInput::AdapterResponded { request, response } => {
                let missing_required: Vec<DebugAdapterCapabilityClass> = request
                    .required
                    .iter()
                    .copied()
                    .filter(|cap| !response.advertised.contains(cap))
                    .collect();
                if !missing_required.is_empty() {
                    let outcome = DebugAdapterNegotiationOutcome::RefusedMissingRequiredCapability;
                    self.apply_negotiation_refusal(
                        session_id,
                        request.requested.clone(),
                        response.advertised.clone(),
                        outcome,
                        strike_count,
                        observed_at.clone(),
                    )?;
                    return Err(DapHostSupervisorError::NegotiationRequiredCapabilityMissing {
                        session_id: session_id.into(),
                        missing: missing_required,
                    });
                }

                let agreed: Vec<DebugAdapterCapabilityClass> = request
                    .requested
                    .iter()
                    .copied()
                    .filter(|cap| response.advertised.contains(cap))
                    .collect();
                let dropped: Vec<DebugAdapterCapabilityClass> = request
                    .requested
                    .iter()
                    .copied()
                    .filter(|cap| !response.advertised.contains(cap))
                    .collect();
                let outcome = if dropped.is_empty() {
                    DebugAdapterNegotiationOutcome::AgreedFull
                } else {
                    DebugAdapterNegotiationOutcome::AgreedWithCapabilityDowngrade
                };

                let summary = if dropped.is_empty() {
                    format!(
                        "Adapter agreed to all {} requested capabilities.",
                        agreed.len()
                    )
                } else {
                    format!(
                        "Adapter agreed with downgrade; {} of {} requested capabilities dropped.",
                        dropped.len(),
                        request.requested.len()
                    )
                };
                let agreed_event = self.event(EventInput {
                    session_id: session_id.into(),
                    event_class: DebugSessionEventClass::CapabilitiesAgreed,
                    state_after: DebugSessionStateClass::HandshakeComplete,
                    exit_reason_class: None,
                    restart_cause: None,
                    restart_strike_count: strike_count,
                    observed_at: observed_at.clone(),
                    summary,
                });
                let snapshot = self.session_mut(session_id)?;
                snapshot.state_class = DebugSessionStateClass::HandshakeComplete;
                snapshot.state_class_token = DebugSessionStateClass::HandshakeComplete
                    .as_str()
                    .to_owned();
                snapshot.negotiation_outcome = Some(outcome);
                snapshot.negotiation_outcome_token = Some(outcome.as_str().to_owned());
                snapshot.requested_capabilities = request.requested;
                snapshot.advertised_capabilities = response.advertised.clone();
                snapshot.agreed_capabilities = agreed;
                snapshot.dropped_capabilities = dropped;
                snapshot.identity.adapter.agreed_dap_protocol_version =
                    Some(response.agreed_dap_protocol_version);
                snapshot.last_state_change_at = observed_at;
                snapshot.event_lineage.push(agreed_event);
            }
            DebugAdapterNegotiationInput::AdapterRefused { request, outcome } => {
                self.apply_negotiation_refusal(
                    session_id,
                    request.requested,
                    Vec::new(),
                    outcome,
                    strike_count,
                    observed_at,
                )?;
            }
        }
        self.session(session_id)
    }

    /// Marks the session as actively serving the debugged target.
    ///
    /// # Errors
    ///
    /// Returns [`DapHostSupervisorError::UnknownSession`] when the session id
    /// is not supervised, or [`DapHostSupervisorError::InvalidTransition`]
    /// when negotiation has not completed.
    pub fn mark_session_ready(
        &mut self,
        session_id: &str,
        observed_at: impl Into<String>,
    ) -> Result<&DebugSessionSnapshot, DapHostSupervisorError> {
        let observed_at = observed_at.into();
        let (state_after, summary, event_class) = {
            let snapshot = self.session(session_id)?;
            if !matches!(
                snapshot.state_class,
                DebugSessionStateClass::HandshakeComplete | DebugSessionStateClass::Reconnecting
            ) {
                return Err(DapHostSupervisorError::InvalidTransition {
                    session_id: session_id.into(),
                    from: snapshot.state_class,
                    attempted: "mark_session_ready",
                });
            }
            let state_after = match snapshot.identity.mode {
                DebugSessionMode::Launch | DebugSessionMode::Reconnect => {
                    DebugSessionStateClass::LaunchedRunning
                }
                DebugSessionMode::Attach => DebugSessionStateClass::AttachedRunning,
            };
            let summary = format!(
                "{} is debugging {}.",
                snapshot.identity.adapter.adapter_label,
                snapshot.identity.target.target_label
            );
            let event_class = match snapshot.state_class {
                DebugSessionStateClass::Reconnecting => DebugSessionEventClass::SessionReconnected,
                _ => DebugSessionEventClass::SessionReady,
            };
            (state_after, summary, event_class)
        };
        let strike_count = self.session(session_id)?.restart_strike_count;
        let event = self.event(EventInput {
            session_id: session_id.into(),
            event_class,
            state_after,
            exit_reason_class: None,
            restart_cause: None,
            restart_strike_count: strike_count,
            observed_at: observed_at.clone(),
            summary: summary.clone(),
        });
        let snapshot = self.session_mut(session_id)?;
        snapshot.state_class = state_after;
        snapshot.state_class_token = state_after.as_str().to_owned();
        snapshot.last_state_change_at = observed_at;
        snapshot.summary = summary;
        snapshot.event_lineage.push(event);
        self.session(session_id)
    }

    /// Records that the debugged target paused (breakpoint or exception).
    pub fn mark_paused(
        &mut self,
        session_id: &str,
        observed_at: impl Into<String>,
        summary: impl Into<String>,
    ) -> Result<&DebugSessionSnapshot, DapHostSupervisorError> {
        let observed_at = observed_at.into();
        let summary = summary.into();
        {
            let snapshot = self.session(session_id)?;
            if !snapshot.state_class.is_running() {
                return Err(DapHostSupervisorError::InvalidTransition {
                    session_id: session_id.into(),
                    from: snapshot.state_class,
                    attempted: "mark_paused",
                });
            }
        }
        let strike_count = self.session(session_id)?.restart_strike_count;
        let event = self.event(EventInput {
            session_id: session_id.into(),
            event_class: DebugSessionEventClass::Paused,
            state_after: DebugSessionStateClass::Paused,
            exit_reason_class: None,
            restart_cause: None,
            restart_strike_count: strike_count,
            observed_at: observed_at.clone(),
            summary: summary.clone(),
        });
        let snapshot = self.session_mut(session_id)?;
        snapshot.state_class = DebugSessionStateClass::Paused;
        snapshot.state_class_token = DebugSessionStateClass::Paused.as_str().to_owned();
        snapshot.last_state_change_at = observed_at;
        snapshot.summary = summary;
        snapshot.event_lineage.push(event);
        self.session(session_id)
    }

    /// Records that the debugged target resumed.
    pub fn mark_resumed(
        &mut self,
        session_id: &str,
        observed_at: impl Into<String>,
    ) -> Result<&DebugSessionSnapshot, DapHostSupervisorError> {
        let observed_at = observed_at.into();
        let (state_after, summary) = {
            let snapshot = self.session(session_id)?;
            if !matches!(snapshot.state_class, DebugSessionStateClass::Paused) {
                return Err(DapHostSupervisorError::InvalidTransition {
                    session_id: session_id.into(),
                    from: snapshot.state_class,
                    attempted: "mark_resumed",
                });
            }
            let state_after = match snapshot.identity.mode {
                DebugSessionMode::Launch | DebugSessionMode::Reconnect => {
                    DebugSessionStateClass::LaunchedRunning
                }
                DebugSessionMode::Attach => DebugSessionStateClass::AttachedRunning,
            };
            let summary = format!(
                "{} resumed debugging {}.",
                snapshot.identity.adapter.adapter_label,
                snapshot.identity.target.target_label
            );
            (state_after, summary)
        };
        let strike_count = self.session(session_id)?.restart_strike_count;
        let event = self.event(EventInput {
            session_id: session_id.into(),
            event_class: DebugSessionEventClass::Resumed,
            state_after,
            exit_reason_class: None,
            restart_cause: None,
            restart_strike_count: strike_count,
            observed_at: observed_at.clone(),
            summary: summary.clone(),
        });
        let snapshot = self.session_mut(session_id)?;
        snapshot.state_class = state_after;
        snapshot.state_class_token = state_after.as_str().to_owned();
        snapshot.last_state_change_at = observed_at;
        snapshot.summary = summary;
        snapshot.event_lineage.push(event);
        self.session(session_id)
    }

    /// Records an adapter exit and applies restart / quarantine policy.
    ///
    /// Adapter crashes degrade only this session: the supervisor records the
    /// typed exit reason, increments the bounded restart budget when the
    /// exit is non-clean, and either moves the session into `reconnecting`
    /// or into `quarantined` when the budget is exhausted.
    pub fn record_adapter_exit(
        &mut self,
        session_id: &str,
        reason: DebugSessionExitReasonClass,
        observed_at: impl Into<String>,
    ) -> Result<&DebugSessionSnapshot, DapHostSupervisorError> {
        let observed_at = observed_at.into();
        let mut strike_count = self.session(session_id)?.restart_strike_count;
        if reason.counts_toward_restart_budget() {
            strike_count += 1;
        }

        let exit_event = self.event(EventInput {
            session_id: session_id.into(),
            event_class: DebugSessionEventClass::AdapterExited,
            state_after: DebugSessionStateClass::Degraded,
            exit_reason_class: Some(reason),
            restart_cause: None,
            restart_strike_count: strike_count,
            observed_at: observed_at.clone(),
            summary: format!(
                "Adapter exit observed by supervisor; reason={}.",
                reason.as_str()
            ),
        });

        let counts = reason.counts_toward_restart_budget();
        let over_budget = counts && strike_count >= self.config.automatic_restarts_in_window;
        let exit_reason_token = reason.as_str().to_owned();
        let exit_reason_class = Some(reason);

        if over_budget {
            let quarantine_ref =
                format!("quarantine:debug:{}:{}", sanitize_id(session_id), strike_count);
            let summary = format!(
                "Debug session quarantined after {strike_count} adapter failures in the restart window."
            );
            let quarantine_event = self.event(EventInput {
                session_id: session_id.into(),
                event_class: DebugSessionEventClass::SessionQuarantined,
                state_after: DebugSessionStateClass::Quarantined,
                exit_reason_class,
                restart_cause: None,
                restart_strike_count: strike_count,
                observed_at: observed_at.clone(),
                summary: summary.clone(),
            });
            let snapshot = self.session_mut(session_id)?;
            snapshot.restart_strike_count = strike_count;
            snapshot.reconnect_attempt_count += 1;
            snapshot.state_class = DebugSessionStateClass::Quarantined;
            snapshot.state_class_token =
                DebugSessionStateClass::Quarantined.as_str().to_owned();
            snapshot.quarantine_ref = Some(quarantine_ref);
            snapshot.last_exit_reason_class = exit_reason_class;
            snapshot.last_exit_reason_token = Some(exit_reason_token);
            snapshot.last_state_change_at = observed_at;
            snapshot.summary = summary;
            snapshot.event_lineage.push(exit_event);
            snapshot.event_lineage.push(quarantine_event);
        } else if counts {
            let restart_cause = match reason {
                DebugSessionExitReasonClass::AdapterCrashUnhandled
                | DebugSessionExitReasonClass::AdapterCrashSignal
                | DebugSessionExitReasonClass::AdapterResourceQuotaExceeded
                | DebugSessionExitReasonClass::UnknownNoncleanExit
                | DebugSessionExitReasonClass::TargetLost
                | DebugSessionExitReasonClass::ContractMismatch => {
                    DebugSessionRestartCause::AdapterCrashInsideBudget
                }
                DebugSessionExitReasonClass::AdapterProtocolViolation
                | DebugSessionExitReasonClass::AdapterInitializeTimeout => {
                    DebugSessionRestartCause::ProtocolViolationInsideBudget
                }
                DebugSessionExitReasonClass::WatchdogForcedKillAfterStall => {
                    DebugSessionRestartCause::WatchdogStallInsideBudget
                }
                _ => DebugSessionRestartCause::AdapterCrashInsideBudget,
            };
            let summary = format!(
                "Debug session reconnect scheduled (strike {strike_count} of {budget}); cause={cause}.",
                budget = self.config.automatic_restarts_in_window,
                cause = restart_cause.as_str(),
            );
            let restart_event = self.event(EventInput {
                session_id: session_id.into(),
                event_class: DebugSessionEventClass::RestartScheduled,
                state_after: DebugSessionStateClass::Reconnecting,
                exit_reason_class,
                restart_cause: Some(restart_cause),
                restart_strike_count: strike_count,
                observed_at: observed_at.clone(),
                summary: summary.clone(),
            });
            let snapshot = self.session_mut(session_id)?;
            snapshot.restart_strike_count = strike_count;
            snapshot.reconnect_attempt_count += 1;
            snapshot.state_class = DebugSessionStateClass::Reconnecting;
            snapshot.state_class_token =
                DebugSessionStateClass::Reconnecting.as_str().to_owned();
            snapshot.last_exit_reason_class = exit_reason_class;
            snapshot.last_exit_reason_token = Some(exit_reason_token);
            snapshot.last_state_change_at = observed_at;
            snapshot.summary = summary;
            snapshot.event_lineage.push(exit_event);
            snapshot.event_lineage.push(restart_event);
        } else {
            let summary = format!(
                "Debug session terminated cleanly; reason={}.",
                reason.as_str()
            );
            let terminated_event = self.event(EventInput {
                session_id: session_id.into(),
                event_class: DebugSessionEventClass::SessionTerminated,
                state_after: DebugSessionStateClass::Terminated,
                exit_reason_class,
                restart_cause: None,
                restart_strike_count: strike_count,
                observed_at: observed_at.clone(),
                summary: summary.clone(),
            });
            let snapshot = self.session_mut(session_id)?;
            snapshot.state_class = DebugSessionStateClass::Terminated;
            snapshot.state_class_token =
                DebugSessionStateClass::Terminated.as_str().to_owned();
            snapshot.last_exit_reason_class = exit_reason_class;
            snapshot.last_exit_reason_token = Some(exit_reason_token);
            snapshot.last_state_change_at = observed_at;
            snapshot.summary = summary;
            snapshot.event_lineage.push(exit_event);
            snapshot.event_lineage.push(terminated_event);
        }
        self.session(session_id)
    }

    /// Terminates a session at the user's or supervisor's request.
    pub fn terminate_session(
        &mut self,
        session_id: &str,
        reason: DebugSessionExitReasonClass,
        observed_at: impl Into<String>,
    ) -> Result<&DebugSessionSnapshot, DapHostSupervisorError> {
        if reason.counts_toward_restart_budget() {
            return Err(DapHostSupervisorError::InvalidTransition {
                session_id: session_id.into(),
                from: self.session(session_id)?.state_class,
                attempted: "terminate_session",
            });
        }
        self.record_adapter_exit(session_id, reason, observed_at)
    }

    /// Returns a session snapshot.
    pub fn snapshot(&self, session_id: &str) -> Option<&DebugSessionSnapshot> {
        self.sessions.get(session_id)
    }

    /// Returns every supervised session snapshot in stable order.
    pub fn snapshots(&self) -> impl Iterator<Item = &DebugSessionSnapshot> {
        self.sessions.values()
    }

    /// Builds an export-safe support packet for a workspace.
    pub fn support_packet(
        &mut self,
        workspace_id: &str,
        captured_at: impl Into<String>,
    ) -> DebugSessionSupportPacket {
        let captured_at = captured_at.into();
        let workspace_id_owned = workspace_id.to_owned();
        let session_ids: Vec<String> = self
            .sessions
            .values()
            .filter(|snapshot| snapshot.identity.workspace_id == workspace_id_owned)
            .map(|snapshot| snapshot.identity.session_id.clone())
            .collect();
        for session_id in &session_ids {
            let strike_count = self
                .session(session_id)
                .map(|snapshot| snapshot.restart_strike_count)
                .unwrap_or(0);
            let state_after = self
                .session(session_id)
                .map(|snapshot| snapshot.state_class)
                .unwrap_or(DebugSessionStateClass::Terminated);
            let event = self.event(EventInput {
                session_id: session_id.clone(),
                event_class: DebugSessionEventClass::SupportExportEmitted,
                state_after,
                exit_reason_class: None,
                restart_cause: None,
                restart_strike_count: strike_count,
                observed_at: captured_at.clone(),
                summary: "Debug-session evidence captured into a support export.".into(),
            });
            if let Ok(snapshot) = self.session_mut(session_id) {
                snapshot.event_lineage.push(event);
            }
        }

        let session_rows: Vec<DebugSessionSnapshot> = self
            .sessions
            .values()
            .filter(|snapshot| snapshot.identity.workspace_id == workspace_id_owned)
            .cloned()
            .collect();
        let export_safe_summary = format!(
            "{} debug-session rows captured for workspace {}.",
            session_rows.len(),
            workspace_id_owned
        );
        DebugSessionSupportPacket {
            record_kind: DEBUG_SESSION_SUPPORT_PACKET_RECORD_KIND.into(),
            debug_session_lifecycle_schema_version: DEBUG_SESSION_LIFECYCLE_SCHEMA_VERSION,
            supervisor_session_id: self.config.supervisor_session_id.clone(),
            workspace_id: workspace_id_owned,
            session_rows,
            captured_at,
            export_safe_summary,
        }
    }

    fn apply_negotiation_refusal(
        &mut self,
        session_id: &str,
        requested: Vec<DebugAdapterCapabilityClass>,
        advertised: Vec<DebugAdapterCapabilityClass>,
        outcome: DebugAdapterNegotiationOutcome,
        strike_count: u32,
        observed_at: String,
    ) -> Result<(), DapHostSupervisorError> {
        let summary = format!(
            "Debug-adapter negotiation refused; outcome={}.",
            outcome.as_str()
        );
        let refusal_event = self.event(EventInput {
            session_id: session_id.into(),
            event_class: DebugSessionEventClass::NegotiationRefused,
            state_after: DebugSessionStateClass::Terminated,
            exit_reason_class: Some(map_refusal_to_exit(outcome)),
            restart_cause: None,
            restart_strike_count: strike_count,
            observed_at: observed_at.clone(),
            summary: summary.clone(),
        });
        let snapshot = self.session_mut(session_id)?;
        snapshot.state_class = DebugSessionStateClass::Terminated;
        snapshot.state_class_token = DebugSessionStateClass::Terminated.as_str().to_owned();
        snapshot.negotiation_outcome = Some(outcome);
        snapshot.negotiation_outcome_token = Some(outcome.as_str().to_owned());
        snapshot.requested_capabilities = requested;
        snapshot.advertised_capabilities = advertised;
        snapshot.agreed_capabilities = Vec::new();
        snapshot.dropped_capabilities = Vec::new();
        snapshot.last_exit_reason_class = Some(map_refusal_to_exit(outcome));
        snapshot.last_exit_reason_token = Some(map_refusal_to_exit(outcome).as_str().to_owned());
        snapshot.last_state_change_at = observed_at;
        snapshot.summary = summary;
        snapshot.event_lineage.push(refusal_event);
        Ok(())
    }

    fn session(
        &self,
        session_id: &str,
    ) -> Result<&DebugSessionSnapshot, DapHostSupervisorError> {
        self.sessions
            .get(session_id)
            .ok_or_else(|| DapHostSupervisorError::UnknownSession(session_id.into()))
    }

    fn session_mut(
        &mut self,
        session_id: &str,
    ) -> Result<&mut DebugSessionSnapshot, DapHostSupervisorError> {
        self.sessions
            .get_mut(session_id)
            .ok_or_else(|| DapHostSupervisorError::UnknownSession(session_id.into()))
    }

    fn event(&mut self, input: EventInput) -> DebugSessionLifecycleEvent {
        let event_id = format!("event:debug-session:{}", self.next_event_counter);
        self.next_event_counter += 1;
        let exit_reason_token = input.exit_reason_class.map(|c| c.as_str().to_owned());
        let restart_cause_token = input.restart_cause.map(|c| c.as_str().to_owned());
        DebugSessionLifecycleEvent {
            record_kind: DEBUG_SESSION_EVENT_RECORD_KIND.into(),
            schema_version: DEBUG_SESSION_LIFECYCLE_SCHEMA_VERSION,
            event_id,
            session_id: input.session_id,
            event_class: input.event_class,
            event_class_token: input.event_class.as_str().to_owned(),
            state_after: input.state_after,
            state_after_token: input.state_after.as_str().to_owned(),
            exit_reason_class: input.exit_reason_class,
            exit_reason_token,
            restart_cause: input.restart_cause,
            restart_cause_token,
            restart_strike_count: input.restart_strike_count,
            observed_at: input.observed_at,
            summary: input.summary,
        }
    }
}

impl Default for DapHostSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

fn map_refusal_to_exit(outcome: DebugAdapterNegotiationOutcome) -> DebugSessionExitReasonClass {
    match outcome {
        DebugAdapterNegotiationOutcome::RefusedIncompatibleProtocol => {
            DebugSessionExitReasonClass::ContractMismatch
        }
        DebugAdapterNegotiationOutcome::RefusedMissingRequiredCapability => {
            DebugSessionExitReasonClass::ContractMismatch
        }
        DebugAdapterNegotiationOutcome::RefusedPolicyBlocked => {
            DebugSessionExitReasonClass::SupervisorRequestedTermination
        }
        DebugAdapterNegotiationOutcome::RefusedInitializeTimeout => {
            DebugSessionExitReasonClass::AdapterInitializeTimeout
        }
        DebugAdapterNegotiationOutcome::AgreedFull
        | DebugAdapterNegotiationOutcome::AgreedWithCapabilityDowngrade => {
            DebugSessionExitReasonClass::CleanTermination
        }
    }
}

fn dedupe_sorted(
    iter: impl IntoIterator<Item = DebugAdapterCapabilityClass>,
) -> Vec<DebugAdapterCapabilityClass> {
    let mut out: Vec<DebugAdapterCapabilityClass> = iter.into_iter().collect();
    out.sort_by_key(|c| c.as_str());
    out.dedup();
    out
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
