//! Concrete infrastructure object packet and shared consumer projections.
//!
//! The matrix in the parent module freezes the family, truth-layer, and
//! relation-edge vocabulary. This packet instantiates that vocabulary as
//! stable objects and relations that graph, review, docs, and incident
//! surfaces can resolve without bespoke parsers or side caches.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::target_context_and_control_plane_boundary::{
    ActionPosture, EnvironmentCompleteness, EnvironmentContext, FreshnessLabel,
    InfraBoundaryFinding, InfraBoundaryFindingSeverity,
};

use super::{
    flows::{projection_covers_context_slice, projection_covers_relation_flow},
    required_edges_for, ConsoleHandoffPosture, InfrastructureFamily, RelationEdgeClass, TruthLayer,
    REQUIRED_FAMILIES, REQUIRED_TRUTH_LAYERS, SOURCE_INTELLIGENCE_RELATIONSHIP_PACKET_RECORD_KIND,
};

/// Schema version for concrete infrastructure object packets.
pub const SOURCE_INTELLIGENCE_OBJECT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind discriminator for [`SourceIntelligenceObjectPacket`].
pub const SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND: &str =
    "infra_source_intelligence_object_packet";

/// JSON Schema reference for object-packet interchange.
pub const SOURCE_INTELLIGENCE_OBJECT_SCHEMA_REF: &str =
    "schemas/infra/source-intelligence-object-packet.schema.json";

/// Fixture corpus directory for object-packet qualification and downgrade drills.
pub const SOURCE_INTELLIGENCE_OBJECT_FIXTURE_DIR: &str =
    "fixtures/infra/source-intelligence-and-resource-relationships";

/// Stable object identity extracted from one infrastructure family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureObjectIdentity {
    /// Identity family, such as `terraform_resource_address` or `kubernetes_gvk`.
    pub identity_class: String,
    /// Stable locator preserved across formatting-only changes.
    pub stable_locator: String,
    /// Namespace, workspace, account, or other scope label when applicable.
    pub namespace_or_scope: Option<String>,
    /// Selector or addressing tokens derived from the object.
    pub selector_refs: Vec<String>,
    /// Owner or controller refs derived from the object.
    pub owner_refs: Vec<String>,
    /// Known source-path ref when the object can be traced to a file.
    pub source_path_ref: Option<String>,
    /// Provider or control-plane handle when the object has one.
    pub provider_handle_ref: Option<String>,
}

/// Provenance and lineage preserved for one infrastructure object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureObjectLineage {
    /// Authored object ids that lead back to repo-owned source.
    pub authored_object_refs: Vec<String>,
    /// Input refs used to derive a rendered or planned object.
    pub source_input_refs: Vec<String>,
    /// Known path-back-to-source refs preserved for explainers and support.
    pub known_path_back_to_source_refs: Vec<String>,
    /// Tool or renderer identity when the object was derived.
    pub tool_identity: Option<String>,
    /// Tool or renderer version when the object was derived.
    pub tool_version: Option<String>,
    /// Parameter, environment, or selector assumptions used during derivation.
    pub parameter_or_environment_refs: Vec<String>,
}

/// One authored, rendered, planned, observed, or provider-overlay object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureObjectRecord {
    /// Stable object id.
    pub object_id: String,
    /// Infrastructure family the object belongs to.
    pub family: InfrastructureFamily,
    /// Truth layer the object represents.
    pub truth_layer: TruthLayer,
    /// Redaction-safe display label.
    pub display_label: String,
    /// Shared target-context ref backing the object.
    pub context_ref: String,
    /// Stable identity extracted from the object.
    pub identity: InfrastructureObjectIdentity,
    /// Freshness posture for the object.
    pub freshness: FreshnessLabel,
    /// Observation timestamp when the object was rendered, planned, observed, or refreshed.
    pub observed_at: Option<String>,
    /// Authority posture carried by the object.
    pub authority_posture: ActionPosture,
    /// True when the object is provider-owned rather than repo-owned or observed locally.
    pub provider_owned: bool,
    /// Lineage and derivation metadata for the object.
    pub lineage: InfrastructureObjectLineage,
    /// Safe provenance refs used to derive or observe the object.
    pub provenance_refs: Vec<String>,
    /// Export-safe object summary.
    pub support_summary: String,
}

/// One stable relation edge between infrastructure objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureObjectRelationRecord {
    /// Stable relation id.
    pub relation_id: String,
    /// Infrastructure family the relation belongs to.
    pub family: InfrastructureFamily,
    /// Stable edge class.
    pub edge_class: RelationEdgeClass,
    /// Source object ref.
    pub from_object_ref: String,
    /// Target object ref.
    pub to_object_ref: String,
    /// Freshness posture for the relation.
    pub freshness: FreshnessLabel,
    /// Authority posture carried by the relation.
    pub authority_posture: ActionPosture,
    /// Console-handoff posture the relation preserves.
    pub console_handoff_posture: ConsoleHandoffPosture,
    /// Selector refs extracted while deriving the relation.
    pub selector_refs: Vec<String>,
    /// Owner or controller refs extracted while deriving the relation.
    pub owner_refs: Vec<String>,
    /// Safe provenance refs used to derive the relation.
    pub provenance_refs: Vec<String>,
    /// Export-safe relation summary.
    pub support_summary: String,
}

/// Consumer surface that resolves infrastructure objects from the shared packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureConsumerSurface {
    /// Semantic graph and topology consumers.
    Graph,
    /// Review and approval consumers.
    Review,
    /// Docs, explainers, and help consumers.
    Docs,
    /// Incident and runbook consumers.
    Incident,
}

/// Projection proving a consumer resolves objects from the shared packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureConsumerProjection {
    /// Stable projection id.
    pub projection_id: String,
    /// Consumer surface covered by the projection.
    pub surface: InfrastructureConsumerSurface,
    /// Shared packet id the consumer reads from.
    pub source_packet_ref: String,
    /// Object refs resolved by the consumer.
    pub object_refs: Vec<String>,
    /// Relation refs resolved by the consumer.
    pub relation_refs: Vec<String>,
    /// True when the surface resolves the shared packet directly.
    pub uses_shared_packet: bool,
    /// True when the consumer created a hidden side cache or private truth store.
    pub hidden_side_cache_created: bool,
    /// Export-safe projection summary.
    pub support_summary: String,
}

/// Canonical object and relation packet for infrastructure source intelligence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceIntelligenceObjectPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Matrix packet ref that froze the vocabulary used here.
    pub matrix_packet_ref: String,
    /// Shared target contexts used by the objects.
    pub environment_contexts: Vec<EnvironmentContext>,
    /// Concrete infrastructure objects grouped by truth layer.
    pub object_records: Vec<InfrastructureObjectRecord>,
    /// Stable relation edges between the objects.
    pub relation_records: Vec<InfrastructureObjectRelationRecord>,
    /// Consumer projections that reuse the same packet.
    pub consumer_projections: Vec<InfrastructureConsumerProjection>,
    /// Export-safe support summary.
    pub support_summary: String,
}

impl SourceIntelligenceObjectPacket {
    /// Validates the packet against canonical object and consumer invariants.
    pub fn validate(&self) -> SourceIntelligenceObjectPacketValidationReport {
        validate_object_packet(self)
    }

    /// Resolves one object by stable object id.
    pub fn object(&self, object_id: &str) -> Option<&InfrastructureObjectRecord> {
        self.object_records
            .iter()
            .find(|object| object.object_id == object_id)
    }

    /// Resolves one relation by stable relation id.
    pub fn relation(&self, relation_id: &str) -> Option<&InfrastructureObjectRelationRecord> {
        self.relation_records
            .iter()
            .find(|relation| relation.relation_id == relation_id)
    }

    /// Resolves one consumer projection by surface.
    pub fn consumer_projection(
        &self,
        surface: InfrastructureConsumerSurface,
    ) -> Option<&InfrastructureConsumerProjection> {
        self.consumer_projections
            .iter()
            .find(|projection| projection.surface == surface)
    }
}

/// Validation report emitted for a concrete object packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceIntelligenceObjectPacketValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Packet id validated.
    pub packet_id: String,
    /// Whether no error-severity checks failed.
    pub passed: bool,
    /// Families covered by the packet.
    pub families: BTreeSet<InfrastructureFamily>,
    /// Truth layers covered by the packet.
    pub truth_layers: BTreeSet<TruthLayer>,
    /// Relation-edge classes covered by the packet.
    pub relation_edges: BTreeSet<RelationEdgeClass>,
    /// Consumer surfaces covered by the packet.
    pub consumer_surfaces: BTreeSet<InfrastructureConsumerSurface>,
    /// Validation findings emitted by the packet.
    pub findings: Vec<InfraBoundaryFinding>,
}

/// Validates one concrete source-intelligence object packet.
pub fn validate_object_packet(
    packet: &SourceIntelligenceObjectPacket,
) -> SourceIntelligenceObjectPacketValidationReport {
    let mut findings = Vec::new();
    let mut families = BTreeSet::new();
    let mut truth_layers = BTreeSet::new();
    let mut relation_edges = BTreeSet::new();
    let mut consumer_surfaces = BTreeSet::new();
    let mut context_ids = BTreeSet::new();
    let mut object_ids = BTreeSet::new();
    let mut relation_ids = BTreeSet::new();
    let mut projection_ids = BTreeSet::new();

    if packet.record_kind != SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND {
        findings.push(error(
            "record_kind",
            "Packet record_kind is not the infrastructure object-packet discriminator.",
        ));
    }
    if packet.schema_version != SOURCE_INTELLIGENCE_OBJECT_SCHEMA_VERSION {
        findings.push(error(
            "schema_version",
            "Packet schema_version does not match this crate.",
        ));
    }
    if packet.matrix_packet_ref != SOURCE_INTELLIGENCE_RELATIONSHIP_PACKET_RECORD_KIND {
        findings.push(error(
            "matrix_packet_ref",
            "Object packet must cite the canonical matrix packet record kind.",
        ));
    }
    if packet.support_summary.trim().is_empty() {
        findings.push(error(
            "support_summary",
            "Object packet is missing a support summary.",
        ));
    }

    for context in &packet.environment_contexts {
        if !context_ids.insert(context.context_id.as_str()) {
            findings.push(error(
                "duplicate_context",
                "Object packet repeats the same environment context.",
            ));
        }
        if !context.ambient_context_prohibited {
            findings.push(error(
                "ambient_context",
                "Object packet environment context allows ambient inheritance.",
            ));
        }
        if context.completeness == EnvironmentCompleteness::Incomplete {
            findings.push(error(
                "context_completeness",
                "Object packet environment context is incomplete.",
            ));
        }
    }

    for object in &packet.object_records {
        families.insert(object.family);
        truth_layers.insert(object.truth_layer);

        if !object_ids.insert(object.object_id.as_str()) {
            findings.push(error(
                "duplicate_object",
                "Object packet repeats the same object id.",
            ));
        }
        if !context_ids.contains(object.context_ref.as_str()) {
            findings.push(error(
                "missing_context_ref",
                "Object record references an unknown environment context.",
            ));
        }
        if object.display_label.trim().is_empty() || object.support_summary.trim().is_empty() {
            findings.push(error(
                "object_text",
                "Object record is missing a display label or support summary.",
            ));
        }
        if object.identity.identity_class.trim().is_empty()
            || object.identity.stable_locator.trim().is_empty()
        {
            findings.push(error(
                "object_identity",
                "Object record is missing its stable identity class or locator.",
            ));
        }
        if object.identity.selector_refs.is_empty() {
            findings.push(error(
                "object_selectors",
                "Object record is missing selector refs.",
            ));
        }
        if matches!(
            object.authority_posture,
            ActionPosture::StepUpRequired
                | ActionPosture::WriteApproved
                | ActionPosture::Blocked
                | ActionPosture::NotClaimed
        ) {
            findings.push(error(
                "object_authority_posture",
                "Object packet may not imply live mutation authority.",
            ));
        }
        if matches!(object.truth_layer, TruthLayer::ProviderOverlay) {
            if !object.provider_owned {
                findings.push(error(
                    "provider_overlay_provider_owned",
                    "Provider-overlay objects must be marked provider-owned.",
                ));
            }
            if object.authority_posture != ActionPosture::HandoffOnly {
                findings.push(error(
                    "provider_overlay_authority",
                    "Provider-overlay objects must remain handoff-only.",
                ));
            }
        } else if object.provider_owned {
            findings.push(error(
                "non_overlay_provider_owned",
                "Only provider-overlay objects may be marked provider-owned.",
            ));
        }
        if object.truth_layer != TruthLayer::AuthoredDesired && object.observed_at.is_none() {
            findings.push(error(
                "observed_at",
                "Derived, planned, live, and overlay objects must carry an observation timestamp.",
            ));
        }
        if matches!(
            object.truth_layer,
            TruthLayer::RenderedExpanded | TruthLayer::PlannedValidated
        ) {
            if object.lineage.authored_object_refs.is_empty()
                || object.lineage.known_path_back_to_source_refs.is_empty()
                || object.lineage.tool_identity.is_none()
                || object.lineage.tool_version.is_none()
            {
                findings.push(error(
                    "derived_lineage",
                    "Rendered and planned objects must preserve authored lineage and tool identity/version.",
                ));
            }
        }
        if matches!(
            object.truth_layer,
            TruthLayer::ObservedLive | TruthLayer::ProviderOverlay
        ) && object.lineage.authored_object_refs.is_empty()
        {
            findings.push(error(
                "live_lineage",
                "Observed and provider-overlay objects must preserve a path back to authored source.",
            ));
        }
    }

    for required in REQUIRED_FAMILIES {
        if !families.contains(&required) {
            findings.push(error(
                "family_coverage",
                "Object packet is missing a required infrastructure family.",
            ));
        }
    }
    for required in REQUIRED_TRUTH_LAYERS {
        if !truth_layers.contains(&required) {
            findings.push(error(
                "truth_layer_coverage",
                "Object packet is missing a required truth layer.",
            ));
        }
    }

    for family in REQUIRED_FAMILIES {
        let family_truth_layers = packet
            .object_records
            .iter()
            .filter(|object| object.family == family)
            .map(|object| object.truth_layer)
            .collect::<BTreeSet<_>>();
        for required in REQUIRED_TRUTH_LAYERS {
            if !family_truth_layers.contains(&required) {
                findings.push(error(
                    "family_truth_layer_coverage",
                    "Infrastructure family is missing one of the required truth layers.",
                ));
            }
        }
    }

    for relation in &packet.relation_records {
        relation_edges.insert(relation.edge_class);

        if !relation_ids.insert(relation.relation_id.as_str()) {
            findings.push(error(
                "duplicate_relation",
                "Object packet repeats the same relation id.",
            ));
        }
        if relation.support_summary.trim().is_empty() {
            findings.push(error(
                "relation_summary",
                "Relation record is missing a support summary.",
            ));
        }
        if matches!(
            relation.authority_posture,
            ActionPosture::StepUpRequired
                | ActionPosture::WriteApproved
                | ActionPosture::Blocked
                | ActionPosture::NotClaimed
        ) {
            findings.push(error(
                "relation_authority_posture",
                "Relation packet may not imply live mutation authority.",
            ));
        }
        let Some(from_object) = packet.object(&relation.from_object_ref) else {
            findings.push(error(
                "relation_from_ref",
                "Relation record references an unknown source object.",
            ));
            continue;
        };
        let Some(to_object) = packet.object(&relation.to_object_ref) else {
            findings.push(error(
                "relation_to_ref",
                "Relation record references an unknown target object.",
            ));
            continue;
        };
        if from_object.family != relation.family || to_object.family != relation.family {
            findings.push(error(
                "relation_family_mismatch",
                "Relation record must stay within a single infrastructure family.",
            ));
        }
        if !relation_layers_match(
            relation.edge_class,
            from_object.truth_layer,
            to_object.truth_layer,
        ) {
            findings.push(error(
                "relation_truth_layers",
                "Relation record binds an edge class to invalid truth layers.",
            ));
        }
        if edge_requires_handoff(relation.edge_class)
            && relation.console_handoff_posture != ConsoleHandoffPosture::OverlayOnlyBoundary
        {
            findings.push(error(
                "relation_handoff_posture",
                "Overlay and handoff edges must preserve explicit overlay-only posture.",
            ));
        }
        if !edge_requires_handoff(relation.edge_class)
            && relation.console_handoff_posture == ConsoleHandoffPosture::OverlayOnlyBoundary
        {
            findings.push(error(
                "relation_unexpected_overlay_handoff",
                "Non-overlay edges may not be marked overlay-only.",
            ));
        }
    }

    for family in REQUIRED_FAMILIES {
        let family_edges = packet
            .relation_records
            .iter()
            .filter(|relation| relation.family == family)
            .map(|relation| relation.edge_class)
            .collect::<BTreeSet<_>>();
        for required in required_edges_for(family) {
            if !family_edges.contains(&required) {
                findings.push(error(
                    "family_relation_edge_coverage",
                    "Infrastructure family is missing a required relation-edge class.",
                ));
            }
        }
    }

    for projection in &packet.consumer_projections {
        consumer_surfaces.insert(projection.surface);

        if !projection_ids.insert(projection.projection_id.as_str()) {
            findings.push(error(
                "duplicate_projection",
                "Object packet repeats the same consumer projection id.",
            ));
        }
        if projection.source_packet_ref != packet.packet_id {
            findings.push(error(
                "projection_packet_ref",
                "Consumer projection must point back to the concrete object packet id.",
            ));
        }
        if !projection.uses_shared_packet {
            findings.push(error(
                "projection_shared_packet",
                "Consumer projection does not use the shared packet.",
            ));
        }
        if projection.hidden_side_cache_created {
            findings.push(error(
                "projection_hidden_side_cache",
                "Consumer projection created a hidden side cache or truth store.",
            ));
        }
        if projection.object_refs.is_empty() || projection.relation_refs.is_empty() {
            findings.push(error(
                "projection_shape",
                "Consumer projection must resolve at least one object and one relation.",
            ));
        }
        for object_ref in &projection.object_refs {
            if packet.object(object_ref).is_none() {
                findings.push(error(
                    "projection_object_ref",
                    "Consumer projection references an unknown object.",
                ));
            }
        }
        for relation_ref in &projection.relation_refs {
            if packet.relation(relation_ref).is_none() {
                findings.push(error(
                    "projection_relation_ref",
                    "Consumer projection references an unknown relation.",
                ));
            }
        }
        validate_projection(packet, projection, &mut findings);
    }

    for required in REQUIRED_CONSUMER_SURFACES {
        if !consumer_surfaces.contains(&required) {
            findings.push(error(
                "consumer_surface_coverage",
                "Object packet is missing a required consumer projection.",
            ));
        }
    }

    let passed = findings
        .iter()
        .all(|finding| finding.severity != InfraBoundaryFindingSeverity::Error);

    SourceIntelligenceObjectPacketValidationReport {
        record_kind: "infra_source_intelligence_object_packet_validation_report".to_string(),
        schema_version: SOURCE_INTELLIGENCE_OBJECT_SCHEMA_VERSION,
        packet_id: packet.packet_id.clone(),
        passed,
        families,
        truth_layers,
        relation_edges,
        consumer_surfaces,
        findings,
    }
}

fn validate_projection(
    packet: &SourceIntelligenceObjectPacket,
    projection: &InfrastructureConsumerProjection,
    findings: &mut Vec<InfraBoundaryFinding>,
) {
    match projection.surface {
        InfrastructureConsumerSurface::Graph => {
            if projection.object_refs.len() != packet.object_records.len()
                || projection.relation_refs.len() != packet.relation_records.len()
            {
                findings.push(error(
                    "graph_projection_scope",
                    "Graph projection must resolve every infrastructure object and relation.",
                ));
            }
        }
        InfrastructureConsumerSurface::Review => {
            if !projection_has_edge(packet, projection, RelationEdgeClass::ReviewAnchor) {
                findings.push(error(
                    "review_projection_anchor",
                    "Review projection must resolve review-anchor relations.",
                ));
            }
        }
        InfrastructureConsumerSurface::Docs => {
            if !projection_has_edge(packet, projection, RelationEdgeClass::SourceOfRender)
                || !projection_has_edge(packet, projection, RelationEdgeClass::RunbookReference)
            {
                findings.push(error(
                    "docs_projection_lineage",
                    "Docs projection must resolve render lineage and runbook-reference edges.",
                ));
            }
        }
        InfrastructureConsumerSurface::Incident => {
            if !projection_has_edge(packet, projection, RelationEdgeClass::LiveCounterpartOf)
                || !projection_has_edge(packet, projection, RelationEdgeClass::Impacts)
            {
                findings.push(error(
                    "incident_projection_live_edges",
                    "Incident projection must resolve live-counterpart and impact edges.",
                ));
            }
        }
    }

    if !projection_covers_contexts(packet, projection) {
        findings.push(error(
            "projection_environment_slice_coverage",
            "Consumer projection must preserve every object in each projected environment slice.",
        ));
    }

    for required_edge in required_flow_edges(projection.surface) {
        if !projection_covers_relation_flow(packet, projection, *required_edge) {
            findings.push(error(
                "projection_flow_coverage",
                "Consumer projection drops relation edges required for shared infrastructure flows.",
            ));
        }
    }
}

fn projection_has_edge(
    packet: &SourceIntelligenceObjectPacket,
    projection: &InfrastructureConsumerProjection,
    edge_class: RelationEdgeClass,
) -> bool {
    projection
        .relation_refs
        .iter()
        .filter_map(|relation_ref| packet.relation(relation_ref))
        .any(|relation| relation.edge_class == edge_class)
}

fn relation_layers_match(
    edge_class: RelationEdgeClass,
    from_layer: TruthLayer,
    to_layer: TruthLayer,
) -> bool {
    match edge_class {
        RelationEdgeClass::SourceOfRender => {
            from_layer == TruthLayer::AuthoredDesired && to_layer == TruthLayer::RenderedExpanded
        }
        RelationEdgeClass::PlanFor => {
            matches!(
                from_layer,
                TruthLayer::AuthoredDesired | TruthLayer::RenderedExpanded
            ) && to_layer == TruthLayer::PlannedValidated
        }
        RelationEdgeClass::LiveCounterpartOf => {
            matches!(
                from_layer,
                TruthLayer::AuthoredDesired
                    | TruthLayer::RenderedExpanded
                    | TruthLayer::PlannedValidated
            ) && to_layer == TruthLayer::ObservedLive
        }
        RelationEdgeClass::AppliedBy => {
            from_layer == TruthLayer::ObservedLive && to_layer == TruthLayer::ProviderOverlay
        }
        RelationEdgeClass::OwnedBy => {
            from_layer == TruthLayer::ObservedLive && to_layer == TruthLayer::ObservedLive
        }
        RelationEdgeClass::Impacts => {
            matches!(
                from_layer,
                TruthLayer::PlannedValidated | TruthLayer::ObservedLive
            ) && matches!(
                to_layer,
                TruthLayer::ObservedLive | TruthLayer::ProviderOverlay
            )
        }
        RelationEdgeClass::RunbookReference => {
            matches!(
                from_layer,
                TruthLayer::AuthoredDesired | TruthLayer::ObservedLive
            ) && to_layer == TruthLayer::ProviderOverlay
        }
        RelationEdgeClass::ReviewAnchor => {
            matches!(
                from_layer,
                TruthLayer::AuthoredDesired | TruthLayer::PlannedValidated
            ) && to_layer == TruthLayer::ProviderOverlay
        }
        RelationEdgeClass::ProviderOverlayOf => {
            from_layer == TruthLayer::ProviderOverlay
                && matches!(
                    to_layer,
                    TruthLayer::ObservedLive | TruthLayer::PlannedValidated
                )
        }
    }
}

fn edge_requires_handoff(edge_class: RelationEdgeClass) -> bool {
    matches!(
        edge_class,
        RelationEdgeClass::AppliedBy
            | RelationEdgeClass::RunbookReference
            | RelationEdgeClass::ReviewAnchor
            | RelationEdgeClass::ProviderOverlayOf
    )
}

fn projection_covers_contexts(
    packet: &SourceIntelligenceObjectPacket,
    projection: &InfrastructureConsumerProjection,
) -> bool {
    projection
        .object_refs
        .iter()
        .filter_map(|object_ref| packet.object(object_ref))
        .map(|object| object.context_ref.as_str())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .all(|context_ref| projection_covers_context_slice(packet, projection, context_ref))
}

fn required_flow_edges(surface: InfrastructureConsumerSurface) -> &'static [RelationEdgeClass] {
    match surface {
        InfrastructureConsumerSurface::Graph => &[
            RelationEdgeClass::SourceOfRender,
            RelationEdgeClass::LiveCounterpartOf,
            RelationEdgeClass::AppliedBy,
            RelationEdgeClass::OwnedBy,
            RelationEdgeClass::Impacts,
            RelationEdgeClass::RunbookReference,
            RelationEdgeClass::ReviewAnchor,
            RelationEdgeClass::ProviderOverlayOf,
        ],
        InfrastructureConsumerSurface::Review => &[
            RelationEdgeClass::LiveCounterpartOf,
            RelationEdgeClass::AppliedBy,
            RelationEdgeClass::OwnedBy,
            RelationEdgeClass::Impacts,
            RelationEdgeClass::RunbookReference,
            RelationEdgeClass::ReviewAnchor,
            RelationEdgeClass::ProviderOverlayOf,
        ],
        InfrastructureConsumerSurface::Docs => &[
            RelationEdgeClass::SourceOfRender,
            RelationEdgeClass::LiveCounterpartOf,
            RelationEdgeClass::AppliedBy,
            RelationEdgeClass::OwnedBy,
            RelationEdgeClass::Impacts,
            RelationEdgeClass::RunbookReference,
            RelationEdgeClass::ReviewAnchor,
            RelationEdgeClass::ProviderOverlayOf,
        ],
        InfrastructureConsumerSurface::Incident => &[
            RelationEdgeClass::LiveCounterpartOf,
            RelationEdgeClass::AppliedBy,
            RelationEdgeClass::OwnedBy,
            RelationEdgeClass::Impacts,
            RelationEdgeClass::RunbookReference,
            RelationEdgeClass::ProviderOverlayOf,
        ],
    }
}

fn error(check_id: &str, message: &str) -> InfraBoundaryFinding {
    InfraBoundaryFinding {
        severity: InfraBoundaryFindingSeverity::Error,
        check_id: check_id.to_string(),
        message: message.to_string(),
    }
}

const REQUIRED_CONSUMER_SURFACES: [InfrastructureConsumerSurface; 4] = [
    InfrastructureConsumerSurface::Graph,
    InfrastructureConsumerSurface::Review,
    InfrastructureConsumerSurface::Docs,
    InfrastructureConsumerSurface::Incident,
];

/// Returns a deterministic qualified object packet for tests and fixtures.
pub fn seeded_source_intelligence_object_packet() -> SourceIntelligenceObjectPacket {
    use ActionPosture::{DryRunOnly, HandoffOnly, InspectOnly};
    use ConsoleHandoffPosture::{
        ExplicitMutationBoundary, ExplicitOptionalBoundary, NoConsoleHandoff, OverlayOnlyBoundary,
    };
    use FreshnessLabel::{CurrentSnapshot, Live};

    fn context(
        context_id: &str,
        provider: &str,
        account: &str,
        cluster: Option<&str>,
        namespace: Option<&str>,
        region: Option<&str>,
        tenant: Option<&str>,
        toolchain: &str,
    ) -> EnvironmentContext {
        EnvironmentContext {
            context_id: context_id.to_string(),
            provider: provider.to_string(),
            account_subscription_project: account.to_string(),
            cluster: cluster.map(str::to_string),
            namespace: namespace.map(str::to_string),
            region_zone: region.map(str::to_string),
            tenant: tenant.map(str::to_string),
            workspace_root: "workspace://checkout".to_string(),
            branch_worktree_or_commit: "refs/heads/main".to_string(),
            execution_context_profile: "exec.local.checkout".to_string(),
            toolchain_cli_identity: toolchain.to_string(),
            credential_handle_class: "delegated_read_only".to_string(),
            issuance_source: "workspace-secret-broker".to_string(),
            expiry: Some("2026-06-12T21:00:00Z".to_string()),
            write_scope: "read_only".to_string(),
            observed_at: "2026-06-12T20:00:00Z".to_string(),
            completeness: EnvironmentCompleteness::Complete,
            ambient_context_prohibited: true,
            high_risk: false,
        }
    }

    fn identity(
        identity_class: &str,
        stable_locator: &str,
        namespace_or_scope: Option<&str>,
        selectors: &[&str],
        owners: &[&str],
        source_path_ref: Option<&str>,
        provider_handle_ref: Option<&str>,
    ) -> InfrastructureObjectIdentity {
        InfrastructureObjectIdentity {
            identity_class: identity_class.to_string(),
            stable_locator: stable_locator.to_string(),
            namespace_or_scope: namespace_or_scope.map(str::to_string),
            selector_refs: selectors.iter().map(|value| (*value).to_string()).collect(),
            owner_refs: owners.iter().map(|value| (*value).to_string()).collect(),
            source_path_ref: source_path_ref.map(str::to_string),
            provider_handle_ref: provider_handle_ref.map(str::to_string),
        }
    }

    fn lineage(
        authored_refs: &[&str],
        source_inputs: &[&str],
        source_paths: &[&str],
        tool_identity: Option<&str>,
        tool_version: Option<&str>,
        parameter_refs: &[&str],
    ) -> InfrastructureObjectLineage {
        InfrastructureObjectLineage {
            authored_object_refs: authored_refs
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            source_input_refs: source_inputs
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            known_path_back_to_source_refs: source_paths
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            tool_identity: tool_identity.map(str::to_string),
            tool_version: tool_version.map(str::to_string),
            parameter_or_environment_refs: parameter_refs
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
        }
    }

    fn object(
        object_id: &str,
        family: InfrastructureFamily,
        truth_layer: TruthLayer,
        label: &str,
        context_ref: &str,
        identity: InfrastructureObjectIdentity,
        freshness: FreshnessLabel,
        observed_at: Option<&str>,
        authority_posture: ActionPosture,
        provider_owned: bool,
        lineage: InfrastructureObjectLineage,
        provenance_refs: &[&str],
        support_summary: &str,
    ) -> InfrastructureObjectRecord {
        InfrastructureObjectRecord {
            object_id: object_id.to_string(),
            family,
            truth_layer,
            display_label: label.to_string(),
            context_ref: context_ref.to_string(),
            identity,
            freshness,
            observed_at: observed_at.map(str::to_string),
            authority_posture,
            provider_owned,
            lineage,
            provenance_refs: provenance_refs
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            support_summary: support_summary.to_string(),
        }
    }

    fn relation(
        relation_id: &str,
        family: InfrastructureFamily,
        edge_class: RelationEdgeClass,
        from_object_ref: &str,
        to_object_ref: &str,
        authority_posture: ActionPosture,
        console_handoff_posture: ConsoleHandoffPosture,
        selectors: &[&str],
        owners: &[&str],
        support_summary: &str,
    ) -> InfrastructureObjectRelationRecord {
        InfrastructureObjectRelationRecord {
            relation_id: relation_id.to_string(),
            family,
            edge_class,
            from_object_ref: from_object_ref.to_string(),
            to_object_ref: to_object_ref.to_string(),
            freshness: CurrentSnapshot,
            authority_posture,
            console_handoff_posture,
            selector_refs: selectors.iter().map(|value| (*value).to_string()).collect(),
            owner_refs: owners.iter().map(|value| (*value).to_string()).collect(),
            provenance_refs: vec![format!("prov:{relation_id}")],
            support_summary: support_summary.to_string(),
        }
    }

    let environments = vec![
        context(
            "ctx:terraform",
            "terraform",
            "aws-prod",
            None,
            None,
            Some("us-west-2"),
            Some("tenant-prod"),
            "terraform-cli@1.9.1",
        ),
        context(
            "ctx:kubernetes",
            "kubernetes",
            "payments-prod",
            Some("cluster/payments-prod"),
            Some("payments"),
            Some("us-west-2"),
            Some("tenant-prod"),
            "helm@3.16.1",
        ),
        context(
            "ctx:devcontainer",
            "devcontainer",
            "workspace-runtime",
            None,
            Some("workspace"),
            None,
            Some("tenant-dev"),
            "devcontainer-cli@0.71.0",
        ),
        context(
            "ctx:ci",
            "ci",
            "github-actions/prod",
            None,
            Some("production"),
            Some("us-west-2"),
            Some("tenant-prod"),
            "gh-actions@2026.06",
        ),
        context(
            "ctx:policy",
            "policy",
            "opa-prod",
            Some("cluster/payments-prod"),
            Some("payments"),
            Some("us-west-2"),
            Some("tenant-prod"),
            "conftest@0.60.0",
        ),
    ];

    let objects = vec![
        object(
            "obj:tf:authored",
            InfrastructureFamily::TerraformHcl,
            TruthLayer::AuthoredDesired,
            "payments module",
            "ctx:terraform",
            identity(
                "terraform_resource_address",
                "module.payments.aws_ecs_service.checkout",
                Some("workspace/prod"),
                &["workspace=prod", "service=checkout"],
                &["team/platform"],
                Some("repo://infra/terraform/payments/main.tf"),
                None,
            ),
            CurrentSnapshot,
            None,
            InspectOnly,
            false,
            lineage(
                &[],
                &["repo://infra/terraform/payments/main.tf"],
                &["repo://infra/terraform/payments/main.tf"],
                None,
                None,
                &["workspace=prod"],
            ),
            &["git:commit:checkout"],
            "Authored Terraform source stays revision-scoped.",
        ),
        object(
            "obj:tf:rendered",
            InfrastructureFamily::TerraformHcl,
            TruthLayer::RenderedExpanded,
            "checkout rendered graph",
            "ctx:terraform",
            identity(
                "terraform_rendered_graph",
                "rendered:module.payments.checkout",
                Some("workspace/prod"),
                &["workspace=prod", "service=checkout"],
                &["team/platform"],
                Some("repo://infra/terraform/payments/main.tf"),
                None,
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:00:00Z"),
            InspectOnly,
            false,
            lineage(
                &["obj:tf:authored"],
                &["repo://infra/terraform/payments/main.tf"],
                &["repo://infra/terraform/payments/main.tf"],
                Some("terraform"),
                Some("1.9.1"),
                &["workspace=prod"],
            ),
            &["terraform:graph:checkout"],
            "Rendered Terraform preserves tool identity and authored lineage.",
        ),
        object(
            "obj:tf:planned",
            InfrastructureFamily::TerraformHcl,
            TruthLayer::PlannedValidated,
            "checkout plan",
            "ctx:terraform",
            identity(
                "terraform_plan",
                "plan:module.payments.checkout",
                Some("workspace/prod"),
                &["workspace=prod", "service=checkout", "target=aws"],
                &["team/platform"],
                Some("repo://infra/terraform/payments/main.tf"),
                None,
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:02:00Z"),
            DryRunOnly,
            false,
            lineage(
                &["obj:tf:authored"],
                &[
                    "repo://infra/terraform/payments/main.tf",
                    "artifact://terraform/rendered/checkout",
                ],
                &["repo://infra/terraform/payments/main.tf"],
                Some("terraform"),
                Some("1.9.1"),
                &["workspace=prod", "target=aws-prod"],
            ),
            &["terraform:plan:checkout"],
            "Terraform plan keeps target selectors and render lineage explicit.",
        ),
        object(
            "obj:tf:observed",
            InfrastructureFamily::TerraformHcl,
            TruthLayer::ObservedLive,
            "checkout ecs service",
            "ctx:terraform",
            identity(
                "aws_ecs_service",
                "aws_ecs_service.checkout",
                Some("cluster/payments"),
                &["service=checkout", "region=us-west-2"],
                &["team/platform"],
                Some("repo://infra/terraform/payments/main.tf"),
                Some("aws:ecs:service/checkout"),
            ),
            Live,
            Some("2026-06-12T20:04:00Z"),
            InspectOnly,
            false,
            lineage(
                &["obj:tf:authored"],
                &["artifact://terraform/plan/checkout"],
                &["repo://infra/terraform/payments/main.tf"],
                Some("terraform-provider-aws"),
                Some("5.55.0"),
                &["workspace=prod"],
            ),
            &["aws:ecs:describe-service"],
            "Observed Terraform counterpart keeps live freshness and path-back-to-source.",
        ),
        object(
            "obj:tf:overlay",
            InfrastructureFamily::TerraformHcl,
            TruthLayer::ProviderOverlay,
            "checkout aws console overlay",
            "ctx:terraform",
            identity(
                "provider_overlay",
                "overlay:aws:ecs:checkout",
                Some("workspace/prod"),
                &["service=checkout", "region=us-west-2"],
                &["team/platform"],
                Some("repo://infra/terraform/payments/main.tf"),
                Some("console:aws:ecs/service/checkout"),
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:05:00Z"),
            HandoffOnly,
            true,
            lineage(
                &["obj:tf:authored"],
                &["overlay://aws/ecs/checkout"],
                &["repo://infra/terraform/payments/main.tf"],
                Some("aws-console"),
                Some("2026.06"),
                &["workspace=prod"],
            ),
            &["overlay:aws:ecs"],
            "Provider overlay stays an explicit handoff surface.",
        ),
        object(
            "obj:k8s:authored",
            InfrastructureFamily::KubernetesHelm,
            TruthLayer::AuthoredDesired,
            "checkout deployment manifest",
            "ctx:kubernetes",
            identity(
                "kubernetes_manifest",
                "apps/v1/Deployment/payments/checkout",
                Some("payments"),
                &["g=apps", "k=Deployment", "n=payments", "name=checkout"],
                &["team/payments"],
                Some("repo://deploy/checkout/deployment.yaml"),
                None,
            ),
            CurrentSnapshot,
            None,
            InspectOnly,
            false,
            lineage(
                &[],
                &["repo://deploy/checkout/deployment.yaml"],
                &["repo://deploy/checkout/deployment.yaml"],
                None,
                None,
                &["namespace=payments"],
            ),
            &["git:commit:checkout"],
            "Authored Kubernetes manifest stays repo-scoped.",
        ),
        object(
            "obj:k8s:rendered",
            InfrastructureFamily::KubernetesHelm,
            TruthLayer::RenderedExpanded,
            "checkout rendered manifest",
            "ctx:kubernetes",
            identity(
                "helm_rendered_manifest",
                "rendered:apps/v1/Deployment/payments/checkout",
                Some("payments"),
                &["g=apps", "k=Deployment", "n=payments", "name=checkout"],
                &["team/payments"],
                Some("repo://deploy/checkout/chart"),
                None,
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:01:00Z"),
            InspectOnly,
            false,
            lineage(
                &["obj:k8s:authored"],
                &[
                    "repo://deploy/checkout/chart",
                    "repo://deploy/checkout/values.yaml",
                ],
                &[
                    "repo://deploy/checkout/chart/templates/deployment.yaml",
                    "repo://deploy/checkout/values.yaml",
                ],
                Some("helm"),
                Some("3.16.1"),
                &["release=checkout", "namespace=payments"],
            ),
            &["helm:template:checkout"],
            "Rendered Kubernetes output preserves chart lineage.",
        ),
        object(
            "obj:k8s:planned",
            InfrastructureFamily::KubernetesHelm,
            TruthLayer::PlannedValidated,
            "checkout server-side diff",
            "ctx:kubernetes",
            identity(
                "kubernetes_server_side_diff",
                "diff:apps/v1/Deployment/payments/checkout",
                Some("payments"),
                &["g=apps", "k=Deployment", "n=payments", "name=checkout"],
                &["team/payments"],
                Some("repo://deploy/checkout/chart"),
                None,
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:03:00Z"),
            DryRunOnly,
            false,
            lineage(
                &["obj:k8s:authored"],
                &["artifact://helm/rendered/checkout"],
                &["repo://deploy/checkout/chart/templates/deployment.yaml"],
                Some("kubectl"),
                Some("1.31.0"),
                &["namespace=payments", "server-side-apply"],
            ),
            &["kubectl:diff:checkout"],
            "Planned Kubernetes diff keeps rendered and authored lineage.",
        ),
        object(
            "obj:k8s:observed",
            InfrastructureFamily::KubernetesHelm,
            TruthLayer::ObservedLive,
            "checkout deployment live",
            "ctx:kubernetes",
            identity(
                "kubernetes_live_object",
                "live:apps/v1/Deployment/payments/checkout",
                Some("payments"),
                &["g=apps", "k=Deployment", "n=payments", "name=checkout"],
                &["team/payments", "controller=replicaset"],
                Some("repo://deploy/checkout/deployment.yaml"),
                Some("k8s://deployments/payments/checkout"),
            ),
            Live,
            Some("2026-06-12T20:04:00Z"),
            InspectOnly,
            false,
            lineage(
                &["obj:k8s:authored"],
                &["artifact://kubectl/diff/checkout"],
                &["repo://deploy/checkout/deployment.yaml"],
                Some("kubernetes"),
                Some("1.31.0"),
                &["namespace=payments"],
            ),
            &["kubernetes:watch:deployment/checkout"],
            "Observed Kubernetes object preserves live target and source lineage.",
        ),
        object(
            "obj:k8s:overlay",
            InfrastructureFamily::KubernetesHelm,
            TruthLayer::ProviderOverlay,
            "checkout rollout overlay",
            "ctx:kubernetes",
            identity(
                "provider_overlay",
                "overlay:kubernetes:deployment/checkout",
                Some("payments"),
                &["g=apps", "k=Deployment", "n=payments", "name=checkout"],
                &["team/payments"],
                Some("repo://deploy/checkout/deployment.yaml"),
                Some("console:kubernetes:deployment/checkout"),
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:05:00Z"),
            HandoffOnly,
            true,
            lineage(
                &["obj:k8s:authored"],
                &["overlay://kubernetes/rollout/checkout"],
                &["repo://deploy/checkout/deployment.yaml"],
                Some("provider-dashboard"),
                Some("2026.06"),
                &["namespace=payments"],
            ),
            &["overlay:kubernetes:rollout"],
            "Kubernetes overlay remains explicit control-plane context.",
        ),
        object(
            "obj:dev:authored",
            InfrastructureFamily::Devcontainer,
            TruthLayer::AuthoredDesired,
            "workspace devcontainer",
            "ctx:devcontainer",
            identity(
                "devcontainer_config",
                "devcontainer:workspace",
                Some("workspace"),
                &["devcontainer=workspace", "service=app"],
                &["team/devexp"],
                Some("repo://.devcontainer/devcontainer.json"),
                None,
            ),
            CurrentSnapshot,
            None,
            InspectOnly,
            false,
            lineage(
                &[],
                &["repo://.devcontainer/devcontainer.json"],
                &["repo://.devcontainer/devcontainer.json"],
                None,
                None,
                &["workspace=checkout"],
            ),
            &["git:commit:checkout"],
            "Authored devcontainer config stays repo-scoped.",
        ),
        object(
            "obj:dev:rendered",
            InfrastructureFamily::Devcontainer,
            TruthLayer::RenderedExpanded,
            "resolved devcontainer",
            "ctx:devcontainer",
            identity(
                "devcontainer_resolved",
                "resolved:workspace",
                Some("workspace"),
                &[
                    "devcontainer=workspace",
                    "service=app",
                    "image=ghcr.io/acme/app",
                ],
                &["team/devexp"],
                Some("repo://.devcontainer/devcontainer.json"),
                None,
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:01:00Z"),
            InspectOnly,
            false,
            lineage(
                &["obj:dev:authored"],
                &[
                    "repo://.devcontainer/devcontainer.json",
                    "repo://docker-compose.dev.yml",
                ],
                &[
                    "repo://.devcontainer/devcontainer.json",
                    "repo://docker-compose.dev.yml",
                ],
                Some("devcontainer-cli"),
                Some("0.71.0"),
                &["workspace=checkout"],
            ),
            &["devcontainer:resolve"],
            "Rendered devcontainer config preserves authored inputs.",
        ),
        object(
            "obj:dev:planned",
            InfrastructureFamily::Devcontainer,
            TruthLayer::PlannedValidated,
            "workspace prebuild plan",
            "ctx:devcontainer",
            identity(
                "devcontainer_prebuild_plan",
                "plan:workspace",
                Some("workspace"),
                &["devcontainer=workspace", "service=app", "cache=warm"],
                &["team/devexp"],
                Some("repo://.devcontainer/devcontainer.json"),
                None,
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:02:00Z"),
            DryRunOnly,
            false,
            lineage(
                &["obj:dev:authored"],
                &["artifact://devcontainer/resolved/workspace"],
                &["repo://.devcontainer/devcontainer.json"],
                Some("devcontainer-cli"),
                Some("0.71.0"),
                &["workspace=checkout", "cache=warm"],
            ),
            &["devcontainer:plan"],
            "Planned devcontainer state keeps render lineage and build assumptions.",
        ),
        object(
            "obj:dev:observed",
            InfrastructureFamily::Devcontainer,
            TruthLayer::ObservedLive,
            "workspace container live",
            "ctx:devcontainer",
            identity(
                "container_runtime_object",
                "live:container/checkout",
                Some("workspace"),
                &["container=checkout", "service=app"],
                &["team/devexp", "owner=devcontainer"],
                Some("repo://.devcontainer/devcontainer.json"),
                Some("docker://containers/checkout"),
            ),
            Live,
            Some("2026-06-12T20:04:00Z"),
            InspectOnly,
            false,
            lineage(
                &["obj:dev:authored"],
                &["artifact://devcontainer/plan/workspace"],
                &["repo://.devcontainer/devcontainer.json"],
                Some("docker"),
                Some("26.1"),
                &["workspace=checkout"],
            ),
            &["docker:inspect:checkout"],
            "Observed workspace container preserves authored mapping and freshness.",
        ),
        object(
            "obj:dev:overlay",
            InfrastructureFamily::Devcontainer,
            TruthLayer::ProviderOverlay,
            "workspace container overlay",
            "ctx:devcontainer",
            identity(
                "provider_overlay",
                "overlay:docker:container/checkout",
                Some("workspace"),
                &["container=checkout", "service=app"],
                &["team/devexp"],
                Some("repo://.devcontainer/devcontainer.json"),
                Some("console:docker:container/checkout"),
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:05:00Z"),
            HandoffOnly,
            true,
            lineage(
                &["obj:dev:authored"],
                &["overlay://docker/containers/checkout"],
                &["repo://.devcontainer/devcontainer.json"],
                Some("docker-desktop"),
                Some("4.31"),
                &["workspace=checkout"],
            ),
            &["overlay:docker:container"],
            "Devcontainer overlay remains a handoff boundary rather than local truth.",
        ),
        object(
            "obj:ci:authored",
            InfrastructureFamily::CiEnvironment,
            TruthLayer::AuthoredDesired,
            "deploy workflow",
            "ctx:ci",
            identity(
                "ci_workflow",
                "workflow:deploy-checkout",
                Some("production"),
                &["workflow=deploy-checkout", "environment=production"],
                &["team/release"],
                Some("repo://.github/workflows/deploy.yml"),
                None,
            ),
            CurrentSnapshot,
            None,
            InspectOnly,
            false,
            lineage(
                &[],
                &["repo://.github/workflows/deploy.yml"],
                &["repo://.github/workflows/deploy.yml"],
                None,
                None,
                &["environment=production"],
            ),
            &["git:commit:checkout"],
            "Authored CI workflow stays repo-scoped.",
        ),
        object(
            "obj:ci:rendered",
            InfrastructureFamily::CiEnvironment,
            TruthLayer::RenderedExpanded,
            "deploy workflow expanded",
            "ctx:ci",
            identity(
                "ci_expanded_workflow",
                "expanded:deploy-checkout",
                Some("production"),
                &[
                    "workflow=deploy-checkout",
                    "environment=production",
                    "matrix=linux",
                ],
                &["team/release"],
                Some("repo://.github/workflows/deploy.yml"),
                None,
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:01:00Z"),
            InspectOnly,
            false,
            lineage(
                &["obj:ci:authored"],
                &["repo://.github/workflows/deploy.yml"],
                &["repo://.github/workflows/deploy.yml"],
                Some("github-actions"),
                Some("2026.06"),
                &["environment=production"],
            ),
            &["github-actions:expand"],
            "Expanded CI workflow preserves authored source and tool identity.",
        ),
        object(
            "obj:ci:planned",
            InfrastructureFamily::CiEnvironment,
            TruthLayer::PlannedValidated,
            "deploy rollout preview",
            "ctx:ci",
            identity(
                "ci_rollout_preview",
                "preview:deploy-checkout",
                Some("production"),
                &[
                    "workflow=deploy-checkout",
                    "environment=production",
                    "target=checkout",
                ],
                &["team/release"],
                Some("repo://.github/workflows/deploy.yml"),
                None,
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:02:00Z"),
            DryRunOnly,
            false,
            lineage(
                &["obj:ci:authored"],
                &["artifact://ci/expanded/deploy-checkout"],
                &["repo://.github/workflows/deploy.yml"],
                Some("rollout-planner"),
                Some("2.4.0"),
                &["environment=production", "target=checkout"],
            ),
            &["ci:preview:checkout"],
            "Planned CI rollout preserves workflow lineage and target selector.",
        ),
        object(
            "obj:ci:observed",
            InfrastructureFamily::CiEnvironment,
            TruthLayer::ObservedLive,
            "deploy run 7421",
            "ctx:ci",
            identity(
                "ci_run",
                "run:7421",
                Some("production"),
                &[
                    "workflow=deploy-checkout",
                    "run=7421",
                    "environment=production",
                ],
                &["team/release"],
                Some("repo://.github/workflows/deploy.yml"),
                Some("gh://actions/runs/7421"),
            ),
            Live,
            Some("2026-06-12T20:04:00Z"),
            InspectOnly,
            false,
            lineage(
                &["obj:ci:authored"],
                &["artifact://ci/preview/deploy-checkout"],
                &["repo://.github/workflows/deploy.yml"],
                Some("github-actions"),
                Some("2026.06"),
                &["environment=production"],
            ),
            &["ci:run:7421"],
            "Observed CI run preserves target scope and workflow lineage.",
        ),
        object(
            "obj:ci:overlay",
            InfrastructureFamily::CiEnvironment,
            TruthLayer::ProviderOverlay,
            "deploy environment overlay",
            "ctx:ci",
            identity(
                "provider_overlay",
                "overlay:ci:environment/production",
                Some("production"),
                &["environment=production", "workflow=deploy-checkout"],
                &["team/release"],
                Some("repo://.github/workflows/deploy.yml"),
                Some("console:ci:environments/production"),
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:05:00Z"),
            HandoffOnly,
            true,
            lineage(
                &["obj:ci:authored"],
                &["overlay://ci/environments/production"],
                &["repo://.github/workflows/deploy.yml"],
                Some("provider-dashboard"),
                Some("2026.06"),
                &["environment=production"],
            ),
            &["overlay:ci:environment"],
            "CI provider overlay remains explicit hosted context.",
        ),
        object(
            "obj:policy:authored",
            InfrastructureFamily::PolicyManifest,
            TruthLayer::AuthoredDesired,
            "checkout deny policy",
            "ctx:policy",
            identity(
                "policy_manifest",
                "policy:checkout-deny-privileged",
                Some("payments"),
                &["policy=checkout-deny-privileged", "namespace=payments"],
                &["team/security"],
                Some("repo://policy/checkout/deny-privileged.rego"),
                None,
            ),
            CurrentSnapshot,
            None,
            InspectOnly,
            false,
            lineage(
                &[],
                &["repo://policy/checkout/deny-privileged.rego"],
                &["repo://policy/checkout/deny-privileged.rego"],
                None,
                None,
                &["namespace=payments"],
            ),
            &["git:commit:checkout"],
            "Authored policy manifest stays repo-scoped.",
        ),
        object(
            "obj:policy:rendered",
            InfrastructureFamily::PolicyManifest,
            TruthLayer::RenderedExpanded,
            "compiled checkout policy",
            "ctx:policy",
            identity(
                "compiled_policy_bundle",
                "compiled:checkout-deny-privileged",
                Some("payments"),
                &[
                    "policy=checkout-deny-privileged",
                    "namespace=payments",
                    "bundle=rego",
                ],
                &["team/security"],
                Some("repo://policy/checkout/deny-privileged.rego"),
                None,
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:01:00Z"),
            InspectOnly,
            false,
            lineage(
                &["obj:policy:authored"],
                &["repo://policy/checkout/deny-privileged.rego"],
                &["repo://policy/checkout/deny-privileged.rego"],
                Some("opa"),
                Some("0.66.0"),
                &["namespace=payments"],
            ),
            &["opa:build:checkout"],
            "Compiled policy preserves authored lineage and compiler identity.",
        ),
        object(
            "obj:policy:planned",
            InfrastructureFamily::PolicyManifest,
            TruthLayer::PlannedValidated,
            "checkout policy eval",
            "ctx:policy",
            identity(
                "policy_evaluation",
                "eval:checkout-deny-privileged",
                Some("payments"),
                &[
                    "policy=checkout-deny-privileged",
                    "namespace=payments",
                    "resource=deployment/checkout",
                ],
                &["team/security"],
                Some("repo://policy/checkout/deny-privileged.rego"),
                None,
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:02:00Z"),
            DryRunOnly,
            false,
            lineage(
                &["obj:policy:authored"],
                &["artifact://opa/build/checkout"],
                &["repo://policy/checkout/deny-privileged.rego"],
                Some("conftest"),
                Some("0.60.0"),
                &["namespace=payments", "resource=deployment/checkout"],
            ),
            &["conftest:test:checkout"],
            "Planned policy evaluation preserves authored lineage and target scope.",
        ),
        object(
            "obj:policy:observed",
            InfrastructureFamily::PolicyManifest,
            TruthLayer::ObservedLive,
            "checkout admission result",
            "ctx:policy",
            identity(
                "policy_observation",
                "live:admission/checkout-deny-privileged",
                Some("payments"),
                &[
                    "policy=checkout-deny-privileged",
                    "namespace=payments",
                    "resource=deployment/checkout",
                ],
                &["team/security"],
                Some("repo://policy/checkout/deny-privileged.rego"),
                Some("opa://decisions/checkout"),
            ),
            Live,
            Some("2026-06-12T20:04:00Z"),
            InspectOnly,
            false,
            lineage(
                &["obj:policy:authored"],
                &["artifact://conftest/eval/checkout"],
                &["repo://policy/checkout/deny-privileged.rego"],
                Some("opa"),
                Some("0.66.0"),
                &["namespace=payments", "resource=deployment/checkout"],
            ),
            &["opa:decision-log:checkout"],
            "Observed policy decision preserves authored linkage and freshness.",
        ),
        object(
            "obj:policy:overlay",
            InfrastructureFamily::PolicyManifest,
            TruthLayer::ProviderOverlay,
            "checkout policy dashboard",
            "ctx:policy",
            identity(
                "provider_overlay",
                "overlay:policy:checkout-deny-privileged",
                Some("payments"),
                &["policy=checkout-deny-privileged", "namespace=payments"],
                &["team/security"],
                Some("repo://policy/checkout/deny-privileged.rego"),
                Some("console:policy/checkout-deny-privileged"),
            ),
            CurrentSnapshot,
            Some("2026-06-12T20:05:00Z"),
            HandoffOnly,
            true,
            lineage(
                &["obj:policy:authored"],
                &["overlay://policy/dashboard/checkout-deny-privileged"],
                &["repo://policy/checkout/deny-privileged.rego"],
                Some("policy-dashboard"),
                Some("2026.06"),
                &["namespace=payments"],
            ),
            &["overlay:policy:dashboard"],
            "Policy overlay stays explicit and provider-owned.",
        ),
    ];

    let relations = vec![
        relation(
            "rel:tf:plan_for",
            InfrastructureFamily::TerraformHcl,
            RelationEdgeClass::PlanFor,
            "obj:tf:rendered",
            "obj:tf:planned",
            DryRunOnly,
            ExplicitOptionalBoundary,
            &["workspace=prod", "service=checkout"],
            &["team/platform"],
            "Terraform rendered graph resolves to the checkout plan.",
        ),
        relation(
            "rel:tf:live_counterpart",
            InfrastructureFamily::TerraformHcl,
            RelationEdgeClass::LiveCounterpartOf,
            "obj:tf:planned",
            "obj:tf:observed",
            InspectOnly,
            ExplicitMutationBoundary,
            &["workspace=prod", "service=checkout"],
            &["team/platform"],
            "Terraform plan maps to the live ECS service counterpart.",
        ),
        relation(
            "rel:tf:applied_by",
            InfrastructureFamily::TerraformHcl,
            RelationEdgeClass::AppliedBy,
            "obj:tf:observed",
            "obj:tf:overlay",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["service=checkout", "region=us-west-2"],
            &["team/platform"],
            "Terraform live service links to provider apply history.",
        ),
        relation(
            "rel:tf:impacts",
            InfrastructureFamily::TerraformHcl,
            RelationEdgeClass::Impacts,
            "obj:tf:planned",
            "obj:tf:overlay",
            DryRunOnly,
            ExplicitOptionalBoundary,
            &["service=checkout"],
            &["team/platform"],
            "Terraform plan impact stays target-scoped.",
        ),
        relation(
            "rel:tf:runbook",
            InfrastructureFamily::TerraformHcl,
            RelationEdgeClass::RunbookReference,
            "obj:tf:authored",
            "obj:tf:overlay",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["service=checkout"],
            &["team/platform"],
            "Terraform source links to its operational overlay and runbook path.",
        ),
        relation(
            "rel:tf:review",
            InfrastructureFamily::TerraformHcl,
            RelationEdgeClass::ReviewAnchor,
            "obj:tf:planned",
            "obj:tf:overlay",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["service=checkout"],
            &["team/platform"],
            "Terraform plan anchors review through the shared packet.",
        ),
        relation(
            "rel:tf:overlay_of",
            InfrastructureFamily::TerraformHcl,
            RelationEdgeClass::ProviderOverlayOf,
            "obj:tf:overlay",
            "obj:tf:observed",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["service=checkout"],
            &["team/platform"],
            "Terraform overlay enriches but does not replace live truth.",
        ),
        relation(
            "rel:k8s:source_of_render",
            InfrastructureFamily::KubernetesHelm,
            RelationEdgeClass::SourceOfRender,
            "obj:k8s:authored",
            "obj:k8s:rendered",
            InspectOnly,
            NoConsoleHandoff,
            &["g=apps", "k=Deployment", "name=checkout"],
            &["team/payments"],
            "Kubernetes source renders to the Helm-expanded manifest.",
        ),
        relation(
            "rel:k8s:plan_for",
            InfrastructureFamily::KubernetesHelm,
            RelationEdgeClass::PlanFor,
            "obj:k8s:rendered",
            "obj:k8s:planned",
            DryRunOnly,
            ExplicitOptionalBoundary,
            &["g=apps", "k=Deployment", "name=checkout"],
            &["team/payments"],
            "Kubernetes rendered manifest resolves to the server-side diff.",
        ),
        relation(
            "rel:k8s:live_counterpart",
            InfrastructureFamily::KubernetesHelm,
            RelationEdgeClass::LiveCounterpartOf,
            "obj:k8s:rendered",
            "obj:k8s:observed",
            InspectOnly,
            ExplicitMutationBoundary,
            &["g=apps", "k=Deployment", "name=checkout"],
            &["team/payments"],
            "Kubernetes rendered manifest maps to the live deployment.",
        ),
        relation(
            "rel:k8s:owned_by",
            InfrastructureFamily::KubernetesHelm,
            RelationEdgeClass::OwnedBy,
            "obj:k8s:observed",
            "obj:k8s:observed",
            InspectOnly,
            ExplicitMutationBoundary,
            &["controller=replicaset"],
            &["team/payments"],
            "Kubernetes owner/controller edge stays explicit in the shared graph.",
        ),
        relation(
            "rel:k8s:impacts",
            InfrastructureFamily::KubernetesHelm,
            RelationEdgeClass::Impacts,
            "obj:k8s:planned",
            "obj:k8s:observed",
            DryRunOnly,
            ExplicitOptionalBoundary,
            &["namespace=payments", "name=checkout"],
            &["team/payments"],
            "Kubernetes diff shows the impacted live deployment.",
        ),
        relation(
            "rel:k8s:runbook",
            InfrastructureFamily::KubernetesHelm,
            RelationEdgeClass::RunbookReference,
            "obj:k8s:observed",
            "obj:k8s:overlay",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["namespace=payments", "name=checkout"],
            &["team/payments"],
            "Kubernetes live object links to rollout and runbook context.",
        ),
        relation(
            "rel:k8s:review",
            InfrastructureFamily::KubernetesHelm,
            RelationEdgeClass::ReviewAnchor,
            "obj:k8s:planned",
            "obj:k8s:overlay",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["namespace=payments", "name=checkout"],
            &["team/payments"],
            "Kubernetes plan anchors review through the shared packet.",
        ),
        relation(
            "rel:k8s:overlay_of",
            InfrastructureFamily::KubernetesHelm,
            RelationEdgeClass::ProviderOverlayOf,
            "obj:k8s:overlay",
            "obj:k8s:observed",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["namespace=payments", "name=checkout"],
            &["team/payments"],
            "Kubernetes overlay enriches the live deployment without replacing it.",
        ),
        relation(
            "rel:dev:source_of_render",
            InfrastructureFamily::Devcontainer,
            RelationEdgeClass::SourceOfRender,
            "obj:dev:authored",
            "obj:dev:rendered",
            InspectOnly,
            NoConsoleHandoff,
            &["devcontainer=workspace", "service=app"],
            &["team/devexp"],
            "Devcontainer source resolves to the expanded workspace config.",
        ),
        relation(
            "rel:dev:live_counterpart",
            InfrastructureFamily::Devcontainer,
            RelationEdgeClass::LiveCounterpartOf,
            "obj:dev:planned",
            "obj:dev:observed",
            InspectOnly,
            ExplicitMutationBoundary,
            &["container=checkout", "service=app"],
            &["team/devexp"],
            "Devcontainer plan resolves to the live workspace container.",
        ),
        relation(
            "rel:dev:owned_by",
            InfrastructureFamily::Devcontainer,
            RelationEdgeClass::OwnedBy,
            "obj:dev:observed",
            "obj:dev:observed",
            InspectOnly,
            ExplicitMutationBoundary,
            &["container=checkout"],
            &["team/devexp"],
            "Workspace runtime ownership stays explicit.",
        ),
        relation(
            "rel:dev:impacts",
            InfrastructureFamily::Devcontainer,
            RelationEdgeClass::Impacts,
            "obj:dev:planned",
            "obj:dev:observed",
            DryRunOnly,
            ExplicitOptionalBoundary,
            &["service=app"],
            &["team/devexp"],
            "Devcontainer plan exposes impacted workspace runtime slices.",
        ),
        relation(
            "rel:dev:review",
            InfrastructureFamily::Devcontainer,
            RelationEdgeClass::ReviewAnchor,
            "obj:dev:authored",
            "obj:dev:overlay",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["devcontainer=workspace"],
            &["team/devexp"],
            "Devcontainer source anchors review and handoff.",
        ),
        relation(
            "rel:dev:overlay_of",
            InfrastructureFamily::Devcontainer,
            RelationEdgeClass::ProviderOverlayOf,
            "obj:dev:overlay",
            "obj:dev:observed",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["container=checkout"],
            &["team/devexp"],
            "Devcontainer overlay enriches the live workspace container.",
        ),
        relation(
            "rel:ci:plan_for",
            InfrastructureFamily::CiEnvironment,
            RelationEdgeClass::PlanFor,
            "obj:ci:rendered",
            "obj:ci:planned",
            DryRunOnly,
            ExplicitOptionalBoundary,
            &["workflow=deploy-checkout", "environment=production"],
            &["team/release"],
            "Expanded CI workflow resolves to the rollout preview.",
        ),
        relation(
            "rel:ci:applied_by",
            InfrastructureFamily::CiEnvironment,
            RelationEdgeClass::AppliedBy,
            "obj:ci:observed",
            "obj:ci:overlay",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["workflow=deploy-checkout", "run=7421"],
            &["team/release"],
            "Hosted CI run links to its provider-owned execution context.",
        ),
        relation(
            "rel:ci:impacts",
            InfrastructureFamily::CiEnvironment,
            RelationEdgeClass::Impacts,
            "obj:ci:planned",
            "obj:ci:observed",
            DryRunOnly,
            ExplicitOptionalBoundary,
            &["environment=production", "target=checkout"],
            &["team/release"],
            "CI rollout preview exposes the affected hosted run slice.",
        ),
        relation(
            "rel:ci:runbook",
            InfrastructureFamily::CiEnvironment,
            RelationEdgeClass::RunbookReference,
            "obj:ci:authored",
            "obj:ci:overlay",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["workflow=deploy-checkout"],
            &["team/release"],
            "CI workflow links to environment and rollback guidance.",
        ),
        relation(
            "rel:ci:review",
            InfrastructureFamily::CiEnvironment,
            RelationEdgeClass::ReviewAnchor,
            "obj:ci:planned",
            "obj:ci:overlay",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["workflow=deploy-checkout", "environment=production"],
            &["team/release"],
            "CI rollout preview anchors review through the shared packet.",
        ),
        relation(
            "rel:ci:overlay_of",
            InfrastructureFamily::CiEnvironment,
            RelationEdgeClass::ProviderOverlayOf,
            "obj:ci:overlay",
            "obj:ci:observed",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["environment=production", "run=7421"],
            &["team/release"],
            "CI environment overlay enriches but does not replace observed run truth.",
        ),
        relation(
            "rel:policy:source_of_render",
            InfrastructureFamily::PolicyManifest,
            RelationEdgeClass::SourceOfRender,
            "obj:policy:authored",
            "obj:policy:rendered",
            InspectOnly,
            NoConsoleHandoff,
            &["policy=checkout-deny-privileged"],
            &["team/security"],
            "Authored policy compiles into the canonical bundle.",
        ),
        relation(
            "rel:policy:plan_for",
            InfrastructureFamily::PolicyManifest,
            RelationEdgeClass::PlanFor,
            "obj:policy:rendered",
            "obj:policy:planned",
            DryRunOnly,
            ExplicitOptionalBoundary,
            &[
                "policy=checkout-deny-privileged",
                "resource=deployment/checkout",
            ],
            &["team/security"],
            "Compiled policy resolves to the evaluation preview.",
        ),
        relation(
            "rel:policy:owned_by",
            InfrastructureFamily::PolicyManifest,
            RelationEdgeClass::OwnedBy,
            "obj:policy:observed",
            "obj:policy:observed",
            InspectOnly,
            ExplicitMutationBoundary,
            &["policy=checkout-deny-privileged"],
            &["team/security"],
            "Observed policy enforcement keeps ownership explicit.",
        ),
        relation(
            "rel:policy:impacts",
            InfrastructureFamily::PolicyManifest,
            RelationEdgeClass::Impacts,
            "obj:policy:planned",
            "obj:policy:observed",
            DryRunOnly,
            ExplicitOptionalBoundary,
            &["resource=deployment/checkout"],
            &["team/security"],
            "Policy preview exposes the impacted admission result.",
        ),
        relation(
            "rel:policy:runbook",
            InfrastructureFamily::PolicyManifest,
            RelationEdgeClass::RunbookReference,
            "obj:policy:observed",
            "obj:policy:overlay",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["policy=checkout-deny-privileged"],
            &["team/security"],
            "Observed policy result links to dashboard and runbook context.",
        ),
        relation(
            "rel:policy:review",
            InfrastructureFamily::PolicyManifest,
            RelationEdgeClass::ReviewAnchor,
            "obj:policy:planned",
            "obj:policy:overlay",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["policy=checkout-deny-privileged"],
            &["team/security"],
            "Planned policy evaluation anchors review through the shared packet.",
        ),
        relation(
            "rel:policy:overlay_of",
            InfrastructureFamily::PolicyManifest,
            RelationEdgeClass::ProviderOverlayOf,
            "obj:policy:overlay",
            "obj:policy:observed",
            HandoffOnly,
            OverlayOnlyBoundary,
            &["policy=checkout-deny-privileged"],
            &["team/security"],
            "Policy overlay enriches observed enforcement without replacing it.",
        ),
    ];

    let graph_projection = InfrastructureConsumerProjection {
        projection_id: "projection:graph".to_string(),
        surface: InfrastructureConsumerSurface::Graph,
        source_packet_ref: "infra-source-intelligence:m5:objects".to_string(),
        object_refs: objects
            .iter()
            .map(|object| object.object_id.clone())
            .collect(),
        relation_refs: relations
            .iter()
            .map(|relation| relation.relation_id.clone())
            .collect(),
        uses_shared_packet: true,
        hidden_side_cache_created: false,
        support_summary: "Graph projection resolves every infrastructure object and relation."
            .to_string(),
    };
    let review_projection = InfrastructureConsumerProjection {
        projection_id: "projection:review".to_string(),
        surface: InfrastructureConsumerSurface::Review,
        source_packet_ref: "infra-source-intelligence:m5:objects".to_string(),
        object_refs: objects.iter().map(|object| object.object_id.clone()).collect(),
        relation_refs: relations
            .iter()
            .map(|relation| relation.relation_id.clone())
            .collect(),
        uses_shared_packet: true,
        hidden_side_cache_created: false,
        support_summary:
            "Review projection reuses the shared object packet so live counterpart, applied-by, owned-by, impacts, and review anchors stay explicit."
                .to_string(),
    };
    let docs_projection = InfrastructureConsumerProjection {
        projection_id: "projection:docs".to_string(),
        surface: InfrastructureConsumerSurface::Docs,
        source_packet_ref: "infra-source-intelligence:m5:objects".to_string(),
        object_refs: objects.iter().map(|object| object.object_id.clone()).collect(),
        relation_refs: relations
            .iter()
            .map(|relation| relation.relation_id.clone())
            .collect(),
        uses_shared_packet: true,
        hidden_side_cache_created: false,
        support_summary:
            "Docs projection reuses the shared object packet so lineage, live counterpart, ownership, impact, runbook, and review cues stay in one vocabulary."
                .to_string(),
    };
    let incident_projection = InfrastructureConsumerProjection {
        projection_id: "projection:incident".to_string(),
        surface: InfrastructureConsumerSurface::Incident,
        source_packet_ref: "infra-source-intelligence:m5:objects".to_string(),
        object_refs: objects.iter().map(|object| object.object_id.clone()).collect(),
        relation_refs: relations
            .iter()
            .map(|relation| relation.relation_id.clone())
            .collect(),
        uses_shared_packet: true,
        hidden_side_cache_created: false,
        support_summary:
            "Incident projection reuses the shared object packet so live counterpart, applied-by, owned-by, impacts, runbooks, and overlays stay target-scoped."
                .to_string(),
    };

    SourceIntelligenceObjectPacket {
        record_kind: SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND.to_string(),
        schema_version: SOURCE_INTELLIGENCE_OBJECT_SCHEMA_VERSION,
        packet_id: "infra-source-intelligence:m5:objects".to_string(),
        captured_at: "2026-06-12T20:05:00Z".to_string(),
        matrix_packet_ref: SOURCE_INTELLIGENCE_RELATIONSHIP_PACKET_RECORD_KIND.to_string(),
        environment_contexts: environments,
        object_records: objects,
        relation_records: relations,
        consumer_projections: vec![
            graph_projection,
            review_projection,
            docs_projection,
            incident_projection,
        ],
        support_summary:
            "Canonical infrastructure object packet keeps authored, rendered, planned, observed, and provider-overlay facts explicit."
                .to_string(),
    }
}
