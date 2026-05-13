//! Semantic graph storage and alpha query-family runtime.
//!
//! This crate is the first runtime consumer of the graph seed object model. It
//! stores validated [`aureline_graph_proto::WorkspaceGraph`] snapshots and
//! exposes a bounded query-family surface for launch-wedge symbols, imports,
//! ownership edges, and the future impact/explainer packet lanes.

mod query;
mod store;

pub use query::{
    GraphAlphaQueryClass, GraphPartialTruthCause, GraphQueryEnvelope, GraphQueryFamilyDescriptor,
    GraphQueryReadiness, GraphQueryRequest, GraphQueryRow, GraphQueryRowClass,
    GRAPH_QUERY_FAMILY_ALPHA_VERSION,
};
pub use store::{GraphStore, GraphStoreError};

pub use aureline_graph_proto::{
    ConfidenceLevel, EdgeClass, EdgeEvidenceState, Freshness, FreshnessFrame, GraphEdge, GraphNode,
    NodeBody, NodeClass, QueryFamilyTag, WorksetScopeRef,
};
