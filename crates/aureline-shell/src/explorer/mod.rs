//! Virtualized file-tree model with stable node ids and explorer actions.
//!
//! The explorer surface is the protected-row consumer of three upstream
//! contracts:
//!
//! - [`aureline_vfs`] — VFS root abstraction and the five-layer identity
//!   model that supplies `(workspace_id, root_id, logical_uri)` for every
//!   reachable document.
//! - [`aureline_workspace`] — multi-root workspace shape and the
//!   workspace-readiness lifecycle that decides whether a root mount is
//!   `Loaded`, `ManifestKnown`, `Cached`, or `Unavailable`.
//! - [`aureline_reactive_state`] — reactive readiness labels that flow into
//!   the per-node [`NodeReadinessClass`] without re-deriving truth.
//!
//! This module does not invent its own filesystem identity, scope, or
//! readiness vocabulary; it projects the upstream truth into a stable,
//! virtualizable tree the chrome can render.
//!
//! ## Stable node identity
//!
//! Every node carries an [`ExplorerNodeId`] derived from the canonical
//! `(workspace_id, root_id, logical_uri)` tuple. The id survives:
//!
//! - **Virtualization churn.** Row positions are recomputed each viewport
//!   materialization; node identity is canonical and never depends on a row
//!   remaining mounted in memory.
//! - **Filtering.** Setting or clearing a filter never mutates node ids and
//!   never collapses persistent root mounts.
//! - **Restore / support export.** The id is opaque, safe in logs, safe in
//!   support bundles, and resolves back to the same node after a restart as
//!   long as the underlying logical identity is unchanged.
//!
//! ## Explorer actions
//!
//! Every action — open, reveal, refresh, basic create / remove placeholder,
//! expand, collapse, select, set-filter — is dispatched through
//! [`actions::dispatch`], which records the canonical `cmd:workspace.*`
//! command id, the touched node ids, the outcome class, and the resulting
//! selection. Records are serializable so support exports replay the same
//! truth the live runtime saw.

pub mod actions;
pub mod filter;
pub mod node;
pub mod tree;

pub use actions::{
    dispatch, ExplorerAction, ExplorerActionClass, ExplorerActionOutcome, ExplorerActionRecord,
};
pub use filter::{
    ancestry_chain, apply_filter, reveal, ExplorerFilterOutcome, ExplorerRevealOutcome,
};
pub use node::{
    ExplorerNode, ExplorerNodeId, ExplorerNodeKind, GeneratedArtifactHint, NodeReadinessClass,
    SpecialFileHint,
};
pub use tree::{
    ExplorerFilter, ExplorerTree, ExplorerTreeError, ExplorerViewport, ExplorerViewportRow,
};
