//! Frozen M5 retired-state, end-of-support closure, successor-routing, and tombstone/archive matrix.
//!
//! This module locks Aureline's terminal-lifecycle object model — the supported lines, stable-facing
//! capabilities, bundles, commands / deep links, schema-bearing surfaces, registry-visible packages, and
//! managed / new-tenant-gated features that must move from `Deprecated` to `Retired` cleanly — into one
//! export-safe packet. Every covered object class is named once here and constrained by the same shared
//! retirement-role taxonomy (last_supported_pin, successor_routing, disable_path, export_rollback_route,
//! archival_note, migration_outcome, support_note_closure), the same required transition metadata
//! (last-supported version or channel, cutoff date, successor path, disable path, export / rollback route,
//! archival note, migration outcome, and support-note closure state), the same
//! no-retired-surface-disappears-without-a-tombstone-archival-route-or-successor-pointer rule, the same
//! no-retired-class-stays-selectable-in-new-install-new-tenant-marketplace-or-upgrade-flow rule, the same
//! no-last-supported-docs-schemas-or-evidence-destroyed-before-support-note-closure rule, the same
//! retirement-state-stays-joined-to-build-line-identity-deployment-profile-and-migration-outcome rule, and the
//! same no-silent-disappearance-stale-selection-ui-or-orphaned-support-truth rule regardless of the surface
//! that renders it.
//!
//! The matrix makes `Retired` mechanically distinct from `Deprecated`, `DisabledByPolicy`, and ordinary
//! stable-line narrowing (see [`M5RetiredStateLifecycleState`]) so downstream automation can key off the
//! terminal state rather than guessing from a disappearance. It does not retire any surface — later rows
//! execute retirements — it is the shared reusable retirement-closure engine contract those rows consume,
//! and it binds back to the already-landed stable-proof-index and migration-task-row packets so terminal
//! lifecycle truth is not split across scattered internal notes. The controlled vocabularies are frozen in
//! one self-describing [`M5RetiredStateVocabularySet`] rather than minted per surface. Raw secret values and
//! private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_retired_state_matrix,
    seeded_m5_retired_state_matrix_managed_tenant_feature_preview_narrowed,
    seeded_m5_retired_state_matrix_registry_visible_package_beta_narrowed,
    M5_RETIRED_STATE_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5RetiredStateMatrixPacket`].
pub const M5_RETIRED_STATE_MATRIX_RECORD_KIND: &str =
    "freeze_m5_retired_state_end_of_support_closure_successor_routing_and_tombstone_archive_matrix";

/// Schema version for M5 retired-state matrix records.
pub const M5_RETIRED_STATE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined retired-state matrix schema.
pub const M5_RETIRED_STATE_MATRIX_SCHEMA_REF: &str =
    "schemas/program/m5-retired-state-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_RETIRED_STATE_MATRIX_DOC_REF: &str = "docs/release/m5-retired-state-ops.md";

/// Repo-relative path of the canonical retirement-manifest domain schema (supported line and stable
/// capability: last-supported version / channel, cutoff date, successor path, disable path, and support-note
/// closure state of a retiring class).
pub const M5_RETIREMENT_MANIFEST_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-retirement-manifest.schema.json";

/// Repo-relative path of the canonical retirement-impact-report domain schema (command / deep link and
/// registry-visible package: no-new-install / no-new-tenant gating, removal from selection surfaces, and
/// successor routing of a retiring class).
pub const M5_RETIREMENT_IMPACT_REPORT_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-retirement-impact-report.schema.json";

/// Repo-relative path of the canonical last-supported-snapshot domain schema (bundle and schema-bearing
/// surface: the exact-build last-supported snapshot, archival note, and export / rollback route of a retiring
/// class).
pub const M5_LAST_SUPPORTED_SNAPSHOT_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-last-supported-snapshot.schema.json";

/// Repo-relative path of the canonical retirement-closure-ledger domain schema (managed / new-tenant feature:
/// support-note closure, archival route, tombstone, and migration outcome of a retiring class).
pub const M5_RETIREMENT_CLOSURE_LEDGER_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-retirement-closure-ledger.schema.json";

/// Repo-relative path of the already-landed stable-proof-index schema the matrix binds back to.
pub const M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF: &str =
    "schemas/release/stable_proof_index.schema.json";

/// Repo-relative path of the already-landed migration-task-row schema the matrix binds back to.
pub const M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF: &str =
    "schemas/release/m5-migration-task-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_RETIRED_STATE_FIXTURE_DIR: &str = "fixtures/release/m5-retired-state";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RETIRED_STATE_ARTIFACT_REF: &str =
    "artifacts/release/m5-retirements/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_RETIRED_STATE_CSV_REF: &str = "artifacts/release/m5-retirements/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_RETIRED_STATE_REPORT_REF: &str = "artifacts/program/m5-retired-state-matrix.md";

/// Repo-relative path of the checked retired-surface-health dashboard.
pub const M5_RETIRED_STATE_DASHBOARD_REF: &str = "dashboards/m5-retired-surface-health.json";

/// One of the seven governed retirement object classes this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateObject {
    /// A claimed M5 supported line reaching terminal Retired state.
    SupportedLine,
    /// A stable-facing capability reaching terminal Retired state.
    StableCapability,
    /// A shipped bundle reaching terminal Retired state.
    Bundle,
    /// A command or deep link reaching terminal Retired state.
    CommandDeepLink,
    /// A schema-bearing surface reaching terminal Retired state.
    SchemaBearingSurface,
    /// A registry-/marketplace-visible package reaching terminal Retired state.
    RegistryVisiblePackage,
    /// A managed / new-tenant-gated feature reaching terminal Retired state.
    ManagedTenantFeature,
}

impl M5RetiredStateObject {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SupportedLine,
        Self::StableCapability,
        Self::Bundle,
        Self::CommandDeepLink,
        Self::SchemaBearingSurface,
        Self::RegistryVisiblePackage,
        Self::ManagedTenantFeature,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportedLine => "supported_line",
            Self::StableCapability => "stable_capability",
            Self::Bundle => "bundle",
            Self::CommandDeepLink => "command_deep_link",
            Self::SchemaBearingSurface => "schema_bearing_surface",
            Self::RegistryVisiblePackage => "registry_visible_package",
            Self::ManagedTenantFeature => "managed_tenant_feature",
        }
    }
    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// class's retirement-manifest, impact-report, last-supported-snapshot, or closure-ledger meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::SupportedLine | Self::StableCapability => {
                M5_RETIREMENT_MANIFEST_DOMAIN_SCHEMA_REF
            }
            Self::Bundle | Self::SchemaBearingSurface => {
                M5_LAST_SUPPORTED_SNAPSHOT_DOMAIN_SCHEMA_REF
            }
            Self::CommandDeepLink | Self::RegistryVisiblePackage => {
                M5_RETIREMENT_IMPACT_REPORT_DOMAIN_SCHEMA_REF
            }
            Self::ManagedTenantFeature => M5_RETIREMENT_CLOSURE_LEDGER_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this class must name a controlled supported line role.
    pub const fn declares_supported_line_roles(self) -> bool {
        matches!(self, Self::SupportedLine)
    }

    /// `true` when this class must name a controlled stable capability role.
    pub const fn declares_stable_capability_roles(self) -> bool {
        matches!(self, Self::StableCapability)
    }

    /// `true` when this class must name a controlled bundle role.
    pub const fn declares_bundle_roles(self) -> bool {
        matches!(self, Self::Bundle)
    }

    /// `true` when this class must name a controlled command deep link role.
    pub const fn declares_command_deep_link_roles(self) -> bool {
        matches!(self, Self::CommandDeepLink)
    }

    /// `true` when this class must name a controlled schema bearing surface role.
    pub const fn declares_schema_bearing_surface_roles(self) -> bool {
        matches!(self, Self::SchemaBearingSurface)
    }

    /// `true` when this class must name a controlled registry visible package role.
    pub const fn declares_registry_visible_package_roles(self) -> bool {
        matches!(self, Self::RegistryVisiblePackage)
    }

    /// `true` when this class must name a controlled managed tenant feature role.
    pub const fn declares_managed_tenant_feature_roles(self) -> bool {
        matches!(self, Self::ManagedTenantFeature)
    }
}

/// The single controlled retirement-role vocabulary every release, help, docs, support, marketplace, install, or partner/procurement consumer binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateRole {
    /// The last-supported version / channel pinned before retirement.
    LastSupportedPin,
    /// The successor path a retiring class routes forward to.
    SuccessorRouting,
    /// The disable path a retiring class exposes.
    DisablePath,
    /// The export / rollback route preserved through retirement.
    ExportRollbackRoute,
    /// The archival / tombstone note preserved for the retired class.
    ArchivalNote,
    /// The recorded migration outcome for the retired class.
    MigrationOutcome,
    /// The support-note closure state for the retired class.
    SupportNoteClosure,
}

impl M5RetiredStateRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LastSupportedPin,
        Self::SuccessorRouting,
        Self::DisablePath,
        Self::ExportRollbackRoute,
        Self::ArchivalNote,
        Self::MigrationOutcome,
        Self::SupportNoteClosure,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LastSupportedPin => "last_supported_pin",
            Self::SuccessorRouting => "successor_routing",
            Self::DisablePath => "disable_path",
            Self::ExportRollbackRoute => "export_rollback_route",
            Self::ArchivalNote => "archival_note",
            Self::MigrationOutcome => "migration_outcome",
            Self::SupportNoteClosure => "support_note_closure",
        }
    }
    /// Whether this role is a hard closure gate that must be complete before a class may flip to
    /// `Retired` (`last_supported_pin`, `successor_routing`, `disable_path`, `support_note_closure`). The
    /// descriptor roles (`export_rollback_route`, `archival_note`, `migration_outcome`) are inspectable
    /// descriptors preserved through retirement rather than pre-flip gates.
    pub const fn must_be_closed_before_flipping_to_retired(self) -> bool {
        matches!(
            self,
            Self::LastSupportedPin
                | Self::SuccessorRouting
                | Self::DisablePath
                | Self::SupportNoteClosure
        )
    }
}

/// Lifecycle state that makes `Retired` mechanically distinct from `Deprecated`, `DisabledByPolicy`, and ordinary stable-line narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateLifecycleState {
    /// Deprecation is active; removal horizon announced but not reached.
    Deprecated,
    /// Disabled by policy but not yet terminally retired.
    DisabledByPolicy,
    /// Ordinary stable-line narrowing, not retirement.
    StableLineNarrowed,
    /// Terminal Retired state: last-supported pinned, successor routed, closure archived.
    Retired,
}

impl M5RetiredStateLifecycleState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Deprecated,
        Self::DisabledByPolicy,
        Self::StableLineNarrowed,
        Self::Retired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deprecated => "deprecated",
            Self::DisabledByPolicy => "disabled_by_policy",
            Self::StableLineNarrowed => "stable_line_narrowed",
            Self::Retired => "retired",
        }
    }
    /// `true` only for the terminal `Retired` state, so downstream automation can key off retirement
    /// rather than confusing it with deprecation, policy disablement, or ordinary stable-line narrowing.
    pub const fn is_retired(self) -> bool {
        matches!(self, Self::Retired)
    }
}

/// Controlled retirement-path role for a supported line reaching Retired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateSupportedLineRole {
    /// Last-supported version pinned to an exact build.
    LastSupportedVersionPinned,
    /// Successor line named for routing forward.
    SuccessorLineNamed,
    /// Disable path published for the retiring line.
    DisablePathPublished,
    /// No-new-install gating enforced for new installs.
    NoNewInstallEnforced,
    /// A role bound to the single retirement registry.
    BoundToRetirementRegistry,
    /// Silent disappearance without a tombstone, which is disallowed.
    SilentDisappearanceDisallowed,
}

impl M5RetiredStateSupportedLineRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LastSupportedVersionPinned,
        Self::SuccessorLineNamed,
        Self::DisablePathPublished,
        Self::NoNewInstallEnforced,
        Self::BoundToRetirementRegistry,
        Self::SilentDisappearanceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LastSupportedVersionPinned => "last_supported_version_pinned",
            Self::SuccessorLineNamed => "successor_line_named",
            Self::DisablePathPublished => "disable_path_published",
            Self::NoNewInstallEnforced => "no_new_install_enforced",
            Self::BoundToRetirementRegistry => "bound_to_retirement_registry",
            Self::SilentDisappearanceDisallowed => "silent_disappearance_disallowed",
        }
    }
}

/// Controlled retirement-path role for a stable-facing capability reaching Retired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateStableCapabilityRole {
    /// Last-supported channel pinned for the capability.
    LastSupportedChannelPinned,
    /// Successor capability named for routing forward.
    CapabilitySuccessorNamed,
    /// Export / rollback route ready for the capability.
    ExportRollbackRouteReady,
    /// No-new-tenant gating enforced for the capability.
    NoNewTenantEnforced,
    /// A role bound to the single retirement registry.
    BoundToRetirementRegistry,
    /// Orphaned support / docs truth for the capability, which is disallowed.
    OrphanedCapabilityTruthDisallowed,
}

impl M5RetiredStateStableCapabilityRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LastSupportedChannelPinned,
        Self::CapabilitySuccessorNamed,
        Self::ExportRollbackRouteReady,
        Self::NoNewTenantEnforced,
        Self::BoundToRetirementRegistry,
        Self::OrphanedCapabilityTruthDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LastSupportedChannelPinned => "last_supported_channel_pinned",
            Self::CapabilitySuccessorNamed => "capability_successor_named",
            Self::ExportRollbackRouteReady => "export_rollback_route_ready",
            Self::NoNewTenantEnforced => "no_new_tenant_enforced",
            Self::BoundToRetirementRegistry => "bound_to_retirement_registry",
            Self::OrphanedCapabilityTruthDisallowed => "orphaned_capability_truth_disallowed",
        }
    }
}

/// Controlled retirement-path role for a bundle reaching Retired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateBundleRole {
    /// Last-supported bundle snapshotted to exact build identity.
    LastSupportedBundleSnapshotted,
    /// Archival note written for the bundle.
    BundleArchivalNoteWritten,
    /// Export route ready for the retiring bundle.
    BundleExportRouteReady,
    /// Bundle removed from the upgrade flow.
    BundleRemovedFromUpgradeFlow,
    /// A role bound to the single retirement registry.
    BoundToRetirementRegistry,
    /// A stale bundle left selectable, which is disallowed.
    StaleBundleSelectableDisallowed,
}

impl M5RetiredStateBundleRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LastSupportedBundleSnapshotted,
        Self::BundleArchivalNoteWritten,
        Self::BundleExportRouteReady,
        Self::BundleRemovedFromUpgradeFlow,
        Self::BoundToRetirementRegistry,
        Self::StaleBundleSelectableDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LastSupportedBundleSnapshotted => "last_supported_bundle_snapshotted",
            Self::BundleArchivalNoteWritten => "bundle_archival_note_written",
            Self::BundleExportRouteReady => "bundle_export_route_ready",
            Self::BundleRemovedFromUpgradeFlow => "bundle_removed_from_upgrade_flow",
            Self::BoundToRetirementRegistry => "bound_to_retirement_registry",
            Self::StaleBundleSelectableDisallowed => "stale_bundle_selectable_disallowed",
        }
    }
}

/// Controlled retirement-path role for a command / deep link reaching Retired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateCommandDeepLinkRole {
    /// Tombstone registered for the retiring command / deep link.
    CommandTombstoneRegistered,
    /// Successor redirect named for the deep link.
    DeepLinkSuccessorRedirectNamed,
    /// Disable path ready for the command.
    CommandDisablePathReady,
    /// Command removed from the palette and selection surfaces.
    RemovedFromCommandPalette,
    /// A role bound to the single retirement registry.
    BoundToRetirementRegistry,
    /// A dangling deep link without a tombstone, which is disallowed.
    DanglingDeepLinkDisallowed,
}

impl M5RetiredStateCommandDeepLinkRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CommandTombstoneRegistered,
        Self::DeepLinkSuccessorRedirectNamed,
        Self::CommandDisablePathReady,
        Self::RemovedFromCommandPalette,
        Self::BoundToRetirementRegistry,
        Self::DanglingDeepLinkDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandTombstoneRegistered => "command_tombstone_registered",
            Self::DeepLinkSuccessorRedirectNamed => "deep_link_successor_redirect_named",
            Self::CommandDisablePathReady => "command_disable_path_ready",
            Self::RemovedFromCommandPalette => "removed_from_command_palette",
            Self::BoundToRetirementRegistry => "bound_to_retirement_registry",
            Self::DanglingDeepLinkDisallowed => "dangling_deep_link_disallowed",
        }
    }
}

/// Controlled retirement-path role for a schema-bearing surface reaching Retired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateSchemaBearingSurfaceRole {
    /// Last-supported schema snapshotted before closure.
    LastSupportedSchemaSnapshotted,
    /// Migration outcome recorded for the schema.
    SchemaMigrationOutcomeRecorded,
    /// Export route ready for the schema-bearing surface.
    SchemaExportRouteReady,
    /// Archival note written for the schema-bearing surface.
    SchemaArchivalNoteWritten,
    /// A role bound to the single retirement registry.
    BoundToRetirementRegistry,
    /// Destroying a last-supported schema before closure, which is disallowed.
    DestroyedSchemaBeforeClosureDisallowed,
}

impl M5RetiredStateSchemaBearingSurfaceRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LastSupportedSchemaSnapshotted,
        Self::SchemaMigrationOutcomeRecorded,
        Self::SchemaExportRouteReady,
        Self::SchemaArchivalNoteWritten,
        Self::BoundToRetirementRegistry,
        Self::DestroyedSchemaBeforeClosureDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LastSupportedSchemaSnapshotted => "last_supported_schema_snapshotted",
            Self::SchemaMigrationOutcomeRecorded => "schema_migration_outcome_recorded",
            Self::SchemaExportRouteReady => "schema_export_route_ready",
            Self::SchemaArchivalNoteWritten => "schema_archival_note_written",
            Self::BoundToRetirementRegistry => "bound_to_retirement_registry",
            Self::DestroyedSchemaBeforeClosureDisallowed => {
                "destroyed_schema_before_closure_disallowed"
            }
        }
    }
}

/// Controlled retirement-path role for a registry-visible package reaching Retired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateRegistryVisiblePackageRole {
    /// Package marked Retired in the registry.
    PackageMarkedRetiredInRegistry,
    /// Successor package named for routing forward.
    PackageSuccessorNamed,
    /// Package removed from the marketplace listing.
    RemovedFromMarketplaceListing,
    /// No-new-install gating enforced from the registry.
    NoNewInstallFromRegistry,
    /// A role bound to the single retirement registry.
    BoundToRetirementRegistry,
    /// A stale marketplace listing left selectable, which is disallowed.
    StaleMarketplaceListingDisallowed,
}

impl M5RetiredStateRegistryVisiblePackageRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PackageMarkedRetiredInRegistry,
        Self::PackageSuccessorNamed,
        Self::RemovedFromMarketplaceListing,
        Self::NoNewInstallFromRegistry,
        Self::BoundToRetirementRegistry,
        Self::StaleMarketplaceListingDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageMarkedRetiredInRegistry => "package_marked_retired_in_registry",
            Self::PackageSuccessorNamed => "package_successor_named",
            Self::RemovedFromMarketplaceListing => "removed_from_marketplace_listing",
            Self::NoNewInstallFromRegistry => "no_new_install_from_registry",
            Self::BoundToRetirementRegistry => "bound_to_retirement_registry",
            Self::StaleMarketplaceListingDisallowed => "stale_marketplace_listing_disallowed",
        }
    }
}

/// Controlled retirement-path role for a managed / new-tenant-gated feature reaching Retired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateManagedTenantFeatureRole {
    /// Feature disabled by policy for new tenants.
    FeatureDisabledByPolicyForNewTenants,
    /// Successor feature named for tenant routing.
    TenantSuccessorNamed,
    /// Export / rollback route ready for the feature.
    TenantExportRollbackRouteReady,
    /// Support note closed for the retiring feature.
    SupportNoteClosedForFeature,
    /// A role bound to the single retirement registry.
    BoundToRetirementRegistry,
    /// New-tenant gating bypass, which is disallowed.
    NewTenantGatingBypassDisallowed,
}

impl M5RetiredStateManagedTenantFeatureRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FeatureDisabledByPolicyForNewTenants,
        Self::TenantSuccessorNamed,
        Self::TenantExportRollbackRouteReady,
        Self::SupportNoteClosedForFeature,
        Self::BoundToRetirementRegistry,
        Self::NewTenantGatingBypassDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FeatureDisabledByPolicyForNewTenants => {
                "feature_disabled_by_policy_for_new_tenants"
            }
            Self::TenantSuccessorNamed => "tenant_successor_named",
            Self::TenantExportRollbackRouteReady => "tenant_export_rollback_route_ready",
            Self::SupportNoteClosedForFeature => "support_note_closed_for_feature",
            Self::BoundToRetirementRegistry => "bound_to_retirement_registry",
            Self::NewTenantGatingBypassDisallowed => "new_tenant_gating_bypass_disallowed",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a retirement object class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateSurfaceFamily {
    /// The release-center surface.
    ReleaseCenter,
    /// The help / docs surface.
    HelpDocs,
    /// The support surface.
    Support,
    /// The marketplace / registry surface.
    MarketplaceRegistry,
    /// The install / update surface.
    InstallUpdate,
    /// The partner / procurement truth feed.
    PartnerProcurement,
}

impl M5RetiredStateSurfaceFamily {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReleaseCenter,
        Self::HelpDocs,
        Self::Support,
        Self::MarketplaceRegistry,
        Self::InstallUpdate,
        Self::PartnerProcurement,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::HelpDocs => "help_docs",
            Self::Support => "support",
            Self::MarketplaceRegistry => "marketplace_registry",
            Self::InstallUpdate => "install_update",
            Self::PartnerProcurement => "partner_procurement",
        }
    }
}

/// Removal-horizon stage a class passes through before it reaches terminal `Retired`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateRemovalHorizonStage {
    /// The retirement-announced horizon stage.
    RetirementAnnounced,
    /// The deprecation-active horizon stage.
    DeprecationActive,
    /// The last-supported-pinned horizon stage.
    LastSupportedPinned,
    /// The disable-path-ready horizon stage.
    DisablePathReady,
    /// The retirement-executed horizon stage.
    RetirementExecuted,
}

impl M5RetiredStateRemovalHorizonStage {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RetirementAnnounced,
        Self::DeprecationActive,
        Self::LastSupportedPinned,
        Self::DisablePathReady,
        Self::RetirementExecuted,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetirementAnnounced => "retirement_announced",
            Self::DeprecationActive => "deprecation_active",
            Self::LastSupportedPinned => "last_supported_pinned",
            Self::DisablePathReady => "disable_path_ready",
            Self::RetirementExecuted => "retirement_executed",
        }
    }
}

/// Subsystem that consumes a class's retirement projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateConsumerSurface {
    /// The release center.
    ReleaseCenter,
    /// The help / docs surface.
    HelpDocs,
    /// The support export.
    Support,
    /// The marketplace / registry.
    MarketplaceRegistry,
    /// The install / update flow.
    InstallUpdate,
    /// The partner / procurement truth feed.
    PartnerProcurement,
    /// The program-governance review.
    ProgramGovernance,
    /// The diagnostics surface.
    Diagnostics,
    /// The CLI / export path.
    CliExport,
}

impl M5RetiredStateConsumerSurface {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ReleaseCenter,
        Self::HelpDocs,
        Self::Support,
        Self::MarketplaceRegistry,
        Self::InstallUpdate,
        Self::PartnerProcurement,
        Self::ProgramGovernance,
        Self::Diagnostics,
        Self::CliExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::HelpDocs => "help_docs",
            Self::Support => "support",
            Self::MarketplaceRegistry => "marketplace_registry",
            Self::InstallUpdate => "install_update",
            Self::PartnerProcurement => "partner_procurement",
            Self::ProgramGovernance => "program_governance",
            Self::Diagnostics => "diagnostics",
            Self::CliExport => "cli_export",
        }
    }
}

/// Non-visual / accessibility route every class must offer so no retirement meaning disappears under zoom, high contrast, keyboard-only use, or export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5RetiredStateAccessibilityRoute {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a class has degraded below its qualified retirement-handling state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateDegradedReason {
    /// The retirement manifest has gone stale.
    RetirementManifestStale,
    /// The successor path is unavailable.
    SuccessorPathUnavailable,
    /// The last-supported snapshot is missing.
    LastSupportedSnapshotMissing,
    /// The retirement closure ledger is unavailable.
    ClosureLedgerUnavailable,
    /// The retirement impact report is unverified.
    ImpactReportUnverified,
    /// The retirement owner is unknown.
    RetirementOwnerUnknown,
}

impl M5RetiredStateDegradedReason {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RetirementManifestStale,
        Self::SuccessorPathUnavailable,
        Self::LastSupportedSnapshotMissing,
        Self::ClosureLedgerUnavailable,
        Self::ImpactReportUnverified,
        Self::RetirementOwnerUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetirementManifestStale => "retirement_manifest_stale",
            Self::SuccessorPathUnavailable => "successor_path_unavailable",
            Self::LastSupportedSnapshotMissing => "last_supported_snapshot_missing",
            Self::ClosureLedgerUnavailable => "closure_ledger_unavailable",
            Self::ImpactReportUnverified => "impact_report_unverified",
            Self::RetirementOwnerUnknown => "retirement_owner_unknown",
        }
    }
}

/// Mandatory label a claimed retirement class must be able to show. The first three are hard requirements; the remaining three close the acceptance-criteria ambiguity about cutoff date, successor path, and last-supported version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateRequiredLabel {
    /// The class's stable identity.
    Identity,
    /// The class's retirement role.
    RetirementRole,
    /// The canonical registry reference the class points at.
    RegistryReference,
    /// The cutoff date the class must publish.
    CutoffDate,
    /// The successor path the class routes to.
    SuccessorPath,
    /// The last-supported version / channel the class pins.
    LastSupportedVersion,
}

impl M5RetiredStateRequiredLabel {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::RetirementRole,
        Self::RegistryReference,
        Self::CutoffDate,
        Self::SuccessorPath,
        Self::LastSupportedVersion,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::RetirementRole => "retirement_role",
            Self::RegistryReference => "registry_reference",
            Self::CutoffDate => "cutoff_date",
            Self::SuccessorPath => "successor_path",
            Self::LastSupportedVersion => "last_supported_version",
        }
    }
    /// The three labels every claimed class must be able to show.
    pub const MANDATORY: [Self; 3] = [
        Self::Identity,
        Self::RetirementRole,
        Self::RegistryReference,
    ];
}

/// Qualification class for an M5 retired-state row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateQualificationClass {
    /// Class retirement handling qualifies for the Stable claim.
    Stable,
    /// Class retirement handling is narrowed to Beta.
    Beta,
    /// Class retirement handling is narrowed to Preview.
    Preview,
    /// Class retirement handling is experimental and not claimed.
    Experimental,
    /// Class retirement handling is unavailable on this build.
    Unavailable,
    /// Class retirement handling is held pending review.
    Held,
}

impl M5RetiredStateQualificationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Experimental,
        Self::Unavailable,
        Self::Held,
    ];

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
    /// Whether the class may carry a public Stable retirement-handling claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a retirement object class below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetiredStateDowngradeTrigger {
    /// A retired surface disappeared without a tombstone or successor pointer.
    RetiredSurfaceDisappearedWithoutTombstone,
    /// A retired class stayed selectable in a new-install flow.
    RetiredClassStayedSelectableInNewInstall,
    /// A retired class stayed selectable for a new tenant.
    RetiredClassStayedSelectableForNewTenant,
    /// The last-supported snapshot was missing at retirement.
    LastSupportedSnapshotMissing,
    /// A retiring class left its successor path unnamed.
    SuccessorPathUnnamed,
    /// A retiring class left its disable path unnamed.
    DisablePathUnnamed,
    /// A retiring class left its cutoff date unstated.
    CutoffDateUnstated,
    /// A retiring class left its archival note missing.
    ArchivalNoteMissing,
    /// A retiring class left its support-note closure incomplete.
    SupportNoteClosureIncomplete,
    /// A retiring class left its canonical registry reference unstated.
    RegistryReferenceUnstated,
    /// Retirement state was left unjoined from exact build and line identity.
    RetirementUnjoinedFromBuildIdentity,
    /// The retirement manifest packet has gone stale.
    RetirementManifestStale,
}

impl M5RetiredStateDowngradeTrigger {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::RetiredSurfaceDisappearedWithoutTombstone,
        Self::RetiredClassStayedSelectableInNewInstall,
        Self::RetiredClassStayedSelectableForNewTenant,
        Self::LastSupportedSnapshotMissing,
        Self::SuccessorPathUnnamed,
        Self::DisablePathUnnamed,
        Self::CutoffDateUnstated,
        Self::ArchivalNoteMissing,
        Self::SupportNoteClosureIncomplete,
        Self::RegistryReferenceUnstated,
        Self::RetirementUnjoinedFromBuildIdentity,
        Self::RetirementManifestStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetiredSurfaceDisappearedWithoutTombstone => {
                "retired_surface_disappeared_without_tombstone"
            }
            Self::RetiredClassStayedSelectableInNewInstall => {
                "retired_class_stayed_selectable_in_new_install"
            }
            Self::RetiredClassStayedSelectableForNewTenant => {
                "retired_class_stayed_selectable_for_new_tenant"
            }
            Self::LastSupportedSnapshotMissing => "last_supported_snapshot_missing",
            Self::SuccessorPathUnnamed => "successor_path_unnamed",
            Self::DisablePathUnnamed => "disable_path_unnamed",
            Self::CutoffDateUnstated => "cutoff_date_unstated",
            Self::ArchivalNoteMissing => "archival_note_missing",
            Self::SupportNoteClosureIncomplete => "support_note_closure_incomplete",
            Self::RegistryReferenceUnstated => "registry_reference_unstated",
            Self::RetirementUnjoinedFromBuildIdentity => "retirement_unjoined_from_build_identity",
            Self::RetirementManifestStale => "retirement_manifest_stale",
        }
    }
}

/// Required transition metadata a class must carry to move from `Deprecated` to `Retired`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RetiredStateTransition {
    /// Last supported version or channel pinned to an exact build.
    pub last_supported_version_or_channel: String,
    /// Cutoff date after which the class is retired.
    pub cutoff_date: String,
    /// Successor path the class routes forward to.
    pub successor_path: String,
    /// Disable path the class exposes.
    pub disable_path: String,
    /// Export / rollback route preserved through retirement.
    pub export_rollback_route: String,
    /// Archival / tombstone note preserved for the retired class.
    pub archival_note: String,
    /// Recorded migration outcome for the retired class.
    pub migration_outcome: String,
    /// Support-note closure state for the retired class.
    pub support_note_closure_state: String,
}

impl M5RetiredStateTransition {
    /// `true` when every required transition-metadata field is present.
    fn is_complete(&self) -> bool {
        !self.last_supported_version_or_channel.trim().is_empty()
            && !self.cutoff_date.trim().is_empty()
            && !self.successor_path.trim().is_empty()
            && !self.disable_path.trim().is_empty()
            && !self.export_rollback_route.trim().is_empty()
            && !self.archival_note.trim().is_empty()
            && !self.migration_outcome.trim().is_empty()
            && !self.support_note_closure_state.trim().is_empty()
    }
}

/// One row in the matrix: one governed retirement object class bound to the surface-specific truth it
/// must project as it moves to terminal `Retired`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RetiredStateRow {
    /// Governed retirement object class.
    pub object_class: M5RetiredStateObject,
    /// Qualification class earned by this class's retirement handling.
    pub qualification: M5RetiredStateQualificationClass,
    /// Terminal lifecycle state this row governs (distinguishes Retired from Deprecated / DisabledByPolicy / narrowing).
    pub lifecycle_state: M5RetiredStateLifecycleState,
    /// Owner role accountable for keeping this class's retirement governed.
    pub owner_role: String,
    /// Backup owner role accountable when the primary owner is unavailable.
    pub backup_owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required transition metadata from Deprecated to Retired.
    pub retirement_transition: M5RetiredStateTransition,
    /// Claimed M5 surface families that render / consume this class.
    pub surface_families: Vec<M5RetiredStateSurfaceFamily>,
    /// Removal-horizon stages this class passes through before Retired.
    pub removal_horizon_stages: Vec<M5RetiredStateRemovalHorizonStage>,
    /// Mandatory labels this class must be able to show (must include the three
    /// [`M5RetiredStateRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5RetiredStateRequiredLabel>,
    /// Retirement roles this class can carry (the frozen AC vocabulary; required on every class).
    pub semantic_roles: Vec<M5RetiredStateRole>,
    /// SupportedLine retirement-path roles this class names (SupportedLine only).
    pub supported_line_roles: Vec<M5RetiredStateSupportedLineRole>,
    /// StableCapability retirement-path roles this class names (StableCapability only).
    pub stable_capability_roles: Vec<M5RetiredStateStableCapabilityRole>,
    /// Bundle retirement-path roles this class names (Bundle only).
    pub bundle_roles: Vec<M5RetiredStateBundleRole>,
    /// CommandDeepLink retirement-path roles this class names (CommandDeepLink only).
    pub command_deep_link_roles: Vec<M5RetiredStateCommandDeepLinkRole>,
    /// SchemaBearingSurface retirement-path roles this class names (SchemaBearingSurface only).
    pub schema_bearing_surface_roles: Vec<M5RetiredStateSchemaBearingSurfaceRole>,
    /// RegistryVisiblePackage retirement-path roles this class names (RegistryVisiblePackage only).
    pub registry_visible_package_roles: Vec<M5RetiredStateRegistryVisiblePackageRole>,
    /// ManagedTenantFeature retirement-path roles this class names (ManagedTenantFeature only).
    pub managed_tenant_feature_roles: Vec<M5RetiredStateManagedTenantFeatureRole>,
    /// Degraded reasons this class can name (required on every class).
    pub degraded_reasons: Vec<M5RetiredStateDegradedReason>,
    /// Non-visual accessibility routes this class offers.
    pub accessibility_routes: Vec<M5RetiredStateAccessibilityRoute>,
    /// First consumer surfaces that consume this class's retirement projection.
    pub consumer_surfaces: Vec<M5RetiredStateConsumerSurface>,
    /// Downgrade triggers that apply to this class.
    pub downgrade_triggers: Vec<M5RetiredStateDowngradeTrigger>,
    /// Required closure-artifact refs that keep this class's retirement provable.
    pub required_closure_artifact_refs: Vec<String>,
    /// Source contract refs consumed by this class (must include its own canonical domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this class never lets a retired surface disappear without a tombstone, archival route, or successor pointer. MUST be `false`.
    pub lets_a_retired_surface_disappear_without_tombstone_archival_route_or_successor_pointer:
        bool,
    /// Hard invariant: this class never keeps a retired class selectable in new-install, new-tenant, marketplace, or upgrade flows. MUST be `false`.
    pub keeps_a_retired_class_selectable_in_new_install_new_tenant_marketplace_or_upgrade_flow:
        bool,
    /// Hard invariant: this class never destroys last-supported docs, schemas, or evidence before support-note closure and export-safe archive handoff. MUST be `false`.
    pub destroys_last_supported_docs_schemas_or_evidence_before_support_note_closure: bool,
    /// Hard invariant: this class never leaves retirement state unjoined to exact build, line identity, deployment profile, and migration outcome. MUST be `false`.
    pub leaves_retirement_state_unjoined_to_build_line_identity_deployment_profile_and_migration_outcome:
        bool,
    /// Hard invariant: this class never retires a surface through silent disappearance, stale selection UI, or orphaned support / docs truth. MUST be `false`.
    pub retires_a_surface_through_silent_disappearance_stale_selection_ui_or_orphaned_support_truth:
        bool,
}

impl M5RetiredStateRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5RetiredStateRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5RetiredStateRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.lets_a_retired_surface_disappear_without_tombstone_archival_route_or_successor_pointer
            && !self.keeps_a_retired_class_selectable_in_new_install_new_tenant_marketplace_or_upgrade_flow
            && !self.destroys_last_supported_docs_schemas_or_evidence_before_support_note_closure
            && !self.leaves_retirement_state_unjoined_to_build_line_identity_deployment_profile_and_migration_outcome
            && !self.retires_a_surface_through_silent_disappearance_stale_selection_ui_or_orphaned_support_truth
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RetiredStateVocabularySet {
    /// Object classes tokens.
    pub object_classes: Vec<String>,
    /// Lifecycle states tokens.
    pub lifecycle_states: Vec<String>,
    /// Semantic roles tokens.
    pub semantic_roles: Vec<String>,
    /// Supported line roles tokens.
    pub supported_line_roles: Vec<String>,
    /// Stable capability roles tokens.
    pub stable_capability_roles: Vec<String>,
    /// Bundle roles tokens.
    pub bundle_roles: Vec<String>,
    /// Command deep link roles tokens.
    pub command_deep_link_roles: Vec<String>,
    /// Schema bearing surface roles tokens.
    pub schema_bearing_surface_roles: Vec<String>,
    /// Registry visible package roles tokens.
    pub registry_visible_package_roles: Vec<String>,
    /// Managed tenant feature roles tokens.
    pub managed_tenant_feature_roles: Vec<String>,
    /// Surface families tokens.
    pub surface_families: Vec<String>,
    /// Removal horizon stages tokens.
    pub removal_horizon_stages: Vec<String>,
    /// Consumer surfaces tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility routes tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded reasons tokens.
    pub degraded_reasons: Vec<String>,
    /// Required labels tokens.
    pub required_labels: Vec<String>,
    /// Downgrade triggers tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5RetiredStateVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            object_classes: tokens(&M5RetiredStateObject::ALL, |v| v.as_str()),
            lifecycle_states: tokens(&M5RetiredStateLifecycleState::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5RetiredStateRole::ALL, |v| v.as_str()),
            supported_line_roles: tokens(&M5RetiredStateSupportedLineRole::ALL, |v| v.as_str()),
            stable_capability_roles: tokens(&M5RetiredStateStableCapabilityRole::ALL, |v| {
                v.as_str()
            }),
            bundle_roles: tokens(&M5RetiredStateBundleRole::ALL, |v| v.as_str()),
            command_deep_link_roles: tokens(&M5RetiredStateCommandDeepLinkRole::ALL, |v| {
                v.as_str()
            }),
            schema_bearing_surface_roles: tokens(
                &M5RetiredStateSchemaBearingSurfaceRole::ALL,
                |v| v.as_str(),
            ),
            registry_visible_package_roles: tokens(
                &M5RetiredStateRegistryVisiblePackageRole::ALL,
                |v| v.as_str(),
            ),
            managed_tenant_feature_roles: tokens(
                &M5RetiredStateManagedTenantFeatureRole::ALL,
                |v| v.as_str(),
            ),
            surface_families: tokens(&M5RetiredStateSurfaceFamily::ALL, |v| v.as_str()),
            removal_horizon_stages: tokens(&M5RetiredStateRemovalHorizonStage::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5RetiredStateConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5RetiredStateAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5RetiredStateDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5RetiredStateRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5RetiredStateDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5RetiredStateGovernanceReview {
    /// No retired surface disappears without tombstone archival route or successor pointer.
    pub no_retired_surface_disappears_without_tombstone_archival_route_or_successor_pointer: bool,
    /// Every covered object class names owner closure artifacts and first consumer.
    pub every_covered_object_class_names_owner_closure_artifacts_and_first_consumer: bool,
    /// Retired is mechanically distinct from deprecated disabled and narrowed.
    pub retired_is_mechanically_distinct_from_deprecated_disabled_and_narrowed: bool,
    /// Last supported snapshots are captured before retirement.
    pub last_supported_snapshots_are_captured_before_retirement: bool,
    /// Successor routing and cutoff review precede every retirement.
    pub successor_routing_and_cutoff_review_precede_every_retirement: bool,
    /// Support and public proof surfaces close cleanly on retirement.
    pub support_and_public_proof_surfaces_close_cleanly_on_retirement: bool,
    /// Archival and tombstone truth is preserved after retirement.
    pub archival_and_tombstone_truth_is_preserved_after_retirement: bool,
    /// No new installs or new tenants can select a retired class.
    pub no_new_installs_or_new_tenants_can_select_a_retired_class: bool,
    /// Retirement state stays joined to exact build and line identity.
    pub retirement_state_stays_joined_to_exact_build_and_line_identity: bool,
    /// Every object declares removal horizon stages.
    pub every_object_declares_removal_horizon_stages: bool,
    /// Every object declares accessibility route.
    pub every_object_declares_accessibility_route: bool,
    /// Support export reads single retirement source.
    pub support_export_reads_single_retirement_source: bool,
    /// Release help and support bind to single retirement source.
    pub release_help_and_support_bind_to_single_retirement_source: bool,
    /// Later rows cannot invent parallel retirement vocabulary.
    pub later_rows_cannot_invent_parallel_retirement_vocabulary: bool,
    /// Retirement truth survives zoom and high contrast.
    pub retirement_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when matrix row missing or stale.
    pub claims_narrow_automatically_when_matrix_row_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RetiredStateConsumerProjection {
    /// Release and help consume shared retirement truth.
    pub release_and_help_consume_shared_retirement_truth: bool,
    /// Support and marketplace consume shared closure and snapshot truth.
    pub support_and_marketplace_consume_shared_closure_and_snapshot_truth: bool,
    /// Install update and tenant gating consume shared no new install truth.
    pub install_update_and_tenant_gating_consume_shared_no_new_install_truth: bool,
    /// Docs help and screenshots read single retirement source.
    pub docs_help_and_screenshots_read_single_retirement_source: bool,
    /// Archives and tombstones bind to shared build identity.
    pub archives_and_tombstones_bind_to_shared_build_identity: bool,
    /// Support export reads single retirement source.
    pub support_export_reads_single_retirement_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RetiredStateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof / audit refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the class.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the retired-state lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RetiredStateReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting retired-state audit for the lane.
    pub retired_state_audit_ref: String,
    /// True when support/export parity is required for every class.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every class.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RetiredStateMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RetiredStateMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Retired-state rows.
    pub retired_state_rows: Vec<M5RetiredStateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RetiredStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RetiredStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RetiredStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RetiredStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RetiredStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 retired-state matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RetiredStateMatrixPacket {
    /// Record kind; must equal [`M5_RETIRED_STATE_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RETIRED_STATE_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Retired-state rows.
    pub retired_state_rows: Vec<M5RetiredStateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RetiredStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RetiredStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RetiredStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RetiredStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RetiredStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RetiredStateMatrixPacket {
    /// Builds an M5 retired-state matrix packet from input.
    pub fn new(input: M5RetiredStateMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_RETIRED_STATE_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_RETIRED_STATE_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            retired_state_rows: input.retired_state_rows,
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

    /// Validates the M5 retired-state matrix invariants.
    pub fn validate(&self) -> Vec<M5RetiredStateMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_RETIRED_STATE_MATRIX_RECORD_KIND {
            violations.push(M5RetiredStateMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RETIRED_STATE_MATRIX_SCHEMA_VERSION {
            violations.push(M5RetiredStateMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RetiredStateMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_retired_state_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 retired-state matrix serializes"),
        ) {
            violations.push(M5RetiredStateMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 retired-state matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed retirement class.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_class,qualification,lifecycle_state,owner,backup_owner,canonical_schema,surface_families,removal_horizon_stages,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.retired_state_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.lifecycle_state.as_str(),
                csv_field(&row.owner_role),
                csv_field(&row.backup_owner_role),
                row.object_class.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.removal_horizon_stages, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic retired-surface-health dashboard JSON that release and support surfaces render from one
    /// canonical matrix instead of hand-authoring readiness chrome.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn render_dashboard_json(&self) -> String {
        let objects: Vec<serde_json::Value> = self
            .retired_state_rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "object_class": row.object_class.as_str(),
                    "qualification": row.qualification.as_str(),
                    "lifecycle_state": row.lifecycle_state.as_str(),
                    "canonical_schema": row.object_class.canonical_domain_schema_ref(),
                    "removal_horizon_stages": row
                        .removal_horizon_stages
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                    "consumer_surfaces": row
                        .consumer_surfaces
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                })
            })
            .collect();
        let dashboard = serde_json::json!({
            "record_kind": "m5_retired_surface_health",
            "packet_id": self.packet_id,
            "matrix_label": self.matrix_label,
            "matrix_schema_ref": M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
            "support_export_ref": M5_RETIRED_STATE_ARTIFACT_REF,
            "removal_horizon_stages": self.vocabulary_set.removal_horizon_stages,
            "downgrade_triggers": self.vocabulary_set.downgrade_triggers,
            "objects": objects,
        });
        serde_json::to_string_pretty(&dashboard)
            .expect("m5 retired-surface-health dashboard serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .retired_state_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Retired-State, End-of-Support Closure, Successor-Routing, and Tombstone/Archive Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Object classes: {} ({} stable)\n",
            self.retired_state_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Retirement roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Removal-horizon stages: {}\n",
            self.vocabulary_set.removal_horizon_stages.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last audit: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Object classes\n\n");
        for row in &self.retired_state_rows {
            out.push_str(&format!(
                "- **{}**: `{}` (lifecycle_state: `{}`)\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.lifecycle_state.as_str()
            ));
            out.push_str(&format!(
                "  - Owner: {} (backup: {})\n",
                row.owner_role, row.backup_owner_role
            ));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.object_class.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Successor path: {}\n",
                row.retirement_transition.successor_path
            ));
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

/// Errors emitted when reading the checked-in M5 retired-state matrix export.
#[derive(Debug)]
pub enum M5RetiredStateMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RetiredStateMatrixViolation>),
}

impl fmt::Display for M5RetiredStateMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 retired-state matrix export parse failed: {error}"
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
                    "m5 retired-state matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RetiredStateMatrixArtifactError {}

/// Validation failures emitted by [`M5RetiredStateMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RetiredStateMatrixViolation {
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
    /// A required governed object class is missing from the matrix.
    RequiredObjectMissing,
    /// A retired-state row is incomplete.
    RetiredStateRowIncomplete,
    /// A retired-state row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A retired-state row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A class declares no retirement roles.
    SemanticRoleMissing,
    /// The SupportedLine class declares no SupportedLine retirement-path roles.
    SupportedLineRoleMissing,
    /// The StableCapability class declares no StableCapability retirement-path roles.
    StableCapabilityRoleMissing,
    /// The Bundle class declares no Bundle retirement-path roles.
    BundleRoleMissing,
    /// The CommandDeepLink class declares no CommandDeepLink retirement-path roles.
    CommandDeepLinkRoleMissing,
    /// The SchemaBearingSurface class declares no SchemaBearingSurface retirement-path roles.
    SchemaBearingSurfaceRoleMissing,
    /// The RegistryVisiblePackage class declares no RegistryVisiblePackage retirement-path roles.
    RegistryVisiblePackageRoleMissing,
    /// The ManagedTenantFeature class declares no ManagedTenantFeature retirement-path roles.
    ManagedTenantFeatureRoleMissing,
    /// A class omits required transition metadata.
    TransitionMetadataIncomplete,
    /// A class declares no degraded reasons.
    DegradedReasonMissing,
    /// A class declares no surface families.
    SurfaceFamilyMissing,
    /// A class declares no removal-horizon stages.
    RemovalHorizonStageMissing,
    /// A class declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A class declares no first consumer surfaces.
    ConsumerSurfacesMissing,
    /// A class declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A class claiming Stable is missing required closure-artifact refs.
    StableObjectMissingClosureArtifact,
    /// A class violates a hard retirement invariant.
    RetiredStateInvariantViolated,
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

impl M5RetiredStateMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::RetiredStateRowIncomplete => "retired_state_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::SupportedLineRoleMissing => "supported_line_role_missing",
            Self::StableCapabilityRoleMissing => "stable_capability_role_missing",
            Self::BundleRoleMissing => "bundle_role_missing",
            Self::CommandDeepLinkRoleMissing => "command_deep_link_role_missing",
            Self::SchemaBearingSurfaceRoleMissing => "schema_bearing_surface_role_missing",
            Self::RegistryVisiblePackageRoleMissing => "registry_visible_package_role_missing",
            Self::ManagedTenantFeatureRoleMissing => "managed_tenant_feature_role_missing",
            Self::TransitionMetadataIncomplete => "transition_metadata_incomplete",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::RemovalHorizonStageMissing => "removal_horizon_stage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableObjectMissingClosureArtifact => "stable_object_missing_closure_artifact",
            Self::RetiredStateInvariantViolated => "retired_state_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 retired-state matrix export.
pub fn current_stable_m5_retired_state_matrix_export(
) -> Result<M5RetiredStateMatrixPacket, M5RetiredStateMatrixArtifactError> {
    let packet: M5RetiredStateMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-retirements/support_export.json"
    )))
    .map_err(M5RetiredStateMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RetiredStateMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5RetiredStateMatrixPacket,
    violations: &mut Vec<M5RetiredStateMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RETIRED_STATE_MATRIX_SCHEMA_REF,
        M5_RETIRED_STATE_MATRIX_DOC_REF,
        M5_RETIREMENT_MANIFEST_DOMAIN_SCHEMA_REF,
        M5_RETIREMENT_IMPACT_REPORT_DOMAIN_SCHEMA_REF,
        M5_LAST_SUPPORTED_SNAPSHOT_DOMAIN_SCHEMA_REF,
        M5_RETIREMENT_CLOSURE_LEDGER_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RetiredStateMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5RetiredStateMatrixPacket,
    violations: &mut Vec<M5RetiredStateMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5RetiredStateMatrixViolation::VocabularySetDrift);
    }
}

fn validate_retired_state_rows(
    packet: &M5RetiredStateMatrixPacket,
    violations: &mut Vec<M5RetiredStateMatrixViolation>,
) {
    let present: BTreeSet<M5RetiredStateObject> = packet
        .retired_state_rows
        .iter()
        .map(|row| row.object_class)
        .collect();
    for required in M5RetiredStateObject::ALL {
        if !present.contains(&required) {
            violations.push(M5RetiredStateMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.retired_state_rows {
        let class = row.object_class;
        if row.owner_role.trim().is_empty()
            || row.backup_owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5RetiredStateMatrixViolation::RetiredStateRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5RetiredStateMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == class.canonical_domain_schema_ref())
        {
            violations.push(M5RetiredStateMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5RetiredStateMatrixViolation::SemanticRoleMissing);
        }
        if class.declares_supported_line_roles() && row.supported_line_roles.is_empty() {
            violations.push(M5RetiredStateMatrixViolation::SupportedLineRoleMissing);
        }
        if class.declares_stable_capability_roles() && row.stable_capability_roles.is_empty() {
            violations.push(M5RetiredStateMatrixViolation::StableCapabilityRoleMissing);
        }
        if class.declares_bundle_roles() && row.bundle_roles.is_empty() {
            violations.push(M5RetiredStateMatrixViolation::BundleRoleMissing);
        }
        if class.declares_command_deep_link_roles() && row.command_deep_link_roles.is_empty() {
            violations.push(M5RetiredStateMatrixViolation::CommandDeepLinkRoleMissing);
        }
        if class.declares_schema_bearing_surface_roles()
            && row.schema_bearing_surface_roles.is_empty()
        {
            violations.push(M5RetiredStateMatrixViolation::SchemaBearingSurfaceRoleMissing);
        }
        if class.declares_registry_visible_package_roles()
            && row.registry_visible_package_roles.is_empty()
        {
            violations.push(M5RetiredStateMatrixViolation::RegistryVisiblePackageRoleMissing);
        }
        if class.declares_managed_tenant_feature_roles()
            && row.managed_tenant_feature_roles.is_empty()
        {
            violations.push(M5RetiredStateMatrixViolation::ManagedTenantFeatureRoleMissing);
        }
        if !row.retirement_transition.is_complete() {
            violations.push(M5RetiredStateMatrixViolation::TransitionMetadataIncomplete);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5RetiredStateMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5RetiredStateMatrixViolation::SurfaceFamilyMissing);
        }
        if row.removal_horizon_stages.is_empty() {
            violations.push(M5RetiredStateMatrixViolation::RemovalHorizonStageMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5RetiredStateMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5RetiredStateMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5RetiredStateMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_closure_artifact_refs.is_empty() {
            violations.push(M5RetiredStateMatrixViolation::StableObjectMissingClosureArtifact);
        }
        if !row.honours_invariants() {
            violations.push(M5RetiredStateMatrixViolation::RetiredStateInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5RetiredStateMatrixPacket,
    violations: &mut Vec<M5RetiredStateMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.no_retired_surface_disappears_without_tombstone_archival_route_or_successor_pointer,
        review.every_covered_object_class_names_owner_closure_artifacts_and_first_consumer,
        review.retired_is_mechanically_distinct_from_deprecated_disabled_and_narrowed,
        review.last_supported_snapshots_are_captured_before_retirement,
        review.successor_routing_and_cutoff_review_precede_every_retirement,
        review.support_and_public_proof_surfaces_close_cleanly_on_retirement,
        review.archival_and_tombstone_truth_is_preserved_after_retirement,
        review.no_new_installs_or_new_tenants_can_select_a_retired_class,
        review.retirement_state_stays_joined_to_exact_build_and_line_identity,
        review.every_object_declares_removal_horizon_stages,
        review.every_object_declares_accessibility_route,
        review.support_export_reads_single_retirement_source,
        review.release_help_and_support_bind_to_single_retirement_source,
        review.later_rows_cannot_invent_parallel_retirement_vocabulary,
        review.retirement_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_matrix_row_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5RetiredStateMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RetiredStateMatrixPacket,
    violations: &mut Vec<M5RetiredStateMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.release_and_help_consume_shared_retirement_truth,
        projection.support_and_marketplace_consume_shared_closure_and_snapshot_truth,
        projection.install_update_and_tenant_gating_consume_shared_no_new_install_truth,
        projection.docs_help_and_screenshots_read_single_retirement_source,
        projection.archives_and_tombstones_bind_to_shared_build_identity,
        projection.support_export_reads_single_retirement_source,
    ] {
        if !ok {
            violations.push(M5RetiredStateMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RetiredStateMatrixPacket,
    violations: &mut Vec<M5RetiredStateMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RetiredStateMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5RetiredStateMatrixPacket,
    violations: &mut Vec<M5RetiredStateMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.retired_state_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RetiredStateMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses retirement / successor / disable / archival words; what is rejected is a raw secret
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
