//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the M5
//! experiment-run-row / dataset-provenance-card / artifact-lineage-panel / run-comparison-table /
//! environment-fingerprint-card / compare-guard-banner / sensitivity-sharing-banner /
//! result-summary-card experiment components.
//!
//! This module is the M05-1018 accessibility-and-auto-narrowing capstone over the frozen M5
//! experiment-component matrix
//! ([`crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix`]).
//! Where the freeze matrix defines the reusable experiment run row, dataset provenance card,
//! artifact lineage panel, run comparison table, environment fingerprint card, compare guard
//! banner, sensitivity / sharing banner, and result summary card primitives, and the 1013-1017
//! implementation / consumer lanes resolve their per-surface truth, this lane certifies — per
//! component family — that experiment claims stay **keyboard-complete, assistive-tech-reachable,
//! CLI/export-safe, and self-narrowing** rather than presenting a partial environment fingerprint,
//! an incomplete comparison, a blocked compare guard, a stale artifact lineage, a severed dataset
//! provenance, or a preview blocked by dataset sensitivity as still a fully exact, comparable,
//! provenanced, apples-to-apples result:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same run origin, code
//!   revision, environment fingerprint, dataset provenance, sensitivity state, comparability /
//!   confounder disclosure, and summary-versus-evidence-versus-raw export scope the rich component
//!   shows — never a hover-only chip that strands assistive-tech or headless-CLI users.
//!   Hierarchy-heavy families (the artifact lineage panel's nested producing-run / artifact /
//!   derived-artifact lineage) additionally bind their tree to a flat list / textual path.
//! - **Export parity.** The support / release / CLI export reconstructs each component's meaning
//!   from typed tokens and opaque refs **without a raw payload**, preserving the same stable run
//!   ids, code revisions, provenance / sensitivity posture, comparability disclosure, export
//!   scope, and narrowing reasons shown in-product so support, docs, and release proof can
//!   reconstruct exactly what the user was actually shown without leaking blocked raw data.
//! - **Honest auto-narrowing.** When a dataset's sensitivity blocks preview, artifact lineage is
//!   stale or missing, an environment fingerprint is partial, comparison evidence is incomplete,
//!   a compare guard is blocked, or a dataset's provenance is severed, the component's result claim
//!   auto-narrows from `ExactComparableResult` / `ReviewableResult` to a partial-fingerprint /
//!   incomparable-runs / guard-blocked / stale-lineage / unprovenanced-data / blocked-preview
//!   projection, discloses the narrowing with a precise trigger and binding dimension, and
//!   preserves the canonical run-identity / provenance / lineage. The underlying experiment lineage
//!   is never dropped opaquely. A component with every dimension intact must NOT carry a spurious
//!   narrowing, and a partial-fingerprint / incomplete-comparison / stale-lineage /
//!   severed-provenance state can never keep an exact comparable-result claim — an unproven
//!   comparison never implies an apples-to-apples fair baseline.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the notebook UI, the
//!   experiment dashboard, the comparison UI, the data-catalog UI, the lineage UI, the review UI,
//!   the CLI surface, the support export, and the product UI so product, docs, and release
//!   publication stay aligned on downgrade behavior rather than drifting in copy — an exact-looking
//!   surface can never outrun the provenance / parity / reproducibility proof it is being viewed
//!   away from.
//!
//! Each [`ExperimentComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix::M5ExperimentComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5ExperimentRequiredLabel`] and
//! [`M5ExperimentDowngradeTrigger`] and the shared [`M5ExperimentConsumerSurface`] consumer
//! surfaces rather than minting parallel synonyms, so the certified labels stay byte-identical to
//! the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw datasets, credentials, tokens, request bodies, and endpoint
//! secrets never cross this boundary; the packet carries only typed class tokens, opaque
//! experiment refs, booleans, and controlled labels so support, release, and diagnostics exports
//! can reconstruct exactly what an accessible fallback would have shown without leaking sensitive
//! material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix::{
    M5ExperimentComponentFamily, M5ExperimentConsumerSurface, M5ExperimentDowngradeTrigger,
    M5ExperimentRequiredLabel, M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1018 experiment-component accessibility fallback packet.
pub const EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ExperimentComponentAccessibilityPacket`].
pub const EXPERIMENT_COMPONENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_experiment_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`ExperimentComponentAccessibilityRow`].
pub const EXPERIMENT_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_experiment_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-experiment-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const EXPERIMENT_COMPONENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/notebooks/m5_experiment_component_accessibility_fallback.md";

/// Repo-relative path of the frozen experiment-component matrix this lane certifies.
pub const EXPERIMENT_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    M5_EXPERIMENT_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const EXPERIMENT_COMPONENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-experiment-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const EXPERIMENT_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-experiment-component-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EXPERIMENT_COMPONENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-experiment-component-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const EXPERIMENT_COMPONENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-experiment-component-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the artifact lineage
/// panel's nested producing-run / artifact / derived-artifact lineage) and therefore MUST bind
/// their tree to an equivalent flat list / textual path so the hierarchy is navigable
/// non-visually.
const fn family_is_hierarchy_heavy(family: M5ExperimentComponentFamily) -> bool {
    matches!(family, M5ExperimentComponentFamily::ArtifactLineagePanel)
}

/// The experiment dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5ExperimentComponentFamily,
) -> M5ExperimentComponentClaimDimension {
    match family {
        M5ExperimentComponentFamily::ExperimentRunRow => {
            M5ExperimentComponentClaimDimension::RunOriginTraceability
        }
        M5ExperimentComponentFamily::DatasetProvenanceCard => {
            M5ExperimentComponentClaimDimension::DatasetProvenance
        }
        M5ExperimentComponentFamily::ArtifactLineagePanel => {
            M5ExperimentComponentClaimDimension::ArtifactLineage
        }
        M5ExperimentComponentFamily::RunComparisonTable => {
            M5ExperimentComponentClaimDimension::ComparabilityEvidence
        }
        M5ExperimentComponentFamily::EnvironmentFingerprintCard => {
            M5ExperimentComponentClaimDimension::EnvironmentFingerprint
        }
        M5ExperimentComponentFamily::CompareGuardBanner => {
            M5ExperimentComponentClaimDimension::CompareGuardClearance
        }
        M5ExperimentComponentFamily::SensitivitySharingBanner => {
            M5ExperimentComponentClaimDimension::SensitivityDisclosure
        }
        M5ExperimentComponentFamily::ResultSummaryCard => {
            M5ExperimentComponentClaimDimension::ExportScopeClarity
        }
    }
}

/// A rendered fallback modality for an experiment component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentComponentFallbackModality {
    /// A rich, structured (nested producing-run / artifact / derived-artifact tree) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5ExperimentComponentFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured
    /// surface (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same
/// component may render at desktop-full capability or narrow to a companion, read-only browser,
/// headless CLI, docs export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentComponentRenderingSurface {
    /// The full-capability desktop experiment surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5ExperimentComponentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability
    /// baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless-CLI users
    /// (red).
    ViewOnlyTrap,
}

impl ExperimentComponentNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the component meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentComponentExportSummaryState {
    /// The component meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl ExperimentComponentExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw payload.
    pub const fn never_requires_raw_payload(self) -> bool {
        !matches!(self, Self::RequiresRawPayload)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawPayload => "reconstructable_without_raw_payload",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawPayload => "requires_raw_payload",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl ExperimentComponentNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The result claim ceiling a component asserts: how strong a comparability / reproducibility
/// posture it lets a surface present. Auto-narrowing lowers this ceiling when an experiment
/// dimension weakens so a partial environment fingerprint, an incomplete comparison, a blocked
/// compare guard, a stale artifact lineage, a severed dataset provenance, or a sensitivity-blocked
/// preview can never keep an old `ExactComparableResult` or `ReviewableResult` label — an unproven
/// comparison never masquerades as an apples-to-apples fair baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentComponentClaim {
    /// Exact comparable result: a fully traceable, provenanced, fingerprinted, apples-to-apples
    /// comparable result — the strongest claim, a surface Aureline can present as exactly true
    /// right now.
    ExactComparableResult,
    /// Reviewable result: a self-sufficient, reviewable read-only result summary (a result a user
    /// can review) that is not itself a certified exact-comparable path.
    ReviewableResult,
    /// Partial-fingerprint projection: the environment fingerprint is only partially captured; the
    /// surface stays a partial-fingerprint projection, never an exact-comparable result.
    PartialFingerprintProjection,
    /// Incomparable-runs projection: comparison evidence is incomplete; the surface stays an
    /// incomparable-runs projection, never an apples-to-apples fair baseline.
    IncomparableRunsProjection,
    /// Guard-blocked projection: a compare guard is blocking the comparison; the surface stays a
    /// guard-blocked projection with its guard reason preserved, never a qualified comparison.
    GuardBlockedProjection,
    /// Stale-lineage projection: the artifact lineage is stale or missing; the surface stays a
    /// stale-lineage projection with its last-known producing run preserved, never an
    /// exact-current lineage.
    StaleLineageProjection,
    /// Unprovenanced-data projection: the dataset provenance is severed; the surface stays an
    /// unprovenanced-data projection with its last-known source preserved, never a
    /// fully-provenanced dataset.
    UnprovenancedDataProjection,
    /// Blocked-preview projection: dataset sensitivity blocks preview; the surface stays a
    /// metadata-only blocked-preview projection, never a raw-data preview.
    BlockedPreviewProjection,
}

impl M5ExperimentComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::ExactComparableResult,
        Self::ReviewableResult,
        Self::PartialFingerprintProjection,
        Self::IncomparableRunsProjection,
        Self::GuardBlockedProjection,
        Self::StaleLineageProjection,
        Self::UnprovenancedDataProjection,
        Self::BlockedPreviewProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::ExactComparableResult => 7,
            Self::ReviewableResult => 6,
            Self::PartialFingerprintProjection => 5,
            Self::IncomparableRunsProjection => 4,
            Self::GuardBlockedProjection => 3,
            Self::StaleLineageProjection => 2,
            Self::UnprovenancedDataProjection => 1,
            Self::BlockedPreviewProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully exact, apples-to-apples comparable result.
    pub const fn asserts_exact_comparable_result(self) -> bool {
        matches!(self, Self::ExactComparableResult)
    }

    /// Returns true when this claim asserts a fully self-sufficient (exact or reviewable) result.
    pub const fn asserts_trustworthy_result(self) -> bool {
        matches!(self, Self::ExactComparableResult | Self::ReviewableResult)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactComparableResult => "exact_comparable_result",
            Self::ReviewableResult => "reviewable_result",
            Self::PartialFingerprintProjection => "partial_fingerprint_projection",
            Self::IncomparableRunsProjection => "incomparable_runs_projection",
            Self::GuardBlockedProjection => "guard_blocked_projection",
            Self::StaleLineageProjection => "stale_lineage_projection",
            Self::UnprovenancedDataProjection => "unprovenanced_data_projection",
            Self::BlockedPreviewProjection => "blocked_preview_projection",
        }
    }
}

/// The experiment dimension whose state governs how far a component may claim to be an exact,
/// comparable result. The dimensions map 1:1 to the eight frozen component families so every
/// family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentComponentClaimDimension {
    /// Run-origin traceability: is the run origin and code revision fully traceable?
    RunOriginTraceability,
    /// Dataset provenance: is the dataset's provenance intact, or is it severed?
    DatasetProvenance,
    /// Artifact lineage: is the artifact lineage exact-current, or is it stale / missing?
    ArtifactLineage,
    /// Comparability evidence: is the comparison parity evidence complete, or incomplete?
    ComparabilityEvidence,
    /// Environment fingerprint: is the environment fingerprint fully captured, or partial?
    EnvironmentFingerprint,
    /// Compare-guard clearance: is the compare guard clear, or blocking the comparison?
    CompareGuardClearance,
    /// Sensitivity disclosure: is a raw preview safe, or does dataset sensitivity block it?
    SensitivityDisclosure,
    /// Export-scope clarity: is the summary-versus-evidence-versus-raw export scope stated?
    ExportScopeClarity,
}

impl M5ExperimentComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::RunOriginTraceability,
        Self::DatasetProvenance,
        Self::ArtifactLineage,
        Self::ComparabilityEvidence,
        Self::EnvironmentFingerprint,
        Self::CompareGuardClearance,
        Self::SensitivityDisclosure,
        Self::ExportScopeClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunOriginTraceability => "run_origin_traceability",
            Self::DatasetProvenance => "dataset_provenance",
            Self::ArtifactLineage => "artifact_lineage",
            Self::ComparabilityEvidence => "comparability_evidence",
            Self::EnvironmentFingerprint => "environment_fingerprint",
            Self::CompareGuardClearance => "compare_guard_clearance",
            Self::SensitivityDisclosure => "sensitivity_disclosure",
            Self::ExportScopeClarity => "export_scope_clarity",
        }
    }
}

/// The observed condition of one experiment dimension. Anything weaker than
/// [`Self::LiveExactResult`] imposes a narrowing ceiling on the component's result claim. The four
/// spec axes the lane must auto-narrow on as *incomplete evidence* — a partial environment
/// fingerprint, an incomplete comparison, a stale artifact lineage, and a severed dataset
/// provenance — are the states that [`Self::cannot_be_proven_exact`] flags. A sensitivity-blocked
/// preview and a blocked compare guard are honest privacy / guard operations, not exactness
/// overstatements, so they are deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExperimentComponentConditionState {
    /// Fully traceable, provenanced, fingerprinted, apples-to-apples comparable — imposes no
    /// ceiling.
    LiveExactResult,
    /// The environment fingerprint is only partially captured — result claim drops to a
    /// partial-fingerprint projection.
    FingerprintPartial,
    /// Comparison parity evidence is incomplete — result claim drops to an incomparable-runs
    /// projection.
    ComparabilityIncomplete,
    /// A compare guard is blocking the comparison — result claim drops to a guard-blocked
    /// projection.
    CompareGuardBlocked,
    /// The artifact lineage is stale or missing — result claim drops to a stale-lineage
    /// projection.
    LineageStale,
    /// The dataset provenance is severed — result claim drops to an unprovenanced-data
    /// projection.
    ProvenanceSevered,
    /// Dataset sensitivity blocks preview — result claim drops to a metadata-only blocked-preview
    /// projection.
    SensitivityBlocksPreview,
}

impl M5ExperimentComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LiveExactResult,
        Self::FingerprintPartial,
        Self::ComparabilityIncomplete,
        Self::CompareGuardBlocked,
        Self::LineageStale,
        Self::ProvenanceSevered,
        Self::SensitivityBlocksPreview,
    ];

    /// Returns true when the dimension is weaker than exact and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::LiveExactResult)
    }

    /// Returns true when the condition reflects incomplete evidence that cannot be proven a
    /// fully exact, apples-to-apples comparable result and must never be shown as such. A
    /// sensitivity-blocked preview and a blocked compare guard are honest privacy / guard
    /// operations, not exactness overstatements, so they are deliberately excluded here.
    pub const fn cannot_be_proven_exact(self) -> bool {
        matches!(
            self,
            Self::FingerprintPartial
                | Self::ComparabilityIncomplete
                | Self::LineageStale
                | Self::ProvenanceSevered
        )
    }

    /// The strongest result claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5ExperimentComponentClaim {
        match self {
            Self::LiveExactResult => M5ExperimentComponentClaim::ExactComparableResult,
            Self::FingerprintPartial => M5ExperimentComponentClaim::PartialFingerprintProjection,
            Self::ComparabilityIncomplete => M5ExperimentComponentClaim::IncomparableRunsProjection,
            Self::CompareGuardBlocked => M5ExperimentComponentClaim::GuardBlockedProjection,
            Self::LineageStale => M5ExperimentComponentClaim::StaleLineageProjection,
            Self::ProvenanceSevered => M5ExperimentComponentClaim::UnprovenancedDataProjection,
            Self::SensitivityBlocksPreview => M5ExperimentComponentClaim::BlockedPreviewProjection,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing.
    /// Each state maps to the on-topic frozen trigger the freeze matrix already governs, so the
    /// certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5ExperimentDowngradeTrigger {
        match self {
            // The exact baseline never narrows; kept for exhaustiveness.
            Self::LiveExactResult => M5ExperimentDowngradeTrigger::ProofStale,
            Self::FingerprintPartial => {
                M5ExperimentDowngradeTrigger::EnvironmentFingerprintUnstated
            }
            Self::ComparabilityIncomplete => M5ExperimentDowngradeTrigger::ComparabilityOverstated,
            Self::CompareGuardBlocked => M5ExperimentDowngradeTrigger::ComparabilityOverstated,
            Self::LineageStale => M5ExperimentDowngradeTrigger::CachedStateHidden,
            Self::ProvenanceSevered => M5ExperimentDowngradeTrigger::DatasetProvenanceSevered,
            Self::SensitivityBlocksPreview => {
                M5ExperimentDowngradeTrigger::SensitivityClassUnstated
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveExactResult => "live_exact_result",
            Self::FingerprintPartial => "fingerprint_partial",
            Self::ComparabilityIncomplete => "comparability_incomplete",
            Self::CompareGuardBlocked => "compare_guard_blocked",
            Self::LineageStale => "lineage_stale",
            Self::ProvenanceSevered => "provenance_severed",
            Self::SensitivityBlocksPreview => "sensitivity_blocks_preview",
        }
    }
}

/// One experiment dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5ExperimentComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5ExperimentComponentConditionState,
}

/// An honest result-claim auto-narrow block. When an experiment dimension weakens, the component's
/// result claim lowers to the permitted ceiling, names the binding dimension and frozen trigger,
/// and preserves the canonical run-identity / provenance / lineage rather than silently dropping
/// it — the underlying experiment lineage is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentComponentClaimAutoNarrow {
    /// The result claim the component is narrowed to.
    pub narrowed_to: M5ExperimentComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5ExperimentComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5ExperimentDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical run identity, dataset provenance, artifact lineage, and export scope are
    /// preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying run-identity / provenance / lineage is preserved (never dropped) across the
    /// narrowing; must hold so partial-fingerprint, incomparable-runs, guard-blocked,
    /// stale-lineage, unprovenanced-data, and blocked-preview states never fail opaquely.
    pub preserves_lineage_continuity: bool,
}

impl ExperimentComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and experiment
    /// lineage and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_lineage_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl ExperimentComponentCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at
    /// least one export field is named, and a raw-payload-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_payload_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5ExperimentComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: ExperimentComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for an experiment-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims comparability, or drops state
    /// silently (red).
    Stranded,
}

impl ExperimentComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one experiment-component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentComponentAccessibilityRow {
    /// Record kind; must equal [`EXPERIMENT_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5ExperimentComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the run / dataset / artifact / comparison object this component represents;
    /// stays visible on every surface, so this is never empty.
    pub experiment_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual (list /
    /// textual / CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5ExperimentComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical run origin, code revision, environment
    /// fingerprint, dataset provenance, sensitivity state, comparability disclosure, and export
    /// scope as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: ExperimentComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: ExperimentComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: ExperimentComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: ExperimentComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: ExperimentComponentCopyExportParity,
    /// The full result claim this family asserts when every dimension is intact.
    pub full_experiment_claim: M5ExperimentComponentClaim,
    /// The observed condition of each modeled experiment dimension.
    #[serde(default)]
    pub claim_conditions: Vec<ExperimentComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's
    /// full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<ExperimentComponentClaimAutoNarrow>,
    /// Whether the underlying experiment lineage is preserved on this component regardless of
    /// narrowing; must hold so partial-fingerprint, incomparable-runs, guard-blocked,
    /// stale-lineage, unprovenanced-data, and blocked-preview states never fail opaquely.
    pub lineage_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5ExperimentComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<ExperimentComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ExperimentRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ExperimentComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat
    /// non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is
    /// offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `LiveExactResult` when the row does not
    /// model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5ExperimentComponentClaimDimension,
    ) -> M5ExperimentComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5ExperimentComponentConditionState::LiveExactResult)
    }

    /// Whether any modeled dimension is weaker than exact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest result claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5ExperimentComponentClaim {
        let mut permitted = self.full_experiment_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_condition(&self) -> Option<&ExperimentComponentClaimConditionEntry> {
        let mut binding: Option<(&ExperimentComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_experiment_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5ExperimentComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The result claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5ExperimentComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_experiment_claim,
        }
    }

    /// AC / auto-narrowing honesty: a partial fingerprint, an incomplete comparison, a blocked
    /// compare guard, a stale artifact lineage, a severed dataset provenance, or a
    /// sensitivity-blocked preview can no longer keep an old `ExactComparableResult` /
    /// `ReviewableResult` label. The effective claim never exceeds the permitted ceiling; when a
    /// dimension narrows below the full claim, an honest narrow block is present, narrows to
    /// exactly the permitted ceiling, binds to the ceiling-imposing dimension with its frozen
    /// trigger, and preserves canonical identity and lineage. When nothing narrows, no spurious
    /// narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / comparable-result honesty: a partial-fingerprint / incomplete-comparison /
    /// stale-lineage / severed-provenance state never keeps an exact comparable-result claim — an
    /// unproven comparison never implies an apples-to-apples fair baseline. When such a state is
    /// modeled, the effective claim must not assert `ExactComparableResult`.
    pub fn comparable_result_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_proven_exact());
        !(has_unprovable_state && self.effective_claim().asserts_exact_comparable_result())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical
    /// truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.experiment_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: partial-fingerprint, incomparable-runs, guard-blocked, stale-lineage,
    /// unprovenanced-data, and blocked-preview states preserve the underlying experiment lineage.
    /// The row must assert `lineage_preserved`, and any narrow block must preserve lineage
    /// continuity too.
    pub fn preserves_lineage_continuity(&self) -> bool {
        self.lineage_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_lineage_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an
    /// honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / docs / release publication stay aligned on
    /// the same narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5ExperimentRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> ExperimentComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.comparable_result_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_lineage_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return ExperimentComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            ExperimentComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            ExperimentComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == EXPERIMENT_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.experiment_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_experiment_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1018 experiment-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_comparable_result_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_lineage_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`ExperimentComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<ExperimentComponentAccessibilityRow>,
}

/// Checked-in M05-1018 experiment-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ExperimentComponentAccessibilityRow>,
    pub summary: ExperimentComponentAccessibilitySummary,
}

impl ExperimentComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ExperimentComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: EXPERIMENT_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: ExperimentComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_comparable_result_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_lineage_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ExperimentComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5ExperimentComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5ExperimentComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Result claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5ExperimentComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5ExperimentConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ExperimentComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5ExperimentConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&ExperimentComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                ExperimentComponentAccessibilityStatus::Parity => green += 1,
                ExperimentComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                ExperimentComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        ExperimentComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(ExperimentComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(ExperimentComponentAccessibilityRow::claim_is_honest),
            all_comparable_result_honesty_holds: self
                .rows
                .iter()
                .all(ExperimentComponentAccessibilityRow::comparable_result_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(ExperimentComponentAccessibilityRow::export_preserves_meaning),
            all_lineage_preserved: self
                .rows
                .iter()
                .all(ExperimentComponentAccessibilityRow::preserves_lineage_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(ExperimentComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ExperimentComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(ExperimentComponentAccessibilityViolation::SchemaVersion {
                expected: EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EXPERIMENT_COMPONENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(ExperimentComponentAccessibilityViolation::RecordKind {
                expected: EXPERIMENT_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ExperimentComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ExperimentComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_proven_exact())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(ExperimentComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    ExperimentComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory experiment label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    ExperimentComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5ExperimentComponentFallbackModality::Structured)
            {
                violations.push(
                    ExperimentComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts an exact / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    ExperimentComponentAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: a partial-fingerprint / incomplete-comparison / stale-lineage /
            // severed-provenance state never keeps an exact comparable-result claim.
            if !row.comparable_result_honesty_holds() {
                violations.push(
                    ExperimentComponentAccessibilityViolation::UnprovableStateShownAsComparable {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    ExperimentComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    ExperimentComponentAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: partial-fingerprint, incomparable-runs, guard-blocked, stale-lineage,
            // unprovenanced-data, and blocked-preview states preserve experiment lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(ExperimentComponentAccessibilityViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    ExperimentComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    ExperimentComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == ExperimentComponentAccessibilityStatus::Stranded {
                violations.push(ExperimentComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5ExperimentComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    ExperimentComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5ExperimentComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    ExperimentComponentAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every condition state (the exact baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5ExperimentComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    ExperimentComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every result claim tier appears as an effective claim, so the full narrowing
        // spectrum (exact-comparable → … → blocked-preview) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5ExperimentComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    ExperimentComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Comparable-result honesty must be proven with at least one partial-fingerprint /
        // incomplete-comparison / stale-lineage / severed-provenance row in the packet, so the
        // "cannot-prove never shown as exact comparable" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations
                .push(ExperimentComponentAccessibilityViolation::ComparableResultHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the notebook, experiment-dashboard,
        // comparison, data-catalog, lineage, review, CLI, support-export, and product surfaces —
        // so every consumer surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5ExperimentConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    ExperimentComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ExperimentComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("experiment-component accessibility fallback packet serializes"),
        ) {
            violations
                .push(ExperimentComponentAccessibilityViolation::RawExperimentMaterialInExport);
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
            .expect("experiment-component accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_experiment_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Experiment-Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5ExperimentComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_experiment_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in experiment-component accessibility fallback export.
pub fn current_m5_experiment_component_a11y_fallback_export(
) -> Result<ExperimentComponentAccessibilityPacket, ExperimentComponentAccessibilityArtifactError> {
    let packet: ExperimentComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-experiment-component-accessibility-fallback/support_export.json"
    )))
    .map_err(ExperimentComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ExperimentComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in experiment-component accessibility fallback export.
#[derive(Debug)]
pub enum ExperimentComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ExperimentComponentAccessibilityViolation>),
}

impl fmt::Display for ExperimentComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "experiment-component accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "experiment-component accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ExperimentComponentAccessibilityArtifactError {}

/// Validation failure for M05-1018 experiment-component accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExperimentComponentAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5ExperimentComponentClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    UnprovableStateShownAsComparable {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawPayload {
        id: String,
    },
    LineageDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5ExperimentComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5ExperimentComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5ExperimentComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5ExperimentComponentClaim,
    },
    ComparableResultHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5ExperimentConsumerSurface,
    },
    SummaryMismatch,
    RawExperimentMaterialInExport,
}

impl ExperimentComponentAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::HierarchyHeavyMissingStructured { .. } => "hierarchy_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::UnprovableStateShownAsComparable { .. } => "unprovable_state_shown_as_comparable",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawPayload { .. } => "export_requires_raw_payload",
            Self::LineageDropped { .. } => "lineage_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingFamilyCoverage { .. } => "missing_family_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::ComparableResultHonestyUnproven => "comparable_result_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawExperimentMaterialInExport => "raw_experiment_material_in_export",
        }
    }
}

impl fmt::Display for ExperimentComponentAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory experiment label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts an exact / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::UnprovableStateShownAsComparable { id } => {
                write!(
                    f,
                    "row {id} shows a partial-fingerprint / incomplete-comparison / stale-lineage / severed-provenance state as an exact comparable result"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawPayload { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw payload"
                )
            }
            Self::LineageDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve experiment lineage across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "result claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::ComparableResultHonestyUnproven => {
                write!(
                    f,
                    "no partial-fingerprint / incomplete-comparison / stale-lineage / severed-provenance row is present to prove the comparable-result-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawExperimentMaterialInExport => {
                write!(f, "export contains raw experiment material")
            }
        }
    }
}

impl Error for ExperimentComponentAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "incomplete"
            | "not comparable"
            | "incomparable"
            | "unprovenanced"
            | "severed"
            | "redacted"
            | "blocked preview"
            | "guard blocked"
            | "imported"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in experiment-component accessibility fallback packet. This is the
/// one source of truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_experiment_component_a11y_fallback_packet(
) -> ExperimentComponentAccessibilityPacket {
    ExperimentComponentAccessibilityPacket::new(ExperimentComponentAccessibilityPacketInput {
        packet_id: "m5-experiment-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-09T00:00:00Z".to_owned(),
        matrix_ref: EXPERIMENT_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:experiment-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5ExperimentRequiredLabel> {
    M5ExperimentRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> ExperimentComponentCopyExportParity {
    ExperimentComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5ExperimentComponentClaimDimension,
    state: M5ExperimentComponentConditionState,
) -> ExperimentComponentClaimConditionEntry {
    ExperimentComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the CLI
/// surface — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5ExperimentConsumerSurface]) -> Vec<M5ExperimentConsumerSurface> {
    let mut out = vec![
        M5ExperimentConsumerSurface::SupportExport,
        M5ExperimentConsumerSurface::CliSurface,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: ExperimentComponentNarrowingDisclosureState,
) -> Vec<ExperimentComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        ExperimentComponentRenderingNarrowingDisclosure {
            rendering_surface: M5ExperimentComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        ExperimentComponentRenderingNarrowingDisclosure {
            rendering_surface: M5ExperimentComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_animated_overlay".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<ExperimentComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ExperimentComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<ExperimentComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ExperimentComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5ExperimentComponentRenderingSurface> {
    vec![
        M5ExperimentComponentRenderingSurface::DesktopFull,
        M5ExperimentComponentRenderingSurface::CliHeadless,
        M5ExperimentComponentRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5ExperimentComponentFallbackModality> {
    vec![
        M5ExperimentComponentFallbackModality::List,
        M5ExperimentComponentFallbackModality::Textual,
        M5ExperimentComponentFallbackModality::Cli,
    ]
}

fn seeded_rows() -> Vec<ExperimentComponentAccessibilityRow> {
    vec![
        // Experiment run row (live / local) — the run origin and code revision are fully
        // traceable, so it is a fully exact comparable result and reachable on every surface
        // (green).
        ExperimentComponentAccessibilityRow {
            record_kind: EXPERIMENT_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:experiment-run-row-live".to_owned(),
            component_family: M5ExperimentComponentFamily::ExperimentRunRow,
            source_family_schema_ref: EXPERIMENT_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            experiment_context_ref: "experiment:experiment-run-row:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                ExperimentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:experiment-run-row-live:a11y".to_owned(),
            copy_export: copy_export(&[
                "run_identity",
                "run_origin_and_revision",
                "run_status",
                "keyboard_route",
            ]),
            full_experiment_claim: M5ExperimentComponentClaim::ExactComparableResult,
            claim_conditions: vec![condition(
                M5ExperimentComponentClaimDimension::RunOriginTraceability,
                M5ExperimentComponentConditionState::LiveExactResult,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "run_identity",
                "run_origin_and_revision",
                "run_status",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ExperimentConsumerSurface::NotebookUi,
                M5ExperimentConsumerSurface::ExperimentDashboardUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.21 experiment run rows".to_owned(),
                EXPERIMENT_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("experiment-run-row-live"),
        },
        // Result summary card (reviewable) — a self-sufficient, reviewable read-only result
        // summary (a result a user can review), not itself a certified exact-comparable path,
        // reachable on every surface (green).
        ExperimentComponentAccessibilityRow {
            record_kind: EXPERIMENT_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:result-summary-card-reviewable".to_owned(),
            component_family: M5ExperimentComponentFamily::ResultSummaryCard,
            source_family_schema_ref: EXPERIMENT_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            experiment_context_ref: "experiment:result-summary-card:0002".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                ExperimentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:result-summary-card-reviewable:a11y".to_owned(),
            copy_export: copy_export(&[
                "summary_identity",
                "summary_content_class",
                "export_scope",
                "keyboard_route",
            ]),
            full_experiment_claim: M5ExperimentComponentClaim::ReviewableResult,
            claim_conditions: vec![condition(
                M5ExperimentComponentClaimDimension::ExportScopeClarity,
                M5ExperimentComponentConditionState::LiveExactResult,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "summary_identity",
                "summary_content_class",
                "export_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ExperimentConsumerSurface::ReviewUi,
                M5ExperimentConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.21 result summary cards".to_owned(),
                EXPERIMENT_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("result-summary-card-reviewable"),
        },
        // Environment fingerprint card (partial capture) — the environment fingerprint is only
        // partially captured, so the card auto-narrows to a partial-fingerprint projection rather
        // than presenting an exact comparable result, while keeping its identity, scope, and
        // captured fields visible (yellow).
        ExperimentComponentAccessibilityRow {
            record_kind: EXPERIMENT_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:environment-fingerprint-card-partial".to_owned(),
            component_family: M5ExperimentComponentFamily::EnvironmentFingerprintCard,
            source_family_schema_ref: EXPERIMENT_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            experiment_context_ref: "experiment:environment-fingerprint-card:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                ExperimentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:environment-fingerprint-card-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "fingerprint_identity",
                "fingerprint_scope",
                "capture_state",
                "partial_capture_note",
            ]),
            full_experiment_claim: M5ExperimentComponentClaim::ExactComparableResult,
            claim_conditions: vec![condition(
                M5ExperimentComponentClaimDimension::EnvironmentFingerprint,
                M5ExperimentComponentConditionState::FingerprintPartial,
            )],
            claim_narrow: Some(ExperimentComponentClaimAutoNarrow {
                narrowed_to: M5ExperimentComponentClaim::PartialFingerprintProjection,
                binding_dimension: M5ExperimentComponentClaimDimension::EnvironmentFingerprint,
                trigger: M5ExperimentDowngradeTrigger::EnvironmentFingerprintUnstated,
                narrowed_label:
                    "This environment fingerprint is only partially captured — shown as a partial-fingerprint projection that names the captured scopes, never as an exact-comparable reproducible result"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "fingerprint_identity",
                "fingerprint_scope",
                "capture_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ExperimentConsumerSurface::ExperimentDashboardUi,
                M5ExperimentConsumerSurface::NotebookUi,
            ]),
            source_refs: vec![
                "TDD §7.10.5 reproducibility metadata".to_owned(),
                EXPERIMENT_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("environment-fingerprint-card-partial"),
        },
        // Run comparison table (comparison evidence incomplete) — the comparison parity evidence
        // is incomplete, so the table auto-narrows to an incomparable-runs projection that never
        // implies an apples-to-apples fair baseline, while keeping the baseline / candidate
        // identity and confounder disclosure visible (yellow).
        ExperimentComponentAccessibilityRow {
            record_kind: EXPERIMENT_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:run-comparison-table-incomplete".to_owned(),
            component_family: M5ExperimentComponentFamily::RunComparisonTable,
            source_family_schema_ref: EXPERIMENT_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            experiment_context_ref: "experiment:run-comparison-table:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                ExperimentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:run-comparison-table-incomplete:a11y".to_owned(),
            copy_export: copy_export(&[
                "comparison_identity",
                "baseline_and_candidate",
                "comparability_state",
                "confounder_disclosure",
            ]),
            full_experiment_claim: M5ExperimentComponentClaim::ExactComparableResult,
            claim_conditions: vec![condition(
                M5ExperimentComponentClaimDimension::ComparabilityEvidence,
                M5ExperimentComponentConditionState::ComparabilityIncomplete,
            )],
            claim_narrow: Some(ExperimentComponentClaimAutoNarrow {
                narrowed_to: M5ExperimentComponentClaim::IncomparableRunsProjection,
                binding_dimension: M5ExperimentComponentClaimDimension::ComparabilityEvidence,
                trigger: M5ExperimentDowngradeTrigger::ComparabilityOverstated,
                narrowed_label:
                    "Parity evidence for this comparison is incomplete — shown as an incomparable-runs projection that names the baseline and candidate with their confounders, never as an apples-to-apples fair baseline"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "comparison_identity",
                "baseline_and_candidate",
                "comparability_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ExperimentConsumerSurface::ComparisonUi,
                M5ExperimentConsumerSurface::ExperimentDashboardUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.21 result comparison".to_owned(),
                EXPERIMENT_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("run-comparison-table-incomplete"),
        },
        // Compare guard banner (guard blocked) — a compare guard is blocking the comparison
        // because parity evidence is incomplete, so the banner auto-narrows to a guard-blocked
        // projection that keeps its guard reason visible, never a qualified comparison (yellow).
        ExperimentComponentAccessibilityRow {
            record_kind: EXPERIMENT_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:compare-guard-banner-blocked".to_owned(),
            component_family: M5ExperimentComponentFamily::CompareGuardBanner,
            source_family_schema_ref: EXPERIMENT_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            experiment_context_ref: "experiment:compare-guard-banner:0005".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                ExperimentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:compare-guard-banner-blocked:a11y".to_owned(),
            copy_export: copy_export(&[
                "guard_identity",
                "guard_reason",
                "guard_state",
                "keyboard_route",
            ]),
            full_experiment_claim: M5ExperimentComponentClaim::ReviewableResult,
            claim_conditions: vec![condition(
                M5ExperimentComponentClaimDimension::CompareGuardClearance,
                M5ExperimentComponentConditionState::CompareGuardBlocked,
            )],
            claim_narrow: Some(ExperimentComponentClaimAutoNarrow {
                narrowed_to: M5ExperimentComponentClaim::GuardBlockedProjection,
                binding_dimension: M5ExperimentComponentClaimDimension::CompareGuardClearance,
                trigger: M5ExperimentDowngradeTrigger::ComparabilityOverstated,
                narrowed_label:
                    "A compare guard is blocking this comparison — shown as a guard-blocked projection that names the guard reason and stable route, never as a qualified fair comparison"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "guard_identity",
                "guard_reason",
                "guard_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ExperimentConsumerSurface::ComparisonUi,
                M5ExperimentConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.21 result comparison guards".to_owned(),
                EXPERIMENT_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("compare-guard-banner-blocked"),
        },
        // Artifact lineage panel (lineage stale) — hierarchy-heavy (nested producing-run /
        // artifact / derived-artifact lineage); the artifact lineage is stale, so the panel
        // auto-narrows to a stale-lineage projection and binds its nested lineage tree to a flat
        // list / textual path (yellow).
        ExperimentComponentAccessibilityRow {
            record_kind: EXPERIMENT_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:artifact-lineage-panel-stale".to_owned(),
            component_family: M5ExperimentComponentFamily::ArtifactLineagePanel,
            source_family_schema_ref: EXPERIMENT_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            experiment_context_ref: "experiment:artifact-lineage-panel:0006".to_owned(),
            fallback_modalities: vec![
                M5ExperimentComponentFallbackModality::Structured,
                M5ExperimentComponentFallbackModality::List,
                M5ExperimentComponentFallbackModality::Textual,
                M5ExperimentComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach:
                ExperimentComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                ExperimentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:artifact-lineage-panel-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "artifact_identity",
                "producing_run",
                "lineage_state",
                "staleness_note",
            ]),
            full_experiment_claim: M5ExperimentComponentClaim::ExactComparableResult,
            claim_conditions: vec![condition(
                M5ExperimentComponentClaimDimension::ArtifactLineage,
                M5ExperimentComponentConditionState::LineageStale,
            )],
            claim_narrow: Some(ExperimentComponentClaimAutoNarrow {
                narrowed_to: M5ExperimentComponentClaim::StaleLineageProjection,
                binding_dimension: M5ExperimentComponentClaimDimension::ArtifactLineage,
                trigger: M5ExperimentDowngradeTrigger::CachedStateHidden,
                narrowed_label:
                    "This artifact lineage is stale — shown as a stale-lineage projection that names the last-known producing run and lineage state, never as an exact-current lineage"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "artifact_identity",
                "producing_run",
                "lineage_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ExperimentConsumerSurface::LineageUi,
                M5ExperimentConsumerSurface::ReviewUi,
            ]),
            source_refs: vec![
                "TDD §7.10.5 experiment provenance".to_owned(),
                EXPERIMENT_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("artifact-lineage-panel-stale"),
        },
        // Dataset provenance card (provenance severed) — the dataset provenance is severed, so the
        // card auto-narrows to an unprovenanced-data projection that keeps its last-known source
        // preserved, never masquerading as a fully-provenanced dataset (yellow).
        ExperimentComponentAccessibilityRow {
            record_kind: EXPERIMENT_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:dataset-provenance-card-severed".to_owned(),
            component_family: M5ExperimentComponentFamily::DatasetProvenanceCard,
            source_family_schema_ref: EXPERIMENT_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            experiment_context_ref: "experiment:dataset-provenance-card:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ExperimentComponentNonVisualReachState::DisclosedReducedButReachable,
            export_summary:
                ExperimentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:dataset-provenance-card-severed:a11y".to_owned(),
            copy_export: copy_export(&[
                "dataset_identity",
                "dataset_source_class",
                "provenance_state",
                "last_known_source",
            ]),
            full_experiment_claim: M5ExperimentComponentClaim::ExactComparableResult,
            claim_conditions: vec![condition(
                M5ExperimentComponentClaimDimension::DatasetProvenance,
                M5ExperimentComponentConditionState::ProvenanceSevered,
            )],
            claim_narrow: Some(ExperimentComponentClaimAutoNarrow {
                narrowed_to: M5ExperimentComponentClaim::UnprovenancedDataProjection,
                binding_dimension: M5ExperimentComponentClaimDimension::DatasetProvenance,
                trigger: M5ExperimentDowngradeTrigger::DatasetProvenanceSevered,
                narrowed_label:
                    "This dataset's provenance is severed — shown as an unprovenanced-data projection that keeps its last-known source class visible, never as a fully-provenanced dataset"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "dataset_identity",
                "dataset_source_class",
                "provenance_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ExperimentConsumerSurface::DataCatalogUi,
                M5ExperimentConsumerSurface::LineageUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.21 dataset governance".to_owned(),
                EXPERIMENT_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("dataset-provenance-card-severed"),
        },
        // Sensitivity / sharing banner (sensitivity blocks preview) — dataset sensitivity blocks
        // preview, so the banner auto-narrows to a metadata-only blocked-preview projection that
        // keeps its sensitivity class and sharing scope visible, never leaking a raw-data preview
        // (yellow).
        ExperimentComponentAccessibilityRow {
            record_kind: EXPERIMENT_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXPERIMENT_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:sensitivity-sharing-banner-blocked-preview".to_owned(),
            component_family: M5ExperimentComponentFamily::SensitivitySharingBanner,
            source_family_schema_ref: EXPERIMENT_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            experiment_context_ref: "experiment:sensitivity-sharing-banner:0008".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ExperimentComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                ExperimentComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:sensitivity-sharing-banner-blocked-preview:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "sensitivity_identity",
                "sensitivity_class",
                "share_scope",
                "blocked_preview_note",
            ]),
            full_experiment_claim: M5ExperimentComponentClaim::ReviewableResult,
            claim_conditions: vec![condition(
                M5ExperimentComponentClaimDimension::SensitivityDisclosure,
                M5ExperimentComponentConditionState::SensitivityBlocksPreview,
            )],
            claim_narrow: Some(ExperimentComponentClaimAutoNarrow {
                narrowed_to: M5ExperimentComponentClaim::BlockedPreviewProjection,
                binding_dimension: M5ExperimentComponentClaimDimension::SensitivityDisclosure,
                trigger: M5ExperimentDowngradeTrigger::SensitivityClassUnstated,
                narrowed_label:
                    "This dataset's sensitivity blocks preview — shown as a metadata-only blocked-preview projection that names the sensitivity class and share scope, never leaking a raw-data preview by default"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "sensitivity_identity",
                "sensitivity_class",
                "share_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ExperimentConsumerSurface::DataCatalogUi,
                M5ExperimentConsumerSurface::ReviewUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.21 dataset governance and sharing".to_owned(),
                EXPERIMENT_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("sensitivity-sharing-banner-blocked-preview"),
        },
    ]
}
