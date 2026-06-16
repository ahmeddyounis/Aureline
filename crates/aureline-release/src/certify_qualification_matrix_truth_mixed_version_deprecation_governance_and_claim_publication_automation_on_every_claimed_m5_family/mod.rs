//! Typed per-family M5 certification register.
//!
//! This register is the certification capstone over the M5 stable-facing families.
//! Where the qualification/skew matrix is the machine-readable qualification truth, the
//! claim-publication manifest is the single public claim every surface reads, and the
//! diff/deprecation and skew-inspector packets govern stable-facing change, this
//! register binds all of them together: for every claimed family it joins one
//! [`FamilyCertificationPacket`] to the four governance pillars that the source
//! documents treat as the public contract and decides whether the family may carry a
//! certified Stable claim or is narrowed. For each family it binds:
//!
//! - the four [`CertificationPillar`]s — the qualification-matrix row
//!   ([`CertificationPillarKind::QualificationMatrix`]), the mixed-version skew window
//!   ([`CertificationPillarKind::SkewWindow`]), the diff/deprecation packet
//!   ([`CertificationPillarKind::DiffDeprecation`]), and the public claim entry
//!   ([`CertificationPillarKind::ClaimPublication`]) — each carrying its own
//!   [`M5ClaimReportState`], so the per-pillar truth never collapses into one global
//!   flag and a stale qualification dimension narrows the family while the skew and
//!   claim pillars stay current,
//! - the row-level governance state every consuming surface reads: the qualification
//!   [`RowState`], the [`SkewWindowClass`], the [`DeprecationStatus`], the
//!   [`CertificationState`], the freshness state, the validity window, and the active
//!   [`CertificationReason`]s,
//! - and the certified claim it puts forward — never greener than the public claim's
//!   published label ([`FamilyCertificationPacket::source_published_label`]) or support
//!   class ([`FamilyCertificationPacket::source_support_class`]), both hard ceilings.
//!
//! The no-overclaim guard is the spine of the register: a certified packet may never
//! claim a greener [`FamilyCertificationPacket::certified_label`] than the public claim,
//! never advertise a broader [`FamilyCertificationPacket::certified_support_class`], and
//! a *certified* family reuses the public claim's label and support class verbatim
//! (claim-manifest parity) while riding all four pillars current inside an open validity
//! window with owner sign-off.
//!
//! A family that merely inherits an upstream narrowing
//! ([`CertificationReason::RowDowngraded`]) is gated by the qualification matrix and
//! claim manifest, not this register. A *certification-layer* failure (a stale or missing
//! pillar; a stale or missing certification proof packet; a broken claim parity; a
//! missing diff report; an expired validity window; a lapsed waiver; or a missing owner
//! sign-off) on a family whose public claim is still at or above the cutline narrows the
//! certified claim and holds promotion through a [`CertificationStopRule`].
//!
//! This register reuses the canonical [`FamilyKind`], [`SupportClass`], [`RowState`],
//! [`SkewWindowClass`], and [`DeprecationStatus`] vocabularies from the
//! qualification/skew matrix, the evidence-state vocabulary ([`M5ClaimReportState`])
//! from the claim-publication manifest, the [`ProofPacket`] and [`FreshnessSloState`]
//! freshness vocabulary from the stable claim manifest, and the [`LaunchCutline`],
//! [`StableClaimLevel`], [`OwnerSignoff`], [`QualificationWaiver`],
//! [`PromotionDecision`], and [`PromotionDecisionRecord`] types from the stable claim
//! matrix rather than minting local synonyms.
//!
//! The register is checked in at [`M5_FAMILY_CERTIFICATION_PATH`] and embedded here, so
//! this typed consumer and the CI gate agree on every row without a cargo build in CI.
//! The model is metadata-only: every field is a typed state or an opaque ref. It carries
//! no raw artifacts, raw logs, signatures, or credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth::M5ClaimReportState;
use crate::freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix::{
    DeprecationStatus, FamilyKind, RowState, SkewWindowClass, SupportClass,
};
use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported register schema version.
pub const M5_FAMILY_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_FAMILY_CERTIFICATION_RECORD_KIND: &str =
    "certify_m5_family_qualification_skew_deprecation_and_claim_publication";

/// Repo-relative path to the checked-in register.
pub const M5_FAMILY_CERTIFICATION_PATH: &str =
    "artifacts/release/m5/certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family.json";

/// Embedded checked-in register JSON.
pub const M5_FAMILY_CERTIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family.json"
));

/// The breadth rank of a support class; a broader class ranks higher. A certified
/// family may never advertise a support class broader than the public claim it reuses.
const fn support_breadth(class: SupportClass) -> u8 {
    match class {
        SupportClass::FullSupport => 4,
        SupportClass::MaintenanceOnly => 3,
        SupportClass::SecurityOnly => 2,
        SupportClass::Limited => 1,
        SupportClass::EndOfLife => 0,
    }
}

/// One of the four governance pillars a certification packet binds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationPillarKind {
    /// The qualification-matrix row with per-dimension states and the freshness window.
    QualificationMatrix,
    /// The mixed-version skew window: negotiated fields, supported range, and behavior.
    SkewWindow,
    /// The public-interface diff/deprecation packet.
    DiffDeprecation,
    /// The claim-publication manifest entry — the single public claim.
    ClaimPublication,
}

impl CertificationPillarKind {
    /// Every pillar kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::QualificationMatrix,
        Self::SkewWindow,
        Self::DiffDeprecation,
        Self::ClaimPublication,
    ];

    /// The pillars every certification packet must bind.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualificationMatrix => "qualification_matrix",
            Self::SkewWindow => "skew_window",
            Self::DiffDeprecation => "diff_deprecation",
            Self::ClaimPublication => "claim_publication",
        }
    }
}

/// Overall certification state a family earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationState {
    /// The family certifies the public claim's label; all four pillars are current.
    Certified,
    /// The family inherited an upstream qualification/claim narrowing.
    NarrowedRowDowngraded,
    /// A pillar or the certification proof packet went stale; the family narrows.
    NarrowedStale,
    /// A retest is pending; the family narrows.
    NarrowedRetestPending,
    /// The family is withheld entirely (expired window/waiver, missing sign-off,
    /// missing claim, missing pillar evidence, or missing diff report).
    Withheld,
}

impl CertificationState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Certified,
        Self::NarrowedRowDowngraded,
        Self::NarrowedStale,
        Self::NarrowedRetestPending,
        Self::Withheld,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedRowDowngraded => "narrowed_row_downgraded",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedRetestPending => "narrowed_retest_pending",
            Self::Withheld => "withheld",
        }
    }

    /// Whether the state lets the family certify the public claim's label.
    pub const fn holds_certification(self) -> bool {
        matches!(self, Self::Certified)
    }
}

/// Closed reason a family narrows below the public claim it reuses or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationReason {
    /// The qualification row or claim publication narrowed below the cutline.
    RowDowngraded,
    /// A qualification dimension's evidence has gone stale.
    QualificationStale,
    /// A dimension or boundary requires a retest before it may re-certify.
    RetestPending,
    /// A peer is outside the supported skew window.
    SkewWindowExceeded,
    /// The family is deprecated with a scheduled removal.
    DeprecationScheduled,
    /// A stable-facing contract changed without a diff/deprecation packet.
    DiffReportMissing,
    /// The certified claim disagrees with the public claim's label or support class.
    ClaimParityBroken,
    /// The certification proof packet or a backing pillar breached its freshness SLO.
    EvidenceStale,
    /// No certification proof packet or backing pillar was captured.
    EvidenceMissing,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
    /// The certification validity window has expired.
    ValidityWindowExpired,
    /// The backing public claim publication is missing.
    ClaimPublicationMissing,
}

impl CertificationReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::RowDowngraded,
        Self::QualificationStale,
        Self::RetestPending,
        Self::SkewWindowExceeded,
        Self::DeprecationScheduled,
        Self::DiffReportMissing,
        Self::ClaimParityBroken,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::OwnerSignoffMissing,
        Self::ValidityWindowExpired,
        Self::ClaimPublicationMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RowDowngraded => "row_downgraded",
            Self::QualificationStale => "qualification_stale",
            Self::RetestPending => "retest_pending",
            Self::SkewWindowExceeded => "skew_window_exceeded",
            Self::DeprecationScheduled => "deprecation_scheduled",
            Self::DiffReportMissing => "diff_report_missing",
            Self::ClaimParityBroken => "claim_parity_broken",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceMissing => "evidence_missing",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::ValidityWindowExpired => "validity_window_expired",
            Self::ClaimPublicationMissing => "claim_publication_missing",
        }
    }

    /// Whether a family whose public claim is at or above the cutline carrying this
    /// reason holds promotion. A reason that merely inherits an upstream narrowing is
    /// gated by the qualification matrix and claim manifest, not this register.
    pub const fn blocks_promotion(self) -> bool {
        !matches!(self, Self::RowDowngraded)
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationStopAction {
    /// Hold the certification publication until the condition clears.
    HoldCertification,
    /// Narrow the family to inherit the public claim.
    NarrowRow,
    /// Withhold the family entirely.
    WithholdRow,
    /// Refresh the certification evidence.
    RefreshEvidence,
    /// Schedule the pending retest.
    ScheduleRetest,
    /// Renew the validity window.
    RenewValidityWindow,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
    /// Publish the missing diff/deprecation packet.
    PublishDiffReport,
    /// Align the certified claim to the public claim parity.
    AlignClaimParity,
}

impl CertificationStopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::HoldCertification,
        Self::NarrowRow,
        Self::WithholdRow,
        Self::RefreshEvidence,
        Self::ScheduleRetest,
        Self::RenewValidityWindow,
        Self::RequestOwnerSignoff,
        Self::PublishDiffReport,
        Self::AlignClaimParity,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldCertification => "hold_certification",
            Self::NarrowRow => "narrow_row",
            Self::WithholdRow => "withhold_row",
            Self::RefreshEvidence => "refresh_evidence",
            Self::ScheduleRetest => "schedule_retest",
            Self::RenewValidityWindow => "renew_validity_window",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::PublishDiffReport => "publish_diff_report",
            Self::AlignClaimParity => "align_claim_parity",
        }
    }
}

/// The validity window the certification is asserted within.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationValidityWindow {
    /// UTC date the certification becomes valid.
    pub starts_at: String,
    /// UTC date the certification expires and must be renewed.
    pub expires_at: String,
    /// Whether the window has expired as of the register's `as_of` date.
    pub expired: bool,
}

/// One governance pillar a certification packet binds, with its freshness state.
///
/// The ref is the reopen handle a shiproom dashboard follows back to the authoritative
/// governance record; the state surfaces stale or missing evidence at pillar level so
/// the per-pillar truth never collapses into one global flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationPillar {
    /// The governance pillar this binding names.
    pub kind: CertificationPillarKind,
    /// Opaque ref to the backing record. Empty only when the state is `missing`.
    pub pillar_ref: String,
    /// The pillar's freshness/integrity state.
    pub state: M5ClaimReportState,
    /// Reviewable one-line statement of the pillar.
    pub summary: String,
}

/// One certification stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched family fires this rule.
    pub trigger_reason: CertificationReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: CertificationStopAction,
    /// Whether firing this rule holds promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One certification packet: the bound governance verdict for one claimed family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyCertificationPacket {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The family this packet governs.
    pub family_kind: FamilyKind,
    /// The family ref this packet speaks about.
    pub family_ref: String,
    /// Reviewable one-line statement of the family.
    pub family_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The qualification-matrix row entry id this packet joins to (a reopen ref).
    pub qualification_row_ref: String,
    /// The claim-publication manifest entry id this packet joins to (a reopen ref).
    pub claim_manifest_entry_ref: String,
    /// The skew-window record ref this packet joins to (a reopen ref).
    pub skew_window_ref: String,
    /// The diff/deprecation packet ref this packet joins to (a reopen ref).
    pub diff_deprecation_packet_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// The public claim's effective published label (the hard ceiling for this family).
    pub source_published_label: StableClaimLevel,
    /// The public claim's support class (the support ceiling for this family).
    pub source_support_class: SupportClass,
    /// The public claim's exact wording, mirrored verbatim from the claim manifest.
    pub source_claim_text: String,
    /// The qualification row state earned upstream.
    pub row_state: RowState,
    /// The declared skew-window class for the family's boundary.
    pub skew_window_class: SkewWindowClass,
    /// The family's deprecation status.
    pub deprecation_status: DeprecationStatus,
    /// The four governance pillars this packet binds. Always covers the required set.
    pub pillars: Vec<CertificationPillar>,
    /// The validity window the certification is asserted within.
    pub validity_window: CertificationValidityWindow,
    /// Overall certification state earned.
    pub certification_state: CertificationState,
    /// The support class the certification advertises. Never broader than the public
    /// class.
    pub certified_support_class: SupportClass,
    /// The lifecycle label the certification effectively publishes. Never greener than
    /// the public claim's published label.
    pub certified_label: StableClaimLevel,
    /// The certification caveats that travel with the family. Non-empty when support is
    /// limited.
    #[serde(default)]
    pub certification_caveats: Vec<String>,
    /// The certification proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional certification, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the family below the public claim label.
    #[serde(default)]
    pub active_certification_reasons: Vec<CertificationReason>,
    /// Reviewable reason the family carries this posture.
    pub rationale: String,
}

impl FamilyCertificationPacket {
    /// True when the certified label is at or above the cutline.
    pub fn certifies_stable(&self) -> bool {
        self.certified_label.is_at_or_above_cutline()
    }

    /// True when the public claim it reuses is itself at or above the cutline.
    pub fn source_holds_stable(&self) -> bool {
        self.source_published_label.is_at_or_above_cutline()
    }

    /// True when the certification state lets the family carry the public claim's label.
    pub fn holds_certification(&self) -> bool {
        self.certification_state.holds_certification()
    }

    /// True when a narrowing reason is active on the family.
    pub fn has_active_reason(&self, reason: CertificationReason) -> bool {
        self.active_certification_reasons.contains(&reason)
    }

    /// True when the certification advertises a label or support class wider than the
    /// public claim it reuses.
    pub fn over_claims_source(&self) -> bool {
        self.certified_label.rank() > self.source_published_label.rank()
            || support_breadth(self.certified_support_class)
                > support_breadth(self.source_support_class)
    }

    /// The freshness state the family discloses (the certification proof packet state).
    pub fn freshness_state(&self) -> FreshnessSloState {
        self.proof_packet.slo_state
    }

    /// Returns the bound pillar of `kind`, when present.
    pub fn pillar(&self, kind: CertificationPillarKind) -> Option<&CertificationPillar> {
        self.pillars.iter().find(|p| p.kind == kind)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationSummary {
    /// Total number of rows.
    pub total_rows: usize,
    /// Distinct families covered.
    pub total_families: usize,
    /// Families certifying at or above the cutline.
    pub rows_certified: usize,
    /// Families narrowed below the cutline.
    pub rows_narrowed: usize,
    /// Total release-blocking families.
    pub release_blocking_total: usize,
    /// Release-blocking families certifying at or above the cutline.
    pub release_blocking_certified: usize,
    /// Release-blocking families narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook rows.
    pub notebook_rows: usize,
    /// AI/provider rows.
    pub ai_provider_rows: usize,
    /// Remote/helper rows.
    pub remote_helper_rows: usize,
    /// Companion rows.
    pub companion_rows: usize,
    /// Ecosystem rows.
    pub ecosystem_rows: usize,
    /// Managed-service rows.
    pub managed_service_rows: usize,
    /// Toolchain/runtime rows.
    pub toolchain_runtime_rows: usize,
    /// Rows in the `certified` state.
    pub state_certified: usize,
    /// Rows in the `narrowed_row_downgraded` state.
    pub state_narrowed_row_downgraded: usize,
    /// Rows in the `narrowed_stale` state.
    pub state_narrowed_stale: usize,
    /// Rows in the `narrowed_retest_pending` state.
    pub state_narrowed_retest_pending: usize,
    /// Rows in the `withheld` state.
    pub state_withheld: usize,
    /// Rows carrying at least one certification caveat.
    pub rows_with_caveats: usize,
    /// Total certification caveats across all rows.
    pub total_caveats: usize,
    /// Total governance pillars across all rows.
    pub total_pillars: usize,
    /// Pillars that are current.
    pub pillars_current: usize,
    /// Pillars that are stale.
    pub pillars_stale: usize,
    /// Pillars that are missing.
    pub pillars_missing: usize,
    /// Pillars that are dropped.
    pub pillars_dropped: usize,
    /// Pillars that are unsigned.
    pub pillars_unsigned: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all rows.
    pub total_active_certification_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyCertificationExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The family this row governs.
    pub family_kind: FamilyKind,
    /// The family ref this row speaks about.
    pub family_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The qualification-matrix row entry id this packet joins to.
    pub qualification_row_ref: String,
    /// The claim-publication manifest entry this packet joins to.
    pub claim_manifest_entry_ref: String,
    /// The skew-window record ref this packet joins to.
    pub skew_window_ref: String,
    /// The diff/deprecation packet ref this packet joins to.
    pub diff_deprecation_packet_ref: String,
    /// The canonical claim label.
    pub claim_label: StableClaimLevel,
    /// The public claim's published label (the ceiling).
    pub source_published_label: StableClaimLevel,
    /// The family's effective certified label.
    pub certified_label: StableClaimLevel,
    /// Whether the family certifies at or above the cutline.
    pub certifies_stable: bool,
    /// Overall certification state earned.
    pub certification_state: CertificationState,
    /// The qualification row state earned upstream.
    pub row_state: RowState,
    /// The declared skew-window class.
    pub skew_window_class: SkewWindowClass,
    /// The deprecation status.
    pub deprecation_status: DeprecationStatus,
    /// The support class the certification advertises.
    pub certified_support_class: SupportClass,
    /// The disclosed freshness state.
    pub freshness_state: FreshnessSloState,
    /// The certification caveats that travel with the family.
    pub certification_caveats: Vec<String>,
    /// The number of bound governance pillars.
    pub pillar_count: usize,
    /// Active narrowing reasons.
    pub active_certification_reasons: Vec<CertificationReason>,
}

/// Export projection for support, shiproom, docs, and partner-review surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyCertificationExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<FamilyCertificationExportRow>,
}

/// The typed per-family M5 certification register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5FamilyCertificationRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub register_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the qualification/skew matrix whose rows this register certifies.
    pub qualification_matrix_ref: String,
    /// Ref to the claim-publication register whose public claims this register reuses.
    pub claim_manifest_ref: String,
    /// Ref to the public-interface diff/deprecation report register.
    pub diff_report_ref: String,
    /// Ref to the mixed-version skew-inspector register.
    pub skew_inspector_ref: String,
    /// Ref to the canonical M5 evidence index this register is recorded under.
    pub evidence_index_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed family-kind vocabulary.
    pub family_kinds: Vec<FamilyKind>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed qualification row-state vocabulary.
    pub row_states: Vec<RowState>,
    /// Closed skew-window-class vocabulary.
    pub skew_window_classes: Vec<SkewWindowClass>,
    /// Closed deprecation-status vocabulary.
    pub deprecation_statuses: Vec<DeprecationStatus>,
    /// Closed evidence-state vocabulary.
    pub evidence_states: Vec<M5ClaimReportState>,
    /// Closed pillar-kind vocabulary.
    pub pillar_kinds: Vec<CertificationPillarKind>,
    /// The required governance pillars every family must bind.
    pub required_pillars: Vec<CertificationPillarKind>,
    /// Closed certification-state vocabulary.
    pub certification_states: Vec<CertificationState>,
    /// Closed freshness-state vocabulary.
    pub freshness_states: Vec<FreshnessSloState>,
    /// Closed narrowing-reason vocabulary.
    pub certification_reasons: Vec<CertificationReason>,
    /// Closed stop-action vocabulary.
    pub stop_actions: Vec<CertificationStopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking family refs this register must cover.
    pub release_blocking_family_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<CertificationStopRule>,
    /// Certification packets.
    pub rows: Vec<FamilyCertificationPacket>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: CertificationSummary,
}

impl M5FamilyCertificationRegister {
    /// Returns the packet registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&FamilyCertificationPacket> {
        self.rows.iter().find(|r| r.entry_id == entry_id)
    }

    /// Returns the families certifying at or above the cutline.
    pub fn rows_certified(&self) -> Vec<&FamilyCertificationPacket> {
        self.rows.iter().filter(|r| r.certifies_stable()).collect()
    }

    /// Returns the families narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&FamilyCertificationPacket> {
        self.rows.iter().filter(|r| !r.certifies_stable()).collect()
    }

    /// Returns the release-blocking families.
    pub fn release_blocking_rows(&self) -> Vec<&FamilyCertificationPacket> {
        self.rows.iter().filter(|r| r.release_blocking).collect()
    }

    /// Returns the rows for one family kind.
    pub fn rows_for_kind(&self, kind: FamilyKind) -> Vec<&FamilyCertificationPacket> {
        self.rows.iter().filter(|r| r.family_kind == kind).collect()
    }

    /// Distinct families (by family ref) the register covers.
    pub fn families(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for r in &self.rows {
            set.insert(r.family_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched family carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &CertificationStopRule) -> bool {
        self.rows.iter().any(|r| {
            rule.applies_to_labels.contains(&r.source_published_label)
                && r.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the rows and stop rules.
    pub fn computed_promotion_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Stop-rule ids that block promotion and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Family ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only families whose public claim is at or above the cutline count: a family whose
    /// public claim is already narrowed merely inherits the ceiling, and the
    /// qualification matrix and claim manifest already hold promotion for it.
    pub fn computed_blocking_claim_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<CertificationReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for r in &self.rows {
            if r.source_holds_stable()
                && r.active_certification_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(r.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Counts the pillars across all rows in `state`.
    fn pillars_in(&self, state: M5ClaimReportState) -> usize {
        self.rows
            .iter()
            .flat_map(|r| r.pillars.iter())
            .filter(|p| p.state == state)
            .count()
    }

    /// Recomputes the summary block from the rows and stop rules.
    pub fn computed_summary(&self) -> CertificationSummary {
        let kind = |kind: FamilyKind| self.rows_for_kind(kind).len();
        let state = |state: CertificationState| {
            self.rows
                .iter()
                .filter(|r| r.certification_state == state)
                .count()
        };
        let packets = |s: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|r| r.proof_packet.slo_state == s)
                .count()
        };
        let release_blocking: Vec<&FamilyCertificationPacket> = self.release_blocking_rows();
        CertificationSummary {
            total_rows: self.rows.len(),
            total_families: self.families().len(),
            rows_certified: self.rows_certified().len(),
            rows_narrowed: self.rows_narrowed().len(),
            release_blocking_total: release_blocking.len(),
            release_blocking_certified: release_blocking
                .iter()
                .filter(|r| r.certifies_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|r| !r.certifies_stable())
                .count(),
            notebook_rows: kind(FamilyKind::Notebook),
            ai_provider_rows: kind(FamilyKind::AiProvider),
            remote_helper_rows: kind(FamilyKind::RemoteHelper),
            companion_rows: kind(FamilyKind::Companion),
            ecosystem_rows: kind(FamilyKind::Ecosystem),
            managed_service_rows: kind(FamilyKind::ManagedService),
            toolchain_runtime_rows: kind(FamilyKind::ToolchainRuntime),
            state_certified: state(CertificationState::Certified),
            state_narrowed_row_downgraded: state(CertificationState::NarrowedRowDowngraded),
            state_narrowed_stale: state(CertificationState::NarrowedStale),
            state_narrowed_retest_pending: state(CertificationState::NarrowedRetestPending),
            state_withheld: state(CertificationState::Withheld),
            rows_with_caveats: self
                .rows
                .iter()
                .filter(|r| !r.certification_caveats.is_empty())
                .count(),
            total_caveats: self
                .rows
                .iter()
                .map(|r| r.certification_caveats.len())
                .sum(),
            total_pillars: self.rows.iter().map(|r| r.pillars.len()).sum(),
            pillars_current: self.pillars_in(M5ClaimReportState::Current),
            pillars_stale: self.pillars_in(M5ClaimReportState::Stale),
            pillars_missing: self.pillars_in(M5ClaimReportState::Missing),
            pillars_dropped: self.pillars_in(M5ClaimReportState::Dropped),
            pillars_unsigned: self.pillars_in(M5ClaimReportState::Unsigned),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_certification_reasons: self
                .rows
                .iter()
                .map(|r| r.active_certification_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export-safe projection that downstream surfaces render instead of
    /// cloning status text. The reopen refs, the certified label, the freshness state,
    /// the row-level reasons, and the caveats travel with every row, so support,
    /// shiproom, docs, and partner review reconstruct from one source.
    pub fn support_export_projection(&self) -> FamilyCertificationExportProjection {
        FamilyCertificationExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|r| FamilyCertificationExportRow {
                    entry_id: r.entry_id.clone(),
                    family_kind: r.family_kind,
                    family_ref: r.family_ref.clone(),
                    release_blocking: r.release_blocking,
                    qualification_row_ref: r.qualification_row_ref.clone(),
                    claim_manifest_entry_ref: r.claim_manifest_entry_ref.clone(),
                    skew_window_ref: r.skew_window_ref.clone(),
                    diff_deprecation_packet_ref: r.diff_deprecation_packet_ref.clone(),
                    claim_label: r.claim_label,
                    source_published_label: r.source_published_label,
                    certified_label: r.certified_label,
                    certifies_stable: r.certifies_stable(),
                    certification_state: r.certification_state,
                    row_state: r.row_state,
                    skew_window_class: r.skew_window_class,
                    deprecation_status: r.deprecation_status,
                    certified_support_class: r.certified_support_class,
                    freshness_state: r.freshness_state(),
                    certification_caveats: r.certification_caveats.clone(),
                    pillar_count: r.pillars.len(),
                    active_certification_reasons: r.active_certification_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<CertificationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for r in &self.rows {
            if !seen.insert(r.entry_id.clone()) {
                violations.push(CertificationViolation::DuplicateEntryId {
                    entry_id: r.entry_id.clone(),
                });
            }
            self.validate_row(r, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(CertificationViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(CertificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<CertificationViolation>) {
        if self.schema_version != M5_FAMILY_CERTIFICATION_SCHEMA_VERSION {
            violations.push(CertificationViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_FAMILY_CERTIFICATION_RECORD_KIND {
            violations.push(CertificationViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("qualification_matrix_ref", &self.qualification_matrix_ref),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("diff_report_ref", &self.diff_report_ref),
            ("skew_inspector_ref", &self.skew_inspector_ref),
            ("evidence_index_ref", &self.evidence_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(CertificationViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        let vocab: [(bool, &'static str); 13] = [
            (
                self.lifecycle_labels == StableClaimLevel::ALL.to_vec(),
                "lifecycle_labels",
            ),
            (
                self.family_kinds == FamilyKind::ALL.to_vec(),
                "family_kinds",
            ),
            (
                self.support_classes == SupportClass::ALL.to_vec(),
                "support_classes",
            ),
            (self.row_states == RowState::ALL.to_vec(), "row_states"),
            (
                self.skew_window_classes == SkewWindowClass::ALL.to_vec(),
                "skew_window_classes",
            ),
            (
                self.deprecation_statuses == DeprecationStatus::ALL.to_vec(),
                "deprecation_statuses",
            ),
            (
                self.evidence_states == M5ClaimReportState::ALL.to_vec(),
                "evidence_states",
            ),
            (
                self.pillar_kinds == CertificationPillarKind::ALL.to_vec(),
                "pillar_kinds",
            ),
            (
                self.required_pillars == CertificationPillarKind::REQUIRED.to_vec(),
                "required_pillars",
            ),
            (
                self.certification_states == CertificationState::ALL.to_vec(),
                "certification_states",
            ),
            (
                self.freshness_states == FreshnessSloState::ALL.to_vec(),
                "freshness_states",
            ),
            (
                self.certification_reasons == CertificationReason::ALL.to_vec(),
                "certification_reasons",
            ),
            (
                self.stop_actions == CertificationStopAction::ALL.to_vec(),
                "stop_actions",
            ),
        ];
        for (ok, field) in vocab {
            if !ok {
                violations.push(CertificationViolation::ClosedVocabularyMismatch { field });
            }
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(CertificationViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(CertificationViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(CertificationViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(CertificationViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<CertificationViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(CertificationViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(CertificationViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(CertificationViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(CertificationViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            if rule.blocks_promotion != rule.trigger_reason.blocks_promotion() {
                violations.push(CertificationViolation::StopRuleBlockingMismatch {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in CertificationReason::ALL {
            if !covered.contains(&reason) {
                violations.push(CertificationViolation::ReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        r: &FamilyCertificationPacket,
        violations: &mut Vec<CertificationViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &r.entry_id),
            ("title", &r.title),
            ("family_ref", &r.family_ref),
            ("family_summary", &r.family_summary),
            ("qualification_row_ref", &r.qualification_row_ref),
            ("claim_manifest_entry_ref", &r.claim_manifest_entry_ref),
            ("skew_window_ref", &r.skew_window_ref),
            (
                "diff_deprecation_packet_ref",
                &r.diff_deprecation_packet_ref,
            ),
            ("source_claim_text", &r.source_claim_text),
            ("validity_window.starts_at", &r.validity_window.starts_at),
            ("validity_window.expires_at", &r.validity_window.expires_at),
            ("proof_packet.packet_id", &r.proof_packet.packet_id),
            ("proof_packet.packet_ref", &r.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &r.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &r.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &r.owner_signoff.owner_ref),
            ("rationale", &r.rationale),
        ] {
            if value.trim().is_empty() {
                violations.push(CertificationViolation::EmptyField {
                    entry_id: r.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_pillars(r, violations);

        // The no-overclaim guard: the family may never certify a greener label or a
        // broader support class than the public claim it reuses.
        if r.certified_label.rank() > r.source_published_label.rank() {
            violations.push(CertificationViolation::RowLabelExceedsSource {
                entry_id: r.entry_id.clone(),
                source: r.source_published_label,
                row: r.certified_label,
            });
        }
        if support_breadth(r.certified_support_class) > support_breadth(r.source_support_class) {
            violations.push(CertificationViolation::RowSupportClassExceedsSource {
                entry_id: r.entry_id.clone(),
                source: r.source_support_class,
                row: r.certified_support_class,
            });
        }

        if r.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(CertificationViolation::EmptyField {
                entry_id: r.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !r.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(CertificationViolation::FreshnessSloInconsistent {
                entry_id: r.entry_id.clone(),
            });
        }

        // A public claim narrowed below the cutline must name the inherited reason.
        if !r.source_published_label.is_at_or_above_cutline()
            && !r.has_active_reason(CertificationReason::RowDowngraded)
        {
            violations.push(CertificationViolation::SourceNarrowedWithoutReason {
                entry_id: r.entry_id.clone(),
            });
        }

        // A limited certified support class must record at least one caveat.
        if r.certified_support_class == SupportClass::Limited
            && r.certification_caveats.iter().all(|c| c.trim().is_empty())
        {
            violations.push(CertificationViolation::LimitedWithoutCaveat {
                entry_id: r.entry_id.clone(),
            });
        }

        // The qualification row's stale/retest/skew/deprecation state must carry its
        // matching certification reason, so the certified verdict never loses the
        // row-level governance truth.
        let required_row_reason = match r.row_state {
            RowState::RetestPending => Some(CertificationReason::RetestPending),
            RowState::Stale => Some(CertificationReason::QualificationStale),
            RowState::UnsupportedSkew => Some(CertificationReason::SkewWindowExceeded),
            RowState::Deprecated => Some(CertificationReason::DeprecationScheduled),
            RowState::Qualified | RowState::Limited | RowState::OnWaiver | RowState::Incomplete => {
                None
            }
        };
        if let Some(reason) = required_row_reason {
            if !r.has_active_reason(reason) {
                violations.push(CertificationViolation::RowStateWithoutReason {
                    entry_id: r.entry_id.clone(),
                    row_state: r.row_state,
                    reason,
                });
            }
        }

        if r.holds_certification() {
            self.validate_certified_row(r, violations);
        } else {
            self.validate_narrowed_row(r, violations);
        }
    }

    fn validate_pillars(
        &self,
        r: &FamilyCertificationPacket,
        violations: &mut Vec<CertificationViolation>,
    ) {
        let mut seen: BTreeSet<CertificationPillarKind> = BTreeSet::new();
        for p in &r.pillars {
            if !seen.insert(p.kind) {
                violations.push(CertificationViolation::DuplicatePillar {
                    entry_id: r.entry_id.clone(),
                    kind: p.kind,
                });
            }
            if p.summary.trim().is_empty() {
                violations.push(CertificationViolation::EmptyField {
                    entry_id: r.entry_id.clone(),
                    field_name: "pillar.summary",
                });
            }
            // A present pillar carries a location; only a missing one carries none.
            if p.state != M5ClaimReportState::Missing && p.pillar_ref.trim().is_empty() {
                violations.push(CertificationViolation::PillarRefIncomplete {
                    entry_id: r.entry_id.clone(),
                    kind: p.kind,
                });
            }
        }
        // Every family must bind all four governance pillars, so the certification
        // packet binds the qualification row, the skew window, the diff/deprecation
        // packet, and the public claim entry into one verdict.
        for required in CertificationPillarKind::REQUIRED {
            if !seen.contains(&required) {
                violations.push(CertificationViolation::RequiredPillarUncovered {
                    entry_id: r.entry_id.clone(),
                    kind: required,
                });
            }
        }

        // Each pillar must name the reopen ref the packet declares, so the bound
        // governance record and the packet's reopen handle agree.
        let pillar_ref_matches = |kind: CertificationPillarKind, expected: &str| {
            r.pillar(kind).map_or(true, |p| p.pillar_ref == expected)
        };
        if !pillar_ref_matches(
            CertificationPillarKind::QualificationMatrix,
            &r.qualification_row_ref,
        ) {
            violations.push(CertificationViolation::PillarRefDrift {
                entry_id: r.entry_id.clone(),
                kind: CertificationPillarKind::QualificationMatrix,
            });
        }
        if !pillar_ref_matches(CertificationPillarKind::SkewWindow, &r.skew_window_ref) {
            violations.push(CertificationViolation::PillarRefDrift {
                entry_id: r.entry_id.clone(),
                kind: CertificationPillarKind::SkewWindow,
            });
        }
        if !pillar_ref_matches(
            CertificationPillarKind::DiffDeprecation,
            &r.diff_deprecation_packet_ref,
        ) {
            violations.push(CertificationViolation::PillarRefDrift {
                entry_id: r.entry_id.clone(),
                kind: CertificationPillarKind::DiffDeprecation,
            });
        }
        if !pillar_ref_matches(
            CertificationPillarKind::ClaimPublication,
            &r.claim_manifest_entry_ref,
        ) {
            violations.push(CertificationViolation::PillarRefDrift {
                entry_id: r.entry_id.clone(),
                kind: CertificationPillarKind::ClaimPublication,
            });
        }

        // A stale qualification pillar or a missing claim/diff pillar must name its
        // matching reason, so a pillar that thins out always narrows the certification.
        if let Some(p) = r.pillar(CertificationPillarKind::QualificationMatrix) {
            if p.state == M5ClaimReportState::Stale
                && !r.has_active_reason(CertificationReason::QualificationStale)
            {
                violations.push(CertificationViolation::PillarStateWithoutReason {
                    entry_id: r.entry_id.clone(),
                    kind: p.kind,
                    reason: CertificationReason::QualificationStale,
                });
            }
        }
        if let Some(p) = r.pillar(CertificationPillarKind::ClaimPublication) {
            if p.state == M5ClaimReportState::Missing
                && !r.has_active_reason(CertificationReason::ClaimPublicationMissing)
            {
                violations.push(CertificationViolation::PillarStateWithoutReason {
                    entry_id: r.entry_id.clone(),
                    kind: p.kind,
                    reason: CertificationReason::ClaimPublicationMissing,
                });
            }
        }
        if let Some(p) = r.pillar(CertificationPillarKind::DiffDeprecation) {
            if p.state == M5ClaimReportState::Missing
                && !r.has_active_reason(CertificationReason::DiffReportMissing)
            {
                violations.push(CertificationViolation::PillarStateWithoutReason {
                    entry_id: r.entry_id.clone(),
                    kind: p.kind,
                    reason: CertificationReason::DiffReportMissing,
                });
            }
        }
    }

    fn validate_certified_row(
        &self,
        r: &FamilyCertificationPacket,
        violations: &mut Vec<CertificationViolation>,
    ) {
        // A certified family reuses the public claim's label and support class verbatim
        // (claim-manifest parity), that label is at or above the cutline, it names no
        // active reason, rides a captured within-SLO packet, all four pillars are
        // current inside an open validity window, and it is owner-signed.
        if r.certified_label != r.source_published_label {
            violations.push(CertificationViolation::CertifiedLabelNotSource {
                entry_id: r.entry_id.clone(),
                source: r.source_published_label,
                row: r.certified_label,
            });
        }
        if r.certified_support_class != r.source_support_class {
            violations.push(CertificationViolation::CertifiedSupportClassNotSource {
                entry_id: r.entry_id.clone(),
                source: r.source_support_class,
                row: r.certified_support_class,
            });
        }
        if !r.certifies_stable() {
            violations.push(CertificationViolation::CertifiedStateNotStable {
                entry_id: r.entry_id.clone(),
                certified: r.certified_label,
            });
        }
        if !r.active_certification_reasons.is_empty() {
            violations.push(CertificationViolation::CertifiedWithActiveGap {
                entry_id: r.entry_id.clone(),
            });
        }
        if !r.proof_packet.has_capture() {
            violations.push(CertificationViolation::CertifiedWithoutFreshPacket {
                entry_id: r.entry_id.clone(),
            });
        }
        if !r.proof_packet.slo_state.is_within_slo() {
            violations.push(CertificationViolation::CertifiedOnStalePacket {
                entry_id: r.entry_id.clone(),
                slo_state: r.proof_packet.slo_state,
            });
        }
        for p in &r.pillars {
            if !p.state.is_current() {
                violations.push(CertificationViolation::CertifiedWithStalePillar {
                    entry_id: r.entry_id.clone(),
                    kind: p.kind,
                    state: p.state,
                });
            }
        }
        if r.validity_window.expired {
            violations.push(CertificationViolation::CertifiedWithExpiredWindow {
                entry_id: r.entry_id.clone(),
            });
        }
        if !(r.owner_signoff.signed_off && r.owner_signoff.signed_at.is_some()) {
            violations.push(CertificationViolation::CertifiedWithoutSignoff {
                entry_id: r.entry_id.clone(),
            });
        }
    }

    fn validate_narrowed_row(
        &self,
        r: &FamilyCertificationPacket,
        violations: &mut Vec<CertificationViolation>,
    ) {
        // A narrowing family must drop below the cutline and name at least one reason.
        if r.certifies_stable() {
            violations.push(CertificationViolation::NarrowedButCertifiedStable {
                entry_id: r.entry_id.clone(),
                state: r.certification_state,
                certified: r.certified_label,
            });
        }
        if r.active_certification_reasons.is_empty() {
            violations.push(CertificationViolation::NarrowingWithoutReason {
                entry_id: r.entry_id.clone(),
                state: r.certification_state,
            });
        }

        // The narrowing state must be coherent with its active reasons.
        let any =
            |reasons: &[CertificationReason]| reasons.iter().any(|r2| r.has_active_reason(*r2));
        let coherent = match r.certification_state {
            CertificationState::NarrowedRowDowngraded => any(&[CertificationReason::RowDowngraded]),
            CertificationState::NarrowedStale => any(&[
                CertificationReason::EvidenceStale,
                CertificationReason::QualificationStale,
            ]),
            CertificationState::NarrowedRetestPending => any(&[CertificationReason::RetestPending]),
            CertificationState::Withheld => any(&[
                CertificationReason::ValidityWindowExpired,
                CertificationReason::OwnerSignoffMissing,
                CertificationReason::ClaimPublicationMissing,
                CertificationReason::DiffReportMissing,
                CertificationReason::EvidenceMissing,
            ]),
            CertificationState::Certified => true,
        };
        if !coherent {
            violations.push(CertificationViolation::StateReasonIncoherent {
                entry_id: r.entry_id.clone(),
                state: r.certification_state,
            });
        }

        // A stale or missing proof packet must name its matching reason.
        if r.proof_packet.slo_state == FreshnessSloState::Breached
            && !r.has_active_reason(CertificationReason::EvidenceStale)
        {
            violations.push(CertificationViolation::StateWithoutReason {
                entry_id: r.entry_id.clone(),
                reason: CertificationReason::EvidenceStale,
            });
        }
        if r.proof_packet.slo_state == FreshnessSloState::Missing
            && !r.has_active_reason(CertificationReason::EvidenceMissing)
        {
            violations.push(CertificationViolation::StateWithoutReason {
                entry_id: r.entry_id.clone(),
                reason: CertificationReason::EvidenceMissing,
            });
        }
        // An expired validity window must name its reason.
        if r.validity_window.expired
            && !r.has_active_reason(CertificationReason::ValidityWindowExpired)
        {
            violations.push(CertificationViolation::StateWithoutReason {
                entry_id: r.entry_id.clone(),
                reason: CertificationReason::ValidityWindowExpired,
            });
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<CertificationViolation>) {
        let covered: BTreeSet<String> = self.rows.iter().map(|r| r.family_ref.clone()).collect();
        for declared in &self.release_blocking_family_refs {
            if !covered.contains(declared) {
                violations.push(CertificationViolation::ReleaseBlockingFamilyUncovered {
                    family_ref: declared.clone(),
                });
            }
        }
        for r in &self.rows {
            if r.release_blocking && !self.release_blocking_family_refs.contains(&r.family_ref) {
                violations.push(CertificationViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: r.entry_id.clone(),
                });
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<CertificationViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(CertificationViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(CertificationViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(CertificationViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(CertificationViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_claim_ids() {
            violations.push(CertificationViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the M5 per-family certification register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificationViolation {
    /// The register carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the register.
        actual: u32,
    },
    /// The register carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the register.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The register has no rows.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Row or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// A stop-rule id appears more than once.
    DuplicateStopRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A stop rule names no labels to watch.
    StopRuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A stop rule's blocking flag disagrees with its reason's semantics.
    StopRuleBlockingMismatch {
        /// Rule id.
        rule_id: String,
    },
    /// A narrowing reason has no stop rule watching for it.
    ReasonWithoutStopRule {
        /// Uncovered reason.
        reason: CertificationReason,
    },
    /// A family binds the same pillar kind twice.
    DuplicatePillar {
        /// Row id.
        entry_id: String,
        /// Duplicated pillar kind.
        kind: CertificationPillarKind,
    },
    /// A family does not bind a required governance pillar.
    RequiredPillarUncovered {
        /// Row id.
        entry_id: String,
        /// Uncovered pillar kind.
        kind: CertificationPillarKind,
    },
    /// A pillar ref is incomplete.
    PillarRefIncomplete {
        /// Row id.
        entry_id: String,
        /// Offending pillar kind.
        kind: CertificationPillarKind,
    },
    /// A pillar ref differs from the reopen ref the packet declares.
    PillarRefDrift {
        /// Row id.
        entry_id: String,
        /// Offending pillar kind.
        kind: CertificationPillarKind,
    },
    /// A pillar that thinned out does not name its matching reason.
    PillarStateWithoutReason {
        /// Row id.
        entry_id: String,
        /// Offending pillar kind.
        kind: CertificationPillarKind,
        /// Reason the pillar state requires.
        reason: CertificationReason,
    },
    /// A limited certified support class records no caveat.
    LimitedWithoutCaveat {
        /// Row id.
        entry_id: String,
    },
    /// The certified label is wider than the public claim it reuses.
    RowLabelExceedsSource {
        /// Row id.
        entry_id: String,
        /// Public claim label.
        source: StableClaimLevel,
        /// Certified label.
        row: StableClaimLevel,
    },
    /// The certified support class is broader than the public claim it reuses.
    RowSupportClassExceedsSource {
        /// Row id.
        entry_id: String,
        /// Public support class.
        source: SupportClass,
        /// Certified support class.
        row: SupportClass,
    },
    /// A public claim narrowed below the cutline does not name the inherited reason.
    SourceNarrowedWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A qualification row-state stale/retest/skew/deprecation does not name its
    /// matching reason.
    RowStateWithoutReason {
        /// Row id.
        entry_id: String,
        /// The qualification row state requiring a reason.
        row_state: RowState,
        /// Reason the row state requires.
        reason: CertificationReason,
    },
    /// A certified family does not reuse the public claim's label.
    CertifiedLabelNotSource {
        /// Row id.
        entry_id: String,
        /// Public claim label.
        source: StableClaimLevel,
        /// Certified label.
        row: StableClaimLevel,
    },
    /// A certified family does not reuse the public claim's support class.
    CertifiedSupportClassNotSource {
        /// Row id.
        entry_id: String,
        /// Public support class.
        source: SupportClass,
        /// Certified support class.
        row: SupportClass,
    },
    /// A certified family does not certify at or above the cutline.
    CertifiedStateNotStable {
        /// Row id.
        entry_id: String,
        /// Certified label.
        certified: StableClaimLevel,
    },
    /// A certified family carries active narrowing reasons.
    CertifiedWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A certified family has no captured proof packet.
    CertifiedWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A certified family rides a packet outside its freshness SLO.
    CertifiedOnStalePacket {
        /// Row id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A certified family rides a non-current pillar.
    CertifiedWithStalePillar {
        /// Row id.
        entry_id: String,
        /// Pillar kind.
        kind: CertificationPillarKind,
        /// Pillar state.
        state: M5ClaimReportState,
    },
    /// A certified family rides an expired validity window.
    CertifiedWithExpiredWindow {
        /// Row id.
        entry_id: String,
    },
    /// A certified family lacks owner sign-off.
    CertifiedWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing family did not drop below the cutline.
    NarrowedButCertifiedStable {
        /// Row id.
        entry_id: String,
        /// Certification state.
        state: CertificationState,
        /// Certified label.
        certified: StableClaimLevel,
    },
    /// A narrowing family names no active reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// Certification state.
        state: CertificationState,
    },
    /// A certification state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Certification state.
        state: CertificationState,
    },
    /// A stale/missing/expired input does not name its reason.
    StateWithoutReason {
        /// Row id.
        entry_id: String,
        /// Reason the input state requires.
        reason: CertificationReason,
    },
    /// A release-blocking family ref has no covering row.
    ReleaseBlockingFamilyUncovered {
        /// Family ref.
        family_ref: String,
    },
    /// A release-blocking row is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Row id.
        entry_id: String,
    },
    /// The declared promotion decision disagrees with the computed one.
    PromotionDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared promotion blocking set disagrees with the computed one.
    PromotionBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
}

impl fmt::Display for CertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported register schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported register record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "register {field} is not the canonical value")
            }
            Self::EmptyRegister => write!(f, "register has no rows"),
            Self::NoStopRules => write!(f, "register has no stop rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => write!(f, "duplicate entry id {entry_id}"),
            Self::DuplicateStopRuleId { rule_id } => write!(f, "duplicate stop rule id {rule_id}"),
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::StopRuleBlockingMismatch { rule_id } => write!(
                f,
                "stop rule {rule_id} blocking flag disagrees with its reason"
            ),
            Self::ReasonWithoutStopRule { reason } => write!(
                f,
                "certification reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::DuplicatePillar { entry_id, kind } => {
                write!(f, "row {entry_id} binds pillar {} twice", kind.as_str())
            }
            Self::RequiredPillarUncovered { entry_id, kind } => write!(
                f,
                "row {entry_id} does not bind required governance pillar {}",
                kind.as_str()
            ),
            Self::PillarRefIncomplete { entry_id, kind } => write!(
                f,
                "row {entry_id} pillar {} has an incomplete ref",
                kind.as_str()
            ),
            Self::PillarRefDrift { entry_id, kind } => write!(
                f,
                "row {entry_id} pillar {} ref drifted from the packet's reopen ref",
                kind.as_str()
            ),
            Self::PillarStateWithoutReason {
                entry_id,
                kind,
                reason,
            } => write!(
                f,
                "row {entry_id} pillar {} thinned out without naming {} reason",
                kind.as_str(),
                reason.as_str()
            ),
            Self::LimitedWithoutCaveat { entry_id } => {
                write!(
                    f,
                    "row {entry_id} is limited without a certification caveat"
                )
            }
            Self::RowLabelExceedsSource {
                entry_id,
                source,
                row,
            } => write!(
                f,
                "row {entry_id} certified {row:?} is greener than the public claim {source:?}"
            ),
            Self::RowSupportClassExceedsSource {
                entry_id,
                source,
                row,
            } => write!(
                f,
                "row {entry_id} support class {} is broader than the public claim {}",
                row.as_str(),
                source.as_str()
            ),
            Self::SourceNarrowedWithoutReason { entry_id } => write!(
                f,
                "row {entry_id} public claim narrowed without row_downgraded reason"
            ),
            Self::RowStateWithoutReason {
                entry_id,
                row_state,
                reason,
            } => write!(
                f,
                "row {entry_id} qualification row state {} does not name {} reason",
                row_state.as_str(),
                reason.as_str()
            ),
            Self::CertifiedLabelNotSource {
                entry_id,
                source,
                row,
            } => write!(
                f,
                "row {entry_id} certified label {row:?} does not equal public claim {source:?}"
            ),
            Self::CertifiedSupportClassNotSource {
                entry_id,
                source,
                row,
            } => write!(
                f,
                "row {entry_id} certified support class {} does not equal public claim {}",
                row.as_str(),
                source.as_str()
            ),
            Self::CertifiedStateNotStable {
                entry_id,
                certified,
            } => write!(
                f,
                "row {entry_id} is certified but certifies {certified:?} below the cutline"
            ),
            Self::CertifiedWithActiveGap { entry_id } => {
                write!(f, "row {entry_id} certifies with an active gap")
            }
            Self::CertifiedWithoutFreshPacket { entry_id } => {
                write!(f, "row {entry_id} certifies without a fresh packet")
            }
            Self::CertifiedOnStalePacket {
                entry_id,
                slo_state,
            } => write!(f, "row {entry_id} certifies on stale packet {slo_state:?}"),
            Self::CertifiedWithStalePillar {
                entry_id,
                kind,
                state,
            } => write!(
                f,
                "row {entry_id} certifies on pillar {} in state {}",
                kind.as_str(),
                state.as_str()
            ),
            Self::CertifiedWithExpiredWindow { entry_id } => {
                write!(f, "row {entry_id} certifies on an expired validity window")
            }
            Self::CertifiedWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} certifies without owner signoff")
            }
            Self::NarrowedButCertifiedStable {
                entry_id,
                state,
                certified,
            } => write!(
                f,
                "row {entry_id} state {state:?} must narrow but certifies {certified:?}"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "row {entry_id} state {state:?} narrows without active reason"
            ),
            Self::StateReasonIncoherent { entry_id, state } => write!(
                f,
                "row {entry_id} state {state:?} is incoherent with its active reasons"
            ),
            Self::StateWithoutReason { entry_id, reason } => write!(
                f,
                "row {entry_id} stale/missing/expired input without {} reason",
                reason.as_str()
            ),
            Self::ReleaseBlockingFamilyUncovered { family_ref } => {
                write!(
                    f,
                    "release-blocking family {family_ref} has no covering row"
                )
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking row {entry_id} is not declared in release_blocking_family_refs"
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => write!(
                f,
                "promotion {declared:?} disagrees with computed {computed:?}"
            ),
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with rows"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "row {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for CertificationViolation {}

/// Loads the embedded M5 per-family certification register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`M5FamilyCertificationRegister`].
pub fn current_m5_family_certification() -> Result<M5FamilyCertificationRegister, serde_json::Error>
{
    serde_json::from_str(M5_FAMILY_CERTIFICATION_JSON)
}

#[cfg(test)]
mod tests;
