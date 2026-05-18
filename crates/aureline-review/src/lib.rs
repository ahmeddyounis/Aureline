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
pub mod infrastructure_intelligence;
pub mod review_pack_dsl;
pub mod review_pack_parity_harness;
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
pub use infrastructure_intelligence::{
    project_infrastructure_relationships_for_review, InfrastructureIntelligenceAlphaPage,
    InfrastructureReviewAnchorRow, InfrastructureReviewProjection,
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
