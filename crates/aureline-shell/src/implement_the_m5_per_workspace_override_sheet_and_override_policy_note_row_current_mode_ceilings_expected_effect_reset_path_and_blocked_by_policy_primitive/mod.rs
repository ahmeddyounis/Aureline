//! Implemented M5 per-workspace override-sheet and override-policy note-row primitives.
//!
//! The frozen [efficiency component matrix][matrix] names the reusable adaptive-efficiency UI
//! components and locks their controlled vocabulary. The [power-state / throttled lane][power] and
//! the [background-work lane][background] turned the first four components into resolvers. This
//! module is the third implement lane over that matrix: it turns the **per-workspace override
//! sheet** (a preview of the current efficiency mode, the allowed policy ceilings, the expected
//! effect on indexing / AI / extensions, and the exact reset path) and the **override-policy note
//! row** (an explanation of when an override is blocked, who owns the policy, and what remains
//! changeable locally) into resolvers that produce export-safe, honest projections instead of
//! dead or misleading override controls.
//!
//! Two acceptance criteria drive the resolvers:
//!
//! * **AC1 — users never see a dead or misleading override control when policy disallows the
//!   requested behavior change.** [`resolve_override_sheet`] and [`resolve_policy_note_row`] refuse
//!   to read as clean when the surface still presents an override as an actionable control while
//!   its policy posture blocks the override; they degrade to
//!   [`M5OverrideSheetDegradeReason::DeadOverrideControlOffered`] and
//!   [`M5PolicyNoteDegradeReason::DeadOverrideControlOffered`] instead. A clean surface facing a
//!   blocking policy shows the override as blocked-by-policy, names the owner, and states what
//!   remains changeable locally.
//! * **AC2 — override sheets are explicit about the performance-versus-freshness trade-off and
//!   never hide side effects behind generic efficiency language.** [`resolve_override_sheet`]
//!   degrades to [`M5OverrideSheetDegradeReason::PerformanceFreshnessTradeoffUnstated`] the moment
//!   a sheet omits the trade-off, and to
//!   [`M5OverrideSheetDegradeReason::SideEffectsHiddenByGenericLanguage`] the moment it collapses
//!   the expected effect into generic low-power wording.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5EfficiencyWorkDisposition`] vocabulary, the [`EfficiencyState`] current-mode vocabulary,
//! the [`WorkloadFamily`] affected-work vocabulary, the [`OverridePosture`] override vocabulary,
//! and the [`M5EfficiencyPolicyOwner`] policy-owner vocabulary — so this lane can never fork its
//! own override or policy wording.
//!
//! [matrix]: crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix
//! [power]: crate::implement_the_m5_power_state_indicator_and_throttled_subsystem_row_source_active_state_affected_subsystem_and_inspect_path_primitive
//! [background]: crate::implement_the_m5_background_work_row_and_background_work_banner_affected_work_class_state_what_still_works_resume_condition_and_override_primitive

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_override_controls, seeded_m5_override_controls_activity_center_preview_narrowed,
    seeded_m5_override_controls_override_settings_beta_narrowed, M5_OVERRIDE_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::efficiency::governance::{OverridePosture, M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF};
use crate::efficiency::{EfficiencyState, WorkloadFamily};
use crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix::{
    M5EfficiencyAccessibilityRoute, M5EfficiencyConsumerSurface, M5EfficiencyDeploymentLine,
    M5EfficiencyDowngradeTrigger, M5EfficiencyPolicyOwner, M5EfficiencyQualificationClass,
    M5EfficiencyRequiredLabel, M5EfficiencyWorkDisposition, M5_EFFICIENCY_COMPONENT_DOC_REF,
    M5_EFFICIENCY_COMPONENT_SCHEMA_REF, M5_EFFICIENCY_OVERRIDE_SHEET_SCHEMA_REF,
    M5_OVERRIDE_POLICY_NOTE_ROW_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5OverrideControlsPacket`].
pub const M5_OVERRIDE_CONTROLS_RECORD_KIND: &str =
    "implement_m5_per_workspace_override_sheet_and_override_policy_note_row_controls";

/// Schema version for M5 override sheet / policy-note controls records.
pub const M5_OVERRIDE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls boundary schema.
pub const M5_OVERRIDE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-override-sheet-policy-note-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_OVERRIDE_CONTROLS_DOC_REF: &str =
    "docs/help/m5_override_sheet_and_policy_note_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_OVERRIDE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-override-sheet-policy-note-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_OVERRIDE_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-override-sheet-policy-note-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_OVERRIDE_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-override-sheet-policy-note-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_OVERRIDE_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-override-sheet-policy-note-controls";

/// Consumer surface an override sheet / policy note projects onto. Reuses the frozen matrix
/// consumer-surface taxonomy so no lane invents a parallel surface set.
pub type M5OverrideConsumerSurface = M5EfficiencyConsumerSurface;

/// One mandatory rendered part an override sheet or policy note must be able to show, so no
/// override truth is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverrideAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The current efficiency mode the override sheet previews.
    CurrentMode,
    /// The allowed policy ceiling / limit an override may reach.
    AllowedCeiling,
    /// The expected effect on indexing / AI / extensions.
    ExpectedEffect,
    /// The exact reset path back to the policy default.
    ResetPath,
    /// The accountable policy owner behind the adaptation or block.
    PolicyOwner,
    /// What remains changeable locally when the override is blocked.
    LocalChangeability,
}

impl M5OverrideAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::CurrentMode,
        Self::AllowedCeiling,
        Self::ExpectedEffect,
        Self::ResetPath,
        Self::PolicyOwner,
        Self::LocalChangeability,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::CurrentMode => "current_mode",
            Self::AllowedCeiling => "allowed_ceiling",
            Self::ExpectedEffect => "expected_effect",
            Self::ResetPath => "reset_path",
            Self::PolicyOwner => "policy_owner",
            Self::LocalChangeability => "local_changeability",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverrideNextAction {
    /// Open efficiency / policy-aware override settings.
    OpenOverrideSettings,
    /// Open the override-policy note explaining the block and its owner.
    OpenPolicyNote,
    /// Open the activity center to review adapting work.
    OpenActivityCenter,
    /// Review diagnostics for the unavailable signal.
    ReviewDiagnostics,
    /// Restore power / clear the pressure source.
    RestorePower,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5OverrideNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenOverrideSettings,
        Self::OpenPolicyNote,
        Self::OpenActivityCenter,
        Self::ReviewDiagnostics,
        Self::RestorePower,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenOverrideSettings => "open_override_settings",
            Self::OpenPolicyNote => "open_policy_note",
            Self::OpenActivityCenter => "open_activity_center",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::RestorePower => "restore_power",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field an override controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverrideExportField {
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
    /// The current efficiency mode previewed by the sheet.
    CurrentMode,
    /// The override posture behind the adaptation.
    OverridePosture,
    /// The accountable policy owner.
    PolicyOwner,
    /// The expected effect on indexing / AI / extensions.
    ExpectedEffect,
    /// The exact reset path.
    ResetPath,
    /// What remains changeable locally.
    LocalChangeability,
}

impl M5OverrideExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::WorkDispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::CurrentMode,
        Self::OverridePosture,
        Self::PolicyOwner,
        Self::ExpectedEffect,
        Self::ResetPath,
        Self::LocalChangeability,
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
            Self::CurrentMode => "current_mode",
            Self::OverridePosture => "override_posture",
            Self::PolicyOwner => "policy_owner",
            Self::ExpectedEffect => "expected_effect",
            Self::ResetPath => "reset_path",
            Self::LocalChangeability => "local_changeability",
        }
    }
}

/// Reason an override sheet degraded below a clean, fully-legible state. The degrade-first ladder
/// returns one of these instead of ever letting a dead, misleading, or generic sheet read as a
/// clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverrideSheetDegradeReason {
    /// No expected effect on any workload was named, so what the override would change cannot be
    /// told.
    ExpectedEffectUnstated,
    /// An override control is presented as actionable even though policy blocks it (AC1
    /// violation): the user would face a dead or misleading control.
    DeadOverrideControlOffered,
    /// The sheet does not state the performance-versus-freshness trade-off (AC2 violation).
    PerformanceFreshnessTradeoffUnstated,
    /// The sheet collapses the expected effect into generic low-power / efficiency language,
    /// hiding side effects (AC2 violation).
    SideEffectsHiddenByGenericLanguage,
    /// The allowed policy ceiling / limit the override may reach is unstated.
    AllowedCeilingUnstated,
    /// The exact reset path back to the policy default is unstated.
    ResetPathUnstated,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5OverrideSheetDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ExpectedEffectUnstated,
        Self::DeadOverrideControlOffered,
        Self::PerformanceFreshnessTradeoffUnstated,
        Self::SideEffectsHiddenByGenericLanguage,
        Self::AllowedCeilingUnstated,
        Self::ResetPathUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpectedEffectUnstated => "expected_effect_unstated",
            Self::DeadOverrideControlOffered => "dead_override_control_offered",
            Self::PerformanceFreshnessTradeoffUnstated => "performance_freshness_tradeoff_unstated",
            Self::SideEffectsHiddenByGenericLanguage => "side_effects_hidden_by_generic_language",
            Self::AllowedCeilingUnstated => "allowed_ceiling_unstated",
            Self::ResetPathUnstated => "reset_path_unstated",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5OverrideNextAction {
        match self {
            Self::DeadOverrideControlOffered => M5OverrideNextAction::OpenPolicyNote,
            Self::ExpectedEffectUnstated
            | Self::PerformanceFreshnessTradeoffUnstated
            | Self::SideEffectsHiddenByGenericLanguage
            | Self::AllowedCeilingUnstated
            | Self::ResetPathUnstated => M5OverrideNextAction::OpenOverrideSettings,
            Self::ProofStale => M5OverrideNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EfficiencyDowngradeTrigger {
        match self {
            Self::ExpectedEffectUnstated | Self::PerformanceFreshnessTradeoffUnstated => {
                M5EfficiencyDowngradeTrigger::WhatStillWorksUnstated
            }
            Self::DeadOverrideControlOffered
            | Self::AllowedCeilingUnstated
            | Self::ResetPathUnstated => M5EfficiencyDowngradeTrigger::OverrideAvailabilityUnstated,
            Self::SideEffectsHiddenByGenericLanguage => {
                M5EfficiencyDowngradeTrigger::GenericLowPowerWordingUsed
            }
            Self::ProofStale => M5EfficiencyDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason an override-policy note row degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PolicyNoteDegradeReason {
    /// No policy owner could be resolved, so who owns the block cannot be told.
    PolicyOwnerUnresolved,
    /// An override control is presented as actionable even though policy blocks it (AC1
    /// violation): the user would face a dead or misleading control.
    DeadOverrideControlOffered,
    /// The override is blocked but the note does not explain when or why it is blocked.
    BlockReasonUnexplained,
    /// What remains changeable locally is unstated, so the user cannot tell what they can still
    /// change.
    LocalChangeabilityUnstated,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PolicyNoteDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PolicyOwnerUnresolved,
        Self::DeadOverrideControlOffered,
        Self::BlockReasonUnexplained,
        Self::LocalChangeabilityUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyOwnerUnresolved => "policy_owner_unresolved",
            Self::DeadOverrideControlOffered => "dead_override_control_offered",
            Self::BlockReasonUnexplained => "block_reason_unexplained",
            Self::LocalChangeabilityUnstated => "local_changeability_unstated",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5OverrideNextAction {
        match self {
            Self::PolicyOwnerUnresolved | Self::ProofStale => {
                M5OverrideNextAction::ReviewDiagnostics
            }
            Self::DeadOverrideControlOffered | Self::BlockReasonUnexplained => {
                M5OverrideNextAction::OpenPolicyNote
            }
            Self::LocalChangeabilityUnstated => M5OverrideNextAction::OpenOverrideSettings,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EfficiencyDowngradeTrigger {
        match self {
            Self::PolicyOwnerUnresolved => M5EfficiencyDowngradeTrigger::PolicyOwnerUnstated,
            Self::DeadOverrideControlOffered | Self::BlockReasonUnexplained => {
                M5EfficiencyDowngradeTrigger::OverrideAvailabilityUnstated
            }
            Self::LocalChangeabilityUnstated => {
                M5EfficiencyDowngradeTrigger::WhatStillWorksUnstated
            }
            Self::ProofStale => M5EfficiencyDowngradeTrigger::ProofStale,
        }
    }
}

/// True when an override posture blocks the override outright, so presenting the override as an
/// actionable control would be dead or misleading.
const fn override_is_blocked(posture: OverridePosture) -> bool {
    matches!(
        posture,
        OverridePosture::NotOverridable
            | OverridePosture::PolicyBlocked
            | OverridePosture::AdminControlled
    )
}

/// Input to [`resolve_override_sheet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5OverrideSheetResolutionInput {
    /// Stable identity of the sheet instance.
    pub sheet_id: String,
    /// The current efficiency mode the sheet previews.
    pub current_mode: EfficiencyState,
    /// The workloads whose expected effect the sheet previews (indexing / AI / extensions).
    pub expected_effect_workloads: Vec<WorkloadFamily>,
    /// Override posture the sheet exposes.
    pub override_posture: OverridePosture,
    /// True when the sheet presents the override as an actionable control the user can flip.
    pub override_presented_actionable: bool,
    /// Policy owner accountable for the adaptation / override.
    pub policy_owner: M5EfficiencyPolicyOwner,
    /// True when the sheet states the allowed policy ceiling / limit an override may reach.
    pub allowed_ceiling_stated: bool,
    /// True when the sheet states the performance-versus-freshness trade-off.
    pub performance_freshness_tradeoff_stated: bool,
    /// True when the sheet collapses the expected effect into generic efficiency language.
    pub uses_generic_efficiency_language: bool,
    /// Exact reset path back to the policy default (`None` means unstated).
    pub reset_path: Option<String>,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe override-sheet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedOverrideSheet {
    /// Stable identity of the sheet instance.
    pub sheet_id: String,
    /// Current efficiency-mode token the sheet previews.
    pub current_mode: String,
    /// Expected-effect workload tokens.
    pub expected_effect_workloads: Vec<String>,
    /// Single controlled work disposition carried by the sheet.
    pub work_disposition: M5EfficiencyWorkDisposition,
    /// Override posture token.
    pub override_posture: String,
    /// Policy owner token.
    pub policy_owner: String,
    /// Whether an override is honestly available to the user.
    pub override_available: bool,
    /// Whether the allowed policy ceiling / limit is stated.
    pub allowed_ceiling_stated: bool,
    /// Whether the performance-versus-freshness trade-off is stated.
    pub performance_freshness_tradeoff_stated: bool,
    /// Whether the sheet hides side effects behind generic efficiency language.
    pub hides_side_effects_generic_language: bool,
    /// Reset-path token, if stated.
    pub reset_path: Option<String>,
    /// AC1: whether the override control is honest under the current policy (never dead or
    /// misleading).
    pub override_control_honest: bool,
    /// Degrade reason, if the sheet could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5OverrideSheetDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5OverrideNextAction,
    /// Guardrail (MUST be `false` on a clean sheet): a dead / misleading override control was
    /// presented while policy blocks it.
    pub presented_dead_override_control: bool,
}

impl M5ResolvedOverrideSheet {
    /// Whether this sheet reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_policy_note_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PolicyNoteResolutionInput {
    /// Stable identity of the note instance.
    pub note_id: String,
    /// Override posture behind the adaptation.
    pub override_posture: OverridePosture,
    /// True when the note presents the override as an actionable control the user can flip.
    pub override_presented_actionable: bool,
    /// Policy owner accountable for the adaptation / block.
    pub policy_owner: M5EfficiencyPolicyOwner,
    /// True when the note explains when or why the override is blocked.
    pub block_reason_explained: bool,
    /// Workloads or lanes that remain changeable locally while the override is blocked.
    pub locally_changeable: Vec<WorkloadFamily>,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe override-policy note-row projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPolicyNoteRow {
    /// Stable identity of the note instance.
    pub note_id: String,
    /// Single controlled work disposition carried by the note.
    pub work_disposition: M5EfficiencyWorkDisposition,
    /// Override posture token.
    pub override_posture: String,
    /// Policy owner token.
    pub policy_owner: String,
    /// Whether an override is honestly available to the user.
    pub override_available: bool,
    /// Whether the note explains when or why the override is blocked.
    pub block_reason_explained: bool,
    /// Locally-changeable workload tokens (what remains changeable locally).
    pub locally_changeable: Vec<String>,
    /// AC1: whether the override control is honest under the current policy (never dead or
    /// misleading).
    pub override_control_honest: bool,
    /// Degrade reason, if the note could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5PolicyNoteDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5OverrideNextAction,
    /// Guardrail (MUST be `false` on a clean note): a dead / misleading override control was
    /// presented while policy blocks it.
    pub presented_dead_override_control: bool,
}

impl M5ResolvedPolicyNoteRow {
    /// Whether this note reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5OverrideResolutionError {
    /// The override-sheet id was empty.
    EmptySheetId,
    /// The override-policy note id was empty.
    EmptyNoteId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5OverrideResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySheetId => "empty_sheet_id",
            Self::EmptyNoteId => "empty_note_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5OverrideResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "m5 override resolution error: {}", self.as_str())
    }
}

impl Error for M5OverrideResolutionError {}

/// Resolves an override sheet, proving AC1 (never a dead override control when policy disallows the
/// change) and AC2 (explicit about the performance-versus-freshness trade-off, never hiding side
/// effects behind generic efficiency language).
pub fn resolve_override_sheet(
    input: M5OverrideSheetResolutionInput,
) -> Result<M5ResolvedOverrideSheet, M5OverrideResolutionError> {
    if input.sheet_id.trim().is_empty() {
        return Err(M5OverrideResolutionError::EmptySheetId);
    }
    if string_is_forbidden(&input.sheet_id)
        || input.reset_path.as_deref().is_some_and(string_is_forbidden)
    {
        return Err(M5OverrideResolutionError::ForbiddenMaterial);
    }

    let blocked = override_is_blocked(input.override_posture);
    let presented_dead_override_control = input.override_presented_actionable && blocked;

    let degrade_reason = if input.expected_effect_workloads.is_empty() {
        Some(M5OverrideSheetDegradeReason::ExpectedEffectUnstated)
    } else if presented_dead_override_control {
        Some(M5OverrideSheetDegradeReason::DeadOverrideControlOffered)
    } else if !input.performance_freshness_tradeoff_stated {
        Some(M5OverrideSheetDegradeReason::PerformanceFreshnessTradeoffUnstated)
    } else if input.uses_generic_efficiency_language {
        Some(M5OverrideSheetDegradeReason::SideEffectsHiddenByGenericLanguage)
    } else if !input.allowed_ceiling_stated {
        Some(M5OverrideSheetDegradeReason::AllowedCeilingUnstated)
    } else if input.reset_path.is_none() {
        Some(M5OverrideSheetDegradeReason::ResetPathUnstated)
    } else if !input.proof_fresh {
        Some(M5OverrideSheetDegradeReason::ProofStale)
    } else {
        None
    };

    let work_disposition = if input.expected_effect_workloads.is_empty() {
        M5EfficiencyWorkDisposition::NotEvaluated
    } else if blocked {
        M5EfficiencyWorkDisposition::OverrideBlocked
    } else {
        M5EfficiencyWorkDisposition::OverrideAvailable
    };

    let override_available = input.override_presented_actionable && !blocked;
    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5OverrideNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedOverrideSheet {
        sheet_id: input.sheet_id,
        current_mode: input.current_mode.as_str().to_owned(),
        expected_effect_workloads: input
            .expected_effect_workloads
            .iter()
            .map(|w| w.as_str().to_owned())
            .collect(),
        work_disposition,
        override_posture: input.override_posture.as_str().to_owned(),
        policy_owner: input.policy_owner.as_str().to_owned(),
        override_available,
        allowed_ceiling_stated: input.allowed_ceiling_stated,
        performance_freshness_tradeoff_stated: input.performance_freshness_tradeoff_stated,
        hides_side_effects_generic_language: input.uses_generic_efficiency_language,
        reset_path: input.reset_path,
        override_control_honest: !presented_dead_override_control,
        degrade_reason,
        next_action,
        presented_dead_override_control,
    })
}

/// Resolves an override-policy note row, proving AC1: when policy blocks the override, the note
/// explains the block, names the owner, and states what remains changeable locally rather than
/// presenting a dead or misleading override control.
pub fn resolve_policy_note_row(
    input: M5PolicyNoteResolutionInput,
) -> Result<M5ResolvedPolicyNoteRow, M5OverrideResolutionError> {
    if input.note_id.trim().is_empty() {
        return Err(M5OverrideResolutionError::EmptyNoteId);
    }
    if string_is_forbidden(&input.note_id) {
        return Err(M5OverrideResolutionError::ForbiddenMaterial);
    }

    let blocked = override_is_blocked(input.override_posture);
    let presented_dead_override_control = input.override_presented_actionable && blocked;

    let degrade_reason = if input.policy_owner == M5EfficiencyPolicyOwner::NoOwnerResolved {
        Some(M5PolicyNoteDegradeReason::PolicyOwnerUnresolved)
    } else if presented_dead_override_control {
        Some(M5PolicyNoteDegradeReason::DeadOverrideControlOffered)
    } else if blocked && !input.block_reason_explained {
        Some(M5PolicyNoteDegradeReason::BlockReasonUnexplained)
    } else if input.locally_changeable.is_empty() {
        Some(M5PolicyNoteDegradeReason::LocalChangeabilityUnstated)
    } else if !input.proof_fresh {
        Some(M5PolicyNoteDegradeReason::ProofStale)
    } else {
        None
    };

    let work_disposition = if input.policy_owner == M5EfficiencyPolicyOwner::NoOwnerResolved {
        M5EfficiencyWorkDisposition::NotEvaluated
    } else if blocked {
        M5EfficiencyWorkDisposition::PolicyBlocked
    } else {
        M5EfficiencyWorkDisposition::OverrideAvailable
    };

    let override_available = input.override_presented_actionable && !blocked;
    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5OverrideNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedPolicyNoteRow {
        note_id: input.note_id,
        work_disposition,
        override_posture: input.override_posture.as_str().to_owned(),
        policy_owner: input.policy_owner.as_str().to_owned(),
        override_available,
        block_reason_explained: input.block_reason_explained,
        locally_changeable: input
            .locally_changeable
            .iter()
            .map(|w| w.as_str().to_owned())
            .collect(),
        override_control_honest: !presented_dead_override_control,
        degrade_reason,
        next_action,
        presented_dead_override_control,
    })
}

/// One controls row: one consumer surface bound to the resolved override-sheet and policy-note
/// examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverrideControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5OverrideConsumerSurface,
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
    pub anatomy_parts: Vec<M5OverrideAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5OverrideExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5EfficiencyDowngradeTrigger>,
    /// Resolved override-sheet examples.
    pub override_sheet_examples: Vec<M5ResolvedOverrideSheet>,
    /// Resolved override-policy note-row examples.
    pub policy_note_examples: Vec<M5ResolvedPolicyNoteRow>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never present an override as available when policy blocks it.
    pub presents_override_available_when_policy_blocks: bool,
    /// Hard invariant: never hide side effects behind generic efficiency language.
    pub hides_side_effects_behind_generic_efficiency_language: bool,
    /// Hard invariant: never collapse distinct pressure sources into one generic warning.
    pub collapses_pressure_sources_into_generic_warning: bool,
    /// Hard invariant: never hide what remains changeable locally.
    pub hides_what_remains_changeable_locally: bool,
}

impl M5OverrideControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5OverrideAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5OverrideAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5OverrideExportField> = self.export_fields.iter().copied().collect();
        M5OverrideExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.presents_override_available_when_policy_blocks
            && !self.hides_side_effects_behind_generic_efficiency_language
            && !self.collapses_pressure_sources_into_generic_warning
            && !self.hides_what_remains_changeable_locally
    }

    /// True when every resolved example on this row is honest: no clean sheet presents a dead
    /// override control, hides the trade-off, or uses generic language; and no clean note presents
    /// a dead override control.
    fn examples_are_honest(&self) -> bool {
        self.override_sheet_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.presented_dead_override_control
                    || !ex.performance_freshness_tradeoff_stated
                    || ex.hides_side_effects_generic_language))
        }) && self
            .policy_note_examples
            .iter()
            .all(|ex| !(ex.is_clean() && ex.presented_dead_override_control))
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverrideVocabularySet {
    /// Work-disposition tokens (bound from the frozen matrix).
    pub work_dispositions: Vec<String>,
    /// Efficiency-state tokens (bound from the efficiency object model) for the current mode.
    pub efficiency_states: Vec<String>,
    /// Affected-workload tokens (bound from the efficiency object model).
    pub affected_workloads: Vec<String>,
    /// Override-posture tokens (bound from the efficiency object model).
    pub override_postures: Vec<String>,
    /// Policy-owner tokens (bound from the frozen matrix).
    pub policy_owners: Vec<String>,
    /// Override-sheet degrade-reason tokens.
    pub sheet_degrade_reasons: Vec<String>,
    /// Policy-note degrade-reason tokens.
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

impl M5OverrideVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            work_dispositions: tokens(&M5EfficiencyWorkDisposition::ALL, |v| v.as_str()),
            efficiency_states: tokens(&EfficiencyState::ALL, |v| v.as_str()),
            affected_workloads: tokens(&AFFECTED_WORKLOADS, |v| v.as_str()),
            override_postures: tokens(&OverridePosture::ALL, |v| v.as_str()),
            policy_owners: tokens(&M5EfficiencyPolicyOwner::ALL, |v| v.as_str()),
            sheet_degrade_reasons: tokens(&M5OverrideSheetDegradeReason::ALL, |v| v.as_str()),
            note_degrade_reasons: tokens(&M5PolicyNoteDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5OverrideAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5OverrideNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5OverrideExportField::ALL, |v| v.as_str()),
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
pub struct M5OverrideGovernanceReview {
    /// The override sheet always previews the current efficiency mode.
    pub sheet_previews_current_mode: bool,
    /// The override sheet always states the allowed policy ceilings.
    pub sheet_states_allowed_ceilings: bool,
    /// The override sheet always states the expected effect on indexing / AI / extensions.
    pub sheet_states_expected_effect: bool,
    /// The override sheet always states the exact reset path.
    pub sheet_states_reset_path: bool,
    /// The override sheet always states the performance-versus-freshness trade-off.
    pub sheet_states_performance_freshness_tradeoff: bool,
    /// No surface ever presents a dead / misleading override control when policy blocks it.
    pub no_dead_override_control_when_policy_blocks: bool,
    /// The policy note always names the accountable policy owner.
    pub note_names_policy_owner: bool,
    /// The policy note always states what remains changeable locally.
    pub note_states_local_changeability: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverrideConsumerProjection {
    /// Override / policy-aware settings surfaces consume the shared override sheet.
    pub override_settings_consumes_shared_sheet: bool,
    /// Shell and activity surfaces consume the shared policy note.
    pub shell_and_activity_consume_shared_note: bool,
    /// Diagnostics surfaces consume the shared override / policy vocabulary.
    pub diagnostics_consumes_override_vocabulary: bool,
    /// Support / export reads a single canonical override source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverrideProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverrideReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting efficiency audit for the lane.
    pub efficiency_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5OverrideControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5OverrideControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5OverrideControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5OverrideVocabularySet,
    /// Governance-review block.
    pub governance_review: M5OverrideGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5OverrideConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5OverrideProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5OverrideReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 override sheet / policy-note controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverrideControlsPacket {
    /// Record kind; must equal [`M5_OVERRIDE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_OVERRIDE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5OverrideControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5OverrideVocabularySet,
    /// Governance-review block.
    pub governance_review: M5OverrideGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5OverrideConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5OverrideProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5OverrideReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5OverrideControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5OverrideControlsPacketInput) -> Self {
        Self {
            record_kind: M5_OVERRIDE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_OVERRIDE_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5OverrideControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_OVERRIDE_CONTROLS_RECORD_KIND {
            violations.push(M5OverrideControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_OVERRIDE_CONTROLS_SCHEMA_VERSION {
            violations.push(M5OverrideControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5OverrideControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5OverrideControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 override controls packet serializes"),
        ) {
            violations.push(M5OverrideControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 override controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,sheet_examples,note_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .override_sheet_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.policy_note_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.override_sheet_examples.len(),
                row.policy_note_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Per-Workspace Override-Sheet and Override-Policy Note-Row Controls\n\n");
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
                "  - Sheet examples: {} / note examples: {}\n",
                row.override_sheet_examples.len(),
                row.policy_note_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5OverrideControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5OverrideControlsViolation>),
}

impl fmt::Display for M5OverrideControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 override controls export parse failed: {error}"
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
                    "m5 override controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5OverrideControlsArtifactError {}

/// Validation failures emitted by [`M5OverrideControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5OverrideControlsViolation {
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
    /// A controls row carries a dishonest clean example (dead override control, hidden trade-off,
    /// or generic language).
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
    /// AC1 is not proven: no sheet or note shows a blocked override degrading to a dead control,
    /// or a clean example still presents a dead control.
    Ac1NotProven,
    /// AC2 is not proven: no sheet shows the trade-off or side-effect language degrading, or a
    /// clean sheet hides the trade-off or uses generic language.
    Ac2NotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5OverrideControlsViolation {
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
pub fn current_stable_m5_override_controls_export(
) -> Result<M5OverrideControlsPacket, M5OverrideControlsArtifactError> {
    let packet: M5OverrideControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-override-sheet-policy-note-controls-proof/support_export.json"
    )))
    .map_err(M5OverrideControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5OverrideControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5OverrideControlsPacket,
    violations: &mut Vec<M5OverrideControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_OVERRIDE_CONTROLS_SCHEMA_REF,
        M5_OVERRIDE_CONTROLS_DOC_REF,
        M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
        M5_EFFICIENCY_COMPONENT_DOC_REF,
        M5_EFFICIENCY_OVERRIDE_SHEET_SCHEMA_REF,
        M5_OVERRIDE_POLICY_NOTE_ROW_SCHEMA_REF,
        M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5OverrideControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5OverrideControlsPacket,
    violations: &mut Vec<M5OverrideControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5OverrideControlsViolation::NoControlsRows);
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
            violations.push(M5OverrideControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5OverrideControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5OverrideControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_EFFICIENCY_OVERRIDE_SHEET_SCHEMA_REF)
            || !refs.contains(M5_OVERRIDE_POLICY_NOTE_ROW_SCHEMA_REF)
        {
            violations.push(M5OverrideControlsViolation::ComponentSchemaRefMissing);
        }
        if row.override_sheet_examples.is_empty() || row.policy_note_examples.is_empty() {
            violations.push(M5OverrideControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5OverrideControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5OverrideControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5OverrideControlsPacket,
    violations: &mut Vec<M5OverrideControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.sheet_previews_current_mode,
        review.sheet_states_allowed_ceilings,
        review.sheet_states_expected_effect,
        review.sheet_states_reset_path,
        review.sheet_states_performance_freshness_tradeoff,
        review.no_dead_override_control_when_policy_blocks,
        review.note_names_policy_owner,
        review.note_states_local_changeability,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5OverrideControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5OverrideControlsPacket,
    violations: &mut Vec<M5OverrideControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.override_settings_consumes_shared_sheet,
        projection.shell_and_activity_consume_shared_note,
        projection.diagnostics_consumes_override_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5OverrideControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5OverrideControlsPacket,
    violations: &mut Vec<M5OverrideControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5OverrideControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5OverrideControlsPacket,
    violations: &mut Vec<M5OverrideControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.efficiency_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5OverrideControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5OverrideControlsPacket,
    violations: &mut Vec<M5OverrideControlsViolation>,
) {
    let sheet_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.override_sheet_examples.iter())
    };
    let note_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.policy_note_examples.iter())
    };

    // AC1: a sheet and a note each degrade to a dead override control when policy blocks, no clean
    // example still presents a dead control, and at least one clean example handles a blocking
    // posture honestly (shown as blocked-by-policy).
    let sheet_dead_degrades = sheet_examples().any(|ex| {
        ex.degrade_reason == Some(M5OverrideSheetDegradeReason::DeadOverrideControlOffered)
            && ex.presented_dead_override_control
    });
    let note_dead_degrades = note_examples().any(|ex| {
        ex.degrade_reason == Some(M5PolicyNoteDegradeReason::DeadOverrideControlOffered)
            && ex.presented_dead_override_control
    });
    let no_clean_dead = sheet_examples()
        .all(|ex| !(ex.is_clean() && ex.presented_dead_override_control))
        && note_examples().all(|ex| !(ex.is_clean() && ex.presented_dead_override_control));
    let clean_blocked_honest = sheet_examples().any(|ex| {
        ex.is_clean() && ex.work_disposition == M5EfficiencyWorkDisposition::OverrideBlocked
    }) || note_examples().any(|ex| {
        ex.is_clean() && ex.work_disposition == M5EfficiencyWorkDisposition::PolicyBlocked
    });
    if !(sheet_dead_degrades && note_dead_degrades && no_clean_dead && clean_blocked_honest) {
        violations.push(M5OverrideControlsViolation::Ac1NotProven);
    }

    // AC2: at least one sheet degrades because the trade-off is unstated and at least one because
    // side effects were hidden behind generic language, and no clean sheet hides either.
    let tradeoff_degrades = sheet_examples().any(|ex| {
        ex.degrade_reason
            == Some(M5OverrideSheetDegradeReason::PerformanceFreshnessTradeoffUnstated)
    });
    let generic_degrades = sheet_examples().any(|ex| {
        ex.degrade_reason == Some(M5OverrideSheetDegradeReason::SideEffectsHiddenByGenericLanguage)
            && ex.hides_side_effects_generic_language
    });
    let no_clean_hides = sheet_examples().all(|ex| {
        !(ex.is_clean()
            && (!ex.performance_freshness_tradeoff_stated
                || ex.hides_side_effects_generic_language))
    });
    if !(tradeoff_degrades && generic_degrades && no_clean_hides) {
        violations.push(M5OverrideControlsViolation::Ac2NotProven);
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
