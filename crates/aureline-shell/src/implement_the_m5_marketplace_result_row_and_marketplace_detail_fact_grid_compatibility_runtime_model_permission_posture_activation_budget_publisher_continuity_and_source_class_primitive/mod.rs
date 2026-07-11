//! Implemented M5 marketplace-result-row and marketplace-detail-fact-grid primitives.
//!
//! The frozen [marketplace / install-review component matrix][matrix] names the reusable
//! extension-marketplace UI components and locks their controlled vocabulary. This module is the
//! first implement lane over that matrix: it turns the two compare-and-inspect components — the
//! **marketplace result row** and the **marketplace detail fact grid** — into resolvers that
//! produce export-safe, honest projections, so a user can compare compatibility, runtime model,
//! permission posture, activation cost, publisher continuity, support class, and registry source
//! from the listing and detail surfaces without opening disconnected marketplace pages.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render the result row with compatibility, runtime model, permission posture, support class,
//!   performance evidence (activation budget), publisher continuity, and registry source class in
//!   compact form.** [`resolve_marketplace_result_row`] refuses to read as a clean, compare-at-a-
//!   glance row when the artifact identity is unstated, the registry source cannot be resolved, the
//!   source class is collapsed into one ambiguous origin, permission widening or activation cost is
//!   hidden, or an incompatible / over-budget artifact reads as ready to install; it degrades
//!   instead.
//! * **Render the detail fact grid with richer version ranges, lifecycle state, trust tier, and
//!   docs/changelog/open-issues linkage on top of the same source/compatibility/permission/budget/
//!   publisher grammar.** [`resolve_marketplace_detail_fact_grid`] degrades when the version range,
//!   lifecycle state, or docs/changelog/open-issues linkage is missing.
//! * **Keep list and detail fields aligned so the same artifact never presents contradictory trust
//!   or support facts across surfaces.** The packet proves, by paired resolved examples, that a
//!   result row and a detail fact grid describing the same artifact share one source class,
//!   compatibility state, host/runtime model, permission posture, activation budget, publisher
//!   continuity, and trust tier.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5MarketplaceInstallDisposition`] marketplace / install-disposition vocabulary, the
//! [`M5RegistrySourceClass`] registry-source vocabulary, the [`M5CompatibilityState`] compatibility
//! vocabulary, the [`M5HostRuntimeModel`] host/runtime vocabulary, the [`M5PermissionPostureState`]
//! permission-posture vocabulary, the [`M5ActivationBudgetBandState`] activation-budget vocabulary,
//! and the [`M5PublisherContinuityState`] publisher-continuity vocabulary — so marketplace,
//! extensions, and registry surfaces can never fork their own source, compatibility, permission,
//! budget, or publisher wording or invent feature-local badges. Raw secret values and private
//! endpoints stay outside the export boundary.
//!
//! [matrix]: crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_marketplace_result_detail_controls,
    seeded_m5_marketplace_result_detail_controls_marketplace_ui_beta_narrowed,
    seeded_m5_marketplace_result_detail_controls_registry_ui_preview_narrowed,
    M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix::{
    M5ActivationBudgetBandState, M5CompatibilityState, M5HostRuntimeModel,
    M5MarketplaceInstallAccessibilityRoute, M5MarketplaceInstallComponentFamily,
    M5MarketplaceInstallConsumerSurface, M5MarketplaceInstallDeploymentLine,
    M5MarketplaceInstallDisposition, M5MarketplaceInstallDowngradeTrigger,
    M5MarketplaceInstallQualificationClass, M5MarketplaceInstallRequiredLabel,
    M5PermissionPostureState, M5PublisherContinuityState, M5RegistrySourceClass,
    M5_MARKETPLACE_DETAIL_FACT_GRID_SCHEMA_REF, M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
    M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF, M5_MARKETPLACE_RESULT_ROW_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5MarketplaceResultDetailControlsPacket`].
pub const M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_RECORD_KIND: &str =
    "implement_m5_marketplace_result_row_and_marketplace_detail_fact_grid_controls";

/// Schema version for M5 marketplace-result-row / detail-fact-grid controls records.
pub const M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-marketplace-result-row-detail-fact-grid-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_DOC_REF: &str =
    "docs/marketplace/m5_marketplace_result_row_and_marketplace_detail_fact_grid_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-marketplace-result-row-detail-fact-grid-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-marketplace-result-row-detail-fact-grid-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-marketplace-result-row-detail-fact-grid-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-marketplace-result-row-detail-fact-grid-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface
/// taxonomy so no lane invents a parallel surface set.
pub type M5MarketplaceResultDetailConsumerSurface = M5MarketplaceInstallConsumerSurface;

/// Controlled support / trust tier a marketplace component names, so a verified artifact is never
/// presented with the same weight as an unreviewed one. Minted by this lane because the frozen
/// matrix carries publisher continuity and registry source but not a per-artifact support tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceTrustTier {
    /// Published by a verified publisher.
    VerifiedPublisher,
    /// Reviewed by the marketplace / registry.
    Reviewed,
    /// Community-contributed, not formally reviewed.
    Community,
    /// Unreviewed.
    Unreviewed,
    /// Currently quarantined.
    Quarantined,
    /// The support / trust tier cannot currently be resolved.
    TierUnknown,
}

impl M5MarketplaceTrustTier {
    /// Every trust tier, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::VerifiedPublisher,
        Self::Reviewed,
        Self::Community,
        Self::Unreviewed,
        Self::Quarantined,
        Self::TierUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedPublisher => "verified_publisher",
            Self::Reviewed => "reviewed",
            Self::Community => "community",
            Self::Unreviewed => "unreviewed",
            Self::Quarantined => "quarantined",
            Self::TierUnknown => "tier_unknown",
        }
    }
}

/// Controlled lifecycle state a detail fact grid names, so a deprecated or yanked artifact is never
/// presented as freshly active. Minted by this lane for the richer detail grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceLifecycleState {
    /// Actively maintained.
    Active,
    /// Available as a preview / prerelease.
    Preview,
    /// Deprecated, still installable.
    Deprecated,
    /// End of life, no longer maintained.
    EndOfLife,
    /// Yanked / withdrawn.
    Yanked,
    /// The lifecycle state cannot currently be resolved.
    LifecycleUnknown,
}

impl M5MarketplaceLifecycleState {
    /// Every lifecycle state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Active,
        Self::Preview,
        Self::Deprecated,
        Self::EndOfLife,
        Self::Yanked,
        Self::LifecycleUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Preview => "preview",
            Self::Deprecated => "deprecated",
            Self::EndOfLife => "end_of_life",
            Self::Yanked => "yanked",
            Self::LifecycleUnknown => "lifecycle_unknown",
        }
    }
}

/// One mandatory rendered part a marketplace result row or detail fact grid must be able to show, so
/// no marketplace fact is left implicit behind compact chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceResultDetailAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed marketplace disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The registry source class behind the artifact (both components).
    SourceClass,
    /// The compatibility range / state (both components).
    CompatibilityRange,
    /// The host / runtime model (both components).
    HostRuntimeModel,
    /// The permission posture and any transitive widening (both components).
    PermissionPosture,
    /// The activation-budget band / performance evidence (both components).
    ActivationBudget,
    /// The support / trust tier (both components).
    TrustTier,
    /// The publisher continuity (both components).
    PublisherContinuity,
    /// The richer version range (detail fact grid).
    VersionRange,
    /// The lifecycle state (detail fact grid).
    LifecycleState,
    /// The docs / changelog / open-issues linkage (detail fact grid).
    DocsChangelogIssuesLinkage,
    /// The command-backed path to open the detail surface (result row).
    DetailCommand,
}

impl M5MarketplaceResultDetailAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::SourceClass,
        Self::CompatibilityRange,
        Self::HostRuntimeModel,
        Self::PermissionPosture,
        Self::ActivationBudget,
        Self::TrustTier,
        Self::PublisherContinuity,
        Self::VersionRange,
        Self::LifecycleState,
        Self::DocsChangelogIssuesLinkage,
        Self::DetailCommand,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::SourceClass => "source_class",
            Self::CompatibilityRange => "compatibility_range",
            Self::HostRuntimeModel => "host_runtime_model",
            Self::PermissionPosture => "permission_posture",
            Self::ActivationBudget => "activation_budget",
            Self::TrustTier => "trust_tier",
            Self::PublisherContinuity => "publisher_continuity",
            Self::VersionRange => "version_range",
            Self::LifecycleState => "lifecycle_state",
            Self::DocsChangelogIssuesLinkage => "docs_changelog_issues_linkage",
            Self::DetailCommand => "detail_command",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to compare or
/// inspect the fact behind a degraded marketplace component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceResultDetailNextAction {
    /// Open the command-backed detail surface.
    OpenDetail,
    /// Review the compatibility range and host / runtime model.
    ReviewCompatibility,
    /// Review the permission manifest and activation budget.
    ReviewPermissionAndBudget,
    /// Review the publisher continuity and registry source class.
    ReviewPublisherAndSource,
    /// Review diagnostics for a stale or unresolved signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5MarketplaceResultDetailNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenDetail,
        Self::ReviewCompatibility,
        Self::ReviewPermissionAndBudget,
        Self::ReviewPublisherAndSource,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDetail => "open_detail",
            Self::ReviewCompatibility => "review_compatibility",
            Self::ReviewPermissionAndBudget => "review_permission_and_budget",
            Self::ReviewPublisherAndSource => "review_publisher_and_source",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceResultDetailExportField {
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
    /// The registry source class named by the components.
    SourceClass,
    /// The compatibility state named by the components.
    Compatibility,
    /// The permission posture named by the components.
    PermissionPosture,
    /// The activation-budget band named by the components.
    ActivationBudget,
    /// The publisher continuity named by the components.
    PublisherContinuity,
    /// The accountable owner role.
    OwnerRole,
}

impl M5MarketplaceResultDetailExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SourceClass,
        Self::Compatibility,
        Self::PermissionPosture,
        Self::ActivationBudget,
        Self::PublisherContinuity,
        Self::OwnerRole,
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
            Self::SourceClass => "source_class",
            Self::Compatibility => "compatibility",
            Self::PermissionPosture => "permission_posture",
            Self::ActivationBudget => "activation_budget",
            Self::PublisherContinuity => "publisher_continuity",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a marketplace result row degraded below a clean, compare-at-a-glance state. The
/// degrade-first ladder returns one of these instead of ever letting an ambiguous row read as a
/// clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceResultRowDegradeReason {
    /// The artifact identity is unstated; a user cannot tell what the row represents.
    ArtifactIdentityUnstated,
    /// The registry source cannot currently be resolved.
    RegistrySourceUnresolved,
    /// The registry source class is collapsed into one ambiguous origin.
    SourceClassCollapsedIntoAmbiguousOrigin,
    /// The compatibility state cannot currently be resolved.
    CompatibilityUnresolved,
    /// An incompatible or over-budget artifact reads as ready to install.
    IncompatibleOrOverBudgetShownAsReady,
    /// Permission widening is hidden behind compact chrome.
    PermissionWideningHidden,
    /// Activation cost is hidden behind compact chrome.
    ActivationCostHidden,
    /// The support / trust tier cannot currently be resolved.
    SupportClassUnresolved,
    /// A publisher transfer or deprecation is hidden.
    PublisherTransferHidden,
    /// No command-backed detail entrypoint is reachable.
    DetailPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5MarketplaceResultRowDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ArtifactIdentityUnstated,
        Self::RegistrySourceUnresolved,
        Self::SourceClassCollapsedIntoAmbiguousOrigin,
        Self::CompatibilityUnresolved,
        Self::IncompatibleOrOverBudgetShownAsReady,
        Self::PermissionWideningHidden,
        Self::ActivationCostHidden,
        Self::SupportClassUnresolved,
        Self::PublisherTransferHidden,
        Self::DetailPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentityUnstated => "artifact_identity_unstated",
            Self::RegistrySourceUnresolved => "registry_source_unresolved",
            Self::SourceClassCollapsedIntoAmbiguousOrigin => {
                "source_class_collapsed_into_ambiguous_origin"
            }
            Self::CompatibilityUnresolved => "compatibility_unresolved",
            Self::IncompatibleOrOverBudgetShownAsReady => {
                "incompatible_or_over_budget_shown_as_ready"
            }
            Self::PermissionWideningHidden => "permission_widening_hidden",
            Self::ActivationCostHidden => "activation_cost_hidden",
            Self::SupportClassUnresolved => "support_class_unresolved",
            Self::PublisherTransferHidden => "publisher_transfer_hidden",
            Self::DetailPathMissing => "detail_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5MarketplaceResultDetailNextAction {
        match self {
            Self::ArtifactIdentityUnstated | Self::DetailPathMissing => {
                M5MarketplaceResultDetailNextAction::OpenDetail
            }
            Self::RegistrySourceUnresolved
            | Self::SourceClassCollapsedIntoAmbiguousOrigin
            | Self::SupportClassUnresolved
            | Self::PublisherTransferHidden => {
                M5MarketplaceResultDetailNextAction::ReviewPublisherAndSource
            }
            Self::CompatibilityUnresolved | Self::IncompatibleOrOverBudgetShownAsReady => {
                M5MarketplaceResultDetailNextAction::ReviewCompatibility
            }
            Self::PermissionWideningHidden | Self::ActivationCostHidden => {
                M5MarketplaceResultDetailNextAction::ReviewPermissionAndBudget
            }
            Self::ProofStale => M5MarketplaceResultDetailNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5MarketplaceInstallDowngradeTrigger {
        match self {
            Self::ArtifactIdentityUnstated
            | Self::DetailPathMissing
            | Self::SupportClassUnresolved => {
                M5MarketplaceInstallDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::RegistrySourceUnresolved | Self::SourceClassCollapsedIntoAmbiguousOrigin => {
                M5MarketplaceInstallDowngradeTrigger::RegistrySourceClassCollapsed
            }
            Self::CompatibilityUnresolved | Self::IncompatibleOrOverBudgetShownAsReady => {
                M5MarketplaceInstallDowngradeTrigger::CompatibilityRangeUnstated
            }
            Self::PermissionWideningHidden => {
                M5MarketplaceInstallDowngradeTrigger::PermissionWideningHidden
            }
            Self::ActivationCostHidden => {
                M5MarketplaceInstallDowngradeTrigger::ActivationCostHidden
            }
            Self::PublisherTransferHidden => {
                M5MarketplaceInstallDowngradeTrigger::PublisherTransferHidden
            }
            Self::ProofStale => M5MarketplaceInstallDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a marketplace detail fact grid degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceDetailFactGridDegradeReason {
    /// The artifact identity is unstated.
    ArtifactIdentityUnstated,
    /// The registry source cannot currently be resolved.
    RegistrySourceUnresolved,
    /// The registry source class is collapsed into one ambiguous origin.
    SourceClassCollapsedIntoAmbiguousOrigin,
    /// The compatibility state cannot currently be resolved.
    CompatibilityUnresolved,
    /// The richer version range is unstated.
    VersionRangeUnstated,
    /// An incompatible or over-budget artifact reads as ready to install.
    IncompatibleOrOverBudgetShownAsReady,
    /// Permission widening is hidden.
    PermissionWideningHidden,
    /// Activation cost is hidden.
    ActivationCostHidden,
    /// The support / trust tier cannot currently be resolved.
    SupportClassUnresolved,
    /// A publisher transfer or deprecation is hidden.
    PublisherTransferHidden,
    /// The lifecycle state is unstated.
    LifecycleStateUnstated,
    /// No docs / changelog / open-issues linkage is present.
    DocsChangelogIssuesUnlinked,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5MarketplaceDetailFactGridDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::ArtifactIdentityUnstated,
        Self::RegistrySourceUnresolved,
        Self::SourceClassCollapsedIntoAmbiguousOrigin,
        Self::CompatibilityUnresolved,
        Self::VersionRangeUnstated,
        Self::IncompatibleOrOverBudgetShownAsReady,
        Self::PermissionWideningHidden,
        Self::ActivationCostHidden,
        Self::SupportClassUnresolved,
        Self::PublisherTransferHidden,
        Self::LifecycleStateUnstated,
        Self::DocsChangelogIssuesUnlinked,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentityUnstated => "artifact_identity_unstated",
            Self::RegistrySourceUnresolved => "registry_source_unresolved",
            Self::SourceClassCollapsedIntoAmbiguousOrigin => {
                "source_class_collapsed_into_ambiguous_origin"
            }
            Self::CompatibilityUnresolved => "compatibility_unresolved",
            Self::VersionRangeUnstated => "version_range_unstated",
            Self::IncompatibleOrOverBudgetShownAsReady => {
                "incompatible_or_over_budget_shown_as_ready"
            }
            Self::PermissionWideningHidden => "permission_widening_hidden",
            Self::ActivationCostHidden => "activation_cost_hidden",
            Self::SupportClassUnresolved => "support_class_unresolved",
            Self::PublisherTransferHidden => "publisher_transfer_hidden",
            Self::LifecycleStateUnstated => "lifecycle_state_unstated",
            Self::DocsChangelogIssuesUnlinked => "docs_changelog_issues_unlinked",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5MarketplaceResultDetailNextAction {
        match self {
            Self::ArtifactIdentityUnstated | Self::DocsChangelogIssuesUnlinked => {
                M5MarketplaceResultDetailNextAction::OpenDetail
            }
            Self::RegistrySourceUnresolved
            | Self::SourceClassCollapsedIntoAmbiguousOrigin
            | Self::SupportClassUnresolved
            | Self::PublisherTransferHidden
            | Self::LifecycleStateUnstated => {
                M5MarketplaceResultDetailNextAction::ReviewPublisherAndSource
            }
            Self::CompatibilityUnresolved
            | Self::VersionRangeUnstated
            | Self::IncompatibleOrOverBudgetShownAsReady => {
                M5MarketplaceResultDetailNextAction::ReviewCompatibility
            }
            Self::PermissionWideningHidden | Self::ActivationCostHidden => {
                M5MarketplaceResultDetailNextAction::ReviewPermissionAndBudget
            }
            Self::ProofStale => M5MarketplaceResultDetailNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5MarketplaceInstallDowngradeTrigger {
        match self {
            Self::ArtifactIdentityUnstated
            | Self::DocsChangelogIssuesUnlinked
            | Self::SupportClassUnresolved
            | Self::LifecycleStateUnstated => {
                M5MarketplaceInstallDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::RegistrySourceUnresolved | Self::SourceClassCollapsedIntoAmbiguousOrigin => {
                M5MarketplaceInstallDowngradeTrigger::RegistrySourceClassCollapsed
            }
            Self::CompatibilityUnresolved
            | Self::VersionRangeUnstated
            | Self::IncompatibleOrOverBudgetShownAsReady => {
                M5MarketplaceInstallDowngradeTrigger::CompatibilityRangeUnstated
            }
            Self::PermissionWideningHidden => {
                M5MarketplaceInstallDowngradeTrigger::PermissionWideningHidden
            }
            Self::ActivationCostHidden => {
                M5MarketplaceInstallDowngradeTrigger::ActivationCostHidden
            }
            Self::PublisherTransferHidden => {
                M5MarketplaceInstallDowngradeTrigger::PublisherTransferHidden
            }
            Self::ProofStale => M5MarketplaceInstallDowngradeTrigger::ProofStale,
        }
    }
}

/// Maps a registry source class to the single controlled marketplace disposition, or `None` when
/// the source cannot be resolved — an unresolved source never borrows a public / mirrored /
/// enterprise word.
fn disposition_for_source(
    source: M5RegistrySourceClass,
) -> Option<M5MarketplaceInstallDisposition> {
    use M5MarketplaceInstallDisposition as D;
    match source {
        M5RegistrySourceClass::PublicRegistry => Some(D::Public),
        M5RegistrySourceClass::MirroredRegistry => Some(D::Mirrored),
        M5RegistrySourceClass::EnterpriseRegistry => Some(D::Enterprise),
        M5RegistrySourceClass::SideLoaded => Some(D::SideLoad),
        M5RegistrySourceClass::VerifiedPartner => Some(D::Verified),
        M5RegistrySourceClass::SourceUnknown => None,
    }
}

/// True when the compatibility state reads as freely installable.
fn compatibility_is_installable(state: M5CompatibilityState) -> bool {
    matches!(
        state,
        M5CompatibilityState::Compatible | M5CompatibilityState::CompatibleWithWarnings
    )
}

/// True when the compatibility state is an incompatible / degraded one.
fn compatibility_is_incompatible(state: M5CompatibilityState) -> bool {
    matches!(
        state,
        M5CompatibilityState::Incompatible
            | M5CompatibilityState::DegradedHost
            | M5CompatibilityState::UnsupportedRuntime
    )
}

/// True when the activation-budget band reads as within budget.
fn budget_is_within(band: M5ActivationBudgetBandState) -> bool {
    matches!(
        band,
        M5ActivationBudgetBandState::WithinBudget | M5ActivationBudgetBandState::NearBudget
    )
}

/// True when the activation-budget band is an over-budget / throttled one that carries a cost the
/// row must state.
fn budget_is_over(band: M5ActivationBudgetBandState) -> bool {
    matches!(
        band,
        M5ActivationBudgetBandState::OverBudget
            | M5ActivationBudgetBandState::Throttled
            | M5ActivationBudgetBandState::SuspendedOverBudget
    )
}

/// True when the activation-budget band carries a cost the component must state explicitly.
fn budget_states_cost(band: M5ActivationBudgetBandState) -> bool {
    !matches!(band, M5ActivationBudgetBandState::WithinBudget)
        && !matches!(band, M5ActivationBudgetBandState::BudgetUnknown)
}

/// True when the permission posture widens permissions beyond the standard set and must be stated.
fn posture_widens(posture: M5PermissionPostureState) -> bool {
    matches!(
        posture,
        M5PermissionPostureState::Elevated | M5PermissionPostureState::WidenedTransitive
    )
}

/// True when the publisher continuity represents a change the component must state.
fn publisher_changed(state: M5PublisherContinuityState) -> bool {
    matches!(
        state,
        M5PublisherContinuityState::Transferred
            | M5PublisherContinuityState::Deprecated
            | M5PublisherContinuityState::Abandoned
    )
}

/// The shared marketplace facts a result row and a detail fact grid must agree on for the same
/// artifact, so list and detail never present contradictory trust or support facts.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SharedFacts {
    registry_source: M5RegistrySourceClass,
    compatibility: M5CompatibilityState,
    host_runtime_model: M5HostRuntimeModel,
    permission_posture: M5PermissionPostureState,
    activation_budget: M5ActivationBudgetBandState,
    publisher_continuity: M5PublisherContinuityState,
    trust_tier: M5MarketplaceTrustTier,
}

/// Input to [`resolve_marketplace_result_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MarketplaceResultRowResolutionInput {
    /// Stable identity of the row instance.
    pub row_id: String,
    /// The artifact identity (name / id) shown; empty means unstated.
    pub artifact_identity: String,
    /// Where the artifact comes from.
    pub registry_source: M5RegistrySourceClass,
    /// The compatibility state.
    pub compatibility: M5CompatibilityState,
    /// The host / runtime model.
    pub host_runtime_model: M5HostRuntimeModel,
    /// The permission posture.
    pub permission_posture: M5PermissionPostureState,
    /// True when a widened permission posture is stated on the row, never menu-only.
    pub permission_widening_stated: bool,
    /// The activation-budget band (performance evidence).
    pub activation_budget: M5ActivationBudgetBandState,
    /// True when a costful activation band is stated on the row.
    pub activation_cost_stated: bool,
    /// The support / trust tier.
    pub trust_tier: M5MarketplaceTrustTier,
    /// The publisher continuity.
    pub publisher_continuity: M5PublisherContinuityState,
    /// True when a publisher change (transfer / deprecation) is stated on the row.
    pub publisher_change_stated: bool,
    /// True when the row reads the source class as one ambiguous origin across public / mirrored /
    /// enterprise.
    pub collapses_source_class: bool,
    /// True when the row reads an incompatible / over-budget artifact as ready to install.
    pub reads_incompatible_or_over_budget_as_ready: bool,
    /// True when a command-backed detail entrypoint is reachable, never menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe marketplace result row projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedMarketplaceResultRow {
    /// Stable identity of the row instance.
    pub row_id: String,
    /// The artifact identity named by the row.
    pub artifact_identity: String,
    /// The registry source token named by the row.
    pub registry_source: String,
    /// Single controlled source disposition, or `null` when the source is unresolved.
    pub source_disposition: Option<M5MarketplaceInstallDisposition>,
    /// The compatibility token named by the row.
    pub compatibility: String,
    /// Whether the artifact reads as installable (compatible and within budget).
    pub is_installable: bool,
    /// The host / runtime token named by the row.
    pub host_runtime_model: String,
    /// The permission-posture token named by the row.
    pub permission_posture: String,
    /// Whether the permission posture widens beyond standard.
    pub permission_widened: bool,
    /// The activation-budget token named by the row.
    pub activation_budget: String,
    /// Whether the artifact is within its activation budget.
    pub within_activation_budget: bool,
    /// The support / trust-tier token named by the row.
    pub trust_tier: String,
    /// The publisher-continuity token named by the row.
    pub publisher_continuity: String,
    /// Whether the publisher continuity represents a change.
    pub publisher_changed: bool,
    /// Guardrail (MUST be `false` on a clean row): the source class is collapsed into one origin.
    pub collapses_source_class: bool,
    /// Guardrail (MUST be `false` on a clean row): an incompatible / over-budget artifact reads as
    /// ready to install.
    pub presents_incompatible_or_over_budget_as_ready: bool,
    /// Whether a command-backed detail entrypoint is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the row could not read as a clean, compare-at-a-glance state.
    pub degrade_reason: Option<M5MarketplaceResultRowDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5MarketplaceResultDetailNextAction,
    /// Whether the marketplace facts are comparable at a glance (clean row naming every fact).
    pub comparable_at_a_glance: bool,
}

impl M5ResolvedMarketplaceResultRow {
    /// Whether this row reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    fn shared_facts(&self) -> Option<SharedFacts> {
        shared_facts_from_tokens(
            &self.registry_source,
            &self.compatibility,
            &self.host_runtime_model,
            &self.permission_posture,
            &self.activation_budget,
            &self.publisher_continuity,
            &self.trust_tier,
        )
    }
}

/// Input to [`resolve_marketplace_detail_fact_grid`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MarketplaceDetailFactGridResolutionInput {
    /// Stable identity of the grid instance.
    pub grid_id: String,
    /// The artifact identity (name / id) shown; empty means unstated.
    pub artifact_identity: String,
    /// Where the artifact comes from.
    pub registry_source: M5RegistrySourceClass,
    /// The compatibility state.
    pub compatibility: M5CompatibilityState,
    /// The host / runtime model.
    pub host_runtime_model: M5HostRuntimeModel,
    /// The permission posture.
    pub permission_posture: M5PermissionPostureState,
    /// True when a widened permission posture is stated on the grid.
    pub permission_widening_stated: bool,
    /// The activation-budget band (performance evidence).
    pub activation_budget: M5ActivationBudgetBandState,
    /// True when a costful activation band is stated on the grid.
    pub activation_cost_stated: bool,
    /// The support / trust tier.
    pub trust_tier: M5MarketplaceTrustTier,
    /// The publisher continuity.
    pub publisher_continuity: M5PublisherContinuityState,
    /// True when a publisher change (transfer / deprecation) is stated on the grid.
    pub publisher_change_stated: bool,
    /// The richer version range; empty means unstated.
    pub version_range: String,
    /// The lifecycle state.
    pub lifecycle: M5MarketplaceLifecycleState,
    /// True when a docs link is present.
    pub docs_linked: bool,
    /// True when a changelog link is present.
    pub changelog_linked: bool,
    /// True when an open-issues link is present.
    pub open_issues_linked: bool,
    /// True when the grid reads the source class as one ambiguous origin.
    pub collapses_source_class: bool,
    /// True when the grid reads an incompatible / over-budget artifact as ready to install.
    pub reads_incompatible_or_over_budget_as_ready: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe marketplace detail fact grid projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedMarketplaceDetailFactGrid {
    /// Stable identity of the grid instance.
    pub grid_id: String,
    /// The artifact identity named by the grid.
    pub artifact_identity: String,
    /// The registry source token named by the grid.
    pub registry_source: String,
    /// Single controlled source disposition, or `null` when the source is unresolved.
    pub source_disposition: Option<M5MarketplaceInstallDisposition>,
    /// The compatibility token named by the grid.
    pub compatibility: String,
    /// Whether the artifact reads as installable.
    pub is_installable: bool,
    /// The host / runtime token named by the grid.
    pub host_runtime_model: String,
    /// The permission-posture token named by the grid.
    pub permission_posture: String,
    /// Whether the permission posture widens beyond standard.
    pub permission_widened: bool,
    /// The activation-budget token named by the grid.
    pub activation_budget: String,
    /// Whether the artifact is within its activation budget.
    pub within_activation_budget: bool,
    /// The support / trust-tier token named by the grid.
    pub trust_tier: String,
    /// The publisher-continuity token named by the grid.
    pub publisher_continuity: String,
    /// Whether the publisher continuity represents a change.
    pub publisher_changed: bool,
    /// The richer version range named by the grid.
    pub version_range: String,
    /// The lifecycle token named by the grid.
    pub lifecycle: String,
    /// Whether a docs link is present.
    pub docs_linked: bool,
    /// Whether a changelog link is present.
    pub changelog_linked: bool,
    /// Whether an open-issues link is present.
    pub open_issues_linked: bool,
    /// Guardrail (MUST be `false` on a clean grid): the source class is collapsed into one origin.
    pub collapses_source_class: bool,
    /// Guardrail (MUST be `false` on a clean grid): an incompatible / over-budget artifact reads as
    /// ready to install.
    pub presents_incompatible_or_over_budget_as_ready: bool,
    /// Degrade reason, if the grid could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5MarketplaceDetailFactGridDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5MarketplaceResultDetailNextAction,
    /// Whether the marketplace facts are legible in full (clean grid naming every fact).
    pub fully_legible: bool,
}

impl M5ResolvedMarketplaceDetailFactGrid {
    /// Whether this grid reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    fn shared_facts(&self) -> Option<SharedFacts> {
        shared_facts_from_tokens(
            &self.registry_source,
            &self.compatibility,
            &self.host_runtime_model,
            &self.permission_posture,
            &self.activation_budget,
            &self.publisher_continuity,
            &self.trust_tier,
        )
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5MarketplaceResultDetailResolutionError {
    /// The row id was empty.
    EmptyRowId,
    /// The grid id was empty.
    EmptyGridId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5MarketplaceResultDetailResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyRowId => "empty_row_id",
            Self::EmptyGridId => "empty_grid_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5MarketplaceResultDetailResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 marketplace-result-detail resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5MarketplaceResultDetailResolutionError {}

/// Resolves a marketplace result row, making compare decisions possible without opening the detail
/// page: the row names its source class, compatibility, runtime model, permission posture,
/// activation budget, support class, and publisher continuity, never collapses the source class,
/// and never reads an incompatible or over-budget artifact as ready to install.
pub fn resolve_marketplace_result_row(
    input: M5MarketplaceResultRowResolutionInput,
) -> Result<M5ResolvedMarketplaceResultRow, M5MarketplaceResultDetailResolutionError> {
    if input.row_id.trim().is_empty() {
        return Err(M5MarketplaceResultDetailResolutionError::EmptyRowId);
    }
    if string_is_forbidden(&input.row_id) || string_is_forbidden(&input.artifact_identity) {
        return Err(M5MarketplaceResultDetailResolutionError::ForbiddenMaterial);
    }

    let is_installable = compatibility_is_installable(input.compatibility)
        && budget_is_within(input.activation_budget);
    let permission_widened = posture_widens(input.permission_posture);
    let within_activation_budget = budget_is_within(input.activation_budget);
    let publisher_changed_now = publisher_changed(input.publisher_continuity);
    let is_incompatible_or_over = compatibility_is_incompatible(input.compatibility)
        || budget_is_over(input.activation_budget);
    let presents_incompatible_or_over_budget_as_ready =
        is_incompatible_or_over && input.reads_incompatible_or_over_budget_as_ready;

    let degrade_reason = if input.artifact_identity.trim().is_empty() {
        Some(M5MarketplaceResultRowDegradeReason::ArtifactIdentityUnstated)
    } else if matches!(input.registry_source, M5RegistrySourceClass::SourceUnknown) {
        Some(M5MarketplaceResultRowDegradeReason::RegistrySourceUnresolved)
    } else if input.collapses_source_class {
        Some(M5MarketplaceResultRowDegradeReason::SourceClassCollapsedIntoAmbiguousOrigin)
    } else if matches!(
        input.compatibility,
        M5CompatibilityState::CompatibilityUnknown
    ) {
        Some(M5MarketplaceResultRowDegradeReason::CompatibilityUnresolved)
    } else if presents_incompatible_or_over_budget_as_ready {
        Some(M5MarketplaceResultRowDegradeReason::IncompatibleOrOverBudgetShownAsReady)
    } else if permission_widened && !input.permission_widening_stated {
        Some(M5MarketplaceResultRowDegradeReason::PermissionWideningHidden)
    } else if budget_states_cost(input.activation_budget) && !input.activation_cost_stated {
        Some(M5MarketplaceResultRowDegradeReason::ActivationCostHidden)
    } else if matches!(input.trust_tier, M5MarketplaceTrustTier::TierUnknown) {
        Some(M5MarketplaceResultRowDegradeReason::SupportClassUnresolved)
    } else if publisher_changed_now && !input.publisher_change_stated {
        Some(M5MarketplaceResultRowDegradeReason::PublisherTransferHidden)
    } else if !input.detail_command_available {
        Some(M5MarketplaceResultRowDegradeReason::DetailPathMissing)
    } else if !input.proof_fresh {
        Some(M5MarketplaceResultRowDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5MarketplaceResultDetailNextAction::OpenDetail,
    };

    Ok(M5ResolvedMarketplaceResultRow {
        row_id: input.row_id,
        artifact_identity: input.artifact_identity,
        registry_source: source_token(input.registry_source),
        source_disposition: disposition_for_source(input.registry_source),
        compatibility: input.compatibility.as_str().to_owned(),
        is_installable,
        host_runtime_model: input.host_runtime_model.as_str().to_owned(),
        permission_posture: input.permission_posture.as_str().to_owned(),
        permission_widened,
        activation_budget: input.activation_budget.as_str().to_owned(),
        within_activation_budget,
        trust_tier: input.trust_tier.as_str().to_owned(),
        publisher_continuity: input.publisher_continuity.as_str().to_owned(),
        publisher_changed: publisher_changed_now,
        collapses_source_class: input.collapses_source_class,
        presents_incompatible_or_over_budget_as_ready,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        comparable_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves a marketplace detail fact grid, exposing the richer facts a compare decision needs:
/// version range, lifecycle state, trust tier, and docs/changelog/open-issues linkage on top of the
/// same source/compatibility/permission/budget/publisher grammar the result row uses.
pub fn resolve_marketplace_detail_fact_grid(
    input: M5MarketplaceDetailFactGridResolutionInput,
) -> Result<M5ResolvedMarketplaceDetailFactGrid, M5MarketplaceResultDetailResolutionError> {
    if input.grid_id.trim().is_empty() {
        return Err(M5MarketplaceResultDetailResolutionError::EmptyGridId);
    }
    if string_is_forbidden(&input.grid_id)
        || string_is_forbidden(&input.artifact_identity)
        || string_is_forbidden(&input.version_range)
    {
        return Err(M5MarketplaceResultDetailResolutionError::ForbiddenMaterial);
    }

    let is_installable = compatibility_is_installable(input.compatibility)
        && budget_is_within(input.activation_budget);
    let permission_widened = posture_widens(input.permission_posture);
    let within_activation_budget = budget_is_within(input.activation_budget);
    let publisher_changed_now = publisher_changed(input.publisher_continuity);
    let is_incompatible_or_over = compatibility_is_incompatible(input.compatibility)
        || budget_is_over(input.activation_budget);
    let presents_incompatible_or_over_budget_as_ready =
        is_incompatible_or_over && input.reads_incompatible_or_over_budget_as_ready;
    let any_link = input.docs_linked || input.changelog_linked || input.open_issues_linked;

    let degrade_reason = if input.artifact_identity.trim().is_empty() {
        Some(M5MarketplaceDetailFactGridDegradeReason::ArtifactIdentityUnstated)
    } else if matches!(input.registry_source, M5RegistrySourceClass::SourceUnknown) {
        Some(M5MarketplaceDetailFactGridDegradeReason::RegistrySourceUnresolved)
    } else if input.collapses_source_class {
        Some(M5MarketplaceDetailFactGridDegradeReason::SourceClassCollapsedIntoAmbiguousOrigin)
    } else if matches!(
        input.compatibility,
        M5CompatibilityState::CompatibilityUnknown
    ) {
        Some(M5MarketplaceDetailFactGridDegradeReason::CompatibilityUnresolved)
    } else if input.version_range.trim().is_empty() {
        Some(M5MarketplaceDetailFactGridDegradeReason::VersionRangeUnstated)
    } else if presents_incompatible_or_over_budget_as_ready {
        Some(M5MarketplaceDetailFactGridDegradeReason::IncompatibleOrOverBudgetShownAsReady)
    } else if permission_widened && !input.permission_widening_stated {
        Some(M5MarketplaceDetailFactGridDegradeReason::PermissionWideningHidden)
    } else if budget_states_cost(input.activation_budget) && !input.activation_cost_stated {
        Some(M5MarketplaceDetailFactGridDegradeReason::ActivationCostHidden)
    } else if matches!(input.trust_tier, M5MarketplaceTrustTier::TierUnknown) {
        Some(M5MarketplaceDetailFactGridDegradeReason::SupportClassUnresolved)
    } else if publisher_changed_now && !input.publisher_change_stated {
        Some(M5MarketplaceDetailFactGridDegradeReason::PublisherTransferHidden)
    } else if matches!(
        input.lifecycle,
        M5MarketplaceLifecycleState::LifecycleUnknown
    ) {
        Some(M5MarketplaceDetailFactGridDegradeReason::LifecycleStateUnstated)
    } else if !any_link {
        Some(M5MarketplaceDetailFactGridDegradeReason::DocsChangelogIssuesUnlinked)
    } else if !input.proof_fresh {
        Some(M5MarketplaceDetailFactGridDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5MarketplaceResultDetailNextAction::OpenDetail,
    };

    Ok(M5ResolvedMarketplaceDetailFactGrid {
        grid_id: input.grid_id,
        artifact_identity: input.artifact_identity,
        registry_source: source_token(input.registry_source),
        source_disposition: disposition_for_source(input.registry_source),
        compatibility: input.compatibility.as_str().to_owned(),
        is_installable,
        host_runtime_model: input.host_runtime_model.as_str().to_owned(),
        permission_posture: input.permission_posture.as_str().to_owned(),
        permission_widened,
        activation_budget: input.activation_budget.as_str().to_owned(),
        within_activation_budget,
        trust_tier: input.trust_tier.as_str().to_owned(),
        publisher_continuity: input.publisher_continuity.as_str().to_owned(),
        publisher_changed: publisher_changed_now,
        version_range: input.version_range,
        lifecycle: input.lifecycle.as_str().to_owned(),
        docs_linked: input.docs_linked,
        changelog_linked: input.changelog_linked,
        open_issues_linked: input.open_issues_linked,
        collapses_source_class: input.collapses_source_class,
        presents_incompatible_or_over_budget_as_ready,
        degrade_reason,
        next_action,
        fully_legible: degrade_reason.is_none(),
    })
}

/// Renders the registry-source token (kept as a helper so the resolver and the fact-parity check
/// share one spelling).
fn source_token(source: M5RegistrySourceClass) -> String {
    source.as_str().to_owned()
}

/// Rebuilds the typed [`SharedFacts`] from the resolved string tokens so a result row and a detail
/// fact grid describing the same artifact can be compared for parity. Returns `None` if any token is
/// unrecognized (which never happens for resolver output).
fn shared_facts_from_tokens(
    registry_source: &str,
    compatibility: &str,
    host_runtime_model: &str,
    permission_posture: &str,
    activation_budget: &str,
    publisher_continuity: &str,
    trust_tier: &str,
) -> Option<SharedFacts> {
    let registry_source = M5RegistrySourceClass::ALL
        .into_iter()
        .find(|v| v.as_str() == registry_source)?;
    let compatibility = M5CompatibilityState::ALL
        .into_iter()
        .find(|v| v.as_str() == compatibility)?;
    let host_runtime_model = M5HostRuntimeModel::ALL
        .into_iter()
        .find(|v| v.as_str() == host_runtime_model)?;
    let permission_posture = M5PermissionPostureState::ALL
        .into_iter()
        .find(|v| v.as_str() == permission_posture)?;
    let activation_budget = M5ActivationBudgetBandState::ALL
        .into_iter()
        .find(|v| v.as_str() == activation_budget)?;
    let publisher_continuity = M5PublisherContinuityState::ALL
        .into_iter()
        .find(|v| v.as_str() == publisher_continuity)?;
    let trust_tier = M5MarketplaceTrustTier::ALL
        .into_iter()
        .find(|v| v.as_str() == trust_tier)?;
    Some(SharedFacts {
        registry_source,
        compatibility,
        host_runtime_model,
        permission_posture,
        activation_budget,
        publisher_continuity,
        trust_tier,
    })
}

/// One controls row: one consumer surface bound to the resolved marketplace result row and detail
/// fact grid examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceResultDetailControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5MarketplaceResultDetailConsumerSurface,
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
    pub anatomy_parts: Vec<M5MarketplaceResultDetailAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5MarketplaceResultDetailExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5MarketplaceInstallDowngradeTrigger>,
    /// Resolved marketplace result row examples.
    pub marketplace_result_row_examples: Vec<M5ResolvedMarketplaceResultRow>,
    /// Resolved marketplace detail fact grid examples.
    pub marketplace_detail_fact_grid_examples: Vec<M5ResolvedMarketplaceDetailFactGrid>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never collapse the registry source class across public / mirrored /
    /// enterprise.
    pub collapses_registry_source_class_across_public_mirrored_enterprise: bool,
    /// Hard invariant: never hide permission widening or activation cost behind compact chrome.
    pub hides_permission_widening_or_activation_cost: bool,
    /// Hard invariant: never hide a publisher transfer or deprecation.
    pub hides_publisher_transfer_or_deprecation: bool,
    /// Hard invariant: never present an incompatible or over-budget artifact as ready to install.
    pub presents_incompatible_or_over_budget_as_ready: bool,
}

impl M5MarketplaceResultDetailControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5MarketplaceResultDetailAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5MarketplaceResultDetailAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5MarketplaceResultDetailExportField> =
            self.export_fields.iter().copied().collect();
        M5MarketplaceResultDetailExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.collapses_registry_source_class_across_public_mirrored_enterprise
            && !self.hides_permission_widening_or_activation_cost
            && !self.hides_publisher_transfer_or_deprecation
            && !self.presents_incompatible_or_over_budget_as_ready
    }

    /// True when every resolved example on this row is honest: no clean row or grid collapses the
    /// source class or presents an incompatible / over-budget artifact as ready, and no clean row
    /// hides its detail path.
    fn examples_are_honest(&self) -> bool {
        self.marketplace_result_row_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.collapses_source_class
                    || ex.presents_incompatible_or_over_budget_as_ready
                    || !ex.detail_command_available))
        }) && self.marketplace_detail_fact_grid_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.collapses_source_class || ex.presents_incompatible_or_over_budget_as_ready))
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceResultDetailVocabularySet {
    /// Marketplace / install-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Registry source-class tokens (bound from the frozen matrix).
    pub registry_source_classes: Vec<String>,
    /// Compatibility-state tokens (bound from the frozen matrix).
    pub compatibility_states: Vec<String>,
    /// Host / runtime-model tokens (bound from the frozen matrix).
    pub host_runtime_models: Vec<String>,
    /// Permission-posture tokens (bound from the frozen matrix).
    pub permission_postures: Vec<String>,
    /// Activation-budget-band tokens (bound from the frozen matrix).
    pub activation_budget_bands: Vec<String>,
    /// Publisher-continuity tokens (bound from the frozen matrix).
    pub publisher_continuity_states: Vec<String>,
    /// Support / trust-tier tokens (minted by this lane).
    pub trust_tiers: Vec<String>,
    /// Lifecycle-state tokens (minted by this lane).
    pub lifecycle_states: Vec<String>,
    /// Result-row degrade-reason tokens.
    pub result_row_degrade_reasons: Vec<String>,
    /// Detail-fact-grid degrade-reason tokens.
    pub detail_fact_grid_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5MarketplaceResultDetailVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5MarketplaceInstallDisposition::ALL, |v| v.as_str()),
            registry_source_classes: tokens(&M5RegistrySourceClass::ALL, |v| v.as_str()),
            compatibility_states: tokens(&M5CompatibilityState::ALL, |v| v.as_str()),
            host_runtime_models: tokens(&M5HostRuntimeModel::ALL, |v| v.as_str()),
            permission_postures: tokens(&M5PermissionPostureState::ALL, |v| v.as_str()),
            activation_budget_bands: tokens(&M5ActivationBudgetBandState::ALL, |v| v.as_str()),
            publisher_continuity_states: tokens(&M5PublisherContinuityState::ALL, |v| v.as_str()),
            trust_tiers: tokens(&M5MarketplaceTrustTier::ALL, |v| v.as_str()),
            lifecycle_states: tokens(&M5MarketplaceLifecycleState::ALL, |v| v.as_str()),
            result_row_degrade_reasons: tokens(&M5MarketplaceResultRowDegradeReason::ALL, |v| {
                v.as_str()
            }),
            detail_fact_grid_degrade_reasons: tokens(
                &M5MarketplaceDetailFactGridDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5MarketplaceResultDetailAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5MarketplaceResultDetailNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5MarketplaceResultDetailExportField::ALL, |v| v.as_str()),
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
pub struct M5MarketplaceResultDetailGovernanceReview {
    /// The result row names its source class, compatibility, and runtime model.
    pub row_names_source_compatibility_and_runtime: bool,
    /// The result row names its permission posture and activation budget.
    pub row_names_permission_and_budget: bool,
    /// The detail fact grid names its version range, lifecycle state, and trust tier.
    pub grid_names_version_lifecycle_and_tier: bool,
    /// The detail fact grid names its docs / changelog / open-issues linkage.
    pub grid_names_docs_changelog_and_issues: bool,
    /// The registry source class is always explicit, never collapsed into one origin.
    pub source_class_always_explicit_never_collapsed: bool,
    /// Permission widening and activation cost are always named, never hidden behind chrome.
    pub permission_and_cost_always_named: bool,
    /// An incompatible or over-budget artifact is never presented as ready to install.
    pub incompatible_or_over_budget_never_ready: bool,
    /// List and detail views share one fact grammar and source vocabulary.
    pub list_and_detail_share_one_fact_grammar: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceResultDetailConsumerProjection {
    /// Marketplace surfaces consume the shared source / compatibility vocabulary.
    pub marketplace_surfaces_consume_source_and_compatibility_vocabulary: bool,
    /// Registry surfaces consume the shared permission / budget / publisher vocabulary.
    pub registry_surfaces_consume_permission_budget_publisher_vocabulary: bool,
    /// Marketplace facts trace back to one canonical component contract.
    pub marketplace_facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical marketplace source.
    pub support_export_reads_single_marketplace_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceResultDetailProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceResultDetailReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5MarketplaceResultDetailControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MarketplaceResultDetailControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5MarketplaceResultDetailControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MarketplaceResultDetailVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MarketplaceResultDetailGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MarketplaceResultDetailConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MarketplaceResultDetailProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MarketplaceResultDetailReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 marketplace-result-row / detail-fact-grid controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceResultDetailControlsPacket {
    /// Record kind; must equal [`M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5MarketplaceResultDetailControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MarketplaceResultDetailVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MarketplaceResultDetailGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MarketplaceResultDetailConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MarketplaceResultDetailProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MarketplaceResultDetailReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5MarketplaceResultDetailControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5MarketplaceResultDetailControlsPacketInput) -> Self {
        Self {
            record_kind: M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5MarketplaceResultDetailControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_RECORD_KIND {
            violations.push(M5MarketplaceResultDetailControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_SCHEMA_VERSION {
            violations.push(M5MarketplaceResultDetailControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5MarketplaceResultDetailControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5MarketplaceResultDetailControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 marketplace-result-detail controls packet serializes"),
        ) {
            violations.push(M5MarketplaceResultDetailControlsViolation::RawMaterialInExport);
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
            .expect("m5 marketplace-result-detail controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,row_examples,grid_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .marketplace_result_row_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.marketplace_detail_fact_grid_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.marketplace_result_row_examples.len(),
                row.marketplace_detail_fact_grid_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Marketplace-Result-Row and Marketplace-Detail-Fact-Grid Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Registry source classes: {}\n",
            self.vocabulary_set.registry_source_classes.join(", ")
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
                "  - Result-row examples: {} / detail-fact-grid examples: {}\n",
                row.marketplace_result_row_examples.len(),
                row.marketplace_detail_fact_grid_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5MarketplaceResultDetailControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5MarketplaceResultDetailControlsViolation>),
}

impl fmt::Display for M5MarketplaceResultDetailControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 marketplace-result-detail controls export parse failed: {error}"
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
                    "m5 marketplace-result-detail controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5MarketplaceResultDetailControlsArtifactError {}

/// Validation failures emitted by [`M5MarketplaceResultDetailControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5MarketplaceResultDetailControlsViolation {
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
    /// A controls row carries a dishonest clean example (collapse, false-ready, or hidden detail).
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
    /// Source-class honesty is not proven: clean examples do not cover public / mirrored /
    /// enterprise source classes, or no collapse / false-ready example degrades.
    SourceClassHonestyNotProven,
    /// List/detail parity is not proven: no artifact appears clean in both a result row and a detail
    /// fact grid with matching facts, or a shared artifact presents contradictory facts.
    ListDetailParityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5MarketplaceResultDetailControlsViolation {
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
            Self::SourceClassHonestyNotProven => "source_class_honesty_not_proven",
            Self::ListDetailParityNotProven => "list_detail_parity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_marketplace_result_detail_controls_export(
) -> Result<M5MarketplaceResultDetailControlsPacket, M5MarketplaceResultDetailControlsArtifactError>
{
    let packet: M5MarketplaceResultDetailControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-marketplace-result-row-detail-fact-grid-controls-proof/support_export.json"
    )))
    .map_err(M5MarketplaceResultDetailControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5MarketplaceResultDetailControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5MarketplaceResultDetailControlsPacket,
    violations: &mut Vec<M5MarketplaceResultDetailControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_SCHEMA_REF,
        M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_DOC_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
        M5_MARKETPLACE_RESULT_ROW_SCHEMA_REF,
        M5_MARKETPLACE_DETAIL_FACT_GRID_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5MarketplaceResultDetailControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5MarketplaceResultDetailControlsPacket,
    violations: &mut Vec<M5MarketplaceResultDetailControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5MarketplaceResultDetailControlsViolation::NoControlsRows);
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
            violations.push(M5MarketplaceResultDetailControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5MarketplaceResultDetailControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5MarketplaceResultDetailControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_MARKETPLACE_RESULT_ROW_SCHEMA_REF)
            || !refs.contains(M5_MARKETPLACE_DETAIL_FACT_GRID_SCHEMA_REF)
        {
            violations.push(M5MarketplaceResultDetailControlsViolation::ComponentSchemaRefMissing);
        }
        if row.marketplace_result_row_examples.is_empty()
            || row.marketplace_detail_fact_grid_examples.is_empty()
        {
            violations.push(M5MarketplaceResultDetailControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5MarketplaceResultDetailControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5MarketplaceResultDetailControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5MarketplaceResultDetailControlsPacket,
    violations: &mut Vec<M5MarketplaceResultDetailControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.row_names_source_compatibility_and_runtime,
        review.row_names_permission_and_budget,
        review.grid_names_version_lifecycle_and_tier,
        review.grid_names_docs_changelog_and_issues,
        review.source_class_always_explicit_never_collapsed,
        review.permission_and_cost_always_named,
        review.incompatible_or_over_budget_never_ready,
        review.list_and_detail_share_one_fact_grammar,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5MarketplaceResultDetailControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5MarketplaceResultDetailControlsPacket,
    violations: &mut Vec<M5MarketplaceResultDetailControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.marketplace_surfaces_consume_source_and_compatibility_vocabulary,
        projection.registry_surfaces_consume_permission_budget_publisher_vocabulary,
        projection.marketplace_facts_trace_to_single_component_contract,
        projection.support_export_reads_single_marketplace_source,
    ] {
        if !ok {
            violations
                .push(M5MarketplaceResultDetailControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5MarketplaceResultDetailControlsPacket,
    violations: &mut Vec<M5MarketplaceResultDetailControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5MarketplaceResultDetailControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5MarketplaceResultDetailControlsPacket,
    violations: &mut Vec<M5MarketplaceResultDetailControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5MarketplaceResultDetailControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5MarketplaceResultDetailControlsPacket,
    violations: &mut Vec<M5MarketplaceResultDetailControlsViolation>,
) {
    let rows = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.marketplace_result_row_examples.iter())
    };
    let grids = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.marketplace_detail_fact_grid_examples.iter())
    };

    // AC: users can compare key facts and the registry source class stays explicit. Clean examples
    // cover the public, mirrored, and enterprise source dispositions, a collapse example degrades,
    // an incompatible / over-budget example degrades (never reads as ready), and no clean example
    // collapses the source class or presents a false-ready artifact.
    let clean_dispositions: BTreeSet<M5MarketplaceInstallDisposition> = rows()
        .filter(|ex| ex.is_clean())
        .filter_map(|ex| ex.source_disposition)
        .chain(
            grids()
                .filter(|ex| ex.is_clean())
                .filter_map(|ex| ex.source_disposition),
        )
        .collect();
    let covers_required_sources = [
        M5MarketplaceInstallDisposition::Public,
        M5MarketplaceInstallDisposition::Mirrored,
        M5MarketplaceInstallDisposition::Enterprise,
    ]
    .iter()
    .all(|disp| clean_dispositions.contains(disp));
    let collapse_degrades = rows().any(|ex| {
        ex.degrade_reason
            == Some(M5MarketplaceResultRowDegradeReason::SourceClassCollapsedIntoAmbiguousOrigin)
    }) || grids().any(|ex| {
        ex.degrade_reason
            == Some(
                M5MarketplaceDetailFactGridDegradeReason::SourceClassCollapsedIntoAmbiguousOrigin,
            )
    });
    let false_ready_degrades = rows().any(|ex| {
        ex.degrade_reason
            == Some(M5MarketplaceResultRowDegradeReason::IncompatibleOrOverBudgetShownAsReady)
    }) || grids().any(|ex| {
        ex.degrade_reason
            == Some(M5MarketplaceDetailFactGridDegradeReason::IncompatibleOrOverBudgetShownAsReady)
    });
    let no_clean_collapse_or_false_ready = rows().all(|ex| {
        !(ex.is_clean()
            && (ex.collapses_source_class || ex.presents_incompatible_or_over_budget_as_ready))
    }) && grids().all(|ex| {
        !(ex.is_clean()
            && (ex.collapses_source_class || ex.presents_incompatible_or_over_budget_as_ready))
    });
    if !(covers_required_sources
        && collapse_degrades
        && false_ready_degrades
        && no_clean_collapse_or_false_ready)
    {
        violations.push(M5MarketplaceResultDetailControlsViolation::SourceClassHonestyNotProven);
    }

    // AC: list and detail share one fact grammar. At least one artifact appears clean in both a
    // result row and a detail fact grid, and every clean row/grid pair with a shared artifact
    // identity agrees on the source class, compatibility, host model, permission posture, activation
    // budget, publisher continuity, and trust tier.
    let mut shared_pair_found = false;
    let mut parity_holds = true;
    for row_ex in rows().filter(|ex| ex.is_clean()) {
        for grid_ex in grids().filter(|ex| ex.is_clean()) {
            if row_ex.artifact_identity == grid_ex.artifact_identity {
                shared_pair_found = true;
                if row_ex.shared_facts() != grid_ex.shared_facts() {
                    parity_holds = false;
                }
            }
        }
    }
    if !(shared_pair_found && parity_holds) {
        violations.push(M5MarketplaceResultDetailControlsViolation::ListDetailParityNotProven);
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

/// The two component families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5MarketplaceInstallComponentFamily; 2] = [
    M5MarketplaceInstallComponentFamily::MarketplaceResultRow,
    M5MarketplaceInstallComponentFamily::MarketplaceDetailFactGrid,
];
