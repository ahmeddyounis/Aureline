//! Fixture-driven coverage for terminal target headers and restore posture.
//!
//! The cases under `fixtures/terminal/header_and_restore_alpha/*.json` pin the
//! first alpha consumer of the terminal header strip: bottom-panel terminal
//! tabs and restored rows expose target, cwd, runtime, and restore chips from
//! the same execution-context vocabulary used by task/test/debug entry rows.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContext, ExecutionContextRequest,
    ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
    TargetClass, TrustState,
};
use aureline_shell::terminal_pane::TerminalPaneSnapshot;
use aureline_terminal::{
    restore_session_as_transcript, HostClass, OpenSessionRequest, PtyHost,
    ScrollbackRedactionClass, TerminalHeaderRecord, TerminalScrollback,
};

#[derive(Debug, Clone, Deserialize)]
struct HeaderFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    driver: HeaderDriver,
    expect: HeaderExpect,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum HeaderDriver {
    LiveLocal,
    RemoteLostTransport,
    RestoredTranscript,
    EndedSession,
}

#[derive(Debug, Clone, Deserialize)]
struct HeaderExpect {
    source_kind_token: String,
    target_chip_display_value: String,
    target_chip_state_token: String,
    cwd_chip_display_value: String,
    cwd_chip_state_token: String,
    runtime_chip_value_token: String,
    runtime_chip_state_token: String,
    runtime_source_surface_token: String,
    runtime_source_target_class_token: String,
    restore_state_token: String,
    restore_chip_state_token: String,
    lifecycle_state_token: String,
    boundary_cue_visible: bool,
    auto_rerun_forbidden: bool,
    fresh_session_required: bool,
    #[serde(default)]
    open_fresh_session_command_id: Option<String>,
}

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

fn local_context(resolver: &mut ExecutionContextResolver) -> ExecutionContext {
    resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    ))
}

fn remote_context(resolver: &mut ExecutionContextResolver) -> ExecutionContext {
    let mut request = ExecutionContextRequest::local_terminal_seed(
        "terminal.open",
        TrustState::Trusted,
        "mono:0",
    );
    request.override_target_class = Some(TargetClass::SshRemote);
    request.override_working_directory = Some("/srv/code");
    resolver.resolve(request)
}

fn host_class_for(context: &ExecutionContext) -> HostClass {
    match context.target_identity.target_class {
        TargetClass::LocalHost => HostClass::HostDesktop,
        TargetClass::ContainerLocal | TargetClass::Devcontainer => HostClass::LocalContainer,
        _ => HostClass::RemoteAgentPrimary,
    }
}

fn open_context_session(host: &mut PtyHost, context: &ExecutionContext, title: &str) {
    host.open_session(OpenSessionRequest {
        workspace_id: &context.invocation_subject.workspace_id,
        host_class: host_class_for(context),
        display_title: title,
        cwd_hint: context.target_identity.working_directory.as_deref(),
        execution_context_ref: context.execution_context_id(),
        trust_state: context.policy_and_trust.trust_state,
        observed_at: "mono:0",
    });
}

fn header_for_fixture(driver: HeaderDriver) -> TerminalHeaderRecord {
    let mut resolver = baseline_resolver();
    match driver {
        HeaderDriver::LiveLocal => {
            let context = local_context(&mut resolver);
            let mut host = PtyHost::new();
            open_context_session(&mut host, &context, "zsh");
            let snapshot =
                TerminalPaneSnapshot::project("ws-test", &host).with_run_contexts([&context]);
            snapshot.tabs[0].header.clone()
        }
        HeaderDriver::RemoteLostTransport => {
            let context = remote_context(&mut resolver);
            let mut host = PtyHost::new();
            open_context_session(&mut host, &context, "agent shell");
            let id = host
                .sessions()
                .next()
                .expect("session exists")
                .session_id()
                .clone();
            host.mark_lost_transport(&id, "mono:1", Some("network_drop"))
                .expect("transport can drop");
            let snapshot =
                TerminalPaneSnapshot::project("ws-test", &host).with_run_contexts([&context]);
            snapshot.tabs[0].header.clone()
        }
        HeaderDriver::RestoredTranscript => {
            let context = local_context(&mut resolver);
            let mut host = PtyHost::new();
            open_context_session(&mut host, &context, "zsh");
            let id = host
                .sessions()
                .next()
                .expect("session exists")
                .session_id()
                .clone();
            host.close(&id, "mono:1", Some("user_closed"))
                .expect("session closes");
            let mut scrollback = TerminalScrollback::new(id.clone());
            scrollback.record_line(
                "$ git status",
                ScrollbackRedactionClass::SupportBundleScoped,
                "mono:0",
            );
            let prior = host.session(&id).expect("prior session exists");
            let restored = restore_session_as_transcript(prior, Some(&scrollback), "mono:restart");
            let snapshot = TerminalPaneSnapshot::project("ws-test", &host)
                .with_restored_terminals(vec![restored])
                .with_run_contexts([&context]);
            snapshot.restored_terminal_headers[0].clone()
        }
        HeaderDriver::EndedSession => {
            let context = local_context(&mut resolver);
            let mut host = PtyHost::new();
            open_context_session(&mut host, &context, "zsh");
            let id = host
                .sessions()
                .next()
                .expect("session exists")
                .session_id()
                .clone();
            host.close(&id, "mono:1", Some("user_closed"))
                .expect("session closes");
            let prior = host.session(&id).expect("prior session exists");
            let restored = restore_session_as_transcript(prior, None, "mono:restart");
            let snapshot = TerminalPaneSnapshot::project("ws-test", &host)
                .with_restored_terminals(vec![restored])
                .with_run_contexts([&context]);
            snapshot.restored_terminal_headers[0].clone()
        }
    }
}

fn assert_header_matches(path: &Path, fixture: &HeaderFixture, header: &TerminalHeaderRecord) {
    assert_eq!(fixture.record_kind, "terminal_header_alpha_case");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(
        header.source_kind_token, fixture.expect.source_kind_token,
        "source kind mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.target_chip.display_value, fixture.expect.target_chip_display_value,
        "target chip display mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.target_chip.state_token, fixture.expect.target_chip_state_token,
        "target chip state mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.cwd_chip.display_value, fixture.expect.cwd_chip_display_value,
        "cwd chip display mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.cwd_chip.state_token, fixture.expect.cwd_chip_state_token,
        "cwd chip state mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.runtime_chip.value_token, fixture.expect.runtime_chip_value_token,
        "runtime chip value mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.runtime_chip.state_token, fixture.expect.runtime_chip_state_token,
        "runtime chip state mismatch in {path:?} ({})",
        fixture.case_name
    );
    let runtime_source = header
        .runtime_source
        .as_ref()
        .expect("fixture drivers always attach runtime source");
    assert_eq!(
        runtime_source.surface_token, fixture.expect.runtime_source_surface_token,
        "runtime surface mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        runtime_source.target_class_token, fixture.expect.runtime_source_target_class_token,
        "runtime target class mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.restore_state_token, fixture.expect.restore_state_token,
        "restore state mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.restore_chip.state_token, fixture.expect.restore_chip_state_token,
        "restore chip state mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.lifecycle_state_token, fixture.expect.lifecycle_state_token,
        "lifecycle token mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.boundary_cue_visible, fixture.expect.boundary_cue_visible,
        "boundary cue mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.auto_rerun_forbidden, fixture.expect.auto_rerun_forbidden,
        "auto-rerun flag mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.fresh_session_required, fixture.expect.fresh_session_required,
        "fresh-session flag mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        header.open_fresh_session_command_id, fixture.expect.open_fresh_session_command_id,
        "fresh-session command mismatch in {path:?} ({})",
        fixture.case_name
    );
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/terminal/header_and_restore_alpha")
}

#[test]
fn terminal_header_alpha_fixtures_match_consumer_projection() {
    let dir = fixtures_dir();
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|err| panic!("header fixtures dir must exist at {dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "expected header fixtures under {dir:?}");

    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let fixture: HeaderFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        let header = header_for_fixture(fixture.driver);
        assert_header_matches(&path, &fixture, &header);
    }
}
