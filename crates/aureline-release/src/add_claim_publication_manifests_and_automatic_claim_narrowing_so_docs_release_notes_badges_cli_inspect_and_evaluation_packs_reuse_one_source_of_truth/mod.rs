//! Typed M5 claim-publication manifest register and automatic claim narrowing.
//!
//! Where the qualification/skew matrix freezes the *machine-readable
//! qualification row* every M5 stable-facing family must hold, and the badge
//! binding register advertises that row on marketable badges, this register is
//! the *single source of truth* every claim-bearing surface reads. For each
//! claimed family it binds one [`M5ClaimPublication`] manifest:
//!
//! - the exact marketable wording, support class, scope caveats, and validity
//!   window the family publishes ([`M5PublishedClaim`]),
//! - the backing report refs — a reference-workspace report, a compatibility
//!   report, and an evaluation report ([`M5ClaimReportRef`]),
//! - and the closed set of consuming destinations ([`M5ClaimDestination`]) the
//!   manifest *drives*: website/docs, release notes, the in-product badge, CLI
//!   inspect, the evaluation pack, and admin export — each recorded as a
//!   [`M5ClaimDestinationRendering`] that must read the same manifest id, the
//!   same published label, the same support class, and the same exact wording.
//!
//! Because every destination renders from the one manifest, there is no
//! hand-maintained copy to drift, and a narrowed manifest downgrades every
//! consuming surface at once: the [`M5ClaimPublication::published_label`] may
//! never be wider than the row's
//! [`M5ClaimPublication::row_published_label`], which in turn may never be wider
//! than the canonical [`M5ClaimPublication::claim_label`]. A manifest that merely
//! inherits an upstream qualification-row narrowing downgrades its surfaces but
//! does not itself hold promotion — the qualification matrix already gates that —
//! while a *manifest-layer* failure (stale, missing, dropped, or unsigned backing
//! evidence, an expired validity window, an over-claiming wording, a missing
//! owner sign-off, or an expired waiver) holds promotion through a
//! [`M5ClaimPublicationStopRule`].
//!
//! This register reuses the canonical [`FamilyKind`] and [`SupportClass`]
//! vocabularies from the qualification/skew matrix, the [`ProofPacket`] and
//! [`FreshnessSloState`] freshness vocabulary from the stable claim manifest, and
//! the [`LaunchCutline`], [`StableClaimLevel`], [`OwnerSignoff`],
//! [`QualificationWaiver`], [`PromotionDecision`], and [`PromotionDecisionRecord`]
//! types from the stable claim matrix rather than minting local synonyms.
//!
//! The register is checked in at [`M5_CLAIM_PUBLICATION_MANIFESTS_PATH`] and
//! embedded here, so this typed consumer and the CI gate agree on every manifest
//! without a cargo build in CI. The model is metadata-only: every field is a typed
//! state or an opaque ref. It carries no raw artifacts, raw logs, signatures, or
//! credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix::{
    FamilyKind, SupportClass,
};
use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported register schema version.
pub const M5_CLAIM_PUBLICATION_MANIFESTS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_CLAIM_PUBLICATION_MANIFESTS_RECORD_KIND: &str =
    "add_m5_claim_publication_manifests_and_automatic_claim_narrowing";

/// Repo-relative path to the checked-in register.
pub const M5_CLAIM_PUBLICATION_MANIFESTS_PATH: &str =
    "artifacts/release/m5/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json";

/// Embedded checked-in register JSON.
pub const M5_CLAIM_PUBLICATION_MANIFESTS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth.json"
));

/// One backing report a claim manifest rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClaimReportKind {
    /// The certified reference-workspace report for the family.
    ReferenceWorkspaceReport,
    /// The family compatibility report.
    CompatibilityReport,
    /// The enterprise evaluation report.
    EvaluationReport,
}

impl M5ClaimReportKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ReferenceWorkspaceReport,
        Self::CompatibilityReport,
        Self::EvaluationReport,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReferenceWorkspaceReport => "reference_workspace_report",
            Self::CompatibilityReport => "compatibility_report",
            Self::EvaluationReport => "evaluation_report",
        }
    }
}

/// Freshness/integrity state of a backing report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClaimReportState {
    /// Current and signed; claim-bearing.
    Current,
    /// Exists but has gone stale.
    Stale,
    /// Has not been produced.
    Missing,
    /// Was produced but has since been dropped or revoked.
    Dropped,
    /// Exists but is not signed, so it is not claim-bearing.
    Unsigned,
}

impl M5ClaimReportState {
    /// Every state, freshest to least usable.
    pub const ALL: [Self; 5] = [
        Self::Current,
        Self::Stale,
        Self::Missing,
        Self::Dropped,
        Self::Unsigned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::Dropped => "dropped",
            Self::Unsigned => "unsigned",
        }
    }

    /// Whether a published claim may ride this report.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// A surface that consumes a published claim from the manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClaimDestination {
    /// The public website / product documentation.
    WebsiteDocs,
    /// Release notes.
    ReleaseNotes,
    /// The in-product support-class badge.
    InProductBadge,
    /// CLI/headless inspect output.
    CliInspect,
    /// The enterprise evaluation pack.
    EvaluationPack,
    /// The admin/support export.
    AdminExport,
    /// The Help/About surface.
    HelpAbout,
    /// The service-health surface.
    ServiceHealth,
    /// The support-export surface.
    SupportExport,
}

impl M5ClaimDestination {
    /// Every destination, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::WebsiteDocs,
        Self::ReleaseNotes,
        Self::InProductBadge,
        Self::CliInspect,
        Self::EvaluationPack,
        Self::AdminExport,
        Self::HelpAbout,
        Self::ServiceHealth,
        Self::SupportExport,
    ];

    /// The destinations every claim manifest must drive, so docs, release notes,
    /// badges, CLI inspect, evaluation packs, and admin export all read one
    /// source of truth.
    pub const REQUIRED: [Self; 6] = [
        Self::WebsiteDocs,
        Self::ReleaseNotes,
        Self::InProductBadge,
        Self::CliInspect,
        Self::EvaluationPack,
        Self::AdminExport,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WebsiteDocs => "website_docs",
            Self::ReleaseNotes => "release_notes",
            Self::InProductBadge => "in_product_badge",
            Self::CliInspect => "cli_inspect",
            Self::EvaluationPack => "evaluation_pack",
            Self::AdminExport => "admin_export",
            Self::HelpAbout => "help_about",
            Self::ServiceHealth => "service_health",
            Self::SupportExport => "support_export",
        }
    }
}

/// Overall state a claim manifest earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClaimManifestState {
    /// The manifest publishes the row's label; all backing evidence is current.
    Published,
    /// The manifest inherited an upstream qualification-row narrowing.
    NarrowedRowDowngraded,
    /// A backing report or the proof packet is stale; the claim narrows.
    NarrowedStale,
    /// A backing report or the proof packet is missing; the claim narrows.
    NarrowedMissing,
    /// The claim is withheld entirely (over-claim, dropped/unsigned report,
    /// expired window or waiver, or missing sign-off).
    Withheld,
}

impl M5ClaimManifestState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Published,
        Self::NarrowedRowDowngraded,
        Self::NarrowedStale,
        Self::NarrowedMissing,
        Self::Withheld,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::NarrowedRowDowngraded => "narrowed_row_downgraded",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedMissing => "narrowed_missing",
            Self::Withheld => "withheld",
        }
    }

    /// Whether the state lets the manifest publish the row's label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Published)
    }
}

/// Closed reason a claim narrows below the row it binds or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClaimNarrowingReason {
    /// The upstream qualification row narrowed below the cutline.
    QualificationRowNarrowed,
    /// The manifest proof packet breached its freshness SLO.
    EvidenceStale,
    /// No manifest proof packet has been captured.
    EvidenceMissing,
    /// A backing report is stale.
    ReportStale,
    /// A backing report is missing.
    ReportMissing,
    /// A backing report was dropped or revoked.
    ReportDropped,
    /// A backing report is unsigned.
    ReportUnsigned,
    /// The published claim's validity window has expired.
    ValidityWindowExpired,
    /// The published wording would advertise wider than the qualification row.
    OverClaimBeyondRow,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
    /// A waiver the claim relied on has expired.
    WaiverExpired,
}

impl M5ClaimNarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::QualificationRowNarrowed,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::ReportStale,
        Self::ReportMissing,
        Self::ReportDropped,
        Self::ReportUnsigned,
        Self::ValidityWindowExpired,
        Self::OverClaimBeyondRow,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualificationRowNarrowed => "qualification_row_narrowed",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceMissing => "evidence_missing",
            Self::ReportStale => "report_stale",
            Self::ReportMissing => "report_missing",
            Self::ReportDropped => "report_dropped",
            Self::ReportUnsigned => "report_unsigned",
            Self::ValidityWindowExpired => "validity_window_expired",
            Self::OverClaimBeyondRow => "over_claim_beyond_row",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether a manifest whose claim is at or above the cutline carrying this
    /// reason holds promotion. A reason that merely inherits an upstream
    /// qualification-row narrowing is gated by the matrix, not this register.
    pub const fn blocks_promotion(self) -> bool {
        !matches!(self, Self::QualificationRowNarrowed)
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClaimStopAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the claim to inherit the row.
    NarrowClaim,
    /// Withhold the claim entirely.
    WithholdClaim,
    /// Refresh the backing report.
    RefreshReport,
    /// Refresh the manifest evidence packet.
    RefreshEvidence,
    /// Align the published wording to the current row.
    AlignCopyToRow,
    /// Renew the claim validity window.
    RenewValidityWindow,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl M5ClaimStopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::HoldPublication,
        Self::NarrowClaim,
        Self::WithholdClaim,
        Self::RefreshReport,
        Self::RefreshEvidence,
        Self::AlignCopyToRow,
        Self::RenewValidityWindow,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowClaim => "narrow_claim",
            Self::WithholdClaim => "withhold_claim",
            Self::RefreshReport => "refresh_report",
            Self::RefreshEvidence => "refresh_evidence",
            Self::AlignCopyToRow => "align_copy_to_row",
            Self::RenewValidityWindow => "renew_validity_window",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// The validity window the published claim is asserted within.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ClaimValidityWindow {
    /// UTC date the published claim becomes valid.
    pub starts_at: String,
    /// UTC date the published claim expires and must be renewed.
    pub expires_at: String,
    /// Whether the window has expired as of the register's `as_of` date.
    pub expired: bool,
}

/// One backing report a claim manifest rests on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ClaimReportRef {
    /// The kind of report this ref names.
    pub report_kind: M5ClaimReportKind,
    /// Ref to the report. Empty only when the state is `missing`.
    pub report_ref: String,
    /// The report's freshness/integrity state.
    pub state: M5ClaimReportState,
    /// UTC date the report was produced, or null when missing.
    #[serde(default)]
    pub captured_at: Option<String>,
}

/// The exact marketable claim every destination renders from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublishedClaim {
    /// The exact copy-safe wording the family publishes. Never wider than the row.
    pub claim_text: String,
    /// The support class the claim advertises.
    pub support_class: SupportClass,
    /// Scope caveats that travel with the claim. Non-empty when support is limited.
    #[serde(default)]
    pub scope_caveats: Vec<String>,
    /// The validity window the claim is asserted within.
    pub validity_window: M5ClaimValidityWindow,
}

/// One consuming destination's rendering of a published claim.
///
/// Each rendering reads the manifest id, the published label, the support class,
/// and the exact wording from the one manifest, so there is no hand-maintained
/// copy to drift and a narrowed manifest downgrades every destination at once.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ClaimDestinationRendering {
    /// The destination this rendering targets.
    pub destination: M5ClaimDestination,
    /// The manifest id this destination renders from. Equals the register id.
    pub source_manifest_id: String,
    /// The label rendered. Equals the manifest's published label.
    pub rendered_label: StableClaimLevel,
    /// The support class rendered. Equals the published claim's support class.
    pub rendered_support_class: SupportClass,
    /// The exact wording rendered. Equals the published claim's text.
    pub rendered_claim_text: String,
    /// Whether the destination discloses the manifest freshness. Always required.
    pub discloses_freshness: bool,
    /// Whether the destination discloses the scope caveats. Required when any exist.
    pub discloses_caveats: bool,
}

/// One claim-publication stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ClaimPublicationStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched manifest fires this rule.
    pub trigger_reason: M5ClaimNarrowingReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: M5ClaimStopAction,
    /// Whether firing this rule holds promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One claim-publication manifest: the single source of truth for one family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ClaimPublication {
    /// Stable manifest id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The family this manifest governs.
    pub family_kind: FamilyKind,
    /// The family ref this manifest speaks about.
    pub family_ref: String,
    /// Reviewable one-line statement of the family.
    pub family_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim entry id whose claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes (the hard ceiling).
    pub claim_label: StableClaimLevel,
    /// The qualification-row entry id this manifest joins to.
    pub qualification_row_ref: String,
    /// The label the upstream qualification row publishes (the claim ceiling).
    pub row_published_label: StableClaimLevel,
    /// Overall manifest state earned.
    pub manifest_state: M5ClaimManifestState,
    /// The exact published claim every destination renders from.
    pub published_claim: M5PublishedClaim,
    /// The backing reference-workspace report.
    pub reference_workspace_report: M5ClaimReportRef,
    /// The backing compatibility report.
    pub compatibility_report: M5ClaimReportRef,
    /// The backing evaluation report.
    pub evaluation_report: M5ClaimReportRef,
    /// The consuming destinations the manifest drives. Always covers the required
    /// set (docs, release notes, badge, CLI inspect, evaluation pack, admin export).
    pub destinations: Vec<M5ClaimDestinationRendering>,
    /// The manifest proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the claim below the row's label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<M5ClaimNarrowingReason>,
    /// The lifecycle label the claim effectively publishes after narrowing.
    pub published_label: StableClaimLevel,
    /// Reviewable reason the manifest carries this posture.
    pub rationale: String,
}

impl M5ClaimPublication {
    /// True when the published claim label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the manifest state lets the claim carry the row's label.
    pub fn holds_label(&self) -> bool {
        self.manifest_state.holds_label()
    }

    /// True when a narrowing reason is active on the manifest.
    pub fn has_active_reason(&self, reason: M5ClaimNarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// The three backing reports, in canonical order.
    pub fn backing_reports(&self) -> [&M5ClaimReportRef; 3] {
        [
            &self.reference_workspace_report,
            &self.compatibility_report,
            &self.evaluation_report,
        ]
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ClaimPublicationSummary {
    /// Total number of manifests.
    pub total_manifests: usize,
    /// Distinct families covered.
    pub total_families: usize,
    /// Manifests publishing a claim at or above the cutline.
    pub manifests_published: usize,
    /// Manifests whose claim narrowed below the cutline.
    pub manifests_narrowed: usize,
    /// Total release-blocking manifests.
    pub release_blocking_total: usize,
    /// Release-blocking manifests publishing at or above the cutline.
    pub release_blocking_published: usize,
    /// Release-blocking manifests narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook manifests.
    pub notebook_manifests: usize,
    /// AI/provider manifests.
    pub ai_provider_manifests: usize,
    /// Remote/helper manifests.
    pub remote_helper_manifests: usize,
    /// Companion manifests.
    pub companion_manifests: usize,
    /// Ecosystem manifests.
    pub ecosystem_manifests: usize,
    /// Managed-service manifests.
    pub managed_service_manifests: usize,
    /// Toolchain/runtime manifests.
    pub toolchain_runtime_manifests: usize,
    /// Manifests in the `published` state.
    pub state_published: usize,
    /// Manifests in the `narrowed_row_downgraded` state.
    pub state_narrowed_row_downgraded: usize,
    /// Manifests in the `narrowed_stale` state.
    pub state_narrowed_stale: usize,
    /// Manifests in the `narrowed_missing` state.
    pub state_narrowed_missing: usize,
    /// Manifests in the `withheld` state.
    pub state_withheld: usize,
    /// Manifests carrying at least one scope caveat.
    pub claims_with_caveats: usize,
    /// Total consuming destination renderings across all manifests.
    pub total_destinations: usize,
    /// Destination renderings that disclose the manifest freshness.
    pub destinations_freshness_disclosed: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Backing reports that are current.
    pub reports_current: usize,
    /// Backing reports that are stale.
    pub reports_stale: usize,
    /// Backing reports that are missing.
    pub reports_missing: usize,
    /// Backing reports that are dropped.
    pub reports_dropped: usize,
    /// Backing reports that are unsigned.
    pub reports_unsigned: usize,
    /// Total active narrowing reasons across all manifests.
    pub total_active_narrowing_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ClaimPublicationExportRow {
    /// Stable manifest id.
    pub entry_id: String,
    /// The family this manifest governs.
    pub family_kind: FamilyKind,
    /// The family ref this manifest speaks about.
    pub family_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The qualification-row entry id this manifest joins to.
    pub qualification_row_ref: String,
    /// The canonical claim label.
    pub claim_label: StableClaimLevel,
    /// The upstream row's published label.
    pub row_published_label: StableClaimLevel,
    /// The claim's effective published label.
    pub published_label: StableClaimLevel,
    /// Whether the claim publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall manifest state earned.
    pub manifest_state: M5ClaimManifestState,
    /// The support class the claim advertises.
    pub support_class: SupportClass,
    /// The exact published wording every destination renders.
    pub claim_text: String,
    /// The disclosed freshness state.
    pub freshness_state: FreshnessSloState,
    /// The scope caveats that travel with the claim.
    pub scope_caveats: Vec<String>,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<M5ClaimNarrowingReason>,
    /// Number of consuming destinations driven from this manifest.
    pub destination_count: usize,
}

/// Export projection for docs, release notes, badges, CLI inspect, evaluation
/// packs, and admin/support export surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ClaimPublicationExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5ClaimPublicationExportRow>,
}

/// The typed M5 claim-publication manifest register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ClaimPublicationRegister {
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
    /// Ref to the stable claim manifest this register ingests.
    pub claim_manifest_ref: String,
    /// Ref to the qualification/skew matrix whose rows this register binds.
    pub qualification_matrix_ref: String,
    /// Ref to the canonical M5 evidence index this register is recorded under.
    pub evidence_index_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed family-kind vocabulary.
    pub family_kinds: Vec<FamilyKind>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed report-kind vocabulary.
    pub report_kinds: Vec<M5ClaimReportKind>,
    /// Closed report-state vocabulary.
    pub report_states: Vec<M5ClaimReportState>,
    /// Closed destination-kind vocabulary.
    pub destination_kinds: Vec<M5ClaimDestination>,
    /// The required consuming destinations every manifest must drive.
    pub required_destinations: Vec<M5ClaimDestination>,
    /// Closed manifest-state vocabulary.
    pub manifest_states: Vec<M5ClaimManifestState>,
    /// Closed freshness-state vocabulary.
    pub freshness_states: Vec<FreshnessSloState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<M5ClaimNarrowingReason>,
    /// Closed stop-action vocabulary.
    pub stop_actions: Vec<M5ClaimStopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking family refs this register must cover.
    pub release_blocking_family_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<M5ClaimPublicationStopRule>,
    /// Claim-publication manifests.
    pub manifests: Vec<M5ClaimPublication>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5ClaimPublicationSummary,
}

impl M5ClaimPublicationRegister {
    /// Returns the manifest registered for `entry_id`.
    pub fn manifest(&self, entry_id: &str) -> Option<&M5ClaimPublication> {
        self.manifests.iter().find(|m| m.entry_id == entry_id)
    }

    /// Returns the manifests publishing a claim at or above the cutline.
    pub fn manifests_published(&self) -> Vec<&M5ClaimPublication> {
        self.manifests
            .iter()
            .filter(|m| m.publishes_stable())
            .collect()
    }

    /// Returns the manifests whose claim narrowed below the cutline.
    pub fn manifests_narrowed(&self) -> Vec<&M5ClaimPublication> {
        self.manifests
            .iter()
            .filter(|m| !m.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking manifests.
    pub fn release_blocking_manifests(&self) -> Vec<&M5ClaimPublication> {
        self.manifests
            .iter()
            .filter(|m| m.release_blocking)
            .collect()
    }

    /// Returns the manifests for one family kind.
    pub fn manifests_for_kind(&self, kind: FamilyKind) -> Vec<&M5ClaimPublication> {
        self.manifests
            .iter()
            .filter(|m| m.family_kind == kind)
            .collect()
    }

    /// Distinct families (by family ref) the register covers.
    pub fn families(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for m in &self.manifests {
            set.insert(m.family_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched manifest carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &M5ClaimPublicationStopRule) -> bool {
        self.manifests.iter().any(|m| {
            rule.applies_to_labels.contains(&m.claim_label)
                && m.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the manifests and stop rules.
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

    /// Manifest ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only manifests whose claim is at or above the cutline count: a manifest
    /// whose claim is already canonically narrowed merely inherits the ceiling.
    pub fn computed_blocking_claim_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<M5ClaimNarrowingReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for m in &self.manifests {
            if m.claim_holds_stable()
                && m.active_narrowing_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(m.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Counts the backing reports across all manifests in `state`.
    fn reports_in(&self, state: M5ClaimReportState) -> usize {
        self.manifests
            .iter()
            .flat_map(|m| m.backing_reports())
            .filter(|r| r.state == state)
            .count()
    }

    /// Recomputes the summary block from the manifests and stop rules.
    pub fn computed_summary(&self) -> M5ClaimPublicationSummary {
        let kind = |kind: FamilyKind| self.manifests_for_kind(kind).len();
        let state = |state: M5ClaimManifestState| {
            self.manifests
                .iter()
                .filter(|m| m.manifest_state == state)
                .count()
        };
        let packets = |s: FreshnessSloState| {
            self.manifests
                .iter()
                .filter(|m| m.proof_packet.slo_state == s)
                .count()
        };
        let release_blocking: Vec<&M5ClaimPublication> = self.release_blocking_manifests();
        M5ClaimPublicationSummary {
            total_manifests: self.manifests.len(),
            total_families: self.families().len(),
            manifests_published: self.manifests_published().len(),
            manifests_narrowed: self.manifests_narrowed().len(),
            release_blocking_total: release_blocking.len(),
            release_blocking_published: release_blocking
                .iter()
                .filter(|m| m.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|m| !m.publishes_stable())
                .count(),
            notebook_manifests: kind(FamilyKind::Notebook),
            ai_provider_manifests: kind(FamilyKind::AiProvider),
            remote_helper_manifests: kind(FamilyKind::RemoteHelper),
            companion_manifests: kind(FamilyKind::Companion),
            ecosystem_manifests: kind(FamilyKind::Ecosystem),
            managed_service_manifests: kind(FamilyKind::ManagedService),
            toolchain_runtime_manifests: kind(FamilyKind::ToolchainRuntime),
            state_published: state(M5ClaimManifestState::Published),
            state_narrowed_row_downgraded: state(M5ClaimManifestState::NarrowedRowDowngraded),
            state_narrowed_stale: state(M5ClaimManifestState::NarrowedStale),
            state_narrowed_missing: state(M5ClaimManifestState::NarrowedMissing),
            state_withheld: state(M5ClaimManifestState::Withheld),
            claims_with_caveats: self
                .manifests
                .iter()
                .filter(|m| !m.published_claim.scope_caveats.is_empty())
                .count(),
            total_destinations: self.manifests.iter().map(|m| m.destinations.len()).sum(),
            destinations_freshness_disclosed: self
                .manifests
                .iter()
                .flat_map(|m| m.destinations.iter())
                .filter(|d| d.discloses_freshness)
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            reports_current: self.reports_in(M5ClaimReportState::Current),
            reports_stale: self.reports_in(M5ClaimReportState::Stale),
            reports_missing: self.reports_in(M5ClaimReportState::Missing),
            reports_dropped: self.reports_in(M5ClaimReportState::Dropped),
            reports_unsigned: self.reports_in(M5ClaimReportState::Unsigned),
            total_active_narrowing_reasons: self
                .manifests
                .iter()
                .map(|m| m.active_narrowing_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection that downstream surfaces
    /// render instead of cloning status text. The exact wording, freshness state,
    /// and caveats travel with every row, so docs, release notes, badges, CLI
    /// inspect, evaluation packs, and admin export read one source of truth.
    pub fn support_export_projection(&self) -> M5ClaimPublicationExportProjection {
        M5ClaimPublicationExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .manifests
                .iter()
                .map(|m| M5ClaimPublicationExportRow {
                    entry_id: m.entry_id.clone(),
                    family_kind: m.family_kind,
                    family_ref: m.family_ref.clone(),
                    release_blocking: m.release_blocking,
                    qualification_row_ref: m.qualification_row_ref.clone(),
                    claim_label: m.claim_label,
                    row_published_label: m.row_published_label,
                    published_label: m.published_label,
                    publishes_stable: m.publishes_stable(),
                    manifest_state: m.manifest_state,
                    support_class: m.published_claim.support_class,
                    claim_text: m.published_claim.claim_text.clone(),
                    freshness_state: m.proof_packet.slo_state,
                    scope_caveats: m.published_claim.scope_caveats.clone(),
                    active_narrowing_reasons: m.active_narrowing_reasons.clone(),
                    destination_count: m.destinations.len(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<M5ClaimPublicationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for m in &self.manifests {
            if !seen.insert(m.entry_id.clone()) {
                violations.push(M5ClaimPublicationViolation::DuplicateEntryId {
                    entry_id: m.entry_id.clone(),
                });
            }
            self.validate_manifest(m, &mut violations);
        }
        if self.manifests.is_empty() {
            violations.push(M5ClaimPublicationViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5ClaimPublicationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ClaimPublicationViolation>) {
        if self.schema_version != M5_CLAIM_PUBLICATION_MANIFESTS_SCHEMA_VERSION {
            violations.push(M5ClaimPublicationViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_CLAIM_PUBLICATION_MANIFESTS_RECORD_KIND {
            violations.push(M5ClaimPublicationViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("qualification_matrix_ref", &self.qualification_matrix_ref),
            ("evidence_index_ref", &self.evidence_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ClaimPublicationViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        let vocab: [(bool, &'static str); 10] = [
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
            (
                self.report_kinds == M5ClaimReportKind::ALL.to_vec(),
                "report_kinds",
            ),
            (
                self.report_states == M5ClaimReportState::ALL.to_vec(),
                "report_states",
            ),
            (
                self.destination_kinds == M5ClaimDestination::ALL.to_vec(),
                "destination_kinds",
            ),
            (
                self.required_destinations == M5ClaimDestination::REQUIRED.to_vec(),
                "required_destinations",
            ),
            (
                self.manifest_states == M5ClaimManifestState::ALL.to_vec(),
                "manifest_states",
            ),
            (
                self.freshness_states == FreshnessSloState::ALL.to_vec(),
                "freshness_states",
            ),
            (
                self.narrowing_reasons == M5ClaimNarrowingReason::ALL.to_vec(),
                "narrowing_reasons",
            ),
        ];
        for (ok, field) in vocab {
            if !ok {
                violations.push(M5ClaimPublicationViolation::ClosedVocabularyMismatch { field });
            }
        }
        if self.stop_actions != M5ClaimStopAction::ALL.to_vec() {
            violations.push(M5ClaimPublicationViolation::ClosedVocabularyMismatch {
                field: "stop_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5ClaimPublicationViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5ClaimPublicationViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5ClaimPublicationViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5ClaimPublicationViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5ClaimPublicationViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5ClaimPublicationViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5ClaimPublicationViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ClaimPublicationViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5ClaimPublicationViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            if rule.blocks_promotion != rule.trigger_reason.blocks_promotion() {
                violations.push(M5ClaimPublicationViolation::StopRuleBlockingMismatch {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in M5ClaimNarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(M5ClaimPublicationViolation::NarrowingReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_manifest(
        &self,
        m: &M5ClaimPublication,
        violations: &mut Vec<M5ClaimPublicationViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &m.entry_id),
            ("title", &m.title),
            ("family_ref", &m.family_ref),
            ("family_summary", &m.family_summary),
            ("claim_ref", &m.claim_ref),
            ("qualification_row_ref", &m.qualification_row_ref),
            ("published_claim.claim_text", &m.published_claim.claim_text),
            (
                "published_claim.validity_window.starts_at",
                &m.published_claim.validity_window.starts_at,
            ),
            (
                "published_claim.validity_window.expires_at",
                &m.published_claim.validity_window.expires_at,
            ),
            ("proof_packet.packet_id", &m.proof_packet.packet_id),
            ("proof_packet.packet_ref", &m.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &m.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &m.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &m.owner_signoff.owner_ref),
            ("rationale", &m.rationale),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ClaimPublicationViolation::EmptyField {
                    entry_id: m.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_reports(m, violations);
        self.validate_destinations(m, violations);

        // The ceilings: the row may not exceed the claim, and the published claim
        // may not exceed the row.
        if m.row_published_label.rank() > m.claim_label.rank() {
            violations.push(M5ClaimPublicationViolation::RowWiderThanClaim {
                entry_id: m.entry_id.clone(),
                claim: m.claim_label,
                row: m.row_published_label,
            });
        }
        if m.published_label.rank() > m.row_published_label.rank() {
            violations.push(M5ClaimPublicationViolation::ClaimPublishedWiderThanRow {
                entry_id: m.entry_id.clone(),
                row: m.row_published_label,
                published: m.published_label,
            });
        }

        if m.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5ClaimPublicationViolation::EmptyField {
                entry_id: m.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !m.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5ClaimPublicationViolation::FreshnessSloInconsistent {
                entry_id: m.entry_id.clone(),
            });
        }

        // A row narrowed below the cutline must name the inherited reason.
        if !m.row_published_label.is_at_or_above_cutline()
            && !m.has_active_reason(M5ClaimNarrowingReason::QualificationRowNarrowed)
        {
            violations.push(M5ClaimPublicationViolation::RowNarrowedWithoutReason {
                entry_id: m.entry_id.clone(),
            });
        }

        // A limited support class must record at least one scope caveat.
        if m.published_claim.support_class == SupportClass::Limited
            && m.published_claim
                .scope_caveats
                .iter()
                .all(|c| c.trim().is_empty())
        {
            violations.push(M5ClaimPublicationViolation::LimitedWithoutCaveat {
                entry_id: m.entry_id.clone(),
            });
        }

        if m.holds_label() {
            self.validate_held_manifest(m, violations);
        } else {
            self.validate_narrowed_manifest(m, violations);
        }
    }

    fn validate_reports(
        &self,
        m: &M5ClaimPublication,
        violations: &mut Vec<M5ClaimPublicationViolation>,
    ) {
        for (report, expected) in [
            (
                &m.reference_workspace_report,
                M5ClaimReportKind::ReferenceWorkspaceReport,
            ),
            (
                &m.compatibility_report,
                M5ClaimReportKind::CompatibilityReport,
            ),
            (&m.evaluation_report, M5ClaimReportKind::EvaluationReport),
        ] {
            if report.report_kind != expected {
                violations.push(M5ClaimPublicationViolation::ReportKindMismatch {
                    entry_id: m.entry_id.clone(),
                    expected,
                    actual: report.report_kind,
                });
            }
            // A present report carries a ref; only a missing one carries none.
            if report.state != M5ClaimReportState::Missing && report.report_ref.trim().is_empty() {
                violations.push(M5ClaimPublicationViolation::ReportRefMissing {
                    entry_id: m.entry_id.clone(),
                    kind: expected,
                });
            }
        }
    }

    fn validate_destinations(
        &self,
        m: &M5ClaimPublication,
        violations: &mut Vec<M5ClaimPublicationViolation>,
    ) {
        let mut seen: BTreeSet<M5ClaimDestination> = BTreeSet::new();
        for rendering in &m.destinations {
            if !seen.insert(rendering.destination) {
                violations.push(M5ClaimPublicationViolation::DuplicateDestination {
                    entry_id: m.entry_id.clone(),
                    destination: rendering.destination,
                });
            }
            // Every destination must render from this one manifest, with the exact
            // published label, support class, and wording — so there is no
            // hand-maintained copy to drift and a narrowed manifest downgrades
            // every surface at once.
            if rendering.source_manifest_id != self.register_id {
                violations.push(M5ClaimPublicationViolation::DestinationSourceMismatch {
                    entry_id: m.entry_id.clone(),
                    destination: rendering.destination,
                });
            }
            if rendering.rendered_label != m.published_label {
                violations.push(M5ClaimPublicationViolation::DestinationLabelDrift {
                    entry_id: m.entry_id.clone(),
                    destination: rendering.destination,
                    rendered: rendering.rendered_label,
                    published: m.published_label,
                });
            }
            if rendering.rendered_support_class != m.published_claim.support_class {
                violations.push(M5ClaimPublicationViolation::DestinationSupportClassDrift {
                    entry_id: m.entry_id.clone(),
                    destination: rendering.destination,
                });
            }
            if rendering.rendered_claim_text != m.published_claim.claim_text {
                violations.push(M5ClaimPublicationViolation::DestinationCopyDrift {
                    entry_id: m.entry_id.clone(),
                    destination: rendering.destination,
                });
            }
            if !rendering.discloses_freshness {
                violations.push(
                    M5ClaimPublicationViolation::DestinationFreshnessNotDisclosed {
                        entry_id: m.entry_id.clone(),
                        destination: rendering.destination,
                    },
                );
            }
            if !m.published_claim.scope_caveats.is_empty() && !rendering.discloses_caveats {
                violations.push(
                    M5ClaimPublicationViolation::DestinationCaveatsNotDisclosed {
                        entry_id: m.entry_id.clone(),
                        destination: rendering.destination,
                    },
                );
            }
        }
        for destination in M5ClaimDestination::REQUIRED {
            if !seen.contains(&destination) {
                violations.push(M5ClaimPublicationViolation::RequiredDestinationUncovered {
                    entry_id: m.entry_id.clone(),
                    destination,
                });
            }
        }
    }

    fn validate_held_manifest(
        &self,
        m: &M5ClaimPublication,
        violations: &mut Vec<M5ClaimPublicationViolation>,
    ) {
        // A published claim carries exactly the row's label, that label is at or
        // above the cutline, it names no active reason, rides a captured within-SLO
        // packet with current and signed backing reports inside an open validity
        // window, and is owner-signed.
        if m.published_label != m.row_published_label {
            violations.push(M5ClaimPublicationViolation::HeldLabelNotEqualRow {
                entry_id: m.entry_id.clone(),
                row: m.row_published_label,
                published: m.published_label,
            });
        }
        if !m.publishes_stable() {
            violations.push(M5ClaimPublicationViolation::PublishedStateNotStable {
                entry_id: m.entry_id.clone(),
                published: m.published_label,
            });
        }
        if !m.active_narrowing_reasons.is_empty() {
            violations.push(M5ClaimPublicationViolation::HeldWithActiveGap {
                entry_id: m.entry_id.clone(),
            });
        }
        if !m.proof_packet.has_capture() {
            violations.push(M5ClaimPublicationViolation::HeldWithoutFreshPacket {
                entry_id: m.entry_id.clone(),
            });
        }
        if !m.proof_packet.slo_state.is_within_slo() {
            violations.push(M5ClaimPublicationViolation::HeldOnStalePacket {
                entry_id: m.entry_id.clone(),
                slo_state: m.proof_packet.slo_state,
            });
        }
        for report in m.backing_reports() {
            if !report.state.is_current() {
                violations.push(M5ClaimPublicationViolation::HeldWithStaleReport {
                    entry_id: m.entry_id.clone(),
                    kind: report.report_kind,
                    state: report.state,
                });
            }
        }
        if m.published_claim.validity_window.expired {
            violations.push(M5ClaimPublicationViolation::HeldWithExpiredWindow {
                entry_id: m.entry_id.clone(),
            });
        }
        if !(m.owner_signoff.signed_off && m.owner_signoff.signed_at.is_some()) {
            violations.push(M5ClaimPublicationViolation::HeldWithoutSignoff {
                entry_id: m.entry_id.clone(),
            });
        }
    }

    fn validate_narrowed_manifest(
        &self,
        m: &M5ClaimPublication,
        violations: &mut Vec<M5ClaimPublicationViolation>,
    ) {
        // A narrowing claim must drop below the cutline and name at least one
        // active reason.
        if m.publishes_stable() {
            violations.push(M5ClaimPublicationViolation::NarrowedButPublishedStable {
                entry_id: m.entry_id.clone(),
                state: m.manifest_state,
                published: m.published_label,
            });
        }
        if m.active_narrowing_reasons.is_empty() {
            violations.push(M5ClaimPublicationViolation::NarrowingWithoutReason {
                entry_id: m.entry_id.clone(),
                state: m.manifest_state,
            });
        }

        // The narrowing state must be coherent with its active reasons.
        let any =
            |reasons: &[M5ClaimNarrowingReason]| reasons.iter().any(|r| m.has_active_reason(*r));
        let coherent = match m.manifest_state {
            M5ClaimManifestState::NarrowedRowDowngraded => {
                any(&[M5ClaimNarrowingReason::QualificationRowNarrowed])
            }
            M5ClaimManifestState::NarrowedStale => any(&[
                M5ClaimNarrowingReason::EvidenceStale,
                M5ClaimNarrowingReason::ReportStale,
            ]),
            M5ClaimManifestState::NarrowedMissing => any(&[
                M5ClaimNarrowingReason::EvidenceMissing,
                M5ClaimNarrowingReason::ReportMissing,
            ]),
            M5ClaimManifestState::Withheld => any(&[
                M5ClaimNarrowingReason::ReportDropped,
                M5ClaimNarrowingReason::ReportUnsigned,
                M5ClaimNarrowingReason::ValidityWindowExpired,
                M5ClaimNarrowingReason::OverClaimBeyondRow,
                M5ClaimNarrowingReason::OwnerSignoffMissing,
                M5ClaimNarrowingReason::WaiverExpired,
            ]),
            M5ClaimManifestState::Published => true,
        };
        if !coherent {
            violations.push(M5ClaimPublicationViolation::StateReasonIncoherent {
                entry_id: m.entry_id.clone(),
                state: m.manifest_state,
            });
        }

        // A stale or missing proof packet must name its matching reason.
        if m.proof_packet.slo_state == FreshnessSloState::Breached
            && !m.has_active_reason(M5ClaimNarrowingReason::EvidenceStale)
        {
            violations.push(M5ClaimPublicationViolation::StateWithoutReason {
                entry_id: m.entry_id.clone(),
                reason: M5ClaimNarrowingReason::EvidenceStale,
            });
        }
        if m.proof_packet.slo_state == FreshnessSloState::Missing
            && !m.has_active_reason(M5ClaimNarrowingReason::EvidenceMissing)
        {
            violations.push(M5ClaimPublicationViolation::StateWithoutReason {
                entry_id: m.entry_id.clone(),
                reason: M5ClaimNarrowingReason::EvidenceMissing,
            });
        }
        // A stale/missing/dropped/unsigned backing report must name its reason.
        for report in m.backing_reports() {
            let reason = match report.state {
                M5ClaimReportState::Stale => Some(M5ClaimNarrowingReason::ReportStale),
                M5ClaimReportState::Missing => Some(M5ClaimNarrowingReason::ReportMissing),
                M5ClaimReportState::Dropped => Some(M5ClaimNarrowingReason::ReportDropped),
                M5ClaimReportState::Unsigned => Some(M5ClaimNarrowingReason::ReportUnsigned),
                M5ClaimReportState::Current => None,
            };
            if let Some(reason) = reason {
                if !m.has_active_reason(reason) {
                    violations.push(M5ClaimPublicationViolation::StateWithoutReason {
                        entry_id: m.entry_id.clone(),
                        reason,
                    });
                }
            }
        }
        // An expired validity window must name its reason.
        if m.published_claim.validity_window.expired
            && !m.has_active_reason(M5ClaimNarrowingReason::ValidityWindowExpired)
        {
            violations.push(M5ClaimPublicationViolation::StateWithoutReason {
                entry_id: m.entry_id.clone(),
                reason: M5ClaimNarrowingReason::ValidityWindowExpired,
            });
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5ClaimPublicationViolation>) {
        let covered: BTreeSet<String> = self
            .manifests
            .iter()
            .map(|m| m.family_ref.clone())
            .collect();
        for declared in &self.release_blocking_family_refs {
            if !covered.contains(declared) {
                violations.push(
                    M5ClaimPublicationViolation::ReleaseBlockingFamilyUncovered {
                        family_ref: declared.clone(),
                    },
                );
            }
        }
        for m in &self.manifests {
            if m.release_blocking && !self.release_blocking_family_refs.contains(&m.family_ref) {
                violations.push(M5ClaimPublicationViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: m.entry_id.clone(),
                });
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<M5ClaimPublicationViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(M5ClaimPublicationViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(M5ClaimPublicationViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(M5ClaimPublicationViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(M5ClaimPublicationViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_claim_ids() {
            violations.push(M5ClaimPublicationViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the M5 claim-publication manifest register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ClaimPublicationViolation {
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
    /// The register has no manifests.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Manifest or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A manifest id appears more than once.
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
    NarrowingReasonWithoutStopRule {
        /// Uncovered reason.
        reason: M5ClaimNarrowingReason,
    },
    /// A backing report carries the wrong report kind.
    ReportKindMismatch {
        /// Manifest id.
        entry_id: String,
        /// Expected kind.
        expected: M5ClaimReportKind,
        /// Kind found.
        actual: M5ClaimReportKind,
    },
    /// A present report has no ref.
    ReportRefMissing {
        /// Manifest id.
        entry_id: String,
        /// Report kind.
        kind: M5ClaimReportKind,
    },
    /// A manifest drives the same destination twice.
    DuplicateDestination {
        /// Manifest id.
        entry_id: String,
        /// Duplicated destination.
        destination: M5ClaimDestination,
    },
    /// A manifest does not drive a required consuming destination.
    RequiredDestinationUncovered {
        /// Manifest id.
        entry_id: String,
        /// Uncovered destination.
        destination: M5ClaimDestination,
    },
    /// A destination renders from a different manifest id.
    DestinationSourceMismatch {
        /// Manifest id.
        entry_id: String,
        /// Offending destination.
        destination: M5ClaimDestination,
    },
    /// A destination renders a label that differs from the manifest's.
    DestinationLabelDrift {
        /// Manifest id.
        entry_id: String,
        /// Offending destination.
        destination: M5ClaimDestination,
        /// Label the destination rendered.
        rendered: StableClaimLevel,
        /// Label the manifest publishes.
        published: StableClaimLevel,
    },
    /// A destination renders a support class that differs from the manifest's.
    DestinationSupportClassDrift {
        /// Manifest id.
        entry_id: String,
        /// Offending destination.
        destination: M5ClaimDestination,
    },
    /// A destination renders wording that drifted from the manifest's.
    DestinationCopyDrift {
        /// Manifest id.
        entry_id: String,
        /// Offending destination.
        destination: M5ClaimDestination,
    },
    /// A destination does not disclose the manifest freshness.
    DestinationFreshnessNotDisclosed {
        /// Manifest id.
        entry_id: String,
        /// Offending destination.
        destination: M5ClaimDestination,
    },
    /// A destination carries caveats it does not disclose.
    DestinationCaveatsNotDisclosed {
        /// Manifest id.
        entry_id: String,
        /// Offending destination.
        destination: M5ClaimDestination,
    },
    /// A limited support class records no scope caveat.
    LimitedWithoutCaveat {
        /// Manifest id.
        entry_id: String,
    },
    /// The row's published label is wider than the backed claim's label.
    RowWiderThanClaim {
        /// Manifest id.
        entry_id: String,
        /// Claim label.
        claim: StableClaimLevel,
        /// Row label.
        row: StableClaimLevel,
    },
    /// The claim's published label is wider than the row it binds.
    ClaimPublishedWiderThanRow {
        /// Manifest id.
        entry_id: String,
        /// Row label.
        row: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A row narrowed below the cutline does not name the inherited reason.
    RowNarrowedWithoutReason {
        /// Manifest id.
        entry_id: String,
    },
    /// A published claim does not equal the row's label.
    HeldLabelNotEqualRow {
        /// Manifest id.
        entry_id: String,
        /// Row label.
        row: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A published claim does not publish at or above the cutline.
    PublishedStateNotStable {
        /// Manifest id.
        entry_id: String,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A published claim carries active narrowing reasons.
    HeldWithActiveGap {
        /// Manifest id.
        entry_id: String,
    },
    /// A published claim has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Manifest id.
        entry_id: String,
    },
    /// A published claim rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Manifest id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A published claim rides a non-current backing report.
    HeldWithStaleReport {
        /// Manifest id.
        entry_id: String,
        /// Report kind.
        kind: M5ClaimReportKind,
        /// Report state.
        state: M5ClaimReportState,
    },
    /// A published claim rides an expired validity window.
    HeldWithExpiredWindow {
        /// Manifest id.
        entry_id: String,
    },
    /// A published claim lacks owner sign-off.
    HeldWithoutSignoff {
        /// Manifest id.
        entry_id: String,
    },
    /// A narrowing claim did not drop below the cutline.
    NarrowedButPublishedStable {
        /// Manifest id.
        entry_id: String,
        /// Manifest state.
        state: M5ClaimManifestState,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A narrowing claim names no active reason.
    NarrowingWithoutReason {
        /// Manifest id.
        entry_id: String,
        /// Manifest state.
        state: M5ClaimManifestState,
    },
    /// A manifest state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Manifest id.
        entry_id: String,
        /// Manifest state.
        state: M5ClaimManifestState,
    },
    /// A stale/missing/dropped/unsigned input does not name its matching reason.
    StateWithoutReason {
        /// Manifest id.
        entry_id: String,
        /// Reason the input state requires.
        reason: M5ClaimNarrowingReason,
    },
    /// A release-blocking family ref has no covering manifest.
    ReleaseBlockingFamilyUncovered {
        /// Family ref.
        family_ref: String,
    },
    /// A release-blocking manifest is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Manifest id.
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
    /// The summary counts disagree with the manifests.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Manifest id.
        entry_id: String,
    },
}

impl fmt::Display for M5ClaimPublicationViolation {
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
            Self::EmptyRegister => write!(f, "register has no manifests"),
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
            Self::NarrowingReasonWithoutStopRule { reason } => write!(
                f,
                "narrowing reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::ReportKindMismatch {
                entry_id,
                expected,
                actual,
            } => write!(
                f,
                "manifest {entry_id} report expected kind {} but found {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::ReportRefMissing { entry_id, kind } => write!(
                f,
                "manifest {entry_id} report {} has no ref",
                kind.as_str()
            ),
            Self::DuplicateDestination {
                entry_id,
                destination,
            } => write!(
                f,
                "manifest {entry_id} drives destination {} twice",
                destination.as_str()
            ),
            Self::RequiredDestinationUncovered {
                entry_id,
                destination,
            } => write!(
                f,
                "manifest {entry_id} does not drive required destination {}",
                destination.as_str()
            ),
            Self::DestinationSourceMismatch {
                entry_id,
                destination,
            } => write!(
                f,
                "manifest {entry_id} destination {} renders from a different manifest id",
                destination.as_str()
            ),
            Self::DestinationLabelDrift {
                entry_id,
                destination,
                rendered,
                published,
            } => write!(
                f,
                "manifest {entry_id} destination {} rendered {rendered:?} but manifest publishes {published:?}",
                destination.as_str()
            ),
            Self::DestinationSupportClassDrift {
                entry_id,
                destination,
            } => write!(
                f,
                "manifest {entry_id} destination {} support class drifted from the manifest",
                destination.as_str()
            ),
            Self::DestinationCopyDrift {
                entry_id,
                destination,
            } => write!(
                f,
                "manifest {entry_id} destination {} wording drifted from the manifest",
                destination.as_str()
            ),
            Self::DestinationFreshnessNotDisclosed {
                entry_id,
                destination,
            } => write!(
                f,
                "manifest {entry_id} destination {} does not disclose freshness",
                destination.as_str()
            ),
            Self::DestinationCaveatsNotDisclosed {
                entry_id,
                destination,
            } => write!(
                f,
                "manifest {entry_id} destination {} does not disclose its caveats",
                destination.as_str()
            ),
            Self::LimitedWithoutCaveat { entry_id } => {
                write!(f, "manifest {entry_id} is limited without a caveat")
            }
            Self::RowWiderThanClaim {
                entry_id,
                claim,
                row,
            } => write!(
                f,
                "manifest {entry_id} row label {row:?} is wider than claim {claim:?}"
            ),
            Self::ClaimPublishedWiderThanRow {
                entry_id,
                row,
                published,
            } => write!(
                f,
                "manifest {entry_id} claim published {published:?} is wider than row {row:?}"
            ),
            Self::RowNarrowedWithoutReason { entry_id } => write!(
                f,
                "manifest {entry_id} row narrowed without qualification_row_narrowed reason"
            ),
            Self::HeldLabelNotEqualRow {
                entry_id,
                row,
                published,
            } => write!(
                f,
                "manifest {entry_id} held label {published:?} does not equal row {row:?}"
            ),
            Self::PublishedStateNotStable {
                entry_id,
                published,
            } => write!(
                f,
                "manifest {entry_id} is published but publishes {published:?} below the cutline"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "manifest {entry_id} publishes with an active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "manifest {entry_id} publishes without a fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => write!(
                f,
                "manifest {entry_id} publishes on stale packet {slo_state:?}"
            ),
            Self::HeldWithStaleReport {
                entry_id,
                kind,
                state,
            } => write!(
                f,
                "manifest {entry_id} publishes on {} report in state {}",
                kind.as_str(),
                state.as_str()
            ),
            Self::HeldWithExpiredWindow { entry_id } => {
                write!(f, "manifest {entry_id} publishes on an expired validity window")
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "manifest {entry_id} publishes without owner signoff")
            }
            Self::NarrowedButPublishedStable {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "manifest {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "manifest {entry_id} state {state:?} narrows without active reason"
            ),
            Self::StateReasonIncoherent { entry_id, state } => write!(
                f,
                "manifest {entry_id} state {state:?} is incoherent with its active reasons"
            ),
            Self::StateWithoutReason { entry_id, reason } => write!(
                f,
                "manifest {entry_id} stale/missing/dropped/unsigned input without {} reason",
                reason.as_str()
            ),
            Self::ReleaseBlockingFamilyUncovered { family_ref } => write!(
                f,
                "release-blocking family {family_ref} has no covering manifest"
            ),
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking manifest {entry_id} is not declared in release_blocking_family_refs"
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => write!(
                f,
                "promotion {declared:?} disagrees with computed {computed:?}"
            ),
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with manifests"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "manifest {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for M5ClaimPublicationViolation {}

/// Loads the embedded M5 claim-publication manifest register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`M5ClaimPublicationRegister`].
pub fn current_m5_claim_publication_manifests(
) -> Result<M5ClaimPublicationRegister, serde_json::Error> {
    serde_json::from_str(M5_CLAIM_PUBLICATION_MANIFESTS_JSON)
}

#[cfg(test)]
mod tests;
