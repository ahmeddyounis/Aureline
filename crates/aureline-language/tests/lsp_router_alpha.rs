use std::path::Path;

use aureline_language::{
    LanguageServerHostIdentity, LanguageServerHostStatus, LspRouter, RouterCapabilityClass,
    RouterCompletenessClass, RouterDecisionRecord, RouterFallbackClass, RouterFaultDomainId,
    RouterFreshnessClass, RouterHealthState, RouterLocalityClass, RouterRequest,
    RouterScopeClaimClass, RouterSurfaceClass, WorkspaceLocalRouterRequest,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    workspace_id: String,
    root_ref: String,
    execution_context_id: String,
    cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
struct Case {
    case_id: String,
    language_id: String,
    server_label: String,
    provider_id: String,
    surface: String,
    capability: String,
    host_health: String,
    expected_selected_provider_id: String,
    expected_degraded_state: String,
    expected_requires_degraded_disclosure: bool,
}

#[test]
fn lsp_router_routes_ready_reconnecting_unavailable_and_quarantined_hosts() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "lsp_router_alpha_cases");
    assert_eq!(fixture.schema_version, 1);

    let router = LspRouter::new();
    for case in &fixture.cases {
        let capability = capability(&case.capability);
        let request = RouterRequest::workspace_local(WorkspaceLocalRouterRequest {
            language_id: case.language_id.clone(),
            surface_class: surface(&case.surface),
            capability_class: capability,
            requested_subject_ref: format!("subject:{}", case.case_id),
            workspace_id: fixture.workspace_id.clone(),
            root_ref: fixture.root_ref.clone(),
            execution_context_id: fixture.execution_context_id.clone(),
            captured_at: "2026-05-13T00:00:00Z".into(),
        });
        let status = host_status(&fixture, case, capability);
        let decision = router.route(request, &[status]);

        assert_eq!(decision.record_kind, RouterDecisionRecord::RECORD_KIND);
        assert_eq!(
            decision.decision_outcome.selected_provider_id, case.expected_selected_provider_id,
            "selected provider mismatch for {}",
            case.case_id
        );
        assert_eq!(
            enum_token(decision.decision_outcome.degraded_state_class),
            case.expected_degraded_state,
            "degraded state mismatch for {}",
            case.case_id
        );
        assert_eq!(
            decision.requires_degraded_disclosure(),
            case.expected_requires_degraded_disclosure,
            "degraded disclosure mismatch for {}",
            case.case_id
        );
        assert!(
            decision
                .provider_stack_rows
                .iter()
                .any(|row| row.provider_id == case.provider_id),
            "LSP host row should remain inspectable for {}",
            case.case_id
        );

        let serialized =
            serde_json::to_string(&decision).expect("router decision serializes to JSON");
        let round_trip: RouterDecisionRecord =
            serde_json::from_str(&serialized).expect("router decision deserializes from JSON");
        assert_eq!(round_trip, decision);
    }
}

fn host_status(
    fixture: &Fixture,
    case: &Case,
    capability: RouterCapabilityClass,
) -> LanguageServerHostStatus {
    let health = health(&case.host_health);
    LanguageServerHostStatus {
        identity: LanguageServerHostIdentity {
            host_instance_id: format!("host:lsp:{}", case.case_id),
            provider_id: case.provider_id.clone(),
            workspace_id: fixture.workspace_id.clone(),
            root_ref: fixture.root_ref.clone(),
            language_id: case.language_id.clone(),
            server_label: case.server_label.clone(),
            execution_context_id: fixture.execution_context_id.clone(),
            locality_class: RouterLocalityClass::LocalSidecar,
            fault_domain_id: RouterFaultDomainId::SessionScopedExecutionHosts,
            restart_budget_ref: "restart_budget:session_scoped_execution_hosts:language:01".into(),
        },
        health_state: health,
        freshness_class: freshness_for(health),
        scope_claim_class: RouterScopeClaimClass::ActiveWorkset,
        completeness_class: completeness_for(health),
        scope_limit_classes: Vec::new(),
        supported_capability_classes: vec![capability],
        restart_strike_count: if health == RouterHealthState::CrashLoopQuarantined {
            3
        } else if health.requires_disclosure() {
            1
        } else {
            0
        },
        quarantine_ref: (health == RouterHealthState::CrashLoopQuarantined)
            .then(|| format!("quarantine:{}", case.case_id)),
        fallback_class: RouterFallbackClass::ProtocolToText,
        health_summary: format!("{} is {}.", case.server_label, case.host_health),
    }
}

fn health(value: &str) -> RouterHealthState {
    match value {
        "ready" => RouterHealthState::Ready,
        "warming" => RouterHealthState::Warming,
        "unavailable" => RouterHealthState::Unavailable,
        "crash_loop_quarantined" => RouterHealthState::CrashLoopQuarantined,
        other => panic!("unknown health state {other}"),
    }
}

fn freshness_for(health: RouterHealthState) -> RouterFreshnessClass {
    match health {
        RouterHealthState::Ready => RouterFreshnessClass::AuthoritativeLive,
        RouterHealthState::Warming | RouterHealthState::Degraded => {
            RouterFreshnessClass::DegradedCached
        }
        _ => RouterFreshnessClass::Unverified,
    }
}

fn completeness_for(health: RouterHealthState) -> RouterCompletenessClass {
    match health {
        RouterHealthState::Ready => RouterCompletenessClass::CompleteForClaimedScope,
        RouterHealthState::Warming | RouterHealthState::Degraded => {
            RouterCompletenessClass::PartialForClaimedScope
        }
        _ => RouterCompletenessClass::UnavailableForClaimedScope,
    }
}

fn surface(value: &str) -> RouterSurfaceClass {
    match value {
        "completion" => RouterSurfaceClass::Completion,
        "hover" => RouterSurfaceClass::Hover,
        "definition" => RouterSurfaceClass::Definition,
        "diagnostic" => RouterSurfaceClass::Diagnostic,
        other => panic!("unknown surface {other}"),
    }
}

fn capability(value: &str) -> RouterCapabilityClass {
    match value {
        "completion" => RouterCapabilityClass::Completion,
        "hover" => RouterCapabilityClass::Hover,
        "definition" => RouterCapabilityClass::Definition,
        "diagnostics" => RouterCapabilityClass::Diagnostics,
        other => panic!("unknown capability {other}"),
    }
}

fn enum_token<T: serde::Serialize>(value: T) -> String {
    serde_json::to_value(value)
        .expect("enum serializes")
        .as_str()
        .expect("enum serializes as string")
        .to_owned()
}

fn load_fixture() -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/language/lsp_router_alpha/router_cases.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
