//! Local review workspace and diff surface contracts.
//!
//! This crate owns the first review-lane data model for Aureline. It projects
//! local Git change-list rows into inspectable diff-open targets and renders
//! alpha diff packets with syntax labels, suspicious-text cues, representation
//! safe copy actions, exact path truth, reopen continuity records, review
//! workspace seeds, stable row anchors, work-item relation projections, and
//! shared collection-view batch-review packets. Review exports consume
//! [`aureline_navigation::target_model`] when semantic navigation, reference,
//! hierarchy, or rename-preview evidence appears in a packet.

#![doc(html_root_url = "https://docs.rs/aureline-review/0.0.0")]

pub mod change_inspector;
pub mod collections;
pub mod diff;
pub mod finalize_git_and_review_support_export_packets_timeline;
pub mod finalize_issue_and_work_item_linkage_with_branch;
pub mod finalize_migration_rollback_checkpoints_diff_review_and_retained;
pub mod harden_browser_handoff_and_in_product_review_boundaries;
pub mod harden_merge_queue_ci_status_and_browser_handoff;
pub mod harden_merge_rebase_cherry_pick_revert_and_reset;
pub mod infrastructure_intelligence;
pub mod landing;
pub mod review_pack_dsl;
pub mod review_pack_parity_harness;
pub mod stabilize_provider_linked_object_models_snapshot_freshness_and;
pub mod stabilize_review_side_ai_evidence_attachment_and_safe;
pub mod stabilize_review_workspace_anchors_stale_base_labels_approval;
pub mod stabilize_work_item_status_transition_review;
pub mod stabilize_worktree_patch_stack_and_explicit_change_object;
pub mod workspace;

pub use aureline_navigation::target_model as navigation_target_model;
pub use change_inspector::{
    project_change_lineage, ChangeLineageAncestorEntry, ChangeLineageAncestryView,
    ChangeLineageConflictState, ChangeLineageError, ChangeLineageProjection,
    ChangeLineagePublishReadiness, ChangeLineageRecord, ChangeLineageReviewInvariants,
    ChangeLineageSupportExport, ChangeLineageTargetSummary, ChangeLineageValidationError,
    CHANGE_LINEAGE_ACTIVE_SCOPE_CLASSES, CHANGE_LINEAGE_ALPHA_RECORD_KIND,
    CHANGE_LINEAGE_ALPHA_SCHEMA_VERSION, CHANGE_LINEAGE_CONFLICT_STATE_CLASSES,
    CHANGE_LINEAGE_CONSUMER_SURFACES, CHANGE_LINEAGE_DIVERGENCE_CLASSES,
    CHANGE_LINEAGE_LANDING_ACTION_CLASSES, CHANGE_LINEAGE_LANDING_STATE_CLASSES,
    CHANGE_LINEAGE_MUTATION_AUTHORITY_CLASSES, CHANGE_LINEAGE_NETWORK_EGRESS_CLASSES,
    CHANGE_LINEAGE_OBJECT_KINDS, CHANGE_LINEAGE_PUBLISH_READINESS_CLASSES,
    CHANGE_LINEAGE_READINESS_BLOCKER_CLASSES, CHANGE_LINEAGE_REMOTE_VISIBILITY_CLASSES,
};
pub use collections::{
    ReviewCollectionAlphaInput, ReviewCollectionAlphaPacket,
    REVIEW_COLLECTION_ALPHA_PACKET_RECORD_KIND, REVIEW_COLLECTION_ALPHA_SCHEMA_VERSION,
};
pub use diff::{
    DiffClosedSessionRecord, DiffCompareTarget, DiffCompareTargetKind, DiffCopyAction,
    DiffCopyRepresentation, DiffFileInput, DiffHunkInput, DiffHunkView, DiffLineInput,
    DiffLineKind, DiffLineView, DiffOpenTarget, DiffPathTruth, DiffReopenProjection,
    DiffScrollAnchor, DiffSuspiciousCue, DiffSyntaxClass, DiffSyntaxProjection, DiffViewMode,
    DiffViewSurfacePacket, DIFF_CLOSED_SESSION_RECORD_KIND, DIFF_OPEN_TARGET_RECORD_KIND,
    DIFF_REOPEN_PROJECTION_RECORD_KIND, DIFF_VIEW_SURFACE_PACKET_RECORD_KIND,
};
pub use finalize_git_and_review_support_export_packets_timeline::{
    project_git_review_timeline_packet, GitReviewSupportExportInput, GitReviewSupportExportPacket,
    GitReviewSupportExportTimelinePacket, GitReviewTimelineError, GitReviewTimelineInput,
    GitReviewTimelineInspectionRecord, GitReviewTimelineProjection, GitReviewTimelineTruthRecord,
    GitReviewTimelineValidationError, OperatorPlaybookInput, OperatorPlaybookRecord,
    OperatorPlaybookStepInput, OperatorPlaybookStepRecord, TimelineEventInput, TimelineEventRecord,
    CHRONOLOGY_STATES, GIT_REVIEW_SUPPORT_EXPORT_PACKET_RECORD_KIND,
    GIT_REVIEW_TIMELINE_CONSUMER_SURFACES, GIT_REVIEW_TIMELINE_INSPECTION_RECORD_KIND,
    GIT_REVIEW_TIMELINE_INVALIDATION_REASONS, GIT_REVIEW_TIMELINE_PACKET_RECORD_KIND,
    GIT_REVIEW_TIMELINE_SCHEMA_VERSION, GIT_REVIEW_TIMELINE_TRUTH_RECORD_KIND,
    OPERATOR_PLAYBOOK_RECORD_KIND, OPERATOR_PLAYBOOK_STATES, OPERATOR_PLAYBOOK_STEP_RECORD_KIND,
    PLAYBOOK_STEP_AUTHORITY_CLASSES, PLAYBOOK_STEP_COMMAND_CLASSES, TIMELINE_CLOCK_SOURCE_CLASSES,
    TIMELINE_EVENT_KINDS, TIMELINE_EVENT_RECORD_KIND, TIMELINE_EVENT_SOURCE_CLASSES,
    TIMELINE_FRESHNESS_CLASSES,
};
pub use finalize_issue_and_work_item_linkage_with_branch::{
    project_work_item_linkage_finalization_packet, OfflineHandoffContinuityInput,
    OfflineHandoffContinuityRecord, PreviewedSideEffectInput, PreviewedSideEffectRecord,
    PublishLaterContinuityInput, PublishLaterContinuityRecord, StatusTransitionSheetInput,
    StatusTransitionSheetRecord, WorkItemBranchLinkInput, WorkItemBranchLinkRecord,
    WorkItemDetailSurfaceInput, WorkItemDetailSurfaceRecord, WorkItemLinkageCommandInput,
    WorkItemLinkageCommandRecord, WorkItemLinkageFinalizationError,
    WorkItemLinkageFinalizationInput, WorkItemLinkageFinalizationPacket,
    WorkItemLinkageFinalizationProjection,
    WorkItemLinkageFinalizationProjection as WorkItemLinkageProjection,
    WorkItemLinkageFinalizationRecord, WorkItemLinkageFinalizationValidationError,
    WorkItemLinkageInspectionRecord, WorkItemLinkageSupportExportInput,
    WorkItemLinkageSupportExportPacket, WorkItemReviewLinkInput, WorkItemReviewLinkRecord,
    FINALIZATION_STATES, OFFLINE_HANDOFF_CONTINUITY_RECORD_KIND,
    PUBLISH_LATER_CONTINUITY_RECORD_KIND, STATUS_TRANSITION_SHEET_RECORD_KIND,
    WORK_ITEM_BRANCH_LINK_RECORD_KIND, WORK_ITEM_DETAIL_SURFACE_RECORD_KIND,
    WORK_ITEM_LINKAGE_COMMAND_CLASSES, WORK_ITEM_LINKAGE_COMMAND_RECORD_KIND,
    WORK_ITEM_LINKAGE_CONSUMER_SURFACES, WORK_ITEM_LINKAGE_FINALIZATION_PACKET_RECORD_KIND,
    WORK_ITEM_LINKAGE_FINALIZATION_RECORD_KIND, WORK_ITEM_LINKAGE_FINALIZATION_SCHEMA_VERSION,
    WORK_ITEM_LINKAGE_INSPECTION_RECORD_KIND, WORK_ITEM_LINKAGE_INVALIDATION_REASONS,
    WORK_ITEM_LINKAGE_SUPPORT_EXPORT_PACKET_RECORD_KIND, WORK_ITEM_REVIEW_LINK_RECORD_KIND,
    WRITE_MODE_DISCLOSURE_CLASSES,
};
pub use finalize_migration_rollback_checkpoints_diff_review_and_retained::{
    project_migration_rollback_diff_review_packet, MigrationCommandInput, MigrationCommandRecord,
    MigrationDiffReviewInput, MigrationDiffReviewRecord, MigrationInspectionRecord,
    MigrationRestartSnapshot, MigrationRollbackCheckpointInput, MigrationRollbackCheckpointRecord,
    MigrationRollbackDiffReviewError, MigrationRollbackDiffReviewInput,
    MigrationRollbackDiffReviewPacket, MigrationRollbackDiffReviewProjection,
    MigrationRollbackDiffReviewRecord, MigrationRollbackDiffReviewValidationError,
    MigrationSupportExportInput, MigrationSupportExportPacket, RetainedDiagnosticInput,
    RetainedDiagnosticRecord, MIGRATION_CHECKPOINT_STATES, MIGRATION_COMMAND_CLASSES,
    MIGRATION_COMMAND_RECORD_KIND, MIGRATION_CONSUMER_SURFACES,
    MIGRATION_DIAGNOSTIC_ACTION_CLASSES, MIGRATION_DIAGNOSTIC_REASON_CLASSES,
    MIGRATION_DIFF_REVIEW_RECORD_KIND, MIGRATION_DIFF_REVIEW_STATES, MIGRATION_FLOW_STATES,
    MIGRATION_INSPECTION_RECORD_KIND, MIGRATION_INVALIDATION_REASONS, MIGRATION_OPERATION_KINDS,
    MIGRATION_ROLLBACK_CHECKPOINT_RECORD_KIND, MIGRATION_ROLLBACK_DIFF_REVIEW_PACKET_RECORD_KIND,
    MIGRATION_ROLLBACK_DIFF_REVIEW_RECORD_KIND, MIGRATION_ROLLBACK_DIFF_REVIEW_SCHEMA_VERSION,
    MIGRATION_SUPPORT_EXPORT_PACKET_RECORD_KIND, RETAINED_DIAGNOSTIC_RECORD_KIND,
};
pub use harden_browser_handoff_and_in_product_review_boundaries::{
    project_review_boundary_hardening_packet, BoundaryFreshnessObservationInput,
    BoundaryFreshnessObservationRecord, BoundaryHardeningCommandInput,
    BoundaryHardeningCommandRecord, BoundaryHardeningInspectionRecord,
    BoundaryHardeningSupportExportInput, BoundaryHardeningSupportExportPacket,
    BoundaryOwnershipSignalInput, BoundaryOwnershipSignalRecord, BrowserHandoffBoundaryInput,
    BrowserHandoffBoundaryRecord, InProductReviewBoundaryInput, InProductReviewBoundaryRecord,
    ProviderSourceIdentityInput, ProviderSourceIdentityRecord, ReturnPathInput, ReturnPathRecord,
    ReviewBoundaryHardeningError, ReviewBoundaryHardeningInput, ReviewBoundaryHardeningPacket,
    ReviewBoundaryHardeningProjection, ReviewBoundaryHardeningRecord,
    ReviewBoundaryHardeningValidationError, BOUNDARY_AUTHORITY_CLASSES, BOUNDARY_FRESHNESS_CLASSES,
    BOUNDARY_FRESHNESS_OBSERVATION_RECORD_KIND, BOUNDARY_HARDENING_COMMAND_CLASSES,
    BOUNDARY_HARDENING_COMMAND_RECORD_KIND, BOUNDARY_HARDENING_CONSUMER_SURFACES,
    BOUNDARY_HARDENING_INSPECTION_RECORD_KIND, BOUNDARY_HARDENING_INVALIDATION_REASONS,
    BOUNDARY_HARDENING_STATES, BOUNDARY_HARDENING_SUPPORT_EXPORT_PACKET_RECORD_KIND,
    BOUNDARY_OWNERSHIP_CLASSES, BOUNDARY_OWNERSHIP_SIGNAL_RECORD_KIND,
    BROWSER_HANDOFF_BOUNDARY_RECORD_KIND, HANDOFF_BOUNDARY_CLASSES,
    IN_PRODUCT_REVIEW_BOUNDARY_RECORD_KIND, PROVIDER_SOURCE_IDENTITY_RECORD_KIND,
    RETURN_PATH_CLASSES, RETURN_PATH_RECORD_KIND, REVIEW_BOUNDARY_HARDENING_PACKET_RECORD_KIND,
    REVIEW_BOUNDARY_HARDENING_RECORD_KIND, REVIEW_BOUNDARY_HARDENING_SCHEMA_VERSION,
};
pub use harden_merge_queue_ci_status_and_browser_handoff::{
    project_merge_queue_ci_status_browser_handoff_audit_packet, AuditCommandInput,
    AuditCommandRecord, AuditError, AuditInspectionRecord, AuditSupportExportInput,
    AuditSupportExportPacket, AuditValidationError, BrowserHandoffAuditInput,
    BrowserHandoffAuditRecord, CiCheckAuditInput, CiCheckAuditRecord, MergeQueueAuditInput,
    MergeQueueAuditRecord, MergeQueueCiStatusBrowserHandoffAuditInput,
    MergeQueueCiStatusBrowserHandoffAuditPacket, MergeQueueCiStatusBrowserHandoffAuditProjection,
    MergeQueueCiStatusBrowserHandoffAuditRecord, PipelineOverlayAuditInput,
    PipelineOverlayAuditRecord, RunControlAuditInput, RunControlAuditRecord, AUDIT_COMMAND_CLASSES,
    AUDIT_COMMAND_RECORD_KIND, AUDIT_CONSUMER_SURFACES, AUDIT_INSPECTION_RECORD_KIND,
    AUDIT_INVALIDATION_REASONS, AUDIT_STATES, AUDIT_SUPPORT_EXPORT_PACKET_RECORD_KIND,
    BROWSER_HANDOFF_AUDIT_CLASSES, BROWSER_HANDOFF_AUDIT_RECORD_KIND, CI_CHECK_AUDIT_RECORD_KIND,
    CI_CHECK_DIVERGENCE_CLASSES, CI_CHECK_FRESHNESS_CLASSES, MERGE_QUEUE_AUDIT_RECORD_KIND,
    MERGE_QUEUE_AUDIT_STATES, MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_PACKET_RECORD_KIND,
    MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_RECORD_KIND,
    MERGE_QUEUE_CI_STATUS_BROWSER_HANDOFF_AUDIT_SCHEMA_VERSION, PIPELINE_OVERLAY_AUDIT_RECORD_KIND,
    PIPELINE_OVERLAY_SUBSET_CLASSES, RUN_CONTROL_AUDIT_RECORD_KIND, RUN_CONTROL_MUTATION_MODES,
};
pub use harden_merge_rebase_cherry_pick_revert_and_reset::{
    project_diff_first_rewrite_flow_packet, DiffFirstReviewInput, DiffFirstReviewRecord,
    DiffFirstRewriteFlowPacket, DiffFirstRewriteFlowProjection, RecoveryCheckpointSummaryInput,
    RecoveryCheckpointSummaryRecord, RewriteFlowCommandInput, RewriteFlowCommandRecord,
    RewriteFlowError, RewriteFlowInput, RewriteFlowInspectionRecord, RewriteFlowSupportExportInput,
    RewriteFlowSupportExportPacket, RewriteFlowValidationError, SequenceEditOperationInput,
    SequenceEditOperationRecord, SequenceEditProposalInput, SequenceEditProposalRecord,
    CHECKPOINT_SUMMARY_STATES, DIFF_FIRST_REVIEW_RECORD_KIND, DIFF_FIRST_REVIEW_STATES,
    DIFF_FIRST_REWRITE_FLOW_PACKET_RECORD_KIND, DIFF_FIRST_REWRITE_FLOW_SCHEMA_VERSION,
    DIVERGENCE_CLASSES, PROTECTED_BRANCH_POSTURES, RECOVERY_CHECKPOINT_SUMMARY_RECORD_KIND,
    REWRITE_FLOW_APPROVAL_STATES, REWRITE_FLOW_CHECKS_FRESHNESS_STATES,
    REWRITE_FLOW_COMMAND_CLASSES, REWRITE_FLOW_COMMAND_RECORD_KIND, REWRITE_FLOW_CONSUMER_SURFACES,
    REWRITE_FLOW_INSPECTION_RECORD_KIND, REWRITE_FLOW_INVALIDATION_REASONS,
    REWRITE_FLOW_OPERATION_KINDS, REWRITE_FLOW_RECORD_KIND, REWRITE_FLOW_STATES,
    REWRITE_FLOW_SUPPORT_EXPORT_PACKET_RECORD_KIND,
};
pub use infrastructure_intelligence::{
    project_infrastructure_relationships_for_review, InfrastructureIntelligenceAlphaPage,
    InfrastructureReviewAnchorRow, InfrastructureReviewProjection,
};
pub use landing::{
    project_landing_candidate_packet, LandingCandidateError, LandingCandidateInput,
    LandingCandidatePacket, LandingCandidateProjection, LandingCandidateRecord,
    LandingCandidateValidationError, LandingCommandInput, LandingCommandRecord,
    LandingEligibilitySnapshot, LandingInspectionRecord, LandingSupportExportInput,
    LandingSupportExportPacket, MergeQueueEntryInput, MergeQueueEntryRecord,
    LANDING_APPROVAL_STATES, LANDING_AUTHORITY_CLASSES, LANDING_BLOCKED_REASONS,
    LANDING_CANDIDATE_PACKET_RECORD_KIND, LANDING_CANDIDATE_RECORD_KIND,
    LANDING_CANDIDATE_SCHEMA_VERSION, LANDING_CHECKS_FRESHNESS_STATES, LANDING_COMMAND_CLASSES,
    LANDING_COMMAND_RECORD_KIND, LANDING_CONSUMER_SURFACES, LANDING_ELIGIBILITY_STATES,
    LANDING_INSPECTION_RECORD_KIND, LANDING_INVALIDATION_REASONS, LANDING_MERGEABLE_STATES,
    LANDING_MERGE_STRATEGY_CLASSES, LANDING_POLICY_BLOCK_STATES, LANDING_PROVIDER_PUBLISH_POSTURES,
    LANDING_STALE_BASE_STATES, LANDING_SUPPORT_EXPORT_PACKET_RECORD_KIND,
    MERGE_QUEUE_ENTRY_RECORD_KIND, MERGE_QUEUE_STATES,
};
pub use review_pack_dsl::{
    project_review_pack, ReviewPackCheck, ReviewPackCheckProjection, ReviewPackError,
    ReviewPackOwnershipHint, ReviewPackOwnershipProjection, ReviewPackParityObservation,
    ReviewPackProjection, ReviewPackRecord, ReviewPackReviewInvariants, ReviewPackSupportExport,
    ReviewPackUnsupportedField, ReviewPackValidationError, REVIEW_PACK_ALPHA_DSL_VERSION,
    REVIEW_PACK_ALPHA_RECORD_KIND, REVIEW_PACK_ALPHA_SCHEMA_VERSION, REVIEW_PACK_AUTHORITY_CLASSES,
    REVIEW_PACK_CHECK_KINDS, REVIEW_PACK_CONSUMER_SURFACES, REVIEW_PACK_EXECUTION_CLASSES,
    REVIEW_PACK_OWNERSHIP_SCOPE_KINDS, REVIEW_PACK_PARITY_CLASSES, REVIEW_PACK_SEVERITY_CLASSES,
    REVIEW_PACK_UNSUPPORTED_FIELD_CLASSES,
};
pub use review_pack_parity_harness::{
    project_review_pack_parity_harness, ReviewPackParityHarnessCheckFinding,
    ReviewPackParityHarnessDowngradeProjection, ReviewPackParityHarnessDriftDowngrade,
    ReviewPackParityHarnessError, ReviewPackParityHarnessFindingProjection,
    ReviewPackParityHarnessLaneObservation, ReviewPackParityHarnessLaneProjection,
    ReviewPackParityHarnessProjection, ReviewPackParityHarnessRecord,
    ReviewPackParityHarnessReviewInvariants, ReviewPackParityHarnessSupportExport,
    ReviewPackParityHarnessValidationError, REVIEW_PACK_PARITY_HARNESS_ALPHA_HARNESS_VERSION,
    REVIEW_PACK_PARITY_HARNESS_ALPHA_RECORD_KIND, REVIEW_PACK_PARITY_HARNESS_ALPHA_SCHEMA_VERSION,
    REVIEW_PACK_PARITY_HARNESS_AUTHORITY_CLASSES, REVIEW_PACK_PARITY_HARNESS_CONSUMER_SURFACES,
    REVIEW_PACK_PARITY_HARNESS_EXPECTED_PARITY_CLASSES, REVIEW_PACK_PARITY_HARNESS_LANE_CLASSES,
    REVIEW_PACK_PARITY_HARNESS_LANE_OUTCOME_CLASSES,
    REVIEW_PACK_PARITY_HARNESS_LANE_STATUS_CLASSES,
    REVIEW_PACK_PARITY_HARNESS_OVERALL_VERDICT_CLASSES,
    REVIEW_PACK_PARITY_HARNESS_PARITY_FINDING_CLASSES,
    REVIEW_PACK_PARITY_HARNESS_ROW_DOWNGRADE_CLASSES,
};
pub use stabilize_provider_linked_object_models_snapshot_freshness_and::{
    project_provider_linked_review_stabilization_packet, ActorTargetIdentityInput,
    ActorTargetIdentityRecord, DeferredIntentInput, DeferredIntentRecord, FreshnessSnapshotInput,
    FreshnessSnapshotRecord, ProviderLinkedCommandInput, ProviderLinkedCommandRecord,
    ProviderLinkedInspectionRecord, ProviderLinkedObjectRowInput, ProviderLinkedObjectRowRecord,
    ProviderLinkedReviewStabilizationError, ProviderLinkedReviewStabilizationInput,
    ProviderLinkedReviewStabilizationPacket, ProviderLinkedReviewStabilizationProjection,
    ProviderLinkedReviewStabilizationRecord, ProviderLinkedReviewStabilizationValidationError,
    ProviderLinkedSupportExportInput, ProviderLinkedSupportExportPacket,
    ACTOR_TARGET_IDENTITY_RECORD_KIND, DEFERRED_INTENT_RECORD_KIND, FRESHNESS_DEGRADATION_CLASSES,
    FRESHNESS_SNAPSHOT_RECORD_KIND, MUTATION_MODE_CLASSES, PROVIDER_LINKED_COMMAND_CLASSES,
    PROVIDER_LINKED_COMMAND_RECORD_KIND, PROVIDER_LINKED_CONSUMER_SURFACES,
    PROVIDER_LINKED_INSPECTION_RECORD_KIND, PROVIDER_LINKED_INVALIDATION_REASONS,
    PROVIDER_LINKED_OBJECT_ROW_RECORD_KIND,
    PROVIDER_LINKED_REVIEW_STABILIZATION_PACKET_RECORD_KIND,
    PROVIDER_LINKED_REVIEW_STABILIZATION_RECORD_KIND,
    PROVIDER_LINKED_REVIEW_STABILIZATION_SCHEMA_VERSION, PROVIDER_LINKED_REVIEW_STATES,
    PROVIDER_LINKED_SUPPORT_EXPORT_PACKET_RECORD_KIND, REPLAY_SAFETY_CLASSES,
};
pub use stabilize_review_side_ai_evidence_attachment_and_safe::{
    project_ai_review_evidence_packet, AiEvidenceAttachmentInput, AiEvidenceAttachmentRecord,
    AiEvidenceCommandInput, AiEvidenceCommandRecord, AiEvidenceInspectionRecord,
    AiEvidenceSupportExportInput, AiEvidenceSupportExportPacket, AiReviewEvidenceError,
    AiReviewEvidenceInput, AiReviewEvidencePacket, AiReviewEvidenceProjection,
    AiReviewEvidenceRecord, AiReviewEvidenceValidationError, SafeSuggestionApplyInput,
    SafeSuggestionApplyRecord, SuggestionApplyCheckpointInput, SuggestionApplyCheckpointRecord,
    AI_EVIDENCE_ATTACHMENT_RECORD_KIND, AI_EVIDENCE_COMMAND_CLASSES,
    AI_EVIDENCE_COMMAND_RECORD_KIND, AI_EVIDENCE_CONSUMER_SURFACES,
    AI_EVIDENCE_INSPECTION_RECORD_KIND, AI_EVIDENCE_INVALIDATION_REASONS,
    AI_EVIDENCE_SOURCE_CLASSES, AI_EVIDENCE_STATES, AI_EVIDENCE_SUPPORT_EXPORT_PACKET_RECORD_KIND,
    AI_REVIEW_EVIDENCE_PACKET_RECORD_KIND, AI_REVIEW_EVIDENCE_RECORD_KIND,
    AI_REVIEW_EVIDENCE_SCHEMA_VERSION, CHECKPOINT_STATES, SAFE_SUGGESTION_APPLY_RECORD_KIND,
    SUGGESTION_APPLY_CHECKPOINT_RECORD_KIND, SUGGESTION_APPLY_STATES, SUGGESTION_AUTHORITY_CLASSES,
};
pub use stabilize_review_workspace_anchors_stale_base_labels_approval::{
    project_review_stabilization_packet, ApprovalInvalidationInput, ApprovalInvalidationRecord,
    MergeabilityTruthInput, MergeabilityTruthRecord, OfflineHandoffInput, OfflineHandoffRecord,
    OwnershipSignalInput, OwnershipSignalRecord, ReviewAnchorStabilityInput,
    ReviewAnchorStabilityRecord, ReviewBundleExportInput, ReviewBundleExportRecord,
    ReviewBundleImportInput, ReviewBundleImportRecord, ReviewStabilizationCommandInput,
    ReviewStabilizationCommandRecord, ReviewStabilizationError, ReviewStabilizationInput,
    ReviewStabilizationInspectionRecord, ReviewStabilizationPacket, ReviewStabilizationProjection,
    ReviewStabilizationRecord, ReviewStabilizationSupportExportInput,
    ReviewStabilizationSupportExportPacket, ReviewStabilizationValidationError,
    StaleBaseLabelInput, StaleBaseLabelRecord, ANCHOR_STABILITY_CLASSES,
    APPROVAL_INVALIDATION_RECORD_KIND, APPROVAL_INVALIDATION_TRIGGER_CLASSES, BUNDLE_EXPORT_STATES,
    BUNDLE_IMPORT_STATES, DIVERGENCE_LABEL_CLASSES, MERGEABILITY_TRUTH_CLASSES,
    MERGEABILITY_TRUTH_RECORD_KIND, OFFLINE_HANDOFF_RECORD_KIND, OFFLINE_HANDOFF_STATES,
    OWNERSHIP_SIGNAL_CLASSES, OWNERSHIP_SIGNAL_RECORD_KIND, REPLAY_EVIDENCE_CLASSES,
    REVIEW_ANCHOR_STABILITY_RECORD_KIND, REVIEW_BUNDLE_EXPORT_RECORD_KIND,
    REVIEW_BUNDLE_IMPORT_RECORD_KIND, REVIEW_STABILIZATION_COMMAND_CLASSES,
    REVIEW_STABILIZATION_COMMAND_RECORD_KIND, REVIEW_STABILIZATION_CONSUMER_SURFACES,
    REVIEW_STABILIZATION_INSPECTION_RECORD_KIND, REVIEW_STABILIZATION_INVALIDATION_REASONS,
    REVIEW_STABILIZATION_PACKET_RECORD_KIND, REVIEW_STABILIZATION_RECORD_KIND,
    REVIEW_STABILIZATION_SCHEMA_VERSION, REVIEW_STABILIZATION_SUPPORT_EXPORT_PACKET_RECORD_KIND,
    STABILIZATION_STATES, STALE_BASE_LABEL_CLASSES, STALE_BASE_LABEL_RECORD_KIND,
};
pub use stabilize_work_item_status_transition_review::{
    audit_stable_work_item_packet, seeded_stable_work_item_status_transition_packet,
    validate_stable_work_item_packet, StableOfflineHandoffInput, StableOfflineHandoffRecord,
    StablePreviewedSideEffectInput, StablePreviewedSideEffectRecord,
    StablePublishLaterContinuityInput, StablePublishLaterContinuityRecord,
    StableStatusTransitionSheetInput, StableStatusTransitionSheetRecord,
    StableWorkItemCommandInput, StableWorkItemCommandRecord, StableWorkItemDetailInput,
    StableWorkItemDetailRecord, StableWorkItemError, StableWorkItemInspectionRecord,
    StableWorkItemStatusTransitionInput, StableWorkItemStatusTransitionPacket,
    StableWorkItemStatusTransitionRecord, StableWorkItemSupportExportInput,
    StableWorkItemSupportExportPacket, StableWorkItemValidationError,
    STABLE_OFFLINE_HANDOFF_RECORD_KIND, STABLE_PUBLISH_LATER_CONTINUITY_RECORD_KIND,
    STABLE_STATUS_TRANSITION_SHEET_RECORD_KIND, STABLE_WORK_ITEM_COMMAND_RECORD_KIND,
    STABLE_WORK_ITEM_DETAIL_RECORD_KIND, STABLE_WORK_ITEM_INSPECTION_RECORD_KIND,
    STABLE_WORK_ITEM_STATUS_TRANSITION_PACKET_RECORD_KIND,
    STABLE_WORK_ITEM_STATUS_TRANSITION_RECORD_KIND,
    STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
    STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SHARED_CONTRACT_REF,
    STABLE_WORK_ITEM_SUPPORT_EXPORT_PACKET_RECORD_KIND,
};
pub use stabilize_worktree_patch_stack_and_explicit_change_object::{
    project_change_object_orchestration_packet, ChangeObjectCommandInput,
    ChangeObjectCommandRecord, ChangeObjectOrchestrationError, ChangeObjectOrchestrationInput,
    ChangeObjectOrchestrationInspectionRecord, ChangeObjectOrchestrationPacket,
    ChangeObjectOrchestrationProjection, ChangeObjectOrchestrationRecord,
    ChangeObjectOrchestrationRestartSnapshot, ChangeObjectOrchestrationSupportExportInput,
    ChangeObjectOrchestrationSupportExportPacket, ChangeObjectOrchestrationValidationError,
    MutationCheckpointInput, MutationCheckpointRecord, PatchStackOperationInput,
    PatchStackOrchestrationRecord, PublishProposalInput, PublishProposalRecord,
    WorktreeOperationInput, WorktreeOrchestrationRecord, CHANGE_OBJECT_COMMAND_RECORD_KIND,
    CHANGE_OBJECT_ORCHESTRATION_COMMAND_CLASSES, CHANGE_OBJECT_ORCHESTRATION_CONSUMER_SURFACES,
    CHANGE_OBJECT_ORCHESTRATION_FLOW_STATES, CHANGE_OBJECT_ORCHESTRATION_INVALIDATION_REASONS,
    CHANGE_OBJECT_ORCHESTRATION_OPERATION_KINDS, CHANGE_OBJECT_ORCHESTRATION_PACKET_RECORD_KIND,
    CHANGE_OBJECT_ORCHESTRATION_RECORD_KIND, CHANGE_OBJECT_ORCHESTRATION_SCHEMA_VERSION,
    CHANGE_OBJECT_ORCHESTRATION_SUPPORT_EXPORT_PACKET_RECORD_KIND, MUTATION_CHECKPOINT_RECORD_KIND,
    MUTATION_CHECKPOINT_STATES, PATCH_STACK_ORCHESTRATION_RECORD_KIND,
    POINTER_BACKED_ASSET_POSTURES, PUBLISH_PROPOSAL_RECORD_KIND, PUBLISH_READINESS_CLASSES,
    REPO_TOPOLOGY_CLASSES, WORKTREE_ORCHESTRATION_RECORD_KIND,
};
pub use workspace::{
    project_review_workspace_beta_packet, ReviewAnchorIdAlphaRecord, ReviewLocalLocator,
    ReviewPolicyContext, ReviewProviderOverlay, ReviewProviderOverlayInput,
    ReviewWorkItemLinkInput, ReviewWorkItemLinkageRecord, ReviewWorkspaceBetaError,
    ReviewWorkspaceBetaInput, ReviewWorkspaceBetaInspectionRecord, ReviewWorkspaceBetaPacket,
    ReviewWorkspaceBetaProjection, ReviewWorkspaceBetaValidationError,
    ReviewWorkspaceBrowserHandoffInput, ReviewWorkspaceBrowserHandoffRecord,
    ReviewWorkspaceCheckFreshnessInput, ReviewWorkspaceCheckFreshnessRecord,
    ReviewWorkspaceDiffEntry, ReviewWorkspaceDurableCommentAnchorInput,
    ReviewWorkspaceDurableCommentAnchorRecord, ReviewWorkspaceInspectionRecord,
    ReviewWorkspaceObjectLineageRecord, ReviewWorkspaceRecord,
    ReviewWorkspaceSearchOperatorTruthExport, ReviewWorkspaceSearchOperatorTruthExportViolation,
    ReviewWorkspaceSeedInput, ReviewWorkspaceSeedPacket, ReviewWorkspaceSupportExportInput,
    ReviewWorkspaceSupportExportPacket, REVIEW_ANCHOR_ID_ALPHA_RECORD_KIND,
    REVIEW_WORKSPACE_BETA_ANCHOR_DRIFT_STATES, REVIEW_WORKSPACE_BETA_ANCHOR_FRESHNESS_CLASSES,
    REVIEW_WORKSPACE_BETA_ANCHOR_REQUIRED_ACTIONS, REVIEW_WORKSPACE_BETA_CHECK_AUTHORITY_CLASSES,
    REVIEW_WORKSPACE_BETA_CHECK_FRESHNESS_CLASSES, REVIEW_WORKSPACE_BETA_CHECK_STATUS_CLASSES,
    REVIEW_WORKSPACE_BETA_CONSUMER_SURFACES, REVIEW_WORKSPACE_BETA_HANDOFF_DESTINATION_CLASSES,
    REVIEW_WORKSPACE_BETA_HANDOFF_REASON_CODES, REVIEW_WORKSPACE_BETA_HANDOFF_REPLAY_POSTURES,
    REVIEW_WORKSPACE_BETA_INSPECTION_RECORD_KIND, REVIEW_WORKSPACE_BETA_PACKET_RECORD_KIND,
    REVIEW_WORKSPACE_BETA_SCHEMA_VERSION, REVIEW_WORKSPACE_BROWSER_HANDOFF_RECORD_KIND,
    REVIEW_WORKSPACE_CHECK_FRESHNESS_RECORD_KIND,
    REVIEW_WORKSPACE_DURABLE_COMMENT_ANCHOR_RECORD_KIND, REVIEW_WORKSPACE_INSPECTION_RECORD_KIND,
    REVIEW_WORKSPACE_OBJECT_LINEAGE_RECORD_KIND, REVIEW_WORKSPACE_RECORD_KIND,
    REVIEW_WORKSPACE_SEARCH_OPERATOR_TRUTH_EXPORT_RECORD_KIND,
    REVIEW_WORKSPACE_SEED_PACKET_RECORD_KIND, REVIEW_WORKSPACE_SUPPORT_EXPORT_PACKET_RECORD_KIND,
    REVIEW_WORK_ITEM_LINKAGE_RECORD_KIND,
};
