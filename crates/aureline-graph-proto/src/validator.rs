//! Identity / label rule enforcement for the workspace-graph seed.
//!
//! Enforces the eleven rules frozen in `§6 Identity and label rules`
//! of `docs/graph/workspace_graph_seed.md`. Each rule returns a
//! typed `ValidationError` rather than a string so downstream tests
//! can assert the exact violation.

use std::collections::{HashMap, HashSet};

use crate::hooks::HookCounters;
use crate::model::{
    ConfidenceRollup, EdgeEvidence, GraphEdge, GraphNode, NodeBody, WorkspaceGraph,
};
use crate::vocab::{
    ConfidenceLevel, EdgeClass, EdgeEvidenceState, Freshness, NodeClass, ProvenanceClass,
    StaleReason,
};

/// Typed identity / label rule violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Rule 1: node body shape must match node_class.
    NodeBodyClassMismatch {
        node_id: String,
        declared: NodeClass,
        body_expects: NodeClass,
    },
    /// Rule 2: node ids are unique.
    DuplicateNodeId { node_id: String },
    /// Rule 2: edge ids are unique.
    DuplicateEdgeId { edge_id: String },
    /// Rule 3: edges resolve to a node in the snapshot (or to a
    /// missing-anchor node).
    DanglingEdgeEndpoint {
        edge_id: String,
        endpoint: Endpoint,
        missing_node_id: String,
    },
    /// Rule 4: every node / edge carries ≥1 query-family,
    /// shard-affinity, and invalidation-producer tag.
    EmptyTagList { subject: Subject, which: TagList },
    /// Rule 4: every node / edge carries ≥1 scope ref.
    EmptyScopeRefs { subject: Subject },
    /// Rule 5: non-authoritative freshness frame MUST carry a
    /// stale_reason; authoritative MUST NOT.
    StaleReasonMismatch {
        subject: Subject,
        freshness: Freshness,
        stale_reason: Option<StaleReason>,
    },
    /// Rule 6: confidence rollup must obey the floor rule (any low
    /// or unknown contributor pulls the rollup to at least low).
    ConfidenceRollupFloorBroken {
        subject: Subject,
        declared_rollup: ConfidenceLevel,
        expected_rollup: ConfidenceLevel,
    },
    /// Rule 7: `imported_external` provenance MUST carry an
    /// imported_bundle_ref; `replayed_capture` MUST carry a
    /// replay_capture_ref.
    ProvenanceAnchorMissing {
        subject: Subject,
        provenance_class: ProvenanceClass,
        missing_field: &'static str,
    },
    /// Rule 8: `imported_evidence` on an edge requires `imported`
    /// freshness or `imported_from_external` stale_reason; vice
    /// versa.
    ImportedEvidenceFramingMismatch { edge_id: String },
    /// Rule 9: `missing_anchor` edge evidence requires that at
    /// least one endpoint is a `missing_anchor_node`.
    MissingAnchorEdgeWithoutMissingEndpoint { edge_id: String },
    /// Rule 10: topology edges (produces_artifact, depends_on,
    /// deployed_to, runs_in, hosted_by, consumes_artifact,
    /// mirrors_upstream) MUST carry a `topology_edge_slot`.
    TopologyEdgeWithoutSlot {
        edge_id: String,
        edge_class: EdgeClass,
    },
    /// Rule 11: `generated_artifact_node` bodies MUST carry a
    /// `lineage_record_ref` (enforced by the enum shape; duplicated
    /// here so the validator flags empty strings the schema also
    /// rejects via minLength).
    GeneratedArtifactLineageRefEmpty { node_id: String },
}

/// The role of a `GraphEdge` endpoint flagged by the validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endpoint {
    From,
    To,
}

/// Whether a rule violation names a node or an edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Subject {
    Node { node_id: String },
    Edge { edge_id: String },
}

/// Which tag list is empty when [`ValidationError::EmptyTagList`]
/// fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagList {
    QueryFamily,
    ShardAffinity,
    InvalidationProducer,
}

impl ValidationError {
    fn as_rule_id(&self) -> &'static str {
        match self {
            Self::NodeBodyClassMismatch { .. } => "rule1_node_body_class_match",
            Self::DuplicateNodeId { .. } | Self::DuplicateEdgeId { .. } => "rule2_unique_ids",
            Self::DanglingEdgeEndpoint { .. } => "rule3_edge_endpoints_resolve",
            Self::EmptyTagList { .. } | Self::EmptyScopeRefs { .. } => "rule4_required_tag_lists",
            Self::StaleReasonMismatch { .. } => "rule5_stale_reason_framing",
            Self::ConfidenceRollupFloorBroken { .. } => "rule6_confidence_rollup_floor",
            Self::ProvenanceAnchorMissing { .. } => "rule7_provenance_anchor_required",
            Self::ImportedEvidenceFramingMismatch { .. } => "rule8_imported_evidence_framing",
            Self::MissingAnchorEdgeWithoutMissingEndpoint { .. } => {
                "rule9_missing_anchor_edge_framing"
            }
            Self::TopologyEdgeWithoutSlot { .. } => "rule10_topology_edge_slot_required",
            Self::GeneratedArtifactLineageRefEmpty { .. } => {
                "rule11_generated_artifact_lineage_ref_required"
            }
        }
    }
}

/// Validate a workspace-graph and return every rule violation in the
/// order it was discovered. Also fires the relevant hook counters;
/// return value = (errors, hook counters).
pub fn validate_graph(graph: &WorkspaceGraph) -> (Vec<ValidationError>, HookCounters) {
    let mut errors: Vec<ValidationError> = Vec::new();
    let mut hooks = HookCounters {
        workspace_graph_snapshot_emitted: 1,
        ..HookCounters::default()
    };

    // Rule 2a: node-id uniqueness; also build the id → class index.
    let mut node_ids: HashSet<&str> = HashSet::new();
    let mut node_class_by_id: HashMap<&str, NodeClass> = HashMap::new();
    for node in &graph.nodes {
        if !node_ids.insert(node.node_id.as_str()) {
            errors.push(ValidationError::DuplicateNodeId {
                node_id: node.node_id.clone(),
            });
        }
        node_class_by_id.insert(node.node_id.as_str(), node.node_class);
    }

    for node in &graph.nodes {
        validate_node(node, &mut errors, &mut hooks);
    }

    // Rule 2b: edge-id uniqueness.
    let mut edge_ids: HashSet<&str> = HashSet::new();
    for edge in &graph.edges {
        if !edge_ids.insert(edge.edge_id.as_str()) {
            errors.push(ValidationError::DuplicateEdgeId {
                edge_id: edge.edge_id.clone(),
            });
        }
    }

    for edge in &graph.edges {
        validate_edge(edge, &node_class_by_id, &mut errors, &mut hooks);
    }

    (errors, hooks)
}

fn validate_node(node: &GraphNode, errors: &mut Vec<ValidationError>, hooks: &mut HookCounters) {
    hooks.graph_node_admitted += 1;

    // Rule 1: body shape matches class.
    let body_class = node.node_body.expected_node_class();
    if body_class != node.node_class {
        errors.push(ValidationError::NodeBodyClassMismatch {
            node_id: node.node_id.clone(),
            declared: node.node_class,
            body_expects: body_class,
        });
    }

    // Rule 11: generated_artifact_node lineage ref must be non-empty.
    if let NodeBody::GeneratedArtifact {
        lineage_record_ref, ..
    } = &node.node_body
    {
        if lineage_record_ref.is_empty() {
            errors.push(ValidationError::GeneratedArtifactLineageRefEmpty {
                node_id: node.node_id.clone(),
            });
        }
    }

    // Rule 4: non-empty tag lists + scope refs.
    if node.query_family_tags.is_empty() {
        errors.push(ValidationError::EmptyTagList {
            subject: Subject::Node {
                node_id: node.node_id.clone(),
            },
            which: TagList::QueryFamily,
        });
    }
    if node.shard_affinity_tags.is_empty() {
        errors.push(ValidationError::EmptyTagList {
            subject: Subject::Node {
                node_id: node.node_id.clone(),
            },
            which: TagList::ShardAffinity,
        });
    }
    if node.invalidation_producer_tags.is_empty() {
        errors.push(ValidationError::EmptyTagList {
            subject: Subject::Node {
                node_id: node.node_id.clone(),
            },
            which: TagList::InvalidationProducer,
        });
    }
    if node.scope_refs.is_empty() {
        errors.push(ValidationError::EmptyScopeRefs {
            subject: Subject::Node {
                node_id: node.node_id.clone(),
            },
        });
    }

    // Rule 5: stale-reason framing.
    check_stale_reason(
        Subject::Node {
            node_id: node.node_id.clone(),
        },
        node.freshness_frame.freshness,
        node.freshness_frame.stale_reason,
        errors,
    );

    // Rule 7: provenance anchors.
    check_provenance_anchors(
        Subject::Node {
            node_id: node.node_id.clone(),
        },
        &node.provenance_stamp.provenance_class,
        node.provenance_stamp.imported_bundle_ref.as_deref(),
        node.provenance_stamp.replay_capture_ref.as_deref(),
        errors,
    );

    // Rule 6: confidence rollup floor.
    if let Some(rollup) = &node.confidence_rollup {
        check_rollup_floor(
            Subject::Node {
                node_id: node.node_id.clone(),
            },
            rollup,
            errors,
        );
    }

    // Observability counters.
    if !matches!(node.freshness_frame.freshness, Freshness::Authoritative) {
        hooks.graph_freshness_downgraded += 1;
    }
    if matches!(
        node.confidence_level,
        ConfidenceLevel::Low | ConfidenceLevel::Unknown
    ) {
        hooks.graph_confidence_downgraded += 1;
    }
    match &node.node_body {
        NodeBody::MissingAnchor { .. } => {
            hooks.graph_missing_anchor_recorded += 1;
        }
        NodeBody::ImportedRoot { .. } => {
            hooks.graph_imported_attach += 1;
        }
        NodeBody::PolicyView { .. } => {
            hooks.graph_policy_view_projected += 1;
        }
        _ => {}
    }
    hooks.graph_impact_reason_attached += node.impact_reasons.len() as u64;
    hooks.graph_explainer_citation_attached += node.explainer_citations.len() as u64;
}

fn validate_edge(
    edge: &GraphEdge,
    node_class_by_id: &HashMap<&str, NodeClass>,
    errors: &mut Vec<ValidationError>,
    hooks: &mut HookCounters,
) {
    hooks.graph_edge_admitted += 1;

    // Rule 3: endpoint resolution.
    if !node_class_by_id.contains_key(edge.from_node_id.as_str()) {
        errors.push(ValidationError::DanglingEdgeEndpoint {
            edge_id: edge.edge_id.clone(),
            endpoint: Endpoint::From,
            missing_node_id: edge.from_node_id.clone(),
        });
    }
    if !node_class_by_id.contains_key(edge.to_node_id.as_str()) {
        errors.push(ValidationError::DanglingEdgeEndpoint {
            edge_id: edge.edge_id.clone(),
            endpoint: Endpoint::To,
            missing_node_id: edge.to_node_id.clone(),
        });
    }

    // Rule 4: non-empty tag lists + scope refs.
    if edge.query_family_tags.is_empty() {
        errors.push(ValidationError::EmptyTagList {
            subject: Subject::Edge {
                edge_id: edge.edge_id.clone(),
            },
            which: TagList::QueryFamily,
        });
    }
    if edge.shard_affinity_tags.is_empty() {
        errors.push(ValidationError::EmptyTagList {
            subject: Subject::Edge {
                edge_id: edge.edge_id.clone(),
            },
            which: TagList::ShardAffinity,
        });
    }
    if edge.invalidation_producer_tags.is_empty() {
        errors.push(ValidationError::EmptyTagList {
            subject: Subject::Edge {
                edge_id: edge.edge_id.clone(),
            },
            which: TagList::InvalidationProducer,
        });
    }
    if edge.scope_refs.is_empty() {
        errors.push(ValidationError::EmptyScopeRefs {
            subject: Subject::Edge {
                edge_id: edge.edge_id.clone(),
            },
        });
    }

    // Rule 5: stale-reason framing.
    check_stale_reason(
        Subject::Edge {
            edge_id: edge.edge_id.clone(),
        },
        edge.evidence.freshness_frame.freshness,
        edge.evidence.freshness_frame.stale_reason,
        errors,
    );

    // Rule 6: rollup floor.
    if let Some(rollup) = &edge.evidence.confidence_rollup {
        check_rollup_floor(
            Subject::Edge {
                edge_id: edge.edge_id.clone(),
            },
            rollup,
            errors,
        );
    }

    // Rule 7: provenance anchors.
    check_provenance_anchors(
        Subject::Edge {
            edge_id: edge.edge_id.clone(),
        },
        &edge.evidence.provenance_stamp.provenance_class,
        edge.evidence
            .provenance_stamp
            .imported_bundle_ref
            .as_deref(),
        edge.evidence.provenance_stamp.replay_capture_ref.as_deref(),
        errors,
    );

    // Rule 8: imported evidence must ride imported framing.
    check_imported_evidence_framing(edge, errors);

    // Rule 9: missing-anchor edge must touch a missing-anchor node.
    if matches!(
        edge.evidence.evidence_state,
        EdgeEvidenceState::MissingAnchor
    ) {
        let from_is_missing = matches!(
            node_class_by_id.get(edge.from_node_id.as_str()),
            Some(NodeClass::MissingAnchorNode)
        );
        let to_is_missing = matches!(
            node_class_by_id.get(edge.to_node_id.as_str()),
            Some(NodeClass::MissingAnchorNode)
        );
        if !from_is_missing && !to_is_missing {
            errors.push(ValidationError::MissingAnchorEdgeWithoutMissingEndpoint {
                edge_id: edge.edge_id.clone(),
            });
        }
    }

    // Rule 10: topology edges must carry a topology_edge_slot.
    if requires_topology_slot(edge.edge_class) && edge.body.topology_edge_slot.is_none() {
        errors.push(ValidationError::TopologyEdgeWithoutSlot {
            edge_id: edge.edge_id.clone(),
            edge_class: edge.edge_class,
        });
    }

    // Observability counters.
    if !matches!(
        edge.evidence.freshness_frame.freshness,
        Freshness::Authoritative
    ) {
        hooks.graph_freshness_downgraded += 1;
    }
    if matches!(
        edge.evidence.confidence_level,
        ConfidenceLevel::Low | ConfidenceLevel::Unknown
    ) {
        hooks.graph_confidence_downgraded += 1;
    }
    if edge.body.topology_edge_slot.is_some() {
        hooks.graph_topology_edge_admitted += 1;
    }
    if matches!(edge.edge_class, EdgeClass::ScopedBy) {
        match edge
            .scope_refs
            .iter()
            .find(|s| {
                matches!(
                    s.visibility,
                    crate::vocab::Visibility::PartialVisible
                        | crate::vocab::Visibility::PolicyHidden
                        | crate::vocab::Visibility::MissingInScope
                )
            })
            .is_some()
        {
            true => hooks.graph_workset_scope_narrowed += 1,
            false => hooks.graph_workset_scope_widened += 1,
        }
    }
    hooks.graph_impact_reason_attached += edge.body.impact_reasons.len() as u64;
    hooks.graph_explainer_citation_attached += edge.body.explainer_citations.len() as u64;
}

fn check_stale_reason(
    subject: Subject,
    freshness: Freshness,
    stale_reason: Option<StaleReason>,
    errors: &mut Vec<ValidationError>,
) {
    let needs_reason = !matches!(freshness, Freshness::Authoritative);
    let has_reason = stale_reason.is_some();
    if needs_reason != has_reason {
        errors.push(ValidationError::StaleReasonMismatch {
            subject,
            freshness,
            stale_reason,
        });
    }
}

fn check_provenance_anchors(
    subject: Subject,
    provenance_class: &ProvenanceClass,
    imported_bundle_ref: Option<&str>,
    replay_capture_ref: Option<&str>,
    errors: &mut Vec<ValidationError>,
) {
    match provenance_class {
        ProvenanceClass::ImportedExternal if imported_bundle_ref.is_none() => {
            errors.push(ValidationError::ProvenanceAnchorMissing {
                subject,
                provenance_class: *provenance_class,
                missing_field: "imported_bundle_ref",
            });
        }
        ProvenanceClass::ReplayedCapture if replay_capture_ref.is_none() => {
            errors.push(ValidationError::ProvenanceAnchorMissing {
                subject,
                provenance_class: *provenance_class,
                missing_field: "replay_capture_ref",
            });
        }
        _ => {}
    }
}

fn check_rollup_floor(
    subject: Subject,
    rollup: &ConfidenceRollup,
    errors: &mut Vec<ValidationError>,
) {
    if let Some(expected) = ConfidenceLevel::roll_up(&rollup.source_confidences) {
        if rollup.rolled_up_level != expected {
            errors.push(ValidationError::ConfidenceRollupFloorBroken {
                subject,
                declared_rollup: rollup.rolled_up_level,
                expected_rollup: expected,
            });
        }
    }
}

fn check_imported_evidence_framing(edge: &GraphEdge, errors: &mut Vec<ValidationError>) {
    let is_imported_evidence = matches!(
        edge.evidence.evidence_state,
        EdgeEvidenceState::ImportedEvidence
    );
    let has_imported_framing =
        matches!(edge.evidence.freshness_frame.freshness, Freshness::Imported)
            || matches!(
                edge.evidence.freshness_frame.stale_reason,
                Some(StaleReason::ImportedFromExternal)
            )
            || matches!(
                edge.evidence.provenance_stamp.provenance_class,
                ProvenanceClass::ImportedExternal
            );
    if is_imported_evidence != has_imported_framing {
        errors.push(ValidationError::ImportedEvidenceFramingMismatch {
            edge_id: edge.edge_id.clone(),
        });
    }
}

fn requires_topology_slot(class: EdgeClass) -> bool {
    matches!(
        class,
        EdgeClass::ProducesArtifact
            | EdgeClass::ConsumesArtifact
            | EdgeClass::DeployedTo
            | EdgeClass::RunsIn
            | EdgeClass::HostedBy
            | EdgeClass::DependsOn
            | EdgeClass::MirrorsUpstream
    )
}

/// Public helper for reporting. Lists the eleven rule ids the
/// validator enforces, in the order the doc names them.
pub fn rule_ids() -> [&'static str; 11] {
    [
        "rule1_node_body_class_match",
        "rule2_unique_ids",
        "rule3_edge_endpoints_resolve",
        "rule4_required_tag_lists",
        "rule5_stale_reason_framing",
        "rule6_confidence_rollup_floor",
        "rule7_provenance_anchor_required",
        "rule8_imported_evidence_framing",
        "rule9_missing_anchor_edge_framing",
        "rule10_topology_edge_slot_required",
        "rule11_generated_artifact_lineage_ref_required",
    ]
}

/// Render a rule id for a `ValidationError`. Used by the harness /
/// render lane so emitted JSON names the rule verbatim.
pub fn rule_id_for(error: &ValidationError) -> &'static str {
    error.as_rule_id()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenarios::all_scenarios;

    #[test]
    fn every_scenario_validates_clean() {
        for scenario in all_scenarios() {
            let (errors, _hooks) = validate_graph(&scenario.graph);
            assert!(
                errors.is_empty(),
                "scenario `{}` produced errors: {:?}",
                scenario.label,
                errors
            );
        }
    }

    #[test]
    fn rule_ids_are_unique() {
        let ids = rule_ids();
        let mut sorted: Vec<&str> = ids.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 11);
    }
}
