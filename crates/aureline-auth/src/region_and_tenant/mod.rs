//! Region-pinning, tenant-boundary, and key-mode truth with first beta drills.
//!
//! This module owns the beta projection that turns claimed managed and
//! enterprise deployment promises into operational truth. Each row names one
//! managed action lane, discloses processing region, tenant boundary, and
//! key-mode posture, and points at the drill packet that exercised the failure
//! or failover scenario tied to that lane. The same record kind is consumed by
//! admin, support, shell, settings, headless, docs, and reviewer surfaces so
//! enterprise pilots see region, tenant, and key-mode state from one auditable
//! page across connected, mirror-only, offline, and enterprise-managed beta
//! profiles.
//!
//! Surfaces never re-derive "is_pinned" or "tenant_ok" from local hints. They
//! consume the seeded page from [`seeded_region_tenant_key_mode_beta_page`]
//! and the support-export wrapper preserves the typed defect vocabulary
//! verbatim while excluding raw private material.
//!
//! Guardrails: a mismatch on a single managed lane narrows authority on that
//! lane only and never widens authority for a sibling lane. The vocabulary
//! refuses undeclared public-endpoint fallback and plaintext key material.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::identity_modes::{KeyMode, RegionMode, ResidencyMode};
use crate::trust::CapabilityAuthorityClass;

/// Beta schema version exported with every region, tenant, key-mode, and drill
/// record.
pub const REGION_TENANT_KEY_MODE_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every region, tenant, key-mode, and
/// drill record.
pub const REGION_TENANT_KEY_MODE_BETA_SHARED_CONTRACT_REF: &str =
    "security:region_tenant_key_mode_beta:v1";

/// Source matrix ref consumed by this beta projection.
pub const REGION_TENANT_KEY_MODE_BETA_SOURCE_MATRIX_REF: &str =
    "artifacts/security/m3/region_tenant_drills/region_tenant_key_mode_matrix.yaml";

/// Stable record kind for [`RegionTenantKeyModeBetaPage`] payloads.
pub const REGION_TENANT_KEY_MODE_BETA_PAGE_RECORD_KIND: &str =
    "security_region_tenant_key_mode_beta_page_record";

/// Stable record kind for [`RegionDisclosureRow`] payloads.
pub const REGION_DISCLOSURE_ROW_RECORD_KIND: &str =
    "security_region_tenant_key_mode_beta_region_disclosure_row_record";

/// Stable record kind for [`TenantBoundaryRow`] payloads.
pub const TENANT_BOUNDARY_ROW_RECORD_KIND: &str =
    "security_region_tenant_key_mode_beta_tenant_boundary_row_record";

/// Stable record kind for [`KeyModeRow`] payloads.
pub const KEY_MODE_ROW_RECORD_KIND: &str =
    "security_region_tenant_key_mode_beta_key_mode_row_record";

/// Stable record kind for [`RegionTenantDrillPacket`] payloads.
pub const REGION_TENANT_DRILL_PACKET_RECORD_KIND: &str =
    "security_region_tenant_key_mode_beta_drill_packet_record";

/// Stable record kind for [`RegionTenantKeyModeBetaDefect`] payloads.
pub const REGION_TENANT_KEY_MODE_BETA_DEFECT_RECORD_KIND: &str =
    "security_region_tenant_key_mode_beta_defect_record";

/// Stable record kind for [`RegionTenantKeyModeBetaSummary`] payloads.
pub const REGION_TENANT_KEY_MODE_BETA_SUMMARY_RECORD_KIND: &str =
    "security_region_tenant_key_mode_beta_summary_record";

/// Stable record kind for [`RegionTenantKeyModeBetaSupportExport`] payloads.
pub const REGION_TENANT_KEY_MODE_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "security_region_tenant_key_mode_beta_support_export_record";

/// Profile under which a region, tenant, key-mode, or drill row is inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionTenantKeyModeBetaProfileClass {
    /// Connected beta profile with live managed control plane reachable.
    Connected,
    /// Mirror-only profile served from a declared signed mirror.
    MirrorOnly,
    /// Offline profile served from a last-known-good or air-gapped snapshot.
    Offline,
    /// Enterprise-managed profile applying signed managed narrowing.
    EnterpriseManaged,
}

impl RegionTenantKeyModeBetaProfileClass {
    /// All required beta profiles in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::MirrorOnly,
        Self::Offline,
        Self::EnterpriseManaged,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
            Self::EnterpriseManaged => "enterprise_managed",
        }
    }
}

/// Managed action lane to which a row applies. Mismatch on one lane never
/// silently widens authority on a sibling lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedActionLaneClass {
    /// Managed AI inference and tool-call lane.
    AiInference,
    /// Managed provider tool-call lane (registries, databases, remote build).
    ProviderToolCall,
    /// Managed remote attach / debug-host lane.
    RemoteAttach,
    /// Managed mirror or content-sync lane.
    MirrorSync,
    /// Managed support-export upload lane.
    SupportExportUpload,
    /// Managed admin policy-push lane.
    AdminPolicyPush,
}

impl ManagedActionLaneClass {
    /// All managed action lanes in canonical order.
    pub const ALL: [Self; 6] = [
        Self::AiInference,
        Self::ProviderToolCall,
        Self::RemoteAttach,
        Self::MirrorSync,
        Self::SupportExportUpload,
        Self::AdminPolicyPush,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiInference => "ai_inference",
            Self::ProviderToolCall => "provider_tool_call",
            Self::RemoteAttach => "remote_attach",
            Self::MirrorSync => "mirror_sync",
            Self::SupportExportUpload => "support_export_upload",
            Self::AdminPolicyPush => "admin_policy_push",
        }
    }
}

/// Region-pinning posture observed on a managed action lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionPinningStateClass {
    /// Pinned region matches the claimed region.
    PinnedMatchesClaim,
    /// Pinned region matches but recheck is required before write resumes.
    PinnedRecheckRequired,
    /// Provider default region is disclosed in place of a pinned region.
    ProviderDefaultDisclosed,
    /// Pinned region was lost or could not be verified; authority narrows.
    PinningLost,
    /// Pinned region drifted from the claimed region; authority closes on the
    /// affected lane only.
    DriftedFromClaim,
    /// Region is not yet known; authority remains visibly unresolved.
    Unresolved,
}

impl RegionPinningStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinnedMatchesClaim => "pinned_matches_claim",
            Self::PinnedRecheckRequired => "pinned_recheck_required",
            Self::ProviderDefaultDisclosed => "provider_default_disclosed",
            Self::PinningLost => "pinning_lost",
            Self::DriftedFromClaim => "drifted_from_claim",
            Self::Unresolved => "unresolved",
        }
    }

    /// True when authority on the affected lane must narrow.
    pub const fn narrows_authority(self) -> bool {
        matches!(
            self,
            Self::PinningLost | Self::DriftedFromClaim | Self::Unresolved
        )
    }
}

/// Tenant-boundary posture observed on a managed action lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TenantBoundaryStateClass {
    /// Bound tenant matches the claimed tenant.
    BoundMatchesClaim,
    /// Bound tenant matches but recheck is required before write resumes.
    BoundRecheckRequired,
    /// Tenant binding drifted from the claim; authority closes on this lane.
    DriftedFromClaim,
    /// Tenant binding could not be verified; authority closes on this lane.
    BindingLost,
    /// Tenant binding is not yet known; authority remains visibly unresolved.
    Unresolved,
    /// No tenant boundary applies (account-free local lane).
    NotApplicableLocal,
}

impl TenantBoundaryStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundMatchesClaim => "bound_matches_claim",
            Self::BoundRecheckRequired => "bound_recheck_required",
            Self::DriftedFromClaim => "drifted_from_claim",
            Self::BindingLost => "binding_lost",
            Self::Unresolved => "unresolved",
            Self::NotApplicableLocal => "not_applicable_local",
        }
    }

    /// True when authority on the affected lane must narrow.
    pub const fn narrows_authority(self) -> bool {
        matches!(
            self,
            Self::DriftedFromClaim | Self::BindingLost | Self::Unresolved
        )
    }
}

/// Key-mode posture observed on a managed action lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyModeStateClass {
    /// Active key mode matches the claimed key mode.
    MatchesClaim,
    /// Active key mode matches but a custody recheck is required.
    MatchesRecheckRequired,
    /// Active key mode drifted from the claim; authority closes on this lane.
    DriftedFromClaim,
    /// Key custody is degraded (locked store, unsigned vault, missing
    /// handle); authority closes on this lane.
    CustodyDegraded,
    /// Key mode is not yet known; authority remains visibly unresolved.
    Unresolved,
}

impl KeyModeStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MatchesClaim => "matches_claim",
            Self::MatchesRecheckRequired => "matches_recheck_required",
            Self::DriftedFromClaim => "drifted_from_claim",
            Self::CustodyDegraded => "custody_degraded",
            Self::Unresolved => "unresolved",
        }
    }

    /// True when authority on the affected lane must narrow.
    pub const fn narrows_authority(self) -> bool {
        matches!(
            self,
            Self::DriftedFromClaim | Self::CustodyDegraded | Self::Unresolved
        )
    }
}

/// Drill kind covered by a [`RegionTenantDrillPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionTenantDrillKindClass {
    /// Region pinning failure (lost / drifted / unresolved).
    RegionPinningFailure,
    /// Region failover from a primary region to a declared secondary region.
    RegionFailover,
    /// Tenant boundary drift between the claimed and bound tenant.
    TenantBoundaryDrift,
    /// Tenant failover from a primary tenant to a declared secondary tenant.
    TenantFailover,
    /// Key-mode drift (claim vs. active mode).
    KeyModeDrift,
    /// Key custody failover (vendor-managed → BYOK / customer-managed).
    KeyModeFailover,
}

impl RegionTenantDrillKindClass {
    /// All required drill kinds in canonical order.
    pub const ALL: [Self; 6] = [
        Self::RegionPinningFailure,
        Self::RegionFailover,
        Self::TenantBoundaryDrift,
        Self::TenantFailover,
        Self::KeyModeDrift,
        Self::KeyModeFailover,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegionPinningFailure => "region_pinning_failure",
            Self::RegionFailover => "region_failover",
            Self::TenantBoundaryDrift => "tenant_boundary_drift",
            Self::TenantFailover => "tenant_failover",
            Self::KeyModeDrift => "key_mode_drift",
            Self::KeyModeFailover => "key_mode_failover",
        }
    }

    /// Coarse axis label exported with the drill so reviewers can group drills
    /// by region / tenant / key-mode.
    pub const fn axis_token(self) -> &'static str {
        match self {
            Self::RegionPinningFailure | Self::RegionFailover => "region",
            Self::TenantBoundaryDrift | Self::TenantFailover => "tenant",
            Self::KeyModeDrift | Self::KeyModeFailover => "key_mode",
        }
    }
}

/// Drill outcome observed during the failure or failover exercise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionTenantDrillOutcomeClass {
    /// The drill succeeded: authority narrowed on the affected lane only and
    /// recovered when the underlying state recovered.
    NarrowedThenRecovered,
    /// The drill succeeded: authority narrowed on the affected lane only and
    /// remains narrowed until admin action.
    NarrowedAwaitingAdmin,
    /// The drill succeeded: authority failed over to a declared secondary
    /// region or tenant without widening on a sibling lane.
    FailedOverToDeclaredFallback,
}

impl RegionTenantDrillOutcomeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NarrowedThenRecovered => "narrowed_then_recovered",
            Self::NarrowedAwaitingAdmin => "narrowed_awaiting_admin",
            Self::FailedOverToDeclaredFallback => "failed_over_to_declared_fallback",
        }
    }
}

/// Disclosure block attached to every row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessingLocationDisclosure {
    /// Stable id of the disclosed region (e.g. `aureline-managed:us-east-1`).
    pub region_id: String,
    /// Human-readable label for the region.
    pub region_label: String,
    /// Stable id of the data-residency zone declared by the upstream contract.
    pub residency_zone_id: String,
    /// Reviewable origin ref for the disclosed region.
    pub disclosed_origin_ref: String,
}

/// One region disclosure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionDisclosureRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Managed action lane the row applies to.
    pub managed_lane: ManagedActionLaneClass,
    /// Stable token for [`Self::managed_lane`].
    pub managed_lane_token: String,
    /// Profile under which the row was inspected.
    pub profile: RegionTenantKeyModeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Claimed upstream region posture.
    pub claimed_region_mode: RegionMode,
    /// Stable token for [`Self::claimed_region_mode`].
    pub claimed_region_mode_token: String,
    /// Active observed region posture.
    pub active_region_mode: RegionMode,
    /// Stable token for [`Self::active_region_mode`].
    pub active_region_mode_token: String,
    /// Active residency posture.
    pub residency_mode: ResidencyMode,
    /// Stable token for [`Self::residency_mode`].
    pub residency_mode_token: String,
    /// Pinning state observed on this lane.
    pub pinning_state: RegionPinningStateClass,
    /// Stable token for [`Self::pinning_state`].
    pub pinning_state_token: String,
    /// Disclosure block exported with the row.
    pub disclosure: ProcessingLocationDisclosure,
    /// Authority applied to the affected lane after disclosure.
    pub effective_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::effective_authority`].
    pub effective_authority_token: String,
    /// Drill packet ref tied to this row (empty when no drill applies).
    pub linked_drill_packet_id: String,
    /// True when no undeclared public-endpoint fallback is permitted on this
    /// lane.
    pub no_public_endpoint_fallback: bool,
    /// True when raw private material is excluded from the record.
    pub raw_private_material_excluded: bool,
    /// True when local-only work continues regardless of this lane's authority.
    pub local_editing_preserved: bool,
}

/// One tenant boundary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TenantBoundaryRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Managed action lane the row applies to.
    pub managed_lane: ManagedActionLaneClass,
    /// Stable token for [`Self::managed_lane`].
    pub managed_lane_token: String,
    /// Profile under which the row was inspected.
    pub profile: RegionTenantKeyModeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Claimed tenant id (synthetic; never carries raw private material).
    pub claimed_tenant_id: String,
    /// Active observed tenant id (synthetic; never carries raw private
    /// material).
    pub active_tenant_id: String,
    /// Workspace boundary id bound to the tenant.
    pub workspace_boundary_id: String,
    /// Tenant boundary state observed on this lane.
    pub boundary_state: TenantBoundaryStateClass,
    /// Stable token for [`Self::boundary_state`].
    pub boundary_state_token: String,
    /// Disclosure origin ref for the bound tenant.
    pub disclosed_origin_ref: String,
    /// Authority applied to the affected lane after disclosure.
    pub effective_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::effective_authority`].
    pub effective_authority_token: String,
    /// Drill packet ref tied to this row (empty when no drill applies).
    pub linked_drill_packet_id: String,
    /// True when no undeclared public-endpoint fallback is permitted on this
    /// lane.
    pub no_public_endpoint_fallback: bool,
    /// True when raw private material is excluded from the record.
    pub raw_private_material_excluded: bool,
    /// True when local-only work continues regardless of this lane's authority.
    pub local_editing_preserved: bool,
}

/// One key-mode row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyModeRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Managed action lane the row applies to.
    pub managed_lane: ManagedActionLaneClass,
    /// Stable token for [`Self::managed_lane`].
    pub managed_lane_token: String,
    /// Profile under which the row was inspected.
    pub profile: RegionTenantKeyModeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Claimed key mode.
    pub claimed_key_mode: KeyMode,
    /// Stable token for [`Self::claimed_key_mode`].
    pub claimed_key_mode_token: String,
    /// Active observed key mode.
    pub active_key_mode: KeyMode,
    /// Stable token for [`Self::active_key_mode`].
    pub active_key_mode_token: String,
    /// State observed for the key custody on this lane.
    pub key_state: KeyModeStateClass,
    /// Stable token for [`Self::key_state`].
    pub key_state_token: String,
    /// Custody origin ref for the active key mode.
    pub custody_origin_ref: String,
    /// Authority applied to the affected lane after disclosure.
    pub effective_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::effective_authority`].
    pub effective_authority_token: String,
    /// Drill packet ref tied to this row (empty when no drill applies).
    pub linked_drill_packet_id: String,
    /// True when no undeclared public-endpoint fallback is permitted on this
    /// lane.
    pub no_public_endpoint_fallback: bool,
    /// True when raw private material is excluded from the record.
    pub raw_private_material_excluded: bool,
    /// True when local-only work continues regardless of this lane's authority.
    pub local_editing_preserved: bool,
}

/// Drill packet for a region, tenant, or key-mode failure / failover scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionTenantDrillPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable drill packet id.
    pub drill_packet_id: String,
    /// Drill kind exercised by the packet.
    pub drill_kind: RegionTenantDrillKindClass,
    /// Stable token for [`Self::drill_kind`].
    pub drill_kind_token: String,
    /// Coarse axis token (`region`, `tenant`, `key_mode`).
    pub axis_token: String,
    /// Managed action lane exercised by the drill.
    pub managed_lane: ManagedActionLaneClass,
    /// Stable token for [`Self::managed_lane`].
    pub managed_lane_token: String,
    /// Profile under which the drill was exercised.
    pub profile: RegionTenantKeyModeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Recorded outcome of the drill.
    pub outcome: RegionTenantDrillOutcomeClass,
    /// Stable token for [`Self::outcome`].
    pub outcome_token: String,
    /// Before-state label.
    pub before_state_label: String,
    /// After-state label.
    pub after_state_label: String,
    /// Effective authority on the affected lane before the drill.
    pub before_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::before_authority`].
    pub before_authority_token: String,
    /// Effective authority on the affected lane after the drill.
    pub after_authority: CapabilityAuthorityClass,
    /// Stable token for [`Self::after_authority`].
    pub after_authority_token: String,
    /// Plain-language explanation rendered by inspectors and exports.
    pub explanation: String,
    /// Started-at timestamp.
    pub started_at: String,
    /// Resolved-at timestamp (may equal `started_at` when failover was
    /// instantaneous).
    pub resolved_at: String,
    /// Reviewable origin ref for the drill artifact.
    pub artifact_ref: String,
    /// True when local-only work was preserved across the entire drill.
    pub local_editing_preserved: bool,
    /// True when no sibling lane's authority widened during the drill.
    pub sibling_lanes_unwidened: bool,
    /// True when raw private material is excluded from the packet.
    pub raw_private_material_excluded: bool,
}

/// Typed validator defect kind for the region / tenant / key-mode beta page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionTenantKeyModeBetaDefectKind {
    /// A token field drifted from its strongly-typed class.
    TokenDrift,
    /// A claimed managed/enterprise profile row did not disclose region truth.
    ClaimedManagedRegionUndisclosed,
    /// A claimed managed/enterprise profile row did not disclose tenant truth.
    ClaimedManagedTenantUndisclosed,
    /// A claimed managed/enterprise profile row did not disclose key-mode
    /// truth.
    ClaimedManagedKeyModeUndisclosed,
    /// A row narrowing authority did so for the wrong lane vocabulary.
    NarrowingAuthorityDrift,
    /// A row indicated mismatch but authority remained `Allowed`.
    MismatchMasksIssue,
    /// A row permits an undeclared public endpoint fallback.
    HiddenPublicEndpointFallback,
    /// A row would expose raw private or secret material.
    RawPrivateMaterialExposed,
    /// A row did not preserve local editing.
    LocalEditingNotPreserved,
    /// A drill packet's axis token does not match its drill kind.
    DrillAxisTokenDrift,
    /// A drill packet's outcome widened authority on a sibling lane.
    DrillSiblingLaneWidened,
    /// A drill packet did not preserve local editing.
    DrillLocalEditingNotPreserved,
    /// A required drill axis (region/tenant/key_mode) is missing from the
    /// page.
    DrillAxisCoverageMissing,
    /// A required beta profile is missing from the page coverage.
    ProfileCoverageMissing,
}

impl RegionTenantKeyModeBetaDefectKind {
    /// Stable token recorded on defect rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenDrift => "token_drift",
            Self::ClaimedManagedRegionUndisclosed => "claimed_managed_region_undisclosed",
            Self::ClaimedManagedTenantUndisclosed => "claimed_managed_tenant_undisclosed",
            Self::ClaimedManagedKeyModeUndisclosed => "claimed_managed_key_mode_undisclosed",
            Self::NarrowingAuthorityDrift => "narrowing_authority_drift",
            Self::MismatchMasksIssue => "mismatch_masks_issue",
            Self::HiddenPublicEndpointFallback => "hidden_public_endpoint_fallback",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::LocalEditingNotPreserved => "local_editing_not_preserved",
            Self::DrillAxisTokenDrift => "drill_axis_token_drift",
            Self::DrillSiblingLaneWidened => "drill_sibling_lane_widened",
            Self::DrillLocalEditingNotPreserved => "drill_local_editing_not_preserved",
            Self::DrillAxisCoverageMissing => "drill_axis_coverage_missing",
            Self::ProfileCoverageMissing => "profile_coverage_missing",
        }
    }
}

/// Typed validation defect for the region / tenant / key-mode beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionTenantKeyModeBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: RegionTenantKeyModeBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id (row id, drill id, or "page").
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl RegionTenantKeyModeBetaDefect {
    fn new(
        defect_kind: RegionTenantKeyModeBetaDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: REGION_TENANT_KEY_MODE_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: REGION_TENANT_KEY_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: REGION_TENANT_KEY_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the region / tenant / key-mode beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionTenantKeyModeBetaSummary {
    /// Stable record kind for the parent page.
    pub page_record_kind: String,
    /// Stable record kind for the summary itself.
    pub record_kind: String,
    /// Number of region disclosure rows.
    pub region_row_count: usize,
    /// Number of tenant boundary rows.
    pub tenant_row_count: usize,
    /// Number of key-mode rows.
    pub key_mode_row_count: usize,
    /// Number of drill packets.
    pub drill_packet_count: usize,
    /// Profile tokens present across the page.
    pub profiles_present: Vec<String>,
    /// Managed action lane tokens present across the page.
    pub managed_lanes_present: Vec<String>,
    /// Drill axes present across the page (`region`, `tenant`, `key_mode`).
    pub drill_axes_present: Vec<String>,
    /// Drill kind tokens present across the page.
    pub drill_kinds_present: Vec<String>,
    /// Number of defects.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl RegionTenantKeyModeBetaSummary {
    /// Builds the summary over region/tenant/key-mode rows, drill packets, and
    /// defects.
    pub fn from_records(
        region_rows: &[RegionDisclosureRow],
        tenant_rows: &[TenantBoundaryRow],
        key_mode_rows: &[KeyModeRow],
        drill_packets: &[RegionTenantDrillPacket],
        defects: &[RegionTenantKeyModeBetaDefect],
    ) -> Self {
        let mut profiles_present: BTreeSet<String> = BTreeSet::new();
        let mut managed_lanes_present: BTreeSet<String> = BTreeSet::new();
        for row in region_rows {
            profiles_present.insert(row.profile_token.clone());
            managed_lanes_present.insert(row.managed_lane_token.clone());
        }
        for row in tenant_rows {
            profiles_present.insert(row.profile_token.clone());
            managed_lanes_present.insert(row.managed_lane_token.clone());
        }
        for row in key_mode_rows {
            profiles_present.insert(row.profile_token.clone());
            managed_lanes_present.insert(row.managed_lane_token.clone());
        }
        for packet in drill_packets {
            profiles_present.insert(packet.profile_token.clone());
            managed_lanes_present.insert(packet.managed_lane_token.clone());
        }

        let drill_axes_present: BTreeSet<String> = drill_packets
            .iter()
            .map(|packet| packet.axis_token.clone())
            .collect();
        let drill_kinds_present: BTreeSet<String> = drill_packets
            .iter()
            .map(|packet| packet.drill_kind_token.clone())
            .collect();

        let mut defect_counts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: REGION_TENANT_KEY_MODE_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: REGION_TENANT_KEY_MODE_BETA_SUMMARY_RECORD_KIND.to_owned(),
            region_row_count: region_rows.len(),
            tenant_row_count: tenant_rows.len(),
            key_mode_row_count: key_mode_rows.len(),
            drill_packet_count: drill_packets.len(),
            profiles_present: profiles_present.into_iter().collect(),
            managed_lanes_present: managed_lanes_present.into_iter().collect(),
            drill_axes_present: drill_axes_present.into_iter().collect(),
            drill_kinds_present: drill_kinds_present.into_iter().collect(),
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level beta page consumed by admin, support, shell, settings, headless,
/// and fixture replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionTenantKeyModeBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Region disclosure rows.
    pub region_rows: Vec<RegionDisclosureRow>,
    /// Tenant boundary rows.
    pub tenant_rows: Vec<TenantBoundaryRow>,
    /// Key-mode rows.
    pub key_mode_rows: Vec<KeyModeRow>,
    /// Drill packets.
    pub drill_packets: Vec<RegionTenantDrillPacket>,
    /// Typed validation defects.
    pub defects: Vec<RegionTenantKeyModeBetaDefect>,
    /// Aggregate summary.
    pub summary: RegionTenantKeyModeBetaSummary,
}

/// Support-export wrapper for the region / tenant / key-mode beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionTenantKeyModeBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported page.
    pub page: RegionTenantKeyModeBetaPage,
    /// Defect kind tokens present.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when the no-public-endpoint-fallback invariant held across the
    /// page.
    pub no_public_endpoint_fallback_invariant: bool,
    /// True when local editing was preserved across every row.
    pub local_editing_preserved_invariant: bool,
}

impl RegionTenantKeyModeBetaSupportExport {
    /// Builds a support-export wrapper from a beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: RegionTenantKeyModeBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        let no_public_endpoint_fallback_invariant = page
            .region_rows
            .iter()
            .all(|row| row.no_public_endpoint_fallback)
            && page
                .tenant_rows
                .iter()
                .all(|row| row.no_public_endpoint_fallback)
            && page
                .key_mode_rows
                .iter()
                .all(|row| row.no_public_endpoint_fallback);
        let local_editing_preserved_invariant = page
            .region_rows
            .iter()
            .all(|row| row.local_editing_preserved)
            && page
                .tenant_rows
                .iter()
                .all(|row| row.local_editing_preserved)
            && page
                .key_mode_rows
                .iter()
                .all(|row| row.local_editing_preserved)
            && page
                .drill_packets
                .iter()
                .all(|packet| packet.local_editing_preserved);
        Self {
            record_kind: REGION_TENANT_KEY_MODE_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: REGION_TENANT_KEY_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: REGION_TENANT_KEY_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_private_material_excluded: true,
            no_public_endpoint_fallback_invariant,
            local_editing_preserved_invariant,
        }
    }
}

/// Validates the region / tenant / key-mode beta page and returns typed defects
/// on failure.
pub fn validate_region_tenant_key_mode_beta_page(
    page: &RegionTenantKeyModeBetaPage,
) -> Result<(), Vec<RegionTenantKeyModeBetaDefect>> {
    let defects = audit_region_tenant_key_mode_beta_page(
        &page.region_rows,
        &page.tenant_rows,
        &page.key_mode_rows,
        &page.drill_packets,
    );
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for the region / tenant / key-mode beta page.
pub fn audit_region_tenant_key_mode_beta_page(
    region_rows: &[RegionDisclosureRow],
    tenant_rows: &[TenantBoundaryRow],
    key_mode_rows: &[KeyModeRow],
    drill_packets: &[RegionTenantDrillPacket],
) -> Vec<RegionTenantKeyModeBetaDefect> {
    let mut defects = Vec::new();

    for row in region_rows {
        audit_region_row(row, &mut defects);
    }
    for row in tenant_rows {
        audit_tenant_row(row, &mut defects);
    }
    for row in key_mode_rows {
        audit_key_mode_row(row, &mut defects);
    }
    for packet in drill_packets {
        audit_drill_packet(packet, &mut defects);
    }

    let required_profiles: BTreeSet<&str> = RegionTenantKeyModeBetaProfileClass::ALL
        .iter()
        .map(|profile| profile.as_str())
        .collect();
    let observed_profiles: BTreeSet<&str> = region_rows
        .iter()
        .map(|row| row.profile_token.as_str())
        .chain(tenant_rows.iter().map(|row| row.profile_token.as_str()))
        .chain(key_mode_rows.iter().map(|row| row.profile_token.as_str()))
        .chain(
            drill_packets
                .iter()
                .map(|packet| packet.profile_token.as_str()),
        )
        .collect();
    for missing in required_profiles.difference(&observed_profiles) {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::ProfileCoverageMissing,
            "page",
            "profiles",
            format!("missing {} profile coverage", missing),
        ));
    }

    let required_axes: BTreeSet<&str> = ["region", "tenant", "key_mode"].into_iter().collect();
    let observed_axes: BTreeSet<&str> = drill_packets
        .iter()
        .map(|packet| packet.axis_token.as_str())
        .collect();
    for missing in required_axes.difference(&observed_axes) {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::DrillAxisCoverageMissing,
            "page",
            "drill_packets",
            format!("missing drill axis coverage: {}", missing),
        ));
    }

    defects
}

fn audit_region_row(row: &RegionDisclosureRow, defects: &mut Vec<RegionTenantKeyModeBetaDefect>) {
    if row.managed_lane_token != row.managed_lane.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "managed_lane_token",
            "managed_lane_token must match managed_lane",
        ));
    }
    if row.profile_token != row.profile.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "profile_token",
            "profile_token must match profile",
        ));
    }
    if row.claimed_region_mode_token != row.claimed_region_mode.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "claimed_region_mode_token",
            "claimed_region_mode_token must match claimed_region_mode",
        ));
    }
    if row.active_region_mode_token != row.active_region_mode.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "active_region_mode_token",
            "active_region_mode_token must match active_region_mode",
        ));
    }
    if row.residency_mode_token != row.residency_mode.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "residency_mode_token",
            "residency_mode_token must match residency_mode",
        ));
    }
    if row.pinning_state_token != row.pinning_state.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "pinning_state_token",
            "pinning_state_token must match pinning_state",
        ));
    }
    if row.effective_authority_token != row.effective_authority.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "effective_authority_token",
            "effective_authority_token must match effective_authority",
        ));
    }
    if claimed_managed_profile(row.profile)
        && (row.disclosure.region_id.is_empty() || row.active_region_mode.is_unknown())
    {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::ClaimedManagedRegionUndisclosed,
            row.row_id.clone(),
            "disclosure.region_id",
            "claimed managed/enterprise row must disclose a region",
        ));
    }
    if row.pinning_state.narrows_authority() && row.effective_authority.is_admitted() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::MismatchMasksIssue,
            row.row_id.clone(),
            "effective_authority",
            "mismatched/degraded region must not remain Allowed/ReadOnly",
        ));
    }
    if !row.pinning_state.narrows_authority()
        && matches!(
            row.effective_authority,
            CapabilityAuthorityClass::PolicyDenied
                | CapabilityAuthorityClass::QuarantineDenied
                | CapabilityAuthorityClass::BlockedPendingTrust
                | CapabilityAuthorityClass::BlockedPendingApproval
        )
    {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::NarrowingAuthorityDrift,
            row.row_id.clone(),
            "effective_authority",
            "region row narrowed authority without a narrowing pinning state",
        ));
    }
    if !row.no_public_endpoint_fallback {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::HiddenPublicEndpointFallback,
            row.row_id.clone(),
            "no_public_endpoint_fallback",
            "region row permits undeclared public endpoint fallback",
        ));
    }
    if !row.raw_private_material_excluded {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::RawPrivateMaterialExposed,
            row.row_id.clone(),
            "raw_private_material_excluded",
            "region row must exclude raw private material",
        ));
    }
    if !row.local_editing_preserved {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::LocalEditingNotPreserved,
            row.row_id.clone(),
            "local_editing_preserved",
            "region row must preserve local editing",
        ));
    }
}

fn audit_tenant_row(row: &TenantBoundaryRow, defects: &mut Vec<RegionTenantKeyModeBetaDefect>) {
    if row.managed_lane_token != row.managed_lane.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "managed_lane_token",
            "managed_lane_token must match managed_lane",
        ));
    }
    if row.profile_token != row.profile.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "profile_token",
            "profile_token must match profile",
        ));
    }
    if row.boundary_state_token != row.boundary_state.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "boundary_state_token",
            "boundary_state_token must match boundary_state",
        ));
    }
    if row.effective_authority_token != row.effective_authority.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "effective_authority_token",
            "effective_authority_token must match effective_authority",
        ));
    }
    if claimed_managed_profile(row.profile)
        && row.boundary_state != TenantBoundaryStateClass::NotApplicableLocal
        && (row.claimed_tenant_id.is_empty() || row.active_tenant_id.is_empty())
    {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::ClaimedManagedTenantUndisclosed,
            row.row_id.clone(),
            "active_tenant_id",
            "claimed managed/enterprise row must disclose a tenant boundary",
        ));
    }
    if row.boundary_state.narrows_authority() && row.effective_authority.is_admitted() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::MismatchMasksIssue,
            row.row_id.clone(),
            "effective_authority",
            "mismatched/degraded tenant boundary must not remain Allowed/ReadOnly",
        ));
    }
    if !row.no_public_endpoint_fallback {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::HiddenPublicEndpointFallback,
            row.row_id.clone(),
            "no_public_endpoint_fallback",
            "tenant row permits undeclared public endpoint fallback",
        ));
    }
    if !row.raw_private_material_excluded {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::RawPrivateMaterialExposed,
            row.row_id.clone(),
            "raw_private_material_excluded",
            "tenant row must exclude raw private material",
        ));
    }
    if !row.local_editing_preserved {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::LocalEditingNotPreserved,
            row.row_id.clone(),
            "local_editing_preserved",
            "tenant row must preserve local editing",
        ));
    }
}

fn audit_key_mode_row(row: &KeyModeRow, defects: &mut Vec<RegionTenantKeyModeBetaDefect>) {
    if row.managed_lane_token != row.managed_lane.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "managed_lane_token",
            "managed_lane_token must match managed_lane",
        ));
    }
    if row.profile_token != row.profile.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "profile_token",
            "profile_token must match profile",
        ));
    }
    if row.claimed_key_mode_token != row.claimed_key_mode.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "claimed_key_mode_token",
            "claimed_key_mode_token must match claimed_key_mode",
        ));
    }
    if row.active_key_mode_token != row.active_key_mode.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "active_key_mode_token",
            "active_key_mode_token must match active_key_mode",
        ));
    }
    if row.key_state_token != row.key_state.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "key_state_token",
            "key_state_token must match key_state",
        ));
    }
    if row.effective_authority_token != row.effective_authority.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            row.row_id.clone(),
            "effective_authority_token",
            "effective_authority_token must match effective_authority",
        ));
    }
    if claimed_managed_profile(row.profile)
        && (row.active_key_mode.is_unknown() || row.custody_origin_ref.is_empty())
    {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::ClaimedManagedKeyModeUndisclosed,
            row.row_id.clone(),
            "active_key_mode",
            "claimed managed/enterprise row must disclose a key mode and custody origin",
        ));
    }
    if row.key_state.narrows_authority() && row.effective_authority.is_admitted() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::MismatchMasksIssue,
            row.row_id.clone(),
            "effective_authority",
            "mismatched/degraded key mode must not remain Allowed/ReadOnly",
        ));
    }
    if !row.no_public_endpoint_fallback {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::HiddenPublicEndpointFallback,
            row.row_id.clone(),
            "no_public_endpoint_fallback",
            "key-mode row permits undeclared public endpoint fallback",
        ));
    }
    if !row.raw_private_material_excluded {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::RawPrivateMaterialExposed,
            row.row_id.clone(),
            "raw_private_material_excluded",
            "key-mode row must exclude raw private material",
        ));
    }
    if !row.local_editing_preserved {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::LocalEditingNotPreserved,
            row.row_id.clone(),
            "local_editing_preserved",
            "key-mode row must preserve local editing",
        ));
    }
}

fn audit_drill_packet(
    packet: &RegionTenantDrillPacket,
    defects: &mut Vec<RegionTenantKeyModeBetaDefect>,
) {
    if packet.drill_kind_token != packet.drill_kind.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "drill_kind_token",
            "drill_kind_token must match drill_kind",
        ));
    }
    if packet.axis_token != packet.drill_kind.axis_token() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::DrillAxisTokenDrift,
            packet.drill_packet_id.clone(),
            "axis_token",
            "axis_token must match drill_kind axis",
        ));
    }
    if packet.managed_lane_token != packet.managed_lane.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "managed_lane_token",
            "managed_lane_token must match managed_lane",
        ));
    }
    if packet.profile_token != packet.profile.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "profile_token",
            "profile_token must match profile",
        ));
    }
    if packet.outcome_token != packet.outcome.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "outcome_token",
            "outcome_token must match outcome",
        ));
    }
    if packet.before_authority_token != packet.before_authority.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "before_authority_token",
            "before_authority_token must match before_authority",
        ));
    }
    if packet.after_authority_token != packet.after_authority.as_str() {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::TokenDrift,
            packet.drill_packet_id.clone(),
            "after_authority_token",
            "after_authority_token must match after_authority",
        ));
    }
    if !packet.sibling_lanes_unwidened {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::DrillSiblingLaneWidened,
            packet.drill_packet_id.clone(),
            "sibling_lanes_unwidened",
            "drill must not widen authority on sibling lanes",
        ));
    }
    if !packet.local_editing_preserved {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::DrillLocalEditingNotPreserved,
            packet.drill_packet_id.clone(),
            "local_editing_preserved",
            "drill must preserve local editing",
        ));
    }
    if !packet.raw_private_material_excluded {
        defects.push(RegionTenantKeyModeBetaDefect::new(
            RegionTenantKeyModeBetaDefectKind::RawPrivateMaterialExposed,
            packet.drill_packet_id.clone(),
            "raw_private_material_excluded",
            "drill packet must exclude raw private material",
        ));
    }
}

const fn claimed_managed_profile(profile: RegionTenantKeyModeBetaProfileClass) -> bool {
    matches!(
        profile,
        RegionTenantKeyModeBetaProfileClass::Connected
            | RegionTenantKeyModeBetaProfileClass::MirrorOnly
            | RegionTenantKeyModeBetaProfileClass::EnterpriseManaged
    )
}

/// Builds the seeded region / tenant / key-mode beta page covering connected,
/// mirror, offline, and enterprise-managed profiles with region disclosure
/// rows, tenant boundary rows, key-mode rows, and one drill packet per axis.
pub fn seeded_region_tenant_key_mode_beta_page() -> RegionTenantKeyModeBetaPage {
    let region_rows = seed_region_rows();
    let tenant_rows = seed_tenant_rows();
    let key_mode_rows = seed_key_mode_rows();
    let drill_packets = seed_drill_packets();

    let defects = audit_region_tenant_key_mode_beta_page(
        &region_rows,
        &tenant_rows,
        &key_mode_rows,
        &drill_packets,
    );
    let summary = RegionTenantKeyModeBetaSummary::from_records(
        &region_rows,
        &tenant_rows,
        &key_mode_rows,
        &drill_packets,
        &defects,
    );

    RegionTenantKeyModeBetaPage {
        record_kind: REGION_TENANT_KEY_MODE_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: REGION_TENANT_KEY_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: REGION_TENANT_KEY_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: REGION_TENANT_KEY_MODE_BETA_SOURCE_MATRIX_REF.to_owned(),
        region_rows,
        tenant_rows,
        key_mode_rows,
        drill_packets,
        defects,
        summary,
    }
}

fn seed_region_rows() -> Vec<RegionDisclosureRow> {
    vec![
        region_row(
            "region-row:connected:ai-inference",
            ManagedActionLaneClass::AiInference,
            RegionTenantKeyModeBetaProfileClass::Connected,
            RegionMode::CustomerRegionPinned,
            RegionMode::CustomerRegionPinned,
            ResidencyMode::ManagedTenantDocumentedRegion,
            RegionPinningStateClass::PinnedMatchesClaim,
            ProcessingLocationDisclosure {
                region_id: "aureline-managed:us-east-1".to_owned(),
                region_label: "AurelineManaged US-East-1".to_owned(),
                residency_zone_id: "residency:us-east".to_owned(),
                disclosed_origin_ref:
                    "https://control.aureline.example/regions/us-east-1?profile=connected"
                        .to_owned(),
            },
            CapabilityAuthorityClass::Allowed,
            "region-tenant-drill:region-failover-001",
        ),
        region_row(
            "region-row:mirror:provider-tool-call",
            ManagedActionLaneClass::ProviderToolCall,
            RegionTenantKeyModeBetaProfileClass::MirrorOnly,
            RegionMode::CustomerRegionPinned,
            RegionMode::CustomerRegionPinned,
            ResidencyMode::ManagedTenantDocumentedRegion,
            RegionPinningStateClass::PinnedRecheckRequired,
            ProcessingLocationDisclosure {
                region_id: "aureline-managed:us-east-1".to_owned(),
                region_label: "AurelineManaged US-East-1 (mirror)".to_owned(),
                residency_zone_id: "residency:us-east".to_owned(),
                disclosed_origin_ref:
                    "mirror://signed-mirror/aureline/regions/us-east-1?profile=mirror".to_owned(),
            },
            CapabilityAuthorityClass::ReadOnly,
            "",
        ),
        region_row(
            "region-row:offline:mirror-sync",
            ManagedActionLaneClass::MirrorSync,
            RegionTenantKeyModeBetaProfileClass::Offline,
            RegionMode::CustomerRegionPinned,
            RegionMode::BoundaryRecheckRequired,
            ResidencyMode::LocalDeviceOnly,
            RegionPinningStateClass::PinningLost,
            ProcessingLocationDisclosure {
                region_id: "airgap:lastknowngood:2026-05-14".to_owned(),
                region_label: "Air-gapped snapshot 2026-05-14".to_owned(),
                residency_zone_id: "residency:local".to_owned(),
                disclosed_origin_ref: "airgap://courier-2026-05-14/aureline/regions".to_owned(),
            },
            CapabilityAuthorityClass::DegradedPreviewOnly,
            "region-tenant-drill:region-pinning-failure-001",
        ),
        region_row(
            "region-row:enterprise:remote-attach",
            ManagedActionLaneClass::RemoteAttach,
            RegionTenantKeyModeBetaProfileClass::EnterpriseManaged,
            RegionMode::CustomerRegionPinned,
            RegionMode::CustomerRegionPinned,
            ResidencyMode::ManagedTenantDocumentedRegion,
            RegionPinningStateClass::PinnedMatchesClaim,
            ProcessingLocationDisclosure {
                region_id: "aureline-managed:eu-central-1".to_owned(),
                region_label: "AurelineManaged EU-Central-1".to_owned(),
                residency_zone_id: "residency:eu-central".to_owned(),
                disclosed_origin_ref:
                    "https://control.aureline.example/regions/eu-central-1?profile=enterprise"
                        .to_owned(),
            },
            CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            "",
        ),
    ]
}

fn seed_tenant_rows() -> Vec<TenantBoundaryRow> {
    vec![
        tenant_row(
            "tenant-row:connected:provider-tool-call",
            ManagedActionLaneClass::ProviderToolCall,
            RegionTenantKeyModeBetaProfileClass::Connected,
            "tenant:enterprise-pilot-alpha",
            "tenant:enterprise-pilot-alpha",
            "workspace:enterprise-pilot:alpha-main",
            TenantBoundaryStateClass::BoundMatchesClaim,
            "https://control.aureline.example/tenants/enterprise-pilot-alpha?profile=connected",
            CapabilityAuthorityClass::Allowed,
            "region-tenant-drill:tenant-boundary-drift-001",
        ),
        tenant_row(
            "tenant-row:mirror:admin-policy-push",
            ManagedActionLaneClass::AdminPolicyPush,
            RegionTenantKeyModeBetaProfileClass::MirrorOnly,
            "tenant:enterprise-pilot-alpha",
            "tenant:enterprise-pilot-alpha",
            "workspace:enterprise-pilot:alpha-main",
            TenantBoundaryStateClass::BoundRecheckRequired,
            "mirror://signed-mirror/aureline/tenants/enterprise-pilot-alpha?profile=mirror",
            CapabilityAuthorityClass::ReadOnly,
            "",
        ),
        tenant_row(
            "tenant-row:offline:support-export-upload",
            ManagedActionLaneClass::SupportExportUpload,
            RegionTenantKeyModeBetaProfileClass::Offline,
            "tenant:enterprise-pilot-alpha",
            "tenant:enterprise-pilot-alpha",
            "workspace:enterprise-pilot:alpha-main",
            TenantBoundaryStateClass::BindingLost,
            "airgap://courier-2026-05-14/aureline/tenants/enterprise-pilot-alpha",
            CapabilityAuthorityClass::BlockedPendingTrust,
            "region-tenant-drill:tenant-failover-001",
        ),
        tenant_row(
            "tenant-row:enterprise:remote-attach",
            ManagedActionLaneClass::RemoteAttach,
            RegionTenantKeyModeBetaProfileClass::EnterpriseManaged,
            "tenant:enterprise-pilot-alpha",
            "tenant:enterprise-pilot-alpha",
            "workspace:enterprise-pilot:alpha-secondary",
            TenantBoundaryStateClass::BoundMatchesClaim,
            "https://control.aureline.example/tenants/enterprise-pilot-alpha?profile=enterprise",
            CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            "",
        ),
    ]
}

fn seed_key_mode_rows() -> Vec<KeyModeRow> {
    vec![
        key_mode_row(
            "key-mode-row:connected:ai-inference",
            ManagedActionLaneClass::AiInference,
            RegionTenantKeyModeBetaProfileClass::Connected,
            KeyMode::VendorManaged,
            KeyMode::VendorManaged,
            KeyModeStateClass::MatchesClaim,
            "https://control.aureline.example/keys/vendor-managed/us-east-1",
            CapabilityAuthorityClass::Allowed,
            "",
        ),
        key_mode_row(
            "key-mode-row:mirror:provider-tool-call",
            ManagedActionLaneClass::ProviderToolCall,
            RegionTenantKeyModeBetaProfileClass::MirrorOnly,
            KeyMode::VendorManaged,
            KeyMode::VendorManaged,
            KeyModeStateClass::MatchesRecheckRequired,
            "mirror://signed-mirror/aureline/keys/vendor-managed/us-east-1",
            CapabilityAuthorityClass::ReadOnly,
            "",
        ),
        key_mode_row(
            "key-mode-row:offline:ai-inference",
            ManagedActionLaneClass::AiInference,
            RegionTenantKeyModeBetaProfileClass::Offline,
            KeyMode::VendorManaged,
            KeyMode::ByokUserManaged,
            KeyModeStateClass::DriftedFromClaim,
            "airgap://courier-2026-05-14/aureline/keys/byok-user-managed",
            CapabilityAuthorityClass::DegradedPreviewOnly,
            "region-tenant-drill:key-mode-failover-001",
        ),
        key_mode_row(
            "key-mode-row:enterprise:admin-policy-push",
            ManagedActionLaneClass::AdminPolicyPush,
            RegionTenantKeyModeBetaProfileClass::EnterpriseManaged,
            KeyMode::CustomerManaged,
            KeyMode::CustomerManaged,
            KeyModeStateClass::MatchesClaim,
            "https://control.aureline.example/keys/customer-managed/eu-central-1",
            CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            "region-tenant-drill:key-mode-drift-001",
        ),
    ]
}

fn seed_drill_packets() -> Vec<RegionTenantDrillPacket> {
    vec![
        drill_packet(
            "region-tenant-drill:region-pinning-failure-001",
            RegionTenantDrillKindClass::RegionPinningFailure,
            ManagedActionLaneClass::MirrorSync,
            RegionTenantKeyModeBetaProfileClass::Offline,
            RegionTenantDrillOutcomeClass::NarrowedAwaitingAdmin,
            "pinned_matches_claim",
            "pinning_lost",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::DegradedPreviewOnly,
            "Air-gapped courier delayed past freshness window. Mirror-sync lane narrowed to preview-only while sibling lanes stayed at their declared offline authority.",
            "2026-05-14T22:00:00Z",
            "2026-05-15T03:00:00Z",
            "artifacts/security/m3/region_tenant_drills/region_pinning_failure_001.json",
        ),
        drill_packet(
            "region-tenant-drill:region-failover-001",
            RegionTenantDrillKindClass::RegionFailover,
            ManagedActionLaneClass::AiInference,
            RegionTenantKeyModeBetaProfileClass::Connected,
            RegionTenantDrillOutcomeClass::FailedOverToDeclaredFallback,
            "active_region:us-east-1",
            "active_region:us-east-2_declared_fallback",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::Allowed,
            "Primary US-East-1 region brown-outed; managed control plane failed AI inference over to the declared US-East-2 secondary while preserving the same residency zone. Sibling lanes unchanged.",
            "2026-05-15T01:00:00Z",
            "2026-05-15T01:08:00Z",
            "artifacts/security/m3/region_tenant_drills/region_failover_001.json",
        ),
        drill_packet(
            "region-tenant-drill:tenant-boundary-drift-001",
            RegionTenantDrillKindClass::TenantBoundaryDrift,
            ManagedActionLaneClass::ProviderToolCall,
            RegionTenantKeyModeBetaProfileClass::Connected,
            RegionTenantDrillOutcomeClass::NarrowedThenRecovered,
            "bound_tenant:enterprise-pilot-alpha",
            "drifted_to_tenant:enterprise-pilot-beta",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::BlockedPendingTrust,
            "Provider tool-call lane observed tenant id drift from alpha to beta during a router test. The lane fell to blocked_pending_trust on its own; sibling lanes stayed at their declared authority and recovered once the router re-pinned alpha.",
            "2026-05-15T01:20:00Z",
            "2026-05-15T01:25:00Z",
            "artifacts/security/m3/region_tenant_drills/tenant_boundary_drift_001.json",
        ),
        drill_packet(
            "region-tenant-drill:tenant-failover-001",
            RegionTenantDrillKindClass::TenantFailover,
            ManagedActionLaneClass::SupportExportUpload,
            RegionTenantKeyModeBetaProfileClass::Offline,
            RegionTenantDrillOutcomeClass::FailedOverToDeclaredFallback,
            "primary_tenant_unreachable",
            "secondary_tenant_declared_fallback",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            "Support-export upload failed over from the primary tenant binding to the declared secondary tenant via the air-gapped courier path. No sibling lane widened.",
            "2026-05-14T22:30:00Z",
            "2026-05-15T03:30:00Z",
            "artifacts/security/m3/region_tenant_drills/tenant_failover_001.json",
        ),
        drill_packet(
            "region-tenant-drill:key-mode-drift-001",
            RegionTenantDrillKindClass::KeyModeDrift,
            ManagedActionLaneClass::AdminPolicyPush,
            RegionTenantKeyModeBetaProfileClass::EnterpriseManaged,
            RegionTenantDrillOutcomeClass::NarrowedThenRecovered,
            "active_key_mode:customer_managed",
            "drifted_to_key_mode:vendor_managed",
            CapabilityAuthorityClass::ApprovalRequiredPerInvocation,
            CapabilityAuthorityClass::PolicyDenied,
            "Admin-policy-push lane briefly observed a vendor-managed key handle during a custody rotation test. The lane closed on its own; sibling lanes stayed at their declared authority and recovered once the customer-managed handle returned.",
            "2026-05-15T02:00:00Z",
            "2026-05-15T02:05:00Z",
            "artifacts/security/m3/region_tenant_drills/key_mode_drift_001.json",
        ),
        drill_packet(
            "region-tenant-drill:key-mode-failover-001",
            RegionTenantDrillKindClass::KeyModeFailover,
            ManagedActionLaneClass::AiInference,
            RegionTenantKeyModeBetaProfileClass::Offline,
            RegionTenantDrillOutcomeClass::FailedOverToDeclaredFallback,
            "active_key_mode:vendor_managed",
            "active_key_mode:byok_user_managed",
            CapabilityAuthorityClass::Allowed,
            CapabilityAuthorityClass::DegradedPreviewOnly,
            "AI inference lane failed over from vendor-managed keys to BYOK user-managed material on the offline profile. Preview-only authority narrows the affected lane; sibling lanes remained at their declared offline authority.",
            "2026-05-14T23:00:00Z",
            "2026-05-15T03:00:00Z",
            "artifacts/security/m3/region_tenant_drills/key_mode_failover_001.json",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn region_row(
    row_id: &str,
    managed_lane: ManagedActionLaneClass,
    profile: RegionTenantKeyModeBetaProfileClass,
    claimed_region_mode: RegionMode,
    active_region_mode: RegionMode,
    residency_mode: ResidencyMode,
    pinning_state: RegionPinningStateClass,
    disclosure: ProcessingLocationDisclosure,
    effective_authority: CapabilityAuthorityClass,
    linked_drill_packet_id: &str,
) -> RegionDisclosureRow {
    RegionDisclosureRow {
        record_kind: REGION_DISCLOSURE_ROW_RECORD_KIND.to_owned(),
        schema_version: REGION_TENANT_KEY_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: REGION_TENANT_KEY_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        managed_lane,
        managed_lane_token: managed_lane.as_str().to_owned(),
        profile,
        profile_token: profile.as_str().to_owned(),
        claimed_region_mode,
        claimed_region_mode_token: claimed_region_mode.as_str().to_owned(),
        active_region_mode,
        active_region_mode_token: active_region_mode.as_str().to_owned(),
        residency_mode,
        residency_mode_token: residency_mode.as_str().to_owned(),
        pinning_state,
        pinning_state_token: pinning_state.as_str().to_owned(),
        disclosure,
        effective_authority,
        effective_authority_token: effective_authority.as_str().to_owned(),
        linked_drill_packet_id: linked_drill_packet_id.to_owned(),
        no_public_endpoint_fallback: true,
        raw_private_material_excluded: true,
        local_editing_preserved: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn tenant_row(
    row_id: &str,
    managed_lane: ManagedActionLaneClass,
    profile: RegionTenantKeyModeBetaProfileClass,
    claimed_tenant_id: &str,
    active_tenant_id: &str,
    workspace_boundary_id: &str,
    boundary_state: TenantBoundaryStateClass,
    disclosed_origin_ref: &str,
    effective_authority: CapabilityAuthorityClass,
    linked_drill_packet_id: &str,
) -> TenantBoundaryRow {
    TenantBoundaryRow {
        record_kind: TENANT_BOUNDARY_ROW_RECORD_KIND.to_owned(),
        schema_version: REGION_TENANT_KEY_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: REGION_TENANT_KEY_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        managed_lane,
        managed_lane_token: managed_lane.as_str().to_owned(),
        profile,
        profile_token: profile.as_str().to_owned(),
        claimed_tenant_id: claimed_tenant_id.to_owned(),
        active_tenant_id: active_tenant_id.to_owned(),
        workspace_boundary_id: workspace_boundary_id.to_owned(),
        boundary_state,
        boundary_state_token: boundary_state.as_str().to_owned(),
        disclosed_origin_ref: disclosed_origin_ref.to_owned(),
        effective_authority,
        effective_authority_token: effective_authority.as_str().to_owned(),
        linked_drill_packet_id: linked_drill_packet_id.to_owned(),
        no_public_endpoint_fallback: true,
        raw_private_material_excluded: true,
        local_editing_preserved: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn key_mode_row(
    row_id: &str,
    managed_lane: ManagedActionLaneClass,
    profile: RegionTenantKeyModeBetaProfileClass,
    claimed_key_mode: KeyMode,
    active_key_mode: KeyMode,
    key_state: KeyModeStateClass,
    custody_origin_ref: &str,
    effective_authority: CapabilityAuthorityClass,
    linked_drill_packet_id: &str,
) -> KeyModeRow {
    KeyModeRow {
        record_kind: KEY_MODE_ROW_RECORD_KIND.to_owned(),
        schema_version: REGION_TENANT_KEY_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: REGION_TENANT_KEY_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        managed_lane,
        managed_lane_token: managed_lane.as_str().to_owned(),
        profile,
        profile_token: profile.as_str().to_owned(),
        claimed_key_mode,
        claimed_key_mode_token: claimed_key_mode.as_str().to_owned(),
        active_key_mode,
        active_key_mode_token: active_key_mode.as_str().to_owned(),
        key_state,
        key_state_token: key_state.as_str().to_owned(),
        custody_origin_ref: custody_origin_ref.to_owned(),
        effective_authority,
        effective_authority_token: effective_authority.as_str().to_owned(),
        linked_drill_packet_id: linked_drill_packet_id.to_owned(),
        no_public_endpoint_fallback: true,
        raw_private_material_excluded: true,
        local_editing_preserved: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn drill_packet(
    drill_packet_id: &str,
    drill_kind: RegionTenantDrillKindClass,
    managed_lane: ManagedActionLaneClass,
    profile: RegionTenantKeyModeBetaProfileClass,
    outcome: RegionTenantDrillOutcomeClass,
    before_state_label: &str,
    after_state_label: &str,
    before_authority: CapabilityAuthorityClass,
    after_authority: CapabilityAuthorityClass,
    explanation: &str,
    started_at: &str,
    resolved_at: &str,
    artifact_ref: &str,
) -> RegionTenantDrillPacket {
    RegionTenantDrillPacket {
        record_kind: REGION_TENANT_DRILL_PACKET_RECORD_KIND.to_owned(),
        schema_version: REGION_TENANT_KEY_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: REGION_TENANT_KEY_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        drill_packet_id: drill_packet_id.to_owned(),
        drill_kind,
        drill_kind_token: drill_kind.as_str().to_owned(),
        axis_token: drill_kind.axis_token().to_owned(),
        managed_lane,
        managed_lane_token: managed_lane.as_str().to_owned(),
        profile,
        profile_token: profile.as_str().to_owned(),
        outcome,
        outcome_token: outcome.as_str().to_owned(),
        before_state_label: before_state_label.to_owned(),
        after_state_label: after_state_label.to_owned(),
        before_authority,
        before_authority_token: before_authority.as_str().to_owned(),
        after_authority,
        after_authority_token: after_authority.as_str().to_owned(),
        explanation: explanation.to_owned(),
        started_at: started_at.to_owned(),
        resolved_at: resolved_at.to_owned(),
        artifact_ref: artifact_ref.to_owned(),
        local_editing_preserved: true,
        sibling_lanes_unwidened: true,
        raw_private_material_excluded: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_with_zero_defects() {
        let page = seeded_region_tenant_key_mode_beta_page();
        validate_region_tenant_key_mode_beta_page(&page).expect("seeded page validates");
        assert!(page.defects.is_empty());
        assert!(!page.region_rows.is_empty());
        assert!(!page.tenant_rows.is_empty());
        assert!(!page.key_mode_rows.is_empty());
        assert!(page.drill_packets.len() >= 3);
        for profile in RegionTenantKeyModeBetaProfileClass::ALL {
            assert!(page
                .summary
                .profiles_present
                .iter()
                .any(|token| token == profile.as_str()));
        }
        for axis in ["region", "tenant", "key_mode"] {
            assert!(page
                .summary
                .drill_axes_present
                .iter()
                .any(|token| token == axis));
        }
    }

    #[test]
    fn claimed_managed_rows_disclose_region_tenant_and_key_mode() {
        let page = seeded_region_tenant_key_mode_beta_page();
        for row in &page.region_rows {
            if claimed_managed_profile(row.profile) {
                assert!(!row.disclosure.region_id.is_empty());
                assert!(!row.disclosure.disclosed_origin_ref.is_empty());
            }
        }
        for row in &page.tenant_rows {
            if claimed_managed_profile(row.profile)
                && row.boundary_state != TenantBoundaryStateClass::NotApplicableLocal
            {
                assert!(!row.active_tenant_id.is_empty());
                assert!(!row.workspace_boundary_id.is_empty());
            }
        }
        for row in &page.key_mode_rows {
            if claimed_managed_profile(row.profile) {
                assert!(!row.active_key_mode.is_unknown());
                assert!(!row.custody_origin_ref.is_empty());
            }
        }
    }

    #[test]
    fn narrowing_states_narrow_only_affected_lane() {
        let mut page = seeded_region_tenant_key_mode_beta_page();
        let baseline_authority_tokens: Vec<String> = page
            .key_mode_rows
            .iter()
            .map(|row| row.effective_authority_token.clone())
            .collect();
        page.region_rows[0].pinning_state = RegionPinningStateClass::DriftedFromClaim;
        page.region_rows[0].pinning_state_token = RegionPinningStateClass::DriftedFromClaim
            .as_str()
            .to_owned();
        page.region_rows[0].effective_authority = CapabilityAuthorityClass::Allowed;
        page.region_rows[0].effective_authority_token =
            CapabilityAuthorityClass::Allowed.as_str().to_owned();
        let defects = audit_region_tenant_key_mode_beta_page(
            &page.region_rows,
            &page.tenant_rows,
            &page.key_mode_rows,
            &page.drill_packets,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == RegionTenantKeyModeBetaDefectKind::MismatchMasksIssue));
        let sibling_tokens: Vec<String> = page
            .key_mode_rows
            .iter()
            .map(|row| row.effective_authority_token.clone())
            .collect();
        assert_eq!(baseline_authority_tokens, sibling_tokens);
    }

    #[test]
    fn drill_packets_cover_region_tenant_and_key_mode_axes() {
        let page = seeded_region_tenant_key_mode_beta_page();
        let axes: BTreeSet<&str> = page
            .drill_packets
            .iter()
            .map(|packet| packet.axis_token.as_str())
            .collect();
        assert!(axes.contains("region"));
        assert!(axes.contains("tenant"));
        assert!(axes.contains("key_mode"));
    }

    #[test]
    fn validator_flags_missing_drill_axis() {
        let mut page = seeded_region_tenant_key_mode_beta_page();
        page.drill_packets
            .retain(|packet| packet.axis_token != "key_mode");
        let defects = audit_region_tenant_key_mode_beta_page(
            &page.region_rows,
            &page.tenant_rows,
            &page.key_mode_rows,
            &page.drill_packets,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RegionTenantKeyModeBetaDefectKind::DrillAxisCoverageMissing));
    }

    #[test]
    fn validator_flags_sibling_widened_drill() {
        let mut page = seeded_region_tenant_key_mode_beta_page();
        page.drill_packets[0].sibling_lanes_unwidened = false;
        let defects = audit_region_tenant_key_mode_beta_page(
            &page.region_rows,
            &page.tenant_rows,
            &page.key_mode_rows,
            &page.drill_packets,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RegionTenantKeyModeBetaDefectKind::DrillSiblingLaneWidened));
    }

    #[test]
    fn validator_flags_public_endpoint_fallback() {
        let mut page = seeded_region_tenant_key_mode_beta_page();
        page.region_rows[0].no_public_endpoint_fallback = false;
        let defects = audit_region_tenant_key_mode_beta_page(
            &page.region_rows,
            &page.tenant_rows,
            &page.key_mode_rows,
            &page.drill_packets,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RegionTenantKeyModeBetaDefectKind::HiddenPublicEndpointFallback));
    }

    #[test]
    fn validator_flags_undisclosed_managed_region() {
        let mut page = seeded_region_tenant_key_mode_beta_page();
        let row = page
            .region_rows
            .iter_mut()
            .find(|row| claimed_managed_profile(row.profile))
            .expect("seeded managed row");
        row.disclosure.region_id.clear();
        let defects = audit_region_tenant_key_mode_beta_page(
            &page.region_rows,
            &page.tenant_rows,
            &page.key_mode_rows,
            &page.drill_packets,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RegionTenantKeyModeBetaDefectKind::ClaimedManagedRegionUndisclosed));
    }

    #[test]
    fn support_export_round_trip_is_metadata_safe() {
        let page = seeded_region_tenant_key_mode_beta_page();
        let export = RegionTenantKeyModeBetaSupportExport::from_page(
            "region-tenant-key-mode:support-export:001",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.no_public_endpoint_fallback_invariant);
        assert!(export.local_editing_preserved_invariant);
        assert!(export.defect_kinds_present.is_empty());
        assert!(export.defect_counts_by_kind.is_empty());
    }
}
