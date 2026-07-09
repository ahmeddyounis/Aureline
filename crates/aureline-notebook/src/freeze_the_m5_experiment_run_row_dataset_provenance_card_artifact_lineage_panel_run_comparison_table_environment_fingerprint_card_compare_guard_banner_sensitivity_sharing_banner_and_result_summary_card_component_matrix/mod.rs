//! Frozen M5 experiment-run-row, dataset-provenance-card, artifact-lineage-panel,
//! run-comparison-table, environment-fingerprint-card, compare-guard-banner,
//! sensitivity-sharing-banner, and result-summary-card component matrix.
//!
//! This module locks Aureline's reusable experiment / reproducibility components into one
//! export-safe packet. Every notebook-adjacent and data-workflow subcomponent M5 claims
//! that still drifts too easily by notebook, experiment-dashboard, comparison, data-catalog,
//! share-review, or CLI surface — the experiment run row, the dataset provenance card, the
//! artifact lineage panel, the run comparison table, the environment fingerprint card, the
//! compare guard banner, the sensitivity / sharing banner, and the result summary card — is
//! named once here and constrained by the same run identity, execution origin, code
//! revision, environment fingerprint, dataset provenance, sensitivity class,
//! comparability / confounder disclosure, and summary-versus-evidence-versus-raw export
//! scope regardless of the surface family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components* themselves: the
//! component families; the one controlled disposition vocabulary every consumer binds
//! (`local_run`, `managed_run`, `imported_run`, `manual_attach`, `reproducible`,
//! `likely_reproducible`, `needs_rerun`, `context_incomplete`); the run origin kinds and
//! status states the run row binds; the dataset source classes and provenance states the
//! dataset provenance card binds; the artifact kinds and lineage states the lineage panel
//! binds; the comparison axes and comparability states the run comparison table binds; the
//! fingerprint scopes and capture states the environment fingerprint card binds; the guard
//! reasons and guard states the compare guard banner binds; the sensitivity classes and
//! share scopes the sensitivity / sharing banner binds; the summary content classes and
//! export scopes the result summary card binds; the deployment lines every component must
//! survive; the non-visual accessibility routes; and the mandatory labels every component
//! must be able to show. It does not re-architect the experiment-run-identity, dataset-card,
//! artifact-lineage, run-comparison, or share/handoff contracts that already own those
//! records — it is the shared experiment-component contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 notebook, experiment,
//! comparison, data, or share surface may publish an experiment run row, a dataset
//! provenance card, an artifact lineage panel, a run comparison table, an environment
//! fingerprint card, a compare guard banner, a sensitivity / sharing banner, or a result
//! summary card. Notebook, experiment, comparison, data, and share consumers all read this
//! packet so one run row names where a run came from and its code revision, one dataset
//! provenance card names what data was used and how completely it is provenanced, one
//! lineage panel names its upstream and downstream, one comparison table names whether two
//! results are actually comparable and never implies apples-to-apples without parity
//! evidence, one environment fingerprint card names its captured environment, one compare
//! guard banner names why a comparison is guarded, one sensitivity / sharing banner names
//! its sensitivity class and share scope and never exposes raw production-like data by
//! default, and one result summary card names whether a shared summary is summary,
//! metadata, evidence, or raw scope. No M5 lane invents a second experiment grammar or an
//! alternate label for a governed origin, provenance, comparability, sensitivity, or
//! export-scope state.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5ExperimentComponentVocabularySet`] rather than minted per surface. Raw dataset
//! payloads, pasted paths, credentials, and private endpoints stay outside the export
//! boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_experiment_component_matrix,
    seeded_m5_experiment_component_matrix_run_comparison_table_beta_narrowed,
    seeded_m5_experiment_component_matrix_sensitivity_sharing_banner_preview_narrowed,
    M5_EXPERIMENT_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ExperimentComponentMatrixPacket`].
pub const M5_EXPERIMENT_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix";

/// Schema version for M5 experiment component-matrix records.
pub const M5_EXPERIMENT_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined experiment-component boundary schema.
pub const M5_EXPERIMENT_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-experiment-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EXPERIMENT_COMPONENT_DOC_REF: &str =
    "docs/notebooks/m5_experiment_component_matrix.md";

/// Repo-relative path of the per-component experiment-run-row schema.
pub const M5_EXPERIMENT_RUN_ROW_SCHEMA_REF: &str = "schemas/ui/m5-experiment-run-row.schema.json";

/// Repo-relative path of the per-component dataset-provenance-card schema.
pub const M5_DATASET_PROVENANCE_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-dataset-provenance-card.schema.json";

/// Repo-relative path of the per-component artifact-lineage-panel schema.
pub const M5_ARTIFACT_LINEAGE_PANEL_SCHEMA_REF: &str =
    "schemas/ui/m5-artifact-lineage-panel.schema.json";

/// Repo-relative path of the per-component run-comparison-table schema.
pub const M5_RUN_COMPARISON_TABLE_SCHEMA_REF: &str =
    "schemas/ui/m5-run-comparison-table.schema.json";

/// Repo-relative path of the per-component environment-fingerprint-card schema.
pub const M5_ENVIRONMENT_FINGERPRINT_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-environment-fingerprint-card.schema.json";

/// Repo-relative path of the per-component compare-guard-banner schema.
pub const M5_COMPARE_GUARD_BANNER_SCHEMA_REF: &str =
    "schemas/ui/m5-compare-guard-banner.schema.json";

/// Repo-relative path of the per-component sensitivity-sharing-banner schema.
pub const M5_SENSITIVITY_SHARING_BANNER_SCHEMA_REF: &str =
    "schemas/ui/m5-sensitivity-sharing-banner.schema.json";

/// Repo-relative path of the per-component result-summary-card schema.
pub const M5_RESULT_SUMMARY_CARD_SCHEMA_REF: &str = "schemas/ui/m5-result-summary-card.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_EXPERIMENT_COMPONENT_FIXTURE_DIR: &str = "fixtures/ui/m5-experiment-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EXPERIMENT_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-experiment-component-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_EXPERIMENT_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-experiment-component-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_EXPERIMENT_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-experiment-component-matrix.md";

/// One of the eight governed experiment-component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentComponentFamily {
    /// An experiment run row carrying its run origin and status.
    ExperimentRunRow,
    /// A dataset provenance card carrying its source class and provenance state.
    DatasetProvenanceCard,
    /// An artifact lineage panel carrying its artifact kind and lineage state.
    ArtifactLineagePanel,
    /// A run comparison table carrying its comparison axes and comparability state.
    RunComparisonTable,
    /// An environment fingerprint card carrying its fingerprint scopes and capture state.
    EnvironmentFingerprintCard,
    /// A compare guard banner carrying its guard reason and guard state.
    CompareGuardBanner,
    /// A sensitivity / sharing banner carrying its sensitivity class and share scope.
    SensitivitySharingBanner,
    /// A result summary card carrying its summary content class and export scope.
    ResultSummaryCard,
}

impl M5ExperimentComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ExperimentRunRow,
        Self::DatasetProvenanceCard,
        Self::ArtifactLineagePanel,
        Self::RunComparisonTable,
        Self::EnvironmentFingerprintCard,
        Self::CompareGuardBanner,
        Self::SensitivitySharingBanner,
        Self::ResultSummaryCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExperimentRunRow => "experiment_run_row",
            Self::DatasetProvenanceCard => "dataset_provenance_card",
            Self::ArtifactLineagePanel => "artifact_lineage_panel",
            Self::RunComparisonTable => "run_comparison_table",
            Self::EnvironmentFingerprintCard => "environment_fingerprint_card",
            Self::CompareGuardBanner => "compare_guard_banner",
            Self::SensitivitySharingBanner => "sensitivity_sharing_banner",
            Self::ResultSummaryCard => "result_summary_card",
        }
    }

    /// `true` when this family is an experiment run row and must therefore declare its run
    /// origin kinds and status states.
    pub const fn is_experiment_run_row(self) -> bool {
        matches!(self, Self::ExperimentRunRow)
    }

    /// `true` when this family is a dataset provenance card and must therefore declare its
    /// dataset source classes and provenance states.
    pub const fn is_dataset_provenance_card(self) -> bool {
        matches!(self, Self::DatasetProvenanceCard)
    }

    /// `true` when this family is an artifact lineage panel and must therefore declare its
    /// artifact kinds and lineage states.
    pub const fn is_artifact_lineage_panel(self) -> bool {
        matches!(self, Self::ArtifactLineagePanel)
    }

    /// `true` when this family is a run comparison table and must therefore declare its
    /// comparison axes and comparability states.
    pub const fn is_run_comparison_table(self) -> bool {
        matches!(self, Self::RunComparisonTable)
    }

    /// `true` when this family is an environment fingerprint card and must therefore declare
    /// its fingerprint scopes and capture states.
    pub const fn is_environment_fingerprint_card(self) -> bool {
        matches!(self, Self::EnvironmentFingerprintCard)
    }

    /// `true` when this family is a compare guard banner and must therefore declare its guard
    /// reasons and guard states.
    pub const fn is_compare_guard_banner(self) -> bool {
        matches!(self, Self::CompareGuardBanner)
    }

    /// `true` when this family is a sensitivity / sharing banner and must therefore declare
    /// its sensitivity classes and share scope states.
    pub const fn is_sensitivity_sharing_banner(self) -> bool {
        matches!(self, Self::SensitivitySharingBanner)
    }

    /// `true` when this family is a result summary card and must therefore declare its
    /// summary content classes and export scopes.
    pub const fn is_result_summary_card(self) -> bool {
        matches!(self, Self::ResultSummaryCard)
    }
}

/// The one controlled disposition vocabulary every experiment-component consumer binds. These
/// are the exact acceptance-criteria labels so no surface invents a parallel word for a
/// local, managed, imported, or manually attached run, or for a reproducible, likely
/// reproducible, needs-rerun, or context-incomplete result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentDisposition {
    /// A local run.
    LocalRun,
    /// A managed run.
    ManagedRun,
    /// An imported run.
    ImportedRun,
    /// A manually attached run.
    ManualAttach,
    /// Reproducible.
    Reproducible,
    /// Likely reproducible.
    LikelyReproducible,
    /// Needs rerun.
    NeedsRerun,
    /// Context incomplete.
    ContextIncomplete,
}

impl M5ExperimentDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::LocalRun,
        Self::ManagedRun,
        Self::ImportedRun,
        Self::ManualAttach,
        Self::Reproducible,
        Self::LikelyReproducible,
        Self::NeedsRerun,
        Self::ContextIncomplete,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalRun => "local_run",
            Self::ManagedRun => "managed_run",
            Self::ImportedRun => "imported_run",
            Self::ManualAttach => "manual_attach",
            Self::Reproducible => "reproducible",
            Self::LikelyReproducible => "likely_reproducible",
            Self::NeedsRerun => "needs_rerun",
            Self::ContextIncomplete => "context_incomplete",
        }
    }
}

/// Controlled run origin kind — where an experiment run came from, so a run row never leaves
/// its notebook / script / task origin implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunOriginKind {
    /// Launched from a notebook cell.
    NotebookCell,
    /// Launched from a script task.
    ScriptTask,
    /// Launched from a scheduled task.
    ScheduledTask,
    /// Manually attached to an external run.
    ManualAttach,
    /// Imported from another tracker.
    ImportedRun,
    /// Origin unknown.
    UnknownOrigin,
}

impl M5RunOriginKind {
    /// Every run origin kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotebookCell,
        Self::ScriptTask,
        Self::ScheduledTask,
        Self::ManualAttach,
        Self::ImportedRun,
        Self::UnknownOrigin,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookCell => "notebook_cell",
            Self::ScriptTask => "script_task",
            Self::ScheduledTask => "scheduled_task",
            Self::ManualAttach => "manual_attach",
            Self::ImportedRun => "imported_run",
            Self::UnknownOrigin => "unknown_origin",
        }
    }
}

/// Controlled run status state — where an experiment run stands, so a run row never leaves
/// its status or staleness implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunStatusState {
    /// Queued.
    Queued,
    /// Running.
    Running,
    /// Succeeded.
    Succeeded,
    /// Failed.
    Failed,
    /// Canceled.
    Canceled,
    /// Stale / superseded.
    Stale,
}

impl M5RunStatusState {
    /// Every run status state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Queued,
        Self::Running,
        Self::Succeeded,
        Self::Failed,
        Self::Canceled,
        Self::Stale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Canceled => "canceled",
            Self::Stale => "stale",
        }
    }
}

/// Controlled dataset source class — where a dataset provenance card's data comes from, so a
/// card never leaves what data was used implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DatasetSourceClass {
    /// A tracked dataset.
    TrackedDataset,
    /// A local file.
    LocalFile,
    /// A remote snapshot.
    RemoteSnapshot,
    /// Synthetic data.
    SyntheticData,
    /// A redacted sample.
    RedactedSample,
    /// An unknown source.
    UnknownSource,
}

impl M5DatasetSourceClass {
    /// Every dataset source class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TrackedDataset,
        Self::LocalFile,
        Self::RemoteSnapshot,
        Self::SyntheticData,
        Self::RedactedSample,
        Self::UnknownSource,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrackedDataset => "tracked_dataset",
            Self::LocalFile => "local_file",
            Self::RemoteSnapshot => "remote_snapshot",
            Self::SyntheticData => "synthetic_data",
            Self::RedactedSample => "redacted_sample",
            Self::UnknownSource => "unknown_source",
        }
    }
}

/// Controlled dataset provenance state — how completely a dataset provenance card is
/// provenanced, so a card never hides that its provenance is partial, missing, or drifted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DatasetProvenanceState {
    /// Provenance complete.
    ProvenanceComplete,
    /// Provenance partial.
    ProvenancePartial,
    /// Provenance missing.
    ProvenanceMissing,
    /// Version pinned.
    VersionPinned,
    /// Version drifted.
    VersionDrifted,
    /// Access restricted.
    AccessRestricted,
}

impl M5DatasetProvenanceState {
    /// Every dataset provenance state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProvenanceComplete,
        Self::ProvenancePartial,
        Self::ProvenanceMissing,
        Self::VersionPinned,
        Self::VersionDrifted,
        Self::AccessRestricted,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenanceComplete => "provenance_complete",
            Self::ProvenancePartial => "provenance_partial",
            Self::ProvenanceMissing => "provenance_missing",
            Self::VersionPinned => "version_pinned",
            Self::VersionDrifted => "version_drifted",
            Self::AccessRestricted => "access_restricted",
        }
    }
}

/// Controlled artifact kind class — what a generated artifact is, so an artifact lineage
/// panel never leaves the kind of artifact it tracks implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArtifactKindClass {
    /// A model checkpoint.
    ModelCheckpoint,
    /// A metrics table.
    MetricsTable,
    /// A plot / figure.
    PlotFigure,
    /// An exported report.
    ExportedReport,
    /// A log bundle.
    LogBundle,
    /// An unknown artifact.
    UnknownArtifact,
}

impl M5ArtifactKindClass {
    /// Every artifact kind class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ModelCheckpoint,
        Self::MetricsTable,
        Self::PlotFigure,
        Self::ExportedReport,
        Self::LogBundle,
        Self::UnknownArtifact,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModelCheckpoint => "model_checkpoint",
            Self::MetricsTable => "metrics_table",
            Self::PlotFigure => "plot_figure",
            Self::ExportedReport => "exported_report",
            Self::LogBundle => "log_bundle",
            Self::UnknownArtifact => "unknown_artifact",
        }
    }
}

/// Controlled lineage state — how completely an artifact lineage panel resolves its upstream
/// and downstream, so a panel never hides a broken or unknown lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LineageState {
    /// Lineage complete.
    LineageComplete,
    /// Lineage partial.
    LineagePartial,
    /// Lineage broken.
    LineageBroken,
    /// Derived from a known upstream.
    DerivedUpstreamKnown,
    /// Derived from an unknown upstream.
    DerivedUpstreamUnknown,
    /// Regenerated.
    Regenerated,
}

impl M5LineageState {
    /// Every lineage state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LineageComplete,
        Self::LineagePartial,
        Self::LineageBroken,
        Self::DerivedUpstreamKnown,
        Self::DerivedUpstreamUnknown,
        Self::Regenerated,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LineageComplete => "lineage_complete",
            Self::LineagePartial => "lineage_partial",
            Self::LineageBroken => "lineage_broken",
            Self::DerivedUpstreamKnown => "derived_upstream_known",
            Self::DerivedUpstreamUnknown => "derived_upstream_unknown",
            Self::Regenerated => "regenerated",
        }
    }
}

/// Controlled comparison axis class — along which axis a run comparison table compares runs,
/// so a table never leaves what it is actually diffing implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComparisonAxisClass {
    /// A metric delta.
    MetricDelta,
    /// A parameter diff.
    ParamDiff,
    /// A dataset diff.
    DatasetDiff,
    /// An environment diff.
    EnvDiff,
    /// A code revision diff.
    CodeRevisionDiff,
    /// An artifact diff.
    ArtifactDiff,
}

impl M5ComparisonAxisClass {
    /// Every comparison axis class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MetricDelta,
        Self::ParamDiff,
        Self::DatasetDiff,
        Self::EnvDiff,
        Self::CodeRevisionDiff,
        Self::ArtifactDiff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetricDelta => "metric_delta",
            Self::ParamDiff => "param_diff",
            Self::DatasetDiff => "dataset_diff",
            Self::EnvDiff => "env_diff",
            Self::CodeRevisionDiff => "code_revision_diff",
            Self::ArtifactDiff => "artifact_diff",
        }
    }
}

/// Controlled comparability state — whether two runs are actually comparable, so a run
/// comparison table never implies apples-to-apples without parity evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComparabilityState {
    /// Comparable.
    Comparable,
    /// Comparable with caveats.
    ComparableWithCaveats,
    /// Not comparable.
    NotComparable,
    /// Confounded.
    Confounded,
    /// Insufficient overlap.
    InsufficientOverlap,
    /// Unknown comparability.
    UnknownComparability,
}

impl M5ComparabilityState {
    /// Every comparability state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Comparable,
        Self::ComparableWithCaveats,
        Self::NotComparable,
        Self::Confounded,
        Self::InsufficientOverlap,
        Self::UnknownComparability,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Comparable => "comparable",
            Self::ComparableWithCaveats => "comparable_with_caveats",
            Self::NotComparable => "not_comparable",
            Self::Confounded => "confounded",
            Self::InsufficientOverlap => "insufficient_overlap",
            Self::UnknownComparability => "unknown_comparability",
        }
    }
}

/// Controlled fingerprint scope class — which slice of the environment an environment
/// fingerprint card captures, so a card never leaves the environment it fingerprints
/// implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FingerprintScopeClass {
    /// The interpreter.
    Interpreter,
    /// The kernel spec.
    KernelSpec,
    /// The installed packages.
    Packages,
    /// The hardware accelerator.
    HardwareAccelerator,
    /// The OS / platform.
    OsPlatform,
    /// The container image.
    ContainerImage,
}

impl M5FingerprintScopeClass {
    /// Every fingerprint scope class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Interpreter,
        Self::KernelSpec,
        Self::Packages,
        Self::HardwareAccelerator,
        Self::OsPlatform,
        Self::ContainerImage,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Interpreter => "interpreter",
            Self::KernelSpec => "kernel_spec",
            Self::Packages => "packages",
            Self::HardwareAccelerator => "hardware_accelerator",
            Self::OsPlatform => "os_platform",
            Self::ContainerImage => "container_image",
        }
    }
}

/// Controlled fingerprint state — how completely an environment fingerprint was captured, so
/// a card never hides that its capture is partial, missing, or drifted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FingerprintState {
    /// Captured complete.
    CapturedComplete,
    /// Captured partial.
    CapturedPartial,
    /// Captured missing.
    CapturedMissing,
    /// Pinned.
    Pinned,
    /// Drifted.
    Drifted,
    /// Unavailable.
    Unavailable,
}

impl M5FingerprintState {
    /// Every fingerprint state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CapturedComplete,
        Self::CapturedPartial,
        Self::CapturedMissing,
        Self::Pinned,
        Self::Drifted,
        Self::Unavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapturedComplete => "captured_complete",
            Self::CapturedPartial => "captured_partial",
            Self::CapturedMissing => "captured_missing",
            Self::Pinned => "pinned",
            Self::Drifted => "drifted",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Controlled compare guard reason — why a compare guard banner is guarding a comparison, so
/// a banner never leaves the reason two results may not be comparable implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompareGuardReason {
    /// A dataset mismatch.
    DatasetMismatch,
    /// Environment drift.
    EnvironmentDrift,
    /// A code revision gap.
    CodeRevisionGap,
    /// A metric definition change.
    MetricDefinitionChange,
    /// A sample size imbalance.
    SampleSizeImbalance,
    /// A confounder present.
    ConfounderPresent,
}

impl M5CompareGuardReason {
    /// Every compare guard reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DatasetMismatch,
        Self::EnvironmentDrift,
        Self::CodeRevisionGap,
        Self::MetricDefinitionChange,
        Self::SampleSizeImbalance,
        Self::ConfounderPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DatasetMismatch => "dataset_mismatch",
            Self::EnvironmentDrift => "environment_drift",
            Self::CodeRevisionGap => "code_revision_gap",
            Self::MetricDefinitionChange => "metric_definition_change",
            Self::SampleSizeImbalance => "sample_size_imbalance",
            Self::ConfounderPresent => "confounder_present",
        }
    }
}

/// Controlled compare guard state — what a compare guard banner permits, so a banner never
/// silently allows an apples-to-apples comparison the guard should block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompareGuardState {
    /// Comparison permitted.
    ComparisonPermitted,
    /// Comparison caveated.
    ComparisonCaveated,
    /// Comparison blocked.
    ComparisonBlocked,
    /// Guard acknowledged.
    GuardAcknowledged,
    /// Guard overridden by explicit choice.
    GuardOverriddenByChoice,
    /// Guard unavailable.
    GuardUnavailable,
}

impl M5CompareGuardState {
    /// Every compare guard state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ComparisonPermitted,
        Self::ComparisonCaveated,
        Self::ComparisonBlocked,
        Self::GuardAcknowledged,
        Self::GuardOverriddenByChoice,
        Self::GuardUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComparisonPermitted => "comparison_permitted",
            Self::ComparisonCaveated => "comparison_caveated",
            Self::ComparisonBlocked => "comparison_blocked",
            Self::GuardAcknowledged => "guard_acknowledged",
            Self::GuardOverriddenByChoice => "guard_overridden_by_choice",
            Self::GuardUnavailable => "guard_unavailable",
        }
    }
}

/// Controlled sensitivity class — how sensitive a result or dataset is, so a sensitivity /
/// sharing banner never leaves its sensitivity implicit before a share.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SensitivityClass {
    /// Public-safe.
    PublicSafe,
    /// Internal.
    Internal,
    /// Confidential.
    Confidential,
    /// Regulated.
    Regulated,
    /// Production-like.
    ProductionLike,
    /// Unknown sensitivity.
    UnknownSensitivity,
}

impl M5SensitivityClass {
    /// Every sensitivity class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PublicSafe,
        Self::Internal,
        Self::Confidential,
        Self::Regulated,
        Self::ProductionLike,
        Self::UnknownSensitivity,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicSafe => "public_safe",
            Self::Internal => "internal",
            Self::Confidential => "confidential",
            Self::Regulated => "regulated",
            Self::ProductionLike => "production_like",
            Self::UnknownSensitivity => "unknown_sensitivity",
        }
    }
}

/// Controlled share scope state — what a sensitivity / sharing banner will actually include
/// in a share, so a banner never exposes raw production-like data by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShareScopeState {
    /// Summary only.
    SummaryOnly,
    /// Summary plus metadata.
    SummaryPlusMetadata,
    /// Evidence included.
    EvidenceIncluded,
    /// Raw payload included.
    RawPayloadIncluded,
    /// A redacted share.
    RedactedShare,
    /// Share blocked.
    ShareBlocked,
}

impl M5ShareScopeState {
    /// Every share scope state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SummaryOnly,
        Self::SummaryPlusMetadata,
        Self::EvidenceIncluded,
        Self::RawPayloadIncluded,
        Self::RedactedShare,
        Self::ShareBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SummaryOnly => "summary_only",
            Self::SummaryPlusMetadata => "summary_plus_metadata",
            Self::EvidenceIncluded => "evidence_included",
            Self::RawPayloadIncluded => "raw_payload_included",
            Self::RedactedShare => "redacted_share",
            Self::ShareBlocked => "share_blocked",
        }
    }
}

/// Controlled summary content class — what a result summary card is actually showing, so a
/// card never blurs a headline metric, a narrative, an evidence link, and a raw payload
/// reference together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SummaryContentClass {
    /// A headline metric.
    HeadlineMetric,
    /// A metric table.
    MetricTable,
    /// A narrative summary.
    NarrativeSummary,
    /// An evidence link.
    EvidenceLink,
    /// A raw payload reference.
    RawPayloadRef,
    /// No result.
    NoResult,
}

impl M5SummaryContentClass {
    /// Every summary content class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HeadlineMetric,
        Self::MetricTable,
        Self::NarrativeSummary,
        Self::EvidenceLink,
        Self::RawPayloadRef,
        Self::NoResult,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeadlineMetric => "headline_metric",
            Self::MetricTable => "metric_table",
            Self::NarrativeSummary => "narrative_summary",
            Self::EvidenceLink => "evidence_link",
            Self::RawPayloadRef => "raw_payload_ref",
            Self::NoResult => "no_result",
        }
    }
}

/// Controlled summary export scope — what scope a result summary card exports, so a shared
/// summary never silently widens from summary to raw scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SummaryExportScope {
    /// Summary scope.
    SummaryScope,
    /// Metadata scope.
    MetadataScope,
    /// Evidence scope.
    EvidenceScope,
    /// Raw scope.
    RawScope,
    /// Redacted scope.
    RedactedScope,
    /// Export withheld.
    ExportWithheld,
}

impl M5SummaryExportScope {
    /// Every summary export scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SummaryScope,
        Self::MetadataScope,
        Self::EvidenceScope,
        Self::RawScope,
        Self::RedactedScope,
        Self::ExportWithheld,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SummaryScope => "summary_scope",
            Self::MetadataScope => "metadata_scope",
            Self::EvidenceScope => "evidence_scope",
            Self::RawScope => "raw_scope",
            Self::RedactedScope => "redacted_scope",
            Self::ExportWithheld => "export_withheld",
        }
    }
}

/// Claimed M5 notebook-adjacent / data-workflow surface family that renders / consumes an
/// experiment component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentSurfaceFamily {
    /// The notebook surface.
    NotebookSurface,
    /// The experiment run dashboard surface.
    ExperimentRunDashboard,
    /// The run comparison view surface.
    RunComparisonView,
    /// The dataset catalog surface.
    DatasetCatalog,
    /// The share review sheet surface.
    ShareReviewSheet,
    /// The CLI surface.
    CliSurface,
}

impl M5ExperimentSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotebookSurface,
        Self::ExperimentRunDashboard,
        Self::RunComparisonView,
        Self::DatasetCatalog,
        Self::ShareReviewSheet,
        Self::CliSurface,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookSurface => "notebook_surface",
            Self::ExperimentRunDashboard => "experiment_run_dashboard",
            Self::RunComparisonView => "run_comparison_view",
            Self::DatasetCatalog => "dataset_catalog",
            Self::ShareReviewSheet => "share_review_sheet",
            Self::CliSurface => "cli_surface",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's run,
/// provenance, comparability, sensitivity, or export truth never silently narrows or widens
/// between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5ExperimentDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentConsumerSurface {
    /// The notebook UI.
    NotebookUi,
    /// The experiment-dashboard UI.
    ExperimentDashboardUi,
    /// The comparison UI.
    ComparisonUi,
    /// The data-catalog UI.
    DataCatalogUi,
    /// The lineage UI.
    LineageUi,
    /// The review UI.
    ReviewUi,
    /// The CLI surface.
    CliSurface,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5ExperimentConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::NotebookUi,
        Self::ExperimentDashboardUi,
        Self::ComparisonUi,
        Self::DataCatalogUi,
        Self::LineageUi,
        Self::ReviewUi,
        Self::CliSurface,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookUi => "notebook_ui",
            Self::ExperimentDashboardUi => "experiment_dashboard_ui",
            Self::ComparisonUi => "comparison_ui",
            Self::DataCatalogUi => "data_catalog_ui",
            Self::LineageUi => "lineage_ui",
            Self::ReviewUi => "review_ui",
            Self::CliSurface => "cli_surface",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no experiment truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5ExperimentAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed experiment component must be able to show. The first three are
/// hard requirements on every component; the remaining three close the acceptance-criteria
/// ambiguity about run origin / revision, dataset provenance / sensitivity, and export scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The run origin and code revision behind the component.
    RunOriginAndRevision,
    /// The dataset provenance and sensitivity posture of the component.
    ProvenanceAndSensitivity,
    /// The summary-versus-evidence-versus-raw export scope of the component.
    ExportScope,
}

impl M5ExperimentRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::RunOriginAndRevision,
        Self::ProvenanceAndSensitivity,
        Self::ExportScope,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::RunOriginAndRevision => "run_origin_and_revision",
            Self::ProvenanceAndSensitivity => "provenance_and_sensitivity",
            Self::ExportScope => "export_scope",
        }
    }
}

/// Qualification class for an M5 experiment-component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5ExperimentQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows an experiment component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentDowngradeTrigger {
    /// A run row left its origin unstated.
    RunOriginUnstated,
    /// A run row left its code revision unstated.
    CodeRevisionUnstated,
    /// An environment fingerprint card left its fingerprint unstated.
    EnvironmentFingerprintUnstated,
    /// A dataset provenance card severed its canonical provenance.
    DatasetProvenanceSevered,
    /// A run comparison table overstated comparability without parity evidence.
    ComparabilityOverstated,
    /// A sensitivity / sharing banner left its sensitivity class unstated.
    SensitivityClassUnstated,
    /// A result summary card left its export scope unstated.
    ExportScopeUnstated,
    /// A component exposed a raw payload by default.
    RawPayloadExposedByDefault,
    /// A component hid that its content is cached.
    CachedStateHidden,
    /// A component left an imported run unmarked.
    ImportedRunUnmarked,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5ExperimentDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::RunOriginUnstated,
        Self::CodeRevisionUnstated,
        Self::EnvironmentFingerprintUnstated,
        Self::DatasetProvenanceSevered,
        Self::ComparabilityOverstated,
        Self::SensitivityClassUnstated,
        Self::ExportScopeUnstated,
        Self::RawPayloadExposedByDefault,
        Self::CachedStateHidden,
        Self::ImportedRunUnmarked,
        Self::AlternateStateLabelInvented,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunOriginUnstated => "run_origin_unstated",
            Self::CodeRevisionUnstated => "code_revision_unstated",
            Self::EnvironmentFingerprintUnstated => "environment_fingerprint_unstated",
            Self::DatasetProvenanceSevered => "dataset_provenance_severed",
            Self::ComparabilityOverstated => "comparability_overstated",
            Self::SensitivityClassUnstated => "sensitivity_class_unstated",
            Self::ExportScopeUnstated => "export_scope_unstated",
            Self::RawPayloadExposedByDefault => "raw_payload_exposed_by_default",
            Self::CachedStateHidden => "cached_state_hidden",
            Self::ImportedRunUnmarked => "imported_run_unmarked",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed experiment-component family bound to the
/// surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentRow {
    /// Governed component family.
    pub component_family: M5ExperimentComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5ExperimentQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 notebook-adjacent / data-workflow surface families that render / consume
    /// this component.
    pub surface_families: Vec<M5ExperimentSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5ExperimentDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5ExperimentRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5ExperimentRequiredLabel>,
    /// Controlled dispositions this component binds (must be non-empty; drawn from the one
    /// shared [`M5ExperimentDisposition`] vocabulary).
    pub dispositions: Vec<M5ExperimentDisposition>,
    /// Run origin kinds this component names (experiment-run-row only).
    pub run_origin_kinds: Vec<M5RunOriginKind>,
    /// Run status states this component names (experiment-run-row only).
    pub run_status_states: Vec<M5RunStatusState>,
    /// Dataset source classes this component names (dataset-provenance-card only).
    pub dataset_source_classes: Vec<M5DatasetSourceClass>,
    /// Dataset provenance states this component names (dataset-provenance-card only).
    pub dataset_provenance_states: Vec<M5DatasetProvenanceState>,
    /// Artifact kind classes this component names (artifact-lineage-panel only).
    pub artifact_kind_classes: Vec<M5ArtifactKindClass>,
    /// Lineage states this component names (artifact-lineage-panel only).
    pub lineage_states: Vec<M5LineageState>,
    /// Comparison axis classes this component names (run-comparison-table only).
    pub comparison_axis_classes: Vec<M5ComparisonAxisClass>,
    /// Comparability states this component names (run-comparison-table only).
    pub comparability_states: Vec<M5ComparabilityState>,
    /// Fingerprint scope classes this component names (environment-fingerprint-card only).
    pub fingerprint_scope_classes: Vec<M5FingerprintScopeClass>,
    /// Fingerprint states this component names (environment-fingerprint-card only).
    pub fingerprint_states: Vec<M5FingerprintState>,
    /// Compare guard reasons this component names (compare-guard-banner only).
    pub compare_guard_reasons: Vec<M5CompareGuardReason>,
    /// Compare guard states this component names (compare-guard-banner only).
    pub compare_guard_states: Vec<M5CompareGuardState>,
    /// Sensitivity classes this component names (sensitivity-sharing-banner only).
    pub sensitivity_classes: Vec<M5SensitivityClass>,
    /// Share scope states this component names (sensitivity-sharing-banner only).
    pub share_scope_states: Vec<M5ShareScopeState>,
    /// Summary content classes this component names (result-summary-card only).
    pub summary_content_classes: Vec<M5SummaryContentClass>,
    /// Summary export scopes this component names (result-summary-card only).
    pub summary_export_scopes: Vec<M5SummaryExportScope>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5ExperimentAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its dataset provenance or sensitivity
    /// state. MUST be `false`.
    pub masks_provenance_or_sensitivity_state: bool,
    /// Hard invariant: this component never hides its run origin or code revision. MUST be
    /// `false`.
    pub hides_run_origin_or_revision: bool,
    /// Hard invariant: this component never implies an apples-to-apples comparison without
    /// parity evidence. MUST be `false`.
    pub implies_apples_to_apples_without_parity: bool,
    /// Hard invariant: this component never invents an alternate label for a governed state.
    /// MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl M5ExperimentComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ExperimentRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ExperimentRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_provenance_or_sensitivity_state
            && !self.hides_run_origin_or_revision
            && !self.implies_apples_to_apples_without_parity
            && !self.invents_alternate_state_label
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Disposition tokens (the one shared consumer vocabulary).
    pub dispositions: Vec<String>,
    /// Run-origin-kind tokens.
    pub run_origin_kinds: Vec<String>,
    /// Run-status-state tokens.
    pub run_status_states: Vec<String>,
    /// Dataset-source-class tokens.
    pub dataset_source_classes: Vec<String>,
    /// Dataset-provenance-state tokens.
    pub dataset_provenance_states: Vec<String>,
    /// Artifact-kind-class tokens.
    pub artifact_kind_classes: Vec<String>,
    /// Lineage-state tokens.
    pub lineage_states: Vec<String>,
    /// Comparison-axis-class tokens.
    pub comparison_axis_classes: Vec<String>,
    /// Comparability-state tokens.
    pub comparability_states: Vec<String>,
    /// Fingerprint-scope-class tokens.
    pub fingerprint_scope_classes: Vec<String>,
    /// Fingerprint-state tokens.
    pub fingerprint_states: Vec<String>,
    /// Compare-guard-reason tokens.
    pub compare_guard_reasons: Vec<String>,
    /// Compare-guard-state tokens.
    pub compare_guard_states: Vec<String>,
    /// Sensitivity-class tokens.
    pub sensitivity_classes: Vec<String>,
    /// Share-scope-state tokens.
    pub share_scope_states: Vec<String>,
    /// Summary-content-class tokens.
    pub summary_content_classes: Vec<String>,
    /// Summary-export-scope tokens.
    pub summary_export_scopes: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5ExperimentComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5ExperimentComponentFamily::ALL, |v| v.as_str()),
            dispositions: tokens(&M5ExperimentDisposition::ALL, |v| v.as_str()),
            run_origin_kinds: tokens(&M5RunOriginKind::ALL, |v| v.as_str()),
            run_status_states: tokens(&M5RunStatusState::ALL, |v| v.as_str()),
            dataset_source_classes: tokens(&M5DatasetSourceClass::ALL, |v| v.as_str()),
            dataset_provenance_states: tokens(&M5DatasetProvenanceState::ALL, |v| v.as_str()),
            artifact_kind_classes: tokens(&M5ArtifactKindClass::ALL, |v| v.as_str()),
            lineage_states: tokens(&M5LineageState::ALL, |v| v.as_str()),
            comparison_axis_classes: tokens(&M5ComparisonAxisClass::ALL, |v| v.as_str()),
            comparability_states: tokens(&M5ComparabilityState::ALL, |v| v.as_str()),
            fingerprint_scope_classes: tokens(&M5FingerprintScopeClass::ALL, |v| v.as_str()),
            fingerprint_states: tokens(&M5FingerprintState::ALL, |v| v.as_str()),
            compare_guard_reasons: tokens(&M5CompareGuardReason::ALL, |v| v.as_str()),
            compare_guard_states: tokens(&M5CompareGuardState::ALL, |v| v.as_str()),
            sensitivity_classes: tokens(&M5SensitivityClass::ALL, |v| v.as_str()),
            share_scope_states: tokens(&M5ShareScopeState::ALL, |v| v.as_str()),
            summary_content_classes: tokens(&M5SummaryContentClass::ALL, |v| v.as_str()),
            summary_export_scopes: tokens(&M5SummaryExportScope::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ExperimentSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ExperimentDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ExperimentConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ExperimentAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ExperimentRequiredLabel::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentGovernanceReview {
    /// The experiment run row shows its origin and code revision.
    pub run_row_shows_origin_and_revision: bool,
    /// The dataset provenance card shows its source and provenance state.
    pub dataset_card_shows_provenance_and_source: bool,
    /// The artifact lineage panel shows its upstream and downstream.
    pub lineage_panel_shows_upstream_and_downstream: bool,
    /// The run comparison table shows its comparability and confounders.
    pub comparison_table_shows_comparability_and_confounders: bool,
    /// The environment fingerprint card shows its captured environment.
    pub fingerprint_card_shows_environment_capture: bool,
    /// The compare guard banner shows its reason and guard state.
    pub compare_guard_shows_reason_and_state: bool,
    /// The sensitivity / sharing banner shows its sensitivity class and share scope.
    pub sensitivity_banner_shows_class_and_share_scope: bool,
    /// The result summary card shows its summary-versus-evidence-versus-raw export scope.
    pub result_summary_shows_export_scope: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// No comparison implies apples-to-apples without parity evidence.
    pub comparison_never_implies_apples_to_apples_without_parity: bool,
    /// No component widens export scope or exposes raw production-like data by default.
    pub no_component_widens_export_scope_or_exposes_raw_by_default: bool,
    /// Run identity and code revision stay explicit.
    pub run_identity_and_revision_always_explicit: bool,
    /// Sensitivity and provenance stay visible.
    pub sensitivity_and_provenance_always_visible: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel experiment vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentConsumerProjection {
    /// Notebook surfaces consume the shared run-row and fingerprint vocabulary.
    pub notebook_surfaces_consume_run_row_and_fingerprint_vocabulary: bool,
    /// Comparison surfaces consume the comparability and guard vocabulary.
    pub comparison_surfaces_consume_comparability_and_guard_vocabulary: bool,
    /// Data surfaces consume the provenance and lineage vocabulary.
    pub data_surfaces_consume_provenance_and_lineage_vocabulary: bool,
    /// Share surfaces consume the sensitivity and export-scope vocabulary.
    pub share_surfaces_consume_sensitivity_and_export_scope_vocabulary: bool,
    /// Result surfaces consume the summary-scope vocabulary.
    pub result_surfaces_consume_summary_scope_vocabulary: bool,
    /// Support / export reads a single canonical experiment source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the experiment-component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting experiment-component audit for the lane.
    pub experiment_component_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ExperimentComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ExperimentComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ExperimentComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ExperimentComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ExperimentComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ExperimentComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ExperimentComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ExperimentComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 experiment-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentMatrixPacket {
    /// Record kind; must equal [`M5_EXPERIMENT_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EXPERIMENT_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ExperimentComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ExperimentComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ExperimentComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ExperimentComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ExperimentComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ExperimentComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ExperimentComponentMatrixPacket {
    /// Builds an M5 experiment-component matrix packet from stable-lane input.
    pub fn new(input: M5ExperimentComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_EXPERIMENT_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_EXPERIMENT_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 experiment-component matrix invariants.
    pub fn validate(&self) -> Vec<M5ExperimentComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EXPERIMENT_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5ExperimentComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EXPERIMENT_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5ExperimentComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ExperimentComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 experiment component matrix packet serializes"),
        ) {
            violations.push(M5ExperimentComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 experiment component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,dispositions,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.dispositions, |v| v.as_str()),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Experiment-Run-Row, Dataset-Provenance-Card, Artifact-Lineage-Panel, Run-Comparison-Table, Environment-Fingerprint-Card, Compare-Guard-Banner, Sensitivity-Sharing-Banner, and Result-Summary-Card Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Dispositions: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Dispositions: {}\n",
                row.dispositions
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 experiment matrix export.
#[derive(Debug)]
pub enum M5ExperimentComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ExperimentComponentMatrixViolation>),
}

impl fmt::Display for M5ExperimentComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 experiment component matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 experiment component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ExperimentComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5ExperimentComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ExperimentComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component row declares no dispositions.
    DispositionsMissing,
    /// An experiment-run-row component declares no run origin kinds.
    RunOriginKindMissing,
    /// An experiment-run-row component declares no run status states.
    RunStatusStateMissing,
    /// A dataset-provenance-card component declares no dataset source classes.
    DatasetSourceClassMissing,
    /// A dataset-provenance-card component declares no dataset provenance states.
    DatasetProvenanceStateMissing,
    /// An artifact-lineage-panel component declares no artifact kind classes.
    ArtifactKindClassMissing,
    /// An artifact-lineage-panel component declares no lineage states.
    LineageStateMissing,
    /// A run-comparison-table component declares no comparison axis classes.
    ComparisonAxisClassMissing,
    /// A run-comparison-table component declares no comparability states.
    ComparabilityStateMissing,
    /// An environment-fingerprint-card component declares no fingerprint scope classes.
    FingerprintScopeClassMissing,
    /// An environment-fingerprint-card component declares no fingerprint states.
    FingerprintStateMissing,
    /// A compare-guard-banner component declares no compare guard reasons.
    CompareGuardReasonMissing,
    /// A compare-guard-banner component declares no compare guard states.
    CompareGuardStateMissing,
    /// A sensitivity-sharing-banner component declares no sensitivity classes.
    SensitivityClassMissing,
    /// A sensitivity-sharing-banner component declares no share scope states.
    ShareScopeStateMissing,
    /// A result-summary-card component declares no summary content classes.
    SummaryContentClassMissing,
    /// A result-summary-card component declares no summary export scopes.
    SummaryExportScopeMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (masked provenance / sensitivity state, hidden
    /// run origin / revision, implied apples-to-apples without parity, or invented alternate
    /// state label).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ExperimentComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::RunOriginKindMissing => "run_origin_kind_missing",
            Self::RunStatusStateMissing => "run_status_state_missing",
            Self::DatasetSourceClassMissing => "dataset_source_class_missing",
            Self::DatasetProvenanceStateMissing => "dataset_provenance_state_missing",
            Self::ArtifactKindClassMissing => "artifact_kind_class_missing",
            Self::LineageStateMissing => "lineage_state_missing",
            Self::ComparisonAxisClassMissing => "comparison_axis_class_missing",
            Self::ComparabilityStateMissing => "comparability_state_missing",
            Self::FingerprintScopeClassMissing => "fingerprint_scope_class_missing",
            Self::FingerprintStateMissing => "fingerprint_state_missing",
            Self::CompareGuardReasonMissing => "compare_guard_reason_missing",
            Self::CompareGuardStateMissing => "compare_guard_state_missing",
            Self::SensitivityClassMissing => "sensitivity_class_missing",
            Self::ShareScopeStateMissing => "share_scope_state_missing",
            Self::SummaryContentClassMissing => "summary_content_class_missing",
            Self::SummaryExportScopeMissing => "summary_export_scope_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 experiment matrix export.
pub fn current_stable_m5_experiment_component_matrix_export(
) -> Result<M5ExperimentComponentMatrixPacket, M5ExperimentComponentMatrixArtifactError> {
    let packet: M5ExperimentComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-experiment-component-proof/support_export.json"
    )))
    .map_err(M5ExperimentComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ExperimentComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ExperimentComponentMatrixPacket,
    violations: &mut Vec<M5ExperimentComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_DOC_REF,
        M5_EXPERIMENT_RUN_ROW_SCHEMA_REF,
        M5_DATASET_PROVENANCE_CARD_SCHEMA_REF,
        M5_ARTIFACT_LINEAGE_PANEL_SCHEMA_REF,
        M5_RUN_COMPARISON_TABLE_SCHEMA_REF,
        M5_ENVIRONMENT_FINGERPRINT_CARD_SCHEMA_REF,
        M5_COMPARE_GUARD_BANNER_SCHEMA_REF,
        M5_SENSITIVITY_SHARING_BANNER_SCHEMA_REF,
        M5_RESULT_SUMMARY_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ExperimentComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ExperimentComponentMatrixPacket,
    violations: &mut Vec<M5ExperimentComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ExperimentComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5ExperimentComponentMatrixPacket,
    violations: &mut Vec<M5ExperimentComponentMatrixViolation>,
) {
    let present: BTreeSet<M5ExperimentComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5ExperimentComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5ExperimentComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5ExperimentComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5ExperimentComponentMatrixViolation::MandatoryLabelMissing);
        }
        if row.dispositions.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::DispositionsMissing);
        }
        if family.is_experiment_run_row() && row.run_origin_kinds.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::RunOriginKindMissing);
        }
        if family.is_experiment_run_row() && row.run_status_states.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::RunStatusStateMissing);
        }
        if family.is_dataset_provenance_card() && row.dataset_source_classes.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::DatasetSourceClassMissing);
        }
        if family.is_dataset_provenance_card() && row.dataset_provenance_states.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::DatasetProvenanceStateMissing);
        }
        if family.is_artifact_lineage_panel() && row.artifact_kind_classes.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::ArtifactKindClassMissing);
        }
        if family.is_artifact_lineage_panel() && row.lineage_states.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::LineageStateMissing);
        }
        if family.is_run_comparison_table() && row.comparison_axis_classes.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::ComparisonAxisClassMissing);
        }
        if family.is_run_comparison_table() && row.comparability_states.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::ComparabilityStateMissing);
        }
        if family.is_environment_fingerprint_card() && row.fingerprint_scope_classes.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::FingerprintScopeClassMissing);
        }
        if family.is_environment_fingerprint_card() && row.fingerprint_states.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::FingerprintStateMissing);
        }
        if family.is_compare_guard_banner() && row.compare_guard_reasons.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::CompareGuardReasonMissing);
        }
        if family.is_compare_guard_banner() && row.compare_guard_states.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::CompareGuardStateMissing);
        }
        if family.is_sensitivity_sharing_banner() && row.sensitivity_classes.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::SensitivityClassMissing);
        }
        if family.is_sensitivity_sharing_banner() && row.share_scope_states.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::ShareScopeStateMissing);
        }
        if family.is_result_summary_card() && row.summary_content_classes.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::SummaryContentClassMissing);
        }
        if family.is_result_summary_card() && row.summary_export_scopes.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::SummaryExportScopeMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ExperimentComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ExperimentComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ExperimentComponentMatrixPacket,
    violations: &mut Vec<M5ExperimentComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.run_row_shows_origin_and_revision,
        review.dataset_card_shows_provenance_and_source,
        review.lineage_panel_shows_upstream_and_downstream,
        review.comparison_table_shows_comparability_and_confounders,
        review.fingerprint_card_shows_environment_capture,
        review.compare_guard_shows_reason_and_state,
        review.sensitivity_banner_shows_class_and_share_scope,
        review.result_summary_shows_export_scope,
        review.no_surface_invents_alternate_state_label,
        review.comparison_never_implies_apples_to_apples_without_parity,
        review.no_component_widens_export_scope_or_exposes_raw_by_default,
        review.run_identity_and_revision_always_explicit,
        review.sensitivity_and_provenance_always_visible,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ExperimentComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ExperimentComponentMatrixPacket,
    violations: &mut Vec<M5ExperimentComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.notebook_surfaces_consume_run_row_and_fingerprint_vocabulary,
        projection.comparison_surfaces_consume_comparability_and_guard_vocabulary,
        projection.data_surfaces_consume_provenance_and_lineage_vocabulary,
        projection.share_surfaces_consume_sensitivity_and_export_scope_vocabulary,
        projection.result_surfaces_consume_summary_scope_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ExperimentComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ExperimentComponentMatrixPacket,
    violations: &mut Vec<M5ExperimentComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ExperimentComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ExperimentComponentMatrixPacket,
    violations: &mut Vec<M5ExperimentComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.experiment_component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ExperimentComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
