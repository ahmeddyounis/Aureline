//! Two reusable M5 test-explorer primitives — the session-summary bar and the watch-mode
//! banner — so a long-running testing session stops collapsing into one ambiguous spinner
//! and a degraded watch stops hiding why it degraded. A summary bar always names its
//! session mode, its exact selection, its target/environment, its running/backlog counts,
//! its retry lineage, and its current watch state; a watch banner always names its watch
//! fidelity in the controlled `live` / `reduced` / `polling` / `unavailable` vocabulary,
//! explains why fidelity dropped, preserves the last successful cycle time, and exposes the
//! recover and pause actions.
//!
//! Aureline's frozen test-explorer / watch / triage component matrix
//! ([`crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix`])
//! names the session-summary bar and the watch-mode banner as two governed component
//! families and freezes their controlled vocabulary — the session outcomes, the attempt
//! lineage kinds, the watch fidelity states, the watch degrade reasons, the test target
//! classes and environment lanes, plus the surface families, the deployment lines, the
//! consumer surfaces, the accessibility routes, the qualification classes, and the
//! downgrade triggers. This module *implements* that contract as two reusable resolvers so
//! a user can tell — from the summary bar alone — the session mode, the exact selection,
//! the target/environment shorthand, the running/backlog counts, the retry lineage, and the
//! current watch state, and — from the watch banner alone — whether watch is live, reduced,
//! polling, or unavailable, why it degraded, when it last completed a successful cycle, and
//! how to recover or pause it. Above all, the four distinct kinds of pending work —
//! discovering tests, executing them, draining a watch backlog, and refreshing imported
//! status — never share one generic loading treatment, and a degraded watch reads with the
//! same controlled vocabulary in the tree, the status bar, the support/export packet, and
//! the triage consumers.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_session_summary_bar`] — takes one session's mode, activity phase, exact
//!    selection scope, session outcome, target class, environment lane, attempt lineage,
//!    current watch fidelity, running / backlog / retry counts, opaque selection label, and
//!    opaque session identity, and produces one [`M5ResolvedSessionSummaryBar`] carrying the
//!    derived session posture (a discovering, executing, watch-backlog, imported-refresh, or
//!    settled session — one distinct posture per activity phase so no two phases collapse
//!    into one spinner), whether the session is still in progress, whether its exact
//!    selection can be rerun, whether its watch state is degraded, and the bounded
//!    reveal-details / rerun-exact-selection / cancel / open-watch-banner / export actions.
//!    It never masks the session outcome, never leaves the attempt lineage unstated, never
//!    collapses a distinct activity phase into a generic loading state, and never hides a
//!    degraded watch behind a still-green summary.
//! 2. [`resolve_watch_mode_banner`] — takes one watch's fidelity state, optional degrade
//!    reason, opaque last-successful-cycle label, watch backlog count, opaque watch label,
//!    and opaque watch identity, and produces one [`M5ResolvedWatchModeBanner`] carrying the
//!    derived watch posture (a live, reduced, polling, reconnecting, paused, or unavailable
//!    watch — the frozen controlled vocabulary), whether watch is degraded, whether it may
//!    be recovered, whether it may be paused, whether it explains its degradation, and the
//!    bounded reveal-details / recover / pause / export actions. It never invents an
//!    alternate label for a degraded watch, never hides why fidelity dropped, and never
//!    drops the last successful cycle a triage or support consumer would show.
//!
//! A single parity matrix — [`M5SessionWatchStatusPacket`] — binds one row per claimed M5
//! status consumer (the test-explorer status bar, the editor status bar, the run-panel
//! status, the headless/CLI status, and the session/watch report export) to the shared
//! session and watch anatomy, the same session outcomes, activity phases, selection scopes,
//! session postures, watch fidelity states, degrade reasons, watch postures, bounded
//! actions, export fields, and non-visual accessibility routes, so the session and watch
//! vocabulary stays identical across the tree, status, headless/export, and triage
//! consumers — the acceptance-criterion parity that keeps a watch degradation visible
//! everywhere with one vocabulary.
//!
//! The session outcome ([`M5TestSessionOutcome`]), attempt lineage kind
//! ([`M5AttemptLineageKind`]), watch fidelity state ([`M5WatchFidelityState`]), watch
//! degrade reason ([`M5WatchDegradeReason`]), target class ([`M5TestTargetClass`]),
//! environment lane ([`M5TestEnvironmentLane`]), surface family ([`M5TestSurfaceFamily`]),
//! deployment line ([`M5TestDeploymentLine`]), consumer surface ([`M5TestConsumerSurface`]),
//! accessibility route ([`M5TestAccessibilityRoute`]), qualification class
//! ([`M5TestQualificationClass`]), and downgrade trigger ([`M5TestDowngradeTrigger`]) are
//! reused verbatim from the frozen matrix. This module mints new vocabulary only for what
//! that matrix left implicit about the two status components themselves: their status
//! consumers, the session mode, the session activity phase, the exact selection scope, the
//! two derived postures, the two bounded action sets, the two anatomies, and the two export
//! field sets. No M5 test surface invents a second session-bar or watch-banner grammar.
//!
//! Raw log bodies, pasted paths, credentials, and private endpoints stay outside the export
//! boundary; every selection label, watch label, and identity is carried only as an opaque,
//! export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_session_watch_status_headless_cli_status_beta_narrowed,
    seeded_m5_session_watch_status_packet,
    seeded_m5_session_watch_status_run_panel_status_preview_narrowed,
    M5_SESSION_WATCH_STATUS_PACKET_ID,
};

// The session outcome, attempt lineage kind, watch fidelity state, watch degrade reason,
// target class, environment lane, surface family, deployment line, consumer surface,
// accessibility route, qualification class, and downgrade triggers are frozen once, in the
// test-explorer / watch / triage component matrix. These primitives reuse them verbatim so
// they never invent parallel session or watch vocabulary.
pub use crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix::{
    M5AttemptLineageKind, M5TestAccessibilityRoute, M5TestConsumerSurface, M5TestDeploymentLine,
    M5TestDowngradeTrigger, M5TestEnvironmentLane, M5TestQualificationClass, M5TestSessionOutcome,
    M5TestSurfaceFamily, M5TestTargetClass, M5WatchDegradeReason, M5WatchFidelityState,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5SessionWatchStatusPacket`].
pub const M5_SESSION_WATCH_STATUS_RECORD_KIND: &str =
    "implement_m5_session_summary_bars_and_watch_mode_banners_with_exact_selection_running_backlog_retry_counts_live_reduced_polling_unavailable_state_last_successful_cycle_and_recover_pause_truth_across_claimed_m5_test_lanes";

/// Schema version for M5 session-summary / watch-banner records.
pub const M5_SESSION_WATCH_STATUS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the session-summary-bar boundary schema (the canonical packet
/// schema).
pub const M5_SESSION_WATCH_STATUS_SESSION_SCHEMA_REF: &str =
    "schemas/ui/m5-test-session-summary-bar.schema.json";

/// Repo-relative path of the watch-mode-banner companion schema.
pub const M5_SESSION_WATCH_STATUS_WATCH_SCHEMA_REF: &str =
    "schemas/ui/m5-test-watch-mode-banner.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SESSION_WATCH_STATUS_DOC_REF: &str =
    "docs/testing/m5_session_summary_watch_banner_primitive.md";

/// Repo-relative path of the frozen test-explorer / watch / triage component matrix these
/// primitives narrow from.
pub const M5_SESSION_WATCH_STATUS_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-test-explorer-watch-triage-component-matrix.schema.json";

/// Repo-relative path of the test-session contract the summary bar binds its
/// mode / selection / outcome truth against.
pub const M5_SESSION_WATCH_STATUS_TEST_SESSION_REF: &str =
    "schemas/testing/test_session.schema.json";

/// Repo-relative path of the watch-state contract the watch banner binds its fidelity /
/// degrade-reason / last-cycle truth against.
pub const M5_SESSION_WATCH_STATUS_WATCH_STATE_REF: &str = "schemas/testing/watch_state.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SESSION_WATCH_STATUS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-session-summary-watch-banner-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SESSION_WATCH_STATUS_ARTIFACT_REF: &str =
    "artifacts/release/m5-session-summary-watch-banner-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SESSION_WATCH_STATUS_CSV_REF: &str =
    "artifacts/release/m5-session-summary-watch-banner-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SESSION_WATCH_STATUS_REPORT_REF: &str =
    "artifacts/design/m5-session-summary-watch-banner-primitive.md";

/// One claimed M5 status consumer that renders the shared session-summary bar and
/// watch-mode banner. These are the consumers the acceptance criteria name — the
/// test-explorer status bar, the editor status bar, the run-panel status, the headless/CLI
/// status, and the session/watch report export — so the same status grammar works across
/// every claimed test lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionWatchConsumerSurface {
    /// The test-explorer status bar surface.
    TestExplorerStatusBar,
    /// The editor status-bar surface.
    EditorStatusBar,
    /// The run-panel status surface.
    RunPanelStatus,
    /// The headless / CLI status surface.
    HeadlessCliStatus,
    /// The session / watch report export surface.
    SessionWatchReportExport,
}

impl M5SessionWatchConsumerSurface {
    /// Every claimed status consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TestExplorerStatusBar,
        Self::EditorStatusBar,
        Self::RunPanelStatus,
        Self::HeadlessCliStatus,
        Self::SessionWatchReportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestExplorerStatusBar => "test_explorer_status_bar",
            Self::EditorStatusBar => "editor_status_bar",
            Self::RunPanelStatus => "run_panel_status",
            Self::HeadlessCliStatus => "headless_cli_status",
            Self::SessionWatchReportExport => "session_watch_report_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::TestExplorerStatusBar => "Test Explorer Status Bar",
            Self::EditorStatusBar => "Editor Status Bar",
            Self::RunPanelStatus => "Run Panel Status",
            Self::HeadlessCliStatus => "Headless / CLI Status",
            Self::SessionWatchReportExport => "Session / Watch Report Export",
        }
    }
}

/// Controlled session mode a summary bar shows, so a bar never leaves implicit whether the
/// session is a one-shot run, a watch loop, a debug session, a coverage run, an imported
/// replay, or a scheduled run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionMode {
    /// A one-shot run of the selection.
    RunOnceSession,
    /// A watch loop that reruns on change.
    WatchSession,
    /// A debug session.
    DebugSession,
    /// A coverage run.
    CoverageSession,
    /// A replay of imported results.
    ImportedReplaySession,
    /// A scheduled / CI-triggered run.
    ScheduledSession,
}

impl M5SessionMode {
    /// Every session mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RunOnceSession,
        Self::WatchSession,
        Self::DebugSession,
        Self::CoverageSession,
        Self::ImportedReplaySession,
        Self::ScheduledSession,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunOnceSession => "run_once_session",
            Self::WatchSession => "watch_session",
            Self::DebugSession => "debug_session",
            Self::CoverageSession => "coverage_session",
            Self::ImportedReplaySession => "imported_replay_session",
            Self::ScheduledSession => "scheduled_session",
        }
    }

    /// True when this session watches for changes and therefore always links its watch
    /// banner.
    pub const fn tracks_watch(self) -> bool {
        matches!(self, Self::WatchSession)
    }
}

/// Controlled session activity phase — the distinct kind of pending work a summary bar is
/// reporting, so discovery, execution, watch-backlog drain, and imported-status refresh
/// never share one generic loading treatment. This is the acceptance-criterion axis that
/// forbids a single ambiguous spinner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionActivityPhase {
    /// Discovering / enumerating tests.
    DiscoveringTests,
    /// Executing the selected tests.
    ExecutingTests,
    /// Draining a watch backlog after file changes.
    ProcessingWatchBacklog,
    /// Refreshing imported-result status from an external run.
    RefreshingImportedStatus,
    /// The session has settled and is no longer pending.
    SettledComplete,
}

impl M5SessionActivityPhase {
    /// Every activity phase, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DiscoveringTests,
        Self::ExecutingTests,
        Self::ProcessingWatchBacklog,
        Self::RefreshingImportedStatus,
        Self::SettledComplete,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiscoveringTests => "discovering_tests",
            Self::ExecutingTests => "executing_tests",
            Self::ProcessingWatchBacklog => "processing_watch_backlog",
            Self::RefreshingImportedStatus => "refreshing_imported_status",
            Self::SettledComplete => "settled_complete",
        }
    }

    /// True when this phase is still pending work (not settled).
    pub const fn is_pending(self) -> bool {
        !matches!(self, Self::SettledComplete)
    }
}

/// Controlled selection scope — the exact selection a summary bar is running, so a bar shows
/// what will actually rerun rather than a vague "running tests".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionSelectionScope {
    /// The whole suite.
    WholeSuiteSelection,
    /// A selected subset of tests.
    SelectedSubsetSelection,
    /// Only the previously failed tests.
    FailedOnlySelection,
    /// A single test case.
    SingleCaseSelection,
    /// Tests changed since a baseline.
    ChangedSinceSelection,
    /// An imported replay selection.
    ImportedReplaySelection,
}

impl M5SessionSelectionScope {
    /// Every selection scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WholeSuiteSelection,
        Self::SelectedSubsetSelection,
        Self::FailedOnlySelection,
        Self::SingleCaseSelection,
        Self::ChangedSinceSelection,
        Self::ImportedReplaySelection,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WholeSuiteSelection => "whole_suite_selection",
            Self::SelectedSubsetSelection => "selected_subset_selection",
            Self::FailedOnlySelection => "failed_only_selection",
            Self::SingleCaseSelection => "single_case_selection",
            Self::ChangedSinceSelection => "changed_since_selection",
            Self::ImportedReplaySelection => "imported_replay_selection",
        }
    }
}

/// The derived posture of a session-summary bar — one distinct posture per activity phase so
/// no two kinds of pending work collapse into one loading treatment. Computed 1:1 from the
/// activity phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionSummaryPosture {
    /// The session is discovering tests.
    DiscoveringSession,
    /// The session is executing tests.
    ExecutingSession,
    /// The session is draining a watch backlog.
    WatchBacklogSession,
    /// The session is refreshing imported status.
    ImportedRefreshSession,
    /// The session has settled.
    SettledSession,
}

impl M5SessionSummaryPosture {
    /// Every session posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DiscoveringSession,
        Self::ExecutingSession,
        Self::WatchBacklogSession,
        Self::ImportedRefreshSession,
        Self::SettledSession,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiscoveringSession => "discovering_session",
            Self::ExecutingSession => "executing_session",
            Self::WatchBacklogSession => "watch_backlog_session",
            Self::ImportedRefreshSession => "imported_refresh_session",
            Self::SettledSession => "settled_session",
        }
    }

    /// True when the session is still doing pending work of some kind.
    pub const fn is_pending(self) -> bool {
        !matches!(self, Self::SettledSession)
    }
}

/// One bounded action a session-summary bar offers, so a bar never hides its reveal /
/// rerun / cancel / open-watch-banner / export affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionSummaryAction {
    /// Reveal the session's mode, exact selection, counts, retry lineage, and watch state.
    RevealSessionDetails,
    /// Rerun the exact selection this session ran.
    RerunExactSelection,
    /// Cancel the running session.
    CancelRunningSession,
    /// Open the watch-mode banner behind this session.
    OpenWatchBanner,
    /// Export the session summary as test evidence.
    ExportSession,
}

impl M5SessionSummaryAction {
    /// Every session action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealSessionDetails,
        Self::RerunExactSelection,
        Self::CancelRunningSession,
        Self::OpenWatchBanner,
        Self::ExportSession,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealSessionDetails => "reveal_session_details",
            Self::RerunExactSelection => "rerun_exact_selection",
            Self::CancelRunningSession => "cancel_running_session",
            Self::OpenWatchBanner => "open_watch_banner",
            Self::ExportSession => "export_session",
        }
    }
}

/// Controlled session-bar anatomy part. The parts in
/// [`M5SessionSummaryAnatomyPart::MANDATORY`] are required on every bar so the session mode,
/// exact selection, running/backlog counts, retry state, and current watch state are never
/// hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionSummaryAnatomyPart {
    /// The session-mode cue.
    SessionModeCue,
    /// The exact-selection cue.
    ExactSelectionCue,
    /// The target / environment shorthand cue.
    TargetEnvironmentCue,
    /// The running / backlog count cue.
    RunningBacklogCountCue,
    /// The retry-state (attempt lineage) cue.
    RetryStateCue,
    /// The current watch-state cue.
    WatchStateCue,
    /// The session-outcome cue.
    OutcomeCue,
    /// The activity-phase cue.
    ActivityPhaseCue,
}

impl M5SessionSummaryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SessionModeCue,
        Self::ExactSelectionCue,
        Self::TargetEnvironmentCue,
        Self::RunningBacklogCountCue,
        Self::RetryStateCue,
        Self::WatchStateCue,
        Self::OutcomeCue,
        Self::ActivityPhaseCue,
    ];

    /// The anatomy parts every session bar must render.
    pub const MANDATORY: [Self; 5] = [
        Self::SessionModeCue,
        Self::ExactSelectionCue,
        Self::RunningBacklogCountCue,
        Self::RetryStateCue,
        Self::WatchStateCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionModeCue => "session_mode_cue",
            Self::ExactSelectionCue => "exact_selection_cue",
            Self::TargetEnvironmentCue => "target_environment_cue",
            Self::RunningBacklogCountCue => "running_backlog_count_cue",
            Self::RetryStateCue => "retry_state_cue",
            Self::WatchStateCue => "watch_state_cue",
            Self::OutcomeCue => "outcome_cue",
            Self::ActivityPhaseCue => "activity_phase_cue",
        }
    }
}

/// A field the session export carries so session-bar truth is reconstructable. The fields in
/// [`M5SessionSummaryExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionSummaryExportField {
    /// The session mode.
    SessionMode,
    /// The exact selection scope.
    SelectionScope,
    /// The session outcome.
    SessionOutcome,
    /// The activity phase.
    ActivityPhase,
    /// The running count.
    RunningCount,
    /// The backlog count.
    BacklogCount,
    /// The retry count.
    RetryCount,
    /// The current watch fidelity.
    WatchFidelity,
    /// The derived session posture.
    SessionPosture,
}

impl M5SessionSummaryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SessionMode,
        Self::SelectionScope,
        Self::SessionOutcome,
        Self::ActivityPhase,
        Self::RunningCount,
        Self::BacklogCount,
        Self::RetryCount,
        Self::WatchFidelity,
        Self::SessionPosture,
    ];

    /// The export fields every session bar must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::SessionMode,
        Self::SessionOutcome,
        Self::ActivityPhase,
        Self::WatchFidelity,
        Self::SessionPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionMode => "session_mode",
            Self::SelectionScope => "selection_scope",
            Self::SessionOutcome => "session_outcome",
            Self::ActivityPhase => "activity_phase",
            Self::RunningCount => "running_count",
            Self::BacklogCount => "backlog_count",
            Self::RetryCount => "retry_count",
            Self::WatchFidelity => "watch_fidelity",
            Self::SessionPosture => "session_posture",
        }
    }
}

/// The derived posture of a watch-mode banner — the frozen controlled vocabulary, one
/// distinct posture per watch fidelity state, so a degraded watch never borrows an alternate
/// label. Computed 1:1 from the watch fidelity state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WatchBannerPosture {
    /// Live watch with full fidelity.
    LiveWatch,
    /// Reduced-fidelity watch.
    ReducedWatch,
    /// Polling-based watch.
    PollingWatch,
    /// Reconnecting watch.
    ReconnectingWatch,
    /// Watch paused by the user.
    PausedWatch,
    /// Watch unavailable.
    UnavailableWatch,
}

impl M5WatchBannerPosture {
    /// Every watch posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiveWatch,
        Self::ReducedWatch,
        Self::PollingWatch,
        Self::ReconnectingWatch,
        Self::PausedWatch,
        Self::UnavailableWatch,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveWatch => "live_watch",
            Self::ReducedWatch => "reduced_watch",
            Self::PollingWatch => "polling_watch",
            Self::ReconnectingWatch => "reconnecting_watch",
            Self::PausedWatch => "paused_watch",
            Self::UnavailableWatch => "unavailable_watch",
        }
    }

    /// True only for a full-fidelity live watch — the only posture that needs no attention.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::LiveWatch)
    }

    /// The frozen watch-fidelity state this posture maps 1:1 to.
    pub const fn fidelity(self) -> M5WatchFidelityState {
        match self {
            Self::LiveWatch => M5WatchFidelityState::Live,
            Self::ReducedWatch => M5WatchFidelityState::Reduced,
            Self::PollingWatch => M5WatchFidelityState::Polling,
            Self::ReconnectingWatch => M5WatchFidelityState::Reconnecting,
            Self::PausedWatch => M5WatchFidelityState::Paused,
            Self::UnavailableWatch => M5WatchFidelityState::Unavailable,
        }
    }
}

/// One bounded action a watch-mode banner offers, so a banner never hides its reveal /
/// recover / pause / export affordances — the recover and pause actions the implementation
/// requirements name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WatchBannerAction {
    /// Reveal the watch's fidelity, degrade reason, and last successful cycle.
    RevealWatchDetails,
    /// Recover / restore watch toward live fidelity.
    RecoverWatch,
    /// Pause the active watch.
    PauseWatch,
    /// Export the watch state as test evidence.
    ExportWatchState,
}

impl M5WatchBannerAction {
    /// Every watch action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RevealWatchDetails,
        Self::RecoverWatch,
        Self::PauseWatch,
        Self::ExportWatchState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealWatchDetails => "reveal_watch_details",
            Self::RecoverWatch => "recover_watch",
            Self::PauseWatch => "pause_watch",
            Self::ExportWatchState => "export_watch_state",
        }
    }
}

/// Controlled watch-banner anatomy part. The parts in
/// [`M5WatchBannerAnatomyPart::MANDATORY`] are required on every banner so the watch
/// fidelity, degrade reason, last successful cycle, and recover/pause actions are never
/// hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WatchBannerAnatomyPart {
    /// The watch-fidelity cue.
    WatchFidelityCue,
    /// The degrade-reason cue.
    DegradeReasonCue,
    /// The last-successful-cycle cue.
    LastSuccessfulCycleCue,
    /// The watch-backlog cue.
    WatchBacklogCue,
    /// The recover-action cue.
    RecoverActionCue,
    /// The pause-action cue.
    PauseActionCue,
}

impl M5WatchBannerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WatchFidelityCue,
        Self::DegradeReasonCue,
        Self::LastSuccessfulCycleCue,
        Self::WatchBacklogCue,
        Self::RecoverActionCue,
        Self::PauseActionCue,
    ];

    /// The anatomy parts every watch banner must render.
    pub const MANDATORY: [Self; 5] = [
        Self::WatchFidelityCue,
        Self::DegradeReasonCue,
        Self::LastSuccessfulCycleCue,
        Self::RecoverActionCue,
        Self::PauseActionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WatchFidelityCue => "watch_fidelity_cue",
            Self::DegradeReasonCue => "degrade_reason_cue",
            Self::LastSuccessfulCycleCue => "last_successful_cycle_cue",
            Self::WatchBacklogCue => "watch_backlog_cue",
            Self::RecoverActionCue => "recover_action_cue",
            Self::PauseActionCue => "pause_action_cue",
        }
    }
}

/// A field the watch export carries so watch-banner truth is reconstructable. The fields in
/// [`M5WatchBannerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WatchBannerExportField {
    /// The watch fidelity state.
    WatchFidelity,
    /// The degrade reason.
    DegradeReason,
    /// The last successful cycle.
    LastSuccessfulCycle,
    /// The watch backlog count.
    BacklogCount,
    /// The derived watch posture.
    WatchPosture,
    /// The bounded available actions.
    AvailableActions,
}

impl M5WatchBannerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WatchFidelity,
        Self::DegradeReason,
        Self::LastSuccessfulCycle,
        Self::BacklogCount,
        Self::WatchPosture,
        Self::AvailableActions,
    ];

    /// The export fields every watch banner must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::WatchFidelity,
        Self::DegradeReason,
        Self::LastSuccessfulCycle,
        Self::WatchPosture,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WatchFidelity => "watch_fidelity",
            Self::DegradeReason => "degrade_reason",
            Self::LastSuccessfulCycle => "last_successful_cycle",
            Self::BacklogCount => "backlog_count",
            Self::WatchPosture => "watch_posture",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// True when a watch fidelity state is a genuine degradation that must carry a degrade
/// reason. A paused watch is user-initiated, not a degradation; a live watch is full
/// fidelity.
pub const fn watch_fidelity_is_degraded(fidelity: M5WatchFidelityState) -> bool {
    matches!(
        fidelity,
        M5WatchFidelityState::Reduced
            | M5WatchFidelityState::Polling
            | M5WatchFidelityState::Unavailable
            | M5WatchFidelityState::Reconnecting
    )
}

// ---- session-summary-bar resolver ---------------------------------------

/// The full input to the session-summary-bar resolver for one session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SessionSummaryResolutionInput {
    /// The session mode.
    pub session_mode: M5SessionMode,
    /// The current activity phase.
    pub activity_phase: M5SessionActivityPhase,
    /// The exact selection scope.
    pub selection_scope: M5SessionSelectionScope,
    /// The overall session outcome.
    pub session_outcome: M5TestSessionOutcome,
    /// The test target class.
    pub target_class: M5TestTargetClass,
    /// The test environment lane.
    pub environment_lane: M5TestEnvironmentLane,
    /// The attempt lineage kind behind the retry state.
    pub attempt_lineage: M5AttemptLineageKind,
    /// The current watch fidelity state.
    pub watch_fidelity: M5WatchFidelityState,
    /// The number of tests currently running.
    pub running_count: u32,
    /// The number of tests queued in the backlog.
    pub backlog_count: u32,
    /// The number of retries in the current lineage.
    pub retry_count: u32,
    /// The opaque user-facing exact-selection label (must be non-empty).
    pub selection_label: String,
    /// The opaque stable session identity (must be non-empty).
    pub session_identity_ref: String,
}

/// The resolved session-summary-bar truth for one session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSessionSummaryBar {
    /// The session mode.
    pub session_mode: M5SessionMode,
    /// The activity phase.
    pub activity_phase: M5SessionActivityPhase,
    /// The exact selection scope.
    pub selection_scope: M5SessionSelectionScope,
    /// The session outcome.
    pub session_outcome: M5TestSessionOutcome,
    /// The test target class.
    pub target_class: M5TestTargetClass,
    /// The test environment lane.
    pub environment_lane: M5TestEnvironmentLane,
    /// The attempt lineage kind.
    pub attempt_lineage: M5AttemptLineageKind,
    /// The current watch fidelity state.
    pub watch_fidelity: M5WatchFidelityState,
    /// The running count, preserved from the input.
    pub running_count: u32,
    /// The backlog count, preserved from the input.
    pub backlog_count: u32,
    /// The retry count, preserved from the input.
    pub retry_count: u32,
    /// The opaque selection label, preserved exactly from the input.
    pub selection_label: String,
    /// The opaque stable session identity, preserved exactly from the input.
    pub session_identity_ref: String,
    /// The derived session posture.
    pub session_posture: M5SessionSummaryPosture,
    /// The bounded actions this bar offers.
    pub available_actions: Vec<M5SessionSummaryAction>,
    /// True when the session is still doing pending work.
    pub is_in_progress: bool,
    /// True when the exact selection can be rerun.
    pub can_rerun: bool,
    /// True when the session is still running and can be cancelled.
    pub can_cancel: bool,
    /// True when there is a watch backlog.
    pub has_backlog: bool,
    /// True when the current lineage carries retries.
    pub has_retries: bool,
    /// True when the current watch state is degraded — kept visible on the summary bar so a
    /// watch degradation stays visible in the status consumer.
    pub watch_is_degraded: bool,
    /// True when the bar reports pending work (never a settled session).
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_session_summary_bar`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SessionSummaryResolutionError {
    /// The selection label was empty.
    EmptySelectionLabel,
    /// The session identity ref was empty.
    EmptySessionIdentity,
    /// A session descriptor carried forbidden material.
    ForbiddenSessionMaterial,
}

impl M5SessionSummaryResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySelectionLabel => "empty_selection_label",
            Self::EmptySessionIdentity => "empty_session_identity",
            Self::ForbiddenSessionMaterial => "forbidden_session_material",
        }
    }
}

impl fmt::Display for M5SessionSummaryResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "session summary bar resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SessionSummaryResolutionError {}

/// Resolves one session-summary bar from its declared session state.
///
/// The derived session posture is 1:1 with the activity phase — discovering, executing,
/// watch-backlog, imported-refresh, or settled — so the four distinct kinds of pending work
/// never share one generic loading treatment. The exact selection can be rerun only once the
/// session has settled; the session can be cancelled only while it is still doing pending
/// work; the watch banner is offered whenever the session watches for changes; reveal and
/// export are always offered. The current watch fidelity is always carried and its
/// degradation always kept visible, so a degraded watch never hides behind a still-green
/// summary.
pub fn resolve_session_summary_bar(
    input: &M5SessionSummaryResolutionInput,
) -> Result<M5ResolvedSessionSummaryBar, M5SessionSummaryResolutionError> {
    if input.selection_label.trim().is_empty() {
        return Err(M5SessionSummaryResolutionError::EmptySelectionLabel);
    }
    if input.session_identity_ref.trim().is_empty() {
        return Err(M5SessionSummaryResolutionError::EmptySessionIdentity);
    }
    if value_repr_is_forbidden(&input.selection_label)
        || value_repr_is_forbidden(&input.session_identity_ref)
    {
        return Err(M5SessionSummaryResolutionError::ForbiddenSessionMaterial);
    }

    let session_posture = derive_session_posture(input.activity_phase);
    let is_in_progress = input.activity_phase.is_pending();
    let can_rerun = !is_in_progress;
    let can_cancel = is_in_progress;
    let available_actions =
        derive_session_actions(can_rerun, can_cancel, input.session_mode.tracks_watch());

    Ok(M5ResolvedSessionSummaryBar {
        session_mode: input.session_mode,
        activity_phase: input.activity_phase,
        selection_scope: input.selection_scope,
        session_outcome: input.session_outcome,
        target_class: input.target_class,
        environment_lane: input.environment_lane,
        attempt_lineage: input.attempt_lineage,
        watch_fidelity: input.watch_fidelity,
        running_count: input.running_count,
        backlog_count: input.backlog_count,
        retry_count: input.retry_count,
        selection_label: input.selection_label.clone(),
        session_identity_ref: input.session_identity_ref.clone(),
        session_posture,
        available_actions,
        is_in_progress,
        can_rerun,
        can_cancel,
        has_backlog: input.backlog_count > 0,
        has_retries: input.retry_count > 0,
        watch_is_degraded: watch_fidelity_is_degraded(input.watch_fidelity),
        needs_attention: session_posture.is_pending(),
    })
}

/// The 1:1 activity-phase → session-posture map.
fn derive_session_posture(activity_phase: M5SessionActivityPhase) -> M5SessionSummaryPosture {
    match activity_phase {
        M5SessionActivityPhase::DiscoveringTests => M5SessionSummaryPosture::DiscoveringSession,
        M5SessionActivityPhase::ExecutingTests => M5SessionSummaryPosture::ExecutingSession,
        M5SessionActivityPhase::ProcessingWatchBacklog => {
            M5SessionSummaryPosture::WatchBacklogSession
        }
        M5SessionActivityPhase::RefreshingImportedStatus => {
            M5SessionSummaryPosture::ImportedRefreshSession
        }
        M5SessionActivityPhase::SettledComplete => M5SessionSummaryPosture::SettledSession,
    }
}

/// Derives the bounded session-action set from the rerun / cancel / watch signals.
fn derive_session_actions(
    can_rerun: bool,
    can_cancel: bool,
    tracks_watch: bool,
) -> Vec<M5SessionSummaryAction> {
    use M5SessionSummaryAction as Action;
    let mut actions = vec![Action::RevealSessionDetails];
    if can_rerun {
        actions.push(Action::RerunExactSelection);
    }
    if can_cancel {
        actions.push(Action::CancelRunningSession);
    }
    if tracks_watch {
        actions.push(Action::OpenWatchBanner);
    }
    actions.push(Action::ExportSession);
    actions
}

// ---- watch-mode-banner resolver -----------------------------------------

/// The full input to the watch-mode-banner resolver for one watch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WatchBannerResolutionInput {
    /// The watch fidelity state.
    pub watch_fidelity: M5WatchFidelityState,
    /// The reason fidelity dropped, when the watch is degraded.
    pub degrade_reason: Option<M5WatchDegradeReason>,
    /// The opaque last-successful-cycle label (must be non-empty).
    pub last_successful_cycle: String,
    /// The number of changes queued in the watch backlog.
    pub backlog_count: u32,
    /// The opaque user-facing watch label (must be non-empty).
    pub watch_label: String,
    /// The opaque stable watch identity (must be non-empty).
    pub watch_identity_ref: String,
}

/// The resolved watch-mode-banner truth for one watch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWatchModeBanner {
    /// The watch fidelity state.
    pub watch_fidelity: M5WatchFidelityState,
    /// The degrade reason, preserved from the input.
    pub degrade_reason: Option<M5WatchDegradeReason>,
    /// The last successful cycle, preserved exactly from the input.
    pub last_successful_cycle: String,
    /// The watch backlog count, preserved from the input.
    pub backlog_count: u32,
    /// The opaque watch label, preserved exactly from the input.
    pub watch_label: String,
    /// The opaque stable watch identity, preserved exactly from the input.
    pub watch_identity_ref: String,
    /// The derived watch posture.
    pub watch_posture: M5WatchBannerPosture,
    /// The bounded actions this banner offers.
    pub available_actions: Vec<M5WatchBannerAction>,
    /// True when watch is degraded (reduced / polling / reconnecting / unavailable).
    pub is_degraded: bool,
    /// True when watch may be recovered toward live fidelity.
    pub can_recover: bool,
    /// True when the active watch may be paused.
    pub can_pause: bool,
    /// True when a degraded watch explains why fidelity dropped.
    pub explains_degradation: bool,
    /// True when the banner preserves a last successful cycle time.
    pub preserves_last_successful_cycle: bool,
    /// True when the banner must degrade visibly before it reads as a full live watch.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_watch_mode_banner`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5WatchBannerResolutionError {
    /// The last-successful-cycle label was empty.
    EmptyLastSuccessfulCycle,
    /// The watch label was empty.
    EmptyWatchLabel,
    /// The watch identity ref was empty.
    EmptyWatchIdentity,
    /// A degraded watch did not explain why fidelity dropped.
    MissingDegradeReason,
    /// A watch descriptor carried forbidden material.
    ForbiddenWatchMaterial,
}

impl M5WatchBannerResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyLastSuccessfulCycle => "empty_last_successful_cycle",
            Self::EmptyWatchLabel => "empty_watch_label",
            Self::EmptyWatchIdentity => "empty_watch_identity",
            Self::MissingDegradeReason => "missing_degrade_reason",
            Self::ForbiddenWatchMaterial => "forbidden_watch_material",
        }
    }
}

impl fmt::Display for M5WatchBannerResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "watch mode banner resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5WatchBannerResolutionError {}

/// Resolves one watch-mode banner from its declared watch state.
///
/// The derived watch posture is 1:1 with the frozen watch fidelity state — live, reduced,
/// polling, reconnecting, paused, or unavailable — so a degraded watch never borrows an
/// alternate label. A degraded watch (reduced / polling / reconnecting / unavailable) MUST
/// carry a degrade reason, or resolution fails: the banner never hides why fidelity dropped.
/// The last successful cycle is always preserved. Recover is offered whenever watch is not
/// already live; pause is offered whenever there is an active watch to pause (never for an
/// already-paused or unavailable watch); reveal and export are always offered.
pub fn resolve_watch_mode_banner(
    input: &M5WatchBannerResolutionInput,
) -> Result<M5ResolvedWatchModeBanner, M5WatchBannerResolutionError> {
    if input.last_successful_cycle.trim().is_empty() {
        return Err(M5WatchBannerResolutionError::EmptyLastSuccessfulCycle);
    }
    if input.watch_label.trim().is_empty() {
        return Err(M5WatchBannerResolutionError::EmptyWatchLabel);
    }
    if input.watch_identity_ref.trim().is_empty() {
        return Err(M5WatchBannerResolutionError::EmptyWatchIdentity);
    }
    let is_degraded = watch_fidelity_is_degraded(input.watch_fidelity);
    if is_degraded && input.degrade_reason.is_none() {
        return Err(M5WatchBannerResolutionError::MissingDegradeReason);
    }
    if value_repr_is_forbidden(&input.last_successful_cycle)
        || value_repr_is_forbidden(&input.watch_label)
        || value_repr_is_forbidden(&input.watch_identity_ref)
    {
        return Err(M5WatchBannerResolutionError::ForbiddenWatchMaterial);
    }

    let watch_posture = derive_watch_posture(input.watch_fidelity);
    let can_recover = !watch_posture.is_live();
    let can_pause = watch_can_be_paused(input.watch_fidelity);
    let available_actions = derive_watch_actions(can_recover, can_pause);

    Ok(M5ResolvedWatchModeBanner {
        watch_fidelity: input.watch_fidelity,
        degrade_reason: input.degrade_reason,
        last_successful_cycle: input.last_successful_cycle.clone(),
        backlog_count: input.backlog_count,
        watch_label: input.watch_label.clone(),
        watch_identity_ref: input.watch_identity_ref.clone(),
        watch_posture,
        available_actions,
        is_degraded,
        can_recover,
        can_pause,
        explains_degradation: !is_degraded || input.degrade_reason.is_some(),
        preserves_last_successful_cycle: !input.last_successful_cycle.trim().is_empty(),
        needs_attention: !watch_posture.is_live(),
    })
}

/// The 1:1 watch-fidelity → watch-posture map.
fn derive_watch_posture(fidelity: M5WatchFidelityState) -> M5WatchBannerPosture {
    match fidelity {
        M5WatchFidelityState::Live => M5WatchBannerPosture::LiveWatch,
        M5WatchFidelityState::Reduced => M5WatchBannerPosture::ReducedWatch,
        M5WatchFidelityState::Polling => M5WatchBannerPosture::PollingWatch,
        M5WatchFidelityState::Unavailable => M5WatchBannerPosture::UnavailableWatch,
        M5WatchFidelityState::Paused => M5WatchBannerPosture::PausedWatch,
        M5WatchFidelityState::Reconnecting => M5WatchBannerPosture::ReconnectingWatch,
    }
}

/// True when there is an active watch that can be paused — a live, reduced, polling, or
/// reconnecting watch. An already-paused or unavailable watch has nothing to pause.
fn watch_can_be_paused(fidelity: M5WatchFidelityState) -> bool {
    matches!(
        fidelity,
        M5WatchFidelityState::Live
            | M5WatchFidelityState::Reduced
            | M5WatchFidelityState::Polling
            | M5WatchFidelityState::Reconnecting
    )
}

/// Derives the bounded watch-action set from the recover / pause signals.
fn derive_watch_actions(can_recover: bool, can_pause: bool) -> Vec<M5WatchBannerAction> {
    use M5WatchBannerAction as Action;
    let mut actions = vec![Action::RevealWatchDetails];
    if can_recover {
        actions.push(Action::RecoverWatch);
    }
    if can_pause {
        actions.push(Action::PauseWatch);
    }
    actions.push(Action::ExportWatchState);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked session-summary-bar resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SessionSummaryResolutionCase {
    /// The resolver input.
    pub input: M5SessionSummaryResolutionInput,
    /// The resolved truth. Must equal `resolve_session_summary_bar(&input)`.
    pub resolved: M5ResolvedSessionSummaryBar,
}

impl M5SessionSummaryResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5SessionSummaryResolutionInput) -> Self {
        let resolved = resolve_session_summary_bar(&input).expect("seed session case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_session_summary_bar(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved session identity preserves the input identity exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.session_identity_ref == self.input.session_identity_ref
            && self.resolved.selection_label == self.input.selection_label
    }
}

/// One worked watch-mode-banner resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WatchBannerResolutionCase {
    /// The resolver input.
    pub input: M5WatchBannerResolutionInput,
    /// The resolved truth. Must equal `resolve_watch_mode_banner(&input)`.
    pub resolved: M5ResolvedWatchModeBanner,
}

impl M5WatchBannerResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5WatchBannerResolutionInput) -> Self {
        let resolved = resolve_watch_mode_banner(&input).expect("seed watch case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_watch_mode_banner(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved watch identity preserves the input identity exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.watch_identity_ref == self.input.watch_identity_ref
            && self.resolved.watch_label == self.input.watch_label
    }
}

/// One row in the primitive matrix: one status consumer bound to the shared session and
/// watch anatomy, session outcomes, activity phases, selection scopes, session postures,
/// watch fidelity states, degrade reasons, watch postures, bounded actions, export fields,
/// and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SessionWatchConsumerRow {
    /// Status consumer family.
    pub consumer_surface: M5SessionWatchConsumerSurface,
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
    /// Session-bar anatomy parts this consumer renders (must include the mandatory parts).
    pub session_anatomy_parts: Vec<M5SessionSummaryAnatomyPart>,
    /// Watch-banner anatomy parts this consumer renders (must include the mandatory parts).
    pub watch_anatomy_parts: Vec<M5WatchBannerAnatomyPart>,
    /// Session modes this consumer distinguishes.
    pub session_modes: Vec<M5SessionMode>,
    /// Activity phases this consumer distinguishes.
    pub activity_phases: Vec<M5SessionActivityPhase>,
    /// Selection scopes this consumer distinguishes.
    pub selection_scopes: Vec<M5SessionSelectionScope>,
    /// Session outcomes this consumer distinguishes.
    pub session_outcomes: Vec<M5TestSessionOutcome>,
    /// Session postures this consumer distinguishes.
    pub session_postures: Vec<M5SessionSummaryPosture>,
    /// Attempt lineage kinds this consumer distinguishes.
    pub attempt_lineage_kinds: Vec<M5AttemptLineageKind>,
    /// Watch fidelity states this consumer distinguishes.
    pub watch_fidelity_states: Vec<M5WatchFidelityState>,
    /// Watch degrade reasons this consumer distinguishes.
    pub watch_degrade_reasons: Vec<M5WatchDegradeReason>,
    /// Watch postures this consumer distinguishes.
    pub watch_postures: Vec<M5WatchBannerPosture>,
    /// Bounded session actions this consumer offers.
    pub session_actions: Vec<M5SessionSummaryAction>,
    /// Bounded watch actions this consumer offers.
    pub watch_actions: Vec<M5WatchBannerAction>,
    /// Session export fields this consumer carries (must include the mandatory fields).
    pub session_export_fields: Vec<M5SessionSummaryExportField>,
    /// Watch export fields this consumer carries (must include the mandatory fields).
    pub watch_export_fields: Vec<M5WatchBannerExportField>,
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
    /// Worked session-bar resolutions proving the resolver on this consumer.
    pub session_examples: Vec<M5SessionSummaryResolutionCase>,
    /// Worked watch-banner resolutions proving the resolver on this consumer.
    pub watch_examples: Vec<M5WatchBannerResolutionCase>,
    /// Hard invariant: this consumer never collapses a distinct activity phase into one
    /// generic loading treatment. MUST be `false`.
    pub collapses_activity_into_one_spinner: bool,
    /// Hard invariant: this consumer never leaves the attempt lineage (retry state)
    /// unstated. MUST be `false`.
    pub drops_retry_lineage: bool,
    /// Hard invariant: this consumer never invents an alternate label for a degraded watch.
    /// MUST be `false`.
    pub invents_alternate_watch_label: bool,
    /// Hard invariant: this consumer never hides why watch degraded or the last successful
    /// cycle. MUST be `false`.
    pub hides_watch_degrade_or_last_cycle: bool,
}

impl M5SessionWatchConsumerRow {
    /// True when the row declares every mandatory session anatomy part.
    fn declares_mandatory_session_anatomy(&self) -> bool {
        let present: BTreeSet<M5SessionSummaryAnatomyPart> =
            self.session_anatomy_parts.iter().copied().collect();
        M5SessionSummaryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory watch anatomy part.
    fn declares_mandatory_watch_anatomy(&self) -> bool {
        let present: BTreeSet<M5WatchBannerAnatomyPart> =
            self.watch_anatomy_parts.iter().copied().collect();
        M5WatchBannerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory session export field.
    fn declares_mandatory_session_export(&self) -> bool {
        let present: BTreeSet<M5SessionSummaryExportField> =
            self.session_export_fields.iter().copied().collect();
        M5SessionSummaryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory watch export field.
    fn declares_mandatory_watch_export(&self) -> bool {
        let present: BTreeSet<M5WatchBannerExportField> =
            self.watch_export_fields.iter().copied().collect();
        M5WatchBannerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_activity_into_one_spinner
            && !self.drops_retry_lineage
            && !self.invents_alternate_watch_label
            && !self.hides_watch_degrade_or_last_cycle
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SessionWatchVocabularySet {
    /// Status consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Session-anatomy-part tokens.
    pub session_anatomy_parts: Vec<String>,
    /// Watch-anatomy-part tokens.
    pub watch_anatomy_parts: Vec<String>,
    /// Session-posture tokens.
    pub session_postures: Vec<String>,
    /// Watch-posture tokens.
    pub watch_postures: Vec<String>,
    /// Session-mode tokens.
    pub session_modes: Vec<String>,
    /// Activity-phase tokens.
    pub activity_phases: Vec<String>,
    /// Selection-scope tokens.
    pub selection_scopes: Vec<String>,
    /// Session-action tokens.
    pub session_actions: Vec<String>,
    /// Watch-action tokens.
    pub watch_actions: Vec<String>,
    /// Session-export-field tokens.
    pub session_export_fields: Vec<String>,
    /// Watch-export-field tokens.
    pub watch_export_fields: Vec<String>,
    /// Session-outcome tokens (reused from the frozen matrix).
    pub session_outcomes: Vec<String>,
    /// Attempt-lineage tokens (reused from the frozen matrix).
    pub attempt_lineage_kinds: Vec<String>,
    /// Watch-fidelity tokens (reused from the frozen matrix).
    pub watch_fidelity_states: Vec<String>,
    /// Watch-degrade-reason tokens (reused from the frozen matrix).
    pub watch_degrade_reasons: Vec<String>,
    /// Target-class tokens (reused from the frozen matrix).
    pub target_classes: Vec<String>,
    /// Environment-lane tokens (reused from the frozen matrix).
    pub environment_lanes: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5SessionWatchVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5SessionWatchConsumerSurface::ALL, |v| v.as_str()),
            session_anatomy_parts: tokens(&M5SessionSummaryAnatomyPart::ALL, |v| v.as_str()),
            watch_anatomy_parts: tokens(&M5WatchBannerAnatomyPart::ALL, |v| v.as_str()),
            session_postures: tokens(&M5SessionSummaryPosture::ALL, |v| v.as_str()),
            watch_postures: tokens(&M5WatchBannerPosture::ALL, |v| v.as_str()),
            session_modes: tokens(&M5SessionMode::ALL, |v| v.as_str()),
            activity_phases: tokens(&M5SessionActivityPhase::ALL, |v| v.as_str()),
            selection_scopes: tokens(&M5SessionSelectionScope::ALL, |v| v.as_str()),
            session_actions: tokens(&M5SessionSummaryAction::ALL, |v| v.as_str()),
            watch_actions: tokens(&M5WatchBannerAction::ALL, |v| v.as_str()),
            session_export_fields: tokens(&M5SessionSummaryExportField::ALL, |v| v.as_str()),
            watch_export_fields: tokens(&M5WatchBannerExportField::ALL, |v| v.as_str()),
            session_outcomes: tokens(&M5TestSessionOutcome::ALL, |v| v.as_str()),
            attempt_lineage_kinds: tokens(&M5AttemptLineageKind::ALL, |v| v.as_str()),
            watch_fidelity_states: tokens(&M5WatchFidelityState::ALL, |v| v.as_str()),
            watch_degrade_reasons: tokens(&M5WatchDegradeReason::ALL, |v| v.as_str()),
            target_classes: tokens(&M5TestTargetClass::ALL, |v| v.as_str()),
            environment_lanes: tokens(&M5TestEnvironmentLane::ALL, |v| v.as_str()),
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
pub struct M5SessionWatchGovernanceReview {
    /// The summary bar shows its session mode and exact selection.
    pub bar_shows_mode_and_exact_selection: bool,
    /// The summary bar shows its target / environment shorthand.
    pub bar_shows_target_and_environment: bool,
    /// The summary bar shows its running / backlog counts.
    pub bar_shows_running_and_backlog_counts: bool,
    /// The summary bar shows its retry state (attempt lineage).
    pub bar_shows_retry_state: bool,
    /// The summary bar shows its current watch state.
    pub bar_shows_current_watch_state: bool,
    /// Discovery, execution, watch backlog, and imported-status refresh never share one
    /// generic loading treatment.
    pub distinct_activity_never_one_spinner: bool,
    /// The watch banner uses the controlled live/reduced/polling/unavailable vocabulary.
    pub banner_uses_controlled_watch_vocabulary: bool,
    /// The watch banner explains why fidelity degraded.
    pub banner_explains_degradation_reason: bool,
    /// The watch banner preserves the last successful cycle time.
    pub banner_preserves_last_successful_cycle: bool,
    /// The watch banner exposes recover and pause actions.
    pub banner_exposes_recover_and_pause: bool,
    /// A watch degradation stays visible in tree, status, support/export, and triage
    /// consumers with identical vocabulary.
    pub watch_degradation_visible_everywhere: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The components keep the same truth across every status consumer surface.
    pub components_stable_across_consumer_surfaces: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// The support / export packet reconstructs session and watch truth.
    pub support_export_reconstructs_status_truth: bool,
    /// Later M5 status components cannot invent parallel session/watch vocabulary.
    pub later_components_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SessionWatchConsumerProjection {
    /// Tree and status surfaces consume the shared session/watch vocabulary.
    pub tree_and_status_surfaces_consume_status_vocabulary: bool,
    /// The session-posture resolver reads a single canonical source.
    pub session_posture_reads_single_source: bool,
    /// The watch-posture resolver reads a single canonical source.
    pub watch_posture_reads_single_source: bool,
    /// The triage and support/export consumers read the same watch vocabulary.
    pub triage_and_support_read_same_watch_vocabulary: bool,
    /// Headless and desktop status read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SessionWatchProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the two status components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SessionWatchReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting test-evidence audit.
    pub test_evidence_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SessionWatchStatusPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SessionWatchStatusPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Status consumer rows.
    pub rows: Vec<M5SessionWatchConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SessionWatchVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SessionWatchGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SessionWatchConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SessionWatchProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SessionWatchReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 session-summary / watch-banner primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SessionWatchStatusPacket {
    /// Record kind; must equal [`M5_SESSION_WATCH_STATUS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SESSION_WATCH_STATUS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Status consumer rows.
    pub rows: Vec<M5SessionWatchConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SessionWatchVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SessionWatchGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SessionWatchConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SessionWatchProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SessionWatchReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SessionWatchStatusPacket {
    /// Builds an M5 session/watch-status-primitive packet from stable-lane input.
    pub fn new(input: M5SessionWatchStatusPacketInput) -> Self {
        Self {
            record_kind: M5_SESSION_WATCH_STATUS_RECORD_KIND.to_owned(),
            schema_version: M5_SESSION_WATCH_STATUS_SCHEMA_VERSION,
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

    /// Validates the M5 session/watch-status-primitive invariants.
    pub fn validate(&self) -> Vec<M5SessionWatchViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SESSION_WATCH_STATUS_RECORD_KIND {
            violations.push(M5SessionWatchViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SESSION_WATCH_STATUS_SCHEMA_VERSION {
            violations.push(M5SessionWatchViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SessionWatchViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_activity_phase_coverage(self, &mut violations);
        validate_watch_fidelity_coverage(self, &mut violations);
        validate_degradation_coverage(self, &mut violations);
        validate_retry_backlog_coverage(self, &mut violations);
        validate_last_cycle_preservation(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 session watch status packet serializes"),
        ) {
            violations.push(M5SessionWatchViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 session watch status packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per status consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,session_anatomy,session_postures,activity_phases,watch_postures,watch_fidelity_states,session_actions,watch_actions,session_examples,watch_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.session_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.session_postures, |v| v.as_str()),
                join_tokens(&row.activity_phases, |v| v.as_str()),
                join_tokens(&row.watch_postures, |v| v.as_str()),
                join_tokens(&row.watch_fidelity_states, |v| v.as_str()),
                join_tokens(&row.session_actions, |v| v.as_str()),
                join_tokens(&row.watch_actions, |v| v.as_str()),
                row.session_examples.len(),
                row.watch_examples.len(),
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
        out.push_str("# M5 Session-Summary-Bar / Watch-Mode-Banner Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Status consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Session postures: {}\n",
            self.vocabulary_set.session_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Watch postures: {}\n",
            self.vocabulary_set.watch_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Watch fidelity states: {}\n",
            self.vocabulary_set.watch_fidelity_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Status consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked sessions: {} / watches: {}\n",
                row.session_examples.len(),
                row.watch_examples.len()
            ));
            for case in &row.session_examples {
                out.push_str(&format!(
                    "    - session `{}` (`{}`) → `{}` (in-progress `{}`, watch-degraded `{}`)\n",
                    case.resolved.session_identity_ref,
                    case.resolved.activity_phase.as_str(),
                    case.resolved.session_posture.as_str(),
                    case.resolved.is_in_progress,
                    case.resolved.watch_is_degraded,
                ));
            }
            for case in &row.watch_examples {
                out.push_str(&format!(
                    "    - watch `{}` (`{}`) → `{}` (degraded `{}`, explains `{}`)\n",
                    case.resolved.watch_identity_ref,
                    case.resolved.watch_fidelity.as_str(),
                    case.resolved.watch_posture.as_str(),
                    case.resolved.is_degraded,
                    case.resolved.explains_degradation,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 session/watch-status-primitive export.
#[derive(Debug)]
pub enum M5SessionWatchArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SessionWatchViolation>),
}

impl fmt::Display for M5SessionWatchArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 session watch status export parse failed: {error}"
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
                    "m5 session watch status export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SessionWatchArtifactError {}

/// Validation failures emitted by [`M5SessionWatchStatusPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SessionWatchViolation {
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
    /// A required status consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A status consumer row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory session anatomy parts.
    MandatorySessionAnatomyMissing,
    /// A row omits one of the mandatory watch anatomy parts.
    MandatoryWatchAnatomyMissing,
    /// A row omits one of the mandatory session export fields.
    MandatorySessionExportMissing,
    /// A row omits one of the mandatory watch export fields.
    MandatoryWatchExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked session or watch resolutions.
    ExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every session activity phase (so a distinct
    /// activity treatment goes unproven).
    ActivityPhaseCoverageUnproven,
    /// The worked resolutions do not exercise every watch fidelity state.
    WatchFidelityCoverageUnproven,
    /// The worked resolutions do not prove both a degraded watch (with reason) and a live
    /// watch.
    DegradationCoverageUnproven,
    /// The worked resolutions do not prove both a session with retries/backlog and one
    /// without.
    RetryBacklogCoverageUnproven,
    /// A worked watch resolution does not preserve its last successful cycle.
    LastCyclePreservationUnproven,
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

impl M5SessionWatchViolation {
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
            Self::MandatorySessionAnatomyMissing => "mandatory_session_anatomy_missing",
            Self::MandatoryWatchAnatomyMissing => "mandatory_watch_anatomy_missing",
            Self::MandatorySessionExportMissing => "mandatory_session_export_missing",
            Self::MandatoryWatchExportMissing => "mandatory_watch_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleMissing => "example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ActivityPhaseCoverageUnproven => "activity_phase_coverage_unproven",
            Self::WatchFidelityCoverageUnproven => "watch_fidelity_coverage_unproven",
            Self::DegradationCoverageUnproven => "degradation_coverage_unproven",
            Self::RetryBacklogCoverageUnproven => "retry_backlog_coverage_unproven",
            Self::LastCyclePreservationUnproven => "last_cycle_preservation_unproven",
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

/// Reads and validates the checked-in stable M5 session/watch-status-primitive export.
pub fn current_stable_m5_session_watch_status_export(
) -> Result<M5SessionWatchStatusPacket, M5SessionWatchArtifactError> {
    let packet: M5SessionWatchStatusPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-session-summary-watch-banner-primitive-proof/support_export.json"
    )))
    .map_err(M5SessionWatchArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SessionWatchArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SESSION_WATCH_STATUS_SESSION_SCHEMA_REF,
        M5_SESSION_WATCH_STATUS_WATCH_SCHEMA_REF,
        M5_SESSION_WATCH_STATUS_DOC_REF,
        M5_SESSION_WATCH_STATUS_COMPONENT_MATRIX_REF,
        M5_SESSION_WATCH_STATUS_TEST_SESSION_REF,
        M5_SESSION_WATCH_STATUS_WATCH_STATE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SessionWatchViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5SessionWatchViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    let present: BTreeSet<M5SessionWatchConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5SessionWatchConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5SessionWatchViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.session_anatomy_parts.is_empty()
            || row.watch_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.session_modes.is_empty()
            || row.activity_phases.is_empty()
            || row.selection_scopes.is_empty()
            || row.session_outcomes.is_empty()
            || row.session_postures.is_empty()
            || row.attempt_lineage_kinds.is_empty()
            || row.watch_fidelity_states.is_empty()
            || row.watch_degrade_reasons.is_empty()
            || row.watch_postures.is_empty()
            || row.session_actions.is_empty()
            || row.watch_actions.is_empty()
            || row.session_export_fields.is_empty()
            || row.watch_export_fields.is_empty()
        {
            violations.push(M5SessionWatchViolation::RowIncomplete);
        }
        if !row.declares_mandatory_session_anatomy() {
            violations.push(M5SessionWatchViolation::MandatorySessionAnatomyMissing);
        }
        if !row.declares_mandatory_watch_anatomy() {
            violations.push(M5SessionWatchViolation::MandatoryWatchAnatomyMissing);
        }
        if !row.declares_mandatory_session_export() {
            violations.push(M5SessionWatchViolation::MandatorySessionExportMissing);
        }
        if !row.declares_mandatory_watch_export() {
            violations.push(M5SessionWatchViolation::MandatoryWatchExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TestAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5SessionWatchViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5SessionWatchViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5SessionWatchViolation::DowngradeTriggersMissing);
        }
        if row.session_examples.is_empty() || row.watch_examples.is_empty() {
            violations.push(M5SessionWatchViolation::ExampleMissing);
        }
        if row
            .session_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .watch_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5SessionWatchViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5SessionWatchViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5SessionWatchViolation::RowInvariantViolated);
        }
    }
}

/// Every session activity phase must be exercised by some worked resolution — the
/// acceptance-criterion proof that discovery, execution, watch backlog, and imported-status
/// refresh each get a distinct treatment rather than one generic loading state.
fn validate_activity_phase_coverage(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    let exercised: BTreeSet<M5SessionSummaryPosture> = packet
        .rows
        .iter()
        .flat_map(|row| row.session_examples.iter())
        .map(|case| case.resolved.session_posture)
        .collect();
    let covered = M5SessionSummaryPosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture));
    if !covered {
        violations.push(M5SessionWatchViolation::ActivityPhaseCoverageUnproven);
    }
}

/// Every watch fidelity state must be exercised by some worked resolution — the proof that
/// the controlled live/reduced/polling/unavailable (and paused/reconnecting) vocabulary is
/// distinguished.
fn validate_watch_fidelity_coverage(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    let exercised: BTreeSet<M5WatchFidelityState> = packet
        .rows
        .iter()
        .flat_map(|row| row.watch_examples.iter())
        .map(|case| case.resolved.watch_fidelity)
        .collect();
    let covered = M5WatchFidelityState::ALL
        .iter()
        .all(|fidelity| exercised.contains(fidelity));
    if !covered {
        violations.push(M5SessionWatchViolation::WatchFidelityCoverageUnproven);
    }
}

/// At least one worked resolution must prove a degraded watch that explains its reason and at
/// least one must prove a live watch — the acceptance-criterion example that a watch
/// degradation is always explained and never collapses into a green banner.
fn validate_degradation_coverage(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    let has_degraded = packet.rows.iter().any(|row| {
        row.watch_examples.iter().any(|case| {
            case.resolved.is_degraded
                && case.resolved.explains_degradation
                && case.resolved.degrade_reason.is_some()
                && case
                    .resolved
                    .available_actions
                    .contains(&M5WatchBannerAction::RecoverWatch)
        })
    });
    let has_live = packet.rows.iter().any(|row| {
        row.watch_examples
            .iter()
            .any(|case| !case.resolved.is_degraded && case.resolved.watch_posture.is_live())
    });
    if !(has_degraded && has_live) {
        violations.push(M5SessionWatchViolation::DegradationCoverageUnproven);
    }
}

/// At least one worked session resolution must prove a session carrying retries or a backlog
/// and at least one must prove a session with neither — the implementation requirement that
/// running/backlog counts and retry state are never left implicit.
fn validate_retry_backlog_coverage(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    let has_active = packet.rows.iter().any(|row| {
        row.session_examples
            .iter()
            .any(|case| case.resolved.has_backlog || case.resolved.has_retries)
    });
    let has_quiet = packet.rows.iter().any(|row| {
        row.session_examples
            .iter()
            .any(|case| !case.resolved.has_backlog && !case.resolved.has_retries)
    });
    if !(has_active && has_quiet) {
        violations.push(M5SessionWatchViolation::RetryBacklogCoverageUnproven);
    }
}

/// Every worked watch resolution must preserve a last successful cycle — the implementation
/// requirement that a banner never drops the last successful cycle time.
fn validate_last_cycle_preservation(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.watch_examples.iter())
        .all(|case| case.resolved.preserves_last_successful_cycle);
    if !preserved {
        violations.push(M5SessionWatchViolation::LastCyclePreservationUnproven);
    }
}

/// Every worked resolution must preserve its exact identity and label — the invariant that
/// neither component rewrites the user's session or watch identity.
fn validate_identity_preservation(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    let sessions_preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.session_examples.iter())
        .all(|case| case.preserves_identity());
    let watches_preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.watch_examples.iter())
        .all(|case| case.preserves_identity());
    if !(sessions_preserved && watches_preserved) {
        violations.push(M5SessionWatchViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.bar_shows_mode_and_exact_selection,
        review.bar_shows_target_and_environment,
        review.bar_shows_running_and_backlog_counts,
        review.bar_shows_retry_state,
        review.bar_shows_current_watch_state,
        review.distinct_activity_never_one_spinner,
        review.banner_uses_controlled_watch_vocabulary,
        review.banner_explains_degradation_reason,
        review.banner_preserves_last_successful_cycle,
        review.banner_exposes_recover_and_pause,
        review.watch_degradation_visible_everywhere,
        review.components_stable_across_deployment_lines,
        review.components_stable_across_consumer_surfaces,
        review.every_component_declares_accessibility_route,
        review.support_export_reconstructs_status_truth,
        review.later_components_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5SessionWatchViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.tree_and_status_surfaces_consume_status_vocabulary,
        projection.session_posture_reads_single_source,
        projection.watch_posture_reads_single_source,
        projection.triage_and_support_read_same_watch_vocabulary,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5SessionWatchViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SessionWatchViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SessionWatchStatusPacket,
    violations: &mut Vec<M5SessionWatchViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.test_evidence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SessionWatchViolation::ReleasePostureIncomplete);
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
