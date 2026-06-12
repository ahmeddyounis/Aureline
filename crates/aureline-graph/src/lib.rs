//! Semantic graph storage and alpha query-family runtime.
//!
//! This crate is the first runtime consumer of the graph seed object model. It
//! stores validated [`aureline_graph_proto::WorkspaceGraph`] snapshots and
//! exposes a bounded query-family surface for launch-wedge symbols, imports,
//! ownership edges, and the future impact/explainer packet lanes.
//! Graph-backed navigation projections use
//! [`aureline_navigation::target_model`] for relation, proof, freshness,
//! ambiguity, and scope-completeness truth.

pub mod audit_topology_explainer_companion_truth_packet;
pub mod drift_packets;
pub mod explainers;
pub mod freshness_propagation_packet;
pub mod journey_budget;
pub mod knowledge_evidence_packet;
pub mod m5_graph_governance;
pub mod m5_workset_scope;
pub mod navigation_target_truth_packet;
pub mod public_proof_publication_truth_packet;
mod query;
pub mod readiness;
pub mod scope_provenance_truth_packet;
pub mod semantic_graph_object_model_and_query_contract;
mod store;
pub mod support_export_parity_truth_packet;

pub use audit_topology_explainer_companion_truth_packet::{
    current_stable_audit_topology_explainer_companion_truth_packet, AuditConfidenceClass,
    AuditConsumerProjection, AuditConsumerSurface, AuditFindingKind, AuditFindingSeverity,
    AuditPillar, AuditPromotionState, AuditRow, AuditRowClass, AuditSurfaceClass,
    AuditTopologyExplainerCompanionTruthArtifactError, AuditTopologyExplainerCompanionTruthPacket,
    AuditTopologyExplainerCompanionTruthPacketInput,
    AuditTopologyExplainerCompanionTruthSupportExport, AuditValidationFinding,
    DowngradeStateDisclosureClass, FreshnessDisclosureClass, ProvenanceDisclosureClass,
    QualificationState, ScopeDisclosureClass,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_ARTIFACT_DOC_REF,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_DOC_REF,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_FIXTURE_DIR,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_PACKET_ARTIFACT_REF,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_PACKET_RECORD_KIND,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SCHEMA_REF,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SCHEMA_VERSION,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use drift_packets::{
    current_graph_drift_corpus, current_graph_drift_fixture_refs, load_graph_drift_packet,
    DataLaneLineage, DriftConsumerSurface, DriftConsumerSurfaceSummaryRow, DriftDowngradeLabel,
    DriftEvidenceExportProjection, DriftIndicator, DriftLineageSummaryRow, DriftOpenGapClass,
    DriftOpenGapEntry, DriftPacketReferences, DriftPacketSafety, DriftReportMatrixRow,
    FreshnessClass, GraphDriftCompileInput, GraphDriftCorpus, GraphDriftCorpusEntry,
    GraphDriftPacket, GraphDriftPacketEvaluator, GraphDriftReport, GraphDriftValidationReport,
    GraphDriftValidationViolation, ReadinessState, ScopeClass, GRAPH_DRIFT_PACKET_CORPUS_DIR,
    GRAPH_DRIFT_PACKET_CORPUS_MANIFEST_REF, GRAPH_DRIFT_PACKET_DOC_REF,
    GRAPH_DRIFT_PACKET_RECORD_KIND, GRAPH_DRIFT_PACKET_REPORT_REF, GRAPH_DRIFT_PACKET_SCHEMA_REF,
    GRAPH_DRIFT_PACKET_SCHEMA_VERSION, GRAPH_DRIFT_REPORT_RECORD_KIND, REQUIRED_DATA_LANE_LINEAGES,
    REQUIRED_DRIFT_CONSUMER_SURFACES,
};
pub use explainers::{
    EvidenceCard, EvidenceCitation, ExplainerSourceKind, GraphExplainerError,
    ImpactEdgeEvidenceClass, ImpactExplainerPacket, ImpactSummary, IndexCoverage,
    NonCanvasTopologyFallback, OpenDetailAction, OpenDetailActionClass, TopologyEdgeProjection,
    TopologyFallbackRow, TopologyNodeProjection, VisualTopologyProjection, WorksetScopeDescriptor,
    WorksetScopeMode, WorksetScopeSource, GRAPH_IMPACT_EXPLAINER_PACKET_RECORD_KIND,
    GRAPH_IMPACT_EXPLAINER_PACKET_SCHEMA_VERSION,
};
pub use freshness_propagation_packet::{
    current_stable_freshness_propagation_packet,
    CapturedVsLiveClass as PropagationCapturedVsLiveClass,
    ConfidenceClass as PropagationConfidenceClass, EpochLabel,
    FreshnessClass as PropagationFreshnessClass, FreshnessPropagationArtifactError,
    FreshnessPropagationConsumerProjection, FreshnessPropagationConsumerSurface,
    FreshnessPropagationFindingKind, FreshnessPropagationFindingSeverity,
    FreshnessPropagationPacket, FreshnessPropagationPacketInput,
    FreshnessPropagationPacketSupportExport, FreshnessPropagationPromotionState,
    FreshnessPropagationRow, FreshnessPropagationValidationFinding, GraphEpochClass, GraphHandle,
    GraphHandleClass, HiddenGraphDependencyDisclosure, HiddenGraphDependencyState,
    InvalidationScope, InvalidationScopeClass, MixedEpochDisclosure,
    RetentionClass as PropagationRetentionClass, VisibilityScopeClass,
    FRESHNESS_PROPAGATION_PACKET_ARTIFACT_DOC_REF, FRESHNESS_PROPAGATION_PACKET_ARTIFACT_REF,
    FRESHNESS_PROPAGATION_PACKET_DOC_REF, FRESHNESS_PROPAGATION_PACKET_FIXTURE_DIR,
    FRESHNESS_PROPAGATION_PACKET_RECORD_KIND, FRESHNESS_PROPAGATION_PACKET_SCHEMA_VERSION,
    FRESHNESS_PROPAGATION_PACKET_SUPPORT_EXPORT_RECORD_KIND,
};
pub use journey_budget::{BudgetOverrun, BudgetUnit, ConsumedRecord, JourneyId, LedgerRollup};
pub use knowledge_evidence_packet::{
    current_stable_knowledge_evidence_packet, EvidenceOpeningAction, EvidenceOpeningActionClass,
    ExplainerCitation, ExplainerSnapshot, ExplainerSourceClass, ImpactCard,
    KnowledgeConsumerProjection, KnowledgeConsumerSurface, KnowledgeEvidenceArtifactError,
    KnowledgeEvidencePacket, KnowledgeEvidencePacketInput, KnowledgeEvidencePacketSupportExport,
    KnowledgeFindingKind, KnowledgeFindingSeverity, KnowledgePromotionState,
    KnowledgeValidationFinding, NoImpactState, OwnershipCard, OwnershipClass, SharedIdentityModel,
    TopologyEdge as KnowledgeTopologyEdge, TopologyNode as KnowledgeTopologyNode,
    TopologyView as KnowledgeTopologyView, KNOWLEDGE_EVIDENCE_PACKET_ARTIFACT_DOC_REF,
    KNOWLEDGE_EVIDENCE_PACKET_ARTIFACT_REF, KNOWLEDGE_EVIDENCE_PACKET_DOC_REF,
    KNOWLEDGE_EVIDENCE_PACKET_FIXTURE_DIR, KNOWLEDGE_EVIDENCE_PACKET_RECORD_KIND,
    KNOWLEDGE_EVIDENCE_PACKET_SCHEMA_REF, KNOWLEDGE_EVIDENCE_PACKET_SCHEMA_VERSION,
    KNOWLEDGE_EVIDENCE_PACKET_SUPPORT_EXPORT_RECORD_KIND,
};
pub use m5_graph_governance::{
    current_m5_graph_governance_matrix, DowngradePath as GraphGovernanceDowngradePath,
    DowngradeReason as GraphGovernanceDowngradeReason, EvidenceBacking, GraphDepthClaim,
    GraphDepthLane, GraphFreshness as GraphGovernanceFreshness, GraphGovernanceDecision,
    GraphGovernanceRow, ImpactResultClass, M5GraphGovernanceExportProjection,
    M5GraphGovernanceExportRow, M5GraphGovernanceMatrix, M5GraphGovernanceSummary,
    M5GraphGovernanceViolation, RelationFidelity, ScopeMode, M5_GRAPH_GOVERNANCE_DOC_REF,
    M5_GRAPH_GOVERNANCE_FIXTURE_DIR, M5_GRAPH_GOVERNANCE_JSON, M5_GRAPH_GOVERNANCE_PATH,
    M5_GRAPH_GOVERNANCE_RECORD_KIND, M5_GRAPH_GOVERNANCE_SCHEMA_REF,
    M5_GRAPH_GOVERNANCE_SCHEMA_VERSION,
};
pub use m5_workset_scope::{
    current_m5_workset_scope_packet, M5WorksetScopeExportProjection, M5WorksetScopeExportRow,
    M5WorksetScopePacket, M5WorksetScopeSummary, M5WorksetScopeViolation, ScopeChangeAction,
    ScopeChangeActuation, ScopeChangeDirection, ScopeConsumerBinding, WorksetScopeConsumerSurface,
    WorksetScopeSnapshot, M5_WORKSET_SCOPE_DOC_REF, M5_WORKSET_SCOPE_FIXTURE_DIR,
    M5_WORKSET_SCOPE_GOVERNANCE_MATRIX_REF, M5_WORKSET_SCOPE_JSON, M5_WORKSET_SCOPE_PATH,
    M5_WORKSET_SCOPE_RECORD_KIND, M5_WORKSET_SCOPE_SCHEMA_REF, M5_WORKSET_SCOPE_SCHEMA_VERSION,
    M5_WORKSET_SCOPE_SOURCE_PACKET_REF,
};
pub use navigation_target_truth_packet::{
    current_stable_navigation_target_truth_packet, AccessKind as NavigationAccessKind,
    AliasingContext as NavigationAliasingContext, AmbiguityClass as NavigationAmbiguityClass,
    ConfidenceClass as NavigationTargetConfidenceClass,
    ConsumerSurface as NavigationTargetConsumerSurface,
    DowngradeState as NavigationTargetDowngradeState, FindingKind as NavigationTargetFindingKind,
    FindingSeverity as NavigationTargetFindingSeverity,
    FreshnessClass as NavigationTargetFreshnessClass,
    HierarchyEdgeContext as NavigationHierarchyEdgeContext,
    HierarchyEdgeKind as NavigationHierarchyEdgeKind, NavigationTargetConsumerProjection,
    NavigationTargetRow, NavigationTargetTruthArtifactError, NavigationTargetTruthPacket,
    NavigationTargetTruthPacketInput, NavigationTargetTruthSupportExport,
    PromotionState as NavigationTargetPromotionState,
    ProviderClass as NavigationTargetProviderClass, ReferenceContext as NavigationReferenceContext,
    RelationKind as NavigationRelationKind, RenameBlockedReason as NavigationRenameBlockedReason,
    RenamePreviewContext as NavigationRenamePreviewContext, RowClass as NavigationTargetRowClass,
    ScopeCompleteness as NavigationTargetScopeCompleteness,
    ValidationFinding as NavigationTargetValidationFinding,
    NAVIGATION_TARGET_TRUTH_ARTIFACT_DOC_REF, NAVIGATION_TARGET_TRUTH_DOC_REF,
    NAVIGATION_TARGET_TRUTH_FIXTURE_DIR, NAVIGATION_TARGET_TRUTH_PACKET_ARTIFACT_REF,
    NAVIGATION_TARGET_TRUTH_PACKET_RECORD_KIND, NAVIGATION_TARGET_TRUTH_SCHEMA_REF,
    NAVIGATION_TARGET_TRUTH_SCHEMA_VERSION, NAVIGATION_TARGET_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use public_proof_publication_truth_packet::{
    current_stable_public_proof_publication_truth_packet,
    ConsumerSurface as PublicProofPublicationConsumerSurface,
    DowngradeAutomationClass as PublicProofDowngradeAutomationClass,
    FindingKind as PublicProofPublicationFindingKind,
    FindingSeverity as PublicProofPublicationFindingSeverity, KnownLimitClass,
    PromotionState as PublicProofPublicationPromotionState, ProofArtifactClass,
    PublicProofPublicationConsumerProjection, PublicProofPublicationRow,
    PublicProofPublicationTruthArtifactError, PublicProofPublicationTruthPacket,
    PublicProofPublicationTruthPacketInput, PublicProofPublicationTruthSupportExport,
    PublicationConfidenceClass, PublicationLaneClass, PublicationRowClass, PublicationState,
    ValidationFinding as PublicProofPublicationValidationFinding,
    PUBLIC_PROOF_PUBLICATION_TRUTH_ARTIFACT_DOC_REF, PUBLIC_PROOF_PUBLICATION_TRUTH_DOC_REF,
    PUBLIC_PROOF_PUBLICATION_TRUTH_FIXTURE_DIR, PUBLIC_PROOF_PUBLICATION_TRUTH_PACKET_ARTIFACT_REF,
    PUBLIC_PROOF_PUBLICATION_TRUTH_PACKET_RECORD_KIND, PUBLIC_PROOF_PUBLICATION_TRUTH_SCHEMA_REF,
    PUBLIC_PROOF_PUBLICATION_TRUTH_SCHEMA_VERSION,
    PUBLIC_PROOF_PUBLICATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use query::{
    result_partiality_for_readiness, GraphAlphaQueryClass, GraphPartialTruthCause,
    GraphQueryDowngradeReason, GraphQueryEnvelope, GraphQueryFamilyDescriptor, GraphQueryReadiness,
    GraphQueryRequest, GraphQueryRow, GraphQueryRowClass, ResultPartialityClass,
    GRAPH_QUERY_FAMILY_ALPHA_VERSION,
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
pub use readiness::{
    GraphCueActionPosture, GraphCueSurface, GraphFactCue, GraphFactCuePacket, GraphFactTruthLane,
    GRAPH_FACT_CUE_PACKET_RECORD_KIND, GRAPH_FACT_CUE_SCHEMA_VERSION,
};
pub use scope_provenance_truth_packet::{
    current_stable_scope_provenance_truth_packet, ArchivedContext,
    ConfidenceClass as ScopeProvenanceConfidenceClass,
    ConsumerSurface as ScopeProvenanceConsumerSurface,
    DowngradeState as ScopeProvenanceDowngradeState, FindingKind as ScopeProvenanceFindingKind,
    FindingSeverity as ScopeProvenanceFindingSeverity,
    FreshnessClass as ScopeProvenanceFreshnessClass, HiddenScopeContext, ImportedMapping,
    ImportedOutcomeLabel, ItemClass as ScopeProvenanceItemClass, PartialScopeContext,
    PromotionState as ScopeProvenancePromotionState,
    ProvenanceClass as ScopeProvenanceProvenanceClass, ScopeProvenanceConsumerProjection,
    ScopeProvenanceRow, ScopeProvenanceTruthArtifactError, ScopeProvenanceTruthPacket,
    ScopeProvenanceTruthPacketInput, ScopeProvenanceTruthSupportExport,
    SurfaceClass as ScopeProvenanceSurfaceClass,
    ValidationFinding as ScopeProvenanceValidationFinding, SCOPE_PROVENANCE_TRUTH_ARTIFACT_DOC_REF,
    SCOPE_PROVENANCE_TRUTH_DOC_REF, SCOPE_PROVENANCE_TRUTH_FIXTURE_DIR,
    SCOPE_PROVENANCE_TRUTH_PACKET_ARTIFACT_REF, SCOPE_PROVENANCE_TRUTH_PACKET_RECORD_KIND,
    SCOPE_PROVENANCE_TRUTH_SCHEMA_REF, SCOPE_PROVENANCE_TRUTH_SCHEMA_VERSION,
    SCOPE_PROVENANCE_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use semantic_graph_object_model_and_query_contract::{
    current_stable_semantic_graph_contract_packet, seeded_semantic_graph_contract_input,
    GraphConfidenceTier as SemanticGraphConfidenceTier, GraphConsumerActionBinding,
    GraphConsumerSurface, GraphFreshnessState as SemanticGraphFreshnessState,
    GraphInvalidationClass, GraphInvalidationEvent, GraphProducerIdentity,
    GraphRetentionClass as SemanticGraphRetentionClass,
    GraphStableClaimLevel as SemanticGraphStableClaimLevel, GraphSupportReconstructionProjection,
    GraphTopologyReuseProof, GraphVisibilityScope as SemanticGraphVisibilityScope,
    SemanticGraphContractArtifactError, SemanticGraphContractFindingKind,
    SemanticGraphContractFindingSeverity, SemanticGraphContractPacket,
    SemanticGraphContractPacketInput, SemanticGraphContractPromotionState,
    SemanticGraphContractSupportExport, SemanticGraphContractValidationFinding, StableGraphObject,
    StableGraphObjectKind, StableGraphQueryHandle, StableQueryFamily,
    SEMANTIC_GRAPH_CONTRACT_ARTIFACT_DOC_REF, SEMANTIC_GRAPH_CONTRACT_DOC_REF,
    SEMANTIC_GRAPH_CONTRACT_FIXTURE_DIR, SEMANTIC_GRAPH_CONTRACT_PACKET_ARTIFACT_REF,
    SEMANTIC_GRAPH_CONTRACT_PACKET_RECORD_KIND, SEMANTIC_GRAPH_CONTRACT_SCHEMA_REF,
    SEMANTIC_GRAPH_CONTRACT_SCHEMA_VERSION, SEMANTIC_GRAPH_CONTRACT_SUPPORT_EXPORT_RECORD_KIND,
};
pub use store::{GraphStore, GraphStoreError};
pub use support_export_parity_truth_packet::{
    current_stable_support_export_parity_truth_packet,
    ConfidenceClass as SupportExportParityConfidenceClass,
    ConsumerSurface as SupportExportParityConsumerSurface, CountSummary, DeepLinkScopeBinding,
    DowngradeState as SupportExportParityDowngradeState, ExportPacketClass,
    FindingKind as SupportExportParityFindingKind,
    FindingSeverity as SupportExportParityFindingSeverity,
    LaneClass as SupportExportParityLaneClass,
    LiveVsCapturedClass as SupportExportParityLiveVsCapturedClass, OperatorReconstructionProof,
    PromotionState as SupportExportParityPromotionState,
    RedactionClass as SupportExportParityRedactionClass, SupportExportParityConsumerProjection,
    SupportExportParityRow, SupportExportParityTruthArtifactError, SupportExportParityTruthPacket,
    SupportExportParityTruthPacketInput, SupportExportParityTruthSupportExport,
    ValidationFinding as SupportExportParityValidationFinding,
    SUPPORT_EXPORT_PARITY_TRUTH_ARTIFACT_DOC_REF, SUPPORT_EXPORT_PARITY_TRUTH_DOC_REF,
    SUPPORT_EXPORT_PARITY_TRUTH_FIXTURE_DIR, SUPPORT_EXPORT_PARITY_TRUTH_PACKET_ARTIFACT_REF,
    SUPPORT_EXPORT_PARITY_TRUTH_PACKET_RECORD_KIND, SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_REF,
    SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_VERSION,
    SUPPORT_EXPORT_PARITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};

pub use aureline_graph_proto::{
    ConfidenceLevel, EdgeClass, EdgeEvidenceState, Freshness, FreshnessFrame, GraphEdge, GraphNode,
    NodeBody, NodeClass, QueryFamilyTag, WorksetScopeRef,
};
pub use aureline_navigation::target_model as navigation_target_model;
