use super::*;

use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest, ExecutionContextResolver,
    ExecutionContextResolverConfig, IdentityMode, PytestDiscoverer, PytestDiscovererConfig,
    PythonEnvironmentDetectorConfig, RerunLastLoop, RerunTargetMode, ScopeClass, TargetClass,
    TestRunnerBetaProjection, TestRunnerBetaSupportExport, TrustState,
};

fn ready_uv_workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/python_task_discovery_alpha/ready_uv")
}

fn baseline_resolver() -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: "workspace:python".to_owned(),
        profile_id: Some("profile:default".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/workspace/python".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "caps:workspace:python".to_owned(),
            capsule_hash: "sha256:workspace:python".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "test-runner-beta-shell-test-resolver".to_owned(),
    })
}

fn pytest_discoverer() -> PytestDiscoverer {
    PytestDiscoverer::new(PytestDiscovererConfig {
        python_detector: PythonEnvironmentDetectorConfig {
            ambient_python_version: Some("3.12.7".to_owned()),
            ambient_interpreter_ref: Some("/usr/bin/python3".to_owned()),
            ambient_uv_version: Some("0.5.7".to_owned()),
            ambient_poetry_version: Some("1.8.4".to_owned()),
            ..PythonEnvironmentDetectorConfig::default()
        },
        workspace_revision: Some("rev:python".to_owned()),
    })
}

#[test]
fn projection_renders_tree_inline_and_parity_rows_for_pytest_workspace() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::test_seed(
        "test.run.pytest",
        TrustState::Trusted,
        "2026-05-15T10:00:00Z",
    ));
    let discovery = pytest_discoverer().discover_workspace(
        &ready_uv_workspace_root(),
        context.clone(),
        "2026-05-15T10:00:01Z",
    );
    let attempt_packets = discovery.test_attempt_alpha_packets("2026-05-15T10:00:02Z");
    let runtime_projection = TestRunnerBetaProjection::from_pytest_discovery(
        &discovery,
        &context,
        attempt_packets,
        None,
        None,
        "2026-05-15T10:00:03Z",
    );
    let export = TestRunnerBetaSupportExport::from_projection(
        &runtime_projection,
        "test-runner-beta:m3:canonical",
        "2026-05-15T10:00:04Z",
    );

    let projection = TestRunnerBetaProjectionView::project(
        &export,
        &runtime_projection.tree.rows,
        &runtime_projection.inline.rows,
    );

    assert_eq!(
        projection.record_kind,
        TEST_RUNNER_BETA_PROJECTION_RECORD_KIND
    );
    assert_eq!(projection.workspace_id, "workspace:python");
    assert!(projection.framework_tokens.contains(&"pytest".to_owned()));
    assert!(!projection.tree_rows.is_empty());
    assert!(!projection.inline_rows.is_empty());
    assert_eq!(projection.parity_rows.len(), projection.inline_rows.len());
    for parity in &projection.parity_rows {
        assert_eq!(parity.rerun_last_command_id, "cmd:test.rerun_last");
        assert_eq!(parity.agreement_state_token, "rerun_lane_unset");
    }
    assert!(!projection.artifact_rows.is_empty());

    let rendered = projection.render_plaintext();
    assert!(rendered.contains("Test runner (beta)"));
    assert!(rendered.contains("Workspace: workspace:python"));
    assert!(rendered.contains("cmd:test.rerun_last"));
    assert!(rendered.contains("rerun_lane_unset"));
}

#[test]
fn projection_marks_rows_agree_after_rerun_last_remembers_a_pytest_attempt() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::test_seed(
        "test.run.pytest",
        TrustState::Trusted,
        "2026-05-15T10:10:00Z",
    ));
    let discovery = pytest_discoverer().discover_workspace(
        &ready_uv_workspace_root(),
        context.clone(),
        "2026-05-15T10:10:01Z",
    );
    let target_contract = discovery
        .run_contracts
        .iter()
        .find(|contract| {
            contract
                .selection
                .test_item_id
                .as_deref()
                .map(|item_id| item_id.contains("test_health"))
                .unwrap_or(false)
        })
        .expect("a pytest contract for test_health")
        .clone();

    let mut loop_state = RerunLastLoop::new();
    loop_state.remember_pytest(
        target_contract.clone(),
        context.clone(),
        "2026-05-15T10:10:02Z",
    );
    let prepared = loop_state.prepare_last_test(
        &context,
        RerunTargetMode::ExactPriorTarget,
        "2026-05-15T10:10:03Z",
    );

    let attempt_packets = discovery.test_attempt_alpha_packets("2026-05-15T10:10:04Z");
    let runtime_projection = TestRunnerBetaProjection::from_pytest_discovery(
        &discovery,
        &context,
        attempt_packets,
        loop_state.last_launch(aureline_runtime::RerunLane::Test),
        Some(&prepared),
        "2026-05-15T10:10:05Z",
    );
    let export = TestRunnerBetaSupportExport::from_projection(
        &runtime_projection,
        "test-runner-beta:m3:canonical",
        "2026-05-15T10:10:06Z",
    );

    let projection = TestRunnerBetaProjectionView::project(
        &export,
        &runtime_projection.tree.rows,
        &runtime_projection.inline.rows,
    );

    let remembered_canonical = target_contract
        .selection
        .test_item_id
        .clone()
        .expect("pytest contract has test_item_id");
    let parity_for_remembered = projection
        .parity_rows
        .iter()
        .find(|parity| parity.canonical_test_item_ref == remembered_canonical)
        .expect("parity row for remembered case");
    assert_eq!(parity_for_remembered.agreement_state_token, "rows_agree");

    // Other case rows still report rerun_lane_unset because the loop only
    // remembers one launch.
    let other_rows: Vec<_> = projection
        .parity_rows
        .iter()
        .filter(|parity| parity.canonical_test_item_ref != remembered_canonical)
        .collect();
    assert!(other_rows
        .iter()
        .all(|parity| parity.agreement_state_token == "rerun_lane_unset"));
}
