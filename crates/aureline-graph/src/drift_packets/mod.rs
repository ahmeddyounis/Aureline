//! Graph drift packets and support-export parity for graph-backed beta
//! surfaces.
//!
//! This module is the canonical loader, validator, projector, and
//! reporter for the graph drift packet contract. A drift packet binds
//! one graph consumer surface (`navigation`, `ai_context`, `review`,
//! `support_export`) to one alpha graph fact cue packet ref and
//! captures the packet-level readiness state, freshness class, scope
//! class, and data-lane lineage in a single metadata-safe exportable
//! object. The `drift_indicator` is re-derived from the
//! `(readiness, freshness, scope, lineage)` quadruple so prose cannot
//! lie about drift, and a closed `downgrade_label` downgrades a failing
//! row without inventing new vocabulary.
//!
//! Bound to the boundary schema at
//! [`/schemas/graph/drift_packet.schema.json`](../../../../schemas/graph/drift_packet.schema.json),
//! the reviewer doc at
//! [`/docs/support/m3/graph_drift_packets_beta.md`](../../../../docs/support/m3/graph_drift_packets_beta.md),
//! and the baseline report at
//! [`/artifacts/support/m3/graph_drift_packets_report.md`](../../../../artifacts/support/m3/graph_drift_packets_report.md).
//!
//! Support and docs read this module's report projection so the chrome,
//! support packets, and reviewer doc reason about the same drift fields
//! without re-running graph producers and without inferring freshness
//! from timing alone.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_graph_proto::Freshness;

use crate::readiness::{GraphCueSurface, GraphFactCuePacket, GraphFactTruthLane};
use crate::GraphQueryReadiness;

/// Stable record-kind tag for a graph drift packet record.
pub const GRAPH_DRIFT_PACKET_RECORD_KIND: &str = "graph_drift_packet_record";

/// Stable record-kind tag for the graph drift report record.
pub const GRAPH_DRIFT_REPORT_RECORD_KIND: &str = "graph_drift_report_record";

/// Frozen schema version for graph drift packet and report records.
pub const GRAPH_DRIFT_PACKET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const GRAPH_DRIFT_PACKET_SCHEMA_REF: &str = "schemas/graph/drift_packet.schema.json";

/// Repo-relative path of the reviewer doc.
pub const GRAPH_DRIFT_PACKET_DOC_REF: &str = "docs/support/m3/graph_drift_packets_beta.md";

/// Repo-relative path of the baseline report.
pub const GRAPH_DRIFT_PACKET_REPORT_REF: &str =
    "artifacts/support/m3/graph_drift_packets_report.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const GRAPH_DRIFT_PACKET_CORPUS_DIR: &str = "fixtures/graph/m3/drift_packets";

/// Repo-relative path of the protected corpus manifest.
pub const GRAPH_DRIFT_PACKET_CORPUS_MANIFEST_REF: &str =
    "fixtures/graph/m3/drift_packets/manifest.yaml";

/// Closed consumer-surface vocabulary for drift packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftConsumerSurface {
    Navigation,
    AiContext,
    Review,
    SupportExport,
}

impl DriftConsumerSurface {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Navigation => "navigation",
            Self::AiContext => "ai_context",
            Self::Review => "review",
            Self::SupportExport => "support_export",
        }
    }

    /// Lifts a graph fact cue surface into a drift consumer surface.
    /// Selection and seed variants collapse onto the parent surface so
    /// drift packets stay bound to the four required lanes.
    pub const fn from_cue_surface(surface: GraphCueSurface) -> Self {
        match surface {
            GraphCueSurface::Navigation => Self::Navigation,
            GraphCueSurface::AiContext | GraphCueSurface::AiContextSelection => Self::AiContext,
            GraphCueSurface::Review | GraphCueSurface::ReviewSeed => Self::Review,
            GraphCueSurface::SupportExport => Self::SupportExport,
        }
    }
}

/// Closed list of consumer surfaces the corpus must cover.
pub const REQUIRED_DRIFT_CONSUMER_SURFACES: [DriftConsumerSurface; 4] = [
    DriftConsumerSurface::Navigation,
    DriftConsumerSurface::AiContext,
    DriftConsumerSurface::Review,
    DriftConsumerSurface::SupportExport,
];

/// Closed readiness vocabulary reused by drift packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessState {
    Ready,
    HotSetReady,
    Partial,
    Warming,
    Stale,
    Unavailable,
    OutOfScope,
}

impl ReadinessState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::HotSetReady => "hot_set_ready",
            Self::Partial => "partial",
            Self::Warming => "warming",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
            Self::OutOfScope => "out_of_scope",
        }
    }

    pub const fn from_alpha(readiness: GraphQueryReadiness) -> Self {
        match readiness {
            GraphQueryReadiness::Ready => Self::Ready,
            GraphQueryReadiness::HotSetReady => Self::HotSetReady,
            GraphQueryReadiness::Partial => Self::Partial,
            GraphQueryReadiness::Warming => Self::Warming,
            GraphQueryReadiness::Stale => Self::Stale,
            GraphQueryReadiness::Unavailable => Self::Unavailable,
            GraphQueryReadiness::OutOfScope => Self::OutOfScope,
        }
    }
}

/// Closed freshness vocabulary derived from the alpha graph freshness
/// frames; `Unknown` covers fallback-search and empty-envelope packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    Authoritative,
    HotSet,
    Warming,
    Cached,
    Stale,
    Replayed,
    Imported,
    Unknown,
}

impl FreshnessClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::HotSet => "hot_set",
            Self::Warming => "warming",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Replayed => "replayed",
            Self::Imported => "imported",
            Self::Unknown => "unknown",
        }
    }

    /// Returns true when the freshness class is at the live edge.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::Authoritative | Self::HotSet)
    }
}

/// Closed scope vocabulary describing where the packet's truth was
/// drawn from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClass {
    FullLocal,
    SparseLocal,
    FullManaged,
    SparseManaged,
    MixedLocalAndManaged,
    OutOfScope,
}

impl ScopeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullLocal => "full_local",
            Self::SparseLocal => "sparse_local",
            Self::FullManaged => "full_managed",
            Self::SparseManaged => "sparse_managed",
            Self::MixedLocalAndManaged => "mixed_local_and_managed",
            Self::OutOfScope => "out_of_scope",
        }
    }

    /// Returns true when the scope class is a strong, full coverage of
    /// the declared workspace.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::FullLocal | Self::FullManaged)
    }
}

/// Closed data-lane lineage vocabulary the packet records as its
/// source-of-truth lineage. Mirrors the alpha graph fact truth lanes
/// the cue packet declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataLaneLineage {
    ExactLocalGraphLineage,
    ImportedProviderLineage,
    InferredDerivedLineage,
    PartialScopeLineage,
    StaleCachedLineage,
    WarmingProviderLineage,
    OutOfScopeLineage,
    FallbackSearchLineage,
}

impl DataLaneLineage {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactLocalGraphLineage => "exact_local_graph_lineage",
            Self::ImportedProviderLineage => "imported_provider_lineage",
            Self::InferredDerivedLineage => "inferred_derived_lineage",
            Self::PartialScopeLineage => "partial_scope_lineage",
            Self::StaleCachedLineage => "stale_cached_lineage",
            Self::WarmingProviderLineage => "warming_provider_lineage",
            Self::OutOfScopeLineage => "out_of_scope_lineage",
            Self::FallbackSearchLineage => "fallback_search_lineage",
        }
    }

    pub const fn from_fact_truth_lane(lane: GraphFactTruthLane) -> Self {
        match lane {
            GraphFactTruthLane::ExactLocalGraphFact => Self::ExactLocalGraphLineage,
            GraphFactTruthLane::ImportedGraphFact => Self::ImportedProviderLineage,
            GraphFactTruthLane::InferredGraphFact => Self::InferredDerivedLineage,
            GraphFactTruthLane::PartialGraphFact => Self::PartialScopeLineage,
            GraphFactTruthLane::StaleGraphFact => Self::StaleCachedLineage,
            GraphFactTruthLane::WaitingOnGraphProvider => Self::WarmingProviderLineage,
            GraphFactTruthLane::OutOfScopeGraphFact => Self::OutOfScopeLineage,
            GraphFactTruthLane::FallbackSearchFact => Self::FallbackSearchLineage,
            // Policy-hidden and missing-anchor lanes collapse onto
            // partial-scope for drift reporting; the drift report is
            // metadata-only and never publishes policy-hidden subject
            // identifiers.
            GraphFactTruthLane::PolicyHiddenGraphFact
            | GraphFactTruthLane::MissingAnchorGraphFact => Self::PartialScopeLineage,
        }
    }
}

/// Closed list of data-lane lineages the corpus must cover.
pub const REQUIRED_DATA_LANE_LINEAGES: [DataLaneLineage; 8] = [
    DataLaneLineage::ExactLocalGraphLineage,
    DataLaneLineage::ImportedProviderLineage,
    DataLaneLineage::InferredDerivedLineage,
    DataLaneLineage::PartialScopeLineage,
    DataLaneLineage::StaleCachedLineage,
    DataLaneLineage::WarmingProviderLineage,
    DataLaneLineage::OutOfScopeLineage,
    DataLaneLineage::FallbackSearchLineage,
];

/// Closed drift-indicator vocabulary; re-derived from
/// `(readiness, freshness, scope, lineage)` by the evaluator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftIndicator {
    Aligned,
    FreshnessSkew,
    ScopeSkew,
    LineageSkew,
    StaleWarning,
    WarmingWarning,
    BlockedByScope,
    FallbackOnly,
}

impl DriftIndicator {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Aligned => "aligned",
            Self::FreshnessSkew => "freshness_skew",
            Self::ScopeSkew => "scope_skew",
            Self::LineageSkew => "lineage_skew",
            Self::StaleWarning => "stale_warning",
            Self::WarmingWarning => "warming_warning",
            Self::BlockedByScope => "blocked_by_scope",
            Self::FallbackOnly => "fallback_only",
        }
    }

    pub const fn is_aligned(self) -> bool {
        matches!(self, Self::Aligned)
    }
}

/// Closed downgrade-label vocabulary; one of these labels applies when
/// a drift packet downgrades a beta row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftDowngradeLabel {
    None,
    RedDriftBlocksBetaRow,
    YellowFreshnessSkew,
    YellowScopeSkew,
    YellowLineageSkew,
    DegradedToFallbackSearchOnly,
    StaleCorpusBlocksReleaseCandidate,
}

impl DriftDowngradeLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RedDriftBlocksBetaRow => "red_drift_blocks_beta_row",
            Self::YellowFreshnessSkew => "yellow_freshness_skew",
            Self::YellowScopeSkew => "yellow_scope_skew",
            Self::YellowLineageSkew => "yellow_lineage_skew",
            Self::DegradedToFallbackSearchOnly => "degraded_to_fallback_search_only",
            Self::StaleCorpusBlocksReleaseCandidate => "stale_corpus_blocks_release_candidate",
        }
    }

    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::None)
    }
}

/// Closed open-gap class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftOpenGapClass {
    None,
    FreshnessPending,
    ScopePending,
    LineagePending,
    EvidenceExportPending,
    FallbackTruthOnly,
    DriftBlocked,
}

impl DriftOpenGapClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::FreshnessPending => "freshness_pending",
            Self::ScopePending => "scope_pending",
            Self::LineagePending => "lineage_pending",
            Self::EvidenceExportPending => "evidence_export_pending",
            Self::FallbackTruthOnly => "fallback_truth_only",
            Self::DriftBlocked => "drift_blocked",
        }
    }
}

/// One open-gap row attached to a graph drift packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftOpenGapEntry {
    pub gap_class: DriftOpenGapClass,
    pub summary: String,
}

/// Metadata-safe evidence-export projection pinned on each packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftEvidenceExportProjection {
    pub preserves_readiness_token: bool,
    pub preserves_freshness_token: bool,
    pub preserves_scope_label: bool,
    pub preserves_lineage_label: bool,
    pub preserves_consumer_surface_label: bool,
    pub preserves_envelope_packet_ref: bool,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub preserves_user_authored_files: bool,
}

impl DriftEvidenceExportProjection {
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            preserves_readiness_token: true,
            preserves_freshness_token: true,
            preserves_scope_label: true,
            preserves_lineage_label: true,
            preserves_consumer_surface_label: true,
            preserves_envelope_packet_ref: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            preserves_user_authored_files: true,
        }
    }
}

/// Safety baseline pinned on every packet and on the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftPacketSafety {
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub destructive_resets_present: bool,
    pub preserves_user_authored_files: bool,
}

impl DriftPacketSafety {
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
            preserves_user_authored_files: true,
        }
    }
}

/// Companion refs quoted on each packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftPacketReferences {
    pub doc_ref: String,
    pub schema_ref: String,
    pub report_ref: String,
}

impl DriftPacketReferences {
    pub fn pinned() -> Self {
        Self {
            doc_ref: GRAPH_DRIFT_PACKET_DOC_REF.to_owned(),
            schema_ref: GRAPH_DRIFT_PACKET_SCHEMA_REF.to_owned(),
            report_ref: GRAPH_DRIFT_PACKET_REPORT_REF.to_owned(),
        }
    }
}

/// One graph drift packet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphDriftPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub title: String,
    pub consumer_surface: DriftConsumerSurface,
    pub subject_ref: String,
    pub envelope_packet_ref: String,
    pub workspace_id: String,
    pub workspace_graph_id: String,
    pub readiness_state: ReadinessState,
    pub freshness_class: FreshnessClass,
    pub scope_class: ScopeClass,
    pub data_lane_lineage: DataLaneLineage,
    pub drift_indicator: DriftIndicator,
    pub evidence_export: DriftEvidenceExportProjection,
    pub downgrade_label: DriftDowngradeLabel,
    #[serde(default)]
    pub open_gaps: Vec<DriftOpenGapEntry>,
    pub safety: DriftPacketSafety,
    pub references: DriftPacketReferences,
    pub captured_at: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reviewer_summary: Option<String>,
}

/// Inputs the evaluator needs to project an alpha cue packet into a
/// drift packet without reading it back from disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphDriftCompileInput<'a> {
    pub packet_id: String,
    pub title: String,
    pub subject_ref: String,
    pub scope_class: ScopeClass,
    pub cue_packet: &'a GraphFactCuePacket,
    pub captured_at: String,
    pub reviewer_summary: Option<String>,
}

impl GraphDriftPacket {
    /// Builds a drift packet from a [`GraphFactCuePacket`]. The
    /// freshness class is taken from the packet-level freshness on the
    /// originating envelope when present; otherwise from the first
    /// graph-backed cue. The lineage is taken from the strongest
    /// (lowest-strength-index) truth lane present in the cue packet so
    /// fallback noise does not mask exact lineage.
    pub fn compile_from_alpha(input: GraphDriftCompileInput<'_>) -> Self {
        let GraphDriftCompileInput {
            packet_id,
            title,
            subject_ref,
            scope_class,
            cue_packet,
            captured_at,
            reviewer_summary,
        } = input;

        let readiness_state = readiness_from_str(&cue_packet.readiness);
        let freshness_class = freshness_class_from_cue_packet(cue_packet);
        let data_lane_lineage = lineage_from_cue_packet(cue_packet);
        let consumer_surface = DriftConsumerSurface::from_cue_surface(cue_packet.consumer_surface);
        let drift_indicator = derive_drift_indicator(
            readiness_state,
            freshness_class,
            scope_class,
            data_lane_lineage,
        );

        Self {
            schema_version: GRAPH_DRIFT_PACKET_SCHEMA_VERSION,
            record_kind: GRAPH_DRIFT_PACKET_RECORD_KIND.to_owned(),
            packet_id,
            title,
            consumer_surface,
            subject_ref,
            envelope_packet_ref: cue_packet.source_packet_ref.clone(),
            workspace_id: cue_packet.workspace_id.clone(),
            workspace_graph_id: cue_packet
                .workspace_graph_id
                .clone()
                .unwrap_or_else(|| format!("graph:fallback:{}", cue_packet.workspace_id)),
            readiness_state,
            freshness_class,
            scope_class,
            data_lane_lineage,
            drift_indicator,
            evidence_export: DriftEvidenceExportProjection::metadata_safe_baseline(),
            downgrade_label: downgrade_for_indicator(drift_indicator),
            open_gaps: open_gaps_for_indicator(drift_indicator),
            safety: DriftPacketSafety::metadata_safe_baseline(),
            references: DriftPacketReferences::pinned(),
            captured_at,
            reviewer_summary,
        }
    }

    /// Returns a deterministic plaintext rendering of the drift packet
    /// suitable for support packets, reviewer docs, and snapshot tests.
    /// The rendering quotes the closed-vocabulary tokens directly so a
    /// reader does not have to infer drift from timing.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "{} {} surface={} subject={}\n",
            self.record_kind,
            self.packet_id,
            self.consumer_surface.as_str(),
            self.subject_ref,
        ));
        out.push_str(&format!(
            "readiness={} freshness={} scope={} lineage={}\n",
            self.readiness_state.as_str(),
            self.freshness_class.as_str(),
            self.scope_class.as_str(),
            self.data_lane_lineage.as_str(),
        ));
        out.push_str(&format!(
            "drift={} downgrade={} envelope={}\n",
            self.drift_indicator.as_str(),
            self.downgrade_label.as_str(),
            self.envelope_packet_ref,
        ));
        if !self.open_gaps.is_empty() {
            out.push_str("open_gaps:\n");
            for gap in &self.open_gaps {
                out.push_str(&format!("  - {} {}\n", gap.gap_class.as_str(), gap.summary));
            }
        }
        out
    }
}

/// One fixture-bound entry in the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphDriftCorpusEntry {
    pub fixture_ref: String,
    pub packet: GraphDriftPacket,
}

/// Graph drift packet corpus loaded from checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphDriftCorpus {
    pub entries: Vec<GraphDriftCorpusEntry>,
}

/// One row in the report matrix projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftReportMatrixRow {
    pub packet_id: String,
    pub consumer_surface: DriftConsumerSurface,
    pub subject_ref: String,
    pub readiness_state: ReadinessState,
    pub freshness_class: FreshnessClass,
    pub scope_class: ScopeClass,
    pub data_lane_lineage: DataLaneLineage,
    pub drift_indicator: DriftIndicator,
    pub downgrade_label: DriftDowngradeLabel,
    pub open_gap_classes: Vec<DriftOpenGapClass>,
}

impl DriftReportMatrixRow {
    fn from_packet(packet: &GraphDriftPacket) -> Self {
        let mut open_gap_classes: Vec<DriftOpenGapClass> =
            packet.open_gaps.iter().map(|gap| gap.gap_class).collect();
        if open_gap_classes.is_empty() {
            open_gap_classes.push(DriftOpenGapClass::None);
        }
        Self {
            packet_id: packet.packet_id.clone(),
            consumer_surface: packet.consumer_surface,
            subject_ref: packet.subject_ref.clone(),
            readiness_state: packet.readiness_state,
            freshness_class: packet.freshness_class,
            scope_class: packet.scope_class,
            data_lane_lineage: packet.data_lane_lineage,
            drift_indicator: packet.drift_indicator,
            downgrade_label: packet.downgrade_label,
            open_gap_classes,
        }
    }
}

/// Per-consumer-surface summary row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftConsumerSurfaceSummaryRow {
    pub consumer_surface: DriftConsumerSurface,
    pub packet_count: u32,
    pub aligned_count: u32,
    pub drift_count: u32,
    pub fallback_count: u32,
    pub downgrade_required_count: u32,
}

/// Per-lineage summary row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftLineageSummaryRow {
    pub data_lane_lineage: DataLaneLineage,
    pub packet_count: u32,
    pub aligned_count: u32,
    pub drift_count: u32,
    pub downgrade_required_count: u32,
}

/// Metadata-safe graph drift report record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphDriftReport {
    pub schema_version: u32,
    pub record_kind: String,
    pub report_id: String,
    pub captured_at: String,
    pub doc_ref: String,
    pub schema_ref: String,
    pub corpus_manifest_ref: String,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub required_consumer_surfaces: Vec<DriftConsumerSurface>,
    pub required_lineages: Vec<DataLaneLineage>,
    pub matrix_rows: Vec<DriftReportMatrixRow>,
    pub consumer_surface_summaries: Vec<DriftConsumerSurfaceSummaryRow>,
    pub lineage_summaries: Vec<DriftLineageSummaryRow>,
}

impl GraphDriftReport {
    pub fn is_export_safe(&self) -> bool {
        if !self.raw_private_material_excluded || !self.ambient_authority_excluded {
            return false;
        }
        if self.matrix_rows.is_empty() {
            return false;
        }
        if self.consumer_surface_summaries.is_empty() || self.lineage_summaries.is_empty() {
            return false;
        }
        true
    }

    /// Returns a deterministic plaintext rendering of the report,
    /// suitable for the baseline artifact and snapshot tests. The
    /// rendering preserves every closed-vocabulary token surfaces
    /// expose in-product.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "{} {} captured_at={}\n",
            self.record_kind, self.report_id, self.captured_at
        ));
        for row in &self.matrix_rows {
            out.push_str(&format!(
                "row {} surface={} readiness={} freshness={} scope={} lineage={} drift={} downgrade={}\n",
                row.packet_id,
                row.consumer_surface.as_str(),
                row.readiness_state.as_str(),
                row.freshness_class.as_str(),
                row.scope_class.as_str(),
                row.data_lane_lineage.as_str(),
                row.drift_indicator.as_str(),
                row.downgrade_label.as_str(),
            ));
        }
        out
    }
}

/// One validation violation emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphDriftValidationViolation {
    pub check_id: String,
    pub subject_ref: String,
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphDriftValidationReport {
    pub violations: Vec<GraphDriftValidationViolation>,
}

impl fmt::Display for GraphDriftValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} graph drift packet violation(s)",
            self.violations.len()
        )
    }
}

impl Error for GraphDriftValidationReport {}

/// Graph drift packet evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct GraphDriftPacketEvaluator;

impl GraphDriftPacketEvaluator {
    pub const fn new() -> Self {
        Self
    }

    pub fn validate_packet(
        &self,
        packet: &GraphDriftPacket,
    ) -> Result<(), GraphDriftValidationReport> {
        let violations = validate_packet(packet);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(GraphDriftValidationReport { violations })
        }
    }

    pub fn validate_corpus(
        &self,
        corpus: &GraphDriftCorpus,
    ) -> Result<(), GraphDriftValidationReport> {
        let violations = validate_corpus(corpus);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(GraphDriftValidationReport { violations })
        }
    }

    pub fn report(
        &self,
        report_id: impl Into<String>,
        captured_at: impl Into<String>,
        corpus: &GraphDriftCorpus,
    ) -> Result<GraphDriftReport, GraphDriftValidationReport> {
        self.validate_corpus(corpus)?;
        let mut matrix_rows: Vec<DriftReportMatrixRow> = corpus
            .entries
            .iter()
            .map(|entry| DriftReportMatrixRow::from_packet(&entry.packet))
            .collect();
        matrix_rows.sort_by(|a, b| a.packet_id.cmp(&b.packet_id));

        let consumer_surface_summaries = REQUIRED_DRIFT_CONSUMER_SURFACES
            .iter()
            .map(|surface| summarize_consumer_surface(corpus, *surface))
            .collect();
        let lineage_summaries = REQUIRED_DATA_LANE_LINEAGES
            .iter()
            .map(|lineage| summarize_lineage(corpus, *lineage))
            .collect();

        Ok(GraphDriftReport {
            schema_version: GRAPH_DRIFT_PACKET_SCHEMA_VERSION,
            record_kind: GRAPH_DRIFT_REPORT_RECORD_KIND.to_owned(),
            report_id: report_id.into(),
            captured_at: captured_at.into(),
            doc_ref: GRAPH_DRIFT_PACKET_DOC_REF.to_owned(),
            schema_ref: GRAPH_DRIFT_PACKET_SCHEMA_REF.to_owned(),
            corpus_manifest_ref: GRAPH_DRIFT_PACKET_CORPUS_MANIFEST_REF.to_owned(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            required_consumer_surfaces: REQUIRED_DRIFT_CONSUMER_SURFACES.to_vec(),
            required_lineages: REQUIRED_DATA_LANE_LINEAGES.to_vec(),
            matrix_rows,
            consumer_surface_summaries,
            lineage_summaries,
        })
    }
}

fn summarize_consumer_surface(
    corpus: &GraphDriftCorpus,
    surface: DriftConsumerSurface,
) -> DriftConsumerSurfaceSummaryRow {
    let mut row = DriftConsumerSurfaceSummaryRow {
        consumer_surface: surface,
        packet_count: 0,
        aligned_count: 0,
        drift_count: 0,
        fallback_count: 0,
        downgrade_required_count: 0,
    };
    for entry in &corpus.entries {
        if entry.packet.consumer_surface != surface {
            continue;
        }
        row.packet_count += 1;
        if entry.packet.drift_indicator.is_aligned() {
            row.aligned_count += 1;
        } else {
            row.drift_count += 1;
        }
        if entry.packet.drift_indicator == DriftIndicator::FallbackOnly {
            row.fallback_count += 1;
        }
        if !entry.packet.downgrade_label.is_healthy() {
            row.downgrade_required_count += 1;
        }
    }
    row
}

fn summarize_lineage(corpus: &GraphDriftCorpus, lineage: DataLaneLineage) -> DriftLineageSummaryRow {
    let mut row = DriftLineageSummaryRow {
        data_lane_lineage: lineage,
        packet_count: 0,
        aligned_count: 0,
        drift_count: 0,
        downgrade_required_count: 0,
    };
    for entry in &corpus.entries {
        if entry.packet.data_lane_lineage != lineage {
            continue;
        }
        row.packet_count += 1;
        if entry.packet.drift_indicator.is_aligned() {
            row.aligned_count += 1;
        } else {
            row.drift_count += 1;
        }
        if !entry.packet.downgrade_label.is_healthy() {
            row.downgrade_required_count += 1;
        }
    }
    row
}

fn validate_corpus(corpus: &GraphDriftCorpus) -> Vec<GraphDriftValidationViolation> {
    let mut violations = Vec::new();

    if corpus.entries.is_empty() {
        push_violation(
            &mut violations,
            "corpus.empty",
            GRAPH_DRIFT_PACKET_CORPUS_DIR,
            "corpus must contain at least one graph drift packet",
        );
        return violations;
    }

    let mut packet_ids: BTreeSet<String> = BTreeSet::new();
    let mut fixture_refs: BTreeSet<String> = BTreeSet::new();
    let mut seen_surfaces: BTreeSet<DriftConsumerSurface> = BTreeSet::new();
    let mut seen_lineages: BTreeSet<DataLaneLineage> = BTreeSet::new();
    let mut seen_drift_row = false;

    for entry in &corpus.entries {
        if !fixture_refs.insert(entry.fixture_ref.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_fixture_ref",
                &entry.fixture_ref,
                "fixture_ref must be unique within the corpus",
            );
        }
        let packet = &entry.packet;
        if !packet_ids.insert(packet.packet_id.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_packet_id",
                &packet.packet_id,
                "packet_id must be unique within the corpus",
            );
        }
        seen_surfaces.insert(packet.consumer_surface);
        seen_lineages.insert(packet.data_lane_lineage);
        if !packet.drift_indicator.is_aligned() {
            seen_drift_row = true;
        }
        violations.extend(validate_packet(packet));
    }

    for surface in REQUIRED_DRIFT_CONSUMER_SURFACES {
        if !seen_surfaces.contains(&surface) {
            push_violation(
                &mut violations,
                "corpus.required_consumer_surface_missing",
                surface.as_str(),
                format!(
                    "corpus must seed at least one packet for consumer_surface = {}",
                    surface.as_str()
                ),
            );
        }
    }
    for lineage in REQUIRED_DATA_LANE_LINEAGES {
        if !seen_lineages.contains(&lineage) {
            push_violation(
                &mut violations,
                "corpus.required_lineage_missing",
                lineage.as_str(),
                format!(
                    "corpus must seed at least one packet with data_lane_lineage = {}",
                    lineage.as_str()
                ),
            );
        }
    }
    if !seen_drift_row {
        push_violation(
            &mut violations,
            "corpus.no_drift_row",
            GRAPH_DRIFT_PACKET_CORPUS_DIR,
            "corpus must seed at least one packet with a non-aligned drift_indicator so the drift contract is exercised by a fixture",
        );
    }

    violations
}

fn validate_packet(packet: &GraphDriftPacket) -> Vec<GraphDriftValidationViolation> {
    let mut violations = Vec::new();
    let target = packet.packet_id.as_str();

    if packet.schema_version != GRAPH_DRIFT_PACKET_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "packet.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if packet.record_kind != GRAPH_DRIFT_PACKET_RECORD_KIND {
        push_violation(
            &mut violations,
            "packet.record_kind",
            target,
            format!("record_kind must be {GRAPH_DRIFT_PACKET_RECORD_KIND}"),
        );
    }
    for (field, value) in [
        ("packet_id", packet.packet_id.as_str()),
        ("title", packet.title.as_str()),
        ("subject_ref", packet.subject_ref.as_str()),
        ("envelope_packet_ref", packet.envelope_packet_ref.as_str()),
        ("workspace_id", packet.workspace_id.as_str()),
        ("workspace_graph_id", packet.workspace_graph_id.as_str()),
        ("captured_at", packet.captured_at.as_str()),
    ] {
        if value.trim().is_empty() {
            push_violation(
                &mut violations,
                format!("packet.{field}"),
                target,
                format!("{field} must be non-empty"),
            );
        }
    }

    validate_drift_indicator(&mut violations, target, packet);
    validate_outcome_and_downgrade(&mut violations, target, packet);
    validate_evidence_export(&mut violations, target, &packet.evidence_export);
    validate_open_gaps(&mut violations, target, &packet.open_gaps);
    validate_safety(&mut violations, target, &packet.safety);
    validate_references(&mut violations, target, &packet.references);

    violations
}

fn validate_drift_indicator(
    violations: &mut Vec<GraphDriftValidationViolation>,
    target: &str,
    packet: &GraphDriftPacket,
) {
    let derived = derive_drift_indicator(
        packet.readiness_state,
        packet.freshness_class,
        packet.scope_class,
        packet.data_lane_lineage,
    );
    if derived != packet.drift_indicator {
        push_violation(
            violations,
            "packet.drift_indicator.derived_mismatch",
            target,
            format!(
                "drift_indicator must be {} for readiness = {}, freshness = {}, scope = {}, lineage = {}; got {}",
                derived.as_str(),
                packet.readiness_state.as_str(),
                packet.freshness_class.as_str(),
                packet.scope_class.as_str(),
                packet.data_lane_lineage.as_str(),
                packet.drift_indicator.as_str(),
            ),
        );
    }
}

fn validate_outcome_and_downgrade(
    violations: &mut Vec<GraphDriftValidationViolation>,
    target: &str,
    packet: &GraphDriftPacket,
) {
    let healthy = packet.downgrade_label.is_healthy();
    match (packet.drift_indicator.is_aligned(), healthy) {
        (true, false) => {
            push_violation(
                violations,
                "packet.outcome.aligned_must_not_carry_downgrade",
                target,
                "aligned drift_indicator must declare downgrade_label = none",
            );
        }
        (false, true) => {
            push_violation(
                violations,
                "packet.outcome.drift_must_declare_downgrade",
                target,
                "non-aligned drift_indicator must declare a non-none downgrade_label",
            );
        }
        _ => {}
    }
    if !healthy {
        let has_open_gap = packet
            .open_gaps
            .iter()
            .any(|gap| gap.gap_class != DriftOpenGapClass::None);
        if !has_open_gap {
            push_violation(
                violations,
                "packet.outcome.drift_must_record_open_gap",
                target,
                "downgraded packets must record at least one open_gap with a non-none gap_class",
            );
        }
    } else if packet
        .open_gaps
        .iter()
        .any(|gap| gap.gap_class != DriftOpenGapClass::None)
    {
        push_violation(
            violations,
            "packet.outcome.aligned_must_not_record_open_gap",
            target,
            "aligned packets must not declare any open_gap with a non-none gap_class",
        );
    }
    let expected = downgrade_for_indicator(packet.drift_indicator);
    if expected != packet.downgrade_label {
        push_violation(
            violations,
            "packet.outcome.downgrade_label_mismatch",
            target,
            format!(
                "downgrade_label must be {} for drift_indicator = {}; got {}",
                expected.as_str(),
                packet.drift_indicator.as_str(),
                packet.downgrade_label.as_str(),
            ),
        );
    }
}

fn validate_evidence_export(
    violations: &mut Vec<GraphDriftValidationViolation>,
    target: &str,
    export: &DriftEvidenceExportProjection,
) {
    if !export.preserves_readiness_token {
        push_violation(
            violations,
            "packet.evidence_export.preserves_readiness_token",
            target,
            "evidence_export.preserves_readiness_token must be true",
        );
    }
    if !export.preserves_freshness_token {
        push_violation(
            violations,
            "packet.evidence_export.preserves_freshness_token",
            target,
            "evidence_export.preserves_freshness_token must be true",
        );
    }
    if !export.preserves_scope_label {
        push_violation(
            violations,
            "packet.evidence_export.preserves_scope_label",
            target,
            "evidence_export.preserves_scope_label must be true",
        );
    }
    if !export.preserves_lineage_label {
        push_violation(
            violations,
            "packet.evidence_export.preserves_lineage_label",
            target,
            "evidence_export.preserves_lineage_label must be true",
        );
    }
    if !export.preserves_consumer_surface_label {
        push_violation(
            violations,
            "packet.evidence_export.preserves_consumer_surface_label",
            target,
            "evidence_export.preserves_consumer_surface_label must be true",
        );
    }
    if !export.preserves_envelope_packet_ref {
        push_violation(
            violations,
            "packet.evidence_export.preserves_envelope_packet_ref",
            target,
            "evidence_export.preserves_envelope_packet_ref must be true",
        );
    }
    if !export.raw_private_material_excluded {
        push_violation(
            violations,
            "packet.evidence_export.raw_private_material_excluded",
            target,
            "evidence_export.raw_private_material_excluded must be true",
        );
    }
    if !export.ambient_authority_excluded {
        push_violation(
            violations,
            "packet.evidence_export.ambient_authority_excluded",
            target,
            "evidence_export.ambient_authority_excluded must be true",
        );
    }
    if !export.preserves_user_authored_files {
        push_violation(
            violations,
            "packet.evidence_export.preserves_user_authored_files",
            target,
            "evidence_export.preserves_user_authored_files must be true",
        );
    }
}

fn validate_open_gaps(
    violations: &mut Vec<GraphDriftValidationViolation>,
    target: &str,
    gaps: &[DriftOpenGapEntry],
) {
    let mut seen: BTreeSet<DriftOpenGapClass> = BTreeSet::new();
    for gap in gaps {
        if gap.summary.trim().is_empty() {
            push_violation(
                violations,
                "packet.open_gaps.summary",
                target,
                "open_gaps.summary must be non-empty",
            );
        }
        if !seen.insert(gap.gap_class) {
            push_violation(
                violations,
                "packet.open_gaps.duplicate_gap_class",
                target,
                format!("duplicate open_gap_class {}", gap.gap_class.as_str()),
            );
        }
    }
}

fn validate_safety(
    violations: &mut Vec<GraphDriftValidationViolation>,
    target: &str,
    safety: &DriftPacketSafety,
) {
    if !safety.raw_private_material_excluded {
        push_violation(
            violations,
            "packet.safety.raw_private_material_excluded",
            target,
            "raw_private_material_excluded must be true",
        );
    }
    if !safety.ambient_authority_excluded {
        push_violation(
            violations,
            "packet.safety.ambient_authority_excluded",
            target,
            "ambient_authority_excluded must be true",
        );
    }
    if safety.destructive_resets_present {
        push_violation(
            violations,
            "packet.safety.destructive_resets_present",
            target,
            "destructive_resets_present must be false",
        );
    }
    if !safety.preserves_user_authored_files {
        push_violation(
            violations,
            "packet.safety.preserves_user_authored_files",
            target,
            "preserves_user_authored_files must be true",
        );
    }
}

fn validate_references(
    violations: &mut Vec<GraphDriftValidationViolation>,
    target: &str,
    refs: &DriftPacketReferences,
) {
    if refs.doc_ref != GRAPH_DRIFT_PACKET_DOC_REF {
        push_violation(
            violations,
            "packet.references.doc_ref",
            target,
            format!("references.doc_ref must pin {GRAPH_DRIFT_PACKET_DOC_REF}"),
        );
    }
    if refs.schema_ref != GRAPH_DRIFT_PACKET_SCHEMA_REF {
        push_violation(
            violations,
            "packet.references.schema_ref",
            target,
            format!("references.schema_ref must pin {GRAPH_DRIFT_PACKET_SCHEMA_REF}"),
        );
    }
    if refs.report_ref != GRAPH_DRIFT_PACKET_REPORT_REF {
        push_violation(
            violations,
            "packet.references.report_ref",
            target,
            format!("references.report_ref must pin {GRAPH_DRIFT_PACKET_REPORT_REF}"),
        );
    }
}

fn push_violation(
    violations: &mut Vec<GraphDriftValidationViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(GraphDriftValidationViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

fn readiness_from_str(token: &str) -> ReadinessState {
    match token {
        "ready" => ReadinessState::Ready,
        "hot_set_ready" => ReadinessState::HotSetReady,
        "partial" => ReadinessState::Partial,
        "warming" => ReadinessState::Warming,
        "stale" => ReadinessState::Stale,
        "unavailable" => ReadinessState::Unavailable,
        "out_of_scope" => ReadinessState::OutOfScope,
        // Defensive default: unknown readiness collapses onto warming
        // so the drift evaluator never claims live truth from a token
        // it cannot decode.
        _ => ReadinessState::Warming,
    }
}

fn freshness_class_from_token(token: &str) -> FreshnessClass {
    if token == Freshness::Authoritative.as_str() {
        FreshnessClass::Authoritative
    } else if token == Freshness::Warming.as_str() {
        FreshnessClass::Warming
    } else if token == Freshness::Cached.as_str() {
        FreshnessClass::Cached
    } else if token == Freshness::Stale.as_str() {
        FreshnessClass::Stale
    } else if token == Freshness::Replayed.as_str() {
        FreshnessClass::Replayed
    } else if token == Freshness::Imported.as_str() {
        FreshnessClass::Imported
    } else {
        FreshnessClass::Unknown
    }
}

fn freshness_class_from_cue_packet(cue_packet: &GraphFactCuePacket) -> FreshnessClass {
    for cue in &cue_packet.cues {
        if let Some(freshness) = cue.freshness.as_deref() {
            let class = freshness_class_from_token(freshness);
            if class != FreshnessClass::Unknown {
                return class;
            }
        }
    }
    // No graph-backed freshness on any cue; fall back to the readiness
    // token so warming/stale packets keep a non-unknown class.
    match cue_packet.readiness.as_str() {
        "ready" => FreshnessClass::HotSet,
        "hot_set_ready" => FreshnessClass::HotSet,
        "warming" => FreshnessClass::Warming,
        "stale" => FreshnessClass::Stale,
        "unavailable" => FreshnessClass::Unknown,
        _ => FreshnessClass::Unknown,
    }
}

fn lineage_from_cue_packet(cue_packet: &GraphFactCuePacket) -> DataLaneLineage {
    cue_packet
        .truth_lanes
        .iter()
        .copied()
        .map(DataLaneLineage::from_fact_truth_lane)
        .min_by_key(lineage_strength_index)
        .unwrap_or(DataLaneLineage::FallbackSearchLineage)
}

fn lineage_strength_index(lineage: &DataLaneLineage) -> u8 {
    match lineage {
        DataLaneLineage::ExactLocalGraphLineage => 0,
        DataLaneLineage::ImportedProviderLineage => 1,
        DataLaneLineage::InferredDerivedLineage => 2,
        DataLaneLineage::PartialScopeLineage => 3,
        DataLaneLineage::StaleCachedLineage => 4,
        DataLaneLineage::WarmingProviderLineage => 5,
        DataLaneLineage::OutOfScopeLineage => 6,
        DataLaneLineage::FallbackSearchLineage => 7,
    }
}

fn derive_drift_indicator(
    readiness: ReadinessState,
    freshness: FreshnessClass,
    scope: ScopeClass,
    lineage: DataLaneLineage,
) -> DriftIndicator {
    // Lineage and readiness rules first — they are stronger than
    // freshness drift on their own and decide the row's posture.
    if lineage == DataLaneLineage::FallbackSearchLineage {
        return DriftIndicator::FallbackOnly;
    }
    if lineage == DataLaneLineage::OutOfScopeLineage || scope == ScopeClass::OutOfScope {
        return DriftIndicator::BlockedByScope;
    }
    if readiness == ReadinessState::Stale || lineage == DataLaneLineage::StaleCachedLineage {
        return DriftIndicator::StaleWarning;
    }
    if readiness == ReadinessState::Warming
        || readiness == ReadinessState::Unavailable
        || lineage == DataLaneLineage::WarmingProviderLineage
    {
        return DriftIndicator::WarmingWarning;
    }
    if lineage == DataLaneLineage::PartialScopeLineage || !scope.is_full() {
        return DriftIndicator::ScopeSkew;
    }
    if lineage == DataLaneLineage::InferredDerivedLineage
        || lineage == DataLaneLineage::ImportedProviderLineage
    {
        return DriftIndicator::LineageSkew;
    }
    if !freshness.is_live() {
        return DriftIndicator::FreshnessSkew;
    }
    DriftIndicator::Aligned
}

fn downgrade_for_indicator(indicator: DriftIndicator) -> DriftDowngradeLabel {
    match indicator {
        DriftIndicator::Aligned => DriftDowngradeLabel::None,
        DriftIndicator::FreshnessSkew => DriftDowngradeLabel::YellowFreshnessSkew,
        DriftIndicator::ScopeSkew => DriftDowngradeLabel::YellowScopeSkew,
        DriftIndicator::LineageSkew => DriftDowngradeLabel::YellowLineageSkew,
        DriftIndicator::StaleWarning => DriftDowngradeLabel::YellowFreshnessSkew,
        DriftIndicator::WarmingWarning => DriftDowngradeLabel::YellowFreshnessSkew,
        DriftIndicator::BlockedByScope => DriftDowngradeLabel::RedDriftBlocksBetaRow,
        DriftIndicator::FallbackOnly => DriftDowngradeLabel::DegradedToFallbackSearchOnly,
    }
}

fn open_gaps_for_indicator(indicator: DriftIndicator) -> Vec<DriftOpenGapEntry> {
    match indicator {
        DriftIndicator::Aligned => Vec::new(),
        DriftIndicator::FreshnessSkew | DriftIndicator::StaleWarning => {
            vec![DriftOpenGapEntry {
                gap_class: DriftOpenGapClass::FreshnessPending,
                summary: "Graph freshness is behind the live frame; refresh before strong claims."
                    .to_owned(),
            }]
        }
        DriftIndicator::WarmingWarning => vec![DriftOpenGapEntry {
            gap_class: DriftOpenGapClass::FreshnessPending,
            summary: "Graph provider is warming; rows may be incomplete.".to_owned(),
        }],
        DriftIndicator::ScopeSkew => vec![DriftOpenGapEntry {
            gap_class: DriftOpenGapClass::ScopePending,
            summary: "Active scope does not fully cover the subject; widen scope to align."
                .to_owned(),
        }],
        DriftIndicator::LineageSkew => vec![DriftOpenGapEntry {
            gap_class: DriftOpenGapClass::LineagePending,
            summary: "Data-lane lineage is weaker than exact local graph; inspect before use."
                .to_owned(),
        }],
        DriftIndicator::BlockedByScope => vec![DriftOpenGapEntry {
            gap_class: DriftOpenGapClass::DriftBlocked,
            summary: "Subject is outside active scope; drift blocks the beta row.".to_owned(),
        }],
        DriftIndicator::FallbackOnly => vec![DriftOpenGapEntry {
            gap_class: DriftOpenGapClass::FallbackTruthOnly,
            summary: "No graph evidence is available; degrades to fallback search.".to_owned(),
        }],
    }
}

/// Loads a YAML-encoded [`GraphDriftPacket`].
pub fn load_graph_drift_packet(yaml: &str) -> Result<GraphDriftPacket, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Returns the checked-in graph drift corpus loaded from the embedded
/// fixtures.
pub fn current_graph_drift_corpus() -> Result<GraphDriftCorpus, serde_yaml::Error> {
    let entries = PACKET_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<GraphDriftPacket>(yaml).map(|packet| GraphDriftCorpusEntry {
                fixture_ref: (*fixture_ref).to_owned(),
                packet,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(GraphDriftCorpus { entries })
}

/// Returns the set of fixture refs the corpus loads, in declaration
/// order.
pub fn current_graph_drift_fixture_refs() -> impl Iterator<Item = &'static str> {
    PACKET_FIXTURES.iter().map(|(fixture_ref, _)| *fixture_ref)
}

const PACKET_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/graph/m3/drift_packets/navigation_exact_local_aligned_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/drift_packets/navigation_exact_local_aligned_packet.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/drift_packets/review_imported_lineage_skew_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/drift_packets/review_imported_lineage_skew_packet.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/drift_packets/ai_context_inferred_lineage_skew_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/drift_packets/ai_context_inferred_lineage_skew_packet.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/drift_packets/support_export_partial_scope_skew_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/drift_packets/support_export_partial_scope_skew_packet.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/drift_packets/support_export_stale_warning_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/drift_packets/support_export_stale_warning_packet.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/drift_packets/navigation_warming_warning_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/drift_packets/navigation_warming_warning_packet.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/drift_packets/navigation_out_of_scope_blocked_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/drift_packets/navigation_out_of_scope_blocked_packet.yaml"
        )),
    ),
    (
        "fixtures/graph/m3/drift_packets/ai_context_fallback_only_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/graph/m3/drift_packets/ai_context_fallback_only_packet.yaml"
        )),
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::readiness::{GraphCueSurface, GraphFactCuePacket};

    fn aligned_navigation_packet() -> GraphDriftPacket {
        GraphDriftPacket {
            schema_version: GRAPH_DRIFT_PACKET_SCHEMA_VERSION,
            record_kind: GRAPH_DRIFT_PACKET_RECORD_KIND.to_owned(),
            packet_id: "packet:test:aligned".to_owned(),
            title: "Aligned test packet".to_owned(),
            consumer_surface: DriftConsumerSurface::Navigation,
            subject_ref: "graph:symbol:test".to_owned(),
            envelope_packet_ref: "envelope:test".to_owned(),
            workspace_id: "workspace:test".to_owned(),
            workspace_graph_id: "graph:test".to_owned(),
            readiness_state: ReadinessState::Ready,
            freshness_class: FreshnessClass::Authoritative,
            scope_class: ScopeClass::FullLocal,
            data_lane_lineage: DataLaneLineage::ExactLocalGraphLineage,
            drift_indicator: DriftIndicator::Aligned,
            evidence_export: DriftEvidenceExportProjection::metadata_safe_baseline(),
            downgrade_label: DriftDowngradeLabel::None,
            open_gaps: Vec::new(),
            safety: DriftPacketSafety::metadata_safe_baseline(),
            references: DriftPacketReferences::pinned(),
            captured_at: "2026-05-16T00:00:00Z".to_owned(),
            reviewer_summary: None,
        }
    }

    #[test]
    fn aligned_packet_validates() {
        GraphDriftPacketEvaluator::new()
            .validate_packet(&aligned_navigation_packet())
            .expect("aligned packet must validate");
    }

    #[test]
    fn refuses_aligned_with_downgrade() {
        let mut packet = aligned_navigation_packet();
        packet.downgrade_label = DriftDowngradeLabel::YellowFreshnessSkew;
        let err = GraphDriftPacketEvaluator::new()
            .validate_packet(&packet)
            .expect_err("aligned with downgrade must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "packet.outcome.aligned_must_not_carry_downgrade"));
    }

    #[test]
    fn refuses_drift_indicator_mismatch() {
        let mut packet = aligned_navigation_packet();
        // Lineage says stale but caller claimed aligned.
        packet.data_lane_lineage = DataLaneLineage::StaleCachedLineage;
        let err = GraphDriftPacketEvaluator::new()
            .validate_packet(&packet)
            .expect_err("mismatch must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "packet.drift_indicator.derived_mismatch"));
    }

    #[test]
    fn refuses_destructive_reset() {
        let mut packet = aligned_navigation_packet();
        packet.safety.destructive_resets_present = true;
        let err = GraphDriftPacketEvaluator::new()
            .validate_packet(&packet)
            .expect_err("destructive reset must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "packet.safety.destructive_resets_present"));
    }

    #[test]
    fn fallback_cue_compiles_into_fallback_only_packet() {
        let cue_packet = GraphFactCuePacket::from_fallback_search(
            "packet:fallback",
            GraphCueSurface::AiContext,
            "request:test",
            "workspace:test",
            "fallback:search:abc",
            "partial",
            "2026-05-16T00:00:00Z",
        );
        let packet = GraphDriftPacket::compile_from_alpha(GraphDriftCompileInput {
            packet_id: "packet:fallback:drift".to_owned(),
            title: "Fallback-only drift packet".to_owned(),
            subject_ref: "graph:symbol:test".to_owned(),
            scope_class: ScopeClass::FullLocal,
            cue_packet: &cue_packet,
            captured_at: "2026-05-16T00:00:00Z".to_owned(),
            reviewer_summary: None,
        });
        assert_eq!(packet.drift_indicator, DriftIndicator::FallbackOnly);
        assert_eq!(
            packet.downgrade_label,
            DriftDowngradeLabel::DegradedToFallbackSearchOnly
        );
        GraphDriftPacketEvaluator::new()
            .validate_packet(&packet)
            .expect("compiled fallback packet must validate");
    }

    #[test]
    fn refuses_corpus_without_drift_row() {
        let corpus = GraphDriftCorpus {
            entries: vec![GraphDriftCorpusEntry {
                fixture_ref: "fixtures/test/only_aligned.yaml".to_owned(),
                packet: aligned_navigation_packet(),
            }],
        };
        let err = GraphDriftPacketEvaluator::new()
            .validate_corpus(&corpus)
            .expect_err("corpus without drift row must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "corpus.no_drift_row"));
    }

    #[test]
    fn checked_in_corpus_loads_and_validates() {
        let corpus = current_graph_drift_corpus().expect("checked-in corpus must parse");
        GraphDriftPacketEvaluator::new()
            .validate_corpus(&corpus)
            .expect("checked-in corpus must validate");
        for surface in REQUIRED_DRIFT_CONSUMER_SURFACES {
            assert!(
                corpus
                    .entries
                    .iter()
                    .any(|entry| entry.packet.consumer_surface == surface),
                "checked-in corpus must seed a packet for consumer_surface = {}",
                surface.as_str()
            );
        }
        for lineage in REQUIRED_DATA_LANE_LINEAGES {
            assert!(
                corpus
                    .entries
                    .iter()
                    .any(|entry| entry.packet.data_lane_lineage == lineage),
                "checked-in corpus must seed a packet with data_lane_lineage = {}",
                lineage.as_str()
            );
        }
    }

    #[test]
    fn report_is_export_safe() {
        let corpus = current_graph_drift_corpus().unwrap();
        let report = GraphDriftPacketEvaluator::new()
            .report("report:test", "2026-05-16T00:00:00Z", &corpus)
            .expect("report builds");
        assert!(report.is_export_safe());
        assert_eq!(report.matrix_rows.len(), corpus.entries.len());
        assert_eq!(
            report.lineage_summaries.len(),
            REQUIRED_DATA_LANE_LINEAGES.len()
        );
        assert_eq!(
            report.consumer_surface_summaries.len(),
            REQUIRED_DRIFT_CONSUMER_SURFACES.len()
        );
    }
}
