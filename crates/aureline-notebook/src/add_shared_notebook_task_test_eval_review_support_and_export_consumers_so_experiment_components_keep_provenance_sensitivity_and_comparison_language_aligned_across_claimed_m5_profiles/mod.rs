//! Shared consumers for the reusable M5 experiment components, so the experiment run row, dataset
//! provenance card, artifact lineage panel, run comparison table, environment fingerprint card,
//! compare guard banner, sensitivity / sharing banner, and result summary card keep lineage /
//! provenance, sensitivity, comparability, and summary-versus-evidence-versus-raw export-scope
//! language aligned across every claimed M5 surface that renders a notebook-adjacent or data-bearing
//! result: the notebook run history, tasks / tests / evals, review evidence, a lightweight compare
//! view, the companion-safe summary, the CLI / headless export, and the support / export packet.
//!
//! Aureline's frozen experiment-component matrix
//! (`crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix`)
//! names the eight governed component families, and four sibling implement lanes narrow those
//! families into working primitives, each with its own canonical schema, contract doc, and
//! support-export artifact:
//!
//! * the experiment run row and environment fingerprint card
//!   (`implement_experiment_run_rows_and_environment_fingerprint_cards_...`),
//! * the dataset provenance card and sensitivity / sharing banner
//!   (`implement_dataset_provenance_cards_and_sensitivity_sharing_banners_...`),
//! * the artifact lineage panel and result summary card
//!   (`implement_artifact_lineage_panels_and_result_summary_cards_...`), and
//! * the run comparison table and compare guard banner
//!   (`implement_run_comparison_tables_and_compare_guard_banners_...`).
//!
//! This module is the *adoption* lane over those primitives. It proves the eight families are
//! reusable components — not one notebook page plus a few isolated data objects — by binding every
//! claimed M5 experiment consumer (the notebook run history, tasks / tests / evals, review
//! evidence, the compare view, the companion summary, the CLI / headless export, and the support /
//! export packet) to the same canonical component schemas and the same descriptor vocabulary. Each
//! consumer points at the primitive's canonical schema and support-export artifact rather than
//! re-wording lineage / provenance, sensitivity, comparability, or export-scope facts in local
//! prose, and each keeps that vocabulary truthful even when a run's producing lineage is incomplete,
//! a comparison lacks parity evidence, data is sensitive and redacted rather than raw, or an export
//! is metadata-only rather than a raw payload.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_experiment_component_binding`] — that takes one consumer's adoption of
//!    one component family, the descriptor set it surfaces, the parity-health mode it renders
//!    under, and any export caveats, and produces one [`M5ExperimentComponentResolvedBinding`]
//!    carrying the derived claim-parity state and — whenever parity is weakened — a self-contained
//!    [`M5ExperimentComponentAutoNarrowBanner`] that names the exact reason (incomplete lineage /
//!    provenance, an unproven comparison, restricted sensitive data, or a metadata-only export), the
//!    descriptors that stay preserved, and the recovery action, rather than a generic "degraded"
//!    note. The resolver never lets a narrowed context drop a required descriptor and never lets an
//!    unproven comparison masquerade as an apples-to-apples fair one.
//! 2. A parity matrix — [`M5ExperimentComponentConsumerPacket`] — that binds one row per claimed M5
//!    experiment consumer to the eight canonical component families, the one shared descriptor
//!    vocabulary, the same parity-health modes, export caveats, parity states, narrowing reasons,
//!    recovery actions, export fields, and non-visual accessibility routes, so lineage / provenance
//!    / sensitivity / comparability / export-scope facts stop diverging between the notebook UIs, the
//!    review panes, the support bundles, and the exported summaries.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes, qualification
//! classes, downgrade triggers, and the eight component families themselves are reused verbatim from
//! the frozen experiment-component matrix. This module mints new vocabulary only for what the
//! adoption lane itself needs: its experiment consumers, the shared descriptor vocabulary, the
//! parity-health modes, the export caveats, the claim-parity states, the narrowing reasons and
//! recovery actions, the consumer anatomy parts, and the export fields.
//!
//! Raw secrets, endpoints, tokens, and raw provider bodies stay outside the support boundary;
//! every label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is `schemas/ui/m5-experiment-component-consumer.schema.json` and the contract
//! doc is `docs/notebooks/m5_experiment_component_consumers.md`. The protected fixture directory is
//! `fixtures/ui/m5-experiment-component-consumers/`.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_experiment_component_consumer_compare_view_beta_narrowed,
    seeded_m5_experiment_component_consumer_packet,
    seeded_m5_experiment_component_consumer_review_evidence_preview_narrowed,
    M5_EXPERIMENT_COMPONENT_CONSUMER_PACKET_ID,
};

// The surface families, deployment lines, consumer surfaces, accessibility routes, qualification
// classes, downgrade triggers, and the eight component families are frozen once, in the
// experiment-component matrix. This adoption lane reuses them verbatim so it never invents a
// parallel experiment vocabulary.
pub use crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix::{
    M5ExperimentAccessibilityRoute, M5ExperimentComponentFamily, M5ExperimentConsumerSurface,
    M5ExperimentDeploymentLine, M5ExperimentDowngradeTrigger, M5ExperimentQualificationClass,
    M5ExperimentSurfaceFamily,
};

// The canonical matrix schema / doc refs this adoption lane points every consumer at, rather than
// re-wording their facts in local prose.
use crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix::{
    M5_EXPERIMENT_COMPONENT_DOC_REF, M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
};
// The canonical primitive schema / doc / artifact refs each family maps to.
use crate::implement_artifact_lineage_panels_and_result_summary_cards_with_producing_run_identity_stale_diverged_notes_include_raw_toggles_and_export_boundary_truth_across_claimed_m5_experiment_surfaces::{
    ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_ARTIFACT_REF,
    ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_DOC_REF,
    ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_REF,
};
use crate::implement_dataset_provenance_cards_and_sensitivity_sharing_banners_with_snapshot_sample_redaction_and_local_remote_location_truth_across_claimed_m5_data_lanes::{
    DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_ARTIFACT_REF,
    DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_DOC_REF,
    DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_REF,
};
use crate::implement_experiment_run_rows_and_environment_fingerprint_cards_with_run_origin_code_revision_execution_target_and_outcome_truth_across_claimed_m5_notebook_and_data_surfaces::{
    EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_ARTIFACT_REF,
    EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_DOC_REF,
    EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_REF,
};
use crate::implement_run_comparison_tables_and_compare_guard_banners_with_baseline_candidate_identity_confounder_disclosure_and_no_fair_delta_claims_when_parity_evidence_is_incomplete_across_claimed_m5_compare_flows::{
    RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_ARTIFACT_REF,
    RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_DOC_REF,
    RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ExperimentComponentConsumerPacket`].
pub const M5_EXPERIMENT_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_notebook_task_test_eval_review_support_and_export_consumers_so_experiment_components_keep_provenance_sensitivity_and_comparison_language_aligned_across_claimed_m5_profiles";

/// Schema version for M5 experiment component-consumer records.
pub const M5_EXPERIMENT_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the experiment component-consumer boundary schema.
pub const M5_EXPERIMENT_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-experiment-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EXPERIMENT_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/notebooks/m5_experiment_component_consumers.md";

/// Repo-relative path of the frozen experiment-component matrix this lane adopts from.
pub const M5_EXPERIMENT_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_EXPERIMENT_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_EXPERIMENT_COMPONENT_CONSUMER_OBJECT_MODEL_REF: &str = M5_EXPERIMENT_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_EXPERIMENT_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-experiment-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EXPERIMENT_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-experiment-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_EXPERIMENT_COMPONENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-experiment-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_EXPERIMENT_COMPONENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-experiment-component-consumer-proof/report.md";

/// The canonical boundary schema ref of the narrowed primitive that owns a family. A consumer that
/// adopts a family must point at this schema, not a local re-description.
pub const fn family_canonical_schema_ref(family: M5ExperimentComponentFamily) -> &'static str {
    use M5ExperimentComponentFamily as Family;
    match family {
        Family::ExperimentRunRow | Family::EnvironmentFingerprintCard => {
            EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_REF
        }
        Family::DatasetProvenanceCard | Family::SensitivitySharingBanner => {
            DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_REF
        }
        Family::ArtifactLineagePanel | Family::ResultSummaryCard => {
            ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_REF
        }
        Family::RunComparisonTable | Family::CompareGuardBanner => {
            RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_REF
        }
    }
}

/// The canonical contract-doc ref of the narrowed primitive that owns a family.
pub const fn family_canonical_doc_ref(family: M5ExperimentComponentFamily) -> &'static str {
    use M5ExperimentComponentFamily as Family;
    match family {
        Family::ExperimentRunRow | Family::EnvironmentFingerprintCard => {
            EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_DOC_REF
        }
        Family::DatasetProvenanceCard | Family::SensitivitySharingBanner => {
            DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_DOC_REF
        }
        Family::ArtifactLineagePanel | Family::ResultSummaryCard => {
            ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_DOC_REF
        }
        Family::RunComparisonTable | Family::CompareGuardBanner => {
            RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_DOC_REF
        }
    }
}

/// The canonical support-export artifact ref of the narrowed primitive that owns a family.
pub const fn family_canonical_artifact_ref(family: M5ExperimentComponentFamily) -> &'static str {
    use M5ExperimentComponentFamily as Family;
    match family {
        Family::ExperimentRunRow | Family::EnvironmentFingerprintCard => {
            EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_ARTIFACT_REF
        }
        Family::DatasetProvenanceCard | Family::SensitivitySharingBanner => {
            DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_ARTIFACT_REF
        }
        Family::ArtifactLineagePanel | Family::ResultSummaryCard => {
            ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_ARTIFACT_REF
        }
        Family::RunComparisonTable | Family::CompareGuardBanner => {
            RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_ARTIFACT_REF
        }
    }
}

/// One claimed M5 experiment consumer that adopts the shared components. These are the consumers the
/// spec names — the notebook run history, tasks / tests / evals, review evidence, a lightweight
/// compare view, the companion-safe summary, the CLI / headless export, and the support / export
/// packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentComponentConsumer {
    /// The notebook run-history surface.
    NotebookRunHistory,
    /// The tasks / tests / evals surface.
    TaskTestEvalRuns,
    /// The review-evidence surface.
    ReviewEvidence,
    /// The lightweight compare view.
    CompareView,
    /// The companion-safe summary.
    CompanionSummary,
    /// The CLI / headless export surface.
    CliHeadlessExport,
    /// The support / export packet.
    SupportExport,
}

impl M5ExperimentComponentConsumer {
    /// Every claimed experiment consumer, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::NotebookRunHistory,
        Self::TaskTestEvalRuns,
        Self::ReviewEvidence,
        Self::CompareView,
        Self::CompanionSummary,
        Self::CliHeadlessExport,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookRunHistory => "notebook_run_history",
            Self::TaskTestEvalRuns => "task_test_eval_runs",
            Self::ReviewEvidence => "review_evidence",
            Self::CompareView => "compare_view",
            Self::CompanionSummary => "companion_summary",
            Self::CliHeadlessExport => "cli_headless_export",
            Self::SupportExport => "support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotebookRunHistory => "Notebook Run History",
            Self::TaskTestEvalRuns => "Tasks / Tests / Evals",
            Self::ReviewEvidence => "Review Evidence",
            Self::CompareView => "Compare View",
            Self::CompanionSummary => "Companion Summary",
            Self::CliHeadlessExport => "CLI / Headless Export",
            Self::SupportExport => "Support / Export Packet",
        }
    }

    /// True when this consumer is the support / export packet — the surface singled out for a
    /// canonical-schema reference so its prose can never drift from the product truth.
    pub const fn is_support_or_export(self) -> bool {
        matches!(self, Self::SupportExport)
    }
}

/// The one shared descriptor vocabulary every experiment component keeps aligned across surfaces, so
/// no consumer invents a new grammar or stale wording. The descriptors in
/// [`M5ExperimentComponentDescriptor::REQUIRED`] must be present on every binding — the
/// acceptance-criterion that lineage / provenance, sensitivity, comparability, and export-scope
/// language stay one truth across notebook UIs, review panes, support bundles, and exported
/// summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentComponentDescriptor {
    /// The lineage / provenance descriptor: run identity, notebook / script / task origin, code
    /// revision, environment fingerprint, dataset provenance, and artifact lineage.
    LineageProvenance,
    /// The sensitivity-state descriptor: sensitivity class and redaction posture.
    SensitivityState,
    /// The comparability descriptor: comparability and confounder disclosure.
    Comparability,
    /// The export-scope descriptor: summary versus evidence versus raw payload.
    ExportScope,
}

impl M5ExperimentComponentDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LineageProvenance,
        Self::SensitivityState,
        Self::Comparability,
        Self::ExportScope,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LineageProvenance => "lineage_provenance",
            Self::SensitivityState => "sensitivity_state",
            Self::Comparability => "comparability",
            Self::ExportScope => "export_scope",
        }
    }
}

/// The parity-health mode a consumer renders a component under. A weakened mode still keeps the
/// descriptor vocabulary — it only discloses that parity is narrowed relative to the authoritative
/// rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentConsumerParityHealth {
    /// Full parity: the authoritative rendering.
    FullParity,
    /// A run's producing lineage / provenance is incomplete, so origin, revision, or fingerprint is
    /// disclosed as a gap rather than assumed.
    ProvenanceIncompleteNarrowed,
    /// A comparison lacks parity evidence, so it is disclosed as not apples-to-apples rather than a
    /// fair baseline.
    NotComparableNarrowed,
    /// Data is sensitive, so it is redacted and never exposed raw by default.
    SensitivityRestrictedNarrowed,
    /// An export carries metadata only, so it is disclosed as metadata rather than a raw payload.
    MetadataOnlyExportNarrowed,
}

impl M5ExperimentConsumerParityHealth {
    /// Every parity-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::ProvenanceIncompleteNarrowed,
        Self::NotComparableNarrowed,
        Self::SensitivityRestrictedNarrowed,
        Self::MetadataOnlyExportNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::ProvenanceIncompleteNarrowed => "provenance_incomplete_narrowed",
            Self::NotComparableNarrowed => "not_comparable_narrowed",
            Self::SensitivityRestrictedNarrowed => "sensitivity_restricted_narrowed",
            Self::MetadataOnlyExportNarrowed => "metadata_only_export_narrowed",
        }
    }

    /// True when the mode renders below the authoritative full parity and so must disclose a
    /// self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5ExperimentConsumerNarrowingReason> {
        Some(match self {
            Self::ProvenanceIncompleteNarrowed => {
                M5ExperimentConsumerNarrowingReason::LineageProvenanceIncomplete
            }
            Self::NotComparableNarrowed => {
                M5ExperimentConsumerNarrowingReason::ComparabilityUnproven
            }
            Self::SensitivityRestrictedNarrowed => {
                M5ExperimentConsumerNarrowingReason::SensitiveDataRestricted
            }
            Self::MetadataOnlyExportNarrowed => {
                M5ExperimentConsumerNarrowingReason::ExportMetadataOnly
            }
            Self::FullParity => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its parity claim language, so an auto-narrow banner
/// never reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentConsumerNarrowingReason {
    /// A run's producing lineage / provenance is incomplete.
    LineageProvenanceIncomplete,
    /// A comparison lacks parity evidence, so it cannot claim an apples-to-apples fair baseline.
    ComparabilityUnproven,
    /// Data is sensitive, so it is redacted and never exposed raw.
    SensitiveDataRestricted,
    /// An export carries metadata only, not a raw payload.
    ExportMetadataOnly,
}

impl M5ExperimentConsumerNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LineageProvenanceIncomplete,
        Self::ComparabilityUnproven,
        Self::SensitiveDataRestricted,
        Self::ExportMetadataOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LineageProvenanceIncomplete => "lineage_provenance_incomplete",
            Self::ComparabilityUnproven => "comparability_unproven",
            Self::SensitiveDataRestricted => "sensitive_data_restricted",
            Self::ExportMetadataOnly => "export_metadata_only",
        }
    }

    /// True when the reason reflects a comparison whose parity evidence is incomplete and which must
    /// never masquerade as an apples-to-apples fair baseline — the acceptance-criterion boundary that
    /// a metric delta never implies a fair comparison without parity evidence.
    pub const fn is_unproven_comparability(self) -> bool {
        matches!(self, Self::ComparabilityUnproven)
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::LineageProvenanceIncomplete => {
                "a run's producing lineage / provenance is incomplete, so its origin, revision, or fingerprint is disclosed as a gap rather than assumed"
            }
            Self::ComparabilityUnproven => {
                "a comparison lacks parity evidence, so it is disclosed as not apples-to-apples and a metric delta never implies a fair baseline"
            }
            Self::SensitiveDataRestricted => {
                "data is sensitive, so it is redacted and never exposed raw by default in this rendering"
            }
            Self::ExportMetadataOnly => {
                "this export carries metadata only, so it is disclosed as metadata rather than a raw payload"
            }
        }
    }

    /// The recovery action a reader should take before trusting full parity.
    pub const fn recovery_action(self) -> M5ExperimentConsumerRecoveryAction {
        match self {
            Self::LineageProvenanceIncomplete => {
                M5ExperimentConsumerRecoveryAction::OpenProducingRunOrCompleteLineage
            }
            Self::ComparabilityUnproven => {
                M5ExperimentConsumerRecoveryAction::ReviewComparabilityBeforeTrustingDelta
            }
            Self::SensitiveDataRestricted => {
                M5ExperimentConsumerRecoveryAction::ReviewSensitivityBeforeSharing
            }
            Self::ExportMetadataOnly => {
                M5ExperimentConsumerRecoveryAction::RequestFullEvidenceExportIfPermitted
            }
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is actionable from
/// the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentConsumerRecoveryAction {
    /// Open the producing run, or complete the lineage, before trusting incomplete provenance.
    OpenProducingRunOrCompleteLineage,
    /// Review comparability and confounders before treating a metric delta as a fair baseline.
    ReviewComparabilityBeforeTrustingDelta,
    /// Review the sensitivity class before sharing, rather than exposing raw data.
    ReviewSensitivityBeforeSharing,
    /// Request the full evidence export if permitted, rather than treating metadata as the raw
    /// payload.
    RequestFullEvidenceExportIfPermitted,
}

impl M5ExperimentConsumerRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpenProducingRunOrCompleteLineage,
        Self::ReviewComparabilityBeforeTrustingDelta,
        Self::ReviewSensitivityBeforeSharing,
        Self::RequestFullEvidenceExportIfPermitted,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenProducingRunOrCompleteLineage => "open_producing_run_or_complete_lineage",
            Self::ReviewComparabilityBeforeTrustingDelta => {
                "review_comparability_before_trusting_delta"
            }
            Self::ReviewSensitivityBeforeSharing => "review_sensitivity_before_sharing",
            Self::RequestFullEvidenceExportIfPermitted => {
                "request_full_evidence_export_if_permitted"
            }
        }
    }
}

/// An export caveat a consumer preserves when a component renders below full parity (incomplete
/// lineage / provenance, an unproven comparison, restricted sensitive data, or a metadata-only
/// export).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentConsumerExportCaveat {
    /// The run's producing lineage / provenance is incomplete.
    LineageProvenanceIncomplete,
    /// The comparison is not apples-to-apples.
    ComparisonNotApplesToApples,
    /// The data is sensitive and redacted, not raw.
    SensitiveDataRedactedNotRaw,
    /// The export is metadata-only, not raw.
    ExportMetadataOnlyNotRaw,
}

impl M5ExperimentConsumerExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LineageProvenanceIncomplete,
        Self::ComparisonNotApplesToApples,
        Self::SensitiveDataRedactedNotRaw,
        Self::ExportMetadataOnlyNotRaw,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LineageProvenanceIncomplete => "lineage_provenance_incomplete",
            Self::ComparisonNotApplesToApples => "comparison_not_apples_to_apples",
            Self::SensitiveDataRedactedNotRaw => "sensitive_data_redacted_not_raw",
            Self::ExportMetadataOnlyNotRaw => "export_metadata_only_not_raw",
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor vocabulary is kept
/// aligned as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentClaimParityState {
    /// The descriptor vocabulary is kept aligned at full parity.
    ClaimsAligned,
    /// The descriptor vocabulary is kept aligned, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5ExperimentClaimParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [Self::ClaimsAligned, Self::ClaimsAutoNarrowed];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimsAligned => "claims_aligned",
            Self::ClaimsAutoNarrowed => "claims_auto_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5ExperimentConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentConsumerAnatomyPart {
    /// The adopted component identity.
    ComponentIdentity,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The shared descriptor set.
    DescriptorSet,
    /// The parity-health cue.
    ParityHealthCue,
    /// The export-caveat list.
    ExportCaveats,
    /// The derived claim-parity verdict.
    ClaimParityVerdict,
    /// The auto-narrow banner (shown when narrowed).
    AutoNarrowBanner,
}

impl M5ExperimentConsumerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealthCue,
        Self::ExportCaveats,
        Self::ClaimParityVerdict,
        Self::AutoNarrowBanner,
    ];

    /// The anatomy parts every consumer projection must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentIdentity => "component_identity",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealthCue => "parity_health_cue",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityVerdict => "claim_parity_verdict",
            Self::AutoNarrowBanner => "auto_narrow_banner",
        }
    }
}

/// A field the support / export packet carries so consumer parity is reconstructable from the
/// shared model. The fields in [`M5ExperimentConsumerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentConsumerExportField {
    /// The consumer identity.
    Consumer,
    /// The adopted component family.
    ComponentFamily,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The descriptor set.
    DescriptorSet,
    /// The parity-health mode.
    ParityHealth,
    /// The export caveats.
    ExportCaveats,
    /// The claim-parity state.
    ClaimParityState,
    /// The narrowing reason (when narrowed).
    NarrowingReason,
}

impl M5ExperimentConsumerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealth,
        Self::ExportCaveats,
        Self::ClaimParityState,
        Self::NarrowingReason,
    ];

    /// The export fields every consumer export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consumer => "consumer",
            Self::ComponentFamily => "component_family",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealth => "parity_health",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityState => "claim_parity_state",
            Self::NarrowingReason => "narrowing_reason",
        }
    }
}

/// A self-contained auto-narrow banner: the exact reason, the descriptors that stay preserved, the
/// export caveats, and the recovery action, so a narrowed rendering is understood from the banner
/// alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5ExperimentConsumerNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5ExperimentConsumerRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5ExperimentComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5ExperimentComponentFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5ExperimentComponentDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5ExperimentConsumerExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved descriptors, and
    /// the recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the experiment component-binding resolver for one consumer/family adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5ExperimentComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5ExperimentComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor so lineage /
    /// provenance, sensitivity, comparability, and export-scope stay explicit.
    pub descriptor_families: Vec<M5ExperimentComponentDescriptor>,
    /// The parity-health mode the binding renders under.
    pub parity_health: M5ExperimentConsumerParityHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5ExperimentConsumerExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentResolvedBinding {
    /// The consumer.
    pub consumer: M5ExperimentComponentConsumer,
    /// The component family.
    pub component_family: M5ExperimentComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5ExperimentComponentDescriptor>,
    /// The parity-health mode.
    pub parity_health: M5ExperimentConsumerParityHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5ExperimentConsumerExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5ExperimentClaimParityState,
    /// True when the binding renders under a weakened parity-health mode.
    pub is_narrowed: bool,
    /// True when the binding reflects a comparison whose parity evidence is incomplete. Such a
    /// binding must always be narrowed and never asserts an apples-to-apples fair comparison.
    pub reflects_unproven_comparability: bool,
    /// Hard invariant: whether this binding asserts an apples-to-apples fair comparison at full
    /// parity. Only a full-parity binding may assert it; every narrowed binding — and in particular
    /// any unproven-comparability one — resolves this to `false`.
    pub asserts_apples_to_apples_parity: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5ExperimentComponentAutoNarrowBanner>,
}

/// Errors returned by [`resolve_experiment_component_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ExperimentComponentBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5ExperimentComponentBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5ExperimentComponentBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "experiment component binding error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ExperimentComponentBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the acceptance-criterion that lineage / provenance,
/// sensitivity, comparability, and export-scope stay explicit on every surface. The claim-parity
/// state is kept aligned at full parity and auto-narrowed under any weakened parity-health mode, and
/// a weakened mode always produces a self-contained banner naming the exact reason and recovery
/// action while keeping the descriptor vocabulary intact. An unproven comparison always narrows and
/// never asserts an apples-to-apples fair comparison.
pub fn resolve_experiment_component_binding(
    input: &M5ExperimentComponentBindingInput,
) -> Result<M5ExperimentComponentResolvedBinding, M5ExperimentComponentBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5ExperimentComponentBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5ExperimentComponentDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5ExperimentComponentDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5ExperimentComponentBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5ExperimentComponentBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future free-text extension
        // from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5ExperimentComponentBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.parity_health.is_narrowed();
    let narrowing_reason = input.parity_health.narrowing_reason();
    let reflects_unproven_comparability = narrowing_reason
        .is_some_and(M5ExperimentConsumerNarrowingReason::is_unproven_comparability);
    // Only a full-parity binding may assert an apples-to-apples fair comparison. Every narrowed
    // binding — and every unproven-comparability one in particular — does not.
    let asserts_apples_to_apples_parity = !is_narrowed;
    let claim_parity_state = if is_narrowed {
        M5ExperimentClaimParityState::ClaimsAutoNarrowed
    } else {
        M5ExperimentClaimParityState::ClaimsAligned
    };

    let auto_narrow_banner = narrowing_reason.map(|reason| {
        let recovery_action = reason.recovery_action();
        let headline = format!(
            "Claim auto-narrowed: {} — {} renders {} with {} descriptor(s) preserved; recovery: {}",
            reason.phrase(),
            input.consumer.as_str(),
            input.component_family.as_str(),
            input.descriptor_families.len(),
            recovery_action.as_str()
        );
        M5ExperimentComponentAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5ExperimentComponentResolvedBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: family_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        parity_health: input.parity_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        reflects_unproven_comparability,
        asserts_apples_to_apples_parity,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet reconstructs
/// consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentBindingCase {
    /// The resolver input.
    pub input: M5ExperimentComponentBindingInput,
    /// The resolved truth. Must equal `resolve_experiment_component_binding(&input)`.
    pub resolved: M5ExperimentComponentResolvedBinding,
}

impl M5ExperimentComponentBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ExperimentComponentBindingInput) -> Self {
        let resolved =
            resolve_experiment_component_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_experiment_component_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the consumer
/// points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5ExperimentComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's canonical schema
    /// ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal the family's
    /// canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local re-description of
    /// its facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5ExperimentComponentBindingCase>,
}

impl M5ExperimentComponentBinding {
    /// True when the binding points at the family's canonical refs and references the canonical
    /// family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == family_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref == family_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one experiment consumer bound to the canonical component
/// families, the shared descriptor vocabulary, the parity-health modes, export caveats, parity
/// states, narrowing reasons, recovery actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentConsumerRow {
    /// Experiment consumer.
    pub consumer: M5ExperimentComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5ExperimentQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 experiment surface families that render / consume this projection.
    pub surface_families: Vec<M5ExperimentSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5ExperimentDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5ExperimentConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5ExperimentComponentDescriptor>,
    /// Parity-health modes this consumer distinguishes.
    pub parity_health_modes: Vec<M5ExperimentConsumerParityHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5ExperimentConsumerExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5ExperimentClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5ExperimentConsumerNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5ExperimentConsumerRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5ExperimentConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5ExperimentAccessibilityRoute>,
    /// Experiment subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5ExperimentComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the claims per surface. MUST be `false`.
    pub rewords_claims_per_surface: bool,
    /// Hard invariant: this consumer never invents a new experiment grammar. MUST be `false`.
    pub invents_new_experiment_grammar: bool,
    /// Hard invariant: this consumer never drops lineage / provenance, sensitivity, or comparability
    /// truth when narrowed. MUST be `false`.
    pub drops_lineage_sensitivity_or_comparability_when_narrowed: bool,
    /// Hard invariant: this consumer never implies an apples-to-apples comparison without parity
    /// evidence. MUST be `false`.
    pub implies_apples_to_apples_without_parity: bool,
    /// Hard invariant: this consumer never exposes a raw payload by default. MUST be `false`.
    pub exposes_raw_payload_by_default: bool,
}

impl M5ExperimentComponentConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ExperimentConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ExperimentConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ExperimentConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5ExperimentConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5ExperimentComponentDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5ExperimentComponentDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5ExperimentComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5ExperimentComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_claims_per_surface
            && !self.invents_new_experiment_grammar
            && !self.drops_lineage_sensitivity_or_comparability_when_narrowed
            && !self.implies_apples_to_apples_without_parity
            && !self.exposes_raw_payload_by_default
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentConsumerVocabularySet {
    /// Experiment-consumer tokens.
    pub consumers: Vec<String>,
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Descriptor tokens.
    pub descriptors: Vec<String>,
    /// Parity-health-mode tokens.
    pub parity_health_modes: Vec<String>,
    /// Export-caveat tokens.
    pub export_caveats: Vec<String>,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Recovery-action tokens.
    pub recovery_actions: Vec<String>,
    /// Claim-parity-state tokens.
    pub claim_parity_states: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5ExperimentComponentConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5ExperimentComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5ExperimentComponentFamily::ALL, |v| v.as_str()),
            descriptors: tokens(&M5ExperimentComponentDescriptor::ALL, |v| v.as_str()),
            parity_health_modes: tokens(&M5ExperimentConsumerParityHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5ExperimentConsumerExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5ExperimentConsumerNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5ExperimentConsumerRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5ExperimentClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ExperimentConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ExperimentConsumerExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ExperimentAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5ExperimentComponentConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new experiment grammar.
    pub no_consumer_invents_new_grammar: bool,
    /// Lineage / provenance, sensitivity, comparability, and export-scope stay explicit everywhere.
    pub lineage_sensitivity_comparability_and_export_scope_explicit_on_every_surface: bool,
    /// Incomplete provenance, unproven comparisons, restricted sensitive data, and metadata-only
    /// exports auto-narrow the claim.
    pub degraded_state_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// A comparison never implies an apples-to-apples fair baseline without parity evidence.
    pub comparison_never_implies_apples_to_apples_without_parity: bool,
    /// The support / export packet presents the same experiment truth shown in-product.
    pub support_export_presents_same_experiment_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentConsumerProjection {
    /// The notebook run history, tasks / tests / evals, review evidence, the compare view, the
    /// companion summary, the CLI / headless export, and the support / export packet all adopt the
    /// shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The lineage / provenance descriptor reads a single canonical source.
    pub lineage_provenance_reads_single_source: bool,
    /// The sensitivity-state descriptor reads a single canonical source.
    pub sensitivity_state_reads_single_source: bool,
    /// The comparability descriptor reads a single canonical source.
    pub comparability_reads_single_source: bool,
    /// The export-scope descriptor reads a single canonical source.
    pub export_scope_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting experiment-component consumer audit.
    pub experiment_component_consumer_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ExperimentComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ExperimentComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5ExperimentComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ExperimentComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ExperimentComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ExperimentComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ExperimentComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ExperimentComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 experiment component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExperimentComponentConsumerPacket {
    /// Record kind; must equal [`M5_EXPERIMENT_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EXPERIMENT_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5ExperimentComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ExperimentComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ExperimentComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ExperimentComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ExperimentComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ExperimentComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ExperimentComponentConsumerPacket {
    /// Builds an M5 experiment component-consumer packet from stable-lane input.
    pub fn new(input: M5ExperimentComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_EXPERIMENT_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_EXPERIMENT_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
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

    /// Validates the M5 experiment component-consumer invariants.
    pub fn validate(&self) -> Vec<M5ExperimentComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EXPERIMENT_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5ExperimentComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EXPERIMENT_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5ExperimentComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ExperimentComponentConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_comparability_honesty(self, &mut violations);
        validate_support_export_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 experiment component consumer packet serializes"),
        ) {
            violations.push(M5ExperimentComponentConsumerViolation::RawMaterialInExport);
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
            .expect("m5 experiment component consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,qualification,owner,adopted_families,parity_health_modes,claim_parity_states,narrowing_reasons,export_fields,binding_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.component_bindings, |b| b.component_family.as_str()),
                join_tokens(&row.parity_health_modes, |v| v.as_str()),
                join_tokens(&row.claim_parity_states, |v| v.as_str()),
                join_tokens(&row.narrowing_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.component_bindings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Experiment Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Experiment consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Component families: {}\n",
            self.vocabulary_set.component_families.join(", ")
        ));
        out.push_str(&format!(
            "- Descriptors: {}\n",
            self.vocabulary_set.descriptors.join(", ")
        ));
        out.push_str(&format!(
            "- Parity-health modes: {}\n",
            self.vocabulary_set.parity_health_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Experiment consumers\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Adopted families: {}\n",
                row.component_bindings.len()
            ));
            for binding in &row.component_bindings {
                out.push_str(&format!(
                    "    - `{}` → `{}` ({} worked binding(s))\n",
                    binding.component_family.as_str(),
                    binding.canonical_schema_ref,
                    binding.example_bindings.len()
                ));
                for case in &binding.example_bindings {
                    let banner = match &case.resolved.auto_narrow_banner {
                        Some(banner) => banner.reason.as_str(),
                        None => "full",
                    };
                    out.push_str(&format!(
                        "      - `{}` → `{}` (banner `{}`)\n",
                        case.resolved.parity_health.as_str(),
                        case.resolved.claim_parity_state.as_str(),
                        banner
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 experiment component-consumer export.
#[derive(Debug)]
pub enum M5ExperimentComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ExperimentComponentConsumerViolation>),
}

impl fmt::Display for M5ExperimentComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 experiment component consumer export parse failed: {error}"
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
                    "m5 experiment component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ExperimentComponentConsumerArtifactError {}

/// Validation failures emitted by [`M5ExperimentComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ExperimentComponentConsumerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required experiment consumer is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A consumer row does not keep every required descriptor.
    RequiredDescriptorMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no component bindings.
    ComponentBindingMissing,
    /// A component binding does not point to its canonical family.
    CanonicalRefMismatch,
    /// A component binding declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// A required component family is never adopted, or is adopted by only one consumer (reuse
    /// across surfaces unproven).
    ComponentFamilyReuseUnproven,
    /// No worked binding proves a narrowed rendering with a self-contained banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-parity rendering with preserved parity and no banner.
    ScopePreservedUnproven,
    /// No worked binding proves that an unproven comparison narrows and never asserts an
    /// apples-to-apples fair comparison, or a binding does so incorrectly.
    ComparabilityHonestyUnproven,
    /// The support / export packet consumer does not reference the canonical component schema.
    SupportExportReferenceMissing,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ExperimentComponentConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::RequiredDescriptorMissing => "required_descriptor_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ComponentBindingMissing => "component_binding_missing",
            Self::CanonicalRefMismatch => "canonical_ref_mismatch",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ComponentFamilyReuseUnproven => "component_family_reuse_unproven",
            Self::NarrowingDisclosureUnproven => "narrowing_disclosure_unproven",
            Self::ScopePreservedUnproven => "scope_preserved_unproven",
            Self::ComparabilityHonestyUnproven => "comparability_honesty_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 experiment component-consumer export.
pub fn current_stable_m5_experiment_component_consumer_export(
) -> Result<M5ExperimentComponentConsumerPacket, M5ExperimentComponentConsumerArtifactError> {
    let packet: M5ExperimentComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-experiment-component-consumer-proof/support_export.json"
    )))
    .map_err(M5ExperimentComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ExperimentComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ExperimentComponentConsumerPacket,
    violations: &mut Vec<M5ExperimentComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EXPERIMENT_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_CONSUMER_DOC_REF,
        M5_EXPERIMENT_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_EXPERIMENT_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_REF,
        DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_REF,
        ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_REF,
        RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ExperimentComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ExperimentComponentConsumerPacket,
    violations: &mut Vec<M5ExperimentComponentConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ExperimentComponentConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5ExperimentComponentConsumerPacket,
    violations: &mut Vec<M5ExperimentComponentConsumerViolation>,
) {
    let present: BTreeSet<M5ExperimentComponentConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5ExperimentComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5ExperimentComponentConsumerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.parity_health_modes.is_empty()
            || row.export_caveats.is_empty()
            || row.claim_parity_states.is_empty()
            || row.narrowing_reasons.is_empty()
            || row.recovery_actions.is_empty()
        {
            violations.push(M5ExperimentComponentConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ExperimentComponentConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5ExperimentComponentConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ExperimentComponentConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ExperimentAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ExperimentComponentConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ExperimentComponentConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ExperimentComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5ExperimentComponentConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5ExperimentComponentConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5ExperimentComponentConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5ExperimentComponentConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ExperimentComponentConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ExperimentComponentConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct consumers — the
/// acceptance-criterion proof that the families are reusable components rather than one notebook page
/// plus a few isolated data objects.
fn validate_family_reuse(
    packet: &M5ExperimentComponentConsumerPacket,
    violations: &mut Vec<M5ExperimentComponentConsumerViolation>,
) {
    for family in M5ExperimentComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5ExperimentComponentConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering whose banner
/// carries a specific reason, a recovery action, and a non-empty set of preserved descriptors — the
/// acceptance-criterion example that a consumer which cannot preserve parity is visibly narrowed
/// rather than silently dropping lineage, sensitivity, or comparability language.
fn validate_narrowing_disclosure(
    packet: &M5ExperimentComponentConsumerPacket,
    violations: &mut Vec<M5ExperimentComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        case.resolved.is_narrowed
            && case
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|banner| {
                    !banner.headline.trim().is_empty() && !banner.preserved_descriptors.is_empty()
                })
    });
    if !proven {
        violations.push(M5ExperimentComponentConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-parity rendering with preserved
/// parity and no banner — the acceptance-criterion example that full-parity consumers keep the
/// descriptor vocabulary without a spurious narrowing note.
fn validate_scope_preserved(
    packet: &M5ExperimentComponentConsumerPacket,
    violations: &mut Vec<M5ExperimentComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5ExperimentClaimParityState::ClaimsAligned
    });
    if !proven {
        violations.push(M5ExperimentComponentConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every worked binding that reflects an unproven comparison must be narrowed and must not assert an
/// apples-to-apples fair comparison, and at least one such binding must be present — the
/// acceptance-criterion that a metric delta no longer implies a fair baseline without parity
/// evidence on any claimed consumer.
fn validate_comparability_honesty(
    packet: &M5ExperimentComponentConsumerPacket,
    violations: &mut Vec<M5ExperimentComponentConsumerViolation>,
) {
    let mut proven = false;
    for case in all_cases(packet) {
        let resolved = &case.resolved;
        if resolved.reflects_unproven_comparability {
            // An unproven-comparability binding that claims apples-to-apples parity, or fails to
            // narrow, breaks the acceptance criterion.
            if resolved.asserts_apples_to_apples_parity
                || !resolved.is_narrowed
                || resolved.claim_parity_state != M5ExperimentClaimParityState::ClaimsAutoNarrowed
            {
                violations
                    .push(M5ExperimentComponentConsumerViolation::ComparabilityHonestyUnproven);
                return;
            }
            proven = true;
        }
    }
    if !proven {
        violations.push(M5ExperimentComponentConsumerViolation::ComparabilityHonestyUnproven);
    }
}

/// The support / export packet consumer must reference the canonical component schema for each
/// family it adopts — the acceptance-criterion that a support / export lane can never drift from
/// the product truth.
fn validate_support_export_reference(
    packet: &M5ExperimentComponentConsumerPacket,
    violations: &mut Vec<M5ExperimentComponentConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_support_or_export() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5ExperimentComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5ExperimentComponentConsumerViolation::SupportExportReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5ExperimentComponentConsumerPacket,
    violations: &mut Vec<M5ExperimentComponentConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_grammar,
        review.lineage_sensitivity_comparability_and_export_scope_explicit_on_every_surface,
        review.degraded_state_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.comparison_never_implies_apples_to_apples_without_parity,
        review.support_export_presents_same_experiment_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ExperimentComponentConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ExperimentComponentConsumerPacket,
    violations: &mut Vec<M5ExperimentComponentConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.lineage_provenance_reads_single_source,
        projection.sensitivity_state_reads_single_source,
        projection.comparability_reads_single_source,
        projection.export_scope_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ExperimentComponentConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ExperimentComponentConsumerPacket,
    violations: &mut Vec<M5ExperimentComponentConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ExperimentComponentConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ExperimentComponentConsumerPacket,
    violations: &mut Vec<M5ExperimentComponentConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture
            .experiment_component_consumer_audit_ref
            .trim()
            .is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ExperimentComponentConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5ExperimentComponentConsumerPacket,
) -> impl Iterator<Item = &M5ExperimentComponentBindingCase> {
    packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|binding| binding.example_bindings.iter())
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
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

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
