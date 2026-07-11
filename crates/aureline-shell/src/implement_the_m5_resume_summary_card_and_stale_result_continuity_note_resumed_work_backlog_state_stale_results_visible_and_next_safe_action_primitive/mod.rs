//! Implemented M5 resume-summary card and stale-result-continuity note primitives.
//!
//! The frozen [efficiency component matrix][matrix] names the reusable adaptive-efficiency UI
//! components and locks their controlled vocabulary. The [power-state / throttled lane][power], the
//! [background-work lane][background], and the [override-sheet / policy-note lane][override] turned
//! the first six components into resolvers. This module is the fourth and final implement lane over
//! that matrix: it turns the **resume-summary card** (a durable summary of what resumed, what
//! backlog remains, whether stale results are still visible, and what the safest next action is for
//! the current task) and the **stale-result continuity note** (an explanation that a still-visible
//! result is cached, partial, or based on a prior constrained state after recovery) into resolvers
//! that produce export-safe, honest projections instead of recovery a user has to infer.
//!
//! Two acceptance criteria drive the resolvers:
//!
//! * **AC1 — returning to nominal conditions never silently removes evidence that a result is still
//!   stale, partial, or based on a prior constrained state.** [`resolve_resume_summary_card`] and
//!   [`resolve_stale_result_continuity_note`] refuse to read as clean when a live stale result is
//!   dropped from view once recovery completes; they degrade to
//!   [`M5ResumeSummaryCardDegradeReason::StaleResultEvidenceDropped`] and
//!   [`M5StaleResultNoteDegradeReason::StaleEvidenceSilentlyRemoved`] instead. A clean surface keeps
//!   a retained or refreshing stale result visible and states that it is based on a prior
//!   constrained state.
//! * **AC2 — users get one durable summary of resumed work instead of having to infer recovery from
//!   disappearing banners or background queue motion.** [`resolve_resume_summary_card`] degrades to
//!   [`M5ResumeSummaryCardDegradeReason::RecoverySummaryNotDurable`] the moment the summary is not
//!   durable, and to [`M5ResumeSummaryCardDegradeReason::ResumeBacklogHidden`] the moment the
//!   resumed-work backlog is hidden.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5EfficiencyWorkDisposition`] vocabulary, the [`EfficiencyRecoveryState`] recovery vocabulary,
//! the [`M5EfficiencyStaleResultState`] stale-result vocabulary, and the [`WorkloadFamily`]
//! affected-work vocabulary — so this lane can never fork its own resume or stale-result wording.
//!
//! [matrix]: crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix
//! [power]: crate::implement_the_m5_power_state_indicator_and_throttled_subsystem_row_source_active_state_affected_subsystem_and_inspect_path_primitive
//! [background]: crate::implement_the_m5_background_work_row_and_background_work_banner_affected_work_class_state_what_still_works_resume_condition_and_override_primitive
//! [override]: crate::implement_the_m5_per_workspace_override_sheet_and_override_policy_note_row_current_mode_ceilings_expected_effect_reset_path_and_blocked_by_policy_primitive

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_resume_controls, seeded_m5_resume_controls_activity_center_beta_narrowed,
    seeded_m5_resume_controls_background_work_preview_narrowed, M5_RESUME_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::efficiency::governance::{EfficiencyRecoveryState, M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF};
use crate::efficiency::WorkloadFamily;
use crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix::{
    M5EfficiencyAccessibilityRoute, M5EfficiencyConsumerSurface, M5EfficiencyDeploymentLine,
    M5EfficiencyDowngradeTrigger, M5EfficiencyQualificationClass, M5EfficiencyRequiredLabel,
    M5EfficiencyStaleResultState, M5EfficiencyWorkDisposition, M5_EFFICIENCY_COMPONENT_DOC_REF,
    M5_EFFICIENCY_COMPONENT_SCHEMA_REF, M5_RESUME_SUMMARY_CARD_SCHEMA_REF,
    M5_STALE_RESULT_CONTINUITY_NOTE_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ResumeControlsPacket`].
pub const M5_RESUME_CONTROLS_RECORD_KIND: &str =
    "implement_m5_resume_summary_card_and_stale_result_continuity_note_controls";

/// Schema version for M5 resume-summary / stale-result-note controls records.
pub const M5_RESUME_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls boundary schema.
pub const M5_RESUME_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-resume-summary-stale-note-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_RESUME_CONTROLS_DOC_REF: &str =
    "docs/help/m5_resume_summary_and_stale_note_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RESUME_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-resume-summary-stale-note-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_RESUME_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-resume-summary-stale-note-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_RESUME_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-resume-summary-stale-note-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_RESUME_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-resume-summary-stale-note-controls";

/// Consumer surface a resume-summary card / stale-result note projects onto. Reuses the frozen
/// matrix consumer-surface taxonomy so no lane invents a parallel surface set.
pub type M5ResumeConsumerSurface = M5EfficiencyConsumerSurface;

/// One mandatory rendered part a resume-summary card or stale-result note must be able to show, so
/// no recovery or stale-result truth is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResumeAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The controlled recovery state after pressure cleared.
    RecoveryState,
    /// The workloads that resumed from their deferred backlog.
    ResumedWork,
    /// The backlog that still remains after resume.
    RemainingBacklog,
    /// Whether stale results are still visible after recovery.
    StaleResultVisibility,
    /// That a still-visible result is based on a prior constrained state.
    PriorConstrainedState,
    /// The safest next action for the current task.
    NextSafeAction,
}

impl M5ResumeAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::RecoveryState,
        Self::ResumedWork,
        Self::RemainingBacklog,
        Self::StaleResultVisibility,
        Self::PriorConstrainedState,
        Self::NextSafeAction,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::RecoveryState => "recovery_state",
            Self::ResumedWork => "resumed_work",
            Self::RemainingBacklog => "remaining_backlog",
            Self::StaleResultVisibility => "stale_result_visibility",
            Self::PriorConstrainedState => "prior_constrained_state",
            Self::NextSafeAction => "next_safe_action",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResumeNextAction {
    /// Open the durable resume summary for the current task.
    OpenResumeSummary,
    /// Review the still-visible stale results.
    ReviewStaleResults,
    /// Refresh the stale result now.
    RefreshNow,
    /// Open the activity center to review resuming work.
    OpenActivityCenter,
    /// Review diagnostics for the unresolved signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5ResumeNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenResumeSummary,
        Self::ReviewStaleResults,
        Self::RefreshNow,
        Self::OpenActivityCenter,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenResumeSummary => "open_resume_summary",
            Self::ReviewStaleResults => "review_stale_results",
            Self::RefreshNow => "refresh_now",
            Self::OpenActivityCenter => "open_activity_center",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a resume controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResumeExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The work dispositions carried.
    WorkDispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The recovery state after pressure cleared.
    RecoveryState,
    /// The workloads that resumed.
    ResumedWork,
    /// The backlog that still remains.
    RemainingBacklog,
    /// The stale-result continuity state.
    StaleResultState,
    /// Whether stale results stay visible after recovery.
    StaleResultVisibility,
    /// The safest next action.
    NextSafeAction,
}

impl M5ResumeExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::WorkDispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::RecoveryState,
        Self::ResumedWork,
        Self::RemainingBacklog,
        Self::StaleResultState,
        Self::StaleResultVisibility,
        Self::NextSafeAction,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::WorkDispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::WorkDispositions => "work_dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::RecoveryState => "recovery_state",
            Self::ResumedWork => "resumed_work",
            Self::RemainingBacklog => "remaining_backlog",
            Self::StaleResultState => "stale_result_state",
            Self::StaleResultVisibility => "stale_result_visibility",
            Self::NextSafeAction => "next_safe_action",
        }
    }
}

/// Reason a resume-summary card degraded below a clean, fully-legible state. The degrade-first
/// ladder returns one of these instead of ever letting a card that drops stale evidence or forces a
/// user to infer recovery read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResumeSummaryCardDegradeReason {
    /// No resumed workload was named, so what resumed cannot be told.
    ResumedWorkUnnamed,
    /// A live stale result was dropped from view once recovery completed (AC1 violation): the user
    /// loses evidence that a result is still stale.
    StaleResultEvidenceDropped,
    /// The recovery summary is not durable, so the user would have to infer recovery from a
    /// disappearing banner (AC2 violation).
    RecoverySummaryNotDurable,
    /// The resumed-work backlog is hidden, so the user would have to infer it from background queue
    /// motion (AC2 violation).
    ResumeBacklogHidden,
    /// The safest next action for the current task is unstated.
    NextSafeActionUnstated,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ResumeSummaryCardDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ResumedWorkUnnamed,
        Self::StaleResultEvidenceDropped,
        Self::RecoverySummaryNotDurable,
        Self::ResumeBacklogHidden,
        Self::NextSafeActionUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResumedWorkUnnamed => "resumed_work_unnamed",
            Self::StaleResultEvidenceDropped => "stale_result_evidence_dropped",
            Self::RecoverySummaryNotDurable => "recovery_summary_not_durable",
            Self::ResumeBacklogHidden => "resume_backlog_hidden",
            Self::NextSafeActionUnstated => "next_safe_action_unstated",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ResumeNextAction {
        match self {
            Self::StaleResultEvidenceDropped => M5ResumeNextAction::ReviewStaleResults,
            Self::ResumeBacklogHidden => M5ResumeNextAction::OpenActivityCenter,
            Self::ResumedWorkUnnamed
            | Self::RecoverySummaryNotDurable
            | Self::NextSafeActionUnstated => M5ResumeNextAction::OpenResumeSummary,
            Self::ProofStale => M5ResumeNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EfficiencyDowngradeTrigger {
        match self {
            Self::ResumedWorkUnnamed | Self::NextSafeActionUnstated => {
                M5EfficiencyDowngradeTrigger::WhatStillWorksUnstated
            }
            Self::StaleResultEvidenceDropped => {
                M5EfficiencyDowngradeTrigger::StaleResultContinuityCleared
            }
            Self::RecoverySummaryNotDurable => M5EfficiencyDowngradeTrigger::PausedWorkToastOnly,
            Self::ResumeBacklogHidden => M5EfficiencyDowngradeTrigger::ResumeBacklogHidden,
            Self::ProofStale => M5EfficiencyDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a stale-result continuity note degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StaleResultNoteDegradeReason {
    /// The result's continuity cannot be determined, so its staleness cannot be told.
    ContinuityUnknown,
    /// A live stale result was silently removed from view (AC1 violation): the user loses evidence
    /// that a result is still stale, partial, or based on a prior constrained state.
    StaleEvidenceSilentlyRemoved,
    /// The note does not state that a still-visible result is based on a prior constrained state.
    PriorConstrainedStateUnstated,
    /// A stale result is refreshing but the note does not state the refresh path.
    RefreshPathUnstated,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5StaleResultNoteDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ContinuityUnknown,
        Self::StaleEvidenceSilentlyRemoved,
        Self::PriorConstrainedStateUnstated,
        Self::RefreshPathUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinuityUnknown => "continuity_unknown",
            Self::StaleEvidenceSilentlyRemoved => "stale_evidence_silently_removed",
            Self::PriorConstrainedStateUnstated => "prior_constrained_state_unstated",
            Self::RefreshPathUnstated => "refresh_path_unstated",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ResumeNextAction {
        match self {
            Self::ContinuityUnknown | Self::ProofStale => M5ResumeNextAction::ReviewDiagnostics,
            Self::StaleEvidenceSilentlyRemoved | Self::PriorConstrainedStateUnstated => {
                M5ResumeNextAction::ReviewStaleResults
            }
            Self::RefreshPathUnstated => M5ResumeNextAction::RefreshNow,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EfficiencyDowngradeTrigger {
        match self {
            Self::ContinuityUnknown
            | Self::StaleEvidenceSilentlyRemoved
            | Self::PriorConstrainedStateUnstated => {
                M5EfficiencyDowngradeTrigger::StaleResultContinuityCleared
            }
            Self::RefreshPathUnstated => M5EfficiencyDowngradeTrigger::WhatStillWorksUnstated,
            Self::ProofStale => M5EfficiencyDowngradeTrigger::ProofStale,
        }
    }
}

/// True when a stale-result state is still live — a result deliberately kept visible pending or
/// during refresh — so silently dropping it from view loses evidence of its staleness.
const fn stale_result_is_live(state: M5EfficiencyStaleResultState) -> bool {
    matches!(
        state,
        M5EfficiencyStaleResultState::StaleResultRetained
            | M5EfficiencyStaleResultState::StaleResultRefreshing
    )
}

/// Input to [`resolve_resume_summary_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ResumeSummaryCardResolutionInput {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// The recovery state after pressure cleared.
    pub recovery_state: EfficiencyRecoveryState,
    /// The workloads that resumed from their deferred backlog.
    pub resumed_workloads: Vec<WorkloadFamily>,
    /// The backlog that still remains after resume.
    pub backlog_workloads: Vec<WorkloadFamily>,
    /// True when the resumed-work backlog size is known (else it is hidden).
    pub backlog_known: bool,
    /// The stale-result continuity state for the current task.
    pub stale_result_state: M5EfficiencyStaleResultState,
    /// True when stale results are still visible after recovery.
    pub stale_results_visible: bool,
    /// True when the recovery summary is durable rather than a disappearing banner.
    pub durable_summary_present: bool,
    /// True when the safest next action for the current task is stated.
    pub next_safe_action_stated: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe resume-summary-card projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedResumeSummaryCard {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// Recovery-state token after pressure cleared.
    pub recovery_state: String,
    /// Resumed-workload tokens.
    pub resumed_workloads: Vec<String>,
    /// Remaining-backlog workload tokens.
    pub backlog_workloads: Vec<String>,
    /// Single controlled work disposition carried by the card.
    pub work_disposition: M5EfficiencyWorkDisposition,
    /// Stale-result continuity-state token for the current task.
    pub stale_result_state: String,
    /// Whether stale results are still visible after recovery.
    pub stale_results_visible: bool,
    /// Whether the resumed-work backlog size is known.
    pub backlog_known: bool,
    /// Whether the recovery summary is durable rather than a disappearing banner.
    pub durable_summary_present: bool,
    /// Whether the safest next action for the current task is stated.
    pub next_safe_action_stated: bool,
    /// AC1: whether the card preserved live stale-result evidence rather than silently dropping it.
    pub stale_evidence_preserved: bool,
    /// Degrade reason, if the card could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5ResumeSummaryCardDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ResumeNextAction,
    /// Guardrail (MUST be `false` on a clean card): a live stale result was silently dropped from
    /// view on resume.
    pub silently_dropped_stale_evidence: bool,
}

impl M5ResolvedResumeSummaryCard {
    /// Whether this card reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_stale_result_continuity_note`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5StaleResultNoteResolutionInput {
    /// Stable identity of the note instance.
    pub note_id: String,
    /// The stale-result continuity state.
    pub stale_result_state: M5EfficiencyStaleResultState,
    /// True when conditions have returned to nominal (recovery completed).
    pub returned_to_nominal: bool,
    /// True when the note keeps the stale result visible.
    pub stale_results_visible: bool,
    /// True when the note states the result is based on a prior constrained state.
    pub based_on_constrained_state_stated: bool,
    /// True when a refreshing stale result states its refresh path.
    pub refresh_path_stated: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe stale-result continuity-note projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedStaleResultNote {
    /// Stable identity of the note instance.
    pub note_id: String,
    /// Stale-result continuity-state token.
    pub stale_result_state: String,
    /// Single controlled work disposition carried by the note.
    pub work_disposition: M5EfficiencyWorkDisposition,
    /// Whether conditions have returned to nominal.
    pub returned_to_nominal: bool,
    /// Whether the note keeps the stale result visible.
    pub stale_results_visible: bool,
    /// Whether the note states the result is based on a prior constrained state.
    pub based_on_constrained_state_stated: bool,
    /// Whether a refreshing stale result states its refresh path.
    pub refresh_path_stated: bool,
    /// AC1: whether the note preserved live stale-result evidence rather than silently removing it.
    pub stale_evidence_preserved: bool,
    /// Degrade reason, if the note could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5StaleResultNoteDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ResumeNextAction,
    /// Guardrail (MUST be `false` on a clean note): a live stale result was silently removed from
    /// view on resume.
    pub silently_removed_stale_evidence: bool,
}

impl M5ResolvedStaleResultNote {
    /// Whether this note reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ResumeResolutionError {
    /// The resume-summary card id was empty.
    EmptyCardId,
    /// The stale-result continuity note id was empty.
    EmptyNoteId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ResumeResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCardId => "empty_card_id",
            Self::EmptyNoteId => "empty_note_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ResumeResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "m5 resume resolution error: {}", self.as_str())
    }
}

impl Error for M5ResumeResolutionError {}

/// Resolves a resume-summary card, proving AC2 (one durable summary of resumed work rather than
/// recovery inferred from disappearing banners or background queue motion) and the card's half of
/// AC1 (a live stale result is never silently dropped from the summary on resume).
pub fn resolve_resume_summary_card(
    input: M5ResumeSummaryCardResolutionInput,
) -> Result<M5ResolvedResumeSummaryCard, M5ResumeResolutionError> {
    if input.card_id.trim().is_empty() {
        return Err(M5ResumeResolutionError::EmptyCardId);
    }
    if string_is_forbidden(&input.card_id) {
        return Err(M5ResumeResolutionError::ForbiddenMaterial);
    }

    let stale_live = stale_result_is_live(input.stale_result_state);
    let silently_dropped_stale_evidence = stale_live && !input.stale_results_visible;

    let degrade_reason = if input.resumed_workloads.is_empty() {
        Some(M5ResumeSummaryCardDegradeReason::ResumedWorkUnnamed)
    } else if silently_dropped_stale_evidence {
        Some(M5ResumeSummaryCardDegradeReason::StaleResultEvidenceDropped)
    } else if !input.durable_summary_present {
        Some(M5ResumeSummaryCardDegradeReason::RecoverySummaryNotDurable)
    } else if !input.backlog_known {
        Some(M5ResumeSummaryCardDegradeReason::ResumeBacklogHidden)
    } else if !input.next_safe_action_stated {
        Some(M5ResumeSummaryCardDegradeReason::NextSafeActionUnstated)
    } else if !input.proof_fresh {
        Some(M5ResumeSummaryCardDegradeReason::ProofStale)
    } else {
        None
    };

    let work_disposition = if input.resumed_workloads.is_empty() {
        M5EfficiencyWorkDisposition::NotEvaluated
    } else if stale_live && input.stale_results_visible {
        M5EfficiencyWorkDisposition::StaleResultShown
    } else if input.recovery_state == EfficiencyRecoveryState::Recovered
        && input.backlog_workloads.is_empty()
    {
        M5EfficiencyWorkDisposition::RunningFull
    } else {
        M5EfficiencyWorkDisposition::Resuming
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ResumeNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedResumeSummaryCard {
        card_id: input.card_id,
        recovery_state: input.recovery_state.as_str().to_owned(),
        resumed_workloads: input
            .resumed_workloads
            .iter()
            .map(|w| w.as_str().to_owned())
            .collect(),
        backlog_workloads: input
            .backlog_workloads
            .iter()
            .map(|w| w.as_str().to_owned())
            .collect(),
        work_disposition,
        stale_result_state: input.stale_result_state.as_str().to_owned(),
        stale_results_visible: input.stale_results_visible,
        backlog_known: input.backlog_known,
        durable_summary_present: input.durable_summary_present,
        next_safe_action_stated: input.next_safe_action_stated,
        stale_evidence_preserved: !silently_dropped_stale_evidence,
        degrade_reason,
        next_action,
        silently_dropped_stale_evidence,
    })
}

/// Resolves a stale-result continuity note, proving AC1: returning to nominal never silently
/// removes evidence that a still-visible result is stale, partial, or based on a prior constrained
/// state.
pub fn resolve_stale_result_continuity_note(
    input: M5StaleResultNoteResolutionInput,
) -> Result<M5ResolvedStaleResultNote, M5ResumeResolutionError> {
    if input.note_id.trim().is_empty() {
        return Err(M5ResumeResolutionError::EmptyNoteId);
    }
    if string_is_forbidden(&input.note_id) {
        return Err(M5ResumeResolutionError::ForbiddenMaterial);
    }

    let stale_live = stale_result_is_live(input.stale_result_state);
    let silently_removed_stale_evidence = stale_live && !input.stale_results_visible;

    let degrade_reason =
        if input.stale_result_state == M5EfficiencyStaleResultState::ContinuityUnknown {
            Some(M5StaleResultNoteDegradeReason::ContinuityUnknown)
        } else if silently_removed_stale_evidence {
            Some(M5StaleResultNoteDegradeReason::StaleEvidenceSilentlyRemoved)
        } else if stale_live && !input.based_on_constrained_state_stated {
            Some(M5StaleResultNoteDegradeReason::PriorConstrainedStateUnstated)
        } else if input.stale_result_state == M5EfficiencyStaleResultState::StaleResultRefreshing
            && !input.refresh_path_stated
        {
            Some(M5StaleResultNoteDegradeReason::RefreshPathUnstated)
        } else if !input.proof_fresh {
            Some(M5StaleResultNoteDegradeReason::ProofStale)
        } else {
            None
        };

    let work_disposition = match input.stale_result_state {
        M5EfficiencyStaleResultState::ContinuityUnknown => {
            M5EfficiencyWorkDisposition::NotEvaluated
        }
        M5EfficiencyStaleResultState::FreshResult
        | M5EfficiencyStaleResultState::StaleResultSuperseded => {
            M5EfficiencyWorkDisposition::RunningFull
        }
        M5EfficiencyStaleResultState::StaleResultRetained
        | M5EfficiencyStaleResultState::StaleResultRefreshing => {
            M5EfficiencyWorkDisposition::StaleResultShown
        }
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ResumeNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedStaleResultNote {
        note_id: input.note_id,
        stale_result_state: input.stale_result_state.as_str().to_owned(),
        work_disposition,
        returned_to_nominal: input.returned_to_nominal,
        stale_results_visible: input.stale_results_visible,
        based_on_constrained_state_stated: input.based_on_constrained_state_stated,
        refresh_path_stated: input.refresh_path_stated,
        stale_evidence_preserved: !silently_removed_stale_evidence,
        degrade_reason,
        next_action,
        silently_removed_stale_evidence,
    })
}

/// One controls row: one consumer surface bound to the resolved resume-summary-card and
/// stale-result-note examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResumeControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ResumeConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5EfficiencyQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5EfficiencyDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5EfficiencyRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5EfficiencyAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5ResumeAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ResumeExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5EfficiencyDowngradeTrigger>,
    /// Resolved resume-summary-card examples.
    pub resume_summary_examples: Vec<M5ResolvedResumeSummaryCard>,
    /// Resolved stale-result continuity-note examples.
    pub stale_result_note_examples: Vec<M5ResolvedStaleResultNote>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never clear stale-result context merely because background work resumed.
    pub clears_stale_result_context_on_resume: bool,
    /// Hard invariant: never require inferring recovery from transient / disappearing banners.
    pub requires_inferring_recovery_from_transient_banners: bool,
    /// Hard invariant: never hide the resumed-work backlog.
    pub hides_resumed_work_backlog: bool,
    /// Hard invariant: never collapse distinct pressure sources into one generic warning.
    pub collapses_pressure_sources_into_generic_warning: bool,
}

impl M5ResumeControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ResumeAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5ResumeAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ResumeExportField> = self.export_fields.iter().copied().collect();
        M5ResumeExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.clears_stale_result_context_on_resume
            && !self.requires_inferring_recovery_from_transient_banners
            && !self.hides_resumed_work_backlog
            && !self.collapses_pressure_sources_into_generic_warning
    }

    /// True when every resolved example on this row is honest: no clean card silently drops stale
    /// evidence, hides the backlog, or reads as non-durable; and no clean note silently removes
    /// stale evidence.
    fn examples_are_honest(&self) -> bool {
        self.resume_summary_examples.iter().all(|ex| {
            !ex.is_clean()
                || (ex.stale_evidence_preserved && ex.durable_summary_present && ex.backlog_known)
        }) && self
            .stale_result_note_examples
            .iter()
            .all(|ex| !ex.is_clean() || ex.stale_evidence_preserved)
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResumeVocabularySet {
    /// Work-disposition tokens (bound from the frozen matrix).
    pub work_dispositions: Vec<String>,
    /// Recovery-state tokens (bound from the efficiency object model).
    pub recovery_states: Vec<String>,
    /// Stale-result continuity-state tokens (bound from the frozen matrix).
    pub stale_result_states: Vec<String>,
    /// Affected-workload tokens (bound from the efficiency object model).
    pub affected_workloads: Vec<String>,
    /// Resume-summary-card degrade-reason tokens.
    pub card_degrade_reasons: Vec<String>,
    /// Stale-result-note degrade-reason tokens.
    pub note_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ResumeVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            work_dispositions: tokens(&M5EfficiencyWorkDisposition::ALL, |v| v.as_str()),
            recovery_states: tokens(&EfficiencyRecoveryState::ALL, |v| v.as_str()),
            stale_result_states: tokens(&M5EfficiencyStaleResultState::ALL, |v| v.as_str()),
            affected_workloads: tokens(&AFFECTED_WORKLOADS, |v| v.as_str()),
            card_degrade_reasons: tokens(&M5ResumeSummaryCardDegradeReason::ALL, |v| v.as_str()),
            note_degrade_reasons: tokens(&M5StaleResultNoteDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ResumeAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ResumeNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ResumeExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5EfficiencyConsumerSurface::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The canonical workload families bound from the shared efficiency object model, in canonical
/// order. [`WorkloadFamily`] does not export its own `ALL`, so this lane pins the full set.
pub const AFFECTED_WORKLOADS: [WorkloadFamily; 9] = [
    WorkloadFamily::AiWarmup,
    WorkloadFamily::SpeculativePrefetch,
    WorkloadFamily::UploadTransfer,
    WorkloadFamily::NonEssentialAnimation,
    WorkloadFamily::IndexingRefresh,
    WorkloadFamily::ExtensionPolling,
    WorkloadFamily::PreviewRefresh,
    WorkloadFamily::GraphEnrichment,
    WorkloadFamily::RemoteSessionHelper,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResumeGovernanceReview {
    /// The resume-summary card always lists what resumed.
    pub card_lists_resumed_work: bool,
    /// The resume-summary card always states what backlog remains.
    pub card_states_remaining_backlog: bool,
    /// The resume-summary card always states whether stale results are still visible.
    pub card_states_stale_result_visibility: bool,
    /// The resume-summary card always states the safest next action.
    pub card_states_next_safe_action: bool,
    /// The recovery summary is always durable rather than a disappearing banner.
    pub recovery_summary_is_durable: bool,
    /// No surface ever silently clears stale-result context merely because work resumed.
    pub no_stale_result_context_cleared_on_resume: bool,
    /// The stale-result note always states that a still-visible result is based on a prior
    /// constrained state.
    pub note_states_prior_constrained_state: bool,
    /// The stale-result note always keeps a live stale result visible after recovery.
    pub note_keeps_stale_result_visible: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResumeConsumerProjection {
    /// The activity center consumes the shared resume-summary card.
    pub activity_center_consumes_shared_card: bool,
    /// Shell and background-work surfaces consume the shared stale-result note.
    pub shell_and_background_consume_shared_note: bool,
    /// Diagnostics surfaces consume the shared resume / stale-result vocabulary.
    pub diagnostics_consumes_resume_vocabulary: bool,
    /// Support / export reads a single canonical resume source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResumeProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResumeReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting efficiency audit for the lane.
    pub efficiency_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ResumeControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ResumeControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ResumeControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ResumeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ResumeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ResumeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ResumeProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ResumeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 resume-summary / stale-result-note controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResumeControlsPacket {
    /// Record kind; must equal [`M5_RESUME_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RESUME_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ResumeControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ResumeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ResumeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ResumeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ResumeProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ResumeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ResumeControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5ResumeControlsPacketInput) -> Self {
        Self {
            record_kind: M5_RESUME_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_RESUME_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            controls_label: input.controls_label,
            controls_rows: input.controls_rows,
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

    /// Validates the controls-packet invariants.
    pub fn validate(&self) -> Vec<M5ResumeControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RESUME_CONTROLS_RECORD_KIND {
            violations.push(M5ResumeControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RESUME_CONTROLS_SCHEMA_VERSION {
            violations.push(M5ResumeControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ResumeControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5ResumeControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 resume controls packet serializes"),
        ) {
            violations.push(M5ResumeControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 resume controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,card_examples,note_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .resume_summary_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.stale_result_note_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.resume_summary_examples.len(),
                row.stale_result_note_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Resume-Summary Card and Stale-Result Continuity Note Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Work dispositions: {}\n",
            self.vocabulary_set.work_dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Card examples: {} / note examples: {}\n",
                row.resume_summary_examples.len(),
                row.stale_result_note_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5ResumeControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ResumeControlsViolation>),
}

impl fmt::Display for M5ResumeControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "m5 resume controls export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 resume controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ResumeControlsArtifactError {}

/// Validation failures emitted by [`M5ResumeControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ResumeControlsViolation {
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
    /// The controls packet declares no rows.
    NoControlsRows,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row does not point at both component schemas.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (dropped stale evidence, hidden backlog, or
    /// non-durable summary).
    DishonestExample,
    /// A controls row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// AC1 is not proven: no card or note shows a live stale result dropped on resume, or a clean
    /// example still drops stale evidence, or no clean example keeps a stale result visible.
    Ac1NotProven,
    /// AC2 is not proven: no card shows a non-durable summary or a hidden backlog degrading, or a
    /// clean card still hides the backlog or reads as non-durable.
    Ac2NotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ResumeControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoControlsRows => "no_controls_rows",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::Ac1NotProven => "ac1_not_proven",
            Self::Ac2NotProven => "ac2_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_resume_controls_export(
) -> Result<M5ResumeControlsPacket, M5ResumeControlsArtifactError> {
    let packet: M5ResumeControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-resume-summary-stale-note-controls-proof/support_export.json"
    )))
    .map_err(M5ResumeControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ResumeControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ResumeControlsPacket,
    violations: &mut Vec<M5ResumeControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RESUME_CONTROLS_SCHEMA_REF,
        M5_RESUME_CONTROLS_DOC_REF,
        M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
        M5_EFFICIENCY_COMPONENT_DOC_REF,
        M5_RESUME_SUMMARY_CARD_SCHEMA_REF,
        M5_STALE_RESULT_CONTINUITY_NOTE_SCHEMA_REF,
        M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ResumeControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5ResumeControlsPacket,
    violations: &mut Vec<M5ResumeControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5ResumeControlsViolation::NoControlsRows);
        return;
    }
    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5ResumeControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ResumeControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ResumeControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_RESUME_SUMMARY_CARD_SCHEMA_REF)
            || !refs.contains(M5_STALE_RESULT_CONTINUITY_NOTE_SCHEMA_REF)
        {
            violations.push(M5ResumeControlsViolation::ComponentSchemaRefMissing);
        }
        if row.resume_summary_examples.is_empty() || row.stale_result_note_examples.is_empty() {
            violations.push(M5ResumeControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5ResumeControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5ResumeControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ResumeControlsPacket,
    violations: &mut Vec<M5ResumeControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.card_lists_resumed_work,
        review.card_states_remaining_backlog,
        review.card_states_stale_result_visibility,
        review.card_states_next_safe_action,
        review.recovery_summary_is_durable,
        review.no_stale_result_context_cleared_on_resume,
        review.note_states_prior_constrained_state,
        review.note_keeps_stale_result_visible,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5ResumeControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ResumeControlsPacket,
    violations: &mut Vec<M5ResumeControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.activity_center_consumes_shared_card,
        projection.shell_and_background_consume_shared_note,
        projection.diagnostics_consumes_resume_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ResumeControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ResumeControlsPacket,
    violations: &mut Vec<M5ResumeControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ResumeControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ResumeControlsPacket,
    violations: &mut Vec<M5ResumeControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.efficiency_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ResumeControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ResumeControlsPacket,
    violations: &mut Vec<M5ResumeControlsViolation>,
) {
    let card_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.resume_summary_examples.iter())
    };
    let note_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.stale_result_note_examples.iter())
    };

    // AC1: a card and a note each degrade to dropped / removed stale evidence when a live stale
    // result is hidden on resume, no clean example still drops stale evidence, and at least one
    // clean example keeps a live stale result visible (shown as stale-result-shown).
    let card_drop_degrades = card_examples().any(|ex| {
        ex.degrade_reason == Some(M5ResumeSummaryCardDegradeReason::StaleResultEvidenceDropped)
            && ex.silently_dropped_stale_evidence
    });
    let note_drop_degrades = note_examples().any(|ex| {
        ex.degrade_reason == Some(M5StaleResultNoteDegradeReason::StaleEvidenceSilentlyRemoved)
            && ex.silently_removed_stale_evidence
    });
    let no_clean_drop = card_examples()
        .all(|ex| !(ex.is_clean() && ex.silently_dropped_stale_evidence))
        && note_examples().all(|ex| !(ex.is_clean() && ex.silently_removed_stale_evidence));
    let clean_stale_shown_honest = card_examples().any(|ex| {
        ex.is_clean() && ex.work_disposition == M5EfficiencyWorkDisposition::StaleResultShown
    }) || note_examples().any(|ex| {
        ex.is_clean() && ex.work_disposition == M5EfficiencyWorkDisposition::StaleResultShown
    });
    if !(card_drop_degrades && note_drop_degrades && no_clean_drop && clean_stale_shown_honest) {
        violations.push(M5ResumeControlsViolation::Ac1NotProven);
    }

    // AC2: at least one card degrades because the summary is not durable and at least one because
    // the backlog is hidden, and no clean card hides the backlog or reads as non-durable.
    let not_durable_degrades = card_examples().any(|ex| {
        ex.degrade_reason == Some(M5ResumeSummaryCardDegradeReason::RecoverySummaryNotDurable)
    });
    let backlog_hidden_degrades = card_examples()
        .any(|ex| ex.degrade_reason == Some(M5ResumeSummaryCardDegradeReason::ResumeBacklogHidden));
    let no_clean_infers = card_examples()
        .all(|ex| !(ex.is_clean() && (!ex.durable_summary_present || !ex.backlog_known)));
    if !(not_durable_degrades && backlog_hidden_degrades && no_clean_infers) {
        violations.push(M5ResumeControlsViolation::Ac2NotProven);
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

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
