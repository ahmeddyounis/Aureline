//! PTY host abstraction and session-header truth.
//!
//! The PTY host is the single owner of terminal-session lifecycle and
//! provenance. Higher surfaces — the shell's terminal pane, the activity
//! center, the title-context bar, support export — never mint their own
//! session ids, headers, or lifecycle vocabulary; they project the records
//! defined here.
//!
//! ## Why one host abstraction
//!
//! Embedding shell launches directly in the UI thread forks session truth
//! across surfaces and produces anonymous tabs the moment a shell exits. The
//! host owns:
//!
//! - **stable identity.** Every session carries a [`PtySessionId`] derived
//!   from `(workspace_id, host_class, sequence)`; the id survives termination,
//!   restart, transport loss, and quarantine so the pane chrome can re-attach
//!   the same row to the same provenance.
//! - **provenance.** Every session carries a [`SessionHeader`] with title,
//!   cwd hint, target badge, host class, execution-context ref, trust state,
//!   and the local-vs-managed boundary cue. The header is the canonical
//!   truth a tab chip, a status mirror, a support packet, and a restore
//!   prompt all quote verbatim.
//! - **lifecycle.** A small [`SessionLifecycleState`] state machine owns the
//!   `Requested → Starting → Active → LostTransport → ReconnectedSameIdentity
//!   → Closed` walk and the `Active → Quarantined` failure branch. Surfaces
//!   never invent "loading" / "broken" euphemisms; they read the canonical
//!   token.
//!
//! ## Failure-drill posture
//!
//! Terminating or restarting a session must not erase its header. The host
//! retains the [`SessionHeader`] across `LostTransport` and
//! `ReconnectedSameIdentity` transitions, and a `Closed` session keeps its
//! provenance row available until the consumer drops it. The fixture suite
//! under `/fixtures/terminal/session_cases/*.json` exercises this contract.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use aureline_workspace::TrustState;

/// Stable identifier for a single terminal session.
///
/// The id is derived from `(workspace_id, host_class, sequence)` at host time
/// and never mutates, even when transport drops, the shell exits, or the
/// session quarantines. The id is opaque on serialization boundaries; callers
/// must not parse it back into structured fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct PtySessionId(String);

impl PtySessionId {
    /// Construct a session id from its host inputs.
    pub fn from_parts(workspace_id: &str, host_class: HostClass, sequence: u64) -> Self {
        Self(format!(
            "pty:{workspace_id}|{host}|{sequence}",
            host = host_class.as_str(),
        ))
    }

    /// Stable string form. Safe to log and to ship in support bundles.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Consume into the underlying string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl std::fmt::Display for PtySessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Closed vocabulary for the session host class.
///
/// The class names whether the session's PTY runs on the local desktop or a
/// managed/remote target. Surfaces use the class to render the local-vs-
/// managed boundary cue without re-deriving truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostClass {
    /// PTY backed by the user's local desktop.
    HostDesktop,
    /// PTY backed by a managed remote agent.
    RemoteAgentPrimary,
    /// PTY backed by a local container or sandbox.
    LocalContainer,
}

impl HostClass {
    /// Stable string token used in records, fixtures, and a11y exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostDesktop => "host_desktop",
            Self::RemoteAgentPrimary => "remote_agent_primary",
            Self::LocalContainer => "local_container",
        }
    }

    /// Short human-readable badge label, e.g. for the bottom-panel tab chip.
    pub const fn target_badge(self) -> &'static str {
        match self {
            Self::HostDesktop => "Local",
            Self::RemoteAgentPrimary => "Remote",
            Self::LocalContainer => "Container",
        }
    }

    /// True when the session's target is not the local desktop and the chrome
    /// MUST render a visible boundary cue.
    pub const fn needs_boundary_cue(self) -> bool {
        !matches!(self, Self::HostDesktop)
    }

    /// Stable boundary-cue token, suitable for chrome and support exports.
    pub const fn boundary_cue_token(self) -> &'static str {
        match self {
            Self::HostDesktop => "boundary_cue_local_session",
            Self::RemoteAgentPrimary => "boundary_cue_remote_session",
            Self::LocalContainer => "boundary_cue_container_session",
        }
    }
}

/// Trust posture projected onto the terminal session.
///
/// Mirrors [`aureline_workspace::TrustState`]; we re-export the projection so
/// callers can consume the typed enum without forking the trust vocabulary.
pub type TerminalTrustState = TrustState;

/// Canonical lifecycle state for a terminal session.
///
/// The state machine is intentionally small. It models the seed contract that
/// downstream rows (M01-074 reconnect, M01-077 task channel, M01-078 debug
/// prep, M01-089 provider seed, M01-096 hot-path tracing) can build on
/// without re-deriving session truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionLifecycleState {
    /// User requested a fresh session through the command-dispatch boundary.
    Requested,
    /// Host is preparing the PTY (allocating, attaching, warming).
    Starting,
    /// PTY is attached and the session is interactive.
    Active,
    /// Transport dropped; the session is detached but its provenance is
    /// preserved for an explicit reconnect or fresh-session decision.
    LostTransport,
    /// Transport recovered against the same target identity. Read-only state
    /// resumes; in-flight mutations are NOT replayed.
    ReconnectedSameIdentity,
    /// Session is shutting down or has shut down. The header remains
    /// addressable until the consumer drops the row.
    Closed,
    /// Supervisor revoked the session because it exceeded a protocol-violation
    /// budget for the current trust tier. Re-admission requires the user to
    /// open a fresh session.
    Quarantined,
}

impl SessionLifecycleState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "session_requested",
            Self::Starting => "session_starting",
            Self::Active => "session_active",
            Self::LostTransport => "session_lost_transport",
            Self::ReconnectedSameIdentity => "session_reconnected_same_identity",
            Self::Closed => "session_closed",
            Self::Quarantined => "session_quarantined",
        }
    }

    /// True when the chrome should render the session as taking input today.
    pub const fn is_interactive(self) -> bool {
        matches!(self, Self::Active | Self::ReconnectedSameIdentity)
    }

    /// True when the chrome should render a degraded-state cue alongside the
    /// header (the row is still addressable, but not currently usable).
    pub const fn is_degraded(self) -> bool {
        matches!(
            self,
            Self::LostTransport | Self::Closed | Self::Quarantined
        )
    }
}

/// One transition frame emitted by the host.
///
/// Frames carry the same lifecycle vocabulary the header exports so audit
/// surfaces, support packets, and the activity center can replay the walk
/// without inventing local timestamps or reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionLifecycleTransition {
    pub session_id: PtySessionId,
    pub from_state: SessionLifecycleState,
    pub to_state: SessionLifecycleState,
    pub observed_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}

/// Canonical session-header record.
///
/// The header is the single truth a shell tab chip, a status-bar mirror, a
/// restore prompt, and a support packet all consume. Surfaces never compute a
/// title or cwd hint locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionHeader {
    pub session_id: PtySessionId,
    pub workspace_id: String,
    pub host_class: HostClass,
    pub target_badge: String,
    pub boundary_cue_token: String,
    pub display_title: String,
    /// Optional cwd hint. Absent when the session has not yet observed a cwd
    /// (e.g. baseline shell with no shell-integration signal).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd_hint: Option<String>,
    /// Stable reference to the execution-context object the session runs
    /// under. Surfaces consume this verbatim to wire context inspectors.
    pub execution_context_ref: String,
    pub trust_state: TerminalTrustState,
    pub trust_state_token: String,
    pub lifecycle_state: SessionLifecycleState,
    pub lifecycle_state_token: String,
    /// Sequence within the host. Stable across renames; bumps only when a new
    /// fresh session is opened.
    pub sequence: u64,
    pub created_at: String,
    pub last_observed_at: String,
}

impl SessionHeader {
    /// True when the chrome MUST render the local-vs-managed boundary cue.
    pub const fn needs_boundary_cue(&self) -> bool {
        self.host_class.needs_boundary_cue()
    }

    /// True when the chrome should render a degraded chip on this row.
    pub const fn is_degraded(&self) -> bool {
        self.lifecycle_state.is_degraded()
    }
}

/// Inputs accepted when opening a fresh session on the host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenSessionRequest<'a> {
    pub workspace_id: &'a str,
    pub host_class: HostClass,
    pub display_title: &'a str,
    pub cwd_hint: Option<&'a str>,
    pub execution_context_ref: &'a str,
    pub trust_state: TerminalTrustState,
    pub observed_at: &'a str,
}

/// Errors emitted by the host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PtyHostError {
    UnknownSession(PtySessionId),
    /// The host refused a transition that would erase a session's identity
    /// (e.g. moving directly from `Requested` to `Closed` without recording a
    /// header).
    InvalidTransition {
        session_id: PtySessionId,
        from_state: SessionLifecycleState,
        to_state: SessionLifecycleState,
    },
}

impl std::fmt::Display for PtyHostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownSession(id) => write!(f, "unknown terminal session: {id}"),
            Self::InvalidTransition {
                session_id,
                from_state,
                to_state,
            } => write!(
                f,
                "invalid lifecycle transition for {session_id}: {from} -> {to}",
                from = from_state.as_str(),
                to = to_state.as_str(),
            ),
        }
    }
}

impl std::error::Error for PtyHostError {}

/// One terminal session as the host knows it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PtySession {
    pub header: SessionHeader,
}

impl PtySession {
    /// Returns the session id.
    pub fn session_id(&self) -> &PtySessionId {
        &self.header.session_id
    }

    /// Returns the canonical session header.
    pub const fn header(&self) -> &SessionHeader {
        &self.header
    }

    /// Returns the current lifecycle state.
    pub const fn lifecycle_state(&self) -> SessionLifecycleState {
        self.header.lifecycle_state
    }
}

/// Single, inspectable PTY host.
///
/// The host does not spawn real processes in M1. It owns the canonical
/// session map, the lifecycle state machine, and the transition log so the
/// shell terminal pane and downstream rows can read one truth.
#[derive(Debug, Clone, Default)]
pub struct PtyHost {
    next_sequence: u64,
    sessions: BTreeMap<PtySessionId, PtySession>,
    order: Vec<PtySessionId>,
    transitions: Vec<SessionLifecycleTransition>,
}

impl PtyHost {
    /// Construct an empty host.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of sessions currently tracked (including closed sessions whose
    /// rows are still pinned for provenance).
    pub fn session_count(&self) -> usize {
        self.order.len()
    }

    /// Iterate sessions in insertion order. The order is stable across
    /// lifecycle transitions, so a tab strip never reshuffles when transport
    /// drops.
    pub fn sessions(&self) -> impl Iterator<Item = &PtySession> {
        self.order
            .iter()
            .filter_map(move |id| self.sessions.get(id))
    }

    /// Look up a session by id.
    pub fn session(&self, id: &PtySessionId) -> Option<&PtySession> {
        self.sessions.get(id)
    }

    /// Drain the transition log.
    pub fn drain_transitions(&mut self) -> Vec<SessionLifecycleTransition> {
        std::mem::take(&mut self.transitions)
    }

    /// Open a fresh session. The host mints a stable [`PtySessionId`], builds
    /// the canonical header, and records the `Requested` transition.
    pub fn open_session(&mut self, request: OpenSessionRequest<'_>) -> PtySessionId {
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);

        let session_id =
            PtySessionId::from_parts(request.workspace_id, request.host_class, sequence);
        let header = SessionHeader {
            session_id: session_id.clone(),
            workspace_id: request.workspace_id.to_owned(),
            host_class: request.host_class,
            target_badge: request.host_class.target_badge().to_owned(),
            boundary_cue_token: request.host_class.boundary_cue_token().to_owned(),
            display_title: request.display_title.to_owned(),
            cwd_hint: request.cwd_hint.map(str::to_owned),
            execution_context_ref: request.execution_context_ref.to_owned(),
            trust_state: request.trust_state,
            trust_state_token: request.trust_state.as_str().to_owned(),
            lifecycle_state: SessionLifecycleState::Requested,
            lifecycle_state_token: SessionLifecycleState::Requested.as_str().to_owned(),
            sequence,
            created_at: request.observed_at.to_owned(),
            last_observed_at: request.observed_at.to_owned(),
        };

        self.sessions.insert(
            session_id.clone(),
            PtySession {
                header: header.clone(),
            },
        );
        self.order.push(session_id.clone());
        self.transitions.push(SessionLifecycleTransition {
            session_id: session_id.clone(),
            from_state: SessionLifecycleState::Requested,
            to_state: SessionLifecycleState::Requested,
            observed_at: request.observed_at.to_owned(),
            reason_code: Some("session_opened".to_owned()),
        });
        session_id
    }

    /// Mark a session as starting (host is preparing the PTY).
    pub fn mark_starting(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        self.transition(
            session_id,
            SessionLifecycleState::Starting,
            observed_at,
            Some("starting"),
            |from| matches!(from, SessionLifecycleState::Requested),
        )
    }

    /// Mark a session as active (PTY attached and interactive).
    pub fn mark_active(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        self.transition(
            session_id,
            SessionLifecycleState::Active,
            observed_at,
            Some("attached"),
            |from| {
                matches!(
                    from,
                    SessionLifecycleState::Requested
                        | SessionLifecycleState::Starting
                        | SessionLifecycleState::ReconnectedSameIdentity
                )
            },
        )
    }

    /// Update the cwd hint as the host observes a new working directory. The
    /// hint is the canonical truth quoted by every header consumer.
    pub fn update_cwd_hint(
        &mut self,
        session_id: &PtySessionId,
        cwd_hint: Option<&str>,
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        let session = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| PtyHostError::UnknownSession(session_id.clone()))?;
        session.header.cwd_hint = cwd_hint.map(str::to_owned);
        session.header.last_observed_at = observed_at.to_owned();
        Ok(())
    }

    /// Update the display title (e.g. command name or window title escape).
    pub fn update_display_title(
        &mut self,
        session_id: &PtySessionId,
        display_title: &str,
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        let session = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| PtyHostError::UnknownSession(session_id.clone()))?;
        session.header.display_title = display_title.to_owned();
        session.header.last_observed_at = observed_at.to_owned();
        Ok(())
    }

    /// Record that transport dropped. The header is preserved verbatim so the
    /// pane chip continues to disclose target, cwd, and context after the
    /// drop.
    pub fn mark_lost_transport(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
        reason_code: Option<&str>,
    ) -> Result<(), PtyHostError> {
        self.transition(
            session_id,
            SessionLifecycleState::LostTransport,
            observed_at,
            reason_code.or(Some("transport_dropped")),
            |from| {
                matches!(
                    from,
                    SessionLifecycleState::Active | SessionLifecycleState::ReconnectedSameIdentity
                )
            },
        )
    }

    /// Record a successful reconnect against the same target identity. The
    /// session id, header, and sequence are unchanged so downstream surfaces
    /// can match the prior provenance row.
    pub fn mark_reconnected_same_identity(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
    ) -> Result<(), PtyHostError> {
        self.transition(
            session_id,
            SessionLifecycleState::ReconnectedSameIdentity,
            observed_at,
            Some("reconnected_same_identity"),
            |from| matches!(from, SessionLifecycleState::LostTransport),
        )
    }

    /// Quarantine a session because the supervisor revoked it.
    pub fn quarantine(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
        reason_code: &str,
    ) -> Result<(), PtyHostError> {
        self.transition(
            session_id,
            SessionLifecycleState::Quarantined,
            observed_at,
            Some(reason_code),
            |from| {
                matches!(
                    from,
                    SessionLifecycleState::Active
                        | SessionLifecycleState::ReconnectedSameIdentity
                        | SessionLifecycleState::LostTransport
                )
            },
        )
    }

    /// Close a session.
    pub fn close(
        &mut self,
        session_id: &PtySessionId,
        observed_at: &str,
        reason_code: Option<&str>,
    ) -> Result<(), PtyHostError> {
        self.transition(
            session_id,
            SessionLifecycleState::Closed,
            observed_at,
            reason_code.or(Some("closed")),
            |from| !matches!(from, SessionLifecycleState::Closed),
        )
    }

    fn transition(
        &mut self,
        session_id: &PtySessionId,
        to: SessionLifecycleState,
        observed_at: &str,
        reason_code: Option<&str>,
        guard: impl FnOnce(SessionLifecycleState) -> bool,
    ) -> Result<(), PtyHostError> {
        let session = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| PtyHostError::UnknownSession(session_id.clone()))?;
        let from = session.header.lifecycle_state;
        if !guard(from) {
            return Err(PtyHostError::InvalidTransition {
                session_id: session_id.clone(),
                from_state: from,
                to_state: to,
            });
        }
        session.header.lifecycle_state = to;
        session.header.lifecycle_state_token = to.as_str().to_owned();
        session.header.last_observed_at = observed_at.to_owned();
        self.transitions.push(SessionLifecycleTransition {
            session_id: session_id.clone(),
            from_state: from,
            to_state: to,
            observed_at: observed_at.to_owned(),
            reason_code: reason_code.map(str::to_owned),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open_local(host: &mut PtyHost) -> PtySessionId {
        host.open_session(OpenSessionRequest {
            workspace_id: "ws-test",
            host_class: HostClass::HostDesktop,
            display_title: "zsh",
            cwd_hint: Some("~/code/aureline"),
            execution_context_ref: "execution_context.local_desktop.workspace_root",
            trust_state: TrustState::Trusted,
            observed_at: "mono:0",
        })
    }

    #[test]
    fn open_session_records_stable_header_and_id() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        assert_eq!(id.as_str(), "pty:ws-test|host_desktop|0");

        let session = host.session(&id).expect("session must exist");
        let header = session.header();
        assert_eq!(header.session_id, id);
        assert_eq!(header.workspace_id, "ws-test");
        assert_eq!(header.host_class, HostClass::HostDesktop);
        assert_eq!(header.target_badge, "Local");
        assert_eq!(header.boundary_cue_token, "boundary_cue_local_session");
        assert_eq!(header.display_title, "zsh");
        assert_eq!(header.cwd_hint.as_deref(), Some("~/code/aureline"));
        assert_eq!(
            header.execution_context_ref,
            "execution_context.local_desktop.workspace_root"
        );
        assert_eq!(header.trust_state_token, "trusted");
        assert_eq!(header.lifecycle_state, SessionLifecycleState::Requested);
        assert!(!header.needs_boundary_cue());
    }

    #[test]
    fn lifecycle_walks_through_starting_active_lost_reconnect() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.update_cwd_hint(&id, Some("~/code/aureline/crates"), "mono:3")
            .unwrap();
        host.mark_lost_transport(&id, "mono:4", Some("network_drop"))
            .unwrap();
        host.mark_reconnected_same_identity(&id, "mono:5").unwrap();
        host.mark_active(&id, "mono:6").unwrap();
        host.close(&id, "mono:7", Some("user_closed")).unwrap();

        let session = host.session(&id).expect("session must exist");
        assert_eq!(session.lifecycle_state(), SessionLifecycleState::Closed);
        assert_eq!(
            session.header().cwd_hint.as_deref(),
            Some("~/code/aureline/crates"),
            "cwd hint preserved across transitions"
        );

        let transitions = host.drain_transitions();
        let walks: Vec<_> = transitions
            .iter()
            .map(|t| (t.from_state, t.to_state))
            .collect();
        assert!(walks.contains(&(
            SessionLifecycleState::Active,
            SessionLifecycleState::LostTransport
        )));
        assert!(walks.contains(&(
            SessionLifecycleState::LostTransport,
            SessionLifecycleState::ReconnectedSameIdentity
        )));
    }

    #[test]
    fn lost_transport_preserves_provenance_for_failure_drill() {
        // Failure drill: terminate transport unexpectedly. The header MUST
        // remain attached so the pane never collapses to an anonymous tab.
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.mark_lost_transport(&id, "mono:3", Some("network_drop"))
            .unwrap();

        let session = host.session(&id).expect("session must exist");
        assert_eq!(
            session.lifecycle_state(),
            SessionLifecycleState::LostTransport
        );
        assert_eq!(session.header().display_title, "zsh");
        assert_eq!(
            session.header().cwd_hint.as_deref(),
            Some("~/code/aureline")
        );
        assert_eq!(session.header().target_badge, "Local");
        assert_eq!(
            session.header().boundary_cue_token,
            "boundary_cue_local_session"
        );
        assert!(session.header().is_degraded());
    }

    #[test]
    fn quarantine_keeps_header_and_blocks_silent_recovery() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.quarantine(&id, "mono:3", "terminal_protocol_violation_budget_exceeded")
            .unwrap();

        let session = host.session(&id).expect("session must exist");
        assert_eq!(
            session.lifecycle_state(),
            SessionLifecycleState::Quarantined
        );
        assert!(session.header().is_degraded());

        // A quarantined session refuses to silently re-attach; the user must
        // open a fresh session through the command-dispatch boundary.
        let err = host.mark_active(&id, "mono:4").unwrap_err();
        assert!(matches!(err, PtyHostError::InvalidTransition { .. }));
    }

    #[test]
    fn remote_session_emits_visible_boundary_cue() {
        let mut host = PtyHost::new();
        let id = host.open_session(OpenSessionRequest {
            workspace_id: "ws-test",
            host_class: HostClass::RemoteAgentPrimary,
            display_title: "agent shell",
            cwd_hint: Some("/srv/code"),
            execution_context_ref: "execution_context.remote_agent.workspace_root",
            trust_state: TrustState::Trusted,
            observed_at: "mono:0",
        });
        let session = host.session(&id).expect("session must exist");
        assert_eq!(session.header().target_badge, "Remote");
        assert_eq!(
            session.header().boundary_cue_token,
            "boundary_cue_remote_session"
        );
        assert!(session.header().needs_boundary_cue());
    }

    #[test]
    fn ordering_is_stable_across_lifecycle_transitions() {
        let mut host = PtyHost::new();
        let a = open_local(&mut host);
        let b = host.open_session(OpenSessionRequest {
            workspace_id: "ws-test",
            host_class: HostClass::HostDesktop,
            display_title: "bash",
            cwd_hint: None,
            execution_context_ref: "execution_context.local_desktop.workspace_root",
            trust_state: TrustState::Trusted,
            observed_at: "mono:1",
        });
        host.mark_starting(&a, "mono:2").unwrap();
        host.mark_active(&a, "mono:3").unwrap();
        host.mark_lost_transport(&a, "mono:4", None).unwrap();

        let order: Vec<_> = host.sessions().map(|s| s.session_id().clone()).collect();
        assert_eq!(order, vec![a, b]);
    }

    #[test]
    fn unknown_session_returns_error() {
        let mut host = PtyHost::new();
        let bogus = PtySessionId::from_parts("ws-other", HostClass::HostDesktop, 99);
        let err = host.mark_active(&bogus, "mono:0").unwrap_err();
        assert_eq!(err, PtyHostError::UnknownSession(bogus));
    }
}
