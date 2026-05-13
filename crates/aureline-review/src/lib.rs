//! Local review workspace and diff surface contracts.
//!
//! This crate owns the first review-lane data model for Aureline. It projects
//! local Git change-list rows into inspectable diff-open targets and renders
//! alpha diff packets with syntax labels, suspicious-text cues, representation
//! safe copy actions, exact path truth, and reopen continuity records.

#![doc(html_root_url = "https://docs.rs/aureline-review/0.0.0")]

pub mod diff;

pub use diff::{
    DiffClosedSessionRecord, DiffCompareTarget, DiffCompareTargetKind, DiffCopyAction,
    DiffCopyRepresentation, DiffFileInput, DiffHunkInput, DiffHunkView, DiffLineInput,
    DiffLineKind, DiffLineView, DiffOpenTarget, DiffPathTruth, DiffReopenProjection,
    DiffScrollAnchor, DiffSuspiciousCue, DiffSyntaxClass, DiffSyntaxProjection, DiffViewMode,
    DiffViewSurfacePacket, DIFF_CLOSED_SESSION_RECORD_KIND, DIFF_OPEN_TARGET_RECORD_KIND,
    DIFF_REOPEN_PROJECTION_RECORD_KIND, DIFF_VIEW_SURFACE_PACKET_RECORD_KIND,
};
