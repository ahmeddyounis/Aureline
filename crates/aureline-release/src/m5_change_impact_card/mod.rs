//! Typed change-impact cards — the per-dimension forecast a user, team lead, admin, or support
//! reviewer reads *before restart* to see what a staged M5 update will actually change, on top of the
//! [typed update-center summary objects](crate::m5_update_summary).
//!
//! The update-center summary answers "what is changing, and did it verify"; this lane answers the
//! harder question the exit-gate anchor calls out: *what will this update do to my workspace,
//! profile, schema, caches, extensions, remote helpers, and toolchain once I restart* — surfaced
//! before the restart commits it. Each forecast [dimension](ImpactDimension) gets its own
//! [card](ChangeImpactCard) rather than being collapsed into one "an update is available" line, so a
//! reviewer can see, per dimension:
//!
//! - a [risk class](RiskClass) that deliberately distinguishes low-risk cache churn from a
//!   destructive or habit-breaking behavior change;
//! - a [forecast confidence](ForecastConfidence) that labels unknown inputs and partial coverage
//!   honestly — a high-risk forecast made on *speculative* inputs is flagged for review, never raised
//!   as a hard failure (the lane's guardrail, enforced by [`ChangeImpactCardSet::validate`]);
//! - the affected artifact classes and deployment profiles, so the disclosed scope is never narrower
//!   than what the update touches;
//! - a manual [follow-up task](FollowUpTask) with its timing and automation; and
//! - a [rollback or pin choice](RollbackChoice), so a reviewer always sees the recovery path.
//!
//! The [consumer surfaces](ImpactConsumer) — update center, migration assistant, release center,
//! team-lead review, admin console, support export — bind the dimensions they read and *derive* their
//! [review readiness](ReviewReadiness) and gaps from the cards, so all of them read this one
//! [`ChangeImpactCardSet`] packet rather than cloning risk fields locally. A card that needs review
//! narrows every consumer that reads it; a *confirmed* destructive card holds those consumers for a
//! pre-restart acknowledgement, while a *speculative* one only recommends review.
//!
//! The packet is inspectable and serde-serializable; it carries metadata, refs, and message ids only
//! — no credential bodies or raw provider payloads — so the impact summary is exportable and
//! reviewable outside the app without forcing an immediate restart.
//!
//! - Packet schema:
//!   [`schemas/release/m5-change-impact-card.schema.json`](../../../../../schemas/release/m5-change-impact-card.schema.json)
//! - Contract doc:
//!   [`docs/release/m5-change-impact-card-contract.md`](../../../../../docs/release/m5-change-impact-card-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_change_impact_card_set, seeded_m5_change_impact_card_set_hold,
    seeded_m5_change_impact_card_set_review, seeded_m5_change_impact_card_set_speculative,
    M5_CHANGE_IMPACT_CARD_SET_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The change-impact cards reuse the update / support-lifecycle governance vocabularies for artifact
// class, channel, and deployment profile, and the descriptor / badge runtime's gate / status / signal
// vocabulary, so this forecast layer can never drift to a different vocabulary than the layers above.
use crate::m5_descriptor_badge::{ConsumerStatus, DescriptorGate, DescriptorSignal};
use crate::m5_update_lifecycle::{ArtifactClass, ChannelScope, DeploymentProfile};

/// Record-kind tag carried by [`ChangeImpactCardSet`].
pub const M5_CHANGE_IMPACT_CARD_SET_RECORD_KIND: &str = "m5_change_impact_card_set";

/// Schema version for the change-impact card-set packet.
pub const M5_CHANGE_IMPACT_CARD_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the card-set packet schema.
pub const M5_CHANGE_IMPACT_CARD_SCHEMA_REF: &str =
    "schemas/release/m5-change-impact-card.schema.json";

/// Repo-relative path of the published card-set inventory.
pub const M5_CHANGE_IMPACT_CARD_SET_REF: &str = "artifacts/release/m5-change-impact-cards.json";

/// Repo-relative path of the release-grade card-set parity proof.
pub const M5_CHANGE_IMPACT_CARD_SET_PROOF_REF: &str =
    "artifacts/release/m5-change-impact-proof/change-impact-cards.json";

/// Repo-relative path of the machine-readable per-card risk export.
pub const M5_CHANGE_IMPACT_CARD_SET_CSV_REF: &str = "artifacts/release/m5-change-impact-cards.csv";

/// Repo-relative path of the card-set contract doc.
pub const M5_CHANGE_IMPACT_CARD_SET_DOC_REF: &str =
    "docs/release/m5-change-impact-card-contract.md";

/// Repo-relative directory of the per-state card-set fixtures.
pub const M5_CHANGE_IMPACT_CARD_SET_FIXTURE_DIR: &str = "fixtures/release/change-impact/";

/// Prefix every change-impact message id carries so consumers can route it.
pub const M5_CHANGE_IMPACT_MESSAGE_ID_PREFIX: &str = "release_change_impact.";

const REDACTION_CLASS: &str = "metadata_safe_default";

// ---------------------------------------------------------------------------
// Controlled vocabularies
// ---------------------------------------------------------------------------

/// One forecast dimension a change-impact card covers. The set is the union of the impact areas the
/// lane forecasts before restart; each is carded once so an update never collapses unrelated impact
/// into a single generic row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactDimension {
    /// Persisted workspace-state migration.
    WorkspaceMigration,
    /// Profile / settings migration.
    ProfileMigration,
    /// Schema / contract compatibility migration.
    SchemaMigration,
    /// Cache rebuild / churn.
    CacheMigration,
    /// Installed-extension compatibility risk.
    ExtensionCompatibility,
    /// Remote-helper version skew.
    RemoteHelperSkew,
    /// Toolchain minimum (floor) change.
    ToolchainFloor,
    /// Toolchain maximum (ceiling) change.
    ToolchainCeiling,
    /// Certified-archetype behavior risk.
    CertifiedArchetype,
    /// Habit-breaking / destructive behavior change.
    BehaviorChange,
}

impl ImpactDimension {
    /// Every forecast dimension, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::WorkspaceMigration,
        Self::ProfileMigration,
        Self::SchemaMigration,
        Self::CacheMigration,
        Self::ExtensionCompatibility,
        Self::RemoteHelperSkew,
        Self::ToolchainFloor,
        Self::ToolchainCeiling,
        Self::CertifiedArchetype,
        Self::BehaviorChange,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceMigration => "workspace_migration",
            Self::ProfileMigration => "profile_migration",
            Self::SchemaMigration => "schema_migration",
            Self::CacheMigration => "cache_migration",
            Self::ExtensionCompatibility => "extension_compatibility",
            Self::RemoteHelperSkew => "remote_helper_skew",
            Self::ToolchainFloor => "toolchain_floor",
            Self::ToolchainCeiling => "toolchain_ceiling",
            Self::CertifiedArchetype => "certified_archetype",
            Self::BehaviorChange => "behavior_change",
        }
    }

    /// Human-facing label for the dimension.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceMigration => "Workspace migration",
            Self::ProfileMigration => "Profile migration",
            Self::SchemaMigration => "Schema migration",
            Self::CacheMigration => "Cache migration",
            Self::ExtensionCompatibility => "Extension compatibility",
            Self::RemoteHelperSkew => "Remote-helper skew",
            Self::ToolchainFloor => "Toolchain floor",
            Self::ToolchainCeiling => "Toolchain ceiling",
            Self::CertifiedArchetype => "Certified archetype",
            Self::BehaviorChange => "Behavior change",
        }
    }

    /// The primary artifact class this dimension forecasts impact on. A card may touch other classes
    /// too; the disclosed set is always the union of the card's affected classes plus this primary.
    pub const fn primary_artifact_class(self) -> ArtifactClass {
        match self {
            Self::WorkspaceMigration => ArtifactClass::WorkspaceState,
            Self::ProfileMigration => ArtifactClass::Configuration,
            Self::SchemaMigration => ArtifactClass::SchemaContracts,
            Self::CacheMigration => ArtifactClass::CoreRuntime,
            Self::ExtensionCompatibility => ArtifactClass::ExtensionPacks,
            Self::RemoteHelperSkew => ArtifactClass::CoreRuntime,
            Self::ToolchainFloor => ArtifactClass::LanguageRuntimes,
            Self::ToolchainCeiling => ArtifactClass::LanguageRuntimes,
            Self::CertifiedArchetype => ArtifactClass::WorkspaceState,
            Self::BehaviorChange => ArtifactClass::CoreRuntime,
        }
    }

    /// Accountable owner role for this dimension's forecast.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::WorkspaceMigration => "workspace_state_owner",
            Self::ProfileMigration => "profile_owner",
            Self::SchemaMigration => "schema_owner",
            Self::CacheMigration => "cache_owner",
            Self::ExtensionCompatibility => "extension_owner",
            Self::RemoteHelperSkew => "remote_helper_owner",
            Self::ToolchainFloor => "toolchain_owner",
            Self::ToolchainCeiling => "toolchain_owner",
            Self::CertifiedArchetype => "certification_owner",
            Self::BehaviorChange => "product_behavior_owner",
        }
    }
}

/// The risk class a card assigns to a forecast change. Declaration order is least→most severe; the
/// vocabulary deliberately separates [`LowRiskCacheChurn`](Self::LowRiskCacheChurn) from
/// [`HabitBreakingBehaviorChange`](Self::HabitBreakingBehaviorChange) and
/// [`DestructiveChange`](Self::DestructiveChange) so a routine cache rebuild can never read like a
/// destructive change, and vice versa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskClass {
    /// No impact is forecast for this dimension.
    NoImpact,
    /// Low-risk cache churn: caches rebuild, no state or behavior change.
    LowRiskCacheChurn,
    /// Compatible, but the user should be warned of a non-breaking change.
    CompatibleWithWarning,
    /// A migration is required before the change is complete.
    MigrationRequired,
    /// A user-facing behavior or habit changes.
    HabitBreakingBehaviorChange,
    /// A destructive change: irreversible or data-affecting without a follow-up.
    DestructiveChange,
}

impl RiskClass {
    /// Every risk class, least→most severe.
    pub const ALL: [Self; 6] = [
        Self::NoImpact,
        Self::LowRiskCacheChurn,
        Self::CompatibleWithWarning,
        Self::MigrationRequired,
        Self::HabitBreakingBehaviorChange,
        Self::DestructiveChange,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoImpact => "no_impact",
            Self::LowRiskCacheChurn => "low_risk_cache_churn",
            Self::CompatibleWithWarning => "compatible_with_warning",
            Self::MigrationRequired => "migration_required",
            Self::HabitBreakingBehaviorChange => "habit_breaking_behavior_change",
            Self::DestructiveChange => "destructive_change",
        }
    }

    /// True only for the one risk class that is a routine, non-destructive cache rebuild.
    pub const fn is_low_risk_cache_churn(self) -> bool {
        matches!(self, Self::LowRiskCacheChurn)
    }

    /// True for the risk classes that change habits or destroy / rewrite state.
    pub const fn is_destructive_or_habit_breaking(self) -> bool {
        matches!(
            self,
            Self::HabitBreakingBehaviorChange | Self::DestructiveChange
        )
    }

    /// The gate this risk class implies *assuming the forecast is certain*. A card's effective gate is
    /// this gate capped by the [forecast confidence](ForecastConfidence), so a speculative high-risk
    /// forecast can never become a hard failure.
    pub const fn risk_gate(self) -> DescriptorGate {
        match self {
            Self::NoImpact | Self::LowRiskCacheChurn => DescriptorGate::Governed,
            Self::CompatibleWithWarning
            | Self::MigrationRequired
            | Self::HabitBreakingBehaviorChange => DescriptorGate::Narrowed,
            Self::DestructiveChange => DescriptorGate::Blocked,
        }
    }
}

/// How confident the forecast is, given the inputs Aureline actually has. Declaration order is
/// best→worst. The lane's guardrail lives here: an [`Unknown`](Self::Unknown) or
/// [`Estimated`](Self::Estimated) forecast caps the card's gate at narrowed, so speculation is labeled
/// honestly and never raised as a hard pre-restart failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForecastConfidence {
    /// Inputs are fully available; the forecast is certain.
    Confirmed,
    /// Inputs are mostly available; the forecast is well-supported.
    Likely,
    /// Inputs are partial; the forecast is an estimate.
    Estimated,
    /// Inputs are unavailable; the impact cannot be forecast and is labeled as such.
    Unknown,
    /// The dimension does not apply to this update.
    NotApplicable,
}

impl ForecastConfidence {
    /// Every confidence level, best→worst.
    pub const ALL: [Self; 5] = [
        Self::Confirmed,
        Self::Likely,
        Self::Estimated,
        Self::Unknown,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",
            Self::Likely => "likely",
            Self::Estimated => "estimated",
            Self::Unknown => "unknown",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the forecast rests on partial or absent inputs. Speculative forecasts are flagged for
    /// review, never raised as hard failures.
    pub const fn is_speculative(self) -> bool {
        matches!(self, Self::Estimated | Self::Unknown)
    }

    /// The most severe gate this confidence allows. The card's effective gate is the *less severe* of
    /// the risk gate and this cap, so unknown / estimated inputs cap at narrowed and a not-applicable
    /// dimension caps at governed.
    pub const fn gate_cap(self) -> DescriptorGate {
        match self {
            Self::Confirmed | Self::Likely => DescriptorGate::Blocked,
            Self::Estimated | Self::Unknown => DescriptorGate::Narrowed,
            Self::NotApplicable => DescriptorGate::Governed,
        }
    }
}

/// The review readiness a card or consumer resolves to. A direct, one-to-one reading of a
/// [`DescriptorGate`] in pre-restart language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewReadiness {
    /// No blocking or narrowing impact; clear to apply before restart.
    ClearToApply,
    /// At least one narrowing impact; review is recommended before restart.
    ReviewRecommended,
    /// At least one confirmed destructive impact; hold for a pre-restart acknowledgement.
    HoldForResolution,
}

impl ReviewReadiness {
    /// Every readiness, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ClearToApply,
        Self::ReviewRecommended,
        Self::HoldForResolution,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClearToApply => "clear_to_apply",
            Self::ReviewRecommended => "review_recommended",
            Self::HoldForResolution => "hold_for_resolution",
        }
    }

    /// The readiness a gate resolves to.
    pub const fn from_gate(gate: DescriptorGate) -> Self {
        match gate {
            DescriptorGate::Governed => Self::ClearToApply,
            DescriptorGate::Narrowed => Self::ReviewRecommended,
            DescriptorGate::Blocked => Self::HoldForResolution,
        }
    }
}

/// The class of manual follow-up a card forecasts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowUpTaskClass {
    /// No follow-up is required.
    NoTaskRequired,
    /// Review the change before restart.
    ReviewBeforeRestart,
    /// Back up state before applying.
    BackupBeforeApply,
    /// Run a migration scan.
    MigrationScanRequired,
    /// Update an extension's supported range.
    ExtensionRangeUpdate,
    /// Upgrade a remote helper to clear skew.
    RemoteHelperUpgrade,
    /// Install or update a toolchain.
    ToolchainInstall,
    /// Rebuild caches.
    CacheRebuild,
    /// Acknowledge a policy change.
    PolicyAcknowledgement,
    /// An administrator must approve before apply.
    AdminApprovalRequired,
}

impl FollowUpTaskClass {
    /// Every follow-up task class, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::NoTaskRequired,
        Self::ReviewBeforeRestart,
        Self::BackupBeforeApply,
        Self::MigrationScanRequired,
        Self::ExtensionRangeUpdate,
        Self::RemoteHelperUpgrade,
        Self::ToolchainInstall,
        Self::CacheRebuild,
        Self::PolicyAcknowledgement,
        Self::AdminApprovalRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoTaskRequired => "no_task_required",
            Self::ReviewBeforeRestart => "review_before_restart",
            Self::BackupBeforeApply => "backup_before_apply",
            Self::MigrationScanRequired => "migration_scan_required",
            Self::ExtensionRangeUpdate => "extension_range_update",
            Self::RemoteHelperUpgrade => "remote_helper_upgrade",
            Self::ToolchainInstall => "toolchain_install",
            Self::CacheRebuild => "cache_rebuild",
            Self::PolicyAcknowledgement => "policy_acknowledgement",
            Self::AdminApprovalRequired => "admin_approval_required",
        }
    }
}

/// When a follow-up task must happen relative to apply / restart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskTiming {
    /// No task, so no timing.
    NotRequired,
    /// Before the update is applied.
    BeforeApply,
    /// Before the restart that activates the update.
    BeforeRestart,
    /// After the restart.
    AfterRestart,
    /// Before the next update.
    BeforeNextUpdate,
    /// A manual review only, with no fixed timing.
    ManualReviewOnly,
}

impl TaskTiming {
    /// Every timing, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotRequired,
        Self::BeforeApply,
        Self::BeforeRestart,
        Self::AfterRestart,
        Self::BeforeNextUpdate,
        Self::ManualReviewOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::BeforeApply => "before_apply",
            Self::BeforeRestart => "before_restart",
            Self::AfterRestart => "after_restart",
            Self::BeforeNextUpdate => "before_next_update",
            Self::ManualReviewOnly => "manual_review_only",
        }
    }
}

/// How much of a follow-up task Aureline can automate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskAutomation {
    /// No task, so automation does not apply.
    NotApplicable,
    /// Fully automatic; no user action.
    Automatic,
    /// An assistant can drive it with user confirmation.
    AssistantAvailable,
    /// The user must perform manual steps.
    ManualStepsRequired,
    /// An administrator must act.
    AdminActionRequired,
}

impl TaskAutomation {
    /// Every automation level, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotApplicable,
        Self::Automatic,
        Self::AssistantAvailable,
        Self::ManualStepsRequired,
        Self::AdminActionRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Automatic => "automatic",
            Self::AssistantAvailable => "assistant_available",
            Self::ManualStepsRequired => "manual_steps_required",
            Self::AdminActionRequired => "admin_action_required",
        }
    }
}

/// The rollback or pin choice a card discloses, so a reviewer always sees the recovery path before
/// restart. The kinds are distinct so a card never implies a true version rollback when only a pin,
/// a side-by-side fallback, or a reinstall remains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackChoice {
    /// No recovery choice applies (nothing changes).
    NotApplicable,
    /// A true rollback to the prior version is supported.
    RollbackSupported,
    /// The current version can be pinned to defer the change.
    PinCurrentVersion,
    /// The prior version coexists; the user can fall back side-by-side.
    SideBySideFallback,
    /// Recovering the prior state requires a reinstall.
    ReinstallOnly,
    /// No rollback, pin, or fallback is available.
    NoRollback,
}

impl RollbackChoice {
    /// Every rollback choice, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotApplicable,
        Self::RollbackSupported,
        Self::PinCurrentVersion,
        Self::SideBySideFallback,
        Self::ReinstallOnly,
        Self::NoRollback,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::RollbackSupported => "rollback_supported",
            Self::PinCurrentVersion => "pin_current_version",
            Self::SideBySideFallback => "side_by_side_fallback",
            Self::ReinstallOnly => "reinstall_only",
            Self::NoRollback => "no_rollback",
        }
    }

    /// True when the user has *some* way to defer or recover (rollback, pin, or side-by-side fallback).
    pub const fn offers_recovery(self) -> bool {
        matches!(
            self,
            Self::RollbackSupported | Self::PinCurrentVersion | Self::SideBySideFallback
        )
    }
}

/// The named cause of a consumer's review gap on one dimension it reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactGapKind {
    /// A read card narrows on a confirmed but non-destructive change; review recommended.
    ReviewRecommended,
    /// A read card narrows because its inputs are speculative; review recommended, not a failure.
    ForecastInputUnknown,
    /// A read card is a confirmed destructive change; resolve before restart.
    ResolveBeforeRestart,
    /// A dimension the consumer reads is not forecast in the card set.
    DimensionNotForecast,
}

impl ImpactGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReviewRecommended,
        Self::ForecastInputUnknown,
        Self::ResolveBeforeRestart,
        Self::DimensionNotForecast,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewRecommended => "review_recommended",
            Self::ForecastInputUnknown => "forecast_input_unknown",
            Self::ResolveBeforeRestart => "resolve_before_restart",
            Self::DimensionNotForecast => "dimension_not_forecast",
        }
    }

    /// The gate this gap forces.
    const fn gate(self) -> DescriptorGate {
        match self {
            Self::ReviewRecommended | Self::ForecastInputUnknown => DescriptorGate::Narrowed,
            Self::ResolveBeforeRestart | Self::DimensionNotForecast => DescriptorGate::Blocked,
        }
    }
}

/// One claimed consumer surface that reads the change-impact cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactConsumer {
    /// The in-product update center's pre-restart surface.
    UpdateCenter,
    /// The migration assistant.
    MigrationAssistant,
    /// The release center / public-truth automation.
    ReleaseCenter,
    /// The exported team-lead review surface.
    TeamLeadReview,
    /// The admin console.
    AdminConsole,
    /// The support export.
    SupportExport,
}

impl ImpactConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UpdateCenter,
        Self::MigrationAssistant,
        Self::ReleaseCenter,
        Self::TeamLeadReview,
        Self::AdminConsole,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpdateCenter => "update_center",
            Self::MigrationAssistant => "migration_assistant",
            Self::ReleaseCenter => "release_center",
            Self::TeamLeadReview => "team_lead_review",
            Self::AdminConsole => "admin_console",
            Self::SupportExport => "support_export",
        }
    }

    /// Human-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UpdateCenter => "Update center",
            Self::MigrationAssistant => "Migration assistant",
            Self::ReleaseCenter => "Release center",
            Self::TeamLeadReview => "Team-lead review",
            Self::AdminConsole => "Admin console",
            Self::SupportExport => "Support export",
        }
    }

    /// Accountable owner role.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::UpdateCenter => "update_center_owner",
            Self::MigrationAssistant => "migration_assistant_owner",
            Self::ReleaseCenter => "release_center_owner",
            Self::TeamLeadReview => "team_lead_owner",
            Self::AdminConsole => "admin_console_owner",
            Self::SupportExport => "support_export_owner",
        }
    }
}

// ---------------------------------------------------------------------------
// Ranking helpers for deterministic ordering
// ---------------------------------------------------------------------------

fn dimension_rank(d: ImpactDimension) -> usize {
    ImpactDimension::ALL
        .iter()
        .position(|x| *x == d)
        .unwrap_or(usize::MAX)
}

fn artifact_rank(c: ArtifactClass) -> usize {
    ArtifactClass::ALL
        .iter()
        .position(|x| *x == c)
        .unwrap_or(usize::MAX)
}

fn profile_rank(p: DeploymentProfile) -> usize {
    DeploymentProfile::ALL
        .iter()
        .position(|x| *x == p)
        .unwrap_or(usize::MAX)
}

fn consumer_rank(c: ImpactConsumer) -> usize {
    ImpactConsumer::ALL
        .iter()
        .position(|x| *x == c)
        .unwrap_or(usize::MAX)
}

fn gate_rank(g: DescriptorGate) -> u8 {
    match g {
        DescriptorGate::Governed => 0,
        DescriptorGate::Narrowed => 1,
        DescriptorGate::Blocked => 2,
    }
}

fn worst_gate(a: DescriptorGate, b: DescriptorGate) -> DescriptorGate {
    if gate_rank(a) >= gate_rank(b) {
        a
    } else {
        b
    }
}

/// Caps a gate at `cap`: returns the *less severe* of the two. This is how a speculative forecast
/// confidence prevents a high-risk card from becoming a hard failure.
fn cap_gate(gate: DescriptorGate, cap: DescriptorGate) -> DescriptorGate {
    if gate_rank(gate) <= gate_rank(cap) {
        gate
    } else {
        cap
    }
}

fn status_for_gate(gate: DescriptorGate) -> ConsumerStatus {
    match gate {
        DescriptorGate::Governed => ConsumerStatus::Mapped,
        DescriptorGate::Narrowed => ConsumerStatus::Provisional,
        DescriptorGate::Blocked => ConsumerStatus::Unmapped,
    }
}

fn signal_for_gate(gate: DescriptorGate) -> DescriptorSignal {
    match gate {
        DescriptorGate::Governed => DescriptorSignal::Green,
        DescriptorGate::Narrowed => DescriptorSignal::Yellow,
        DescriptorGate::Blocked => DescriptorSignal::Red,
    }
}

// ---------------------------------------------------------------------------
// Follow-up task
// ---------------------------------------------------------------------------

/// The manual follow-up a card forecasts: its class, timing, automation, and any task refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowUpTask {
    /// The class of follow-up.
    pub task_class: FollowUpTaskClass,
    /// When the task must happen.
    pub timing: TaskTiming,
    /// How much of the task is automatable.
    pub automation: TaskAutomation,
    /// Opaque refs to task guidance (no raw payloads).
    pub task_refs: Vec<String>,
    /// Routable message id for the task's summary.
    pub task_message_id: String,
}

impl FollowUpTask {
    /// A no-op follow-up: nothing required.
    pub fn none(dimension: ImpactDimension) -> Self {
        Self::new(
            dimension,
            FollowUpTaskClass::NoTaskRequired,
            TaskTiming::NotRequired,
            TaskAutomation::NotApplicable,
            &[],
        )
    }

    /// Builds a follow-up task with the given class, timing, automation, and refs.
    pub fn new(
        dimension: ImpactDimension,
        task_class: FollowUpTaskClass,
        timing: TaskTiming,
        automation: TaskAutomation,
        task_refs: &[&str],
    ) -> Self {
        Self {
            task_class,
            timing,
            automation,
            task_refs: task_refs.iter().map(|s| (*s).to_owned()).collect(),
            task_message_id: format!(
                "{}card.{}.follow_up",
                M5_CHANGE_IMPACT_MESSAGE_ID_PREFIX,
                dimension.as_str()
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// Change-impact card (per dimension)
// ---------------------------------------------------------------------------

/// Builder input for [`ChangeImpactCard::new`].
#[derive(Debug, Clone)]
pub struct ChangeImpactCardInput {
    /// The forecast dimension this card covers.
    pub dimension: ImpactDimension,
    /// The forecast risk class.
    pub risk_class: RiskClass,
    /// The forecast confidence.
    pub confidence: ForecastConfidence,
    /// Artifact classes the change affects (the primary class is always added).
    pub affected_artifact_classes: Vec<ArtifactClass>,
    /// Deployment profiles the change affects.
    pub affected_profiles: Vec<DeploymentProfile>,
    /// The version the dimension moves from (absent when unknown / not applicable).
    pub from_version: Option<String>,
    /// The version the dimension moves to (absent when unknown / not applicable).
    pub to_version: Option<String>,
    /// The manual follow-up the change forecasts.
    pub follow_up: FollowUpTask,
    /// The rollback or pin choice the change offers.
    pub rollback_choice: RollbackChoice,
    /// Opaque evidence refs backing the forecast (no raw payloads).
    pub evidence_refs: Vec<String>,
}

/// The typed change-impact forecast for one [dimension](ImpactDimension): the risk class, the
/// forecast confidence, the affected scope, the follow-up task, the rollback / pin choice, and the
/// derived review verdict. The card's gate is the [risk gate](RiskClass::risk_gate) *capped* by the
/// [confidence](ForecastConfidence::gate_cap), so a speculative high-risk forecast is flagged for
/// review rather than raised as a hard failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeImpactCard {
    /// The forecast dimension.
    pub dimension: ImpactDimension,
    /// Human-facing label.
    pub dimension_label: String,
    /// The dimension's primary artifact class.
    pub primary_artifact_class: ArtifactClass,
    /// Accountable owner role.
    pub owner_role: String,
    /// The forecast risk class.
    pub risk_class: RiskClass,
    /// The forecast confidence.
    pub confidence: ForecastConfidence,
    /// True when the forecast rests on partial or absent inputs.
    pub speculative: bool,
    /// The union of artifact classes the change affects (always includes the primary class).
    pub affected_artifact_classes: Vec<ArtifactClass>,
    /// The deployment profiles the change affects.
    pub affected_profiles: Vec<DeploymentProfile>,
    /// The version the dimension moves from.
    pub from_version: Option<String>,
    /// The version the dimension moves to.
    pub to_version: Option<String>,
    /// The manual follow-up the change forecasts.
    pub follow_up: FollowUpTask,
    /// The rollback or pin choice the change offers.
    pub rollback_choice: RollbackChoice,
    /// Opaque evidence refs backing the forecast.
    pub evidence_refs: Vec<String>,
    /// Set only when the forecast is speculative: a routable message id naming the unknown / partial
    /// inputs, so the gap is labeled honestly rather than presented as a failure.
    pub unknown_input_message_id: Option<String>,
    /// Gate derived from the risk class capped by the confidence.
    pub gate: DescriptorGate,
    /// Review readiness mirroring [`gate`](Self::gate).
    pub review_readiness: ReviewReadiness,
    /// Coverage status mirroring [`gate`](Self::gate).
    pub status: ConsumerStatus,
    /// Traffic-light signal mirroring [`gate`](Self::gate).
    pub signal: DescriptorSignal,
    /// True only when the card is a confirmed destructive change that must be acknowledged before
    /// restart. A speculative destructive forecast is false here.
    pub requires_pre_restart_acknowledgement: bool,
    /// Routable message id for the card's summary line.
    pub summary_message_id: String,
    /// Routable message id for the card's detail.
    pub detail_message_id: String,
}

impl ChangeImpactCard {
    /// Builds a card from its inputs, deriving the gate, readiness, speculative flag, and disclosed
    /// scope.
    pub fn new(input: ChangeImpactCardInput) -> Self {
        let dimension = input.dimension;
        let mut card = Self {
            dimension,
            dimension_label: dimension.label().to_owned(),
            primary_artifact_class: dimension.primary_artifact_class(),
            owner_role: dimension.owner_role().to_owned(),
            risk_class: input.risk_class,
            confidence: input.confidence,
            speculative: false,
            affected_artifact_classes: input.affected_artifact_classes,
            affected_profiles: input.affected_profiles,
            from_version: input.from_version,
            to_version: input.to_version,
            follow_up: input.follow_up,
            rollback_choice: input.rollback_choice,
            evidence_refs: input.evidence_refs,
            unknown_input_message_id: None,
            gate: DescriptorGate::Governed,
            review_readiness: ReviewReadiness::ClearToApply,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            requires_pre_restart_acknowledgement: false,
            summary_message_id: format!(
                "{}card.{}.summary",
                M5_CHANGE_IMPACT_MESSAGE_ID_PREFIX,
                dimension.as_str()
            ),
            detail_message_id: format!(
                "{}card.{}.detail",
                M5_CHANGE_IMPACT_MESSAGE_ID_PREFIX,
                dimension.as_str()
            ),
        };
        card.recompute();
        card
    }

    /// Recomputes the disclosed scope and derived verdict from the card's inputs. The disclosed
    /// artifact classes are the union of the affected classes plus the primary class; the gate is the
    /// risk gate capped by the confidence; the speculative flag and unknown-input message id come from
    /// the confidence.
    pub fn recompute(&mut self) {
        let mut classes = vec![self.primary_artifact_class];
        classes.extend(self.affected_artifact_classes.iter().copied());
        classes.sort_by_key(|c| artifact_rank(*c));
        classes.dedup();
        self.affected_artifact_classes = classes;

        let mut profiles = self.affected_profiles.clone();
        profiles.sort_by_key(|p| profile_rank(*p));
        profiles.dedup();
        self.affected_profiles = profiles;

        self.speculative = self.confidence.is_speculative();
        self.unknown_input_message_id = if self.speculative {
            Some(format!(
                "{}card.{}.unknown_input",
                M5_CHANGE_IMPACT_MESSAGE_ID_PREFIX,
                self.dimension.as_str()
            ))
        } else {
            None
        };

        let gate = cap_gate(self.risk_class.risk_gate(), self.confidence.gate_cap());
        self.gate = gate;
        self.review_readiness = ReviewReadiness::from_gate(gate);
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.requires_pre_restart_acknowledgement = gate == DescriptorGate::Blocked;
    }

    /// The gap kind this card contributes to a consumer that reads it, if any.
    fn gap_kind(&self) -> Option<ImpactGapKind> {
        match self.gate {
            DescriptorGate::Governed => None,
            DescriptorGate::Narrowed => Some(if self.speculative {
                ImpactGapKind::ForecastInputUnknown
            } else {
                ImpactGapKind::ReviewRecommended
            }),
            DescriptorGate::Blocked => Some(ImpactGapKind::ResolveBeforeRestart),
        }
    }
}

// ---------------------------------------------------------------------------
// Consumer rows
// ---------------------------------------------------------------------------

/// A review gap a consumer carries for one dimension it reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactGap {
    /// The consumer that carries the gap.
    pub consumer: ImpactConsumer,
    /// The dimension whose card caused the gap.
    pub dimension: ImpactDimension,
    /// The dimension's primary artifact class.
    pub artifact_class: ArtifactClass,
    /// The named cause of the gap.
    pub gap_kind: ImpactGapKind,
    /// Routable message id naming the cause.
    pub cause_message_id: String,
}

/// A consumer surface bound to the dimensions it reads, with its review readiness, decision, and gaps
/// derived from those dimensions' cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactConsumerRow {
    /// The consumer surface.
    pub consumer: ImpactConsumer,
    /// Human-facing label.
    pub consumer_label: String,
    /// Accountable owner role.
    pub owner_role: String,
    /// The dimensions this consumer reads.
    pub read_dimensions: Vec<ImpactDimension>,
    /// The union of artifact classes disclosed across the read dimensions.
    pub disclosed_artifact_classes: Vec<ArtifactClass>,
    /// The union of profiles across the read dimensions.
    pub profiles: Vec<DeploymentProfile>,
    /// The derived review readiness.
    pub review_readiness: ReviewReadiness,
    /// Coverage status.
    pub status: ConsumerStatus,
    /// Traffic-light signal.
    pub signal: DescriptorSignal,
    /// Gate decision.
    pub gate_decision: DescriptorGate,
    /// True when at least one read card is a confirmed destructive change.
    pub requires_pre_restart_acknowledgement: bool,
    /// Review gaps, one per (dimension, cause).
    pub gaps: Vec<ImpactGap>,
    /// Routable status message id.
    pub status_message_id: String,
    /// Routable decision message id.
    pub decision_message_id: String,
}

impl ImpactConsumerRow {
    /// Builds a consumer row; the resolved unions, gaps, and verdict are recomputed against the
    /// packet's cards when the packet is assembled.
    pub fn new(consumer: ImpactConsumer, read_dimensions: &[ImpactDimension]) -> Self {
        Self {
            consumer,
            consumer_label: consumer.label().to_owned(),
            owner_role: consumer.owner_role().to_owned(),
            read_dimensions: read_dimensions.to_vec(),
            disclosed_artifact_classes: Vec::new(),
            profiles: Vec::new(),
            review_readiness: ReviewReadiness::ClearToApply,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            gate_decision: DescriptorGate::Governed,
            requires_pre_restart_acknowledgement: false,
            gaps: Vec::new(),
            status_message_id: format!(
                "{}consumer.{}.status",
                M5_CHANGE_IMPACT_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
            decision_message_id: format!(
                "{}consumer.{}.decision",
                M5_CHANGE_IMPACT_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
        }
    }

    /// Recomputes the resolved unions, gaps, and verdict from the packet's cards, so a consumer's
    /// review verdict is always generated from the same checked-in cards rather than a hand-maintained
    /// status.
    pub fn recompute(&mut self, cards: &[ChangeImpactCard]) {
        let mut read = self.read_dimensions.clone();
        read.sort_by_key(|d| dimension_rank(*d));
        read.dedup();
        self.read_dimensions = read.clone();

        let card_for = |dimension: ImpactDimension| -> Option<&ChangeImpactCard> {
            cards.iter().find(|c| c.dimension == dimension)
        };

        let mut classes: Vec<ArtifactClass> = Vec::new();
        let mut profiles: Vec<DeploymentProfile> = Vec::new();
        let mut gaps: Vec<ImpactGap> = Vec::new();
        let consumer = self.consumer;
        for &dimension in &read {
            match card_for(dimension) {
                None => {
                    gaps.push(make_gap(
                        consumer,
                        dimension,
                        ImpactGapKind::DimensionNotForecast,
                    ));
                }
                Some(card) => {
                    classes.extend(card.affected_artifact_classes.iter().copied());
                    profiles.extend(card.affected_profiles.iter().copied());
                    if let Some(kind) = card.gap_kind() {
                        gaps.push(make_gap(consumer, dimension, kind));
                    }
                }
            }
        }
        classes.sort_by_key(|c| artifact_rank(*c));
        classes.dedup();
        profiles.sort_by_key(|p| profile_rank(*p));
        profiles.dedup();
        gaps.sort_by(|a, b| {
            dimension_rank(a.dimension)
                .cmp(&dimension_rank(b.dimension))
                .then(a.gap_kind.as_str().cmp(b.gap_kind.as_str()))
        });

        self.disclosed_artifact_classes = classes;
        self.profiles = profiles;
        self.gaps = gaps;

        let mut gate = DescriptorGate::Governed;
        for gap in &self.gaps {
            gate = worst_gate(gate, gap.gap_kind.gate());
        }
        self.gate_decision = gate;
        self.review_readiness = ReviewReadiness::from_gate(gate);
        self.status = status_for_gate(gate);
        self.signal = signal_for_gate(gate);
        self.requires_pre_restart_acknowledgement = gate == DescriptorGate::Blocked;
    }

    /// True when the consumer reads every card as clear to apply.
    pub fn is_clear(&self) -> bool {
        self.gate_decision == DescriptorGate::Governed
    }

    /// True when at least one read card narrows the consumer to a review-recommended state.
    pub fn is_review(&self) -> bool {
        self.gate_decision == DescriptorGate::Narrowed
    }

    /// True when at least one read card holds the consumer for a pre-restart acknowledgement.
    pub fn is_hold(&self) -> bool {
        self.gate_decision == DescriptorGate::Blocked
    }
}

fn make_gap(
    consumer: ImpactConsumer,
    dimension: ImpactDimension,
    kind: ImpactGapKind,
) -> ImpactGap {
    ImpactGap {
        consumer,
        dimension,
        artifact_class: dimension.primary_artifact_class(),
        gap_kind: kind,
        cause_message_id: format!(
            "{}consumer.{}.{}.{}.gap",
            M5_CHANGE_IMPACT_MESSAGE_ID_PREFIX,
            consumer.as_str(),
            dimension.as_str(),
            kind.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Aggregate sub-objects
// ---------------------------------------------------------------------------

/// The staged update the card set forecasts impact for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeImpactTarget {
    /// The channel the staged update is on.
    pub channel: ChannelScope,
    /// The deployment profiles the staged update covers.
    pub profiles: Vec<DeploymentProfile>,
    /// The currently installed version.
    pub current_version: String,
    /// The version the staged update moves to.
    pub target_version: String,
    /// The basis of the forecast, labeled honestly.
    pub forecast_basis: ForecastBasis,
}

/// The kind of inputs the forecast was generated from, labeled so partial coverage is honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForecastBasis {
    /// Forecast from release evidence only.
    ReleaseEvidenceOnly,
    /// Forecast from release evidence plus a local scan.
    ReleaseAndLocalScan,
    /// Forecast from a local scan only.
    LocalScanOnly,
    /// Forecast as a mirror-import preflight.
    MirrorImportPreflight,
    /// Forecast reconstructed for support review.
    SupportReconstruction,
}

impl ForecastBasis {
    /// Every basis, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReleaseEvidenceOnly,
        Self::ReleaseAndLocalScan,
        Self::LocalScanOnly,
        Self::MirrorImportPreflight,
        Self::SupportReconstruction,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseEvidenceOnly => "release_evidence_only",
            Self::ReleaseAndLocalScan => "release_and_local_scan",
            Self::LocalScanOnly => "local_scan_only",
            Self::MirrorImportPreflight => "mirror_import_preflight",
            Self::SupportReconstruction => "support_reconstruction",
        }
    }
}

/// Disclosure flags asserting every claimed consumer ingests this one card set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactDisclosure {
    /// The update center consumes the card set.
    pub update_center_consumes_cards: bool,
    /// The migration assistant consumes the card set.
    pub migration_assistant_consumes_cards: bool,
    /// The release center consumes the card set.
    pub release_center_consumes_cards: bool,
    /// The team-lead review surface consumes the card set.
    pub team_lead_review_consumes_cards: bool,
    /// The admin console consumes the card set.
    pub admin_console_consumes_cards: bool,
    /// The support export consumes the card set.
    pub support_export_consumes_cards: bool,
}

impl ImpactDisclosure {
    fn canonical() -> Self {
        Self {
            update_center_consumes_cards: true,
            migration_assistant_consumes_cards: true,
            release_center_consumes_cards: true,
            team_lead_review_consumes_cards: true,
            admin_console_consumes_cards: true,
            support_export_consumes_cards: true,
        }
    }

    /// True when every consumer is asserted to consume the card set.
    pub fn all_consume(&self) -> bool {
        self.update_center_consumes_cards
            && self.migration_assistant_consumes_cards
            && self.release_center_consumes_cards
            && self.team_lead_review_consumes_cards
            && self.admin_console_consumes_cards
            && self.support_export_consumes_cards
    }
}

/// Roll-up counts over the cards and consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactCounts {
    /// Total cards.
    pub total_cards: u32,
    /// Cards clear to apply (governed).
    pub clear_cards: u32,
    /// Cards needing review (narrowed).
    pub review_cards: u32,
    /// Cards held for resolution (blocked).
    pub hold_cards: u32,
    /// Cards that are low-risk cache churn.
    pub low_risk_cache_churn_cards: u32,
    /// Cards that are destructive or habit-breaking.
    pub destructive_or_habit_breaking_cards: u32,
    /// Cards whose forecast is speculative (estimated / unknown inputs).
    pub speculative_cards: u32,
    /// Total consumers.
    pub total_consumers: u32,
    /// Consumers clear to apply.
    pub clear_consumers: u32,
    /// Consumers needing review.
    pub review_consumers: u32,
    /// Consumers held for resolution.
    pub hold_consumers: u32,
    /// Whether the packet requires a pre-restart acknowledgement.
    pub requires_pre_restart_acknowledgement: bool,
}

/// The packet-level forecast-coverage honesty block: how much of the forecast is fully grounded vs.
/// speculative or not-applicable, so partial coverage is disclosed rather than implied complete.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastCoverage {
    /// Cards with confirmed / likely confidence.
    pub fully_forecast_cards: u32,
    /// Cards with estimated confidence.
    pub estimated_cards: u32,
    /// Cards with unknown inputs.
    pub unknown_input_cards: u32,
    /// Cards the dimension does not apply to.
    pub not_applicable_cards: u32,
    /// True when at least one card rests on partial or absent inputs.
    pub has_partial_coverage: bool,
}

/// The packet-level pre-restart review gate aggregating the per-consumer decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactReleaseGate {
    /// Whether any consumer is held for a pre-restart acknowledgement.
    pub requires_pre_restart_acknowledgement: bool,
    /// Tokens of the held consumers.
    pub hold_consumers: Vec<String>,
    /// Tokens of the review-recommended consumers.
    pub review_consumers: Vec<String>,
    /// Tokens of the clear consumers.
    pub clear_consumers: Vec<String>,
    /// Tokens of the dimensions that contributed a gap.
    pub affected_dimensions: Vec<String>,
    /// Routable gate message id.
    pub gate_message_id: String,
}

/// The frozen controlled vocabulary the cards draw from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactVocabulary {
    /// Dimension tokens.
    pub dimensions: Vec<String>,
    /// Artifact-class tokens.
    pub artifact_classes: Vec<String>,
    /// Risk-class tokens.
    pub risk_classes: Vec<String>,
    /// Forecast-confidence tokens.
    pub confidence_levels: Vec<String>,
    /// Review-readiness tokens.
    pub review_readiness: Vec<String>,
    /// Follow-up task-class tokens.
    pub follow_up_task_classes: Vec<String>,
    /// Task-timing tokens.
    pub task_timings: Vec<String>,
    /// Task-automation tokens.
    pub task_automations: Vec<String>,
    /// Rollback-choice tokens.
    pub rollback_choices: Vec<String>,
    /// Profile tokens.
    pub profiles: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
    /// Forecast-basis tokens.
    pub forecast_bases: Vec<String>,
}

impl ImpactVocabulary {
    /// The canonical frozen vocabulary.
    pub fn canonical() -> Self {
        Self {
            dimensions: tokens(&ImpactDimension::ALL, |x| x.as_str()),
            artifact_classes: tokens(&ArtifactClass::ALL, |x| x.as_str()),
            risk_classes: tokens(&RiskClass::ALL, |x| x.as_str()),
            confidence_levels: tokens(&ForecastConfidence::ALL, |x| x.as_str()),
            review_readiness: tokens(&ReviewReadiness::ALL, |x| x.as_str()),
            follow_up_task_classes: tokens(&FollowUpTaskClass::ALL, |x| x.as_str()),
            task_timings: tokens(&TaskTiming::ALL, |x| x.as_str()),
            task_automations: tokens(&TaskAutomation::ALL, |x| x.as_str()),
            rollback_choices: tokens(&RollbackChoice::ALL, |x| x.as_str()),
            profiles: tokens(&DeploymentProfile::ALL, |x| x.as_str()),
            consumers: tokens(&ImpactConsumer::ALL, |x| x.as_str()),
            gap_kinds: tokens(&ImpactGapKind::ALL, |x| x.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |x| x.as_str()),
            forecast_bases: tokens(&ForecastBasis::ALL, |x| x.as_str()),
        }
    }

    /// True when this vocabulary equals the canonical frozen vocabulary.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance flags every canonical card set asserts. They restate the acceptance bar so a tampered
/// packet that flips one to false fails [`ChangeImpactCardSet::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactConformance {
    /// Every forecast dimension is carded exactly once.
    pub every_dimension_forecast: bool,
    /// Risk class is disclosed on every card.
    pub risk_class_disclosed_per_card: bool,
    /// Low-risk cache churn is distinguished from destructive / habit-breaking change.
    pub low_risk_distinguished_from_destructive: bool,
    /// Forecast confidence is labeled on every card.
    pub forecast_confidence_labelled: bool,
    /// Unknown / partial inputs are labeled, never raised as a hard failure.
    pub unknown_inputs_labelled_not_failed: bool,
    /// A manual follow-up task is disclosed on every card.
    pub follow_up_tasks_disclosed: bool,
    /// A rollback or pin choice is disclosed on every card.
    pub rollback_or_pin_choice_disclosed: bool,
    /// The affected artifact-class / profile scope is disclosed on every card.
    pub affected_scope_disclosed: bool,
    /// The cards are computed and visible before restart.
    pub visible_before_restart: bool,
    /// The card set is exportable and reviewable outside the app.
    pub exportable_outside_app: bool,
    /// Every claimed consumer reads this one card set.
    pub consumers_read_one_card_set: bool,
    /// Every consumer verdict is derived from the cards, not hand-maintained.
    pub consumer_verdict_derived_from_cards: bool,
    /// The controlled enums are frozen.
    pub controlled_enums_frozen: bool,
    /// The export carries metadata and refs only — no credential bodies or raw payloads.
    pub export_carries_no_raw_material: bool,
}

impl ImpactConformance {
    fn canonical() -> Self {
        Self {
            every_dimension_forecast: true,
            risk_class_disclosed_per_card: true,
            low_risk_distinguished_from_destructive: true,
            forecast_confidence_labelled: true,
            unknown_inputs_labelled_not_failed: true,
            follow_up_tasks_disclosed: true,
            rollback_or_pin_choice_disclosed: true,
            affected_scope_disclosed: true,
            visible_before_restart: true,
            exportable_outside_app: true,
            consumers_read_one_card_set: true,
            consumer_verdict_derived_from_cards: true,
            controlled_enums_frozen: true,
            export_carries_no_raw_material: true,
        }
    }

    /// True when every conformance flag holds.
    pub fn all_hold(&self) -> bool {
        *self == Self::canonical()
    }
}

// ---------------------------------------------------------------------------
// Render channel
// ---------------------------------------------------------------------------

/// The render channels the packet must serialize identically across.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImpactChannel {
    /// The desktop update center / migration assistant.
    DesktopUi,
    /// The CLI / headless emitter.
    CliHeadless,
    /// The offline / exported review surface.
    OfflineExport,
}

// ---------------------------------------------------------------------------
// Validation violations
// ---------------------------------------------------------------------------

/// A reason a card set failed [`ChangeImpactCardSet::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeImpactViolation {
    /// The record kind or schema version is wrong.
    HeaderDrift,
    /// A dimension is missing or carded more than once.
    DimensionCoverageDrift,
    /// A card's derived gate / readiness / signal / scope / acknowledgement drifted.
    CardDerivationDrift,
    /// A speculative card was raised to a hard failure (blocked) — the lane's guardrail.
    SpeculativeHardFailure,
    /// A consumer's derived verdict, unions, or gaps drifted.
    ConsumerVerdictDrift,
    /// The summary counts, coverage, or release gate drifted.
    SummaryDrift,
    /// The disclosure flags do not all assert consumption of the one card set.
    DisclosureDrift,
    /// The controlled vocabulary drifted.
    VocabularyDrift,
    /// A conformance flag does not hold.
    ConformanceDrift,
    /// The export carried forbidden raw material.
    ForbiddenMaterial,
}

impl ChangeImpactViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeaderDrift => "header_drift",
            Self::DimensionCoverageDrift => "dimension_coverage_drift",
            Self::CardDerivationDrift => "card_derivation_drift",
            Self::SpeculativeHardFailure => "speculative_hard_failure",
            Self::ConsumerVerdictDrift => "consumer_verdict_drift",
            Self::SummaryDrift => "summary_drift",
            Self::DisclosureDrift => "disclosure_drift",
            Self::VocabularyDrift => "vocabulary_drift",
            Self::ConformanceDrift => "conformance_drift",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

// ---------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------

/// Builder input for [`ChangeImpactCardSet::new`].
#[derive(Debug, Clone)]
pub struct ChangeImpactCardSetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-facing report label.
    pub report_label: String,
    /// Evaluation timestamp.
    pub evaluated_at: String,
    /// The staged update the cards forecast.
    pub target: ChangeImpactTarget,
    /// The per-dimension cards.
    pub cards: Vec<ChangeImpactCard>,
    /// The claimed consumer rows.
    pub consumers: Vec<ImpactConsumerRow>,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable change-impact card set the update center, migration
/// assistant, release center, team-lead review, admin console, and support export consume before
/// restart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeImpactCardSet {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-facing report label.
    pub report_label: String,
    /// Evaluation timestamp.
    pub evaluated_at: String,
    /// The staged update the cards forecast.
    pub target: ChangeImpactTarget,
    /// The per-dimension cards.
    pub cards: Vec<ChangeImpactCard>,
    /// The dimension tokens this packet covers.
    pub dimensions: Vec<String>,
    /// The consumer rows reading the cards.
    pub consumers: Vec<ImpactConsumerRow>,
    /// The consumer tokens, in canonical order.
    pub consumer_tokens: Vec<String>,
    /// Disclosure flags.
    pub disclosure: ImpactDisclosure,
    /// Roll-up counts.
    pub summary: ImpactCounts,
    /// Forecast-coverage honesty block.
    pub coverage: ForecastCoverage,
    /// Packet-level pre-restart review gate.
    pub release_gate: ImpactReleaseGate,
    /// Controlled vocabulary.
    pub vocabulary: ImpactVocabulary,
    /// Conformance flags.
    pub conformance: ImpactConformance,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl ChangeImpactCardSet {
    /// Builds a packet from the given cards and consumer rows, recomputing every derived field so the
    /// published packet is always generated from the same checked-in cards.
    pub fn new(input: ChangeImpactCardSetInput) -> Self {
        let mut cards = input.cards;
        for card in &mut cards {
            card.recompute();
        }
        cards.sort_by_key(|c| dimension_rank(c.dimension));

        let mut consumers = input.consumers;
        for consumer in &mut consumers {
            consumer.recompute(&cards);
        }
        consumers.sort_by_key(|c| consumer_rank(c.consumer));

        let mut target = input.target;
        target.profiles.sort_by_key(|p| profile_rank(*p));
        target.profiles.dedup();

        let summary = derive_counts(&cards, &consumers);
        let coverage = derive_coverage(&cards);
        let release_gate = derive_release_gate(&consumers);

        Self {
            record_kind: M5_CHANGE_IMPACT_CARD_SET_RECORD_KIND.to_owned(),
            schema_version: M5_CHANGE_IMPACT_CARD_SET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            target,
            dimensions: tokens(&ImpactDimension::ALL, |x| x.as_str()),
            cards,
            consumer_tokens: tokens(&ImpactConsumer::ALL, |x| x.as_str()),
            consumers,
            disclosure: ImpactDisclosure::canonical(),
            summary,
            coverage,
            release_gate,
            vocabulary: ImpactVocabulary::canonical(),
            conformance: ImpactConformance::canonical(),
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Looks up the card for a dimension.
    pub fn card(&self, dimension: ImpactDimension) -> Option<&ChangeImpactCard> {
        self.cards.iter().find(|c| c.dimension == dimension)
    }

    /// Looks up the consumer row for a consumer.
    pub fn consumer(&self, consumer: ImpactConsumer) -> Option<&ImpactConsumerRow> {
        self.consumers.iter().find(|c| c.consumer == consumer)
    }

    /// Whether the packet requires a pre-restart acknowledgement.
    pub fn requires_pre_restart_acknowledgement(&self) -> bool {
        self.release_gate.requires_pre_restart_acknowledgement
    }

    /// Validates every derived field by recomputing it from the cards and comparing. Returns an empty
    /// vector when the packet is internally consistent.
    pub fn validate(&self) -> Vec<ChangeImpactViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CHANGE_IMPACT_CARD_SET_RECORD_KIND
            || self.schema_version != M5_CHANGE_IMPACT_CARD_SET_SCHEMA_VERSION
        {
            violations.push(ChangeImpactViolation::HeaderDrift);
        }

        // Every dimension carded exactly once.
        for dimension in ImpactDimension::ALL {
            let count = self
                .cards
                .iter()
                .filter(|c| c.dimension == dimension)
                .count();
            if count != 1 {
                violations.push(ChangeImpactViolation::DimensionCoverageDrift);
                break;
            }
        }

        for card in &self.cards {
            // Recompute the card from its inputs and compare the derived verdict.
            let mut fresh = card.clone();
            fresh.recompute();
            if fresh.gate != card.gate
                || fresh.review_readiness != card.review_readiness
                || fresh.status != card.status
                || fresh.signal != card.signal
                || fresh.speculative != card.speculative
                || fresh.affected_artifact_classes != card.affected_artifact_classes
                || fresh.affected_profiles != card.affected_profiles
                || fresh.unknown_input_message_id != card.unknown_input_message_id
                || fresh.requires_pre_restart_acknowledgement
                    != card.requires_pre_restart_acknowledgement
            {
                violations.push(ChangeImpactViolation::CardDerivationDrift);
            }
            // Guardrail: a speculative forecast can never be a hard failure.
            if card.speculative && card.gate == DescriptorGate::Blocked {
                violations.push(ChangeImpactViolation::SpeculativeHardFailure);
            }
            // The primary class must always be disclosed.
            if !card
                .affected_artifact_classes
                .contains(&card.primary_artifact_class)
            {
                violations.push(ChangeImpactViolation::CardDerivationDrift);
            }
        }

        // Consumers: recompute and compare verdict, unions, and gaps.
        for consumer in &self.consumers {
            let mut fresh = ImpactConsumerRow::new(consumer.consumer, &consumer.read_dimensions);
            fresh.recompute(&self.cards);
            if fresh.gate_decision != consumer.gate_decision
                || fresh.review_readiness != consumer.review_readiness
                || fresh.status != consumer.status
                || fresh.signal != consumer.signal
                || fresh.requires_pre_restart_acknowledgement
                    != consumer.requires_pre_restart_acknowledgement
                || fresh.disclosed_artifact_classes != consumer.disclosed_artifact_classes
                || fresh.profiles != consumer.profiles
                || fresh.gaps != consumer.gaps
            {
                violations.push(ChangeImpactViolation::ConsumerVerdictDrift);
                break;
            }
        }

        if self.summary != derive_counts(&self.cards, &self.consumers)
            || self.coverage != derive_coverage(&self.cards)
            || self.release_gate != derive_release_gate(&self.consumers)
        {
            violations.push(ChangeImpactViolation::SummaryDrift);
        }

        if !self.disclosure.all_consume()
            || self.consumer_tokens != tokens(&ImpactConsumer::ALL, |x| x.as_str())
            || self.dimensions != tokens(&ImpactDimension::ALL, |x| x.as_str())
        {
            violations.push(ChangeImpactViolation::DisclosureDrift);
        }

        if !self.vocabulary.matches_canonical() {
            violations.push(ChangeImpactViolation::VocabularyDrift);
        }

        if !self.conformance.all_hold() {
            violations.push(ChangeImpactViolation::ConformanceDrift);
        }

        if contains_forbidden_material(self) {
            violations.push(ChangeImpactViolation::ForbiddenMaterial);
        }

        violations
    }

    /// The canonical export form: pretty JSON, identical across every render channel.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("card set serializes")
    }

    /// Renders the packet for a channel. Every channel produces byte-identical output.
    pub fn render_for_channel(&self, _channel: ImpactChannel) -> String {
        self.export_safe_json()
    }

    /// A compact Markdown summary of the cards and consumer verdicts, for export and review outside
    /// the app.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", self.report_label));
        out.push_str(&format!(
            "Update `{}` → `{}` on channel `{}` — {} cards ({} review, {} hold), {} consumers.\n\n",
            self.target.current_version,
            self.target.target_version,
            self.target.channel.as_str(),
            self.summary.total_cards,
            self.summary.review_cards,
            self.summary.hold_cards,
            self.summary.total_consumers,
        ));
        if self.coverage.has_partial_coverage {
            out.push_str(&format!(
                "> Partial coverage: {} estimated, {} unknown-input card(s) labeled, not failed.\n\n",
                self.coverage.estimated_cards, self.coverage.unknown_input_cards,
            ));
        }
        out.push_str("## Change-impact cards\n\n");
        out.push_str(
            "| Dimension | Risk | Confidence | Readiness | Follow-up | Rollback / pin | Scope |\n",
        );
        out.push_str("|---|---|---|---|---|---|---|\n");
        for c in &self.cards {
            let scope: Vec<&str> = c
                .affected_artifact_classes
                .iter()
                .map(|x| x.as_str())
                .collect();
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                c.dimension.as_str(),
                c.risk_class.as_str(),
                c.confidence.as_str(),
                c.review_readiness.as_str(),
                c.follow_up.task_class.as_str(),
                c.rollback_choice.as_str(),
                scope.join(", "),
            ));
        }
        out.push_str("\n## Consumers\n\n");
        for c in &self.consumers {
            out.push_str(&format!(
                "- `{}` → `{}` ({}",
                c.consumer.as_str(),
                c.review_readiness.as_str(),
                c.gate_decision.as_str(),
            ));
            if c.gaps.is_empty() {
                out.push_str(")\n");
            } else {
                let gaps: Vec<String> = c
                    .gaps
                    .iter()
                    .map(|g| format!("{}:{}", g.dimension.as_str(), g.gap_kind.as_str()))
                    .collect();
                out.push_str(&format!("; gap: {})\n", gaps.join(", ")));
            }
        }
        out
    }

    /// A machine-readable CSV of every change-impact card, for export and review outside the app.
    pub fn render_card_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "dimension,risk_class,confidence,speculative,review_readiness,follow_up_task,task_timing,rollback_choice,from_version,to_version,gate\n",
        );
        for c in &self.cards {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                c.dimension.as_str(),
                c.risk_class.as_str(),
                c.confidence.as_str(),
                c.speculative,
                c.review_readiness.as_str(),
                c.follow_up.task_class.as_str(),
                c.follow_up.timing.as_str(),
                c.rollback_choice.as_str(),
                c.from_version.as_deref().unwrap_or(""),
                c.to_version.as_deref().unwrap_or(""),
                c.gate.as_str(),
            ));
        }
        out
    }
}

fn derive_counts(cards: &[ChangeImpactCard], consumers: &[ImpactConsumerRow]) -> ImpactCounts {
    let clear_cards = cards
        .iter()
        .filter(|c| c.gate == DescriptorGate::Governed)
        .count() as u32;
    let review_cards = cards
        .iter()
        .filter(|c| c.gate == DescriptorGate::Narrowed)
        .count() as u32;
    let hold_cards = cards
        .iter()
        .filter(|c| c.gate == DescriptorGate::Blocked)
        .count() as u32;
    let clear_consumers = consumers.iter().filter(|c| c.is_clear()).count() as u32;
    let review_consumers = consumers.iter().filter(|c| c.is_review()).count() as u32;
    let hold_consumers = consumers.iter().filter(|c| c.is_hold()).count() as u32;
    ImpactCounts {
        total_cards: cards.len() as u32,
        clear_cards,
        review_cards,
        hold_cards,
        low_risk_cache_churn_cards: cards
            .iter()
            .filter(|c| c.risk_class.is_low_risk_cache_churn())
            .count() as u32,
        destructive_or_habit_breaking_cards: cards
            .iter()
            .filter(|c| c.risk_class.is_destructive_or_habit_breaking())
            .count() as u32,
        speculative_cards: cards.iter().filter(|c| c.speculative).count() as u32,
        total_consumers: consumers.len() as u32,
        clear_consumers,
        review_consumers,
        hold_consumers,
        requires_pre_restart_acknowledgement: hold_consumers > 0,
    }
}

fn derive_coverage(cards: &[ChangeImpactCard]) -> ForecastCoverage {
    let fully = cards
        .iter()
        .filter(|c| {
            matches!(
                c.confidence,
                ForecastConfidence::Confirmed | ForecastConfidence::Likely
            )
        })
        .count() as u32;
    let estimated = cards
        .iter()
        .filter(|c| c.confidence == ForecastConfidence::Estimated)
        .count() as u32;
    let unknown = cards
        .iter()
        .filter(|c| c.confidence == ForecastConfidence::Unknown)
        .count() as u32;
    let not_applicable = cards
        .iter()
        .filter(|c| c.confidence == ForecastConfidence::NotApplicable)
        .count() as u32;
    ForecastCoverage {
        fully_forecast_cards: fully,
        estimated_cards: estimated,
        unknown_input_cards: unknown,
        not_applicable_cards: not_applicable,
        has_partial_coverage: estimated > 0 || unknown > 0,
    }
}

fn derive_release_gate(consumers: &[ImpactConsumerRow]) -> ImpactReleaseGate {
    let collect = |pred: fn(&ImpactConsumerRow) -> bool| -> Vec<String> {
        consumers
            .iter()
            .filter(|c| pred(c))
            .map(|c| c.consumer.as_str().to_owned())
            .collect()
    };
    let mut affected: Vec<ImpactDimension> = consumers
        .iter()
        .flat_map(|c| c.gaps.iter().map(|g| g.dimension))
        .collect();
    affected.sort_by_key(|d| dimension_rank(*d));
    affected.dedup();
    let hold = collect(ImpactConsumerRow::is_hold);
    ImpactReleaseGate {
        requires_pre_restart_acknowledgement: !hold.is_empty(),
        hold_consumers: hold,
        review_consumers: collect(ImpactConsumerRow::is_review),
        clear_consumers: collect(ImpactConsumerRow::is_clear),
        affected_dimensions: affected.iter().map(|d| d.as_str().to_owned()).collect(),
        gate_message_id: format!("{}release_gate", M5_CHANGE_IMPACT_MESSAGE_ID_PREFIX),
    }
}

/// Scans the export for forbidden raw material (credential bodies / raw provider payloads).
fn contains_forbidden_material(packet: &ChangeImpactCardSet) -> bool {
    let json = serde_json::to_string(packet)
        .unwrap_or_default()
        .to_ascii_lowercase();
    const FORBIDDEN: [&str; 6] = [
        "bearer_token",
        "authorization:",
        "private_key",
        "begin rsa",
        "set-cookie",
        "client_secret",
    ];
    FORBIDDEN.iter().any(|needle| json.contains(needle))
}

/// Maps each variant of an `as_str`-bearing enum to its token, in declaration order.
fn tokens<T: Copy, const N: usize>(all: &[T; N], f: impl Fn(&T) -> &'static str) -> Vec<String> {
    all.iter().map(|x| f(x).to_owned()).collect()
}
