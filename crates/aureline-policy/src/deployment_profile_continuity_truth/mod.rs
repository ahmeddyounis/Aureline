//! Canonical deployment-profile continuity truth for managed, self-hosted,
//! mirrored, and offline-capable product claims.
//!
//! This module publishes one metadata-safe packet that About, Help,
//! diagnostics, service-health, admin, and support-export surfaces can reuse
//! when they need to answer the same deployment-boundary questions:
//!
//! 1. Which deployment profile is active, which tenant or org boundary applies,
//!    which region or locality label is in force, and which control-plane and
//!    data-plane services are actually in play.
//! 2. Which still-vendor-hosted control-plane, content, telemetry, or AI
//!    dependencies remain on profiles that market a self-hosted, mirrored, or
//!    air-gapped posture.
//! 3. Which local-safe workflows remain usable during control-plane degradation,
//!    mirror staleness, offline grace, or reauthentication gates.
//! 4. Which mirrored or offline artifact families are current, stale, or
//!    inspect-only, and what stays blocked until they refresh.
//! 5. Whether About, Help, diagnostics, admin, service-health, and
//!    support-export surfaces are reusing the same fact source instead of
//!    cloning status prose.
//!
//! The packet is intentionally metadata-only. It carries closed-vocabulary
//! profile and mirror-state tokens, export-safe labels, UTC timestamps, and
//! opaque refs only. Raw hostnames, raw tenant identifiers, raw KMS handles,
//! raw trust roots, raw support payloads, and raw secret material never cross
//! this boundary.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::finalize_signed_policy_bundle_offline_entitlement_and_mirror::BundleDeliverySourceClass;
use crate::stabilize_deployment_and_residency_truth::{
    DeploymentProfileClass, MirrorOfflineStateClass, TenantOrgScopeClass,
};

#[cfg(test)]
mod tests;

/// Schema version carried on every record in this module.
pub const DEPLOYMENT_PROFILE_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const DEPLOYMENT_PROFILE_CONTINUITY_SHARED_CONTRACT_REF: &str =
    "policy:deployment_profile_continuity_truth:v1";

/// Record-kind tag for [`DeploymentProfileContinuityPage`] payloads.
pub const DEPLOYMENT_PROFILE_CONTINUITY_PAGE_RECORD_KIND: &str =
    "policy_deployment_profile_continuity_page_record";

/// Record-kind tag for [`DeploymentProfileContinuityDefect`] payloads.
pub const DEPLOYMENT_PROFILE_CONTINUITY_DEFECT_RECORD_KIND: &str =
    "policy_deployment_profile_continuity_defect_record";

/// Record-kind tag for [`DeploymentProfileContinuitySupportExport`] payloads.
pub const DEPLOYMENT_PROFILE_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_deployment_profile_continuity_support_export_record";

/// Repo-relative path of the canonical reviewer doc for this lane.
pub const DEPLOYMENT_PROFILE_CONTINUITY_DOC_REF: &str =
    "docs/policy/deployment_profile_continuity_truth.md";

/// Repo-relative path of the checked-in artifact summary for this lane.
pub const DEPLOYMENT_PROFILE_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/policy/deployment_profile_continuity_truth.md";

/// Product surface that must reuse the canonical deployment facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactFamilyClass {
    /// Deployment summary cards that explain the running operating mode.
    DeploymentSummary,
    /// Residual-dependency rows that keep self-hosted and sovereign claims honest.
    ResidualDependency,
    /// Mirror-freshness and offline-artifact disclosure cards.
    MirrorFreshness,
    /// Local-safe fallback cards that explain degraded continuity.
    LocalSafeFallback,
}

impl FactFamilyClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeploymentSummary => "deployment_summary",
            Self::ResidualDependency => "residual_dependency",
            Self::MirrorFreshness => "mirror_freshness",
            Self::LocalSafeFallback => "local_safe_fallback",
        }
    }

    /// All required fact families in canonical order.
    pub const ALL: [Self; 4] = [
        Self::DeploymentSummary,
        Self::ResidualDependency,
        Self::MirrorFreshness,
        Self::LocalSafeFallback,
    ];
}

/// Whether a service belongs to the control plane or the data plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaneClass {
    /// Identity, policy, catalog, mirror, relay, or other control metadata services.
    ControlPlane,
    /// Local editing, runtime, artifact-bytes, session, or other execution-path services.
    DataPlane,
}

impl PlaneClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlPlane => "control_plane",
            Self::DataPlane => "data_plane",
        }
    }
}

/// Hosting posture of a service or residual dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostingPostureClass {
    /// Entirely local to the desktop or headless client.
    LocalOnly,
    /// Customer-operated service or tenant-owned infrastructure.
    CustomerHosted,
    /// Vendor-operated Aureline service.
    VendorHosted,
    /// Non-Aureline third-party service.
    ThirdPartyHosted,
    /// Served from an approved mirror snapshot.
    MirrorBacked,
    /// Served from a signed offline snapshot.
    OfflineSnapshot,
}

impl HostingPostureClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::CustomerHosted => "customer_hosted",
            Self::VendorHosted => "vendor_hosted",
            Self::ThirdPartyHosted => "third_party_hosted",
            Self::MirrorBacked => "mirror_backed",
            Self::OfflineSnapshot => "offline_snapshot",
        }
    }

    /// True when this posture still leaves the customer boundary.
    pub const fn is_external_dependency(self) -> bool {
        matches!(self, Self::VendorHosted | Self::ThirdPartyHosted)
    }
}

/// Dependency family that remains relevant outside the core local desktop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidualDependencyClass {
    /// Identity, policy, quotas, tenancy, or other control-plane authority.
    ControlPlane,
    /// Documentation, catalog, model, extension, or other content delivery.
    Content,
    /// Telemetry, symbols, crash, or support upload path.
    Telemetry,
    /// Hosted AI routing, moderation, or model execution path.
    Ai,
}

impl ResidualDependencyClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlPlane => "control_plane",
            Self::Content => "content",
            Self::Telemetry => "telemetry",
            Self::Ai => "ai",
        }
    }
}

/// Freshness state for mirrored or offline artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessStateClass {
    /// Verified current within the declared freshness budget.
    Current,
    /// Stale but still inside an explicit grace window.
    StaleWithinGrace,
    /// Stale enough that refresh is required before some managed actions resume.
    StaleNeedsRefresh,
    /// Intentionally offline, with freshness frozen to the imported snapshot.
    OfflineSnapshot,
    /// Freshness could not be established honestly.
    Unknown,
}

impl FreshnessStateClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleWithinGrace => "stale_within_grace",
            Self::StaleNeedsRefresh => "stale_needs_refresh",
            Self::OfflineSnapshot => "offline_snapshot",
            Self::Unknown => "unknown",
        }
    }

    /// True when the card must explain blocked or narrowed actions.
    pub const fn blocks_without_refresh(self) -> bool {
        matches!(self, Self::StaleNeedsRefresh | Self::Unknown)
    }
}

/// Signer continuity posture for mirrored or offline artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignerContinuityClass {
    /// Signer continuity is current and verified.
    VerifiedCurrent,
    /// Signer continuity is verified through an approved mirror.
    VerifiedMirror,
    /// Signer continuity is verified against an offline root.
    VerifiedOffline,
    /// Signer continuity is known, but a rotation review is required.
    RotationReviewRequired,
    /// Signer continuity could not be proven.
    Unverified,
}

impl SignerContinuityClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCurrent => "verified_current",
            Self::VerifiedMirror => "verified_mirror",
            Self::VerifiedOffline => "verified_offline",
            Self::RotationReviewRequired => "rotation_review_required",
            Self::Unverified => "unverified",
        }
    }
}

/// Local-safe continuity state for a deployment profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalSafeStateClass {
    /// No active degradation is affecting local-safe work.
    Healthy,
    /// Control-plane state is degraded while local-safe work continues.
    ControlPlaneDegraded,
    /// Fresh authorization is required before managed actions may resume.
    ReauthRequired,
    /// A mirror or offline snapshot is in use and local-safe work continues.
    OfflineLocalSafe,
    /// Mirror freshness or signer continuity is stale enough to narrow managed work.
    MirrorStale,
}

impl LocalSafeStateClass {
    /// Stable token recorded on serialized rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::ControlPlaneDegraded => "control_plane_degraded",
            Self::ReauthRequired => "reauth_required",
            Self::OfflineLocalSafe => "offline_local_safe",
            Self::MirrorStale => "mirror_stale",
        }
    }

    /// True when the card must explain what remains usable locally.
    pub const fn requires_explicit_local_safe_copy(self) -> bool {
        !matches!(self, Self::Healthy)
    }
}

/// Stability qualification tier for the overall packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfileContinuityQualificationClass {
    /// Every required disclosure and parity condition is satisfied.
    Stable,
    /// The packet remains claimable, but one or more disclosures are incomplete.
    Beta,
    /// Coverage gaps prevent the packet from claiming the lane beyond preview.
    Preview,
    /// The packet would overstate a sovereign or offline boundary and is withdrawn.
    Withdrawn,
}

impl DeploymentProfileContinuityQualificationClass {
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

/// Typed reason the packet narrowed below stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfileContinuityNarrowReasonClass {
    /// No narrowing is active.
    NotNarrowed,
    /// One or more claimed deployment profiles do not have complete card coverage.
    MissingProfileCoverage,
    /// A deployment summary card omitted required boundary or plane truth.
    SummaryCardTruthMissing,
    /// Residual vendor-hosted or third-party dependencies are not fully disclosed.
    ResidualDependencyDisclosureMissing,
    /// A claimed self-hosted or offline boundary would overstate reality.
    SovereignBoundaryOverclaimed,
    /// A degraded profile omitted explicit local-safe continuity guidance.
    LocalSafeFallbackTruthMissing,
    /// Mirror or offline freshness truth is absent or too vague.
    MirrorFreshnessTruthMissing,
    /// Required surfaces do not all reuse the same canonical fact source.
    SurfaceFactReuseMissing,
}

impl DeploymentProfileContinuityNarrowReasonClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::MissingProfileCoverage => "missing_profile_coverage",
            Self::SummaryCardTruthMissing => "summary_card_truth_missing",
            Self::ResidualDependencyDisclosureMissing => "residual_dependency_disclosure_missing",
            Self::SovereignBoundaryOverclaimed => "sovereign_boundary_overclaimed",
            Self::LocalSafeFallbackTruthMissing => "local_safe_fallback_truth_missing",
            Self::MirrorFreshnessTruthMissing => "mirror_freshness_truth_missing",
            Self::SurfaceFactReuseMissing => "surface_fact_reuse_missing",
        }
    }

    /// True when this reason withdraws the packet immediately.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::SovereignBoundaryOverclaimed)
    }

    /// True when this reason narrows the packet to preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(self, Self::MissingProfileCoverage)
    }
}

/// Visibility declaration for required deployment-truth consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuitySurfaceVisibility {
    /// True when About reuses the fact.
    pub about: bool,
    /// True when Help reuses the fact.
    pub help: bool,
    /// True when diagnostics reuse the fact.
    pub diagnostics: bool,
    /// True when service-health surfaces reuse the fact.
    pub service_health: bool,
    /// True when admin surfaces reuse the fact.
    pub admin: bool,
    /// True when support exports reuse the fact.
    pub support_export: bool,
}

impl ContinuitySurfaceVisibility {
    /// Returns a visibility declaration with every required surface enabled.
    pub const fn all_required() -> Self {
        Self {
            about: true,
            help: true,
            diagnostics: true,
            service_health: true,
            admin: true,
            support_export: true,
        }
    }

    /// True when all required surfaces are covered.
    pub const fn all_visible(&self) -> bool {
        self.about
            && self.help
            && self.diagnostics
            && self.service_health
            && self.admin
            && self.support_export
    }
}

/// One named service participating in the deployment summary card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentServiceFact {
    /// Opaque service reference safe for exports and support packets.
    pub service_ref: String,
    /// Reviewable label naming the service role.
    pub service_label: String,
    /// Plane this service belongs to.
    pub plane: PlaneClass,
    /// Stable token for [`Self::plane`].
    pub plane_token: String,
    /// Hosting posture for this service.
    pub hosting_posture: HostingPostureClass,
    /// Stable token for [`Self::hosting_posture`].
    pub hosting_posture_token: String,
    /// Current health or continuity status label.
    pub status_label: String,
    /// Export-safe note describing why this service matters to continuity.
    pub continuity_note: String,
}

/// Deployment summary card for one claimed deployment profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentSummaryCard {
    /// Opaque card identifier.
    pub card_id: String,
    /// Deployment profile this card describes.
    pub profile: DeploymentProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Tenant or org scope for this card.
    pub tenant_scope: TenantOrgScopeClass,
    /// Stable token for [`Self::tenant_scope`].
    pub tenant_scope_token: String,
    /// Export-safe tenant or org label, where relevant.
    pub tenant_org_label: String,
    /// Export-safe region or locality label.
    pub region_label: String,
    /// Key-ownership label shown in About, admin, and support surfaces.
    pub key_ownership_label: String,
    /// Mirror or offline state for the profile.
    pub mirror_offline_state: MirrorOfflineStateClass,
    /// Stable token for [`Self::mirror_offline_state`].
    pub mirror_offline_state_token: String,
    /// Distribution sources currently in play for the profile's governed artifacts.
    pub delivery_sources: Vec<BundleDeliverySourceClass>,
    /// Stable tokens for [`Self::delivery_sources`].
    pub delivery_source_tokens: Vec<String>,
    /// Exact control-plane services involved in the claim.
    pub control_plane_services: Vec<DeploymentServiceFact>,
    /// Exact data-plane services involved in the claim.
    pub data_plane_services: Vec<DeploymentServiceFact>,
    /// Opaque refs for all vendor-bound or third-party dependencies that require disclosure.
    pub vendor_dependency_refs: Vec<String>,
    /// UTC timestamp of the last successful control-plane sync or equivalent snapshot verification.
    pub last_control_plane_sync: String,
    /// Plain-language local-safe baseline quoted across surfaces.
    pub local_safe_baseline: Vec<String>,
    /// Required surface coverage for this card.
    pub surface_visibility: ContinuitySurfaceVisibility,
}

/// Residual vendor or third-party dependency disclosure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidualDependencyRow {
    /// Opaque row identifier.
    pub row_id: String,
    /// Deployment profile the dependency belongs to.
    pub profile: DeploymentProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Opaque dependency reference matched against summary-card disclosure refs.
    pub dependency_ref: String,
    /// Reviewable capability label for the dependency.
    pub feature_label: String,
    /// Dependency family for this row.
    pub dependency_class: ResidualDependencyClass,
    /// Stable token for [`Self::dependency_class`].
    pub dependency_class_token: String,
    /// Hosting posture for the dependency.
    pub host_posture: HostingPostureClass,
    /// Stable token for [`Self::host_posture`].
    pub host_posture_token: String,
    /// Failure consequence when this dependency is unreachable.
    pub failure_consequence: String,
    /// Explicit local-safe or customer-bounded alternative that remains available.
    pub local_safe_alternative: String,
    /// Action label naming how to disable or replace the dependency.
    pub disable_or_alternative_action: String,
    /// Required surface coverage for this row.
    pub surface_visibility: ContinuitySurfaceVisibility,
}

/// Mirror or offline freshness disclosure card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorFreshnessCard {
    /// Opaque card identifier.
    pub card_id: String,
    /// Deployment profile this artifact family belongs to.
    pub profile: DeploymentProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Reviewable artifact-family label such as policy bundle or docs pack.
    pub artifact_label: String,
    /// Delivery source for the mirrored or offline artifact.
    pub delivery_source: BundleDeliverySourceClass,
    /// Stable token for [`Self::delivery_source`].
    pub delivery_source_token: String,
    /// Reviewable source label such as internal mirror or signed offline bundle.
    pub source_label: String,
    /// Freshness posture of the artifact.
    pub freshness_state: FreshnessStateClass,
    /// Stable token for [`Self::freshness_state`].
    pub freshness_state_token: String,
    /// Signer continuity posture of the artifact.
    pub signer_continuity: SignerContinuityClass,
    /// Stable token for [`Self::signer_continuity`].
    pub signer_continuity_token: String,
    /// UTC timestamp of the last verification event.
    pub last_verified_at: String,
    /// Functions that remain usable from cache or snapshot even if the artifact is stale.
    pub cached_usable_now: Vec<String>,
    /// Functions that stay blocked or narrowed until refresh succeeds.
    pub blocked_until_refresh: Vec<String>,
    /// User-facing action label for refreshing or reviewing the artifact.
    pub refresh_action_label: String,
    /// Required surface coverage for this card.
    pub surface_visibility: ContinuitySurfaceVisibility,
}

/// Local-safe fallback and degraded-continuity card for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalSafeFallbackCard {
    /// Opaque card identifier.
    pub card_id: String,
    /// Deployment profile this fallback state belongs to.
    pub profile: DeploymentProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Active local-safe continuity state.
    pub state: LocalSafeStateClass,
    /// Stable token for [`Self::state`].
    pub state_token: String,
    /// Explicit list of workflows that remain usable immediately.
    pub usable_now: Vec<String>,
    /// Cached or stale truth that stays inspectable while degraded.
    pub cached_or_stale: Vec<String>,
    /// Actions that stay blocked or narrowed until refresh or reauth succeeds.
    pub blocked_until_refresh: Vec<String>,
    /// Safe next-step actions offered by the surface.
    pub next_steps: Vec<String>,
    /// Required surface coverage for this card.
    pub surface_visibility: ContinuitySurfaceVisibility,
}

/// Cross-surface reuse declaration for one fact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceReuseRow {
    /// Fact family being reused.
    pub fact_family: FactFamilyClass,
    /// Stable token for [`Self::fact_family`].
    pub fact_family_token: String,
    /// Reviewable label naming the canonical source.
    pub canonical_source_label: String,
    /// Opaque ref naming the canonical source packet or artifact.
    pub canonical_source_ref: String,
    /// Required surface coverage for this fact family.
    pub surface_visibility: ContinuitySurfaceVisibility,
}

/// Full auditable input for deployment-profile continuity truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentProfileContinuityInput {
    /// Deployment profiles the packet claims coverage for.
    pub claimed_profiles: Vec<DeploymentProfileClass>,
    /// The currently running deployment profile.
    pub current_profile: DeploymentProfileClass,
    /// Canonical deployment summary cards.
    pub deployment_summary_cards: Vec<DeploymentSummaryCard>,
    /// Residual-dependency disclosure rows.
    pub residual_dependency_rows: Vec<ResidualDependencyRow>,
    /// Mirror-freshness and offline-artifact disclosure cards.
    pub mirror_freshness_cards: Vec<MirrorFreshnessCard>,
    /// Local-safe fallback cards.
    pub local_safe_fallback_cards: Vec<LocalSafeFallbackCard>,
    /// Cross-surface fact-reuse declarations.
    pub surface_reuse_rows: Vec<SurfaceReuseRow>,
}

/// Aggregate summary for a continuity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentProfileContinuitySummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable token for the current running deployment profile.
    pub current_profile_token: String,
    /// Overall qualification for the packet.
    pub overall_qualification_token: String,
    /// Number of claimed deployment profiles.
    pub claimed_profile_count: usize,
    /// Number of deployment summary cards.
    pub deployment_summary_card_count: usize,
    /// Number of residual-dependency rows.
    pub residual_dependency_row_count: usize,
    /// Number of mirror-freshness cards.
    pub mirror_freshness_card_count: usize,
    /// Number of local-safe fallback cards.
    pub local_safe_fallback_card_count: usize,
    /// Number of surface-reuse rows whose visibility is complete.
    pub complete_surface_reuse_row_count: usize,
    /// Number of claimed profiles currently in a stale, mirrored, or offline posture.
    pub stale_or_offline_profile_count: usize,
    /// Number of defects recorded for the packet.
    pub defect_count: usize,
}

impl DeploymentProfileContinuitySummary {
    fn from_input_and_defects(
        input: &DeploymentProfileContinuityInput,
        defects: &[DeploymentProfileContinuityDefect],
    ) -> Self {
        let qualification = if defects
            .iter()
            .any(|defect| defect.narrow_reason.is_withdrawal_reason())
        {
            DeploymentProfileContinuityQualificationClass::Withdrawn
        } else if defects
            .iter()
            .any(|defect| defect.narrow_reason.is_preview_reason())
        {
            DeploymentProfileContinuityQualificationClass::Preview
        } else if defects.is_empty() {
            DeploymentProfileContinuityQualificationClass::Stable
        } else {
            DeploymentProfileContinuityQualificationClass::Beta
        };

        Self {
            record_kind: "policy_deployment_profile_continuity_summary_record".to_owned(),
            schema_version: DEPLOYMENT_PROFILE_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: DEPLOYMENT_PROFILE_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
            current_profile_token: input.current_profile.as_str().to_owned(),
            overall_qualification_token: qualification.as_str().to_owned(),
            claimed_profile_count: input.claimed_profiles.len(),
            deployment_summary_card_count: input.deployment_summary_cards.len(),
            residual_dependency_row_count: input.residual_dependency_rows.len(),
            mirror_freshness_card_count: input.mirror_freshness_cards.len(),
            local_safe_fallback_card_count: input.local_safe_fallback_cards.len(),
            complete_surface_reuse_row_count: input
                .surface_reuse_rows
                .iter()
                .filter(|row| row.surface_visibility.all_visible())
                .count(),
            stale_or_offline_profile_count: input
                .deployment_summary_cards
                .iter()
                .filter(|card| profile_needs_mirror_freshness(card.mirror_offline_state))
                .count(),
            defect_count: defects.len(),
        }
    }
}

/// Typed defect emitted by the continuity audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentProfileContinuityDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque defect identifier.
    pub defect_id: String,
    /// Typed narrow reason.
    pub narrow_reason: DeploymentProfileContinuityNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Opaque source ref or profile token that triggered the defect.
    pub source: String,
    /// Export-safe explanation of the defect.
    pub note: String,
}

impl DeploymentProfileContinuityDefect {
    fn new(
        narrow_reason: DeploymentProfileContinuityNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: DEPLOYMENT_PROFILE_CONTINUITY_DEFECT_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_PROFILE_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: DEPLOYMENT_PROFILE_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:deployment-profile-continuity:{}:{}",
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

/// Canonical proof packet for deployment-boundary, residual-dependency,
/// mirror-freshness, and local-safe continuity truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentProfileContinuityPage {
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
    pub summary: DeploymentProfileContinuitySummary,
    /// Typed defects for the packet.
    pub defects: Vec<DeploymentProfileContinuityDefect>,
    /// The audited input embedded as evidence.
    pub input: DeploymentProfileContinuityInput,
}

impl DeploymentProfileContinuityPage {
    /// Builds a continuity page from the supplied input.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: DeploymentProfileContinuityInput,
    ) -> Self {
        let defects = audit_deployment_profile_continuity_input(&input);
        let summary = DeploymentProfileContinuitySummary::from_input_and_defects(&input, &defects);
        Self {
            record_kind: DEPLOYMENT_PROFILE_CONTINUITY_PAGE_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_PROFILE_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: DEPLOYMENT_PROFILE_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            defects,
            input,
        }
    }

    /// True when the packet qualifies stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == DeploymentProfileContinuityQualificationClass::Stable.as_str()
    }

    /// True when every claimed profile has exactly one deployment summary card.
    pub fn covers_claimed_profiles(&self) -> bool {
        required_profile_coverage_is_complete(&self.input)
    }

    /// True when every residual dependency required by a summary card is disclosed.
    pub fn residual_vendor_dependencies_disclosed(&self) -> bool {
        residual_dependency_coverage_is_complete(&self.input)
    }

    /// True when every degraded or offline profile has explicit local-safe fallback truth.
    pub fn local_safe_fallback_truth_complete(&self) -> bool {
        local_safe_fallback_truth_is_complete(&self.input)
    }

    /// True when every mirror-backed or offline profile has mirror-freshness truth.
    pub fn mirror_freshness_truth_complete(&self) -> bool {
        mirror_freshness_truth_is_complete(&self.input)
    }

    /// True when every required surface reuses the same canonical fact source.
    pub fn all_required_surfaces_reuse_facts(&self) -> bool {
        surface_fact_reuse_is_complete(&self.input)
    }
}

/// Support-export wrapper for the continuity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentProfileContinuitySupportExport {
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
    /// The continuity packet embedded as evidence.
    pub page: DeploymentProfileContinuityPage,
    /// Typed narrow reasons present in the embedded packet.
    pub narrow_reasons_present: Vec<DeploymentProfileContinuityNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl DeploymentProfileContinuitySupportExport {
    /// Wraps a continuity page inside a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: DeploymentProfileContinuityPage,
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
            record_kind: DEPLOYMENT_PROFILE_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_PROFILE_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: DEPLOYMENT_PROFILE_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

/// Re-runs the continuity audit over the embedded input.
pub fn audit_deployment_profile_continuity_page(
    page: &DeploymentProfileContinuityPage,
) -> Vec<DeploymentProfileContinuityDefect> {
    audit_deployment_profile_continuity_input(&page.input)
}

/// Validates a continuity page and returns `Ok(())` when the audit is clean.
pub fn validate_deployment_profile_continuity_page(
    page: &DeploymentProfileContinuityPage,
) -> Result<(), Vec<DeploymentProfileContinuityDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

/// Returns a seeded stable continuity page that covers the canonical deployment profiles.
pub fn seeded_deployment_profile_continuity_page() -> DeploymentProfileContinuityPage {
    DeploymentProfileContinuityPage::new(
        "policy:deployment-profile-continuity:seeded",
        "Deployment profile continuity truth packet",
        "2026-06-01T00:00:00Z",
        seeded_deployment_profile_continuity_input(),
    )
}

/// Returns the seeded input used by the canonical continuity page.
pub fn seeded_deployment_profile_continuity_input() -> DeploymentProfileContinuityInput {
    DeploymentProfileContinuityInput {
        claimed_profiles: vec![
            DeploymentProfileClass::IndividualLocal,
            DeploymentProfileClass::ManagedCloud,
            DeploymentProfileClass::EnterpriseOnline,
            DeploymentProfileClass::SelfHosted,
            DeploymentProfileClass::AirGapped,
        ],
        current_profile: DeploymentProfileClass::SelfHosted,
        deployment_summary_cards: seeded_summary_cards(),
        residual_dependency_rows: seeded_residual_dependencies(),
        mirror_freshness_cards: seeded_mirror_freshness_cards(),
        local_safe_fallback_cards: seeded_local_safe_fallback_cards(),
        surface_reuse_rows: seeded_surface_reuse_rows(),
    }
}

fn audit_deployment_profile_continuity_input(
    input: &DeploymentProfileContinuityInput,
) -> Vec<DeploymentProfileContinuityDefect> {
    let mut defects = Vec::new();

    if !required_profile_coverage_is_complete(input) {
        defects.push(DeploymentProfileContinuityDefect::new(
            DeploymentProfileContinuityNarrowReasonClass::MissingProfileCoverage,
            "claimed_profiles",
            "every claimed deployment profile must have exactly one deployment summary card and one local-safe fallback card",
        ));
    }

    let current_profile_present = input
        .deployment_summary_cards
        .iter()
        .any(|card| card.profile == input.current_profile);
    if !current_profile_present {
        defects.push(DeploymentProfileContinuityDefect::new(
            DeploymentProfileContinuityNarrowReasonClass::MissingProfileCoverage,
            input.current_profile.as_str(),
            "the current running deployment profile is not represented by a deployment summary card",
        ));
    }

    for card in &input.deployment_summary_cards {
        if card.tenant_scope.requires_sign_out_scope_declaration()
            && card.tenant_org_label.is_empty()
        {
            defects.push(DeploymentProfileContinuityDefect::new(
                DeploymentProfileContinuityNarrowReasonClass::SummaryCardTruthMissing,
                card.profile.as_str(),
                "tenanted deployment cards must name the tenant or organization boundary",
            ));
        }
        if card.region_label.is_empty()
            || card.key_ownership_label.is_empty()
            || card.last_control_plane_sync.is_empty()
            || card.local_safe_baseline.is_empty()
            || card.control_plane_services.is_empty()
            || card.data_plane_services.is_empty()
        {
            defects.push(DeploymentProfileContinuityDefect::new(
                DeploymentProfileContinuityNarrowReasonClass::SummaryCardTruthMissing,
                card.profile.as_str(),
                "deployment summary cards must declare region, key ownership, last control-plane sync, local-safe baseline, and exact control-plane plus data-plane services",
            ));
        }
        if !card.surface_visibility.all_visible() {
            defects.push(DeploymentProfileContinuityDefect::new(
                DeploymentProfileContinuityNarrowReasonClass::SurfaceFactReuseMissing,
                card.card_id.clone(),
                "deployment summary card truth must be reused by About, Help, diagnostics, service-health, admin, and support-export surfaces",
            ));
        }
        if card
            .control_plane_services
            .iter()
            .any(|service| service.plane != PlaneClass::ControlPlane)
            || card
                .data_plane_services
                .iter()
                .any(|service| service.plane != PlaneClass::DataPlane)
        {
            defects.push(DeploymentProfileContinuityDefect::new(
                DeploymentProfileContinuityNarrowReasonClass::SummaryCardTruthMissing,
                card.card_id.clone(),
                "control-plane and data-plane services must remain explicitly separated on deployment summary cards",
            ));
        }
    }

    let residual_refs_by_profile = residual_refs_by_profile(input);
    for card in &input.deployment_summary_cards {
        let disclosed = residual_refs_by_profile
            .get(&card.profile)
            .cloned()
            .unwrap_or_default();
        let missing: Vec<_> = card
            .vendor_dependency_refs
            .iter()
            .filter(|dependency_ref| !disclosed.contains(*dependency_ref))
            .cloned()
            .collect();
        if !missing.is_empty() {
            let reason = if card.profile.claims_sovereignty() {
                DeploymentProfileContinuityNarrowReasonClass::SovereignBoundaryOverclaimed
            } else {
                DeploymentProfileContinuityNarrowReasonClass::ResidualDependencyDisclosureMissing
            };
            defects.push(DeploymentProfileContinuityDefect::new(
                reason,
                card.profile.as_str(),
                format!(
                    "profile '{}' does not disclose all vendor or third-party dependencies; missing refs: {}",
                    card.profile.as_str(),
                    missing.join(", ")
                ),
            ));
        }
    }

    if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_withdrawal_reason())
    {
        return defects;
    }

    for row in &input.residual_dependency_rows {
        if !row.host_posture.is_external_dependency()
            || row.feature_label.is_empty()
            || row.failure_consequence.is_empty()
            || row.local_safe_alternative.is_empty()
            || row.disable_or_alternative_action.is_empty()
        {
            defects.push(DeploymentProfileContinuityDefect::new(
                DeploymentProfileContinuityNarrowReasonClass::ResidualDependencyDisclosureMissing,
                row.row_id.clone(),
                "residual-dependency rows must name an external dependency, the failure consequence, the local-safe alternative, and the disable or replacement path",
            ));
        }
        if !row.surface_visibility.all_visible() {
            defects.push(DeploymentProfileContinuityDefect::new(
                DeploymentProfileContinuityNarrowReasonClass::SurfaceFactReuseMissing,
                row.row_id.clone(),
                "residual-dependency rows must be reused by About, Help, diagnostics, service-health, admin, and support-export surfaces",
            ));
        }
    }

    if !mirror_freshness_truth_is_complete(input) {
        defects.push(DeploymentProfileContinuityDefect::new(
            DeploymentProfileContinuityNarrowReasonClass::MirrorFreshnessTruthMissing,
            "mirror_freshness",
            "mirror-backed, offline-grace, and air-gapped profiles must declare mirror freshness, signer continuity, blocked actions, and refresh guidance",
        ));
    }

    for card in &input.mirror_freshness_cards {
        if card.artifact_label.is_empty()
            || card.source_label.is_empty()
            || card.last_verified_at.is_empty()
            || card.refresh_action_label.is_empty()
            || (card.freshness_state.blocks_without_refresh()
                && card.blocked_until_refresh.is_empty())
        {
            defects.push(DeploymentProfileContinuityDefect::new(
                DeploymentProfileContinuityNarrowReasonClass::MirrorFreshnessTruthMissing,
                card.card_id.clone(),
                "mirror-freshness cards must name the artifact, source, verification time, refresh path, and blocked actions when freshness is insufficient",
            ));
        }
        if !card.surface_visibility.all_visible() {
            defects.push(DeploymentProfileContinuityDefect::new(
                DeploymentProfileContinuityNarrowReasonClass::SurfaceFactReuseMissing,
                card.card_id.clone(),
                "mirror-freshness cards must be reused by About, Help, diagnostics, service-health, admin, and support-export surfaces",
            ));
        }
    }

    if !local_safe_fallback_truth_is_complete(input) {
        defects.push(DeploymentProfileContinuityDefect::new(
            DeploymentProfileContinuityNarrowReasonClass::LocalSafeFallbackTruthMissing,
            "local_safe_fallback",
            "every degraded, reauth-gated, mirrored, or offline profile must explain what remains usable locally, what is cached or stale, and what stays blocked until refresh succeeds",
        ));
    }

    for card in &input.local_safe_fallback_cards {
        let missing_local_safe_truth = card.state.requires_explicit_local_safe_copy()
            && (card.usable_now.is_empty()
                || (card.cached_or_stale.is_empty() && card.blocked_until_refresh.is_empty())
                || card.next_steps.is_empty());
        if missing_local_safe_truth {
            defects.push(DeploymentProfileContinuityDefect::new(
                DeploymentProfileContinuityNarrowReasonClass::LocalSafeFallbackTruthMissing,
                card.card_id.clone(),
                "degraded local-safe cards must name usable workflows, cached or stale truth, blocked actions, and safe next steps",
            ));
        }
        if !card.surface_visibility.all_visible() {
            defects.push(DeploymentProfileContinuityDefect::new(
                DeploymentProfileContinuityNarrowReasonClass::SurfaceFactReuseMissing,
                card.card_id.clone(),
                "local-safe fallback cards must be reused by About, Help, diagnostics, service-health, admin, and support-export surfaces",
            ));
        }
    }

    if !surface_fact_reuse_is_complete(input) {
        defects.push(DeploymentProfileContinuityDefect::new(
            DeploymentProfileContinuityNarrowReasonClass::SurfaceFactReuseMissing,
            "surface_reuse",
            "deployment summary, residual dependency, mirror freshness, and local-safe fallback facts must each declare one canonical source reused by all required surfaces",
        ));
    }

    defects
}

fn required_profile_coverage_is_complete(input: &DeploymentProfileContinuityInput) -> bool {
    let summary_profiles: BTreeSet<_> = input
        .deployment_summary_cards
        .iter()
        .map(|card| card.profile)
        .collect();
    let fallback_profiles: BTreeSet<_> = input
        .local_safe_fallback_cards
        .iter()
        .map(|card| card.profile)
        .collect();
    let claimed_profiles: BTreeSet<_> = input.claimed_profiles.iter().copied().collect();

    summary_profiles == claimed_profiles
        && fallback_profiles == claimed_profiles
        && input.deployment_summary_cards.len() == claimed_profiles.len()
        && input.local_safe_fallback_cards.len() == claimed_profiles.len()
}

fn residual_refs_by_profile(
    input: &DeploymentProfileContinuityInput,
) -> BTreeMap<DeploymentProfileClass, BTreeSet<String>> {
    let mut refs = BTreeMap::new();
    for row in &input.residual_dependency_rows {
        refs.entry(row.profile)
            .or_insert_with(BTreeSet::new)
            .insert(row.dependency_ref.clone());
    }
    refs
}

fn residual_dependency_coverage_is_complete(input: &DeploymentProfileContinuityInput) -> bool {
    let disclosed = residual_refs_by_profile(input);
    input.deployment_summary_cards.iter().all(|card| {
        let disclosed_refs = disclosed.get(&card.profile).cloned().unwrap_or_default();
        card.vendor_dependency_refs
            .iter()
            .all(|dependency_ref| disclosed_refs.contains(dependency_ref))
    })
}

fn profile_needs_mirror_freshness(state: MirrorOfflineStateClass) -> bool {
    matches!(
        state,
        MirrorOfflineStateClass::OnlineMirrorOnly
            | MirrorOfflineStateClass::OfflineGracePreserved
            | MirrorOfflineStateClass::OfflineAirGapped
    )
}

fn mirror_freshness_truth_is_complete(input: &DeploymentProfileContinuityInput) -> bool {
    let freshness_profiles: BTreeSet<_> = input
        .mirror_freshness_cards
        .iter()
        .map(|card| card.profile)
        .collect();
    input.deployment_summary_cards.iter().all(|card| {
        !profile_needs_mirror_freshness(card.mirror_offline_state)
            || freshness_profiles.contains(&card.profile)
    })
}

fn local_safe_fallback_truth_is_complete(input: &DeploymentProfileContinuityInput) -> bool {
    let fallback_by_profile: BTreeMap<_, _> = input
        .local_safe_fallback_cards
        .iter()
        .map(|card| (card.profile, card))
        .collect();
    input.claimed_profiles.iter().all(|profile| {
        let Some(card) = fallback_by_profile.get(profile) else {
            return false;
        };
        !card.state.requires_explicit_local_safe_copy()
            || (!card.usable_now.is_empty()
                && (!card.cached_or_stale.is_empty() || !card.blocked_until_refresh.is_empty())
                && !card.next_steps.is_empty())
    })
}

fn surface_fact_reuse_is_complete(input: &DeploymentProfileContinuityInput) -> bool {
    let families: BTreeSet<_> = input
        .surface_reuse_rows
        .iter()
        .filter(|row| row.surface_visibility.all_visible() && !row.canonical_source_ref.is_empty())
        .map(|row| row.fact_family)
        .collect();
    families == FactFamilyClass::ALL.iter().copied().collect()
}

fn seeded_summary_cards() -> Vec<DeploymentSummaryCard> {
    vec![
        DeploymentSummaryCard {
            card_id: "deployment-card:individual-local".to_owned(),
            profile: DeploymentProfileClass::IndividualLocal,
            profile_token: DeploymentProfileClass::IndividualLocal.as_str().to_owned(),
            tenant_scope: TenantOrgScopeClass::SingleUserLocal,
            tenant_scope_token: TenantOrgScopeClass::SingleUserLocal.as_str().to_owned(),
            tenant_org_label: "Personal device".to_owned(),
            region_label: "device-local".to_owned(),
            key_ownership_label: "OS credential store".to_owned(),
            mirror_offline_state: MirrorOfflineStateClass::NotApplicable,
            mirror_offline_state_token: MirrorOfflineStateClass::NotApplicable.as_str().to_owned(),
            delivery_sources: vec![BundleDeliverySourceClass::FileImport],
            delivery_source_tokens: vec![BundleDeliverySourceClass::FileImport.as_str().to_owned()],
            control_plane_services: vec![service_fact(
                "service:local-policy",
                "Local policy file and trust preferences",
                PlaneClass::ControlPlane,
                HostingPostureClass::LocalOnly,
                "Healthy",
                "No remote authority is required for core local work.",
            )],
            data_plane_services: vec![
                service_fact(
                    "service:local-workspace",
                    "Local editing, save, search, and Git",
                    PlaneClass::DataPlane,
                    HostingPostureClass::LocalOnly,
                    "Healthy",
                    "Core local workflows remain available without network access.",
                ),
                service_fact(
                    "service:local-debug",
                    "Local tasks and debuggers",
                    PlaneClass::DataPlane,
                    HostingPostureClass::LocalOnly,
                    "Healthy",
                    "Local execution is bounded to the current device.",
                ),
            ],
            vendor_dependency_refs: Vec::new(),
            last_control_plane_sync: "2026-06-01T00:00:00Z".to_owned(),
            local_safe_baseline: vec![
                "Edit, save, search, Git, tasks, and diagnostics stay local.".to_owned(),
                "No vendor sign-in or policy round trip is required.".to_owned(),
            ],
            surface_visibility: ContinuitySurfaceVisibility::all_required(),
        },
        DeploymentSummaryCard {
            card_id: "deployment-card:managed-cloud".to_owned(),
            profile: DeploymentProfileClass::ManagedCloud,
            profile_token: DeploymentProfileClass::ManagedCloud.as_str().to_owned(),
            tenant_scope: TenantOrgScopeClass::SharedMultiTenant,
            tenant_scope_token: TenantOrgScopeClass::SharedMultiTenant.as_str().to_owned(),
            tenant_org_label: "Aureline Cloud tenant".to_owned(),
            region_label: "us-west managed region".to_owned(),
            key_ownership_label: "Vendor-managed keys".to_owned(),
            mirror_offline_state: MirrorOfflineStateClass::OnlineLiveAllowed,
            mirror_offline_state_token: MirrorOfflineStateClass::OnlineLiveAllowed.as_str().to_owned(),
            delivery_sources: vec![BundleDeliverySourceClass::ManagedPull],
            delivery_source_tokens: vec![BundleDeliverySourceClass::ManagedPull.as_str().to_owned()],
            control_plane_services: vec![
                service_fact(
                    "service:managed-identity-policy",
                    "Managed identity, policy, and entitlement service",
                    PlaneClass::ControlPlane,
                    HostingPostureClass::VendorHosted,
                    "Healthy",
                    "Fresh auth and policy widen managed actions but do not define the local-safe floor.",
                ),
                service_fact(
                    "service:managed-catalog",
                    "Managed registry, docs, and release catalog",
                    PlaneClass::ControlPlane,
                    HostingPostureClass::VendorHosted,
                    "Healthy",
                    "Catalog freshness controls installs and updates, not local editing.",
                ),
            ],
            data_plane_services: vec![
                service_fact(
                    "service:local-core",
                    "Local editor, search, and Git",
                    PlaneClass::DataPlane,
                    HostingPostureClass::LocalOnly,
                    "Healthy",
                    "Core local workflows remain available if the service plane degrades.",
                ),
                service_fact(
                    "service:managed-runtime",
                    "Managed collaboration and runtime streams",
                    PlaneClass::DataPlane,
                    HostingPostureClass::VendorHosted,
                    "Healthy",
                    "Managed runtime collaboration may narrow independently of local work.",
                ),
            ],
            vendor_dependency_refs: vec![
                "dependency:managed-cloud:identity-policy".to_owned(),
                "dependency:managed-cloud:telemetry".to_owned(),
                "dependency:managed-cloud:ai-gateway".to_owned(),
            ],
            last_control_plane_sync: "2026-06-01T00:00:00Z".to_owned(),
            local_safe_baseline: vec![
                "Local editing, save, search, Git, and cached inspection continue during service outages.".to_owned(),
                "Fresh managed actions wait for auth, policy, or quota recovery.".to_owned(),
            ],
            surface_visibility: ContinuitySurfaceVisibility::all_required(),
        },
        DeploymentSummaryCard {
            card_id: "deployment-card:enterprise-online".to_owned(),
            profile: DeploymentProfileClass::EnterpriseOnline,
            profile_token: DeploymentProfileClass::EnterpriseOnline.as_str().to_owned(),
            tenant_scope: TenantOrgScopeClass::CustomerTenant,
            tenant_scope_token: TenantOrgScopeClass::CustomerTenant.as_str().to_owned(),
            tenant_org_label: "Acme Platform".to_owned(),
            region_label: "customer tenant in eu-central".to_owned(),
            key_ownership_label: "Customer-managed KMS plus local OS store".to_owned(),
            mirror_offline_state: MirrorOfflineStateClass::OnlineLiveAllowed,
            mirror_offline_state_token: MirrorOfflineStateClass::OnlineLiveAllowed.as_str().to_owned(),
            delivery_sources: vec![BundleDeliverySourceClass::ManagedPull],
            delivery_source_tokens: vec![BundleDeliverySourceClass::ManagedPull.as_str().to_owned()],
            control_plane_services: vec![
                service_fact(
                    "service:customer-idp-policy",
                    "Customer IdP and signed policy distribution",
                    PlaneClass::ControlPlane,
                    HostingPostureClass::CustomerHosted,
                    "Healthy",
                    "Customer-operated auth and policy still separate from local editing.",
                ),
                service_fact(
                    "service:approved-external-docs",
                    "Approved external docs and package catalog metadata",
                    PlaneClass::ControlPlane,
                    HostingPostureClass::ThirdPartyHosted,
                    "Healthy",
                    "Approved external metadata narrows installs without redefining local-safe work.",
                ),
            ],
            data_plane_services: vec![
                service_fact(
                    "service:local-core-enterprise",
                    "Local editor, search, and Git",
                    PlaneClass::DataPlane,
                    HostingPostureClass::LocalOnly,
                    "Healthy",
                    "The local desktop remains authoritative for current workspace edits.",
                ),
                service_fact(
                    "service:direct-remote-attach",
                    "Customer runtime and direct remote attach",
                    PlaneClass::DataPlane,
                    HostingPostureClass::CustomerHosted,
                    "Healthy",
                    "Direct customer runtime attach can remain available when external metadata narrows.",
                ),
            ],
            vendor_dependency_refs: vec![
                "dependency:enterprise-online:docs-catalog".to_owned(),
                "dependency:enterprise-online:model-routing".to_owned(),
            ],
            last_control_plane_sync: "2026-06-01T00:00:00Z".to_owned(),
            local_safe_baseline: vec![
                "Local editing and direct customer-hosted runtime work remain available.".to_owned(),
                "New external installs or hosted AI routes may narrow if approved external metadata is unavailable.".to_owned(),
            ],
            surface_visibility: ContinuitySurfaceVisibility::all_required(),
        },
        DeploymentSummaryCard {
            card_id: "deployment-card:self-hosted".to_owned(),
            profile: DeploymentProfileClass::SelfHosted,
            profile_token: DeploymentProfileClass::SelfHosted.as_str().to_owned(),
            tenant_scope: TenantOrgScopeClass::CustomerTenant,
            tenant_scope_token: TenantOrgScopeClass::CustomerTenant.as_str().to_owned(),
            tenant_org_label: "Acme Platform".to_owned(),
            region_label: "customer-operated eu-central region".to_owned(),
            key_ownership_label: "Customer-managed KMS".to_owned(),
            mirror_offline_state: MirrorOfflineStateClass::OnlineMirrorOnly,
            mirror_offline_state_token: MirrorOfflineStateClass::OnlineMirrorOnly.as_str().to_owned(),
            delivery_sources: vec![
                BundleDeliverySourceClass::MirrorPublication,
                BundleDeliverySourceClass::FileImport,
            ],
            delivery_source_tokens: vec![
                BundleDeliverySourceClass::MirrorPublication.as_str().to_owned(),
                BundleDeliverySourceClass::FileImport.as_str().to_owned(),
            ],
            control_plane_services: vec![
                service_fact(
                    "service:self-hosted-idp-policy",
                    "Customer-hosted identity, policy, and tenancy control",
                    PlaneClass::ControlPlane,
                    HostingPostureClass::CustomerHosted,
                    "Mirror only",
                    "Customer control remains explicit even when distribution is limited to the approved mirror.",
                ),
                service_fact(
                    "service:customer-mirror",
                    "Approved customer mirror for docs, bundles, and extensions",
                    PlaneClass::ControlPlane,
                    HostingPostureClass::MirrorBacked,
                    "Healthy",
                    "Mirror freshness, signer continuity, and blocked actions stay reviewable.",
                ),
            ],
            data_plane_services: vec![
                service_fact(
                    "service:self-hosted-local-core",
                    "Local editor, search, Git, and diagnostics",
                    PlaneClass::DataPlane,
                    HostingPostureClass::LocalOnly,
                    "Healthy",
                    "Core desktop work remains available when mirror or policy refresh degrades.",
                ),
                service_fact(
                    "service:customer-runtime",
                    "Customer-hosted runtime and collaboration data plane",
                    PlaneClass::DataPlane,
                    HostingPostureClass::CustomerHosted,
                    "Healthy",
                    "Customer-operated runtime data stays inside the customer boundary.",
                ),
            ],
            vendor_dependency_refs: vec![
                "dependency:self-hosted:crash-symbol-upload".to_owned(),
                "dependency:self-hosted:model-gateway".to_owned(),
            ],
            last_control_plane_sync: "2026-06-01T00:00:00Z".to_owned(),
            local_safe_baseline: vec![
                "Edit, save, search, Git, export, and customer-hosted runtime work stay available when mirror refresh is delayed.".to_owned(),
                "Fresh managed widenings stay paused until mirror freshness and signer continuity revalidate.".to_owned(),
            ],
            surface_visibility: ContinuitySurfaceVisibility::all_required(),
        },
        DeploymentSummaryCard {
            card_id: "deployment-card:air-gapped".to_owned(),
            profile: DeploymentProfileClass::AirGapped,
            profile_token: DeploymentProfileClass::AirGapped.as_str().to_owned(),
            tenant_scope: TenantOrgScopeClass::CustomerTenant,
            tenant_scope_token: TenantOrgScopeClass::CustomerTenant.as_str().to_owned(),
            tenant_org_label: "Acme Isolated Lab".to_owned(),
            region_label: "isolated customer network".to_owned(),
            key_ownership_label: "Customer-managed offline root".to_owned(),
            mirror_offline_state: MirrorOfflineStateClass::OfflineAirGapped,
            mirror_offline_state_token: MirrorOfflineStateClass::OfflineAirGapped.as_str().to_owned(),
            delivery_sources: vec![
                BundleDeliverySourceClass::AirGapTransfer,
                BundleDeliverySourceClass::LastKnownGoodCache,
            ],
            delivery_source_tokens: vec![
                BundleDeliverySourceClass::AirGapTransfer.as_str().to_owned(),
                BundleDeliverySourceClass::LastKnownGoodCache.as_str().to_owned(),
            ],
            control_plane_services: vec![
                service_fact(
                    "service:offline-policy-snapshot",
                    "Offline policy and entitlement snapshot",
                    PlaneClass::ControlPlane,
                    HostingPostureClass::OfflineSnapshot,
                    "Offline snapshot",
                    "Authority is frozen to the imported snapshot and never widened silently.",
                ),
                service_fact(
                    "service:internal-airgap-mirror",
                    "Internal air-gap mirror and evidence store",
                    PlaneClass::ControlPlane,
                    HostingPostureClass::MirrorBacked,
                    "Offline snapshot",
                    "Mirror freshness and signer continuity remain visible even without public reachability.",
                ),
            ],
            data_plane_services: vec![
                service_fact(
                    "service:airgap-local-core",
                    "Local editor, search, Git, tasks, and diagnostics",
                    PlaneClass::DataPlane,
                    HostingPostureClass::LocalOnly,
                    "Healthy",
                    "The desktop remains fully useful without public internet access.",
                ),
                service_fact(
                    "service:airgap-local-runtimes",
                    "Local or customer-bounded runtimes",
                    PlaneClass::DataPlane,
                    HostingPostureClass::CustomerHosted,
                    "Healthy",
                    "Only customer-bounded runtimes or local execution paths are claimed.",
                ),
            ],
            vendor_dependency_refs: Vec::new(),
            last_control_plane_sync: "2026-05-30T12:00:00Z".to_owned(),
            local_safe_baseline: vec![
                "Editing, search, Git, tasks, diagnostics, and cached help remain available from signed local state.".to_owned(),
                "Installs, updates, or policy widenings wait for the next approved import.".to_owned(),
            ],
            surface_visibility: ContinuitySurfaceVisibility::all_required(),
        },
    ]
}

fn seeded_residual_dependencies() -> Vec<ResidualDependencyRow> {
    vec![
        residual_dependency(
            "residual:managed-cloud:identity-policy",
            DeploymentProfileClass::ManagedCloud,
            "dependency:managed-cloud:identity-policy",
            "Managed identity and entitlement control plane",
            ResidualDependencyClass::ControlPlane,
            HostingPostureClass::VendorHosted,
            "Fresh managed sign-in, policy, and quota widenings pause until the control plane recovers.",
            "Continue local editing, search, Git, cached inspection, and export.",
            "Switch to local-only work or complete reauth when the control plane recovers.",
        ),
        residual_dependency(
            "residual:managed-cloud:telemetry",
            DeploymentProfileClass::ManagedCloud,
            "dependency:managed-cloud:telemetry",
            "Crash and support upload lane",
            ResidualDependencyClass::Telemetry,
            HostingPostureClass::VendorHosted,
            "Uploads queue locally and procurement or support packets stay unpublished until connectivity returns.",
            "Local queue and manual export remain available.",
            "Disable uploads or use manual support export.",
        ),
        residual_dependency(
            "residual:managed-cloud:ai-gateway",
            DeploymentProfileClass::ManagedCloud,
            "dependency:managed-cloud:ai-gateway",
            "Hosted AI routing and moderation gateway",
            ResidualDependencyClass::Ai,
            HostingPostureClass::VendorHosted,
            "Hosted AI routes narrow to manual, BYOK, or local alternatives.",
            "Use manual or local-model workflows where configured.",
            "Disable hosted routing or switch to an allowed local route.",
        ),
        residual_dependency(
            "residual:enterprise-online:docs-catalog",
            DeploymentProfileClass::EnterpriseOnline,
            "dependency:enterprise-online:docs-catalog",
            "Approved external docs and package metadata",
            ResidualDependencyClass::Content,
            HostingPostureClass::ThirdPartyHosted,
            "New docs and package installs narrow until approved metadata becomes reachable again.",
            "Keep working from cached docs and already-installed packages.",
            "Use a mirrored pack or disable external docs dependency.",
        ),
        residual_dependency(
            "residual:enterprise-online:model-routing",
            DeploymentProfileClass::EnterpriseOnline,
            "dependency:enterprise-online:model-routing",
            "Approved external model-routing overlay",
            ResidualDependencyClass::Ai,
            HostingPostureClass::ThirdPartyHosted,
            "Hosted model routing narrows to customer-hosted or manual alternatives.",
            "Continue local editing and customer-bounded runtime work.",
            "Disable external routing or switch to a customer-hosted route.",
        ),
        residual_dependency(
            "residual:self-hosted:crash-symbol-upload",
            DeploymentProfileClass::SelfHosted,
            "dependency:self-hosted:crash-symbol-upload",
            "Vendor crash-symbol and support publication lane",
            ResidualDependencyClass::Telemetry,
            HostingPostureClass::VendorHosted,
            "Crash and support publication stay queued or manual until that vendor lane is disabled or reachable.",
            "Local diagnostics, local export, and internal evidence handling remain available.",
            "Disable vendor crash-symbol upload or route support through an internal case system.",
        ),
        residual_dependency(
            "residual:self-hosted:model-gateway",
            DeploymentProfileClass::SelfHosted,
            "dependency:self-hosted:model-gateway",
            "Hosted model fallback gateway",
            ResidualDependencyClass::Ai,
            HostingPostureClass::VendorHosted,
            "Hosted fallback routing is unavailable; AI narrows to customer-hosted, BYOK, or manual workflows.",
            "Local editing and customer-hosted model routes remain available.",
            "Disable the hosted fallback gateway or replace it with a customer-hosted route.",
        ),
    ]
}

fn seeded_mirror_freshness_cards() -> Vec<MirrorFreshnessCard> {
    vec![
        mirror_freshness(
            "mirror-card:self-hosted:policy-bundle",
            DeploymentProfileClass::SelfHosted,
            "Signed policy bundle",
            BundleDeliverySourceClass::MirrorPublication,
            "Approved customer mirror",
            FreshnessStateClass::Current,
            SignerContinuityClass::VerifiedMirror,
            "2026-06-01T00:00:00Z",
            vec![
                "Current settings locks and managed narrowing are inspectable from the mirror."
                    .to_owned(),
            ],
            Vec::new(),
            "Refresh mirror metadata",
        ),
        mirror_freshness(
            "mirror-card:self-hosted:docs-pack",
            DeploymentProfileClass::SelfHosted,
            "Docs pack",
            BundleDeliverySourceClass::MirrorPublication,
            "Approved customer mirror",
            FreshnessStateClass::StaleWithinGrace,
            SignerContinuityClass::VerifiedMirror,
            "2026-05-31T18:00:00Z",
            vec!["Cached docs and prior support references remain readable.".to_owned()],
            vec![
                "Publishing a documentation freshness claim waits for the next mirror sync."
                    .to_owned(),
            ],
            "Request mirror refresh",
        ),
        mirror_freshness(
            "mirror-card:air-gapped:policy-bundle",
            DeploymentProfileClass::AirGapped,
            "Signed policy bundle snapshot",
            BundleDeliverySourceClass::AirGapTransfer,
            "Signed offline bundle",
            FreshnessStateClass::OfflineSnapshot,
            SignerContinuityClass::VerifiedOffline,
            "2026-05-30T12:00:00Z",
            vec!["Current offline policy and entitlement scope stay inspectable.".to_owned()],
            vec!["New managed widenings remain blocked until the next approved import.".to_owned()],
            "Review next offline import",
        ),
        mirror_freshness(
            "mirror-card:air-gapped:docs-pack",
            DeploymentProfileClass::AirGapped,
            "Docs pack snapshot",
            BundleDeliverySourceClass::AirGapTransfer,
            "Internal mirror bundle",
            FreshnessStateClass::OfflineSnapshot,
            SignerContinuityClass::VerifiedOffline,
            "2026-05-30T12:00:00Z",
            vec!["Cached help and support references remain available locally.".to_owned()],
            vec![
                "Fresh upstream docs and advisories wait for the next approved import.".to_owned(),
            ],
            "Open import manifest",
        ),
    ]
}

fn seeded_local_safe_fallback_cards() -> Vec<LocalSafeFallbackCard> {
    vec![
        local_safe_fallback(
            "fallback-card:individual-local",
            DeploymentProfileClass::IndividualLocal,
            LocalSafeStateClass::Healthy,
            vec![
                "Edit, save, search, Git, tasks, debug, and diagnostics.".to_owned(),
            ],
            Vec::new(),
            Vec::new(),
            vec!["Open deployment details".to_owned()],
        ),
        local_safe_fallback(
            "fallback-card:managed-cloud",
            DeploymentProfileClass::ManagedCloud,
            LocalSafeStateClass::Healthy,
            vec![
                "Edit, save, search, Git, and cached help.".to_owned(),
                "Manual export and local diagnostics.".to_owned(),
            ],
            Vec::new(),
            vec!["Fresh managed policy widenings if auth, quota, or service state degrades.".to_owned()],
            vec!["Open managed status".to_owned()],
        ),
        local_safe_fallback(
            "fallback-card:enterprise-online",
            DeploymentProfileClass::EnterpriseOnline,
            LocalSafeStateClass::ControlPlaneDegraded,
            vec![
                "Edit, save, search, Git, and direct customer-hosted runtime work.".to_owned(),
                "Cached docs and already-installed packages.".to_owned(),
            ],
            vec![
                "Approved external metadata is stale; cached package and docs facts are still readable.".to_owned(),
            ],
            vec![
                "New external installs and hosted model-routing changes wait for metadata recovery.".to_owned(),
            ],
            vec![
                "Continue local work".to_owned(),
                "Retry approved metadata sync".to_owned(),
            ],
        ),
        local_safe_fallback(
            "fallback-card:self-hosted",
            DeploymentProfileClass::SelfHosted,
            LocalSafeStateClass::MirrorStale,
            vec![
                "Edit, save, search, Git, export, and customer-hosted runtime work.".to_owned(),
                "Review active settings locks and signed policy state.".to_owned(),
            ],
            vec![
                "Docs pack is stale within grace and remains readable from cache.".to_owned(),
                "Last-known-good mirror metadata stays inspectable.".to_owned(),
            ],
            vec![
                "New managed widenings and mirror-fed installs wait for a fresh signed mirror sync.".to_owned(),
            ],
            vec![
                "Continue local work".to_owned(),
                "Request mirror refresh".to_owned(),
                "Export support metadata".to_owned(),
            ],
        ),
        local_safe_fallback(
            "fallback-card:air-gapped",
            DeploymentProfileClass::AirGapped,
            LocalSafeStateClass::OfflineLocalSafe,
            vec![
                "Edit, save, search, Git, tasks, diagnostics, and cached help.".to_owned(),
                "Review offline policy, entitlement, and provenance packets.".to_owned(),
            ],
            vec![
                "Offline policy bundle snapshot verified on 2026-05-30T12:00:00Z.".to_owned(),
                "Cached docs pack and mirror manifest remain inspectable.".to_owned(),
            ],
            vec![
                "New installs, updates, and policy widenings wait for the next approved import.".to_owned(),
            ],
            vec![
                "Continue local work".to_owned(),
                "Open import manifest".to_owned(),
                "Export offline support packet".to_owned(),
            ],
        ),
    ]
}

fn seeded_surface_reuse_rows() -> Vec<SurfaceReuseRow> {
    FactFamilyClass::ALL
        .iter()
        .copied()
        .map(|fact_family| SurfaceReuseRow {
            fact_family,
            fact_family_token: fact_family.as_str().to_owned(),
            canonical_source_label: "Deployment profile continuity packet".to_owned(),
            canonical_source_ref: DEPLOYMENT_PROFILE_CONTINUITY_ARTIFACT_REF.to_owned(),
            surface_visibility: ContinuitySurfaceVisibility::all_required(),
        })
        .collect()
}

fn service_fact(
    service_ref: &str,
    service_label: &str,
    plane: PlaneClass,
    hosting_posture: HostingPostureClass,
    status_label: &str,
    continuity_note: &str,
) -> DeploymentServiceFact {
    DeploymentServiceFact {
        service_ref: service_ref.to_owned(),
        service_label: service_label.to_owned(),
        plane,
        plane_token: plane.as_str().to_owned(),
        hosting_posture,
        hosting_posture_token: hosting_posture.as_str().to_owned(),
        status_label: status_label.to_owned(),
        continuity_note: continuity_note.to_owned(),
    }
}

fn residual_dependency(
    row_id: &str,
    profile: DeploymentProfileClass,
    dependency_ref: &str,
    feature_label: &str,
    dependency_class: ResidualDependencyClass,
    host_posture: HostingPostureClass,
    failure_consequence: &str,
    local_safe_alternative: &str,
    disable_or_alternative_action: &str,
) -> ResidualDependencyRow {
    ResidualDependencyRow {
        row_id: row_id.to_owned(),
        profile,
        profile_token: profile.as_str().to_owned(),
        dependency_ref: dependency_ref.to_owned(),
        feature_label: feature_label.to_owned(),
        dependency_class,
        dependency_class_token: dependency_class.as_str().to_owned(),
        host_posture,
        host_posture_token: host_posture.as_str().to_owned(),
        failure_consequence: failure_consequence.to_owned(),
        local_safe_alternative: local_safe_alternative.to_owned(),
        disable_or_alternative_action: disable_or_alternative_action.to_owned(),
        surface_visibility: ContinuitySurfaceVisibility::all_required(),
    }
}

fn mirror_freshness(
    card_id: &str,
    profile: DeploymentProfileClass,
    artifact_label: &str,
    delivery_source: BundleDeliverySourceClass,
    source_label: &str,
    freshness_state: FreshnessStateClass,
    signer_continuity: SignerContinuityClass,
    last_verified_at: &str,
    cached_usable_now: Vec<String>,
    blocked_until_refresh: Vec<String>,
    refresh_action_label: &str,
) -> MirrorFreshnessCard {
    MirrorFreshnessCard {
        card_id: card_id.to_owned(),
        profile,
        profile_token: profile.as_str().to_owned(),
        artifact_label: artifact_label.to_owned(),
        delivery_source,
        delivery_source_token: delivery_source.as_str().to_owned(),
        source_label: source_label.to_owned(),
        freshness_state,
        freshness_state_token: freshness_state.as_str().to_owned(),
        signer_continuity,
        signer_continuity_token: signer_continuity.as_str().to_owned(),
        last_verified_at: last_verified_at.to_owned(),
        cached_usable_now,
        blocked_until_refresh,
        refresh_action_label: refresh_action_label.to_owned(),
        surface_visibility: ContinuitySurfaceVisibility::all_required(),
    }
}

fn local_safe_fallback(
    card_id: &str,
    profile: DeploymentProfileClass,
    state: LocalSafeStateClass,
    usable_now: Vec<String>,
    cached_or_stale: Vec<String>,
    blocked_until_refresh: Vec<String>,
    next_steps: Vec<String>,
) -> LocalSafeFallbackCard {
    LocalSafeFallbackCard {
        card_id: card_id.to_owned(),
        profile,
        profile_token: profile.as_str().to_owned(),
        state,
        state_token: state.as_str().to_owned(),
        usable_now,
        cached_or_stale,
        blocked_until_refresh,
        next_steps,
        surface_visibility: ContinuitySurfaceVisibility::all_required(),
    }
}
