//! Local-first data and experiment provenance contracts.
//!
//! This crate owns the typed records that keep notebook-adjacent experiment
//! runs, dataset provenance, generated artifacts, environment fingerprints,
//! comparisons, and exports attributable without depending on a hosted tracker.
//! It also owns the promoted-build database qualification packet that keeps
//! connection truth, statement safety, query history, result-grid/export truth,
//! explain-plan freshness, and data handoffs from inheriting generic table or
//! notebook credibility. The experiment-provenance boundary schema is
//! [`/schemas/data/experiment-provenance.schema.json`](../../../schemas/data/experiment-provenance.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/qualify-experiment-provenance-and-result-comparison.json`](../../../artifacts/data/qualify-experiment-provenance-and-result-comparison.json).
//!
//! Raw dataset rows, raw artifact payloads, raw secrets, raw hostnames, and raw
//! URLs do not belong in these records. They carry stable IDs, closed posture
//! vocabularies, and reviewable summaries that UI, CLI, export, support, and
//! public-proof surfaces can ingest safely.

#![doc(html_root_url = "https://docs.rs/aureline-data/0.0.0")]

pub mod database_qualification;
pub mod experiment_provenance;

pub use database_qualification::{
    current_database_qualification, DataToolingSurfaceKind, DatabaseAuthSourceMode,
    DatabaseConnectionClass, DatabaseConnectionCorpusRow, DatabaseExecutionOrigin,
    DatabaseExplainPlanLabRow, DatabaseExplainPlanMode, DatabaseHandoffLabRow,
    DatabaseQualificationLabel, DatabaseQualificationPacket, DatabaseQualificationProof,
    DatabaseQualificationSummary, DatabaseQualificationViolation,
    DatabaseQualificationViolationKind, DatabaseRedactionMode, DatabaseResultGridLabRow,
    DatabaseResultScope, DatabaseStatementSafetyClass, DatabaseStatementSafetyLabRow,
    DatabaseSurfaceGuardSet, DatabaseSurfaceQualificationRow, DatabaseTransactionPosture,
    DatabaseWritePosture, DATABASE_QUALIFICATION_PACKET_JSON, DATABASE_QUALIFICATION_PACKET_PATH,
    DATABASE_QUALIFICATION_RECORD_KIND, DATABASE_QUALIFICATION_SCHEMA_VERSION,
};
pub use experiment_provenance::{
    current_experiment_provenance_qualification, ArtifactKind, ArtifactLineageEntry,
    ArtifactLineageState, ComparisonAxisState, ComparisonBasis, ComparisonGuardBanner,
    ComparisonMetricRow, DatasetScopeClass, DatasetSensitivityState, DatasetSourceClass,
    DatasetSummary, EnvironmentFingerprint, EnvironmentFreshnessClass, ExperimentOriginClass,
    ExperimentProvenancePacket, ExperimentProvenanceSummary, ExperimentProvenanceViolation,
    ExperimentRun, ExportPayloadScope, ExportReview, ExportTrustClass, LocationClass, OutcomeClass,
    ReproducibilityLabel, ResultComparisonRow, SourceKind, EXPERIMENT_PROVENANCE_PACKET_JSON,
    EXPERIMENT_PROVENANCE_PACKET_PATH, EXPERIMENT_PROVENANCE_RECORD_KIND,
    EXPERIMENT_PROVENANCE_SCHEMA_VERSION,
};
