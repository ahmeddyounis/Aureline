//! Quick-open query session: recent targets, commands, and lexical results.
//!
//! Quick open is the first universal lookup surface in M1. It is **not** the
//! command palette, and it is **not** a private repaint of the lexical search
//! shell — it is a query session that merges three canonical sources behind
//! one mental model:
//!
//! - **Recent targets** (files / places the user has navigated to recently),
//!   served from hot local state.
//! - **Commands**, projected from the canonical
//!   [`aureline_commands::CommandRegistry`] (the same registry the command
//!   palette consumes — quick open never invents its own command truth).
//! - **Lexical file/path hits**, projected from the canonical
//!   [`aureline_search::LexicalShell`] (the same shell the workspace search
//!   surface consumes — quick open never invents its own lexical truth).
//!
//! Honesty contract:
//!
//! 1. Every row carries the [`QuickOpenSourceClass`] that produced it. Rows
//!    never claim a higher-confidence lane than they earned, and rows never
//!    relabel a lexical hit as semantic just because a future surface ships
//!    alongside.
//! 2. Every source carries a [`QuickOpenSourceState`]. The chrome must surface
//!    `warming`, `partial`, `unavailable`, or `not_requested` directly; it
//!    must not collapse them into a generic "loading" badge.
//! 3. Recent targets win on duplicates. When the same workspace-relative path
//!    appears in both recents and lexical results, the row appears once under
//!    the recents lane with `winning_source_class` set to `recent_target`.
//! 4. Command rows quote canonical command identity (`command_id`),
//!    `disabled_reason_class` (when blocked), and the command's
//!    `invocation_preview_class` so the same identity tuple shows up across
//!    palette, quick open, command diagnostics, and support exports.
//!
//! The session intentionally does not own filesystem IO or the lexical scan
//! itself — the consumer feeds in command rows, recent targets, and a
//! lexical-shell projection. This keeps the runtime path small enough to
//! verify deterministically against fixtures.

pub mod session;

pub use session::{
    QuickOpenCommandRow, QuickOpenLexicalRow, QuickOpenQuerySession, QuickOpenRecentTarget,
    QuickOpenResultRow, QuickOpenRowKey, QuickOpenRowKind, QuickOpenScopeChip, QuickOpenSnapshot,
    QuickOpenSnapshotRow, QuickOpenSnapshotSource, QuickOpenSourceClass, QuickOpenSourceState,
    COMMANDS_LANE_CAP, LEXICAL_LANE_CAP, RECENTS_LANE_CAP,
};
