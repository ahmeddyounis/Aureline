//! Implemented M5 permission-manifest-summary and transitive-capability-drawer primitives.
//!
//! The frozen [marketplace / install-review component matrix][matrix] names the reusable
//! extension-marketplace UI components and locks their controlled vocabulary. This module is the
//! third implement lane over that matrix: it turns the permission-review component — the
//! **permission manifest summary** and its **transitive capability drawer** — into resolvers that
//! produce export-safe, honest projections, so a user can read the permission posture, the required
//! / optional / inherited capability classes, the runtime / host model, the data / network
//! boundaries, and any transitive or dependency-contributed widening from the listing, detail,
//! install, update, and diagnostics surfaces without a vague "full access" label quietly standing in
//! for the manifest.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render permission-manifest summaries with required, optional, and inherited capability
//!   classes plus runtime / host model and data / network boundaries.**
//!   [`resolve_permission_manifest_summary`] refuses to read as a clean summary when the artifact
//!   identity, permission posture, or host / runtime model is unstated, when a capability-requesting
//!   posture names no required-capability grouping, when the data / network boundary is unstated,
//!   when the summary flattens the manifest into one vague full-access label, or when it cannot be
//!   traced back to a canonical manifest digest; it degrades instead.
//! * **Support disclosure of transitive widening and dependency-contributed permissions without
//!   flattening everything into one full-access label.** [`resolve_transitive_capability_drawer`]
//!   degrades when a transitively-widened posture hides its widening, when dependency-contributed
//!   permissions carry no attribution to the dependency that contributed them, when the drawer
//!   collapses into one full-access label, or when it is severed from the manifest digest.
//! * **Keep listing / detail / install / diagnostics views aligned to the same manifest digest and
//!   grouping model.** Both resolvers carry the manifest digest and narrow the moment it is missing,
//!   and the packet proves — by resolved examples, not governance bools — that the permission posture
//!   stays explicit across surfaces and that transitive widening stays visible and attributable
//!   before install trust silently continues.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5MarketplaceInstallDisposition`] marketplace / install-disposition vocabulary, the
//! [`M5PermissionPostureState`] permission-posture vocabulary, and the [`M5HostRuntimeModel`] host /
//! runtime vocabulary — so marketplace, extensions, registry, help, and support surfaces can never
//! fork their own permission wording or invent feature-local badges. Raw secret values and private
//! endpoints stay outside the export boundary.
//!
//! [matrix]: crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_permission_manifest_controls,
    seeded_m5_permission_manifest_controls_install_review_ui_preview_narrowed,
    seeded_m5_permission_manifest_controls_marketplace_ui_beta_narrowed,
    M5_PERMISSION_MANIFEST_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix::{
    M5HostRuntimeModel, M5MarketplaceInstallAccessibilityRoute, M5MarketplaceInstallComponentFamily,
    M5MarketplaceInstallConsumerSurface, M5MarketplaceInstallDeploymentLine,
    M5MarketplaceInstallDisposition, M5MarketplaceInstallDowngradeTrigger,
    M5MarketplaceInstallQualificationClass, M5MarketplaceInstallRequiredLabel,
    M5PermissionPostureState, M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
    M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF, M5_PERMISSION_MANIFEST_SUMMARY_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5PermissionManifestControlsPacket`].
pub const M5_PERMISSION_MANIFEST_CONTROLS_RECORD_KIND: &str =
    "implement_m5_permission_manifest_summary_and_transitive_capability_drawer_controls";

/// Schema version for M5 permission-manifest-summary / transitive-capability-drawer controls records.
pub const M5_PERMISSION_MANIFEST_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_PERMISSION_MANIFEST_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-permission-manifest-summary-transitive-capability-drawer-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_PERMISSION_MANIFEST_CONTROLS_DOC_REF: &str =
    "docs/marketplace/m5_permission_manifest_summary_and_transitive_capability_drawer_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PERMISSION_MANIFEST_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-permission-manifest-summary-transitive-capability-drawer-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_PERMISSION_MANIFEST_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-permission-manifest-summary-transitive-capability-drawer-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_PERMISSION_MANIFEST_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-permission-manifest-summary-transitive-capability-drawer-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_PERMISSION_MANIFEST_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-permission-manifest-summary-transitive-capability-drawer-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface
/// taxonomy so no lane invents a parallel surface set.
pub type M5PermissionManifestConsumerSurface = M5MarketplaceInstallConsumerSurface;

/// Controlled capability class a permission-manifest summary groups its capabilities under, so a
/// required, optional, or inherited (dependency-contributed) permission is never flattened into one
/// full-access label. Minted by this lane because the frozen matrix carries a permission posture but
/// not the required / optional / inherited grouping model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PermissionCapabilityClass {
    /// Required for the artifact to function.
    Required,
    /// Optional, granted only when a feature is used.
    Optional,
    /// Inherited from a dependency (dependency-contributed).
    Inherited,
}

impl M5PermissionCapabilityClass {
    /// Every capability class, in declaration order.
    pub const ALL: [Self; 3] = [Self::Required, Self::Optional, Self::Inherited];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::Inherited => "inherited",
        }
    }
}

/// Controlled boundary class a permission-manifest summary must disclose, so the data, network, and
/// runtime reach of a permission set is always named rather than hidden behind compact chrome. Minted
/// by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PermissionBoundaryClass {
    /// The data the artifact can read or write.
    DataAccess,
    /// The network reach the artifact requests.
    NetworkAccess,
    /// The runtime / host the artifact executes in.
    RuntimeHost,
}

impl M5PermissionBoundaryClass {
    /// Every boundary class, in declaration order.
    pub const ALL: [Self; 3] = [Self::DataAccess, Self::NetworkAccess, Self::RuntimeHost];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DataAccess => "data_access",
            Self::NetworkAccess => "network_access",
            Self::RuntimeHost => "runtime_host",
        }
    }
}

/// One mandatory rendered part a permission-manifest summary or transitive-capability drawer must be
/// able to show, so no capability class, boundary, or widening fact is left implicit behind compact
/// chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PermissionManifestAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The permission posture (both components).
    PermissionPosture,
    /// The required capability class (summary).
    RequiredCapabilities,
    /// The optional capability class (summary).
    OptionalCapabilities,
    /// The inherited / dependency-contributed capability class (summary).
    InheritedCapabilities,
    /// The runtime / host model (summary).
    RuntimeHostModel,
    /// The data boundary (summary).
    DataBoundary,
    /// The network boundary (summary).
    NetworkBoundary,
    /// The transitive-widening disclosure (drawer).
    TransitiveWidening,
    /// The dependency attribution for contributed permissions (drawer).
    DependencyAttribution,
    /// The canonical manifest digest (both components).
    ManifestDigest,
}

impl M5PermissionManifestAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::PermissionPosture,
        Self::RequiredCapabilities,
        Self::OptionalCapabilities,
        Self::InheritedCapabilities,
        Self::RuntimeHostModel,
        Self::DataBoundary,
        Self::NetworkBoundary,
        Self::TransitiveWidening,
        Self::DependencyAttribution,
        Self::ManifestDigest,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::PermissionPosture => "permission_posture",
            Self::RequiredCapabilities => "required_capabilities",
            Self::OptionalCapabilities => "optional_capabilities",
            Self::InheritedCapabilities => "inherited_capabilities",
            Self::RuntimeHostModel => "runtime_host_model",
            Self::DataBoundary => "data_boundary",
            Self::NetworkBoundary => "network_boundary",
            Self::TransitiveWidening => "transitive_widening",
            Self::DependencyAttribution => "dependency_attribution",
            Self::ManifestDigest => "manifest_digest",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to review the fact
/// behind a degraded permission component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PermissionManifestNextAction {
    /// Review the permission posture.
    ReviewPermissionPosture,
    /// Review the required / optional / inherited capability classes.
    ReviewCapabilityClasses,
    /// Review the data / network boundary.
    ReviewDataNetworkBoundary,
    /// Review the transitive widening and dependency attribution.
    ReviewTransitiveWidening,
    /// Review the canonical manifest digest.
    ReviewManifestDigest,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5PermissionManifestNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewPermissionPosture,
        Self::ReviewCapabilityClasses,
        Self::ReviewDataNetworkBoundary,
        Self::ReviewTransitiveWidening,
        Self::ReviewManifestDigest,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewPermissionPosture => "review_permission_posture",
            Self::ReviewCapabilityClasses => "review_capability_classes",
            Self::ReviewDataNetworkBoundary => "review_data_network_boundary",
            Self::ReviewTransitiveWidening => "review_transitive_widening",
            Self::ReviewManifestDigest => "review_manifest_digest",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PermissionManifestExportField {
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
    /// The permission posture named by the summary and drawer.
    PermissionPosture,
    /// The required / optional / inherited capability classes named by the summary.
    CapabilityClasses,
    /// The data / network boundary named by the summary.
    DataNetworkBoundary,
    /// The transitive-widening disclosure named by the drawer.
    TransitiveWidening,
    /// The dependency attribution named by the drawer.
    DependencyAttribution,
    /// The canonical manifest digest named by both components.
    ManifestDigest,
}

impl M5PermissionManifestExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::PermissionPosture,
        Self::CapabilityClasses,
        Self::DataNetworkBoundary,
        Self::TransitiveWidening,
        Self::DependencyAttribution,
        Self::ManifestDigest,
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
            Self::PermissionPosture => "permission_posture",
            Self::CapabilityClasses => "capability_classes",
            Self::DataNetworkBoundary => "data_network_boundary",
            Self::TransitiveWidening => "transitive_widening",
            Self::DependencyAttribution => "dependency_attribution",
            Self::ManifestDigest => "manifest_digest",
        }
    }
}

/// Reason a permission-manifest summary degraded below a clean, fully-legible state. The degrade-first
/// ladder returns one of these instead of ever letting an ambiguous summary read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PermissionManifestSummaryDegradeReason {
    /// The artifact identity is unstated.
    ArtifactIdentityUnstated,
    /// The permission posture cannot currently be resolved.
    PermissionPostureUnresolved,
    /// The host / runtime model cannot currently be resolved.
    HostModelUnresolved,
    /// A capability-requesting posture names no required-capability grouping.
    CapabilityGroupingUnstated,
    /// The data / network boundary is unstated.
    DataNetworkBoundaryUnstated,
    /// The manifest is flattened into one vague full-access label.
    FlattenedIntoFullAccess,
    /// The summary cannot be traced back to a canonical manifest digest.
    ManifestDigestUnstated,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PermissionManifestSummaryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ArtifactIdentityUnstated,
        Self::PermissionPostureUnresolved,
        Self::HostModelUnresolved,
        Self::CapabilityGroupingUnstated,
        Self::DataNetworkBoundaryUnstated,
        Self::FlattenedIntoFullAccess,
        Self::ManifestDigestUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentityUnstated => "artifact_identity_unstated",
            Self::PermissionPostureUnresolved => "permission_posture_unresolved",
            Self::HostModelUnresolved => "host_model_unresolved",
            Self::CapabilityGroupingUnstated => "capability_grouping_unstated",
            Self::DataNetworkBoundaryUnstated => "data_network_boundary_unstated",
            Self::FlattenedIntoFullAccess => "flattened_into_full_access",
            Self::ManifestDigestUnstated => "manifest_digest_unstated",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5PermissionManifestNextAction {
        match self {
            Self::ArtifactIdentityUnstated
            | Self::PermissionPostureUnresolved
            | Self::HostModelUnresolved => M5PermissionManifestNextAction::ReviewPermissionPosture,
            Self::CapabilityGroupingUnstated | Self::FlattenedIntoFullAccess => {
                M5PermissionManifestNextAction::ReviewCapabilityClasses
            }
            Self::DataNetworkBoundaryUnstated => {
                M5PermissionManifestNextAction::ReviewDataNetworkBoundary
            }
            Self::ManifestDigestUnstated => M5PermissionManifestNextAction::ReviewManifestDigest,
            Self::ProofStale => M5PermissionManifestNextAction::ReviewPermissionPosture,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5MarketplaceInstallDowngradeTrigger {
        match self {
            Self::ArtifactIdentityUnstated | Self::ManifestDigestUnstated => {
                M5MarketplaceInstallDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::PermissionPostureUnresolved
            | Self::CapabilityGroupingUnstated
            | Self::DataNetworkBoundaryUnstated
            | Self::FlattenedIntoFullAccess => {
                M5MarketplaceInstallDowngradeTrigger::PermissionWideningHidden
            }
            Self::HostModelUnresolved => M5MarketplaceInstallDowngradeTrigger::HostModelUnstated,
            Self::ProofStale => M5MarketplaceInstallDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a transitive-capability drawer degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TransitiveCapabilityDrawerDegradeReason {
    /// The artifact identity is unstated.
    ArtifactIdentityUnstated,
    /// The permission posture cannot currently be resolved.
    PermissionPostureUnresolved,
    /// A transitively-widened posture hides its widening.
    TransitiveWideningHidden,
    /// Dependency-contributed permissions carry no attribution to their dependency.
    DependencyAttributionMissing,
    /// The drawer collapses into one vague full-access label.
    FlattenedIntoFullAccess,
    /// The drawer cannot be traced back to a canonical manifest digest.
    ManifestDigestUnstated,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5TransitiveCapabilityDrawerDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ArtifactIdentityUnstated,
        Self::PermissionPostureUnresolved,
        Self::TransitiveWideningHidden,
        Self::DependencyAttributionMissing,
        Self::FlattenedIntoFullAccess,
        Self::ManifestDigestUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentityUnstated => "artifact_identity_unstated",
            Self::PermissionPostureUnresolved => "permission_posture_unresolved",
            Self::TransitiveWideningHidden => "transitive_widening_hidden",
            Self::DependencyAttributionMissing => "dependency_attribution_missing",
            Self::FlattenedIntoFullAccess => "flattened_into_full_access",
            Self::ManifestDigestUnstated => "manifest_digest_unstated",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5PermissionManifestNextAction {
        match self {
            Self::ArtifactIdentityUnstated | Self::PermissionPostureUnresolved => {
                M5PermissionManifestNextAction::ReviewPermissionPosture
            }
            Self::TransitiveWideningHidden | Self::DependencyAttributionMissing => {
                M5PermissionManifestNextAction::ReviewTransitiveWidening
            }
            Self::FlattenedIntoFullAccess => {
                M5PermissionManifestNextAction::ReviewCapabilityClasses
            }
            Self::ManifestDigestUnstated => M5PermissionManifestNextAction::ReviewManifestDigest,
            Self::ProofStale => M5PermissionManifestNextAction::ReviewPermissionPosture,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5MarketplaceInstallDowngradeTrigger {
        match self {
            Self::ArtifactIdentityUnstated | Self::ManifestDigestUnstated => {
                M5MarketplaceInstallDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::TransitiveWideningHidden | Self::DependencyAttributionMissing => {
                M5MarketplaceInstallDowngradeTrigger::TransitivePermissionHidden
            }
            Self::PermissionPostureUnresolved | Self::FlattenedIntoFullAccess => {
                M5MarketplaceInstallDowngradeTrigger::PermissionWideningHidden
            }
            Self::ProofStale => M5MarketplaceInstallDowngradeTrigger::ProofStale,
        }
    }
}

/// True when the permission posture requests a capability set the summary must group.
fn posture_requests_capabilities(posture: M5PermissionPostureState) -> bool {
    matches!(
        posture,
        M5PermissionPostureState::Minimal
            | M5PermissionPostureState::Standard
            | M5PermissionPostureState::Elevated
            | M5PermissionPostureState::WidenedTransitive
    )
}

/// True when the permission posture widens permissions transitively through dependencies.
fn posture_widens_transitively(posture: M5PermissionPostureState) -> bool {
    matches!(posture, M5PermissionPostureState::WidenedTransitive)
}

/// Input to [`resolve_permission_manifest_summary`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PermissionManifestSummaryResolutionInput {
    /// Stable identity of the summary instance.
    pub summary_id: String,
    /// The artifact identity (name / id) shown; empty means unstated.
    pub artifact_identity: String,
    /// The permission posture.
    pub posture: M5PermissionPostureState,
    /// The host / runtime model.
    pub host_runtime_model: M5HostRuntimeModel,
    /// The required-capability grouping; empty means unstated for a capability-requesting posture.
    pub required_capabilities: Vec<String>,
    /// The optional-capability grouping.
    pub optional_capabilities: Vec<String>,
    /// The inherited / dependency-contributed capability grouping.
    pub inherited_capabilities: Vec<String>,
    /// The data boundary the artifact reaches; empty means unstated.
    pub data_boundary: String,
    /// The network boundary the artifact reaches; empty means unstated.
    pub network_boundary: String,
    /// The canonical manifest digest the summary is grouped from; empty means unstated.
    pub manifest_digest: String,
    /// True when the summary flattens the manifest into one vague full-access label.
    pub flattens_into_full_access: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe permission-manifest summary projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPermissionManifestSummary {
    /// Stable identity of the summary instance.
    pub summary_id: String,
    /// The artifact identity named by the summary.
    pub artifact_identity: String,
    /// The permission-posture token named by the summary.
    pub posture: String,
    /// Whether the posture requests a capability set that must be grouped.
    pub requests_capabilities: bool,
    /// Whether the posture widens permissions transitively.
    pub widens_transitively: bool,
    /// The host / runtime token named by the summary.
    pub host_runtime_model: String,
    /// The required-capability grouping named by the summary.
    pub required_capabilities: Vec<String>,
    /// The optional-capability grouping named by the summary.
    pub optional_capabilities: Vec<String>,
    /// The inherited / dependency-contributed grouping named by the summary.
    pub inherited_capabilities: Vec<String>,
    /// The data boundary named by the summary.
    pub data_boundary: String,
    /// The network boundary named by the summary.
    pub network_boundary: String,
    /// The canonical manifest digest named by the summary.
    pub manifest_digest: String,
    /// Guardrail (MUST be `false` on a clean summary): the manifest is flattened into one vague
    /// full-access label.
    pub flattens_into_full_access: bool,
    /// Degrade reason, if the summary could not read as a clean state.
    pub degrade_reason: Option<M5PermissionManifestSummaryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5PermissionManifestNextAction,
    /// Whether the permission facts are legible in full (clean summary naming every fact).
    pub fully_legible: bool,
}

impl M5ResolvedPermissionManifestSummary {
    /// Whether this summary reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_transitive_capability_drawer`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TransitiveCapabilityDrawerResolutionInput {
    /// Stable identity of the drawer instance.
    pub drawer_id: String,
    /// The artifact identity (name / id) shown; empty means unstated.
    pub artifact_identity: String,
    /// The permission posture.
    pub posture: M5PermissionPostureState,
    /// True when a transitively-widened posture's widening is disclosed on the drawer.
    pub transitive_widening_disclosed: bool,
    /// The dependency-contributed capabilities the drawer discloses.
    pub dependency_contributed_capabilities: Vec<String>,
    /// The attribution lines naming which dependency contributed each permission; empty means the
    /// contributed permissions are unattributed.
    pub dependency_attributions: Vec<String>,
    /// The canonical manifest digest the drawer is grouped from; empty means unstated.
    pub manifest_digest: String,
    /// True when the drawer collapses into one vague full-access label.
    pub flattens_into_full_access: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe transitive-capability drawer projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTransitiveCapabilityDrawer {
    /// Stable identity of the drawer instance.
    pub drawer_id: String,
    /// The artifact identity named by the drawer.
    pub artifact_identity: String,
    /// The permission-posture token named by the drawer.
    pub posture: String,
    /// Whether the posture widens permissions transitively.
    pub widens_transitively: bool,
    /// Whether the transitive widening is disclosed.
    pub transitive_widening_disclosed: bool,
    /// The dependency-contributed capabilities named by the drawer.
    pub dependency_contributed_capabilities: Vec<String>,
    /// The attribution lines naming which dependency contributed each permission.
    pub dependency_attributions: Vec<String>,
    /// The canonical manifest digest named by the drawer.
    pub manifest_digest: String,
    /// Guardrail (MUST be `false` on a clean drawer): the drawer collapses into one vague full-access
    /// label.
    pub flattens_into_full_access: bool,
    /// Guardrail (MUST be `false` on a clean drawer): a transitively-widened posture hides its
    /// widening.
    pub hides_transitive_widening: bool,
    /// Degrade reason, if the drawer could not read as a clean state.
    pub degrade_reason: Option<M5TransitiveCapabilityDrawerDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5PermissionManifestNextAction,
    /// Whether the transitive-capability facts are legible in full (clean drawer naming every fact).
    pub fully_legible: bool,
}

impl M5ResolvedTransitiveCapabilityDrawer {
    /// Whether this drawer reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5PermissionManifestResolutionError {
    /// The summary id was empty.
    EmptySummaryId,
    /// The drawer id was empty.
    EmptyDrawerId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5PermissionManifestResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySummaryId => "empty_summary_id",
            Self::EmptyDrawerId => "empty_drawer_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5PermissionManifestResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 permission-manifest resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5PermissionManifestResolutionError {}

/// Resolves a permission-manifest summary, keeping the permission posture explicit at search, detail,
/// install, update, and diagnostics time: the summary names its permission posture, required /
/// optional / inherited capability classes, runtime / host model, and data / network boundaries,
/// never flattens the manifest into one vague full-access label, and narrows the claim the moment it
/// cannot be traced back to a canonical manifest digest.
pub fn resolve_permission_manifest_summary(
    input: M5PermissionManifestSummaryResolutionInput,
) -> Result<M5ResolvedPermissionManifestSummary, M5PermissionManifestResolutionError> {
    if input.summary_id.trim().is_empty() {
        return Err(M5PermissionManifestResolutionError::EmptySummaryId);
    }
    if string_is_forbidden(&input.summary_id)
        || string_is_forbidden(&input.artifact_identity)
        || string_is_forbidden(&input.data_boundary)
        || string_is_forbidden(&input.network_boundary)
        || string_is_forbidden(&input.manifest_digest)
        || input
            .required_capabilities
            .iter()
            .any(|c| string_is_forbidden(c))
        || input
            .optional_capabilities
            .iter()
            .any(|c| string_is_forbidden(c))
        || input
            .inherited_capabilities
            .iter()
            .any(|c| string_is_forbidden(c))
    {
        return Err(M5PermissionManifestResolutionError::ForbiddenMaterial);
    }

    let requests_capabilities = posture_requests_capabilities(input.posture);
    let widens_transitively = posture_widens_transitively(input.posture);

    let degrade_reason = if input.artifact_identity.trim().is_empty() {
        Some(M5PermissionManifestSummaryDegradeReason::ArtifactIdentityUnstated)
    } else if matches!(input.posture, M5PermissionPostureState::PostureUnknown) {
        Some(M5PermissionManifestSummaryDegradeReason::PermissionPostureUnresolved)
    } else if matches!(input.host_runtime_model, M5HostRuntimeModel::HostUnknown) {
        Some(M5PermissionManifestSummaryDegradeReason::HostModelUnresolved)
    } else if requests_capabilities && input.required_capabilities.is_empty() {
        Some(M5PermissionManifestSummaryDegradeReason::CapabilityGroupingUnstated)
    } else if input.data_boundary.trim().is_empty() || input.network_boundary.trim().is_empty() {
        Some(M5PermissionManifestSummaryDegradeReason::DataNetworkBoundaryUnstated)
    } else if input.flattens_into_full_access {
        Some(M5PermissionManifestSummaryDegradeReason::FlattenedIntoFullAccess)
    } else if input.manifest_digest.trim().is_empty() {
        Some(M5PermissionManifestSummaryDegradeReason::ManifestDigestUnstated)
    } else if !input.proof_fresh {
        Some(M5PermissionManifestSummaryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5PermissionManifestNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedPermissionManifestSummary {
        summary_id: input.summary_id,
        artifact_identity: input.artifact_identity,
        posture: input.posture.as_str().to_owned(),
        requests_capabilities,
        widens_transitively,
        host_runtime_model: input.host_runtime_model.as_str().to_owned(),
        required_capabilities: input.required_capabilities,
        optional_capabilities: input.optional_capabilities,
        inherited_capabilities: input.inherited_capabilities,
        data_boundary: input.data_boundary,
        network_boundary: input.network_boundary,
        manifest_digest: input.manifest_digest,
        flattens_into_full_access: input.flattens_into_full_access,
        degrade_reason,
        next_action,
        fully_legible: degrade_reason.is_none(),
    })
}

/// Resolves a transitive-capability drawer, keeping transitive widening visible and attributable
/// before install trust silently continues: the drawer discloses a transitively-widened posture,
/// attributes each dependency-contributed permission to the dependency that contributed it, never
/// collapses into one vague full-access label, and narrows the claim the moment it cannot be traced
/// back to a canonical manifest digest.
pub fn resolve_transitive_capability_drawer(
    input: M5TransitiveCapabilityDrawerResolutionInput,
) -> Result<M5ResolvedTransitiveCapabilityDrawer, M5PermissionManifestResolutionError> {
    if input.drawer_id.trim().is_empty() {
        return Err(M5PermissionManifestResolutionError::EmptyDrawerId);
    }
    if string_is_forbidden(&input.drawer_id)
        || string_is_forbidden(&input.artifact_identity)
        || string_is_forbidden(&input.manifest_digest)
        || input
            .dependency_contributed_capabilities
            .iter()
            .any(|c| string_is_forbidden(c))
        || input
            .dependency_attributions
            .iter()
            .any(|c| string_is_forbidden(c))
    {
        return Err(M5PermissionManifestResolutionError::ForbiddenMaterial);
    }

    let widens_transitively = posture_widens_transitively(input.posture);
    let hides_transitive_widening = widens_transitively && !input.transitive_widening_disclosed;
    let contributed_present = !input.dependency_contributed_capabilities.is_empty();
    let attribution_missing = contributed_present && input.dependency_attributions.is_empty();

    let degrade_reason = if input.artifact_identity.trim().is_empty() {
        Some(M5TransitiveCapabilityDrawerDegradeReason::ArtifactIdentityUnstated)
    } else if matches!(input.posture, M5PermissionPostureState::PostureUnknown) {
        Some(M5TransitiveCapabilityDrawerDegradeReason::PermissionPostureUnresolved)
    } else if hides_transitive_widening {
        Some(M5TransitiveCapabilityDrawerDegradeReason::TransitiveWideningHidden)
    } else if attribution_missing {
        Some(M5TransitiveCapabilityDrawerDegradeReason::DependencyAttributionMissing)
    } else if input.flattens_into_full_access {
        Some(M5TransitiveCapabilityDrawerDegradeReason::FlattenedIntoFullAccess)
    } else if input.manifest_digest.trim().is_empty() {
        Some(M5TransitiveCapabilityDrawerDegradeReason::ManifestDigestUnstated)
    } else if !input.proof_fresh {
        Some(M5TransitiveCapabilityDrawerDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5PermissionManifestNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedTransitiveCapabilityDrawer {
        drawer_id: input.drawer_id,
        artifact_identity: input.artifact_identity,
        posture: input.posture.as_str().to_owned(),
        widens_transitively,
        transitive_widening_disclosed: input.transitive_widening_disclosed,
        dependency_contributed_capabilities: input.dependency_contributed_capabilities,
        dependency_attributions: input.dependency_attributions,
        manifest_digest: input.manifest_digest,
        flattens_into_full_access: input.flattens_into_full_access,
        hides_transitive_widening,
        degrade_reason,
        next_action,
        fully_legible: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved permission-manifest summary and
/// transitive-capability drawer examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PermissionManifestControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5PermissionManifestConsumerSurface,
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
    pub anatomy_parts: Vec<M5PermissionManifestAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5PermissionManifestExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5MarketplaceInstallDowngradeTrigger>,
    /// Resolved permission-manifest summary examples.
    pub permission_manifest_summary_examples: Vec<M5ResolvedPermissionManifestSummary>,
    /// Resolved transitive-capability drawer examples.
    pub transitive_capability_drawer_examples: Vec<M5ResolvedTransitiveCapabilityDrawer>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the component schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never flatten permissions into a vague full-access label.
    pub flattens_permissions_into_vague_full_access: bool,
    /// Hard invariant: never hide transitive or dependency-contributed widening.
    pub hides_transitive_or_dependency_contributed_widening: bool,
    /// Hard invariant: never hide the data / network / runtime boundary.
    pub hides_data_network_or_runtime_boundary: bool,
    /// Hard invariant: never sever a summary from its canonical manifest digest.
    pub severs_summary_from_canonical_manifest_digest: bool,
}

impl M5PermissionManifestControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5PermissionManifestAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5PermissionManifestAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5PermissionManifestExportField> =
            self.export_fields.iter().copied().collect();
        M5PermissionManifestExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.flattens_permissions_into_vague_full_access
            && !self.hides_transitive_or_dependency_contributed_widening
            && !self.hides_data_network_or_runtime_boundary
            && !self.severs_summary_from_canonical_manifest_digest
    }

    /// True when every resolved example on this row is honest: no clean summary flattens the manifest
    /// into one full-access label, and no clean drawer flattens into a full-access label or hides its
    /// transitive widening.
    fn examples_are_honest(&self) -> bool {
        self.permission_manifest_summary_examples
            .iter()
            .all(|ex| !(ex.is_clean() && ex.flattens_into_full_access))
            && self.transitive_capability_drawer_examples.iter().all(|ex| {
                !(ex.is_clean() && (ex.flattens_into_full_access || ex.hides_transitive_widening))
            })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PermissionManifestVocabularySet {
    /// Marketplace / install-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Permission-posture tokens (bound from the frozen matrix).
    pub permission_postures: Vec<String>,
    /// Host / runtime-model tokens (bound from the frozen matrix).
    pub host_runtime_models: Vec<String>,
    /// Capability-class tokens (minted by this lane).
    pub capability_classes: Vec<String>,
    /// Boundary-class tokens (minted by this lane).
    pub boundary_classes: Vec<String>,
    /// Permission-manifest-summary degrade-reason tokens.
    pub permission_manifest_summary_degrade_reasons: Vec<String>,
    /// Transitive-capability-drawer degrade-reason tokens.
    pub transitive_capability_drawer_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5PermissionManifestVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5MarketplaceInstallDisposition::ALL, |v| v.as_str()),
            permission_postures: tokens(&M5PermissionPostureState::ALL, |v| v.as_str()),
            host_runtime_models: tokens(&M5HostRuntimeModel::ALL, |v| v.as_str()),
            capability_classes: tokens(&M5PermissionCapabilityClass::ALL, |v| v.as_str()),
            boundary_classes: tokens(&M5PermissionBoundaryClass::ALL, |v| v.as_str()),
            permission_manifest_summary_degrade_reasons: tokens(
                &M5PermissionManifestSummaryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            transitive_capability_drawer_degrade_reasons: tokens(
                &M5TransitiveCapabilityDrawerDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5PermissionManifestAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5PermissionManifestNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5PermissionManifestExportField::ALL, |v| v.as_str()),
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
pub struct M5PermissionManifestGovernanceReview {
    /// The permission-manifest summary names its permission posture and required / optional /
    /// inherited capability classes.
    pub summary_names_posture_and_capability_classes: bool,
    /// The permission-manifest summary names its runtime / host model and data / network boundaries.
    pub summary_names_runtime_host_and_boundaries: bool,
    /// The transitive-capability drawer discloses transitive widening.
    pub drawer_discloses_transitive_widening: bool,
    /// The transitive-capability drawer attributes dependency-contributed permissions.
    pub drawer_attributes_dependency_contributed_permissions: bool,
    /// Permissions are never flattened into one vague full-access label.
    pub permissions_never_flattened_into_full_access: bool,
    /// Transitive and dependency-contributed widening is always visible and attributable.
    pub transitive_widening_always_visible_and_attributable: bool,
    /// The data / network / runtime boundary is always explicit, never hidden behind chrome.
    pub data_network_runtime_boundary_always_explicit: bool,
    /// Permission summaries always trace back to one canonical manifest digest.
    pub summaries_trace_to_single_manifest_digest: bool,
    /// Permission posture stays explicit across search, detail, install, update, and diagnostics
    /// views.
    pub posture_explicit_across_all_surfaces: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PermissionManifestConsumerProjection {
    /// Marketplace and extensions surfaces consume the shared permission-posture / capability-class
    /// vocabulary.
    pub marketplace_surfaces_consume_permission_vocabulary: bool,
    /// Install and update surfaces consume the shared transitive-widening / attribution vocabulary.
    pub install_surfaces_consume_transitive_widening_vocabulary: bool,
    /// Permission facts trace back to one canonical component contract.
    pub facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical permission source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PermissionManifestProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PermissionManifestReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5PermissionManifestControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PermissionManifestControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5PermissionManifestControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PermissionManifestVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PermissionManifestGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PermissionManifestConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PermissionManifestProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PermissionManifestReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 permission-manifest-summary / transitive-capability-drawer controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PermissionManifestControlsPacket {
    /// Record kind; must equal [`M5_PERMISSION_MANIFEST_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PERMISSION_MANIFEST_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5PermissionManifestControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PermissionManifestVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PermissionManifestGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PermissionManifestConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PermissionManifestProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PermissionManifestReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PermissionManifestControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5PermissionManifestControlsPacketInput) -> Self {
        Self {
            record_kind: M5_PERMISSION_MANIFEST_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_PERMISSION_MANIFEST_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5PermissionManifestControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PERMISSION_MANIFEST_CONTROLS_RECORD_KIND {
            violations.push(M5PermissionManifestControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PERMISSION_MANIFEST_CONTROLS_SCHEMA_VERSION {
            violations.push(M5PermissionManifestControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PermissionManifestControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5PermissionManifestControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 permission-manifest controls packet serializes"),
        ) {
            violations.push(M5PermissionManifestControlsViolation::RawMaterialInExport);
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
            .expect("m5 permission-manifest controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,summary_examples,drawer_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .permission_manifest_summary_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.transitive_capability_drawer_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.permission_manifest_summary_examples.len(),
                row.transitive_capability_drawer_examples.len(),
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
            "# M5 Permission-Manifest-Summary and Transitive-Capability-Drawer Controls\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Permission postures: {}\n",
            self.vocabulary_set.permission_postures.join(", ")
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
                "  - Summary examples: {} / transitive-drawer examples: {}\n",
                row.permission_manifest_summary_examples.len(),
                row.transitive_capability_drawer_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5PermissionManifestControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PermissionManifestControlsViolation>),
}

impl fmt::Display for M5PermissionManifestControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 permission-manifest controls export parse failed: {error}"
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
                    "m5 permission-manifest controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PermissionManifestControlsArtifactError {}

/// Validation failures emitted by [`M5PermissionManifestControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PermissionManifestControlsViolation {
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
    /// A controls row does not point at the component schema.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (flattened full-access or hidden widening).
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
    /// Permission-posture explicitness is not proven: no clean summary names posture, capability
    /// classes, boundaries, and a manifest digest, or a posture / boundary / digest example does not
    /// degrade, or a clean example flattens the manifest into full access.
    PermissionPostureExplicitNotProven,
    /// Transitive-widening attributability is not proven: no clean drawer discloses transitive
    /// widening, or a hidden-widening / missing-attribution example does not degrade, or a clean
    /// drawer hides its widening.
    TransitiveWideningAttributableNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5PermissionManifestControlsViolation {
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
            Self::PermissionPostureExplicitNotProven => "permission_posture_explicit_not_proven",
            Self::TransitiveWideningAttributableNotProven => {
                "transitive_widening_attributable_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_permission_manifest_controls_export(
) -> Result<M5PermissionManifestControlsPacket, M5PermissionManifestControlsArtifactError> {
    let packet: M5PermissionManifestControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-permission-manifest-summary-transitive-capability-drawer-controls-proof/support_export.json"
    )))
    .map_err(M5PermissionManifestControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PermissionManifestControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5PermissionManifestControlsPacket,
    violations: &mut Vec<M5PermissionManifestControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PERMISSION_MANIFEST_CONTROLS_SCHEMA_REF,
        M5_PERMISSION_MANIFEST_CONTROLS_DOC_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
        M5_PERMISSION_MANIFEST_SUMMARY_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5PermissionManifestControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5PermissionManifestControlsPacket,
    violations: &mut Vec<M5PermissionManifestControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5PermissionManifestControlsViolation::NoControlsRows);
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
            violations.push(M5PermissionManifestControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5PermissionManifestControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5PermissionManifestControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_PERMISSION_MANIFEST_SUMMARY_SCHEMA_REF) {
            violations.push(M5PermissionManifestControlsViolation::ComponentSchemaRefMissing);
        }
        if row.permission_manifest_summary_examples.is_empty()
            || row.transitive_capability_drawer_examples.is_empty()
        {
            violations.push(M5PermissionManifestControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5PermissionManifestControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5PermissionManifestControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5PermissionManifestControlsPacket,
    violations: &mut Vec<M5PermissionManifestControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.summary_names_posture_and_capability_classes,
        review.summary_names_runtime_host_and_boundaries,
        review.drawer_discloses_transitive_widening,
        review.drawer_attributes_dependency_contributed_permissions,
        review.permissions_never_flattened_into_full_access,
        review.transitive_widening_always_visible_and_attributable,
        review.data_network_runtime_boundary_always_explicit,
        review.summaries_trace_to_single_manifest_digest,
        review.posture_explicit_across_all_surfaces,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5PermissionManifestControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5PermissionManifestControlsPacket,
    violations: &mut Vec<M5PermissionManifestControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.marketplace_surfaces_consume_permission_vocabulary,
        projection.install_surfaces_consume_transitive_widening_vocabulary,
        projection.facts_trace_to_single_component_contract,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5PermissionManifestControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5PermissionManifestControlsPacket,
    violations: &mut Vec<M5PermissionManifestControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5PermissionManifestControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5PermissionManifestControlsPacket,
    violations: &mut Vec<M5PermissionManifestControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5PermissionManifestControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5PermissionManifestControlsPacket,
    violations: &mut Vec<M5PermissionManifestControlsViolation>,
) {
    let summaries = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.permission_manifest_summary_examples.iter())
    };
    let drawers = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.transitive_capability_drawer_examples.iter())
    };

    // AC: permission posture stays explicit at search, detail, install, update, and diagnostics time,
    // and every summary can be traced back to one canonical manifest grouping contract. A clean
    // summary names its posture, a required-capability grouping, a data / network boundary, and a
    // manifest digest; a posture-unresolved summary degrades; a boundary-unstated summary degrades; a
    // manifest-digest-unstated summary degrades; and no clean summary or drawer flattens the manifest
    // into one vague full-access label.
    let clean_summary_explicit = summaries().any(|ex| {
        ex.is_clean()
            && ex.requests_capabilities
            && !ex.required_capabilities.is_empty()
            && !ex.data_boundary.trim().is_empty()
            && !ex.network_boundary.trim().is_empty()
            && !ex.manifest_digest.trim().is_empty()
    });
    let posture_unresolved_degrades = summaries().any(|ex| {
        ex.degrade_reason
            == Some(M5PermissionManifestSummaryDegradeReason::PermissionPostureUnresolved)
    });
    let boundary_unstated_degrades = summaries().any(|ex| {
        ex.degrade_reason
            == Some(M5PermissionManifestSummaryDegradeReason::DataNetworkBoundaryUnstated)
    });
    let digest_unstated_degrades = summaries().any(|ex| {
        ex.degrade_reason == Some(M5PermissionManifestSummaryDegradeReason::ManifestDigestUnstated)
    }) || drawers().any(|ex| {
        ex.degrade_reason == Some(M5TransitiveCapabilityDrawerDegradeReason::ManifestDigestUnstated)
    });
    let no_clean_flatten = summaries().all(|ex| !(ex.is_clean() && ex.flattens_into_full_access))
        && drawers().all(|ex| !(ex.is_clean() && ex.flattens_into_full_access));
    if !(clean_summary_explicit
        && posture_unresolved_degrades
        && boundary_unstated_degrades
        && digest_unstated_degrades
        && no_clean_flatten)
    {
        violations.push(M5PermissionManifestControlsViolation::PermissionPostureExplicitNotProven);
    }

    // AC: transitive widening is visible and attributable before trust silently continues. A clean
    // drawer discloses a transitively-widened posture, a hidden-widening drawer degrades, a
    // missing-attribution drawer degrades, and no clean drawer hides its widening.
    let clean_drawer_discloses = drawers()
        .any(|ex| ex.is_clean() && ex.widens_transitively && ex.transitive_widening_disclosed);
    let widening_hidden_degrades = drawers().any(|ex| {
        ex.degrade_reason
            == Some(M5TransitiveCapabilityDrawerDegradeReason::TransitiveWideningHidden)
    });
    let attribution_missing_degrades = drawers().any(|ex| {
        ex.degrade_reason
            == Some(M5TransitiveCapabilityDrawerDegradeReason::DependencyAttributionMissing)
    });
    let no_clean_hides_widening =
        drawers().all(|ex| !(ex.is_clean() && ex.hides_transitive_widening));
    if !(clean_drawer_discloses
        && widening_hidden_degrades
        && attribution_missing_degrades
        && no_clean_hides_widening)
    {
        violations
            .push(M5PermissionManifestControlsViolation::TransitiveWideningAttributableNotProven);
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

/// The single component family this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5MarketplaceInstallComponentFamily; 1] =
    [M5MarketplaceInstallComponentFamily::PermissionManifestSummary];
