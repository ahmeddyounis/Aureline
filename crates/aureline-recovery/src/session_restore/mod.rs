// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Session-restore skeleton persistence.
//!
//! Session restore is split into two durable packet families:
//!
//! - Workspace-authority checkpoints (durable identity and recovery journal refs).
//! - Window-topology snapshots (pane and tab layout inventory + pane-tree body refs).
//!
//! This module provides file-backed stores and schema-shaped record types so
//! shell surfaces can offer restore proposals and recovery summaries after
//! abnormal termination without inferring state from ad hoc logs.
//!
//! Durable capture publication is create-new and join-first: checkpoint,
//! topology packet, and pane-tree bodies are synchronized before an immutable
//! latest-index record is published, then the exact joined refs are reopened
//! and validated before capture reports success. Ancestor redirects and
//! unbounded directory inventories fail closed. If a hard-link install has
//! happened but namespace durability or final identity cannot be proven,
//! capture returns [`SessionRestoreError::CommitStateUncertain`] with the
//! minted refs; callers must reopen the store and must not treat that outcome
//! as an ordinary pre-commit failure.
//!
//! Unix publication synchronizes an already-open parent-directory handle.
//! Rust 1.75 exposes no portable equivalent on Windows, so Windows publication
//! synchronizes each file and validates create-new namespace identity but does
//! not claim power-loss durability for the directory entry itself.
//! Stable standard-library APIs also lack a portable directory-handle-relative
//! hard-link operation; parent identity checks reject observed swaps, while a
//! swap and restore entirely inside the final name-operation window cannot be
//! excluded without a platform adapter.

pub mod proposal;
pub mod records;
mod store;

pub use proposal::{
    RestoreDirtyBufferFailure, RestoreDirtyBufferFailureKind, RestoreDirtyBufferReplay,
    RestoreOutcome, RestorePaneExecutionKind, RestorePaneOutcome, RestoreProposal,
    RestoreProposalArtifactRefs, RestoreProposalCounts, RestoreProposalDirtyBufferEntry,
    RestoreProposalPanePlan, RestoreProposalPlanKind, RestoreProposalSchemaVersion, RestoreRuntime,
};
pub use store::{
    SessionRestoreCaptureInput, SessionRestoreError, SessionRestoreLatestRefs,
    SessionRestoreSelection, SessionRestoreSelectionWarning, SessionRestoreSelectionWarningClass,
    SessionRestoreStore, SessionRestoreSummary, TabGroupCaptureInput, TabGroupLayoutCapture,
    TabItemCaptureInput,
};
