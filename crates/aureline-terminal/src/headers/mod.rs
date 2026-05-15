//! Terminal header strip and chip projection.
//!
//! The header strip is the canonical terminal-owned record that bottom-panel
//! tabs, restored transcript rows, title-context bars, and support exports can
//! quote without re-deriving target, cwd, runtime, or restore posture. It
//! projects the existing [`crate::pty_host::SessionHeader`] and
//! [`crate::restore::RestoredTerminalRecord`] records into one stable chip set.

use serde::{Deserialize, Serialize};

use crate::pty_host::{
    HostClass, PtySession, PtySessionId, SessionLifecycleState, TerminalSessionRestoreMetadata,
};
use crate::restore::{
    RestoredTerminalKind, RestoredTerminalRecord, TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID,
};

/// Stable record-kind tag for [`TerminalHeaderRecord`] payloads.
pub const TERMINAL_HEADER_RECORD_KIND: &str = "terminal_header_record";
/// Schema version for terminal header records.
pub const TERMINAL_HEADER_SCHEMA_VERSION: u32 = 1;

/// Source family projected into a terminal header strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalHeaderSourceKind {
    /// A live or recently live PTY session row from the host.
    LiveSession,
    /// A restored transcript, ended-session, or declined restore row.
    RestoredTerminal,
}

impl TerminalHeaderSourceKind {
    /// Stable string token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveSession => "live_session",
            Self::RestoredTerminal => "restored_terminal",
        }
    }
}

/// Chip role in the terminal header strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalHeaderChipKind {
    /// Target / host boundary chip.
    Target,
    /// Current or last-known working-directory chip.
    Cwd,
    /// Runtime / toolchain chip projected from the shared execution context.
    Runtime,
    /// Live, transcript, rerun, reconnect, or blocked restore posture chip.
    Restore,
}

impl TerminalHeaderChipKind {
    /// Stable string token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Target => "target",
            Self::Cwd => "cwd",
            Self::Runtime => "runtime",
            Self::Restore => "restore",
        }
    }
}

/// Visible state of one header chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalHeaderChipState {
    /// The chip is current enough for the active row.
    Current,
    /// The chip is last-known metadata and should not imply live authority.
    LastKnown,
    /// The chip is missing because the owning signal or context is absent.
    Missing,
    /// The row is warming and not ready for input yet.
    Warming,
    /// The chip is present but narrowed by confidence, policy, or drift.
    Degraded,
    /// The terminal row needs an explicit reconnect before live authority.
    ReconnectRequired,
    /// Live execution requires a new command-dispatch path.
    CommandRerunRequired,
    /// The row is an inspect-only restored transcript.
    TranscriptRestored,
    /// The row is inspect-only and cannot admit input.
    InspectOnly,
    /// Policy or trust blocks live terminal authority.
    PolicyBlocked,
}

impl TerminalHeaderChipState {
    /// Stable string token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::LastKnown => "last_known",
            Self::Missing => "missing",
            Self::Warming => "warming",
            Self::Degraded => "degraded",
            Self::ReconnectRequired => "reconnect_required",
            Self::CommandRerunRequired => "command_rerun_required",
            Self::TranscriptRestored => "transcript_restored",
            Self::InspectOnly => "inspect_only",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Restore and live-authority posture for the whole terminal header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalHeaderRestoreState {
    /// The session is live and can accept input under its trust posture.
    Live,
    /// Host is preparing a PTY or remote attach.
    Warming,
    /// A prior transcript is restored as evidence only.
    TranscriptRestored,
    /// The previous session ended and live execution requires a fresh command.
    CommandRerunRequired,
    /// Transport or target reachability requires explicit reconnect.
    ReconnectRequired,
    /// The restored row is inspect-only without live authority.
    InspectOnly,
    /// Restore or live authority is blocked by policy, trust, or quarantine.
    RestoreBlocked,
}

impl TerminalHeaderRestoreState {
    /// Stable string token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Warming => "warming",
            Self::TranscriptRestored => "transcript_restored",
            Self::CommandRerunRequired => "command_rerun_required",
            Self::ReconnectRequired => "reconnect_required",
            Self::InspectOnly => "inspect_only",
            Self::RestoreBlocked => "restore_blocked",
        }
    }
}

/// Runtime chip input projected from the shared execution-context summary.
///
/// This crate keeps the source shape string-token based so it does not depend
/// on `aureline-runtime`; the shell consumer fills these fields from
/// `RunContextSummary`, which itself is projected from
/// `aureline_runtime::ExecutionContext`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalRuntimeChipSource {
    /// Execution-context id that owns the runtime/toolchain decision.
    pub execution_context_ref: String,
    /// Runtime surface token, for example `terminal` or `task`.
    pub surface_token: String,
    /// Shared execution-context target class token.
    pub target_class_token: String,
    /// Shared execution-context toolchain class token.
    pub toolchain_class_token: String,
    /// Shared execution-context toolchain id.
    pub toolchain_id: String,
    /// Shared execution-context resolved version token.
    pub resolved_version: String,
    /// Shared target-confidence level token.
    pub target_confidence_level_token: String,
    /// Shared prebuild reuse-state token.
    pub prebuild_reuse_state_token: String,
    /// Shared mixed-version drift-state token.
    pub mixed_version_state_token: String,
}

/// One visible chip in the terminal header strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalHeaderChip {
    /// Chip role.
    pub kind: TerminalHeaderChipKind,
    /// Stable token for [`Self::kind`].
    pub kind_token: String,
    /// Short label the chrome can render before the value.
    pub label: String,
    /// Stable value token for comparison and support export.
    pub value_token: String,
    /// Human-readable value for the tab/header surface.
    pub display_value: String,
    /// Visible chip state.
    pub state: TerminalHeaderChipState,
    /// Stable token for [`Self::state`].
    pub state_token: String,
    /// Owning source ref, usually the execution-context id or restore record.
    pub source_ref: String,
}

impl TerminalHeaderChip {
    fn new(
        kind: TerminalHeaderChipKind,
        label: &str,
        value_token: impl Into<String>,
        display_value: impl Into<String>,
        state: TerminalHeaderChipState,
        source_ref: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            kind_token: kind.as_str().to_owned(),
            label: label.to_owned(),
            value_token: value_token.into(),
            display_value: display_value.into(),
            state,
            state_token: state.as_str().to_owned(),
            source_ref: source_ref.into(),
        }
    }
}

/// Canonical target/cwd/runtime/restore header record for a terminal row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalHeaderRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub source_kind: TerminalHeaderSourceKind,
    pub source_kind_token: String,
    pub session_id: PtySessionId,
    pub workspace_id: String,
    pub host_class: HostClass,
    pub display_title: String,
    pub execution_context_ref: String,
    /// Restore metadata projected from the terminal session or restored row.
    /// This is evidence-only and never carries raw command or environment
    /// bodies.
    #[serde(default)]
    pub restore_metadata: TerminalSessionRestoreMetadata,
    pub target_chip: TerminalHeaderChip,
    pub cwd_chip: TerminalHeaderChip,
    pub runtime_chip: TerminalHeaderChip,
    pub restore_chip: TerminalHeaderChip,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_source: Option<TerminalRuntimeChipSource>,
    pub restore_state: TerminalHeaderRestoreState,
    pub restore_state_token: String,
    pub boundary_cue_token: String,
    pub boundary_cue_visible: bool,
    pub lifecycle_state_token: String,
    pub auto_rerun_forbidden: bool,
    pub fresh_session_required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_fresh_session_command_id: Option<String>,
    pub captured_at: String,
}

impl TerminalHeaderRecord {
    /// Project a header strip from one live or retained PTY session row.
    pub fn project_session(session: &PtySession) -> Self {
        let header = session.header();
        let restore_state = restore_state_for_lifecycle(header.lifecycle_state);
        let restore_chip_state = restore_chip_state_for_lifecycle(header.lifecycle_state);
        let source_ref = header.execution_context_ref.clone();
        let cwd = cwd_chip(header.cwd_hint.as_deref(), &source_ref, false);
        let target_chip = TerminalHeaderChip::new(
            TerminalHeaderChipKind::Target,
            "Target",
            header.host_class.as_str(),
            &header.target_badge,
            if header.lifecycle_state.is_degraded() {
                TerminalHeaderChipState::LastKnown
            } else {
                TerminalHeaderChipState::Current
            },
            &source_ref,
        );
        let restore_chip = TerminalHeaderChip::new(
            TerminalHeaderChipKind::Restore,
            "State",
            restore_state.as_str(),
            restore_display_value(restore_state),
            restore_chip_state,
            &source_ref,
        );
        Self {
            record_kind: TERMINAL_HEADER_RECORD_KIND.to_owned(),
            schema_version: TERMINAL_HEADER_SCHEMA_VERSION,
            source_kind: TerminalHeaderSourceKind::LiveSession,
            source_kind_token: TerminalHeaderSourceKind::LiveSession.as_str().to_owned(),
            session_id: header.session_id.clone(),
            workspace_id: header.workspace_id.clone(),
            host_class: header.host_class,
            display_title: header.display_title.clone(),
            execution_context_ref: source_ref.clone(),
            restore_metadata: header.restore_metadata.clone(),
            target_chip,
            cwd_chip: cwd,
            runtime_chip: runtime_missing_chip(&source_ref),
            restore_chip,
            runtime_source: None,
            restore_state,
            restore_state_token: restore_state.as_str().to_owned(),
            boundary_cue_token: header.boundary_cue_token.clone(),
            boundary_cue_visible: header.host_class.needs_boundary_cue(),
            lifecycle_state_token: header.lifecycle_state_token.clone(),
            auto_rerun_forbidden: true,
            fresh_session_required: matches!(
                restore_state,
                TerminalHeaderRestoreState::CommandRerunRequired
                    | TerminalHeaderRestoreState::RestoreBlocked
            ),
            open_fresh_session_command_id: matches!(
                restore_state,
                TerminalHeaderRestoreState::CommandRerunRequired
                    | TerminalHeaderRestoreState::RestoreBlocked
            )
            .then(|| TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID.to_owned()),
            captured_at: header.last_observed_at.clone(),
        }
    }

    /// Project a header strip from one restored transcript / ended-session
    /// record.
    pub fn project_restored(record: &RestoredTerminalRecord) -> Self {
        let restore_state = restore_state_for_restored(record);
        let restore_chip_state = restore_chip_state_for_restored(record);
        let source_ref = record.execution_context_ref.clone();
        let target_chip_state = if matches!(record.kind, RestoredTerminalKind::Declined) {
            TerminalHeaderChipState::InspectOnly
        } else {
            TerminalHeaderChipState::LastKnown
        };
        let target_chip = TerminalHeaderChip::new(
            TerminalHeaderChipKind::Target,
            "Target",
            record.host_class.as_str(),
            &record.target_badge,
            target_chip_state,
            &source_ref,
        );
        let restore_chip = TerminalHeaderChip::new(
            TerminalHeaderChipKind::Restore,
            "State",
            restore_state.as_str(),
            restore_display_value(restore_state),
            restore_chip_state,
            &source_ref,
        );
        Self {
            record_kind: TERMINAL_HEADER_RECORD_KIND.to_owned(),
            schema_version: TERMINAL_HEADER_SCHEMA_VERSION,
            source_kind: TerminalHeaderSourceKind::RestoredTerminal,
            source_kind_token: TerminalHeaderSourceKind::RestoredTerminal
                .as_str()
                .to_owned(),
            session_id: record.session_id.clone(),
            workspace_id: record.workspace_id.clone(),
            host_class: record.host_class,
            display_title: record.display_title.clone(),
            execution_context_ref: source_ref.clone(),
            restore_metadata: record.restore_metadata.clone(),
            target_chip,
            cwd_chip: cwd_chip(record.cwd_hint.as_deref(), &source_ref, true),
            runtime_chip: runtime_missing_chip(&source_ref),
            restore_chip,
            runtime_source: None,
            restore_state,
            restore_state_token: restore_state.as_str().to_owned(),
            boundary_cue_token: record.boundary_cue_token.clone(),
            boundary_cue_visible: record.host_class.needs_boundary_cue(),
            lifecycle_state_token: record.prior_lifecycle_state_token.clone(),
            auto_rerun_forbidden: record.auto_rerun_forbidden,
            fresh_session_required: record.fresh_session_required,
            open_fresh_session_command_id: Some(record.open_fresh_session_command_id.clone()),
            captured_at: record.captured_at.clone(),
        }
    }

    /// Attach runtime/toolchain chip truth from the shared execution-context
    /// projection.
    pub fn with_runtime_source(mut self, source: TerminalRuntimeChipSource) -> Self {
        if source.execution_context_ref == self.execution_context_ref {
            self.runtime_chip = runtime_chip_from_source(&source);
            self.runtime_source = Some(source);
        }
        self
    }

    /// True when the header is inspect-only evidence rather than a live input
    /// surface.
    pub fn is_inspect_only(&self) -> bool {
        matches!(
            self.restore_state,
            TerminalHeaderRestoreState::TranscriptRestored
                | TerminalHeaderRestoreState::InspectOnly
                | TerminalHeaderRestoreState::RestoreBlocked
        )
    }
}

fn cwd_chip(cwd_hint: Option<&str>, source_ref: &str, restored: bool) -> TerminalHeaderChip {
    match cwd_hint {
        Some(cwd) => TerminalHeaderChip::new(
            TerminalHeaderChipKind::Cwd,
            "Cwd",
            cwd,
            cwd,
            if restored {
                TerminalHeaderChipState::LastKnown
            } else {
                TerminalHeaderChipState::Current
            },
            source_ref,
        ),
        None => TerminalHeaderChip::new(
            TerminalHeaderChipKind::Cwd,
            "Cwd",
            "cwd_unavailable",
            "cwd unavailable",
            TerminalHeaderChipState::Missing,
            source_ref,
        ),
    }
}

fn runtime_missing_chip(source_ref: &str) -> TerminalHeaderChip {
    TerminalHeaderChip::new(
        TerminalHeaderChipKind::Runtime,
        "Runtime",
        "runtime_unresolved",
        "runtime unresolved",
        TerminalHeaderChipState::Missing,
        source_ref,
    )
}

fn runtime_chip_from_source(source: &TerminalRuntimeChipSource) -> TerminalHeaderChip {
    let value_token = format!(
        "{class}:{id}:{version}",
        class = source.toolchain_class_token,
        id = source.toolchain_id,
        version = source.resolved_version,
    );
    let display_value = format!(
        "{class} {version}",
        class = source.toolchain_class_token,
        version = source.resolved_version,
    );
    TerminalHeaderChip::new(
        TerminalHeaderChipKind::Runtime,
        "Runtime",
        value_token,
        display_value,
        runtime_state_for_source(source),
        &source.execution_context_ref,
    )
}

fn runtime_state_for_source(source: &TerminalRuntimeChipSource) -> TerminalHeaderChipState {
    if source.target_confidence_level_token == "low"
        || source.mixed_version_state_token == "drift_detected"
        || source.prebuild_reuse_state_token.starts_with("rejected_")
    {
        TerminalHeaderChipState::Degraded
    } else {
        TerminalHeaderChipState::Current
    }
}

const fn restore_state_for_lifecycle(state: SessionLifecycleState) -> TerminalHeaderRestoreState {
    match state {
        SessionLifecycleState::Requested | SessionLifecycleState::Starting => {
            TerminalHeaderRestoreState::Warming
        }
        SessionLifecycleState::Active | SessionLifecycleState::ReconnectedSameIdentity => {
            TerminalHeaderRestoreState::Live
        }
        SessionLifecycleState::LostTransport => TerminalHeaderRestoreState::ReconnectRequired,
        SessionLifecycleState::Closed => TerminalHeaderRestoreState::CommandRerunRequired,
        SessionLifecycleState::Quarantined => TerminalHeaderRestoreState::RestoreBlocked,
    }
}

const fn restore_chip_state_for_lifecycle(state: SessionLifecycleState) -> TerminalHeaderChipState {
    match state {
        SessionLifecycleState::Requested | SessionLifecycleState::Starting => {
            TerminalHeaderChipState::Warming
        }
        SessionLifecycleState::Active | SessionLifecycleState::ReconnectedSameIdentity => {
            TerminalHeaderChipState::Current
        }
        SessionLifecycleState::LostTransport => TerminalHeaderChipState::ReconnectRequired,
        SessionLifecycleState::Closed => TerminalHeaderChipState::CommandRerunRequired,
        SessionLifecycleState::Quarantined => TerminalHeaderChipState::PolicyBlocked,
    }
}

const fn restore_state_for_restored(record: &RestoredTerminalRecord) -> TerminalHeaderRestoreState {
    match record.kind {
        RestoredTerminalKind::Transcript => TerminalHeaderRestoreState::TranscriptRestored,
        RestoredTerminalKind::EndedSession => TerminalHeaderRestoreState::CommandRerunRequired,
        RestoredTerminalKind::Declined => TerminalHeaderRestoreState::RestoreBlocked,
    }
}

const fn restore_chip_state_for_restored(
    record: &RestoredTerminalRecord,
) -> TerminalHeaderChipState {
    match record.kind {
        RestoredTerminalKind::Transcript => TerminalHeaderChipState::TranscriptRestored,
        RestoredTerminalKind::EndedSession => TerminalHeaderChipState::CommandRerunRequired,
        RestoredTerminalKind::Declined => TerminalHeaderChipState::PolicyBlocked,
    }
}

const fn restore_display_value(state: TerminalHeaderRestoreState) -> &'static str {
    match state {
        TerminalHeaderRestoreState::Live => "Live",
        TerminalHeaderRestoreState::Warming => "Warming",
        TerminalHeaderRestoreState::TranscriptRestored => "Transcript restored",
        TerminalHeaderRestoreState::CommandRerunRequired => "Rerun required",
        TerminalHeaderRestoreState::ReconnectRequired => "Reconnect required",
        TerminalHeaderRestoreState::InspectOnly => "Inspect only",
        TerminalHeaderRestoreState::RestoreBlocked => "Restore blocked",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pty_host::{
        OpenSessionRequest, PtyHost, TerminalEnvironmentScope, TerminalLastCommandClass,
    };
    use crate::restore::{decline_session_restore, restore_session_as_transcript};
    use crate::scrollback::{ScrollbackRedactionClass, TerminalScrollback};

    use aureline_workspace::TrustState;

    fn open_local(host: &mut PtyHost) -> PtySessionId {
        host.open_session(OpenSessionRequest {
            workspace_id: "ws-test",
            host_class: HostClass::HostDesktop,
            display_title: "zsh",
            cwd_hint: Some("~/code/aureline"),
            execution_context_ref: "exec:ws-test:terminal:0",
            trust_state: TrustState::Trusted,
            observed_at: "mono:0",
        })
    }

    fn runtime_source() -> TerminalRuntimeChipSource {
        TerminalRuntimeChipSource {
            execution_context_ref: "exec:ws-test:terminal:0".to_owned(),
            surface_token: "terminal".to_owned(),
            target_class_token: "local_host".to_owned(),
            toolchain_class_token: "login_shell".to_owned(),
            toolchain_id: "shell.login_shell".to_owned(),
            resolved_version: "seed".to_owned(),
            target_confidence_level_token: "high".to_owned(),
            prebuild_reuse_state_token: "not_applicable".to_owned(),
            mixed_version_state_token: "not_applicable".to_owned(),
        }
    }

    #[test]
    fn live_session_header_shows_target_cwd_runtime_and_live_state() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        let session = host.session(&id).expect("session exists");
        let header =
            TerminalHeaderRecord::project_session(session).with_runtime_source(runtime_source());

        assert_eq!(header.record_kind, TERMINAL_HEADER_RECORD_KIND);
        assert_eq!(header.target_chip.display_value, "Local");
        assert_eq!(header.cwd_chip.display_value, "~/code/aureline");
        assert_eq!(
            header.runtime_chip.value_token,
            "login_shell:shell.login_shell:seed"
        );
        assert_eq!(header.restore_state, TerminalHeaderRestoreState::Live);
        assert_eq!(header.restore_chip.state, TerminalHeaderChipState::Current);
        assert_eq!(
            header.restore_metadata.working_directory.as_deref(),
            Some("~/code/aureline")
        );
        assert_eq!(
            header.restore_metadata.environment_scope,
            TerminalEnvironmentScope::Workspace
        );
        assert!(!header.fresh_session_required);
        assert!(header.open_fresh_session_command_id.is_none());
    }

    #[test]
    fn lost_transport_header_distinguishes_reconnect_from_rerun() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_lost_transport(&id, "mono:1", Some("network_drop"))
            .unwrap();
        let session = host.session(&id).expect("session exists");
        let header =
            TerminalHeaderRecord::project_session(session).with_runtime_source(runtime_source());

        assert_eq!(
            header.restore_state,
            TerminalHeaderRestoreState::ReconnectRequired
        );
        assert_eq!(
            header.restore_chip.state,
            TerminalHeaderChipState::ReconnectRequired
        );
        assert!(!header.fresh_session_required);
        assert!(header.open_fresh_session_command_id.is_none());
    }

    #[test]
    fn restored_transcript_header_is_inspect_only_not_rerun_or_reconnect() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.update_last_command_class(&id, TerminalLastCommandClass::VersionControl, "mono:0.5")
            .unwrap();
        host.close(&id, "mono:1", Some("user_closed")).unwrap();

        let mut scrollback = TerminalScrollback::new(id.clone());
        scrollback.record_line(
            "$ git status",
            ScrollbackRedactionClass::SupportBundleScoped,
            "mono:0",
        );
        let prior = host.session(&id).expect("session exists");
        let restored = restore_session_as_transcript(prior, Some(&scrollback), "mono:restart");
        let header =
            TerminalHeaderRecord::project_restored(&restored).with_runtime_source(runtime_source());

        assert_eq!(
            header.restore_state,
            TerminalHeaderRestoreState::TranscriptRestored
        );
        assert_eq!(
            header.restore_chip.state,
            TerminalHeaderChipState::TranscriptRestored
        );
        assert!(header.auto_rerun_forbidden);
        assert!(header.fresh_session_required);
        assert_eq!(
            header.restore_metadata.last_command_class,
            TerminalLastCommandClass::VersionControl
        );
        assert_eq!(
            header.open_fresh_session_command_id.as_deref(),
            Some(TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID)
        );
        assert!(header.is_inspect_only());
    }

    #[test]
    fn declined_restore_header_stays_blocked_with_last_known_chips() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.quarantine(&id, "mono:1", "terminal_protocol_violation_budget_exceeded")
            .unwrap();
        let prior = host.session(&id).expect("session exists");
        let declined = decline_session_restore(
            prior,
            crate::RestoreDeclinedReason::DeclinedByPolicy,
            "mono:restart",
        );
        let header = TerminalHeaderRecord::project_restored(&declined);

        assert_eq!(
            header.restore_state,
            TerminalHeaderRestoreState::RestoreBlocked
        );
        assert_eq!(
            header.target_chip.state,
            TerminalHeaderChipState::InspectOnly
        );
        assert_eq!(header.cwd_chip.state, TerminalHeaderChipState::LastKnown);
        assert!(header.fresh_session_required);
        assert!(header.is_inspect_only());
    }
}
