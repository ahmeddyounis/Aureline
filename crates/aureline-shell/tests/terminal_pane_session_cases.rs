//! Fixture-driven coverage for the M1 terminal-pane projection.
//!
//! Each case under `fixtures/terminal/session_cases/*.json` exercises one row
//! of the seed contract: nominal active local session, the failure drill
//! (transport drop preserves provenance), reconnect-against-same-identity,
//! supervisor quarantine, and a restricted-trust opener. The tests drive the
//! canonical PTY host and the [`TerminalPaneSnapshot`] projection so the
//! header vocabulary cannot drift between the host, the pane, and the
//! fixture corpus.

use std::path::Path;

use serde::Deserialize;

use aureline_shell::terminal_pane::TerminalPaneSnapshot;
use aureline_terminal::{
    HostClass, OpenSessionRequest, PtyHost, PtySessionId, SessionLifecycleState, TerminalTrustState,
};

#[derive(Debug, Clone, Deserialize)]
struct TerminalPaneFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    #[allow(dead_code)]
    workspace_id: String,
    active_workspace_id: String,
    steps: Vec<FixtureStep>,
    expect: ExpectBlock,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
enum FixtureStep {
    OpenSession {
        host_class: String,
        display_title: String,
        #[serde(default)]
        cwd_hint: Option<String>,
        execution_context_ref: String,
        trust_state: String,
        observed_at: String,
    },
    MarkStarting {
        session_index: usize,
        observed_at: String,
    },
    MarkActive {
        session_index: usize,
        observed_at: String,
    },
    MarkLostTransport {
        session_index: usize,
        observed_at: String,
        #[serde(default)]
        reason_code: Option<String>,
    },
    MarkReconnectedSameIdentity {
        session_index: usize,
        observed_at: String,
    },
    Quarantine {
        session_index: usize,
        observed_at: String,
        reason_code: String,
    },
    Close {
        session_index: usize,
        observed_at: String,
        #[serde(default)]
        reason_code: Option<String>,
    },
    UpdateCwdHint {
        session_index: usize,
        #[serde(default)]
        cwd_hint: Option<String>,
        observed_at: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectBlock {
    tabs: Vec<ExpectedTab>,
    #[serde(default)]
    active_tab_id: Option<String>,
    #[serde(default)]
    required_transitions: Vec<ExpectedTransition>,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectedTab {
    session_id: String,
    host_class: String,
    target_badge: String,
    boundary_cue_token: String,
    boundary_cue_visible: bool,
    display_title: String,
    #[serde(default)]
    cwd_hint: Option<String>,
    execution_context_ref: String,
    trust_state_token: String,
    lifecycle_state_token: String,
    is_interactive: bool,
    #[serde(default)]
    degraded_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectedTransition {
    from: String,
    to: String,
}

fn host_class_from(token: &str) -> HostClass {
    match token {
        "host_desktop" => HostClass::HostDesktop,
        "remote_agent_primary" => HostClass::RemoteAgentPrimary,
        "local_container" => HostClass::LocalContainer,
        other => panic!("unsupported host_class token: {other}"),
    }
}

fn trust_state_from(token: &str) -> TerminalTrustState {
    match token {
        "trusted" => TerminalTrustState::Trusted,
        "restricted" => TerminalTrustState::Restricted,
        "pending_evaluation" => TerminalTrustState::PendingEvaluation,
        other => panic!("unsupported trust_state token: {other}"),
    }
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/terminal/session_cases")
}

fn run_fixture(path: &Path, fixture: &TerminalPaneFixture) {
    assert_eq!(
        fixture.record_kind, "terminal_pane_session_case",
        "unexpected record_kind in {path:?}"
    );
    assert_eq!(
        fixture.schema_version, 1,
        "unexpected schema_version in {path:?}"
    );

    let mut host = PtyHost::new();
    let mut sessions: Vec<PtySessionId> = Vec::new();
    let mut transitions: Vec<(SessionLifecycleState, SessionLifecycleState)> = Vec::new();

    for step in &fixture.steps {
        match step {
            FixtureStep::OpenSession {
                host_class,
                display_title,
                cwd_hint,
                execution_context_ref,
                trust_state,
                observed_at,
            } => {
                let id = host.open_session(OpenSessionRequest {
                    workspace_id: &fixture.workspace_id,
                    host_class: host_class_from(host_class),
                    display_title,
                    cwd_hint: cwd_hint.as_deref(),
                    execution_context_ref,
                    trust_state: trust_state_from(trust_state),
                    observed_at,
                });
                sessions.push(id);
            }
            FixtureStep::MarkStarting {
                session_index,
                observed_at,
            } => {
                host.mark_starting(&sessions[*session_index], observed_at)
                    .expect("mark_starting must succeed");
            }
            FixtureStep::MarkActive {
                session_index,
                observed_at,
            } => {
                host.mark_active(&sessions[*session_index], observed_at)
                    .expect("mark_active must succeed");
            }
            FixtureStep::MarkLostTransport {
                session_index,
                observed_at,
                reason_code,
            } => {
                host.mark_lost_transport(
                    &sessions[*session_index],
                    observed_at,
                    reason_code.as_deref(),
                )
                .expect("mark_lost_transport must succeed");
            }
            FixtureStep::MarkReconnectedSameIdentity {
                session_index,
                observed_at,
            } => {
                host.mark_reconnected_same_identity(&sessions[*session_index], observed_at)
                    .expect("mark_reconnected_same_identity must succeed");
            }
            FixtureStep::Quarantine {
                session_index,
                observed_at,
                reason_code,
            } => {
                host.quarantine(&sessions[*session_index], observed_at, reason_code)
                    .expect("quarantine must succeed");
            }
            FixtureStep::Close {
                session_index,
                observed_at,
                reason_code,
            } => {
                host.close(
                    &sessions[*session_index],
                    observed_at,
                    reason_code.as_deref(),
                )
                .expect("close must succeed");
            }
            FixtureStep::UpdateCwdHint {
                session_index,
                cwd_hint,
                observed_at,
            } => {
                host.update_cwd_hint(&sessions[*session_index], cwd_hint.as_deref(), observed_at)
                    .expect("update_cwd_hint must succeed");
            }
        }
        for frame in host.drain_transitions() {
            transitions.push((frame.from_state, frame.to_state));
        }
    }

    let snapshot = TerminalPaneSnapshot::project(&fixture.active_workspace_id, &host);
    assert_eq!(
        snapshot.tabs.len(),
        fixture.expect.tabs.len(),
        "tab count mismatch in {path:?} ({})",
        fixture.case_name
    );

    for (actual, expected) in snapshot.tabs.iter().zip(fixture.expect.tabs.iter()) {
        assert_eq!(
            actual.session_id.as_str(),
            expected.session_id,
            "session_id mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            actual.host_class.as_str(),
            expected.host_class,
            "host_class mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            actual.target_badge, expected.target_badge,
            "target_badge mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            actual.boundary_cue_token, expected.boundary_cue_token,
            "boundary_cue_token mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            actual.boundary_cue_visible, expected.boundary_cue_visible,
            "boundary_cue_visible mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            actual.display_title, expected.display_title,
            "display_title mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            actual.cwd_hint, expected.cwd_hint,
            "cwd_hint mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            actual.execution_context_ref, expected.execution_context_ref,
            "execution_context_ref mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            actual.trust_state_token, expected.trust_state_token,
            "trust_state_token mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            actual.lifecycle_state_token, expected.lifecycle_state_token,
            "lifecycle_state_token mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            actual.is_interactive, expected.is_interactive,
            "is_interactive mismatch in {path:?} ({})",
            fixture.case_name
        );
        assert_eq!(
            actual.degraded_token, expected.degraded_token,
            "degraded_token mismatch in {path:?} ({})",
            fixture.case_name
        );
    }

    if let Some(expected_active) = fixture.expect.active_tab_id.as_deref() {
        let actual_active = snapshot
            .active_tab_id
            .as_ref()
            .map(|id| id.as_str().to_owned());
        assert_eq!(
            actual_active.as_deref(),
            Some(expected_active),
            "active_tab_id mismatch in {path:?} ({})",
            fixture.case_name
        );
    }

    for required in &fixture.expect.required_transitions {
        let from = lifecycle_from(&required.from);
        let to = lifecycle_from(&required.to);
        assert!(
            transitions.contains(&(from, to)),
            "missing transition {:?} -> {:?} in {path:?} ({})",
            required.from,
            required.to,
            fixture.case_name
        );
    }
}

fn lifecycle_from(token: &str) -> SessionLifecycleState {
    match token {
        "session_requested" => SessionLifecycleState::Requested,
        "session_starting" => SessionLifecycleState::Starting,
        "session_active" => SessionLifecycleState::Active,
        "session_lost_transport" => SessionLifecycleState::LostTransport,
        "session_reconnected_same_identity" => SessionLifecycleState::ReconnectedSameIdentity,
        "session_closed" => SessionLifecycleState::Closed,
        "session_quarantined" => SessionLifecycleState::Quarantined,
        other => panic!("unsupported lifecycle token: {other}"),
    }
}

#[test]
fn terminal_pane_session_case_fixtures_match_expectations() {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .unwrap_or_else(|err| panic!("session_cases dir must exist at {dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    assert!(
        !paths.is_empty(),
        "expected at least one terminal session-case fixture under {dir:?}"
    );

    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let fixture: TerminalPaneFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        run_fixture(&path, &fixture);
    }
}
