use std::path::{Path, PathBuf};

use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContext, ExecutionContextRequest,
    ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode,
    LaunchProfileAdapterBinding, LaunchProfileArguments, LaunchProfileCreateRequest,
    LaunchProfileEditClass, LaunchProfileEnvironmentBinding, LaunchProfileKind, LaunchProfileMode,
    LaunchProfilePreviewState, LaunchProfileSideEffectClass, LaunchProfileStore,
    LaunchProfileSupportExport, LaunchProfileTargetBinding, ScopeClass, TargetClass, TrustState,
    LAUNCH_PROFILE_SUPPORT_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    workspace_id: String,
    profile_id: String,
    create_request: FixtureCreate,
    edits: Vec<FixtureEdit>,
    preview_context: FixturePreviewContext,
    expect: FixtureExpect,
}

#[derive(Debug, Deserialize)]
struct FixtureCreate {
    display_name: String,
    mode: String,
    kind: String,
    target: FixtureTarget,
    adapter: Option<FixtureAdapter>,
    environment: FixtureEnvironment,
    arguments: FixtureArguments,
    declared_side_effects: Vec<String>,
    observed_at: String,
}

#[derive(Debug, Deserialize)]
struct FixtureTarget {
    canonical_target_id: String,
    target_class_token: String,
    target_label: String,
    working_directory: Option<String>,
    scope_class_token: String,
}

#[derive(Debug, Deserialize)]
struct FixtureAdapter {
    adapter_id: String,
    adapter_label: String,
    transport_class_token: String,
    requested_dap_protocol_version: String,
    #[serde(default)]
    required_capability_tokens: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureEnvironment {
    capsule_id: String,
    capsule_hash: String,
    #[serde(default)]
    declared_overlay_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureArguments {
    program: Option<String>,
    #[serde(default)]
    args: Vec<String>,
    working_directory: Option<String>,
    attach_process_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum FixtureEdit {
    Rename {
        new_display_name: String,
        observed_at: String,
    },
    RollbackToFirst {
        observed_at: String,
    },
}

#[derive(Debug, Deserialize)]
struct FixturePreviewContext {
    kind: String,
    override_target_class: Option<String>,
    override_working_directory: Option<String>,
    observed_at: String,
}

#[derive(Debug, Deserialize)]
struct FixtureExpect {
    preview_state: String,
    requires_review_before_dispatch: bool,
    honesty_marker_present: bool,
    current_target_reachable: bool,
    drift_field_paths: Vec<String>,
    revision_count: usize,
    side_effect_disclosure_tokens: Vec<String>,
    invalid_reason_token: Option<String>,
    #[serde(default)]
    final_display_name: Option<String>,
    captured_at: String,
}

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/launch_profiles_beta")
        .join(name)
}

fn load_fixture(name: &str) -> Fixture {
    let path = fixture_path(name);
    let body = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn parse_mode(token: &str) -> LaunchProfileMode {
    match token {
        "launch" => LaunchProfileMode::Launch,
        "attach" => LaunchProfileMode::Attach,
        other => panic!("unknown mode token {other}"),
    }
}

fn parse_kind(token: &str) -> LaunchProfileKind {
    match token {
        "task" => LaunchProfileKind::Task,
        "test" => LaunchProfileKind::Test,
        "debug" => LaunchProfileKind::Debug,
        other => panic!("unknown kind token {other}"),
    }
}

fn parse_side_effect(token: &str) -> LaunchProfileSideEffectClass {
    match token {
        "target_process_spawn" => LaunchProfileSideEffectClass::TargetProcessSpawn,
        "target_process_attach" => LaunchProfileSideEffectClass::TargetProcessAttach,
        "workspace_filesystem_write" => LaunchProfileSideEffectClass::WorkspaceFilesystemWrite,
        "outbound_network" => LaunchProfileSideEffectClass::OutboundNetwork,
        "inbound_network_listener" => LaunchProfileSideEffectClass::InboundNetworkListener,
        "process_env_mutation" => LaunchProfileSideEffectClass::ProcessEnvMutation,
        "remote_host_handoff" => LaunchProfileSideEffectClass::RemoteHostHandoff,
        other => panic!("unknown side-effect token {other}"),
    }
}

fn parse_target_class(token: &str) -> TargetClass {
    match token {
        "local_host" => TargetClass::LocalHost,
        "ssh_remote" => TargetClass::SshRemote,
        "container_local" => TargetClass::ContainerLocal,
        "devcontainer" => TargetClass::Devcontainer,
        "remote_workspace_vm" => TargetClass::RemoteWorkspaceVm,
        "prebuild_runtime" => TargetClass::PrebuildRuntime,
        "managed_workspace" => TargetClass::ManagedWorkspace,
        "notebook_kernel_local" => TargetClass::NotebookKernelLocal,
        "notebook_kernel_remote" => TargetClass::NotebookKernelRemote,
        "ai_sandbox" => TargetClass::AiSandbox,
        other => panic!("unknown target-class token {other}"),
    }
}

fn build_create_request(fixture: &Fixture) -> LaunchProfileCreateRequest {
    let create = &fixture.create_request;
    LaunchProfileCreateRequest {
        profile_id: fixture.profile_id.clone(),
        workspace_id: fixture.workspace_id.clone(),
        display_name: create.display_name.clone(),
        mode: parse_mode(&create.mode),
        kind: parse_kind(&create.kind),
        target: LaunchProfileTargetBinding {
            canonical_target_id: create.target.canonical_target_id.clone(),
            target_class_token: create.target.target_class_token.clone(),
            target_label: create.target.target_label.clone(),
            working_directory: create.target.working_directory.clone(),
            scope_class_token: create.target.scope_class_token.clone(),
        },
        adapter: create
            .adapter
            .as_ref()
            .map(|a| LaunchProfileAdapterBinding {
                adapter_id: a.adapter_id.clone(),
                adapter_label: a.adapter_label.clone(),
                transport_class_token: a.transport_class_token.clone(),
                requested_dap_protocol_version: a.requested_dap_protocol_version.clone(),
                required_capability_tokens: a.required_capability_tokens.clone(),
            }),
        environment: LaunchProfileEnvironmentBinding {
            capsule_id: create.environment.capsule_id.clone(),
            capsule_hash: create.environment.capsule_hash.clone(),
            declared_overlay_keys: create.environment.declared_overlay_keys.clone(),
        },
        arguments: LaunchProfileArguments {
            program: create.arguments.program.clone(),
            args: create.arguments.args.clone(),
            working_directory: create.arguments.working_directory.clone(),
            attach_process_id: create.arguments.attach_process_id,
        },
        declared_side_effects: create
            .declared_side_effects
            .iter()
            .map(|s| parse_side_effect(s))
            .collect(),
        observed_at: create.observed_at.clone(),
    }
}

fn fixture_resolver(workspace_id: &str) -> ExecutionContextResolver {
    ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: workspace_id.to_owned(),
        profile_id: Some("profile:default".to_owned()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/workspace/web".to_owned()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: format!("caps:{workspace_id}"),
            capsule_hash: format!("sha256:{workspace_id}"),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "launch-profile-fixture-resolver".to_owned(),
    })
}

fn resolve_preview_context(fixture: &Fixture) -> ExecutionContext {
    let mut resolver = fixture_resolver(&fixture.workspace_id);
    let context = &fixture.preview_context;
    let trust = TrustState::Trusted;
    let mut request = match context.kind.as_str() {
        "task" => {
            ExecutionContextRequest::task_seed("task.run.profile", trust, &context.observed_at)
        }
        "test" => {
            ExecutionContextRequest::test_seed("test.run.profile", trust, &context.observed_at)
        }
        "debug" => ExecutionContextRequest::debug_prep_seed(
            "debug.run.profile",
            trust,
            &context.observed_at,
        ),
        other => panic!("unknown preview-context kind {other}"),
    };
    if let Some(token) = &context.override_target_class {
        request.override_target_class = Some(parse_target_class(token));
    }
    if let Some(dir) = &context.override_working_directory {
        request.override_working_directory = Some(dir);
    }
    resolver.resolve(request)
}

fn apply_edits(store: &mut LaunchProfileStore, fixture: &Fixture) {
    let revisions_at_start = store
        .revisions(&fixture.profile_id)
        .expect("revisions")
        .to_vec();
    let first_revision_id = revisions_at_start[0].revision_id.clone();
    for edit in &fixture.edits {
        match edit {
            FixtureEdit::Rename {
                new_display_name,
                observed_at,
            } => {
                store
                    .apply_edit(
                        &fixture.profile_id,
                        LaunchProfileEditClass::RenamedDisplayName,
                        observed_at,
                        |mutable| {
                            *mutable.display_name = new_display_name.clone();
                        },
                    )
                    .expect("rename edit");
            }
            FixtureEdit::RollbackToFirst { observed_at } => {
                store
                    .rollback_to(&fixture.profile_id, &first_revision_id, observed_at)
                    .expect("rollback edit");
            }
        }
    }
}

fn build_support_export(
    store: &LaunchProfileStore,
    fixture: &Fixture,
    current: ExecutionContext,
) -> LaunchProfileSupportExport {
    let target_id = fixture.profile_id.clone();
    store.support_export("support:lp:fixture", &fixture.expect.captured_at, |id| {
        if id == target_id {
            Some(current.clone())
        } else {
            None
        }
    })
}

fn run_fixture(name: &str) -> (LaunchProfileSupportExport, Fixture) {
    let fixture = load_fixture(name);
    let mut store = LaunchProfileStore::new(&fixture.workspace_id);
    store
        .create_profile(build_create_request(&fixture))
        .expect("create profile");
    apply_edits(&mut store, &fixture);
    let current = resolve_preview_context(&fixture);
    let export = build_support_export(&store, &fixture, current);
    (export, fixture)
}

fn check_export(export: &LaunchProfileSupportExport, fixture: &Fixture) {
    assert_eq!(
        export.record_kind,
        LAUNCH_PROFILE_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.workspace_id, fixture.workspace_id);
    assert_eq!(export.profile_rows.len(), 1);
    let row = &export.profile_rows[0];
    assert_eq!(row.profile_id, fixture.profile_id);
    assert_eq!(row.revision_count, fixture.expect.revision_count);
    if let Some(name) = &fixture.expect.final_display_name {
        assert_eq!(row.display_name, *name);
    }
    let preview = row.latest_preview.as_ref().expect("preview present");
    assert_eq!(preview.state_token, fixture.expect.preview_state);
    let expected_state = match fixture.expect.preview_state.as_str() {
        "ready_to_dispatch" => LaunchProfilePreviewState::ReadyToDispatch,
        "drift_requires_review" => LaunchProfilePreviewState::DriftRequiresReview,
        "target_unreachable" => LaunchProfilePreviewState::TargetUnreachable,
        "unavailable_invalid_config" => LaunchProfilePreviewState::UnavailableInvalidConfig,
        other => panic!("unknown preview state {other}"),
    };
    assert_eq!(preview.state, expected_state);
    assert_eq!(
        preview.requires_review_before_dispatch,
        fixture.expect.requires_review_before_dispatch
    );
    assert_eq!(
        preview.honesty_marker_present,
        fixture.expect.honesty_marker_present
    );
    assert_eq!(
        preview.current_target_reachable,
        fixture.expect.current_target_reachable
    );
    assert_eq!(
        preview.invalid_reason_token,
        fixture.expect.invalid_reason_token
    );
    assert_eq!(
        preview.side_effect_disclosure_tokens,
        fixture.expect.side_effect_disclosure_tokens
    );
    for path in &fixture.expect.drift_field_paths {
        assert!(
            preview.drift_rows.iter().any(|row| row.field_path == *path),
            "missing drift field {path}; got {:?}",
            preview
                .drift_rows
                .iter()
                .map(|r| r.field_path.clone())
                .collect::<Vec<_>>()
        );
    }
    assert_eq!(
        export.honesty_marker_present,
        fixture.expect.honesty_marker_present
    );
}

fn round_trip(export: &LaunchProfileSupportExport) {
    let serialized = serde_json::to_string(export).expect("export serializes");
    let parsed: LaunchProfileSupportExport =
        serde_json::from_str(&serialized).expect("export deserializes");
    assert_eq!(&parsed, export);
}

#[test]
fn protected_walk_local_is_ready_to_dispatch() {
    let (export, fixture) = run_fixture("protected_walk_local.json");
    check_export(&export, &fixture);
    let row = &export.profile_rows[0];
    let preview = row.latest_preview.as_ref().expect("preview");
    assert_eq!(preview.state, LaunchProfilePreviewState::ReadyToDispatch);
    assert!(preview.drift_rows.is_empty());
    round_trip(&export);
}

#[test]
fn edit_and_rollback_retains_lineage_and_restores_display_name() {
    let (export, fixture) = run_fixture("edit_and_rollback.json");
    check_export(&export, &fixture);
    let row = &export.profile_rows[0];
    assert_eq!(row.edit_lineage.len(), 3);
    let lineage_tokens: Vec<&str> = row
        .edit_lineage
        .iter()
        .map(|edit| edit.edit_class_token.as_str())
        .collect();
    assert_eq!(
        lineage_tokens,
        vec!["created", "renamed_display_name", "rolled_back"]
    );
    let last_edit = row.edit_lineage.last().expect("last edit");
    assert!(last_edit.rollback_target_revision_id.is_some());
    assert_eq!(row.display_name, "Web debug");
    round_trip(&export);
}

#[test]
fn current_context_target_drift_requires_review() {
    let (export, fixture) = run_fixture("current_context_target_drift.json");
    check_export(&export, &fixture);
    let row = &export.profile_rows[0];
    let preview = row.latest_preview.as_ref().expect("preview");
    assert_eq!(
        preview.state,
        LaunchProfilePreviewState::DriftRequiresReview
    );
    assert!(preview.target_or_boundary_changed);
    assert!(preview.honesty_marker_present);
    assert!(preview.requires_review_before_dispatch);
    round_trip(&export);
}

#[test]
fn launch_profile_schema_published_with_module() {
    // Sanity-check that the schema doc is in place. The shell consumer test
    // covers the runtime / UI agreement; this guards against the schema or
    // doc being deleted without updating the lane.
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas/runtime/launch_profile.schema.json");
    assert!(
        schema_path.exists(),
        "launch profile schema missing at {schema_path:?}"
    );
    let doc_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/runtime/m3/run_debug_profiles_beta.md");
    assert!(doc_path.exists(), "reviewer doc missing at {doc_path:?}");
}
