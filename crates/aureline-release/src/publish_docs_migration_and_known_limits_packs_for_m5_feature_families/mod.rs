//! Typed publication-pack register binding the M5 docs packs, migration packs,
//! known-limits packs, and the publication index across the feature families to
//! per-pack readiness scorecards, support-posture disclosure, and downgrade
//! automation.
//!
//! This module freezes the canonical control surface for the M5 publication-pack
//! lanes: the docs packs, the migration packs, the known-limits packs, and the
//! publication index that ties them together across the feature families
//! (notebooks, DB/API, profiler, AI, frameworks, companion, sync, and
//! offboarding). Where the depth-claim manifest speaks for the *depth claim* each
//! feature family publishes and the field-readiness register speaks for the
//! *field-readiness surface* every surface kind exposes, this register speaks for
//! the *publication pack* every pack kind exposes — the docs-pack surface, the
//! migration-pack surface, the known-limits-pack surface, and the
//! publication-index surface. Each [`PublicationPackSurface`] binds one pack to:
//!
//! - the stable claim it backs ([`PublicationPackSurface::claim_ref`],
//!   [`PublicationPackSurface::claim_label`]),
//! - a readiness scorecard ([`PublicationPackSurface::scorecard`]) of one
//!   [`ReadinessCell`] per [`ReadinessDimension`], so docs coverage, migration
//!   completeness, known-limits disclosure, redaction safety, proof freshness, and
//!   locale parity are each an explicit, inspectable grade,
//! - the support posture it discloses ([`PublicationPackSurface::disclosure`]):
//!   the support window, the policy ref, the maintainer [`TrustTier`], the scope
//!   refs, and whether the redaction/provenance posture is disclosed to the
//!   operator,
//! - an owner manifest ([`PublicationPackSurface::owner_signoff`]) recording who
//!   signed the claim,
//! - an explicit downgrade/rollback automation record
//!   ([`PublicationPackSurface::downgrade_automation`]) binding the surface to a
//!   verified frozen-fallback rollback plan and the trigger and floor it narrows
//!   to,
//! - the overall surface state earned ([`SurfaceState`]), the active narrowing
//!   reasons ([`NarrowingReason`]), and the effective label after narrowing
//!   ([`PublicationPackSurface::published_label`]),
//! - a [`ProofPacket`] (reused from the stable claim manifest) and its freshness
//!   SLO, plus an optional waiver.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a surface that may publish a Stable claim and one that must narrow below
//! it. The [`PublicationPackStopRule`] set names the closed conditions that gate
//! promotion — one per [`NarrowingReason`] — and
//! [`PublicationPackRegister::promotion`] records the proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/m5/publish_docs_migration_and_known_limits_packs_for_m5_feature_families.json`
//! and embedded here, so this typed consumer and the CI gate agree on every
//! publication pack without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no pack payloads, rendered docs bodies, signatures, or credential
//! material.

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
pub const PUBLICATION_PACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const PUBLICATION_PACK_RECORD_KIND: &str =
    "publish_docs_migration_and_known_limits_packs_for_m5_feature_families";

/// Repo-relative path to the checked-in register.
pub const PUBLICATION_PACK_PATH: &str =
    "artifacts/release/m5/publish_docs_migration_and_known_limits_packs_for_m5_feature_families.json";

/// Embedded checked-in register JSON.
pub const PUBLICATION_PACK_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/publish_docs_migration_and_known_limits_packs_for_m5_feature_families.json"
));

/// Surface kind a publication pack governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    /// A docs publication pack for a feature family.
    DocsPack,
    /// A migration publication pack for a feature family.
    MigrationPack,
    /// A known-limits publication pack for a feature family.
    KnownLimitsPack,
    /// The publication index tying the packs together across families.
    PublicationIndex,
}

impl SurfaceKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DocsPack,
        Self::MigrationPack,
        Self::KnownLimitsPack,
        Self::PublicationIndex,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsPack => "docs_pack",
            Self::MigrationPack => "migration_pack",
            Self::KnownLimitsPack => "known_limits_pack",
            Self::PublicationIndex => "publication_index",
        }
    }
}

/// One dimension of the readiness scorecard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessDimension {
    /// The docs pack covers every claimed capability of the feature family.
    DocsCoverage,
    /// The migration steps are complete, ordered, and reversible.
    MigrationCompleteness,
    /// The known limits are disclosed and current.
    KnownLimitsDisclosure,
    /// Redaction safety: no credential bodies or raw provider payloads leak.
    RedactionSafety,
    /// The proof-freshness automation is wired and exercised.
    ProofFreshness,
    /// Locale parity holds: the critical states are translated.
    LocaleParity,
}

impl ReadinessDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DocsCoverage,
        Self::MigrationCompleteness,
        Self::KnownLimitsDisclosure,
        Self::RedactionSafety,
        Self::ProofFreshness,
        Self::LocaleParity,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsCoverage => "docs_coverage",
            Self::MigrationCompleteness => "migration_completeness",
            Self::KnownLimitsDisclosure => "known_limits_disclosure",
            Self::RedactionSafety => "redaction_safety",
            Self::ProofFreshness => "proof_freshness",
            Self::LocaleParity => "locale_parity",
        }
    }

    /// The narrowing reason a non-passing, non-waived cell must name, given the
    /// cell's [`DimensionGrade`].
    pub const fn reason_for_grade(self, grade: DimensionGrade) -> Option<NarrowingReason> {
        match grade {
            DimensionGrade::Missing => Some(NarrowingReason::ReadinessDimensionMissing),
            DimensionGrade::Fail | DimensionGrade::Partial => {
                Some(NarrowingReason::ReadinessDimensionFailed)
            }
            DimensionGrade::Pass | DimensionGrade::Waived => None,
        }
    }
}

/// The grade earned on one readiness dimension.
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
    /// The dimension has no readiness evidence at all.
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

    /// Whether a cell in this grade lets the surface hold its claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Pass | Self::Waived)
    }
}

/// Trust tier of the maintainer behind a publication pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustTier {
    /// First-party Aureline-maintained surface.
    FirstParty,
    /// A verified partner/vendor-maintained surface.
    VerifiedPartner,
    /// A community-maintained surface.
    Community,
    /// A scaffolded/generated surface whose support posture must be disclosed.
    Generated,
}

impl TrustTier {
    /// Every tier, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FirstParty,
        Self::VerifiedPartner,
        Self::Community,
        Self::Generated,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::VerifiedPartner => "verified_partner",
            Self::Community => "community",
            Self::Generated => "generated",
        }
    }
}

/// Overall readiness/lifecycle state a publication pack earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceState {
    /// Every dimension passes, the support posture is disclosed, the owner
    /// manifest is signed, and downgrade automation is defined and verified.
    Certified,
    /// One or more readiness dimensions failed, are partial, or are missing.
    ReadinessRegressed,
    /// The proof packet has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers a
    /// recorded gap.
    OnWaiver,
    /// Downgrade automation is undefined or its frozen-fallback rollback plan is
    /// unverified.
    AutomationUndefined,
    /// The owner manifest is unsigned.
    OwnerUnsigned,
}

impl SurfaceState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Certified,
        Self::ReadinessRegressed,
        Self::Stale,
        Self::OnWaiver,
        Self::AutomationUndefined,
        Self::OwnerUnsigned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::ReadinessRegressed => "readiness_regressed",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::AutomationUndefined => "automation_undefined",
            Self::OwnerUnsigned => "owner_unsigned",
        }
    }

    /// Whether the state lets a surface carry the claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Certified | Self::OnWaiver)
    }

    /// Whether the state forces the surface below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a publication pack claim narrows or a stop rule
/// fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// A readiness dimension failed or is only partial.
    ReadinessDimensionFailed,
    /// A readiness dimension is missing.
    ReadinessDimensionMissing,
    /// The proof packet is missing.
    ProofPacketMissing,
    /// The proof packet is stale.
    ProofPacketStale,
    /// The owner manifest is unsigned.
    OwnerManifestUnsigned,
    /// The frozen-fallback rollback plan is unverified.
    RollbackPlanUnverified,
    /// The downgrade automation is undefined.
    DowngradeAutomationUndefined,
    /// A waiver the surface relied on has expired.
    WaiverExpired,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ReadinessDimensionFailed,
        Self::ReadinessDimensionMissing,
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
            Self::ReadinessDimensionFailed => "readiness_dimension_failed",
            Self::ReadinessDimensionMissing => "readiness_dimension_missing",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::OwnerManifestUnsigned => "owner_manifest_unsigned",
            Self::RollbackPlanUnverified => "rollback_plan_unverified",
            Self::DowngradeAutomationUndefined => "downgrade_automation_undefined",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether this reason marks a downgrade-automation gap.
    pub const fn is_automation_gap(self) -> bool {
        matches!(
            self,
            Self::RollbackPlanUnverified | Self::DowngradeAutomationUndefined
        )
    }

    /// Whether this reason marks a readiness-dimension gap.
    pub const fn is_dimension_gap(self) -> bool {
        matches!(
            self,
            Self::ReadinessDimensionFailed | Self::ReadinessDimensionMissing
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
    /// Remediate the failing or missing readiness dimension.
    RemediateReadiness,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Obtain the required owner-manifest sign-off.
    RequestOwnerSignoff,
    /// Verify the frozen-fallback rollback plan.
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
        Self::RemediateReadiness,
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
            Self::RemediateReadiness => "remediate_readiness",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::VerifyRollbackPlan => "verify_rollback_plan",
            Self::DefineDowngradeAutomation => "define_downgrade_automation",
            Self::RenewWaiver => "renew_waiver",
        }
    }
}

/// What triggers a surface's automated downgrade to a frozen floor label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationTrigger {
    /// Fires when the proof packet goes stale.
    ProofStale,
    /// Fires when a readiness dimension regresses.
    ReadinessRegressed,
    /// Fires when owner sign-off is revoked.
    OwnerRevoked,
    /// Operator-driven manual downgrade.
    Manual,
}

impl AutomationTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProofStale,
        Self::ReadinessRegressed,
        Self::OwnerRevoked,
        Self::Manual,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::ReadinessRegressed => "readiness_regressed",
            Self::OwnerRevoked => "owner_revoked",
            Self::Manual => "manual",
        }
    }
}

/// The defined/verified state of a surface's downgrade automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationState {
    /// The automation is defined and its frozen-fallback rollback plan is verified.
    Defined,
    /// The automation is defined but its frozen-fallback rollback plan is
    /// unverified.
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

    /// Whether the automation is defined and verified, letting a surface hold a
    /// Stable claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Defined)
    }
}

/// One cell of the readiness scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadinessCell {
    /// The readiness dimension this cell speaks for.
    pub dimension: ReadinessDimension,
    /// The grade earned for the dimension.
    pub grade: DimensionGrade,
    /// Ref to the dimension's evidence. Empty only on a missing cell.
    pub evidence_ref: String,
}

/// The disclosed support posture of a publication pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportDisclosure {
    /// Stable ref to the support window the surface commits to.
    pub support_window_ref: String,
    /// Stable ref to the docs/migration/known-limits publication policy the surface rides.
    pub policy_ref: String,
    /// Trust tier of the maintainer.
    pub trust_tier: TrustTier,
    /// Refs to the release-line scopes the surface covers.
    #[serde(default)]
    pub scope_refs: Vec<String>,
    /// Whether the redaction/provenance posture of the surface is disclosed to the
    /// operator.
    pub redaction_disclosed: bool,
}

/// A surface's downgrade automation, falling back to a frozen floor label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DowngradeAutomation {
    /// Stable ref to the downgrade-automation definition.
    pub automation_ref: String,
    /// Ref to the frozen-fallback rollback plan the automation drives.
    pub rollback_plan_ref: String,
    /// What triggers the automated downgrade.
    pub trigger: AutomationTrigger,
    /// The lifecycle label the downgrade narrows the surface to.
    pub target_floor: StableClaimLevel,
    /// The defined/verified state of the automation.
    pub state: AutomationState,
    /// Whether the frozen-fallback rollback plan has been verified end-to-end.
    pub rollback_verified: bool,
}

/// One publication-pack stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationPackStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched surface fires this rule.
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

/// One publication pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationPackSurface {
    /// Stable surface id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The surface kind this surface governs.
    pub surface_kind: SurfaceKind,
    /// The surface ref this entry speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the surface.
    pub surface_summary: String,
    /// Whether the surface is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this surface backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes.
    pub claim_label: StableClaimLevel,
    /// Overall surface state earned for the surface.
    pub surface_state: SurfaceState,
    /// The readiness scorecard: one cell per [`ReadinessDimension`].
    pub scorecard: Vec<ReadinessCell>,
    /// The disclosed support posture of the surface.
    pub disclosure: SupportDisclosure,
    /// The downgrade automation backing the surface.
    pub downgrade_automation: DowngradeAutomation,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner manifest sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the surface below its claim label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The lifecycle label the surface effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this surface's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the surface carries this posture.
    pub rationale: String,
}

impl PublicationPackSurface {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the surface's state lets it carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.surface_state.holds_label()
    }

    /// True when a narrowing reason is active on the surface.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// Returns the cell registered for `dimension`, if any.
    pub fn cell(&self, dimension: ReadinessDimension) -> Option<&ReadinessCell> {
        self.scorecard
            .iter()
            .find(|cell| cell.dimension == dimension)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationPackSummary {
    /// Total number of publication packs.
    pub total_entries: usize,
    /// Distinct claims covered.
    pub total_claims: usize,
    /// Surfaces publishing a label at or above the cutline.
    pub entries_certified: usize,
    /// Surfaces narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Surfaces holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Surfaces carrying a readiness-dimension gap (failed or missing dimension).
    pub entries_with_dimension_gap: usize,
    /// Surfaces carrying an owner-manifest-unsigned reason.
    pub entries_with_owner_gap: usize,
    /// Surfaces carrying a downgrade-automation gap.
    pub entries_with_automation_gap: usize,
    /// Surfaces whose redaction/provenance posture is not disclosed.
    pub entries_redaction_undisclosed: usize,
    /// Total release-blocking surfaces.
    pub release_blocking_total: usize,
    /// Release-blocking surfaces publishing a label at or above the cutline.
    pub release_blocking_certified: usize,
    /// Release-blocking surfaces narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Docs-pack surfaces.
    pub docs_pack_entries: usize,
    /// Migration-pack surfaces.
    pub migration_pack_entries: usize,
    /// Known-limits-pack surfaces.
    pub known_limits_pack_entries: usize,
    /// Publication-index surfaces.
    pub publication_index_entries: usize,
    /// First-party-trust surfaces.
    pub first_party_entries: usize,
    /// Verified-partner-trust surfaces.
    pub verified_partner_entries: usize,
    /// Community-trust surfaces.
    pub community_entries: usize,
    /// Generated surfaces.
    pub generated_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all surfaces.
    pub total_active_narrowing_reasons: usize,
    /// Total readiness cells across all surfaces.
    pub total_readiness_cells: usize,
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
pub struct PublicationPackExportRow {
    /// Stable surface id.
    pub entry_id: String,
    /// The surface kind this surface governs.
    pub surface_kind: SurfaceKind,
    /// The surface ref this entry speaks about.
    pub surface_ref: String,
    /// Whether the surface is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this surface backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the surface publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall surface state earned.
    pub surface_state: SurfaceState,
    /// Trust tier of the maintainer.
    pub trust_tier: TrustTier,
    /// Whether the redaction/provenance posture is disclosed to the operator.
    pub redaction_disclosed: bool,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Downgrade-automation state.
    pub automation_state: AutomationState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationPackExportProjection {
    /// Register identifier.
    pub manifest_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<PublicationPackExportRow>,
}

/// The typed publication-pack register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationPackRegister {
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
    /// Closed surface-kind vocabulary.
    pub surface_kinds: Vec<SurfaceKind>,
    /// Closed readiness-dimension vocabulary.
    pub readiness_dimensions: Vec<ReadinessDimension>,
    /// Closed dimension-grade vocabulary.
    pub dimension_grades: Vec<DimensionGrade>,
    /// Closed surface-state vocabulary.
    pub surface_states: Vec<SurfaceState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// Closed downgrade-automation-trigger vocabulary.
    pub automation_triggers: Vec<AutomationTrigger>,
    /// Closed downgrade-automation-state vocabulary.
    pub automation_states: Vec<AutomationState>,
    /// Closed trust-tier vocabulary.
    pub trust_tiers: Vec<TrustTier>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_surface_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<PublicationPackStopRule>,
    /// Publication-pack surfaces.
    pub rows: Vec<PublicationPackSurface>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: PublicationPackSummary,
}

impl PublicationPackRegister {
    /// Returns the surface registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&PublicationPackSurface> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the surfaces publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&PublicationPackSurface> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the surfaces narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&PublicationPackSurface> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking surfaces.
    pub fn release_blocking_rows(&self) -> Vec<&PublicationPackSurface> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the surfaces for one surface kind.
    pub fn rows_for_kind(&self, kind: SurfaceKind) -> Vec<&PublicationPackSurface> {
        self.rows
            .iter()
            .filter(|row| row.surface_kind == kind)
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

    /// True when `rule` fires: a watched surface carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &PublicationPackStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the surfaces and stop rules.
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

    /// Surface ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only surfaces whose claim is at or above the cutline count: a surface whose
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

    /// Recomputes the summary block from the surfaces and stop rules.
    pub fn computed_summary(&self) -> PublicationPackSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: SurfaceKind| self.rows_for_kind(kind).len();
        let trust = |tier: TrustTier| {
            self.rows
                .iter()
                .filter(|row| row.disclosure.trust_tier == tier)
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
        let release_blocking: Vec<&PublicationPackSurface> = self.release_blocking_rows();
        PublicationPackSummary {
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
                .filter(|row| row.surface_state == SurfaceState::OnWaiver)
                .count(),
            entries_with_dimension_gap: with_predicate(NarrowingReason::is_dimension_gap),
            entries_with_owner_gap: with_reason(NarrowingReason::OwnerManifestUnsigned),
            entries_with_automation_gap: with_predicate(NarrowingReason::is_automation_gap),
            entries_redaction_undisclosed: self
                .rows
                .iter()
                .filter(|row| !row.disclosure.redaction_disclosed)
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
            docs_pack_entries: kind(SurfaceKind::DocsPack),
            migration_pack_entries: kind(SurfaceKind::MigrationPack),
            known_limits_pack_entries: kind(SurfaceKind::KnownLimitsPack),
            publication_index_entries: kind(SurfaceKind::PublicationIndex),
            first_party_entries: trust(TrustTier::FirstParty),
            verified_partner_entries: trust(TrustTier::VerifiedPartner),
            community_entries: trust(TrustTier::Community),
            generated_entries: trust(TrustTier::Generated),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_narrowing_reasons: self
                .rows
                .iter()
                .map(|row| row.active_narrowing_reasons.len())
                .sum(),
            total_readiness_cells: self.rows.iter().map(|row| row.scorecard.len()).sum(),
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
    pub fn support_export_projection(&self) -> PublicationPackExportProjection {
        PublicationPackExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| PublicationPackExportRow {
                    entry_id: row.entry_id.clone(),
                    surface_kind: row.surface_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    surface_state: row.surface_state,
                    trust_tier: row.disclosure.trust_tier,
                    redaction_disclosed: row.disclosure.redaction_disclosed,
                    slo_state: row.proof_packet.slo_state,
                    automation_state: row.downgrade_automation.state,
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<PublicationPackViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(PublicationPackViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(PublicationPackViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(PublicationPackViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<PublicationPackViolation>) {
        if self.schema_version != PUBLICATION_PACK_SCHEMA_VERSION {
            violations.push(PublicationPackViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != PUBLICATION_PACK_RECORD_KIND {
            violations.push(PublicationPackViolation::UnsupportedRecordKind {
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
                violations.push(PublicationPackViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.surface_kinds != SurfaceKind::ALL.to_vec() {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "surface_kinds",
            });
        }
        if self.readiness_dimensions != ReadinessDimension::ALL.to_vec() {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "readiness_dimensions",
            });
        }
        if self.dimension_grades != DimensionGrade::ALL.to_vec() {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "dimension_grades",
            });
        }
        if self.surface_states != SurfaceState::ALL.to_vec() {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "surface_states",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.stop_rule_actions != StopAction::ALL.to_vec() {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }
        if self.automation_triggers != AutomationTrigger::ALL.to_vec() {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "automation_triggers",
            });
        }
        if self.automation_states != AutomationState::ALL.to_vec() {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "automation_states",
            });
        }
        if self.trust_tiers != TrustTier::ALL.to_vec() {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "trust_tiers",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(PublicationPackViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(PublicationPackViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<PublicationPackViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(PublicationPackViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(PublicationPackViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(PublicationPackViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(PublicationPackViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(PublicationPackViolation::NarrowingReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &PublicationPackSurface,
        violations: &mut Vec<PublicationPackViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("surface_ref", &row.surface_ref),
            ("surface_summary", &row.surface_summary),
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
            (
                "disclosure.support_window_ref",
                &row.disclosure.support_window_ref,
            ),
            ("disclosure.policy_ref", &row.disclosure.policy_ref),
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
                violations.push(PublicationPackViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_scorecard(row, violations);
        self.validate_downgrade_automation(row, violations);

        // The ceiling: no surface may carry a label wider than the claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(PublicationPackViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(PublicationPackViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(PublicationPackViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A claim whose canonical label is below the cutline forces the surface to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(PublicationPackViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(PublicationPackViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.surface_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed surface publishes exactly the claim's canonical label,
            // carries no active reason, rides a captured within-SLO packet,
            // discloses the redaction posture, is owner-signed, and rides
            // defined-and-verified downgrade automation.
            if row.published_label != row.claim_label {
                violations.push(PublicationPackViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(PublicationPackViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.disclosure.redaction_disclosed {
                violations.push(PublicationPackViolation::HeldWithoutRedactionDisclosure {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(PublicationPackViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(PublicationPackViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(PublicationPackViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.downgrade_automation.state.holds()
                || !row.downgrade_automation.rollback_verified
            {
                violations.push(PublicationPackViolation::HeldWithoutDowngradeAutomation {
                    entry_id: row.entry_id.clone(),
                    state: row.downgrade_automation.state,
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(PublicationPackViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.surface_state,
                    published: row.published_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(PublicationPackViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.surface_state,
                });
            }
            // A narrowing surface whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::ProofPacketStale)
            {
                violations.push(PublicationPackViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
            {
                violations.push(PublicationPackViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_scorecard(
        &self,
        row: &PublicationPackSurface,
        violations: &mut Vec<PublicationPackViolation>,
    ) {
        let mut seen: BTreeSet<ReadinessDimension> = BTreeSet::new();
        for cell in &row.scorecard {
            if !seen.insert(cell.dimension) {
                violations.push(PublicationPackViolation::DuplicateDimension {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A missing cell carries no evidence ref; every other grade must.
            if cell.grade != DimensionGrade::Missing && cell.evidence_ref.trim().is_empty() {
                violations.push(PublicationPackViolation::CellEvidenceMissing {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A waived cell only holds under an unexpired waiver.
            if cell.grade == DimensionGrade::Waived && row.waiver.is_none() {
                violations.push(PublicationPackViolation::WaivedCellWithoutWaiver {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A non-passing, non-waived cell must name its narrowing reason.
            if !cell.grade.holds() {
                if let Some(reason) = cell.dimension.reason_for_grade(cell.grade) {
                    if !row.has_active_reason(reason) {
                        violations.push(PublicationPackViolation::CellReasonNotActive {
                            entry_id: row.entry_id.clone(),
                            dimension: cell.dimension,
                            reason,
                        });
                    }
                }
            }
        }
        // The scorecard must carry exactly one cell per dimension.
        for dimension in ReadinessDimension::ALL {
            if !seen.contains(&dimension) {
                violations.push(PublicationPackViolation::ReadinessIncompleteCoverage {
                    entry_id: row.entry_id.clone(),
                    dimension,
                });
            }
        }
    }

    fn validate_downgrade_automation(
        &self,
        row: &PublicationPackSurface,
        violations: &mut Vec<PublicationPackViolation>,
    ) {
        let automation = &row.downgrade_automation;
        // A downgrade narrows the claim, so its floor must be below the cutline.
        if automation.target_floor.is_at_or_above_cutline() {
            violations.push(PublicationPackViolation::AutomationFloorNotBelowCutline {
                entry_id: row.entry_id.clone(),
                floor: automation.target_floor,
            });
        }
        // An undefined automation must name the undefined reason.
        if automation.state == AutomationState::Undefined
            && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
        {
            violations.push(PublicationPackViolation::AutomationStateWithoutReason {
                entry_id: row.entry_id.clone(),
                state: automation.state,
            });
        }
        // An unverified frozen-fallback rollback plan must name a rollback reason.
        if !automation.rollback_verified
            && !row.has_active_reason(NarrowingReason::RollbackPlanUnverified)
            && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
        {
            violations.push(PublicationPackViolation::RollbackUnverifiedWithoutReason {
                entry_id: row.entry_id.clone(),
            });
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &PublicationPackSurface,
        violations: &mut Vec<PublicationPackViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<PublicationPackViolation>,
                               expected: NarrowingReason| {
            violations.push(PublicationPackViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.surface_state,
                expected_reason: expected,
            });
        };

        match row.surface_state {
            SurfaceState::ReadinessRegressed => {
                if !row.has_active_reason(NarrowingReason::ReadinessDimensionFailed)
                    && !row.has_active_reason(NarrowingReason::ReadinessDimensionMissing)
                {
                    push_incoherent(violations, NarrowingReason::ReadinessDimensionFailed);
                }
            }
            SurfaceState::Stale => {
                if !row.has_active_reason(NarrowingReason::ProofPacketStale) {
                    push_incoherent(violations, NarrowingReason::ProofPacketStale);
                }
            }
            SurfaceState::AutomationUndefined => {
                if !row.has_active_reason(NarrowingReason::RollbackPlanUnverified)
                    && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
                {
                    push_incoherent(violations, NarrowingReason::DowngradeAutomationUndefined);
                }
            }
            SurfaceState::OwnerUnsigned => {
                if !row.has_active_reason(NarrowingReason::OwnerManifestUnsigned) {
                    push_incoherent(violations, NarrowingReason::OwnerManifestUnsigned);
                }
            }
            SurfaceState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(PublicationPackViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.surface_state,
                    });
                }
            }
            SurfaceState::Certified => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<PublicationPackViolation>) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.surface_ref.clone())
            .collect();
        for declared in &self.release_blocking_surface_refs {
            if !covered.contains(declared) {
                violations.push(PublicationPackViolation::ReleaseBlockingSurfaceUncovered {
                    surface_ref: declared.clone(),
                });
            }
        }
        for row in &self.rows {
            if row.release_blocking
                && !self
                    .release_blocking_surface_refs
                    .contains(&row.surface_ref)
            {
                violations.push(PublicationPackViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<PublicationPackViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(PublicationPackViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(PublicationPackViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(PublicationPackViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(PublicationPackViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(PublicationPackViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the publication-pack register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicationPackViolation {
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
    /// The register has no surfaces.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Surface or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A surface id appears more than once.
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
    /// A surface has two cells for one dimension.
    DuplicateDimension {
        /// Surface id.
        entry_id: String,
        /// Duplicated dimension.
        dimension: ReadinessDimension,
    },
    /// A surface is missing a dimension cell.
    ReadinessIncompleteCoverage {
        /// Surface id.
        entry_id: String,
        /// Uncovered dimension.
        dimension: ReadinessDimension,
    },
    /// A non-missing cell has no evidence ref.
    CellEvidenceMissing {
        /// Surface id.
        entry_id: String,
        /// Dimension.
        dimension: ReadinessDimension,
    },
    /// A waived cell is carried without a waiver.
    WaivedCellWithoutWaiver {
        /// Surface id.
        entry_id: String,
        /// Dimension.
        dimension: ReadinessDimension,
    },
    /// A non-passing cell does not name its narrowing reason.
    CellReasonNotActive {
        /// Surface id.
        entry_id: String,
        /// Dimension.
        dimension: ReadinessDimension,
        /// The reason the cell requires.
        reason: NarrowingReason,
    },
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Surface id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A surface holds a label while the claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Surface id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing state carries no active reason.
    NarrowingWithoutReason {
        /// Surface id.
        entry_id: String,
        /// Surface state.
        state: SurfaceState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Surface id.
        entry_id: String,
        /// Surface state.
        state: SurfaceState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held surface carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Surface id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held surface has active narrowing reasons.
    HeldWithActiveGap {
        /// Surface id.
        entry_id: String,
    },
    /// A held surface does not disclose its redaction/provenance posture.
    HeldWithoutRedactionDisclosure {
        /// Surface id.
        entry_id: String,
    },
    /// A held surface has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Surface id.
        entry_id: String,
    },
    /// A held surface rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Surface id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held surface lacks owner-manifest sign-off.
    HeldWithoutSignoff {
        /// Surface id.
        entry_id: String,
    },
    /// A held surface lacks defined-and-verified downgrade automation.
    HeldWithoutDowngradeAutomation {
        /// Surface id.
        entry_id: String,
        /// Downgrade-automation state.
        state: AutomationState,
    },
    /// The downgrade floor is not below the cutline.
    AutomationFloorNotBelowCutline {
        /// Surface id.
        entry_id: String,
        /// Declared floor.
        floor: StableClaimLevel,
    },
    /// An undefined automation does not name the undefined reason.
    AutomationStateWithoutReason {
        /// Surface id.
        entry_id: String,
        /// Downgrade-automation state.
        state: AutomationState,
    },
    /// An unverified frozen-fallback rollback plan does not name a rollback reason.
    RollbackUnverifiedWithoutReason {
        /// Surface id.
        entry_id: String,
    },
    /// A narrowing surface with a breached proof packet does not name the stale
    /// reason.
    BreachedPacketWithoutReason {
        /// Surface id.
        entry_id: String,
    },
    /// A narrowing surface with a missing proof packet does not name the missing
    /// reason.
    MissingPacketWithoutReason {
        /// Surface id.
        entry_id: String,
    },
    /// A surface state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Surface id.
        entry_id: String,
        /// Surface state.
        state: SurfaceState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Surface id.
        entry_id: String,
        /// Surface state.
        state: SurfaceState,
    },
    /// A release-blocking surface ref has no covering surface.
    ReleaseBlockingSurfaceUncovered {
        /// Surface ref.
        surface_ref: String,
    },
    /// A release-blocking surface is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Surface id.
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
    /// The summary counts disagree with the surfaces.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Surface id.
        entry_id: String,
    },
}

impl fmt::Display for PublicationPackViolation {
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
            Self::EmptyRegister => write!(f, "register has no surfaces"),
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
                "surface {entry_id} has duplicate dimension {}",
                dimension.as_str()
            ),
            Self::ReadinessIncompleteCoverage {
                entry_id,
                dimension,
            } => write!(
                f,
                "surface {entry_id} is missing dimension {}",
                dimension.as_str()
            ),
            Self::CellEvidenceMissing {
                entry_id,
                dimension,
            } => write!(
                f,
                "surface {entry_id} dimension {} has no evidence ref",
                dimension.as_str()
            ),
            Self::WaivedCellWithoutWaiver {
                entry_id,
                dimension,
            } => write!(
                f,
                "surface {entry_id} dimension {} is waived without a waiver",
                dimension.as_str()
            ),
            Self::CellReasonNotActive {
                entry_id,
                dimension,
                reason,
            } => write!(
                f,
                "surface {entry_id} dimension {} requires active reason {}",
                dimension.as_str(),
                reason.as_str()
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "surface {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "surface {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "surface {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "surface {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "surface {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "surface {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutRedactionDisclosure { entry_id } => {
                write!(
                    f,
                    "surface {entry_id} holds stable without disclosing its redaction posture"
                )
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "surface {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "surface {entry_id} holds stable on stale packet {slo_state:?}"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "surface {entry_id} holds stable without owner signoff")
            }
            Self::HeldWithoutDowngradeAutomation { entry_id, state } => write!(
                f,
                "surface {entry_id} holds stable without defined+verified downgrade automation ({state:?})"
            ),
            Self::AutomationFloorNotBelowCutline { entry_id, floor } => write!(
                f,
                "surface {entry_id} downgrade floor {floor:?} is not below the cutline"
            ),
            Self::AutomationStateWithoutReason { entry_id, state } => write!(
                f,
                "surface {entry_id} downgrade-automation state {state:?} names no narrowing reason"
            ),
            Self::RollbackUnverifiedWithoutReason { entry_id } => write!(
                f,
                "surface {entry_id} has an unverified frozen-fallback rollback plan without a reason"
            ),
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "surface {entry_id} breached packet without proof_packet_stale reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "surface {entry_id} missing packet without proof_packet_missing reason"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "surface {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "surface {entry_id} state {state:?} names no waiver")
            }
            Self::ReleaseBlockingSurfaceUncovered { surface_ref } => {
                write!(
                    f,
                    "release-blocking surface {surface_ref} has no covering surface"
                )
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking surface {entry_id} is not declared in release_blocking_surface_refs"
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => {
                write!(f, "promotion {declared:?} disagrees with computed {computed:?}")
            }
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with surfaces"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "surface {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for PublicationPackViolation {}

/// Loads the embedded publication-pack register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`PublicationPackRegister`].
pub fn current_publication_pack_register() -> Result<PublicationPackRegister, serde_json::Error> {
    serde_json::from_str(PUBLICATION_PACK_JSON)
}

#[cfg(test)]
mod tests;
