//! Account-free, self-hosted, and managed identity-mode baseline.
//!
//! This module owns the alpha identity-mode baseline packet that joins the
//! existing auth callback, credential-state, restricted-mode, boundary, and
//! entitlement artifacts without copying their full schemas. Each row names
//! one identity mode, keeps account-free local work explicit, and exposes
//! policy-source and offline-entitlement inspectors for shell, CLI, support,
//! and docs surfaces.
//!
//! The packet cites the governance boundary manifest and entitlement snapshot
//! artifacts by opaque ref. It does not embed signed bundle bodies, raw policy
//! rules, raw tenant names, raw user emails, raw credentials, or hosted console
//! payloads.

use serde::{Deserialize, Serialize};

pub use crate::browser_callback::{
    AccountBoundaryClass, IdentityModeAlias, RetryPathClass, TrustState,
};

/// Record-kind tag carried on serialized [`IdentityModeBaselineRow`] payloads.
pub const IDENTITY_MODE_BASELINE_ROW_RECORD_KIND: &str = "identity_mode_baseline_row_record";

/// Record-kind tag carried on serialized [`IdentityModeBaselinePacket`] payloads.
pub const IDENTITY_MODE_BASELINE_PACKET_RECORD_KIND: &str = "identity_mode_baseline_packet_record";

/// Schema version of the identity-mode baseline row and packet payloads.
pub const IDENTITY_MODE_BASELINE_SCHEMA_VERSION: u32 = 1;

/// Required local-core capability ids for an account-free local path.
pub const REQUIRED_LOCAL_CORE_CAPABILITY_IDS: &[&str] = &[
    "editor_core",
    "search_local",
    "local_git",
    "local_tasks_debug",
    "local_history",
    "local_ai_byok",
];

/// Deployment profile claimed by the current row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityModeDeploymentProfileClass {
    /// Individual desktop-local install with no org boundary.
    IndividualLocal,
    /// Customer-run identity, policy, and service endpoints.
    SelfHosted,
    /// Enterprise online profile that may combine self-hosted and managed paths.
    EnterpriseOnline,
    /// Vendor-managed convenience profile.
    ManagedCloud,
    /// Offline or air-gapped profile using imported bundles and snapshots.
    AirGapped,
}

impl IdentityModeDeploymentProfileClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndividualLocal => "individual_local",
            Self::SelfHosted => "self_hosted",
            Self::EnterpriseOnline => "enterprise_online",
            Self::ManagedCloud => "managed_cloud",
            Self::AirGapped => "air_gapped",
        }
    }
}

/// Auth mode disclosed by an identity-mode row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityAuthModeClass {
    /// No account or remote identity is required.
    AccountFreeLocal,
    /// OIDC or equivalent sign-in through the system browser.
    SystemBrowserOidc,
    /// Passkey-capable federated sign-in is available.
    PasskeyCapableFederated,
    /// Signed file or imported snapshot is the active auth input.
    SignedFileOrSnapshot,
    /// Auth has not been configured yet.
    NotConfigured,
}

impl IdentityAuthModeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountFreeLocal => "account_free_local",
            Self::SystemBrowserOidc => "system_browser_oidc",
            Self::PasskeyCapableFederated => "passkey_capable_federated",
            Self::SignedFileOrSnapshot => "signed_file_or_snapshot",
            Self::NotConfigured => "not_configured",
        }
    }
}

/// Provisioning class disclosed by an identity-mode row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvisioningClass {
    /// No provisioning applies to account-free local use.
    NotApplicableLocal,
    /// Manual local or file-based setup.
    ManualLocal,
    /// Signed bundle or offline import provides org lifecycle state.
    SignedFileBundle,
    /// SCIM or equivalent lifecycle provisioning is active.
    ScimProvisioned,
    /// Managed seat or hosted admin lifecycle is active.
    ManagedSeat,
}

impl ProvisioningClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicableLocal => "not_applicable_local",
            Self::ManualLocal => "manual_local",
            Self::SignedFileBundle => "signed_file_bundle",
            Self::ScimProvisioned => "scim_provisioned",
            Self::ManagedSeat => "managed_seat",
        }
    }
}

/// Source class for policy-source inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicySourceClass {
    /// No policy source is required for local core.
    NoPolicyRequiredLocal,
    /// Local advisory file or CLI/profile setting.
    LocalAdvisoryFile,
    /// Customer-operated self-hosted policy origin.
    CustomerSelfHostedOrigin,
    /// Vendor-managed policy origin.
    VendorManagedOrigin,
    /// Signed mirror of an authoritative policy origin.
    SignedMirrorOrigin,
    /// Manual signed file import.
    ManualFileImportOrigin,
    /// Air-gapped signed transfer.
    AirGappedTransferOrigin,
    /// Build preload or first-run seed.
    RuntimePreloadOrigin,
    /// Policy source is missing or unresolved.
    Unknown,
}

impl PolicySourceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPolicyRequiredLocal => "no_policy_required_local",
            Self::LocalAdvisoryFile => "local_advisory_file",
            Self::CustomerSelfHostedOrigin => "customer_self_hosted_origin",
            Self::VendorManagedOrigin => "vendor_managed_origin",
            Self::SignedMirrorOrigin => "signed_mirror_origin",
            Self::ManualFileImportOrigin => "manual_file_import_origin",
            Self::AirGappedTransferOrigin => "air_gapped_transfer_origin",
            Self::RuntimePreloadOrigin => "runtime_preload_origin",
            Self::Unknown => "unknown",
        }
    }

    /// True when this source is sufficient for managed or self-hosted disclosure.
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

/// Freshness class for a policy-source inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyFreshnessClass {
    /// Policy freshness does not apply to account-free local mode.
    NotApplicableAccountFree,
    /// Live policy has been verified.
    AuthoritativeLive,
    /// Mirrored or cached policy is current.
    VerifiedCurrent,
    /// Last-known-good policy is stale but still inside grace.
    StaleWithinGrace,
    /// Last-known-good policy is past grace and may only narrow managed actions.
    StalePastGrace,
    /// Offline signed snapshot is still inside its window.
    OfflineSnapshotUnexpired,
    /// Offline signed snapshot is expired.
    OfflineSnapshotExpired,
    /// Policy was never refreshed beyond the seed.
    NeverRefreshedSeed,
}

impl PolicyFreshnessClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicableAccountFree => "not_applicable_account_free",
            Self::AuthoritativeLive => "authoritative_live",
            Self::VerifiedCurrent => "verified_current",
            Self::StaleWithinGrace => "stale_within_grace",
            Self::StalePastGrace => "stale_past_grace",
            Self::OfflineSnapshotUnexpired => "offline_snapshot_unexpired",
            Self::OfflineSnapshotExpired => "offline_snapshot_expired",
            Self::NeverRefreshedSeed => "never_refreshed_seed",
        }
    }
}

/// Entitlement state exposed by an identity-mode row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntitlementStateClass {
    /// Entitlement does not apply to account-free local mode.
    NotApplicableAccountFree,
    /// Managed or self-hosted entitlement is active.
    Active,
    /// No org-scoped managed action is available because the user is signed out.
    SignedOutNoOrgScopedAction,
    /// Cached entitlement is stale and only local-safe behavior is admitted.
    StaleCache,
    /// Offline last-known-good state is active.
    OfflineLastKnownGood,
    /// Grace window is active.
    Grace,
    /// Entitlement expired.
    Expired,
    /// Entitlement or seat was revoked.
    Revoked,
    /// Managed features are restricted, but local work is preserved.
    RestrictedManagedOnly,
    /// Managed access ended and local artifacts remain available.
    OffboardedLocalPreserved,
}

impl EntitlementStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicableAccountFree => "not_applicable_account_free",
            Self::Active => "active",
            Self::SignedOutNoOrgScopedAction => "signed_out_no_org_scoped_action",
            Self::StaleCache => "stale_cache",
            Self::OfflineLastKnownGood => "offline_last_known_good",
            Self::Grace => "grace",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::RestrictedManagedOnly => "restricted_managed_only",
            Self::OffboardedLocalPreserved => "offboarded_local_preserved",
        }
    }

    /// True when new managed actions must not proceed without visible recovery.
    pub const fn blocks_new_managed_actions(self) -> bool {
        matches!(
            self,
            Self::SignedOutNoOrgScopedAction
                | Self::StaleCache
                | Self::Expired
                | Self::Revoked
                | Self::RestrictedManagedOnly
                | Self::OffboardedLocalPreserved
        )
    }
}

/// Offline behavior disclosed by an entitlement inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineBehaviorClass {
    /// Local-safe capabilities have no offline expiry.
    UnlimitedOfflineLocalSafe,
    /// Last-known-good state preserves local-safe behavior only.
    LastKnownGoodLocalSafeOnly,
    /// Managed-only capabilities pause with visible recovery.
    ManagedOnlyPausedVisibleRecovery,
    /// Fresh refresh is required before a new managed action.
    RefreshRequiredBeforeNewManagedUse,
    /// Offline use is not permitted for the managed-only feature.
    OfflineNotPermittedManagedOnlyFeature,
    /// Offline behavior was not declared.
    Unknown,
}

impl OfflineBehaviorClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnlimitedOfflineLocalSafe => "unlimited_offline_local_safe",
            Self::LastKnownGoodLocalSafeOnly => "last_known_good_local_safe_only",
            Self::ManagedOnlyPausedVisibleRecovery => "managed_only_paused_visible_recovery",
            Self::RefreshRequiredBeforeNewManagedUse => "refresh_required_before_new_managed_use",
            Self::OfflineNotPermittedManagedOnlyFeature => {
                "offline_not_permitted_managed_only_feature"
            }
            Self::Unknown => "unknown",
        }
    }
}

/// Boundary that the current deployment actually provides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentDeploymentBoundaryClass {
    /// No managed boundary exists for this row.
    LocalOnlyNoManagedBoundary,
    /// Customer-operated self-hosted control plane.
    CustomerSelfHostedControlPlane,
    /// Vendor-managed control plane.
    VendorManagedControlPlane,
    /// Mirror or file import supplies policy and artifacts.
    MirrorOrFileImport,
    /// Air-gapped local/offline posture.
    AirGappedLocalOnly,
}

impl CurrentDeploymentBoundaryClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyNoManagedBoundary => "local_only_no_managed_boundary",
            Self::CustomerSelfHostedControlPlane => "customer_self_hosted_control_plane",
            Self::VendorManagedControlPlane => "vendor_managed_control_plane",
            Self::MirrorOrFileImport => "mirror_or_file_import",
            Self::AirGappedLocalOnly => "air_gapped_local_only",
        }
    }
}

/// Region posture consumed by managed and provider-linked truth rows.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum RegionMode {
    /// Customer-selected region is pinned by the upstream contract.
    CustomerRegionPinned,
    /// User-owned remote target region applies.
    RemoteTargetRegion,
    /// Connected provider's documented default region applies.
    ProviderDefaultDisclosed,
    /// Boundary must be rechecked before writes resume.
    BoundaryRecheckRequired,
    /// Region mode is not known and must remain visibly unresolved.
    #[default]
    Unknown,
}

impl RegionMode {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CustomerRegionPinned => "customer_region_pinned",
            Self::RemoteTargetRegion => "remote_target_region",
            Self::ProviderDefaultDisclosed => "provider_default_disclosed",
            Self::BoundaryRecheckRequired => "boundary_recheck_required",
            Self::Unknown => "unknown",
        }
    }

    /// Human-readable label for shell and support projections.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CustomerRegionPinned => "Customer region pinned",
            Self::RemoteTargetRegion => "Remote target region",
            Self::ProviderDefaultDisclosed => "Provider default disclosed",
            Self::BoundaryRecheckRequired => "Region recheck required",
            Self::Unknown => "Unknown",
        }
    }

    /// True when the upstream row did not disclose region truth.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Data-residency posture consumed by managed and provider-linked truth rows.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum ResidencyMode {
    /// Data remains on the local device.
    #[serde(rename = "residency_local_device_only")]
    LocalDeviceOnly,
    /// Data may rest on a user-owned remote target.
    #[serde(rename = "residency_user_owned_remote_target")]
    UserOwnedRemoteTarget,
    /// Connected provider default applies and is disclosed.
    #[serde(rename = "residency_provider_default")]
    ProviderDefault,
    /// Managed tenant documents a region without this row certifying enforcement.
    #[serde(rename = "residency_managed_tenant_documented_region")]
    ManagedTenantDocumentedRegion,
    /// Residency mode is not known and must remain visibly unresolved.
    #[serde(rename = "residency_unknown")]
    #[default]
    Unknown,
}

impl ResidencyMode {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDeviceOnly => "residency_local_device_only",
            Self::UserOwnedRemoteTarget => "residency_user_owned_remote_target",
            Self::ProviderDefault => "residency_provider_default",
            Self::ManagedTenantDocumentedRegion => "residency_managed_tenant_documented_region",
            Self::Unknown => "residency_unknown",
        }
    }

    /// Human-readable label for shell and support projections.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalDeviceOnly => "Local device only",
            Self::UserOwnedRemoteTarget => "User-owned remote target",
            Self::ProviderDefault => "Provider default",
            Self::ManagedTenantDocumentedRegion => "Managed tenant documented region",
            Self::Unknown => "Unknown",
        }
    }

    /// True when the upstream row did not disclose residency truth.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Key ownership or storage posture consumed by managed and provider-linked truth rows.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum KeyMode {
    /// OS credential store or keychain.
    OsStore,
    /// Vendor-managed key material.
    VendorManaged,
    /// Customer-managed key material.
    CustomerManaged,
    /// Offline trust-root posture.
    OfflineTrustRoot,
    /// Connected provider manages the key material.
    ProviderManaged,
    /// User supplies BYOK material treated as opaque by Aureline.
    ByokUserManaged,
    /// Key mode is not known and must remain visibly unresolved.
    #[default]
    Unknown,
}

impl KeyMode {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsStore => "os_store",
            Self::VendorManaged => "vendor_managed",
            Self::CustomerManaged => "customer_managed",
            Self::OfflineTrustRoot => "offline_trust_root",
            Self::ProviderManaged => "provider_managed",
            Self::ByokUserManaged => "byok_user_managed",
            Self::Unknown => "unknown",
        }
    }

    /// Human-readable label for shell and support projections.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OsStore => "OS credential store",
            Self::VendorManaged => "Vendor-managed keys",
            Self::CustomerManaged => "Customer-managed keys",
            Self::OfflineTrustRoot => "Offline trust root",
            Self::ProviderManaged => "Provider-managed keys",
            Self::ByokUserManaged => "BYOK user-managed",
            Self::Unknown => "Unknown",
        }
    }

    /// True when the upstream row did not disclose key-mode truth.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Local-core continuity block attached to every row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalCoreContinuity {
    /// Whether local core requires account creation.
    pub account_required_for_local_core: bool,
    /// Whether local core is available in this row.
    pub local_core_available: bool,
    /// Capability ids proven available without account creation.
    pub local_capability_ids: Vec<String>,
    /// Export-safe continuity label.
    pub local_continuity_label: String,
    /// Recovery action used to stay local.
    pub stay_local_action: RetryPathClass,
    /// Stable token for [`Self::stay_local_action`].
    pub stay_local_action_token: String,
}

impl LocalCoreContinuity {
    /// Builds a local-core continuity block and fills the action token.
    pub fn new(
        account_required_for_local_core: bool,
        local_core_available: bool,
        local_capability_ids: Vec<String>,
        local_continuity_label: impl Into<String>,
        stay_local_action: RetryPathClass,
    ) -> Self {
        Self {
            account_required_for_local_core,
            local_core_available,
            local_capability_ids,
            local_continuity_label: local_continuity_label.into(),
            stay_local_action,
            stay_local_action_token: stay_local_action.as_str().to_owned(),
        }
    }

    /// True when all required local-core capabilities are available without an account.
    pub fn account_free_local_core_available(&self) -> bool {
        !self.account_required_for_local_core
            && self.local_core_available
            && REQUIRED_LOCAL_CORE_CAPABILITY_IDS.iter().all(|required| {
                self.local_capability_ids
                    .iter()
                    .any(|capability| capability == required)
            })
    }

    /// Returns the required local-core capability ids missing from this block.
    pub fn missing_required_capabilities(&self) -> Vec<&'static str> {
        REQUIRED_LOCAL_CORE_CAPABILITY_IDS
            .iter()
            .copied()
            .filter(|required| {
                !self
                    .local_capability_ids
                    .iter()
                    .any(|capability| capability == required)
            })
            .collect()
    }
}

/// Policy-source inspector shown on identity-mode rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityPolicySourceInspector {
    /// Policy source class.
    pub source_class: PolicySourceClass,
    /// Stable token for [`Self::source_class`].
    pub source_token: String,
    /// Export-safe source label.
    pub source_label: String,
    /// Opaque policy-bundle ref, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_bundle_ref: Option<String>,
    /// Opaque policy epoch ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_epoch_ref: Option<String>,
    /// Freshness class for the policy source.
    pub freshness_class: PolicyFreshnessClass,
    /// Stable token for [`Self::freshness_class`].
    pub freshness_token: String,
    /// Last successful refresh timestamp, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_refreshed_at: Option<String>,
    /// Last-known-good policy ref, when a fallback exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good_ref: Option<String>,
    /// Whether policy detail is locally inspectable without a hosted console.
    pub local_inspection_available: bool,
    /// Whether a hosted console is required for full detail.
    pub vendor_console_required_for_full_detail: bool,
    /// Action used to open policy details or request policy repair.
    pub policy_detail_action: RetryPathClass,
    /// Stable token for [`Self::policy_detail_action`].
    pub policy_detail_action_token: String,
    /// Export-safe policy-source explanation.
    pub explanation_label: String,
}

impl IdentityPolicySourceInspector {
    /// Builds a policy-source inspector and fills stable tokens.
    pub fn new(request: IdentityPolicySourceInspectorRequest<'_>) -> Self {
        Self {
            source_class: request.source_class,
            source_token: request.source_class.as_str().to_owned(),
            source_label: request.source_label.to_owned(),
            policy_bundle_ref: request.policy_bundle_ref.map(str::to_owned),
            policy_epoch_ref: request.policy_epoch_ref.map(str::to_owned),
            freshness_class: request.freshness_class,
            freshness_token: request.freshness_class.as_str().to_owned(),
            last_refreshed_at: request.last_refreshed_at.map(str::to_owned),
            last_known_good_ref: request.last_known_good_ref.map(str::to_owned),
            local_inspection_available: request.local_inspection_available,
            vendor_console_required_for_full_detail: request
                .vendor_console_required_for_full_detail,
            policy_detail_action: request.policy_detail_action,
            policy_detail_action_token: request.policy_detail_action.as_str().to_owned(),
            explanation_label: request.explanation_label.to_owned(),
        }
    }

    /// True when this inspector can explain policy source locally.
    pub fn discloses_policy_source(&self) -> bool {
        self.source_class.is_disclosed()
            && !self.source_label.trim().is_empty()
            && !self.explanation_label.trim().is_empty()
            && self.local_inspection_available
    }
}

/// Request used to build an [`IdentityPolicySourceInspector`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityPolicySourceInspectorRequest<'a> {
    /// Policy source class.
    pub source_class: PolicySourceClass,
    /// Export-safe source label.
    pub source_label: &'a str,
    /// Opaque policy-bundle ref, when one exists.
    pub policy_bundle_ref: Option<&'a str>,
    /// Opaque policy epoch ref.
    pub policy_epoch_ref: Option<&'a str>,
    /// Freshness class for the policy source.
    pub freshness_class: PolicyFreshnessClass,
    /// Last successful refresh timestamp, when known.
    pub last_refreshed_at: Option<&'a str>,
    /// Last-known-good policy ref, when a fallback exists.
    pub last_known_good_ref: Option<&'a str>,
    /// Whether policy detail is locally inspectable without a hosted console.
    pub local_inspection_available: bool,
    /// Whether a hosted console is required for full detail.
    pub vendor_console_required_for_full_detail: bool,
    /// Action used to open policy details or request policy repair.
    pub policy_detail_action: RetryPathClass,
    /// Export-safe policy-source explanation.
    pub explanation_label: &'a str,
}

/// Offline-entitlement inspector shown on identity-mode rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineEntitlementInspector {
    /// Entitlement state class.
    pub state_class: EntitlementStateClass,
    /// Stable token for [`Self::state_class`].
    pub state_token: String,
    /// Opaque entitlement snapshot ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitlement_snapshot_ref: Option<String>,
    /// Opaque entitlement epoch ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitlement_epoch_ref: Option<String>,
    /// Export-safe seat state label.
    pub seat_state_label: String,
    /// Offline behavior class.
    pub offline_behavior_class: OfflineBehaviorClass,
    /// Stable token for [`Self::offline_behavior_class`].
    pub offline_behavior_token: String,
    /// Export-safe offline behavior label.
    pub offline_behavior_label: String,
    /// Grace expiry timestamp, when a grace window exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grace_expires_at: Option<String>,
    /// Whether local core remains available under this entitlement state.
    pub local_core_available: bool,
    /// Whether new managed actions are blocked.
    pub managed_actions_blocked: bool,
    /// Whether usage or admin export remains available.
    pub usage_or_admin_export_available: bool,
    /// Action used to recover, refresh, or continue locally.
    pub recovery_action: RetryPathClass,
    /// Stable token for [`Self::recovery_action`].
    pub recovery_action_token: String,
    /// Export-safe entitlement explanation.
    pub explanation_label: String,
}

impl OfflineEntitlementInspector {
    /// Builds an offline-entitlement inspector and fills stable tokens.
    pub fn new(request: OfflineEntitlementInspectorRequest<'_>) -> Self {
        Self {
            state_class: request.state_class,
            state_token: request.state_class.as_str().to_owned(),
            entitlement_snapshot_ref: request.entitlement_snapshot_ref.map(str::to_owned),
            entitlement_epoch_ref: request.entitlement_epoch_ref.map(str::to_owned),
            seat_state_label: request.seat_state_label.to_owned(),
            offline_behavior_class: request.offline_behavior_class,
            offline_behavior_token: request.offline_behavior_class.as_str().to_owned(),
            offline_behavior_label: request.offline_behavior_label.to_owned(),
            grace_expires_at: request.grace_expires_at.map(str::to_owned),
            local_core_available: request.local_core_available,
            managed_actions_blocked: request.managed_actions_blocked,
            usage_or_admin_export_available: request.usage_or_admin_export_available,
            recovery_action: request.recovery_action,
            recovery_action_token: request.recovery_action.as_str().to_owned(),
            explanation_label: request.explanation_label.to_owned(),
        }
    }

    /// True when entitlement state and offline behavior are locally explainable.
    pub fn discloses_entitlement_state(&self) -> bool {
        !self.seat_state_label.trim().is_empty()
            && !self.offline_behavior_label.trim().is_empty()
            && !self.explanation_label.trim().is_empty()
            && self.offline_behavior_class != OfflineBehaviorClass::Unknown
    }
}

/// Request used to build an [`OfflineEntitlementInspector`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineEntitlementInspectorRequest<'a> {
    /// Entitlement state class.
    pub state_class: EntitlementStateClass,
    /// Opaque entitlement snapshot ref.
    pub entitlement_snapshot_ref: Option<&'a str>,
    /// Opaque entitlement epoch ref.
    pub entitlement_epoch_ref: Option<&'a str>,
    /// Export-safe seat state label.
    pub seat_state_label: &'a str,
    /// Offline behavior class.
    pub offline_behavior_class: OfflineBehaviorClass,
    /// Export-safe offline behavior label.
    pub offline_behavior_label: &'a str,
    /// Grace expiry timestamp, when a grace window exists.
    pub grace_expires_at: Option<&'a str>,
    /// Whether local core remains available under this entitlement state.
    pub local_core_available: bool,
    /// Whether new managed actions are blocked.
    pub managed_actions_blocked: bool,
    /// Whether usage or admin export remains available.
    pub usage_or_admin_export_available: bool,
    /// Action used to recover, refresh, or continue locally.
    pub recovery_action: RetryPathClass,
    /// Export-safe entitlement explanation.
    pub explanation_label: &'a str,
}

/// Current deployment-boundary disclosure shown on identity-mode rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentBoundaryDisclosure {
    /// Deployment profile class.
    pub deployment_profile_class: IdentityModeDeploymentProfileClass,
    /// Stable token for [`Self::deployment_profile_class`].
    pub deployment_profile_token: String,
    /// Boundary the current deployment actually provides.
    pub current_boundary_class: CurrentDeploymentBoundaryClass,
    /// Stable token for [`Self::current_boundary_class`].
    pub current_boundary_token: String,
    /// Whether a self-hosted path exists for this row.
    pub self_hosted_path_available: bool,
    /// Whether vendor-managed services are active on this row.
    pub managed_services_active: bool,
    /// Residual vendor dependencies disclosed for self-hosted or air-gapped rows.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub residual_vendor_dependency_refs: Vec<String>,
    /// Export-safe boundary label.
    pub boundary_label: String,
}

impl DeploymentBoundaryDisclosure {
    /// Builds a deployment-boundary disclosure and fills stable tokens.
    pub fn new(
        deployment_profile_class: IdentityModeDeploymentProfileClass,
        current_boundary_class: CurrentDeploymentBoundaryClass,
        self_hosted_path_available: bool,
        managed_services_active: bool,
        residual_vendor_dependency_refs: Vec<String>,
        boundary_label: impl Into<String>,
    ) -> Self {
        Self {
            deployment_profile_class,
            deployment_profile_token: deployment_profile_class.as_str().to_owned(),
            current_boundary_class,
            current_boundary_token: current_boundary_class.as_str().to_owned(),
            self_hosted_path_available,
            managed_services_active,
            residual_vendor_dependency_refs,
            boundary_label: boundary_label.into(),
        }
    }

    /// True when the row implies a broader managed boundary than the mode allows.
    pub fn overstates_boundary_for(&self, identity_mode: IdentityModeAlias) -> bool {
        match identity_mode {
            IdentityModeAlias::AccountFreeLocal => {
                self.managed_services_active
                    || !matches!(
                        self.current_boundary_class,
                        CurrentDeploymentBoundaryClass::LocalOnlyNoManagedBoundary
                            | CurrentDeploymentBoundaryClass::AirGappedLocalOnly
                    )
            }
            IdentityModeAlias::SelfHostedOrg => {
                self.managed_services_active
                    || matches!(
                        self.current_boundary_class,
                        CurrentDeploymentBoundaryClass::VendorManagedControlPlane
                    )
            }
            IdentityModeAlias::ManagedConvenience => !matches!(
                self.current_boundary_class,
                CurrentDeploymentBoundaryClass::VendorManagedControlPlane
                    | CurrentDeploymentBoundaryClass::MirrorOrFileImport
            ),
        }
    }
}

/// Refs tying identity-mode rows to existing auth, policy, entitlement, and export truth.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct IdentityModeArtifactRefs {
    /// Boundary manifest ref consumed from governance artifacts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_manifest_ref: Option<String>,
    /// Entitlement snapshot set ref consumed from governance artifacts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitlement_snapshot_set_ref: Option<String>,
    /// Schema registry ref consumed from governance artifacts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_registry_ref: Option<String>,
    /// Record-class registry ref consumed from governance artifacts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_class_registry_ref: Option<String>,
    /// System-browser claimed identity row ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claimed_identity_row_ref: Option<String>,
    /// Credential-state row refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub credential_state_row_refs: Vec<String>,
    /// Support or admin export ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_export_ref: Option<String>,
    /// Execution-context ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
}

/// Request used to stage an [`IdentityModeBaselineRow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageIdentityModeBaselineRowRequest<'a> {
    /// Stable row id.
    pub row_id: &'a str,
    /// Export-safe display label.
    pub display_label: &'a str,
    /// Identity mode.
    pub identity_mode: IdentityModeAlias,
    /// Account boundary class.
    pub account_boundary_class: AccountBoundaryClass,
    /// Workspace trust state.
    pub trust_state: TrustState,
    /// Auth mode class.
    pub auth_mode_class: IdentityAuthModeClass,
    /// Provisioning class.
    pub provisioning_class: ProvisioningClass,
    /// Local-core continuity block.
    pub local_core: LocalCoreContinuity,
    /// Policy-source inspector.
    pub policy_source: IdentityPolicySourceInspector,
    /// Offline-entitlement inspector.
    pub offline_entitlement: OfflineEntitlementInspector,
    /// Deployment-boundary disclosure.
    pub boundary: DeploymentBoundaryDisclosure,
    /// Artifact refs.
    pub artifact_refs: IdentityModeArtifactRefs,
    /// Recovery copy rendered by shell and support rows.
    pub recovery_copy_label: &'a str,
    /// Mint timestamp.
    pub minted_at: &'a str,
}

/// Errors raised while staging identity-mode rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityModeBaselineRowError {
    /// The request did not provide a stable row id.
    EmptyRowId,
    /// The request did not provide a display label.
    EmptyDisplayLabel,
    /// The row would make local core require account creation.
    LocalCoreRequiresAccount,
    /// The row omitted a required local-core capability id.
    MissingLocalCoreCapability(String),
    /// A managed or self-hosted row omitted policy-source disclosure.
    MissingPolicySourceDisclosure,
    /// A managed or self-hosted row omitted entitlement or offline disclosure.
    MissingEntitlementDisclosure,
    /// The row implies a broader managed boundary than the deployment provides.
    OverstatedDeploymentBoundary,
}

impl std::fmt::Display for IdentityModeBaselineRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRowId => f.write_str("identity-mode row id cannot be empty"),
            Self::EmptyDisplayLabel => f.write_str("identity-mode display label cannot be empty"),
            Self::LocalCoreRequiresAccount => {
                f.write_str("local core cannot require account creation")
            }
            Self::MissingLocalCoreCapability(capability) => {
                write!(
                    f,
                    "identity-mode row omitted required local capability {capability}"
                )
            }
            Self::MissingPolicySourceDisclosure => {
                f.write_str("managed or self-hosted row must disclose policy source")
            }
            Self::MissingEntitlementDisclosure => f.write_str(
                "managed or self-hosted row must disclose entitlement and offline behavior",
            ),
            Self::OverstatedDeploymentBoundary => {
                f.write_str("identity-mode row overstates the current deployment boundary")
            }
        }
    }
}

impl std::error::Error for IdentityModeBaselineRowError {}

/// Canonical row for one identity-mode baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityModeBaselineRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Export-safe display label.
    pub display_label: String,
    /// Identity mode.
    pub identity_mode: IdentityModeAlias,
    /// Stable token for [`Self::identity_mode`].
    pub identity_mode_token: String,
    /// Account boundary class.
    pub account_boundary_class: AccountBoundaryClass,
    /// Stable token for [`Self::account_boundary_class`].
    pub account_boundary_class_token: String,
    /// Workspace trust state.
    pub trust_state: TrustState,
    /// Auth mode class.
    pub auth_mode_class: IdentityAuthModeClass,
    /// Stable token for [`Self::auth_mode_class`].
    pub auth_mode_token: String,
    /// Provisioning class.
    pub provisioning_class: ProvisioningClass,
    /// Stable token for [`Self::provisioning_class`].
    pub provisioning_token: String,
    /// Local-core continuity block.
    pub local_core: LocalCoreContinuity,
    /// Policy-source inspector.
    pub policy_source: IdentityPolicySourceInspector,
    /// Offline-entitlement inspector.
    pub offline_entitlement: OfflineEntitlementInspector,
    /// Current deployment-boundary disclosure.
    pub boundary: DeploymentBoundaryDisclosure,
    /// Refs into existing auth, governance, and export artifacts.
    pub artifact_refs: IdentityModeArtifactRefs,
    /// Recovery copy rendered by shell and support rows.
    pub recovery_copy_label: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl IdentityModeBaselineRow {
    /// Stage an identity-mode baseline row and enforce the local-core floor.
    ///
    /// # Errors
    ///
    /// Returns [`IdentityModeBaselineRowError`] when required identity,
    /// local-core, policy-source, entitlement, or boundary fields are missing
    /// or contradictory.
    pub fn stage(
        request: StageIdentityModeBaselineRowRequest<'_>,
    ) -> Result<Self, IdentityModeBaselineRowError> {
        if request.row_id.trim().is_empty() {
            return Err(IdentityModeBaselineRowError::EmptyRowId);
        }
        if request.display_label.trim().is_empty() {
            return Err(IdentityModeBaselineRowError::EmptyDisplayLabel);
        }
        if !request.local_core.account_free_local_core_available() {
            if request.local_core.account_required_for_local_core
                || !request.local_core.local_core_available
            {
                return Err(IdentityModeBaselineRowError::LocalCoreRequiresAccount);
            }
            if let Some(missing) = request.local_core.missing_required_capabilities().first() {
                return Err(IdentityModeBaselineRowError::MissingLocalCoreCapability(
                    (*missing).to_owned(),
                ));
            }
        }
        let org_mode = !matches!(request.identity_mode, IdentityModeAlias::AccountFreeLocal);
        if org_mode && !request.policy_source.discloses_policy_source() {
            return Err(IdentityModeBaselineRowError::MissingPolicySourceDisclosure);
        }
        if org_mode && !request.offline_entitlement.discloses_entitlement_state() {
            return Err(IdentityModeBaselineRowError::MissingEntitlementDisclosure);
        }
        if request
            .boundary
            .overstates_boundary_for(request.identity_mode)
        {
            return Err(IdentityModeBaselineRowError::OverstatedDeploymentBoundary);
        }

        Ok(Self {
            record_kind: IDENTITY_MODE_BASELINE_ROW_RECORD_KIND.to_owned(),
            schema_version: IDENTITY_MODE_BASELINE_SCHEMA_VERSION,
            row_id: request.row_id.to_owned(),
            display_label: request.display_label.to_owned(),
            identity_mode: request.identity_mode,
            identity_mode_token: request.identity_mode.as_str().to_owned(),
            account_boundary_class: request.account_boundary_class,
            account_boundary_class_token: request.account_boundary_class.as_str().to_owned(),
            trust_state: request.trust_state,
            auth_mode_class: request.auth_mode_class,
            auth_mode_token: request.auth_mode_class.as_str().to_owned(),
            provisioning_class: request.provisioning_class,
            provisioning_token: request.provisioning_class.as_str().to_owned(),
            local_core: request.local_core,
            policy_source: request.policy_source,
            offline_entitlement: request.offline_entitlement,
            boundary: request.boundary,
            artifact_refs: request.artifact_refs,
            recovery_copy_label: request.recovery_copy_label.to_owned(),
            minted_at: request.minted_at.to_owned(),
        })
    }

    /// True when local core is available without account creation.
    pub fn account_free_local_core_available(&self) -> bool {
        self.local_core.account_free_local_core_available()
    }

    /// True when the policy-source inspector is locally readable.
    pub fn policy_source_inspectable(&self) -> bool {
        self.policy_source.discloses_policy_source()
    }

    /// True when entitlement and offline behavior are locally readable.
    pub fn entitlement_inspectable(&self) -> bool {
        self.offline_entitlement.discloses_entitlement_state()
    }

    /// True when this row implies a broader managed boundary than is active.
    pub fn overstates_current_boundary(&self) -> bool {
        self.boundary.overstates_boundary_for(self.identity_mode)
    }

    /// True when this row needs visible recovery because managed actions are blocked.
    pub fn visible_recovery_required(&self) -> bool {
        self.offline_entitlement.managed_actions_blocked
            || self
                .offline_entitlement
                .state_class
                .blocks_new_managed_actions()
            || matches!(
                self.policy_source.freshness_class,
                PolicyFreshnessClass::StalePastGrace
                    | PolicyFreshnessClass::OfflineSnapshotExpired
                    | PolicyFreshnessClass::NeverRefreshedSeed
            )
    }
}

/// Shell, CLI, and support projection for one identity-mode row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityModeSurfaceRow {
    /// Source row id.
    pub row_id: String,
    /// Export-safe display label.
    pub display_label: String,
    /// Identity mode token.
    pub identity_mode_token: String,
    /// Account boundary token.
    pub account_boundary_class_token: String,
    /// Deployment profile token.
    pub deployment_profile_token: String,
    /// Current deployment boundary token.
    pub current_boundary_token: String,
    /// Auth mode token.
    pub auth_mode_token: String,
    /// Provisioning token.
    pub provisioning_token: String,
    /// Policy source token.
    pub policy_source_token: String,
    /// Policy freshness token.
    pub policy_freshness_token: String,
    /// Entitlement state token.
    pub entitlement_state_token: String,
    /// Offline behavior token.
    pub offline_behavior_token: String,
    /// Whether local core is available without account creation.
    pub local_core_available_without_account: bool,
    /// Whether the row's policy detail is inspectable.
    pub policy_detail_available: bool,
    /// Whether entitlement detail is inspectable.
    pub entitlement_detail_available: bool,
    /// Whether new managed actions are blocked.
    pub managed_actions_blocked: bool,
    /// Whether visible recovery is required.
    pub visible_recovery_required: bool,
    /// Whether the current boundary is overstated.
    pub overstates_current_boundary: bool,
    /// Export-safe local continuity label.
    pub local_continuity_label: String,
    /// Export-safe policy source label.
    pub policy_source_label: String,
    /// Export-safe entitlement label.
    pub entitlement_label: String,
    /// Export-safe boundary label.
    pub boundary_label: String,
    /// Recovery copy rendered by shell and support rows.
    pub recovery_copy_label: String,
    /// Stay-local action token.
    pub stay_local_action_token: String,
    /// Policy detail action token.
    pub policy_detail_action_token: String,
    /// Entitlement recovery action token.
    pub entitlement_recovery_action_token: String,
    /// Boundary manifest ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_manifest_ref: Option<String>,
    /// Entitlement snapshot set ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitlement_snapshot_set_ref: Option<String>,
    /// Execution-context ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
}

impl IdentityModeSurfaceRow {
    /// Project a shell, CLI, and support row from a baseline row.
    pub fn from_row(row: &IdentityModeBaselineRow) -> Self {
        Self {
            row_id: row.row_id.clone(),
            display_label: row.display_label.clone(),
            identity_mode_token: row.identity_mode_token.clone(),
            account_boundary_class_token: row.account_boundary_class_token.clone(),
            deployment_profile_token: row.boundary.deployment_profile_token.clone(),
            current_boundary_token: row.boundary.current_boundary_token.clone(),
            auth_mode_token: row.auth_mode_token.clone(),
            provisioning_token: row.provisioning_token.clone(),
            policy_source_token: row.policy_source.source_token.clone(),
            policy_freshness_token: row.policy_source.freshness_token.clone(),
            entitlement_state_token: row.offline_entitlement.state_token.clone(),
            offline_behavior_token: row.offline_entitlement.offline_behavior_token.clone(),
            local_core_available_without_account: row.account_free_local_core_available(),
            policy_detail_available: row.policy_source_inspectable(),
            entitlement_detail_available: row.entitlement_inspectable(),
            managed_actions_blocked: row.offline_entitlement.managed_actions_blocked,
            visible_recovery_required: row.visible_recovery_required(),
            overstates_current_boundary: row.overstates_current_boundary(),
            local_continuity_label: row.local_core.local_continuity_label.clone(),
            policy_source_label: row.policy_source.source_label.clone(),
            entitlement_label: row.offline_entitlement.explanation_label.clone(),
            boundary_label: row.boundary.boundary_label.clone(),
            recovery_copy_label: row.recovery_copy_label.clone(),
            stay_local_action_token: row.local_core.stay_local_action_token.clone(),
            policy_detail_action_token: row.policy_source.policy_detail_action_token.clone(),
            entitlement_recovery_action_token: row
                .offline_entitlement
                .recovery_action_token
                .clone(),
            boundary_manifest_ref: row.artifact_refs.boundary_manifest_ref.clone(),
            entitlement_snapshot_set_ref: row.artifact_refs.entitlement_snapshot_set_ref.clone(),
            execution_context_ref: row.artifact_refs.execution_context_ref.clone(),
        }
    }
}

/// Packet grouping identity-mode baseline rows for shell, CLI, and support consumption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityModeBaselinePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Identity-mode rows in stable render order.
    pub identity_mode_rows: Vec<IdentityModeBaselineRow>,
    /// Source contract refs consumed by this packet.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_contract_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

impl IdentityModeBaselinePacket {
    /// Build a packet from identity-mode baseline rows.
    pub fn new(
        packet_id: impl Into<String>,
        identity_mode_rows: Vec<IdentityModeBaselineRow>,
        source_contract_refs: Vec<String>,
        minted_at: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: IDENTITY_MODE_BASELINE_PACKET_RECORD_KIND.to_owned(),
            schema_version: IDENTITY_MODE_BASELINE_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            identity_mode_rows,
            source_contract_refs,
            minted_at: minted_at.into(),
        }
    }

    /// Project all identity-mode rows into shell, CLI, and support rows.
    pub fn surface_rows(&self) -> Vec<IdentityModeSurfaceRow> {
        self.identity_mode_rows
            .iter()
            .map(IdentityModeSurfaceRow::from_row)
            .collect()
    }

    /// True when every row keeps local core available without account creation.
    pub fn local_core_remains_account_free(&self) -> bool {
        self.identity_mode_rows
            .iter()
            .all(IdentityModeBaselineRow::account_free_local_core_available)
    }

    /// True when self-hosted and managed rows disclose policy and entitlement state.
    pub fn org_rows_disclose_policy_entitlement_and_offline_behavior(&self) -> bool {
        self.identity_mode_rows
            .iter()
            .filter(|row| row.identity_mode != IdentityModeAlias::AccountFreeLocal)
            .all(|row| row.policy_source_inspectable() && row.entitlement_inspectable())
    }

    /// True when no row implies a broader managed boundary than is active.
    pub fn no_row_overstates_current_boundary(&self) -> bool {
        self.identity_mode_rows
            .iter()
            .all(|row| !row.overstates_current_boundary())
    }

    /// True when the packet includes the three frozen identity modes.
    pub fn has_required_identity_modes(&self) -> bool {
        self.identity_mode_rows
            .iter()
            .any(|row| row.identity_mode == IdentityModeAlias::AccountFreeLocal)
            && self
                .identity_mode_rows
                .iter()
                .any(|row| row.identity_mode == IdentityModeAlias::SelfHostedOrg)
            && self
                .identity_mode_rows
                .iter()
                .any(|row| row.identity_mode == IdentityModeAlias::ManagedConvenience)
    }

    /// Validate the packet against alpha identity-mode baseline invariants.
    pub fn validate(&self) -> Vec<IdentityModeBaselineViolation> {
        let mut violations = Vec::new();
        if self.record_kind != IDENTITY_MODE_BASELINE_PACKET_RECORD_KIND {
            violations.push(IdentityModeBaselineViolation::WrongPacketRecordKind);
        }
        if self.schema_version != IDENTITY_MODE_BASELINE_SCHEMA_VERSION {
            violations.push(IdentityModeBaselineViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty() {
            violations.push(IdentityModeBaselineViolation::MissingPacketId);
        }
        if self.identity_mode_rows.is_empty() {
            violations.push(IdentityModeBaselineViolation::MissingRows);
        }
        if !self.has_required_identity_modes() {
            violations.push(IdentityModeBaselineViolation::MissingRequiredIdentityMode);
        }
        for row in &self.identity_mode_rows {
            validate_row(row, &mut violations);
        }
        violations
    }
}

/// Validation failures emitted by [`IdentityModeBaselinePacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityModeBaselineViolation {
    /// Packet record kind does not match the identity-mode baseline contract.
    WrongPacketRecordKind,
    /// Row record kind does not match the identity-mode baseline contract.
    WrongRowRecordKind,
    /// Packet or row schema version does not match the contract.
    WrongSchemaVersion,
    /// Packet id is empty.
    MissingPacketId,
    /// Packet carries no rows.
    MissingRows,
    /// Packet does not include account-free, self-hosted, and managed rows.
    MissingRequiredIdentityMode,
    /// A row id is empty.
    MissingRowId,
    /// A stable token field does not match its enum value.
    TokenMismatch(String),
    /// A row requires account creation or omits local-core capability ids.
    LocalCoreNotAccountFree(String),
    /// A managed or self-hosted row omits policy-source disclosure.
    MissingPolicySourceDisclosure(String),
    /// A managed or self-hosted row omits entitlement or offline disclosure.
    MissingEntitlementDisclosure(String),
    /// A row implies a broader managed boundary than the deployment provides.
    OverstatedDeploymentBoundary(String),
}

fn validate_row(
    row: &IdentityModeBaselineRow,
    violations: &mut Vec<IdentityModeBaselineViolation>,
) {
    if row.record_kind != IDENTITY_MODE_BASELINE_ROW_RECORD_KIND {
        violations.push(IdentityModeBaselineViolation::WrongRowRecordKind);
    }
    if row.schema_version != IDENTITY_MODE_BASELINE_SCHEMA_VERSION {
        violations.push(IdentityModeBaselineViolation::WrongSchemaVersion);
    }
    if row.row_id.trim().is_empty() {
        violations.push(IdentityModeBaselineViolation::MissingRowId);
    }
    if row.identity_mode_token != row.identity_mode.as_str() {
        violations.push(IdentityModeBaselineViolation::TokenMismatch(
            row.row_id.clone(),
        ));
    }
    if row.account_boundary_class_token != row.account_boundary_class.as_str()
        || row.auth_mode_token != row.auth_mode_class.as_str()
        || row.provisioning_token != row.provisioning_class.as_str()
        || row.local_core.stay_local_action_token != row.local_core.stay_local_action.as_str()
        || row.policy_source.source_token != row.policy_source.source_class.as_str()
        || row.policy_source.freshness_token != row.policy_source.freshness_class.as_str()
        || row.policy_source.policy_detail_action_token
            != row.policy_source.policy_detail_action.as_str()
        || row.offline_entitlement.state_token != row.offline_entitlement.state_class.as_str()
        || row.offline_entitlement.offline_behavior_token
            != row.offline_entitlement.offline_behavior_class.as_str()
        || row.offline_entitlement.recovery_action_token
            != row.offline_entitlement.recovery_action.as_str()
        || row.boundary.deployment_profile_token != row.boundary.deployment_profile_class.as_str()
        || row.boundary.current_boundary_token != row.boundary.current_boundary_class.as_str()
    {
        violations.push(IdentityModeBaselineViolation::TokenMismatch(
            row.row_id.clone(),
        ));
    }
    if !row.account_free_local_core_available() {
        violations.push(IdentityModeBaselineViolation::LocalCoreNotAccountFree(
            row.row_id.clone(),
        ));
    }
    if row.identity_mode != IdentityModeAlias::AccountFreeLocal && !row.policy_source_inspectable()
    {
        violations
            .push(IdentityModeBaselineViolation::MissingPolicySourceDisclosure(row.row_id.clone()));
    }
    if row.identity_mode != IdentityModeAlias::AccountFreeLocal && !row.entitlement_inspectable() {
        violations.push(IdentityModeBaselineViolation::MissingEntitlementDisclosure(
            row.row_id.clone(),
        ));
    }
    if row.overstates_current_boundary() {
        violations.push(IdentityModeBaselineViolation::OverstatedDeploymentBoundary(
            row.row_id.clone(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local_core() -> LocalCoreContinuity {
        LocalCoreContinuity::new(
            false,
            true,
            REQUIRED_LOCAL_CORE_CAPABILITY_IDS
                .iter()
                .map(|capability| (*capability).to_owned())
                .collect(),
            "Local editing, search, Git, tasks, history, and BYOK AI remain available.",
            RetryPathClass::ContinueLocalWithoutSignIn,
        )
    }

    fn local_policy() -> IdentityPolicySourceInspector {
        IdentityPolicySourceInspector::new(IdentityPolicySourceInspectorRequest {
            source_class: PolicySourceClass::NoPolicyRequiredLocal,
            source_label: "No org policy required",
            policy_bundle_ref: None,
            policy_epoch_ref: None,
            freshness_class: PolicyFreshnessClass::NotApplicableAccountFree,
            last_refreshed_at: None,
            last_known_good_ref: None,
            local_inspection_available: true,
            vendor_console_required_for_full_detail: false,
            policy_detail_action: RetryPathClass::ContinueLocalWithoutSignIn,
            explanation_label: "Local core uses local trust and settings only.",
        })
    }

    fn local_entitlement() -> OfflineEntitlementInspector {
        OfflineEntitlementInspector::new(OfflineEntitlementInspectorRequest {
            state_class: EntitlementStateClass::NotApplicableAccountFree,
            entitlement_snapshot_ref: None,
            entitlement_epoch_ref: None,
            seat_state_label: "No seat required",
            offline_behavior_class: OfflineBehaviorClass::UnlimitedOfflineLocalSafe,
            offline_behavior_label: "Unlimited offline local-safe use.",
            grace_expires_at: None,
            local_core_available: true,
            managed_actions_blocked: false,
            usage_or_admin_export_available: true,
            recovery_action: RetryPathClass::ContinueLocalWithoutSignIn,
            explanation_label: "No entitlement is needed for local core.",
        })
    }

    fn managed_policy() -> IdentityPolicySourceInspector {
        IdentityPolicySourceInspector::new(IdentityPolicySourceInspectorRequest {
            source_class: PolicySourceClass::VendorManagedOrigin,
            source_label: "Vendor-managed signed policy bundle",
            policy_bundle_ref: Some("policy_bundle.alpha.managed.current"),
            policy_epoch_ref: Some("policy_epoch.managed.alpha.0001"),
            freshness_class: PolicyFreshnessClass::AuthoritativeLive,
            last_refreshed_at: Some("2026-05-13T08:00:00Z"),
            last_known_good_ref: Some("policy_bundle.alpha.managed.lkg"),
            local_inspection_available: true,
            vendor_console_required_for_full_detail: false,
            policy_detail_action: RetryPathClass::RequestAdminPolicyChange,
            explanation_label: "Signed managed policy is inspectable from the local packet.",
        })
    }

    fn managed_entitlement() -> OfflineEntitlementInspector {
        OfflineEntitlementInspector::new(OfflineEntitlementInspectorRequest {
            state_class: EntitlementStateClass::Active,
            entitlement_snapshot_ref: Some("entitlement_snapshot.alpha.active"),
            entitlement_epoch_ref: Some("entitlement_epoch.managed.alpha.0001"),
            seat_state_label: "Seat active",
            offline_behavior_class: OfflineBehaviorClass::RefreshRequiredBeforeNewManagedUse,
            offline_behavior_label: "Refresh before new managed use if offline.",
            grace_expires_at: None,
            local_core_available: true,
            managed_actions_blocked: false,
            usage_or_admin_export_available: true,
            recovery_action: RetryPathClass::RetryInSystemBrowser,
            explanation_label:
                "Managed capability state is active; local core remains independent.",
        })
    }

    fn local_row() -> IdentityModeBaselineRow {
        IdentityModeBaselineRow::stage(StageIdentityModeBaselineRowRequest {
            row_id: "identity-mode:account-free-local",
            display_label: "Account-free local",
            identity_mode: IdentityModeAlias::AccountFreeLocal,
            account_boundary_class: AccountBoundaryClass::LocalOnly,
            trust_state: TrustState::Trusted,
            auth_mode_class: IdentityAuthModeClass::AccountFreeLocal,
            provisioning_class: ProvisioningClass::NotApplicableLocal,
            local_core: local_core(),
            policy_source: local_policy(),
            offline_entitlement: local_entitlement(),
            boundary: DeploymentBoundaryDisclosure::new(
                IdentityModeDeploymentProfileClass::IndividualLocal,
                CurrentDeploymentBoundaryClass::LocalOnlyNoManagedBoundary,
                true,
                false,
                Vec::new(),
                "Local desktop only; no managed boundary is active.",
            ),
            artifact_refs: IdentityModeArtifactRefs {
                boundary_manifest_ref: Some(
                    "artifacts/governance/boundary_manifest_alpha.yaml".to_owned(),
                ),
                entitlement_snapshot_set_ref: Some(
                    "artifacts/governance/entitlement_snapshot_alpha.yaml".to_owned(),
                ),
                ..IdentityModeArtifactRefs::default()
            },
            recovery_copy_label: "Continue without sign-in.",
            minted_at: "2026-05-13T08:00:00Z",
        })
        .expect("local row stages")
    }

    fn managed_row() -> IdentityModeBaselineRow {
        IdentityModeBaselineRow::stage(StageIdentityModeBaselineRowRequest {
            row_id: "identity-mode:managed-convenience",
            display_label: "Managed convenience",
            identity_mode: IdentityModeAlias::ManagedConvenience,
            account_boundary_class: AccountBoundaryClass::Managed,
            trust_state: TrustState::Trusted,
            auth_mode_class: IdentityAuthModeClass::SystemBrowserOidc,
            provisioning_class: ProvisioningClass::ManagedSeat,
            local_core: local_core(),
            policy_source: managed_policy(),
            offline_entitlement: managed_entitlement(),
            boundary: DeploymentBoundaryDisclosure::new(
                IdentityModeDeploymentProfileClass::ManagedCloud,
                CurrentDeploymentBoundaryClass::VendorManagedControlPlane,
                true,
                true,
                Vec::new(),
                "Vendor-managed services are active only for managed capabilities.",
            ),
            artifact_refs: IdentityModeArtifactRefs::default(),
            recovery_copy_label: "Refresh managed state or continue local.",
            minted_at: "2026-05-13T08:00:00Z",
        })
        .expect("managed row stages")
    }

    fn self_hosted_row() -> IdentityModeBaselineRow {
        IdentityModeBaselineRow::stage(StageIdentityModeBaselineRowRequest {
            row_id: "identity-mode:self-hosted-org",
            display_label: "Self-hosted organization",
            identity_mode: IdentityModeAlias::SelfHostedOrg,
            account_boundary_class: AccountBoundaryClass::SelfHosted,
            trust_state: TrustState::Trusted,
            auth_mode_class: IdentityAuthModeClass::SystemBrowserOidc,
            provisioning_class: ProvisioningClass::ScimProvisioned,
            local_core: local_core(),
            policy_source: IdentityPolicySourceInspector::new(
                IdentityPolicySourceInspectorRequest {
                    source_class: PolicySourceClass::CustomerSelfHostedOrigin,
                    source_label: "Customer self-hosted signed policy bundle",
                    policy_bundle_ref: Some("policy_bundle.alpha.self_hosted.current"),
                    policy_epoch_ref: Some("policy_epoch.self_hosted.alpha.0001"),
                    freshness_class: PolicyFreshnessClass::OfflineSnapshotUnexpired,
                    last_refreshed_at: Some("2026-05-13T07:55:00Z"),
                    last_known_good_ref: Some("policy_bundle.alpha.self_hosted.lkg"),
                    local_inspection_available: true,
                    vendor_console_required_for_full_detail: false,
                    policy_detail_action: RetryPathClass::RequestAdminPolicyChange,
                    explanation_label: "Customer-hosted signed policy remains locally inspectable.",
                },
            ),
            offline_entitlement: OfflineEntitlementInspector::new(
                OfflineEntitlementInspectorRequest {
                    state_class: EntitlementStateClass::OfflineLastKnownGood,
                    entitlement_snapshot_ref: Some("entitlement_snapshot.alpha.self_hosted.lkg"),
                    entitlement_epoch_ref: Some("entitlement_epoch.self_hosted.alpha.0001"),
                    seat_state_label: "Signed snapshot imported",
                    offline_behavior_class: OfflineBehaviorClass::LastKnownGoodLocalSafeOnly,
                    offline_behavior_label: "Local-safe behavior remains available offline.",
                    grace_expires_at: None,
                    local_core_available: true,
                    managed_actions_blocked: true,
                    usage_or_admin_export_available: true,
                    recovery_action: RetryPathClass::ImportSignedSessionSnapshot,
                    explanation_label:
                        "Self-hosted managed actions wait for refresh; local work continues.",
                },
            ),
            boundary: DeploymentBoundaryDisclosure::new(
                IdentityModeDeploymentProfileClass::SelfHosted,
                CurrentDeploymentBoundaryClass::CustomerSelfHostedControlPlane,
                true,
                false,
                Vec::new(),
                "Customer self-hosted control plane is active.",
            ),
            artifact_refs: IdentityModeArtifactRefs::default(),
            recovery_copy_label: "Import a fresh signed snapshot or continue local.",
            minted_at: "2026-05-13T08:00:00Z",
        })
        .expect("self-hosted row stages")
    }

    #[test]
    fn baseline_packet_validates_required_modes_and_inspectors() {
        let packet = IdentityModeBaselinePacket::new(
            "identity-mode-baseline:test",
            vec![local_row(), self_hosted_row(), managed_row()],
            vec![
                "artifacts/governance/boundary_manifest_alpha.yaml".to_owned(),
                "artifacts/governance/entitlement_snapshot_alpha.yaml".to_owned(),
            ],
            "2026-05-13T08:00:00Z",
        );

        assert_eq!(packet.validate(), Vec::new());
        assert!(packet.has_required_identity_modes());
        assert!(packet.local_core_remains_account_free());
        assert!(packet.org_rows_disclose_policy_entitlement_and_offline_behavior());
        assert!(packet.no_row_overstates_current_boundary());

        let surface_rows = packet.surface_rows();
        assert_eq!(surface_rows.len(), 3);
        assert!(surface_rows
            .iter()
            .all(|row| row.local_core_available_without_account));
        assert!(surface_rows
            .iter()
            .any(|row| row.identity_mode_token == "managed_convenience"
                && row.current_boundary_token == "vendor_managed_control_plane"));
    }

    #[test]
    fn staging_rejects_local_core_account_requirement() {
        let err = IdentityModeBaselineRow::stage(StageIdentityModeBaselineRowRequest {
            row_id: "identity-mode:bad-local",
            display_label: "Bad local",
            identity_mode: IdentityModeAlias::AccountFreeLocal,
            account_boundary_class: AccountBoundaryClass::LocalOnly,
            trust_state: TrustState::Trusted,
            auth_mode_class: IdentityAuthModeClass::AccountFreeLocal,
            provisioning_class: ProvisioningClass::NotApplicableLocal,
            local_core: LocalCoreContinuity::new(
                true,
                false,
                Vec::new(),
                "Account required.",
                RetryPathClass::ContinueLocalWithoutSignIn,
            ),
            policy_source: local_policy(),
            offline_entitlement: local_entitlement(),
            boundary: DeploymentBoundaryDisclosure::new(
                IdentityModeDeploymentProfileClass::IndividualLocal,
                CurrentDeploymentBoundaryClass::LocalOnlyNoManagedBoundary,
                true,
                false,
                Vec::new(),
                "Local only.",
            ),
            artifact_refs: IdentityModeArtifactRefs::default(),
            recovery_copy_label: "Continue local.",
            minted_at: "2026-05-13T08:00:00Z",
        })
        .expect_err("local core account gate must be rejected");

        assert_eq!(err, IdentityModeBaselineRowError::LocalCoreRequiresAccount);
    }

    #[test]
    fn staging_rejects_overstated_self_hosted_boundary() {
        let err = IdentityModeBaselineRow::stage(StageIdentityModeBaselineRowRequest {
            row_id: "identity-mode:self-hosted-overstated",
            display_label: "Self-hosted overstated",
            identity_mode: IdentityModeAlias::SelfHostedOrg,
            account_boundary_class: AccountBoundaryClass::SelfHosted,
            trust_state: TrustState::Trusted,
            auth_mode_class: IdentityAuthModeClass::SystemBrowserOidc,
            provisioning_class: ProvisioningClass::ScimProvisioned,
            local_core: local_core(),
            policy_source: managed_policy(),
            offline_entitlement: managed_entitlement(),
            boundary: DeploymentBoundaryDisclosure::new(
                IdentityModeDeploymentProfileClass::SelfHosted,
                CurrentDeploymentBoundaryClass::VendorManagedControlPlane,
                true,
                true,
                Vec::new(),
                "Incorrectly claims a vendor-managed control plane.",
            ),
            artifact_refs: IdentityModeArtifactRefs::default(),
            recovery_copy_label: "Continue local.",
            minted_at: "2026-05-13T08:00:00Z",
        })
        .expect_err("self-hosted row must not imply managed boundary");

        assert_eq!(
            err,
            IdentityModeBaselineRowError::OverstatedDeploymentBoundary
        );
    }
}
