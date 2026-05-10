//! Terminal pane: bottom-panel projection of the canonical PTY host.
//!
//! The terminal pane is the protected-row consumer for terminal sessions in
//! the live shell. It is a thin projection: it owns no session lifecycle,
//! identity, or provenance vocabulary of its own. It reads
//! [`aureline_terminal::PtyHost`] and projects each tracked session into a
//! serializable [`TerminalPaneTabRecord`] the bottom-panel chrome renders
//! verbatim.
//!
//! ## Why a projection rather than a private cache
//!
//! Two surfaces (the bottom-panel tab strip and the title-context bar's
//! activity indicator) must agree on session identity, lifecycle state, and
//! degraded chrome. Projecting from the host (rather than minting a private
//! struct) keeps both rows on the same truth and lets a support packet quote
//! the same record without translation.
//!
//! ## Failure-drill posture
//!
//! When transport drops or a session quarantines, the pane keeps the row
//! addressable, preserves the header, and surfaces a typed degraded chip
//! (`Offline` / `Limited` / `PolicyBlocked`) so the tab never collapses to an
//! anonymous "Terminal" label. The fixture suite under
//! `/fixtures/terminal/session_cases/*.json` exercises the drill.

use serde::{Deserialize, Serialize};

use aureline_terminal::{
    HostClass, PtyHost, PtySession, PtySessionId, SessionLifecycleState, TerminalTrustState,
};

use crate::state_cards::DegradedStateToken;

/// Stable record-kind tag carried in serialized terminal-pane snapshots.
pub const TERMINAL_PANE_SNAPSHOT_RECORD_KIND: &str = "terminal_pane_snapshot_record";
/// Schema version for the [`TerminalPaneSnapshot`] payload shape.
pub const TERMINAL_PANE_SNAPSHOT_SCHEMA_VERSION: u32 = 1;

/// One bottom-panel terminal tab row.
///
/// Every field is derived from the canonical session header. The row carries
/// its own `degraded_token` so the chrome renders one Local-vs-Remote chip
/// and at most one degraded chip per row without re-deriving lifecycle
/// vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalPaneTabRecord {
    pub session_id: PtySessionId,
    pub workspace_id: String,
    pub host_class: HostClass,
    pub target_badge: String,
    pub boundary_cue_token: String,
    /// True when the session's host is not the local desktop and the chrome
    /// MUST render the local-vs-managed boundary cue.
    pub boundary_cue_visible: bool,
    pub display_title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd_hint: Option<String>,
    pub execution_context_ref: String,
    pub trust_state: TerminalTrustState,
    pub trust_state_token: String,
    pub lifecycle_state: SessionLifecycleState,
    pub lifecycle_state_token: String,
    pub is_interactive: bool,
    /// Degraded-state chip the chrome renders next to the tab. `None` for an
    /// active row on a trusted local target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_token: Option<String>,
    pub created_at: String,
    pub last_observed_at: String,
}

impl TerminalPaneTabRecord {
    /// Project a tab row from one tracked session.
    pub fn project(session: &PtySession) -> Self {
        let header = session.header();
        let degraded = derive_degraded_token(header.lifecycle_state, header.trust_state);
        Self {
            session_id: header.session_id.clone(),
            workspace_id: header.workspace_id.clone(),
            host_class: header.host_class,
            target_badge: header.target_badge.clone(),
            boundary_cue_token: header.boundary_cue_token.clone(),
            boundary_cue_visible: header.host_class.needs_boundary_cue(),
            display_title: header.display_title.clone(),
            cwd_hint: header.cwd_hint.clone(),
            execution_context_ref: header.execution_context_ref.clone(),
            trust_state: header.trust_state,
            trust_state_token: header.trust_state_token.clone(),
            lifecycle_state: header.lifecycle_state,
            lifecycle_state_token: header.lifecycle_state_token.clone(),
            is_interactive: header.lifecycle_state.is_interactive(),
            degraded_token: degraded.map(|t| t.token().to_owned()),
            created_at: header.created_at.clone(),
            last_observed_at: header.last_observed_at.clone(),
        }
    }
}

/// One snapshot of the terminal pane.
///
/// The snapshot is the truth a tab strip renders, a support packet quotes,
/// and a restore prompt can replay against. The `tabs` order is the host's
/// stable insertion order; rows do not reshuffle on lifecycle churn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalPaneSnapshot {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub tabs: Vec<TerminalPaneTabRecord>,
    pub active_tab_id: Option<PtySessionId>,
}

impl TerminalPaneSnapshot {
    /// Project a snapshot from the canonical host. The first interactive tab
    /// (or the first tab when none is interactive) becomes the active tab so
    /// the chrome always has a focused row to render.
    pub fn project(workspace_id: &str, host: &PtyHost) -> Self {
        let tabs: Vec<TerminalPaneTabRecord> = host
            .sessions()
            .filter(|session| session.header().workspace_id == workspace_id)
            .map(TerminalPaneTabRecord::project)
            .collect();
        let active_tab_id = tabs
            .iter()
            .find(|tab| tab.is_interactive)
            .or_else(|| tabs.first())
            .map(|tab| tab.session_id.clone());
        Self {
            record_kind: TERMINAL_PANE_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: TERMINAL_PANE_SNAPSHOT_SCHEMA_VERSION,
            workspace_id: workspace_id.to_owned(),
            tabs,
            active_tab_id,
        }
    }

    /// True when the pane has at least one tab to render.
    pub fn has_tabs(&self) -> bool {
        !self.tabs.is_empty()
    }
}

/// Compute the degraded chip the chrome renders next to a tab. Returns `None`
/// for an interactive row on a trusted target.
const fn derive_degraded_token(
    state: SessionLifecycleState,
    trust: TerminalTrustState,
) -> Option<DegradedStateToken> {
    if !matches!(trust, TerminalTrustState::Trusted) {
        return Some(DegradedStateToken::PolicyBlocked);
    }
    match state {
        SessionLifecycleState::Requested | SessionLifecycleState::Starting => {
            Some(DegradedStateToken::Warming)
        }
        SessionLifecycleState::LostTransport => Some(DegradedStateToken::Offline),
        SessionLifecycleState::Closed => Some(DegradedStateToken::Limited),
        SessionLifecycleState::Quarantined => Some(DegradedStateToken::PolicyBlocked),
        SessionLifecycleState::Active | SessionLifecycleState::ReconnectedSameIdentity => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_terminal::pty_host::OpenSessionRequest;
    use aureline_workspace::TrustState;

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
    fn snapshot_projects_active_session_without_degraded_chip() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();

        let snapshot = TerminalPaneSnapshot::project("ws-test", &host);
        assert!(snapshot.has_tabs());
        assert_eq!(snapshot.active_tab_id.as_ref(), Some(&id));
        let tab = &snapshot.tabs[0];
        assert_eq!(tab.session_id, id);
        assert_eq!(tab.target_badge, "Local");
        assert!(!tab.boundary_cue_visible);
        assert!(tab.is_interactive);
        assert!(tab.degraded_token.is_none());
        assert_eq!(tab.cwd_hint.as_deref(), Some("~/code/aureline"));
    }

    #[test]
    fn lost_transport_keeps_provenance_and_renders_offline_chip() {
        // Failure drill: terminate the session unexpectedly. The tab must keep
        // the same id, header, and target badge, and surface an Offline chip
        // rather than collapsing to an anonymous "Terminal" tab.
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.mark_lost_transport(&id, "mono:3", Some("network_drop"))
            .unwrap();

        let snapshot = TerminalPaneSnapshot::project("ws-test", &host);
        let tab = snapshot
            .tabs
            .iter()
            .find(|tab| tab.session_id == id)
            .expect("tab must remain");
        assert_eq!(tab.lifecycle_state, SessionLifecycleState::LostTransport);
        assert_eq!(tab.target_badge, "Local");
        assert_eq!(tab.cwd_hint.as_deref(), Some("~/code/aureline"));
        assert_eq!(tab.degraded_token.as_deref(), Some("Offline"));
        assert!(!tab.is_interactive);
    }

    #[test]
    fn quarantined_session_renders_policy_blocked_chip() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.quarantine(&id, "mono:3", "terminal_protocol_violation_budget_exceeded")
            .unwrap();

        let snapshot = TerminalPaneSnapshot::project("ws-test", &host);
        let tab = &snapshot.tabs[0];
        assert_eq!(tab.lifecycle_state, SessionLifecycleState::Quarantined);
        assert_eq!(tab.degraded_token.as_deref(), Some("PolicyBlocked"));
    }

    #[test]
    fn remote_session_marks_boundary_cue_visible() {
        let mut host = PtyHost::new();
        host.open_session(OpenSessionRequest {
            workspace_id: "ws-test",
            host_class: HostClass::RemoteAgentPrimary,
            display_title: "agent shell",
            cwd_hint: Some("/srv/code"),
            execution_context_ref: "execution_context.remote_agent.workspace_root",
            trust_state: TrustState::Trusted,
            observed_at: "mono:0",
        });
        let snapshot = TerminalPaneSnapshot::project("ws-test", &host);
        let tab = &snapshot.tabs[0];
        assert_eq!(tab.target_badge, "Remote");
        assert!(tab.boundary_cue_visible);
        assert_eq!(tab.boundary_cue_token, "boundary_cue_remote_session");
    }

    #[test]
    fn snapshot_filters_to_active_workspace() {
        let mut host = PtyHost::new();
        let _local_a = open_local(&mut host);
        host.open_session(OpenSessionRequest {
            workspace_id: "ws-other",
            host_class: HostClass::HostDesktop,
            display_title: "bash",
            cwd_hint: None,
            execution_context_ref: "execution_context.local_desktop.workspace_root",
            trust_state: TrustState::Trusted,
            observed_at: "mono:1",
        });
        let snapshot = TerminalPaneSnapshot::project("ws-test", &host);
        assert_eq!(snapshot.tabs.len(), 1);
        assert_eq!(snapshot.workspace_id, "ws-test");
    }
}
