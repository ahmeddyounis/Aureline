//! Frozen M5 power-state-indicator, throttled-subsystem-row, background-work-row,
//! background-work-banner, per-workspace-override-sheet, override-policy-note-row,
//! resume-summary-card, and stale-result-continuity-note component matrix.
//!
//! This module locks Aureline's reusable adaptive-efficiency UI components into one
//! export-safe packet. Every efficiency-surface subcomponent M5 claims that still drifts too
//! easily between the shell status bar, the activity center, diagnostics, Help/About, the
//! support export, and policy-aware settings — the power-state indicator, the throttled-
//! subsystem row, the background-work row, the background-work banner, the per-workspace
//! override sheet, the override-policy note row, the resume-summary card, and the stale-result
//! continuity note — is named once here and constrained by the same source-of-change, active
//! efficiency state, slowed-versus-paused work disposition, override availability, policy
//! owner, resumed-work backlog, and stale-result continuity vocabulary regardless of the
//! surface family that renders it.
//!
//! The matrix does not re-architect the efficiency-state governance, battery/thermal shedding,
//! background-work queue, or shell-primitives progress rows that already own those records — it
//! is the shared adaptive-efficiency component contract layered on top of them. It binds
//! directly to the frozen [M5 efficiency-state governance matrix][gov] so no later consumer can
//! fork its own low-power or thermal wording: the power-state indicator reuses the
//! [`EfficiencyPressureSource`] source-of-change and [`EfficiencyState`] active-state
//! vocabularies, the throttled-subsystem/background-work components reuse the
//! [`WorkloadFamily`] workload vocabulary, the override sheet and policy note reuse the
//! [`OverridePosture`] override vocabulary, and the resume-summary card reuses the
//! [`EfficiencyRecoveryState`] recovery vocabulary.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5EfficiencyComponentVocabularySet`] rather than minted per surface. The single controlled
//! work-disposition vocabulary consumers bind to — running-full, slowed, paused, policy-blocked,
//! override-available, override-blocked, resuming, stale-result-shown, and not-evaluated —
//! keeps slowed-versus-paused work honest, keeps overrides from reading as available when policy
//! blocks them, and keeps stale-result context from being cleared merely because background
//! work resumed. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [gov]: crate::efficiency::governance

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_efficiency_component_matrix,
    seeded_m5_efficiency_component_matrix_override_sheet_beta_narrowed,
    seeded_m5_efficiency_component_matrix_stale_result_note_preview_narrowed,
    M5_EFFICIENCY_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::efficiency::governance::{
    EfficiencyRecoveryState, OverridePosture, M5_EFFICIENCY_GOVERNANCE_MATRIX_REF,
    M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
};
use crate::efficiency::{EfficiencyPressureSource, EfficiencyState, WorkloadFamily};

/// Stable record-kind tag carried by [`M5EfficiencyComponentMatrixPacket`].
pub const M5_EFFICIENCY_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix";

/// Schema version for M5 efficiency component-matrix records.
pub const M5_EFFICIENCY_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined efficiency component-matrix boundary schema.
pub const M5_EFFICIENCY_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-efficiency-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EFFICIENCY_COMPONENT_DOC_REF: &str = "docs/help/m5_efficiency_components_contract.md";

/// Repo-relative path of the power-state-indicator canonical component schema.
pub const M5_POWER_STATE_INDICATOR_SCHEMA_REF: &str =
    "schemas/ui/m5-power-state-indicator.schema.json";

/// Repo-relative path of the throttled-subsystem-row canonical component schema.
pub const M5_THROTTLED_SUBSYSTEM_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-throttled-subsystem-row.schema.json";

/// Repo-relative path of the background-work-row canonical component schema.
pub const M5_BACKGROUND_WORK_ROW_SCHEMA_REF: &str = "schemas/ui/m5-background-work-row.schema.json";

/// Repo-relative path of the background-work-banner canonical component schema.
pub const M5_BACKGROUND_WORK_BANNER_SCHEMA_REF: &str =
    "schemas/ui/m5-background-work-banner.schema.json";

/// Repo-relative path of the per-workspace-override-sheet canonical component schema.
pub const M5_EFFICIENCY_OVERRIDE_SHEET_SCHEMA_REF: &str =
    "schemas/ui/m5-efficiency-override-sheet.schema.json";

/// Repo-relative path of the override-policy-note-row canonical component schema.
pub const M5_OVERRIDE_POLICY_NOTE_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-override-policy-note-row.schema.json";

/// Repo-relative path of the resume-summary-card canonical component schema.
pub const M5_RESUME_SUMMARY_CARD_SCHEMA_REF: &str = "schemas/ui/m5-resume-summary-card.schema.json";

/// Repo-relative path of the stale-result-continuity-note canonical component schema.
pub const M5_STALE_RESULT_CONTINUITY_NOTE_SCHEMA_REF: &str =
    "schemas/ui/m5-stale-result-continuity-note.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_EFFICIENCY_COMPONENT_FIXTURE_DIR: &str = "fixtures/ui/m5-efficiency-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EFFICIENCY_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-efficiency-components-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_EFFICIENCY_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-efficiency-components-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_EFFICIENCY_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-efficiency-component-matrix.md";

/// One of the eight governed adaptive-efficiency component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyComponentFamily {
    /// A power-state indicator naming the source of change and the active efficiency state.
    PowerStateIndicator,
    /// A throttled-subsystem row naming which subsystem's work is slowed or paused.
    ThrottledSubsystemRow,
    /// A background-work row naming one job's slowed-versus-paused disposition.
    BackgroundWorkRow,
    /// A background-work banner naming aggregate paused/slowed work explicitly.
    BackgroundWorkBanner,
    /// A per-workspace override sheet naming override availability and its policy owner.
    PerWorkspaceOverrideSheet,
    /// An override-policy note row naming the policy owner behind an adaptation.
    OverridePolicyNoteRow,
    /// A resume-summary card naming the resumed-work backlog after pressure cleared.
    ResumeSummaryCard,
    /// A stale-result continuity note naming stale-result continuity after resume.
    StaleResultContinuityNote,
}

impl M5EfficiencyComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::PowerStateIndicator,
        Self::ThrottledSubsystemRow,
        Self::BackgroundWorkRow,
        Self::BackgroundWorkBanner,
        Self::PerWorkspaceOverrideSheet,
        Self::OverridePolicyNoteRow,
        Self::ResumeSummaryCard,
        Self::StaleResultContinuityNote,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PowerStateIndicator => "power_state_indicator",
            Self::ThrottledSubsystemRow => "throttled_subsystem_row",
            Self::BackgroundWorkRow => "background_work_row",
            Self::BackgroundWorkBanner => "background_work_banner",
            Self::PerWorkspaceOverrideSheet => "per_workspace_override_sheet",
            Self::OverridePolicyNoteRow => "override_policy_note_row",
            Self::ResumeSummaryCard => "resume_summary_card",
            Self::StaleResultContinuityNote => "stale_result_continuity_note",
        }
    }

    /// The canonical per-component schema ref a downstream row points at instead of restating
    /// this component's adaptive-efficiency truth by hand.
    pub const fn canonical_component_schema_ref(self) -> &'static str {
        match self {
            Self::PowerStateIndicator => M5_POWER_STATE_INDICATOR_SCHEMA_REF,
            Self::ThrottledSubsystemRow => M5_THROTTLED_SUBSYSTEM_ROW_SCHEMA_REF,
            Self::BackgroundWorkRow => M5_BACKGROUND_WORK_ROW_SCHEMA_REF,
            Self::BackgroundWorkBanner => M5_BACKGROUND_WORK_BANNER_SCHEMA_REF,
            Self::PerWorkspaceOverrideSheet => M5_EFFICIENCY_OVERRIDE_SHEET_SCHEMA_REF,
            Self::OverridePolicyNoteRow => M5_OVERRIDE_POLICY_NOTE_ROW_SCHEMA_REF,
            Self::ResumeSummaryCard => M5_RESUME_SUMMARY_CARD_SCHEMA_REF,
            Self::StaleResultContinuityNote => M5_STALE_RESULT_CONTINUITY_NOTE_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled source-of-change pressure signal.
    pub const fn declares_pressure_source(self) -> bool {
        matches!(self, Self::PowerStateIndicator)
    }

    /// `true` when this family must name a controlled active efficiency state.
    pub const fn declares_efficiency_state(self) -> bool {
        matches!(self, Self::PowerStateIndicator)
    }

    /// `true` when this family must name a controlled affected workload family.
    pub const fn declares_affected_workload(self) -> bool {
        matches!(
            self,
            Self::ThrottledSubsystemRow | Self::BackgroundWorkRow | Self::BackgroundWorkBanner
        )
    }

    /// `true` when this family must name a controlled override posture.
    pub const fn declares_override_posture(self) -> bool {
        matches!(
            self,
            Self::PerWorkspaceOverrideSheet | Self::OverridePolicyNoteRow
        )
    }

    /// `true` when this family must name a controlled policy owner.
    pub const fn declares_policy_owner(self) -> bool {
        matches!(
            self,
            Self::PerWorkspaceOverrideSheet | Self::OverridePolicyNoteRow
        )
    }

    /// `true` when this family must name a controlled recovery state.
    pub const fn declares_recovery_state(self) -> bool {
        matches!(self, Self::ResumeSummaryCard)
    }

    /// `true` when this family must name a controlled stale-result continuity state.
    pub const fn declares_stale_result_state(self) -> bool {
        matches!(self, Self::StaleResultContinuityNote)
    }
}

/// The single controlled work-disposition vocabulary every adaptive-efficiency consumer binds
/// to. These are the exact acceptance-criteria tokens that keep slowed-versus-paused work,
/// override availability, resume, and stale-result continuity honest. No efficiency surface
/// invents a parallel word for any of these dispositions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyWorkDisposition {
    /// The work is running at its full published budget.
    RunningFull,
    /// The work is slowed / throttled but still progressing.
    Slowed,
    /// The work is paused / deferred and not progressing.
    Paused,
    /// The work is blocked by an admin or local policy cap.
    PolicyBlocked,
    /// An override to restore the work is available to the user.
    OverrideAvailable,
    /// An override is blocked by policy and must not read as available.
    OverrideBlocked,
    /// The work is resuming from its deferred backlog after pressure cleared.
    Resuming,
    /// A stale result is deliberately kept visible pending refresh.
    StaleResultShown,
    /// The disposition cannot currently be evaluated.
    NotEvaluated,
}

impl M5EfficiencyWorkDisposition {
    /// Every disposition token, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RunningFull,
        Self::Slowed,
        Self::Paused,
        Self::PolicyBlocked,
        Self::OverrideAvailable,
        Self::OverrideBlocked,
        Self::Resuming,
        Self::StaleResultShown,
        Self::NotEvaluated,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunningFull => "running_full",
            Self::Slowed => "slowed",
            Self::Paused => "paused",
            Self::PolicyBlocked => "policy_blocked",
            Self::OverrideAvailable => "override_available",
            Self::OverrideBlocked => "override_blocked",
            Self::Resuming => "resuming",
            Self::StaleResultShown => "stale_result_shown",
            Self::NotEvaluated => "not_evaluated",
        }
    }

    /// Whether this disposition is the clean, unconstrained full-speed state.
    pub const fn is_running_full(self) -> bool {
        matches!(self, Self::RunningFull)
    }
}

/// Controlled policy-owner class — who owns the policy behind an adaptation or override, so a
/// policy owner is never left implicit on an override sheet or policy note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyPolicyOwner {
    /// The user controls the adaptation directly.
    UserControlled,
    /// A local (workspace or device) policy owns the adaptation.
    LocalPolicy,
    /// An admin policy owns the adaptation.
    AdminPolicy,
    /// A managed provider policy owns the adaptation.
    ProviderPolicy,
    /// No policy owner could be resolved.
    NoOwnerResolved,
}

impl M5EfficiencyPolicyOwner {
    /// Every policy owner, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::UserControlled,
        Self::LocalPolicy,
        Self::AdminPolicy,
        Self::ProviderPolicy,
        Self::NoOwnerResolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserControlled => "user_controlled",
            Self::LocalPolicy => "local_policy",
            Self::AdminPolicy => "admin_policy",
            Self::ProviderPolicy => "provider_policy",
            Self::NoOwnerResolved => "no_owner_resolved",
        }
    }
}

/// Controlled stale-result continuity state — whether a result is fresh, retained-but-stale,
/// refreshing, or superseded, so stale-result context is never cleared merely because
/// background work resumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyStaleResultState {
    /// The result is fresh and current.
    FreshResult,
    /// A stale result is retained and labelled as stale.
    StaleResultRetained,
    /// A stale result is retained while a refresh is in progress.
    StaleResultRefreshing,
    /// A stale result has been superseded by a fresh one.
    StaleResultSuperseded,
    /// Continuity of the result cannot currently be determined.
    ContinuityUnknown,
}

impl M5EfficiencyStaleResultState {
    /// Every stale-result state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FreshResult,
        Self::StaleResultRetained,
        Self::StaleResultRefreshing,
        Self::StaleResultSuperseded,
        Self::ContinuityUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshResult => "fresh_result",
            Self::StaleResultRetained => "stale_result_retained",
            Self::StaleResultRefreshing => "stale_result_refreshing",
            Self::StaleResultSuperseded => "stale_result_superseded",
            Self::ContinuityUnknown => "continuity_unknown",
        }
    }
}

/// Claimed M5 surface family that renders / consumes an efficiency component. No component may
/// invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencySurfaceFamily {
    /// The shell status bar / mode strip.
    ShellStatusBar,
    /// The activity center.
    ActivityCenter,
    /// The diagnostics surface.
    Diagnostics,
    /// The Help/About surface.
    HelpAbout,
    /// The support export.
    SupportExport,
    /// The policy-aware settings surface.
    PolicyAwareSettings,
}

impl M5EfficiencySurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ShellStatusBar,
        Self::ActivityCenter,
        Self::Diagnostics,
        Self::HelpAbout,
        Self::SupportExport,
        Self::PolicyAwareSettings,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellStatusBar => "shell_status_bar",
            Self::ActivityCenter => "activity_center",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
            Self::PolicyAwareSettings => "policy_aware_settings",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's source,
/// state, disposition, override, or continuity truth never silently narrows or widens between
/// deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyDeploymentLine {
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

impl M5EfficiencyDeploymentLine {
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

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyConsumerSurface {
    /// The shell status UI.
    ShellStatusUi,
    /// The activity-center UI.
    ActivityCenterUi,
    /// The background-work UI.
    BackgroundWorkUi,
    /// The override / policy-aware settings UI.
    OverrideSettingsUi,
    /// The diagnostics UI.
    DiagnosticsUi,
    /// The support export.
    SupportExport,
    /// The Help/About UI.
    HelpAboutUi,
    /// The general product UI.
    ProductUi,
}

impl M5EfficiencyConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ShellStatusUi,
        Self::ActivityCenterUi,
        Self::BackgroundWorkUi,
        Self::OverrideSettingsUi,
        Self::DiagnosticsUi,
        Self::SupportExport,
        Self::HelpAboutUi,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellStatusUi => "shell_status_ui",
            Self::ActivityCenterUi => "activity_center_ui",
            Self::BackgroundWorkUi => "background_work_ui",
            Self::OverrideSettingsUi => "override_settings_ui",
            Self::DiagnosticsUi => "diagnostics_ui",
            Self::SupportExport => "support_export",
            Self::HelpAboutUi => "help_about_ui",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no efficiency truth is
/// hover-only, pointer-only, toast-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyAccessibilityRoute {
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
    /// Present in the support / export packet, never toast-only.
    SupportExportable,
}

impl M5EfficiencyAccessibilityRoute {
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

/// Reason an adaptive-efficiency component has degraded below its qualified state. Required on
/// every row so a stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The power/thermal pressure signal is unavailable.
    PressureSignalUnavailable,
    /// Policy evaluation for the override is stale.
    PolicyEvaluationStale,
    /// Override evaluation could not be resolved.
    OverrideEvaluationUnresolved,
    /// The resumed-work backlog size is unknown.
    ResumeBacklogUnknown,
    /// An upstream governance-matrix lane narrowed.
    UpstreamGovernanceNarrowed,
}

impl M5EfficiencyDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::PressureSignalUnavailable,
        Self::PolicyEvaluationStale,
        Self::OverrideEvaluationUnresolved,
        Self::ResumeBacklogUnknown,
        Self::UpstreamGovernanceNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PressureSignalUnavailable => "pressure_signal_unavailable",
            Self::PolicyEvaluationStale => "policy_evaluation_stale",
            Self::OverrideEvaluationUnresolved => "override_evaluation_unresolved",
            Self::ResumeBacklogUnknown => "resume_backlog_unknown",
            Self::UpstreamGovernanceNarrowed => "upstream_governance_narrowed",
        }
    }
}

/// Mandatory label a claimed efficiency component must be able to show. The first three are
/// hard requirements on every component; the remaining three close the acceptance-criteria
/// ambiguity about source-of-change, override/policy owner, and resume/stale continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state / disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The source of change and active efficiency state behind the component.
    SourceOfChange,
    /// The override availability and policy owner behind the component.
    OverrideAndPolicyOwner,
    /// The resumed-work backlog and stale-result continuity behind the component.
    ResumeAndStaleContinuity,
}

impl M5EfficiencyRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::SourceOfChange,
        Self::OverrideAndPolicyOwner,
        Self::ResumeAndStaleContinuity,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::SourceOfChange => "source_of_change",
            Self::OverrideAndPolicyOwner => "override_and_policy_owner",
            Self::ResumeAndStaleContinuity => "resume_and_stale_continuity",
        }
    }
}

/// Qualification class for an M5 efficiency component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyQualificationClass {
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

impl M5EfficiencyQualificationClass {
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

/// Downgrade trigger that narrows an efficiency component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyDowngradeTrigger {
    /// A component left its source of change unstated.
    SourceOfChangeUnstated,
    /// A component left its active efficiency state unstated.
    EfficiencyStateUnstated,
    /// A component left slowed-versus-paused work ambiguous.
    SlowedVersusPausedAmbiguous,
    /// A component left what still works unstated.
    WhatStillWorksUnstated,
    /// A component left override availability unstated.
    OverrideAvailabilityUnstated,
    /// A component left its policy owner unstated.
    PolicyOwnerUnstated,
    /// A component hid the resumed-work backlog.
    ResumeBacklogHidden,
    /// A component cleared stale-result continuity on resume.
    StaleResultContinuityCleared,
    /// Generic low-power wording concealed source, state, or disposition truth.
    GenericLowPowerWordingUsed,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// Paused work was surfaced through toast-only messaging.
    PausedWorkToastOnly,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5EfficiencyDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::SourceOfChangeUnstated,
        Self::EfficiencyStateUnstated,
        Self::SlowedVersusPausedAmbiguous,
        Self::WhatStillWorksUnstated,
        Self::OverrideAvailabilityUnstated,
        Self::PolicyOwnerUnstated,
        Self::ResumeBacklogHidden,
        Self::StaleResultContinuityCleared,
        Self::GenericLowPowerWordingUsed,
        Self::AlternateStateLabelInvented,
        Self::PausedWorkToastOnly,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceOfChangeUnstated => "source_of_change_unstated",
            Self::EfficiencyStateUnstated => "efficiency_state_unstated",
            Self::SlowedVersusPausedAmbiguous => "slowed_versus_paused_ambiguous",
            Self::WhatStillWorksUnstated => "what_still_works_unstated",
            Self::OverrideAvailabilityUnstated => "override_availability_unstated",
            Self::PolicyOwnerUnstated => "policy_owner_unstated",
            Self::ResumeBacklogHidden => "resume_backlog_hidden",
            Self::StaleResultContinuityCleared => "stale_result_continuity_cleared",
            Self::GenericLowPowerWordingUsed => "generic_low_power_wording_used",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::PausedWorkToastOnly => "paused_work_toast_only",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed efficiency component family bound to the surface-
/// specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EfficiencyComponentRow {
    /// Governed component family.
    pub component_family: M5EfficiencyComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5EfficiencyQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this component.
    pub surface_families: Vec<M5EfficiencySurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5EfficiencyDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5EfficiencyRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5EfficiencyRequiredLabel>,
    /// Work dispositions this component can carry (the frozen AC vocabulary; required on every
    /// component).
    pub work_dispositions: Vec<M5EfficiencyWorkDisposition>,
    /// Source-of-change pressure signals this component names (power-state-indicator only).
    pub pressure_sources: Vec<EfficiencyPressureSource>,
    /// Active efficiency states this component names (power-state-indicator only).
    pub efficiency_states: Vec<EfficiencyState>,
    /// Affected workload families this component names (throttled/background families only).
    pub affected_workloads: Vec<WorkloadFamily>,
    /// Override postures this component names (override sheet / policy note only).
    pub override_postures: Vec<OverridePosture>,
    /// Policy owners this component names (override sheet / policy note only).
    pub policy_owners: Vec<M5EfficiencyPolicyOwner>,
    /// Recovery states this component names (resume-summary-card only).
    pub recovery_states: Vec<EfficiencyRecoveryState>,
    /// Stale-result continuity states this component names (stale-result note only).
    pub stale_result_states: Vec<M5EfficiencyStaleResultState>,
    /// Degraded reasons this component can name (required on every component).
    pub degraded_reasons: Vec<M5EfficiencyDegradedReason>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5EfficiencyAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5EfficiencyConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5EfficiencyDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component (must include its own canonical
    /// component schema so downstream rows have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never collapses battery saver, thermal pressure,
    /// low-power mode, and policy cap into one generic warning. MUST be `false`.
    pub collapses_pressure_sources_into_generic_warning: bool,
    /// Hard invariant: this component never hides paused work behind toast-only messaging.
    /// MUST be `false`.
    pub hides_paused_work_behind_toast_only: bool,
    /// Hard invariant: this component never presents an override as available when policy
    /// blocks it. MUST be `false`.
    pub presents_override_available_when_policy_blocks: bool,
    /// Hard invariant: this component never clears stale-result context merely because
    /// background work resumed. MUST be `false`.
    pub clears_stale_context_on_resume: bool,
}

impl M5EfficiencyComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5EfficiencyRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5EfficiencyRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_pressure_sources_into_generic_warning
            && !self.hides_paused_work_behind_toast_only
            && !self.presents_override_available_when_policy_blocks
            && !self.clears_stale_context_on_resume
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EfficiencyComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Work-disposition tokens.
    pub work_dispositions: Vec<String>,
    /// Pressure-source tokens (bound from the efficiency object model).
    pub pressure_sources: Vec<String>,
    /// Efficiency-state tokens (bound from the efficiency object model).
    pub efficiency_states: Vec<String>,
    /// Affected-workload tokens (bound from the efficiency object model).
    pub affected_workloads: Vec<String>,
    /// Override-posture tokens (bound from the efficiency governance model).
    pub override_postures: Vec<String>,
    /// Policy-owner tokens.
    pub policy_owners: Vec<String>,
    /// Recovery-state tokens (bound from the efficiency governance model).
    pub recovery_states: Vec<String>,
    /// Stale-result-state tokens.
    pub stale_result_states: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5EfficiencyComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5EfficiencyComponentFamily::ALL, |v| v.as_str()),
            work_dispositions: tokens(&M5EfficiencyWorkDisposition::ALL, |v| v.as_str()),
            pressure_sources: tokens(&EfficiencyPressureSource::ALL, |v| v.as_str()),
            efficiency_states: tokens(&EfficiencyState::ALL, |v| v.as_str()),
            affected_workloads: tokens(&AFFECTED_WORKLOADS, |v| v.as_str()),
            override_postures: tokens(&OverridePosture::ALL, |v| v.as_str()),
            policy_owners: tokens(&M5EfficiencyPolicyOwner::ALL, |v| v.as_str()),
            recovery_states: tokens(&EfficiencyRecoveryState::ALL, |v| v.as_str()),
            stale_result_states: tokens(&M5EfficiencyStaleResultState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5EfficiencySurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5EfficiencyDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5EfficiencyConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5EfficiencyAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5EfficiencyDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5EfficiencyRequiredLabel::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The canonical workload families bound from the shared efficiency object model, in canonical
/// order. [`WorkloadFamily`] does not export its own `ALL`, so the matrix pins the full set here
/// to keep the frozen vocabulary stable and complete.
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
pub struct M5EfficiencyComponentGovernanceReview {
    /// The power-state indicator shows its source of change and active state.
    pub power_state_indicator_shows_source_and_state: bool,
    /// The throttled-subsystem row shows its affected workload.
    pub throttled_subsystem_row_shows_affected_workload: bool,
    /// The background-work row shows slowed-versus-paused explicitly.
    pub background_work_row_shows_slowed_versus_paused: bool,
    /// The background-work banner shows paused work explicitly, never toast-only.
    pub background_work_banner_shows_paused_work_explicitly: bool,
    /// The per-workspace override sheet shows override availability.
    pub per_workspace_override_sheet_shows_override_availability: bool,
    /// The override-policy note row shows its policy owner.
    pub override_policy_note_row_shows_policy_owner: bool,
    /// The resume-summary card shows the resumed-work backlog.
    pub resume_summary_card_shows_resumed_backlog: bool,
    /// The stale-result continuity note keeps stale context across resume.
    pub stale_result_continuity_note_keeps_stale_context: bool,
    /// No surface collapses distinct pressure sources into one generic warning.
    pub no_surface_collapses_pressure_into_generic_warning: bool,
    /// The source of change is always explicit.
    pub source_of_change_always_explicit: bool,
    /// The active efficiency state is always explicit.
    pub active_efficiency_state_always_explicit: bool,
    /// Slowed-versus-paused work is always explicit.
    pub slowed_versus_paused_always_explicit: bool,
    /// Override availability and policy owner are always explicit where they apply.
    pub override_availability_and_policy_owner_always_explicit: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel efficiency vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EfficiencyComponentConsumerProjection {
    /// Shell surfaces consume the shared efficiency-state vocabulary.
    pub shell_surfaces_consume_state_vocabulary: bool,
    /// Activity surfaces consume the shared work-disposition vocabulary.
    pub activity_surfaces_consume_disposition_vocabulary: bool,
    /// Override surfaces consume the shared override / policy vocabulary.
    pub override_surfaces_consume_policy_vocabulary: bool,
    /// Resume surfaces consume the shared recovery vocabulary.
    pub resume_surfaces_consume_recovery_vocabulary: bool,
    /// Diagnostics surfaces consume the shared source-of-change vocabulary.
    pub diagnostics_surfaces_consume_source_vocabulary: bool,
    /// Support / export reads a single canonical efficiency source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EfficiencyComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the efficiency component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EfficiencyComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting efficiency audit for the lane.
    pub efficiency_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5EfficiencyComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EfficiencyComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5EfficiencyComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EfficiencyComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EfficiencyComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EfficiencyComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EfficiencyComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EfficiencyComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 efficiency component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EfficiencyComponentMatrixPacket {
    /// Record kind; must equal [`M5_EFFICIENCY_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EFFICIENCY_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5EfficiencyComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EfficiencyComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EfficiencyComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EfficiencyComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EfficiencyComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EfficiencyComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5EfficiencyComponentMatrixPacket {
    /// Builds an M5 efficiency component matrix packet from stable-lane input.
    pub fn new(input: M5EfficiencyComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_EFFICIENCY_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_EFFICIENCY_COMPONENT_MATRIX_SCHEMA_VERSION,
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

    /// Validates the M5 efficiency component matrix invariants.
    pub fn validate(&self) -> Vec<M5EfficiencyComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EFFICIENCY_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5EfficiencyComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EFFICIENCY_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5EfficiencyComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5EfficiencyComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 efficiency component matrix packet serializes"),
        ) {
            violations.push(M5EfficiencyComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 efficiency component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.component_family.canonical_component_schema_ref(),
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
            "# M5 Power-State-Indicator, Throttled-Subsystem-Row, Background-Work-Row, Background-Work-Banner, Per-Workspace-Override-Sheet, Override-Policy-Note-Row, Resume-Summary-Card, and Stale-Result-Continuity-Note Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Work dispositions: {}\n",
            self.vocabulary_set.work_dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Pressure sources: {}\n",
            self.vocabulary_set.pressure_sources.join(", ")
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
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.component_family.canonical_component_schema_ref()
            ));
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

/// Errors emitted when reading the checked-in M5 efficiency matrix export.
#[derive(Debug)]
pub enum M5EfficiencyComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5EfficiencyComponentMatrixViolation>),
}

impl fmt::Display for M5EfficiencyComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 efficiency component matrix export parse failed: {error}"
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
                    "m5 efficiency component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5EfficiencyComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5EfficiencyComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5EfficiencyComponentMatrixViolation {
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
    /// A component row does not point at its own canonical component schema.
    ComponentSchemaRefMissing,
    /// A component declares no work dispositions.
    WorkDispositionMissing,
    /// A power-state-indicator component declares no pressure sources.
    PressureSourceMissing,
    /// A power-state-indicator component declares no efficiency states.
    EfficiencyStateMissing,
    /// A throttled/background component declares no affected workloads.
    AffectedWorkloadMissing,
    /// An override sheet / policy-note component declares no override postures.
    OverridePostureMissing,
    /// An override sheet / policy-note component declares no policy owners.
    PolicyOwnerMissing,
    /// A resume-summary-card component declares no recovery states.
    RecoveryStateMissing,
    /// A stale-result note component declares no stale-result states.
    StaleResultStateMissing,
    /// A component declares no degraded reasons.
    DegradedReasonMissing,
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
    /// A component violates a hard invariant (collapsed pressure sources, toast-only paused
    /// work, override-available-when-blocked, or cleared stale context on resume).
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

impl M5EfficiencyComponentMatrixViolation {
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
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::WorkDispositionMissing => "work_disposition_missing",
            Self::PressureSourceMissing => "pressure_source_missing",
            Self::EfficiencyStateMissing => "efficiency_state_missing",
            Self::AffectedWorkloadMissing => "affected_workload_missing",
            Self::OverridePostureMissing => "override_posture_missing",
            Self::PolicyOwnerMissing => "policy_owner_missing",
            Self::RecoveryStateMissing => "recovery_state_missing",
            Self::StaleResultStateMissing => "stale_result_state_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
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

/// Reads and validates the checked-in stable M5 efficiency matrix export.
pub fn current_stable_m5_efficiency_component_matrix_export(
) -> Result<M5EfficiencyComponentMatrixPacket, M5EfficiencyComponentMatrixArtifactError> {
    let packet: M5EfficiencyComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-efficiency-components-proof/support_export.json"
    )))
    .map_err(M5EfficiencyComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5EfficiencyComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5EfficiencyComponentMatrixPacket,
    violations: &mut Vec<M5EfficiencyComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
        M5_EFFICIENCY_COMPONENT_DOC_REF,
        M5_POWER_STATE_INDICATOR_SCHEMA_REF,
        M5_THROTTLED_SUBSYSTEM_ROW_SCHEMA_REF,
        M5_BACKGROUND_WORK_ROW_SCHEMA_REF,
        M5_BACKGROUND_WORK_BANNER_SCHEMA_REF,
        M5_EFFICIENCY_OVERRIDE_SHEET_SCHEMA_REF,
        M5_OVERRIDE_POLICY_NOTE_ROW_SCHEMA_REF,
        M5_RESUME_SUMMARY_CARD_SCHEMA_REF,
        M5_STALE_RESULT_CONTINUITY_NOTE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5EfficiencyComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5EfficiencyComponentMatrixPacket,
    violations: &mut Vec<M5EfficiencyComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5EfficiencyComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5EfficiencyComponentMatrixPacket,
    violations: &mut Vec<M5EfficiencyComponentMatrixViolation>,
) {
    let present: BTreeSet<M5EfficiencyComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5EfficiencyComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5EfficiencyComponentMatrixViolation::RequiredComponentMissing);
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
            violations.push(M5EfficiencyComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5EfficiencyComponentMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_component_schema_ref())
        {
            violations.push(M5EfficiencyComponentMatrixViolation::ComponentSchemaRefMissing);
        }
        if row.work_dispositions.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::WorkDispositionMissing);
        }
        if family.declares_pressure_source() && row.pressure_sources.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::PressureSourceMissing);
        }
        if family.declares_efficiency_state() && row.efficiency_states.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::EfficiencyStateMissing);
        }
        if family.declares_affected_workload() && row.affected_workloads.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::AffectedWorkloadMissing);
        }
        if family.declares_override_posture() && row.override_postures.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::OverridePostureMissing);
        }
        if family.declares_policy_owner() && row.policy_owners.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::PolicyOwnerMissing);
        }
        if family.declares_recovery_state() && row.recovery_states.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::RecoveryStateMissing);
        }
        if family.declares_stale_result_state() && row.stale_result_states.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::StaleResultStateMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5EfficiencyComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5EfficiencyComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5EfficiencyComponentMatrixPacket,
    violations: &mut Vec<M5EfficiencyComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.power_state_indicator_shows_source_and_state,
        review.throttled_subsystem_row_shows_affected_workload,
        review.background_work_row_shows_slowed_versus_paused,
        review.background_work_banner_shows_paused_work_explicitly,
        review.per_workspace_override_sheet_shows_override_availability,
        review.override_policy_note_row_shows_policy_owner,
        review.resume_summary_card_shows_resumed_backlog,
        review.stale_result_continuity_note_keeps_stale_context,
        review.no_surface_collapses_pressure_into_generic_warning,
        review.source_of_change_always_explicit,
        review.active_efficiency_state_always_explicit,
        review.slowed_versus_paused_always_explicit,
        review.override_availability_and_policy_owner_always_explicit,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5EfficiencyComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5EfficiencyComponentMatrixPacket,
    violations: &mut Vec<M5EfficiencyComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_surfaces_consume_state_vocabulary,
        projection.activity_surfaces_consume_disposition_vocabulary,
        projection.override_surfaces_consume_policy_vocabulary,
        projection.resume_surfaces_consume_recovery_vocabulary,
        projection.diagnostics_surfaces_consume_source_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5EfficiencyComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5EfficiencyComponentMatrixPacket,
    violations: &mut Vec<M5EfficiencyComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5EfficiencyComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5EfficiencyComponentMatrixPacket,
    violations: &mut Vec<M5EfficiencyComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.efficiency_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5EfficiencyComponentMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled
/// vocabulary deliberately uses efficiency words; what is rejected is a raw secret *value*
/// shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Repo-relative refs of the efficiency governance object model this matrix binds against, so
/// no consumer forks its own low-power vocabulary. Re-exported for callers that assemble the
/// full source-contract set.
pub const M5_EFFICIENCY_GOVERNANCE_BINDING_REFS: [&str; 2] = [
    M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
    M5_EFFICIENCY_GOVERNANCE_MATRIX_REF,
];
