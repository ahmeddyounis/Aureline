//! Semantic graph storage and alpha query-family runtime.
//!
//! This crate is the first runtime consumer of the graph seed object model. It
//! stores validated [`aureline_graph_proto::WorkspaceGraph`] snapshots and
//! exposes a bounded query-family surface for launch-wedge symbols, imports,
//! ownership edges, and the future impact/explainer packet lanes.

pub mod explainers;
pub mod journey_budget;
mod query;
pub mod readiness;
mod store;

pub use explainers::{
    EvidenceCard, EvidenceCitation, ExplainerSourceKind, GraphExplainerError,
    ImpactEdgeEvidenceClass, ImpactExplainerPacket, ImpactSummary, IndexCoverage,
    NonCanvasTopologyFallback, OpenDetailAction, OpenDetailActionClass, TopologyEdgeProjection,
    TopologyFallbackRow, TopologyNodeProjection, VisualTopologyProjection, WorksetScopeDescriptor,
    WorksetScopeMode, WorksetScopeSource, GRAPH_IMPACT_EXPLAINER_PACKET_RECORD_KIND,
    GRAPH_IMPACT_EXPLAINER_PACKET_SCHEMA_VERSION,
};
pub use journey_budget::{BudgetOverrun, BudgetUnit, ConsumedRecord, JourneyId, LedgerRollup};
pub use query::{
    result_partiality_for_readiness, GraphAlphaQueryClass, GraphPartialTruthCause,
    GraphQueryDowngradeReason, GraphQueryEnvelope, GraphQueryFamilyDescriptor, GraphQueryReadiness,
    GraphQueryRequest, GraphQueryRow, GraphQueryRowClass, ResultPartialityClass,
    GRAPH_QUERY_FAMILY_ALPHA_VERSION,
};
pub use readiness::{
    GraphCueActionPosture, GraphCueSurface, GraphFactCue, GraphFactCuePacket, GraphFactTruthLane,
    GRAPH_FACT_CUE_PACKET_RECORD_KIND, GRAPH_FACT_CUE_SCHEMA_VERSION,
};
pub use readiness::beta::{
    current_graph_readiness_beta_corpus, current_graph_readiness_beta_fixture_refs,
    load_graph_readiness_beta_case, BetaConsumerSurface, CaseReferences as BetaCaseReferences,
    CaseSafety as BetaCaseSafety, ClaimAlignmentState, ConsumerSurfaceSummaryRow, DowngradeLabel,
    EvidenceExportProjection, FactLane, FactLaneSummaryRow, GraphReadinessBetaCase,
    GraphReadinessBetaCorpus, GraphReadinessBetaCorpusEntry, GraphReadinessBetaEvaluator,
    GraphReadinessBetaReport, GraphReadinessBetaValidationReport, GraphReadinessBetaViolation,
    OpenGapClass as BetaOpenGapClass, OpenGapEntry as BetaOpenGapEntry, ReadinessClaim,
    ReportMatrixRow as BetaReportMatrixRow, GRAPH_READINESS_BETA_CASE_RECORD_KIND,
    GRAPH_READINESS_BETA_CORPUS_DIR, GRAPH_READINESS_BETA_CORPUS_MANIFEST_REF,
    GRAPH_READINESS_BETA_DOC_REF, GRAPH_READINESS_BETA_REPORT_RECORD_KIND,
    GRAPH_READINESS_BETA_REPORT_REF, GRAPH_READINESS_BETA_SCHEMA_REF,
    GRAPH_READINESS_BETA_SCHEMA_VERSION, REQUIRED_CONSUMER_SURFACES, REQUIRED_FACT_LANES,
};
pub use store::{GraphStore, GraphStoreError};

pub use aureline_graph_proto::{
    ConfidenceLevel, EdgeClass, EdgeEvidenceState, Freshness, FreshnessFrame, GraphEdge, GraphNode,
    NodeBody, NodeClass, QueryFamilyTag, WorksetScopeRef,
};
