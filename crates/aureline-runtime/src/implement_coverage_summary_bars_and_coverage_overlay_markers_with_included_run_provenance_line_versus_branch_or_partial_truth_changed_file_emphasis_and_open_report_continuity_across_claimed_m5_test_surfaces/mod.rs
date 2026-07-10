//! Two reusable M5 test-intelligence primitives — the coverage-summary bar and the
//! coverage-overlay marker — so a green coverage number stops hiding what run set it measured
//! and an editor gutter glyph stops losing the exact coverage state and the path back to the
//! evidence that produced it. A summary bar always names its scope, its line-versus-branch (or
//! combined / partial) metric dimension, its included run set, its freshness, its
//! imported/merged/live source note, and its open-uncovered-lines action; an overlay marker
//! always preserves its line/branch/partial/unknown state in the frozen controlled
//! vocabulary, its changed-line emphasis, the source run-set identity behind the glyph, and a
//! durable path back to the evidence object that produced the marker.
//!
//! Aureline's frozen test-intelligence component matrix
//! ([`crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix`])
//! names the coverage-summary bar and the coverage-overlay marker as two governed component
//! families and freezes their controlled vocabulary — the provenance / freshness classes, the
//! coverage scope classes, the coverage metric kinds, the coverage overlay states, the overlay
//! emphasis classes, plus the surface families, the deployment lines, the consumer surfaces,
//! the accessibility routes, the qualification classes, and the downgrade triggers. This
//! module *implements* that contract as two reusable resolvers so a user can tell — from the
//! summary bar alone — the coverage scope, the metric dimension, the included run set, the
//! freshness, and whether the number is a live local run, an imported report, a merged
//! multi-run, a cached reuse, or a stale replay, and — from the overlay marker alone —
//! whether a line is covered, uncovered, partial, branch-missed, excluded, or unknown, whether
//! it is an emphasized changed line, which source run set produced the glyph, and how to jump
//! back to the evidence object behind it. Above all, a single percentage never hides a shard
//! omission or a stale provenance, a merged multi-run or imported report never collapses into
//! one unlabeled number, and an editor overlay preserves its exact coverage-state meaning and
//! a durable path back to the evidence object across the report, the editor, CI, and export.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_coverage_summary_bar`] — takes one coverage summary's scope class, metric
//!    kind, provenance class, freshness state, source note, included run count, covered /
//!    total units, shard-omission flag, opaque scope label, and opaque summary identity, and
//!    produces one [`M5ResolvedCoverageSummaryBar`] carrying the derived coverage posture (a
//!    full-suite, changed-files, single-shard, merged-multi-shard, imported-report, or
//!    partial-incomplete summary — one distinct posture per scope so no two scopes collapse
//!    into one percentage), whether the summary is multi-run, imported, or stale, whether it
//!    must show an included-run label, whether it discloses a shard omission, whether there are
//!    uncovered lines to open, and the bounded reveal / open-uncovered-lines / open-report /
//!    rerun / export actions. It never collapses a multi-run or imported scope into one
//!    unlabeled percentage, never hides a shard omission or a stale provenance, and never drops
//!    the line-versus-branch metric dimension.
//! 2. [`resolve_coverage_overlay_marker`] — takes one overlay marker's coverage state, emphasis
//!    class, provenance class, changed-line flag, opaque source run-set ref, opaque evidence
//!    object ref, and opaque line reference, and produces one [`M5ResolvedCoverageOverlayMarker`]
//!    carrying the derived overlay posture (a covered, uncovered, partial, branch-missed,
//!    excluded, or unknown marker — the frozen controlled vocabulary), whether it is an
//!    emphasized changed line, whether it preserves its exact state meaning, whether it offers a
//!    durable path back to the evidence object, and the bounded reveal / open-report /
//!    open-uncovered-context / export actions. It never invents an alternate label for a
//!    governed coverage state, never drops the source run-set identity, and never severs the
//!    editor-to-report continuity.
//!
//! A single parity matrix — [`M5CoverageComponentsPacket`] — binds one row per claimed M5
//! coverage consumer (the coverage-report panel, the editor gutter overlay, the CI coverage
//! summary, the headless/CLI coverage surface, and the coverage report export) to the shared
//! summary and overlay anatomy, the same scope classes, metric kinds, provenance classes,
//! freshness states, source notes, coverage postures, overlay states, overlay emphasis classes,
//! overlay postures, bounded actions, export fields, and non-visual accessibility routes, so
//! the coverage vocabulary stays identical across the report, the editor, CI, headless/export,
//! and support consumers — the acceptance-criterion parity that keeps included-run and imported
//! evidence labeled everywhere with one vocabulary.
//!
//! The provenance class ([`M5TestIntelligenceProvenanceClass`]), coverage scope class
//! ([`M5CoverageScopeClass`]), coverage metric kind ([`M5CoverageMetricKind`]), coverage
//! overlay state ([`M5CoverageOverlayState`]), overlay emphasis class ([`M5OverlayEmphasisClass`]),
//! surface family ([`M5TestIntelligenceSurfaceFamily`]), deployment line
//! ([`M5TestIntelligenceDeploymentLine`]), consumer surface ([`M5TestIntelligenceConsumerSurface`]),
//! accessibility route ([`M5TestIntelligenceAccessibilityRoute`]), qualification class
//! ([`M5TestIntelligenceQualificationClass`]), and downgrade trigger
//! ([`M5TestIntelligenceDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the two coverage
//! components themselves: their coverage consumers, the coverage freshness state, the coverage
//! source note, the two derived postures, the two bounded action sets, the two anatomies, and
//! the two export field sets. No M5 test surface invents a second coverage-bar or
//! overlay-marker grammar.
//!
//! Raw coverage payloads, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every scope label, source run-set ref, evidence object ref, line reference,
//! and identity is carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_coverage_components_editor_gutter_overlay_beta_narrowed,
    seeded_m5_coverage_components_packet,
    seeded_m5_coverage_components_report_panel_preview_narrowed, M5_COVERAGE_COMPONENTS_PACKET_ID,
};

// The provenance class, coverage scope class, coverage metric kind, coverage overlay state,
// overlay emphasis class, surface family, deployment line, consumer surface, accessibility
// route, qualification class, and downgrade triggers are frozen once, in the test-intelligence
// component matrix. These primitives reuse them verbatim so they never invent parallel coverage
// vocabulary.
pub use crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix::{
    M5CoverageMetricKind, M5CoverageOverlayState, M5CoverageScopeClass, M5OverlayEmphasisClass,
    M5TestIntelligenceAccessibilityRoute, M5TestIntelligenceConsumerSurface,
    M5TestIntelligenceDeploymentLine, M5TestIntelligenceDowngradeTrigger,
    M5TestIntelligenceProvenanceClass, M5TestIntelligenceQualificationClass,
    M5TestIntelligenceSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5CoverageComponentsPacket`].
pub const M5_COVERAGE_COMPONENTS_RECORD_KIND: &str =
    "implement_m5_coverage_summary_bars_and_coverage_overlay_markers_with_included_run_provenance_line_versus_branch_or_partial_truth_changed_file_emphasis_and_open_report_continuity_across_claimed_m5_test_surfaces";

/// Schema version for M5 coverage-summary / overlay-marker records.
pub const M5_COVERAGE_COMPONENTS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the coverage-summary-bar boundary schema (the canonical packet
/// schema).
pub const M5_COVERAGE_COMPONENTS_SUMMARY_SCHEMA_REF: &str =
    "schemas/ui/m5-coverage-summary-bar.schema.json";

/// Repo-relative path of the coverage-overlay-marker companion schema.
pub const M5_COVERAGE_COMPONENTS_OVERLAY_SCHEMA_REF: &str =
    "schemas/ui/m5-coverage-overlay-marker.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_COVERAGE_COMPONENTS_DOC_REF: &str =
    "docs/testing/m5_coverage_summary_overlay_primitive.md";

/// Repo-relative path of the frozen test-intelligence component matrix these primitives narrow
/// from.
pub const M5_COVERAGE_COMPONENTS_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-test-intelligence-component-matrix.schema.json";

/// Repo-relative path of the coverage-merge contract the summary bar binds its
/// included-run / scope / merge truth against.
pub const M5_COVERAGE_COMPONENTS_COVERAGE_MERGE_REF: &str =
    "schemas/testing/coverage_merge_result.schema.json";

/// Repo-relative path of the coverage-overlay contract the overlay marker binds its
/// state / emphasis / evidence truth against.
pub const M5_COVERAGE_COMPONENTS_COVERAGE_OVERLAY_REF: &str =
    "schemas/testing/coverage-overlays-and-snapshot-golden-review.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_COVERAGE_COMPONENTS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-coverage-summary-overlay-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COVERAGE_COMPONENTS_ARTIFACT_REF: &str =
    "artifacts/release/m5-coverage-summary-overlay-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_COVERAGE_COMPONENTS_CSV_REF: &str =
    "artifacts/release/m5-coverage-summary-overlay-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_COVERAGE_COMPONENTS_REPORT_REF: &str =
    "artifacts/design/m5-coverage-summary-overlay-primitive.md";

/// One claimed M5 coverage consumer that renders the shared coverage-summary bar and
/// coverage-overlay marker. These are the consumers the acceptance criteria name — the
/// coverage-report panel, the editor gutter overlay, the CI coverage summary, the
/// headless/CLI coverage surface, and the coverage report export — so the same coverage
/// grammar works across every claimed test surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageComponentConsumerSurface {
    /// The coverage-report panel surface.
    CoverageReportPanel,
    /// The editor gutter-overlay surface.
    EditorGutterOverlay,
    /// The CI coverage-summary surface.
    CiCoverageSummary,
    /// The headless / CLI coverage surface.
    HeadlessCliCoverage,
    /// The coverage report export surface.
    CoverageReportExport,
}

impl M5CoverageComponentConsumerSurface {
    /// Every claimed coverage consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CoverageReportPanel,
        Self::EditorGutterOverlay,
        Self::CiCoverageSummary,
        Self::HeadlessCliCoverage,
        Self::CoverageReportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoverageReportPanel => "coverage_report_panel",
            Self::EditorGutterOverlay => "editor_gutter_overlay",
            Self::CiCoverageSummary => "ci_coverage_summary",
            Self::HeadlessCliCoverage => "headless_cli_coverage",
            Self::CoverageReportExport => "coverage_report_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CoverageReportPanel => "Coverage Report Panel",
            Self::EditorGutterOverlay => "Editor Gutter Overlay",
            Self::CiCoverageSummary => "CI Coverage Summary",
            Self::HeadlessCliCoverage => "Headless / CLI Coverage",
            Self::CoverageReportExport => "Coverage Report Export",
        }
    }
}

/// Controlled coverage freshness state a summary bar shows, so a coverage number never leaves
/// implicit whether it was just measured, recently measured, gone stale and needs a rerun,
/// imported as a snapshot, or of unknown freshness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageFreshnessState {
    /// Measured by the current local run.
    FreshCurrentRun,
    /// Measured recently but not by the current run.
    RecentlyMeasured,
    /// Older than the current source and in need of a rerun.
    StaleNeedsRerun,
    /// Imported as a snapshot from an external report.
    ImportedSnapshot,
    /// Freshness could not be determined.
    UnknownFreshness,
}

impl M5CoverageFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FreshCurrentRun,
        Self::RecentlyMeasured,
        Self::StaleNeedsRerun,
        Self::ImportedSnapshot,
        Self::UnknownFreshness,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshCurrentRun => "fresh_current_run",
            Self::RecentlyMeasured => "recently_measured",
            Self::StaleNeedsRerun => "stale_needs_rerun",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::UnknownFreshness => "unknown_freshness",
        }
    }

    /// True when the freshness is a fully current, just-measured run.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::FreshCurrentRun)
    }

    /// True when the freshness itself signals a stale coverage number.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::StaleNeedsRerun)
    }
}

/// Controlled coverage source note — the imported/merged/live note a summary bar shows, so a
/// number is never left ambiguous about whether it came from a live local run, an imported
/// report, a merged multi-run, a cached reuse, or a stale replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageSourceNote {
    /// A live local run.
    LiveLocalRun,
    /// An imported external report.
    ImportedReport,
    /// A merged multi-run set.
    MergedMultiRun,
    /// A cached prior result reused without a fresh run.
    CachedReuse,
    /// A stale replay of an older result.
    StaleReplay,
}

impl M5CoverageSourceNote {
    /// Every source note, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveLocalRun,
        Self::ImportedReport,
        Self::MergedMultiRun,
        Self::CachedReuse,
        Self::StaleReplay,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveLocalRun => "live_local_run",
            Self::ImportedReport => "imported_report",
            Self::MergedMultiRun => "merged_multi_run",
            Self::CachedReuse => "cached_reuse",
            Self::StaleReplay => "stale_replay",
        }
    }

    /// True when the note itself already marks the number as imported.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedReport)
    }

    /// True when the note itself already marks the number as a merged multi-run.
    pub const fn is_merged(self) -> bool {
        matches!(self, Self::MergedMultiRun)
    }

    /// True when the note itself already marks the number as a stale replay.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::StaleReplay)
    }
}

/// The derived posture of a coverage-summary bar — one distinct posture per coverage scope so
/// no two scopes collapse into one percentage. Computed 1:1 from the coverage scope class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageSummaryPosture {
    /// A full-suite coverage summary.
    FullSuiteSummary,
    /// A changed-files coverage summary.
    ChangedFilesSummary,
    /// A single-shard coverage summary.
    SingleShardSummary,
    /// A merged multi-shard coverage summary.
    MergedMultiShardSummary,
    /// An imported-report coverage summary.
    ImportedReportSummary,
    /// A partial / incomplete coverage summary.
    PartialIncompleteSummary,
}

impl M5CoverageSummaryPosture {
    /// Every coverage posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullSuiteSummary,
        Self::ChangedFilesSummary,
        Self::SingleShardSummary,
        Self::MergedMultiShardSummary,
        Self::ImportedReportSummary,
        Self::PartialIncompleteSummary,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSuiteSummary => "full_suite_summary",
            Self::ChangedFilesSummary => "changed_files_summary",
            Self::SingleShardSummary => "single_shard_summary",
            Self::MergedMultiShardSummary => "merged_multi_shard_summary",
            Self::ImportedReportSummary => "imported_report_summary",
            Self::PartialIncompleteSummary => "partial_incomplete_summary",
        }
    }

    /// The frozen coverage-scope class this posture maps 1:1 to.
    pub const fn scope(self) -> M5CoverageScopeClass {
        match self {
            Self::FullSuiteSummary => M5CoverageScopeClass::FullSuite,
            Self::ChangedFilesSummary => M5CoverageScopeClass::ChangedFilesOnly,
            Self::SingleShardSummary => M5CoverageScopeClass::SingleShard,
            Self::MergedMultiShardSummary => M5CoverageScopeClass::MergedMultiShard,
            Self::ImportedReportSummary => M5CoverageScopeClass::ImportedReport,
            Self::PartialIncompleteSummary => M5CoverageScopeClass::PartialIncomplete,
        }
    }

    /// True when the scope itself spans more than one run and therefore always needs an
    /// included-run label so it never collapses into one unlabeled percentage.
    pub const fn is_multi_run_scope(self) -> bool {
        matches!(
            self,
            Self::MergedMultiShardSummary | Self::ImportedReportSummary
        )
    }

    /// True when the scope names its coverage as complete for what it claims to measure.
    pub const fn is_complete_scope(self) -> bool {
        !matches!(
            self,
            Self::SingleShardSummary | Self::PartialIncompleteSummary
        )
    }
}

/// One bounded action a coverage-summary bar offers, so a bar never hides its reveal /
/// open-uncovered-lines / open-report / rerun / export affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageSummaryAction {
    /// Reveal the summary's scope, metric dimension, included run set, freshness, and source
    /// note.
    RevealCoverageDetails,
    /// Open the list of uncovered lines behind this summary.
    OpenUncoveredLines,
    /// Open the full coverage report this summary was produced from.
    OpenCoverageReport,
    /// Rerun coverage to refresh a stale, imported, or non-current number.
    RerunCoverage,
    /// Export the coverage summary as test evidence.
    ExportCoverage,
}

impl M5CoverageSummaryAction {
    /// Every summary action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealCoverageDetails,
        Self::OpenUncoveredLines,
        Self::OpenCoverageReport,
        Self::RerunCoverage,
        Self::ExportCoverage,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealCoverageDetails => "reveal_coverage_details",
            Self::OpenUncoveredLines => "open_uncovered_lines",
            Self::OpenCoverageReport => "open_coverage_report",
            Self::RerunCoverage => "rerun_coverage",
            Self::ExportCoverage => "export_coverage",
        }
    }
}

/// Controlled coverage-summary anatomy part. The parts in
/// [`M5CoverageSummaryAnatomyPart::MANDATORY`] are required on every bar so the scope, metric
/// dimension, included run set, freshness, and source note are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageSummaryAnatomyPart {
    /// The coverage-scope cue.
    ScopeCue,
    /// The metric-dimension (line-versus-branch) cue.
    MetricDimensionCue,
    /// The included-run-set cue.
    IncludedRunSetCue,
    /// The freshness cue.
    FreshnessCue,
    /// The imported/merged/live source-note cue.
    SourceNoteCue,
    /// The coverage-percentage cue.
    PercentageCue,
    /// The open-uncovered-lines action cue.
    UncoveredActionCue,
    /// The provenance cue.
    ProvenanceCue,
}

impl M5CoverageSummaryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ScopeCue,
        Self::MetricDimensionCue,
        Self::IncludedRunSetCue,
        Self::FreshnessCue,
        Self::SourceNoteCue,
        Self::PercentageCue,
        Self::UncoveredActionCue,
        Self::ProvenanceCue,
    ];

    /// The anatomy parts every summary bar must render.
    pub const MANDATORY: [Self; 5] = [
        Self::ScopeCue,
        Self::MetricDimensionCue,
        Self::IncludedRunSetCue,
        Self::FreshnessCue,
        Self::SourceNoteCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScopeCue => "scope_cue",
            Self::MetricDimensionCue => "metric_dimension_cue",
            Self::IncludedRunSetCue => "included_run_set_cue",
            Self::FreshnessCue => "freshness_cue",
            Self::SourceNoteCue => "source_note_cue",
            Self::PercentageCue => "percentage_cue",
            Self::UncoveredActionCue => "uncovered_action_cue",
            Self::ProvenanceCue => "provenance_cue",
        }
    }
}

/// A field the coverage-summary export carries so summary-bar truth is reconstructable. The
/// fields in [`M5CoverageSummaryExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageSummaryExportField {
    /// The coverage scope class.
    ScopeClass,
    /// The coverage metric kind.
    MetricKind,
    /// The provenance class.
    ProvenanceClass,
    /// The freshness state.
    FreshnessState,
    /// The source note.
    SourceNote,
    /// The included run count.
    IncludedRunCount,
    /// The covered unit count.
    CoveredUnits,
    /// The total unit count.
    TotalUnits,
    /// The derived coverage posture.
    CoveragePosture,
}

impl M5CoverageSummaryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ScopeClass,
        Self::MetricKind,
        Self::ProvenanceClass,
        Self::FreshnessState,
        Self::SourceNote,
        Self::IncludedRunCount,
        Self::CoveredUnits,
        Self::TotalUnits,
        Self::CoveragePosture,
    ];

    /// The export fields every summary bar must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ScopeClass,
        Self::MetricKind,
        Self::ProvenanceClass,
        Self::SourceNote,
        Self::CoveragePosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScopeClass => "scope_class",
            Self::MetricKind => "metric_kind",
            Self::ProvenanceClass => "provenance_class",
            Self::FreshnessState => "freshness_state",
            Self::SourceNote => "source_note",
            Self::IncludedRunCount => "included_run_count",
            Self::CoveredUnits => "covered_units",
            Self::TotalUnits => "total_units",
            Self::CoveragePosture => "coverage_posture",
        }
    }
}

/// The derived posture of a coverage-overlay marker — the frozen controlled vocabulary, one
/// distinct posture per coverage overlay state, so a governed coverage state never borrows an
/// alternate label. Computed 1:1 from the coverage overlay state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverlayMarkerPosture {
    /// A covered-line marker.
    CoveredMarker,
    /// An uncovered-line marker.
    UncoveredMarker,
    /// A partially-covered-line marker.
    PartialMarker,
    /// A branch-missed marker.
    BranchMissedMarker,
    /// An excluded-line marker.
    ExcludedMarker,
    /// An unknown / no-data marker.
    UnknownMarker,
}

impl M5OverlayMarkerPosture {
    /// Every overlay posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CoveredMarker,
        Self::UncoveredMarker,
        Self::PartialMarker,
        Self::BranchMissedMarker,
        Self::ExcludedMarker,
        Self::UnknownMarker,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoveredMarker => "covered_marker",
            Self::UncoveredMarker => "uncovered_marker",
            Self::PartialMarker => "partial_marker",
            Self::BranchMissedMarker => "branch_missed_marker",
            Self::ExcludedMarker => "excluded_marker",
            Self::UnknownMarker => "unknown_marker",
        }
    }

    /// True only for a fully-covered marker — the only posture that needs no attention.
    pub const fn is_covered(self) -> bool {
        matches!(self, Self::CoveredMarker)
    }

    /// True when the marker flags a coverage gap a reviewer should act on.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::UncoveredMarker | Self::PartialMarker | Self::BranchMissedMarker
        )
    }

    /// The frozen coverage-overlay state this posture maps 1:1 to.
    pub const fn overlay_state(self) -> M5CoverageOverlayState {
        match self {
            Self::CoveredMarker => M5CoverageOverlayState::CoveredLine,
            Self::UncoveredMarker => M5CoverageOverlayState::UncoveredLine,
            Self::PartialMarker => M5CoverageOverlayState::PartiallyCovered,
            Self::BranchMissedMarker => M5CoverageOverlayState::BranchMissed,
            Self::ExcludedMarker => M5CoverageOverlayState::ExcludedLine,
            Self::UnknownMarker => M5CoverageOverlayState::NoOverlayData,
        }
    }
}

/// One bounded action a coverage-overlay marker offers, so a marker never hides its reveal /
/// open-report / open-uncovered-context / export affordances — the editor-to-report continuity
/// the implementation requirements name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverlayMarkerAction {
    /// Reveal the marker's coverage state, emphasis, provenance, and source run set.
    RevealMarkerDetails,
    /// Open the coverage report / evidence object behind this marker.
    OpenCoverageReport,
    /// Open the uncovered / partial context around this marker.
    OpenUncoveredContext,
    /// Export the marker as test evidence.
    ExportMarker,
}

impl M5OverlayMarkerAction {
    /// Every overlay action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RevealMarkerDetails,
        Self::OpenCoverageReport,
        Self::OpenUncoveredContext,
        Self::ExportMarker,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealMarkerDetails => "reveal_marker_details",
            Self::OpenCoverageReport => "open_coverage_report",
            Self::OpenUncoveredContext => "open_uncovered_context",
            Self::ExportMarker => "export_marker",
        }
    }
}

/// Controlled overlay-marker anatomy part. The parts in
/// [`M5OverlayMarkerAnatomyPart::MANDATORY`] are required on every marker so the coverage
/// state, changed-line emphasis, source run-set identity, evidence link, and line reference
/// are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverlayMarkerAnatomyPart {
    /// The coverage-overlay-state cue.
    OverlayStateCue,
    /// The changed-line emphasis cue.
    EmphasisCue,
    /// The source run-set identity cue.
    SourceRunSetCue,
    /// The evidence-object link (report continuity) cue.
    EvidenceLinkCue,
    /// The line-reference cue.
    LineReferenceCue,
    /// The provenance cue.
    ProvenanceCue,
}

impl M5OverlayMarkerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OverlayStateCue,
        Self::EmphasisCue,
        Self::SourceRunSetCue,
        Self::EvidenceLinkCue,
        Self::LineReferenceCue,
        Self::ProvenanceCue,
    ];

    /// The anatomy parts every overlay marker must render.
    pub const MANDATORY: [Self; 5] = [
        Self::OverlayStateCue,
        Self::EmphasisCue,
        Self::SourceRunSetCue,
        Self::EvidenceLinkCue,
        Self::LineReferenceCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OverlayStateCue => "overlay_state_cue",
            Self::EmphasisCue => "emphasis_cue",
            Self::SourceRunSetCue => "source_run_set_cue",
            Self::EvidenceLinkCue => "evidence_link_cue",
            Self::LineReferenceCue => "line_reference_cue",
            Self::ProvenanceCue => "provenance_cue",
        }
    }
}

/// A field the overlay export carries so overlay-marker truth is reconstructable. The fields in
/// [`M5OverlayMarkerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverlayMarkerExportField {
    /// The coverage overlay state.
    OverlayState,
    /// The overlay emphasis class.
    EmphasisClass,
    /// The provenance class.
    ProvenanceClass,
    /// The source run-set ref.
    SourceRunSetRef,
    /// The evidence object ref.
    EvidenceObjectRef,
    /// The derived overlay posture.
    OverlayPosture,
}

impl M5OverlayMarkerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OverlayState,
        Self::EmphasisClass,
        Self::ProvenanceClass,
        Self::SourceRunSetRef,
        Self::EvidenceObjectRef,
        Self::OverlayPosture,
    ];

    /// The export fields every overlay marker must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::OverlayState,
        Self::EmphasisClass,
        Self::SourceRunSetRef,
        Self::EvidenceObjectRef,
        Self::OverlayPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OverlayState => "overlay_state",
            Self::EmphasisClass => "emphasis_class",
            Self::ProvenanceClass => "provenance_class",
            Self::SourceRunSetRef => "source_run_set_ref",
            Self::EvidenceObjectRef => "evidence_object_ref",
            Self::OverlayPosture => "overlay_posture",
        }
    }
}

/// True when a provenance class marks the coverage number as one that must be labeled as
/// imported rather than a live local run.
pub const fn provenance_is_imported(provenance: M5TestIntelligenceProvenanceClass) -> bool {
    matches!(
        provenance,
        M5TestIntelligenceProvenanceClass::ImportedCiArtifact
    )
}

/// True when a provenance class marks the coverage number as stale rather than a fresh result.
pub const fn provenance_is_stale(provenance: M5TestIntelligenceProvenanceClass) -> bool {
    matches!(
        provenance,
        M5TestIntelligenceProvenanceClass::StalePriorResult
    )
}

/// True when the emphasis class is one that genuinely emphasizes a changed line rather than a
/// context / stable / suppressed region.
pub const fn emphasis_is_changed_line(emphasis: M5OverlayEmphasisClass) -> bool {
    matches!(
        emphasis,
        M5OverlayEmphasisClass::ChangedLineEmphasis
            | M5OverlayEmphasisClass::NewlyUncovered
            | M5OverlayEmphasisClass::RegressionHotspot
    )
}

// ---- coverage-summary-bar resolver --------------------------------------

/// The full input to the coverage-summary-bar resolver for one summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoverageSummaryResolutionInput {
    /// The coverage scope class.
    pub scope_class: M5CoverageScopeClass,
    /// The coverage metric kind (line versus branch versus combined).
    pub metric_kind: M5CoverageMetricKind,
    /// The provenance / freshness class behind the number.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// The freshness state of the coverage data.
    pub freshness_state: M5CoverageFreshnessState,
    /// The imported/merged/live source note.
    pub source_note: M5CoverageSourceNote,
    /// The number of runs included in this summary's run set.
    pub included_run_count: u32,
    /// The number of covered units (lines / branches / regions).
    pub covered_units: u32,
    /// The total number of measured units.
    pub total_units: u32,
    /// Whether some shard / partition is omitted from this summary.
    pub has_shard_omission: bool,
    /// The opaque user-facing scope label (must be non-empty).
    pub scope_label: String,
    /// The opaque stable summary identity / evidence object ref (must be non-empty).
    pub summary_identity_ref: String,
}

/// The resolved coverage-summary-bar truth for one summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCoverageSummaryBar {
    /// The coverage scope class.
    pub scope_class: M5CoverageScopeClass,
    /// The coverage metric kind.
    pub metric_kind: M5CoverageMetricKind,
    /// The provenance class.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// The freshness state.
    pub freshness_state: M5CoverageFreshnessState,
    /// The source note.
    pub source_note: M5CoverageSourceNote,
    /// The included run count, preserved from the input.
    pub included_run_count: u32,
    /// The covered unit count, preserved from the input.
    pub covered_units: u32,
    /// The total unit count, preserved from the input.
    pub total_units: u32,
    /// The opaque scope label, preserved exactly from the input.
    pub scope_label: String,
    /// The opaque stable summary identity, preserved exactly from the input.
    pub summary_identity_ref: String,
    /// The derived coverage posture.
    pub coverage_posture: M5CoverageSummaryPosture,
    /// The bounded actions this bar offers.
    pub available_actions: Vec<M5CoverageSummaryAction>,
    /// True when the summary spans more than one run (merged or multi-run set).
    pub is_multi_run: bool,
    /// True when the summary is imported evidence rather than a live local run.
    pub is_imported: bool,
    /// True when the summary's provenance / freshness / source note is stale.
    pub is_stale: bool,
    /// True when the summary must show an included-run label so it never collapses a multi-run
    /// or imported scope into one unlabeled percentage.
    pub requires_included_run_label: bool,
    /// True when the summary discloses a shard omission (kept visible, never hidden).
    pub discloses_shard_omission: bool,
    /// True when there are uncovered units to open.
    pub has_uncovered: bool,
    /// True when the number is not a fully current run and offers a rerun.
    pub can_rerun: bool,
    /// True when the summary needs a reviewer's attention before it reads as a trustworthy
    /// green number.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_coverage_summary_bar`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5CoverageSummaryResolutionError {
    /// The scope label was empty.
    EmptyScopeLabel,
    /// The summary identity ref was empty.
    EmptySummaryIdentity,
    /// The covered units exceeded the total units.
    InvalidUnitCount,
    /// A summary descriptor carried forbidden material.
    ForbiddenCoverageMaterial,
}

impl M5CoverageSummaryResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyScopeLabel => "empty_scope_label",
            Self::EmptySummaryIdentity => "empty_summary_identity",
            Self::InvalidUnitCount => "invalid_unit_count",
            Self::ForbiddenCoverageMaterial => "forbidden_coverage_material",
        }
    }
}

impl fmt::Display for M5CoverageSummaryResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "coverage summary bar resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5CoverageSummaryResolutionError {}

/// Resolves one coverage-summary bar from its declared coverage state.
///
/// The derived coverage posture is 1:1 with the coverage scope class — full-suite,
/// changed-files, single-shard, merged-multi-shard, imported-report, or partial-incomplete —
/// so no two scopes collapse into one percentage. A multi-run or imported summary always
/// requires an included-run label; a shard omission is always kept disclosed; a stale or
/// non-current number always offers a rerun; uncovered units always offer an
/// open-uncovered-lines action; reveal, open-report, and export are always offered. The
/// provenance, freshness, and source note are always carried, so a green number never hides a
/// shard omission or a stale provenance.
pub fn resolve_coverage_summary_bar(
    input: &M5CoverageSummaryResolutionInput,
) -> Result<M5ResolvedCoverageSummaryBar, M5CoverageSummaryResolutionError> {
    if input.scope_label.trim().is_empty() {
        return Err(M5CoverageSummaryResolutionError::EmptyScopeLabel);
    }
    if input.summary_identity_ref.trim().is_empty() {
        return Err(M5CoverageSummaryResolutionError::EmptySummaryIdentity);
    }
    if input.covered_units > input.total_units {
        return Err(M5CoverageSummaryResolutionError::InvalidUnitCount);
    }
    if value_repr_is_forbidden(&input.scope_label)
        || value_repr_is_forbidden(&input.summary_identity_ref)
    {
        return Err(M5CoverageSummaryResolutionError::ForbiddenCoverageMaterial);
    }

    let coverage_posture = derive_coverage_posture(input.scope_class);
    let is_multi_run = coverage_posture.is_multi_run_scope()
        || input.source_note.is_merged()
        || input.included_run_count > 1;
    let is_imported = matches!(input.scope_class, M5CoverageScopeClass::ImportedReport)
        || input.source_note.is_imported()
        || provenance_is_imported(input.provenance_class)
        || matches!(
            input.freshness_state,
            M5CoverageFreshnessState::ImportedSnapshot
        );
    let is_stale = provenance_is_stale(input.provenance_class)
        || input.freshness_state.is_stale()
        || input.source_note.is_stale();
    let has_uncovered = input.covered_units < input.total_units;
    let can_rerun = !input.freshness_state.is_current() || is_stale;
    let available_actions = derive_summary_actions(has_uncovered, can_rerun);

    Ok(M5ResolvedCoverageSummaryBar {
        scope_class: input.scope_class,
        metric_kind: input.metric_kind,
        provenance_class: input.provenance_class,
        freshness_state: input.freshness_state,
        source_note: input.source_note,
        included_run_count: input.included_run_count,
        covered_units: input.covered_units,
        total_units: input.total_units,
        scope_label: input.scope_label.clone(),
        summary_identity_ref: input.summary_identity_ref.clone(),
        coverage_posture,
        available_actions,
        is_multi_run,
        is_imported,
        is_stale,
        requires_included_run_label: is_multi_run || is_imported,
        discloses_shard_omission: input.has_shard_omission,
        has_uncovered,
        can_rerun,
        needs_attention: is_stale
            || is_imported
            || has_uncovered
            || input.has_shard_omission
            || !coverage_posture.is_complete_scope(),
    })
}

/// The 1:1 coverage-scope → coverage-posture map.
fn derive_coverage_posture(scope_class: M5CoverageScopeClass) -> M5CoverageSummaryPosture {
    match scope_class {
        M5CoverageScopeClass::FullSuite => M5CoverageSummaryPosture::FullSuiteSummary,
        M5CoverageScopeClass::ChangedFilesOnly => M5CoverageSummaryPosture::ChangedFilesSummary,
        M5CoverageScopeClass::SingleShard => M5CoverageSummaryPosture::SingleShardSummary,
        M5CoverageScopeClass::MergedMultiShard => M5CoverageSummaryPosture::MergedMultiShardSummary,
        M5CoverageScopeClass::ImportedReport => M5CoverageSummaryPosture::ImportedReportSummary,
        M5CoverageScopeClass::PartialIncomplete => {
            M5CoverageSummaryPosture::PartialIncompleteSummary
        }
    }
}

/// Derives the bounded summary-action set from the uncovered / rerun signals.
fn derive_summary_actions(has_uncovered: bool, can_rerun: bool) -> Vec<M5CoverageSummaryAction> {
    use M5CoverageSummaryAction as Action;
    let mut actions = vec![Action::RevealCoverageDetails];
    if has_uncovered {
        actions.push(Action::OpenUncoveredLines);
    }
    actions.push(Action::OpenCoverageReport);
    if can_rerun {
        actions.push(Action::RerunCoverage);
    }
    actions.push(Action::ExportCoverage);
    actions
}

// ---- coverage-overlay-marker resolver -----------------------------------

/// The full input to the coverage-overlay-marker resolver for one marker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverlayMarkerResolutionInput {
    /// The coverage overlay state.
    pub overlay_state: M5CoverageOverlayState,
    /// The overlay emphasis class.
    pub emphasis_class: M5OverlayEmphasisClass,
    /// The provenance / freshness class behind the marker.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// Whether this marker sits on a changed line.
    pub is_changed_line: bool,
    /// The opaque source run-set identity behind the glyph (must be non-empty).
    pub source_run_set_ref: String,
    /// The opaque durable path back to the evidence object (must be non-empty).
    pub evidence_object_ref: String,
    /// The opaque line reference (must be non-empty).
    pub line_reference: String,
}

/// The resolved coverage-overlay-marker truth for one marker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCoverageOverlayMarker {
    /// The coverage overlay state.
    pub overlay_state: M5CoverageOverlayState,
    /// The overlay emphasis class.
    pub emphasis_class: M5OverlayEmphasisClass,
    /// The provenance class.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// Whether this marker sits on a changed line, preserved from the input.
    pub is_changed_line: bool,
    /// The opaque source run-set ref, preserved exactly from the input.
    pub source_run_set_ref: String,
    /// The opaque evidence object ref, preserved exactly from the input.
    pub evidence_object_ref: String,
    /// The opaque line reference, preserved exactly from the input.
    pub line_reference: String,
    /// The derived overlay posture.
    pub overlay_posture: M5OverlayMarkerPosture,
    /// The bounded actions this marker offers.
    pub available_actions: Vec<M5OverlayMarkerAction>,
    /// True when this marker is an emphasized changed line (changed-line emphasis preserved).
    pub is_emphasized_change: bool,
    /// True when the marker preserves its exact coverage-state meaning (always true — the
    /// posture is 1:1 with the frozen overlay state).
    pub preserves_state_meaning: bool,
    /// True when the marker carries a durable path back to the evidence object that produced
    /// it — the editor-to-report continuity.
    pub has_report_continuity: bool,
    /// True when the marker flags a coverage gap a reviewer should act on.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_coverage_overlay_marker`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5OverlayMarkerResolutionError {
    /// The source run-set ref was empty.
    EmptySourceRunSet,
    /// The evidence object ref was empty — the marker would lose its path back to the report.
    EmptyEvidenceObject,
    /// The line reference was empty.
    EmptyLineReference,
    /// An overlay descriptor carried forbidden material.
    ForbiddenOverlayMaterial,
}

impl M5OverlayMarkerResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySourceRunSet => "empty_source_run_set",
            Self::EmptyEvidenceObject => "empty_evidence_object",
            Self::EmptyLineReference => "empty_line_reference",
            Self::ForbiddenOverlayMaterial => "forbidden_overlay_material",
        }
    }
}

impl fmt::Display for M5OverlayMarkerResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "coverage overlay marker resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5OverlayMarkerResolutionError {}

/// Resolves one coverage-overlay marker from its declared overlay state.
///
/// The derived overlay posture is 1:1 with the frozen coverage overlay state — covered,
/// uncovered, partial, branch-missed, excluded, or unknown — so a governed coverage state never
/// borrows an alternate label and the exact state meaning is always preserved. A changed line
/// carrying a changed-line / newly-uncovered / regression-hotspot emphasis is kept emphasized.
/// The source run-set identity and a durable path back to the evidence object are always
/// preserved (a missing evidence ref fails resolution), so an editor overlay never severs its
/// continuity to the report. Reveal, open-report, and export are always offered;
/// open-uncovered-context is offered whenever the marker flags a coverage gap.
pub fn resolve_coverage_overlay_marker(
    input: &M5OverlayMarkerResolutionInput,
) -> Result<M5ResolvedCoverageOverlayMarker, M5OverlayMarkerResolutionError> {
    if input.source_run_set_ref.trim().is_empty() {
        return Err(M5OverlayMarkerResolutionError::EmptySourceRunSet);
    }
    if input.evidence_object_ref.trim().is_empty() {
        return Err(M5OverlayMarkerResolutionError::EmptyEvidenceObject);
    }
    if input.line_reference.trim().is_empty() {
        return Err(M5OverlayMarkerResolutionError::EmptyLineReference);
    }
    if value_repr_is_forbidden(&input.source_run_set_ref)
        || value_repr_is_forbidden(&input.evidence_object_ref)
        || value_repr_is_forbidden(&input.line_reference)
    {
        return Err(M5OverlayMarkerResolutionError::ForbiddenOverlayMaterial);
    }

    let overlay_posture = derive_overlay_posture(input.overlay_state);
    let needs_attention = overlay_posture.needs_attention();
    let available_actions = derive_overlay_actions(needs_attention);

    Ok(M5ResolvedCoverageOverlayMarker {
        overlay_state: input.overlay_state,
        emphasis_class: input.emphasis_class,
        provenance_class: input.provenance_class,
        is_changed_line: input.is_changed_line,
        source_run_set_ref: input.source_run_set_ref.clone(),
        evidence_object_ref: input.evidence_object_ref.clone(),
        line_reference: input.line_reference.clone(),
        overlay_posture,
        available_actions,
        is_emphasized_change: input.is_changed_line
            && emphasis_is_changed_line(input.emphasis_class),
        preserves_state_meaning: true,
        has_report_continuity: !input.evidence_object_ref.trim().is_empty(),
        needs_attention,
    })
}

/// The 1:1 coverage-overlay-state → overlay-posture map.
fn derive_overlay_posture(overlay_state: M5CoverageOverlayState) -> M5OverlayMarkerPosture {
    match overlay_state {
        M5CoverageOverlayState::CoveredLine => M5OverlayMarkerPosture::CoveredMarker,
        M5CoverageOverlayState::UncoveredLine => M5OverlayMarkerPosture::UncoveredMarker,
        M5CoverageOverlayState::PartiallyCovered => M5OverlayMarkerPosture::PartialMarker,
        M5CoverageOverlayState::BranchMissed => M5OverlayMarkerPosture::BranchMissedMarker,
        M5CoverageOverlayState::ExcludedLine => M5OverlayMarkerPosture::ExcludedMarker,
        M5CoverageOverlayState::NoOverlayData => M5OverlayMarkerPosture::UnknownMarker,
    }
}

/// Derives the bounded overlay-action set from the needs-attention signal.
fn derive_overlay_actions(needs_attention: bool) -> Vec<M5OverlayMarkerAction> {
    use M5OverlayMarkerAction as Action;
    let mut actions = vec![Action::RevealMarkerDetails, Action::OpenCoverageReport];
    if needs_attention {
        actions.push(Action::OpenUncoveredContext);
    }
    actions.push(Action::ExportMarker);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked coverage-summary-bar resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoverageSummaryResolutionCase {
    /// The resolver input.
    pub input: M5CoverageSummaryResolutionInput,
    /// The resolved truth. Must equal `resolve_coverage_summary_bar(&input)`.
    pub resolved: M5ResolvedCoverageSummaryBar,
}

impl M5CoverageSummaryResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5CoverageSummaryResolutionInput) -> Self {
        let resolved = resolve_coverage_summary_bar(&input).expect("seed summary case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_coverage_summary_bar(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved summary identity preserves the input identity exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.summary_identity_ref == self.input.summary_identity_ref
            && self.resolved.scope_label == self.input.scope_label
    }
}

/// One worked coverage-overlay-marker resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverlayMarkerResolutionCase {
    /// The resolver input.
    pub input: M5OverlayMarkerResolutionInput,
    /// The resolved truth. Must equal `resolve_coverage_overlay_marker(&input)`.
    pub resolved: M5ResolvedCoverageOverlayMarker,
}

impl M5OverlayMarkerResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5OverlayMarkerResolutionInput) -> Self {
        let resolved = resolve_coverage_overlay_marker(&input).expect("seed overlay case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_coverage_overlay_marker(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved overlay identity preserves the input source run-set and evidence
    /// refs exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.source_run_set_ref == self.input.source_run_set_ref
            && self.resolved.evidence_object_ref == self.input.evidence_object_ref
            && self.resolved.line_reference == self.input.line_reference
    }
}

/// One row in the primitive matrix: one coverage consumer bound to the shared summary and
/// overlay anatomy, scope classes, metric kinds, provenance classes, freshness states, source
/// notes, coverage postures, overlay states, overlay emphasis classes, overlay postures,
/// bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoverageComponentConsumerRow {
    /// Coverage consumer family.
    pub consumer_surface: M5CoverageComponentConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5TestIntelligenceQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 test surface families that render / consume these components.
    pub surface_families: Vec<M5TestIntelligenceSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5TestIntelligenceDeploymentLine>,
    /// Summary-bar anatomy parts this consumer renders (must include the mandatory parts).
    pub summary_anatomy_parts: Vec<M5CoverageSummaryAnatomyPart>,
    /// Overlay-marker anatomy parts this consumer renders (must include the mandatory parts).
    pub overlay_anatomy_parts: Vec<M5OverlayMarkerAnatomyPart>,
    /// Coverage scope classes this consumer distinguishes.
    pub coverage_scope_classes: Vec<M5CoverageScopeClass>,
    /// Coverage metric kinds this consumer distinguishes.
    pub coverage_metric_kinds: Vec<M5CoverageMetricKind>,
    /// Provenance classes this consumer distinguishes.
    pub provenance_classes: Vec<M5TestIntelligenceProvenanceClass>,
    /// Coverage freshness states this consumer distinguishes.
    pub freshness_states: Vec<M5CoverageFreshnessState>,
    /// Coverage source notes this consumer distinguishes.
    pub source_notes: Vec<M5CoverageSourceNote>,
    /// Coverage postures this consumer distinguishes.
    pub coverage_postures: Vec<M5CoverageSummaryPosture>,
    /// Coverage overlay states this consumer distinguishes.
    pub overlay_states: Vec<M5CoverageOverlayState>,
    /// Overlay emphasis classes this consumer distinguishes.
    pub overlay_emphasis_classes: Vec<M5OverlayEmphasisClass>,
    /// Overlay postures this consumer distinguishes.
    pub overlay_postures: Vec<M5OverlayMarkerPosture>,
    /// Bounded summary actions this consumer offers.
    pub summary_actions: Vec<M5CoverageSummaryAction>,
    /// Bounded overlay actions this consumer offers.
    pub overlay_actions: Vec<M5OverlayMarkerAction>,
    /// Summary export fields this consumer carries (must include the mandatory fields).
    pub summary_export_fields: Vec<M5CoverageSummaryExportField>,
    /// Overlay export fields this consumer carries (must include the mandatory fields).
    pub overlay_export_fields: Vec<M5OverlayMarkerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5TestIntelligenceAccessibilityRoute>,
    /// Test-intelligence subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5TestIntelligenceConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5TestIntelligenceDowngradeTrigger>,
    /// Proof packet refs that keep these components current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by these components.
    pub source_contract_refs: Vec<String>,
    /// Worked summary-bar resolutions proving the resolver on this consumer.
    pub summary_examples: Vec<M5CoverageSummaryResolutionCase>,
    /// Worked overlay-marker resolutions proving the resolver on this consumer.
    pub overlay_examples: Vec<M5OverlayMarkerResolutionCase>,
    /// Hard invariant: this consumer never collapses a multi-run or imported scope into one
    /// unlabeled percentage. MUST be `false`.
    pub collapses_multi_run_into_single_percentage: bool,
    /// Hard invariant: this consumer never hides a shard omission or a stale provenance behind
    /// a green number. MUST be `false`.
    pub hides_shard_omission_or_stale_provenance: bool,
    /// Hard invariant: this consumer never drops the line-versus-branch metric dimension.
    /// MUST be `false`.
    pub drops_line_versus_branch_dimension: bool,
    /// Hard invariant: this consumer never invents an alternate label for a governed coverage
    /// state. MUST be `false`.
    pub invents_alternate_coverage_state_label: bool,
}

impl M5CoverageComponentConsumerRow {
    /// True when the row declares every mandatory summary anatomy part.
    fn declares_mandatory_summary_anatomy(&self) -> bool {
        let present: BTreeSet<M5CoverageSummaryAnatomyPart> =
            self.summary_anatomy_parts.iter().copied().collect();
        M5CoverageSummaryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory overlay anatomy part.
    fn declares_mandatory_overlay_anatomy(&self) -> bool {
        let present: BTreeSet<M5OverlayMarkerAnatomyPart> =
            self.overlay_anatomy_parts.iter().copied().collect();
        M5OverlayMarkerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory summary export field.
    fn declares_mandatory_summary_export(&self) -> bool {
        let present: BTreeSet<M5CoverageSummaryExportField> =
            self.summary_export_fields.iter().copied().collect();
        M5CoverageSummaryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory overlay export field.
    fn declares_mandatory_overlay_export(&self) -> bool {
        let present: BTreeSet<M5OverlayMarkerExportField> =
            self.overlay_export_fields.iter().copied().collect();
        M5OverlayMarkerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_multi_run_into_single_percentage
            && !self.hides_shard_omission_or_stale_provenance
            && !self.drops_line_versus_branch_dimension
            && !self.invents_alternate_coverage_state_label
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoverageComponentVocabularySet {
    /// Coverage consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Summary-anatomy-part tokens.
    pub summary_anatomy_parts: Vec<String>,
    /// Overlay-anatomy-part tokens.
    pub overlay_anatomy_parts: Vec<String>,
    /// Coverage-posture tokens.
    pub coverage_postures: Vec<String>,
    /// Overlay-posture tokens.
    pub overlay_postures: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Source-note tokens.
    pub source_notes: Vec<String>,
    /// Summary-action tokens.
    pub summary_actions: Vec<String>,
    /// Overlay-action tokens.
    pub overlay_actions: Vec<String>,
    /// Summary-export-field tokens.
    pub summary_export_fields: Vec<String>,
    /// Overlay-export-field tokens.
    pub overlay_export_fields: Vec<String>,
    /// Coverage-scope-class tokens (reused from the frozen matrix).
    pub coverage_scope_classes: Vec<String>,
    /// Coverage-metric-kind tokens (reused from the frozen matrix).
    pub coverage_metric_kinds: Vec<String>,
    /// Provenance-class tokens (reused from the frozen matrix).
    pub provenance_classes: Vec<String>,
    /// Coverage-overlay-state tokens (reused from the frozen matrix).
    pub overlay_states: Vec<String>,
    /// Overlay-emphasis-class tokens (reused from the frozen matrix).
    pub overlay_emphasis_classes: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5CoverageComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5CoverageComponentConsumerSurface::ALL, |v| v.as_str()),
            summary_anatomy_parts: tokens(&M5CoverageSummaryAnatomyPart::ALL, |v| v.as_str()),
            overlay_anatomy_parts: tokens(&M5OverlayMarkerAnatomyPart::ALL, |v| v.as_str()),
            coverage_postures: tokens(&M5CoverageSummaryPosture::ALL, |v| v.as_str()),
            overlay_postures: tokens(&M5OverlayMarkerPosture::ALL, |v| v.as_str()),
            freshness_states: tokens(&M5CoverageFreshnessState::ALL, |v| v.as_str()),
            source_notes: tokens(&M5CoverageSourceNote::ALL, |v| v.as_str()),
            summary_actions: tokens(&M5CoverageSummaryAction::ALL, |v| v.as_str()),
            overlay_actions: tokens(&M5OverlayMarkerAction::ALL, |v| v.as_str()),
            summary_export_fields: tokens(&M5CoverageSummaryExportField::ALL, |v| v.as_str()),
            overlay_export_fields: tokens(&M5OverlayMarkerExportField::ALL, |v| v.as_str()),
            coverage_scope_classes: tokens(&M5CoverageScopeClass::ALL, |v| v.as_str()),
            coverage_metric_kinds: tokens(&M5CoverageMetricKind::ALL, |v| v.as_str()),
            provenance_classes: tokens(&M5TestIntelligenceProvenanceClass::ALL, |v| v.as_str()),
            overlay_states: tokens(&M5CoverageOverlayState::ALL, |v| v.as_str()),
            overlay_emphasis_classes: tokens(&M5OverlayEmphasisClass::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TestIntelligenceSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TestIntelligenceDeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TestIntelligenceAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
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
pub struct M5CoverageComponentGovernanceReview {
    /// The summary bar shows its scope and metric dimension.
    pub bar_shows_scope_and_metric_dimension: bool,
    /// The summary bar shows its included run set.
    pub bar_shows_included_run_set: bool,
    /// The summary bar shows its freshness and imported/merged/live source note.
    pub bar_shows_freshness_and_source_note: bool,
    /// A multi-run or imported summary never collapses into one unlabeled percentage.
    pub bar_never_collapses_multi_run_into_one_percentage: bool,
    /// The summary bar exposes an open-uncovered-lines action.
    pub bar_exposes_open_uncovered_lines: bool,
    /// The overlay marker preserves its exact coverage-state meaning.
    pub overlay_preserves_exact_state_meaning: bool,
    /// The overlay marker preserves its changed-line emphasis.
    pub overlay_preserves_changed_line_emphasis: bool,
    /// The overlay marker preserves its source run-set identity.
    pub overlay_preserves_source_run_set_identity: bool,
    /// The overlay marker offers a durable path back to the evidence object.
    pub overlay_offers_durable_path_back_to_evidence: bool,
    /// A shard omission or stale provenance is never hidden behind a single percentage.
    pub shard_omission_and_stale_never_hidden: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The components keep the same truth across every coverage consumer surface.
    pub components_stable_across_consumer_surfaces: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// The support / export packet reconstructs coverage truth.
    pub support_export_reconstructs_coverage_truth: bool,
    /// Later M5 coverage components cannot invent parallel coverage vocabulary.
    pub later_components_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoverageComponentConsumerProjection {
    /// Report and editor surfaces consume the shared coverage vocabulary.
    pub report_and_editor_surfaces_consume_coverage_vocabulary: bool,
    /// The coverage-posture resolver reads a single canonical source.
    pub summary_posture_reads_single_source: bool,
    /// The overlay-posture resolver reads a single canonical source.
    pub overlay_posture_reads_single_source: bool,
    /// The CI and support/export consumers read the same coverage vocabulary.
    pub ci_and_support_read_same_coverage_vocabulary: bool,
    /// Headless and desktop coverage read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoverageComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the two coverage components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoverageComponentReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting test-evidence audit.
    pub test_evidence_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CoverageComponentsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CoverageComponentsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Coverage consumer rows.
    pub rows: Vec<M5CoverageComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CoverageComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CoverageComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CoverageComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CoverageComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CoverageComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 coverage-summary-bar / overlay-marker primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoverageComponentsPacket {
    /// Record kind; must equal [`M5_COVERAGE_COMPONENTS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COVERAGE_COMPONENTS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Coverage consumer rows.
    pub rows: Vec<M5CoverageComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CoverageComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CoverageComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CoverageComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CoverageComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CoverageComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CoverageComponentsPacket {
    /// Builds an M5 coverage-components primitive packet from stable-lane input.
    pub fn new(input: M5CoverageComponentsPacketInput) -> Self {
        Self {
            record_kind: M5_COVERAGE_COMPONENTS_RECORD_KIND.to_owned(),
            schema_version: M5_COVERAGE_COMPONENTS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
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

    /// Validates the M5 coverage-components primitive invariants.
    pub fn validate(&self) -> Vec<M5CoverageComponentViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COVERAGE_COMPONENTS_RECORD_KIND {
            violations.push(M5CoverageComponentViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COVERAGE_COMPONENTS_SCHEMA_VERSION {
            violations.push(M5CoverageComponentViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CoverageComponentViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_coverage_posture_coverage(self, &mut violations);
        validate_overlay_posture_coverage(self, &mut violations);
        validate_multi_run_disclosure(self, &mut violations);
        validate_stale_disclosure(self, &mut violations);
        validate_report_continuity(self, &mut violations);
        validate_changed_line_emphasis(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 coverage components packet serializes"),
        ) {
            violations.push(M5CoverageComponentViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 coverage components packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per coverage consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,summary_anatomy,coverage_postures,source_notes,overlay_postures,overlay_states,summary_actions,overlay_actions,summary_examples,overlay_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.summary_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.coverage_postures, |v| v.as_str()),
                join_tokens(&row.source_notes, |v| v.as_str()),
                join_tokens(&row.overlay_postures, |v| v.as_str()),
                join_tokens(&row.overlay_states, |v| v.as_str()),
                join_tokens(&row.summary_actions, |v| v.as_str()),
                join_tokens(&row.overlay_actions, |v| v.as_str()),
                row.summary_examples.len(),
                row.overlay_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Coverage-Summary-Bar / Coverage-Overlay-Marker Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Coverage consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Coverage postures: {}\n",
            self.vocabulary_set.coverage_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Overlay postures: {}\n",
            self.vocabulary_set.overlay_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Source notes: {}\n",
            self.vocabulary_set.source_notes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Coverage consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked summaries: {} / overlays: {}\n",
                row.summary_examples.len(),
                row.overlay_examples.len()
            ));
            for case in &row.summary_examples {
                out.push_str(&format!(
                    "    - summary `{}` (`{}`) -> `{}` (multi-run `{}`, imported `{}`, stale `{}`)\n",
                    case.resolved.summary_identity_ref,
                    case.resolved.scope_class.as_str(),
                    case.resolved.coverage_posture.as_str(),
                    case.resolved.is_multi_run,
                    case.resolved.is_imported,
                    case.resolved.is_stale,
                ));
            }
            for case in &row.overlay_examples {
                out.push_str(&format!(
                    "    - overlay `{}` (`{}`) -> `{}` (changed `{}`, continuity `{}`)\n",
                    case.resolved.evidence_object_ref,
                    case.resolved.overlay_state.as_str(),
                    case.resolved.overlay_posture.as_str(),
                    case.resolved.is_emphasized_change,
                    case.resolved.has_report_continuity,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 coverage-components export.
#[derive(Debug)]
pub enum M5CoverageComponentArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CoverageComponentViolation>),
}

impl fmt::Display for M5CoverageComponentArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 coverage components export parse failed: {error}"
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
                    "m5 coverage components export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CoverageComponentArtifactError {}

/// Validation failures emitted by [`M5CoverageComponentsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CoverageComponentViolation {
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
    /// A required coverage consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A coverage consumer row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory summary anatomy parts.
    MandatorySummaryAnatomyMissing,
    /// A row omits one of the mandatory overlay anatomy parts.
    MandatoryOverlayAnatomyMissing,
    /// A row omits one of the mandatory summary export fields.
    MandatorySummaryExportMissing,
    /// A row omits one of the mandatory overlay export fields.
    MandatoryOverlayExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked summary or overlay resolutions.
    ExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every coverage posture (so a scope that would
    /// otherwise collapse into one percentage goes unproven).
    CoveragePostureCoverageUnproven,
    /// The worked resolutions do not exercise every overlay posture.
    OverlayPostureCoverageUnproven,
    /// The worked resolutions do not prove both a labeled multi-run/imported summary and a
    /// single-run one.
    MultiRunDisclosureUnproven,
    /// The worked resolutions do not prove both a stale summary and a fresh one.
    StaleDisclosureUnproven,
    /// A worked overlay resolution does not preserve a durable path back to the evidence
    /// object.
    ReportContinuityUnproven,
    /// The worked resolutions do not prove an emphasized changed-line overlay.
    ChangedLineEmphasisUnproven,
    /// A worked resolution does not preserve its exact identity and label.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
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

impl M5CoverageComponentViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatorySummaryAnatomyMissing => "mandatory_summary_anatomy_missing",
            Self::MandatoryOverlayAnatomyMissing => "mandatory_overlay_anatomy_missing",
            Self::MandatorySummaryExportMissing => "mandatory_summary_export_missing",
            Self::MandatoryOverlayExportMissing => "mandatory_overlay_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleMissing => "example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::CoveragePostureCoverageUnproven => "coverage_posture_coverage_unproven",
            Self::OverlayPostureCoverageUnproven => "overlay_posture_coverage_unproven",
            Self::MultiRunDisclosureUnproven => "multi_run_disclosure_unproven",
            Self::StaleDisclosureUnproven => "stale_disclosure_unproven",
            Self::ReportContinuityUnproven => "report_continuity_unproven",
            Self::ChangedLineEmphasisUnproven => "changed_line_emphasis_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 coverage-components export.
pub fn current_stable_m5_coverage_components_export(
) -> Result<M5CoverageComponentsPacket, M5CoverageComponentArtifactError> {
    let packet: M5CoverageComponentsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-coverage-summary-overlay-primitive-proof/support_export.json"
    )))
    .map_err(M5CoverageComponentArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CoverageComponentArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COVERAGE_COMPONENTS_SUMMARY_SCHEMA_REF,
        M5_COVERAGE_COMPONENTS_OVERLAY_SCHEMA_REF,
        M5_COVERAGE_COMPONENTS_DOC_REF,
        M5_COVERAGE_COMPONENTS_COMPONENT_MATRIX_REF,
        M5_COVERAGE_COMPONENTS_COVERAGE_MERGE_REF,
        M5_COVERAGE_COMPONENTS_COVERAGE_OVERLAY_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CoverageComponentViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5CoverageComponentViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    let present: BTreeSet<M5CoverageComponentConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5CoverageComponentConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5CoverageComponentViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.summary_anatomy_parts.is_empty()
            || row.overlay_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.coverage_scope_classes.is_empty()
            || row.coverage_metric_kinds.is_empty()
            || row.provenance_classes.is_empty()
            || row.freshness_states.is_empty()
            || row.source_notes.is_empty()
            || row.coverage_postures.is_empty()
            || row.overlay_states.is_empty()
            || row.overlay_emphasis_classes.is_empty()
            || row.overlay_postures.is_empty()
            || row.summary_actions.is_empty()
            || row.overlay_actions.is_empty()
            || row.summary_export_fields.is_empty()
            || row.overlay_export_fields.is_empty()
        {
            violations.push(M5CoverageComponentViolation::RowIncomplete);
        }
        if !row.declares_mandatory_summary_anatomy() {
            violations.push(M5CoverageComponentViolation::MandatorySummaryAnatomyMissing);
        }
        if !row.declares_mandatory_overlay_anatomy() {
            violations.push(M5CoverageComponentViolation::MandatoryOverlayAnatomyMissing);
        }
        if !row.declares_mandatory_summary_export() {
            violations.push(M5CoverageComponentViolation::MandatorySummaryExportMissing);
        }
        if !row.declares_mandatory_overlay_export() {
            violations.push(M5CoverageComponentViolation::MandatoryOverlayExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TestIntelligenceAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5CoverageComponentViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5CoverageComponentViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5CoverageComponentViolation::DowngradeTriggersMissing);
        }
        if row.summary_examples.is_empty() || row.overlay_examples.is_empty() {
            violations.push(M5CoverageComponentViolation::ExampleMissing);
        }
        if row
            .summary_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .overlay_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5CoverageComponentViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5CoverageComponentViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5CoverageComponentViolation::RowInvariantViolated);
        }
    }
}

/// Every coverage posture must be exercised by some worked resolution — the acceptance-criterion
/// proof that full-suite, changed-files, single-shard, merged-multi-shard, imported-report, and
/// partial-incomplete scopes each get a distinct treatment rather than one collapsed percentage.
fn validate_coverage_posture_coverage(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    let exercised: BTreeSet<M5CoverageSummaryPosture> = packet
        .rows
        .iter()
        .flat_map(|row| row.summary_examples.iter())
        .map(|case| case.resolved.coverage_posture)
        .collect();
    let covered = M5CoverageSummaryPosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture));
    if !covered {
        violations.push(M5CoverageComponentViolation::CoveragePostureCoverageUnproven);
    }
}

/// Every overlay posture must be exercised by some worked resolution — the proof that the
/// controlled covered/uncovered/partial/branch-missed/excluded/unknown vocabulary is
/// distinguished.
fn validate_overlay_posture_coverage(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    let exercised: BTreeSet<M5OverlayMarkerPosture> = packet
        .rows
        .iter()
        .flat_map(|row| row.overlay_examples.iter())
        .map(|case| case.resolved.overlay_posture)
        .collect();
    let covered = M5OverlayMarkerPosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture));
    if !covered {
        violations.push(M5CoverageComponentViolation::OverlayPostureCoverageUnproven);
    }
}

/// At least one worked summary resolution must prove a multi-run or imported summary that
/// requires an included-run label, and at least one must prove a single-run summary — the
/// acceptance-criterion example that multi-run and imported evidence never collapse into one
/// unlabeled percentage.
fn validate_multi_run_disclosure(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    let has_multi = packet.rows.iter().any(|row| {
        row.summary_examples.iter().any(|case| {
            (case.resolved.is_multi_run || case.resolved.is_imported)
                && case.resolved.requires_included_run_label
        })
    });
    let has_single = packet.rows.iter().any(|row| {
        row.summary_examples
            .iter()
            .any(|case| !case.resolved.is_multi_run && !case.resolved.is_imported)
    });
    if !(has_multi && has_single) {
        violations.push(M5CoverageComponentViolation::MultiRunDisclosureUnproven);
    }
}

/// At least one worked summary resolution must prove a stale summary and at least one a fresh
/// one — the guardrail that stale provenance is never hidden behind a green number.
fn validate_stale_disclosure(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    let has_stale = packet.rows.iter().any(|row| {
        row.summary_examples
            .iter()
            .any(|case| case.resolved.is_stale)
    });
    let has_fresh = packet.rows.iter().any(|row| {
        row.summary_examples
            .iter()
            .any(|case| !case.resolved.is_stale)
    });
    if !(has_stale && has_fresh) {
        violations.push(M5CoverageComponentViolation::StaleDisclosureUnproven);
    }
}

/// Every worked overlay resolution must preserve a durable path back to the evidence object —
/// the acceptance-criterion requirement that an editor overlay never severs its report
/// continuity.
fn validate_report_continuity(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.overlay_examples.iter())
        .all(|case| case.resolved.has_report_continuity);
    if !preserved {
        violations.push(M5CoverageComponentViolation::ReportContinuityUnproven);
    }
}

/// At least one worked overlay resolution must prove an emphasized changed line — the
/// implementation requirement that changed-line emphasis is preserved.
fn validate_changed_line_emphasis(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    let has_emphasis = packet.rows.iter().any(|row| {
        row.overlay_examples
            .iter()
            .any(|case| case.resolved.is_emphasized_change)
    });
    if !has_emphasis {
        violations.push(M5CoverageComponentViolation::ChangedLineEmphasisUnproven);
    }
}

/// Every worked resolution must preserve its exact identity and label — the invariant that
/// neither component rewrites the user's coverage or evidence identity.
fn validate_identity_preservation(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    let summaries_preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.summary_examples.iter())
        .all(|case| case.preserves_identity());
    let overlays_preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.overlay_examples.iter())
        .all(|case| case.preserves_identity());
    if !(summaries_preserved && overlays_preserved) {
        violations.push(M5CoverageComponentViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.bar_shows_scope_and_metric_dimension,
        review.bar_shows_included_run_set,
        review.bar_shows_freshness_and_source_note,
        review.bar_never_collapses_multi_run_into_one_percentage,
        review.bar_exposes_open_uncovered_lines,
        review.overlay_preserves_exact_state_meaning,
        review.overlay_preserves_changed_line_emphasis,
        review.overlay_preserves_source_run_set_identity,
        review.overlay_offers_durable_path_back_to_evidence,
        review.shard_omission_and_stale_never_hidden,
        review.components_stable_across_deployment_lines,
        review.components_stable_across_consumer_surfaces,
        review.every_component_declares_accessibility_route,
        review.support_export_reconstructs_coverage_truth,
        review.later_components_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5CoverageComponentViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.report_and_editor_surfaces_consume_coverage_vocabulary,
        projection.summary_posture_reads_single_source,
        projection.overlay_posture_reads_single_source,
        projection.ci_and_support_read_same_coverage_vocabulary,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5CoverageComponentViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CoverageComponentViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CoverageComponentsPacket,
    violations: &mut Vec<M5CoverageComponentViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.test_evidence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CoverageComponentViolation::ReleasePostureIncomplete);
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
