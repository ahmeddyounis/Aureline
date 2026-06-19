//! Canonical locality, tenant/key-mode, backup/restore/failover, and
//! continuity-drill matrix for claimed managed, self-hosted, and sovereign
//! product surfaces.
//!
//! This module freezes one metadata-safe packet that turns deployment
//! footnotes into typed continuity-claim rows. About, Help, service-health,
//! support exports, docs/public-truth pages, and partner qualification packets
//! reuse it instead of restating continuity claims by hand, so every claimed
//! managed, self-hosted, or sovereign surface answers the same questions the
//! same way:
//!
//! 1. Where does processing and storage actually happen, which tenant boundary
//!    applies, and which key mode protects durable state?
//! 2. Does this surface sit on the local-core continuity lane or the managed
//!    continuity lane, and does its degraded fallback distinguish control-plane
//!    impairment from data-plane impairment?
//! 3. Which named backup, restore, failover, or snapshot continuity packet
//!    family backs the claim, what restore identity does a recovery produce,
//!    and what partial loss is disclosed?
//! 4. On what cadence is that continuity packet drilled, who owns the drill now
//!    and next, and is the drill evidence current, reconstructable, stale, or
//!    never run?
//! 5. When locality, tenant/key posture, degraded-fallback class, continuity
//!    packet family, or drill evidence is missing, stale, or profile-mismatched,
//!    the claim narrows automatically rather than inheriting green
//!    enterprise/managed language.
//!
//! The packet is intentionally metadata-only. It carries closed-vocabulary
//! tokens, export-safe labels, UTC timestamps, and opaque refs only. Raw
//! hostnames, raw tenant identifiers, raw KMS handles, raw trust roots, raw
//! backup bytes, and secret material never cross this boundary.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version carried on every record in this module.
pub const CONTINUITY_CLAIM_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const CONTINUITY_CLAIM_MATRIX_SHARED_CONTRACT_REF: &str =
    "continuity:m5_locality_tenant_keymode_and_drill_matrix:v1";

/// Record-kind tag for [`ContinuityClaimMatrixPage`] payloads.
pub const CONTINUITY_CLAIM_MATRIX_PAGE_RECORD_KIND: &str = "continuity_claim_matrix_page_record";

/// Record-kind tag for [`ContinuityClaimMatrixSummary`] payloads.
pub const CONTINUITY_CLAIM_MATRIX_SUMMARY_RECORD_KIND: &str =
    "continuity_claim_matrix_summary_record";

/// Record-kind tag for [`ContinuityClaimRowOutcome`] payloads.
pub const CONTINUITY_CLAIM_ROW_OUTCOME_RECORD_KIND: &str = "continuity_claim_row_outcome_record";

/// Record-kind tag for [`ContinuityClaimDefect`] payloads.
pub const CONTINUITY_CLAIM_DEFECT_RECORD_KIND: &str = "continuity_claim_defect_record";

/// Record-kind tag for [`ContinuityClaimMatrixSupportExport`] payloads.
pub const CONTINUITY_CLAIM_MATRIX_SUPPORT_EXPORT_RECORD_KIND: &str =
    "continuity_claim_matrix_support_export_record";

/// Repo-relative path of the canonical reviewer doc for this lane.
pub const CONTINUITY_CLAIM_MATRIX_DOC_REF: &str =
    "docs/m5/continuity/locality_tenant_keymode_and_drill_matrix.md";

/// Repo-relative path of the checked-in artifact for this lane.
pub const CONTINUITY_CLAIM_MATRIX_ARTIFACT_REF: &str =
    "artifacts/m5/continuity/claim_rows_and_drill_schedule.md";

/// Repo-relative path of the canonical JSON schema for this lane.
pub const CONTINUITY_CLAIM_MATRIX_SCHEMA_REF: &str =
    "schemas/continuity/m5-continuity-claim-row.schema.json";

/// Claimed deployment posture for a continuity-claim row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityProfileClass {
    /// Vendor-operated managed-cloud surface.
    Managed,
    /// Customer-operated self-hosted surface.
    SelfHosted,
    /// Sovereign or air-gapped surface that keeps authority inside a boundary.
    Sovereign,
    /// Pure local desktop surface with no claimed managed dependency.
    LocalOnly,
}

impl ContinuityProfileClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Managed => "managed",
            Self::SelfHosted => "self_hosted",
            Self::Sovereign => "sovereign",
            Self::LocalOnly => "local_only",
        }
    }

    /// True when this profile must keep keys and restore/failover inside its own boundary.
    pub const fn is_self_governed(self) -> bool {
        matches!(self, Self::SelfHosted | Self::Sovereign)
    }
}

/// Continuity lane a row belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityLaneClass {
    /// Local-core continuity that survives without any managed lane.
    LocalCore,
    /// Managed continuity provided by a hosted, self-hosted, or sovereign lane.
    ManagedLane,
}

impl ContinuityLaneClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCore => "local_core",
            Self::ManagedLane => "managed_lane",
        }
    }
}

/// Processing or storage locality for a continuity-claim row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalityClass {
    /// Entirely on the local device.
    DeviceLocal,
    /// A single managed or customer region.
    SingleRegion,
    /// More than one region for resilience or routing.
    MultiRegion,
    /// A customer-operated region.
    CustomerRegion,
    /// An in-country sovereign boundary.
    InCountrySovereign,
    /// An isolated, air-gapped boundary.
    AirGappedIsolated,
    /// Locality is not disclosed; the claim must narrow.
    Undisclosed,
}

impl LocalityClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeviceLocal => "device_local",
            Self::SingleRegion => "single_region",
            Self::MultiRegion => "multi_region",
            Self::CustomerRegion => "customer_region",
            Self::InCountrySovereign => "in_country_sovereign",
            Self::AirGappedIsolated => "air_gapped_isolated",
            Self::Undisclosed => "undisclosed",
        }
    }
}

/// Tenant or org boundary for a continuity-claim row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TenantScopeClass {
    /// A single local user with no shared tenancy.
    SingleUserLocal,
    /// A customer-owned tenant.
    CustomerTenant,
    /// A dedicated single-customer tenant on managed infrastructure.
    DedicatedTenant,
    /// A shared multi-tenant managed surface.
    SharedMultiTenant,
    /// The tenant boundary needs an explicit recheck before claiming.
    TenantBoundaryRecheckRequired,
    /// Tenancy does not apply to this row.
    NotApplicable,
}

impl TenantScopeClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleUserLocal => "single_user_local",
            Self::CustomerTenant => "customer_tenant",
            Self::DedicatedTenant => "dedicated_tenant",
            Self::SharedMultiTenant => "shared_multi_tenant",
            Self::TenantBoundaryRecheckRequired => "tenant_boundary_recheck_required",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Key-mode posture protecting durable state for a continuity-claim row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyModeClass {
    /// Keys held in the local OS keystore.
    LocalOsKeystore,
    /// Keys are vendor-managed.
    VendorManagedKeys,
    /// Customer-managed keys in a customer KMS.
    CustomerManagedKeys,
    /// A customer-held root with no vendor escrow.
    CustomerHeldRoot,
    /// A hybrid envelope mixing customer and vendor key material.
    HybridEnvelope,
    /// Key mode does not apply to this row.
    NotApplicable,
}

impl KeyModeClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOsKeystore => "local_os_keystore",
            Self::VendorManagedKeys => "vendor_managed_keys",
            Self::CustomerManagedKeys => "customer_managed_keys",
            Self::CustomerHeldRoot => "customer_held_root",
            Self::HybridEnvelope => "hybrid_envelope",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Which plane a degraded-fallback claim covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaneImpairmentClass {
    /// Control-plane (identity, policy, catalog, mirror) impairment.
    ControlPlaneImpairment,
    /// Data-plane (editing, runtime, artifact bytes) impairment.
    DataPlaneImpairment,
    /// Both planes can be impaired and both are addressed.
    BothPlanes,
}

impl PlaneImpairmentClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlPlaneImpairment => "control_plane_impairment",
            Self::DataPlaneImpairment => "data_plane_impairment",
            Self::BothPlanes => "both_planes",
        }
    }

    /// True when the class covers the control plane.
    pub const fn covers_control_plane(self) -> bool {
        matches!(self, Self::ControlPlaneImpairment | Self::BothPlanes)
    }

    /// True when the class covers the data plane.
    pub const fn covers_data_plane(self) -> bool {
        matches!(self, Self::DataPlaneImpairment | Self::BothPlanes)
    }
}

/// Named continuity packet family that backs a claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityPacketFamilyClass {
    /// Backup capture and verification packets.
    Backup,
    /// Restore execution and validation packets.
    Restore,
    /// Failover and fallback routing packets.
    Failover,
    /// Snapshot and replication packets.
    SnapshotReplication,
    /// Local-core continuity that needs no managed packet family.
    LocalCoreContinuity,
}

impl ContinuityPacketFamilyClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Backup => "backup",
            Self::Restore => "restore",
            Self::Failover => "failover",
            Self::SnapshotReplication => "snapshot_replication",
            Self::LocalCoreContinuity => "local_core_continuity",
        }
    }

    /// True when this family represents a managed continuity packet.
    pub const fn is_managed_family(self) -> bool {
        !matches!(self, Self::LocalCoreContinuity)
    }

    /// True when this family must declare an explicit restore identity.
    pub const fn requires_restore_identity(self) -> bool {
        self.is_managed_family()
    }
}

/// Where the restore or failover path actually executes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreFailoverHostingClass {
    /// Entirely local-core recovery.
    LocalCore,
    /// Customer-operated recovery.
    CustomerOperated,
    /// Vendor-operated recovery.
    VendorOperated,
    /// Recovery served from an approved mirror.
    MirrorBacked,
    /// Recovery served from a signed offline snapshot.
    OfflineSnapshot,
}

impl RestoreFailoverHostingClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCore => "local_core",
            Self::CustomerOperated => "customer_operated",
            Self::VendorOperated => "vendor_operated",
            Self::MirrorBacked => "mirror_backed",
            Self::OfflineSnapshot => "offline_snapshot",
        }
    }
}

/// Identity a successful restore or failover reproduces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreIdentityClass {
    /// Recovery reproduces the same durable identity.
    SameIdentityRestore,
    /// Recovery reissues a derived identity that must be re-trusted.
    ReissuedIdentityRestore,
    /// Recovery requires a new install rebind.
    NewInstallRebind,
    /// Restore identity does not apply to this row.
    NotApplicable,
}

impl RestoreIdentityClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameIdentityRestore => "same_identity_restore",
            Self::ReissuedIdentityRestore => "reissued_identity_restore",
            Self::NewInstallRebind => "new_install_rebind",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Partial-loss disclosure for a continuity-claim row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartialLossClass {
    /// No partial loss is possible for the covered state.
    NoPartialLoss,
    /// A bounded recent window of writes may be lost.
    BoundedRecentWindowLoss,
    /// Queued or in-flight actions may be lost on recovery.
    QueuedActionLoss,
    /// Only cache or derived state may be lost.
    CacheOnlyLoss,
    /// Partial-loss behavior is not disclosed; the claim must narrow.
    Undisclosed,
}

impl PartialLossClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPartialLoss => "no_partial_loss",
            Self::BoundedRecentWindowLoss => "bounded_recent_window_loss",
            Self::QueuedActionLoss => "queued_action_loss",
            Self::CacheOnlyLoss => "cache_only_loss",
            Self::Undisclosed => "undisclosed",
        }
    }
}

/// Cadence at which a continuity packet family is drilled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillCadenceClass {
    /// Drilled with every release.
    PerRelease,
    /// Drilled monthly.
    Monthly,
    /// Drilled quarterly.
    Quarterly,
    /// Drilled twice a year.
    Semiannual,
    /// Drilled once a year.
    Annual,
    /// Exercised only on demand; insufficient for a managed-lane claim.
    OnDemandOnly,
}

impl DrillCadenceClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerRelease => "per_release",
            Self::Monthly => "monthly",
            Self::Quarterly => "quarterly",
            Self::Semiannual => "semiannual",
            Self::Annual => "annual",
            Self::OnDemandOnly => "on_demand_only",
        }
    }
}

/// Freshness state of a continuity drill's evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillEvidenceStateClass {
    /// Drill evidence is current within the cadence window.
    Current,
    /// Drill evidence is stale but within an explicit grace window.
    StaleWithinGrace,
    /// Drill evidence is stale enough that a fresh drill is required.
    StaleNeedsDrill,
    /// The drill has never been run.
    NeverRun,
    /// No live drill, but recovery is reconstructable from a verified snapshot.
    ReconstructableFromSnapshot,
}

impl DrillEvidenceStateClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleWithinGrace => "stale_within_grace",
            Self::StaleNeedsDrill => "stale_needs_drill",
            Self::NeverRun => "never_run",
            Self::ReconstructableFromSnapshot => "reconstructable_from_snapshot",
        }
    }

    /// True when the evidence is current or reconstructable and need not narrow.
    pub const fn is_acceptable(self) -> bool {
        matches!(
            self,
            Self::Current | Self::StaleWithinGrace | Self::ReconstructableFromSnapshot
        )
    }

    /// True when the evidence requires a fresh drill before claiming managed continuity.
    pub const fn needs_drill(self) -> bool {
        matches!(self, Self::StaleNeedsDrill | Self::NeverRun)
    }

    /// True when the evidence must record a last-drill timestamp.
    pub const fn requires_last_drill_timestamp(self) -> bool {
        matches!(self, Self::Current | Self::StaleWithinGrace)
    }
}

/// Stability qualification for a row or for the whole matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityClaimQualificationClass {
    /// Every required disclosure and drill condition is satisfied.
    Stable,
    /// The claim stands but one or more disclosures are incomplete.
    Beta,
    /// Coverage gaps hold the claim at preview.
    Preview,
    /// The claim overstates a sovereign or self-hosted boundary and is withdrawn.
    Withdrawn,
}

impl ContinuityClaimQualificationClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Derives a qualification from the narrow reasons present.
    pub fn from_reasons<'a>(
        reasons: impl IntoIterator<Item = &'a ContinuityClaimNarrowReasonClass>,
    ) -> Self {
        let mut saw_any = false;
        let mut saw_preview = false;
        for reason in reasons {
            saw_any = true;
            if reason.is_withdrawal_reason() {
                return Self::Withdrawn;
            }
            if reason.is_preview_reason() {
                saw_preview = true;
            }
        }
        if saw_preview {
            Self::Preview
        } else if saw_any {
            Self::Beta
        } else {
            Self::Stable
        }
    }
}

/// Typed reason a continuity claim narrowed below stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityClaimNarrowReasonClass {
    /// No narrowing is active.
    NotNarrowed,
    /// Processing or storage locality, or residency, is not disclosed.
    LocalityUndisclosed,
    /// Tenant scope or key mode posture is missing.
    TenantKeyPostureMissing,
    /// The matrix does not distinguish control-plane from data-plane impairment.
    DegradedFallbackClassMissing,
    /// The matrix does not distinguish local-core from managed-lane continuity.
    ContinuityLaneDistinctionMissing,
    /// A managed-lane row does not name a managed continuity packet family or ref.
    ContinuityPacketFamilyMissing,
    /// A managed-lane row does not name a drill cadence or current/future owner.
    DrillCadenceOrOwnerMissing,
    /// Drill evidence is stale and a fresh drill is required.
    DrillEvidenceStale,
    /// The continuity drill has never been run.
    DrillNeverRun,
    /// A row with a managed continuity packet does not declare a restore identity.
    RestoreIdentityUndeclared,
    /// A row does not disclose partial-loss behavior.
    PartialLossUndisclosed,
    /// The claimed profile is inconsistent with its own posture or evidence.
    ProfileMismatch,
    /// A sovereign or self-hosted row hides a vendor-operated restore/failover lane.
    SovereignContinuityOverclaimed,
    /// A local-only row is marketed as managed continuity without a managed dependency.
    LocalOnlyOverclaimedAsManaged,
    /// A row's continuity facts are not reused across the required surfaces.
    SurfaceReuseIncomplete,
}

impl ContinuityClaimNarrowReasonClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::LocalityUndisclosed => "locality_undisclosed",
            Self::TenantKeyPostureMissing => "tenant_key_posture_missing",
            Self::DegradedFallbackClassMissing => "degraded_fallback_class_missing",
            Self::ContinuityLaneDistinctionMissing => "continuity_lane_distinction_missing",
            Self::ContinuityPacketFamilyMissing => "continuity_packet_family_missing",
            Self::DrillCadenceOrOwnerMissing => "drill_cadence_or_owner_missing",
            Self::DrillEvidenceStale => "drill_evidence_stale",
            Self::DrillNeverRun => "drill_never_run",
            Self::RestoreIdentityUndeclared => "restore_identity_undeclared",
            Self::PartialLossUndisclosed => "partial_loss_undisclosed",
            Self::ProfileMismatch => "profile_mismatch",
            Self::SovereignContinuityOverclaimed => "sovereign_continuity_overclaimed",
            Self::LocalOnlyOverclaimedAsManaged => "local_only_overclaimed_as_managed",
            Self::SurfaceReuseIncomplete => "surface_reuse_incomplete",
        }
    }

    /// True when this reason withdraws the claim immediately.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::SovereignContinuityOverclaimed)
    }

    /// True when this reason holds the claim at preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(
            self,
            Self::DrillNeverRun | Self::LocalOnlyOverclaimedAsManaged | Self::ProfileMismatch
        )
    }
}

/// Visibility declaration for the surfaces that must reuse continuity facts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimSurfaceVisibility {
    /// True when About reuses the fact.
    pub about: bool,
    /// True when Help reuses the fact.
    pub help: bool,
    /// True when service-health surfaces reuse the fact.
    pub service_health: bool,
    /// True when support exports reuse the fact.
    pub support_export: bool,
    /// True when docs / public-truth pages reuse the fact.
    pub docs_public_truth: bool,
    /// True when partner qualification packets reuse the fact.
    pub partner_qualification: bool,
}

impl ClaimSurfaceVisibility {
    /// Returns a declaration with every surface enabled.
    pub const fn all_required() -> Self {
        Self {
            about: true,
            help: true,
            service_health: true,
            support_export: true,
            docs_public_truth: true,
            partner_qualification: true,
        }
    }

    /// Returns a declaration covering only the in-product and public-truth surfaces.
    pub const fn local_core_required() -> Self {
        Self {
            about: true,
            help: true,
            service_health: true,
            support_export: false,
            docs_public_truth: true,
            partner_qualification: false,
        }
    }

    /// True when every surface is covered.
    pub const fn all_visible(&self) -> bool {
        self.about
            && self.help
            && self.service_health
            && self.support_export
            && self.docs_public_truth
            && self.partner_qualification
    }

    /// True when the in-product and public-truth surfaces are covered.
    pub const fn local_core_visible(&self) -> bool {
        self.about && self.help && self.service_health && self.docs_public_truth
    }
}

/// Processing and storage locality posture for a continuity-claim row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalityPosture {
    /// Where processing happens.
    pub processing_locality: LocalityClass,
    /// Stable token for [`Self::processing_locality`].
    pub processing_locality_token: String,
    /// Where durable storage lives.
    pub storage_locality: LocalityClass,
    /// Stable token for [`Self::storage_locality`].
    pub storage_locality_token: String,
    /// Export-safe region or residency label.
    pub residency_label: String,
}

impl LocalityPosture {
    /// True when locality is fully disclosed.
    pub fn is_disclosed(&self) -> bool {
        self.processing_locality != LocalityClass::Undisclosed
            && self.storage_locality != LocalityClass::Undisclosed
            && !self.residency_label.is_empty()
    }
}

/// Drill cadence, ownership, and evidence for a continuity-claim row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityDrill {
    /// Cadence at which the continuity packet is drilled.
    pub cadence: DrillCadenceClass,
    /// Stable token for [`Self::cadence`].
    pub cadence_token: String,
    /// Freshness state of the drill evidence.
    pub evidence_state: DrillEvidenceStateClass,
    /// Stable token for [`Self::evidence_state`].
    pub evidence_state_token: String,
    /// Export-safe label naming the current drill owner.
    pub current_owner_label: String,
    /// Export-safe label naming the future or backup drill owner.
    pub future_owner_label: String,
    /// UTC timestamp of the last successful drill, empty when never run.
    pub last_drill_at: String,
    /// Opaque ref to the drill evidence or continuity packet.
    pub drill_packet_ref: String,
}

/// One continuity-claim row for a claimed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityClaimRow {
    /// Opaque row identifier.
    pub row_id: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Claimed deployment profile.
    pub profile_class: ContinuityProfileClass,
    /// Stable token for [`Self::profile_class`].
    pub profile_class_token: String,
    /// True when this row actually depends on a claimed managed or self-hosted lane.
    pub has_claimed_managed_dependency: bool,
    /// Continuity lane this row belongs to.
    pub continuity_lane: ContinuityLaneClass,
    /// Stable token for [`Self::continuity_lane`].
    pub continuity_lane_token: String,
    /// Processing and storage locality posture.
    pub locality: LocalityPosture,
    /// Tenant or org boundary.
    pub tenant_scope: TenantScopeClass,
    /// Stable token for [`Self::tenant_scope`].
    pub tenant_scope_token: String,
    /// Key-mode posture protecting durable state.
    pub key_mode: KeyModeClass,
    /// Stable token for [`Self::key_mode`].
    pub key_mode_token: String,
    /// Plane(s) the degraded fallback addresses.
    pub degraded_fallback_class: PlaneImpairmentClass,
    /// Stable token for [`Self::degraded_fallback_class`].
    pub degraded_fallback_class_token: String,
    /// Named continuity packet family backing the claim.
    pub continuity_packet_family: ContinuityPacketFamilyClass,
    /// Stable token for [`Self::continuity_packet_family`].
    pub continuity_packet_family_token: String,
    /// Opaque ref to the continuity packet or evidence.
    pub continuity_packet_ref: String,
    /// Where the restore or failover path executes.
    pub restore_failover_hosting: RestoreFailoverHostingClass,
    /// Stable token for [`Self::restore_failover_hosting`].
    pub restore_failover_hosting_token: String,
    /// True when any external restore/failover dependency is disclosed.
    pub external_dependency_disclosed: bool,
    /// Identity a successful restore reproduces.
    pub restore_identity: RestoreIdentityClass,
    /// Stable token for [`Self::restore_identity`].
    pub restore_identity_token: String,
    /// Partial-loss disclosure class.
    pub partial_loss: PartialLossClass,
    /// Stable token for [`Self::partial_loss`].
    pub partial_loss_token: String,
    /// Export-safe note describing the partial-loss boundary.
    pub partial_loss_note: String,
    /// Drill cadence, ownership, and evidence.
    pub drill: ContinuityDrill,
    /// Required surface coverage for this row.
    pub surface_visibility: ClaimSurfaceVisibility,
}

impl ContinuityClaimRow {
    /// True when this row sits inside managed continuity scope.
    ///
    /// Managed, self-hosted, and sovereign profiles are always in scope, as is
    /// any row on the managed continuity lane or carrying a claimed managed
    /// dependency. A pure local-only row with no claimed managed dependency is
    /// out of managed continuity scope and is not held to managed-lane
    /// requirements.
    pub fn in_managed_scope(&self) -> bool {
        self.profile_class != ContinuityProfileClass::LocalOnly
            || self.continuity_lane == ContinuityLaneClass::ManagedLane
            || self.has_claimed_managed_dependency
    }
}

/// Per-row verdict joining a row to its computed qualification and reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityClaimRowOutcome {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque row identifier this outcome describes.
    pub row_id: String,
    /// Stable token for the row's claimed profile.
    pub profile_class_token: String,
    /// True when the row is in managed continuity scope.
    pub in_managed_scope: bool,
    /// Computed qualification token for the row.
    pub qualification_token: String,
    /// True when the row narrowed below stable.
    pub narrowed: bool,
    /// True when the row's claim is withheld entirely.
    pub claim_withheld: bool,
    /// Stable narrow-reason tokens that applied to the row.
    pub narrow_reason_tokens: Vec<String>,
}

/// Consolidated drill-schedule entry for one continuity packet family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillScheduleEntry {
    /// Continuity packet family.
    pub packet_family: ContinuityPacketFamilyClass,
    /// Stable token for [`Self::packet_family`].
    pub packet_family_token: String,
    /// Number of rows covered by this family.
    pub row_count: usize,
    /// Distinct cadence tokens present in this family.
    pub cadence_tokens: Vec<String>,
    /// Distinct current drill owner labels in this family.
    pub current_owner_labels: Vec<String>,
    /// Distinct future drill owner labels in this family.
    pub future_owner_labels: Vec<String>,
    /// Distinct drill-evidence state tokens in this family.
    pub evidence_state_tokens: Vec<String>,
    /// Number of rows in this family that need a fresh drill.
    pub needs_drill_row_count: usize,
}

/// Full auditable input for the continuity-claim matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityClaimMatrixInput {
    /// Reviewable label for the matrix.
    pub matrix_label: String,
    /// Claimed continuity rows.
    pub claim_rows: Vec<ContinuityClaimRow>,
}

/// Aggregate summary for a continuity-claim matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityClaimMatrixSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Overall qualification for the matrix.
    pub overall_qualification_token: String,
    /// Number of claim rows.
    pub claim_row_count: usize,
    /// Number of rows in managed continuity scope.
    pub managed_scope_row_count: usize,
    /// Number of rows on the local-core continuity lane.
    pub local_core_row_count: usize,
    /// Number of rows on the managed continuity lane.
    pub managed_lane_row_count: usize,
    /// Number of rows whose degraded fallback covers the control plane.
    pub control_plane_impairment_row_count: usize,
    /// Number of rows whose degraded fallback covers the data plane.
    pub data_plane_impairment_row_count: usize,
    /// Number of rows that narrowed below stable.
    pub narrowed_row_count: usize,
    /// Number of rows whose claim is withheld.
    pub withdrawn_row_count: usize,
    /// Number of distinct continuity packet families in the drill schedule.
    pub drill_family_count: usize,
    /// Number of rows that need a fresh drill.
    pub needs_drill_row_count: usize,
    /// Number of defects recorded for the matrix.
    pub defect_count: usize,
}

/// Typed defect emitted by the continuity-claim audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityClaimDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque defect identifier.
    pub defect_id: String,
    /// Typed narrow reason.
    pub narrow_reason: ContinuityClaimNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Opaque source row id or matrix concern that triggered the defect.
    pub source: String,
    /// Export-safe explanation of the defect.
    pub note: String,
}

impl ContinuityClaimDefect {
    fn new(
        narrow_reason: ContinuityClaimNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: CONTINUITY_CLAIM_DEFECT_RECORD_KIND.to_owned(),
            schema_version: CONTINUITY_CLAIM_MATRIX_SCHEMA_VERSION,
            shared_contract_ref: CONTINUITY_CLAIM_MATRIX_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "continuity:defect:claim-matrix:{}:{}",
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

/// Canonical proof packet for the locality, tenant/key-mode, and drill matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityClaimMatrixPage {
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
    pub summary: ContinuityClaimMatrixSummary,
    /// Typed defects for the packet.
    pub defects: Vec<ContinuityClaimDefect>,
    /// Per-row verdicts joining each row to its computed qualification.
    pub row_outcomes: Vec<ContinuityClaimRowOutcome>,
    /// Consolidated drill schedule grouped by continuity packet family.
    pub drill_schedule: Vec<DrillScheduleEntry>,
    /// The audited input embedded as evidence.
    pub input: ContinuityClaimMatrixInput,
}

impl ContinuityClaimMatrixPage {
    /// Builds a continuity-claim matrix page from the supplied input.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: ContinuityClaimMatrixInput,
    ) -> Self {
        let defects = audit_continuity_claim_matrix_input(&input);
        let row_outcomes = build_row_outcomes(&input, &defects);
        let drill_schedule = build_drill_schedule(&input);
        let summary = build_summary(&input, &row_outcomes, &drill_schedule, &defects);
        Self {
            record_kind: CONTINUITY_CLAIM_MATRIX_PAGE_RECORD_KIND.to_owned(),
            schema_version: CONTINUITY_CLAIM_MATRIX_SCHEMA_VERSION,
            shared_contract_ref: CONTINUITY_CLAIM_MATRIX_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            defects,
            row_outcomes,
            drill_schedule,
            input,
        }
    }

    /// True when the matrix qualifies stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == ContinuityClaimQualificationClass::Stable.as_str()
    }

    /// True when the matrix distinguishes control-plane from data-plane impairment.
    pub fn distinguishes_control_and_data_plane(&self) -> bool {
        plane_distinction_is_complete(&self.input.claim_rows)
    }

    /// True when the matrix distinguishes local-core from managed-lane continuity.
    pub fn distinguishes_local_core_and_managed_lane(&self) -> bool {
        lane_distinction_is_complete(&self.input.claim_rows)
    }

    /// True when every managed-scope row names a current and future drill owner.
    pub fn managed_rows_have_named_drill_owners(&self) -> bool {
        self.input
            .claim_rows
            .iter()
            .filter(|row| row.in_managed_scope())
            .all(|row| {
                !row.drill.current_owner_label.is_empty()
                    && !row.drill.future_owner_label.is_empty()
            })
    }

    /// Returns the computed outcome for a row id, if present.
    pub fn row_outcome(&self, row_id: &str) -> Option<&ContinuityClaimRowOutcome> {
        self.row_outcomes
            .iter()
            .find(|outcome| outcome.row_id == row_id)
    }
}

/// Support-export wrapper for the continuity-claim matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityClaimMatrixSupportExport {
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
    /// The continuity-claim matrix packet embedded as evidence.
    pub page: ContinuityClaimMatrixPage,
    /// Typed narrow reasons present in the embedded packet.
    pub narrow_reasons_present: Vec<ContinuityClaimNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl ContinuityClaimMatrixSupportExport {
    /// Wraps a continuity-claim matrix page inside a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: ContinuityClaimMatrixPage,
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
        Self {
            record_kind: CONTINUITY_CLAIM_MATRIX_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: CONTINUITY_CLAIM_MATRIX_SCHEMA_VERSION,
            shared_contract_ref: CONTINUITY_CLAIM_MATRIX_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

/// Re-runs the continuity-claim audit over the embedded input.
pub fn audit_continuity_claim_matrix_page(
    page: &ContinuityClaimMatrixPage,
) -> Vec<ContinuityClaimDefect> {
    audit_continuity_claim_matrix_input(&page.input)
}

/// Validates a matrix page and returns `Ok(())` when the audit is clean.
pub fn validate_continuity_claim_matrix_page(
    page: &ContinuityClaimMatrixPage,
) -> Result<(), Vec<ContinuityClaimDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

/// Returns the seeded stable continuity-claim matrix page.
pub fn seeded_continuity_claim_matrix_page() -> ContinuityClaimMatrixPage {
    ContinuityClaimMatrixPage::new(
        "continuity:claim-matrix:seeded",
        "Locality, tenant/key-mode, and continuity-drill matrix",
        "2026-06-01T00:00:00Z",
        seeded_continuity_claim_matrix_input(),
    )
}

/// Returns the seeded input used by the canonical matrix page.
pub fn seeded_continuity_claim_matrix_input() -> ContinuityClaimMatrixInput {
    ContinuityClaimMatrixInput {
        matrix_label: "Claimed managed, self-hosted, and sovereign continuity rows".to_owned(),
        claim_rows: seeded_claim_rows(),
    }
}

fn audit_continuity_claim_matrix_input(
    input: &ContinuityClaimMatrixInput,
) -> Vec<ContinuityClaimDefect> {
    let mut defects = Vec::new();

    for row in &input.claim_rows {
        audit_row(row, &mut defects);
    }

    if !input.claim_rows.is_empty() {
        if !plane_distinction_is_complete(&input.claim_rows) {
            defects.push(ContinuityClaimDefect::new(
                ContinuityClaimNarrowReasonClass::DegradedFallbackClassMissing,
                "matrix:degraded_fallback",
                "the matrix must classify at least one control-plane impairment row and one data-plane impairment row",
            ));
        }
        if !lane_distinction_is_complete(&input.claim_rows) {
            defects.push(ContinuityClaimDefect::new(
                ContinuityClaimNarrowReasonClass::ContinuityLaneDistinctionMissing,
                "matrix:continuity_lane",
                "the matrix must distinguish local-core continuity from managed-lane continuity",
            ));
        }
    }

    defects
}

fn audit_row(row: &ContinuityClaimRow, defects: &mut Vec<ContinuityClaimDefect>) {
    // Locality and partial-loss disclosure apply to every row.
    if !row.locality.is_disclosed() {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::LocalityUndisclosed,
            row.row_id.clone(),
            "every claimed row must disclose processing locality, storage locality, and a residency label",
        ));
    }
    if row.partial_loss == PartialLossClass::Undisclosed {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::PartialLossUndisclosed,
            row.row_id.clone(),
            "every claimed row must disclose its partial-loss behavior on recovery",
        ));
    }

    // Guardrail: a local-only row may not be marketed as managed continuity
    // unless it actually carries a claimed managed dependency.
    if row.profile_class == ContinuityProfileClass::LocalOnly
        && !row.has_claimed_managed_dependency
        && (row.continuity_lane == ContinuityLaneClass::ManagedLane
            || row.continuity_packet_family.is_managed_family())
    {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::LocalOnlyOverclaimedAsManaged,
            row.row_id.clone(),
            "a local-only row may not claim managed continuity scope without a claimed managed or self-hosted dependency",
        ));
    }

    if !row.in_managed_scope() {
        // Out-of-scope local rows still owe in-product and public-truth reuse.
        if !row.surface_visibility.local_core_visible() {
            defects.push(ContinuityClaimDefect::new(
                ContinuityClaimNarrowReasonClass::SurfaceReuseIncomplete,
                row.row_id.clone(),
                "local-core continuity rows must still reach About, Help, service-health, and docs/public-truth surfaces",
            ));
        }
        return;
    }

    // Tenant and key-mode posture must be explicit for managed-scope rows.
    if row.tenant_scope == TenantScopeClass::NotApplicable
        || row.key_mode == KeyModeClass::NotApplicable
    {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::TenantKeyPostureMissing,
            row.row_id.clone(),
            "managed, self-hosted, and sovereign rows must declare an explicit tenant scope and key mode",
        ));
    }

    // Continuity packet family and reference.
    if row.continuity_lane == ContinuityLaneClass::ManagedLane
        && !row.continuity_packet_family.is_managed_family()
    {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::ContinuityPacketFamilyMissing,
            row.row_id.clone(),
            "managed-lane rows must name a backup, restore, failover, or snapshot continuity packet family",
        ));
    }
    if row.continuity_packet_ref.is_empty() {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::ContinuityPacketFamilyMissing,
            row.row_id.clone(),
            "managed-scope rows must reference the continuity packet or evidence backing the claim",
        ));
    }

    // Drill cadence and ownership.
    if row.drill.current_owner_label.is_empty()
        || row.drill.future_owner_label.is_empty()
        || (row.continuity_lane == ContinuityLaneClass::ManagedLane
            && row.drill.cadence == DrillCadenceClass::OnDemandOnly)
    {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::DrillCadenceOrOwnerMissing,
            row.row_id.clone(),
            "managed-scope rows must name a drill cadence and both a current and future drill owner",
        ));
    }

    // Drill evidence freshness.
    match row.drill.evidence_state {
        DrillEvidenceStateClass::NeverRun => defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::DrillNeverRun,
            row.row_id.clone(),
            "the continuity drill has never been run; the claim cannot exceed preview",
        )),
        DrillEvidenceStateClass::StaleNeedsDrill => defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::DrillEvidenceStale,
            row.row_id.clone(),
            "drill evidence is stale and a fresh drill is required before the claim stays stable",
        )),
        state if state.requires_last_drill_timestamp() && row.drill.last_drill_at.is_empty() => {
            defects.push(ContinuityClaimDefect::new(
                ContinuityClaimNarrowReasonClass::DrillEvidenceStale,
                row.row_id.clone(),
                "current or graced drill evidence must record a last-drill timestamp",
            ));
        }
        _ => {}
    }

    // Restore identity must be declared for managed continuity families.
    if row.continuity_packet_family.requires_restore_identity()
        && row.restore_identity == RestoreIdentityClass::NotApplicable
    {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::RestoreIdentityUndeclared,
            row.row_id.clone(),
            "rows backed by a managed continuity packet must declare the restore identity recovery reproduces",
        ));
    }

    // Hard guardrail: a self-governed boundary may not hide a vendor-operated
    // restore/failover lane.
    if row.profile_class.is_self_governed()
        && row.restore_failover_hosting == RestoreFailoverHostingClass::VendorOperated
        && !row.external_dependency_disclosed
    {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::SovereignContinuityOverclaimed,
            row.row_id.clone(),
            "a self-hosted or sovereign row may not hide a vendor-operated restore or failover lane",
        ));
    }

    // Profile-vs-posture mismatches.
    if row.profile_class == ContinuityProfileClass::Sovereign
        && row.tenant_scope == TenantScopeClass::SharedMultiTenant
    {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::ProfileMismatch,
            row.row_id.clone(),
            "a sovereign row cannot claim a shared multi-tenant boundary",
        ));
    }
    if row.profile_class.is_self_governed() && row.key_mode == KeyModeClass::VendorManagedKeys {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::ProfileMismatch,
            row.row_id.clone(),
            "a self-hosted or sovereign row cannot rely on vendor-managed keys",
        ));
    }
    if row.profile_class == ContinuityProfileClass::Managed
        && row.restore_failover_hosting == RestoreFailoverHostingClass::LocalCore
    {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::ProfileMismatch,
            row.row_id.clone(),
            "a managed row cannot claim purely local-core restore or failover continuity",
        ));
    }

    // Surface reuse across every required surface.
    if !row.surface_visibility.all_visible() {
        defects.push(ContinuityClaimDefect::new(
            ContinuityClaimNarrowReasonClass::SurfaceReuseIncomplete,
            row.row_id.clone(),
            "managed-scope row facts must be reused by About, Help, service-health, support exports, docs/public-truth, and partner qualification packets",
        ));
    }
}

fn plane_distinction_is_complete(rows: &[ContinuityClaimRow]) -> bool {
    let has_control = rows
        .iter()
        .any(|row| row.degraded_fallback_class.covers_control_plane());
    let has_data = rows
        .iter()
        .any(|row| row.degraded_fallback_class.covers_data_plane());
    has_control && has_data
}

fn lane_distinction_is_complete(rows: &[ContinuityClaimRow]) -> bool {
    let has_local_core = rows
        .iter()
        .any(|row| row.continuity_lane == ContinuityLaneClass::LocalCore);
    let has_managed_lane = rows
        .iter()
        .any(|row| row.continuity_lane == ContinuityLaneClass::ManagedLane);
    has_local_core && has_managed_lane
}

fn build_row_outcomes(
    input: &ContinuityClaimMatrixInput,
    defects: &[ContinuityClaimDefect],
) -> Vec<ContinuityClaimRowOutcome> {
    input
        .claim_rows
        .iter()
        .map(|row| {
            let reasons: Vec<ContinuityClaimNarrowReasonClass> = defects
                .iter()
                .filter(|defect| defect.source == row.row_id)
                .map(|defect| defect.narrow_reason)
                .collect();
            let qualification = ContinuityClaimQualificationClass::from_reasons(reasons.iter());
            let mut reason_tokens: Vec<String> = reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect();
            reason_tokens.sort();
            reason_tokens.dedup();
            ContinuityClaimRowOutcome {
                record_kind: CONTINUITY_CLAIM_ROW_OUTCOME_RECORD_KIND.to_owned(),
                schema_version: CONTINUITY_CLAIM_MATRIX_SCHEMA_VERSION,
                shared_contract_ref: CONTINUITY_CLAIM_MATRIX_SHARED_CONTRACT_REF.to_owned(),
                row_id: row.row_id.clone(),
                profile_class_token: row.profile_class.as_str().to_owned(),
                in_managed_scope: row.in_managed_scope(),
                qualification_token: qualification.as_str().to_owned(),
                narrowed: qualification != ContinuityClaimQualificationClass::Stable,
                claim_withheld: qualification == ContinuityClaimQualificationClass::Withdrawn,
                narrow_reason_tokens: reason_tokens,
            }
        })
        .collect()
}

fn build_drill_schedule(input: &ContinuityClaimMatrixInput) -> Vec<DrillScheduleEntry> {
    let mut by_family: BTreeMap<ContinuityPacketFamilyClass, Vec<&ContinuityClaimRow>> =
        BTreeMap::new();
    for row in &input.claim_rows {
        by_family
            .entry(row.continuity_packet_family)
            .or_default()
            .push(row);
    }

    by_family
        .into_iter()
        .map(|(packet_family, rows)| {
            let cadence_tokens =
                sorted_unique(rows.iter().map(|row| row.drill.cadence.as_str().to_owned()));
            let current_owner_labels = sorted_unique(
                rows.iter()
                    .map(|row| row.drill.current_owner_label.clone())
                    .filter(|label| !label.is_empty()),
            );
            let future_owner_labels = sorted_unique(
                rows.iter()
                    .map(|row| row.drill.future_owner_label.clone())
                    .filter(|label| !label.is_empty()),
            );
            let evidence_state_tokens = sorted_unique(
                rows.iter()
                    .map(|row| row.drill.evidence_state.as_str().to_owned()),
            );
            let needs_drill_row_count = rows
                .iter()
                .filter(|row| row.drill.evidence_state.needs_drill())
                .count();
            DrillScheduleEntry {
                packet_family,
                packet_family_token: packet_family.as_str().to_owned(),
                row_count: rows.len(),
                cadence_tokens,
                current_owner_labels,
                future_owner_labels,
                evidence_state_tokens,
                needs_drill_row_count,
            }
        })
        .collect()
}

fn build_summary(
    input: &ContinuityClaimMatrixInput,
    row_outcomes: &[ContinuityClaimRowOutcome],
    drill_schedule: &[DrillScheduleEntry],
    defects: &[ContinuityClaimDefect],
) -> ContinuityClaimMatrixSummary {
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

    ContinuityClaimMatrixSummary {
        record_kind: CONTINUITY_CLAIM_MATRIX_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: CONTINUITY_CLAIM_MATRIX_SCHEMA_VERSION,
        shared_contract_ref: CONTINUITY_CLAIM_MATRIX_SHARED_CONTRACT_REF.to_owned(),
        overall_qualification_token: overall.as_str().to_owned(),
        claim_row_count: input.claim_rows.len(),
        managed_scope_row_count: input
            .claim_rows
            .iter()
            .filter(|row| row.in_managed_scope())
            .count(),
        local_core_row_count: input
            .claim_rows
            .iter()
            .filter(|row| row.continuity_lane == ContinuityLaneClass::LocalCore)
            .count(),
        managed_lane_row_count: input
            .claim_rows
            .iter()
            .filter(|row| row.continuity_lane == ContinuityLaneClass::ManagedLane)
            .count(),
        control_plane_impairment_row_count: input
            .claim_rows
            .iter()
            .filter(|row| row.degraded_fallback_class.covers_control_plane())
            .count(),
        data_plane_impairment_row_count: input
            .claim_rows
            .iter()
            .filter(|row| row.degraded_fallback_class.covers_data_plane())
            .count(),
        narrowed_row_count: row_outcomes
            .iter()
            .filter(|outcome| outcome.narrowed)
            .count(),
        withdrawn_row_count: row_outcomes
            .iter()
            .filter(|outcome| outcome.claim_withheld)
            .count(),
        drill_family_count: drill_schedule.len(),
        needs_drill_row_count: input
            .claim_rows
            .iter()
            .filter(|row| row.drill.evidence_state.needs_drill())
            .count(),
        defect_count: defects.len(),
    }
}

fn sorted_unique(values: impl IntoIterator<Item = String>) -> Vec<String> {
    let set: BTreeSet<String> = values.into_iter().collect();
    set.into_iter().collect()
}

fn seeded_claim_rows() -> Vec<ContinuityClaimRow> {
    vec![
        claim_row(
            "continuity-row:managed-cloud-sync",
            "Managed cloud workspace sync and backup",
            ContinuityProfileClass::Managed,
            true,
            ContinuityLaneClass::ManagedLane,
            locality(
                LocalityClass::SingleRegion,
                LocalityClass::SingleRegion,
                "us-west managed region",
            ),
            TenantScopeClass::SharedMultiTenant,
            KeyModeClass::VendorManagedKeys,
            PlaneImpairmentClass::ControlPlaneImpairment,
            ContinuityPacketFamilyClass::Backup,
            "continuity-packet:managed-cloud:backup",
            RestoreFailoverHostingClass::VendorOperated,
            true,
            RestoreIdentityClass::SameIdentityRestore,
            PartialLossClass::BoundedRecentWindowLoss,
            "A bounded window of unsynced edits since the last backup may need a local replay.",
            drill(
                DrillCadenceClass::PerRelease,
                DrillEvidenceStateClass::Current,
                "Managed platform on-call",
                "Reliability guild",
                "2026-06-01T00:00:00Z",
                "drill:managed-cloud:backup:2026-06-01",
            ),
            ClaimSurfaceVisibility::all_required(),
        ),
        claim_row(
            "continuity-row:managed-relay-failover",
            "Managed relay and collaboration failover",
            ContinuityProfileClass::Managed,
            true,
            ContinuityLaneClass::ManagedLane,
            locality(
                LocalityClass::MultiRegion,
                LocalityClass::MultiRegion,
                "us-west and us-east managed regions",
            ),
            TenantScopeClass::DedicatedTenant,
            KeyModeClass::VendorManagedKeys,
            PlaneImpairmentClass::DataPlaneImpairment,
            ContinuityPacketFamilyClass::Failover,
            "continuity-packet:managed-relay:failover",
            RestoreFailoverHostingClass::MirrorBacked,
            true,
            RestoreIdentityClass::SameIdentityRestore,
            PartialLossClass::QueuedActionLoss,
            "In-flight relay actions may need replay; durable workspace state is unaffected.",
            drill(
                DrillCadenceClass::Quarterly,
                DrillEvidenceStateClass::Current,
                "Managed platform on-call",
                "Reliability guild",
                "2026-05-20T00:00:00Z",
                "drill:managed-relay:failover:2026-05-20",
            ),
            ClaimSurfaceVisibility::all_required(),
        ),
        claim_row(
            "continuity-row:self-hosted-restore",
            "Customer self-hosted restore and rebuild",
            ContinuityProfileClass::SelfHosted,
            true,
            ContinuityLaneClass::ManagedLane,
            locality(
                LocalityClass::CustomerRegion,
                LocalityClass::CustomerRegion,
                "customer-operated eu-central region",
            ),
            TenantScopeClass::CustomerTenant,
            KeyModeClass::CustomerManagedKeys,
            PlaneImpairmentClass::ControlPlaneImpairment,
            ContinuityPacketFamilyClass::Restore,
            "continuity-packet:self-hosted:restore",
            RestoreFailoverHostingClass::CustomerOperated,
            true,
            RestoreIdentityClass::ReissuedIdentityRestore,
            PartialLossClass::BoundedRecentWindowLoss,
            "Restore reissues service identity; operators must re-trust the reissued identity once.",
            drill(
                DrillCadenceClass::Semiannual,
                DrillEvidenceStateClass::ReconstructableFromSnapshot,
                "Customer success SRE",
                "Field reliability owner",
                "",
                "continuity-packet:self-hosted:restore-runbook",
            ),
            ClaimSurfaceVisibility::all_required(),
        ),
        claim_row(
            "continuity-row:sovereign-airgap-snapshot",
            "Sovereign air-gapped snapshot and replication",
            ContinuityProfileClass::Sovereign,
            true,
            ContinuityLaneClass::ManagedLane,
            locality(
                LocalityClass::InCountrySovereign,
                LocalityClass::AirGappedIsolated,
                "isolated in-country customer network",
            ),
            TenantScopeClass::CustomerTenant,
            KeyModeClass::CustomerHeldRoot,
            PlaneImpairmentClass::BothPlanes,
            ContinuityPacketFamilyClass::SnapshotReplication,
            "continuity-packet:sovereign:snapshot",
            RestoreFailoverHostingClass::OfflineSnapshot,
            true,
            RestoreIdentityClass::NewInstallRebind,
            PartialLossClass::CacheOnlyLoss,
            "Recovery rebinds a new install from the last signed snapshot; only cache is lost.",
            drill(
                DrillCadenceClass::Annual,
                DrillEvidenceStateClass::StaleWithinGrace,
                "Sovereign operations lead",
                "Customer compliance owner",
                "2026-05-15T00:00:00Z",
                "drill:sovereign:snapshot:2026-05-15",
            ),
            ClaimSurfaceVisibility::all_required(),
        ),
        claim_row(
            "continuity-row:local-desktop-core",
            "Local desktop core continuity",
            ContinuityProfileClass::LocalOnly,
            false,
            ContinuityLaneClass::LocalCore,
            locality(
                LocalityClass::DeviceLocal,
                LocalityClass::DeviceLocal,
                "device-local",
            ),
            TenantScopeClass::SingleUserLocal,
            KeyModeClass::LocalOsKeystore,
            PlaneImpairmentClass::DataPlaneImpairment,
            ContinuityPacketFamilyClass::LocalCoreContinuity,
            "continuity-packet:local-core:autosave",
            RestoreFailoverHostingClass::LocalCore,
            true,
            RestoreIdentityClass::NotApplicable,
            PartialLossClass::NoPartialLoss,
            "Local autosave and Git keep durable edits; no managed lane is claimed.",
            drill(
                DrillCadenceClass::OnDemandOnly,
                DrillEvidenceStateClass::ReconstructableFromSnapshot,
                "Local user",
                "Local user",
                "",
                "continuity-packet:local-core:autosave",
            ),
            ClaimSurfaceVisibility::local_core_required(),
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn claim_row(
    row_id: &str,
    surface_label: &str,
    profile_class: ContinuityProfileClass,
    has_claimed_managed_dependency: bool,
    continuity_lane: ContinuityLaneClass,
    locality: LocalityPosture,
    tenant_scope: TenantScopeClass,
    key_mode: KeyModeClass,
    degraded_fallback_class: PlaneImpairmentClass,
    continuity_packet_family: ContinuityPacketFamilyClass,
    continuity_packet_ref: &str,
    restore_failover_hosting: RestoreFailoverHostingClass,
    external_dependency_disclosed: bool,
    restore_identity: RestoreIdentityClass,
    partial_loss: PartialLossClass,
    partial_loss_note: &str,
    drill: ContinuityDrill,
    surface_visibility: ClaimSurfaceVisibility,
) -> ContinuityClaimRow {
    ContinuityClaimRow {
        row_id: row_id.to_owned(),
        surface_label: surface_label.to_owned(),
        profile_class,
        profile_class_token: profile_class.as_str().to_owned(),
        has_claimed_managed_dependency,
        continuity_lane,
        continuity_lane_token: continuity_lane.as_str().to_owned(),
        locality,
        tenant_scope,
        tenant_scope_token: tenant_scope.as_str().to_owned(),
        key_mode,
        key_mode_token: key_mode.as_str().to_owned(),
        degraded_fallback_class,
        degraded_fallback_class_token: degraded_fallback_class.as_str().to_owned(),
        continuity_packet_family,
        continuity_packet_family_token: continuity_packet_family.as_str().to_owned(),
        continuity_packet_ref: continuity_packet_ref.to_owned(),
        restore_failover_hosting,
        restore_failover_hosting_token: restore_failover_hosting.as_str().to_owned(),
        external_dependency_disclosed,
        restore_identity,
        restore_identity_token: restore_identity.as_str().to_owned(),
        partial_loss,
        partial_loss_token: partial_loss.as_str().to_owned(),
        partial_loss_note: partial_loss_note.to_owned(),
        drill,
        surface_visibility,
    }
}

fn locality(
    processing_locality: LocalityClass,
    storage_locality: LocalityClass,
    residency_label: &str,
) -> LocalityPosture {
    LocalityPosture {
        processing_locality,
        processing_locality_token: processing_locality.as_str().to_owned(),
        storage_locality,
        storage_locality_token: storage_locality.as_str().to_owned(),
        residency_label: residency_label.to_owned(),
    }
}

fn drill(
    cadence: DrillCadenceClass,
    evidence_state: DrillEvidenceStateClass,
    current_owner_label: &str,
    future_owner_label: &str,
    last_drill_at: &str,
    drill_packet_ref: &str,
) -> ContinuityDrill {
    ContinuityDrill {
        cadence,
        cadence_token: cadence.as_str().to_owned(),
        evidence_state,
        evidence_state_token: evidence_state.as_str().to_owned(),
        current_owner_label: current_owner_label.to_owned(),
        future_owner_label: future_owner_label.to_owned(),
        last_drill_at: last_drill_at.to_owned(),
        drill_packet_ref: drill_packet_ref.to_owned(),
    }
}
