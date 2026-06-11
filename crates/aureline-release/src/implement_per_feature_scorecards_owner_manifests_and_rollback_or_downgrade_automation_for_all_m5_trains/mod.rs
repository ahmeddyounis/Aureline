//! Typed M5 per-train scorecard register, owner manifests, and rollback/downgrade automation.
//!
//! This module freezes the canonical M5 train-qualification control surface.
//! Where the depth-claim manifest speaks for the *depth claim* each feature
//! family publishes, this register speaks for the *per-feature scorecard*, the
//! *owner manifest*, and the *rollback/downgrade automation* every M5 feature
//! train carries. Each [`TrainScorecard`] binds one M5 train to:
//!
//! - the stable claim it backs ([`TrainScorecard::claim_ref`],
//!   [`TrainScorecard::claim_label`]),
//! - a per-feature scorecard ([`TrainScorecard::scorecard`]) of one
//!   [`ScorecardCell`] per [`ScorecardAxis`], so functionality, performance,
//!   accessibility, compatibility, localization, and support readiness are each
//!   an explicit, inspectable grade,
//! - an owner manifest ([`TrainScorecard::owner_signoff`]) recording who signed
//!   the claim,
//! - an explicit rollback/downgrade automation record
//!   ([`TrainScorecard::downgrade_automation`]) binding the train to a verified
//!   rollback plan and the trigger and floor it narrows to,
//! - the overall train state earned ([`TrainState`]), the active narrowing
//!   reasons ([`NarrowingReason`]), and the effective label after narrowing
//!   ([`TrainScorecard::published_label`]),
//! - a [`ProofPacket`] (reused from the stable claim manifest) and its freshness
//!   SLO, plus an optional waiver.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between a train that may publish a Stable claim and one that must
//! narrow below it. The [`TrainStopRule`] set names the closed conditions that
//! gate M5 train promotion — one per [`NarrowingReason`] — and
//! [`TrainScorecardRegister::promotion`] records the proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/m5/implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains.json`
//! and embedded here, so this typed consumer and the CI gate agree on every
//! scorecard without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.
//! It carries no raw artifacts, raw logs, signatures, or credential material.

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
pub const IMPLEMENT_M5_TRAIN_SCORECARDS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const IMPLEMENT_M5_TRAIN_SCORECARDS_RECORD_KIND: &str =
    "implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains";

/// Repo-relative path to the checked-in register.
pub const IMPLEMENT_M5_TRAIN_SCORECARDS_PATH: &str =
    "artifacts/release/m5/implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains.json";

/// Embedded checked-in register JSON.
pub const IMPLEMENT_M5_TRAIN_SCORECARDS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains.json"
));

/// M5 feature train a scorecard governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainKind {
    /// Notebook and data-rich notebook depth surfaces.
    Notebook,
    /// Data-heavy surfaces (result grids, variable explorers).
    DataRich,
    /// AI-adjacent surfaces and language intelligence.
    AiAdjacent,
    /// Core framework and platform foundations.
    Framework,
    /// Review and diff surfaces.
    Review,
    /// Browser/mobile companion surfaces.
    Companion,
    /// Managed-depth and infrastructure surfaces.
    ManagedDepth,
}

impl TrainKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Notebook,
        Self::DataRich,
        Self::AiAdjacent,
        Self::Framework,
        Self::Review,
        Self::Companion,
        Self::ManagedDepth,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::DataRich => "data_rich",
            Self::AiAdjacent => "ai_adjacent",
            Self::Framework => "framework",
            Self::Review => "review",
            Self::Companion => "companion",
            Self::ManagedDepth => "managed_depth",
        }
    }
}

/// One axis of the per-feature scorecard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScorecardAxis {
    /// Core functional behavior is verified.
    Functionality,
    /// Hot-path performance is within budget.
    Performance,
    /// Accessibility checks pass.
    Accessibility,
    /// Compatibility across supported surfaces holds.
    Compatibility,
    /// Localization parity holds.
    Localization,
    /// Docs/help/support packets match shipped behavior.
    SupportReadiness,
}

impl ScorecardAxis {
    /// Every axis, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Functionality,
        Self::Performance,
        Self::Accessibility,
        Self::Compatibility,
        Self::Localization,
        Self::SupportReadiness,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Functionality => "functionality",
            Self::Performance => "performance",
            Self::Accessibility => "accessibility",
            Self::Compatibility => "compatibility",
            Self::Localization => "localization",
            Self::SupportReadiness => "support_readiness",
        }
    }

    /// The narrowing reason a non-passing, non-waived cell must name, given the
    /// cell's [`ScoreGrade`].
    pub const fn reason_for_grade(self, grade: ScoreGrade) -> Option<NarrowingReason> {
        match grade {
            ScoreGrade::Missing => Some(NarrowingReason::ScorecardAxisMissing),
            ScoreGrade::Fail | ScoreGrade::Partial => Some(NarrowingReason::ScorecardAxisFailed),
            ScoreGrade::Pass | ScoreGrade::Waived => None,
        }
    }
}

/// The grade earned on one scorecard axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScoreGrade {
    /// The axis fully passes.
    Pass,
    /// The axis partially passes; remediation is required.
    Partial,
    /// The axis fails.
    Fail,
    /// Held provisionally under an active, unexpired waiver.
    Waived,
    /// The axis has no scorecard evidence at all.
    Missing,
}

impl ScoreGrade {
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

    /// Whether a cell in this grade lets the train hold its claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Pass | Self::Waived)
    }
}

/// Overall qualification state a train scorecard earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainState {
    /// Every axis passes, the owner manifest is signed, and rollback/downgrade
    /// automation is defined and verified.
    Qualified,
    /// One or more scorecard axes failed, are partial, or are missing.
    ScorecardRegressed,
    /// The proof packet has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers a
    /// recorded gap.
    OnWaiver,
    /// Rollback/downgrade automation is undefined or its rollback plan is
    /// unverified.
    RollbackUndefined,
    /// The owner manifest is unsigned.
    OwnerUnsigned,
}

impl TrainState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Qualified,
        Self::ScorecardRegressed,
        Self::Stale,
        Self::OnWaiver,
        Self::RollbackUndefined,
        Self::OwnerUnsigned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::ScorecardRegressed => "scorecard_regressed",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::RollbackUndefined => "rollback_undefined",
            Self::OwnerUnsigned => "owner_unsigned",
        }
    }

    /// Whether the state lets a train carry the claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Qualified | Self::OnWaiver)
    }

    /// Whether the state forces the train below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason an M5 train claim narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// A scorecard axis failed or is only partial.
    ScorecardAxisFailed,
    /// A scorecard axis is missing.
    ScorecardAxisMissing,
    /// The proof packet is missing.
    ProofPacketMissing,
    /// The proof packet is stale.
    ProofPacketStale,
    /// The owner manifest is unsigned.
    OwnerManifestUnsigned,
    /// The rollback plan is unverified.
    RollbackPlanUnverified,
    /// The downgrade automation is undefined.
    DowngradeAutomationUndefined,
    /// A waiver the train relied on has expired.
    WaiverExpired,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ScorecardAxisFailed,
        Self::ScorecardAxisMissing,
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
            Self::ScorecardAxisFailed => "scorecard_axis_failed",
            Self::ScorecardAxisMissing => "scorecard_axis_missing",
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

    /// Whether this reason marks a scorecard gap.
    pub const fn is_scorecard_gap(self) -> bool {
        matches!(self, Self::ScorecardAxisFailed | Self::ScorecardAxisMissing)
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
    /// Remediate the failing or missing scorecard axis.
    RemediateScorecard,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Obtain the required owner-manifest sign-off.
    RequestOwnerSignoff,
    /// Verify the rollback plan.
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
        Self::RemediateScorecard,
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
            Self::RemediateScorecard => "remediate_scorecard",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::VerifyRollbackPlan => "verify_rollback_plan",
            Self::DefineDowngradeAutomation => "define_downgrade_automation",
            Self::RenewWaiver => "renew_waiver",
        }
    }
}

/// What triggers a train's automated rollback/downgrade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeTrigger {
    /// Fires when the proof packet goes stale.
    ProofStale,
    /// Fires when the scorecard regresses.
    ScorecardRegressed,
    /// Fires when owner sign-off is revoked.
    OwnerRevoked,
    /// Operator-driven manual downgrade.
    Manual,
}

impl DowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProofStale,
        Self::ScorecardRegressed,
        Self::OwnerRevoked,
        Self::Manual,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::ScorecardRegressed => "scorecard_regressed",
            Self::OwnerRevoked => "owner_revoked",
            Self::Manual => "manual",
        }
    }
}

/// The defined/verified state of a train's rollback/downgrade automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationState {
    /// The automation is defined and its rollback plan is verified.
    Defined,
    /// The automation is defined but its rollback plan is unverified.
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

    /// Whether the automation is defined and verified, letting a train hold a
    /// Stable claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Defined)
    }
}

/// One cell of the per-feature scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScorecardCell {
    /// The scorecard axis this cell speaks for.
    pub axis: ScorecardAxis,
    /// The grade earned for the axis.
    pub grade: ScoreGrade,
    /// Ref to the axis's evidence. Empty only on a missing cell.
    pub evidence_ref: String,
}

/// A train's rollback/downgrade automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DowngradeAutomation {
    /// Stable ref to the automation definition.
    pub automation_ref: String,
    /// Ref to the rollback plan the automation drives.
    pub rollback_plan_ref: String,
    /// What triggers the automated downgrade.
    pub trigger: DowngradeTrigger,
    /// The lifecycle label the automation narrows the train to.
    pub target_floor: StableClaimLevel,
    /// The defined/verified state of the automation.
    pub state: AutomationState,
    /// Whether the rollback plan has been verified end-to-end.
    pub rollback_verified: bool,
}

/// One M5 train-scorecard stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrainStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched scorecard fires this rule.
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

/// One M5 train scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrainScorecard {
    /// Stable scorecard id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The feature train this scorecard governs.
    pub train_kind: TrainKind,
    /// The train ref this scorecard speaks about.
    pub train_ref: String,
    /// Reviewable one-line statement of the train.
    pub train_summary: String,
    /// Whether the train is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this train backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes.
    pub claim_label: StableClaimLevel,
    /// Overall qualification state earned for the scorecard.
    pub train_state: TrainState,
    /// The per-feature scorecard: one cell per [`ScorecardAxis`].
    pub scorecard: Vec<ScorecardCell>,
    /// The rollback/downgrade automation backing the train.
    pub downgrade_automation: DowngradeAutomation,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner manifest sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the scorecard below its claim label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The lifecycle label the train effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this scorecard's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the scorecard carries this posture.
    pub rationale: String,
}

impl TrainScorecard {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the scorecard's state lets the train carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.train_state.holds_label()
    }

    /// True when a narrowing reason is active on the scorecard.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// Returns the cell registered for `axis`, if any.
    pub fn cell(&self, axis: ScorecardAxis) -> Option<&ScorecardCell> {
        self.scorecard.iter().find(|cell| cell.axis == axis)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrainScorecardRegisterSummary {
    /// Total number of train scorecards.
    pub total_entries: usize,
    /// Distinct claims covered.
    pub total_claims: usize,
    /// Scorecards publishing a label at or above the cutline.
    pub entries_qualified: usize,
    /// Scorecards narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Scorecards holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Scorecards carrying a scorecard gap (failed or missing axis).
    pub entries_with_scorecard_gap: usize,
    /// Scorecards carrying an owner-manifest-unsigned reason.
    pub entries_with_owner_gap: usize,
    /// Scorecards carrying a rollback/downgrade automation gap.
    pub entries_with_rollback_gap: usize,
    /// Total release-blocking scorecards.
    pub release_blocking_total: usize,
    /// Release-blocking scorecards publishing a label at or above the cutline.
    pub release_blocking_qualified: usize,
    /// Release-blocking scorecards narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook scorecards.
    pub notebook_entries: usize,
    /// Data-rich scorecards.
    pub data_rich_entries: usize,
    /// AI-adjacent scorecards.
    pub ai_adjacent_entries: usize,
    /// Framework scorecards.
    pub framework_entries: usize,
    /// Review scorecards.
    pub review_entries: usize,
    /// Companion scorecards.
    pub companion_entries: usize,
    /// Managed-depth scorecards.
    pub managed_depth_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all scorecards.
    pub total_active_narrowing_reasons: usize,
    /// Total scorecard cells across all scorecards.
    pub total_scorecard_cells: usize,
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
pub struct TrainScorecardExportRow {
    /// Stable scorecard id.
    pub entry_id: String,
    /// The feature train this scorecard governs.
    pub train_kind: TrainKind,
    /// The train ref this scorecard speaks about.
    pub train_ref: String,
    /// Whether the train is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this train backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the scorecard publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall train state earned.
    pub train_state: TrainState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Rollback/downgrade automation state.
    pub automation_state: AutomationState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrainScorecardExportProjection {
    /// Register identifier.
    pub manifest_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<TrainScorecardExportRow>,
}

/// The typed M5 per-train scorecard register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrainScorecardRegister {
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
    /// Closed train-kind vocabulary.
    pub train_kinds: Vec<TrainKind>,
    /// Closed scorecard-axis vocabulary.
    pub scorecard_axes: Vec<ScorecardAxis>,
    /// Closed score-grade vocabulary.
    pub score_grades: Vec<ScoreGrade>,
    /// Closed train-state vocabulary.
    pub train_states: Vec<TrainState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// Closed downgrade-trigger vocabulary.
    pub downgrade_triggers: Vec<DowngradeTrigger>,
    /// Closed automation-state vocabulary.
    pub automation_states: Vec<AutomationState>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking train refs this register must cover.
    pub release_blocking_train_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<TrainStopRule>,
    /// Train scorecards.
    pub rows: Vec<TrainScorecard>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: TrainScorecardRegisterSummary,
}

impl TrainScorecardRegister {
    /// Returns the scorecard registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&TrainScorecard> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the scorecards publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&TrainScorecard> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the scorecards narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&TrainScorecard> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking scorecards.
    pub fn release_blocking_rows(&self) -> Vec<&TrainScorecard> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the scorecards for one train kind.
    pub fn rows_for_kind(&self, kind: TrainKind) -> Vec<&TrainScorecard> {
        self.rows
            .iter()
            .filter(|row| row.train_kind == kind)
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

    /// True when `rule` fires: a watched scorecard carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &TrainStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the scorecards and stop rules.
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

    /// Scorecard ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only scorecards whose claim is at or above the cutline count: a scorecard
    /// whose claim is already canonically narrowed is not a *promotion* blocker,
    /// it merely inherits the upstream ceiling.
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

    /// Recomputes the summary block from the scorecards and stop rules.
    pub fn computed_summary(&self) -> TrainScorecardRegisterSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: TrainKind| self.rows_for_kind(kind).len();
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
        let cell_grade = |grade: ScoreGrade| {
            self.rows
                .iter()
                .flat_map(|row| row.scorecard.iter())
                .filter(|cell| cell.grade == grade)
                .count()
        };
        let release_blocking: Vec<&TrainScorecard> = self.release_blocking_rows();
        TrainScorecardRegisterSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_qualified: self
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
            entries_with_scorecard_gap: with_predicate(NarrowingReason::is_scorecard_gap),
            entries_with_owner_gap: with_reason(NarrowingReason::OwnerManifestUnsigned),
            entries_with_rollback_gap: with_predicate(NarrowingReason::is_rollback_gap),
            release_blocking_total: release_blocking.len(),
            release_blocking_qualified: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            notebook_entries: kind(TrainKind::Notebook),
            data_rich_entries: kind(TrainKind::DataRich),
            ai_adjacent_entries: kind(TrainKind::AiAdjacent),
            framework_entries: kind(TrainKind::Framework),
            review_entries: kind(TrainKind::Review),
            companion_entries: kind(TrainKind::Companion),
            managed_depth_entries: kind(TrainKind::ManagedDepth),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_narrowing_reasons: self
                .rows
                .iter()
                .map(|row| row.active_narrowing_reasons.len())
                .sum(),
            total_scorecard_cells: self.rows.iter().map(|row| row.scorecard.len()).sum(),
            cells_pass: cell_grade(ScoreGrade::Pass),
            cells_partial: cell_grade(ScoreGrade::Partial),
            cells_fail: cell_grade(ScoreGrade::Fail),
            cells_waived: cell_grade(ScoreGrade::Waived),
            cells_missing: cell_grade(ScoreGrade::Missing),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection that downstream surfaces
    /// render instead of cloning status text.
    pub fn support_export_projection(&self) -> TrainScorecardExportProjection {
        TrainScorecardExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| TrainScorecardExportRow {
                    entry_id: row.entry_id.clone(),
                    train_kind: row.train_kind,
                    train_ref: row.train_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    train_state: row.train_state,
                    slo_state: row.proof_packet.slo_state,
                    automation_state: row.downgrade_automation.state,
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<TrainScorecardRegisterViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(TrainScorecardRegisterViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(TrainScorecardRegisterViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(TrainScorecardRegisterViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<TrainScorecardRegisterViolation>) {
        if self.schema_version != IMPLEMENT_M5_TRAIN_SCORECARDS_SCHEMA_VERSION {
            violations.push(TrainScorecardRegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != IMPLEMENT_M5_TRAIN_SCORECARDS_RECORD_KIND {
            violations.push(TrainScorecardRegisterViolation::UnsupportedRecordKind {
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
                violations.push(TrainScorecardRegisterViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(TrainScorecardRegisterViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.train_kinds != TrainKind::ALL.to_vec() {
            violations.push(TrainScorecardRegisterViolation::ClosedVocabularyMismatch {
                field: "train_kinds",
            });
        }
        if self.scorecard_axes != ScorecardAxis::ALL.to_vec() {
            violations.push(TrainScorecardRegisterViolation::ClosedVocabularyMismatch {
                field: "scorecard_axes",
            });
        }
        if self.score_grades != ScoreGrade::ALL.to_vec() {
            violations.push(TrainScorecardRegisterViolation::ClosedVocabularyMismatch {
                field: "score_grades",
            });
        }
        if self.train_states != TrainState::ALL.to_vec() {
            violations.push(TrainScorecardRegisterViolation::ClosedVocabularyMismatch {
                field: "train_states",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(TrainScorecardRegisterViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.stop_rule_actions != StopAction::ALL.to_vec() {
            violations.push(TrainScorecardRegisterViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }
        if self.downgrade_triggers != DowngradeTrigger::ALL.to_vec() {
            violations.push(TrainScorecardRegisterViolation::ClosedVocabularyMismatch {
                field: "downgrade_triggers",
            });
        }
        if self.automation_states != AutomationState::ALL.to_vec() {
            violations.push(TrainScorecardRegisterViolation::ClosedVocabularyMismatch {
                field: "automation_states",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(TrainScorecardRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(TrainScorecardRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(TrainScorecardRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(TrainScorecardRegisterViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<TrainScorecardRegisterViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(TrainScorecardRegisterViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(TrainScorecardRegisterViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(TrainScorecardRegisterViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(TrainScorecardRegisterViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations.push(
                    TrainScorecardRegisterViolation::NarrowingReasonWithoutStopRule { reason },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &TrainScorecard,
        violations: &mut Vec<TrainScorecardRegisterViolation>,
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
                "downgrade_automation.automation_ref",
                &row.downgrade_automation.automation_ref,
            ),
            (
                "downgrade_automation.rollback_plan_ref",
                &row.downgrade_automation.rollback_plan_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(TrainScorecardRegisterViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_scorecard(row, violations);
        self.validate_automation(row, violations);

        // The ceiling: no train may carry a label wider than the claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(TrainScorecardRegisterViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(TrainScorecardRegisterViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(TrainScorecardRegisterViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A claim whose canonical label is below the cutline forces the train to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(TrainScorecardRegisterViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(TrainScorecardRegisterViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.train_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed train publishes exactly the claim's canonical label,
            // carries no active reason, rides a captured within-SLO packet, is
            // owner-signed, and rides defined-and-verified downgrade automation.
            if row.published_label != row.claim_label {
                violations.push(TrainScorecardRegisterViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(TrainScorecardRegisterViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(TrainScorecardRegisterViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(TrainScorecardRegisterViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(TrainScorecardRegisterViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.downgrade_automation.state.holds()
                || !row.downgrade_automation.rollback_verified
            {
                violations.push(
                    TrainScorecardRegisterViolation::HeldWithoutDowngradeAutomation {
                        entry_id: row.entry_id.clone(),
                        state: row.downgrade_automation.state,
                    },
                );
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(TrainScorecardRegisterViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.train_state,
                    published: row.published_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(TrainScorecardRegisterViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.train_state,
                });
            }
            // A narrowing train whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::ProofPacketStale)
            {
                violations.push(
                    TrainScorecardRegisterViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
            {
                violations.push(
                    TrainScorecardRegisterViolation::MissingPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_scorecard(
        &self,
        row: &TrainScorecard,
        violations: &mut Vec<TrainScorecardRegisterViolation>,
    ) {
        let mut seen: BTreeSet<ScorecardAxis> = BTreeSet::new();
        for cell in &row.scorecard {
            if !seen.insert(cell.axis) {
                violations.push(TrainScorecardRegisterViolation::DuplicateAxis {
                    entry_id: row.entry_id.clone(),
                    axis: cell.axis,
                });
            }
            // A missing cell carries no evidence ref; every other grade must.
            if cell.grade != ScoreGrade::Missing && cell.evidence_ref.trim().is_empty() {
                violations.push(TrainScorecardRegisterViolation::CellEvidenceMissing {
                    entry_id: row.entry_id.clone(),
                    axis: cell.axis,
                });
            }
            // A waived cell only holds under an unexpired waiver.
            if cell.grade == ScoreGrade::Waived && row.waiver.is_none() {
                violations.push(TrainScorecardRegisterViolation::WaivedCellWithoutWaiver {
                    entry_id: row.entry_id.clone(),
                    axis: cell.axis,
                });
            }
            // A non-passing, non-waived cell must name its narrowing reason.
            if !cell.grade.holds() {
                if let Some(reason) = cell.axis.reason_for_grade(cell.grade) {
                    if !row.has_active_reason(reason) {
                        violations.push(TrainScorecardRegisterViolation::CellReasonNotActive {
                            entry_id: row.entry_id.clone(),
                            axis: cell.axis,
                            reason,
                        });
                    }
                }
            }
        }
        // The scorecard must carry exactly one cell per axis.
        for axis in ScorecardAxis::ALL {
            if !seen.contains(&axis) {
                violations.push(
                    TrainScorecardRegisterViolation::ScorecardIncompleteCoverage {
                        entry_id: row.entry_id.clone(),
                        axis,
                    },
                );
            }
        }
    }

    fn validate_automation(
        &self,
        row: &TrainScorecard,
        violations: &mut Vec<TrainScorecardRegisterViolation>,
    ) {
        let automation = &row.downgrade_automation;
        // A downgrade narrows the claim, so its floor must be below the cutline.
        if automation.target_floor.is_at_or_above_cutline() {
            violations.push(
                TrainScorecardRegisterViolation::DowngradeFloorNotBelowCutline {
                    entry_id: row.entry_id.clone(),
                    floor: automation.target_floor,
                },
            );
        }
        // An undefined automation must name the undefined reason.
        if automation.state == AutomationState::Undefined
            && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
        {
            violations.push(
                TrainScorecardRegisterViolation::AutomationStateWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: automation.state,
                },
            );
        }
        // An unverified rollback plan must name a rollback or downgrade reason.
        if !automation.rollback_verified
            && !row.has_active_reason(NarrowingReason::RollbackPlanUnverified)
            && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
        {
            violations.push(
                TrainScorecardRegisterViolation::RollbackUnverifiedWithoutReason {
                    entry_id: row.entry_id.clone(),
                },
            );
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &TrainScorecard,
        violations: &mut Vec<TrainScorecardRegisterViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<TrainScorecardRegisterViolation>,
                               expected: NarrowingReason| {
            violations.push(TrainScorecardRegisterViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.train_state,
                expected_reason: expected,
            });
        };

        match row.train_state {
            TrainState::ScorecardRegressed => {
                if !row.has_active_reason(NarrowingReason::ScorecardAxisFailed)
                    && !row.has_active_reason(NarrowingReason::ScorecardAxisMissing)
                {
                    push_incoherent(violations, NarrowingReason::ScorecardAxisFailed);
                }
            }
            TrainState::Stale => {
                if !row.has_active_reason(NarrowingReason::ProofPacketStale) {
                    push_incoherent(violations, NarrowingReason::ProofPacketStale);
                }
            }
            TrainState::RollbackUndefined => {
                if !row.has_active_reason(NarrowingReason::RollbackPlanUnverified)
                    && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
                {
                    push_incoherent(violations, NarrowingReason::DowngradeAutomationUndefined);
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
                    violations.push(TrainScorecardRegisterViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.train_state,
                    });
                }
            }
            TrainState::Qualified => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<TrainScorecardRegisterViolation>) {
        let covered: BTreeSet<String> = self.rows.iter().map(|row| row.train_ref.clone()).collect();
        for declared in &self.release_blocking_train_refs {
            if !covered.contains(declared) {
                violations.push(
                    TrainScorecardRegisterViolation::ReleaseBlockingTrainUncovered {
                        train_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_train_refs.contains(&row.train_ref) {
                violations.push(
                    TrainScorecardRegisterViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<TrainScorecardRegisterViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(TrainScorecardRegisterViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(TrainScorecardRegisterViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(
                TrainScorecardRegisterViolation::PromotionDecisionInconsistent {
                    declared: self.promotion.decision,
                    computed,
                },
            );
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                TrainScorecardRegisterViolation::PromotionBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(
                TrainScorecardRegisterViolation::PromotionBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// A validation violation for the M5 per-train scorecard register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrainScorecardRegisterViolation {
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
    /// The register has no scorecards.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Scorecard or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A scorecard id appears more than once.
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
    /// A scorecard has two cells for one axis.
    DuplicateAxis {
        /// Scorecard id.
        entry_id: String,
        /// Duplicated axis.
        axis: ScorecardAxis,
    },
    /// A scorecard is missing an axis cell.
    ScorecardIncompleteCoverage {
        /// Scorecard id.
        entry_id: String,
        /// Uncovered axis.
        axis: ScorecardAxis,
    },
    /// A non-missing cell has no evidence ref.
    CellEvidenceMissing {
        /// Scorecard id.
        entry_id: String,
        /// Axis.
        axis: ScorecardAxis,
    },
    /// A waived cell is carried without a waiver.
    WaivedCellWithoutWaiver {
        /// Scorecard id.
        entry_id: String,
        /// Axis.
        axis: ScorecardAxis,
    },
    /// A non-passing cell does not name its narrowing reason.
    CellReasonNotActive {
        /// Scorecard id.
        entry_id: String,
        /// Axis.
        axis: ScorecardAxis,
        /// The reason the cell requires.
        reason: NarrowingReason,
    },
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Scorecard id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A scorecard holds a label while the claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Scorecard id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing state carries no active reason.
    NarrowingWithoutReason {
        /// Scorecard id.
        entry_id: String,
        /// Train state.
        state: TrainState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Scorecard id.
        entry_id: String,
        /// Train state.
        state: TrainState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held scorecard carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Scorecard id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held scorecard has active narrowing reasons.
    HeldWithActiveGap {
        /// Scorecard id.
        entry_id: String,
    },
    /// A held scorecard has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Scorecard id.
        entry_id: String,
    },
    /// A held scorecard rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Scorecard id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held scorecard lacks owner-manifest sign-off.
    HeldWithoutSignoff {
        /// Scorecard id.
        entry_id: String,
    },
    /// A held scorecard lacks defined-and-verified downgrade automation.
    HeldWithoutDowngradeAutomation {
        /// Scorecard id.
        entry_id: String,
        /// Automation state.
        state: AutomationState,
    },
    /// The downgrade automation floor is not below the cutline.
    DowngradeFloorNotBelowCutline {
        /// Scorecard id.
        entry_id: String,
        /// Declared floor.
        floor: StableClaimLevel,
    },
    /// An undefined automation does not name the undefined reason.
    AutomationStateWithoutReason {
        /// Scorecard id.
        entry_id: String,
        /// Automation state.
        state: AutomationState,
    },
    /// An unverified rollback plan does not name a rollback/downgrade reason.
    RollbackUnverifiedWithoutReason {
        /// Scorecard id.
        entry_id: String,
    },
    /// A narrowing scorecard with a breached proof packet does not name the stale reason.
    BreachedPacketWithoutReason {
        /// Scorecard id.
        entry_id: String,
    },
    /// A narrowing scorecard with a missing proof packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Scorecard id.
        entry_id: String,
    },
    /// A train state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Scorecard id.
        entry_id: String,
        /// Train state.
        state: TrainState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Scorecard id.
        entry_id: String,
        /// Train state.
        state: TrainState,
    },
    /// A release-blocking train ref has no covering scorecard.
    ReleaseBlockingTrainUncovered {
        /// Train ref.
        train_ref: String,
    },
    /// A release-blocking scorecard is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Scorecard id.
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
    /// The summary counts disagree with the scorecards.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Scorecard id.
        entry_id: String,
    },
}

impl fmt::Display for TrainScorecardRegisterViolation {
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
            Self::EmptyRegister => write!(f, "register has no scorecards"),
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
            Self::DuplicateAxis { entry_id, axis } => {
                write!(f, "scorecard {entry_id} has duplicate axis {}", axis.as_str())
            }
            Self::ScorecardIncompleteCoverage { entry_id, axis } => write!(
                f,
                "scorecard {entry_id} is missing axis {}",
                axis.as_str()
            ),
            Self::CellEvidenceMissing { entry_id, axis } => write!(
                f,
                "scorecard {entry_id} axis {} has no evidence ref",
                axis.as_str()
            ),
            Self::WaivedCellWithoutWaiver { entry_id, axis } => write!(
                f,
                "scorecard {entry_id} axis {} is waived without a waiver",
                axis.as_str()
            ),
            Self::CellReasonNotActive {
                entry_id,
                axis,
                reason,
            } => write!(
                f,
                "scorecard {entry_id} axis {} requires active reason {}",
                axis.as_str(),
                reason.as_str()
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "scorecard {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "scorecard {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "scorecard {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "scorecard {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "scorecard {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "scorecard {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "scorecard {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "scorecard {entry_id} holds stable on stale packet {slo_state:?}"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "scorecard {entry_id} holds stable without owner signoff")
            }
            Self::HeldWithoutDowngradeAutomation { entry_id, state } => write!(
                f,
                "scorecard {entry_id} holds stable without defined+verified downgrade automation ({state:?})"
            ),
            Self::DowngradeFloorNotBelowCutline { entry_id, floor } => write!(
                f,
                "scorecard {entry_id} downgrade floor {floor:?} is not below the cutline"
            ),
            Self::AutomationStateWithoutReason { entry_id, state } => write!(
                f,
                "scorecard {entry_id} automation state {state:?} names no narrowing reason"
            ),
            Self::RollbackUnverifiedWithoutReason { entry_id } => write!(
                f,
                "scorecard {entry_id} has an unverified rollback plan without a reason"
            ),
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "scorecard {entry_id} breached packet without proof_packet_stale reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "scorecard {entry_id} missing packet without proof_packet_missing reason"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "scorecard {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "scorecard {entry_id} state {state:?} names no waiver")
            }
            Self::ReleaseBlockingTrainUncovered { train_ref } => {
                write!(
                    f,
                    "release-blocking train {train_ref} has no covering scorecard"
                )
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking scorecard {entry_id} is not declared in release_blocking_train_refs"
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => {
                write!(f, "promotion {declared:?} disagrees with computed {computed:?}")
            }
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with scorecards"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "scorecard {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for TrainScorecardRegisterViolation {}

/// Loads the embedded M5 per-train scorecard register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`TrainScorecardRegister`].
pub fn current_m5_train_scorecard_register() -> Result<TrainScorecardRegister, serde_json::Error> {
    serde_json::from_str(IMPLEMENT_M5_TRAIN_SCORECARDS_JSON)
}

#[cfg(test)]
mod tests;
