//! Region, residency, tenant, and key-mode truth for claimed managed rows.
//!
//! This module is the shell-side consumer for the managed/provider-linked
//! truth lane. It does not run a managed control plane, enforce residency, or
//! custody keys. Instead, it projects existing boundary, identity, provider,
//! and service-region records into one inspectable row shape that shell,
//! support, and review surfaces can quote without inventing stronger claims.

use std::collections::BTreeSet;
use std::fmt;

use aureline_auth::{KeyMode, RegionMode, ResidencyMode};
use aureline_provider::ConnectedProviderDescriptor;
use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ManagedTruthSnapshot`].
pub const MANAGED_TRUTH_SNAPSHOT_RECORD_KIND: &str = "managed_truth_snapshot_record";

/// Stable record-kind tag carried by [`ManagedTruthRow`].
pub const MANAGED_TRUTH_ROW_RECORD_KIND: &str = "managed_truth_row_record";

/// Stable record-kind tag carried by [`ManagedTruthExportPacket`].
pub const MANAGED_TRUTH_EXPORT_PACKET_RECORD_KIND: &str = "managed_truth_export_packet";

/// Schema version for snapshot, row, and export-packet payloads.
pub const MANAGED_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Shell surface family that consumes a managed-truth row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedTruthSurfaceClass {
    /// Managed workspace lifecycle, attach, or runtime row.
    ManagedWorkspace,
    /// Optional managed service such as sync, AI gateway, registry, or support ingest.
    ManagedService,
    /// Provider-linked row backed by a user-attached or imported provider descriptor.
    ProviderLinked,
}

impl ManagedTruthSurfaceClass {
    /// Stable token used in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedWorkspace => "managed_workspace",
            Self::ManagedService => "managed_service",
            Self::ProviderLinked => "provider_linked",
        }
    }

    /// Short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ManagedWorkspace => "Managed workspace",
            Self::ManagedService => "Managed service",
            Self::ProviderLinked => "Provider-linked",
        }
    }

    /// True when the row is provider-linked rather than Aureline-managed.
    pub const fn is_provider_linked(self) -> bool {
        matches!(self, Self::ProviderLinked)
    }
}

/// Claim family the row is allowed to make.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedTruthClaimClass {
    /// Shared or community cloud managed lane.
    SharedCommunityCloud,
    /// Vendor-operated enterprise SaaS lane.
    EnterpriseSaas,
    /// Customer-operated self-hosted lane.
    SelfHosted,
    /// Customer-operated regulated or sovereign lane.
    Sovereign,
    /// Provider-linked lane whose boundary is the connected provider, not Aureline.
    ProviderLinked,
}

impl ManagedTruthClaimClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedCommunityCloud => "shared_community_cloud",
            Self::EnterpriseSaas => "enterprise_saas",
            Self::SelfHosted => "self_hosted",
            Self::Sovereign => "sovereign",
            Self::ProviderLinked => "provider_linked",
        }
    }

    /// True when the row claims an Aureline-managed or customer-operated managed lane.
    pub const fn is_managed_claim(self) -> bool {
        !matches!(self, Self::ProviderLinked)
    }
}

/// Operating mode re-exported from the managed-service region/key contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatingModeClass {
    /// Desktop core has no managed prerequisite.
    LocalOnly,
    /// Shared community cloud is in scope.
    SharedCommunityCloud,
    /// Vendor-operated enterprise SaaS is in scope.
    EnterpriseSaas,
    /// Customer-operated self-hosted control plane is in scope.
    SelfHosted,
    /// Customer-operated regulated or sovereign profile is in scope.
    Sovereign,
    /// Provider-linked row does not resolve through an Aureline operating-mode card.
    ProviderLinked,
}

impl OperatingModeClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::SharedCommunityCloud => "shared_community_cloud",
            Self::EnterpriseSaas => "enterprise_saas",
            Self::SelfHosted => "self_hosted",
            Self::Sovereign => "sovereign",
            Self::ProviderLinked => "provider_linked",
        }
    }
}

/// Processing location vocabulary used by displayed managed rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingLocationClass {
    /// Work runs only on the local device.
    OnDeviceOnly,
    /// Work runs in a vendor-operated control plane.
    VendorControlPlane,
    /// Work runs in a customer-operated control plane.
    CustomerControlPlane,
    /// Work may cross customer and vendor control-plane components.
    CustomerOrVendorControlPlane,
    /// Work runs in the connected provider's service boundary.
    ProviderControlPlane,
}

impl ProcessingLocationClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnDeviceOnly => "on_device_only",
            Self::VendorControlPlane => "vendor_control_plane",
            Self::CustomerControlPlane => "customer_control_plane",
            Self::CustomerOrVendorControlPlane => "customer_or_vendor_control_plane",
            Self::ProviderControlPlane => "provider_control_plane",
        }
    }

    /// Short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OnDeviceOnly => "Local device",
            Self::VendorControlPlane => "Vendor control plane",
            Self::CustomerControlPlane => "Customer control plane",
            Self::CustomerOrVendorControlPlane => "Customer/vendor control plane",
            Self::ProviderControlPlane => "Provider control plane",
        }
    }
}

/// Storage location vocabulary used by displayed managed rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageLocationClass {
    /// Data lives on the local disk.
    DeviceLocalDisk,
    /// Data lives in vendor-operated managed storage.
    VendorControlPlaneStorage,
    /// Data lives in customer-operated managed storage.
    CustomerControlPlaneStorage,
    /// Data lives in the connected provider's storage boundary.
    ProviderControlledStorage,
    /// Data lives in a mirror or offline bundle.
    MirrorOrOfflineBundle,
    /// Storage is not applicable for this row.
    NotApplicable,
}

impl StorageLocationClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeviceLocalDisk => "device_local_disk",
            Self::VendorControlPlaneStorage => "vendor_control_plane_storage",
            Self::CustomerControlPlaneStorage => "customer_control_plane_storage",
            Self::ProviderControlledStorage => "provider_controlled_storage",
            Self::MirrorOrOfflineBundle => "mirror_or_offline_bundle",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DeviceLocalDisk => "Local disk",
            Self::VendorControlPlaneStorage => "Vendor managed storage",
            Self::CustomerControlPlaneStorage => "Customer managed storage",
            Self::ProviderControlledStorage => "Provider storage",
            Self::MirrorOrOfflineBundle => "Mirror/offline bundle",
            Self::NotApplicable => "Not applicable",
        }
    }
}

/// Copy posture that keeps managed/provider copies distinct from local artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyPostureClass {
    /// Local-only copy; no managed/provider copy is claimed.
    LocalOnly,
    /// Managed copy exists while local-safe artifacts remain available.
    ManagedCopyWithLocalSafeArtifacts,
    /// Provider copy exists while local draft/export remains available.
    ProviderCopyWithLocalDraft,
    /// Customer-hosted copy exists with no vendor fallback.
    CustomerHostedCopyNoVendorFallback,
    /// Imported or snapshot copy is inspect-only.
    SnapshotOrImportedCopy,
}

impl CopyPostureClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ManagedCopyWithLocalSafeArtifacts => "managed_copy_with_local_safe_artifacts",
            Self::ProviderCopyWithLocalDraft => "provider_copy_with_local_draft",
            Self::CustomerHostedCopyNoVendorFallback => "customer_hosted_copy_no_vendor_fallback",
            Self::SnapshotOrImportedCopy => "snapshot_or_imported_copy",
        }
    }

    /// Short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalOnly => "Local only",
            Self::ManagedCopyWithLocalSafeArtifacts => "Managed copy + local-safe artifacts",
            Self::ProviderCopyWithLocalDraft => "Provider copy + local draft",
            Self::CustomerHostedCopyNoVendorFallback => "Customer-hosted copy, no vendor fallback",
            Self::SnapshotOrImportedCopy => "Snapshot/imported copy",
        }
    }
}

/// Tenant or organization scope vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TenantOrgScopeClass {
    /// Single local user with no managed tenant.
    SingleUserLocal,
    /// Customer tenant boundary.
    CustomerTenant,
    /// Shared multi-tenant service boundary.
    SharedMultiTenant,
    /// Provider account, project, or organization boundary.
    ProviderAccountOrProject,
    /// Tenant boundary must be rechecked before managed writes resume.
    TenantBoundaryRecheckRequired,
    /// Tenant scope does not apply.
    NotApplicable,
}

impl TenantOrgScopeClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleUserLocal => "single_user_local",
            Self::CustomerTenant => "customer_tenant",
            Self::SharedMultiTenant => "shared_multi_tenant",
            Self::ProviderAccountOrProject => "provider_account_or_project",
            Self::TenantBoundaryRecheckRequired => "tenant_boundary_recheck_required",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SingleUserLocal => "Single local user",
            Self::CustomerTenant => "Customer tenant",
            Self::SharedMultiTenant => "Shared multi-tenant",
            Self::ProviderAccountOrProject => "Provider account/project",
            Self::TenantBoundaryRecheckRequired => "Tenant recheck required",
            Self::NotApplicable => "Not applicable",
        }
    }
}

/// Region scope vocabulary across managed and provider-linked rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionScopeClass {
    /// Customer-selected region is pinned by the managed deployment.
    CustomerRegionPinned,
    /// User-owned remote target region applies.
    RemoteTargetRegion,
    /// Connected provider's default region/residency applies.
    ProviderDefaultDisclosed,
    /// Region boundary must be rechecked before managed writes resume.
    BoundaryRecheckRequired,
    /// Region is unknown and must remain visibly unresolved.
    Unknown,
    /// Region does not apply.
    NotApplicable,
}

impl RegionScopeClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CustomerRegionPinned => "customer_region_pinned",
            Self::RemoteTargetRegion => "remote_target_region",
            Self::ProviderDefaultDisclosed => "provider_default_disclosed",
            Self::BoundaryRecheckRequired => "boundary_recheck_required",
            Self::Unknown => "unknown",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CustomerRegionPinned => "Customer region pinned",
            Self::RemoteTargetRegion => "Remote target region",
            Self::ProviderDefaultDisclosed => "Provider default disclosed",
            Self::BoundaryRecheckRequired => "Region recheck required",
            Self::Unknown => "Unknown",
            Self::NotApplicable => "Not applicable",
        }
    }

    fn is_disclosed(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::Unknown)
    }
}

/// Data-residency disclosure class reused from the locality/key-mode seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataResidencyDisclosureClass {
    /// Data remains on the local device.
    ResidencyLocalDeviceOnly,
    /// Data may rest on a user-owned remote target.
    ResidencyUserOwnedRemoteTarget,
    /// Connected provider default applies and is disclosed, not enforced by Aureline.
    ResidencyProviderDefault,
    /// Managed tenant documents a region; this row does not certify enforcement.
    ResidencyManagedTenantDocumentedRegion,
    /// Residency is unknown and must remain visibly unknown.
    ResidencyUnknown,
}

impl DataResidencyDisclosureClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResidencyLocalDeviceOnly => "residency_local_device_only",
            Self::ResidencyUserOwnedRemoteTarget => "residency_user_owned_remote_target",
            Self::ResidencyProviderDefault => "residency_provider_default",
            Self::ResidencyManagedTenantDocumentedRegion => {
                "residency_managed_tenant_documented_region"
            }
            Self::ResidencyUnknown => "residency_unknown",
        }
    }
}

/// Residency scope shown by region/residency strip rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidencyScopeClass {
    /// Customer region is pinned.
    CustomerRegionPinned,
    /// Regulated jurisdiction posture is active.
    RegulatedJurisdiction,
    /// Cross-region egress is audited and disclosed.
    CrossRegionAuditedEgress,
    /// Connected provider's residency/default applies.
    ProviderDefaultDisclosed,
    /// Residency boundary must be rechecked before managed writes resume.
    BoundaryRecheckRequired,
    /// Residency is unknown and must remain visibly unresolved.
    Unknown,
    /// Residency does not apply.
    NotApplicable,
}

impl ResidencyScopeClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CustomerRegionPinned => "customer_region_pinned",
            Self::RegulatedJurisdiction => "regulated_jurisdiction",
            Self::CrossRegionAuditedEgress => "cross_region_audited_egress",
            Self::ProviderDefaultDisclosed => "provider_default_disclosed",
            Self::BoundaryRecheckRequired => "boundary_recheck_required",
            Self::Unknown => "unknown",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Key ownership or storage mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyModeClass {
    /// OS credential store or keychain.
    OsStore,
    /// Vendor-managed key material.
    VendorManaged,
    /// Customer-managed key material.
    CustomerManaged,
    /// Offline trust-root posture.
    OfflineTrustRoot,
    /// Connected provider manages the key.
    ProviderManaged,
    /// User supplies BYOK material treated as opaque by Aureline.
    ByokUserManaged,
    /// Key mode is unknown and must remain visibly unresolved.
    Unknown,
    /// Key mode does not apply.
    NotApplicable,
}

impl KeyModeClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsStore => "os_store",
            Self::VendorManaged => "vendor_managed",
            Self::CustomerManaged => "customer_managed",
            Self::OfflineTrustRoot => "offline_trust_root",
            Self::ProviderManaged => "provider_managed",
            Self::ByokUserManaged => "byok_user_managed",
            Self::Unknown => "unknown",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OsStore => "OS credential store",
            Self::VendorManaged => "Vendor-managed keys",
            Self::CustomerManaged => "Customer-managed keys",
            Self::OfflineTrustRoot => "Offline trust root",
            Self::ProviderManaged => "Provider-managed keys",
            Self::ByokUserManaged => "BYOK user-managed",
            Self::Unknown => "Unknown",
            Self::NotApplicable => "Not applicable",
        }
    }

    fn is_disclosed(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::Unknown)
    }
}

/// Key state vocabulary surfaced by the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyStateClass {
    /// Key binding is current.
    BoundAndCurrent,
    /// Key binding is pending rotation.
    BoundPendingRotation,
    /// Key rotation is in progress.
    RotationInProgress,
    /// Rotation completed and a boundary recheck is required.
    RotationCompletedRecheckRequired,
    /// Key was revoked and a boundary recheck is required.
    RevokedRecheckRequired,
    /// Key binding mismatches active state and requires recheck.
    MismatchRecheckRequired,
    /// Customer-managed key path is unreachable.
    CustomerManagedUnreachable,
    /// Key state does not apply.
    NotApplicable,
}

impl KeyStateClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundAndCurrent => "bound_and_current",
            Self::BoundPendingRotation => "bound_pending_rotation",
            Self::RotationInProgress => "rotation_in_progress",
            Self::RotationCompletedRecheckRequired => "rotation_completed_recheck_required",
            Self::RevokedRecheckRequired => "revoked_recheck_required",
            Self::MismatchRecheckRequired => "mismatch_recheck_required",
            Self::CustomerManagedUnreachable => "customer_managed_unreachable",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the state blocks or pauses managed action families.
    pub const fn requires_boundary_action(self) -> bool {
        !matches!(self, Self::BoundAndCurrent | Self::NotApplicable)
    }
}

/// Control-plane or data-plane health state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaneStateClass {
    /// Plane is healthy.
    Healthy,
    /// Last-known-good or imported snapshot is being used.
    StaleCache,
    /// Plane is degraded but not fully unavailable.
    Degraded,
    /// Plane is unavailable.
    Unavailable,
    /// Boundary recheck is required before writes resume.
    BoundaryRecheckRequired,
    /// Plane is not applicable.
    NotApplicable,
}

impl PlaneStateClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::StaleCache => "stale_cache",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::BoundaryRecheckRequired => "boundary_recheck_required",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Short display label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Healthy => "Healthy",
            Self::StaleCache => "Stale cache",
            Self::Degraded => "Degraded",
            Self::Unavailable => "Unavailable",
            Self::BoundaryRecheckRequired => "Boundary recheck required",
            Self::NotApplicable => "Not applicable",
        }
    }

    /// True when the plane is impaired or no longer authoritative.
    pub const fn is_impaired(self) -> bool {
        matches!(
            self,
            Self::StaleCache | Self::Degraded | Self::Unavailable | Self::BoundaryRecheckRequired
        )
    }
}

/// Action-family vocabulary named by key or plane impairments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffectedActionFamilyClass {
    /// Settings or workspace sync.
    SyncActionFamily,
    /// Marketplace publish.
    MarketplacePublishActionFamily,
    /// Marketplace install.
    MarketplaceInstallActionFamily,
    /// Collaboration publish.
    CollaborationPublishActionFamily,
    /// Remote attach.
    RemoteAttachActionFamily,
    /// Remote execute.
    RemoteExecuteActionFamily,
    /// Managed AI inference.
    AiInferenceActionFamily,
    /// Managed AI evidence retention.
    AiEvidenceRetentionActionFamily,
    /// Telemetry export.
    TelemetryExportActionFamily,
    /// Support export.
    SupportExportActionFamily,
    /// Offboarding export.
    OffboardingExportActionFamily,
    /// Policy distribution.
    PolicyDistributionActionFamily,
    /// Identity refresh.
    IdentityRefreshActionFamily,
    /// Provider publish, rerun, cancel, or retry.
    ProviderMutationActionFamily,
    /// Provider open-in-provider or browser handoff.
    ProviderHandoffActionFamily,
}

impl AffectedActionFamilyClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncActionFamily => "sync_action_family",
            Self::MarketplacePublishActionFamily => "marketplace_publish_action_family",
            Self::MarketplaceInstallActionFamily => "marketplace_install_action_family",
            Self::CollaborationPublishActionFamily => "collaboration_publish_action_family",
            Self::RemoteAttachActionFamily => "remote_attach_action_family",
            Self::RemoteExecuteActionFamily => "remote_execute_action_family",
            Self::AiInferenceActionFamily => "ai_inference_action_family",
            Self::AiEvidenceRetentionActionFamily => "ai_evidence_retention_action_family",
            Self::TelemetryExportActionFamily => "telemetry_export_action_family",
            Self::SupportExportActionFamily => "support_export_action_family",
            Self::OffboardingExportActionFamily => "offboarding_export_action_family",
            Self::PolicyDistributionActionFamily => "policy_distribution_action_family",
            Self::IdentityRefreshActionFamily => "identity_refresh_action_family",
            Self::ProviderMutationActionFamily => "provider_mutation_action_family",
            Self::ProviderHandoffActionFamily => "provider_handoff_action_family",
        }
    }
}

/// Fail posture paired with affected action families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailPostureClass {
    /// Managed action is paused; local-safe work continues.
    FailClosedManagedOnly,
    /// Local-safe equivalent may continue.
    FailOpenLocalSafe,
    /// Local-safe equivalent may continue with a visible label.
    FailOpenLocalSafeWithLabel,
    /// Boundary recheck is required before managed writes resume.
    BoundaryRecheckRequired,
    /// Provider snapshot may be inspected only.
    InspectOnlyProviderSnapshot,
    /// User can leave the product and open the provider directly.
    OpenInProviderOnly,
    /// No fail posture applies.
    NotApplicable,
}

impl FailPostureClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FailClosedManagedOnly => "fail_closed_managed_only",
            Self::FailOpenLocalSafe => "fail_open_local_safe",
            Self::FailOpenLocalSafeWithLabel => "fail_open_local_safe_with_label",
            Self::BoundaryRecheckRequired => "boundary_recheck_required",
            Self::InspectOnlyProviderSnapshot => "inspect_only_provider_snapshot",
            Self::OpenInProviderOnly => "open_in_provider_only",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Explicit sovereignty boundary disclosed by the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SovereigntyBoundaryClass {
    /// No managed boundary exists.
    NoManagedBoundary,
    /// Connected provider default applies and is disclosed.
    ProviderDefaultDisclosed,
    /// Vendor-managed lane documents customer-pinned region.
    CustomerPinnedManaged,
    /// Customer operates the control plane or storage.
    CustomerOperatedSelfHosted,
    /// Regulated jurisdiction posture is active for this row.
    RegulatedJurisdiction,
    /// Boundary is unknown and review is required.
    UnknownRequiresReview,
}

impl SovereigntyBoundaryClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoManagedBoundary => "no_managed_boundary",
            Self::ProviderDefaultDisclosed => "provider_default_disclosed",
            Self::CustomerPinnedManaged => "customer_pinned_managed",
            Self::CustomerOperatedSelfHosted => "customer_operated_self_hosted",
            Self::RegulatedJurisdiction => "regulated_jurisdiction",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// True when a provider-linked row would overclaim this boundary.
    pub const fn overclaims_provider_boundary(self) -> bool {
        matches!(
            self,
            Self::CustomerPinnedManaged
                | Self::CustomerOperatedSelfHosted
                | Self::RegulatedJurisdiction
        )
    }
}

/// Upstream records consumed by a managed-truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedTruthRefs {
    /// Boundary-manifest capability ref from governance artifacts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_manifest_capability_ref: Option<String>,
    /// Identity-mode baseline row ref when identity/policy state matters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_mode_row_ref: Option<String>,
    /// Service `region_key_state_record` ref when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region_key_state_ref: Option<String>,
    /// Service `operating_mode_card_record` ref when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operating_mode_card_ref: Option<String>,
    /// Connected-provider descriptor ref for provider-linked rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_descriptor_ref: Option<String>,
    /// Connected-provider registry packet ref for provider-linked rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_registry_packet_ref: Option<String>,
    /// Additional docs, schema, fixture, or artifact refs that reconstruct the row.
    #[serde(default)]
    pub source_refs: Vec<String>,
}

impl ManagedTruthRefs {
    fn has_managed_refs(&self) -> bool {
        self.boundary_manifest_capability_ref
            .as_deref()
            .is_some_and(non_empty)
            && self.identity_mode_row_ref.as_deref().is_some_and(non_empty)
            && self.region_key_state_ref.as_deref().is_some_and(non_empty)
            && self
                .operating_mode_card_ref
                .as_deref()
                .is_some_and(non_empty)
    }

    fn has_provider_refs(&self) -> bool {
        self.provider_descriptor_ref
            .as_deref()
            .is_some_and(non_empty)
            && self
                .provider_registry_packet_ref
                .as_deref()
                .is_some_and(non_empty)
    }
}

/// Region and residency truth rendered on the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionResidencyTruth {
    /// Region scope token.
    pub region_scope: RegionScopeClass,
    /// Opaque region reference; raw cloud-region ids must not cross this boundary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region_ref: Option<String>,
    /// Residency scope token.
    pub residency_scope_class: ResidencyScopeClass,
    /// Locality seed residency disclosure class.
    pub data_residency_disclosure_class: DataResidencyDisclosureClass,
    /// Short reviewable summary.
    pub residency_summary: String,
}

/// Tenant truth rendered on the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TenantTruth {
    /// Tenant or organization scope.
    pub tenant_org_scope: TenantOrgScopeClass,
    /// Opaque tenant/account/project ref; raw names must not cross this boundary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_ref: Option<String>,
    /// Short reviewable summary.
    pub tenant_summary: String,
}

/// Storage and copy posture rendered on the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCopyTruth {
    /// Where processing occurs.
    pub processing_location: ProcessingLocationClass,
    /// Where data is stored or copied.
    pub storage_location: StorageLocationClass,
    /// Copy posture visible to the user.
    pub copy_posture: CopyPostureClass,
    /// Retention class token quoted from the upstream row.
    pub retention_class: String,
    /// Short reviewable summary.
    pub copy_summary: String,
}

/// Key ownership and key-state truth rendered on the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyModeTruth {
    /// Key mode token.
    pub key_mode: KeyModeClass,
    /// Key state token.
    pub key_state_class: KeyStateClass,
    /// Opaque key ref; raw key bytes, fingerprints, or cert bodies must not cross this boundary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_ref: Option<String>,
    /// Short reviewable summary.
    pub key_state_summary: String,
    /// Minimum action families bounded by the current key state.
    #[serde(default)]
    pub affected_action_families: Vec<AffectedActionFamilyClass>,
    /// Fail posture for affected action families.
    pub fail_posture: FailPostureClass,
}

/// Control-plane and data-plane state rendered on the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaneImpairmentTruth {
    /// Control-plane state.
    pub control_plane_state: PlaneStateClass,
    /// Data-plane state.
    pub data_plane_state: PlaneStateClass,
    /// Short control-plane summary.
    pub control_plane_summary: String,
    /// Short data-plane summary.
    pub data_plane_summary: String,
    /// Action families bounded by this plane state.
    #[serde(default)]
    pub affected_action_families: Vec<AffectedActionFamilyClass>,
    /// Fail posture for affected action families.
    pub fail_posture: FailPostureClass,
    /// Last control-plane sync time or monotonic fixture time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_control_plane_sync_at: Option<String>,
    /// Last data-plane probe time or monotonic fixture time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_data_plane_probe_at: Option<String>,
}

impl PlaneImpairmentTruth {
    fn distinguishes_planes(&self) -> bool {
        self.control_plane_state != self.data_plane_state
            || self.control_plane_summary != self.data_plane_summary
    }

    fn has_relevant_impairment(&self) -> bool {
        self.control_plane_state.is_impaired() || self.data_plane_state.is_impaired()
    }
}

/// Local-safe continuation truth kept visible beside managed/provider state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalContinuityTruth {
    /// Whether local core remains available.
    pub local_core_available: bool,
    /// Export-safe local capability summaries that remain available.
    #[serde(default)]
    pub retained_local_safe_capabilities: Vec<String>,
    /// Export-safe managed/provider-only summaries that are blocked or narrowed.
    #[serde(default)]
    pub blocked_managed_or_provider_capabilities: Vec<String>,
    /// Short reviewable summary.
    pub continuity_summary: String,
}

/// Sovereignty and residual-dependency truth rendered on the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SovereigntyTruth {
    /// Actual sovereignty boundary for this row.
    pub sovereignty_boundary: SovereigntyBoundaryClass,
    /// Residual vendor/provider dependencies that remain for this row.
    #[serde(default)]
    pub residual_dependency_refs: Vec<String>,
    /// Short reviewable summary.
    pub sovereignty_summary: String,
}

/// Display-copy invariants that must stay false or true as declared.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedTruthDisplayCopy {
    /// False when the row does not imply the whole product is unavailable.
    pub whole_product_failure_implied: bool,
    /// False when the row does not imply a stronger sovereignty boundary than active truth.
    pub stronger_sovereignty_boundary_implied: bool,
    /// False when managed writes cannot silently fail open under unknown state.
    pub silent_fail_open_under_unknown_state: bool,
    /// False when no plaintext secret fallback is implied.
    pub plaintext_secret_fallback_implied: bool,
}

impl ManagedTruthDisplayCopy {
    fn is_safe(&self) -> bool {
        !self.whole_product_failure_implied
            && !self.stronger_sovereignty_boundary_implied
            && !self.silent_fail_open_under_unknown_state
            && !self.plaintext_secret_fallback_implied
    }
}

/// Request used to project a connected-provider descriptor into a shell truth row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderLinkedManagedTruthRowRequest<'a> {
    /// Provider descriptor carrying region, residency, and key-mode truth.
    pub descriptor: &'a ConnectedProviderDescriptor,
    /// Connected-provider registry packet ref the descriptor came from.
    pub provider_registry_packet_ref: &'a str,
    /// Stable row id for the shell row.
    pub row_id: &'a str,
    /// Opaque shell surface ref that will render the row.
    pub surface_ref: &'a str,
    /// Short row title.
    pub title: &'a str,
    /// Reviewable row summary.
    pub summary: &'a str,
    /// Additional docs, schema, fixture, or artifact refs that reconstruct the row.
    pub source_refs: Vec<String>,
}

/// One claimed managed/provider-linked truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedTruthRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Row schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Surface family.
    pub surface_class: ManagedTruthSurfaceClass,
    /// Claim family.
    pub claim_class: ManagedTruthClaimClass,
    /// Operating mode class.
    pub operating_mode_class: OperatingModeClass,
    /// Opaque source surface ref.
    pub surface_ref: String,
    /// Short title.
    pub title: String,
    /// Reviewable summary.
    pub summary: String,
    /// Upstream refs the row consumes.
    pub refs: ManagedTruthRefs,
    /// Region/residency truth.
    pub region_residency: RegionResidencyTruth,
    /// Tenant truth.
    pub tenant: TenantTruth,
    /// Storage/copy posture truth.
    pub storage_copy: StorageCopyTruth,
    /// Key mode/state truth.
    pub key: KeyModeTruth,
    /// Control-plane/data-plane truth.
    pub planes: PlaneImpairmentTruth,
    /// Local-safe continuation truth.
    pub local_continuity: LocalContinuityTruth,
    /// Sovereignty and residual-dependency truth.
    pub sovereignty: SovereigntyTruth,
    /// Display-copy safety invariants.
    pub display_copy: ManagedTruthDisplayCopy,
}

impl ManagedTruthRow {
    /// Builds the dense display row shown by shell, support, or review surfaces.
    pub fn display_state(&self) -> ManagedTruthDisplayRow {
        ManagedTruthDisplayRow {
            row_id: self.row_id.clone(),
            surface_class: self.surface_class,
            claim_class: self.claim_class,
            title: self.title.clone(),
            region_label: self.region_residency.region_scope.label().to_owned(),
            tenant_label: self.tenant.tenant_org_scope.label().to_owned(),
            storage_label: self.storage_copy.storage_location.label().to_owned(),
            copy_label: self.storage_copy.copy_posture.label().to_owned(),
            key_mode_label: self.key.key_mode.label().to_owned(),
            control_plane_label: self.planes.control_plane_state.label().to_owned(),
            data_plane_label: self.planes.data_plane_state.label().to_owned(),
            local_continuity_summary: self.local_continuity.continuity_summary.clone(),
            boundary_summary: self.sovereignty.sovereignty_summary.clone(),
            primary_action: self.primary_action(),
        }
    }

    /// Projects a connected-provider descriptor into a provider-linked shell row.
    pub fn from_provider_descriptor(request: ProviderLinkedManagedTruthRowRequest<'_>) -> Self {
        let descriptor = request.descriptor;
        let plane_state = provider_freshness_to_plane_state(descriptor.freshness.freshness_class);
        let plane_is_impaired = plane_state.is_impaired();
        let mut source_refs = vec![
            "docs/managed/region_residency_alpha.md".to_owned(),
            "docs/providers/connected_provider_alpha.md".to_owned(),
            descriptor.connected_provider_record_ref.clone(),
        ];
        source_refs.extend(request.source_refs);

        Self {
            record_kind: MANAGED_TRUTH_ROW_RECORD_KIND.to_owned(),
            schema_version: MANAGED_TRUTH_SCHEMA_VERSION,
            row_id: request.row_id.to_owned(),
            surface_class: ManagedTruthSurfaceClass::ProviderLinked,
            claim_class: ManagedTruthClaimClass::ProviderLinked,
            operating_mode_class: OperatingModeClass::ProviderLinked,
            surface_ref: request.surface_ref.to_owned(),
            title: request.title.to_owned(),
            summary: request.summary.to_owned(),
            refs: ManagedTruthRefs {
                boundary_manifest_capability_ref: None,
                identity_mode_row_ref: None,
                region_key_state_ref: None,
                operating_mode_card_ref: None,
                provider_descriptor_ref: Some(descriptor.descriptor_id.clone()),
                provider_registry_packet_ref: Some(request.provider_registry_packet_ref.to_owned()),
                source_refs,
            },
            region_residency: RegionResidencyTruth {
                region_scope: region_mode_to_scope(descriptor.region_mode),
                region_ref: None,
                residency_scope_class: residency_mode_to_scope(
                    descriptor.region_mode,
                    descriptor.residency_mode,
                ),
                data_residency_disclosure_class: residency_mode_to_disclosure(
                    descriptor.residency_mode,
                ),
                residency_summary: format!(
                    "{}; {}.",
                    descriptor.region_mode.label(),
                    descriptor.residency_mode.label()
                ),
            },
            tenant: TenantTruth {
                tenant_org_scope: TenantOrgScopeClass::ProviderAccountOrProject,
                tenant_ref: Some(descriptor.source.tenant_or_org_scope_ref.clone()),
                tenant_summary:
                    "Provider account/project boundary; raw provider account names are excluded."
                        .to_owned(),
            },
            storage_copy: StorageCopyTruth {
                processing_location: ProcessingLocationClass::ProviderControlPlane,
                storage_location: StorageLocationClass::ProviderControlledStorage,
                copy_posture: CopyPostureClass::ProviderCopyWithLocalDraft,
                retention_class: "provider_retention_policy_applies".to_owned(),
                copy_summary:
                    "Provider copies remain in the connected-provider boundary; local drafts stay available."
                        .to_owned(),
            },
            key: KeyModeTruth {
                key_mode: key_mode_to_class(descriptor.key_mode),
                key_state_class: KeyStateClass::BoundAndCurrent,
                key_ref: None,
                key_state_summary: format!("{} posture applies.", descriptor.key_mode.label()),
                affected_action_families: vec![],
                fail_posture: FailPostureClass::NotApplicable,
            },
            planes: PlaneImpairmentTruth {
                control_plane_state: plane_state,
                data_plane_state: PlaneStateClass::Healthy,
                control_plane_summary: provider_freshness_summary(
                    descriptor.freshness.freshness_class,
                    descriptor.freshness.degraded_reason.as_deref(),
                ),
                data_plane_summary: "Local task truth and local drafts remain available."
                    .to_owned(),
                affected_action_families: if plane_is_impaired {
                    vec![AffectedActionFamilyClass::ProviderMutationActionFamily]
                } else {
                    vec![]
                },
                fail_posture: if plane_is_impaired {
                    FailPostureClass::InspectOnlyProviderSnapshot
                } else {
                    FailPostureClass::NotApplicable
                },
                last_control_plane_sync_at: descriptor.freshness.observed_at.clone(),
                last_data_plane_probe_at: descriptor.freshness.observed_at.clone(),
            },
            local_continuity: LocalContinuityTruth {
                local_core_available: true,
                retained_local_safe_capabilities: vec![
                    "Continue local work and local provider drafts.".to_owned(),
                ],
                blocked_managed_or_provider_capabilities: if plane_is_impaired {
                    vec![
                        "Provider mutation remains inspect-only until provider truth is refreshed."
                            .to_owned(),
                    ]
                } else {
                    vec![]
                },
                continuity_summary:
                    "Local work remains available while provider boundary truth is displayed."
                        .to_owned(),
            },
            sovereignty: SovereigntyTruth {
                sovereignty_boundary: if descriptor.region_mode.is_unknown()
                    || descriptor.residency_mode.is_unknown()
                {
                    SovereigntyBoundaryClass::UnknownRequiresReview
                } else {
                    SovereigntyBoundaryClass::ProviderDefaultDisclosed
                },
                residual_dependency_refs: vec![descriptor.connected_provider_record_ref.clone()],
                sovereignty_summary:
                    "Provider-linked row; the provider boundary is disclosed without a sovereign claim."
                        .to_owned(),
            },
            display_copy: ManagedTruthDisplayCopy {
                whole_product_failure_implied: false,
                stronger_sovereignty_boundary_implied: false,
                silent_fail_open_under_unknown_state: false,
                plaintext_secret_fallback_implied: false,
            },
        }
    }

    /// Validates row-level safety and disclosure invariants.
    pub fn validate(&self) -> Result<(), ManagedTruthValidationError> {
        if self.record_kind != MANAGED_TRUTH_ROW_RECORD_KIND {
            return Err(ManagedTruthValidationError::WrongRowRecordKind {
                row_id: self.row_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if self.schema_version != MANAGED_TRUTH_SCHEMA_VERSION {
            return Err(ManagedTruthValidationError::WrongRowSchemaVersion {
                row_id: self.row_id.clone(),
                actual: self.schema_version,
            });
        }
        for (field, value) in [
            ("row_id", self.row_id.as_str()),
            ("surface_ref", self.surface_ref.as_str()),
            ("title", self.title.as_str()),
            ("summary", self.summary.as_str()),
            (
                "residency_summary",
                self.region_residency.residency_summary.as_str(),
            ),
            ("tenant_summary", self.tenant.tenant_summary.as_str()),
            ("copy_summary", self.storage_copy.copy_summary.as_str()),
            (
                "retention_class",
                self.storage_copy.retention_class.as_str(),
            ),
            ("key_state_summary", self.key.key_state_summary.as_str()),
            (
                "control_plane_summary",
                self.planes.control_plane_summary.as_str(),
            ),
            (
                "data_plane_summary",
                self.planes.data_plane_summary.as_str(),
            ),
            (
                "continuity_summary",
                self.local_continuity.continuity_summary.as_str(),
            ),
            (
                "sovereignty_summary",
                self.sovereignty.sovereignty_summary.as_str(),
            ),
        ] {
            if !non_empty(value) {
                return Err(ManagedTruthValidationError::MissingRequiredField {
                    row_id: self.row_id.clone(),
                    field,
                });
            }
        }

        if self.claim_class.is_managed_claim() && !self.refs.has_managed_refs() {
            return Err(ManagedTruthValidationError::MissingManagedRefs {
                row_id: self.row_id.clone(),
            });
        }
        if self.claim_class == ManagedTruthClaimClass::ProviderLinked
            && !self.refs.has_provider_refs()
        {
            return Err(ManagedTruthValidationError::MissingProviderRefs {
                row_id: self.row_id.clone(),
            });
        }
        if self.surface_class.is_provider_linked()
            != (self.claim_class == ManagedTruthClaimClass::ProviderLinked)
        {
            return Err(ManagedTruthValidationError::SurfaceClaimMismatch {
                row_id: self.row_id.clone(),
            });
        }
        if self.claim_class.is_managed_claim()
            && (!self.region_residency.region_scope.is_disclosed()
                || self.tenant.tenant_org_scope == TenantOrgScopeClass::NotApplicable
                || !self.key.key_mode.is_disclosed()
                || self.storage_copy.storage_location == StorageLocationClass::NotApplicable)
        {
            return Err(
                ManagedTruthValidationError::ManagedClaimMissingBoundaryTruth {
                    row_id: self.row_id.clone(),
                },
            );
        }
        if self.claim_class == ManagedTruthClaimClass::ProviderLinked
            && (!self.region_residency.region_scope.is_disclosed()
                || self.region_residency.data_residency_disclosure_class
                    == DataResidencyDisclosureClass::ResidencyUnknown
                || !self.key.key_mode.is_disclosed())
        {
            return Err(
                ManagedTruthValidationError::ProviderLinkedUnknownBoundaryTruth {
                    row_id: self.row_id.clone(),
                },
            );
        }
        if self.claim_class == ManagedTruthClaimClass::ProviderLinked
            && self
                .sovereignty
                .sovereignty_boundary
                .overclaims_provider_boundary()
        {
            return Err(
                ManagedTruthValidationError::ProviderLinkedOverclaimsSovereignty {
                    row_id: self.row_id.clone(),
                    sovereignty_boundary: self.sovereignty.sovereignty_boundary,
                },
            );
        }
        if self.claim_class == ManagedTruthClaimClass::ProviderLinked
            && matches!(
                self.region_residency.data_residency_disclosure_class,
                DataResidencyDisclosureClass::ResidencyManagedTenantDocumentedRegion
            )
        {
            return Err(
                ManagedTruthValidationError::ProviderLinkedUsesManagedResidency {
                    row_id: self.row_id.clone(),
                },
            );
        }
        if self.key.key_state_class.requires_boundary_action()
            && (self.key.affected_action_families.is_empty()
                || self.key.fail_posture == FailPostureClass::NotApplicable)
        {
            return Err(
                ManagedTruthValidationError::KeyStateMissingAffectedActions {
                    row_id: self.row_id.clone(),
                },
            );
        }
        if self.planes.has_relevant_impairment()
            && (self.planes.affected_action_families.is_empty()
                || self.planes.fail_posture == FailPostureClass::NotApplicable)
        {
            return Err(ManagedTruthValidationError::PlaneImpairmentMissingScope {
                row_id: self.row_id.clone(),
            });
        }
        if self.planes.has_relevant_impairment() && !self.planes.distinguishes_planes() {
            return Err(
                ManagedTruthValidationError::PlaneImpairmentNotDistinguished {
                    row_id: self.row_id.clone(),
                },
            );
        }
        if !self.local_continuity.local_core_available
            || self
                .local_continuity
                .retained_local_safe_capabilities
                .is_empty()
        {
            return Err(ManagedTruthValidationError::MissingLocalContinuity {
                row_id: self.row_id.clone(),
            });
        }
        if !self.display_copy.is_safe() {
            return Err(ManagedTruthValidationError::UnsafeDisplayCopy {
                row_id: self.row_id.clone(),
            });
        }
        if self.refs.source_refs.is_empty() {
            return Err(ManagedTruthValidationError::MissingSourceRefs {
                row_id: self.row_id.clone(),
            });
        }

        Ok(())
    }

    fn primary_action(&self) -> String {
        if self.planes.control_plane_state == PlaneStateClass::Unavailable {
            "Continue local-safe work; retry control-plane sync".to_owned()
        } else if self.planes.data_plane_state.is_impaired() {
            "Inspect data-plane state before retrying managed action".to_owned()
        } else if self.key.key_state_class.requires_boundary_action() {
            "Run key or boundary recheck".to_owned()
        } else if self.claim_class == ManagedTruthClaimClass::ProviderLinked {
            "Inspect provider boundary and open provider if needed".to_owned()
        } else {
            "Inspect managed boundary details".to_owned()
        }
    }

    fn combined_source_refs(&self) -> Vec<String> {
        let mut refs = BTreeSet::new();
        refs.extend(self.refs.source_refs.iter().cloned());
        for item in [
            self.refs.boundary_manifest_capability_ref.as_ref(),
            self.refs.identity_mode_row_ref.as_ref(),
            self.refs.region_key_state_ref.as_ref(),
            self.refs.operating_mode_card_ref.as_ref(),
            self.refs.provider_descriptor_ref.as_ref(),
            self.refs.provider_registry_packet_ref.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            refs.insert(item.clone());
        }
        refs.into_iter().collect()
    }
}

fn region_mode_to_scope(mode: RegionMode) -> RegionScopeClass {
    match mode {
        RegionMode::CustomerRegionPinned => RegionScopeClass::CustomerRegionPinned,
        RegionMode::RemoteTargetRegion => RegionScopeClass::RemoteTargetRegion,
        RegionMode::ProviderDefaultDisclosed => RegionScopeClass::ProviderDefaultDisclosed,
        RegionMode::BoundaryRecheckRequired => RegionScopeClass::BoundaryRecheckRequired,
        RegionMode::Unknown => RegionScopeClass::Unknown,
    }
}

fn residency_mode_to_scope(
    region_mode: RegionMode,
    residency_mode: ResidencyMode,
) -> ResidencyScopeClass {
    match residency_mode {
        ResidencyMode::LocalDeviceOnly | ResidencyMode::UserOwnedRemoteTarget => {
            ResidencyScopeClass::NotApplicable
        }
        ResidencyMode::ProviderDefault => ResidencyScopeClass::ProviderDefaultDisclosed,
        ResidencyMode::ManagedTenantDocumentedRegion => {
            if region_mode == RegionMode::CustomerRegionPinned {
                ResidencyScopeClass::CustomerRegionPinned
            } else {
                ResidencyScopeClass::BoundaryRecheckRequired
            }
        }
        ResidencyMode::Unknown => ResidencyScopeClass::Unknown,
    }
}

fn residency_mode_to_disclosure(mode: ResidencyMode) -> DataResidencyDisclosureClass {
    match mode {
        ResidencyMode::LocalDeviceOnly => DataResidencyDisclosureClass::ResidencyLocalDeviceOnly,
        ResidencyMode::UserOwnedRemoteTarget => {
            DataResidencyDisclosureClass::ResidencyUserOwnedRemoteTarget
        }
        ResidencyMode::ProviderDefault => DataResidencyDisclosureClass::ResidencyProviderDefault,
        ResidencyMode::ManagedTenantDocumentedRegion => {
            DataResidencyDisclosureClass::ResidencyManagedTenantDocumentedRegion
        }
        ResidencyMode::Unknown => DataResidencyDisclosureClass::ResidencyUnknown,
    }
}

fn key_mode_to_class(mode: KeyMode) -> KeyModeClass {
    match mode {
        KeyMode::OsStore => KeyModeClass::OsStore,
        KeyMode::VendorManaged => KeyModeClass::VendorManaged,
        KeyMode::CustomerManaged => KeyModeClass::CustomerManaged,
        KeyMode::OfflineTrustRoot => KeyModeClass::OfflineTrustRoot,
        KeyMode::ProviderManaged => KeyModeClass::ProviderManaged,
        KeyMode::ByokUserManaged => KeyModeClass::ByokUserManaged,
        KeyMode::Unknown => KeyModeClass::Unknown,
    }
}

fn provider_freshness_to_plane_state(
    freshness: aureline_provider::FreshnessLabel,
) -> PlaneStateClass {
    match freshness {
        aureline_provider::FreshnessLabel::Fresh => PlaneStateClass::Healthy,
        aureline_provider::FreshnessLabel::StaleWithinWindow => PlaneStateClass::StaleCache,
        aureline_provider::FreshnessLabel::ExpiredBeyondWindow
        | aureline_provider::FreshnessLabel::RevokedOrDisconnected => PlaneStateClass::Unavailable,
        aureline_provider::FreshnessLabel::NeverObserved => {
            PlaneStateClass::BoundaryRecheckRequired
        }
    }
}

fn provider_freshness_summary(
    freshness: aureline_provider::FreshnessLabel,
    degraded_reason: Option<&str>,
) -> String {
    degraded_reason.map_or_else(
        || format!("Provider freshness is {}.", freshness.as_str()),
        str::to_owned,
    )
}

/// Dense display row projected from [`ManagedTruthRow`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedTruthDisplayRow {
    /// Stable row id.
    pub row_id: String,
    /// Surface class.
    pub surface_class: ManagedTruthSurfaceClass,
    /// Claim class.
    pub claim_class: ManagedTruthClaimClass,
    /// Display title.
    pub title: String,
    /// Region label.
    pub region_label: String,
    /// Tenant label.
    pub tenant_label: String,
    /// Storage label.
    pub storage_label: String,
    /// Copy posture label.
    pub copy_label: String,
    /// Key-mode label.
    pub key_mode_label: String,
    /// Control-plane label.
    pub control_plane_label: String,
    /// Data-plane label.
    pub data_plane_label: String,
    /// Local-continuity summary.
    pub local_continuity_summary: String,
    /// Boundary summary.
    pub boundary_summary: String,
    /// First safe action.
    pub primary_action: String,
}

/// Top-level snapshot for the managed-truth alpha lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedTruthSnapshot {
    /// Record discriminator.
    pub record_kind: String,
    /// Snapshot schema version.
    pub schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Emission timestamp or monotonic fixture time.
    pub emitted_at: String,
    /// Rows projected into display and support/review exports.
    #[serde(default)]
    pub rows: Vec<ManagedTruthRow>,
    /// Optional redaction-safe note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ManagedTruthSnapshot {
    /// Validates snapshot and row-level invariants.
    pub fn validate(&self) -> Result<(), ManagedTruthValidationError> {
        if self.record_kind != MANAGED_TRUTH_SNAPSHOT_RECORD_KIND {
            return Err(ManagedTruthValidationError::WrongSnapshotRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        if self.schema_version != MANAGED_TRUTH_SCHEMA_VERSION {
            return Err(ManagedTruthValidationError::WrongSnapshotSchemaVersion {
                actual: self.schema_version,
            });
        }
        if !non_empty(&self.snapshot_id) {
            return Err(ManagedTruthValidationError::MissingSnapshotId);
        }
        if self.rows.is_empty() {
            return Err(ManagedTruthValidationError::EmptySnapshot);
        }
        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            row.validate()?;
            if !row_ids.insert(row.row_id.clone()) {
                return Err(ManagedTruthValidationError::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
        }
        Ok(())
    }

    /// Returns shell-ready display rows in snapshot order.
    pub fn display_rows(&self) -> Vec<ManagedTruthDisplayRow> {
        self.rows
            .iter()
            .map(ManagedTruthRow::display_state)
            .collect()
    }

    /// Returns all claim classes covered by this snapshot.
    pub fn claim_classes(&self) -> BTreeSet<ManagedTruthClaimClass> {
        self.rows.iter().map(|row| row.claim_class).collect()
    }

    /// True when at least one managed row and one provider-linked row are covered.
    pub fn covers_managed_and_provider_rows(&self) -> bool {
        let classes = self.claim_classes();
        classes.iter().any(|class| class.is_managed_claim())
            && classes.contains(&ManagedTruthClaimClass::ProviderLinked)
    }

    /// True when at least one relevant impairment distinguishes control and data planes.
    pub fn covers_plane_impairment_split(&self) -> bool {
        self.rows
            .iter()
            .any(|row| row.planes.has_relevant_impairment() && row.planes.distinguishes_planes())
    }

    /// True when all claimed rows disclose region, tenant, storage/copy, and key mode.
    pub fn all_claimed_rows_disclose_boundary_truth(&self) -> bool {
        self.rows.iter().all(|row| {
            row.region_residency.region_scope.is_disclosed()
                && row.tenant.tenant_org_scope != TenantOrgScopeClass::NotApplicable
                && row.storage_copy.storage_location != StorageLocationClass::NotApplicable
                && row.key.key_mode.is_disclosed()
                && row.region_residency.data_residency_disclosure_class
                    != DataResidencyDisclosureClass::ResidencyUnknown
        })
    }

    /// True when provider-linked rows do not overclaim managed or sovereign boundaries.
    pub fn provider_rows_do_not_overclaim_sovereignty(&self) -> bool {
        self.rows
            .iter()
            .filter(|row| row.claim_class == ManagedTruthClaimClass::ProviderLinked)
            .all(|row| {
                !row.sovereignty
                    .sovereignty_boundary
                    .overclaims_provider_boundary()
                    && !row.display_copy.stronger_sovereignty_boundary_implied
            })
    }

    /// True when the snapshot exercises the task acceptance states.
    pub fn has_acceptance_coverage(&self) -> bool {
        self.covers_managed_and_provider_rows()
            && self.covers_plane_impairment_split()
            && self.all_claimed_rows_disclose_boundary_truth()
            && self.provider_rows_do_not_overclaim_sovereignty()
    }

    /// Builds a metadata-only support/review export packet.
    pub fn export_packet(&self) -> ManagedTruthExportPacket {
        ManagedTruthExportPacket {
            record_kind: MANAGED_TRUTH_EXPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: MANAGED_TRUTH_SCHEMA_VERSION,
            packet_id: format!("{}:support", self.snapshot_id),
            snapshot_id: self.snapshot_id.clone(),
            emitted_at: self.emitted_at.clone(),
            raw_payloads_excluded: true,
            rows: self.rows.iter().map(ManagedTruthExportRow::from_row).collect(),
            notes: "Metadata-only managed truth packet; raw tenant names, cloud regions, provider payloads, URLs, secret material, key bytes, and certificate bodies are excluded."
                .to_owned(),
        }
    }

    /// Deterministic plaintext summary for support and proof captures.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] snapshot={} emitted_at={}\n",
            MANAGED_TRUTH_EXPORT_PACKET_RECORD_KIND, self.snapshot_id, self.emitted_at,
        ));
        for row in &self.rows {
            out.push_str(&format!(
                "- row={} surface={} claim={} mode={}\n",
                row.row_id,
                row.surface_class.as_str(),
                row.claim_class.as_str(),
                row.operating_mode_class.as_str(),
            ));
            out.push_str(&format!(
                "  region={} residency={} tenant={} storage={} copy={} key={}\n",
                row.region_residency.region_scope.as_str(),
                row.region_residency
                    .data_residency_disclosure_class
                    .as_str(),
                row.tenant.tenant_org_scope.as_str(),
                row.storage_copy.storage_location.as_str(),
                row.storage_copy.copy_posture.as_str(),
                row.key.key_mode.as_str(),
            ));
            out.push_str(&format!(
                "  planes: control={} data={} fail={}\n",
                row.planes.control_plane_state.as_str(),
                row.planes.data_plane_state.as_str(),
                row.planes.fail_posture.as_str(),
            ));
            out.push_str(&format!(
                "  sovereignty={} local_core={}\n",
                row.sovereignty.sovereignty_boundary.as_str(),
                row.local_continuity.local_core_available,
            ));
        }
        out
    }
}

/// One metadata-only export row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedTruthExportRow {
    /// Stable row id.
    pub row_id: String,
    /// Surface class.
    pub surface_class: ManagedTruthSurfaceClass,
    /// Claim class.
    pub claim_class: ManagedTruthClaimClass,
    /// Operating mode.
    pub operating_mode_class: OperatingModeClass,
    /// Region scope.
    pub region_scope: RegionScopeClass,
    /// Residency disclosure.
    pub data_residency_disclosure_class: DataResidencyDisclosureClass,
    /// Tenant scope.
    pub tenant_org_scope: TenantOrgScopeClass,
    /// Processing location.
    pub processing_location: ProcessingLocationClass,
    /// Storage location.
    pub storage_location: StorageLocationClass,
    /// Copy posture.
    pub copy_posture: CopyPostureClass,
    /// Key mode.
    pub key_mode: KeyModeClass,
    /// Key state.
    pub key_state_class: KeyStateClass,
    /// Control-plane state.
    pub control_plane_state: PlaneStateClass,
    /// Data-plane state.
    pub data_plane_state: PlaneStateClass,
    /// Fail posture for active narrowing.
    pub fail_posture: FailPostureClass,
    /// Sovereignty boundary.
    pub sovereignty_boundary: SovereigntyBoundaryClass,
    /// Redaction-safe summary.
    pub summary: String,
    /// Redaction-safe local continuity summary.
    pub local_continuity_summary: String,
    /// Source refs needed to reconstruct the row.
    pub source_refs: Vec<String>,
}

impl ManagedTruthExportRow {
    /// Projects one managed-truth row into metadata-only support shape.
    pub fn from_row(row: &ManagedTruthRow) -> Self {
        Self {
            row_id: row.row_id.clone(),
            surface_class: row.surface_class,
            claim_class: row.claim_class,
            operating_mode_class: row.operating_mode_class,
            region_scope: row.region_residency.region_scope,
            data_residency_disclosure_class: row.region_residency.data_residency_disclosure_class,
            tenant_org_scope: row.tenant.tenant_org_scope,
            processing_location: row.storage_copy.processing_location,
            storage_location: row.storage_copy.storage_location,
            copy_posture: row.storage_copy.copy_posture,
            key_mode: row.key.key_mode,
            key_state_class: row.key.key_state_class,
            control_plane_state: row.planes.control_plane_state,
            data_plane_state: row.planes.data_plane_state,
            fail_posture: if row.planes.fail_posture == FailPostureClass::NotApplicable {
                row.key.fail_posture
            } else {
                row.planes.fail_posture
            },
            sovereignty_boundary: row.sovereignty.sovereignty_boundary,
            summary: row.summary.clone(),
            local_continuity_summary: row.local_continuity.continuity_summary.clone(),
            source_refs: row.combined_source_refs(),
        }
    }
}

/// Metadata-only support/review packet for managed truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedTruthExportPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Packet schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Source snapshot id.
    pub snapshot_id: String,
    /// Emission timestamp or monotonic fixture time.
    pub emitted_at: String,
    /// Always true for this packet.
    pub raw_payloads_excluded: bool,
    /// Export rows.
    pub rows: Vec<ManagedTruthExportRow>,
    /// Redaction-safe note.
    pub notes: String,
}

impl ManagedTruthExportPacket {
    /// True when the packet is safe for support/export use.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payloads_excluded
            && self.record_kind == MANAGED_TRUTH_EXPORT_PACKET_RECORD_KIND
            && self.schema_version == MANAGED_TRUTH_SCHEMA_VERSION
            && self
                .rows
                .iter()
                .all(|row| !row.source_refs.is_empty() && non_empty(&row.summary))
    }
}

/// Validation error for managed-truth records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManagedTruthValidationError {
    /// Snapshot record kind was not recognized.
    WrongSnapshotRecordKind { actual: String },
    /// Snapshot schema version is unsupported.
    WrongSnapshotSchemaVersion { actual: u32 },
    /// Row record kind was not recognized.
    WrongRowRecordKind { row_id: String, actual: String },
    /// Row schema version is unsupported.
    WrongRowSchemaVersion { row_id: String, actual: u32 },
    /// Snapshot id is empty.
    MissingSnapshotId,
    /// Snapshot contains no rows.
    EmptySnapshot,
    /// A required field is empty.
    MissingRequiredField { row_id: String, field: &'static str },
    /// A managed row did not cite boundary, identity, region-key, and operating-mode refs.
    MissingManagedRefs { row_id: String },
    /// A provider-linked row did not cite provider registry refs.
    MissingProviderRefs { row_id: String },
    /// Surface class and claim class disagree.
    SurfaceClaimMismatch { row_id: String },
    /// Managed row is missing region, tenant, storage, or key disclosure.
    ManagedClaimMissingBoundaryTruth { row_id: String },
    /// Provider-linked row contains unknown region, residency, or key truth.
    ProviderLinkedUnknownBoundaryTruth { row_id: String },
    /// Provider-linked row implies customer-managed or sovereign boundary.
    ProviderLinkedOverclaimsSovereignty {
        row_id: String,
        sovereignty_boundary: SovereigntyBoundaryClass,
    },
    /// Provider-linked row reused managed-tenant residency vocabulary.
    ProviderLinkedUsesManagedResidency { row_id: String },
    /// Key state requires affected action families and fail posture.
    KeyStateMissingAffectedActions { row_id: String },
    /// Plane impairment requires affected action families and fail posture.
    PlaneImpairmentMissingScope { row_id: String },
    /// Plane impairment did not distinguish control plane from data plane.
    PlaneImpairmentNotDistinguished { row_id: String },
    /// Local-safe continuation is missing.
    MissingLocalContinuity { row_id: String },
    /// Display copy would overclaim or fail open.
    UnsafeDisplayCopy { row_id: String },
    /// Source refs are missing.
    MissingSourceRefs { row_id: String },
    /// Row ids must be unique.
    DuplicateRowId { row_id: String },
}

impl fmt::Display for ManagedTruthValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongSnapshotRecordKind { actual } => {
                write!(f, "unsupported managed-truth snapshot record kind {actual}")
            }
            Self::WrongSnapshotSchemaVersion { actual } => {
                write!(
                    f,
                    "unsupported managed-truth snapshot schema version {actual}"
                )
            }
            Self::WrongRowRecordKind { row_id, actual } => {
                write!(f, "row {row_id} has unsupported record kind {actual}")
            }
            Self::WrongRowSchemaVersion { row_id, actual } => {
                write!(f, "row {row_id} has unsupported schema version {actual}")
            }
            Self::MissingSnapshotId => write!(f, "managed-truth snapshot id is required"),
            Self::EmptySnapshot => write!(f, "managed-truth snapshot must contain rows"),
            Self::MissingRequiredField { row_id, field } => {
                write!(f, "row {row_id} is missing required field {field}")
            }
            Self::MissingManagedRefs { row_id } => {
                write!(f, "row {row_id} is missing managed upstream refs")
            }
            Self::MissingProviderRefs { row_id } => {
                write!(f, "row {row_id} is missing provider upstream refs")
            }
            Self::SurfaceClaimMismatch { row_id } => {
                write!(f, "row {row_id} surface class and claim class disagree")
            }
            Self::ManagedClaimMissingBoundaryTruth { row_id } => {
                write!(f, "row {row_id} is missing managed boundary truth")
            }
            Self::ProviderLinkedUnknownBoundaryTruth { row_id } => {
                write!(f, "row {row_id} has unknown provider boundary truth")
            }
            Self::ProviderLinkedOverclaimsSovereignty {
                row_id,
                sovereignty_boundary,
            } => write!(
                f,
                "row {row_id} overclaims provider sovereignty as {}",
                sovereignty_boundary.as_str()
            ),
            Self::ProviderLinkedUsesManagedResidency { row_id } => {
                write!(f, "row {row_id} reuses managed residency vocabulary")
            }
            Self::KeyStateMissingAffectedActions { row_id } => {
                write!(f, "row {row_id} key state is missing affected actions")
            }
            Self::PlaneImpairmentMissingScope { row_id } => {
                write!(f, "row {row_id} plane impairment is missing scope")
            }
            Self::PlaneImpairmentNotDistinguished { row_id } => {
                write!(f, "row {row_id} does not distinguish impaired planes")
            }
            Self::MissingLocalContinuity { row_id } => {
                write!(f, "row {row_id} is missing local continuity")
            }
            Self::UnsafeDisplayCopy { row_id } => {
                write!(f, "row {row_id} has unsafe display copy invariants")
            }
            Self::MissingSourceRefs { row_id } => {
                write!(f, "row {row_id} is missing source refs")
            }
            Self::DuplicateRowId { row_id } => write!(f, "duplicate managed-truth row {row_id}"),
        }
    }
}

impl std::error::Error for ManagedTruthValidationError {}

fn non_empty(value: &str) -> bool {
    !value.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn display_copy() -> ManagedTruthDisplayCopy {
        ManagedTruthDisplayCopy {
            whole_product_failure_implied: false,
            stronger_sovereignty_boundary_implied: false,
            silent_fail_open_under_unknown_state: false,
            plaintext_secret_fallback_implied: false,
        }
    }

    fn managed_row() -> ManagedTruthRow {
        ManagedTruthRow {
            record_kind: MANAGED_TRUTH_ROW_RECORD_KIND.to_owned(),
            schema_version: MANAGED_TRUTH_SCHEMA_VERSION,
            row_id: "managed_truth.enterprise.region".to_owned(),
            surface_class: ManagedTruthSurfaceClass::ManagedService,
            claim_class: ManagedTruthClaimClass::EnterpriseSaas,
            operating_mode_class: OperatingModeClass::EnterpriseSaas,
            surface_ref: "surface.managed.ai".to_owned(),
            title: "Enterprise SaaS region strip".to_owned(),
            summary: "Enterprise SaaS row exposes region, tenant, storage, and key truth."
                .to_owned(),
            refs: ManagedTruthRefs {
                boundary_manifest_capability_ref: Some("boundary.capability.managed_ai".to_owned()),
                identity_mode_row_ref: Some("identity_mode.enterprise_online.active".to_owned()),
                region_key_state_ref: Some("region_key_state.enterprise".to_owned()),
                operating_mode_card_ref: Some("operating_mode_card.enterprise".to_owned()),
                provider_descriptor_ref: None,
                provider_registry_packet_ref: None,
                source_refs: vec!["docs/service/operating_mode_and_capacity_contract.md".to_owned()],
            },
            region_residency: RegionResidencyTruth {
                region_scope: RegionScopeClass::CustomerRegionPinned,
                region_ref: Some("region.ref.enterprise".to_owned()),
                residency_scope_class: ResidencyScopeClass::RegulatedJurisdiction,
                data_residency_disclosure_class:
                    DataResidencyDisclosureClass::ResidencyManagedTenantDocumentedRegion,
                residency_summary: "Customer-pinned region with audited egress.".to_owned(),
            },
            tenant: TenantTruth {
                tenant_org_scope: TenantOrgScopeClass::CustomerTenant,
                tenant_ref: Some("tenant.ref.enterprise".to_owned()),
                tenant_summary: "Customer tenant boundary.".to_owned(),
            },
            storage_copy: StorageCopyTruth {
                processing_location: ProcessingLocationClass::VendorControlPlane,
                storage_location: StorageLocationClass::VendorControlPlaneStorage,
                copy_posture: CopyPostureClass::ManagedCopyWithLocalSafeArtifacts,
                retention_class: "vendor_retention_window_with_customer_policy".to_owned(),
                copy_summary: "Managed copy exists; local-safe artifacts continue.".to_owned(),
            },
            key: KeyModeTruth {
                key_mode: KeyModeClass::CustomerManaged,
                key_state_class: KeyStateClass::BoundAndCurrent,
                key_ref: Some("key.ref.enterprise".to_owned()),
                key_state_summary: "Customer-managed keys are current.".to_owned(),
                affected_action_families: vec![],
                fail_posture: FailPostureClass::NotApplicable,
            },
            planes: PlaneImpairmentTruth {
                control_plane_state: PlaneStateClass::Healthy,
                data_plane_state: PlaneStateClass::Healthy,
                control_plane_summary: "Control plane healthy.".to_owned(),
                data_plane_summary: "Data plane healthy.".to_owned(),
                affected_action_families: vec![],
                fail_posture: FailPostureClass::NotApplicable,
                last_control_plane_sync_at: Some("2026-05-14T00:00:00Z".to_owned()),
                last_data_plane_probe_at: Some("2026-05-14T00:00:00Z".to_owned()),
            },
            local_continuity: LocalContinuityTruth {
                local_core_available: true,
                retained_local_safe_capabilities: vec!["Continue local edit and Git.".to_owned()],
                blocked_managed_or_provider_capabilities: vec![],
                continuity_summary: "Local core remains available.".to_owned(),
            },
            sovereignty: SovereigntyTruth {
                sovereignty_boundary: SovereigntyBoundaryClass::CustomerPinnedManaged,
                residual_dependency_refs: vec!["residual.managed_ai_gateway".to_owned()],
                sovereignty_summary: "Customer-pinned managed boundary; not sovereign.".to_owned(),
            },
            display_copy: display_copy(),
        }
    }

    #[test]
    fn managed_row_validates_and_exports() {
        let snapshot = ManagedTruthSnapshot {
            record_kind: MANAGED_TRUTH_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: MANAGED_TRUTH_SCHEMA_VERSION,
            snapshot_id: "managed_truth.snapshot.test".to_owned(),
            emitted_at: "2026-05-14T00:00:00Z".to_owned(),
            rows: vec![managed_row()],
            notes: None,
        };
        snapshot.validate().expect("snapshot validates");
        let export = snapshot.export_packet();
        assert!(export.is_export_safe());
        assert_eq!(export.rows[0].key_mode, KeyModeClass::CustomerManaged);
    }

    #[test]
    fn provider_linked_overclaim_is_rejected() {
        let mut row = managed_row();
        row.surface_class = ManagedTruthSurfaceClass::ProviderLinked;
        row.claim_class = ManagedTruthClaimClass::ProviderLinked;
        row.operating_mode_class = OperatingModeClass::ProviderLinked;
        row.refs.provider_descriptor_ref = Some("provider_descriptor.ci.primary".to_owned());
        row.refs.provider_registry_packet_ref =
            Some("provider_alpha.registry.launch_wedge".to_owned());
        row.sovereignty.sovereignty_boundary = SovereigntyBoundaryClass::RegulatedJurisdiction;
        let err = row.validate().unwrap_err();
        assert!(matches!(
            err,
            ManagedTruthValidationError::ProviderLinkedOverclaimsSovereignty { .. }
        ));
    }

    #[test]
    fn impaired_planes_must_be_distinct_and_scoped() {
        let mut row = managed_row();
        row.planes.control_plane_state = PlaneStateClass::Unavailable;
        row.planes.data_plane_state = PlaneStateClass::Unavailable;
        row.planes.control_plane_summary = "Plane unavailable.".to_owned();
        row.planes.data_plane_summary = "Plane unavailable.".to_owned();
        row.planes.affected_action_families =
            vec![AffectedActionFamilyClass::IdentityRefreshActionFamily];
        row.planes.fail_posture = FailPostureClass::FailClosedManagedOnly;
        let err = row.validate().unwrap_err();
        assert!(matches!(
            err,
            ManagedTruthValidationError::PlaneImpairmentNotDistinguished { .. }
        ));
    }
}
