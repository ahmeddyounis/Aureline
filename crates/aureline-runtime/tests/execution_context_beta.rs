//! Integration coverage for the beta execution-context resolver lane.
//!
//! The test replays the checked-in fixtures under
//! [`fixtures/runtime/execution_context_beta/`] end-to-end:
//!
//! 1. Each lane fixture drives the canonical resolver through a typed request
//!    constructor and asserts the lane the runtime classifies the context
//!    onto, the resolved target class, the resolved toolchain class, and the
//!    visible boundary-cue posture.
//! 2. The ticket-drift fixture mints a stored binding from a local-host
//!    context and a fresh remote-attach context, then asserts the typed
//!    drift rows the evaluator emits.
//! 3. The canonical lane-coverage manifest fixture round-trips through serde
//!    so reviewer evidence and the runtime emit the same record shape.
//! 4. A shell-side projection consumer wires the resolved fresh context onto
//!    the [`aureline_terminal::pty_host::PtyHost`] session header so the
//!    boundary cue lights when a stored local ticket is invalidated against
//!    a remote-attach context.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    evaluate_ticket_drift, lane_for_context, CapsuleDriftState, EnvironmentCapsuleRef,
    ExecutionContext, ExecutionContextBetaCoverageManifest, ExecutionContextBetaLane,
    ExecutionContextBetaLaneSample, ExecutionContextBetaSupportExport, ExecutionContextRequest,
    ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
    SurfaceClass, TargetClass, TicketDriftBinding, TicketDriftField, TicketDriftOutcome,
    ToolchainClass, TrustState, EXECUTION_CONTEXT_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    EXECUTION_CONTEXT_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
use aureline_terminal::pty_host::{HostClass, OpenSessionRequest, PtyHost};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("runtime")
        .join("execution_context_beta")
}

fn baseline_resolver() -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: "ws-beta".to_owned(),
        profile_id: Some("prof.beta".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/workspace".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "caps:ws-beta:seed".to_owned(),
            capsule_hash: "sha256:beta".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "beta-0".to_owned(),
    })
}

fn host_class_for_target(target_class: TargetClass) -> HostClass {
    match target_class {
        TargetClass::LocalHost | TargetClass::NotebookKernelLocal => HostClass::HostDesktop,
        TargetClass::ContainerLocal | TargetClass::Devcontainer => HostClass::LocalContainer,
        _ => HostClass::RemoteAgentPrimary,
    }
}

#[derive(Debug, Deserialize)]
struct LaneFixture {
    record_kind: String,
    schema_version: u32,
    input: LaneFixtureInput,
    expect: LaneFixtureExpect,
}

#[derive(Debug, Deserialize)]
struct LaneFixtureInput {
    constructor: String,
    command_id: String,
    #[serde(default)]
    target_class: Option<TargetClass>,
    trust_state: TrustState,
    observed_at: String,
}

#[derive(Debug, Deserialize)]
struct LaneFixtureExpect {
    lane: ExecutionContextBetaLane,
    target_class: TargetClass,
    surface: SurfaceClass,
    toolchain_class: ToolchainClass,
    boundary_cue_visible: bool,
    has_degraded_field: bool,
}

#[derive(Debug, Deserialize)]
struct DriftFixture {
    record_kind: String,
    schema_version: u32,
    stored: LaneFixtureInput,
    fresh: LaneFixtureInput,
    expect: DriftFixtureExpect,
}

#[derive(Debug, Deserialize)]
struct DriftFixtureExpect {
    outcome: String,
    fresh_lane: ExecutionContextBetaLane,
    drift_fields: Vec<TicketDriftField>,
}

fn resolve_fixture(
    resolver: &mut ExecutionContextResolver,
    input: &LaneFixtureInput,
) -> ExecutionContext {
    match input.constructor.as_str() {
        "local_terminal_seed" => resolver.resolve(ExecutionContextRequest::local_terminal_seed(
            &input.command_id,
            input.trust_state,
            &input.observed_at,
        )),
        "task_seed" => resolver.resolve(ExecutionContextRequest::task_seed(
            &input.command_id,
            input.trust_state,
            &input.observed_at,
        )),
        "test_seed" => resolver.resolve(ExecutionContextRequest::test_seed(
            &input.command_id,
            input.trust_state,
            &input.observed_at,
        )),
        "debug_prep_seed" => resolver.resolve(ExecutionContextRequest::debug_prep_seed(
            &input.command_id,
            input.trust_state,
            &input.observed_at,
        )),
        "ai_tool_call_seed" => resolver.resolve(ExecutionContextRequest::ai_tool_call_seed(
            &input.command_id,
            input.trust_state,
            &input.observed_at,
        )),
        "container_task_seed" => {
            let target_class = input
                .target_class
                .expect("container_task_seed requires target_class");
            resolver.resolve(ExecutionContextRequest::container_task_seed(
                &input.command_id,
                target_class,
                input.trust_state,
                &input.observed_at,
            ))
        }
        "remote_attach_task_seed" => {
            let target_class = input
                .target_class
                .expect("remote_attach_task_seed requires target_class");
            resolver.resolve(ExecutionContextRequest::remote_attach_task_seed(
                &input.command_id,
                target_class,
                input.trust_state,
                &input.observed_at,
            ))
        }
        "request_workspace_task_seed" => {
            let target_class = input
                .target_class
                .expect("request_workspace_task_seed requires target_class");
            resolver.resolve(ExecutionContextRequest::request_workspace_task_seed(
                &input.command_id,
                target_class,
                input.trust_state,
                &input.observed_at,
            ))
        }
        other => panic!("unknown fixture constructor: {other}"),
    }
}

#[test]
fn every_beta_lane_fixture_replays_through_the_canonical_resolver() {
    for fixture_name in [
        "local_lane.json",
        "remote_lane.json",
        "container_lane.json",
        "request_workspace_lane.json",
    ] {
        let path = fixture_root().join(fixture_name);
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read fixture {fixture_name}: {err}"));
        let fixture: LaneFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse fixture {fixture_name}: {err}"));
        assert_eq!(fixture.record_kind, "execution_context_beta_case");
        assert_eq!(fixture.schema_version, 1);

        let mut resolver = baseline_resolver();
        let context = resolve_fixture(&mut resolver, &fixture.input);

        assert_eq!(
            lane_for_context(&context),
            fixture.expect.lane,
            "{fixture_name}: lane mismatch"
        );
        assert_eq!(
            context.target_identity.target_class, fixture.expect.target_class,
            "{fixture_name}: target_class mismatch"
        );
        assert_eq!(
            context.invocation_subject.surface, fixture.expect.surface,
            "{fixture_name}: surface mismatch"
        );
        assert_eq!(
            context.toolchain_identity.toolchain_class, fixture.expect.toolchain_class,
            "{fixture_name}: toolchain_class mismatch"
        );
        assert_eq!(
            context.boundary_cue_visible(),
            fixture.expect.boundary_cue_visible,
            "{fixture_name}: boundary_cue mismatch"
        );
        assert_eq!(
            context.has_degraded_field(),
            fixture.expect.has_degraded_field,
            "{fixture_name}: degraded_field mismatch"
        );
    }
}

#[test]
fn ticket_drift_fixture_records_typed_drift_rows() {
    let path = fixture_root().join("ticket_drift_invalidated.json");
    let payload = std::fs::read_to_string(&path).expect("read drift fixture");
    let fixture: DriftFixture = serde_json::from_str(&payload).expect("parse drift fixture");
    assert_eq!(fixture.record_kind, "execution_context_beta_drift_case");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(fixture.expect.outcome, "invalidated");

    let mut resolver = baseline_resolver();
    let stored_context = resolve_fixture(&mut resolver, &fixture.stored);
    let stored_binding = TicketDriftBinding::from_context(&stored_context);

    let fresh_context = resolve_fixture(&mut resolver, &fixture.fresh);
    let evaluation = evaluate_ticket_drift(&stored_binding, &fresh_context);

    assert_eq!(evaluation.fresh_lane, fixture.expect.fresh_lane);
    assert!(
        evaluation.outcome.is_invalidated(),
        "drift evaluator must invalidate stored binding"
    );
    let drift_fields: Vec<TicketDriftField> = evaluation
        .outcome
        .drift_rows()
        .iter()
        .map(|row| row.field)
        .collect();
    for required in &fixture.expect.drift_fields {
        assert!(
            drift_fields.contains(required),
            "missing drift field {:?} in evaluator output {:?}",
            required,
            drift_fields
        );
    }
}

#[test]
fn canonical_lane_coverage_manifest_matches_fixture_shape() {
    let path = fixture_root().join("beta_lane_coverage.json");
    let payload = std::fs::read_to_string(&path).expect("read coverage manifest fixture");
    let fixture: ExecutionContextBetaCoverageManifest =
        serde_json::from_str(&payload).expect("parse coverage manifest fixture");
    assert_eq!(
        fixture.record_kind,
        EXECUTION_CONTEXT_BETA_COVERAGE_MANIFEST_RECORD_KIND
    );

    let canonical = ExecutionContextBetaCoverageManifest::canonical(
        "execution-context-beta:canonical",
        "2026-05-15T00:00:00Z",
    );
    assert_eq!(fixture, canonical);
    assert!(canonical.covers_every_target_class());
}

#[test]
fn drift_invalidation_lights_boundary_cue_on_terminal_session_header() {
    // Shell-side projection: when a fresh resolve advances the lane from
    // local_host to remote_attach, the terminal session minted against the
    // new context MUST light the local-vs-managed boundary cue. The same
    // ExecutionContext object the support export records is wired through
    // the PtyHost so the chrome stays in sync with the resolved truth.
    let mut resolver = baseline_resolver();
    let stored_context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));
    let stored_binding = TicketDriftBinding::from_context(&stored_context);

    let fresh_context = resolver.resolve(ExecutionContextRequest::remote_attach_task_seed(
        "task.run.ssh_remote",
        TargetClass::SshRemote,
        TrustState::Trusted,
        "mono:1",
    ));
    let evaluation = evaluate_ticket_drift(&stored_binding, &fresh_context);
    assert!(evaluation.outcome.is_invalidated());
    assert_eq!(
        evaluation.fresh_lane,
        ExecutionContextBetaLane::RemoteAttach
    );

    let mut host = PtyHost::new();
    let session_id = host.open_session(OpenSessionRequest {
        workspace_id: &fresh_context.invocation_subject.workspace_id,
        host_class: host_class_for_target(fresh_context.target_identity.target_class),
        display_title: "agent shell",
        cwd_hint: fresh_context.target_identity.working_directory.as_deref(),
        execution_context_ref: fresh_context.execution_context_id(),
        trust_state: fresh_context.policy_and_trust.trust_state,
        observed_at: "mono:1",
    });
    let session = host.session(&session_id).expect("session must exist");
    assert!(
        session.header().needs_boundary_cue(),
        "drift-invalidated remote context MUST light the boundary cue"
    );
    assert_eq!(session.header().target_badge, "Remote");
    assert_eq!(
        session.header().execution_context_ref,
        fresh_context.execution_context_id(),
        "session header MUST quote the freshly resolved execution_context_id"
    );
}

#[test]
fn beta_support_export_packet_round_trips_lane_samples_and_drift_evaluations() {
    let mut resolver = baseline_resolver();
    let local = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));
    let remote = resolver.resolve(ExecutionContextRequest::remote_attach_task_seed(
        "task.run.ssh_remote",
        TargetClass::SshRemote,
        TrustState::Trusted,
        "mono:1",
    ));
    let container = resolver.resolve(ExecutionContextRequest::container_task_seed(
        "task.run.devcontainer",
        TargetClass::Devcontainer,
        TrustState::Trusted,
        "mono:2",
    ));
    let workspace = resolver.resolve(ExecutionContextRequest::request_workspace_task_seed(
        "task.run.managed_workspace",
        TargetClass::ManagedWorkspace,
        TrustState::Restricted,
        "mono:3",
    ));
    let stored_binding = TicketDriftBinding::from_context(&local);
    let drift = evaluate_ticket_drift(&stored_binding, &remote);

    let lane_samples = vec![
        ExecutionContextBetaLaneSample::from_context(&local),
        ExecutionContextBetaLaneSample::from_context(&remote),
        ExecutionContextBetaLaneSample::from_context(&container),
        ExecutionContextBetaLaneSample::from_context(&workspace),
    ];
    let packet = ExecutionContextBetaSupportExport::new(
        "execution-context-beta:packet",
        "2026-05-15T00:00:01Z",
        lane_samples,
        vec![drift],
    );

    let json = serde_json::to_string(&packet).expect("serialize packet");
    let round: ExecutionContextBetaSupportExport =
        serde_json::from_str(&json).expect("deserialize packet");
    assert_eq!(round, packet);
    assert_eq!(
        round.record_kind,
        EXECUTION_CONTEXT_BETA_SUPPORT_EXPORT_RECORD_KIND
    );
    assert!(round.coverage_manifest.covers_every_target_class());
    assert!(round
        .ticket_drift_evaluations
        .iter()
        .any(|eval| eval.outcome.is_invalidated()));

    let lanes: Vec<ExecutionContextBetaLane> = round.lane_samples.iter().map(|s| s.lane).collect();
    for lane in ExecutionContextBetaLane::ALL {
        assert!(
            lanes.contains(&lane),
            "support export must surface a sample for every lane: missing {lane:?}"
        );
    }

    let workspace_sample = round
        .lane_samples
        .iter()
        .find(|sample| sample.lane == ExecutionContextBetaLane::RequestWorkspace)
        .expect("request_workspace sample");
    assert!(
        workspace_sample.boundary_cue_visible,
        "request_workspace lane sample MUST light the boundary cue"
    );

    // Verify that the resolved ticket-drift outcome variant survives serde
    // round-trip with both shape and drift-row identity intact.
    let evaluation = round
        .ticket_drift_evaluations
        .first()
        .expect("at least one evaluation present");
    match &evaluation.outcome {
        TicketDriftOutcome::Invalidated { drift_rows } => {
            assert!(drift_rows
                .iter()
                .any(|row| row.field == TicketDriftField::TargetClass));
        }
        TicketDriftOutcome::Fresh => {
            panic!("expected invalidated outcome in support-export drift evaluation");
        }
    }
}
