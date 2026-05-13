use std::path::Path;

use aureline_language::{
    LspRouter, RouterCapabilityClass, RouterDegradedStateClass, RouterHealthState, RouterRequest,
    RouterSurfaceClass, WorkspaceLocalRouterRequest,
};
use aureline_runtime::{
    LanguageHostEventClass, LanguageHostExitReasonClass, LanguageHostLaunchSpec,
    LanguageHostRuntimeStateClass, LanguageHostScopeKey, LanguageHostSupervisor,
    LanguageHostSupportPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    workspace_id: String,
    root_ref: String,
    execution_context_id: String,
}

#[test]
fn language_hosts_are_scoped_by_workspace_root_and_language() {
    let fixture = load_fixture();
    let mut supervisor = LanguageHostSupervisor::new();
    let first = supervisor
        .launch_or_reuse_host(ts_spec(&fixture, &fixture.root_ref), "2026-05-13T00:00:00Z");
    let reused = supervisor
        .launch_or_reuse_host(ts_spec(&fixture, &fixture.root_ref), "2026-05-13T00:00:01Z");
    let sibling = supervisor.launch_or_reuse_host(
        ts_spec(&fixture, "workspace:root:api"),
        "2026-05-13T00:00:02Z",
    );

    assert_eq!(first, reused, "same workspace/root/language reuses host");
    assert_ne!(first, sibling, "different root gets an isolated host");
    assert_eq!(
        supervisor.host_for_scope(&LanguageHostScopeKey::new(
            &fixture.workspace_id,
            &fixture.root_ref,
            "language:typescript"
        )),
        Some(first.as_str())
    );
}

#[test]
fn reconnect_and_quarantine_states_are_visible_to_router_and_support_packets() {
    let fixture = load_fixture();
    let mut supervisor = LanguageHostSupervisor::new();
    let host_id = supervisor
        .launch_or_reuse_host(ts_spec(&fixture, &fixture.root_ref), "2026-05-13T00:00:00Z");
    supervisor
        .mark_ready(&host_id, "2026-05-13T00:00:01Z")
        .expect("host should become ready");

    let ready = supervisor.snapshot(&host_id).expect("host exists");
    assert_eq!(
        ready.runtime_state_class,
        LanguageHostRuntimeStateClass::Ready
    );
    assert_eq!(ready.health_state, RouterHealthState::Ready);
    assert!(!ready.requires_shell_disclosure());

    supervisor
        .record_exit(
            &host_id,
            LanguageHostExitReasonClass::CrashUnhandledPanic,
            "2026-05-13T00:00:02Z",
        )
        .expect("exit should be recorded");
    let reconnecting = supervisor.snapshot(&host_id).expect("host exists");
    assert_eq!(
        reconnecting.runtime_state_class,
        LanguageHostRuntimeStateClass::Reconnecting
    );
    assert_eq!(reconnecting.health_state, RouterHealthState::Warming);
    assert!(reconnecting.requires_shell_disclosure());

    let router = LspRouter::new();
    let hover = router.route(
        RouterRequest::workspace_local(WorkspaceLocalRouterRequest {
            language_id: "language:typescript".into(),
            surface_class: RouterSurfaceClass::Hover,
            capability_class: RouterCapabilityClass::Hover,
            requested_subject_ref: "subject:hover:webapp".into(),
            workspace_id: fixture.workspace_id.clone(),
            root_ref: fixture.root_ref.clone(),
            execution_context_id: fixture.execution_context_id.clone(),
            captured_at: "2026-05-13T00:00:03Z".into(),
        }),
        &supervisor.router_host_statuses(),
    );
    assert_eq!(
        hover.decision_outcome.degraded_state_class,
        RouterDegradedStateClass::DegradedProviderUnavailable
    );
    assert!(hover.requires_degraded_disclosure());

    supervisor
        .record_exit(
            &host_id,
            LanguageHostExitReasonClass::CrashSignal,
            "2026-05-13T00:00:04Z",
        )
        .expect("second exit should be recorded");
    supervisor
        .record_exit(
            &host_id,
            LanguageHostExitReasonClass::RpcContractMismatch,
            "2026-05-13T00:00:05Z",
        )
        .expect("third exit should quarantine");

    let quarantined = supervisor.snapshot(&host_id).expect("host exists");
    assert_eq!(
        quarantined.runtime_state_class,
        LanguageHostRuntimeStateClass::Quarantined
    );
    assert_eq!(
        quarantined.health_state,
        RouterHealthState::CrashLoopQuarantined
    );
    assert!(quarantined.quarantine_ref.is_some());

    let diagnostics = router.route(
        RouterRequest::workspace_local(WorkspaceLocalRouterRequest {
            language_id: "language:typescript".into(),
            surface_class: RouterSurfaceClass::Diagnostic,
            capability_class: RouterCapabilityClass::Diagnostics,
            requested_subject_ref: "subject:diagnostics:webapp".into(),
            workspace_id: fixture.workspace_id.clone(),
            root_ref: fixture.root_ref.clone(),
            execution_context_id: fixture.execution_context_id.clone(),
            captured_at: "2026-05-13T00:00:06Z".into(),
        }),
        &supervisor.router_host_statuses(),
    );
    assert_eq!(
        diagnostics.decision_outcome.degraded_state_class,
        RouterDegradedStateClass::DegradedCrashLoopQuarantine
    );
    assert!(diagnostics
        .provider_stack_rows
        .iter()
        .any(|row| row.quarantine_ref == quarantined.quarantine_ref));

    let support = supervisor.support_packet(&fixture.workspace_id, "2026-05-13T00:00:07Z");
    assert_eq!(support.record_kind, LanguageHostSupportPacket::RECORD_KIND);
    assert_eq!(support.host_rows.len(), 1);
    assert_eq!(
        support.host_rows[0].identity.host_instance_id,
        quarantined.identity.host_instance_id
    );
    assert!(support.host_rows[0]
        .event_lineage
        .iter()
        .any(|event| event.event_class == LanguageHostEventClass::Quarantined));

    let serialized =
        serde_json::to_string(&support).expect("language-host support packet serializes");
    let round_trip: LanguageHostSupportPacket =
        serde_json::from_str(&serialized).expect("support packet deserializes");
    assert_eq!(round_trip, support);
}

fn ts_spec(fixture: &Fixture, root_ref: &str) -> LanguageHostLaunchSpec {
    LanguageHostLaunchSpec::local_sidecar(
        &fixture.workspace_id,
        root_ref,
        "language:typescript",
        "TypeScript language service",
        &fixture.execution_context_id,
        "provider:lsp:tsserver:webapp",
    )
}

fn load_fixture() -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/language/lsp_router_alpha/router_cases.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
