//! Implemented M5 power-state-indicator and throttled-subsystem-row primitives.
//!
//! The frozen [efficiency component matrix][matrix] names the reusable adaptive-efficiency UI
//! components and locks their controlled vocabulary. This module is the first implement lane over
//! that matrix: it turns the two top-of-funnel components — the **power-state indicator** and the
//! **throttled-subsystem row** — into resolvers that produce export-safe, honest projections
//! instead of prose or a single status-bar implementation.
//!
//! Two acceptance criteria drive the resolvers:
//!
//! * **AC1 — users can distinguish *why* Aureline adapted and *which* subsystems were affected**
//!   without opening logs or inferring from missing behavior. [`resolve_power_state_indicator`]
//!   refuses to read as a clean generic "low power" state when the source of change is unstated
//!   or when distinct causes (system battery saver, thermal pressure, user-selected low-power
//!   mode, policy cap, …) are collapsed into one warning; it degrades instead. Together with
//!   [`resolve_throttled_subsystem_row`], which enumerates exactly which background lanes slowed
//!   or paused and which protected tasks remain preserved, the pair keeps the source of change,
//!   active efficiency state, affected subsystem, and inspect path explicit.
//! * **AC2 — no claimed M5 shell surface silently widens or hides slowed background work once
//!   adaptive behavior became user-visible.** [`resolve_throttled_subsystem_row`] degrades to
//!   [`M5ThrottledDegradeReason::SlowedWorkSilentlyHidden`] the moment a surface tries to hide
//!   slowed work that a user has already seen, so the honesty is enforced rather than assumed.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5EfficiencyWorkDisposition`] slowed-versus-paused vocabulary, the
//! [`EfficiencyPressureSource`] source-of-change vocabulary, the [`EfficiencyState`] active-state
//! vocabulary, and the [`WorkloadFamily`] affected-subsystem vocabulary — so this lane can never
//! fork its own low-power or thermal wording.
//!
//! [matrix]: crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_power_throttle_controls,
    seeded_m5_power_throttle_controls_activity_center_beta_narrowed,
    seeded_m5_power_throttle_controls_diagnostics_preview_narrowed,
    M5_POWER_THROTTLE_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::efficiency::governance::M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF;
use crate::efficiency::{EfficiencyPressureSource, EfficiencyState, WorkloadFamily};
use crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix::{
    M5EfficiencyAccessibilityRoute, M5EfficiencyConsumerSurface, M5EfficiencyDeploymentLine,
    M5EfficiencyDowngradeTrigger, M5EfficiencyQualificationClass, M5EfficiencyRequiredLabel,
    M5EfficiencyWorkDisposition, M5_EFFICIENCY_COMPONENT_DOC_REF, M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
    M5_POWER_STATE_INDICATOR_SCHEMA_REF, M5_THROTTLED_SUBSYSTEM_ROW_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5PowerThrottleControlsPacket`].
pub const M5_POWER_THROTTLE_CONTROLS_RECORD_KIND: &str =
    "implement_m5_power_state_indicator_and_throttled_subsystem_row_controls";

/// Schema version for M5 power-state / throttled-subsystem controls records.
pub const M5_POWER_THROTTLE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls boundary schema.
pub const M5_POWER_THROTTLE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-power-state-throttled-subsystem-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_POWER_THROTTLE_CONTROLS_DOC_REF: &str =
    "docs/help/m5_power_state_indicator_and_throttled_subsystem_row_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_POWER_THROTTLE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-power-state-throttled-subsystem-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_POWER_THROTTLE_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-power-state-throttled-subsystem-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_POWER_THROTTLE_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-power-state-throttled-subsystem-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_POWER_THROTTLE_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-power-state-throttled-subsystem-controls";

/// Consumer surface a power-state / throttled controls row projects onto. Reuses the frozen
/// matrix consumer-surface taxonomy so no lane invents a parallel surface set.
pub type M5PowerThrottleConsumerSurface = M5EfficiencyConsumerSurface;

/// One mandatory rendered part a power-state indicator or throttled-subsystem row must be able to
/// show, so no efficiency truth is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PowerThrottleAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The source of change behind the adaptation (power-state indicator).
    SourceOfChange,
    /// The active efficiency state (power-state indicator).
    ActiveEfficiencyState,
    /// The inspect path to fuller detail (power-state indicator).
    InspectPath,
    /// The affected subsystem / workload lane (throttled-subsystem row).
    AffectedSubsystem,
    /// The slowed-versus-paused disposition (throttled-subsystem row).
    SlowedVersusPaused,
    /// The protected tasks that remain preserved (throttled-subsystem row).
    PreservedProtectedWork,
}

impl M5PowerThrottleAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::SourceOfChange,
        Self::ActiveEfficiencyState,
        Self::InspectPath,
        Self::AffectedSubsystem,
        Self::SlowedVersusPaused,
        Self::PreservedProtectedWork,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::SourceOfChange => "source_of_change",
            Self::ActiveEfficiencyState => "active_efficiency_state",
            Self::InspectPath => "inspect_path",
            Self::AffectedSubsystem => "affected_subsystem",
            Self::SlowedVersusPaused => "slowed_versus_paused",
            Self::PreservedProtectedWork => "preserved_protected_work",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PowerThrottleNextAction {
    /// Open the inspect path to fuller power/thermal detail.
    OpenInspectPath,
    /// Open the activity center to see paused / slowed work.
    OpenActivityCenter,
    /// Open efficiency / policy-aware settings.
    OpenEfficiencySettings,
    /// Review diagnostics for the unavailable signal.
    ReviewDiagnostics,
    /// Restore power / clear the pressure source.
    RestorePower,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5PowerThrottleNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenInspectPath,
        Self::OpenActivityCenter,
        Self::OpenEfficiencySettings,
        Self::ReviewDiagnostics,
        Self::RestorePower,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenInspectPath => "open_inspect_path",
            Self::OpenActivityCenter => "open_activity_center",
            Self::OpenEfficiencySettings => "open_efficiency_settings",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::RestorePower => "restore_power",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a power-state / throttled controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PowerThrottleExportField {
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
    /// The source of change named by the power-state indicator.
    SourceOfChange,
    /// The active efficiency state.
    ActiveState,
    /// The affected subsystems named by the throttled row.
    AffectedSubsystems,
    /// The preserved protected work.
    PreservedWork,
    /// The inspect path route.
    InspectPath,
    /// The accountable owner role.
    OwnerRole,
}

impl M5PowerThrottleExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::WorkDispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SourceOfChange,
        Self::ActiveState,
        Self::AffectedSubsystems,
        Self::PreservedWork,
        Self::InspectPath,
        Self::OwnerRole,
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
            Self::SourceOfChange => "source_of_change",
            Self::ActiveState => "active_state",
            Self::AffectedSubsystems => "affected_subsystems",
            Self::PreservedWork => "preserved_work",
            Self::InspectPath => "inspect_path",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a power-state indicator degraded below a clean, fully-legible state. The degrade-first
/// ladder returns one of these instead of ever letting an ambiguous or collapsed indicator read
/// as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PowerStateDegradeReason {
    /// The power/thermal pressure signal is unavailable, so no source can be trusted.
    PressureSignalUnavailable,
    /// The source of change is unstated; the indicator would read as a generic warning.
    SourceOfChangeUnstated,
    /// Multiple distinct causes were collapsed into one generic low-power warning.
    CausesCollapsedIntoGeneric,
    /// No inspect path to fuller detail is offered.
    InspectPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PowerStateDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PressureSignalUnavailable,
        Self::SourceOfChangeUnstated,
        Self::CausesCollapsedIntoGeneric,
        Self::InspectPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PressureSignalUnavailable => "pressure_signal_unavailable",
            Self::SourceOfChangeUnstated => "source_of_change_unstated",
            Self::CausesCollapsedIntoGeneric => "causes_collapsed_into_generic",
            Self::InspectPathMissing => "inspect_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Work disposition an indicator carries while degraded for this reason.
    pub const fn disposition(self) -> M5EfficiencyWorkDisposition {
        M5EfficiencyWorkDisposition::NotEvaluated
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5PowerThrottleNextAction {
        match self {
            Self::PressureSignalUnavailable => M5PowerThrottleNextAction::ReviewDiagnostics,
            Self::SourceOfChangeUnstated | Self::CausesCollapsedIntoGeneric => {
                M5PowerThrottleNextAction::OpenInspectPath
            }
            Self::InspectPathMissing => M5PowerThrottleNextAction::OpenEfficiencySettings,
            Self::ProofStale => M5PowerThrottleNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EfficiencyDowngradeTrigger {
        match self {
            Self::PressureSignalUnavailable => M5EfficiencyDowngradeTrigger::SourceOfChangeUnstated,
            Self::SourceOfChangeUnstated => M5EfficiencyDowngradeTrigger::SourceOfChangeUnstated,
            Self::CausesCollapsedIntoGeneric => {
                M5EfficiencyDowngradeTrigger::GenericLowPowerWordingUsed
            }
            Self::InspectPathMissing => M5EfficiencyDowngradeTrigger::EfficiencyStateUnstated,
            Self::ProofStale => M5EfficiencyDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a throttled-subsystem row degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ThrottledDegradeReason {
    /// No affected subsystem was named, so which work was reduced cannot be told.
    NoAffectedSubsystemNamed,
    /// Slowed work already visible to the user is being hidden by this surface (AC2 violation).
    SlowedWorkSilentlyHidden,
    /// The same lane is claimed both slowed and paused; disposition is ambiguous.
    SlowedVersusPausedAmbiguous,
    /// What still works (preserved protected tasks) is unstated.
    WhatStillWorksUnstated,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ThrottledDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoAffectedSubsystemNamed,
        Self::SlowedWorkSilentlyHidden,
        Self::SlowedVersusPausedAmbiguous,
        Self::WhatStillWorksUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAffectedSubsystemNamed => "no_affected_subsystem_named",
            Self::SlowedWorkSilentlyHidden => "slowed_work_silently_hidden",
            Self::SlowedVersusPausedAmbiguous => "slowed_versus_paused_ambiguous",
            Self::WhatStillWorksUnstated => "what_still_works_unstated",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5PowerThrottleNextAction {
        match self {
            Self::NoAffectedSubsystemNamed | Self::SlowedVersusPausedAmbiguous => {
                M5PowerThrottleNextAction::OpenActivityCenter
            }
            Self::SlowedWorkSilentlyHidden => M5PowerThrottleNextAction::OpenActivityCenter,
            Self::WhatStillWorksUnstated => M5PowerThrottleNextAction::OpenInspectPath,
            Self::ProofStale => M5PowerThrottleNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EfficiencyDowngradeTrigger {
        match self {
            Self::NoAffectedSubsystemNamed => M5EfficiencyDowngradeTrigger::WhatStillWorksUnstated,
            Self::SlowedWorkSilentlyHidden => M5EfficiencyDowngradeTrigger::PausedWorkToastOnly,
            Self::SlowedVersusPausedAmbiguous => {
                M5EfficiencyDowngradeTrigger::SlowedVersusPausedAmbiguous
            }
            Self::WhatStillWorksUnstated => M5EfficiencyDowngradeTrigger::WhatStillWorksUnstated,
            Self::ProofStale => M5EfficiencyDowngradeTrigger::ProofStale,
        }
    }
}

/// Maps a shared active efficiency state to the single controlled work disposition.
const fn disposition_for_state(state: EfficiencyState) -> M5EfficiencyWorkDisposition {
    match state {
        EfficiencyState::Nominal => M5EfficiencyWorkDisposition::RunningFull,
        EfficiencyState::EfficiencyAware | EfficiencyState::ThermalConstrained => {
            M5EfficiencyWorkDisposition::Slowed
        }
        EfficiencyState::ProtectCore => M5EfficiencyWorkDisposition::Paused,
        EfficiencyState::Recovery => M5EfficiencyWorkDisposition::Resuming,
    }
}

/// Input to [`resolve_power_state_indicator`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PowerStateResolutionInput {
    /// Stable identity of the indicator instance.
    pub indicator_id: String,
    /// Source-of-change pressure signals driving the adaptation.
    pub pressure_sources: Vec<EfficiencyPressureSource>,
    /// Active efficiency state.
    pub active_state: EfficiencyState,
    /// True when the power/thermal pressure signal could be read at all.
    pub pressure_signal_available: bool,
    /// True when the surface names each distinct cause rather than collapsing them into one.
    pub distinct_causes_named: bool,
    /// Inspect-path route to fuller detail (empty means no inspect path).
    pub inspect_path: String,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe power-state indicator projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPowerStateIndicator {
    /// Stable identity of the indicator instance.
    pub indicator_id: String,
    /// Source-of-change tokens named by the indicator.
    pub source_of_change: Vec<String>,
    /// Active efficiency state token.
    pub active_state: String,
    /// Single controlled work disposition carried by the indicator.
    pub work_disposition: M5EfficiencyWorkDisposition,
    /// Inspect-path route to fuller detail.
    pub inspect_path: String,
    /// Degrade reason, if the indicator could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5PowerStateDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5PowerThrottleNextAction,
    /// AC1: whether a user can distinguish *why* Aureline adapted from this indicator alone.
    pub distinguishable_cause: bool,
    /// Guardrail (MUST be `false` on a clean indicator): distinct causes collapsed into one
    /// generic warning.
    pub collapses_into_generic_warning: bool,
}

impl M5ResolvedPowerStateIndicator {
    /// Whether this indicator reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_throttled_subsystem_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ThrottledResolutionInput {
    /// Stable identity of the row instance.
    pub row_id: String,
    /// Workload lanes that slowed (still progressing).
    pub slowed_workloads: Vec<WorkloadFamily>,
    /// Workload lanes that paused (not progressing).
    pub paused_workloads: Vec<WorkloadFamily>,
    /// Protected tasks that remain preserved (what still works).
    pub preserved_protected_tasks: Vec<String>,
    /// True when adaptive behavior already became user-visible on this surface.
    pub adaptive_behavior_user_visible: bool,
    /// True when this surface hides the slowed work rather than keeping it visible.
    pub surface_hides_slowed_work: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe throttled-subsystem row projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedThrottledSubsystemRow {
    /// Stable identity of the row instance.
    pub row_id: String,
    /// Slowed workload tokens.
    pub slowed_workloads: Vec<String>,
    /// Paused workload tokens.
    pub paused_workloads: Vec<String>,
    /// Preserved protected tasks (what still works).
    pub preserved_protected_tasks: Vec<String>,
    /// AC1: whether the affected subsystems are named.
    pub affected_subsystems_named: bool,
    /// Controlled work dispositions carried by the row.
    pub work_dispositions: Vec<M5EfficiencyWorkDisposition>,
    /// Degrade reason, if the row could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5ThrottledDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5PowerThrottleNextAction,
    /// Guardrail (MUST be `false` on a clean row): slowed work already visible to the user is
    /// being hidden by this surface.
    pub silently_hid_slowed_work: bool,
}

impl M5ResolvedThrottledSubsystemRow {
    /// Whether this row reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5PowerThrottleResolutionError {
    /// The power-state indicator id was empty.
    EmptyIndicatorId,
    /// The throttled-subsystem row id was empty.
    EmptyRowId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5PowerThrottleResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyIndicatorId => "empty_indicator_id",
            Self::EmptyRowId => "empty_row_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5PowerThrottleResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 power/throttle resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5PowerThrottleResolutionError {}

/// Resolves a power-state indicator, proving AC1: the indicator names its source of change and
/// active state and never reads as a clean generic warning when the cause is unstated or
/// collapsed.
pub fn resolve_power_state_indicator(
    input: M5PowerStateResolutionInput,
) -> Result<M5ResolvedPowerStateIndicator, M5PowerThrottleResolutionError> {
    if input.indicator_id.trim().is_empty() {
        return Err(M5PowerThrottleResolutionError::EmptyIndicatorId);
    }
    if string_is_forbidden(&input.indicator_id) || string_is_forbidden(&input.inspect_path) {
        return Err(M5PowerThrottleResolutionError::ForbiddenMaterial);
    }

    let inspect_present = !input.inspect_path.trim().is_empty();
    let collapses_into_generic_warning =
        input.pressure_sources.len() > 1 && !input.distinct_causes_named;

    let degrade_reason = if !input.pressure_signal_available {
        Some(M5PowerStateDegradeReason::PressureSignalUnavailable)
    } else if input.pressure_sources.is_empty() {
        Some(M5PowerStateDegradeReason::SourceOfChangeUnstated)
    } else if collapses_into_generic_warning {
        Some(M5PowerStateDegradeReason::CausesCollapsedIntoGeneric)
    } else if !inspect_present {
        Some(M5PowerStateDegradeReason::InspectPathMissing)
    } else if !input.proof_fresh {
        Some(M5PowerStateDegradeReason::ProofStale)
    } else {
        None
    };

    let work_disposition = match degrade_reason {
        Some(reason) => reason.disposition(),
        None => disposition_for_state(input.active_state),
    };
    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None if input.active_state == EfficiencyState::Nominal => {
            M5PowerThrottleNextAction::NoActionNeeded
        }
        None => M5PowerThrottleNextAction::OpenInspectPath,
    };

    // AC1: a user can distinguish *why* only when the cause is named, not collapsed, and the
    // signal is available.
    let distinguishable_cause = degrade_reason.is_none()
        || (input.pressure_signal_available
            && !input.pressure_sources.is_empty()
            && !collapses_into_generic_warning);

    Ok(M5ResolvedPowerStateIndicator {
        indicator_id: input.indicator_id,
        source_of_change: input
            .pressure_sources
            .iter()
            .map(|s| s.as_str().to_owned())
            .collect(),
        active_state: input.active_state.as_str().to_owned(),
        work_disposition,
        inspect_path: input.inspect_path,
        degrade_reason,
        next_action,
        distinguishable_cause,
        collapses_into_generic_warning,
    })
}

/// Resolves a throttled-subsystem row, proving AC1 (which subsystems were affected and what still
/// works) and AC2 (slowed work already visible is never silently hidden or widened).
pub fn resolve_throttled_subsystem_row(
    input: M5ThrottledResolutionInput,
) -> Result<M5ResolvedThrottledSubsystemRow, M5PowerThrottleResolutionError> {
    if input.row_id.trim().is_empty() {
        return Err(M5PowerThrottleResolutionError::EmptyRowId);
    }
    if string_is_forbidden(&input.row_id)
        || input
            .preserved_protected_tasks
            .iter()
            .any(|t| string_is_forbidden(t))
    {
        return Err(M5PowerThrottleResolutionError::ForbiddenMaterial);
    }

    let slowed: std::collections::HashSet<WorkloadFamily> =
        input.slowed_workloads.iter().copied().collect();
    let paused: std::collections::HashSet<WorkloadFamily> =
        input.paused_workloads.iter().copied().collect();
    let overlap = slowed.intersection(&paused).next().is_some();
    let affected_subsystems_named = !slowed.is_empty() || !paused.is_empty();
    let silently_hid_slowed_work =
        input.adaptive_behavior_user_visible && input.surface_hides_slowed_work;

    let degrade_reason = if !affected_subsystems_named {
        Some(M5ThrottledDegradeReason::NoAffectedSubsystemNamed)
    } else if silently_hid_slowed_work {
        Some(M5ThrottledDegradeReason::SlowedWorkSilentlyHidden)
    } else if overlap {
        Some(M5ThrottledDegradeReason::SlowedVersusPausedAmbiguous)
    } else if input.preserved_protected_tasks.is_empty() {
        Some(M5ThrottledDegradeReason::WhatStillWorksUnstated)
    } else if !input.proof_fresh {
        Some(M5ThrottledDegradeReason::ProofStale)
    } else {
        None
    };

    let mut work_dispositions = Vec::new();
    if !input.slowed_workloads.is_empty() {
        work_dispositions.push(M5EfficiencyWorkDisposition::Slowed);
    }
    if !input.paused_workloads.is_empty() {
        work_dispositions.push(M5EfficiencyWorkDisposition::Paused);
    }
    if work_dispositions.is_empty() {
        work_dispositions.push(M5EfficiencyWorkDisposition::NotEvaluated);
    }

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5PowerThrottleNextAction::OpenActivityCenter,
    };

    Ok(M5ResolvedThrottledSubsystemRow {
        row_id: input.row_id,
        slowed_workloads: input
            .slowed_workloads
            .iter()
            .map(|w| w.as_str().to_owned())
            .collect(),
        paused_workloads: input
            .paused_workloads
            .iter()
            .map(|w| w.as_str().to_owned())
            .collect(),
        preserved_protected_tasks: input.preserved_protected_tasks,
        affected_subsystems_named,
        work_dispositions,
        degrade_reason,
        next_action,
        silently_hid_slowed_work,
    })
}

/// One controls row: one consumer surface bound to the resolved power-state and throttled
/// examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PowerThrottleControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5PowerThrottleConsumerSurface,
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
    pub anatomy_parts: Vec<M5PowerThrottleAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5PowerThrottleExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5EfficiencyDowngradeTrigger>,
    /// Resolved power-state indicator examples.
    pub power_state_examples: Vec<M5ResolvedPowerStateIndicator>,
    /// Resolved throttled-subsystem row examples.
    pub throttled_subsystem_examples: Vec<M5ResolvedThrottledSubsystemRow>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never collapse distinct pressure sources into one generic warning.
    pub collapses_pressure_sources_into_generic_warning: bool,
    /// Hard invariant: never hide slowed work once adaptive behavior became user-visible.
    pub hides_slowed_work_after_user_visible: bool,
    /// Hard invariant: never leave what-still-works unstated on a throttled row.
    pub hides_what_still_works: bool,
    /// Hard invariant: never invent an alternate label for a governed state.
    pub invents_alternate_state_label: bool,
}

impl M5PowerThrottleControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5PowerThrottleAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5PowerThrottleAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5PowerThrottleExportField> =
            self.export_fields.iter().copied().collect();
        M5PowerThrottleExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.collapses_pressure_sources_into_generic_warning
            && !self.hides_slowed_work_after_user_visible
            && !self.hides_what_still_works
            && !self.invents_alternate_state_label
    }

    /// True when every resolved example on this row is honest: no clean example is collapsed or
    /// silently hidden.
    fn examples_are_honest(&self) -> bool {
        self.power_state_examples
            .iter()
            .all(|ex| !(ex.is_clean() && ex.collapses_into_generic_warning))
            && self
                .throttled_subsystem_examples
                .iter()
                .all(|ex| !(ex.is_clean() && ex.silently_hid_slowed_work))
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PowerThrottleVocabularySet {
    /// Work-disposition tokens (bound from the frozen matrix).
    pub work_dispositions: Vec<String>,
    /// Pressure-source tokens (bound from the efficiency object model).
    pub pressure_sources: Vec<String>,
    /// Efficiency-state tokens (bound from the efficiency object model).
    pub efficiency_states: Vec<String>,
    /// Affected-workload tokens (bound from the efficiency object model).
    pub affected_workloads: Vec<String>,
    /// Power-state degrade-reason tokens.
    pub power_state_degrade_reasons: Vec<String>,
    /// Throttled degrade-reason tokens.
    pub throttled_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5PowerThrottleVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            work_dispositions: tokens(&M5EfficiencyWorkDisposition::ALL, |v| v.as_str()),
            pressure_sources: tokens(&EfficiencyPressureSource::ALL, |v| v.as_str()),
            efficiency_states: tokens(&EfficiencyState::ALL, |v| v.as_str()),
            affected_workloads: tokens(&AFFECTED_WORKLOADS, |v| v.as_str()),
            power_state_degrade_reasons: tokens(&M5PowerStateDegradeReason::ALL, |v| v.as_str()),
            throttled_degrade_reasons: tokens(&M5ThrottledDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5PowerThrottleAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5PowerThrottleNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5PowerThrottleExportField::ALL, |v| v.as_str()),
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
pub struct M5PowerThrottleGovernanceReview {
    /// The power-state indicator always names its source of change and active state.
    pub power_state_indicator_names_source_and_state: bool,
    /// The throttled row always enumerates which subsystems slowed or paused.
    pub throttled_row_enumerates_affected_subsystems: bool,
    /// The throttled row always names what still works (preserved protected tasks).
    pub throttled_row_names_preserved_work: bool,
    /// No indicator collapses distinct pressure sources into one generic warning.
    pub no_indicator_collapses_into_generic_warning: bool,
    /// No surface hides slowed work once adaptive behavior became user-visible.
    pub no_surface_hides_slowed_work_after_user_visible: bool,
    /// Slowed-versus-paused work is always explicit.
    pub slowed_versus_paused_always_explicit: bool,
    /// Every component offers an inspect path or degrades when it cannot.
    pub inspect_path_offered_or_degraded: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PowerThrottleConsumerProjection {
    /// Shell surfaces consume the shared power-state indicator.
    pub shell_surfaces_consume_power_state: bool,
    /// Activity surfaces consume the shared throttled-subsystem rows.
    pub activity_surfaces_consume_throttled_rows: bool,
    /// Diagnostics surfaces consume the shared source-of-change vocabulary.
    pub diagnostics_surfaces_consume_source_vocabulary: bool,
    /// Support / export reads a single canonical efficiency source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PowerThrottleProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PowerThrottleReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting efficiency audit for the lane.
    pub efficiency_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5PowerThrottleControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PowerThrottleControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5PowerThrottleControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PowerThrottleVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PowerThrottleGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PowerThrottleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PowerThrottleProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PowerThrottleReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 power-state / throttled-subsystem controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PowerThrottleControlsPacket {
    /// Record kind; must equal [`M5_POWER_THROTTLE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_POWER_THROTTLE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5PowerThrottleControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PowerThrottleVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PowerThrottleGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PowerThrottleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PowerThrottleProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PowerThrottleReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PowerThrottleControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5PowerThrottleControlsPacketInput) -> Self {
        Self {
            record_kind: M5_POWER_THROTTLE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_POWER_THROTTLE_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5PowerThrottleControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_POWER_THROTTLE_CONTROLS_RECORD_KIND {
            violations.push(M5PowerThrottleControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_POWER_THROTTLE_CONTROLS_SCHEMA_VERSION {
            violations.push(M5PowerThrottleControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PowerThrottleControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5PowerThrottleControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 power/throttle controls packet serializes"),
        ) {
            violations.push(M5PowerThrottleControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 power/throttle controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,power_state_examples,throttled_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .power_state_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.throttled_subsystem_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.power_state_examples.len(),
                row.throttled_subsystem_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Power-State-Indicator and Throttled-Subsystem-Row Controls\n\n");
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
                "  - Power-state examples: {} / throttled examples: {}\n",
                row.power_state_examples.len(),
                row.throttled_subsystem_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5PowerThrottleControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PowerThrottleControlsViolation>),
}

impl fmt::Display for M5PowerThrottleControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 power/throttle controls export parse failed: {error}"
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
                    "m5 power/throttle controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PowerThrottleControlsArtifactError {}

/// Validation failures emitted by [`M5PowerThrottleControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PowerThrottleControlsViolation {
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
    /// A controls row carries a dishonest clean example (collapsed or silently hidden).
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
    /// AC1 is not proven: no example distinguishes why Aureline adapted, or none shows a
    /// collapsed cause degrading.
    Ac1NotProven,
    /// AC2 is not proven: no example shows slowed work degrading rather than being silently
    /// hidden once user-visible.
    Ac2NotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5PowerThrottleControlsViolation {
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
pub fn current_stable_m5_power_throttle_controls_export(
) -> Result<M5PowerThrottleControlsPacket, M5PowerThrottleControlsArtifactError> {
    let packet: M5PowerThrottleControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-power-state-throttled-subsystem-controls-proof/support_export.json"
    )))
    .map_err(M5PowerThrottleControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PowerThrottleControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5PowerThrottleControlsPacket,
    violations: &mut Vec<M5PowerThrottleControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_POWER_THROTTLE_CONTROLS_SCHEMA_REF,
        M5_POWER_THROTTLE_CONTROLS_DOC_REF,
        M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
        M5_EFFICIENCY_COMPONENT_DOC_REF,
        M5_POWER_STATE_INDICATOR_SCHEMA_REF,
        M5_THROTTLED_SUBSYSTEM_ROW_SCHEMA_REF,
        M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5PowerThrottleControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5PowerThrottleControlsPacket,
    violations: &mut Vec<M5PowerThrottleControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5PowerThrottleControlsViolation::NoControlsRows);
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
            violations.push(M5PowerThrottleControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5PowerThrottleControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5PowerThrottleControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_POWER_STATE_INDICATOR_SCHEMA_REF)
            || !refs.contains(M5_THROTTLED_SUBSYSTEM_ROW_SCHEMA_REF)
        {
            violations.push(M5PowerThrottleControlsViolation::ComponentSchemaRefMissing);
        }
        if row.power_state_examples.is_empty() || row.throttled_subsystem_examples.is_empty() {
            violations.push(M5PowerThrottleControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5PowerThrottleControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5PowerThrottleControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5PowerThrottleControlsPacket,
    violations: &mut Vec<M5PowerThrottleControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.power_state_indicator_names_source_and_state,
        review.throttled_row_enumerates_affected_subsystems,
        review.throttled_row_names_preserved_work,
        review.no_indicator_collapses_into_generic_warning,
        review.no_surface_hides_slowed_work_after_user_visible,
        review.slowed_versus_paused_always_explicit,
        review.inspect_path_offered_or_degraded,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5PowerThrottleControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5PowerThrottleControlsPacket,
    violations: &mut Vec<M5PowerThrottleControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_surfaces_consume_power_state,
        projection.activity_surfaces_consume_throttled_rows,
        projection.diagnostics_surfaces_consume_source_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5PowerThrottleControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5PowerThrottleControlsPacket,
    violations: &mut Vec<M5PowerThrottleControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5PowerThrottleControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5PowerThrottleControlsPacket,
    violations: &mut Vec<M5PowerThrottleControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.efficiency_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5PowerThrottleControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5PowerThrottleControlsPacket,
    violations: &mut Vec<M5PowerThrottleControlsViolation>,
) {
    let power_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.power_state_examples.iter())
    };
    let throttled_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.throttled_subsystem_examples.iter())
    };

    // AC1: at least one clean power-state indicator distinguishes *why* with a named source, and
    // at least one throttled row names *which* subsystems were affected; plus at least one
    // collapsed / unstated cause degrades rather than reading clean.
    let clean_distinguishable = power_examples()
        .any(|ex| ex.is_clean() && ex.distinguishable_cause && ex.source_of_change.len() >= 2);
    let affected_named =
        throttled_examples().any(|ex| ex.is_clean() && ex.affected_subsystems_named);
    let collapsed_degrades = power_examples().any(|ex| {
        matches!(
            ex.degrade_reason,
            Some(M5PowerStateDegradeReason::CausesCollapsedIntoGeneric)
                | Some(M5PowerStateDegradeReason::SourceOfChangeUnstated)
        )
    });
    if !(clean_distinguishable && affected_named && collapsed_degrades) {
        violations.push(M5PowerThrottleControlsViolation::Ac1NotProven);
    }

    // AC2: at least one throttled example proves that slowed work already visible to the user
    // degrades to SlowedWorkSilentlyHidden rather than being hidden, and no clean example hides
    // slowed work.
    let hidden_degrades = throttled_examples().any(|ex| {
        ex.degrade_reason == Some(M5ThrottledDegradeReason::SlowedWorkSilentlyHidden)
            && ex.silently_hid_slowed_work
    });
    let no_clean_hides =
        throttled_examples().all(|ex| !(ex.is_clean() && ex.silently_hid_slowed_work));
    if !(hidden_degrades && no_clean_hides) {
        violations.push(M5PowerThrottleControlsViolation::Ac2NotProven);
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
