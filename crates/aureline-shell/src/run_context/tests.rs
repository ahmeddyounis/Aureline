use std::path::Path;

use serde::Deserialize;

use aureline_runtime::{
    ActorClass, CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
    ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ResolverInputField,
    ScopeClass, SurfaceClass, TargetClass, ToolchainClass, TrustState,
};
use aureline_terminal::pty_host::{HostClass, OpenSessionRequest, PtyHost};

use super::*;
use crate::terminal_pane::TerminalPaneSnapshot;

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
fn entry_surfaces_share_one_summary_shape_from_fixture() {
    let fixture: EntryPointsFixture = load_fixture("all_entry_points_share_summary.json");
    let mut resolver = baseline_resolver();
    let mut entries = Vec::new();

    for input in &fixture.inputs {
        let context = resolver.resolve(input.request());
        let surface = ExecutionEntrySurface::project(input.entry_point, &context);

        assert_eq!(surface.record_kind, EXECUTION_ENTRY_SURFACE_RECORD_KIND);
        assert!(surface.surface_matches_context);
        assert_eq!(
            surface.context_summary.record_kind,
            RUN_CONTEXT_SUMMARY_RECORD_KIND
        );
        assert_eq!(
            surface.context_summary.target_class_token,
            input
                .requested_target_class
                .expect("fixture supplies target")
                .as_str()
        );
        assert_eq!(
            surface.context_summary.toolchain_class_token,
            input
                .requested_toolchain_class
                .expect("fixture supplies toolchain")
                .as_str()
        );
        assert!(surface
            .context_summary
            .input_decisions
            .iter()
            .any(|decision| decision.field == ResolverInputField::TargetClass));
        assert!(surface
            .context_summary
            .explanation_reason_code_tokens
            .contains(&"shared_context_contract".to_owned()));

        entries.push(surface);
    }

    let snapshot = ExecutionEntryTruthSnapshot::from_entries("ws-test", entries);
    assert_eq!(
        snapshot.record_kind,
        EXECUTION_ENTRY_TRUTH_SNAPSHOT_RECORD_KIND
    );
    assert_eq!(snapshot.entries.len(), fixture.expect.entry_count);
    assert!(snapshot.all_entries_share_summary_shape);
    assert!(!snapshot.any_entry_requires_review_before_dispatch);
    for entry_point in &fixture.expect.required_entry_points {
        assert!(
            snapshot.entry(*entry_point).is_some(),
            "missing entry point {entry_point:?}"
        );
    }
}

#[test]
fn terminal_tab_can_join_the_shared_summary_by_execution_context_ref() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));

    let mut host = PtyHost::new();
    host.open_session(OpenSessionRequest {
        workspace_id: &context.invocation_subject.workspace_id,
        host_class: HostClass::HostDesktop,
        display_title: "zsh",
        cwd_hint: context.target_identity.working_directory.as_deref(),
        execution_context_ref: context.execution_context_id(),
        trust_state: context.policy_and_trust.trust_state,
        observed_at: "mono:0",
    });

    let snapshot = TerminalPaneSnapshot::project("ws-test", &host).with_run_contexts([&context]);
    let tab = snapshot.tabs.first().expect("terminal tab");
    let summary = tab.context_summary.as_ref().expect("summary is joined");
    assert_eq!(summary.execution_context_ref, tab.execution_context_ref);
    assert_eq!(summary.surface, SurfaceClass::Terminal);
    assert_eq!(summary.toolchain_class_token, "login_shell");
}

#[test]
fn exact_rerun_current_drift_is_visible_before_dispatch() {
    let fixture: ExactRerunFixture = load_fixture("exact_rerun_current_drift.json");
    let mut resolver = baseline_resolver();
    let exact = resolver.resolve(fixture.exact.request());
    let current = resolver.resolve(fixture.current.request());

    let surface = ExecutionEntrySurface::project_with_exact_rerun(
        fixture.expect.entry_point,
        &current,
        Some(&exact),
    );
    assert!(surface.requires_review_before_dispatch);
    let comparison = surface
        .exact_rerun_comparison
        .as_ref()
        .expect("comparison is attached");
    assert_eq!(comparison.has_drift, fixture.expect.has_drift);
    assert_eq!(
        comparison.requires_review_before_dispatch,
        fixture.expect.requires_review_before_dispatch
    );
    assert_eq!(
        comparison.current_summary.mixed_version_state_token,
        fixture.expect.current_mixed_version_state
    );
    for required in &fixture.expect.required_diff_fields {
        assert!(
            comparison
                .diff_rows
                .iter()
                .any(|row| row.field_path == *required),
            "missing diff field {required}"
        );
    }

    let block = surface.render_plaintext();
    assert!(block.contains("Exact rerun drift: true"));
    assert!(block.contains("target_identity.target_class"));
    assert!(block.contains("policy_and_trust.trust_state"));
}

fn load_fixture<T>(name: &str) -> T
where
    T: for<'de> Deserialize<'de>,
{
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/execution_entry_points")
        .join(name);
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    serde_json::from_str(&payload).expect("fixture must parse")
}

#[derive(Debug, Deserialize)]
struct EntryPointsFixture {
    inputs: Vec<EntryInput>,
    expect: EntryPointsExpect,
}

#[derive(Debug, Deserialize)]
struct EntryPointsExpect {
    entry_count: usize,
    required_entry_points: Vec<ExecutionEntryPoint>,
}

#[derive(Debug, Deserialize)]
struct ExactRerunFixture {
    exact: EntryInput,
    current: EntryInput,
    expect: ExactRerunExpect,
}

#[derive(Debug, Deserialize)]
struct ExactRerunExpect {
    entry_point: ExecutionEntryPoint,
    has_drift: bool,
    requires_review_before_dispatch: bool,
    current_mixed_version_state: String,
    required_diff_fields: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct EntryInput {
    entry_point: ExecutionEntryPoint,
    command_id: String,
    surface: SurfaceClass,
    actor_class: ActorClass,
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

impl EntryInput {
    fn request(&self) -> ExecutionContextRequest<'_> {
        ExecutionContextRequest {
            command_id: &self.command_id,
            surface: self.surface,
            actor_class: self.actor_class,
            trust_state: self.trust_state,
            observed_at: &self.observed_at,
            requested_target_class: self.requested_target_class,
            requested_working_directory: self.requested_working_directory.as_deref(),
            requested_toolchain_class: self.requested_toolchain_class,
            override_target_class: self.override_target_class,
            override_working_directory: self.override_working_directory.as_deref(),
            override_toolchain_class: self.override_toolchain_class,
        }
    }
}
