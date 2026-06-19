//! Plain-language locality descriptors and tenant-boundary cards for claimed
//! managed, self-hosted, and sovereign surfaces.
//!
//! This module is the in-product surfacing lane that sits on top of the frozen
//! continuity-claim matrix
//! ([`crate::m5_locality_tenant_keymode_and_drill_matrix`]). The matrix is the
//! truth source for where processing and storage happen, which tenant boundary
//! and key mode apply, and which continuity lane a surface sits on. This lane
//! turns each claimed managed row into two things a person can read directly in
//! the product and in exportable evidence:
//!
//! 1. A [`LocalityDescriptor`] — plain-language processing location, storage
//!    location, explicit region pinning with a honored/cannot-honor state, and
//!    the retention/export class in force.
//! 2. A [`TenantBoundaryCard`] — plain-language tenant/org scope, isolation
//!    posture, and key mode.
//!
//! The same descriptor and card are projected onto every claimed surface
//! (desktop, CLI/headless inspect, service-health, support export, About/Help,
//! and docs/public-truth) through a [`LocalitySurfaceProjection`], so the exact
//! locality and tenant vocabulary stays byte-identical everywhere instead of
//! drifting per surface.
//!
//! Two guardrails are load-bearing:
//!
//! - A region-pinned or tenant-scoped row on the protected managed lane **fails
//!   closed** when the declared region pin cannot be honored: its claim is
//!   withdrawn rather than silently downgraded. Local-core desktop work is never
//!   pulled into this rule and stays accurately labeled.
//! - A self-hosted or sovereign row may not imply stronger locality than the
//!   running topology provides; claiming a broad vendor region under a
//!   self-governed banner withdraws the claim.
//!
//! The packet is metadata-only. It carries closed-vocabulary tokens, export-safe
//! plain-language labels, and opaque refs. Raw hostnames, raw tenant
//! identifiers, raw KMS handles, and secret material never cross this boundary.

use serde::{Deserialize, Serialize};

use crate::m5_locality_tenant_keymode_and_drill_matrix::{
    seeded_continuity_claim_matrix_input, ContinuityClaimQualificationClass, ContinuityClaimRow,
    ContinuityLaneClass, ContinuityProfileClass, KeyModeClass, LocalityClass, TenantScopeClass,
};

#[cfg(test)]
mod tests;

/// Schema version carried on every record in this module.
pub const LOCALITY_TENANT_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const LOCALITY_TENANT_SHARED_CONTRACT_REF: &str =
    "continuity:m5_locality_descriptors_and_tenant_cards:v1";

/// Record-kind tag for [`LocalityTenantCardPage`] payloads.
pub const LOCALITY_TENANT_PAGE_RECORD_KIND: &str = "locality_tenant_card_page_record";

/// Record-kind tag for [`LocalityTenantSummary`] payloads.
pub const LOCALITY_TENANT_SUMMARY_RECORD_KIND: &str = "locality_tenant_summary_record";

/// Record-kind tag for [`LocalityDescriptor`] payloads.
pub const LOCALITY_DESCRIPTOR_RECORD_KIND: &str = "locality_descriptor_record";

/// Record-kind tag for [`TenantBoundaryCard`] payloads.
pub const TENANT_BOUNDARY_CARD_RECORD_KIND: &str = "tenant_boundary_card_record";

/// Record-kind tag for [`LocalitySurfaceProjection`] payloads.
pub const LOCALITY_SURFACE_PROJECTION_RECORD_KIND: &str = "locality_surface_projection_record";

/// Record-kind tag for [`LocalityTenantRowOutcome`] payloads.
pub const LOCALITY_TENANT_ROW_OUTCOME_RECORD_KIND: &str = "locality_tenant_row_outcome_record";

/// Record-kind tag for [`LocalityTenantDefect`] payloads.
pub const LOCALITY_TENANT_DEFECT_RECORD_KIND: &str = "locality_tenant_defect_record";

/// Record-kind tag for [`LocalityTenantSupportExport`] payloads.
pub const LOCALITY_TENANT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "locality_tenant_support_export_record";

/// Repo-relative path of the canonical reviewer doc for this lane.
pub const LOCALITY_TENANT_DOC_REF: &str =
    "docs/m5/continuity/locality-and-tenant-boundary-surfaces.md";

/// Repo-relative path of the checked-in artifact for this lane.
pub const LOCALITY_TENANT_ARTIFACT_REF: &str =
    "artifacts/m5/continuity/locality_and_tenant_boundary_surfaces.md";

/// Repo-relative path of the canonical JSON schema for this lane.
pub const LOCALITY_TENANT_SCHEMA_REF: &str = "schemas/continuity/locality_descriptor.schema.json";

/// Surface a locality descriptor and tenant card are projected onto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalitySurfaceClass {
    /// The desktop product UI.
    Desktop,
    /// The CLI / headless inspect surface.
    CliHeadless,
    /// The service-health surface.
    ServiceHealth,
    /// A support export packet.
    SupportExport,
    /// The About / Help surfaces.
    AboutHelp,
    /// Docs and public-truth pages.
    DocsPublicTruth,
}

impl LocalitySurfaceClass {
    /// Every surface in canonical projection order.
    pub const ALL: [LocalitySurfaceClass; 6] = [
        Self::Desktop,
        Self::CliHeadless,
        Self::ServiceHealth,
        Self::SupportExport,
        Self::AboutHelp,
        Self::DocsPublicTruth,
    ];

    /// Surfaces a local-core row must still reach (support export is optional).
    pub const LOCAL_CORE: [LocalitySurfaceClass; 5] = [
        Self::Desktop,
        Self::CliHeadless,
        Self::ServiceHealth,
        Self::AboutHelp,
        Self::DocsPublicTruth,
    ];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::CliHeadless => "cli_headless",
            Self::ServiceHealth => "service_health",
            Self::SupportExport => "support_export",
            Self::AboutHelp => "about_help",
            Self::DocsPublicTruth => "docs_public_truth",
        }
    }
}

/// Declared region-pin posture for a claimed row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionPinClass {
    /// No region pin is declared.
    Unpinned,
    /// Pinned to a single named region.
    SingleRegionPinned,
    /// Pinned to a named set of regions.
    MultiRegionPinned,
    /// Pinned inside an in-country sovereign boundary.
    InCountryPinned,
    /// Pinned to a customer-operated region.
    CustomerRegionPinned,
    /// Region pinning does not apply to this row.
    PinNotApplicable,
}

impl RegionPinClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unpinned => "unpinned",
            Self::SingleRegionPinned => "single_region_pinned",
            Self::MultiRegionPinned => "multi_region_pinned",
            Self::InCountryPinned => "in_country_pinned",
            Self::CustomerRegionPinned => "customer_region_pinned",
            Self::PinNotApplicable => "pin_not_applicable",
        }
    }

    /// Plain-language summary of the region pin.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::Unpinned => "no region pin",
            Self::SingleRegionPinned => "pinned to a single region",
            Self::MultiRegionPinned => "pinned to a region set",
            Self::InCountryPinned => "pinned in-country",
            Self::CustomerRegionPinned => "pinned to a customer region",
            Self::PinNotApplicable => "not applicable",
        }
    }

    /// True when the class names an explicit region pin.
    pub const fn is_pinned(self) -> bool {
        matches!(
            self,
            Self::SingleRegionPinned
                | Self::MultiRegionPinned
                | Self::InCountryPinned
                | Self::CustomerRegionPinned
        )
    }
}

/// Whether a declared region pin is currently honored by the running topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionPinHonorState {
    /// The running topology honors the declared pin.
    Honored,
    /// The pin has temporarily drifted inside an approved fallback window.
    DriftedWithinGrace,
    /// The declared boundary cannot be honored; the managed lane fails closed.
    CannotHonor,
    /// Honoring a pin does not apply to this row.
    NotApplicable,
}

impl RegionPinHonorState {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Honored => "honored",
            Self::DriftedWithinGrace => "drifted_within_grace",
            Self::CannotHonor => "cannot_honor",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Plain-language summary of the honor state.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::Honored => "honored",
            Self::DriftedWithinGrace => "temporarily drifted within grace",
            Self::CannotHonor => "cannot be honored",
            Self::NotApplicable => "not applicable",
        }
    }
}

/// Retention / export class in force for a claimed row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// Durable state is retained on the local device.
    DeviceLocalRetention,
    /// Retention follows a customer-configured window.
    CustomerConfiguredRetention,
    /// Retention follows a vendor default window.
    VendorDefaultRetention,
    /// The row is under legal hold; deletion is suspended.
    LegalHoldRetention,
    /// No durable retention; data is processing-only.
    EphemeralNoRetention,
    /// Retention is not disclosed; the claim must narrow.
    RetentionUndisclosed,
}

impl RetentionClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeviceLocalRetention => "device_local_retention",
            Self::CustomerConfiguredRetention => "customer_configured_retention",
            Self::VendorDefaultRetention => "vendor_default_retention",
            Self::LegalHoldRetention => "legal_hold_retention",
            Self::EphemeralNoRetention => "ephemeral_no_retention",
            Self::RetentionUndisclosed => "retention_undisclosed",
        }
    }

    /// Plain-language summary of the retention class.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::DeviceLocalRetention => "retained on this device",
            Self::CustomerConfiguredRetention => "customer-configured retention",
            Self::VendorDefaultRetention => "vendor default retention",
            Self::LegalHoldRetention => "under legal hold",
            Self::EphemeralNoRetention => "no durable retention",
            Self::RetentionUndisclosed => "not disclosed",
        }
    }
}

/// Isolation posture backing a tenant boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TenantIsolationClass {
    /// A single local user with no shared tenancy.
    ProcessLocalIsolation,
    /// Dedicated single-customer infrastructure.
    DedicatedInfrastructure,
    /// Logically isolated shared tenancy.
    LogicalMultiTenant,
    /// Isolation lives inside the customer's own boundary.
    CustomerBoundary,
    /// Isolation has not been verified; the claim must narrow.
    IsolationUnverified,
    /// Tenant isolation does not apply to this row.
    NotApplicable,
}

impl TenantIsolationClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProcessLocalIsolation => "process_local_isolation",
            Self::DedicatedInfrastructure => "dedicated_infrastructure",
            Self::LogicalMultiTenant => "logical_multi_tenant",
            Self::CustomerBoundary => "customer_boundary",
            Self::IsolationUnverified => "isolation_unverified",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Plain-language summary of the isolation posture.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::ProcessLocalIsolation => "single local user, no shared tenancy",
            Self::DedicatedInfrastructure => "dedicated single-customer infrastructure",
            Self::LogicalMultiTenant => "logically isolated shared tenancy",
            Self::CustomerBoundary => "inside the customer's own boundary",
            Self::IsolationUnverified => "isolation not yet verified",
            Self::NotApplicable => "not applicable",
        }
    }
}

/// Typed reason a locality/tenant claim narrowed below stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalityTenantNarrowReasonClass {
    /// No narrowing is active.
    NotNarrowed,
    /// Processing location is not disclosed.
    ProcessingLocationUndisclosed,
    /// Storage location is not disclosed.
    StorageLocationUndisclosed,
    /// A managed-scope row does not declare a region pin.
    RegionPinUndeclaredOnManaged,
    /// A managed-lane region pin cannot be honored; the row fails closed.
    RegionPinUnhonored,
    /// The retention / export class is not disclosed.
    RetentionClassUndisclosed,
    /// A managed-scope row does not disclose its tenant scope.
    TenantScopeUndisclosed,
    /// The tenant boundary isolation has not been verified.
    TenantBoundaryUnverified,
    /// A surface renders different locality/tenant vocabulary than the descriptor.
    LocalityVocabularyDrift,
    /// A self-hosted or sovereign row implies stronger locality than its topology.
    SelfHostedLocalityOverclaimed,
    /// A row is not projected onto every required surface.
    SurfaceProjectionIncomplete,
}

impl LocalityTenantNarrowReasonClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::ProcessingLocationUndisclosed => "processing_location_undisclosed",
            Self::StorageLocationUndisclosed => "storage_location_undisclosed",
            Self::RegionPinUndeclaredOnManaged => "region_pin_undeclared_on_managed",
            Self::RegionPinUnhonored => "region_pin_unhonored",
            Self::RetentionClassUndisclosed => "retention_class_undisclosed",
            Self::TenantScopeUndisclosed => "tenant_scope_undisclosed",
            Self::TenantBoundaryUnverified => "tenant_boundary_unverified",
            Self::LocalityVocabularyDrift => "locality_vocabulary_drift",
            Self::SelfHostedLocalityOverclaimed => "self_hosted_locality_overclaimed",
            Self::SurfaceProjectionIncomplete => "surface_projection_incomplete",
        }
    }

    /// True when this reason withdraws the claim immediately (fails closed).
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::RegionPinUnhonored | Self::SelfHostedLocalityOverclaimed
        )
    }

    /// True when this reason holds the claim at preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(
            self,
            Self::RegionPinUndeclaredOnManaged
                | Self::TenantBoundaryUnverified
                | Self::LocalityVocabularyDrift
        )
    }
}

/// Derives a qualification from the locality/tenant narrow reasons present.
fn qualification_from_reasons<'a>(
    reasons: impl IntoIterator<Item = &'a LocalityTenantNarrowReasonClass>,
) -> ContinuityClaimQualificationClass {
    let mut saw_any = false;
    let mut saw_preview = false;
    for reason in reasons {
        saw_any = true;
        if reason.is_withdrawal_reason() {
            return ContinuityClaimQualificationClass::Withdrawn;
        }
        if reason.is_preview_reason() {
            saw_preview = true;
        }
    }
    if saw_preview {
        ContinuityClaimQualificationClass::Preview
    } else if saw_any {
        ContinuityClaimQualificationClass::Beta
    } else {
        ContinuityClaimQualificationClass::Stable
    }
}

/// One claimed surface decorated with the facts needed to build its descriptor
/// and tenant card.
///
/// The reused profile, lane, locality, tenant, and key-mode fields are sourced
/// from the frozen continuity-claim matrix so the locality vocabulary stays
/// identical; the region-pin, retention, and isolation fields are this lane's
/// additive truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalityTenantEntry {
    /// Opaque row identifier shared with the continuity-claim matrix.
    pub row_id: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Claimed deployment profile.
    pub profile_class: ContinuityProfileClass,
    /// Stable token for [`Self::profile_class`].
    pub profile_class_token: String,
    /// True when this row depends on a claimed managed or self-hosted lane.
    pub has_claimed_managed_dependency: bool,
    /// Continuity lane this row belongs to.
    pub continuity_lane: ContinuityLaneClass,
    /// Stable token for [`Self::continuity_lane`].
    pub continuity_lane_token: String,
    /// Where processing happens.
    pub processing_location: LocalityClass,
    /// Stable token for [`Self::processing_location`].
    pub processing_location_token: String,
    /// Where durable storage lives.
    pub storage_location: LocalityClass,
    /// Stable token for [`Self::storage_location`].
    pub storage_location_token: String,
    /// Export-safe region or residency label.
    pub residency_label: String,
    /// Declared region-pin posture.
    pub region_pin: RegionPinClass,
    /// Stable token for [`Self::region_pin`].
    pub region_pin_token: String,
    /// Whether the declared pin is honored.
    pub region_pin_honor: RegionPinHonorState,
    /// Stable token for [`Self::region_pin_honor`].
    pub region_pin_honor_token: String,
    /// Export-safe label naming the pinned region(s).
    pub region_pin_label: String,
    /// Retention / export class in force.
    pub retention_class: RetentionClass,
    /// Stable token for [`Self::retention_class`].
    pub retention_class_token: String,
    /// Tenant or org boundary.
    pub tenant_scope: TenantScopeClass,
    /// Stable token for [`Self::tenant_scope`].
    pub tenant_scope_token: String,
    /// Isolation posture backing the tenant boundary.
    pub tenant_isolation: TenantIsolationClass,
    /// Stable token for [`Self::tenant_isolation`].
    pub tenant_isolation_token: String,
    /// Key-mode posture protecting durable state.
    pub key_mode: KeyModeClass,
    /// Stable token for [`Self::key_mode`].
    pub key_mode_token: String,
    /// Surfaces this row is projected onto.
    pub projected_surfaces: Vec<LocalitySurfaceClass>,
}

impl LocalityTenantEntry {
    /// True when this row sits inside managed continuity scope.
    ///
    /// Managed, self-hosted, and sovereign profiles are always in scope, as is
    /// any row on the managed continuity lane or carrying a claimed managed
    /// dependency. A pure local-only row with no managed dependency is out of
    /// scope and is not held to managed-lane locality requirements.
    pub fn in_managed_scope(&self) -> bool {
        self.profile_class != ContinuityProfileClass::LocalOnly
            || self.continuity_lane == ContinuityLaneClass::ManagedLane
            || self.has_claimed_managed_dependency
    }

    /// Surfaces this row is required to reach.
    pub fn required_surfaces(&self) -> &'static [LocalitySurfaceClass] {
        if self.in_managed_scope() {
            &LocalitySurfaceClass::ALL
        } else {
            &LocalitySurfaceClass::LOCAL_CORE
        }
    }

    /// True when the managed lane must fail closed for this row.
    ///
    /// A managed-scope row whose declared region pin cannot be honored fails
    /// closed; local-core work never enters this rule.
    pub fn fail_closed_on_managed_lane(&self) -> bool {
        self.in_managed_scope() && self.region_pin_honor == RegionPinHonorState::CannotHonor
    }

    /// Builds an entry from a frozen continuity-claim row plus this lane's facts.
    #[allow(clippy::too_many_arguments)]
    pub fn from_claim_row(
        row: &ContinuityClaimRow,
        region_pin: RegionPinClass,
        region_pin_honor: RegionPinHonorState,
        region_pin_label: impl Into<String>,
        retention_class: RetentionClass,
        tenant_isolation: TenantIsolationClass,
        projected_surfaces: Vec<LocalitySurfaceClass>,
    ) -> Self {
        Self {
            row_id: row.row_id.clone(),
            surface_label: row.surface_label.clone(),
            profile_class: row.profile_class,
            profile_class_token: row.profile_class.as_str().to_owned(),
            has_claimed_managed_dependency: row.has_claimed_managed_dependency,
            continuity_lane: row.continuity_lane,
            continuity_lane_token: row.continuity_lane.as_str().to_owned(),
            processing_location: row.locality.processing_locality,
            processing_location_token: row.locality.processing_locality.as_str().to_owned(),
            storage_location: row.locality.storage_locality,
            storage_location_token: row.locality.storage_locality.as_str().to_owned(),
            residency_label: row.locality.residency_label.clone(),
            region_pin,
            region_pin_token: region_pin.as_str().to_owned(),
            region_pin_honor,
            region_pin_honor_token: region_pin_honor.as_str().to_owned(),
            region_pin_label: region_pin_label.into(),
            retention_class,
            retention_class_token: retention_class.as_str().to_owned(),
            tenant_scope: row.tenant_scope,
            tenant_scope_token: row.tenant_scope.as_str().to_owned(),
            tenant_isolation,
            tenant_isolation_token: tenant_isolation.as_str().to_owned(),
            key_mode: row.key_mode,
            key_mode_token: row.key_mode.as_str().to_owned(),
            projected_surfaces,
        }
    }
}

/// Plain-language processing/storage location, region pin, and retention for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalityDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque descriptor identifier.
    pub descriptor_id: String,
    /// Row this descriptor describes.
    pub row_id: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Stable token for the claimed profile.
    pub profile_class_token: String,
    /// Plain-language profile summary.
    pub profile_plain: String,
    /// Stable token for the continuity lane.
    pub continuity_lane_token: String,
    /// Plain-language continuity-lane summary.
    pub continuity_lane_plain: String,
    /// Stable token for the processing location.
    pub processing_location_token: String,
    /// Plain-language processing location.
    pub processing_location_plain: String,
    /// Stable token for the storage location.
    pub storage_location_token: String,
    /// Plain-language storage location.
    pub storage_location_plain: String,
    /// Export-safe region or residency label.
    pub residency_label: String,
    /// Stable token for the region pin.
    pub region_pin_token: String,
    /// Plain-language region pin.
    pub region_pin_plain: String,
    /// Stable token for the region-pin honor state.
    pub region_pin_honor_token: String,
    /// Plain-language region-pin honor state.
    pub region_pin_honor_plain: String,
    /// Export-safe label naming the pinned region(s).
    pub region_pin_label: String,
    /// Stable token for the retention class.
    pub retention_class_token: String,
    /// Plain-language retention summary.
    pub retention_plain: String,
    /// Canonical one-line locality summary reused by every surface projection.
    pub locality_summary_line: String,
    /// True when the managed lane fails closed because the pin cannot be honored.
    pub fail_closed_on_managed_lane: bool,
}

impl LocalityDescriptor {
    /// Builds a descriptor from a decorated entry.
    pub fn from_entry(entry: &LocalityTenantEntry) -> Self {
        Self {
            record_kind: LOCALITY_DESCRIPTOR_RECORD_KIND.to_owned(),
            schema_version: LOCALITY_TENANT_SCHEMA_VERSION,
            shared_contract_ref: LOCALITY_TENANT_SHARED_CONTRACT_REF.to_owned(),
            descriptor_id: format!("continuity:locality-descriptor:{}", entry.row_id),
            row_id: entry.row_id.clone(),
            surface_label: entry.surface_label.clone(),
            profile_class_token: entry.profile_class_token.clone(),
            profile_plain: profile_plain(entry.profile_class).to_owned(),
            continuity_lane_token: entry.continuity_lane_token.clone(),
            continuity_lane_plain: lane_plain(entry.continuity_lane).to_owned(),
            processing_location_token: entry.processing_location_token.clone(),
            processing_location_plain: locality_plain(entry.processing_location).to_owned(),
            storage_location_token: entry.storage_location_token.clone(),
            storage_location_plain: locality_plain(entry.storage_location).to_owned(),
            residency_label: entry.residency_label.clone(),
            region_pin_token: entry.region_pin_token.clone(),
            region_pin_plain: entry.region_pin.plain().to_owned(),
            region_pin_honor_token: entry.region_pin_honor_token.clone(),
            region_pin_honor_plain: entry.region_pin_honor.plain().to_owned(),
            region_pin_label: entry.region_pin_label.clone(),
            retention_class_token: entry.retention_class_token.clone(),
            retention_plain: entry.retention_class.plain().to_owned(),
            locality_summary_line: locality_summary_line(entry),
            fail_closed_on_managed_lane: entry.fail_closed_on_managed_lane(),
        }
    }
}

/// Plain-language tenant/org scope, isolation posture, and key mode for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TenantBoundaryCard {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque card identifier.
    pub card_id: String,
    /// Row this card describes.
    pub row_id: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Stable token for the tenant scope.
    pub tenant_scope_token: String,
    /// Plain-language tenant scope.
    pub tenant_scope_plain: String,
    /// Stable token for the tenant isolation.
    pub tenant_isolation_token: String,
    /// Plain-language tenant isolation.
    pub tenant_isolation_plain: String,
    /// Stable token for the key mode.
    pub key_mode_token: String,
    /// Plain-language key mode.
    pub key_mode_plain: String,
    /// Canonical one-line tenant summary reused by every surface projection.
    pub tenant_summary_line: String,
    /// True when the tenant boundary isolation is verified.
    pub boundary_verified: bool,
}

impl TenantBoundaryCard {
    /// Builds a tenant-boundary card from a decorated entry.
    pub fn from_entry(entry: &LocalityTenantEntry) -> Self {
        let boundary_verified = entry.tenant_isolation != TenantIsolationClass::IsolationUnverified
            && entry.tenant_scope != TenantScopeClass::TenantBoundaryRecheckRequired;
        Self {
            record_kind: TENANT_BOUNDARY_CARD_RECORD_KIND.to_owned(),
            schema_version: LOCALITY_TENANT_SCHEMA_VERSION,
            shared_contract_ref: LOCALITY_TENANT_SHARED_CONTRACT_REF.to_owned(),
            card_id: format!("continuity:tenant-card:{}", entry.row_id),
            row_id: entry.row_id.clone(),
            surface_label: entry.surface_label.clone(),
            tenant_scope_token: entry.tenant_scope_token.clone(),
            tenant_scope_plain: tenant_scope_plain(entry.tenant_scope).to_owned(),
            tenant_isolation_token: entry.tenant_isolation_token.clone(),
            tenant_isolation_plain: entry.tenant_isolation.plain().to_owned(),
            key_mode_token: entry.key_mode_token.clone(),
            key_mode_plain: key_mode_plain(entry.key_mode).to_owned(),
            tenant_summary_line: tenant_summary_line(entry),
            boundary_verified,
        }
    }
}

/// One surface rendering of a row's locality descriptor and tenant card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalitySurfaceProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Surface this projection renders on.
    pub surface: LocalitySurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Row this projection describes.
    pub row_id: String,
    /// Descriptor id rendered on this surface.
    pub descriptor_id: String,
    /// Card id rendered on this surface.
    pub card_id: String,
    /// Locality summary line rendered on this surface.
    pub locality_summary_line: String,
    /// Tenant summary line rendered on this surface.
    pub tenant_summary_line: String,
}

/// Per-row verdict joining a row to its computed qualification and reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalityTenantRowOutcome {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Row this outcome describes.
    pub row_id: String,
    /// Stable token for the claimed profile.
    pub profile_class_token: String,
    /// True when the row is in managed continuity scope.
    pub in_managed_scope: bool,
    /// Computed qualification token for the row.
    pub qualification_token: String,
    /// True when the row narrowed below stable.
    pub narrowed: bool,
    /// True when the row's claim is withheld entirely.
    pub claim_withheld: bool,
    /// True when the managed lane failed closed for this row.
    pub fail_closed: bool,
    /// Stable narrow-reason tokens that applied to the row.
    pub narrow_reason_tokens: Vec<String>,
}

/// Typed defect emitted by the locality/tenant audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalityTenantDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque defect identifier.
    pub defect_id: String,
    /// Typed narrow reason.
    pub narrow_reason: LocalityTenantNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Opaque source row id that triggered the defect.
    pub source: String,
    /// Export-safe explanation of the defect.
    pub note: String,
}

impl LocalityTenantDefect {
    fn new(
        narrow_reason: LocalityTenantNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: LOCALITY_TENANT_DEFECT_RECORD_KIND.to_owned(),
            schema_version: LOCALITY_TENANT_SCHEMA_VERSION,
            shared_contract_ref: LOCALITY_TENANT_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "continuity:defect:locality-tenant:{}:{}",
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

/// Aggregate summary for a locality/tenant card page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalityTenantSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Overall qualification for the page.
    pub overall_qualification_token: String,
    /// Number of claimed entries.
    pub entry_count: usize,
    /// Number of entries in managed continuity scope.
    pub managed_scope_entry_count: usize,
    /// Number of entries on the local-core continuity lane.
    pub local_core_entry_count: usize,
    /// Number of entries declaring an explicit region pin.
    pub region_pinned_entry_count: usize,
    /// Number of entries whose managed lane failed closed.
    pub fail_closed_entry_count: usize,
    /// Number of entries that narrowed below stable.
    pub narrowed_entry_count: usize,
    /// Number of entries whose claim is withheld.
    pub withdrawn_entry_count: usize,
    /// Number of surface projections emitted.
    pub surface_projection_count: usize,
    /// True when every surface renders the same locality/tenant vocabulary.
    pub vocabulary_consistent: bool,
    /// Number of defects recorded for the page.
    pub defect_count: usize,
}

/// Full auditable input for the locality/tenant card page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalityTenantInput {
    /// Reviewable label for the page.
    pub input_label: String,
    /// Claimed entries.
    pub entries: Vec<LocalityTenantEntry>,
}

/// Canonical proof packet for locality descriptors and tenant-boundary cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalityTenantCardPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page identifier.
    pub page_id: String,
    /// Reviewable page label.
    pub page_label: String,
    /// UTC timestamp when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary derived from the embedded input and defects.
    pub summary: LocalityTenantSummary,
    /// Typed defects for the packet.
    pub defects: Vec<LocalityTenantDefect>,
    /// Locality descriptors, one per entry.
    pub descriptors: Vec<LocalityDescriptor>,
    /// Tenant-boundary cards, one per entry.
    pub tenant_cards: Vec<TenantBoundaryCard>,
    /// Per-surface projections proving identical vocabulary across surfaces.
    pub surface_projections: Vec<LocalitySurfaceProjection>,
    /// Per-row verdicts joining each row to its computed qualification.
    pub row_outcomes: Vec<LocalityTenantRowOutcome>,
    /// The audited input embedded as evidence.
    pub input: LocalityTenantInput,
}

impl LocalityTenantCardPage {
    /// Builds a locality/tenant card page from the supplied input.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: LocalityTenantInput,
    ) -> Self {
        let descriptors: Vec<LocalityDescriptor> = input
            .entries
            .iter()
            .map(LocalityDescriptor::from_entry)
            .collect();
        let tenant_cards: Vec<TenantBoundaryCard> = input
            .entries
            .iter()
            .map(TenantBoundaryCard::from_entry)
            .collect();
        let surface_projections = build_surface_projections(&input.entries);
        let defects = audit(&input, &surface_projections);
        let row_outcomes = build_row_outcomes(&input, &defects);
        let summary = build_summary(&input, &surface_projections, &row_outcomes, &defects);
        Self {
            record_kind: LOCALITY_TENANT_PAGE_RECORD_KIND.to_owned(),
            schema_version: LOCALITY_TENANT_SCHEMA_VERSION,
            shared_contract_ref: LOCALITY_TENANT_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            defects,
            descriptors,
            tenant_cards,
            surface_projections,
            row_outcomes,
            input,
        }
    }

    /// True when the page qualifies stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == ContinuityClaimQualificationClass::Stable.as_str()
    }

    /// True when every surface renders identical locality/tenant vocabulary.
    pub fn surfaces_share_vocabulary(&self) -> bool {
        self.summary.vocabulary_consistent
    }

    /// Returns the descriptor for a row id, if present.
    pub fn descriptor(&self, row_id: &str) -> Option<&LocalityDescriptor> {
        self.descriptors.iter().find(|d| d.row_id == row_id)
    }

    /// Returns the tenant card for a row id, if present.
    pub fn tenant_card(&self, row_id: &str) -> Option<&TenantBoundaryCard> {
        self.tenant_cards.iter().find(|c| c.row_id == row_id)
    }

    /// Returns the computed outcome for a row id, if present.
    pub fn row_outcome(&self, row_id: &str) -> Option<&LocalityTenantRowOutcome> {
        self.row_outcomes.iter().find(|o| o.row_id == row_id)
    }

    /// Returns the descriptors whose managed lane failed closed.
    pub fn fail_closed_descriptors(&self) -> Vec<&LocalityDescriptor> {
        self.descriptors
            .iter()
            .filter(|d| d.fail_closed_on_managed_lane)
            .collect()
    }
}

/// Support-export wrapper for the locality/tenant card page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalityTenantSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export identifier.
    pub export_id: String,
    /// UTC timestamp when the export was produced.
    pub generated_at: String,
    /// The locality/tenant card page embedded as evidence.
    pub page: LocalityTenantCardPage,
    /// Typed narrow reasons present in the embedded packet.
    pub narrow_reasons_present: Vec<LocalityTenantNarrowReasonClass>,
    /// True when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl LocalityTenantSupportExport {
    /// Wraps a locality/tenant card page inside a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: LocalityTenantCardPage,
    ) -> Self {
        let mut reasons: Vec<LocalityTenantNarrowReasonClass> = Vec::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
        }
        reasons.sort();
        Self {
            record_kind: LOCALITY_TENANT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: LOCALITY_TENANT_SCHEMA_VERSION,
            shared_contract_ref: LOCALITY_TENANT_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            raw_private_material_excluded: true,
        }
    }
}

/// Re-runs the locality/tenant audit over a page, including its stored projections.
///
/// Unlike [`LocalityTenantCardPage::new`], this validates the page's stored
/// surface projections against freshly derived canonical lines, so a tampered
/// projection (one that renders different vocabulary than its descriptor) is
/// caught on re-validation.
pub fn audit_locality_tenant_card_page(page: &LocalityTenantCardPage) -> Vec<LocalityTenantDefect> {
    audit(&page.input, &page.surface_projections)
}

/// Validates a card page and returns `Ok(())` when the audit is clean.
pub fn validate_locality_tenant_card_page(
    page: &LocalityTenantCardPage,
) -> Result<(), Vec<LocalityTenantDefect>> {
    let defects = audit_locality_tenant_card_page(page);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Returns the seeded stable locality/tenant card page.
pub fn seeded_locality_tenant_card_page() -> LocalityTenantCardPage {
    LocalityTenantCardPage::new(
        "continuity:locality-tenant:seeded",
        "Locality descriptors and tenant-boundary cards",
        "2026-06-01T00:00:00Z",
        seeded_locality_tenant_input(),
    )
}

/// Returns the seeded input used by the canonical card page.
///
/// The entries reuse the frozen continuity-claim matrix rows as their locality,
/// tenant, and key-mode source, decorated with this lane's region-pin,
/// retention, and isolation truth.
pub fn seeded_locality_tenant_input() -> LocalityTenantInput {
    let claim_rows = seeded_continuity_claim_matrix_input().claim_rows;
    let entries = claim_rows.iter().map(decorate_seed_row).collect();
    LocalityTenantInput {
        input_label: "Claimed managed, self-hosted, and sovereign locality and tenant surfaces"
            .to_owned(),
        entries,
    }
}

fn decorate_seed_row(row: &ContinuityClaimRow) -> LocalityTenantEntry {
    let all = LocalitySurfaceClass::ALL.to_vec();
    let local_core = LocalitySurfaceClass::LOCAL_CORE.to_vec();
    match row.row_id.as_str() {
        "continuity-row:managed-cloud-sync" => LocalityTenantEntry::from_claim_row(
            row,
            RegionPinClass::SingleRegionPinned,
            RegionPinHonorState::Honored,
            "us-west managed region",
            RetentionClass::VendorDefaultRetention,
            TenantIsolationClass::LogicalMultiTenant,
            all,
        ),
        "continuity-row:managed-relay-failover" => LocalityTenantEntry::from_claim_row(
            row,
            RegionPinClass::MultiRegionPinned,
            RegionPinHonorState::Honored,
            "us-west and us-east managed regions",
            RetentionClass::VendorDefaultRetention,
            TenantIsolationClass::DedicatedInfrastructure,
            all,
        ),
        "continuity-row:self-hosted-restore" => LocalityTenantEntry::from_claim_row(
            row,
            RegionPinClass::CustomerRegionPinned,
            RegionPinHonorState::Honored,
            "customer-operated eu-central region",
            RetentionClass::CustomerConfiguredRetention,
            TenantIsolationClass::CustomerBoundary,
            all,
        ),
        "continuity-row:sovereign-airgap-snapshot" => LocalityTenantEntry::from_claim_row(
            row,
            RegionPinClass::InCountryPinned,
            RegionPinHonorState::Honored,
            "isolated in-country customer network",
            RetentionClass::LegalHoldRetention,
            TenantIsolationClass::CustomerBoundary,
            all,
        ),
        _ => LocalityTenantEntry::from_claim_row(
            row,
            RegionPinClass::PinNotApplicable,
            RegionPinHonorState::NotApplicable,
            "",
            RetentionClass::DeviceLocalRetention,
            TenantIsolationClass::ProcessLocalIsolation,
            local_core,
        ),
    }
}

fn audit(
    input: &LocalityTenantInput,
    projections: &[LocalitySurfaceProjection],
) -> Vec<LocalityTenantDefect> {
    let mut defects = Vec::new();
    for entry in &input.entries {
        audit_entry(entry, &mut defects);
    }
    audit_vocabulary(input, projections, &mut defects);
    defects
}

fn audit_entry(entry: &LocalityTenantEntry, defects: &mut Vec<LocalityTenantDefect>) {
    // Locality and retention disclosure apply to every claimed row.
    if entry.processing_location == LocalityClass::Undisclosed {
        defects.push(LocalityTenantDefect::new(
            LocalityTenantNarrowReasonClass::ProcessingLocationUndisclosed,
            entry.row_id.clone(),
            "every claimed row must disclose where processing happens",
        ));
    }
    if entry.storage_location == LocalityClass::Undisclosed {
        defects.push(LocalityTenantDefect::new(
            LocalityTenantNarrowReasonClass::StorageLocationUndisclosed,
            entry.row_id.clone(),
            "every claimed row must disclose where durable storage lives",
        ));
    }
    if entry.retention_class == RetentionClass::RetentionUndisclosed {
        defects.push(LocalityTenantDefect::new(
            LocalityTenantNarrowReasonClass::RetentionClassUndisclosed,
            entry.row_id.clone(),
            "every claimed row must disclose the retention or export class in force",
        ));
    }

    // Surface projection completeness.
    let missing: Vec<&LocalitySurfaceClass> = entry
        .required_surfaces()
        .iter()
        .filter(|surface| !entry.projected_surfaces.contains(surface))
        .collect();
    if !missing.is_empty() {
        defects.push(LocalityTenantDefect::new(
            LocalityTenantNarrowReasonClass::SurfaceProjectionIncomplete,
            entry.row_id.clone(),
            "this row's locality descriptor and tenant card must reach every required surface",
        ));
    }

    if !entry.in_managed_scope() {
        // Local-core rows owe disclosure and projection only; the managed-lane
        // region-pin and tenant rules below never apply, so they stay accurately
        // labeled rather than being narrowed against managed expectations.
        return;
    }

    // Region pinning must be explicit on the protected managed lane.
    if !entry.region_pin.is_pinned() || entry.region_pin_label.is_empty() {
        defects.push(LocalityTenantDefect::new(
            LocalityTenantNarrowReasonClass::RegionPinUndeclaredOnManaged,
            entry.row_id.clone(),
            "managed, self-hosted, and sovereign rows must declare and name an explicit region pin",
        ));
    }

    // Fail closed: a declared managed-lane pin that cannot be honored withdraws.
    if entry.region_pin_honor == RegionPinHonorState::CannotHonor {
        defects.push(LocalityTenantDefect::new(
            LocalityTenantNarrowReasonClass::RegionPinUnhonored,
            entry.row_id.clone(),
            "the declared region pin cannot be honored; the managed lane fails closed",
        ));
    }

    // Tenant scope and boundary verification.
    if entry.tenant_scope == TenantScopeClass::NotApplicable {
        defects.push(LocalityTenantDefect::new(
            LocalityTenantNarrowReasonClass::TenantScopeUndisclosed,
            entry.row_id.clone(),
            "managed-scope rows must disclose an explicit tenant or org scope",
        ));
    }
    if entry.tenant_isolation == TenantIsolationClass::IsolationUnverified
        || entry.tenant_scope == TenantScopeClass::TenantBoundaryRecheckRequired
    {
        defects.push(LocalityTenantDefect::new(
            LocalityTenantNarrowReasonClass::TenantBoundaryUnverified,
            entry.row_id.clone(),
            "the tenant boundary isolation has not been verified for this row",
        ));
    }

    // Guardrail: a self-hosted or sovereign row may not imply vendor-grade broad
    // locality the running topology does not actually provide.
    if entry.profile_class.is_self_governed()
        && matches!(
            entry.storage_location,
            LocalityClass::SingleRegion | LocalityClass::MultiRegion
        )
    {
        defects.push(LocalityTenantDefect::new(
            LocalityTenantNarrowReasonClass::SelfHostedLocalityOverclaimed,
            entry.row_id.clone(),
            "a self-hosted or sovereign row may not claim a broad vendor region it does not operate",
        ));
    }
}

fn audit_vocabulary(
    input: &LocalityTenantInput,
    projections: &[LocalitySurfaceProjection],
    defects: &mut Vec<LocalityTenantDefect>,
) {
    for entry in &input.entries {
        let canonical_locality = locality_summary_line(entry);
        let canonical_tenant = tenant_summary_line(entry);
        let drifted = projections
            .iter()
            .filter(|projection| projection.row_id == entry.row_id)
            .any(|projection| {
                projection.locality_summary_line != canonical_locality
                    || projection.tenant_summary_line != canonical_tenant
            });
        if drifted {
            defects.push(LocalityTenantDefect::new(
                LocalityTenantNarrowReasonClass::LocalityVocabularyDrift,
                entry.row_id.clone(),
                "a surface renders different locality or tenant vocabulary than the descriptor",
            ));
        }
    }
}

fn build_surface_projections(entries: &[LocalityTenantEntry]) -> Vec<LocalitySurfaceProjection> {
    let mut projections = Vec::new();
    for entry in entries {
        let locality_summary_line = locality_summary_line(entry);
        let tenant_summary_line = tenant_summary_line(entry);
        let descriptor_id = format!("continuity:locality-descriptor:{}", entry.row_id);
        let card_id = format!("continuity:tenant-card:{}", entry.row_id);
        for surface in LocalitySurfaceClass::ALL {
            if !entry.projected_surfaces.contains(&surface) {
                continue;
            }
            projections.push(LocalitySurfaceProjection {
                record_kind: LOCALITY_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
                schema_version: LOCALITY_TENANT_SCHEMA_VERSION,
                shared_contract_ref: LOCALITY_TENANT_SHARED_CONTRACT_REF.to_owned(),
                surface,
                surface_token: surface.as_str().to_owned(),
                row_id: entry.row_id.clone(),
                descriptor_id: descriptor_id.clone(),
                card_id: card_id.clone(),
                locality_summary_line: locality_summary_line.clone(),
                tenant_summary_line: tenant_summary_line.clone(),
            });
        }
    }
    projections
}

fn build_row_outcomes(
    input: &LocalityTenantInput,
    defects: &[LocalityTenantDefect],
) -> Vec<LocalityTenantRowOutcome> {
    input
        .entries
        .iter()
        .map(|entry| {
            let reasons: Vec<LocalityTenantNarrowReasonClass> = defects
                .iter()
                .filter(|defect| defect.source == entry.row_id)
                .map(|defect| defect.narrow_reason)
                .collect();
            let qualification = qualification_from_reasons(reasons.iter());
            let mut reason_tokens: Vec<String> = reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect();
            reason_tokens.sort();
            reason_tokens.dedup();
            LocalityTenantRowOutcome {
                record_kind: LOCALITY_TENANT_ROW_OUTCOME_RECORD_KIND.to_owned(),
                schema_version: LOCALITY_TENANT_SCHEMA_VERSION,
                shared_contract_ref: LOCALITY_TENANT_SHARED_CONTRACT_REF.to_owned(),
                row_id: entry.row_id.clone(),
                profile_class_token: entry.profile_class_token.clone(),
                in_managed_scope: entry.in_managed_scope(),
                qualification_token: qualification.as_str().to_owned(),
                narrowed: qualification != ContinuityClaimQualificationClass::Stable,
                claim_withheld: qualification == ContinuityClaimQualificationClass::Withdrawn,
                fail_closed: entry.fail_closed_on_managed_lane(),
                narrow_reason_tokens: reason_tokens,
            }
        })
        .collect()
}

fn build_summary(
    input: &LocalityTenantInput,
    projections: &[LocalitySurfaceProjection],
    row_outcomes: &[LocalityTenantRowOutcome],
    defects: &[LocalityTenantDefect],
) -> LocalityTenantSummary {
    let overall = if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_withdrawal_reason())
    {
        ContinuityClaimQualificationClass::Withdrawn
    } else if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_preview_reason())
    {
        ContinuityClaimQualificationClass::Preview
    } else if defects.is_empty() {
        ContinuityClaimQualificationClass::Stable
    } else {
        ContinuityClaimQualificationClass::Beta
    };

    let vocabulary_consistent = !defects.iter().any(|defect| {
        defect.narrow_reason == LocalityTenantNarrowReasonClass::LocalityVocabularyDrift
    });

    LocalityTenantSummary {
        record_kind: LOCALITY_TENANT_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: LOCALITY_TENANT_SCHEMA_VERSION,
        shared_contract_ref: LOCALITY_TENANT_SHARED_CONTRACT_REF.to_owned(),
        overall_qualification_token: overall.as_str().to_owned(),
        entry_count: input.entries.len(),
        managed_scope_entry_count: input
            .entries
            .iter()
            .filter(|entry| entry.in_managed_scope())
            .count(),
        local_core_entry_count: input
            .entries
            .iter()
            .filter(|entry| entry.continuity_lane == ContinuityLaneClass::LocalCore)
            .count(),
        region_pinned_entry_count: input
            .entries
            .iter()
            .filter(|entry| entry.region_pin.is_pinned())
            .count(),
        fail_closed_entry_count: input
            .entries
            .iter()
            .filter(|entry| entry.fail_closed_on_managed_lane())
            .count(),
        narrowed_entry_count: row_outcomes
            .iter()
            .filter(|outcome| outcome.narrowed)
            .count(),
        withdrawn_entry_count: row_outcomes
            .iter()
            .filter(|outcome| outcome.claim_withheld)
            .count(),
        surface_projection_count: projections.len(),
        vocabulary_consistent,
        defect_count: defects.len(),
    }
}

fn locality_summary_line(entry: &LocalityTenantEntry) -> String {
    format!(
        "Processing {}; storage {}; region pin {}, {}; retention {}.",
        clause(
            locality_plain(entry.processing_location),
            &entry.residency_label
        ),
        clause(
            locality_plain(entry.storage_location),
            &entry.residency_label
        ),
        clause(entry.region_pin.plain(), &entry.region_pin_label),
        entry.region_pin_honor.plain(),
        entry.retention_class.plain(),
    )
}

fn tenant_summary_line(entry: &LocalityTenantEntry) -> String {
    format!(
        "Tenant {}; isolation {}; keys {}.",
        tenant_scope_plain(entry.tenant_scope),
        entry.tenant_isolation.plain(),
        key_mode_plain(entry.key_mode),
    )
}

fn clause(plain: &str, label: &str) -> String {
    if label.is_empty() {
        plain.to_owned()
    } else {
        format!("{plain} ({label})")
    }
}

fn profile_plain(class: ContinuityProfileClass) -> &'static str {
    match class {
        ContinuityProfileClass::Managed => "vendor-managed cloud",
        ContinuityProfileClass::SelfHosted => "customer self-hosted",
        ContinuityProfileClass::Sovereign => "sovereign or air-gapped",
        ContinuityProfileClass::LocalOnly => "local desktop only",
    }
}

fn lane_plain(class: ContinuityLaneClass) -> &'static str {
    match class {
        ContinuityLaneClass::LocalCore => "local-core continuity",
        ContinuityLaneClass::ManagedLane => "managed continuity lane",
    }
}

fn locality_plain(class: LocalityClass) -> &'static str {
    match class {
        LocalityClass::DeviceLocal => "on this device",
        LocalityClass::SingleRegion => "in a single managed region",
        LocalityClass::MultiRegion => "across multiple managed regions",
        LocalityClass::CustomerRegion => "in a customer-operated region",
        LocalityClass::InCountrySovereign => "inside an in-country sovereign boundary",
        LocalityClass::AirGappedIsolated => "inside an air-gapped isolated boundary",
        LocalityClass::Undisclosed => "not disclosed",
    }
}

fn tenant_scope_plain(class: TenantScopeClass) -> &'static str {
    match class {
        TenantScopeClass::SingleUserLocal => "single local user",
        TenantScopeClass::CustomerTenant => "customer-owned tenant",
        TenantScopeClass::DedicatedTenant => "dedicated single-customer tenant",
        TenantScopeClass::SharedMultiTenant => "shared multi-tenant",
        TenantScopeClass::TenantBoundaryRecheckRequired => "tenant boundary needs recheck",
        TenantScopeClass::NotApplicable => "not applicable",
    }
}

fn key_mode_plain(class: KeyModeClass) -> &'static str {
    match class {
        KeyModeClass::LocalOsKeystore => "local OS keystore",
        KeyModeClass::VendorManagedKeys => "vendor-managed keys",
        KeyModeClass::CustomerManagedKeys => "customer-managed keys",
        KeyModeClass::CustomerHeldRoot => "customer-held root key",
        KeyModeClass::HybridEnvelope => "hybrid key envelope",
        KeyModeClass::NotApplicable => "not applicable",
    }
}
