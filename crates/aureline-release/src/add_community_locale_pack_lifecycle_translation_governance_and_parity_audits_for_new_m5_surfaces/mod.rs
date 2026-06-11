//! Typed community locale-pack governance register, translation disclosure, and downgrade automation.
//!
//! This module freezes the canonical control surface for the lifecycle and
//! translation governance of community locale packs covering the new release-line
//! surfaces. Where the depth-claim manifest speaks for the *depth claim* each
//! feature family publishes and the generated-artifact lineage register speaks for
//! the *lineage surface* every generated-artifact family exposes, this register
//! speaks for the *locale-pack lane* every pack channel exposes — core shipped
//! locales, community packs, partner packs, and machine-assisted packs. Each
//! [`LocalePackLane`] binds one locale pack to:
//!
//! - the stable claim it backs ([`LocalePackLane::claim_ref`],
//!   [`LocalePackLane::claim_label`]),
//! - a translation-governance scorecard ([`LocalePackLane::scorecard`]) of one
//!   [`GovernanceCell`] per [`GovernanceDimension`], so string coverage,
//!   critical-state coverage, terminology, translation review, locale parity, and
//!   source sync are each an explicit, inspectable grade,
//! - the translation governance it discloses ([`LocalePackLane::governance`]): the
//!   maintainer, the source-string origin, the [`TrustTier`], the localized string
//!   sets, and whether untranslated-string fallback is disclosed to the user,
//! - an owner manifest ([`LocalePackLane::owner_signoff`]) recording who signed the
//!   claim,
//! - an explicit rollback/downgrade automation record
//!   ([`LocalePackLane::downgrade_automation`]) binding the lane to a verified
//!   base-locale fallback plan and the trigger and floor it narrows to,
//! - the overall pack state earned ([`PackState`]), the active narrowing reasons
//!   ([`NarrowingReason`]), and the effective label after narrowing
//!   ([`LocalePackLane::published_label`]),
//! - a [`ProofPacket`] (reused from the stable claim manifest) and its freshness
//!   SLO, plus an optional waiver.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a lane that may publish a Stable claim and one that must narrow below
//! it. The [`LocalePackStopRule`] set names the closed conditions that gate
//! promotion — one per [`NarrowingReason`] — and
//! [`LocalePackGovernanceRegister::promotion`] records the proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/m5/add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces.json`
//! and embedded here, so this typed consumer and the CI gate agree on every
//! locale-pack lane without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no translated string bodies, raw provider payloads, signatures, or
//! credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported register schema version.
pub const LOCALE_PACK_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const LOCALE_PACK_GOVERNANCE_RECORD_KIND: &str =
    "add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces";

/// Repo-relative path to the checked-in register.
pub const LOCALE_PACK_GOVERNANCE_PATH: &str =
    "artifacts/release/m5/add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces.json";

/// Embedded checked-in register JSON.
pub const LOCALE_PACK_GOVERNANCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces.json"
));

/// Pack channel a locale-pack lane governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackChannel {
    /// First-party shipped locale.
    CoreLocale,
    /// Community-contributed locale pack.
    CommunityPack,
    /// Partner/vendor-maintained locale pack.
    PartnerPack,
    /// Machine-translation-seeded, human-reviewed locale pack.
    MachineAssisted,
}

impl PackChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CoreLocale,
        Self::CommunityPack,
        Self::PartnerPack,
        Self::MachineAssisted,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreLocale => "core_locale",
            Self::CommunityPack => "community_pack",
            Self::PartnerPack => "partner_pack",
            Self::MachineAssisted => "machine_assisted",
        }
    }
}

/// One dimension of the translation-governance scorecard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDimension {
    /// All source strings for the surface are translated.
    StringCoverage,
    /// Error, consent, destructive, and recovery strings are translated.
    CriticalStateCoverage,
    /// Glossary/terminology is consistent with the locale's term base.
    Terminology,
    /// A human accuracy review/sign-off covers the pack.
    TranslationReview,
    /// A parity audit confirms no drift from the source surface.
    LocaleParity,
    /// The pack is synced with the current source-string revision.
    SourceSync,
}

impl GovernanceDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StringCoverage,
        Self::CriticalStateCoverage,
        Self::Terminology,
        Self::TranslationReview,
        Self::LocaleParity,
        Self::SourceSync,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StringCoverage => "string_coverage",
            Self::CriticalStateCoverage => "critical_state_coverage",
            Self::Terminology => "terminology",
            Self::TranslationReview => "translation_review",
            Self::LocaleParity => "locale_parity",
            Self::SourceSync => "source_sync",
        }
    }

    /// The narrowing reason a non-passing, non-waived cell must name, given the
    /// cell's [`DimensionGrade`].
    pub const fn reason_for_grade(self, grade: DimensionGrade) -> Option<NarrowingReason> {
        match grade {
            DimensionGrade::Missing => Some(NarrowingReason::GovernanceDimensionMissing),
            DimensionGrade::Fail | DimensionGrade::Partial => {
                Some(NarrowingReason::GovernanceDimensionFailed)
            }
            DimensionGrade::Pass | DimensionGrade::Waived => None,
        }
    }
}

/// The grade earned on one governance dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DimensionGrade {
    /// The dimension fully passes.
    Pass,
    /// The dimension partially passes; remediation is required.
    Partial,
    /// The dimension fails.
    Fail,
    /// Held provisionally under an active, unexpired waiver.
    Waived,
    /// The dimension has no governance evidence at all.
    Missing,
}

impl DimensionGrade {
    /// Every grade, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Pass,
        Self::Partial,
        Self::Fail,
        Self::Waived,
        Self::Missing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Partial => "partial",
            Self::Fail => "fail",
            Self::Waived => "waived",
            Self::Missing => "missing",
        }
    }

    /// Whether a cell in this grade lets the lane hold its claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Pass | Self::Waived)
    }
}

/// Trust tier of the maintainer/source behind a locale pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustTier {
    /// First-party Aureline localization.
    FirstParty,
    /// A verified partner/vendor maintainer.
    VerifiedPartner,
    /// A community-sourced pack maintainer.
    Community,
    /// An unverified or untrusted maintainer.
    Untrusted,
}

impl TrustTier {
    /// Every tier, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FirstParty,
        Self::VerifiedPartner,
        Self::Community,
        Self::Untrusted,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::VerifiedPartner => "verified_partner",
            Self::Community => "community",
            Self::Untrusted => "untrusted",
        }
    }
}

/// Overall lifecycle/governance state a locale-pack lane earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackState {
    /// Every dimension passes, fallback is disclosed, the owner manifest is
    /// signed, and rollback/downgrade automation is defined and verified.
    Certified,
    /// One or more governance dimensions failed, are partial, or are missing —
    /// including a failed locale-parity audit.
    ParityDrifted,
    /// The proof packet has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers a
    /// recorded gap.
    OnWaiver,
    /// Rollback/downgrade automation is undefined or its base-locale fallback plan
    /// is unverified.
    RollbackUndefined,
    /// The owner manifest is unsigned.
    OwnerUnsigned,
}

impl PackState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Certified,
        Self::ParityDrifted,
        Self::Stale,
        Self::OnWaiver,
        Self::RollbackUndefined,
        Self::OwnerUnsigned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::ParityDrifted => "parity_drifted",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::RollbackUndefined => "rollback_undefined",
            Self::OwnerUnsigned => "owner_unsigned",
        }
    }

    /// Whether the state lets a lane carry the claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Certified | Self::OnWaiver)
    }

    /// Whether the state forces the lane below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a locale-pack lane claim narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// A governance dimension failed or is only partial.
    GovernanceDimensionFailed,
    /// A governance dimension is missing.
    GovernanceDimensionMissing,
    /// The proof packet is missing.
    ProofPacketMissing,
    /// The proof packet is stale.
    ProofPacketStale,
    /// The owner manifest is unsigned.
    OwnerManifestUnsigned,
    /// The base-locale fallback rollback plan is unverified.
    RollbackPlanUnverified,
    /// The downgrade automation is undefined.
    DowngradeAutomationUndefined,
    /// A waiver the lane relied on has expired.
    WaiverExpired,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::GovernanceDimensionFailed,
        Self::GovernanceDimensionMissing,
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::OwnerManifestUnsigned,
        Self::RollbackPlanUnverified,
        Self::DowngradeAutomationUndefined,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GovernanceDimensionFailed => "governance_dimension_failed",
            Self::GovernanceDimensionMissing => "governance_dimension_missing",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::OwnerManifestUnsigned => "owner_manifest_unsigned",
            Self::RollbackPlanUnverified => "rollback_plan_unverified",
            Self::DowngradeAutomationUndefined => "downgrade_automation_undefined",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether this reason marks a rollback/downgrade automation gap.
    pub const fn is_rollback_gap(self) -> bool {
        matches!(
            self,
            Self::RollbackPlanUnverified | Self::DowngradeAutomationUndefined
        )
    }

    /// Whether this reason marks a governance-dimension gap.
    pub const fn is_dimension_gap(self) -> bool {
        matches!(
            self,
            Self::GovernanceDimensionFailed | Self::GovernanceDimensionMissing
        )
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopAction {
    /// Hold promotion until the condition clears.
    HoldPromotion,
    /// Narrow the claim below the cutline.
    NarrowLabel,
    /// Remediate the failing or missing translation-governance dimension.
    RemediateTranslation,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Obtain the required owner-manifest sign-off.
    RequestOwnerSignoff,
    /// Verify the base-locale fallback rollback plan.
    VerifyRollbackPlan,
    /// Define the downgrade automation.
    DefineDowngradeAutomation,
    /// Renew the expired waiver.
    RenewWaiver,
}

impl StopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::HoldPromotion,
        Self::NarrowLabel,
        Self::RemediateTranslation,
        Self::RefreshProofPacket,
        Self::RequestOwnerSignoff,
        Self::VerifyRollbackPlan,
        Self::DefineDowngradeAutomation,
        Self::RenewWaiver,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowLabel => "narrow_label",
            Self::RemediateTranslation => "remediate_translation",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::VerifyRollbackPlan => "verify_rollback_plan",
            Self::DefineDowngradeAutomation => "define_downgrade_automation",
            Self::RenewWaiver => "renew_waiver",
        }
    }
}

/// What triggers a lane's automated rollback/downgrade to the base locale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeTrigger {
    /// Fires when the proof packet goes stale.
    ProofStale,
    /// Fires when the locale-parity audit regresses.
    ParityDrifted,
    /// Fires when owner sign-off is revoked.
    OwnerRevoked,
    /// Operator-driven manual downgrade.
    Manual,
}

impl DowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProofStale,
        Self::ParityDrifted,
        Self::OwnerRevoked,
        Self::Manual,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::ParityDrifted => "parity_drifted",
            Self::OwnerRevoked => "owner_revoked",
            Self::Manual => "manual",
        }
    }
}

/// The defined/verified state of a lane's rollback/downgrade automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationState {
    /// The automation is defined and its base-locale fallback plan is verified.
    Defined,
    /// The automation is defined but its fallback plan is unverified.
    Unverified,
    /// The automation is undefined.
    Undefined,
}

impl AutomationState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Defined, Self::Unverified, Self::Undefined];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Defined => "defined",
            Self::Unverified => "unverified",
            Self::Undefined => "undefined",
        }
    }

    /// Whether the automation is defined and verified, letting a lane hold a
    /// Stable claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Defined)
    }
}

/// One cell of the translation-governance scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceCell {
    /// The governance dimension this cell speaks for.
    pub dimension: GovernanceDimension,
    /// The grade earned for the dimension.
    pub grade: DimensionGrade,
    /// Ref to the dimension's evidence. Empty only on a missing cell.
    pub evidence_ref: String,
}

/// The disclosed translation governance of a locale pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranslationGovernance {
    /// Stable ref to the pack maintainer.
    pub maintainer_ref: String,
    /// Stable ref to the source-string origin the pack localizes.
    pub source_ref: String,
    /// Trust tier of the maintainer/source.
    pub trust_tier: TrustTier,
    /// Refs to the localized string sets the pack covers.
    #[serde(default)]
    pub string_set_refs: Vec<String>,
    /// Whether untranslated-string fallback to the base locale is disclosed to
    /// the user.
    pub fallback_disclosed: bool,
}

/// A lane's rollback/downgrade automation, falling back to the base locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DowngradeAutomation {
    /// Stable ref to the automation definition.
    pub automation_ref: String,
    /// Ref to the base-locale fallback rollback plan the automation drives.
    pub rollback_plan_ref: String,
    /// What triggers the automated downgrade.
    pub trigger: DowngradeTrigger,
    /// The lifecycle label the automation narrows the lane to.
    pub target_floor: StableClaimLevel,
    /// The defined/verified state of the automation.
    pub state: AutomationState,
    /// Whether the base-locale fallback plan has been verified end-to-end.
    pub rollback_verified: bool,
}

/// One locale-pack governance stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalePackStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched lane fires this rule.
    pub trigger_reason: NarrowingReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: StopAction,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One community locale-pack lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalePackLane {
    /// Stable lane id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The pack channel this lane governs.
    pub pack_channel: PackChannel,
    /// The pack ref this entry speaks about.
    pub pack_ref: String,
    /// Reviewable one-line statement of the pack.
    pub pack_summary: String,
    /// Whether the pack is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this lane backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes.
    pub claim_label: StableClaimLevel,
    /// Overall pack state earned for the lane.
    pub pack_state: PackState,
    /// The governance scorecard: one cell per [`GovernanceDimension`].
    pub scorecard: Vec<GovernanceCell>,
    /// The disclosed translation governance of the pack.
    pub governance: TranslationGovernance,
    /// The rollback/downgrade automation backing the lane.
    pub downgrade_automation: DowngradeAutomation,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner manifest sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the lane below its claim label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The lifecycle label the lane effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this lane's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the lane carries this posture.
    pub rationale: String,
}

impl LocalePackLane {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the lane's state lets it carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.pack_state.holds_label()
    }

    /// True when a narrowing reason is active on the lane.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// Returns the cell registered for `dimension`, if any.
    pub fn cell(&self, dimension: GovernanceDimension) -> Option<&GovernanceCell> {
        self.scorecard
            .iter()
            .find(|cell| cell.dimension == dimension)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalePackGovernanceSummary {
    /// Total number of locale-pack lanes.
    pub total_entries: usize,
    /// Distinct claims covered.
    pub total_claims: usize,
    /// Lanes publishing a label at or above the cutline.
    pub entries_certified: usize,
    /// Lanes narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Lanes holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Lanes carrying a governance-dimension gap (failed or missing dimension).
    pub entries_with_dimension_gap: usize,
    /// Lanes carrying an owner-manifest-unsigned reason.
    pub entries_with_owner_gap: usize,
    /// Lanes carrying a rollback/downgrade automation gap.
    pub entries_with_rollback_gap: usize,
    /// Lanes whose untranslated-string fallback is not disclosed.
    pub entries_fallback_undisclosed: usize,
    /// Total release-blocking lanes.
    pub release_blocking_total: usize,
    /// Release-blocking lanes publishing a label at or above the cutline.
    pub release_blocking_certified: usize,
    /// Release-blocking lanes narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Core-locale lanes.
    pub core_locale_entries: usize,
    /// Community-pack lanes.
    pub community_pack_entries: usize,
    /// Partner-pack lanes.
    pub partner_pack_entries: usize,
    /// Machine-assisted lanes.
    pub machine_assisted_entries: usize,
    /// First-party-trust lanes.
    pub first_party_entries: usize,
    /// Verified-partner-trust lanes.
    pub verified_partner_entries: usize,
    /// Community-trust lanes.
    pub community_entries: usize,
    /// Untrusted lanes.
    pub untrusted_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all lanes.
    pub total_active_narrowing_reasons: usize,
    /// Total governance cells across all lanes.
    pub total_governance_cells: usize,
    /// Cells graded `pass`.
    pub cells_pass: usize,
    /// Cells graded `partial`.
    pub cells_partial: usize,
    /// Cells graded `fail`.
    pub cells_fail: usize,
    /// Cells graded `waived`.
    pub cells_waived: usize,
    /// Cells graded `missing`.
    pub cells_missing: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackExportRow {
    /// Stable lane id.
    pub entry_id: String,
    /// The pack channel this lane governs.
    pub pack_channel: PackChannel,
    /// The pack ref this entry speaks about.
    pub pack_ref: String,
    /// Whether the lane is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this lane backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the lane publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall pack state earned.
    pub pack_state: PackState,
    /// Trust tier of the maintainer/source.
    pub trust_tier: TrustTier,
    /// Whether untranslated-string fallback is disclosed to the user.
    pub fallback_disclosed: bool,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Rollback/downgrade automation state.
    pub automation_state: AutomationState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackExportProjection {
    /// Register identifier.
    pub manifest_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<LocalePackExportRow>,
}

/// The typed community locale-pack governance register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalePackGovernanceRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub manifest_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this register ingests.
    pub claim_manifest_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed pack-channel vocabulary.
    pub pack_channels: Vec<PackChannel>,
    /// Closed governance-dimension vocabulary.
    pub governance_dimensions: Vec<GovernanceDimension>,
    /// Closed dimension-grade vocabulary.
    pub dimension_grades: Vec<DimensionGrade>,
    /// Closed pack-state vocabulary.
    pub pack_states: Vec<PackState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// Closed downgrade-trigger vocabulary.
    pub downgrade_triggers: Vec<DowngradeTrigger>,
    /// Closed automation-state vocabulary.
    pub automation_states: Vec<AutomationState>,
    /// Closed trust-tier vocabulary.
    pub trust_tiers: Vec<TrustTier>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking pack refs this register must cover.
    pub release_blocking_pack_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<LocalePackStopRule>,
    /// Locale-pack lanes.
    pub rows: Vec<LocalePackLane>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: LocalePackGovernanceSummary,
}

impl LocalePackGovernanceRegister {
    /// Returns the lane registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&LocalePackLane> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the lanes publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&LocalePackLane> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the lanes narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&LocalePackLane> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking lanes.
    pub fn release_blocking_rows(&self) -> Vec<&LocalePackLane> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the lanes for one pack channel.
    pub fn rows_for_channel(&self, channel: PackChannel) -> Vec<&LocalePackLane> {
        self.rows
            .iter()
            .filter(|row| row.pack_channel == channel)
            .collect()
    }

    /// Distinct claims (by claim ref) the register covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched lane carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &LocalePackStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the lanes and stop rules.
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

    /// Lane ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only lanes whose claim is at or above the cutline count: a lane whose
    /// claim is already canonically narrowed is not a *promotion* blocker, it
    /// merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<NarrowingReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_narrowing_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the lanes and stop rules.
    pub fn computed_summary(&self) -> LocalePackGovernanceSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let channel = |channel: PackChannel| self.rows_for_channel(channel).len();
        let trust = |tier: TrustTier| {
            self.rows
                .iter()
                .filter(|row| row.governance.trust_tier == tier)
                .count()
        };
        let with_predicate = |pred: fn(NarrowingReason) -> bool| {
            self.rows
                .iter()
                .filter(|row| row.active_narrowing_reasons.iter().any(|r| pred(*r)))
                .count()
        };
        let with_reason = |reason: NarrowingReason| {
            self.rows
                .iter()
                .filter(|row| row.has_active_reason(reason))
                .count()
        };
        let cell_grade = |grade: DimensionGrade| {
            self.rows
                .iter()
                .flat_map(|row| row.scorecard.iter())
                .filter(|cell| cell.grade == grade)
                .count()
        };
        let release_blocking: Vec<&LocalePackLane> = self.release_blocking_rows();
        LocalePackGovernanceSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_certified: self
                .rows
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            entries_narrowed: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.pack_state == PackState::OnWaiver)
                .count(),
            entries_with_dimension_gap: with_predicate(NarrowingReason::is_dimension_gap),
            entries_with_owner_gap: with_reason(NarrowingReason::OwnerManifestUnsigned),
            entries_with_rollback_gap: with_predicate(NarrowingReason::is_rollback_gap),
            entries_fallback_undisclosed: self
                .rows
                .iter()
                .filter(|row| !row.governance.fallback_disclosed)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_certified: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            core_locale_entries: channel(PackChannel::CoreLocale),
            community_pack_entries: channel(PackChannel::CommunityPack),
            partner_pack_entries: channel(PackChannel::PartnerPack),
            machine_assisted_entries: channel(PackChannel::MachineAssisted),
            first_party_entries: trust(TrustTier::FirstParty),
            verified_partner_entries: trust(TrustTier::VerifiedPartner),
            community_entries: trust(TrustTier::Community),
            untrusted_entries: trust(TrustTier::Untrusted),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_narrowing_reasons: self
                .rows
                .iter()
                .map(|row| row.active_narrowing_reasons.len())
                .sum(),
            total_governance_cells: self.rows.iter().map(|row| row.scorecard.len()).sum(),
            cells_pass: cell_grade(DimensionGrade::Pass),
            cells_partial: cell_grade(DimensionGrade::Partial),
            cells_fail: cell_grade(DimensionGrade::Fail),
            cells_waived: cell_grade(DimensionGrade::Waived),
            cells_missing: cell_grade(DimensionGrade::Missing),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection that downstream surfaces
    /// render instead of cloning status text.
    pub fn support_export_projection(&self) -> LocalePackExportProjection {
        LocalePackExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| LocalePackExportRow {
                    entry_id: row.entry_id.clone(),
                    pack_channel: row.pack_channel,
                    pack_ref: row.pack_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    pack_state: row.pack_state,
                    trust_tier: row.governance.trust_tier,
                    fallback_disclosed: row.governance.fallback_disclosed,
                    slo_state: row.proof_packet.slo_state,
                    automation_state: row.downgrade_automation.state,
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<LocalePackRegisterViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(LocalePackRegisterViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(LocalePackRegisterViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(LocalePackRegisterViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<LocalePackRegisterViolation>) {
        if self.schema_version != LOCALE_PACK_GOVERNANCE_SCHEMA_VERSION {
            violations.push(LocalePackRegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != LOCALE_PACK_GOVERNANCE_RECORD_KIND {
            violations.push(LocalePackRegisterViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("manifest_id", &self.manifest_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(LocalePackRegisterViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.pack_channels != PackChannel::ALL.to_vec() {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "pack_channels",
            });
        }
        if self.governance_dimensions != GovernanceDimension::ALL.to_vec() {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "governance_dimensions",
            });
        }
        if self.dimension_grades != DimensionGrade::ALL.to_vec() {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "dimension_grades",
            });
        }
        if self.pack_states != PackState::ALL.to_vec() {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "pack_states",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.stop_rule_actions != StopAction::ALL.to_vec() {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }
        if self.downgrade_triggers != DowngradeTrigger::ALL.to_vec() {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "downgrade_triggers",
            });
        }
        if self.automation_states != AutomationState::ALL.to_vec() {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "automation_states",
            });
        }
        if self.trust_tiers != TrustTier::ALL.to_vec() {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "trust_tiers",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(LocalePackRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(LocalePackRegisterViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<LocalePackRegisterViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(LocalePackRegisterViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(LocalePackRegisterViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(LocalePackRegisterViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(LocalePackRegisterViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(LocalePackRegisterViolation::NarrowingReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &LocalePackLane,
        violations: &mut Vec<LocalePackRegisterViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("pack_ref", &row.pack_ref),
            ("pack_summary", &row.pack_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("proof_packet.packet_id", &row.proof_packet.packet_id),
            ("proof_packet.packet_ref", &row.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &row.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &row.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
            ("governance.maintainer_ref", &row.governance.maintainer_ref),
            ("governance.source_ref", &row.governance.source_ref),
            (
                "downgrade_automation.automation_ref",
                &row.downgrade_automation.automation_ref,
            ),
            (
                "downgrade_automation.rollback_plan_ref",
                &row.downgrade_automation.rollback_plan_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(LocalePackRegisterViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_scorecard(row, violations);
        self.validate_automation(row, violations);

        // The ceiling: no lane may carry a label wider than the claim's canonical
        // label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(LocalePackRegisterViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(LocalePackRegisterViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(LocalePackRegisterViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A claim whose canonical label is below the cutline forces the lane to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(LocalePackRegisterViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(LocalePackRegisterViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.pack_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed lane publishes exactly the claim's canonical label, carries
            // no active reason, rides a captured within-SLO packet, discloses
            // fallback, is owner-signed, and rides defined-and-verified downgrade
            // automation.
            if row.published_label != row.claim_label {
                violations.push(LocalePackRegisterViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(LocalePackRegisterViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.governance.fallback_disclosed {
                violations.push(LocalePackRegisterViolation::HeldWithoutFallbackDisclosure {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(LocalePackRegisterViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(LocalePackRegisterViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(LocalePackRegisterViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.downgrade_automation.state.holds()
                || !row.downgrade_automation.rollback_verified
            {
                violations.push(
                    LocalePackRegisterViolation::HeldWithoutDowngradeAutomation {
                        entry_id: row.entry_id.clone(),
                        state: row.downgrade_automation.state,
                    },
                );
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(LocalePackRegisterViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.pack_state,
                    published: row.published_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(LocalePackRegisterViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.pack_state,
                });
            }
            // A narrowing lane whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::ProofPacketStale)
            {
                violations.push(LocalePackRegisterViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
            {
                violations.push(LocalePackRegisterViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_scorecard(
        &self,
        row: &LocalePackLane,
        violations: &mut Vec<LocalePackRegisterViolation>,
    ) {
        let mut seen: BTreeSet<GovernanceDimension> = BTreeSet::new();
        for cell in &row.scorecard {
            if !seen.insert(cell.dimension) {
                violations.push(LocalePackRegisterViolation::DuplicateDimension {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A missing cell carries no evidence ref; every other grade must.
            if cell.grade != DimensionGrade::Missing && cell.evidence_ref.trim().is_empty() {
                violations.push(LocalePackRegisterViolation::CellEvidenceMissing {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A waived cell only holds under an unexpired waiver.
            if cell.grade == DimensionGrade::Waived && row.waiver.is_none() {
                violations.push(LocalePackRegisterViolation::WaivedCellWithoutWaiver {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A non-passing, non-waived cell must name its narrowing reason.
            if !cell.grade.holds() {
                if let Some(reason) = cell.dimension.reason_for_grade(cell.grade) {
                    if !row.has_active_reason(reason) {
                        violations.push(LocalePackRegisterViolation::CellReasonNotActive {
                            entry_id: row.entry_id.clone(),
                            dimension: cell.dimension,
                            reason,
                        });
                    }
                }
            }
        }
        // The scorecard must carry exactly one cell per dimension.
        for dimension in GovernanceDimension::ALL {
            if !seen.contains(&dimension) {
                violations.push(LocalePackRegisterViolation::GovernanceIncompleteCoverage {
                    entry_id: row.entry_id.clone(),
                    dimension,
                });
            }
        }
    }

    fn validate_automation(
        &self,
        row: &LocalePackLane,
        violations: &mut Vec<LocalePackRegisterViolation>,
    ) {
        let automation = &row.downgrade_automation;
        // A downgrade narrows the claim, so its floor must be below the cutline.
        if automation.target_floor.is_at_or_above_cutline() {
            violations.push(LocalePackRegisterViolation::DowngradeFloorNotBelowCutline {
                entry_id: row.entry_id.clone(),
                floor: automation.target_floor,
            });
        }
        // An undefined automation must name the undefined reason.
        if automation.state == AutomationState::Undefined
            && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
        {
            violations.push(LocalePackRegisterViolation::AutomationStateWithoutReason {
                entry_id: row.entry_id.clone(),
                state: automation.state,
            });
        }
        // An unverified fallback plan must name a rollback or downgrade reason.
        if !automation.rollback_verified
            && !row.has_active_reason(NarrowingReason::RollbackPlanUnverified)
            && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
        {
            violations.push(
                LocalePackRegisterViolation::RollbackUnverifiedWithoutReason {
                    entry_id: row.entry_id.clone(),
                },
            );
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &LocalePackLane,
        violations: &mut Vec<LocalePackRegisterViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<LocalePackRegisterViolation>,
                               expected: NarrowingReason| {
            violations.push(LocalePackRegisterViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.pack_state,
                expected_reason: expected,
            });
        };

        match row.pack_state {
            PackState::ParityDrifted => {
                if !row.has_active_reason(NarrowingReason::GovernanceDimensionFailed)
                    && !row.has_active_reason(NarrowingReason::GovernanceDimensionMissing)
                {
                    push_incoherent(violations, NarrowingReason::GovernanceDimensionFailed);
                }
            }
            PackState::Stale => {
                if !row.has_active_reason(NarrowingReason::ProofPacketStale) {
                    push_incoherent(violations, NarrowingReason::ProofPacketStale);
                }
            }
            PackState::RollbackUndefined => {
                if !row.has_active_reason(NarrowingReason::RollbackPlanUnverified)
                    && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
                {
                    push_incoherent(violations, NarrowingReason::DowngradeAutomationUndefined);
                }
            }
            PackState::OwnerUnsigned => {
                if !row.has_active_reason(NarrowingReason::OwnerManifestUnsigned) {
                    push_incoherent(violations, NarrowingReason::OwnerManifestUnsigned);
                }
            }
            PackState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(LocalePackRegisterViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.pack_state,
                    });
                }
            }
            PackState::Certified => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<LocalePackRegisterViolation>) {
        let covered: BTreeSet<String> = self.rows.iter().map(|row| row.pack_ref.clone()).collect();
        for declared in &self.release_blocking_pack_refs {
            if !covered.contains(declared) {
                violations.push(LocalePackRegisterViolation::ReleaseBlockingPackUncovered {
                    pack_ref: declared.clone(),
                });
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_pack_refs.contains(&row.pack_ref) {
                violations.push(LocalePackRegisterViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<LocalePackRegisterViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(LocalePackRegisterViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(LocalePackRegisterViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(LocalePackRegisterViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(LocalePackRegisterViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(LocalePackRegisterViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the community locale-pack governance register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalePackRegisterViolation {
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
    /// The register has no lanes.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Lane or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A lane id appears more than once.
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
    /// A narrowing reason has no stop rule watching for it.
    NarrowingReasonWithoutStopRule {
        /// Uncovered reason.
        reason: NarrowingReason,
    },
    /// A lane has two cells for one dimension.
    DuplicateDimension {
        /// Lane id.
        entry_id: String,
        /// Duplicated dimension.
        dimension: GovernanceDimension,
    },
    /// A lane is missing a dimension cell.
    GovernanceIncompleteCoverage {
        /// Lane id.
        entry_id: String,
        /// Uncovered dimension.
        dimension: GovernanceDimension,
    },
    /// A non-missing cell has no evidence ref.
    CellEvidenceMissing {
        /// Lane id.
        entry_id: String,
        /// Dimension.
        dimension: GovernanceDimension,
    },
    /// A waived cell is carried without a waiver.
    WaivedCellWithoutWaiver {
        /// Lane id.
        entry_id: String,
        /// Dimension.
        dimension: GovernanceDimension,
    },
    /// A non-passing cell does not name its narrowing reason.
    CellReasonNotActive {
        /// Lane id.
        entry_id: String,
        /// Dimension.
        dimension: GovernanceDimension,
        /// The reason the cell requires.
        reason: NarrowingReason,
    },
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Lane id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A lane holds a label while the claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Lane id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing state carries no active reason.
    NarrowingWithoutReason {
        /// Lane id.
        entry_id: String,
        /// Pack state.
        state: PackState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Lane id.
        entry_id: String,
        /// Pack state.
        state: PackState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held lane carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Lane id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held lane has active narrowing reasons.
    HeldWithActiveGap {
        /// Lane id.
        entry_id: String,
    },
    /// A held lane does not disclose untranslated-string fallback.
    HeldWithoutFallbackDisclosure {
        /// Lane id.
        entry_id: String,
    },
    /// A held lane has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Lane id.
        entry_id: String,
    },
    /// A held lane rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Lane id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held lane lacks owner-manifest sign-off.
    HeldWithoutSignoff {
        /// Lane id.
        entry_id: String,
    },
    /// A held lane lacks defined-and-verified downgrade automation.
    HeldWithoutDowngradeAutomation {
        /// Lane id.
        entry_id: String,
        /// Automation state.
        state: AutomationState,
    },
    /// The downgrade automation floor is not below the cutline.
    DowngradeFloorNotBelowCutline {
        /// Lane id.
        entry_id: String,
        /// Declared floor.
        floor: StableClaimLevel,
    },
    /// An undefined automation does not name the undefined reason.
    AutomationStateWithoutReason {
        /// Lane id.
        entry_id: String,
        /// Automation state.
        state: AutomationState,
    },
    /// An unverified fallback plan does not name a rollback/downgrade reason.
    RollbackUnverifiedWithoutReason {
        /// Lane id.
        entry_id: String,
    },
    /// A narrowing lane with a breached proof packet does not name the stale reason.
    BreachedPacketWithoutReason {
        /// Lane id.
        entry_id: String,
    },
    /// A narrowing lane with a missing proof packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Lane id.
        entry_id: String,
    },
    /// A pack state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Lane id.
        entry_id: String,
        /// Pack state.
        state: PackState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Lane id.
        entry_id: String,
        /// Pack state.
        state: PackState,
    },
    /// A release-blocking pack ref has no covering lane.
    ReleaseBlockingPackUncovered {
        /// Pack ref.
        pack_ref: String,
    },
    /// A release-blocking lane is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Lane id.
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
    /// The summary counts disagree with the lanes.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Lane id.
        entry_id: String,
    },
}

impl fmt::Display for LocalePackRegisterViolation {
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
            Self::EmptyRegister => write!(f, "register has no lanes"),
            Self::NoStopRules => write!(f, "register has no stop rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id {entry_id}")
            }
            Self::DuplicateStopRuleId { rule_id } => {
                write!(f, "duplicate stop rule id {rule_id}")
            }
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::NarrowingReasonWithoutStopRule { reason } => write!(
                f,
                "narrowing reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::DuplicateDimension {
                entry_id,
                dimension,
            } => write!(
                f,
                "lane {entry_id} has duplicate dimension {}",
                dimension.as_str()
            ),
            Self::GovernanceIncompleteCoverage {
                entry_id,
                dimension,
            } => write!(
                f,
                "lane {entry_id} is missing dimension {}",
                dimension.as_str()
            ),
            Self::CellEvidenceMissing {
                entry_id,
                dimension,
            } => write!(
                f,
                "lane {entry_id} dimension {} has no evidence ref",
                dimension.as_str()
            ),
            Self::WaivedCellWithoutWaiver {
                entry_id,
                dimension,
            } => write!(
                f,
                "lane {entry_id} dimension {} is waived without a waiver",
                dimension.as_str()
            ),
            Self::CellReasonNotActive {
                entry_id,
                dimension,
                reason,
            } => write!(
                f,
                "lane {entry_id} dimension {} requires active reason {}",
                dimension.as_str(),
                reason.as_str()
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "lane {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "lane {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "lane {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "lane {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "lane {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "lane {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutFallbackDisclosure { entry_id } => {
                write!(
                    f,
                    "lane {entry_id} holds stable without disclosing untranslated-string fallback"
                )
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "lane {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "lane {entry_id} holds stable on stale packet {slo_state:?}"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "lane {entry_id} holds stable without owner signoff")
            }
            Self::HeldWithoutDowngradeAutomation { entry_id, state } => write!(
                f,
                "lane {entry_id} holds stable without defined+verified downgrade automation ({state:?})"
            ),
            Self::DowngradeFloorNotBelowCutline { entry_id, floor } => write!(
                f,
                "lane {entry_id} downgrade floor {floor:?} is not below the cutline"
            ),
            Self::AutomationStateWithoutReason { entry_id, state } => write!(
                f,
                "lane {entry_id} automation state {state:?} names no narrowing reason"
            ),
            Self::RollbackUnverifiedWithoutReason { entry_id } => write!(
                f,
                "lane {entry_id} has an unverified fallback plan without a reason"
            ),
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "lane {entry_id} breached packet without proof_packet_stale reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "lane {entry_id} missing packet without proof_packet_missing reason"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "lane {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "lane {entry_id} state {state:?} names no waiver")
            }
            Self::ReleaseBlockingPackUncovered { pack_ref } => {
                write!(f, "release-blocking pack {pack_ref} has no covering lane")
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking lane {entry_id} is not declared in release_blocking_pack_refs"
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => {
                write!(f, "promotion {declared:?} disagrees with computed {computed:?}")
            }
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with lanes"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "lane {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for LocalePackRegisterViolation {}

/// Loads the embedded community locale-pack governance register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`LocalePackGovernanceRegister`].
pub fn current_locale_pack_governance_register(
) -> Result<LocalePackGovernanceRegister, serde_json::Error> {
    serde_json::from_str(LOCALE_PACK_GOVERNANCE_JSON)
}

#[cfg(test)]
mod tests;
