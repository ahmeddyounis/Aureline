//! Canonical Git service contracts for launch-wedge repository truth.
//!
//! This crate owns the first local Git status substrate for Aureline. It wraps
//! the system Git implementation behind a typed service, parses branch/status
//! output once, and publishes shared projections that shell chrome, activity
//! center rows, review seed surfaces, support exports, and CLI mirrors can all
//! consume without gathering repository state independently.

#![doc(html_root_url = "https://docs.rs/aureline-git/0.0.0")]

pub mod branches;
pub mod change_objects;
pub mod commit;
pub mod conflicts;
pub mod mutations;
pub mod publish;
pub mod status;

pub use change_objects::{
    project_change_object, ChangeObjectBranchVariant, ChangeObjectError,
    ChangeObjectLandingState, ChangeObjectLineage, ChangeObjectLineageEntry,
    ChangeObjectPatchStackVariant, ChangeObjectProjection, ChangeObjectRecord,
    ChangeObjectReviewInvariants, ChangeObjectSupportExport, ChangeObjectValidationError,
    ChangeObjectWorktreeVariant, CHANGE_OBJECT_ALPHA_RECORD_KIND,
    CHANGE_OBJECT_ALPHA_SCHEMA_VERSION, CHANGE_OBJECT_BRANCH_KIND_CLASSES,
    CHANGE_OBJECT_CONSUMER_SURFACES, CHANGE_OBJECT_DIVERGENCE_CLASSES, CHANGE_OBJECT_KINDS,
    CHANGE_OBJECT_LANDING_ACTION_CLASSES, CHANGE_OBJECT_LANDING_STATE_CLASSES,
    CHANGE_OBJECT_MUTATION_AUTHORITY_CLASSES, CHANGE_OBJECT_NETWORK_EGRESS_CLASSES,
    CHANGE_OBJECT_PATCH_STACK_TARGET_CLASSES, CHANGE_OBJECT_PATCH_STATE_CLASSES,
    CHANGE_OBJECT_REMOTE_VISIBILITY_CLASSES, CHANGE_OBJECT_REVIEW_CLASSES,
    CHANGE_OBJECT_WORKTREE_ATTACHMENT_CLASSES, CHANGE_OBJECT_WORKTREE_KIND_CLASSES,
};
pub use branches::{
    GitBranchActivityRecord, GitBranchActorRef, GitBranchBackend, GitBranchBackendError,
    GitBranchCommandOutput, GitBranchCurrentWorkReview, GitBranchJournalRecord,
    GitBranchOperationKind, GitBranchOutcomeState, GitBranchPreview, GitBranchPreviewState,
    GitBranchRemoteState, GitBranchRequest, GitBranchResult, GitBranchService,
    GitBranchSupportExportRecord, GitBranchTargetKind, GitBranchTargetReview,
    SystemGitBranchBackend, GIT_BRANCH_ACTIVITY_RECORD_KIND, GIT_BRANCH_JOURNAL_RECORD_KIND,
    GIT_BRANCH_PREVIEW_RECORD_KIND, GIT_BRANCH_RESULT_RECORD_KIND,
    GIT_BRANCH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use commit::{
    GitCommitActivityRecord, GitCommitActorRef, GitCommitAuthorIdentity, GitCommitAuthorInput,
    GitCommitAuthorSource, GitCommitAuthorState, GitCommitBackend, GitCommitBackendError,
    GitCommitCommandOutput, GitCommitHistoryGuardrail, GitCommitJournalRecord, GitCommitMode,
    GitCommitOutcomeState, GitCommitPreview, GitCommitPreviewState,
    GitCommitPublishReadinessRecord, GitCommitRequest, GitCommitResult, GitCommitScopeTarget,
    GitCommitService, GitCommitStagedScopeReview, GitCommitSupportExportRecord,
    SystemGitCommitBackend, GIT_COMMIT_ACTIVITY_RECORD_KIND, GIT_COMMIT_JOURNAL_RECORD_KIND,
    GIT_COMMIT_PREVIEW_RECORD_KIND, GIT_COMMIT_PUBLISH_READINESS_RECORD_KIND,
    GIT_COMMIT_RESULT_RECORD_KIND, GIT_COMMIT_SUPPORT_EXPORT_RECORD_KIND,
};
pub use conflicts::{
    GitConflictDivergenceSource, GitConflictExternalCompareProjection,
    GitConflictGitStateProjection, GitConflictHandoffPacket, GitConflictHandoffRequest,
    GitConflictHandoffService, GitConflictPathIdentity, GitConflictRollbackCheckpoint,
    GitConflictSafeAction, GitConflictSupportExportRecord, GitConflictSurfaceKind,
    GitConflictSurfaceRecord, GitConflictSurfaceState, GitExternalChangeHandoffInput,
    GIT_CONFLICT_HANDOFF_PACKET_RECORD_KIND, GIT_CONFLICT_SUPPORT_EXPORT_RECORD_KIND,
    GIT_CONFLICT_SURFACE_RECORD_KIND,
};
pub use mutations::{
    GitMutationActivityRecord, GitMutationActorRef, GitMutationBackend, GitMutationBackendError,
    GitMutationCheckpointRecord, GitMutationCommandOutput, GitMutationDiffPreview,
    GitMutationJournalRecord, GitMutationOperationKind, GitMutationOutcomeState,
    GitMutationPreview, GitMutationPreviewState, GitMutationRequest, GitMutationResult,
    GitMutationScopeReview, GitMutationService, GitMutationSupportExportRecord,
    GitMutationTargetReview, SystemGitMutationBackend, GIT_MUTATION_ACTIVITY_RECORD_KIND,
    GIT_MUTATION_JOURNAL_RECORD_KIND, GIT_MUTATION_PREVIEW_RECORD_KIND,
    GIT_MUTATION_RESULT_RECORD_KIND, GIT_MUTATION_SUPPORT_EXPORT_RECORD_KIND,
};
pub use publish::{
    GitPublishActivityRecord, GitPublishActorRef, GitPublishBackend, GitPublishBackendError,
    GitPublishCommandOutput, GitPublishFailureRecoveryRecord, GitPublishJournalRecord,
    GitPublishMode, GitPublishOriginScope, GitPublishOutcomeState, GitPublishPreview,
    GitPublishPreviewState, GitPublishRemoteState, GitPublishRequest, GitPublishResult,
    GitPublishRouteClass, GitPublishRouteReview, GitPublishService, GitPublishSupportExportRecord,
    GitPublishTargetReview, SystemGitPublishBackend, GIT_PUBLISH_ACTIVITY_RECORD_KIND,
    GIT_PUBLISH_FAILURE_RECOVERY_RECORD_KIND, GIT_PUBLISH_JOURNAL_RECORD_KIND,
    GIT_PUBLISH_PREVIEW_RECORD_KIND, GIT_PUBLISH_RESULT_RECORD_KIND,
    GIT_PUBLISH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use status::{
    BranchState, ChangeDiscovery, ChangeKind, ChangeSummary, ConsumerProjectionBundle,
    GitActivityRecord, GitBackendError, GitBackendErrorClass, GitChange, GitCommandOutput,
    GitConsumerRef, GitReviewSeedRecord, GitServiceState, GitShellStatusRecord, GitStatusBackend,
    GitStatusRequest, GitStatusService, GitStatusSnapshot, HeadIdentity, RepositoryIdentity,
    StatusPorcelainParseError, SystemGitStatusBackend, GIT_ACTIVITY_RECORD_KIND,
    GIT_REVIEW_SEED_RECORD_KIND, GIT_SHELL_STATUS_RECORD_KIND, GIT_STATUS_SNAPSHOT_RECORD_KIND,
};
