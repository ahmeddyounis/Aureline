//! Consumer-ready graph fact cues and evidence packet projections.
//!
//! This module projects graph query envelopes into the compact labels that
//! navigation, AI context selection, review seeds, and support exports need
//! in order to show graph truth without exposing raw graph internals.

pub mod beta;

use serde::{Deserialize, Serialize};

use aureline_graph_proto::{EdgeEvidenceState, Freshness};

use crate::{
    GraphPartialTruthCause, GraphQueryEnvelope, GraphQueryReadiness, GraphQueryRow,
    GraphQueryRowClass,
};

/// Schema version for graph fact cue packets.
pub const GRAPH_FACT_CUE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by graph fact cue packets.
pub const GRAPH_FACT_CUE_PACKET_RECORD_KIND: &str = "graph_fact_cue_packet";

/// Surface family consuming graph fact cues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphCueSurface {
    /// Graph-backed navigation such as symbol jump, breadcrumbs, or quick open.
    Navigation,
    /// AI context assembly, prompt inspection, and evidence handoff surfaces.
    AiContext,
    /// Review surfaces such as diff review, impact review, and review seeds.
    Review,
    /// AI context selection and context-inspector candidates.
    AiContextSelection,
    /// Review seed, impact seed, or change-review setup packets.
    ReviewSeed,
    /// Support or evidence export surfaces.
    SupportExport,
}

impl GraphCueSurface {
    /// Returns the stable token used in exported packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Navigation => "navigation",
            Self::AiContext => "ai_context",
            Self::Review => "review",
            Self::AiContextSelection => "ai_context_selection",
            Self::ReviewSeed => "review_seed",
            Self::SupportExport => "support_export",
        }
    }

    /// Returns a compact label suitable for a surface header.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Navigation => "Navigation",
            Self::AiContext => "AI context",
            Self::Review => "Review",
            Self::AiContextSelection => "AI context",
            Self::ReviewSeed => "Review seed",
            Self::SupportExport => "Support export",
        }
    }

    /// Returns true when the surface consumes cues for AI context assembly.
    pub const fn is_ai_context(self) -> bool {
        matches!(self, Self::AiContext | Self::AiContextSelection)
    }

    /// Returns true when the surface consumes cues for review flows.
    pub const fn is_review(self) -> bool {
        matches!(self, Self::Review | Self::ReviewSeed)
    }
}

/// Truth lane for a fact-like graph or fallback-search cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphFactTruthLane {
    /// Fact is proven by current local graph rows for the declared scope.
    ExactLocalGraphFact,
    /// Fact is graph-backed but imported from a bundle, provider overlay, or docs pack.
    ImportedGraphFact,
    /// Fact is graph-backed but inferred or derived rather than directly observed.
    InferredGraphFact,
    /// Fact is graph-backed but stale, cached, or replayed.
    StaleGraphFact,
    /// Fact is graph-backed but incomplete for the declared scope.
    PartialGraphFact,
    /// Fact exists only as a policy-limited projection.
    PolicyHiddenGraphFact,
    /// Fact references a missing anchor.
    MissingAnchorGraphFact,
    /// Graph rows are not ready because a graph producer or richer provider is pending.
    WaitingOnGraphProvider,
    /// Graph subject is outside the active scope.
    OutOfScopeGraphFact,
    /// Candidate came from fallback search rather than graph truth.
    FallbackSearchFact,
}

impl GraphFactTruthLane {
    /// Returns the stable token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactLocalGraphFact => "exact_local_graph_fact",
            Self::ImportedGraphFact => "imported_graph_fact",
            Self::InferredGraphFact => "inferred_graph_fact",
            Self::StaleGraphFact => "stale_graph_fact",
            Self::PartialGraphFact => "partial_graph_fact",
            Self::PolicyHiddenGraphFact => "policy_hidden_graph_fact",
            Self::MissingAnchorGraphFact => "missing_anchor_graph_fact",
            Self::WaitingOnGraphProvider => "waiting_on_graph_provider",
            Self::OutOfScopeGraphFact => "out_of_scope_graph_fact",
            Self::FallbackSearchFact => "fallback_search_fact",
        }
    }

    /// Returns the compact user-facing label for this truth lane.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExactLocalGraphFact => "Exact local graph fact",
            Self::ImportedGraphFact => "Imported graph fact",
            Self::InferredGraphFact => "Inferred graph fact",
            Self::StaleGraphFact => "Stale graph fact",
            Self::PartialGraphFact => "Partial graph fact",
            Self::PolicyHiddenGraphFact => "Policy-limited graph fact",
            Self::MissingAnchorGraphFact => "Missing graph anchor",
            Self::WaitingOnGraphProvider => "Waiting on graph provider",
            Self::OutOfScopeGraphFact => "Outside active graph scope",
            Self::FallbackSearchFact => "Fallback search fact",
        }
    }

    /// Returns true when the cue came from graph truth rather than fallback search.
    pub const fn is_graph_backed(self) -> bool {
        !matches!(self, Self::FallbackSearchFact)
    }

    /// Returns true when surfaces must show a visible cue for this lane.
    pub const fn requires_visible_cue(self) -> bool {
        !matches!(self, Self::ExactLocalGraphFact)
    }
}

/// Action posture surfaces should apply to a graph fact cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphCueActionPosture {
    /// Direct navigation is safe for the declared scope.
    DirectNavigation,
    /// The user should inspect provenance before trusting the row for action.
    InspectBeforeUse,
    /// Imported content should default to read-only or review-before-edit behavior.
    ReadOnlyImported,
    /// The surface should refresh, wait, or narrow scope before strong action.
    RefreshOrNarrowScope,
    /// The surface is waiting on graph or provider readiness.
    WaitingOnProvider,
    /// The candidate is a fallback-search result only.
    FallbackSearchOnly,
    /// The cue is blocked or limited by policy, missing anchors, or scope.
    BlockedByPolicyOrScope,
}

impl GraphCueActionPosture {
    /// Returns the stable token used in exported packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectNavigation => "direct_navigation",
            Self::InspectBeforeUse => "inspect_before_use",
            Self::ReadOnlyImported => "read_only_imported",
            Self::RefreshOrNarrowScope => "refresh_or_narrow_scope",
            Self::WaitingOnProvider => "waiting_on_provider",
            Self::FallbackSearchOnly => "fallback_search_only",
            Self::BlockedByPolicyOrScope => "blocked_by_policy_or_scope",
        }
    }
}

/// One renderable graph fact cue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphFactCue {
    /// Stable cue id inside the packet.
    pub cue_id: String,
    /// Canonical graph id when this cue came from a graph row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_ref: Option<String>,
    /// Human-readable row label safe for result titles.
    pub display_label: String,
    /// Graph row class token or `fallback_search`.
    pub row_class: String,
    /// Truth lane surfaces must render or preserve.
    pub truth_lane: GraphFactTruthLane,
    /// Readiness token observed on the graph or fallback lane.
    pub readiness: String,
    /// Action posture derived from truth lane and readiness.
    pub action_posture: GraphCueActionPosture,
    /// Confidence token when graph evidence supplied one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence_level: Option<String>,
    /// Freshness token when graph evidence supplied one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness: Option<String>,
    /// Edge evidence token when the cue came from an edge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_state: Option<String>,
    /// Partial-truth cause tokens copied from the graph query row.
    #[serde(default)]
    pub partial_truth_causes: Vec<String>,
    /// Workspace-relative path when navigation can use one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    /// Stable symbol ref when navigation can use one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_ref: Option<String>,
    /// Labels that must survive evidence export.
    #[serde(default)]
    pub export_labels: Vec<String>,
}

impl GraphFactCue {
    /// Returns true when this cue came from a graph-backed lane.
    pub const fn is_graph_backed(&self) -> bool {
        self.truth_lane.is_graph_backed()
    }
}

/// Exportable packet of graph fact cues for one consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphFactCuePacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this packet.
    pub schema_version: u32,
    /// Stable packet id for evidence joins.
    pub packet_id: String,
    /// Surface family consuming these cues.
    pub consumer_surface: GraphCueSurface,
    /// Envelope id or fallback-search packet id used as source.
    pub source_packet_ref: String,
    /// Query request id copied from the graph or fallback lane.
    pub query_request_id: String,
    /// Workspace id the packet answers.
    pub workspace_id: String,
    /// Workspace graph id when graph-backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_graph_id: Option<String>,
    /// Query class token when graph-backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_class: Option<String>,
    /// Query-family tag token when graph-backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_family_tag: Option<String>,
    /// Monotonic or fixture timestamp for export parity.
    pub emitted_at: String,
    /// Packet-level readiness token.
    pub readiness: String,
    /// Unique truth-lane tokens present in this packet.
    pub truth_lanes: Vec<GraphFactTruthLane>,
    /// True when every cue preserves its truth labels for export.
    pub export_preserves_fact_labels: bool,
    /// Ordered consumer cues.
    pub cues: Vec<GraphFactCue>,
}

impl GraphFactCuePacket {
    /// Builds a cue packet from an alpha graph query envelope.
    pub fn from_graph_query_envelope(
        packet_id: impl Into<String>,
        consumer_surface: GraphCueSurface,
        envelope: &GraphQueryEnvelope,
    ) -> Self {
        let mut cues = envelope
            .rows
            .iter()
            .map(|row| cue_from_graph_row(consumer_surface, envelope, row))
            .collect::<Vec<_>>();
        if cues.is_empty() {
            cues.push(cue_from_empty_graph_envelope(consumer_surface, envelope));
        }
        let truth_lanes = unique_truth_lanes(&cues);
        Self {
            record_kind: GRAPH_FACT_CUE_PACKET_RECORD_KIND.to_owned(),
            schema_version: GRAPH_FACT_CUE_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            consumer_surface,
            source_packet_ref: envelope.envelope_id.clone(),
            query_request_id: envelope.query_request_id.clone(),
            workspace_id: envelope.workspace_id.clone(),
            workspace_graph_id: Some(envelope.workspace_graph_id.clone()),
            query_class: Some(envelope.query_class.as_str().to_owned()),
            query_family_tag: Some(envelope.query_family_tag.as_str().to_owned()),
            emitted_at: envelope.emitted_at.clone(),
            readiness: envelope.readiness.as_str().to_owned(),
            truth_lanes,
            export_preserves_fact_labels: cues.iter().all(|cue| {
                cue.export_labels
                    .iter()
                    .any(|label| label == cue.truth_lane.as_str())
            }),
            cues,
        }
    }

    /// Builds a cue packet for fallback search candidates.
    pub fn from_fallback_search(
        packet_id: impl Into<String>,
        consumer_surface: GraphCueSurface,
        query_request_id: impl Into<String>,
        workspace_id: impl Into<String>,
        fallback_result_ref: impl Into<String>,
        readiness: impl Into<String>,
        emitted_at: impl Into<String>,
    ) -> Self {
        let packet_id = packet_id.into();
        let query_request_id = query_request_id.into();
        let workspace_id = workspace_id.into();
        let fallback_result_ref = fallback_result_ref.into();
        let readiness = readiness.into();
        let emitted_at = emitted_at.into();
        let cue = GraphFactCue {
            cue_id: format!("{packet_id}:cue:fallback"),
            graph_ref: None,
            display_label: fallback_result_ref.clone(),
            row_class: "fallback_search".to_owned(),
            truth_lane: GraphFactTruthLane::FallbackSearchFact,
            readiness: readiness.clone(),
            action_posture: GraphCueActionPosture::FallbackSearchOnly,
            confidence_level: None,
            freshness: None,
            evidence_state: None,
            partial_truth_causes: Vec::new(),
            relative_path: None,
            symbol_ref: None,
            export_labels: vec![
                GraphFactTruthLane::FallbackSearchFact.as_str().to_owned(),
                format!("readiness:{readiness}"),
                format!("surface:{}", consumer_surface.as_str()),
            ],
        };
        Self {
            record_kind: GRAPH_FACT_CUE_PACKET_RECORD_KIND.to_owned(),
            schema_version: GRAPH_FACT_CUE_SCHEMA_VERSION,
            packet_id: packet_id.clone(),
            consumer_surface,
            source_packet_ref: fallback_result_ref,
            query_request_id,
            workspace_id,
            workspace_graph_id: None,
            query_class: None,
            query_family_tag: None,
            emitted_at,
            readiness,
            truth_lanes: vec![GraphFactTruthLane::FallbackSearchFact],
            export_preserves_fact_labels: true,
            cues: vec![cue],
        }
    }

    /// Returns true when at least one cue came from graph truth.
    pub fn has_graph_backed_cues(&self) -> bool {
        self.cues.iter().any(GraphFactCue::is_graph_backed)
    }

    /// Returns true when visible cue labels are required.
    pub fn requires_visible_cues(&self) -> bool {
        self.readiness != GraphQueryReadiness::Ready.as_str()
            || self
                .truth_lanes
                .iter()
                .any(|lane| lane.requires_visible_cue())
    }

    /// Renders a deterministic plaintext evidence block.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "{} {} surface={} readiness={}\n",
            self.record_kind,
            self.packet_id,
            self.consumer_surface.as_str(),
            self.readiness
        ));
        out.push_str(&format!(
            "source={} query={} workspace={}\n",
            self.source_packet_ref, self.query_request_id, self.workspace_id
        ));
        if let Some(graph_id) = &self.workspace_graph_id {
            out.push_str(&format!("workspace_graph={graph_id}\n"));
        }
        out.push_str("truth_lanes:");
        for lane in &self.truth_lanes {
            out.push_str(&format!(" {}", lane.as_str()));
        }
        out.push('\n');
        for cue in &self.cues {
            out.push_str(&format!(
                "cue {} lane={} readiness={} posture={} label=\"{}\"\n",
                cue.cue_id,
                cue.truth_lane.as_str(),
                cue.readiness,
                cue.action_posture.as_str(),
                cue.display_label
            ));
        }
        out
    }
}

fn cue_from_graph_row(
    consumer_surface: GraphCueSurface,
    envelope: &GraphQueryEnvelope,
    row: &GraphQueryRow,
) -> GraphFactCue {
    let truth_lane = truth_lane_for_graph_row(envelope, row);
    let readiness = envelope.readiness.as_str().to_owned();
    let partial_truth_causes = row
        .partial_truth_causes
        .iter()
        .map(|cause| cause.as_str().to_owned())
        .collect::<Vec<_>>();
    let graph_ref = row.canonical_id().map(str::to_owned);
    let freshness = row.freshness_frame.freshness.as_str().to_owned();
    let evidence_state = row.evidence_state.map(|state| state.as_str().to_owned());
    GraphFactCue {
        cue_id: format!("{}:cue:{}", envelope.envelope_id, row.row_index),
        graph_ref,
        display_label: row.display_label.clone(),
        row_class: row.row_class.as_str().to_owned(),
        truth_lane,
        readiness: readiness.clone(),
        action_posture: action_posture_for(truth_lane),
        confidence_level: Some(row.confidence_level.as_str().to_owned()),
        freshness: Some(freshness.clone()),
        evidence_state,
        partial_truth_causes,
        relative_path: row.relative_path.clone(),
        symbol_ref: row.symbol_ref.clone(),
        export_labels: export_labels_for(
            consumer_surface,
            truth_lane,
            &readiness,
            Some(&freshness),
        ),
    }
}

fn cue_from_empty_graph_envelope(
    consumer_surface: GraphCueSurface,
    envelope: &GraphQueryEnvelope,
) -> GraphFactCue {
    let truth_lane = truth_lane_for_empty_envelope(envelope.readiness);
    let readiness = envelope.readiness.as_str().to_owned();
    GraphFactCue {
        cue_id: format!("{}:cue:none", envelope.envelope_id),
        graph_ref: None,
        display_label: empty_envelope_label(envelope.readiness).to_owned(),
        row_class: "graph_readiness".to_owned(),
        truth_lane,
        readiness: readiness.clone(),
        action_posture: action_posture_for(truth_lane),
        confidence_level: None,
        freshness: None,
        evidence_state: None,
        partial_truth_causes: envelope
            .partial_truth_causes
            .iter()
            .map(|cause| cause.as_str().to_owned())
            .collect(),
        relative_path: None,
        symbol_ref: None,
        export_labels: export_labels_for(consumer_surface, truth_lane, &readiness, None),
    }
}

fn truth_lane_for_graph_row(
    envelope: &GraphQueryEnvelope,
    row: &GraphQueryRow,
) -> GraphFactTruthLane {
    if row.row_class == GraphQueryRowClass::PolicyHidden
        || row
            .partial_truth_causes
            .contains(&GraphPartialTruthCause::PolicyHidden)
    {
        return GraphFactTruthLane::PolicyHiddenGraphFact;
    }
    if row.row_class == GraphQueryRowClass::MissingAnchor
        || row
            .partial_truth_causes
            .contains(&GraphPartialTruthCause::MissingAnchor)
    {
        return GraphFactTruthLane::MissingAnchorGraphFact;
    }
    if envelope.readiness == GraphQueryReadiness::OutOfScope {
        return GraphFactTruthLane::OutOfScopeGraphFact;
    }
    if row
        .partial_truth_causes
        .contains(&GraphPartialTruthCause::Warming)
        || row.freshness_frame.freshness == Freshness::Warming
    {
        return GraphFactTruthLane::WaitingOnGraphProvider;
    }
    if row
        .partial_truth_causes
        .contains(&GraphPartialTruthCause::Imported)
        || row.evidence_state == Some(EdgeEvidenceState::ImportedEvidence)
        || row.freshness_frame.freshness == Freshness::Imported
    {
        return GraphFactTruthLane::ImportedGraphFact;
    }
    if row
        .partial_truth_causes
        .contains(&GraphPartialTruthCause::Derived)
        || row.evidence_state == Some(EdgeEvidenceState::InferredRelation)
    {
        return GraphFactTruthLane::InferredGraphFact;
    }
    if row.partial_truth_causes.iter().any(|cause| {
        matches!(
            cause,
            GraphPartialTruthCause::Stale | GraphPartialTruthCause::Replayed
        )
    }) || row.evidence_state == Some(EdgeEvidenceState::StaleRelation)
        || matches!(
            row.freshness_frame.freshness,
            Freshness::Cached | Freshness::Stale | Freshness::Replayed
        )
    {
        return GraphFactTruthLane::StaleGraphFact;
    }
    if row
        .partial_truth_causes
        .contains(&GraphPartialTruthCause::PartialScope)
    {
        return GraphFactTruthLane::PartialGraphFact;
    }
    GraphFactTruthLane::ExactLocalGraphFact
}

fn truth_lane_for_empty_envelope(readiness: GraphQueryReadiness) -> GraphFactTruthLane {
    match readiness {
        GraphQueryReadiness::Ready | GraphQueryReadiness::HotSetReady => {
            GraphFactTruthLane::ExactLocalGraphFact
        }
        GraphQueryReadiness::Partial => GraphFactTruthLane::PartialGraphFact,
        GraphQueryReadiness::Warming | GraphQueryReadiness::Unavailable => {
            GraphFactTruthLane::WaitingOnGraphProvider
        }
        GraphQueryReadiness::Stale => GraphFactTruthLane::StaleGraphFact,
        GraphQueryReadiness::OutOfScope => GraphFactTruthLane::OutOfScopeGraphFact,
    }
}

fn empty_envelope_label(readiness: GraphQueryReadiness) -> &'static str {
    match readiness {
        GraphQueryReadiness::Ready | GraphQueryReadiness::HotSetReady => "No graph rows matched",
        GraphQueryReadiness::Partial => "Graph facts are partial",
        GraphQueryReadiness::Warming => "Graph facts are warming",
        GraphQueryReadiness::Stale => "Graph facts are stale",
        GraphQueryReadiness::Unavailable => "Graph provider unavailable",
        GraphQueryReadiness::OutOfScope => "Graph subject outside active scope",
    }
}

fn action_posture_for(truth_lane: GraphFactTruthLane) -> GraphCueActionPosture {
    match truth_lane {
        GraphFactTruthLane::ExactLocalGraphFact => GraphCueActionPosture::DirectNavigation,
        GraphFactTruthLane::ImportedGraphFact => GraphCueActionPosture::ReadOnlyImported,
        GraphFactTruthLane::InferredGraphFact | GraphFactTruthLane::StaleGraphFact => {
            GraphCueActionPosture::InspectBeforeUse
        }
        GraphFactTruthLane::PartialGraphFact => GraphCueActionPosture::RefreshOrNarrowScope,
        GraphFactTruthLane::WaitingOnGraphProvider => GraphCueActionPosture::WaitingOnProvider,
        GraphFactTruthLane::FallbackSearchFact => GraphCueActionPosture::FallbackSearchOnly,
        GraphFactTruthLane::PolicyHiddenGraphFact
        | GraphFactTruthLane::MissingAnchorGraphFact
        | GraphFactTruthLane::OutOfScopeGraphFact => GraphCueActionPosture::BlockedByPolicyOrScope,
    }
}

fn export_labels_for(
    consumer_surface: GraphCueSurface,
    truth_lane: GraphFactTruthLane,
    readiness: &str,
    freshness: Option<&str>,
) -> Vec<String> {
    let mut labels = vec![
        truth_lane.as_str().to_owned(),
        format!("readiness:{readiness}"),
        format!("surface:{}", consumer_surface.as_str()),
    ];
    if let Some(freshness) = freshness {
        labels.push(format!("freshness:{freshness}"));
    }
    labels
}

fn unique_truth_lanes(cues: &[GraphFactCue]) -> Vec<GraphFactTruthLane> {
    let mut truth_lanes = Vec::new();
    for cue in cues {
        if !truth_lanes.contains(&cue.truth_lane) {
            truth_lanes.push(cue.truth_lane);
        }
    }
    truth_lanes
}
