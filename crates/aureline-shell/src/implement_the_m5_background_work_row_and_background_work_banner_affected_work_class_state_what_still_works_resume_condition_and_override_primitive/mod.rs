//! Implemented M5 background-work-row and background-work-banner primitives.
//!
//! The frozen [efficiency component matrix][matrix] names the reusable adaptive-efficiency UI
//! components and locks their controlled vocabulary. The [power-state / throttled lane][power]
//! turned the two top-of-funnel components into resolvers. This module is the second implement
//! lane over that matrix: it turns the **background-work row** (one job's slowed-versus-paused
//! disposition, affected work class, what still works, resume condition, and override existence)
//! and the **background-work banner** (aggregate paused / slowed work coalesced into one durable
//! surface) into resolvers that produce export-safe, honest projections instead of transient
//! toast noise.
//!
//! Two acceptance criteria drive the resolvers:
//!
//! * **AC1 — paused indexing, AI warmups, docs sync, prebuild refresh, or package metadata
//!   refresh remain reviewable after the user looks away.** [`resolve_background_work_row`]
//!   refuses to read as a clean row when its adaptive change became user-visible but the row is
//!   only carried in transient, toast-only messaging that vanishes after dismissal; it degrades
//!   to [`M5BackgroundWorkRowDegradeReason::ToastOnlyNotDurable`] instead. A clean row names its
//!   affected work class, its current disposition, what still works, when or how the work may
//!   resume, and whether an override exists, so the work stays reviewable in a durable shell or
//!   activity surface.
//! * **AC2 — broad or repeated pressure events never degrade into duplicate toast spam or
//!   generic service-failure copy.** [`resolve_background_work_banner`] coalesces repeated
//!   pressure events into a single durable banner and degrades to
//!   [`M5BackgroundWorkBannerDegradeReason::DuplicateToastSpam`] the moment it would emit one
//!   toast per event, and to [`M5BackgroundWorkBannerDegradeReason::GenericServiceFailureCopy`]
//!   the moment adaptive-efficiency truth is collapsed into a generic "something went wrong"
//!   message.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5EfficiencyWorkDisposition`] slowed-versus-paused vocabulary, the [`WorkloadFamily`]
//! affected-work-class vocabulary, the [`OverridePosture`] override vocabulary, and the
//! [`EfficiencyRecoveryState`] resume vocabulary — so this lane can never fork its own
//! background-work wording.
//!
//! [matrix]: crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix
//! [power]: crate::implement_the_m5_power_state_indicator_and_throttled_subsystem_row_source_active_state_affected_subsystem_and_inspect_path_primitive

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_background_work_controls,
    seeded_m5_background_work_controls_activity_center_beta_narrowed,
    seeded_m5_background_work_controls_background_work_preview_narrowed,
    M5_BACKGROUND_WORK_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::efficiency::governance::{
    EfficiencyRecoveryState, OverridePosture, M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
};
use crate::efficiency::WorkloadFamily;
use crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix::{
    M5EfficiencyAccessibilityRoute, M5EfficiencyConsumerSurface, M5EfficiencyDeploymentLine,
    M5EfficiencyDowngradeTrigger, M5EfficiencyPolicyOwner, M5EfficiencyQualificationClass,
    M5EfficiencyRequiredLabel, M5EfficiencyWorkDisposition, M5_BACKGROUND_WORK_BANNER_SCHEMA_REF,
    M5_BACKGROUND_WORK_ROW_SCHEMA_REF, M5_EFFICIENCY_COMPONENT_DOC_REF,
    M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5BackgroundWorkControlsPacket`].
pub const M5_BACKGROUND_WORK_CONTROLS_RECORD_KIND: &str =
    "implement_m5_background_work_row_and_background_work_banner_controls";

/// Schema version for M5 background-work row / banner controls records.
pub const M5_BACKGROUND_WORK_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls boundary schema.
pub const M5_BACKGROUND_WORK_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-background-work-row-banner-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_BACKGROUND_WORK_CONTROLS_DOC_REF: &str =
    "docs/help/m5_background_work_row_and_banner_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BACKGROUND_WORK_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-background-work-row-banner-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_BACKGROUND_WORK_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-background-work-row-banner-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_BACKGROUND_WORK_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-background-work-row-banner-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_BACKGROUND_WORK_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-background-work-row-banner-controls";

/// Consumer surface a background-work row / banner projects onto. Reuses the frozen matrix
/// consumer-surface taxonomy so no lane invents a parallel surface set.
pub type M5BackgroundWorkConsumerSurface = M5EfficiencyConsumerSurface;

/// One mandatory rendered part a background-work row or banner must be able to show, so no
/// background-work truth is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BackgroundWorkAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The affected work class / workload lane.
    AffectedWorkClass,
    /// The slowed-versus-paused disposition.
    SlowedVersusPaused,
    /// The protected tasks that remain preserved (what still works).
    WhatStillWorks,
    /// The condition under which the work may resume.
    ResumeCondition,
    /// Whether an override exists and its policy owner.
    OverrideAvailability,
    /// The durable surface that keeps the row / banner reviewable after the user looks away.
    DurableSurface,
}

impl M5BackgroundWorkAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::AffectedWorkClass,
        Self::SlowedVersusPaused,
        Self::WhatStillWorks,
        Self::ResumeCondition,
        Self::OverrideAvailability,
        Self::DurableSurface,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::AffectedWorkClass => "affected_work_class",
            Self::SlowedVersusPaused => "slowed_versus_paused",
            Self::WhatStillWorks => "what_still_works",
            Self::ResumeCondition => "resume_condition",
            Self::OverrideAvailability => "override_availability",
            Self::DurableSurface => "durable_surface",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BackgroundWorkNextAction {
    /// Open the activity center to review paused / slowed work.
    OpenActivityCenter,
    /// Open the background-work surface for fuller per-job detail.
    OpenBackgroundWork,
    /// Open efficiency / policy-aware settings (override, policy owner).
    OpenEfficiencySettings,
    /// Review diagnostics for the unavailable signal.
    ReviewDiagnostics,
    /// Restore power / clear the pressure source.
    RestorePower,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5BackgroundWorkNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenActivityCenter,
        Self::OpenBackgroundWork,
        Self::OpenEfficiencySettings,
        Self::ReviewDiagnostics,
        Self::RestorePower,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenActivityCenter => "open_activity_center",
            Self::OpenBackgroundWork => "open_background_work",
            Self::OpenEfficiencySettings => "open_efficiency_settings",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::RestorePower => "restore_power",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a background-work controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BackgroundWorkExportField {
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
    /// The affected work class named by the row / banner.
    AffectedWorkClass,
    /// The current work state / disposition.
    WorkState,
    /// The preserved protected work (what still works).
    PreservedWork,
    /// The resume condition.
    ResumeCondition,
    /// The override availability and policy owner.
    OverrideAvailability,
    /// The accountable owner role.
    OwnerRole,
}

impl M5BackgroundWorkExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::WorkDispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::AffectedWorkClass,
        Self::WorkState,
        Self::PreservedWork,
        Self::ResumeCondition,
        Self::OverrideAvailability,
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
            Self::AffectedWorkClass => "affected_work_class",
            Self::WorkState => "work_state",
            Self::PreservedWork => "preserved_work",
            Self::ResumeCondition => "resume_condition",
            Self::OverrideAvailability => "override_availability",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a background-work row degraded below a clean, fully-legible state. The degrade-first
/// ladder returns one of these instead of ever letting a toast-only or dishonest row read as a
/// clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BackgroundWorkRowDegradeReason {
    /// No affected work class was named, so which job was reduced cannot be told.
    AffectedWorkClassUnnamed,
    /// The adaptive change became user-visible but the row is toast-only and vanishes after
    /// dismissal (AC1 violation).
    ToastOnlyNotDurable,
    /// An override is presented as available even though policy blocks it (guardrail violation).
    OverridePresentedWhenBlocked,
    /// The condition under which the work may resume is unstated.
    ResumeConditionUnstated,
    /// What still works (preserved protected tasks) is unstated.
    WhatStillWorksUnstated,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5BackgroundWorkRowDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AffectedWorkClassUnnamed,
        Self::ToastOnlyNotDurable,
        Self::OverridePresentedWhenBlocked,
        Self::ResumeConditionUnstated,
        Self::WhatStillWorksUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AffectedWorkClassUnnamed => "affected_work_class_unnamed",
            Self::ToastOnlyNotDurable => "toast_only_not_durable",
            Self::OverridePresentedWhenBlocked => "override_presented_when_blocked",
            Self::ResumeConditionUnstated => "resume_condition_unstated",
            Self::WhatStillWorksUnstated => "what_still_works_unstated",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5BackgroundWorkNextAction {
        match self {
            Self::AffectedWorkClassUnnamed | Self::ToastOnlyNotDurable => {
                M5BackgroundWorkNextAction::OpenActivityCenter
            }
            Self::OverridePresentedWhenBlocked => {
                M5BackgroundWorkNextAction::OpenEfficiencySettings
            }
            Self::ResumeConditionUnstated => M5BackgroundWorkNextAction::OpenBackgroundWork,
            Self::WhatStillWorksUnstated => M5BackgroundWorkNextAction::OpenBackgroundWork,
            Self::ProofStale => M5BackgroundWorkNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EfficiencyDowngradeTrigger {
        match self {
            Self::AffectedWorkClassUnnamed => {
                M5EfficiencyDowngradeTrigger::SlowedVersusPausedAmbiguous
            }
            Self::ToastOnlyNotDurable => M5EfficiencyDowngradeTrigger::PausedWorkToastOnly,
            Self::OverridePresentedWhenBlocked => {
                M5EfficiencyDowngradeTrigger::OverrideAvailabilityUnstated
            }
            Self::ResumeConditionUnstated => M5EfficiencyDowngradeTrigger::ResumeBacklogHidden,
            Self::WhatStillWorksUnstated => M5EfficiencyDowngradeTrigger::WhatStillWorksUnstated,
            Self::ProofStale => M5EfficiencyDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a background-work banner degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BackgroundWorkBannerDegradeReason {
    /// No affected work was named across the aggregate.
    NoAffectedWorkNamed,
    /// Repeated pressure events were emitted as one toast each instead of one durable banner
    /// (AC2 violation).
    DuplicateToastSpam,
    /// Adaptive-efficiency truth was collapsed into generic service-failure copy (AC2 violation).
    GenericServiceFailureCopy,
    /// Paused work is present but is not shown explicitly (hidden behind toast-only messaging).
    PausedWorkNotExplicit,
    /// An override is presented as available even though policy blocks it (guardrail violation).
    OverridePresentedWhenBlocked,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5BackgroundWorkBannerDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoAffectedWorkNamed,
        Self::DuplicateToastSpam,
        Self::GenericServiceFailureCopy,
        Self::PausedWorkNotExplicit,
        Self::OverridePresentedWhenBlocked,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAffectedWorkNamed => "no_affected_work_named",
            Self::DuplicateToastSpam => "duplicate_toast_spam",
            Self::GenericServiceFailureCopy => "generic_service_failure_copy",
            Self::PausedWorkNotExplicit => "paused_work_not_explicit",
            Self::OverridePresentedWhenBlocked => "override_presented_when_blocked",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5BackgroundWorkNextAction {
        match self {
            Self::NoAffectedWorkNamed | Self::PausedWorkNotExplicit => {
                M5BackgroundWorkNextAction::OpenActivityCenter
            }
            Self::DuplicateToastSpam | Self::GenericServiceFailureCopy => {
                M5BackgroundWorkNextAction::OpenBackgroundWork
            }
            Self::OverridePresentedWhenBlocked => {
                M5BackgroundWorkNextAction::OpenEfficiencySettings
            }
            Self::ProofStale => M5BackgroundWorkNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EfficiencyDowngradeTrigger {
        match self {
            Self::NoAffectedWorkNamed => M5EfficiencyDowngradeTrigger::WhatStillWorksUnstated,
            Self::DuplicateToastSpam => M5EfficiencyDowngradeTrigger::PausedWorkToastOnly,
            Self::GenericServiceFailureCopy => {
                M5EfficiencyDowngradeTrigger::GenericLowPowerWordingUsed
            }
            Self::PausedWorkNotExplicit => M5EfficiencyDowngradeTrigger::PausedWorkToastOnly,
            Self::OverridePresentedWhenBlocked => {
                M5EfficiencyDowngradeTrigger::OverrideAvailabilityUnstated
            }
            Self::ProofStale => M5EfficiencyDowngradeTrigger::ProofStale,
        }
    }
}

/// True when an override posture blocks the override outright, so presenting the override as
/// available would be dishonest.
const fn override_is_blocked(posture: OverridePosture) -> bool {
    matches!(
        posture,
        OverridePosture::NotOverridable
            | OverridePosture::PolicyBlocked
            | OverridePosture::AdminControlled
    )
}

/// Input to [`resolve_background_work_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BackgroundWorkRowResolutionInput {
    /// Stable identity of the row instance.
    pub row_id: String,
    /// Affected work class for this job (`None` means the class was left unnamed).
    pub affected_work_class: Option<WorkloadFamily>,
    /// True when the job is paused (not progressing).
    pub paused: bool,
    /// True when the job is slowed (still progressing).
    pub slowed: bool,
    /// Condition under which the work may resume (`None` means unstated).
    pub resume_condition: Option<EfficiencyRecoveryState>,
    /// Override posture behind the adaptation.
    pub override_posture: OverridePosture,
    /// True when the surface presents the override as available to the user.
    pub override_presented_available: bool,
    /// Policy owner accountable for the adaptation / override.
    pub policy_owner: M5EfficiencyPolicyOwner,
    /// Protected tasks that remain preserved (what still works).
    pub preserved_protected_tasks: Vec<String>,
    /// True when the adaptive change already became user-visible on this surface.
    pub adaptive_change_user_visible: bool,
    /// True when the row is anchored in a durable shell / activity surface (not toast-only).
    pub durable_surface_present: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe background-work row projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBackgroundWorkRow {
    /// Stable identity of the row instance.
    pub row_id: String,
    /// Affected work-class token, if named.
    pub affected_work_class: Option<String>,
    /// Single controlled work disposition carried by the row.
    pub work_disposition: M5EfficiencyWorkDisposition,
    /// Preserved protected tasks (what still works).
    pub preserved_protected_tasks: Vec<String>,
    /// Resume-condition token, if stated.
    pub resume_condition: Option<String>,
    /// Override posture token.
    pub override_posture: String,
    /// Policy owner token.
    pub policy_owner: String,
    /// Whether an override is honestly available to the user.
    pub override_available: bool,
    /// Whether the row is carried in a durable surface.
    pub durable_surface: bool,
    /// AC1: whether this row stays reviewable after the user looks away.
    pub reviewable_after_looking_away: bool,
    /// Degrade reason, if the row could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5BackgroundWorkRowDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5BackgroundWorkNextAction,
    /// Guardrail (MUST be `false` on a clean row): an override was presented as available while
    /// policy blocks it.
    pub presented_override_when_blocked: bool,
}

impl M5ResolvedBackgroundWorkRow {
    /// Whether this row reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_background_work_banner`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BackgroundWorkBannerResolutionInput {
    /// Stable identity of the banner instance.
    pub banner_id: String,
    /// Workload lanes that slowed (still progressing) across the aggregate.
    pub slowed_workloads: Vec<WorkloadFamily>,
    /// Workload lanes that paused (not progressing) across the aggregate.
    pub paused_workloads: Vec<WorkloadFamily>,
    /// Protected tasks that remain preserved (what still works).
    pub preserved_protected_tasks: Vec<String>,
    /// How many pressure events the banner coalesced.
    pub pressure_event_count: u32,
    /// True when repeated pressure events were coalesced into one durable banner.
    pub coalesced_into_single_banner: bool,
    /// True when the banner shows the paused work explicitly rather than hiding it.
    pub shows_paused_work_explicitly: bool,
    /// True when the banner fell back to generic service-failure copy.
    pub uses_generic_service_failure_copy: bool,
    /// Condition under which the work may resume (`None` means unstated).
    pub resume_condition: Option<EfficiencyRecoveryState>,
    /// Override posture behind the adaptation.
    pub override_posture: OverridePosture,
    /// True when the surface presents the override as available to the user.
    pub override_presented_available: bool,
    /// Policy owner accountable for the adaptation / override.
    pub policy_owner: M5EfficiencyPolicyOwner,
    /// True when the banner is anchored in a durable shell / activity surface.
    pub durable_surface_present: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe background-work banner projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBackgroundWorkBanner {
    /// Stable identity of the banner instance.
    pub banner_id: String,
    /// Slowed workload tokens.
    pub slowed_workloads: Vec<String>,
    /// Paused workload tokens.
    pub paused_workloads: Vec<String>,
    /// Preserved protected tasks (what still works).
    pub preserved_protected_tasks: Vec<String>,
    /// Whether any affected work was named across the aggregate.
    pub affected_work_named: bool,
    /// Controlled work dispositions carried by the banner.
    pub work_dispositions: Vec<M5EfficiencyWorkDisposition>,
    /// Resume-condition token, if stated.
    pub resume_condition: Option<String>,
    /// Override posture token.
    pub override_posture: String,
    /// Policy owner token.
    pub policy_owner: String,
    /// Whether an override is honestly available to the user.
    pub override_available: bool,
    /// How many pressure events the banner coalesced.
    pub pressure_event_count: u32,
    /// Whether repeated pressure events were coalesced into one durable banner.
    pub coalesced_into_single_banner: bool,
    /// Whether the banner shows paused work explicitly.
    pub shows_paused_work_explicitly: bool,
    /// Whether the banner is carried in a durable surface.
    pub durable_surface: bool,
    /// Degrade reason, if the banner could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5BackgroundWorkBannerDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5BackgroundWorkNextAction,
    /// Guardrail (MUST be `false` on a clean banner): repeated pressure emitted one toast each.
    pub duplicate_toast_spam: bool,
    /// Guardrail (MUST be `false` on a clean banner): adaptive truth collapsed into generic copy.
    pub generic_service_failure_copy: bool,
    /// Guardrail (MUST be `false` on a clean banner): an override presented as available while
    /// policy blocks it.
    pub presented_override_when_blocked: bool,
}

impl M5ResolvedBackgroundWorkBanner {
    /// Whether this banner reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5BackgroundWorkResolutionError {
    /// The background-work row id was empty.
    EmptyRowId,
    /// The background-work banner id was empty.
    EmptyBannerId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5BackgroundWorkResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyRowId => "empty_row_id",
            Self::EmptyBannerId => "empty_banner_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5BackgroundWorkResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 background-work resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5BackgroundWorkResolutionError {}

/// Resolves a background-work row, proving AC1: the row names its affected work class, current
/// disposition, what still works, resume condition, and override existence, and never reads as a
/// clean row when it is only carried in transient toast-only messaging.
pub fn resolve_background_work_row(
    input: M5BackgroundWorkRowResolutionInput,
) -> Result<M5ResolvedBackgroundWorkRow, M5BackgroundWorkResolutionError> {
    if input.row_id.trim().is_empty() {
        return Err(M5BackgroundWorkResolutionError::EmptyRowId);
    }
    if string_is_forbidden(&input.row_id)
        || input
            .preserved_protected_tasks
            .iter()
            .any(|t| string_is_forbidden(t))
    {
        return Err(M5BackgroundWorkResolutionError::ForbiddenMaterial);
    }

    let presented_override_when_blocked =
        input.override_presented_available && override_is_blocked(input.override_posture);
    // A resume condition is only required when the job is actually deferred or slowed.
    let resume_required = input.paused || input.slowed;
    let toast_only = input.adaptive_change_user_visible && !input.durable_surface_present;

    let degrade_reason = if input.affected_work_class.is_none() {
        Some(M5BackgroundWorkRowDegradeReason::AffectedWorkClassUnnamed)
    } else if toast_only {
        Some(M5BackgroundWorkRowDegradeReason::ToastOnlyNotDurable)
    } else if presented_override_when_blocked {
        Some(M5BackgroundWorkRowDegradeReason::OverridePresentedWhenBlocked)
    } else if resume_required && input.resume_condition.is_none() {
        Some(M5BackgroundWorkRowDegradeReason::ResumeConditionUnstated)
    } else if input.preserved_protected_tasks.is_empty() {
        Some(M5BackgroundWorkRowDegradeReason::WhatStillWorksUnstated)
    } else if !input.proof_fresh {
        Some(M5BackgroundWorkRowDegradeReason::ProofStale)
    } else {
        None
    };

    let work_disposition = if input.affected_work_class.is_none() {
        M5EfficiencyWorkDisposition::NotEvaluated
    } else {
        row_disposition(input.paused, input.slowed, input.resume_condition)
    };

    let override_available =
        input.override_presented_available && !override_is_blocked(input.override_posture);
    let reviewable_after_looking_away = degrade_reason.is_none() && input.durable_surface_present;

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None if work_disposition.is_running_full() => M5BackgroundWorkNextAction::NoActionNeeded,
        None => M5BackgroundWorkNextAction::OpenBackgroundWork,
    };

    Ok(M5ResolvedBackgroundWorkRow {
        row_id: input.row_id,
        affected_work_class: input.affected_work_class.map(|w| w.as_str().to_owned()),
        work_disposition,
        preserved_protected_tasks: input.preserved_protected_tasks,
        resume_condition: input.resume_condition.map(|r| r.as_str().to_owned()),
        override_posture: input.override_posture.as_str().to_owned(),
        policy_owner: input.policy_owner.as_str().to_owned(),
        override_available,
        durable_surface: input.durable_surface_present,
        reviewable_after_looking_away,
        degrade_reason,
        next_action,
        presented_override_when_blocked,
    })
}

/// Resolves a background-work banner, proving AC2: broad or repeated pressure events coalesce
/// into one durable banner and never degrade into duplicate toast spam or generic service-failure
/// copy.
pub fn resolve_background_work_banner(
    input: M5BackgroundWorkBannerResolutionInput,
) -> Result<M5ResolvedBackgroundWorkBanner, M5BackgroundWorkResolutionError> {
    if input.banner_id.trim().is_empty() {
        return Err(M5BackgroundWorkResolutionError::EmptyBannerId);
    }
    if string_is_forbidden(&input.banner_id)
        || input
            .preserved_protected_tasks
            .iter()
            .any(|t| string_is_forbidden(t))
    {
        return Err(M5BackgroundWorkResolutionError::ForbiddenMaterial);
    }

    let affected_work_named =
        !input.slowed_workloads.is_empty() || !input.paused_workloads.is_empty();
    let has_paused = !input.paused_workloads.is_empty();
    let duplicate_toast_spam =
        input.pressure_event_count > 1 && !input.coalesced_into_single_banner;
    let presented_override_when_blocked =
        input.override_presented_available && override_is_blocked(input.override_posture);

    let degrade_reason = if !affected_work_named {
        Some(M5BackgroundWorkBannerDegradeReason::NoAffectedWorkNamed)
    } else if duplicate_toast_spam {
        Some(M5BackgroundWorkBannerDegradeReason::DuplicateToastSpam)
    } else if input.uses_generic_service_failure_copy {
        Some(M5BackgroundWorkBannerDegradeReason::GenericServiceFailureCopy)
    } else if has_paused && !input.shows_paused_work_explicitly {
        Some(M5BackgroundWorkBannerDegradeReason::PausedWorkNotExplicit)
    } else if presented_override_when_blocked {
        Some(M5BackgroundWorkBannerDegradeReason::OverridePresentedWhenBlocked)
    } else if !input.proof_fresh {
        Some(M5BackgroundWorkBannerDegradeReason::ProofStale)
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
    if input.resume_condition == Some(EfficiencyRecoveryState::StagedResume) {
        work_dispositions.push(M5EfficiencyWorkDisposition::Resuming);
    }
    if work_dispositions.is_empty() {
        work_dispositions.push(M5EfficiencyWorkDisposition::NotEvaluated);
    }

    let override_available =
        input.override_presented_available && !override_is_blocked(input.override_posture);

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5BackgroundWorkNextAction::OpenActivityCenter,
    };

    Ok(M5ResolvedBackgroundWorkBanner {
        banner_id: input.banner_id,
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
        affected_work_named,
        work_dispositions,
        resume_condition: input.resume_condition.map(|r| r.as_str().to_owned()),
        override_posture: input.override_posture.as_str().to_owned(),
        policy_owner: input.policy_owner.as_str().to_owned(),
        override_available,
        pressure_event_count: input.pressure_event_count,
        coalesced_into_single_banner: input.coalesced_into_single_banner,
        shows_paused_work_explicitly: input.shows_paused_work_explicitly,
        durable_surface: input.durable_surface_present,
        degrade_reason,
        next_action,
        duplicate_toast_spam,
        generic_service_failure_copy: input.uses_generic_service_failure_copy,
        presented_override_when_blocked,
    })
}

/// Maps a job's slowed / paused / resuming signal to the single controlled work disposition.
const fn row_disposition(
    paused: bool,
    slowed: bool,
    resume_condition: Option<EfficiencyRecoveryState>,
) -> M5EfficiencyWorkDisposition {
    if paused {
        M5EfficiencyWorkDisposition::Paused
    } else if slowed {
        M5EfficiencyWorkDisposition::Slowed
    } else if matches!(
        resume_condition,
        Some(EfficiencyRecoveryState::StagedResume)
    ) {
        M5EfficiencyWorkDisposition::Resuming
    } else {
        M5EfficiencyWorkDisposition::RunningFull
    }
}

/// One controls row: one consumer surface bound to the resolved background-work row and banner
/// examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BackgroundWorkControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5BackgroundWorkConsumerSurface,
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
    pub anatomy_parts: Vec<M5BackgroundWorkAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5BackgroundWorkExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5EfficiencyDowngradeTrigger>,
    /// Resolved background-work row examples.
    pub background_work_row_examples: Vec<M5ResolvedBackgroundWorkRow>,
    /// Resolved background-work banner examples.
    pub background_work_banner_examples: Vec<M5ResolvedBackgroundWorkBanner>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never collapse pressure into generic service-failure copy.
    pub collapses_pressure_into_generic_service_failure: bool,
    /// Hard invariant: never hide paused work behind toast-only messaging.
    pub hides_paused_work_behind_toast_only: bool,
    /// Hard invariant: never present an override as available when policy blocks it.
    pub presents_override_available_when_policy_blocks: bool,
    /// Hard invariant: never drop background work after a toast is dismissed.
    pub drops_background_work_after_toast_dismissal: bool,
}

impl M5BackgroundWorkControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5BackgroundWorkAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5BackgroundWorkAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5BackgroundWorkExportField> =
            self.export_fields.iter().copied().collect();
        M5BackgroundWorkExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.collapses_pressure_into_generic_service_failure
            && !self.hides_paused_work_behind_toast_only
            && !self.presents_override_available_when_policy_blocks
            && !self.drops_background_work_after_toast_dismissal
    }

    /// True when every resolved example on this row is honest: no clean row is toast-only or
    /// presents a blocked override as available, and no clean banner spams toasts, uses generic
    /// copy, or presents a blocked override as available.
    fn examples_are_honest(&self) -> bool {
        self.background_work_row_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (!ex.reviewable_after_looking_away || ex.presented_override_when_blocked))
        }) && self.background_work_banner_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.duplicate_toast_spam
                    || ex.generic_service_failure_copy
                    || ex.presented_override_when_blocked))
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BackgroundWorkVocabularySet {
    /// Work-disposition tokens (bound from the frozen matrix).
    pub work_dispositions: Vec<String>,
    /// Affected-workload tokens (bound from the efficiency object model).
    pub affected_workloads: Vec<String>,
    /// Override-posture tokens (bound from the efficiency object model).
    pub override_postures: Vec<String>,
    /// Policy-owner tokens (bound from the frozen matrix).
    pub policy_owners: Vec<String>,
    /// Recovery-state tokens (bound from the efficiency object model).
    pub recovery_states: Vec<String>,
    /// Background-work row degrade-reason tokens.
    pub row_degrade_reasons: Vec<String>,
    /// Background-work banner degrade-reason tokens.
    pub banner_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5BackgroundWorkVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            work_dispositions: tokens(&M5EfficiencyWorkDisposition::ALL, |v| v.as_str()),
            affected_workloads: tokens(&AFFECTED_WORKLOADS, |v| v.as_str()),
            override_postures: tokens(&OverridePosture::ALL, |v| v.as_str()),
            policy_owners: tokens(&M5EfficiencyPolicyOwner::ALL, |v| v.as_str()),
            recovery_states: tokens(&EfficiencyRecoveryState::ALL, |v| v.as_str()),
            row_degrade_reasons: tokens(&M5BackgroundWorkRowDegradeReason::ALL, |v| v.as_str()),
            banner_degrade_reasons: tokens(&M5BackgroundWorkBannerDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5BackgroundWorkAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5BackgroundWorkNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5BackgroundWorkExportField::ALL, |v| v.as_str()),
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
pub struct M5BackgroundWorkGovernanceReview {
    /// The background-work row always names its affected work class.
    pub row_names_affected_work_class: bool,
    /// The background-work row always makes slowed-versus-paused work explicit.
    pub row_shows_slowed_versus_paused: bool,
    /// Every row and banner always names what still works.
    pub always_names_what_still_works: bool,
    /// Every row and banner always states a resume condition when work is deferred.
    pub resume_condition_stated_when_deferred: bool,
    /// No override is ever presented as available when policy blocks it.
    pub no_override_presented_when_policy_blocks: bool,
    /// The banner always shows paused work explicitly rather than toast-only.
    pub banner_shows_paused_work_explicitly: bool,
    /// The banner always coalesces repeated pressure rather than spamming toasts.
    pub banner_coalesces_repeated_pressure: bool,
    /// No surface drops background-work truth after a toast is dismissed.
    pub no_background_work_dropped_after_toast: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BackgroundWorkConsumerProjection {
    /// Shell surfaces consume the shared background-work rows.
    pub shell_surfaces_consume_background_rows: bool,
    /// Activity surfaces consume the shared background-work banner.
    pub activity_surfaces_consume_background_banner: bool,
    /// Diagnostics surfaces consume the shared affected-work vocabulary.
    pub diagnostics_surfaces_consume_work_vocabulary: bool,
    /// Support / export reads a single canonical background-work source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BackgroundWorkProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BackgroundWorkReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting efficiency audit for the lane.
    pub efficiency_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BackgroundWorkControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BackgroundWorkControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5BackgroundWorkControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BackgroundWorkVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BackgroundWorkGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BackgroundWorkConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BackgroundWorkProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BackgroundWorkReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 background-work row / banner controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BackgroundWorkControlsPacket {
    /// Record kind; must equal [`M5_BACKGROUND_WORK_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BACKGROUND_WORK_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5BackgroundWorkControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BackgroundWorkVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BackgroundWorkGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BackgroundWorkConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BackgroundWorkProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BackgroundWorkReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BackgroundWorkControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5BackgroundWorkControlsPacketInput) -> Self {
        Self {
            record_kind: M5_BACKGROUND_WORK_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_BACKGROUND_WORK_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5BackgroundWorkControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BACKGROUND_WORK_CONTROLS_RECORD_KIND {
            violations.push(M5BackgroundWorkControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BACKGROUND_WORK_CONTROLS_SCHEMA_VERSION {
            violations.push(M5BackgroundWorkControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BackgroundWorkControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5BackgroundWorkControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 background-work controls packet serializes"),
        ) {
            violations.push(M5BackgroundWorkControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 background-work controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,row_examples,banner_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .background_work_row_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.background_work_banner_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.background_work_row_examples.len(),
                row.background_work_banner_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Background-Work-Row and Background-Work-Banner Controls\n\n");
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
                "  - Row examples: {} / banner examples: {}\n",
                row.background_work_row_examples.len(),
                row.background_work_banner_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5BackgroundWorkControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BackgroundWorkControlsViolation>),
}

impl fmt::Display for M5BackgroundWorkControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 background-work controls export parse failed: {error}"
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
                    "m5 background-work controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BackgroundWorkControlsArtifactError {}

/// Validation failures emitted by [`M5BackgroundWorkControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BackgroundWorkControlsViolation {
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
    /// A controls row carries a dishonest clean example (toast-only, generic, or blocked
    /// override presented available).
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
    /// AC1 is not proven: no clean row stays reviewable after the user looks away, or none shows
    /// a toast-only row degrading.
    Ac1NotProven,
    /// AC2 is not proven: no banner shows repeated pressure degrading to toast spam or generic
    /// service-failure copy rather than reading clean.
    Ac2NotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BackgroundWorkControlsViolation {
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
pub fn current_stable_m5_background_work_controls_export(
) -> Result<M5BackgroundWorkControlsPacket, M5BackgroundWorkControlsArtifactError> {
    let packet: M5BackgroundWorkControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-background-work-row-banner-controls-proof/support_export.json"
    )))
    .map_err(M5BackgroundWorkControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BackgroundWorkControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5BackgroundWorkControlsPacket,
    violations: &mut Vec<M5BackgroundWorkControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BACKGROUND_WORK_CONTROLS_SCHEMA_REF,
        M5_BACKGROUND_WORK_CONTROLS_DOC_REF,
        M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
        M5_EFFICIENCY_COMPONENT_DOC_REF,
        M5_BACKGROUND_WORK_ROW_SCHEMA_REF,
        M5_BACKGROUND_WORK_BANNER_SCHEMA_REF,
        M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BackgroundWorkControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5BackgroundWorkControlsPacket,
    violations: &mut Vec<M5BackgroundWorkControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5BackgroundWorkControlsViolation::NoControlsRows);
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
            violations.push(M5BackgroundWorkControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5BackgroundWorkControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5BackgroundWorkControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_BACKGROUND_WORK_ROW_SCHEMA_REF)
            || !refs.contains(M5_BACKGROUND_WORK_BANNER_SCHEMA_REF)
        {
            violations.push(M5BackgroundWorkControlsViolation::ComponentSchemaRefMissing);
        }
        if row.background_work_row_examples.is_empty()
            || row.background_work_banner_examples.is_empty()
        {
            violations.push(M5BackgroundWorkControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5BackgroundWorkControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5BackgroundWorkControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5BackgroundWorkControlsPacket,
    violations: &mut Vec<M5BackgroundWorkControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.row_names_affected_work_class,
        review.row_shows_slowed_versus_paused,
        review.always_names_what_still_works,
        review.resume_condition_stated_when_deferred,
        review.no_override_presented_when_policy_blocks,
        review.banner_shows_paused_work_explicitly,
        review.banner_coalesces_repeated_pressure,
        review.no_background_work_dropped_after_toast,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5BackgroundWorkControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BackgroundWorkControlsPacket,
    violations: &mut Vec<M5BackgroundWorkControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_surfaces_consume_background_rows,
        projection.activity_surfaces_consume_background_banner,
        projection.diagnostics_surfaces_consume_work_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5BackgroundWorkControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5BackgroundWorkControlsPacket,
    violations: &mut Vec<M5BackgroundWorkControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5BackgroundWorkControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5BackgroundWorkControlsPacket,
    violations: &mut Vec<M5BackgroundWorkControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.efficiency_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BackgroundWorkControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5BackgroundWorkControlsPacket,
    violations: &mut Vec<M5BackgroundWorkControlsViolation>,
) {
    let row_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.background_work_row_examples.iter())
    };
    let banner_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.background_work_banner_examples.iter())
    };

    // AC1: at least one clean row stays reviewable after the user looks away with a named work
    // class and a stated resume condition, and at least one row degrades to ToastOnlyNotDurable;
    // no clean row is toast-only.
    let clean_reviewable = row_examples().any(|ex| {
        ex.is_clean()
            && ex.reviewable_after_looking_away
            && ex.affected_work_class.is_some()
            && ex.resume_condition.is_some()
    });
    let toast_degrades = row_examples()
        .any(|ex| ex.degrade_reason == Some(M5BackgroundWorkRowDegradeReason::ToastOnlyNotDurable));
    let no_clean_toast_only =
        row_examples().all(|ex| !(ex.is_clean() && !ex.reviewable_after_looking_away));
    if !(clean_reviewable && toast_degrades && no_clean_toast_only) {
        violations.push(M5BackgroundWorkControlsViolation::Ac1NotProven);
    }

    // AC2: at least one banner degrades to DuplicateToastSpam and at least one to
    // GenericServiceFailureCopy, and no clean banner spams toasts or uses generic copy.
    let spam_degrades = banner_examples().any(|ex| {
        ex.degrade_reason == Some(M5BackgroundWorkBannerDegradeReason::DuplicateToastSpam)
            && ex.duplicate_toast_spam
    });
    let generic_degrades = banner_examples().any(|ex| {
        ex.degrade_reason == Some(M5BackgroundWorkBannerDegradeReason::GenericServiceFailureCopy)
            && ex.generic_service_failure_copy
    });
    let no_clean_spam = banner_examples()
        .all(|ex| !(ex.is_clean() && (ex.duplicate_toast_spam || ex.generic_service_failure_copy)));
    if !(spam_degrades && generic_degrades && no_clean_spam) {
        violations.push(M5BackgroundWorkControlsViolation::Ac2NotProven);
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
