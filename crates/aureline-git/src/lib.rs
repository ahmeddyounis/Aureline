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
pub mod finalize_sequence_edit_conflict_session_stash_entry_and_ref_update_truth;
pub mod freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix;
pub mod harden_conflict_resolution_external_change_reconciliation_and_merge;
pub mod history_rewrite;
pub mod history_sessions;
pub mod mutations;
pub mod publish;
pub mod stabilize_repository_topology_truth;
pub mod stabilize_the_daily_git_loop_status_diff_stage;
pub mod status;
pub mod topology;
pub mod topology_actions;

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
pub use change_objects::{
    project_change_object, ChangeObjectBranchVariant, ChangeObjectError, ChangeObjectLandingState,
    ChangeObjectLineage, ChangeObjectLineageEntry, ChangeObjectPatchStackVariant,
    ChangeObjectProjection, ChangeObjectRecord, ChangeObjectReviewInvariants,
    ChangeObjectSupportExport, ChangeObjectValidationError, ChangeObjectWorktreeVariant,
    CHANGE_OBJECT_ALPHA_RECORD_KIND, CHANGE_OBJECT_ALPHA_SCHEMA_VERSION,
    CHANGE_OBJECT_BRANCH_KIND_CLASSES, CHANGE_OBJECT_CONSUMER_SURFACES,
    CHANGE_OBJECT_DIVERGENCE_CLASSES, CHANGE_OBJECT_KINDS, CHANGE_OBJECT_LANDING_ACTION_CLASSES,
    CHANGE_OBJECT_LANDING_STATE_CLASSES, CHANGE_OBJECT_MUTATION_AUTHORITY_CLASSES,
    CHANGE_OBJECT_NETWORK_EGRESS_CLASSES, CHANGE_OBJECT_PATCH_STACK_TARGET_CLASSES,
    CHANGE_OBJECT_PATCH_STATE_CLASSES, CHANGE_OBJECT_REMOTE_VISIBILITY_CLASSES,
    CHANGE_OBJECT_REVIEW_CLASSES, CHANGE_OBJECT_WORKTREE_ATTACHMENT_CLASSES,
    CHANGE_OBJECT_WORKTREE_KIND_CLASSES,
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
pub use finalize_sequence_edit_conflict_session_stash_entry_and_ref_update_truth::{
    parse_risky_vcs_truth_packet, project_risky_vcs_truth_packet, RiskyVcsCommandBinding,
    RiskyVcsConflictSession, RiskyVcsInspectionRecord, RiskyVcsRecoveryCheckpoint,
    RiskyVcsRefPosition, RiskyVcsRefUpdateProposal, RiskyVcsSequenceEditSession,
    RiskyVcsSequenceOperation, RiskyVcsStashEntry, RiskyVcsSupportExport, RiskyVcsTruthError,
    RiskyVcsTruthPacket, RiskyVcsTruthProjection, RiskyVcsTruthValidationError,
    RISKY_VCS_APPROVAL_STATES, RISKY_VCS_CHECKPOINT_STATES, RISKY_VCS_CHECKPOINT_TRIGGER_KINDS,
    RISKY_VCS_CHECK_INVALIDATION_STATES, RISKY_VCS_COMMAND_BINDING_RECORD_KIND,
    RISKY_VCS_COMMAND_CLASSES, RISKY_VCS_CONFLICT_RESOLUTION_MODES,
    RISKY_VCS_CONFLICT_SESSION_RECORD_KIND, RISKY_VCS_CONFLICT_STATES, RISKY_VCS_CONSUMER_SURFACES,
    RISKY_VCS_DIVERGENCE_CLASSES, RISKY_VCS_INSPECTION_RECORD_KIND, RISKY_VCS_OPERATION_KINDS,
    RISKY_VCS_PROTECTED_BRANCH_POSTURES, RISKY_VCS_PUBLISH_MODES,
    RISKY_VCS_RECOVERY_CHECKPOINT_RECORD_KIND, RISKY_VCS_REF_UPDATE_PROPOSAL_RECORD_KIND,
    RISKY_VCS_REF_UPDATE_STATES, RISKY_VCS_RESTORE_OPTION_CLASSES,
    RISKY_VCS_SEQUENCE_EDIT_SESSION_RECORD_KIND, RISKY_VCS_SEQUENCE_STATES,
    RISKY_VCS_SEQUENCE_VERBS, RISKY_VCS_STASH_ENTRY_RECORD_KIND, RISKY_VCS_STASH_STATES,
    RISKY_VCS_SUPPORT_EXPORT_RECORD_KIND, RISKY_VCS_TARGET_SELECTION_STATES,
    RISKY_VCS_TODO_VERSION_STATES, RISKY_VCS_TRUTH_PACKET_RECORD_KIND,
    RISKY_VCS_TRUTH_SCHEMA_VERSION, RISKY_VCS_UNTRACKED_STATE_POSTURES,
};
pub use freeze_the_m5_repository_topology_worktree_scope_history_surgery_and_checkpoint_recovery_matrix::{
    current_stable_m5_git_topology_history_matrix_export, DegradedStateRow, DegradedTopologyState,
    HistorySurgerySession, M5GitTopologyHistoryMatrixArtifactError, M5GitTopologyHistoryMatrixPacket,
    M5GitTopologyHistoryMatrixPacketInput, M5GitTopologyHistoryMatrixViolation, MatrixConsumerParity,
    MatrixConsumerSurface, MatrixFreezePosture, MatrixGovernanceReview, OperationPreviewClass,
    OperationRecoveryClass, RiskyHistoryOperation, RiskyOperationRow, SessionObjectRow,
    TopologyClassRow, M5_GIT_TOPOLOGY_HISTORY_MATRIX_ARTIFACT_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_CONFLICT_SESSION_CONTRACT_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_DOC_REF, M5_GIT_TOPOLOGY_HISTORY_MATRIX_FIXTURE_DIR,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_RECORD_KIND,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_RECOVERY_CHECKPOINT_CONTRACT_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_REF_UPDATE_CONTRACT_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_REQUIRED_CLASSES, M5_GIT_TOPOLOGY_HISTORY_MATRIX_SCHEMA_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_SCHEMA_VERSION,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_SEQUENCE_EDIT_CONTRACT_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_STASH_CONTRACT_REF, M5_GIT_TOPOLOGY_HISTORY_MATRIX_SUMMARY_REF,
    M5_GIT_TOPOLOGY_HISTORY_MATRIX_TOPOLOGY_CONTRACT_REF,
};
pub use harden_conflict_resolution_external_change_reconciliation_and_merge::{
    build_stable_conflict_session_packet, parse_stable_conflict_session_record,
    project_stable_conflict_session, ConflictProvenanceInput, ConflictProvenanceRecord,
    ConflictResolutionMode, StableConflictAuditEvent, StableConflictSessionCommandInput,
    StableConflictSessionCommandRecord, StableConflictSessionError, StableConflictSessionInput,
    StableConflictSessionInspectionRecord, StableConflictSessionPacket,
    StableConflictSessionProjection, StableConflictSessionRecord,
    StableConflictSessionRestartSnapshot, StableConflictSessionSupportExport,
    StableConflictSessionSupportExportInput, StableConflictSessionSupportExportPacket,
    StableConflictValidationError, CONFLICT_INPUT_FRESHNESS_CLASSES,
    CONFLICT_PROVENANCE_SOURCE_CLASSES, CONFLICT_RESOLUTION_MODES, STABLE_CONFLICT_AUDIT_EVENTS,
    STABLE_CONFLICT_COMMAND_CLASSES, STABLE_CONFLICT_CONSUMER_SURFACES,
    STABLE_CONFLICT_OPERATION_KINDS, STABLE_CONFLICT_SESSION_COMMAND_RECORD_KIND,
    STABLE_CONFLICT_SESSION_INSPECTION_RECORD_KIND, STABLE_CONFLICT_SESSION_LIFECYCLE_STATES,
    STABLE_CONFLICT_SESSION_PACKET_RECORD_KIND, STABLE_CONFLICT_SESSION_RECORD_KIND,
    STABLE_CONFLICT_SESSION_SCHEMA_VERSION,
    STABLE_CONFLICT_SESSION_SUPPORT_EXPORT_PACKET_RECORD_KIND,
};
pub use history_rewrite::{
    parse_history_rewrite_record, project_history_rewrite_record, ConflictSessionRecord,
    HistoryRewriteAuditEvent, HistoryRewriteBlock, HistoryRewriteError, HistoryRewriteNextSafePath,
    HistoryRewriteProjection, HistoryRewriteRecord, HistoryRewriteRecoveryPosture,
    HistoryRewriteRefId, HistoryRewriteSupportExport, HistoryRewriteValidationError,
    HistoryRewriteWorktreeContext, RecoveryCheckpointRecord, RefUpdateProposalRecord,
    SequenceEditSessionRecord, SequenceEditStep, StashEntryRecord, CONFLICT_ACTION_CLASSES,
    CONFLICT_SESSION_LIFECYCLE_STATES, CONFLICT_SESSION_RECORD_KIND, HISTORY_REWRITE_AUDIT_EVENTS,
    HISTORY_REWRITE_BETA_SCHEMA_VERSION, HISTORY_REWRITE_CONSUMER_SURFACES,
    HISTORY_REWRITE_OPERATION_KINDS, NEXT_SAFE_PATH_CLASSES, RECOVERY_CHECKPOINT_LIFECYCLE_STATES,
    RECOVERY_CHECKPOINT_RECORD_KIND, RECOVERY_POSTURE_CLASSES, REF_UPDATE_BLOCK_CLASSES,
    REF_UPDATE_PROPOSAL_LIFECYCLE_STATES, REF_UPDATE_PROPOSAL_RECORD_KIND,
    SEQUENCE_EDIT_SESSION_LIFECYCLE_STATES, SEQUENCE_EDIT_SESSION_RECORD_KIND, SEQUENCE_EDIT_VERBS,
    STASH_ENTRY_LIFECYCLE_STATES, STASH_ENTRY_RECORD_KIND,
};
pub use history_sessions::{
    current_history_session_first_consumers_map, HistorySession, HistorySessionConsumerMap,
    HistorySessionError, HistorySessionSupportExport, HistorySessionValidationError,
    SessionConsumerBinding, SessionConsumerSurface, HISTORY_SESSION_ACTIONS,
    HISTORY_SESSION_ARTIFACT_REF, HISTORY_SESSION_CONSUMER_BINDING_RECORD_KIND,
    HISTORY_SESSION_DESCRIPTOR_RECORD_KIND, HISTORY_SESSION_FIXTURE_DIR,
    HISTORY_SESSION_MAP_RECORD_KIND, HISTORY_SESSION_REQUIRED_RECONSTRUCTION_FIELDS,
    HISTORY_SESSION_SCHEMA_REF, HISTORY_SESSION_SCHEMA_VERSION, HISTORY_SESSION_STASH_VERBS,
    HISTORY_SESSION_SUPPORT_EXPORT_RECORD_KIND,
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
pub use stabilize_repository_topology_truth::{
    CoverageClaimPosture, RepositoryTopologyClass, RepositoryTopologyDescriptor,
    RepositoryTopologyPacketError, RepositoryTopologySupportExport, RepositoryTopologyTruthPacket,
    RepositoryTopologyValidationError, SurfaceResultTruth, SurfaceTopologyTruthRow, TopologyAction,
    TopologyActionApproval, TopologyActionClass, TopologyHonestyLabel, TopologyOperationScope,
    TopologySurface, REPOSITORY_TOPOLOGY_DESCRIPTOR_RECORD_KIND,
    REPOSITORY_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND, REPOSITORY_TOPOLOGY_TRUTH_PACKET_RECORD_KIND,
    REPOSITORY_TOPOLOGY_TRUTH_SCHEMA_VERSION, SURFACE_TOPOLOGY_TRUTH_ROW_RECORD_KIND,
};
pub use stabilize_the_daily_git_loop_status_diff_stage::{
    BlameLineRecord, DailyLoopActivityRecord, DailyLoopBackendError, DailyLoopBackendErrorClass,
    DailyLoopCommandOutput, DailyLoopCommitPreview, DailyLoopDiffFile, DailyLoopDiffHunk,
    DailyLoopDiffLine, DailyLoopDiffLineKind, DailyLoopFileChangeKind, DailyLoopJournalRecord,
    DailyLoopOperationKind, DailyLoopOutcomeState, DailyLoopPathChangeKind, DailyLoopPathStatus,
    DailyLoopPreview, DailyLoopPreviewState, DailyLoopRequest, DailyLoopResult, DailyLoopService,
    DailyLoopSnapshot, DailyLoopSnapshotState, DailyLoopSupportExportRecord, DailyLoopTarget,
    HistoryCommitRecord, RepoTarget, StashShelfEntry, SystemDailyLoopBackend, WorktreeTarget,
    BLAME_LINE_RECORD_KIND, CONTENT_AVAILABILITY_CLASSES, DAILY_LOOP_ACTIVITY_RECORD_KIND,
    DAILY_LOOP_JOURNAL_RECORD_KIND, DAILY_LOOP_OPERATION_KINDS, DAILY_LOOP_OUTCOME_STATES,
    DAILY_LOOP_PREVIEW_RECORD_KIND, DAILY_LOOP_PREVIEW_STATES, DAILY_LOOP_RESULT_RECORD_KIND,
    DAILY_LOOP_SNAPSHOT_RECORD_KIND, DAILY_LOOP_SUPPORT_EXPORT_RECORD_KIND,
    HISTORY_COMMIT_RECORD_KIND, STASH_COMMAND_CLASSES, STASH_SHELF_ENTRY_LIFECYCLE_STATES,
    STASH_SHELF_ENTRY_RECORD_KIND,
};
pub use topology::{
    current_git_topology_first_consumers_map, CheckoutFilterClass, DepthBoundary,
    GeneratedVendorClass, GeneratedVendorOrigin, GitTopologyError, GitTopologyValidationError,
    HistoryDepthClass, LfsObjectState, LfsState, ObjectAvailability, OmittedPathSet, RepoIdentity,
    RepoIdentityKind, RepositoryTopologyMap, SurfaceTopologyBinding, TopologyConsumerSurface,
    TopologyRootDescriptor, TopologySupportExport, WorktreeKind, WorktreeScope,
    GIT_TOPOLOGY_ARTIFACT_REF, GIT_TOPOLOGY_FIXTURE_DIR, GIT_TOPOLOGY_MAP_RECORD_KIND,
    GIT_TOPOLOGY_REQUIRED_RECONSTRUCTION_FIELDS, GIT_TOPOLOGY_ROOT_DESCRIPTOR_RECORD_KIND,
    GIT_TOPOLOGY_SCHEMA_REF, GIT_TOPOLOGY_SCHEMA_VERSION, GIT_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND,
    GIT_TOPOLOGY_SURFACE_BINDING_RECORD_KIND,
};
pub use topology_actions::{
    current_topology_action_review_packet, ActionTargetSelector, MultiRootPreview,
    NetworkSideEffect, ObjectScopeDisclosure, ProviderAuthPosture, RecoveryDisclosure,
    ReviewExportParity, TopologyActionError, TopologyActionKind, TopologyActionReviewPacket,
    TopologyActionSheet, TopologyActionSupportExport, TopologyActionValidationError,
    TopologyRecoveryClass, TopologyTargetKind, WrongRootGuard,
    TOPOLOGY_ACTION_REQUIRED_RECONSTRUCTION_FIELDS, TOPOLOGY_ACTION_REVIEW_ARTIFACT_REF,
    TOPOLOGY_ACTION_REVIEW_FIXTURE_DIR, TOPOLOGY_ACTION_REVIEW_PACKET_RECORD_KIND,
    TOPOLOGY_ACTION_REVIEW_SCHEMA_REF, TOPOLOGY_ACTION_REVIEW_SCHEMA_VERSION,
    TOPOLOGY_ACTION_SHEET_RECORD_KIND, TOPOLOGY_ACTION_SUPPORT_EXPORT_RECORD_KIND,
};
pub use status::{
    BranchState, ChangeDiscovery, ChangeKind, ChangeSummary, ConsumerProjectionBundle,
    GitActivityRecord, GitBackendError, GitBackendErrorClass, GitChange, GitCommandOutput,
    GitConsumerRef, GitReviewSeedRecord, GitServiceState, GitShellStatusRecord, GitStatusBackend,
    GitStatusRequest, GitStatusService, GitStatusSnapshot, HeadIdentity, RepositoryIdentity,
    StatusPorcelainParseError, SystemGitStatusBackend, GIT_ACTIVITY_RECORD_KIND,
    GIT_REVIEW_SEED_RECORD_KIND, GIT_SHELL_STATUS_RECORD_KIND, GIT_STATUS_SNAPSHOT_RECORD_KIND,
};
