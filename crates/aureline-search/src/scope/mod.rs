//! Workspace search scope: workset/slice-aware scope projection.
//!
//! This module is the canonical resolver every search and quick-open surface
//! reads when it must answer "what does the active workset/slice include and
//! what does the user see on the chip?". It carries:
//!
//! - the workset/slice identity (workset_id, workset_name) plus the canonical
//!   [`crate::lexical::ScopeClass`] vocabulary, projected from the workspace
//!   crate so the search shell never forks scope truth;
//! - the include/exclude pattern set used to filter the lexical file list, so
//!   switching worksets actually narrows the result set rather than just the
//!   chip;
//! - the chip presentation state and the `partial_scope` flag the chrome
//!   surfaces alongside any narrowed view; and
//! - a serializable [`projection::WorkspaceSearchScopeMetadata`] that flows
//!   into the search-shell and quick-open snapshot exports so an exported
//!   session can be replayed with the same scope label, partial-truth flag,
//!   and pattern fingerprint that produced it.
//!
//! The protected walk is: open a multi-root or sparse-scope workspace, the
//! chip says what scope means right now, and search/open only walks files in
//! that scope. The failure drill is: switch worksets while the user is mid
//! search — the scope chip must update before any result row can be mistaken
//! for repo-wide truth.

pub mod filter;
pub mod projection;

pub use filter::{
    glob_matches_relative_path, ScopeFilterOutcome, ScopePatternKind, ScopePatternRecord,
};
pub use projection::{ScopePresentationState, WorkspaceSearchScope, WorkspaceSearchScopeMetadata};
