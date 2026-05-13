//! Protocol-corpus state and conformance projections for alpha terminal claims.
//!
//! The corpus keeps escape/control sequence coverage, high-risk paste review,
//! clipboard-write policy handling, and restore posture in one terminal-owned
//! vocabulary. Fixture validators and future support exports consume these
//! records so terminal safety does not drift into separate UI-only rules.

use serde::{Deserialize, Serialize};

use crate::headers::{TerminalHeaderRecord, TerminalHeaderRestoreState};
use crate::pty_host::HostClass;
use crate::restore::TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID;

/// Stable record-kind tag for protocol-corpus case fixtures.
pub const TERMINAL_PROTOCOL_CORPUS_CASE_KIND: &str = "terminal_protocol_corpus_alpha_case";
/// Stable record-kind tag for the protocol-corpus fixture manifest.
pub const TERMINAL_PROTOCOL_CORPUS_MANIFEST_KIND: &str = "terminal_protocol_corpus_alpha_manifest";
/// Schema version shared by the protocol-corpus manifest and cases.
pub const TERMINAL_PROTOCOL_CORPUS_SCHEMA_VERSION: u32 = 1;
/// Fixture-set identifier for the first alpha terminal protocol corpus.
pub const TERMINAL_PROTOCOL_CORPUS_FIXTURE_SET_ID: &str = "terminal_protocol_corpus_alpha";
/// Required escape/control sequence tokens for claimed alpha terminal rows.
pub const TERMINAL_ALPHA_REQUIRED_ESCAPE_SEQUENCE_TOKENS: &[&str] = &[
    "utf8_stream",
    "wide_glyph",
    "combining_mark",
    "alternate_screen",
    "mouse_reporting",
    "bracketed_paste_control",
    "hyperlink",
    "truecolor",
    "osc7_cwd",
    "osc133_command_boundary",
];

/// Corpus lane represented by one fixture case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalProtocolCorpusCaseKind {
    /// Escape/control sequence coverage for renderer and PTY parsing claims.
    EscapeControl,
    /// Paste review coverage for multiline, remote, bridge, or production paste.
    PasteReview,
    /// Clipboard-write coverage for OSC 52 and remote bridge writes.
    ClipboardWrite,
    /// Restore-state coverage for ended, reconnect, and transcript rows.
    RestoreConformance,
}

impl TerminalProtocolCorpusCaseKind {
    /// Stable string token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EscapeControl => "escape_control",
            Self::PasteReview => "paste_review",
            Self::ClipboardWrite => "clipboard_write",
            Self::RestoreConformance => "restore_conformance",
        }
    }
}

/// Escape/control sequence fixture input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalEscapeControlInput {
    /// Advertised terminal protocol level for the case.
    pub advertised_protocol_level: String,
    /// Shell family token, for example `bash`, `zsh`, or `powershell`.
    pub shell_family_token: String,
    /// Covered sequence/capability tokens, without raw escape bytes.
    pub covered_sequence_tokens: Vec<String>,
    /// True when the rendered case keeps target or host boundary visible.
    pub boundary_label_visible: bool,
    /// True when the fixture excludes raw PTY escape payload bodies.
    pub raw_escape_payloads_excluded: bool,
}

/// Escape/control sequence conformance report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalEscapeControlReport {
    /// Number of required alpha sequence tokens covered by the input.
    pub covered_required_sequence_count: usize,
    /// True when all required alpha sequence tokens are represented.
    pub covers_required_escape_baseline: bool,
    /// True when the fixture does not embed raw escape payload bytes.
    pub raw_escape_payloads_excluded: bool,
    /// True when the surface keeps the target or host boundary visible.
    pub boundary_label_visible: bool,
}

/// Evaluate one escape/control sequence fixture input.
pub fn evaluate_escape_control(input: &TerminalEscapeControlInput) -> TerminalEscapeControlReport {
    let covered_required_sequence_count = TERMINAL_ALPHA_REQUIRED_ESCAPE_SEQUENCE_TOKENS
        .iter()
        .filter(|token| {
            input
                .covered_sequence_tokens
                .iter()
                .any(|covered| covered == *token)
        })
        .count();
    TerminalEscapeControlReport {
        covered_required_sequence_count,
        covers_required_escape_baseline: covered_required_sequence_count
            == TERMINAL_ALPHA_REQUIRED_ESCAPE_SEQUENCE_TOKENS.len(),
        raw_escape_payloads_excluded: input.raw_escape_payloads_excluded,
        boundary_label_visible: input.boundary_label_visible,
    }
}

/// Policy result shown on a terminal paste review surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalPastePolicyResult {
    /// Policy allows the paste without additional blocking.
    Allowed,
    /// Policy requires a review surface before paste commit.
    ReviewRequired,
    /// Policy blocks the paste.
    Blocked,
}

impl TerminalPastePolicyResult {
    /// Stable string token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::ReviewRequired => "review_required",
            Self::Blocked => "blocked",
        }
    }

    /// True when the policy result requires user-visible review before commit.
    pub const fn requires_review(self) -> bool {
        matches!(self, Self::ReviewRequired | Self::Blocked)
    }
}

/// Submit behavior allowed by a terminal paste path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalPasteSubmitBehavior {
    /// The paste is inserted without submitting to the shell automatically.
    NoAutoSubmit,
    /// The paste path may submit only after an explicit review commit.
    SubmitAfterReview,
}

impl TerminalPasteSubmitBehavior {
    /// Stable string token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAutoSubmit => "no_auto_submit",
            Self::SubmitAfterReview => "submit_after_review",
        }
    }
}

/// Terminal paste review fixture input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalPasteReviewInput {
    /// Stable terminal session reference.
    pub session_id: String,
    /// Host class receiving the paste.
    pub host_class: HostClass,
    /// User-visible target label shown before commit.
    pub target_label: String,
    /// True when the target/host boundary label is visible before commit.
    pub boundary_label_visible: bool,
    /// Number of text lines that would be inserted.
    pub line_count: u32,
    /// True when bracketed paste is available and disclosed.
    pub bracketed_paste_available: bool,
    /// True when the paste crosses a remote clipboard bridge.
    pub remote_clipboard_bridge: bool,
    /// True when the target is labeled as production or equivalent high risk.
    pub production_labeled_target: bool,
    /// Policy result shown by the paste review path.
    pub policy_result: TerminalPastePolicyResult,
    /// Submit behavior for the paste path.
    pub submit_behavior: TerminalPasteSubmitBehavior,
    /// True when a review surface is shown before commit.
    pub review_surface_present: bool,
}

/// Terminal paste review conformance report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalPasteReviewReport {
    /// True when the paste is classified as high risk.
    pub high_risk: bool,
    /// Stable reason tokens explaining why the paste is high risk.
    pub risk_reason_tokens: Vec<String>,
    /// True when target boundary is visible before commit.
    pub target_boundary_visible: bool,
    /// True when policy result is visible before commit.
    pub policy_result_visible: bool,
    /// True when line count is visible before commit.
    pub line_count_visible: bool,
    /// True when the paste cannot commit before review.
    pub review_required_before_commit: bool,
    /// True when the paste cannot submit to the shell automatically.
    pub auto_submit_forbidden: bool,
    /// True when bracketed paste availability is disclosed.
    pub bracketed_paste_visible: bool,
    /// True when a high-risk or blocked paste cannot bypass review.
    pub commit_without_review_forbidden: bool,
}

/// Evaluate one terminal paste review fixture input.
pub fn evaluate_paste_review(input: &TerminalPasteReviewInput) -> TerminalPasteReviewReport {
    let mut risk_reason_tokens = Vec::new();
    if input.line_count > 1 {
        risk_reason_tokens.push("multiline_paste".to_owned());
    }
    if input.remote_clipboard_bridge {
        risk_reason_tokens.push("remote_clipboard_bridge".to_owned());
    }
    if input.production_labeled_target {
        risk_reason_tokens.push("production_labeled_target".to_owned());
    }
    let high_risk = !risk_reason_tokens.is_empty();
    let review_required_before_commit = input.policy_result.requires_review() || high_risk;
    let auto_submit_forbidden = matches!(
        input.submit_behavior,
        TerminalPasteSubmitBehavior::NoAutoSubmit
    );
    TerminalPasteReviewReport {
        high_risk,
        risk_reason_tokens,
        target_boundary_visible: input.boundary_label_visible && !input.target_label.is_empty(),
        policy_result_visible: !input.policy_result.as_str().is_empty(),
        line_count_visible: input.line_count > 0,
        review_required_before_commit,
        auto_submit_forbidden,
        bracketed_paste_visible: input.bracketed_paste_available,
        commit_without_review_forbidden: review_required_before_commit
            && input.review_surface_present
            && auto_submit_forbidden,
    }
}

/// Clipboard-write mechanism represented by a corpus fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalClipboardWriteKind {
    /// OSC 52 or equivalent terminal escape writes to the host clipboard.
    Osc52Write,
    /// Remote clipboard bridge write requested through a remote terminal.
    RemoteClipboardBridgeWrite,
}

impl TerminalClipboardWriteKind {
    /// Stable string token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Osc52Write => "osc52_write",
            Self::RemoteClipboardBridgeWrite => "remote_clipboard_bridge_write",
        }
    }
}

/// Gate disposition for admin policy or workspace trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalGateDisposition {
    /// No gate applies to this case.
    NotRequired,
    /// The gate applies and permits the write.
    Satisfied,
    /// The gate applies and blocks the write.
    Blocked,
}

impl TerminalGateDisposition {
    /// Stable string token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Satisfied => "satisfied",
            Self::Blocked => "blocked",
        }
    }

    /// True when the gate applies to the write path.
    pub const fn is_enforced(self) -> bool {
        !matches!(self, Self::NotRequired)
    }

    /// True when the gate blocks the write path.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// Clipboard-write suppression or admission class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalClipboardSuppressionClass {
    /// The write may proceed after preview.
    AllowedWithPreview,
    /// The write is suppressed by admin policy.
    SuppressedByPolicy,
    /// The write is suppressed by workspace trust.
    SuppressedByTrust,
    /// The write is suppressed because the payload is secret-adjacent.
    SuppressedBySecretClass,
}

impl TerminalClipboardSuppressionClass {
    /// Stable string token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowedWithPreview => "allowed_with_preview",
            Self::SuppressedByPolicy => "suppressed_by_policy",
            Self::SuppressedByTrust => "suppressed_by_trust",
            Self::SuppressedBySecretClass => "suppressed_by_secret_class",
        }
    }
}

/// Clipboard-write fixture input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalClipboardWriteInput {
    /// Clipboard-write mechanism under test.
    pub write_kind: TerminalClipboardWriteKind,
    /// Host class that originated the write.
    pub host_class: HostClass,
    /// User-visible target label shown with the write decision.
    pub target_label: String,
    /// True when the local/remote/container boundary label is visible.
    pub boundary_label_visible: bool,
    /// Admin policy gate disposition for this write.
    pub admin_policy_gate: TerminalGateDisposition,
    /// Workspace trust gate disposition for this write.
    pub trust_gate: TerminalGateDisposition,
    /// True when the payload intersects a secret-adjacent class.
    pub secret_class_detected: bool,
    /// True when an allowed write has a preview before commit.
    pub preview_surface_present: bool,
    /// Metadata-only audit label for the decision.
    pub audit_label_class: String,
    /// True if the fixture embeds raw clipboard payload bytes.
    pub raw_payload_body_present: bool,
}

/// Clipboard-write conformance report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalClipboardWriteReport {
    /// Admission or suppression class selected for the write.
    pub suppression_class: TerminalClipboardSuppressionClass,
    /// Stable token for [`Self::suppression_class`].
    pub suppression_token: String,
    /// True when the write is admitted after preview.
    pub write_admitted: bool,
    /// True when the write is blocked or suppressed.
    pub write_blocked: bool,
    /// True when admin policy was evaluated.
    pub admin_gate_enforced: bool,
    /// True when workspace trust was evaluated.
    pub trust_gate_enforced: bool,
    /// True when the audit label is metadata-only and non-empty.
    pub audit_safe_label_present: bool,
    /// True when no raw clipboard payload is present in the fixture.
    pub raw_payload_excluded: bool,
    /// True when the target boundary is visible with the decision.
    pub boundary_label_visible: bool,
    /// True when an admitted write must show preview before commit.
    pub preview_required: bool,
}

/// Evaluate one clipboard-write fixture input.
pub fn evaluate_clipboard_write(
    input: &TerminalClipboardWriteInput,
) -> TerminalClipboardWriteReport {
    let suppression_class = if input.admin_policy_gate.is_blocked() {
        TerminalClipboardSuppressionClass::SuppressedByPolicy
    } else if input.trust_gate.is_blocked() {
        TerminalClipboardSuppressionClass::SuppressedByTrust
    } else if input.secret_class_detected {
        TerminalClipboardSuppressionClass::SuppressedBySecretClass
    } else {
        TerminalClipboardSuppressionClass::AllowedWithPreview
    };
    let write_admitted = matches!(
        suppression_class,
        TerminalClipboardSuppressionClass::AllowedWithPreview
    ) && input.preview_surface_present;
    TerminalClipboardWriteReport {
        suppression_class,
        suppression_token: suppression_class.as_str().to_owned(),
        write_admitted,
        write_blocked: !write_admitted,
        admin_gate_enforced: input.admin_policy_gate.is_enforced(),
        trust_gate_enforced: input.trust_gate.is_enforced(),
        audit_safe_label_present: !input.audit_label_class.is_empty()
            && !input.raw_payload_body_present,
        raw_payload_excluded: !input.raw_payload_body_present,
        boundary_label_visible: input.boundary_label_visible && !input.target_label.is_empty(),
        preview_required: write_admitted,
    }
}

/// Restore conformance state projected from a terminal header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalRestoreConformanceState {
    /// The terminal row is live and may accept input.
    Live,
    /// The terminal row is warming and not yet input-ready.
    Warming,
    /// The prior session ended and needs a fresh session for execution.
    Ended,
    /// The target requires explicit reconnect before execution.
    ReconnectRequired,
    /// The row is a restored transcript and is evidence-only.
    RestoredTranscript,
    /// Restore or terminal authority is blocked.
    RestoreBlocked,
    /// The row is inspect-only.
    InspectOnly,
}

impl TerminalRestoreConformanceState {
    /// Stable string token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Warming => "warming",
            Self::Ended => "ended",
            Self::ReconnectRequired => "reconnect_required",
            Self::RestoredTranscript => "restored_transcript",
            Self::RestoreBlocked => "restore_blocked",
            Self::InspectOnly => "inspect_only",
        }
    }
}

/// Restore conformance report projected from one terminal header record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalRestoreConformanceReport {
    /// Restore state selected for conformance.
    pub state: TerminalRestoreConformanceState,
    /// Stable token for [`Self::state`].
    pub state_token: String,
    /// True when execution requires explicit reconnect.
    pub reconnect_required: bool,
    /// True when the row is a restored transcript.
    pub transcript_restored: bool,
    /// True when an ended row requires a fresh-session command path.
    pub ended_requires_fresh_session: bool,
    /// True when restore cannot auto-rerun prior commands.
    pub auto_rerun_forbidden: bool,
    /// True when live execution must enter through a fresh-session command.
    pub fresh_session_required: bool,
    /// True when the fresh-session command id is visible on the record.
    pub open_fresh_session_command_visible: bool,
    /// True when the terminal boundary cue remains visible.
    pub boundary_cue_visible: bool,
}

/// Project restore conformance from a terminal header record.
pub fn restore_conformance_from_header(
    header: &TerminalHeaderRecord,
) -> TerminalRestoreConformanceReport {
    let state = match header.restore_state {
        TerminalHeaderRestoreState::Live => TerminalRestoreConformanceState::Live,
        TerminalHeaderRestoreState::Warming => TerminalRestoreConformanceState::Warming,
        TerminalHeaderRestoreState::TranscriptRestored => {
            TerminalRestoreConformanceState::RestoredTranscript
        }
        TerminalHeaderRestoreState::CommandRerunRequired => TerminalRestoreConformanceState::Ended,
        TerminalHeaderRestoreState::ReconnectRequired => {
            TerminalRestoreConformanceState::ReconnectRequired
        }
        TerminalHeaderRestoreState::InspectOnly => TerminalRestoreConformanceState::InspectOnly,
        TerminalHeaderRestoreState::RestoreBlocked => {
            TerminalRestoreConformanceState::RestoreBlocked
        }
    };
    TerminalRestoreConformanceReport {
        state,
        state_token: state.as_str().to_owned(),
        reconnect_required: matches!(state, TerminalRestoreConformanceState::ReconnectRequired),
        transcript_restored: matches!(state, TerminalRestoreConformanceState::RestoredTranscript),
        ended_requires_fresh_session: matches!(state, TerminalRestoreConformanceState::Ended)
            && header.fresh_session_required,
        auto_rerun_forbidden: header.auto_rerun_forbidden,
        fresh_session_required: header.fresh_session_required,
        open_fresh_session_command_visible: header.open_fresh_session_command_id.as_deref()
            == Some(TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID),
        boundary_cue_visible: header.boundary_cue_visible,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_risk_paste_exposes_boundary_policy_line_count_and_no_submit() {
        let report = evaluate_paste_review(&TerminalPasteReviewInput {
            session_id: "pty:ws|remote_agent_primary|1".to_owned(),
            host_class: HostClass::RemoteAgentPrimary,
            target_label: "prod-shell".to_owned(),
            boundary_label_visible: true,
            line_count: 14,
            bracketed_paste_available: true,
            remote_clipboard_bridge: false,
            production_labeled_target: true,
            policy_result: TerminalPastePolicyResult::ReviewRequired,
            submit_behavior: TerminalPasteSubmitBehavior::NoAutoSubmit,
            review_surface_present: true,
        });

        assert!(report.high_risk);
        assert!(report.target_boundary_visible);
        assert!(report.policy_result_visible);
        assert!(report.line_count_visible);
        assert!(report.review_required_before_commit);
        assert!(report.auto_submit_forbidden);
        assert!(report.commit_without_review_forbidden);
    }

    #[test]
    fn policy_blocked_clipboard_write_keeps_gates_and_audit_label() {
        let report = evaluate_clipboard_write(&TerminalClipboardWriteInput {
            write_kind: TerminalClipboardWriteKind::Osc52Write,
            host_class: HostClass::RemoteAgentPrimary,
            target_label: "prod-shell".to_owned(),
            boundary_label_visible: true,
            admin_policy_gate: TerminalGateDisposition::Blocked,
            trust_gate: TerminalGateDisposition::Blocked,
            secret_class_detected: false,
            preview_surface_present: false,
            audit_label_class: "clipboard_write_blocked_policy_metadata_only".to_owned(),
            raw_payload_body_present: false,
        });

        assert_eq!(
            report.suppression_class,
            TerminalClipboardSuppressionClass::SuppressedByPolicy
        );
        assert!(report.write_blocked);
        assert!(report.admin_gate_enforced);
        assert!(report.trust_gate_enforced);
        assert!(report.audit_safe_label_present);
        assert!(report.boundary_label_visible);
    }
}
