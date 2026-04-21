//! Semantic-workspace-graph seed prototype.
//!
//! This crate is a contract-first prototype for the object model
//! frozen in
//! [`docs/graph/workspace_graph_seed.md`](https://github.com/ahmeddyounis/Aureline/blob/main/docs/graph/workspace_graph_seed.md)
//! and the boundary schema at
//! [`schemas/graph/workspace_graph_seed.schema.json`](https://github.com/ahmeddyounis/Aureline/blob/main/schemas/graph/workspace_graph_seed.schema.json).
//! Its goal is not performance or production graph storage: it is to
//! make every node class, every edge class, every evidence state,
//! every provenance class, every freshness value, every confidence
//! level, every query-family / shard-affinity / invalidation-producer
//! tag, every topology-edge / impact-reason / explainer-citation slot,
//! and every identity / label rule the doc names observable against a
//! frozen scenario table so the vocabulary cannot silently drift.
//!
//! Pieces behind this crate's public surface:
//!
//! - [`vocab`] — the frozen vocabulary enums and a hand-rolled
//!   canonical JSON renderer. Every `as_str()` matches the exact token
//!   emitted in JSON and validated by the boundary schema.
//! - [`model`] — the in-memory [`WorkspaceGraph`], [`GraphNode`],
//!   [`GraphEdge`], and supporting records. Consumers construct these
//!   and the validator enforces identity / label rules.
//! - [`validator`] — enforces the eleven identity / label rules the
//!   design doc names; returns a typed [`ValidationError`] for any
//!   violation.
//! - [`hooks`] — protected-hot-path and observability counters. The
//!   prototype counts; a production graph engine replaces the struct
//!   with a telemetry seam behind the same names.
//! - [`render`] — hand-rolled canonical JSON renderer. The rendered
//!   JSON is byte-stable across hosts (no wall-clock times, no
//!   serde dependency, synthetic monotonic tokens).
//! - [`scenarios`] — the frozen five-scenario table that mirrors the
//!   fixtures under `fixtures/graph/example_workspace_graphs/`. Each
//!   scenario is constructed in Rust, validated, rendered, and
//!   reported.
//! - [`harness`] — runs the scenario table end-to-end and emits a
//!   counts-only aggregate report that stays byte-stable across
//!   hosts.
//!
//! Known holes (no persistence layer, no real search planner, no real
//! subscription fabric, no AI context assembler, no review-pack
//! exporter) live in
//! [`prototypes/graph/README.md`](https://github.com/ahmeddyounis/Aureline/blob/main/prototypes/graph/README.md)
//! and are tracked as carry-forward items, not silent capabilities of
//! this prototype.

#![doc(html_root_url = "https://docs.rs/aureline-graph-proto/0.0.0")]

pub mod harness;
pub mod hooks;
pub mod model;
pub mod render;
pub mod scenarios;
pub mod validator;
pub mod vocab;

pub use hooks::HookCounters;
pub use model::{
    ConfidenceRollup, EdgeBody, EdgeEvidence, ExplainerCitation, FilesystemIdentity,
    FreshnessFrame, GraphEdge, GraphNode, ImpactReason, NodeBody, ProvenanceStamp, SourceAnchor,
    TopologyEdgeSlot, WorkspaceGraph, WorksetScopeRef,
};
pub use harness::{run_harness, Report, ScenarioReport};
pub use render::{graph_to_json, report_to_json, scenario_to_json};
pub use scenarios::{all_scenarios, Scenario};
pub use validator::{validate_graph, ValidationError};
pub use vocab::{
    AnchorKind, AuditEventId, CitationClass, ConfidenceLevel, EdgeClass, EdgeEvidenceState,
    EnvironmentClass, Freshness, ImpactReasonClass, InvalidationProducerTag, MissingReason,
    NodeClass, ProvenanceClass, QueryFamilyTag, ReachabilityState, ShardAffinityTag, SourceClass,
    StaleReason, SymbolVisibility, TopologyKind, TrustState, Visibility, WarmingProgressHint,
    WorksetScopeClass, WORKSPACE_GRAPH_SCHEMA_VERSION,
};
