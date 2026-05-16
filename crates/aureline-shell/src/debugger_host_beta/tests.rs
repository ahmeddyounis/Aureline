use super::*;

use aureline_runtime::{
    DapHostSupervisor, DebugAdapterCapabilityClass, DebugAdapterCapabilityRequest,
    DebugAdapterCapabilityResponse, DebugAdapterNegotiationInput, DebugSessionExitReasonClass,
    DebugSessionLaunchSpec, DebugSessionTargetIdentity,
};

fn local_target() -> DebugSessionTargetIdentity {
    DebugSessionTargetIdentity {
        canonical_target_id: "target:local:debugged:01".into(),
        target_class_token: "local_host".into(),
        target_label: "Local debugged process".into(),
        working_directory_digest: Some("digest:cwd:01".into()),
        inferior_process_id: Some(4242),
    }
}

fn open_local_session(supervisor: &mut DapHostSupervisor) -> String {
    supervisor.open_session(
        DebugSessionLaunchSpec::local_launch(
            "ws-test",
            "workspace:root:webapp",
            "language:typescript",
            "ctx:ws-test:01",
            "adapter:debugpy:node",
            "Node DAP adapter",
            "1.2.3",
            "DAP/1.55",
            local_target(),
        ),
        "2026-05-13T00:00:00Z",
    )
}

#[test]
fn projection_renders_running_session_without_disclosure() {
    let mut supervisor = DapHostSupervisor::new();
    let session_id = open_local_session(&mut supervisor);
    supervisor
        .negotiate(
            &session_id,
            DebugAdapterNegotiationInput::AdapterResponded {
                request: DebugAdapterCapabilityRequest::new(
                    [
                        DebugAdapterCapabilityClass::FunctionBreakpoints,
                        DebugAdapterCapabilityClass::ConditionalBreakpoints,
                    ],
                    [DebugAdapterCapabilityClass::FunctionBreakpoints],
                ),
                response: DebugAdapterCapabilityResponse::new(
                    [
                        DebugAdapterCapabilityClass::FunctionBreakpoints,
                        DebugAdapterCapabilityClass::ConditionalBreakpoints,
                    ],
                    "DAP/1.55",
                ),
            },
            "2026-05-13T00:00:01Z",
        )
        .expect("negotiation succeeds");
    supervisor
        .mark_session_ready(&session_id, "2026-05-13T00:00:02Z")
        .expect("ready");

    let packet = supervisor.support_packet("ws-test", "2026-05-13T00:00:03Z");
    let projection = DebuggerHostBetaProjection::project(&packet);

    assert_eq!(projection.record_kind, DEBUGGER_HOST_BETA_PROJECTION_RECORD_KIND);
    assert_eq!(projection.workspace_id, "ws-test");
    assert_eq!(projection.sessions.len(), 1);
    assert!(!projection.honesty_marker_present);
    let row = &projection.sessions[0];
    assert_eq!(row.state_class_token, "launched_running");
    assert_eq!(row.canonical_target_id, "target:local:debugged:01");
    assert!(!row.state_requires_disclosure);
}

#[test]
fn projection_lights_honesty_marker_on_quarantine() {
    let mut supervisor = DapHostSupervisor::new();
    let session_id = open_local_session(&mut supervisor);
    supervisor
        .negotiate(
            &session_id,
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
        .expect("negotiation succeeds");
    supervisor
        .mark_session_ready(&session_id, "2026-05-13T00:00:02Z")
        .expect("ready");

    for ts in [
        "2026-05-13T00:00:03Z",
        "2026-05-13T00:00:04Z",
        "2026-05-13T00:00:05Z",
    ] {
        supervisor
            .record_adapter_exit(
                &session_id,
                DebugSessionExitReasonClass::AdapterCrashUnhandled,
                ts,
            )
            .expect("exit recorded");
    }

    let packet = supervisor.support_packet("ws-test", "2026-05-13T00:00:06Z");
    let projection = DebuggerHostBetaProjection::project(&packet);
    assert!(projection.honesty_marker_present);
    let row = &projection.sessions[0];
    assert_eq!(row.state_class_token, "quarantined");
    assert!(row.quarantine_ref.is_some());
    assert!(projection.render_plaintext().contains("Quarantine:"));
}
