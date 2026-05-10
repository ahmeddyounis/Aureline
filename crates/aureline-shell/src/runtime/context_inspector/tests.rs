use std::path::Path;

use aureline_runtime::{
    ActorClass, CacheDisposition, CapsuleDriftState, ConfidenceLevel, DegradedFieldReason,
    DegradedFieldRecord, EnvironmentCapsuleRef, ExecutionContextRequest, ExecutionContextResolver,
    ExecutionContextResolverConfig, IdentityMode, ResolverInputField, ResolverInputSource,
    ScopeClass, SurfaceClass, TargetClass, ToolchainClass, TrustState,
    EXECUTION_CONTEXT_RECORD_KIND,
};

use serde::Deserialize;

use super::*;

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
fn terminal_seed_projection_quotes_resolver_truth_without_honesty_marker() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));

    let snapshot = ExecutionContextInspectorSnapshot::project(&context);
    assert_eq!(
        snapshot.record_kind,
        EXECUTION_CONTEXT_INSPECTOR_RECORD_KIND
    );
    assert_eq!(
        snapshot.schema_version,
        EXECUTION_CONTEXT_INSPECTOR_SCHEMA_VERSION
    );
    assert_eq!(snapshot.opening_surface, InspectorOpeningSurface::Terminal);
    assert_eq!(snapshot.execution_context_id, context.execution_context_id);
    assert_eq!(snapshot.workspace_id, "ws-test");
    assert!(!snapshot.honesty_marker_present);

    // Every section is rendered, even when it reduces to a single row, so a
    // green snapshot is never silently smaller than a degraded one.
    let section_ids: Vec<_> = snapshot
        .sections
        .iter()
        .map(|section| section.section_id)
        .collect();
    assert_eq!(
        section_ids,
        vec![
            InspectorSectionId::InvocationSubject,
            InspectorSectionId::TargetIdentity,
            InspectorSectionId::ToolchainIdentity,
            InspectorSectionId::EnvironmentCapsule,
            InspectorSectionId::PolicyAndTrust,
            InspectorSectionId::Scope,
            InspectorSectionId::Cache,
            InspectorSectionId::Provenance,
            InspectorSectionId::DegradedFields,
        ]
    );

    let target = snapshot
        .section(InspectorSectionId::TargetIdentity)
        .expect("target section");
    let target_class_row = target
        .rows
        .iter()
        .find(|row| row.row_id == "target_class")
        .expect("target_class row");
    assert_eq!(target_class_row.value, "Local desktop");
    assert_eq!(target_class_row.value_token.as_deref(), Some("local_host"));
    assert_eq!(
        target_class_row.winning_source,
        Some(ResolverInputSource::SurfaceRequested)
    );

    let cwd_row = target
        .rows
        .iter()
        .find(|row| row.row_id == "working_directory")
        .expect("working_directory row");
    assert_eq!(cwd_row.value, "/workspace");

    let toolchain = snapshot
        .section(InspectorSectionId::ToolchainIdentity)
        .expect("toolchain section");
    let toolchain_class_row = toolchain
        .rows
        .iter()
        .find(|row| row.row_id == "toolchain_class")
        .expect("toolchain_class row");
    assert_eq!(toolchain_class_row.value, "Login shell");
    assert_eq!(
        toolchain_class_row.value_token.as_deref(),
        Some("login_shell")
    );

    let degraded = snapshot
        .section(InspectorSectionId::DegradedFields)
        .expect("degraded section");
    assert_eq!(degraded.rows.len(), 1);
    assert!(degraded.rows[0].degraded_reason.is_none());
    assert_eq!(degraded.rows[0].row_id, "no_degraded_fields");

    // Stable actions are exposed without changing across surfaces.
    let actions: Vec<_> = snapshot.actions().map(|row| row.action).collect();
    assert_eq!(
        actions,
        vec![
            InspectorAction::CopyContext,
            InspectorAction::ViewResolverDetails,
            InspectorAction::OpenTargetSettings,
            InspectorAction::ReturnToInvokingSurface,
        ]
    );
}

#[test]
fn task_and_debug_seeds_share_the_same_inspector_section_layout() {
    let mut resolver = baseline_resolver();
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

    let task_snapshot = ExecutionContextInspectorSnapshot::project(&task);
    let debug_snapshot = ExecutionContextInspectorSnapshot::project(&debug);

    assert_eq!(task_snapshot.opening_surface, InspectorOpeningSurface::Task);
    assert_eq!(
        debug_snapshot.opening_surface,
        InspectorOpeningSurface::DebugPrep
    );

    let task_layout: Vec<_> = task_snapshot
        .sections
        .iter()
        .map(|s| s.section_id)
        .collect();
    let debug_layout: Vec<_> = debug_snapshot
        .sections
        .iter()
        .map(|s| s.section_id)
        .collect();
    assert_eq!(task_layout, debug_layout);

    // Toolchain class row differs per lane but the row addresses don't fork.
    let task_toolchain = task_snapshot
        .section(InspectorSectionId::ToolchainIdentity)
        .and_then(|section| {
            section
                .rows
                .iter()
                .find(|row| row.row_id == "toolchain_class")
        })
        .expect("toolchain_class row on task snapshot");
    assert_eq!(
        task_toolchain.value_token.as_deref(),
        Some("build_driver_runtime")
    );

    let debug_toolchain = debug_snapshot
        .section(InspectorSectionId::ToolchainIdentity)
        .and_then(|section| {
            section
                .rows
                .iter()
                .find(|row| row.row_id == "toolchain_class")
        })
        .expect("toolchain_class row on debug snapshot");
    assert_eq!(
        debug_toolchain.value_token.as_deref(),
        Some("debug_adapter_runtime")
    );
}

#[test]
fn explicit_override_lights_boundary_cue_and_records_conflicting_sources() {
    let mut resolver = baseline_resolver();
    let mut request = ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    );
    request.override_target_class = Some(TargetClass::SshRemote);
    request.override_working_directory = Some("/srv/code");
    request.requested_working_directory = Some("/workspace/active");

    let context = resolver.resolve(request);
    let snapshot = ExecutionContextInspectorSnapshot::project(&context);

    let target = snapshot
        .section(InspectorSectionId::TargetIdentity)
        .expect("target section");
    let target_class_row = target
        .rows
        .iter()
        .find(|row| row.row_id == "target_class")
        .expect("target_class row");
    assert_eq!(target_class_row.value, "Remote (SSH)");
    assert_eq!(
        target_class_row.winning_source,
        Some(ResolverInputSource::ExplicitOverride)
    );
    assert!(target_class_row
        .conflicting_sources
        .contains(&ResolverInputSource::SurfaceRequested));
    assert!(target_class_row
        .conflicting_sources
        .contains(&ResolverInputSource::WorkspaceDefault));

    let cue_row = target
        .rows
        .iter()
        .find(|row| row.row_id == "boundary_cue_visible")
        .expect("boundary cue row");
    assert!(cue_row.value.starts_with("Visible"));

    let cwd_row = target
        .rows
        .iter()
        .find(|row| row.row_id == "working_directory")
        .expect("cwd row");
    assert_eq!(cwd_row.value, "/srv/code");
    assert_eq!(
        cwd_row.winning_source,
        Some(ResolverInputSource::ExplicitOverride)
    );

    let provenance = snapshot
        .section(InspectorSectionId::Provenance)
        .expect("provenance section");
    let target_decision = provenance
        .rows
        .iter()
        .find(|row| row.row_id == "decision_target_class")
        .expect("target decision row");
    assert_eq!(target_decision.value, "ssh_remote");
    assert_eq!(
        target_decision.winning_source,
        Some(ResolverInputSource::ExplicitOverride)
    );
}

#[test]
fn pending_trust_lights_honesty_marker_and_keeps_the_inspector_addressable() {
    // Failure drill: inspect a partially resolved context. The resolver
    // records a degraded field for the unresolved trust posture; the
    // inspector MUST surface the marker rather than fabricating completeness.
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::PendingEvaluation,
        "mono:0",
    ));
    assert!(context.has_degraded_field());

    let snapshot = ExecutionContextInspectorSnapshot::project(&context);
    assert!(snapshot.honesty_marker_present);

    let degraded = snapshot
        .section(InspectorSectionId::DegradedFields)
        .expect("degraded section");
    assert_eq!(degraded.rows.len(), 1);
    let row = &degraded.rows[0];
    assert_eq!(row.label, "policy_and_trust.trust_state");
    assert_eq!(
        row.degraded_reason,
        Some(DegradedFieldReason::TrustStateUnresolved)
    );

    let policy = snapshot
        .section(InspectorSectionId::PolicyAndTrust)
        .expect("policy section");
    let trust_row = policy
        .rows
        .iter()
        .find(|row| row.row_id == "trust_state")
        .expect("trust_state row");
    assert_eq!(trust_row.value, "Pending evaluation");

    let provenance = snapshot
        .section(InspectorSectionId::Provenance)
        .expect("provenance section");
    let confidence_row = provenance
        .rows
        .iter()
        .find(|row| row.row_id == "confidence_level")
        .expect("confidence_level row");
    assert_eq!(confidence_row.value_token.as_deref(), Some("medium"));
}

#[test]
fn missing_working_directory_renders_honesty_marker_instead_of_blank_value() {
    // Failure drill: inspect a context that did not settle a working
    // directory. Surfaces with no caller, surface, or workspace input may
    // hand the resolver an empty record; the inspector MUST label the row
    // honestly instead of silently omitting it.
    let mut config = ExecutionContextResolverConfig {
        workspace_id: "ws-test".to_owned(),
        profile_id: None,
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: None,
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "caps:ws-test:seed".to_owned(),
            capsule_hash: "sha256:seed".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "seed-0".to_owned(),
    };
    config.workspace_default_working_directory = None;

    let mut resolver = ExecutionContextResolver::new(config);
    let context = resolver.resolve(ExecutionContextRequest {
        command_id: "terminal.open",
        surface: SurfaceClass::Terminal,
        actor_class: ActorClass::UserCommand,
        trust_state: TrustState::Trusted,
        observed_at: "mono:0",
        requested_target_class: Some(TargetClass::LocalHost),
        requested_working_directory: None,
        requested_toolchain_class: Some(ToolchainClass::LoginShell),
        override_target_class: None,
        override_working_directory: None,
        override_toolchain_class: None,
    });

    let snapshot = ExecutionContextInspectorSnapshot::project(&context);
    assert!(snapshot.honesty_marker_present);

    let target = snapshot
        .section(InspectorSectionId::TargetIdentity)
        .expect("target section");
    let cwd_row = target
        .rows
        .iter()
        .find(|row| row.row_id == "working_directory")
        .expect("cwd row");
    assert_eq!(
        cwd_row.missing_field_reason,
        Some(InspectorMissingFieldReason::ResolverUnsettled)
    );
    assert_eq!(cwd_row.value, "Not settled by resolver");
    assert!(cwd_row.value_token.is_none());

    let invocation = snapshot
        .section(InspectorSectionId::InvocationSubject)
        .expect("invocation section");
    let profile_row = invocation
        .rows
        .iter()
        .find(|row| row.row_id == "profile_id")
        .expect("profile_id row");
    assert_eq!(
        profile_row.missing_field_reason,
        Some(InspectorMissingFieldReason::ResolverUnsettled)
    );
}

#[test]
fn render_plaintext_includes_section_headings_and_honesty_markers() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::PendingEvaluation,
        "mono:0",
    ));
    let snapshot = ExecutionContextInspectorSnapshot::project(&context);

    let block = snapshot.render_plaintext();
    assert!(block.contains("Execution context inspector"));
    assert!(block.contains("Opened from: terminal"));
    assert!(block.contains("[Target]"));
    assert!(block.contains("[Honesty markers]"));
    assert!(block.contains("trust_state_unresolved"));
    assert!(block.contains("Working directory: /workspace"));
}

#[test]
fn opened_from_support_flow_keeps_record_shape_stable() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::task_seed(
        "task.run.cargo_build",
        TrustState::Trusted,
        "mono:1",
    ));
    let snapshot = ExecutionContextInspectorSnapshot::project_from_surface(
        &context,
        InspectorOpeningSurface::SupportFlow,
    );
    assert_eq!(
        snapshot.opening_surface,
        InspectorOpeningSurface::SupportFlow
    );
    // The task lane's record shape must not change just because a support
    // flow opened the inspector instead of the task channel.
    assert_eq!(snapshot.execution_context_id, context.execution_context_id);
    assert_eq!(snapshot.sections.len(), 9);
}

#[test]
fn additional_degraded_field_records_appear_in_their_own_rows() {
    // Construct a synthetic context that carries an extra degraded field so
    // the inspector projection covers the multi-field path. We borrow the
    // resolver's nominal output and append a second degraded record to it.
    let mut resolver = baseline_resolver();
    let mut context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::PendingEvaluation,
        "mono:0",
    ));
    context.degraded_fields.push(DegradedFieldRecord {
        field_path: "environment_capsule_ref.drift_state".to_owned(),
        reason: DegradedFieldReason::CapsuleDriftDetected,
        repair_hook_ref: None,
    });

    let snapshot = ExecutionContextInspectorSnapshot::project(&context);
    let degraded = snapshot
        .section(InspectorSectionId::DegradedFields)
        .expect("degraded section");
    assert_eq!(degraded.rows.len(), 2);
    assert_eq!(
        degraded.rows[0].degraded_reason,
        Some(DegradedFieldReason::TrustStateUnresolved)
    );
    assert_eq!(
        degraded.rows[1].degraded_reason,
        Some(DegradedFieldReason::CapsuleDriftDetected)
    );
}

#[test]
fn cache_disposition_label_is_explicit_about_no_reuse() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));
    assert_eq!(context.cache_disposition, CacheDisposition::Cold);
    let snapshot = ExecutionContextInspectorSnapshot::project(&context);
    let cache = snapshot
        .section(InspectorSectionId::Cache)
        .expect("cache section");
    let row = cache
        .rows
        .iter()
        .find(|row| row.row_id == "cache_disposition")
        .expect("cache row");
    assert_eq!(row.value, "Cold (no reuse)");
}

#[test]
fn fixture_failure_drill_replays_into_the_inspector_projection() {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/runtime/context_inspector_cases/conflicting_inputs_remote_target.json",
    );
    let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
    let fixture: ConflictingInputsCase =
        serde_json::from_str(&payload).expect("fixture must parse");

    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest {
        command_id: &fixture.input.command_id,
        surface: fixture.input.surface,
        actor_class: fixture.input.actor_class,
        trust_state: fixture.input.trust_state,
        observed_at: &fixture.input.observed_at,
        requested_target_class: fixture.input.requested_target_class,
        requested_working_directory: fixture.input.requested_working_directory.as_deref(),
        requested_toolchain_class: fixture.input.requested_toolchain_class,
        override_target_class: fixture.input.override_target_class,
        override_working_directory: fixture.input.override_working_directory.as_deref(),
        override_toolchain_class: fixture.input.override_toolchain_class,
    });

    let snapshot = ExecutionContextInspectorSnapshot::project(&context);
    assert_eq!(snapshot.record_kind, fixture.expect.snapshot_record_kind);
    assert_eq!(snapshot.opening_surface, fixture.expect.opening_surface);
    assert_eq!(
        snapshot.honesty_marker_present,
        fixture.expect.honesty_marker_present
    );

    let target = snapshot
        .section(InspectorSectionId::TargetIdentity)
        .expect("target section");
    let target_row = target
        .rows
        .iter()
        .find(|row| row.row_id == "target_class")
        .expect("target_class row");
    assert_eq!(
        target_row.value_token.as_deref(),
        Some(fixture.expect.target_class.as_str())
    );
    assert_eq!(
        target_row.winning_source,
        Some(fixture.expect.target_winning_source)
    );
    for source in &fixture.expect.target_conflicting_sources {
        assert!(target_row.conflicting_sources.contains(source));
    }

    let cwd_row = target
        .rows
        .iter()
        .find(|row| row.row_id == "working_directory")
        .expect("cwd row");
    assert_eq!(cwd_row.value, fixture.expect.working_directory);
    assert_eq!(
        cwd_row.winning_source,
        Some(fixture.expect.working_directory_winning_source)
    );

    // The execution-context record kind must be preserved verbatim by the
    // resolver so support exports can correlate the inspector snapshot back
    // to the underlying context object.
    assert_eq!(context.record_kind, EXECUTION_CONTEXT_RECORD_KIND);
}

#[test]
fn fixture_partially_resolved_replays_honesty_markers_into_inspector() {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/context_inspector_cases/partially_resolved_terminal.json");
    let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
    let fixture: PartiallyResolvedCase =
        serde_json::from_str(&payload).expect("fixture must parse");

    let config = ExecutionContextResolverConfig {
        workspace_id: "ws-test".to_owned(),
        profile_id: None,
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: None,
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: "caps:ws-test:seed".to_owned(),
            capsule_hash: "sha256:seed".to_owned(),
            resolved_schema_version: "1".to_owned(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "seed-0".to_owned(),
    };
    let mut resolver = ExecutionContextResolver::new(config);

    let context = resolver.resolve(ExecutionContextRequest {
        command_id: &fixture.input.command_id,
        surface: fixture.input.surface,
        actor_class: fixture.input.actor_class,
        trust_state: fixture.input.trust_state,
        observed_at: &fixture.input.observed_at,
        requested_target_class: fixture.input.requested_target_class,
        requested_working_directory: fixture.input.requested_working_directory.as_deref(),
        requested_toolchain_class: fixture.input.requested_toolchain_class,
        override_target_class: fixture.input.override_target_class,
        override_working_directory: fixture.input.override_working_directory.as_deref(),
        override_toolchain_class: fixture.input.override_toolchain_class,
    });

    let snapshot = ExecutionContextInspectorSnapshot::project(&context);
    assert!(snapshot.honesty_marker_present);

    let target = snapshot
        .section(InspectorSectionId::TargetIdentity)
        .expect("target section");
    let cwd_row = target
        .rows
        .iter()
        .find(|row| row.row_id == "working_directory")
        .expect("cwd row");
    assert_eq!(
        cwd_row.missing_field_reason.map(|r| r.as_str()),
        Some(fixture.expect.working_directory_missing_reason.as_str())
    );

    let invocation = snapshot
        .section(InspectorSectionId::InvocationSubject)
        .expect("invocation section");
    let profile_row = invocation
        .rows
        .iter()
        .find(|row| row.row_id == "profile_id")
        .expect("profile_id row");
    assert_eq!(
        profile_row.missing_field_reason.map(|r| r.as_str()),
        Some(fixture.expect.profile_missing_reason.as_str())
    );

    if let Some(reason) = fixture.expect.degraded_reason {
        let degraded = snapshot
            .section(InspectorSectionId::DegradedFields)
            .expect("degraded section");
        assert!(degraded
            .rows
            .iter()
            .any(|row| row.degraded_reason.map(|r| r.as_str()) == Some(reason.as_str())));
    }
}

#[derive(Debug, Deserialize)]
struct ConflictingInputsCase {
    input: FixtureInput,
    expect: ConflictingInputsExpect,
}

#[derive(Debug, Deserialize)]
struct ConflictingInputsExpect {
    snapshot_record_kind: String,
    opening_surface: InspectorOpeningSurface,
    honesty_marker_present: bool,
    target_class: TargetClass,
    target_winning_source: ResolverInputSource,
    target_conflicting_sources: Vec<ResolverInputSource>,
    working_directory: String,
    working_directory_winning_source: ResolverInputSource,
}

#[derive(Debug, Deserialize)]
struct PartiallyResolvedCase {
    input: FixtureInput,
    expect: PartiallyResolvedExpect,
}

#[derive(Debug, Deserialize)]
struct PartiallyResolvedExpect {
    working_directory_missing_reason: InspectorMissingFieldReason,
    profile_missing_reason: InspectorMissingFieldReason,
    #[serde(default)]
    degraded_reason: Option<DegradedFieldReason>,
}

#[derive(Debug, Deserialize)]
struct FixtureInput {
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

#[test]
fn decision_for_field_returns_the_recorded_decision() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));
    let decision = decision_for(&context.provenance, ResolverInputField::ToolchainClass)
        .expect("toolchain decision recorded");
    assert_eq!(decision.field, ResolverInputField::ToolchainClass);
    assert_eq!(decision.resolved_value_token, "login_shell");
}

#[test]
fn surface_class_label_covers_seed_lanes_with_human_readable_strings() {
    assert_eq!(surface_class_label(SurfaceClass::Terminal), "Terminal");
    assert_eq!(surface_class_label(SurfaceClass::Task), "Task");
    assert_eq!(surface_class_label(SurfaceClass::Debug), "Debug");
}

#[test]
fn confidence_level_label_token_is_high_for_trusted() {
    let mut resolver = baseline_resolver();
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ));
    assert_eq!(context.provenance.confidence_level, ConfidenceLevel::High);
}
