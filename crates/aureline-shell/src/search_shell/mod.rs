//! Workspace search shell: filename/path lexical surface.
//!
//! This module is the protected-row consumer of the
//! [`aureline_search::lexical`] runtime. It owns the live wiring between:
//!
//! - [`aureline_workspace::WorkspaceLifecycleMachine::readiness_inputs`]
//!   (the lifecycle-truth contract), and
//! - [`aureline_reactive_state::ReadinessLabel`] frames flowing out of the
//!   live reactive store (the readiness-truth contract).
//!
//! It does NOT invent a search-only readiness vocabulary or a search-only
//! scope vocabulary. The shell projects the canonical workspace readiness
//! and scope into one stable [`WorkspaceSearchSurfaceState`] that the
//! chrome can render. The same projection is exported as a snapshot record
//! for support bundles so the rendered truth is byte-replayable.

pub mod state;

pub use state::{
    project_scope_label, WorkspaceSearchSurfaceCard, WorkspaceSearchSurfaceCardItem,
    WorkspaceSearchSurfaceCardRow, WorkspaceSearchSurfaceLineageHint,
    WorkspaceSearchSurfaceState,
};

#[doc(inline)]
pub use crate::scope_truth::{
    ScopeCountsClass, ScopeCountsRecord, ScopeTruthChipCard, ScopeTruthSurfaceClass,
};
