//! Experiment run, dataset, lineage, comparison, and export provenance records.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for experiment provenance packets.
pub const EXPERIMENT_PROVENANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ExperimentProvenancePacket`].
pub const EXPERIMENT_PROVENANCE_RECORD_KIND: &str = "experiment_provenance_packet";

/// Repo-relative path to the checked-in qualification packet.
pub const EXPERIMENT_PROVENANCE_PACKET_PATH: &str =
    "artifacts/data/qualify-experiment-provenance-and-result-comparison.json";

/// Embedded checked-in packet JSON.
pub const EXPERIMENT_PROVENANCE_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/qualify-experiment-provenance-and-result-comparison.json"
));

/// Origin class for an experiment run or manually attached artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentOriginClass {
    /// Run was produced by local execution.
    Local,
    /// Run was produced by a managed workspace or managed service.
    Managed,
    /// Run was imported from another tool, packet, or tracker.
    Imported,
    /// Artifact or result was attached manually and is not a recoverable run.
    ManualAttach,
}

/// Source family that produced an experiment run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    /// Notebook document or selected notebook cells.
    Notebook,
    /// Script or module launched outside a notebook.
    Script,
    /// Task runner entry.
    Task,
    /// Test or benchmark runner entry.
    Test,
    /// Imported or manually attached evidence.
    ImportedEvidence,
}

/// Outcome class for an experiment run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeClass {
    /// Declared success condition completed.
    Success,
    /// Declared failure condition completed.
    Failure,
    /// Run completed with usable partial output.
    Partial,
    /// Run was cancelled by user, policy, or runtime.
    Cancelled,
    /// Outcome is unknown because lineage was imported or incomplete.
    Unknown,
}

/// Reproducibility label shown before compare, open, or export actions are trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReproducibilityLabel {
    /// Evidence supports a direct rerun under the same context.
    Reproducible,
    /// Most identity fields match, but exact repeatability is not guaranteed.
    LikelyReproducible,
    /// A fresh rerun is required before trusting current deltas.
    NeedsRerun,
    /// Required run, data, or environment context is incomplete.
    ContextIncomplete,
}

/// Source class for a dataset summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatasetSourceClass {
    /// Local file or local workspace path.
    LocalFile,
    /// Database or warehouse query.
    DatabaseQuery,
    /// Remote object store or remote file.
    RemoteObject,
    /// Captured sample from a larger dataset.
    CapturedSample,
    /// Imported metadata from another tool or packet.
    ImportedMetadata,
    /// Manually attached dataset reference.
    ManualAttach,
}

/// Scope represented by a dataset summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatasetScopeClass {
    /// Full dataset, partition, or table was used.
    Full,
    /// Sample was used and must stay visible.
    Sample,
    /// Scope is estimated or bounded.
    Estimated,
    /// Scope is unknown because lineage is incomplete.
    Unknown,
}

/// Sensitivity and redaction state for dataset and export summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatasetSensitivityState {
    /// Metadata and samples are safe for normal sharing.
    PublicSafe,
    /// Internal metadata is shareable, raw rows need policy review.
    InternalMetadataSafe,
    /// Sensitive data exists, with redacted identifiers or fields.
    RedactedSensitive,
    /// Raw preview is blocked by policy.
    RawPreviewBlocked,
}

/// Location class safe for UI, support, and export surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocationClass {
    /// Local workspace or local project storage.
    LocalWorkspace,
    /// Local external storage outside the workspace.
    LocalExternal,
    /// Managed-region storage without raw host details.
    ManagedRegion,
    /// Remote object or data service without raw URL details.
    RemoteService,
    /// Imported packet or tracker reference.
    ImportedPacket,
    /// Location is unknown or withheld.
    Unknown,
}

/// Artifact kind attached to a producing run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    /// Figure or chart output.
    Figure,
    /// Rendered report.
    Report,
    /// Model or model checkpoint.
    Model,
    /// Exported dataset or table.
    ExportedDataset,
    /// Metric table or result record.
    MetricTable,
}

/// Lineage state shown on artifact rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactLineageState {
    /// Artifact is current for the producing run.
    Current,
    /// Artifact is stale relative to the producing run or current workspace.
    Stale,
    /// Artifact was imported and does not claim local production.
    Imported,
    /// Artifact was attached manually.
    ManualAttach,
    /// Producing lineage is unknown.
    Unknown,
}

/// Human-readable environment freshness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentFreshnessClass {
    /// Fingerprint is current.
    Current,
    /// Fingerprint is retained but stale.
    Stale,
    /// Fingerprint was imported.
    Imported,
    /// Fingerprint is incomplete.
    Incomplete,
}

/// Machine-readable axis state for comparison truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonAxisState {
    /// Axis matches closely enough for the comparison claim.
    Matching,
    /// Axis differs and must be disclosed beside the delta.
    Different,
    /// Axis is missing and blocks fair comparison.
    Missing,
}

impl ComparisonAxisState {
    const fn matches(self) -> bool {
        matches!(self, Self::Matching)
    }

    const fn missing(self) -> bool {
        matches!(self, Self::Missing)
    }
}

/// Comparison label rendered in UI and carried by machines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonGuardBanner {
    /// Code revision, data scope, and environment are sufficiently aligned.
    Comparable,
    /// Code and data are close enough to compare, but environment changed materially.
    EnvironmentSkewed,
    /// Environment is similar, but dataset snapshot or partition changed materially.
    DataSkewed,
    /// Required provenance is missing, including a missing code or metric-schema axis.
    LineageMissing,
}

impl ComparisonGuardBanner {
    /// Stable user-visible label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Comparable => "Comparable",
            Self::EnvironmentSkewed => "Environment-skewed",
            Self::DataSkewed => "Data-skewed",
            Self::LineageMissing => "Lineage missing",
        }
    }
}

/// Export payload scope reviewed before sharing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportPayloadScope {
    /// Notebook file and metadata.
    NotebookFile,
    /// Rendered report or summary output.
    RenderedReport,
    /// Metadata-only run, dataset, lineage, and environment summary.
    MetadataOnlySummary,
    /// Raw artifact payload or raw dataset payload.
    RawArtifactPayload,
}

/// Trust class carried by export review records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportTrustClass {
    /// Metadata is safe by default.
    MetadataSafe,
    /// Redacted evidence is safe for support or review.
    RedactedEvidence,
    /// Raw payload requires explicit review.
    RawPayloadRequiresReview,
    /// Export is blocked by policy.
    BlockedByPolicy,
}

/// Human-readable environment fingerprint card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentFingerprint {
    /// Stable fingerprint identifier.
    pub environment_fingerprint_id: String,
    /// Reviewer-facing environment identity.
    pub identity_summary: String,
    /// Interpreter, kernel, or runtime label.
    pub interpreter_or_kernel: String,
    /// Package, lockfile, or toolchain summary.
    pub package_or_toolchain_summary: String,
    /// Execution target origin.
    pub target_origin: ExperimentOriginClass,
    /// Hardware or profile class when relevant.
    pub hardware_class: String,
    /// Policy epoch reference when relevant.
    pub policy_epoch_ref: String,
    /// Freshness class for the fingerprint.
    pub freshness: EnvironmentFreshnessClass,
}

/// One experiment run summary card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExperimentRun {
    /// Stable run identifier.
    pub run_id: String,
    /// Title or task label.
    pub title: String,
    /// Source family.
    pub source_kind: SourceKind,
    /// Source notebook/script/task/test reference.
    pub source_ref: String,
    /// Start timestamp or monotonic run marker.
    pub started_at: String,
    /// End timestamp or retained completion marker.
    pub ended_at: String,
    /// Origin class.
    pub origin_class: ExperimentOriginClass,
    /// Outcome class.
    pub outcome: OutcomeClass,
    /// Commit, branch, or workspace revision.
    pub code_revision: String,
    /// Dataset refs used by this run.
    pub dataset_refs: Vec<String>,
    /// Environment fingerprint ref.
    pub environment_fingerprint_ref: String,
    /// User-visible reproducibility label.
    pub reproducibility_label: ReproducibilityLabel,
    /// Compare action is available only after the summary truth is visible.
    pub compare_action_available: bool,
    /// Open source run action is available.
    pub open_action_available: bool,
    /// Export summary action is available.
    pub export_action_available: bool,
}

/// Metadata-first dataset provenance card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatasetSummary {
    /// Stable dataset summary identifier.
    pub dataset_id: String,
    /// Dataset or table label.
    pub label: String,
    /// Source class.
    pub source_class: DatasetSourceClass,
    /// Version, snapshot, partition, or query summary.
    pub version_or_snapshot: String,
    /// Sample, full, estimated, or unknown scope.
    pub scope: DatasetScopeClass,
    /// Row, file, or size estimate.
    pub size_estimate: String,
    /// Sensitivity state.
    pub sensitivity_state: DatasetSensitivityState,
    /// Location class.
    pub location_class: LocationClass,
    /// Schema or query summary.
    pub schema_or_query_summary: String,
    /// Metadata-only disclosure is the default share surface.
    pub metadata_only_default: bool,
    /// Raw samples require explicit drill-down.
    pub raw_sample_drill_down: bool,
    /// Raw data is selected by default.
    pub raw_data_default_share: bool,
}

/// One artifact lineage entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactLineageEntry {
    /// Stable artifact identifier.
    pub artifact_id: String,
    /// Artifact kind.
    pub artifact_kind: ArtifactKind,
    /// Producing run, import source, or manual attach ref.
    pub producing_run_ref: String,
    /// Generator step label.
    pub generator_step: String,
    /// Environment or model fingerprint ref.
    pub environment_or_model_fingerprint_ref: String,
    /// Saved location class.
    pub saved_location_class: LocationClass,
    /// Lineage state shown to users.
    pub lineage_state: ArtifactLineageState,
    /// Open-producing-run action is available when lineage points at a run.
    pub open_producing_run_action: bool,
    /// Copy action is available.
    pub copy_action_available: bool,
    /// Export action is available.
    pub export_action_available: bool,
}

/// Axis-level basis for a result comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonBasis {
    /// Whether code revisions match.
    pub code_revision: ComparisonAxisState,
    /// Whether data snapshots or partitions match.
    pub data_snapshot: ComparisonAxisState,
    /// Whether environment fingerprints match.
    pub environment_fingerprint: ComparisonAxisState,
    /// Whether hardware/profile classes match.
    pub hardware_class: ComparisonAxisState,
    /// Whether metric schemas match.
    pub metric_schema: ComparisonAxisState,
    /// Whether run, dataset, artifact, and environment lineage survived.
    pub retained_lineage: bool,
    /// Reviewable context note shown beside deltas.
    pub context_note: String,
}

impl ComparisonBasis {
    /// Computes the mandatory guard label from axis truth.
    pub const fn expected_guard_label(&self) -> ComparisonGuardBanner {
        if !self.retained_lineage
            || self.code_revision.missing()
            || self.data_snapshot.missing()
            || self.environment_fingerprint.missing()
            || self.hardware_class.missing()
            || self.metric_schema.missing()
        {
            return ComparisonGuardBanner::LineageMissing;
        }
        if !self.data_snapshot.matches() {
            return ComparisonGuardBanner::DataSkewed;
        }
        if !self.environment_fingerprint.matches() || !self.hardware_class.matches() {
            return ComparisonGuardBanner::EnvironmentSkewed;
        }
        if !self.code_revision.matches() || !self.metric_schema.matches() {
            return ComparisonGuardBanner::LineageMissing;
        }
        ComparisonGuardBanner::Comparable
    }
}

/// One metric row inside a result comparison.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonMetricRow {
    /// Metric name.
    pub metric: String,
    /// Baseline value.
    pub baseline_value: f64,
    /// Candidate value.
    pub candidate_value: f64,
    /// Delta value.
    pub delta: f64,
    /// Threshold or confidence note.
    pub threshold_note: String,
}

/// One result comparison row with machine-readable guard state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultComparisonRow {
    /// Stable comparison identifier.
    pub comparison_id: String,
    /// Baseline run identifier.
    pub baseline_run_ref: String,
    /// Candidate run identifier.
    pub candidate_run_ref: String,
    /// Metric schema version.
    pub metric_schema_version: String,
    /// Comparator type label.
    pub comparator_type: String,
    /// Machine-readable and user-visible guard label.
    pub guard_label: ComparisonGuardBanner,
    /// User-visible label text rendered next to deltas.
    pub visible_guard_label: String,
    /// Axis-level comparison basis.
    pub basis: ComparisonBasis,
    /// Data-scope differences shown beside the comparison.
    pub data_scope_difference: String,
    /// Metric rows.
    pub metrics: Vec<ComparisonMetricRow>,
}

/// One export/share review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExportReview {
    /// Stable export review identifier.
    pub export_id: String,
    /// Run or comparison ref being exported.
    pub subject_ref: String,
    /// Payload scope.
    pub payload_scope: ExportPayloadScope,
    /// Trust/redaction class.
    pub trust_class: ExportTrustClass,
    /// Destination class.
    pub destination_class: String,
    /// Retention note.
    pub retention_note: String,
    /// Whether this scope is selected by default.
    pub default_selected: bool,
    /// Whether explicit review is required before export.
    pub explicit_review_required: bool,
}

/// Summary counts for an experiment provenance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExperimentProvenanceSummary {
    /// Number of run cards.
    pub run_count: usize,
    /// Number of dataset cards.
    pub dataset_count: usize,
    /// Number of artifact lineage rows.
    pub artifact_count: usize,
    /// Number of result comparison rows.
    pub comparison_count: usize,
    /// Number of comparable rows.
    pub comparable_count: usize,
    /// Number of downgraded comparison rows.
    pub downgraded_comparison_count: usize,
    /// Whether metadata export remains available when raw preview is blocked.
    pub metadata_export_survives_raw_block: bool,
}

/// Canonical experiment provenance qualification packet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExperimentProvenancePacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release or public proof document.
    pub proof_doc_ref: String,
    /// User-facing data workflow document.
    pub data_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Environment fingerprint cards.
    pub environment_fingerprints: Vec<EnvironmentFingerprint>,
    /// Experiment run summary cards.
    pub runs: Vec<ExperimentRun>,
    /// Dataset provenance cards.
    pub datasets: Vec<DatasetSummary>,
    /// Artifact lineage rows.
    pub artifacts: Vec<ArtifactLineageEntry>,
    /// Result comparison rows.
    pub comparisons: Vec<ResultComparisonRow>,
    /// Export/share review rows.
    pub export_reviews: Vec<ExportReview>,
    /// Summary counts.
    pub summary: ExperimentProvenanceSummary,
}

impl ExperimentProvenancePacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> ExperimentProvenanceSummary {
        let comparable_count = self
            .comparisons
            .iter()
            .filter(|comparison| comparison.guard_label == ComparisonGuardBanner::Comparable)
            .count();
        let metadata_export_survives_raw_block = self.datasets.iter().any(|dataset| {
            dataset.sensitivity_state == DatasetSensitivityState::RawPreviewBlocked
                && dataset.metadata_only_default
                && !dataset.raw_data_default_share
        }) && self.export_reviews.iter().any(|review| {
            review.payload_scope == ExportPayloadScope::MetadataOnlySummary
                && review.trust_class != ExportTrustClass::BlockedByPolicy
        });
        ExperimentProvenanceSummary {
            run_count: self.runs.len(),
            dataset_count: self.datasets.len(),
            artifact_count: self.artifacts.len(),
            comparison_count: self.comparisons.len(),
            comparable_count,
            downgraded_comparison_count: self.comparisons.len().saturating_sub(comparable_count),
            metadata_export_survives_raw_block,
        }
    }

    /// Validates packet invariants for UI, export, support, and public proof consumers.
    pub fn validate(&self) -> Vec<ExperimentProvenanceViolation> {
        let mut violations = Vec::new();
        if self.schema_version != EXPERIMENT_PROVENANCE_SCHEMA_VERSION {
            violations.push(ExperimentProvenanceViolation::SchemaVersion {
                expected: EXPERIMENT_PROVENANCE_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EXPERIMENT_PROVENANCE_RECORD_KIND {
            violations.push(ExperimentProvenanceViolation::RecordKind {
                expected: EXPERIMENT_PROVENANCE_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let environment_ids = collect_ids(
            self.environment_fingerprints
                .iter()
                .map(|env| env.environment_fingerprint_id.as_str()),
            &mut violations,
            ExperimentProvenanceViolationKind::Environment,
        );
        let run_ids = collect_ids(
            self.runs.iter().map(|run| run.run_id.as_str()),
            &mut violations,
            ExperimentProvenanceViolationKind::Run,
        );
        let dataset_ids = collect_ids(
            self.datasets
                .iter()
                .map(|dataset| dataset.dataset_id.as_str()),
            &mut violations,
            ExperimentProvenanceViolationKind::Dataset,
        );
        let _artifact_ids = collect_ids(
            self.artifacts
                .iter()
                .map(|artifact| artifact.artifact_id.as_str()),
            &mut violations,
            ExperimentProvenanceViolationKind::Artifact,
        );
        let _comparison_ids = collect_ids(
            self.comparisons
                .iter()
                .map(|comparison| comparison.comparison_id.as_str()),
            &mut violations,
            ExperimentProvenanceViolationKind::Comparison,
        );

        for run in &self.runs {
            if run.source_ref.is_empty()
                || run.code_revision.is_empty()
                || run.dataset_refs.is_empty()
                || run.environment_fingerprint_ref.is_empty()
                || !run.compare_action_available
                || !run.open_action_available
                || !run.export_action_available
            {
                violations.push(ExperimentProvenanceViolation::IncompleteRunSummary {
                    run_id: run.run_id.clone(),
                });
            }
            if !environment_ids.contains(&run.environment_fingerprint_ref) {
                violations.push(ExperimentProvenanceViolation::UnknownEnvironmentRef {
                    subject_id: run.run_id.clone(),
                    environment_ref: run.environment_fingerprint_ref.clone(),
                });
            }
            for dataset_ref in &run.dataset_refs {
                if !dataset_ids.contains(dataset_ref) {
                    violations.push(ExperimentProvenanceViolation::UnknownDatasetRef {
                        run_id: run.run_id.clone(),
                        dataset_ref: dataset_ref.clone(),
                    });
                }
            }
        }

        for dataset in &self.datasets {
            if !dataset.metadata_only_default
                || !dataset.raw_sample_drill_down
                || dataset.raw_data_default_share
                || dataset.version_or_snapshot.is_empty()
                || dataset.size_estimate.is_empty()
                || dataset.schema_or_query_summary.is_empty()
            {
                violations.push(
                    ExperimentProvenanceViolation::DatasetDoesNotDefaultToMetadata {
                        dataset_id: dataset.dataset_id.clone(),
                    },
                );
            }
        }

        for artifact in &self.artifacts {
            if matches!(
                artifact.lineage_state,
                ArtifactLineageState::Current | ArtifactLineageState::Stale
            ) && !run_ids.contains(&artifact.producing_run_ref)
            {
                violations.push(ExperimentProvenanceViolation::UnknownProducingRunRef {
                    artifact_id: artifact.artifact_id.clone(),
                    producing_run_ref: artifact.producing_run_ref.clone(),
                });
            }
            if artifact.lineage_state == ArtifactLineageState::Unknown
                && artifact.open_producing_run_action
            {
                violations.push(
                    ExperimentProvenanceViolation::UnknownLineageHasOpenRunAction {
                        artifact_id: artifact.artifact_id.clone(),
                    },
                );
            }
        }

        for comparison in &self.comparisons {
            if !run_ids.contains(&comparison.baseline_run_ref)
                || !run_ids.contains(&comparison.candidate_run_ref)
            {
                violations.push(ExperimentProvenanceViolation::UnknownComparisonRunRef {
                    comparison_id: comparison.comparison_id.clone(),
                });
            }
            let expected = comparison.basis.expected_guard_label();
            if comparison.guard_label != expected {
                violations.push(ExperimentProvenanceViolation::ComparisonGuardMismatch {
                    comparison_id: comparison.comparison_id.clone(),
                    expected,
                    actual: comparison.guard_label,
                });
            }
            if comparison.visible_guard_label != comparison.guard_label.label() {
                violations.push(
                    ExperimentProvenanceViolation::ComparisonVisibleLabelMismatch {
                        comparison_id: comparison.comparison_id.clone(),
                    },
                );
            }
            if comparison.metrics.is_empty()
                || comparison.metric_schema_version.is_empty()
                || comparison.comparator_type.is_empty()
                || comparison.basis.context_note.is_empty()
            {
                violations.push(ExperimentProvenanceViolation::IncompleteComparison {
                    comparison_id: comparison.comparison_id.clone(),
                });
            }
        }

        let has_notebook = self
            .export_reviews
            .iter()
            .any(|review| review.payload_scope == ExportPayloadScope::NotebookFile);
        let has_report = self
            .export_reviews
            .iter()
            .any(|review| review.payload_scope == ExportPayloadScope::RenderedReport);
        let has_metadata = self
            .export_reviews
            .iter()
            .any(|review| review.payload_scope == ExportPayloadScope::MetadataOnlySummary);
        let has_raw = self
            .export_reviews
            .iter()
            .any(|review| review.payload_scope == ExportPayloadScope::RawArtifactPayload);
        if !(has_notebook && has_report && has_metadata && has_raw) {
            violations.push(ExperimentProvenanceViolation::MissingExportScopeReview);
        }
        for review in &self.export_reviews {
            if review.destination_class.is_empty() || review.retention_note.is_empty() {
                violations.push(ExperimentProvenanceViolation::IncompleteExportReview {
                    export_id: review.export_id.clone(),
                });
            }
            if review.payload_scope == ExportPayloadScope::RawArtifactPayload
                && (review.default_selected || !review.explicit_review_required)
            {
                violations.push(ExperimentProvenanceViolation::RawPayloadDefaultExport {
                    export_id: review.export_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ExperimentProvenanceViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in experiment provenance qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_experiment_provenance_qualification(
) -> Result<ExperimentProvenancePacket, serde_json::Error> {
    serde_json::from_str(EXPERIMENT_PROVENANCE_PACKET_JSON)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExperimentProvenanceViolationKind {
    /// Environment fingerprint identity family.
    Environment,
    /// Experiment run identity family.
    Run,
    /// Dataset summary identity family.
    Dataset,
    /// Artifact lineage identity family.
    Artifact,
    /// Result comparison identity family.
    Comparison,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<ExperimentProvenanceViolation>,
    kind: ExperimentProvenanceViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(ExperimentProvenanceViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for experiment provenance packets.
#[derive(Debug, Clone, PartialEq)]
pub enum ExperimentProvenanceViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: ExperimentProvenanceViolationKind,
        id: String,
    },
    /// Run summary is missing required visible fields or trusted actions.
    IncompleteRunSummary { run_id: String },
    /// Run references an unknown environment fingerprint.
    UnknownEnvironmentRef {
        subject_id: String,
        environment_ref: String,
    },
    /// Run references an unknown dataset.
    UnknownDatasetRef { run_id: String, dataset_ref: String },
    /// Dataset card does not default to metadata-only disclosure.
    DatasetDoesNotDefaultToMetadata { dataset_id: String },
    /// Artifact references an unknown producing run.
    UnknownProducingRunRef {
        artifact_id: String,
        producing_run_ref: String,
    },
    /// Unknown lineage cannot offer an open-producing-run action.
    UnknownLineageHasOpenRunAction { artifact_id: String },
    /// Comparison references unknown runs.
    UnknownComparisonRunRef { comparison_id: String },
    /// Comparison guard label does not match axis truth.
    ComparisonGuardMismatch {
        comparison_id: String,
        expected: ComparisonGuardBanner,
        actual: ComparisonGuardBanner,
    },
    /// Comparison visible label does not match the machine label.
    ComparisonVisibleLabelMismatch { comparison_id: String },
    /// Comparison row lacks required metric or context fields.
    IncompleteComparison { comparison_id: String },
    /// Export review does not cover all required payload scopes.
    MissingExportScopeReview,
    /// Export review lacks destination or retention truth.
    IncompleteExportReview { export_id: String },
    /// Raw payload export was selected by default or lacked explicit review.
    RawPayloadDefaultExport { export_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for ExperimentProvenanceViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::IncompleteRunSummary { run_id } => {
                write!(f, "{run_id} lacks complete run summary truth")
            }
            Self::UnknownEnvironmentRef {
                subject_id,
                environment_ref,
            } => write!(
                f,
                "{subject_id} references unknown environment {environment_ref}"
            ),
            Self::UnknownDatasetRef {
                run_id,
                dataset_ref,
            } => write!(f, "{run_id} references unknown dataset {dataset_ref}"),
            Self::DatasetDoesNotDefaultToMetadata { dataset_id } => {
                write!(
                    f,
                    "{dataset_id} does not default to metadata-only disclosure"
                )
            }
            Self::UnknownProducingRunRef {
                artifact_id,
                producing_run_ref,
            } => write!(
                f,
                "{artifact_id} references unknown producing run {producing_run_ref}"
            ),
            Self::UnknownLineageHasOpenRunAction { artifact_id } => write!(
                f,
                "{artifact_id} has unknown lineage but offers open-producing-run"
            ),
            Self::UnknownComparisonRunRef { comparison_id } => {
                write!(f, "{comparison_id} references unknown comparison runs")
            }
            Self::ComparisonGuardMismatch {
                comparison_id,
                expected,
                actual,
            } => write!(
                f,
                "{comparison_id} guard expected {expected:?}, got {actual:?}"
            ),
            Self::ComparisonVisibleLabelMismatch { comparison_id } => {
                write!(f, "{comparison_id} visible guard label does not match")
            }
            Self::IncompleteComparison { comparison_id } => {
                write!(f, "{comparison_id} lacks complete comparison truth")
            }
            Self::MissingExportScopeReview => write!(f, "export reviews do not cover all scopes"),
            Self::IncompleteExportReview { export_id } => {
                write!(f, "{export_id} lacks destination or retention truth")
            }
            Self::RawPayloadDefaultExport { export_id } => {
                write!(f, "{export_id} defaults raw payload export")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for ExperimentProvenanceViolation {}
