use super::*;
use crate::execution_context::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest, ExecutionContextResolver,
    ExecutionContextResolverConfig, IdentityMode, ScopeClass, TargetClass, TrustState,
};

fn baseline_resolver(workspace_id: &str, cwd: &str) -> ExecutionContextResolver {
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
        resolver_version: "launch-profile-test-resolver".to_owned(),
    })
}

fn debug_context(resolver: &mut ExecutionContextResolver, observed_at: &str) -> ExecutionContext {
    resolver.resolve(ExecutionContextRequest::debug_prep_seed(
        "debug.run.profile",
        TrustState::Trusted,
        observed_at,
    ))
}

fn baseline_target() -> LaunchProfileTargetBinding {
    LaunchProfileTargetBinding {
        canonical_target_id: "localhost:darwin-arm64".to_owned(),
        target_class_token: "local_host".to_owned(),
        target_label: "Local desktop".to_owned(),
        working_directory: Some("/workspace/web".to_owned()),
        scope_class_token: "current_root".to_owned(),
    }
}

fn baseline_adapter() -> LaunchProfileAdapterBinding {
    LaunchProfileAdapterBinding {
        adapter_id: "adapter:node:dap".to_owned(),
        adapter_label: "Node DAP adapter".to_owned(),
        transport_class_token: "local_sidecar_stdio".to_owned(),
        requested_dap_protocol_version: "DAP/1.55".to_owned(),
        required_capability_tokens: vec!["function_breakpoints".to_owned()],
    }
}

fn baseline_environment(workspace_id: &str) -> LaunchProfileEnvironmentBinding {
    LaunchProfileEnvironmentBinding {
        capsule_id: format!("caps:{workspace_id}"),
        capsule_hash: format!("sha256:{workspace_id}"),
        declared_overlay_keys: vec!["NODE_ENV".to_owned()],
    }
}

fn launch_arguments() -> LaunchProfileArguments {
    LaunchProfileArguments {
        program: Some("./node_modules/.bin/jest".to_owned()),
        args: vec!["--runInBand".to_owned()],
        working_directory: Some("/workspace/web".to_owned()),
        attach_process_id: None,
    }
}

fn debug_launch_request() -> LaunchProfileCreateRequest {
    LaunchProfileCreateRequest {
        profile_id: "launch:web-debug".to_owned(),
        workspace_id: "workspace:web".to_owned(),
        display_name: "Web debug".to_owned(),
        mode: LaunchProfileMode::Launch,
        kind: LaunchProfileKind::Debug,
        target: baseline_target(),
        adapter: Some(baseline_adapter()),
        environment: baseline_environment("workspace:web"),
        arguments: launch_arguments(),
        declared_side_effects: vec![
            LaunchProfileSideEffectClass::TargetProcessSpawn,
            LaunchProfileSideEffectClass::InboundNetworkListener,
        ],
        observed_at: "2026-05-13T18:00:00Z".to_owned(),
    }
}

#[test]
fn create_profile_seeds_initial_revision_with_sorted_side_effects() {
    let mut store = LaunchProfileStore::new("workspace:web");
    let revision = store
        .create_profile(debug_launch_request())
        .expect("create profile");
    assert_eq!(revision.parent_revision_id, None);
    assert_eq!(revision.snapshot.revision_id, revision.revision_id);
    assert_eq!(
        revision.snapshot.declared_side_effects,
        vec![
            LaunchProfileSideEffectClass::TargetProcessSpawn,
            LaunchProfileSideEffectClass::InboundNetworkListener,
        ]
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
    );
    let profile = store.profile("launch:web-debug").expect("profile present");
    assert_eq!(profile.mode_token, "launch");
    assert_eq!(profile.kind_token, "debug");
    assert_eq!(profile.parent_revision_id, None);
    assert_eq!(profile.last_edited_at, "2026-05-13T18:00:00Z");
}

#[test]
fn duplicate_profile_id_is_rejected() {
    let mut store = LaunchProfileStore::new("workspace:web");
    store
        .create_profile(debug_launch_request())
        .expect("create profile");
    let err = store
        .create_profile(debug_launch_request())
        .expect_err("duplicate is rejected");
    assert_eq!(
        err,
        LaunchProfileStoreError::DuplicateProfileId {
            profile_id: "launch:web-debug".to_owned()
        }
    );
}

#[test]
fn apply_edit_creates_new_revision_pointing_to_parent() {
    let mut store = LaunchProfileStore::new("workspace:web");
    let first = store
        .create_profile(debug_launch_request())
        .expect("create profile");
    let second = store
        .apply_edit(
            "launch:web-debug",
            LaunchProfileEditClass::RenamedDisplayName,
            "2026-05-13T18:01:00Z",
            |mutable| {
                *mutable.display_name = "Web debug (renamed)".to_owned();
            },
        )
        .expect("apply rename");
    assert_eq!(
        second.parent_revision_id.as_deref(),
        Some(first.revision_id.as_str())
    );
    assert_eq!(second.snapshot.display_name, "Web debug (renamed)");
    assert_eq!(
        second.edit.edit_class,
        LaunchProfileEditClass::RenamedDisplayName
    );
    let revisions = store.revisions("launch:web-debug").expect("revisions");
    assert_eq!(revisions.len(), 2);
    assert_eq!(
        revisions[0].edit.edit_class,
        LaunchProfileEditClass::Created
    );
    assert_eq!(
        revisions[1].edit.edit_class,
        LaunchProfileEditClass::RenamedDisplayName
    );
}

#[test]
fn rollback_to_prior_revision_creates_new_revision_with_old_snapshot() {
    let mut store = LaunchProfileStore::new("workspace:web");
    let first = store
        .create_profile(debug_launch_request())
        .expect("create profile");
    store
        .apply_edit(
            "launch:web-debug",
            LaunchProfileEditClass::RenamedDisplayName,
            "2026-05-13T18:01:00Z",
            |mutable| {
                *mutable.display_name = "Web debug (renamed)".to_owned();
            },
        )
        .expect("rename");
    let restored = store
        .rollback_to(
            "launch:web-debug",
            &first.revision_id,
            "2026-05-13T18:02:00Z",
        )
        .expect("rollback");
    assert_eq!(restored.edit.edit_class, LaunchProfileEditClass::RolledBack);
    assert_eq!(restored.snapshot.display_name, "Web debug");
    assert_eq!(
        restored.edit.rollback_target_revision_id.as_deref(),
        Some(first.revision_id.as_str())
    );
    let revisions = store.revisions("launch:web-debug").expect("revisions");
    assert_eq!(revisions.len(), 3);
    assert_eq!(revisions.last().unwrap().snapshot.display_name, "Web debug");
}

#[test]
fn rollback_unknown_revision_errors() {
    let mut store = LaunchProfileStore::new("workspace:web");
    store
        .create_profile(debug_launch_request())
        .expect("create profile");
    let err = store
        .rollback_to(
            "launch:web-debug",
            "rev:does-not-exist",
            "2026-05-13T18:02:00Z",
        )
        .expect_err("missing revision");
    assert!(matches!(
        err,
        LaunchProfileStoreError::RevisionNotFound { .. }
    ));
}

#[test]
fn preview_against_matching_context_is_ready_to_dispatch() {
    let mut store = LaunchProfileStore::new("workspace:web");
    store
        .create_profile(debug_launch_request())
        .expect("create profile");
    let mut resolver = baseline_resolver("workspace:web", "/workspace/web");
    let current = debug_context(&mut resolver, "2026-05-13T18:05:00Z");
    let preview = store
        .preview("launch:web-debug", &current, "2026-05-13T18:05:01Z")
        .expect("preview");
    assert_eq!(preview.state, LaunchProfilePreviewState::ReadyToDispatch);
    assert!(!preview.requires_review_before_dispatch);
    assert!(!preview.honesty_marker_present);
    assert!(preview.current_target_reachable);
    assert!(preview.drift_rows.is_empty());
    assert_eq!(
        preview.side_effect_disclosure_tokens,
        vec![
            "target_process_spawn".to_owned(),
            "inbound_network_listener".to_owned(),
        ]
    );
    // adapter rows always appear for debug profiles even when context is silent.
    assert!(preview
        .adapter_disclosure
        .iter()
        .any(|row| row.field_path == "adapter_binding.adapter_id"));
}

#[test]
fn preview_against_drifted_target_requires_review() {
    let mut store = LaunchProfileStore::new("workspace:web");
    store
        .create_profile(debug_launch_request())
        .expect("create profile");
    let mut resolver = baseline_resolver("workspace:web", "/workspace/web");
    let mut request = ExecutionContextRequest::debug_prep_seed(
        "debug.run.profile",
        TrustState::Trusted,
        "2026-05-13T18:05:02Z",
    );
    request.override_target_class = Some(TargetClass::ManagedWorkspace);
    request.override_working_directory = Some("/srv/web");
    let current = resolver.resolve(request);

    let preview = store
        .preview("launch:web-debug", &current, "2026-05-13T18:05:03Z")
        .expect("preview");
    assert_eq!(
        preview.state,
        LaunchProfilePreviewState::DriftRequiresReview
    );
    assert!(preview.requires_review_before_dispatch);
    assert!(preview.honesty_marker_present);
    assert!(preview.target_or_boundary_changed);
    assert!(preview
        .drift_rows
        .iter()
        .any(|row| row.field_path == "target_binding.target_class"));
    assert!(preview
        .drift_rows
        .iter()
        .any(|row| row.field_path == "target_binding.working_directory"));
}

#[test]
fn invalid_profile_preview_discloses_invalid_reason() {
    let mut store = LaunchProfileStore::new("workspace:web");
    let mut request = debug_launch_request();
    request.arguments.program = None;
    store.create_profile(request).expect("create profile");
    let mut resolver = baseline_resolver("workspace:web", "/workspace/web");
    let current = debug_context(&mut resolver, "2026-05-13T18:06:00Z");
    let preview = store
        .preview("launch:web-debug", &current, "2026-05-13T18:06:01Z")
        .expect("preview");
    assert_eq!(
        preview.state,
        LaunchProfilePreviewState::UnavailableInvalidConfig
    );
    assert_eq!(
        preview.invalid_reason,
        Some(LaunchProfileInvalidReason::LaunchMissingProgram)
    );
    assert!(preview.requires_review_before_dispatch);
}

#[test]
fn support_export_captures_revisions_previews_and_serializes_round_trip() {
    let mut store = LaunchProfileStore::new("workspace:web");
    store
        .create_profile(debug_launch_request())
        .expect("create profile");
    store
        .apply_edit(
            "launch:web-debug",
            LaunchProfileEditClass::RenamedDisplayName,
            "2026-05-13T18:07:00Z",
            |mutable| {
                *mutable.display_name = "Web debug (renamed)".to_owned();
            },
        )
        .expect("rename");
    let mut resolver = baseline_resolver("workspace:web", "/workspace/web");
    let current = debug_context(&mut resolver, "2026-05-13T18:07:02Z");
    let export = store.support_export("support:lp:01", "2026-05-13T18:07:03Z", |profile_id| {
        if profile_id == "launch:web-debug" {
            Some(current.clone())
        } else {
            None
        }
    });
    assert_eq!(
        export.record_kind,
        LAUNCH_PROFILE_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.workspace_id, "workspace:web");
    assert_eq!(export.profile_rows.len(), 1);
    let row = &export.profile_rows[0];
    assert_eq!(row.profile_id, "launch:web-debug");
    assert_eq!(row.revision_count, 2);
    assert_eq!(row.edit_lineage.len(), 2);
    assert!(row.latest_preview.is_some());

    let rendered = export.render_plaintext();
    assert!(rendered.contains("Web debug (renamed)"));
    assert!(rendered.contains("launch:web-debug"));
    assert!(rendered.contains(LaunchProfileEditClass::Created.as_str()));
    assert!(rendered.contains(LaunchProfileEditClass::RenamedDisplayName.as_str()));

    let serialized = serde_json::to_string(&export).expect("serializes");
    let parsed: LaunchProfileSupportExport =
        serde_json::from_str(&serialized).expect("deserializes");
    assert_eq!(parsed, export);
}

#[test]
fn attach_profile_missing_pid_is_invalid() {
    let mut store = LaunchProfileStore::new("workspace:web");
    let mut request = debug_launch_request();
    request.mode = LaunchProfileMode::Attach;
    request.arguments.program = None;
    request.arguments.attach_process_id = None;
    store.create_profile(request).expect("create profile");
    let profile = store.profile("launch:web-debug").expect("profile");
    assert_eq!(
        profile.invalid_reason(),
        Some(LaunchProfileInvalidReason::AttachMissingProcessId)
    );
}
