//! Canonical M5 impact-query packet: the honest answer a change-impact or architecture-explorer
//! query returns, keeping *no impact found* distinct from *unknown*, *out of scope*, *policy
//! limited*, *provider unavailable*, and *stale graph*.
//!
//! Where [`crate::m5_workset_scope`] answers *what slice am I looking at?*,
//! [`crate::m5_topology_identity`] answers *which exact graph object is this?*, and
//! [`crate::m5_graph_governance`] freezes *which depth claim a lane may publish*, this packet
//! answers the question refactor planning, review explanation, topology cards, and support all
//! ask of an impact query: *is this empty answer safe, or does it merely mean the graph could not
//! see far enough?* It carries one [`ImpactQueryResult`] per query — each with the subject ids it
//! reasons about, the affected objects it includes, an out-of-scope count, a policy-hidden count,
//! an evidence-class summary, freshness, confidence, an [`ImpactResultClass`], an explicit
//! empty-result reason, and the [`RemediationAction`]s a narrowed answer offers — plus one
//! [`ImpactConsumerBinding`] per surface that carries the same answer beyond a single panel
//! render.
//!
//! Four invariants hold across the packet:
//!
//! - **No collapsed empty states.** Every non-[`ImpactResultClass::InScopeImpact`] result carries
//!   an explicit [`ImpactQueryResult::empty_reason`], and a [`ImpactResultClass::NoImpact`] result
//!   may not hide out-of-scope or policy-limited objects, so an empty answer never silently reads
//!   as *safe* when the graph is partial, stale, or scope-limited.
//! - **Counts before trust.** Every result exposes its included-result count alongside its
//!   out-of-scope and policy-hidden counts, so a user can see how many affected objects were
//!   shown versus withheld before approving or trusting a graph-backed action.
//! - **Explicit widen or refresh.** A result narrowed by scope, policy, freshness, or a missing
//!   provider must offer the matching [`RemediationAction`] — widen scope, refresh the index,
//!   connect the provider, or request policy access — rather than silently broadening.
//! - **Survives beyond one panel.** Every declared query is carried by the support-export
//!   binding, so support, evidence, and AI-context surfaces can reconstruct the impact answer and
//!   its scope or freshness class without scraping panel text.
//!
//! The packet reuses the stable topology identity space ([`TopologyNodeKind`]) and the stable
//! relation-fidelity vocabulary ([`RelationFidelity`]) rather than minting one-off impact strings
//! per surface, binds upstream to the canonical graph-depth governance matrix, the workset-scope
//! packet, and the topology-identity packet it extends, and stamps every consumer binding with
//! the active scope snapshot so replay can reconstruct the exact slice the user queried.
//!
//! The packet is checked in at `artifacts/graph/m5/m5-impact-query.json` and embedded here. It is
//! metadata-only: every field is a typed state, a count, a label, or an opaque ref, and it
//! carries no credential bodies, raw provider payloads, or graph node contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_topology_identity::{RelationFidelity, TopologyNodeKind, TopologyScopeAnchor};

/// Supported M5 impact-query packet schema version.
pub const M5_IMPACT_QUERY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_IMPACT_QUERY_RECORD_KIND: &str = "m5_impact_query_packet";

/// Repo-relative path to the checked-in packet.
pub const M5_IMPACT_QUERY_PATH: &str = "artifacts/graph/m5/m5-impact-query.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_IMPACT_QUERY_SCHEMA_REF: &str = "schemas/graph/m5-impact-query.schema.json";

/// Repo-relative path to the companion document.
pub const M5_IMPACT_QUERY_DOC_REF: &str = "docs/graph/m5/m5-impact-query.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_IMPACT_QUERY_FIXTURE_DIR: &str = "fixtures/graph/m5/m5-impact-query";

/// Repo-relative path to the canonical graph-depth governance matrix this packet extends.
pub const M5_IMPACT_QUERY_GOVERNANCE_MATRIX_REF: &str =
    "artifacts/graph/m5/m5-graph-governance.json";

/// Repo-relative path to the canonical workset-scope packet this packet is bound to.
pub const M5_IMPACT_QUERY_SCOPE_PACKET_REF: &str = "artifacts/graph/m5/m5-workset-scope.json";

/// Repo-relative path to the canonical topology-identity packet whose id space this packet reuses.
pub const M5_IMPACT_QUERY_TOPOLOGY_PACKET_REF: &str =
    "artifacts/graph/m5/m5-topology-identity.json";

/// Embedded checked-in packet JSON.
pub const M5_IMPACT_QUERY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/graph/m5/m5-impact-query.json"
));

/// How an impact-query result is classed, keeping *no impact* distinct from the several ways an
/// answer can be empty without being safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactResultClass {
    /// The query found affected objects within the active scope.
    InScopeImpact,
    /// The query resolved fully across the active scope and found no affected objects.
    NoImpact,
    /// Impact could not be determined; the graph has an unresolved gap for this subject.
    Unknown,
    /// Candidate impact resolves outside the active workset or hot-set slice.
    OutOfScope,
    /// Affected objects are suppressed by an access or visibility policy.
    PolicyLimited,
    /// A required provider connection is unavailable, so downstream impact cannot be computed.
    ProviderUnavailable,
    /// The backing graph index is stale or expired; an empty answer cannot be trusted.
    StaleGraph,
}

impl ImpactResultClass {
    /// Every impact-result class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::InScopeImpact,
        Self::NoImpact,
        Self::Unknown,
        Self::OutOfScope,
        Self::PolicyLimited,
        Self::ProviderUnavailable,
        Self::StaleGraph,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InScopeImpact => "in_scope_impact",
            Self::NoImpact => "no_impact",
            Self::Unknown => "unknown",
            Self::OutOfScope => "out_of_scope",
            Self::PolicyLimited => "policy_limited",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::StaleGraph => "stale_graph",
        }
    }

    /// Whether the result found affected objects within scope.
    pub const fn is_in_scope_impact(self) -> bool {
        matches!(self, Self::InScopeImpact)
    }

    /// Whether the result is the one empty state that genuinely means *no impact*.
    pub const fn implies_no_impact(self) -> bool {
        matches!(self, Self::NoImpact)
    }

    /// Whether the result is a definitive, fully-resolved answer (impact found, or none found).
    pub const fn is_definitive(self) -> bool {
        matches!(self, Self::InScopeImpact | Self::NoImpact)
    }

    /// Whether the class itself narrows the answer and therefore requires a remediation action.
    pub const fn requires_remediation(self) -> bool {
        matches!(
            self,
            Self::Unknown
                | Self::OutOfScope
                | Self::PolicyLimited
                | Self::ProviderUnavailable
                | Self::StaleGraph
        )
    }

    /// The remediation action this class demands, if any.
    pub const fn required_action(self) -> Option<RemediationAction> {
        match self {
            Self::Unknown => Some(RemediationAction::ResolveRelations),
            Self::OutOfScope => Some(RemediationAction::WidenScope),
            Self::PolicyLimited => Some(RemediationAction::RequestPolicyAccess),
            Self::ProviderUnavailable => Some(RemediationAction::ConnectProvider),
            Self::StaleGraph => Some(RemediationAction::RefreshIndex),
            Self::InScopeImpact | Self::NoImpact => None,
        }
    }
}

/// An explicit recovery path offered when an impact answer is narrowed instead of silently
/// broadened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemediationAction {
    /// Widen the active workset or hot set toward the full workspace.
    WidenScope,
    /// Refresh or reindex the graph to clear stale or expired state.
    RefreshIndex,
    /// Connect the missing provider so downstream impact can be computed.
    ConnectProvider,
    /// Request access to policy-suppressed objects.
    RequestPolicyAccess,
    /// Resolve approximate or unresolved relations behind an unknown answer.
    ResolveRelations,
    /// No remediation is needed; only valid for a definitive, fully-in-scope answer.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl RemediationAction {
    /// Every remediation action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WidenScope,
        Self::RefreshIndex,
        Self::ConnectProvider,
        Self::RequestPolicyAccess,
        Self::ResolveRelations,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WidenScope => "widen_scope",
            Self::RefreshIndex => "refresh_index",
            Self::ConnectProvider => "connect_provider",
            Self::RequestPolicyAccess => "request_policy_access",
            Self::ResolveRelations => "resolve_relations",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real recovery path the user can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// A surface that carries an impact answer beyond the panel that first rendered it.
///
/// The closed vocabulary is exhaustive: the originating panel plus every downstream surface the
/// answer must survive into, so an impact result is never trapped in one render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactConsumerSurface {
    /// The originating impact panel.
    ImpactPanel,
    /// AI refactor planning.
    RefactorPlanning,
    /// Review explanation.
    ReviewExplanation,
    /// A codebase topology card.
    TopologyCard,
    /// The support/export bundle.
    SupportExport,
}

impl ImpactConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ImpactPanel,
        Self::RefactorPlanning,
        Self::ReviewExplanation,
        Self::TopologyCard,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImpactPanel => "impact_panel",
            Self::RefactorPlanning => "refactor_planning",
            Self::ReviewExplanation => "review_explanation",
            Self::TopologyCard => "topology_card",
            Self::SupportExport => "support_export",
        }
    }

    /// Whether this is the originating panel rather than a downstream carrier.
    pub const fn is_origin_panel(self) -> bool {
        matches!(self, Self::ImpactPanel)
    }

    /// Whether this is the durable support-export surface that must carry every answer.
    pub const fn is_support_export(self) -> bool {
        matches!(self, Self::SupportExport)
    }
}

/// A per-evidence-class breakdown of the affected objects an answer includes.
///
/// The counts reuse the stable [`RelationFidelity`] vocabulary so every surface reads the same
/// evidence labels rather than minting impact-only strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceClassSummary {
    /// Included objects backed by exact, authoritative evidence.
    pub exact: usize,
    /// Included objects backed by approximate or inferred evidence.
    pub approximate: usize,
    /// Included objects hydrated from imported evidence.
    pub imported: usize,
    /// Included objects truncated at the active scope boundary.
    pub partial: usize,
    /// Included objects older than the current revision of an endpoint.
    pub stale: usize,
    /// Included objects whose relation is withheld by policy or a missing connection.
    pub blocked: usize,
}

impl EvidenceClassSummary {
    /// Total objects counted across every evidence class.
    pub const fn total(&self) -> usize {
        self.exact + self.approximate + self.imported + self.partial + self.stale + self.blocked
    }
}

/// One affected object an impact answer includes, carried in the shared topology identity space.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AffectedObject {
    /// Canonical, stable topology node id of the affected object.
    pub node_id: String,
    /// Node kind of the affected object.
    pub kind: TopologyNodeKind,
    /// Redaction-aware display label or path.
    pub display_label: String,
    /// How exactly the impact relation to this object is known.
    pub evidence_class: RelationFidelity,
    /// Explicit disclosure reason; required for every non-exact evidence class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_reason: Option<String>,
    /// Freshness token copied from the graph object.
    pub freshness: String,
    /// Confidence token copied from the graph object.
    pub confidence: String,
    /// Export-safe, copy-safe permalink that embeds the canonical node id.
    pub export_permalink: String,
}

impl AffectedObject {
    /// Whether a non-exact evidence class carries an explicit disclosure reason.
    pub fn evidence_is_labeled(&self) -> bool {
        if self.evidence_class.is_exact() {
            return true;
        }
        self.evidence_reason
            .as_ref()
            .is_some_and(|reason| !reason.trim().is_empty())
    }

    /// Whether the permalink is non-empty and embeds the canonical node id.
    pub fn permalink_is_export_safe(&self) -> bool {
        !self.export_permalink.trim().is_empty() && self.export_permalink.contains(&self.node_id)
    }
}

/// One impact-query result: the honest answer to a single change-impact or explorer query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImpactQueryResult {
    /// Stable query id inside the packet.
    pub query_id: String,
    /// Canonical topology node ids the query reasons about (the changed or selected objects).
    pub subject_ids: Vec<String>,
    /// Node kind of the subject.
    pub subject_kind: TopologyNodeKind,
    /// Reviewer-facing query label.
    pub query_label: String,
    /// How the result is classed.
    pub result_class: ImpactResultClass,
    /// Affected objects included (shown) within the active scope.
    #[serde(default)]
    pub included_objects: Vec<AffectedObject>,
    /// Count of included objects; must equal `included_objects.len()`.
    pub included_count: usize,
    /// Count of affected objects known to resolve outside the active slice, counted but not shown.
    pub out_of_scope_count: usize,
    /// Count of affected objects suppressed by policy, counted but not shown.
    pub hidden_count: usize,
    /// Overall freshness token for the answer.
    pub freshness: String,
    /// Overall confidence token for the answer.
    pub confidence: String,
    /// Per-evidence-class breakdown of the included objects.
    pub evidence_summary: EvidenceClassSummary,
    /// Explicit empty-result reason; required for every non-`in_scope_impact` result.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty_reason: Option<String>,
    /// Remediation actions the answer offers.
    #[serde(default)]
    pub remediation_actions: Vec<RemediationAction>,
    /// Export-safe, copy-safe permalink that embeds the canonical query id.
    pub export_permalink: String,
}

impl ImpactQueryResult {
    /// Whether the answer surfaces no included objects.
    pub fn is_empty(&self) -> bool {
        self.included_objects.is_empty()
    }

    /// Whether this result class must carry an explicit empty-result reason.
    pub fn requires_empty_reason(&self) -> bool {
        !self.result_class.is_in_scope_impact()
    }

    /// Whether the required empty-result reason is present and non-empty.
    pub fn empty_reason_is_labeled(&self) -> bool {
        if !self.requires_empty_reason() {
            return true;
        }
        self.empty_reason
            .as_ref()
            .is_some_and(|reason| !reason.trim().is_empty())
    }

    /// Whether the answer offers a given remediation action.
    pub fn offers(&self, action: RemediationAction) -> bool {
        self.remediation_actions.contains(&action)
    }

    /// Whether the answer is narrowed by class or by withheld counts and so needs remediation.
    pub fn requires_remediation(&self) -> bool {
        self.result_class.requires_remediation()
            || self.out_of_scope_count > 0
            || self.hidden_count > 0
    }

    /// Whether the permalink is non-empty and embeds the canonical query id.
    pub fn permalink_is_export_safe(&self) -> bool {
        !self.export_permalink.trim().is_empty() && self.export_permalink.contains(&self.query_id)
    }

    /// Recomputes the evidence-class breakdown from the included objects.
    pub fn computed_evidence_summary(&self) -> EvidenceClassSummary {
        let count = |fidelity: RelationFidelity| {
            self.included_objects
                .iter()
                .filter(|o| o.evidence_class == fidelity)
                .count()
        };
        EvidenceClassSummary {
            exact: count(RelationFidelity::Exact),
            approximate: count(RelationFidelity::Approximate),
            imported: count(RelationFidelity::Imported),
            partial: count(RelationFidelity::Partial),
            stale: count(RelationFidelity::Stale),
            blocked: count(RelationFidelity::Blocked),
        }
    }
}

/// One surface bound to the active scope snapshot, carrying a set of impact answers forward.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImpactConsumerBinding {
    /// Stable binding id inside the packet.
    pub binding_id: String,
    /// Surface this binding carries answers into.
    pub surface: ImpactConsumerSurface,
    /// Snapshot id this surface is bound to; must equal the active snapshot id.
    pub snapshot_id: String,
    /// Scope id this surface renders; must equal the active scope id.
    pub scope_id: String,
    /// Canonical query ids this surface carries; every id must be declared in the packet.
    #[serde(default)]
    pub carries_query_ids: Vec<String>,
    /// Ref to the surface artifact that ingests these answers.
    pub consumer_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ImpactQuerySummary {
    /// Total declared queries.
    pub query_count: usize,
    /// Total consumer bindings.
    pub consumer_binding_count: usize,
    /// Number of distinct surfaces bound.
    pub surface_count: usize,
    /// Queries classed `in_scope_impact`.
    pub in_scope_impact_count: usize,
    /// Queries classed `no_impact`.
    pub no_impact_count: usize,
    /// Queries classed `unknown`.
    pub unknown_count: usize,
    /// Queries classed `out_of_scope`.
    pub out_of_scope_query_count: usize,
    /// Queries classed `policy_limited`.
    pub policy_limited_count: usize,
    /// Queries classed `provider_unavailable`.
    pub provider_unavailable_count: usize,
    /// Queries classed `stale_graph`.
    pub stale_graph_count: usize,
    /// Total included objects across every query.
    pub total_included_objects: usize,
    /// Total out-of-scope objects counted across every query.
    pub total_out_of_scope_objects: usize,
    /// Total policy-hidden objects counted across every query.
    pub total_hidden_objects: usize,
    /// Queries that offer at least one real remediation action.
    pub queries_with_remediation: usize,
    /// Queries that carry an explicit empty-result reason.
    pub queries_with_empty_reason: usize,
}

/// A redaction-safe export row projected from one impact-query result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ImpactQueryExportRow {
    /// Canonical query id.
    pub query_id: String,
    /// Result-class token.
    pub result_class: String,
    /// Included-object count.
    pub included_count: usize,
    /// Out-of-scope object count.
    pub out_of_scope_count: usize,
    /// Policy-hidden object count.
    pub hidden_count: usize,
    /// Freshness token.
    pub freshness: String,
    /// Confidence token.
    pub confidence: String,
    /// Explicit empty-result reason, if any.
    pub empty_reason: Option<String>,
    /// Remediation-action tokens the answer offers.
    pub remediation_actions: Vec<String>,
    /// Export-safe permalink that points at the exact query.
    pub permalink: String,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet — the impact index downstream surfaces render
/// instead of re-describing impact answers by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ImpactQueryExportProjection {
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
    /// Projected query rows.
    pub queries: Vec<M5ImpactQueryExportRow>,
    /// Whether every non-in-scope result carries an explicit empty-result reason.
    pub all_empty_states_have_reason: bool,
    /// Whether every `no_impact` result hides no out-of-scope or policy-limited objects.
    pub no_impact_never_hides_objects: bool,
    /// Whether every narrowed result offers at least one real remediation action.
    pub all_narrowed_results_offer_remediation: bool,
    /// Whether every declared query is carried by the support-export binding.
    pub every_query_in_support_export: bool,
}

/// The typed M5 impact-query packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ImpactQueryPacket {
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
    /// Ref to the canonical workset-scope packet this packet is bound to.
    pub scope_packet_ref: String,
    /// Ref to the canonical topology-identity packet whose id space this packet reuses.
    pub topology_packet_ref: String,
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
    /// Closed result-class vocabulary.
    pub result_classes: Vec<ImpactResultClass>,
    /// Closed evidence-class vocabulary (the stable relation-fidelity space).
    pub evidence_classes: Vec<RelationFidelity>,
    /// Closed remediation-action vocabulary.
    pub remediation_actions: Vec<RemediationAction>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<ImpactConsumerSurface>,
    /// The active scope snapshot every binding is stamped with.
    pub active_scope: TopologyScopeAnchor,
    /// Declared impact-query results.
    #[serde(default)]
    pub queries: Vec<ImpactQueryResult>,
    /// Consumer bindings, one per surface.
    #[serde(default)]
    pub consumer_bindings: Vec<ImpactConsumerBinding>,
    /// Summary counts.
    pub summary: M5ImpactQuerySummary,
}

impl M5ImpactQueryPacket {
    /// Returns the query result for a query id.
    pub fn query(&self, query_id: &str) -> Option<&ImpactQueryResult> {
        self.queries.iter().find(|q| q.query_id == query_id)
    }

    /// Returns the binding for a surface.
    pub fn consumer_binding(
        &self,
        surface: ImpactConsumerSurface,
    ) -> Option<&ImpactConsumerBinding> {
        self.consumer_bindings.iter().find(|b| b.surface == surface)
    }

    /// Returns the export-safe permalink for a query id.
    pub fn permalink_for_query(&self, query_id: &str) -> Option<&str> {
        self.query(query_id).map(|q| q.export_permalink.as_str())
    }

    /// Whether every non-in-scope result carries an explicit empty-result reason.
    pub fn all_empty_states_have_reason(&self) -> bool {
        self.queries
            .iter()
            .all(ImpactQueryResult::empty_reason_is_labeled)
    }

    /// Whether every `no_impact` result hides no out-of-scope or policy-limited objects.
    ///
    /// This is the headline guardrail: an empty *no impact* answer may not stand in for objects
    /// that were merely out of scope or policy-hidden.
    pub fn no_impact_never_hides_objects(&self) -> bool {
        self.queries
            .iter()
            .filter(|q| q.result_class.implies_no_impact())
            .all(|q| {
                q.out_of_scope_count == 0 && q.hidden_count == 0 && q.included_objects.is_empty()
            })
    }

    /// Whether every narrowed result offers at least one real remediation action.
    pub fn all_narrowed_results_offer_remediation(&self) -> bool {
        self.queries.iter().all(|q| {
            !q.requires_remediation() || q.remediation_actions.iter().any(|a| a.is_offered())
        })
    }

    /// Whether every declared query is carried by the support-export binding.
    pub fn every_query_in_support_export(&self) -> bool {
        let Some(binding) = self.consumer_binding(ImpactConsumerSurface::SupportExport) else {
            return self.queries.is_empty();
        };
        let carried: BTreeSet<&str> = binding
            .carries_query_ids
            .iter()
            .map(String::as_str)
            .collect();
        self.queries
            .iter()
            .all(|q| carried.contains(q.query_id.as_str()))
    }

    /// Recomputes the summary block from the queries and bindings.
    pub fn computed_summary(&self) -> M5ImpactQuerySummary {
        let class_count = |class: ImpactResultClass| {
            self.queries
                .iter()
                .filter(|q| q.result_class == class)
                .count()
        };
        let distinct_surfaces: BTreeSet<ImpactConsumerSurface> =
            self.consumer_bindings.iter().map(|b| b.surface).collect();
        M5ImpactQuerySummary {
            query_count: self.queries.len(),
            consumer_binding_count: self.consumer_bindings.len(),
            surface_count: distinct_surfaces.len(),
            in_scope_impact_count: class_count(ImpactResultClass::InScopeImpact),
            no_impact_count: class_count(ImpactResultClass::NoImpact),
            unknown_count: class_count(ImpactResultClass::Unknown),
            out_of_scope_query_count: class_count(ImpactResultClass::OutOfScope),
            policy_limited_count: class_count(ImpactResultClass::PolicyLimited),
            provider_unavailable_count: class_count(ImpactResultClass::ProviderUnavailable),
            stale_graph_count: class_count(ImpactResultClass::StaleGraph),
            total_included_objects: self.queries.iter().map(|q| q.included_objects.len()).sum(),
            total_out_of_scope_objects: self.queries.iter().map(|q| q.out_of_scope_count).sum(),
            total_hidden_objects: self.queries.iter().map(|q| q.hidden_count).sum(),
            queries_with_remediation: self
                .queries
                .iter()
                .filter(|q| q.remediation_actions.iter().any(|a| a.is_offered()))
                .count(),
            queries_with_empty_reason: self
                .queries
                .iter()
                .filter(|q| {
                    q.empty_reason
                        .as_ref()
                        .is_some_and(|r| !r.trim().is_empty())
                })
                .count(),
        }
    }

    /// Produces the impact index downstream surfaces — release evidence, help/service-health,
    /// docs badges, refactor planning, review explanation, topology cards, and support exports —
    /// render instead of re-describing impact answers by hand.
    pub fn export_projection(&self) -> M5ImpactQueryExportProjection {
        let queries = self
            .queries
            .iter()
            .map(|q| M5ImpactQueryExportRow {
                query_id: q.query_id.clone(),
                result_class: q.result_class.as_str().to_owned(),
                included_count: q.included_objects.len(),
                out_of_scope_count: q.out_of_scope_count,
                hidden_count: q.hidden_count,
                freshness: q.freshness.clone(),
                confidence: q.confidence.clone(),
                empty_reason: q.empty_reason.clone(),
                remediation_actions: q
                    .remediation_actions
                    .iter()
                    .map(|a| a.as_str().to_owned())
                    .collect(),
                permalink: q.export_permalink.clone(),
                summary: format!(
                    "query {} ({}): {} included, {} out-of-scope, {} hidden, {}/{}",
                    q.query_id,
                    q.result_class.as_str(),
                    q.included_objects.len(),
                    q.out_of_scope_count,
                    q.hidden_count,
                    q.freshness,
                    q.confidence
                ),
            })
            .collect();
        M5ImpactQueryExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            snapshot_id: self.active_scope.snapshot_id.clone(),
            scope_id: self.active_scope.scope_id.clone(),
            scope_mode: self.active_scope.scope_mode.as_str().to_owned(),
            queries,
            all_empty_states_have_reason: self.all_empty_states_have_reason(),
            no_impact_never_hides_objects: self.no_impact_never_hides_objects(),
            all_narrowed_results_offer_remediation: self.all_narrowed_results_offer_remediation(),
            every_query_in_support_export: self.every_query_in_support_export(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5ImpactQueryViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_anchor(&mut violations);
        self.validate_queries(&mut violations);
        self.validate_bindings(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5ImpactQueryViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ImpactQueryViolation>) {
        if self.schema_version != M5_IMPACT_QUERY_SCHEMA_VERSION {
            violations.push(M5ImpactQueryViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_IMPACT_QUERY_RECORD_KIND {
            violations.push(M5ImpactQueryViolation::UnsupportedRecordKind {
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
            ("topology_packet_ref", &self.topology_packet_ref),
            ("conformance_ref", &self.conformance_ref),
            ("release_evidence_ref", &self.release_evidence_ref),
            ("help_surface_ref", &self.help_surface_ref),
            ("docs_badge_ref", &self.docs_badge_ref),
            ("support_export_ref", &self.support_export_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ImpactQueryViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        // The packet must bind upstream to the canonical governance matrix, workset-scope packet,
        // and topology-identity packet it extends, so the impact answer has one provenance root.
        if self.governance_matrix_ref != M5_IMPACT_QUERY_GOVERNANCE_MATRIX_REF {
            violations.push(M5ImpactQueryViolation::GovernanceMatrixRefMismatch);
        }
        if self.scope_packet_ref != M5_IMPACT_QUERY_SCOPE_PACKET_REF {
            violations.push(M5ImpactQueryViolation::ScopePacketRefMismatch);
        }
        if self.topology_packet_ref != M5_IMPACT_QUERY_TOPOLOGY_PACKET_REF {
            violations.push(M5ImpactQueryViolation::TopologyPacketRefMismatch);
        }
        for (field, ok) in [
            (
                "result_classes",
                self.result_classes == ImpactResultClass::ALL.to_vec(),
            ),
            (
                "evidence_classes",
                self.evidence_classes == RelationFidelity::ALL.to_vec(),
            ),
            (
                "remediation_actions",
                self.remediation_actions == RemediationAction::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == ImpactConsumerSurface::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5ImpactQueryViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_anchor(&self, violations: &mut Vec<M5ImpactQueryViolation>) {
        for (field, value) in [
            ("snapshot_id", &self.active_scope.snapshot_id),
            ("scope_id", &self.active_scope.scope_id),
            ("taken_as_of", &self.active_scope.taken_as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ImpactQueryViolation::EmptyField {
                    id: "<active_scope>".to_owned(),
                    field_name: field,
                });
            }
        }
    }

    fn validate_queries(&self, violations: &mut Vec<M5ImpactQueryViolation>) {
        let mut seen_ids = BTreeSet::new();
        for query in &self.queries {
            if !seen_ids.insert(query.query_id.clone()) {
                violations.push(M5ImpactQueryViolation::DuplicateQueryId {
                    query_id: query.query_id.clone(),
                });
            }
            for (field, value) in [
                ("query_id", &query.query_id),
                ("query_label", &query.query_label),
                ("freshness", &query.freshness),
                ("confidence", &query.confidence),
                ("export_permalink", &query.export_permalink),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ImpactQueryViolation::EmptyField {
                        id: query.query_id.clone(),
                        field_name: field,
                    });
                }
            }
            // A query must reason about at least one subject id, or it cannot be replayed.
            if query.subject_ids.is_empty() || query.subject_ids.iter().any(|s| s.trim().is_empty())
            {
                violations.push(M5ImpactQueryViolation::EmptySubjectIds {
                    query_id: query.query_id.clone(),
                });
            }
            // The included_count must agree with the body so a surface cannot claim a different
            // number than it shows.
            if query.included_count != query.included_objects.len() {
                violations.push(M5ImpactQueryViolation::IncludedCountMismatch {
                    query_id: query.query_id.clone(),
                });
            }
            self.validate_query_class(query, violations);
            self.validate_query_remediation(query, violations);
            self.validate_query_objects(query, violations);

            if query.evidence_summary != query.computed_evidence_summary() {
                violations.push(M5ImpactQueryViolation::EvidenceSummaryMismatch {
                    query_id: query.query_id.clone(),
                });
            }
            if !query.permalink_is_export_safe() {
                violations.push(M5ImpactQueryViolation::UnsafeQueryPermalink {
                    query_id: query.query_id.clone(),
                });
            }
        }
    }

    fn validate_query_class(
        &self,
        query: &ImpactQueryResult,
        violations: &mut Vec<M5ImpactQueryViolation>,
    ) {
        // Every empty state but in-scope impact must carry an explicit reason so it never collapses
        // into a misleading bare "no impact" message.
        if !query.empty_reason_is_labeled() {
            violations.push(M5ImpactQueryViolation::MissingEmptyReason {
                query_id: query.query_id.clone(),
                result_class: query.result_class.as_str(),
            });
        }
        match query.result_class {
            ImpactResultClass::InScopeImpact => {
                if query.included_objects.is_empty() {
                    violations.push(M5ImpactQueryViolation::InScopeImpactWithoutObjects {
                        query_id: query.query_id.clone(),
                    });
                }
            }
            // The guardrail: a no-impact answer may not stand in for out-of-scope or hidden objects.
            ImpactResultClass::NoImpact => {
                if query.out_of_scope_count > 0
                    || query.hidden_count > 0
                    || !query.included_objects.is_empty()
                {
                    violations.push(M5ImpactQueryViolation::NoImpactHidesObjects {
                        query_id: query.query_id.clone(),
                    });
                }
            }
            ImpactResultClass::OutOfScope => {
                if query.out_of_scope_count == 0 {
                    violations.push(M5ImpactQueryViolation::OutOfScopeWithoutCount {
                        query_id: query.query_id.clone(),
                    });
                }
            }
            ImpactResultClass::PolicyLimited => {
                if query.hidden_count == 0 {
                    violations.push(M5ImpactQueryViolation::PolicyLimitedWithoutHidden {
                        query_id: query.query_id.clone(),
                    });
                }
            }
            ImpactResultClass::Unknown
            | ImpactResultClass::ProviderUnavailable
            | ImpactResultClass::StaleGraph => {}
        }
    }

    fn validate_query_remediation(
        &self,
        query: &ImpactQueryResult,
        violations: &mut Vec<M5ImpactQueryViolation>,
    ) {
        if query.requires_remediation() {
            // A narrowed answer must offer a real recovery path rather than silently broadening.
            if !query.remediation_actions.iter().any(|a| a.is_offered()) {
                violations.push(M5ImpactQueryViolation::MissingRemediationAction {
                    query_id: query.query_id.clone(),
                });
            }
            // The class-specific action (widen, refresh, connect, request access, resolve) must
            // be the one offered, so the recovery path matches the reason the answer narrowed.
            if let Some(required) = query.result_class.required_action() {
                if !query.offers(required) {
                    violations.push(M5ImpactQueryViolation::MissingRequiredAction {
                        query_id: query.query_id.clone(),
                        action: required.as_str(),
                    });
                }
            }
            if query.out_of_scope_count > 0 && !query.offers(RemediationAction::WidenScope) {
                violations.push(M5ImpactQueryViolation::MissingRequiredAction {
                    query_id: query.query_id.clone(),
                    action: RemediationAction::WidenScope.as_str(),
                });
            }
            if query.hidden_count > 0 && !query.offers(RemediationAction::RequestPolicyAccess) {
                violations.push(M5ImpactQueryViolation::MissingRequiredAction {
                    query_id: query.query_id.clone(),
                    action: RemediationAction::RequestPolicyAccess.as_str(),
                });
            }
        } else {
            // A definitive, fully-in-scope answer must not imply a recovery path is needed.
            if query.remediation_actions != vec![RemediationAction::NoneNeeded] {
                violations.push(M5ImpactQueryViolation::UnexpectedRemediationAction {
                    query_id: query.query_id.clone(),
                });
            }
        }
    }

    fn validate_query_objects(
        &self,
        query: &ImpactQueryResult,
        violations: &mut Vec<M5ImpactQueryViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for object in &query.included_objects {
            for (field, value) in [
                ("node_id", &object.node_id),
                ("display_label", &object.display_label),
                ("freshness", &object.freshness),
                ("confidence", &object.confidence),
                ("export_permalink", &object.export_permalink),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ImpactQueryViolation::EmptyField {
                        id: format!("{}:{}", query.query_id, object.node_id),
                        field_name: field,
                    });
                }
            }
            if !seen.insert(object.node_id.clone()) {
                violations.push(M5ImpactQueryViolation::DuplicateAffectedObject {
                    query_id: query.query_id.clone(),
                    node_id: object.node_id.clone(),
                });
            }
            // A non-exact affected object must carry an explicit evidence reason so presentation
            // never implies stronger certainty than the graph carries.
            if !object.evidence_is_labeled() {
                violations.push(M5ImpactQueryViolation::UnlabeledAffectedEvidence {
                    query_id: query.query_id.clone(),
                    node_id: object.node_id.clone(),
                    evidence_class: object.evidence_class.as_str(),
                });
            }
            if !object.permalink_is_export_safe() {
                violations.push(M5ImpactQueryViolation::UnsafeAffectedPermalink {
                    query_id: query.query_id.clone(),
                    node_id: object.node_id.clone(),
                });
            }
        }
    }

    fn validate_bindings(&self, violations: &mut Vec<M5ImpactQueryViolation>) {
        let snapshot_id = &self.active_scope.snapshot_id;
        let scope_id = &self.active_scope.scope_id;
        let query_ids: BTreeSet<&str> = self.queries.iter().map(|q| q.query_id.as_str()).collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        for binding in &self.consumer_bindings {
            if !seen_ids.insert(binding.binding_id.clone()) {
                violations.push(M5ImpactQueryViolation::DuplicateBindingId {
                    binding_id: binding.binding_id.clone(),
                });
            }
            if !seen_surfaces.insert(binding.surface) {
                violations.push(M5ImpactQueryViolation::DuplicateSurfaceBinding {
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
                    violations.push(M5ImpactQueryViolation::EmptyField {
                        id: binding.binding_id.clone(),
                        field_name: field,
                    });
                }
            }
            // Every binding must be stamped with the active snapshot and scope so support export
            // and replay can reconstruct the slice the user queried.
            if &binding.snapshot_id != snapshot_id {
                violations.push(M5ImpactQueryViolation::SnapshotBindingMismatch {
                    binding_id: binding.binding_id.clone(),
                });
            }
            if &binding.scope_id != scope_id {
                violations.push(M5ImpactQueryViolation::ScopeIdMismatch {
                    binding_id: binding.binding_id.clone(),
                });
            }
            // A surface may only carry queries declared in the packet.
            for query_id in &binding.carries_query_ids {
                if !query_ids.contains(query_id.as_str()) {
                    violations.push(M5ImpactQueryViolation::UnresolvedQueryRef {
                        binding_id: binding.binding_id.clone(),
                        query_id: query_id.clone(),
                    });
                }
            }
        }

        // Every surface must carry a binding so no consumer leaves its handoff implicit.
        for surface in ImpactConsumerSurface::ALL {
            if !seen_surfaces.contains(&surface) {
                violations.push(M5ImpactQueryViolation::MissingSurfaceBinding {
                    surface: surface.as_str(),
                });
            }
        }

        // Guardrail: every declared query must be carried by the durable support-export surface,
        // so an answer survives beyond one panel render and support can reconstruct it.
        if let Some(binding) = self.consumer_binding(ImpactConsumerSurface::SupportExport) {
            let carried: BTreeSet<&str> = binding
                .carries_query_ids
                .iter()
                .map(String::as_str)
                .collect();
            for query in &self.queries {
                if !carried.contains(query.query_id.as_str()) {
                    violations.push(M5ImpactQueryViolation::QueryMissingFromSupportExport {
                        query_id: query.query_id.clone(),
                    });
                }
            }
        }
    }
}

/// A validation violation for the M5 impact-query packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ImpactQueryViolation {
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
    /// The packet does not bind to the canonical topology-identity packet.
    TopologyPacketRefMismatch,
    /// A query id appears more than once.
    DuplicateQueryId {
        /// Duplicate query id.
        query_id: String,
    },
    /// A query reasons about no subject id.
    EmptySubjectIds {
        /// Query id.
        query_id: String,
    },
    /// A query's included_count disagrees with its body.
    IncludedCountMismatch {
        /// Query id.
        query_id: String,
    },
    /// A non-in-scope result carries no explicit empty-result reason.
    MissingEmptyReason {
        /// Query id.
        query_id: String,
        /// Result-class token.
        result_class: &'static str,
    },
    /// A no-impact result hides out-of-scope or policy-limited objects.
    NoImpactHidesObjects {
        /// Query id.
        query_id: String,
    },
    /// An in-scope-impact result includes no affected objects.
    InScopeImpactWithoutObjects {
        /// Query id.
        query_id: String,
    },
    /// An out-of-scope result carries no out-of-scope count.
    OutOfScopeWithoutCount {
        /// Query id.
        query_id: String,
    },
    /// A policy-limited result carries no hidden count.
    PolicyLimitedWithoutHidden {
        /// Query id.
        query_id: String,
    },
    /// A narrowed result offers no real remediation action.
    MissingRemediationAction {
        /// Query id.
        query_id: String,
    },
    /// A narrowed result is missing the recovery path its reason demands.
    MissingRequiredAction {
        /// Query id.
        query_id: String,
        /// Required action token.
        action: &'static str,
    },
    /// A definitive, fully-in-scope result offers a recovery path it does not need.
    UnexpectedRemediationAction {
        /// Query id.
        query_id: String,
    },
    /// An affected object appears more than once in one query.
    DuplicateAffectedObject {
        /// Query id.
        query_id: String,
        /// Duplicate node id.
        node_id: String,
    },
    /// A non-exact affected object carries no explicit evidence reason.
    UnlabeledAffectedEvidence {
        /// Query id.
        query_id: String,
        /// Affected node id.
        node_id: String,
        /// Evidence-class token.
        evidence_class: &'static str,
    },
    /// A query carries a permalink that is empty or does not embed the query id.
    UnsafeQueryPermalink {
        /// Query id.
        query_id: String,
    },
    /// An affected object carries a permalink that is empty or does not embed the node id.
    UnsafeAffectedPermalink {
        /// Query id.
        query_id: String,
        /// Affected node id.
        node_id: String,
    },
    /// A query's evidence-class summary disagrees with its included objects.
    EvidenceSummaryMismatch {
        /// Query id.
        query_id: String,
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
    /// A binding carries a query id not declared in the packet.
    UnresolvedQueryRef {
        /// Binding id.
        binding_id: String,
        /// Unresolved query id.
        query_id: String,
    },
    /// A declared query is not carried by the support-export binding.
    QueryMissingFromSupportExport {
        /// Query id.
        query_id: String,
    },
    /// The summary counts disagree with the packet body.
    SummaryMismatch,
}

impl fmt::Display for M5ImpactQueryViolation {
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
            Self::TopologyPacketRefMismatch => write!(
                f,
                "packet topology_packet_ref must be the canonical topology-identity packet"
            ),
            Self::DuplicateQueryId { query_id } => write!(f, "duplicate query id {query_id}"),
            Self::EmptySubjectIds { query_id } => {
                write!(f, "query {query_id} reasons about no subject id")
            }
            Self::IncludedCountMismatch { query_id } => write!(
                f,
                "query {query_id} included_count disagrees with its included_objects"
            ),
            Self::MissingEmptyReason {
                query_id,
                result_class,
            } => write!(
                f,
                "query {query_id} is {result_class} but carries no explicit empty_reason"
            ),
            Self::NoImpactHidesObjects { query_id } => write!(
                f,
                "query {query_id} is no_impact but hides out-of-scope or policy-limited objects"
            ),
            Self::InScopeImpactWithoutObjects { query_id } => write!(
                f,
                "query {query_id} is in_scope_impact but includes no affected objects"
            ),
            Self::OutOfScopeWithoutCount { query_id } => write!(
                f,
                "query {query_id} is out_of_scope but carries no out_of_scope_count"
            ),
            Self::PolicyLimitedWithoutHidden { query_id } => write!(
                f,
                "query {query_id} is policy_limited but carries no hidden_count"
            ),
            Self::MissingRemediationAction { query_id } => write!(
                f,
                "query {query_id} is narrowed but offers no real remediation action"
            ),
            Self::MissingRequiredAction { query_id, action } => write!(
                f,
                "query {query_id} is narrowed but does not offer the required action {action}"
            ),
            Self::UnexpectedRemediationAction { query_id } => write!(
                f,
                "query {query_id} is definitive and in scope but offers a remediation action"
            ),
            Self::DuplicateAffectedObject { query_id, node_id } => write!(
                f,
                "query {query_id} includes affected object {node_id} more than once"
            ),
            Self::UnlabeledAffectedEvidence {
                query_id,
                node_id,
                evidence_class,
            } => write!(
                f,
                "query {query_id} object {node_id} is {evidence_class} but carries no evidence_reason"
            ),
            Self::UnsafeQueryPermalink { query_id } => write!(
                f,
                "query {query_id} has an empty permalink or one that does not embed its id"
            ),
            Self::UnsafeAffectedPermalink { query_id, node_id } => write!(
                f,
                "query {query_id} object {node_id} has an empty permalink or one that does not embed its id"
            ),
            Self::EvidenceSummaryMismatch { query_id } => write!(
                f,
                "query {query_id} evidence_summary disagrees with its included objects"
            ),
            Self::DuplicateBindingId { binding_id } => {
                write!(f, "duplicate binding id {binding_id}")
            }
            Self::DuplicateSurfaceBinding { surface } => {
                write!(f, "duplicate binding for surface {surface}")
            }
            Self::MissingSurfaceBinding { surface } => {
                write!(f, "missing binding for surface {surface}")
            }
            Self::SnapshotBindingMismatch { binding_id } => write!(
                f,
                "binding {binding_id} is not stamped with the active snapshot id"
            ),
            Self::ScopeIdMismatch { binding_id } => write!(
                f,
                "binding {binding_id} renders a scope other than the active scope"
            ),
            Self::UnresolvedQueryRef {
                binding_id,
                query_id,
            } => write!(
                f,
                "binding {binding_id} carries query {query_id} that is not declared in the packet"
            ),
            Self::QueryMissingFromSupportExport { query_id } => write!(
                f,
                "query {query_id} is not carried by the support-export binding"
            ),
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the packet body")
            }
        }
    }
}

impl Error for M5ImpactQueryViolation {}

/// Loads the embedded M5 impact-query packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5ImpactQueryPacket`].
pub fn current_m5_impact_query_packet() -> Result<M5ImpactQueryPacket, serde_json::Error> {
    serde_json::from_str(M5_IMPACT_QUERY_JSON)
}

#[cfg(test)]
mod tests;
