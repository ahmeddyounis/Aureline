//! Shell consumer for canonical Git status snapshots.
//!
//! This module deliberately reuses the projections owned by
//! [`aureline_git::status`] so shell, activity-center, and review seed records
//! remain tied to one Git service snapshot instead of re-running repository
//! discovery per surface.

pub use aureline_git::status::{
    ConsumerProjectionBundle as GitStatusSurfaceBundle, GitActivityRecord, GitReviewSeedRecord,
    GitShellStatusRecord, GitStatusSnapshot, GIT_ACTIVITY_RECORD_KIND, GIT_REVIEW_SEED_RECORD_KIND,
    GIT_SHELL_STATUS_RECORD_KIND, GIT_STATUS_SNAPSHOT_RECORD_KIND,
};
pub use aureline_git::{
    GitBranchActivityRecord, GitBranchPreview, GitBranchResult, GIT_BRANCH_ACTIVITY_RECORD_KIND,
    GIT_BRANCH_PREVIEW_RECORD_KIND, GIT_BRANCH_RESULT_RECORD_KIND,
};
