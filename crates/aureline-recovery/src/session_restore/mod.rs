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
    SessionRestoreCaptureInput, SessionRestoreError, SessionRestoreLatestRefs, SessionRestoreStore,
    SessionRestoreSummary, TabGroupCaptureInput, TabItemCaptureInput,
};
