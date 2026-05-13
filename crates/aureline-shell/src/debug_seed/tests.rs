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
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::debug_prep_seed(
        "debug.prep.attach",
        TrustState::Trusted,
        "mono:0",
    ));

    let surface = DebugPrepSeedSurface::project(&context);
    assert_eq!(surface.record_kind, DEBUG_PREP_SEED_SURFACE_RECORD_KIND);
    assert_eq!(
        surface.schema_version,
        DEBUG_PREP_SEED_SURFACE_SCHEMA_VERSION
    );
    assert_eq!(surface.entry_point, BadgeEntryPoint::DebugPrepSeed);
    assert_eq!(surface.workspace_id, "ws-test");
    assert_eq!(surface.execution_context_ref, context.execution_context_id);
    assert_eq!(
        surface.context_summary.record_kind,
        crate::run_context::RUN_CONTEXT_SUMMARY_RECORD_KIND
    );
    assert_eq!(
        surface.context_summary.execution_context_ref,
        surface.execution_context_ref
    );
    assert_eq!(surface.target_class, TargetBadgeClass::LocalDesktop);
    assert_eq!(surface.toolchain_class, ToolchainClass::DebugAdapterRuntime);
    assert_eq!(surface.toolchain_class_token, "debug_adapter_runtime");
    assert_eq!(
        surface.context_summary.toolchain_class_token,
        surface.toolchain_class_token
    );
    assert_eq!(surface.toolchain_class_label, "Debug-adapter runtime");
    assert_eq!(surface.working_directory.as_deref(), Some("/workspace"));
    assert_eq!(surface.boundary_cue, HostBoundaryCue::Hidden);
    assert!(!surface.honesty_marker_present);
    assert!(surface.blocked_prerequisites.is_empty());

    // Reserved actions stay reserved (Attach / Launch / Breakpoint /
    // Profile) without the surface ever overstating depth.
    let reserved: Vec<_> = surface
        .actions
        .iter()
        .filter(|a| {
            matches!(
                a.availability,
                DebugPrepSeedActionAvailability::ReservedForLaterMilestone
            )
        })
        .map(|a| a.action_class)
        .collect();
    for class in [
        DebugPrepSeedActionClass::AttachToRunningProcess,
        DebugPrepSeedActionClass::LaunchDebuggedTarget,
        DebugPrepSeedActionClass::ConfigureBreakpointMap,
        DebugPrepSeedActionClass::ManageDebugProfile,
    ] {
        assert!(
            reserved.contains(&class),
            "reserved should contain {class:?}"
        );
    }
}

#[test]
fn pending_trust_lights_honesty_row_and_blocks_terminal_handoff() {
    // Failure drill: open the debug-prep seed with the workspace trust
    // unresolved. The surface MUST surface a typed prerequisite, light
    // the honesty marker, and downgrade the terminal hand-off action to
    // BlockedByPendingTrust without removing it.
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::debug_prep_seed(
        "debug.prep.attach",
        TrustState::PendingEvaluation,
        "mono:0",
    ));

    let surface = DebugPrepSeedSurface::project(&context);
    assert!(surface.honesty_marker_present);
    let trust_row = surface
        .blocked_prerequisites
        .iter()
        .find(|row| {
            matches!(
                row.reason_class,
                DebugPrepSeedPrerequisiteReasonClass::PendingTrust
            )
        })
        .expect("pending-trust row");
    assert_eq!(
        trust_row.degraded_reason,
        Some(DegradedFieldReason::TrustStateUnresolved)
    );

    let inspector = surface
        .actions
        .iter()
        .find(|a| {
            matches!(
                a.action_class,
                DebugPrepSeedActionClass::OpenExecutionContextInspector
            )
        })
        .expect("inspector action");
    assert_eq!(
        inspector.availability,
        DebugPrepSeedActionAvailability::Live
    );
    let terminal = surface
        .actions
        .iter()
        .find(|a| {
            matches!(
                a.action_class,
                DebugPrepSeedActionClass::OpenInvokingTerminal
            )
        })
        .expect("terminal action");
    assert_eq!(
        terminal.availability,
        DebugPrepSeedActionAvailability::BlockedByPendingTrust
    );
}

#[test]
fn remote_target_lights_boundary_cue_consistently_with_terminal_badge() {
    let mut resolver = baseline_resolver();
    let mut request = ExecutionContextRequest::debug_prep_seed(
        "debug.prep.remote_attach",
        TrustState::Trusted,
        "mono:1",
    );
    request.override_target_class = Some(TargetClass::SshRemote);
    request.override_working_directory = Some("/srv/code");
    let context = resolver.resolve(request);

    let surface = DebugPrepSeedSurface::project(&context);
    assert_eq!(surface.target_class, TargetBadgeClass::RemoteHost);
    assert_eq!(surface.boundary_cue, HostBoundaryCue::LocalToRemote);
    assert!(surface.boundary_cue_visible);
    assert_eq!(surface.boundary_cue, surface.badge.boundary_cue);
}

#[test]
fn policy_blocked_activator_renders_blocked_by_policy_actions() {
    let mut resolver = baseline_resolver();
    let mut context = resolver.resolve(ExecutionContextRequest::debug_prep_seed(
        "debug.prep.attach",
        TrustState::Trusted,
        "mono:2",
    ));
    context.degraded_fields.push(DegradedFieldRecord {
        field_path: "toolchain_identity.activation_strategy".to_owned(),
        reason: DegradedFieldReason::ActivatorBlockedByPolicy,
        repair_hook_ref: None,
    });

    let surface = DebugPrepSeedSurface::project(&context);
    assert!(surface.honesty_marker_present);
    assert!(surface.blocked_prerequisites.iter().any(|row| matches!(
        row.reason_class,
        DebugPrepSeedPrerequisiteReasonClass::PolicyBlocked
    )));
    let terminal = surface
        .actions
        .iter()
        .find(|a| {
            matches!(
                a.action_class,
                DebugPrepSeedActionClass::OpenInvokingTerminal
            )
        })
        .unwrap();
    assert_eq!(
        terminal.availability,
        DebugPrepSeedActionAvailability::BlockedByPolicy
    );
}

#[test]
fn policy_blocked_target_reachability_renders_a_prerequisite_row() {
    let mut resolver = baseline_resolver();
    let mut context = resolver.resolve(ExecutionContextRequest::debug_prep_seed(
        "debug.prep.attach",
        TrustState::Trusted,
        "mono:3",
    ));
    context.target_identity.reachability_state = ReachabilityState::PolicyBlocked;
    context.target_identity.local_vs_managed_boundary_visible = false;

    let surface = DebugPrepSeedSurface::project(&context);
    assert!(surface
        .blocked_prerequisites
        .iter()
        .any(|row| row.field_path.as_deref() == Some("target_identity.reachability_state")));
    assert!(surface.honesty_marker_present);
}

#[test]
fn render_plaintext_quotes_actions_and_seed_notice() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::debug_prep_seed(
        "debug.prep.attach",
        TrustState::Trusted,
        "mono:0",
    ));
    let surface = DebugPrepSeedSurface::project(&context);

    let block = surface.render_plaintext();
    assert!(block.contains("Debug-prep seed surface"));
    assert!(block.contains("Workspace: ws-test"));
    assert!(block.contains("Runtime: Debug-adapter runtime"));
    assert!(block.contains("attach_to_running_process"));
    assert!(block.contains("reserved_for_later_milestone"));
    assert!(block.contains("reserved for a later milestone"));
}

#[test]
fn fixture_protected_walk_replays_into_debug_prep_surface_projection() {
    let fixture: DebugPrepSeedFixture = load_fixture("debug_prep_seed_protected_walk_local.json");
    let surface = build_surface_from_fixture(&fixture);
    assert_surface_matches(&surface, &fixture.expect);
}

#[test]
fn fixture_failure_drill_replays_pending_trust_honesty_row() {
    let fixture: DebugPrepSeedFixture =
        load_fixture("debug_prep_seed_pending_trust_failure_drill.json");
    let surface = build_surface_from_fixture(&fixture);
    assert_surface_matches(&surface, &fixture.expect);
    assert!(surface.blocked_prerequisites.iter().any(|row| matches!(
        row.reason_class,
        DebugPrepSeedPrerequisiteReasonClass::PendingTrust
    )));
}

fn load_fixture(name: &str) -> DebugPrepSeedFixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/task_debug_seed_cases")
        .join(name);
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    serde_json::from_str(&payload).expect("fixture must parse")
}

fn build_surface_from_fixture(fixture: &DebugPrepSeedFixture) -> DebugPrepSeedSurface {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest {
        command_id: &fixture.input.command_id,
        surface: aureline_runtime::SurfaceClass::Debug,
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
    DebugPrepSeedSurface::project(&context)
}

fn assert_surface_matches(surface: &DebugPrepSeedSurface, expect: &DebugPrepSeedExpect) {
    assert_eq!(surface.record_kind, expect.record_kind);
    assert_eq!(surface.target_class_token, expect.target_class_token);
    assert_eq!(surface.target_label, expect.target_label);
    assert_eq!(surface.boundary_cue_token, expect.boundary_cue_token);
    assert_eq!(surface.boundary_cue_visible, expect.boundary_cue_visible);
    assert_eq!(surface.toolchain_class_token, expect.toolchain_class_token);
    assert_eq!(surface.trust_state_token, expect.trust_state_token);
    assert_eq!(
        surface.honesty_marker_present,
        expect.honesty_marker_present
    );
    if let Some(expected_cwd) = &expect.working_directory {
        assert_eq!(
            surface.working_directory.as_deref(),
            Some(expected_cwd.as_str())
        );
    }
    let inspector = surface
        .actions
        .iter()
        .find(|a| {
            matches!(
                a.action_class,
                DebugPrepSeedActionClass::OpenExecutionContextInspector
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
        .find(|a| {
            matches!(
                a.action_class,
                DebugPrepSeedActionClass::OpenInvokingTerminal
            )
        })
        .expect("terminal action");
    assert_eq!(
        terminal.availability_token,
        expect.terminal_action_availability
    );
    let attach = surface
        .actions
        .iter()
        .find(|a| {
            matches!(
                a.action_class,
                DebugPrepSeedActionClass::AttachToRunningProcess
            )
        })
        .expect("attach action");
    assert_eq!(attach.availability_token, expect.attach_action_availability);
}

#[derive(Debug, Deserialize)]
struct DebugPrepSeedFixture {
    input: DebugPrepSeedFixtureInput,
    expect: DebugPrepSeedExpect,
}

#[derive(Debug, Deserialize)]
struct DebugPrepSeedFixtureInput {
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
struct DebugPrepSeedExpect {
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
    attach_action_availability: String,
}
