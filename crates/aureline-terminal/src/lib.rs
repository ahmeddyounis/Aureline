//! Terminal foundation: the PTY host abstraction and the canonical
//! terminal-session truth model.
//!
//! This crate owns:
//!
//! - one [`pty_host::PtyHost`] abstraction that manages every terminal session
//!   and the local PTY process/runtime behind it,
//!   and
//! - one [`pty_host::SessionHeader`] vocabulary that carries title, cwd hint,
//!   target identity, execution-context reference, trust posture, and
//!   local-vs-managed boundary cue — the same provenance tuple a tab/pane chip
//!   shows in the bottom panel and that a support export quotes verbatim.

#![doc(html_root_url = "https://docs.rs/aureline-terminal/0.0.0")]

/// Terminal header strip and target/cwd/runtime/restore chip projection.
pub mod headers;
/// Terminal protocol corpus and conformance projections for escape handling,
/// paste review, clipboard writes, and restore-state proofs.
pub mod protocol_corpus;
pub mod pty_host;
/// Transcript / ended-session restore projection. Restored records never
/// claim a live shell survived; auto-rerun is always forbidden.
pub mod restore;
/// Bounded, redaction-aware scrollback ring used by transcript restore and
/// support / export bundles.
pub mod scrollback;

pub use headers::{
    TerminalHeaderChip, TerminalHeaderChipKind, TerminalHeaderChipState, TerminalHeaderRecord,
    TerminalHeaderRestoreState, TerminalHeaderSourceKind, TerminalRuntimeChipSource,
    TERMINAL_HEADER_RECORD_KIND, TERMINAL_HEADER_SCHEMA_VERSION,
};
pub use portable_pty::PtySize;
pub use protocol_corpus::{
    evaluate_clipboard_write, evaluate_escape_control, evaluate_paste_review,
    restore_conformance_from_header, TerminalClipboardSuppressionClass,
    TerminalClipboardWriteInput, TerminalClipboardWriteKind, TerminalClipboardWriteReport,
    TerminalEscapeControlInput, TerminalEscapeControlReport, TerminalGateDisposition,
    TerminalPastePolicyResult, TerminalPasteReviewInput, TerminalPasteReviewReport,
    TerminalPasteSubmitBehavior, TerminalProtocolCorpusCaseKind, TerminalRestoreConformanceReport,
    TerminalRestoreConformanceState, TERMINAL_ALPHA_REQUIRED_ESCAPE_SEQUENCE_TOKENS,
    TERMINAL_PROTOCOL_CORPUS_CASE_KIND, TERMINAL_PROTOCOL_CORPUS_FIXTURE_SET_ID,
    TERMINAL_PROTOCOL_CORPUS_MANIFEST_KIND, TERMINAL_PROTOCOL_CORPUS_SCHEMA_VERSION,
};
pub use pty_host::{
    HostClass, OpenSessionRequest, PtyCommand, PtyHost, PtyHostError, PtyLaunchFailureReason,
    PtyOutputDrain, PtySession, PtySessionId, SessionHeader, SessionLifecycleState,
    SessionLifecycleTransition, TerminalTrustState, DEFAULT_PTY_OUTPUT_RING_CAPACITY,
    DEFAULT_PTY_SIZE,
};
pub use restore::{
    decline_session_restore, restore_session_as_transcript, RestoreDeclinedReason,
    RestoredTerminalKind, RestoredTerminalRecord, TerminalRestoreDecision, TerminalRestoreLevel,
    RESTORED_TERMINAL_RECORD_KIND, RESTORED_TERMINAL_SCHEMA_VERSION,
    TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID,
};
pub use scrollback::{
    ScrollbackBound, ScrollbackLineRecord, ScrollbackRedactionClass, TerminalScrollback,
    TerminalScrollbackSnapshot, DEFAULT_SCROLLBACK_LINE_BOUND, SCROLLBACK_LINE_RECORD_KIND,
    SCROLLBACK_SCHEMA_VERSION, SCROLLBACK_SNAPSHOT_RECORD_KIND,
};
