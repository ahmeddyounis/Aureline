//! Canonical M5 topology-identity packet: the stable node and edge identities every
//! code-understanding surface points at, so the same graph object survives canvas, list,
//! table, breadcrumb, review, and support/export views without lossy translation.
//!
//! Where [`crate::m5_workset_scope`] answers *what slice am I looking at?* and
//! [`crate::m5_graph_governance`] freezes *which depth claim a lane may publish*, this packet
//! answers the question search, review, AI, onboarding, docs, and support all ask next: *which
//! exact graph object is this, and how do I point at it again later?* It carries the canonical
//! [`TopologyNodeIdentity`] and [`TopologyEdgeIdentity`] records — each with a stable id, a
//! kind, a label/path, namespace/workspace refs, freshness, confidence, a source class,
//! contract badges, and an export-safe permalink — plus one [`TopologySurfaceBinding`] per
//! surface that resolves those same ids rather than minting transient per-view identities.
//!
//! Four invariants hold across the packet:
//!
//! - **Stable identity across surfaces.** Every [`TopologySurfaceBinding`] resolves only ids
//!   that are declared in the packet, and every declared node and edge is resolvable from at
//!   least one non-canvas surface, so a map, breadcrumb, explainer, review surface, and support
//!   packet all point at the same [`TopologyNodeIdentity::node_id`] and
//!   [`TopologyEdgeIdentity::edge_id`].
//! - **No canvas-only source of truth.** Anything the canvas ([`TopologySurface::MapCanvas`])
//!   resolves must also be resolvable from a non-canvas accessible view, so a visual map never
//!   becomes the sole owner of an identity.
//! - **Approximate relations stay explicit.** A non-[`RelationFidelity::Exact`] edge —
//!   approximate, imported, partial, stale, or blocked — must carry a
//!   [`TopologyEdgeIdentity::fidelity_reason`], so presentation never implies stronger
//!   certainty than the underlying graph carries.
//! - **Export-safe references.** Every node and edge carries a unique
//!   [`TopologyNodeIdentity::export_permalink`] that embeds its canonical id, so support,
//!   issue reports, review comments, and evidence packets can point at exact topology objects
//!   without screenshots or hand-written descriptions.
//!
//! The packet binds upstream to the canonical graph-depth governance matrix and the
//! workset-scope packet it extends, stamps every surface binding with the active scope
//! snapshot, and exports release-evidence, help-surface, docs-badge, and support-export refs so
//! those surfaces narrow from one packet rather than parallel spreadsheets.
//!
//! The packet is checked in at `artifacts/graph/m5/m5-topology-identity.json` and embedded
//! here. It is metadata-only: every field is a typed state, a count, a label, or an opaque ref,
//! and it carries no credential bodies, raw provider payloads, or graph node contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::explainers::WorksetScopeMode;

/// Supported M5 topology-identity packet schema version.
pub const M5_TOPOLOGY_IDENTITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_TOPOLOGY_IDENTITY_RECORD_KIND: &str = "m5_topology_identity_packet";

/// Repo-relative path to the checked-in packet.
pub const M5_TOPOLOGY_IDENTITY_PATH: &str = "artifacts/graph/m5/m5-topology-identity.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_TOPOLOGY_IDENTITY_SCHEMA_REF: &str = "schemas/graph/m5-topology-identity.schema.json";

/// Repo-relative path to the companion document.
pub const M5_TOPOLOGY_IDENTITY_DOC_REF: &str = "docs/graph/m5/m5-topology-identity.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_TOPOLOGY_IDENTITY_FIXTURE_DIR: &str = "fixtures/graph/m5/m5-topology-identity";

/// Repo-relative path to the canonical graph-depth governance matrix this packet extends.
pub const M5_TOPOLOGY_IDENTITY_GOVERNANCE_MATRIX_REF: &str =
    "artifacts/graph/m5/m5-graph-governance.json";

/// Repo-relative path to the canonical workset-scope packet this topology is bound to.
pub const M5_TOPOLOGY_IDENTITY_SCOPE_PACKET_REF: &str = "artifacts/graph/m5/m5-workset-scope.json";

/// Embedded checked-in packet JSON.
pub const M5_TOPOLOGY_IDENTITY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/graph/m5/m5-topology-identity.json"
));

/// The kind of graph object a topology node identity names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyNodeKind {
    /// A workspace file.
    File,
    /// A workspace directory.
    Directory,
    /// A code symbol such as a function, type, or method.
    Symbol,
    /// A module, crate, or package grouping.
    Module,
    /// A documentation or knowledge object.
    Doc,
    /// An ownership descriptor such as a team or codeowner rule.
    Ownership,
    /// A connected provider resource.
    ProviderResource,
    /// A workset-scope object.
    WorksetScope,
}

impl TopologyNodeKind {
    /// Every node kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::File,
        Self::Directory,
        Self::Symbol,
        Self::Module,
        Self::Doc,
        Self::Ownership,
        Self::ProviderResource,
        Self::WorksetScope,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Directory => "directory",
            Self::Symbol => "symbol",
            Self::Module => "module",
            Self::Doc => "doc",
            Self::Ownership => "ownership",
            Self::ProviderResource => "provider_resource",
            Self::WorksetScope => "workset_scope",
        }
    }
}

/// The kind of relation a topology edge identity names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyEdgeKind {
    /// An import or dependency-manifest relation.
    Imports,
    /// A call relation between symbols.
    Calls,
    /// A containment relation (module contains file, file contains symbol).
    Contains,
    /// An impact relation surfaced by an impact query.
    Impacts,
    /// An ownership relation between an object and its owner.
    OwnedBy,
    /// A reference relation (docs reference code, code references docs).
    References,
    /// A coarse dependency relation between groupings.
    DependsOn,
}

impl TopologyEdgeKind {
    /// Every edge kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Imports,
        Self::Calls,
        Self::Contains,
        Self::Impacts,
        Self::OwnedBy,
        Self::References,
        Self::DependsOn,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Imports => "imports",
            Self::Calls => "calls",
            Self::Contains => "contains",
            Self::Impacts => "impacts",
            Self::OwnedBy => "owned_by",
            Self::References => "references",
            Self::DependsOn => "depends_on",
        }
    }
}

/// How exactly an edge is known, surfaced explicitly so presentation never implies stronger
/// certainty than the graph carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationFidelity {
    /// Direct, authoritative evidence supports the edge.
    Exact,
    /// The edge is heuristic or inferred and may be incomplete.
    Approximate,
    /// The edge was hydrated from imported evidence rather than indexed locally.
    Imported,
    /// The edge is truncated at the active workset or scope boundary.
    Partial,
    /// The edge is older than the current revision of one of its endpoints.
    Stale,
    /// The edge is known to exist but is withheld by policy or a missing connection.
    Blocked,
}

impl RelationFidelity {
    /// Every fidelity, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Exact,
        Self::Approximate,
        Self::Imported,
        Self::Partial,
        Self::Stale,
        Self::Blocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::Imported => "imported",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Blocked => "blocked",
        }
    }

    /// Whether the fidelity is exact and therefore needs no disclosure reason.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Whether the fidelity must carry an explicit disclosure reason.
    pub const fn requires_disclosure(self) -> bool {
        !self.is_exact()
    }
}

/// Where a topology identity's evidence came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologySourceClass {
    /// Indexed from local workspace state.
    Indexed,
    /// Hydrated from an imported bundle.
    Imported,
    /// Inferred by a heuristic or AI producer.
    Inferred,
    /// Read from a connected provider.
    Provider,
    /// Supplied by a manual annotation or ownership rule.
    Annotation,
}

impl TopologySourceClass {
    /// Every source class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Indexed,
        Self::Imported,
        Self::Inferred,
        Self::Provider,
        Self::Annotation,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Indexed => "indexed",
            Self::Imported => "imported",
            Self::Inferred => "inferred",
            Self::Provider => "provider",
            Self::Annotation => "annotation",
        }
    }
}

/// An accessible surface that renders topology identities.
///
/// The closed vocabulary is exhaustive: every surface that can point at a graph object carries
/// a binding so its identity resolution stays explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologySurface {
    /// The visual map/canvas projection.
    MapCanvas,
    /// A flat list projection.
    List,
    /// A non-canvas edge/node table projection.
    Table,
    /// A breadcrumb trail projection.
    Breadcrumb,
    /// The architecture explainer projection.
    Explainer,
    /// The review-explanation projection.
    Review,
    /// The support/export projection.
    SupportExport,
}

impl TopologySurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::MapCanvas,
        Self::List,
        Self::Table,
        Self::Breadcrumb,
        Self::Explainer,
        Self::Review,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MapCanvas => "map_canvas",
            Self::List => "list",
            Self::Table => "table",
            Self::Breadcrumb => "breadcrumb",
            Self::Explainer => "explainer",
            Self::Review => "review",
            Self::SupportExport => "support_export",
        }
    }

    /// Whether this surface is the visual canvas.
    pub const fn is_canvas(self) -> bool {
        matches!(self, Self::MapCanvas)
    }
}

/// A contract badge that travels with a topology identity across every view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractBadge {
    /// The object id is stable across canvas, list, table, breadcrumb, review, and export.
    StableIdentity,
    /// The object carries an export-safe permalink.
    ExportSafePermalink,
    /// The object is bound to the active workset scope.
    ScopeBound,
    /// The object carries a freshness label.
    FreshnessLabeled,
    /// The object carries a confidence label.
    ConfidenceLabeled,
    /// The object carries a source-class label.
    SourceLabeled,
}

impl ContractBadge {
    /// Every badge, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StableIdentity,
        Self::ExportSafePermalink,
        Self::ScopeBound,
        Self::FreshnessLabeled,
        Self::ConfidenceLabeled,
        Self::SourceLabeled,
    ];

    /// Badges every node and edge must carry so the identity contract holds across views.
    pub const REQUIRED: [Self; 2] = [Self::StableIdentity, Self::ExportSafePermalink];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableIdentity => "stable_identity",
            Self::ExportSafePermalink => "export_safe_permalink",
            Self::ScopeBound => "scope_bound",
            Self::FreshnessLabeled => "freshness_labeled",
            Self::ConfidenceLabeled => "confidence_labeled",
            Self::SourceLabeled => "source_labeled",
        }
    }
}

/// The active scope snapshot every topology surface binding is stamped with.
///
/// This is the replay anchor copied from the workset-scope packet: its
/// [`TopologyScopeAnchor::snapshot_id`] and [`TopologyScopeAnchor::scope_id`] are recorded on
/// every binding so support export and replay can reconstruct the exact slice the user queried.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TopologyScopeAnchor {
    /// Stable snapshot id selections and results are stamped with.
    pub snapshot_id: String,
    /// Active scope id the topology renders.
    pub scope_id: String,
    /// UTC date the snapshot was taken.
    pub taken_as_of: String,
    /// Whether the active scope is full or sparse.
    pub scope_mode: WorksetScopeMode,
}

/// A stable topology node identity that survives every view change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TopologyNodeIdentity {
    /// Canonical, stable node id.
    pub node_id: String,
    /// Node kind.
    pub kind: TopologyNodeKind,
    /// Redaction-aware display label or path.
    pub display_label: String,
    /// Namespace ref (for example a repository) the node belongs to.
    pub namespace_ref: String,
    /// Workspace ref the node belongs to.
    pub workspace_ref: String,
    /// Freshness token copied from the graph object.
    pub freshness: String,
    /// Confidence token copied from the graph object.
    pub confidence: String,
    /// Where the node identity was sourced from.
    pub source_class: TopologySourceClass,
    /// Contract badges that travel with the node across views.
    pub contract_badges: Vec<ContractBadge>,
    /// Export-safe, copy-safe permalink that embeds the canonical node id.
    pub export_permalink: String,
}

impl TopologyNodeIdentity {
    /// Whether the permalink is non-empty and embeds the canonical node id.
    pub fn permalink_is_export_safe(&self) -> bool {
        !self.export_permalink.trim().is_empty() && self.export_permalink.contains(&self.node_id)
    }

    /// Whether the node carries every required contract badge.
    pub fn carries_required_badges(&self) -> bool {
        ContractBadge::REQUIRED
            .iter()
            .all(|badge| self.contract_badges.contains(badge))
    }
}

/// A stable topology edge identity that survives every view change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TopologyEdgeIdentity {
    /// Canonical, stable edge id.
    pub edge_id: String,
    /// Edge kind.
    pub kind: TopologyEdgeKind,
    /// Canonical source node id; must reference a declared node.
    pub from_node_id: String,
    /// Canonical target node id; must reference a declared node.
    pub to_node_id: String,
    /// How exactly the edge is known.
    pub relation_fidelity: RelationFidelity,
    /// Explicit disclosure reason; required for every non-exact fidelity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fidelity_reason: Option<String>,
    /// Freshness token copied from the graph object.
    pub freshness: String,
    /// Confidence token copied from the graph object.
    pub confidence: String,
    /// Where the edge identity was sourced from.
    pub source_class: TopologySourceClass,
    /// Contract badges that travel with the edge across views.
    pub contract_badges: Vec<ContractBadge>,
    /// Export-safe, copy-safe permalink that embeds the canonical edge id.
    pub export_permalink: String,
}

impl TopologyEdgeIdentity {
    /// Whether a non-exact edge carries an explicit disclosure reason.
    pub fn fidelity_is_labeled(&self) -> bool {
        if self.relation_fidelity.is_exact() {
            return true;
        }
        self.fidelity_reason
            .as_ref()
            .is_some_and(|reason| !reason.trim().is_empty())
    }

    /// Whether the permalink is non-empty and embeds the canonical edge id.
    pub fn permalink_is_export_safe(&self) -> bool {
        !self.export_permalink.trim().is_empty() && self.export_permalink.contains(&self.edge_id)
    }

    /// Whether the edge carries every required contract badge.
    pub fn carries_required_badges(&self) -> bool {
        ContractBadge::REQUIRED
            .iter()
            .all(|badge| self.contract_badges.contains(badge))
    }
}

/// One accessible surface bound to the active scope snapshot, resolving shared topology ids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TopologySurfaceBinding {
    /// Stable binding id inside the packet.
    pub binding_id: String,
    /// Surface this binding renders.
    pub surface: TopologySurface,
    /// Whether the surface is the visual canvas; must match [`TopologySurface::is_canvas`].
    pub is_canvas: bool,
    /// Snapshot id this surface is bound to; must equal the active snapshot id.
    pub snapshot_id: String,
    /// Scope id this surface renders; must equal the active scope id.
    pub scope_id: String,
    /// Canonical node ids this surface resolves; every id must be declared in the packet.
    #[serde(default)]
    pub resolves_node_ids: Vec<String>,
    /// Canonical edge ids this surface resolves; every id must be declared in the packet.
    #[serde(default)]
    pub resolves_edge_ids: Vec<String>,
    /// Ref to the surface artifact that ingests these identities.
    pub consumer_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5TopologyIdentitySummary {
    /// Total declared nodes.
    pub node_count: usize,
    /// Total declared edges.
    pub edge_count: usize,
    /// Total surface bindings.
    pub surface_binding_count: usize,
    /// Number of distinct surfaces bound.
    pub surface_count: usize,
    /// Bindings that are the visual canvas.
    pub canvas_surface_count: usize,
    /// Bindings that are non-canvas accessible views.
    pub non_canvas_surface_count: usize,
    /// Edges backed by exact evidence.
    pub exact_edge_count: usize,
    /// Edges labeled approximate.
    pub approximate_edge_count: usize,
    /// Edges labeled imported.
    pub imported_edge_count: usize,
    /// Edges labeled partial.
    pub partial_edge_count: usize,
    /// Edges labeled stale.
    pub stale_edge_count: usize,
    /// Edges labeled blocked.
    pub blocked_edge_count: usize,
    /// Non-exact edges that carry an explicit disclosure reason.
    pub labeled_nonexact_edge_count: usize,
    /// Nodes that carry an export-safe permalink.
    pub nodes_with_permalink: usize,
    /// Edges that carry an export-safe permalink.
    pub edges_with_permalink: usize,
}

/// A redaction-safe export row projected from a topology node or edge identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TopologyIdentityExportRow {
    /// Object class, `node` or `edge`.
    pub object_class: String,
    /// Canonical object id.
    pub object_id: String,
    /// Object kind token.
    pub kind: String,
    /// Export-safe permalink that points at the exact object.
    pub permalink: String,
    /// Relation fidelity token for edges; `None` for nodes.
    pub relation_fidelity: Option<String>,
    /// Freshness token.
    pub freshness: String,
    /// Confidence token.
    pub confidence: String,
    /// Source-class token.
    pub source_class: String,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet — the identity index downstream surfaces
/// render instead of restating topology objects by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TopologyIdentityExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Active snapshot id every binding is stamped with.
    pub snapshot_id: String,
    /// Active scope id.
    pub scope_id: String,
    /// Active scope-mode token.
    pub scope_mode: String,
    /// Projected node and edge rows.
    pub objects: Vec<M5TopologyIdentityExportRow>,
    /// Whether every node and edge carries an export-safe permalink.
    pub all_objects_have_permalink: bool,
    /// Whether every non-exact edge carries an explicit disclosure reason.
    pub all_nonexact_edges_labeled: bool,
    /// Whether every surface binding resolves only declared shared identities.
    pub all_surfaces_resolve_shared_identity: bool,
    /// Whether the canvas owns no identity that a non-canvas surface cannot resolve.
    pub canvas_is_not_source_of_truth: bool,
}

/// The typed M5 topology-identity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5TopologyIdentityPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the canonical graph-depth governance matrix this packet extends.
    pub governance_matrix_ref: String,
    /// Ref to the canonical workset-scope packet this topology is bound to.
    pub scope_packet_ref: String,
    /// Ref to the graph-conformance suite backing the packet.
    pub conformance_ref: String,
    /// Ref binding this packet into the release-evidence surface.
    pub release_evidence_ref: String,
    /// Ref binding this packet into the help/service-health surface.
    pub help_surface_ref: String,
    /// Ref binding this packet into the docs-badge surface.
    pub docs_badge_ref: String,
    /// Ref binding this packet into the support-export surface.
    pub support_export_ref: String,
    /// Closed node-kind vocabulary.
    pub node_kinds: Vec<TopologyNodeKind>,
    /// Closed edge-kind vocabulary.
    pub edge_kinds: Vec<TopologyEdgeKind>,
    /// Closed relation-fidelity vocabulary.
    pub relation_fidelities: Vec<RelationFidelity>,
    /// Closed source-class vocabulary.
    pub source_classes: Vec<TopologySourceClass>,
    /// Closed surface vocabulary.
    pub surfaces: Vec<TopologySurface>,
    /// Closed contract-badge vocabulary.
    pub contract_badges: Vec<ContractBadge>,
    /// The active scope snapshot every surface binds to.
    pub active_scope: TopologyScopeAnchor,
    /// Declared topology node identities.
    #[serde(default)]
    pub nodes: Vec<TopologyNodeIdentity>,
    /// Declared topology edge identities.
    #[serde(default)]
    pub edges: Vec<TopologyEdgeIdentity>,
    /// Surface bindings, one per claimed surface.
    #[serde(default)]
    pub surface_bindings: Vec<TopologySurfaceBinding>,
    /// Summary counts.
    pub summary: M5TopologyIdentitySummary,
}

impl M5TopologyIdentityPacket {
    /// Returns the node identity for a node id.
    pub fn node(&self, node_id: &str) -> Option<&TopologyNodeIdentity> {
        self.nodes.iter().find(|n| n.node_id == node_id)
    }

    /// Returns the edge identity for an edge id.
    pub fn edge(&self, edge_id: &str) -> Option<&TopologyEdgeIdentity> {
        self.edges.iter().find(|e| e.edge_id == edge_id)
    }

    /// Returns the binding for a surface.
    pub fn surface_binding(&self, surface: TopologySurface) -> Option<&TopologySurfaceBinding> {
        self.surface_bindings.iter().find(|b| b.surface == surface)
    }

    /// Returns the export-safe permalink for a node id.
    pub fn permalink_for_node(&self, node_id: &str) -> Option<&str> {
        self.node(node_id).map(|n| n.export_permalink.as_str())
    }

    /// Returns the export-safe permalink for an edge id.
    pub fn permalink_for_edge(&self, edge_id: &str) -> Option<&str> {
        self.edge(edge_id).map(|e| e.export_permalink.as_str())
    }

    /// Whether every node and edge carries an export-safe permalink.
    pub fn all_objects_have_permalink(&self) -> bool {
        self.nodes
            .iter()
            .all(TopologyNodeIdentity::permalink_is_export_safe)
            && self
                .edges
                .iter()
                .all(TopologyEdgeIdentity::permalink_is_export_safe)
    }

    /// Whether every non-exact edge carries an explicit disclosure reason.
    pub fn all_nonexact_edges_labeled(&self) -> bool {
        self.edges
            .iter()
            .all(TopologyEdgeIdentity::fidelity_is_labeled)
    }

    /// Whether every surface binding resolves only ids declared in the packet.
    pub fn all_surfaces_resolve_shared_identity(&self) -> bool {
        let node_ids: BTreeSet<&str> = self.nodes.iter().map(|n| n.node_id.as_str()).collect();
        let edge_ids: BTreeSet<&str> = self.edges.iter().map(|e| e.edge_id.as_str()).collect();
        self.surface_bindings.iter().all(|binding| {
            binding
                .resolves_node_ids
                .iter()
                .all(|id| node_ids.contains(id.as_str()))
                && binding
                    .resolves_edge_ids
                    .iter()
                    .all(|id| edge_ids.contains(id.as_str()))
        })
    }

    /// Node ids resolved by at least one non-canvas surface.
    fn non_canvas_node_ids(&self) -> BTreeSet<&str> {
        self.surface_bindings
            .iter()
            .filter(|binding| !binding.surface.is_canvas())
            .flat_map(|binding| binding.resolves_node_ids.iter())
            .map(String::as_str)
            .collect()
    }

    /// Edge ids resolved by at least one non-canvas surface.
    fn non_canvas_edge_ids(&self) -> BTreeSet<&str> {
        self.surface_bindings
            .iter()
            .filter(|binding| !binding.surface.is_canvas())
            .flat_map(|binding| binding.resolves_edge_ids.iter())
            .map(String::as_str)
            .collect()
    }

    /// Whether the canvas owns no identity a non-canvas accessible view cannot resolve.
    ///
    /// This is the guardrail probe: every node and edge the canvas resolves must also be
    /// resolvable from a non-canvas surface, so a visual map never becomes the source of truth.
    pub fn canvas_is_not_source_of_truth(&self) -> bool {
        let non_canvas_nodes = self.non_canvas_node_ids();
        let non_canvas_edges = self.non_canvas_edge_ids();
        self.surface_bindings
            .iter()
            .filter(|binding| binding.surface.is_canvas())
            .all(|binding| {
                binding
                    .resolves_node_ids
                    .iter()
                    .all(|id| non_canvas_nodes.contains(id.as_str()))
                    && binding
                        .resolves_edge_ids
                        .iter()
                        .all(|id| non_canvas_edges.contains(id.as_str()))
            })
    }

    /// Recomputes the summary block from the nodes, edges, and bindings.
    pub fn computed_summary(&self) -> M5TopologyIdentitySummary {
        let fidelity_count = |fidelity: RelationFidelity| {
            self.edges
                .iter()
                .filter(|e| e.relation_fidelity == fidelity)
                .count()
        };
        let distinct_surfaces: BTreeSet<TopologySurface> =
            self.surface_bindings.iter().map(|b| b.surface).collect();
        M5TopologyIdentitySummary {
            node_count: self.nodes.len(),
            edge_count: self.edges.len(),
            surface_binding_count: self.surface_bindings.len(),
            surface_count: distinct_surfaces.len(),
            canvas_surface_count: self
                .surface_bindings
                .iter()
                .filter(|b| b.surface.is_canvas())
                .count(),
            non_canvas_surface_count: self
                .surface_bindings
                .iter()
                .filter(|b| !b.surface.is_canvas())
                .count(),
            exact_edge_count: fidelity_count(RelationFidelity::Exact),
            approximate_edge_count: fidelity_count(RelationFidelity::Approximate),
            imported_edge_count: fidelity_count(RelationFidelity::Imported),
            partial_edge_count: fidelity_count(RelationFidelity::Partial),
            stale_edge_count: fidelity_count(RelationFidelity::Stale),
            blocked_edge_count: fidelity_count(RelationFidelity::Blocked),
            labeled_nonexact_edge_count: self
                .edges
                .iter()
                .filter(|e| e.relation_fidelity.requires_disclosure() && e.fidelity_is_labeled())
                .count(),
            nodes_with_permalink: self
                .nodes
                .iter()
                .filter(|n| n.permalink_is_export_safe())
                .count(),
            edges_with_permalink: self
                .edges
                .iter()
                .filter(|e| e.permalink_is_export_safe())
                .count(),
        }
    }

    /// Produces the identity index downstream surfaces — release evidence,
    /// help/service-health, docs badges, and support exports — render instead of restating
    /// topology objects by hand.
    pub fn export_projection(&self) -> M5TopologyIdentityExportProjection {
        let mut objects = Vec::with_capacity(self.nodes.len() + self.edges.len());
        for node in &self.nodes {
            objects.push(M5TopologyIdentityExportRow {
                object_class: "node".to_owned(),
                object_id: node.node_id.clone(),
                kind: node.kind.as_str().to_owned(),
                permalink: node.export_permalink.clone(),
                relation_fidelity: None,
                freshness: node.freshness.clone(),
                confidence: node.confidence.clone(),
                source_class: node.source_class.as_str().to_owned(),
                summary: format!(
                    "node {} ({}): {}, {}/{}, {}",
                    node.node_id,
                    node.kind.as_str(),
                    node.source_class.as_str(),
                    node.freshness,
                    node.confidence,
                    node.export_permalink
                ),
            });
        }
        for edge in &self.edges {
            objects.push(M5TopologyIdentityExportRow {
                object_class: "edge".to_owned(),
                object_id: edge.edge_id.clone(),
                kind: edge.kind.as_str().to_owned(),
                permalink: edge.export_permalink.clone(),
                relation_fidelity: Some(edge.relation_fidelity.as_str().to_owned()),
                freshness: edge.freshness.clone(),
                confidence: edge.confidence.clone(),
                source_class: edge.source_class.as_str().to_owned(),
                summary: format!(
                    "edge {} ({}) {} -> {}: fidelity {}, {}",
                    edge.edge_id,
                    edge.kind.as_str(),
                    edge.from_node_id,
                    edge.to_node_id,
                    edge.relation_fidelity.as_str(),
                    edge.export_permalink
                ),
            });
        }
        M5TopologyIdentityExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            snapshot_id: self.active_scope.snapshot_id.clone(),
            scope_id: self.active_scope.scope_id.clone(),
            scope_mode: self.active_scope.scope_mode.as_str().to_owned(),
            objects,
            all_objects_have_permalink: self.all_objects_have_permalink(),
            all_nonexact_edges_labeled: self.all_nonexact_edges_labeled(),
            all_surfaces_resolve_shared_identity: self.all_surfaces_resolve_shared_identity(),
            canvas_is_not_source_of_truth: self.canvas_is_not_source_of_truth(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5TopologyIdentityViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_anchor(&mut violations);
        self.validate_nodes(&mut violations);
        self.validate_edges(&mut violations);
        self.validate_bindings(&mut violations);
        self.validate_permalink_uniqueness(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5TopologyIdentityViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5TopologyIdentityViolation>) {
        if self.schema_version != M5_TOPOLOGY_IDENTITY_SCHEMA_VERSION {
            violations.push(M5TopologyIdentityViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_TOPOLOGY_IDENTITY_RECORD_KIND {
            violations.push(M5TopologyIdentityViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("governance_matrix_ref", &self.governance_matrix_ref),
            ("scope_packet_ref", &self.scope_packet_ref),
            ("conformance_ref", &self.conformance_ref),
            ("release_evidence_ref", &self.release_evidence_ref),
            ("help_surface_ref", &self.help_surface_ref),
            ("docs_badge_ref", &self.docs_badge_ref),
            ("support_export_ref", &self.support_export_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5TopologyIdentityViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        // The packet must bind upstream to the canonical governance matrix and workset-scope
        // packet it extends, so the shared identity model has one provenance root.
        if self.governance_matrix_ref != M5_TOPOLOGY_IDENTITY_GOVERNANCE_MATRIX_REF {
            violations.push(M5TopologyIdentityViolation::GovernanceMatrixRefMismatch);
        }
        if self.scope_packet_ref != M5_TOPOLOGY_IDENTITY_SCOPE_PACKET_REF {
            violations.push(M5TopologyIdentityViolation::ScopePacketRefMismatch);
        }
        for (field, ok) in [
            (
                "node_kinds",
                self.node_kinds == TopologyNodeKind::ALL.to_vec(),
            ),
            (
                "edge_kinds",
                self.edge_kinds == TopologyEdgeKind::ALL.to_vec(),
            ),
            (
                "relation_fidelities",
                self.relation_fidelities == RelationFidelity::ALL.to_vec(),
            ),
            (
                "source_classes",
                self.source_classes == TopologySourceClass::ALL.to_vec(),
            ),
            ("surfaces", self.surfaces == TopologySurface::ALL.to_vec()),
            (
                "contract_badges",
                self.contract_badges == ContractBadge::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5TopologyIdentityViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_anchor(&self, violations: &mut Vec<M5TopologyIdentityViolation>) {
        for (field, value) in [
            ("snapshot_id", &self.active_scope.snapshot_id),
            ("scope_id", &self.active_scope.scope_id),
            ("taken_as_of", &self.active_scope.taken_as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5TopologyIdentityViolation::EmptyField {
                    id: "<active_scope>".to_owned(),
                    field_name: field,
                });
            }
        }
    }

    fn validate_nodes(&self, violations: &mut Vec<M5TopologyIdentityViolation>) {
        let mut seen_ids = BTreeSet::new();
        for node in &self.nodes {
            if !seen_ids.insert(node.node_id.clone()) {
                violations.push(M5TopologyIdentityViolation::DuplicateNodeId {
                    node_id: node.node_id.clone(),
                });
            }
            for (field, value) in [
                ("node_id", &node.node_id),
                ("display_label", &node.display_label),
                ("namespace_ref", &node.namespace_ref),
                ("workspace_ref", &node.workspace_ref),
                ("freshness", &node.freshness),
                ("confidence", &node.confidence),
                ("export_permalink", &node.export_permalink),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5TopologyIdentityViolation::EmptyField {
                        id: node.node_id.clone(),
                        field_name: field,
                    });
                }
            }
            // Every node must point at itself with an export-safe permalink so support, issue
            // reports, review comments, and evidence packets can reference the exact object.
            if !node.permalink_is_export_safe() {
                violations.push(M5TopologyIdentityViolation::UnsafeNodePermalink {
                    node_id: node.node_id.clone(),
                });
            }
            if !node.carries_required_badges() {
                violations.push(M5TopologyIdentityViolation::MissingRequiredBadges {
                    id: node.node_id.clone(),
                });
            }
        }
    }

    fn validate_edges(&self, violations: &mut Vec<M5TopologyIdentityViolation>) {
        let node_ids: BTreeSet<&str> = self.nodes.iter().map(|n| n.node_id.as_str()).collect();
        let mut seen_ids = BTreeSet::new();
        for edge in &self.edges {
            if !seen_ids.insert(edge.edge_id.clone()) {
                violations.push(M5TopologyIdentityViolation::DuplicateEdgeId {
                    edge_id: edge.edge_id.clone(),
                });
            }
            for (field, value) in [
                ("edge_id", &edge.edge_id),
                ("from_node_id", &edge.from_node_id),
                ("to_node_id", &edge.to_node_id),
                ("freshness", &edge.freshness),
                ("confidence", &edge.confidence),
                ("export_permalink", &edge.export_permalink),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5TopologyIdentityViolation::EmptyField {
                        id: edge.edge_id.clone(),
                        field_name: field,
                    });
                }
            }
            // An edge must connect two declared nodes; a dangling endpoint would be an identity
            // the rest of the packet cannot resolve.
            if !node_ids.contains(edge.from_node_id.as_str()) {
                violations.push(M5TopologyIdentityViolation::DanglingEdgeEndpoint {
                    edge_id: edge.edge_id.clone(),
                    node_id: edge.from_node_id.clone(),
                });
            }
            if !node_ids.contains(edge.to_node_id.as_str()) {
                violations.push(M5TopologyIdentityViolation::DanglingEdgeEndpoint {
                    edge_id: edge.edge_id.clone(),
                    node_id: edge.to_node_id.clone(),
                });
            }
            // An approximate, imported, partial, stale, or blocked edge must carry an explicit
            // disclosure reason so presentation never implies stronger certainty than the graph.
            if !edge.fidelity_is_labeled() {
                violations.push(M5TopologyIdentityViolation::UnlabeledNonExactRelation {
                    edge_id: edge.edge_id.clone(),
                    fidelity: edge.relation_fidelity.as_str(),
                });
            }
            if !edge.permalink_is_export_safe() {
                violations.push(M5TopologyIdentityViolation::UnsafeEdgePermalink {
                    edge_id: edge.edge_id.clone(),
                });
            }
            if !edge.carries_required_badges() {
                violations.push(M5TopologyIdentityViolation::MissingRequiredBadges {
                    id: edge.edge_id.clone(),
                });
            }
        }
    }

    fn validate_bindings(&self, violations: &mut Vec<M5TopologyIdentityViolation>) {
        let snapshot_id = &self.active_scope.snapshot_id;
        let scope_id = &self.active_scope.scope_id;
        let node_ids: BTreeSet<&str> = self.nodes.iter().map(|n| n.node_id.as_str()).collect();
        let edge_ids: BTreeSet<&str> = self.edges.iter().map(|e| e.edge_id.as_str()).collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        for binding in &self.surface_bindings {
            if !seen_ids.insert(binding.binding_id.clone()) {
                violations.push(M5TopologyIdentityViolation::DuplicateBindingId {
                    binding_id: binding.binding_id.clone(),
                });
            }
            if !seen_surfaces.insert(binding.surface) {
                violations.push(M5TopologyIdentityViolation::DuplicateSurfaceBinding {
                    surface: binding.surface.as_str(),
                });
            }
            for (field, value) in [
                ("binding_id", &binding.binding_id),
                ("snapshot_id", &binding.snapshot_id),
                ("scope_id", &binding.scope_id),
                ("consumer_ref", &binding.consumer_ref),
                ("note", &binding.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5TopologyIdentityViolation::EmptyField {
                        id: binding.binding_id.clone(),
                        field_name: field,
                    });
                }
            }
            // The is_canvas flag must agree with the surface so the canvas guardrail cannot be
            // dodged by mislabeling.
            if binding.is_canvas != binding.surface.is_canvas() {
                violations.push(M5TopologyIdentityViolation::SurfaceCanvasFlagMismatch {
                    binding_id: binding.binding_id.clone(),
                });
            }
            // Every binding must be stamped with the active snapshot and scope so support export
            // and replay can reconstruct the slice the user queried.
            if &binding.snapshot_id != snapshot_id {
                violations.push(M5TopologyIdentityViolation::SnapshotBindingMismatch {
                    binding_id: binding.binding_id.clone(),
                });
            }
            if &binding.scope_id != scope_id {
                violations.push(M5TopologyIdentityViolation::ScopeIdMismatch {
                    binding_id: binding.binding_id.clone(),
                });
            }
            // A surface may only resolve ids declared in the packet; a transient per-view id
            // would break the shared identity contract.
            for node_id in &binding.resolves_node_ids {
                if !node_ids.contains(node_id.as_str()) {
                    violations.push(M5TopologyIdentityViolation::UnresolvedNodeRef {
                        binding_id: binding.binding_id.clone(),
                        node_id: node_id.clone(),
                    });
                }
            }
            for edge_id in &binding.resolves_edge_ids {
                if !edge_ids.contains(edge_id.as_str()) {
                    violations.push(M5TopologyIdentityViolation::UnresolvedEdgeRef {
                        binding_id: binding.binding_id.clone(),
                        edge_id: edge_id.clone(),
                    });
                }
            }
        }

        // Every surface must carry a binding so no accessible view leaves its identity
        // resolution implicit.
        for surface in TopologySurface::ALL {
            if !seen_surfaces.contains(&surface) {
                violations.push(M5TopologyIdentityViolation::MissingSurfaceBinding {
                    surface: surface.as_str(),
                });
            }
        }

        // Guardrail: the canvas must not own an identity a non-canvas accessible view cannot
        // resolve, and every declared node and edge must be resolvable from a non-canvas view.
        let non_canvas_nodes = self.non_canvas_node_ids();
        let non_canvas_edges = self.non_canvas_edge_ids();
        for node in &self.nodes {
            if !non_canvas_nodes.contains(node.node_id.as_str()) {
                violations.push(M5TopologyIdentityViolation::NodeMissingNonCanvasSurface {
                    node_id: node.node_id.clone(),
                });
            }
        }
        for edge in &self.edges {
            if !non_canvas_edges.contains(edge.edge_id.as_str()) {
                violations.push(M5TopologyIdentityViolation::EdgeMissingNonCanvasSurface {
                    edge_id: edge.edge_id.clone(),
                });
            }
        }
        for binding in self
            .surface_bindings
            .iter()
            .filter(|b| b.surface.is_canvas())
        {
            for node_id in &binding.resolves_node_ids {
                if node_ids.contains(node_id.as_str())
                    && !non_canvas_nodes.contains(node_id.as_str())
                {
                    violations.push(M5TopologyIdentityViolation::CanvasOnlyNodeIdentity {
                        node_id: node_id.clone(),
                    });
                }
            }
            for edge_id in &binding.resolves_edge_ids {
                if edge_ids.contains(edge_id.as_str())
                    && !non_canvas_edges.contains(edge_id.as_str())
                {
                    violations.push(M5TopologyIdentityViolation::CanvasOnlyEdgeIdentity {
                        edge_id: edge_id.clone(),
                    });
                }
            }
        }
    }

    fn validate_permalink_uniqueness(&self, violations: &mut Vec<M5TopologyIdentityViolation>) {
        let mut seen = BTreeSet::new();
        for permalink in self
            .nodes
            .iter()
            .map(|n| &n.export_permalink)
            .chain(self.edges.iter().map(|e| &e.export_permalink))
        {
            if !seen.insert(permalink.clone()) {
                violations.push(M5TopologyIdentityViolation::DuplicatePermalink {
                    permalink: permalink.clone(),
                });
            }
        }
    }
}

/// A validation violation for the M5 topology-identity packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5TopologyIdentityViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// The packet does not bind to the canonical governance matrix.
    GovernanceMatrixRefMismatch,
    /// The packet does not bind to the canonical workset-scope packet.
    ScopePacketRefMismatch,
    /// A node id appears more than once.
    DuplicateNodeId {
        /// Duplicate node id.
        node_id: String,
    },
    /// An edge id appears more than once.
    DuplicateEdgeId {
        /// Duplicate edge id.
        edge_id: String,
    },
    /// An edge endpoint references a node not declared in the packet.
    DanglingEdgeEndpoint {
        /// Edge id.
        edge_id: String,
        /// Missing node id.
        node_id: String,
    },
    /// A non-exact edge carries no explicit disclosure reason.
    UnlabeledNonExactRelation {
        /// Edge id.
        edge_id: String,
        /// Fidelity token.
        fidelity: &'static str,
    },
    /// A node carries a permalink that is empty or does not embed the node id.
    UnsafeNodePermalink {
        /// Node id.
        node_id: String,
    },
    /// An edge carries a permalink that is empty or does not embed the edge id.
    UnsafeEdgePermalink {
        /// Edge id.
        edge_id: String,
    },
    /// A permalink appears on more than one object.
    DuplicatePermalink {
        /// Duplicate permalink.
        permalink: String,
    },
    /// A node or edge is missing a required contract badge.
    MissingRequiredBadges {
        /// Object id.
        id: String,
    },
    /// A binding id appears more than once.
    DuplicateBindingId {
        /// Duplicate binding id.
        binding_id: String,
    },
    /// A surface carries more than one binding.
    DuplicateSurfaceBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A surface has no binding.
    MissingSurfaceBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A binding's is_canvas flag disagrees with its surface.
    SurfaceCanvasFlagMismatch {
        /// Binding id.
        binding_id: String,
    },
    /// A binding is not stamped with the active snapshot id.
    SnapshotBindingMismatch {
        /// Binding id.
        binding_id: String,
    },
    /// A binding renders a scope id other than the active scope.
    ScopeIdMismatch {
        /// Binding id.
        binding_id: String,
    },
    /// A binding resolves a node id not declared in the packet.
    UnresolvedNodeRef {
        /// Binding id.
        binding_id: String,
        /// Unresolved node id.
        node_id: String,
    },
    /// A binding resolves an edge id not declared in the packet.
    UnresolvedEdgeRef {
        /// Binding id.
        binding_id: String,
        /// Unresolved edge id.
        edge_id: String,
    },
    /// A declared node is resolvable only from the canvas.
    NodeMissingNonCanvasSurface {
        /// Node id.
        node_id: String,
    },
    /// A declared edge is resolvable only from the canvas.
    EdgeMissingNonCanvasSurface {
        /// Edge id.
        edge_id: String,
    },
    /// The canvas owns a node identity no non-canvas surface can resolve.
    CanvasOnlyNodeIdentity {
        /// Node id.
        node_id: String,
    },
    /// The canvas owns an edge identity no non-canvas surface can resolve.
    CanvasOnlyEdgeIdentity {
        /// Edge id.
        edge_id: String,
    },
    /// The summary counts disagree with the packet body.
    SummaryMismatch,
}

impl fmt::Display for M5TopologyIdentityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::GovernanceMatrixRefMismatch => write!(
                f,
                "packet governance_matrix_ref must be the canonical graph-depth governance matrix"
            ),
            Self::ScopePacketRefMismatch => write!(
                f,
                "packet scope_packet_ref must be the canonical workset-scope packet"
            ),
            Self::DuplicateNodeId { node_id } => write!(f, "duplicate node id {node_id}"),
            Self::DuplicateEdgeId { edge_id } => write!(f, "duplicate edge id {edge_id}"),
            Self::DanglingEdgeEndpoint { edge_id, node_id } => write!(
                f,
                "edge {edge_id} references node {node_id} that is not declared in the packet"
            ),
            Self::UnlabeledNonExactRelation { edge_id, fidelity } => write!(
                f,
                "edge {edge_id} is {fidelity} but carries no explicit fidelity_reason"
            ),
            Self::UnsafeNodePermalink { node_id } => write!(
                f,
                "node {node_id} has an empty permalink or one that does not embed its id"
            ),
            Self::UnsafeEdgePermalink { edge_id } => write!(
                f,
                "edge {edge_id} has an empty permalink or one that does not embed its id"
            ),
            Self::DuplicatePermalink { permalink } => {
                write!(f, "permalink {permalink} appears on more than one object")
            }
            Self::MissingRequiredBadges { id } => {
                write!(f, "{id} is missing a required contract badge")
            }
            Self::DuplicateBindingId { binding_id } => {
                write!(f, "duplicate binding id {binding_id}")
            }
            Self::DuplicateSurfaceBinding { surface } => {
                write!(f, "duplicate binding for surface {surface}")
            }
            Self::MissingSurfaceBinding { surface } => {
                write!(f, "missing binding for surface {surface}")
            }
            Self::SurfaceCanvasFlagMismatch { binding_id } => {
                write!(
                    f,
                    "binding {binding_id} has an is_canvas flag that disagrees with its surface"
                )
            }
            Self::SnapshotBindingMismatch { binding_id } => {
                write!(
                    f,
                    "binding {binding_id} is not stamped with the active snapshot id"
                )
            }
            Self::ScopeIdMismatch { binding_id } => {
                write!(
                    f,
                    "binding {binding_id} renders a scope other than the active scope"
                )
            }
            Self::UnresolvedNodeRef {
                binding_id,
                node_id,
            } => write!(
                f,
                "binding {binding_id} resolves node {node_id} that is not declared in the packet"
            ),
            Self::UnresolvedEdgeRef {
                binding_id,
                edge_id,
            } => write!(
                f,
                "binding {binding_id} resolves edge {edge_id} that is not declared in the packet"
            ),
            Self::NodeMissingNonCanvasSurface { node_id } => write!(
                f,
                "node {node_id} is resolvable only from the canvas, not a non-canvas view"
            ),
            Self::EdgeMissingNonCanvasSurface { edge_id } => write!(
                f,
                "edge {edge_id} is resolvable only from the canvas, not a non-canvas view"
            ),
            Self::CanvasOnlyNodeIdentity { node_id } => write!(
                f,
                "canvas owns node identity {node_id} that no non-canvas surface can resolve"
            ),
            Self::CanvasOnlyEdgeIdentity { edge_id } => write!(
                f,
                "canvas owns edge identity {edge_id} that no non-canvas surface can resolve"
            ),
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the packet body")
            }
        }
    }
}

impl Error for M5TopologyIdentityViolation {}

/// Loads the embedded M5 topology-identity packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5TopologyIdentityPacket`].
pub fn current_m5_topology_identity_packet() -> Result<M5TopologyIdentityPacket, serde_json::Error>
{
    serde_json::from_str(M5_TOPOLOGY_IDENTITY_JSON)
}

#[cfg(test)]
mod tests;
