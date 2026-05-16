use super::*;

use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest, ExecutionContextResolver,
    ExecutionContextResolverConfig, IdentityMode, LaunchProfileAdapterBinding,
    LaunchProfileArguments, LaunchProfileCreateRequest, LaunchProfileEditClass,
    LaunchProfileEnvironmentBinding, LaunchProfileKind, LaunchProfileMode,
    LaunchProfileSideEffectClass, LaunchProfileStore, LaunchProfileTargetBinding, ScopeClass,
    TargetClass, TrustState,
};

fn resolver(workspace_id: &str, cwd: &str) -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: workspace_id.to_owned(),
        profile_id: Some("profile:default".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some(cwd.to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: format!("caps:{workspace_id}"),
            capsule_hash: format!("sha256:{workspace_id}"),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "run-debug-profiles-test-resolver".to_owned(),
    })
}

fn seeded_store() -> LaunchProfileStore {
    let mut store = LaunchProfileStore::new("workspace:web");
    store
        .create_profile(LaunchProfileCreateRequest {
            profile_id: "launch:web-debug".to_owned(),
            workspace_id: "workspace:web".to_owned(),
            display_name: "Web debug".to_owned(),
            mode: LaunchProfileMode::Launch,
            kind: LaunchProfileKind::Debug,
            target: LaunchProfileTargetBinding {
                canonical_target_id: "localhost:darwin-arm64".to_owned(),
                target_class_token: "local_host".to_owned(),
                target_label: "Local desktop".to_owned(),
                working_directory: Some("/workspace/web".to_owned()),
                scope_class_token: "current_root".to_owned(),
            },
            adapter: Some(LaunchProfileAdapterBinding {
                adapter_id: "adapter:node:dap".to_owned(),
                adapter_label: "Node DAP adapter".to_owned(),
                transport_class_token: "local_sidecar_stdio".to_owned(),
                requested_dap_protocol_version: "DAP/1.55".to_owned(),
                required_capability_tokens: vec!["function_breakpoints".to_owned()],
            }),
            environment: LaunchProfileEnvironmentBinding {
                capsule_id: "caps:workspace:web".to_owned(),
                capsule_hash: "sha256:workspace:web".to_owned(),
                declared_overlay_keys: vec!["NODE_ENV".to_owned()],
            },
            arguments: LaunchProfileArguments {
                program: Some("./node_modules/.bin/jest".to_owned()),
                args: vec!["--runInBand".to_owned()],
                working_directory: Some("/workspace/web".to_owned()),
                attach_process_id: None,
            },
            declared_side_effects: vec![LaunchProfileSideEffectClass::TargetProcessSpawn],
            observed_at: "2026-05-13T18:00:00Z".to_owned(),
        })
        .expect("create profile");
    store
}

#[test]
fn projection_renders_ready_profile_without_disclosure() {
    let store = seeded_store();
    let mut resolver = resolver("workspace:web", "/workspace/web");
    let context = resolver.resolve(ExecutionContextRequest::debug_prep_seed(
        "debug.run.profile",
        TrustState::Trusted,
        "2026-05-13T18:00:01Z",
    ));
    let export = store.support_export("support:lp:01", "2026-05-13T18:00:02Z", |id| {
        if id == "launch:web-debug" {
            Some(context.clone())
        } else {
            None
        }
    });
    let projection = RunDebugProfilesBetaProjection::project(&export);
    assert_eq!(
        projection.record_kind,
        RUN_DEBUG_PROFILES_BETA_PROJECTION_RECORD_KIND
    );
    assert_eq!(projection.workspace_id, "workspace:web");
    assert_eq!(projection.profiles.len(), 1);
    assert!(!projection.honesty_marker_present);
    let row = &projection.profiles[0];
    assert_eq!(
        row.preview_state_token.as_deref(),
        Some("ready_to_dispatch")
    );
    assert!(!row.requires_review_before_dispatch);
    assert!(row.current_target_reachable);
    assert!(row.drift_field_paths.is_empty());
    assert!(row.invalid_reason_token.is_none());
    assert_eq!(row.adapter_id.as_deref(), Some("adapter:node:dap"));
}

#[test]
fn projection_lights_honesty_marker_on_drift_and_includes_rollback_history() {
    let mut store = seeded_store();
    store
        .apply_edit(
            "launch:web-debug",
            LaunchProfileEditClass::RenamedDisplayName,
            "2026-05-13T18:00:03Z",
            |mutable| {
                *mutable.display_name = "Web debug (renamed)".to_owned();
            },
        )
        .expect("rename");
    let revisions = store
        .revisions("launch:web-debug")
        .expect("revisions")
        .to_vec();
    let original_revision_id = revisions[0].revision_id.clone();
    store
        .rollback_to(
            "launch:web-debug",
            &original_revision_id,
            "2026-05-13T18:00:04Z",
        )
        .expect("rollback");

    let mut resolver = resolver("workspace:web", "/workspace/web");
    let mut request = ExecutionContextRequest::debug_prep_seed(
        "debug.run.profile",
        TrustState::Trusted,
        "2026-05-13T18:00:05Z",
    );
    request.override_target_class = Some(TargetClass::ManagedWorkspace);
    request.override_working_directory = Some("/srv/web");
    let context = resolver.resolve(request);

    let export = store.support_export("support:lp:02", "2026-05-13T18:00:06Z", |id| {
        if id == "launch:web-debug" {
            Some(context.clone())
        } else {
            None
        }
    });
    let projection = RunDebugProfilesBetaProjection::project(&export);
    assert!(projection.honesty_marker_present);
    let row = &projection.profiles[0];
    assert_eq!(
        row.preview_state_token.as_deref(),
        Some("drift_requires_review")
    );
    assert!(row.requires_review_before_dispatch);
    assert!(row
        .drift_field_paths
        .iter()
        .any(|path| path == "target_binding.target_class"));
    // Edit lineage retains created, rename, rollback
    assert_eq!(row.edit_lineage_summary.len(), 3);
    let lineage = row.edit_lineage_summary.join("\n");
    assert!(lineage.contains("created"));
    assert!(lineage.contains("renamed_display_name"));
    assert!(lineage.contains("rolled_back"));

    let rendered = projection.render_plaintext();
    assert!(rendered.contains("Web debug"));
    assert!(rendered.contains("drift_requires_review"));
}
