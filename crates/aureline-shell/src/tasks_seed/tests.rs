use std::path::Path;

use aureline_runtime::{
    CapsuleDriftState, DegradedFieldReason, DegradedFieldRecord, EnvironmentCapsuleRef,
    ExecutionContextRequest, ExecutionContextResolver, ExecutionContextResolverConfig,
    IdentityMode, ReachabilityState, ScopeClass, TargetClass, ToolchainClass, TrustState,
};

use serde::Deserialize;

use super::*;
use crate::badges::target_origin::{HostBoundaryCue, TargetBadgeClass};

fn baseline_resolver() -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: "ws-test".to_owned(),
        profile_id: Some("prof.default".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/workspace".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "caps:ws-test:seed".to_owned(),
            capsule_hash: "sha256:seed".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "seed-0".to_owned(),
    })
}

#[test]
fn protected_walk_local_seed_renders_live_actions_without_honesty_marker() {
    // Protected walk: open the task seed against a trusted local desktop and
    // confirm the surface quotes the same target/runtime fields the terminal
    // pane uses, with reserved actions clearly labeled and no honesty marker.
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::task_seed(
        "task.run.cargo_build",
        TrustState::Trusted,
        "mono:0",
    ));

    let surface = TaskSeedSurface::project(&context);
    assert_eq!(surface.record_kind, TASK_SEED_SURFACE_RECORD_KIND);
    assert_eq!(surface.schema_version, TASK_SEED_SURFACE_SCHEMA_VERSION);
    assert_eq!(surface.entry_point, BadgeEntryPoint::TaskSeed);
    assert_eq!(surface.workspace_id, "ws-test");
    assert_eq!(surface.execution_context_ref, context.execution_context_id);
    assert_eq!(surface.target_class, TargetBadgeClass::LocalDesktop);
    assert_eq!(surface.target_class_token, "local_desktop");
    assert_eq!(surface.target_label, "Local");
    assert_eq!(surface.toolchain_class, ToolchainClass::BuildDriverRuntime);
    assert_eq!(surface.toolchain_class_token, "build_driver_runtime");
    assert_eq!(surface.toolchain_class_label, "Build-driver runtime");
    assert_eq!(surface.working_directory.as_deref(), Some("/workspace"));
    assert_eq!(surface.boundary_cue, HostBoundaryCue::Hidden);
    assert!(!surface.boundary_cue_visible);
    assert!(!surface.honesty_marker_present);
    assert!(surface.blocked_prerequisites.is_empty());

    // The badge embedded on the surface MUST agree with the surface fields —
    // surface and badge are projected from the same upstream record so a
    // support export can correlate them.
    assert_eq!(surface.badge.entry_point, BadgeEntryPoint::TaskSeed);
    assert_eq!(
        surface.badge.execution_context_ref,
        context.execution_context_id
    );
    assert_eq!(surface.badge.target_class_token, surface.target_class_token);

    // Live and reserved actions are both present and distinguishable.
    let live: Vec<_> = surface
        .actions
        .iter()
        .filter(|a| matches!(a.availability, TaskSeedActionAvailability::Live))
        .map(|a| a.action_class)
        .collect();
    assert!(live.contains(&TaskSeedActionClass::OpenExecutionContextInspector));
    assert!(live.contains(&TaskSeedActionClass::CopyContextForSupportExport));
    assert!(live.contains(&TaskSeedActionClass::OpenInvokingTerminal));

    let reserved: Vec<_> = surface
        .actions
        .iter()
        .filter(|a| matches!(
            a.availability,
            TaskSeedActionAvailability::ReservedForLaterMilestone
        ))
        .map(|a| a.action_class)
        .collect();
    assert!(reserved.contains(&TaskSeedActionClass::RunTaskFromTemplate));
    assert!(reserved.contains(&TaskSeedActionClass::ConfigureTaskWatchers));
    assert!(reserved.contains(&TaskSeedActionClass::OpenTaskQueueInspector));

    // Seed scope notice MUST be quoted verbatim so the chrome can render it
    // and the user can see the surface scope without inferring from labels.
    assert!(surface
        .seed_scope_notice
        .contains("reserved for a later milestone"));
}

#[test]
fn pending_trust_lights_honesty_row_and_blocks_live_launch_action() {
    // Failure drill: open the task seed with the workspace trust unresolved.
    // The surface MUST surface a typed prerequisite, light the honesty
    // marker, and downgrade the launch-style live action to
    // BlockedByPendingTrust without removing it.
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::task_seed(
        "task.run.cargo_build",
        TrustState::PendingEvaluation,
        "mono:0",
    ));

    let surface = TaskSeedSurface::project(&context);
    assert!(surface.honesty_marker_present);
    assert_eq!(surface.trust_state_token, "pending_evaluation");
    assert!(!surface.blocked_prerequisites.is_empty());
    let trust_row = surface
        .blocked_prerequisites
        .iter()
        .find(|row| {
            matches!(
                row.reason_class,
                TaskSeedPrerequisiteReasonClass::PendingTrust
            )
        })
        .expect("pending-trust prerequisite row");
    assert_eq!(
        trust_row.field_path.as_deref(),
        Some("policy_and_trust.trust_state")
    );
    assert_eq!(
        trust_row.degraded_reason,
        Some(DegradedFieldReason::TrustStateUnresolved)
    );

    let inspector_action = surface
        .actions
        .iter()
        .find(|action| {
            matches!(
                action.action_class,
                TaskSeedActionClass::OpenExecutionContextInspector
            )
        })
        .expect("inspector action present");
    assert_eq!(
        inspector_action.availability,
        TaskSeedActionAvailability::Live,
        "the inspector path is the recovery surface and stays live"
    );
    let copy_action = surface
        .actions
        .iter()
        .find(|action| {
            matches!(
                action.action_class,
                TaskSeedActionClass::CopyContextForSupportExport
            )
        })
        .expect("copy action present");
    assert_eq!(
        copy_action.availability,
        TaskSeedActionAvailability::Live,
        "support-export copy stays live so the user can attach context"
    );
    let terminal_action = surface
        .actions
        .iter()
        .find(|action| {
            matches!(action.action_class, TaskSeedActionClass::OpenInvokingTerminal)
        })
        .expect("terminal hand-off action present");
    assert_eq!(
        terminal_action.availability,
        TaskSeedActionAvailability::BlockedByPendingTrust,
        "live launch-style actions go behind the pending-trust gate"
    );
    let reserved_action = surface
        .actions
        .iter()
        .find(|action| {
            matches!(
                action.action_class,
                TaskSeedActionClass::RunTaskFromTemplate
            )
        })
        .expect("reserved action present");
    assert_eq!(
        reserved_action.availability,
        TaskSeedActionAvailability::ReservedForLaterMilestone,
        "reserved actions never promote to live just because trust pending"
    );
}

#[test]
fn remote_target_lights_boundary_cue_consistently_with_terminal_badge() {
    // Failure drill: switch the seed onto an SSH-remote target. The badge
    // MUST light LocalToRemote and the surface MUST report the same
    // boundary truth as the upstream badge so the chrome cannot drift.
    let mut resolver = baseline_resolver();
    let mut request = ExecutionContextRequest::task_seed(
        "task.run.remote_build",
        TrustState::Trusted,
        "mono:1",
    );
    request.override_target_class = Some(TargetClass::SshRemote);
    request.override_working_directory = Some("/srv/code");
    let context = resolver.resolve(request);

    let surface = TaskSeedSurface::project(&context);
    assert_eq!(surface.target_class, TargetBadgeClass::RemoteHost);
    assert_eq!(surface.target_label, "Remote");
    assert_eq!(surface.boundary_cue, HostBoundaryCue::LocalToRemote);
    assert!(surface.boundary_cue_visible);
    assert_eq!(
        surface.boundary_cue,
        surface.badge.boundary_cue,
        "badge and surface must agree on the boundary cue"
    );
    assert_eq!(surface.working_directory.as_deref(), Some("/srv/code"));
}

#[test]
fn policy_blocked_activator_renders_blocked_by_policy_actions() {
    // Failure drill: degraded field flagged ActivatorBlockedByPolicy.
    // The surface MUST surface a policy-blocked prerequisite row, the live
    // launch-style action MUST flip to BlockedByPolicy, and the inspector
    // path MUST stay live so the user can see *why*.
    let mut resolver = baseline_resolver();
    let mut context = resolver.resolve(ExecutionContextRequest::task_seed(
        "task.run.cargo_build",
        TrustState::Trusted,
        "mono:2",
    ));
    context.degraded_fields.push(DegradedFieldRecord {
        field_path: "toolchain_identity.activation_strategy".to_owned(),
        reason: DegradedFieldReason::ActivatorBlockedByPolicy,
        repair_hook_ref: None,
    });

    let surface = TaskSeedSurface::project(&context);
    assert!(surface.honesty_marker_present);
    assert!(surface.blocked_prerequisites.iter().any(|row| matches!(
        row.reason_class,
        TaskSeedPrerequisiteReasonClass::PolicyBlocked
    )));
    let terminal = surface
        .actions
        .iter()
        .find(|action| {
            matches!(action.action_class, TaskSeedActionClass::OpenInvokingTerminal)
        })
        .unwrap();
    assert_eq!(
        terminal.availability,
        TaskSeedActionAvailability::BlockedByPolicy
    );
    let inspector = surface
        .actions
        .iter()
        .find(|action| {
            matches!(
                action.action_class,
                TaskSeedActionClass::OpenExecutionContextInspector
            )
        })
        .unwrap();
    assert_eq!(inspector.availability, TaskSeedActionAvailability::Live);
}

#[test]
fn policy_blocked_target_reachability_renders_a_prerequisite_row() {
    let mut resolver = baseline_resolver();
    let mut context = resolver.resolve(ExecutionContextRequest::task_seed(
        "task.run.cargo_build",
        TrustState::Trusted,
        "mono:3",
    ));
    context.target_identity.reachability_state = ReachabilityState::PolicyBlocked;
    context.target_identity.local_vs_managed_boundary_visible = false;

    let surface = TaskSeedSurface::project(&context);
    assert!(surface
        .blocked_prerequisites
        .iter()
        .any(|row| row.field_path.as_deref() == Some("target_identity.reachability_state")));
    assert!(surface.honesty_marker_present);
}

#[test]
fn render_plaintext_quotes_actions_and_prerequisite_rows() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::task_seed(
        "task.run.cargo_build",
        TrustState::PendingEvaluation,
        "mono:0",
    ));
    let surface = TaskSeedSurface::project(&context);

    let block = surface.render_plaintext();
    assert!(block.contains("Task seed surface"));
    assert!(block.contains("Workspace: ws-test"));
    assert!(block.contains("Runtime: Build-driver runtime"));
    assert!(block.contains("Trust: pending_evaluation"));
    assert!(block.contains("open_execution_context_inspector"));
    assert!(block.contains("run_task_from_template"));
    assert!(block.contains("reserved_for_later_milestone"));
    assert!(block.contains("Blocked / missing prerequisites"));
    assert!(block.contains("trust_state_unresolved"));
}

#[test]
fn fixture_protected_walk_replays_into_task_surface_projection() {
    let fixture: TaskSeedFixture = load_fixture("task_seed_protected_walk_local.json");
    let surface = build_surface_from_fixture(&fixture);
    assert_surface_matches(&surface, &fixture.expect);
}

#[test]
fn fixture_failure_drill_replays_pending_trust_honesty_row() {
    let fixture: TaskSeedFixture = load_fixture("task_seed_pending_trust_failure_drill.json");
    let surface = build_surface_from_fixture(&fixture);
    assert_surface_matches(&surface, &fixture.expect);
    assert!(surface
        .blocked_prerequisites
        .iter()
        .any(|row| matches!(
            row.reason_class,
            TaskSeedPrerequisiteReasonClass::PendingTrust
        )));
}

fn load_fixture(name: &str) -> TaskSeedFixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/task_debug_seed_cases")
        .join(name);
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    serde_json::from_str(&payload).expect("fixture must parse")
}

fn build_surface_from_fixture(fixture: &TaskSeedFixture) -> TaskSeedSurface {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest {
        command_id: &fixture.input.command_id,
        surface: aureline_runtime::SurfaceClass::Task,
        actor_class: aureline_runtime::ActorClass::UserCommand,
        trust_state: fixture.input.trust_state,
        observed_at: &fixture.input.observed_at,
        requested_target_class: fixture.input.requested_target_class,
        requested_working_directory: fixture.input.requested_working_directory.as_deref(),
        requested_toolchain_class: fixture.input.requested_toolchain_class,
        override_target_class: fixture.input.override_target_class,
        override_working_directory: fixture.input.override_working_directory.as_deref(),
        override_toolchain_class: fixture.input.override_toolchain_class,
    });
    TaskSeedSurface::project(&context)
}

fn assert_surface_matches(surface: &TaskSeedSurface, expect: &TaskSeedExpect) {
    assert_eq!(surface.record_kind, expect.record_kind);
    assert_eq!(surface.target_class_token, expect.target_class_token);
    assert_eq!(surface.target_label, expect.target_label);
    assert_eq!(surface.boundary_cue_token, expect.boundary_cue_token);
    assert_eq!(surface.boundary_cue_visible, expect.boundary_cue_visible);
    assert_eq!(surface.toolchain_class_token, expect.toolchain_class_token);
    assert_eq!(surface.trust_state_token, expect.trust_state_token);
    assert_eq!(surface.honesty_marker_present, expect.honesty_marker_present);
    if let Some(expected_cwd) = &expect.working_directory {
        assert_eq!(surface.working_directory.as_deref(), Some(expected_cwd.as_str()));
    }
    let inspector = surface
        .actions
        .iter()
        .find(|a| {
            matches!(
                a.action_class,
                TaskSeedActionClass::OpenExecutionContextInspector
            )
        })
        .expect("inspector action");
    assert_eq!(
        inspector.availability_token,
        expect.inspector_action_availability
    );
    let terminal = surface
        .actions
        .iter()
        .find(|a| matches!(a.action_class, TaskSeedActionClass::OpenInvokingTerminal))
        .expect("terminal action");
    assert_eq!(
        terminal.availability_token,
        expect.terminal_action_availability
    );
    let template = surface
        .actions
        .iter()
        .find(|a| matches!(a.action_class, TaskSeedActionClass::RunTaskFromTemplate))
        .expect("reserved action");
    assert_eq!(
        template.availability_token,
        expect.template_action_availability
    );
}

#[derive(Debug, Deserialize)]
struct TaskSeedFixture {
    input: TaskSeedFixtureInput,
    expect: TaskSeedExpect,
}

#[derive(Debug, Deserialize)]
struct TaskSeedFixtureInput {
    command_id: String,
    trust_state: TrustState,
    observed_at: String,
    #[serde(default)]
    requested_target_class: Option<TargetClass>,
    #[serde(default)]
    requested_working_directory: Option<String>,
    #[serde(default)]
    requested_toolchain_class: Option<ToolchainClass>,
    #[serde(default)]
    override_target_class: Option<TargetClass>,
    #[serde(default)]
    override_working_directory: Option<String>,
    #[serde(default)]
    override_toolchain_class: Option<ToolchainClass>,
}

#[derive(Debug, Deserialize)]
struct TaskSeedExpect {
    record_kind: String,
    target_class_token: String,
    target_label: String,
    boundary_cue_token: String,
    boundary_cue_visible: bool,
    toolchain_class_token: String,
    trust_state_token: String,
    honesty_marker_present: bool,
    #[serde(default)]
    working_directory: Option<String>,
    inspector_action_availability: String,
    terminal_action_availability: String,
    template_action_availability: String,
}
