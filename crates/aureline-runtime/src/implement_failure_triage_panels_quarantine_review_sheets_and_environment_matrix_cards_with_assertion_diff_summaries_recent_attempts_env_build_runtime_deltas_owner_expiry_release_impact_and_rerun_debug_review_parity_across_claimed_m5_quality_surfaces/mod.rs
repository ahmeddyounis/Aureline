//! Three reusable M5 triage-and-suppression primitives — the failure-triage panel, the
//! quarantine/mute review sheet, and the environment-matrix card — so a red test row stops
//! leading straight to destructive suppression without evidence context, a quarantined or
//! muted test stops disappearing into a hidden filtered state, and an environment card stops
//! implying safe equivalence across environments that are not actually compatible.
//!
//! A failure-triage panel always names its failure category and triage disposition, its
//! classifier confidence, its result origin, its recent attempt sequence, and whether it
//! carries an assertion/diff summary and environment/build/runtime deltas — and it only
//! escalates to the quarantine-review sheet once that evidence context is present, so the
//! user reviews evidence before suppressing. A quarantine/mute review sheet always preserves
//! its suppression scope, kind, reason, owner, expiry, linked artifacts, and release-impact
//! note, keeps the suppressed test visible rather than hiding it behind a filter, and always
//! offers the restore action. An environment-matrix card always compares its target, runtime,
//! toolchain, and build compatibility classes across every leg and never asserts safe
//! equivalence when any axis is incompatible or unverified.
//!
//! Aureline's frozen test-explorer / watch / triage component matrix
//! ([`crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix`])
//! names the failure-triage panel, the quarantine-review sheet, and the environment-matrix
//! card as three governed component families and freezes their controlled vocabulary — the
//! failure categories, the triage dispositions, the quarantine ownership classes, the release
//! impacts, the test target classes, and the environment lanes, plus the surface families,
//! the deployment lines, the consumer surfaces, the accessibility routes, the qualification
//! classes, and the downgrade triggers. This module *implements* that contract as three
//! reusable resolvers.
//!
//! The module has three resolvers:
//!
//! 1. [`resolve_failure_triage_panel`] — takes one failure's category, triage disposition,
//!    result origin, classifier confidence, recent attempt sequence, and whether it carries an
//!    assertion/diff summary and environment/build/runtime deltas, and produces one
//!    [`M5ResolvedFailureTriagePanel`] carrying the derived triage posture (one distinct
//!    posture per failure category), whether it provides evidence context, whether the exact
//!    selection can be rerun, whether a debug session can be opened, whether the quarantine
//!    review can be opened (only once evidence context is present), and the bounded
//!    reveal-evidence / rerun / debug / open-review / export actions. It never offers a
//!    destructive suppression path without evidence context and never drops the recent attempt
//!    sequence or the deltas.
//! 2. [`resolve_quarantine_review_sheet`] — takes one suppression's kind, scope, ownership,
//!    release impact, expiry state, linked-artifacts flag, reason, and owner, and produces one
//!    [`M5ResolvedQuarantineReviewSheet`] carrying the derived review posture (an expired,
//!    unowned, hidden-release, review-due, blocking, or governed suppression — honesty-first),
//!    whether it stays visible, whether it preserves its reason, whether its owner is
//!    accountable, and the bounded reveal / restore / renew / reassign / open-artifacts /
//!    export actions. It always keeps the suppressed test visible, always preserves the reason,
//!    always offers restore, and never hides owner, expiry, or release impact.
//! 3. [`resolve_environment_matrix_card`] — takes one card's target class, primary environment
//!    lane, and the compared environment legs (each with its target/runtime/toolchain/build
//!    compatibility class), and produces one [`M5ResolvedEnvironmentMatrixCard`] carrying the
//!    derived card posture (a compatible, mixed, unverified, or incompatible matrix — the worst
//!    axis across every leg), the overall compatibility, whether any leg is incompatible, and
//!    the bounded reveal / inspect / rerun-on-leg / export actions. It never asserts safe
//!    equivalence across incompatible or unverified environments.
//!
//! A single parity matrix — [`M5QualityTriageStatusPacket`] — binds one row per claimed M5
//! quality surface (the test-explorer triage view, the editor inline triage, the notebook
//! triage view, the run-panel triage, and the quality report export) to the shared triage,
//! quarantine, and environment anatomy, the same failure categories, dispositions, confidences,
//! ownership classes, release impacts, expiry states, compatibility classes, postures, bounded
//! actions, export fields, and non-visual accessibility routes, so the triage, suppression, and
//! environment vocabulary stays identical across every quality surface.
//!
//! The failure category ([`M5FailureCategory`]), triage disposition ([`M5TriageDisposition`]),
//! quarantine ownership ([`M5QuarantineOwnership`]), release impact ([`M5TestReleaseImpact`]),
//! target class ([`M5TestTargetClass`]), environment lane ([`M5TestEnvironmentLane`]), attempt
//! lineage kind ([`M5AttemptLineageKind`]), result origin ([`M5TestResultOrigin`]), surface
//! family ([`M5TestSurfaceFamily`]), deployment line ([`M5TestDeploymentLine`]), consumer
//! surface ([`M5TestConsumerSurface`]), accessibility route ([`M5TestAccessibilityRoute`]),
//! qualification class ([`M5TestQualificationClass`]), and downgrade trigger
//! ([`M5TestDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module mints
//! new vocabulary only for what that matrix left implicit about the three triage components
//! themselves.
//!
//! Raw log bodies, pasted paths, credentials, and private endpoints stay outside the export
//! boundary; every reason, label, and identity is carried only as an opaque, export-safe
//! representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_quality_triage_status_editor_inline_triage_beta_narrowed,
    seeded_m5_quality_triage_status_notebook_triage_preview_narrowed,
    seeded_m5_quality_triage_status_packet, M5_QUALITY_TRIAGE_STATUS_PACKET_ID,
};

// The failure category, triage disposition, quarantine ownership, release impact, target
// class, environment lane, attempt lineage kind, result origin, surface family, deployment
// line, consumer surface, accessibility route, qualification class, and downgrade triggers are
// frozen once, in the test-explorer / watch / triage component matrix. These primitives reuse
// them verbatim so they never invent parallel triage, suppression, or environment vocabulary.
pub use crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix::{
    M5AttemptLineageKind, M5FailureCategory, M5QuarantineOwnership, M5TestAccessibilityRoute,
    M5TestConsumerSurface, M5TestDeploymentLine, M5TestDowngradeTrigger, M5TestEnvironmentLane,
    M5TestQualificationClass, M5TestReleaseImpact, M5TestResultOrigin, M5TestSurfaceFamily,
    M5TestTargetClass, M5TriageDisposition,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5QualityTriageStatusPacket`].
pub const M5_QUALITY_TRIAGE_STATUS_RECORD_KIND: &str =
    "implement_m5_failure_triage_panels_quarantine_review_sheets_and_environment_matrix_cards_with_assertion_diff_summaries_recent_attempts_env_build_runtime_deltas_owner_expiry_release_impact_and_rerun_debug_review_parity_across_claimed_m5_quality_surfaces";

/// Schema version for M5 failure-triage / quarantine-review / environment-matrix records.
pub const M5_QUALITY_TRIAGE_STATUS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the failure-triage-panel boundary schema (the canonical packet
/// schema).
pub const M5_QUALITY_TRIAGE_STATUS_TRIAGE_SCHEMA_REF: &str =
    "schemas/ui/m5-test-failure-triage-panel.schema.json";

/// Repo-relative path of the quarantine-review-sheet companion schema.
pub const M5_QUALITY_TRIAGE_STATUS_QUARANTINE_SCHEMA_REF: &str =
    "schemas/ui/m5-test-quarantine-review-sheet.schema.json";

/// Repo-relative path of the environment-matrix-card companion schema.
pub const M5_QUALITY_TRIAGE_STATUS_ENVIRONMENT_SCHEMA_REF: &str =
    "schemas/ui/m5-test-environment-matrix-card.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_QUALITY_TRIAGE_STATUS_DOC_REF: &str =
    "docs/testing/m5_failure_triage_quarantine_environment_primitive.md";

/// Repo-relative path of the frozen test-explorer / watch / triage component matrix these
/// primitives narrow from.
pub const M5_QUALITY_TRIAGE_STATUS_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-test-explorer-watch-triage-component-matrix.schema.json";

/// Repo-relative path of the quarantine-record contract the review sheet binds its scope /
/// owner / expiry / release-impact truth against.
pub const M5_QUALITY_TRIAGE_STATUS_QUARANTINE_RECORD_REF: &str =
    "schemas/testing/test_quarantine_record.schema.json";

/// Repo-relative path of the stability-verdict / release-visibility contract the review sheet
/// binds its release-impact truth against.
pub const M5_QUALITY_TRIAGE_STATUS_RELEASE_VISIBILITY_REF: &str =
    "schemas/testing/stability-verdicts-quarantines-and-release-visibility.schema.json";

/// Repo-relative path of the session-plan / attempt-record contract the triage panel binds its
/// recent-attempt-sequence truth against.
pub const M5_QUALITY_TRIAGE_STATUS_ATTEMPT_RECORDS_REF: &str =
    "schemas/testing/session-plans-attempt-records-and-execution-lineage.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_QUALITY_TRIAGE_STATUS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-failure-triage-quarantine-environment-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_QUALITY_TRIAGE_STATUS_ARTIFACT_REF: &str =
    "artifacts/release/m5-failure-triage-quarantine-environment-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_QUALITY_TRIAGE_STATUS_CSV_REF: &str =
    "artifacts/release/m5-failure-triage-quarantine-environment-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_QUALITY_TRIAGE_STATUS_REPORT_REF: &str =
    "artifacts/design/m5-failure-triage-quarantine-environment-primitive.md";

/// One claimed M5 quality surface that renders the shared failure-triage panel, quarantine-
/// review sheet, and environment-matrix card. These are the surfaces the integration
/// touchpoints name — the test-explorer triage view, the editor inline triage, the notebook
/// triage view, the run-panel triage, and the quality report export — so the same triage
/// grammar works across every claimed quality lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QualityTriageConsumerSurface {
    /// The test-explorer triage view surface.
    TestExplorerTriageView,
    /// The editor inline triage surface.
    EditorInlineTriage,
    /// The notebook triage view surface.
    NotebookTriageView,
    /// The run-panel triage surface.
    RunPanelTriage,
    /// The quality report export surface.
    QualityReportExport,
}

impl M5QualityTriageConsumerSurface {
    /// Every claimed quality surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TestExplorerTriageView,
        Self::EditorInlineTriage,
        Self::NotebookTriageView,
        Self::RunPanelTriage,
        Self::QualityReportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestExplorerTriageView => "test_explorer_triage_view",
            Self::EditorInlineTriage => "editor_inline_triage",
            Self::NotebookTriageView => "notebook_triage_view",
            Self::RunPanelTriage => "run_panel_triage",
            Self::QualityReportExport => "quality_report_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::TestExplorerTriageView => "Test Explorer Triage View",
            Self::EditorInlineTriage => "Editor Inline Triage",
            Self::NotebookTriageView => "Notebook Triage View",
            Self::RunPanelTriage => "Run Panel Triage",
            Self::QualityReportExport => "Quality Report Export",
        }
    }
}

// ===== failure-triage-panel vocabulary ===================================

/// Controlled classifier confidence — how confident the flaky/failure classifier is in its
/// disposition, so a triage panel never presents a low-confidence classification as a settled
/// verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClassifierConfidence {
    /// High classifier confidence.
    HighConfidence,
    /// Medium classifier confidence.
    MediumConfidence,
    /// Low classifier confidence.
    LowConfidence,
    /// Confidence is unknown / not computed.
    UnknownConfidence,
}

impl M5ClassifierConfidence {
    /// Every classifier confidence, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::HighConfidence,
        Self::MediumConfidence,
        Self::LowConfidence,
        Self::UnknownConfidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighConfidence => "high_confidence",
            Self::MediumConfidence => "medium_confidence",
            Self::LowConfidence => "low_confidence",
            Self::UnknownConfidence => "unknown_confidence",
        }
    }

    /// True when the confidence is too low to treat the disposition as settled.
    pub const fn is_provisional(self) -> bool {
        matches!(self, Self::LowConfidence | Self::UnknownConfidence)
    }
}

/// The derived posture of a failure-triage panel — one distinct posture per failure category,
/// so a panel always names what class of failure it is triaging. Computed 1:1 from the failure
/// category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TriagePanelPosture {
    /// An assertion failure under triage.
    AssertionEvidencePanel,
    /// A runtime error under triage.
    RuntimeEvidencePanel,
    /// A timeout under triage.
    TimeoutEvidencePanel,
    /// An environment error under triage.
    EnvironmentEvidencePanel,
    /// A flaky failure under review.
    FlakyReviewPanel,
    /// An unclassified failure under triage.
    UnclassifiedEvidencePanel,
}

impl M5TriagePanelPosture {
    /// Every triage posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AssertionEvidencePanel,
        Self::RuntimeEvidencePanel,
        Self::TimeoutEvidencePanel,
        Self::EnvironmentEvidencePanel,
        Self::FlakyReviewPanel,
        Self::UnclassifiedEvidencePanel,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssertionEvidencePanel => "assertion_evidence_panel",
            Self::RuntimeEvidencePanel => "runtime_evidence_panel",
            Self::TimeoutEvidencePanel => "timeout_evidence_panel",
            Self::EnvironmentEvidencePanel => "environment_evidence_panel",
            Self::FlakyReviewPanel => "flaky_review_panel",
            Self::UnclassifiedEvidencePanel => "unclassified_evidence_panel",
        }
    }
}

/// One bounded action a failure-triage panel offers, so a panel never hides its reveal / rerun
/// / debug / open-review / export affordances — and the only route from a red failure to a
/// suppression is through the evidence-first open-review action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TriagePanelAction {
    /// Reveal the assertion/diff summary, recent attempts, deltas, and confidence.
    RevealTriageEvidence,
    /// Rerun the exact selection this failure came from.
    RerunExactSelection,
    /// Open a debug session for this failure.
    OpenDebugSession,
    /// Open the quarantine-review sheet (only once evidence context is present).
    OpenQuarantineReview,
    /// Export the triage panel as test evidence.
    ExportTriage,
}

impl M5TriagePanelAction {
    /// Every triage action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealTriageEvidence,
        Self::RerunExactSelection,
        Self::OpenDebugSession,
        Self::OpenQuarantineReview,
        Self::ExportTriage,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealTriageEvidence => "reveal_triage_evidence",
            Self::RerunExactSelection => "rerun_exact_selection",
            Self::OpenDebugSession => "open_debug_session",
            Self::OpenQuarantineReview => "open_quarantine_review",
            Self::ExportTriage => "export_triage",
        }
    }
}

/// Controlled failure-triage-panel anatomy part. The parts in
/// [`M5TriagePanelAnatomyPart::MANDATORY`] are required on every panel so the assertion/diff
/// summary, recent attempt sequence, environment/build/runtime deltas, classifier confidence,
/// and action row are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TriagePanelAnatomyPart {
    /// The assertion/diff summary cue.
    AssertionDiffSummaryCue,
    /// The recent attempt sequence cue.
    RecentAttemptSequenceCue,
    /// The environment/build/runtime delta cue.
    EnvBuildRuntimeDeltaCue,
    /// The classifier-confidence cue.
    ClassifierConfidenceCue,
    /// The failure-category cue.
    FailureCategoryCue,
    /// The triage-disposition cue.
    TriageDispositionCue,
    /// The result-origin cue.
    ResultOriginCue,
    /// The triage-action row cue.
    TriageActionCue,
}

impl M5TriagePanelAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::AssertionDiffSummaryCue,
        Self::RecentAttemptSequenceCue,
        Self::EnvBuildRuntimeDeltaCue,
        Self::ClassifierConfidenceCue,
        Self::FailureCategoryCue,
        Self::TriageDispositionCue,
        Self::ResultOriginCue,
        Self::TriageActionCue,
    ];

    /// The anatomy parts every triage panel must render.
    pub const MANDATORY: [Self; 5] = [
        Self::AssertionDiffSummaryCue,
        Self::RecentAttemptSequenceCue,
        Self::EnvBuildRuntimeDeltaCue,
        Self::ClassifierConfidenceCue,
        Self::TriageActionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssertionDiffSummaryCue => "assertion_diff_summary_cue",
            Self::RecentAttemptSequenceCue => "recent_attempt_sequence_cue",
            Self::EnvBuildRuntimeDeltaCue => "env_build_runtime_delta_cue",
            Self::ClassifierConfidenceCue => "classifier_confidence_cue",
            Self::FailureCategoryCue => "failure_category_cue",
            Self::TriageDispositionCue => "triage_disposition_cue",
            Self::ResultOriginCue => "result_origin_cue",
            Self::TriageActionCue => "triage_action_cue",
        }
    }
}

/// A field the triage export carries so triage-panel truth is reconstructable. The fields in
/// [`M5TriagePanelExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TriagePanelExportField {
    /// The failure category.
    FailureCategory,
    /// The triage disposition.
    TriageDisposition,
    /// The classifier confidence.
    ClassifierConfidence,
    /// The result origin.
    ResultOrigin,
    /// The recent attempt count.
    RecentAttemptCount,
    /// Whether an assertion/diff summary is present.
    HasAssertionDiff,
    /// Whether environment/build/runtime deltas are present.
    HasEnvDelta,
    /// The derived triage posture.
    TriagePosture,
}

impl M5TriagePanelExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::FailureCategory,
        Self::TriageDisposition,
        Self::ClassifierConfidence,
        Self::ResultOrigin,
        Self::RecentAttemptCount,
        Self::HasAssertionDiff,
        Self::HasEnvDelta,
        Self::TriagePosture,
    ];

    /// The export fields every triage panel must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::FailureCategory,
        Self::TriageDisposition,
        Self::ClassifierConfidence,
        Self::RecentAttemptCount,
        Self::TriagePosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FailureCategory => "failure_category",
            Self::TriageDisposition => "triage_disposition",
            Self::ClassifierConfidence => "classifier_confidence",
            Self::ResultOrigin => "result_origin",
            Self::RecentAttemptCount => "recent_attempt_count",
            Self::HasAssertionDiff => "has_assertion_diff",
            Self::HasEnvDelta => "has_env_delta",
            Self::TriagePosture => "triage_posture",
        }
    }
}

// ===== quarantine-review-sheet vocabulary ================================

/// Controlled suppression kind — how a failing test is suppressed, so a review sheet never
/// leaves implicit whether a result is muted, quarantined, or skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuppressionKind {
    /// The result is muted but the test still runs.
    Muted,
    /// The test is quarantined / isolated from gating.
    Quarantined,
    /// The test is skipped and does not run.
    Skipped,
}

impl M5SuppressionKind {
    /// Every suppression kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::Muted, Self::Quarantined, Self::Skipped];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Muted => "muted",
            Self::Quarantined => "quarantined",
            Self::Skipped => "skipped",
        }
    }
}

/// Controlled suppression scope — what a mute / quarantine covers, so the scope is never left
/// implicit and a wide suppression is never mistaken for a narrow one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuppressionScope {
    /// A single test case.
    SingleCase,
    /// A parametrization subset within a case.
    ParametrizationSubset,
    /// A whole file.
    WholeFile,
    /// A whole suite.
    WholeSuite,
    /// A tagged group of tests.
    TaggedGroup,
}

impl M5SuppressionScope {
    /// Every suppression scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SingleCase,
        Self::ParametrizationSubset,
        Self::WholeFile,
        Self::WholeSuite,
        Self::TaggedGroup,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleCase => "single_case",
            Self::ParametrizationSubset => "parametrization_subset",
            Self::WholeFile => "whole_file",
            Self::WholeSuite => "whole_suite",
            Self::TaggedGroup => "tagged_group",
        }
    }
}

/// Controlled quarantine expiry state — when a suppression lapses, so a review sheet never
/// leaves expiry implicit and an expired suppression never silently lingers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QuarantineExpiry {
    /// No expiry is set.
    NoExpiry,
    /// An expiry is scheduled in the future.
    ExpiresScheduled,
    /// A review is due now.
    ReviewDue,
    /// The suppression has expired.
    Expired,
    /// A permanent policy suppression with no expiry.
    PermanentPolicy,
}

impl M5QuarantineExpiry {
    /// Every expiry state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoExpiry,
        Self::ExpiresScheduled,
        Self::ReviewDue,
        Self::Expired,
        Self::PermanentPolicy,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoExpiry => "no_expiry",
            Self::ExpiresScheduled => "expires_scheduled",
            Self::ReviewDue => "review_due",
            Self::Expired => "expired",
            Self::PermanentPolicy => "permanent_policy",
        }
    }
}

/// The derived posture of a quarantine-review sheet — honesty-first, so the most urgent review
/// condition is named first and a governed suppression is only claimed once nothing needs
/// attention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QuarantineReviewPosture {
    /// The suppression has expired and must be renewed or restored.
    ExpiredSuppression,
    /// The suppression has no accountable owner.
    UnownedSuppression,
    /// The suppression is hidden from release gating.
    HiddenReleaseSuppression,
    /// A review is due for this suppression.
    ReviewDueSuppression,
    /// The suppression still blocks release.
    BlockingSuppression,
    /// A governed, owned, non-expired suppression with disclosed impact.
    GovernedSuppression,
}

impl M5QuarantineReviewPosture {
    /// Every review posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpiredSuppression,
        Self::UnownedSuppression,
        Self::HiddenReleaseSuppression,
        Self::ReviewDueSuppression,
        Self::BlockingSuppression,
        Self::GovernedSuppression,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpiredSuppression => "expired_suppression",
            Self::UnownedSuppression => "unowned_suppression",
            Self::HiddenReleaseSuppression => "hidden_release_suppression",
            Self::ReviewDueSuppression => "review_due_suppression",
            Self::BlockingSuppression => "blocking_suppression",
            Self::GovernedSuppression => "governed_suppression",
        }
    }

    /// True only for a governed suppression — the only posture that needs no review attention.
    pub const fn is_governed(self) -> bool {
        matches!(self, Self::GovernedSuppression)
    }
}

/// One bounded action a quarantine-review sheet offers, so a sheet never hides its reveal /
/// restore / renew / reassign / open-artifacts / export affordances — the restore action is
/// always present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QuarantineReviewAction {
    /// Reveal the scope, reason, owner, expiry, linked artifacts, and release impact.
    RevealQuarantineDetails,
    /// Restore the test (lift the suppression).
    RestoreTest,
    /// Renew / re-time an expired or review-due suppression.
    RenewSuppression,
    /// Reassign an owner to an unowned or owner-expired suppression.
    ReassignOwner,
    /// Open the linked artifacts.
    OpenLinkedArtifacts,
    /// Export the quarantine review as test evidence.
    ExportQuarantine,
}

impl M5QuarantineReviewAction {
    /// Every review action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RevealQuarantineDetails,
        Self::RestoreTest,
        Self::RenewSuppression,
        Self::ReassignOwner,
        Self::OpenLinkedArtifacts,
        Self::ExportQuarantine,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealQuarantineDetails => "reveal_quarantine_details",
            Self::RestoreTest => "restore_test",
            Self::RenewSuppression => "renew_suppression",
            Self::ReassignOwner => "reassign_owner",
            Self::OpenLinkedArtifacts => "open_linked_artifacts",
            Self::ExportQuarantine => "export_quarantine",
        }
    }
}

/// Controlled quarantine-review-sheet anatomy part. The parts in
/// [`M5QuarantineReviewAnatomyPart::MANDATORY`] are required on every sheet so the scope,
/// reason, owner, expiry, release impact, and restore action are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QuarantineReviewAnatomyPart {
    /// The suppression-scope cue.
    SuppressionScopeCue,
    /// The suppression-reason cue.
    SuppressionReasonCue,
    /// The owner cue.
    OwnerCue,
    /// The expiry cue.
    ExpiryCue,
    /// The linked-artifacts cue.
    LinkedArtifactsCue,
    /// The release-impact cue.
    ReleaseImpactCue,
    /// The suppression-kind cue.
    SuppressionKindCue,
    /// The restore-action cue.
    RestoreActionCue,
}

impl M5QuarantineReviewAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SuppressionScopeCue,
        Self::SuppressionReasonCue,
        Self::OwnerCue,
        Self::ExpiryCue,
        Self::LinkedArtifactsCue,
        Self::ReleaseImpactCue,
        Self::SuppressionKindCue,
        Self::RestoreActionCue,
    ];

    /// The anatomy parts every quarantine review sheet must render.
    pub const MANDATORY: [Self; 6] = [
        Self::SuppressionScopeCue,
        Self::SuppressionReasonCue,
        Self::OwnerCue,
        Self::ExpiryCue,
        Self::ReleaseImpactCue,
        Self::RestoreActionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuppressionScopeCue => "suppression_scope_cue",
            Self::SuppressionReasonCue => "suppression_reason_cue",
            Self::OwnerCue => "owner_cue",
            Self::ExpiryCue => "expiry_cue",
            Self::LinkedArtifactsCue => "linked_artifacts_cue",
            Self::ReleaseImpactCue => "release_impact_cue",
            Self::SuppressionKindCue => "suppression_kind_cue",
            Self::RestoreActionCue => "restore_action_cue",
        }
    }
}

/// A field the quarantine export carries so review-sheet truth is reconstructable. The fields
/// in [`M5QuarantineReviewExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QuarantineReviewExportField {
    /// The suppression scope.
    SuppressionScope,
    /// The suppression kind.
    SuppressionKind,
    /// The quarantine ownership.
    Ownership,
    /// The release impact.
    ReleaseImpact,
    /// The expiry state.
    ExpiryState,
    /// Whether linked artifacts are present.
    HasLinkedArtifacts,
    /// Whether a reason is present.
    ReasonPresent,
    /// The derived quarantine posture.
    QuarantinePosture,
}

impl M5QuarantineReviewExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SuppressionScope,
        Self::SuppressionKind,
        Self::Ownership,
        Self::ReleaseImpact,
        Self::ExpiryState,
        Self::HasLinkedArtifacts,
        Self::ReasonPresent,
        Self::QuarantinePosture,
    ];

    /// The export fields every quarantine review sheet must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::SuppressionScope,
        Self::SuppressionKind,
        Self::Ownership,
        Self::ReleaseImpact,
        Self::ExpiryState,
        Self::QuarantinePosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuppressionScope => "suppression_scope",
            Self::SuppressionKind => "suppression_kind",
            Self::Ownership => "ownership",
            Self::ReleaseImpact => "release_impact",
            Self::ExpiryState => "expiry_state",
            Self::HasLinkedArtifacts => "has_linked_artifacts",
            Self::ReasonPresent => "reason_present",
            Self::QuarantinePosture => "quarantine_posture",
        }
    }
}

// ===== environment-matrix-card vocabulary ================================

/// Controlled environment compatibility class — how compatible a target/runtime/toolchain/build
/// axis is on a given leg, so an environment-matrix card never implies safe equivalence across
/// an axis that is not actually compatible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EnvCompatibilityClass {
    /// Fully compatible.
    FullyCompatible,
    /// Partially compatible.
    PartiallyCompatible,
    /// Incompatible.
    Incompatible,
    /// Unverified / not yet checked.
    Unverified,
    /// Not applicable on this axis.
    NotApplicable,
}

impl M5EnvCompatibilityClass {
    /// Every compatibility class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullyCompatible,
        Self::PartiallyCompatible,
        Self::Incompatible,
        Self::Unverified,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyCompatible => "fully_compatible",
            Self::PartiallyCompatible => "partially_compatible",
            Self::Incompatible => "incompatible",
            Self::Unverified => "unverified",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Severity rank used to pick the worst axis across a matrix. Lower is worse; `NotApplicable`
    /// ranks highest so it never dominates a genuine compatibility signal.
    const fn severity_rank(self) -> u8 {
        match self {
            Self::Incompatible => 0,
            Self::Unverified => 1,
            Self::PartiallyCompatible => 2,
            Self::FullyCompatible => 3,
            Self::NotApplicable => 4,
        }
    }

    /// True when this axis permits actually running the target here.
    const fn is_runnable(self) -> bool {
        matches!(self, Self::FullyCompatible | Self::PartiallyCompatible)
    }
}

/// The derived posture of an environment-matrix card — the worst compatibility across every
/// axis of every leg, so a card is only claimed compatible when nothing is incompatible,
/// unverified, or partial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EnvironmentCardPosture {
    /// At least one axis is incompatible.
    IncompatibleMatrix,
    /// At least one axis is unverified (and none incompatible).
    UnverifiedMatrix,
    /// At least one axis is only partially compatible (and none incompatible/unverified).
    MixedMatrix,
    /// Every applicable axis is fully compatible.
    CompatibleMatrix,
}

impl M5EnvironmentCardPosture {
    /// Every card posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::IncompatibleMatrix,
        Self::UnverifiedMatrix,
        Self::MixedMatrix,
        Self::CompatibleMatrix,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncompatibleMatrix => "incompatible_matrix",
            Self::UnverifiedMatrix => "unverified_matrix",
            Self::MixedMatrix => "mixed_matrix",
            Self::CompatibleMatrix => "compatible_matrix",
        }
    }

    /// True only for a fully compatible matrix — the only posture that may read as equivalent.
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::CompatibleMatrix)
    }
}

/// One bounded action an environment-matrix card offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EnvironmentCardAction {
    /// Reveal the per-axis compatibility details.
    RevealCompatibilityDetails,
    /// Inspect the per-leg runtime/toolchain/build deltas.
    InspectLegDeltas,
    /// Rerun on a selected compatible leg.
    RerunOnLeg,
    /// Export the environment matrix as test evidence.
    ExportEnvironmentMatrix,
}

impl M5EnvironmentCardAction {
    /// Every card action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RevealCompatibilityDetails,
        Self::InspectLegDeltas,
        Self::RerunOnLeg,
        Self::ExportEnvironmentMatrix,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealCompatibilityDetails => "reveal_compatibility_details",
            Self::InspectLegDeltas => "inspect_leg_deltas",
            Self::RerunOnLeg => "rerun_on_leg",
            Self::ExportEnvironmentMatrix => "export_environment_matrix",
        }
    }
}

/// Controlled environment-matrix-card anatomy part. The parts in
/// [`M5EnvironmentCardAnatomyPart::MANDATORY`] are required on every card so the target,
/// environment, runtime/toolchain/build deltas, and compatibility class are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EnvironmentCardAnatomyPart {
    /// The target-class cue.
    TargetClassCue,
    /// The environment-lane cue.
    EnvironmentLaneCue,
    /// The runtime-delta cue.
    RuntimeDeltaCue,
    /// The toolchain-delta cue.
    ToolchainDeltaCue,
    /// The build-delta cue.
    BuildDeltaCue,
    /// The compatibility-class cue.
    CompatibilityClassCue,
    /// The compatibility-action row cue.
    CompatibilityActionCue,
}

impl M5EnvironmentCardAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::TargetClassCue,
        Self::EnvironmentLaneCue,
        Self::RuntimeDeltaCue,
        Self::ToolchainDeltaCue,
        Self::BuildDeltaCue,
        Self::CompatibilityClassCue,
        Self::CompatibilityActionCue,
    ];

    /// The anatomy parts every environment card must render.
    pub const MANDATORY: [Self; 6] = [
        Self::TargetClassCue,
        Self::EnvironmentLaneCue,
        Self::RuntimeDeltaCue,
        Self::ToolchainDeltaCue,
        Self::BuildDeltaCue,
        Self::CompatibilityClassCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetClassCue => "target_class_cue",
            Self::EnvironmentLaneCue => "environment_lane_cue",
            Self::RuntimeDeltaCue => "runtime_delta_cue",
            Self::ToolchainDeltaCue => "toolchain_delta_cue",
            Self::BuildDeltaCue => "build_delta_cue",
            Self::CompatibilityClassCue => "compatibility_class_cue",
            Self::CompatibilityActionCue => "compatibility_action_cue",
        }
    }
}

/// A field the environment export carries so card truth is reconstructable. The fields in
/// [`M5EnvironmentCardExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EnvironmentCardExportField {
    /// The target class.
    TargetClass,
    /// The primary environment lane.
    EnvironmentLane,
    /// The number of compared legs.
    LegCount,
    /// The overall (worst) compatibility.
    OverallCompatibility,
    /// Whether any leg is incompatible.
    HasIncompatibleLeg,
    /// Whether any axis is unverified.
    HasUnverifiedAxis,
    /// The derived environment posture.
    EnvironmentPosture,
}

impl M5EnvironmentCardExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::TargetClass,
        Self::EnvironmentLane,
        Self::LegCount,
        Self::OverallCompatibility,
        Self::HasIncompatibleLeg,
        Self::HasUnverifiedAxis,
        Self::EnvironmentPosture,
    ];

    /// The export fields every environment card must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::TargetClass,
        Self::LegCount,
        Self::OverallCompatibility,
        Self::HasIncompatibleLeg,
        Self::EnvironmentPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetClass => "target_class",
            Self::EnvironmentLane => "environment_lane",
            Self::LegCount => "leg_count",
            Self::OverallCompatibility => "overall_compatibility",
            Self::HasIncompatibleLeg => "has_incompatible_leg",
            Self::HasUnverifiedAxis => "has_unverified_axis",
            Self::EnvironmentPosture => "environment_posture",
        }
    }
}

/// True when a result origin is a live-local result that can be debugged directly here.
const fn origin_is_live_local(origin: M5TestResultOrigin) -> bool {
    matches!(origin, M5TestResultOrigin::LiveLocal)
}

/// True when a quarantine ownership class names an accountable owner (not unowned or expired).
const fn ownership_is_accountable(ownership: M5QuarantineOwnership) -> bool {
    matches!(
        ownership,
        M5QuarantineOwnership::SelfOwned
            | M5QuarantineOwnership::TeamOwned
            | M5QuarantineOwnership::CiEnforced
            | M5QuarantineOwnership::ImportedPolicy
    )
}

// ---- failure-triage-panel resolver --------------------------------------

/// The full input to the failure-triage-panel resolver for one failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TriagePanelResolutionInput {
    /// The failure category.
    pub failure_category: M5FailureCategory,
    /// The triage disposition.
    pub triage_disposition: M5TriageDisposition,
    /// The result origin (live-local vs imported), which gates debug.
    pub result_origin: M5TestResultOrigin,
    /// The classifier confidence behind the disposition.
    pub classifier_confidence: M5ClassifierConfidence,
    /// The recent attempt sequence (must be non-empty — a panel always shows recent attempts).
    pub recent_attempts: Vec<M5AttemptLineageKind>,
    /// Whether the panel carries an assertion/diff summary.
    pub has_assertion_or_diff_summary: bool,
    /// Whether the panel carries environment/build/runtime deltas.
    pub has_env_build_runtime_delta: bool,
    /// The opaque user-facing assertion/diff summary label (must be non-empty).
    pub assertion_summary_label: String,
    /// The opaque stable panel identity (must be non-empty).
    pub panel_identity_ref: String,
}

/// The resolved failure-triage-panel truth for one failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedFailureTriagePanel {
    /// The failure category.
    pub failure_category: M5FailureCategory,
    /// The triage disposition.
    pub triage_disposition: M5TriageDisposition,
    /// The result origin.
    pub result_origin: M5TestResultOrigin,
    /// The classifier confidence.
    pub classifier_confidence: M5ClassifierConfidence,
    /// The recent attempt sequence, preserved exactly from the input.
    pub recent_attempts: Vec<M5AttemptLineageKind>,
    /// The recent attempt count, derived from the sequence.
    pub recent_attempt_count: u32,
    /// Whether the panel carries an assertion/diff summary.
    pub has_assertion_or_diff_summary: bool,
    /// Whether the panel carries environment/build/runtime deltas.
    pub has_env_build_runtime_delta: bool,
    /// The opaque assertion/diff summary label, preserved exactly from the input.
    pub assertion_summary_label: String,
    /// The opaque stable panel identity, preserved exactly from the input.
    pub panel_identity_ref: String,
    /// The derived triage posture.
    pub triage_posture: M5TriagePanelPosture,
    /// The bounded actions this panel offers.
    pub available_actions: Vec<M5TriagePanelAction>,
    /// True when the panel provides evidence context (assertion/diff, deltas, or attempts).
    pub provides_evidence_context: bool,
    /// True when the exact selection can be rerun.
    pub can_rerun: bool,
    /// True when a debug session can be opened (live-local origin only).
    pub can_debug: bool,
    /// True when the quarantine review can be opened — only once evidence context is present.
    pub can_open_review: bool,
    /// True when the disposition is provisional (low/unknown confidence or needs-investigation).
    pub disposition_is_provisional: bool,
    /// True when the panel still needs triage attention (not resolved).
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_failure_triage_panel`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5TriagePanelResolutionError {
    /// The recent attempt sequence was empty.
    EmptyRecentAttempts,
    /// The assertion/diff summary label was empty.
    EmptyAssertionSummary,
    /// The panel identity ref was empty.
    EmptyPanelIdentity,
    /// A triage descriptor carried forbidden material.
    ForbiddenTriageMaterial,
}

impl M5TriagePanelResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyRecentAttempts => "empty_recent_attempts",
            Self::EmptyAssertionSummary => "empty_assertion_summary",
            Self::EmptyPanelIdentity => "empty_panel_identity",
            Self::ForbiddenTriageMaterial => "forbidden_triage_material",
        }
    }
}

impl fmt::Display for M5TriagePanelResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "failure triage panel resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5TriagePanelResolutionError {}

/// Resolves one failure-triage panel from its declared failure state.
///
/// The derived triage posture is 1:1 with the failure category, so a panel always names what
/// class of failure it is triaging. The recent attempt sequence must be non-empty — a panel
/// always shows recent attempts. The panel provides evidence context whenever it carries an
/// assertion/diff summary, environment/build/runtime deltas, or a recent attempt sequence, and
/// it only exposes the quarantine-review path (the route to a destructive suppression) once that
/// evidence context is present, so a user never jumps from a red row straight to suppression
/// without evidence. Rerun is always offered; debug is offered only for a live-local origin.
pub fn resolve_failure_triage_panel(
    input: &M5TriagePanelResolutionInput,
) -> Result<M5ResolvedFailureTriagePanel, M5TriagePanelResolutionError> {
    if input.recent_attempts.is_empty() {
        return Err(M5TriagePanelResolutionError::EmptyRecentAttempts);
    }
    if input.assertion_summary_label.trim().is_empty() {
        return Err(M5TriagePanelResolutionError::EmptyAssertionSummary);
    }
    if input.panel_identity_ref.trim().is_empty() {
        return Err(M5TriagePanelResolutionError::EmptyPanelIdentity);
    }
    if value_repr_is_forbidden(&input.assertion_summary_label)
        || value_repr_is_forbidden(&input.panel_identity_ref)
    {
        return Err(M5TriagePanelResolutionError::ForbiddenTriageMaterial);
    }

    let triage_posture = derive_triage_posture(input.failure_category);
    let provides_evidence_context = input.has_assertion_or_diff_summary
        || input.has_env_build_runtime_delta
        || !input.recent_attempts.is_empty();
    let can_rerun = true;
    let can_debug = origin_is_live_local(input.result_origin);
    let can_open_review = provides_evidence_context;
    let disposition_is_provisional = input.classifier_confidence.is_provisional()
        || matches!(
            input.triage_disposition,
            M5TriageDisposition::NeedsInvestigation
        );
    let available_actions = derive_triage_actions(can_rerun, can_debug, can_open_review);

    Ok(M5ResolvedFailureTriagePanel {
        failure_category: input.failure_category,
        triage_disposition: input.triage_disposition,
        result_origin: input.result_origin,
        classifier_confidence: input.classifier_confidence,
        recent_attempts: input.recent_attempts.clone(),
        recent_attempt_count: input.recent_attempts.len() as u32,
        has_assertion_or_diff_summary: input.has_assertion_or_diff_summary,
        has_env_build_runtime_delta: input.has_env_build_runtime_delta,
        assertion_summary_label: input.assertion_summary_label.clone(),
        panel_identity_ref: input.panel_identity_ref.clone(),
        triage_posture,
        available_actions,
        provides_evidence_context,
        can_rerun,
        can_debug,
        can_open_review,
        disposition_is_provisional,
        needs_attention: !matches!(
            input.triage_disposition,
            M5TriageDisposition::ResolvedTriage
        ),
    })
}

/// The 1:1 failure-category → triage-posture map.
fn derive_triage_posture(category: M5FailureCategory) -> M5TriagePanelPosture {
    match category {
        M5FailureCategory::AssertionFailure => M5TriagePanelPosture::AssertionEvidencePanel,
        M5FailureCategory::RuntimeError => M5TriagePanelPosture::RuntimeEvidencePanel,
        M5FailureCategory::Timeout => M5TriagePanelPosture::TimeoutEvidencePanel,
        M5FailureCategory::EnvironmentError => M5TriagePanelPosture::EnvironmentEvidencePanel,
        M5FailureCategory::FlakyUnderReview => M5TriagePanelPosture::FlakyReviewPanel,
        M5FailureCategory::UnknownFailure => M5TriagePanelPosture::UnclassifiedEvidencePanel,
    }
}

/// Derives the bounded triage-action set from the rerun / debug / open-review signals. The
/// open-review action — the only route to a suppression — is offered only when evidence context
/// is present.
fn derive_triage_actions(
    can_rerun: bool,
    can_debug: bool,
    can_open_review: bool,
) -> Vec<M5TriagePanelAction> {
    use M5TriagePanelAction as Action;
    let mut actions = vec![Action::RevealTriageEvidence];
    if can_rerun {
        actions.push(Action::RerunExactSelection);
    }
    if can_debug {
        actions.push(Action::OpenDebugSession);
    }
    if can_open_review {
        actions.push(Action::OpenQuarantineReview);
    }
    actions.push(Action::ExportTriage);
    actions
}

// ---- quarantine-review-sheet resolver -----------------------------------

/// The full input to the quarantine-review-sheet resolver for one suppression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QuarantineReviewResolutionInput {
    /// The suppression kind.
    pub suppression_kind: M5SuppressionKind,
    /// The suppression scope.
    pub suppression_scope: M5SuppressionScope,
    /// The quarantine ownership class.
    pub ownership: M5QuarantineOwnership,
    /// The release impact.
    pub release_impact: M5TestReleaseImpact,
    /// The expiry state.
    pub expiry_state: M5QuarantineExpiry,
    /// Whether linked artifacts are present.
    pub has_linked_artifacts: bool,
    /// The opaque suppression reason (must be non-empty — the reason is always preserved).
    pub reason_label: String,
    /// The opaque owner label (must be non-empty).
    pub owner_label: String,
    /// The opaque stable sheet identity (must be non-empty).
    pub sheet_identity_ref: String,
}

/// The resolved quarantine-review-sheet truth for one suppression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedQuarantineReviewSheet {
    /// The suppression kind.
    pub suppression_kind: M5SuppressionKind,
    /// The suppression scope.
    pub suppression_scope: M5SuppressionScope,
    /// The quarantine ownership class.
    pub ownership: M5QuarantineOwnership,
    /// The release impact.
    pub release_impact: M5TestReleaseImpact,
    /// The expiry state.
    pub expiry_state: M5QuarantineExpiry,
    /// Whether linked artifacts are present.
    pub has_linked_artifacts: bool,
    /// The opaque suppression reason, preserved exactly from the input.
    pub reason_label: String,
    /// The opaque owner label, preserved exactly from the input.
    pub owner_label: String,
    /// The opaque stable sheet identity, preserved exactly from the input.
    pub sheet_identity_ref: String,
    /// The derived review posture.
    pub review_posture: M5QuarantineReviewPosture,
    /// The bounded actions this sheet offers.
    pub available_actions: Vec<M5QuarantineReviewAction>,
    /// True when the suppressed test stays visible (never hidden behind a filter).
    pub stays_visible: bool,
    /// True when the reason is preserved.
    pub preserves_reason: bool,
    /// True when the owner is accountable (not unowned or owner-expired).
    pub owner_is_accountable: bool,
    /// True when the suppression has expired.
    pub is_expired: bool,
    /// True when a review is due.
    pub review_due: bool,
    /// True when the suppression still blocks release.
    pub blocks_release: bool,
    /// True when the suppression is hidden from release gating.
    pub hidden_from_release: bool,
    /// True when the test can be restored (always).
    pub can_restore: bool,
    /// True when the sheet still needs review attention (not a governed suppression).
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_quarantine_review_sheet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5QuarantineReviewResolutionError {
    /// The suppression reason was empty.
    EmptyReason,
    /// The owner label was empty.
    EmptyOwnerLabel,
    /// The sheet identity ref was empty.
    EmptyQuarantineIdentity,
    /// A quarantine descriptor carried forbidden material.
    ForbiddenQuarantineMaterial,
}

impl M5QuarantineReviewResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyReason => "empty_reason",
            Self::EmptyOwnerLabel => "empty_owner_label",
            Self::EmptyQuarantineIdentity => "empty_quarantine_identity",
            Self::ForbiddenQuarantineMaterial => "forbidden_quarantine_material",
        }
    }
}

impl fmt::Display for M5QuarantineReviewResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "quarantine review sheet resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5QuarantineReviewResolutionError {}

/// Resolves one quarantine-review sheet from its declared suppression state.
///
/// The derived review posture is honesty-first — an expired suppression is named before an
/// unowned one, before a hidden-release one, before a review-due one, before a blocking one, and
/// only a fully owned, non-expired, disclosed suppression reads as governed. The suppressed test
/// always stays visible (never hidden behind a filter), the reason is always preserved, the owner
/// / expiry / release impact are always carried, and restore is always offered. Renew is offered
/// for an expired or review-due suppression; reassign for an unowned or owner-expired one;
/// open-artifacts when linked artifacts are present.
pub fn resolve_quarantine_review_sheet(
    input: &M5QuarantineReviewResolutionInput,
) -> Result<M5ResolvedQuarantineReviewSheet, M5QuarantineReviewResolutionError> {
    if input.reason_label.trim().is_empty() {
        return Err(M5QuarantineReviewResolutionError::EmptyReason);
    }
    if input.owner_label.trim().is_empty() {
        return Err(M5QuarantineReviewResolutionError::EmptyOwnerLabel);
    }
    if input.sheet_identity_ref.trim().is_empty() {
        return Err(M5QuarantineReviewResolutionError::EmptyQuarantineIdentity);
    }
    if value_repr_is_forbidden(&input.reason_label)
        || value_repr_is_forbidden(&input.owner_label)
        || value_repr_is_forbidden(&input.sheet_identity_ref)
    {
        return Err(M5QuarantineReviewResolutionError::ForbiddenQuarantineMaterial);
    }

    let owner_is_accountable = ownership_is_accountable(input.ownership);
    let is_expired = matches!(input.expiry_state, M5QuarantineExpiry::Expired);
    let review_due = matches!(input.expiry_state, M5QuarantineExpiry::ReviewDue);
    let blocks_release = matches!(input.release_impact, M5TestReleaseImpact::BlocksRelease);
    let hidden_from_release =
        matches!(input.release_impact, M5TestReleaseImpact::HiddenFromRelease);

    let review_posture = derive_quarantine_posture(
        is_expired,
        owner_is_accountable,
        hidden_from_release,
        review_due,
        blocks_release,
    );
    let available_actions = derive_quarantine_actions(
        is_expired,
        review_due,
        owner_is_accountable,
        input.has_linked_artifacts,
    );

    Ok(M5ResolvedQuarantineReviewSheet {
        suppression_kind: input.suppression_kind,
        suppression_scope: input.suppression_scope,
        ownership: input.ownership,
        release_impact: input.release_impact,
        expiry_state: input.expiry_state,
        has_linked_artifacts: input.has_linked_artifacts,
        reason_label: input.reason_label.clone(),
        owner_label: input.owner_label.clone(),
        sheet_identity_ref: input.sheet_identity_ref.clone(),
        review_posture,
        available_actions,
        stays_visible: true,
        preserves_reason: !input.reason_label.trim().is_empty(),
        owner_is_accountable,
        is_expired,
        review_due,
        blocks_release,
        hidden_from_release,
        can_restore: true,
        needs_attention: !review_posture.is_governed(),
    })
}

/// The honesty-first suppression-state → review-posture ladder.
fn derive_quarantine_posture(
    is_expired: bool,
    owner_is_accountable: bool,
    hidden_from_release: bool,
    review_due: bool,
    blocks_release: bool,
) -> M5QuarantineReviewPosture {
    if is_expired {
        M5QuarantineReviewPosture::ExpiredSuppression
    } else if !owner_is_accountable {
        M5QuarantineReviewPosture::UnownedSuppression
    } else if hidden_from_release {
        M5QuarantineReviewPosture::HiddenReleaseSuppression
    } else if review_due {
        M5QuarantineReviewPosture::ReviewDueSuppression
    } else if blocks_release {
        M5QuarantineReviewPosture::BlockingSuppression
    } else {
        M5QuarantineReviewPosture::GovernedSuppression
    }
}

/// Derives the bounded quarantine-action set. Restore is always present.
fn derive_quarantine_actions(
    is_expired: bool,
    review_due: bool,
    owner_is_accountable: bool,
    has_linked_artifacts: bool,
) -> Vec<M5QuarantineReviewAction> {
    use M5QuarantineReviewAction as Action;
    let mut actions = vec![Action::RevealQuarantineDetails, Action::RestoreTest];
    if is_expired || review_due {
        actions.push(Action::RenewSuppression);
    }
    if !owner_is_accountable {
        actions.push(Action::ReassignOwner);
    }
    if has_linked_artifacts {
        actions.push(Action::OpenLinkedArtifacts);
    }
    actions.push(Action::ExportQuarantine);
    actions
}

// ---- environment-matrix-card resolver -----------------------------------

/// One compared environment leg on a matrix card, carrying its per-axis compatibility classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvironmentCompatibilityLeg {
    /// The environment lane this leg runs in.
    pub environment_lane: M5TestEnvironmentLane,
    /// The target compatibility on this leg.
    pub target_compatibility: M5EnvCompatibilityClass,
    /// The runtime compatibility on this leg.
    pub runtime_compatibility: M5EnvCompatibilityClass,
    /// The toolchain compatibility on this leg.
    pub toolchain_compatibility: M5EnvCompatibilityClass,
    /// The build compatibility on this leg.
    pub build_compatibility: M5EnvCompatibilityClass,
    /// The opaque user-facing leg label (must be non-empty).
    pub leg_label: String,
}

impl M5EnvironmentCompatibilityLeg {
    /// The four per-axis compatibility classes on this leg.
    fn axes(&self) -> [M5EnvCompatibilityClass; 4] {
        [
            self.target_compatibility,
            self.runtime_compatibility,
            self.toolchain_compatibility,
            self.build_compatibility,
        ]
    }

    /// The worst compatibility class across this leg's axes.
    fn worst_axis(&self) -> M5EnvCompatibilityClass {
        self.axes()
            .into_iter()
            .min_by_key(|axis| axis.severity_rank())
            .unwrap_or(M5EnvCompatibilityClass::NotApplicable)
    }
}

/// The full input to the environment-matrix-card resolver for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvironmentCardResolutionInput {
    /// The test target class this card compares.
    pub target_class: M5TestTargetClass,
    /// The primary environment lane (the one the user is anchored on).
    pub primary_environment_lane: M5TestEnvironmentLane,
    /// The compared environment legs (must have at least two — a card compares environments).
    pub legs: Vec<M5EnvironmentCompatibilityLeg>,
    /// The opaque stable card identity (must be non-empty).
    pub card_identity_ref: String,
}

/// The resolved environment-matrix-card truth for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEnvironmentMatrixCard {
    /// The test target class.
    pub target_class: M5TestTargetClass,
    /// The primary environment lane.
    pub primary_environment_lane: M5TestEnvironmentLane,
    /// The compared environment legs, preserved exactly from the input.
    pub legs: Vec<M5EnvironmentCompatibilityLeg>,
    /// The number of compared legs.
    pub leg_count: u32,
    /// The opaque stable card identity, preserved exactly from the input.
    pub card_identity_ref: String,
    /// The derived card posture.
    pub card_posture: M5EnvironmentCardPosture,
    /// The overall (worst) compatibility across every axis of every leg.
    pub overall_compatibility: M5EnvCompatibilityClass,
    /// The bounded actions this card offers.
    pub available_actions: Vec<M5EnvironmentCardAction>,
    /// True when any leg has an incompatible axis.
    pub has_incompatible_leg: bool,
    /// True when any leg has an unverified axis.
    pub has_unverified_axis: bool,
    /// Hard truth: the card never asserts safe equivalence across environments. Always `false`.
    pub asserts_safe_equivalence: bool,
    /// True when the card must warn about incompatibility (any non-compatible posture).
    pub warns_on_incompatibility: bool,
    /// True when at least one leg can actually be rerun on.
    pub can_rerun_on_leg: bool,
    /// True when the card still needs compatibility attention (not a fully compatible matrix).
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_environment_matrix_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5EnvironmentCardResolutionError {
    /// Fewer than two legs were supplied — a card must compare environments.
    InsufficientComparisonLegs,
    /// A leg label was empty.
    EmptyLegLabel,
    /// The card identity ref was empty.
    EmptyCardIdentity,
    /// An environment descriptor carried forbidden material.
    ForbiddenEnvironmentMaterial,
}

impl M5EnvironmentCardResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::InsufficientComparisonLegs => "insufficient_comparison_legs",
            Self::EmptyLegLabel => "empty_leg_label",
            Self::EmptyCardIdentity => "empty_card_identity",
            Self::ForbiddenEnvironmentMaterial => "forbidden_environment_material",
        }
    }
}

impl fmt::Display for M5EnvironmentCardResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "environment matrix card resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5EnvironmentCardResolutionError {}

/// Resolves one environment-matrix card from its declared compatibility legs.
///
/// The overall compatibility is the worst axis across every leg, and the derived card posture is
/// incompatible before unverified before mixed before compatible, so a card is only claimed
/// compatible when every applicable axis is fully compatible. The card never asserts safe
/// equivalence: `asserts_safe_equivalence` is always `false`, and any non-compatible posture
/// raises `warns_on_incompatibility`. A card must compare at least two environments.
pub fn resolve_environment_matrix_card(
    input: &M5EnvironmentCardResolutionInput,
) -> Result<M5ResolvedEnvironmentMatrixCard, M5EnvironmentCardResolutionError> {
    if input.legs.len() < 2 {
        return Err(M5EnvironmentCardResolutionError::InsufficientComparisonLegs);
    }
    if input.legs.iter().any(|leg| leg.leg_label.trim().is_empty()) {
        return Err(M5EnvironmentCardResolutionError::EmptyLegLabel);
    }
    if input.card_identity_ref.trim().is_empty() {
        return Err(M5EnvironmentCardResolutionError::EmptyCardIdentity);
    }
    if value_repr_is_forbidden(&input.card_identity_ref)
        || input
            .legs
            .iter()
            .any(|leg| value_repr_is_forbidden(&leg.leg_label))
    {
        return Err(M5EnvironmentCardResolutionError::ForbiddenEnvironmentMaterial);
    }

    let overall_compatibility = input
        .legs
        .iter()
        .map(M5EnvironmentCompatibilityLeg::worst_axis)
        .min_by_key(|axis| axis.severity_rank())
        .unwrap_or(M5EnvCompatibilityClass::NotApplicable);
    let card_posture = derive_environment_posture(overall_compatibility);
    let has_incompatible_leg = input.legs.iter().any(|leg| {
        leg.axes()
            .iter()
            .any(|axis| matches!(axis, M5EnvCompatibilityClass::Incompatible))
    });
    let has_unverified_axis = input.legs.iter().any(|leg| {
        leg.axes()
            .iter()
            .any(|axis| matches!(axis, M5EnvCompatibilityClass::Unverified))
    });
    let can_rerun_on_leg = input
        .legs
        .iter()
        .any(|leg| leg.target_compatibility.is_runnable());
    let available_actions = derive_environment_actions(can_rerun_on_leg);

    Ok(M5ResolvedEnvironmentMatrixCard {
        target_class: input.target_class,
        primary_environment_lane: input.primary_environment_lane,
        legs: input.legs.clone(),
        leg_count: input.legs.len() as u32,
        card_identity_ref: input.card_identity_ref.clone(),
        card_posture,
        overall_compatibility,
        available_actions,
        has_incompatible_leg,
        has_unverified_axis,
        asserts_safe_equivalence: false,
        warns_on_incompatibility: !card_posture.is_compatible(),
        can_rerun_on_leg,
        needs_attention: !card_posture.is_compatible(),
    })
}

/// The overall-compatibility → card-posture map.
fn derive_environment_posture(overall: M5EnvCompatibilityClass) -> M5EnvironmentCardPosture {
    match overall {
        M5EnvCompatibilityClass::Incompatible => M5EnvironmentCardPosture::IncompatibleMatrix,
        M5EnvCompatibilityClass::Unverified => M5EnvironmentCardPosture::UnverifiedMatrix,
        M5EnvCompatibilityClass::PartiallyCompatible => M5EnvironmentCardPosture::MixedMatrix,
        M5EnvCompatibilityClass::FullyCompatible | M5EnvCompatibilityClass::NotApplicable => {
            M5EnvironmentCardPosture::CompatibleMatrix
        }
    }
}

/// Derives the bounded environment-action set from the rerun signal.
fn derive_environment_actions(can_rerun_on_leg: bool) -> Vec<M5EnvironmentCardAction> {
    use M5EnvironmentCardAction as Action;
    let mut actions = vec![Action::RevealCompatibilityDetails, Action::InspectLegDeltas];
    if can_rerun_on_leg {
        actions.push(Action::RerunOnLeg);
    }
    actions.push(Action::ExportEnvironmentMatrix);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked failure-triage-panel resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TriagePanelResolutionCase {
    /// The resolver input.
    pub input: M5TriagePanelResolutionInput,
    /// The resolved truth. Must equal `resolve_failure_triage_panel(&input)`.
    pub resolved: M5ResolvedFailureTriagePanel,
}

impl M5TriagePanelResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5TriagePanelResolutionInput) -> Self {
        let resolved = resolve_failure_triage_panel(&input).expect("seed triage case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_failure_triage_panel(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved panel identity preserves the input identity and attempts exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.panel_identity_ref == self.input.panel_identity_ref
            && self.resolved.assertion_summary_label == self.input.assertion_summary_label
            && self.resolved.recent_attempts == self.input.recent_attempts
    }
}

/// One worked quarantine-review-sheet resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QuarantineReviewResolutionCase {
    /// The resolver input.
    pub input: M5QuarantineReviewResolutionInput,
    /// The resolved truth. Must equal `resolve_quarantine_review_sheet(&input)`.
    pub resolved: M5ResolvedQuarantineReviewSheet,
}

impl M5QuarantineReviewResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5QuarantineReviewResolutionInput) -> Self {
        let resolved =
            resolve_quarantine_review_sheet(&input).expect("seed quarantine case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_quarantine_review_sheet(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved sheet identity preserves the input identity, reason, and owner.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.sheet_identity_ref == self.input.sheet_identity_ref
            && self.resolved.reason_label == self.input.reason_label
            && self.resolved.owner_label == self.input.owner_label
    }
}

/// One worked environment-matrix-card resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvironmentCardResolutionCase {
    /// The resolver input.
    pub input: M5EnvironmentCardResolutionInput,
    /// The resolved truth. Must equal `resolve_environment_matrix_card(&input)`.
    pub resolved: M5ResolvedEnvironmentMatrixCard,
}

impl M5EnvironmentCardResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5EnvironmentCardResolutionInput) -> Self {
        let resolved =
            resolve_environment_matrix_card(&input).expect("seed environment case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_environment_matrix_card(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved card identity and legs preserve the input exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.card_identity_ref == self.input.card_identity_ref
            && self.resolved.legs == self.input.legs
    }
}

/// One row in the primitive matrix: one quality surface bound to the shared triage, quarantine,
/// and environment anatomy, vocabulary, postures, bounded actions, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualityTriageConsumerRow {
    /// Quality surface family.
    pub consumer_surface: M5QualityTriageConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5TestQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 test surface families that render / consume these components.
    pub surface_families: Vec<M5TestSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5TestDeploymentLine>,
    /// Triage-panel anatomy parts this consumer renders (must include the mandatory parts).
    pub triage_anatomy_parts: Vec<M5TriagePanelAnatomyPart>,
    /// Quarantine-sheet anatomy parts this consumer renders (must include the mandatory parts).
    pub quarantine_anatomy_parts: Vec<M5QuarantineReviewAnatomyPart>,
    /// Environment-card anatomy parts this consumer renders (must include the mandatory parts).
    pub environment_anatomy_parts: Vec<M5EnvironmentCardAnatomyPart>,
    /// Failure categories this consumer distinguishes.
    pub failure_categories: Vec<M5FailureCategory>,
    /// Triage dispositions this consumer distinguishes.
    pub triage_dispositions: Vec<M5TriageDisposition>,
    /// Classifier confidences this consumer distinguishes.
    pub classifier_confidences: Vec<M5ClassifierConfidence>,
    /// Triage postures this consumer distinguishes.
    pub triage_postures: Vec<M5TriagePanelPosture>,
    /// Suppression kinds this consumer distinguishes.
    pub suppression_kinds: Vec<M5SuppressionKind>,
    /// Suppression scopes this consumer distinguishes.
    pub suppression_scopes: Vec<M5SuppressionScope>,
    /// Quarantine ownership classes this consumer distinguishes.
    pub quarantine_ownership_classes: Vec<M5QuarantineOwnership>,
    /// Release impacts this consumer distinguishes.
    pub release_impacts: Vec<M5TestReleaseImpact>,
    /// Expiry states this consumer distinguishes.
    pub expiry_states: Vec<M5QuarantineExpiry>,
    /// Quarantine review postures this consumer distinguishes.
    pub quarantine_postures: Vec<M5QuarantineReviewPosture>,
    /// Test target classes this consumer distinguishes.
    pub target_classes: Vec<M5TestTargetClass>,
    /// Environment lanes this consumer distinguishes.
    pub environment_lanes: Vec<M5TestEnvironmentLane>,
    /// Compatibility classes this consumer distinguishes.
    pub compatibility_classes: Vec<M5EnvCompatibilityClass>,
    /// Environment card postures this consumer distinguishes.
    pub environment_postures: Vec<M5EnvironmentCardPosture>,
    /// Attempt lineage kinds this consumer distinguishes.
    pub attempt_lineage_kinds: Vec<M5AttemptLineageKind>,
    /// Result origins this consumer distinguishes.
    pub result_origins: Vec<M5TestResultOrigin>,
    /// Bounded triage actions this consumer offers.
    pub triage_actions: Vec<M5TriagePanelAction>,
    /// Bounded quarantine actions this consumer offers.
    pub quarantine_actions: Vec<M5QuarantineReviewAction>,
    /// Bounded environment actions this consumer offers.
    pub environment_actions: Vec<M5EnvironmentCardAction>,
    /// Triage export fields this consumer carries (must include the mandatory fields).
    pub triage_export_fields: Vec<M5TriagePanelExportField>,
    /// Quarantine export fields this consumer carries (must include the mandatory fields).
    pub quarantine_export_fields: Vec<M5QuarantineReviewExportField>,
    /// Environment export fields this consumer carries (must include the mandatory fields).
    pub environment_export_fields: Vec<M5EnvironmentCardExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5TestAccessibilityRoute>,
    /// Test / triage subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5TestConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5TestDowngradeTrigger>,
    /// Proof packet refs that keep these components current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by these components.
    pub source_contract_refs: Vec<String>,
    /// Worked triage-panel resolutions proving the resolver on this consumer.
    pub triage_examples: Vec<M5TriagePanelResolutionCase>,
    /// Worked quarantine-review resolutions proving the resolver on this consumer.
    pub quarantine_examples: Vec<M5QuarantineReviewResolutionCase>,
    /// Worked environment-card resolutions proving the resolver on this consumer.
    pub environment_examples: Vec<M5EnvironmentCardResolutionCase>,
    /// Hard invariant: this consumer never offers a destructive suppression path without evidence
    /// context. MUST be `false`.
    pub offers_suppression_without_evidence: bool,
    /// Hard invariant: this consumer never hides a suppression's owner, expiry, or release
    /// impact. MUST be `false`.
    pub hides_owner_expiry_or_release_impact: bool,
    /// Hard invariant: this consumer never implies safe equivalence across incompatible
    /// environments. MUST be `false`.
    pub implies_safe_environment_equivalence: bool,
    /// Hard invariant: this consumer never drops the recent attempt sequence or the deltas.
    /// MUST be `false`.
    pub drops_recent_attempts_or_deltas: bool,
}

impl M5QualityTriageConsumerRow {
    fn declares_mandatory_triage_anatomy(&self) -> bool {
        let present: BTreeSet<M5TriagePanelAnatomyPart> =
            self.triage_anatomy_parts.iter().copied().collect();
        M5TriagePanelAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_quarantine_anatomy(&self) -> bool {
        let present: BTreeSet<M5QuarantineReviewAnatomyPart> =
            self.quarantine_anatomy_parts.iter().copied().collect();
        M5QuarantineReviewAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_environment_anatomy(&self) -> bool {
        let present: BTreeSet<M5EnvironmentCardAnatomyPart> =
            self.environment_anatomy_parts.iter().copied().collect();
        M5EnvironmentCardAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_triage_export(&self) -> bool {
        let present: BTreeSet<M5TriagePanelExportField> =
            self.triage_export_fields.iter().copied().collect();
        M5TriagePanelExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn declares_mandatory_quarantine_export(&self) -> bool {
        let present: BTreeSet<M5QuarantineReviewExportField> =
            self.quarantine_export_fields.iter().copied().collect();
        M5QuarantineReviewExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn declares_mandatory_environment_export(&self) -> bool {
        let present: BTreeSet<M5EnvironmentCardExportField> =
            self.environment_export_fields.iter().copied().collect();
        M5EnvironmentCardExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.offers_suppression_without_evidence
            && !self.hides_owner_expiry_or_release_impact
            && !self.implies_safe_environment_equivalence
            && !self.drops_recent_attempts_or_deltas
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualityTriageVocabularySet {
    /// Quality-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Triage-anatomy-part tokens.
    pub triage_anatomy_parts: Vec<String>,
    /// Quarantine-anatomy-part tokens.
    pub quarantine_anatomy_parts: Vec<String>,
    /// Environment-anatomy-part tokens.
    pub environment_anatomy_parts: Vec<String>,
    /// Classifier-confidence tokens.
    pub classifier_confidences: Vec<String>,
    /// Triage-posture tokens.
    pub triage_postures: Vec<String>,
    /// Suppression-kind tokens.
    pub suppression_kinds: Vec<String>,
    /// Suppression-scope tokens.
    pub suppression_scopes: Vec<String>,
    /// Expiry-state tokens.
    pub expiry_states: Vec<String>,
    /// Quarantine-posture tokens.
    pub quarantine_postures: Vec<String>,
    /// Compatibility-class tokens.
    pub compatibility_classes: Vec<String>,
    /// Environment-posture tokens.
    pub environment_postures: Vec<String>,
    /// Triage-action tokens.
    pub triage_actions: Vec<String>,
    /// Quarantine-action tokens.
    pub quarantine_actions: Vec<String>,
    /// Environment-action tokens.
    pub environment_actions: Vec<String>,
    /// Triage-export-field tokens.
    pub triage_export_fields: Vec<String>,
    /// Quarantine-export-field tokens.
    pub quarantine_export_fields: Vec<String>,
    /// Environment-export-field tokens.
    pub environment_export_fields: Vec<String>,
    /// Failure-category tokens (reused from the frozen matrix).
    pub failure_categories: Vec<String>,
    /// Triage-disposition tokens (reused from the frozen matrix).
    pub triage_dispositions: Vec<String>,
    /// Quarantine-ownership tokens (reused from the frozen matrix).
    pub quarantine_ownership_classes: Vec<String>,
    /// Release-impact tokens (reused from the frozen matrix).
    pub release_impacts: Vec<String>,
    /// Target-class tokens (reused from the frozen matrix).
    pub target_classes: Vec<String>,
    /// Environment-lane tokens (reused from the frozen matrix).
    pub environment_lanes: Vec<String>,
    /// Attempt-lineage tokens (reused from the frozen matrix).
    pub attempt_lineage_kinds: Vec<String>,
    /// Result-origin tokens (reused from the frozen matrix).
    pub result_origins: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5QualityTriageVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5QualityTriageConsumerSurface::ALL, |v| v.as_str()),
            triage_anatomy_parts: tokens(&M5TriagePanelAnatomyPart::ALL, |v| v.as_str()),
            quarantine_anatomy_parts: tokens(&M5QuarantineReviewAnatomyPart::ALL, |v| v.as_str()),
            environment_anatomy_parts: tokens(&M5EnvironmentCardAnatomyPart::ALL, |v| v.as_str()),
            classifier_confidences: tokens(&M5ClassifierConfidence::ALL, |v| v.as_str()),
            triage_postures: tokens(&M5TriagePanelPosture::ALL, |v| v.as_str()),
            suppression_kinds: tokens(&M5SuppressionKind::ALL, |v| v.as_str()),
            suppression_scopes: tokens(&M5SuppressionScope::ALL, |v| v.as_str()),
            expiry_states: tokens(&M5QuarantineExpiry::ALL, |v| v.as_str()),
            quarantine_postures: tokens(&M5QuarantineReviewPosture::ALL, |v| v.as_str()),
            compatibility_classes: tokens(&M5EnvCompatibilityClass::ALL, |v| v.as_str()),
            environment_postures: tokens(&M5EnvironmentCardPosture::ALL, |v| v.as_str()),
            triage_actions: tokens(&M5TriagePanelAction::ALL, |v| v.as_str()),
            quarantine_actions: tokens(&M5QuarantineReviewAction::ALL, |v| v.as_str()),
            environment_actions: tokens(&M5EnvironmentCardAction::ALL, |v| v.as_str()),
            triage_export_fields: tokens(&M5TriagePanelExportField::ALL, |v| v.as_str()),
            quarantine_export_fields: tokens(&M5QuarantineReviewExportField::ALL, |v| v.as_str()),
            environment_export_fields: tokens(&M5EnvironmentCardExportField::ALL, |v| v.as_str()),
            failure_categories: tokens(&M5FailureCategory::ALL, |v| v.as_str()),
            triage_dispositions: tokens(&M5TriageDisposition::ALL, |v| v.as_str()),
            quarantine_ownership_classes: tokens(&M5QuarantineOwnership::ALL, |v| v.as_str()),
            release_impacts: tokens(&M5TestReleaseImpact::ALL, |v| v.as_str()),
            target_classes: tokens(&M5TestTargetClass::ALL, |v| v.as_str()),
            environment_lanes: tokens(&M5TestEnvironmentLane::ALL, |v| v.as_str()),
            attempt_lineage_kinds: tokens(&M5AttemptLineageKind::ALL, |v| v.as_str()),
            result_origins: tokens(&M5TestResultOrigin::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TestSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TestDeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TestAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5QualityTriageGovernanceReview {
    /// The triage panel shows its assertion/diff summary and recent attempt sequence.
    pub panel_shows_assertion_and_recent_attempts: bool,
    /// The triage panel shows its environment/build/runtime deltas and classifier confidence.
    pub panel_shows_deltas_and_confidence: bool,
    /// The triage panel offers rerun, debug, and open-review actions.
    pub panel_offers_rerun_debug_review: bool,
    /// No red row jumps to a destructive suppression without evidence context.
    pub no_suppression_without_evidence_context: bool,
    /// The quarantine sheet preserves scope, reason, owner, expiry, linked artifacts, and impact.
    pub sheet_preserves_scope_reason_owner_expiry_artifacts_impact: bool,
    /// The quarantine sheet keeps the suppressed test visible instead of hiding it.
    pub sheet_keeps_suppressed_test_visible: bool,
    /// The quarantine sheet always offers a restore action.
    pub sheet_offers_restore_action: bool,
    /// The environment card compares target/runtime/toolchain/build compatibility classes.
    pub card_compares_target_runtime_toolchain_build: bool,
    /// The environment card never implies safe equivalence across incompatible environments.
    pub card_never_implies_safe_equivalence: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The components keep the same truth across every quality surface.
    pub components_stable_across_consumer_surfaces: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// The support / export packet reconstructs triage, suppression, and environment truth.
    pub support_export_reconstructs_quality_truth: bool,
    /// Later M5 quality components cannot invent parallel triage/suppression/environment vocab.
    pub later_components_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualityTriageConsumerProjection {
    /// Tree and triage surfaces consume the shared triage/quarantine/environment vocabulary.
    pub tree_and_triage_surfaces_consume_quality_vocabulary: bool,
    /// The triage-posture resolver reads a single canonical source.
    pub triage_posture_reads_single_source: bool,
    /// The quarantine-posture resolver reads a single canonical source.
    pub quarantine_posture_reads_single_source: bool,
    /// The environment-posture resolver reads a single canonical source.
    pub environment_posture_reads_single_source: bool,
    /// Headless and desktop quality surfaces read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualityTriageProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the three triage components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualityTriageReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting test-evidence audit.
    pub test_evidence_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5QualityTriageStatusPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5QualityTriageStatusPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Quality surface rows.
    pub rows: Vec<M5QualityTriageConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5QualityTriageVocabularySet,
    /// Governance-review block.
    pub governance_review: M5QualityTriageGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5QualityTriageConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5QualityTriageProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5QualityTriageReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 failure-triage / quarantine-review / environment-matrix primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5QualityTriageStatusPacket {
    /// Record kind; must equal [`M5_QUALITY_TRIAGE_STATUS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_QUALITY_TRIAGE_STATUS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Quality surface rows.
    pub rows: Vec<M5QualityTriageConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5QualityTriageVocabularySet,
    /// Governance-review block.
    pub governance_review: M5QualityTriageGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5QualityTriageConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5QualityTriageProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5QualityTriageReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5QualityTriageStatusPacket {
    /// Builds an M5 quality-triage-primitive packet from stable-lane input.
    pub fn new(input: M5QualityTriageStatusPacketInput) -> Self {
        Self {
            record_kind: M5_QUALITY_TRIAGE_STATUS_RECORD_KIND.to_owned(),
            schema_version: M5_QUALITY_TRIAGE_STATUS_SCHEMA_VERSION,
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

    /// Validates the M5 quality-triage-primitive invariants.
    pub fn validate(&self) -> Vec<M5QualityTriageViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_QUALITY_TRIAGE_STATUS_RECORD_KIND {
            violations.push(M5QualityTriageViolation::WrongRecordKind);
        }
        if self.schema_version != M5_QUALITY_TRIAGE_STATUS_SCHEMA_VERSION {
            violations.push(M5QualityTriageViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5QualityTriageViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_triage_posture_coverage(self, &mut violations);
        validate_quarantine_posture_coverage(self, &mut violations);
        validate_environment_posture_coverage(self, &mut violations);
        validate_confidence_coverage(self, &mut violations);
        validate_evidence_context_coverage(self, &mut violations);
        validate_owner_expiry_release_coverage(self, &mut violations);
        validate_safe_equivalence_coverage(self, &mut violations);
        validate_recent_attempts_preservation(self, &mut violations);
        validate_restore_action_coverage(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 quality triage status packet serializes"),
        ) {
            violations.push(M5QualityTriageViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 quality triage status packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per quality surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,triage_postures,quarantine_postures,environment_postures,triage_actions,quarantine_actions,environment_actions,triage_examples,quarantine_examples,environment_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.triage_postures, |v| v.as_str()),
                join_tokens(&row.quarantine_postures, |v| v.as_str()),
                join_tokens(&row.environment_postures, |v| v.as_str()),
                join_tokens(&row.triage_actions, |v| v.as_str()),
                join_tokens(&row.quarantine_actions, |v| v.as_str()),
                join_tokens(&row.environment_actions, |v| v.as_str()),
                row.triage_examples.len(),
                row.quarantine_examples.len(),
                row.environment_examples.len(),
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
        out.push_str(
            "# M5 Failure-Triage-Panel / Quarantine-Review-Sheet / Environment-Matrix-Card Primitive\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Quality surfaces: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Triage postures: {}\n",
            self.vocabulary_set.triage_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Quarantine postures: {}\n",
            self.vocabulary_set.quarantine_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Environment postures: {}\n",
            self.vocabulary_set.environment_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Quality surfaces\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked triage: {} / quarantine: {} / environment: {}\n",
                row.triage_examples.len(),
                row.quarantine_examples.len(),
                row.environment_examples.len(),
            ));
            for case in &row.triage_examples {
                out.push_str(&format!(
                    "    - triage `{}` (`{}`) → `{}` (evidence `{}`, open-review `{}`)\n",
                    case.resolved.panel_identity_ref,
                    case.resolved.failure_category.as_str(),
                    case.resolved.triage_posture.as_str(),
                    case.resolved.provides_evidence_context,
                    case.resolved.can_open_review,
                ));
            }
            for case in &row.quarantine_examples {
                out.push_str(&format!(
                    "    - quarantine `{}` (`{}`) → `{}` (visible `{}`, restore `{}`)\n",
                    case.resolved.sheet_identity_ref,
                    case.resolved.ownership.as_str(),
                    case.resolved.review_posture.as_str(),
                    case.resolved.stays_visible,
                    case.resolved.can_restore,
                ));
            }
            for case in &row.environment_examples {
                out.push_str(&format!(
                    "    - environment `{}` (`{}`) → `{}` (incompatible-leg `{}`, safe-equivalence `{}`)\n",
                    case.resolved.card_identity_ref,
                    case.resolved.overall_compatibility.as_str(),
                    case.resolved.card_posture.as_str(),
                    case.resolved.has_incompatible_leg,
                    case.resolved.asserts_safe_equivalence,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 quality-triage-primitive export.
#[derive(Debug)]
pub enum M5QualityTriageArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5QualityTriageViolation>),
}

impl fmt::Display for M5QualityTriageArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 quality triage status export parse failed: {error}"
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
                    "m5 quality triage status export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5QualityTriageArtifactError {}

/// Validation failures emitted by [`M5QualityTriageStatusPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5QualityTriageViolation {
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
    /// A required quality surface family is missing from the matrix.
    RequiredConsumerMissing,
    /// A quality surface row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory triage anatomy parts.
    MandatoryTriageAnatomyMissing,
    /// A row omits one of the mandatory quarantine anatomy parts.
    MandatoryQuarantineAnatomyMissing,
    /// A row omits one of the mandatory environment anatomy parts.
    MandatoryEnvironmentAnatomyMissing,
    /// A row omits one of the mandatory triage export fields.
    MandatoryTriageExportMissing,
    /// A row omits one of the mandatory quarantine export fields.
    MandatoryQuarantineExportMissing,
    /// A row omits one of the mandatory environment export fields.
    MandatoryEnvironmentExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked triage, quarantine, or environment resolutions.
    ExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every triage posture.
    TriagePostureCoverageUnproven,
    /// The worked resolutions do not exercise every quarantine posture.
    QuarantinePostureCoverageUnproven,
    /// The worked resolutions do not exercise every environment posture.
    EnvironmentPostureCoverageUnproven,
    /// The worked resolutions do not exercise every classifier confidence.
    ConfidenceCoverageUnproven,
    /// A worked triage resolution reaches open-review without evidence context (AC1).
    EvidenceContextUnproven,
    /// A worked quarantine resolution hides the suppression or drops owner/expiry/impact (AC2).
    OwnerExpiryReleaseCoverageUnproven,
    /// The worked resolutions do not prove both an incompatible-matrix card that withholds safe
    /// equivalence and a compatible one (AC3).
    SafeEquivalenceCoverageUnproven,
    /// A worked triage resolution drops its recent attempt sequence.
    RecentAttemptsPreservationUnproven,
    /// A worked quarantine resolution omits the always-present restore action.
    RestoreActionCoverageUnproven,
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

impl M5QualityTriageViolation {
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
            Self::MandatoryTriageAnatomyMissing => "mandatory_triage_anatomy_missing",
            Self::MandatoryQuarantineAnatomyMissing => "mandatory_quarantine_anatomy_missing",
            Self::MandatoryEnvironmentAnatomyMissing => "mandatory_environment_anatomy_missing",
            Self::MandatoryTriageExportMissing => "mandatory_triage_export_missing",
            Self::MandatoryQuarantineExportMissing => "mandatory_quarantine_export_missing",
            Self::MandatoryEnvironmentExportMissing => "mandatory_environment_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleMissing => "example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::TriagePostureCoverageUnproven => "triage_posture_coverage_unproven",
            Self::QuarantinePostureCoverageUnproven => "quarantine_posture_coverage_unproven",
            Self::EnvironmentPostureCoverageUnproven => "environment_posture_coverage_unproven",
            Self::ConfidenceCoverageUnproven => "confidence_coverage_unproven",
            Self::EvidenceContextUnproven => "evidence_context_unproven",
            Self::OwnerExpiryReleaseCoverageUnproven => "owner_expiry_release_coverage_unproven",
            Self::SafeEquivalenceCoverageUnproven => "safe_equivalence_coverage_unproven",
            Self::RecentAttemptsPreservationUnproven => "recent_attempts_preservation_unproven",
            Self::RestoreActionCoverageUnproven => "restore_action_coverage_unproven",
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

/// Reads and validates the checked-in stable M5 quality-triage-primitive export.
pub fn current_stable_m5_quality_triage_status_export(
) -> Result<M5QualityTriageStatusPacket, M5QualityTriageArtifactError> {
    let packet: M5QualityTriageStatusPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-failure-triage-quarantine-environment-primitive-proof/support_export.json"
    )))
    .map_err(M5QualityTriageArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5QualityTriageArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_QUALITY_TRIAGE_STATUS_TRIAGE_SCHEMA_REF,
        M5_QUALITY_TRIAGE_STATUS_QUARANTINE_SCHEMA_REF,
        M5_QUALITY_TRIAGE_STATUS_ENVIRONMENT_SCHEMA_REF,
        M5_QUALITY_TRIAGE_STATUS_DOC_REF,
        M5_QUALITY_TRIAGE_STATUS_COMPONENT_MATRIX_REF,
        M5_QUALITY_TRIAGE_STATUS_QUARANTINE_RECORD_REF,
        M5_QUALITY_TRIAGE_STATUS_RELEASE_VISIBILITY_REF,
        M5_QUALITY_TRIAGE_STATUS_ATTEMPT_RECORDS_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5QualityTriageViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5QualityTriageViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let present: BTreeSet<M5QualityTriageConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5QualityTriageConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5QualityTriageViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.triage_anatomy_parts.is_empty()
            || row.quarantine_anatomy_parts.is_empty()
            || row.environment_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.failure_categories.is_empty()
            || row.triage_dispositions.is_empty()
            || row.classifier_confidences.is_empty()
            || row.triage_postures.is_empty()
            || row.suppression_kinds.is_empty()
            || row.suppression_scopes.is_empty()
            || row.quarantine_ownership_classes.is_empty()
            || row.release_impacts.is_empty()
            || row.expiry_states.is_empty()
            || row.quarantine_postures.is_empty()
            || row.target_classes.is_empty()
            || row.environment_lanes.is_empty()
            || row.compatibility_classes.is_empty()
            || row.environment_postures.is_empty()
            || row.attempt_lineage_kinds.is_empty()
            || row.result_origins.is_empty()
            || row.triage_actions.is_empty()
            || row.quarantine_actions.is_empty()
            || row.environment_actions.is_empty()
            || row.triage_export_fields.is_empty()
            || row.quarantine_export_fields.is_empty()
            || row.environment_export_fields.is_empty()
        {
            violations.push(M5QualityTriageViolation::RowIncomplete);
        }
        if !row.declares_mandatory_triage_anatomy() {
            violations.push(M5QualityTriageViolation::MandatoryTriageAnatomyMissing);
        }
        if !row.declares_mandatory_quarantine_anatomy() {
            violations.push(M5QualityTriageViolation::MandatoryQuarantineAnatomyMissing);
        }
        if !row.declares_mandatory_environment_anatomy() {
            violations.push(M5QualityTriageViolation::MandatoryEnvironmentAnatomyMissing);
        }
        if !row.declares_mandatory_triage_export() {
            violations.push(M5QualityTriageViolation::MandatoryTriageExportMissing);
        }
        if !row.declares_mandatory_quarantine_export() {
            violations.push(M5QualityTriageViolation::MandatoryQuarantineExportMissing);
        }
        if !row.declares_mandatory_environment_export() {
            violations.push(M5QualityTriageViolation::MandatoryEnvironmentExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TestAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5QualityTriageViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5QualityTriageViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5QualityTriageViolation::DowngradeTriggersMissing);
        }
        if row.triage_examples.is_empty()
            || row.quarantine_examples.is_empty()
            || row.environment_examples.is_empty()
        {
            violations.push(M5QualityTriageViolation::ExampleMissing);
        }
        if row
            .triage_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .quarantine_examples
                .iter()
                .any(|case| !case.is_self_consistent())
            || row
                .environment_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5QualityTriageViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5QualityTriageViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5QualityTriageViolation::RowInvariantViolated);
        }
    }
}

/// Every triage posture must be exercised by some worked resolution — the proof that each
/// failure category gets a distinct triage treatment.
fn validate_triage_posture_coverage(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let exercised: BTreeSet<M5TriagePanelPosture> = packet
        .rows
        .iter()
        .flat_map(|row| row.triage_examples.iter())
        .map(|case| case.resolved.triage_posture)
        .collect();
    if !M5TriagePanelPosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture))
    {
        violations.push(M5QualityTriageViolation::TriagePostureCoverageUnproven);
    }
}

/// Every quarantine posture must be exercised by some worked resolution.
fn validate_quarantine_posture_coverage(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let exercised: BTreeSet<M5QuarantineReviewPosture> = packet
        .rows
        .iter()
        .flat_map(|row| row.quarantine_examples.iter())
        .map(|case| case.resolved.review_posture)
        .collect();
    if !M5QuarantineReviewPosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture))
    {
        violations.push(M5QualityTriageViolation::QuarantinePostureCoverageUnproven);
    }
}

/// Every environment posture must be exercised by some worked resolution.
fn validate_environment_posture_coverage(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let exercised: BTreeSet<M5EnvironmentCardPosture> = packet
        .rows
        .iter()
        .flat_map(|row| row.environment_examples.iter())
        .map(|case| case.resolved.card_posture)
        .collect();
    if !M5EnvironmentCardPosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture))
    {
        violations.push(M5QualityTriageViolation::EnvironmentPostureCoverageUnproven);
    }
}

/// Every classifier confidence must be exercised by some worked triage resolution.
fn validate_confidence_coverage(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let exercised: BTreeSet<M5ClassifierConfidence> = packet
        .rows
        .iter()
        .flat_map(|row| row.triage_examples.iter())
        .map(|case| case.resolved.classifier_confidence)
        .collect();
    if !M5ClassifierConfidence::ALL
        .iter()
        .all(|confidence| exercised.contains(confidence))
    {
        violations.push(M5QualityTriageViolation::ConfidenceCoverageUnproven);
    }
}

/// AC1: every worked triage resolution that can open the quarantine review must first provide
/// evidence context — no red row jumps to a destructive suppression without evidence.
fn validate_evidence_context_coverage(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let all_gated = packet
        .rows
        .iter()
        .flat_map(|row| row.triage_examples.iter())
        .all(|case| {
            let resolved = &case.resolved;
            // open-review is only ever offered together with evidence context.
            (!resolved.can_open_review || resolved.provides_evidence_context)
                && (resolved
                    .available_actions
                    .contains(&M5TriagePanelAction::OpenQuarantineReview)
                    == resolved.can_open_review)
        });
    // At least one example must actually prove the gated open-review path.
    let has_proven = packet.rows.iter().any(|row| {
        row.triage_examples.iter().any(|case| {
            case.resolved.provides_evidence_context
                && case
                    .resolved
                    .available_actions
                    .contains(&M5TriagePanelAction::OpenQuarantineReview)
        })
    });
    if !(all_gated && has_proven) {
        violations.push(M5QualityTriageViolation::EvidenceContextUnproven);
    }
}

/// AC2: every worked quarantine resolution keeps the test visible and preserves its reason, and
/// at least one proves a hidden-from-release suppression still staying visible with its impact
/// disclosed rather than disappearing into a filter.
fn validate_owner_expiry_release_coverage(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let all_visible = packet
        .rows
        .iter()
        .flat_map(|row| row.quarantine_examples.iter())
        .all(|case| case.resolved.stays_visible && case.resolved.preserves_reason);
    let hidden_still_visible = packet.rows.iter().any(|row| {
        row.quarantine_examples
            .iter()
            .any(|case| case.resolved.hidden_from_release && case.resolved.stays_visible)
    });
    if !(all_visible && hidden_still_visible) {
        violations.push(M5QualityTriageViolation::OwnerExpiryReleaseCoverageUnproven);
    }
}

/// AC3: at least one worked environment resolution proves an incompatible matrix that withholds
/// safe equivalence and warns, and at least one proves a fully compatible matrix.
fn validate_safe_equivalence_coverage(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let no_card_asserts_equivalence = packet
        .rows
        .iter()
        .flat_map(|row| row.environment_examples.iter())
        .all(|case| !case.resolved.asserts_safe_equivalence);
    let has_incompatible = packet.rows.iter().any(|row| {
        row.environment_examples.iter().any(|case| {
            case.resolved.has_incompatible_leg
                && case.resolved.warns_on_incompatibility
                && !case.resolved.asserts_safe_equivalence
        })
    });
    let has_compatible = packet.rows.iter().any(|row| {
        row.environment_examples
            .iter()
            .any(|case| case.resolved.card_posture.is_compatible())
    });
    if !(no_card_asserts_equivalence && has_incompatible && has_compatible) {
        violations.push(M5QualityTriageViolation::SafeEquivalenceCoverageUnproven);
    }
}

/// Every worked triage resolution must preserve a non-empty recent attempt sequence.
fn validate_recent_attempts_preservation(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.triage_examples.iter())
        .all(|case| {
            !case.resolved.recent_attempts.is_empty()
                && case.resolved.recent_attempt_count as usize
                    == case.resolved.recent_attempts.len()
        });
    if !preserved {
        violations.push(M5QualityTriageViolation::RecentAttemptsPreservationUnproven);
    }
}

/// Every worked quarantine resolution must carry the always-present restore action.
fn validate_restore_action_coverage(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let all_restore = packet
        .rows
        .iter()
        .flat_map(|row| row.quarantine_examples.iter())
        .all(|case| {
            case.resolved.can_restore
                && case
                    .resolved
                    .available_actions
                    .contains(&M5QuarantineReviewAction::RestoreTest)
        });
    if !all_restore {
        violations.push(M5QualityTriageViolation::RestoreActionCoverageUnproven);
    }
}

/// Every worked resolution must preserve its exact identity and label.
fn validate_identity_preservation(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let triage_ok = packet
        .rows
        .iter()
        .flat_map(|row| row.triage_examples.iter())
        .all(|case| case.preserves_identity());
    let quarantine_ok = packet
        .rows
        .iter()
        .flat_map(|row| row.quarantine_examples.iter())
        .all(|case| case.preserves_identity());
    let environment_ok = packet
        .rows
        .iter()
        .flat_map(|row| row.environment_examples.iter())
        .all(|case| case.preserves_identity());
    if !(triage_ok && quarantine_ok && environment_ok) {
        violations.push(M5QualityTriageViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.panel_shows_assertion_and_recent_attempts,
        review.panel_shows_deltas_and_confidence,
        review.panel_offers_rerun_debug_review,
        review.no_suppression_without_evidence_context,
        review.sheet_preserves_scope_reason_owner_expiry_artifacts_impact,
        review.sheet_keeps_suppressed_test_visible,
        review.sheet_offers_restore_action,
        review.card_compares_target_runtime_toolchain_build,
        review.card_never_implies_safe_equivalence,
        review.components_stable_across_deployment_lines,
        review.components_stable_across_consumer_surfaces,
        review.every_component_declares_accessibility_route,
        review.support_export_reconstructs_quality_truth,
        review.later_components_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5QualityTriageViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.tree_and_triage_surfaces_consume_quality_vocabulary,
        projection.triage_posture_reads_single_source,
        projection.quarantine_posture_reads_single_source,
        projection.environment_posture_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5QualityTriageViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5QualityTriageViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5QualityTriageStatusPacket,
    violations: &mut Vec<M5QualityTriageViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.test_evidence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5QualityTriageViolation::ReleasePostureIncomplete);
    }
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
