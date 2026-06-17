//! Typed M5 qualification-matrix and claim-scope export-packet register.
//!
//! Where the qualification/skew matrix is the machine-readable truth for every
//! claimed M5 family and the claim-publication manifest is the single public claim
//! every claim-bearing surface reads, this register is the *export* layer that
//! answers, for support, shiproom, docs, and partner review, exactly which M5 rows
//! are being claimed, what freshness and expiry state each carries, what skew window
//! applies, and what stale or retest-needed states are live — without anyone holding
//! tribal memory. For each claimed family it binds one [`ClaimScopeRow`] that joins:
//!
//! - the upstream qualification row ([`ClaimScopeRow::qualification_row_ref`]), its
//!   deprecation packet ([`ClaimScopeRow::deprecation_packet_ref`]), and the public
//!   claim entry ([`ClaimScopeRow::claim_manifest_entry_ref`]) — the reopen refs a
//!   shiproom dashboard follows back to the authoritative record,
//! - the row-level truth that must never collapse into one flag: the
//!   [`RowState`] (qualified / limited / on-waiver / retest-pending / stale /
//!   unsupported-skew / deprecated / incomplete), the [`SkewWindowClass`], the
//!   [`SupportClass`], the [`DeprecationStatus`], the freshness state, the validity
//!   window, the [`ScopeEvidenceRef`] list, and the active [`ClaimScopeReason`]s,
//! - and the copy-safe scope wording every audience renders
//!   ([`ClaimScopeRow::scope_claim_text`]) — never greener than the public claim's
//!   published label ([`ClaimScopeRow::source_published_label`]) or support class
//!   ([`ClaimScopeRow::source_support_class`]), both hard ceilings.
//!
//! The no-overclaim guard is the spine of the register: an export row may never
//! publish a greener [`ClaimScopeRow::published_label`] than the public claim, never
//! advertise a broader [`ClaimScopeRow::scope_support_class`], and a row that holds
//! the public label must reuse the public wording verbatim. Because every audience
//! ([`ClaimScopeAudience`]) renders from the one row, a narrowed row downgrades the
//! support export, the shiproom card, the docs surface, and the partner-review packet
//! at once, and no audience can keep a greener scope. Every audience must disclose the
//! row's freshness, its active stale/retest reasons, and its caveats, so no exported
//! packet loses the row-level stale or retest-needed reason.
//!
//! A row that merely inherits an upstream narrowing
//! ([`ClaimScopeReason::RowDowngraded`]) downgrades its export audiences but does not
//! itself hold promotion — the qualification matrix and claim manifest already gate
//! the public claim. An *export-layer* failure (stale or missing export evidence; an
//! expired validity window; a lapsed waiver; a missing owner sign-off; or copy that
//! over-claims the public label or support class) on a row whose public claim is
//! still at or above the cutline holds promotion through a [`ClaimScopeStopRule`].
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
//! The register is checked in at [`M5_CLAIM_SCOPE_EXPORT_PACKETS_PATH`] and embedded
//! here, so this typed consumer and the CI gate agree on every row without a cargo
//! build in CI. The model is metadata-only: every field is a typed state or an opaque
//! ref. It carries no raw artifacts, raw logs, signatures, or credential material.

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
pub const M5_CLAIM_SCOPE_EXPORT_PACKETS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_CLAIM_SCOPE_EXPORT_PACKETS_RECORD_KIND: &str =
    "implement_m5_qualification_matrix_and_claim_scope_export_packets";

/// Repo-relative path to the checked-in register.
pub const M5_CLAIM_SCOPE_EXPORT_PACKETS_PATH: &str =
    "artifacts/release/m5/implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth.json";

/// Embedded checked-in register JSON.
pub const M5_CLAIM_SCOPE_EXPORT_PACKETS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth.json"
));

/// The breadth rank of a support class; a broader class ranks higher. An export row
/// may never advertise a support class broader than the public claim it reuses.
const fn support_breadth(class: SupportClass) -> u8 {
    match class {
        SupportClass::FullSupport => 4,
        SupportClass::MaintenanceOnly => 3,
        SupportClass::SecurityOnly => 2,
        SupportClass::Limited => 1,
        SupportClass::EndOfLife => 0,
    }
}

/// A reviewing audience that consumes a claim-scope export packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimScopeAudience {
    /// The support / field-readiness export.
    Support,
    /// The shiproom dashboard card.
    Shiproom,
    /// The public product documentation surface.
    Docs,
    /// The partner / ecosystem review packet.
    PartnerReview,
    /// The release-notes surface.
    ReleaseNotes,
}

impl ClaimScopeAudience {
    /// Every audience, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Support,
        Self::Shiproom,
        Self::Docs,
        Self::PartnerReview,
        Self::ReleaseNotes,
    ];

    /// The audiences every row must drive, so support, shiproom, docs, and partner
    /// review reconstruct the current claim scope from one source.
    pub const REQUIRED: [Self; 4] = [
        Self::Support,
        Self::Shiproom,
        Self::Docs,
        Self::PartnerReview,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Support => "support",
            Self::Shiproom => "shiproom",
            Self::Docs => "docs",
            Self::PartnerReview => "partner_review",
            Self::ReleaseNotes => "release_notes",
        }
    }

    /// Whether an audience must follow the reopen refs back to the authoritative
    /// qualification row and deprecation packet. Shiproom always reopens.
    pub const fn must_reopen_authoritative_row(self) -> bool {
        matches!(self, Self::Shiproom)
    }
}

/// The kind of backing record a [`ScopeEvidenceRef`] points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeEvidenceKind {
    /// The upstream qualification matrix row.
    QualificationRow,
    /// The family's deprecation packet.
    DeprecationPacket,
    /// The declared skew-window record.
    SkewWindow,
    /// The declared support-window record.
    SupportWindow,
    /// The backing compatibility report.
    CompatibilityReport,
    /// The public claim-publication manifest entry.
    ClaimManifest,
    /// The export proof packet / proof-index row.
    ProofPacket,
}

impl ScopeEvidenceKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::QualificationRow,
        Self::DeprecationPacket,
        Self::SkewWindow,
        Self::SupportWindow,
        Self::CompatibilityReport,
        Self::ClaimManifest,
        Self::ProofPacket,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualificationRow => "qualification_row",
            Self::DeprecationPacket => "deprecation_packet",
            Self::SkewWindow => "skew_window",
            Self::SupportWindow => "support_window",
            Self::CompatibilityReport => "compatibility_report",
            Self::ClaimManifest => "claim_manifest",
            Self::ProofPacket => "proof_packet",
        }
    }
}

/// Overall export state a claim-scope row earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimScopeRowState {
    /// The row publishes the public claim's label; all export evidence is current.
    Published,
    /// The row inherited an upstream qualification/claim narrowing.
    NarrowedRowDowngraded,
    /// Export evidence or a backing report has gone stale; the row narrows.
    NarrowedStale,
    /// A retest is pending on the row; the row narrows.
    NarrowedRetestPending,
    /// The row is withheld entirely (expired window/waiver, missing sign-off,
    /// missing claim, or missing evidence).
    Withheld,
}

impl ClaimScopeRowState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Published,
        Self::NarrowedRowDowngraded,
        Self::NarrowedStale,
        Self::NarrowedRetestPending,
        Self::Withheld,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::NarrowedRowDowngraded => "narrowed_row_downgraded",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedRetestPending => "narrowed_retest_pending",
            Self::Withheld => "withheld",
        }
    }

    /// Whether the state lets the row publish the public claim's label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Published)
    }
}

/// Closed reason a claim-scope row narrows below the public claim it reuses or a stop
/// rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimScopeReason {
    /// The reused qualification row or claim publication narrowed below the cutline.
    RowDowngraded,
    /// A qualification dimension's evidence has gone stale.
    QualificationStale,
    /// A dimension or boundary requires a retest before it may re-qualify.
    RetestPending,
    /// A peer is outside the supported skew window.
    SkewWindowExceeded,
    /// The family is deprecated with a scheduled removal.
    DeprecationScheduled,
    /// The support window has ended.
    SupportWindowEnded,
    /// The claim-scope validity window has expired.
    ValidityWindowExpired,
    /// The export proof packet or a backing report breached its freshness SLO.
    EvidenceStale,
    /// No export proof packet or backing report has been captured.
    EvidenceMissing,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
    /// A waiver the row relied on has expired.
    WaiverExpired,
    /// The backing public claim publication is missing.
    ClaimPublicationMissing,
}

impl ClaimScopeReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::RowDowngraded,
        Self::QualificationStale,
        Self::RetestPending,
        Self::SkewWindowExceeded,
        Self::DeprecationScheduled,
        Self::SupportWindowEnded,
        Self::ValidityWindowExpired,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
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
            Self::SupportWindowEnded => "support_window_ended",
            Self::ValidityWindowExpired => "validity_window_expired",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceMissing => "evidence_missing",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::ClaimPublicationMissing => "claim_publication_missing",
        }
    }

    /// Whether a row whose public claim is at or above the cutline carrying this
    /// reason holds promotion. A reason that merely inherits an upstream narrowing is
    /// gated by the qualification matrix and claim manifest, not this register.
    pub const fn blocks_promotion(self) -> bool {
        !matches!(self, Self::RowDowngraded)
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimScopeStopAction {
    /// Hold the export publication until the condition clears.
    HoldExport,
    /// Narrow the row to inherit the public claim.
    NarrowRow,
    /// Withhold the row entirely.
    WithholdRow,
    /// Refresh the export evidence.
    RefreshEvidence,
    /// Schedule the pending retest.
    ScheduleRetest,
    /// Renew the validity window.
    RenewValidityWindow,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
    /// Align the scope wording to the public claim.
    AlignCopyToSource,
}

impl ClaimScopeStopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::HoldExport,
        Self::NarrowRow,
        Self::WithholdRow,
        Self::RefreshEvidence,
        Self::ScheduleRetest,
        Self::RenewValidityWindow,
        Self::RequestOwnerSignoff,
        Self::AlignCopyToSource,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldExport => "hold_export",
            Self::NarrowRow => "narrow_row",
            Self::WithholdRow => "withhold_row",
            Self::RefreshEvidence => "refresh_evidence",
            Self::ScheduleRetest => "schedule_retest",
            Self::RenewValidityWindow => "renew_validity_window",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::AlignCopyToSource => "align_copy_to_source",
        }
    }
}

/// The validity window the claim scope is asserted within.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimScopeValidityWindow {
    /// UTC date the claim scope becomes valid.
    pub starts_at: String,
    /// UTC date the claim scope expires and must be renewed.
    pub expires_at: String,
    /// Whether the window has expired as of the register's `as_of` date.
    pub expired: bool,
}

/// One backing record the export row points at, with its freshness state.
///
/// The refs are the reopen handles a shiproom dashboard follows back to the
/// authoritative qualification row, deprecation packet, or claim manifest entry; the
/// state surfaces stale or missing evidence at row level.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeEvidenceRef {
    /// The kind of record this ref names.
    pub kind: ScopeEvidenceKind,
    /// Opaque ref to the record. Empty only when the state is `missing`.
    pub evidence_ref: String,
    /// The record's freshness/integrity state.
    pub state: M5ClaimReportState,
}

/// One reviewing audience's rendering of a claim-scope row.
///
/// Each rendering reads the row id, the published label, the support class, and the
/// exact scope wording from the one row, so a narrowed row downgrades every audience
/// at once and no audience can keep a greener scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimScopeAudienceRendering {
    /// The audience this rendering targets.
    pub audience: ClaimScopeAudience,
    /// The row id this audience renders from. Equals the row entry id.
    pub source_row_id: String,
    /// The label rendered. Equals the row's published label.
    pub rendered_label: StableClaimLevel,
    /// The support class rendered. Equals the row's scope support class.
    pub rendered_support_class: SupportClass,
    /// The exact wording rendered. Equals the row's scope claim text.
    pub rendered_claim_text: String,
    /// Whether the audience discloses the row freshness. Always required.
    pub discloses_freshness: bool,
    /// Whether the audience discloses the active stale/retest reasons. Required when
    /// the row carries any.
    pub discloses_scope_reasons: bool,
    /// Whether the audience discloses the scope caveats. Required when any exist.
    pub discloses_caveats: bool,
    /// Whether the audience exposes the reopen refs back to the authoritative
    /// qualification row and deprecation packet. Always required for shiproom.
    pub reopens_authoritative_row: bool,
}

/// One claim-scope export stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimScopeStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched row fires this rule.
    pub trigger_reason: ClaimScopeReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: ClaimScopeStopAction,
    /// Whether firing this rule holds promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One claim-scope export row: the support/shiproom/docs/partner view over one
/// family's qualification row and public claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimScopeRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The family this row governs.
    pub family_kind: FamilyKind,
    /// The family ref this row speaks about.
    pub family_ref: String,
    /// Reviewable one-line statement of the family.
    pub family_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The qualification-matrix row entry id this export joins to (a reopen ref).
    pub qualification_row_ref: String,
    /// The deprecation-packet ref this export joins to (a reopen ref).
    pub deprecation_packet_ref: String,
    /// The claim-publication manifest entry id this export joins to (a reopen ref).
    pub claim_manifest_entry_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// The public claim's effective published label (the hard ceiling for this row).
    pub source_published_label: StableClaimLevel,
    /// The public claim's support class (the support ceiling for this row).
    pub source_support_class: SupportClass,
    /// The public claim's exact wording, mirrored verbatim from the claim manifest.
    pub source_claim_text: String,
    /// The qualification row state earned upstream.
    pub row_state: RowState,
    /// The declared skew-window class for the family's boundary.
    pub skew_window_class: SkewWindowClass,
    /// The family's deprecation status.
    pub deprecation_status: DeprecationStatus,
    /// The backing records this row points at. Always at least one.
    pub evidence_refs: Vec<ScopeEvidenceRef>,
    /// The validity window the claim scope is asserted within.
    pub validity_window: ClaimScopeValidityWindow,
    /// Overall export state earned.
    pub export_state: ClaimScopeRowState,
    /// The support class the export advertises. Never broader than the public class.
    pub scope_support_class: SupportClass,
    /// The lifecycle label the export effectively publishes. Never greener than the
    /// public claim's published label.
    pub published_label: StableClaimLevel,
    /// The copy-safe scope wording every audience renders. A row that holds the
    /// public label reuses the public wording verbatim.
    pub scope_claim_text: String,
    /// The scope caveats that travel with the row. Non-empty when support is limited.
    #[serde(default)]
    pub scope_caveats: Vec<String>,
    /// The reviewing audiences the row drives. Always covers the required set
    /// (support, shiproom, docs, partner review).
    pub audiences: Vec<ClaimScopeAudienceRendering>,
    /// The export proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional scope, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the row below the public claim label.
    #[serde(default)]
    pub active_scope_reasons: Vec<ClaimScopeReason>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl ClaimScopeRow {
    /// True when the row's published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim it reuses is itself at or above the cutline.
    pub fn source_holds_stable(&self) -> bool {
        self.source_published_label.is_at_or_above_cutline()
    }

    /// True when the export state lets the row carry the public claim's label.
    pub fn holds_label(&self) -> bool {
        self.export_state.holds_label()
    }

    /// True when a narrowing reason is active on the row.
    pub fn has_active_reason(&self, reason: ClaimScopeReason) -> bool {
        self.active_scope_reasons.contains(&reason)
    }

    /// True when the export advertises a label or support class wider than the public
    /// claim it reuses.
    pub fn over_claims_source(&self) -> bool {
        self.published_label.rank() > self.source_published_label.rank()
            || support_breadth(self.scope_support_class)
                > support_breadth(self.source_support_class)
    }

    /// The freshness state the row discloses (the export proof packet's SLO state).
    pub fn freshness_state(&self) -> FreshnessSloState {
        self.proof_packet.slo_state
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimScopeSummary {
    /// Total number of rows.
    pub total_rows: usize,
    /// Distinct families covered.
    pub total_families: usize,
    /// Rows publishing at or above the cutline.
    pub rows_published: usize,
    /// Rows narrowed below the cutline.
    pub rows_narrowed: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing at or above the cutline.
    pub release_blocking_published: usize,
    /// Release-blocking rows narrowed below the cutline.
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
    /// Rows in the `published` state.
    pub state_published: usize,
    /// Rows in the `narrowed_row_downgraded` state.
    pub state_narrowed_row_downgraded: usize,
    /// Rows in the `narrowed_stale` state.
    pub state_narrowed_stale: usize,
    /// Rows in the `narrowed_retest_pending` state.
    pub state_narrowed_retest_pending: usize,
    /// Rows in the `withheld` state.
    pub state_withheld: usize,
    /// Rows carrying at least one scope caveat.
    pub rows_with_caveats: usize,
    /// Total scope caveats across all rows.
    pub total_caveats: usize,
    /// Total evidence refs across all rows.
    pub total_evidence_refs: usize,
    /// Evidence refs that are current.
    pub evidence_current: usize,
    /// Evidence refs that are stale.
    pub evidence_stale: usize,
    /// Evidence refs that are missing.
    pub evidence_missing: usize,
    /// Evidence refs that are dropped.
    pub evidence_dropped: usize,
    /// Evidence refs that are unsigned.
    pub evidence_unsigned: usize,
    /// Total audience renderings across all rows.
    pub total_audiences: usize,
    /// Audience renderings that disclose the row freshness.
    pub audiences_freshness_disclosed: usize,
    /// Audience renderings that disclose the active stale/retest reasons.
    pub audiences_reasons_disclosed: usize,
    /// Audience renderings that expose the reopen refs.
    pub audiences_reopen_disclosed: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all rows.
    pub total_active_scope_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimScopeExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The family this row governs.
    pub family_kind: FamilyKind,
    /// The family ref this row speaks about.
    pub family_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The qualification-matrix row entry id this export joins to.
    pub qualification_row_ref: String,
    /// The deprecation-packet ref this export joins to.
    pub deprecation_packet_ref: String,
    /// The claim-publication manifest entry this export joins to.
    pub claim_manifest_entry_ref: String,
    /// The canonical claim label.
    pub claim_label: StableClaimLevel,
    /// The public claim's published label (the ceiling).
    pub source_published_label: StableClaimLevel,
    /// The row's effective published label.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall export state earned.
    pub export_state: ClaimScopeRowState,
    /// The qualification row state earned upstream.
    pub row_state: RowState,
    /// The declared skew-window class.
    pub skew_window_class: SkewWindowClass,
    /// The deprecation status.
    pub deprecation_status: DeprecationStatus,
    /// The support class the export advertises.
    pub scope_support_class: SupportClass,
    /// The copy-safe scope wording every audience renders.
    pub scope_claim_text: String,
    /// The disclosed freshness state.
    pub freshness_state: FreshnessSloState,
    /// The scope caveats that travel with the row.
    pub scope_caveats: Vec<String>,
    /// The number of backing evidence refs.
    pub evidence_ref_count: usize,
    /// Active narrowing reasons.
    pub active_scope_reasons: Vec<ClaimScopeReason>,
}

/// Export projection for support, shiproom, docs, and partner-review surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimScopeExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<ClaimScopeExportRow>,
}

/// The typed M5 qualification-matrix and claim-scope export-packet register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimScopeExportRegister {
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
    /// Ref to the qualification/skew matrix whose rows this register exports.
    pub qualification_matrix_ref: String,
    /// Ref to the claim-publication register whose public claims this register reuses.
    pub claim_manifest_ref: String,
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
    /// Closed evidence-kind vocabulary.
    pub evidence_kinds: Vec<ScopeEvidenceKind>,
    /// Closed audience vocabulary.
    pub audiences: Vec<ClaimScopeAudience>,
    /// The required reviewing audiences every row must drive.
    pub required_audiences: Vec<ClaimScopeAudience>,
    /// Closed export row-state vocabulary.
    pub export_states: Vec<ClaimScopeRowState>,
    /// Closed freshness-state vocabulary.
    pub freshness_states: Vec<FreshnessSloState>,
    /// Closed narrowing-reason vocabulary.
    pub scope_reasons: Vec<ClaimScopeReason>,
    /// Closed stop-action vocabulary.
    pub stop_actions: Vec<ClaimScopeStopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking family refs this register must cover.
    pub release_blocking_family_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<ClaimScopeStopRule>,
    /// Claim-scope export rows.
    pub rows: Vec<ClaimScopeRow>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: ClaimScopeSummary,
}

impl ClaimScopeExportRegister {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&ClaimScopeRow> {
        self.rows.iter().find(|r| r.entry_id == entry_id)
    }

    /// Returns the rows publishing at or above the cutline.
    pub fn rows_published(&self) -> Vec<&ClaimScopeRow> {
        self.rows.iter().filter(|r| r.publishes_stable()).collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&ClaimScopeRow> {
        self.rows.iter().filter(|r| !r.publishes_stable()).collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&ClaimScopeRow> {
        self.rows.iter().filter(|r| r.release_blocking).collect()
    }

    /// Returns the rows for one family kind.
    pub fn rows_for_kind(&self, kind: FamilyKind) -> Vec<&ClaimScopeRow> {
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

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &ClaimScopeStopRule) -> bool {
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

    /// Row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose
    /// public claim is already narrowed merely inherits the ceiling, and the
    /// qualification matrix and claim manifest already hold promotion for it.
    pub fn computed_blocking_claim_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<ClaimScopeReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for r in &self.rows {
            if r.source_holds_stable()
                && r.active_scope_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(r.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Counts the evidence refs across all rows in `state`.
    fn evidence_in(&self, state: M5ClaimReportState) -> usize {
        self.rows
            .iter()
            .flat_map(|r| r.evidence_refs.iter())
            .filter(|e| e.state == state)
            .count()
    }

    /// Recomputes the summary block from the rows and stop rules.
    pub fn computed_summary(&self) -> ClaimScopeSummary {
        let kind = |kind: FamilyKind| self.rows_for_kind(kind).len();
        let state = |state: ClaimScopeRowState| {
            self.rows.iter().filter(|r| r.export_state == state).count()
        };
        let packets = |s: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|r| r.proof_packet.slo_state == s)
                .count()
        };
        let release_blocking: Vec<&ClaimScopeRow> = self.release_blocking_rows();
        ClaimScopeSummary {
            total_rows: self.rows.len(),
            total_families: self.families().len(),
            rows_published: self.rows_published().len(),
            rows_narrowed: self.rows_narrowed().len(),
            release_blocking_total: release_blocking.len(),
            release_blocking_published: release_blocking
                .iter()
                .filter(|r| r.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|r| !r.publishes_stable())
                .count(),
            notebook_rows: kind(FamilyKind::Notebook),
            ai_provider_rows: kind(FamilyKind::AiProvider),
            remote_helper_rows: kind(FamilyKind::RemoteHelper),
            companion_rows: kind(FamilyKind::Companion),
            ecosystem_rows: kind(FamilyKind::Ecosystem),
            managed_service_rows: kind(FamilyKind::ManagedService),
            toolchain_runtime_rows: kind(FamilyKind::ToolchainRuntime),
            state_published: state(ClaimScopeRowState::Published),
            state_narrowed_row_downgraded: state(ClaimScopeRowState::NarrowedRowDowngraded),
            state_narrowed_stale: state(ClaimScopeRowState::NarrowedStale),
            state_narrowed_retest_pending: state(ClaimScopeRowState::NarrowedRetestPending),
            state_withheld: state(ClaimScopeRowState::Withheld),
            rows_with_caveats: self
                .rows
                .iter()
                .filter(|r| !r.scope_caveats.is_empty())
                .count(),
            total_caveats: self.rows.iter().map(|r| r.scope_caveats.len()).sum(),
            total_evidence_refs: self.rows.iter().map(|r| r.evidence_refs.len()).sum(),
            evidence_current: self.evidence_in(M5ClaimReportState::Current),
            evidence_stale: self.evidence_in(M5ClaimReportState::Stale),
            evidence_missing: self.evidence_in(M5ClaimReportState::Missing),
            evidence_dropped: self.evidence_in(M5ClaimReportState::Dropped),
            evidence_unsigned: self.evidence_in(M5ClaimReportState::Unsigned),
            total_audiences: self.rows.iter().map(|r| r.audiences.len()).sum(),
            audiences_freshness_disclosed: self
                .rows
                .iter()
                .flat_map(|r| r.audiences.iter())
                .filter(|a| a.discloses_freshness)
                .count(),
            audiences_reasons_disclosed: self
                .rows
                .iter()
                .flat_map(|r| r.audiences.iter())
                .filter(|a| a.discloses_scope_reasons)
                .count(),
            audiences_reopen_disclosed: self
                .rows
                .iter()
                .flat_map(|r| r.audiences.iter())
                .filter(|a| a.reopens_authoritative_row)
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_scope_reasons: self
                .rows
                .iter()
                .map(|r| r.active_scope_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export-safe projection that downstream surfaces render instead of
    /// cloning status text. The reopen refs, the exact scope wording, the freshness
    /// state, the row-level reasons, and the caveats travel with every row, so
    /// support, shiproom, docs, and partner review reconstruct from one source.
    pub fn support_export_projection(&self) -> ClaimScopeExportProjection {
        ClaimScopeExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|r| ClaimScopeExportRow {
                    entry_id: r.entry_id.clone(),
                    family_kind: r.family_kind,
                    family_ref: r.family_ref.clone(),
                    release_blocking: r.release_blocking,
                    qualification_row_ref: r.qualification_row_ref.clone(),
                    deprecation_packet_ref: r.deprecation_packet_ref.clone(),
                    claim_manifest_entry_ref: r.claim_manifest_entry_ref.clone(),
                    claim_label: r.claim_label,
                    source_published_label: r.source_published_label,
                    published_label: r.published_label,
                    publishes_stable: r.publishes_stable(),
                    export_state: r.export_state,
                    row_state: r.row_state,
                    skew_window_class: r.skew_window_class,
                    deprecation_status: r.deprecation_status,
                    scope_support_class: r.scope_support_class,
                    scope_claim_text: r.scope_claim_text.clone(),
                    freshness_state: r.freshness_state(),
                    scope_caveats: r.scope_caveats.clone(),
                    evidence_ref_count: r.evidence_refs.len(),
                    active_scope_reasons: r.active_scope_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<ClaimScopeViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for r in &self.rows {
            if !seen.insert(r.entry_id.clone()) {
                violations.push(ClaimScopeViolation::DuplicateEntryId {
                    entry_id: r.entry_id.clone(),
                });
            }
            self.validate_row(r, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(ClaimScopeViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(ClaimScopeViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<ClaimScopeViolation>) {
        if self.schema_version != M5_CLAIM_SCOPE_EXPORT_PACKETS_SCHEMA_VERSION {
            violations.push(ClaimScopeViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_CLAIM_SCOPE_EXPORT_PACKETS_RECORD_KIND {
            violations.push(ClaimScopeViolation::UnsupportedRecordKind {
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
            ("evidence_index_ref", &self.evidence_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(ClaimScopeViolation::EmptyField {
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
                self.evidence_kinds == ScopeEvidenceKind::ALL.to_vec(),
                "evidence_kinds",
            ),
            (
                self.audiences == ClaimScopeAudience::ALL.to_vec(),
                "audiences",
            ),
            (
                self.required_audiences == ClaimScopeAudience::REQUIRED.to_vec(),
                "required_audiences",
            ),
            (
                self.export_states == ClaimScopeRowState::ALL.to_vec(),
                "export_states",
            ),
            (
                self.freshness_states == FreshnessSloState::ALL.to_vec(),
                "freshness_states",
            ),
            (
                self.scope_reasons == ClaimScopeReason::ALL.to_vec(),
                "scope_reasons",
            ),
        ];
        for (ok, field) in vocab {
            if !ok {
                violations.push(ClaimScopeViolation::ClosedVocabularyMismatch { field });
            }
        }
        if self.stop_actions != ClaimScopeStopAction::ALL.to_vec() {
            violations.push(ClaimScopeViolation::ClosedVocabularyMismatch {
                field: "stop_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(ClaimScopeViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(ClaimScopeViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(ClaimScopeViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(ClaimScopeViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<ClaimScopeViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(ClaimScopeViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(ClaimScopeViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(ClaimScopeViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(ClaimScopeViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            if rule.blocks_promotion != rule.trigger_reason.blocks_promotion() {
                violations.push(ClaimScopeViolation::StopRuleBlockingMismatch {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in ClaimScopeReason::ALL {
            if !covered.contains(&reason) {
                violations.push(ClaimScopeViolation::ScopeReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(&self, r: &ClaimScopeRow, violations: &mut Vec<ClaimScopeViolation>) {
        for (field, value) in [
            ("entry_id", &r.entry_id),
            ("title", &r.title),
            ("family_ref", &r.family_ref),
            ("family_summary", &r.family_summary),
            ("qualification_row_ref", &r.qualification_row_ref),
            ("deprecation_packet_ref", &r.deprecation_packet_ref),
            ("claim_manifest_entry_ref", &r.claim_manifest_entry_ref),
            ("source_claim_text", &r.source_claim_text),
            ("scope_claim_text", &r.scope_claim_text),
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
                violations.push(ClaimScopeViolation::EmptyField {
                    entry_id: r.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_evidence(r, violations);
        self.validate_audiences(r, violations);

        // The no-overclaim guard: the row may never publish a greener label or a
        // broader support class than the public claim it reuses.
        if r.published_label.rank() > r.source_published_label.rank() {
            violations.push(ClaimScopeViolation::RowLabelExceedsSource {
                entry_id: r.entry_id.clone(),
                source: r.source_published_label,
                row: r.published_label,
            });
        }
        if support_breadth(r.scope_support_class) > support_breadth(r.source_support_class) {
            violations.push(ClaimScopeViolation::RowSupportClassExceedsSource {
                entry_id: r.entry_id.clone(),
                source: r.source_support_class,
                row: r.scope_support_class,
            });
        }

        if r.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(ClaimScopeViolation::EmptyField {
                entry_id: r.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !r.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(ClaimScopeViolation::FreshnessSloInconsistent {
                entry_id: r.entry_id.clone(),
            });
        }

        // A public claim narrowed below the cutline must name the inherited reason.
        if !r.source_published_label.is_at_or_above_cutline()
            && !r.has_active_reason(ClaimScopeReason::RowDowngraded)
        {
            violations.push(ClaimScopeViolation::SourceNarrowedWithoutReason {
                entry_id: r.entry_id.clone(),
            });
        }

        // A limited support class must record at least one scope caveat.
        if r.scope_support_class == SupportClass::Limited
            && r.scope_caveats.iter().all(|c| c.trim().is_empty())
        {
            violations.push(ClaimScopeViolation::LimitedWithoutCaveat {
                entry_id: r.entry_id.clone(),
            });
        }

        // The qualification row's stale/retest/skew/deprecation state must carry its
        // matching export reason, so the exported packet never loses the row-level
        // stale or retest-needed truth.
        let required_row_reason = match r.row_state {
            RowState::RetestPending => Some(ClaimScopeReason::RetestPending),
            RowState::Stale => Some(ClaimScopeReason::QualificationStale),
            RowState::UnsupportedSkew => Some(ClaimScopeReason::SkewWindowExceeded),
            RowState::Deprecated => Some(ClaimScopeReason::DeprecationScheduled),
            RowState::Qualified | RowState::Limited | RowState::OnWaiver | RowState::Incomplete => {
                None
            }
        };
        if let Some(reason) = required_row_reason {
            if !r.has_active_reason(reason) {
                violations.push(ClaimScopeViolation::RowStateWithoutReason {
                    entry_id: r.entry_id.clone(),
                    row_state: r.row_state,
                    reason,
                });
            }
        }

        if r.holds_label() {
            self.validate_published_row(r, violations);
        } else {
            self.validate_narrowed_row(r, violations);
        }
    }

    fn validate_evidence(&self, r: &ClaimScopeRow, violations: &mut Vec<ClaimScopeViolation>) {
        if r.evidence_refs.is_empty() {
            violations.push(ClaimScopeViolation::RowWithoutEvidence {
                entry_id: r.entry_id.clone(),
            });
        }
        // Every row must point at the qualification row and the claim manifest, so a
        // shiproom dashboard can always reopen the authoritative records.
        let kinds: BTreeSet<ScopeEvidenceKind> = r.evidence_refs.iter().map(|e| e.kind).collect();
        for required in [
            ScopeEvidenceKind::QualificationRow,
            ScopeEvidenceKind::ClaimManifest,
        ] {
            if !kinds.contains(&required) {
                violations.push(ClaimScopeViolation::ReopenEvidenceUncovered {
                    entry_id: r.entry_id.clone(),
                    kind: required,
                });
            }
        }
        for evidence in &r.evidence_refs {
            // A present ref carries a location; only a missing one carries none.
            if evidence.state != M5ClaimReportState::Missing
                && evidence.evidence_ref.trim().is_empty()
            {
                violations.push(ClaimScopeViolation::EvidenceRefIncomplete {
                    entry_id: r.entry_id.clone(),
                });
            }
        }
    }

    fn validate_audiences(&self, r: &ClaimScopeRow, violations: &mut Vec<ClaimScopeViolation>) {
        let mut seen: BTreeSet<ClaimScopeAudience> = BTreeSet::new();
        let has_reasons = !r.active_scope_reasons.is_empty();
        let has_caveats = !r.scope_caveats.is_empty();
        for rendering in &r.audiences {
            if !seen.insert(rendering.audience) {
                violations.push(ClaimScopeViolation::DuplicateAudience {
                    entry_id: r.entry_id.clone(),
                    audience: rendering.audience,
                });
            }
            // Every audience must render from this one row, with the exact row label,
            // support class, and wording, so a narrowed row downgrades every audience
            // at once.
            if rendering.source_row_id != r.entry_id {
                violations.push(ClaimScopeViolation::AudienceSourceMismatch {
                    entry_id: r.entry_id.clone(),
                    audience: rendering.audience,
                });
            }
            if rendering.rendered_label != r.published_label {
                violations.push(ClaimScopeViolation::AudienceLabelDrift {
                    entry_id: r.entry_id.clone(),
                    audience: rendering.audience,
                    rendered: rendering.rendered_label,
                    published: r.published_label,
                });
            }
            if rendering.rendered_support_class != r.scope_support_class {
                violations.push(ClaimScopeViolation::AudienceSupportClassDrift {
                    entry_id: r.entry_id.clone(),
                    audience: rendering.audience,
                });
            }
            if rendering.rendered_claim_text != r.scope_claim_text {
                violations.push(ClaimScopeViolation::AudienceCopyDrift {
                    entry_id: r.entry_id.clone(),
                    audience: rendering.audience,
                });
            }
            if !rendering.discloses_freshness {
                violations.push(ClaimScopeViolation::AudienceFreshnessNotDisclosed {
                    entry_id: r.entry_id.clone(),
                    audience: rendering.audience,
                });
            }
            if has_reasons && !rendering.discloses_scope_reasons {
                violations.push(ClaimScopeViolation::AudienceReasonsNotDisclosed {
                    entry_id: r.entry_id.clone(),
                    audience: rendering.audience,
                });
            }
            if has_caveats && !rendering.discloses_caveats {
                violations.push(ClaimScopeViolation::AudienceCaveatsNotDisclosed {
                    entry_id: r.entry_id.clone(),
                    audience: rendering.audience,
                });
            }
            if rendering.audience.must_reopen_authoritative_row()
                && !rendering.reopens_authoritative_row
            {
                violations.push(ClaimScopeViolation::ReopenRefNotDisclosed {
                    entry_id: r.entry_id.clone(),
                    audience: rendering.audience,
                });
            }
        }
        for audience in ClaimScopeAudience::REQUIRED {
            if !seen.contains(&audience) {
                violations.push(ClaimScopeViolation::RequiredAudienceUncovered {
                    entry_id: r.entry_id.clone(),
                    audience,
                });
            }
        }
    }

    fn validate_published_row(&self, r: &ClaimScopeRow, violations: &mut Vec<ClaimScopeViolation>) {
        // A published row publishes exactly the public claim's label, that label is
        // at or above the cutline, it reuses the public wording verbatim, names no
        // active reason, rides a captured within-SLO packet, all evidence is current
        // inside an open validity window, it is owner-signed.
        if r.published_label != r.source_published_label {
            violations.push(ClaimScopeViolation::PublishedLabelNotSource {
                entry_id: r.entry_id.clone(),
                source: r.source_published_label,
                row: r.published_label,
            });
        }
        if !r.publishes_stable() {
            violations.push(ClaimScopeViolation::PublishedStateNotStable {
                entry_id: r.entry_id.clone(),
                published: r.published_label,
            });
        }
        if r.scope_claim_text != r.source_claim_text {
            violations.push(ClaimScopeViolation::PublishedCopyNotSource {
                entry_id: r.entry_id.clone(),
            });
        }
        if !r.active_scope_reasons.is_empty() {
            violations.push(ClaimScopeViolation::PublishedWithActiveGap {
                entry_id: r.entry_id.clone(),
            });
        }
        if !r.proof_packet.has_capture() {
            violations.push(ClaimScopeViolation::PublishedWithoutFreshPacket {
                entry_id: r.entry_id.clone(),
            });
        }
        if !r.proof_packet.slo_state.is_within_slo() {
            violations.push(ClaimScopeViolation::PublishedOnStalePacket {
                entry_id: r.entry_id.clone(),
                slo_state: r.proof_packet.slo_state,
            });
        }
        for evidence in &r.evidence_refs {
            if !evidence.state.is_current() {
                violations.push(ClaimScopeViolation::PublishedWithStaleEvidence {
                    entry_id: r.entry_id.clone(),
                    kind: evidence.kind,
                    state: evidence.state,
                });
            }
        }
        if r.validity_window.expired {
            violations.push(ClaimScopeViolation::PublishedWithExpiredWindow {
                entry_id: r.entry_id.clone(),
            });
        }
        if !(r.owner_signoff.signed_off && r.owner_signoff.signed_at.is_some()) {
            violations.push(ClaimScopeViolation::PublishedWithoutSignoff {
                entry_id: r.entry_id.clone(),
            });
        }
    }

    fn validate_narrowed_row(&self, r: &ClaimScopeRow, violations: &mut Vec<ClaimScopeViolation>) {
        // A narrowing row must drop below the cutline and name at least one reason.
        if r.publishes_stable() {
            violations.push(ClaimScopeViolation::NarrowedButPublishedStable {
                entry_id: r.entry_id.clone(),
                state: r.export_state,
                published: r.published_label,
            });
        }
        if r.active_scope_reasons.is_empty() {
            violations.push(ClaimScopeViolation::NarrowingWithoutReason {
                entry_id: r.entry_id.clone(),
                state: r.export_state,
            });
        }

        // A row narrowed below the public claim must carry its own copy-safe wording,
        // never the greener public claim text.
        if r.published_label.rank() < r.source_published_label.rank()
            && r.scope_claim_text == r.source_claim_text
        {
            violations.push(ClaimScopeViolation::NarrowedCopyReusesGreenerSource {
                entry_id: r.entry_id.clone(),
            });
        }

        // The narrowing state must be coherent with its active reasons.
        let any = |reasons: &[ClaimScopeReason]| reasons.iter().any(|r2| r.has_active_reason(*r2));
        let coherent = match r.export_state {
            ClaimScopeRowState::NarrowedRowDowngraded => any(&[ClaimScopeReason::RowDowngraded]),
            ClaimScopeRowState::NarrowedStale => any(&[
                ClaimScopeReason::EvidenceStale,
                ClaimScopeReason::QualificationStale,
            ]),
            ClaimScopeRowState::NarrowedRetestPending => any(&[ClaimScopeReason::RetestPending]),
            ClaimScopeRowState::Withheld => any(&[
                ClaimScopeReason::ValidityWindowExpired,
                ClaimScopeReason::OwnerSignoffMissing,
                ClaimScopeReason::WaiverExpired,
                ClaimScopeReason::ClaimPublicationMissing,
                ClaimScopeReason::EvidenceMissing,
            ]),
            ClaimScopeRowState::Published => true,
        };
        if !coherent {
            violations.push(ClaimScopeViolation::StateReasonIncoherent {
                entry_id: r.entry_id.clone(),
                state: r.export_state,
            });
        }

        // A stale or missing proof packet must name its matching reason.
        if r.proof_packet.slo_state == FreshnessSloState::Breached
            && !r.has_active_reason(ClaimScopeReason::EvidenceStale)
        {
            violations.push(ClaimScopeViolation::StateWithoutReason {
                entry_id: r.entry_id.clone(),
                reason: ClaimScopeReason::EvidenceStale,
            });
        }
        if r.proof_packet.slo_state == FreshnessSloState::Missing
            && !r.has_active_reason(ClaimScopeReason::EvidenceMissing)
        {
            violations.push(ClaimScopeViolation::StateWithoutReason {
                entry_id: r.entry_id.clone(),
                reason: ClaimScopeReason::EvidenceMissing,
            });
        }
        // An expired validity window must name its reason.
        if r.validity_window.expired
            && !r.has_active_reason(ClaimScopeReason::ValidityWindowExpired)
        {
            violations.push(ClaimScopeViolation::StateWithoutReason {
                entry_id: r.entry_id.clone(),
                reason: ClaimScopeReason::ValidityWindowExpired,
            });
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<ClaimScopeViolation>) {
        let covered: BTreeSet<String> = self.rows.iter().map(|r| r.family_ref.clone()).collect();
        for declared in &self.release_blocking_family_refs {
            if !covered.contains(declared) {
                violations.push(ClaimScopeViolation::ReleaseBlockingFamilyUncovered {
                    family_ref: declared.clone(),
                });
            }
        }
        for r in &self.rows {
            if r.release_blocking && !self.release_blocking_family_refs.contains(&r.family_ref) {
                violations.push(ClaimScopeViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: r.entry_id.clone(),
                });
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<ClaimScopeViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(ClaimScopeViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(ClaimScopeViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(ClaimScopeViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(ClaimScopeViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_claim_ids() {
            violations.push(ClaimScopeViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the M5 claim-scope export-packet register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimScopeViolation {
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
    ScopeReasonWithoutStopRule {
        /// Uncovered reason.
        reason: ClaimScopeReason,
    },
    /// A row carries no backing evidence ref.
    RowWithoutEvidence {
        /// Row id.
        entry_id: String,
    },
    /// A row does not point at a required reopen record (qualification row / claim).
    ReopenEvidenceUncovered {
        /// Row id.
        entry_id: String,
        /// Uncovered evidence kind.
        kind: ScopeEvidenceKind,
    },
    /// An evidence ref is incomplete.
    EvidenceRefIncomplete {
        /// Row id.
        entry_id: String,
    },
    /// A row drives the same audience twice.
    DuplicateAudience {
        /// Row id.
        entry_id: String,
        /// Duplicated audience.
        audience: ClaimScopeAudience,
    },
    /// A row does not drive a required reviewing audience.
    RequiredAudienceUncovered {
        /// Row id.
        entry_id: String,
        /// Uncovered audience.
        audience: ClaimScopeAudience,
    },
    /// An audience renders from a different row id.
    AudienceSourceMismatch {
        /// Row id.
        entry_id: String,
        /// Offending audience.
        audience: ClaimScopeAudience,
    },
    /// An audience renders a label that differs from the row's.
    AudienceLabelDrift {
        /// Row id.
        entry_id: String,
        /// Offending audience.
        audience: ClaimScopeAudience,
        /// Label the audience rendered.
        rendered: StableClaimLevel,
        /// Label the row publishes.
        published: StableClaimLevel,
    },
    /// An audience renders a support class that differs from the row's.
    AudienceSupportClassDrift {
        /// Row id.
        entry_id: String,
        /// Offending audience.
        audience: ClaimScopeAudience,
    },
    /// An audience renders wording that drifted from the row's.
    AudienceCopyDrift {
        /// Row id.
        entry_id: String,
        /// Offending audience.
        audience: ClaimScopeAudience,
    },
    /// An audience does not disclose the row freshness.
    AudienceFreshnessNotDisclosed {
        /// Row id.
        entry_id: String,
        /// Offending audience.
        audience: ClaimScopeAudience,
    },
    /// An audience carries active reasons it does not disclose.
    AudienceReasonsNotDisclosed {
        /// Row id.
        entry_id: String,
        /// Offending audience.
        audience: ClaimScopeAudience,
    },
    /// An audience carries caveats it does not disclose.
    AudienceCaveatsNotDisclosed {
        /// Row id.
        entry_id: String,
        /// Offending audience.
        audience: ClaimScopeAudience,
    },
    /// A shiproom audience does not expose the reopen refs.
    ReopenRefNotDisclosed {
        /// Row id.
        entry_id: String,
        /// Offending audience.
        audience: ClaimScopeAudience,
    },
    /// A limited support class records no scope caveat.
    LimitedWithoutCaveat {
        /// Row id.
        entry_id: String,
    },
    /// The row's published label is wider than the public claim it reuses.
    RowLabelExceedsSource {
        /// Row id.
        entry_id: String,
        /// Public claim label.
        source: StableClaimLevel,
        /// Row published label.
        row: StableClaimLevel,
    },
    /// The row's support class is broader than the public claim it reuses.
    RowSupportClassExceedsSource {
        /// Row id.
        entry_id: String,
        /// Public support class.
        source: SupportClass,
        /// Row support class.
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
        reason: ClaimScopeReason,
    },
    /// A published row does not publish the public claim's label.
    PublishedLabelNotSource {
        /// Row id.
        entry_id: String,
        /// Public claim label.
        source: StableClaimLevel,
        /// Row published label.
        row: StableClaimLevel,
    },
    /// A published row does not publish at or above the cutline.
    PublishedStateNotStable {
        /// Row id.
        entry_id: String,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A published row does not reuse the public claim wording verbatim.
    PublishedCopyNotSource {
        /// Row id.
        entry_id: String,
    },
    /// A published row carries active narrowing reasons.
    PublishedWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A published row has no captured proof packet.
    PublishedWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A published row rides a packet outside its freshness SLO.
    PublishedOnStalePacket {
        /// Row id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A published row rides a non-current evidence ref.
    PublishedWithStaleEvidence {
        /// Row id.
        entry_id: String,
        /// Evidence kind.
        kind: ScopeEvidenceKind,
        /// Evidence state.
        state: M5ClaimReportState,
    },
    /// A published row rides an expired validity window.
    PublishedWithExpiredWindow {
        /// Row id.
        entry_id: String,
    },
    /// A published row lacks owner sign-off.
    PublishedWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing row did not drop below the cutline.
    NarrowedButPublishedStable {
        /// Row id.
        entry_id: String,
        /// Export state.
        state: ClaimScopeRowState,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A narrowing row names no active reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// Export state.
        state: ClaimScopeRowState,
    },
    /// A row narrowed below the public claim reuses the greener public claim wording.
    NarrowedCopyReusesGreenerSource {
        /// Row id.
        entry_id: String,
    },
    /// An export state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Export state.
        state: ClaimScopeRowState,
    },
    /// A stale/missing/expired input does not name its reason.
    StateWithoutReason {
        /// Row id.
        entry_id: String,
        /// Reason the input state requires.
        reason: ClaimScopeReason,
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

impl fmt::Display for ClaimScopeViolation {
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
            Self::ScopeReasonWithoutStopRule { reason } => write!(
                f,
                "scope reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::RowWithoutEvidence { entry_id } => {
                write!(f, "row {entry_id} has no backing evidence ref")
            }
            Self::ReopenEvidenceUncovered { entry_id, kind } => write!(
                f,
                "row {entry_id} does not point at required reopen record {}",
                kind.as_str()
            ),
            Self::EvidenceRefIncomplete { entry_id } => {
                write!(f, "row {entry_id} has an incomplete evidence ref")
            }
            Self::DuplicateAudience { entry_id, audience } => write!(
                f,
                "row {entry_id} drives audience {} twice",
                audience.as_str()
            ),
            Self::RequiredAudienceUncovered { entry_id, audience } => write!(
                f,
                "row {entry_id} does not drive required audience {}",
                audience.as_str()
            ),
            Self::AudienceSourceMismatch { entry_id, audience } => write!(
                f,
                "row {entry_id} audience {} renders from a different row id",
                audience.as_str()
            ),
            Self::AudienceLabelDrift {
                entry_id,
                audience,
                rendered,
                published,
            } => write!(
                f,
                "row {entry_id} audience {} rendered {rendered:?} but row publishes {published:?}",
                audience.as_str()
            ),
            Self::AudienceSupportClassDrift { entry_id, audience } => write!(
                f,
                "row {entry_id} audience {} support class drifted from the row",
                audience.as_str()
            ),
            Self::AudienceCopyDrift { entry_id, audience } => write!(
                f,
                "row {entry_id} audience {} wording drifted from the row",
                audience.as_str()
            ),
            Self::AudienceFreshnessNotDisclosed { entry_id, audience } => write!(
                f,
                "row {entry_id} audience {} does not disclose freshness",
                audience.as_str()
            ),
            Self::AudienceReasonsNotDisclosed { entry_id, audience } => write!(
                f,
                "row {entry_id} audience {} does not disclose its active scope reasons",
                audience.as_str()
            ),
            Self::AudienceCaveatsNotDisclosed { entry_id, audience } => write!(
                f,
                "row {entry_id} audience {} does not disclose its caveats",
                audience.as_str()
            ),
            Self::ReopenRefNotDisclosed { entry_id, audience } => write!(
                f,
                "row {entry_id} audience {} does not expose the reopen refs",
                audience.as_str()
            ),
            Self::LimitedWithoutCaveat { entry_id } => {
                write!(f, "row {entry_id} is limited without a scope caveat")
            }
            Self::RowLabelExceedsSource {
                entry_id,
                source,
                row,
            } => write!(
                f,
                "row {entry_id} published {row:?} is greener than the public claim {source:?}"
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
            Self::PublishedLabelNotSource {
                entry_id,
                source,
                row,
            } => write!(
                f,
                "row {entry_id} published label {row:?} does not equal public claim {source:?}"
            ),
            Self::PublishedStateNotStable {
                entry_id,
                published,
            } => write!(
                f,
                "row {entry_id} is published but publishes {published:?} below the cutline"
            ),
            Self::PublishedCopyNotSource { entry_id } => write!(
                f,
                "row {entry_id} publishes wording that does not reuse the public claim verbatim"
            ),
            Self::PublishedWithActiveGap { entry_id } => {
                write!(f, "row {entry_id} publishes with an active gap")
            }
            Self::PublishedWithoutFreshPacket { entry_id } => {
                write!(f, "row {entry_id} publishes without a fresh packet")
            }
            Self::PublishedOnStalePacket {
                entry_id,
                slo_state,
            } => write!(f, "row {entry_id} publishes on stale packet {slo_state:?}"),
            Self::PublishedWithStaleEvidence {
                entry_id,
                kind,
                state,
            } => write!(
                f,
                "row {entry_id} publishes on evidence {} in state {}",
                kind.as_str(),
                state.as_str()
            ),
            Self::PublishedWithExpiredWindow { entry_id } => {
                write!(f, "row {entry_id} publishes on an expired validity window")
            }
            Self::PublishedWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} publishes without owner signoff")
            }
            Self::NarrowedButPublishedStable {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "row {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "row {entry_id} state {state:?} narrows without active reason"
            ),
            Self::NarrowedCopyReusesGreenerSource { entry_id } => write!(
                f,
                "row {entry_id} narrowed below the public claim but reuses the greener wording"
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

impl Error for ClaimScopeViolation {}

/// Loads the embedded M5 claim-scope export-packet register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`ClaimScopeExportRegister`].
pub fn current_m5_claim_scope_export_packets() -> Result<ClaimScopeExportRegister, serde_json::Error>
{
    serde_json::from_str(M5_CLAIM_SCOPE_EXPORT_PACKETS_JSON)
}

#[cfg(test)]
mod tests;
