//! Two reusable M5 topology-explorer components — the route / endpoint row and the component /
//! service tree node — so a user can inspect a framework topology row without hiding its evidence
//! basis: every route row names its route / path or matcher, its source file / symbol, its HTTP /
//! UI / runtime kind, its owning framework / app, its params / guards notes, its freshness, and its
//! evidence source, and links back to a canonical proving source; every tree node names its entity
//! kind, its source file / symbol, its parent / child or provider / consumer relation, its related
//! test / story / doc links, and its partial or derived notes, and links back to a canonical
//! proving source. Neither component acts like a hidden parallel model — the authored-versus-
//! generated boundary and the exact-versus-heuristic-versus-runtime-confirmed certainty stay
//! visible at row level rather than only in a buried detail panel.
//!
//! Aureline's frozen framework-component matrix
//! ([`crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix`])
//! names the route / endpoint row and the component / service tree node as two governed component
//! families and freezes their controlled vocabulary — the one controlled certainty disposition
//! (`core_native`, `framework_pack`, `bridge`, `heuristic_convention`, `verified`,
//! `derived_by_convention`, `runtime_confirmed`, `partial`); the route evidence classes
//! (`exact_from_source`, `heuristic_convention`, `runtime_confirmed`, `derived_by_convention`,
//! `partial_evidence`, `unresolved`) and route authorship states (`authored`, `generated`,
//! `generated_then_edited`, `framework_provided`, `runtime_only`, `unknown_origin`); the topology
//! node kinds (`component_node`, `service_node`, `module_node`, `dependency_edge`,
//! `external_boundary`, `unknown_node`) and topology evidence classes (`exact_from_source`,
//! `heuristic_inferred`, `runtime_confirmed`, `derived_by_convention`, `partial_evidence`,
//! `unresolved`); the surface families; the deployment lines; the consumer surfaces; the
//! accessibility routes; the required labels; and the downgrade triggers. This module *implements*
//! those contracts as two co-equal component vectors — a route / endpoint row and a component /
//! service tree node — so a claimed M5 route-explorer, topology-explorer, editor-gutter, CLI, or
//! support-export surface can project a row and a node that keep the same certainty, authorship,
//! and proving-source truth.
//!
//! The module has two derived resolvers:
//!
//! * [`resolve_route_evidence_posture`] — takes a route's frozen route evidence class and route
//!   authorship and derives its certainty posture (exact from source, runtime confirmed, heuristic,
//!   or partial / unresolved), its authorship posture (authored, generated, framework provided,
//!   runtime only, or unknown origin), whether the row claims exact-from-source, whether it is
//!   generated, whether it has a source form to prove, and which notes it must carry — so a
//!   heuristic route can never read as exact, the authored-versus-generated boundary is never left
//!   implicit, and a runtime-only or unknown-origin route can never pretend to link to a proving
//!   source file that does not exist.
//! * [`resolve_topology_evidence_posture`] — takes a node's frozen topology node kind and topology
//!   evidence class and derives its certainty posture, whether the node claims exact-from-source,
//!   whether it has a source form to prove, and which notes it must carry — so an inferred or
//!   derived relationship can never read as exact and an unresolved node can never pretend to link
//!   to a proving source it does not have.
//!
//! A single controls packet — [`RouteEndpointTreeNodeControlsPacket`] — binds one vector of route /
//! endpoint rows and one vector of component / service tree nodes to the same certainty, authorship,
//! proving-source, and non-visual accessibility vocabulary, so certainty and authorship stay
//! explicit across the route-explorer, topology-explorer, editor-gutter, CLI, and support consumers.
//!
//! The component family ([`M5FrameworkComponentFamily`]), route evidence class
//! ([`M5RouteEvidenceClass`]), route authorship ([`M5RouteAuthorship`]), topology node kind
//! ([`M5TopologyNodeKind`]), topology evidence class ([`M5TopologyEvidenceClass`]), certainty
//! disposition ([`M5FrameworkCertaintyDisposition`]), surface family
//! ([`M5FrameworkSurfaceFamily`]), deployment line ([`M5FrameworkDeploymentLine`]), consumer
//! surface ([`M5FrameworkConsumerSurface`]), accessibility route
//! ([`M5FrameworkAccessibilityRoute`]), required label ([`M5FrameworkRequiredLabel`]), and
//! downgrade trigger ([`M5FrameworkDowngradeTrigger`]) are reused verbatim from the frozen matrix.
//! This module mints new vocabulary only for what that matrix left implicit about the two
//! components themselves: the derived certainty posture, the derived route-authorship posture, the
//! route kind, the node relation kind, the proving-source link kind, the row freshness state, and
//! the bounded row and node actions. No M5 topology surface invents a second row or node grammar.
//!
//! Raw file bodies, raw source trees, pasted local paths, repository URLs, credentials, and secrets
//! stay outside the export boundary; every note, proving-source reference, and component identity is
//! carried only as an opaque, export-safe representation.

#[cfg(test)]
mod tests;

// The component family, the route / topology evidence and authorship vocabularies, the certainty
// disposition, and the surface / deployment / consumer / accessibility / label / downgrade
// vocabularies are frozen once, in the framework-component matrix. This lane reuses them verbatim so
// it never invents a parallel row or node vocabulary.
pub use crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix::{
    M5FrameworkAccessibilityRoute, M5FrameworkCertaintyDisposition, M5FrameworkComponentFamily,
    M5FrameworkConsumerSurface, M5FrameworkDeploymentLine, M5FrameworkDowngradeTrigger,
    M5FrameworkRequiredLabel, M5FrameworkSurfaceFamily, M5RouteAuthorship, M5RouteEvidenceClass,
    M5TopologyEvidenceClass, M5TopologyNodeKind, M5_COMPONENT_SERVICE_TREE_NODE_SCHEMA_REF,
    M5_FRAMEWORK_COMPONENT_DOC_REF, M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
    M5_ROUTE_ENDPOINT_ROW_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`RouteEndpointTreeNodeControlsPacket`].
pub const ROUTE_TREE_CONTROLS_RECORD_KIND: &str =
    "implement_route_endpoint_rows_and_component_service_tree_nodes_with_authored_versus_generated_state_proving_source_files_or_symbols_exact_versus_heuristic_labels_and_open_source_or_open_references_continuity";

/// Schema version for M5 route-endpoint-row / component-service-tree-node control records.
pub const ROUTE_TREE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const ROUTE_TREE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-route-endpoint-component-service-tree-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const ROUTE_TREE_CONTROLS_DOC_REF: &str =
    "docs/frameworks/m5/m5_route_endpoint_component_service_tree_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const ROUTE_TREE_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-route-endpoint-component-service-tree-controls";

/// Repo-relative path of the checked support-export artifact.
pub const ROUTE_TREE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-route-endpoint-tree-node-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const ROUTE_TREE_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-route-endpoint-tree-node-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const ROUTE_TREE_CONTROLS_REPORT_REF: &str = "artifacts/design/m5-route-endpoint-tree-node.md";

// ---- shared derived vocabulary ------------------------------------------

/// Derived certainty posture a route row or tree node may present. These are the exact
/// acceptance-criteria labels so a user can tell at a glance whether the row is exact from source,
/// runtime confirmed, a heuristic guess, or only partial / unresolved — a heuristic row can never
/// read as an exact one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCertaintyPosture {
    /// Exact, read directly from source.
    ExactFromSource,
    /// Confirmed by observing the running application.
    RuntimeConfirmed,
    /// A heuristic convention or derived-by-convention guess, not an exact fact.
    Heuristic,
    /// Partial evidence only, or unresolved.
    PartialOrUnresolved,
}

impl EvidenceCertaintyPosture {
    /// Every certainty posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExactFromSource,
        Self::RuntimeConfirmed,
        Self::Heuristic,
        Self::PartialOrUnresolved,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactFromSource => "exact_from_source",
            Self::RuntimeConfirmed => "runtime_confirmed",
            Self::Heuristic => "heuristic",
            Self::PartialOrUnresolved => "partial_or_unresolved",
        }
    }

    /// True only when the row is exact, read directly from source.
    pub const fn is_exact_from_source(self) -> bool {
        matches!(self, Self::ExactFromSource)
    }

    /// True when the row is confirmed by observing the running application.
    pub const fn is_runtime_confirmed(self) -> bool {
        matches!(self, Self::RuntimeConfirmed)
    }

    /// True when the posture is heuristic or partial / unresolved and must therefore never read as
    /// exact from source.
    pub const fn must_not_read_as_exact(self) -> bool {
        matches!(self, Self::Heuristic | Self::PartialOrUnresolved)
    }

    /// True when the row must carry an explicit heuristic note.
    pub const fn needs_heuristic_note(self) -> bool {
        matches!(self, Self::Heuristic)
    }

    /// True when the row must carry an explicit partial / unresolved note.
    pub const fn needs_partial_note(self) -> bool {
        matches!(self, Self::PartialOrUnresolved)
    }
}

/// The kind of stable proving source a topology component links its next step against, so a route
/// row or tree node never acts like a hidden parallel model — every next step is a canonical source
/// file, source symbol, runtime trace, or docs reference the user can reopen, or an explicit
/// no-proving-source state when none exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvingSourceLink {
    /// A canonical source-file reference.
    SourceFile,
    /// A canonical source-symbol reference.
    SourceSymbol,
    /// A runtime-trace reference (confirmed by observation, no static source form).
    RuntimeTrace,
    /// A docs / reference anchor.
    DocsAnchor,
    /// No proving source exists (the component names that it links nowhere).
    NoProvingSource,
}

impl ProvingSourceLink {
    /// Every proving-source link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SourceFile,
        Self::SourceSymbol,
        Self::RuntimeTrace,
        Self::DocsAnchor,
        Self::NoProvingSource,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceFile => "source_file",
            Self::SourceSymbol => "source_symbol",
            Self::RuntimeTrace => "runtime_trace",
            Self::DocsAnchor => "docs_anchor",
            Self::NoProvingSource => "no_proving_source",
        }
    }

    /// True when this kind names a resolvable proving-source target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoProvingSource)
    }
}

/// Row freshness state a route / endpoint row carries, so a stale or never-scanned route signal
/// never reads as current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowFreshnessState {
    /// The route signal is current.
    Current,
    /// The route signal was imported from another environment.
    Imported,
    /// The route signal is stale.
    Stale,
    /// The route has never been scanned.
    NeverScanned,
    /// Freshness is unknown.
    Unknown,
}

impl RowFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Current,
        Self::Imported,
        Self::Stale,
        Self::NeverScanned,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Imported => "imported",
            Self::Stale => "stale",
            Self::NeverScanned => "never_scanned",
            Self::Unknown => "unknown",
        }
    }

    /// True when the freshness signal must carry an explicit not-current note.
    pub const fn needs_note(self) -> bool {
        !matches!(self, Self::Current)
    }
}

// ---- route vocabulary ---------------------------------------------------

/// Derived route-authorship posture a route / endpoint row may present — the authored-versus-
/// generated boundary the acceptance criteria pin, so a generated route never leaves that boundary
/// implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteAuthorshipPosture {
    /// Hand-authored.
    Authored,
    /// Generated by a tool (including generated then hand-edited).
    Generated,
    /// Provided by the framework itself.
    FrameworkProvided,
    /// Runtime-only, no source form.
    RuntimeOnly,
    /// Unknown origin.
    UnknownOrigin,
}

impl RouteAuthorshipPosture {
    /// Every authorship posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Authored,
        Self::Generated,
        Self::FrameworkProvided,
        Self::RuntimeOnly,
        Self::UnknownOrigin,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authored => "authored",
            Self::Generated => "generated",
            Self::FrameworkProvided => "framework_provided",
            Self::RuntimeOnly => "runtime_only",
            Self::UnknownOrigin => "unknown_origin",
        }
    }

    /// True when the route was generated by a tool.
    pub const fn is_generated(self) -> bool {
        matches!(self, Self::Generated)
    }

    /// True when the route has a static source form that can be proven.
    pub const fn has_source_form(self) -> bool {
        matches!(
            self,
            Self::Authored | Self::Generated | Self::FrameworkProvided
        )
    }
}

/// The HTTP / UI / runtime kind of a route / endpoint row, so a row never leaves what kind of route
/// it is implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteKind {
    /// An HTTP route.
    HttpRoute,
    /// A UI / client route.
    UiRoute,
    /// A websocket route.
    WebsocketRoute,
    /// An RPC endpoint.
    RpcEndpoint,
    /// A runtime binding, observed rather than declared.
    RuntimeBinding,
    /// An unknown kind.
    UnknownKind,
}

impl RouteKind {
    /// Every route kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HttpRoute,
        Self::UiRoute,
        Self::WebsocketRoute,
        Self::RpcEndpoint,
        Self::RuntimeBinding,
        Self::UnknownKind,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HttpRoute => "http_route",
            Self::UiRoute => "ui_route",
            Self::WebsocketRoute => "websocket_route",
            Self::RpcEndpoint => "rpc_endpoint",
            Self::RuntimeBinding => "runtime_binding",
            Self::UnknownKind => "unknown_kind",
        }
    }
}

/// One keyboard-complete default action a route / endpoint row offers, so a row never hides its
/// proving-source, evidence, or params affordance behind a pointer-only gesture.
/// `OpenProvingSource`, `InspectEvidenceAndAuthorship`, and `ReviewParamsAndGuards` are always
/// offered so the proving source, the certainty / authorship, and the params / guards stay
/// inspectable before a user trusts the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteRowAction {
    /// Open the canonical proving source (always available).
    OpenProvingSource,
    /// Inspect the evidence class and authorship (always available).
    InspectEvidenceAndAuthorship,
    /// Review the params and guards (always available).
    ReviewParamsAndGuards,
    /// Copy the stable route id.
    CopyRouteId,
    /// Open a docs / reference anchor.
    OpenReference,
}

impl RouteRowAction {
    /// Every route-row action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenProvingSource,
        Self::InspectEvidenceAndAuthorship,
        Self::ReviewParamsAndGuards,
        Self::CopyRouteId,
        Self::OpenReference,
    ];

    /// The default actions every keyboard-complete route row must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenProvingSource,
        Self::InspectEvidenceAndAuthorship,
        Self::ReviewParamsAndGuards,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenProvingSource => "open_proving_source",
            Self::InspectEvidenceAndAuthorship => "inspect_evidence_and_authorship",
            Self::ReviewParamsAndGuards => "review_params_and_guards",
            Self::CopyRouteId => "copy_route_id",
            Self::OpenReference => "open_reference",
        }
    }
}

// ---- topology vocabulary ------------------------------------------------

/// The parent / child or provider / consumer relation a component / service tree node preserves, so
/// a node never leaves how it relates to the rest of the topology implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeRelationKind {
    /// A parent / child relation.
    ParentChild,
    /// A provider / consumer relation.
    ProviderConsumer,
    /// A dependency relation.
    Dependency,
    /// A root node with no parent.
    RootNode,
    /// No relation (a standalone node).
    None,
}

impl NodeRelationKind {
    /// Every relation kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ParentChild,
        Self::ProviderConsumer,
        Self::Dependency,
        Self::RootNode,
        Self::None,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParentChild => "parent_child",
            Self::ProviderConsumer => "provider_consumer",
            Self::Dependency => "dependency",
            Self::RootNode => "root_node",
            Self::None => "none",
        }
    }
}

/// One keyboard-complete default action a component / service tree node offers, so a node never
/// hides its proving-source, evidence, or related-links affordance behind a pointer-only gesture.
/// `OpenProvingSource` and `InspectEvidenceAndRelation` are always offered so the proving source and
/// the certainty / relation stay inspectable before a user trusts the node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TreeNodeAction {
    /// Open the canonical proving source (always available).
    OpenProvingSource,
    /// Inspect the evidence class and relation (always available).
    InspectEvidenceAndRelation,
    /// Open the related test / story / doc links.
    OpenRelatedLinks,
    /// Copy the stable node id.
    CopyNodeId,
    /// Open a docs / reference anchor.
    OpenReference,
}

impl TreeNodeAction {
    /// Every tree-node action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenProvingSource,
        Self::InspectEvidenceAndRelation,
        Self::OpenRelatedLinks,
        Self::CopyNodeId,
        Self::OpenReference,
    ];

    /// The default actions every keyboard-complete tree node must offer.
    pub const MANDATORY: [Self; 2] = [Self::OpenProvingSource, Self::InspectEvidenceAndRelation];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenProvingSource => "open_proving_source",
            Self::InspectEvidenceAndRelation => "inspect_evidence_and_relation",
            Self::OpenRelatedLinks => "open_related_links",
            Self::CopyNodeId => "copy_node_id",
            Self::OpenReference => "open_reference",
        }
    }
}

// ---- resolvers ----------------------------------------------------------

/// Disclosures a route / endpoint row must carry, derived from its route evidence class and route
/// authorship.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteEvidenceDisclosure {
    /// The derived certainty posture this row may present.
    pub certainty_posture: EvidenceCertaintyPosture,
    /// The derived authorship posture this row may present.
    pub authorship_posture: RouteAuthorshipPosture,
    /// Whether the row is exact, read directly from source.
    pub is_exact_from_source: bool,
    /// Whether the row is runtime confirmed.
    pub is_runtime_confirmed: bool,
    /// Whether the row must never read as exact from source.
    pub must_not_read_as_exact: bool,
    /// Whether the route was generated by a tool.
    pub is_generated: bool,
    /// Whether the route has a static source form that can be proven.
    pub has_source_form: bool,
    /// Whether the row must carry an explicit heuristic note.
    pub needs_heuristic_note: bool,
    /// Whether the row must carry an explicit partial / unresolved note.
    pub needs_partial_note: bool,
    /// Whether the row must carry an explicit generated note.
    pub needs_generated_note: bool,
    /// Whether the row must carry an explicit no-source-form note (runtime-only / unknown-origin).
    pub needs_no_source_form_note: bool,
}

/// Resolves the certainty, authorship, and proving-source truth a route / endpoint row may present.
///
/// An `exact_from_source` evidence class is exact; a `runtime_confirmed` one is runtime confirmed; a
/// `heuristic_convention` or `derived_by_convention` one is heuristic; a `partial_evidence` or
/// `unresolved` one is partial / unresolved — so a heuristic route can never read as an exact one.
/// An `authored` route is authored; a `generated` or `generated_then_edited` one is generated; a
/// `framework_provided` one is framework provided; a `runtime_only` one is runtime only; an
/// `unknown_origin` one is unknown — so the authored-versus-generated boundary is never left
/// implicit, and a runtime-only or unknown-origin route can never pretend to link to a source file
/// it does not have.
pub fn resolve_route_evidence_posture(
    evidence: M5RouteEvidenceClass,
    authorship: M5RouteAuthorship,
) -> RouteEvidenceDisclosure {
    use EvidenceCertaintyPosture as Certainty;
    use M5RouteAuthorship as Authorship;
    use M5RouteEvidenceClass as Evidence;
    use RouteAuthorshipPosture as Posture;

    let certainty_posture = match evidence {
        Evidence::ExactFromSource => Certainty::ExactFromSource,
        Evidence::RuntimeConfirmed => Certainty::RuntimeConfirmed,
        Evidence::HeuristicConvention | Evidence::DerivedByConvention => Certainty::Heuristic,
        Evidence::PartialEvidence | Evidence::Unresolved => Certainty::PartialOrUnresolved,
    };
    let authorship_posture = match authorship {
        Authorship::Authored => Posture::Authored,
        Authorship::Generated | Authorship::GeneratedThenEdited => Posture::Generated,
        Authorship::FrameworkProvided => Posture::FrameworkProvided,
        Authorship::RuntimeOnly => Posture::RuntimeOnly,
        Authorship::UnknownOrigin => Posture::UnknownOrigin,
    };

    RouteEvidenceDisclosure {
        certainty_posture,
        authorship_posture,
        is_exact_from_source: certainty_posture.is_exact_from_source(),
        is_runtime_confirmed: certainty_posture.is_runtime_confirmed(),
        must_not_read_as_exact: certainty_posture.must_not_read_as_exact(),
        is_generated: authorship_posture.is_generated(),
        has_source_form: authorship_posture.has_source_form(),
        needs_heuristic_note: certainty_posture.needs_heuristic_note(),
        needs_partial_note: certainty_posture.needs_partial_note(),
        needs_generated_note: authorship_posture.is_generated(),
        needs_no_source_form_note: !authorship_posture.has_source_form(),
    }
}

/// Disclosures a component / service tree node must carry, derived from its topology node kind and
/// topology evidence class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TopologyEvidenceDisclosure {
    /// The derived certainty posture this node may present.
    pub certainty_posture: EvidenceCertaintyPosture,
    /// Whether the node is exact, read directly from source.
    pub is_exact_from_source: bool,
    /// Whether the node is runtime confirmed.
    pub is_runtime_confirmed: bool,
    /// Whether the node must never read as exact from source.
    pub must_not_read_as_exact: bool,
    /// Whether the node represents a dependency-edge relationship rather than an entity.
    pub is_relationship_node: bool,
    /// Whether the node has a static source form that can be proven.
    pub has_source_form: bool,
    /// Whether the node must carry an explicit heuristic note.
    pub needs_heuristic_note: bool,
    /// Whether the node must carry an explicit partial / unresolved note.
    pub needs_partial_note: bool,
    /// Whether the node must carry an explicit no-source-form note (unknown node / unresolved).
    pub needs_no_source_form_note: bool,
}

/// Resolves the certainty and proving-source truth a component / service tree node may present.
///
/// An `exact_from_source` evidence class is exact; a `runtime_confirmed` one is runtime confirmed; a
/// `heuristic_inferred` or `derived_by_convention` one is heuristic; a `partial_evidence` or
/// `unresolved` one is partial / unresolved — so an inferred or derived relationship can never read
/// as an exact one. An `unknown_node` kind or an `unresolved` evidence class has no source form to
/// prove, so it can never pretend to link to a proving source it does not have.
pub fn resolve_topology_evidence_posture(
    node_kind: M5TopologyNodeKind,
    evidence: M5TopologyEvidenceClass,
) -> TopologyEvidenceDisclosure {
    use EvidenceCertaintyPosture as Certainty;
    use M5TopologyEvidenceClass as Evidence;
    use M5TopologyNodeKind as Kind;

    let certainty_posture = match evidence {
        Evidence::ExactFromSource => Certainty::ExactFromSource,
        Evidence::RuntimeConfirmed => Certainty::RuntimeConfirmed,
        Evidence::HeuristicInferred | Evidence::DerivedByConvention => Certainty::Heuristic,
        Evidence::PartialEvidence | Evidence::Unresolved => Certainty::PartialOrUnresolved,
    };
    let has_source_form =
        !matches!(node_kind, Kind::UnknownNode) && !matches!(evidence, Evidence::Unresolved);

    TopologyEvidenceDisclosure {
        certainty_posture,
        is_exact_from_source: certainty_posture.is_exact_from_source(),
        is_runtime_confirmed: certainty_posture.is_runtime_confirmed(),
        must_not_read_as_exact: certainty_posture.must_not_read_as_exact(),
        is_relationship_node: matches!(node_kind, Kind::DependencyEdge),
        has_source_form,
        needs_heuristic_note: certainty_posture.needs_heuristic_note(),
        needs_partial_note: certainty_posture.needs_partial_note(),
        needs_no_source_form_note: !has_source_form,
    }
}

// ---- component structs --------------------------------------------------

/// A route / endpoint row naming its route / path or matcher, source file / symbol, HTTP / UI /
/// runtime kind, owning framework / app, params / guards notes, freshness, and evidence source,
/// with a derived certainty posture and authorship posture, a canonical proving-source link, and
/// bounded open-proving-source / inspect / review actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteEndpointRow {
    /// Frozen component this control implements; must be `route_endpoint_row`.
    pub component: M5FrameworkComponentFamily,
    /// Stable route id.
    pub route_id: String,
    /// Route / path or matcher label; required and non-empty.
    pub route_or_matcher_label: String,
    /// HTTP / UI / runtime kind of this route.
    pub route_kind: RouteKind,
    /// Source file label; always required so the proving file stays explicit.
    pub source_file_label: String,
    /// Source symbol label; always required so the proving symbol stays explicit.
    pub source_symbol_label: String,
    /// Owning framework label; required and non-empty.
    pub owning_framework_label: String,
    /// Owning app label; required and non-empty.
    pub owning_app_label: String,
    /// Route evidence class, reused from the frozen matrix.
    pub route_evidence_class: M5RouteEvidenceClass,
    /// Route authorship, reused from the frozen matrix.
    pub route_authorship: M5RouteAuthorship,
    /// Certainty disposition, reused from the frozen matrix.
    pub certainty: M5FrameworkCertaintyDisposition,
    /// Derived certainty posture (must equal the resolved posture).
    pub derived_certainty_posture: EvidenceCertaintyPosture,
    /// Derived authorship posture (must equal the resolved posture).
    pub derived_authorship_posture: RouteAuthorshipPosture,
    /// Whether the row claims exact-from-source (must equal derived truth).
    pub claims_exact_from_source: bool,
    /// Whether the row claims to be generated (must equal derived truth).
    pub claims_generated: bool,
    /// Whether the row has a static source form to prove (must equal derived truth).
    pub has_proving_source_form: bool,
    /// Params notes; always required so the params stay explicit.
    pub params_notes: String,
    /// Guards notes; always required so the guards stay explicit.
    pub guards_notes: String,
    /// Row freshness state.
    pub freshness_state: RowFreshnessState,
    /// Freshness label; always required so how current the signal is stays explicit.
    pub freshness_label: String,
    /// Evidence source label; always required so the evidence source stays explicit.
    pub evidence_source_label: String,
    /// Heuristic note; required when the certainty posture is heuristic.
    pub heuristic_note: String,
    /// Partial note; required when the certainty posture is partial / unresolved.
    pub partial_note: String,
    /// Generated note; required when the route is generated.
    pub generated_note: String,
    /// No-source-form note; required when the route has no static source form.
    pub no_source_form_note: String,
    /// Certainty and authorship note; always required so the row states both at row level.
    pub certainty_and_authorship_note: String,
    /// Kind of canonical proving source this row links its next step against.
    pub proving_source_kind: ProvingSourceLink,
    /// Opaque canonical proving-source reference; required when the kind resolves.
    pub proving_source_ref: String,
    /// Context note; always required so the row names what to check before trusting it.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub row_actions: Vec<RouteRowAction>,
    /// Certainty dispositions this row binds (required, from the one shared vocabulary).
    pub dispositions: Vec<M5FrameworkCertaintyDisposition>,
    /// Downgrade triggers this row can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Mandatory labels this row can show (must include the mandatory labels).
    pub required_labels: Vec<M5FrameworkRequiredLabel>,
    /// Claimed M5 surface families that render this row.
    pub surface_families: Vec<M5FrameworkSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5FrameworkDeploymentLine>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5FrameworkAccessibilityRoute>,
    /// Framework subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never lets a heuristic route masquerade as exact. MUST be `false`.
    pub lets_heuristic_masquerade_as_exact: bool,
    /// Hard invariant: never hides the authored-versus-generated state. MUST be `false`.
    pub hides_authored_versus_generated_state: bool,
    /// Hard invariant: never acts like a hidden parallel model without a proving source. MUST be
    /// `false`.
    pub acts_as_hidden_parallel_model: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl RouteEndpointRow {
    /// Certainty / authorship / proving-source disclosures this row must carry, derived from the
    /// frozen classes.
    pub fn posture_disclosure(&self) -> RouteEvidenceDisclosure {
        resolve_route_evidence_posture(self.route_evidence_class, self.route_authorship)
    }

    /// Whether the row offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<RouteRowAction> = self.row_actions.iter().copied().collect();
        RouteRowAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }
}

/// A component / service tree node preserving its entity kind, source file / symbol, parent / child
/// or provider / consumer relation, related test / story / doc links, and partial or derived notes,
/// with a derived certainty posture, a canonical proving-source link, and bounded open-proving-
/// source / inspect actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentServiceTreeNode {
    /// Frozen component this control implements; must be `component_service_tree_node`.
    pub component: M5FrameworkComponentFamily,
    /// Stable node id.
    pub node_id: String,
    /// Entity label; required and non-empty.
    pub entity_label: String,
    /// Topology node kind (the entity kind), reused from the frozen matrix.
    pub topology_node_kind: M5TopologyNodeKind,
    /// Source file label; always required so the proving file stays explicit.
    pub source_file_label: String,
    /// Source symbol label; always required so the proving symbol stays explicit.
    pub source_symbol_label: String,
    /// Topology evidence class, reused from the frozen matrix.
    pub topology_evidence_class: M5TopologyEvidenceClass,
    /// Certainty disposition, reused from the frozen matrix.
    pub certainty: M5FrameworkCertaintyDisposition,
    /// Derived certainty posture (must equal the resolved posture).
    pub derived_certainty_posture: EvidenceCertaintyPosture,
    /// Whether the node claims exact-from-source (must equal derived truth).
    pub claims_exact_from_source: bool,
    /// Whether the node has a static source form to prove (must equal derived truth).
    pub has_proving_source_form: bool,
    /// The parent / child or provider / consumer relation this node preserves.
    pub relation_kind: NodeRelationKind,
    /// Relation label; always required so the relation stays explicit.
    pub relation_label: String,
    /// Related test / story / doc links label; always required so the related links stay explicit.
    pub related_links_label: String,
    /// Heuristic note; required when the certainty posture is heuristic.
    pub heuristic_note: String,
    /// Partial note; required when the certainty posture is partial / unresolved.
    pub partial_note: String,
    /// No-source-form note; required when the node has no static source form.
    pub no_source_form_note: String,
    /// Certainty and relation note; always required so the node states both at node level.
    pub certainty_and_relation_note: String,
    /// Kind of canonical proving source this node links its next step against.
    pub proving_source_kind: ProvingSourceLink,
    /// Opaque canonical proving-source reference; required when the kind resolves.
    pub proving_source_ref: String,
    /// Context note; always required so the node names what to check before trusting it.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub node_actions: Vec<TreeNodeAction>,
    /// Certainty dispositions this node binds (required, from the one shared vocabulary).
    pub dispositions: Vec<M5FrameworkCertaintyDisposition>,
    /// Downgrade triggers this node can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Mandatory labels this node can show (must include the mandatory labels).
    pub required_labels: Vec<M5FrameworkRequiredLabel>,
    /// Claimed M5 surface families that render this node.
    pub surface_families: Vec<M5FrameworkSurfaceFamily>,
    /// Deployment lines this node keeps the same truth across.
    pub deployment_lines: Vec<M5FrameworkDeploymentLine>,
    /// Non-visual accessibility routes this node offers.
    pub accessibility_routes: Vec<M5FrameworkAccessibilityRoute>,
    /// Framework subsystems that consume this node's projection.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this node.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never lets a heuristic node masquerade as exact. MUST be `false`.
    pub lets_heuristic_masquerade_as_exact: bool,
    /// Hard invariant: never hides the partial-or-derived state. MUST be `false`.
    pub hides_partial_or_derived_state: bool,
    /// Hard invariant: never acts like a hidden parallel model without a proving source. MUST be
    /// `false`.
    pub acts_as_hidden_parallel_model: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl ComponentServiceTreeNode {
    /// Certainty / proving-source disclosures this node must carry, derived from the frozen classes.
    pub fn posture_disclosure(&self) -> TopologyEvidenceDisclosure {
        resolve_topology_evidence_posture(self.topology_node_kind, self.topology_evidence_class)
    }

    /// Whether the node offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<TreeNodeAction> = self.node_actions.iter().copied().collect();
        TreeNodeAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the node declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }
}

/// Whether a required-label list declares all three mandatory labels.
fn declares_mandatory_labels(labels: &[M5FrameworkRequiredLabel]) -> bool {
    let present: BTreeSet<M5FrameworkRequiredLabel> = labels.iter().copied().collect();
    M5FrameworkRequiredLabel::MANDATORY
        .iter()
        .all(|label| present.contains(label))
}

// ---- review blocks ------------------------------------------------------

/// First-glance route / topology review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteTopologyReview {
    /// The route row names its route / path or matcher and its source file / symbol.
    pub route_row_shows_matcher_and_source: bool,
    /// The route row names its HTTP / UI / runtime kind and owning framework / app.
    pub route_row_shows_kind_and_owner: bool,
    /// The route row offers an open-proving-source action.
    pub route_row_offers_proving_source: bool,
    /// The tree node names its entity kind and its source file / symbol.
    pub tree_node_shows_entity_and_source: bool,
    /// The tree node names its relation and related test / story / doc links.
    pub tree_node_shows_relation_and_links: bool,
    /// The tree node offers an open-proving-source action.
    pub tree_node_offers_proving_source: bool,
    /// Certainty and authorship are derived from state, never asserted.
    pub certainty_and_authorship_derived_never_asserted: bool,
    /// A heuristic route or node is never shown as exact.
    pub heuristic_never_shown_as_exact: bool,
    /// The authored-versus-generated boundary stays visible at row level.
    pub authored_versus_generated_visible_at_row_level: bool,
    /// A partial or derived relationship is always labelled.
    pub partial_or_derived_always_labelled: bool,
    /// Every row and node links back to a canonical proving source rather than acting as a hidden
    /// parallel model.
    pub every_row_links_to_proving_source: bool,
    /// A runtime-only or unresolved component never pretends to link to a source it does not have.
    pub runtime_only_never_fakes_a_source: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl RouteTopologyReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.route_row_shows_matcher_and_source
            && self.route_row_shows_kind_and_owner
            && self.route_row_offers_proving_source
            && self.tree_node_shows_entity_and_source
            && self.tree_node_shows_relation_and_links
            && self.tree_node_offers_proving_source
            && self.certainty_and_authorship_derived_never_asserted
            && self.heuristic_never_shown_as_exact
            && self.authored_versus_generated_visible_at_row_level
            && self.partial_or_derived_always_labelled
            && self.every_row_links_to_proving_source
            && self.runtime_only_never_fakes_a_source
            && self.no_surface_invents_alternate_state_label
            && self.components_stable_across_deployment_lines
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteTopologyConsumerProjection {
    /// The route-explorer surface reads a single canonical source.
    pub route_explorer_reads_single_source: bool,
    /// The topology-explorer surface reads a single canonical source.
    pub topology_explorer_reads_single_source: bool,
    /// The editor-gutter surface reads a single canonical source.
    pub editor_gutter_reads_single_source: bool,
    /// Certainty and authorship are visible before a user trusts the row.
    pub certainty_and_authorship_visible_before_trust: bool,
    /// The proving source is reachable before a user trusts the row.
    pub proving_source_reachable_before_trust: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

impl RouteTopologyConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.route_explorer_reads_single_source
            && self.topology_explorer_reads_single_source
            && self.editor_gutter_reads_single_source
            && self.certainty_and_authorship_visible_before_trust
            && self.proving_source_reachable_before_trust
            && self.support_export_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteTopologyProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`RouteEndpointTreeNodeControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteEndpointTreeNodeControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Route / endpoint rows.
    pub route_rows: Vec<RouteEndpointRow>,
    /// Component / service tree nodes.
    pub tree_nodes: Vec<ComponentServiceTreeNode>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Route / topology review block.
    pub route_topology_review: RouteTopologyReview,
    /// Consumer projection block.
    pub consumer_projection: RouteTopologyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RouteTopologyProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe route-endpoint-row / component-service-tree-node controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteEndpointTreeNodeControlsPacket {
    /// Record kind; must equal [`ROUTE_TREE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ROUTE_TREE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Route / endpoint rows.
    pub route_rows: Vec<RouteEndpointRow>,
    /// Component / service tree nodes.
    pub tree_nodes: Vec<ComponentServiceTreeNode>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Route / topology review block.
    pub route_topology_review: RouteTopologyReview,
    /// Consumer projection block.
    pub consumer_projection: RouteTopologyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RouteTopologyProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RouteEndpointTreeNodeControlsPacket {
    /// Builds a route-endpoint-row / tree-node controls packet from stable-lane input.
    pub fn new(input: RouteEndpointTreeNodeControlsPacketInput) -> Self {
        Self {
            record_kind: ROUTE_TREE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: ROUTE_TREE_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            route_rows: input.route_rows,
            tree_nodes: input.tree_nodes,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            route_topology_review: input.route_topology_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the route-endpoint-row / tree-node control invariants.
    pub fn validate(&self) -> Vec<RouteTreeControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != ROUTE_TREE_CONTROLS_RECORD_KIND {
            violations.push(RouteTreeControlsViolation::WrongRecordKind);
        }
        if self.schema_version != ROUTE_TREE_CONTROLS_SCHEMA_VERSION {
            violations.push(RouteTreeControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RouteTreeControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(RouteTreeControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(RouteTreeControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_route_rows(self, &mut violations);
        validate_tree_nodes(self, &mut violations);

        if !self.route_topology_review.all_hold() {
            violations.push(RouteTreeControlsViolation::RouteTopologyReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(RouteTreeControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(RouteTreeControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("route tree controls packet serializes"),
        ) {
            violations.push(RouteTreeControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("route tree controls packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component,id,evidence_class,secondary,certainty_posture,exact_from_source,proving_source_kind\n",
        );
        for row in &self.route_rows {
            let disclosure = row.posture_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "route_endpoint_row",
                csv_field(&row.route_id),
                row.route_evidence_class.as_str(),
                row.route_authorship.as_str(),
                disclosure.certainty_posture.as_str(),
                disclosure.is_exact_from_source,
                row.proving_source_kind.as_str(),
            ));
        }
        for node in &self.tree_nodes {
            let disclosure = node.posture_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "component_service_tree_node",
                csv_field(&node.node_id),
                node.topology_evidence_class.as_str(),
                node.topology_node_kind.as_str(),
                disclosure.certainty_posture.as_str(),
                disclosure.is_exact_from_source,
                node.proving_source_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let generated = self
            .route_rows
            .iter()
            .filter(|row| row.posture_disclosure().is_generated)
            .count();
        let heuristic = self
            .tree_nodes
            .iter()
            .filter(|node| node.posture_disclosure().must_not_read_as_exact)
            .count();

        let mut out = String::new();
        out.push_str("# Route / endpoint rows and component / service tree nodes\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Route / endpoint rows: {} ({} generated)\n",
            self.route_rows.len(),
            generated
        ));
        out.push_str(&format!(
            "- Component / service tree nodes: {} ({} heuristic or partial)\n",
            self.tree_nodes.len(),
            heuristic
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Route / endpoint rows\n\n");
        for row in &self.route_rows {
            let disclosure = row.posture_disclosure();
            out.push_str(&format!(
                "- **{}** — kind `{}`, evidence `{}`, certainty `{}`, authorship `{}`, freshness `{}`, proving source `{}`\n",
                row.route_or_matcher_label,
                row.route_kind.as_str(),
                row.route_evidence_class.as_str(),
                disclosure.certainty_posture.as_str(),
                disclosure.authorship_posture.as_str(),
                row.freshness_state.as_str(),
                row.proving_source_kind.as_str(),
            ));
        }

        out.push_str("\n## Component / service tree nodes\n\n");
        for node in &self.tree_nodes {
            let disclosure = node.posture_disclosure();
            out.push_str(&format!(
                "- **{}** — kind `{}`, evidence `{}`, certainty `{}`, relation `{}`, proving source `{}`\n",
                node.entity_label,
                node.topology_node_kind.as_str(),
                node.topology_evidence_class.as_str(),
                disclosure.certainty_posture.as_str(),
                node.relation_kind.as_str(),
                node.proving_source_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in route-tree controls export.
#[derive(Debug)]
pub enum RouteTreeControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RouteTreeControlsViolation>),
}

impl fmt::Display for RouteTreeControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "route tree controls export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "route tree controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RouteTreeControlsArtifactError {}

/// Validation failures emitted by [`RouteEndpointTreeNodeControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RouteTreeControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No route / endpoint rows are present.
    RouteRowsMissing,
    /// A route / endpoint row is incomplete.
    RouteRowIncomplete,
    /// A route / endpoint row carries the wrong frozen component class.
    RouteRowWrongComponentClass,
    /// A route row misrepresents its derived certainty posture, authorship, or claims.
    RoutePostureMisrepresented,
    /// No component / service tree nodes are present.
    TreeNodesMissing,
    /// A component / service tree node is incomplete.
    TreeNodeIncomplete,
    /// A component / service tree node carries the wrong frozen component class.
    TreeNodeWrongComponentClass,
    /// A tree node misrepresents its derived certainty posture or claims.
    TopologyPostureMisrepresented,
    /// A heuristic or partial component claims exact-from-source.
    HeuristicClaimsExact,
    /// A heuristic component does not name its heuristic basis.
    HeuristicNoteMissing,
    /// A partial / unresolved component does not name its partial basis.
    PartialNoteMissing,
    /// A generated route does not name its generated authorship.
    GeneratedNoteMissing,
    /// A component with no source form does not name why it has no proving source.
    NoSourceFormNoteMissing,
    /// A component claims a resolvable proving source but has no source form.
    ProvingSourceClaimedWithoutForm,
    /// A component with a source form does not link to a resolvable proving source.
    ProvingSourceUnresolved,
    /// A component names a resolvable proving-source kind but not its reference.
    ProvingSourceRefMissing,
    /// A route row does not name its evidence source.
    EvidenceSourceMissing,
    /// A route row does not name its params or guards.
    ParamsOrGuardsMissing,
    /// A route row does not name its freshness.
    FreshnessLabelMissing,
    /// A tree node does not name its relation.
    RelationLabelMissing,
    /// A tree node does not name its related test / story / doc links.
    RelatedLinksMissing,
    /// A component does not name its certainty / authorship / relation at row level.
    RowLevelStateNoteMissing,
    /// The route rows do not cover every route evidence class.
    RouteEvidenceClassCoverageMissing,
    /// The route rows do not cover every route authorship state.
    RouteAuthorshipCoverageMissing,
    /// The route rows do not cover every route kind.
    RouteKindCoverageMissing,
    /// The route rows do not cover every freshness state.
    RowFreshnessCoverageMissing,
    /// The tree nodes do not cover every topology node kind.
    TopologyNodeKindCoverageMissing,
    /// The tree nodes do not cover every topology evidence class.
    TopologyEvidenceClassCoverageMissing,
    /// The tree nodes do not cover every node relation kind.
    NodeRelationKindCoverageMissing,
    /// The components do not cover every derived certainty posture.
    CertaintyPostureCoverageMissing,
    /// The components do not cover every proving-source link kind.
    ProvingSourceLinkCoverageMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A route row omits a mandatory action.
    RouteRowActionsIncomplete,
    /// A tree node omits a mandatory action.
    TreeNodeActionsIncomplete,
    /// A component does not bind any certainty disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component lets a heuristic route or node masquerade as exact.
    HeuristicMasqueradesAsExact,
    /// A route row hides its authored-versus-generated state.
    AuthoredVersusGeneratedHidden,
    /// A tree node hides its partial-or-derived state.
    PartialOrDerivedHidden,
    /// A component acts like a hidden parallel model without a proving source.
    HiddenParallelModel,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Route / topology review does not satisfy required invariants.
    RouteTopologyReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl RouteTreeControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RouteRowsMissing => "route_rows_missing",
            Self::RouteRowIncomplete => "route_row_incomplete",
            Self::RouteRowWrongComponentClass => "route_row_wrong_component_class",
            Self::RoutePostureMisrepresented => "route_posture_misrepresented",
            Self::TreeNodesMissing => "tree_nodes_missing",
            Self::TreeNodeIncomplete => "tree_node_incomplete",
            Self::TreeNodeWrongComponentClass => "tree_node_wrong_component_class",
            Self::TopologyPostureMisrepresented => "topology_posture_misrepresented",
            Self::HeuristicClaimsExact => "heuristic_claims_exact",
            Self::HeuristicNoteMissing => "heuristic_note_missing",
            Self::PartialNoteMissing => "partial_note_missing",
            Self::GeneratedNoteMissing => "generated_note_missing",
            Self::NoSourceFormNoteMissing => "no_source_form_note_missing",
            Self::ProvingSourceClaimedWithoutForm => "proving_source_claimed_without_form",
            Self::ProvingSourceUnresolved => "proving_source_unresolved",
            Self::ProvingSourceRefMissing => "proving_source_ref_missing",
            Self::EvidenceSourceMissing => "evidence_source_missing",
            Self::ParamsOrGuardsMissing => "params_or_guards_missing",
            Self::FreshnessLabelMissing => "freshness_label_missing",
            Self::RelationLabelMissing => "relation_label_missing",
            Self::RelatedLinksMissing => "related_links_missing",
            Self::RowLevelStateNoteMissing => "row_level_state_note_missing",
            Self::RouteEvidenceClassCoverageMissing => "route_evidence_class_coverage_missing",
            Self::RouteAuthorshipCoverageMissing => "route_authorship_coverage_missing",
            Self::RouteKindCoverageMissing => "route_kind_coverage_missing",
            Self::RowFreshnessCoverageMissing => "row_freshness_coverage_missing",
            Self::TopologyNodeKindCoverageMissing => "topology_node_kind_coverage_missing",
            Self::TopologyEvidenceClassCoverageMissing => {
                "topology_evidence_class_coverage_missing"
            }
            Self::NodeRelationKindCoverageMissing => "node_relation_kind_coverage_missing",
            Self::CertaintyPostureCoverageMissing => "certainty_posture_coverage_missing",
            Self::ProvingSourceLinkCoverageMissing => "proving_source_link_coverage_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::RouteRowActionsIncomplete => "route_row_actions_incomplete",
            Self::TreeNodeActionsIncomplete => "tree_node_actions_incomplete",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::HeuristicMasqueradesAsExact => "heuristic_masquerades_as_exact",
            Self::AuthoredVersusGeneratedHidden => "authored_versus_generated_hidden",
            Self::PartialOrDerivedHidden => "partial_or_derived_hidden",
            Self::HiddenParallelModel => "hidden_parallel_model",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::RouteTopologyReviewIncomplete => "route_topology_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable route-tree controls export.
///
/// This is the first real consumer of the route-endpoint-row / tree-node component lane: a
/// route-explorer, topology-explorer, editor-gutter, or support-export surface calls it to ingest
/// the canonical components rather than cloning row text.
///
/// # Errors
///
/// Returns [`RouteTreeControlsArtifactError`] when the checked-in support export fails to parse or
/// fails validation.
pub fn current_route_tree_controls_export(
) -> Result<RouteEndpointTreeNodeControlsPacket, RouteTreeControlsArtifactError> {
    let packet: RouteEndpointTreeNodeControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-route-endpoint-tree-node-proof/support_export.json"
    )))
    .map_err(RouteTreeControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RouteTreeControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &RouteEndpointTreeNodeControlsPacket,
    violations: &mut Vec<RouteTreeControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        ROUTE_TREE_CONTROLS_SCHEMA_REF,
        ROUTE_TREE_CONTROLS_DOC_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_DOC_REF,
        M5_ROUTE_ENDPOINT_ROW_SCHEMA_REF,
        M5_COMPONENT_SERVICE_TREE_NODE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(RouteTreeControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

/// The four hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    lets_heuristic_masquerade_as_exact: bool,
    hides_secondary_axis: bool,
    acts_as_hidden_parallel_model: bool,
    invents_alternate_state_label: bool,
    /// The violation to emit when `hides_secondary_axis` is set — family-specific.
    hidden_secondary_violation: RouteTreeControlsViolation,
}

/// Validates the certainty / exact-claim cross-checks and the proving-source truth shared by both
/// component vectors.
#[allow(clippy::too_many_arguments)]
fn validate_shared_evidence(
    certainty_posture: EvidenceCertaintyPosture,
    is_exact_from_source: bool,
    claims_exact_from_source: bool,
    has_source_form_derived: bool,
    has_proving_source_form: bool,
    needs_heuristic_note: bool,
    heuristic_note: &str,
    needs_partial_note: bool,
    partial_note: &str,
    proving_source_kind: ProvingSourceLink,
    proving_source_ref: &str,
    misrepresented_violation: RouteTreeControlsViolation,
    violations: &mut Vec<RouteTreeControlsViolation>,
) {
    if is_exact_from_source != claims_exact_from_source
        || has_source_form_derived != has_proving_source_form
    {
        violations.push(misrepresented_violation);
    }
    if certainty_posture.must_not_read_as_exact() && claims_exact_from_source {
        violations.push(RouteTreeControlsViolation::HeuristicClaimsExact);
    }
    if needs_heuristic_note && heuristic_note.trim().is_empty() {
        violations.push(RouteTreeControlsViolation::HeuristicNoteMissing);
    }
    if needs_partial_note && partial_note.trim().is_empty() {
        violations.push(RouteTreeControlsViolation::PartialNoteMissing);
    }
    // Proving-source truth: a component with a source form must link to a resolvable proving
    // source; a component with no source form must not claim one.
    if has_proving_source_form && !proving_source_kind.is_resolvable() {
        violations.push(RouteTreeControlsViolation::ProvingSourceUnresolved);
    }
    if !has_proving_source_form && proving_source_kind.is_resolvable() {
        violations.push(RouteTreeControlsViolation::ProvingSourceClaimedWithoutForm);
    }
    if proving_source_kind.is_resolvable() && proving_source_ref.trim().is_empty() {
        violations.push(RouteTreeControlsViolation::ProvingSourceRefMissing);
    }
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5FrameworkCertaintyDisposition],
    downgrade_triggers: &[M5FrameworkDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5FrameworkAccessibilityRoute],
    context_note: &str,
    invariants: ControlInvariants,
    violations: &mut Vec<RouteTreeControlsViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(RouteTreeControlsViolation::ContextNoteMissing);
    }
    if dispositions.is_empty() {
        violations.push(RouteTreeControlsViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(RouteTreeControlsViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(RouteTreeControlsViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(RouteTreeControlsViolation::AccessibilityRouteMissing);
    }
    if invariants.lets_heuristic_masquerade_as_exact {
        violations.push(RouteTreeControlsViolation::HeuristicMasqueradesAsExact);
    }
    if invariants.hides_secondary_axis {
        violations.push(invariants.hidden_secondary_violation);
    }
    if invariants.acts_as_hidden_parallel_model {
        violations.push(RouteTreeControlsViolation::HiddenParallelModel);
    }
    if invariants.invents_alternate_state_label {
        violations.push(RouteTreeControlsViolation::AlternateStateLabelInvented);
    }
}

fn validate_route_rows(
    packet: &RouteEndpointTreeNodeControlsPacket,
    violations: &mut Vec<RouteTreeControlsViolation>,
) {
    if packet.route_rows.is_empty() {
        violations.push(RouteTreeControlsViolation::RouteRowsMissing);
        return;
    }

    let mut evidence: BTreeSet<M5RouteEvidenceClass> = BTreeSet::new();
    let mut authorship: BTreeSet<M5RouteAuthorship> = BTreeSet::new();
    let mut kinds: BTreeSet<RouteKind> = BTreeSet::new();
    let mut freshness: BTreeSet<RowFreshnessState> = BTreeSet::new();

    for row in &packet.route_rows {
        let disclosure = row.posture_disclosure();
        evidence.insert(row.route_evidence_class);
        authorship.insert(row.route_authorship);
        kinds.insert(row.route_kind);
        freshness.insert(row.freshness_state);

        if row.route_id.trim().is_empty()
            || row.route_or_matcher_label.trim().is_empty()
            || row.source_file_label.trim().is_empty()
            || row.source_symbol_label.trim().is_empty()
            || row.owning_framework_label.trim().is_empty()
            || row.owning_app_label.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.consumer_surfaces.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(RouteTreeControlsViolation::RouteRowIncomplete);
        }
        if row.component != M5FrameworkComponentFamily::RouteEndpointRow {
            violations.push(RouteTreeControlsViolation::RouteRowWrongComponentClass);
        }
        if row.derived_certainty_posture != disclosure.certainty_posture
            || row.derived_authorship_posture != disclosure.authorship_posture
            || row.claims_generated != disclosure.is_generated
        {
            violations.push(RouteTreeControlsViolation::RoutePostureMisrepresented);
        }
        validate_shared_evidence(
            disclosure.certainty_posture,
            disclosure.is_exact_from_source,
            row.claims_exact_from_source,
            disclosure.has_source_form,
            row.has_proving_source_form,
            disclosure.needs_heuristic_note,
            &row.heuristic_note,
            disclosure.needs_partial_note,
            &row.partial_note,
            row.proving_source_kind,
            &row.proving_source_ref,
            RouteTreeControlsViolation::RoutePostureMisrepresented,
            violations,
        );
        if disclosure.needs_generated_note && row.generated_note.trim().is_empty() {
            violations.push(RouteTreeControlsViolation::GeneratedNoteMissing);
        }
        if disclosure.needs_no_source_form_note && row.no_source_form_note.trim().is_empty() {
            violations.push(RouteTreeControlsViolation::NoSourceFormNoteMissing);
        }
        if row.evidence_source_label.trim().is_empty() {
            violations.push(RouteTreeControlsViolation::EvidenceSourceMissing);
        }
        if row.params_notes.trim().is_empty() || row.guards_notes.trim().is_empty() {
            violations.push(RouteTreeControlsViolation::ParamsOrGuardsMissing);
        }
        if row.freshness_label.trim().is_empty() {
            violations.push(RouteTreeControlsViolation::FreshnessLabelMissing);
        }
        if row.certainty_and_authorship_note.trim().is_empty() {
            violations.push(RouteTreeControlsViolation::RowLevelStateNoteMissing);
        }
        if !row.declares_mandatory_actions() {
            violations.push(RouteTreeControlsViolation::RouteRowActionsIncomplete);
        }
        validate_common_control(
            &row.dispositions,
            &row.downgrade_triggers,
            row.declares_mandatory_labels(),
            &row.accessibility_routes,
            &row.context_note,
            ControlInvariants {
                lets_heuristic_masquerade_as_exact: row.lets_heuristic_masquerade_as_exact,
                hides_secondary_axis: row.hides_authored_versus_generated_state,
                acts_as_hidden_parallel_model: row.acts_as_hidden_parallel_model,
                invents_alternate_state_label: row.invents_alternate_state_label,
                hidden_secondary_violation:
                    RouteTreeControlsViolation::AuthoredVersusGeneratedHidden,
            },
            violations,
        );
    }

    for required in M5RouteEvidenceClass::ALL {
        if !evidence.contains(&required) {
            violations.push(RouteTreeControlsViolation::RouteEvidenceClassCoverageMissing);
            break;
        }
    }
    for required in M5RouteAuthorship::ALL {
        if !authorship.contains(&required) {
            violations.push(RouteTreeControlsViolation::RouteAuthorshipCoverageMissing);
            break;
        }
    }
    for required in RouteKind::ALL {
        if !kinds.contains(&required) {
            violations.push(RouteTreeControlsViolation::RouteKindCoverageMissing);
            break;
        }
    }
    for required in RowFreshnessState::ALL {
        if !freshness.contains(&required) {
            violations.push(RouteTreeControlsViolation::RowFreshnessCoverageMissing);
            break;
        }
    }

    validate_shared_coverage(packet, violations);
}

fn validate_tree_nodes(
    packet: &RouteEndpointTreeNodeControlsPacket,
    violations: &mut Vec<RouteTreeControlsViolation>,
) {
    if packet.tree_nodes.is_empty() {
        violations.push(RouteTreeControlsViolation::TreeNodesMissing);
        return;
    }

    let mut node_kinds: BTreeSet<M5TopologyNodeKind> = BTreeSet::new();
    let mut evidence: BTreeSet<M5TopologyEvidenceClass> = BTreeSet::new();
    let mut relations: BTreeSet<NodeRelationKind> = BTreeSet::new();

    for node in &packet.tree_nodes {
        let disclosure = node.posture_disclosure();
        node_kinds.insert(node.topology_node_kind);
        evidence.insert(node.topology_evidence_class);
        relations.insert(node.relation_kind);

        if node.node_id.trim().is_empty()
            || node.entity_label.trim().is_empty()
            || node.source_file_label.trim().is_empty()
            || node.source_symbol_label.trim().is_empty()
            || node.fields_shown.is_empty()
            || node.surface_families.is_empty()
            || node.deployment_lines.is_empty()
            || node.consumer_surfaces.is_empty()
            || node.source_contract_refs.is_empty()
        {
            violations.push(RouteTreeControlsViolation::TreeNodeIncomplete);
        }
        if node.component != M5FrameworkComponentFamily::ComponentServiceTreeNode {
            violations.push(RouteTreeControlsViolation::TreeNodeWrongComponentClass);
        }
        if node.derived_certainty_posture != disclosure.certainty_posture {
            violations.push(RouteTreeControlsViolation::TopologyPostureMisrepresented);
        }
        validate_shared_evidence(
            disclosure.certainty_posture,
            disclosure.is_exact_from_source,
            node.claims_exact_from_source,
            disclosure.has_source_form,
            node.has_proving_source_form,
            disclosure.needs_heuristic_note,
            &node.heuristic_note,
            disclosure.needs_partial_note,
            &node.partial_note,
            node.proving_source_kind,
            &node.proving_source_ref,
            RouteTreeControlsViolation::TopologyPostureMisrepresented,
            violations,
        );
        if disclosure.needs_no_source_form_note && node.no_source_form_note.trim().is_empty() {
            violations.push(RouteTreeControlsViolation::NoSourceFormNoteMissing);
        }
        if node.relation_label.trim().is_empty() {
            violations.push(RouteTreeControlsViolation::RelationLabelMissing);
        }
        if node.related_links_label.trim().is_empty() {
            violations.push(RouteTreeControlsViolation::RelatedLinksMissing);
        }
        if node.certainty_and_relation_note.trim().is_empty() {
            violations.push(RouteTreeControlsViolation::RowLevelStateNoteMissing);
        }
        if !node.declares_mandatory_actions() {
            violations.push(RouteTreeControlsViolation::TreeNodeActionsIncomplete);
        }
        validate_common_control(
            &node.dispositions,
            &node.downgrade_triggers,
            node.declares_mandatory_labels(),
            &node.accessibility_routes,
            &node.context_note,
            ControlInvariants {
                lets_heuristic_masquerade_as_exact: node.lets_heuristic_masquerade_as_exact,
                hides_secondary_axis: node.hides_partial_or_derived_state,
                acts_as_hidden_parallel_model: node.acts_as_hidden_parallel_model,
                invents_alternate_state_label: node.invents_alternate_state_label,
                hidden_secondary_violation: RouteTreeControlsViolation::PartialOrDerivedHidden,
            },
            violations,
        );
    }

    for required in M5TopologyNodeKind::ALL {
        if !node_kinds.contains(&required) {
            violations.push(RouteTreeControlsViolation::TopologyNodeKindCoverageMissing);
            break;
        }
    }
    for required in M5TopologyEvidenceClass::ALL {
        if !evidence.contains(&required) {
            violations.push(RouteTreeControlsViolation::TopologyEvidenceClassCoverageMissing);
            break;
        }
    }
    for required in NodeRelationKind::ALL {
        if !relations.contains(&required) {
            violations.push(RouteTreeControlsViolation::NodeRelationKindCoverageMissing);
            break;
        }
    }
}

/// Validates that the union of both component vectors covers every derived certainty posture and
/// proving-source link kind the acceptance criteria pin.
fn validate_shared_coverage(
    packet: &RouteEndpointTreeNodeControlsPacket,
    violations: &mut Vec<RouteTreeControlsViolation>,
) {
    let mut postures: BTreeSet<EvidenceCertaintyPosture> = BTreeSet::new();
    let mut links: BTreeSet<ProvingSourceLink> = BTreeSet::new();

    for row in &packet.route_rows {
        postures.insert(row.posture_disclosure().certainty_posture);
        links.insert(row.proving_source_kind);
    }
    for node in &packet.tree_nodes {
        postures.insert(node.posture_disclosure().certainty_posture);
        links.insert(node.proving_source_kind);
    }

    for required in EvidenceCertaintyPosture::ALL {
        if !postures.contains(&required) {
            violations.push(RouteTreeControlsViolation::CertaintyPostureCoverageMissing);
            break;
        }
    }
    for required in ProvingSourceLink::ALL {
        if !links.contains(&required) {
            violations.push(RouteTreeControlsViolation::ProvingSourceLinkCoverageMissing);
            break;
        }
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Canonical seed builders
//
// These builders are the single producer of the checked-in support export and the scenario
// fixtures. The headless emitter example and the inline tests both call them so the in-code
// components, the artifact, and the fixtures never drift.
// ---------------------------------------------------------------------------

/// Stable packet id for the canonical route-tree controls packet.
pub const ROUTE_TREE_CONTROLS_PACKET_ID: &str =
    "m5-route-endpoint-component-service-tree-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn route_source_refs() -> Vec<String> {
    strings(&[
        M5_ROUTE_ENDPOINT_ROW_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
    ])
}

fn node_source_refs() -> Vec<String> {
    strings(&[
        M5_COMPONENT_SERVICE_TREE_NODE_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
    ])
}

fn route_row_downgrade_triggers() -> Vec<M5FrameworkDowngradeTrigger> {
    vec![
        M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated,
        M5FrameworkDowngradeTrigger::AuthorshipUnstated,
        M5FrameworkDowngradeTrigger::ProvingSourceOmitted,
        M5FrameworkDowngradeTrigger::AlternateStateLabelInvented,
        M5FrameworkDowngradeTrigger::ProofStale,
    ]
}

fn tree_node_downgrade_triggers() -> Vec<M5FrameworkDowngradeTrigger> {
    vec![
        M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated,
        M5FrameworkDowngradeTrigger::DerivedStateUnlabeled,
        M5FrameworkDowngradeTrigger::ProvingSourceOmitted,
        M5FrameworkDowngradeTrigger::AlternateStateLabelInvented,
        M5FrameworkDowngradeTrigger::ProofStale,
    ]
}

/// The three mandatory labels plus one extra truth label.
fn label_set(extra: M5FrameworkRequiredLabel) -> Vec<M5FrameworkRequiredLabel> {
    let mut labels = M5FrameworkRequiredLabel::MANDATORY.to_vec();
    labels.push(extra);
    labels
}

/// Returns `text` when `needed`, else an empty string.
fn note_if(needed: bool, text: &str) -> String {
    if needed {
        text.to_owned()
    } else {
        String::new()
    }
}

/// Builds a route / endpoint row, deriving the certainty posture, authorship posture, exact / local
/// claims, source form, and required notes from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn route_row(
    route_id: &str,
    route_or_matcher_label: &str,
    route_kind: RouteKind,
    source_file_label: &str,
    source_symbol_label: &str,
    owning_framework_label: &str,
    owning_app_label: &str,
    route_evidence_class: M5RouteEvidenceClass,
    route_authorship: M5RouteAuthorship,
    certainty: M5FrameworkCertaintyDisposition,
    params_notes: &str,
    guards_notes: &str,
    freshness_state: RowFreshnessState,
    freshness_label: &str,
    evidence_source_label: &str,
    context_note: &str,
    proving_source_kind: ProvingSourceLink,
    proving_source_ref: &str,
    row_actions: Vec<RouteRowAction>,
) -> RouteEndpointRow {
    let disclosure = resolve_route_evidence_posture(route_evidence_class, route_authorship);
    RouteEndpointRow {
        component: M5FrameworkComponentFamily::RouteEndpointRow,
        route_id: route_id.to_owned(),
        route_or_matcher_label: route_or_matcher_label.to_owned(),
        route_kind,
        source_file_label: source_file_label.to_owned(),
        source_symbol_label: source_symbol_label.to_owned(),
        owning_framework_label: owning_framework_label.to_owned(),
        owning_app_label: owning_app_label.to_owned(),
        route_evidence_class,
        route_authorship,
        certainty,
        derived_certainty_posture: disclosure.certainty_posture,
        derived_authorship_posture: disclosure.authorship_posture,
        claims_exact_from_source: disclosure.is_exact_from_source,
        claims_generated: disclosure.is_generated,
        has_proving_source_form: disclosure.has_source_form,
        params_notes: params_notes.to_owned(),
        guards_notes: guards_notes.to_owned(),
        freshness_state,
        freshness_label: freshness_label.to_owned(),
        evidence_source_label: evidence_source_label.to_owned(),
        heuristic_note: note_if(
            disclosure.needs_heuristic_note,
            "Route is inferred by a heuristic convention; treat it as a guess, not an exact fact",
        ),
        partial_note: note_if(
            disclosure.needs_partial_note,
            "Route evidence is only partial or unresolved; do not treat it as complete",
        ),
        generated_note: note_if(
            disclosure.needs_generated_note,
            "Route was generated by a tool; the authored-versus-generated boundary is explicit",
        ),
        no_source_form_note: note_if(
            disclosure.needs_no_source_form_note,
            "Route has no static source form; it is runtime-only or of unknown origin, so no proving file exists",
        ),
        certainty_and_authorship_note: format!(
            "Certainty {}; authorship {}",
            disclosure.certainty_posture.as_str(),
            disclosure.authorship_posture.as_str()
        ),
        proving_source_kind,
        proving_source_ref: proving_source_ref.to_owned(),
        context_note: context_note.to_owned(),
        row_actions,
        dispositions: vec![certainty],
        downgrade_triggers: route_row_downgrade_triggers(),
        required_labels: label_set(M5FrameworkRequiredLabel::ProvingSourceAndRecoveryBoundary),
        surface_families: M5FrameworkSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5FrameworkDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5FrameworkAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "route_or_matcher_label",
            "route_kind",
            "source_file_label",
            "source_symbol_label",
            "owning_framework_label",
            "route_evidence_class",
            "route_authorship",
            "freshness_state",
            "proving_source_kind",
        ]),
        source_contract_refs: route_source_refs(),
        lets_heuristic_masquerade_as_exact: false,
        hides_authored_versus_generated_state: false,
        acts_as_hidden_parallel_model: false,
        invents_alternate_state_label: false,
    }
}

/// Builds a component / service tree node, deriving the certainty posture, exact claim, source form,
/// and required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
#[allow(clippy::too_many_arguments)]
fn tree_node(
    node_id: &str,
    entity_label: &str,
    topology_node_kind: M5TopologyNodeKind,
    source_file_label: &str,
    source_symbol_label: &str,
    topology_evidence_class: M5TopologyEvidenceClass,
    certainty: M5FrameworkCertaintyDisposition,
    relation_kind: NodeRelationKind,
    relation_label: &str,
    related_links_label: &str,
    context_note: &str,
    proving_source_kind: ProvingSourceLink,
    proving_source_ref: &str,
    node_actions: Vec<TreeNodeAction>,
) -> ComponentServiceTreeNode {
    let disclosure = resolve_topology_evidence_posture(topology_node_kind, topology_evidence_class);
    ComponentServiceTreeNode {
        component: M5FrameworkComponentFamily::ComponentServiceTreeNode,
        node_id: node_id.to_owned(),
        entity_label: entity_label.to_owned(),
        topology_node_kind,
        source_file_label: source_file_label.to_owned(),
        source_symbol_label: source_symbol_label.to_owned(),
        topology_evidence_class,
        certainty,
        derived_certainty_posture: disclosure.certainty_posture,
        claims_exact_from_source: disclosure.is_exact_from_source,
        has_proving_source_form: disclosure.has_source_form,
        relation_kind,
        relation_label: relation_label.to_owned(),
        related_links_label: related_links_label.to_owned(),
        heuristic_note: note_if(
            disclosure.needs_heuristic_note,
            "Node relationship is inferred by a heuristic; treat it as a guess, not an exact fact",
        ),
        partial_note: note_if(
            disclosure.needs_partial_note,
            "Node evidence is only partial or unresolved; do not treat the relationship as complete",
        ),
        no_source_form_note: note_if(
            disclosure.needs_no_source_form_note,
            "Node has no static source form; it is unresolved or unknown, so no proving file exists",
        ),
        certainty_and_relation_note: format!(
            "Certainty {}; relation {}",
            disclosure.certainty_posture.as_str(),
            relation_kind.as_str()
        ),
        proving_source_kind,
        proving_source_ref: proving_source_ref.to_owned(),
        context_note: context_note.to_owned(),
        node_actions,
        dispositions: vec![certainty],
        downgrade_triggers: tree_node_downgrade_triggers(),
        required_labels: label_set(M5FrameworkRequiredLabel::ProvingSourceAndRecoveryBoundary),
        surface_families: M5FrameworkSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5FrameworkDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5FrameworkAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "entity_label",
            "topology_node_kind",
            "source_file_label",
            "source_symbol_label",
            "topology_evidence_class",
            "relation_kind",
            "related_links_label",
            "proving_source_kind",
        ]),
        source_contract_refs: node_source_refs(),
        lets_heuristic_masquerade_as_exact: false,
        hides_partial_or_derived_state: false,
        acts_as_hidden_parallel_model: false,
        invents_alternate_state_label: false,
    }
}

fn route_rows() -> Vec<RouteEndpointRow> {
    use M5FrameworkCertaintyDisposition as Certainty;
    use M5RouteAuthorship as Authorship;
    use M5RouteEvidenceClass as Evidence;
    use ProvingSourceLink as Link;
    use RouteKind as Kind;
    use RouteRowAction as Action;
    use RowFreshnessState as Fresh;

    vec![
        // 1. Exact from source / authored / HTTP / current → exact, authored, source file.
        route_row(
            "route-users-index",
            "GET /users",
            Kind::HttpRoute,
            "app/routes/users.rs",
            "users::index",
            "Next.js",
            "Storefront app",
            Evidence::ExactFromSource,
            Authorship::Authored,
            Certainty::CoreNative,
            "path params: none",
            "guards: auth_required",
            Fresh::Current,
            "Scanned just now",
            "Read directly from app/routes/users.rs",
            "Exact, authored HTTP route; open the proving source before trusting the row",
            Link::SourceFile,
            "src:app/routes/users.rs",
            vec![
                Action::OpenProvingSource,
                Action::InspectEvidenceAndAuthorship,
                Action::ReviewParamsAndGuards,
                Action::CopyRouteId,
            ],
        ),
        // 2. Runtime confirmed / framework provided / UI / imported → runtime confirmed, runtime
        //    trace.
        route_row(
            "route-app-shell",
            "/_app",
            Kind::UiRoute,
            "framework internals",
            "next::app_shell",
            "Next.js",
            "Storefront app",
            Evidence::RuntimeConfirmed,
            Authorship::FrameworkProvided,
            Certainty::RuntimeConfirmed,
            "path params: none",
            "guards: none",
            Fresh::Imported,
            "Imported from a runtime scan",
            "Confirmed by observing the running application",
            "Framework-provided UI route confirmed at runtime; inspect the runtime trace",
            Link::RuntimeTrace,
            "trace:runtime/app-shell",
            vec![
                Action::OpenProvingSource,
                Action::InspectEvidenceAndAuthorship,
                Action::ReviewParamsAndGuards,
                Action::OpenReference,
            ],
        ),
        // 3. Heuristic convention / generated / RPC / stale → heuristic, generated, source symbol.
        route_row(
            "route-rpc-sync",
            "rpc: SyncService.Push",
            Kind::RpcEndpoint,
            "generated/rpc/sync.rs",
            "SyncService::push",
            "tonic",
            "Sync worker",
            Evidence::HeuristicConvention,
            Authorship::Generated,
            Certainty::HeuristicConvention,
            "path params: message id",
            "guards: internal_only",
            Fresh::Stale,
            "Scan is stale",
            "Inferred from a naming convention",
            "Heuristic, generated RPC endpoint; do not treat the match as exact",
            Link::SourceSymbol,
            "symbol:SyncService::push",
            vec![
                Action::OpenProvingSource,
                Action::InspectEvidenceAndAuthorship,
                Action::ReviewParamsAndGuards,
                Action::CopyRouteId,
            ],
        ),
        // 4. Derived by convention / generated then edited / websocket / never scanned → heuristic,
        //    generated, docs anchor.
        route_row(
            "route-ws-events",
            "ws: /events",
            Kind::WebsocketRoute,
            "generated/ws/events.rs",
            "events::stream",
            "actix",
            "Realtime app",
            Evidence::DerivedByConvention,
            Authorship::GeneratedThenEdited,
            Certainty::DerivedByConvention,
            "path params: channel",
            "guards: auth_required",
            Fresh::NeverScanned,
            "Never scanned",
            "Derived by a routing convention, then hand-edited",
            "Derived, generated-then-edited websocket; the convention basis is explicit",
            Link::DocsAnchor,
            "docs:frameworks/websocket-conventions",
            vec![
                Action::OpenProvingSource,
                Action::InspectEvidenceAndAuthorship,
                Action::ReviewParamsAndGuards,
                Action::OpenReference,
            ],
        ),
        // 5. Partial evidence / runtime only / runtime binding / unknown freshness → partial,
        //    runtime only, no proving source.
        route_row(
            "route-runtime-probe",
            "runtime: /internal/probe",
            Kind::RuntimeBinding,
            "no static source",
            "runtime binding only",
            "unresolved framework",
            "Sync worker",
            Evidence::PartialEvidence,
            Authorship::RuntimeOnly,
            Certainty::Partial,
            "path params: unknown",
            "guards: unknown",
            Fresh::Unknown,
            "Freshness unknown",
            "Observed only at runtime; no static form to read",
            "Runtime-only binding with partial evidence; there is no proving file to open",
            Link::NoProvingSource,
            "",
            vec![
                Action::OpenProvingSource,
                Action::InspectEvidenceAndAuthorship,
                Action::ReviewParamsAndGuards,
            ],
        ),
        // 6. Unresolved / unknown origin / unknown kind / stale → partial, unknown, no proving
        //    source.
        route_row(
            "route-unknown",
            "unresolved route",
            Kind::UnknownKind,
            "no static source",
            "unresolved symbol",
            "unresolved framework",
            "Unresolved app",
            Evidence::Unresolved,
            Authorship::UnknownOrigin,
            Certainty::Partial,
            "path params: unknown",
            "guards: unknown",
            Fresh::Stale,
            "Scan is stale",
            "Could not be resolved from the workspace",
            "Unresolved route of unknown origin; do not trust it and open nothing that does not exist",
            Link::NoProvingSource,
            "",
            vec![
                Action::OpenProvingSource,
                Action::InspectEvidenceAndAuthorship,
                Action::ReviewParamsAndGuards,
            ],
        ),
    ]
}

fn tree_nodes() -> Vec<ComponentServiceTreeNode> {
    use M5FrameworkCertaintyDisposition as Certainty;
    use M5TopologyEvidenceClass as Evidence;
    use M5TopologyNodeKind as Kind;
    use NodeRelationKind as Relation;
    use ProvingSourceLink as Link;
    use TreeNodeAction as Action;

    vec![
        // 1. Component node / exact from source / parent-child → exact, source file.
        tree_node(
            "node-cart-view",
            "CartView component",
            Kind::ComponentNode,
            "app/components/cart_view.rs",
            "CartView",
            Evidence::ExactFromSource,
            Certainty::CoreNative,
            Relation::ParentChild,
            "child of CheckoutPage",
            "tests: cart_view_test; story: CartView.story; doc: cart.md",
            "Exact, source-read component node; open the proving source before trusting it",
            Link::SourceFile,
            "src:app/components/cart_view.rs",
            vec![
                Action::OpenProvingSource,
                Action::InspectEvidenceAndRelation,
                Action::OpenRelatedLinks,
                Action::CopyNodeId,
            ],
        ),
        // 2. Service node / runtime confirmed / provider-consumer → runtime confirmed, runtime
        //    trace.
        tree_node(
            "node-payments-service",
            "Payments service",
            Kind::ServiceNode,
            "services/payments/mod.rs",
            "PaymentsService",
            Evidence::RuntimeConfirmed,
            Certainty::RuntimeConfirmed,
            Relation::ProviderConsumer,
            "provider to CheckoutPage",
            "tests: payments_test; story: none; doc: payments.md",
            "Service node confirmed at runtime; inspect the runtime trace and its relation",
            Link::RuntimeTrace,
            "trace:runtime/payments-service",
            vec![
                Action::OpenProvingSource,
                Action::InspectEvidenceAndRelation,
                Action::OpenRelatedLinks,
                Action::CopyNodeId,
            ],
        ),
        // 3. Module node / heuristic inferred / dependency → heuristic, source symbol.
        tree_node(
            "node-auth-module",
            "Auth module",
            Kind::ModuleNode,
            "modules/auth/mod.rs",
            "auth",
            Evidence::HeuristicInferred,
            Certainty::HeuristicConvention,
            Relation::Dependency,
            "depends on Session store",
            "tests: auth_test; story: none; doc: auth.md",
            "Heuristically inferred module dependency; do not treat the link as exact",
            Link::SourceSymbol,
            "symbol:auth",
            vec![
                Action::OpenProvingSource,
                Action::InspectEvidenceAndRelation,
                Action::OpenRelatedLinks,
                Action::CopyNodeId,
            ],
        ),
        // 4. Dependency edge / derived by convention / root node → heuristic, docs anchor.
        tree_node(
            "node-edge-di",
            "DI dependency edge",
            Kind::DependencyEdge,
            "app/di/graph.rs",
            "di::graph_edge",
            Evidence::DerivedByConvention,
            Certainty::DerivedByConvention,
            Relation::RootNode,
            "root of the injection graph",
            "tests: di_graph_test; story: none; doc: di.md",
            "Dependency edge derived by a wiring convention; the derived basis is explicit",
            Link::DocsAnchor,
            "docs:frameworks/di-conventions",
            vec![
                Action::OpenProvingSource,
                Action::InspectEvidenceAndRelation,
                Action::OpenRelatedLinks,
                Action::OpenReference,
            ],
        ),
        // 5. External boundary / partial evidence / none → partial, source file (declared in
        //    config).
        tree_node(
            "node-external-billing",
            "External billing boundary",
            Kind::ExternalBoundary,
            "config/external.toml",
            "external.billing",
            Evidence::PartialEvidence,
            Certainty::Partial,
            Relation::None,
            "no in-repo relation",
            "tests: none; story: none; doc: external.md",
            "External boundary with only partial evidence; treat the relation as incomplete",
            Link::SourceFile,
            "src:config/external.toml",
            vec![
                Action::OpenProvingSource,
                Action::InspectEvidenceAndRelation,
                Action::OpenRelatedLinks,
                Action::OpenReference,
            ],
        ),
        // 6. Unknown node / unresolved / none → partial, no proving source.
        tree_node(
            "node-unknown",
            "Unresolved node",
            Kind::UnknownNode,
            "no static source",
            "unresolved symbol",
            Evidence::Unresolved,
            Certainty::Partial,
            Relation::None,
            "no resolvable relation",
            "tests: none; story: none; doc: none",
            "Unresolved node of unknown kind; there is no proving source to open",
            Link::NoProvingSource,
            "",
            vec![
                Action::OpenProvingSource,
                Action::InspectEvidenceAndRelation,
                Action::OpenRelatedLinks,
            ],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5FrameworkDowngradeTrigger> {
    vec![
        M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated,
        M5FrameworkDowngradeTrigger::AuthorshipUnstated,
        M5FrameworkDowngradeTrigger::ProvingSourceOmitted,
        M5FrameworkDowngradeTrigger::DerivedStateUnlabeled,
        M5FrameworkDowngradeTrigger::AlternateStateLabelInvented,
        M5FrameworkDowngradeTrigger::ProofStale,
    ]
}

fn route_topology_review() -> RouteTopologyReview {
    RouteTopologyReview {
        route_row_shows_matcher_and_source: true,
        route_row_shows_kind_and_owner: true,
        route_row_offers_proving_source: true,
        tree_node_shows_entity_and_source: true,
        tree_node_shows_relation_and_links: true,
        tree_node_offers_proving_source: true,
        certainty_and_authorship_derived_never_asserted: true,
        heuristic_never_shown_as_exact: true,
        authored_versus_generated_visible_at_row_level: true,
        partial_or_derived_always_labelled: true,
        every_row_links_to_proving_source: true,
        runtime_only_never_fakes_a_source: true,
        no_surface_invents_alternate_state_label: true,
        components_stable_across_deployment_lines: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> RouteTopologyConsumerProjection {
    RouteTopologyConsumerProjection {
        route_explorer_reads_single_source: true,
        topology_explorer_reads_single_source: true,
        editor_gutter_reads_single_source: true,
        certainty_and_authorship_visible_before_trust: true,
        proving_source_reachable_before_trust: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> RouteTopologyProofFreshness {
    RouteTopologyProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        ROUTE_TREE_CONTROLS_SCHEMA_REF,
        ROUTE_TREE_CONTROLS_DOC_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_DOC_REF,
        M5_ROUTE_ENDPOINT_ROW_SCHEMA_REF,
        M5_COMPONENT_SERVICE_TREE_NODE_SCHEMA_REF,
    ])
}

/// Builds the canonical route-endpoint-row / component-service-tree-node controls packet.
pub fn seeded_route_tree_controls() -> RouteEndpointTreeNodeControlsPacket {
    RouteEndpointTreeNodeControlsPacket::new(RouteEndpointTreeNodeControlsPacketInput {
        packet_id: ROUTE_TREE_CONTROLS_PACKET_ID.to_owned(),
        surface_label:
            "M5 route / endpoint rows and component / service tree nodes: route / matcher, source file / symbol, HTTP / UI / runtime kind, owning framework / app, params / guards, freshness, evidence source, authored-versus-generated state, exact-versus-heuristic-versus-runtime-confirmed certainty, and canonical proving-source truth across claimed topology explorers"
                .to_owned(),
        route_rows: route_rows(),
        tree_nodes: tree_nodes(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
        route_topology_review: route_topology_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Scenario fixture: spotlights a heuristic, generated route row that must never read as an exact,
/// authored one. Every route evidence class, authorship, route kind, and freshness state stays
/// covered so the fixture validates on its own.
pub fn seeded_route_tree_controls_heuristic_generated_route() -> RouteEndpointTreeNodeControlsPacket
{
    let mut packet = seeded_route_tree_controls();
    packet.packet_id =
        "m5-route-endpoint-component-service-tree-controls:fixture:heuristic-generated-route"
            .to_owned();
    packet.surface_label =
        "M5 route / endpoint rows: a heuristic, generated route never reads as exact or authored"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights an inferred / unresolved tree node that must never pretend to link
/// to a proving source it does not have. Every topology node kind, evidence class, and relation kind
/// stays covered so the fixture validates on its own.
pub fn seeded_route_tree_controls_inferred_node() -> RouteEndpointTreeNodeControlsPacket {
    let mut packet = seeded_route_tree_controls();
    packet.packet_id =
        "m5-route-endpoint-component-service-tree-controls:fixture:inferred-node".to_owned();
    packet.surface_label =
        "M5 component / service tree nodes: an inferred or unresolved node never fakes a proving source"
            .to_owned();
    packet
}
