//! Infrastructure relation-graph parity for incident, support, and proof flows.
//!
//! This module wraps the canonical infrastructure source-intelligence object
//! packet in a reopenable parity packet for incident timelines, support
//! exports, and proof corpora. It preserves the exact environment slice,
//! relation set, truth-layer mix, stale-live overlay posture, connector-skew
//! label, locality mismatch label, and control-plane handoff lineage that a
//! user saw in-product, so exported evidence can reopen the same governed
//! context later.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    provider_overlay_and_vendor_console_handoff_continuity::PROVIDER_OVERLAY_HANDOFF_PACKET_RECORD_KIND,
    source_intelligence_and_resource_relationships::{
        seeded_source_intelligence_object_packet, InfrastructureFamily,
        SourceIntelligenceObjectPacket, TruthLayer, SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND,
    },
    target_context_and_control_plane_boundary::{
        ActionPosture, ControlPlaneHandoffReason, ControlPlaneReturnSurface, FreshnessLabel,
        InfraBoundaryFinding, InfraBoundaryFindingSeverity,
    },
};

/// Schema version for infrastructure relation-graph incident/support parity packets.
pub const RELATION_GRAPH_PARITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind discriminator for [`RelationGraphIncidentSupportParityPacket`].
pub const RELATION_GRAPH_PARITY_PACKET_RECORD_KIND: &str =
    "infra_relation_graph_incident_support_parity_packet";

/// JSON Schema reference for packet interchange.
pub const RELATION_GRAPH_PARITY_SCHEMA_REF: &str =
    "schemas/infra/relation-graph-incident-support-parity.schema.json";

/// Reviewer-facing documentation reference.
pub const RELATION_GRAPH_PARITY_DOC_REF: &str =
    "docs/infra/relation-graph-incident-support-parity.md";

/// Fixture corpus directory for relation-graph parity and drill coverage.
pub const RELATION_GRAPH_PARITY_FIXTURE_DIR: &str =
    "fixtures/infra/relation-graph-incident-support-parity";

/// Checked support-export artifact for the qualified parity packet.
pub const RELATION_GRAPH_PARITY_ARTIFACT_REF: &str =
    "artifacts/infra/relation-graph-incident-support-parity-support-export.json";

/// Consumer surface that must preserve the same relation-graph context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationGraphParitySurface {
    /// Incident timeline or incident workspace replay packet.
    IncidentPacket,
    /// Support export or escalation bundle.
    SupportExport,
    /// Proof packet or release-evidence corpus.
    ProofCorpus,
}

/// Stale/live posture preserved for provider-overlay rows inside a graph view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaleLiveOverlayState {
    /// Live and overlay facts are current together.
    LiveCurrent,
    /// Live facts remain current, but overlay detail is stale.
    LiveWithStaleOverlay,
    /// Overlay detail is available only through narrowed permissions.
    PermissionLimitedOverlay,
    /// Overlay detail is unavailable and the gap is explicit.
    OverlayUnavailable,
}

/// Closed connector-skew vocabulary exported beside the graph selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectorSkewState {
    /// Connector and packet vocabulary match.
    Matched,
    /// Connector can still explain the graph, but retry or review is required.
    RetryRequired,
    /// Connector skew is outside the supported window.
    UnsupportedSkew,
}

/// Execution plane that produced the visible graph state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionPlane {
    /// A local CLI or resolver produced the state.
    Local,
    /// A remote agent or helper produced the state.
    Remote,
    /// A managed control plane or hosted worker produced the state.
    Managed,
}

/// Local/remote/managed mismatch state preserved for reopened graphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalityMismatchState {
    /// The visible graph and the selected execution plane match.
    Matched,
    /// The graph was captured locally but follow-up would run remotely.
    LocalVsRemote,
    /// The graph was captured remotely but follow-up would run on a managed plane.
    RemoteVsManaged,
    /// The graph was captured locally but follow-up would run on a managed plane.
    LocalVsManaged,
}

/// Drill family that every claimed infrastructure family must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityDrillClass {
    /// Wrong-target posture blocks or downgrades the graph action safely.
    WrongTarget,
    /// Stale-live overlay posture remains explicit.
    StaleLiveOverlay,
    /// Missing-permission posture remains explicit.
    MissingPermission,
    /// Connector skew remains explicit.
    ConnectorSkew,
    /// Local/remote/managed mismatch remains explicit.
    LocalityMismatch,
}

/// Resolution state for one parity drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityDrillResolution {
    /// The drill blocks unsafe continuation.
    Blocked,
    /// The drill narrows to an explicit read-only or handoff posture.
    Downgraded,
    /// The drill stays reopenable with preserved labels.
    Reopenable,
}

/// One exported control-plane handoff lineage row reused by reopened graphs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationGraphHandoffLineage {
    /// Stable lineage id.
    pub lineage_id: String,
    /// Environment context this lineage belongs to.
    pub context_ref: String,
    /// Stable handoff ref preserved across surfaces.
    pub handoff_ref: String,
    /// Stable target object ref that the handoff preserved.
    pub stable_target_ref: String,
    /// Structured reason the handoff was required.
    pub handoff_reason: ControlPlaneHandoffReason,
    /// Surface the user returns to after the handoff.
    pub return_surface: ControlPlaneReturnSurface,
    /// Structured return-anchor ref preserved by the packet.
    pub return_anchor_ref: String,
    /// True when the exported lineage keeps the control-plane boundary explicit.
    pub control_plane_boundary_visible: bool,
    /// Export-safe lineage summary.
    pub support_summary: String,
}

/// One exact relation-graph selection preserved for later reopen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationGraphSelection {
    /// Stable selection id.
    pub selection_id: String,
    /// Infrastructure family the graph selection belongs to.
    pub family: InfrastructureFamily,
    /// Shared environment context shown to the user.
    pub context_ref: String,
    /// Primary object ref that anchored the graph or timeline slice.
    pub primary_object_ref: String,
    /// Root object refs that were visible in the graph selection.
    pub root_object_refs: Vec<String>,
    /// Exact relation ids shown in the graph selection.
    pub relation_refs: Vec<String>,
    /// Truth layers visible in the selection.
    pub visible_truth_layers: Vec<TruthLayer>,
    /// Freshness labels visible in the selection.
    pub freshness_labels: Vec<FreshnessLabel>,
    /// Authority postures visible in the selection.
    pub authority_postures: Vec<ActionPosture>,
    /// Explicit stale/live overlay posture visible in the selection.
    pub stale_live_overlay_state: StaleLiveOverlayState,
    /// Explicit connector-skew label visible in the selection.
    pub connector_skew_state: ConnectorSkewState,
    /// Execution plane that produced the selection.
    pub execution_plane: ExecutionPlane,
    /// Local/remote/managed mismatch state visible in the selection.
    pub locality_mismatch_state: LocalityMismatchState,
    /// Handoff lineage rows needed to preserve the same boundary story.
    pub handoff_lineage_refs: Vec<String>,
    /// Stable digest proving the exact relation set can be reopened.
    pub relation_set_signature: String,
    /// Export-safe graph-selection summary.
    pub support_summary: String,
}

/// Binding proving one consumer can reopen the same graph state safely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationGraphConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Consumer surface covered by the binding.
    pub surface: RelationGraphParitySurface,
    /// Graph selection preserved by the surface.
    pub selection_ref: String,
    /// Stable context ref used to reopen the same environment slice.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen the graph.
    pub reopen_command_id_ref: String,
    /// True when reopen is exact and may not widen to a generic shell.
    pub exact_reopen_only: bool,
    /// True when the surface preserves target identity verbatim.
    pub preserves_target_identity: bool,
    /// True when the surface preserves the exact relation-set signature.
    pub preserves_relation_set: bool,
    /// True when the surface preserves the same visible truth layers.
    pub preserves_truth_layers: bool,
    /// True when the surface preserves the same stale-live overlay posture.
    pub preserves_stale_live_overlay_state: bool,
    /// True when connector skew remains explicit.
    pub preserves_connector_skew_state: bool,
    /// True when locality mismatch remains explicit.
    pub preserves_locality_mismatch_state: bool,
    /// True when the control-plane handoff lineage remains visible.
    pub preserves_handoff_lineage: bool,
    /// True when permission-limited freshness remains explicit when present.
    pub preserves_permission_limited_state: bool,
    /// True when the surface reads the shared packet directly.
    pub uses_shared_packet: bool,
    /// True when the binding remains export-safe.
    pub export_safe_only: bool,
    /// Export-safe binding summary.
    pub support_summary: String,
}

/// One required wrong-target, stale-live, or skew drill row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationGraphParityDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Infrastructure family covered by the drill.
    pub family: InfrastructureFamily,
    /// Drill family represented by the row.
    pub drill_class: ParityDrillClass,
    /// Graph selection used as the proof anchor.
    pub selection_ref: String,
    /// Shared environment context used by the drill.
    pub context_ref: String,
    /// Connector skew state shown during the drill.
    pub connector_skew_state: ConnectorSkewState,
    /// Locality mismatch state shown during the drill.
    pub locality_mismatch_state: LocalityMismatchState,
    /// Stale/live overlay state shown during the drill.
    pub stale_live_overlay_state: StaleLiveOverlayState,
    /// Freshness labels preserved by the drill.
    pub visible_freshness_labels: Vec<FreshnessLabel>,
    /// Resolution taken by the surface.
    pub resolution: ParityDrillResolution,
    /// True when the drill row is present in the proof packet.
    pub proof_packet_included: bool,
    /// Export-safe drill summary.
    pub support_summary: String,
}

/// Canonical packet for infrastructure relation-graph incident/support parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationGraphIncidentSupportParityPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Record-kind of the shared source-intelligence packet.
    pub source_intelligence_packet_ref: String,
    /// Record-kind of the provider-overlay continuity packet used for lineage.
    pub overlay_continuity_packet_ref: String,
    /// Exact shared source-intelligence packet preserved by the export.
    pub source_object_packet: SourceIntelligenceObjectPacket,
    /// Exact graph selections that incident, support, and proof consumers reopen.
    pub relation_graph_selections: Vec<RelationGraphSelection>,
    /// Handoff lineage preserved for reopened selections.
    pub handoff_lineage: Vec<RelationGraphHandoffLineage>,
    /// Consumer bindings proving reopen parity.
    pub consumer_bindings: Vec<RelationGraphConsumerBinding>,
    /// Required wrong-target, stale-live, permission, skew, and locality drills.
    pub drill_records: Vec<RelationGraphParityDrill>,
    /// Export-safe packet summary.
    pub support_summary: String,
}

impl RelationGraphIncidentSupportParityPacket {
    /// Validates the packet against relation-graph parity invariants.
    pub fn validate(&self) -> RelationGraphIncidentSupportParityValidationReport {
        validate_relation_graph_incident_support_parity_packet(self)
    }
}

/// Validation report emitted for relation-graph incident/support parity packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationGraphIncidentSupportParityValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Packet id validated.
    pub packet_id: String,
    /// True when no error-severity finding was emitted.
    pub passed: bool,
    /// Infrastructure families covered by the packet.
    pub families: BTreeSet<InfrastructureFamily>,
    /// Consumer surfaces covered by the packet.
    pub consumer_surfaces: BTreeSet<RelationGraphParitySurface>,
    /// Drill classes covered by the packet.
    pub drill_classes: BTreeSet<ParityDrillClass>,
    /// Findings emitted during validation.
    pub findings: Vec<InfraBoundaryFinding>,
}

/// Validates one relation-graph incident/support parity packet.
pub fn validate_relation_graph_incident_support_parity_packet(
    packet: &RelationGraphIncidentSupportParityPacket,
) -> RelationGraphIncidentSupportParityValidationReport {
    let mut findings = Vec::new();
    let mut families = BTreeSet::new();
    let mut consumer_surfaces = BTreeSet::new();
    let mut drill_classes = BTreeSet::new();
    let mut selection_ids = BTreeSet::new();
    let mut binding_ids = BTreeSet::new();
    let mut drill_ids = BTreeSet::new();
    let mut lineage_ids = BTreeSet::new();
    let mut context_ids = BTreeSet::new();
    let mut object_ids = BTreeSet::new();
    let mut relation_ids = BTreeSet::new();
    let mut handoff_lineage_refs = BTreeSet::new();

    if packet.record_kind != RELATION_GRAPH_PARITY_PACKET_RECORD_KIND {
        findings.push(error(
            "record_kind",
            "Packet record_kind is not the relation-graph incident/support parity discriminator.",
        ));
    }
    if packet.schema_version != RELATION_GRAPH_PARITY_SCHEMA_VERSION {
        findings.push(error(
            "schema_version",
            "Packet schema_version does not match this crate.",
        ));
    }
    if packet.source_intelligence_packet_ref != SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND {
        findings.push(error(
            "source_intelligence_packet_ref",
            "Parity packet must cite the canonical infrastructure object packet record kind.",
        ));
    }
    if packet.overlay_continuity_packet_ref != PROVIDER_OVERLAY_HANDOFF_PACKET_RECORD_KIND {
        findings.push(error(
            "overlay_continuity_packet_ref",
            "Parity packet must cite the provider-overlay continuity packet record kind.",
        ));
    }
    if packet.support_summary.trim().is_empty() {
        findings.push(error(
            "support_summary",
            "Parity packet is missing a support summary.",
        ));
    }

    let source_report = packet.source_object_packet.validate();
    if !source_report.passed {
        findings.push(error(
            "source_object_packet",
            "Parity packet embeds an invalid source-intelligence object packet.",
        ));
    }

    for context in &packet.source_object_packet.environment_contexts {
        context_ids.insert(context.context_id.as_str());
    }
    for object in &packet.source_object_packet.object_records {
        object_ids.insert(object.object_id.as_str());
    }
    for relation in &packet.source_object_packet.relation_records {
        relation_ids.insert(relation.relation_id.as_str());
    }

    for lineage in &packet.handoff_lineage {
        if !lineage_ids.insert(lineage.lineage_id.as_str()) {
            findings.push(error(
                "duplicate_lineage",
                "Parity packet repeats the same handoff lineage id.",
            ));
        }
        if !context_ids.contains(lineage.context_ref.as_str()) {
            findings.push(error(
                "lineage_context_ref",
                "Handoff lineage references an unknown environment context.",
            ));
        }
        if !object_ids.contains(lineage.stable_target_ref.as_str()) {
            findings.push(error(
                "lineage_target_ref",
                "Handoff lineage references an unknown stable target object.",
            ));
        }
        if lineage.handoff_ref.trim().is_empty()
            || lineage.return_anchor_ref.trim().is_empty()
            || lineage.support_summary.trim().is_empty()
        {
            findings.push(error(
                "lineage_shape",
                "Handoff lineage is missing a required handoff, return-anchor, or summary field.",
            ));
        }
        if !lineage.control_plane_boundary_visible {
            findings.push(error(
                "lineage_boundary_visibility",
                "Handoff lineage must keep the control-plane boundary visible.",
            ));
        }
        handoff_lineage_refs.insert(lineage.lineage_id.as_str());
    }

    let mut selection_by_id = BTreeMap::new();
    for selection in &packet.relation_graph_selections {
        families.insert(selection.family);
        if !selection_ids.insert(selection.selection_id.as_str()) {
            findings.push(error(
                "duplicate_selection",
                "Parity packet repeats the same graph-selection id.",
            ));
        }
        if !context_ids.contains(selection.context_ref.as_str()) {
            findings.push(error(
                "selection_context_ref",
                "Graph selection references an unknown environment context.",
            ));
        }
        if !object_ids.contains(selection.primary_object_ref.as_str()) {
            findings.push(error(
                "selection_primary_object_ref",
                "Graph selection references an unknown primary object.",
            ));
        }
        if selection.root_object_refs.is_empty()
            || selection.relation_refs.is_empty()
            || selection.visible_truth_layers.is_empty()
            || selection.freshness_labels.is_empty()
            || selection.authority_postures.is_empty()
            || selection.relation_set_signature.trim().is_empty()
            || selection.support_summary.trim().is_empty()
        {
            findings.push(error(
                "selection_shape",
                "Graph selection is missing root objects, relations, truth layers, freshness, authority, signature, or summary.",
            ));
        }
        for object_ref in &selection.root_object_refs {
            if !object_ids.contains(object_ref.as_str()) {
                findings.push(error(
                    "selection_object_ref",
                    "Graph selection references an unknown root object.",
                ));
            }
        }
        for relation_ref in &selection.relation_refs {
            if !relation_ids.contains(relation_ref.as_str()) {
                findings.push(error(
                    "selection_relation_ref",
                    "Graph selection references an unknown relation.",
                ));
            }
        }
        for lineage_ref in &selection.handoff_lineage_refs {
            if !handoff_lineage_refs.contains(lineage_ref.as_str()) {
                findings.push(error(
                    "selection_handoff_lineage_ref",
                    "Graph selection references unknown handoff lineage.",
                ));
            }
        }
        validate_overlay_state(selection, &mut findings);
        selection_by_id.insert(selection.selection_id.as_str(), selection);
    }

    for required in REQUIRED_FAMILIES {
        if !families.contains(&required) {
            findings.push(error(
                "family_coverage",
                "Parity packet is missing a required infrastructure family.",
            ));
        }
    }

    let mut bindings_by_surface = BTreeMap::<RelationGraphParitySurface, BTreeSet<&str>>::new();
    for binding in &packet.consumer_bindings {
        consumer_surfaces.insert(binding.surface);
        if !binding_ids.insert(binding.binding_id.as_str()) {
            findings.push(error(
                "duplicate_binding",
                "Parity packet repeats the same consumer binding id.",
            ));
        }
        let Some(selection) = selection_by_id.get(binding.selection_ref.as_str()) else {
            findings.push(error(
                "binding_selection_ref",
                "Consumer binding references an unknown graph selection.",
            ));
            continue;
        };
        if binding.reopen_context_ref != selection.context_ref {
            findings.push(error(
                "binding_reopen_context_ref",
                "Consumer binding must reopen the same environment slice context.",
            ));
        }
        if binding.reopen_command_id_ref.trim().is_empty()
            || binding.support_summary.trim().is_empty()
        {
            findings.push(error(
                "binding_shape",
                "Consumer binding is missing reopen command or summary data.",
            ));
        }
        if !binding.exact_reopen_only
            || !binding.preserves_target_identity
            || !binding.preserves_relation_set
            || !binding.preserves_truth_layers
            || !binding.preserves_stale_live_overlay_state
            || !binding.preserves_connector_skew_state
            || !binding.preserves_locality_mismatch_state
            || !binding.preserves_handoff_lineage
            || !binding.uses_shared_packet
            || !binding.export_safe_only
        {
            findings.push(error(
                "binding_parity",
                "Consumer binding must preserve exact reopen, graph, truth, skew, locality, handoff, shared-packet, and export-safe parity.",
            ));
        }
        if selection
            .freshness_labels
            .contains(&FreshnessLabel::PermissionLimited)
            && !binding.preserves_permission_limited_state
        {
            findings.push(error(
                "binding_permission_limited",
                "Permission-limited graph selections must stay explicit in every consumer binding.",
            ));
        }
        bindings_by_surface
            .entry(binding.surface)
            .or_default()
            .insert(binding.selection_ref.as_str());
    }

    for required in REQUIRED_CONSUMER_SURFACES {
        match bindings_by_surface.get(&required) {
            Some(selection_refs) if !selection_refs.is_empty() => {}
            _ => findings.push(error(
                "consumer_surface_coverage",
                "Parity packet is missing a required consumer surface binding.",
            )),
        }
    }

    let mut drills_per_family = BTreeMap::<InfrastructureFamily, BTreeSet<ParityDrillClass>>::new();
    for drill in &packet.drill_records {
        drill_classes.insert(drill.drill_class);
        if !drill_ids.insert(drill.drill_id.as_str()) {
            findings.push(error(
                "duplicate_drill",
                "Parity packet repeats the same drill id.",
            ));
        }
        if !selection_ids.contains(drill.selection_ref.as_str()) {
            findings.push(error(
                "drill_selection_ref",
                "Drill row references an unknown graph selection.",
            ));
        }
        if !context_ids.contains(drill.context_ref.as_str()) {
            findings.push(error(
                "drill_context_ref",
                "Drill row references an unknown environment context.",
            ));
        }
        if drill.visible_freshness_labels.is_empty()
            || drill.support_summary.trim().is_empty()
            || !drill.proof_packet_included
        {
            findings.push(error(
                "drill_shape",
                "Drill row is missing freshness labels, proof inclusion, or summary data.",
            ));
        }
        validate_drill(drill, &mut findings);
        drills_per_family
            .entry(drill.family)
            .or_default()
            .insert(drill.drill_class);
    }

    for family in REQUIRED_FAMILIES {
        let Some(drill_set) = drills_per_family.get(&family) else {
            findings.push(error(
                "drill_family_coverage",
                "Parity packet is missing drill coverage for a claimed infrastructure family.",
            ));
            continue;
        };
        for required in REQUIRED_DRILL_CLASSES {
            if !drill_set.contains(&required) {
                findings.push(error(
                    "drill_class_coverage",
                    "Claimed infrastructure family is missing a required parity drill class.",
                ));
            }
        }
    }

    let passed = findings
        .iter()
        .all(|finding| finding.severity != InfraBoundaryFindingSeverity::Error);
    RelationGraphIncidentSupportParityValidationReport {
        record_kind: "infra_relation_graph_incident_support_parity_validation_report".to_string(),
        schema_version: RELATION_GRAPH_PARITY_SCHEMA_VERSION,
        packet_id: packet.packet_id.clone(),
        passed,
        families,
        consumer_surfaces,
        drill_classes,
        findings,
    }
}

fn validate_overlay_state(
    selection: &RelationGraphSelection,
    findings: &mut Vec<InfraBoundaryFinding>,
) {
    match selection.stale_live_overlay_state {
        StaleLiveOverlayState::LiveCurrent => {
            if selection.freshness_labels.contains(&FreshnessLabel::Stale) {
                findings.push(error(
                    "selection_live_current",
                    "Live-current graph selection may not include stale freshness.",
                ));
            }
        }
        StaleLiveOverlayState::LiveWithStaleOverlay => {
            if !selection.freshness_labels.contains(&FreshnessLabel::Stale)
                || !selection
                    .visible_truth_layers
                    .contains(&TruthLayer::ProviderOverlay)
            {
                findings.push(error(
                    "selection_stale_overlay",
                    "Stale-overlay graph selection must preserve stale freshness and provider-overlay truth.",
                ));
            }
        }
        StaleLiveOverlayState::PermissionLimitedOverlay => {
            if !selection
                .freshness_labels
                .contains(&FreshnessLabel::PermissionLimited)
            {
                findings.push(error(
                    "selection_permission_limited_overlay",
                    "Permission-limited overlay selection must preserve permission-limited freshness.",
                ));
            }
        }
        StaleLiveOverlayState::OverlayUnavailable => {
            if !selection
                .freshness_labels
                .contains(&FreshnessLabel::Unavailable)
            {
                findings.push(error(
                    "selection_overlay_unavailable",
                    "Unavailable-overlay graph selection must preserve unavailable freshness.",
                ));
            }
        }
    }
}

fn validate_drill(drill: &RelationGraphParityDrill, findings: &mut Vec<InfraBoundaryFinding>) {
    match drill.drill_class {
        ParityDrillClass::WrongTarget => {
            if drill.resolution != ParityDrillResolution::Blocked {
                findings.push(error(
                    "drill_wrong_target",
                    "Wrong-target drills must block unsafe continuation.",
                ));
            }
        }
        ParityDrillClass::StaleLiveOverlay => {
            if drill.stale_live_overlay_state != StaleLiveOverlayState::LiveWithStaleOverlay
                || !drill
                    .visible_freshness_labels
                    .contains(&FreshnessLabel::Stale)
            {
                findings.push(error(
                    "drill_stale_overlay",
                    "Stale-live-overlay drills must preserve stale overlay posture explicitly.",
                ));
            }
        }
        ParityDrillClass::MissingPermission => {
            if !drill
                .visible_freshness_labels
                .contains(&FreshnessLabel::PermissionLimited)
            {
                findings.push(error(
                    "drill_missing_permission",
                    "Missing-permission drills must preserve permission-limited freshness.",
                ));
            }
        }
        ParityDrillClass::ConnectorSkew => {
            if drill.connector_skew_state == ConnectorSkewState::Matched {
                findings.push(error(
                    "drill_connector_skew",
                    "Connector-skew drills must preserve a non-matched skew state.",
                ));
            }
        }
        ParityDrillClass::LocalityMismatch => {
            if drill.locality_mismatch_state == LocalityMismatchState::Matched {
                findings.push(error(
                    "drill_locality_mismatch",
                    "Locality-mismatch drills must preserve a non-matched locality state.",
                ));
            }
        }
    }
}

fn error(check_id: &str, message: &str) -> InfraBoundaryFinding {
    InfraBoundaryFinding {
        severity: InfraBoundaryFindingSeverity::Error,
        check_id: check_id.to_string(),
        message: message.to_string(),
    }
}

const REQUIRED_FAMILIES: [InfrastructureFamily; 5] = [
    InfrastructureFamily::TerraformHcl,
    InfrastructureFamily::KubernetesHelm,
    InfrastructureFamily::Devcontainer,
    InfrastructureFamily::CiEnvironment,
    InfrastructureFamily::PolicyManifest,
];

const REQUIRED_CONSUMER_SURFACES: [RelationGraphParitySurface; 3] = [
    RelationGraphParitySurface::IncidentPacket,
    RelationGraphParitySurface::SupportExport,
    RelationGraphParitySurface::ProofCorpus,
];

const REQUIRED_DRILL_CLASSES: [ParityDrillClass; 5] = [
    ParityDrillClass::WrongTarget,
    ParityDrillClass::StaleLiveOverlay,
    ParityDrillClass::MissingPermission,
    ParityDrillClass::ConnectorSkew,
    ParityDrillClass::LocalityMismatch,
];

/// Returns a deterministic qualified relation-graph parity packet for tests and fixtures.
pub fn seeded_relation_graph_incident_support_parity_packet(
) -> RelationGraphIncidentSupportParityPacket {
    let mut source_object_packet = seeded_source_intelligence_object_packet();

    for object in &mut source_object_packet.object_records {
        match object.object_id.as_str() {
            "obj:k8s:overlay" => object.freshness = FreshnessLabel::Stale,
            "obj:policy:overlay" | "obj:policy:observed" => {
                object.freshness = FreshnessLabel::PermissionLimited
            }
            _ => {}
        }
    }
    for relation in &mut source_object_packet.relation_records {
        match relation.relation_id.as_str() {
            "rel:k8s:overlay_of" => relation.freshness = FreshnessLabel::Stale,
            "rel:policy:overlay_of" | "rel:policy:owned_by" => {
                relation.freshness = FreshnessLabel::PermissionLimited
            }
            _ => {}
        }
    }

    let relation_graph_selections = vec![
        RelationGraphSelection {
            selection_id: "selection:terraform:checkout".to_string(),
            family: InfrastructureFamily::TerraformHcl,
            context_ref: "ctx:terraform".to_string(),
            primary_object_ref: "obj:tf:planned".to_string(),
            root_object_refs: vec![
                "obj:tf:planned".to_string(),
                "obj:tf:observed".to_string(),
                "obj:tf:overlay".to_string(),
            ],
            relation_refs: vec![
                "rel:tf:live_counterpart".to_string(),
                "rel:tf:applied_by".to_string(),
                "rel:tf:impacts".to_string(),
                "rel:tf:runbook".to_string(),
                "rel:tf:overlay_of".to_string(),
            ],
            visible_truth_layers: vec![
                TruthLayer::PlannedValidated,
                TruthLayer::ObservedLive,
                TruthLayer::ProviderOverlay,
            ],
            freshness_labels: vec![FreshnessLabel::CurrentSnapshot, FreshnessLabel::Live],
            authority_postures: vec![
                ActionPosture::DryRunOnly,
                ActionPosture::InspectOnly,
                ActionPosture::HandoffOnly,
            ],
            stale_live_overlay_state: StaleLiveOverlayState::LiveCurrent,
            connector_skew_state: ConnectorSkewState::Matched,
            execution_plane: ExecutionPlane::Local,
            locality_mismatch_state: LocalityMismatchState::Matched,
            handoff_lineage_refs: vec!["lineage:terraform:console".to_string()],
            relation_set_signature: "sig:selection:terraform:checkout:v1".to_string(),
            support_summary:
                "Terraform incident/export parity reopens the same checkout relation graph."
                    .to_string(),
        },
        RelationGraphSelection {
            selection_id: "selection:kubernetes:checkout".to_string(),
            family: InfrastructureFamily::KubernetesHelm,
            context_ref: "ctx:kubernetes".to_string(),
            primary_object_ref: "obj:k8s:observed".to_string(),
            root_object_refs: vec![
                "obj:k8s:planned".to_string(),
                "obj:k8s:observed".to_string(),
                "obj:k8s:overlay".to_string(),
            ],
            relation_refs: vec![
                "rel:k8s:live_counterpart".to_string(),
                "rel:k8s:owned_by".to_string(),
                "rel:k8s:impacts".to_string(),
                "rel:k8s:runbook".to_string(),
                "rel:k8s:overlay_of".to_string(),
            ],
            visible_truth_layers: vec![
                TruthLayer::PlannedValidated,
                TruthLayer::ObservedLive,
                TruthLayer::ProviderOverlay,
            ],
            freshness_labels: vec![
                FreshnessLabel::Live,
                FreshnessLabel::CurrentSnapshot,
                FreshnessLabel::Stale,
            ],
            authority_postures: vec![
                ActionPosture::InspectOnly,
                ActionPosture::HandoffOnly,
            ],
            stale_live_overlay_state: StaleLiveOverlayState::LiveWithStaleOverlay,
            connector_skew_state: ConnectorSkewState::Matched,
            execution_plane: ExecutionPlane::Remote,
            locality_mismatch_state: LocalityMismatchState::LocalVsRemote,
            handoff_lineage_refs: vec!["lineage:kubernetes:rollout".to_string()],
            relation_set_signature: "sig:selection:kubernetes:checkout:v1".to_string(),
            support_summary:
                "Kubernetes parity keeps the stale rollout overlay explicit beside live cluster truth."
                    .to_string(),
        },
        RelationGraphSelection {
            selection_id: "selection:devcontainer:workspace".to_string(),
            family: InfrastructureFamily::Devcontainer,
            context_ref: "ctx:devcontainer".to_string(),
            primary_object_ref: "obj:dev:observed".to_string(),
            root_object_refs: vec![
                "obj:dev:planned".to_string(),
                "obj:dev:observed".to_string(),
                "obj:dev:overlay".to_string(),
            ],
            relation_refs: vec![
                "rel:dev:live_counterpart".to_string(),
                "rel:dev:owned_by".to_string(),
                "rel:dev:impacts".to_string(),
                "rel:dev:review".to_string(),
                "rel:dev:overlay_of".to_string(),
            ],
            visible_truth_layers: vec![
                TruthLayer::PlannedValidated,
                TruthLayer::ObservedLive,
                TruthLayer::ProviderOverlay,
            ],
            freshness_labels: vec![FreshnessLabel::CurrentSnapshot, FreshnessLabel::Live],
            authority_postures: vec![
                ActionPosture::InspectOnly,
                ActionPosture::HandoffOnly,
            ],
            stale_live_overlay_state: StaleLiveOverlayState::LiveCurrent,
            connector_skew_state: ConnectorSkewState::RetryRequired,
            execution_plane: ExecutionPlane::Remote,
            locality_mismatch_state: LocalityMismatchState::LocalVsRemote,
            handoff_lineage_refs: vec!["lineage:devcontainer:workspace".to_string()],
            relation_set_signature: "sig:selection:devcontainer:workspace:v1".to_string(),
            support_summary:
                "Devcontainer parity preserves remote-helper provenance and retry-required skew."
                    .to_string(),
        },
        RelationGraphSelection {
            selection_id: "selection:ci:production".to_string(),
            family: InfrastructureFamily::CiEnvironment,
            context_ref: "ctx:ci".to_string(),
            primary_object_ref: "obj:ci:observed".to_string(),
            root_object_refs: vec![
                "obj:ci:planned".to_string(),
                "obj:ci:observed".to_string(),
                "obj:ci:overlay".to_string(),
            ],
            relation_refs: vec![
                "rel:ci:applied_by".to_string(),
                "rel:ci:impacts".to_string(),
                "rel:ci:runbook".to_string(),
                "rel:ci:review".to_string(),
                "rel:ci:overlay_of".to_string(),
            ],
            visible_truth_layers: vec![
                TruthLayer::PlannedValidated,
                TruthLayer::ObservedLive,
                TruthLayer::ProviderOverlay,
            ],
            freshness_labels: vec![FreshnessLabel::CurrentSnapshot, FreshnessLabel::Live],
            authority_postures: vec![
                ActionPosture::DryRunOnly,
                ActionPosture::InspectOnly,
                ActionPosture::HandoffOnly,
            ],
            stale_live_overlay_state: StaleLiveOverlayState::LiveCurrent,
            connector_skew_state: ConnectorSkewState::Matched,
            execution_plane: ExecutionPlane::Managed,
            locality_mismatch_state: LocalityMismatchState::RemoteVsManaged,
            handoff_lineage_refs: vec!["lineage:ci:environment".to_string()],
            relation_set_signature: "sig:selection:ci:production:v1".to_string(),
            support_summary:
                "CI parity preserves the managed environment slice and its hosted overlay lineage."
                    .to_string(),
        },
        RelationGraphSelection {
            selection_id: "selection:policy:payments".to_string(),
            family: InfrastructureFamily::PolicyManifest,
            context_ref: "ctx:policy".to_string(),
            primary_object_ref: "obj:policy:planned".to_string(),
            root_object_refs: vec![
                "obj:policy:planned".to_string(),
                "obj:policy:observed".to_string(),
                "obj:policy:overlay".to_string(),
            ],
            relation_refs: vec![
                "rel:policy:owned_by".to_string(),
                "rel:policy:impacts".to_string(),
                "rel:policy:runbook".to_string(),
                "rel:policy:review".to_string(),
                "rel:policy:overlay_of".to_string(),
            ],
            visible_truth_layers: vec![
                TruthLayer::PlannedValidated,
                TruthLayer::ObservedLive,
                TruthLayer::ProviderOverlay,
            ],
            freshness_labels: vec![
                FreshnessLabel::CurrentSnapshot,
                FreshnessLabel::PermissionLimited,
            ],
            authority_postures: vec![
                ActionPosture::DryRunOnly,
                ActionPosture::InspectOnly,
                ActionPosture::HandoffOnly,
            ],
            stale_live_overlay_state: StaleLiveOverlayState::PermissionLimitedOverlay,
            connector_skew_state: ConnectorSkewState::UnsupportedSkew,
            execution_plane: ExecutionPlane::Managed,
            locality_mismatch_state: LocalityMismatchState::LocalVsManaged,
            handoff_lineage_refs: vec!["lineage:policy:dashboard".to_string()],
            relation_set_signature: "sig:selection:policy:payments:v1".to_string(),
            support_summary:
                "Policy parity preserves permission-limited and unsupported-skew posture explicitly."
                    .to_string(),
        },
    ];

    let handoff_lineage = vec![
        RelationGraphHandoffLineage {
            lineage_id: "lineage:terraform:console".to_string(),
            context_ref: "ctx:terraform".to_string(),
            handoff_ref: "handoff:terraform:console".to_string(),
            stable_target_ref: "obj:tf:observed".to_string(),
            handoff_reason: ControlPlaneHandoffReason::MutationRequiresVendorConsole,
            return_surface: ControlPlaneReturnSurface::InfrastructurePanel,
            return_anchor_ref: "anchor:terraform:checkout".to_string(),
            control_plane_boundary_visible: true,
            support_summary:
                "Terraform export preserves the vendor-console lineage for the checkout service."
                    .to_string(),
        },
        RelationGraphHandoffLineage {
            lineage_id: "lineage:kubernetes:rollout".to_string(),
            context_ref: "ctx:kubernetes".to_string(),
            handoff_ref: "handoff:kubernetes:rollout".to_string(),
            stable_target_ref: "obj:k8s:observed".to_string(),
            handoff_reason: ControlPlaneHandoffReason::IncidentRunbookContinuation,
            return_surface: ControlPlaneReturnSurface::IncidentWorkspace,
            return_anchor_ref: "anchor:kubernetes:checkout".to_string(),
            control_plane_boundary_visible: true,
            support_summary:
                "Kubernetes export preserves the rollout handoff and return-safe incident anchor."
                    .to_string(),
        },
        RelationGraphHandoffLineage {
            lineage_id: "lineage:devcontainer:workspace".to_string(),
            context_ref: "ctx:devcontainer".to_string(),
            handoff_ref: "handoff:devcontainer:workspace".to_string(),
            stable_target_ref: "obj:dev:observed".to_string(),
            handoff_reason: ControlPlaneHandoffReason::ProviderOverlayInspection,
            return_surface: ControlPlaneReturnSurface::CodeBreadcrumb,
            return_anchor_ref: "anchor:devcontainer:workspace".to_string(),
            control_plane_boundary_visible: true,
            support_summary:
                "Devcontainer export preserves the workspace-container overlay handoff lineage."
                    .to_string(),
        },
        RelationGraphHandoffLineage {
            lineage_id: "lineage:ci:environment".to_string(),
            context_ref: "ctx:ci".to_string(),
            handoff_ref: "handoff:ci:environment".to_string(),
            stable_target_ref: "obj:ci:observed".to_string(),
            handoff_reason: ControlPlaneHandoffReason::PreviewOrRouteContinuation,
            return_surface: ControlPlaneReturnSurface::InfrastructurePanel,
            return_anchor_ref: "anchor:ci:production".to_string(),
            control_plane_boundary_visible: true,
            support_summary:
                "CI export preserves the hosted environment handoff lineage and return anchor."
                    .to_string(),
        },
        RelationGraphHandoffLineage {
            lineage_id: "lineage:policy:dashboard".to_string(),
            context_ref: "ctx:policy".to_string(),
            handoff_ref: "handoff:policy:dashboard".to_string(),
            stable_target_ref: "obj:policy:overlay".to_string(),
            handoff_reason: ControlPlaneHandoffReason::PermissionScopeGap,
            return_surface: ControlPlaneReturnSurface::InfrastructurePanel,
            return_anchor_ref: "anchor:policy:payments".to_string(),
            control_plane_boundary_visible: true,
            support_summary:
                "Policy export preserves the permission-gap handoff lineage to the provider dashboard."
                    .to_string(),
        },
    ];

    let consumer_bindings = relation_graph_selections
        .iter()
        .flat_map(|selection| {
            [
                (
                    RelationGraphParitySurface::IncidentPacket,
                    format!("cmd:incident.reopen.{}", selection.selection_id),
                    "Incident packet reopens the same environment slice and relation graph.",
                ),
                (
                    RelationGraphParitySurface::SupportExport,
                    format!("cmd:support.reopen.{}", selection.selection_id),
                    "Support export reopens the same environment slice and relation graph.",
                ),
                (
                    RelationGraphParitySurface::ProofCorpus,
                    format!("cmd:proof.reopen.{}", selection.selection_id),
                    "Proof corpus reopens the same environment slice and relation graph.",
                ),
            ]
            .into_iter()
            .map(move |(surface, reopen_command_id_ref, support_summary)| {
                RelationGraphConsumerBinding {
                    binding_id: format!("binding:{}:{surface:?}", selection.selection_id)
                        .to_lowercase(),
                    surface,
                    selection_ref: selection.selection_id.clone(),
                    reopen_context_ref: selection.context_ref.clone(),
                    reopen_command_id_ref,
                    exact_reopen_only: true,
                    preserves_target_identity: true,
                    preserves_relation_set: true,
                    preserves_truth_layers: true,
                    preserves_stale_live_overlay_state: true,
                    preserves_connector_skew_state: true,
                    preserves_locality_mismatch_state: true,
                    preserves_handoff_lineage: true,
                    preserves_permission_limited_state: true,
                    uses_shared_packet: true,
                    export_safe_only: true,
                    support_summary: support_summary.to_string(),
                }
            })
        })
        .collect();

    let drill_records = REQUIRED_FAMILIES
        .iter()
        .zip(relation_graph_selections.iter())
        .flat_map(|(family, selection)| {
            [
                RelationGraphParityDrill {
                    drill_id: format!("drill:{family:?}:wrong_target").to_lowercase(),
                    family: *family,
                    drill_class: ParityDrillClass::WrongTarget,
                    selection_ref: selection.selection_id.clone(),
                    context_ref: selection.context_ref.clone(),
                    connector_skew_state: selection.connector_skew_state,
                    locality_mismatch_state: selection.locality_mismatch_state,
                    stale_live_overlay_state: selection.stale_live_overlay_state,
                    visible_freshness_labels: selection.freshness_labels.clone(),
                    resolution: ParityDrillResolution::Blocked,
                    proof_packet_included: true,
                    support_summary:
                        "Wrong-target drill blocks mutation and preserves the selected relation graph."
                            .to_string(),
                },
                RelationGraphParityDrill {
                    drill_id: format!("drill:{family:?}:stale_overlay").to_lowercase(),
                    family: *family,
                    drill_class: ParityDrillClass::StaleLiveOverlay,
                    selection_ref: selection.selection_id.clone(),
                    context_ref: selection.context_ref.clone(),
                    connector_skew_state: selection.connector_skew_state,
                    locality_mismatch_state: selection.locality_mismatch_state,
                    stale_live_overlay_state: StaleLiveOverlayState::LiveWithStaleOverlay,
                    visible_freshness_labels: vec![
                        FreshnessLabel::Live,
                        FreshnessLabel::Stale,
                    ],
                    resolution: ParityDrillResolution::Reopenable,
                    proof_packet_included: true,
                    support_summary:
                        "Stale-live-overlay drill preserves stale overlay state beside live target truth."
                            .to_string(),
                },
                RelationGraphParityDrill {
                    drill_id: format!("drill:{family:?}:missing_permission").to_lowercase(),
                    family: *family,
                    drill_class: ParityDrillClass::MissingPermission,
                    selection_ref: selection.selection_id.clone(),
                    context_ref: selection.context_ref.clone(),
                    connector_skew_state: selection.connector_skew_state,
                    locality_mismatch_state: selection.locality_mismatch_state,
                    stale_live_overlay_state: StaleLiveOverlayState::PermissionLimitedOverlay,
                    visible_freshness_labels: vec![FreshnessLabel::PermissionLimited],
                    resolution: ParityDrillResolution::Downgraded,
                    proof_packet_included: true,
                    support_summary:
                        "Missing-permission drill narrows the graph while preserving permission-limited posture."
                            .to_string(),
                },
                RelationGraphParityDrill {
                    drill_id: format!("drill:{family:?}:connector_skew").to_lowercase(),
                    family: *family,
                    drill_class: ParityDrillClass::ConnectorSkew,
                    selection_ref: selection.selection_id.clone(),
                    context_ref: selection.context_ref.clone(),
                    connector_skew_state: if *family == InfrastructureFamily::PolicyManifest {
                        ConnectorSkewState::UnsupportedSkew
                    } else {
                        ConnectorSkewState::RetryRequired
                    },
                    locality_mismatch_state: selection.locality_mismatch_state,
                    stale_live_overlay_state: selection.stale_live_overlay_state,
                    visible_freshness_labels: selection.freshness_labels.clone(),
                    resolution: ParityDrillResolution::Downgraded,
                    proof_packet_included: true,
                    support_summary:
                        "Connector-skew drill preserves retry-required or unsupported-skew posture explicitly."
                            .to_string(),
                },
                RelationGraphParityDrill {
                    drill_id: format!("drill:{family:?}:locality_mismatch").to_lowercase(),
                    family: *family,
                    drill_class: ParityDrillClass::LocalityMismatch,
                    selection_ref: selection.selection_id.clone(),
                    context_ref: selection.context_ref.clone(),
                    connector_skew_state: selection.connector_skew_state,
                    locality_mismatch_state: match family {
                        InfrastructureFamily::TerraformHcl => LocalityMismatchState::LocalVsRemote,
                        InfrastructureFamily::KubernetesHelm => LocalityMismatchState::LocalVsRemote,
                        InfrastructureFamily::Devcontainer => LocalityMismatchState::LocalVsRemote,
                        InfrastructureFamily::CiEnvironment => LocalityMismatchState::RemoteVsManaged,
                        InfrastructureFamily::PolicyManifest => LocalityMismatchState::LocalVsManaged,
                    },
                    stale_live_overlay_state: selection.stale_live_overlay_state,
                    visible_freshness_labels: selection.freshness_labels.clone(),
                    resolution: ParityDrillResolution::Downgraded,
                    proof_packet_included: true,
                    support_summary:
                        "Locality-mismatch drill keeps local, remote, and managed execution skew explicit."
                            .to_string(),
                },
            ]
        })
        .collect();

    RelationGraphIncidentSupportParityPacket {
        record_kind: RELATION_GRAPH_PARITY_PACKET_RECORD_KIND.to_string(),
        schema_version: RELATION_GRAPH_PARITY_SCHEMA_VERSION,
        packet_id: "infra-relation-graph-parity:checkout".to_string(),
        captured_at: "2026-06-12T20:20:00Z".to_string(),
        source_intelligence_packet_ref: SOURCE_INTELLIGENCE_OBJECT_PACKET_RECORD_KIND.to_string(),
        overlay_continuity_packet_ref: PROVIDER_OVERLAY_HANDOFF_PACKET_RECORD_KIND.to_string(),
        source_object_packet,
        relation_graph_selections,
        handoff_lineage,
        consumer_bindings,
        drill_records,
        support_summary:
            "Incident, support, and proof consumers reopen the same infrastructure relation graph with explicit target, stale-overlay, skew, mismatch, and handoff lineage labels."
                .to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_packet_passes() {
        let report = seeded_relation_graph_incident_support_parity_packet().validate();
        assert!(report.passed, "expected pass: {:#?}", report.findings);
        assert_eq!(report.families.len(), 5);
        assert_eq!(report.consumer_surfaces.len(), 3);
    }

    #[test]
    fn missing_support_binding_fails() {
        let mut packet = seeded_relation_graph_incident_support_parity_packet();
        packet
            .consumer_bindings
            .retain(|binding| binding.surface != RelationGraphParitySurface::SupportExport);
        let report = packet.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.check_id == "consumer_surface_coverage"));
    }

    #[test]
    fn permission_limited_binding_must_preserve_permission_state() {
        let mut packet = seeded_relation_graph_incident_support_parity_packet();
        let binding = packet
            .consumer_bindings
            .iter_mut()
            .find(|binding| binding.selection_ref == "selection:policy:payments")
            .expect("policy binding");
        binding.preserves_permission_limited_state = false;
        let report = packet.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.check_id == "binding_permission_limited"));
    }

    #[test]
    fn drill_coverage_requires_every_class() {
        let mut packet = seeded_relation_graph_incident_support_parity_packet();
        packet.drill_records.retain(|drill| {
            !(drill.family == InfrastructureFamily::TerraformHcl
                && drill.drill_class == ParityDrillClass::ConnectorSkew)
        });
        let report = packet.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.check_id == "drill_class_coverage"));
    }
}
