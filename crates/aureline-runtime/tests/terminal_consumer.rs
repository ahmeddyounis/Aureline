//! Integration coverage proving the terminal pane consumer reads the same
//! [`aureline_runtime::ExecutionContext`] object the task and debug-prep seeds
//! resolve through.
//!
//! The PTY host is the named integration touchpoint for the M1 seed: every
//! `OpenSessionRequest` quotes the resolver's `execution_context_id`, the
//! `target_class` projects onto the host class, and the local-vs-managed
//! boundary cue follows from the resolved [`aureline_runtime::TargetIdentity`].
//! The fixture `fixtures/runtime/execution_context_seed_cases/conflicting_inputs.json`
//! seeds the failure drill end-to-end so a support export can quote the same
//! truth a tab strip renders.

use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContext, ExecutionContextRequest,
    ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
    SurfaceClass, TargetClass, ToolchainClass, TrustState,
};
use aureline_terminal::pty_host::{HostClass, OpenSessionRequest, PtyHost};

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

fn host_class_for(context: &ExecutionContext) -> HostClass {
    match context.target_identity.target_class {
        TargetClass::LocalHost => HostClass::HostDesktop,
        TargetClass::ContainerLocal | TargetClass::Devcontainer => HostClass::LocalContainer,
        _ => HostClass::RemoteAgentPrimary,
    }
}

#[test]
fn terminal_pane_session_quotes_resolver_execution_context_id() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));

    let mut host = PtyHost::new();
    let session_id = host.open_session(OpenSessionRequest {
        workspace_id: &context.invocation_subject.workspace_id,
        host_class: host_class_for(&context),
        display_title: "zsh",
        cwd_hint: context.target_identity.working_directory.as_deref(),
        execution_context_ref: context.execution_context_id(),
        trust_state: context.policy_and_trust.trust_state,
        observed_at: "mono:0",
    });

    let session = host.session(&session_id).expect("session must exist");
    assert_eq!(
        session.header().execution_context_ref,
        context.execution_context_id()
    );
    assert_eq!(session.header().target_badge, "Local");
    assert!(!session.header().needs_boundary_cue());
    assert_eq!(
        session.header().cwd_hint.as_deref(),
        Some("/workspace"),
        "session quotes the resolver-resolved working directory verbatim"
    );
}

#[test]
fn task_and_debug_seeds_resolve_through_the_same_resolver_and_match_terminal_context_shape() {
    let mut resolver = baseline_resolver();
    let terminal = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));
    let task = resolver.resolve(ExecutionContextRequest::task_seed(
        "task.run.cargo_build",
        TrustState::Trusted,
        "mono:1",
    ));
    let debug = resolver.resolve(ExecutionContextRequest::debug_prep_seed(
        "debug.prep.attach",
        TrustState::Trusted,
        "mono:2",
    ));

    // Same canonical record shape: schema version, scope class, identity mode,
    // and trust state come from the same authority for every lane.
    for ctx in [&terminal, &task, &debug] {
        assert_eq!(ctx.schema_version, terminal.schema_version);
        assert_eq!(ctx.workset_scope_class, ScopeClass::CurrentRoot);
        assert_eq!(
            ctx.policy_and_trust.identity_mode,
            IdentityMode::AccountFreeLocal
        );
        assert_eq!(ctx.policy_and_trust.trust_state, TrustState::Trusted);
        assert_eq!(ctx.target_identity.target_class, TargetClass::LocalHost);
    }

    // Each lane carries its own surface and toolchain class so a downstream
    // consumer (terminal pane chip, task channel header, debug-prep stub) can
    // tell which lane minted the record.
    assert_eq!(terminal.invocation_subject.surface, SurfaceClass::Terminal);
    assert_eq!(task.invocation_subject.surface, SurfaceClass::Task);
    assert_eq!(debug.invocation_subject.surface, SurfaceClass::Debug);
    assert_eq!(
        terminal.toolchain_identity.toolchain_class,
        ToolchainClass::LoginShell
    );
    assert_eq!(
        task.toolchain_identity.toolchain_class,
        ToolchainClass::BuildDriverRuntime
    );
    assert_eq!(
        debug.toolchain_identity.toolchain_class,
        ToolchainClass::DebugAdapterRuntime
    );

    // The terminal pane consumer wires only the terminal context onto the PTY
    // host today, but the task and debug seeds are addressable through the
    // same resolver and produce records the same downstream surfaces could
    // read without forking truth.
    let mut host = PtyHost::new();
    let session_id = host.open_session(OpenSessionRequest {
        workspace_id: &terminal.invocation_subject.workspace_id,
        host_class: host_class_for(&terminal),
        display_title: "zsh",
        cwd_hint: terminal.target_identity.working_directory.as_deref(),
        execution_context_ref: terminal.execution_context_id(),
        trust_state: terminal.policy_and_trust.trust_state,
        observed_at: "mono:0",
    });
    let session = host.session(&session_id).expect("session must exist");
    assert_eq!(
        session.header().execution_context_ref,
        terminal.execution_context_id()
    );
}

#[test]
fn explicit_override_to_remote_target_lights_the_boundary_cue_on_the_session_header() {
    // Failure drill: the terminal pane requests a local target while the
    // caller supplies an explicit remote override. The resolver records the
    // winning source on the provenance row and the PTY session quotes the
    // remote canonical target id; the chrome MUST render the local-vs-managed
    // boundary cue.
    let mut resolver = baseline_resolver();
    let mut request = ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    );
    request.override_target_class = Some(TargetClass::SshRemote);
    request.override_working_directory = Some("/srv/code");

    let context = resolver.resolve(request);
    assert_eq!(context.target_identity.target_class, TargetClass::SshRemote);
    assert!(context.boundary_cue_visible());

    let mut host = PtyHost::new();
    let session_id = host.open_session(OpenSessionRequest {
        workspace_id: &context.invocation_subject.workspace_id,
        host_class: host_class_for(&context),
        display_title: "agent shell",
        cwd_hint: context.target_identity.working_directory.as_deref(),
        execution_context_ref: context.execution_context_id(),
        trust_state: context.policy_and_trust.trust_state,
        observed_at: "mono:0",
    });
    let session = host.session(&session_id).expect("session must exist");
    assert!(session.header().needs_boundary_cue());
    assert_eq!(session.header().target_badge, "Remote");
    assert_eq!(
        session.header().cwd_hint.as_deref(),
        Some("/srv/code"),
        "session quotes the override-resolved working directory verbatim"
    );
}
