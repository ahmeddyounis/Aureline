//! Frozen M5 coverage-summary-bar, coverage-overlay-marker, flaky-state-badge,
//! retry-history-row, snapshot-review-card, coverage-import-merge-sheet, and
//! test-generation-suggestion-card component matrix.
//!
//! This module locks Aureline's reusable test-intelligence, quality-evidence, and
//! AI-generated-test review components into one export-safe packet. Every
//! coverage-, flake-, snapshot-, and generated-test-facing subcomponent M5 claims
//! that still drifts too easily by editor overlay, coverage report, test tree,
//! review surface, CI summary, or CLI surface — the coverage-summary bar, the
//! coverage-overlay marker, the flaky-state badge, the retry-history row, the
//! snapshot/golden review card, the coverage-import/merge sheet, and the
//! test-generation suggestion card — is named once here and constrained by the same
//! included-run provenance, freshness / source class, line-versus-branch measure,
//! artifact baseline identity, raw / text fallback, classifier confidence, and
//! generated-test assumption boundary regardless of the surface family that renders
//! it.
//!
//! What this matrix freezes is the stable vocabulary for the *components*
//! themselves: the component families, the one controlled provenance vocabulary
//! (`verified current run`, `imported CI artifact`, `cached local result`, `stale
//! prior result`, `suspected flaky`, `reproduced flaky`, `stable again`, `manually
//! muted`, and `unknown`) every component binds, the coverage scope classes and
//! metric kinds, the overlay states and emphasis classes, the flaky classifications
//! and confidence classes, the retry attempt outcomes and rerun scope classes, the
//! snapshot baseline identities and diff states, the coverage import sources and
//! merge-resolution states, the generated-test assumption classes and apply scopes,
//! the deployment lines every component must survive, the non-visual accessibility
//! routes, and the mandatory labels every component must be able to show. It does
//! not re-architect coverage import backends, runner backends, or CI provider
//! integrations that already own those records — it is the shared quality-evidence
//! component contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 editor,
//! coverage report, test tree, review, CI summary, or CLI surface may publish a
//! coverage, overlay, flake, retry, snapshot, import/merge, or generated-test claim.
//! Coverage, overlay, flaky, retry, snapshot, import, generation, and export
//! consumers all read this packet so one coverage-summary bar names its scope and
//! metric, one coverage-overlay marker names its overlay state and emphasis, one
//! flaky-state badge names its classification and classifier confidence, one
//! retry-history row names its attempt outcome and rerun scope, one snapshot-review
//! card names its baseline identity and diff state, one coverage-import/merge sheet
//! names its source and merge resolution, and one test-generation suggestion card
//! names its assumptions and apply scope. No M5 lane invents a second test-evidence
//! grammar or an alternate label for a stale / imported result, an omitted shard, an
//! intermittent-versus-confirmed flake, or an opaque generated-test apply path.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5TestIntelligenceComponentVocabularySet`] rather than minted per surface. Raw
//! log bodies, pasted paths, credentials, and private endpoints stay outside the
//! export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_test_intelligence_component_matrix,
    seeded_m5_test_intelligence_component_matrix_coverage_import_merge_sheet_preview_narrowed,
    seeded_m5_test_intelligence_component_matrix_flaky_state_badge_beta_narrowed,
    M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5TestIntelligenceComponentMatrixPacket`].
pub const M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix";

/// Schema version for M5 test-intelligence component-matrix records.
pub const M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the test-intelligence component boundary schema.
pub const M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-test-intelligence-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_TEST_INTELLIGENCE_COMPONENT_DOC_REF: &str =
    "docs/testing/m5_test_intelligence_component_matrix.md";

/// Repo-relative path of the coverage-merge-result contract this matrix binds against.
pub const M5_TEST_INTELLIGENCE_COMPONENT_COVERAGE_MERGE_REF: &str =
    "schemas/testing/coverage_merge_result.schema.json";

/// Repo-relative path of the coverage-overlay / snapshot-golden contract this matrix
/// binds against.
pub const M5_TEST_INTELLIGENCE_COMPONENT_COVERAGE_OVERLAY_REF: &str =
    "schemas/testing/coverage-overlays-and-snapshot-golden-review.schema.json";

/// Repo-relative path of the flaky-verdict contract this matrix binds against.
pub const M5_TEST_INTELLIGENCE_COMPONENT_FLAKY_VERDICT_REF: &str =
    "schemas/testing/flaky_verdict.schema.json";

/// Repo-relative path of the test-attempt (retry-history) contract this matrix binds
/// against.
pub const M5_TEST_INTELLIGENCE_COMPONENT_TEST_ATTEMPT_REF: &str =
    "schemas/testing/test_attempt.schema.json";

/// Repo-relative path of the snapshot-acceptance-review contract this matrix binds
/// against.
pub const M5_TEST_INTELLIGENCE_COMPONENT_SNAPSHOT_REVIEW_REF: &str =
    "schemas/testing/snapshot_acceptance_review.schema.json";

/// Repo-relative path of the test-generation-suggestion contract this matrix binds
/// against.
pub const M5_TEST_INTELLIGENCE_COMPONENT_TEST_GENERATION_REF: &str =
    "schemas/testing/test-generation-suggestion-cards-and-diff-first-apply.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_TEST_INTELLIGENCE_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-test-intelligence-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TEST_INTELLIGENCE_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-test-intelligence-component-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_TEST_INTELLIGENCE_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-test-intelligence-component-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_TEST_INTELLIGENCE_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-test-intelligence-component-matrix.md";

/// One of the seven governed test-intelligence component families this matrix
/// freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestIntelligenceComponentFamily {
    /// A coverage-summary bar carrying its included-run scope and line/branch metric.
    CoverageSummaryBar,
    /// A coverage-overlay marker carrying its gutter state and changed-line emphasis.
    CoverageOverlayMarker,
    /// A flaky-state badge carrying its classification and classifier confidence.
    FlakyStateBadge,
    /// A retry-history row carrying its attempt outcome and rerun scope.
    RetryHistoryRow,
    /// A snapshot / golden review card carrying its baseline identity and diff state.
    SnapshotReviewCard,
    /// A coverage-import / merge sheet carrying its source and merge resolution.
    CoverageImportMergeSheet,
    /// A test-generation suggestion card carrying its assumptions and apply scope.
    TestGenerationSuggestionCard,
}

impl M5TestIntelligenceComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::CoverageSummaryBar,
        Self::CoverageOverlayMarker,
        Self::FlakyStateBadge,
        Self::RetryHistoryRow,
        Self::SnapshotReviewCard,
        Self::CoverageImportMergeSheet,
        Self::TestGenerationSuggestionCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoverageSummaryBar => "coverage_summary_bar",
            Self::CoverageOverlayMarker => "coverage_overlay_marker",
            Self::FlakyStateBadge => "flaky_state_badge",
            Self::RetryHistoryRow => "retry_history_row",
            Self::SnapshotReviewCard => "snapshot_review_card",
            Self::CoverageImportMergeSheet => "coverage_import_merge_sheet",
            Self::TestGenerationSuggestionCard => "test_generation_suggestion_card",
        }
    }

    /// `true` when this family is a coverage-summary bar and must therefore declare
    /// its coverage scope classes and metric kinds.
    pub const fn is_coverage_summary_bar(self) -> bool {
        matches!(self, Self::CoverageSummaryBar)
    }

    /// `true` when this family is a coverage-overlay marker and must therefore
    /// declare its overlay states and emphasis classes.
    pub const fn is_coverage_overlay_marker(self) -> bool {
        matches!(self, Self::CoverageOverlayMarker)
    }

    /// `true` when this family is a flaky-state badge and must therefore declare its
    /// flaky classifications and confidence classes.
    pub const fn is_flaky_state_badge(self) -> bool {
        matches!(self, Self::FlakyStateBadge)
    }

    /// `true` when this family is a retry-history row and must therefore declare its
    /// retry attempt outcomes and rerun scope classes.
    pub const fn is_retry_history_row(self) -> bool {
        matches!(self, Self::RetryHistoryRow)
    }

    /// `true` when this family is a snapshot-review card and must therefore declare
    /// its baseline identities and diff states.
    pub const fn is_snapshot_review_card(self) -> bool {
        matches!(self, Self::SnapshotReviewCard)
    }

    /// `true` when this family is a coverage-import / merge sheet and must therefore
    /// declare its import sources and merge-resolution states.
    pub const fn is_coverage_import_merge_sheet(self) -> bool {
        matches!(self, Self::CoverageImportMergeSheet)
    }

    /// `true` when this family is a test-generation suggestion card and must
    /// therefore declare its assumption classes and apply scopes.
    pub const fn is_test_generation_suggestion_card(self) -> bool {
        matches!(self, Self::TestGenerationSuggestionCard)
    }
}

/// The one controlled provenance / freshness vocabulary every test-intelligence
/// component binds, so a coverage number, an overlay glyph, a flaky verdict, a retry
/// outcome, a snapshot accept, or a generated test never leaves its origin implicit.
/// These are the exact tokens the acceptance criteria require.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestIntelligenceProvenanceClass {
    /// A result verified by the current local run.
    VerifiedCurrentRun,
    /// A result imported from a CI artifact.
    ImportedCiArtifact,
    /// A cached local result reused without a fresh run.
    CachedLocalResult,
    /// A stale prior result older than the current source.
    StalePriorResult,
    /// A test suspected flaky from a single or unconfirmed occurrence.
    SuspectedFlaky,
    /// A test whose flakiness has been reproduced across attempts.
    ReproducedFlaky,
    /// A previously flaky test that is stable again.
    StableAgain,
    /// A test manually muted / quarantined by a human.
    ManuallyMuted,
    /// An unknown or unattributable provenance.
    Unknown,
}

impl M5TestIntelligenceProvenanceClass {
    /// Every provenance class, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::VerifiedCurrentRun,
        Self::ImportedCiArtifact,
        Self::CachedLocalResult,
        Self::StalePriorResult,
        Self::SuspectedFlaky,
        Self::ReproducedFlaky,
        Self::StableAgain,
        Self::ManuallyMuted,
        Self::Unknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCurrentRun => "verified_current_run",
            Self::ImportedCiArtifact => "imported_ci_artifact",
            Self::CachedLocalResult => "cached_local_result",
            Self::StalePriorResult => "stale_prior_result",
            Self::SuspectedFlaky => "suspected_flaky",
            Self::ReproducedFlaky => "reproduced_flaky",
            Self::StableAgain => "stable_again",
            Self::ManuallyMuted => "manually_muted",
            Self::Unknown => "unknown",
        }
    }
}

/// Controlled coverage scope class — which run set a coverage-summary bar measures,
/// so a single percentage never hides a partial or single-shard scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageScopeClass {
    /// The full test suite.
    FullSuite,
    /// Changed files only.
    ChangedFilesOnly,
    /// A single shard.
    SingleShard,
    /// A merged multi-shard run.
    MergedMultiShard,
    /// An imported coverage report.
    ImportedReport,
    /// A partial / incomplete scope.
    PartialIncomplete,
}

impl M5CoverageScopeClass {
    /// Every coverage scope class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullSuite,
        Self::ChangedFilesOnly,
        Self::SingleShard,
        Self::MergedMultiShard,
        Self::ImportedReport,
        Self::PartialIncomplete,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSuite => "full_suite",
            Self::ChangedFilesOnly => "changed_files_only",
            Self::SingleShard => "single_shard",
            Self::MergedMultiShard => "merged_multi_shard",
            Self::ImportedReport => "imported_report",
            Self::PartialIncomplete => "partial_incomplete",
        }
    }
}

/// Controlled coverage metric kind — line-versus-branch support, so a summary bar
/// never conflates distinct measures under one number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageMetricKind {
    /// Line coverage.
    LineCoverage,
    /// Branch coverage.
    BranchCoverage,
    /// Function coverage.
    FunctionCoverage,
    /// Statement coverage.
    StatementCoverage,
    /// Region coverage.
    RegionCoverage,
    /// A combined metric summarizing several measures.
    CombinedMetric,
}

impl M5CoverageMetricKind {
    /// Every coverage metric kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LineCoverage,
        Self::BranchCoverage,
        Self::FunctionCoverage,
        Self::StatementCoverage,
        Self::RegionCoverage,
        Self::CombinedMetric,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LineCoverage => "line_coverage",
            Self::BranchCoverage => "branch_coverage",
            Self::FunctionCoverage => "function_coverage",
            Self::StatementCoverage => "statement_coverage",
            Self::RegionCoverage => "region_coverage",
            Self::CombinedMetric => "combined_metric",
        }
    }
}

/// Controlled coverage-overlay state — what a per-line overlay marker asserts, so a
/// gutter glyph is never left ambiguous about coverage status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageOverlayState {
    /// The line is covered.
    CoveredLine,
    /// The line is uncovered.
    UncoveredLine,
    /// The line is partially covered.
    PartiallyCovered,
    /// A branch on the line was missed.
    BranchMissed,
    /// The line is excluded from coverage.
    ExcludedLine,
    /// No overlay data is available for the line.
    NoOverlayData,
}

impl M5CoverageOverlayState {
    /// Every coverage overlay state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CoveredLine,
        Self::UncoveredLine,
        Self::PartiallyCovered,
        Self::BranchMissed,
        Self::ExcludedLine,
        Self::NoOverlayData,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoveredLine => "covered_line",
            Self::UncoveredLine => "uncovered_line",
            Self::PartiallyCovered => "partially_covered",
            Self::BranchMissed => "branch_missed",
            Self::ExcludedLine => "excluded_line",
            Self::NoOverlayData => "no_overlay_data",
        }
    }
}

/// Controlled overlay emphasis class — changed-file emphasis for an overlay marker,
/// so a regression on a changed line is never lost in a wall of uniform gutter color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverlayEmphasisClass {
    /// A changed line is emphasized.
    ChangedLineEmphasis,
    /// A context (unchanged) line.
    ContextLine,
    /// A newly uncovered line.
    NewlyUncovered,
    /// A regression hotspot.
    RegressionHotspot,
    /// A stably covered line.
    StableCovered,
    /// A suppressed / de-emphasized region.
    SuppressedRegion,
}

impl M5OverlayEmphasisClass {
    /// Every overlay emphasis class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ChangedLineEmphasis,
        Self::ContextLine,
        Self::NewlyUncovered,
        Self::RegressionHotspot,
        Self::StableCovered,
        Self::SuppressedRegion,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChangedLineEmphasis => "changed_line_emphasis",
            Self::ContextLine => "context_line",
            Self::NewlyUncovered => "newly_uncovered",
            Self::RegressionHotspot => "regression_hotspot",
            Self::StableCovered => "stable_covered",
            Self::SuppressedRegion => "suppressed_region",
        }
    }
}

/// Controlled flaky classification — what a flaky-state badge asserts, so an
/// intermittent failure is never labelled as confirmed flakiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FlakyClassification {
    /// The test is stable.
    Stable,
    /// The test is suspected flaky.
    SuspectedFlaky,
    /// The test's flakiness has been reproduced.
    ReproducedFlaky,
    /// The test is stable again after a flaky period.
    StableAgain,
    /// The test is manually muted / quarantined.
    ManuallyMuted,
    /// The flaky state is unknown.
    UnknownFlaky,
}

impl M5FlakyClassification {
    /// Every flaky classification, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::SuspectedFlaky,
        Self::ReproducedFlaky,
        Self::StableAgain,
        Self::ManuallyMuted,
        Self::UnknownFlaky,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::SuspectedFlaky => "suspected_flaky",
            Self::ReproducedFlaky => "reproduced_flaky",
            Self::StableAgain => "stable_again",
            Self::ManuallyMuted => "manually_muted",
            Self::UnknownFlaky => "unknown_flaky",
        }
    }
}

/// Controlled flaky confidence class — how confident the classifier is, so a
/// single-occurrence suspicion never presents with the authority of a reproduced
/// verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FlakyConfidenceClass {
    /// High classifier confidence.
    HighConfidence,
    /// Moderate classifier confidence.
    ModerateConfidence,
    /// Low classifier confidence.
    LowConfidence,
    /// A single observed occurrence.
    SingleOccurrence,
    /// Insufficient data to classify.
    InsufficientData,
    /// The verdict is overridden by policy.
    PolicyOverridden,
}

impl M5FlakyConfidenceClass {
    /// Every flaky confidence class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HighConfidence,
        Self::ModerateConfidence,
        Self::LowConfidence,
        Self::SingleOccurrence,
        Self::InsufficientData,
        Self::PolicyOverridden,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighConfidence => "high_confidence",
            Self::ModerateConfidence => "moderate_confidence",
            Self::LowConfidence => "low_confidence",
            Self::SingleOccurrence => "single_occurrence",
            Self::InsufficientData => "insufficient_data",
            Self::PolicyOverridden => "policy_overridden",
        }
    }
}

/// Controlled retry attempt outcome — what a single attempt in a retry-history row
/// resulted in, so a pass-on-retry is never shown as a clean first-try pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetryAttemptOutcome {
    /// Passed on the first try.
    PassedFirstTry,
    /// Passed on a retry.
    PassedOnRetry,
    /// Failed across all retries.
    FailedAllRetries,
    /// Errored before asserting.
    ErroredAttempt,
    /// Skipped attempt.
    SkippedAttempt,
    /// Aborted attempt.
    AbortedAttempt,
}

impl M5RetryAttemptOutcome {
    /// Every retry attempt outcome, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PassedFirstTry,
        Self::PassedOnRetry,
        Self::FailedAllRetries,
        Self::ErroredAttempt,
        Self::SkippedAttempt,
        Self::AbortedAttempt,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PassedFirstTry => "passed_first_try",
            Self::PassedOnRetry => "passed_on_retry",
            Self::FailedAllRetries => "failed_all_retries",
            Self::ErroredAttempt => "errored_attempt",
            Self::SkippedAttempt => "skipped_attempt",
            Self::AbortedAttempt => "aborted_attempt",
        }
    }
}

/// Controlled retry scope class — how the rerun behind a retry-history row was
/// scoped, so a widened rerun is never presented as the same selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetryScopeClass {
    /// The same selection was rerun.
    SameSelection,
    /// Only failed tests were rerun.
    FailedOnlyRerun,
    /// The selection was widened.
    WidenedSelection,
    /// A single test was rerun.
    SingleTestRerun,
    /// An imported attempt from another run.
    ImportedAttempt,
    /// An unknown rerun scope.
    UnknownScope,
}

impl M5RetryScopeClass {
    /// Every retry scope class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SameSelection,
        Self::FailedOnlyRerun,
        Self::WidenedSelection,
        Self::SingleTestRerun,
        Self::ImportedAttempt,
        Self::UnknownScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameSelection => "same_selection",
            Self::FailedOnlyRerun => "failed_only_rerun",
            Self::WidenedSelection => "widened_selection",
            Self::SingleTestRerun => "single_test_rerun",
            Self::ImportedAttempt => "imported_attempt",
            Self::UnknownScope => "unknown_scope",
        }
    }
}

/// Controlled snapshot baseline identity — which baseline a snapshot / golden review
/// card compares against, so an imported baseline never reads as a local accept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SnapshotBaselineIdentity {
    /// A committed baseline.
    CommittedBaseline,
    /// A pending new baseline.
    PendingNewBaseline,
    /// An updated baseline.
    UpdatedBaseline,
    /// An imported baseline.
    ImportedBaseline,
    /// A missing baseline.
    MissingBaseline,
    /// An ambiguous baseline.
    AmbiguousBaseline,
}

impl M5SnapshotBaselineIdentity {
    /// Every snapshot baseline identity, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CommittedBaseline,
        Self::PendingNewBaseline,
        Self::UpdatedBaseline,
        Self::ImportedBaseline,
        Self::MissingBaseline,
        Self::AmbiguousBaseline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommittedBaseline => "committed_baseline",
            Self::PendingNewBaseline => "pending_new_baseline",
            Self::UpdatedBaseline => "updated_baseline",
            Self::ImportedBaseline => "imported_baseline",
            Self::MissingBaseline => "missing_baseline",
            Self::AmbiguousBaseline => "ambiguous_baseline",
        }
    }
}

/// Controlled snapshot diff state — what a snapshot / golden review card shows about
/// the diff, so a binary-only change always keeps a raw / text fallback and is never
/// blind-accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SnapshotDiffState {
    /// The snapshot matches the baseline.
    MatchesBaseline,
    /// A diff was detected.
    DiffDetected,
    /// A new snapshot with no baseline.
    NewSnapshot,
    /// An obsolete snapshot no longer produced.
    ObsoleteSnapshot,
    /// The rendered diff is unavailable.
    RenderUnavailable,
    /// A raw / text fallback view is shown.
    RawTextFallback,
}

impl M5SnapshotDiffState {
    /// Every snapshot diff state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MatchesBaseline,
        Self::DiffDetected,
        Self::NewSnapshot,
        Self::ObsoleteSnapshot,
        Self::RenderUnavailable,
        Self::RawTextFallback,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MatchesBaseline => "matches_baseline",
            Self::DiffDetected => "diff_detected",
            Self::NewSnapshot => "new_snapshot",
            Self::ObsoleteSnapshot => "obsolete_snapshot",
            Self::RenderUnavailable => "render_unavailable",
            Self::RawTextFallback => "raw_text_fallback",
        }
    }
}

/// Controlled coverage import source — where a coverage-import / merge sheet drew a
/// report from, so a local run is never confused with an imported or cached report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoverageImportSource {
    /// A local run.
    LocalRun,
    /// An imported CI artifact.
    ImportedCiArtifact,
    /// A cached local report.
    CachedLocal,
    /// A stale prior report.
    StalePrior,
    /// An uploaded report.
    UploadedReport,
    /// An unknown source.
    UnknownSource,
}

impl M5CoverageImportSource {
    /// Every coverage import source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalRun,
        Self::ImportedCiArtifact,
        Self::CachedLocal,
        Self::StalePrior,
        Self::UploadedReport,
        Self::UnknownSource,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalRun => "local_run",
            Self::ImportedCiArtifact => "imported_ci_artifact",
            Self::CachedLocal => "cached_local",
            Self::StalePrior => "stale_prior",
            Self::UploadedReport => "uploaded_report",
            Self::UnknownSource => "unknown_source",
        }
    }
}

/// Controlled merge-resolution state — how a coverage-import / merge sheet resolved
/// overlapping reports, so a shard omission is never hidden behind a merged total.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MergeResolutionState {
    /// The reports merged cleanly.
    MergedClean,
    /// A shard omission was detected.
    ShardOmissionDetected,
    /// The reports conflicted / overlapped.
    ConflictingOverlap,
    /// A partial merge.
    PartialMerge,
    /// The report was superseded by a newer run.
    SupersededByNewer,
    /// The merge is unavailable.
    MergeUnavailable,
}

impl M5MergeResolutionState {
    /// Every merge-resolution state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MergedClean,
        Self::ShardOmissionDetected,
        Self::ConflictingOverlap,
        Self::PartialMerge,
        Self::SupersededByNewer,
        Self::MergeUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MergedClean => "merged_clean",
            Self::ShardOmissionDetected => "shard_omission_detected",
            Self::ConflictingOverlap => "conflicting_overlap",
            Self::PartialMerge => "partial_merge",
            Self::SupersededByNewer => "superseded_by_newer",
            Self::MergeUnavailable => "merge_unavailable",
        }
    }
}

/// Controlled generated-test assumption class — what a test-generation suggestion
/// card assumed, so a generated test never hides the assumptions behind its
/// assertions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeneratedAssumptionClass {
    /// A fixture was assumed.
    FixtureAssumed,
    /// An assertion was inferred.
    AssertionInferred,
    /// A snapshot was generated.
    SnapshotGenerated,
    /// A mock was synthesized.
    MockSynthesized,
    /// A dependency was assumed.
    DependencyAssumed,
    /// An unverified behavior was asserted.
    UnverifiedBehavior,
}

impl M5GeneratedAssumptionClass {
    /// Every generated-test assumption class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FixtureAssumed,
        Self::AssertionInferred,
        Self::SnapshotGenerated,
        Self::MockSynthesized,
        Self::DependencyAssumed,
        Self::UnverifiedBehavior,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FixtureAssumed => "fixture_assumed",
            Self::AssertionInferred => "assertion_inferred",
            Self::SnapshotGenerated => "snapshot_generated",
            Self::MockSynthesized => "mock_synthesized",
            Self::DependencyAssumed => "dependency_assumed",
            Self::UnverifiedBehavior => "unverified_behavior",
        }
    }
}

/// Controlled generated-test apply scope — what a test-generation suggestion card
/// would apply, so assertion, fixture, and snapshot changes are never silently
/// bundled into one opaque apply path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeneratedApplyScope {
    /// Assertions only.
    AssertionOnly,
    /// Fixture and assertion changes.
    FixtureAndAssertion,
    /// A snapshot is included.
    SnapshotIncluded,
    /// A full bundle apply.
    FullBundleApply,
    /// Review is required before apply.
    ReviewRequired,
    /// Apply is blocked.
    ApplyBlocked,
}

impl M5GeneratedApplyScope {
    /// Every generated-test apply scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AssertionOnly,
        Self::FixtureAndAssertion,
        Self::SnapshotIncluded,
        Self::FullBundleApply,
        Self::ReviewRequired,
        Self::ApplyBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssertionOnly => "assertion_only",
            Self::FixtureAndAssertion => "fixture_and_assertion",
            Self::SnapshotIncluded => "snapshot_included",
            Self::FullBundleApply => "full_bundle_apply",
            Self::ReviewRequired => "review_required",
            Self::ApplyBlocked => "apply_blocked",
        }
    }
}

/// Claimed M5 test surface family that renders / consumes a test-intelligence
/// component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestIntelligenceSurfaceFamily {
    /// The coverage-report surface.
    CoverageReport,
    /// The editor-overlay surface.
    EditorOverlay,
    /// The test-tree surface.
    TestTree,
    /// The review surface.
    ReviewSurface,
    /// The CI-summary surface.
    CiSummary,
    /// The CLI surface.
    CliSurface,
}

impl M5TestIntelligenceSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CoverageReport,
        Self::EditorOverlay,
        Self::TestTree,
        Self::ReviewSurface,
        Self::CiSummary,
        Self::CliSurface,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoverageReport => "coverage_report",
            Self::EditorOverlay => "editor_overlay",
            Self::TestTree => "test_tree",
            Self::ReviewSurface => "review_surface",
            Self::CiSummary => "ci_summary",
            Self::CliSurface => "cli_surface",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's
/// provenance, freshness, scope, or assumption truth never silently narrows or
/// widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestIntelligenceDeploymentLine {
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

impl M5TestIntelligenceDeploymentLine {
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

/// Test-intelligence subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestIntelligenceConsumerSurface {
    /// The coverage-report UI.
    CoverageReportUi,
    /// The editor-overlay UI.
    EditorOverlayUi,
    /// The flaky-dashboard UI.
    FlakyDashboardUi,
    /// The retry-history UI.
    RetryHistoryUi,
    /// The snapshot-review UI.
    SnapshotReviewUi,
    /// The coverage-import UI.
    CoverageImportUi,
    /// The test-generation UI.
    TestGenerationUi,
    /// The support export.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
}

impl M5TestIntelligenceConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::CoverageReportUi,
        Self::EditorOverlayUi,
        Self::FlakyDashboardUi,
        Self::RetryHistoryUi,
        Self::SnapshotReviewUi,
        Self::CoverageImportUi,
        Self::TestGenerationUi,
        Self::SupportExport,
        Self::CliInspect,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoverageReportUi => "coverage_report_ui",
            Self::EditorOverlayUi => "editor_overlay_ui",
            Self::FlakyDashboardUi => "flaky_dashboard_ui",
            Self::RetryHistoryUi => "retry_history_ui",
            Self::SnapshotReviewUi => "snapshot_review_ui",
            Self::CoverageImportUi => "coverage_import_ui",
            Self::TestGenerationUi => "test_generation_ui",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no test-evidence
/// truth is hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestIntelligenceAccessibilityRoute {
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

impl M5TestIntelligenceAccessibilityRoute {
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

/// Mandatory label a claimed test-intelligence component must be able to show. The
/// first three are hard requirements on every component; the remaining three close
/// the acceptance-criteria ambiguity about provenance / freshness, baseline / scope
/// identity, and generated-test assumption / recovery boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestIntelligenceRequiredLabel {
    /// The component's stable identity / what evidence object it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The provenance and freshness behind the component.
    ProvenanceAndFreshness,
    /// The baseline or scope identity behind the component.
    BaselineOrScopeIdentity,
    /// The generated-test assumption and recovery / rerun boundary.
    AssumptionAndRecoveryBoundary,
}

impl M5TestIntelligenceRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ProvenanceAndFreshness,
        Self::BaselineOrScopeIdentity,
        Self::AssumptionAndRecoveryBoundary,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ProvenanceAndFreshness => "provenance_and_freshness",
            Self::BaselineOrScopeIdentity => "baseline_or_scope_identity",
            Self::AssumptionAndRecoveryBoundary => "assumption_and_recovery_boundary",
        }
    }
}

/// Qualification class for an M5 test-intelligence component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestIntelligenceQualificationClass {
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

impl M5TestIntelligenceQualificationClass {
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

/// Downgrade trigger that narrows a test-intelligence component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestIntelligenceDowngradeTrigger {
    /// A component left its provenance class unstated.
    ProvenanceClassUnstated,
    /// A component left its freshness / source class undisclosed.
    FreshnessClassUndisclosed,
    /// A coverage summary or import hid a shard omission.
    ShardOmissionHidden,
    /// A coverage summary left its line-versus-branch metric unstated.
    LineVersusBranchUnstated,
    /// A flaky badge overstated classifier confidence.
    FlakyConfidenceOverstated,
    /// A retry row widened its rerun scope without disclosure.
    RetryScopeWidened,
    /// A snapshot card left its baseline identity unstated.
    SnapshotBaselineUnstated,
    /// A snapshot card dropped its raw / text fallback.
    RawTextFallbackMissing,
    /// A generation card hid an assumption behind its assertions.
    GeneratedAssumptionHidden,
    /// A generation card bundled changes into one opaque apply path.
    OpaqueApplyBundle,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5TestIntelligenceDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProvenanceClassUnstated,
        Self::FreshnessClassUndisclosed,
        Self::ShardOmissionHidden,
        Self::LineVersusBranchUnstated,
        Self::FlakyConfidenceOverstated,
        Self::RetryScopeWidened,
        Self::SnapshotBaselineUnstated,
        Self::RawTextFallbackMissing,
        Self::GeneratedAssumptionHidden,
        Self::OpaqueApplyBundle,
        Self::AlternateStateLabelInvented,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenanceClassUnstated => "provenance_class_unstated",
            Self::FreshnessClassUndisclosed => "freshness_class_undisclosed",
            Self::ShardOmissionHidden => "shard_omission_hidden",
            Self::LineVersusBranchUnstated => "line_versus_branch_unstated",
            Self::FlakyConfidenceOverstated => "flaky_confidence_overstated",
            Self::RetryScopeWidened => "retry_scope_widened",
            Self::SnapshotBaselineUnstated => "snapshot_baseline_unstated",
            Self::RawTextFallbackMissing => "raw_text_fallback_missing",
            Self::GeneratedAssumptionHidden => "generated_assumption_hidden",
            Self::OpaqueApplyBundle => "opaque_apply_bundle",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed test-intelligence component family bound to
/// the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestIntelligenceComponentRow {
    /// Governed component family.
    pub component_family: M5TestIntelligenceComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5TestIntelligenceQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 test surface families that render / consume this component.
    pub surface_families: Vec<M5TestIntelligenceSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5TestIntelligenceDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5TestIntelligenceRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5TestIntelligenceRequiredLabel>,
    /// Provenance / freshness classes this component binds (required for every
    /// family).
    pub provenance_classes: Vec<M5TestIntelligenceProvenanceClass>,
    /// Coverage scope classes this component names (coverage-summary-bar only).
    pub coverage_scope_classes: Vec<M5CoverageScopeClass>,
    /// Coverage metric kinds this component distinguishes (coverage-summary-bar only).
    pub coverage_metric_kinds: Vec<M5CoverageMetricKind>,
    /// Overlay states this component names (coverage-overlay-marker only).
    pub overlay_states: Vec<M5CoverageOverlayState>,
    /// Overlay emphasis classes this component names (coverage-overlay-marker only).
    pub overlay_emphasis_classes: Vec<M5OverlayEmphasisClass>,
    /// Flaky classifications this component names (flaky-state-badge only).
    pub flaky_classifications: Vec<M5FlakyClassification>,
    /// Flaky confidence classes this component names (flaky-state-badge only).
    pub flaky_confidence_classes: Vec<M5FlakyConfidenceClass>,
    /// Retry attempt outcomes this component names (retry-history-row only).
    pub retry_attempt_outcomes: Vec<M5RetryAttemptOutcome>,
    /// Retry scope classes this component names (retry-history-row only).
    pub retry_scope_classes: Vec<M5RetryScopeClass>,
    /// Snapshot baseline identities this component names (snapshot-review-card only).
    pub snapshot_baseline_identities: Vec<M5SnapshotBaselineIdentity>,
    /// Snapshot diff states this component names (snapshot-review-card only).
    pub snapshot_diff_states: Vec<M5SnapshotDiffState>,
    /// Coverage import sources this component names (coverage-import-merge-sheet
    /// only).
    pub coverage_import_sources: Vec<M5CoverageImportSource>,
    /// Merge-resolution states this component names (coverage-import-merge-sheet
    /// only).
    pub merge_resolution_states: Vec<M5MergeResolutionState>,
    /// Generated-test assumption classes this component names
    /// (test-generation-suggestion-card only).
    pub generated_assumption_classes: Vec<M5GeneratedAssumptionClass>,
    /// Generated-test apply scopes this component names
    /// (test-generation-suggestion-card only).
    pub generated_apply_scopes: Vec<M5GeneratedApplyScope>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5TestIntelligenceAccessibilityRoute>,
    /// Test-intelligence subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5TestIntelligenceConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5TestIntelligenceDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its provenance or freshness class.
    /// MUST be `false`.
    pub masks_provenance_or_freshness_class: bool,
    /// Hard invariant: this component never hides a shard omission behind a single
    /// percentage. MUST be `false`.
    pub hides_shard_omission_behind_single_percentage: bool,
    /// Hard invariant: this component never labels an intermittent failure as
    /// confirmed flakiness. MUST be `false`.
    pub labels_intermittent_failure_as_confirmed_flaky: bool,
    /// Hard invariant: this component never bundles generated changes into one opaque
    /// apply path. MUST be `false`.
    pub bundles_generated_changes_into_opaque_apply: bool,
    /// Hard invariant: this component never invents an alternate label for a governed
    /// state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl M5TestIntelligenceComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5TestIntelligenceRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5TestIntelligenceRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_provenance_or_freshness_class
            && !self.hides_shard_omission_behind_single_percentage
            && !self.labels_intermittent_failure_as_confirmed_flaky
            && !self.bundles_generated_changes_into_opaque_apply
            && !self.invents_alternate_state_label
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestIntelligenceComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Provenance-class tokens.
    pub provenance_classes: Vec<String>,
    /// Coverage-scope-class tokens.
    pub coverage_scope_classes: Vec<String>,
    /// Coverage-metric-kind tokens.
    pub coverage_metric_kinds: Vec<String>,
    /// Overlay-state tokens.
    pub overlay_states: Vec<String>,
    /// Overlay-emphasis-class tokens.
    pub overlay_emphasis_classes: Vec<String>,
    /// Flaky-classification tokens.
    pub flaky_classifications: Vec<String>,
    /// Flaky-confidence-class tokens.
    pub flaky_confidence_classes: Vec<String>,
    /// Retry-attempt-outcome tokens.
    pub retry_attempt_outcomes: Vec<String>,
    /// Retry-scope-class tokens.
    pub retry_scope_classes: Vec<String>,
    /// Snapshot-baseline-identity tokens.
    pub snapshot_baseline_identities: Vec<String>,
    /// Snapshot-diff-state tokens.
    pub snapshot_diff_states: Vec<String>,
    /// Coverage-import-source tokens.
    pub coverage_import_sources: Vec<String>,
    /// Merge-resolution-state tokens.
    pub merge_resolution_states: Vec<String>,
    /// Generated-assumption-class tokens.
    pub generated_assumption_classes: Vec<String>,
    /// Generated-apply-scope tokens.
    pub generated_apply_scopes: Vec<String>,
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

impl M5TestIntelligenceComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5TestIntelligenceComponentFamily::ALL, |v| v.as_str()),
            provenance_classes: tokens(&M5TestIntelligenceProvenanceClass::ALL, |v| v.as_str()),
            coverage_scope_classes: tokens(&M5CoverageScopeClass::ALL, |v| v.as_str()),
            coverage_metric_kinds: tokens(&M5CoverageMetricKind::ALL, |v| v.as_str()),
            overlay_states: tokens(&M5CoverageOverlayState::ALL, |v| v.as_str()),
            overlay_emphasis_classes: tokens(&M5OverlayEmphasisClass::ALL, |v| v.as_str()),
            flaky_classifications: tokens(&M5FlakyClassification::ALL, |v| v.as_str()),
            flaky_confidence_classes: tokens(&M5FlakyConfidenceClass::ALL, |v| v.as_str()),
            retry_attempt_outcomes: tokens(&M5RetryAttemptOutcome::ALL, |v| v.as_str()),
            retry_scope_classes: tokens(&M5RetryScopeClass::ALL, |v| v.as_str()),
            snapshot_baseline_identities: tokens(&M5SnapshotBaselineIdentity::ALL, |v| v.as_str()),
            snapshot_diff_states: tokens(&M5SnapshotDiffState::ALL, |v| v.as_str()),
            coverage_import_sources: tokens(&M5CoverageImportSource::ALL, |v| v.as_str()),
            merge_resolution_states: tokens(&M5MergeResolutionState::ALL, |v| v.as_str()),
            generated_assumption_classes: tokens(&M5GeneratedAssumptionClass::ALL, |v| v.as_str()),
            generated_apply_scopes: tokens(&M5GeneratedApplyScope::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TestIntelligenceSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TestIntelligenceDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5TestIntelligenceConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TestIntelligenceAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            required_labels: tokens(&M5TestIntelligenceRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5TestIntelligenceComponentGovernanceReview {
    /// The coverage-summary bar shows its scope and line/branch metric.
    pub coverage_summary_shows_scope_and_metric: bool,
    /// The coverage-overlay marker shows its state and changed-line emphasis.
    pub coverage_overlay_shows_state_and_emphasis: bool,
    /// The flaky-state badge shows its classification and classifier confidence.
    pub flaky_badge_shows_classification_and_confidence: bool,
    /// The retry-history row shows its attempt outcome and rerun scope.
    pub retry_history_shows_outcome_and_scope: bool,
    /// The snapshot-review card shows its baseline identity and diff state.
    pub snapshot_card_shows_baseline_and_diff: bool,
    /// The coverage-import / merge sheet shows its source and merge resolution.
    pub coverage_import_shows_source_and_merge_resolution: bool,
    /// The test-generation suggestion card shows its assumptions and apply scope.
    pub test_generation_shows_assumptions_and_apply_scope: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The one `verified current run` / `imported CI artifact` / `cached local
    /// result` / `stale prior result` provenance vocabulary is named once.
    pub provenance_vocabulary_named_once: bool,
    /// A single percentage never hides a shard omission or stale provenance.
    pub single_percentage_never_hides_shard_omission: bool,
    /// An intermittent failure is never labelled as confirmed flakiness.
    pub intermittent_never_labeled_confirmed_flaky: bool,
    /// Generated changes are never bundled into one opaque apply path.
    pub generated_changes_never_bundled_opaquely: bool,
    /// A raw / text fallback is always available for snapshot review.
    pub raw_text_fallback_always_available: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel test-evidence vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestIntelligenceComponentConsumerProjection {
    /// Coverage surfaces consume the shared scope and metric vocabulary.
    pub coverage_surfaces_consume_scope_and_metric_vocabulary: bool,
    /// Overlay surfaces consume the freshness and provenance vocabulary.
    pub overlay_surfaces_consume_freshness_and_provenance_vocabulary: bool,
    /// Flaky surfaces consume the classification and confidence vocabulary.
    pub flaky_surfaces_consume_classification_and_confidence_vocabulary: bool,
    /// Snapshot surfaces consume the baseline-identity vocabulary.
    pub snapshot_surfaces_consume_baseline_identity_vocabulary: bool,
    /// Generation surfaces consume the assumption vocabulary.
    pub generation_surfaces_consume_assumption_vocabulary: bool,
    /// Support / export reads a single canonical test-evidence source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestIntelligenceComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the test-intelligence component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestIntelligenceComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting test-evidence audit for the lane.
    pub test_evidence_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5TestIntelligenceComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TestIntelligenceComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5TestIntelligenceComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TestIntelligenceComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TestIntelligenceComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TestIntelligenceComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TestIntelligenceComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TestIntelligenceComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 test-intelligence component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestIntelligenceComponentMatrixPacket {
    /// Record kind; must equal [`M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5TestIntelligenceComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TestIntelligenceComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TestIntelligenceComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TestIntelligenceComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TestIntelligenceComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TestIntelligenceComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TestIntelligenceComponentMatrixPacket {
    /// Builds an M5 test-intelligence component matrix packet from stable-lane input.
    pub fn new(input: M5TestIntelligenceComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_SCHEMA_VERSION,
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

    /// Validates the M5 test-intelligence component matrix invariants.
    pub fn validate(&self) -> Vec<M5TestIntelligenceComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5TestIntelligenceComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5TestIntelligenceComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TestIntelligenceComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 test-intelligence component matrix packet serializes"),
        ) {
            violations.push(M5TestIntelligenceComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 test-intelligence component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
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
            "# M5 Coverage-Summary-Bar, Coverage-Overlay-Marker, Flaky-State-Badge, Retry-History-Row, Snapshot-Review-Card, Coverage-Import-Merge-Sheet, and Test-Generation-Suggestion-Card Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Provenance classes: {}\n",
            self.vocabulary_set.provenance_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Coverage metric kinds: {}\n",
            self.vocabulary_set.coverage_metric_kinds.join(", ")
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

/// Errors emitted when reading the checked-in M5 test-intelligence matrix export.
#[derive(Debug)]
pub enum M5TestIntelligenceComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TestIntelligenceComponentMatrixViolation>),
}

impl fmt::Display for M5TestIntelligenceComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 test-intelligence component matrix export parse failed: {error}"
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
                    "m5 test-intelligence component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TestIntelligenceComponentMatrixArtifactError {}

/// Validation failures emitted by
/// [`M5TestIntelligenceComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TestIntelligenceComponentMatrixViolation {
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
    /// A component row declares no provenance classes.
    ProvenanceClassMissing,
    /// A coverage-summary-bar component declares no coverage scope classes.
    CoverageScopeMissing,
    /// A coverage-summary-bar component declares no coverage metric kinds.
    CoverageMetricMissing,
    /// A coverage-overlay-marker component declares no overlay states.
    OverlayStateMissing,
    /// A coverage-overlay-marker component declares no overlay emphasis classes.
    OverlayEmphasisMissing,
    /// A flaky-state-badge component declares no flaky classifications.
    FlakyClassificationMissing,
    /// A flaky-state-badge component declares no flaky confidence classes.
    FlakyConfidenceMissing,
    /// A retry-history-row component declares no retry attempt outcomes.
    RetryOutcomeMissing,
    /// A retry-history-row component declares no retry scope classes.
    RetryScopeMissing,
    /// A snapshot-review-card component declares no baseline identities.
    SnapshotBaselineMissing,
    /// A snapshot-review-card component declares no snapshot diff states.
    SnapshotDiffStateMissing,
    /// A coverage-import-merge-sheet component declares no import sources.
    CoverageImportSourceMissing,
    /// A coverage-import-merge-sheet component declares no merge-resolution states.
    MergeResolutionMissing,
    /// A test-generation-suggestion-card component declares no assumption classes.
    GeneratedAssumptionMissing,
    /// A test-generation-suggestion-card component declares no apply scopes.
    GeneratedApplyScopeMissing,
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
    /// A component violates a hard invariant (masked provenance/freshness, hidden
    /// shard omission, intermittent labelled confirmed flaky, opaque generated apply,
    /// or invented alternate state label).
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

impl M5TestIntelligenceComponentMatrixViolation {
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
            Self::ProvenanceClassMissing => "provenance_class_missing",
            Self::CoverageScopeMissing => "coverage_scope_missing",
            Self::CoverageMetricMissing => "coverage_metric_missing",
            Self::OverlayStateMissing => "overlay_state_missing",
            Self::OverlayEmphasisMissing => "overlay_emphasis_missing",
            Self::FlakyClassificationMissing => "flaky_classification_missing",
            Self::FlakyConfidenceMissing => "flaky_confidence_missing",
            Self::RetryOutcomeMissing => "retry_outcome_missing",
            Self::RetryScopeMissing => "retry_scope_missing",
            Self::SnapshotBaselineMissing => "snapshot_baseline_missing",
            Self::SnapshotDiffStateMissing => "snapshot_diff_state_missing",
            Self::CoverageImportSourceMissing => "coverage_import_source_missing",
            Self::MergeResolutionMissing => "merge_resolution_missing",
            Self::GeneratedAssumptionMissing => "generated_assumption_missing",
            Self::GeneratedApplyScopeMissing => "generated_apply_scope_missing",
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

/// Reads and validates the checked-in stable M5 test-intelligence matrix export.
pub fn current_stable_m5_test_intelligence_component_matrix_export(
) -> Result<M5TestIntelligenceComponentMatrixPacket, M5TestIntelligenceComponentMatrixArtifactError>
{
    let packet: M5TestIntelligenceComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-test-intelligence-component-proof/support_export.json"
        )))
        .map_err(M5TestIntelligenceComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TestIntelligenceComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5TestIntelligenceComponentMatrixPacket,
    violations: &mut Vec<M5TestIntelligenceComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_DOC_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_COVERAGE_MERGE_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_COVERAGE_OVERLAY_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_FLAKY_VERDICT_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_TEST_ATTEMPT_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_SNAPSHOT_REVIEW_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_TEST_GENERATION_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TestIntelligenceComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5TestIntelligenceComponentMatrixPacket,
    violations: &mut Vec<M5TestIntelligenceComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5TestIntelligenceComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5TestIntelligenceComponentMatrixPacket,
    violations: &mut Vec<M5TestIntelligenceComponentMatrixViolation>,
) {
    let present: BTreeSet<M5TestIntelligenceComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5TestIntelligenceComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5TestIntelligenceComponentMatrixViolation::RequiredComponentMissing);
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
            violations.push(M5TestIntelligenceComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::MandatoryLabelMissing);
        }
        if row.provenance_classes.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::ProvenanceClassMissing);
        }
        if family.is_coverage_summary_bar() && row.coverage_scope_classes.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::CoverageScopeMissing);
        }
        if family.is_coverage_summary_bar() && row.coverage_metric_kinds.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::CoverageMetricMissing);
        }
        if family.is_coverage_overlay_marker() && row.overlay_states.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::OverlayStateMissing);
        }
        if family.is_coverage_overlay_marker() && row.overlay_emphasis_classes.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::OverlayEmphasisMissing);
        }
        if family.is_flaky_state_badge() && row.flaky_classifications.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::FlakyClassificationMissing);
        }
        if family.is_flaky_state_badge() && row.flaky_confidence_classes.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::FlakyConfidenceMissing);
        }
        if family.is_retry_history_row() && row.retry_attempt_outcomes.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::RetryOutcomeMissing);
        }
        if family.is_retry_history_row() && row.retry_scope_classes.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::RetryScopeMissing);
        }
        if family.is_snapshot_review_card() && row.snapshot_baseline_identities.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::SnapshotBaselineMissing);
        }
        if family.is_snapshot_review_card() && row.snapshot_diff_states.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::SnapshotDiffStateMissing);
        }
        if family.is_coverage_import_merge_sheet() && row.coverage_import_sources.is_empty() {
            violations
                .push(M5TestIntelligenceComponentMatrixViolation::CoverageImportSourceMissing);
        }
        if family.is_coverage_import_merge_sheet() && row.merge_resolution_states.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::MergeResolutionMissing);
        }
        if family.is_test_generation_suggestion_card()
            && row.generated_assumption_classes.is_empty()
        {
            violations.push(M5TestIntelligenceComponentMatrixViolation::GeneratedAssumptionMissing);
        }
        if family.is_test_generation_suggestion_card() && row.generated_apply_scopes.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::GeneratedApplyScopeMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations
                .push(M5TestIntelligenceComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5TestIntelligenceComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5TestIntelligenceComponentMatrixPacket,
    violations: &mut Vec<M5TestIntelligenceComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.coverage_summary_shows_scope_and_metric,
        review.coverage_overlay_shows_state_and_emphasis,
        review.flaky_badge_shows_classification_and_confidence,
        review.retry_history_shows_outcome_and_scope,
        review.snapshot_card_shows_baseline_and_diff,
        review.coverage_import_shows_source_and_merge_resolution,
        review.test_generation_shows_assumptions_and_apply_scope,
        review.no_surface_invents_alternate_state_label,
        review.provenance_vocabulary_named_once,
        review.single_percentage_never_hides_shard_omission,
        review.intermittent_never_labeled_confirmed_flaky,
        review.generated_changes_never_bundled_opaquely,
        review.raw_text_fallback_always_available,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5TestIntelligenceComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TestIntelligenceComponentMatrixPacket,
    violations: &mut Vec<M5TestIntelligenceComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.coverage_surfaces_consume_scope_and_metric_vocabulary,
        projection.overlay_surfaces_consume_freshness_and_provenance_vocabulary,
        projection.flaky_surfaces_consume_classification_and_confidence_vocabulary,
        projection.snapshot_surfaces_consume_baseline_identity_vocabulary,
        projection.generation_surfaces_consume_assumption_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations
                .push(M5TestIntelligenceComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TestIntelligenceComponentMatrixPacket,
    violations: &mut Vec<M5TestIntelligenceComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TestIntelligenceComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5TestIntelligenceComponentMatrixPacket,
    violations: &mut Vec<M5TestIntelligenceComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.test_evidence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5TestIntelligenceComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces
/// a stray comma.
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
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
