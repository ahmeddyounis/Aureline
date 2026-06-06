//! Stabilized profile-switch review and temporary-profile lifecycle contract.
//!
//! The module exports a deterministic corpus and validation model for claimed
//! stable profile lanes. It covers pre-apply switch review, restart-delta truth,
//! temporary-profile lifecycle actions, text-based profile artifacts,
//! non-widening import and sync conflict review, rollback checkpoints, and
//! local-authoritative fallback when sync or device registry state is degraded.

pub mod corpus;
pub mod model;

pub use corpus::{profile_switch_lifecycle_corpus, ProfileSwitchLifecycleScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, validate_profile_switch_lifecycle_record, ApplyAuditRow,
    ApplyTimingClass, ArtifactExclusionClass, ConflictSourceClass, ExcludedMachineStateRow,
    ImportConflictReviewRow, LocalAuthoritativeReason, NarrowingEffectRow,
    ProfileArtifactBoundaryRow, ProfileCardRow, ProfileDurabilityClass, ProfileScopeClass,
    ProfileSourceClass, ProfileSwitchDeltaRow, ProfileSwitchLifecycleCertification,
    ProfileSwitchLifecyclePillars, ProfileSwitchLifecycleQualification,
    ProfileSwitchLifecycleValidationError, ProfileSwitchNarrowingReason, ProfileSwitchReviewSheet,
    StableClaimClass, SurfaceClass, SurfaceTruthRow, SyncFallbackRow, TemporaryProfileActionClass,
    TemporaryProfileActionRow, TemporaryProfileLifecycle, PROFILE_SWITCH_REVIEW_RECORD_KIND,
    PROFILE_SWITCH_REVIEW_SCHEMA_VERSION, PROFILE_SWITCH_REVIEW_SHARED_CONTRACT_REF,
};
