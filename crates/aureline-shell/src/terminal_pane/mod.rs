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

use aureline_auth::{
    BrowserCallbackPacket, ClaimedIdentityRow, ClaimedIdentitySurfaceRow, CredentialStateChip,
    CredentialStateRow, ProviderAccountRegistry, ShellAuthChip, SystemBrowserAlphaPacket,
};
use aureline_runtime::ExecutionContext;
use aureline_terminal::{
    HostClass, PtyHost, PtySession, PtySessionId, RestoredTerminalRecord, SessionLifecycleState,
    TerminalHeaderRecord, TerminalRuntimeChipSource, TerminalTrustState,
};

use crate::run_context::RunContextSummary;
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
    /// Canonical header strip with target, cwd, runtime, and restore chips.
    pub header: TerminalHeaderRecord,
    pub target_badge: String,
    pub boundary_cue_token: String,
    /// True when the session's host is not the local desktop and the chrome
    /// MUST render the local-vs-managed boundary cue.
    pub boundary_cue_visible: bool,
    pub display_title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd_hint: Option<String>,
    pub execution_context_ref: String,
    /// Shared execution-context summary joined by `execution_context_ref`
    /// when the caller has the canonical runtime record available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_summary: Option<RunContextSummary>,
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
        let header_record = TerminalHeaderRecord::project_session(session);
        let degraded = derive_degraded_token(header.lifecycle_state, header.trust_state);
        Self {
            session_id: header.session_id.clone(),
            workspace_id: header.workspace_id.clone(),
            host_class: header.host_class,
            header: header_record,
            target_badge: header.target_badge.clone(),
            boundary_cue_token: header.boundary_cue_token.clone(),
            boundary_cue_visible: header.host_class.needs_boundary_cue(),
            display_title: header.display_title.clone(),
            cwd_hint: header.cwd_hint.clone(),
            execution_context_ref: header.execution_context_ref.clone(),
            context_summary: None,
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
///
/// The optional `shell_auth_chip` row carries the local-versus-managed
/// vocabulary projected from an [`aureline_auth::BrowserCallbackPacket`]. The
/// pane consumes this projection by reference; it never invents a local
/// `is_signed_in` boolean and never collapses the boundary chip into a
/// generic `Connected` badge. When no auth packet is wired the pane keeps
/// rendering the no-account local path truthfully.
///
/// The optional `credential_state_chips` rows carry the per-credential
/// storage / scope / expiry / revoke-action / locked-or-unavailable posture
/// projected from a seed [`aureline_auth::ProviderAccountRegistry`]. The pane
/// quotes each chip verbatim; it never collapses a locked or unavailable
/// store posture into a generic warning chip and never silently downgrades
/// to a plaintext-file fallback.
///
/// The optional `claimed_identity_rows` carry the system-browser defaulting
/// posture for managed / provider identity rows. They expose provider/org
/// scope, timeout or expiry, device-code and stay-local alternatives, and
/// the local-work continuation note without re-deriving auth state in shell
/// code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalPaneSnapshot {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub tabs: Vec<TerminalPaneTabRecord>,
    pub active_tab_id: Option<PtySessionId>,
    /// Projected auth chip the bottom-panel chrome renders next to the
    /// terminal tab strip. Absent when no auth packet has been wired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shell_auth_chip: Option<ShellAuthChip>,
    /// Projected credential-state chips the bottom-panel chrome renders below
    /// the shell-auth chip. Empty when no provider/account registry has been
    /// wired.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub credential_state_chips: Vec<CredentialStateChip>,
    /// Claimed identity rows projected from the system-browser alpha packet.
    /// Empty when no claimed identity packet has been wired.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claimed_identity_rows: Vec<ClaimedIdentitySurfaceRow>,
    /// Restored transcript / ended-session rows projected from the canonical
    /// terminal restore module. The bottom-panel chrome renders these as
    /// closed-tab transcript objects with no implicit rerun action; live
    /// execution requires a fresh session through the command-dispatch
    /// boundary. Empty when no prior session was restored.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restored_terminals: Vec<RestoredTerminalRecord>,
    /// Header strips for restored terminal rows. Kept parallel to
    /// `restored_terminals` so restored transcript state is inspectable even
    /// when there is no live tab.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restored_terminal_headers: Vec<TerminalHeaderRecord>,
}

impl TerminalPaneSnapshot {
    /// Project a snapshot from the canonical host. The first interactive tab
    /// (or the first tab when none is interactive) becomes the active tab so
    /// the chrome always has a focused row to render.
    pub fn project(workspace_id: &str, host: &PtyHost) -> Self {
        Self::project_with_auth_chip(workspace_id, host, None)
    }

    /// Project a snapshot and attach the local-versus-managed shell auth
    /// chip from a seed [`aureline_auth::BrowserCallbackPacket`].
    pub fn project_with_auth_packet(
        workspace_id: &str,
        host: &PtyHost,
        packet: &BrowserCallbackPacket,
    ) -> Self {
        Self::project_with_auth_chip(workspace_id, host, Some(ShellAuthChip::from_packet(packet)))
    }

    /// Project a snapshot with a pre-built shell auth chip.
    pub fn project_with_auth_chip(
        workspace_id: &str,
        host: &PtyHost,
        shell_auth_chip: Option<ShellAuthChip>,
    ) -> Self {
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
            shell_auth_chip,
            credential_state_chips: Vec::new(),
            claimed_identity_rows: Vec::new(),
            restored_terminals: Vec::new(),
            restored_terminal_headers: Vec::new(),
        }
    }

    /// Attach credential-state chips projected from a seed
    /// [`aureline_auth::ProviderAccountRegistry`]. The chrome renders the
    /// chips below the shell-auth chip in registry insertion order; locked
    /// and unavailable rows stay readable so the user can tell that a saved
    /// alias exists but cannot be resolved.
    pub fn with_credential_registry(mut self, registry: &ProviderAccountRegistry) -> Self {
        self.credential_state_chips = registry
            .credential_states
            .iter()
            .map(CredentialStateChip::from_row)
            .collect();
        self
    }

    /// Attach credential-state chips projected from an explicit list of
    /// [`aureline_auth::CredentialStateRow`] records. Surfaces that consume a
    /// filtered subset of the seed registry (for example, the rows bound to
    /// the active workspace) reach for this entry point so the snapshot
    /// stays joined to the same row vocabulary.
    pub fn with_credential_rows<'a, I>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = &'a CredentialStateRow>,
    {
        self.credential_state_chips = rows
            .into_iter()
            .map(CredentialStateChip::from_row)
            .collect();
        self
    }

    /// Attach claimed identity rows projected from a
    /// [`aureline_auth::SystemBrowserAlphaPacket`]. The pane renders these
    /// rows near the shell-auth chip so managed / provider identity scope,
    /// expiry, device-code fallback, and stay-local continuation remain
    /// inspectable in the shell.
    pub fn with_system_browser_alpha_packet(mut self, packet: &SystemBrowserAlphaPacket) -> Self {
        self.claimed_identity_rows = packet.surface_rows();
        self
    }

    /// Attach claimed identity rows from an explicit iterator of
    /// [`aureline_auth::ClaimedIdentityRow`] records.
    pub fn with_claimed_identity_rows<'a, I>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = &'a ClaimedIdentityRow>,
    {
        self.claimed_identity_rows = rows
            .into_iter()
            .map(ClaimedIdentitySurfaceRow::from_row)
            .collect();
        self
    }

    /// Attach restored transcript / ended-session rows projected from the
    /// canonical terminal restore module. The bottom-panel renders these as
    /// closed-tab objects scoped to the pane's workspace; rows for other
    /// workspaces are filtered out so the chrome never displays foreign
    /// provenance. The seed contract guarantees `auto_rerun_forbidden` is
    /// preserved verbatim on every attached record.
    pub fn with_restored_terminals<I>(mut self, restored: I) -> Self
    where
        I: IntoIterator<Item = RestoredTerminalRecord>,
    {
        let restored_terminals: Vec<RestoredTerminalRecord> = restored
            .into_iter()
            .filter(|record| record.workspace_id == self.workspace_id)
            .collect();
        self.restored_terminal_headers = restored_terminals
            .iter()
            .map(TerminalHeaderRecord::project_restored)
            .collect();
        self.restored_terminals = restored_terminals;
        self
    }

    /// Attach shared run-context summaries to matching terminal tabs.
    ///
    /// The join key is the terminal session header's `execution_context_ref`;
    /// unmatched tabs remain visible without fabricating context truth.
    pub fn with_run_contexts<'a, I>(self, contexts: I) -> Self
    where
        I: IntoIterator<Item = &'a ExecutionContext>,
    {
        self.with_run_context_summaries(contexts.into_iter().map(RunContextSummary::project))
    }

    /// Attach pre-projected summaries to matching terminal tabs.
    pub fn with_run_context_summaries<I>(mut self, summaries: I) -> Self
    where
        I: IntoIterator<Item = RunContextSummary>,
    {
        let summaries: Vec<RunContextSummary> = summaries.into_iter().collect();
        for tab in &mut self.tabs {
            let summary = summaries
                .iter()
                .find(|summary| summary.execution_context_ref == tab.execution_context_ref);
            tab.context_summary = summary.cloned();
            if let Some(summary) = summary {
                tab.header = tab
                    .header
                    .clone()
                    .with_runtime_source(runtime_source_from_summary(summary));
            }
        }
        for header in &mut self.restored_terminal_headers {
            if let Some(summary) = summaries
                .iter()
                .find(|summary| summary.execution_context_ref == header.execution_context_ref)
            {
                *header = header
                    .clone()
                    .with_runtime_source(runtime_source_from_summary(summary));
            }
        }
        self
    }

    /// True when the pane has at least one tab to render.
    pub fn has_tabs(&self) -> bool {
        !self.tabs.is_empty()
    }

    /// True when the pane has at least one restored transcript / ended-session
    /// row to render.
    pub fn has_restored_terminals(&self) -> bool {
        !self.restored_terminals.is_empty()
    }

    /// True when at least one attached restored terminal carries a retained
    /// transcript body (i.e. is not an ended-session-only or declined record).
    pub fn has_restored_transcripts(&self) -> bool {
        self.restored_terminals
            .iter()
            .any(RestoredTerminalRecord::has_transcript)
    }

    /// True when at least one attached credential-state chip sits in an
    /// unavailable state class. The chrome reads this to know whether to
    /// surface the visible-recovery row.
    pub fn has_unavailable_credential_state(&self) -> bool {
        self.credential_state_chips
            .iter()
            .any(|chip| chip.state_class.is_unavailable_class())
    }

    /// True when any claimed identity row would strand the user without an
    /// available device-code or stay-local path. Conforming snapshots return
    /// `false`.
    pub fn has_claimed_identity_dead_end(&self) -> bool {
        self.claimed_identity_rows
            .iter()
            .any(|row| row.dead_end_without_local_continuation)
    }
}

fn runtime_source_from_summary(summary: &RunContextSummary) -> TerminalRuntimeChipSource {
    TerminalRuntimeChipSource {
        execution_context_ref: summary.execution_context_ref.clone(),
        surface_token: summary.surface_token.clone(),
        target_class_token: summary.target_class_token.clone(),
        toolchain_class_token: summary.toolchain_class_token.clone(),
        toolchain_id: summary.toolchain_id.clone(),
        resolved_version: summary.resolved_version.clone(),
        target_confidence_level_token: summary.target_confidence_level_token.clone(),
        prebuild_reuse_state_token: summary.prebuild_reuse_state_token.clone(),
        mixed_version_state_token: summary.mixed_version_state_token.clone(),
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
    fn snapshot_attaches_local_only_auth_chip_for_no_account_path() {
        // Protected walk: open a terminal session against the no-account local
        // path and confirm the bottom-panel snapshot quotes the projected
        // local-versus-managed chip from the seed auth packet without
        // blocking local work.
        use aureline_auth::{
            BrowserCallbackHandoff, ShellAuthVocabulary, StageAccountFreeLocalRequest,
        };

        let mut host = PtyHost::new();
        let _id = open_local(&mut host);
        let handoff =
            BrowserCallbackHandoff::stage_account_free_local(StageAccountFreeLocalRequest {
                packet_id: "browser_callback_packet.account_free_local.demo",
                correlation_id: "callback_correlation.account_free_local.demo",
                pending_session_id: "pending_session.account_free_local.demo",
                provider_domain_label: "No sign-in required",
                destination_class_label: "No browser handoff required",
                return_anchor_ref: "return_anchor.account_free_local.desktop",
                return_target_label: "Aureline desktop – local workspace",
                minted_at: "2026-04-23T10:00:00Z",
                recovery_copy_label:
                    "You are using Aureline without a sign-in. Local work stays on this device.",
                execution_context_ref: Some("execution_context.local_desktop.workspace_root"),
            });

        let snapshot =
            TerminalPaneSnapshot::project_with_auth_packet("ws-test", &host, handoff.packet());
        let chip = snapshot
            .shell_auth_chip
            .as_ref()
            .expect("snapshot quotes the seed auth chip");
        assert_eq!(chip.vocabulary, ShellAuthVocabulary::AccountFreeLocal);
        assert_eq!(chip.chip_label, "Local only");
        assert!(chip.local_path_available);
        assert!(!chip.visible_recovery_required);
    }

    #[test]
    fn snapshot_attaches_reauth_required_chip_when_managed_callback_is_pending() {
        // Failure-drill posture in the consumer: a managed sign-in is staged
        // but the browser return has not yet been validated. The snapshot
        // surfaces the typed reauth chip, the visible-recovery flag, and the
        // preserved local-path hint so the no-account local flow keeps
        // working.
        use aureline_auth::{
            AccountBoundaryClass, BrowserCallbackHandoff, IdentityModeAlias, PreservedLocalWork,
            PreservedLocalWorkPostureClass, RetryPathClass, ReturnModeClass,
            ReturnOriginValidationClass, ReturnTenantOrWorkspaceMatchRule, ShellAuthVocabulary,
            StageSystemBrowserHandoffRequest, TrustState as AuthTrustState,
        };

        let mut host = PtyHost::new();
        let _id = open_local(&mut host);
        let handoff = BrowserCallbackHandoff::stage_system_browser_handoff(
            StageSystemBrowserHandoffRequest {
                packet_id: "browser_callback_packet.managed_sign_in.demo",
                identity_mode: IdentityModeAlias::ManagedConvenience,
                account_boundary_class: AccountBoundaryClass::Managed,
                trust_state: AuthTrustState::Trusted,
                provider_domain_label: "login.acme.example",
                destination_class_label: "Customer-managed identity provider (system browser)",
                return_target_label: "Aureline desktop – payments-prod workspace",
                return_anchor_ref: "return_anchor.managed_sign_in.payments_prod",
                return_mode_class: ReturnModeClass::LoopbackHttpReturn,
                return_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
                return_tenant_or_workspace_match_rule:
                    ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspaceAndTenant,
                return_policy_check_refs: &[],
                bound_workspace_ref: Some("workspace.payments_prod"),
                bound_tenant_or_org_ref: Some("tenant.acme_prod"),
                bound_actor_subject_ref: Some("actor_subject.sam.acme"),
                correlation_id: "callback_correlation.managed_sign_in.demo",
                pending_session_id: "pending_session.managed_sign_in.demo",
                state_token_alias: "state_alias.managed_sign_in.demo",
                nonce_alias: "nonce_alias.managed_sign_in.demo",
                pkce_challenge_alias: Some("pkce_alias.managed_sign_in.demo"),
                issued_at: "2026-04-23T10:10:00Z",
                expires_at: "2026-04-23T10:20:00Z",
                recovery_copy_label:
                    "Continue sign-in in your browser. Local work keeps saving to this device.",
                primary_recovery_action: RetryPathClass::RetryInSystemBrowser,
                fallback_recovery_actions: &[
                    RetryPathClass::SwitchToDeviceCode,
                    RetryPathClass::ContinueLocalWithoutSignIn,
                ],
                repair_hook_ref: None,
                preserved_local_work: PreservedLocalWork {
                    posture_class:
                        PreservedLocalWorkPostureClass::LocalWorkIntactWithManagedNarrowed,
                    note: "Local work intact while managed sign-in is incomplete.".to_owned(),
                    retained_capabilities: vec!["Edit, save, undo, search locally.".to_owned()],
                    blocked_capabilities: vec![
                        "Fetch managed settings sync while sign-in is incomplete.".to_owned(),
                    ],
                },
                execution_context_ref: Some("execution_context.auth.managed_sign_in.payments_prod"),
            },
        )
        .expect("managed outbound handoff stages cleanly");

        let snapshot =
            TerminalPaneSnapshot::project_with_auth_packet("ws-test", &host, handoff.packet());
        let chip = snapshot
            .shell_auth_chip
            .as_ref()
            .expect("snapshot quotes the seed auth chip");
        assert_eq!(chip.vocabulary, ShellAuthVocabulary::ReauthRequired);
        assert!(chip.visible_recovery_required);
        assert!(
            chip.local_path_available,
            "managed sign-in pending must not block the no-account local path"
        );
    }

    #[test]
    fn snapshot_surfaces_claimed_identity_scope_expiry_and_fallbacks() {
        // Protected walk: a managed provider row is claimed by the shell. The
        // terminal-pane snapshot must expose provider/org scope, expiry, the
        // system-browser default, device-code fallback, and stay-local
        // continuation rather than reducing the row to a generic auth badge.
        use aureline_auth::{
            AccountBoundaryClass, BrowserLaunchPolicyClass, ClaimedIdentityRow,
            ClaimedIdentityStateClass, IdentityModeAlias, PreservedLocalWork,
            PreservedLocalWorkPostureClass, StageClaimedIdentityRowRequest,
            SystemBrowserAlphaPacket, TrustState as AuthTrustState,
        };

        let mut host = PtyHost::new();
        let _id = open_local(&mut host);
        let row = ClaimedIdentityRow::stage(StageClaimedIdentityRowRequest {
            row_id: "claimed-identity:managed:payments-prod",
            state_class: ClaimedIdentityStateClass::AwaitingSystemBrowser,
            account_boundary_class: AccountBoundaryClass::Managed,
            identity_mode: IdentityModeAlias::ManagedConvenience,
            trust_state: AuthTrustState::Trusted,
            provider_label: "Acme identity",
            provider_domain_label: "login.acme.example",
            provider_scope_label: "payments-prod tenant",
            bound_workspace_ref: Some("workspace:payments-prod"),
            bound_tenant_or_org_ref: Some("tenant:acme-prod"),
            bound_actor_subject_ref: Some("actor-subject:sam.acme"),
            browser_launch_policy_class: BrowserLaunchPolicyClass::SystemDefaultBrowserRequired,
            system_browser_supported: true,
            device_code_supported: true,
            stay_local_supported: true,
            issued_at: Some("2026-05-13T08:00:00Z"),
            expires_at: Some("2026-05-13T08:10:00Z"),
            expiry_summary_label: "System-browser handoff expires in 10 minutes.",
            device_code_expires_at: Some("2026-05-13T08:15:00Z"),
            device_code_ref: Some("device-code:managed:payments-prod"),
            local_continuity_label: "Local files and unsaved edits remain available.",
            preserved_local_work: PreservedLocalWork {
                posture_class: PreservedLocalWorkPostureClass::LocalWorkIntactWithManagedNarrowed,
                note: "Local work remains available while managed auth is incomplete.".to_owned(),
                retained_capabilities: vec![
                    "Edit local files.".to_owned(),
                    "Save local files.".to_owned(),
                    "Use local Git.".to_owned(),
                ],
                blocked_capabilities: vec!["Managed settings sync waits for sign-in.".to_owned()],
            },
            auth_callback_packet_ref: Some("auth-callback:managed:payments-prod"),
            browser_handoff_packet_ref: Some("browser-handoff:managed:payments-prod"),
            native_boundary_handoff_ref: Some("native-handoff:auth-callback:payments-prod"),
            embedded_boundary_card_ref: None,
            managed_session_state_ref: Some("managed-session:payments-prod"),
            recovery_copy_label: "Continue sign-in in your browser or stay local.",
            primary_recovery_action: None,
            support_export_ref: Some("support-export:auth:payments-prod"),
            execution_context_ref: Some("execution-context:auth:payments-prod"),
            minted_at: "2026-05-13T08:00:01Z",
        })
        .expect("claimed identity row stages");
        let packet = SystemBrowserAlphaPacket::new(
            "system-browser-alpha:terminal-pane:test",
            vec![row],
            "2026-05-13T08:00:01Z",
        );

        let snapshot = TerminalPaneSnapshot::project("ws-test", &host)
            .with_system_browser_alpha_packet(&packet);
        assert_eq!(snapshot.claimed_identity_rows.len(), 1);
        assert!(!snapshot.has_claimed_identity_dead_end());
        let row = &snapshot.claimed_identity_rows[0];
        assert_eq!(row.provider_domain_label, "login.acme.example");
        assert_eq!(row.provider_scope_label, "payments-prod tenant");
        assert_eq!(row.default_action_token, "open_system_browser");
        assert_eq!(row.expires_at.as_deref(), Some("2026-05-13T08:10:00Z"));
        assert!(row.device_code_available);
        assert!(row.stay_local_available);
        assert!(row.local_work_available);
        assert_eq!(
            row.native_boundary_handoff_ref.as_deref(),
            Some("native-handoff:auth-callback:payments-prod")
        );
    }

    #[test]
    fn snapshot_attaches_credential_state_chips_from_provider_registry() {
        // Protected walk: open a terminal session and attach the seed
        // provider/account registry. The bottom-panel snapshot quotes one
        // chip per credential-state row verbatim — storage mode, scope,
        // expiry, revoke action, and local-work continuity stay readable.
        use aureline_auth::{
            CredentialLifetime, CredentialScope, CredentialStateClass, CredentialStateRow,
            IdentityModeAlias as AuthIdentityMode, LifetimeClass, ProviderAccountRecord,
            ProviderAccountRegistry, RetryPathClass, RevokeActionClass, StorageModeClass,
            StoragePosture, StoreSourceClass, TrustState as AuthTrustState,
            CREDENTIAL_STATE_ROW_RECORD_KIND, CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
            PROVIDER_ACCOUNT_RECORD_KIND,
        };

        let mut host = PtyHost::new();
        let _id = open_local(&mut host);

        let mut registry = ProviderAccountRegistry::new(
            "provider_account_registry.terminal_pane.demo",
            "2026-04-29T09:05:00Z",
        );
        registry.upsert_account(ProviderAccountRecord {
            record_kind: PROVIDER_ACCOUNT_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
            provider_account_id: "provider_account.local.byok_ai".to_owned(),
            provider_domain_label: "Local BYOK AI provider".to_owned(),
            destination_class_label: "BYOK AI key (local)".to_owned(),
            account_boundary_class: aureline_auth::AccountBoundaryClass::LocalOnly,
            identity_mode: AuthIdentityMode::AccountFreeLocal,
            trust_state: AuthTrustState::Trusted,
            bound_workspace_ref: Some("workspace.local.demo".to_owned()),
            bound_tenant_or_org_ref: None,
            bound_actor_subject_ref: None,
            credential_state_row_refs: vec!["credential_state.local.byok_ai.0001".to_owned()],
            minted_at: "2026-04-29T09:05:00Z".to_owned(),
        });
        registry.upsert_credential_state(CredentialStateRow {
            record_kind: CREDENTIAL_STATE_ROW_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
            credential_state_id: "credential_state.local.byok_ai.0001".to_owned(),
            state_class: CredentialStateClass::HandleOnly,
            display_label: "Local BYOK AI provider".to_owned(),
            provider_account_ref: "provider_account.local.byok_ai".to_owned(),
            source_label: "OS keychain item".to_owned(),
            authority_alias_ref: Some("credential_alias.byok_ai.default".to_owned()),
            authority_handle_ref: Some("credential_handle.byok_ai.default".to_owned()),
            scope: CredentialScope {
                scope_label: "Current local workspace".to_owned(),
                audience_label: "Local AI provider requests".to_owned(),
                bound_workspace_ref: Some("workspace.local.demo".to_owned()),
                bound_tenant_or_org_ref: None,
                bound_actor_subject_ref: None,
            },
            storage: StoragePosture {
                storage_mode: StorageModeClass::SystemCredentialStore,
                store_source: StoreSourceClass::OsKeychain,
                session_only_downgrade_visible: false,
                plaintext_fallback_allowed: false,
                raw_secret_material_present: false,
                storage_note: "OS keychain holds the BYOK AI alias.".to_owned(),
            },
            lifetime: CredentialLifetime {
                lifetime_class: LifetimeClass::PersistentUntilRevoked,
                issued_at: Some("2026-04-29T09:00:00Z".to_owned()),
                expires_at: None,
                revocation_path_label: "Remove saved BYOK AI key".to_owned(),
                revoke_action: RevokeActionClass::RemoveSavedProviderSession,
            },
            identity_mode: AuthIdentityMode::AccountFreeLocal,
            trust_state: AuthTrustState::Trusted,
            local_work_continues: true,
            unavailable_reason: None,
            recovery_copy_label: "BYOK AI is ready. Local work stays on this device.".to_owned(),
            primary_recovery_action: RetryPathClass::ContinueLocalWithoutSignIn,
            execution_context_ref: Some(
                "execution_context.local_desktop.workspace_root".to_owned(),
            ),
            minted_at: "2026-04-29T09:05:05Z".to_owned(),
        });

        let snapshot =
            TerminalPaneSnapshot::project("ws-test", &host).with_credential_registry(&registry);
        assert_eq!(snapshot.credential_state_chips.len(), 1);
        let chip = &snapshot.credential_state_chips[0];
        assert_eq!(chip.state_class, CredentialStateClass::HandleOnly);
        assert_eq!(chip.storage_mode_token, "system_credential_store");
        assert_eq!(chip.store_source_token, "os_keychain");
        assert_eq!(chip.revocation_path_label, "Remove saved BYOK AI key");
        assert_eq!(
            chip.revoke_action,
            RevokeActionClass::RemoveSavedProviderSession
        );
        assert!(chip.local_work_continues);
        assert!(!chip.visible_recovery_required);
        assert!(!chip.plaintext_fallback_allowed);
        assert!(!chip.raw_secret_material_present);
        assert!(!snapshot.has_unavailable_credential_state());
    }

    #[test]
    fn snapshot_surfaces_locked_store_chip_after_failure_drill() {
        // Failure drill: lock the OS keychain after the registry is staged.
        // The snapshot's credential-state chip flips to `locked`, the
        // unavailable-reason token is `store_locked`, the recovery action
        // becomes `resume_after_credential_store_unlock`, and the chrome can
        // tell that the saved alias still exists. The seed contract forbids
        // a silent plaintext-file fallback.
        use aureline_auth::{
            CredentialLifetime, CredentialScope, CredentialStateClass, CredentialStateRow,
            CredentialUnavailableReason, IdentityModeAlias as AuthIdentityMode, LifetimeClass,
            ProviderAccountRegistry, RetryPathClass, RevokeActionClass, StorageModeClass,
            StoragePosture, StoreSourceClass, TrustState as AuthTrustState,
            CREDENTIAL_STATE_ROW_RECORD_KIND, CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
        };

        let mut host = PtyHost::new();
        let _id = open_local(&mut host);
        let mut registry = ProviderAccountRegistry::new(
            "provider_account_registry.terminal_pane.failure_drill",
            "2026-04-29T09:05:00Z",
        );
        registry.upsert_credential_state(CredentialStateRow {
            record_kind: CREDENTIAL_STATE_ROW_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
            credential_state_id: "credential_state.managed.payments_prod.0001".to_owned(),
            state_class: CredentialStateClass::HandleOnly,
            display_label: "Managed provider session".to_owned(),
            provider_account_ref: "provider_account.managed.payments_prod".to_owned(),
            source_label: "OS keychain item".to_owned(),
            authority_alias_ref: Some("credential_alias.managed.payments_prod".to_owned()),
            authority_handle_ref: Some("credential_handle.managed.payments_prod".to_owned()),
            scope: CredentialScope {
                scope_label: "payments-prod workspace".to_owned(),
                audience_label: "Managed sign-in refresh".to_owned(),
                bound_workspace_ref: Some("workspace.payments_prod".to_owned()),
                bound_tenant_or_org_ref: Some("tenant.acme_prod".to_owned()),
                bound_actor_subject_ref: Some("actor_subject.sam.acme".to_owned()),
            },
            storage: StoragePosture {
                storage_mode: StorageModeClass::SystemCredentialStore,
                store_source: StoreSourceClass::OsKeychain,
                session_only_downgrade_visible: false,
                plaintext_fallback_allowed: false,
                raw_secret_material_present: false,
                storage_note: "OS keychain holds the managed provider-session alias.".to_owned(),
            },
            lifetime: CredentialLifetime {
                lifetime_class: LifetimeClass::PersistentUntilRevoked,
                issued_at: Some("2026-04-29T09:00:00Z".to_owned()),
                expires_at: None,
                revocation_path_label: "Remove saved provider session".to_owned(),
                revoke_action: RevokeActionClass::RemoveSavedProviderSession,
            },
            identity_mode: AuthIdentityMode::ManagedConvenience,
            trust_state: AuthTrustState::Trusted,
            local_work_continues: true,
            unavailable_reason: None,
            recovery_copy_label: "Managed sign-in is ready. Local work keeps saving to this \
                                  device."
                .to_owned(),
            primary_recovery_action: RetryPathClass::RetryInSystemBrowser,
            execution_context_ref: Some(
                "execution_context.auth.managed_sign_in.payments_prod".to_owned(),
            ),
            minted_at: "2026-04-29T09:05:05Z".to_owned(),
        });

        let affected = registry.lock_store(StoreSourceClass::OsKeychain);
        assert_eq!(affected, 1);

        let snapshot =
            TerminalPaneSnapshot::project("ws-test", &host).with_credential_registry(&registry);
        assert!(snapshot.has_unavailable_credential_state());
        let chip = &snapshot.credential_state_chips[0];
        assert_eq!(chip.state_class, CredentialStateClass::Locked);
        assert_eq!(chip.state_class_token, "locked");
        assert_eq!(
            chip.unavailable_reason,
            Some(CredentialUnavailableReason::StoreLocked)
        );
        assert_eq!(
            chip.unavailable_reason_token.as_deref(),
            Some("store_locked")
        );
        assert_eq!(
            chip.primary_recovery_action,
            RetryPathClass::ResumeAfterCredentialStoreUnlock
        );
        assert!(chip.visible_recovery_required);
        assert!(
            chip.local_work_continues,
            "no-account local path stays usable when the keychain is locked",
        );
        assert!(!chip.plaintext_fallback_allowed);
    }

    #[test]
    fn snapshot_attaches_restored_transcript_record_after_protected_walk() {
        // Protected walk: a local zsh session ran commands in a prior run. After
        // restart the bottom-panel snapshot must surface the prior session as a
        // restored transcript object — never as a live tab — and must preserve
        // auto_rerun_forbidden so the chrome routes a rerun through the
        // fresh-session command id.
        use aureline_terminal::{
            restore_session_as_transcript, RestoredTerminalKind, ScrollbackRedactionClass,
            TerminalRestoreLevel, TerminalScrollback, TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID,
        };

        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.close(&id, "mono:3", Some("user_closed")).unwrap();

        let mut scrollback = TerminalScrollback::new(id.clone());
        scrollback.record_line(
            "$ git status",
            ScrollbackRedactionClass::SupportBundleScoped,
            "mono:2",
        );

        let prior = host.session(&id).expect("session must exist");
        let restored = restore_session_as_transcript(prior, Some(&scrollback), "mono:restart");

        let snapshot = TerminalPaneSnapshot::project("ws-test", &host)
            .with_restored_terminals(vec![restored.clone()]);

        assert!(snapshot.has_restored_terminals());
        assert!(snapshot.has_restored_transcripts());
        let row = snapshot
            .restored_terminals
            .iter()
            .find(|record| record.session_id == id)
            .expect("restored row preserved");
        assert_eq!(row.kind, RestoredTerminalKind::Transcript);
        assert_eq!(
            row.restore_level,
            TerminalRestoreLevel::RestoreUiWithTranscript
        );
        assert!(row.auto_rerun_forbidden);
        assert!(row.fresh_session_required);
        assert_eq!(
            row.open_fresh_session_command_id,
            TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID
        );
    }

    #[test]
    fn snapshot_filters_restored_terminals_to_active_workspace() {
        // The pane must never project a restored row from another workspace.
        use aureline_terminal::{
            restore_session_as_transcript, RestoredTerminalRecord, ScrollbackRedactionClass,
            TerminalScrollback,
        };

        let mut host = PtyHost::new();
        let id_local = open_local(&mut host);
        let id_other = host.open_session(OpenSessionRequest {
            workspace_id: "ws-other",
            host_class: HostClass::HostDesktop,
            display_title: "bash",
            cwd_hint: None,
            execution_context_ref: "execution_context.local_desktop.workspace_root",
            trust_state: TrustState::Trusted,
            observed_at: "mono:0",
        });
        host.close(&id_local, "mono:1", None).unwrap();
        host.close(&id_other, "mono:2", None).unwrap();

        let mut scrollback = TerminalScrollback::new(id_local.clone());
        scrollback.record_line(
            "$ build",
            ScrollbackRedactionClass::SupportBundleScoped,
            "mono:1",
        );

        let prior_local = host.session(&id_local).expect("local session exists");
        let prior_other = host.session(&id_other).expect("other session exists");
        let restored_local =
            restore_session_as_transcript(prior_local, Some(&scrollback), "mono:restart");
        let restored_other = restore_session_as_transcript(prior_other, None, "mono:restart");

        let restored: Vec<RestoredTerminalRecord> = vec![restored_local, restored_other];
        let snapshot =
            TerminalPaneSnapshot::project("ws-test", &host).with_restored_terminals(restored);

        assert_eq!(snapshot.restored_terminals.len(), 1);
        assert_eq!(
            snapshot.restored_terminals[0].workspace_id, "ws-test",
            "restored rows from other workspaces must be filtered"
        );
    }

    #[test]
    fn snapshot_attaches_declined_restore_record_with_typed_reason() {
        // Failure drill: the prior session was quarantined and the runtime
        // declines a transcript restore by policy. The pane projects the
        // declined row so the chrome can disclose the typed reason instead of
        // silently dropping the prior session.
        use aureline_terminal::{
            decline_session_restore, RestoreDeclinedReason, RestoredTerminalKind,
            TerminalRestoreLevel,
        };

        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.quarantine(&id, "mono:3", "terminal_protocol_violation_budget_exceeded")
            .unwrap();

        let prior = host.session(&id).expect("session must exist");
        let declined = decline_session_restore(
            prior,
            RestoreDeclinedReason::DeclinedByPolicy,
            "mono:restart",
        );

        let snapshot =
            TerminalPaneSnapshot::project("ws-test", &host).with_restored_terminals(vec![declined]);

        assert!(snapshot.has_restored_terminals());
        assert!(!snapshot.has_restored_transcripts());
        let row = &snapshot.restored_terminals[0];
        assert_eq!(row.kind, RestoredTerminalKind::Declined);
        assert_eq!(
            row.restore_level,
            TerminalRestoreLevel::RestoreDeclinedByPolicy
        );
        assert!(row.auto_rerun_forbidden);
        assert!(row.fresh_session_required);
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
