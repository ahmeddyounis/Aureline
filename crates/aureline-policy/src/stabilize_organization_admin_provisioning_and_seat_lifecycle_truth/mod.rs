//! Organization-admin, provisioning-source, seat-lifecycle, rollout-ring, and
//! local-safety truth for enterprise-managed rows.
//!
//! This module publishes the canonical packet consumed by admin, diagnostics,
//! About, Help, support export, and evidence surfaces when a row claims an
//! enterprise-managed boundary. The packet keeps tenant identity, deployment
//! mode, provisioning class, seat lifecycle, rollout ring, and deprovision
//! local-safety guarantees in one typed record so surfaces do not clone prose or
//! collapse failures into generic sign-in copy.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version carried on every organization-admin truth record.
pub const ORGANIZATION_ADMIN_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Shared contract reference carried by every organization-admin truth record.
pub const ORGANIZATION_ADMIN_TRUTH_SHARED_CONTRACT_REF: &str =
    "policy:organization_admin_provisioning_seat_lifecycle_truth:v1";

/// Record-kind tag for [`OrganizationAdminTruthPage`] payloads.
pub const ORGANIZATION_ADMIN_TRUTH_PAGE_RECORD_KIND: &str =
    "policy_organization_admin_truth_page_record";

/// Record-kind tag for [`OrganizationOverviewCard`] payloads.
pub const ORGANIZATION_OVERVIEW_CARD_RECORD_KIND: &str = "policy_organization_overview_card_record";

/// Record-kind tag for [`DirectoryProviderCard`] payloads.
pub const DIRECTORY_PROVIDER_CARD_RECORD_KIND: &str = "policy_directory_provider_card_record";

/// Record-kind tag for [`SeatLifecycleRow`] payloads.
pub const SEAT_LIFECYCLE_ROW_RECORD_KIND: &str = "policy_seat_lifecycle_row_record";

/// Record-kind tag for [`LifecycleImpactPreview`] payloads.
pub const LIFECYCLE_IMPACT_PREVIEW_RECORD_KIND: &str = "policy_lifecycle_impact_preview_record";

/// Record-kind tag for [`RolloutRingAuditRow`] payloads.
pub const ROLLOUT_RING_AUDIT_ROW_RECORD_KIND: &str = "policy_rollout_ring_audit_row_record";

/// Record-kind tag for [`OrganizationAdminTruthDefect`] payloads.
pub const ORGANIZATION_ADMIN_TRUTH_DEFECT_RECORD_KIND: &str =
    "policy_organization_admin_truth_defect_record";

/// Record-kind tag for [`OrganizationAdminTruthSupportExport`] payloads.
pub const ORGANIZATION_ADMIN_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_organization_admin_truth_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const ORGANIZATION_ADMIN_TRUTH_DOC_REF: &str =
    "docs/enterprise/m4/stabilize-organization-admin-provisioning-and-seat-lifecycle-truth.md";

/// Repo-relative path of the artifact summary for this lane.
pub const ORGANIZATION_ADMIN_TRUTH_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/stabilize-organization-admin-provisioning-and-seat-lifecycle-truth.md";

/// Deployment mode disclosed on the organization overview surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationDeploymentMode {
    /// Desktop-local profile with no managed control plane.
    IndividualLocal,
    /// Customer-operated control plane.
    SelfHostedControlPlane,
    /// Enterprise online mode with controlled outbound access.
    EnterpriseOnline,
    /// Vendor-managed cloud control plane.
    ManagedCloud,
    /// Air-gapped or mirror-only operation.
    AirGapped,
}

impl OrganizationDeploymentMode {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndividualLocal => "individual_local",
            Self::SelfHostedControlPlane => "self_hosted_control_plane",
            Self::EnterpriseOnline => "enterprise_online",
            Self::ManagedCloud => "managed_cloud",
            Self::AirGapped => "air_gapped",
        }
    }
}

/// Provisioning source class for provider and lifecycle rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationProvisioningClass {
    /// OpenID Connect identity source.
    Oidc,
    /// SCIM lifecycle source.
    Scim,
    /// Signed file or bundle lifecycle source.
    SignedFileBundle,
    /// Manual administration path.
    Manual,
}

impl OrganizationProvisioningClass {
    /// All provisioning classes required in enterprise lab fixtures.
    pub const ALL: [Self; 4] = [Self::Oidc, Self::Scim, Self::SignedFileBundle, Self::Manual];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Oidc => "oidc",
            Self::Scim => "scim",
            Self::SignedFileBundle => "signed_file_bundle",
            Self::Manual => "manual",
        }
    }
}

/// Provider operational state shown on directory/provider cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderStateClass {
    /// Provider sync is healthy.
    SyncHealthy,
    /// Provider data is served from cache.
    Cached,
    /// Provider is degraded but local-safe operation remains available.
    Degraded,
    /// Provider is blocked for a typed reason.
    Blocked,
    /// Manual path is the active path.
    ManualOnly,
}

impl ProviderStateClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncHealthy => "sync_healthy",
            Self::Cached => "cached",
            Self::Degraded => "degraded",
            Self::Blocked => "blocked",
            Self::ManualOnly => "manual_only",
        }
    }

    /// True when the provider state needs a specific failure kind.
    pub const fn requires_failure_kind(self) -> bool {
        matches!(self, Self::Degraded | Self::Blocked)
    }
}

/// Freshness posture for provider, lifecycle, and rollout state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminSyncFreshnessClass {
    /// Live sync inside the freshness window.
    Live,
    /// Cached sync still inside the grace window.
    Cached,
    /// Stale sync outside the preferred freshness window.
    Stale,
    /// Expired sync outside the usable grace window.
    Expired,
    /// No successful sync exists.
    Missing,
}

impl AdminSyncFreshnessClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Missing => "missing",
        }
    }
}

/// Specific failure kind for provisioning and policy-target failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminFailureKind {
    /// Provider endpoint outage.
    ProviderOutage,
    /// Local auth state drifted from the provider or tenant expectation.
    AuthDrift,
    /// Resolved scope does not match the requested target population.
    ScopeMismatch,
    /// Seat was removed, lost, or transferred away.
    SeatLoss,
    /// Region or policy blocker prevents the managed lane.
    RegionPolicyBlocker,
    /// Deprovisioning impact must be reviewed before commit.
    DeprovisioningImpact,
}

impl AdminFailureKind {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderOutage => "provider_outage",
            Self::AuthDrift => "auth_drift",
            Self::ScopeMismatch => "scope_mismatch",
            Self::SeatLoss => "seat_loss",
            Self::RegionPolicyBlocker => "region_policy_blocker",
            Self::DeprovisioningImpact => "deprovisioning_impact",
        }
    }

    fn is_known_token(token: &str) -> bool {
        matches!(
            token,
            "provider_outage"
                | "auth_drift"
                | "scope_mismatch"
                | "seat_loss"
                | "region_policy_blocker"
                | "deprovisioning_impact"
        )
    }
}

/// Seat class shown in user or seat lifecycle rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeatClass {
    /// Seat is governed by enterprise lifecycle and policy.
    EnterpriseManaged,
    /// Seat has managed AI entitlement.
    ManagedAi,
    /// Seat has reviewer-only managed entitlement.
    Reviewer,
    /// Seat has only local continuation rights.
    LocalOnlyContinuation,
}

impl SeatClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnterpriseManaged => "enterprise_managed",
            Self::ManagedAi => "managed_ai",
            Self::Reviewer => "reviewer",
            Self::LocalOnlyContinuation => "local_only_continuation",
        }
    }
}

/// Seat lifecycle state shown in user or seat lifecycle rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeatLifecycleStateClass {
    /// Seat is active.
    Active,
    /// Seat is pending activation.
    Pending,
    /// Seat is suspended.
    Suspended,
    /// Seat is inside an offline or admin grace window.
    GraceWindow,
    /// Seat was transferred.
    Transferred,
    /// Seat was downgraded.
    Downgraded,
    /// Seat was deprovisioned.
    Deprovisioned,
}

impl SeatLifecycleStateClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Pending => "pending",
            Self::Suspended => "suspended",
            Self::GraceWindow => "grace_window",
            Self::Transferred => "transferred",
            Self::Downgraded => "downgraded",
            Self::Deprovisioned => "deprovisioned",
        }
    }
}

/// Lifecycle flow class covered by an impact preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleFlowClass {
    /// Seat transfer impact preview.
    SeatTransfer,
    /// Suspension impact preview.
    Suspension,
    /// Downgrade impact preview.
    Downgrade,
    /// Organization switch impact preview.
    OrgSwitch,
    /// Deprovision impact preview.
    Deprovision,
}

impl LifecycleFlowClass {
    /// All lifecycle impact previews required for stable qualification.
    pub const ALL: [Self; 5] = [
        Self::SeatTransfer,
        Self::Suspension,
        Self::Downgrade,
        Self::OrgSwitch,
        Self::Deprovision,
    ];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SeatTransfer => "seat_transfer",
            Self::Suspension => "suspension",
            Self::Downgrade => "downgrade",
            Self::OrgSwitch => "org_switch",
            Self::Deprovision => "deprovision",
        }
    }
}

/// Entitlement impact shown by lifecycle previews.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntitlementImpactClass {
    /// Entitlement remains unchanged.
    Unchanged,
    /// Entitlement is narrowed but not removed.
    Narrowed,
    /// Entitlement is disabled.
    Disabled,
    /// Entitlement requires admin review before continuing.
    RequiresReview,
    /// Local-only capability remains available.
    LocalOnlyAvailable,
}

impl EntitlementImpactClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Narrowed => "narrowed",
            Self::Disabled => "disabled",
            Self::RequiresReview => "requires_review",
            Self::LocalOnlyAvailable => "local_only_available",
        }
    }
}

/// Rollout ring class shown in fleet and support surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutRingClass {
    /// Stable rollout ring.
    Stable,
    /// Pilot rollout ring.
    Pilot,
    /// Canary rollout ring.
    Canary,
    /// Beta rollout ring.
    Beta,
    /// Local mirror ring.
    LocalMirror,
}

impl RolloutRingClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Pilot => "pilot",
            Self::Canary => "canary",
            Self::Beta => "beta",
            Self::LocalMirror => "local_mirror",
        }
    }
}

/// Rollout ring state shown in fleet and support surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutRingStateClass {
    /// Ring is current.
    Current,
    /// Ring has pending rollout.
    Pending,
    /// Ring is drifted.
    Drifted,
    /// Ring is stale.
    Stale,
    /// Ring is operating from offline cache only.
    OfflineCacheOnly,
    /// Ring has signature verification failure.
    VerificationFailed,
    /// Ring is eligible for rollback.
    RollbackEligible,
}

impl RolloutRingStateClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Pending => "pending",
            Self::Drifted => "drifted",
            Self::Stale => "stale",
            Self::OfflineCacheOnly => "offline_cache_only",
            Self::VerificationFailed => "verification_failed",
            Self::RollbackEligible => "rollback_eligible",
        }
    }
}

/// Derived qualification tier for the organization-admin truth page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationAdminTruthQualificationClass {
    /// All required conditions hold.
    Stable,
    /// Non-critical defects are present.
    Beta,
    /// Required coverage is missing.
    Preview,
    /// A hard local-safety or raw-material guardrail was triggered.
    Withdrawn,
}

impl OrganizationAdminTruthQualificationClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }
}

/// Typed reason the page was narrowed below stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationAdminTruthNarrowReasonClass {
    /// No narrowing.
    NotNarrowed,
    /// The organization overview is missing required truth.
    OverviewTruthMissing,
    /// A required provisioning source class is absent.
    MissingProvisioningClassCoverage,
    /// A provider card is missing required state, freshness, scope, or fallback.
    ProviderCardIncomplete,
    /// A provider or dry-run failure does not name a specific failure kind.
    FailureKindNotSpecific,
    /// A seat lifecycle row is missing principal, lifecycle, safety, or lineage truth.
    SeatLifecycleTruthMissing,
    /// A required lifecycle impact preview is absent.
    MissingImpactPreviewCoverage,
    /// A lifecycle impact preview hides local continuation or export rights.
    ImpactPreviewTruthMissing,
    /// Rollout-ring audit truth is absent or incomplete.
    RolloutRingTruthMissing,
    /// Tenant, rollout, or provisioning source is not visible on all required surfaces.
    BoundaryVisibilityIncomplete,
    /// Raw secret, raw token, private key, or private payload material crossed the boundary.
    RawPrivateMaterialExposed,
    /// Local editing, local history, unsaved work, or export/offboarding rights are not preserved.
    LocalSafetyGuaranteeMissing,
}

impl OrganizationAdminTruthNarrowReasonClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::OverviewTruthMissing => "overview_truth_missing",
            Self::MissingProvisioningClassCoverage => "missing_provisioning_class_coverage",
            Self::ProviderCardIncomplete => "provider_card_incomplete",
            Self::FailureKindNotSpecific => "failure_kind_not_specific",
            Self::SeatLifecycleTruthMissing => "seat_lifecycle_truth_missing",
            Self::MissingImpactPreviewCoverage => "missing_impact_preview_coverage",
            Self::ImpactPreviewTruthMissing => "impact_preview_truth_missing",
            Self::RolloutRingTruthMissing => "rollout_ring_truth_missing",
            Self::BoundaryVisibilityIncomplete => "boundary_visibility_incomplete",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::LocalSafetyGuaranteeMissing => "local_safety_guarantee_missing",
        }
    }

    /// True when this reason withdraws the stable claim immediately.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::RawPrivateMaterialExposed | Self::LocalSafetyGuaranteeMissing
        )
    }

    /// True when this reason narrows the packet to preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(
            self,
            Self::MissingProvisioningClassCoverage
                | Self::MissingImpactPreviewCoverage
                | Self::RolloutRingTruthMissing
        )
    }
}

/// Surface visibility flags for tenant, provisioning, rollout, and local-safety truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceVisibility {
    /// True when the admin surface shows this truth.
    pub admin: bool,
    /// True when diagnostics show this truth.
    pub diagnostics: bool,
    /// True when About shows this truth.
    pub about: bool,
    /// True when Help shows this truth.
    pub help: bool,
    /// True when support packets include this truth.
    pub support_packet: bool,
    /// True when export packets include this truth.
    pub export_packet: bool,
}

impl SurfaceVisibility {
    /// Returns a visibility declaration with every required surface enabled.
    pub const fn all_required() -> Self {
        Self {
            admin: true,
            diagnostics: true,
            about: true,
            help: true,
            support_packet: true,
            export_packet: true,
        }
    }

    /// True when all required surfaces carry the truth.
    pub const fn all_visible(&self) -> bool {
        self.admin
            && self.diagnostics
            && self.about
            && self.help
            && self.support_packet
            && self.export_packet
    }
}

/// Admin action and result lineage attached to lifecycle-impacting rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminLifecycleActionLineage {
    /// Action token such as `seat_transfer`, `suspend`, or `deprovision`.
    pub action_token: String,
    /// Export-safe action label.
    pub action_label: String,
    /// Result token such as `succeeded`, `previewed`, `blocked`, or `narrowed`.
    pub result_token: String,
    /// Export-safe result label.
    pub result_label: String,
    /// Opaque actor reference.
    pub actor_ref: String,
    /// UTC timestamp when the action was applied or previewed.
    pub applied_at: String,
    /// Opaque audit transaction reference.
    pub transaction_id: String,
}

/// Seat-count summary shown on the organization overview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeatSummary {
    /// Number of active seats.
    pub active: usize,
    /// Number of pending seats.
    pub pending: usize,
    /// Number of suspended seats.
    pub suspended: usize,
    /// Number of seats inside a grace window.
    pub grace_window: usize,
    /// Number of deprovisioned seats retained in audit/offboarding state.
    pub deprovisioned: usize,
}

/// Organization or tenant overview card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationOverviewCard {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// Opaque organization and tenant reference shown across surfaces.
    pub org_tenant_ref: String,
    /// Deployment mode.
    pub deployment_mode: OrganizationDeploymentMode,
    /// Stable token for [`Self::deployment_mode`].
    pub deployment_mode_token: String,
    /// Policy source label with freshness.
    pub policy_source_label: String,
    /// Seat summary counts.
    pub seat_summary: SeatSummary,
    /// Rollout-ring summary label.
    pub rollout_ring_summary: String,
    /// UTC timestamp for last successful identity, policy, or fleet sync.
    pub last_successful_sync: String,
    /// Admin export action label.
    pub export_action_label: String,
    /// Support packet action label.
    pub support_action_label: String,
    /// Required surface visibility declaration.
    pub surface_visibility: SurfaceVisibility,
    /// True when raw secret, token, key, and private payload material is excluded.
    pub raw_private_material_excluded: bool,
}

/// Directory or provider card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectoryProviderCard {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// Stable provider card id.
    pub provider_id: String,
    /// Opaque organization and tenant reference.
    pub org_tenant_ref: String,
    /// Rollout ring reference visible beside the provider row.
    pub rollout_ring_ref: String,
    /// Provisioning class.
    pub provisioning_class: OrganizationProvisioningClass,
    /// Stable token for [`Self::provisioning_class`].
    pub provisioning_class_token: String,
    /// Provider state.
    pub provider_state: ProviderStateClass,
    /// Stable token for [`Self::provider_state`].
    pub provider_state_token: String,
    /// Sync freshness.
    pub sync_freshness: AdminSyncFreshnessClass,
    /// Stable token for [`Self::sync_freshness`].
    pub sync_freshness_token: String,
    /// UTC timestamp for the last successful provider sync.
    pub last_successful_sync: String,
    /// Export-safe scope summary.
    pub scope_summary_label: String,
    /// Managed user count resolved by this provider.
    pub managed_user_count: usize,
    /// Managed group count resolved by this provider.
    pub managed_group_count: usize,
    /// Fallback or manual path label.
    pub fallback_manual_path_label: String,
    /// Specific failure kind when the provider state is degraded or blocked.
    pub failure_kind: Option<AdminFailureKind>,
    /// Stable token for [`Self::failure_kind`], or empty when none.
    pub failure_kind_token: String,
    /// Required surface visibility declaration.
    pub surface_visibility: SurfaceVisibility,
    /// True when raw secret, token, key, and private payload material is excluded.
    pub raw_private_material_excluded: bool,
}

/// User or seat lifecycle row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeatLifecycleRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// Stable lifecycle row id.
    pub row_id: String,
    /// Opaque organization and tenant reference.
    pub org_tenant_ref: String,
    /// Rollout ring reference visible beside the lifecycle row.
    pub rollout_ring_ref: String,
    /// Opaque principal reference.
    pub principal_ref: String,
    /// Source of truth label.
    pub source_of_truth_label: String,
    /// Role label.
    pub role_label: String,
    /// Seat class.
    pub seat_class: SeatClass,
    /// Stable token for [`Self::seat_class`].
    pub seat_class_token: String,
    /// Lifecycle state.
    pub lifecycle_state: SeatLifecycleStateClass,
    /// Stable token for [`Self::lifecycle_state`].
    pub lifecycle_state_token: String,
    /// UTC timestamp when this principal was last seen.
    pub last_seen: String,
    /// Local-artifact safety note shown in admin and user-facing flows.
    pub local_artifact_safety_note: String,
    /// True when local editing remains available.
    pub local_editing_preserved: bool,
    /// True when local history remains available.
    pub local_history_preserved: bool,
    /// True when export and offboarding rights remain available.
    pub export_offboarding_available: bool,
    /// Admin action/result lineage for the current lifecycle state.
    pub action_lineage: AdminLifecycleActionLineage,
    /// Impact preview ids that explain this row's likely transitions.
    pub impact_preview_refs: Vec<String>,
    /// Required surface visibility declaration.
    pub surface_visibility: SurfaceVisibility,
    /// True when raw secret, token, key, and private payload material is excluded.
    pub raw_private_material_excluded: bool,
}

/// Impact preview for seat transfer, suspension, downgrade, org switch, or deprovision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleImpactPreview {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// Stable preview id.
    pub preview_id: String,
    /// Lifecycle flow class.
    pub flow_class: LifecycleFlowClass,
    /// Stable token for [`Self::flow_class`].
    pub flow_class_token: String,
    /// Opaque principal reference.
    pub principal_ref: String,
    /// Impact on managed AI entitlement.
    pub managed_ai_impact: EntitlementImpactClass,
    /// Stable token for [`Self::managed_ai_impact`].
    pub managed_ai_impact_token: String,
    /// Impact on sync entitlement.
    pub sync_impact: EntitlementImpactClass,
    /// Stable token for [`Self::sync_impact`].
    pub sync_impact_token: String,
    /// Impact on collaboration entitlement.
    pub collaboration_impact: EntitlementImpactClass,
    /// Stable token for [`Self::collaboration_impact`].
    pub collaboration_impact_token: String,
    /// Impact on review entitlement.
    pub review_impact: EntitlementImpactClass,
    /// Stable token for [`Self::review_impact`].
    pub review_impact_token: String,
    /// Impact on marketplace entitlement.
    pub marketplace_impact: EntitlementImpactClass,
    /// Stable token for [`Self::marketplace_impact`].
    pub marketplace_impact_token: String,
    /// Label describing local-only continuation.
    pub local_only_continuation_label: String,
    /// Label describing export and offboarding rights.
    pub export_rights_label: String,
    /// True when local editing remains available.
    pub local_editing_preserved: bool,
    /// True when local history remains available.
    pub local_history_preserved: bool,
    /// True when unsaved work remains recoverable.
    pub unsaved_work_preserved: bool,
    /// Specific failure kind when this preview represents a failure or narrowing.
    pub failure_kind: Option<AdminFailureKind>,
    /// Stable token for [`Self::failure_kind`], or empty when none.
    pub failure_kind_token: String,
    /// Admin action/result lineage for the preview.
    pub action_lineage: AdminLifecycleActionLineage,
    /// True when raw secret, token, key, and private payload material is excluded.
    pub raw_private_material_excluded: bool,
}

/// Rollout-ring audit row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutRingAuditRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// Stable ring id.
    pub ring_id: String,
    /// Opaque organization and tenant reference.
    pub org_tenant_ref: String,
    /// Rollout ring class.
    pub ring_class: RolloutRingClass,
    /// Stable token for [`Self::ring_class`].
    pub ring_class_token: String,
    /// Rollout ring state.
    pub ring_state: RolloutRingStateClass,
    /// Stable token for [`Self::ring_state`].
    pub ring_state_token: String,
    /// Policy source label with freshness.
    pub policy_source_label: String,
    /// Build or channel summary label.
    pub build_channel_summary: String,
    /// Number of enrolled devices.
    pub enrolled_device_count: usize,
    /// Number of drifted devices.
    pub drifted_device_count: usize,
    /// UTC timestamp when the ring was last audited.
    pub last_audited: String,
    /// Local-safety note for rollback or ring changes.
    pub local_safety_note: String,
    /// Rollback or support action label.
    pub rollback_action_label: String,
    /// Required surface visibility declaration.
    pub surface_visibility: SurfaceVisibility,
    /// True when raw secret, token, key, and private payload material is excluded.
    pub raw_private_material_excluded: bool,
}

/// Aggregate summary for the organization-admin truth page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OrganizationAdminTruthSummary {
    /// Number of provider cards.
    pub provider_card_count: usize,
    /// Number of seat lifecycle rows.
    pub seat_lifecycle_row_count: usize,
    /// Number of lifecycle impact previews.
    pub impact_preview_count: usize,
    /// Number of rollout-ring audit rows.
    pub rollout_ring_count: usize,
    /// Provisioning classes present across provider cards.
    pub provisioning_classes_present: Vec<String>,
    /// Lifecycle flows present across impact previews.
    pub lifecycle_flows_present: Vec<String>,
    /// Rollout rings present across rollout audit rows.
    pub rollout_rings_present: Vec<String>,
    /// Failure kinds present across provider cards and impact previews.
    pub failure_kinds_present: Vec<String>,
    /// Number of lifecycle rows preserving local editing, local history, and export rights.
    pub local_safe_lifecycle_row_count: usize,
    /// Number of impact previews preserving local editing, local history, and unsaved work.
    pub local_safe_impact_preview_count: usize,
    /// Number of typed defects.
    pub defect_count: usize,
    /// Overall qualification token.
    pub overall_qualification_token: String,
    /// Overall narrow reason token.
    pub overall_narrow_reason_token: String,
}

impl OrganizationAdminTruthSummary {
    fn from_records(
        provider_cards: &[DirectoryProviderCard],
        seat_lifecycle_rows: &[SeatLifecycleRow],
        impact_previews: &[LifecycleImpactPreview],
        rollout_rings: &[RolloutRingAuditRow],
        defects: &[OrganizationAdminTruthDefect],
    ) -> Self {
        let mut provisioning_classes = BTreeSet::new();
        let mut lifecycle_flows = BTreeSet::new();
        let mut rollout_rings_present = BTreeSet::new();
        let mut failure_kinds = BTreeSet::new();

        for provider in provider_cards {
            provisioning_classes.insert(provider.provisioning_class_token.clone());
            if !provider.failure_kind_token.is_empty() {
                failure_kinds.insert(provider.failure_kind_token.clone());
            }
        }
        for preview in impact_previews {
            lifecycle_flows.insert(preview.flow_class_token.clone());
            if !preview.failure_kind_token.is_empty() {
                failure_kinds.insert(preview.failure_kind_token.clone());
            }
        }
        for ring in rollout_rings {
            rollout_rings_present.insert(ring.ring_class_token.clone());
        }

        let local_safe_lifecycle_row_count = seat_lifecycle_rows
            .iter()
            .filter(|row| {
                row.local_editing_preserved
                    && row.local_history_preserved
                    && row.export_offboarding_available
                    && !row.local_artifact_safety_note.is_empty()
            })
            .count();
        let local_safe_impact_preview_count = impact_previews
            .iter()
            .filter(|preview| {
                preview.local_editing_preserved
                    && preview.local_history_preserved
                    && preview.unsaved_work_preserved
                    && !preview.local_only_continuation_label.is_empty()
                    && !preview.export_rights_label.is_empty()
            })
            .count();

        let overall_reason = defects
            .iter()
            .map(|d| d.narrow_reason)
            .max()
            .unwrap_or(OrganizationAdminTruthNarrowReasonClass::NotNarrowed);
        let qualification = if defects
            .iter()
            .any(|d| d.narrow_reason.is_withdrawal_reason())
        {
            OrganizationAdminTruthQualificationClass::Withdrawn
        } else if defects.iter().any(|d| d.narrow_reason.is_preview_reason()) {
            OrganizationAdminTruthQualificationClass::Preview
        } else if defects.is_empty() {
            OrganizationAdminTruthQualificationClass::Stable
        } else {
            OrganizationAdminTruthQualificationClass::Beta
        };

        Self {
            provider_card_count: provider_cards.len(),
            seat_lifecycle_row_count: seat_lifecycle_rows.len(),
            impact_preview_count: impact_previews.len(),
            rollout_ring_count: rollout_rings.len(),
            provisioning_classes_present: provisioning_classes.into_iter().collect(),
            lifecycle_flows_present: lifecycle_flows.into_iter().collect(),
            rollout_rings_present: rollout_rings_present.into_iter().collect(),
            failure_kinds_present: failure_kinds.into_iter().collect(),
            local_safe_lifecycle_row_count,
            local_safe_impact_preview_count,
            defect_count: defects.len(),
            overall_qualification_token: qualification.as_str().to_owned(),
            overall_narrow_reason_token: overall_reason.as_str().to_owned(),
        }
    }
}

/// Typed validation defect for the organization-admin truth page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationAdminTruthDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason.
    pub narrow_reason: OrganizationAdminTruthNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id for the defect.
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl OrganizationAdminTruthDefect {
    fn new(
        narrow_reason: OrganizationAdminTruthNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: ORGANIZATION_ADMIN_TRUTH_DEFECT_RECORD_KIND.to_owned(),
            schema_version: ORGANIZATION_ADMIN_TRUTH_SCHEMA_VERSION,
            shared_contract_ref: ORGANIZATION_ADMIN_TRUTH_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:organization-admin-truth:{}:{}",
                narrow_reason.as_str(),
                source
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source,
            note: note.into(),
        }
    }
}

/// Canonical organization-admin truth page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationAdminTruthPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC timestamp when this page was generated.
    pub generated_at: String,
    /// Organization or tenant overview card.
    pub overview: OrganizationOverviewCard,
    /// Directory/provider cards.
    pub provider_cards: Vec<DirectoryProviderCard>,
    /// User or seat lifecycle rows.
    pub seat_lifecycle_rows: Vec<SeatLifecycleRow>,
    /// Lifecycle impact previews.
    pub impact_previews: Vec<LifecycleImpactPreview>,
    /// Rollout-ring audit rows.
    pub rollout_rings: Vec<RolloutRingAuditRow>,
    /// Aggregate summary.
    pub summary: OrganizationAdminTruthSummary,
    /// Typed validation defects.
    pub defects: Vec<OrganizationAdminTruthDefect>,
}

impl OrganizationAdminTruthPage {
    /// Builds and audits an organization-admin truth page.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        overview: OrganizationOverviewCard,
        provider_cards: Vec<DirectoryProviderCard>,
        seat_lifecycle_rows: Vec<SeatLifecycleRow>,
        impact_previews: Vec<LifecycleImpactPreview>,
        rollout_rings: Vec<RolloutRingAuditRow>,
    ) -> Self {
        let defects = audit_organization_admin_truth_records(
            &overview,
            &provider_cards,
            &seat_lifecycle_rows,
            &impact_previews,
            &rollout_rings,
        );
        let summary = OrganizationAdminTruthSummary::from_records(
            &provider_cards,
            &seat_lifecycle_rows,
            &impact_previews,
            &rollout_rings,
            &defects,
        );
        Self {
            record_kind: ORGANIZATION_ADMIN_TRUTH_PAGE_RECORD_KIND.to_owned(),
            schema_version: ORGANIZATION_ADMIN_TRUTH_SCHEMA_VERSION,
            shared_contract_ref: ORGANIZATION_ADMIN_TRUTH_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            overview,
            provider_cards,
            seat_lifecycle_rows,
            impact_previews,
            rollout_rings,
            summary,
            defects,
        }
    }

    /// True when the page qualifies stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == OrganizationAdminTruthQualificationClass::Stable.as_str()
    }

    /// True when provider cards cover OIDC, SCIM, signed-file, and manual sources.
    pub fn covers_all_provisioning_classes(&self) -> bool {
        let observed: BTreeSet<&str> = self
            .provider_cards
            .iter()
            .map(|provider| provider.provisioning_class_token.as_str())
            .collect();
        OrganizationProvisioningClass::ALL
            .iter()
            .all(|class| observed.contains(class.as_str()))
    }

    /// True when impact previews cover transfer, suspension, downgrade, org switch, and deprovision.
    pub fn covers_all_lifecycle_flows(&self) -> bool {
        let observed: BTreeSet<&str> = self
            .impact_previews
            .iter()
            .map(|preview| preview.flow_class_token.as_str())
            .collect();
        LifecycleFlowClass::ALL
            .iter()
            .all(|flow| observed.contains(flow.as_str()))
    }

    /// True when every lifecycle and impact row preserves local safety guarantees.
    pub fn all_local_safety_guarantees_preserved(&self) -> bool {
        self.seat_lifecycle_rows.iter().all(|row| {
            row.local_editing_preserved
                && row.local_history_preserved
                && row.export_offboarding_available
                && !row.local_artifact_safety_note.is_empty()
        }) && self.impact_previews.iter().all(|preview| {
            preview.local_editing_preserved
                && preview.local_history_preserved
                && preview.unsaved_work_preserved
                && !preview.local_only_continuation_label.is_empty()
                && !preview.export_rights_label.is_empty()
        })
    }

    /// True when tenant, rollout, and provisioning source truth is visible everywhere required.
    pub fn boundary_truth_visible_everywhere(&self) -> bool {
        self.overview.surface_visibility.all_visible()
            && self
                .provider_cards
                .iter()
                .all(|row| row.surface_visibility.all_visible())
            && self
                .seat_lifecycle_rows
                .iter()
                .all(|row| row.surface_visibility.all_visible())
            && self
                .rollout_rings
                .iter()
                .all(|row| row.surface_visibility.all_visible())
    }
}

/// Support/export projection for organization-admin truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationAdminTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC timestamp when the export was generated.
    pub generated_at: String,
    /// Opaque organization and tenant reference.
    pub org_tenant_ref: String,
    /// Rollout ring refs carried into support/export packets.
    pub rollout_ring_refs: Vec<String>,
    /// Provisioning source tokens carried into support/export packets.
    pub provisioning_source_tokens: Vec<String>,
    /// Seat lifecycle row refs carried into support/export packets.
    pub seat_lifecycle_refs: Vec<String>,
    /// Local-safety guarantees carried into support/export packets.
    pub local_safety_guarantees: Vec<String>,
    /// Embedded canonical page.
    pub page: OrganizationAdminTruthPage,
    /// Narrow-reason tokens present in the page.
    pub narrow_reasons_present: Vec<OrganizationAdminTruthNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw secret, token, key, and private payload material is excluded.
    pub raw_private_material_excluded: bool,
}

impl OrganizationAdminTruthSupportExport {
    /// Wraps a page into a support/export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: OrganizationAdminTruthPage,
    ) -> Self {
        let mut reasons = Vec::new();
        let mut counts = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();

        let mut rollout_ring_refs: Vec<String> = page
            .rollout_rings
            .iter()
            .map(|ring| ring.ring_id.clone())
            .collect();
        rollout_ring_refs.sort();

        let mut provisioning_source_tokens: Vec<String> = page
            .provider_cards
            .iter()
            .map(|provider| provider.provisioning_class_token.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        provisioning_source_tokens.sort();

        let seat_lifecycle_refs = page
            .seat_lifecycle_rows
            .iter()
            .map(|row| row.row_id.clone())
            .collect();
        let local_safety_guarantees = page
            .seat_lifecycle_rows
            .iter()
            .map(|row| row.local_artifact_safety_note.clone())
            .collect();

        Self {
            record_kind: ORGANIZATION_ADMIN_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ORGANIZATION_ADMIN_TRUTH_SCHEMA_VERSION,
            shared_contract_ref: ORGANIZATION_ADMIN_TRUTH_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            org_tenant_ref: page.overview.org_tenant_ref.clone(),
            rollout_ring_refs,
            provisioning_source_tokens,
            seat_lifecycle_refs,
            local_safety_guarantees,
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

/// Re-runs the organization-admin truth audit over a page.
pub fn audit_organization_admin_truth_page(
    page: &OrganizationAdminTruthPage,
) -> Vec<OrganizationAdminTruthDefect> {
    audit_organization_admin_truth_records(
        &page.overview,
        &page.provider_cards,
        &page.seat_lifecycle_rows,
        &page.impact_previews,
        &page.rollout_rings,
    )
}

/// Validates the organization-admin truth page and returns defects on failure.
pub fn validate_organization_admin_truth_page(
    page: &OrganizationAdminTruthPage,
) -> Result<(), Vec<OrganizationAdminTruthDefect>> {
    let defects = audit_organization_admin_truth_page(page);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Builds the seeded stable organization-admin truth page.
pub fn seeded_organization_admin_truth_page() -> OrganizationAdminTruthPage {
    OrganizationAdminTruthPage::new(
        "policy:organization-admin-truth:seeded:0001",
        "Organization admin, provisioning, seat lifecycle, rollout ring, and local-safety truth",
        "2026-06-01T00:00:00Z",
        seeded_overview(),
        seeded_provider_cards(),
        seeded_seat_lifecycle_rows(),
        seeded_impact_previews(),
        seeded_rollout_rings(),
    )
}

fn audit_organization_admin_truth_records(
    overview: &OrganizationOverviewCard,
    provider_cards: &[DirectoryProviderCard],
    seat_lifecycle_rows: &[SeatLifecycleRow],
    impact_previews: &[LifecycleImpactPreview],
    rollout_rings: &[RolloutRingAuditRow],
) -> Vec<OrganizationAdminTruthDefect> {
    let mut defects = Vec::new();

    if !overview.raw_private_material_excluded
        || provider_cards
            .iter()
            .any(|row| !row.raw_private_material_excluded)
        || seat_lifecycle_rows
            .iter()
            .any(|row| !row.raw_private_material_excluded)
        || impact_previews
            .iter()
            .any(|row| !row.raw_private_material_excluded)
        || rollout_rings
            .iter()
            .any(|row| !row.raw_private_material_excluded)
    {
        defects.push(OrganizationAdminTruthDefect::new(
            OrganizationAdminTruthNarrowReasonClass::RawPrivateMaterialExposed,
            "page",
            "raw secret, token, key, or private payload material crossed the organization-admin truth boundary",
        ));
    }

    for row in seat_lifecycle_rows {
        if !row.local_editing_preserved
            || !row.local_history_preserved
            || !row.export_offboarding_available
            || row.local_artifact_safety_note.is_empty()
        {
            defects.push(OrganizationAdminTruthDefect::new(
                OrganizationAdminTruthNarrowReasonClass::LocalSafetyGuaranteeMissing,
                row.row_id.clone(),
                "seat lifecycle row does not preserve local editing, local history, export/offboarding rights, or an explicit local-artifact safety note",
            ));
        }
    }

    for preview in impact_previews {
        if !preview.local_editing_preserved
            || !preview.local_history_preserved
            || !preview.unsaved_work_preserved
            || preview.local_only_continuation_label.is_empty()
            || preview.export_rights_label.is_empty()
        {
            defects.push(OrganizationAdminTruthDefect::new(
                OrganizationAdminTruthNarrowReasonClass::LocalSafetyGuaranteeMissing,
                preview.preview_id.clone(),
                "impact preview does not preserve local editing, local history, unsaved work, local-only continuation, or export rights",
            ));
        }
    }

    if !overview.surface_visibility.all_visible()
        || provider_cards
            .iter()
            .any(|row| !row.surface_visibility.all_visible())
        || seat_lifecycle_rows
            .iter()
            .any(|row| !row.surface_visibility.all_visible())
        || rollout_rings
            .iter()
            .any(|row| !row.surface_visibility.all_visible())
    {
        defects.push(OrganizationAdminTruthDefect::new(
            OrganizationAdminTruthNarrowReasonClass::BoundaryVisibilityIncomplete,
            "page",
            "tenant, provisioning source, rollout ring, or local-safety truth is not visible in admin, diagnostics, About, Help, support, and export surfaces",
        ));
    }

    if overview.org_tenant_ref.is_empty()
        || overview.policy_source_label.is_empty()
        || overview.rollout_ring_summary.is_empty()
        || overview.last_successful_sync.is_empty()
        || overview.export_action_label.is_empty()
        || overview.support_action_label.is_empty()
    {
        defects.push(OrganizationAdminTruthDefect::new(
            OrganizationAdminTruthNarrowReasonClass::OverviewTruthMissing,
            "overview",
            "organization overview is missing tenant identity, policy source, rollout summary, last sync, export action, or support action",
        ));
    }

    let observed_provisioning: BTreeSet<&str> = provider_cards
        .iter()
        .map(|provider| provider.provisioning_class_token.as_str())
        .collect();
    for required in OrganizationProvisioningClass::ALL {
        if !observed_provisioning.contains(required.as_str()) {
            defects.push(OrganizationAdminTruthDefect::new(
                OrganizationAdminTruthNarrowReasonClass::MissingProvisioningClassCoverage,
                "provider_cards",
                format!(
                    "missing provider card for provisioning class '{}'",
                    required.as_str()
                ),
            ));
        }
    }

    for provider in provider_cards {
        if provider.org_tenant_ref.is_empty()
            || provider.rollout_ring_ref.is_empty()
            || provider.provisioning_class_token.is_empty()
            || provider.provider_state_token.is_empty()
            || provider.sync_freshness_token.is_empty()
            || provider.last_successful_sync.is_empty()
            || provider.scope_summary_label.is_empty()
            || provider.fallback_manual_path_label.is_empty()
        {
            defects.push(OrganizationAdminTruthDefect::new(
                OrganizationAdminTruthNarrowReasonClass::ProviderCardIncomplete,
                provider.provider_id.clone(),
                "directory/provider card is missing provisioning class, provider state, freshness, scope, fallback, tenant, or rollout truth",
            ));
        }
        if provider.provider_state.requires_failure_kind() && provider.failure_kind_token.is_empty()
        {
            defects.push(OrganizationAdminTruthDefect::new(
                OrganizationAdminTruthNarrowReasonClass::FailureKindNotSpecific,
                provider.provider_id.clone(),
                "degraded or blocked provider row does not name a specific failure kind",
            ));
        }
        if !provider.failure_kind_token.is_empty()
            && !AdminFailureKind::is_known_token(&provider.failure_kind_token)
        {
            defects.push(OrganizationAdminTruthDefect::new(
                OrganizationAdminTruthNarrowReasonClass::FailureKindNotSpecific,
                provider.provider_id.clone(),
                "provider row uses an unknown or generic failure kind token",
            ));
        }
    }

    for row in seat_lifecycle_rows {
        if row.org_tenant_ref.is_empty()
            || row.rollout_ring_ref.is_empty()
            || row.principal_ref.is_empty()
            || row.source_of_truth_label.is_empty()
            || row.role_label.is_empty()
            || row.seat_class_token.is_empty()
            || row.lifecycle_state_token.is_empty()
            || row.last_seen.is_empty()
            || row.action_lineage.action_token.is_empty()
            || row.action_lineage.result_token.is_empty()
            || row.action_lineage.actor_ref.is_empty()
            || row.action_lineage.transaction_id.is_empty()
        {
            defects.push(OrganizationAdminTruthDefect::new(
                OrganizationAdminTruthNarrowReasonClass::SeatLifecycleTruthMissing,
                row.row_id.clone(),
                "seat lifecycle row is missing principal, source of truth, role, seat class, state, last seen, local safety, or admin action lineage",
            ));
        }
    }

    let observed_flows: BTreeSet<&str> = impact_previews
        .iter()
        .map(|preview| preview.flow_class_token.as_str())
        .collect();
    for required in LifecycleFlowClass::ALL {
        if !observed_flows.contains(required.as_str()) {
            defects.push(OrganizationAdminTruthDefect::new(
                OrganizationAdminTruthNarrowReasonClass::MissingImpactPreviewCoverage,
                "impact_previews",
                format!(
                    "missing lifecycle impact preview for '{}'",
                    required.as_str()
                ),
            ));
        }
    }

    for preview in impact_previews {
        if preview.preview_id.is_empty()
            || preview.flow_class_token.is_empty()
            || preview.principal_ref.is_empty()
            || preview.managed_ai_impact_token.is_empty()
            || preview.sync_impact_token.is_empty()
            || preview.collaboration_impact_token.is_empty()
            || preview.review_impact_token.is_empty()
            || preview.marketplace_impact_token.is_empty()
            || preview.action_lineage.action_token.is_empty()
            || preview.action_lineage.result_token.is_empty()
        {
            defects.push(OrganizationAdminTruthDefect::new(
                OrganizationAdminTruthNarrowReasonClass::ImpactPreviewTruthMissing,
                preview.preview_id.clone(),
                "lifecycle impact preview is missing flow, entitlement impact, local continuation, export rights, or action lineage truth",
            ));
        }
        if !preview.failure_kind_token.is_empty()
            && !AdminFailureKind::is_known_token(&preview.failure_kind_token)
        {
            defects.push(OrganizationAdminTruthDefect::new(
                OrganizationAdminTruthNarrowReasonClass::FailureKindNotSpecific,
                preview.preview_id.clone(),
                "impact preview uses an unknown or generic failure kind token",
            ));
        }
    }

    if rollout_rings.is_empty() {
        defects.push(OrganizationAdminTruthDefect::new(
            OrganizationAdminTruthNarrowReasonClass::RolloutRingTruthMissing,
            "rollout_rings",
            "no rollout-ring audit rows are present",
        ));
    }
    for ring in rollout_rings {
        if ring.ring_id.is_empty()
            || ring.org_tenant_ref.is_empty()
            || ring.ring_class_token.is_empty()
            || ring.ring_state_token.is_empty()
            || ring.policy_source_label.is_empty()
            || ring.build_channel_summary.is_empty()
            || ring.last_audited.is_empty()
            || ring.local_safety_note.is_empty()
            || ring.rollback_action_label.is_empty()
        {
            defects.push(OrganizationAdminTruthDefect::new(
                OrganizationAdminTruthNarrowReasonClass::RolloutRingTruthMissing,
                ring.ring_id.clone(),
                "rollout-ring audit row is missing ring, policy source, build/channel, audit freshness, local safety, or rollback truth",
            ));
        }
    }

    defects
}

fn seeded_overview() -> OrganizationOverviewCard {
    OrganizationOverviewCard {
        record_kind: ORGANIZATION_OVERVIEW_CARD_RECORD_KIND.to_owned(),
        schema_version: ORGANIZATION_ADMIN_TRUTH_SCHEMA_VERSION,
        shared_contract_ref: ORGANIZATION_ADMIN_TRUTH_SHARED_CONTRACT_REF.to_owned(),
        org_tenant_ref: "org:acme-platform/tenant:sovereign-eu".to_owned(),
        deployment_mode: OrganizationDeploymentMode::SelfHostedControlPlane,
        deployment_mode_token: OrganizationDeploymentMode::SelfHostedControlPlane
            .as_str()
            .to_owned(),
        policy_source_label: "signed policy bundle 2026-06-01T00:00:00Z".to_owned(),
        seat_summary: SeatSummary {
            active: 412,
            pending: 9,
            suspended: 3,
            grace_window: 2,
            deprovisioned: 6,
        },
        rollout_ring_summary: "stable ring 82 percent current; pilot ring 18 percent pending"
            .to_owned(),
        last_successful_sync: "2026-06-01T00:07:00Z".to_owned(),
        export_action_label: "Export evaluation packet".to_owned(),
        support_action_label: "Generate support packet".to_owned(),
        surface_visibility: SurfaceVisibility::all_required(),
        raw_private_material_excluded: true,
    }
}

fn seeded_provider_cards() -> Vec<DirectoryProviderCard> {
    vec![
        provider_card(
            "provider:oidc:primary",
            OrganizationProvisioningClass::Oidc,
            ProviderStateClass::SyncHealthy,
            AdminSyncFreshnessClass::Live,
            "OIDC issuer healthy; 412 managed users in org tenant",
            412,
            0,
            "Manual system-browser sign-in and signed recovery bundle available",
            None,
        ),
        provider_card(
            "provider:scim:lifecycle",
            OrganizationProvisioningClass::Scim,
            ProviderStateClass::Degraded,
            AdminSyncFreshnessClass::Stale,
            "18 groups / 409 managed users resolved from cached SCIM state",
            409,
            18,
            "Last-known-good signed file import available while SCIM endpoint recovers",
            Some(AdminFailureKind::ProviderOutage),
        ),
        provider_card(
            "provider:signed-file:air-gap",
            OrganizationProvisioningClass::SignedFileBundle,
            ProviderStateClass::Cached,
            AdminSyncFreshnessClass::Cached,
            "signed bundle covers 18 groups / 412 seats",
            412,
            18,
            "Manual import from signed bundle remains available without vendor console",
            None,
        ),
        provider_card(
            "provider:manual:break-glass",
            OrganizationProvisioningClass::Manual,
            ProviderStateClass::ManualOnly,
            AdminSyncFreshnessClass::Live,
            "manual break-glass path scoped to 3 administrators",
            3,
            1,
            "Admin-signed request file can create, suspend, transfer, or deprovision seats",
            None,
        ),
    ]
}

fn provider_card(
    provider_id: &str,
    provisioning_class: OrganizationProvisioningClass,
    provider_state: ProviderStateClass,
    sync_freshness: AdminSyncFreshnessClass,
    scope_summary_label: &str,
    managed_user_count: usize,
    managed_group_count: usize,
    fallback_manual_path_label: &str,
    failure_kind: Option<AdminFailureKind>,
) -> DirectoryProviderCard {
    DirectoryProviderCard {
        record_kind: DIRECTORY_PROVIDER_CARD_RECORD_KIND.to_owned(),
        schema_version: ORGANIZATION_ADMIN_TRUTH_SCHEMA_VERSION,
        shared_contract_ref: ORGANIZATION_ADMIN_TRUTH_SHARED_CONTRACT_REF.to_owned(),
        provider_id: provider_id.to_owned(),
        org_tenant_ref: "org:acme-platform/tenant:sovereign-eu".to_owned(),
        rollout_ring_ref: "ring:stable".to_owned(),
        provisioning_class,
        provisioning_class_token: provisioning_class.as_str().to_owned(),
        provider_state,
        provider_state_token: provider_state.as_str().to_owned(),
        sync_freshness,
        sync_freshness_token: sync_freshness.as_str().to_owned(),
        last_successful_sync: "2026-06-01T00:07:00Z".to_owned(),
        scope_summary_label: scope_summary_label.to_owned(),
        managed_user_count,
        managed_group_count,
        fallback_manual_path_label: fallback_manual_path_label.to_owned(),
        failure_kind,
        failure_kind_token: failure_kind
            .map(|kind| kind.as_str().to_owned())
            .unwrap_or_default(),
        surface_visibility: SurfaceVisibility::all_required(),
        raw_private_material_excluded: true,
    }
}

fn seeded_seat_lifecycle_rows() -> Vec<SeatLifecycleRow> {
    vec![
        seat_row(
            "seat:principal-active:0001",
            "principal:managed:0001",
            "OIDC identity plus SCIM lifecycle source",
            "Maintainer",
            SeatClass::EnterpriseManaged,
            SeatLifecycleStateClass::Active,
            "2026-06-01T00:06:00Z",
            "Active managed capabilities are tenant-scoped; local projects, local history, unsaved work, and export rights remain on the device.",
            lineage(
                "provision",
                "Provision managed seat",
                "succeeded",
                "Seat active from SCIM lifecycle source",
                "admin:directory:001",
                "2026-06-01T00:02:00Z",
                "txn:seat:provision:0001",
            ),
            vec!["preview:seat_transfer".to_owned(), "preview:suspension".to_owned()],
        ),
        seat_row(
            "seat:principal-suspended:0002",
            "principal:managed:0002",
            "SCIM lifecycle source",
            "Reviewer",
            SeatClass::Reviewer,
            SeatLifecycleStateClass::Suspended,
            "2026-05-31T21:16:00Z",
            "Suspension stops managed AI, sync, collaboration, review, and marketplace writes; local editing, local history, and export/offboarding remain available.",
            lineage(
                "suspend",
                "Suspend managed seat",
                "succeeded",
                "Managed entitlements suspended; local safety guarantees preserved",
                "admin:directory:001",
                "2026-05-31T21:18:00Z",
                "txn:seat:suspend:0002",
            ),
            vec!["preview:suspension".to_owned(), "preview:deprovision".to_owned()],
        ),
        seat_row(
            "seat:principal-grace:0003",
            "principal:managed:0003",
            "signed file lifecycle source",
            "Contributor",
            SeatClass::LocalOnlyContinuation,
            SeatLifecycleStateClass::GraceWindow,
            "2026-05-31T20:01:00Z",
            "Grace-window seat keeps local-only continuation, local history replay, and export rights explicit while managed entitlements wait for admin review.",
            lineage(
                "downgrade",
                "Downgrade to local-only continuation during seat loss",
                "narrowed",
                "Managed entitlements narrowed; local editing and export remain available",
                "admin:directory:001",
                "2026-05-31T20:05:00Z",
                "txn:seat:downgrade:0003",
            ),
            vec!["preview:downgrade".to_owned(), "preview:org_switch".to_owned()],
        ),
        seat_row(
            "seat:principal-deprovisioned:0004",
            "principal:managed:0004",
            "SCIM deprovision source",
            "Former contributor",
            SeatClass::LocalOnlyContinuation,
            SeatLifecycleStateClass::Deprovisioned,
            "2026-05-30T12:00:00Z",
            "Deprovision removed managed access only; local clones, local history, unsaved recovery, and export/offboarding packet access remain available.",
            lineage(
                "deprovision",
                "Deprovision managed account",
                "succeeded",
                "Managed access removed without deleting local artifacts",
                "admin:directory:001",
                "2026-05-30T12:04:00Z",
                "txn:seat:deprovision:0004",
            ),
            vec!["preview:deprovision".to_owned()],
        ),
    ]
}

fn seat_row(
    row_id: &str,
    principal_ref: &str,
    source_of_truth_label: &str,
    role_label: &str,
    seat_class: SeatClass,
    lifecycle_state: SeatLifecycleStateClass,
    last_seen: &str,
    local_artifact_safety_note: &str,
    action_lineage: AdminLifecycleActionLineage,
    impact_preview_refs: Vec<String>,
) -> SeatLifecycleRow {
    SeatLifecycleRow {
        record_kind: SEAT_LIFECYCLE_ROW_RECORD_KIND.to_owned(),
        schema_version: ORGANIZATION_ADMIN_TRUTH_SCHEMA_VERSION,
        shared_contract_ref: ORGANIZATION_ADMIN_TRUTH_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        org_tenant_ref: "org:acme-platform/tenant:sovereign-eu".to_owned(),
        rollout_ring_ref: "ring:stable".to_owned(),
        principal_ref: principal_ref.to_owned(),
        source_of_truth_label: source_of_truth_label.to_owned(),
        role_label: role_label.to_owned(),
        seat_class,
        seat_class_token: seat_class.as_str().to_owned(),
        lifecycle_state,
        lifecycle_state_token: lifecycle_state.as_str().to_owned(),
        last_seen: last_seen.to_owned(),
        local_artifact_safety_note: local_artifact_safety_note.to_owned(),
        local_editing_preserved: true,
        local_history_preserved: true,
        export_offboarding_available: true,
        action_lineage,
        impact_preview_refs,
        surface_visibility: SurfaceVisibility::all_required(),
        raw_private_material_excluded: true,
    }
}

fn seeded_impact_previews() -> Vec<LifecycleImpactPreview> {
    vec![
        impact_preview(
            LifecycleFlowClass::SeatTransfer,
            EntitlementImpactClass::Narrowed,
            EntitlementImpactClass::RequiresReview,
            EntitlementImpactClass::RequiresReview,
            EntitlementImpactClass::Unchanged,
            EntitlementImpactClass::RequiresReview,
            None,
        ),
        impact_preview(
            LifecycleFlowClass::Suspension,
            EntitlementImpactClass::Disabled,
            EntitlementImpactClass::Disabled,
            EntitlementImpactClass::Disabled,
            EntitlementImpactClass::Narrowed,
            EntitlementImpactClass::Disabled,
            Some(AdminFailureKind::SeatLoss),
        ),
        impact_preview(
            LifecycleFlowClass::Downgrade,
            EntitlementImpactClass::Disabled,
            EntitlementImpactClass::Narrowed,
            EntitlementImpactClass::Disabled,
            EntitlementImpactClass::LocalOnlyAvailable,
            EntitlementImpactClass::Narrowed,
            Some(AdminFailureKind::SeatLoss),
        ),
        impact_preview(
            LifecycleFlowClass::OrgSwitch,
            EntitlementImpactClass::RequiresReview,
            EntitlementImpactClass::RequiresReview,
            EntitlementImpactClass::RequiresReview,
            EntitlementImpactClass::RequiresReview,
            EntitlementImpactClass::RequiresReview,
            Some(AdminFailureKind::ScopeMismatch),
        ),
        impact_preview(
            LifecycleFlowClass::Deprovision,
            EntitlementImpactClass::Disabled,
            EntitlementImpactClass::Disabled,
            EntitlementImpactClass::Disabled,
            EntitlementImpactClass::Disabled,
            EntitlementImpactClass::Disabled,
            Some(AdminFailureKind::DeprovisioningImpact),
        ),
    ]
}

fn impact_preview(
    flow_class: LifecycleFlowClass,
    managed_ai_impact: EntitlementImpactClass,
    sync_impact: EntitlementImpactClass,
    collaboration_impact: EntitlementImpactClass,
    review_impact: EntitlementImpactClass,
    marketplace_impact: EntitlementImpactClass,
    failure_kind: Option<AdminFailureKind>,
) -> LifecycleImpactPreview {
    let flow_token = flow_class.as_str();
    LifecycleImpactPreview {
        record_kind: LIFECYCLE_IMPACT_PREVIEW_RECORD_KIND.to_owned(),
        schema_version: ORGANIZATION_ADMIN_TRUTH_SCHEMA_VERSION,
        shared_contract_ref: ORGANIZATION_ADMIN_TRUTH_SHARED_CONTRACT_REF.to_owned(),
        preview_id: format!("preview:{flow_token}"),
        flow_class,
        flow_class_token: flow_token.to_owned(),
        principal_ref: "principal:managed:preview".to_owned(),
        managed_ai_impact,
        managed_ai_impact_token: managed_ai_impact.as_str().to_owned(),
        sync_impact,
        sync_impact_token: sync_impact.as_str().to_owned(),
        collaboration_impact,
        collaboration_impact_token: collaboration_impact.as_str().to_owned(),
        review_impact,
        review_impact_token: review_impact.as_str().to_owned(),
        marketplace_impact,
        marketplace_impact_token: marketplace_impact.as_str().to_owned(),
        local_only_continuation_label:
            "Local editing, local history, and local search continue without managed services"
                .to_owned(),
        export_rights_label:
            "Export and offboarding packet access remain available without support intervention"
                .to_owned(),
        local_editing_preserved: true,
        local_history_preserved: true,
        unsaved_work_preserved: true,
        failure_kind,
        failure_kind_token: failure_kind
            .map(|kind| kind.as_str().to_owned())
            .unwrap_or_default(),
        action_lineage: lineage(
            flow_token,
            &format!("Preview {flow_token} impact"),
            "previewed",
            "Managed entitlement impact previewed; local safety guarantees explicit",
            "admin:directory:001",
            "2026-06-01T00:08:00Z",
            &format!("txn:preview:{flow_token}:0001"),
        ),
        raw_private_material_excluded: true,
    }
}

fn seeded_rollout_rings() -> Vec<RolloutRingAuditRow> {
    vec![
        rollout_ring(
            "ring:stable",
            RolloutRingClass::Stable,
            RolloutRingStateClass::Current,
            "stable channel 2.1.0 signed; 82 percent current",
            412,
            0,
        ),
        rollout_ring(
            "ring:pilot",
            RolloutRingClass::Pilot,
            RolloutRingStateClass::Pending,
            "pilot channel 2.1.1 signed; 18 percent pending",
            91,
            2,
        ),
    ]
}

fn rollout_ring(
    ring_id: &str,
    ring_class: RolloutRingClass,
    ring_state: RolloutRingStateClass,
    build_channel_summary: &str,
    enrolled_device_count: usize,
    drifted_device_count: usize,
) -> RolloutRingAuditRow {
    RolloutRingAuditRow {
        record_kind: ROLLOUT_RING_AUDIT_ROW_RECORD_KIND.to_owned(),
        schema_version: ORGANIZATION_ADMIN_TRUTH_SCHEMA_VERSION,
        shared_contract_ref: ORGANIZATION_ADMIN_TRUTH_SHARED_CONTRACT_REF.to_owned(),
        ring_id: ring_id.to_owned(),
        org_tenant_ref: "org:acme-platform/tenant:sovereign-eu".to_owned(),
        ring_class,
        ring_class_token: ring_class.as_str().to_owned(),
        ring_state,
        ring_state_token: ring_state.as_str().to_owned(),
        policy_source_label: "signed policy bundle 2026-06-01T00:00:00Z".to_owned(),
        build_channel_summary: build_channel_summary.to_owned(),
        enrolled_device_count,
        drifted_device_count,
        last_audited: "2026-06-01T00:09:00Z".to_owned(),
        local_safety_note: "Ring changes may narrow managed services; local editing, local history, and export remain available.".to_owned(),
        rollback_action_label: "Rollback eligible with signed bundle checkpoint".to_owned(),
        surface_visibility: SurfaceVisibility::all_required(),
        raw_private_material_excluded: true,
    }
}

fn lineage(
    action_token: &str,
    action_label: &str,
    result_token: &str,
    result_label: &str,
    actor_ref: &str,
    applied_at: &str,
    transaction_id: &str,
) -> AdminLifecycleActionLineage {
    AdminLifecycleActionLineage {
        action_token: action_token.to_owned(),
        action_label: action_label.to_owned(),
        result_token: result_token.to_owned(),
        result_label: result_label.to_owned(),
        actor_ref: actor_ref.to_owned(),
        applied_at: applied_at.to_owned(),
        transaction_id: transaction_id.to_owned(),
    }
}
