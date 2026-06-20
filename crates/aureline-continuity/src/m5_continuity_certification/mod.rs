//! Canonical continuity certification verdict for claimed managed, self-hosted,
//! and sovereign product rows.
//!
//! Where the upstream continuity lanes each freeze one slice of continuity truth —
//! [`m5_locality_descriptors_and_tenant_cards`](crate::m5_locality_descriptors_and_tenant_cards)
//! and the
//! [`m5_locality_tenant_keymode_and_drill_matrix`](crate::m5_locality_tenant_keymode_and_drill_matrix)
//! for locality/tenant/key posture, the
//! [`m5_control_plane_vs_data_plane_outage`](crate::m5_control_plane_vs_data_plane_outage)
//! taxonomy for typed degradation, the
//! [`m5_backup_restore_failover_packets`](crate::m5_backup_restore_failover_packets)
//! and [`m5_restore_from_backup_reviews`](crate::m5_restore_from_backup_reviews)
//! lanes for backup/restore/failover drills and restore identity, the
//! [`m5_mirror_airgap_continuity_packets`](crate::m5_mirror_airgap_continuity_packets)
//! lane for mirror/offline continuity, and the
//! [`m5_continuity_freshness_slo`](crate::m5_continuity_freshness_slo) dashboard
//! for proof freshness — this module folds them into one certification verdict per
//! claimed row. It is the canonical source release packets, Help/About truth,
//! service-health summaries, support exports, and partner qualification packets
//! read instead of re-deriving "is this continuity claim certified?" by hand.
//!
//! For every claimed continuity row the report answers:
//!
//! 1. Is the row in certification scope (any managed, self-hosted, or sovereign
//!    surface, or a row carrying a claimed managed dependency), or does it ride
//!    the local-core continuity lane that is never held to managed evidence?
//! 2. For each required continuity dimension — locality/tenant/key disclosure,
//!    typed control-plane/data-plane degradation, current backup/restore/failover
//!    drill, restore-identity/partial-loss semantics, mirror/offline continuity
//!    where the row is air-gapped or mirror-only, and proof freshness — is the
//!    backing evidence current, stale, partial, missing, or profile-mismatched?
//! 3. Does the row earn a current certification verdict, or does its claim narrow
//!    automatically — to beta, preview, or withdrawn — driven by whichever
//!    dimension's evidence is stale, partial, missing, or profile-mismatched?
//!
//! Two hard guardrails hold regardless of input:
//!
//! - The local-core continuity lane never narrows or withdraws on managed
//!   evidence; a stale managed row may not be conflated with the local core.
//! - A single reference-environment drill may not stand in for every claimed
//!   profile row: when two certification-scope rows reuse the same
//!   backup/restore/failover evidence ref, both narrow.
//!
//! The packet is metadata-only. It carries closed-vocabulary tokens, export-safe
//! labels, UTC dates, and opaque refs. Raw backup bytes, raw drill logs, raw KMS
//! handles, raw tenant identifiers, and secret material never cross this boundary.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::m5_locality_tenant_keymode_and_drill_matrix::{
    ClaimSurfaceVisibility, ContinuityClaimQualificationClass, ContinuityLaneClass,
    ContinuityProfileClass,
};

#[cfg(test)]
mod tests;

/// Schema version carried on every record in this module.
pub const CONTINUITY_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const CONTINUITY_CERTIFICATION_SHARED_CONTRACT_REF: &str =
    "continuity:m5_continuity_certification:v1";

/// Record-kind tag for [`ContinuityCertificationReport`] payloads.
pub const CONTINUITY_CERTIFICATION_REPORT_RECORD_KIND: &str =
    "continuity_certification_report_record";

/// Record-kind tag for [`ContinuityCertificationSummary`] payloads.
pub const CONTINUITY_CERTIFICATION_SUMMARY_RECORD_KIND: &str =
    "continuity_certification_summary_record";

/// Record-kind tag for [`CertifiedRowOutcome`] payloads.
pub const CERTIFIED_ROW_OUTCOME_RECORD_KIND: &str = "certified_row_outcome_record";

/// Record-kind tag for [`ContinuityCertificationDefect`] payloads.
pub const CONTINUITY_CERTIFICATION_DEFECT_RECORD_KIND: &str =
    "continuity_certification_defect_record";

/// Record-kind tag for [`ContinuityCertificationSupportExport`] payloads.
pub const CONTINUITY_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "continuity_certification_support_export_record";

/// Repo-relative path of the canonical reviewer doc for this lane.
pub const CONTINUITY_CERTIFICATION_DOC_REF: &str =
    "docs/m5/continuity/qualified-managed-and-self-hosted-rows.md";

/// Repo-relative path of the checked-in certified-row registry artifact.
pub const CONTINUITY_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/m5/continuity/certification/certified_rows.json";

/// Repo-relative path of the canonical JSON schema for this lane.
pub const CONTINUITY_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/continuity/continuity_certification_report.schema.json";

/// One continuity dimension a certification-scope row must prove.
///
/// Each dimension maps to an upstream continuity lane; the certification report
/// folds the per-dimension evidence state into one verdict per row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDimension {
    /// Processing/storage locality, tenant boundary, and key-mode disclosure.
    LocalityTenantKey,
    /// Typed control-plane versus data-plane degradation taxonomy.
    ControlDataPlaneDegradation,
    /// Current backup, restore, and failover drill evidence.
    BackupRestoreFailover,
    /// Restore-identity and partial-loss semantics for recovery.
    RestoreIdentityPartialLoss,
    /// Mirror-only and air-gapped offline continuity posture.
    MirrorOfflineContinuity,
    /// Continuity-proof freshness against its SLO.
    DrillFreshnessSlo,
}

impl CertificationDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalityTenantKey,
        Self::ControlDataPlaneDegradation,
        Self::BackupRestoreFailover,
        Self::RestoreIdentityPartialLoss,
        Self::MirrorOfflineContinuity,
        Self::DrillFreshnessSlo,
    ];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalityTenantKey => "locality_tenant_key",
            Self::ControlDataPlaneDegradation => "control_data_plane_degradation",
            Self::BackupRestoreFailover => "backup_restore_failover",
            Self::RestoreIdentityPartialLoss => "restore_identity_partial_loss",
            Self::MirrorOfflineContinuity => "mirror_offline_continuity",
            Self::DrillFreshnessSlo => "drill_freshness_slo",
        }
    }

    /// True when this dimension carries a backup/restore/failover drill ref that
    /// may not be shared across rows.
    pub const fn is_drill_dimension(self) -> bool {
        matches!(self, Self::BackupRestoreFailover)
    }

    /// The narrow reason this dimension contributes when its evidence is not certified.
    pub const fn narrow_reason(self) -> CertificationNarrowReasonClass {
        match self {
            Self::LocalityTenantKey => CertificationNarrowReasonClass::LocalityTenantKeyUncertified,
            Self::ControlDataPlaneDegradation => {
                CertificationNarrowReasonClass::ControlDataPlaneDegradationUncertified
            }
            Self::BackupRestoreFailover => {
                CertificationNarrowReasonClass::BackupRestoreFailoverUncertified
            }
            Self::RestoreIdentityPartialLoss => {
                CertificationNarrowReasonClass::RestoreIdentityPartialLossUncertified
            }
            Self::MirrorOfflineContinuity => {
                CertificationNarrowReasonClass::MirrorOfflineContinuityUncertified
            }
            Self::DrillFreshnessSlo => CertificationNarrowReasonClass::DrillFreshnessUncertified,
        }
    }
}

/// The state of the backing evidence for one continuity dimension on one row.
///
/// `current` and `not_applicable` keep a claim certified; every other state
/// forces the row to narrow below its claimed label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationEvidenceState {
    /// Evidence is present, complete, and within its freshness SLO.
    Current,
    /// Evidence exists but is stale and must be refreshed.
    Stale,
    /// Evidence covers only part of the claimed scope.
    Partial,
    /// No evidence backs this dimension.
    Missing,
    /// Evidence contradicts the claimed profile or posture.
    ProfileMismatched,
    /// The dimension does not apply to this row.
    NotApplicable,
}

impl CertificationEvidenceState {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::Missing => "missing",
            Self::ProfileMismatched => "profile_mismatched",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the state keeps the dimension certified (current or not applicable).
    pub const fn is_certified(self) -> bool {
        matches!(self, Self::Current | Self::NotApplicable)
    }

    /// True when the state forces the row to narrow below its claimed label.
    pub const fn forces_narrowing(self) -> bool {
        !self.is_certified()
    }

    /// True when a non-empty evidence ref must back this state.
    pub const fn requires_evidence_ref(self) -> bool {
        !matches!(self, Self::Missing | Self::NotApplicable)
    }

    /// The qualification floor this state imposes on the row, if any.
    pub const fn qualification_floor(self) -> Option<ContinuityClaimQualificationClass> {
        match self {
            Self::Current | Self::NotApplicable => None,
            Self::Stale | Self::Partial => Some(ContinuityClaimQualificationClass::Beta),
            Self::Missing => Some(ContinuityClaimQualificationClass::Preview),
            Self::ProfileMismatched => Some(ContinuityClaimQualificationClass::Withdrawn),
        }
    }
}

/// The certification verdict a row earns once every dimension is folded in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowCertificationVerdict {
    /// The claim holds: every required dimension is certified.
    Certified,
    /// The claim narrowed automatically below its claimed label.
    Narrowed,
    /// The claim is withdrawn: evidence contradicts the claimed profile.
    Withdrawn,
}

impl RowCertificationVerdict {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Narrowed => "narrowed",
            Self::Withdrawn => "withdrawn",
        }
    }
}

/// Closed reason a continuity certification narrowed below its claimed label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationNarrowReasonClass {
    /// No narrowing is active.
    NotNarrowed,
    /// Locality, tenant, or key-mode disclosure is stale, partial, or missing.
    LocalityTenantKeyUncertified,
    /// The control-plane/data-plane degradation taxonomy is uncertified.
    ControlDataPlaneDegradationUncertified,
    /// Backup/restore/failover drill evidence is stale, partial, or missing.
    BackupRestoreFailoverUncertified,
    /// Restore-identity or partial-loss semantics are undisclosed.
    RestoreIdentityPartialLossUncertified,
    /// Mirror-only or air-gapped offline continuity is uncertified.
    MirrorOfflineContinuityUncertified,
    /// Continuity-proof freshness breached its SLO.
    DrillFreshnessUncertified,
    /// A required continuity dimension has no evidence at all.
    RequiredEvidenceMissing,
    /// Evidence contradicts the claimed managed/self-hosted/sovereign profile.
    ContinuityProfileMismatch,
    /// A backup/restore/failover drill ref is reused across claimed rows.
    SharedReferenceDrillReused,
    /// The row's certification verdict is not reused across the required surfaces.
    SurfaceReuseIncomplete,
}

impl CertificationNarrowReasonClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::LocalityTenantKeyUncertified => "locality_tenant_key_uncertified",
            Self::ControlDataPlaneDegradationUncertified => {
                "control_data_plane_degradation_uncertified"
            }
            Self::BackupRestoreFailoverUncertified => "backup_restore_failover_uncertified",
            Self::RestoreIdentityPartialLossUncertified => {
                "restore_identity_partial_loss_uncertified"
            }
            Self::MirrorOfflineContinuityUncertified => "mirror_offline_continuity_uncertified",
            Self::DrillFreshnessUncertified => "drill_freshness_uncertified",
            Self::RequiredEvidenceMissing => "required_evidence_missing",
            Self::ContinuityProfileMismatch => "continuity_profile_mismatch",
            Self::SharedReferenceDrillReused => "shared_reference_drill_reused",
            Self::SurfaceReuseIncomplete => "surface_reuse_incomplete",
        }
    }
}

/// Closed structural defect kind emitted by the certification audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDefectKind {
    /// A certification-scope row is missing a required continuity dimension.
    RequiredDimensionMissing,
    /// A dimension's evidence state disagrees with whether it carries a ref.
    EvidenceRefIncoherent,
    /// A backup/restore/failover drill ref is reused across claimed rows.
    SharedReferenceDrillEvidence,
    /// A local-core row was marked narrowed or withdrawn, violating the guardrail.
    LocalCoreNarrowed,
    /// A row's certification verdict is not reused across the required surfaces.
    SurfaceReuseIncomplete,
}

impl CertificationDefectKind {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequiredDimensionMissing => "required_dimension_missing",
            Self::EvidenceRefIncoherent => "evidence_ref_incoherent",
            Self::SharedReferenceDrillEvidence => "shared_reference_drill_evidence",
            Self::LocalCoreNarrowed => "local_core_narrowed",
            Self::SurfaceReuseIncomplete => "surface_reuse_incomplete",
        }
    }
}

/// The backing evidence for one continuity dimension on one certified row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationEvidence {
    /// The continuity dimension this evidence covers.
    pub dimension: CertificationDimension,
    /// Stable token for [`Self::dimension`].
    pub dimension_token: String,
    /// The state of the backing evidence.
    pub state: CertificationEvidenceState,
    /// Stable token for [`Self::state`].
    pub state_token: String,
    /// Opaque ref to the upstream continuity packet or evidence (empty only when
    /// the dimension is missing or not applicable).
    pub evidence_ref: String,
    /// Export-safe note describing the evidence or the reason it is uncertified.
    pub note: String,
}

impl CertificationEvidence {
    /// Builds a certification-evidence cell.
    pub fn new(
        dimension: CertificationDimension,
        state: CertificationEvidenceState,
        evidence_ref: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            dimension,
            dimension_token: dimension.as_str().to_owned(),
            state,
            state_token: state.as_str().to_owned(),
            evidence_ref: evidence_ref.into(),
            note: note.into(),
        }
    }
}

/// One claimed continuity row to certify.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedRow {
    /// Opaque row identifier; matches the continuity-claim matrix row id.
    pub row_id: String,
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
    /// True when this row depends on a claimed managed or self-hosted lane.
    pub has_claimed_managed_dependency: bool,
    /// True when this row is mirror-only or air-gapped and owes offline continuity.
    pub requires_offline_continuity: bool,
    /// The lifecycle label the row is put forward as.
    pub claimed_qualification: ContinuityClaimQualificationClass,
    /// Stable token for [`Self::claimed_qualification`].
    pub claimed_qualification_token: String,
    /// Per-dimension continuity evidence backing the row.
    pub evidence: Vec<CertificationEvidence>,
    /// Surfaces that reuse this row's certification verdict.
    pub surface_visibility: ClaimSurfaceVisibility,
}

impl CertifiedRow {
    /// True when this row is held to managed continuity certification.
    ///
    /// Managed, self-hosted, and sovereign profiles are always in scope, as is
    /// any row on the managed continuity lane or carrying a claimed managed
    /// dependency. A pure local-only row with no claimed managed dependency rides
    /// the local-core lane and is never held to managed evidence.
    pub fn in_certification_scope(&self) -> bool {
        self.profile_class != ContinuityProfileClass::LocalOnly
            || self.continuity_lane == ContinuityLaneClass::ManagedLane
            || self.has_claimed_managed_dependency
    }

    /// True when this row rides the local-core continuity lane.
    pub fn is_local_core(&self) -> bool {
        !self.in_certification_scope()
    }

    /// The continuity dimensions this row must prove to stay certified.
    pub fn required_dimensions(&self) -> Vec<CertificationDimension> {
        if !self.in_certification_scope() {
            return Vec::new();
        }
        let mut dims = vec![
            CertificationDimension::LocalityTenantKey,
            CertificationDimension::ControlDataPlaneDegradation,
            CertificationDimension::BackupRestoreFailover,
            CertificationDimension::RestoreIdentityPartialLoss,
            CertificationDimension::DrillFreshnessSlo,
        ];
        if self.requires_offline_continuity {
            dims.push(CertificationDimension::MirrorOfflineContinuity);
        }
        dims
    }

    /// Returns the evidence cell for a dimension, if present.
    pub fn evidence_for(
        &self,
        dimension: CertificationDimension,
    ) -> Option<&CertificationEvidence> {
        self.evidence
            .iter()
            .find(|cell| cell.dimension == dimension)
    }

    /// The opaque backup/restore/failover drill ref this row depends on, if any.
    fn drill_evidence_ref(&self) -> Option<&str> {
        self.evidence
            .iter()
            .find(|cell| cell.dimension.is_drill_dimension() && !cell.evidence_ref.is_empty())
            .map(|cell| cell.evidence_ref.as_str())
    }

    /// The qualification floors every dimension imposes, plus any structural floor.
    fn qualification_floors(&self, shared_drill: bool) -> Vec<ContinuityClaimQualificationClass> {
        let mut floors = Vec::new();
        if !self.in_certification_scope() {
            return floors;
        }
        for cell in &self.evidence {
            if let Some(floor) = cell.state.qualification_floor() {
                floors.push(floor);
            }
        }
        for dimension in self.required_dimensions() {
            if self.evidence_for(dimension).is_none() {
                floors.push(ContinuityClaimQualificationClass::Preview);
            }
        }
        if !self.surface_visibility.all_visible() {
            floors.push(ContinuityClaimQualificationClass::Beta);
        }
        if shared_drill {
            floors.push(ContinuityClaimQualificationClass::Beta);
        }
        floors
    }

    /// The lifecycle label this row effectively publishes after narrowing.
    fn effective_qualification(&self, shared_drill: bool) -> ContinuityClaimQualificationClass {
        let mut effective = self.claimed_qualification;
        for floor in self.qualification_floors(shared_drill) {
            effective = effective.max(floor);
        }
        effective
    }

    /// The certification verdict this row earns.
    fn verdict(&self, shared_drill: bool) -> RowCertificationVerdict {
        if !self.in_certification_scope() {
            return RowCertificationVerdict::Certified;
        }
        let effective = self.effective_qualification(shared_drill);
        if effective == ContinuityClaimQualificationClass::Withdrawn {
            RowCertificationVerdict::Withdrawn
        } else if effective != self.claimed_qualification {
            RowCertificationVerdict::Narrowed
        } else {
            RowCertificationVerdict::Certified
        }
    }

    /// The narrow reasons this row carries.
    fn narrow_reasons(&self, shared_drill: bool) -> Vec<CertificationNarrowReasonClass> {
        let mut reasons = Vec::new();
        if !self.in_certification_scope() {
            return reasons;
        }
        for cell in &self.evidence {
            if cell.state == CertificationEvidenceState::ProfileMismatched {
                reasons.push(CertificationNarrowReasonClass::ContinuityProfileMismatch);
            } else if cell.state.forces_narrowing() {
                reasons.push(cell.dimension.narrow_reason());
            }
        }
        for dimension in self.required_dimensions() {
            if self.evidence_for(dimension).is_none() {
                reasons.push(CertificationNarrowReasonClass::RequiredEvidenceMissing);
            }
        }
        if !self.surface_visibility.all_visible() {
            reasons.push(CertificationNarrowReasonClass::SurfaceReuseIncomplete);
        }
        if shared_drill {
            reasons.push(CertificationNarrowReasonClass::SharedReferenceDrillReused);
        }
        reasons
    }

    /// Dimension tokens whose evidence is stale or missing.
    fn stale_or_missing_dimension_tokens(&self) -> Vec<String> {
        let mut tokens: Vec<String> = self
            .evidence
            .iter()
            .filter(|cell| {
                matches!(
                    cell.state,
                    CertificationEvidenceState::Stale
                        | CertificationEvidenceState::Partial
                        | CertificationEvidenceState::Missing
                        | CertificationEvidenceState::ProfileMismatched
                )
            })
            .map(|cell| cell.dimension.as_str().to_owned())
            .collect();
        for dimension in self.required_dimensions() {
            if self.evidence_for(dimension).is_none() {
                tokens.push(dimension.as_str().to_owned());
            }
        }
        tokens.sort();
        tokens.dedup();
        tokens
    }
}

/// Per-row certification verdict joining a row to its computed outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedRowOutcome {
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
    /// True when the row is held to managed continuity certification.
    pub in_certification_scope: bool,
    /// Stable token for the certification verdict the row earned.
    pub verdict_token: String,
    /// True when the row holds its claim with no narrowing.
    pub certified: bool,
    /// True when the row narrowed below its claimed label.
    pub narrowed: bool,
    /// Stable token for the label the row is put forward as.
    pub claimed_qualification_token: String,
    /// Stable token for the label the row effectively publishes after narrowing.
    pub effective_qualification_token: String,
    /// Stable narrow-reason tokens active on the row.
    pub narrow_reason_tokens: Vec<String>,
    /// Dimension tokens whose evidence is stale or missing.
    pub stale_or_missing_dimension_tokens: Vec<String>,
}

/// Typed defect emitted by the certification audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityCertificationDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque defect identifier.
    pub defect_id: String,
    /// Typed defect kind.
    pub defect_kind: CertificationDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Opaque source row id or report concern that triggered the defect.
    pub source: String,
    /// Export-safe explanation of the defect.
    pub note: String,
}

impl ContinuityCertificationDefect {
    fn new(
        defect_kind: CertificationDefectKind,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: CONTINUITY_CERTIFICATION_DEFECT_RECORD_KIND.to_owned(),
            schema_version: CONTINUITY_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: CONTINUITY_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "continuity:defect:certification:{}:{}",
                defect_kind.as_str(),
                source
            ),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            source,
            note: note.into(),
        }
    }
}

/// Opaque refs to the upstream continuity lanes this report certifies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSourceRefs {
    /// Ref to the locality, tenant/key-mode, and continuity-drill matrix.
    pub claim_matrix_ref: String,
    /// Ref to the control-plane/data-plane degradation taxonomy.
    pub outage_taxonomy_ref: String,
    /// Ref to the backup/restore/failover drill packets.
    pub backup_restore_failover_ref: String,
    /// Ref to the restore-from-backup reviews.
    pub restore_review_ref: String,
    /// Ref to the mirror-only and air-gapped continuity packets.
    pub mirror_airgap_ref: String,
    /// Ref to the continuity-proof freshness SLO dashboard.
    pub freshness_slo_ref: String,
}

/// Full auditable input for the continuity certification report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityCertificationInput {
    /// Reviewable label for the report.
    pub report_label: String,
    /// UTC date the certification was computed against.
    pub as_of: String,
    /// Opaque refs to the upstream continuity lanes.
    pub source_refs: CertificationSourceRefs,
    /// Claimed continuity rows to certify.
    pub rows: Vec<CertifiedRow>,
}

/// Aggregate summary for a continuity certification report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityCertificationSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Overall certification decision token (`certified`, `narrowed`, or `withdrawn`).
    pub overall_decision_token: String,
    /// Number of claimed rows.
    pub row_count: usize,
    /// Number of rows held to managed continuity certification.
    pub certification_scope_row_count: usize,
    /// Number of rows on the local-core continuity lane.
    pub local_core_row_count: usize,
    /// Number of rows that hold their claim with no narrowing.
    pub certified_row_count: usize,
    /// Number of rows that narrowed below their claimed label.
    pub narrowed_row_count: usize,
    /// Number of rows whose claim was withdrawn.
    pub withdrawn_row_count: usize,
    /// Number of certification-scope rows with at least one stale or missing dimension.
    pub stale_or_missing_evidence_row_count: usize,
    /// Number of rows whose backup/restore/failover drill is stale, partial, or missing.
    pub backup_restore_failover_uncertified_row_count: usize,
    /// Number of rows whose continuity-proof freshness breached its SLO.
    pub drill_freshness_uncertified_row_count: usize,
    /// Number of defects recorded for the report.
    pub defect_count: usize,
}

/// Canonical certified-row registry packet for the continuity certification lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityCertificationReport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable report identifier.
    pub report_id: String,
    /// Reviewable report label.
    pub report_label: String,
    /// UTC timestamp when the report was generated.
    pub generated_at: String,
    /// UTC date the certification was computed against.
    pub as_of: String,
    /// Opaque refs to the upstream continuity lanes.
    pub source_refs: CertificationSourceRefs,
    /// Aggregate summary derived from the embedded input and defects.
    pub summary: ContinuityCertificationSummary,
    /// Typed defects for the report.
    pub defects: Vec<ContinuityCertificationDefect>,
    /// Per-row certification verdicts.
    pub row_outcomes: Vec<CertifiedRowOutcome>,
    /// The audited input embedded as evidence.
    pub input: ContinuityCertificationInput,
}

impl ContinuityCertificationReport {
    /// Builds a continuity certification report from the supplied input.
    pub fn new(
        report_id: impl Into<String>,
        report_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: ContinuityCertificationInput,
    ) -> Self {
        let shared_drill_refs = shared_drill_refs(&input);
        let row_outcomes = build_row_outcomes(&input, &shared_drill_refs);
        let defects = audit_certification_input(&input, &shared_drill_refs, &row_outcomes);
        let summary = build_summary(&input, &row_outcomes, &defects);
        Self {
            record_kind: CONTINUITY_CERTIFICATION_REPORT_RECORD_KIND.to_owned(),
            schema_version: CONTINUITY_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: CONTINUITY_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
            report_id: report_id.into(),
            report_label: report_label.into(),
            generated_at: generated_at.into(),
            as_of: input.as_of.clone(),
            source_refs: input.source_refs.clone(),
            summary,
            defects,
            row_outcomes,
            input,
        }
    }

    /// True when no defect was recorded and every claimed row is certified.
    pub fn is_fully_certified(&self) -> bool {
        self.defects.is_empty()
            && self.summary.narrowed_row_count == 0
            && self.summary.withdrawn_row_count == 0
    }

    /// True when no structural defect was recorded for the report.
    pub fn is_structurally_clean(&self) -> bool {
        self.defects.is_empty()
    }

    /// Returns the computed outcome for a row id, if present.
    pub fn row_outcome(&self, row_id: &str) -> Option<&CertifiedRowOutcome> {
        self.row_outcomes
            .iter()
            .find(|outcome| outcome.row_id == row_id)
    }
}

/// Support-export wrapper for the continuity certification report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityCertificationSupportExport {
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
    /// The certification report embedded as evidence.
    pub report: ContinuityCertificationReport,
    /// Narrow-reason tokens present across the embedded report.
    pub narrow_reasons_present: Vec<String>,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl ContinuityCertificationSupportExport {
    /// Wraps a certification report inside a support-export envelope.
    pub fn from_report(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        report: ContinuityCertificationReport,
    ) -> Self {
        let mut reasons: BTreeSet<String> = BTreeSet::new();
        for outcome in &report.row_outcomes {
            for token in &outcome.narrow_reason_tokens {
                reasons.insert(token.clone());
            }
        }
        let mut counts = BTreeMap::new();
        for defect in &report.defects {
            *counts.entry(defect.defect_kind_token.clone()).or_insert(0) += 1;
        }
        Self {
            record_kind: CONTINUITY_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: CONTINUITY_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: CONTINUITY_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            report,
            narrow_reasons_present: reasons.into_iter().collect(),
            defect_counts_by_kind: counts,
            raw_private_material_excluded: true,
        }
    }
}

/// Re-runs the certification audit over the report's embedded input.
pub fn audit_continuity_certification_report(
    report: &ContinuityCertificationReport,
) -> Vec<ContinuityCertificationDefect> {
    let shared_drill_refs = shared_drill_refs(&report.input);
    let row_outcomes = build_row_outcomes(&report.input, &shared_drill_refs);
    audit_certification_input(&report.input, &shared_drill_refs, &row_outcomes)
}

/// Validates a certification report and returns `Ok(())` when the audit is clean.
pub fn validate_continuity_certification_report(
    report: &ContinuityCertificationReport,
) -> Result<(), Vec<ContinuityCertificationDefect>> {
    if report.defects.is_empty() {
        Ok(())
    } else {
        Err(report.defects.clone())
    }
}

/// Returns the seeded fully-certified continuity certification report.
pub fn seeded_continuity_certification_report() -> ContinuityCertificationReport {
    ContinuityCertificationReport::new(
        "continuity:certification:seeded",
        "Certified managed, self-hosted, and sovereign continuity rows",
        "2026-06-19T00:00:00Z",
        seeded_continuity_certification_input(),
    )
}

/// Returns the seeded input used by the canonical certification report.
pub fn seeded_continuity_certification_input() -> ContinuityCertificationInput {
    ContinuityCertificationInput {
        report_label: "Claimed managed, self-hosted, and sovereign continuity certification"
            .to_owned(),
        as_of: "2026-06-19".to_owned(),
        source_refs: CertificationSourceRefs {
            claim_matrix_ref: "artifacts/m5/continuity/claim_rows_and_drill_schedule.md".to_owned(),
            outage_taxonomy_ref:
                "artifacts/m5/continuity/control_plane_vs_data_plane_degradation.md".to_owned(),
            backup_restore_failover_ref:
                "artifacts/m5/continuity/drill_packets/drill_packet_registry.json".to_owned(),
            restore_review_ref:
                "artifacts/m5/continuity/restore_reviews/restore_review_registry.json".to_owned(),
            mirror_airgap_ref:
                "artifacts/m5/continuity/mirror_airgap/offline_continuity_registry.json".to_owned(),
            freshness_slo_ref: "artifacts/m5/continuity/freshness_slo_dashboard.json".to_owned(),
        },
        rows: seeded_rows(),
    }
}

/// Collects the backup/restore/failover drill refs reused by more than one
/// certification-scope row — a single reference drill may not stand for many rows.
fn shared_drill_refs(input: &ContinuityCertificationInput) -> BTreeSet<String> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for row in &input.rows {
        if !row.in_certification_scope() {
            continue;
        }
        if let Some(drill_ref) = row.drill_evidence_ref() {
            *counts.entry(drill_ref.to_owned()).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(drill_ref, _)| drill_ref)
        .collect()
}

fn row_has_shared_drill(row: &CertifiedRow, shared_drill_refs: &BTreeSet<String>) -> bool {
    row.in_certification_scope()
        && row
            .drill_evidence_ref()
            .is_some_and(|drill_ref| shared_drill_refs.contains(drill_ref))
}

fn build_row_outcomes(
    input: &ContinuityCertificationInput,
    shared_drill_refs: &BTreeSet<String>,
) -> Vec<CertifiedRowOutcome> {
    input
        .rows
        .iter()
        .map(|row| {
            let shared_drill = row_has_shared_drill(row, shared_drill_refs);
            let verdict = row.verdict(shared_drill);
            let effective = row.effective_qualification(shared_drill);
            let mut reason_tokens: Vec<String> = row
                .narrow_reasons(shared_drill)
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect();
            reason_tokens.sort();
            reason_tokens.dedup();
            CertifiedRowOutcome {
                record_kind: CERTIFIED_ROW_OUTCOME_RECORD_KIND.to_owned(),
                schema_version: CONTINUITY_CERTIFICATION_SCHEMA_VERSION,
                shared_contract_ref: CONTINUITY_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
                row_id: row.row_id.clone(),
                profile_class_token: row.profile_class.as_str().to_owned(),
                in_certification_scope: row.in_certification_scope(),
                verdict_token: verdict.as_str().to_owned(),
                certified: verdict == RowCertificationVerdict::Certified,
                narrowed: verdict != RowCertificationVerdict::Certified,
                claimed_qualification_token: row.claimed_qualification.as_str().to_owned(),
                effective_qualification_token: effective.as_str().to_owned(),
                narrow_reason_tokens: reason_tokens,
                stale_or_missing_dimension_tokens: row.stale_or_missing_dimension_tokens(),
            }
        })
        .collect()
}

fn audit_certification_input(
    input: &ContinuityCertificationInput,
    shared_drill_refs: &BTreeSet<String>,
    row_outcomes: &[CertifiedRowOutcome],
) -> Vec<ContinuityCertificationDefect> {
    let mut defects = Vec::new();

    for row in &input.rows {
        // Every dimension cell must keep its evidence ref coherent with its state.
        for cell in &row.evidence {
            let has_ref = !cell.evidence_ref.is_empty();
            if cell.state.requires_evidence_ref() && !has_ref {
                defects.push(ContinuityCertificationDefect::new(
                    CertificationDefectKind::EvidenceRefIncoherent,
                    format!("{}:{}", row.row_id, cell.dimension.as_str()),
                    "a current, stale, partial, or profile-mismatched dimension must carry an evidence ref",
                ));
            }
            if !cell.state.requires_evidence_ref() && has_ref {
                defects.push(ContinuityCertificationDefect::new(
                    CertificationDefectKind::EvidenceRefIncoherent,
                    format!("{}:{}", row.row_id, cell.dimension.as_str()),
                    "a missing or not-applicable dimension may not carry an evidence ref",
                ));
            }
        }

        // Certification-scope rows must declare every required dimension.
        for dimension in row.required_dimensions() {
            if row.evidence_for(dimension).is_none() {
                defects.push(ContinuityCertificationDefect::new(
                    CertificationDefectKind::RequiredDimensionMissing,
                    format!("{}:{}", row.row_id, dimension.as_str()),
                    "a certification-scope row must declare evidence for every required continuity dimension",
                ));
            }
        }

        // Surface reuse: scope rows owe every surface; local-core owes the
        // in-product and public-truth surfaces.
        let surface_complete = if row.in_certification_scope() {
            row.surface_visibility.all_visible()
        } else {
            row.surface_visibility.local_core_visible()
        };
        if !surface_complete {
            defects.push(ContinuityCertificationDefect::new(
                CertificationDefectKind::SurfaceReuseIncomplete,
                row.row_id.clone(),
                "every row's certification verdict must be reused across its required surfaces",
            ));
        }

        if row_has_shared_drill(row, shared_drill_refs) {
            defects.push(ContinuityCertificationDefect::new(
                CertificationDefectKind::SharedReferenceDrillEvidence,
                row.row_id.clone(),
                "a single reference-environment backup/restore/failover drill may not stand in for more than one claimed row",
            ));
        }
    }

    // Guardrail: a local-core row may never be reported as narrowed or withdrawn.
    for outcome in row_outcomes {
        if !outcome.in_certification_scope && outcome.narrowed {
            defects.push(ContinuityCertificationDefect::new(
                CertificationDefectKind::LocalCoreNarrowed,
                outcome.row_id.clone(),
                "a local-core continuity row may not narrow or withdraw when a managed row goes stale",
            ));
        }
    }

    defects
}

fn build_summary(
    input: &ContinuityCertificationInput,
    row_outcomes: &[CertifiedRowOutcome],
    defects: &[ContinuityCertificationDefect],
) -> ContinuityCertificationSummary {
    let withdrawn = row_outcomes
        .iter()
        .filter(|o| o.verdict_token == RowCertificationVerdict::Withdrawn.as_str())
        .count();
    let narrowed = row_outcomes.iter().filter(|o| o.narrowed).count();
    let overall = if withdrawn > 0 {
        RowCertificationVerdict::Withdrawn
    } else if narrowed > 0 {
        RowCertificationVerdict::Narrowed
    } else {
        RowCertificationVerdict::Certified
    };

    let dimension_uncertified = |dimension: CertificationDimension| {
        input
            .rows
            .iter()
            .filter(|row| row.in_certification_scope())
            .filter(|row| match row.evidence_for(dimension) {
                Some(cell) => cell.state.forces_narrowing(),
                None => row.required_dimensions().contains(&dimension),
            })
            .count()
    };

    ContinuityCertificationSummary {
        record_kind: CONTINUITY_CERTIFICATION_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: CONTINUITY_CERTIFICATION_SCHEMA_VERSION,
        shared_contract_ref: CONTINUITY_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
        overall_decision_token: overall.as_str().to_owned(),
        row_count: input.rows.len(),
        certification_scope_row_count: input
            .rows
            .iter()
            .filter(|row| row.in_certification_scope())
            .count(),
        local_core_row_count: input.rows.iter().filter(|row| row.is_local_core()).count(),
        certified_row_count: row_outcomes.iter().filter(|o| o.certified).count(),
        narrowed_row_count: narrowed,
        withdrawn_row_count: withdrawn,
        stale_or_missing_evidence_row_count: row_outcomes
            .iter()
            .filter(|o| o.in_certification_scope && !o.stale_or_missing_dimension_tokens.is_empty())
            .count(),
        backup_restore_failover_uncertified_row_count: dimension_uncertified(
            CertificationDimension::BackupRestoreFailover,
        ),
        drill_freshness_uncertified_row_count: dimension_uncertified(
            CertificationDimension::DrillFreshnessSlo,
        ),
        defect_count: defects.len(),
    }
}

fn seeded_rows() -> Vec<CertifiedRow> {
    vec![
        certified_row(
            "continuity-row:managed-cloud-sync",
            "Managed cloud workspace sync and backup",
            ContinuityProfileClass::Managed,
            ContinuityLaneClass::ManagedLane,
            true,
            false,
            ContinuityClaimQualificationClass::Stable,
            vec![
                evidence(
                    CertificationDimension::LocalityTenantKey,
                    CertificationEvidenceState::Current,
                    "continuity-row:managed-cloud-sync:locality",
                    "Single us-west managed region, shared multi-tenant, vendor-managed keys disclosed.",
                ),
                evidence(
                    CertificationDimension::ControlDataPlaneDegradation,
                    CertificationEvidenceState::Current,
                    "outage-taxonomy:managed-cloud:control-plane",
                    "Control-plane impairment falls back to local-core editing.",
                ),
                evidence(
                    CertificationDimension::BackupRestoreFailover,
                    CertificationEvidenceState::Current,
                    "drill:managed-cloud:backup:2026-06-01",
                    "Per-release backup drill, same-identity restore, bounded recent-window loss.",
                ),
                evidence(
                    CertificationDimension::RestoreIdentityPartialLoss,
                    CertificationEvidenceState::Current,
                    "restore-review:managed-cloud:backup",
                    "Same-identity restore; a bounded window of unsynced edits replays locally.",
                ),
                evidence(
                    CertificationDimension::DrillFreshnessSlo,
                    CertificationEvidenceState::Current,
                    "freshness:managed-cloud:backup:current",
                    "Proof packet current within its 90-day freshness SLO.",
                ),
            ],
            ClaimSurfaceVisibility::all_required(),
        ),
        certified_row(
            "continuity-row:managed-relay-failover",
            "Managed relay and collaboration failover",
            ContinuityProfileClass::Managed,
            ContinuityLaneClass::ManagedLane,
            true,
            false,
            ContinuityClaimQualificationClass::Stable,
            vec![
                evidence(
                    CertificationDimension::LocalityTenantKey,
                    CertificationEvidenceState::Current,
                    "continuity-row:managed-relay-failover:locality",
                    "Multi-region managed regions, dedicated tenant, vendor-managed keys disclosed.",
                ),
                evidence(
                    CertificationDimension::ControlDataPlaneDegradation,
                    CertificationEvidenceState::Current,
                    "outage-taxonomy:managed-relay:data-plane",
                    "Data-plane relay impairment falls back to local editing with queued replay.",
                ),
                evidence(
                    CertificationDimension::BackupRestoreFailover,
                    CertificationEvidenceState::Current,
                    "drill:managed-relay:failover:2026-05-20",
                    "Quarterly failover drill, mirror-backed, same-identity restore.",
                ),
                evidence(
                    CertificationDimension::RestoreIdentityPartialLoss,
                    CertificationEvidenceState::Current,
                    "restore-review:managed-relay:failover",
                    "Same-identity failover; in-flight relay actions may replay, durable state intact.",
                ),
                evidence(
                    CertificationDimension::DrillFreshnessSlo,
                    CertificationEvidenceState::Current,
                    "freshness:managed-relay:failover:due-soon",
                    "Proof packet within its freshness SLO; a rerun is due soon.",
                ),
            ],
            ClaimSurfaceVisibility::all_required(),
        ),
        certified_row(
            "continuity-row:self-hosted-restore",
            "Customer self-hosted restore and rebuild",
            ContinuityProfileClass::SelfHosted,
            ContinuityLaneClass::ManagedLane,
            true,
            false,
            ContinuityClaimQualificationClass::Stable,
            vec![
                evidence(
                    CertificationDimension::LocalityTenantKey,
                    CertificationEvidenceState::Current,
                    "continuity-row:self-hosted-restore:locality",
                    "Customer-operated eu-central region, customer tenant, customer-managed keys.",
                ),
                evidence(
                    CertificationDimension::ControlDataPlaneDegradation,
                    CertificationEvidenceState::Current,
                    "outage-taxonomy:self-hosted:control-plane",
                    "Control-plane impairment is customer-operated; data plane stays local.",
                ),
                evidence(
                    CertificationDimension::BackupRestoreFailover,
                    CertificationEvidenceState::Current,
                    "drill:self-hosted:restore:2026-05-01",
                    "Semiannual restore drill reconstructable from a verified snapshot.",
                ),
                evidence(
                    CertificationDimension::RestoreIdentityPartialLoss,
                    CertificationEvidenceState::Current,
                    "restore-review:self-hosted:restore",
                    "Reissued-identity restore; operators re-trust the reissued identity once.",
                ),
                evidence(
                    CertificationDimension::DrillFreshnessSlo,
                    CertificationEvidenceState::Current,
                    "freshness:self-hosted:restore:current",
                    "Proof packet current within its 180-day freshness SLO.",
                ),
            ],
            ClaimSurfaceVisibility::all_required(),
        ),
        certified_row(
            "continuity-row:sovereign-airgap-snapshot",
            "Sovereign air-gapped snapshot and replication",
            ContinuityProfileClass::Sovereign,
            ContinuityLaneClass::ManagedLane,
            true,
            true,
            ContinuityClaimQualificationClass::Stable,
            vec![
                evidence(
                    CertificationDimension::LocalityTenantKey,
                    CertificationEvidenceState::Current,
                    "continuity-row:sovereign-airgap-snapshot:locality",
                    "In-country sovereign processing, air-gapped storage, customer-held root.",
                ),
                evidence(
                    CertificationDimension::ControlDataPlaneDegradation,
                    CertificationEvidenceState::Current,
                    "outage-taxonomy:sovereign:both-planes",
                    "Both planes addressed inside the isolated boundary.",
                ),
                evidence(
                    CertificationDimension::BackupRestoreFailover,
                    CertificationEvidenceState::Current,
                    "drill:sovereign:snapshot:2026-05-15",
                    "Annual snapshot/replication drill, offline-snapshot recovery.",
                ),
                evidence(
                    CertificationDimension::RestoreIdentityPartialLoss,
                    CertificationEvidenceState::Current,
                    "restore-review:sovereign:snapshot",
                    "New-install rebind from the last signed snapshot; only cache is lost.",
                ),
                evidence(
                    CertificationDimension::MirrorOfflineContinuity,
                    CertificationEvidenceState::Current,
                    "mirror-airgap:sovereign:offline",
                    "Air-gapped trust roots and offline import/export posture certified.",
                ),
                evidence(
                    CertificationDimension::DrillFreshnessSlo,
                    CertificationEvidenceState::Current,
                    "freshness:sovereign:snapshot:current",
                    "Proof packet current within its 365-day freshness SLO.",
                ),
            ],
            ClaimSurfaceVisibility::all_required(),
        ),
        certified_row(
            "continuity-row:local-desktop-core",
            "Local desktop core continuity",
            ContinuityProfileClass::LocalOnly,
            ContinuityLaneClass::LocalCore,
            false,
            false,
            ContinuityClaimQualificationClass::Stable,
            vec![evidence(
                CertificationDimension::DrillFreshnessSlo,
                CertificationEvidenceState::Current,
                "freshness:local-core:autosave:current",
                "Local autosave and Git keep durable edits; no managed lane is claimed.",
            )],
            ClaimSurfaceVisibility::local_core_required(),
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn certified_row(
    row_id: &str,
    surface_label: &str,
    profile_class: ContinuityProfileClass,
    continuity_lane: ContinuityLaneClass,
    has_claimed_managed_dependency: bool,
    requires_offline_continuity: bool,
    claimed_qualification: ContinuityClaimQualificationClass,
    evidence: Vec<CertificationEvidence>,
    surface_visibility: ClaimSurfaceVisibility,
) -> CertifiedRow {
    CertifiedRow {
        row_id: row_id.to_owned(),
        surface_label: surface_label.to_owned(),
        profile_class,
        profile_class_token: profile_class.as_str().to_owned(),
        continuity_lane,
        continuity_lane_token: continuity_lane.as_str().to_owned(),
        has_claimed_managed_dependency,
        requires_offline_continuity,
        claimed_qualification,
        claimed_qualification_token: claimed_qualification.as_str().to_owned(),
        evidence,
        surface_visibility,
    }
}

fn evidence(
    dimension: CertificationDimension,
    state: CertificationEvidenceState,
    evidence_ref: &str,
    note: &str,
) -> CertificationEvidence {
    CertificationEvidence::new(dimension, state, evidence_ref, note)
}
