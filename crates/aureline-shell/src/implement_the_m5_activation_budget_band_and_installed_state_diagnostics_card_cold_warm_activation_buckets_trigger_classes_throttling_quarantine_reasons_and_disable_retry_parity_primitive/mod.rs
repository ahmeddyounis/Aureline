//! Implemented M5 activation-budget-band and installed-state-diagnostics-card primitives.
//!
//! The frozen [marketplace / install-review component matrix][matrix] names the reusable
//! extension-marketplace UI components and locks their controlled vocabulary. This module is the
//! third implement lane over that matrix: it turns the two performance-and-stability components —
//! the **activation-budget band** and the **installed-state diagnostics card** — into resolvers
//! that produce export-safe, honest projections, so a user can read the activation-budget class,
//! cold / warm activation evidence, activation triggers, exercised capabilities, throttling /
//! quarantine reasons, and disable / retry actions from the marketplace listing, install review,
//! installed-state diagnostics, help, and exported surfaces without digging into logs.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render activation-budget bands with low / medium / high / over-budget classes plus cold /
//!   warm evidence where available.** [`resolve_activation_budget_band`] refuses to read as a clean
//!   band when the artifact identity or budget band is unstated, when an over-budget artifact reads
//!   as cost-free, when a runtime-degraded artifact carries no cold / warm activation evidence, or
//!   when Certified / Supported language is left in place while the evidence is no longer current;
//!   it degrades instead.
//! * **Render installed-state diagnostics cards with activation triggers, exercised capability
//!   summaries, throttling / quarantine reasons, and disable / retry actions.**
//!   [`resolve_installed_state_diagnostics_card`] degrades when the budget or quarantine state is
//!   unresolved, when activation triggers or exercised capabilities are unstated, when a quarantined
//!   artifact reads as healthy, when a throttled / quarantined artifact carries no reason, when the
//!   disable / retry action pair is broken, or when Certified / Supported language survives on stale
//!   evidence.
//! * **Keep activation-budget language aligned across marketplace, installed-state diagnostics,
//!   help, and support exports.** Both resolvers reuse the frozen matrix activation-budget and
//!   quarantine vocabularies directly, so no surface forks its own budget or quarantine wording, and
//!   the packet proves — by resolved examples, not governance bools — that performance and
//!   quarantine implications are legible before install and after runtime degradation.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5MarketplaceInstallDisposition`] marketplace / install-disposition vocabulary, the
//! [`M5ActivationBudgetBandState`] activation-budget vocabulary, the [`M5QuarantineState`]
//! quarantine vocabulary, and the [`M5CompatibilityState`] compatibility vocabulary — so
//! marketplace, extensions, install-review, help, and support surfaces can never fork their own
//! budget, quarantine, or compatibility wording or invent feature-local badges. Raw secret values
//! and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_activation_diagnostics_controls,
    seeded_m5_activation_diagnostics_controls_installed_state_ui_preview_narrowed,
    seeded_m5_activation_diagnostics_controls_marketplace_ui_beta_narrowed,
    M5_ACTIVATION_DIAGNOSTICS_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix::{
    M5ActivationBudgetBandState, M5CompatibilityState, M5MarketplaceInstallAccessibilityRoute,
    M5MarketplaceInstallComponentFamily, M5MarketplaceInstallConsumerSurface,
    M5MarketplaceInstallDeploymentLine, M5MarketplaceInstallDisposition,
    M5MarketplaceInstallDowngradeTrigger, M5MarketplaceInstallQualificationClass,
    M5MarketplaceInstallRequiredLabel, M5QuarantineState, M5_ACTIVATION_BUDGET_BAND_SCHEMA_REF,
    M5_INSTALLED_STATE_DIAGNOSTICS_CARD_SCHEMA_REF, M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
    M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ActivationDiagnosticsControlsPacket`].
pub const M5_ACTIVATION_DIAGNOSTICS_CONTROLS_RECORD_KIND: &str =
    "implement_m5_activation_budget_band_and_installed_state_diagnostics_card_controls";

/// Schema version for M5 activation-budget-band / installed-state-diagnostics-card controls records.
pub const M5_ACTIVATION_DIAGNOSTICS_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_ACTIVATION_DIAGNOSTICS_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-activation-budget-band-installed-state-diagnostics-card-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_ACTIVATION_DIAGNOSTICS_CONTROLS_DOC_REF: &str =
    "docs/marketplace/m5_activation_budget_band_and_installed_state_diagnostics_card_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ACTIVATION_DIAGNOSTICS_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-activation-budget-band-installed-state-diagnostics-card-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_ACTIVATION_DIAGNOSTICS_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-activation-budget-band-installed-state-diagnostics-card-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_ACTIVATION_DIAGNOSTICS_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-activation-budget-band-installed-state-diagnostics-card-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_ACTIVATION_DIAGNOSTICS_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-activation-budget-band-installed-state-diagnostics-card-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface
/// taxonomy so no lane invents a parallel surface set.
pub type M5ActivationDiagnosticsConsumerSurface = M5MarketplaceInstallConsumerSurface;

/// Controlled activation-budget band class a band renders — the spec's low / medium / high /
/// over-budget presentation. Minted by this lane because the frozen matrix carries the
/// [`M5ActivationBudgetBandState`] runtime state but not the coarse legibility class the band shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActivationBudgetBandClass {
    /// Low activation cost, well within budget.
    Low,
    /// Medium activation cost.
    Medium,
    /// High activation cost, approaching budget.
    High,
    /// Over the activation budget.
    OverBudget,
    /// The activation-budget class cannot currently be resolved.
    Unknown,
}

impl M5ActivationBudgetBandClass {
    /// Every band class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Low,
        Self::Medium,
        Self::High,
        Self::OverBudget,
        Self::Unknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::OverBudget => "over_budget",
            Self::Unknown => "unknown",
        }
    }
}

/// Controlled activation cost level a single cold- or warm-start evidence measurement carries.
/// Minted by this lane so the low / medium / high band class can be refined by measured cold / warm
/// activation cost where evidence is available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActivationCostLevel {
    /// Low measured activation cost.
    Low,
    /// Medium measured activation cost.
    Medium,
    /// High measured activation cost.
    High,
}

impl M5ActivationCostLevel {
    /// Every cost level, in declaration order.
    pub const ALL: [Self; 3] = [Self::Low, Self::Medium, Self::High];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }

    /// Ordinal used to pick the worst of two measurements.
    const fn rank(self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Medium => 1,
            Self::High => 2,
        }
    }

    /// The corresponding band class for a within-budget measurement.
    const fn band_class(self) -> M5ActivationBudgetBandClass {
        match self {
            Self::Low => M5ActivationBudgetBandClass::Low,
            Self::Medium => M5ActivationBudgetBandClass::Medium,
            Self::High => M5ActivationBudgetBandClass::High,
        }
    }
}

/// Controlled activation-trigger class an installed-state diagnostics card names, so an artifact's
/// activation cost is attributable to concrete triggers rather than hidden. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActivationTriggerClass {
    /// Activates on editor / host startup.
    OnStartup,
    /// Activates on an explicit command invocation.
    OnCommand,
    /// Activates on a language / grammar event.
    OnLanguageEvent,
    /// Activates on a file-type open.
    OnFileType,
    /// Activates on a view / panel open.
    OnViewOpen,
    /// Activates on a debug session.
    OnDebugSession,
}

impl M5ActivationTriggerClass {
    /// Every activation-trigger class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OnStartup,
        Self::OnCommand,
        Self::OnLanguageEvent,
        Self::OnFileType,
        Self::OnViewOpen,
        Self::OnDebugSession,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnStartup => "on_startup",
            Self::OnCommand => "on_command",
            Self::OnLanguageEvent => "on_language_event",
            Self::OnFileType => "on_file_type",
            Self::OnViewOpen => "on_view_open",
            Self::OnDebugSession => "on_debug_session",
        }
    }
}

/// Controlled exercised-capability class an installed-state diagnostics card summarizes, so the
/// capabilities an artifact actually used at runtime are legible rather than inferred. Minted by
/// this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExercisedCapabilityClass {
    /// Read the file system.
    FileSystemRead,
    /// Wrote to the file system.
    FileSystemWrite,
    /// Made network requests.
    Network,
    /// Spawned a child process.
    ProcessSpawn,
    /// Accessed the clipboard.
    ClipboardAccess,
    /// Emitted telemetry.
    Telemetry,
}

impl M5ExercisedCapabilityClass {
    /// Every exercised-capability class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FileSystemRead,
        Self::FileSystemWrite,
        Self::Network,
        Self::ProcessSpawn,
        Self::ClipboardAccess,
        Self::Telemetry,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileSystemRead => "file_system_read",
            Self::FileSystemWrite => "file_system_write",
            Self::Network => "network",
            Self::ProcessSpawn => "process_spawn",
            Self::ClipboardAccess => "clipboard_access",
            Self::Telemetry => "telemetry",
        }
    }
}

/// Controlled throttle / quarantine reason an installed-state diagnostics card names, so a throttled
/// or quarantined artifact never leaves its cause implicit. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ThrottleQuarantineReason {
    /// Exceeded its activation budget.
    ActivationBudgetExceeded,
    /// Crashed repeatedly.
    RepeatedCrashes,
    /// Placed the host under high memory pressure.
    HighMemoryPressure,
    /// Violated an installed policy.
    PolicyViolation,
    /// The publisher signature was revoked.
    PublisherRevoked,
    /// Manually quarantined by an operator.
    ManualQuarantine,
}

impl M5ThrottleQuarantineReason {
    /// Every throttle / quarantine reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ActivationBudgetExceeded,
        Self::RepeatedCrashes,
        Self::HighMemoryPressure,
        Self::PolicyViolation,
        Self::PublisherRevoked,
        Self::ManualQuarantine,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivationBudgetExceeded => "activation_budget_exceeded",
            Self::RepeatedCrashes => "repeated_crashes",
            Self::HighMemoryPressure => "high_memory_pressure",
            Self::PolicyViolation => "policy_violation",
            Self::PublisherRevoked => "publisher_revoked",
            Self::ManualQuarantine => "manual_quarantine",
        }
    }
}

/// Controlled remediation action an installed-state diagnostics card offers. Minted by this lane so
/// a diagnostics card always keeps the disable / retry pair legible rather than routing a user to a
/// single generic action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticsRemediationAction {
    /// Retry activation for the artifact.
    RetryActivation,
    /// Disable in this workspace.
    DisableWorkspace,
    /// Disable globally.
    DisableGlobal,
    /// Release a prior quarantine.
    ReleaseQuarantine,
    /// View the activation / crash logs.
    ViewLogs,
    /// Report an issue to the publisher.
    ReportIssue,
}

impl M5DiagnosticsRemediationAction {
    /// Every remediation action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RetryActivation,
        Self::DisableWorkspace,
        Self::DisableGlobal,
        Self::ReleaseQuarantine,
        Self::ViewLogs,
        Self::ReportIssue,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetryActivation => "retry_activation",
            Self::DisableWorkspace => "disable_workspace",
            Self::DisableGlobal => "disable_global",
            Self::ReleaseQuarantine => "release_quarantine",
            Self::ViewLogs => "view_logs",
            Self::ReportIssue => "report_issue",
        }
    }

    /// True when this action retries activation.
    const fn is_retry(self) -> bool {
        matches!(self, Self::RetryActivation)
    }

    /// True when this action disables the artifact in some scope.
    const fn is_disable(self) -> bool {
        matches!(self, Self::DisableWorkspace | Self::DisableGlobal)
    }
}

/// One mandatory rendered part an activation-budget band or installed-state diagnostics card must be
/// able to show, so no performance, trigger, quarantine, or remediation fact is left implicit behind
/// compact chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActivationDiagnosticsAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The activation-budget band / state (band).
    ActivationBudgetBand,
    /// The low / medium / high / over-budget band class (band).
    BandClass,
    /// The cold / warm activation evidence (band).
    ColdWarmEvidence,
    /// The over-budget disclosure (band).
    OverBudgetDisclosure,
    /// The activation triggers (card).
    ActivationTriggers,
    /// The exercised-capability summary (card).
    ExercisedCapabilities,
    /// The throttle / quarantine reason (card).
    ThrottleQuarantineReason,
    /// The quarantine history (card).
    QuarantineHistory,
    /// The disable action (card).
    DisableAction,
    /// The retry action (card).
    RetryAction,
    /// The evidence-freshness disclosure (both components).
    EvidenceFreshness,
}

impl M5ActivationDiagnosticsAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ActivationBudgetBand,
        Self::BandClass,
        Self::ColdWarmEvidence,
        Self::OverBudgetDisclosure,
        Self::ActivationTriggers,
        Self::ExercisedCapabilities,
        Self::ThrottleQuarantineReason,
        Self::QuarantineHistory,
        Self::DisableAction,
        Self::RetryAction,
        Self::EvidenceFreshness,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ActivationBudgetBand => "activation_budget_band",
            Self::BandClass => "band_class",
            Self::ColdWarmEvidence => "cold_warm_evidence",
            Self::OverBudgetDisclosure => "over_budget_disclosure",
            Self::ActivationTriggers => "activation_triggers",
            Self::ExercisedCapabilities => "exercised_capabilities",
            Self::ThrottleQuarantineReason => "throttle_quarantine_reason",
            Self::QuarantineHistory => "quarantine_history",
            Self::DisableAction => "disable_action",
            Self::RetryAction => "retry_action",
            Self::EvidenceFreshness => "evidence_freshness",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to review the fact
/// behind a degraded activation-budget band or diagnostics card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActivationDiagnosticsNextAction {
    /// Review the activation-budget band, class, triggers, and exercised capabilities.
    ReviewActivationBudget,
    /// Review the cold / warm activation evidence.
    ReviewColdWarmEvidence,
    /// Review the throttle / quarantine reason and quarantine history.
    ReviewQuarantineReason,
    /// Review the disable / retry remediation actions.
    ReviewDisableRetryActions,
    /// Review the evidence freshness for a stale signal.
    ReviewEvidenceFreshness,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5ActivationDiagnosticsNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewActivationBudget,
        Self::ReviewColdWarmEvidence,
        Self::ReviewQuarantineReason,
        Self::ReviewDisableRetryActions,
        Self::ReviewEvidenceFreshness,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewActivationBudget => "review_activation_budget",
            Self::ReviewColdWarmEvidence => "review_cold_warm_evidence",
            Self::ReviewQuarantineReason => "review_quarantine_reason",
            Self::ReviewDisableRetryActions => "review_disable_retry_actions",
            Self::ReviewEvidenceFreshness => "review_evidence_freshness",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActivationDiagnosticsExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The marketplace dispositions carried.
    Dispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The activation-budget band class named by the band.
    ActivationBudgetBandClass,
    /// The cold / warm activation evidence named by the band.
    ColdWarmEvidence,
    /// The quarantine state named by the card.
    QuarantineState,
    /// The throttle / quarantine reason named by the card.
    ThrottleQuarantineReason,
    /// The remediation actions named by the card.
    RemediationActions,
    /// The evidence-freshness disclosure named by both components.
    EvidenceFreshness,
}

impl M5ActivationDiagnosticsExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::ActivationBudgetBandClass,
        Self::ColdWarmEvidence,
        Self::QuarantineState,
        Self::ThrottleQuarantineReason,
        Self::RemediationActions,
        Self::EvidenceFreshness,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::Dispositions => "dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::ActivationBudgetBandClass => "activation_budget_band_class",
            Self::ColdWarmEvidence => "cold_warm_evidence",
            Self::QuarantineState => "quarantine_state",
            Self::ThrottleQuarantineReason => "throttle_quarantine_reason",
            Self::RemediationActions => "remediation_actions",
            Self::EvidenceFreshness => "evidence_freshness",
        }
    }
}

/// Reason an activation-budget band degraded below a clean, fully-legible state. The degrade-first
/// ladder returns one of these instead of ever letting an ambiguous band read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActivationBudgetBandDegradeReason {
    /// The artifact identity is unstated.
    ArtifactIdentityUnstated,
    /// The activation-budget band cannot currently be resolved.
    BudgetBandUnresolved,
    /// An over-budget artifact reads as cost-free.
    OverBudgetShownAsCostFree,
    /// A runtime-degraded artifact carries no cold / warm activation evidence.
    ActivationEvidenceMissingAfterDegradation,
    /// Certified / Supported language is left in place while the evidence is stale.
    StaleEvidenceCertifiedOverclaim,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ActivationBudgetBandDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ArtifactIdentityUnstated,
        Self::BudgetBandUnresolved,
        Self::OverBudgetShownAsCostFree,
        Self::ActivationEvidenceMissingAfterDegradation,
        Self::StaleEvidenceCertifiedOverclaim,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentityUnstated => "artifact_identity_unstated",
            Self::BudgetBandUnresolved => "budget_band_unresolved",
            Self::OverBudgetShownAsCostFree => "over_budget_shown_as_cost_free",
            Self::ActivationEvidenceMissingAfterDegradation => {
                "activation_evidence_missing_after_degradation"
            }
            Self::StaleEvidenceCertifiedOverclaim => "stale_evidence_certified_overclaim",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ActivationDiagnosticsNextAction {
        match self {
            Self::ArtifactIdentityUnstated
            | Self::BudgetBandUnresolved
            | Self::OverBudgetShownAsCostFree => {
                M5ActivationDiagnosticsNextAction::ReviewActivationBudget
            }
            Self::ActivationEvidenceMissingAfterDegradation => {
                M5ActivationDiagnosticsNextAction::ReviewColdWarmEvidence
            }
            Self::StaleEvidenceCertifiedOverclaim | Self::ProofStale => {
                M5ActivationDiagnosticsNextAction::ReviewEvidenceFreshness
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5MarketplaceInstallDowngradeTrigger {
        match self {
            Self::ArtifactIdentityUnstated => {
                M5MarketplaceInstallDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::BudgetBandUnresolved
            | Self::OverBudgetShownAsCostFree
            | Self::ActivationEvidenceMissingAfterDegradation => {
                M5MarketplaceInstallDowngradeTrigger::ActivationCostHidden
            }
            Self::StaleEvidenceCertifiedOverclaim | Self::ProofStale => {
                M5MarketplaceInstallDowngradeTrigger::ProofStale
            }
        }
    }
}

/// Reason an installed-state diagnostics card degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstalledStateDiagnosticsCardDegradeReason {
    /// The artifact identity is unstated.
    ArtifactIdentityUnstated,
    /// The activation-budget band cannot currently be resolved.
    BudgetBandUnresolved,
    /// The quarantine state cannot currently be resolved.
    QuarantineStateUnresolved,
    /// The activation triggers are unstated.
    ActivationTriggersUnstated,
    /// The exercised-capability summary is unstated.
    ExercisedCapabilitiesUnstated,
    /// A quarantined artifact reads as healthy.
    QuarantineHistoryHidden,
    /// A throttled / quarantined artifact carries no reason.
    ThrottleQuarantineReasonMissing,
    /// The disable / retry action pair is broken (one offered without the other).
    DisableRetryActionsMissing,
    /// Certified / Supported language is left in place while the evidence is stale.
    StaleEvidenceCertifiedOverclaim,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5InstalledStateDiagnosticsCardDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ArtifactIdentityUnstated,
        Self::BudgetBandUnresolved,
        Self::QuarantineStateUnresolved,
        Self::ActivationTriggersUnstated,
        Self::ExercisedCapabilitiesUnstated,
        Self::QuarantineHistoryHidden,
        Self::ThrottleQuarantineReasonMissing,
        Self::DisableRetryActionsMissing,
        Self::StaleEvidenceCertifiedOverclaim,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentityUnstated => "artifact_identity_unstated",
            Self::BudgetBandUnresolved => "budget_band_unresolved",
            Self::QuarantineStateUnresolved => "quarantine_state_unresolved",
            Self::ActivationTriggersUnstated => "activation_triggers_unstated",
            Self::ExercisedCapabilitiesUnstated => "exercised_capabilities_unstated",
            Self::QuarantineHistoryHidden => "quarantine_history_hidden",
            Self::ThrottleQuarantineReasonMissing => "throttle_quarantine_reason_missing",
            Self::DisableRetryActionsMissing => "disable_retry_actions_missing",
            Self::StaleEvidenceCertifiedOverclaim => "stale_evidence_certified_overclaim",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ActivationDiagnosticsNextAction {
        match self {
            Self::ArtifactIdentityUnstated
            | Self::BudgetBandUnresolved
            | Self::ActivationTriggersUnstated
            | Self::ExercisedCapabilitiesUnstated => {
                M5ActivationDiagnosticsNextAction::ReviewActivationBudget
            }
            Self::QuarantineStateUnresolved
            | Self::QuarantineHistoryHidden
            | Self::ThrottleQuarantineReasonMissing => {
                M5ActivationDiagnosticsNextAction::ReviewQuarantineReason
            }
            Self::DisableRetryActionsMissing => {
                M5ActivationDiagnosticsNextAction::ReviewDisableRetryActions
            }
            Self::StaleEvidenceCertifiedOverclaim | Self::ProofStale => {
                M5ActivationDiagnosticsNextAction::ReviewEvidenceFreshness
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5MarketplaceInstallDowngradeTrigger {
        match self {
            Self::ArtifactIdentityUnstated
            | Self::ActivationTriggersUnstated
            | Self::ExercisedCapabilitiesUnstated
            | Self::DisableRetryActionsMissing => {
                M5MarketplaceInstallDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::BudgetBandUnresolved => {
                M5MarketplaceInstallDowngradeTrigger::ActivationCostHidden
            }
            Self::QuarantineStateUnresolved
            | Self::QuarantineHistoryHidden
            | Self::ThrottleQuarantineReasonMissing => {
                M5MarketplaceInstallDowngradeTrigger::QuarantineHistoryHidden
            }
            Self::StaleEvidenceCertifiedOverclaim | Self::ProofStale => {
                M5MarketplaceInstallDowngradeTrigger::ProofStale
            }
        }
    }
}

/// True when the activation-budget band state reads as over its budget / degraded at runtime.
fn budget_is_over(state: M5ActivationBudgetBandState) -> bool {
    matches!(
        state,
        M5ActivationBudgetBandState::OverBudget
            | M5ActivationBudgetBandState::Throttled
            | M5ActivationBudgetBandState::SuspendedOverBudget
    )
}

/// True when the quarantine state means the artifact has ever been quarantined.
fn quarantine_ever(state: M5QuarantineState) -> bool {
    matches!(
        state,
        M5QuarantineState::QuarantinedActive
            | M5QuarantineState::QuarantinedHistory
            | M5QuarantineState::QuarantinePending
            | M5QuarantineState::ReleasedFromQuarantine
    )
}

/// True when the quarantine state is currently actionable (active or pending review).
fn quarantine_active(state: M5QuarantineState) -> bool {
    matches!(
        state,
        M5QuarantineState::QuarantinedActive | M5QuarantineState::QuarantinePending
    )
}

/// Projects the frozen budget-band state plus optional cold / warm cost evidence into the controlled
/// low / medium / high / over-budget class the band renders.
fn band_class_for(
    state: M5ActivationBudgetBandState,
    cold: Option<M5ActivationCostLevel>,
    warm: Option<M5ActivationCostLevel>,
) -> M5ActivationBudgetBandClass {
    use M5ActivationBudgetBandClass as Class;
    if budget_is_over(state) {
        return Class::OverBudget;
    }
    if matches!(state, M5ActivationBudgetBandState::BudgetUnknown) {
        return Class::Unknown;
    }
    // Within / near budget: the class reflects the worst measured cold / warm cost, floored at
    // Medium when the artifact is already near its budget.
    let measured = match (cold, warm) {
        (Some(a), Some(b)) => {
            if a.rank() >= b.rank() {
                a
            } else {
                b
            }
        }
        (Some(a), None) => a,
        (None, Some(b)) => b,
        (None, None) => M5ActivationCostLevel::Low,
    };
    let class = measured.band_class();
    if matches!(state, M5ActivationBudgetBandState::NearBudget) && matches!(class, Class::Low) {
        Class::Medium
    } else {
        class
    }
}

/// Input to [`resolve_activation_budget_band`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ActivationBudgetBandResolutionInput {
    /// Stable identity of the band instance.
    pub band_id: String,
    /// The artifact identity (name / id) shown; empty means unstated.
    pub artifact_identity: String,
    /// The activation-budget band state.
    pub budget_state: M5ActivationBudgetBandState,
    /// The cold-start activation cost evidence, where available.
    pub cold_start_evidence: Option<M5ActivationCostLevel>,
    /// The warm-start activation cost evidence, where available.
    pub warm_start_evidence: Option<M5ActivationCostLevel>,
    /// True when the band carries Certified / Supported language.
    pub certified_or_supported_claimed: bool,
    /// True when the underlying activation evidence is current.
    pub evidence_fresh: bool,
    /// True when the band reads an over-budget artifact as cost-free.
    pub reads_over_budget_as_cost_free: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe activation-budget band projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedActivationBudgetBand {
    /// Stable identity of the band instance.
    pub band_id: String,
    /// The artifact identity named by the band.
    pub artifact_identity: String,
    /// The activation-budget band-state token named by the band.
    pub budget_state: String,
    /// The controlled low / medium / high / over-budget band class named by the band.
    pub band_class: String,
    /// Whether the artifact is over its activation budget.
    pub is_over_budget: bool,
    /// The cold-start activation cost token named by the band, or `null` when unavailable.
    pub cold_start_evidence: Option<String>,
    /// The warm-start activation cost token named by the band, or `null` when unavailable.
    pub warm_start_evidence: Option<String>,
    /// Whether any cold / warm activation evidence is available.
    pub has_cold_warm_evidence: bool,
    /// Whether Certified / Supported language is claimed.
    pub certified_or_supported_claimed: bool,
    /// Whether the underlying evidence is current.
    pub evidence_fresh: bool,
    /// Guardrail (MUST be `false` on a clean band): an over-budget artifact reads as cost-free.
    pub presents_over_budget_as_cost_free: bool,
    /// Guardrail (MUST be `false` on a clean band): stale evidence leaves a Certified / Supported
    /// overclaim in place.
    pub leaves_stale_certified_overclaim: bool,
    /// Degrade reason, if the band could not read as a clean state.
    pub degrade_reason: Option<M5ActivationBudgetBandDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ActivationDiagnosticsNextAction,
    /// Whether the activation-budget facts are legible in full (clean band naming every fact).
    pub fully_legible: bool,
}

impl M5ResolvedActivationBudgetBand {
    /// Whether this band reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_installed_state_diagnostics_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5InstalledStateDiagnosticsCardResolutionInput {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// The artifact identity (name / id) shown; empty means unstated.
    pub artifact_identity: String,
    /// The activation-budget band state.
    pub budget_state: M5ActivationBudgetBandState,
    /// The quarantine state.
    pub quarantine_state: M5QuarantineState,
    /// The compatibility state of the installed artifact.
    pub compatibility: M5CompatibilityState,
    /// The activation triggers exercised; empty means unstated.
    pub activation_triggers: Vec<M5ActivationTriggerClass>,
    /// The exercised-capability summary; empty means unstated.
    pub exercised_capabilities: Vec<M5ExercisedCapabilityClass>,
    /// The throttle / quarantine reason, where the artifact is throttled or quarantined.
    pub throttle_quarantine_reason: Option<M5ThrottleQuarantineReason>,
    /// The remediation actions the card offers.
    pub remediation_actions: Vec<M5DiagnosticsRemediationAction>,
    /// True when the card carries Certified / Supported language.
    pub certified_or_supported_claimed: bool,
    /// True when the underlying diagnostics evidence is current.
    pub evidence_fresh: bool,
    /// True when the card reads a quarantined artifact as healthy.
    pub reads_quarantine_as_healthy: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe installed-state diagnostics card projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedInstalledStateDiagnosticsCard {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// The artifact identity named by the card.
    pub artifact_identity: String,
    /// The activation-budget band-state token named by the card.
    pub budget_state: String,
    /// The quarantine-state token named by the card.
    pub quarantine_state: String,
    /// The compatibility token named by the card.
    pub compatibility: String,
    /// The activation-trigger tokens named by the card.
    pub activation_triggers: Vec<String>,
    /// The exercised-capability tokens named by the card.
    pub exercised_capabilities: Vec<String>,
    /// The throttle / quarantine reason token named by the card, or `null` when not applicable.
    pub throttle_quarantine_reason: Option<String>,
    /// The remediation-action tokens named by the card.
    pub remediation_actions: Vec<String>,
    /// Whether the card is in an actionable degraded state (throttled / suspended / quarantined).
    pub is_actionable: bool,
    /// Whether the card offers a disable action.
    pub offers_disable: bool,
    /// Whether the card offers a retry action.
    pub offers_retry: bool,
    /// Whether the card offers both the disable and retry actions (parity).
    pub disable_retry_parity: bool,
    /// Whether Certified / Supported language is claimed.
    pub certified_or_supported_claimed: bool,
    /// Whether the underlying evidence is current.
    pub evidence_fresh: bool,
    /// Guardrail (MUST be `false` on a clean card): a quarantined artifact reads as healthy.
    pub hides_quarantine_history: bool,
    /// Guardrail (MUST be `false` on a clean card): stale evidence leaves a Certified / Supported
    /// overclaim in place.
    pub leaves_stale_certified_overclaim: bool,
    /// Degrade reason, if the card could not read as a clean state.
    pub degrade_reason: Option<M5InstalledStateDiagnosticsCardDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ActivationDiagnosticsNextAction,
    /// Whether the diagnostics facts are legible in full (clean card naming every fact).
    pub fully_legible: bool,
}

impl M5ResolvedInstalledStateDiagnosticsCard {
    /// Whether this card reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ActivationDiagnosticsResolutionError {
    /// The band id was empty.
    EmptyBandId,
    /// The card id was empty.
    EmptyCardId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ActivationDiagnosticsResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyBandId => "empty_band_id",
            Self::EmptyCardId => "empty_card_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ActivationDiagnosticsResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 activation-diagnostics resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ActivationDiagnosticsResolutionError {}

/// Resolves an activation-budget band, keeping the performance cost explicit: the band names its
/// budget state, low / medium / high / over-budget class, and cold / warm activation evidence, never
/// reads an over-budget artifact as cost-free, keeps activation evidence legible after runtime
/// degradation, and narrows the claim the moment Certified / Supported evidence goes stale.
pub fn resolve_activation_budget_band(
    input: M5ActivationBudgetBandResolutionInput,
) -> Result<M5ResolvedActivationBudgetBand, M5ActivationDiagnosticsResolutionError> {
    if input.band_id.trim().is_empty() {
        return Err(M5ActivationDiagnosticsResolutionError::EmptyBandId);
    }
    if string_is_forbidden(&input.band_id) || string_is_forbidden(&input.artifact_identity) {
        return Err(M5ActivationDiagnosticsResolutionError::ForbiddenMaterial);
    }

    let is_over_budget = budget_is_over(input.budget_state);
    let has_cold_warm_evidence =
        input.cold_start_evidence.is_some() || input.warm_start_evidence.is_some();
    let presents_over_budget_as_cost_free = is_over_budget && input.reads_over_budget_as_cost_free;
    let leaves_stale_certified_overclaim =
        input.certified_or_supported_claimed && !input.evidence_fresh;
    let band_class = band_class_for(
        input.budget_state,
        input.cold_start_evidence,
        input.warm_start_evidence,
    );

    let degrade_reason = if input.artifact_identity.trim().is_empty() {
        Some(M5ActivationBudgetBandDegradeReason::ArtifactIdentityUnstated)
    } else if matches!(
        input.budget_state,
        M5ActivationBudgetBandState::BudgetUnknown
    ) {
        Some(M5ActivationBudgetBandDegradeReason::BudgetBandUnresolved)
    } else if presents_over_budget_as_cost_free {
        Some(M5ActivationBudgetBandDegradeReason::OverBudgetShownAsCostFree)
    } else if is_over_budget && !has_cold_warm_evidence {
        Some(M5ActivationBudgetBandDegradeReason::ActivationEvidenceMissingAfterDegradation)
    } else if leaves_stale_certified_overclaim {
        Some(M5ActivationBudgetBandDegradeReason::StaleEvidenceCertifiedOverclaim)
    } else if !input.proof_fresh {
        Some(M5ActivationBudgetBandDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ActivationDiagnosticsNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedActivationBudgetBand {
        band_id: input.band_id,
        artifact_identity: input.artifact_identity,
        budget_state: input.budget_state.as_str().to_owned(),
        band_class: band_class.as_str().to_owned(),
        is_over_budget,
        cold_start_evidence: input.cold_start_evidence.map(|c| c.as_str().to_owned()),
        warm_start_evidence: input.warm_start_evidence.map(|c| c.as_str().to_owned()),
        has_cold_warm_evidence,
        certified_or_supported_claimed: input.certified_or_supported_claimed,
        evidence_fresh: input.evidence_fresh,
        presents_over_budget_as_cost_free,
        leaves_stale_certified_overclaim,
        degrade_reason,
        next_action,
        fully_legible: degrade_reason.is_none(),
    })
}

/// Resolves an installed-state diagnostics card, keeping the performance and stability implications
/// explicit without digging into logs: the card names its activation triggers, exercised
/// capabilities, throttle / quarantine reason, and disable / retry actions, never reads a quarantined
/// artifact as healthy, keeps the disable / retry pair intact, and narrows the claim the moment
/// Certified / Supported evidence goes stale.
pub fn resolve_installed_state_diagnostics_card(
    input: M5InstalledStateDiagnosticsCardResolutionInput,
) -> Result<M5ResolvedInstalledStateDiagnosticsCard, M5ActivationDiagnosticsResolutionError> {
    if input.card_id.trim().is_empty() {
        return Err(M5ActivationDiagnosticsResolutionError::EmptyCardId);
    }
    if string_is_forbidden(&input.card_id) || string_is_forbidden(&input.artifact_identity) {
        return Err(M5ActivationDiagnosticsResolutionError::ForbiddenMaterial);
    }

    let is_actionable =
        budget_is_over(input.budget_state) || quarantine_active(input.quarantine_state);
    let offers_disable = input.remediation_actions.iter().any(|a| a.is_disable());
    let offers_retry = input.remediation_actions.iter().any(|a| a.is_retry());
    let disable_retry_parity = offers_disable && offers_retry;
    let hides_quarantine_history =
        quarantine_ever(input.quarantine_state) && input.reads_quarantine_as_healthy;
    let reason_present = input.throttle_quarantine_reason.is_some();
    let leaves_stale_certified_overclaim =
        input.certified_or_supported_claimed && !input.evidence_fresh;

    let degrade_reason = if input.artifact_identity.trim().is_empty() {
        Some(M5InstalledStateDiagnosticsCardDegradeReason::ArtifactIdentityUnstated)
    } else if matches!(
        input.budget_state,
        M5ActivationBudgetBandState::BudgetUnknown
    ) {
        Some(M5InstalledStateDiagnosticsCardDegradeReason::BudgetBandUnresolved)
    } else if matches!(input.quarantine_state, M5QuarantineState::QuarantineUnknown) {
        Some(M5InstalledStateDiagnosticsCardDegradeReason::QuarantineStateUnresolved)
    } else if input.activation_triggers.is_empty() {
        Some(M5InstalledStateDiagnosticsCardDegradeReason::ActivationTriggersUnstated)
    } else if input.exercised_capabilities.is_empty() {
        Some(M5InstalledStateDiagnosticsCardDegradeReason::ExercisedCapabilitiesUnstated)
    } else if hides_quarantine_history {
        Some(M5InstalledStateDiagnosticsCardDegradeReason::QuarantineHistoryHidden)
    } else if is_actionable && !reason_present {
        Some(M5InstalledStateDiagnosticsCardDegradeReason::ThrottleQuarantineReasonMissing)
    } else if is_actionable && !disable_retry_parity {
        Some(M5InstalledStateDiagnosticsCardDegradeReason::DisableRetryActionsMissing)
    } else if leaves_stale_certified_overclaim {
        Some(M5InstalledStateDiagnosticsCardDegradeReason::StaleEvidenceCertifiedOverclaim)
    } else if !input.proof_fresh {
        Some(M5InstalledStateDiagnosticsCardDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ActivationDiagnosticsNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedInstalledStateDiagnosticsCard {
        card_id: input.card_id,
        artifact_identity: input.artifact_identity,
        budget_state: input.budget_state.as_str().to_owned(),
        quarantine_state: input.quarantine_state.as_str().to_owned(),
        compatibility: input.compatibility.as_str().to_owned(),
        activation_triggers: input
            .activation_triggers
            .iter()
            .map(|t| t.as_str().to_owned())
            .collect(),
        exercised_capabilities: input
            .exercised_capabilities
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect(),
        throttle_quarantine_reason: input
            .throttle_quarantine_reason
            .map(|r| r.as_str().to_owned()),
        remediation_actions: input
            .remediation_actions
            .iter()
            .map(|a| a.as_str().to_owned())
            .collect(),
        is_actionable,
        offers_disable,
        offers_retry,
        disable_retry_parity,
        certified_or_supported_claimed: input.certified_or_supported_claimed,
        evidence_fresh: input.evidence_fresh,
        hides_quarantine_history,
        leaves_stale_certified_overclaim,
        degrade_reason,
        next_action,
        fully_legible: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved activation-budget band and
/// installed-state diagnostics card examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivationDiagnosticsControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ActivationDiagnosticsConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5MarketplaceInstallQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5MarketplaceInstallDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5MarketplaceInstallRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5MarketplaceInstallAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5ActivationDiagnosticsAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ActivationDiagnosticsExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5MarketplaceInstallDowngradeTrigger>,
    /// Resolved activation-budget band examples.
    pub activation_budget_band_examples: Vec<M5ResolvedActivationBudgetBand>,
    /// Resolved installed-state diagnostics card examples.
    pub installed_state_diagnostics_card_examples: Vec<M5ResolvedInstalledStateDiagnosticsCard>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hide activation cost or present an over-budget band as cost-free.
    pub hides_activation_cost_or_over_budget_band: bool,
    /// Hard invariant: never hide a throttling or quarantine reason behind a healthy card.
    pub hides_throttling_or_quarantine_reason: bool,
    /// Hard invariant: never collapse the disable / retry pair into one generic action.
    pub collapses_disable_and_retry_into_generic_action: bool,
    /// Hard invariant: never leave Certified / Supported language on stale evidence.
    pub leaves_stale_evidence_certified_or_supported: bool,
}

impl M5ActivationDiagnosticsControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ActivationDiagnosticsAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ActivationDiagnosticsAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ActivationDiagnosticsExportField> =
            self.export_fields.iter().copied().collect();
        M5ActivationDiagnosticsExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.hides_activation_cost_or_over_budget_band
            && !self.hides_throttling_or_quarantine_reason
            && !self.collapses_disable_and_retry_into_generic_action
            && !self.leaves_stale_evidence_certified_or_supported
    }

    /// True when every resolved example on this row is honest: no clean band presents an over-budget
    /// artifact as cost-free or leaves a stale-certified overclaim, and no clean card hides
    /// quarantine history, breaks the disable / retry pair while actionable, or leaves a
    /// stale-certified overclaim.
    fn examples_are_honest(&self) -> bool {
        self.activation_budget_band_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.presents_over_budget_as_cost_free || ex.leaves_stale_certified_overclaim))
        }) && self
            .installed_state_diagnostics_card_examples
            .iter()
            .all(|ex| {
                !(ex.is_clean()
                    && (ex.hides_quarantine_history
                        || (ex.is_actionable && !ex.disable_retry_parity)
                        || ex.leaves_stale_certified_overclaim))
            })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivationDiagnosticsVocabularySet {
    /// Marketplace / install-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Activation-budget band-state tokens (bound from the frozen matrix).
    pub activation_budget_band_states: Vec<String>,
    /// Quarantine-state tokens (bound from the frozen matrix).
    pub quarantine_states: Vec<String>,
    /// Compatibility-state tokens (bound from the frozen matrix).
    pub compatibility_states: Vec<String>,
    /// Activation-budget band-class tokens (minted by this lane).
    pub band_classes: Vec<String>,
    /// Activation cost-level tokens (minted by this lane).
    pub cost_levels: Vec<String>,
    /// Activation-trigger-class tokens (minted by this lane).
    pub activation_trigger_classes: Vec<String>,
    /// Exercised-capability-class tokens (minted by this lane).
    pub exercised_capability_classes: Vec<String>,
    /// Throttle / quarantine-reason tokens (minted by this lane).
    pub throttle_quarantine_reasons: Vec<String>,
    /// Remediation-action tokens (minted by this lane).
    pub remediation_actions: Vec<String>,
    /// Activation-budget-band degrade-reason tokens.
    pub activation_budget_band_degrade_reasons: Vec<String>,
    /// Installed-state-diagnostics-card degrade-reason tokens.
    pub installed_state_diagnostics_card_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ActivationDiagnosticsVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5MarketplaceInstallDisposition::ALL, |v| v.as_str()),
            activation_budget_band_states: tokens(&M5ActivationBudgetBandState::ALL, |v| {
                v.as_str()
            }),
            quarantine_states: tokens(&M5QuarantineState::ALL, |v| v.as_str()),
            compatibility_states: tokens(&M5CompatibilityState::ALL, |v| v.as_str()),
            band_classes: tokens(&M5ActivationBudgetBandClass::ALL, |v| v.as_str()),
            cost_levels: tokens(&M5ActivationCostLevel::ALL, |v| v.as_str()),
            activation_trigger_classes: tokens(&M5ActivationTriggerClass::ALL, |v| v.as_str()),
            exercised_capability_classes: tokens(&M5ExercisedCapabilityClass::ALL, |v| v.as_str()),
            throttle_quarantine_reasons: tokens(&M5ThrottleQuarantineReason::ALL, |v| v.as_str()),
            remediation_actions: tokens(&M5DiagnosticsRemediationAction::ALL, |v| v.as_str()),
            activation_budget_band_degrade_reasons: tokens(
                &M5ActivationBudgetBandDegradeReason::ALL,
                |v| v.as_str(),
            ),
            installed_state_diagnostics_card_degrade_reasons: tokens(
                &M5InstalledStateDiagnosticsCardDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5ActivationDiagnosticsAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ActivationDiagnosticsNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ActivationDiagnosticsExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5MarketplaceInstallConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5ActivationDiagnosticsGovernanceReview {
    /// The activation-budget band names its budget state, low / medium / high / over-budget class,
    /// and cold / warm evidence.
    pub band_names_budget_class_and_cold_warm_evidence: bool,
    /// An over-budget artifact is never presented as cost-free.
    pub over_budget_never_cost_free: bool,
    /// The diagnostics card names its activation triggers and exercised-capability summary.
    pub card_names_triggers_and_exercised_capabilities: bool,
    /// The diagnostics card names its throttling / quarantine reason where applicable.
    pub card_names_throttle_quarantine_reason_where_applicable: bool,
    /// A quarantined artifact is never presented as healthy; quarantine history stays explicit.
    pub quarantine_history_always_explicit: bool,
    /// The disable / retry action pair is always kept intact, never collapsed into one action.
    pub disable_retry_pair_always_intact: bool,
    /// Performance and quarantine implications are legible without digging into logs.
    pub implications_legible_without_logs: bool,
    /// Stale evidence never leaves Certified / Supported language in place.
    pub stale_evidence_never_leaves_certified_language: bool,
    /// Activation-budget and quarantine language stays aligned across marketplace, diagnostics,
    /// help, and support exports.
    pub budget_and_quarantine_language_aligned_across_surfaces: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivationDiagnosticsConsumerProjection {
    /// Marketplace surfaces consume the shared activation-budget vocabulary.
    pub marketplace_surfaces_consume_activation_budget_vocabulary: bool,
    /// Installed-state diagnostics surfaces consume the shared budget / quarantine vocabulary.
    pub diagnostics_surfaces_consume_budget_and_quarantine_vocabulary: bool,
    /// Activation-budget and quarantine facts trace back to one canonical component contract.
    pub facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical budget / quarantine source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivationDiagnosticsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivationDiagnosticsReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ActivationDiagnosticsControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ActivationDiagnosticsControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ActivationDiagnosticsControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ActivationDiagnosticsVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ActivationDiagnosticsGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ActivationDiagnosticsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ActivationDiagnosticsProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ActivationDiagnosticsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 activation-budget-band / installed-state-diagnostics-card controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivationDiagnosticsControlsPacket {
    /// Record kind; must equal [`M5_ACTIVATION_DIAGNOSTICS_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ACTIVATION_DIAGNOSTICS_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ActivationDiagnosticsControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ActivationDiagnosticsVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ActivationDiagnosticsGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ActivationDiagnosticsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ActivationDiagnosticsProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ActivationDiagnosticsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ActivationDiagnosticsControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5ActivationDiagnosticsControlsPacketInput) -> Self {
        Self {
            record_kind: M5_ACTIVATION_DIAGNOSTICS_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_ACTIVATION_DIAGNOSTICS_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5ActivationDiagnosticsControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ACTIVATION_DIAGNOSTICS_CONTROLS_RECORD_KIND {
            violations.push(M5ActivationDiagnosticsControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ACTIVATION_DIAGNOSTICS_CONTROLS_SCHEMA_VERSION {
            violations.push(M5ActivationDiagnosticsControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ActivationDiagnosticsControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5ActivationDiagnosticsControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 activation-diagnostics controls packet serializes"),
        ) {
            violations.push(M5ActivationDiagnosticsControlsViolation::RawMaterialInExport);
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
            .expect("m5 activation-diagnostics controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,band_examples,card_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .activation_budget_band_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.installed_state_diagnostics_card_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.activation_budget_band_examples.len(),
                row.installed_state_diagnostics_card_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Activation-Budget-Band and Installed-State-Diagnostics-Card Controls\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Activation-budget band states: {}\n",
            self.vocabulary_set.activation_budget_band_states.join(", ")
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
                "  - Activation-budget-band examples: {} / diagnostics-card examples: {}\n",
                row.activation_budget_band_examples.len(),
                row.installed_state_diagnostics_card_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5ActivationDiagnosticsControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ActivationDiagnosticsControlsViolation>),
}

impl fmt::Display for M5ActivationDiagnosticsControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 activation-diagnostics controls export parse failed: {error}"
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
                    "m5 activation-diagnostics controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ActivationDiagnosticsControlsArtifactError {}

/// Validation failures emitted by [`M5ActivationDiagnosticsControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ActivationDiagnosticsControlsViolation {
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
    /// A controls row carries a dishonest clean example (over-budget cost-free, hidden quarantine,
    /// broken disable/retry, or stale overclaim).
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
    /// Budget legibility is not proven: no clean band shows cold/warm evidence for an over-budget
    /// artifact, no over-budget-cost-free or evidence-missing band degrades, or a clean band leaves
    /// an over-budget-cost-free or stale overclaim in place.
    BudgetLegibilityNotProven,
    /// Quarantine reason and disable/retry parity are not proven: no clean card shows a throttle /
    /// quarantine reason with disable+retry parity for an actionable artifact, no quarantine-hidden,
    /// reason-missing, or disable-retry-missing card degrades, or a clean card hides quarantine or
    /// breaks the disable/retry pair.
    QuarantineReasonAndDisableRetryNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ActivationDiagnosticsControlsViolation {
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
            Self::BudgetLegibilityNotProven => "budget_legibility_not_proven",
            Self::QuarantineReasonAndDisableRetryNotProven => {
                "quarantine_reason_and_disable_retry_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_activation_diagnostics_controls_export(
) -> Result<M5ActivationDiagnosticsControlsPacket, M5ActivationDiagnosticsControlsArtifactError> {
    let packet: M5ActivationDiagnosticsControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-activation-budget-band-installed-state-diagnostics-card-controls-proof/support_export.json"
    )))
    .map_err(M5ActivationDiagnosticsControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ActivationDiagnosticsControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ActivationDiagnosticsControlsPacket,
    violations: &mut Vec<M5ActivationDiagnosticsControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ACTIVATION_DIAGNOSTICS_CONTROLS_SCHEMA_REF,
        M5_ACTIVATION_DIAGNOSTICS_CONTROLS_DOC_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
        M5_ACTIVATION_BUDGET_BAND_SCHEMA_REF,
        M5_INSTALLED_STATE_DIAGNOSTICS_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ActivationDiagnosticsControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5ActivationDiagnosticsControlsPacket,
    violations: &mut Vec<M5ActivationDiagnosticsControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5ActivationDiagnosticsControlsViolation::NoControlsRows);
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
            violations.push(M5ActivationDiagnosticsControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ActivationDiagnosticsControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ActivationDiagnosticsControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_ACTIVATION_BUDGET_BAND_SCHEMA_REF)
            || !refs.contains(M5_INSTALLED_STATE_DIAGNOSTICS_CARD_SCHEMA_REF)
        {
            violations.push(M5ActivationDiagnosticsControlsViolation::ComponentSchemaRefMissing);
        }
        if row.activation_budget_band_examples.is_empty()
            || row.installed_state_diagnostics_card_examples.is_empty()
        {
            violations.push(M5ActivationDiagnosticsControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5ActivationDiagnosticsControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5ActivationDiagnosticsControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ActivationDiagnosticsControlsPacket,
    violations: &mut Vec<M5ActivationDiagnosticsControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.band_names_budget_class_and_cold_warm_evidence,
        review.over_budget_never_cost_free,
        review.card_names_triggers_and_exercised_capabilities,
        review.card_names_throttle_quarantine_reason_where_applicable,
        review.quarantine_history_always_explicit,
        review.disable_retry_pair_always_intact,
        review.implications_legible_without_logs,
        review.stale_evidence_never_leaves_certified_language,
        review.budget_and_quarantine_language_aligned_across_surfaces,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5ActivationDiagnosticsControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ActivationDiagnosticsControlsPacket,
    violations: &mut Vec<M5ActivationDiagnosticsControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.marketplace_surfaces_consume_activation_budget_vocabulary,
        projection.diagnostics_surfaces_consume_budget_and_quarantine_vocabulary,
        projection.facts_trace_to_single_component_contract,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ActivationDiagnosticsControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ActivationDiagnosticsControlsPacket,
    violations: &mut Vec<M5ActivationDiagnosticsControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ActivationDiagnosticsControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ActivationDiagnosticsControlsPacket,
    violations: &mut Vec<M5ActivationDiagnosticsControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ActivationDiagnosticsControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ActivationDiagnosticsControlsPacket,
    violations: &mut Vec<M5ActivationDiagnosticsControlsViolation>,
) {
    let bands = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.activation_budget_band_examples.iter())
    };
    let cards = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.installed_state_diagnostics_card_examples.iter())
    };

    // AC: activation-budget evidence is legible before install and after runtime degradation. A
    // clean band covers an over-budget artifact still carrying cold / warm evidence, an
    // over-budget-cost-free band degrades, an evidence-missing-after-degradation band degrades, and
    // no clean band presents an over-budget artifact as cost-free or leaves a stale overclaim.
    let clean_over_budget_with_evidence =
        bands().any(|ex| ex.is_clean() && ex.is_over_budget && ex.has_cold_warm_evidence);
    let over_budget_cost_free_degrades = bands().any(|ex| {
        ex.degrade_reason == Some(M5ActivationBudgetBandDegradeReason::OverBudgetShownAsCostFree)
    });
    let evidence_missing_degrades = bands().any(|ex| {
        ex.degrade_reason
            == Some(M5ActivationBudgetBandDegradeReason::ActivationEvidenceMissingAfterDegradation)
    });
    let no_clean_cost_free_or_overclaim = bands().all(|ex| {
        !(ex.is_clean()
            && (ex.presents_over_budget_as_cost_free || ex.leaves_stale_certified_overclaim))
    });
    if !(clean_over_budget_with_evidence
        && over_budget_cost_free_degrades
        && evidence_missing_degrades
        && no_clean_cost_free_or_overclaim)
    {
        violations.push(M5ActivationDiagnosticsControlsViolation::BudgetLegibilityNotProven);
    }

    // AC: users can see performance and quarantine implications without digging into logs, and the
    // disable / retry pair stays intact. A clean card covers a throttled / quarantined artifact
    // carrying a reason and the disable+retry pair, a quarantine-hidden card degrades, a
    // reason-missing card degrades, a disable-retry-missing card degrades, and no clean card hides
    // quarantine history or breaks the disable / retry pair while actionable.
    let clean_actionable_with_reason_and_parity = cards().any(|ex| {
        ex.is_clean()
            && ex.is_actionable
            && ex.throttle_quarantine_reason.is_some()
            && ex.disable_retry_parity
    });
    let quarantine_hidden_degrades = cards().any(|ex| {
        ex.degrade_reason
            == Some(M5InstalledStateDiagnosticsCardDegradeReason::QuarantineHistoryHidden)
    });
    let reason_missing_degrades = cards().any(|ex| {
        ex.degrade_reason
            == Some(M5InstalledStateDiagnosticsCardDegradeReason::ThrottleQuarantineReasonMissing)
    });
    let disable_retry_missing_degrades = cards().any(|ex| {
        ex.degrade_reason
            == Some(M5InstalledStateDiagnosticsCardDegradeReason::DisableRetryActionsMissing)
    });
    let no_clean_hidden_or_broken = cards().all(|ex| {
        !(ex.is_clean()
            && (ex.hides_quarantine_history || (ex.is_actionable && !ex.disable_retry_parity)))
    });
    if !(clean_actionable_with_reason_and_parity
        && quarantine_hidden_degrades
        && reason_missing_degrades
        && disable_retry_missing_degrades
        && no_clean_hidden_or_broken)
    {
        violations.push(
            M5ActivationDiagnosticsControlsViolation::QuarantineReasonAndDisableRetryNotProven,
        );
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
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

/// The two component families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5MarketplaceInstallComponentFamily; 2] = [
    M5MarketplaceInstallComponentFamily::ActivationBudgetBand,
    M5MarketplaceInstallComponentFamily::InstalledStateDiagnosticsCard,
];
