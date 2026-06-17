//! Canonical M5 package-state, manifest-scope, registry-auth, and
//! lockfile-authority matrix that the whole package-mutation lane references.
//!
//! Where [`crate::package_mutation_and_registry_review`] reviews one operation
//! and [`crate::package_set_inventory_and_scope_truth`] inventories a package
//! set, this module freezes the **cross-ecosystem vocabulary** the package lane
//! must agree on before any mutation widens: the canonical package-state labels
//! (direct, transitive, workspace-local, path/VCS source, resolved-exact,
//! policy-pinned, advisory-open, suppressed-until, license-review-required,
//! offline-snapshot-only, auth-required, and unknown/stale), the manifest-scope,
//! registry-source, auth-mode, lockfile-authority, resolver-identity, and
//! rollback-class control objects, and the privacy/retention rules for operation
//! history, registry credentials, and support/export packets.
//!
//! The packet is a release-control freeze, not a label store. Three derived
//! invariants are recomputed and validated so the matrix cannot drift by hand:
//!
//! 1. **Requested-versus-resolved stays separate.** Every package-state label
//!    carries an [`IdentitySide`], and a label may describe a requested
//!    constraint or a resolved identity but never both, so requested-versus
//!    resolved dependency truth can never collapse into a single field.
//! 2. **No state collapses into a generic message.** Every state row and every
//!    registry-source cell maps to a *specific* [`PackageStateMessageClass`];
//!    the offline-snapshot, cache-only, mirror-backed, auth-required, and
//!    unknown/stale states are guarded so they can never render as the forbidden
//!    `generic_package_not_found` or `generic_install_failed` messages that the
//!    closed vocabulary names only to forbid.
//! 3. **Every claimed surface references one matrix.** Each marketed M5 package
//!    surface — the desktop workspace, CLI/headless, AI context, review
//!    workspace, support export, and release/public-truth — binds to this
//!    packet's id and pins the write authority it may carry, so product, CLI,
//!    and support/export paths express identity, lockfile authority, and
//!    registry/auth posture mechanically instead of by hand.
//!
//! Because [`PackageStateRow::identity_side`],
//! [`PackageStateRow::message_class`], [`RegistrySourceCell::message_class`],
//! [`SurfaceBinding::write_authority`], and the retention bindings are all
//! validated against the recomputed contract, docs/help, support export, and
//! release/public-truth surfaces can prove the package lane shares one governed
//! matrix rather than copying ecosystem-specific folklore.
//!
//! The packet is checked in at
//! `artifacts/deps/m5/freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.json`
//! and embedded here, so this typed consumer and any CI gate agree on every
//! label without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.
//! It carries no credential bodies, raw provider payloads, registry tokens, or
//! private registry URLs.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 package-state matrix schema version.
pub const M5_PACKAGE_STATE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_PACKAGE_STATE_MATRIX_RECORD_KIND: &str = "m5_package_state_mutation_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_PACKAGE_STATE_MATRIX_PATH: &str =
    "artifacts/deps/m5/freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.json";

/// Embedded checked-in packet JSON.
pub const M5_PACKAGE_STATE_MATRIX_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.json"
));

/// A canonical package-state label frozen by the matrix.
///
/// These twelve labels are the shared package-state vocabulary every M5 package
/// surface renders. Each label keeps requested-versus-resolved truth separate
/// through its [`IdentitySide`] and renders a specific
/// [`PackageStateMessageClass`] that never collapses into a generic failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageStateLabel {
    /// Direct dependency of the target manifest.
    Direct,
    /// Transitive dependency resolved through another package.
    Transitive,
    /// Workspace-local member dependency.
    WorkspaceLocal,
    /// Filesystem-path or version-control package source.
    PathOrVcsSource,
    /// Exactly resolved version, commit, path, or snapshot id.
    ResolvedExact,
    /// Policy-pinned version or source constraint.
    PolicyPinned,
    /// An advisory is open against the package.
    AdvisoryOpen,
    /// An advisory is suppressed until a stated expiry or condition.
    SuppressedUntil,
    /// License review is required before the package may ship.
    LicenseReviewRequired,
    /// Only an offline snapshot or local cache is available for the package.
    OfflineSnapshotOnly,
    /// Registry access requires authentication that is not satisfied.
    AuthRequired,
    /// The package state could not be established or is stale.
    UnknownOrStale,
}

impl PackageStateLabel {
    /// Every package-state label, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Direct,
        Self::Transitive,
        Self::WorkspaceLocal,
        Self::PathOrVcsSource,
        Self::ResolvedExact,
        Self::PolicyPinned,
        Self::AdvisoryOpen,
        Self::SuppressedUntil,
        Self::LicenseReviewRequired,
        Self::OfflineSnapshotOnly,
        Self::AuthRequired,
        Self::UnknownOrStale,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Transitive => "transitive",
            Self::WorkspaceLocal => "workspace_local",
            Self::PathOrVcsSource => "path_or_vcs_source",
            Self::ResolvedExact => "resolved_exact",
            Self::PolicyPinned => "policy_pinned",
            Self::AdvisoryOpen => "advisory_open",
            Self::SuppressedUntil => "suppressed_until",
            Self::LicenseReviewRequired => "license_review_required",
            Self::OfflineSnapshotOnly => "offline_snapshot_only",
            Self::AuthRequired => "auth_required",
            Self::UnknownOrStale => "unknown_or_stale",
        }
    }

    /// The identity side this label describes.
    ///
    /// This is the requested-versus-resolved separation made mechanical: a label
    /// is a requested constraint, a resolved identity, a finding overlay, a
    /// resolution-environment posture, or an indeterminate state — never more
    /// than one.
    pub const fn identity_side(self) -> IdentitySide {
        match self {
            Self::Direct
            | Self::Transitive
            | Self::WorkspaceLocal
            | Self::PathOrVcsSource
            | Self::ResolvedExact => IdentitySide::ResolvedIdentity,
            Self::PolicyPinned => IdentitySide::RequestedConstraint,
            Self::AdvisoryOpen | Self::SuppressedUntil | Self::LicenseReviewRequired => {
                IdentitySide::FindingOverlay
            }
            Self::OfflineSnapshotOnly | Self::AuthRequired => IdentitySide::ResolutionEnvironment,
            Self::UnknownOrStale => IdentitySide::IndeterminateState,
        }
    }

    /// The specific message class this label must render to.
    pub const fn canonical_message_class(self) -> PackageStateMessageClass {
        match self {
            Self::Direct => PackageStateMessageClass::DirectDependency,
            Self::Transitive => PackageStateMessageClass::TransitiveDependency,
            Self::WorkspaceLocal => PackageStateMessageClass::WorkspaceLocalSource,
            Self::PathOrVcsSource => PackageStateMessageClass::PathOrVcsSource,
            Self::ResolvedExact => PackageStateMessageClass::ResolvedExactPin,
            Self::PolicyPinned => PackageStateMessageClass::PolicyPinnedConstraint,
            Self::AdvisoryOpen => PackageStateMessageClass::AdvisoryOpenFinding,
            Self::SuppressedUntil => PackageStateMessageClass::SuppressedUntilFinding,
            Self::LicenseReviewRequired => PackageStateMessageClass::LicenseReviewRequiredFinding,
            Self::OfflineSnapshotOnly => PackageStateMessageClass::OfflineSnapshotDisclosure,
            Self::AuthRequired => PackageStateMessageClass::AuthRequiredDisclosure,
            Self::UnknownOrStale => PackageStateMessageClass::UnknownOrStaleDisclosure,
        }
    }

    /// Whether this label is one the matrix guards against collapsing into a
    /// generic message.
    ///
    /// The offline-snapshot/cache-only, auth-required, and unknown/stale states
    /// are exactly the states that historically read as "package not found" or
    /// "install failed"; they must always render their specific disclosure.
    pub const fn is_non_collapse_guarded(self) -> bool {
        matches!(
            self,
            Self::OfflineSnapshotOnly | Self::AuthRequired | Self::UnknownOrStale
        )
    }

    /// Whether this label describes the requested identity.
    pub const fn describes_requested(self) -> bool {
        self.identity_side().describes_requested()
    }

    /// Whether this label describes the resolved identity.
    pub const fn describes_resolved(self) -> bool {
        self.identity_side().describes_resolved()
    }
}

/// Which side of the requested-versus-resolved boundary a label describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentitySide {
    /// A constraint the user or automation requested before resolution.
    RequestedConstraint,
    /// A fact about the package the resolver produced.
    ResolvedIdentity,
    /// An advisory, suppression, or license overlay on a resolved package.
    FindingOverlay,
    /// A registry, mirror, cache, or auth posture of the resolution environment.
    ResolutionEnvironment,
    /// The package state could not be established.
    IndeterminateState,
}

impl IdentitySide {
    /// Every identity side, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RequestedConstraint,
        Self::ResolvedIdentity,
        Self::FindingOverlay,
        Self::ResolutionEnvironment,
        Self::IndeterminateState,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestedConstraint => "requested_constraint",
            Self::ResolvedIdentity => "resolved_identity",
            Self::FindingOverlay => "finding_overlay",
            Self::ResolutionEnvironment => "resolution_environment",
            Self::IndeterminateState => "indeterminate_state",
        }
    }

    /// Whether this side describes the requested identity.
    pub const fn describes_requested(self) -> bool {
        matches!(self, Self::RequestedConstraint)
    }

    /// Whether this side describes the resolved identity.
    pub const fn describes_resolved(self) -> bool {
        matches!(self, Self::ResolvedIdentity)
    }
}

/// A specific message class a package-state label or registry source renders to.
///
/// The two `Generic*` variants are named only so the matrix can forbid them:
/// no state row or registry cell may carry a generic class, which is how the
/// packet proves offline/mirror/cache-only and auth-required states never
/// collapse into "package not found" or "install failed".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageStateMessageClass {
    /// "Direct dependency" message.
    DirectDependency,
    /// "Transitive dependency" message.
    TransitiveDependency,
    /// "Workspace-local source" message.
    WorkspaceLocalSource,
    /// "Path/VCS source" message.
    PathOrVcsSource,
    /// "Resolved to exact pin" message.
    ResolvedExactPin,
    /// "Policy-pinned constraint" message.
    PolicyPinnedConstraint,
    /// "Advisory open" finding message.
    AdvisoryOpenFinding,
    /// "Suppressed until" finding message.
    SuppressedUntilFinding,
    /// "License review required" finding message.
    LicenseReviewRequiredFinding,
    /// "Offline snapshot only" disclosure message.
    OfflineSnapshotDisclosure,
    /// "Auth required" disclosure message.
    AuthRequiredDisclosure,
    /// "Unknown or stale" disclosure message.
    UnknownOrStaleDisclosure,
    /// "From the public registry" source message.
    PublicRegistrySource,
    /// "From a private registry" source message.
    PrivateRegistrySource,
    /// "From an enterprise mirror" source message.
    MirrorBackedSource,
    /// "From the local cache only" source message.
    CacheOnlySource,
    /// Forbidden generic "package not found" message.
    GenericPackageNotFound,
    /// Forbidden generic "install failed" message.
    GenericInstallFailed,
}

impl PackageStateMessageClass {
    /// Every message class, in declaration order.
    pub const ALL: [Self; 18] = [
        Self::DirectDependency,
        Self::TransitiveDependency,
        Self::WorkspaceLocalSource,
        Self::PathOrVcsSource,
        Self::ResolvedExactPin,
        Self::PolicyPinnedConstraint,
        Self::AdvisoryOpenFinding,
        Self::SuppressedUntilFinding,
        Self::LicenseReviewRequiredFinding,
        Self::OfflineSnapshotDisclosure,
        Self::AuthRequiredDisclosure,
        Self::UnknownOrStaleDisclosure,
        Self::PublicRegistrySource,
        Self::PrivateRegistrySource,
        Self::MirrorBackedSource,
        Self::CacheOnlySource,
        Self::GenericPackageNotFound,
        Self::GenericInstallFailed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectDependency => "direct_dependency",
            Self::TransitiveDependency => "transitive_dependency",
            Self::WorkspaceLocalSource => "workspace_local_source",
            Self::PathOrVcsSource => "path_or_vcs_source",
            Self::ResolvedExactPin => "resolved_exact_pin",
            Self::PolicyPinnedConstraint => "policy_pinned_constraint",
            Self::AdvisoryOpenFinding => "advisory_open_finding",
            Self::SuppressedUntilFinding => "suppressed_until_finding",
            Self::LicenseReviewRequiredFinding => "license_review_required_finding",
            Self::OfflineSnapshotDisclosure => "offline_snapshot_disclosure",
            Self::AuthRequiredDisclosure => "auth_required_disclosure",
            Self::UnknownOrStaleDisclosure => "unknown_or_stale_disclosure",
            Self::PublicRegistrySource => "public_registry_source",
            Self::PrivateRegistrySource => "private_registry_source",
            Self::MirrorBackedSource => "mirror_backed_source",
            Self::CacheOnlySource => "cache_only_source",
            Self::GenericPackageNotFound => "generic_package_not_found",
            Self::GenericInstallFailed => "generic_install_failed",
        }
    }

    /// Whether this class is one of the forbidden generic collapse messages.
    pub const fn is_generic_collapse(self) -> bool {
        matches!(
            self,
            Self::GenericPackageNotFound | Self::GenericInstallFailed
        )
    }

    /// Whether this class is a specific, non-collapsing message.
    pub const fn is_specific(self) -> bool {
        !self.is_generic_collapse()
    }
}

/// Manifest scope an operation can select.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestScopeClass {
    /// Every manifest in the workspace.
    WholeWorkspace,
    /// A single selected manifest.
    SelectedManifest,
    /// A named slice or workset of manifests.
    WorksetSlice,
    /// A single workspace member.
    WorkspaceMember,
    /// A path or version-control target outside the registry.
    PathOrVcsTarget,
}

impl ManifestScopeClass {
    /// Every manifest scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::WholeWorkspace,
        Self::SelectedManifest,
        Self::WorksetSlice,
        Self::WorkspaceMember,
        Self::PathOrVcsTarget,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WholeWorkspace => "whole_workspace",
            Self::SelectedManifest => "selected_manifest",
            Self::WorksetSlice => "workset_slice",
            Self::WorkspaceMember => "workspace_member",
            Self::PathOrVcsTarget => "path_or_vcs_target",
        }
    }

    /// Whether this scope must be confirmed explicitly before a bulk mutation.
    ///
    /// A whole-workspace mutation can never be applied ambiently; it requires an
    /// explicit, scoped confirmation.
    pub const fn requires_explicit_confirmation(self) -> bool {
        matches!(self, Self::WholeWorkspace)
    }
}

/// Registry or mirror source authority for a package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistrySourceAuthority {
    /// Public upstream registry.
    PublicRegistry,
    /// Private registry.
    PrivateRegistry,
    /// Enterprise mirror of an upstream registry.
    EnterpriseMirror,
    /// Local cache only.
    LocalCache,
    /// Offline snapshot only.
    OfflineSnapshot,
}

impl RegistrySourceAuthority {
    /// Every registry source authority, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PublicRegistry,
        Self::PrivateRegistry,
        Self::EnterpriseMirror,
        Self::LocalCache,
        Self::OfflineSnapshot,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicRegistry => "public_registry",
            Self::PrivateRegistry => "private_registry",
            Self::EnterpriseMirror => "enterprise_mirror",
            Self::LocalCache => "local_cache",
            Self::OfflineSnapshot => "offline_snapshot",
        }
    }

    /// The specific message class this source must render to.
    pub const fn canonical_message_class(self) -> PackageStateMessageClass {
        match self {
            Self::PublicRegistry => PackageStateMessageClass::PublicRegistrySource,
            Self::PrivateRegistry => PackageStateMessageClass::PrivateRegistrySource,
            Self::EnterpriseMirror => PackageStateMessageClass::MirrorBackedSource,
            Self::LocalCache => PackageStateMessageClass::CacheOnlySource,
            Self::OfflineSnapshot => PackageStateMessageClass::OfflineSnapshotDisclosure,
        }
    }

    /// Whether this source must always disclose itself specifically rather than
    /// collapsing into a generic not-found or install-failed message.
    pub const fn requires_specific_disclosure(self) -> bool {
        matches!(
            self,
            Self::EnterpriseMirror | Self::LocalCache | Self::OfflineSnapshot
        )
    }
}

/// Auth mode used to reach a registry or mirror for an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthMode {
    /// Anonymous registry access.
    Anonymous,
    /// Secret broker resolves credentials from the OS store.
    OsStoreCredential,
    /// Token-backed credential.
    TokenCredential,
    /// Browser or device-code sign-in.
    BrowserOrDeviceSignIn,
    /// Credential mode inherited from policy.
    PolicyInheritedCredential,
    /// Auth is required but not satisfied; the operation cannot proceed.
    AuthRequiredUnsatisfied,
}

impl AuthMode {
    /// Every auth mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Anonymous,
        Self::OsStoreCredential,
        Self::TokenCredential,
        Self::BrowserOrDeviceSignIn,
        Self::PolicyInheritedCredential,
        Self::AuthRequiredUnsatisfied,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Anonymous => "anonymous",
            Self::OsStoreCredential => "os_store_credential",
            Self::TokenCredential => "token_credential",
            Self::BrowserOrDeviceSignIn => "browser_or_device_sign_in",
            Self::PolicyInheritedCredential => "policy_inherited_credential",
            Self::AuthRequiredUnsatisfied => "auth_required_unsatisfied",
        }
    }

    /// Whether this mode blocks a mutation until auth is satisfied.
    pub const fn blocks_until_satisfied(self) -> bool {
        matches!(self, Self::AuthRequiredUnsatisfied)
    }
}

/// Authority that governs the resolved dependency set for a manifest scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfileAuthority {
    /// The lockfile pins exact resolutions and is authoritative.
    ExactLockfilePinned,
    /// The manifest range governs and the lockfile is derived from it.
    ManifestRangeGoverned,
    /// The lockfile is frozen by policy; resolution may not change it.
    FrozenByPolicy,
    /// No lockfile is present; resolution is recomputed each time.
    LockfileMissing,
    /// The lockfile and manifest disagree; mutation is blocked until reconciled.
    LockfileDivergent,
    /// Lockfile authority could not be established.
    AuthorityUnknown,
}

impl LockfileAuthority {
    /// Every lockfile authority, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactLockfilePinned,
        Self::ManifestRangeGoverned,
        Self::FrozenByPolicy,
        Self::LockfileMissing,
        Self::LockfileDivergent,
        Self::AuthorityUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactLockfilePinned => "exact_lockfile_pinned",
            Self::ManifestRangeGoverned => "manifest_range_governed",
            Self::FrozenByPolicy => "frozen_by_policy",
            Self::LockfileMissing => "lockfile_missing",
            Self::LockfileDivergent => "lockfile_divergent",
            Self::AuthorityUnknown => "authority_unknown",
        }
    }

    /// Whether this authority blocks a mutation until the lockfile is reconciled.
    pub const fn blocks_until_reconciled(self) -> bool {
        matches!(self, Self::LockfileDivergent)
    }
}

/// Identity of the resolver that produced the resolved dependency set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolverIdentityClass {
    /// Aureline's first-party resolver.
    FirstPartyResolver,
    /// The ecosystem's native resolver (for example cargo, npm, pip).
    EcosystemNativeResolver,
    /// A mirror-backed resolver pointed at an enterprise mirror.
    MirrorBackedResolver,
    /// An offline-cache resolver working only from local snapshots.
    OfflineCacheResolver,
    /// The resolver identity could not be established.
    ResolverUnknown,
}

impl ResolverIdentityClass {
    /// Every resolver identity, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FirstPartyResolver,
        Self::EcosystemNativeResolver,
        Self::MirrorBackedResolver,
        Self::OfflineCacheResolver,
        Self::ResolverUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyResolver => "first_party_resolver",
            Self::EcosystemNativeResolver => "ecosystem_native_resolver",
            Self::MirrorBackedResolver => "mirror_backed_resolver",
            Self::OfflineCacheResolver => "offline_cache_resolver",
            Self::ResolverUnknown => "resolver_unknown",
        }
    }
}

/// Rollback class for a package mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackClass {
    /// Exact reversal from a durable checkpoint.
    ReversibleCheckpointed,
    /// Manifest/lockfile reversal with no other side effects.
    ReversibleManifestOnly,
    /// Only a compensating cleanup of scripts or native artifacts is possible.
    CompensatingOnly,
    /// No safe reversal exists.
    Irreversible,
    /// No rollback applies; the operation is read-only.
    NotApplicable,
}

impl RollbackClass {
    /// Every rollback class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReversibleCheckpointed,
        Self::ReversibleManifestOnly,
        Self::CompensatingOnly,
        Self::Irreversible,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReversibleCheckpointed => "reversible_checkpointed",
            Self::ReversibleManifestOnly => "reversible_manifest_only",
            Self::CompensatingOnly => "compensating_only",
            Self::Irreversible => "irreversible",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// A subject whose privacy and retention the matrix binds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionSubject {
    /// Package operation history.
    OperationHistory,
    /// Registry credentials.
    RegistryCredentials,
    /// Support and export packets.
    SupportExportPacket,
}

impl RetentionSubject {
    /// Every retention subject, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::OperationHistory,
        Self::RegistryCredentials,
        Self::SupportExportPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OperationHistory => "operation_history",
            Self::RegistryCredentials => "registry_credentials",
            Self::SupportExportPacket => "support_export_packet",
        }
    }

    /// The retention class this subject must carry.
    pub const fn canonical_retention_class(self) -> RetentionClass {
        match self {
            Self::OperationHistory => RetentionClass::BoundedLocalHistory,
            Self::RegistryCredentials => RetentionClass::BrokerResolvedNeverPersisted,
            Self::SupportExportPacket => RetentionClass::RedactionRequiredExport,
        }
    }

    /// Whether projections of this subject must be redacted.
    pub const fn requires_redaction(self) -> bool {
        matches!(self, Self::RegistryCredentials | Self::SupportExportPacket)
    }
}

/// Retention class bound to a subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// Bounded, local-only history retained for a stated window.
    BoundedLocalHistory,
    /// Resolved by the secret broker on demand and never persisted in a packet.
    BrokerResolvedNeverPersisted,
    /// Retained only in redaction-required export form.
    RedactionRequiredExport,
}

impl RetentionClass {
    /// Every retention class, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::BoundedLocalHistory,
        Self::BrokerResolvedNeverPersisted,
        Self::RedactionRequiredExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundedLocalHistory => "bounded_local_history",
            Self::BrokerResolvedNeverPersisted => "broker_resolved_never_persisted",
            Self::RedactionRequiredExport => "redaction_required_export",
        }
    }
}

/// A marketed M5 package surface that must reference the shared matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageSurface {
    /// The desktop package workspace.
    DesktopPackageWorkspace,
    /// The CLI/headless package surface.
    CliHeadless,
    /// The AI context/inspect surface.
    AiContext,
    /// The review workspace.
    ReviewWorkspace,
    /// The support-export surface.
    SupportExport,
    /// The release/public-truth surface.
    ReleasePublicTruth,
}

impl PackageSurface {
    /// Every package surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopPackageWorkspace,
        Self::CliHeadless,
        Self::AiContext,
        Self::ReviewWorkspace,
        Self::SupportExport,
        Self::ReleasePublicTruth,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopPackageWorkspace => "desktop_package_workspace",
            Self::CliHeadless => "cli_headless",
            Self::AiContext => "ai_context",
            Self::ReviewWorkspace => "review_workspace",
            Self::SupportExport => "support_export",
            Self::ReleasePublicTruth => "release_public_truth",
        }
    }

    /// The write authority this surface may carry.
    pub const fn canonical_write_authority(self) -> SurfaceWriteAuthority {
        match self {
            Self::DesktopPackageWorkspace | Self::CliHeadless => SurfaceWriteAuthority::Mutates,
            Self::ReviewWorkspace => SurfaceWriteAuthority::Stages,
            Self::AiContext => SurfaceWriteAuthority::InspectOnly,
            Self::SupportExport | Self::ReleasePublicTruth => SurfaceWriteAuthority::RedactedExport,
        }
    }
}

/// Write authority a surface carries over package mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceWriteAuthority {
    /// May apply a mutation after review.
    Mutates,
    /// May stage a mutation for review but not apply it.
    Stages,
    /// Inspect-only; carries no write authority.
    InspectOnly,
    /// Produces a redacted export; carries no write authority.
    RedactedExport,
}

impl SurfaceWriteAuthority {
    /// Every write authority, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Mutates,
        Self::Stages,
        Self::InspectOnly,
        Self::RedactedExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mutates => "mutates",
            Self::Stages => "stages",
            Self::InspectOnly => "inspect_only",
            Self::RedactedExport => "redacted_export",
        }
    }

    /// Whether this authority can apply a mutation.
    pub const fn can_mutate(self) -> bool {
        matches!(self, Self::Mutates)
    }
}

/// One frozen row for a canonical package-state label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageStateRow {
    /// Stable row id.
    pub row_id: String,
    /// The canonical package-state label this row freezes.
    pub label: PackageStateLabel,
    /// Identity side; must equal [`PackageStateLabel::identity_side`].
    pub identity_side: IdentitySide,
    /// Message class; must equal [`PackageStateLabel::canonical_message_class`].
    pub message_class: PackageStateMessageClass,
    /// Whether this state is guarded against generic collapse; must equal
    /// [`PackageStateLabel::is_non_collapse_guarded`].
    pub non_collapse_guarded: bool,
    /// Doc anchor into the overview page.
    pub doc_anchor: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl PackageStateRow {
    /// Whether the stored identity side, message class, and guard flag all agree
    /// with the recomputed contract.
    pub fn is_consistent(&self) -> bool {
        self.identity_side == self.label.identity_side()
            && self.message_class == self.label.canonical_message_class()
            && self.non_collapse_guarded == self.label.is_non_collapse_guarded()
            && self.message_class.is_specific()
            && !(self.describes_requested() && self.describes_resolved())
    }

    /// Whether this row describes the requested identity.
    pub fn describes_requested(&self) -> bool {
        self.identity_side.describes_requested()
    }

    /// Whether this row describes the resolved identity.
    pub fn describes_resolved(&self) -> bool {
        self.identity_side.describes_resolved()
    }
}

/// One frozen cell for a registry source authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistrySourceCell {
    /// The registry source authority this cell freezes.
    pub source: RegistrySourceAuthority,
    /// Message class; must equal [`RegistrySourceAuthority::canonical_message_class`].
    pub message_class: PackageStateMessageClass,
    /// Whether the source must disclose itself specifically; must equal
    /// [`RegistrySourceAuthority::requires_specific_disclosure`].
    pub requires_specific_disclosure: bool,
    /// Redacted source label safe for support exports; never a raw URL or token.
    pub redacted_source_label: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl RegistrySourceCell {
    /// Whether the stored message class and disclosure flag agree with the
    /// recomputed contract and the message is never generic.
    pub fn is_consistent(&self) -> bool {
        self.message_class == self.source.canonical_message_class()
            && self.requires_specific_disclosure == self.source.requires_specific_disclosure()
            && self.message_class.is_specific()
    }
}

/// One frozen binding from a marketed surface to the shared matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceBinding {
    /// The package surface this binding governs.
    pub surface: PackageSurface,
    /// Write authority; must equal [`PackageSurface::canonical_write_authority`].
    pub write_authority: SurfaceWriteAuthority,
    /// The matrix packet id this surface references; must equal the packet id.
    pub references_matrix_id: String,
    /// Ref to the surface that renders the shared matrix.
    pub surface_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl SurfaceBinding {
    /// Whether the stored write authority agrees with the recomputed contract
    /// and the surface references the given matrix id.
    pub fn is_consistent(&self, packet_id: &str) -> bool {
        self.write_authority == self.surface.canonical_write_authority()
            && self.references_matrix_id == packet_id
    }
}

/// One frozen privacy/retention rule for a subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetentionRule {
    /// The subject this rule binds.
    pub subject: RetentionSubject,
    /// Retention class; must equal [`RetentionSubject::canonical_retention_class`].
    pub retention_class: RetentionClass,
    /// Whether the packet stores a credential body; must always be false.
    pub stores_credential_body: bool,
    /// Whether projections must be redacted; must equal
    /// [`RetentionSubject::requires_redaction`].
    pub redaction_required: bool,
    /// Human-readable retention window label.
    pub retention_window_label: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl RetentionRule {
    /// Whether the stored retention class, redaction flag, and credential-body
    /// flag agree with the recomputed contract.
    pub fn is_consistent(&self) -> bool {
        self.retention_class == self.subject.canonical_retention_class()
            && self.redaction_required == self.subject.requires_redaction()
            && !self.stores_credential_body
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PackageStateMatrixSummary {
    /// Total state rows.
    pub total_state_rows: usize,
    /// Total registry-source cells.
    pub total_registry_cells: usize,
    /// Total surface bindings.
    pub total_surface_bindings: usize,
    /// Total retention rules.
    pub total_retention_rules: usize,
    /// Number of canonical package-state labels claimed.
    pub state_label_count: usize,
    /// State rows guarded against generic collapse.
    pub non_collapse_guarded_states: usize,
    /// State rows describing the requested identity.
    pub requested_identity_states: usize,
    /// State rows describing the resolved identity.
    pub resolved_identity_states: usize,
    /// State rows that are finding overlays.
    pub finding_overlay_states: usize,
    /// State rows that are resolution-environment postures.
    pub resolution_environment_states: usize,
    /// Registry cells that must disclose themselves specifically.
    pub specific_disclosure_sources: usize,
    /// Surface bindings that may mutate.
    pub mutating_surfaces: usize,
    /// Surface bindings that produce redacted exports.
    pub redacted_export_surfaces: usize,
    /// Retention rules that require redaction.
    pub redaction_required_subjects: usize,
}

/// A redaction-safe export row projected from a state row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PackageStateMatrixExportRow {
    /// Row id.
    pub row_id: String,
    /// Package-state label token.
    pub label: String,
    /// Identity-side token.
    pub identity_side: String,
    /// Message-class token.
    pub message_class: String,
    /// Whether the state is guarded against generic collapse.
    pub non_collapse_guarded: bool,
    /// Whether the row describes the requested identity.
    pub describes_requested: bool,
    /// Whether the row describes the resolved identity.
    pub describes_resolved: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PackageStateMatrixExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected state rows.
    pub states: Vec<M5PackageStateMatrixExportRow>,
    /// Whether every row, cell, binding, and rule agrees with the contract.
    pub all_consistent: bool,
    /// Whether every surface references this matrix id.
    pub all_surfaces_reference_matrix: bool,
    /// Whether no state row or registry cell carries a generic message class.
    pub no_generic_collapse: bool,
    /// Number of states guarded against generic collapse.
    pub non_collapse_guarded_count: usize,
}

/// The typed M5 package-state, manifest-scope, registry-auth, and
/// lockfile-authority matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PackageStateMatrix {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed package-state label vocabulary.
    pub package_state_labels: Vec<PackageStateLabel>,
    /// Closed identity-side vocabulary.
    pub identity_sides: Vec<IdentitySide>,
    /// Closed message-class vocabulary.
    pub message_classes: Vec<PackageStateMessageClass>,
    /// Closed manifest-scope vocabulary.
    pub manifest_scope_classes: Vec<ManifestScopeClass>,
    /// Closed registry-source vocabulary.
    pub registry_source_classes: Vec<RegistrySourceAuthority>,
    /// Closed auth-mode vocabulary.
    pub auth_modes: Vec<AuthMode>,
    /// Closed lockfile-authority vocabulary.
    pub lockfile_authorities: Vec<LockfileAuthority>,
    /// Closed resolver-identity vocabulary.
    pub resolver_identities: Vec<ResolverIdentityClass>,
    /// Closed rollback-class vocabulary.
    pub rollback_classes: Vec<RollbackClass>,
    /// Closed retention-subject vocabulary.
    pub retention_subjects: Vec<RetentionSubject>,
    /// Closed retention-class vocabulary.
    pub retention_classes: Vec<RetentionClass>,
    /// Closed package-surface vocabulary.
    pub package_surfaces: Vec<PackageSurface>,
    /// Closed surface-write-authority vocabulary.
    pub surface_write_authorities: Vec<SurfaceWriteAuthority>,
    /// State rows, one per canonical package-state label.
    #[serde(default)]
    pub state_rows: Vec<PackageStateRow>,
    /// Registry-source cells, one per registry source authority.
    #[serde(default)]
    pub registry_source_cells: Vec<RegistrySourceCell>,
    /// Surface bindings, one per marketed surface.
    #[serde(default)]
    pub surface_bindings: Vec<SurfaceBinding>,
    /// Retention rules, one per retention subject.
    #[serde(default)]
    pub retention_rules: Vec<RetentionRule>,
    /// Summary counts.
    pub summary: M5PackageStateMatrixSummary,
}

impl M5PackageStateMatrix {
    /// Returns the row for a canonical package-state label.
    pub fn state(&self, label: PackageStateLabel) -> Option<&PackageStateRow> {
        self.state_rows.iter().find(|r| r.label == label)
    }

    /// Returns the cell for a registry source authority.
    pub fn registry_cell(&self, source: RegistrySourceAuthority) -> Option<&RegistrySourceCell> {
        self.registry_source_cells
            .iter()
            .find(|c| c.source == source)
    }

    /// Returns the binding for a marketed surface.
    pub fn binding(&self, surface: PackageSurface) -> Option<&SurfaceBinding> {
        self.surface_bindings.iter().find(|b| b.surface == surface)
    }

    /// Returns the rule for a retention subject.
    pub fn retention_rule(&self, subject: RetentionSubject) -> Option<&RetentionRule> {
        self.retention_rules.iter().find(|r| r.subject == subject)
    }

    /// Whether every state row agrees with the recomputed contract.
    pub fn all_states_consistent(&self) -> bool {
        self.state_rows.iter().all(PackageStateRow::is_consistent)
    }

    /// Whether every registry cell agrees with the recomputed contract.
    pub fn all_cells_consistent(&self) -> bool {
        self.registry_source_cells
            .iter()
            .all(RegistrySourceCell::is_consistent)
    }

    /// Whether every surface references this matrix id with the right authority.
    pub fn all_surfaces_reference_matrix(&self) -> bool {
        self.surface_bindings
            .iter()
            .all(|b| b.is_consistent(&self.packet_id))
    }

    /// Whether every retention rule agrees with the recomputed contract.
    pub fn all_retention_consistent(&self) -> bool {
        self.retention_rules
            .iter()
            .all(RetentionRule::is_consistent)
    }

    /// Whether no state row or registry cell carries a generic message class.
    pub fn no_generic_collapse(&self) -> bool {
        self.state_rows
            .iter()
            .all(|r| r.message_class.is_specific())
            && self
                .registry_source_cells
                .iter()
                .all(|c| c.message_class.is_specific())
    }

    /// Whether every part of the packet agrees with the recomputed contract.
    pub fn all_consistent(&self) -> bool {
        self.all_states_consistent()
            && self.all_cells_consistent()
            && self.all_surfaces_reference_matrix()
            && self.all_retention_consistent()
            && self.no_generic_collapse()
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5PackageStateMatrixSummary {
        let count_side = |side: IdentitySide| {
            self.state_rows
                .iter()
                .filter(|r| r.identity_side == side)
                .count()
        };
        M5PackageStateMatrixSummary {
            total_state_rows: self.state_rows.len(),
            total_registry_cells: self.registry_source_cells.len(),
            total_surface_bindings: self.surface_bindings.len(),
            total_retention_rules: self.retention_rules.len(),
            state_label_count: self.package_state_labels.len(),
            non_collapse_guarded_states: self
                .state_rows
                .iter()
                .filter(|r| r.non_collapse_guarded)
                .count(),
            requested_identity_states: self
                .state_rows
                .iter()
                .filter(|r| r.describes_requested())
                .count(),
            resolved_identity_states: self
                .state_rows
                .iter()
                .filter(|r| r.describes_resolved())
                .count(),
            finding_overlay_states: count_side(IdentitySide::FindingOverlay),
            resolution_environment_states: count_side(IdentitySide::ResolutionEnvironment),
            specific_disclosure_sources: self
                .registry_source_cells
                .iter()
                .filter(|c| c.requires_specific_disclosure)
                .count(),
            mutating_surfaces: self
                .surface_bindings
                .iter()
                .filter(|b| b.write_authority.can_mutate())
                .count(),
            redacted_export_surfaces: self
                .surface_bindings
                .iter()
                .filter(|b| b.write_authority == SurfaceWriteAuthority::RedactedExport)
                .count(),
            redaction_required_subjects: self
                .retention_rules
                .iter()
                .filter(|r| r.redaction_required)
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — the desktop
    /// workspace, CLI/headless, AI context, review workspace, support exports,
    /// and release/public-truth packets — render instead of restating
    /// package-state text by hand.
    pub fn export_projection(&self) -> M5PackageStateMatrixExportProjection {
        let states = self
            .state_rows
            .iter()
            .map(|r| M5PackageStateMatrixExportRow {
                row_id: r.row_id.clone(),
                label: r.label.as_str().to_owned(),
                identity_side: r.identity_side.as_str().to_owned(),
                message_class: r.message_class.as_str().to_owned(),
                non_collapse_guarded: r.non_collapse_guarded,
                describes_requested: r.describes_requested(),
                describes_resolved: r.describes_resolved(),
                summary: format!(
                    "{}: identity side {}, renders {}{}",
                    r.label.as_str(),
                    r.identity_side.as_str(),
                    r.message_class.as_str(),
                    if r.non_collapse_guarded {
                        " (guarded against generic collapse)"
                    } else {
                        ""
                    }
                ),
            })
            .collect();
        M5PackageStateMatrixExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            states,
            all_consistent: self.all_consistent(),
            all_surfaces_reference_matrix: self.all_surfaces_reference_matrix(),
            no_generic_collapse: self.no_generic_collapse(),
            non_collapse_guarded_count: self
                .state_rows
                .iter()
                .filter(|r| r.non_collapse_guarded)
                .count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5PackageStateMatrixViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_state_rows(&mut violations);
        self.validate_registry_cells(&mut violations);
        self.validate_surface_bindings(&mut violations);
        self.validate_retention_rules(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5PackageStateMatrixViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5PackageStateMatrixViolation>) {
        if self.schema_version != M5_PACKAGE_STATE_MATRIX_SCHEMA_VERSION {
            violations.push(M5PackageStateMatrixViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_PACKAGE_STATE_MATRIX_RECORD_KIND {
            violations.push(M5PackageStateMatrixViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5PackageStateMatrixViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "package_state_labels",
                self.package_state_labels == PackageStateLabel::ALL.to_vec(),
            ),
            (
                "identity_sides",
                self.identity_sides == IdentitySide::ALL.to_vec(),
            ),
            (
                "message_classes",
                self.message_classes == PackageStateMessageClass::ALL.to_vec(),
            ),
            (
                "manifest_scope_classes",
                self.manifest_scope_classes == ManifestScopeClass::ALL.to_vec(),
            ),
            (
                "registry_source_classes",
                self.registry_source_classes == RegistrySourceAuthority::ALL.to_vec(),
            ),
            ("auth_modes", self.auth_modes == AuthMode::ALL.to_vec()),
            (
                "lockfile_authorities",
                self.lockfile_authorities == LockfileAuthority::ALL.to_vec(),
            ),
            (
                "resolver_identities",
                self.resolver_identities == ResolverIdentityClass::ALL.to_vec(),
            ),
            (
                "rollback_classes",
                self.rollback_classes == RollbackClass::ALL.to_vec(),
            ),
            (
                "retention_subjects",
                self.retention_subjects == RetentionSubject::ALL.to_vec(),
            ),
            (
                "retention_classes",
                self.retention_classes == RetentionClass::ALL.to_vec(),
            ),
            (
                "package_surfaces",
                self.package_surfaces == PackageSurface::ALL.to_vec(),
            ),
            (
                "surface_write_authorities",
                self.surface_write_authorities == SurfaceWriteAuthority::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5PackageStateMatrixViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_state_rows(&self, violations: &mut Vec<M5PackageStateMatrixViolation>) {
        let mut seen_ids = BTreeSet::new();
        let mut seen_labels = BTreeSet::new();
        for row in &self.state_rows {
            if !seen_ids.insert(row.row_id.clone()) {
                violations.push(M5PackageStateMatrixViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            if !seen_labels.insert(row.label) {
                violations.push(M5PackageStateMatrixViolation::DuplicateStateRow {
                    label: row.label.as_str(),
                });
            }
            for (field, value) in [
                ("row_id", &row.row_id),
                ("doc_anchor", &row.doc_anchor),
                ("note", &row.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5PackageStateMatrixViolation::EmptyField {
                        id: row.row_id.clone(),
                        field_name: field,
                    });
                }
            }
            if row.identity_side != row.label.identity_side() {
                violations.push(M5PackageStateMatrixViolation::IdentitySideMismatch {
                    row_id: row.row_id.clone(),
                    declared: row.identity_side.as_str(),
                    required: row.label.identity_side().as_str(),
                });
            }
            if row.message_class != row.label.canonical_message_class() {
                violations.push(M5PackageStateMatrixViolation::MessageClassMismatch {
                    row_id: row.row_id.clone(),
                    declared: row.message_class.as_str(),
                    required: row.label.canonical_message_class().as_str(),
                });
            }
            if row.non_collapse_guarded != row.label.is_non_collapse_guarded() {
                violations.push(M5PackageStateMatrixViolation::GuardFlagMismatch {
                    row_id: row.row_id.clone(),
                });
            }
            if row.message_class.is_generic_collapse() {
                violations.push(M5PackageStateMatrixViolation::GenericCollapseMessage {
                    id: row.row_id.clone(),
                    message: row.message_class.as_str(),
                });
            }
            if row.describes_requested() && row.describes_resolved() {
                violations.push(M5PackageStateMatrixViolation::RequestedResolvedConflated {
                    row_id: row.row_id.clone(),
                });
            }
        }
        for &label in &self.package_state_labels {
            if !seen_labels.contains(&label) {
                violations.push(M5PackageStateMatrixViolation::MissingStateRow {
                    label: label.as_str(),
                });
            }
        }
    }

    fn validate_registry_cells(&self, violations: &mut Vec<M5PackageStateMatrixViolation>) {
        let mut seen = BTreeSet::new();
        for cell in &self.registry_source_cells {
            if !seen.insert(cell.source) {
                violations.push(M5PackageStateMatrixViolation::DuplicateRegistryCell {
                    source: cell.source.as_str(),
                });
            }
            if cell.redacted_source_label.trim().is_empty() {
                violations.push(M5PackageStateMatrixViolation::EmptyField {
                    id: cell.source.as_str().to_owned(),
                    field_name: "redacted_source_label",
                });
            }
            if cell.note.trim().is_empty() {
                violations.push(M5PackageStateMatrixViolation::EmptyField {
                    id: cell.source.as_str().to_owned(),
                    field_name: "note",
                });
            }
            if cell.message_class != cell.source.canonical_message_class() {
                violations.push(M5PackageStateMatrixViolation::MessageClassMismatch {
                    row_id: cell.source.as_str().to_owned(),
                    declared: cell.message_class.as_str(),
                    required: cell.source.canonical_message_class().as_str(),
                });
            }
            if cell.requires_specific_disclosure != cell.source.requires_specific_disclosure() {
                violations.push(M5PackageStateMatrixViolation::DisclosureFlagMismatch {
                    source: cell.source.as_str(),
                });
            }
            if cell.message_class.is_generic_collapse() {
                violations.push(M5PackageStateMatrixViolation::GenericCollapseMessage {
                    id: cell.source.as_str().to_owned(),
                    message: cell.message_class.as_str(),
                });
            }
        }
        for &source in &self.registry_source_classes {
            if !seen.contains(&source) {
                violations.push(M5PackageStateMatrixViolation::MissingRegistryCell {
                    source: source.as_str(),
                });
            }
        }
    }

    fn validate_surface_bindings(&self, violations: &mut Vec<M5PackageStateMatrixViolation>) {
        let mut seen = BTreeSet::new();
        for binding in &self.surface_bindings {
            if !seen.insert(binding.surface) {
                violations.push(M5PackageStateMatrixViolation::DuplicateSurfaceBinding {
                    surface: binding.surface.as_str(),
                });
            }
            if binding.surface_ref.trim().is_empty() {
                violations.push(M5PackageStateMatrixViolation::EmptyField {
                    id: binding.surface.as_str().to_owned(),
                    field_name: "surface_ref",
                });
            }
            if binding.note.trim().is_empty() {
                violations.push(M5PackageStateMatrixViolation::EmptyField {
                    id: binding.surface.as_str().to_owned(),
                    field_name: "note",
                });
            }
            if binding.write_authority != binding.surface.canonical_write_authority() {
                violations.push(M5PackageStateMatrixViolation::WriteAuthorityMismatch {
                    surface: binding.surface.as_str(),
                    declared: binding.write_authority.as_str(),
                    required: binding.surface.canonical_write_authority().as_str(),
                });
            }
            if binding.references_matrix_id != self.packet_id {
                violations.push(
                    M5PackageStateMatrixViolation::SurfaceReferencesWrongMatrix {
                        surface: binding.surface.as_str(),
                        referenced: binding.references_matrix_id.clone(),
                    },
                );
            }
        }
        for &surface in &self.package_surfaces {
            if !seen.contains(&surface) {
                violations.push(M5PackageStateMatrixViolation::MissingSurfaceBinding {
                    surface: surface.as_str(),
                });
            }
        }
    }

    fn validate_retention_rules(&self, violations: &mut Vec<M5PackageStateMatrixViolation>) {
        let mut seen = BTreeSet::new();
        for rule in &self.retention_rules {
            if !seen.insert(rule.subject) {
                violations.push(M5PackageStateMatrixViolation::DuplicateRetentionRule {
                    subject: rule.subject.as_str(),
                });
            }
            for (field, value) in [
                ("retention_window_label", &rule.retention_window_label),
                ("note", &rule.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5PackageStateMatrixViolation::EmptyField {
                        id: rule.subject.as_str().to_owned(),
                        field_name: field,
                    });
                }
            }
            if rule.retention_class != rule.subject.canonical_retention_class() {
                violations.push(M5PackageStateMatrixViolation::RetentionClassMismatch {
                    subject: rule.subject.as_str(),
                    declared: rule.retention_class.as_str(),
                    required: rule.subject.canonical_retention_class().as_str(),
                });
            }
            if rule.redaction_required != rule.subject.requires_redaction() {
                violations.push(M5PackageStateMatrixViolation::RedactionFlagMismatch {
                    subject: rule.subject.as_str(),
                });
            }
            // No packet may ever carry a credential body, regardless of subject.
            if rule.stores_credential_body {
                violations.push(M5PackageStateMatrixViolation::CredentialBodyStored {
                    subject: rule.subject.as_str(),
                });
            }
        }
        for &subject in &self.retention_subjects {
            if !seen.contains(&subject) {
                violations.push(M5PackageStateMatrixViolation::MissingRetentionRule {
                    subject: subject.as_str(),
                });
            }
        }
    }
}

/// A validation violation for the M5 package-state matrix packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5PackageStateMatrixViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A state-row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A package-state label carries more than one row.
    DuplicateStateRow {
        /// Label token.
        label: &'static str,
    },
    /// A claimed package-state label has no row.
    MissingStateRow {
        /// Label token.
        label: &'static str,
    },
    /// A row's identity side disagrees with the recomputed side.
    IdentitySideMismatch {
        /// Row id.
        row_id: String,
        /// Declared identity-side token.
        declared: &'static str,
        /// Required identity-side token.
        required: &'static str,
    },
    /// A row or cell's message class disagrees with the recomputed class.
    MessageClassMismatch {
        /// Row or source id.
        row_id: String,
        /// Declared message-class token.
        declared: &'static str,
        /// Required message-class token.
        required: &'static str,
    },
    /// A row's non-collapse guard flag disagrees with the recomputed flag.
    GuardFlagMismatch {
        /// Row id.
        row_id: String,
    },
    /// A row or cell carries a forbidden generic message class.
    GenericCollapseMessage {
        /// Row or source id.
        id: String,
        /// Generic message-class token.
        message: &'static str,
    },
    /// A row claims to describe both the requested and resolved identity.
    RequestedResolvedConflated {
        /// Row id.
        row_id: String,
    },
    /// A registry source authority carries more than one cell.
    DuplicateRegistryCell {
        /// Source token.
        source: &'static str,
    },
    /// A claimed registry source authority has no cell.
    MissingRegistryCell {
        /// Source token.
        source: &'static str,
    },
    /// A cell's disclosure flag disagrees with the recomputed flag.
    DisclosureFlagMismatch {
        /// Source token.
        source: &'static str,
    },
    /// A marketed surface carries more than one binding.
    DuplicateSurfaceBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A claimed marketed surface has no binding.
    MissingSurfaceBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A surface binding's write authority disagrees with the recomputed value.
    WriteAuthorityMismatch {
        /// Surface token.
        surface: &'static str,
        /// Declared write-authority token.
        declared: &'static str,
        /// Required write-authority token.
        required: &'static str,
    },
    /// A surface references a matrix id other than this packet's.
    SurfaceReferencesWrongMatrix {
        /// Surface token.
        surface: &'static str,
        /// Referenced matrix id.
        referenced: String,
    },
    /// A retention subject carries more than one rule.
    DuplicateRetentionRule {
        /// Subject token.
        subject: &'static str,
    },
    /// A claimed retention subject has no rule.
    MissingRetentionRule {
        /// Subject token.
        subject: &'static str,
    },
    /// A retention rule's class disagrees with the recomputed class.
    RetentionClassMismatch {
        /// Subject token.
        subject: &'static str,
        /// Declared retention-class token.
        declared: &'static str,
        /// Required retention-class token.
        required: &'static str,
    },
    /// A retention rule's redaction flag disagrees with the recomputed flag.
    RedactionFlagMismatch {
        /// Subject token.
        subject: &'static str,
    },
    /// A retention rule claims the packet stores a credential body.
    CredentialBodyStored {
        /// Subject token.
        subject: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5PackageStateMatrixViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical vocabulary")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => {
                write!(f, "duplicate state-row id {row_id}")
            }
            Self::DuplicateStateRow { label } => {
                write!(f, "duplicate row for package-state label {label}")
            }
            Self::MissingStateRow { label } => {
                write!(f, "missing row for claimed package-state label {label}")
            }
            Self::IdentitySideMismatch {
                row_id,
                declared,
                required,
            } => write!(
                f,
                "row {row_id} records identity side {declared} but the label requires {required}"
            ),
            Self::MessageClassMismatch {
                row_id,
                declared,
                required,
            } => write!(
                f,
                "{row_id} records message class {declared} but the contract requires {required}"
            ),
            Self::GuardFlagMismatch { row_id } => {
                write!(f, "row {row_id} non-collapse guard flag disagrees with the label")
            }
            Self::GenericCollapseMessage { id, message } => write!(
                f,
                "{id} renders forbidden generic message class {message}"
            ),
            Self::RequestedResolvedConflated { row_id } => write!(
                f,
                "row {row_id} describes both the requested and resolved identity"
            ),
            Self::DuplicateRegistryCell { source } => {
                write!(f, "duplicate cell for registry source {source}")
            }
            Self::MissingRegistryCell { source } => {
                write!(f, "missing cell for claimed registry source {source}")
            }
            Self::DisclosureFlagMismatch { source } => write!(
                f,
                "registry source {source} disclosure flag disagrees with the contract"
            ),
            Self::DuplicateSurfaceBinding { surface } => {
                write!(f, "duplicate binding for surface {surface}")
            }
            Self::MissingSurfaceBinding { surface } => {
                write!(f, "missing binding for claimed surface {surface}")
            }
            Self::WriteAuthorityMismatch {
                surface,
                declared,
                required,
            } => write!(
                f,
                "surface {surface} records write authority {declared} but the contract requires {required}"
            ),
            Self::SurfaceReferencesWrongMatrix {
                surface,
                referenced,
            } => write!(
                f,
                "surface {surface} references matrix id {referenced} instead of this packet"
            ),
            Self::DuplicateRetentionRule { subject } => {
                write!(f, "duplicate rule for retention subject {subject}")
            }
            Self::MissingRetentionRule { subject } => {
                write!(f, "missing rule for claimed retention subject {subject}")
            }
            Self::RetentionClassMismatch {
                subject,
                declared,
                required,
            } => write!(
                f,
                "subject {subject} records retention class {declared} but the contract requires {required}"
            ),
            Self::RedactionFlagMismatch { subject } => write!(
                f,
                "subject {subject} redaction flag disagrees with the contract"
            ),
            Self::CredentialBodyStored { subject } => write!(
                f,
                "subject {subject} claims the packet stores a credential body"
            ),
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5PackageStateMatrixViolation {}

/// Loads the embedded M5 package-state matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5PackageStateMatrix`].
pub fn current_m5_package_state_matrix() -> Result<M5PackageStateMatrix, serde_json::Error> {
    serde_json::from_str(M5_PACKAGE_STATE_MATRIX_JSON)
}

#[cfg(test)]
mod tests;
