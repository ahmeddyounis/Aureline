use std::path::{Path, PathBuf};

use aureline_runtime::{
    DapHostSupervisor, DapHostSupervisorConfig, DapHostSupervisorError,
    DebugAdapterCapabilityClass, DebugAdapterCapabilityRequest, DebugAdapterCapabilityResponse,
    DebugAdapterIdentity, DebugAdapterNegotiationInput, DebugAdapterNegotiationOutcome,
    DebugAdapterTransportClass, DebugSessionExitReasonClass, DebugSessionLaunchSpec,
    DebugSessionMode, DebugSessionStateClass, DebugSessionSupportPacket,
    DebugSessionTargetIdentity, DEBUG_SESSION_SUPPORT_PACKET_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    supervisor_session_id: String,
    workspace_id: String,
    spec: FixtureSpec,
    negotiation: FixtureNegotiation,
    steps: Vec<FixtureStep>,
    expect: FixtureExpect,
}

#[derive(Debug, Deserialize)]
struct FixtureSpec {
    mode: String,
    root_ref: String,
    language_id: String,
    execution_context_id: String,
    adapter: FixtureAdapter,
    target: FixtureTarget,
}

#[derive(Debug, Deserialize)]
struct FixtureAdapter {
    adapter_id: String,
    adapter_label: String,
    adapter_version: String,
    requested_dap_protocol_version: String,
    transport_class: String,
}

#[derive(Debug, Deserialize)]
struct FixtureTarget {
    canonical_target_id: String,
    target_class_token: String,
    target_label: String,
    working_directory_digest: Option<String>,
    inferior_process_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct FixtureNegotiation {
    requested: Vec<String>,
    required: Vec<String>,
    advertised: Vec<String>,
    agreed_dap_protocol_version: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum FixtureStep {
    Open {
        observed_at: String,
    },
    Negotiate {
        observed_at: String,
    },
    MarkReady {
        observed_at: String,
    },
    Pause {
        observed_at: String,
        summary: String,
    },
    Resume {
        observed_at: String,
    },
    AdapterExit {
        observed_at: String,
        reason: String,
    },
    Terminate {
        observed_at: String,
        reason: String,
    },
}

#[derive(Debug, Deserialize)]
struct FixtureExpect {
    support_packet_record_kind: String,
    session_count: usize,
    final_state_class_token: String,
    negotiation_outcome_token: String,
    #[serde(default)]
    agreed_capability_tokens: Option<Vec<String>>,
    #[serde(default)]
    dropped_capability_tokens: Option<Vec<String>>,
    last_exit_reason_token: String,
    restart_strike_count: u32,
    #[serde(default)]
    reconnect_attempt_count: Option<u32>,
    quarantine_ref_present: bool,
    honesty_marker_present: bool,
    captured_at: String,
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/debugger_host_beta")
        .join(name)
}

fn load_fixture(name: &str) -> Fixture {
    let path = fixture_path(name);
    let body = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn parse_mode(token: &str) -> DebugSessionMode {
    match token {
        "launch" => DebugSessionMode::Launch,
        "attach" => DebugSessionMode::Attach,
        "reconnect" => DebugSessionMode::Reconnect,
        other => panic!("unknown mode token {other}"),
    }
}

fn parse_transport(token: &str) -> DebugAdapterTransportClass {
    match token {
        "local_sidecar_stdio" => DebugAdapterTransportClass::LocalSidecarStdio,
        "local_sidecar_loopback_socket" => DebugAdapterTransportClass::LocalSidecarLoopbackSocket,
        "managed_connector" => DebugAdapterTransportClass::ManagedConnector,
        other => panic!("unknown transport token {other}"),
    }
}

fn parse_capability(token: &str) -> DebugAdapterCapabilityClass {
    match token {
        "function_breakpoints" => DebugAdapterCapabilityClass::FunctionBreakpoints,
        "conditional_breakpoints" => DebugAdapterCapabilityClass::ConditionalBreakpoints,
        "hit_count_breakpoints" => DebugAdapterCapabilityClass::HitCountBreakpoints,
        "log_points" => DebugAdapterCapabilityClass::LogPoints,
        "data_breakpoints" => DebugAdapterCapabilityClass::DataBreakpoints,
        "exception_breakpoint_filters" => DebugAdapterCapabilityClass::ExceptionBreakpointFilters,
        "target_restart_request" => DebugAdapterCapabilityClass::TargetRestartRequest,
        "terminate_request" => DebugAdapterCapabilityClass::TerminateRequest,
        "reverse_execution" => DebugAdapterCapabilityClass::ReverseExecution,
        "hot_reload" => DebugAdapterCapabilityClass::HotReload,
        "breakpoint_locations" => DebugAdapterCapabilityClass::BreakpointLocations,
        "granularity_stepping" => DebugAdapterCapabilityClass::GranularityStepping,
        "modules_events" => DebugAdapterCapabilityClass::ModulesEvents,
        "loaded_sources_events" => DebugAdapterCapabilityClass::LoadedSourcesEvents,
        "memory_access" => DebugAdapterCapabilityClass::MemoryAccess,
        other => panic!("unknown capability token {other}"),
    }
}

fn parse_exit_reason(token: &str) -> DebugSessionExitReasonClass {
    match token {
        "clean_termination" => DebugSessionExitReasonClass::CleanTermination,
        "user_requested_termination" => DebugSessionExitReasonClass::UserRequestedTermination,
        "supervisor_requested_termination" => {
            DebugSessionExitReasonClass::SupervisorRequestedTermination
        }
        "adapter_crash_unhandled" => DebugSessionExitReasonClass::AdapterCrashUnhandled,
        "adapter_crash_signal" => DebugSessionExitReasonClass::AdapterCrashSignal,
        "adapter_protocol_violation" => DebugSessionExitReasonClass::AdapterProtocolViolation,
        "adapter_initialize_timeout" => DebugSessionExitReasonClass::AdapterInitializeTimeout,
        "adapter_resource_quota_exceeded" => {
            DebugSessionExitReasonClass::AdapterResourceQuotaExceeded
        }
        "watchdog_forced_kill_after_stall" => {
            DebugSessionExitReasonClass::WatchdogForcedKillAfterStall
        }
        "target_lost" => DebugSessionExitReasonClass::TargetLost,
        "contract_mismatch" => DebugSessionExitReasonClass::ContractMismatch,
        "unknown_nonclean_exit" => DebugSessionExitReasonClass::UnknownNoncleanExit,
        other => panic!("unknown exit reason token {other}"),
    }
}

fn build_target(target: &FixtureTarget) -> DebugSessionTargetIdentity {
    DebugSessionTargetIdentity {
        canonical_target_id: target.canonical_target_id.clone(),
        target_class_token: target.target_class_token.clone(),
        target_label: target.target_label.clone(),
        working_directory_digest: target.working_directory_digest.clone(),
        inferior_process_id: target.inferior_process_id,
    }
}

fn build_adapter(spec: &FixtureAdapter) -> DebugAdapterIdentity {
    DebugAdapterIdentity::new(
        &spec.adapter_id,
        &spec.adapter_label,
        &spec.adapter_version,
        &spec.requested_dap_protocol_version,
        parse_transport(&spec.transport_class),
    )
}

fn replay(fixture: &Fixture) -> (DapHostSupervisor, String, Vec<DapHostSupervisorError>) {
    let mut supervisor = DapHostSupervisor::with_config(DapHostSupervisorConfig {
        supervisor_session_id: fixture.supervisor_session_id.clone(),
        automatic_restarts_in_window: 3,
    });
    let mut session_id = String::new();
    let mut errors = Vec::new();
    for step in &fixture.steps {
        match step {
            FixtureStep::Open { observed_at } => {
                let launch_spec = DebugSessionLaunchSpec {
                    workspace_id: fixture.workspace_id.clone(),
                    root_ref: fixture.spec.root_ref.clone(),
                    language_id: fixture.spec.language_id.clone(),
                    execution_context_id: fixture.spec.execution_context_id.clone(),
                    mode: parse_mode(&fixture.spec.mode),
                    adapter: build_adapter(&fixture.spec.adapter),
                    target: build_target(&fixture.spec.target),
                };
                session_id = supervisor.open_session(launch_spec, observed_at);
            }
            FixtureStep::Negotiate { observed_at } => {
                let request = DebugAdapterCapabilityRequest::new(
                    fixture
                        .negotiation
                        .requested
                        .iter()
                        .map(|s| parse_capability(s)),
                    fixture
                        .negotiation
                        .required
                        .iter()
                        .map(|s| parse_capability(s)),
                );
                let response = DebugAdapterCapabilityResponse::new(
                    fixture
                        .negotiation
                        .advertised
                        .iter()
                        .map(|s| parse_capability(s)),
                    &fixture.negotiation.agreed_dap_protocol_version,
                );
                if let Err(err) = supervisor.negotiate(
                    &session_id,
                    DebugAdapterNegotiationInput::AdapterResponded { request, response },
                    observed_at,
                ) {
                    errors.push(err);
                }
            }
            FixtureStep::MarkReady { observed_at } => {
                supervisor
                    .mark_session_ready(&session_id, observed_at)
                    .expect("mark_session_ready");
            }
            FixtureStep::Pause {
                observed_at,
                summary,
            } => {
                supervisor
                    .mark_paused(&session_id, observed_at, summary)
                    .expect("mark_paused");
            }
            FixtureStep::Resume { observed_at } => {
                supervisor
                    .mark_resumed(&session_id, observed_at)
                    .expect("mark_resumed");
            }
            FixtureStep::AdapterExit {
                observed_at,
                reason,
            } => {
                supervisor
                    .record_adapter_exit(&session_id, parse_exit_reason(reason), observed_at)
                    .expect("record_adapter_exit");
            }
            FixtureStep::Terminate {
                observed_at,
                reason,
            } => {
                supervisor
                    .terminate_session(&session_id, parse_exit_reason(reason), observed_at)
                    .expect("terminate_session");
            }
        }
    }
    (supervisor, session_id, errors)
}

fn check_packet(packet: &DebugSessionSupportPacket, fixture: &Fixture) {
    assert_eq!(
        packet.record_kind,
        fixture.expect.support_packet_record_kind
    );
    assert_eq!(packet.record_kind, DEBUG_SESSION_SUPPORT_PACKET_RECORD_KIND);
    assert_eq!(packet.workspace_id, fixture.workspace_id);
    assert_eq!(packet.session_rows.len(), fixture.expect.session_count);
    let row = &packet.session_rows[0];
    assert_eq!(
        row.state_class_token,
        fixture.expect.final_state_class_token
    );
    assert_eq!(
        row.negotiation_outcome_token.as_deref(),
        Some(fixture.expect.negotiation_outcome_token.as_str())
    );
    if let Some(expected) = &fixture.expect.agreed_capability_tokens {
        let actual: Vec<String> = row
            .agreed_capabilities
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        assert_eq!(&actual, expected, "agreed capabilities mismatch");
    }
    if let Some(expected) = &fixture.expect.dropped_capability_tokens {
        let actual: Vec<String> = row
            .dropped_capabilities
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        assert_eq!(&actual, expected, "dropped capabilities mismatch");
    }
    assert_eq!(
        row.last_exit_reason_token.as_deref(),
        Some(fixture.expect.last_exit_reason_token.as_str())
    );
    assert_eq!(
        row.restart_strike_count,
        fixture.expect.restart_strike_count
    );
    if let Some(expected) = fixture.expect.reconnect_attempt_count {
        assert_eq!(row.reconnect_attempt_count, expected);
    }
    assert_eq!(
        row.quarantine_ref.is_some(),
        fixture.expect.quarantine_ref_present,
        "quarantine_ref presence mismatch"
    );
    assert_eq!(
        row.requires_shell_disclosure(),
        fixture.expect.honesty_marker_present,
        "honesty marker mismatch"
    );
}

fn round_trip(packet: &DebugSessionSupportPacket) {
    let serialized = serde_json::to_string(packet).expect("packet serializes");
    let parsed: DebugSessionSupportPacket =
        serde_json::from_str(&serialized).expect("packet deserializes");
    assert_eq!(&parsed, packet);
}

#[test]
fn protected_walk_local_terminates_cleanly_with_capability_downgrade() {
    let fixture = load_fixture("protected_walk_local.json");
    let (mut supervisor, _session_id, errors) = replay(&fixture);
    assert!(errors.is_empty(), "no negotiation errors expected");
    let packet = supervisor.support_packet(&fixture.workspace_id, &fixture.expect.captured_at);
    check_packet(&packet, &fixture);
    let row = &packet.session_rows[0];
    assert_eq!(row.identity.mode, DebugSessionMode::Launch);
    assert_eq!(
        row.negotiation_outcome,
        Some(DebugAdapterNegotiationOutcome::AgreedWithCapabilityDowngrade)
    );
    assert_eq!(row.state_class, DebugSessionStateClass::Terminated);
    round_trip(&packet);
}

#[test]
fn adapter_crash_loop_drives_session_into_quarantine() {
    let fixture = load_fixture("adapter_crash_loop_quarantine.json");
    let (mut supervisor, session_id, _errors) = replay(&fixture);
    let packet = supervisor.support_packet(&fixture.workspace_id, &fixture.expect.captured_at);
    check_packet(&packet, &fixture);
    let row = &packet.session_rows[0];
    assert_eq!(row.state_class, DebugSessionStateClass::Quarantined);
    assert!(row.quarantine_ref.is_some());
    // Crash isolation: an unrelated session id should not exist on the same supervisor.
    assert!(supervisor.snapshot("session:dap:other:1").is_none());
    // Lineage must include at least one restart_scheduled before the quarantine.
    let restart_scheduled = row
        .event_lineage
        .iter()
        .any(|event| event.event_class_token == "restart_scheduled");
    assert!(
        restart_scheduled,
        "restart scheduled events must precede quarantine"
    );
    assert!(row
        .event_lineage
        .iter()
        .any(|event| event.event_class_token == "session_quarantined"));
    assert_eq!(row.identity.session_id, session_id);
    round_trip(&packet);
}

#[test]
fn missing_required_capability_refuses_negotiation() {
    let fixture = load_fixture("negotiation_missing_required_capability.json");
    let (mut supervisor, _session_id, errors) = replay(&fixture);
    assert_eq!(errors.len(), 1, "exactly one negotiation error expected");
    match &errors[0] {
        DapHostSupervisorError::NegotiationRequiredCapabilityMissing { missing, .. } => {
            assert!(
                missing.contains(&DebugAdapterCapabilityClass::DataBreakpoints),
                "missing list must name data_breakpoints"
            );
        }
        other => panic!("unexpected error variant {other:?}"),
    }
    let packet = supervisor.support_packet(&fixture.workspace_id, &fixture.expect.captured_at);
    check_packet(&packet, &fixture);
    let row = &packet.session_rows[0];
    assert_eq!(row.state_class, DebugSessionStateClass::Terminated);
    assert_eq!(
        row.negotiation_outcome,
        Some(DebugAdapterNegotiationOutcome::RefusedMissingRequiredCapability)
    );
    round_trip(&packet);
}

#[test]
fn one_adapter_crash_does_not_destabilize_unrelated_session() {
    let mut supervisor = DapHostSupervisor::new();
    let crash_target = DebugSessionTargetIdentity {
        canonical_target_id: "target:local:debugged:a".into(),
        target_class_token: "local_host".into(),
        target_label: "Local A".into(),
        working_directory_digest: None,
        inferior_process_id: None,
    };
    let healthy_target = DebugSessionTargetIdentity {
        canonical_target_id: "target:local:debugged:b".into(),
        target_class_token: "local_host".into(),
        target_label: "Local B".into(),
        working_directory_digest: None,
        inferior_process_id: None,
    };
    let crash_id = supervisor.open_session(
        DebugSessionLaunchSpec::local_launch(
            "ws-test",
            "workspace:root:a",
            "language:typescript",
            "ctx:a",
            "adapter:a",
            "Adapter A",
            "1.0.0",
            "DAP/1.55",
            crash_target,
        ),
        "2026-05-13T00:00:00Z",
    );
    let healthy_id = supervisor.open_session(
        DebugSessionLaunchSpec::local_launch(
            "ws-test",
            "workspace:root:b",
            "language:python",
            "ctx:b",
            "adapter:b",
            "Adapter B",
            "1.0.0",
            "DAP/1.55",
            healthy_target,
        ),
        "2026-05-13T00:00:00Z",
    );
    for id in [&crash_id, &healthy_id] {
        supervisor
            .negotiate(
                id,
                DebugAdapterNegotiationInput::AdapterResponded {
                    request: DebugAdapterCapabilityRequest::new(
                        [DebugAdapterCapabilityClass::FunctionBreakpoints],
                        [DebugAdapterCapabilityClass::FunctionBreakpoints],
                    ),
                    response: DebugAdapterCapabilityResponse::new(
                        [DebugAdapterCapabilityClass::FunctionBreakpoints],
                        "DAP/1.55",
                    ),
                },
                "2026-05-13T00:00:01Z",
            )
            .expect("negotiate");
        supervisor
            .mark_session_ready(id, "2026-05-13T00:00:02Z")
            .expect("ready");
    }
    supervisor
        .record_adapter_exit(
            &crash_id,
            DebugSessionExitReasonClass::AdapterCrashUnhandled,
            "2026-05-13T00:00:03Z",
        )
        .expect("crash recorded");
    let crashed = supervisor.snapshot(&crash_id).expect("crashed session");
    let healthy = supervisor.snapshot(&healthy_id).expect("healthy session");
    assert_eq!(crashed.state_class, DebugSessionStateClass::Reconnecting);
    assert_eq!(healthy.state_class, DebugSessionStateClass::LaunchedRunning);
    assert!(healthy.quarantine_ref.is_none());
    assert_eq!(healthy.restart_strike_count, 0);
}
