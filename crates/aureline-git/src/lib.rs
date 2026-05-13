//! Canonical Git service contracts for launch-wedge repository truth.
//!
//! This crate owns the first local Git status substrate for Aureline. It wraps
//! the system Git implementation behind a typed service, parses branch/status
//! output once, and publishes shared projections that shell chrome, activity
//! center rows, review seed surfaces, support exports, and CLI mirrors can all
//! consume without gathering repository state independently.

#![doc(html_root_url = "https://docs.rs/aureline-git/0.0.0")]

pub mod status;

pub use status::{
    BranchState, ChangeDiscovery, ChangeKind, ChangeSummary, ConsumerProjectionBundle,
    GitActivityRecord, GitBackendError, GitBackendErrorClass, GitChange, GitCommandOutput,
    GitConsumerRef, GitReviewSeedRecord, GitServiceState, GitShellStatusRecord, GitStatusBackend,
    GitStatusRequest, GitStatusService, GitStatusSnapshot, HeadIdentity, RepositoryIdentity,
    StatusPorcelainParseError, SystemGitStatusBackend, GIT_ACTIVITY_RECORD_KIND,
    GIT_REVIEW_SEED_RECORD_KIND, GIT_SHELL_STATUS_RECORD_KIND, GIT_STATUS_SNAPSHOT_RECORD_KIND,
};
