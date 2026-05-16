use super::*;

use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest, ExecutionContextResolver,
    ExecutionContextResolverConfig, IdentityMode, ScopeClass, TargetClass,
    TargetDiscoveryBetaSupportExport, ToolchainClass, TrustState,
};

fn resolver() -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: "workspace:shell:target-discovery-beta".to_owned(),
        profile_id: Some("profile:default".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/workspace/shell".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "caps:shell".to_owned(),
            capsule_hash: "sha256:shell".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "shell-target-discovery-beta-test".to_owned(),
    })
}

#[test]
fn panel_renders_one_row_per_runtime_row_and_blocks_helper_dispatch() {
    let mut resolver = resolver();
    let local = resolver.resolve(ExecutionContextRequest::task_seed(
        "task.run.local",
        TrustState::Trusted,
        "2026-05-15T19:40:00Z",
    ));
    let mut helper_request = ExecutionContextRequest::task_seed(
        "task.run.helper",
        TrustState::Restricted,
        "2026-05-15T19:41:00Z",
    );
    helper_request.requested_target_class = Some(TargetClass::ManagedWorkspace);
    helper_request.requested_toolchain_class = Some(ToolchainClass::BuildDriverRuntime);
    let helper = resolver.resolve(helper_request);

    let export = TargetDiscoveryBetaSupportExport::from_contexts(
        "support-export:shell:target-discovery-beta",
        "2026-05-15T19:42:00Z",
        [&local, &helper],
    );
    let panel = TargetDiscoveryBetaPanel::from_support_export(&export);

    assert_eq!(panel.record_kind, TARGET_DISCOVERY_BETA_PANEL_RECORD_KIND);
    assert_eq!(panel.rows.len(), 2);
    assert!(panel.any_row_blocks_protected_dispatch);

    let local_row = panel
        .rows
        .iter()
        .find(|r| r.target_class_token == "local_host")
        .expect("local row");
    assert_eq!(local_row.discovery_source_token, "native_protocol");
    assert!(!local_row.blocks_all_protected_dispatch);

    let helper_row = panel
        .rows
        .iter()
        .find(|r| r.target_class_token == "managed_workspace")
        .expect("helper row");
    assert_eq!(helper_row.discovery_source_token, "structured_adapter");
    assert_eq!(helper_row.discovery_freshness_token, "stale_imported");
    // Helper row blocks every dispatch action, but export_artifact is still
    // marked allowed in the per-action list.
    assert!(helper_row
        .decision_summaries
        .iter()
        .any(|line| line == "dispatch_run=blocked_freshness_stale"));
    assert!(helper_row
        .decision_summaries
        .iter()
        .any(|line| line == "export_artifact=allowed"));

    let plaintext = panel.render_plaintext();
    assert!(plaintext.contains("Source: native_protocol"));
    assert!(plaintext.contains("Source: structured_adapter"));
    assert!(plaintext.contains("dispatch_run=blocked_freshness_stale"));
    assert!(plaintext.contains("export_artifact=allowed"));
    // Working directory string must not leak into the panel.
    assert!(!plaintext.contains("/workspace/shell"));
}
