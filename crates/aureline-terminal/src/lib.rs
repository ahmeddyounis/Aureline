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

pub use pty_host::{
    HostClass, OpenSessionRequest, PtyHost, PtyHostError, PtySession, PtySessionId, SessionHeader,
    SessionLifecycleState, SessionLifecycleTransition, TerminalTrustState,
};
