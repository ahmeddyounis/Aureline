//! Stable profile sync, snapshot, backup, restore, and offboarding contract.
//!
//! The module mints one governed record for profile portability surfaces. The
//! record names the snapshot class, included/excluded state classes, merge rule,
//! restore preview, rollback checkpoint, secret-boundary audit, retention
//! posture, offboarding package, and local-authoritative fallback before any
//! profile sync or restore mutation.

pub mod corpus;
pub mod model;

pub use corpus::{profile_sync_restore_corpus, ProfileSyncRestoreScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, BuildError, ConflictClass, MergeRuleClass, MergeRuleRow,
    MergeSubjectClass, NarrowingReason, OffboardingRetentionSummary,
    ProfileSyncRestoreCertification, ProfileSyncRestoreInput, ProfileSyncRestorePillars,
    ProfileSyncRestoreQualification, RestorePreviewRow, SecretBoundaryAuditRow, SnapshotClass,
    SnapshotManifestRow, StableClaimClass, StateClass, SurfaceClass, SurfaceTruthRow,
    PROFILE_SYNC_RESTORE_RECORD_KIND, PROFILE_SYNC_RESTORE_SCHEMA_VERSION,
    PROFILE_SYNC_RESTORE_SHARED_CONTRACT_REF,
};
