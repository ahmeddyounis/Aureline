//! Impact-explorer projections with one controlled reason vocabulary.
//!
//! The rows in [`ImpactExplorerSurface`] can be rendered in UI, CLI/headless
//! output, review packets, and support exports without translating reason
//! classes or scope labels.

use aureline_graph::{GraphStore, ImpactExplainerPacket, TopologyFallbackRow};
use aureline_graph_proto::{EdgeEvidenceState, ImpactReasonClass as GraphImpactReasonClass};
use serde::{Deserialize, Serialize};

use crate::topology::{ScopeVocabularyClass, SurfaceAction};

/// Schema version for impact explorer surfaces.
pub const IMPACT_EXPLORER_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for impact explorer surfaces.
pub const IMPACT_EXPLORER_SURFACE_RECORD_KIND: &str = "impact_explorer_surface_record";

/// Controlled impact reason vocabulary shared by UI, CLI, exports, and support packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactReasonClass {
    /// A stored exact graph edge directly supports the impact.
    ExactEdge,
    /// The impacted object shares a target, owner, or dependency with the changed object.
    SharedTarget,
    /// An ownership or routing rule predicts the impact.
    OwnershipRule,
    /// A generated-artifact or lineage link predicts the impact.
    GeneratedLinkage,
    /// Heuristic similarity predicts the impact.
    HeuristicSimilarity,
    /// A policy, trust, or scope coupling predicts the impact.
    PolicyCoupling,
}

impl ImpactReasonClass {
    /// Returns the stable token used in serialized surface packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactEdge => "exact_edge",
            Self::SharedTarget => "shared_target",
            Self::OwnershipRule => "ownership_rule",
            Self::GeneratedLinkage => "generated_linkage",
            Self::HeuristicSimilarity => "heuristic_similarity",
            Self::PolicyCoupling => "policy_coupling",
        }
    }

    /// Returns every reason class required across surfaces.
    pub const fn all() -> &'static [Self] {
        &[
            Self::ExactEdge,
            Self::SharedTarget,
            Self::OwnershipRule,
            Self::GeneratedLinkage,
            Self::HeuristicSimilarity,
            Self::PolicyCoupling,
        ]
    }
}

/// Exactness family shown beside an impact reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactConfidenceFamily {
    /// Exact graph evidence backs the reason.
    Exact,
    /// Derived graph evidence backs the reason.
    Derived,
    /// Heuristic or low-confidence evidence backs the reason.
    Heuristic,
}

impl ImpactConfidenceFamily {
    /// Returns the stable token used in serialized surface packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Derived => "derived",
            Self::Heuristic => "heuristic",
        }
    }
}

/// Loaded-scope note required on every impact row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadedScopeNote {
    /// Shared scope vocabulary class.
    pub scope_class: ScopeVocabularyClass,
    /// Active scope id.
    pub scope_id: String,
    /// Loaded scope state token.
    pub loaded_scope_state: String,
    /// Human-readable scope note.
    pub note: String,
}

/// Batch action class exposed by the impact explorer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchActionClass {
    /// Export all visible rows.
    ExportVisibleRows,
    /// Open a review packet for all visible rows.
    OpenReviewPacket,
    /// Request scope widening through review.
    RequestWidenScope,
    /// Open the parent topology map.
    OpenTopologyMap,
    /// Open the cited evidence packet.
    OpenEvidencePacket,
}

impl BatchActionClass {
    /// Returns the stable token used in serialized surface packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportVisibleRows => "export_visible_rows",
            Self::OpenReviewPacket => "open_review_packet",
            Self::RequestWidenScope => "request_widen_scope",
            Self::OpenTopologyMap => "open_topology_map",
            Self::OpenEvidencePacket => "open_evidence_packet",
        }
    }
}

/// One impact-explorer row with reason and evidence identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactExplorerRow {
    /// Stable row id.
    pub row_id: String,
    /// Canonical impacted graph node id.
    pub impacted_object_id: String,
    /// Canonical `impacts` graph edge id.
    pub impacts_edge_id: String,
    /// Controlled reason class.
    pub reason_class: ImpactReasonClass,
    /// Exact, derived, or heuristic family.
    pub reason_family: ImpactConfidenceFamily,
    /// Confidence token copied from the graph edge.
    pub graph_confidence: String,
    /// Loaded-scope note for this row.
    pub loaded_scope_note: LoadedScopeNote,
    /// Evidence ids the row can open.
    pub evidence_refs: Vec<String>,
    /// Actions available for the row.
    pub actions: Vec<SurfaceAction>,
    /// Redaction-aware explanation of why this item is impacted.
    pub reason_note: String,
}

/// Exportable impact-explorer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactExplorerSurface {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable surface id.
    pub surface_id: String,
    /// Shared scope vocabulary class.
    pub scope_class: ScopeVocabularyClass,
    /// Visible impact rows.
    pub rows: Vec<ImpactExplorerRow>,
    /// Count of impact edges outside the active scope.
    pub hidden_out_of_scope_count: usize,
    /// Batch action classes available to the surface.
    pub batch_actions: Vec<BatchActionClass>,
    /// Export actions that preserve evidence parity.
    pub export_actions: Vec<SurfaceAction>,
    /// Full controlled vocabulary exported for CLI/headless parity.
    pub supported_reason_vocabulary: Vec<ImpactReasonClass>,
}

impl ImpactExplorerSurface {
    /// Builds an impact explorer surface from a graph store and impact packet.
    pub fn from_packet(store: &GraphStore, packet: &ImpactExplainerPacket) -> Self {
        let scope_class =
            ScopeVocabularyClass::from_graph_scope_token(&packet.workset_scope.scope_class);
        let rows = packet
            .non_canvas_fallback
            .rows
            .iter()
            .map(|row| impact_row(store, packet, row, scope_class))
            .collect::<Vec<_>>();
        let export_actions = packet
            .evidence_card
            .open_detail_actions
            .iter()
            .filter(|action| action.action_class.as_str() == "export_evidence_card")
            .map(|action| SurfaceAction {
                action_id: action.action_id.clone(),
                action_class: action.action_class.as_str().to_owned(),
                subject_ref: action.subject_ref.clone(),
                preserves_scope: action.preserves_scope,
            })
            .collect::<Vec<_>>();
        let mut batch_actions = vec![
            BatchActionClass::ExportVisibleRows,
            BatchActionClass::OpenReviewPacket,
            BatchActionClass::OpenTopologyMap,
            BatchActionClass::OpenEvidencePacket,
        ];
        if packet.impact_summary.out_of_scope_count > 0 {
            batch_actions.push(BatchActionClass::RequestWidenScope);
        }

        Self {
            record_kind: IMPACT_EXPLORER_SURFACE_RECORD_KIND.to_owned(),
            schema_version: IMPACT_EXPLORER_SURFACE_SCHEMA_VERSION,
            surface_id: format!("surface:impact_explorer:{}", packet.query_request_id),
            scope_class,
            rows,
            hidden_out_of_scope_count: packet.impact_summary.out_of_scope_count,
            batch_actions,
            export_actions,
            supported_reason_vocabulary: ImpactReasonClass::all().to_vec(),
        }
    }
}

fn impact_row(
    store: &GraphStore,
    packet: &ImpactExplainerPacket,
    row: &TopologyFallbackRow,
    scope_class: ScopeVocabularyClass,
) -> ImpactExplorerRow {
    let edge = store.edge(&row.edge_id);
    let reason_class = edge.map_or(
        ImpactReasonClass::HeuristicSimilarity,
        reason_class_for_edge,
    );
    let reason_family = reason_family(reason_class, row.evidence_state.as_str());
    let graph_note = edge
        .and_then(|edge| edge.body.impact_reasons.first())
        .and_then(|reason| reason.note.clone())
        .unwrap_or_else(|| fallback_reason_note(reason_class, row));
    ImpactExplorerRow {
        row_id: format!("row:impact:{}:{}", packet.query_request_id, row.edge_id),
        impacted_object_id: row.to_node_id.clone(),
        impacts_edge_id: row.edge_id.clone(),
        reason_class,
        reason_family,
        graph_confidence: row.confidence.clone(),
        loaded_scope_note: LoadedScopeNote {
            scope_class,
            scope_id: packet.workset_scope.scope_id.clone(),
            loaded_scope_state: packet.workset_scope.index_coverage.coverage_state.clone(),
            note: packet.coverage_claim.clone(),
        },
        evidence_refs: vec![
            row.edge_id.clone(),
            row.from_node_id.clone(),
            row.to_node_id.clone(),
        ],
        actions: vec![
            SurfaceAction {
                action_id: format!("action:{}:open_evidence", row.edge_id),
                action_class: "inspect_graph_relation".to_owned(),
                subject_ref: row.edge_id.clone(),
                preserves_scope: true,
            },
            SurfaceAction {
                action_id: format!("action:{}:open_impacted_object", row.to_node_id),
                action_class: "open_source_at_anchor".to_owned(),
                subject_ref: row.to_node_id.clone(),
                preserves_scope: true,
            },
        ],
        reason_note: graph_note,
    }
}

fn reason_class_for_edge(edge: &aureline_graph::GraphEdge) -> ImpactReasonClass {
    if let Some(reason) = edge.body.impact_reasons.first() {
        return match reason.reason_class {
            GraphImpactReasonClass::ExactEdge => ImpactReasonClass::ExactEdge,
            GraphImpactReasonClass::SharedTarget => ImpactReasonClass::SharedTarget,
            GraphImpactReasonClass::OwnershipRule => ImpactReasonClass::OwnershipRule,
            GraphImpactReasonClass::GeneratedLinkage => ImpactReasonClass::GeneratedLinkage,
            GraphImpactReasonClass::HeuristicSimilarity => ImpactReasonClass::HeuristicSimilarity,
            GraphImpactReasonClass::PolicyCoupling => ImpactReasonClass::PolicyCoupling,
            GraphImpactReasonClass::OwnershipChange => ImpactReasonClass::OwnershipRule,
            GraphImpactReasonClass::GeneratedArtifactRegeneration => {
                ImpactReasonClass::GeneratedLinkage
            }
            GraphImpactReasonClass::PolicyChange
            | GraphImpactReasonClass::WorksetScopeNarrowed
            | GraphImpactReasonClass::WorksetScopeWidened => ImpactReasonClass::PolicyCoupling,
            GraphImpactReasonClass::InferredTransitiveImpact => {
                ImpactReasonClass::HeuristicSimilarity
            }
            GraphImpactReasonClass::ImportedBundleRollover => ImpactReasonClass::SharedTarget,
            GraphImpactReasonClass::DependencyBump
            | GraphImpactReasonClass::ProviderResourceUpdate => ImpactReasonClass::SharedTarget,
            GraphImpactReasonClass::DirectEdit
            | GraphImpactReasonClass::SymbolRename
            | GraphImpactReasonClass::SignatureChange => ImpactReasonClass::ExactEdge,
        };
    }

    match edge.evidence.evidence_state {
        EdgeEvidenceState::DirectEvidence => ImpactReasonClass::ExactEdge,
        EdgeEvidenceState::ImportedEvidence => ImpactReasonClass::SharedTarget,
        EdgeEvidenceState::InferredRelation
        | EdgeEvidenceState::StaleRelation
        | EdgeEvidenceState::MissingAnchor => ImpactReasonClass::HeuristicSimilarity,
    }
}

fn reason_family(reason_class: ImpactReasonClass, evidence_state: &str) -> ImpactConfidenceFamily {
    if reason_class == ImpactReasonClass::HeuristicSimilarity
        || matches!(
            evidence_state,
            "inferred_relation" | "stale_relation" | "missing_anchor"
        )
    {
        return ImpactConfidenceFamily::Heuristic;
    }
    if reason_class == ImpactReasonClass::ExactEdge && evidence_state == "direct_evidence" {
        ImpactConfidenceFamily::Exact
    } else {
        ImpactConfidenceFamily::Derived
    }
}

fn fallback_reason_note(reason_class: ImpactReasonClass, row: &TopologyFallbackRow) -> String {
    format!(
        "{} predicts impact from {} to {} via edge {}.",
        reason_class.as_str(),
        row.from_node_id,
        row.to_node_id,
        row.edge_id
    )
}
