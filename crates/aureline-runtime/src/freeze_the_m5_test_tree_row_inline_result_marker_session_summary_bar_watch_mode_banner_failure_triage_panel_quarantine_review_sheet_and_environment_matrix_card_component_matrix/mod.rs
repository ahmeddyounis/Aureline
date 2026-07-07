//! Frozen M5 test-tree-row, inline-result-marker, session-summary-bar,
//! watch-mode-banner, failure-triage-panel, quarantine-review-sheet, and
//! environment-matrix-card component matrix.
//!
//! This module locks Aureline's reusable test-explorer, watch, and triage components
//! into one export-safe packet. Every test-intelligence- and triage-facing subcomponent
//! M5 claims that still drifts too easily by test explorer, editor gutter, status bar,
//! run panel, CI summary, or CLI surface — the test-tree row, the inline result marker,
//! the session-summary bar, the watch-mode banner, the failure-triage panel, the
//! quarantine-review sheet, and the environment-matrix card — is named once here and
//! constrained by the same test identity class, imported-versus-live result origin,
//! result freshness, session and attempt lineage, watch fidelity, failure category and
//! triage disposition, quarantine ownership, release impact, and test target and
//! environment regardless of the surface family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components* themselves:
//! the component families, the test identity classes and result origins the tree row
//! and inline marker bind, the marker verdicts and result freshness, the session
//! outcomes and attempt lineage kinds, the watch fidelity states (`live`, `reduced`,
//! `polling`, `unavailable`) and watch degrade reasons, the failure categories and
//! triage dispositions, the quarantine ownership classes and release impacts, the test
//! target classes and environment lanes, the deployment lines every component must
//! survive, the non-visual accessibility routes, and the mandatory labels every
//! component must be able to show. It does not re-architect test discovery, execution
//! scheduling, or verdict storage that already own those records — it is the shared
//! test-explorer / watch / triage contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 test explorer,
//! editor gutter, status bar, run panel, CI summary, or CLI test surface may publish an
//! identity, origin, freshness, watch, triage, quarantine, or environment claim. Tree,
//! editor, panel, status, quarantine, and export consumers all read this packet so one
//! test-tree row names its identity class and imported/live origin, one inline result
//! marker names its verdict and freshness, one session-summary bar names its outcome and
//! attempt lineage, one watch-mode banner names its fidelity and why it degraded, one
//! failure-triage panel names its failure category and disposition, one quarantine-
//! review sheet names its ownership and release impact, and one environment-matrix card
//! names its target and environment. No M5 lane invents a second testing grammar or an
//! alternate label for stale/imported results, widened rerun scope, or hidden quarantine
//! impact.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5TestExplorerWatchTriageComponentVocabularySet`] rather than minted per surface.
//! Raw log bodies, pasted paths, credentials, and private endpoints stay outside the
//! export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_test_explorer_watch_triage_component_matrix,
    seeded_m5_test_explorer_watch_triage_component_matrix_quarantine_review_sheet_preview_narrowed,
    seeded_m5_test_explorer_watch_triage_component_matrix_watch_mode_banner_beta_narrowed,
    M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5TestExplorerWatchTriageComponentMatrixPacket`].
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix";

/// Schema version for M5 test-explorer / watch / triage component-matrix records.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the test-explorer / watch / triage component boundary schema.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-test-explorer-watch-triage-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_DOC_REF: &str =
    "docs/testing/m5_test_explorer_watch_triage_component_matrix.md";

/// Repo-relative path of the test-item-identity contract this matrix binds against.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_TEST_ITEM_IDENTITY_REF: &str =
    "schemas/testing/test_item_identity.schema.json";

/// Repo-relative path of the test-session contract this matrix binds against.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_TEST_SESSION_REF: &str =
    "schemas/testing/test_session.schema.json";

/// Repo-relative path of the watch-state contract this matrix binds against.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_WATCH_STATE_REF: &str =
    "schemas/testing/watch_state.schema.json";

/// Repo-relative path of the quarantine-record contract this matrix binds against.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_QUARANTINE_RECORD_REF: &str =
    "schemas/testing/test_quarantine_record.schema.json";

/// Repo-relative path of the stability-verdict / release-visibility contract this matrix
/// binds against.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_RELEASE_VISIBILITY_REF: &str =
    "schemas/testing/stability-verdicts-quarantines-and-release-visibility.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-test-explorer-watch-triage-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-test-explorer-watch-triage-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-test-explorer-watch-triage-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-test-explorer-watch-triage-component-matrix.md";

/// One of the seven governed test-explorer / watch / triage component families this
/// matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestExplorerWatchTriageComponentFamily {
    /// A test-tree row carrying a test identity class and imported/live result origin.
    TestTreeRow,
    /// An inline result marker carrying its verdict, freshness, and result origin.
    InlineResultMarker,
    /// A session-summary bar carrying its session outcome and attempt lineage.
    SessionSummaryBar,
    /// A watch-mode banner carrying its watch fidelity and degrade reason.
    WatchModeBanner,
    /// A failure-triage panel carrying its failure category and triage disposition.
    FailureTriagePanel,
    /// A quarantine-review sheet carrying its quarantine ownership and release impact.
    QuarantineReviewSheet,
    /// An environment-matrix card carrying its test target class and environment lane.
    EnvironmentMatrixCard,
}

impl M5TestExplorerWatchTriageComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::TestTreeRow,
        Self::InlineResultMarker,
        Self::SessionSummaryBar,
        Self::WatchModeBanner,
        Self::FailureTriagePanel,
        Self::QuarantineReviewSheet,
        Self::EnvironmentMatrixCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestTreeRow => "test_tree_row",
            Self::InlineResultMarker => "inline_result_marker",
            Self::SessionSummaryBar => "session_summary_bar",
            Self::WatchModeBanner => "watch_mode_banner",
            Self::FailureTriagePanel => "failure_triage_panel",
            Self::QuarantineReviewSheet => "quarantine_review_sheet",
            Self::EnvironmentMatrixCard => "environment_matrix_card",
        }
    }

    /// `true` when this family is a test-tree row and must therefore declare its test
    /// identity classes and result origins.
    pub const fn is_test_tree_row(self) -> bool {
        matches!(self, Self::TestTreeRow)
    }

    /// `true` when this family is an inline result marker and must therefore declare its
    /// marker verdicts, result freshness, and result origins.
    pub const fn is_inline_result_marker(self) -> bool {
        matches!(self, Self::InlineResultMarker)
    }

    /// `true` when this family is a session-summary bar and must therefore declare its
    /// session outcomes and attempt lineage kinds.
    pub const fn is_session_summary_bar(self) -> bool {
        matches!(self, Self::SessionSummaryBar)
    }

    /// `true` when this family is a watch-mode banner and must therefore declare its
    /// watch fidelity states and degrade reasons.
    pub const fn is_watch_mode_banner(self) -> bool {
        matches!(self, Self::WatchModeBanner)
    }

    /// `true` when this family is a failure-triage panel and must therefore declare its
    /// failure categories and triage dispositions.
    pub const fn is_failure_triage_panel(self) -> bool {
        matches!(self, Self::FailureTriagePanel)
    }

    /// `true` when this family is a quarantine-review sheet and must therefore declare
    /// its quarantine ownership classes and release impacts.
    pub const fn is_quarantine_review_sheet(self) -> bool {
        matches!(self, Self::QuarantineReviewSheet)
    }

    /// `true` when this family is an environment-matrix card and must therefore declare
    /// its test target classes and environment lanes.
    pub const fn is_environment_matrix_card(self) -> bool {
        matches!(self, Self::EnvironmentMatrixCard)
    }
}

/// Controlled test identity class — how a test-tree row identifies a test, so a row
/// never leaves identity implicit or invents a parallel identity taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestIdentityClass {
    /// A durable keyed identity that survives file moves and reruns.
    DurableKeyed,
    /// A path-derived identity keyed on file and location.
    PathDerived,
    /// A discovery-only identity not yet run.
    DiscoveredOnly,
    /// An imported identity from an external run.
    ImportedRecord,
    /// A parametrized case identity within a matrix.
    ParametrizedCase,
    /// An ambiguous identity that could not be resolved uniquely.
    AmbiguousIdentity,
}

impl M5TestIdentityClass {
    /// Every identity class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DurableKeyed,
        Self::PathDerived,
        Self::DiscoveredOnly,
        Self::ImportedRecord,
        Self::ParametrizedCase,
        Self::AmbiguousIdentity,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableKeyed => "durable_keyed",
            Self::PathDerived => "path_derived",
            Self::DiscoveredOnly => "discovered_only",
            Self::ImportedRecord => "imported_record",
            Self::ParametrizedCase => "parametrized_case",
            Self::AmbiguousIdentity => "ambiguous_identity",
        }
    }
}

/// Controlled result origin — whether a result is live-local or imported, so a red mark
/// is never left ambiguous about whether it was produced here or elsewhere. Declared by
/// the test-tree row and the inline result marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestResultOrigin {
    /// A live result produced locally on this host.
    LiveLocal,
    /// An imported result from a CI run.
    ImportedCi,
    /// An imported result from a teammate.
    ImportedTeammate,
    /// A replayed result from a snapshot.
    ReplayedSnapshot,
    /// A synthetic seed result.
    SyntheticSeed,
    /// An unknown origin that could not be attributed.
    UnknownOrigin,
}

impl M5TestResultOrigin {
    /// Every result origin, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiveLocal,
        Self::ImportedCi,
        Self::ImportedTeammate,
        Self::ReplayedSnapshot,
        Self::SyntheticSeed,
        Self::UnknownOrigin,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveLocal => "live_local",
            Self::ImportedCi => "imported_ci",
            Self::ImportedTeammate => "imported_teammate",
            Self::ReplayedSnapshot => "replayed_snapshot",
            Self::SyntheticSeed => "synthetic_seed",
            Self::UnknownOrigin => "unknown_origin",
        }
    }
}

/// Controlled inline marker verdict — what an inline result marker asserts, so a marker
/// never leaves the verdict implicit or collapses distinct verdicts into one glyph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InlineMarkerVerdict {
    /// The test passed.
    Passed,
    /// The test failed an assertion.
    Failed,
    /// The test errored before asserting.
    Errored,
    /// The test was skipped.
    Skipped,
    /// The test is suspected flaky.
    FlakySuspected,
    /// The test has not been run.
    NotRun,
}

impl M5InlineMarkerVerdict {
    /// Every marker verdict, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Passed,
        Self::Failed,
        Self::Errored,
        Self::Skipped,
        Self::FlakySuspected,
        Self::NotRun,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Errored => "errored",
            Self::Skipped => "skipped",
            Self::FlakySuspected => "flaky_suspected",
            Self::NotRun => "not_run",
        }
    }
}

/// Controlled result freshness — how current a result is, so a marker never shows a
/// stale or imported result as if it were freshly produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestResultFreshness {
    /// The result is fresh for the current source.
    Fresh,
    /// The result is stale relative to the current source.
    Stale,
    /// The source changed after the result was produced.
    OutdatedSource,
    /// The test has never been run.
    NeverRun,
    /// The test is currently running.
    Running,
    /// The result has expired past its retention window.
    Expired,
}

impl M5TestResultFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Fresh,
        Self::Stale,
        Self::OutdatedSource,
        Self::NeverRun,
        Self::Running,
        Self::Expired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::OutdatedSource => "outdated_source",
            Self::NeverRun => "never_run",
            Self::Running => "running",
            Self::Expired => "expired",
        }
    }
}

/// Controlled session outcome — the overall result of a test session, so a summary bar
/// never leaves the outcome implicit or hides a partial discovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestSessionOutcome {
    /// Every selected test passed.
    AllPassed,
    /// Some selected tests failed.
    SomeFailed,
    /// The run errored before completing.
    ErroredRun,
    /// Discovery was partial and some tests were not enumerated.
    PartialDiscovery,
    /// The session was cancelled.
    Cancelled,
    /// The session is still in progress.
    InProgress,
}

impl M5TestSessionOutcome {
    /// Every session outcome, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AllPassed,
        Self::SomeFailed,
        Self::ErroredRun,
        Self::PartialDiscovery,
        Self::Cancelled,
        Self::InProgress,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllPassed => "all_passed",
            Self::SomeFailed => "some_failed",
            Self::ErroredRun => "errored_run",
            Self::PartialDiscovery => "partial_discovery",
            Self::Cancelled => "cancelled",
            Self::InProgress => "in_progress",
        }
    }
}

/// Controlled attempt lineage kind — how the current attempt relates to prior attempts,
/// so retry lineage and rerun scope are never left implicit or silently widened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AttemptLineageKind {
    /// The first attempt for this selection.
    FirstAttempt,
    /// A retry that passed.
    RetriedPass,
    /// A retry that failed again.
    RetriedFail,
    /// A rerun of the same selection.
    RerunSelected,
    /// A rerun narrowed to the failed-only subset.
    RerunFailedOnly,
    /// A replayed imported attempt.
    ReplayedImport,
}

impl M5AttemptLineageKind {
    /// Every attempt lineage kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FirstAttempt,
        Self::RetriedPass,
        Self::RetriedFail,
        Self::RerunSelected,
        Self::RerunFailedOnly,
        Self::ReplayedImport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstAttempt => "first_attempt",
            Self::RetriedPass => "retried_pass",
            Self::RetriedFail => "retried_fail",
            Self::RerunSelected => "rerun_selected",
            Self::RerunFailedOnly => "rerun_failed_only",
            Self::ReplayedImport => "replayed_import",
        }
    }
}

/// Controlled watch fidelity state — how faithfully watch mode is observing, bound to
/// the frozen `live` / `reduced` / `polling` / `unavailable` vocabulary so no surface
/// invents an alternate label for a degraded watch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WatchFidelityState {
    /// Live watch with full fidelity.
    Live,
    /// Reduced fidelity watch.
    Reduced,
    /// Polling-based watch.
    Polling,
    /// Watch is unavailable.
    Unavailable,
    /// Watch is paused by the user.
    Paused,
    /// Watch is reconnecting.
    Reconnecting,
}

impl M5WatchFidelityState {
    /// Every watch fidelity state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Live,
        Self::Reduced,
        Self::Polling,
        Self::Unavailable,
        Self::Paused,
        Self::Reconnecting,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Reduced => "reduced",
            Self::Polling => "polling",
            Self::Unavailable => "unavailable",
            Self::Paused => "paused",
            Self::Reconnecting => "reconnecting",
        }
    }
}

/// Controlled watch degrade reason — why watch fidelity dropped, so a banner never hides
/// why watch degraded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WatchDegradeReason {
    /// Host resource pressure.
    ResourcePressure,
    /// Adapter limitation.
    AdapterLimited,
    /// The host went offline.
    OfflineHost,
    /// The file watch handle was lost.
    FileWatchLost,
    /// Too many files to watch efficiently.
    TooManyFiles,
    /// Watch is limited by policy.
    PolicyLimited,
}

impl M5WatchDegradeReason {
    /// Every watch degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ResourcePressure,
        Self::AdapterLimited,
        Self::OfflineHost,
        Self::FileWatchLost,
        Self::TooManyFiles,
        Self::PolicyLimited,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResourcePressure => "resource_pressure",
            Self::AdapterLimited => "adapter_limited",
            Self::OfflineHost => "offline_host",
            Self::FileWatchLost => "file_watch_lost",
            Self::TooManyFiles => "too_many_files",
            Self::PolicyLimited => "policy_limited",
        }
    }
}

/// Controlled failure category — what class of failure a triage panel names, so a
/// failure is never left uncategorized or mislabelled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FailureCategory {
    /// An assertion failure.
    AssertionFailure,
    /// A runtime error.
    RuntimeError,
    /// A timeout.
    Timeout,
    /// An environment error.
    EnvironmentError,
    /// A failure under flaky review.
    FlakyUnderReview,
    /// An unknown failure.
    UnknownFailure,
}

impl M5FailureCategory {
    /// Every failure category, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AssertionFailure,
        Self::RuntimeError,
        Self::Timeout,
        Self::EnvironmentError,
        Self::FlakyUnderReview,
        Self::UnknownFailure,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssertionFailure => "assertion_failure",
            Self::RuntimeError => "runtime_error",
            Self::Timeout => "timeout",
            Self::EnvironmentError => "environment_error",
            Self::FlakyUnderReview => "flaky_under_review",
            Self::UnknownFailure => "unknown_failure",
        }
    }
}

/// Controlled triage disposition — where a failure sits in triage, so a disposition is
/// always explicit and never left as a dead end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TriageDisposition {
    /// The failure needs investigation.
    NeedsInvestigation,
    /// The failure is a known flaky.
    KnownFlaky,
    /// The failure is an environment issue.
    EnvironmentIssue,
    /// The failure is a product bug.
    ProductBug,
    /// The failure is a test bug.
    TestBug,
    /// The failure has been resolved in triage.
    ResolvedTriage,
}

impl M5TriageDisposition {
    /// Every triage disposition, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NeedsInvestigation,
        Self::KnownFlaky,
        Self::EnvironmentIssue,
        Self::ProductBug,
        Self::TestBug,
        Self::ResolvedTriage,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NeedsInvestigation => "needs_investigation",
            Self::KnownFlaky => "known_flaky",
            Self::EnvironmentIssue => "environment_issue",
            Self::ProductBug => "product_bug",
            Self::TestBug => "test_bug",
            Self::ResolvedTriage => "resolved_triage",
        }
    }
}

/// Controlled quarantine ownership — who owns a mute / quarantine, so ownership is never
/// left implicit and an unowned quarantine is never mistaken for a governed one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QuarantineOwnership {
    /// No owner is assigned.
    Unowned,
    /// Owned by the current user.
    SelfOwned,
    /// Owned by the team.
    TeamOwned,
    /// Enforced by CI.
    CiEnforced,
    /// Imported from a policy.
    ImportedPolicy,
    /// The owner assignment has expired.
    OwnerExpired,
}

impl M5QuarantineOwnership {
    /// Every quarantine ownership class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Unowned,
        Self::SelfOwned,
        Self::TeamOwned,
        Self::CiEnforced,
        Self::ImportedPolicy,
        Self::OwnerExpired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unowned => "unowned",
            Self::SelfOwned => "self_owned",
            Self::TeamOwned => "team_owned",
            Self::CiEnforced => "ci_enforced",
            Self::ImportedPolicy => "imported_policy",
            Self::OwnerExpired => "owner_expired",
        }
    }
}

/// Controlled release impact — what a mute / quarantine hides from release and support
/// surfaces, so hidden quarantine impact is never masked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestReleaseImpact {
    /// The quarantined test still blocks release.
    BlocksRelease,
    /// The quarantined test is hidden from release gating.
    HiddenFromRelease,
    /// The quarantined test soft-gates release with a warning.
    SoftGated,
    /// The quarantine is informational only.
    Informational,
    /// The quarantine has no release impact.
    NoImpact,
    /// The release impact is unknown.
    UnknownImpact,
}

impl M5TestReleaseImpact {
    /// Every release impact, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BlocksRelease,
        Self::HiddenFromRelease,
        Self::SoftGated,
        Self::Informational,
        Self::NoImpact,
        Self::UnknownImpact,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlocksRelease => "blocks_release",
            Self::HiddenFromRelease => "hidden_from_release",
            Self::SoftGated => "soft_gated",
            Self::Informational => "informational",
            Self::NoImpact => "no_impact",
            Self::UnknownImpact => "unknown_impact",
        }
    }
}

/// Controlled test target class — what kind of test an environment-matrix card names, so
/// the target class is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestTargetClass {
    /// A unit test.
    UnitTest,
    /// An integration test.
    IntegrationTest,
    /// An end-to-end test.
    EndToEndTest,
    /// A UI snapshot test.
    UiSnapshotTest,
    /// A benchmark test.
    BenchmarkTest,
    /// A contract test.
    ContractTest,
}

impl M5TestTargetClass {
    /// Every test target class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UnitTest,
        Self::IntegrationTest,
        Self::EndToEndTest,
        Self::UiSnapshotTest,
        Self::BenchmarkTest,
        Self::ContractTest,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnitTest => "unit_test",
            Self::IntegrationTest => "integration_test",
            Self::EndToEndTest => "end_to_end_test",
            Self::UiSnapshotTest => "ui_snapshot_test",
            Self::BenchmarkTest => "benchmark_test",
            Self::ContractTest => "contract_test",
        }
    }
}

/// Controlled test environment lane — where a test runs, so the environment is never
/// left implicit and a local result is never confused with a remote or CI-matrix result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestEnvironmentLane {
    /// The local host.
    LocalHost,
    /// A container.
    Container,
    /// A remote runner.
    RemoteRunner,
    /// A CI matrix leg.
    CiMatrix,
    /// A browser matrix leg.
    BrowserMatrix,
    /// An emulated device.
    EmulatedDevice,
}

impl M5TestEnvironmentLane {
    /// Every environment lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalHost,
        Self::Container,
        Self::RemoteRunner,
        Self::CiMatrix,
        Self::BrowserMatrix,
        Self::EmulatedDevice,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHost => "local_host",
            Self::Container => "container",
            Self::RemoteRunner => "remote_runner",
            Self::CiMatrix => "ci_matrix",
            Self::BrowserMatrix => "browser_matrix",
            Self::EmulatedDevice => "emulated_device",
        }
    }
}

/// Claimed M5 test surface family that renders / consumes a test-explorer / watch /
/// triage component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestSurfaceFamily {
    /// The test-explorer surface.
    TestExplorer,
    /// The editor-gutter surface.
    EditorGutter,
    /// The status-bar surface.
    StatusBar,
    /// The run-panel surface.
    RunPanel,
    /// The CI-summary surface.
    CiSummary,
    /// The CLI test surface.
    CliTest,
}

impl M5TestSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TestExplorer,
        Self::EditorGutter,
        Self::StatusBar,
        Self::RunPanel,
        Self::CiSummary,
        Self::CliTest,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestExplorer => "test_explorer",
            Self::EditorGutter => "editor_gutter",
            Self::StatusBar => "status_bar",
            Self::RunPanel => "run_panel",
            Self::CiSummary => "ci_summary",
            Self::CliTest => "cli_test",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's
/// identity, origin, freshness, watch, or quarantine truth never silently narrows or
/// widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestDeploymentLine {
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

impl M5TestDeploymentLine {
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

/// Test / triage subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestConsumerSurface {
    /// The test-tree UI.
    TestTreeUi,
    /// The editor-gutter UI.
    EditorGutterUi,
    /// The session-summary UI.
    SessionSummaryUi,
    /// The watch-banner UI.
    WatchBannerUi,
    /// The triage-panel UI.
    TriagePanelUi,
    /// The quarantine-sheet UI.
    QuarantineSheetUi,
    /// The support export.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The general product UI.
    ProductUi,
}

impl M5TestConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TestTreeUi,
        Self::EditorGutterUi,
        Self::SessionSummaryUi,
        Self::WatchBannerUi,
        Self::TriagePanelUi,
        Self::QuarantineSheetUi,
        Self::SupportExport,
        Self::CliInspect,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestTreeUi => "test_tree_ui",
            Self::EditorGutterUi => "editor_gutter_ui",
            Self::SessionSummaryUi => "session_summary_ui",
            Self::WatchBannerUi => "watch_banner_ui",
            Self::TriagePanelUi => "triage_panel_ui",
            Self::QuarantineSheetUi => "quarantine_sheet_ui",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no test truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestAccessibilityRoute {
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

impl M5TestAccessibilityRoute {
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

/// Mandatory label a claimed test-explorer / watch / triage component must be able to
/// show. The first three are hard requirements on every component; the remaining three
/// close the acceptance-criteria ambiguity about origin/freshness, watch fidelity, and
/// quarantine / release impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestRequiredLabel {
    /// The component's stable identity / what test object it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The imported/live origin and freshness behind the component.
    OriginAndFreshness,
    /// The watch fidelity behind the component.
    WatchFidelity,
    /// The quarantine ownership and release impact behind the component.
    QuarantineAndReleaseImpact,
}

impl M5TestRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::OriginAndFreshness,
        Self::WatchFidelity,
        Self::QuarantineAndReleaseImpact,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::OriginAndFreshness => "origin_and_freshness",
            Self::WatchFidelity => "watch_fidelity",
            Self::QuarantineAndReleaseImpact => "quarantine_and_release_impact",
        }
    }
}

/// Qualification class for an M5 test-explorer / watch / triage component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestQualificationClass {
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

impl M5TestQualificationClass {
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

/// Downgrade trigger that narrows a test-explorer / watch / triage component below its
/// claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestDowngradeTrigger {
    /// A tree row left its identity class unstated.
    IdentityClassUnstated,
    /// A component left its imported/live result origin unstated.
    ResultOriginUnstated,
    /// A marker left its result freshness undisclosed.
    ResultFreshnessUndisclosed,
    /// A watch banner left its fidelity unstated.
    WatchFidelityUnstated,
    /// A watch banner hid why watch degraded.
    WatchDegradeReasonHidden,
    /// A rerun widened its scope without disclosure.
    RerunScopeWidened,
    /// A summary bar left the attempt lineage unstated.
    AttemptLineageUnstated,
    /// A quarantine sheet left its ownership unstated.
    QuarantineOwnershipUnstated,
    /// A quarantine sheet hid its release impact.
    QuarantineReleaseImpactHidden,
    /// An environment card left its target or environment unstated.
    EnvironmentOrTargetUnstated,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5TestDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::IdentityClassUnstated,
        Self::ResultOriginUnstated,
        Self::ResultFreshnessUndisclosed,
        Self::WatchFidelityUnstated,
        Self::WatchDegradeReasonHidden,
        Self::RerunScopeWidened,
        Self::AttemptLineageUnstated,
        Self::QuarantineOwnershipUnstated,
        Self::QuarantineReleaseImpactHidden,
        Self::EnvironmentOrTargetUnstated,
        Self::AlternateStateLabelInvented,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityClassUnstated => "identity_class_unstated",
            Self::ResultOriginUnstated => "result_origin_unstated",
            Self::ResultFreshnessUndisclosed => "result_freshness_undisclosed",
            Self::WatchFidelityUnstated => "watch_fidelity_unstated",
            Self::WatchDegradeReasonHidden => "watch_degrade_reason_hidden",
            Self::RerunScopeWidened => "rerun_scope_widened",
            Self::AttemptLineageUnstated => "attempt_lineage_unstated",
            Self::QuarantineOwnershipUnstated => "quarantine_ownership_unstated",
            Self::QuarantineReleaseImpactHidden => "quarantine_release_impact_hidden",
            Self::EnvironmentOrTargetUnstated => "environment_or_target_unstated",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed test-explorer / watch / triage component family
/// bound to the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestExplorerWatchTriageComponentRow {
    /// Governed component family.
    pub component_family: M5TestExplorerWatchTriageComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5TestQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 test surface families that render / consume this component.
    pub surface_families: Vec<M5TestSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5TestDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5TestRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5TestRequiredLabel>,
    /// Test identity classes this component names (test-tree-row only).
    pub test_identity_classes: Vec<M5TestIdentityClass>,
    /// Result origins this component distinguishes (test-tree-row and
    /// inline-result-marker).
    pub result_origins: Vec<M5TestResultOrigin>,
    /// Marker verdicts this component names (inline-result-marker only).
    pub marker_verdicts: Vec<M5InlineMarkerVerdict>,
    /// Result freshness states this component discloses (inline-result-marker only).
    pub result_freshness: Vec<M5TestResultFreshness>,
    /// Session outcomes this component names (session-summary-bar only).
    pub session_outcomes: Vec<M5TestSessionOutcome>,
    /// Attempt lineage kinds this component names (session-summary-bar only).
    pub attempt_lineage_kinds: Vec<M5AttemptLineageKind>,
    /// Watch fidelity states this component distinguishes (watch-mode-banner only).
    pub watch_fidelity_states: Vec<M5WatchFidelityState>,
    /// Watch degrade reasons this component discloses (watch-mode-banner only).
    pub watch_degrade_reasons: Vec<M5WatchDegradeReason>,
    /// Failure categories this component names (failure-triage-panel only).
    pub failure_categories: Vec<M5FailureCategory>,
    /// Triage dispositions this component names (failure-triage-panel only).
    pub triage_dispositions: Vec<M5TriageDisposition>,
    /// Quarantine ownership classes this component names (quarantine-review-sheet only).
    pub quarantine_ownership_classes: Vec<M5QuarantineOwnership>,
    /// Release impacts this component discloses (quarantine-review-sheet only).
    pub release_impacts: Vec<M5TestReleaseImpact>,
    /// Test target classes this component names (environment-matrix-card only).
    pub target_classes: Vec<M5TestTargetClass>,
    /// Environment lanes this component names (environment-matrix-card only).
    pub environment_lanes: Vec<M5TestEnvironmentLane>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5TestAccessibilityRoute>,
    /// Test / triage subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5TestConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5TestDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its test identity class or imported/
    /// live result origin. MUST be `false`.
    pub masks_identity_or_origin: bool,
    /// Hard invariant: this component never hides a quarantine's release impact. MUST be
    /// `false`.
    pub hides_quarantine_release_impact: bool,
    /// Hard invariant: this component never invents an alternate label for a governed
    /// state. MUST be `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: this component never silently widens rerun scope. MUST be
    /// `false`.
    pub widens_rerun_scope_silently: bool,
}

impl M5TestExplorerWatchTriageComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5TestRequiredLabel> = self.required_labels.iter().copied().collect();
        M5TestRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_identity_or_origin
            && !self.hides_quarantine_release_impact
            && !self.invents_alternate_state_label
            && !self.widens_rerun_scope_silently
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestExplorerWatchTriageComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Test-identity-class tokens.
    pub test_identity_classes: Vec<String>,
    /// Result-origin tokens.
    pub result_origins: Vec<String>,
    /// Marker-verdict tokens.
    pub marker_verdicts: Vec<String>,
    /// Result-freshness tokens.
    pub result_freshness: Vec<String>,
    /// Session-outcome tokens.
    pub session_outcomes: Vec<String>,
    /// Attempt-lineage-kind tokens.
    pub attempt_lineage_kinds: Vec<String>,
    /// Watch-fidelity-state tokens.
    pub watch_fidelity_states: Vec<String>,
    /// Watch-degrade-reason tokens.
    pub watch_degrade_reasons: Vec<String>,
    /// Failure-category tokens.
    pub failure_categories: Vec<String>,
    /// Triage-disposition tokens.
    pub triage_dispositions: Vec<String>,
    /// Quarantine-ownership tokens.
    pub quarantine_ownership_classes: Vec<String>,
    /// Release-impact tokens.
    pub release_impacts: Vec<String>,
    /// Test-target-class tokens.
    pub target_classes: Vec<String>,
    /// Environment-lane tokens.
    pub environment_lanes: Vec<String>,
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

impl M5TestExplorerWatchTriageComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5TestExplorerWatchTriageComponentFamily::ALL, |v| {
                v.as_str()
            }),
            test_identity_classes: tokens(&M5TestIdentityClass::ALL, |v| v.as_str()),
            result_origins: tokens(&M5TestResultOrigin::ALL, |v| v.as_str()),
            marker_verdicts: tokens(&M5InlineMarkerVerdict::ALL, |v| v.as_str()),
            result_freshness: tokens(&M5TestResultFreshness::ALL, |v| v.as_str()),
            session_outcomes: tokens(&M5TestSessionOutcome::ALL, |v| v.as_str()),
            attempt_lineage_kinds: tokens(&M5AttemptLineageKind::ALL, |v| v.as_str()),
            watch_fidelity_states: tokens(&M5WatchFidelityState::ALL, |v| v.as_str()),
            watch_degrade_reasons: tokens(&M5WatchDegradeReason::ALL, |v| v.as_str()),
            failure_categories: tokens(&M5FailureCategory::ALL, |v| v.as_str()),
            triage_dispositions: tokens(&M5TriageDisposition::ALL, |v| v.as_str()),
            quarantine_ownership_classes: tokens(&M5QuarantineOwnership::ALL, |v| v.as_str()),
            release_impacts: tokens(&M5TestReleaseImpact::ALL, |v| v.as_str()),
            target_classes: tokens(&M5TestTargetClass::ALL, |v| v.as_str()),
            environment_lanes: tokens(&M5TestEnvironmentLane::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TestSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TestDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5TestConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TestAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5TestRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5TestExplorerWatchTriageComponentGovernanceReview {
    /// The test-tree row shows its identity class and imported/live origin.
    pub test_tree_row_shows_identity_and_origin: bool,
    /// The inline result marker shows its verdict and freshness.
    pub inline_marker_shows_verdict_and_freshness: bool,
    /// The session-summary bar shows its outcome and attempt lineage.
    pub session_summary_shows_outcome_and_attempt_lineage: bool,
    /// The watch-mode banner shows its fidelity and degrade reason.
    pub watch_banner_shows_fidelity_and_degrade_reason: bool,
    /// The failure-triage panel shows its category and disposition.
    pub failure_triage_shows_category_and_disposition: bool,
    /// The quarantine-review sheet shows its ownership and release impact.
    pub quarantine_sheet_shows_ownership_and_release_impact: bool,
    /// The environment-matrix card shows its target and environment.
    pub environment_card_shows_target_and_environment: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// `live`, `reduced`, `polling`, and `unavailable` are named once.
    pub live_reduced_polling_unavailable_named_once: bool,
    /// The imported-versus-live origin is always explicit.
    pub imported_versus_live_always_explicit: bool,
    /// The rerun scope is always explicit and never silently widened.
    pub rerun_scope_always_explicit: bool,
    /// The quarantine release impact is always explicit.
    pub quarantine_release_impact_always_explicit: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel testing vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestExplorerWatchTriageComponentConsumerProjection {
    /// Tree and editor surfaces consume the shared identity vocabulary.
    pub tree_and_editor_surfaces_consume_identity_vocabulary: bool,
    /// Marker surfaces consume the freshness and origin vocabulary.
    pub marker_surfaces_consume_freshness_and_origin_vocabulary: bool,
    /// Watch surfaces consume the fidelity vocabulary.
    pub watch_surfaces_consume_fidelity_vocabulary: bool,
    /// Triage surfaces consume the failure-category vocabulary.
    pub triage_surfaces_consume_failure_category_vocabulary: bool,
    /// Quarantine surfaces consume the ownership and release-impact vocabulary.
    pub quarantine_surfaces_consume_ownership_and_release_impact_vocabulary: bool,
    /// Support / export reads a single canonical test source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestExplorerWatchTriageComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the test-explorer / watch / triage component
/// lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestExplorerWatchTriageComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting test-evidence audit for the lane.
    pub test_evidence_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5TestExplorerWatchTriageComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TestExplorerWatchTriageComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5TestExplorerWatchTriageComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TestExplorerWatchTriageComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TestExplorerWatchTriageComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TestExplorerWatchTriageComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TestExplorerWatchTriageComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TestExplorerWatchTriageComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 test-explorer / watch / triage component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestExplorerWatchTriageComponentMatrixPacket {
    /// Record kind; must equal
    /// [`M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5TestExplorerWatchTriageComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TestExplorerWatchTriageComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TestExplorerWatchTriageComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TestExplorerWatchTriageComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TestExplorerWatchTriageComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TestExplorerWatchTriageComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TestExplorerWatchTriageComponentMatrixPacket {
    /// Builds an M5 test-explorer / watch / triage component matrix packet from
    /// stable-lane input.
    pub fn new(input: M5TestExplorerWatchTriageComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_SCHEMA_VERSION,
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

    /// Validates the M5 test-explorer / watch / triage component matrix invariants.
    pub fn validate(&self) -> Vec<M5TestExplorerWatchTriageComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5TestExplorerWatchTriageComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5TestExplorerWatchTriageComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TestExplorerWatchTriageComponentMatrixViolation::MissingIdentity);
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
                .expect("m5 test-explorer watch triage component matrix packet serializes"),
        ) {
            violations.push(M5TestExplorerWatchTriageComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 test-explorer watch triage component matrix packet serializes")
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
            "# M5 Test-Tree-Row, Inline-Result-Marker, Session-Summary-Bar, Watch-Mode-Banner, Failure-Triage-Panel, Quarantine-Review-Sheet, and Environment-Matrix-Card Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Watch fidelity states: {}\n",
            self.vocabulary_set.watch_fidelity_states.join(", ")
        ));
        out.push_str(&format!(
            "- Result origins: {}\n",
            self.vocabulary_set.result_origins.join(", ")
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

/// Errors emitted when reading the checked-in M5 test-explorer matrix export.
#[derive(Debug)]
pub enum M5TestExplorerWatchTriageComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TestExplorerWatchTriageComponentMatrixViolation>),
}

impl fmt::Display for M5TestExplorerWatchTriageComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 test-explorer watch triage component matrix export parse failed: {error}"
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
                    "m5 test-explorer watch triage component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TestExplorerWatchTriageComponentMatrixArtifactError {}

/// Validation failures emitted by
/// [`M5TestExplorerWatchTriageComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TestExplorerWatchTriageComponentMatrixViolation {
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
    /// A test-tree-row component declares no test identity classes.
    TestIdentityClassMissing,
    /// A test-tree-row or inline-result-marker component declares no result origins.
    ResultOriginMissing,
    /// An inline-result-marker component declares no marker verdicts.
    MarkerVerdictMissing,
    /// An inline-result-marker component declares no result freshness states.
    ResultFreshnessMissing,
    /// A session-summary-bar component declares no session outcomes.
    SessionOutcomeMissing,
    /// A session-summary-bar component declares no attempt lineage kinds.
    AttemptLineageMissing,
    /// A watch-mode-banner component declares no watch fidelity states.
    WatchFidelityStateMissing,
    /// A watch-mode-banner component declares no watch degrade reasons.
    WatchDegradeReasonMissing,
    /// A failure-triage-panel component declares no failure categories.
    FailureCategoryMissing,
    /// A failure-triage-panel component declares no triage dispositions.
    TriageDispositionMissing,
    /// A quarantine-review-sheet component declares no quarantine ownership classes.
    QuarantineOwnershipMissing,
    /// A quarantine-review-sheet component declares no release impacts.
    ReleaseImpactMissing,
    /// An environment-matrix-card component declares no test target classes.
    TargetClassMissing,
    /// An environment-matrix-card component declares no environment lanes.
    EnvironmentLaneMissing,
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
    /// A component violates a hard invariant (masked identity/origin, hidden quarantine
    /// release impact, invented alternate state label, or silently widened rerun scope).
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

impl M5TestExplorerWatchTriageComponentMatrixViolation {
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
            Self::TestIdentityClassMissing => "test_identity_class_missing",
            Self::ResultOriginMissing => "result_origin_missing",
            Self::MarkerVerdictMissing => "marker_verdict_missing",
            Self::ResultFreshnessMissing => "result_freshness_missing",
            Self::SessionOutcomeMissing => "session_outcome_missing",
            Self::AttemptLineageMissing => "attempt_lineage_missing",
            Self::WatchFidelityStateMissing => "watch_fidelity_state_missing",
            Self::WatchDegradeReasonMissing => "watch_degrade_reason_missing",
            Self::FailureCategoryMissing => "failure_category_missing",
            Self::TriageDispositionMissing => "triage_disposition_missing",
            Self::QuarantineOwnershipMissing => "quarantine_ownership_missing",
            Self::ReleaseImpactMissing => "release_impact_missing",
            Self::TargetClassMissing => "target_class_missing",
            Self::EnvironmentLaneMissing => "environment_lane_missing",
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

/// Reads and validates the checked-in stable M5 test-explorer matrix export.
pub fn current_stable_m5_test_explorer_watch_triage_component_matrix_export() -> Result<
    M5TestExplorerWatchTriageComponentMatrixPacket,
    M5TestExplorerWatchTriageComponentMatrixArtifactError,
> {
    let packet: M5TestExplorerWatchTriageComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-test-explorer-watch-triage-proof/support_export.json"
        )))
        .map_err(M5TestExplorerWatchTriageComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TestExplorerWatchTriageComponentMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5TestExplorerWatchTriageComponentMatrixPacket,
    violations: &mut Vec<M5TestExplorerWatchTriageComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_DOC_REF,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_TEST_ITEM_IDENTITY_REF,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_TEST_SESSION_REF,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_WATCH_STATE_REF,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_QUARANTINE_RECORD_REF,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_RELEASE_VISIBILITY_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5TestExplorerWatchTriageComponentMatrixPacket,
    violations: &mut Vec<M5TestExplorerWatchTriageComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5TestExplorerWatchTriageComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5TestExplorerWatchTriageComponentMatrixPacket,
    violations: &mut Vec<M5TestExplorerWatchTriageComponentMatrixViolation>,
) {
    let present: BTreeSet<M5TestExplorerWatchTriageComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5TestExplorerWatchTriageComponentFamily::ALL {
        if !present.contains(&required) {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::RequiredComponentMissing);
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
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::MandatoryLabelMissing);
        }
        if family.is_test_tree_row() && row.test_identity_classes.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::TestIdentityClassMissing);
        }
        // Result origin is shared by the test-tree row and the inline result marker.
        if (family.is_test_tree_row() || family.is_inline_result_marker())
            && row.result_origins.is_empty()
        {
            violations.push(M5TestExplorerWatchTriageComponentMatrixViolation::ResultOriginMissing);
        }
        if family.is_inline_result_marker() && row.marker_verdicts.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::MarkerVerdictMissing);
        }
        if family.is_inline_result_marker() && row.result_freshness.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::ResultFreshnessMissing);
        }
        if family.is_session_summary_bar() && row.session_outcomes.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::SessionOutcomeMissing);
        }
        if family.is_session_summary_bar() && row.attempt_lineage_kinds.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::AttemptLineageMissing);
        }
        if family.is_watch_mode_banner() && row.watch_fidelity_states.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::WatchFidelityStateMissing);
        }
        if family.is_watch_mode_banner() && row.watch_degrade_reasons.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::WatchDegradeReasonMissing);
        }
        if family.is_failure_triage_panel() && row.failure_categories.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::FailureCategoryMissing);
        }
        if family.is_failure_triage_panel() && row.triage_dispositions.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::TriageDispositionMissing);
        }
        if family.is_quarantine_review_sheet() && row.quarantine_ownership_classes.is_empty() {
            violations.push(
                M5TestExplorerWatchTriageComponentMatrixViolation::QuarantineOwnershipMissing,
            );
        }
        if family.is_quarantine_review_sheet() && row.release_impacts.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::ReleaseImpactMissing);
        }
        if family.is_environment_matrix_card() && row.target_classes.is_empty() {
            violations.push(M5TestExplorerWatchTriageComponentMatrixViolation::TargetClassMissing);
        }
        if family.is_environment_matrix_card() && row.environment_lanes.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::EnvironmentLaneMissing);
        }
        if row.surface_families.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations
                .push(M5TestExplorerWatchTriageComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(
                M5TestExplorerWatchTriageComponentMatrixViolation::StableComponentMissingProof,
            );
        }
        if !row.honours_invariants() {
            violations.push(
                M5TestExplorerWatchTriageComponentMatrixViolation::ComponentInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5TestExplorerWatchTriageComponentMatrixPacket,
    violations: &mut Vec<M5TestExplorerWatchTriageComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.test_tree_row_shows_identity_and_origin,
        review.inline_marker_shows_verdict_and_freshness,
        review.session_summary_shows_outcome_and_attempt_lineage,
        review.watch_banner_shows_fidelity_and_degrade_reason,
        review.failure_triage_shows_category_and_disposition,
        review.quarantine_sheet_shows_ownership_and_release_impact,
        review.environment_card_shows_target_and_environment,
        review.no_surface_invents_alternate_state_label,
        review.live_reduced_polling_unavailable_named_once,
        review.imported_versus_live_always_explicit,
        review.rerun_scope_always_explicit,
        review.quarantine_release_impact_always_explicit,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5TestExplorerWatchTriageComponentMatrixViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TestExplorerWatchTriageComponentMatrixPacket,
    violations: &mut Vec<M5TestExplorerWatchTriageComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.tree_and_editor_surfaces_consume_identity_vocabulary,
        projection.marker_surfaces_consume_freshness_and_origin_vocabulary,
        projection.watch_surfaces_consume_fidelity_vocabulary,
        projection.triage_surfaces_consume_failure_category_vocabulary,
        projection.quarantine_surfaces_consume_ownership_and_release_impact_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(
                M5TestExplorerWatchTriageComponentMatrixViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TestExplorerWatchTriageComponentMatrixPacket,
    violations: &mut Vec<M5TestExplorerWatchTriageComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations
            .push(M5TestExplorerWatchTriageComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5TestExplorerWatchTriageComponentMatrixPacket,
    violations: &mut Vec<M5TestExplorerWatchTriageComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.test_evidence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations
            .push(M5TestExplorerWatchTriageComponentMatrixViolation::ReleasePostureIncomplete);
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
