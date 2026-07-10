//! Two reusable M5 test-intelligence primitives — the flaky-state badge and the retry-history
//! row — so a flaky verdict stays proportional to its evidence instead of folklore and a
//! retry-history row keeps enough context to explain why the same test passed here and failed
//! there. A flaky-state badge always names its controlled classification, its classifier
//! confidence, the retry window it was measured over, the classifier source behind the verdict,
//! the last attempt outcome, and its mute / quarantine status; a retry-history row always names
//! its stable test identity, its recent attempt outcomes in order, its environment / build /
//! runtime deltas, its classifier confidence, the origin the attempt ran on, and a
//! rerun-or-open-logs path back to the raw attempt.
//!
//! Aureline's frozen test-intelligence component matrix
//! ([`crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix`])
//! names the flaky-state badge and the retry-history row as two governed component families and
//! freezes their controlled vocabulary — the flaky classifications, the flaky confidence
//! classes, the retry attempt outcomes, and the retry scope classes, plus the provenance
//! classes, surface families, deployment lines, consumer surfaces, accessibility routes,
//! qualification classes, and downgrade triggers. This module *implements* that contract as two
//! reusable resolvers so a user can tell — from the badge alone — whether a test is stable,
//! suspected flaky, reproduced flaky, stable again, manually muted, or of unknown state, how
//! confident the classifier is, how large the evidence window was, and whether the verdict is
//! muted or quarantined, and — from the retry-history row alone — whether the last attempt
//! passed first try, passed on retry, failed across all retries, errored, was skipped, or was
//! aborted, how the recent attempts diverged, and what environment / build / runtime deltas
//! explain the divergence. Above all, a single intermittent failure never masquerades as
//! reproduced flakiness without the required evidence window, and a retry-history row never
//! drops the context needed to explain divergent outcomes across local, remote, notebook, and
//! imported-CI attempts.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_flaky_state_badge`] — takes one badge's flaky classification, classifier
//!    confidence class, classifier source, provenance class, mute state, retry-window size,
//!    observed-failure count, last attempt outcome, opaque badge identity, and opaque test
//!    identity, and produces one [`M5ResolvedFlakyBadge`] carrying the derived flaky posture (a
//!    stable, suspected-flaky, reproduced-flaky, stable-again, manually-muted, or unknown-flaky
//!    badge — one distinct posture per classification), whether the evidence window is
//!    sufficient to support a reproduced verdict, whether the verdict is a confirmed flake,
//!    whether it is muted or quarantined, and the bounded reveal / open-retry-history / rerun /
//!    mute-or-quarantine / export actions. It refuses to resolve a reproduced-flaky verdict that
//!    lacks the required evidence window, so one intermittent failure can never visually
//!    masquerade as reproduced flakiness.
//! 2. [`resolve_retry_history_row`] — takes one row's last attempt outcome, its recent attempt
//!    outcomes in order, its retry scope class, its attempt origin, its classifier confidence
//!    class, its provenance class, its environment / build / runtime delta flags, an opaque
//!    stable test identity, and an opaque attempt-log ref, and produces one
//!    [`M5ResolvedRetryRow`] carrying the derived retry posture (a passed-first-try,
//!    passed-on-retry, failed-all-retries, errored, skipped, or aborted row — one distinct
//!    posture per outcome), whether the recent outcomes diverged, whether the row discloses its
//!    environment / build / runtime deltas, whether it is imported, whether its rerun widened
//!    the selection, and a durable rerun-or-open-logs path back to the raw attempt. It never
//!    drops the ordered outcome sequence needed to explain a divergence and never severs the
//!    open-logs continuity.
//!
//! A single parity matrix — [`M5FlakyRetryComponentsPacket`] — binds one row per claimed M5
//! quality consumer (the flaky dashboard, the editor / test-tree flaky badge, the retry-history
//! panel, the headless / CLI flaky-retry surface, and the flaky-retry export) to the shared
//! badge and row anatomy, the same flaky classifications, confidence classes, classifier
//! sources, mute states, retry outcomes, retry scope classes, attempt origins, flaky and retry
//! postures, bounded actions, export fields, and non-visual accessibility routes, so the flaky
//! and retry vocabulary stays identical across the dashboard, the editor, the retry panel,
//! CI / headless, and support consumers — the acceptance-criterion parity that keeps a flaky
//! verdict proportional to its evidence everywhere with one vocabulary.
//!
//! The flaky classification ([`M5FlakyClassification`]), flaky confidence class
//! ([`M5FlakyConfidenceClass`]), retry attempt outcome ([`M5RetryAttemptOutcome`]), retry scope
//! class ([`M5RetryScopeClass`]), provenance class ([`M5TestIntelligenceProvenanceClass`]),
//! surface family ([`M5TestIntelligenceSurfaceFamily`]), deployment line
//! ([`M5TestIntelligenceDeploymentLine`]), consumer surface ([`M5TestIntelligenceConsumerSurface`]),
//! accessibility route ([`M5TestIntelligenceAccessibilityRoute`]), qualification class
//! ([`M5TestIntelligenceQualificationClass`]), and downgrade trigger
//! ([`M5TestIntelligenceDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the two components
//! themselves: their quality consumers, the classifier source, the mute state, the attempt
//! origin, the two derived postures, the two bounded action sets, the two anatomies, and the two
//! export field sets. No M5 quality surface invents a second flaky-badge or retry-row grammar.
//!
//! Raw test payloads, pasted paths, credentials, and private endpoints stay outside the export
//! boundary; every badge identity, test identity, and attempt-log ref is carried only as an
//! opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_flaky_retry_components_editor_badge_beta_narrowed,
    seeded_m5_flaky_retry_components_flaky_dashboard_preview_narrowed,
    seeded_m5_flaky_retry_components_packet, M5_FLAKY_RETRY_COMPONENTS_PACKET_ID,
};

// The flaky classification, flaky confidence class, retry attempt outcome, retry scope class,
// provenance class, surface family, deployment line, consumer surface, accessibility route,
// qualification class, and downgrade triggers are frozen once, in the test-intelligence
// component matrix. These primitives reuse them verbatim so they never invent parallel flaky /
// retry vocabulary.
pub use crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix::{
    M5FlakyClassification, M5FlakyConfidenceClass, M5RetryAttemptOutcome, M5RetryScopeClass,
    M5TestIntelligenceAccessibilityRoute, M5TestIntelligenceConsumerSurface,
    M5TestIntelligenceDeploymentLine, M5TestIntelligenceDowngradeTrigger,
    M5TestIntelligenceProvenanceClass, M5TestIntelligenceQualificationClass,
    M5TestIntelligenceSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5FlakyRetryComponentsPacket`].
pub const M5_FLAKY_RETRY_COMPONENTS_RECORD_KIND: &str =
    "implement_m5_flaky_state_badges_and_retry_history_rows_with_controlled_verdict_vocabulary_classifier_confidence_retry_window_visibility_environment_drift_notes_and_rerun_or_open_logs_parity_across_claimed_m5_quality_surfaces";

/// Schema version for M5 flaky-state-badge / retry-history-row records.
pub const M5_FLAKY_RETRY_COMPONENTS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the flaky-state-badge boundary schema (the canonical packet schema).
pub const M5_FLAKY_RETRY_COMPONENTS_FLAKY_SCHEMA_REF: &str =
    "schemas/ui/m5-flaky-state-badge.schema.json";

/// Repo-relative path of the retry-history-row companion schema.
pub const M5_FLAKY_RETRY_COMPONENTS_RETRY_SCHEMA_REF: &str =
    "schemas/ui/m5-retry-history-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_FLAKY_RETRY_COMPONENTS_DOC_REF: &str = "docs/testing/m5_flaky_retry_primitive.md";

/// Repo-relative path of the frozen test-intelligence component matrix these primitives narrow
/// from.
pub const M5_FLAKY_RETRY_COMPONENTS_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-test-intelligence-component-matrix.schema.json";

/// Repo-relative path of the flaky-verdict contract the badge binds its classification /
/// confidence / evidence truth against.
pub const M5_FLAKY_RETRY_COMPONENTS_FLAKY_VERDICT_REF: &str =
    "schemas/testing/flaky_verdict.schema.json";

/// Repo-relative path of the test-attempt (retry-history) contract the row binds its
/// outcome / delta / origin truth against.
pub const M5_FLAKY_RETRY_COMPONENTS_TEST_ATTEMPT_REF: &str =
    "schemas/testing/test_attempt.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_FLAKY_RETRY_COMPONENTS_FIXTURE_DIR: &str = "fixtures/ui/m5-flaky-retry-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_FLAKY_RETRY_COMPONENTS_ARTIFACT_REF: &str =
    "artifacts/release/m5-flaky-retry-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_FLAKY_RETRY_COMPONENTS_CSV_REF: &str =
    "artifacts/release/m5-flaky-retry-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_FLAKY_RETRY_COMPONENTS_REPORT_REF: &str =
    "artifacts/design/m5-flaky-retry-primitive.md";

/// The minimum retry-window size and observed-failure count a reproduced-flaky verdict must be
/// measured over before it can be claimed as reproduced rather than suspected. Below this
/// window, a single intermittent failure cannot present with the authority of a reproduced
/// verdict.
pub const REQUIRED_REPRODUCED_WINDOW: u32 = 2;

/// The minimum number of observed failures within the window a reproduced-flaky verdict must
/// carry.
pub const REQUIRED_REPRODUCED_FAILURES: u32 = 2;

/// One claimed M5 quality consumer that renders the shared flaky-state badge and retry-history
/// row. These are the consumers the acceptance criteria name — the flaky dashboard, the editor
/// / test-tree flaky badge, the retry-history panel, the headless / CLI flaky-retry surface, and
/// the flaky-retry export — so the same flaky / retry grammar works across every claimed quality
/// surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FlakyRetryComponentConsumerSurface {
    /// The flaky-dashboard panel surface.
    FlakyDashboardPanel,
    /// The editor / test-tree flaky-badge surface.
    EditorTestTreeBadge,
    /// The retry-history panel surface.
    RetryHistoryPanel,
    /// The headless / CLI flaky-retry surface.
    HeadlessCliFlakyRetry,
    /// The flaky-retry export surface.
    FlakyRetryExport,
}

impl M5FlakyRetryComponentConsumerSurface {
    /// Every claimed quality consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FlakyDashboardPanel,
        Self::EditorTestTreeBadge,
        Self::RetryHistoryPanel,
        Self::HeadlessCliFlakyRetry,
        Self::FlakyRetryExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FlakyDashboardPanel => "flaky_dashboard_panel",
            Self::EditorTestTreeBadge => "editor_test_tree_badge",
            Self::RetryHistoryPanel => "retry_history_panel",
            Self::HeadlessCliFlakyRetry => "headless_cli_flaky_retry",
            Self::FlakyRetryExport => "flaky_retry_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FlakyDashboardPanel => "Flaky Dashboard Panel",
            Self::EditorTestTreeBadge => "Editor / Test-Tree Badge",
            Self::RetryHistoryPanel => "Retry History Panel",
            Self::HeadlessCliFlakyRetry => "Headless / CLI Flaky-Retry",
            Self::FlakyRetryExport => "Flaky-Retry Export",
        }
    }
}

/// Controlled classifier source — where the flaky verdict came from, so a heuristic guess never
/// presents with the authority of a reproduced statistical model or an imported CI history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FlakyClassifierSource {
    /// A local heuristic classifier.
    LocalHeuristic,
    /// A statistical flakiness model.
    StatisticalModel,
    /// An imported CI flakiness classifier.
    ImportedCiClassifier,
    /// A manual human annotation / override.
    ManualOverride,
    /// An unknown / unattributed classifier.
    UnknownClassifier,
}

impl M5FlakyClassifierSource {
    /// Every classifier source, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalHeuristic,
        Self::StatisticalModel,
        Self::ImportedCiClassifier,
        Self::ManualOverride,
        Self::UnknownClassifier,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHeuristic => "local_heuristic",
            Self::StatisticalModel => "statistical_model",
            Self::ImportedCiClassifier => "imported_ci_classifier",
            Self::ManualOverride => "manual_override",
            Self::UnknownClassifier => "unknown_classifier",
        }
    }
}

/// Controlled mute / quarantine status a badge shows, so a muted or quarantined verdict is never
/// left implicit and an expired quarantine never silently keeps suppressing a failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FlakyMuteState {
    /// The verdict is not muted.
    NotMuted,
    /// The verdict is manually muted.
    MutedManual,
    /// The verdict is quarantined and the quarantine is active.
    QuarantineActive,
    /// The verdict was quarantined but the quarantine has expired.
    QuarantineExpired,
    /// Muting is blocked by policy.
    PolicyBlocked,
}

impl M5FlakyMuteState {
    /// Every mute state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotMuted,
        Self::MutedManual,
        Self::QuarantineActive,
        Self::QuarantineExpired,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotMuted => "not_muted",
            Self::MutedManual => "muted_manual",
            Self::QuarantineActive => "quarantine_active",
            Self::QuarantineExpired => "quarantine_expired",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// True when the verdict is currently muted or under an active quarantine.
    pub const fn is_muted(self) -> bool {
        matches!(self, Self::MutedManual | Self::QuarantineActive)
    }

    /// True when muting / quarantine may still be toggled (not policy-blocked).
    pub const fn can_toggle(self) -> bool {
        !matches!(self, Self::PolicyBlocked)
    }
}

/// The derived posture of a flaky-state badge — one distinct posture per flaky classification so
/// a suspected badge never borrows the authority of a reproduced one. Computed 1:1 from the
/// flaky classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FlakyBadgePosture {
    /// A stable badge.
    StableBadge,
    /// A suspected-flaky badge.
    SuspectedFlakyBadge,
    /// A reproduced-flaky badge.
    ReproducedFlakyBadge,
    /// A stable-again badge.
    StableAgainBadge,
    /// A manually-muted badge.
    ManuallyMutedBadge,
    /// An unknown-flaky badge.
    UnknownFlakyBadge,
}

impl M5FlakyBadgePosture {
    /// Every flaky posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StableBadge,
        Self::SuspectedFlakyBadge,
        Self::ReproducedFlakyBadge,
        Self::StableAgainBadge,
        Self::ManuallyMutedBadge,
        Self::UnknownFlakyBadge,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableBadge => "stable_badge",
            Self::SuspectedFlakyBadge => "suspected_flaky_badge",
            Self::ReproducedFlakyBadge => "reproduced_flaky_badge",
            Self::StableAgainBadge => "stable_again_badge",
            Self::ManuallyMutedBadge => "manually_muted_badge",
            Self::UnknownFlakyBadge => "unknown_flaky_badge",
        }
    }

    /// The frozen flaky classification this posture maps 1:1 to.
    pub const fn classification(self) -> M5FlakyClassification {
        match self {
            Self::StableBadge => M5FlakyClassification::Stable,
            Self::SuspectedFlakyBadge => M5FlakyClassification::SuspectedFlaky,
            Self::ReproducedFlakyBadge => M5FlakyClassification::ReproducedFlaky,
            Self::StableAgainBadge => M5FlakyClassification::StableAgain,
            Self::ManuallyMutedBadge => M5FlakyClassification::ManuallyMuted,
            Self::UnknownFlakyBadge => M5FlakyClassification::UnknownFlaky,
        }
    }

    /// True only for a reproduced-flaky badge — the only posture that names a confirmed flake.
    pub const fn is_confirmed_flaky(self) -> bool {
        matches!(self, Self::ReproducedFlakyBadge)
    }

    /// True when the badge is one an intermittent failure could be mistaken for without an
    /// evidence window (suspected, unknown).
    pub const fn is_unconfirmed_suspicion(self) -> bool {
        matches!(self, Self::SuspectedFlakyBadge | Self::UnknownFlakyBadge)
    }
}

/// One bounded action a flaky-state badge offers, so a badge never hides its reveal /
/// open-retry-history / rerun / mute-or-quarantine / export affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FlakyBadgeAction {
    /// Reveal the badge's classification, confidence, retry window, classifier source, last
    /// outcome, and mute status.
    RevealFlakyDetails,
    /// Open the retry history behind this verdict.
    OpenRetryHistory,
    /// Rerun the test to gather more evidence.
    RerunTest,
    /// Mute or quarantine (or unmute) the verdict.
    MuteOrQuarantine,
    /// Export the flaky badge as test evidence.
    ExportFlakyBadge,
}

impl M5FlakyBadgeAction {
    /// Every flaky action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealFlakyDetails,
        Self::OpenRetryHistory,
        Self::RerunTest,
        Self::MuteOrQuarantine,
        Self::ExportFlakyBadge,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealFlakyDetails => "reveal_flaky_details",
            Self::OpenRetryHistory => "open_retry_history",
            Self::RerunTest => "rerun_test",
            Self::MuteOrQuarantine => "mute_or_quarantine",
            Self::ExportFlakyBadge => "export_flaky_badge",
        }
    }
}

/// Controlled flaky-badge anatomy part. The parts in [`M5FlakyBadgeAnatomyPart::MANDATORY`] are
/// required on every badge so the classification, confidence, retry window, classifier source,
/// last outcome, and mute status are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FlakyBadgeAnatomyPart {
    /// The flaky-classification cue.
    ClassificationCue,
    /// The classifier-confidence cue.
    ConfidenceCue,
    /// The retry-window cue.
    RetryWindowCue,
    /// The classifier-source cue.
    ClassifierSourceCue,
    /// The last-outcome cue.
    LastOutcomeCue,
    /// The mute / quarantine status cue.
    MuteStatusCue,
    /// The provenance cue.
    ProvenanceCue,
    /// The badge-state cue.
    BadgeStateCue,
}

impl M5FlakyBadgeAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ClassificationCue,
        Self::ConfidenceCue,
        Self::RetryWindowCue,
        Self::ClassifierSourceCue,
        Self::LastOutcomeCue,
        Self::MuteStatusCue,
        Self::ProvenanceCue,
        Self::BadgeStateCue,
    ];

    /// The anatomy parts every flaky badge must render.
    pub const MANDATORY: [Self; 6] = [
        Self::ClassificationCue,
        Self::ConfidenceCue,
        Self::RetryWindowCue,
        Self::ClassifierSourceCue,
        Self::LastOutcomeCue,
        Self::MuteStatusCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClassificationCue => "classification_cue",
            Self::ConfidenceCue => "confidence_cue",
            Self::RetryWindowCue => "retry_window_cue",
            Self::ClassifierSourceCue => "classifier_source_cue",
            Self::LastOutcomeCue => "last_outcome_cue",
            Self::MuteStatusCue => "mute_status_cue",
            Self::ProvenanceCue => "provenance_cue",
            Self::BadgeStateCue => "badge_state_cue",
        }
    }
}

/// A field the flaky-badge export carries so badge truth is reconstructable. The fields in
/// [`M5FlakyBadgeExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FlakyBadgeExportField {
    /// The flaky classification.
    Classification,
    /// The flaky confidence class.
    ConfidenceClass,
    /// The classifier source.
    ClassifierSource,
    /// The provenance class.
    ProvenanceClass,
    /// The mute state.
    MuteState,
    /// The retry-window size.
    RetryWindowSize,
    /// The observed-failure count.
    ObservedFailures,
    /// The last attempt outcome.
    LastOutcome,
    /// The derived flaky posture.
    FlakyPosture,
}

impl M5FlakyBadgeExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Classification,
        Self::ConfidenceClass,
        Self::ClassifierSource,
        Self::ProvenanceClass,
        Self::MuteState,
        Self::RetryWindowSize,
        Self::ObservedFailures,
        Self::LastOutcome,
        Self::FlakyPosture,
    ];

    /// The export fields every flaky badge must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Classification,
        Self::ConfidenceClass,
        Self::ClassifierSource,
        Self::MuteState,
        Self::FlakyPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Classification => "classification",
            Self::ConfidenceClass => "confidence_class",
            Self::ClassifierSource => "classifier_source",
            Self::ProvenanceClass => "provenance_class",
            Self::MuteState => "mute_state",
            Self::RetryWindowSize => "retry_window_size",
            Self::ObservedFailures => "observed_failures",
            Self::LastOutcome => "last_outcome",
            Self::FlakyPosture => "flaky_posture",
        }
    }
}

/// Controlled attempt origin — where a retry attempt ran, so a retry-history row can explain why
/// the same test passed here and failed there across local, remote, notebook, and imported-CI
/// attempts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetryAttemptOrigin {
    /// A local attempt.
    LocalAttempt,
    /// A remote attempt.
    RemoteAttempt,
    /// A notebook attempt.
    NotebookAttempt,
    /// An imported CI attempt.
    ImportedCiAttempt,
    /// An unknown attempt origin.
    UnknownOrigin,
}

impl M5RetryAttemptOrigin {
    /// Every attempt origin, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalAttempt,
        Self::RemoteAttempt,
        Self::NotebookAttempt,
        Self::ImportedCiAttempt,
        Self::UnknownOrigin,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalAttempt => "local_attempt",
            Self::RemoteAttempt => "remote_attempt",
            Self::NotebookAttempt => "notebook_attempt",
            Self::ImportedCiAttempt => "imported_ci_attempt",
            Self::UnknownOrigin => "unknown_origin",
        }
    }

    /// True when the attempt is imported rather than run locally / remotely by Aureline.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedCiAttempt)
    }
}

/// The derived posture of a retry-history row — one distinct posture per retry attempt outcome,
/// so a pass-on-retry never reads as a clean first-try pass. Computed 1:1 from the last attempt
/// outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetryRowPosture {
    /// A passed-first-try row.
    PassedFirstTryRow,
    /// A passed-on-retry row.
    PassedOnRetryRow,
    /// A failed-all-retries row.
    FailedAllRetriesRow,
    /// An errored row.
    ErroredRow,
    /// A skipped row.
    SkippedRow,
    /// An aborted row.
    AbortedRow,
}

impl M5RetryRowPosture {
    /// Every retry posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PassedFirstTryRow,
        Self::PassedOnRetryRow,
        Self::FailedAllRetriesRow,
        Self::ErroredRow,
        Self::SkippedRow,
        Self::AbortedRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PassedFirstTryRow => "passed_first_try_row",
            Self::PassedOnRetryRow => "passed_on_retry_row",
            Self::FailedAllRetriesRow => "failed_all_retries_row",
            Self::ErroredRow => "errored_row",
            Self::SkippedRow => "skipped_row",
            Self::AbortedRow => "aborted_row",
        }
    }

    /// The frozen retry attempt outcome this posture maps 1:1 to.
    pub const fn outcome(self) -> M5RetryAttemptOutcome {
        match self {
            Self::PassedFirstTryRow => M5RetryAttemptOutcome::PassedFirstTry,
            Self::PassedOnRetryRow => M5RetryAttemptOutcome::PassedOnRetry,
            Self::FailedAllRetriesRow => M5RetryAttemptOutcome::FailedAllRetries,
            Self::ErroredRow => M5RetryAttemptOutcome::ErroredAttempt,
            Self::SkippedRow => M5RetryAttemptOutcome::SkippedAttempt,
            Self::AbortedRow => M5RetryAttemptOutcome::AbortedAttempt,
        }
    }

    /// True when the last outcome itself already signals a divergence (a pass that only happened
    /// after a failing attempt).
    pub const fn is_divergent(self) -> bool {
        matches!(self, Self::PassedOnRetryRow)
    }

    /// True when the row flags an outcome a reviewer should act on.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::PassedOnRetryRow
                | Self::FailedAllRetriesRow
                | Self::ErroredRow
                | Self::AbortedRow
        )
    }
}

/// One bounded action a retry-history row offers, so a row never hides its reveal / rerun /
/// open-logs / export affordances — the rerun-or-open-logs parity the acceptance criteria name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetryRowAction {
    /// Reveal the row's outcome sequence, deltas, confidence, and origin.
    RevealRetryDetails,
    /// Rerun the test behind this row.
    RerunTest,
    /// Open the raw attempt logs behind this row.
    OpenLogs,
    /// Export the retry history as test evidence.
    ExportRetryHistory,
}

impl M5RetryRowAction {
    /// Every retry action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RevealRetryDetails,
        Self::RerunTest,
        Self::OpenLogs,
        Self::ExportRetryHistory,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealRetryDetails => "reveal_retry_details",
            Self::RerunTest => "rerun_test",
            Self::OpenLogs => "open_logs",
            Self::ExportRetryHistory => "export_retry_history",
        }
    }
}

/// Controlled retry-row anatomy part. The parts in [`M5RetryRowAnatomyPart::MANDATORY`] are
/// required on every row so the stable test identity, the ordered outcomes, the environment /
/// build / runtime deltas, the classifier confidence, and the rerun-or-open-logs actions are
/// never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetryRowAnatomyPart {
    /// The stable test-identity cue.
    TestIdentityCue,
    /// The ordered-outcome-sequence cue.
    OutcomeSequenceCue,
    /// The environment / build / runtime delta cue.
    EnvBuildRuntimeDeltaCue,
    /// The classifier-confidence cue.
    ConfidenceCue,
    /// The attempt-origin cue.
    OriginCue,
    /// The rerun-or-open-logs action cue.
    RerunOpenLogsCue,
}

impl M5RetryRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TestIdentityCue,
        Self::OutcomeSequenceCue,
        Self::EnvBuildRuntimeDeltaCue,
        Self::ConfidenceCue,
        Self::OriginCue,
        Self::RerunOpenLogsCue,
    ];

    /// The anatomy parts every retry row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::TestIdentityCue,
        Self::OutcomeSequenceCue,
        Self::EnvBuildRuntimeDeltaCue,
        Self::ConfidenceCue,
        Self::RerunOpenLogsCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestIdentityCue => "test_identity_cue",
            Self::OutcomeSequenceCue => "outcome_sequence_cue",
            Self::EnvBuildRuntimeDeltaCue => "env_build_runtime_delta_cue",
            Self::ConfidenceCue => "confidence_cue",
            Self::OriginCue => "origin_cue",
            Self::RerunOpenLogsCue => "rerun_open_logs_cue",
        }
    }
}

/// A field the retry-row export carries so row truth is reconstructable. The fields in
/// [`M5RetryRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetryRowExportField {
    /// The stable test-identity ref.
    TestIdentityRef,
    /// The recent ordered outcomes.
    RecentOutcomes,
    /// The attempt origin.
    AttemptOrigin,
    /// The classifier confidence class.
    ConfidenceClass,
    /// The retry scope class.
    ScopeClass,
    /// The derived retry posture.
    RowPosture,
}

impl M5RetryRowExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TestIdentityRef,
        Self::RecentOutcomes,
        Self::AttemptOrigin,
        Self::ConfidenceClass,
        Self::ScopeClass,
        Self::RowPosture,
    ];

    /// The export fields every retry row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::TestIdentityRef,
        Self::RecentOutcomes,
        Self::AttemptOrigin,
        Self::ScopeClass,
        Self::RowPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestIdentityRef => "test_identity_ref",
            Self::RecentOutcomes => "recent_outcomes",
            Self::AttemptOrigin => "attempt_origin",
            Self::ConfidenceClass => "confidence_class",
            Self::ScopeClass => "scope_class",
            Self::RowPosture => "row_posture",
        }
    }
}

/// True when a provenance class marks the verdict / attempt as imported rather than a live local
/// run.
pub const fn provenance_is_imported(provenance: M5TestIntelligenceProvenanceClass) -> bool {
    matches!(
        provenance,
        M5TestIntelligenceProvenanceClass::ImportedCiArtifact
    )
}

/// True when a retry attempt outcome is a passing outcome.
pub const fn outcome_is_pass(outcome: M5RetryAttemptOutcome) -> bool {
    matches!(
        outcome,
        M5RetryAttemptOutcome::PassedFirstTry | M5RetryAttemptOutcome::PassedOnRetry
    )
}

/// True when a retry attempt outcome is a failing / erroring / aborting outcome.
pub const fn outcome_is_failure(outcome: M5RetryAttemptOutcome) -> bool {
    matches!(
        outcome,
        M5RetryAttemptOutcome::FailedAllRetries
            | M5RetryAttemptOutcome::ErroredAttempt
            | M5RetryAttemptOutcome::AbortedAttempt
    )
}

// ---- flaky-state-badge resolver -----------------------------------------

/// The full input to the flaky-state-badge resolver for one badge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FlakyBadgeResolutionInput {
    /// The flaky classification the badge asserts.
    pub classification: M5FlakyClassification,
    /// The classifier confidence class.
    pub confidence_class: M5FlakyConfidenceClass,
    /// The classifier source behind the verdict.
    pub classifier_source: M5FlakyClassifierSource,
    /// The provenance class behind the verdict.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// The mute / quarantine status.
    pub mute_state: M5FlakyMuteState,
    /// The number of attempts in the retry window this verdict was measured over.
    pub retry_window_size: u32,
    /// The number of failures observed within the retry window.
    pub observed_failures: u32,
    /// The last attempt outcome.
    pub last_outcome: M5RetryAttemptOutcome,
    /// The opaque stable badge identity (must be non-empty).
    pub badge_identity_ref: String,
    /// The opaque stable test identity (must be non-empty).
    pub test_identity_ref: String,
}

/// The resolved flaky-state-badge truth for one badge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedFlakyBadge {
    /// The flaky classification.
    pub classification: M5FlakyClassification,
    /// The classifier confidence class.
    pub confidence_class: M5FlakyConfidenceClass,
    /// The classifier source.
    pub classifier_source: M5FlakyClassifierSource,
    /// The provenance class.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// The mute state.
    pub mute_state: M5FlakyMuteState,
    /// The retry-window size, preserved from the input.
    pub retry_window_size: u32,
    /// The observed-failure count, preserved from the input.
    pub observed_failures: u32,
    /// The last attempt outcome, preserved from the input.
    pub last_outcome: M5RetryAttemptOutcome,
    /// The opaque badge identity, preserved exactly from the input.
    pub badge_identity_ref: String,
    /// The opaque test identity, preserved exactly from the input.
    pub test_identity_ref: String,
    /// The derived flaky posture.
    pub flaky_posture: M5FlakyBadgePosture,
    /// The bounded actions this badge offers.
    pub available_actions: Vec<M5FlakyBadgeAction>,
    /// True when the retry window is large enough to support a reproduced verdict.
    pub has_sufficient_evidence_window: bool,
    /// True when the badge claims reproduced flakiness.
    pub claims_reproduced_flaky: bool,
    /// True when a reproduced claim is backed by a sufficient evidence window (always true after
    /// resolution — an unsupported claim fails resolution).
    pub reproduced_claim_supported: bool,
    /// True when the badge names a confirmed flake (reproduced with evidence).
    pub is_confirmed_flaky: bool,
    /// True when the verdict is muted or under an active quarantine.
    pub is_muted_or_quarantined: bool,
    /// True when the badge needs a reviewer's attention before it reads as a settled verdict.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_flaky_state_badge`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5FlakyBadgeResolutionError {
    /// The badge identity ref was empty.
    EmptyBadgeIdentity,
    /// The test identity ref was empty.
    EmptyTestIdentity,
    /// The observed failures exceeded the retry-window size.
    InvalidFailureCount,
    /// A reproduced-flaky verdict was claimed without the required evidence window — one
    /// intermittent failure would masquerade as reproduced flakiness.
    ReproducedWithoutEvidenceWindow,
    /// A badge descriptor carried forbidden material.
    ForbiddenFlakyMaterial,
}

impl M5FlakyBadgeResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyBadgeIdentity => "empty_badge_identity",
            Self::EmptyTestIdentity => "empty_test_identity",
            Self::InvalidFailureCount => "invalid_failure_count",
            Self::ReproducedWithoutEvidenceWindow => "reproduced_without_evidence_window",
            Self::ForbiddenFlakyMaterial => "forbidden_flaky_material",
        }
    }
}

impl fmt::Display for M5FlakyBadgeResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "flaky state badge resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5FlakyBadgeResolutionError {}

/// Resolves one flaky-state badge from its declared flaky state.
///
/// The derived flaky posture is 1:1 with the flaky classification — stable, suspected-flaky,
/// reproduced-flaky, stable-again, manually-muted, or unknown-flaky — so a suspected badge never
/// borrows the authority of a reproduced one. A reproduced-flaky verdict is only accepted when
/// its retry window and observed-failure count meet the required evidence threshold and its
/// confidence is not a single occurrence or insufficient data; otherwise resolution fails, so
/// one intermittent failure can never visually masquerade as reproduced flakiness. The retry
/// window, classifier source, confidence, last outcome, and mute status are always carried, so
/// none of the evidence behind a verdict is hidden.
pub fn resolve_flaky_state_badge(
    input: &M5FlakyBadgeResolutionInput,
) -> Result<M5ResolvedFlakyBadge, M5FlakyBadgeResolutionError> {
    if input.badge_identity_ref.trim().is_empty() {
        return Err(M5FlakyBadgeResolutionError::EmptyBadgeIdentity);
    }
    if input.test_identity_ref.trim().is_empty() {
        return Err(M5FlakyBadgeResolutionError::EmptyTestIdentity);
    }
    if input.observed_failures > input.retry_window_size {
        return Err(M5FlakyBadgeResolutionError::InvalidFailureCount);
    }
    if value_repr_is_forbidden(&input.badge_identity_ref)
        || value_repr_is_forbidden(&input.test_identity_ref)
    {
        return Err(M5FlakyBadgeResolutionError::ForbiddenFlakyMaterial);
    }

    let flaky_posture = derive_flaky_posture(input.classification);
    let has_sufficient_evidence_window = input.retry_window_size >= REQUIRED_REPRODUCED_WINDOW
        && input.observed_failures >= REQUIRED_REPRODUCED_FAILURES
        && !matches!(
            input.confidence_class,
            M5FlakyConfidenceClass::SingleOccurrence | M5FlakyConfidenceClass::InsufficientData
        );
    let claims_reproduced_flaky = flaky_posture.is_confirmed_flaky();
    if claims_reproduced_flaky && !has_sufficient_evidence_window {
        return Err(M5FlakyBadgeResolutionError::ReproducedWithoutEvidenceWindow);
    }

    let is_confirmed_flaky = claims_reproduced_flaky && has_sufficient_evidence_window;
    let is_muted_or_quarantined = input.mute_state.is_muted();
    let available_actions = derive_flaky_actions(input.mute_state);

    Ok(M5ResolvedFlakyBadge {
        classification: input.classification,
        confidence_class: input.confidence_class,
        classifier_source: input.classifier_source,
        provenance_class: input.provenance_class,
        mute_state: input.mute_state,
        retry_window_size: input.retry_window_size,
        observed_failures: input.observed_failures,
        last_outcome: input.last_outcome,
        badge_identity_ref: input.badge_identity_ref.clone(),
        test_identity_ref: input.test_identity_ref.clone(),
        flaky_posture,
        available_actions,
        has_sufficient_evidence_window,
        claims_reproduced_flaky,
        reproduced_claim_supported: !claims_reproduced_flaky || has_sufficient_evidence_window,
        is_confirmed_flaky,
        is_muted_or_quarantined,
        needs_attention: flaky_posture.is_confirmed_flaky()
            || flaky_posture.is_unconfirmed_suspicion()
            || matches!(input.mute_state, M5FlakyMuteState::QuarantineExpired)
            || outcome_is_failure(input.last_outcome),
    })
}

/// The 1:1 flaky-classification → flaky-posture map.
fn derive_flaky_posture(classification: M5FlakyClassification) -> M5FlakyBadgePosture {
    match classification {
        M5FlakyClassification::Stable => M5FlakyBadgePosture::StableBadge,
        M5FlakyClassification::SuspectedFlaky => M5FlakyBadgePosture::SuspectedFlakyBadge,
        M5FlakyClassification::ReproducedFlaky => M5FlakyBadgePosture::ReproducedFlakyBadge,
        M5FlakyClassification::StableAgain => M5FlakyBadgePosture::StableAgainBadge,
        M5FlakyClassification::ManuallyMuted => M5FlakyBadgePosture::ManuallyMutedBadge,
        M5FlakyClassification::UnknownFlaky => M5FlakyBadgePosture::UnknownFlakyBadge,
    }
}

/// Derives the bounded flaky-action set from the mute state.
fn derive_flaky_actions(mute_state: M5FlakyMuteState) -> Vec<M5FlakyBadgeAction> {
    use M5FlakyBadgeAction as Action;
    let mut actions = vec![
        Action::RevealFlakyDetails,
        Action::OpenRetryHistory,
        Action::RerunTest,
    ];
    if mute_state.can_toggle() {
        actions.push(Action::MuteOrQuarantine);
    }
    actions.push(Action::ExportFlakyBadge);
    actions
}

// ---- retry-history-row resolver -----------------------------------------

/// The full input to the retry-history-row resolver for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RetryRowResolutionInput {
    /// The last / summary attempt outcome the posture derives from.
    pub last_outcome: M5RetryAttemptOutcome,
    /// The recent attempt outcomes in order (most-recent last; must be non-empty).
    pub recent_outcomes: Vec<M5RetryAttemptOutcome>,
    /// The retry scope class behind the rerun.
    pub scope_class: M5RetryScopeClass,
    /// The origin the attempt ran on.
    pub attempt_origin: M5RetryAttemptOrigin,
    /// The classifier confidence class.
    pub confidence_class: M5FlakyConfidenceClass,
    /// The provenance class behind the row.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// Whether the attempts differ by environment.
    pub has_env_delta: bool,
    /// Whether the attempts differ by build.
    pub has_build_delta: bool,
    /// Whether the attempts differ by runtime.
    pub has_runtime_delta: bool,
    /// The opaque stable test identity (must be non-empty).
    pub test_identity_ref: String,
    /// The opaque durable ref to the raw attempt logs (must be non-empty).
    pub attempt_log_ref: String,
}

/// The resolved retry-history-row truth for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRetryRow {
    /// The last attempt outcome.
    pub last_outcome: M5RetryAttemptOutcome,
    /// The recent ordered outcomes, preserved exactly from the input.
    pub recent_outcomes: Vec<M5RetryAttemptOutcome>,
    /// The retry scope class.
    pub scope_class: M5RetryScopeClass,
    /// The attempt origin.
    pub attempt_origin: M5RetryAttemptOrigin,
    /// The classifier confidence class.
    pub confidence_class: M5FlakyConfidenceClass,
    /// The provenance class.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// Whether the attempts differ by environment, preserved from the input.
    pub has_env_delta: bool,
    /// Whether the attempts differ by build, preserved from the input.
    pub has_build_delta: bool,
    /// Whether the attempts differ by runtime, preserved from the input.
    pub has_runtime_delta: bool,
    /// The opaque test identity, preserved exactly from the input.
    pub test_identity_ref: String,
    /// The opaque attempt-log ref, preserved exactly from the input.
    pub attempt_log_ref: String,
    /// The derived retry posture.
    pub row_posture: M5RetryRowPosture,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5RetryRowAction>,
    /// True when the recent outcomes diverged (a pass and a failure across the window).
    pub is_divergent: bool,
    /// True when the row preserves its ordered outcome sequence (always true — the sequence is
    /// carried verbatim).
    pub preserves_outcome_order: bool,
    /// True when the row discloses at least one environment / build / runtime delta.
    pub discloses_env_build_runtime_delta: bool,
    /// True when a divergence is explained by the ordered sequence and disclosed deltas (always
    /// true after resolution — an unexplained divergence fails resolution).
    pub explains_divergent_outcomes: bool,
    /// True when the attempt is imported rather than a live local / remote run.
    pub is_imported: bool,
    /// True when the rerun widened the selection (kept disclosed, never hidden).
    pub widened_scope: bool,
    /// True when the row carries a durable path back to the raw attempt logs — the open-logs
    /// continuity.
    pub has_log_continuity: bool,
    /// True when the row flags an outcome a reviewer should act on.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_retry_history_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RetryRowResolutionError {
    /// The test identity ref was empty.
    EmptyTestIdentity,
    /// The attempt-log ref was empty — the row would lose its path back to the raw logs.
    EmptyLogReference,
    /// The recent outcome sequence was empty.
    EmptyOutcomeSequence,
    /// A divergent row lacked the ordered sequence needed to explain the divergence.
    DivergenceWithoutSequence,
    /// A row descriptor carried forbidden material.
    ForbiddenRetryMaterial,
}

impl M5RetryRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyTestIdentity => "empty_test_identity",
            Self::EmptyLogReference => "empty_log_reference",
            Self::EmptyOutcomeSequence => "empty_outcome_sequence",
            Self::DivergenceWithoutSequence => "divergence_without_sequence",
            Self::ForbiddenRetryMaterial => "forbidden_retry_material",
        }
    }
}

impl fmt::Display for M5RetryRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "retry history row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RetryRowResolutionError {}

/// Resolves one retry-history row from its declared attempt history.
///
/// The derived retry posture is 1:1 with the last attempt outcome — passed-first-try,
/// passed-on-retry, failed-all-retries, errored, skipped, or aborted — so a pass-on-retry never
/// reads as a clean first-try pass. A divergence (a pass and a failure across the recent
/// outcomes, or a pass-on-retry) must carry an ordered sequence of at least two outcomes to
/// explain it; otherwise resolution fails, so a row always preserves enough context to explain
/// why the same test passed here and failed there. The environment / build / runtime deltas, the
/// attempt origin, and a durable path back to the raw attempt logs are always carried; reveal,
/// rerun, open-logs, and export are always offered.
pub fn resolve_retry_history_row(
    input: &M5RetryRowResolutionInput,
) -> Result<M5ResolvedRetryRow, M5RetryRowResolutionError> {
    if input.test_identity_ref.trim().is_empty() {
        return Err(M5RetryRowResolutionError::EmptyTestIdentity);
    }
    if input.attempt_log_ref.trim().is_empty() {
        return Err(M5RetryRowResolutionError::EmptyLogReference);
    }
    if input.recent_outcomes.is_empty() {
        return Err(M5RetryRowResolutionError::EmptyOutcomeSequence);
    }
    if value_repr_is_forbidden(&input.test_identity_ref)
        || value_repr_is_forbidden(&input.attempt_log_ref)
    {
        return Err(M5RetryRowResolutionError::ForbiddenRetryMaterial);
    }

    let row_posture = derive_retry_posture(input.last_outcome);
    let has_pass = input.recent_outcomes.iter().copied().any(outcome_is_pass);
    let has_failure = input
        .recent_outcomes
        .iter()
        .copied()
        .any(outcome_is_failure);
    let is_divergent = row_posture.is_divergent() || (has_pass && has_failure);
    if is_divergent && input.recent_outcomes.len() < 2 {
        return Err(M5RetryRowResolutionError::DivergenceWithoutSequence);
    }

    let discloses_env_build_runtime_delta =
        input.has_env_delta || input.has_build_delta || input.has_runtime_delta;
    let is_imported = matches!(input.scope_class, M5RetryScopeClass::ImportedAttempt)
        || input.attempt_origin.is_imported()
        || provenance_is_imported(input.provenance_class);
    let widened_scope = matches!(input.scope_class, M5RetryScopeClass::WidenedSelection);

    Ok(M5ResolvedRetryRow {
        last_outcome: input.last_outcome,
        recent_outcomes: input.recent_outcomes.clone(),
        scope_class: input.scope_class,
        attempt_origin: input.attempt_origin,
        confidence_class: input.confidence_class,
        provenance_class: input.provenance_class,
        has_env_delta: input.has_env_delta,
        has_build_delta: input.has_build_delta,
        has_runtime_delta: input.has_runtime_delta,
        test_identity_ref: input.test_identity_ref.clone(),
        attempt_log_ref: input.attempt_log_ref.clone(),
        row_posture,
        available_actions: vec![
            M5RetryRowAction::RevealRetryDetails,
            M5RetryRowAction::RerunTest,
            M5RetryRowAction::OpenLogs,
            M5RetryRowAction::ExportRetryHistory,
        ],
        is_divergent,
        preserves_outcome_order: true,
        discloses_env_build_runtime_delta,
        explains_divergent_outcomes: !is_divergent || input.recent_outcomes.len() >= 2,
        is_imported,
        widened_scope,
        has_log_continuity: !input.attempt_log_ref.trim().is_empty(),
        needs_attention: row_posture.needs_attention() || is_divergent || widened_scope,
    })
}

/// The 1:1 retry-outcome → retry-posture map.
fn derive_retry_posture(outcome: M5RetryAttemptOutcome) -> M5RetryRowPosture {
    match outcome {
        M5RetryAttemptOutcome::PassedFirstTry => M5RetryRowPosture::PassedFirstTryRow,
        M5RetryAttemptOutcome::PassedOnRetry => M5RetryRowPosture::PassedOnRetryRow,
        M5RetryAttemptOutcome::FailedAllRetries => M5RetryRowPosture::FailedAllRetriesRow,
        M5RetryAttemptOutcome::ErroredAttempt => M5RetryRowPosture::ErroredRow,
        M5RetryAttemptOutcome::SkippedAttempt => M5RetryRowPosture::SkippedRow,
        M5RetryAttemptOutcome::AbortedAttempt => M5RetryRowPosture::AbortedRow,
    }
}

// ---- worked cases -------------------------------------------------------

/// One worked flaky-state-badge resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FlakyBadgeResolutionCase {
    /// The resolver input.
    pub input: M5FlakyBadgeResolutionInput,
    /// The resolved truth. Must equal `resolve_flaky_state_badge(&input)`.
    pub resolved: M5ResolvedFlakyBadge,
}

impl M5FlakyBadgeResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5FlakyBadgeResolutionInput) -> Self {
        let resolved = resolve_flaky_state_badge(&input).expect("seed flaky case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_flaky_state_badge(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved badge preserves the input identities exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.badge_identity_ref == self.input.badge_identity_ref
            && self.resolved.test_identity_ref == self.input.test_identity_ref
    }
}

/// One worked retry-history-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RetryRowResolutionCase {
    /// The resolver input.
    pub input: M5RetryRowResolutionInput,
    /// The resolved truth. Must equal `resolve_retry_history_row(&input)`.
    pub resolved: M5ResolvedRetryRow,
}

impl M5RetryRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5RetryRowResolutionInput) -> Self {
        let resolved = resolve_retry_history_row(&input).expect("seed retry case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_retry_history_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved row preserves the input identity, log ref, and ordered outcomes
    /// exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.test_identity_ref == self.input.test_identity_ref
            && self.resolved.attempt_log_ref == self.input.attempt_log_ref
            && self.resolved.recent_outcomes == self.input.recent_outcomes
    }
}

/// One row in the primitive matrix: one quality consumer bound to the shared badge and row
/// anatomy, flaky classifications, confidence classes, classifier sources, mute states, retry
/// outcomes, retry scope classes, attempt origins, flaky and retry postures, bounded actions,
/// export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FlakyRetryComponentConsumerRow {
    /// Quality consumer family.
    pub consumer_surface: M5FlakyRetryComponentConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5TestIntelligenceQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 quality surface families that render / consume these components.
    pub surface_families: Vec<M5TestIntelligenceSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5TestIntelligenceDeploymentLine>,
    /// Flaky-badge anatomy parts this consumer renders (must include the mandatory parts).
    pub flaky_anatomy_parts: Vec<M5FlakyBadgeAnatomyPart>,
    /// Retry-row anatomy parts this consumer renders (must include the mandatory parts).
    pub retry_anatomy_parts: Vec<M5RetryRowAnatomyPart>,
    /// Flaky classifications this consumer distinguishes.
    pub flaky_classifications: Vec<M5FlakyClassification>,
    /// Flaky confidence classes this consumer distinguishes.
    pub flaky_confidence_classes: Vec<M5FlakyConfidenceClass>,
    /// Classifier sources this consumer distinguishes.
    pub classifier_sources: Vec<M5FlakyClassifierSource>,
    /// Mute states this consumer distinguishes.
    pub mute_states: Vec<M5FlakyMuteState>,
    /// Provenance classes this consumer distinguishes.
    pub provenance_classes: Vec<M5TestIntelligenceProvenanceClass>,
    /// Flaky postures this consumer distinguishes.
    pub flaky_postures: Vec<M5FlakyBadgePosture>,
    /// Retry attempt outcomes this consumer distinguishes.
    pub retry_attempt_outcomes: Vec<M5RetryAttemptOutcome>,
    /// Retry scope classes this consumer distinguishes.
    pub retry_scope_classes: Vec<M5RetryScopeClass>,
    /// Retry attempt origins this consumer distinguishes.
    pub retry_attempt_origins: Vec<M5RetryAttemptOrigin>,
    /// Retry postures this consumer distinguishes.
    pub retry_postures: Vec<M5RetryRowPosture>,
    /// Bounded flaky actions this consumer offers.
    pub flaky_actions: Vec<M5FlakyBadgeAction>,
    /// Bounded retry actions this consumer offers.
    pub retry_actions: Vec<M5RetryRowAction>,
    /// Flaky export fields this consumer carries (must include the mandatory fields).
    pub flaky_export_fields: Vec<M5FlakyBadgeExportField>,
    /// Retry export fields this consumer carries (must include the mandatory fields).
    pub retry_export_fields: Vec<M5RetryRowExportField>,
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
    /// Worked flaky-badge resolutions proving the resolver on this consumer.
    pub flaky_examples: Vec<M5FlakyBadgeResolutionCase>,
    /// Worked retry-row resolutions proving the resolver on this consumer.
    pub retry_examples: Vec<M5RetryRowResolutionCase>,
    /// Hard invariant: this consumer never labels one intermittent failure as confirmed
    /// flakiness. MUST be `false`.
    pub labels_intermittent_as_confirmed_flaky: bool,
    /// Hard invariant: this consumer never hides the retry window or the classifier source
    /// behind a bare verdict. MUST be `false`.
    pub hides_retry_window_or_classifier_source: bool,
    /// Hard invariant: this consumer never drops the environment / build / runtime delta context
    /// a retry row needs to explain divergent outcomes. MUST be `false`.
    pub drops_env_build_runtime_delta_context: bool,
    /// Hard invariant: this consumer never invents an alternate label for a governed flaky or
    /// retry state. MUST be `false`.
    pub invents_alternate_flaky_or_retry_state_label: bool,
}

impl M5FlakyRetryComponentConsumerRow {
    /// True when the row declares every mandatory flaky anatomy part.
    fn declares_mandatory_flaky_anatomy(&self) -> bool {
        let present: BTreeSet<M5FlakyBadgeAnatomyPart> =
            self.flaky_anatomy_parts.iter().copied().collect();
        M5FlakyBadgeAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory retry anatomy part.
    fn declares_mandatory_retry_anatomy(&self) -> bool {
        let present: BTreeSet<M5RetryRowAnatomyPart> =
            self.retry_anatomy_parts.iter().copied().collect();
        M5RetryRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory flaky export field.
    fn declares_mandatory_flaky_export(&self) -> bool {
        let present: BTreeSet<M5FlakyBadgeExportField> =
            self.flaky_export_fields.iter().copied().collect();
        M5FlakyBadgeExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory retry export field.
    fn declares_mandatory_retry_export(&self) -> bool {
        let present: BTreeSet<M5RetryRowExportField> =
            self.retry_export_fields.iter().copied().collect();
        M5RetryRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.labels_intermittent_as_confirmed_flaky
            && !self.hides_retry_window_or_classifier_source
            && !self.drops_env_build_runtime_delta_context
            && !self.invents_alternate_flaky_or_retry_state_label
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FlakyRetryComponentVocabularySet {
    /// Quality consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Flaky-anatomy-part tokens.
    pub flaky_anatomy_parts: Vec<String>,
    /// Retry-anatomy-part tokens.
    pub retry_anatomy_parts: Vec<String>,
    /// Flaky-posture tokens.
    pub flaky_postures: Vec<String>,
    /// Retry-posture tokens.
    pub retry_postures: Vec<String>,
    /// Classifier-source tokens.
    pub classifier_sources: Vec<String>,
    /// Mute-state tokens.
    pub mute_states: Vec<String>,
    /// Attempt-origin tokens.
    pub retry_attempt_origins: Vec<String>,
    /// Flaky-action tokens.
    pub flaky_actions: Vec<String>,
    /// Retry-action tokens.
    pub retry_actions: Vec<String>,
    /// Flaky-export-field tokens.
    pub flaky_export_fields: Vec<String>,
    /// Retry-export-field tokens.
    pub retry_export_fields: Vec<String>,
    /// Flaky-classification tokens (reused from the frozen matrix).
    pub flaky_classifications: Vec<String>,
    /// Flaky-confidence-class tokens (reused from the frozen matrix).
    pub flaky_confidence_classes: Vec<String>,
    /// Retry-attempt-outcome tokens (reused from the frozen matrix).
    pub retry_attempt_outcomes: Vec<String>,
    /// Retry-scope-class tokens (reused from the frozen matrix).
    pub retry_scope_classes: Vec<String>,
    /// Provenance-class tokens (reused from the frozen matrix).
    pub provenance_classes: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5FlakyRetryComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5FlakyRetryComponentConsumerSurface::ALL, |v| v.as_str()),
            flaky_anatomy_parts: tokens(&M5FlakyBadgeAnatomyPart::ALL, |v| v.as_str()),
            retry_anatomy_parts: tokens(&M5RetryRowAnatomyPart::ALL, |v| v.as_str()),
            flaky_postures: tokens(&M5FlakyBadgePosture::ALL, |v| v.as_str()),
            retry_postures: tokens(&M5RetryRowPosture::ALL, |v| v.as_str()),
            classifier_sources: tokens(&M5FlakyClassifierSource::ALL, |v| v.as_str()),
            mute_states: tokens(&M5FlakyMuteState::ALL, |v| v.as_str()),
            retry_attempt_origins: tokens(&M5RetryAttemptOrigin::ALL, |v| v.as_str()),
            flaky_actions: tokens(&M5FlakyBadgeAction::ALL, |v| v.as_str()),
            retry_actions: tokens(&M5RetryRowAction::ALL, |v| v.as_str()),
            flaky_export_fields: tokens(&M5FlakyBadgeExportField::ALL, |v| v.as_str()),
            retry_export_fields: tokens(&M5RetryRowExportField::ALL, |v| v.as_str()),
            flaky_classifications: tokens(&M5FlakyClassification::ALL, |v| v.as_str()),
            flaky_confidence_classes: tokens(&M5FlakyConfidenceClass::ALL, |v| v.as_str()),
            retry_attempt_outcomes: tokens(&M5RetryAttemptOutcome::ALL, |v| v.as_str()),
            retry_scope_classes: tokens(&M5RetryScopeClass::ALL, |v| v.as_str()),
            provenance_classes: tokens(&M5TestIntelligenceProvenanceClass::ALL, |v| v.as_str()),
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
pub struct M5FlakyRetryComponentGovernanceReview {
    /// The flaky badge shows its classification and classifier confidence.
    pub badge_shows_classification_and_confidence: bool,
    /// The flaky badge shows the retry window it was measured over.
    pub badge_shows_retry_window: bool,
    /// The flaky badge shows its classifier source and last outcome.
    pub badge_shows_classifier_source_and_last_outcome: bool,
    /// The flaky badge shows its mute / quarantine status.
    pub badge_shows_mute_or_quarantine_status: bool,
    /// One intermittent failure never presents as confirmed flakiness without the evidence
    /// window.
    pub intermittent_never_confirmed_without_evidence_window: bool,
    /// The retry row shows its recent outcomes in order.
    pub retry_row_shows_ordered_outcomes: bool,
    /// The retry row shows its environment / build / runtime deltas.
    pub retry_row_shows_env_build_runtime_deltas: bool,
    /// The retry row shows its classifier confidence.
    pub retry_row_shows_classifier_confidence: bool,
    /// The retry row offers rerun and open-logs actions.
    pub retry_row_offers_rerun_and_open_logs: bool,
    /// The retry row preserves its stable test identity.
    pub retry_row_preserves_stable_test_identity: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The components keep the same truth across every quality consumer surface.
    pub components_stable_across_consumer_surfaces: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// The support / export packet reconstructs flaky / retry truth.
    pub support_export_reconstructs_flaky_retry_truth: bool,
    /// Later M5 quality components cannot invent parallel flaky / retry vocabulary.
    pub later_components_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FlakyRetryComponentConsumerProjection {
    /// Flaky and retry surfaces consume the shared classification / confidence / outcome
    /// vocabulary.
    pub flaky_and_retry_surfaces_consume_shared_vocabulary: bool,
    /// The flaky-posture resolver reads a single canonical source.
    pub flaky_posture_reads_single_source: bool,
    /// The retry-posture resolver reads a single canonical source.
    pub retry_posture_reads_single_source: bool,
    /// The CI and support/export consumers read the same flaky / retry vocabulary.
    pub ci_and_support_read_same_flaky_retry_vocabulary: bool,
    /// Headless and desktop flaky / retry read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FlakyRetryComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the two quality components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FlakyRetryComponentReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting test-evidence audit.
    pub test_evidence_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5FlakyRetryComponentsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5FlakyRetryComponentsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Quality consumer rows.
    pub rows: Vec<M5FlakyRetryComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5FlakyRetryComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5FlakyRetryComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FlakyRetryComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5FlakyRetryComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5FlakyRetryComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 flaky-state-badge / retry-history-row primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FlakyRetryComponentsPacket {
    /// Record kind; must equal [`M5_FLAKY_RETRY_COMPONENTS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FLAKY_RETRY_COMPONENTS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Quality consumer rows.
    pub rows: Vec<M5FlakyRetryComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5FlakyRetryComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5FlakyRetryComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FlakyRetryComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5FlakyRetryComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5FlakyRetryComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5FlakyRetryComponentsPacket {
    /// Builds an M5 flaky-retry-components primitive packet from stable-lane input.
    pub fn new(input: M5FlakyRetryComponentsPacketInput) -> Self {
        Self {
            record_kind: M5_FLAKY_RETRY_COMPONENTS_RECORD_KIND.to_owned(),
            schema_version: M5_FLAKY_RETRY_COMPONENTS_SCHEMA_VERSION,
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

    /// Validates the M5 flaky-retry-components primitive invariants.
    pub fn validate(&self) -> Vec<M5FlakyRetryComponentViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_FLAKY_RETRY_COMPONENTS_RECORD_KIND {
            violations.push(M5FlakyRetryComponentViolation::WrongRecordKind);
        }
        if self.schema_version != M5_FLAKY_RETRY_COMPONENTS_SCHEMA_VERSION {
            violations.push(M5FlakyRetryComponentViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5FlakyRetryComponentViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_flaky_posture_coverage(self, &mut violations);
        validate_retry_posture_coverage(self, &mut violations);
        validate_evidence_window_disclosure(self, &mut violations);
        validate_mute_disclosure(self, &mut violations);
        validate_divergence_context(self, &mut violations);
        validate_attempt_origin_coverage(self, &mut violations);
        validate_log_continuity(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 flaky retry components packet serializes"),
        ) {
            violations.push(M5FlakyRetryComponentViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 flaky retry components packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per quality consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,flaky_anatomy,flaky_postures,mute_states,retry_postures,retry_origins,flaky_actions,retry_actions,flaky_examples,retry_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.flaky_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.flaky_postures, |v| v.as_str()),
                join_tokens(&row.mute_states, |v| v.as_str()),
                join_tokens(&row.retry_postures, |v| v.as_str()),
                join_tokens(&row.retry_attempt_origins, |v| v.as_str()),
                join_tokens(&row.flaky_actions, |v| v.as_str()),
                join_tokens(&row.retry_actions, |v| v.as_str()),
                row.flaky_examples.len(),
                row.retry_examples.len(),
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
        out.push_str("# M5 Flaky-State-Badge / Retry-History-Row Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Quality consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Flaky postures: {}\n",
            self.vocabulary_set.flaky_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Retry postures: {}\n",
            self.vocabulary_set.retry_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Classifier sources: {}\n",
            self.vocabulary_set.classifier_sources.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Quality consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked badges: {} / rows: {}\n",
                row.flaky_examples.len(),
                row.retry_examples.len()
            ));
            for case in &row.flaky_examples {
                out.push_str(&format!(
                    "    - badge `{}` (`{}`) -> `{}` (confirmed `{}`, window `{}`, muted `{}`)\n",
                    case.resolved.badge_identity_ref,
                    case.resolved.classification.as_str(),
                    case.resolved.flaky_posture.as_str(),
                    case.resolved.is_confirmed_flaky,
                    case.resolved.has_sufficient_evidence_window,
                    case.resolved.is_muted_or_quarantined,
                ));
            }
            for case in &row.retry_examples {
                out.push_str(&format!(
                    "    - row `{}` (`{}`) -> `{}` (divergent `{}`, delta `{}`, logs `{}`)\n",
                    case.resolved.test_identity_ref,
                    case.resolved.last_outcome.as_str(),
                    case.resolved.row_posture.as_str(),
                    case.resolved.is_divergent,
                    case.resolved.discloses_env_build_runtime_delta,
                    case.resolved.has_log_continuity,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 flaky-retry-components export.
#[derive(Debug)]
pub enum M5FlakyRetryComponentArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5FlakyRetryComponentViolation>),
}

impl fmt::Display for M5FlakyRetryComponentArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 flaky retry components export parse failed: {error}"
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
                    "m5 flaky retry components export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5FlakyRetryComponentArtifactError {}

/// Validation failures emitted by [`M5FlakyRetryComponentsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5FlakyRetryComponentViolation {
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
    /// A required quality consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A quality consumer row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory flaky anatomy parts.
    MandatoryFlakyAnatomyMissing,
    /// A row omits one of the mandatory retry anatomy parts.
    MandatoryRetryAnatomyMissing,
    /// A row omits one of the mandatory flaky export fields.
    MandatoryFlakyExportMissing,
    /// A row omits one of the mandatory retry export fields.
    MandatoryRetryExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked flaky or retry resolutions.
    ExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every flaky posture.
    FlakyPostureCoverageUnproven,
    /// The worked resolutions do not exercise every retry posture.
    RetryPostureCoverageUnproven,
    /// The worked resolutions do not prove both a confirmed reproduced flake with a sufficient
    /// evidence window and a suspected flake that is not confirmed.
    EvidenceWindowDisclosureUnproven,
    /// The worked resolutions do not prove both a muted / quarantined verdict and an unmuted
    /// one.
    MuteDisclosureUnproven,
    /// The worked resolutions do not prove a divergent retry row that discloses an environment /
    /// build / runtime delta.
    DivergenceContextUnproven,
    /// The worked resolutions do not exercise the local, remote, notebook, and imported-CI
    /// attempt origins.
    AttemptOriginCoverageUnproven,
    /// A worked retry resolution does not preserve a durable path back to the raw attempt logs.
    LogContinuityUnproven,
    /// A worked resolution does not preserve its exact identity and ordered outcomes.
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

impl M5FlakyRetryComponentViolation {
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
            Self::MandatoryFlakyAnatomyMissing => "mandatory_flaky_anatomy_missing",
            Self::MandatoryRetryAnatomyMissing => "mandatory_retry_anatomy_missing",
            Self::MandatoryFlakyExportMissing => "mandatory_flaky_export_missing",
            Self::MandatoryRetryExportMissing => "mandatory_retry_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleMissing => "example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::FlakyPostureCoverageUnproven => "flaky_posture_coverage_unproven",
            Self::RetryPostureCoverageUnproven => "retry_posture_coverage_unproven",
            Self::EvidenceWindowDisclosureUnproven => "evidence_window_disclosure_unproven",
            Self::MuteDisclosureUnproven => "mute_disclosure_unproven",
            Self::DivergenceContextUnproven => "divergence_context_unproven",
            Self::AttemptOriginCoverageUnproven => "attempt_origin_coverage_unproven",
            Self::LogContinuityUnproven => "log_continuity_unproven",
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

/// Reads and validates the checked-in stable M5 flaky-retry-components export.
pub fn current_stable_m5_flaky_retry_components_export(
) -> Result<M5FlakyRetryComponentsPacket, M5FlakyRetryComponentArtifactError> {
    let packet: M5FlakyRetryComponentsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-flaky-retry-primitive-proof/support_export.json"
    )))
    .map_err(M5FlakyRetryComponentArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5FlakyRetryComponentArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_FLAKY_RETRY_COMPONENTS_FLAKY_SCHEMA_REF,
        M5_FLAKY_RETRY_COMPONENTS_RETRY_SCHEMA_REF,
        M5_FLAKY_RETRY_COMPONENTS_DOC_REF,
        M5_FLAKY_RETRY_COMPONENTS_COMPONENT_MATRIX_REF,
        M5_FLAKY_RETRY_COMPONENTS_FLAKY_VERDICT_REF,
        M5_FLAKY_RETRY_COMPONENTS_TEST_ATTEMPT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5FlakyRetryComponentViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5FlakyRetryComponentViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let present: BTreeSet<M5FlakyRetryComponentConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5FlakyRetryComponentConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5FlakyRetryComponentViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.flaky_anatomy_parts.is_empty()
            || row.retry_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.flaky_classifications.is_empty()
            || row.flaky_confidence_classes.is_empty()
            || row.classifier_sources.is_empty()
            || row.mute_states.is_empty()
            || row.provenance_classes.is_empty()
            || row.flaky_postures.is_empty()
            || row.retry_attempt_outcomes.is_empty()
            || row.retry_scope_classes.is_empty()
            || row.retry_attempt_origins.is_empty()
            || row.retry_postures.is_empty()
            || row.flaky_actions.is_empty()
            || row.retry_actions.is_empty()
            || row.flaky_export_fields.is_empty()
            || row.retry_export_fields.is_empty()
        {
            violations.push(M5FlakyRetryComponentViolation::RowIncomplete);
        }
        if !row.declares_mandatory_flaky_anatomy() {
            violations.push(M5FlakyRetryComponentViolation::MandatoryFlakyAnatomyMissing);
        }
        if !row.declares_mandatory_retry_anatomy() {
            violations.push(M5FlakyRetryComponentViolation::MandatoryRetryAnatomyMissing);
        }
        if !row.declares_mandatory_flaky_export() {
            violations.push(M5FlakyRetryComponentViolation::MandatoryFlakyExportMissing);
        }
        if !row.declares_mandatory_retry_export() {
            violations.push(M5FlakyRetryComponentViolation::MandatoryRetryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TestIntelligenceAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5FlakyRetryComponentViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5FlakyRetryComponentViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5FlakyRetryComponentViolation::DowngradeTriggersMissing);
        }
        if row.flaky_examples.is_empty() || row.retry_examples.is_empty() {
            violations.push(M5FlakyRetryComponentViolation::ExampleMissing);
        }
        if row
            .flaky_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .retry_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5FlakyRetryComponentViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5FlakyRetryComponentViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5FlakyRetryComponentViolation::RowInvariantViolated);
        }
    }
}

/// Every flaky posture must be exercised by some worked resolution — the proof that stable,
/// suspected, reproduced, stable-again, muted, and unknown verdicts each get a distinct posture
/// rather than one collapsed verdict.
fn validate_flaky_posture_coverage(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let exercised: BTreeSet<M5FlakyBadgePosture> = packet
        .rows
        .iter()
        .flat_map(|row| row.flaky_examples.iter())
        .map(|case| case.resolved.flaky_posture)
        .collect();
    let covered = M5FlakyBadgePosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture));
    if !covered {
        violations.push(M5FlakyRetryComponentViolation::FlakyPostureCoverageUnproven);
    }
}

/// Every retry posture must be exercised by some worked resolution — the proof that a
/// pass-on-retry never collapses into a clean first-try pass.
fn validate_retry_posture_coverage(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let exercised: BTreeSet<M5RetryRowPosture> = packet
        .rows
        .iter()
        .flat_map(|row| row.retry_examples.iter())
        .map(|case| case.resolved.row_posture)
        .collect();
    let covered = M5RetryRowPosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture));
    if !covered {
        violations.push(M5FlakyRetryComponentViolation::RetryPostureCoverageUnproven);
    }
}

/// At least one worked flaky resolution must prove a confirmed reproduced flake backed by a
/// sufficient evidence window, and at least one must prove a suspected flake that is *not*
/// confirmed — the acceptance-criterion example that one intermittent failure cannot masquerade
/// as reproduced flakiness without the evidence window.
fn validate_evidence_window_disclosure(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let has_confirmed = packet.rows.iter().any(|row| {
        row.flaky_examples.iter().any(|case| {
            case.resolved.is_confirmed_flaky && case.resolved.has_sufficient_evidence_window
        })
    });
    let has_unconfirmed_suspicion = packet.rows.iter().any(|row| {
        row.flaky_examples.iter().any(|case| {
            case.resolved.flaky_posture == M5FlakyBadgePosture::SuspectedFlakyBadge
                && !case.resolved.is_confirmed_flaky
        })
    });
    if !(has_confirmed && has_unconfirmed_suspicion) {
        violations.push(M5FlakyRetryComponentViolation::EvidenceWindowDisclosureUnproven);
    }
}

/// At least one worked flaky resolution must prove a muted / quarantined verdict and at least one
/// an unmuted one — the guardrail that mute / quarantine status is always disclosed.
fn validate_mute_disclosure(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let has_muted = packet.rows.iter().any(|row| {
        row.flaky_examples
            .iter()
            .any(|case| case.resolved.is_muted_or_quarantined)
    });
    let has_unmuted = packet.rows.iter().any(|row| {
        row.flaky_examples
            .iter()
            .any(|case| !case.resolved.is_muted_or_quarantined)
    });
    if !(has_muted && has_unmuted) {
        violations.push(M5FlakyRetryComponentViolation::MuteDisclosureUnproven);
    }
}

/// At least one worked retry resolution must prove a divergent row that discloses an environment
/// / build / runtime delta — the acceptance-criterion requirement that a retry row preserves
/// enough context to explain divergent outcomes.
fn validate_divergence_context(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let has_explained_divergence = packet.rows.iter().any(|row| {
        row.retry_examples.iter().any(|case| {
            case.resolved.is_divergent && case.resolved.discloses_env_build_runtime_delta
        })
    });
    if !has_explained_divergence {
        violations.push(M5FlakyRetryComponentViolation::DivergenceContextUnproven);
    }
}

/// The worked retry resolutions must exercise the local, remote, notebook, and imported-CI
/// attempt origins — the acceptance-criterion requirement that a retry row explains divergent
/// outcomes across every attempt origin.
fn validate_attempt_origin_coverage(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let exercised: BTreeSet<M5RetryAttemptOrigin> = packet
        .rows
        .iter()
        .flat_map(|row| row.retry_examples.iter())
        .map(|case| case.resolved.attempt_origin)
        .collect();
    let covered = [
        M5RetryAttemptOrigin::LocalAttempt,
        M5RetryAttemptOrigin::RemoteAttempt,
        M5RetryAttemptOrigin::NotebookAttempt,
        M5RetryAttemptOrigin::ImportedCiAttempt,
    ]
    .iter()
    .all(|origin| exercised.contains(origin));
    if !covered {
        violations.push(M5FlakyRetryComponentViolation::AttemptOriginCoverageUnproven);
    }
}

/// Every worked retry resolution must preserve a durable path back to the raw attempt logs — the
/// acceptance-criterion requirement that the open-logs continuity is never severed.
fn validate_log_continuity(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.retry_examples.iter())
        .all(|case| case.resolved.has_log_continuity);
    if !preserved {
        violations.push(M5FlakyRetryComponentViolation::LogContinuityUnproven);
    }
}

/// Every worked resolution must preserve its exact identity and ordered outcomes — the invariant
/// that neither component rewrites the user's verdict or attempt identity.
fn validate_identity_preservation(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let flaky_preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.flaky_examples.iter())
        .all(|case| case.preserves_identity());
    let retry_preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.retry_examples.iter())
        .all(|case| case.preserves_identity());
    if !(flaky_preserved && retry_preserved) {
        violations.push(M5FlakyRetryComponentViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.badge_shows_classification_and_confidence,
        review.badge_shows_retry_window,
        review.badge_shows_classifier_source_and_last_outcome,
        review.badge_shows_mute_or_quarantine_status,
        review.intermittent_never_confirmed_without_evidence_window,
        review.retry_row_shows_ordered_outcomes,
        review.retry_row_shows_env_build_runtime_deltas,
        review.retry_row_shows_classifier_confidence,
        review.retry_row_offers_rerun_and_open_logs,
        review.retry_row_preserves_stable_test_identity,
        review.components_stable_across_deployment_lines,
        review.components_stable_across_consumer_surfaces,
        review.every_component_declares_accessibility_route,
        review.support_export_reconstructs_flaky_retry_truth,
        review.later_components_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5FlakyRetryComponentViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.flaky_and_retry_surfaces_consume_shared_vocabulary,
        projection.flaky_posture_reads_single_source,
        projection.retry_posture_reads_single_source,
        projection.ci_and_support_read_same_flaky_retry_vocabulary,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5FlakyRetryComponentViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5FlakyRetryComponentViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5FlakyRetryComponentsPacket,
    violations: &mut Vec<M5FlakyRetryComponentViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.test_evidence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5FlakyRetryComponentViolation::ReleasePostureIncomplete);
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
