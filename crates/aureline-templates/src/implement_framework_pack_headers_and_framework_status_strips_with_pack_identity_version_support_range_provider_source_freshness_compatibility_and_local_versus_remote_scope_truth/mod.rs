//! Two reusable M5 framework-aware components — the framework pack header and the framework
//! status strip — so a user can tell which framework pack and version is active, how it is
//! supported, who provides it, whether the active experience is core native, pack-backed,
//! bridged, or heuristic, and whether the current scope is local or remote before they trust a
//! framework lens: the pack header names its pack identity and version range, its support class,
//! its provider source, its selected workspace scope, its freshness, and its derived support
//! posture, experience class, and scope posture, and offers a first-class open-compatibility-
//! details action; the status strip preserves the detected framework and version, the pack
//! health, the compatibility notes, and the bridge-or-heuristic posture wherever a
//! framework-aware feature is claimed.
//!
//! Aureline's frozen framework-component matrix
//! ([`crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix`])
//! names the framework pack header as one governed component family and freezes its controlled
//! vocabulary — the one controlled certainty disposition (`core_native`, `framework_pack`,
//! `bridge`, `heuristic_convention`, `verified`, `derived_by_convention`, `runtime_confirmed`,
//! `partial`); the pack support classes (`officially_supported`, `community_supported`,
//! `experimental`, `bridge_only`, `deprecated`, `unsupported`); the pack identity states
//! (`identified_versioned`, `version_pinned`, `version_drifted`, `multiple_detected`,
//! `unversioned`, `unknown_pack`); the execution boundary classes (`local_process`, `container`,
//! `ssh_remote`, `managed_workspace`, `cloud_remote`, `unknown_boundary`) that pin where a
//! framework experience actually runs; the surface families; the deployment lines; the consumer
//! surfaces; the accessibility routes; the required labels; and the downgrade triggers. This
//! module *implements* that contract as two co-equal component vectors — a full pack header and a
//! compact status strip — that share one resolver so a claimed M5 framework-pack, route-explorer,
//! topology-explorer, convention-diagnostics, or generator-review surface can project a header and
//! a strip that keep the same identity, support, experience, and scope truth.
//!
//! The module has one derived resolver:
//!
//! * [`resolve_framework_pack_posture`] — takes a pack's frozen support class, pack identity
//!   state, certainty disposition, and execution boundary and derives its support posture (fully
//!   supported, community supported, experimental / bridge, or unsupported / deprecated), its
//!   experience class (core native, pack-backed, bridged, or heuristic), its scope posture (local,
//!   container, remote, managed, or unknown), whether the support is exact first-party support,
//!   whether the scope is local, and which notes the component must carry — so bridge or heuristic
//!   behavior can never read as exact first-party support, a drifted or multiple-detected pack can
//!   never leave its identity implicit, and a remote, managed, container, or unknown scope can
//!   never read as local before a user trusts a framework lens.
//!
//! A single controls packet — [`FrameworkPackHeaderStatusStripControlsPacket`] — binds one vector
//! of pack headers and one vector of status strips to the same identity / support / experience /
//! scope, freshness / health, and non-visual accessibility vocabulary, so pack identity and
//! framework certainty stay explicit across the framework-pack, route / topology, diagnostics,
//! generator-review, CLI, and support consumers.
//!
//! The pack support class ([`M5FrameworkPackSupportClass`]), pack identity state
//! ([`M5FrameworkPackIdentityState`]), certainty disposition
//! ([`M5FrameworkCertaintyDisposition`]), execution boundary class
//! ([`M5ExecutionBoundaryClass`]), component family ([`M5FrameworkComponentFamily`]), surface
//! family ([`M5FrameworkSurfaceFamily`]), deployment line ([`M5FrameworkDeploymentLine`]),
//! consumer surface ([`M5FrameworkConsumerSurface`]), accessibility route
//! ([`M5FrameworkAccessibilityRoute`]), required label ([`M5FrameworkRequiredLabel`]), and
//! downgrade trigger ([`M5FrameworkDowngradeTrigger`]) are reused verbatim from the frozen matrix.
//! This module mints new vocabulary only for what that matrix left implicit about the two
//! components themselves: the derived support posture, the derived framework-experience class the
//! acceptance criteria pin, the derived scope posture, the pack freshness and health states, the
//! bounded pack-header and status-strip actions, and the deep-link kinds. No M5 framework surface
//! invents a second pack-header or status-strip grammar.
//!
//! Raw file bodies, raw manifests, pasted local paths, repository URLs, credentials, and secrets
//! stay outside the export boundary; every note, deep-link reference, and component identity is
//! carried only as an opaque, export-safe representation.

#[cfg(test)]
mod tests;

// The pack support classes, pack identity states, certainty disposition, execution boundary
// classes, component family, and the surface / deployment / consumer / accessibility / label /
// downgrade vocabularies are frozen once, in the framework-component matrix. This lane reuses them
// verbatim so it never invents a parallel pack-header or status-strip vocabulary.
pub use crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix::{
    M5ExecutionBoundaryClass, M5FrameworkAccessibilityRoute, M5FrameworkCertaintyDisposition,
    M5FrameworkComponentFamily, M5FrameworkConsumerSurface, M5FrameworkDeploymentLine,
    M5FrameworkDowngradeTrigger, M5FrameworkPackIdentityState, M5FrameworkPackSupportClass,
    M5FrameworkRequiredLabel, M5FrameworkSurfaceFamily, M5_FRAMEWORK_COMPONENT_DOC_REF,
    M5_FRAMEWORK_COMPONENT_SCHEMA_REF, M5_FRAMEWORK_PACK_HEADER_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`FrameworkPackHeaderStatusStripControlsPacket`].
pub const FRAMEWORK_PACK_HEADER_CONTROLS_RECORD_KIND: &str =
    "implement_framework_pack_headers_and_framework_status_strips_with_pack_identity_version_support_range_provider_source_freshness_compatibility_and_local_versus_remote_scope_truth";

/// Schema version for M5 framework-pack-header / status-strip control records.
pub const FRAMEWORK_PACK_HEADER_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const FRAMEWORK_PACK_HEADER_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-framework-pack-header-status-strip-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const FRAMEWORK_PACK_HEADER_CONTROLS_DOC_REF: &str =
    "docs/frameworks/m5/m5_framework_pack_header_status_strip_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const FRAMEWORK_PACK_HEADER_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-framework-pack-header-status-strip-controls";

/// Repo-relative path of the checked support-export artifact.
pub const FRAMEWORK_PACK_HEADER_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-framework-pack-header-status-strip-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const FRAMEWORK_PACK_HEADER_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-framework-pack-header-status-strip-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const FRAMEWORK_PACK_HEADER_CONTROLS_REPORT_REF: &str =
    "artifacts/design/m5-framework-pack-header-status-strip.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a framework component binds its next step against, so a pack
/// header or status strip never routes through an ephemeral overlay — every next step is a stable
/// pack manifest, provider-registry entry, docs, or compatibility reference the user can reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable pack-manifest reference.
    PackManifest,
    /// A stable provider-registry entry reference.
    ProviderRegistryEntry,
    /// A stable docs anchor.
    DocsAnchor,
    /// A stable compatibility reference.
    CompatibilityReference,
    /// No deep link is bound (the component names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PackManifest,
        Self::ProviderRegistryEntry,
        Self::DocsAnchor,
        Self::CompatibilityReference,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackManifest => "pack_manifest",
            Self::ProviderRegistryEntry => "provider_registry_entry",
            Self::DocsAnchor => "docs_anchor",
            Self::CompatibilityReference => "compatibility_reference",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- derived framework-pack vocabulary ----------------------------------

/// Derived support posture a framework pack header or status strip may present.
///
/// This is the support honesty axis: the posture is derived from the frozen pack support class,
/// never asserted, so bridge or heuristic behavior can never present as exact first-party support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackSupportPosture {
    /// Officially / fully supported.
    FullySupported,
    /// Community-supported, best effort.
    CommunitySupported,
    /// Experimental or bridge-only behavior, not exact first-party support.
    ExperimentalOrBridge,
    /// Unsupported or deprecated.
    UnsupportedOrDeprecated,
}

impl PackSupportPosture {
    /// Every support posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullySupported,
        Self::CommunitySupported,
        Self::ExperimentalOrBridge,
        Self::UnsupportedOrDeprecated,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullySupported => "fully_supported",
            Self::CommunitySupported => "community_supported",
            Self::ExperimentalOrBridge => "experimental_or_bridge",
            Self::UnsupportedOrDeprecated => "unsupported_or_deprecated",
        }
    }

    /// True only when the pack carries exact first-party support.
    pub const fn is_exact_first_party_support(self) -> bool {
        matches!(self, Self::FullySupported)
    }
}

/// Derived framework-experience class a component may present. These are the exact
/// acceptance-criteria labels so a user can tell at a glance whether the active framework
/// experience is core native, pack-backed, bridged, or heuristic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkExperienceClass {
    /// Core-native behavior owned by Aureline directly.
    CoreNative,
    /// Pack-backed behavior provided by a framework pack (including verified or runtime-confirmed
    /// pack truth).
    PackBacked,
    /// Bridge behavior, not exact core-native or first-party pack support.
    Bridged,
    /// A heuristic convention rather than an exact fact.
    Heuristic,
}

impl FrameworkExperienceClass {
    /// Every experience class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CoreNative,
        Self::PackBacked,
        Self::Bridged,
        Self::Heuristic,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreNative => "core_native",
            Self::PackBacked => "pack_backed",
            Self::Bridged => "bridged",
            Self::Heuristic => "heuristic",
        }
    }

    /// True when the experience is bridged or heuristic and must therefore never read as exact
    /// first-party support.
    pub const fn is_bridge_or_heuristic(self) -> bool {
        matches!(self, Self::Bridged | Self::Heuristic)
    }
}

/// Derived scope posture a component may present — whether the active framework experience runs
/// locally or remotely, so a remote, managed, container, or unknown scope can never read as local.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkScopePosture {
    /// Runs as a local process on this machine.
    LocalScope,
    /// Runs in a container on this machine.
    ContainerScope,
    /// Runs on an SSH or cloud remote.
    RemoteScope,
    /// Runs in a managed workspace.
    ManagedScope,
    /// Scope could not be resolved.
    UnknownScope,
}

impl FrameworkScopePosture {
    /// Every scope posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalScope,
        Self::ContainerScope,
        Self::RemoteScope,
        Self::ManagedScope,
        Self::UnknownScope,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalScope => "local_scope",
            Self::ContainerScope => "container_scope",
            Self::RemoteScope => "remote_scope",
            Self::ManagedScope => "managed_scope",
            Self::UnknownScope => "unknown_scope",
        }
    }

    /// True only when the active framework experience runs locally on this machine.
    pub const fn is_local(self) -> bool {
        matches!(self, Self::LocalScope)
    }
}

/// Pack freshness state a framework pack header carries, so a stale or never-scanned pack signal
/// never reads as current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackFreshnessState {
    /// The pack signal is current.
    Current,
    /// The pack signal was imported from another environment.
    Imported,
    /// The pack signal is stale.
    Stale,
    /// The pack has never been scanned.
    NeverScanned,
    /// Freshness is unknown.
    Unknown,
}

impl PackFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Current,
        Self::Imported,
        Self::Stale,
        Self::NeverScanned,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Imported => "imported",
            Self::Stale => "stale",
            Self::NeverScanned => "never_scanned",
            Self::Unknown => "unknown",
        }
    }

    /// True when the freshness signal must carry an explicit not-current note.
    pub const fn needs_note(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// Pack health class a framework status strip carries, so a degraded, compatibility-warned, or
/// broken pack never reads as healthy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackHealthClass {
    /// The pack is healthy.
    Healthy,
    /// The pack is degraded.
    Degraded,
    /// The pack carries a compatibility warning.
    CompatibilityWarning,
    /// The pack is broken.
    Broken,
    /// Pack health is unknown.
    Unknown,
}

impl PackHealthClass {
    /// Every health class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Healthy,
        Self::Degraded,
        Self::CompatibilityWarning,
        Self::Broken,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::CompatibilityWarning => "compatibility_warning",
            Self::Broken => "broken",
            Self::Unknown => "unknown",
        }
    }

    /// True when the health signal must carry an explicit non-healthy note.
    pub const fn needs_note(self) -> bool {
        !matches!(self, Self::Healthy)
    }
}

/// Disclosures a framework component must carry, derived from the support class, pack identity
/// state, certainty disposition, and execution boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameworkPackDisclosure {
    /// The derived support posture this component may present.
    pub support_posture: PackSupportPosture,
    /// The derived framework-experience class this component may present.
    pub experience_class: FrameworkExperienceClass,
    /// The derived scope posture this component may present.
    pub scope_posture: FrameworkScopePosture,
    /// Whether the pack carries exact first-party support.
    pub is_exact_first_party_support: bool,
    /// Whether the active experience is core native.
    pub is_core_native_experience: bool,
    /// Whether the active experience is bridged or heuristic.
    pub is_bridge_or_heuristic_experience: bool,
    /// Whether the active scope is local.
    pub is_local_scope: bool,
    /// Whether the component must carry an explicit not-exactly-first-party support note.
    pub needs_nonexact_support_note: bool,
    /// Whether the component must carry an explicit bridge / heuristic note.
    pub needs_bridge_or_heuristic_note: bool,
    /// Whether the component must carry an explicit remote / managed / unknown-scope note.
    pub needs_remote_scope_note: bool,
    /// Whether the component must carry an explicit version-drift note.
    pub needs_version_drift_note: bool,
    /// Whether the component must carry an explicit multiple-pack note.
    pub needs_multiple_pack_note: bool,
    /// Whether the component must carry an explicit unversioned-pack note.
    pub needs_unversioned_note: bool,
    /// Whether the component must carry an explicit unknown-pack note.
    pub needs_unknown_pack_note: bool,
}

/// Resolves the support, experience, and scope truth a framework pack header or status strip may
/// present.
///
/// An `officially_supported` pack is fully supported; a `community_supported` one is
/// community-supported; an `experimental` or `bridge_only` one is experimental / bridge; a
/// `deprecated` or `unsupported` one is unsupported / deprecated — so bridge or heuristic behavior
/// can never read as exact first-party support. A `core_native` certainty is a core-native
/// experience; a `framework_pack`, `verified`, or `runtime_confirmed` certainty is pack-backed; a
/// `bridge` certainty is bridged; a `heuristic_convention`, `derived_by_convention`, or `partial`
/// certainty is heuristic. A `local_process` boundary is a local scope, a `container` is a
/// container scope, an `ssh_remote` or `cloud_remote` is a remote scope, a `managed_workspace` is
/// a managed scope, and an `unknown_boundary` is an unknown scope — so a remote, managed,
/// container, or unknown scope can never read as local.
pub fn resolve_framework_pack_posture(
    support: M5FrameworkPackSupportClass,
    identity: M5FrameworkPackIdentityState,
    certainty: M5FrameworkCertaintyDisposition,
    boundary: M5ExecutionBoundaryClass,
) -> FrameworkPackDisclosure {
    use FrameworkExperienceClass as Experience;
    use FrameworkScopePosture as Scope;
    use M5ExecutionBoundaryClass as Boundary;
    use M5FrameworkCertaintyDisposition as Certainty;
    use M5FrameworkPackIdentityState as Identity;
    use M5FrameworkPackSupportClass as Support;
    use PackSupportPosture as Posture;

    let support_posture = match support {
        Support::OfficiallySupported => Posture::FullySupported,
        Support::CommunitySupported => Posture::CommunitySupported,
        Support::Experimental | Support::BridgeOnly => Posture::ExperimentalOrBridge,
        Support::Deprecated | Support::Unsupported => Posture::UnsupportedOrDeprecated,
    };
    let experience_class = match certainty {
        Certainty::CoreNative => Experience::CoreNative,
        Certainty::FrameworkPack | Certainty::Verified | Certainty::RuntimeConfirmed => {
            Experience::PackBacked
        }
        Certainty::Bridge => Experience::Bridged,
        Certainty::HeuristicConvention | Certainty::DerivedByConvention | Certainty::Partial => {
            Experience::Heuristic
        }
    };
    let scope_posture = match boundary {
        Boundary::LocalProcess => Scope::LocalScope,
        Boundary::Container => Scope::ContainerScope,
        Boundary::SshRemote | Boundary::CloudRemote => Scope::RemoteScope,
        Boundary::ManagedWorkspace => Scope::ManagedScope,
        Boundary::UnknownBoundary => Scope::UnknownScope,
    };

    FrameworkPackDisclosure {
        support_posture,
        experience_class,
        scope_posture,
        is_exact_first_party_support: support_posture.is_exact_first_party_support(),
        is_core_native_experience: matches!(experience_class, Experience::CoreNative),
        is_bridge_or_heuristic_experience: experience_class.is_bridge_or_heuristic(),
        is_local_scope: scope_posture.is_local(),
        needs_nonexact_support_note: !support_posture.is_exact_first_party_support(),
        needs_bridge_or_heuristic_note: experience_class.is_bridge_or_heuristic(),
        needs_remote_scope_note: !scope_posture.is_local(),
        needs_version_drift_note: matches!(identity, Identity::VersionDrifted),
        needs_multiple_pack_note: matches!(identity, Identity::MultipleDetected),
        needs_unversioned_note: matches!(identity, Identity::Unversioned),
        needs_unknown_pack_note: matches!(identity, Identity::UnknownPack),
    }
}

/// One keyboard-complete default action a framework pack header offers, so a header never hides
/// its compatibility-details, inspect, or scope affordance behind a pointer-only gesture.
/// `OpenCompatibilityDetails`, `InspectPackSourceAndSupport`, and `ReviewScopeBoundary` are always
/// offered so support, source, and scope posture are inspectable before a user trusts the lens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackHeaderAction {
    /// Open the compatibility details (always available).
    OpenCompatibilityDetails,
    /// Inspect the pack source and support class (always available).
    InspectPackSourceAndSupport,
    /// Review the scope boundary the framework experience runs against (always available).
    ReviewScopeBoundary,
    /// Select the workspace scope.
    SelectWorkspaceScope,
    /// Open the stable manifest / registry / docs / compatibility deep link.
    OpenDeepLink,
    /// Copy the stable pack id.
    CopyPackId,
}

impl PackHeaderAction {
    /// Every pack-header action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenCompatibilityDetails,
        Self::InspectPackSourceAndSupport,
        Self::ReviewScopeBoundary,
        Self::SelectWorkspaceScope,
        Self::OpenDeepLink,
        Self::CopyPackId,
    ];

    /// The default actions every keyboard-complete pack header must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenCompatibilityDetails,
        Self::InspectPackSourceAndSupport,
        Self::ReviewScopeBoundary,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCompatibilityDetails => "open_compatibility_details",
            Self::InspectPackSourceAndSupport => "inspect_pack_source_and_support",
            Self::ReviewScopeBoundary => "review_scope_boundary",
            Self::SelectWorkspaceScope => "select_workspace_scope",
            Self::OpenDeepLink => "open_deep_link",
            Self::CopyPackId => "copy_pack_id",
        }
    }
}

/// One keyboard-complete default action a framework status strip offers, so a strip never hides
/// its inspect or review affordance behind a pointer-only gesture. `InspectFrameworkAndVersion`
/// and `ReviewCompatibilityNotes` are always offered so framework identity and compatibility stay
/// inspectable wherever a framework-aware feature is claimed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusStripAction {
    /// Inspect the detected framework and version (always available).
    InspectFrameworkAndVersion,
    /// Review the compatibility notes (always available).
    ReviewCompatibilityNotes,
    /// Open the full pack header.
    OpenPackHeader,
    /// Open the stable manifest / registry / docs / compatibility deep link.
    OpenDeepLink,
    /// Copy the stable pack id.
    CopyPackId,
}

impl StatusStripAction {
    /// Every status-strip action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InspectFrameworkAndVersion,
        Self::ReviewCompatibilityNotes,
        Self::OpenPackHeader,
        Self::OpenDeepLink,
        Self::CopyPackId,
    ];

    /// The default actions every keyboard-complete status strip must offer.
    pub const MANDATORY: [Self; 2] = [
        Self::InspectFrameworkAndVersion,
        Self::ReviewCompatibilityNotes,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectFrameworkAndVersion => "inspect_framework_and_version",
            Self::ReviewCompatibilityNotes => "review_compatibility_notes",
            Self::OpenPackHeader => "open_pack_header",
            Self::OpenDeepLink => "open_deep_link",
            Self::CopyPackId => "copy_pack_id",
        }
    }
}

/// A framework pack header naming its pack identity and version range, support class, provider
/// source, selected workspace scope, freshness, certainty, and execution boundary, with a derived
/// support posture, experience class, and scope posture, bounded open-compatibility-details /
/// inspect / scope actions, and a stable manifest / registry / docs / compatibility deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkPackHeader {
    /// Frozen component this control implements; must be `framework_pack_header`.
    pub component: M5FrameworkComponentFamily,
    /// Stable pack id.
    pub pack_id: String,
    /// Human-readable pack name; required and non-empty.
    pub pack_name: String,
    /// Detected framework name; required and non-empty.
    pub framework_name: String,
    /// Framework / version range label; required and non-empty.
    pub framework_version_range: String,
    /// Pack support class, reused from the frozen matrix.
    pub support_class: M5FrameworkPackSupportClass,
    /// Pack identity state, reused from the frozen matrix.
    pub identity_state: M5FrameworkPackIdentityState,
    /// Certainty disposition, reused from the frozen matrix.
    pub certainty: M5FrameworkCertaintyDisposition,
    /// Execution boundary the framework experience runs against.
    pub execution_boundary: M5ExecutionBoundaryClass,
    /// Derived support posture (must equal the resolved posture).
    pub derived_support_posture: PackSupportPosture,
    /// Derived experience class (must equal the resolved class).
    pub derived_experience_class: FrameworkExperienceClass,
    /// Derived scope posture (must equal the resolved posture).
    pub derived_scope_posture: FrameworkScopePosture,
    /// Whether the header claims exact first-party support (must equal derived truth).
    pub claims_exact_first_party_support: bool,
    /// Whether the header claims a local scope (must equal derived truth).
    pub claims_local_scope: bool,
    /// Provider source label; always required so who provides the pack stays explicit.
    pub provider_source_label: String,
    /// Selected workspace scope label; always required.
    pub workspace_scope_label: String,
    /// Pack freshness state.
    pub freshness_state: PackFreshnessState,
    /// Freshness label; always required so how current the signal is stays explicit.
    pub freshness_label: String,
    /// Not-exactly-first-party support note; required when support is not fully supported.
    pub nonexact_support_note: String,
    /// Bridge / heuristic note; required when the experience is bridged or heuristic.
    pub bridge_or_heuristic_note: String,
    /// Remote / managed / unknown scope note; required when the scope is not local.
    pub remote_scope_note: String,
    /// Version-drift note; required when the pack version drifted from the pinned version.
    pub version_drift_note: String,
    /// Multiple-pack note; required when multiple candidate packs were detected.
    pub multiple_pack_note: String,
    /// Unversioned note; required when the pack is detected but unversioned.
    pub unversioned_note: String,
    /// Unknown-pack note; required when the pack could not be resolved.
    pub unknown_pack_note: String,
    /// Pack source / certainty note; always required so which pack and how certain stays explicit.
    pub pack_source_and_certainty_note: String,
    /// Opaque compatibility-details reference; always required.
    pub compatibility_details_ref: String,
    /// Context note; always required so the header names what to check before trusting the lens.
    pub context_note: String,
    /// Kind of stable deep link this header binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub header_actions: Vec<PackHeaderAction>,
    /// Certainty dispositions this header binds (required, from the one shared vocabulary).
    pub dispositions: Vec<M5FrameworkCertaintyDisposition>,
    /// Downgrade triggers this header can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Mandatory labels this header can show (must include the mandatory labels).
    pub required_labels: Vec<M5FrameworkRequiredLabel>,
    /// Claimed M5 surface families that render this header.
    pub surface_families: Vec<M5FrameworkSurfaceFamily>,
    /// Deployment lines this header keeps the same truth across.
    pub deployment_lines: Vec<M5FrameworkDeploymentLine>,
    /// Non-visual accessibility routes this header offers.
    pub accessibility_routes: Vec<M5FrameworkAccessibilityRoute>,
    /// Framework subsystems that consume this header's projection.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this header.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides its pack identity, version, or support class. MUST be `false`.
    pub hides_pack_identity_or_support_class: bool,
    /// Hard invariant: never lets bridge or heuristic behavior masquerade as exact. MUST be
    /// `false`.
    pub lets_heuristic_masquerade_as_exact: bool,
    /// Hard invariant: never hides the local / container / SSH / managed boundary. MUST be
    /// `false`.
    pub hides_local_container_ssh_or_managed_boundary: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl FrameworkPackHeader {
    /// Support / experience / scope disclosures this header must carry, derived from the frozen
    /// classes.
    pub fn posture_disclosure(&self) -> FrameworkPackDisclosure {
        resolve_framework_pack_posture(
            self.support_class,
            self.identity_state,
            self.certainty,
            self.execution_boundary,
        )
    }

    /// Whether the header offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<PackHeaderAction> = self.header_actions.iter().copied().collect();
        PackHeaderAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the header declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }

    /// Whether the header offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.header_actions
            .contains(&PackHeaderAction::OpenDeepLink)
    }
}

/// A framework status strip preserving the detected framework and version, the pack health, the
/// compatibility notes, and the bridge-or-heuristic posture wherever a framework-aware feature is
/// claimed, with a derived support posture, experience class, and scope posture, bounded inspect /
/// review actions, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkStatusStrip {
    /// Frozen component this control implements; must be `framework_pack_header`.
    pub component: M5FrameworkComponentFamily,
    /// Stable pack id.
    pub pack_id: String,
    /// Detected framework label; required and non-empty.
    pub detected_framework_label: String,
    /// Detected version label; required and non-empty.
    pub detected_version_label: String,
    /// Pack support class, reused from the frozen matrix.
    pub support_class: M5FrameworkPackSupportClass,
    /// Pack identity state, reused from the frozen matrix.
    pub identity_state: M5FrameworkPackIdentityState,
    /// Certainty disposition, reused from the frozen matrix.
    pub certainty: M5FrameworkCertaintyDisposition,
    /// Execution boundary the framework experience runs against.
    pub execution_boundary: M5ExecutionBoundaryClass,
    /// Derived support posture (must equal the resolved posture).
    pub derived_support_posture: PackSupportPosture,
    /// Derived experience class (must equal the resolved class).
    pub derived_experience_class: FrameworkExperienceClass,
    /// Derived scope posture (must equal the resolved posture).
    pub derived_scope_posture: FrameworkScopePosture,
    /// Whether the strip claims exact first-party support (must equal derived truth).
    pub claims_exact_first_party_support: bool,
    /// Whether the strip claims a local scope (must equal derived truth).
    pub claims_local_scope: bool,
    /// Pack health class.
    pub pack_health_class: PackHealthClass,
    /// Pack health label; always required so pack health stays explicit.
    pub pack_health_label: String,
    /// Compatibility notes label; always required so compatibility notes stay explicit.
    pub compatibility_notes_label: String,
    /// Not-exactly-first-party support note; required when support is not fully supported.
    pub nonexact_support_note: String,
    /// Bridge / heuristic note; required when the experience is bridged or heuristic.
    pub bridge_or_heuristic_note: String,
    /// Remote / managed / unknown scope note; required when the scope is not local.
    pub remote_scope_note: String,
    /// Version-drift note; required when the pack version drifted from the pinned version.
    pub version_drift_note: String,
    /// Degraded-health note; required when the pack is not healthy.
    pub degraded_health_note: String,
    /// Pack source / certainty note; always required so which pack and how certain stays explicit.
    pub pack_source_and_certainty_note: String,
    /// Context note; always required so the strip names what to check before trusting the lens.
    pub context_note: String,
    /// Kind of stable deep link this strip binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub strip_actions: Vec<StatusStripAction>,
    /// Certainty dispositions this strip binds (required, from the one shared vocabulary).
    pub dispositions: Vec<M5FrameworkCertaintyDisposition>,
    /// Downgrade triggers this strip can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Mandatory labels this strip can show (must include the mandatory labels).
    pub required_labels: Vec<M5FrameworkRequiredLabel>,
    /// Claimed M5 surface families that render this strip.
    pub surface_families: Vec<M5FrameworkSurfaceFamily>,
    /// Deployment lines this strip keeps the same truth across.
    pub deployment_lines: Vec<M5FrameworkDeploymentLine>,
    /// Non-visual accessibility routes this strip offers.
    pub accessibility_routes: Vec<M5FrameworkAccessibilityRoute>,
    /// Framework subsystems that consume this strip's projection.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this strip.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides its pack identity, version, or support class. MUST be `false`.
    pub hides_pack_identity_or_support_class: bool,
    /// Hard invariant: never lets bridge or heuristic behavior masquerade as exact. MUST be
    /// `false`.
    pub lets_heuristic_masquerade_as_exact: bool,
    /// Hard invariant: never hides the local / container / SSH / managed boundary. MUST be
    /// `false`.
    pub hides_local_container_ssh_or_managed_boundary: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl FrameworkStatusStrip {
    /// Support / experience / scope disclosures this strip must carry, derived from the frozen
    /// classes.
    pub fn posture_disclosure(&self) -> FrameworkPackDisclosure {
        resolve_framework_pack_posture(
            self.support_class,
            self.identity_state,
            self.certainty,
            self.execution_boundary,
        )
    }

    /// Whether the strip offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<StatusStripAction> = self.strip_actions.iter().copied().collect();
        StatusStripAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the strip declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }

    /// Whether the strip offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.strip_actions
            .contains(&StatusStripAction::OpenDeepLink)
    }
}

/// Whether a required-label list declares all three mandatory labels.
fn declares_mandatory_labels(labels: &[M5FrameworkRequiredLabel]) -> bool {
    let present: BTreeSet<M5FrameworkRequiredLabel> = labels.iter().copied().collect();
    M5FrameworkRequiredLabel::MANDATORY
        .iter()
        .all(|label| present.contains(label))
}

// ---- review blocks ------------------------------------------------------

/// First-glance framework-pack review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkPackReview {
    /// The pack header names its pack identity, version, and support class.
    pub pack_header_shows_identity_and_support: bool,
    /// The pack header names its provider source and workspace scope.
    pub pack_header_shows_provider_and_scope: bool,
    /// The pack header offers an open-compatibility-details action.
    pub pack_header_offers_compatibility_details: bool,
    /// The status strip names its detected framework and version.
    pub status_strip_shows_framework_and_version: bool,
    /// The status strip names its pack health and compatibility notes.
    pub status_strip_shows_health_and_compatibility: bool,
    /// The status strip offers inspect and review.
    pub status_strip_offers_inspect_and_review: bool,
    /// Support, experience, and scope are derived from state, never asserted.
    pub support_experience_and_scope_derived_never_asserted: bool,
    /// Bridge or heuristic behavior is never shown as exact first-party support.
    pub bridge_or_heuristic_never_shown_as_exact: bool,
    /// A drifted or multiple-detected pack always discloses its identity.
    pub version_drift_and_multiple_pack_always_disclosed: bool,
    /// A remote, managed, container, or unknown scope is never shown as local.
    pub remote_scope_never_shown_as_local: bool,
    /// Freshness and health stay explicit.
    pub freshness_and_health_always_explicit: bool,
    /// Every next step names one stable manifest / registry / docs / compatibility deep link.
    pub every_next_step_names_stable_deep_link: bool,
    /// The execution boundary stays visible.
    pub execution_boundary_always_visible: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl FrameworkPackReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.pack_header_shows_identity_and_support
            && self.pack_header_shows_provider_and_scope
            && self.pack_header_offers_compatibility_details
            && self.status_strip_shows_framework_and_version
            && self.status_strip_shows_health_and_compatibility
            && self.status_strip_offers_inspect_and_review
            && self.support_experience_and_scope_derived_never_asserted
            && self.bridge_or_heuristic_never_shown_as_exact
            && self.version_drift_and_multiple_pack_always_disclosed
            && self.remote_scope_never_shown_as_local
            && self.freshness_and_health_always_explicit
            && self.every_next_step_names_stable_deep_link
            && self.execution_boundary_always_visible
            && self.no_surface_invents_alternate_state_label
            && self.components_stable_across_deployment_lines
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkPackConsumerProjection {
    /// The framework-pack surface reads a single canonical source.
    pub framework_pack_surface_reads_single_source: bool,
    /// The route and topology surfaces read a single canonical source.
    pub route_and_topology_surfaces_read_single_source: bool,
    /// The diagnostics surface reads a single canonical source.
    pub diagnostics_surface_reads_single_source: bool,
    /// Pack identity and support are visible before a user trusts the lens.
    pub identity_and_support_visible_before_trust: bool,
    /// Scope and experience are visible before a user trusts the lens.
    pub scope_and_experience_visible_before_trust: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

impl FrameworkPackConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.framework_pack_surface_reads_single_source
            && self.route_and_topology_surfaces_read_single_source
            && self.diagnostics_surface_reads_single_source
            && self.identity_and_support_visible_before_trust
            && self.scope_and_experience_visible_before_trust
            && self.support_export_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkPackProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`FrameworkPackHeaderStatusStripControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameworkPackHeaderStatusStripControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Framework pack headers.
    pub pack_headers: Vec<FrameworkPackHeader>,
    /// Framework status strips.
    pub status_strips: Vec<FrameworkStatusStrip>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Framework-pack review block.
    pub framework_review: FrameworkPackReview,
    /// Consumer projection block.
    pub consumer_projection: FrameworkPackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: FrameworkPackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe framework-pack-header / status-strip controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkPackHeaderStatusStripControlsPacket {
    /// Record kind; must equal [`FRAMEWORK_PACK_HEADER_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`FRAMEWORK_PACK_HEADER_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Framework pack headers.
    pub pack_headers: Vec<FrameworkPackHeader>,
    /// Framework status strips.
    pub status_strips: Vec<FrameworkStatusStrip>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Framework-pack review block.
    pub framework_review: FrameworkPackReview,
    /// Consumer projection block.
    pub consumer_projection: FrameworkPackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: FrameworkPackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl FrameworkPackHeaderStatusStripControlsPacket {
    /// Builds a framework-pack-header / status-strip controls packet from stable-lane input.
    pub fn new(input: FrameworkPackHeaderStatusStripControlsPacketInput) -> Self {
        Self {
            record_kind: FRAMEWORK_PACK_HEADER_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: FRAMEWORK_PACK_HEADER_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            pack_headers: input.pack_headers,
            status_strips: input.status_strips,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            framework_review: input.framework_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the framework-pack-header / status-strip control invariants.
    pub fn validate(&self) -> Vec<FrameworkPackHeaderControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != FRAMEWORK_PACK_HEADER_CONTROLS_RECORD_KIND {
            violations.push(FrameworkPackHeaderControlsViolation::WrongRecordKind);
        }
        if self.schema_version != FRAMEWORK_PACK_HEADER_CONTROLS_SCHEMA_VERSION {
            violations.push(FrameworkPackHeaderControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(FrameworkPackHeaderControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(FrameworkPackHeaderControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(FrameworkPackHeaderControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_pack_headers(self, &mut violations);
        validate_status_strips(self, &mut violations);

        if !self.framework_review.all_hold() {
            violations.push(FrameworkPackHeaderControlsViolation::FrameworkReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(FrameworkPackHeaderControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(FrameworkPackHeaderControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("framework pack header controls packet serializes"),
        ) {
            violations.push(FrameworkPackHeaderControlsViolation::RawMaterialInExport);
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
            .expect("framework pack header controls packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component,id,support_class,identity_state,experience,scope,exact_support,deep_link_kind\n",
        );
        for header in &self.pack_headers {
            let disclosure = header.posture_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                "framework_pack_header",
                csv_field(&header.pack_id),
                header.support_class.as_str(),
                header.identity_state.as_str(),
                disclosure.experience_class.as_str(),
                disclosure.scope_posture.as_str(),
                disclosure.is_exact_first_party_support,
                header.deep_link_kind.as_str(),
            ));
        }
        for strip in &self.status_strips {
            let disclosure = strip.posture_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                "framework_status_strip",
                csv_field(&strip.pack_id),
                strip.support_class.as_str(),
                strip.identity_state.as_str(),
                disclosure.experience_class.as_str(),
                disclosure.scope_posture.as_str(),
                disclosure.is_exact_first_party_support,
                strip.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let bridge_or_heuristic = self
            .pack_headers
            .iter()
            .filter(|header| {
                header
                    .posture_disclosure()
                    .is_bridge_or_heuristic_experience
            })
            .count();
        let remote = self
            .status_strips
            .iter()
            .filter(|strip| !strip.posture_disclosure().is_local_scope)
            .count();

        let mut out = String::new();
        out.push_str("# Framework pack headers and framework status strips\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Framework pack headers: {} ({} bridged or heuristic)\n",
            self.pack_headers.len(),
            bridge_or_heuristic
        ));
        out.push_str(&format!(
            "- Framework status strips: {} ({} not local scope)\n",
            self.status_strips.len(),
            remote
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Framework pack headers\n\n");
        for header in &self.pack_headers {
            let disclosure = header.posture_disclosure();
            out.push_str(&format!(
                "- **{}** — support `{}` → `{}`, experience `{}`, scope `{}`, freshness `{}`, deep link `{}`\n",
                header.pack_name,
                header.support_class.as_str(),
                disclosure.support_posture.as_str(),
                disclosure.experience_class.as_str(),
                disclosure.scope_posture.as_str(),
                header.freshness_state.as_str(),
                header.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Framework status strips\n\n");
        for strip in &self.status_strips {
            let disclosure = strip.posture_disclosure();
            out.push_str(&format!(
                "- **{}** — support `{}`, identity `{}`, experience `{}`, scope `{}`, health `{}`, deep link `{}`\n",
                strip.detected_framework_label,
                strip.support_class.as_str(),
                strip.identity_state.as_str(),
                disclosure.experience_class.as_str(),
                disclosure.scope_posture.as_str(),
                strip.pack_health_class.as_str(),
                strip.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in framework-pack-header controls export.
#[derive(Debug)]
pub enum FrameworkPackHeaderControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<FrameworkPackHeaderControlsViolation>),
}

impl fmt::Display for FrameworkPackHeaderControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "framework pack header controls export parse failed: {error}"
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
                    "framework pack header controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for FrameworkPackHeaderControlsArtifactError {}

/// Validation failures emitted by [`FrameworkPackHeaderStatusStripControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameworkPackHeaderControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No framework pack headers are present.
    PackHeadersMissing,
    /// A framework pack header is incomplete.
    PackHeaderIncomplete,
    /// A framework pack header carries the wrong frozen component class.
    PackHeaderWrongComponentClass,
    /// A pack header misrepresents its derived support posture, experience class, or scope.
    PackPostureMisrepresented,
    /// A bridged or heuristic component claims exact first-party support.
    HeuristicClaimsExactSupport,
    /// A not-exactly-first-party-support component does not name its non-exact support.
    NonexactSupportNoteMissing,
    /// A bridged or heuristic component does not name its bridge / heuristic posture.
    BridgeOrHeuristicNoteMissing,
    /// A remote / managed / unknown-scope component does not name its scope.
    RemoteScopeNoteMissing,
    /// A version-drifted component does not name its drift.
    VersionDriftNoteMissing,
    /// A multiple-detected component does not name its multiple-pack ambiguity.
    MultiplePackNoteMissing,
    /// An unversioned component does not name its unversioned pack.
    UnversionedNoteMissing,
    /// An unknown-pack component does not name its unknown pack.
    UnknownPackNoteMissing,
    /// A component does not name its pack source / certainty.
    PackSourceAndCertaintyNoteMissing,
    /// A pack header does not name its provider source.
    ProviderSourceMissing,
    /// A pack header does not name its workspace scope.
    WorkspaceScopeMissing,
    /// A pack header does not name its freshness.
    FreshnessLabelMissing,
    /// A pack header does not name its compatibility-details reference.
    CompatibilityDetailsRefMissing,
    /// A pack header omits a mandatory action.
    PackHeaderActionsIncomplete,
    /// No framework status strips are present.
    StatusStripsMissing,
    /// A framework status strip is incomplete.
    StatusStripIncomplete,
    /// A framework status strip carries the wrong frozen component class.
    StatusStripWrongComponentClass,
    /// A status strip does not name its pack health.
    HealthLabelMissing,
    /// A status strip does not name its compatibility notes.
    CompatibilityNotesMissing,
    /// A non-healthy status strip does not name its degraded health.
    DegradedHealthNoteMissing,
    /// A status strip omits a mandatory action.
    StatusStripActionsIncomplete,
    /// The components do not cover every pack support class.
    PackSupportClassCoverageMissing,
    /// The components do not cover every pack identity state.
    PackIdentityStateCoverageMissing,
    /// The components do not cover every execution boundary class.
    ExecutionBoundaryCoverageMissing,
    /// The components do not cover every derived support posture.
    SupportPostureCoverageMissing,
    /// The components do not cover every derived experience class.
    ExperienceClassCoverageMissing,
    /// The components do not cover every derived scope posture.
    ScopePostureCoverageMissing,
    /// The pack headers do not cover every freshness state.
    FreshnessStateCoverageMissing,
    /// The status strips do not cover every pack health class.
    HealthClassCoverageMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component offers a deep-link action but its deep link does not resolve exactly.
    DeepLinkUnresolved,
    /// A component names a deep-link kind but not its stable reference.
    DeepLinkRefMissing,
    /// A component does not bind any certainty disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component hides its pack identity, version, or support class.
    PackIdentityOrSupportHidden,
    /// A component lets bridge or heuristic behavior masquerade as exact.
    HeuristicMasqueradesAsExact,
    /// A component hides the local / container / SSH / managed boundary.
    ExecutionBoundaryHidden,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Framework review does not satisfy required invariants.
    FrameworkReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl FrameworkPackHeaderControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::PackHeadersMissing => "pack_headers_missing",
            Self::PackHeaderIncomplete => "pack_header_incomplete",
            Self::PackHeaderWrongComponentClass => "pack_header_wrong_component_class",
            Self::PackPostureMisrepresented => "pack_posture_misrepresented",
            Self::HeuristicClaimsExactSupport => "heuristic_claims_exact_support",
            Self::NonexactSupportNoteMissing => "nonexact_support_note_missing",
            Self::BridgeOrHeuristicNoteMissing => "bridge_or_heuristic_note_missing",
            Self::RemoteScopeNoteMissing => "remote_scope_note_missing",
            Self::VersionDriftNoteMissing => "version_drift_note_missing",
            Self::MultiplePackNoteMissing => "multiple_pack_note_missing",
            Self::UnversionedNoteMissing => "unversioned_note_missing",
            Self::UnknownPackNoteMissing => "unknown_pack_note_missing",
            Self::PackSourceAndCertaintyNoteMissing => "pack_source_and_certainty_note_missing",
            Self::ProviderSourceMissing => "provider_source_missing",
            Self::WorkspaceScopeMissing => "workspace_scope_missing",
            Self::FreshnessLabelMissing => "freshness_label_missing",
            Self::CompatibilityDetailsRefMissing => "compatibility_details_ref_missing",
            Self::PackHeaderActionsIncomplete => "pack_header_actions_incomplete",
            Self::StatusStripsMissing => "status_strips_missing",
            Self::StatusStripIncomplete => "status_strip_incomplete",
            Self::StatusStripWrongComponentClass => "status_strip_wrong_component_class",
            Self::HealthLabelMissing => "health_label_missing",
            Self::CompatibilityNotesMissing => "compatibility_notes_missing",
            Self::DegradedHealthNoteMissing => "degraded_health_note_missing",
            Self::StatusStripActionsIncomplete => "status_strip_actions_incomplete",
            Self::PackSupportClassCoverageMissing => "pack_support_class_coverage_missing",
            Self::PackIdentityStateCoverageMissing => "pack_identity_state_coverage_missing",
            Self::ExecutionBoundaryCoverageMissing => "execution_boundary_coverage_missing",
            Self::SupportPostureCoverageMissing => "support_posture_coverage_missing",
            Self::ExperienceClassCoverageMissing => "experience_class_coverage_missing",
            Self::ScopePostureCoverageMissing => "scope_posture_coverage_missing",
            Self::FreshnessStateCoverageMissing => "freshness_state_coverage_missing",
            Self::HealthClassCoverageMissing => "health_class_coverage_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::PackIdentityOrSupportHidden => "pack_identity_or_support_hidden",
            Self::HeuristicMasqueradesAsExact => "heuristic_masquerades_as_exact",
            Self::ExecutionBoundaryHidden => "execution_boundary_hidden",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::FrameworkReviewIncomplete => "framework_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable framework-pack-header controls export.
///
/// This is the first real consumer of the framework-pack-header component lane: a framework-pack,
/// route / topology, diagnostics, or support-export surface calls it to ingest the canonical
/// components rather than cloning status text.
///
/// # Errors
///
/// Returns [`FrameworkPackHeaderControlsArtifactError`] when the checked-in support export fails
/// to parse or fails validation.
pub fn current_framework_pack_header_controls_export(
) -> Result<FrameworkPackHeaderStatusStripControlsPacket, FrameworkPackHeaderControlsArtifactError>
{
    let packet: FrameworkPackHeaderStatusStripControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-framework-pack-header-status-strip-proof/support_export.json"
        )))
        .map_err(FrameworkPackHeaderControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(FrameworkPackHeaderControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &FrameworkPackHeaderStatusStripControlsPacket,
    violations: &mut Vec<FrameworkPackHeaderControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        FRAMEWORK_PACK_HEADER_CONTROLS_SCHEMA_REF,
        FRAMEWORK_PACK_HEADER_CONTROLS_DOC_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_DOC_REF,
        M5_FRAMEWORK_PACK_HEADER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(FrameworkPackHeaderControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

/// The four hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    hides_pack_identity_or_support_class: bool,
    lets_heuristic_masquerade_as_exact: bool,
    hides_local_container_ssh_or_managed_boundary: bool,
    invents_alternate_state_label: bool,
}

/// Validates the posture, notes, and cross-checks shared by both component vectors.
#[allow(clippy::too_many_arguments)]
fn validate_shared_posture(
    disclosure: &FrameworkPackDisclosure,
    derived_support_posture: PackSupportPosture,
    derived_experience_class: FrameworkExperienceClass,
    derived_scope_posture: FrameworkScopePosture,
    claims_exact_first_party_support: bool,
    claims_local_scope: bool,
    nonexact_support_note: &str,
    bridge_or_heuristic_note: &str,
    remote_scope_note: &str,
    version_drift_note: &str,
    multiple_pack_note: &str,
    unversioned_note: &str,
    unknown_pack_note: &str,
    pack_source_and_certainty_note: &str,
    violations: &mut Vec<FrameworkPackHeaderControlsViolation>,
) {
    if derived_support_posture != disclosure.support_posture
        || derived_experience_class != disclosure.experience_class
        || derived_scope_posture != disclosure.scope_posture
        || claims_exact_first_party_support != disclosure.is_exact_first_party_support
        || claims_local_scope != disclosure.is_local_scope
    {
        violations.push(FrameworkPackHeaderControlsViolation::PackPostureMisrepresented);
    }
    if disclosure.is_bridge_or_heuristic_experience && claims_exact_first_party_support {
        violations.push(FrameworkPackHeaderControlsViolation::HeuristicClaimsExactSupport);
    }
    if disclosure.needs_nonexact_support_note && nonexact_support_note.trim().is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::NonexactSupportNoteMissing);
    }
    if disclosure.needs_bridge_or_heuristic_note && bridge_or_heuristic_note.trim().is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::BridgeOrHeuristicNoteMissing);
    }
    if disclosure.needs_remote_scope_note && remote_scope_note.trim().is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::RemoteScopeNoteMissing);
    }
    if disclosure.needs_version_drift_note && version_drift_note.trim().is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::VersionDriftNoteMissing);
    }
    if disclosure.needs_multiple_pack_note && multiple_pack_note.trim().is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::MultiplePackNoteMissing);
    }
    if disclosure.needs_unversioned_note && unversioned_note.trim().is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::UnversionedNoteMissing);
    }
    if disclosure.needs_unknown_pack_note && unknown_pack_note.trim().is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::UnknownPackNoteMissing);
    }
    if pack_source_and_certainty_note.trim().is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::PackSourceAndCertaintyNoteMissing);
    }
}

fn validate_pack_headers(
    packet: &FrameworkPackHeaderStatusStripControlsPacket,
    violations: &mut Vec<FrameworkPackHeaderControlsViolation>,
) {
    if packet.pack_headers.is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::PackHeadersMissing);
        return;
    }

    for header in &packet.pack_headers {
        let disclosure = header.posture_disclosure();

        if header.pack_id.trim().is_empty()
            || header.pack_name.trim().is_empty()
            || header.framework_name.trim().is_empty()
            || header.framework_version_range.trim().is_empty()
            || header.fields_shown.is_empty()
            || header.surface_families.is_empty()
            || header.deployment_lines.is_empty()
            || header.consumer_surfaces.is_empty()
            || header.source_contract_refs.is_empty()
        {
            violations.push(FrameworkPackHeaderControlsViolation::PackHeaderIncomplete);
        }
        if header.component != M5FrameworkComponentFamily::FrameworkPackHeader {
            violations.push(FrameworkPackHeaderControlsViolation::PackHeaderWrongComponentClass);
        }
        validate_shared_posture(
            &disclosure,
            header.derived_support_posture,
            header.derived_experience_class,
            header.derived_scope_posture,
            header.claims_exact_first_party_support,
            header.claims_local_scope,
            &header.nonexact_support_note,
            &header.bridge_or_heuristic_note,
            &header.remote_scope_note,
            &header.version_drift_note,
            &header.multiple_pack_note,
            &header.unversioned_note,
            &header.unknown_pack_note,
            &header.pack_source_and_certainty_note,
            violations,
        );
        if header.provider_source_label.trim().is_empty() {
            violations.push(FrameworkPackHeaderControlsViolation::ProviderSourceMissing);
        }
        if header.workspace_scope_label.trim().is_empty() {
            violations.push(FrameworkPackHeaderControlsViolation::WorkspaceScopeMissing);
        }
        if header.freshness_label.trim().is_empty() {
            violations.push(FrameworkPackHeaderControlsViolation::FreshnessLabelMissing);
        }
        if header.compatibility_details_ref.trim().is_empty() {
            violations.push(FrameworkPackHeaderControlsViolation::CompatibilityDetailsRefMissing);
        }
        if !header.declares_mandatory_actions() {
            violations.push(FrameworkPackHeaderControlsViolation::PackHeaderActionsIncomplete);
        }
        validate_deep_link(
            header.offers_deep_link_action(),
            header.deep_link_kind,
            &header.deep_link_ref,
            &header.context_note,
            violations,
        );
        validate_common_control(
            &header.dispositions,
            &header.downgrade_triggers,
            header.declares_mandatory_labels(),
            &header.accessibility_routes,
            ControlInvariants {
                hides_pack_identity_or_support_class: header.hides_pack_identity_or_support_class,
                lets_heuristic_masquerade_as_exact: header.lets_heuristic_masquerade_as_exact,
                hides_local_container_ssh_or_managed_boundary: header
                    .hides_local_container_ssh_or_managed_boundary,
                invents_alternate_state_label: header.invents_alternate_state_label,
            },
            violations,
        );
    }

    validate_shared_coverage(
        packet.pack_headers.iter().map(|header| ComponentCoverage {
            support_class: header.support_class,
            identity_state: header.identity_state,
            execution_boundary: header.execution_boundary,
            disclosure: header.posture_disclosure(),
        }),
        packet.status_strips.iter().map(|strip| ComponentCoverage {
            support_class: strip.support_class,
            identity_state: strip.identity_state,
            execution_boundary: strip.execution_boundary,
            disclosure: strip.posture_disclosure(),
        }),
        violations,
    );

    let mut freshness: BTreeSet<PackFreshnessState> = BTreeSet::new();
    for header in &packet.pack_headers {
        freshness.insert(header.freshness_state);
    }
    for required in PackFreshnessState::ALL {
        if !freshness.contains(&required) {
            violations.push(FrameworkPackHeaderControlsViolation::FreshnessStateCoverageMissing);
            break;
        }
    }
}

fn validate_status_strips(
    packet: &FrameworkPackHeaderStatusStripControlsPacket,
    violations: &mut Vec<FrameworkPackHeaderControlsViolation>,
) {
    if packet.status_strips.is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::StatusStripsMissing);
        return;
    }

    let mut health: BTreeSet<PackHealthClass> = BTreeSet::new();

    for strip in &packet.status_strips {
        let disclosure = strip.posture_disclosure();
        health.insert(strip.pack_health_class);

        if strip.pack_id.trim().is_empty()
            || strip.detected_framework_label.trim().is_empty()
            || strip.detected_version_label.trim().is_empty()
            || strip.fields_shown.is_empty()
            || strip.surface_families.is_empty()
            || strip.deployment_lines.is_empty()
            || strip.consumer_surfaces.is_empty()
            || strip.source_contract_refs.is_empty()
        {
            violations.push(FrameworkPackHeaderControlsViolation::StatusStripIncomplete);
        }
        if strip.component != M5FrameworkComponentFamily::FrameworkPackHeader {
            violations.push(FrameworkPackHeaderControlsViolation::StatusStripWrongComponentClass);
        }
        validate_shared_posture(
            &disclosure,
            strip.derived_support_posture,
            strip.derived_experience_class,
            strip.derived_scope_posture,
            strip.claims_exact_first_party_support,
            strip.claims_local_scope,
            &strip.nonexact_support_note,
            &strip.bridge_or_heuristic_note,
            &strip.remote_scope_note,
            &strip.version_drift_note,
            // A compact strip has no dedicated multiple / unversioned / unknown-pack note field;
            // it discloses the pack identity state through its always-present pack source /
            // certainty note, so route those three checks through that note.
            &strip.pack_source_and_certainty_note,
            &strip.pack_source_and_certainty_note,
            &strip.pack_source_and_certainty_note,
            &strip.pack_source_and_certainty_note,
            violations,
        );
        if strip.pack_health_label.trim().is_empty() {
            violations.push(FrameworkPackHeaderControlsViolation::HealthLabelMissing);
        }
        if strip.compatibility_notes_label.trim().is_empty() {
            violations.push(FrameworkPackHeaderControlsViolation::CompatibilityNotesMissing);
        }
        if strip.pack_health_class.needs_note() && strip.degraded_health_note.trim().is_empty() {
            violations.push(FrameworkPackHeaderControlsViolation::DegradedHealthNoteMissing);
        }
        if !strip.declares_mandatory_actions() {
            violations.push(FrameworkPackHeaderControlsViolation::StatusStripActionsIncomplete);
        }
        validate_deep_link(
            strip.offers_deep_link_action(),
            strip.deep_link_kind,
            &strip.deep_link_ref,
            &strip.context_note,
            violations,
        );
        validate_common_control(
            &strip.dispositions,
            &strip.downgrade_triggers,
            strip.declares_mandatory_labels(),
            &strip.accessibility_routes,
            ControlInvariants {
                hides_pack_identity_or_support_class: strip.hides_pack_identity_or_support_class,
                lets_heuristic_masquerade_as_exact: strip.lets_heuristic_masquerade_as_exact,
                hides_local_container_ssh_or_managed_boundary: strip
                    .hides_local_container_ssh_or_managed_boundary,
                invents_alternate_state_label: strip.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in PackHealthClass::ALL {
        if !health.contains(&required) {
            violations.push(FrameworkPackHeaderControlsViolation::HealthClassCoverageMissing);
            break;
        }
    }
}

/// The coverage-relevant axes of one component.
struct ComponentCoverage {
    support_class: M5FrameworkPackSupportClass,
    identity_state: M5FrameworkPackIdentityState,
    execution_boundary: M5ExecutionBoundaryClass,
    disclosure: FrameworkPackDisclosure,
}

/// Validates that the union of both component vectors covers every frozen and derived vocabulary
/// the acceptance criteria pin.
fn validate_shared_coverage(
    headers: impl Iterator<Item = ComponentCoverage>,
    strips: impl Iterator<Item = ComponentCoverage>,
    violations: &mut Vec<FrameworkPackHeaderControlsViolation>,
) {
    let mut supports: BTreeSet<M5FrameworkPackSupportClass> = BTreeSet::new();
    let mut identities: BTreeSet<M5FrameworkPackIdentityState> = BTreeSet::new();
    let mut boundaries: BTreeSet<M5ExecutionBoundaryClass> = BTreeSet::new();
    let mut postures: BTreeSet<PackSupportPosture> = BTreeSet::new();
    let mut experiences: BTreeSet<FrameworkExperienceClass> = BTreeSet::new();
    let mut scopes: BTreeSet<FrameworkScopePosture> = BTreeSet::new();

    for component in headers.chain(strips) {
        supports.insert(component.support_class);
        identities.insert(component.identity_state);
        boundaries.insert(component.execution_boundary);
        postures.insert(component.disclosure.support_posture);
        experiences.insert(component.disclosure.experience_class);
        scopes.insert(component.disclosure.scope_posture);
    }

    for required in M5FrameworkPackSupportClass::ALL {
        if !supports.contains(&required) {
            violations.push(FrameworkPackHeaderControlsViolation::PackSupportClassCoverageMissing);
            break;
        }
    }
    for required in M5FrameworkPackIdentityState::ALL {
        if !identities.contains(&required) {
            violations.push(FrameworkPackHeaderControlsViolation::PackIdentityStateCoverageMissing);
            break;
        }
    }
    for required in M5ExecutionBoundaryClass::ALL {
        if !boundaries.contains(&required) {
            violations.push(FrameworkPackHeaderControlsViolation::ExecutionBoundaryCoverageMissing);
            break;
        }
    }
    for required in PackSupportPosture::ALL {
        if !postures.contains(&required) {
            violations.push(FrameworkPackHeaderControlsViolation::SupportPostureCoverageMissing);
            break;
        }
    }
    for required in FrameworkExperienceClass::ALL {
        if !experiences.contains(&required) {
            violations.push(FrameworkPackHeaderControlsViolation::ExperienceClassCoverageMissing);
            break;
        }
    }
    for required in FrameworkScopePosture::ALL {
        if !scopes.contains(&required) {
            violations.push(FrameworkPackHeaderControlsViolation::ScopePostureCoverageMissing);
            break;
        }
    }
}

/// Validates the context and stable deep-link truth shared by both component vectors.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    context_note: &str,
    violations: &mut Vec<FrameworkPackHeaderControlsViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(FrameworkPackHeaderControlsViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::DeepLinkRefMissing);
    }
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5FrameworkCertaintyDisposition],
    downgrade_triggers: &[M5FrameworkDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5FrameworkAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<FrameworkPackHeaderControlsViolation>,
) {
    if dispositions.is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(FrameworkPackHeaderControlsViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(FrameworkPackHeaderControlsViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(FrameworkPackHeaderControlsViolation::AccessibilityRouteMissing);
    }
    if invariants.hides_pack_identity_or_support_class {
        violations.push(FrameworkPackHeaderControlsViolation::PackIdentityOrSupportHidden);
    }
    if invariants.lets_heuristic_masquerade_as_exact {
        violations.push(FrameworkPackHeaderControlsViolation::HeuristicMasqueradesAsExact);
    }
    if invariants.hides_local_container_ssh_or_managed_boundary {
        violations.push(FrameworkPackHeaderControlsViolation::ExecutionBoundaryHidden);
    }
    if invariants.invents_alternate_state_label {
        violations.push(FrameworkPackHeaderControlsViolation::AlternateStateLabelInvented);
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Canonical seed builders
//
// These builders are the single producer of the checked-in support export and the scenario
// fixtures. The headless emitter example and the inline tests both call them so the in-code
// components, the artifact, and the fixtures never drift.
// ---------------------------------------------------------------------------

/// Stable packet id for the canonical framework-pack-header controls packet.
pub const FRAMEWORK_PACK_HEADER_CONTROLS_PACKET_ID: &str =
    "m5-framework-pack-header-status-strip-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn component_source_refs() -> Vec<String> {
    strings(&[
        M5_FRAMEWORK_PACK_HEADER_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
    ])
}

fn pack_header_downgrade_triggers() -> Vec<M5FrameworkDowngradeTrigger> {
    vec![
        M5FrameworkDowngradeTrigger::PackIdentityUnstated,
        M5FrameworkDowngradeTrigger::SupportClassUnstated,
        M5FrameworkDowngradeTrigger::ExecutionBoundaryUnstated,
        M5FrameworkDowngradeTrigger::AlternateStateLabelInvented,
        M5FrameworkDowngradeTrigger::ProofStale,
    ]
}

fn status_strip_downgrade_triggers() -> Vec<M5FrameworkDowngradeTrigger> {
    vec![
        M5FrameworkDowngradeTrigger::PackIdentityUnstated,
        M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated,
        M5FrameworkDowngradeTrigger::ExecutionBoundaryUnstated,
        M5FrameworkDowngradeTrigger::AlternateStateLabelInvented,
        M5FrameworkDowngradeTrigger::ProofStale,
    ]
}

/// The three mandatory labels plus one extra truth label.
fn label_set(extra: M5FrameworkRequiredLabel) -> Vec<M5FrameworkRequiredLabel> {
    let mut labels = M5FrameworkRequiredLabel::MANDATORY.to_vec();
    labels.push(extra);
    labels
}

/// Builds a framework pack header, deriving the support posture, experience class, scope posture,
/// exact / local claims, and the required notes from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn pack_header(
    pack_id: &str,
    pack_name: &str,
    framework_name: &str,
    framework_version_range: &str,
    support_class: M5FrameworkPackSupportClass,
    identity_state: M5FrameworkPackIdentityState,
    certainty: M5FrameworkCertaintyDisposition,
    execution_boundary: M5ExecutionBoundaryClass,
    provider_source_label: &str,
    workspace_scope_label: &str,
    freshness_state: PackFreshnessState,
    freshness_label: &str,
    compatibility_details_ref: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    header_actions: Vec<PackHeaderAction>,
) -> FrameworkPackHeader {
    let disclosure = resolve_framework_pack_posture(
        support_class,
        identity_state,
        certainty,
        execution_boundary,
    );
    FrameworkPackHeader {
        component: M5FrameworkComponentFamily::FrameworkPackHeader,
        pack_id: pack_id.to_owned(),
        pack_name: pack_name.to_owned(),
        framework_name: framework_name.to_owned(),
        framework_version_range: framework_version_range.to_owned(),
        support_class,
        identity_state,
        certainty,
        execution_boundary,
        derived_support_posture: disclosure.support_posture,
        derived_experience_class: disclosure.experience_class,
        derived_scope_posture: disclosure.scope_posture,
        claims_exact_first_party_support: disclosure.is_exact_first_party_support,
        claims_local_scope: disclosure.is_local_scope,
        provider_source_label: provider_source_label.to_owned(),
        workspace_scope_label: workspace_scope_label.to_owned(),
        freshness_state,
        freshness_label: freshness_label.to_owned(),
        nonexact_support_note: note_if(
            disclosure.needs_nonexact_support_note,
            &format!(
                "Support posture is {}; this is not exact first-party support",
                disclosure.support_posture.as_str()
            ),
        ),
        bridge_or_heuristic_note: note_if(
            disclosure.needs_bridge_or_heuristic_note,
            "Active experience is bridged or heuristic; treat it as convention, not exact truth",
        ),
        remote_scope_note: note_if(
            disclosure.needs_remote_scope_note,
            &format!(
                "Active scope is {}; it does not run locally on this machine",
                disclosure.scope_posture.as_str()
            ),
        ),
        version_drift_note: note_if(
            disclosure.needs_version_drift_note,
            "Pack version drifted from the pinned version; reconcile before trusting the lens",
        ),
        multiple_pack_note: note_if(
            disclosure.needs_multiple_pack_note,
            "Multiple candidate packs detected; the active pack is ambiguous until resolved",
        ),
        unversioned_note: note_if(
            disclosure.needs_unversioned_note,
            "Pack detected but unversioned; version-specific behavior is not guaranteed",
        ),
        unknown_pack_note: note_if(
            disclosure.needs_unknown_pack_note,
            "Pack could not be resolved; do not treat it as a governed framework pack",
        ),
        pack_source_and_certainty_note: format!(
            "Pack {}; certainty {}; experience {}",
            identity_state.as_str(),
            certainty.as_str(),
            disclosure.experience_class.as_str()
        ),
        compatibility_details_ref: compatibility_details_ref.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        header_actions,
        dispositions: vec![certainty],
        downgrade_triggers: pack_header_downgrade_triggers(),
        required_labels: label_set(M5FrameworkRequiredLabel::PackSourceAndCertainty),
        surface_families: M5FrameworkSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5FrameworkDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5FrameworkAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "pack_name",
            "framework_version_range",
            "support_class",
            "identity_state",
            "provider_source_label",
            "workspace_scope_label",
            "freshness_state",
            "deep_link_kind",
        ]),
        source_contract_refs: component_source_refs(),
        hides_pack_identity_or_support_class: false,
        lets_heuristic_masquerade_as_exact: false,
        hides_local_container_ssh_or_managed_boundary: false,
        invents_alternate_state_label: false,
    }
}

/// Builds a framework status strip, deriving the support posture, experience class, scope posture,
/// exact / local claims, and the required notes from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn status_strip(
    pack_id: &str,
    detected_framework_label: &str,
    detected_version_label: &str,
    support_class: M5FrameworkPackSupportClass,
    identity_state: M5FrameworkPackIdentityState,
    certainty: M5FrameworkCertaintyDisposition,
    execution_boundary: M5ExecutionBoundaryClass,
    pack_health_class: PackHealthClass,
    pack_health_label: &str,
    compatibility_notes_label: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    strip_actions: Vec<StatusStripAction>,
) -> FrameworkStatusStrip {
    let disclosure = resolve_framework_pack_posture(
        support_class,
        identity_state,
        certainty,
        execution_boundary,
    );
    FrameworkStatusStrip {
        component: M5FrameworkComponentFamily::FrameworkPackHeader,
        pack_id: pack_id.to_owned(),
        detected_framework_label: detected_framework_label.to_owned(),
        detected_version_label: detected_version_label.to_owned(),
        support_class,
        identity_state,
        certainty,
        execution_boundary,
        derived_support_posture: disclosure.support_posture,
        derived_experience_class: disclosure.experience_class,
        derived_scope_posture: disclosure.scope_posture,
        claims_exact_first_party_support: disclosure.is_exact_first_party_support,
        claims_local_scope: disclosure.is_local_scope,
        pack_health_class,
        pack_health_label: pack_health_label.to_owned(),
        compatibility_notes_label: compatibility_notes_label.to_owned(),
        nonexact_support_note: note_if(
            disclosure.needs_nonexact_support_note,
            &format!(
                "Support posture is {}; this is not exact first-party support",
                disclosure.support_posture.as_str()
            ),
        ),
        bridge_or_heuristic_note: note_if(
            disclosure.needs_bridge_or_heuristic_note,
            "Active experience is bridged or heuristic; treat it as convention, not exact truth",
        ),
        remote_scope_note: note_if(
            disclosure.needs_remote_scope_note,
            &format!(
                "Active scope is {}; it does not run locally on this machine",
                disclosure.scope_posture.as_str()
            ),
        ),
        version_drift_note: note_if(
            disclosure.needs_version_drift_note,
            "Pack version drifted from the pinned version; reconcile before trusting the lens",
        ),
        degraded_health_note: note_if(
            pack_health_class.needs_note(),
            &format!(
                "Pack health is {}; framework-aware output may be incomplete",
                pack_health_class.as_str()
            ),
        ),
        pack_source_and_certainty_note: format!(
            "Pack {}; certainty {}; experience {}",
            identity_state.as_str(),
            certainty.as_str(),
            disclosure.experience_class.as_str()
        ),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        strip_actions,
        dispositions: vec![certainty],
        downgrade_triggers: status_strip_downgrade_triggers(),
        required_labels: label_set(M5FrameworkRequiredLabel::PackSourceAndCertainty),
        surface_families: M5FrameworkSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5FrameworkDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5FrameworkAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "detected_framework_label",
            "detected_version_label",
            "support_class",
            "identity_state",
            "pack_health_class",
            "compatibility_notes_label",
            "deep_link_kind",
        ]),
        source_contract_refs: component_source_refs(),
        hides_pack_identity_or_support_class: false,
        lets_heuristic_masquerade_as_exact: false,
        hides_local_container_ssh_or_managed_boundary: false,
        invents_alternate_state_label: false,
    }
}

/// Returns `text` when `needed`, else an empty string.
fn note_if(needed: bool, text: &str) -> String {
    if needed {
        text.to_owned()
    } else {
        String::new()
    }
}

fn pack_headers() -> Vec<FrameworkPackHeader> {
    use DeepLinkKind as Link;
    use M5ExecutionBoundaryClass as Boundary;
    use M5FrameworkCertaintyDisposition as Certainty;
    use M5FrameworkPackIdentityState as Identity;
    use M5FrameworkPackSupportClass as Support;
    use PackFreshnessState as Fresh;
    use PackHeaderAction as Action;

    vec![
        // 1. Officially supported / identified-versioned / local / core-native → exact, local.
        pack_header(
            "pack-next-app",
            "Next.js app pack",
            "Next.js",
            "14.x (App Router)",
            Support::OfficiallySupported,
            Identity::IdentifiedVersioned,
            Certainty::CoreNative,
            Boundary::LocalProcess,
            "First-party Aureline pack",
            "This workspace (local)",
            Fresh::Current,
            "Scanned just now",
            "compat:packs/next-app",
            "Core-native Next.js pack; open compatibility details before trusting the lens",
            Link::PackManifest,
            "manifest:packs/next-app",
            vec![
                Action::OpenCompatibilityDetails,
                Action::InspectPackSourceAndSupport,
                Action::ReviewScopeBoundary,
                Action::SelectWorkspaceScope,
                Action::OpenDeepLink,
            ],
        ),
        // 2. Community supported / version-pinned / container / verified → pack-backed, container.
        pack_header(
            "pack-django",
            "Django pack",
            "Django",
            "5.0 (pinned)",
            Support::CommunitySupported,
            Identity::VersionPinned,
            Certainty::Verified,
            Boundary::Container,
            "Community registry",
            "Dev container",
            Fresh::Imported,
            "Imported from CI scan",
            "compat:packs/django",
            "Community pack running in a container; verify support before relying on it",
            Link::ProviderRegistryEntry,
            "registry:community/django",
            vec![
                Action::OpenCompatibilityDetails,
                Action::InspectPackSourceAndSupport,
                Action::ReviewScopeBoundary,
                Action::CopyPackId,
            ],
        ),
        // 3. Experimental / version-drifted / ssh-remote / heuristic → heuristic, remote.
        pack_header(
            "pack-svelte",
            "Svelte pack",
            "SvelteKit",
            "detected 2.x, pinned 1.x (drifted)",
            Support::Experimental,
            Identity::VersionDrifted,
            Certainty::HeuristicConvention,
            Boundary::SshRemote,
            "Community registry",
            "SSH remote host",
            Fresh::Stale,
            "Scan is stale",
            "compat:packs/svelte",
            "Experimental pack on an SSH remote with drifted version; heuristic, not exact",
            Link::CompatibilityReference,
            "compat:packs/svelte#drift",
            vec![
                Action::OpenCompatibilityDetails,
                Action::InspectPackSourceAndSupport,
                Action::ReviewScopeBoundary,
                Action::OpenDeepLink,
            ],
        ),
        // 4. Bridge-only / multiple-detected / managed / bridge → bridged, managed.
        pack_header(
            "pack-rails-bridge",
            "Rails bridge pack",
            "Rails",
            "7.x (bridge)",
            Support::BridgeOnly,
            Identity::MultipleDetected,
            Certainty::Bridge,
            Boundary::ManagedWorkspace,
            "Bridge adapter",
            "Managed workspace",
            Fresh::NeverScanned,
            "Never scanned",
            "compat:packs/rails-bridge",
            "Bridge pack in a managed workspace with multiple candidates; not exact first-party",
            Link::DocsAnchor,
            "docs:frameworks/bridge-packs",
            vec![
                Action::OpenCompatibilityDetails,
                Action::InspectPackSourceAndSupport,
                Action::ReviewScopeBoundary,
                Action::OpenDeepLink,
            ],
        ),
        // 5. Deprecated / unversioned / cloud-remote / derived-by-convention → heuristic, remote.
        pack_header(
            "pack-legacy-php",
            "Legacy PHP pack",
            "PHP",
            "unversioned",
            Support::Deprecated,
            Identity::Unversioned,
            Certainty::DerivedByConvention,
            Boundary::CloudRemote,
            "Mirror provider",
            "Cloud remote",
            Fresh::Unknown,
            "Freshness unknown",
            "compat:packs/legacy-php",
            "Deprecated, unversioned pack on a cloud remote; derived by convention only",
            Link::DocsAnchor,
            "docs:frameworks/deprecated-packs",
            vec![
                Action::OpenCompatibilityDetails,
                Action::InspectPackSourceAndSupport,
                Action::ReviewScopeBoundary,
                Action::OpenDeepLink,
            ],
        ),
        // 6. Unsupported / unknown-pack / unknown-boundary / partial → heuristic, unknown scope.
        pack_header(
            "pack-unknown",
            "Unidentified pack",
            "Unresolved framework",
            "unknown",
            Support::Unsupported,
            Identity::UnknownPack,
            Certainty::Partial,
            Boundary::UnknownBoundary,
            "Provider unresolved",
            "Scope unresolved",
            Fresh::Stale,
            "Scan is stale",
            "compat:packs/unknown",
            "Pack could not be resolved and its scope is unknown; do not trust the lens yet",
            Link::NoDeepLink,
            "",
            vec![
                Action::OpenCompatibilityDetails,
                Action::InspectPackSourceAndSupport,
                Action::ReviewScopeBoundary,
            ],
        ),
    ]
}

fn status_strips() -> Vec<FrameworkStatusStrip> {
    use DeepLinkKind as Link;
    use M5ExecutionBoundaryClass as Boundary;
    use M5FrameworkCertaintyDisposition as Certainty;
    use M5FrameworkPackIdentityState as Identity;
    use M5FrameworkPackSupportClass as Support;
    use PackHealthClass as Health;
    use StatusStripAction as Action;

    vec![
        // 1. Officially supported / identified / local / core-native / healthy.
        status_strip(
            "pack-next-app",
            "Next.js",
            "14.2.3",
            Support::OfficiallySupported,
            Identity::IdentifiedVersioned,
            Certainty::CoreNative,
            Boundary::LocalProcess,
            Health::Healthy,
            "Healthy",
            "No compatibility notes",
            "Core-native Next.js strip; framework and version detected and healthy",
            Link::PackManifest,
            "manifest:packs/next-app",
            vec![
                Action::InspectFrameworkAndVersion,
                Action::ReviewCompatibilityNotes,
                Action::OpenPackHeader,
                Action::OpenDeepLink,
            ],
        ),
        // 2. Community supported / version-pinned / container / verified / degraded.
        status_strip(
            "pack-django",
            "Django",
            "5.0.1",
            Support::CommunitySupported,
            Identity::VersionPinned,
            Certainty::FrameworkPack,
            Boundary::Container,
            Health::Degraded,
            "Degraded",
            "One dependency below the supported range",
            "Community Django strip in a container; pack health degraded",
            Link::ProviderRegistryEntry,
            "registry:community/django",
            vec![
                Action::InspectFrameworkAndVersion,
                Action::ReviewCompatibilityNotes,
                Action::OpenPackHeader,
                Action::OpenDeepLink,
            ],
        ),
        // 3. Experimental / version-drifted / ssh-remote / heuristic / compatibility warning.
        status_strip(
            "pack-svelte",
            "SvelteKit",
            "2.5.0 (pinned 1.x)",
            Support::Experimental,
            Identity::VersionDrifted,
            Certainty::HeuristicConvention,
            Boundary::SshRemote,
            Health::CompatibilityWarning,
            "Compatibility warning",
            "Detected version drifted from the pinned range",
            "Experimental SvelteKit strip on an SSH remote; compatibility warning active",
            Link::CompatibilityReference,
            "compat:packs/svelte#drift",
            vec![
                Action::InspectFrameworkAndVersion,
                Action::ReviewCompatibilityNotes,
                Action::OpenPackHeader,
                Action::OpenDeepLink,
            ],
        ),
        // 4. Bridge-only / multiple-detected / managed / bridge / broken.
        status_strip(
            "pack-rails-bridge",
            "Rails",
            "7.1.0 (bridge)",
            Support::BridgeOnly,
            Identity::MultipleDetected,
            Certainty::Bridge,
            Boundary::ManagedWorkspace,
            Health::Broken,
            "Broken",
            "Bridge adapter cannot resolve the active pack",
            "Rails bridge strip in a managed workspace; pack health broken",
            Link::DocsAnchor,
            "docs:frameworks/bridge-packs",
            vec![
                Action::InspectFrameworkAndVersion,
                Action::ReviewCompatibilityNotes,
                Action::OpenPackHeader,
                Action::OpenDeepLink,
            ],
        ),
        // 5. Deprecated / unversioned / cloud-remote / derived-by-convention / unknown health.
        status_strip(
            "pack-legacy-php",
            "PHP",
            "unversioned",
            Support::Deprecated,
            Identity::Unversioned,
            Certainty::DerivedByConvention,
            Boundary::CloudRemote,
            Health::Unknown,
            "Health unknown",
            "Compatibility cannot be determined without a version",
            "Deprecated PHP strip on a cloud remote; health unknown, derived by convention",
            Link::DocsAnchor,
            "docs:frameworks/deprecated-packs",
            vec![
                Action::InspectFrameworkAndVersion,
                Action::ReviewCompatibilityNotes,
                Action::OpenPackHeader,
                Action::OpenDeepLink,
            ],
        ),
        // 6. Unsupported / unknown-pack / unknown-boundary / partial / degraded.
        status_strip(
            "pack-unknown",
            "Unresolved framework",
            "unknown",
            Support::Unsupported,
            Identity::UnknownPack,
            Certainty::Partial,
            Boundary::UnknownBoundary,
            Health::Degraded,
            "Degraded",
            "Framework could not be identified from the workspace",
            "Unidentified strip with unknown scope; treat every framework claim as partial",
            Link::CompatibilityReference,
            "compat:packs/unknown",
            vec![
                Action::InspectFrameworkAndVersion,
                Action::ReviewCompatibilityNotes,
                Action::OpenPackHeader,
                Action::OpenDeepLink,
            ],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5FrameworkDowngradeTrigger> {
    vec![
        M5FrameworkDowngradeTrigger::PackIdentityUnstated,
        M5FrameworkDowngradeTrigger::SupportClassUnstated,
        M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated,
        M5FrameworkDowngradeTrigger::ExecutionBoundaryUnstated,
        M5FrameworkDowngradeTrigger::AlternateStateLabelInvented,
        M5FrameworkDowngradeTrigger::ProofStale,
    ]
}

fn framework_review() -> FrameworkPackReview {
    FrameworkPackReview {
        pack_header_shows_identity_and_support: true,
        pack_header_shows_provider_and_scope: true,
        pack_header_offers_compatibility_details: true,
        status_strip_shows_framework_and_version: true,
        status_strip_shows_health_and_compatibility: true,
        status_strip_offers_inspect_and_review: true,
        support_experience_and_scope_derived_never_asserted: true,
        bridge_or_heuristic_never_shown_as_exact: true,
        version_drift_and_multiple_pack_always_disclosed: true,
        remote_scope_never_shown_as_local: true,
        freshness_and_health_always_explicit: true,
        every_next_step_names_stable_deep_link: true,
        execution_boundary_always_visible: true,
        no_surface_invents_alternate_state_label: true,
        components_stable_across_deployment_lines: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> FrameworkPackConsumerProjection {
    FrameworkPackConsumerProjection {
        framework_pack_surface_reads_single_source: true,
        route_and_topology_surfaces_read_single_source: true,
        diagnostics_surface_reads_single_source: true,
        identity_and_support_visible_before_trust: true,
        scope_and_experience_visible_before_trust: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> FrameworkPackProofFreshness {
    FrameworkPackProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        FRAMEWORK_PACK_HEADER_CONTROLS_SCHEMA_REF,
        FRAMEWORK_PACK_HEADER_CONTROLS_DOC_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_DOC_REF,
        M5_FRAMEWORK_PACK_HEADER_SCHEMA_REF,
    ])
}

/// Builds the canonical framework-pack-header / status-strip controls packet.
pub fn seeded_framework_pack_header_controls() -> FrameworkPackHeaderStatusStripControlsPacket {
    FrameworkPackHeaderStatusStripControlsPacket::new(
        FrameworkPackHeaderStatusStripControlsPacketInput {
            packet_id: FRAMEWORK_PACK_HEADER_CONTROLS_PACKET_ID.to_owned(),
            surface_label:
                "M5 framework pack headers and framework status strips: pack identity, version range, support class, provider source, workspace scope, freshness, health, and local-versus-remote scope truth across claimed framework surfaces"
                    .to_owned(),
            pack_headers: pack_headers(),
            status_strips: status_strips(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
            framework_review: framework_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a bridged pack header on a remote / managed scope that must never
/// read as exact first-party support or as a local scope. Every support class, identity state,
/// execution boundary, experience class, and scope posture stays covered so the fixture validates
/// on its own.
pub fn seeded_framework_pack_header_controls_bridged_remote(
) -> FrameworkPackHeaderStatusStripControlsPacket {
    let mut packet = seeded_framework_pack_header_controls();
    packet.packet_id =
        "m5-framework-pack-header-status-strip-controls:fixture:pack-header-bridged-remote"
            .to_owned();
    packet.surface_label =
        "M5 framework pack headers: a bridged, remote pack never reads as exact first-party or local"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a version-drifted status strip with degraded pack health that must
/// never read as a healthy, exact pack. Every support class, identity state, execution boundary,
/// and pack health class stays covered so the fixture validates on its own.
pub fn seeded_framework_pack_header_controls_status_strip_drifted(
) -> FrameworkPackHeaderStatusStripControlsPacket {
    let mut packet = seeded_framework_pack_header_controls();
    packet.packet_id =
        "m5-framework-pack-header-status-strip-controls:fixture:status-strip-drifted".to_owned();
    packet.surface_label =
        "M5 framework status strips: a version-drifted strip never reads as a healthy, exact pack"
            .to_owned();
    packet
}
