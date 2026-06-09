//! Typed M5 rollback, downgrade, claim-narrowing, and staged-promotion rules.
//!
//! This module freezes the canonical control register that governs how every M5
//! depth lane rolls back, downgrades, narrows its claim, and advances through
//! staged promotion. Each [`M5RollbackDowngradeRow`] binds one M5 lane to:
//!
//! - a [`RollbackPathState`] that records whether the lane's rollback path is
//!   defined, tested, exercised, or missing,
//! - a set of [`M5DowngradeRule`] entries that prescribe the downgrade behavior
//!   when the lane narrows,
//! - a set of [`M5ClaimNarrowingRule`] entries that automate claim narrowing
//!   when specific gap reasons are active,
//! - a set of [`M5PromotionStage`] records that track the staged-promotion
//!   posture (canary, pilot, stable) for the lane,
//! - the register state earned ([`M5RollbackDowngradeState`]), the active gap
//!   reasons ([`M5RollbackGapReason`]), and the effective label after narrowing
//!   ([`M5RollbackDowngradeRow::published_label`]),
//! - a proof packet with freshness SLO ([`ProofPacket`]),
//! - owner sign-off ([`OwnerSignoff`]),
//! - stop rules that gate promotion when rollback paths are missing, downgrade
//!   rules are incomplete, claim-narrowing automation is absent, or promotion
//!   stages are blocked.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between a lane that may publish as Stable and one that must narrow
//! below it. The [`M5RollbackStopRule`] set names the closed conditions that
//! gate M5 promotion, and [`M5RollbackDowngradeRegister::promotion`] records
//! the proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/m5/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.
//! It carries no raw artifacts, raw logs, signatures, or credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_feature_train_matrix_scorecards_and_dependency_graph::M5LaneKind;
use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported register schema version.
pub const FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_RECORD_KIND: &str =
    "freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules";

/// Repo-relative path to the checked-in register.
pub const FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_PATH: &str =
    "artifacts/release/m5/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules.json";

/// Embedded checked-in register JSON.
pub const FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_JSON: &str =
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules.json"
    ));

/// Rollback path state a lane earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackPathState {
    /// Rollback path is defined but not yet tested.
    Defined,
    /// Rollback path has been tested in a dry-run.
    Tested,
    /// Rollback path has been exercised in a real rehearsal.
    Exercised,
    /// No rollback path is defined.
    Missing,
}

impl RollbackPathState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Defined, Self::Tested, Self::Exercised, Self::Missing];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Defined => "defined",
            Self::Tested => "tested",
            Self::Exercised => "exercised",
            Self::Missing => "missing",
        }
    }

    /// Whether the state allows the lane to hold a stable claim.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Tested | Self::Exercised)
    }

    /// Whether the state forces the lane below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Downgrade kind that applies when a lane narrows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeKind {
    /// Automatic label narrowing when evidence goes stale.
    AutomaticNarrowing,
    /// Manual hold required before narrowing.
    ManualHold,
    /// Emergency rollback protocol.
    EmergencyRollback,
    /// Staged reversal through promotion stages.
    StagedReversal,
}

impl DowngradeKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AutomaticNarrowing,
        Self::ManualHold,
        Self::EmergencyRollback,
        Self::StagedReversal,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutomaticNarrowing => "automatic_narrowing",
            Self::ManualHold => "manual_hold",
            Self::EmergencyRollback => "emergency_rollback",
            Self::StagedReversal => "staged_reversal",
        }
    }
}

/// Staged promotion stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionStageKind {
    /// Canary stage.
    Canary,
    /// Pilot stage.
    Pilot,
    /// Stable stage.
    Stable,
}

impl PromotionStageKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::Canary, Self::Pilot, Self::Stable];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Canary => "canary",
            Self::Pilot => "pilot",
            Self::Stable => "stable",
        }
    }
}

/// State of a promotion stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageState {
    /// Stage is complete.
    Complete,
    /// Stage is in progress.
    InProgress,
    /// Stage has not started.
    NotStarted,
    /// Stage is blocked.
    Blocked,
}

impl StageState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Complete,
        Self::InProgress,
        Self::NotStarted,
        Self::Blocked,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::InProgress => "in_progress",
            Self::NotStarted => "not_started",
            Self::Blocked => "blocked",
        }
    }

    /// Whether the stage allows progression to the next stage.
    pub const fn allows_progression(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Register state a lane earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackDowngradeState {
    /// All rollback, downgrade, narrowing, and promotion rules are present and current.
    Complete,
    /// One or more required rules are missing.
    Incomplete,
    /// A rule or packet has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers a recorded gap.
    OnWaiver,
    /// Blocked by a missing rollback path or blocked promotion stage.
    Blocked,
    /// Missing a defined downgrade or claim-narrowing rule.
    RuleMissing,
}

impl M5RollbackDowngradeState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Complete,
        Self::Incomplete,
        Self::Stale,
        Self::OnWaiver,
        Self::Blocked,
        Self::RuleMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::Blocked => "blocked",
            Self::RuleMissing => "rule_missing",
        }
    }

    /// Whether the state lets a lane carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Complete | Self::OnWaiver)
    }

    /// Whether the state forces the lane below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason an M5 rollback/downgrade lane narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackGapReason {
    /// Rollback path is missing.
    RollbackPathMissing,
    /// Rollback path is defined but not tested.
    RollbackPathUntested,
    /// Rollback path is tested but not exercised.
    RollbackPathUnexercised,
    /// Downgrade rule is missing.
    DowngradeRuleMissing,
    /// Claim-narrowing rule is missing.
    ClaimNarrowingRuleMissing,
    /// Staged-promotion rule is missing.
    StagedPromotionRuleMissing,
    /// A promotion stage is blocked.
    PromotionStageBlocked,
    /// Proof packet is missing.
    ProofPacketMissing,
    /// Proof packet is stale.
    ProofPacketStale,
    /// A waiver the lane relied on has expired.
    WaiverExpired,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl M5RollbackGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::RollbackPathMissing,
        Self::RollbackPathUntested,
        Self::RollbackPathUnexercised,
        Self::DowngradeRuleMissing,
        Self::ClaimNarrowingRuleMissing,
        Self::StagedPromotionRuleMissing,
        Self::PromotionStageBlocked,
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RollbackPathMissing => "rollback_path_missing",
            Self::RollbackPathUntested => "rollback_path_untested",
            Self::RollbackPathUnexercised => "rollback_path_unexercised",
            Self::DowngradeRuleMissing => "downgrade_rule_missing",
            Self::ClaimNarrowingRuleMissing => "claim_narrowing_rule_missing",
            Self::StagedPromotionRuleMissing => "staged_promotion_rule_missing",
            Self::PromotionStageBlocked => "promotion_stage_blocked",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackAction {
    /// Hold promotion until the condition clears.
    HoldPromotion,
    /// Narrow the public claim below the cutline.
    NarrowLabel,
    /// Define the rollback path.
    DefineRollbackPath,
    /// Test the rollback path in a dry-run.
    TestRollbackPath,
    /// Exercise the rollback path in a rehearsal.
    ExerciseRollbackPath,
    /// Define the downgrade rule.
    DefineDowngradeRule,
    /// Define the claim-narrowing rule.
    DefineClaimNarrowingRule,
    /// Define the staged-promotion rule.
    DefineStagedPromotionRule,
    /// Advance the promotion stage.
    AdvancePromotionStage,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl M5RollbackAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::HoldPromotion,
        Self::NarrowLabel,
        Self::DefineRollbackPath,
        Self::TestRollbackPath,
        Self::ExerciseRollbackPath,
        Self::DefineDowngradeRule,
        Self::DefineClaimNarrowingRule,
        Self::DefineStagedPromotionRule,
        Self::AdvancePromotionStage,
        Self::RefreshProofPacket,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowLabel => "narrow_label",
            Self::DefineRollbackPath => "define_rollback_path",
            Self::TestRollbackPath => "test_rollback_path",
            Self::ExerciseRollbackPath => "exercise_rollback_path",
            Self::DefineDowngradeRule => "define_downgrade_rule",
            Self::DefineClaimNarrowingRule => "define_claim_narrowing_rule",
            Self::DefineStagedPromotionRule => "define_staged_promotion_rule",
            Self::AdvancePromotionStage => "advance_promotion_stage",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One staged-promotion stage record for an M5 lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PromotionStage {
    /// The stage kind.
    pub stage_kind: PromotionStageKind,
    /// The stage state.
    pub stage_state: StageState,
    /// Ref to the stage evidence packet.
    pub stage_ref: String,
    /// UTC date the stage completed, or null.
    #[serde(default)]
    pub completed_at: Option<String>,
    /// Reviewable rationale for the stage posture.
    pub rationale: String,
}

/// One downgrade rule for an M5 lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5DowngradeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// The downgrade kind.
    pub downgrade_kind: DowngradeKind,
    /// The label that triggers this rule.
    pub trigger_label: StableClaimLevel,
    /// The label the lane narrows to when the rule fires.
    pub target_label: StableClaimLevel,
    /// Ref to the rule definition artifact.
    pub rule_ref: String,
    /// Reviewable rationale for the rule.
    pub rationale: String,
}

/// One claim-narrowing rule for an M5 lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ClaimNarrowingRule {
    /// Stable rule id.
    pub rule_id: String,
    /// The gap reason that triggers this rule.
    pub trigger_reason: M5RollbackGapReason,
    /// The label the lane narrows to when the rule fires.
    pub target_label: StableClaimLevel,
    /// Whether the rule applies automatically.
    pub auto_apply: bool,
    /// Ref to the rule definition artifact.
    pub rule_ref: String,
    /// Reviewable rationale for the rule.
    pub rationale: String,
}

/// One M5 rollback/downgrade stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RollbackStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: M5RollbackGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: M5RollbackAction,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One M5 rollback/downgrade row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RollbackDowngradeRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The lane kind this row governs.
    pub lane_kind: M5LaneKind,
    /// The surface id this lane speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the lane.
    pub surface_summary: String,
    /// Whether the lane is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this lane backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// Rollback path state earned for the row.
    pub rollback_path_state: RollbackPathState,
    /// Ref to the rollback path definition artifact.
    pub rollback_path_ref: String,
    /// Downgrade rules bound to the lane.
    #[serde(default)]
    pub downgrade_rules: Vec<M5DowngradeRule>,
    /// Claim-narrowing rules bound to the lane.
    #[serde(default)]
    pub claim_narrowing_rules: Vec<M5ClaimNarrowingRule>,
    /// Staged-promotion stages bound to the lane.
    #[serde(default)]
    pub promotion_stages: Vec<M5PromotionStage>,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<M5RollbackGapReason>,
    /// The lifecycle label the lane effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl M5RollbackDowngradeRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's rollback path state lets the lane carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.rollback_path_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: M5RollbackGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// True when all promotion stages are complete.
    pub fn stages_complete(&self) -> bool {
        self.promotion_stages
            .iter()
            .all(|s| s.stage_state.allows_progression())
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RollbackDowngradeSummary {
    /// Total number of lane rows.
    pub total_entries: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows publishing a label at or above the cutline.
    pub entries_holding_stable: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Rows blocked by a missing rollback path or blocked stage.
    pub entries_blocked: usize,
    /// Rows missing a downgrade or narrowing rule.
    pub entries_rule_missing: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_holding: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook rows.
    pub notebook_entries: usize,
    /// Data-rich rows.
    pub data_rich_entries: usize,
    /// AI-adjacent rows.
    pub ai_adjacent_entries: usize,
    /// Framework rows.
    pub framework_entries: usize,
    /// Review rows.
    pub review_entries: usize,
    /// Companion rows.
    pub companion_entries: usize,
    /// Managed-depth rows.
    pub managed_depth_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
    /// Total downgrade rules across all rows.
    pub total_downgrade_rules: usize,
    /// Total claim-narrowing rules across all rows.
    pub total_claim_narrowing_rules: usize,
    /// Total promotion stages across all rows.
    pub total_promotion_stages: usize,
    /// Rows with a missing rollback path.
    pub rollback_path_missing: usize,
    /// Rows with an untested rollback path.
    pub rollback_path_untested: usize,
    /// Rows with an unexercised rollback path.
    pub rollback_path_unexercised: usize,
    /// Rows with at least one blocked promotion stage.
    pub stages_blocked: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RollbackDowngradeExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The lane kind this row governs.
    pub lane_kind: M5LaneKind,
    /// The surface id this lane speaks about.
    pub surface_ref: String,
    /// Whether the lane is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this lane backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Rollback path state earned.
    pub rollback_path_state: RollbackPathState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<M5RollbackGapReason>,
    /// Number of downgrade rules.
    pub downgrade_rule_count: usize,
    /// Number of claim-narrowing rules.
    pub claim_narrowing_rule_count: usize,
    /// Number of promotion stages.
    pub promotion_stage_count: usize,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RollbackDowngradeExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5RollbackDowngradeExportRow>,
}

/// The typed M5 rollback/downgrade register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RollbackDowngradeRegister {
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
    /// Ref to the M5 feature-train matrix this register extends.
    pub feature_train_matrix_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed lane-kind vocabulary.
    pub lane_kinds: Vec<M5LaneKind>,
    /// Closed rollback-path-state vocabulary.
    pub rollback_path_states: Vec<RollbackPathState>,
    /// Closed downgrade-kind vocabulary.
    pub downgrade_kinds: Vec<DowngradeKind>,
    /// Closed promotion-stage-kind vocabulary.
    pub promotion_stage_kinds: Vec<PromotionStageKind>,
    /// Closed stage-state vocabulary.
    pub stage_states: Vec<StageState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<M5RollbackGapReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<M5RollbackAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_lane_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<M5RollbackStopRule>,
    /// Lane rows.
    pub rows: Vec<M5RollbackDowngradeRow>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5RollbackDowngradeSummary,
}

impl M5RollbackDowngradeRegister {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&M5RollbackDowngradeRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&M5RollbackDowngradeRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&M5RollbackDowngradeRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&M5RollbackDowngradeRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one lane kind.
    pub fn rows_for_kind(&self, kind: M5LaneKind) -> Vec<&M5RollbackDowngradeRow> {
        self.rows
            .iter()
            .filter(|row| row.lane_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the register covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &M5RollbackStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
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

    /// Lane-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row
    /// whose claim is already canonically narrowed is not a *promotion*
    /// blocker, it merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<M5RollbackGapReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_gap_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and stop rules.
    pub fn computed_summary(&self) -> M5RollbackDowngradeSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: M5LaneKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&M5RollbackDowngradeRow> = self.release_blocking_rows();
        M5RollbackDowngradeSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_holding_stable: self
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
                .filter(|row| {
                    row.waiver
                        .as_ref()
                        .map(|w| !w.waiver_ref.trim().is_empty())
                        .unwrap_or(false)
                })
                .count(),
            entries_blocked: self
                .rows
                .iter()
                .filter(|row| row.has_active_reason(M5RollbackGapReason::PromotionStageBlocked))
                .count(),
            entries_rule_missing: self
                .rows
                .iter()
                .filter(|row| {
                    row.has_active_reason(M5RollbackGapReason::DowngradeRuleMissing)
                        || row.has_active_reason(M5RollbackGapReason::ClaimNarrowingRuleMissing)
                        || row.has_active_reason(M5RollbackGapReason::StagedPromotionRuleMissing)
                })
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_holding: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            notebook_entries: kind(M5LaneKind::Notebook),
            data_rich_entries: kind(M5LaneKind::DataRich),
            ai_adjacent_entries: kind(M5LaneKind::AiAdjacent),
            framework_entries: kind(M5LaneKind::Framework),
            review_entries: kind(M5LaneKind::Review),
            companion_entries: kind(M5LaneKind::Companion),
            managed_depth_entries: kind(M5LaneKind::ManagedDepth),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
            total_downgrade_rules: self.rows.iter().map(|row| row.downgrade_rules.len()).sum(),
            total_claim_narrowing_rules: self
                .rows
                .iter()
                .map(|row| row.claim_narrowing_rules.len())
                .sum(),
            total_promotion_stages: self.rows.iter().map(|row| row.promotion_stages.len()).sum(),
            rollback_path_missing: self
                .rows
                .iter()
                .filter(|row| row.rollback_path_state == RollbackPathState::Missing)
                .count(),
            rollback_path_untested: self
                .rows
                .iter()
                .filter(|row| row.rollback_path_state == RollbackPathState::Defined)
                .count(),
            rollback_path_unexercised: self
                .rows
                .iter()
                .filter(|row| row.rollback_path_state == RollbackPathState::Tested)
                .count(),
            stages_blocked: self
                .rows
                .iter()
                .filter(|row| {
                    row.promotion_stages
                        .iter()
                        .any(|s| s.stage_state == StageState::Blocked)
                })
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the register that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> M5RollbackDowngradeExportProjection {
        M5RollbackDowngradeExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5RollbackDowngradeExportRow {
                    entry_id: row.entry_id.clone(),
                    lane_kind: row.lane_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    rollback_path_state: row.rollback_path_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                    downgrade_rule_count: row.downgrade_rules.len(),
                    claim_narrowing_rule_count: row.claim_narrowing_rules.len(),
                    promotion_stage_count: row.promotion_stages.len(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<M5RollbackDowngradeViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5RollbackDowngradeViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5RollbackDowngradeViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5RollbackDowngradeViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5RollbackDowngradeViolation>) {
        if self.schema_version
            != FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_SCHEMA_VERSION
        {
            violations.push(M5RollbackDowngradeViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind
            != FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_RECORD_KIND
        {
            violations.push(M5RollbackDowngradeViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("feature_train_matrix_ref", &self.feature_train_matrix_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5RollbackDowngradeViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5RollbackDowngradeViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.lane_kinds != M5LaneKind::ALL.to_vec() {
            violations.push(M5RollbackDowngradeViolation::ClosedVocabularyMismatch {
                field: "lane_kinds",
            });
        }
        if self.rollback_path_states != RollbackPathState::ALL.to_vec() {
            violations.push(M5RollbackDowngradeViolation::ClosedVocabularyMismatch {
                field: "rollback_path_states",
            });
        }
        if self.downgrade_kinds != DowngradeKind::ALL.to_vec() {
            violations.push(M5RollbackDowngradeViolation::ClosedVocabularyMismatch {
                field: "downgrade_kinds",
            });
        }
        if self.promotion_stage_kinds != PromotionStageKind::ALL.to_vec() {
            violations.push(M5RollbackDowngradeViolation::ClosedVocabularyMismatch {
                field: "promotion_stage_kinds",
            });
        }
        if self.stage_states != StageState::ALL.to_vec() {
            violations.push(M5RollbackDowngradeViolation::ClosedVocabularyMismatch {
                field: "stage_states",
            });
        }
        if self.gap_reasons != M5RollbackGapReason::ALL.to_vec() {
            violations.push(M5RollbackDowngradeViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.stop_rule_actions != M5RollbackAction::ALL.to_vec() {
            violations.push(M5RollbackDowngradeViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5RollbackDowngradeViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5RollbackDowngradeViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5RollbackDowngradeViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5RollbackDowngradeViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5RollbackDowngradeViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5RollbackDowngradeViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5RollbackDowngradeViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5RollbackDowngradeViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5RollbackDowngradeViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in M5RollbackGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(M5RollbackDowngradeViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &M5RollbackDowngradeRow,
        violations: &mut Vec<M5RollbackDowngradeViolation>,
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
        ] {
            if value.trim().is_empty() {
                violations.push(M5RollbackDowngradeViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // Rollback path ref may be empty only when the path is missing.
        if row.rollback_path_state != RollbackPathState::Missing
            && row.rollback_path_ref.trim().is_empty()
        {
            violations.push(M5RollbackDowngradeViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "rollback_path_ref",
            });
        }

        // The ceiling: no lane may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5RollbackDowngradeViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5RollbackDowngradeViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5RollbackDowngradeViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A row publishing stable must have a tested or exercised rollback path.
        if row.publishes_stable() && row.rollback_path_state.forces_narrowing() {
            violations.push(M5RollbackDowngradeViolation::HeldWithUntestedRollbackPath {
                entry_id: row.entry_id.clone(),
            });
        }

        // A public claim whose canonical label is below the cutline forces the lane
        // to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(M5RollbackDowngradeViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5RollbackDowngradeViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    rollback_path_state: row.rollback_path_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed row carries exactly the public claim's canonical label,
            // carries no active gap reason, rides a captured within-SLO packet,
            // and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(M5RollbackDowngradeViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(M5RollbackDowngradeViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(M5RollbackDowngradeViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(M5RollbackDowngradeViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(M5RollbackDowngradeViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            // A held lane must have at least one downgrade rule and one claim-narrowing rule.
            if row.downgrade_rules.is_empty() {
                violations.push(M5RollbackDowngradeViolation::HeldWithoutDowngradeRules {
                    entry_id: row.entry_id.clone(),
                });
            }
            if row.claim_narrowing_rules.is_empty() {
                violations.push(
                    M5RollbackDowngradeViolation::HeldWithoutClaimNarrowingRules {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(M5RollbackDowngradeViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    rollback_path_state: row.rollback_path_state,
                    published: row.published_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5RollbackDowngradeViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    rollback_path_state: row.rollback_path_state,
                });
            }
            // A narrowing row whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(M5RollbackGapReason::ProofPacketStale)
            {
                violations.push(M5RollbackDowngradeViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(M5RollbackGapReason::ProofPacketMissing)
            {
                violations.push(M5RollbackDowngradeViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);

        // Promotion stage check: a held row must not have blocked stages.
        if row.holds_label() {
            for stage in &row.promotion_stages {
                if stage.stage_state == StageState::Blocked {
                    violations.push(M5RollbackDowngradeViolation::HeldWithBlockedStage {
                        entry_id: row.entry_id.clone(),
                        stage_kind: stage.stage_kind,
                    });
                }
            }
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &M5RollbackDowngradeRow,
        violations: &mut Vec<M5RollbackDowngradeViolation>,
    ) {
        // Only states that force narrowing require an active gap reason.
        if !row.rollback_path_state.forces_narrowing() {
            return;
        }

        let push_incoherent = |violations: &mut Vec<M5RollbackDowngradeViolation>,
                               expected: M5RollbackGapReason| {
            violations.push(M5RollbackDowngradeViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                rollback_path_state: row.rollback_path_state,
                expected_reason: expected,
            });
        };

        match row.rollback_path_state {
            RollbackPathState::Missing => {
                if !row.has_active_reason(M5RollbackGapReason::RollbackPathMissing) {
                    push_incoherent(violations, M5RollbackGapReason::RollbackPathMissing);
                }
            }
            RollbackPathState::Defined => {
                if !row.has_active_reason(M5RollbackGapReason::RollbackPathUntested) {
                    push_incoherent(violations, M5RollbackGapReason::RollbackPathUntested);
                }
            }
            RollbackPathState::Tested | RollbackPathState::Exercised => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5RollbackDowngradeViolation>) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.surface_ref.clone())
            .collect();
        for declared in &self.release_blocking_lane_refs {
            if !covered.contains(declared) {
                violations.push(
                    M5RollbackDowngradeViolation::ReleaseBlockingSurfaceUncovered {
                        surface_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_lane_refs.contains(&row.surface_ref) {
                violations.push(
                    M5RollbackDowngradeViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<M5RollbackDowngradeViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(M5RollbackDowngradeViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(M5RollbackDowngradeViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(
                M5RollbackDowngradeViolation::PromotionDecisionInconsistent {
                    declared: self.promotion.decision,
                    computed,
                },
            );
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(M5RollbackDowngradeViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(M5RollbackDowngradeViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the M5 rollback/downgrade register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RollbackDowngradeViolation {
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
    /// A gap reason has no stop rule watching for it.
    GapReasonWithoutStopRule {
        /// Uncovered reason.
        reason: M5RollbackGapReason,
    },
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A row holds a label while the public claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing state carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// Rollback path state.
        rollback_path_state: RollbackPathState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Rollback path state.
        rollback_path_state: RollbackPathState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held row carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held row has active gap reasons.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A held row has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A held row rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held row has an untested or missing rollback path.
    HeldWithUntestedRollbackPath {
        /// Row id.
        entry_id: String,
    },
    /// A held row lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A held row has no downgrade rules.
    HeldWithoutDowngradeRules {
        /// Row id.
        entry_id: String,
    },
    /// A held row has no claim-narrowing rules.
    HeldWithoutClaimNarrowingRules {
        /// Row id.
        entry_id: String,
    },
    /// A held row has a blocked promotion stage.
    HeldWithBlockedStage {
        /// Row id.
        entry_id: String,
        /// Stage kind.
        stage_kind: PromotionStageKind,
    },
    /// A narrowing row with a breached packet does not name the stale reason.
    BreachedPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing row with a missing packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A rollback path state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Rollback path state.
        rollback_path_state: RollbackPathState,
        /// Reason the state requires.
        expected_reason: M5RollbackGapReason,
    },
    /// A release-blocking surface ref has no covering row.
    ReleaseBlockingSurfaceUncovered {
        /// Surface ref.
        surface_ref: String,
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

impl fmt::Display for M5RollbackDowngradeViolation {
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
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id {entry_id}")
            }
            Self::DuplicateStopRuleId { rule_id } => {
                write!(f, "duplicate stop rule id {rule_id}")
            }
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutStopRule { reason } => write!(
                f,
                "gap reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "row {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason {
                entry_id,
                rollback_path_state,
            } => write!(
                f,
                "row {entry_id} rollback path state {rollback_path_state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                rollback_path_state,
                published,
            } => write!(
                f,
                "row {entry_id} rollback path state {rollback_path_state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "row {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "row {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket { entry_id, slo_state } => {
                write!(
                    f,
                    "row {entry_id} holds stable on stale packet {slo_state:?}"
                )
            }
            Self::HeldWithUntestedRollbackPath { entry_id } => {
                write!(f, "row {entry_id} holds stable with untested rollback path")
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} holds stable without owner signoff")
            }
            Self::HeldWithoutDowngradeRules { entry_id } => {
                write!(f, "row {entry_id} holds stable without downgrade rules")
            }
            Self::HeldWithoutClaimNarrowingRules { entry_id } => {
                write!(
                    f,
                    "row {entry_id} holds stable without claim-narrowing rules"
                )
            }
            Self::HeldWithBlockedStage {
                entry_id,
                stage_kind,
            } => {
                write!(
                    f,
                    "row {entry_id} holds stable with blocked stage {stage_kind:?}"
                )
            }
            Self::BreachedPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} breached packet without proof_packet_stale reason"
                )
            }
            Self::MissingPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} missing packet without proof_packet_missing reason"
                )
            }
            Self::StateReasonIncoherent {
                entry_id,
                rollback_path_state,
                expected_reason,
            } => write!(
                f,
                "row {entry_id} rollback path state {rollback_path_state:?} requires reason {expected_reason:?}"
            ),
            Self::ReleaseBlockingSurfaceUncovered { surface_ref } => {
                write!(
                    f,
                    "release-blocking surface {surface_ref} has no covering row"
                )
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => {
                write!(
                    f,
                    "release-blocking row {entry_id} is not declared in release_blocking_lane_refs"
                )
            }
            Self::PromotionDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "promotion {declared:?} disagrees with computed {computed:?}"
                )
            }
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

impl Error for M5RollbackDowngradeViolation {}

/// Loads the embedded M5 rollback/downgrade register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`M5RollbackDowngradeRegister`].
pub fn current_m5_rollback_downgrade_register(
) -> Result<M5RollbackDowngradeRegister, serde_json::Error> {
    serde_json::from_str(
        FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_JSON,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn register() -> M5RollbackDowngradeRegister {
        current_m5_rollback_downgrade_register().expect("register parses")
    }

    #[test]
    fn embedded_register_parses_and_validates() {
        let r = register();
        assert_eq!(
            r.schema_version,
            FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_SCHEMA_VERSION
        );
        assert_eq!(
            r.record_kind,
            FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_RECORD_KIND
        );
        assert_eq!(r.validate(), Vec::new());
        assert!(!r.rows.is_empty());
    }

    #[test]
    fn covers_every_lane_kind() {
        let r = register();
        for kind in M5LaneKind::ALL {
            assert!(
                !r.rows_for_kind(kind).is_empty(),
                "lane kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn covers_every_declared_release_blocking_surface() {
        let r = register();
        assert!(!r.release_blocking_lane_refs.is_empty());
        let covered: Vec<&str> = r
            .release_blocking_rows()
            .iter()
            .map(|row| row.surface_ref.as_str())
            .collect();
        for declared in &r.release_blocking_lane_refs {
            assert!(
                covered.contains(&declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let r = register();
        assert_eq!(r.summary, r.computed_summary());
        assert_eq!(
            r.summary.entries_holding_stable + r.summary.entries_narrowed,
            r.rows.len()
        );
    }

    #[test]
    fn promotion_decision_matches_computed() {
        let r = register();
        assert_eq!(r.promotion.decision, r.computed_promotion_decision());
        assert_eq!(
            r.promotion.blocking_rule_ids,
            r.computed_blocking_rule_ids()
        );
        assert_eq!(
            r.promotion.blocking_claim_ids,
            r.computed_blocking_entry_ids()
        );
    }

    #[test]
    fn every_gap_reason_has_a_stop_rule() {
        let r = register();
        let covered: BTreeSet<M5RollbackGapReason> = r
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in M5RollbackGapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn validate_flags_a_held_row_with_active_gap() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.active_gap_reasons
            .push(M5RollbackGapReason::ProofPacketMissing);
        r.summary = r.computed_summary();
        assert!(r
            .validate()
            .iter()
            .any(|v| matches!(v, M5RollbackDowngradeViolation::HeldWithActiveGap { .. })));
    }

    #[test]
    fn validate_flags_a_narrowing_row_that_does_not_narrow() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.rollback_path_state = RollbackPathState::Missing;
        row.active_gap_reasons
            .push(M5RollbackGapReason::RollbackPathMissing);
        row.published_label = StableClaimLevel::Stable;
        r.summary = r.computed_summary();
        r.promotion.decision = r.computed_promotion_decision();
        r.promotion.blocking_rule_ids = r.computed_blocking_rule_ids();
        r.promotion.blocking_claim_ids = r.computed_blocking_entry_ids();
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5RollbackDowngradeViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_promotion_decision() {
        let mut r = register();
        // Force a blocking rule to fire so the computed decision is Hold.
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.active_gap_reasons
            .push(M5RollbackGapReason::ProofPacketMissing);
        r.promotion.decision = PromotionDecision::Proceed;
        r.promotion.blocking_rule_ids = r.computed_blocking_rule_ids();
        r.promotion.blocking_claim_ids = r.computed_blocking_entry_ids();
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5RollbackDowngradeViolation::PromotionDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_claim_without_signoff() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        r.summary = r.computed_summary();
        assert!(r
            .validate()
            .iter()
            .any(|v| matches!(v, M5RollbackDowngradeViolation::HeldWithoutSignoff { .. })));
    }

    #[test]
    fn validate_flags_a_held_row_with_untested_rollback_path() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.rollback_path_state = RollbackPathState::Defined;
        row.active_gap_reasons
            .push(M5RollbackGapReason::RollbackPathUntested);
        // published_label stays Stable so the row is still publishing stable
        // with an untested (defined-only) rollback path.
        r.summary = r.computed_summary();
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5RollbackDowngradeViolation::HeldWithUntestedRollbackPath { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_row_without_downgrade_rules() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.downgrade_rules.clear();
        r.summary = r.computed_summary();
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5RollbackDowngradeViolation::HeldWithoutDowngradeRules { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_row_without_claim_narrowing_rules() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.claim_narrowing_rules.clear();
        r.summary = r.computed_summary();
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5RollbackDowngradeViolation::HeldWithoutClaimNarrowingRules { .. }
        )));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let r = register();
        let projection = r.support_export_projection();
        assert_eq!(projection.rows.len(), r.rows.len());
        for (row, proj) in r.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, proj.entry_id);
            assert_eq!(row.publishes_stable(), proj.publishes_stable);
        }
    }
}
