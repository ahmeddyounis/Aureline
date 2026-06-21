//! Cross-client follow-state truth: follow, break away, request follow, take
//! over, and cached snapshot, spoken identically on desktop, browser, and
//! companion.
//!
//! A presentation can be observed from more than one claimed M5 client at once.
//! This module makes each client's follow posture an **explicit, attributable
//! state** drawn from one shared vocabulary, so audience state is never inferred
//! from viewport drift, connection timing, or a transient toast.
//!
//! - [`state`] holds the cross-client vocabulary — [`FollowMode`], its
//!   [`LivenessClass`] (live / independent / cached snapshot), the durable
//!   [`DurableBreakawayBanner`], the self-identifying [`SnapshotIdentity`], and
//!   the canonical [`RecoveryAction`]s — plus [`project_follow_state_truth`],
//!   which builds a [`FollowStateTruth`] packet, and
//!   [`FollowStateTruth::validate`], which re-checks every honesty invariant.
//! - [`corpus`] is the mint-from-truth seed corpus, support export, and
//!   validation that the checked-in fixtures and headless inspectors share.
//!
//! The support-export boundary schema is
//! [`schemas/presentation/follow-state-truth.schema.json`](../../../../../schemas/presentation/follow-state-truth.schema.json);
//! the human-readable contract is `docs/ux/presentation-follow-and-breakaway.md`
//! and the cross-client coverage matrix is
//! `artifacts/presentation/cross-client-follow-matrix.md`.

pub mod corpus;
pub mod state;

pub use corpus::{
    follow_state_support_export, seeded_follow_state_corpus, validate_follow_state_corpus,
    FollowStateCase, FollowStateCorpus, FollowStateCorpusError, FollowStateSummary,
    FOLLOW_STATE_CASE_RECORD_KIND, FOLLOW_STATE_CORPUS_RECORD_KIND,
};
pub use state::{
    project_follow_state_truth, ClientFollowInput, ClientFollowView, ClientSurface,
    DurableBreakawayBanner, FollowMode, FollowStateSupportExport, FollowStateSupportExportRow,
    FollowStateTruth, FollowStateViolation, LivenessClass, RecoveryAction, RecoveryKind,
    SnapshotIdentity, SnapshotStalenessReason, CLIENT_FOLLOW_VIEW_RECORD_KIND,
    CROSS_CLIENT_FOLLOW_MATRIX_REF, FOLLOW_STATE_SUPPORT_EXPORT_RECORD_KIND,
    FOLLOW_STATE_SUPPORT_EXPORT_ROW_RECORD_KIND, FOLLOW_STATE_TRUTH_RECORD_KIND,
    PRESENTATION_FOLLOW_AND_BREAKAWAY_DOC_REF, PRESENTATION_FOLLOW_FIXTURE_DIR,
};

#[cfg(test)]
mod tests;
