//! Canonical M5 graph-governance matrix: the single qualification report that freezes
//! the workset-scope, topology-identity, impact-query, ownership-source, architecture
//! explainer, graph-freshness, and navigation-recall depth lanes into one
//! non-inheriting graph-depth gate.
//!
//! Each [`GraphGovernanceRow`] governs one M5 code-understanding depth lane
//! ([`GraphDepthLane`]) against the canonical graph-truth packet it draws from, and
//! answers, for that lane, who owns the evidence ([`GraphGovernanceRow::owner`]), how
//! much of the workspace it actually knows ([`ScopeMode`]), how fresh the graph is
//! ([`GraphFreshness`]), how exact its relations are ([`RelationFidelity`]), and how its
//! prose and edges are backed ([`EvidenceBacking`]). The row then publishes a
//! [`GraphDepthClaim`] no input can exceed.
//!
//! The [`GraphDepthClaim`] a lane may publish is the weakest ceiling implied by its
//! observed states, so a workset-, hot-set-, or unscoped slice, a stale or expired graph,
//! approximate or unresolved relations, or generated or uncited explanation all narrow or
//! withhold the published claim automatically. The guardrail this enforces: a lane never
//! implies whole-workspace certainty when it only knows the current workset, hot set, or
//! policy-limited slice — a lane whose scope shrank, whose index went stale, whose
//! relations are approximate, or whose explanation lost its citations is narrowed to a
//! scope-qualified or provisional label, or withheld from publication entirely, rather than
//! left quietly authoritative. The [`GraphGovernanceDecision`] records the gate's action —
//! publish the lane, qualify it to the active slice, mark it provisional, or withhold it —
//! and the recomputed [`DowngradeReason`]s and [`DowngradePath`] explain it; all are
//! validated against the gate.
//!
//! Impact reasoning stays explicit. Each row carries an [`ImpactResultClass`] plus a
//! hidden-result and an out-of-scope count, so the matrix keeps no-impact, out-of-scope,
//! and policy-limited results distinct instead of collapsing them into one empty answer,
//! and an authoritative lane is forbidden from hiding any result.
//!
//! The lane vocabulary is closed and provenance-bound. [`GraphDepthLane`] is the single
//! controlled vocabulary the matrix reuses, and each lane is pinned to the canonical
//! graph-truth packet it governs via [`GraphDepthLane::source_packet`], so a clean
//! topology lane never lends its certainty to a withheld navigation-recall lane, and no
//! scope-sensitive lane inherits a broader whole-workspace claim.
//!
//! Because every row also carries a release-evidence ref, a help-surface ref, a docs-badge
//! ref, and a support-export ref, release evidence, help/service-health, docs, and support
//! exports ingest the *same* governance packet rather than parallel spreadsheets, so a
//! narrowed lane cannot stay authoritative in one surface while it is downgraded in another.
//!
//! The packet is checked in at `artifacts/graph/m5/m5-graph-governance.json` and embedded
//! here. It is metadata-only: every field is a typed state, a count, or an opaque ref, and
//! it carries no credential bodies, raw provider payloads, or graph node contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 graph-governance matrix schema version.
pub const M5_GRAPH_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_GRAPH_GOVERNANCE_RECORD_KIND: &str = "m5_graph_governance_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_GRAPH_GOVERNANCE_PATH: &str = "artifacts/graph/m5/m5-graph-governance.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_GRAPH_GOVERNANCE_SCHEMA_REF: &str = "schemas/graph/m5-graph-governance.schema.json";

/// Repo-relative path to the companion document.
pub const M5_GRAPH_GOVERNANCE_DOC_REF: &str = "docs/graph/m5/m5-graph-governance.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_GRAPH_GOVERNANCE_FIXTURE_DIR: &str = "fixtures/graph/m5/m5-graph-governance";

/// Embedded checked-in packet JSON.
pub const M5_GRAPH_GOVERNANCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/graph/m5/m5-graph-governance.json"
));

/// An M5 code-understanding depth lane the governance matrix gates.
///
/// Each lane is governed from the canonical graph-truth packet it draws its evidence from,
/// so the matrix aggregates the landed stable-line graph packets into one report instead of
/// re-deriving each lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphDepthLane {
    /// Workset and sparse-scope honesty: which slice of the workspace the lane knows.
    WorksetScope,
    /// Topology node and edge identity stability across views.
    GraphTopology,
    /// Impact reasoning: distinguishing no-impact from out-of-scope or policy-limited.
    ImpactQuery,
    /// Ownership-source classification of graph ownership edges.
    OwnershipSource,
    /// Generated-versus-curated architecture-explainer evidence and citations.
    ArchitectureExplainer,
    /// Graph freshness and invalidation propagation.
    GraphFreshness,
    /// Graph-backed navigation, docs recall, and onboarding context.
    NavigationRecall,
}

impl GraphDepthLane {
    /// Every depth lane, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::WorksetScope,
        Self::GraphTopology,
        Self::ImpactQuery,
        Self::OwnershipSource,
        Self::ArchitectureExplainer,
        Self::GraphFreshness,
        Self::NavigationRecall,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorksetScope => "workset_scope",
            Self::GraphTopology => "graph_topology",
            Self::ImpactQuery => "impact_query",
            Self::OwnershipSource => "ownership_source",
            Self::ArchitectureExplainer => "architecture_explainer",
            Self::GraphFreshness => "graph_freshness",
            Self::NavigationRecall => "navigation_recall",
        }
    }

    /// Repo-relative path to the canonical graph-truth packet this lane governs.
    ///
    /// The matrix is pinned to this packet so a lane never publishes a claim its own source
    /// packet does not back, and the `packet_ref` recorded on every row is validated against
    /// it.
    pub const fn source_packet(self) -> &'static str {
        match self {
            Self::WorksetScope => "artifacts/search/m4/scope_provenance_truth_packet.json",
            Self::GraphTopology => {
                "artifacts/graph/m4/semantic-graph-object-model-and-query-contract.json"
            }
            Self::ImpactQuery | Self::OwnershipSource => {
                "artifacts/search/m4/knowledge_evidence_packet.json"
            }
            Self::ArchitectureExplainer => {
                "artifacts/search/m4/audit_topology_explainer_companion_truth_packet.json"
            }
            Self::GraphFreshness => "artifacts/search/m4/freshness_propagation_packet.json",
            Self::NavigationRecall => "artifacts/search/m4/navigation_target_truth_packet.json",
        }
    }

    /// Whether this lane reasons over the active workset, hot set, or recall slice rather
    /// than the whole structural graph, and so must narrow safely instead of inheriting a
    /// broader whole-workspace claim.
    pub const fn is_scope_sensitive(self) -> bool {
        matches!(
            self,
            Self::WorksetScope | Self::ImpactQuery | Self::NavigationRecall
        )
    }
}

/// How authoritative a lane's published graph-depth claim is.
///
/// Ordered low-to-high by [`GraphDepthClaim::rank`]: a [`GraphDepthClaim::Withheld`] lane has
/// no publishable claim, and an [`GraphDepthClaim::Authoritative`] lane is backed by a
/// full-workspace, fresh, exact, curated graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphDepthClaim {
    /// A whole-workspace claim backed by fresh, exact, curated truth.
    Authoritative,
    /// Narrowed to the active workset, hot set, or resolved slice.
    ScopeQualified,
    /// Narrowed to a provisional label; approximate, stale, or generated.
    Provisional,
    /// Withheld from publication; no publishable claim.
    Withheld,
}

impl GraphDepthClaim {
    /// Every depth claim, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Authoritative,
        Self::ScopeQualified,
        Self::Provisional,
        Self::Withheld,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::ScopeQualified => "scope_qualified",
            Self::Provisional => "provisional",
            Self::Withheld => "withheld",
        }
    }

    /// Monotonic rank; higher means more authoritative.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Withheld => 0,
            Self::Provisional => 1,
            Self::ScopeQualified => 2,
            Self::Authoritative => 3,
        }
    }

    /// The weaker (lower-rank) of two depth claims.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }
}

/// How much of the workspace a lane actually knows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeMode {
    /// The lane spans the full workspace graph.
    FullWorkspace,
    /// The lane knows only the active workset; caps at scope-qualified.
    Workset,
    /// The lane knows only the indexed hot set; caps at provisional.
    HotSet,
    /// The lane has no resolvable scope; caps at withheld.
    Unscoped,
}

impl ScopeMode {
    /// Every scope mode, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullWorkspace,
        Self::Workset,
        Self::HotSet,
        Self::Unscoped,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullWorkspace => "full_workspace",
            Self::Workset => "workset",
            Self::HotSet => "hot_set",
            Self::Unscoped => "unscoped",
        }
    }

    /// Highest depth claim this scope mode permits a lane to publish.
    pub const fn claim_ceiling(self) -> GraphDepthClaim {
        match self {
            Self::FullWorkspace => GraphDepthClaim::Authoritative,
            Self::Workset => GraphDepthClaim::ScopeQualified,
            Self::HotSet => GraphDepthClaim::Provisional,
            Self::Unscoped => GraphDepthClaim::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::ScopeNarrowed`] trigger.
    pub const fn is_narrow_trigger(self) -> bool {
        !matches!(self, Self::FullWorkspace)
    }
}

/// How fresh the graph backing a lane is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphFreshness {
    /// The graph index is current.
    Fresh,
    /// The graph index is lagging but within tolerance; caps at scope-qualified.
    Lagging,
    /// The graph index is stale; caps at provisional.
    Stale,
    /// The graph index is expired; caps at withheld.
    Expired,
}

impl GraphFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Fresh, Self::Lagging, Self::Stale, Self::Expired];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Lagging => "lagging",
            Self::Stale => "stale",
            Self::Expired => "expired",
        }
    }

    /// Highest depth claim this freshness state permits a lane to publish.
    pub const fn claim_ceiling(self) -> GraphDepthClaim {
        match self {
            Self::Fresh => GraphDepthClaim::Authoritative,
            Self::Lagging => GraphDepthClaim::ScopeQualified,
            Self::Stale => GraphDepthClaim::Provisional,
            Self::Expired => GraphDepthClaim::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::StaleGraph`] trigger.
    pub const fn is_stale_trigger(self) -> bool {
        matches!(self, Self::Stale | Self::Expired)
    }
}

/// How exact the relations a lane reports are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationFidelity {
    /// Relations are exact.
    Exact,
    /// Relations are resolved but heuristically completed; caps at scope-qualified.
    Resolved,
    /// Relations are approximate; caps at provisional.
    Approximate,
    /// Relations are unresolved; caps at withheld.
    Unresolved,
}

impl RelationFidelity {
    /// Every fidelity state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Exact,
        Self::Resolved,
        Self::Approximate,
        Self::Unresolved,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Resolved => "resolved",
            Self::Approximate => "approximate",
            Self::Unresolved => "unresolved",
        }
    }

    /// Highest depth claim this fidelity state permits a lane to publish.
    pub const fn claim_ceiling(self) -> GraphDepthClaim {
        match self {
            Self::Exact => GraphDepthClaim::Authoritative,
            Self::Resolved => GraphDepthClaim::ScopeQualified,
            Self::Approximate => GraphDepthClaim::Provisional,
            Self::Unresolved => GraphDepthClaim::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::ApproximateRelations`] trigger.
    pub const fn is_approximate_trigger(self) -> bool {
        matches!(self, Self::Approximate | Self::Unresolved)
    }
}

/// How a lane's prose and edges are backed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceBacking {
    /// Curated, citation-complete truth.
    Curated,
    /// Generated but fully citation-backed; caps at scope-qualified.
    Cited,
    /// Generated and only partly cited; caps at provisional.
    Generated,
    /// Generated and uncited; caps at withheld.
    Uncited,
}

impl EvidenceBacking {
    /// Every backing state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Curated, Self::Cited, Self::Generated, Self::Uncited];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Curated => "curated",
            Self::Cited => "cited",
            Self::Generated => "generated",
            Self::Uncited => "uncited",
        }
    }

    /// Highest depth claim this backing state permits a lane to publish.
    pub const fn claim_ceiling(self) -> GraphDepthClaim {
        match self {
            Self::Curated => GraphDepthClaim::Authoritative,
            Self::Cited => GraphDepthClaim::ScopeQualified,
            Self::Generated => GraphDepthClaim::Provisional,
            Self::Uncited => GraphDepthClaim::Withheld,
        }
    }

    /// Whether this state raises the [`DowngradeReason::UncitedExplanation`] trigger.
    pub const fn is_uncited_trigger(self) -> bool {
        matches!(self, Self::Generated | Self::Uncited)
    }

    /// Whether this is a generated (non-curated) explanation source.
    pub const fn is_generated(self) -> bool {
        matches!(self, Self::Generated | Self::Uncited)
    }
}

/// How an impact-query result is classed, keeping no-impact distinct from out-of-scope and
/// policy-limited results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactResultClass {
    /// The query resolved fully and found no impact in scope.
    NoImpact,
    /// The query found impact within the current scope.
    InScopeImpact,
    /// Candidate impacts resolve outside the current slice.
    OutOfScope,
    /// Results were suppressed by an access or visibility policy.
    PolicyLimited,
}

impl ImpactResultClass {
    /// Every impact-result class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoImpact,
        Self::InScopeImpact,
        Self::OutOfScope,
        Self::PolicyLimited,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoImpact => "no_impact",
            Self::InScopeImpact => "in_scope_impact",
            Self::OutOfScope => "out_of_scope",
            Self::PolicyLimited => "policy_limited",
        }
    }
}

/// The recovery path surfaced when a lane's claim is narrowed or withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradePath {
    /// Widen the active workset or hot set toward the full workspace.
    WidenScope,
    /// Reindex the graph to refresh stale or expired state.
    Reindex,
    /// Resolve approximate or unresolved relations.
    ResolveRelations,
    /// Cite or curate generated explanation prose.
    CiteOrCurate,
    /// Withhold the lane's claim from publication.
    WithholdClaim,
    /// No downgrade is needed; only valid when the lane is published authoritative.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl DowngradePath {
    /// Every downgrade path, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WidenScope,
        Self::Reindex,
        Self::ResolveRelations,
        Self::CiteOrCurate,
        Self::WithholdClaim,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WidenScope => "widen_scope",
            Self::Reindex => "reindex",
            Self::ResolveRelations => "resolve_relations",
            Self::CiteOrCurate => "cite_or_curate",
            Self::WithholdClaim => "withhold_claim",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real recovery path the lane owner can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// A headline reason the governance gate narrows a lane.
///
/// These are the canonical downgrade reasons: a narrowed scope, a stale graph, approximate
/// relations, and uncited explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReason {
    /// The lane knows only a workset, hot set, or unscoped slice.
    ScopeNarrowed,
    /// The lane's graph index is stale or expired.
    StaleGraph,
    /// The lane's relations are approximate or unresolved.
    ApproximateRelations,
    /// The lane's explanation is generated and only partly or wholly uncited.
    UncitedExplanation,
}

impl DowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ScopeNarrowed,
        Self::StaleGraph,
        Self::ApproximateRelations,
        Self::UncitedExplanation,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScopeNarrowed => "scope_narrowed",
            Self::StaleGraph => "stale_graph",
            Self::ApproximateRelations => "approximate_relations",
            Self::UncitedExplanation => "uncited_explanation",
        }
    }
}

/// The action the governance gate takes on a lane relative to a clean authoritative claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphGovernanceDecision {
    /// No narrowing; the lane publishes an authoritative claim.
    Publish,
    /// The lane is qualified to the active workset, hot set, or resolved slice.
    QualifyScope,
    /// The lane is marked provisional.
    MarkProvisional,
    /// The lane's claim is withheld from publication.
    Withhold,
}

impl GraphGovernanceDecision {
    /// Every governance decision, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Publish,
        Self::QualifyScope,
        Self::MarkProvisional,
        Self::Withhold,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Publish => "publish",
            Self::QualifyScope => "qualify_scope",
            Self::MarkProvisional => "mark_provisional",
            Self::Withhold => "withhold",
        }
    }

    /// Whether the gate narrowed or withheld the lane's claim.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::Publish)
    }

    /// The decision implied by a published depth claim.
    pub const fn for_claim(claim: GraphDepthClaim) -> Self {
        match claim {
            GraphDepthClaim::Authoritative => Self::Publish,
            GraphDepthClaim::ScopeQualified => Self::QualifyScope,
            GraphDepthClaim::Provisional => Self::MarkProvisional,
            GraphDepthClaim::Withheld => Self::Withhold,
        }
    }
}

/// One governance row for an M5 code-understanding depth lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphGovernanceRow {
    /// Stable governance-row id.
    pub lane_id: String,
    /// Depth lane this row governs.
    pub lane: GraphDepthLane,
    /// Owner accountable for the lane's evidence and conformance.
    pub owner: String,
    /// How much of the workspace the lane actually knows.
    pub scope_mode: ScopeMode,
    /// How fresh the lane's graph index is.
    pub graph_freshness: GraphFreshness,
    /// How exact the lane's relations are.
    pub relation_fidelity: RelationFidelity,
    /// How the lane's prose and edges are backed.
    pub evidence_backing: EvidenceBacking,
    /// How the lane's impact-query results are classed.
    pub impact_result_class: ImpactResultClass,
    /// Results hidden by an access or visibility policy.
    pub hidden_result_count: u64,
    /// Candidate results that resolve outside the lane's current scope.
    pub out_of_scope_count: u64,
    /// Stable namespace the lane mints topology node identities under.
    pub node_id_namespace: String,
    /// Stable namespace the lane mints topology edge identities under.
    pub edge_id_namespace: String,
    /// Depth claim the lane's own evidence asserts, before the gate.
    pub declared_claim: GraphDepthClaim,
    /// Depth claim actually published after the gate narrows the lane.
    ///
    /// Must equal [`GraphGovernanceRow::effective_claim`].
    pub published_claim: GraphDepthClaim,
    /// Decision the gate takes; must equal the recomputed decision.
    pub governance_decision: GraphGovernanceDecision,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Recovery path surfaced when the claim is narrowed or withheld.
    pub downgrade_path: DowngradePath,
    /// Scope or slice labels this lane still backs.
    #[serde(default)]
    pub supported_scopes: Vec<String>,
    /// Caveats attached to the published claim.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale, missing, or narrowing the claim.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Ref to the canonical graph-truth packet this lane governs.
    ///
    /// Must equal [`GraphDepthLane::source_packet`].
    pub packet_ref: String,
    /// Ref to the graph-conformance suite backing the lane.
    pub conformance_ref: String,
    /// Ref to the lane's supporting evidence.
    pub evidence_ref: String,
    /// Ref to the machine-readable governance receipt for audit and release evidence.
    pub governance_receipt_ref: String,
    /// Ref binding this row into the release-evidence surface.
    pub release_evidence_ref: String,
    /// Ref binding this row into the help/service-health surface.
    pub help_surface_ref: String,
    /// Ref binding this row into the docs-badge surface.
    pub docs_badge_ref: String,
    /// Ref binding this row into the support-export surface.
    pub support_export_ref: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl GraphGovernanceRow {
    /// The claim the lane's own evidence asserted, before environmental narrowing.
    pub fn capability_floor(&self) -> GraphDepthClaim {
        self.declared_claim
    }

    /// The depth claim the gate permits this lane to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the scope mode, graph
    /// freshness, relation fidelity, and evidence backing, so a slice-only scope, a stale or
    /// expired index, approximate relations, or uncited explanation can never publish an
    /// authoritative claim.
    pub fn effective_claim(&self) -> GraphDepthClaim {
        self.capability_floor()
            .min(self.scope_mode.claim_ceiling())
            .min(self.graph_freshness.claim_ceiling())
            .min(self.relation_fidelity.claim_ceiling())
            .min(self.evidence_backing.claim_ceiling())
    }

    /// The headline downgrade reasons recomputed from the lane's observed states.
    pub fn computed_downgrade_reasons(&self) -> Vec<DowngradeReason> {
        let mut reasons = Vec::new();
        if self.scope_mode.is_narrow_trigger() {
            reasons.push(DowngradeReason::ScopeNarrowed);
        }
        if self.graph_freshness.is_stale_trigger() {
            reasons.push(DowngradeReason::StaleGraph);
        }
        if self.relation_fidelity.is_approximate_trigger() {
            reasons.push(DowngradeReason::ApproximateRelations);
        }
        if self.evidence_backing.is_uncited_trigger() {
            reasons.push(DowngradeReason::UncitedExplanation);
        }
        reasons
    }

    /// The decision the gate must record for this lane, derived from its effective claim.
    pub fn required_decision(&self) -> GraphGovernanceDecision {
        GraphGovernanceDecision::for_claim(self.effective_claim())
    }

    /// Whether the lane publishes a clean authoritative claim.
    pub fn is_authoritative(&self) -> bool {
        self.effective_claim() == GraphDepthClaim::Authoritative
    }

    /// Whether the gate narrowed the published claim below what the lane declared.
    ///
    /// This is the automatic downgrade: a slice-only, stale, approximate, or uncited lane
    /// that declared a stronger claim has its published claim lowered rather than left
    /// quietly authoritative.
    pub fn is_downgraded(&self) -> bool {
        self.effective_claim().rank() < self.capability_floor().rank()
    }

    /// Whether the lane carries its own non-empty source, conformance, evidence, receipt, and
    /// downstream-consumer refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.packet_ref.trim().is_empty()
            && !self.conformance_ref.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
            && !self.governance_receipt_ref.trim().is_empty()
            && !self.release_evidence_ref.trim().is_empty()
            && !self.help_surface_ref.trim().is_empty()
            && !self.docs_badge_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// Whether the stored published claim, decision, and downgrade reasons all agree with the
    /// recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_claim == self.effective_claim()
            && self.governance_decision == self.required_decision()
            && self.downgrade_reasons == self.computed_downgrade_reasons()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5GraphGovernanceSummary {
    /// Total lane rows.
    pub total_lanes: usize,
    /// Number of claimed lanes.
    pub lane_count: usize,
    /// Lanes published as authoritative.
    pub authoritative_lanes: usize,
    /// Lanes narrowed to a scope-qualified claim.
    pub scope_qualified_lanes: usize,
    /// Lanes narrowed to a provisional claim.
    pub provisional_lanes: usize,
    /// Lanes withheld from publication.
    pub withheld_lanes: usize,
    /// Lanes the gate published.
    pub publish_decisions: usize,
    /// Lanes the gate qualified to a slice.
    pub qualify_scope_decisions: usize,
    /// Lanes the gate marked provisional.
    pub mark_provisional_decisions: usize,
    /// Lanes the gate withheld.
    pub withhold_decisions: usize,
    /// Lanes whose published claim was downgraded below what they declared.
    pub downgraded_lanes: usize,
    /// Scope-sensitive lanes.
    pub scope_sensitive_lanes: usize,
    /// Lanes whose graph index is stale or expired.
    pub stale_graph_lanes: usize,
    /// Lanes whose explanation is generated rather than curated.
    pub generated_explanation_lanes: usize,
    /// Lanes carrying at least one downgrade reason.
    pub lanes_with_downgrade_reasons: usize,
    /// Lanes reporting at least one policy-hidden result.
    pub lanes_with_hidden_results: usize,
    /// Lanes reporting at least one out-of-scope result.
    pub lanes_with_out_of_scope_results: usize,
}

/// A redaction-safe export row projected from a governance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GraphGovernanceExportRow {
    /// Governance-row id.
    pub lane_id: String,
    /// Lane token.
    pub lane: String,
    /// Owner accountable for the lane.
    pub owner: String,
    /// Scope-mode token.
    pub scope_mode: String,
    /// Graph-freshness token.
    pub graph_freshness: String,
    /// Relation-fidelity token.
    pub relation_fidelity: String,
    /// Evidence-backing token.
    pub evidence_backing: String,
    /// Impact-result-class token.
    pub impact_result_class: String,
    /// Results hidden by policy.
    pub hidden_result_count: u64,
    /// Results out of the lane's scope.
    pub out_of_scope_count: u64,
    /// Declared-claim token.
    pub declared_claim: String,
    /// Published-claim token.
    pub published_claim: String,
    /// Governance-decision token.
    pub governance_decision: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Downgrade-path token.
    pub downgrade_path: String,
    /// Supported scope or slice labels.
    pub supported_scopes: Vec<String>,
    /// Caveats attached to the published claim.
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale or missing.
    pub stale_or_missing_fields: Vec<String>,
    /// Source-packet ref this lane governs.
    pub packet_ref: String,
    /// Governance-receipt ref.
    pub governance_receipt_ref: String,
    /// Release-evidence ref.
    pub release_evidence_ref: String,
    /// Help-surface ref.
    pub help_surface_ref: String,
    /// Docs-badge ref.
    pub docs_badge_ref: String,
    /// Support-export ref.
    pub support_export_ref: String,
    /// Whether the lane is scope-sensitive.
    pub scope_sensitive: bool,
    /// Whether the lane publishes an authoritative claim.
    pub authoritative: bool,
    /// Whether the published claim was downgraded below the declared claim.
    pub downgraded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet — the canonical graph-depth index
/// downstream surfaces render instead of restating each lane's claim by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GraphGovernanceExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub lanes: Vec<M5GraphGovernanceExportRow>,
    /// Whether every lane's published claim and decision agree with the gate.
    pub all_lanes_gate_consistent: bool,
    /// Lanes that publish an authoritative claim.
    pub authoritative_count: usize,
    /// Lanes the gate narrowed or withheld.
    pub narrowed_count: usize,
    /// Lanes the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed M5 graph-governance matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5GraphGovernanceMatrix {
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
    /// Scheme the matrix mints stable topology identities under.
    pub topology_identity_scheme: String,
    /// Claimed lanes; one row per lane.
    pub lanes: Vec<GraphDepthLane>,
    /// Closed depth-claim vocabulary.
    pub depth_claims: Vec<GraphDepthClaim>,
    /// Closed scope-mode vocabulary.
    pub scope_modes: Vec<ScopeMode>,
    /// Closed graph-freshness vocabulary.
    pub graph_freshness_states: Vec<GraphFreshness>,
    /// Closed relation-fidelity vocabulary.
    pub relation_fidelities: Vec<RelationFidelity>,
    /// Closed evidence-backing vocabulary.
    pub evidence_backings: Vec<EvidenceBacking>,
    /// Closed impact-result-class vocabulary.
    pub impact_result_classes: Vec<ImpactResultClass>,
    /// Closed downgrade-path vocabulary.
    pub downgrade_paths: Vec<DowngradePath>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Closed governance-decision vocabulary.
    pub governance_decisions: Vec<GraphGovernanceDecision>,
    /// Governance rows, one per claimed lane.
    #[serde(default)]
    pub lane_rows: Vec<GraphGovernanceRow>,
    /// Summary counts.
    pub summary: M5GraphGovernanceSummary,
}

impl M5GraphGovernanceMatrix {
    /// Returns the row for a claimed lane.
    pub fn lane_row(&self, lane: GraphDepthLane) -> Option<&GraphGovernanceRow> {
        self.lane_rows.iter().find(|c| c.lane == lane)
    }

    /// Lanes that publish an authoritative claim.
    pub fn authoritative_lanes(&self) -> impl Iterator<Item = &GraphGovernanceRow> {
        self.lane_rows.iter().filter(|c| c.is_authoritative())
    }

    /// Lanes the gate narrowed or withheld in any way.
    pub fn narrowed_lanes(&self) -> impl Iterator<Item = &GraphGovernanceRow> {
        self.lane_rows
            .iter()
            .filter(|c| c.required_decision().is_narrowed())
    }

    /// Lanes the gate withheld entirely.
    pub fn withheld_lanes(&self) -> impl Iterator<Item = &GraphGovernanceRow> {
        self.lane_rows
            .iter()
            .filter(|c| c.required_decision() == GraphGovernanceDecision::Withhold)
    }

    /// Whether every lane's stored published claim, decision, and reasons agree with the
    /// recomputed gate decision.
    pub fn all_lanes_gate_consistent(&self) -> bool {
        self.lane_rows.iter().all(|c| c.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5GraphGovernanceSummary {
        let count_published = |claim: GraphDepthClaim| {
            self.lane_rows
                .iter()
                .filter(|c| c.published_claim == claim)
                .count()
        };
        let count_decision = |decision: GraphGovernanceDecision| {
            self.lane_rows
                .iter()
                .filter(|c| c.governance_decision == decision)
                .count()
        };
        M5GraphGovernanceSummary {
            total_lanes: self.lane_rows.len(),
            lane_count: self.lanes.len(),
            authoritative_lanes: count_published(GraphDepthClaim::Authoritative),
            scope_qualified_lanes: count_published(GraphDepthClaim::ScopeQualified),
            provisional_lanes: count_published(GraphDepthClaim::Provisional),
            withheld_lanes: count_published(GraphDepthClaim::Withheld),
            publish_decisions: count_decision(GraphGovernanceDecision::Publish),
            qualify_scope_decisions: count_decision(GraphGovernanceDecision::QualifyScope),
            mark_provisional_decisions: count_decision(GraphGovernanceDecision::MarkProvisional),
            withhold_decisions: count_decision(GraphGovernanceDecision::Withhold),
            downgraded_lanes: self.lane_rows.iter().filter(|c| c.is_downgraded()).count(),
            scope_sensitive_lanes: self
                .lane_rows
                .iter()
                .filter(|c| c.lane.is_scope_sensitive())
                .count(),
            stale_graph_lanes: self
                .lane_rows
                .iter()
                .filter(|c| c.graph_freshness.is_stale_trigger())
                .count(),
            generated_explanation_lanes: self
                .lane_rows
                .iter()
                .filter(|c| c.evidence_backing.is_generated())
                .count(),
            lanes_with_downgrade_reasons: self
                .lane_rows
                .iter()
                .filter(|c| !c.downgrade_reasons.is_empty())
                .count(),
            lanes_with_hidden_results: self
                .lane_rows
                .iter()
                .filter(|c| c.hidden_result_count > 0)
                .count(),
            lanes_with_out_of_scope_results: self
                .lane_rows
                .iter()
                .filter(|c| c.out_of_scope_count > 0)
                .count(),
        }
    }

    /// Produces the graph-depth index downstream surfaces — release evidence,
    /// help/service-health, docs badges, and support exports — render instead of restating
    /// each lane's depth posture by hand.
    pub fn export_projection(&self) -> M5GraphGovernanceExportProjection {
        let lanes = self
            .lane_rows
            .iter()
            .map(|c| M5GraphGovernanceExportRow {
                lane_id: c.lane_id.clone(),
                lane: c.lane.as_str().to_owned(),
                owner: c.owner.clone(),
                scope_mode: c.scope_mode.as_str().to_owned(),
                graph_freshness: c.graph_freshness.as_str().to_owned(),
                relation_fidelity: c.relation_fidelity.as_str().to_owned(),
                evidence_backing: c.evidence_backing.as_str().to_owned(),
                impact_result_class: c.impact_result_class.as_str().to_owned(),
                hidden_result_count: c.hidden_result_count,
                out_of_scope_count: c.out_of_scope_count,
                declared_claim: c.declared_claim.as_str().to_owned(),
                published_claim: c.published_claim.as_str().to_owned(),
                governance_decision: c.governance_decision.as_str().to_owned(),
                downgrade_reasons: c
                    .downgrade_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                downgrade_path: c.downgrade_path.as_str().to_owned(),
                supported_scopes: c.supported_scopes.clone(),
                caveats: c.caveats.clone(),
                stale_or_missing_fields: c.stale_or_missing_fields.clone(),
                packet_ref: c.packet_ref.clone(),
                governance_receipt_ref: c.governance_receipt_ref.clone(),
                release_evidence_ref: c.release_evidence_ref.clone(),
                help_surface_ref: c.help_surface_ref.clone(),
                docs_badge_ref: c.docs_badge_ref.clone(),
                support_export_ref: c.support_export_ref.clone(),
                scope_sensitive: c.lane.is_scope_sensitive(),
                authoritative: c.is_authoritative(),
                downgraded: c.is_downgraded(),
                summary: format!(
                    "{}: scope {}, freshness {}, relations {}, backing {}, declared {}, published {} ({}), recovery {}",
                    c.lane.as_str(),
                    c.scope_mode.as_str(),
                    c.graph_freshness.as_str(),
                    c.relation_fidelity.as_str(),
                    c.evidence_backing.as_str(),
                    c.declared_claim.as_str(),
                    c.published_claim.as_str(),
                    c.governance_decision.as_str(),
                    c.downgrade_path.as_str()
                ),
            })
            .collect();
        M5GraphGovernanceExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            lanes,
            all_lanes_gate_consistent: self.all_lanes_gate_consistent(),
            authoritative_count: self.authoritative_lanes().count(),
            narrowed_count: self.narrowed_lanes().count(),
            withheld_count: self.withheld_lanes().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5GraphGovernanceViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<GraphDepthLane> = self.lanes.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_lanes = BTreeSet::new();
        for row in &self.lane_rows {
            if !seen_ids.insert(row.lane_id.clone()) {
                violations.push(M5GraphGovernanceViolation::DuplicateLaneId {
                    lane_id: row.lane_id.clone(),
                });
            }
            if !seen_lanes.insert(row.lane) {
                violations.push(M5GraphGovernanceViolation::DuplicateLaneRow {
                    lane: row.lane.as_str(),
                });
            }
            if !claimed.contains(&row.lane) {
                violations.push(M5GraphGovernanceViolation::UnclaimedLaneRow {
                    lane_id: row.lane_id.clone(),
                    lane: row.lane.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed lane must carry its own row, so a lane never inherits an authoritative
        // claim from an adjacent one.
        for &lane in &self.lanes {
            if !seen_lanes.contains(&lane) {
                violations.push(M5GraphGovernanceViolation::MissingLaneRow {
                    lane: lane.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5GraphGovernanceViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5GraphGovernanceViolation>) {
        if self.schema_version != M5_GRAPH_GOVERNANCE_SCHEMA_VERSION {
            violations.push(M5GraphGovernanceViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_GRAPH_GOVERNANCE_RECORD_KIND {
            violations.push(M5GraphGovernanceViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("topology_identity_scheme", &self.topology_identity_scheme),
        ] {
            if value.trim().is_empty() {
                violations.push(M5GraphGovernanceViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            ("lanes", self.lanes == GraphDepthLane::ALL.to_vec()),
            (
                "depth_claims",
                self.depth_claims == GraphDepthClaim::ALL.to_vec(),
            ),
            ("scope_modes", self.scope_modes == ScopeMode::ALL.to_vec()),
            (
                "graph_freshness_states",
                self.graph_freshness_states == GraphFreshness::ALL.to_vec(),
            ),
            (
                "relation_fidelities",
                self.relation_fidelities == RelationFidelity::ALL.to_vec(),
            ),
            (
                "evidence_backings",
                self.evidence_backings == EvidenceBacking::ALL.to_vec(),
            ),
            (
                "impact_result_classes",
                self.impact_result_classes == ImpactResultClass::ALL.to_vec(),
            ),
            (
                "downgrade_paths",
                self.downgrade_paths == DowngradePath::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == DowngradeReason::ALL.to_vec(),
            ),
            (
                "governance_decisions",
                self.governance_decisions == GraphGovernanceDecision::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5GraphGovernanceViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &GraphGovernanceRow,
        violations: &mut Vec<M5GraphGovernanceViolation>,
    ) {
        for (field, value) in [
            ("lane_id", &row.lane_id),
            ("owner", &row.owner),
            ("node_id_namespace", &row.node_id_namespace),
            ("edge_id_namespace", &row.edge_id_namespace),
            ("packet_ref", &row.packet_ref),
            ("conformance_ref", &row.conformance_ref),
            ("evidence_ref", &row.evidence_ref),
            ("governance_receipt_ref", &row.governance_receipt_ref),
            ("release_evidence_ref", &row.release_evidence_ref),
            ("help_surface_ref", &row.help_surface_ref),
            ("docs_badge_ref", &row.docs_badge_ref),
            ("support_export_ref", &row.support_export_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5GraphGovernanceViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: field,
                });
            }
        }

        // The lane's source packet must be the canonical graph-truth packet it governs, so a
        // lane never publishes a claim its own source packet does not back.
        if row.packet_ref != row.lane.source_packet() {
            violations.push(M5GraphGovernanceViolation::SourcePacketMismatch {
                lane_id: row.lane_id.clone(),
                expected: row.lane.source_packet(),
            });
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5GraphGovernanceViolation::DuplicateDowngradeReason {
                    lane_id: row.lane_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published claim must equal the gate's recomputed ceiling, so a slice-only,
        // stale, approximate, or uncited lane can never read as authoritative.
        let effective = row.effective_claim();
        if row.published_claim != effective {
            violations.push(M5GraphGovernanceViolation::OverstatedClaim {
                lane_id: row.lane_id.clone(),
                published: row.published_claim.as_str(),
                computed: effective.as_str(),
            });
        }

        // The recorded decision must match the gate's recomputed decision.
        let required = row.required_decision();
        if row.governance_decision != required {
            violations.push(M5GraphGovernanceViolation::DecisionMismatch {
                lane_id: row.lane_id.clone(),
                declared: row.governance_decision.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded downgrade reasons must equal the reasons recomputed from the observed
        // states, so a downgrade can never be asserted or hidden by hand.
        let computed = row.computed_downgrade_reasons();
        if row.downgrade_reasons != computed {
            violations.push(M5GraphGovernanceViolation::DowngradeReasonsMismatch {
                lane_id: row.lane_id.clone(),
            });
        }

        // A narrowed or withheld lane must offer a real recovery path, list at least one
        // caveat, and name what is stale or narrowing, so a degraded lane never drops its
        // recovery semantics or hides why it was narrowed.
        if row.governance_decision.is_narrowed() {
            if !row.downgrade_path.is_offered() {
                violations.push(M5GraphGovernanceViolation::MissingDowngradePath {
                    lane_id: row.lane_id.clone(),
                });
            }
            if row.caveats.is_empty() {
                violations.push(M5GraphGovernanceViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: "caveats",
                });
            }
            if row.stale_or_missing_fields.is_empty() {
                violations.push(M5GraphGovernanceViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: "stale_or_missing_fields",
                });
            }
        }

        // A lane that still backs a publishable claim must name at least one supported scope
        // or slice label.
        if row.published_claim != GraphDepthClaim::Withheld && row.supported_scopes.is_empty() {
            violations.push(M5GraphGovernanceViolation::EmptyField {
                id: row.lane_id.clone(),
                field_name: "supported_scopes",
            });
        }

        // Impact-result classes stay distinct: no-impact never hides an out-of-scope result,
        // out-of-scope always reports a non-zero out-of-scope count, and policy-limited always
        // reports a non-zero hidden count.
        let impact_ok = match row.impact_result_class {
            ImpactResultClass::NoImpact => row.out_of_scope_count == 0,
            ImpactResultClass::OutOfScope => row.out_of_scope_count > 0,
            ImpactResultClass::PolicyLimited => row.hidden_result_count > 0,
            ImpactResultClass::InScopeImpact => true,
        };
        if !impact_ok {
            violations.push(M5GraphGovernanceViolation::ImpactCountMismatch {
                lane_id: row.lane_id.clone(),
                class: row.impact_result_class.as_str(),
            });
        }

        // An authoritative lane must be genuinely whole-knowable: a full-workspace scope, a
        // fresh, exact, curated graph, a declared authoritative floor, no downgrade reason, no
        // caveat, no stale-or-missing field, a no-op recovery path, and nothing hidden or out
        // of scope. This is the non-inheritance guardrail — a lane never implies
        // whole-workspace certainty over a slice.
        if row.is_authoritative()
            && (row.scope_mode.claim_ceiling() != GraphDepthClaim::Authoritative
                || row.graph_freshness.claim_ceiling() != GraphDepthClaim::Authoritative
                || row.relation_fidelity.claim_ceiling() != GraphDepthClaim::Authoritative
                || row.evidence_backing.claim_ceiling() != GraphDepthClaim::Authoritative
                || row.capability_floor() != GraphDepthClaim::Authoritative
                || !row.downgrade_reasons.is_empty()
                || !row.caveats.is_empty()
                || !row.stale_or_missing_fields.is_empty()
                || row.downgrade_path.is_offered()
                || row.hidden_result_count != 0
                || row.out_of_scope_count != 0)
        {
            violations.push(M5GraphGovernanceViolation::AuthoritativeLaneNotWhole {
                lane_id: row.lane_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5 graph-governance packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5GraphGovernanceViolation {
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
    /// A closed vocabulary or pinned value is not canonical.
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
    /// A governance-row id appears more than once.
    DuplicateLaneId {
        /// Duplicate lane id.
        lane_id: String,
    },
    /// A claimed lane carries more than one row.
    DuplicateLaneRow {
        /// Lane token.
        lane: &'static str,
    },
    /// A claimed lane has no row.
    MissingLaneRow {
        /// Lane token.
        lane: &'static str,
    },
    /// A row covers a lane the packet does not claim.
    UnclaimedLaneRow {
        /// Row id.
        lane_id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// A row's source packet is not the canonical packet for its lane.
    SourcePacketMismatch {
        /// Row id.
        lane_id: String,
        /// Expected source-packet path.
        expected: &'static str,
    },
    /// A row lists a downgrade reason more than once.
    DuplicateDowngradeReason {
        /// Row id.
        lane_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A lane publishes a claim beyond what its evidence supports.
    OverstatedClaim {
        /// Row id.
        lane_id: String,
        /// Published claim token.
        published: &'static str,
        /// Computed effective claim token.
        computed: &'static str,
    },
    /// A lane's decision disagrees with its gate decision.
    DecisionMismatch {
        /// Row id.
        lane_id: String,
        /// Declared decision token.
        declared: &'static str,
        /// Required decision token.
        required: &'static str,
    },
    /// A lane's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Row id.
        lane_id: String,
    },
    /// A narrowed or withheld lane offers no recovery path.
    MissingDowngradePath {
        /// Row id.
        lane_id: String,
    },
    /// A lane's impact-result class disagrees with its hidden or out-of-scope counts.
    ImpactCountMismatch {
        /// Row id.
        lane_id: String,
        /// Impact-result-class token.
        class: &'static str,
    },
    /// An authoritative lane still narrows a state, hides a result, or carries a downgrade
    /// reason.
    AuthoritativeLaneNotWhole {
        /// Row id.
        lane_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5GraphGovernanceViolation {
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
            Self::DuplicateLaneId { lane_id } => {
                write!(f, "duplicate lane id {lane_id}")
            }
            Self::DuplicateLaneRow { lane } => {
                write!(f, "duplicate row for lane {lane}")
            }
            Self::MissingLaneRow { lane } => {
                write!(f, "missing row for claimed lane {lane}")
            }
            Self::UnclaimedLaneRow { lane_id, lane } => {
                write!(f, "row {lane_id} covers unclaimed lane {lane}")
            }
            Self::SourcePacketMismatch { lane_id, expected } => {
                write!(
                    f,
                    "row {lane_id} packet_ref must be the canonical source packet {expected}"
                )
            }
            Self::DuplicateDowngradeReason { lane_id, reason } => {
                write!(f, "row {lane_id} repeats downgrade reason {reason}")
            }
            Self::OverstatedClaim {
                lane_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {lane_id} publishes claim {published} but the gate computes {computed}"
                )
            }
            Self::DecisionMismatch {
                lane_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {lane_id} records decision {declared} but the gate requires {required}"
                )
            }
            Self::DowngradeReasonsMismatch { lane_id } => {
                write!(f, "row {lane_id} downgrade reasons disagree with the gate")
            }
            Self::MissingDowngradePath { lane_id } => {
                write!(
                    f,
                    "row {lane_id} is narrowed or withheld but offers no recovery path"
                )
            }
            Self::ImpactCountMismatch { lane_id, class } => {
                write!(
                    f,
                    "row {lane_id} impact class {class} disagrees with its hidden or out-of-scope counts"
                )
            }
            Self::AuthoritativeLaneNotWhole { lane_id } => {
                write!(
                    f,
                    "row {lane_id} is authoritative but narrows a state, hides a result, or carries a downgrade reason"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5GraphGovernanceViolation {}

/// Loads the embedded M5 graph-governance matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5GraphGovernanceMatrix`].
pub fn current_m5_graph_governance_matrix() -> Result<M5GraphGovernanceMatrix, serde_json::Error> {
    serde_json::from_str(M5_GRAPH_GOVERNANCE_JSON)
}

#[cfg(test)]
mod tests;
