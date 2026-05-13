//! Shell status projections for shared runtime truth surfaces.
//!
//! Status modules consume canonical product-state records from owner crates
//! and materialize chrome-facing rows without inventing new vocabulary.

pub mod git_status;
pub mod index_state;
pub mod query_envelope;

pub use git_status::{
    GitActivityRecord, GitReviewSeedRecord, GitShellStatusRecord, GitStatusSnapshot,
    GitStatusSurfaceBundle, GIT_ACTIVITY_RECORD_KIND, GIT_REVIEW_SEED_RECORD_KIND,
    GIT_SHELL_STATUS_RECORD_KIND, GIT_STATUS_SNAPSHOT_RECORD_KIND,
};
pub use index_state::{
    IndexStateResultPaneRecord, IndexStateStatusRecord, IndexStateSurfaceBundle,
    INDEX_STATE_RESULT_PANE_RECORD_KIND, INDEX_STATE_STATUS_RECORD_KIND,
    INDEX_STATE_SURFACE_BUNDLE_RECORD_KIND,
};
pub use query_envelope::{
    QueryEnvelopeStatusRecord, QueryEnvelopeSurfaceBundle, QUERY_ENVELOPE_STATUS_RECORD_KIND,
    QUERY_ENVELOPE_SURFACE_BUNDLE_RECORD_KIND,
};
