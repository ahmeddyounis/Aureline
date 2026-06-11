//! Typed feature-train compatibility register, provider-family support windows, and change-freeze guidance.
//!
//! This module freezes the canonical control surface for the compatibility
//! reports, provider-family support windows, and change-freeze guidance of the
//! release-line feature trains. Where the depth-claim manifest speaks for the
//! *depth claim* each feature family publishes and the locale-pack register speaks
//! for the *locale-pack lane* every pack channel exposes, this register speaks for
//! the *feature-train lane* every train channel exposes — the core platform train,
//! the AI assistant train, the collaboration train, and the extensions train. Each
//! [`FeatureTrainLane`] binds one feature train to:
//!
//! - the stable claim it backs ([`FeatureTrainLane::claim_ref`],
//!   [`FeatureTrainLane::claim_label`]),
//! - a compatibility-report scorecard ([`FeatureTrainLane::scorecard`]) of one
//!   [`CompatibilityCell`] per [`CompatibilityDimension`], so forward
//!   compatibility, backward compatibility, schema versioning, the provider
//!   support window, deprecation policy, and change-freeze adherence are each an
//!   explicit, inspectable grade,
//! - the provider-family support window it discloses
//!   ([`FeatureTrainLane::support_window`]): the provider family, the compatibility
//!   baseline, the [`TrustTier`], the supported version sets, and whether the
//!   end-of-support boundary is disclosed to the operator,
//! - an owner manifest ([`FeatureTrainLane::owner_signoff`]) recording who signed
//!   the claim,
//! - an explicit change-freeze guidance record
//!   ([`FeatureTrainLane::change_freeze`]) binding the lane to a verified
//!   frozen-fallback plan and the trigger and floor it narrows to,
//! - the overall train state earned ([`TrainState`]), the active narrowing reasons
//!   ([`NarrowingReason`]), and the effective label after narrowing
//!   ([`FeatureTrainLane::published_label`]),
//! - a [`ProofPacket`] (reused from the stable claim manifest) and its freshness
//!   SLO, plus an optional waiver.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a lane that may publish a Stable claim and one that must narrow below
//! it. The [`FeatureTrainStopRule`] set names the closed conditions that gate
//! promotion — one per [`NarrowingReason`] — and
//! [`FeatureTrainCompatibilityRegister::promotion`] records the proceed/hold
//! verdict.
//!
//! The register is checked in at
//! `artifacts/release/m5/implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance.json`
//! and embedded here, so this typed consumer and the CI gate agree on every
//! feature-train lane without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no provider payloads, raw compatibility logs, signatures, or credential
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
pub const FEATURE_TRAIN_COMPATIBILITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const FEATURE_TRAIN_COMPATIBILITY_RECORD_KIND: &str =
    "implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance";

/// Repo-relative path to the checked-in register.
pub const FEATURE_TRAIN_COMPATIBILITY_PATH: &str =
    "artifacts/release/m5/implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance.json";

/// Embedded checked-in register JSON.
pub const FEATURE_TRAIN_COMPATIBILITY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance.json"
));

/// Train channel a feature-train lane governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainChannel {
    /// Core platform feature train.
    CorePlatform,
    /// AI assistant feature train.
    AiAssistant,
    /// Collaboration/sync feature train.
    Collaboration,
    /// Extensions/integration feature train.
    Extensions,
}

impl TrainChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CorePlatform,
        Self::AiAssistant,
        Self::Collaboration,
        Self::Extensions,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorePlatform => "core_platform",
            Self::AiAssistant => "ai_assistant",
            Self::Collaboration => "collaboration",
            Self::Extensions => "extensions",
        }
    }
}

/// One dimension of the compatibility-report scorecard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityDimension {
    /// A new client interoperates with an older server (forward compatibility).
    ForwardCompatibility,
    /// An older client interoperates with a new server (backward compatibility).
    BackwardCompatibility,
    /// The train honors the published schema-version windows.
    SchemaVersioning,
    /// The provider-family support window is honored and within bounds.
    ProviderSupportWindow,
    /// A deprecation policy with notice and window is published.
    DeprecationPolicy,
    /// The change-freeze guidance is adhered to.
    ChangeFreezeAdherence,
}

impl CompatibilityDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ForwardCompatibility,
        Self::BackwardCompatibility,
        Self::SchemaVersioning,
        Self::ProviderSupportWindow,
        Self::DeprecationPolicy,
        Self::ChangeFreezeAdherence,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ForwardCompatibility => "forward_compatibility",
            Self::BackwardCompatibility => "backward_compatibility",
            Self::SchemaVersioning => "schema_versioning",
            Self::ProviderSupportWindow => "provider_support_window",
            Self::DeprecationPolicy => "deprecation_policy",
            Self::ChangeFreezeAdherence => "change_freeze_adherence",
        }
    }

    /// The narrowing reason a non-passing, non-waived cell must name, given the
    /// cell's [`DimensionGrade`].
    pub const fn reason_for_grade(self, grade: DimensionGrade) -> Option<NarrowingReason> {
        match grade {
            DimensionGrade::Missing => Some(NarrowingReason::CompatibilityDimensionMissing),
            DimensionGrade::Fail | DimensionGrade::Partial => {
                Some(NarrowingReason::CompatibilityDimensionFailed)
            }
            DimensionGrade::Pass | DimensionGrade::Waived => None,
        }
    }
}

/// The grade earned on one compatibility dimension.
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
    /// The dimension has no compatibility evidence at all.
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

/// Trust tier of the provider family/maintainer behind a feature train.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustTier {
    /// First-party Aureline-maintained train.
    FirstParty,
    /// A verified partner/vendor provider family.
    VerifiedPartner,
    /// A community-sourced provider family.
    Community,
    /// An unverified or untrusted provider family.
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

/// Overall compatibility/lifecycle state a feature-train lane earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainState {
    /// Every dimension passes, the support window is disclosed, the owner manifest
    /// is signed, and change-freeze guidance is defined and verified.
    Certified,
    /// One or more compatibility dimensions failed, are partial, or are missing —
    /// including a failed forward/backward-compatibility report.
    CompatibilityBroken,
    /// The proof packet has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers a
    /// recorded gap.
    OnWaiver,
    /// Change-freeze guidance is undefined or its frozen-fallback plan is
    /// unverified.
    FreezeUndefined,
    /// The owner manifest is unsigned.
    OwnerUnsigned,
}

impl TrainState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Certified,
        Self::CompatibilityBroken,
        Self::Stale,
        Self::OnWaiver,
        Self::FreezeUndefined,
        Self::OwnerUnsigned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::CompatibilityBroken => "compatibility_broken",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::FreezeUndefined => "freeze_undefined",
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

/// Closed reason a feature-train lane claim narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// A compatibility dimension failed or is only partial.
    CompatibilityDimensionFailed,
    /// A compatibility dimension is missing.
    CompatibilityDimensionMissing,
    /// The proof packet is missing.
    ProofPacketMissing,
    /// The proof packet is stale.
    ProofPacketStale,
    /// The owner manifest is unsigned.
    OwnerManifestUnsigned,
    /// The frozen-fallback change-freeze plan is unverified.
    FreezePlanUnverified,
    /// The change-freeze guidance is undefined.
    ChangeFreezeUndefined,
    /// A waiver the lane relied on has expired.
    WaiverExpired,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CompatibilityDimensionFailed,
        Self::CompatibilityDimensionMissing,
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::OwnerManifestUnsigned,
        Self::FreezePlanUnverified,
        Self::ChangeFreezeUndefined,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompatibilityDimensionFailed => "compatibility_dimension_failed",
            Self::CompatibilityDimensionMissing => "compatibility_dimension_missing",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::OwnerManifestUnsigned => "owner_manifest_unsigned",
            Self::FreezePlanUnverified => "freeze_plan_unverified",
            Self::ChangeFreezeUndefined => "change_freeze_undefined",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether this reason marks a change-freeze guidance gap.
    pub const fn is_freeze_gap(self) -> bool {
        matches!(
            self,
            Self::FreezePlanUnverified | Self::ChangeFreezeUndefined
        )
    }

    /// Whether this reason marks a compatibility-dimension gap.
    pub const fn is_dimension_gap(self) -> bool {
        matches!(
            self,
            Self::CompatibilityDimensionFailed | Self::CompatibilityDimensionMissing
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
    /// Remediate the failing or missing compatibility dimension.
    RemediateCompatibility,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Obtain the required owner-manifest sign-off.
    RequestOwnerSignoff,
    /// Verify the frozen-fallback change-freeze plan.
    VerifyFreezePlan,
    /// Define the change-freeze guidance.
    DefineChangeFreeze,
    /// Renew the expired waiver.
    RenewWaiver,
}

impl StopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::HoldPromotion,
        Self::NarrowLabel,
        Self::RemediateCompatibility,
        Self::RefreshProofPacket,
        Self::RequestOwnerSignoff,
        Self::VerifyFreezePlan,
        Self::DefineChangeFreeze,
        Self::RenewWaiver,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowLabel => "narrow_label",
            Self::RemediateCompatibility => "remediate_compatibility",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::VerifyFreezePlan => "verify_freeze_plan",
            Self::DefineChangeFreeze => "define_change_freeze",
            Self::RenewWaiver => "renew_waiver",
        }
    }
}

/// What triggers a lane's automated change-freeze/downgrade to a frozen label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreezeTrigger {
    /// Fires when the proof packet goes stale.
    ProofStale,
    /// Fires when the compatibility report regresses.
    CompatibilityBroken,
    /// Fires when owner sign-off is revoked.
    OwnerRevoked,
    /// Operator-driven manual change freeze.
    Manual,
}

impl FreezeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProofStale,
        Self::CompatibilityBroken,
        Self::OwnerRevoked,
        Self::Manual,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::CompatibilityBroken => "compatibility_broken",
            Self::OwnerRevoked => "owner_revoked",
            Self::Manual => "manual",
        }
    }
}

/// The defined/verified state of a lane's change-freeze guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreezeState {
    /// The guidance is defined and its frozen-fallback plan is verified.
    Defined,
    /// The guidance is defined but its frozen-fallback plan is unverified.
    Unverified,
    /// The guidance is undefined.
    Undefined,
}

impl FreezeState {
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

    /// Whether the guidance is defined and verified, letting a lane hold a Stable
    /// claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Defined)
    }
}

/// One cell of the compatibility-report scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityCell {
    /// The compatibility dimension this cell speaks for.
    pub dimension: CompatibilityDimension,
    /// The grade earned for the dimension.
    pub grade: DimensionGrade,
    /// Ref to the dimension's evidence. Empty only on a missing cell.
    pub evidence_ref: String,
}

/// The disclosed provider-family support window of a feature train.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderSupportWindow {
    /// Stable ref to the provider family the train integrates.
    pub provider_family_ref: String,
    /// Stable ref to the compatibility baseline the train measures against.
    pub baseline_ref: String,
    /// Trust tier of the provider family/maintainer.
    pub trust_tier: TrustTier,
    /// Refs to the supported version sets the window covers.
    #[serde(default)]
    pub supported_version_refs: Vec<String>,
    /// Whether the end-of-support boundary of the support window is disclosed to
    /// the operator.
    pub eol_disclosed: bool,
}

/// A lane's change-freeze guidance, falling back to a frozen label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChangeFreezeGuidance {
    /// Stable ref to the change-freeze guidance definition.
    pub guidance_ref: String,
    /// Ref to the frozen-fallback plan the guidance drives.
    pub freeze_plan_ref: String,
    /// What triggers the automated change freeze.
    pub trigger: FreezeTrigger,
    /// The lifecycle label the change freeze narrows the lane to.
    pub target_floor: StableClaimLevel,
    /// The defined/verified state of the guidance.
    pub state: FreezeState,
    /// Whether the frozen-fallback plan has been verified end-to-end.
    pub freeze_verified: bool,
}

/// One feature-train compatibility stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureTrainStopRule {
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

/// One feature-train compatibility lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureTrainLane {
    /// Stable lane id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The train channel this lane governs.
    pub train_channel: TrainChannel,
    /// The train ref this entry speaks about.
    pub train_ref: String,
    /// Reviewable one-line statement of the train.
    pub train_summary: String,
    /// Whether the train is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this lane backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes.
    pub claim_label: StableClaimLevel,
    /// Overall train state earned for the lane.
    pub train_state: TrainState,
    /// The compatibility scorecard: one cell per [`CompatibilityDimension`].
    pub scorecard: Vec<CompatibilityCell>,
    /// The disclosed provider-family support window of the train.
    pub support_window: ProviderSupportWindow,
    /// The change-freeze guidance backing the lane.
    pub change_freeze: ChangeFreezeGuidance,
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

impl FeatureTrainLane {
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
        self.train_state.holds_label()
    }

    /// True when a narrowing reason is active on the lane.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// Returns the cell registered for `dimension`, if any.
    pub fn cell(&self, dimension: CompatibilityDimension) -> Option<&CompatibilityCell> {
        self.scorecard
            .iter()
            .find(|cell| cell.dimension == dimension)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureTrainCompatibilitySummary {
    /// Total number of feature-train lanes.
    pub total_entries: usize,
    /// Distinct claims covered.
    pub total_claims: usize,
    /// Lanes publishing a label at or above the cutline.
    pub entries_certified: usize,
    /// Lanes narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Lanes holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Lanes carrying a compatibility-dimension gap (failed or missing dimension).
    pub entries_with_dimension_gap: usize,
    /// Lanes carrying an owner-manifest-unsigned reason.
    pub entries_with_owner_gap: usize,
    /// Lanes carrying a change-freeze guidance gap.
    pub entries_with_freeze_gap: usize,
    /// Lanes whose provider-family end-of-support window is not disclosed.
    pub entries_eol_undisclosed: usize,
    /// Total release-blocking lanes.
    pub release_blocking_total: usize,
    /// Release-blocking lanes publishing a label at or above the cutline.
    pub release_blocking_certified: usize,
    /// Release-blocking lanes narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Core-platform lanes.
    pub core_platform_entries: usize,
    /// AI-assistant lanes.
    pub ai_assistant_entries: usize,
    /// Collaboration lanes.
    pub collaboration_entries: usize,
    /// Extensions lanes.
    pub extensions_entries: usize,
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
    /// Total compatibility cells across all lanes.
    pub total_compatibility_cells: usize,
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
pub struct FeatureTrainExportRow {
    /// Stable lane id.
    pub entry_id: String,
    /// The train channel this lane governs.
    pub train_channel: TrainChannel,
    /// The train ref this entry speaks about.
    pub train_ref: String,
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
    /// Overall train state earned.
    pub train_state: TrainState,
    /// Trust tier of the provider family/maintainer.
    pub trust_tier: TrustTier,
    /// Whether the end-of-support window is disclosed to the operator.
    pub eol_disclosed: bool,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Change-freeze guidance state.
    pub freeze_state: FreezeState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureTrainExportProjection {
    /// Register identifier.
    pub manifest_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<FeatureTrainExportRow>,
}

/// The typed feature-train compatibility register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureTrainCompatibilityRegister {
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
    /// Closed train-channel vocabulary.
    pub train_channels: Vec<TrainChannel>,
    /// Closed compatibility-dimension vocabulary.
    pub compatibility_dimensions: Vec<CompatibilityDimension>,
    /// Closed dimension-grade vocabulary.
    pub dimension_grades: Vec<DimensionGrade>,
    /// Closed train-state vocabulary.
    pub train_states: Vec<TrainState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// Closed change-freeze-trigger vocabulary.
    pub freeze_triggers: Vec<FreezeTrigger>,
    /// Closed change-freeze-state vocabulary.
    pub freeze_states: Vec<FreezeState>,
    /// Closed trust-tier vocabulary.
    pub trust_tiers: Vec<TrustTier>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking train refs this register must cover.
    pub release_blocking_train_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<FeatureTrainStopRule>,
    /// Feature-train lanes.
    pub rows: Vec<FeatureTrainLane>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: FeatureTrainCompatibilitySummary,
}

impl FeatureTrainCompatibilityRegister {
    /// Returns the lane registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&FeatureTrainLane> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the lanes publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&FeatureTrainLane> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the lanes narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&FeatureTrainLane> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking lanes.
    pub fn release_blocking_rows(&self) -> Vec<&FeatureTrainLane> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the lanes for one train channel.
    pub fn rows_for_channel(&self, channel: TrainChannel) -> Vec<&FeatureTrainLane> {
        self.rows
            .iter()
            .filter(|row| row.train_channel == channel)
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
    pub fn stop_rule_fires(&self, rule: &FeatureTrainStopRule) -> bool {
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
    pub fn computed_summary(&self) -> FeatureTrainCompatibilitySummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let channel = |channel: TrainChannel| self.rows_for_channel(channel).len();
        let trust = |tier: TrustTier| {
            self.rows
                .iter()
                .filter(|row| row.support_window.trust_tier == tier)
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
        let release_blocking: Vec<&FeatureTrainLane> = self.release_blocking_rows();
        FeatureTrainCompatibilitySummary {
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
                .filter(|row| row.train_state == TrainState::OnWaiver)
                .count(),
            entries_with_dimension_gap: with_predicate(NarrowingReason::is_dimension_gap),
            entries_with_owner_gap: with_reason(NarrowingReason::OwnerManifestUnsigned),
            entries_with_freeze_gap: with_predicate(NarrowingReason::is_freeze_gap),
            entries_eol_undisclosed: self
                .rows
                .iter()
                .filter(|row| !row.support_window.eol_disclosed)
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
            core_platform_entries: channel(TrainChannel::CorePlatform),
            ai_assistant_entries: channel(TrainChannel::AiAssistant),
            collaboration_entries: channel(TrainChannel::Collaboration),
            extensions_entries: channel(TrainChannel::Extensions),
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
            total_compatibility_cells: self.rows.iter().map(|row| row.scorecard.len()).sum(),
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
    pub fn support_export_projection(&self) -> FeatureTrainExportProjection {
        FeatureTrainExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| FeatureTrainExportRow {
                    entry_id: row.entry_id.clone(),
                    train_channel: row.train_channel,
                    train_ref: row.train_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    train_state: row.train_state,
                    trust_tier: row.support_window.trust_tier,
                    eol_disclosed: row.support_window.eol_disclosed,
                    slo_state: row.proof_packet.slo_state,
                    freeze_state: row.change_freeze.state,
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<FeatureTrainRegisterViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(FeatureTrainRegisterViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(FeatureTrainRegisterViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(FeatureTrainRegisterViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<FeatureTrainRegisterViolation>) {
        if self.schema_version != FEATURE_TRAIN_COMPATIBILITY_SCHEMA_VERSION {
            violations.push(FeatureTrainRegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != FEATURE_TRAIN_COMPATIBILITY_RECORD_KIND {
            violations.push(FeatureTrainRegisterViolation::UnsupportedRecordKind {
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
                violations.push(FeatureTrainRegisterViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.train_channels != TrainChannel::ALL.to_vec() {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "train_channels",
            });
        }
        if self.compatibility_dimensions != CompatibilityDimension::ALL.to_vec() {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "compatibility_dimensions",
            });
        }
        if self.dimension_grades != DimensionGrade::ALL.to_vec() {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "dimension_grades",
            });
        }
        if self.train_states != TrainState::ALL.to_vec() {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "train_states",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.stop_rule_actions != StopAction::ALL.to_vec() {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }
        if self.freeze_triggers != FreezeTrigger::ALL.to_vec() {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "freeze_triggers",
            });
        }
        if self.freeze_states != FreezeState::ALL.to_vec() {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "freeze_states",
            });
        }
        if self.trust_tiers != TrustTier::ALL.to_vec() {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "trust_tiers",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(FeatureTrainRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(FeatureTrainRegisterViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<FeatureTrainRegisterViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(FeatureTrainRegisterViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(FeatureTrainRegisterViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(FeatureTrainRegisterViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(FeatureTrainRegisterViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(FeatureTrainRegisterViolation::NarrowingReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &FeatureTrainLane,
        violations: &mut Vec<FeatureTrainRegisterViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("train_ref", &row.train_ref),
            ("train_summary", &row.train_summary),
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
                "support_window.provider_family_ref",
                &row.support_window.provider_family_ref,
            ),
            (
                "support_window.baseline_ref",
                &row.support_window.baseline_ref,
            ),
            (
                "change_freeze.guidance_ref",
                &row.change_freeze.guidance_ref,
            ),
            (
                "change_freeze.freeze_plan_ref",
                &row.change_freeze.freeze_plan_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(FeatureTrainRegisterViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_scorecard(row, violations);
        self.validate_change_freeze(row, violations);

        // The ceiling: no lane may carry a label wider than the claim's canonical
        // label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(FeatureTrainRegisterViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(FeatureTrainRegisterViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(FeatureTrainRegisterViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A claim whose canonical label is below the cutline forces the lane to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(FeatureTrainRegisterViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(FeatureTrainRegisterViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.train_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed lane publishes exactly the claim's canonical label, carries
            // no active reason, rides a captured within-SLO packet, discloses the
            // support window, is owner-signed, and rides defined-and-verified
            // change-freeze guidance.
            if row.published_label != row.claim_label {
                violations.push(FeatureTrainRegisterViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(FeatureTrainRegisterViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.support_window.eol_disclosed {
                violations.push(FeatureTrainRegisterViolation::HeldWithoutEolDisclosure {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(FeatureTrainRegisterViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(FeatureTrainRegisterViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(FeatureTrainRegisterViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.change_freeze.state.holds() || !row.change_freeze.freeze_verified {
                violations.push(FeatureTrainRegisterViolation::HeldWithoutChangeFreeze {
                    entry_id: row.entry_id.clone(),
                    state: row.change_freeze.state,
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(FeatureTrainRegisterViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.train_state,
                    published: row.published_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(FeatureTrainRegisterViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.train_state,
                });
            }
            // A narrowing lane whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::ProofPacketStale)
            {
                violations.push(FeatureTrainRegisterViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
            {
                violations.push(FeatureTrainRegisterViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_scorecard(
        &self,
        row: &FeatureTrainLane,
        violations: &mut Vec<FeatureTrainRegisterViolation>,
    ) {
        let mut seen: BTreeSet<CompatibilityDimension> = BTreeSet::new();
        for cell in &row.scorecard {
            if !seen.insert(cell.dimension) {
                violations.push(FeatureTrainRegisterViolation::DuplicateDimension {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A missing cell carries no evidence ref; every other grade must.
            if cell.grade != DimensionGrade::Missing && cell.evidence_ref.trim().is_empty() {
                violations.push(FeatureTrainRegisterViolation::CellEvidenceMissing {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A waived cell only holds under an unexpired waiver.
            if cell.grade == DimensionGrade::Waived && row.waiver.is_none() {
                violations.push(FeatureTrainRegisterViolation::WaivedCellWithoutWaiver {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A non-passing, non-waived cell must name its narrowing reason.
            if !cell.grade.holds() {
                if let Some(reason) = cell.dimension.reason_for_grade(cell.grade) {
                    if !row.has_active_reason(reason) {
                        violations.push(FeatureTrainRegisterViolation::CellReasonNotActive {
                            entry_id: row.entry_id.clone(),
                            dimension: cell.dimension,
                            reason,
                        });
                    }
                }
            }
        }
        // The scorecard must carry exactly one cell per dimension.
        for dimension in CompatibilityDimension::ALL {
            if !seen.contains(&dimension) {
                violations.push(
                    FeatureTrainRegisterViolation::CompatibilityIncompleteCoverage {
                        entry_id: row.entry_id.clone(),
                        dimension,
                    },
                );
            }
        }
    }

    fn validate_change_freeze(
        &self,
        row: &FeatureTrainLane,
        violations: &mut Vec<FeatureTrainRegisterViolation>,
    ) {
        let freeze = &row.change_freeze;
        // A change freeze narrows the claim, so its floor must be below the cutline.
        if freeze.target_floor.is_at_or_above_cutline() {
            violations.push(FeatureTrainRegisterViolation::FreezeFloorNotBelowCutline {
                entry_id: row.entry_id.clone(),
                floor: freeze.target_floor,
            });
        }
        // An undefined guidance must name the undefined reason.
        if freeze.state == FreezeState::Undefined
            && !row.has_active_reason(NarrowingReason::ChangeFreezeUndefined)
        {
            violations.push(FeatureTrainRegisterViolation::FreezeStateWithoutReason {
                entry_id: row.entry_id.clone(),
                state: freeze.state,
            });
        }
        // An unverified frozen-fallback plan must name a freeze reason.
        if !freeze.freeze_verified
            && !row.has_active_reason(NarrowingReason::FreezePlanUnverified)
            && !row.has_active_reason(NarrowingReason::ChangeFreezeUndefined)
        {
            violations.push(
                FeatureTrainRegisterViolation::FreezeUnverifiedWithoutReason {
                    entry_id: row.entry_id.clone(),
                },
            );
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &FeatureTrainLane,
        violations: &mut Vec<FeatureTrainRegisterViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<FeatureTrainRegisterViolation>,
                               expected: NarrowingReason| {
            violations.push(FeatureTrainRegisterViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.train_state,
                expected_reason: expected,
            });
        };

        match row.train_state {
            TrainState::CompatibilityBroken => {
                if !row.has_active_reason(NarrowingReason::CompatibilityDimensionFailed)
                    && !row.has_active_reason(NarrowingReason::CompatibilityDimensionMissing)
                {
                    push_incoherent(violations, NarrowingReason::CompatibilityDimensionFailed);
                }
            }
            TrainState::Stale => {
                if !row.has_active_reason(NarrowingReason::ProofPacketStale) {
                    push_incoherent(violations, NarrowingReason::ProofPacketStale);
                }
            }
            TrainState::FreezeUndefined => {
                if !row.has_active_reason(NarrowingReason::FreezePlanUnverified)
                    && !row.has_active_reason(NarrowingReason::ChangeFreezeUndefined)
                {
                    push_incoherent(violations, NarrowingReason::ChangeFreezeUndefined);
                }
            }
            TrainState::OwnerUnsigned => {
                if !row.has_active_reason(NarrowingReason::OwnerManifestUnsigned) {
                    push_incoherent(violations, NarrowingReason::OwnerManifestUnsigned);
                }
            }
            TrainState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(FeatureTrainRegisterViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.train_state,
                    });
                }
            }
            TrainState::Certified => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<FeatureTrainRegisterViolation>) {
        let covered: BTreeSet<String> = self.rows.iter().map(|row| row.train_ref.clone()).collect();
        for declared in &self.release_blocking_train_refs {
            if !covered.contains(declared) {
                violations.push(
                    FeatureTrainRegisterViolation::ReleaseBlockingTrainUncovered {
                        train_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_train_refs.contains(&row.train_ref) {
                violations.push(
                    FeatureTrainRegisterViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<FeatureTrainRegisterViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(FeatureTrainRegisterViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(FeatureTrainRegisterViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(
                FeatureTrainRegisterViolation::PromotionDecisionInconsistent {
                    declared: self.promotion.decision,
                    computed,
                },
            );
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                FeatureTrainRegisterViolation::PromotionBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(
                FeatureTrainRegisterViolation::PromotionBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// A validation violation for the feature-train compatibility register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureTrainRegisterViolation {
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
        dimension: CompatibilityDimension,
    },
    /// A lane is missing a dimension cell.
    CompatibilityIncompleteCoverage {
        /// Lane id.
        entry_id: String,
        /// Uncovered dimension.
        dimension: CompatibilityDimension,
    },
    /// A non-missing cell has no evidence ref.
    CellEvidenceMissing {
        /// Lane id.
        entry_id: String,
        /// Dimension.
        dimension: CompatibilityDimension,
    },
    /// A waived cell is carried without a waiver.
    WaivedCellWithoutWaiver {
        /// Lane id.
        entry_id: String,
        /// Dimension.
        dimension: CompatibilityDimension,
    },
    /// A non-passing cell does not name its narrowing reason.
    CellReasonNotActive {
        /// Lane id.
        entry_id: String,
        /// Dimension.
        dimension: CompatibilityDimension,
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
        /// Train state.
        state: TrainState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Lane id.
        entry_id: String,
        /// Train state.
        state: TrainState,
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
    /// A held lane does not disclose its provider-family end-of-support window.
    HeldWithoutEolDisclosure {
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
    /// A held lane lacks defined-and-verified change-freeze guidance.
    HeldWithoutChangeFreeze {
        /// Lane id.
        entry_id: String,
        /// Change-freeze state.
        state: FreezeState,
    },
    /// The change-freeze floor is not below the cutline.
    FreezeFloorNotBelowCutline {
        /// Lane id.
        entry_id: String,
        /// Declared floor.
        floor: StableClaimLevel,
    },
    /// An undefined guidance does not name the undefined reason.
    FreezeStateWithoutReason {
        /// Lane id.
        entry_id: String,
        /// Change-freeze state.
        state: FreezeState,
    },
    /// An unverified frozen-fallback plan does not name a freeze reason.
    FreezeUnverifiedWithoutReason {
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
    /// A train state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Lane id.
        entry_id: String,
        /// Train state.
        state: TrainState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Lane id.
        entry_id: String,
        /// Train state.
        state: TrainState,
    },
    /// A release-blocking train ref has no covering lane.
    ReleaseBlockingTrainUncovered {
        /// Train ref.
        train_ref: String,
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

impl fmt::Display for FeatureTrainRegisterViolation {
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
            Self::CompatibilityIncompleteCoverage {
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
            Self::HeldWithoutEolDisclosure { entry_id } => {
                write!(
                    f,
                    "lane {entry_id} holds stable without disclosing its end-of-support window"
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
            Self::HeldWithoutChangeFreeze { entry_id, state } => write!(
                f,
                "lane {entry_id} holds stable without defined+verified change-freeze guidance ({state:?})"
            ),
            Self::FreezeFloorNotBelowCutline { entry_id, floor } => write!(
                f,
                "lane {entry_id} change-freeze floor {floor:?} is not below the cutline"
            ),
            Self::FreezeStateWithoutReason { entry_id, state } => write!(
                f,
                "lane {entry_id} change-freeze state {state:?} names no narrowing reason"
            ),
            Self::FreezeUnverifiedWithoutReason { entry_id } => write!(
                f,
                "lane {entry_id} has an unverified frozen-fallback plan without a reason"
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
            Self::ReleaseBlockingTrainUncovered { train_ref } => {
                write!(f, "release-blocking train {train_ref} has no covering lane")
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking lane {entry_id} is not declared in release_blocking_train_refs"
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

impl Error for FeatureTrainRegisterViolation {}

/// Loads the embedded feature-train compatibility register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`FeatureTrainCompatibilityRegister`].
pub fn current_feature_train_compatibility_register(
) -> Result<FeatureTrainCompatibilityRegister, serde_json::Error> {
    serde_json::from_str(FEATURE_TRAIN_COMPATIBILITY_JSON)
}

#[cfg(test)]
mod tests;
