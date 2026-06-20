//! Canonical operator/support continuity summary surfaced in About, Help,
//! service-health, the support center, and support/issue-report exports.
//!
//! The other continuity modules each freeze one slice of the truth: the
//! [`m5_locality_tenant_keymode_and_drill_matrix`](crate::m5_locality_tenant_keymode_and_drill_matrix)
//! freezes *what* a claimed row discloses, the
//! [`m5_control_plane_vs_data_plane_outage`](crate::m5_control_plane_vs_data_plane_outage)
//! taxonomy freezes *how* a managed lane degrades, and the
//! [`m5_continuity_freshness_slo`](crate::m5_continuity_freshness_slo) dashboard
//! freezes *how fresh* the backing evidence is. This module freezes the one
//! operator- and support-facing object that joins those slices so About, Help,
//! service health, the support center, and support/export packets can answer the
//! same questions in the same plain product language without a bespoke
//! per-surface explanation:
//!
//! 1. Which exact continuity row is in effect right now — named, not a generic
//!    "service degraded" banner?
//! 2. What is its locality, tenant, and key posture in plain product language?
//! 3. If a managed lane is impaired, which outage-taxonomy state and affected
//!    plane is it, and what narrower fallback remains?
//! 4. Is the backing continuity evidence current, stale, or missing — and does
//!    the claim narrow automatically when it is?
//!
//! Two hard guardrails fail closed rather than narrow. A summary may never use
//! generic degraded wording when the exact continuity row and its narrower
//! fallback class are known, and it may never carry admin-only internal routing
//! or raw secret material. A summary that violates either is withheld
//! ([`ContinuityClaimQualificationClass::Withdrawn`]) so no surface renders it.
//!
//! The local-core continuity lane is never narrowed or withheld because a
//! managed lane went stale or impaired — a local-core row keeps its claim and
//! still reaches About, Help, service health, and the support center.
//!
//! The packet is metadata-only. It carries closed-vocabulary tokens, export-safe
//! plain-language labels, UTC timestamps, and opaque refs. Raw hostnames, raw
//! tenant identifiers, raw KMS handles, raw routing, raw incident bodies, and
//! secret material never cross this boundary.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::m5_control_plane_vs_data_plane_outage::{
    DegradedFallbackClass, ImpairmentSeverityClass, OutageDegradedStateClass,
    OutageEvidenceStateClass,
};
use crate::m5_locality_tenant_keymode_and_drill_matrix::{
    ContinuityClaimQualificationClass, ContinuityLaneClass, ContinuityProfileClass, KeyModeClass,
    LocalityClass, PlaneImpairmentClass, TenantScopeClass,
};

#[cfg(test)]
mod tests;

/// Schema version carried on every record in this module.
pub const OPERATOR_SUPPORT_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const OPERATOR_SUPPORT_CONTINUITY_SHARED_CONTRACT_REF: &str =
    "continuity:m5_operator_support_continuity_summary:v1";

/// Record-kind tag for [`ContinuityRowSummary`] payloads.
pub const CONTINUITY_ROW_SUMMARY_RECORD_KIND: &str = "continuity_row_summary_record";

/// Record-kind tag for [`OperatorSupportContinuityPage`] payloads.
pub const OPERATOR_SUPPORT_CONTINUITY_PAGE_RECORD_KIND: &str =
    "operator_support_continuity_page_record";

/// Record-kind tag for [`OperatorSupportContinuitySummary`] payloads.
pub const OPERATOR_SUPPORT_CONTINUITY_SUMMARY_RECORD_KIND: &str =
    "operator_support_continuity_summary_record";

/// Record-kind tag for [`ContinuityRowSummaryOutcome`] payloads.
pub const CONTINUITY_ROW_SUMMARY_OUTCOME_RECORD_KIND: &str =
    "continuity_row_summary_outcome_record";

/// Record-kind tag for [`OperatorSupportContinuityDefect`] payloads.
pub const OPERATOR_SUPPORT_CONTINUITY_DEFECT_RECORD_KIND: &str =
    "operator_support_continuity_defect_record";

/// Record-kind tag for [`OperatorSupportContinuitySupportExport`] payloads.
pub const OPERATOR_SUPPORT_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "operator_support_continuity_support_export_record";

/// Repo-relative path of the canonical reviewer doc for this lane.
pub const OPERATOR_SUPPORT_CONTINUITY_DOC_REF: &str =
    "docs/m5/continuity/operator-and-support-truth-surfaces.md";

/// Repo-relative path of the checked-in artifact for this lane.
pub const OPERATOR_SUPPORT_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/m5/continuity/operator_support_continuity_summary.json";

/// Repo-relative path of the canonical JSON schema for this lane.
pub const OPERATOR_SUPPORT_CONTINUITY_SCHEMA_REF: &str =
    "schemas/continuity/operator_support_continuity_summary.schema.json";

/// Generic degraded phrasings that must never be shown when the exact continuity
/// row and its narrower fallback class are known.
///
/// The guardrail does not rely on this list alone — a phrasing also fails when
/// it names none of the active row, the degraded state, or the narrower
/// fallback. The list rejects the most common generic banners outright.
const GENERIC_DEGRADED_PHRASES: &[&str] = &[
    "service degraded",
    "service is degraded",
    "service unavailable",
    "service is unavailable",
    "service disruption",
    "degraded",
    "unavailable",
    "something went wrong",
    "an error occurred",
    "temporarily unavailable",
];

/// Typed reason an operator/support continuity summary narrows or is withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SummaryNarrowReasonClass {
    /// No narrowing is active.
    NotNarrowed,
    /// The summary does not name the exact continuity row in effect.
    ActiveContinuityRowUnnamed,
    /// Processing or storage locality, or the residency label, is missing.
    LocalityPostureMissing,
    /// Tenant scope or key-mode posture is missing for a managed-lane row.
    TenantKeyPostureMissing,
    /// The affected outage taxonomy state or plane is not labeled.
    OutageTaxonomyUnlabeled,
    /// An impaired lane does not name the narrower fallback that remains.
    NarrowerFallbackUndeclared,
    /// Generic degraded wording was used when the exact row and fallback are known.
    GenericDegradedWordingUsed,
    /// The summary carries admin-only routing or raw secret material.
    AdminOnlyMaterialLeaked,
    /// The canonical summary's backing continuity evidence is stale.
    CanonicalSummaryStale,
    /// The canonical summary's backing continuity evidence is missing.
    CanonicalSummaryMissing,
    /// The summary is not reused across every required operator/support surface.
    SurfaceReuseIncomplete,
    /// The summary's posture is inconsistent with its own claimed profile.
    ProfileMismatch,
}

impl SummaryNarrowReasonClass {
    /// Every narrow reason, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::NotNarrowed,
        Self::ActiveContinuityRowUnnamed,
        Self::LocalityPostureMissing,
        Self::TenantKeyPostureMissing,
        Self::OutageTaxonomyUnlabeled,
        Self::NarrowerFallbackUndeclared,
        Self::GenericDegradedWordingUsed,
        Self::AdminOnlyMaterialLeaked,
        Self::CanonicalSummaryStale,
        Self::CanonicalSummaryMissing,
        Self::SurfaceReuseIncomplete,
        Self::ProfileMismatch,
    ];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::ActiveContinuityRowUnnamed => "active_continuity_row_unnamed",
            Self::LocalityPostureMissing => "locality_posture_missing",
            Self::TenantKeyPostureMissing => "tenant_key_posture_missing",
            Self::OutageTaxonomyUnlabeled => "outage_taxonomy_unlabeled",
            Self::NarrowerFallbackUndeclared => "narrower_fallback_undeclared",
            Self::GenericDegradedWordingUsed => "generic_degraded_wording_used",
            Self::AdminOnlyMaterialLeaked => "admin_only_material_leaked",
            Self::CanonicalSummaryStale => "canonical_summary_stale",
            Self::CanonicalSummaryMissing => "canonical_summary_missing",
            Self::SurfaceReuseIncomplete => "surface_reuse_incomplete",
            Self::ProfileMismatch => "profile_mismatch",
        }
    }

    /// True when this reason withholds the summary entirely (a hard guardrail).
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::GenericDegradedWordingUsed | Self::AdminOnlyMaterialLeaked
        )
    }

    /// True when this reason holds the summary at preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(
            self,
            Self::ActiveContinuityRowUnnamed
                | Self::CanonicalSummaryMissing
                | Self::ProfileMismatch
        )
    }
}

/// Derives a qualification from the narrow reasons present on a summary.
///
/// Mirrors the matrix's lifecycle derivation: any withdrawal reason wins, then
/// any preview reason, then any narrowing reason drops to beta, otherwise stable.
fn qualification_from_reasons(
    reasons: &[SummaryNarrowReasonClass],
) -> ContinuityClaimQualificationClass {
    let mut saw_any = false;
    let mut saw_preview = false;
    for reason in reasons {
        if *reason == SummaryNarrowReasonClass::NotNarrowed {
            continue;
        }
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

/// The operator/support surfaces that must reuse one continuity summary.
///
/// These are exactly the surfaces named by the exit gate: About, Help, service
/// health, the support center, and support/issue-report exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuitySummarySurfaceCoverage {
    /// True when the About surface renders this summary.
    pub about: bool,
    /// True when Help renders this summary.
    pub help: bool,
    /// True when the service-health surface renders this summary.
    pub service_health: bool,
    /// True when the support center renders this summary.
    pub support_center: bool,
    /// True when support / issue-report exports embed this summary.
    pub support_export: bool,
}

impl ContinuitySummarySurfaceCoverage {
    /// Coverage with every operator/support surface enabled.
    pub const fn all_required() -> Self {
        Self {
            about: true,
            help: true,
            service_health: true,
            support_center: true,
            support_export: true,
        }
    }

    /// Coverage for a local-core row: every in-product and support-center
    /// surface, but support export is optional because a local-core row carries
    /// no managed continuity packet to attach.
    pub const fn local_core_required() -> Self {
        Self {
            about: true,
            help: true,
            service_health: true,
            support_center: true,
            support_export: false,
        }
    }

    /// True when every operator/support surface reuses the summary.
    pub const fn all_covered(&self) -> bool {
        self.about && self.help && self.service_health && self.support_center && self.support_export
    }

    /// True when the in-product and support-center surfaces reuse the summary.
    pub const fn local_core_covered(&self) -> bool {
        self.about && self.help && self.service_health && self.support_center
    }
}

/// Plain-language locality, tenant, and key posture for one continuity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalityKeyTenantPosture {
    /// Where processing happens.
    pub processing_locality: LocalityClass,
    /// Stable token for [`Self::processing_locality`].
    pub processing_locality_token: String,
    /// Where durable storage lives.
    pub storage_locality: LocalityClass,
    /// Stable token for [`Self::storage_locality`].
    pub storage_locality_token: String,
    /// Export-safe plain-language region or residency label.
    pub residency_label: String,
    /// Tenant or org boundary.
    pub tenant_scope: TenantScopeClass,
    /// Stable token for [`Self::tenant_scope`].
    pub tenant_scope_token: String,
    /// Export-safe plain-language tenant label.
    pub tenant_label: String,
    /// Key-mode posture protecting durable state.
    pub key_mode: KeyModeClass,
    /// Stable token for [`Self::key_mode`].
    pub key_mode_token: String,
    /// Export-safe plain-language key-posture label.
    pub key_label: String,
}

impl LocalityKeyTenantPosture {
    /// True when processing and storage locality are disclosed with a residency label.
    pub fn locality_is_disclosed(&self) -> bool {
        self.processing_locality != LocalityClass::Undisclosed
            && self.storage_locality != LocalityClass::Undisclosed
            && !self.residency_label.is_empty()
    }

    /// True when tenant scope and key mode are disclosed in plain language.
    pub fn tenant_key_is_disclosed(&self) -> bool {
        self.tenant_scope != TenantScopeClass::NotApplicable
            && self.key_mode != KeyModeClass::NotApplicable
            && !self.tenant_label.is_empty()
            && !self.key_label.is_empty()
    }
}

/// The affected outage-taxonomy label and narrower fallback for one row.
///
/// Reuses the control-plane-vs-data-plane outage vocabulary verbatim so the
/// operator/support summary speaks the same plain language as the service-health
/// outage cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedOutageLabel {
    /// Current operational severity of the row's managed lane.
    pub severity: ImpairmentSeverityClass,
    /// Stable token for [`Self::severity`].
    pub severity_token: String,
    /// Typed degraded state, distinguishing control- from data-plane impairment.
    pub degraded_state: OutageDegradedStateClass,
    /// Stable token for [`Self::degraded_state`].
    pub degraded_state_token: String,
    /// Which plane the impairment affects.
    pub affected_plane: PlaneImpairmentClass,
    /// Stable token for [`Self::affected_plane`].
    pub affected_plane_token: String,
    /// The narrower fallback that remains while the lane is impaired.
    pub narrower_fallback: DegradedFallbackClass,
    /// Stable token for [`Self::narrower_fallback`].
    pub narrower_fallback_token: String,
    /// Export-safe plain-language line the surfaces render for this row.
    pub status_phrasing: String,
}

impl AffectedOutageLabel {
    /// True when the row's managed lane is impaired in any way.
    pub fn is_impaired(&self) -> bool {
        self.severity.is_impaired()
    }

    /// True when an actual narrower fallback path is declared.
    pub fn has_active_fallback(&self) -> bool {
        self.narrower_fallback.is_active()
    }

    /// True when the taxonomy state is labeled consistently with the severity.
    pub fn is_labeled(&self) -> bool {
        if self.status_phrasing.is_empty() {
            return false;
        }
        if self.is_impaired() {
            self.degraded_state != OutageDegradedStateClass::Operational
        } else {
            self.degraded_state == OutageDegradedStateClass::Operational
        }
    }

    /// True when the rendered phrasing names the exact row, state, or fallback
    /// rather than a generic degraded banner.
    fn phrasing_is_specific(&self, row_label: &str) -> bool {
        let phrasing = self.status_phrasing.trim().to_lowercase();
        if phrasing.is_empty() {
            return false;
        }
        if GENERIC_DEGRADED_PHRASES
            .iter()
            .any(|generic| phrasing == *generic)
        {
            return false;
        }
        let fallback = self.narrower_fallback.plain().to_lowercase();
        let state = self.degraded_state.plain().to_lowercase();
        let row = row_label.trim().to_lowercase();
        (self.has_active_fallback() && phrasing.contains(&fallback))
            || phrasing.contains(&state)
            || (!row.is_empty() && phrasing.contains(&row))
    }
}

/// The backing continuity evidence and its freshness for one summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryEvidence {
    /// Freshness state of the backing continuity evidence.
    pub evidence_state: OutageEvidenceStateClass,
    /// Stable token for [`Self::evidence_state`].
    pub evidence_state_token: String,
    /// Opaque ref to the backing continuity packet or matrix row.
    pub continuity_packet_ref: String,
    /// UTC timestamp the summary was last refreshed; empty when evidence is missing.
    pub last_refreshed_at: String,
}

impl SummaryEvidence {
    /// True when backing continuity evidence is present.
    pub fn is_present(&self) -> bool {
        self.evidence_state != OutageEvidenceStateClass::Missing
    }

    /// True when the backing evidence is fresh enough to leave the claim stable.
    pub fn is_fresh(&self) -> bool {
        self.evidence_state.is_acceptable()
    }
}

/// Export-safety declaration for one summary.
///
/// A summary is only export-safe when admin-only routing and raw secret material
/// are both excluded; otherwise the summary is withheld.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryRedaction {
    /// True when admin-only internal routing is excluded.
    pub admin_routing_excluded: bool,
    /// True when raw secret material is excluded.
    pub raw_secret_material_excluded: bool,
    /// True when the summary is safe to copy, export, and attach to support packets.
    pub export_safe: bool,
}

impl SummaryRedaction {
    /// Builds a redaction declaration, deriving [`Self::export_safe`].
    pub fn new(admin_routing_excluded: bool, raw_secret_material_excluded: bool) -> Self {
        Self {
            admin_routing_excluded,
            raw_secret_material_excluded,
            export_safe: admin_routing_excluded && raw_secret_material_excluded,
        }
    }

    /// A fully export-safe declaration.
    pub fn safe() -> Self {
        Self::new(true, true)
    }
}

/// The canonical operator/support continuity summary for one claimed row.
///
/// This is the single object About, Help, service health, the support center,
/// and support/export packets all read. Once it is present, no surface needs a
/// bespoke continuity explanation: it names the exact continuity row in effect,
/// summarizes locality/tenant/key posture and the affected outage taxonomy in
/// plain product language, and carries the freshness of its own backing
/// evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityRowSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable summary identifier.
    pub summary_id: String,
    /// Opaque id of the exact continuity row in effect.
    pub active_continuity_row_id: String,
    /// Plain-language label naming the exact continuity row in effect.
    pub active_continuity_row_label: String,
    /// Opaque ref to the continuity-claim matrix row this summary mirrors.
    pub claim_matrix_ref: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Claimed deployment profile.
    pub profile_class: ContinuityProfileClass,
    /// Stable token for [`Self::profile_class`].
    pub profile_class_token: String,
    /// Continuity lane this row belongs to.
    pub continuity_lane: ContinuityLaneClass,
    /// Stable token for [`Self::continuity_lane`].
    pub continuity_lane_token: String,
    /// Plain-language locality, tenant, and key posture.
    pub posture: LocalityKeyTenantPosture,
    /// Affected outage taxonomy and narrower fallback.
    pub outage: AffectedOutageLabel,
    /// Backing continuity evidence and its freshness.
    pub evidence: SummaryEvidence,
    /// Export-safety declaration.
    pub redaction: SummaryRedaction,
    /// The lifecycle label the summary is put forward as.
    pub claimed_qualification: ContinuityClaimQualificationClass,
    /// Stable token for [`Self::claimed_qualification`].
    pub claimed_qualification_token: String,
    /// Required operator/support surface coverage.
    pub surface_coverage: ContinuitySummarySurfaceCoverage,
}

impl ContinuityRowSummary {
    /// True when this summary rides the local-core continuity lane.
    ///
    /// A local-core summary keeps its claim without any managed lane, so it never
    /// narrows or is withheld because a managed lane went stale, missing, or
    /// impaired — the guardrail against conflating a stale managed row with the
    /// local core.
    pub fn is_local_core(&self) -> bool {
        self.continuity_lane == ContinuityLaneClass::LocalCore
            && self.profile_class == ContinuityProfileClass::LocalOnly
    }

    /// True when this summary is held to managed-lane disclosure and freshness.
    pub fn in_release_scope(&self) -> bool {
        !self.is_local_core()
    }

    /// True when the summary names the exact continuity row in effect.
    pub fn names_exact_row(&self) -> bool {
        !self.active_continuity_row_id.is_empty() && !self.active_continuity_row_label.is_empty()
    }

    /// The narrow reasons this summary carries.
    fn narrow_reasons(&self) -> Vec<SummaryNarrowReasonClass> {
        let mut reasons = Vec::new();

        if !self.names_exact_row() {
            reasons.push(SummaryNarrowReasonClass::ActiveContinuityRowUnnamed);
        }
        if !self.posture.locality_is_disclosed() {
            reasons.push(SummaryNarrowReasonClass::LocalityPostureMissing);
        }
        if !self.outage.is_labeled() {
            reasons.push(SummaryNarrowReasonClass::OutageTaxonomyUnlabeled);
        }
        if self.outage.is_impaired() && !self.outage.has_active_fallback() {
            reasons.push(SummaryNarrowReasonClass::NarrowerFallbackUndeclared);
        }
        // Hard guardrail: generic degraded wording when the specifics are known.
        if self.outage.is_impaired()
            && self.outage.has_active_fallback()
            && !self
                .outage
                .phrasing_is_specific(&self.active_continuity_row_label)
        {
            reasons.push(SummaryNarrowReasonClass::GenericDegradedWordingUsed);
        }
        // Hard guardrail: a summary that leaks admin-only material is withheld.
        if !self.redaction.export_safe {
            reasons.push(SummaryNarrowReasonClass::AdminOnlyMaterialLeaked);
        }

        if self.in_release_scope() {
            if !self.posture.tenant_key_is_disclosed() {
                reasons.push(SummaryNarrowReasonClass::TenantKeyPostureMissing);
            }
            match self.evidence.evidence_state {
                OutageEvidenceStateClass::Missing => {
                    reasons.push(SummaryNarrowReasonClass::CanonicalSummaryMissing)
                }
                OutageEvidenceStateClass::StaleNeedsRefresh => {
                    reasons.push(SummaryNarrowReasonClass::CanonicalSummaryStale)
                }
                OutageEvidenceStateClass::Current | OutageEvidenceStateClass::StaleWithinGrace => {
                    if self.evidence.last_refreshed_at.is_empty() {
                        reasons.push(SummaryNarrowReasonClass::CanonicalSummaryStale);
                    }
                }
            }
            if self.profile_class == ContinuityProfileClass::Sovereign
                && self.posture.tenant_scope == TenantScopeClass::SharedMultiTenant
            {
                reasons.push(SummaryNarrowReasonClass::ProfileMismatch);
            }
            if self.profile_class.is_self_governed()
                && self.posture.key_mode == KeyModeClass::VendorManagedKeys
            {
                reasons.push(SummaryNarrowReasonClass::ProfileMismatch);
            }
            if !self.surface_coverage.all_covered() {
                reasons.push(SummaryNarrowReasonClass::SurfaceReuseIncomplete);
            }
        } else if !self.surface_coverage.local_core_covered() {
            reasons.push(SummaryNarrowReasonClass::SurfaceReuseIncomplete);
        }

        reasons.sort();
        reasons.dedup();
        reasons
    }

    /// The lifecycle label this summary effectively publishes after narrowing.
    fn effective_qualification(&self) -> ContinuityClaimQualificationClass {
        let reasons = self.narrow_reasons();
        self.claimed_qualification
            .max(qualification_from_reasons(&reasons))
    }
}

/// Per-summary verdict joining a summary to its computed qualification and reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityRowSummaryOutcome {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque summary identifier this outcome describes.
    pub summary_id: String,
    /// Opaque id of the exact continuity row in effect.
    pub active_continuity_row_id: String,
    /// Stable token for the summary's claimed profile.
    pub profile_class_token: String,
    /// True when the summary is held to managed-lane disclosure and freshness.
    pub in_release_scope: bool,
    /// True when the row's managed lane is currently impaired.
    pub impaired: bool,
    /// Stable token for the current operational severity.
    pub severity_token: String,
    /// Stable token for the affected degraded state.
    pub degraded_state_token: String,
    /// Stable token for the narrower fallback that remains.
    pub narrower_fallback_token: String,
    /// Stable token for the backing-evidence freshness state.
    pub evidence_state_token: String,
    /// Stable token for the label the summary is put forward as.
    pub claimed_qualification_token: String,
    /// Stable token for the label the summary effectively publishes after narrowing.
    pub effective_qualification_token: String,
    /// True when the summary narrowed below its claimed label.
    pub narrowed: bool,
    /// True when the summary is withheld from every surface.
    pub withheld: bool,
    /// True when the summary is export-safe.
    pub export_safe: bool,
    /// Stable narrow-reason tokens active on the summary.
    pub narrow_reason_tokens: Vec<String>,
}

/// Typed defect emitted by the operator/support continuity audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorSupportContinuityDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque defect identifier.
    pub defect_id: String,
    /// Typed narrow reason.
    pub narrow_reason: SummaryNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Opaque source summary id or page concern that triggered the defect.
    pub source: String,
    /// Export-safe explanation of the defect.
    pub note: String,
}

impl OperatorSupportContinuityDefect {
    fn new(
        narrow_reason: SummaryNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: OPERATOR_SUPPORT_CONTINUITY_DEFECT_RECORD_KIND.to_owned(),
            schema_version: OPERATOR_SUPPORT_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: OPERATOR_SUPPORT_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "continuity:defect:operator-support:{}:{}",
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

/// Full auditable input for the operator/support continuity page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorSupportContinuityInput {
    /// Reviewable label for the page.
    pub page_label: String,
    /// Opaque ref to the continuity-claim matrix the summaries mirror.
    pub claim_matrix_ref: String,
    /// The canonical continuity summaries, one per claimed row.
    pub summaries: Vec<ContinuityRowSummary>,
}

/// Aggregate summary for an operator/support continuity page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorSupportContinuitySummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Overall qualification for the page.
    pub overall_qualification_token: String,
    /// Number of continuity summaries on the page.
    pub summary_count: usize,
    /// Number of summaries held to managed-lane disclosure and freshness.
    pub release_scope_count: usize,
    /// Number of summaries on the local-core continuity lane.
    pub local_core_count: usize,
    /// Number of summaries whose managed lane is currently impaired.
    pub impaired_count: usize,
    /// Number of summaries whose managed lane is currently operational.
    pub operational_count: usize,
    /// Number of summaries that narrowed below their claimed label.
    pub narrowed_count: usize,
    /// Number of summaries withheld from every surface.
    pub withheld_count: usize,
    /// Number of summaries with stale backing evidence.
    pub stale_evidence_count: usize,
    /// Number of summaries with missing backing evidence.
    pub missing_evidence_count: usize,
    /// Number of summaries that are export-safe.
    pub export_safe_count: usize,
    /// Number of summaries reused across every required operator/support surface.
    pub surfaces_fully_covered_count: usize,
    /// Number of defects recorded for the page.
    pub defect_count: usize,
}

/// Canonical proof packet for the operator/support continuity surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorSupportContinuityPage {
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
    /// Opaque ref to the continuity-claim matrix the summaries mirror.
    pub claim_matrix_ref: String,
    /// Aggregate summary derived from the embedded input and defects.
    pub summary: OperatorSupportContinuitySummary,
    /// Typed defects for the packet.
    pub defects: Vec<OperatorSupportContinuityDefect>,
    /// Per-summary verdicts joining each summary to its computed qualification.
    pub summary_outcomes: Vec<ContinuityRowSummaryOutcome>,
    /// The audited input embedded as evidence.
    pub input: OperatorSupportContinuityInput,
}

impl OperatorSupportContinuityPage {
    /// Builds an operator/support continuity page from the supplied input.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: OperatorSupportContinuityInput,
    ) -> Self {
        let defects = audit_operator_support_continuity_input(&input);
        let summary_outcomes = build_summary_outcomes(&input);
        let summary = build_summary(&input, &summary_outcomes, &defects);
        Self {
            record_kind: OPERATOR_SUPPORT_CONTINUITY_PAGE_RECORD_KIND.to_owned(),
            schema_version: OPERATOR_SUPPORT_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: OPERATOR_SUPPORT_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            claim_matrix_ref: input.claim_matrix_ref.clone(),
            summary,
            defects,
            summary_outcomes,
            input,
        }
    }

    /// True when the page qualifies stable (no defect narrowed any summary).
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == ContinuityClaimQualificationClass::Stable.as_str()
    }

    /// True when no defect was recorded for the page.
    pub fn is_structurally_clean(&self) -> bool {
        self.defects.is_empty()
    }

    /// True when every summary names the exact continuity row in effect.
    pub fn names_every_active_row(&self) -> bool {
        self.input
            .summaries
            .iter()
            .all(ContinuityRowSummary::names_exact_row)
    }

    /// True when every summary is reused across its required operator/support surfaces.
    pub fn every_surface_covered(&self) -> bool {
        self.input.summaries.iter().all(|summary| {
            if summary.in_release_scope() {
                summary.surface_coverage.all_covered()
            } else {
                summary.surface_coverage.local_core_covered()
            }
        })
    }

    /// Returns the computed outcome for a summary id, if present.
    pub fn summary_outcome(&self, summary_id: &str) -> Option<&ContinuityRowSummaryOutcome> {
        self.summary_outcomes
            .iter()
            .find(|outcome| outcome.summary_id == summary_id)
    }
}

/// Support-export wrapper for the operator/support continuity page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorSupportContinuitySupportExport {
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
    /// The operator/support continuity page embedded as evidence.
    pub page: OperatorSupportContinuityPage,
    /// Typed narrow reasons present in the embedded page.
    pub narrow_reasons_present: Vec<SummaryNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl OperatorSupportContinuitySupportExport {
    /// Wraps an operator/support continuity page inside a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: OperatorSupportContinuityPage,
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
            record_kind: OPERATOR_SUPPORT_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: OPERATOR_SUPPORT_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: OPERATOR_SUPPORT_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

/// Re-runs the operator/support continuity audit over the embedded input.
pub fn audit_operator_support_continuity_page(
    page: &OperatorSupportContinuityPage,
) -> Vec<OperatorSupportContinuityDefect> {
    audit_operator_support_continuity_input(&page.input)
}

/// Validates a page and returns `Ok(())` when the audit is clean.
pub fn validate_operator_support_continuity_page(
    page: &OperatorSupportContinuityPage,
) -> Result<(), Vec<OperatorSupportContinuityDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

/// Returns the seeded clean operator/support continuity page.
pub fn seeded_operator_support_continuity_page() -> OperatorSupportContinuityPage {
    OperatorSupportContinuityPage::new(
        "continuity:operator-support:seeded",
        "Operator and support continuity truth surfaces",
        "2026-06-19T00:00:00Z",
        seeded_operator_support_continuity_input(),
    )
}

/// Returns the seeded input used by the canonical page.
pub fn seeded_operator_support_continuity_input() -> OperatorSupportContinuityInput {
    OperatorSupportContinuityInput {
        page_label: "Operator/support continuity summaries for claimed managed, self-hosted, and sovereign rows"
            .to_owned(),
        claim_matrix_ref: "artifacts/m5/continuity/claim_rows_and_drill_schedule.md".to_owned(),
        summaries: seeded_summaries(),
    }
}

fn audit_operator_support_continuity_input(
    input: &OperatorSupportContinuityInput,
) -> Vec<OperatorSupportContinuityDefect> {
    let mut defects = Vec::new();
    for summary in &input.summaries {
        audit_summary(summary, &mut defects);
    }
    defects
}

fn audit_summary(
    summary: &ContinuityRowSummary,
    defects: &mut Vec<OperatorSupportContinuityDefect>,
) {
    for reason in summary.narrow_reasons() {
        defects.push(OperatorSupportContinuityDefect::new(
            reason,
            summary.summary_id.clone(),
            defect_note(reason),
        ));
    }
}

fn defect_note(reason: SummaryNarrowReasonClass) -> &'static str {
    match reason {
        SummaryNarrowReasonClass::NotNarrowed => "no narrowing is active",
        SummaryNarrowReasonClass::ActiveContinuityRowUnnamed => {
            "the summary must name the exact continuity row in effect, not a generic banner"
        }
        SummaryNarrowReasonClass::LocalityPostureMissing => {
            "the summary must disclose processing locality, storage locality, and a residency label"
        }
        SummaryNarrowReasonClass::TenantKeyPostureMissing => {
            "a managed, self-hosted, or sovereign summary must disclose tenant scope and key mode in plain language"
        }
        SummaryNarrowReasonClass::OutageTaxonomyUnlabeled => {
            "the summary must label its outage-taxonomy state consistently with its current severity"
        }
        SummaryNarrowReasonClass::NarrowerFallbackUndeclared => {
            "an impaired lane must name the narrower fallback that remains"
        }
        SummaryNarrowReasonClass::GenericDegradedWordingUsed => {
            "generic degraded wording may not be used when the exact continuity row and narrower fallback are known"
        }
        SummaryNarrowReasonClass::AdminOnlyMaterialLeaked => {
            "the summary must exclude admin-only internal routing and raw secret material"
        }
        SummaryNarrowReasonClass::CanonicalSummaryStale => {
            "the backing continuity evidence is stale; the claim narrows until it is refreshed"
        }
        SummaryNarrowReasonClass::CanonicalSummaryMissing => {
            "the backing continuity evidence is missing; the claim narrows to preview"
        }
        SummaryNarrowReasonClass::SurfaceReuseIncomplete => {
            "the summary must be reused by About, Help, service health, the support center, and support exports"
        }
        SummaryNarrowReasonClass::ProfileMismatch => {
            "the summary's posture is inconsistent with its own claimed profile"
        }
    }
}

fn build_summary_outcomes(
    input: &OperatorSupportContinuityInput,
) -> Vec<ContinuityRowSummaryOutcome> {
    input
        .summaries
        .iter()
        .map(|summary| {
            let reasons = summary.narrow_reasons();
            let effective = summary.effective_qualification();
            let reason_tokens: Vec<String> = reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect();
            ContinuityRowSummaryOutcome {
                record_kind: CONTINUITY_ROW_SUMMARY_OUTCOME_RECORD_KIND.to_owned(),
                schema_version: OPERATOR_SUPPORT_CONTINUITY_SCHEMA_VERSION,
                shared_contract_ref: OPERATOR_SUPPORT_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
                summary_id: summary.summary_id.clone(),
                active_continuity_row_id: summary.active_continuity_row_id.clone(),
                profile_class_token: summary.profile_class.as_str().to_owned(),
                in_release_scope: summary.in_release_scope(),
                impaired: summary.outage.is_impaired(),
                severity_token: summary.outage.severity.as_str().to_owned(),
                degraded_state_token: summary.outage.degraded_state.as_str().to_owned(),
                narrower_fallback_token: summary.outage.narrower_fallback.as_str().to_owned(),
                evidence_state_token: summary.evidence.evidence_state.as_str().to_owned(),
                claimed_qualification_token: summary.claimed_qualification.as_str().to_owned(),
                effective_qualification_token: effective.as_str().to_owned(),
                narrowed: effective != summary.claimed_qualification,
                withheld: effective == ContinuityClaimQualificationClass::Withdrawn,
                export_safe: summary.redaction.export_safe,
                narrow_reason_tokens: reason_tokens,
            }
        })
        .collect()
}

fn build_summary(
    input: &OperatorSupportContinuityInput,
    outcomes: &[ContinuityRowSummaryOutcome],
    defects: &[OperatorSupportContinuityDefect],
) -> OperatorSupportContinuitySummary {
    let overall = outcomes
        .iter()
        .map(|outcome| qualification_token_rank(&outcome.effective_qualification_token))
        .max()
        .map(qualification_from_rank)
        .unwrap_or(ContinuityClaimQualificationClass::Stable);

    let count_evidence = |token: &str| {
        input
            .summaries
            .iter()
            .filter(|summary| {
                summary.in_release_scope() && summary.evidence.evidence_state.as_str() == token
            })
            .count()
    };

    OperatorSupportContinuitySummary {
        record_kind: OPERATOR_SUPPORT_CONTINUITY_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: OPERATOR_SUPPORT_CONTINUITY_SCHEMA_VERSION,
        shared_contract_ref: OPERATOR_SUPPORT_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
        overall_qualification_token: overall.as_str().to_owned(),
        summary_count: input.summaries.len(),
        release_scope_count: input
            .summaries
            .iter()
            .filter(|summary| summary.in_release_scope())
            .count(),
        local_core_count: input
            .summaries
            .iter()
            .filter(|summary| summary.is_local_core())
            .count(),
        impaired_count: outcomes.iter().filter(|outcome| outcome.impaired).count(),
        operational_count: outcomes.iter().filter(|outcome| !outcome.impaired).count(),
        narrowed_count: outcomes.iter().filter(|outcome| outcome.narrowed).count(),
        withheld_count: outcomes.iter().filter(|outcome| outcome.withheld).count(),
        stale_evidence_count: count_evidence(OutageEvidenceStateClass::StaleNeedsRefresh.as_str()),
        missing_evidence_count: count_evidence(OutageEvidenceStateClass::Missing.as_str()),
        export_safe_count: outcomes
            .iter()
            .filter(|outcome| outcome.export_safe)
            .count(),
        surfaces_fully_covered_count: input
            .summaries
            .iter()
            .filter(|summary| {
                if summary.in_release_scope() {
                    summary.surface_coverage.all_covered()
                } else {
                    summary.surface_coverage.local_core_covered()
                }
            })
            .count(),
        defect_count: defects.len(),
    }
}

fn qualification_token_rank(token: &str) -> u8 {
    match token {
        "stable" => 0,
        "beta" => 1,
        "preview" => 2,
        "withdrawn" => 3,
        _ => 0,
    }
}

fn qualification_from_rank(rank: u8) -> ContinuityClaimQualificationClass {
    match rank {
        0 => ContinuityClaimQualificationClass::Stable,
        1 => ContinuityClaimQualificationClass::Beta,
        2 => ContinuityClaimQualificationClass::Preview,
        _ => ContinuityClaimQualificationClass::Withdrawn,
    }
}

fn seeded_summaries() -> Vec<ContinuityRowSummary> {
    vec![
        summary(
            "continuity:operator-support:managed-cloud-sync",
            "continuity-row:managed-cloud-sync",
            "Managed cloud workspace sync and backup",
            "Managed cloud workspace sync and backup",
            ContinuityProfileClass::Managed,
            ContinuityLaneClass::ManagedLane,
            posture(
                LocalityClass::MultiRegion,
                LocalityClass::SingleRegion,
                "United States managed regions (us-east primary, us-west replica)",
                TenantScopeClass::DedicatedTenant,
                "your dedicated managed tenant",
                KeyModeClass::VendorManagedKeys,
                "vendor-managed keys with automated rotation",
            ),
            operational_outage("Managed cloud sync is operational; nothing has failed over."),
            evidence(OutageEvidenceStateClass::Current, "2026-06-18"),
            ContinuitySummarySurfaceCoverage::all_required(),
        ),
        summary(
            "continuity:operator-support:managed-relay",
            "continuity-row:managed-relay-failover",
            "Managed relay and collaboration",
            "Managed relay and collaboration failover",
            ContinuityProfileClass::Managed,
            ContinuityLaneClass::ManagedLane,
            posture(
                LocalityClass::MultiRegion,
                LocalityClass::MultiRegion,
                "United States managed regions (active/active relay)",
                TenantScopeClass::SharedMultiTenant,
                "a shared multi-tenant managed relay",
                KeyModeClass::VendorManagedKeys,
                "vendor-managed transport keys",
            ),
            // Truthfully degraded: control plane impaired, work queues locally.
            AffectedOutageLabel {
                severity: ImpairmentSeverityClass::Degraded,
                severity_token: ImpairmentSeverityClass::Degraded.as_str().to_owned(),
                degraded_state:
                    OutageDegradedStateClass::ControlPlaneImpairedLocalCorePreserved,
                degraded_state_token:
                    OutageDegradedStateClass::ControlPlaneImpairedLocalCorePreserved
                        .as_str()
                        .to_owned(),
                affected_plane: PlaneImpairmentClass::ControlPlaneImpairment,
                affected_plane_token: PlaneImpairmentClass::ControlPlaneImpairment
                    .as_str()
                    .to_owned(),
                narrower_fallback: DegradedFallbackClass::QueueAndReconcile,
                narrower_fallback_token: DegradedFallbackClass::QueueAndReconcile
                    .as_str()
                    .to_owned(),
                status_phrasing:
                    "Managed relay control plane is impaired; edits queue locally and reconcile on reconnect."
                        .to_owned(),
            },
            evidence(OutageEvidenceStateClass::Current, "2026-06-17"),
            ContinuitySummarySurfaceCoverage::all_required(),
        ),
        summary(
            "continuity:operator-support:self-hosted-restore",
            "continuity-row:self-hosted-restore",
            "Customer self-hosted restore",
            "Customer self-hosted restore and rebuild",
            ContinuityProfileClass::SelfHosted,
            ContinuityLaneClass::ManagedLane,
            posture(
                LocalityClass::CustomerRegion,
                LocalityClass::CustomerRegion,
                "your self-hosted region (operated by your team)",
                TenantScopeClass::CustomerTenant,
                "your customer-operated tenant",
                KeyModeClass::CustomerManagedKeys,
                "customer-managed keys in your KMS",
            ),
            operational_outage(
                "Self-hosted restore is operational; recovery reproduces the same workspace identity.",
            ),
            evidence(OutageEvidenceStateClass::Current, "2026-05-30"),
            ContinuitySummarySurfaceCoverage::all_required(),
        ),
        summary(
            "continuity:operator-support:sovereign-airgap",
            "continuity-row:sovereign-airgap-snapshot",
            "Sovereign air-gapped snapshot",
            "Sovereign air-gapped snapshot and replication",
            ContinuityProfileClass::Sovereign,
            ContinuityLaneClass::ManagedLane,
            posture(
                LocalityClass::InCountrySovereign,
                LocalityClass::AirGappedIsolated,
                "your in-country sovereign boundary (air-gapped)",
                TenantScopeClass::DedicatedTenant,
                "your dedicated sovereign tenant",
                KeyModeClass::CustomerHeldRoot,
                "a customer-held root key with no vendor escrow",
            ),
            operational_outage(
                "Sovereign snapshot replication is operational inside the air-gapped boundary.",
            ),
            evidence(OutageEvidenceStateClass::StaleWithinGrace, "2026-04-15"),
            ContinuitySummarySurfaceCoverage::all_required(),
        ),
        summary(
            "continuity:operator-support:local-desktop-core",
            "continuity-row:local-desktop-core",
            "Local desktop core continuity",
            "Local desktop core continuity",
            ContinuityProfileClass::LocalOnly,
            ContinuityLaneClass::LocalCore,
            posture(
                LocalityClass::DeviceLocal,
                LocalityClass::DeviceLocal,
                "entirely on your device",
                TenantScopeClass::SingleUserLocal,
                "your single local user",
                KeyModeClass::LocalOsKeystore,
                "keys held in your operating-system keystore",
            ),
            operational_outage(
                "Local editing core is operational and continues even when every managed lane is down.",
            ),
            evidence(OutageEvidenceStateClass::Current, "2026-06-19"),
            ContinuitySummarySurfaceCoverage::local_core_required(),
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn summary(
    summary_id: &str,
    active_continuity_row_id: &str,
    active_continuity_row_label: &str,
    surface_label: &str,
    profile_class: ContinuityProfileClass,
    continuity_lane: ContinuityLaneClass,
    posture: LocalityKeyTenantPosture,
    outage: AffectedOutageLabel,
    evidence: SummaryEvidence,
    surface_coverage: ContinuitySummarySurfaceCoverage,
) -> ContinuityRowSummary {
    ContinuityRowSummary {
        record_kind: CONTINUITY_ROW_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: OPERATOR_SUPPORT_CONTINUITY_SCHEMA_VERSION,
        shared_contract_ref: OPERATOR_SUPPORT_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
        summary_id: summary_id.to_owned(),
        active_continuity_row_id: active_continuity_row_id.to_owned(),
        active_continuity_row_label: active_continuity_row_label.to_owned(),
        claim_matrix_ref: "artifacts/m5/continuity/claim_rows_and_drill_schedule.md".to_owned(),
        surface_label: surface_label.to_owned(),
        profile_class,
        profile_class_token: profile_class.as_str().to_owned(),
        continuity_lane,
        continuity_lane_token: continuity_lane.as_str().to_owned(),
        posture,
        outage,
        evidence,
        redaction: SummaryRedaction::safe(),
        claimed_qualification: ContinuityClaimQualificationClass::Stable,
        claimed_qualification_token: ContinuityClaimQualificationClass::Stable
            .as_str()
            .to_owned(),
        surface_coverage,
    }
}

#[allow(clippy::too_many_arguments)]
fn posture(
    processing_locality: LocalityClass,
    storage_locality: LocalityClass,
    residency_label: &str,
    tenant_scope: TenantScopeClass,
    tenant_label: &str,
    key_mode: KeyModeClass,
    key_label: &str,
) -> LocalityKeyTenantPosture {
    LocalityKeyTenantPosture {
        processing_locality,
        processing_locality_token: processing_locality.as_str().to_owned(),
        storage_locality,
        storage_locality_token: storage_locality.as_str().to_owned(),
        residency_label: residency_label.to_owned(),
        tenant_scope,
        tenant_scope_token: tenant_scope.as_str().to_owned(),
        tenant_label: tenant_label.to_owned(),
        key_mode,
        key_mode_token: key_mode.as_str().to_owned(),
        key_label: key_label.to_owned(),
    }
}

fn operational_outage(status_phrasing: &str) -> AffectedOutageLabel {
    AffectedOutageLabel {
        severity: ImpairmentSeverityClass::Operational,
        severity_token: ImpairmentSeverityClass::Operational.as_str().to_owned(),
        degraded_state: OutageDegradedStateClass::Operational,
        degraded_state_token: OutageDegradedStateClass::Operational.as_str().to_owned(),
        affected_plane: PlaneImpairmentClass::BothPlanes,
        affected_plane_token: PlaneImpairmentClass::BothPlanes.as_str().to_owned(),
        narrower_fallback: DegradedFallbackClass::NoneNeeded,
        narrower_fallback_token: DegradedFallbackClass::NoneNeeded.as_str().to_owned(),
        status_phrasing: status_phrasing.to_owned(),
    }
}

fn evidence(evidence_state: OutageEvidenceStateClass, last_refreshed_at: &str) -> SummaryEvidence {
    SummaryEvidence {
        evidence_state,
        evidence_state_token: evidence_state.as_str().to_owned(),
        continuity_packet_ref: "artifacts/m5/continuity/claim_rows_and_drill_schedule.md"
            .to_owned(),
        last_refreshed_at: if evidence_state == OutageEvidenceStateClass::Missing {
            String::new()
        } else {
            last_refreshed_at.to_owned()
        },
    }
}
