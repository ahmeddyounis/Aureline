//! Package-manager mutation review alpha for the TS/JS launch wedge.
//!
//! This module owns the first runtime consumer for package dependency
//! mutations. It reads a Node workspace manifest and lockfile topology,
//! combines that with registry/auth and script-risk descriptors supplied by
//! the caller, and emits review, audit, and support packets before any package
//! manager can write files or execute lifecycle hooks.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::detectors::node::{
    NodePackageManagerKind, NodeToolchainDetection, NodeToolchainDetector,
    NodeToolchainDetectorConfig, NodeToolchainResolutionState,
};

/// Schema version for package-mutation alpha packets.
pub const PACKAGE_OPERATION_ALPHA_SCHEMA_VERSION: u32 = 1;
/// Stable record-kind tag for [`ManifestScopeAlphaDescriptor`].
pub const MANIFEST_SCOPE_ALPHA_RECORD_KIND: &str = "manifest_scope_alpha_descriptor";
/// Stable record-kind tag for [`RegistrySourceAlphaDescriptor`].
pub const REGISTRY_SOURCE_ALPHA_RECORD_KIND: &str = "registry_source_alpha_descriptor";
/// Stable record-kind tag for [`LockfileImpactAlphaRecord`].
pub const LOCKFILE_IMPACT_ALPHA_RECORD_KIND: &str = "lockfile_impact_alpha_record";
/// Stable record-kind tag for [`PackageOperationAlphaPacket`].
pub const PACKAGE_OPERATION_ALPHA_RECORD_KIND: &str = "package_operation_alpha_packet";
/// Stable record-kind tag for [`PackageOperationAuditPacket`].
pub const PACKAGE_OPERATION_AUDIT_RECORD_KIND: &str = "package_operation_audit_packet";
/// Stable record-kind tag for [`PackageOperationSupportExport`].
pub const PACKAGE_OPERATION_SUPPORT_EXPORT_RECORD_KIND: &str = "package_operation_support_export";
/// Runtime implementation version quoted by package-operation packets.
pub const PACKAGE_MUTATION_REVIEWER_VERSION: &str = "package_mutation.reviewer.alpha.v1";

/// Package-manager family for the bounded runtime alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageManagerFamily {
    /// npm workspaces or a single npm package.
    Npm,
    /// pnpm workspaces or a single pnpm package.
    Pnpm,
    /// The package manager could not be safely classified.
    PackageManagerUnknownRequiresReview,
}

impl PackageManagerFamily {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Pnpm => "pnpm",
            Self::PackageManagerUnknownRequiresReview => "package_manager_unknown_requires_review",
        }
    }

    fn from_node_detection(detection: &NodeToolchainDetection) -> Self {
        match (
            detection.package_manager.kind,
            detection.package_manager.resolution_state,
        ) {
            (Some(NodePackageManagerKind::Npm), NodeToolchainResolutionState::Resolved)
            | (Some(NodePackageManagerKind::Npm), NodeToolchainResolutionState::Fallback) => {
                Self::Npm
            }
            (Some(NodePackageManagerKind::Pnpm), NodeToolchainResolutionState::Resolved)
            | (Some(NodePackageManagerKind::Pnpm), NodeToolchainResolutionState::Fallback) => {
                Self::Pnpm
            }
            _ => Self::PackageManagerUnknownRequiresReview,
        }
    }
}

/// Package operation class re-exported from the package-action vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageOperationClass {
    /// Install a new dependency entry into the active manifest.
    InstallNewDependency,
    /// Upgrade an existing dependency entry in the active manifest.
    UpgradeExistingDependency,
    /// Remove an existing dependency entry from the active manifest.
    RemoveExistingDependency,
    /// Inspect dependency state without mutating files.
    AuditOnlyNoStateChange,
    /// Restore a lockfile to a previously reviewed checkpoint.
    RestoreLockfileToCheckpoint,
}

impl PackageOperationClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallNewDependency => "install_new_dependency",
            Self::UpgradeExistingDependency => "upgrade_existing_dependency",
            Self::RemoveExistingDependency => "remove_existing_dependency",
            Self::AuditOnlyNoStateChange => "audit_only_no_state_change",
            Self::RestoreLockfileToCheckpoint => "restore_lockfile_to_checkpoint",
        }
    }

    /// True when the operation can change manifest, lockfile, cache, or hooks.
    pub const fn is_mutating(self) -> bool {
        !matches!(self, Self::AuditOnlyNoStateChange)
    }
}

/// Dependency table targeted inside a JavaScript manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencySection {
    /// `dependencies`.
    Dependencies,
    /// `devDependencies`.
    DevDependencies,
    /// `optionalDependencies`.
    OptionalDependencies,
    /// `peerDependencies`.
    PeerDependencies,
}

impl DependencySection {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dependencies => "dependencies",
            Self::DevDependencies => "dev_dependencies",
            Self::OptionalDependencies => "optional_dependencies",
            Self::PeerDependencies => "peer_dependencies",
        }
    }

    fn package_json_key(self) -> &'static str {
        match self {
            Self::Dependencies => "dependencies",
            Self::DevDependencies => "devDependencies",
            Self::OptionalDependencies => "optionalDependencies",
            Self::PeerDependencies => "peerDependencies",
        }
    }
}

/// Manifest scope class re-exported from the package-action vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestScopeClass {
    /// The root manifest owns the requested dependency change.
    WorkspaceRootManifest,
    /// A workspace member manifest owns the requested dependency change.
    WorkspaceMemberManifest,
    /// A monorepo root manifest owns the dependency set.
    MonorepoRootManifest,
    /// Application-only dev dependency manifest.
    ApplicationOnlyDevDependencyManifest,
    /// Vendored or offline manifest.
    VendoredOrOfflineManifest,
    /// Out-of-tree global manifest requiring admin review.
    OutOfTreeGlobalManifestAdminOnly,
    /// Scope could not be proven and must be reviewed.
    ManifestScopeUnknownRequiresReview,
}

impl ManifestScopeClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceRootManifest => "workspace_root_manifest",
            Self::WorkspaceMemberManifest => "workspace_member_manifest",
            Self::MonorepoRootManifest => "monorepo_root_manifest",
            Self::ApplicationOnlyDevDependencyManifest => {
                "application_only_dev_dependency_manifest"
            }
            Self::VendoredOrOfflineManifest => "vendored_or_offline_manifest",
            Self::OutOfTreeGlobalManifestAdminOnly => "out_of_tree_global_manifest_admin_only",
            Self::ManifestScopeUnknownRequiresReview => "manifest_scope_unknown_requires_review",
        }
    }
}

/// Manifest delta class re-exported from execution/package schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestDeltaClass {
    /// Dependency entry will be added.
    DependencyEntryAdded,
    /// Dependency entry will be updated.
    DependencyEntryUpdated,
    /// Dependency entry will be removed.
    DependencyEntryRemoved,
    /// Manifest remains unchanged because the operation is read-only.
    ManifestUnchangedAuditOnly,
    /// Manifest delta could not be proven and must be reviewed.
    ManifestDeltaUnknownRequiresReview,
}

impl ManifestDeltaClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DependencyEntryAdded => "dependency_entry_added",
            Self::DependencyEntryUpdated => "dependency_entry_updated",
            Self::DependencyEntryRemoved => "dependency_entry_removed",
            Self::ManifestUnchangedAuditOnly => "manifest_unchanged_audit_only",
            Self::ManifestDeltaUnknownRequiresReview => "manifest_delta_unknown_requires_review",
        }
    }
}

/// State of the requested dependency entry in the active manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestRequirementState {
    /// The dependency entry is absent.
    Absent,
    /// The dependency entry exists and matches the requested requirement.
    PresentSameRequirement,
    /// The dependency entry exists but differs from the requested requirement.
    PresentDifferentRequirement,
    /// The manifest could not be read or parsed.
    UnknownManifestUnavailable,
}

impl ManifestRequirementState {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::PresentSameRequirement => "present_same_requirement",
            Self::PresentDifferentRequirement => "present_different_requirement",
            Self::UnknownManifestUnavailable => "unknown_manifest_unavailable",
        }
    }
}

/// Registry source class re-exported from package-action and execution schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistrySourceClass {
    /// Public default registry for the package manager.
    PublicDefaultRegistry,
    /// Public alternate registry.
    PublicAlternateRegistry,
    /// Vendor-published mirror.
    VendorPublishedMirror,
    /// Customer-operated mirror.
    CustomerOperatedMirror,
    /// Private internal registry.
    PrivateInternalRegistry,
    /// Managed organization-curated registry.
    ManagedOrgCuratedRegistry,
    /// Offline bundle registry.
    OfflineBundleRegistry,
    /// Vendored directory without registry lookup.
    VendoredDirectoryNoRegistry,
    /// Git or path dependency without registry lookup.
    GitOrPathDependencyNoRegistry,
    /// Source could not be proven and must be reviewed.
    RegistrySourceUnknownRequiresReview,
}

impl RegistrySourceClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicDefaultRegistry => "public_default_registry",
            Self::PublicAlternateRegistry => "public_alternate_registry",
            Self::VendorPublishedMirror => "vendor_published_mirror",
            Self::CustomerOperatedMirror => "customer_operated_mirror",
            Self::PrivateInternalRegistry => "private_internal_registry",
            Self::ManagedOrgCuratedRegistry => "managed_org_curated_registry",
            Self::OfflineBundleRegistry => "offline_bundle_registry",
            Self::VendoredDirectoryNoRegistry => "vendored_directory_no_registry",
            Self::GitOrPathDependencyNoRegistry => "git_or_path_dependency_no_registry",
            Self::RegistrySourceUnknownRequiresReview => "registry_source_unknown_requires_review",
        }
    }

    /// Coarse source family rendered by compact registry/auth banners.
    pub const fn source_family(self) -> &'static str {
        match self {
            Self::PublicDefaultRegistry | Self::PublicAlternateRegistry => "public",
            Self::PrivateInternalRegistry | Self::ManagedOrgCuratedRegistry => "private",
            Self::VendorPublishedMirror | Self::CustomerOperatedMirror => "mirror",
            Self::OfflineBundleRegistry => "offline",
            Self::VendoredDirectoryNoRegistry | Self::GitOrPathDependencyNoRegistry => {
                "workspace_or_vendored"
            }
            Self::RegistrySourceUnknownRequiresReview => "unknown",
        }
    }
}

/// Registry auth mode re-exported from the package-action vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryAuthModeClass {
    /// No auth required for public registry reads.
    NoAuthPublicRegistry,
    /// Auth resolved through the secret broker.
    SecretBrokerHandleAuth,
    /// Auth resolved through delegated identity.
    DelegatedIdentityAuth,
    /// Auth injected by policy.
    PolicyInjectedCredentialAuth,
    /// Mirror or offline registry requires no credential.
    MirrorOrOfflineNoAuthRequired,
    /// Managed service identity.
    ManagedServiceIdentityAuth,
    /// mTLS client certificate.
    MtlsClientCertificateAuth,
    /// Device-flow callback.
    DeviceFlowCallbackAuth,
    /// Auth could not be proven and must be reviewed.
    RegistryAuthUnknownRequiresReview,
    /// Auth mode is unsupported and blocked.
    RegistryAuthUnsupportedBlocked,
}

impl RegistryAuthModeClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAuthPublicRegistry => "no_auth_public_registry",
            Self::SecretBrokerHandleAuth => "secret_broker_handle_auth",
            Self::DelegatedIdentityAuth => "delegated_identity_auth",
            Self::PolicyInjectedCredentialAuth => "policy_injected_credential_auth",
            Self::MirrorOrOfflineNoAuthRequired => "mirror_or_offline_no_auth_required",
            Self::ManagedServiceIdentityAuth => "managed_service_identity_auth",
            Self::MtlsClientCertificateAuth => "mtls_client_certificate_auth",
            Self::DeviceFlowCallbackAuth => "device_flow_callback_auth",
            Self::RegistryAuthUnknownRequiresReview => "registry_auth_unknown_requires_review",
            Self::RegistryAuthUnsupportedBlocked => "registry_auth_unsupported_blocked",
        }
    }

    /// True when this auth mode blocks mutation until the user or policy fixes it.
    pub const fn blocks_mutation(self) -> bool {
        matches!(
            self,
            Self::RegistryAuthUnknownRequiresReview | Self::RegistryAuthUnsupportedBlocked
        )
    }
}

/// Freshness state for registry or mirror evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryFreshnessClass {
    /// Live source is authoritative.
    AuthoritativeLive,
    /// Cached source is within the freshness window.
    CachedWithinFreshnessWindow,
    /// Mirror snapshot is within the policy window.
    MirrorSnapshotWithinPolicyWindow,
    /// Source is stale and must be reviewed.
    StaleRequiresReview,
    /// Offline bundle is pinned.
    OfflineBundlePinned,
    /// Freshness could not be proven and must be reviewed.
    FreshnessUnknownRequiresReview,
}

impl RegistryFreshnessClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::CachedWithinFreshnessWindow => "cached_within_freshness_window",
            Self::MirrorSnapshotWithinPolicyWindow => "mirror_snapshot_within_policy_window",
            Self::StaleRequiresReview => "stale_requires_review",
            Self::OfflineBundlePinned => "offline_bundle_pinned",
            Self::FreshnessUnknownRequiresReview => "freshness_unknown_requires_review",
        }
    }
}

/// Revocation or yank state for registry evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryRevocationStateClass {
    /// Revocation was checked against the current source.
    RevocationCheckedCurrent,
    /// Revocation was checked from mirror evidence.
    RevocationCheckedFromMirror,
    /// Revocation check is stale or unavailable.
    RevocationCheckUnavailableStale,
    /// Revocation is blocked by policy.
    RevocationBlockedByPolicy,
    /// Revocation state could not be proven and must be reviewed.
    RevocationStatusUnknownRequiresReview,
}

impl RegistryRevocationStateClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevocationCheckedCurrent => "revocation_checked_current",
            Self::RevocationCheckedFromMirror => "revocation_checked_from_mirror",
            Self::RevocationCheckUnavailableStale => "revocation_check_unavailable_stale",
            Self::RevocationBlockedByPolicy => "revocation_blocked_by_policy",
            Self::RevocationStatusUnknownRequiresReview => {
                "revocation_status_unknown_requires_review"
            }
        }
    }
}

/// Mirror/offline state re-exported from the package-action vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorOrOfflineStateClass {
    /// Online default origin is admissible.
    OnlineDefaultOriginAdmissible,
    /// Online mirror is pinned and direct origin is not admissible.
    OnlineMirrorPinnedNoDirectOrigin,
    /// Offline grace is using a warm cache.
    OfflineGraceWindowUsingWarmCache,
    /// Air-gapped profile uses an offline bundle only.
    OfflineAirGappedUsingOfflineBundleOnly,
    /// Network disabled by user setting or policy.
    NetworkDisabledUserSettingOrPolicy,
    /// Mirror/offline state could not be proven and must be reviewed.
    MirrorOrOfflineStateUnknownRequiresReview,
}

impl MirrorOrOfflineStateClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnlineDefaultOriginAdmissible => "online_default_origin_admissible",
            Self::OnlineMirrorPinnedNoDirectOrigin => "online_mirror_pinned_no_direct_origin",
            Self::OfflineGraceWindowUsingWarmCache => "offline_grace_window_using_warm_cache",
            Self::OfflineAirGappedUsingOfflineBundleOnly => {
                "offline_air_gapped_using_offline_bundle_only"
            }
            Self::NetworkDisabledUserSettingOrPolicy => "network_disabled_user_setting_or_policy",
            Self::MirrorOrOfflineStateUnknownRequiresReview => {
                "mirror_or_offline_state_unknown_requires_review"
            }
        }
    }
}

/// Script and native-build risk class re-exported from package-action schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptRiskClass {
    /// No scripts or native build steps are expected.
    NoScriptNoNativeBuild,
    /// Declarative metadata only.
    DeclarativeMetadataOnly,
    /// Post-install script runs inside a sandbox.
    PostInstallScriptRunsInSandbox,
    /// Post-install script needs unsandboxed user consent.
    PostInstallScriptRunsUnsandboxedUserConsentRequired,
    /// Native compilation is required on the local toolchain.
    NativeCompilationRequiredLocalToolchain,
    /// Prebuilt binary loader is expected.
    PrebuiltBinaryWithRuntimeLoader,
    /// Platform-specific binary origin is unverified.
    PlatformSpecificBinaryUnverifiedOrigin,
    /// Script risk could not be proven and must be reviewed.
    ScriptRiskUnknownRequiresReview,
}

impl ScriptRiskClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoScriptNoNativeBuild => "no_script_no_native_build",
            Self::DeclarativeMetadataOnly => "declarative_metadata_only",
            Self::PostInstallScriptRunsInSandbox => "post_install_script_runs_in_sandbox",
            Self::PostInstallScriptRunsUnsandboxedUserConsentRequired => {
                "post_install_script_runs_unsandboxed_user_consent_required"
            }
            Self::NativeCompilationRequiredLocalToolchain => {
                "native_compilation_required_local_toolchain"
            }
            Self::PrebuiltBinaryWithRuntimeLoader => "prebuilt_binary_with_runtime_loader",
            Self::PlatformSpecificBinaryUnverifiedOrigin => {
                "platform_specific_binary_unverified_origin"
            }
            Self::ScriptRiskUnknownRequiresReview => "script_risk_unknown_requires_review",
        }
    }

    /// True when review cannot admit apply without an explicit decision.
    pub const fn requires_explicit_consent(self) -> bool {
        matches!(
            self,
            Self::PostInstallScriptRunsUnsandboxedUserConsentRequired
                | Self::NativeCompilationRequiredLocalToolchain
                | Self::PlatformSpecificBinaryUnverifiedOrigin
                | Self::ScriptRiskUnknownRequiresReview
        )
    }
}

/// Lockfile impact class re-exported from package-action schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfileImpactClass {
    /// No lockfile change; metadata-only operation.
    NoLockfileChangeMetadataOnly,
    /// New lockfile entries are expected.
    NewLockfileEntriesAdded,
    /// Existing lockfile entries are expected to update.
    ExistingLockfileEntriesUpdated,
    /// Lockfile entries are expected to be removed.
    LockfileEntriesRemoved,
    /// Resolver strategy changes.
    LockfileResolverStrategyChanged,
    /// Lockfile format migration.
    LockfileFormatMigration,
    /// Lockfile is pinned and unchanged for a query-only operation.
    LockfileLockedUnchangedPinnedQueryOnly,
    /// Lockfile is absent and will be created.
    LockfileAbsentWillBeCreated,
    /// Lockfile impact could not be proven and must be reviewed.
    LockfileImpactUnknownRequiresReview,
}

impl LockfileImpactClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoLockfileChangeMetadataOnly => "no_lockfile_change_metadata_only",
            Self::NewLockfileEntriesAdded => "new_lockfile_entries_added",
            Self::ExistingLockfileEntriesUpdated => "existing_lockfile_entries_updated",
            Self::LockfileEntriesRemoved => "lockfile_entries_removed",
            Self::LockfileResolverStrategyChanged => "lockfile_resolver_strategy_changed",
            Self::LockfileFormatMigration => "lockfile_format_migration",
            Self::LockfileLockedUnchangedPinnedQueryOnly => {
                "lockfile_locked_unchanged_pinned_query_only"
            }
            Self::LockfileAbsentWillBeCreated => "lockfile_absent_will_be_created",
            Self::LockfileImpactUnknownRequiresReview => "lockfile_impact_unknown_requires_review",
        }
    }
}

/// How the alpha lane will handle lockfile bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfileMutationMode {
    /// Round-trip-safe inline structured edit is proven.
    InlineEditRoundTripSafe,
    /// Package manager must regenerate and the user reviews the diff.
    RegenerateAndReview,
    /// The lane is read-only and only compares lockfile state.
    CompareOnlyInspect,
    /// The mode could not be proven and must be reviewed.
    BlockedUnknownRequiresReview,
}

impl LockfileMutationMode {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineEditRoundTripSafe => "inline_edit_round_trip_safe",
            Self::RegenerateAndReview => "regenerate_and_review",
            Self::CompareOnlyInspect => "compare_only_inspect",
            Self::BlockedUnknownRequiresReview => "blocked_unknown_requires_review",
        }
    }
}

/// Transitive impact class re-exported from package-action schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitiveImpactClass {
    /// No transitive change.
    NoTransitiveChange,
    /// Transitively added packages remain within a minor floor.
    TransitiveAddedWithinMinorFloor,
    /// A transitive crosses a major boundary.
    TransitiveAddedAcrossMajorBoundary,
    /// Transitive entries are removed.
    TransitiveRemoved,
    /// A transitive pinning pattern changes.
    TransitivePinnedPatternChanged,
    /// Resolver conflict requires user review.
    TransitiveResolverConflictUserReviewRequired,
    /// Circular dependency detected.
    TransitiveCircularDependencyDetected,
    /// Transitive impact could not be proven and must be reviewed.
    TransitiveImpactUnknownRequiresReview,
}

impl TransitiveImpactClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoTransitiveChange => "no_transitive_change",
            Self::TransitiveAddedWithinMinorFloor => "transitive_added_within_minor_floor",
            Self::TransitiveAddedAcrossMajorBoundary => "transitive_added_across_major_boundary",
            Self::TransitiveRemoved => "transitive_removed",
            Self::TransitivePinnedPatternChanged => "transitive_pinned_pattern_changed",
            Self::TransitiveResolverConflictUserReviewRequired => {
                "transitive_resolver_conflict_user_review_required"
            }
            Self::TransitiveCircularDependencyDetected => "transitive_circular_dependency_detected",
            Self::TransitiveImpactUnknownRequiresReview => {
                "transitive_impact_unknown_requires_review"
            }
        }
    }
}

/// Rollback posture class re-exported from package-action schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackPostureClass {
    /// Rollback restores a lockfile checkpoint.
    RollbackViaLockfileCheckpoint,
    /// Rollback restores a workspace snapshot checkpoint.
    RollbackViaWorkspaceSnapshotCheckpoint,
    /// Rollback requires re-resolving because inline checkpoint is absent.
    RollbackRequiresReResolveNoInlineCheckpoint,
    /// Native artifacts must be recompiled before rollback is complete.
    RollbackBlockedNativeArtifactsMustBeRecompiled,
    /// Unsandboxed post-install scripts make rollback incomplete.
    RollbackBlockedPostInstallScriptWasUnsandboxed,
    /// Destructive remove has no checkpoint.
    RollbackUnavailableDestructiveRemoveWithNoCheckpoint,
    /// Rollback is not applicable to a read-only operation.
    RollbackPostureNotApplicableReadOnly,
}

impl RollbackPostureClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RollbackViaLockfileCheckpoint => "rollback_via_lockfile_checkpoint",
            Self::RollbackViaWorkspaceSnapshotCheckpoint => {
                "rollback_via_workspace_snapshot_checkpoint"
            }
            Self::RollbackRequiresReResolveNoInlineCheckpoint => {
                "rollback_requires_re_resolve_no_inline_checkpoint"
            }
            Self::RollbackBlockedNativeArtifactsMustBeRecompiled => {
                "rollback_blocked_native_artifacts_must_be_recompiled"
            }
            Self::RollbackBlockedPostInstallScriptWasUnsandboxed => {
                "rollback_blocked_post_install_script_was_unsandboxed"
            }
            Self::RollbackUnavailableDestructiveRemoveWithNoCheckpoint => {
                "rollback_unavailable_destructive_remove_with_no_checkpoint"
            }
            Self::RollbackPostureNotApplicableReadOnly => {
                "rollback_posture_not_applicable_read_only"
            }
        }
    }
}

/// Validation task class re-exported from package-action schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationTaskClass {
    /// Type checking.
    Typecheck,
    /// Linting.
    Lint,
    /// Build.
    Build,
    /// Unit tests.
    UnitTest,
    /// Integration tests.
    IntegrationTest,
    /// License audit.
    LicenseAudit,
    /// Security audit.
    SecurityAudit,
    /// Dependency audit.
    DependencyAudit,
    /// Format check.
    FormatCheck,
}

impl ValidationTaskClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Typecheck => "typecheck",
            Self::Lint => "lint",
            Self::Build => "build",
            Self::UnitTest => "unit_test",
            Self::IntegrationTest => "integration_test",
            Self::LicenseAudit => "license_audit",
            Self::SecurityAudit => "security_audit",
            Self::DependencyAudit => "dependency_audit",
            Self::FormatCheck => "format_check",
        }
    }
}

/// Review outcome for a pre-apply package operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageReviewOutcomeClass {
    /// Review is admitted and apply is still pending.
    ReviewAdmittedApplyPending,
    /// Review is blocked until the user consents to script or native-build risk.
    ReviewBlockedPendingNativeBuildConsent,
    /// Review is blocked until lockfile resolution is renewed.
    ReviewBlockedPendingLockfileResolution,
    /// Review is blocked by registry/auth/policy.
    ReviewBlockedPendingPolicy,
    /// Review is blocked because the user must decide on a destructive action.
    ReviewBlockedPendingUserDecision,
    /// Review is read-only and reported without mutation.
    ReviewReportedReadOnly,
}

impl PackageReviewOutcomeClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewAdmittedApplyPending => "review_admitted_apply_pending",
            Self::ReviewBlockedPendingNativeBuildConsent => {
                "review_blocked_pending_native_build_consent"
            }
            Self::ReviewBlockedPendingLockfileResolution => {
                "review_blocked_pending_lockfile_resolution"
            }
            Self::ReviewBlockedPendingPolicy => "review_blocked_pending_policy",
            Self::ReviewBlockedPendingUserDecision => "review_blocked_pending_user_decision",
            Self::ReviewReportedReadOnly => "review_reported_read_only",
        }
    }

    /// True when apply remains blocked by review or policy.
    pub const fn blocks_apply(self) -> bool {
        !matches!(
            self,
            Self::ReviewAdmittedApplyPending | Self::ReviewReportedReadOnly
        )
    }
}

/// Audit result class for package-operation lineage packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageAuditResultClass {
    /// A pre-apply review packet was created.
    PreviewCreated,
    /// Apply was admitted but not yet executed.
    ApplyPendingUserConfirmation,
    /// Apply was blocked by review.
    ApplyBlockedByReview,
    /// Apply completed cleanly.
    ApplyCompletedClean,
    /// Rollback preview was created.
    RollbackPreviewCreated,
}

impl PackageAuditResultClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewCreated => "preview_created",
            Self::ApplyPendingUserConfirmation => "apply_pending_user_confirmation",
            Self::ApplyBlockedByReview => "apply_blocked_by_review",
            Self::ApplyCompletedClean => "apply_completed_clean",
            Self::RollbackPreviewCreated => "rollback_preview_created",
        }
    }
}

/// Redaction class for package-operation packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageRedactionClass {
    /// Metadata is safe by default.
    MetadataSafeDefault,
    /// Operator review is required before export.
    OperatorOnlyRestricted,
    /// Internal support review is required before export.
    InternalSupportRestricted,
    /// Signing evidence only.
    SigningEvidenceOnly,
}

impl PackageRedactionClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::InternalSupportRestricted => "internal_support_restricted",
            Self::SigningEvidenceOnly => "signing_evidence_only",
        }
    }
}

/// A typed validation issue surfaced on package-operation packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageOperationAlphaViolation {
    /// Manifest path or manifest locator is absent.
    ManifestScopeMissing,
    /// Manifest could not be read or parsed.
    ManifestUnavailable,
    /// Package manager could not be safely classified as npm or pnpm.
    PackageManagerUnknown,
    /// Registry source is unknown.
    RegistrySourceUnknown,
    /// Registry auth is unknown or unsupported.
    RegistryAuthBlocked,
    /// Raw registry secret was observed in workspace state.
    RawRegistrySecretObserved,
    /// Script or native-build risk is unknown or requires explicit consent.
    ScriptRiskRequiresConsent,
    /// Lockfile impact is unknown.
    LockfileImpactUnknown,
    /// Mutating operation has no rollback checkpoint.
    RollbackCheckpointMissing,
    /// One of the no-hidden-mutation guards is false.
    HiddenMutationGuardMissing,
}

impl PackageOperationAlphaViolation {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManifestScopeMissing => "manifest_scope_missing",
            Self::ManifestUnavailable => "manifest_unavailable",
            Self::PackageManagerUnknown => "package_manager_unknown",
            Self::RegistrySourceUnknown => "registry_source_unknown",
            Self::RegistryAuthBlocked => "registry_auth_blocked",
            Self::RawRegistrySecretObserved => "raw_registry_secret_observed",
            Self::ScriptRiskRequiresConsent => "script_risk_requires_consent",
            Self::LockfileImpactUnknown => "lockfile_impact_unknown",
            Self::RollbackCheckpointMissing => "rollback_checkpoint_missing",
            Self::HiddenMutationGuardMissing => "hidden_mutation_guard_missing",
        }
    }
}

/// Coupling between the active manifest and a lockfile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfileCouplingClass {
    /// A single workspace-level lockfile is shared by multiple manifests.
    SharedWorkspaceLockfile,
    /// Lockfile is colocated with the active manifest.
    ManifestLocalLockfile,
    /// No lockfile exists yet and apply would create one.
    LockfileAbsentWillBeCreated,
}

impl LockfileCouplingClass {
    /// Stable token used in schemas, fixtures, support exports, and docs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedWorkspaceLockfile => "shared_workspace_lockfile",
            Self::ManifestLocalLockfile => "manifest_local_lockfile",
            Self::LockfileAbsentWillBeCreated => "lockfile_absent_will_be_created",
        }
    }
}

/// One lockfile reference in a manifest-scope or impact descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockfileAlphaRef {
    /// Opaque lockfile ref safe for exports.
    pub lockfile_ref: String,
    /// Workspace-relative lockfile path shown in review.
    pub lockfile_path: String,
    /// How this lockfile is coupled to the active manifest.
    pub coupling_class: LockfileCouplingClass,
    /// Stable coupling token.
    pub coupling_token: String,
    /// True when the lockfile exists before apply.
    pub exists_before_apply: bool,
}

impl LockfileAlphaRef {
    fn new(path: impl Into<String>, coupling_class: LockfileCouplingClass, exists: bool) -> Self {
        let lockfile_path = path.into();
        Self {
            lockfile_ref: format!("lockfile:{}", stable_fragment(&lockfile_path)),
            lockfile_path,
            coupling_class,
            coupling_token: coupling_class.as_str().to_owned(),
            exists_before_apply: exists,
        }
    }
}

/// Manifest-scope descriptor shared by package review, audit, and export lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestScopeAlphaDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable manifest-scope descriptor id.
    pub manifest_scope_id: String,
    /// Package-manager family for this scope.
    pub package_manager_family: PackageManagerFamily,
    /// Stable package-manager token.
    pub package_manager_family_token: String,
    /// Manifest-scope class.
    pub manifest_scope_class: ManifestScopeClass,
    /// Stable manifest-scope token.
    pub manifest_scope_token: String,
    /// Opaque workspace root ref.
    pub workspace_root_ref: String,
    /// Opaque workspace member ref, when the active manifest is a member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_member_ref: Option<String>,
    /// Opaque module identity ref, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_identity_ref: Option<String>,
    /// Workspace-relative active manifest path shown before mutation.
    pub active_manifest_path: String,
    /// Opaque active manifest ref.
    pub active_manifest_ref: String,
    /// Registry source inherited by this manifest scope.
    pub inherited_registry_source_ref: String,
    /// Lockfiles coupled to this manifest scope.
    pub lockfile_refs: Vec<LockfileAlphaRef>,
    /// Opaque workspace slice ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_slice_ref: Option<String>,
}

impl ManifestScopeAlphaDescriptor {
    /// Creates a manifest-scope descriptor with stable display and ref fields.
    pub fn new(
        package_manager_family: PackageManagerFamily,
        manifest_scope_class: ManifestScopeClass,
        workspace_root_ref: impl Into<String>,
        workspace_member_ref: Option<String>,
        module_identity_ref: Option<String>,
        active_manifest_path: impl Into<String>,
        inherited_registry_source_ref: impl Into<String>,
        lockfile_refs: Vec<LockfileAlphaRef>,
    ) -> Self {
        let active_manifest_path = active_manifest_path.into();
        let workspace_root_ref = workspace_root_ref.into();
        let manifest_scope_id = format!(
            "manifest-scope:{}:{}",
            stable_fragment(&workspace_root_ref),
            stable_fragment(&active_manifest_path)
        );
        Self {
            record_kind: MANIFEST_SCOPE_ALPHA_RECORD_KIND.to_owned(),
            schema_version: PACKAGE_OPERATION_ALPHA_SCHEMA_VERSION,
            manifest_scope_id,
            package_manager_family,
            package_manager_family_token: package_manager_family.as_str().to_owned(),
            manifest_scope_class,
            manifest_scope_token: manifest_scope_class.as_str().to_owned(),
            workspace_root_ref,
            workspace_member_ref,
            module_identity_ref,
            active_manifest_ref: format!("manifest:{}", stable_fragment(&active_manifest_path)),
            active_manifest_path,
            inherited_registry_source_ref: inherited_registry_source_ref.into(),
            lockfile_refs,
            workspace_slice_ref: None,
        }
    }
}

/// Registry-source/auth descriptor shown before package mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistrySourceAlphaDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable registry-source descriptor id.
    pub registry_source_id: String,
    /// Package-manager family.
    pub package_manager_family: PackageManagerFamily,
    /// Stable package-manager token.
    pub package_manager_family_token: String,
    /// Registry source class.
    pub registry_source_class: RegistrySourceClass,
    /// Stable registry-source token.
    pub registry_source_token: String,
    /// Coarse source family for compact banners.
    pub source_family_token: String,
    /// Registry auth mode.
    pub auth_mode_class: RegistryAuthModeClass,
    /// Stable auth-mode token.
    pub auth_mode_token: String,
    /// Policy owner that governs this source, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_owner_ref: Option<String>,
    /// Secret-broker handle ref, when auth uses brokered credentials.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_broker_handle_ref: Option<String>,
    /// Delegated identity ref, when auth uses delegated identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegated_identity_ref: Option<String>,
    /// True when a raw workspace secret was observed.
    pub raw_secret_observed: bool,
    /// Freshness class for the source evidence.
    pub freshness_class: RegistryFreshnessClass,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Revocation or yank state.
    pub revocation_state_class: RegistryRevocationStateClass,
    /// Stable revocation token.
    pub revocation_state_token: String,
    /// Mirror/offline posture.
    pub mirror_or_offline_state_class: MirrorOrOfflineStateClass,
    /// Stable mirror/offline token.
    pub mirror_or_offline_state_token: String,
    /// Short review-safe disclosure sentence.
    pub disclosure: String,
    /// True because source is visible before write.
    pub visible_before_write: bool,
    /// True because source is visible before network.
    pub visible_before_network: bool,
}

impl RegistrySourceAlphaDescriptor {
    /// Creates a public default npm/pnpm registry descriptor with no auth.
    pub fn public_default(package_manager_family: PackageManagerFamily) -> Self {
        Self::new(
            "registry-source:public-default",
            package_manager_family,
            RegistrySourceClass::PublicDefaultRegistry,
            RegistryAuthModeClass::NoAuthPublicRegistry,
            RegistryFreshnessClass::AuthoritativeLive,
            RegistryRevocationStateClass::RevocationCheckedCurrent,
            MirrorOrOfflineStateClass::OnlineDefaultOriginAdmissible,
            "Public default registry; no auth required for review or apply.",
        )
    }

    /// Creates a registry-source descriptor from explicit reviewed fields.
    pub fn new(
        registry_source_id: impl Into<String>,
        package_manager_family: PackageManagerFamily,
        registry_source_class: RegistrySourceClass,
        auth_mode_class: RegistryAuthModeClass,
        freshness_class: RegistryFreshnessClass,
        revocation_state_class: RegistryRevocationStateClass,
        mirror_or_offline_state_class: MirrorOrOfflineStateClass,
        disclosure: impl Into<String>,
    ) -> Self {
        let registry_source_id = registry_source_id.into();
        Self {
            record_kind: REGISTRY_SOURCE_ALPHA_RECORD_KIND.to_owned(),
            schema_version: PACKAGE_OPERATION_ALPHA_SCHEMA_VERSION,
            registry_source_id,
            package_manager_family,
            package_manager_family_token: package_manager_family.as_str().to_owned(),
            registry_source_class,
            registry_source_token: registry_source_class.as_str().to_owned(),
            source_family_token: registry_source_class.source_family().to_owned(),
            auth_mode_class,
            auth_mode_token: auth_mode_class.as_str().to_owned(),
            policy_owner_ref: None,
            secret_broker_handle_ref: None,
            delegated_identity_ref: None,
            raw_secret_observed: false,
            freshness_class,
            freshness_token: freshness_class.as_str().to_owned(),
            revocation_state_class,
            revocation_state_token: revocation_state_class.as_str().to_owned(),
            mirror_or_offline_state_class,
            mirror_or_offline_state_token: mirror_or_offline_state_class.as_str().to_owned(),
            disclosure: disclosure.into(),
            visible_before_write: true,
            visible_before_network: true,
        }
    }
}

/// Script and native-build descriptor for a package operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptRiskAlphaDescriptor {
    /// Script/native-build risk class.
    pub script_risk_class: ScriptRiskClass,
    /// Stable script-risk token.
    pub script_risk_token: String,
    /// Opaque lifecycle script refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lifecycle_script_refs: Vec<String>,
    /// Opaque postinstall hook refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub postinstall_hook_refs: Vec<String>,
    /// Native toolchain ref when native compilation is expected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_toolchain_ref: Option<String>,
    /// Sandbox policy ref when scripts are sandboxed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sandbox_policy_ref: Option<String>,
    /// Consent ticket ref when consent has already been admitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_ticket_ref: Option<String>,
    /// Short review-safe disclosure sentence.
    pub disclosure: String,
}

impl ScriptRiskAlphaDescriptor {
    /// Creates a no-script descriptor for metadata-only package candidates.
    pub fn no_scripts() -> Self {
        Self::new(
            ScriptRiskClass::NoScriptNoNativeBuild,
            "No lifecycle scripts, postinstall hooks, or native build steps are expected.",
        )
    }

    /// Creates a script-risk descriptor from explicit reviewed fields.
    pub fn new(script_risk_class: ScriptRiskClass, disclosure: impl Into<String>) -> Self {
        Self {
            script_risk_class,
            script_risk_token: script_risk_class.as_str().to_owned(),
            lifecycle_script_refs: Vec::new(),
            postinstall_hook_refs: Vec::new(),
            native_toolchain_ref: None,
            sandbox_policy_ref: None,
            consent_ticket_ref: None,
            disclosure: disclosure.into(),
        }
    }
}

/// Resolver identity used for a lockfile-impact record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageResolverIdentity {
    /// Opaque resolver identity ref.
    pub resolver_identity_ref: String,
    /// Package-manager family.
    pub package_manager_family: PackageManagerFamily,
    /// Stable package-manager token.
    pub package_manager_family_token: String,
    /// Resolver version label or `unresolved`.
    pub resolver_version: String,
    /// Detector version that selected the resolver.
    pub detector_version: String,
}

impl PackageResolverIdentity {
    fn from_detection(
        package_manager_family: PackageManagerFamily,
        detection: &NodeToolchainDetection,
    ) -> Self {
        let resolver_version = detection
            .package_manager
            .version
            .clone()
            .unwrap_or_else(|| "unresolved".to_owned());
        Self {
            resolver_identity_ref: format!(
                "resolver:{}:{}",
                package_manager_family.as_str(),
                stable_fragment(&resolver_version)
            ),
            package_manager_family,
            package_manager_family_token: package_manager_family.as_str().to_owned(),
            resolver_version,
            detector_version: detection.detector_version.clone(),
        }
    }
}

/// Lockfile impact record carried by package-operation review packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockfileImpactAlphaRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable lockfile-impact id.
    pub lockfile_impact_id: String,
    /// Resolver identity.
    pub resolver_identity: PackageResolverIdentity,
    /// Lockfiles affected by this operation.
    pub affected_lockfiles: Vec<LockfileAlphaRef>,
    /// Lockfile impact class.
    pub lockfile_impact_class: LockfileImpactClass,
    /// Stable lockfile-impact token.
    pub lockfile_impact_token: String,
    /// How lockfile bytes will be handled.
    pub mutation_mode: LockfileMutationMode,
    /// Stable mutation-mode token.
    pub mutation_mode_token: String,
    /// Prior lockfile snapshot ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_lockfile_snapshot_ref: Option<String>,
    /// Proposed lockfile snapshot ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proposed_lockfile_snapshot_ref: Option<String>,
    /// Lockfile checkpoint ref created before apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lockfile_checkpoint_ref: Option<String>,
    /// Approximate count of added entries.
    pub added_entry_count_bucket: u32,
    /// Approximate count of updated entries.
    pub updated_entry_count_bucket: u32,
    /// Approximate count of removed entries.
    pub removed_entry_count_bucket: u32,
    /// Transitive impact class.
    pub transitive_impact_class: TransitiveImpactClass,
    /// Stable transitive-impact token.
    pub transitive_impact_token: String,
    /// Approximate count of transitive additions.
    pub transitive_added_count_bucket: u32,
    /// Approximate count of transitive removals.
    pub transitive_removed_count_bucket: u32,
    /// Approximate count of major-boundary transitive changes.
    pub transitive_major_boundary_count_bucket: u32,
    /// Approximate count of resolver conflicts.
    pub transitive_conflict_count_bucket: u32,
    /// Generated-artifact posture token for lockfile review.
    pub generated_artifact_posture_token: String,
    /// Opaque operation audit refs joined to this lockfile record.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operation_audit_refs: Vec<String>,
    /// Short review-safe disclosure sentence.
    pub disclosure: String,
}

/// Manifest diff summary without raw manifest body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestDiffAlphaSummary {
    /// Dependency section targeted by the operation.
    pub dependency_section: DependencySection,
    /// Stable dependency-section token.
    pub dependency_section_token: String,
    /// Manifest delta class.
    pub manifest_delta_class: ManifestDeltaClass,
    /// Stable manifest-delta token.
    pub manifest_delta_token: String,
    /// Opaque package coordinate ref.
    pub package_coordinate_ref: String,
    /// State of the requirement in the active manifest.
    pub current_requirement_state: ManifestRequirementState,
    /// Stable requirement-state token.
    pub current_requirement_state_token: String,
    /// True when the preview has enough information to stage a manifest diff.
    pub manifest_diff_precomputed: bool,
    /// Short review-safe disclosure sentence.
    pub disclosure: String,
}

/// Rollback/checkpoint summary for a package operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackCheckpointAlphaSummary {
    /// Rollback posture class.
    pub rollback_posture_class: RollbackPostureClass,
    /// Stable rollback-posture token.
    pub rollback_posture_token: String,
    /// Lockfile checkpoint ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lockfile_checkpoint_ref: Option<String>,
    /// Workspace snapshot checkpoint ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_snapshot_checkpoint_ref: Option<String>,
    /// Cache checkpoint ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_checkpoint_ref: Option<String>,
    /// True when a checkpoint was created before apply.
    pub checkpoint_created_before_apply: bool,
    /// Short review-safe disclosure sentence.
    pub disclosure: String,
}

/// No-hidden-mutation guard set carried by every package operation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageOperationNoHiddenMutationGuards {
    /// Lockfile diff or regenerate-and-review posture was computed.
    pub lockfile_impact_declared: bool,
    /// Package cache write posture was declared.
    pub package_cache_writes_declared: bool,
    /// Sidecar downloads were declared.
    pub sidecar_downloads_declared: bool,
    /// Workspace scripts were declared separately from package hooks.
    pub workspace_scripts_declared: bool,
    /// Registry source/remapping was declared.
    pub registry_source_declared: bool,
    /// Auth source was declared.
    pub auth_source_declared: bool,
    /// Script/native-build risk was declared.
    pub script_risk_declared: bool,
    /// Target context was bound before any network action.
    pub target_context_bound_before_network: bool,
    /// Support and automation can compare this packet across surfaces.
    pub cross_surface_comparison_ready: bool,
}

impl PackageOperationNoHiddenMutationGuards {
    /// Creates the all-true guard set required for admitted review packets.
    pub const fn all_declared() -> Self {
        Self {
            lockfile_impact_declared: true,
            package_cache_writes_declared: true,
            sidecar_downloads_declared: true,
            workspace_scripts_declared: true,
            registry_source_declared: true,
            auth_source_declared: true,
            script_risk_declared: true,
            target_context_bound_before_network: true,
            cross_surface_comparison_ready: true,
        }
    }

    fn all_true(&self) -> bool {
        self.lockfile_impact_declared
            && self.package_cache_writes_declared
            && self.sidecar_downloads_declared
            && self.workspace_scripts_declared
            && self.registry_source_declared
            && self.auth_source_declared
            && self.script_risk_declared
            && self.target_context_bound_before_network
            && self.cross_surface_comparison_ready
    }
}

/// Audit lineage refs copied into package-operation review packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageOperationAuditLineage {
    /// Actor ref that requested or admitted the operation.
    pub actor_ref: String,
    /// Command id that requested or admitted the operation.
    pub command_id_ref: String,
    /// Issuing surface token.
    pub issuing_surface: String,
    /// Execution-context ref.
    pub execution_context_ref: String,
    /// Target identity ref.
    pub target_identity_ref: String,
    /// Workspace scope ref.
    pub workspace_scope_ref: String,
    /// Policy epoch ref.
    pub policy_epoch_ref: String,
    /// Approval ticket refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub approval_ticket_refs: Vec<String>,
    /// Audit event refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audit_event_refs: Vec<String>,
    /// Support export refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub support_export_refs: Vec<String>,
}

/// Complete pre-apply package-operation alpha packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageOperationAlphaPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Runtime implementation version.
    pub reviewer_version: String,
    /// Stable operation id.
    pub package_operation_id: String,
    /// Package operation class.
    pub operation_class: PackageOperationClass,
    /// Stable operation token.
    pub operation_token: String,
    /// Package-manager family.
    pub package_manager_family: PackageManagerFamily,
    /// Stable package-manager token.
    pub package_manager_family_token: String,
    /// Timestamp supplied by the caller.
    pub reviewed_at: String,
    /// Manifest-scope descriptor.
    pub manifest_scope: ManifestScopeAlphaDescriptor,
    /// Registry-source/auth descriptor.
    pub registry_source: RegistrySourceAlphaDescriptor,
    /// Script/native-build descriptor.
    pub script_risk: ScriptRiskAlphaDescriptor,
    /// Manifest diff summary.
    pub manifest_diff: ManifestDiffAlphaSummary,
    /// Lockfile-impact record.
    pub lockfile_impact: LockfileImpactAlphaRecord,
    /// Rollback/checkpoint summary.
    pub rollback: RollbackCheckpointAlphaSummary,
    /// Validation tasks proposed after apply.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_tasks: Vec<ValidationTaskClass>,
    /// Stable validation task tokens.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_task_tokens: Vec<String>,
    /// No-hidden-mutation guard set.
    pub no_hidden_mutation_guards: PackageOperationNoHiddenMutationGuards,
    /// Review outcome.
    pub review_outcome_class: PackageReviewOutcomeClass,
    /// Stable review-outcome token.
    pub review_outcome_token: String,
    /// Typed blocked reasons.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocked_reason_tokens: Vec<String>,
    /// Audit lineage.
    pub audit_lineage: PackageOperationAuditLineage,
    /// Redaction class for this packet.
    pub redaction_class: PackageRedactionClass,
    /// Stable redaction token.
    pub redaction_token: String,
    /// True when packet omits raw manifests, lockfiles, URLs, script bodies, and secret material.
    pub export_safe: bool,
}

impl PackageOperationAlphaPacket {
    /// Returns typed validation issues for package review and tests.
    pub fn validation_issues(&self) -> Vec<PackageOperationAlphaViolation> {
        let mut issues = Vec::new();
        if self.manifest_scope.active_manifest_path.is_empty()
            || self.manifest_scope.active_manifest_ref.is_empty()
        {
            issues.push(PackageOperationAlphaViolation::ManifestScopeMissing);
        }
        if self.manifest_diff.current_requirement_state
            == ManifestRequirementState::UnknownManifestUnavailable
        {
            issues.push(PackageOperationAlphaViolation::ManifestUnavailable);
        }
        if self.package_manager_family == PackageManagerFamily::PackageManagerUnknownRequiresReview
        {
            issues.push(PackageOperationAlphaViolation::PackageManagerUnknown);
        }
        if self.registry_source.registry_source_class
            == RegistrySourceClass::RegistrySourceUnknownRequiresReview
        {
            issues.push(PackageOperationAlphaViolation::RegistrySourceUnknown);
        }
        if self.registry_source.auth_mode_class.blocks_mutation()
            && self.operation_class.is_mutating()
        {
            issues.push(PackageOperationAlphaViolation::RegistryAuthBlocked);
        }
        if self.registry_source.raw_secret_observed && self.operation_class.is_mutating() {
            issues.push(PackageOperationAlphaViolation::RawRegistrySecretObserved);
        }
        if self
            .script_risk
            .script_risk_class
            .requires_explicit_consent()
            && self.script_risk.consent_ticket_ref.is_none()
        {
            issues.push(PackageOperationAlphaViolation::ScriptRiskRequiresConsent);
        }
        if self.lockfile_impact.lockfile_impact_class
            == LockfileImpactClass::LockfileImpactUnknownRequiresReview
        {
            issues.push(PackageOperationAlphaViolation::LockfileImpactUnknown);
        }
        if self.operation_class.is_mutating()
            && matches!(
                self.rollback.rollback_posture_class,
                RollbackPostureClass::RollbackViaLockfileCheckpoint
                    | RollbackPostureClass::RollbackViaWorkspaceSnapshotCheckpoint
            )
            && !self.rollback.checkpoint_created_before_apply
        {
            issues.push(PackageOperationAlphaViolation::RollbackCheckpointMissing);
        }
        if !self.no_hidden_mutation_guards.all_true() {
            issues.push(PackageOperationAlphaViolation::HiddenMutationGuardMissing);
        }
        issues
    }

    /// True when the packet blocks apply pending review, consent, or policy.
    pub fn blocks_apply(&self) -> bool {
        self.review_outcome_class.blocks_apply() || !self.validation_issues().is_empty()
    }

    /// Creates an audit packet linked to this package-operation review.
    pub fn audit_packet(
        &self,
        audit_packet_id: impl Into<String>,
        result_class: PackageAuditResultClass,
        recorded_at: impl Into<String>,
    ) -> PackageOperationAuditPacket {
        PackageOperationAuditPacket {
            record_kind: PACKAGE_OPERATION_AUDIT_RECORD_KIND.to_owned(),
            schema_version: PACKAGE_OPERATION_ALPHA_SCHEMA_VERSION,
            audit_packet_id: audit_packet_id.into(),
            package_operation_ref: self.package_operation_id.clone(),
            actor_ref: self.audit_lineage.actor_ref.clone(),
            command_id_ref: self.audit_lineage.command_id_ref.clone(),
            manifest_scope_ref: self.manifest_scope.manifest_scope_id.clone(),
            registry_source_ref: self.registry_source.registry_source_id.clone(),
            lockfile_impact_ref: self.lockfile_impact.lockfile_impact_id.clone(),
            result_class,
            result_token: result_class.as_str().to_owned(),
            review_outcome_class: self.review_outcome_class,
            review_outcome_token: self.review_outcome_class.as_str().to_owned(),
            rollback_checkpoint_refs: self.rollback_checkpoint_refs(),
            recorded_at: recorded_at.into(),
            redaction_class: self.redaction_class,
            redaction_token: self.redaction_class.as_str().to_owned(),
            export_safe: true,
        }
    }

    fn rollback_checkpoint_refs(&self) -> Vec<String> {
        [
            self.rollback.lockfile_checkpoint_ref.clone(),
            self.rollback.workspace_snapshot_checkpoint_ref.clone(),
            self.rollback.cache_checkpoint_ref.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

/// Export-safe audit packet for a package operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageOperationAuditPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable audit packet id.
    pub audit_packet_id: String,
    /// Package-operation packet ref.
    pub package_operation_ref: String,
    /// Actor ref.
    pub actor_ref: String,
    /// Command id ref.
    pub command_id_ref: String,
    /// Manifest-scope descriptor ref.
    pub manifest_scope_ref: String,
    /// Registry-source descriptor ref.
    pub registry_source_ref: String,
    /// Lockfile-impact record ref.
    pub lockfile_impact_ref: String,
    /// Audit result class.
    pub result_class: PackageAuditResultClass,
    /// Stable audit result token.
    pub result_token: String,
    /// Review outcome copied from the package operation.
    pub review_outcome_class: PackageReviewOutcomeClass,
    /// Stable review outcome token.
    pub review_outcome_token: String,
    /// Rollback checkpoint refs available for recovery.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rollback_checkpoint_refs: Vec<String>,
    /// Timestamp supplied by the caller.
    pub recorded_at: String,
    /// Redaction class.
    pub redaction_class: PackageRedactionClass,
    /// Stable redaction token.
    pub redaction_token: String,
    /// True because the audit packet carries refs and enums only.
    pub export_safe: bool,
}

/// One metadata-only package operation row for support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageOperationSupportExportRow {
    /// Package-operation packet ref.
    pub package_operation_ref: String,
    /// Operation token.
    pub operation_token: String,
    /// Package-manager token.
    pub package_manager_family_token: String,
    /// Actor ref.
    pub actor_ref: String,
    /// Command id ref.
    pub command_id_ref: String,
    /// Active manifest path shown by the review.
    pub active_manifest_path: String,
    /// Manifest scope token.
    pub manifest_scope_token: String,
    /// Registry source token.
    pub registry_source_token: String,
    /// Registry auth token.
    pub auth_mode_token: String,
    /// Script-risk token.
    pub script_risk_token: String,
    /// Lockfile impact token.
    pub lockfile_impact_token: String,
    /// Lockfile mutation mode token.
    pub lockfile_mutation_mode_token: String,
    /// Rollback posture token.
    pub rollback_posture_token: String,
    /// Review outcome token.
    pub review_outcome_token: String,
    /// True because the row omits raw manifests, lockfiles, registry URLs, and secrets.
    pub raw_payloads_exported: bool,
}

/// Metadata-only support export for package-operation reviews.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageOperationSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub support_export_id: String,
    /// Timestamp supplied by the caller.
    pub generated_at: String,
    /// Export rows.
    pub rows: Vec<PackageOperationSupportExportRow>,
    /// True when every row omits raw package payloads.
    pub redaction_safe: bool,
}

impl PackageOperationSupportExport {
    /// Builds a support export from reviewed package-operation packets.
    pub fn from_packets(
        support_export_id: impl Into<String>,
        generated_at: impl Into<String>,
        packets: &[PackageOperationAlphaPacket],
    ) -> Self {
        let rows = packets
            .iter()
            .map(|packet| PackageOperationSupportExportRow {
                package_operation_ref: packet.package_operation_id.clone(),
                operation_token: packet.operation_token.clone(),
                package_manager_family_token: packet.package_manager_family_token.clone(),
                actor_ref: packet.audit_lineage.actor_ref.clone(),
                command_id_ref: packet.audit_lineage.command_id_ref.clone(),
                active_manifest_path: packet.manifest_scope.active_manifest_path.clone(),
                manifest_scope_token: packet.manifest_scope.manifest_scope_token.clone(),
                registry_source_token: packet.registry_source.registry_source_token.clone(),
                auth_mode_token: packet.registry_source.auth_mode_token.clone(),
                script_risk_token: packet.script_risk.script_risk_token.clone(),
                lockfile_impact_token: packet.lockfile_impact.lockfile_impact_token.clone(),
                lockfile_mutation_mode_token: packet.lockfile_impact.mutation_mode_token.clone(),
                rollback_posture_token: packet.rollback.rollback_posture_token.clone(),
                review_outcome_token: packet.review_outcome_token.clone(),
                raw_payloads_exported: false,
            })
            .collect::<Vec<_>>();
        Self {
            record_kind: PACKAGE_OPERATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PACKAGE_OPERATION_ALPHA_SCHEMA_VERSION,
            support_export_id: support_export_id.into(),
            generated_at: generated_at.into(),
            redaction_safe: rows.iter().all(|row| !row.raw_payloads_exported),
            rows,
        }
    }

    /// Renders a compact text summary for diagnostics and tests.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!(
            "package_operation_support_export {} rows={}\n",
            self.support_export_id,
            self.rows.len()
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{} {} {} {} {} {}\n",
                row.package_operation_ref,
                row.operation_token,
                row.active_manifest_path,
                row.registry_source_token,
                row.script_risk_token,
                row.review_outcome_token
            ));
        }
        out
    }
}

/// Configuration for [`NodePackageMutationReviewer`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodePackageMutationReviewerConfig {
    /// Node detector configuration used to select npm or pnpm.
    pub node_detector: NodeToolchainDetectorConfig,
    /// Opaque workspace id.
    pub workspace_id: String,
    /// Opaque workspace root ref used in emitted records.
    pub workspace_root_ref: String,
    /// Execution-context ref already resolved for this action.
    pub execution_context_ref: String,
    /// Target identity ref already resolved for this action.
    pub target_identity_ref: String,
    /// Workspace scope ref already resolved for this action.
    pub workspace_scope_ref: String,
}

impl Default for NodePackageMutationReviewerConfig {
    fn default() -> Self {
        Self {
            node_detector: NodeToolchainDetectorConfig::default(),
            workspace_id: "workspace:default".to_owned(),
            workspace_root_ref: "workspace:root".to_owned(),
            execution_context_ref: "execution-context:package-review".to_owned(),
            target_identity_ref: "target:local-host".to_owned(),
            workspace_scope_ref: "workspace-scope:current-root".to_owned(),
        }
    }
}

/// Request for one Node package-mutation preview.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodePackageMutationReviewRequest {
    /// Package operation class.
    pub operation_class: PackageOperationClass,
    /// Dependency section to edit.
    pub dependency_section: DependencySection,
    /// Package name used only to inspect the manifest; emitted packets carry the coordinate ref.
    pub package_name: String,
    /// Opaque package coordinate ref emitted in review packets.
    pub package_coordinate_ref: String,
    /// Requested requirement token used for local comparison.
    pub requested_requirement: String,
    /// Workspace-relative active manifest path.
    pub active_manifest_path: String,
    /// Manifest scope class.
    pub manifest_scope_class: ManifestScopeClass,
    /// Workspace member ref, when the active manifest is a member.
    pub workspace_member_ref: Option<String>,
    /// Module identity ref, when known.
    pub module_identity_ref: Option<String>,
    /// Actor ref.
    pub actor_ref: String,
    /// Command id ref.
    pub command_id_ref: String,
    /// Issuing surface token.
    pub issuing_surface: String,
    /// Policy epoch ref.
    pub policy_epoch_ref: String,
    /// Registry-source/auth descriptor reviewed before apply.
    pub registry_source: RegistrySourceAlphaDescriptor,
    /// Script/native-build descriptor reviewed before apply.
    pub script_risk: ScriptRiskAlphaDescriptor,
    /// Validation tasks proposed after apply.
    pub validation_tasks: Vec<ValidationTaskClass>,
}

/// Read-only reviewer for TS/JS package mutation previews.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodePackageMutationReviewer {
    config: NodePackageMutationReviewerConfig,
}

impl NodePackageMutationReviewer {
    /// Creates a reviewer with explicit context refs and detector config.
    pub fn new(config: NodePackageMutationReviewerConfig) -> Self {
        Self { config }
    }

    /// Creates a reviewer with default context refs and no ambient detector facts.
    pub fn default_read_only() -> Self {
        Self::new(NodePackageMutationReviewerConfig::default())
    }

    /// Reviews one Node package mutation without writing manifests or lockfiles.
    pub fn review_workspace(
        &self,
        workspace_root: &Path,
        request: NodePackageMutationReviewRequest,
        reviewed_at: &str,
    ) -> PackageOperationAlphaPacket {
        let detection = NodeToolchainDetector::new(self.config.node_detector.clone())
            .detect_workspace(workspace_root, reviewed_at);
        let package_manager_family = PackageManagerFamily::from_node_detection(&detection);
        let mut registry_source = request.registry_source.clone();
        registry_source.package_manager_family = package_manager_family;
        registry_source.package_manager_family_token = package_manager_family.as_str().to_owned();

        let manifest_path = request.active_manifest_path.clone();
        let manifest_value = read_manifest_json(workspace_root, &manifest_path);
        let current_requirement_state = manifest_requirement_state(
            manifest_value.as_ref(),
            request.dependency_section,
            &request.package_name,
            &request.requested_requirement,
        );
        let manifest_delta_class = manifest_delta_for(
            request.operation_class,
            current_requirement_state,
            manifest_value.is_some(),
        );

        let lockfile_refs = detect_lockfiles(workspace_root, package_manager_family);
        let manifest_scope = ManifestScopeAlphaDescriptor::new(
            package_manager_family,
            request.manifest_scope_class,
            self.config.workspace_root_ref.clone(),
            request.workspace_member_ref.clone(),
            request.module_identity_ref.clone(),
            manifest_path.clone(),
            registry_source.registry_source_id.clone(),
            lockfile_refs.clone(),
        );
        let manifest_diff = ManifestDiffAlphaSummary {
            dependency_section: request.dependency_section,
            dependency_section_token: request.dependency_section.as_str().to_owned(),
            manifest_delta_class,
            manifest_delta_token: manifest_delta_class.as_str().to_owned(),
            package_coordinate_ref: request.package_coordinate_ref.clone(),
            current_requirement_state,
            current_requirement_state_token: current_requirement_state.as_str().to_owned(),
            manifest_diff_precomputed: manifest_value.is_some(),
            disclosure: manifest_diff_disclosure(
                manifest_delta_class,
                request.dependency_section,
                &manifest_path,
            ),
        };
        let lockfile_impact = lockfile_impact_for(
            package_manager_family,
            request.operation_class,
            manifest_delta_class,
            &detection,
            &manifest_scope,
            &lockfile_refs,
        );
        let rollback = rollback_for(
            request.operation_class,
            request.script_risk.script_risk_class,
            &lockfile_impact,
        );
        let validation_task_tokens = request
            .validation_tasks
            .iter()
            .map(|task| task.as_str().to_owned())
            .collect::<Vec<_>>();
        let mut packet = PackageOperationAlphaPacket {
            record_kind: PACKAGE_OPERATION_ALPHA_RECORD_KIND.to_owned(),
            schema_version: PACKAGE_OPERATION_ALPHA_SCHEMA_VERSION,
            reviewer_version: PACKAGE_MUTATION_REVIEWER_VERSION.to_owned(),
            package_operation_id: format!(
                "package-operation:{}:{}:{}",
                request.operation_class.as_str(),
                stable_fragment(&manifest_path),
                stable_fragment(&request.package_coordinate_ref)
            ),
            operation_class: request.operation_class,
            operation_token: request.operation_class.as_str().to_owned(),
            package_manager_family,
            package_manager_family_token: package_manager_family.as_str().to_owned(),
            reviewed_at: reviewed_at.to_owned(),
            manifest_scope,
            registry_source,
            script_risk: request.script_risk,
            manifest_diff,
            lockfile_impact,
            rollback,
            validation_tasks: request.validation_tasks,
            validation_task_tokens,
            no_hidden_mutation_guards: PackageOperationNoHiddenMutationGuards::all_declared(),
            review_outcome_class: PackageReviewOutcomeClass::ReviewAdmittedApplyPending,
            review_outcome_token: PackageReviewOutcomeClass::ReviewAdmittedApplyPending
                .as_str()
                .to_owned(),
            blocked_reason_tokens: Vec::new(),
            audit_lineage: PackageOperationAuditLineage {
                actor_ref: request.actor_ref,
                command_id_ref: request.command_id_ref,
                issuing_surface: request.issuing_surface,
                execution_context_ref: self.config.execution_context_ref.clone(),
                target_identity_ref: self.config.target_identity_ref.clone(),
                workspace_scope_ref: self.config.workspace_scope_ref.clone(),
                policy_epoch_ref: request.policy_epoch_ref,
                approval_ticket_refs: Vec::new(),
                audit_event_refs: Vec::new(),
                support_export_refs: Vec::new(),
            },
            redaction_class: PackageRedactionClass::MetadataSafeDefault,
            redaction_token: PackageRedactionClass::MetadataSafeDefault
                .as_str()
                .to_owned(),
            export_safe: true,
        };
        let issues = packet.validation_issues();
        packet.blocked_reason_tokens = issues
            .iter()
            .map(|issue| issue.as_str().to_owned())
            .collect();
        packet.review_outcome_class = review_outcome_for(&packet, &issues);
        packet.review_outcome_token = packet.review_outcome_class.as_str().to_owned();
        packet
    }
}

fn read_manifest_json(workspace_root: &Path, manifest_path: &str) -> Option<Value> {
    let path = workspace_root.join(manifest_path);
    let payload = fs::read_to_string(path).ok()?;
    serde_json::from_str(&payload).ok()
}

fn manifest_requirement_state(
    manifest: Option<&Value>,
    dependency_section: DependencySection,
    package_name: &str,
    requested_requirement: &str,
) -> ManifestRequirementState {
    let Some(manifest) = manifest else {
        return ManifestRequirementState::UnknownManifestUnavailable;
    };
    let Some(section) = manifest.get(dependency_section.package_json_key()) else {
        return ManifestRequirementState::Absent;
    };
    let Some(requirement) = section.get(package_name).and_then(Value::as_str) else {
        return ManifestRequirementState::Absent;
    };
    if requirement == requested_requirement {
        ManifestRequirementState::PresentSameRequirement
    } else {
        ManifestRequirementState::PresentDifferentRequirement
    }
}

fn manifest_delta_for(
    operation_class: PackageOperationClass,
    requirement_state: ManifestRequirementState,
    manifest_available: bool,
) -> ManifestDeltaClass {
    if !manifest_available {
        return ManifestDeltaClass::ManifestDeltaUnknownRequiresReview;
    }
    match operation_class {
        PackageOperationClass::AuditOnlyNoStateChange => {
            ManifestDeltaClass::ManifestUnchangedAuditOnly
        }
        PackageOperationClass::InstallNewDependency => match requirement_state {
            ManifestRequirementState::Absent => ManifestDeltaClass::DependencyEntryAdded,
            ManifestRequirementState::PresentSameRequirement => {
                ManifestDeltaClass::ManifestUnchangedAuditOnly
            }
            ManifestRequirementState::PresentDifferentRequirement => {
                ManifestDeltaClass::DependencyEntryUpdated
            }
            ManifestRequirementState::UnknownManifestUnavailable => {
                ManifestDeltaClass::ManifestDeltaUnknownRequiresReview
            }
        },
        PackageOperationClass::UpgradeExistingDependency => {
            ManifestDeltaClass::DependencyEntryUpdated
        }
        PackageOperationClass::RemoveExistingDependency => {
            ManifestDeltaClass::DependencyEntryRemoved
        }
        PackageOperationClass::RestoreLockfileToCheckpoint => {
            ManifestDeltaClass::ManifestUnchangedAuditOnly
        }
    }
}

fn detect_lockfiles(
    workspace_root: &Path,
    package_manager_family: PackageManagerFamily,
) -> Vec<LockfileAlphaRef> {
    let mut refs = Vec::new();
    match package_manager_family {
        PackageManagerFamily::Pnpm => {
            push_lockfile_if_present(workspace_root, &mut refs, "pnpm-lock.yaml");
        }
        PackageManagerFamily::Npm => {
            push_lockfile_if_present(workspace_root, &mut refs, "package-lock.json");
            push_lockfile_if_present(workspace_root, &mut refs, "npm-shrinkwrap.json");
        }
        PackageManagerFamily::PackageManagerUnknownRequiresReview => {
            for path in ["pnpm-lock.yaml", "package-lock.json", "npm-shrinkwrap.json"] {
                push_lockfile_if_present(workspace_root, &mut refs, path);
            }
        }
    }
    if refs.is_empty() {
        refs.push(LockfileAlphaRef::new(
            lockfile_default_path(package_manager_family),
            LockfileCouplingClass::LockfileAbsentWillBeCreated,
            false,
        ));
    }
    refs
}

fn push_lockfile_if_present(workspace_root: &Path, refs: &mut Vec<LockfileAlphaRef>, path: &str) {
    if workspace_root.join(path).is_file() {
        refs.push(LockfileAlphaRef::new(
            path,
            LockfileCouplingClass::SharedWorkspaceLockfile,
            true,
        ));
    }
}

fn lockfile_default_path(package_manager_family: PackageManagerFamily) -> &'static str {
    match package_manager_family {
        PackageManagerFamily::Pnpm => "pnpm-lock.yaml",
        PackageManagerFamily::Npm | PackageManagerFamily::PackageManagerUnknownRequiresReview => {
            "package-lock.json"
        }
    }
}

fn lockfile_impact_for(
    package_manager_family: PackageManagerFamily,
    operation_class: PackageOperationClass,
    manifest_delta_class: ManifestDeltaClass,
    detection: &NodeToolchainDetection,
    manifest_scope: &ManifestScopeAlphaDescriptor,
    lockfile_refs: &[LockfileAlphaRef],
) -> LockfileImpactAlphaRecord {
    let resolver_identity =
        PackageResolverIdentity::from_detection(package_manager_family, detection);
    let all_absent = lockfile_refs
        .iter()
        .all(|lockfile| !lockfile.exists_before_apply);
    let lockfile_impact_class = if operation_class == PackageOperationClass::AuditOnlyNoStateChange
    {
        LockfileImpactClass::NoLockfileChangeMetadataOnly
    } else if all_absent {
        LockfileImpactClass::LockfileAbsentWillBeCreated
    } else {
        match manifest_delta_class {
            ManifestDeltaClass::DependencyEntryAdded => {
                LockfileImpactClass::NewLockfileEntriesAdded
            }
            ManifestDeltaClass::DependencyEntryUpdated => {
                LockfileImpactClass::ExistingLockfileEntriesUpdated
            }
            ManifestDeltaClass::DependencyEntryRemoved => {
                LockfileImpactClass::LockfileEntriesRemoved
            }
            ManifestDeltaClass::ManifestUnchangedAuditOnly => {
                LockfileImpactClass::LockfileLockedUnchangedPinnedQueryOnly
            }
            ManifestDeltaClass::ManifestDeltaUnknownRequiresReview => {
                LockfileImpactClass::LockfileImpactUnknownRequiresReview
            }
        }
    };
    let mutation_mode = if operation_class == PackageOperationClass::AuditOnlyNoStateChange {
        LockfileMutationMode::CompareOnlyInspect
    } else if lockfile_impact_class == LockfileImpactClass::LockfileImpactUnknownRequiresReview {
        LockfileMutationMode::BlockedUnknownRequiresReview
    } else {
        LockfileMutationMode::RegenerateAndReview
    };
    let primary = lockfile_refs
        .first()
        .map(|lockfile| stable_fragment(&lockfile.lockfile_path))
        .unwrap_or_else(|| "lockfile_unknown".to_owned());
    let lockfile_checkpoint_ref = if operation_class.is_mutating() {
        Some(format!("lockfile-checkpoint:{}:pre-apply", primary))
    } else {
        None
    };
    LockfileImpactAlphaRecord {
        record_kind: LOCKFILE_IMPACT_ALPHA_RECORD_KIND.to_owned(),
        schema_version: PACKAGE_OPERATION_ALPHA_SCHEMA_VERSION,
        lockfile_impact_id: format!(
            "lockfile-impact:{}:{}",
            manifest_scope.manifest_scope_id, primary
        ),
        resolver_identity,
        affected_lockfiles: lockfile_refs.to_vec(),
        lockfile_impact_class,
        lockfile_impact_token: lockfile_impact_class.as_str().to_owned(),
        mutation_mode,
        mutation_mode_token: mutation_mode.as_str().to_owned(),
        prior_lockfile_snapshot_ref: if operation_class.is_mutating() {
            Some(format!("lockfile-snapshot:{}:prior", primary))
        } else {
            None
        },
        proposed_lockfile_snapshot_ref: if operation_class.is_mutating() {
            Some(format!("lockfile-snapshot:{}:proposed", primary))
        } else {
            None
        },
        lockfile_checkpoint_ref,
        added_entry_count_bucket: match lockfile_impact_class {
            LockfileImpactClass::NewLockfileEntriesAdded
            | LockfileImpactClass::LockfileAbsentWillBeCreated => 1,
            _ => 0,
        },
        updated_entry_count_bucket: u32::from(
            lockfile_impact_class == LockfileImpactClass::ExistingLockfileEntriesUpdated,
        ),
        removed_entry_count_bucket: u32::from(
            lockfile_impact_class == LockfileImpactClass::LockfileEntriesRemoved,
        ),
        transitive_impact_class: if operation_class.is_mutating() {
            TransitiveImpactClass::TransitiveImpactUnknownRequiresReview
        } else {
            TransitiveImpactClass::NoTransitiveChange
        },
        transitive_impact_token: if operation_class.is_mutating() {
            TransitiveImpactClass::TransitiveImpactUnknownRequiresReview
        } else {
            TransitiveImpactClass::NoTransitiveChange
        }
        .as_str()
        .to_owned(),
        transitive_added_count_bucket: 0,
        transitive_removed_count_bucket: 0,
        transitive_major_boundary_count_bucket: 0,
        transitive_conflict_count_bucket: 0,
        generated_artifact_posture_token: "lockfile_only_generated_artifact".to_owned(),
        operation_audit_refs: Vec::new(),
        disclosure: lockfile_disclosure(lockfile_impact_class, mutation_mode),
    }
}

fn rollback_for(
    operation_class: PackageOperationClass,
    script_risk_class: ScriptRiskClass,
    lockfile_impact: &LockfileImpactAlphaRecord,
) -> RollbackCheckpointAlphaSummary {
    let rollback_posture_class = if operation_class == PackageOperationClass::AuditOnlyNoStateChange
    {
        RollbackPostureClass::RollbackPostureNotApplicableReadOnly
    } else if script_risk_class
        == ScriptRiskClass::PostInstallScriptRunsUnsandboxedUserConsentRequired
    {
        RollbackPostureClass::RollbackBlockedPostInstallScriptWasUnsandboxed
    } else if script_risk_class == ScriptRiskClass::NativeCompilationRequiredLocalToolchain {
        RollbackPostureClass::RollbackBlockedNativeArtifactsMustBeRecompiled
    } else if lockfile_impact.lockfile_checkpoint_ref.is_some() {
        RollbackPostureClass::RollbackViaLockfileCheckpoint
    } else {
        RollbackPostureClass::RollbackRequiresReResolveNoInlineCheckpoint
    };
    let lockfile_checkpoint_ref = lockfile_impact.lockfile_checkpoint_ref.clone();
    let checkpoint_created_before_apply = lockfile_checkpoint_ref.is_some()
        || rollback_posture_class == RollbackPostureClass::RollbackPostureNotApplicableReadOnly;
    RollbackCheckpointAlphaSummary {
        rollback_posture_class,
        rollback_posture_token: rollback_posture_class.as_str().to_owned(),
        lockfile_checkpoint_ref,
        workspace_snapshot_checkpoint_ref: None,
        cache_checkpoint_ref: None,
        checkpoint_created_before_apply,
        disclosure: rollback_disclosure(rollback_posture_class),
    }
}

fn review_outcome_for(
    packet: &PackageOperationAlphaPacket,
    issues: &[PackageOperationAlphaViolation],
) -> PackageReviewOutcomeClass {
    if packet.operation_class == PackageOperationClass::AuditOnlyNoStateChange {
        return PackageReviewOutcomeClass::ReviewReportedReadOnly;
    }
    if issues.iter().any(|issue| {
        matches!(
            issue,
            PackageOperationAlphaViolation::RegistryAuthBlocked
                | PackageOperationAlphaViolation::RawRegistrySecretObserved
                | PackageOperationAlphaViolation::RegistrySourceUnknown
                | PackageOperationAlphaViolation::PackageManagerUnknown
        )
    }) {
        return PackageReviewOutcomeClass::ReviewBlockedPendingPolicy;
    }
    if issues
        .iter()
        .any(|issue| *issue == PackageOperationAlphaViolation::ScriptRiskRequiresConsent)
    {
        return PackageReviewOutcomeClass::ReviewBlockedPendingNativeBuildConsent;
    }
    if issues.iter().any(|issue| {
        matches!(
            issue,
            PackageOperationAlphaViolation::LockfileImpactUnknown
                | PackageOperationAlphaViolation::ManifestUnavailable
        )
    }) {
        return PackageReviewOutcomeClass::ReviewBlockedPendingLockfileResolution;
    }
    if issues.is_empty() {
        PackageReviewOutcomeClass::ReviewAdmittedApplyPending
    } else {
        PackageReviewOutcomeClass::ReviewBlockedPendingUserDecision
    }
}

fn manifest_diff_disclosure(
    manifest_delta_class: ManifestDeltaClass,
    dependency_section: DependencySection,
    manifest_path: &str,
) -> String {
    match manifest_delta_class {
        ManifestDeltaClass::DependencyEntryAdded => format!(
            "Review will add one entry to {} in {} before the lockfile is regenerated for review.",
            dependency_section.package_json_key(),
            manifest_path
        ),
        ManifestDeltaClass::DependencyEntryUpdated => format!(
            "Review will update one entry in {} in {} before lockfile regeneration is reviewed.",
            dependency_section.package_json_key(),
            manifest_path
        ),
        ManifestDeltaClass::DependencyEntryRemoved => format!(
            "Review will remove one entry from {} in {} before lockfile regeneration is reviewed.",
            dependency_section.package_json_key(),
            manifest_path
        ),
        ManifestDeltaClass::ManifestUnchangedAuditOnly => {
            format!("Review keeps {} unchanged.", manifest_path)
        }
        ManifestDeltaClass::ManifestDeltaUnknownRequiresReview => {
            format!(
                "Review could not prove a safe manifest delta for {}.",
                manifest_path
            )
        }
    }
}

fn lockfile_disclosure(
    lockfile_impact_class: LockfileImpactClass,
    mutation_mode: LockfileMutationMode,
) -> String {
    match mutation_mode {
        LockfileMutationMode::RegenerateAndReview => format!(
            "{}; inline lockfile editing is not claimed, so the package manager regenerates and the diff is reviewed.",
            lockfile_impact_class.as_str()
        ),
        LockfileMutationMode::CompareOnlyInspect => {
            "Read-only operation; lockfile is inspected without mutation.".to_owned()
        }
        LockfileMutationMode::BlockedUnknownRequiresReview => {
            "Lockfile impact is unknown; apply is blocked until review is renewed.".to_owned()
        }
        LockfileMutationMode::InlineEditRoundTripSafe => {
            "Round-trip-safe inline lockfile edit is proven for this operation.".to_owned()
        }
    }
}

fn rollback_disclosure(rollback_posture_class: RollbackPostureClass) -> String {
    match rollback_posture_class {
        RollbackPostureClass::RollbackViaLockfileCheckpoint => {
            "Rollback restores the pre-apply lockfile checkpoint.".to_owned()
        }
        RollbackPostureClass::RollbackViaWorkspaceSnapshotCheckpoint => {
            "Rollback restores the workspace snapshot checkpoint.".to_owned()
        }
        RollbackPostureClass::RollbackRequiresReResolveNoInlineCheckpoint => {
            "Rollback requires a fresh resolver run because no inline checkpoint exists.".to_owned()
        }
        RollbackPostureClass::RollbackBlockedNativeArtifactsMustBeRecompiled => {
            "Rollback is bounded by native artifacts that must be recompiled.".to_owned()
        }
        RollbackPostureClass::RollbackBlockedPostInstallScriptWasUnsandboxed => {
            "Rollback is bounded by unsandboxed postinstall side effects.".to_owned()
        }
        RollbackPostureClass::RollbackUnavailableDestructiveRemoveWithNoCheckpoint => {
            "Rollback is unavailable because no destructive-remove checkpoint exists.".to_owned()
        }
        RollbackPostureClass::RollbackPostureNotApplicableReadOnly => {
            "Read-only operation; no rollback is required.".to_owned()
        }
    }
}

fn stable_fragment(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_matches('_').to_owned()
}

#[allow(dead_code)]
fn normalize_relative_path(path: &Path) -> String {
    path.components()
        .collect::<PathBuf>()
        .to_string_lossy()
        .replace('\\', "/")
}
