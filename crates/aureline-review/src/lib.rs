//! Local review workspace and diff surface contracts.
//!
//! This crate owns the first review-lane data model for Aureline. It projects
//! local Git change-list rows into inspectable diff-open targets and renders
//! alpha diff packets with syntax labels, suspicious-text cues, representation
//! safe copy actions, exact path truth, reopen continuity records, review
//! workspace seeds, stable row anchors, work-item relation projections, and
//! shared collection-view batch-review packets.

#![doc(html_root_url = "https://docs.rs/aureline-review/0.0.0")]

pub mod collections;
pub mod diff;
pub mod workspace;

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
pub use workspace::{
    ReviewAnchorIdAlphaRecord, ReviewLocalLocator, ReviewPolicyContext, ReviewProviderOverlay,
    ReviewProviderOverlayInput, ReviewWorkItemLinkInput, ReviewWorkItemLinkageRecord,
    ReviewWorkspaceDiffEntry, ReviewWorkspaceInspectionRecord, ReviewWorkspaceRecord,
    ReviewWorkspaceSeedInput, ReviewWorkspaceSeedPacket, REVIEW_ANCHOR_ID_ALPHA_RECORD_KIND,
    REVIEW_WORKSPACE_INSPECTION_RECORD_KIND, REVIEW_WORKSPACE_RECORD_KIND,
    REVIEW_WORKSPACE_SEED_PACKET_RECORD_KIND, REVIEW_WORK_ITEM_LINKAGE_RECORD_KIND,
};
