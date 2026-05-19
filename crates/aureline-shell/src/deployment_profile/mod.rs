//! Deployment profile, residual dependency, and control-plane/data-plane
//! status truth for claimed beta deployment rows.
//!
//! ## Why one truth lane, not seven
//!
//! Local-only, managed, self-hosted, sovereign, mirrored, and air-gapped are
//! product facts about the running install. About panels, diagnostics views,
//! support packets, admin-audit exports, release-evidence excerpts,
//! status-bar deployment cells, companion surfaces, and CLI text formatters
//! all need the same answer when a reviewer or user asks "what is the
//! deployment posture right now, which residual vendor or public lanes
//! remain, and what is the safest bounded next action when something is
//! degraded?". Forking that answer per surface lets one help screen claim
//! "self-hosted" while a support packet quotes a vendor-hosted control
//! plane, or a status bar render generic "service degraded" copy where
//! local editing is still safe.
//!
//! This module mints one [`DeploymentProfilePage`] that composes:
//!
//! - one [`ProfileSummary`] — the inspectable posture (deployment profile,
//!   tenant/org scope, region scope, retention class, key mode,
//!   mirror/offline state, last control-plane sync or cache age, control-
//!   plane worst-state, data-plane worst-state, residual-dependency row
//!   refs, mirror/offline artifact row refs, plane-status strip ref,
//!   prohibited-implied-claim guardrails, and an inspect-only open-details
//!   action);
//! - one [`PlaneStatusStrip`] — the separated control-plane and data-plane
//!   status strip with one [`SafestNextAction`] drawn from a closed
//!   vocabulary (`continue_local`, `retry_policy_sync`, `switch_mirror`,
//!   `export_packet`, `reconnect_managed_session`, `recheck_boundary`,
//!   `await_resolution`, `open_outage_notice`);
//! - any number of [`ResidualDependencyRow`] records naming residual
//!   vendor or public dependencies with the exact feature consequence if
//!   unreachable;
//! - any number of [`MirrorOfflineArtifactRow`] records naming the five
//!   frozen artifact families (`updates`, `extensions`, `docs_pack`,
//!   `policy_bundle`, `models`) with signer/digest/freshness/cache state.
//!
//! ## Cross-surface invariants this module enforces
//!
//! - `self_hosted` and `enterprise_online` profiles MUST carry actionable
//!   tenant, region, and key posture; `not_applicable` on any of those
//!   axes is a [`DeploymentProfileDefect`].
//! - `self_hosted` profiles MUST NOT silently carry `vendor_managed` keys.
//! - `air_gapped` profiles MUST declare `offline_air_gapped` mirror/offline
//!   state, emit at least one mirror/offline artifact row, and never route
//!   through `companion_surface` consumers.
//! - `managed_cloud` profiles MUST list the
//!   `implied_self_hosted_when_managed_cloud` and
//!   `implied_managed_independence_when_local_dependent` guardrails.
//! - When the data plane stays `available_local_safe` and the control
//!   plane is `healthy` or `not_applicable`, the safest next action MUST
//!   be `continue_local` (or `await_resolution`). Generic
//!   "service degraded" copy is non-conforming where local editing remains
//!   safe.
//! - When the control plane is impaired but the data plane is still
//!   `available_local_safe` or `available_mirror_backed`, the safest next
//!   action MUST be drawn from `{continue_local, retry_policy_sync,
//!   switch_mirror, export_packet, reconnect_managed_session,
//!   recheck_boundary, open_outage_notice}`.
//! - `mirror_only` and `air_gapped` mirror/offline state MUST emit at
//!   least one mirror/offline artifact row.
//!
//! ## Out of scope
//!
//! The module does not run a managed control plane, fetch policy bundles,
//! switch mirrors, drive auth flows, or own the locality matrix. It
//! projects existing posture facts the shell already has (typically
//! produced by `aureline-auth`, `aureline-release`, and the local-core
//! continuity packet) into the three records the M3 beta-exit lane needs.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`ProfileSummary`].
pub const PROFILE_SUMMARY_RECORD_KIND: &str = "deployment_profile_summary_record";
/// Stable record-kind tag for [`PlaneStatusStrip`].
pub const PLANE_STATUS_STRIP_RECORD_KIND: &str = "plane_status_strip_record";
/// Stable record-kind tag for [`ResidualDependencyRow`].
pub const RESIDUAL_DEPENDENCY_ROW_RECORD_KIND: &str = "residual_dependency_row_record";
/// Stable record-kind tag for [`MirrorOfflineArtifactRow`].
pub const MIRROR_OFFLINE_ARTIFACT_ROW_RECORD_KIND: &str = "mirror_offline_artifact_row_record";
/// Stable record-kind tag for [`DeploymentProfilePage`].
pub const DEPLOYMENT_PROFILE_PAGE_RECORD_KIND: &str = "deployment_profile_page_record";
/// Stable record-kind tag for [`DeploymentProfileSupportExport`].
pub const DEPLOYMENT_PROFILE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "deployment_profile_support_export_record";

/// Schema version this module produces for every record kind.
pub const DEPLOYMENT_PROFILE_SCHEMA_VERSION: u32 = 1;

/// Reviewer notice rendered on every projection so the lane's scope is
/// not overstated.
pub const DEPLOYMENT_PROFILE_NOTICE: &str =
    "Deployment profile lane: posture is projected from real runtime facts, signed configuration, \
     and current evidence packets. The shell never invents tenant, region, key, mirror, or \
     residual-dependency truth of its own.";

/// Closed deployment-profile vocabulary mirroring
/// `/artifacts/governance/deployment_profiles.yaml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfileClass {
    IndividualLocal,
    SelfHosted,
    EnterpriseOnline,
    AirGapped,
    ManagedCloud,
}

impl DeploymentProfileClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndividualLocal => "individual_local",
            Self::SelfHosted => "self_hosted",
            Self::EnterpriseOnline => "enterprise_online",
            Self::AirGapped => "air_gapped",
            Self::ManagedCloud => "managed_cloud",
        }
    }

    /// True when the profile carries no managed control plane in scope.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::IndividualLocal)
    }
}

/// Product-facing label vocabulary mirroring
/// `product_facing_label_vocabulary` in the deployment-profiles ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductFacingLabelClass {
    DesktopLocalFirst,
    SelfHostedSovereign,
    HybridRemoteAttach,
    AirGappedMirrorOnly,
    BrowserCompanionHandoffDefaultHome,
}

impl ProductFacingLabelClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopLocalFirst => "desktop_local_first",
            Self::SelfHostedSovereign => "self_hosted_sovereign",
            Self::HybridRemoteAttach => "hybrid_remote_attach",
            Self::AirGappedMirrorOnly => "air_gapped_mirror_only",
            Self::BrowserCompanionHandoffDefaultHome => "browser_companion_handoff_default_home",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TenantOrgScopeClass {
    SingleUserLocal,
    CustomerTenant,
    SharedMultiTenant,
    TenantBoundaryRecheckRequired,
    NotApplicable,
}

impl TenantOrgScopeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleUserLocal => "single_user_local",
            Self::CustomerTenant => "customer_tenant",
            Self::SharedMultiTenant => "shared_multi_tenant",
            Self::TenantBoundaryRecheckRequired => "tenant_boundary_recheck_required",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionScopeClass {
    CustomerRegionPinned,
    RemoteTargetRegion,
    BoundaryRecheckRequired,
    NotApplicable,
}

impl RegionScopeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CustomerRegionPinned => "customer_region_pinned",
            Self::RemoteTargetRegion => "remote_target_region",
            Self::BoundaryRecheckRequired => "boundary_recheck_required",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    NoRetentionBeyondLocalDisk,
    WorkspaceRepoRetained,
    CustomerRetentionWindow,
    VendorRetentionWindowWithCustomerPolicy,
    VendorRetentionWindowDefault,
    RetentionNotApplicable,
}

impl RetentionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRetentionBeyondLocalDisk => "no_retention_beyond_local_disk",
            Self::WorkspaceRepoRetained => "workspace_repo_retained",
            Self::CustomerRetentionWindow => "customer_retention_window",
            Self::VendorRetentionWindowWithCustomerPolicy => {
                "vendor_retention_window_with_customer_policy"
            }
            Self::VendorRetentionWindowDefault => "vendor_retention_window_default",
            Self::RetentionNotApplicable => "retention_not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyModeClass {
    OsStore,
    VendorManaged,
    CustomerManaged,
    OfflineTrustRoot,
    NotApplicable,
}

impl KeyModeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsStore => "os_store",
            Self::VendorManaged => "vendor_managed",
            Self::CustomerManaged => "customer_managed",
            Self::OfflineTrustRoot => "offline_trust_root",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorOfflineStateClass {
    OnlineLiveAllowed,
    OnlineMirrorOnly,
    OfflineGracePreserved,
    OfflineAirGapped,
    DenyAllEnforced,
    NetworkDisabledByUser,
    NetworkDegradedHeuristic,
    NotApplicable,
}

impl MirrorOfflineStateClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnlineLiveAllowed => "online_live_allowed",
            Self::OnlineMirrorOnly => "online_mirror_only",
            Self::OfflineGracePreserved => "offline_grace_preserved",
            Self::OfflineAirGapped => "offline_air_gapped",
            Self::DenyAllEnforced => "deny_all_enforced",
            Self::NetworkDisabledByUser => "network_disabled_by_user",
            Self::NetworkDegradedHeuristic => "network_degraded_heuristic",
            Self::NotApplicable => "not_applicable",
        }
    }

    pub const fn requires_mirror_offline_artifact_row(self) -> bool {
        matches!(self, Self::OnlineMirrorOnly | Self::OfflineAirGapped)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlPlaneServiceClass {
    SyncService,
    RegistryService,
    RelayService,
    AiBrokerService,
    AuthIdentityService,
    PolicyService,
    DocsPackService,
    CatalogService,
    TelemetrySinkService,
}

impl ControlPlaneServiceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncService => "sync_service",
            Self::RegistryService => "registry_service",
            Self::RelayService => "relay_service",
            Self::AiBrokerService => "ai_broker_service",
            Self::AuthIdentityService => "auth_identity_service",
            Self::PolicyService => "policy_service",
            Self::DocsPackService => "docs_pack_service",
            Self::CatalogService => "catalog_service",
            Self::TelemetrySinkService => "telemetry_sink_service",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlPlaneServiceStateClass {
    Healthy,
    StaleCache,
    Unavailable,
    MirrorOnly,
    BoundaryRecheckRequired,
    NotApplicable,
}

impl ControlPlaneServiceStateClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::StaleCache => "stale_cache",
            Self::Unavailable => "unavailable",
            Self::MirrorOnly => "mirror_only",
            Self::BoundaryRecheckRequired => "boundary_recheck_required",
            Self::NotApplicable => "not_applicable",
        }
    }

    pub const fn is_impaired(self) -> bool {
        matches!(
            self,
            Self::StaleCache | Self::Unavailable | Self::MirrorOnly | Self::BoundaryRecheckRequired
        )
    }

    pub const fn is_healthy_or_not_applicable(self) -> bool {
        matches!(self, Self::Healthy | Self::NotApplicable)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataPlaneCapabilityClass {
    LocalEditing,
    LocalSave,
    LocalSearch,
    LocalGit,
    LocalTasks,
    LocalDocsInspect,
    LocalExport,
    LocalDiagnostics,
}

impl DataPlaneCapabilityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalEditing => "local_editing",
            Self::LocalSave => "local_save",
            Self::LocalSearch => "local_search",
            Self::LocalGit => "local_git",
            Self::LocalTasks => "local_tasks",
            Self::LocalDocsInspect => "local_docs_inspect",
            Self::LocalExport => "local_export",
            Self::LocalDiagnostics => "local_diagnostics",
        }
    }

    /// The eight capabilities that make up the local-core baseline.
    pub const fn local_core_baseline() -> [Self; 8] {
        [
            Self::LocalEditing,
            Self::LocalSave,
            Self::LocalSearch,
            Self::LocalGit,
            Self::LocalTasks,
            Self::LocalDocsInspect,
            Self::LocalExport,
            Self::LocalDiagnostics,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataPlaneCapabilityStateClass {
    AvailableLocalSafe,
    AvailableMirrorBacked,
    ReducedReadOnly,
    BlockedPendingReconnect,
    BlockedPendingBoundaryRecheck,
    BlockedByPolicy,
    NotApplicable,
}

impl DataPlaneCapabilityStateClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AvailableLocalSafe => "available_local_safe",
            Self::AvailableMirrorBacked => "available_mirror_backed",
            Self::ReducedReadOnly => "reduced_read_only",
            Self::BlockedPendingReconnect => "blocked_pending_reconnect",
            Self::BlockedPendingBoundaryRecheck => "blocked_pending_boundary_recheck",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::NotApplicable => "not_applicable",
        }
    }

    pub const fn is_local_safe_or_mirror_backed(self) -> bool {
        matches!(self, Self::AvailableLocalSafe | Self::AvailableMirrorBacked)
    }

    pub const fn is_impaired(self) -> bool {
        matches!(
            self,
            Self::ReducedReadOnly
                | Self::BlockedPendingReconnect
                | Self::BlockedPendingBoundaryRecheck
                | Self::BlockedByPolicy
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyClass {
    SignIn,
    PackageRegistry,
    RemoteMirror,
    RemoteAgent,
    SymbolService,
    AiProvider,
    PolicyBundle,
    DocsPack,
    BrowserHandoff,
    CompanionNotificationChannel,
    HostedControlPlaneReachability,
}

impl DependencyClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignIn => "sign_in",
            Self::PackageRegistry => "package_registry",
            Self::RemoteMirror => "remote_mirror",
            Self::RemoteAgent => "remote_agent",
            Self::SymbolService => "symbol_service",
            Self::AiProvider => "ai_provider",
            Self::PolicyBundle => "policy_bundle",
            Self::DocsPack => "docs_pack",
            Self::BrowserHandoff => "browser_handoff",
            Self::CompanionNotificationChannel => "companion_notification_channel",
            Self::HostedControlPlaneReachability => "hosted_control_plane_reachability",
        }
    }

    /// True when a `required` row of this class must declare a vendor or
    /// public dependence.
    pub const fn is_vendor_bound_when_required(self) -> bool {
        matches!(
            self,
            Self::AiProvider
                | Self::BrowserHandoff
                | Self::CompanionNotificationChannel
                | Self::HostedControlPlaneReachability
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostureClass {
    Required,
    Optional,
    Cached,
    Mirrored,
    Forbidden,
    NotApplicableStructural,
}

impl PostureClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::Cached => "cached",
            Self::Mirrored => "mirrored",
            Self::Forbidden => "forbidden",
            Self::NotApplicableStructural => "not_applicable_structural",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AbsenceImpactClass {
    NoImpactCapabilityNotClaimedForProfile,
    NarrowsToLocalCoreCapabilities,
    NarrowsToMirrorBackedReadOnly,
    NarrowsToCachedLastKnownGood,
    NarrowsToReviewOnlyBoundaryRecheck,
    BlockedPendingReconnect,
    BlockedPendingBoundaryRecheck,
    BlockedPendingMirrorRefresh,
    BlockedByPolicy,
    FailClosedForbiddenInProfile,
}

impl AbsenceImpactClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoImpactCapabilityNotClaimedForProfile => {
                "no_impact_capability_not_claimed_for_profile"
            }
            Self::NarrowsToLocalCoreCapabilities => "narrows_to_local_core_capabilities",
            Self::NarrowsToMirrorBackedReadOnly => "narrows_to_mirror_backed_read_only",
            Self::NarrowsToCachedLastKnownGood => "narrows_to_cached_last_known_good",
            Self::NarrowsToReviewOnlyBoundaryRecheck => "narrows_to_review_only_boundary_recheck",
            Self::BlockedPendingReconnect => "blocked_pending_reconnect",
            Self::BlockedPendingBoundaryRecheck => "blocked_pending_boundary_recheck",
            Self::BlockedPendingMirrorRefresh => "blocked_pending_mirror_refresh",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::FailClosedForbiddenInProfile => "fail_closed_forbidden_in_profile",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityFallbackClass {
    ContinueLocalNoRestore,
    ReplayCachedSnapshot,
    MirrorSnapshotImport,
    ResumeAfterReconnect,
    ManualReconcileAfterBoundaryChange,
    FailClosedNoFallback,
    NotApplicableStructural,
}

impl ContinuityFallbackClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinueLocalNoRestore => "continue_local_no_restore",
            Self::ReplayCachedSnapshot => "replay_cached_snapshot",
            Self::MirrorSnapshotImport => "mirror_snapshot_import",
            Self::ResumeAfterReconnect => "resume_after_reconnect",
            Self::ManualReconcileAfterBoundaryChange => "manual_reconcile_after_boundary_change",
            Self::FailClosedNoFallback => "fail_closed_no_fallback",
            Self::NotApplicableStructural => "not_applicable_structural",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactClass {
    Updates,
    Extensions,
    DocsPack,
    PolicyBundle,
    Models,
}

impl ArtifactClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Updates => "updates",
            Self::Extensions => "extensions",
            Self::DocsPack => "docs_pack",
            Self::PolicyBundle => "policy_bundle",
            Self::Models => "models",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignerStateClass {
    SignedTrustRootPinned,
    SignedOfflineTrustRoot,
    SignedOrgCaPinned,
    Unsigned,
    SignerUnknown,
    NotApplicable,
}

impl SignerStateClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedTrustRootPinned => "signed_trust_root_pinned",
            Self::SignedOfflineTrustRoot => "signed_offline_trust_root",
            Self::SignedOrgCaPinned => "signed_org_ca_pinned",
            Self::Unsigned => "unsigned",
            Self::SignerUnknown => "signer_unknown",
            Self::NotApplicable => "not_applicable",
        }
    }

    pub const fn requires_fingerprint(self) -> bool {
        matches!(
            self,
            Self::SignedTrustRootPinned | Self::SignedOfflineTrustRoot | Self::SignedOrgCaPinned
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigestStateClass {
    DigestVerified,
    DigestPending,
    DigestMismatch,
    DigestUnknown,
    NotApplicable,
}

impl DigestStateClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DigestVerified => "digest_verified",
            Self::DigestPending => "digest_pending",
            Self::DigestMismatch => "digest_mismatch",
            Self::DigestUnknown => "digest_unknown",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorFreshnessClass {
    MirrorFreshWithinWindow,
    MirrorWithinExtendedWindow,
    MirrorPastExtendedWindow,
    MirrorFreshnessUnknown,
    NotApplicable,
}

impl MirrorFreshnessClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorFreshWithinWindow => "mirror_fresh_within_window",
            Self::MirrorWithinExtendedWindow => "mirror_within_extended_window",
            Self::MirrorPastExtendedWindow => "mirror_past_extended_window",
            Self::MirrorFreshnessUnknown => "mirror_freshness_unknown",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineCachePostureClass {
    OfflineBundlePresent,
    MirrorSnapshotPresent,
    NoCacheRequired,
    CacheMissingBlocked,
    NotApplicable,
}

impl OfflineCachePostureClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfflineBundlePresent => "offline_bundle_present",
            Self::MirrorSnapshotPresent => "mirror_snapshot_present",
            Self::NoCacheRequired => "no_cache_required",
            Self::CacheMissingBlocked => "cache_missing_blocked",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorSourceClass {
    CustomerOperatedMirror,
    VendorPublishedMirrorForCustomer,
    OfflineBundleDerivedMirror,
    NotApplicable,
}

impl MirrorSourceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CustomerOperatedMirror => "customer_operated_mirror",
            Self::VendorPublishedMirrorForCustomer => "vendor_published_mirror_for_customer",
            Self::OfflineBundleDerivedMirror => "offline_bundle_derived_mirror",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProhibitedImpliedClaimClass {
    ImpliedAirGappedWhenEgressAllowed,
    ImpliedSovereignWhenVendorManaged,
    ImpliedSelfHostedWhenManagedCloud,
    ImpliedNoResidualDependencyWhenRequiredPresent,
    ImpliedOfflineParityWhenMirrorOnly,
    ImpliedManagedIndependenceWhenLocalDependent,
    ImpliedAlwaysFreshWhenBoundedOrUnboundedStale,
}

impl ProhibitedImpliedClaimClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImpliedAirGappedWhenEgressAllowed => "implied_air_gapped_when_egress_allowed",
            Self::ImpliedSovereignWhenVendorManaged => "implied_sovereign_when_vendor_managed",
            Self::ImpliedSelfHostedWhenManagedCloud => "implied_self_hosted_when_managed_cloud",
            Self::ImpliedNoResidualDependencyWhenRequiredPresent => {
                "implied_no_residual_dependency_when_required_present"
            }
            Self::ImpliedOfflineParityWhenMirrorOnly => "implied_offline_parity_when_mirror_only",
            Self::ImpliedManagedIndependenceWhenLocalDependent => {
                "implied_managed_independence_when_local_dependent"
            }
            Self::ImpliedAlwaysFreshWhenBoundedOrUnboundedStale => {
                "implied_always_fresh_when_bounded_or_unbounded_stale"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceClass {
    AboutPanel,
    DiagnosticsView,
    SupportPacketExport,
    AdminAuditExport,
    ReleaseEvidenceExcerpt,
    StatusBarCell,
    BannerNotice,
    CompanionSurface,
    CliTextFormatter,
}

impl ConsumerSurfaceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AboutPanel => "about_panel",
            Self::DiagnosticsView => "diagnostics_view",
            Self::SupportPacketExport => "support_packet_export",
            Self::AdminAuditExport => "admin_audit_export",
            Self::ReleaseEvidenceExcerpt => "release_evidence_excerpt",
            Self::StatusBarCell => "status_bar_cell",
            Self::BannerNotice => "banner_notice",
            Self::CompanionSurface => "companion_surface",
            Self::CliTextFormatter => "cli_text_formatter",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    MetadataSafeDefault,
    OperatorOnlyRestricted,
    InternalSupportRestricted,
    SigningEvidenceOnly,
}

impl RedactionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::InternalSupportRestricted => "internal_support_restricted",
            Self::SigningEvidenceOnly => "signing_evidence_only",
        }
    }

    pub const fn is_export_safe_compatible(self) -> bool {
        matches!(self, Self::MetadataSafeDefault | Self::OperatorOnlyRestricted)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StalenessClass {
    Fresh,
    BoundedStale,
    UnboundedStale,
}

impl StalenessClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::BoundedStale => "bounded_stale",
            Self::UnboundedStale => "unbounded_stale",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafestNextActionClass {
    ContinueLocal,
    RetryPolicySync,
    SwitchMirror,
    ExportPacket,
    ReconnectManagedSession,
    RecheckBoundary,
    AwaitResolution,
    OpenOutageNotice,
}

impl SafestNextActionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinueLocal => "continue_local",
            Self::RetryPolicySync => "retry_policy_sync",
            Self::SwitchMirror => "switch_mirror",
            Self::ExportPacket => "export_packet",
            Self::ReconnectManagedSession => "reconnect_managed_session",
            Self::RecheckBoundary => "recheck_boundary",
            Self::AwaitResolution => "await_resolution",
            Self::OpenOutageNotice => "open_outage_notice",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::ContinueLocal => "Continue local",
            Self::RetryPolicySync => "Retry policy sync",
            Self::SwitchMirror => "Switch mirror",
            Self::ExportPacket => "Export packet",
            Self::ReconnectManagedSession => "Reconnect managed session",
            Self::RecheckBoundary => "Recheck boundary",
            Self::AwaitResolution => "Await resolution",
            Self::OpenOutageNotice => "Open outage notice",
        }
    }

    /// True when this action preserves local work and may run without
    /// explicit managed consent.
    pub const fn is_local_safe(self) -> bool {
        matches!(self, Self::ContinueLocal | Self::ExportPacket | Self::AwaitResolution)
    }
}

/// Freshness summary projected from the underlying continuity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessSummary {
    pub staleness_class: StalenessClass,
    pub summary_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_control_plane_sync_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_age_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_floor_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staleness_rationale: Option<String>,
}

/// Inspect-only open-details action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectOnlyAction {
    pub action_id: String,
    pub label: String,
    pub target_route_ref: String,
    pub scope_class: String,
    pub authority_class: String,
    pub consent_class: String,
    pub side_effects: Vec<String>,
    pub preserves_evidence_context: bool,
    pub revalidation_on_open: String,
    pub modal_prohibited: bool,
}

impl InspectOnlyAction {
    /// Build the default inspect-only open-details action for a profile
    /// summary card.
    pub fn open_details(action_id: impl Into<String>, target_route_ref: impl Into<String>) -> Self {
        Self {
            action_id: action_id.into(),
            label: "Open deployment details".to_owned(),
            target_route_ref: target_route_ref.into(),
            scope_class: "scope_local_only".to_owned(),
            authority_class: "user_local_authority".to_owned(),
            consent_class: "no_consent_required_safe_default".to_owned(),
            side_effects: vec!["no_side_effect_inspect_only".to_owned()],
            preserves_evidence_context: true,
            revalidation_on_open: "snapshot_open_read_only".to_owned(),
            modal_prohibited: true,
        }
    }
}

/// The profile-summary record consumers read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSummary {
    pub record_kind: String,
    pub schema_version: u32,
    pub summary_id: String,
    pub emitted_at: String,
    pub deployment_profile: DeploymentProfileClass,
    pub product_facing_label_class: ProductFacingLabelClass,
    pub tenant_org_scope_class: TenantOrgScopeClass,
    pub region_scope_class: RegionScopeClass,
    pub retention_class: RetentionClass,
    pub key_mode_class: KeyModeClass,
    pub mirror_offline_state_class: MirrorOfflineStateClass,
    pub freshness: FreshnessSummary,
    pub control_plane_worst_state_class: ControlPlaneServiceStateClass,
    pub data_plane_worst_state_class: DataPlaneCapabilityStateClass,
    pub residual_dependency_row_refs: Vec<String>,
    pub mirror_offline_artifact_row_refs: Vec<String>,
    pub plane_status_strip_ref: String,
    pub prohibited_implied_claim_classes: Vec<ProhibitedImpliedClaimClass>,
    pub open_details_action: InspectOnlyAction,
    pub consumer_surfaces: Vec<ConsumerSurfaceClass>,
    pub redaction_class: RedactionClass,
    pub export_safe: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_summary_card_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_continuity_packet_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_outage_notice_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_transport_posture_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlPlaneSummary {
    pub worst_state_class: ControlPlaneServiceStateClass,
    pub summary_label: String,
    pub impaired_service_classes: Vec<ControlPlaneServiceClass>,
    pub healthy_service_classes: Vec<ControlPlaneServiceClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_sync_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataPlaneSummary {
    pub worst_state_class: DataPlaneCapabilityStateClass,
    pub summary_label: String,
    pub impaired_capability_classes: Vec<DataPlaneCapabilityClass>,
    pub available_local_safe_capability_classes: Vec<DataPlaneCapabilityClass>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafestNextAction {
    pub action_id: String,
    pub action_class: SafestNextActionClass,
    pub label: String,
    pub scope_class: String,
    pub authority_class: String,
    pub consent_class: String,
    pub side_effects: Vec<String>,
    pub preserves_local_state: bool,
    pub modal_prohibited: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_route_ref: Option<String>,
}

impl SafestNextAction {
    /// Build a `continue_local` safest-next-action.
    pub fn continue_local(action_id: impl Into<String>) -> Self {
        Self {
            action_id: action_id.into(),
            action_class: SafestNextActionClass::ContinueLocal,
            label: SafestNextActionClass::ContinueLocal.label().to_owned(),
            scope_class: "scope_local_only".to_owned(),
            authority_class: "user_local_authority".to_owned(),
            consent_class: "no_consent_required_safe_default".to_owned(),
            side_effects: vec!["no_side_effect_inspect_only".to_owned()],
            preserves_local_state: true,
            modal_prohibited: true,
            target_route_ref: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaneStatusStrip {
    pub record_kind: String,
    pub schema_version: u32,
    pub strip_id: String,
    pub emitted_at: String,
    pub control_plane_summary: ControlPlaneSummary,
    pub data_plane_summary: DataPlaneSummary,
    pub safest_next_action: SafestNextAction,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternate_actions: Vec<SafestNextAction>,
    pub consumer_surfaces: Vec<ConsumerSurfaceClass>,
    pub redaction_class: RedactionClass,
    pub export_safe: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_profile_summary_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_continuity_packet_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_outage_notice_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidualDependencyRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub row_id: String,
    pub profile_summary_ref: String,
    pub dependency_class: DependencyClass,
    pub posture_class: PostureClass,
    pub vendor_or_public_dependence: bool,
    pub dependent_feature_label: String,
    pub dependent_feature_refs: Vec<String>,
    pub unreachable_impact_class: AbsenceImpactClass,
    pub unreachable_impact_label: String,
    pub continuity_fallback_class: ContinuityFallbackClass,
    pub ledger_row_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_label: Option<String>,
    pub redaction_class: RedactionClass,
    pub export_safe: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_outage_notice_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_continuity_packet_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_links: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifyOrOpenManifestAction {
    pub action_id: String,
    pub label: String,
    pub scope_class: String,
    pub authority_class: String,
    pub consent_class: String,
    pub side_effects: Vec<String>,
    pub preserves_evidence_context: bool,
    pub modal_prohibited: bool,
    pub revalidation_on_open: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_links: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorOfflineArtifactRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub row_id: String,
    pub profile_summary_ref: String,
    pub artifact_class: ArtifactClass,
    pub artifact_label: String,
    pub signer_state_class: SignerStateClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_fingerprint_ref: Option<String>,
    pub digest_state_class: DigestStateClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest_ref: Option<String>,
    pub mirror_freshness_class: MirrorFreshnessClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_refresh_at: Option<String>,
    pub offline_cache_posture_class: OfflineCachePostureClass,
    pub mirror_source_class: MirrorSourceClass,
    pub verify_action: VerifyOrOpenManifestAction,
    pub open_manifest_action: VerifyOrOpenManifestAction,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_links: Vec<String>,
    pub redaction_class: RedactionClass,
    pub export_safe: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Composed deployment-profile page: one profile summary, one plane-status
/// strip, the residual-dependency rows, and the mirror/offline artifact
/// rows it references.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentProfilePage {
    pub record_kind: String,
    pub schema_version: u32,
    pub notice: String,
    pub page_id: String,
    pub profile_summary: ProfileSummary,
    pub plane_status_strip: PlaneStatusStrip,
    pub residual_dependency_rows: Vec<ResidualDependencyRow>,
    pub mirror_offline_artifact_rows: Vec<MirrorOfflineArtifactRow>,
    pub summary: DeploymentProfilePageSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DeploymentProfilePageSummary {
    pub residual_dependency_row_count: usize,
    pub residual_dependency_required_count: usize,
    pub residual_dependency_mirrored_count: usize,
    pub residual_dependency_cached_count: usize,
    pub residual_dependency_forbidden_count: usize,
    pub mirror_offline_artifact_row_count: usize,
    pub mirror_offline_artifact_digest_mismatch_count: usize,
    pub mirror_offline_artifact_cache_missing_count: usize,
    pub control_plane_impaired: bool,
    pub data_plane_impaired: bool,
    pub local_safe_remains: bool,
    pub honesty_marker_present: bool,
}

/// Reasons a [`DeploymentProfilePage`] fails its honesty invariants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentProfileDefect {
    /// A profile that requires actionable tenant/region/key posture left one
    /// of those axes at `not_applicable`.
    NotApplicableTenancyOrRegionOrKeyOnManagedProfile {
        profile: DeploymentProfileClass,
        tenant_org_scope_class: TenantOrgScopeClass,
        region_scope_class: RegionScopeClass,
        key_mode_class: KeyModeClass,
    },
    /// A self-hosted card carried vendor-managed keys.
    SelfHostedClaimedVendorManagedKeys,
    /// An air-gapped profile did not declare offline_air_gapped mirror state.
    AirGappedMissingOfflineAirGappedState {
        actual: MirrorOfflineStateClass,
    },
    /// An air-gapped profile routed through a companion surface.
    AirGappedRoutedThroughCompanionSurface,
    /// Mirror-only or air-gapped state emitted no mirror/offline artifact row.
    MirrorOrAirGappedMissingArtifactRow {
        mirror_offline_state_class: MirrorOfflineStateClass,
    },
    /// Managed-cloud profile missing the required prohibited-implied-claim
    /// guardrails.
    ManagedCloudMissingGuardrails,
    /// Mirror-only state missing the offline-parity guardrail.
    MirrorOnlyMissingOfflineParityGuardrail,
    /// The page surfaced generic "service degraded" copy where local-safe
    /// data plane and healthy control plane should have produced
    /// `continue_local` or `await_resolution`.
    GenericServiceDegradedWhereLocalSafeRemains {
        chose: SafestNextActionClass,
    },
    /// Required posture with a vendor-bound dependency class but
    /// `vendor_or_public_dependence` is false.
    RequiredVendorBoundDependencyMissingVendorDependenceFlag {
        dependency_class: DependencyClass,
    },
    /// A digest-mismatched mirror/offline artifact row did not block its
    /// verify action.
    DigestMismatchVerifyActionNotBlocked {
        artifact_class: ArtifactClass,
        revalidation_on_open: String,
    },
    /// Signed artifact row missing the signer fingerprint.
    SignedArtifactMissingSignerFingerprint {
        artifact_class: ArtifactClass,
        signer_state_class: SignerStateClass,
    },
    /// Mirror/offline artifact row claimed export_safe but carried a wider
    /// redaction class.
    ExportSafeArtifactRowWidenedRedaction {
        artifact_class: ArtifactClass,
        redaction_class: RedactionClass,
    },
    /// Profile summary claimed export_safe but carried a wider redaction
    /// class.
    ExportSafeProfileSummaryWidenedRedaction {
        redaction_class: RedactionClass,
    },
    /// The profile summary's plane-status strip ref does not match the page's
    /// plane-status strip id.
    PlaneStatusStripRefMismatch {
        expected: String,
        actual: String,
    },
    /// A residual-dependency row referenced a profile summary id other than
    /// the page's.
    ResidualDependencyRowProfileSummaryRefMismatch {
        row_id: String,
        expected: String,
        actual: String,
    },
    /// A mirror/offline artifact row referenced a profile summary id other
    /// than the page's.
    MirrorOfflineArtifactRowProfileSummaryRefMismatch {
        row_id: String,
        expected: String,
        actual: String,
    },
}

impl DeploymentProfilePage {
    /// Compose a page from its constituent records, returning the page plus
    /// any honesty defects detected.
    pub fn compose(
        page_id: impl Into<String>,
        profile_summary: ProfileSummary,
        plane_status_strip: PlaneStatusStrip,
        residual_dependency_rows: Vec<ResidualDependencyRow>,
        mirror_offline_artifact_rows: Vec<MirrorOfflineArtifactRow>,
    ) -> (Self, Vec<DeploymentProfileDefect>) {
        let summary = DeploymentProfilePageSummary::compute(
            &profile_summary,
            &plane_status_strip,
            &residual_dependency_rows,
            &mirror_offline_artifact_rows,
        );

        let mut page = Self {
            record_kind: DEPLOYMENT_PROFILE_PAGE_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
            notice: DEPLOYMENT_PROFILE_NOTICE.to_owned(),
            page_id: page_id.into(),
            profile_summary,
            plane_status_strip,
            residual_dependency_rows,
            mirror_offline_artifact_rows,
            summary,
        };
        let defects = page.audit();
        // Recompute the honesty marker now that defects are known.
        page.summary.honesty_marker_present = !defects.is_empty()
            || page.summary.control_plane_impaired
            || page.summary.data_plane_impaired
            || page.summary.mirror_offline_artifact_digest_mismatch_count > 0
            || page.summary.mirror_offline_artifact_cache_missing_count > 0;
        (page, defects)
    }

    /// Audit cross-record invariants and return any defects detected.
    pub fn audit(&self) -> Vec<DeploymentProfileDefect> {
        let mut defects = Vec::new();
        let ps = &self.profile_summary;

        match ps.deployment_profile {
            DeploymentProfileClass::SelfHosted
            | DeploymentProfileClass::EnterpriseOnline
            | DeploymentProfileClass::ManagedCloud => {
                if ps.tenant_org_scope_class == TenantOrgScopeClass::NotApplicable
                    || ps.region_scope_class == RegionScopeClass::NotApplicable
                    || ps.key_mode_class == KeyModeClass::NotApplicable
                {
                    defects.push(
                        DeploymentProfileDefect::NotApplicableTenancyOrRegionOrKeyOnManagedProfile {
                            profile: ps.deployment_profile,
                            tenant_org_scope_class: ps.tenant_org_scope_class,
                            region_scope_class: ps.region_scope_class,
                            key_mode_class: ps.key_mode_class,
                        },
                    );
                }
            }
            _ => {}
        }

        if ps.deployment_profile == DeploymentProfileClass::SelfHosted
            && ps.key_mode_class == KeyModeClass::VendorManaged
        {
            defects.push(DeploymentProfileDefect::SelfHostedClaimedVendorManagedKeys);
        }

        if ps.deployment_profile == DeploymentProfileClass::AirGapped {
            if ps.mirror_offline_state_class != MirrorOfflineStateClass::OfflineAirGapped {
                defects.push(DeploymentProfileDefect::AirGappedMissingOfflineAirGappedState {
                    actual: ps.mirror_offline_state_class,
                });
            }
            if ps
                .consumer_surfaces
                .iter()
                .any(|s| *s == ConsumerSurfaceClass::CompanionSurface)
            {
                defects.push(DeploymentProfileDefect::AirGappedRoutedThroughCompanionSurface);
            }
        }

        if ps.mirror_offline_state_class.requires_mirror_offline_artifact_row()
            && ps.mirror_offline_artifact_row_refs.is_empty()
        {
            defects.push(DeploymentProfileDefect::MirrorOrAirGappedMissingArtifactRow {
                mirror_offline_state_class: ps.mirror_offline_state_class,
            });
        }

        if ps.mirror_offline_state_class == MirrorOfflineStateClass::OnlineMirrorOnly
            && !ps
                .prohibited_implied_claim_classes
                .contains(&ProhibitedImpliedClaimClass::ImpliedOfflineParityWhenMirrorOnly)
        {
            defects.push(DeploymentProfileDefect::MirrorOnlyMissingOfflineParityGuardrail);
        }

        if ps.deployment_profile == DeploymentProfileClass::ManagedCloud {
            let needs = [
                ProhibitedImpliedClaimClass::ImpliedSelfHostedWhenManagedCloud,
                ProhibitedImpliedClaimClass::ImpliedManagedIndependenceWhenLocalDependent,
            ];
            let all_present = needs
                .iter()
                .all(|c| ps.prohibited_implied_claim_classes.contains(c));
            if !all_present {
                defects.push(DeploymentProfileDefect::ManagedCloudMissingGuardrails);
            }
        }

        if ps.export_safe && !ps.redaction_class.is_export_safe_compatible() {
            defects.push(DeploymentProfileDefect::ExportSafeProfileSummaryWidenedRedaction {
                redaction_class: ps.redaction_class,
            });
        }

        if ps.plane_status_strip_ref != self.plane_status_strip.strip_id {
            defects.push(DeploymentProfileDefect::PlaneStatusStripRefMismatch {
                expected: self.plane_status_strip.strip_id.clone(),
                actual: ps.plane_status_strip_ref.clone(),
            });
        }

        for row in &self.residual_dependency_rows {
            if row.profile_summary_ref != ps.summary_id {
                defects
                    .push(DeploymentProfileDefect::ResidualDependencyRowProfileSummaryRefMismatch {
                        row_id: row.row_id.clone(),
                        expected: ps.summary_id.clone(),
                        actual: row.profile_summary_ref.clone(),
                    });
            }
            if row.posture_class == PostureClass::Required
                && row.dependency_class.is_vendor_bound_when_required()
                && !row.vendor_or_public_dependence
            {
                defects.push(
                    DeploymentProfileDefect::RequiredVendorBoundDependencyMissingVendorDependenceFlag {
                        dependency_class: row.dependency_class,
                    },
                );
            }
        }

        for row in &self.mirror_offline_artifact_rows {
            if row.profile_summary_ref != ps.summary_id {
                defects.push(
                    DeploymentProfileDefect::MirrorOfflineArtifactRowProfileSummaryRefMismatch {
                        row_id: row.row_id.clone(),
                        expected: ps.summary_id.clone(),
                        actual: row.profile_summary_ref.clone(),
                    },
                );
            }
            if row.signer_state_class.requires_fingerprint()
                && row.signer_fingerprint_ref.is_none()
            {
                defects.push(
                    DeploymentProfileDefect::SignedArtifactMissingSignerFingerprint {
                        artifact_class: row.artifact_class,
                        signer_state_class: row.signer_state_class,
                    },
                );
            }
            if row.digest_state_class == DigestStateClass::DigestMismatch
                && row.verify_action.revalidation_on_open != "blocked_until_fresh"
            {
                defects.push(DeploymentProfileDefect::DigestMismatchVerifyActionNotBlocked {
                    artifact_class: row.artifact_class,
                    revalidation_on_open: row.verify_action.revalidation_on_open.clone(),
                });
            }
            if row.export_safe && !row.redaction_class.is_export_safe_compatible() {
                defects.push(DeploymentProfileDefect::ExportSafeArtifactRowWidenedRedaction {
                    artifact_class: row.artifact_class,
                    redaction_class: row.redaction_class,
                });
            }
        }

        let data_local_safe = self
            .plane_status_strip
            .data_plane_summary
            .worst_state_class
            .is_local_safe_or_mirror_backed();
        let control_ok = self
            .plane_status_strip
            .control_plane_summary
            .worst_state_class
            .is_healthy_or_not_applicable();
        if data_local_safe
            && control_ok
            && self.plane_status_strip.data_plane_summary.worst_state_class
                == DataPlaneCapabilityStateClass::AvailableLocalSafe
        {
            // Where local editing is safe and the control plane is healthy,
            // the safest next action must be local.
            match self.plane_status_strip.safest_next_action.action_class {
                SafestNextActionClass::ContinueLocal | SafestNextActionClass::AwaitResolution => {}
                other => defects.push(
                    DeploymentProfileDefect::GenericServiceDegradedWhereLocalSafeRemains {
                        chose: other,
                    },
                ),
            }
        }

        defects
    }

    /// Render a deterministic plaintext block for support-export and
    /// reviewer-facing previews. Stable for the same input.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Deployment profile page\n");
        out.push_str(&format!("Page id: {}\n", self.page_id));
        out.push_str(&format!(
            "Profile: {} ({})\n",
            self.profile_summary.deployment_profile.as_str(),
            self.profile_summary.product_facing_label_class.as_str(),
        ));
        out.push_str(&format!(
            "Tenant: {} | Region: {} | Retention: {} | Key: {}\n",
            self.profile_summary.tenant_org_scope_class.as_str(),
            self.profile_summary.region_scope_class.as_str(),
            self.profile_summary.retention_class.as_str(),
            self.profile_summary.key_mode_class.as_str(),
        ));
        out.push_str(&format!(
            "Mirror/offline: {}\n",
            self.profile_summary.mirror_offline_state_class.as_str(),
        ));
        out.push_str(&format!(
            "Control plane: {} | Data plane: {}\n",
            self.plane_status_strip
                .control_plane_summary
                .worst_state_class
                .as_str(),
            self.plane_status_strip
                .data_plane_summary
                .worst_state_class
                .as_str(),
        ));
        out.push_str(&format!(
            "Safest next action: {} ({})\n",
            self.plane_status_strip.safest_next_action.label,
            self.plane_status_strip.safest_next_action.action_class.as_str(),
        ));
        out.push_str(&format!(
            "Freshness: {} | {}\n",
            self.profile_summary.freshness.staleness_class.as_str(),
            self.profile_summary.freshness.summary_label,
        ));
        out.push_str(&format!(
            "Residual dependency rows: {} (required={}, mirrored={}, cached={}, forbidden={})\n",
            self.summary.residual_dependency_row_count,
            self.summary.residual_dependency_required_count,
            self.summary.residual_dependency_mirrored_count,
            self.summary.residual_dependency_cached_count,
            self.summary.residual_dependency_forbidden_count,
        ));
        out.push_str(&format!(
            "Mirror/offline artifact rows: {} (digest_mismatch={}, cache_missing={})\n",
            self.summary.mirror_offline_artifact_row_count,
            self.summary.mirror_offline_artifact_digest_mismatch_count,
            self.summary.mirror_offline_artifact_cache_missing_count,
        ));
        out.push_str(&format!(
            "Honesty marker: {}\n",
            if self.summary.honesty_marker_present {
                "present"
            } else {
                "none"
            },
        ));
        out
    }

    /// Project a support-safe export packet. Drops anything whose
    /// redaction class is not export-safe.
    pub fn project_support_export(&self) -> DeploymentProfileSupportExport {
        let residual = self
            .residual_dependency_rows
            .iter()
            .filter(|row| row.export_safe && row.redaction_class.is_export_safe_compatible())
            .cloned()
            .collect();
        let mirror = self
            .mirror_offline_artifact_rows
            .iter()
            .filter(|row| row.export_safe && row.redaction_class.is_export_safe_compatible())
            .cloned()
            .collect();
        DeploymentProfileSupportExport {
            record_kind: DEPLOYMENT_PROFILE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
            page_id: self.page_id.clone(),
            profile_summary: if self.profile_summary.export_safe
                && self
                    .profile_summary
                    .redaction_class
                    .is_export_safe_compatible()
            {
                Some(self.profile_summary.clone())
            } else {
                None
            },
            plane_status_strip: if self.plane_status_strip.export_safe
                && self
                    .plane_status_strip
                    .redaction_class
                    .is_export_safe_compatible()
            {
                Some(self.plane_status_strip.clone())
            } else {
                None
            },
            residual_dependency_rows: residual,
            mirror_offline_artifact_rows: mirror,
        }
    }
}

impl DeploymentProfilePageSummary {
    fn compute(
        profile_summary: &ProfileSummary,
        plane_status_strip: &PlaneStatusStrip,
        residual: &[ResidualDependencyRow],
        artifacts: &[MirrorOfflineArtifactRow],
    ) -> Self {
        let mut summary = Self {
            residual_dependency_row_count: residual.len(),
            mirror_offline_artifact_row_count: artifacts.len(),
            control_plane_impaired: plane_status_strip
                .control_plane_summary
                .worst_state_class
                .is_impaired(),
            data_plane_impaired: plane_status_strip
                .data_plane_summary
                .worst_state_class
                .is_impaired(),
            local_safe_remains: plane_status_strip
                .data_plane_summary
                .worst_state_class
                .is_local_safe_or_mirror_backed(),
            ..Default::default()
        };
        for row in residual {
            match row.posture_class {
                PostureClass::Required => summary.residual_dependency_required_count += 1,
                PostureClass::Mirrored => summary.residual_dependency_mirrored_count += 1,
                PostureClass::Cached => summary.residual_dependency_cached_count += 1,
                PostureClass::Forbidden => summary.residual_dependency_forbidden_count += 1,
                _ => {}
            }
        }
        for row in artifacts {
            if row.digest_state_class == DigestStateClass::DigestMismatch {
                summary.mirror_offline_artifact_digest_mismatch_count += 1;
            }
            if row.offline_cache_posture_class == OfflineCachePostureClass::CacheMissingBlocked {
                summary.mirror_offline_artifact_cache_missing_count += 1;
            }
        }
        summary.honesty_marker_present = summary.control_plane_impaired
            || summary.data_plane_impaired
            || summary.mirror_offline_artifact_digest_mismatch_count > 0
            || summary.mirror_offline_artifact_cache_missing_count > 0
            || profile_summary.freshness.staleness_class != StalenessClass::Fresh;
        summary
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentProfileSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub page_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_summary: Option<ProfileSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plane_status_strip: Option<PlaneStatusStrip>,
    pub residual_dependency_rows: Vec<ResidualDependencyRow>,
    pub mirror_offline_artifact_rows: Vec<MirrorOfflineArtifactRow>,
}

impl DeploymentProfileSupportExport {
    /// Confirms that no payload field carries a non-export-safe redaction
    /// class. Used by the support-bundle assembler to refuse a packet
    /// whose deployment lane widened redaction.
    pub fn is_redaction_consistent(&self) -> bool {
        if let Some(ps) = &self.profile_summary {
            if !ps.redaction_class.is_export_safe_compatible() {
                return false;
            }
        }
        if let Some(strip) = &self.plane_status_strip {
            if !strip.redaction_class.is_export_safe_compatible() {
                return false;
            }
        }
        self.residual_dependency_rows
            .iter()
            .all(|r| r.redaction_class.is_export_safe_compatible())
            && self
                .mirror_offline_artifact_rows
                .iter()
                .all(|r| r.redaction_class.is_export_safe_compatible())
    }
}

/// Build a baseline `individual_local` deployment-profile page. The
/// returned page passes all audits; tests and surfaces narrow from it
/// rather than constructing one from scratch.
pub fn individual_local_baseline_page(emitted_at: impl Into<String>) -> DeploymentProfilePage {
    let emitted_at = emitted_at.into();
    let summary_id = "summary.deployment.individual_local_baseline".to_owned();
    let strip_id = "strip.deployment.individual_local_baseline".to_owned();
    let profile_summary = ProfileSummary {
        record_kind: PROFILE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        summary_id: summary_id.clone(),
        emitted_at: emitted_at.clone(),
        deployment_profile: DeploymentProfileClass::IndividualLocal,
        product_facing_label_class: ProductFacingLabelClass::DesktopLocalFirst,
        tenant_org_scope_class: TenantOrgScopeClass::SingleUserLocal,
        region_scope_class: RegionScopeClass::NotApplicable,
        retention_class: RetentionClass::NoRetentionBeyondLocalDisk,
        key_mode_class: KeyModeClass::OsStore,
        mirror_offline_state_class: MirrorOfflineStateClass::NotApplicable,
        freshness: FreshnessSummary {
            staleness_class: StalenessClass::Fresh,
            summary_label: "Local-only install; no control-plane sync required.".to_owned(),
            last_control_plane_sync_at: None,
            cache_age_label: None,
            freshness_floor_ref: None,
            staleness_rationale: None,
        },
        control_plane_worst_state_class: ControlPlaneServiceStateClass::NotApplicable,
        data_plane_worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
        residual_dependency_row_refs: Vec::new(),
        mirror_offline_artifact_row_refs: Vec::new(),
        plane_status_strip_ref: strip_id.clone(),
        prohibited_implied_claim_classes: Vec::new(),
        open_details_action: InspectOnlyAction::open_details(
            "action.summary.individual_local_baseline.open_details",
            "route.deployment.details.individual_local",
        ),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::DiagnosticsView,
            ConsumerSurfaceClass::SupportPacketExport,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::CliTextFormatter,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_summary_card_ref: None,
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        linked_transport_posture_ref: None,
        notes: None,
    };
    let plane_status_strip = PlaneStatusStrip {
        record_kind: PLANE_STATUS_STRIP_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_PROFILE_SCHEMA_VERSION,
        strip_id: strip_id.clone(),
        emitted_at,
        control_plane_summary: ControlPlaneSummary {
            worst_state_class: ControlPlaneServiceStateClass::NotApplicable,
            summary_label: "No managed control plane is in scope on this profile.".to_owned(),
            impaired_service_classes: Vec::new(),
            healthy_service_classes: Vec::new(),
            last_sync_at: None,
        },
        data_plane_summary: DataPlaneSummary {
            worst_state_class: DataPlaneCapabilityStateClass::AvailableLocalSafe,
            summary_label: "Local editing, save, search, Git, tasks, docs inspection, export, and diagnostics are available on device."
                .to_owned(),
            impaired_capability_classes: Vec::new(),
            available_local_safe_capability_classes: DataPlaneCapabilityClass::local_core_baseline()
                .to_vec(),
        },
        safest_next_action: SafestNextAction::continue_local(
            "action.strip.individual_local_baseline.continue_local",
        ),
        alternate_actions: Vec::new(),
        consumer_surfaces: vec![
            ConsumerSurfaceClass::AboutPanel,
            ConsumerSurfaceClass::StatusBarCell,
            ConsumerSurfaceClass::DiagnosticsView,
        ],
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe: true,
        linked_profile_summary_ref: Some(summary_id.clone()),
        linked_continuity_packet_ref: None,
        linked_outage_notice_refs: Vec::new(),
        notes: None,
    };
    let (page, _defects) = DeploymentProfilePage::compose(
        "page.deployment.individual_local_baseline",
        profile_summary,
        plane_status_strip,
        Vec::new(),
        Vec::new(),
    );
    page
}

/// Convenience: returns the set of consumer surfaces that consume this
/// page. Used by the support-bundle assembler and by tests asserting
/// at least one read-only surface remains hooked up.
pub fn consumer_surfaces_present(page: &DeploymentProfilePage) -> BTreeSet<ConsumerSurfaceClass> {
    let mut set: BTreeSet<ConsumerSurfaceClass> =
        page.profile_summary.consumer_surfaces.iter().copied().collect();
    for s in &page.plane_status_strip.consumer_surfaces {
        set.insert(*s);
    }
    set
}

pub mod corpus;

#[cfg(test)]
mod tests;
