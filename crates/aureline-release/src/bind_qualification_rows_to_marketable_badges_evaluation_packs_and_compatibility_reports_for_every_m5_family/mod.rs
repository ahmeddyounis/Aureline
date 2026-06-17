//! Typed M5 qualification-row badge / evaluation-pack / compatibility-report binding register.
//!
//! Where the qualification/skew matrix freezes the *machine-readable
//! qualification row* every M5 stable-facing family must hold — platform,
//! deployment profile, archetype/workflow bundle, toolchain envelope, and
//! client-scope cells, with declared skew window, support window, deprecation
//! packet, evidence freshness, and a published label — this register is the
//! *publication* layer on top of it. For every family it binds that one
//! qualification row to the marketable artifacts that advertise it:
//!
//! - a [`MarketableBadge`] that carries the published lifecycle label, the
//!   support class, the live evidence-freshness state, and the known
//!   compatibility caveats, so freshness and caveats travel with the badge
//!   wherever a support-class badge appears,
//! - an evaluation pack ([`BindingArtifactKind::EvaluationPack`]),
//! - a compatibility report ([`BindingArtifactKind::CompatibilityReport`]),
//! - and a release-center card ([`BindingArtifactKind::ReleaseCenterCard`]),
//!
//! each rendered across a closed set of [`BadgeSurface`]s that always covers the
//! product-truth surfaces (release center, Help/About, service health, support
//! export).
//!
//! The binding auto-narrows the badge below the row it inherits when its binding
//! evidence is stale or missing, or when marketable wording would exceed the
//! current row: the [`QualificationBadgeBinding::published_label`] may never be
//! wider than the row's [`QualificationBadgeBinding::row_published_label`], which
//! in turn may never be wider than the canonical
//! [`QualificationBadgeBinding::claim_label`]. A row that merely inherits an
//! upstream narrowing narrows the badge but does not itself hold promotion — the
//! qualification matrix already gates that — while a *binding-layer* failure
//! (stale evidence, a stale or missing evaluation pack or compatibility report, an
//! over-claiming badge, a missing owner sign-off, or an expired waiver) holds
//! promotion through a [`BadgeBindingStopRule`].
//!
//! This register reuses the canonical [`FamilyKind`] and [`SupportClass`]
//! vocabularies from the qualification/skew matrix, the [`ProofPacket`] and
//! [`FreshnessSloState`] freshness vocabulary from the stable claim manifest, and
//! the [`LaunchCutline`], [`StableClaimLevel`], [`OwnerSignoff`],
//! [`QualificationWaiver`], [`PromotionDecision`], and [`PromotionDecisionRecord`]
//! types from the stable claim matrix rather than minting local synonyms.
//!
//! The register is checked in at [`BIND_M5_QUALIFICATION_BADGE_BINDINGS_PATH`] and
//! embedded here, so this typed consumer and the CI gate agree on every binding
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
pub const BIND_M5_QUALIFICATION_BADGE_BINDINGS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const BIND_M5_QUALIFICATION_BADGE_BINDINGS_RECORD_KIND: &str =
    "bind_m5_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports";

/// Repo-relative path to the checked-in register.
pub const BIND_M5_QUALIFICATION_BADGE_BINDINGS_PATH: &str =
    "artifacts/release/m5/bind_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports_for_every_m5_family.json";

/// Embedded checked-in register JSON.
pub const BIND_M5_QUALIFICATION_BADGE_BINDINGS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/bind_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports_for_every_m5_family.json"
));

/// One marketable artifact a qualification row binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingArtifactKind {
    /// The support-class badge rendered across surfaces.
    MarketableBadge,
    /// The evaluation pack a family publishes.
    EvaluationPack,
    /// The compatibility report a family publishes.
    CompatibilityReport,
    /// The release-center card a family publishes.
    ReleaseCenterCard,
}

impl BindingArtifactKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MarketableBadge,
        Self::EvaluationPack,
        Self::CompatibilityReport,
        Self::ReleaseCenterCard,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarketableBadge => "marketable_badge",
            Self::EvaluationPack => "evaluation_pack",
            Self::CompatibilityReport => "compatibility_report",
            Self::ReleaseCenterCard => "release_center_card",
        }
    }
}

/// Freshness state of a bound marketable artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactState {
    /// The artifact is current.
    Current,
    /// The artifact exists but has gone stale.
    Stale,
    /// The artifact has not been produced.
    Missing,
}

impl ArtifactState {
    /// Every state, freshest to stalest.
    pub const ALL: [Self; 3] = [Self::Current, Self::Stale, Self::Missing];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }

    /// Whether a held badge may ride this artifact.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// A surface that renders a qualification badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeSurface {
    /// The release-center surface.
    ReleaseCenter,
    /// The Help/About surface.
    HelpAbout,
    /// The service-health surface.
    ServiceHealth,
    /// The support-export surface.
    SupportExport,
    /// Product documentation.
    Docs,
    /// Release notes.
    ReleaseNotes,
    /// CLI/headless inspect output.
    CliInspect,
    /// The marketplace listing surface.
    MarketplaceListing,
}

impl BadgeSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ReleaseCenter,
        Self::HelpAbout,
        Self::ServiceHealth,
        Self::SupportExport,
        Self::Docs,
        Self::ReleaseNotes,
        Self::CliInspect,
        Self::MarketplaceListing,
    ];

    /// The product-truth surfaces every binding must render its badge on, so the
    /// freshness state and caveats appear wherever a support-class badge appears.
    pub const TRUTH_SURFACES: [Self; 4] = [
        Self::ReleaseCenter,
        Self::HelpAbout,
        Self::ServiceHealth,
        Self::SupportExport,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::HelpAbout => "help_about",
            Self::ServiceHealth => "service_health",
            Self::SupportExport => "support_export",
            Self::Docs => "docs",
            Self::ReleaseNotes => "release_notes",
            Self::CliInspect => "cli_inspect",
            Self::MarketplaceListing => "marketplace_listing",
        }
    }
}

/// Overall state a badge binding earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingState {
    /// The badge publishes the row's label; all binding artifacts are current.
    Published,
    /// The badge inherited an upstream qualification-row narrowing.
    NarrowedRowDowngraded,
    /// A bound artifact or the proof packet is stale; the badge narrows.
    NarrowedStale,
    /// A bound artifact or the proof packet is missing; the badge narrows.
    NarrowedMissing,
    /// The badge is withheld entirely (over-claim or unverifiable).
    Withheld,
}

impl BindingState {
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

    /// Whether the state lets the badge publish the row's label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Published)
    }
}

/// Closed reason a badge narrows below the row it binds or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingNarrowingReason {
    /// The upstream qualification row narrowed below the cutline.
    QualificationRowNarrowed,
    /// The binding proof packet breached its freshness SLO.
    EvidenceStale,
    /// No binding proof packet has been captured.
    EvidenceMissing,
    /// The bound evaluation pack is stale.
    EvaluationPackStale,
    /// No evaluation pack has been produced.
    EvaluationPackMissing,
    /// The bound compatibility report is stale.
    CompatibilityReportStale,
    /// No compatibility report has been produced.
    CompatibilityReportMissing,
    /// The badge would advertise wider than the qualification row.
    OverClaimBeyondRow,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
    /// A waiver the badge relied on has expired.
    WaiverExpired,
}

impl BindingNarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::QualificationRowNarrowed,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::EvaluationPackStale,
        Self::EvaluationPackMissing,
        Self::CompatibilityReportStale,
        Self::CompatibilityReportMissing,
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
            Self::EvaluationPackStale => "evaluation_pack_stale",
            Self::EvaluationPackMissing => "evaluation_pack_missing",
            Self::CompatibilityReportStale => "compatibility_report_stale",
            Self::CompatibilityReportMissing => "compatibility_report_missing",
            Self::OverClaimBeyondRow => "over_claim_beyond_row",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether a binding whose claim is at or above the cutline carrying this
    /// reason holds promotion. A reason that merely inherits an upstream
    /// qualification-row narrowing is gated by the matrix, not this register.
    pub const fn blocks_promotion(self) -> bool {
        !matches!(self, Self::QualificationRowNarrowed)
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingStopAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the badge to inherit the row.
    NarrowBadge,
    /// Withhold the badge entirely.
    WithholdBadge,
    /// Refresh the evaluation pack.
    RefreshEvaluationPack,
    /// Refresh the compatibility report.
    RefreshCompatibilityReport,
    /// Refresh the binding evidence packet.
    RefreshEvidence,
    /// Align marketable wording to the current row.
    AlignMarketingToRow,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl BindingStopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::HoldPublication,
        Self::NarrowBadge,
        Self::WithholdBadge,
        Self::RefreshEvaluationPack,
        Self::RefreshCompatibilityReport,
        Self::RefreshEvidence,
        Self::AlignMarketingToRow,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowBadge => "narrow_badge",
            Self::WithholdBadge => "withhold_badge",
            Self::RefreshEvaluationPack => "refresh_evaluation_pack",
            Self::RefreshCompatibilityReport => "refresh_compatibility_report",
            Self::RefreshEvidence => "refresh_evidence",
            Self::AlignMarketingToRow => "align_marketing_to_row",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// The marketable support-class badge a family publishes.
///
/// The badge carries the evidence freshness and known caveats inline so that,
/// wherever a support-class badge renders, the freshness and caveats render with
/// it. The badge label may never exceed the binding's published label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketableBadge {
    /// Copy-safe marketable text. Never wider than the row it binds.
    pub badge_text: String,
    /// The lifecycle label the badge advertises. Equals the published label.
    pub badge_label: StableClaimLevel,
    /// The support class the badge advertises.
    pub support_class: SupportClass,
    /// The live evidence-freshness state the badge discloses.
    pub freshness_state: FreshnessSloState,
    /// Known compatibility caveats that travel with the badge.
    #[serde(default)]
    pub caveat_summary: Vec<String>,
    /// Whether the badge discloses its freshness state. Always required.
    pub freshness_disclosed: bool,
    /// Whether the badge discloses its caveats. Required when caveats exist.
    pub caveats_disclosed: bool,
}

/// A bound marketable artifact (evaluation pack, compatibility report, or
/// release-center card) joined to a qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BindingArtifactRef {
    /// The kind of artifact this ref names.
    pub artifact_kind: BindingArtifactKind,
    /// Ref to the artifact. Empty only when the state is `missing`.
    pub artifact_ref: String,
    /// The artifact's freshness state.
    pub state: ArtifactState,
    /// UTC date the artifact was produced, or null when missing.
    #[serde(default)]
    pub captured_at: Option<String>,
}

/// One badge-binding stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BadgeBindingStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched binding fires this rule.
    pub trigger_reason: BindingNarrowingReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: BindingStopAction,
    /// Whether firing this rule holds promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One qualification-row badge binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationBadgeBinding {
    /// Stable binding id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The family this binding governs.
    pub family_kind: FamilyKind,
    /// The family ref this binding speaks about.
    pub family_ref: String,
    /// Reviewable one-line statement of the family.
    pub family_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim entry id whose claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes (the hard ceiling).
    pub claim_label: StableClaimLevel,
    /// The qualification-row entry id this binding joins to.
    pub qualification_row_ref: String,
    /// The label the upstream qualification row publishes (the badge ceiling).
    pub row_published_label: StableClaimLevel,
    /// Overall binding state earned.
    pub binding_state: BindingState,
    /// The support class the family commits to.
    pub support_class: SupportClass,
    /// The marketable badge.
    pub badge: MarketableBadge,
    /// The bound evaluation pack.
    pub evaluation_pack: BindingArtifactRef,
    /// The bound compatibility report.
    pub compatibility_report: BindingArtifactRef,
    /// The bound release-center card.
    pub release_center_card: BindingArtifactRef,
    /// Surfaces the badge renders on. Always covers the truth surfaces.
    pub surfaces: Vec<BadgeSurface>,
    /// Recorded compatibility caveats. Non-empty when support is limited.
    #[serde(default)]
    pub compatibility_caveats: Vec<String>,
    /// The binding proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional badge, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the badge below the row's label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<BindingNarrowingReason>,
    /// The lifecycle label the badge effectively publishes after narrowing.
    pub published_label: StableClaimLevel,
    /// Reviewable reason the binding carries this posture.
    pub rationale: String,
}

impl QualificationBadgeBinding {
    /// True when the published badge label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the binding state lets the badge carry the row's label.
    pub fn holds_label(&self) -> bool {
        self.binding_state.holds_label()
    }

    /// True when a narrowing reason is active on the binding.
    pub fn has_active_reason(&self, reason: BindingNarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationBadgeBindingSummary {
    /// Total number of bindings.
    pub total_bindings: usize,
    /// Distinct families covered.
    pub total_families: usize,
    /// Bindings publishing a badge at or above the cutline.
    pub bindings_published: usize,
    /// Bindings whose badge narrowed below the cutline.
    pub bindings_narrowed: usize,
    /// Total release-blocking bindings.
    pub release_blocking_total: usize,
    /// Release-blocking bindings publishing at or above the cutline.
    pub release_blocking_published: usize,
    /// Release-blocking bindings narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook bindings.
    pub notebook_bindings: usize,
    /// AI/provider bindings.
    pub ai_provider_bindings: usize,
    /// Remote/helper bindings.
    pub remote_helper_bindings: usize,
    /// Companion bindings.
    pub companion_bindings: usize,
    /// Ecosystem bindings.
    pub ecosystem_bindings: usize,
    /// Managed-service bindings.
    pub managed_service_bindings: usize,
    /// Toolchain/runtime bindings.
    pub toolchain_runtime_bindings: usize,
    /// Bindings in the `published` state.
    pub state_published: usize,
    /// Bindings in the `narrowed_row_downgraded` state.
    pub state_narrowed_row_downgraded: usize,
    /// Bindings in the `narrowed_stale` state.
    pub state_narrowed_stale: usize,
    /// Bindings in the `narrowed_missing` state.
    pub state_narrowed_missing: usize,
    /// Bindings in the `withheld` state.
    pub state_withheld: usize,
    /// Badges carrying at least one caveat.
    pub badges_with_caveats: usize,
    /// Badges that disclose their freshness state.
    pub badges_freshness_disclosed: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Evaluation packs that are current.
    pub evaluation_packs_current: usize,
    /// Evaluation packs that are stale.
    pub evaluation_packs_stale: usize,
    /// Evaluation packs that are missing.
    pub evaluation_packs_missing: usize,
    /// Compatibility reports that are current.
    pub compatibility_reports_current: usize,
    /// Compatibility reports that are stale.
    pub compatibility_reports_stale: usize,
    /// Compatibility reports that are missing.
    pub compatibility_reports_missing: usize,
    /// Total active narrowing reasons across all bindings.
    pub total_active_narrowing_reasons: usize,
    /// Total surface renderings across all bindings.
    pub total_surface_renderings: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeBindingExportRow {
    /// Stable binding id.
    pub entry_id: String,
    /// The family this binding governs.
    pub family_kind: FamilyKind,
    /// The family ref this binding speaks about.
    pub family_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The qualification-row entry id this binding joins to.
    pub qualification_row_ref: String,
    /// The canonical claim label.
    pub claim_label: StableClaimLevel,
    /// The upstream row's published label.
    pub row_published_label: StableClaimLevel,
    /// The badge's effective published label.
    pub published_label: StableClaimLevel,
    /// Whether the badge publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall binding state earned.
    pub binding_state: BindingState,
    /// The support class the badge advertises.
    pub support_class: SupportClass,
    /// The badge's disclosed freshness state.
    pub freshness_state: FreshnessSloState,
    /// The badge's caveat summary.
    pub caveat_summary: Vec<String>,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<BindingNarrowingReason>,
}

/// Export projection for Help/About, service-health, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeBindingExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<BadgeBindingExportRow>,
}

/// The typed M5 qualification-row badge binding register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationBadgeBindingRegister {
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
    /// Closed artifact-kind vocabulary.
    pub artifact_kinds: Vec<BindingArtifactKind>,
    /// Closed artifact-state vocabulary.
    pub artifact_states: Vec<ArtifactState>,
    /// Closed badge-surface vocabulary.
    pub badge_surfaces: Vec<BadgeSurface>,
    /// Closed binding-state vocabulary.
    pub binding_states: Vec<BindingState>,
    /// Closed freshness-state vocabulary.
    pub freshness_states: Vec<FreshnessSloState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<BindingNarrowingReason>,
    /// Closed stop-action vocabulary.
    pub stop_actions: Vec<BindingStopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking family refs this register must cover.
    pub release_blocking_family_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<BadgeBindingStopRule>,
    /// Badge bindings.
    pub bindings: Vec<QualificationBadgeBinding>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: QualificationBadgeBindingSummary,
}

impl QualificationBadgeBindingRegister {
    /// Returns the binding registered for `entry_id`.
    pub fn binding(&self, entry_id: &str) -> Option<&QualificationBadgeBinding> {
        self.bindings.iter().find(|b| b.entry_id == entry_id)
    }

    /// Returns the bindings publishing a badge at or above the cutline.
    pub fn bindings_published(&self) -> Vec<&QualificationBadgeBinding> {
        self.bindings
            .iter()
            .filter(|b| b.publishes_stable())
            .collect()
    }

    /// Returns the bindings whose badge narrowed below the cutline.
    pub fn bindings_narrowed(&self) -> Vec<&QualificationBadgeBinding> {
        self.bindings
            .iter()
            .filter(|b| !b.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking bindings.
    pub fn release_blocking_bindings(&self) -> Vec<&QualificationBadgeBinding> {
        self.bindings
            .iter()
            .filter(|b| b.release_blocking)
            .collect()
    }

    /// Returns the bindings for one family kind.
    pub fn bindings_for_kind(&self, kind: FamilyKind) -> Vec<&QualificationBadgeBinding> {
        self.bindings
            .iter()
            .filter(|b| b.family_kind == kind)
            .collect()
    }

    /// Distinct families (by family ref) the register covers.
    pub fn families(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for b in &self.bindings {
            set.insert(b.family_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched binding carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &BadgeBindingStopRule) -> bool {
        self.bindings.iter().any(|b| {
            rule.applies_to_labels.contains(&b.claim_label)
                && b.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the bindings and stop rules.
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

    /// Binding ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only bindings whose claim is at or above the cutline count: a binding whose
    /// claim is already canonically narrowed merely inherits the upstream ceiling.
    pub fn computed_blocking_claim_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<BindingNarrowingReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for b in &self.bindings {
            if b.claim_holds_stable()
                && b.active_narrowing_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(b.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the bindings and stop rules.
    pub fn computed_summary(&self) -> QualificationBadgeBindingSummary {
        let kind = |kind: FamilyKind| self.bindings_for_kind(kind).len();
        let state = |state: BindingState| {
            self.bindings
                .iter()
                .filter(|b| b.binding_state == state)
                .count()
        };
        let packets = |s: FreshnessSloState| {
            self.bindings
                .iter()
                .filter(|b| b.proof_packet.slo_state == s)
                .count()
        };
        let eval = |s: ArtifactState| {
            self.bindings
                .iter()
                .filter(|b| b.evaluation_pack.state == s)
                .count()
        };
        let report = |s: ArtifactState| {
            self.bindings
                .iter()
                .filter(|b| b.compatibility_report.state == s)
                .count()
        };
        let release_blocking: Vec<&QualificationBadgeBinding> = self.release_blocking_bindings();
        QualificationBadgeBindingSummary {
            total_bindings: self.bindings.len(),
            total_families: self.families().len(),
            bindings_published: self.bindings_published().len(),
            bindings_narrowed: self.bindings_narrowed().len(),
            release_blocking_total: release_blocking.len(),
            release_blocking_published: release_blocking
                .iter()
                .filter(|b| b.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|b| !b.publishes_stable())
                .count(),
            notebook_bindings: kind(FamilyKind::Notebook),
            ai_provider_bindings: kind(FamilyKind::AiProvider),
            remote_helper_bindings: kind(FamilyKind::RemoteHelper),
            companion_bindings: kind(FamilyKind::Companion),
            ecosystem_bindings: kind(FamilyKind::Ecosystem),
            managed_service_bindings: kind(FamilyKind::ManagedService),
            toolchain_runtime_bindings: kind(FamilyKind::ToolchainRuntime),
            state_published: state(BindingState::Published),
            state_narrowed_row_downgraded: state(BindingState::NarrowedRowDowngraded),
            state_narrowed_stale: state(BindingState::NarrowedStale),
            state_narrowed_missing: state(BindingState::NarrowedMissing),
            state_withheld: state(BindingState::Withheld),
            badges_with_caveats: self
                .bindings
                .iter()
                .filter(|b| !b.badge.caveat_summary.is_empty())
                .count(),
            badges_freshness_disclosed: self
                .bindings
                .iter()
                .filter(|b| b.badge.freshness_disclosed)
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            evaluation_packs_current: eval(ArtifactState::Current),
            evaluation_packs_stale: eval(ArtifactState::Stale),
            evaluation_packs_missing: eval(ArtifactState::Missing),
            compatibility_reports_current: report(ArtifactState::Current),
            compatibility_reports_stale: report(ArtifactState::Stale),
            compatibility_reports_missing: report(ArtifactState::Missing),
            total_active_narrowing_reasons: self
                .bindings
                .iter()
                .map(|b| b.active_narrowing_reasons.len())
                .sum(),
            total_surface_renderings: self.bindings.iter().map(|b| b.surfaces.len()).sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection that downstream surfaces
    /// render instead of cloning status text. The freshness state and caveats
    /// travel with every row so they appear wherever a support-class badge does.
    pub fn support_export_projection(&self) -> BadgeBindingExportProjection {
        BadgeBindingExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .bindings
                .iter()
                .map(|b| BadgeBindingExportRow {
                    entry_id: b.entry_id.clone(),
                    family_kind: b.family_kind,
                    family_ref: b.family_ref.clone(),
                    release_blocking: b.release_blocking,
                    qualification_row_ref: b.qualification_row_ref.clone(),
                    claim_label: b.claim_label,
                    row_published_label: b.row_published_label,
                    published_label: b.published_label,
                    publishes_stable: b.publishes_stable(),
                    binding_state: b.binding_state,
                    support_class: b.support_class,
                    freshness_state: b.badge.freshness_state,
                    caveat_summary: b.badge.caveat_summary.clone(),
                    active_narrowing_reasons: b.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<QualificationBadgeBindingViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for b in &self.bindings {
            if !seen.insert(b.entry_id.clone()) {
                violations.push(QualificationBadgeBindingViolation::DuplicateEntryId {
                    entry_id: b.entry_id.clone(),
                });
            }
            self.validate_binding(b, &mut violations);
        }
        if self.bindings.is_empty() {
            violations.push(QualificationBadgeBindingViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(QualificationBadgeBindingViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<QualificationBadgeBindingViolation>) {
        if self.schema_version != BIND_M5_QUALIFICATION_BADGE_BINDINGS_SCHEMA_VERSION {
            violations.push(
                QualificationBadgeBindingViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != BIND_M5_QUALIFICATION_BADGE_BINDINGS_RECORD_KIND {
            violations.push(QualificationBadgeBindingViolation::UnsupportedRecordKind {
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
                violations.push(QualificationBadgeBindingViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        let vocab: [(bool, &'static str); 9] = [
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
                self.artifact_kinds == BindingArtifactKind::ALL.to_vec(),
                "artifact_kinds",
            ),
            (
                self.artifact_states == ArtifactState::ALL.to_vec(),
                "artifact_states",
            ),
            (
                self.badge_surfaces == BadgeSurface::ALL.to_vec(),
                "badge_surfaces",
            ),
            (
                self.binding_states == BindingState::ALL.to_vec(),
                "binding_states",
            ),
            (
                self.freshness_states == FreshnessSloState::ALL.to_vec(),
                "freshness_states",
            ),
            (
                self.narrowing_reasons == BindingNarrowingReason::ALL.to_vec(),
                "narrowing_reasons",
            ),
        ];
        for (ok, field) in vocab {
            if !ok {
                violations
                    .push(QualificationBadgeBindingViolation::ClosedVocabularyMismatch { field });
            }
        }
        if self.stop_actions != BindingStopAction::ALL.to_vec() {
            violations.push(
                QualificationBadgeBindingViolation::ClosedVocabularyMismatch {
                    field: "stop_actions",
                },
            );
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(
                QualificationBadgeBindingViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.cutline_level",
                },
            );
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(
                QualificationBadgeBindingViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.above_cutline_levels",
                },
            );
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(
                QualificationBadgeBindingViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.below_cutline_levels",
                },
            );
        }
        if cutline.description.trim().is_empty() {
            violations.push(QualificationBadgeBindingViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<QualificationBadgeBindingViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(QualificationBadgeBindingViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(QualificationBadgeBindingViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(QualificationBadgeBindingViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(QualificationBadgeBindingViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            // The stop rule's blocks_promotion flag must agree with the reason's
            // canonical promotion semantics.
            if rule.blocks_promotion != rule.trigger_reason.blocks_promotion() {
                violations.push(
                    QualificationBadgeBindingViolation::StopRuleBlockingMismatch {
                        rule_id: rule.rule_id.clone(),
                    },
                );
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in BindingNarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations.push(
                    QualificationBadgeBindingViolation::NarrowingReasonWithoutStopRule { reason },
                );
            }
        }
    }

    fn validate_binding(
        &self,
        b: &QualificationBadgeBinding,
        violations: &mut Vec<QualificationBadgeBindingViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &b.entry_id),
            ("title", &b.title),
            ("family_ref", &b.family_ref),
            ("family_summary", &b.family_summary),
            ("claim_ref", &b.claim_ref),
            ("qualification_row_ref", &b.qualification_row_ref),
            ("badge.badge_text", &b.badge.badge_text),
            ("proof_packet.packet_id", &b.proof_packet.packet_id),
            ("proof_packet.packet_ref", &b.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &b.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &b.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &b.owner_signoff.owner_ref),
            ("rationale", &b.rationale),
        ] {
            if value.trim().is_empty() {
                violations.push(QualificationBadgeBindingViolation::EmptyField {
                    entry_id: b.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_artifacts(b, violations);
        self.validate_surfaces(b, violations);
        self.validate_badge(b, violations);

        // The ceilings: the row may not exceed the claim, and the badge may not
        // exceed the row.
        if b.row_published_label.rank() > b.claim_label.rank() {
            violations.push(QualificationBadgeBindingViolation::RowWiderThanClaim {
                entry_id: b.entry_id.clone(),
                claim: b.claim_label,
                row: b.row_published_label,
            });
        }
        if b.published_label.rank() > b.row_published_label.rank() {
            violations.push(
                QualificationBadgeBindingViolation::BadgePublishedWiderThanRow {
                    entry_id: b.entry_id.clone(),
                    row: b.row_published_label,
                    published: b.published_label,
                },
            );
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if b.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(QualificationBadgeBindingViolation::EmptyField {
                entry_id: b.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !b.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(
                QualificationBadgeBindingViolation::FreshnessSloInconsistent {
                    entry_id: b.entry_id.clone(),
                },
            );
        }

        // A row narrowed below the cutline must name the inherited reason.
        if !b.row_published_label.is_at_or_above_cutline()
            && !b.has_active_reason(BindingNarrowingReason::QualificationRowNarrowed)
        {
            violations.push(
                QualificationBadgeBindingViolation::RowNarrowedWithoutReason {
                    entry_id: b.entry_id.clone(),
                },
            );
        }

        // A limited support class must record at least one caveat.
        if b.support_class == SupportClass::Limited
            && b.compatibility_caveats.iter().all(|c| c.trim().is_empty())
            && b.badge.caveat_summary.iter().all(|c| c.trim().is_empty())
        {
            violations.push(QualificationBadgeBindingViolation::LimitedWithoutCaveat {
                entry_id: b.entry_id.clone(),
            });
        }

        if b.holds_label() {
            self.validate_held_binding(b, violations);
        } else {
            self.validate_narrowed_binding(b, violations);
        }
    }

    fn validate_artifacts(
        &self,
        b: &QualificationBadgeBinding,
        violations: &mut Vec<QualificationBadgeBindingViolation>,
    ) {
        for (artifact, expected) in [
            (&b.evaluation_pack, BindingArtifactKind::EvaluationPack),
            (
                &b.compatibility_report,
                BindingArtifactKind::CompatibilityReport,
            ),
            (
                &b.release_center_card,
                BindingArtifactKind::ReleaseCenterCard,
            ),
        ] {
            if artifact.artifact_kind != expected {
                violations.push(QualificationBadgeBindingViolation::ArtifactKindMismatch {
                    entry_id: b.entry_id.clone(),
                    expected,
                    actual: artifact.artifact_kind,
                });
            }
            // A present artifact carries a ref; a missing one carries none.
            if artifact.state != ArtifactState::Missing && artifact.artifact_ref.trim().is_empty() {
                violations.push(QualificationBadgeBindingViolation::ArtifactRefMissing {
                    entry_id: b.entry_id.clone(),
                    kind: expected,
                });
            }
        }
    }

    fn validate_surfaces(
        &self,
        b: &QualificationBadgeBinding,
        violations: &mut Vec<QualificationBadgeBindingViolation>,
    ) {
        let mut seen: BTreeSet<BadgeSurface> = BTreeSet::new();
        for surface in &b.surfaces {
            if !seen.insert(*surface) {
                violations.push(QualificationBadgeBindingViolation::DuplicateSurface {
                    entry_id: b.entry_id.clone(),
                    surface: *surface,
                });
            }
        }
        for surface in BadgeSurface::TRUTH_SURFACES {
            if !seen.contains(&surface) {
                violations.push(QualificationBadgeBindingViolation::TruthSurfaceUncovered {
                    entry_id: b.entry_id.clone(),
                    surface,
                });
            }
        }
    }

    fn validate_badge(
        &self,
        b: &QualificationBadgeBinding,
        violations: &mut Vec<QualificationBadgeBindingViolation>,
    ) {
        // The badge label, support class, and freshness must mirror the binding.
        if b.badge.badge_label != b.published_label {
            violations.push(QualificationBadgeBindingViolation::BadgeLabelMismatch {
                entry_id: b.entry_id.clone(),
                badge: b.badge.badge_label,
                published: b.published_label,
            });
        }
        if b.badge.support_class != b.support_class {
            violations.push(
                QualificationBadgeBindingViolation::BadgeSupportClassMismatch {
                    entry_id: b.entry_id.clone(),
                },
            );
        }
        if b.badge.freshness_state != b.proof_packet.slo_state {
            violations.push(QualificationBadgeBindingViolation::BadgeFreshnessMismatch {
                entry_id: b.entry_id.clone(),
                badge: b.badge.freshness_state,
                packet: b.proof_packet.slo_state,
            });
        }
        // Freshness must always be disclosed; caveats must be disclosed when any
        // exist, so freshness and caveats render wherever the badge does.
        if !b.badge.freshness_disclosed {
            violations.push(QualificationBadgeBindingViolation::FreshnessNotDisclosed {
                entry_id: b.entry_id.clone(),
            });
        }
        if !b.badge.caveat_summary.is_empty() && !b.badge.caveats_disclosed {
            violations.push(QualificationBadgeBindingViolation::CaveatsNotDisclosed {
                entry_id: b.entry_id.clone(),
            });
        }
    }

    fn validate_held_binding(
        &self,
        b: &QualificationBadgeBinding,
        violations: &mut Vec<QualificationBadgeBindingViolation>,
    ) {
        // A published badge carries exactly the row's label, that label is at or
        // above the cutline, it carries no active reason, rides a captured
        // within-SLO packet with current binding artifacts, and is owner-signed.
        if b.published_label != b.row_published_label {
            violations.push(QualificationBadgeBindingViolation::HeldLabelNotEqualRow {
                entry_id: b.entry_id.clone(),
                row: b.row_published_label,
                published: b.published_label,
            });
        }
        if !b.publishes_stable() {
            violations.push(
                QualificationBadgeBindingViolation::PublishedStateNotStable {
                    entry_id: b.entry_id.clone(),
                    published: b.published_label,
                },
            );
        }
        if !b.active_narrowing_reasons.is_empty() {
            violations.push(QualificationBadgeBindingViolation::HeldWithActiveGap {
                entry_id: b.entry_id.clone(),
            });
        }
        if !b.proof_packet.has_capture() {
            violations.push(QualificationBadgeBindingViolation::HeldWithoutFreshPacket {
                entry_id: b.entry_id.clone(),
            });
        }
        if !b.proof_packet.slo_state.is_within_slo() {
            violations.push(QualificationBadgeBindingViolation::HeldOnStalePacket {
                entry_id: b.entry_id.clone(),
                slo_state: b.proof_packet.slo_state,
            });
        }
        if !b.evaluation_pack.state.is_current() {
            violations.push(QualificationBadgeBindingViolation::HeldWithStaleArtifact {
                entry_id: b.entry_id.clone(),
                kind: BindingArtifactKind::EvaluationPack,
            });
        }
        if !b.compatibility_report.state.is_current() {
            violations.push(QualificationBadgeBindingViolation::HeldWithStaleArtifact {
                entry_id: b.entry_id.clone(),
                kind: BindingArtifactKind::CompatibilityReport,
            });
        }
        if !(b.owner_signoff.signed_off && b.owner_signoff.signed_at.is_some()) {
            violations.push(QualificationBadgeBindingViolation::HeldWithoutSignoff {
                entry_id: b.entry_id.clone(),
            });
        }
    }

    fn validate_narrowed_binding(
        &self,
        b: &QualificationBadgeBinding,
        violations: &mut Vec<QualificationBadgeBindingViolation>,
    ) {
        // A narrowing badge must drop below the cutline and name at least one
        // active reason.
        if b.publishes_stable() {
            violations.push(
                QualificationBadgeBindingViolation::NarrowedButPublishedStable {
                    entry_id: b.entry_id.clone(),
                    state: b.binding_state,
                    published: b.published_label,
                },
            );
        }
        if b.active_narrowing_reasons.is_empty() {
            violations.push(QualificationBadgeBindingViolation::NarrowingWithoutReason {
                entry_id: b.entry_id.clone(),
                state: b.binding_state,
            });
        }

        // The narrowing state must be coherent with its active reasons.
        let any =
            |reasons: &[BindingNarrowingReason]| reasons.iter().any(|r| b.has_active_reason(*r));
        let coherent = match b.binding_state {
            BindingState::NarrowedRowDowngraded => {
                any(&[BindingNarrowingReason::QualificationRowNarrowed])
            }
            BindingState::NarrowedStale => any(&[
                BindingNarrowingReason::EvidenceStale,
                BindingNarrowingReason::EvaluationPackStale,
                BindingNarrowingReason::CompatibilityReportStale,
            ]),
            BindingState::NarrowedMissing => any(&[
                BindingNarrowingReason::EvidenceMissing,
                BindingNarrowingReason::EvaluationPackMissing,
                BindingNarrowingReason::CompatibilityReportMissing,
            ]),
            BindingState::Withheld => any(&[
                BindingNarrowingReason::OverClaimBeyondRow,
                BindingNarrowingReason::OwnerSignoffMissing,
                BindingNarrowingReason::WaiverExpired,
            ]),
            BindingState::Published => true,
        };
        if !coherent {
            violations.push(QualificationBadgeBindingViolation::StateReasonIncoherent {
                entry_id: b.entry_id.clone(),
                state: b.binding_state,
            });
        }

        // A stale or missing proof packet / evaluation pack / compatibility report
        // must name its matching reason.
        if b.proof_packet.slo_state == FreshnessSloState::Breached
            && !b.has_active_reason(BindingNarrowingReason::EvidenceStale)
        {
            violations.push(
                QualificationBadgeBindingViolation::ArtifactStateWithoutReason {
                    entry_id: b.entry_id.clone(),
                    reason: BindingNarrowingReason::EvidenceStale,
                },
            );
        }
        if b.proof_packet.slo_state == FreshnessSloState::Missing
            && !b.has_active_reason(BindingNarrowingReason::EvidenceMissing)
        {
            violations.push(
                QualificationBadgeBindingViolation::ArtifactStateWithoutReason {
                    entry_id: b.entry_id.clone(),
                    reason: BindingNarrowingReason::EvidenceMissing,
                },
            );
        }
        for (artifact, stale, missing) in [
            (
                &b.evaluation_pack,
                BindingNarrowingReason::EvaluationPackStale,
                BindingNarrowingReason::EvaluationPackMissing,
            ),
            (
                &b.compatibility_report,
                BindingNarrowingReason::CompatibilityReportStale,
                BindingNarrowingReason::CompatibilityReportMissing,
            ),
        ] {
            if artifact.state == ArtifactState::Stale && !b.has_active_reason(stale) {
                violations.push(
                    QualificationBadgeBindingViolation::ArtifactStateWithoutReason {
                        entry_id: b.entry_id.clone(),
                        reason: stale,
                    },
                );
            }
            if artifact.state == ArtifactState::Missing && !b.has_active_reason(missing) {
                violations.push(
                    QualificationBadgeBindingViolation::ArtifactStateWithoutReason {
                        entry_id: b.entry_id.clone(),
                        reason: missing,
                    },
                );
            }
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<QualificationBadgeBindingViolation>) {
        let covered: BTreeSet<String> =
            self.bindings.iter().map(|b| b.family_ref.clone()).collect();
        for declared in &self.release_blocking_family_refs {
            if !covered.contains(declared) {
                violations.push(
                    QualificationBadgeBindingViolation::ReleaseBlockingFamilyUncovered {
                        family_ref: declared.clone(),
                    },
                );
            }
        }
        for b in &self.bindings {
            if b.release_blocking && !self.release_blocking_family_refs.contains(&b.family_ref) {
                violations.push(
                    QualificationBadgeBindingViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: b.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<QualificationBadgeBindingViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(QualificationBadgeBindingViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(QualificationBadgeBindingViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(
                QualificationBadgeBindingViolation::PromotionDecisionInconsistent {
                    declared: self.promotion.decision,
                    computed,
                },
            );
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                QualificationBadgeBindingViolation::PromotionBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_claim_ids() {
            violations.push(
                QualificationBadgeBindingViolation::PromotionBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// A validation violation for the M5 qualification-row badge binding register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualificationBadgeBindingViolation {
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
    /// The register has no bindings.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Binding or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A binding id appears more than once.
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
        reason: BindingNarrowingReason,
    },
    /// A bound artifact carries the wrong artifact kind.
    ArtifactKindMismatch {
        /// Binding id.
        entry_id: String,
        /// Expected kind.
        expected: BindingArtifactKind,
        /// Kind found.
        actual: BindingArtifactKind,
    },
    /// A present artifact has no ref.
    ArtifactRefMissing {
        /// Binding id.
        entry_id: String,
        /// Artifact kind.
        kind: BindingArtifactKind,
    },
    /// A binding renders the badge on a surface twice.
    DuplicateSurface {
        /// Binding id.
        entry_id: String,
        /// Duplicated surface.
        surface: BadgeSurface,
    },
    /// A binding does not render the badge on a product-truth surface.
    TruthSurfaceUncovered {
        /// Binding id.
        entry_id: String,
        /// Uncovered surface.
        surface: BadgeSurface,
    },
    /// The badge label does not equal the published label.
    BadgeLabelMismatch {
        /// Binding id.
        entry_id: String,
        /// Badge label.
        badge: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// The badge support class does not equal the binding support class.
    BadgeSupportClassMismatch {
        /// Binding id.
        entry_id: String,
    },
    /// The badge freshness state does not equal the proof packet's SLO state.
    BadgeFreshnessMismatch {
        /// Binding id.
        entry_id: String,
        /// Badge freshness state.
        badge: FreshnessSloState,
        /// Packet SLO state.
        packet: FreshnessSloState,
    },
    /// The badge does not disclose its freshness state.
    FreshnessNotDisclosed {
        /// Binding id.
        entry_id: String,
    },
    /// The badge carries caveats it does not disclose.
    CaveatsNotDisclosed {
        /// Binding id.
        entry_id: String,
    },
    /// A limited support class records no compatibility caveat.
    LimitedWithoutCaveat {
        /// Binding id.
        entry_id: String,
    },
    /// The row's published label is wider than the backed claim's label.
    RowWiderThanClaim {
        /// Binding id.
        entry_id: String,
        /// Claim label.
        claim: StableClaimLevel,
        /// Row label.
        row: StableClaimLevel,
    },
    /// The badge's published label is wider than the row it binds.
    BadgePublishedWiderThanRow {
        /// Binding id.
        entry_id: String,
        /// Row label.
        row: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A row narrowed below the cutline does not name the inherited reason.
    RowNarrowedWithoutReason {
        /// Binding id.
        entry_id: String,
    },
    /// A published badge does not equal the row's label.
    HeldLabelNotEqualRow {
        /// Binding id.
        entry_id: String,
        /// Row label.
        row: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A published badge does not publish at or above the cutline.
    PublishedStateNotStable {
        /// Binding id.
        entry_id: String,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A published badge carries active narrowing reasons.
    HeldWithActiveGap {
        /// Binding id.
        entry_id: String,
    },
    /// A published badge has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Binding id.
        entry_id: String,
    },
    /// A published badge rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Binding id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A published badge rides a stale or missing binding artifact.
    HeldWithStaleArtifact {
        /// Binding id.
        entry_id: String,
        /// Artifact kind.
        kind: BindingArtifactKind,
    },
    /// A published badge lacks owner sign-off.
    HeldWithoutSignoff {
        /// Binding id.
        entry_id: String,
    },
    /// A narrowing badge did not drop below the cutline.
    NarrowedButPublishedStable {
        /// Binding id.
        entry_id: String,
        /// Binding state.
        state: BindingState,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A narrowing badge names no active reason.
    NarrowingWithoutReason {
        /// Binding id.
        entry_id: String,
        /// Binding state.
        state: BindingState,
    },
    /// A binding state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Binding id.
        entry_id: String,
        /// Binding state.
        state: BindingState,
    },
    /// A stale or missing artifact does not name its matching reason.
    ArtifactStateWithoutReason {
        /// Binding id.
        entry_id: String,
        /// Reason the artifact state requires.
        reason: BindingNarrowingReason,
    },
    /// A release-blocking family ref has no covering binding.
    ReleaseBlockingFamilyUncovered {
        /// Family ref.
        family_ref: String,
    },
    /// A release-blocking binding is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Binding id.
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
    /// The summary counts disagree with the bindings.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Binding id.
        entry_id: String,
    },
}

impl fmt::Display for QualificationBadgeBindingViolation {
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
            Self::EmptyRegister => write!(f, "register has no bindings"),
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
            Self::ArtifactKindMismatch {
                entry_id,
                expected,
                actual,
            } => write!(
                f,
                "binding {entry_id} artifact expected kind {} but found {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::ArtifactRefMissing { entry_id, kind } => write!(
                f,
                "binding {entry_id} artifact {} has no ref",
                kind.as_str()
            ),
            Self::DuplicateSurface { entry_id, surface } => write!(
                f,
                "binding {entry_id} renders on surface {} twice",
                surface.as_str()
            ),
            Self::TruthSurfaceUncovered { entry_id, surface } => write!(
                f,
                "binding {entry_id} does not render the badge on truth surface {}",
                surface.as_str()
            ),
            Self::BadgeLabelMismatch {
                entry_id,
                badge,
                published,
            } => write!(
                f,
                "binding {entry_id} badge label {badge:?} does not equal published {published:?}"
            ),
            Self::BadgeSupportClassMismatch { entry_id } => {
                write!(f, "binding {entry_id} badge support class mismatch")
            }
            Self::BadgeFreshnessMismatch {
                entry_id,
                badge,
                packet,
            } => write!(
                f,
                "binding {entry_id} badge freshness {badge:?} does not equal packet {packet:?}"
            ),
            Self::FreshnessNotDisclosed { entry_id } => {
                write!(f, "binding {entry_id} badge does not disclose freshness")
            }
            Self::CaveatsNotDisclosed { entry_id } => {
                write!(f, "binding {entry_id} badge does not disclose its caveats")
            }
            Self::LimitedWithoutCaveat { entry_id } => {
                write!(f, "binding {entry_id} is limited without a caveat")
            }
            Self::RowWiderThanClaim {
                entry_id,
                claim,
                row,
            } => write!(
                f,
                "binding {entry_id} row label {row:?} is wider than claim {claim:?}"
            ),
            Self::BadgePublishedWiderThanRow {
                entry_id,
                row,
                published,
            } => write!(
                f,
                "binding {entry_id} badge published {published:?} is wider than row {row:?}"
            ),
            Self::RowNarrowedWithoutReason { entry_id } => write!(
                f,
                "binding {entry_id} row narrowed without qualification_row_narrowed reason"
            ),
            Self::HeldLabelNotEqualRow {
                entry_id,
                row,
                published,
            } => write!(
                f,
                "binding {entry_id} held label {published:?} does not equal row {row:?}"
            ),
            Self::PublishedStateNotStable {
                entry_id,
                published,
            } => write!(
                f,
                "binding {entry_id} is published but publishes {published:?} below the cutline"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "binding {entry_id} publishes with an active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "binding {entry_id} publishes without a fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => write!(
                f,
                "binding {entry_id} publishes on stale packet {slo_state:?}"
            ),
            Self::HeldWithStaleArtifact { entry_id, kind } => write!(
                f,
                "binding {entry_id} publishes on stale or missing {}",
                kind.as_str()
            ),
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "binding {entry_id} publishes without owner signoff")
            }
            Self::NarrowedButPublishedStable {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "binding {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "binding {entry_id} state {state:?} narrows without active reason"
            ),
            Self::StateReasonIncoherent { entry_id, state } => write!(
                f,
                "binding {entry_id} state {state:?} is incoherent with its active reasons"
            ),
            Self::ArtifactStateWithoutReason { entry_id, reason } => write!(
                f,
                "binding {entry_id} stale/missing artifact without {} reason",
                reason.as_str()
            ),
            Self::ReleaseBlockingFamilyUncovered { family_ref } => write!(
                f,
                "release-blocking family {family_ref} has no covering binding"
            ),
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking binding {entry_id} is not declared in release_blocking_family_refs"
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => write!(
                f,
                "promotion {declared:?} disagrees with computed {computed:?}"
            ),
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with bindings"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "binding {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for QualificationBadgeBindingViolation {}

/// Loads the embedded M5 qualification-row badge binding register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`QualificationBadgeBindingRegister`].
pub fn current_m5_qualification_badge_bindings(
) -> Result<QualificationBadgeBindingRegister, serde_json::Error> {
    serde_json::from_str(BIND_M5_QUALIFICATION_BADGE_BINDINGS_JSON)
}

#[cfg(test)]
mod tests;
