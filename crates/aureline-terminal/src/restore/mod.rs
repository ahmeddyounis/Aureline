//! Transcript / ended-session restore projection for terminal sessions.
//!
//! When the live shell relaunches after abnormal termination — or when the
//! user explicitly opts into a restore review — the terminal pane MUST NOT
//! pretend that a prior PTY survived. The restore projection re-opens each
//! prior session as one of:
//!
//! - a **transcript object** (`RestoredTerminalKind::Transcript`), with the
//!   bounded scrollback retained from the previous run, or
//! - an **ended-session object** (`RestoredTerminalKind::EndedSession`),
//!   when the prior run finished cleanly and produced no scrollback worth
//!   reopening, or
//! - a **declined object** (`RestoredTerminalKind::Declined`), when policy,
//!   trust, or a missing execution-context root forbids transcript restore.
//!
//! Every restored record carries `auto_rerun_forbidden: true` and
//! `fresh_session_required: true`. A user who wants live execution opens a
//! fresh session through the command-dispatch boundary; the projection has
//! no path back to live state.
//!
//! ## Vocabulary anchor
//!
//! `restore_level` and `restore_decision` reuse the frozen tokens from
//! `schemas/terminal/session_restore_metadata.schema.json` so the runtime
//! restore-metadata path, the support-export bundle, and this seed quote one
//! truth.

use serde::{Deserialize, Serialize};

use crate::pty_host::{HostClass, PtySession, PtySessionId, SessionLifecycleState};
use crate::scrollback::{TerminalScrollback, TerminalScrollbackSnapshot};

/// Stable record-kind tag carried in serialized restored-terminal records.
pub const RESTORED_TERMINAL_RECORD_KIND: &str = "restored_terminal_record";
/// Schema version for the restored-terminal record family.
pub const RESTORED_TERMINAL_SCHEMA_VERSION: u32 = 1;

/// Stable command id callers route through when the user wants a fresh
/// session in place of a restored transcript.
pub const TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID: &str = "cmd:terminal.open_fresh_session";

/// Frozen kind vocabulary for a restored terminal record.
///
/// `Transcript` and `EndedSession` are read-only objects; the projection
/// never re-opens them as live shells. `Declined` records carry the typed
/// reason a transcript was withheld so the chrome can disclose it instead of
/// silently dropping the prior session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoredTerminalKind {
    /// The prior session left scrollback worth reopening; the record is a
    /// transcript-only object the chrome renders as a closed tab with
    /// retained history.
    Transcript,
    /// The prior session ended cleanly and produced no scrollback worth
    /// reopening. The record is an ended-session object the chrome renders
    /// as a closed tab with provenance only.
    EndedSession,
    /// The transcript was withheld because policy, trust, or a missing
    /// execution-context root forbade the restore.
    Declined,
}

impl RestoredTerminalKind {
    /// Stable string token used in records, fixtures, and a11y exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Transcript => "transcript",
            Self::EndedSession => "ended_session",
            Self::Declined => "declined",
        }
    }

    /// True when the chrome should render a transcript body for the row.
    pub const fn shows_transcript_body(self) -> bool {
        matches!(self, Self::Transcript)
    }
}

/// Frozen restore-level vocabulary mirroring the boundary schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalRestoreLevel {
    /// Header / chrome only; no scrollback was retained.
    RestoreUiOnly,
    /// Header / chrome plus the bounded scrollback transcript.
    RestoreUiWithTranscript,
    /// Header / chrome, transcript, and additional shell-integration hints.
    RestoreUiWithTranscriptAndHints,
    RestoreDeclinedByPolicy,
    RestoreDeclinedByTrust,
    RestoreDeclinedByMissingRoot,
}

impl TerminalRestoreLevel {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreUiOnly => "restore_ui_only",
            Self::RestoreUiWithTranscript => "restore_ui_with_transcript",
            Self::RestoreUiWithTranscriptAndHints => "restore_ui_with_transcript_and_hints",
            Self::RestoreDeclinedByPolicy => "restore_declined_by_policy",
            Self::RestoreDeclinedByTrust => "restore_declined_by_trust",
            Self::RestoreDeclinedByMissingRoot => "restore_declined_by_missing_root",
        }
    }

    /// True when the level represents an honest decline (policy / trust /
    /// missing root) rather than a successful restore.
    pub const fn is_declined(self) -> bool {
        matches!(
            self,
            Self::RestoreDeclinedByPolicy
                | Self::RestoreDeclinedByTrust
                | Self::RestoreDeclinedByMissingRoot
        )
    }
}

/// Frozen restore-decision vocabulary mirroring the boundary schema.
///
/// The seed never returns `RestoreApprovedUserInitiatedFreshSession` from the
/// projection alone; that decision belongs to the command-dispatch boundary
/// after the user explicitly opens a fresh session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalRestoreDecision {
    RestoreApprovedUserInitiatedFreshSession,
    RestoreApprovedEvidenceOnly,
    RestoreDeclined,
    RestoreDeclinedAutomatic,
}

impl TerminalRestoreDecision {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreApprovedUserInitiatedFreshSession => {
                "restore_approved_user_initiated_fresh_session"
            }
            Self::RestoreApprovedEvidenceOnly => "restore_approved_evidence_only",
            Self::RestoreDeclined => "restore_declined",
            Self::RestoreDeclinedAutomatic => "restore_declined_automatic",
        }
    }
}

/// Reason class for declining a transcript restore. Keeps the seed surface
/// honest about why the chrome is rendering a `Declined` record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreDeclinedReason {
    DeclinedByPolicy,
    DeclinedByTrust,
    DeclinedByMissingRoot,
}

impl RestoreDeclinedReason {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeclinedByPolicy => "declined_by_policy",
            Self::DeclinedByTrust => "declined_by_trust",
            Self::DeclinedByMissingRoot => "declined_by_missing_root",
        }
    }

    /// Map onto the corresponding `terminal_restore_level` token.
    pub const fn restore_level(self) -> TerminalRestoreLevel {
        match self {
            Self::DeclinedByPolicy => TerminalRestoreLevel::RestoreDeclinedByPolicy,
            Self::DeclinedByTrust => TerminalRestoreLevel::RestoreDeclinedByTrust,
            Self::DeclinedByMissingRoot => TerminalRestoreLevel::RestoreDeclinedByMissingRoot,
        }
    }
}

/// One restored terminal record. The chrome renders this as a closed-tab
/// transcript object; live execution requires a fresh session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoredTerminalRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub session_id: PtySessionId,
    pub workspace_id: String,
    pub host_class: HostClass,
    pub target_badge: String,
    pub boundary_cue_token: String,
    pub display_title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd_hint: Option<String>,
    pub execution_context_ref: String,
    pub kind: RestoredTerminalKind,
    pub kind_token: String,
    pub restore_level: TerminalRestoreLevel,
    pub restore_level_token: String,
    pub restore_decision: TerminalRestoreDecision,
    pub restore_decision_token: String,
    /// True for every restored record. The chrome MUST NOT offer an implicit
    /// rerun action; the only path back to live execution is
    /// [`TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID`].
    pub auto_rerun_forbidden: bool,
    /// True for every restored record. Surfaces consume this verbatim to
    /// disable any "reuse" affordance and route to the fresh-session command.
    pub fresh_session_required: bool,
    pub open_fresh_session_command_id: String,
    /// Snapshot of the prior session's bounded scrollback. `None` for ended-
    /// session and declined records, or when no scrollback was retained.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript: Option<TerminalScrollbackSnapshot>,
    /// Stable token disclosing the prior lifecycle state. Always one of the
    /// degraded states (`session_closed`, `session_lost_transport`,
    /// `session_quarantined`); the projection never claims a restored record
    /// is interactive.
    pub prior_lifecycle_state_token: String,
    /// Reason a restore was declined. `None` for transcript and ended-session
    /// records.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declined_reason: Option<RestoreDeclinedReason>,
    pub captured_at: String,
}

impl RestoredTerminalRecord {
    /// True when the record carries a retained transcript body.
    pub fn has_transcript(&self) -> bool {
        matches!(self.kind, RestoredTerminalKind::Transcript)
            && self
                .transcript
                .as_ref()
                .map(|snap| snap.retained_line_count() > 0)
                .unwrap_or(false)
    }

    /// True when the record was withheld and carries a typed declined reason.
    pub fn is_declined(&self) -> bool {
        matches!(self.kind, RestoredTerminalKind::Declined)
    }
}

/// Restore the prior session as a transcript or ended-session record.
///
/// `prior_session` MUST be the canonical session row from the host. The
/// caller has typically already transitioned the session to a closed,
/// lost-transport, or quarantined state before invoking the projection.
/// `scrollback` is consumed verbatim; pass `None` when the prior run did not
/// retain any.
pub fn restore_session_as_transcript(
    prior_session: &PtySession,
    scrollback: Option<&TerminalScrollback>,
    captured_at: &str,
) -> RestoredTerminalRecord {
    let snapshot = scrollback.map(|ring| ring.snapshot(captured_at));
    let kind = match snapshot.as_ref() {
        Some(snap) if snap.retained_line_count() > 0 => RestoredTerminalKind::Transcript,
        _ => RestoredTerminalKind::EndedSession,
    };
    let restore_level = match kind {
        RestoredTerminalKind::Transcript => TerminalRestoreLevel::RestoreUiWithTranscript,
        RestoredTerminalKind::EndedSession => TerminalRestoreLevel::RestoreUiOnly,
        RestoredTerminalKind::Declined => unreachable!(),
    };
    build_record(
        prior_session,
        kind,
        restore_level,
        TerminalRestoreDecision::RestoreApprovedEvidenceOnly,
        snapshot,
        None,
        captured_at,
    )
}

/// Decline a transcript restore for the prior session and produce a typed
/// declined record. The caller cites a frozen reason class so the chrome can
/// disclose why the restore was withheld.
pub fn decline_session_restore(
    prior_session: &PtySession,
    reason: RestoreDeclinedReason,
    captured_at: &str,
) -> RestoredTerminalRecord {
    let restore_level = reason.restore_level();
    let restore_decision = TerminalRestoreDecision::RestoreDeclined;
    build_record(
        prior_session,
        RestoredTerminalKind::Declined,
        restore_level,
        restore_decision,
        None,
        Some(reason),
        captured_at,
    )
}

fn build_record(
    prior_session: &PtySession,
    kind: RestoredTerminalKind,
    restore_level: TerminalRestoreLevel,
    restore_decision: TerminalRestoreDecision,
    transcript: Option<TerminalScrollbackSnapshot>,
    declined_reason: Option<RestoreDeclinedReason>,
    captured_at: &str,
) -> RestoredTerminalRecord {
    let header = prior_session.header();
    let prior_state = degraded_lifecycle_token(header.lifecycle_state);
    RestoredTerminalRecord {
        record_kind: RESTORED_TERMINAL_RECORD_KIND.to_owned(),
        schema_version: RESTORED_TERMINAL_SCHEMA_VERSION,
        session_id: header.session_id.clone(),
        workspace_id: header.workspace_id.clone(),
        host_class: header.host_class,
        target_badge: header.target_badge.clone(),
        boundary_cue_token: header.boundary_cue_token.clone(),
        display_title: header.display_title.clone(),
        cwd_hint: header.cwd_hint.clone(),
        execution_context_ref: header.execution_context_ref.clone(),
        kind,
        kind_token: kind.as_str().to_owned(),
        restore_level,
        restore_level_token: restore_level.as_str().to_owned(),
        restore_decision,
        restore_decision_token: restore_decision.as_str().to_owned(),
        auto_rerun_forbidden: true,
        fresh_session_required: true,
        open_fresh_session_command_id: TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID.to_owned(),
        transcript,
        prior_lifecycle_state_token: prior_state.to_owned(),
        declined_reason,
        captured_at: captured_at.to_owned(),
    }
}

/// Map any incoming lifecycle state to the canonical degraded token the
/// restore record discloses. A live or warming state is rewritten to
/// `session_closed` so a restored record never claims to be interactive.
const fn degraded_lifecycle_token(state: SessionLifecycleState) -> &'static str {
    match state {
        SessionLifecycleState::LostTransport => SessionLifecycleState::LostTransport.as_str(),
        SessionLifecycleState::Quarantined => SessionLifecycleState::Quarantined.as_str(),
        SessionLifecycleState::Closed
        | SessionLifecycleState::Requested
        | SessionLifecycleState::Starting
        | SessionLifecycleState::Active
        | SessionLifecycleState::ReconnectedSameIdentity => SessionLifecycleState::Closed.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pty_host::{OpenSessionRequest, PtyHost};
    use crate::scrollback::{ScrollbackBound, ScrollbackRedactionClass, TerminalScrollback};

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
    fn protected_walk_reopens_prior_session_as_transcript() {
        // Protected walk: a local zsh session runs commands, gets closed, and
        // is reopened after a restart. The restored record is a transcript
        // object with retained scrollback, auto-rerun forbidden, and the
        // fresh-session command id wired in.
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
        scrollback.record_line(
            "On branch main",
            ScrollbackRedactionClass::SupportBundleScoped,
            "mono:2",
        );

        let prior = host.session(&id).expect("session must exist");
        let restored = restore_session_as_transcript(prior, Some(&scrollback), "mono:restart");

        assert_eq!(restored.kind, RestoredTerminalKind::Transcript);
        assert_eq!(restored.kind_token, "transcript");
        assert_eq!(
            restored.restore_level,
            TerminalRestoreLevel::RestoreUiWithTranscript
        );
        assert_eq!(restored.restore_level_token, "restore_ui_with_transcript");
        assert_eq!(
            restored.restore_decision,
            TerminalRestoreDecision::RestoreApprovedEvidenceOnly
        );
        assert!(restored.auto_rerun_forbidden);
        assert!(restored.fresh_session_required);
        assert_eq!(
            restored.open_fresh_session_command_id,
            TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID
        );
        assert!(restored.has_transcript());
        let transcript = restored.transcript.as_ref().expect("transcript present");
        assert_eq!(transcript.retained_line_count(), 2);
        assert_eq!(restored.prior_lifecycle_state_token, "session_closed");
    }

    #[test]
    fn failure_drill_lost_transport_becomes_transcript_not_live() {
        // Failure drill: transport drops mid-session. After restart the record
        // returns as a transcript object — never as `session_active` or
        // `session_reconnected_same_identity`. The prior degraded token is
        // preserved verbatim so the chrome can disclose why.
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.mark_lost_transport(&id, "mono:3", Some("network_drop"))
            .unwrap();

        let mut scrollback = TerminalScrollback::with_bound(id.clone(), ScrollbackBound::custom(2));
        scrollback.record_line(
            "$ tail -f log",
            ScrollbackRedactionClass::SupportBundleScoped,
            "mono:2",
        );

        let prior = host.session(&id).expect("session must exist");
        let restored = restore_session_as_transcript(prior, Some(&scrollback), "mono:restart");

        assert_eq!(restored.kind, RestoredTerminalKind::Transcript);
        assert!(restored.auto_rerun_forbidden);
        assert!(restored.fresh_session_required);
        assert_eq!(
            restored.prior_lifecycle_state_token,
            "session_lost_transport"
        );
        assert_eq!(restored.session_id, id);
    }

    #[test]
    fn ended_session_without_scrollback_omits_transcript_body() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.close(&id, "mono:3", Some("user_closed")).unwrap();

        let prior = host.session(&id).expect("session must exist");
        let restored = restore_session_as_transcript(prior, None, "mono:restart");

        assert_eq!(restored.kind, RestoredTerminalKind::EndedSession);
        assert_eq!(restored.kind_token, "ended_session");
        assert_eq!(restored.restore_level, TerminalRestoreLevel::RestoreUiOnly);
        assert!(!restored.has_transcript());
        assert!(restored.auto_rerun_forbidden);
        assert!(restored.fresh_session_required);
    }

    #[test]
    fn declined_record_carries_typed_reason_and_no_transcript() {
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

        assert!(declined.is_declined());
        assert_eq!(declined.kind_token, "declined");
        assert_eq!(
            declined.restore_level,
            TerminalRestoreLevel::RestoreDeclinedByPolicy
        );
        assert!(declined.transcript.is_none());
        assert_eq!(
            declined.declined_reason,
            Some(RestoreDeclinedReason::DeclinedByPolicy)
        );
        assert!(declined.auto_rerun_forbidden);
        assert!(declined.fresh_session_required);
        assert_eq!(declined.prior_lifecycle_state_token, "session_quarantined");
    }

    #[test]
    fn fixture_cases_round_trip_against_seed_projection() {
        // Each fixture under /fixtures/terminal/restore_cases/* describes one
        // restore drill and pins the canonical record the seed must produce.
        // Loading every fixture proves that the seed implementation, the
        // worked JSON examples, and the reviewer-facing contract under
        // /docs/terminal/restore_contract.md stay aligned.
        use serde_json::Value;
        use std::path::Path;

        let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.join("fixtures").join("terminal").join("restore_cases"))
            .expect("derive fixtures dir");

        let cases = [
            "protected_walk_transcript_after_close.json",
            "failure_drill_lost_transport_becomes_transcript.json",
            "declined_by_policy_after_quarantine.json",
        ];

        for case in cases {
            let path = fixtures_dir.join(case);
            let bytes = std::fs::read(&path)
                .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
            let outer: Value = serde_json::from_slice(&bytes)
                .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()));

            let expect = outer
                .get("expect_restored_terminal")
                .unwrap_or_else(|| panic!("fixture {case} missing expect_restored_terminal"));
            let record: RestoredTerminalRecord = serde_json::from_value(expect.clone())
                .unwrap_or_else(|err| panic!("fixture {case} restored record must parse: {err}"));

            assert_eq!(
                record.record_kind, RESTORED_TERMINAL_RECORD_KIND,
                "fixture {case} must carry the canonical record kind"
            );
            assert_eq!(record.schema_version, RESTORED_TERMINAL_SCHEMA_VERSION);
            assert!(
                record.auto_rerun_forbidden,
                "fixture {case} must keep auto_rerun_forbidden=true"
            );
            assert!(
                record.fresh_session_required,
                "fixture {case} must keep fresh_session_required=true"
            );
            assert_eq!(
                record.open_fresh_session_command_id,
                TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID
            );

            // Restored rows never claim a live or recovered-live lifecycle.
            assert_ne!(
                record.prior_lifecycle_state_token, "session_active",
                "fixture {case} must not claim session_active"
            );
            assert_ne!(
                record.prior_lifecycle_state_token, "session_reconnected_same_identity",
                "fixture {case} must not claim session_reconnected_same_identity"
            );

            if record.is_declined() {
                assert!(
                    record.transcript.is_none(),
                    "fixture {case} declined record must omit transcript"
                );
                assert!(
                    record.declined_reason.is_some(),
                    "fixture {case} declined record must cite a typed reason"
                );
                assert!(record.restore_level.is_declined());
            } else {
                assert!(record.declined_reason.is_none());
            }
        }
    }

    #[test]
    fn restored_record_round_trips_via_serde() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.mark_starting(&id, "mono:1").unwrap();
        host.mark_active(&id, "mono:2").unwrap();
        host.close(&id, "mono:3", None).unwrap();

        let mut scrollback = TerminalScrollback::new(id.clone());
        scrollback.record_line(
            "$ build",
            ScrollbackRedactionClass::MetadataAndHashesOnly,
            "mono:2",
        );
        let prior = host.session(&id).expect("session must exist");
        let restored = restore_session_as_transcript(prior, Some(&scrollback), "mono:restart");
        let json = serde_json::to_string(&restored).expect("serialize");
        let round: RestoredTerminalRecord = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round, restored);
        assert!(round.auto_rerun_forbidden);
        assert!(round.fresh_session_required);
    }
}
