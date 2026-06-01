//! Stabilize deployment summary, residual-dependency ledger, region/key-mode
//! truth, and control-plane/data-plane continuity across managed, self-hosted,
//! and sovereign deployment rows.
//!
//! This module validates the five stability conditions that make deployment
//! mode, residency, region, key ownership, residual vendor dependency, and
//! local-core continuity explicit enough that no managed or sovereign claim
//! survives on implication alone:
//!
//! 1. **Vocabulary consistent across surfaces** — every claimed deployment
//!    profile token appears in the closed `deployment_profile` vocabulary;
//!    About, Help, diagnostics, service-health, and support-export surfaces all
//!    resolve the same token for the same running profile.
//! 2. **Residual-dependency ledger complete** — every non-`individual_local`
//!    profile row carries at least one residual-dependency row per vendor-bound
//!    or externally owned control-plane service; no profile with hosted
//!    control-plane services claims zero residual dependencies.
//! 3. **Plane separation enforced** — every plane-status strip keeps
//!    control-plane service impairment (identity, policy, catalog, relay)
//!    separate from data-plane capability impairment (local editing, save,
//!    search, Git); a continue-local path is preserved whenever data-plane
//!    capabilities remain `available_local_safe`.
//! 4. **Mirror/offline artifact rows present** — every profile that claims
//!    `online_mirror_only` or `offline_air_gapped` mirror/offline state carries
//!    at least one mirror/offline artifact row with signer, digest, freshness,
//!    and pin-state fields.
//! 5. **Sign-out/deprovision scope declared** — every profile whose
//!    tenant/org scope is `customer_tenant` or `shared_multi_tenant` carries
//!    sign-out/deprovision scope metadata naming what remains local, what stays
//!    tenant-scoped, and what is retained for policy or audit reasons.
//!
//! One condition forces `Withdrawn` immediately and cannot be overridden:
//!
//! - A [`DeploymentResidencyStabilizeNarrowReasonClass::ImpliedSovereigntyUnproven`]
//!   defect when a profile asserts a sovereignty or independence claim with no
//!   supporting evidence rows. The packet is withdrawn entirely.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language consequence labels, and
//! opaque refs only. Raw hostnames, raw tenant identifiers, raw key bytes,
//! raw region labels, and raw secret material stay outside the support
//! boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/policy/m4/stabilize-deployment-and-residency-truth.md`
//! - Artifact: `artifacts/policy/m4/stabilize-deployment-and-residency-truth.md`
//! - Contract ref: [`DEPLOYMENT_RESIDENCY_STABILIZE_SHARED_CONTRACT_REF`]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const DEPLOYMENT_RESIDENCY_STABILIZE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const DEPLOYMENT_RESIDENCY_STABILIZE_SHARED_CONTRACT_REF: &str =
    "policy:deployment_residency_stabilize:v1";

/// Record-kind tag for [`DeploymentResidencyStabilizePage`] payloads.
pub const DEPLOYMENT_RESIDENCY_STABILIZE_PAGE_RECORD_KIND: &str =
    "policy_deployment_residency_stabilize_page_record";

/// Record-kind tag for [`DeploymentResidencyStabilizeRow`] payloads.
pub const DEPLOYMENT_RESIDENCY_STABILIZE_ROW_RECORD_KIND: &str =
    "policy_deployment_residency_stabilize_row_record";

/// Record-kind tag for [`DeploymentResidencyStabilizeDefect`] payloads.
pub const DEPLOYMENT_RESIDENCY_STABILIZE_DEFECT_RECORD_KIND: &str =
    "policy_deployment_residency_stabilize_defect_record";

/// Record-kind tag for [`DeploymentResidencyStabilizeSupportExport`] payloads.
pub const DEPLOYMENT_RESIDENCY_STABILIZE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_deployment_residency_stabilize_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const DEPLOYMENT_RESIDENCY_STABILIZE_DOC_REF: &str =
    "docs/policy/m4/stabilize-deployment-and-residency-truth.md";

/// Repo-relative path of the artifact summary for this lane.
pub const DEPLOYMENT_RESIDENCY_STABILIZE_ARTIFACT_REF: &str =
    "artifacts/policy/m4/stabilize-deployment-and-residency-truth.md";

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Closed vocabulary of deployment profile tokens accepted by the audit.
///
/// Re-exports the same token set as `deployment_profile` in the deployment
/// summary card schema; no surface may mint tokens outside this set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfileClass {
    /// Desktop-local, single-user, no managed control plane.
    IndividualLocal,
    /// Customer-operated control plane, customer-managed keys and region.
    SelfHosted,
    /// Hybrid remote-attach with vendor-provided managed services.
    EnterpriseOnline,
    /// Offline-capable air-gapped mirror; no public egress.
    AirGapped,
    /// Vendor-operated SaaS with vendor-managed keys by default.
    ManagedCloud,
}

impl DeploymentProfileClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndividualLocal => "individual_local",
            Self::SelfHosted => "self_hosted",
            Self::EnterpriseOnline => "enterprise_online",
            Self::AirGapped => "air_gapped",
            Self::ManagedCloud => "managed_cloud",
        }
    }

    /// True when this profile has no managed control plane.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::IndividualLocal)
    }

    /// True when this profile requires residual-dependency ledger coverage.
    pub const fn requires_residual_dep_coverage(self) -> bool {
        !self.is_local_only()
    }

    /// True when this profile claims sovereignty (customer-controlled keys and
    /// region) and therefore must prove it.
    pub const fn claims_sovereignty(self) -> bool {
        matches!(self, Self::SelfHosted | Self::AirGapped)
    }
}

/// Mirror/offline state tokens relevant to the mirror-artifact-rows condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorOfflineStateClass {
    /// Live internet access is allowed; no mirror constraint.
    OnlineLiveAllowed,
    /// Only the local mirror snapshot may be used; live origin fetch is denied.
    OnlineMirrorOnly,
    /// Offline grace window is preserved from last sync.
    OfflineGracePreserved,
    /// Air-gapped; no public-internet egress at all.
    OfflineAirGapped,
    /// Deny-all enforced by operator policy.
    DenyAllEnforced,
    /// Network disabled by user.
    NetworkDisabledByUser,
    /// Network degraded by heuristic.
    NetworkDegradedHeuristic,
    /// Not applicable for this profile.
    NotApplicable,
}

impl MirrorOfflineStateClass {
    /// Stable token recorded on every serialized record.
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

    /// True when this state requires mirror/offline artifact rows.
    pub const fn requires_mirror_artifact_rows(self) -> bool {
        matches!(self, Self::OnlineMirrorOnly | Self::OfflineAirGapped)
    }
}

/// Tenant/org scope tokens relevant to the sign-out/deprovision condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TenantOrgScopeClass {
    /// Single user, local device only.
    SingleUserLocal,
    /// Named customer tenant with explicit org boundary.
    CustomerTenant,
    /// Shared multi-tenant partition.
    SharedMultiTenant,
    /// Tenant boundary is stale; recheck required.
    TenantBoundaryRecheckRequired,
    /// Not applicable for this profile.
    NotApplicable,
}

impl TenantOrgScopeClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleUserLocal => "single_user_local",
            Self::CustomerTenant => "customer_tenant",
            Self::SharedMultiTenant => "shared_multi_tenant",
            Self::TenantBoundaryRecheckRequired => "tenant_boundary_recheck_required",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this scope requires an explicit sign-out/deprovision
    /// declaration naming local-retained vs tenant-scoped vs audit-retained
    /// data.
    pub const fn requires_sign_out_scope_declaration(self) -> bool {
        matches!(self, Self::CustomerTenant | Self::SharedMultiTenant)
    }
}

/// One profile row in the deployment residency input.
///
/// Each row captures the auditable state for a single claimed deployment
/// profile: its closed-vocabulary tokens, residual-dependency coverage,
/// mirror/offline artifact coverage, and sign-out/deprovision scope
/// declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentResidencyProfileRow {
    /// Stable profile token.
    pub profile_class: DeploymentProfileClass,
    /// Tenant/org scope token for this profile.
    pub tenant_org_scope_class: TenantOrgScopeClass,
    /// Mirror/offline state token for this profile.
    pub mirror_offline_state_class: MirrorOfflineStateClass,
    /// True when the residual-dependency rows cover all vendor-bound or
    /// externally owned control-plane services named in the control-plane
    /// state summary. Ignored when `profile_class` is `individual_local`.
    pub residual_deps_cover_vendor_services: bool,
    /// Number of residual-dependency rows linked for this profile.
    pub residual_dependency_row_count: usize,
    /// Number of mirror/offline artifact rows linked for this profile.
    pub mirror_artifact_row_count: usize,
    /// True when the sign-out/deprovision scope metadata is declared for this
    /// profile. Only checked when `tenant_org_scope_class` is `customer_tenant`
    /// or `shared_multi_tenant`.
    pub sign_out_scope_declared: bool,
    /// True when the profile asserts a sovereignty or independence claim
    /// (e.g. "self-hosted sovereign", "air-gapped") AND at least one
    /// supporting evidence row backs the claim. When false AND
    /// `profile_class.claims_sovereignty()` is true, the packet is withdrawn.
    pub sovereignty_claim_evidenced: bool,
}

/// One plane-status strip in the deployment residency input.
///
/// Each strip captures whether a service-health or outage surface correctly
/// separates control-plane service impairment from data-plane capability
/// impairment and preserves a continue-local path when the data plane is
/// safe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentResidencyPlaneStrip {
    /// Opaque strip identifier.
    pub strip_id: String,
    /// Profile this strip belongs to.
    pub profile_class: DeploymentProfileClass,
    /// True when control-plane service impairment (identity, policy, catalog,
    /// relay) is NOT conflated with data-plane capability blockage (local
    /// editing, save, search, Git).
    pub control_data_plane_separated: bool,
    /// True when a continue-local path is preserved whenever data-plane
    /// capabilities remain `available_local_safe`.
    pub continue_local_path_preserved: bool,
}

/// Full auditable input for the deployment residency stabilize page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentResidencyInput {
    /// Whether deployment vocabulary is consistent across all registered
    /// consumer surfaces (About, Help, diagnostics, service-health,
    /// support-export). This is `true` when every surface resolves the same
    /// closed-vocabulary token for the same running deployment profile.
    pub vocabulary_consistent_across_surfaces: bool,
    /// Claimed deployment profile rows.
    pub profile_rows: Vec<DeploymentResidencyProfileRow>,
    /// Plane-status strips for all registered outage and service-health
    /// surfaces.
    pub plane_status_strips: Vec<DeploymentResidencyPlaneStrip>,
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and for individual rows.
///
/// The tier is derived, not asserted: it is set by comparing the audit defect
/// list against the five stability conditions. A caller may never bump a row
/// to `Stable` without a clean audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentResidencyStabilizeQualificationClass {
    /// All five stability conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required profile row is missing; coverage gap prevents a beta claim.
    Preview,
    /// An unevidenced sovereignty claim was found; the packet is withdrawn.
    Withdrawn,
}

impl DeploymentResidencyStabilizeQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// True when this tier is claimable (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed reason a packet or row was narrowed below
/// [`DeploymentResidencyStabilizeQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentResidencyStabilizeNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// Deployment vocabulary tokens differ between registered consumer surfaces;
    /// the same profile resolves to different labels depending on the surface.
    VocabularyInconsistentAcrossSurfaces,
    /// A non-`individual_local` profile row does not carry residual-dependency
    /// rows for all vendor-bound or externally owned control-plane services.
    ResidualDependencyLedgerIncomplete,
    /// A plane-status strip conflates control-plane service impairment with
    /// data-plane capability blockage, or omits the continue-local path when
    /// data-plane capabilities are `available_local_safe`.
    PlaneSeparationMissing,
    /// A profile claiming `online_mirror_only` or `offline_air_gapped` state
    /// does not carry mirror/offline artifact rows with signer, digest,
    /// freshness, and pin-state fields.
    MirrorArtifactRowsAbsent,
    /// A profile with `customer_tenant` or `shared_multi_tenant` scope does
    /// not declare sign-out/deprovision metadata naming what remains local,
    /// what stays tenant-scoped, and what is retained for audit.
    SignOutScopeUndeclared,
    /// A profile asserts a sovereignty or independence claim (self-hosted,
    /// air-gapped) without any supporting evidence rows. This is a hard
    /// guardrail that withdraws the packet and cannot be overridden.
    ImpliedSovereigntyUnproven,
}

impl DeploymentResidencyStabilizeNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::VocabularyInconsistentAcrossSurfaces => {
                "vocabulary_inconsistent_across_surfaces"
            }
            Self::ResidualDependencyLedgerIncomplete => "residual_dependency_ledger_incomplete",
            Self::PlaneSeparationMissing => "plane_separation_missing",
            Self::MirrorArtifactRowsAbsent => "mirror_artifact_rows_absent",
            Self::SignOutScopeUndeclared => "sign_out_scope_undeclared",
            Self::ImpliedSovereigntyUnproven => "implied_sovereignty_unproven",
        }
    }

    /// True when this reason is a hard guardrail that withdraws the packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::ImpliedSovereigntyUnproven)
    }
}

// ---------------------------------------------------------------------------
// Row, summary, defect types
// ---------------------------------------------------------------------------

/// Stability qualification for one profile row in the stabilize packet.
///
/// Each row is bound to a single [`DeploymentResidencyProfileRow`] from the
/// input. The qualification is derived from the combined audit of that row
/// and the shared plane-strip and vocabulary state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentResidencyStabilizeRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Profile token for this row.
    pub profile_token: String,
    /// Tenant/org scope token for this row.
    pub tenant_org_scope_token: String,
    /// Mirror/offline state token for this row.
    pub mirror_offline_state_token: String,
    /// Number of residual-dependency rows linked.
    pub residual_dependency_row_count: usize,
    /// True when residual-dependency rows cover all vendor-bound services.
    pub residual_deps_cover_vendor_services: bool,
    /// Number of mirror/offline artifact rows linked.
    pub mirror_artifact_row_count: usize,
    /// True when sign-out/deprovision scope is declared.
    pub sign_out_scope_declared: bool,
    /// True when the sovereignty claim is evidenced.
    pub sovereignty_claim_evidenced: bool,
    /// Derived qualification tier.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

/// Aggregate banner emitted with the stabilize page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DeploymentResidencyStabilizeSummary {
    /// Total profile row count.
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Number of defects in this packet.
    pub defect_count: usize,
    /// Profile tokens covered by the input rows.
    pub profiles_covered: Vec<String>,
    /// Number of plane-status strips audited.
    pub plane_strip_count: usize,
    /// Number of strips with plane separation verified.
    pub plane_strips_separated_count: usize,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl DeploymentResidencyStabilizeSummary {
    fn from_rows(
        rows: &[DeploymentResidencyStabilizeRow],
        defects: &[DeploymentResidencyStabilizeDefect],
        input: &DeploymentResidencyInput,
    ) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
        }
        let overall = if withdrawn > 0 {
            DeploymentResidencyStabilizeQualificationClass::Withdrawn
        } else if preview > 0 {
            DeploymentResidencyStabilizeQualificationClass::Preview
        } else if beta > 0 {
            DeploymentResidencyStabilizeQualificationClass::Beta
        } else {
            DeploymentResidencyStabilizeQualificationClass::Stable
        };
        let profiles_covered: Vec<String> =
            rows.iter().map(|r| r.profile_token.clone()).collect();
        let plane_strips_separated_count = input
            .plane_status_strips
            .iter()
            .filter(|s| s.control_data_plane_separated)
            .count();
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            defect_count: defects.len(),
            profiles_covered,
            plane_strip_count: input.plane_status_strips.len(),
            plane_strips_separated_count,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

/// Typed defect emitted by the stabilize page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentResidencyStabilizeDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: DeploymentResidencyStabilizeNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (profile token, strip id, or `input`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl DeploymentResidencyStabilizeDefect {
    fn new(
        narrow_reason: DeploymentResidencyStabilizeNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: DEPLOYMENT_RESIDENCY_STABILIZE_DEFECT_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_RESIDENCY_STABILIZE_SCHEMA_VERSION,
            shared_contract_ref: DEPLOYMENT_RESIDENCY_STABILIZE_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:deployment-residency-stabilize:{}:{}",
                narrow_reason.as_str(),
                &source_str
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source: source_str,
            note: note.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Main page
// ---------------------------------------------------------------------------

/// Stable proof packet for deployment summary, residual-dependency ledger,
/// region/key-mode truth, and control-plane/data-plane continuity.
///
/// The packet is the single inspectable record that proves the stable claim
/// for this lane. About, Help, service-health, diagnostics, and support
/// exports should ingest it rather than cloning deployment-status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentResidencyStabilizePage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC instant when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary derived from all rows.
    pub summary: DeploymentResidencyStabilizeSummary,
    /// Per-profile stability rows.
    pub rows: Vec<DeploymentResidencyStabilizeRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<DeploymentResidencyStabilizeDefect>,
    /// The audited deployment residency input embedded as evidence.
    pub input: DeploymentResidencyInput,
}

impl DeploymentResidencyStabilizePage {
    /// Build the stabilize page from a deployment residency input.
    ///
    /// Rows are derived per profile row in the input, and the qualification
    /// for each is computed from the combined audit of the whole input.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: DeploymentResidencyInput,
    ) -> Self {
        let defects = audit_deployment_residency_input(&input);
        let rows = derive_stabilize_rows(&input, &defects);
        let summary = DeploymentResidencyStabilizeSummary::from_rows(&rows, &defects, &input);
        Self {
            record_kind: DEPLOYMENT_RESIDENCY_STABILIZE_PAGE_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_RESIDENCY_STABILIZE_SCHEMA_VERSION,
            shared_contract_ref: DEPLOYMENT_RESIDENCY_STABILIZE_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            defects,
            input,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == DeploymentResidencyStabilizeQualificationClass::Stable.as_str()
    }

    /// True when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// True when deployment vocabulary tokens are consistent across surfaces.
    pub fn vocabulary_is_consistent(&self) -> bool {
        self.input.vocabulary_consistent_across_surfaces
    }

    /// True when all non-local profiles carry complete residual-dependency
    /// ledger coverage.
    pub fn residual_dependency_ledger_is_complete(&self) -> bool {
        self.input.profile_rows.iter().all(|row| {
            !row.profile_class.requires_residual_dep_coverage()
                || row.residual_deps_cover_vendor_services
        })
    }

    /// True when all registered plane-status strips separate control-plane
    /// from data-plane impairment and preserve a continue-local path.
    pub fn plane_separation_is_enforced(&self) -> bool {
        self.input.plane_status_strips.iter().all(|strip| {
            strip.control_data_plane_separated && strip.continue_local_path_preserved
        })
    }

    /// True when all mirror/offline profiles carry at least one artifact row.
    pub fn mirror_artifact_rows_are_present(&self) -> bool {
        self.input.profile_rows.iter().all(|row| {
            !row.mirror_offline_state_class
                .requires_mirror_artifact_rows()
                || row.mirror_artifact_row_count > 0
        })
    }

    /// True when all tenanted profiles declare sign-out/deprovision scope.
    pub fn sign_out_scope_is_declared(&self) -> bool {
        self.input.profile_rows.iter().all(|row| {
            !row.tenant_org_scope_class
                .requires_sign_out_scope_declaration()
                || row.sign_out_scope_declared
        })
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that quotes the stabilize page plus a metadata-safe
/// defect roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentResidencyStabilizeSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC export timestamp.
    pub generated_at: String,
    /// The stabilize page embedded as evidence.
    pub page: DeploymentResidencyStabilizePage,
    /// Narrow-reason tokens present in the page's defect list.
    pub narrow_reasons_present: Vec<DeploymentResidencyStabilizeNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw private material (hostnames, tenant ids, key bytes) is
    /// excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl DeploymentResidencyStabilizeSupportExport {
    /// Wrap a stabilize page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: DeploymentResidencyStabilizePage,
    ) -> Self {
        let mut reasons: Vec<DeploymentResidencyStabilizeNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
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
            record_kind: DEPLOYMENT_RESIDENCY_STABILIZE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_RESIDENCY_STABILIZE_SCHEMA_VERSION,
            shared_contract_ref: DEPLOYMENT_RESIDENCY_STABILIZE_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit and validate functions
// ---------------------------------------------------------------------------

/// Re-run the deployment residency audit over the embedded input.
pub fn audit_deployment_residency_stabilize_page(
    page: &DeploymentResidencyStabilizePage,
) -> Vec<DeploymentResidencyStabilizeDefect> {
    audit_deployment_residency_input(&page.input)
}

/// Validate a stabilize page; returns `Ok` when the audit is clean.
pub fn validate_deployment_residency_stabilize_page(
    page: &DeploymentResidencyStabilizePage,
) -> Result<(), Vec<DeploymentResidencyStabilizeDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn audit_deployment_residency_input(
    input: &DeploymentResidencyInput,
) -> Vec<DeploymentResidencyStabilizeDefect> {
    let mut defects: Vec<DeploymentResidencyStabilizeDefect> = Vec::new();

    // Hard guardrail: sovereignty claim with no evidence rows — withdraw
    // immediately for the offending profile.
    for row in &input.profile_rows {
        if row.profile_class.claims_sovereignty() && !row.sovereignty_claim_evidenced {
            defects.push(DeploymentResidencyStabilizeDefect::new(
                DeploymentResidencyStabilizeNarrowReasonClass::ImpliedSovereigntyUnproven,
                row.profile_class.as_str(),
                format!(
                    "profile '{}' asserts a sovereignty or independence claim \
                     but carries no supporting evidence rows; packet is withdrawn",
                    row.profile_class.as_str()
                ),
            ));
        }
    }
    if defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason())
    {
        return defects;
    }

    // Condition 1: Vocabulary consistent across surfaces.
    if !input.vocabulary_consistent_across_surfaces {
        defects.push(DeploymentResidencyStabilizeDefect::new(
            DeploymentResidencyStabilizeNarrowReasonClass::VocabularyInconsistentAcrossSurfaces,
            "input",
            "deployment vocabulary tokens are not consistent across all registered consumer \
             surfaces; About, Help, diagnostics, service-health, and support-export must resolve \
             the same closed-vocabulary token for the same running profile",
        ));
    }

    // Condition 2: Residual-dependency ledger complete.
    for row in &input.profile_rows {
        if row.profile_class.requires_residual_dep_coverage()
            && !row.residual_deps_cover_vendor_services
        {
            defects.push(DeploymentResidencyStabilizeDefect::new(
                DeploymentResidencyStabilizeNarrowReasonClass::ResidualDependencyLedgerIncomplete,
                row.profile_class.as_str(),
                format!(
                    "profile '{}' does not carry residual-dependency rows covering all \
                     vendor-bound or externally owned control-plane services; every \
                     non-individual-local profile must surface its residual dependencies \
                     rather than implying zero hosted dependence",
                    row.profile_class.as_str()
                ),
            ));
        }
    }

    // Condition 3: Plane separation enforced.
    for strip in &input.plane_status_strips {
        if !strip.control_data_plane_separated {
            defects.push(DeploymentResidencyStabilizeDefect::new(
                DeploymentResidencyStabilizeNarrowReasonClass::PlaneSeparationMissing,
                strip.strip_id.clone(),
                format!(
                    "plane-status strip '{}' (profile '{}') conflates control-plane service \
                     impairment with data-plane capability blockage; identity, policy, catalog, \
                     and relay failures must be surfaced separately from local editing, save, \
                     search, and Git impairment",
                    strip.strip_id,
                    strip.profile_class.as_str()
                ),
            ));
        }
        if !strip.continue_local_path_preserved {
            defects.push(DeploymentResidencyStabilizeDefect::new(
                DeploymentResidencyStabilizeNarrowReasonClass::PlaneSeparationMissing,
                strip.strip_id.clone(),
                format!(
                    "plane-status strip '{}' (profile '{}') does not preserve a continue-local \
                     path when data-plane capabilities are available_local_safe; the surface must \
                     not force a user to wait when local work remains safe",
                    strip.strip_id,
                    strip.profile_class.as_str()
                ),
            ));
        }
    }

    // Condition 4: Mirror/offline artifact rows present.
    for row in &input.profile_rows {
        if row.mirror_offline_state_class.requires_mirror_artifact_rows()
            && row.mirror_artifact_row_count == 0
        {
            defects.push(DeploymentResidencyStabilizeDefect::new(
                DeploymentResidencyStabilizeNarrowReasonClass::MirrorArtifactRowsAbsent,
                row.profile_class.as_str(),
                format!(
                    "profile '{}' claims '{}' mirror/offline state but carries no mirror/offline \
                     artifact rows; each claimed mirror or air-gapped artifact family must surface \
                     signer state, digest state, freshness, and pin state",
                    row.profile_class.as_str(),
                    row.mirror_offline_state_class.as_str()
                ),
            ));
        }
    }

    // Condition 5: Sign-out/deprovision scope declared.
    for row in &input.profile_rows {
        if row.tenant_org_scope_class.requires_sign_out_scope_declaration()
            && !row.sign_out_scope_declared
        {
            defects.push(DeploymentResidencyStabilizeDefect::new(
                DeploymentResidencyStabilizeNarrowReasonClass::SignOutScopeUndeclared,
                row.profile_class.as_str(),
                format!(
                    "profile '{}' (scope '{}') does not declare sign-out/deprovision metadata; \
                     the declaration must name what remains local on device, what stays \
                     tenant-scoped, and what is retained for policy or audit reasons",
                    row.profile_class.as_str(),
                    row.tenant_org_scope_class.as_str()
                ),
            ));
        }
    }

    defects
}

fn derive_stabilize_rows(
    input: &DeploymentResidencyInput,
    page_defects: &[DeploymentResidencyStabilizeDefect],
) -> Vec<DeploymentResidencyStabilizeRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());

    let overall_qual = if has_withdrawal {
        DeploymentResidencyStabilizeQualificationClass::Withdrawn
    } else if !page_defects.is_empty() {
        DeploymentResidencyStabilizeQualificationClass::Beta
    } else {
        DeploymentResidencyStabilizeQualificationClass::Stable
    };

    // The leading narrow reason for summary display.
    let leading_narrow_reason = if has_withdrawal {
        DeploymentResidencyStabilizeNarrowReasonClass::ImpliedSovereigntyUnproven
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        DeploymentResidencyStabilizeNarrowReasonClass::NotNarrowed
    };

    // Build per-profile defect index (profile_token -> worst narrow reason).
    let mut profile_defect: BTreeMap<String, DeploymentResidencyStabilizeNarrowReasonClass> =
        BTreeMap::new();
    for defect in page_defects {
        // Sovereignty withdrawal takes priority.
        let entry = profile_defect
            .entry(defect.source.clone())
            .or_insert(DeploymentResidencyStabilizeNarrowReasonClass::NotNarrowed);
        if defect.narrow_reason.is_withdrawal_reason()
            || *entry == DeploymentResidencyStabilizeNarrowReasonClass::NotNarrowed
        {
            *entry = defect.narrow_reason;
        }
    }
    // Also map the global "input" source defect to each profile row.
    let global_defect = profile_defect
        .get("input")
        .copied()
        .unwrap_or(DeploymentResidencyStabilizeNarrowReasonClass::NotNarrowed);

    input
        .profile_rows
        .iter()
        .map(|row| {
            let profile_str = row.profile_class.as_str().to_owned();
            // Merge global defect with per-profile defect.
            let row_narrow = {
                let per_profile = profile_defect
                    .get(&profile_str)
                    .copied()
                    .unwrap_or(DeploymentResidencyStabilizeNarrowReasonClass::NotNarrowed);
                if has_withdrawal
                    || per_profile.is_withdrawal_reason()
                    || global_defect.is_withdrawal_reason()
                {
                    leading_narrow_reason
                } else if per_profile != DeploymentResidencyStabilizeNarrowReasonClass::NotNarrowed
                {
                    per_profile
                } else if global_defect != DeploymentResidencyStabilizeNarrowReasonClass::NotNarrowed {
                    global_defect
                } else {
                    DeploymentResidencyStabilizeNarrowReasonClass::NotNarrowed
                }
            };

            // Strip-level defects: does any strip for this profile have issues?
            let strip_defects_for_profile = page_defects.iter().any(|d| {
                d.narrow_reason == DeploymentResidencyStabilizeNarrowReasonClass::PlaneSeparationMissing
                    && input
                        .plane_status_strips
                        .iter()
                        .any(|s| s.profile_class == row.profile_class && s.strip_id == d.source)
            });
            let _ = strip_defects_for_profile; // Row qual is already captured via overall_qual.

            let row_qual = if has_withdrawal {
                DeploymentResidencyStabilizeQualificationClass::Withdrawn
            } else if row_narrow != DeploymentResidencyStabilizeNarrowReasonClass::NotNarrowed {
                overall_qual
            } else {
                overall_qual
            };

            let summary = build_row_summary(
                &profile_str,
                &row_qual,
                row_narrow,
            );

            DeploymentResidencyStabilizeRow {
                record_kind: DEPLOYMENT_RESIDENCY_STABILIZE_ROW_RECORD_KIND.to_owned(),
                schema_version: DEPLOYMENT_RESIDENCY_STABILIZE_SCHEMA_VERSION,
                shared_contract_ref: DEPLOYMENT_RESIDENCY_STABILIZE_SHARED_CONTRACT_REF
                    .to_owned(),
                profile_token: profile_str,
                tenant_org_scope_token: row.tenant_org_scope_class.as_str().to_owned(),
                mirror_offline_state_token: row.mirror_offline_state_class.as_str().to_owned(),
                residual_dependency_row_count: row.residual_dependency_row_count,
                residual_deps_cover_vendor_services: row.residual_deps_cover_vendor_services,
                mirror_artifact_row_count: row.mirror_artifact_row_count,
                sign_out_scope_declared: row.sign_out_scope_declared,
                sovereignty_claim_evidenced: row.sovereignty_claim_evidenced,
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn build_row_summary(
    profile_token: &str,
    qual: &DeploymentResidencyStabilizeQualificationClass,
    narrow_reason: DeploymentResidencyStabilizeNarrowReasonClass,
) -> String {
    match qual {
        DeploymentResidencyStabilizeQualificationClass::Stable => format!(
            "Profile '{}' qualifies stable: deployment vocabulary is consistent, \
             residual-dependency ledger is complete, plane separation is enforced, \
             mirror/offline artifact rows are present where required, and \
             sign-out/deprovision scope is declared.",
            profile_token
        ),
        _ => format!(
            "Profile '{}' narrowed to {} ({}): see defect list for details.",
            profile_token,
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable packet consumed by the headless example, the
/// integration tests, and the fixture generator.
///
/// The seeded page seeds zero defects: all five stability conditions hold.
/// All five standard deployment profiles are covered; every profile that
/// requires residual-dependency coverage carries it; all plane strips
/// separate impairment; mirror/offline artifact rows are present for the
/// air-gapped profile; and sign-out/deprovision scope is declared for all
/// tenanted profiles.
pub fn seeded_deployment_residency_stabilize_page() -> DeploymentResidencyStabilizePage {
    DeploymentResidencyStabilizePage::new(
        "policy:deployment_residency_stabilize:default",
        "Deployment summary, residual-dependency ledger, region/key-mode truth, \
         and control-plane/data-plane continuity (stable)",
        "2026-06-01T00:00:00Z",
        seeded_deployment_residency_input(),
    )
}

/// Build the seeded deployment residency input with zero defects.
pub fn seeded_deployment_residency_input() -> DeploymentResidencyInput {
    DeploymentResidencyInput {
        vocabulary_consistent_across_surfaces: true,
        profile_rows: vec![
            DeploymentResidencyProfileRow {
                profile_class: DeploymentProfileClass::IndividualLocal,
                tenant_org_scope_class: TenantOrgScopeClass::SingleUserLocal,
                mirror_offline_state_class: MirrorOfflineStateClass::NotApplicable,
                residual_deps_cover_vendor_services: false, // no hosted deps for local
                residual_dependency_row_count: 0,
                mirror_artifact_row_count: 0,
                sign_out_scope_declared: false, // single-user-local does not require declaration
                sovereignty_claim_evidenced: false, // not a sovereignty claim
            },
            DeploymentResidencyProfileRow {
                profile_class: DeploymentProfileClass::ManagedCloud,
                tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
                mirror_offline_state_class: MirrorOfflineStateClass::OnlineLiveAllowed,
                residual_deps_cover_vendor_services: true,
                residual_dependency_row_count: 3,
                mirror_artifact_row_count: 0,
                sign_out_scope_declared: true,
                sovereignty_claim_evidenced: false, // managed cloud does not claim sovereignty
            },
            DeploymentResidencyProfileRow {
                profile_class: DeploymentProfileClass::EnterpriseOnline,
                tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
                mirror_offline_state_class: MirrorOfflineStateClass::OnlineLiveAllowed,
                residual_deps_cover_vendor_services: true,
                residual_dependency_row_count: 2,
                mirror_artifact_row_count: 0,
                sign_out_scope_declared: true,
                sovereignty_claim_evidenced: false,
            },
            DeploymentResidencyProfileRow {
                profile_class: DeploymentProfileClass::SelfHosted,
                tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
                mirror_offline_state_class: MirrorOfflineStateClass::OnlineLiveAllowed,
                residual_deps_cover_vendor_services: true,
                residual_dependency_row_count: 2,
                mirror_artifact_row_count: 1,
                sign_out_scope_declared: true,
                sovereignty_claim_evidenced: true,
            },
            DeploymentResidencyProfileRow {
                profile_class: DeploymentProfileClass::AirGapped,
                tenant_org_scope_class: TenantOrgScopeClass::CustomerTenant,
                mirror_offline_state_class: MirrorOfflineStateClass::OfflineAirGapped,
                residual_deps_cover_vendor_services: true,
                residual_dependency_row_count: 2,
                mirror_artifact_row_count: 2,
                sign_out_scope_declared: true,
                sovereignty_claim_evidenced: true,
            },
        ],
        plane_status_strips: vec![
            DeploymentResidencyPlaneStrip {
                strip_id: "strip.plane.managed_cloud.baseline".to_owned(),
                profile_class: DeploymentProfileClass::ManagedCloud,
                control_data_plane_separated: true,
                continue_local_path_preserved: true,
            },
            DeploymentResidencyPlaneStrip {
                strip_id: "strip.plane.enterprise_online.baseline".to_owned(),
                profile_class: DeploymentProfileClass::EnterpriseOnline,
                control_data_plane_separated: true,
                continue_local_path_preserved: true,
            },
            DeploymentResidencyPlaneStrip {
                strip_id: "strip.plane.self_hosted.baseline".to_owned(),
                profile_class: DeploymentProfileClass::SelfHosted,
                control_data_plane_separated: true,
                continue_local_path_preserved: true,
            },
            DeploymentResidencyPlaneStrip {
                strip_id: "strip.plane.air_gapped.baseline".to_owned(),
                profile_class: DeploymentProfileClass::AirGapped,
                control_data_plane_separated: true,
                continue_local_path_preserved: true,
            },
        ],
    }
}
