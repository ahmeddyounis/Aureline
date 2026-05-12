//! Terminal foundation: the PTY host abstraction and the canonical
//! terminal-session truth model.
//!
//! This crate is the M1 seed for the terminal lane. It owns:
//!
//! - one [`pty_host::PtyHost`] abstraction that manages every terminal session
//!   instead of letting callers embed shell launches directly in the UI thread,
//!   and
//! - one [`pty_host::SessionHeader`] vocabulary that carries title, cwd hint,
//!   target identity, execution-context reference, trust posture, and
//!   local-vs-managed boundary cue — the same provenance tuple a tab/pane chip
//!   shows in the bottom panel and that a support export quotes verbatim.
//!
//! The host does not spawn real processes in M1. It models session lifecycle
//! and provenance so the live shell can render a truthful terminal pane in the
//! bottom panel, prove session identity stays visible across termination /
//! restart, and unblock M01-074 / M01-077 / M01-078 / M01-089 / M01-096
//! without forking session truth into shell-only fields.

#![doc(html_root_url = "https://docs.rs/aureline-terminal/0.0.0")]

pub mod pty_host;
/// Transcript / ended-session restore projection. Restored records never
/// claim a live shell survived; auto-rerun is always forbidden.
pub mod restore;
/// Bounded, redaction-aware scrollback ring used by transcript restore and
/// support / export bundles.
pub mod scrollback;

pub use pty_host::{
    HostClass, OpenSessionRequest, PtyHost, PtyHostError, PtySession, PtySessionId, SessionHeader,
    SessionLifecycleState, SessionLifecycleTransition, TerminalTrustState,
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
