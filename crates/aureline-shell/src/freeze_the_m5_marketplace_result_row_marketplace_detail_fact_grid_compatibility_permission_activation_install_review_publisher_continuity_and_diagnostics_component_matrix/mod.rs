//! Frozen M5 marketplace-result-row, marketplace-detail-fact-grid, compatibility-label-strip,
//! permission-manifest-summary, activation-budget-band, install/update/disable/rollback
//! review-sheet, publisher-continuity-row, and installed-state-diagnostics-card component matrix.
//!
//! This module locks Aureline's reusable extension-marketplace and install-review UI components
//! into one export-safe packet. Every extension-marketplace or registry surface M5 claims that
//! still ships its own listing, detail, compatibility, permission, budget, install, publisher, or
//! diagnostics chrome — the marketplace result row, the marketplace detail fact grid, the
//! compatibility-label strip, the permission-manifest summary, the activation-budget band, the
//! install/update/disable/rollback review sheet, the publisher-continuity row, and the
//! installed-state diagnostics card — is named once here and constrained by the same registry
//! source class, compatibility, host/runtime model, permission posture, activation-budget band,
//! publisher continuity, disable scope, rollback compatibility, and quarantine vocabulary
//! regardless of the surface family that renders it.
//!
//! The matrix does not re-architect extension packaging, signing, registry transport, or the SDK
//! runtime — it is the shared marketplace-and-install-honesty component contract layered on top of
//! them. The controlled vocabularies are frozen in one self-describing
//! [`M5MarketplaceInstallVocabularySet`] rather than minted per surface. The single controlled
//! marketplace/install-disposition vocabulary consumers bind to — public, mirrored, enterprise,
//! side-load, verified, transferred, deprecated, limited, incompatible, over-budget, throttled,
//! quarantined, disable-scope, and rollback-compatibility — keeps compact marketplace chrome from
//! ever hiding permission widening or activation cost, keeps a publisher transfer or a disable
//! scope or a rollback incompatibility from staying implicit, and keeps public versus mirrored
//! versus enterprise source class explicit before mutation or help/export handoff. Raw secret
//! values and private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_marketplace_install_component_matrix,
    seeded_m5_marketplace_install_component_matrix_compatibility_label_strip_beta_narrowed,
    seeded_m5_marketplace_install_component_matrix_install_review_sheet_preview_narrowed,
    M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5MarketplaceInstallComponentMatrixPacket`].
pub const M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_label_strip_permission_manifest_summary_activation_budget_band_install_update_disable_rollback_review_sheet_publisher_continuity_row_and_installed_state_diagnostics_card_component_matrix";

/// Schema version for M5 marketplace-install component-matrix records.
pub const M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined marketplace-install component-matrix schema.
pub const M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-marketplace-install-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF: &str =
    "docs/marketplace/m5_marketplace_install_components_contract.md";

/// Repo-relative path of the marketplace-result-row canonical component schema.
pub const M5_MARKETPLACE_RESULT_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-marketplace-result-row.schema.json";

/// Repo-relative path of the marketplace-detail-fact-grid canonical component schema.
pub const M5_MARKETPLACE_DETAIL_FACT_GRID_SCHEMA_REF: &str =
    "schemas/ui/m5-marketplace-detail-fact-grid.schema.json";

/// Repo-relative path of the compatibility-label-strip canonical component schema.
pub const M5_COMPATIBILITY_LABEL_STRIP_SCHEMA_REF: &str =
    "schemas/ui/m5-compatibility-label-strip.schema.json";

/// Repo-relative path of the permission-manifest-summary canonical component schema.
pub const M5_PERMISSION_MANIFEST_SUMMARY_SCHEMA_REF: &str =
    "schemas/ui/m5-permission-manifest-summary.schema.json";

/// Repo-relative path of the activation-budget-band canonical component schema.
pub const M5_ACTIVATION_BUDGET_BAND_SCHEMA_REF: &str =
    "schemas/ui/m5-activation-budget-band.schema.json";

/// Repo-relative path of the install/update/disable/rollback review-sheet canonical component
/// schema.
pub const M5_INSTALL_UPDATE_DISABLE_ROLLBACK_REVIEW_SHEET_SCHEMA_REF: &str =
    "schemas/ui/m5-install-update-disable-rollback-review-sheet.schema.json";

/// Repo-relative path of the publisher-continuity-row canonical component schema.
pub const M5_PUBLISHER_CONTINUITY_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-publisher-continuity-row.schema.json";

/// Repo-relative path of the installed-state-diagnostics-card canonical component schema.
pub const M5_INSTALLED_STATE_DIAGNOSTICS_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-installed-state-diagnostics-card.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_MARKETPLACE_INSTALL_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-marketplace-install-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MARKETPLACE_INSTALL_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-marketplace-install-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_MARKETPLACE_INSTALL_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-marketplace-install-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_MARKETPLACE_INSTALL_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-marketplace-install-component-matrix.md";

/// One of the eight governed marketplace / install-review component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceInstallComponentFamily {
    /// A marketplace result row naming registry source class, compatibility, and publisher
    /// continuity for one listed artifact.
    MarketplaceResultRow,
    /// A marketplace detail fact grid naming source class, compatibility, host model, permission
    /// posture, activation budget, and publisher continuity together in one place.
    MarketplaceDetailFactGrid,
    /// A compatibility-label strip naming the compatibility range and runtime/host model.
    CompatibilityLabelStrip,
    /// A permission-manifest summary naming the permission posture and any transitive widening.
    PermissionManifestSummary,
    /// An activation-budget band naming whether the artifact is within, near, over budget, or
    /// throttled.
    ActivationBudgetBand,
    /// An install/update/disable/rollback review sheet naming disable scope and rollback
    /// compatibility before mutation.
    InstallUpdateDisableRollbackReviewSheet,
    /// A publisher-continuity row naming publisher transfer, deprecation, and source class.
    PublisherContinuityRow,
    /// An installed-state diagnostics card naming quarantine history, activation budget, and
    /// installed health.
    InstalledStateDiagnosticsCard,
}

impl M5MarketplaceInstallComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::MarketplaceResultRow,
        Self::MarketplaceDetailFactGrid,
        Self::CompatibilityLabelStrip,
        Self::PermissionManifestSummary,
        Self::ActivationBudgetBand,
        Self::InstallUpdateDisableRollbackReviewSheet,
        Self::PublisherContinuityRow,
        Self::InstalledStateDiagnosticsCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarketplaceResultRow => "marketplace_result_row",
            Self::MarketplaceDetailFactGrid => "marketplace_detail_fact_grid",
            Self::CompatibilityLabelStrip => "compatibility_label_strip",
            Self::PermissionManifestSummary => "permission_manifest_summary",
            Self::ActivationBudgetBand => "activation_budget_band",
            Self::InstallUpdateDisableRollbackReviewSheet => {
                "install_update_disable_rollback_review_sheet"
            }
            Self::PublisherContinuityRow => "publisher_continuity_row",
            Self::InstalledStateDiagnosticsCard => "installed_state_diagnostics_card",
        }
    }

    /// The canonical per-component schema ref a downstream row points at instead of restating this
    /// component's marketplace / install truth by hand.
    pub const fn canonical_component_schema_ref(self) -> &'static str {
        match self {
            Self::MarketplaceResultRow => M5_MARKETPLACE_RESULT_ROW_SCHEMA_REF,
            Self::MarketplaceDetailFactGrid => M5_MARKETPLACE_DETAIL_FACT_GRID_SCHEMA_REF,
            Self::CompatibilityLabelStrip => M5_COMPATIBILITY_LABEL_STRIP_SCHEMA_REF,
            Self::PermissionManifestSummary => M5_PERMISSION_MANIFEST_SUMMARY_SCHEMA_REF,
            Self::ActivationBudgetBand => M5_ACTIVATION_BUDGET_BAND_SCHEMA_REF,
            Self::InstallUpdateDisableRollbackReviewSheet => {
                M5_INSTALL_UPDATE_DISABLE_ROLLBACK_REVIEW_SHEET_SCHEMA_REF
            }
            Self::PublisherContinuityRow => M5_PUBLISHER_CONTINUITY_ROW_SCHEMA_REF,
            Self::InstalledStateDiagnosticsCard => M5_INSTALLED_STATE_DIAGNOSTICS_CARD_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled registry source class.
    pub const fn declares_registry_source(self) -> bool {
        matches!(
            self,
            Self::MarketplaceResultRow
                | Self::MarketplaceDetailFactGrid
                | Self::PublisherContinuityRow
        )
    }

    /// `true` when this family must name a controlled compatibility state.
    pub const fn declares_compatibility(self) -> bool {
        matches!(
            self,
            Self::MarketplaceResultRow
                | Self::MarketplaceDetailFactGrid
                | Self::CompatibilityLabelStrip
                | Self::InstalledStateDiagnosticsCard
        )
    }

    /// `true` when this family must name a controlled host / runtime model.
    pub const fn declares_host_model(self) -> bool {
        matches!(
            self,
            Self::MarketplaceDetailFactGrid | Self::CompatibilityLabelStrip
        )
    }

    /// `true` when this family must name a controlled permission posture.
    pub const fn declares_permission_posture(self) -> bool {
        matches!(
            self,
            Self::MarketplaceDetailFactGrid | Self::PermissionManifestSummary
        )
    }

    /// `true` when this family must name a controlled activation-budget band.
    pub const fn declares_activation_budget(self) -> bool {
        matches!(
            self,
            Self::MarketplaceDetailFactGrid
                | Self::ActivationBudgetBand
                | Self::InstalledStateDiagnosticsCard
        )
    }

    /// `true` when this family must name a controlled publisher-continuity state.
    pub const fn declares_publisher_continuity(self) -> bool {
        matches!(
            self,
            Self::MarketplaceResultRow
                | Self::MarketplaceDetailFactGrid
                | Self::PublisherContinuityRow
        )
    }

    /// `true` when this family must name a controlled disable-scope class.
    pub const fn declares_disable_scope(self) -> bool {
        matches!(self, Self::InstallUpdateDisableRollbackReviewSheet)
    }

    /// `true` when this family must name a controlled rollback-compatibility state.
    pub const fn declares_rollback_compat(self) -> bool {
        matches!(self, Self::InstallUpdateDisableRollbackReviewSheet)
    }

    /// `true` when this family must name a controlled quarantine state.
    pub const fn declares_quarantine(self) -> bool {
        matches!(self, Self::InstalledStateDiagnosticsCard)
    }
}

/// The single controlled marketplace / install-disposition vocabulary every extension-marketplace
/// or install-review consumer binds to. These are the exact acceptance-criteria tokens that keep
/// compact marketplace chrome from hiding permission widening, activation cost, publisher-transfer
/// risk, disable scope, or rollback incompatibility, and keep public versus mirrored versus
/// enterprise source class explicit. No marketplace or install surface invents a parallel word for
/// any of these dispositions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceInstallDisposition {
    /// The artifact comes from the public registry.
    Public,
    /// The artifact comes from a mirrored / offline registry.
    Mirrored,
    /// The artifact comes from an enterprise / managed registry.
    Enterprise,
    /// The artifact was side-loaded outside a registry.
    SideLoad,
    /// The artifact / publisher is verified.
    Verified,
    /// The publisher was transferred to a new owner.
    Transferred,
    /// The artifact is deprecated.
    Deprecated,
    /// The artifact runs with a limited or narrowed capability set.
    Limited,
    /// The artifact is incompatible with the current host or runtime.
    Incompatible,
    /// The artifact is over its activation budget.
    OverBudget,
    /// The artifact is throttled for exceeding its activation budget.
    Throttled,
    /// The artifact is quarantined.
    Quarantined,
    /// A disable action is scoped (workspace, global, profile) rather than a blanket removal.
    DisableScope,
    /// A rollback is bounded by its rollback-compatibility class.
    RollbackCompatibility,
}

impl M5MarketplaceInstallDisposition {
    /// Every disposition token, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::Public,
        Self::Mirrored,
        Self::Enterprise,
        Self::SideLoad,
        Self::Verified,
        Self::Transferred,
        Self::Deprecated,
        Self::Limited,
        Self::Incompatible,
        Self::OverBudget,
        Self::Throttled,
        Self::Quarantined,
        Self::DisableScope,
        Self::RollbackCompatibility,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Mirrored => "mirrored",
            Self::Enterprise => "enterprise",
            Self::SideLoad => "side_load",
            Self::Verified => "verified",
            Self::Transferred => "transferred",
            Self::Deprecated => "deprecated",
            Self::Limited => "limited",
            Self::Incompatible => "incompatible",
            Self::OverBudget => "over_budget",
            Self::Throttled => "throttled",
            Self::Quarantined => "quarantined",
            Self::DisableScope => "disable_scope",
            Self::RollbackCompatibility => "rollback_compatibility",
        }
    }

    /// Whether this disposition is the one clean verified state.
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }
}

/// Controlled registry source class — where the artifact comes from, so public, mirrored,
/// enterprise, and side-load sources are never collapsed into one ambiguous origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RegistrySourceClass {
    /// The public Aureline registry.
    PublicRegistry,
    /// A mirrored / offline registry.
    MirroredRegistry,
    /// An enterprise / managed private registry.
    EnterpriseRegistry,
    /// A side-loaded artifact outside any registry.
    SideLoaded,
    /// A verified first-party / partner source.
    VerifiedPartner,
    /// The registry source cannot currently be resolved.
    SourceUnknown,
}

impl M5RegistrySourceClass {
    /// Every registry source class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PublicRegistry,
        Self::MirroredRegistry,
        Self::EnterpriseRegistry,
        Self::SideLoaded,
        Self::VerifiedPartner,
        Self::SourceUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicRegistry => "public_registry",
            Self::MirroredRegistry => "mirrored_registry",
            Self::EnterpriseRegistry => "enterprise_registry",
            Self::SideLoaded => "side_loaded",
            Self::VerifiedPartner => "verified_partner",
            Self::SourceUnknown => "source_unknown",
        }
    }
}

/// Controlled compatibility state — whether the artifact fits the current host and runtime, so an
/// incompatible or degraded artifact never reads as freely installable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityState {
    /// Fully compatible with the current host and runtime.
    Compatible,
    /// Compatible but with warnings the user must read.
    CompatibleWithWarnings,
    /// Incompatible with the current host or runtime.
    Incompatible,
    /// The host is present but degraded for this artifact.
    DegradedHost,
    /// The required runtime is unsupported on this build.
    UnsupportedRuntime,
    /// The compatibility state cannot currently be resolved.
    CompatibilityUnknown,
}

impl M5CompatibilityState {
    /// Every compatibility state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Compatible,
        Self::CompatibleWithWarnings,
        Self::Incompatible,
        Self::DegradedHost,
        Self::UnsupportedRuntime,
        Self::CompatibilityUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::CompatibleWithWarnings => "compatible_with_warnings",
            Self::Incompatible => "incompatible",
            Self::DegradedHost => "degraded_host",
            Self::UnsupportedRuntime => "unsupported_runtime",
            Self::CompatibilityUnknown => "compatibility_unknown",
        }
    }
}

/// Controlled host / runtime model — how the artifact executes, so an in-process artifact is never
/// presented with the same isolation as a sandboxed or remote one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HostRuntimeModel {
    /// Runs in the main process.
    InProcess,
    /// Runs in a sandboxed worker.
    Sandboxed,
    /// Runs on a remote host.
    RemoteHost,
    /// Runs in a web worker.
    WebWorker,
    /// Runs in a native host process.
    NativeHost,
    /// The host / runtime model cannot currently be resolved.
    HostUnknown,
}

impl M5HostRuntimeModel {
    /// Every host / runtime model, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InProcess,
        Self::Sandboxed,
        Self::RemoteHost,
        Self::WebWorker,
        Self::NativeHost,
        Self::HostUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProcess => "in_process",
            Self::Sandboxed => "sandboxed",
            Self::RemoteHost => "remote_host",
            Self::WebWorker => "web_worker",
            Self::NativeHost => "native_host",
            Self::HostUnknown => "host_unknown",
        }
    }
}

/// Controlled permission posture — the permission set the artifact requests, so permission widening
/// and transitive widening are always named rather than hidden behind compact chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PermissionPostureState {
    /// Requests a minimal permission set.
    Minimal,
    /// Requests a standard permission set.
    Standard,
    /// Requests an elevated permission set.
    Elevated,
    /// Widens permissions transitively through its dependencies.
    WidenedTransitive,
    /// Permissions are restricted by policy.
    PolicyRestricted,
    /// The permission posture cannot currently be resolved.
    PostureUnknown,
}

impl M5PermissionPostureState {
    /// Every permission posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Minimal,
        Self::Standard,
        Self::Elevated,
        Self::WidenedTransitive,
        Self::PolicyRestricted,
        Self::PostureUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::Standard => "standard",
            Self::Elevated => "elevated",
            Self::WidenedTransitive => "widened_transitive",
            Self::PolicyRestricted => "policy_restricted",
            Self::PostureUnknown => "posture_unknown",
        }
    }
}

/// Controlled activation-budget band — the artifact's activation cost against its budget, so an
/// over-budget or throttled artifact never reads as cost-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActivationBudgetBandState {
    /// Well within the activation budget.
    WithinBudget,
    /// Near the activation budget.
    NearBudget,
    /// Over the activation budget.
    OverBudget,
    /// Throttled for exceeding the activation budget.
    Throttled,
    /// Suspended for exceeding the activation budget.
    SuspendedOverBudget,
    /// The activation-budget band cannot currently be resolved.
    BudgetUnknown,
}

impl M5ActivationBudgetBandState {
    /// Every activation-budget band, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WithinBudget,
        Self::NearBudget,
        Self::OverBudget,
        Self::Throttled,
        Self::SuspendedOverBudget,
        Self::BudgetUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinBudget => "within_budget",
            Self::NearBudget => "near_budget",
            Self::OverBudget => "over_budget",
            Self::Throttled => "throttled",
            Self::SuspendedOverBudget => "suspended_over_budget",
            Self::BudgetUnknown => "budget_unknown",
        }
    }
}

/// Controlled publisher-continuity state — the continuity of the publishing account, so a
/// transferred, deprecated, or abandoned publisher is never presented as continuous.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublisherContinuityState {
    /// The publisher is continuous with the original owner.
    Continuous,
    /// The publisher was transferred to a new owner.
    Transferred,
    /// The publisher deprecated the artifact.
    Deprecated,
    /// The publisher abandoned the artifact.
    Abandoned,
    /// The publisher is verified.
    VerifiedPublisher,
    /// The publisher continuity cannot currently be resolved.
    ContinuityUnknown,
}

impl M5PublisherContinuityState {
    /// Every publisher-continuity state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Continuous,
        Self::Transferred,
        Self::Deprecated,
        Self::Abandoned,
        Self::VerifiedPublisher,
        Self::ContinuityUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Continuous => "continuous",
            Self::Transferred => "transferred",
            Self::Deprecated => "deprecated",
            Self::Abandoned => "abandoned",
            Self::VerifiedPublisher => "verified_publisher",
            Self::ContinuityUnknown => "continuity_unknown",
        }
    }
}

/// Controlled disable-scope class — the scope a disable / uninstall action covers, so a
/// workspace-only disable is never mistaken for a blanket global removal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisableScopeClass {
    /// Disable in this workspace only.
    DisableWorkspace,
    /// Disable globally.
    DisableGlobal,
    /// Disable for this profile.
    DisableProfile,
    /// Fully uninstall.
    UninstallFull,
    /// Disable but keep user data.
    KeepDataDisable,
    /// The disable scope cannot currently be resolved.
    ScopeUnknown,
}

impl M5DisableScopeClass {
    /// Every disable-scope class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DisableWorkspace,
        Self::DisableGlobal,
        Self::DisableProfile,
        Self::UninstallFull,
        Self::KeepDataDisable,
        Self::ScopeUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisableWorkspace => "disable_workspace",
            Self::DisableGlobal => "disable_global",
            Self::DisableProfile => "disable_profile",
            Self::UninstallFull => "uninstall_full",
            Self::KeepDataDisable => "keep_data_disable",
            Self::ScopeUnknown => "scope_unknown",
        }
    }
}

/// Controlled rollback-compatibility state — how safely an update can be rolled back, so a
/// rollback with data loss or no prior version is never implied to be a clean revert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackCompatibilityState {
    /// Rollback restores the exact prior state.
    RollbackExact,
    /// Rollback is compatible but not byte-exact.
    RollbackCompatible,
    /// Rollback is incompatible with the current data.
    RollbackIncompatible,
    /// Rollback risks data loss.
    RollbackDataLoss,
    /// No prior version is available to roll back to.
    NoPriorVersion,
    /// The rollback-compatibility state cannot currently be resolved.
    RollbackUnknown,
}

impl M5RollbackCompatibilityState {
    /// Every rollback-compatibility state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RollbackExact,
        Self::RollbackCompatible,
        Self::RollbackIncompatible,
        Self::RollbackDataLoss,
        Self::NoPriorVersion,
        Self::RollbackUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RollbackExact => "rollback_exact",
            Self::RollbackCompatible => "rollback_compatible",
            Self::RollbackIncompatible => "rollback_incompatible",
            Self::RollbackDataLoss => "rollback_data_loss",
            Self::NoPriorVersion => "no_prior_version",
            Self::RollbackUnknown => "rollback_unknown",
        }
    }
}

/// Controlled quarantine state — the quarantine history of an installed artifact, so quarantine
/// history is never hidden behind an otherwise healthy diagnostics card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QuarantineState {
    /// Never quarantined.
    NotQuarantined,
    /// Currently quarantined.
    QuarantinedActive,
    /// Quarantined at some point in its history.
    QuarantinedHistory,
    /// Released from a prior quarantine.
    ReleasedFromQuarantine,
    /// Quarantine is pending review.
    QuarantinePending,
    /// The quarantine state cannot currently be resolved.
    QuarantineUnknown,
}

impl M5QuarantineState {
    /// Every quarantine state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotQuarantined,
        Self::QuarantinedActive,
        Self::QuarantinedHistory,
        Self::ReleasedFromQuarantine,
        Self::QuarantinePending,
        Self::QuarantineUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotQuarantined => "not_quarantined",
            Self::QuarantinedActive => "quarantined_active",
            Self::QuarantinedHistory => "quarantined_history",
            Self::ReleasedFromQuarantine => "released_from_quarantine",
            Self::QuarantinePending => "quarantine_pending",
            Self::QuarantineUnknown => "quarantine_unknown",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a marketplace-install component. No component
/// may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceInstallSurfaceFamily {
    /// The marketplace catalog / browse surface.
    MarketplaceCatalog,
    /// The extension manager.
    ExtensionManager,
    /// The registry admin surface.
    RegistryAdmin,
    /// The install / update review surface.
    InstallReview,
    /// The help center.
    HelpCenter,
    /// The support export.
    SupportExport,
}

impl M5MarketplaceInstallSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MarketplaceCatalog,
        Self::ExtensionManager,
        Self::RegistryAdmin,
        Self::InstallReview,
        Self::HelpCenter,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarketplaceCatalog => "marketplace_catalog",
            Self::ExtensionManager => "extension_manager",
            Self::RegistryAdmin => "registry_admin",
            Self::InstallReview => "install_review",
            Self::HelpCenter => "help_center",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's source class,
/// compatibility, permission, budget, or rollback truth never silently narrows or widens between
/// deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceInstallDeploymentLine {
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

impl M5MarketplaceInstallDeploymentLine {
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
pub enum M5MarketplaceInstallConsumerSurface {
    /// The marketplace UI.
    MarketplaceUi,
    /// The extensions UI.
    ExtensionsUi,
    /// The registry UI.
    RegistryUi,
    /// The install-review UI.
    InstallReviewUi,
    /// The settings UI.
    SettingsUi,
    /// The help UI.
    HelpUi,
    /// The AI context surface.
    AiContextUi,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5MarketplaceInstallConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::MarketplaceUi,
        Self::ExtensionsUi,
        Self::RegistryUi,
        Self::InstallReviewUi,
        Self::SettingsUi,
        Self::HelpUi,
        Self::AiContextUi,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarketplaceUi => "marketplace_ui",
            Self::ExtensionsUi => "extensions_ui",
            Self::RegistryUi => "registry_ui",
            Self::InstallReviewUi => "install_review_ui",
            Self::SettingsUi => "settings_ui",
            Self::HelpUi => "help_ui",
            Self::AiContextUi => "ai_context_ui",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no marketplace or install truth
/// is hover-only, pointer-only, menu-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceInstallAccessibilityRoute {
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
    /// Present in the support / export packet, never menu-only.
    SupportExportable,
}

impl M5MarketplaceInstallAccessibilityRoute {
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

/// Reason a marketplace-install component has degraded below its qualified state. Required on every
/// row so a stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceInstallDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The compatibility signal is unavailable.
    CompatibilitySignalUnavailable,
    /// The permission delta could not be verified.
    PermissionDeltaUnverified,
    /// The activation-budget signal is unavailable.
    ActivationBudgetSignalUnavailable,
    /// Publisher continuity could not be verified.
    PublisherContinuityUnverified,
    /// The registry source could not be resolved.
    RegistrySourceUnresolved,
}

impl M5MarketplaceInstallDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::CompatibilitySignalUnavailable,
        Self::PermissionDeltaUnverified,
        Self::ActivationBudgetSignalUnavailable,
        Self::PublisherContinuityUnverified,
        Self::RegistrySourceUnresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::CompatibilitySignalUnavailable => "compatibility_signal_unavailable",
            Self::PermissionDeltaUnverified => "permission_delta_unverified",
            Self::ActivationBudgetSignalUnavailable => "activation_budget_signal_unavailable",
            Self::PublisherContinuityUnverified => "publisher_continuity_unverified",
            Self::RegistrySourceUnresolved => "registry_source_unresolved",
        }
    }
}

/// Mandatory label a claimed marketplace-install component must be able to show. The first three
/// are hard requirements on every component; the remaining three close the acceptance-criteria
/// ambiguity about compatibility / host, permission / budget, and publisher / source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceInstallRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state / disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The compatibility range and host / runtime model behind the component.
    CompatibilityAndHost,
    /// The permission posture and activation budget behind the component.
    PermissionAndBudget,
    /// The publisher continuity and registry source class behind the component.
    PublisherAndSourceClass,
}

impl M5MarketplaceInstallRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::CompatibilityAndHost,
        Self::PermissionAndBudget,
        Self::PublisherAndSourceClass,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::CompatibilityAndHost => "compatibility_and_host",
            Self::PermissionAndBudget => "permission_and_budget",
            Self::PublisherAndSourceClass => "publisher_and_source_class",
        }
    }
}

/// Qualification class for an M5 marketplace-install component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceInstallQualificationClass {
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

impl M5MarketplaceInstallQualificationClass {
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

/// Downgrade trigger that narrows a marketplace-install component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceInstallDowngradeTrigger {
    /// A component left its compatibility range unstated.
    CompatibilityRangeUnstated,
    /// A component left its host / runtime model unstated.
    HostModelUnstated,
    /// A component hid permission widening.
    PermissionWideningHidden,
    /// A component hid transitive permission widening.
    TransitivePermissionHidden,
    /// A component hid activation cost.
    ActivationCostHidden,
    /// A component hid a publisher transfer.
    PublisherTransferHidden,
    /// A component collapsed the registry source class across public / mirrored / enterprise.
    RegistrySourceClassCollapsed,
    /// A component left its disable scope unstated.
    DisableScopeUnstated,
    /// A component hid a rollback incompatibility.
    RollbackIncompatibilityHidden,
    /// A component hid quarantine history.
    QuarantineHistoryHidden,
    /// Generic chrome wording concealed marketplace or install truth.
    GenericChromeWordingUsed,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5MarketplaceInstallDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::CompatibilityRangeUnstated,
        Self::HostModelUnstated,
        Self::PermissionWideningHidden,
        Self::TransitivePermissionHidden,
        Self::ActivationCostHidden,
        Self::PublisherTransferHidden,
        Self::RegistrySourceClassCollapsed,
        Self::DisableScopeUnstated,
        Self::RollbackIncompatibilityHidden,
        Self::QuarantineHistoryHidden,
        Self::GenericChromeWordingUsed,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompatibilityRangeUnstated => "compatibility_range_unstated",
            Self::HostModelUnstated => "host_model_unstated",
            Self::PermissionWideningHidden => "permission_widening_hidden",
            Self::TransitivePermissionHidden => "transitive_permission_hidden",
            Self::ActivationCostHidden => "activation_cost_hidden",
            Self::PublisherTransferHidden => "publisher_transfer_hidden",
            Self::RegistrySourceClassCollapsed => "registry_source_class_collapsed",
            Self::DisableScopeUnstated => "disable_scope_unstated",
            Self::RollbackIncompatibilityHidden => "rollback_incompatibility_hidden",
            Self::QuarantineHistoryHidden => "quarantine_history_hidden",
            Self::GenericChromeWordingUsed => "generic_chrome_wording_used",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed marketplace-install component family bound to the surface-
/// specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceInstallComponentRow {
    /// Governed component family.
    pub component_family: M5MarketplaceInstallComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5MarketplaceInstallQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this component.
    pub surface_families: Vec<M5MarketplaceInstallSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5MarketplaceInstallDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5MarketplaceInstallRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5MarketplaceInstallRequiredLabel>,
    /// Marketplace / install dispositions this component can carry (the frozen AC vocabulary;
    /// required on every component).
    pub dispositions: Vec<M5MarketplaceInstallDisposition>,
    /// Registry source classes this component names (source-bearing families only).
    pub registry_source_classes: Vec<M5RegistrySourceClass>,
    /// Compatibility states this component names (compatibility-bearing families only).
    pub compatibility_states: Vec<M5CompatibilityState>,
    /// Host / runtime models this component names (host-bearing families only).
    pub host_runtime_models: Vec<M5HostRuntimeModel>,
    /// Permission postures this component names (permission-bearing families only).
    pub permission_postures: Vec<M5PermissionPostureState>,
    /// Activation-budget bands this component names (budget-bearing families only).
    pub activation_budget_bands: Vec<M5ActivationBudgetBandState>,
    /// Publisher-continuity states this component names (publisher-bearing families only).
    pub publisher_continuity_states: Vec<M5PublisherContinuityState>,
    /// Disable-scope classes this component names (disable-bearing families only).
    pub disable_scope_classes: Vec<M5DisableScopeClass>,
    /// Rollback-compatibility states this component names (rollback-bearing families only).
    pub rollback_compatibility_states: Vec<M5RollbackCompatibilityState>,
    /// Quarantine states this component names (quarantine-bearing families only).
    pub quarantine_states: Vec<M5QuarantineState>,
    /// Degraded reasons this component can name (required on every component).
    pub degraded_reasons: Vec<M5MarketplaceInstallDegradedReason>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5MarketplaceInstallAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5MarketplaceInstallConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5MarketplaceInstallDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component (must include its own canonical component
    /// schema so downstream rows have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never hides permission widening or activation cost behind
    /// compact chrome. MUST be `false`.
    pub hides_permission_widening_or_activation_cost: bool,
    /// Hard invariant: this component never hides a publisher transfer, disable scope, or rollback
    /// incompatibility. MUST be `false`.
    pub hides_publisher_transfer_disable_scope_or_rollback_incompatibility: bool,
    /// Hard invariant: this component never collapses the registry source class across public /
    /// mirrored / enterprise. MUST be `false`.
    pub collapses_registry_source_class_across_public_mirrored_enterprise: bool,
    /// Hard invariant: this component never presents an incompatible or over-budget artifact as
    /// ready to install. MUST be `false`.
    pub presents_incompatible_or_over_budget_as_ready: bool,
}

impl M5MarketplaceInstallComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5MarketplaceInstallRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5MarketplaceInstallRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_permission_widening_or_activation_cost
            && !self.hides_publisher_transfer_disable_scope_or_rollback_incompatibility
            && !self.collapses_registry_source_class_across_public_mirrored_enterprise
            && !self.presents_incompatible_or_over_budget_as_ready
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceInstallVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Marketplace / install-disposition tokens.
    pub dispositions: Vec<String>,
    /// Registry source-class tokens.
    pub registry_source_classes: Vec<String>,
    /// Compatibility-state tokens.
    pub compatibility_states: Vec<String>,
    /// Host / runtime-model tokens.
    pub host_runtime_models: Vec<String>,
    /// Permission-posture tokens.
    pub permission_postures: Vec<String>,
    /// Activation-budget-band tokens.
    pub activation_budget_bands: Vec<String>,
    /// Publisher-continuity tokens.
    pub publisher_continuity_states: Vec<String>,
    /// Disable-scope tokens.
    pub disable_scope_classes: Vec<String>,
    /// Rollback-compatibility tokens.
    pub rollback_compatibility_states: Vec<String>,
    /// Quarantine-state tokens.
    pub quarantine_states: Vec<String>,
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
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5MarketplaceInstallVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5MarketplaceInstallComponentFamily::ALL, |v| v.as_str()),
            dispositions: tokens(&M5MarketplaceInstallDisposition::ALL, |v| v.as_str()),
            registry_source_classes: tokens(&M5RegistrySourceClass::ALL, |v| v.as_str()),
            compatibility_states: tokens(&M5CompatibilityState::ALL, |v| v.as_str()),
            host_runtime_models: tokens(&M5HostRuntimeModel::ALL, |v| v.as_str()),
            permission_postures: tokens(&M5PermissionPostureState::ALL, |v| v.as_str()),
            activation_budget_bands: tokens(&M5ActivationBudgetBandState::ALL, |v| v.as_str()),
            publisher_continuity_states: tokens(&M5PublisherContinuityState::ALL, |v| v.as_str()),
            disable_scope_classes: tokens(&M5DisableScopeClass::ALL, |v| v.as_str()),
            rollback_compatibility_states: tokens(&M5RollbackCompatibilityState::ALL, |v| {
                v.as_str()
            }),
            quarantine_states: tokens(&M5QuarantineState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5MarketplaceInstallSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5MarketplaceInstallDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5MarketplaceInstallConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5MarketplaceInstallAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5MarketplaceInstallDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5MarketplaceInstallRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5MarketplaceInstallDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5MarketplaceInstallGovernanceReview {
    /// The marketplace result row shows source class and compatibility.
    pub marketplace_result_row_shows_source_class_and_compatibility: bool,
    /// The marketplace detail fact grid shows every marketplace fact together.
    pub marketplace_detail_fact_grid_shows_all_facts_together: bool,
    /// The compatibility-label strip shows the compatibility range and host / runtime model.
    pub compatibility_label_strip_shows_range_and_host_model: bool,
    /// The permission-manifest summary shows the posture and any transitive widening.
    pub permission_manifest_summary_shows_posture_and_transitive_widening: bool,
    /// The activation-budget band shows the budget band and throttle state.
    pub activation_budget_band_shows_band_and_throttle: bool,
    /// The install/update/disable/rollback review sheet shows disable scope and rollback.
    pub install_review_sheet_shows_disable_scope_and_rollback: bool,
    /// The publisher-continuity row shows transfer and deprecation.
    pub publisher_continuity_row_shows_transfer_and_deprecation: bool,
    /// The installed-state diagnostics card shows quarantine history and health.
    pub installed_state_diagnostics_card_shows_quarantine_and_health: bool,
    /// No component hides permission widening behind compact chrome.
    pub no_component_hides_permission_widening: bool,
    /// Activation cost is always explicit.
    pub activation_cost_always_explicit: bool,
    /// A publisher transfer is never hidden.
    pub publisher_transfer_never_hidden: bool,
    /// Registry source class is always explicit before mutation or handoff.
    pub registry_source_class_always_explicit: bool,
    /// A rollback incompatibility is never hidden.
    pub rollback_incompatibility_never_hidden: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel marketplace / install vocabulary.
    pub later_rows_cannot_invent_parallel_marketplace_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceInstallConsumerProjection {
    /// Marketplace surfaces consume the shared registry source-class vocabulary.
    pub marketplace_surfaces_consume_source_class_vocabulary: bool,
    /// The extension manager consumes the shared permission-posture vocabulary.
    pub extension_manager_consumes_permission_posture_vocabulary: bool,
    /// Install review consumes the shared disable-scope and rollback vocabulary.
    pub install_review_consumes_disable_scope_and_rollback_vocabulary: bool,
    /// The registry admin consumes the shared publisher-continuity vocabulary.
    pub registry_admin_consumes_publisher_continuity_vocabulary: bool,
    /// Help consumes the shared compatibility vocabulary.
    pub help_consumes_compatibility_vocabulary: bool,
    /// Support / export reads a single canonical marketplace / install source.
    pub support_export_reads_single_marketplace_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceInstallProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the marketplace-install component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceInstallReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting marketplace / install component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5MarketplaceInstallComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MarketplaceInstallComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5MarketplaceInstallComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MarketplaceInstallVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MarketplaceInstallGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MarketplaceInstallConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MarketplaceInstallProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MarketplaceInstallReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 marketplace-install component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MarketplaceInstallComponentMatrixPacket {
    /// Record kind; must equal [`M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5MarketplaceInstallComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MarketplaceInstallVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MarketplaceInstallGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MarketplaceInstallConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MarketplaceInstallProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MarketplaceInstallReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5MarketplaceInstallComponentMatrixPacket {
    /// Builds an M5 marketplace-install component matrix packet from stable-lane input.
    pub fn new(input: M5MarketplaceInstallComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_SCHEMA_VERSION,
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

    /// Validates the M5 marketplace-install component matrix invariants.
    pub fn validate(&self) -> Vec<M5MarketplaceInstallComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MARKETPLACE_INSTALL_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 marketplace-install component matrix serializes"),
        ) {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 marketplace-install component matrix packet serializes")
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
            "# M5 Marketplace-Result-Row, Marketplace-Detail-Fact-Grid, Compatibility-Label-Strip, Permission-Manifest-Summary, Activation-Budget-Band, Install/Update/Disable/Rollback Review-Sheet, Publisher-Continuity-Row, and Installed-State-Diagnostics-Card Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Marketplace / install dispositions: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Registry source classes: {}\n",
            self.vocabulary_set.registry_source_classes.join(", ")
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

/// Errors emitted when reading the checked-in M5 marketplace-install matrix export.
#[derive(Debug)]
pub enum M5MarketplaceInstallComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5MarketplaceInstallComponentMatrixViolation>),
}

impl fmt::Display for M5MarketplaceInstallComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 marketplace-install component matrix export parse failed: {error}"
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
                    "m5 marketplace-install component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5MarketplaceInstallComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5MarketplaceInstallComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5MarketplaceInstallComponentMatrixViolation {
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
    /// A component declares no marketplace / install dispositions.
    DispositionMissing,
    /// A source-bearing component declares no registry source classes.
    RegistrySourceMissing,
    /// A compatibility-bearing component declares no compatibility states.
    CompatibilityMissing,
    /// A host-bearing component declares no host / runtime models.
    HostModelMissing,
    /// A permission-bearing component declares no permission postures.
    PermissionPostureMissing,
    /// A budget-bearing component declares no activation-budget bands.
    ActivationBudgetMissing,
    /// A publisher-bearing component declares no publisher-continuity states.
    PublisherContinuityMissing,
    /// A disable-bearing component declares no disable-scope classes.
    DisableScopeMissing,
    /// A rollback-bearing component declares no rollback-compatibility states.
    RollbackCompatibilityMissing,
    /// A quarantine-bearing component declares no quarantine states.
    QuarantineStateMissing,
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
    /// A component violates a hard invariant (hides permission widening or activation cost, hides a
    /// publisher transfer / disable scope / rollback incompatibility, collapses the registry source
    /// class, or presents an incompatible or over-budget artifact as ready).
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

impl M5MarketplaceInstallComponentMatrixViolation {
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
            Self::DispositionMissing => "disposition_missing",
            Self::RegistrySourceMissing => "registry_source_missing",
            Self::CompatibilityMissing => "compatibility_missing",
            Self::HostModelMissing => "host_model_missing",
            Self::PermissionPostureMissing => "permission_posture_missing",
            Self::ActivationBudgetMissing => "activation_budget_missing",
            Self::PublisherContinuityMissing => "publisher_continuity_missing",
            Self::DisableScopeMissing => "disable_scope_missing",
            Self::RollbackCompatibilityMissing => "rollback_compatibility_missing",
            Self::QuarantineStateMissing => "quarantine_state_missing",
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

/// Reads and validates the checked-in stable M5 marketplace-install matrix export.
pub fn current_stable_m5_marketplace_install_component_matrix_export() -> Result<
    M5MarketplaceInstallComponentMatrixPacket,
    M5MarketplaceInstallComponentMatrixArtifactError,
> {
    let packet: M5MarketplaceInstallComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-marketplace-install-proof/support_export.json"
        )))
        .map_err(M5MarketplaceInstallComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5MarketplaceInstallComponentMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5MarketplaceInstallComponentMatrixPacket,
    violations: &mut Vec<M5MarketplaceInstallComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
        M5_MARKETPLACE_RESULT_ROW_SCHEMA_REF,
        M5_MARKETPLACE_DETAIL_FACT_GRID_SCHEMA_REF,
        M5_COMPATIBILITY_LABEL_STRIP_SCHEMA_REF,
        M5_PERMISSION_MANIFEST_SUMMARY_SCHEMA_REF,
        M5_ACTIVATION_BUDGET_BAND_SCHEMA_REF,
        M5_INSTALL_UPDATE_DISABLE_ROLLBACK_REVIEW_SHEET_SCHEMA_REF,
        M5_PUBLISHER_CONTINUITY_ROW_SCHEMA_REF,
        M5_INSTALLED_STATE_DIAGNOSTICS_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5MarketplaceInstallComponentMatrixPacket,
    violations: &mut Vec<M5MarketplaceInstallComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5MarketplaceInstallComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5MarketplaceInstallComponentMatrixPacket,
    violations: &mut Vec<M5MarketplaceInstallComponentMatrixViolation>,
) {
    let present: BTreeSet<M5MarketplaceInstallComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5MarketplaceInstallComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::RequiredComponentMissing);
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
            violations.push(M5MarketplaceInstallComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_component_schema_ref())
        {
            violations
                .push(M5MarketplaceInstallComponentMatrixViolation::ComponentSchemaRefMissing);
        }
        if row.dispositions.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::DispositionMissing);
        }
        if family.declares_registry_source() && row.registry_source_classes.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::RegistrySourceMissing);
        }
        if family.declares_compatibility() && row.compatibility_states.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::CompatibilityMissing);
        }
        if family.declares_host_model() && row.host_runtime_models.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::HostModelMissing);
        }
        if family.declares_permission_posture() && row.permission_postures.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::PermissionPostureMissing);
        }
        if family.declares_activation_budget() && row.activation_budget_bands.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::ActivationBudgetMissing);
        }
        if family.declares_publisher_continuity() && row.publisher_continuity_states.is_empty() {
            violations
                .push(M5MarketplaceInstallComponentMatrixViolation::PublisherContinuityMissing);
        }
        if family.declares_disable_scope() && row.disable_scope_classes.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::DisableScopeMissing);
        }
        if family.declares_rollback_compat() && row.rollback_compatibility_states.is_empty() {
            violations
                .push(M5MarketplaceInstallComponentMatrixViolation::RollbackCompatibilityMissing);
        }
        if family.declares_quarantine() && row.quarantine_states.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::QuarantineStateMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations
                .push(M5MarketplaceInstallComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5MarketplaceInstallComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations
                .push(M5MarketplaceInstallComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations
                .push(M5MarketplaceInstallComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5MarketplaceInstallComponentMatrixPacket,
    violations: &mut Vec<M5MarketplaceInstallComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.marketplace_result_row_shows_source_class_and_compatibility,
        review.marketplace_detail_fact_grid_shows_all_facts_together,
        review.compatibility_label_strip_shows_range_and_host_model,
        review.permission_manifest_summary_shows_posture_and_transitive_widening,
        review.activation_budget_band_shows_band_and_throttle,
        review.install_review_sheet_shows_disable_scope_and_rollback,
        review.publisher_continuity_row_shows_transfer_and_deprecation,
        review.installed_state_diagnostics_card_shows_quarantine_and_health,
        review.no_component_hides_permission_widening,
        review.activation_cost_always_explicit,
        review.publisher_transfer_never_hidden,
        review.registry_source_class_always_explicit,
        review.rollback_incompatibility_never_hidden,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_marketplace_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5MarketplaceInstallComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5MarketplaceInstallComponentMatrixPacket,
    violations: &mut Vec<M5MarketplaceInstallComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.marketplace_surfaces_consume_source_class_vocabulary,
        projection.extension_manager_consumes_permission_posture_vocabulary,
        projection.install_review_consumes_disable_scope_and_rollback_vocabulary,
        projection.registry_admin_consumes_publisher_continuity_vocabulary,
        projection.help_consumes_compatibility_vocabulary,
        projection.support_export_reads_single_marketplace_source,
    ] {
        if !ok {
            violations
                .push(M5MarketplaceInstallComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5MarketplaceInstallComponentMatrixPacket,
    violations: &mut Vec<M5MarketplaceInstallComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5MarketplaceInstallComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5MarketplaceInstallComponentMatrixPacket,
    violations: &mut Vec<M5MarketplaceInstallComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5MarketplaceInstallComponentMatrixViolation::ReleasePostureIncomplete);
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
/// vocabulary deliberately uses marketplace / install words; what is rejected is a raw secret
/// *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
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
