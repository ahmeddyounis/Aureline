//! Typed M5 proof-freshness, backport, and evidence-expiry automation for depth trains.
//!
//! This module freezes the canonical automation register that governs how every M5
//! depth lane maintains proof freshness, backport eligibility, and evidence expiry.
//! Each [`M5DepthTrainRow`] binds one M5 feature family to:
//!
//! - the stable claim it backs ([`M5DepthTrainRow::claim_ref`],
//!   [`M5DepthTrainRow::claim_label`]),
//! - a [`BackportEligibility`] record that tracks backport window, kind, and policy
//!   posture,
//! - an [`EvidenceExpiryRecord`] that tracks evidence kind, expiry date, and
//!   refresh state,
//! - the automation state earned ([`AutomationState`]), the active gap reasons
//!   ([`AutomationGapReason`]), and the effective label after narrowing
//!   ([`M5DepthTrainRow::published_label`]),
//! - upstream and downstream lane refs inherited from the feature-train matrix.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between a lane that may ship as Stable and one that must narrow
//! below it. The [`AutomationStopRule`] set names the closed conditions that gate
//! M5 promotion, and [`M5DepthTrainAutomationRegister::promotion`] records the
//! proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/m5/generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains.json`
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
pub const GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_RECORD_KIND: &str =
    "generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains";

/// Repo-relative path to the checked-in register.
pub const GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_PATH: &str =
    "artifacts/release/m5/generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains.json";

/// Embedded checked-in register JSON.
pub const GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_JSON: &str =
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5/generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains.json"
    ));

/// Automation state a depth-train lane earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationState {
    /// All automation items are present and current.
    Complete,
    /// One or more required automation items are missing.
    Incomplete,
    /// A proof packet or evidence item has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers
    /// a recorded gap.
    OnWaiver,
    /// Blocked by a missing backport policy or expired evidence.
    Blocked,
    /// Missing a defined evidence-expiry or backport rule.
    RuleMissing,
}

impl AutomationState {
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

/// Closed reason an automation lane narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationGapReason {
    /// Proof packet is missing.
    ProofPacketMissing,
    /// Proof packet is stale.
    ProofPacketStale,
    /// Evidence item has expired.
    EvidenceExpired,
    /// Evidence item is missing.
    EvidenceMissing,
    /// Backport window has closed.
    BackportWindowClosed,
    /// Backport policy is missing.
    BackportPolicyMissing,
    /// Compatibility report is stale.
    CompatibilityReportStale,
    /// Rollback path evidence is stale.
    RollbackPathStale,
    /// A waiver the lane relied on has expired.
    WaiverExpired,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl AutomationGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::EvidenceExpired,
        Self::EvidenceMissing,
        Self::BackportWindowClosed,
        Self::BackportPolicyMissing,
        Self::CompatibilityReportStale,
        Self::RollbackPathStale,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::EvidenceExpired => "evidence_expired",
            Self::EvidenceMissing => "evidence_missing",
            Self::BackportWindowClosed => "backport_window_closed",
            Self::BackportPolicyMissing => "backport_policy_missing",
            Self::CompatibilityReportStale => "compatibility_report_stale",
            Self::RollbackPathStale => "rollback_path_stale",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationAction {
    /// Hold promotion until the condition clears.
    HoldPromotion,
    /// Narrow the public claim below the cutline.
    NarrowLabel,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Refresh the expired evidence.
    RefreshEvidence,
    /// Extend the backport window.
    ExtendBackportWindow,
    /// Define the backport policy.
    DefineBackportPolicy,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl AutomationAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::HoldPromotion,
        Self::NarrowLabel,
        Self::RefreshProofPacket,
        Self::RefreshEvidence,
        Self::ExtendBackportWindow,
        Self::DefineBackportPolicy,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowLabel => "narrow_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RefreshEvidence => "refresh_evidence",
            Self::ExtendBackportWindow => "extend_backport_window",
            Self::DefineBackportPolicy => "define_backport_policy",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Kind of backport a lane supports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackportKind {
    /// Security-only backport.
    SecurityOnly,
    /// Critical fix backport.
    CriticalFix,
    /// Feature backport.
    FeatureBackport,
    /// Policy-exempt backport.
    PolicyExempt,
}

impl BackportKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SecurityOnly,
        Self::CriticalFix,
        Self::FeatureBackport,
        Self::PolicyExempt,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SecurityOnly => "security_only",
            Self::CriticalFix => "critical_fix",
            Self::FeatureBackport => "feature_backport",
            Self::PolicyExempt => "policy_exempt",
        }
    }
}

/// Kind of evidence whose expiry is tracked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    /// Proof packet evidence.
    ProofPacket,
    /// Compatibility report evidence.
    CompatibilityReport,
    /// Admin/policy story evidence.
    AdminPolicy,
    /// Rollback path evidence.
    RollbackPath,
    /// Benchmark result evidence.
    BenchmarkResult,
}

impl EvidenceKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProofPacket,
        Self::CompatibilityReport,
        Self::AdminPolicy,
        Self::RollbackPath,
        Self::BenchmarkResult,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofPacket => "proof_packet",
            Self::CompatibilityReport => "compatibility_report",
            Self::AdminPolicy => "admin_policy",
            Self::RollbackPath => "rollback_path",
            Self::BenchmarkResult => "benchmark_result",
        }
    }
}

/// Backport eligibility record for a depth-train lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackportEligibility {
    /// Whether the lane is currently eligible for backport.
    pub eligible: bool,
    /// The backport kind supported.
    pub backport_kind: BackportKind,
    /// UTC date the backport window closes, or null.
    #[serde(default)]
    pub window_closes_at: Option<String>,
    /// Ref to the backport policy artifact.
    pub policy_ref: String,
    /// Reviewable rationale for the backport posture.
    pub rationale: String,
}

impl BackportEligibility {
    /// True when the policy ref is non-empty.
    pub fn has_policy(&self) -> bool {
        !self.policy_ref.trim().is_empty()
    }
}

/// Evidence expiry record for a depth-train lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceExpiryRecord {
    /// The evidence kind tracked.
    pub evidence_kind: EvidenceKind,
    /// Ref to the evidence artifact.
    pub evidence_ref: String,
    /// UTC date the evidence was captured.
    pub captured_at: String,
    /// UTC date the evidence expires.
    pub expires_at: String,
    /// Days remaining before expiry.
    pub days_remaining: i32,
    /// Whether the evidence has expired.
    pub expired: bool,
    /// Reviewable rationale for the expiry posture.
    pub rationale: String,
}

/// One automation stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutomationStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: AutomationGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: AutomationAction,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One M5 depth-train automation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5DepthTrainRow {
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
    /// Automation state earned for the row.
    pub automation_state: AutomationState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Backport eligibility record.
    pub backport_eligibility: BackportEligibility,
    /// Evidence expiry records.
    #[serde(default)]
    pub evidence_expiry_records: Vec<EvidenceExpiryRecord>,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<AutomationGapReason>,
    /// The lifecycle label the lane effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Upstream lane refs.
    #[serde(default)]
    pub upstream_lane_refs: Vec<String>,
    /// Downstream lane refs.
    #[serde(default)]
    pub downstream_lane_refs: Vec<String>,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl M5DepthTrainRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the lane carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.automation_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: AutomationGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// True when at least one evidence record is expired.
    pub fn has_expired_evidence(&self) -> bool {
        self.evidence_expiry_records.iter().any(|e| e.expired)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5DepthTrainAutomationSummary {
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
    /// Rows blocked by expired evidence or missing backport policy.
    pub entries_blocked: usize,
    /// Rows missing an automation rule.
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
    /// Rows with expired evidence.
    pub entries_expired_evidence: usize,
    /// Rows with a missing backport policy.
    pub entries_missing_backport_policy: usize,
    /// Rows with a closed backport window.
    pub entries_backport_window_closed: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DepthTrainAutomationExportRow {
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
    /// Automation state earned.
    pub automation_state: AutomationState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<AutomationGapReason>,
    /// Whether the row has expired evidence.
    pub has_expired_evidence: bool,
    /// Backport kind.
    pub backport_kind: BackportKind,
    /// Backport eligible.
    pub backport_eligible: bool,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DepthTrainAutomationExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5DepthTrainAutomationExportRow>,
}

/// The typed M5 depth-train automation register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5DepthTrainAutomationRegister {
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
    /// Closed automation-state vocabulary.
    pub automation_states: Vec<AutomationState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<AutomationGapReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<AutomationAction>,
    /// Closed backport-kind vocabulary.
    pub backport_kinds: Vec<BackportKind>,
    /// Closed evidence-kind vocabulary.
    pub evidence_kinds: Vec<EvidenceKind>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_lane_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<AutomationStopRule>,
    /// Lane rows.
    pub rows: Vec<M5DepthTrainRow>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5DepthTrainAutomationSummary,
}

impl M5DepthTrainAutomationRegister {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&M5DepthTrainRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&M5DepthTrainRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&M5DepthTrainRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&M5DepthTrainRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one lane kind.
    pub fn rows_for_kind(&self, kind: M5LaneKind) -> Vec<&M5DepthTrainRow> {
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
    pub fn stop_rule_fires(&self, rule: &AutomationStopRule) -> bool {
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
        let blocking_triggers: BTreeSet<AutomationGapReason> = self
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
    pub fn computed_summary(&self) -> M5DepthTrainAutomationSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: M5LaneKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&M5DepthTrainRow> = self.release_blocking_rows();
        M5DepthTrainAutomationSummary {
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
                .filter(|row| row.automation_state == AutomationState::OnWaiver)
                .count(),
            entries_blocked: self
                .rows
                .iter()
                .filter(|row| row.automation_state == AutomationState::Blocked)
                .count(),
            entries_rule_missing: self
                .rows
                .iter()
                .filter(|row| row.automation_state == AutomationState::RuleMissing)
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
            entries_expired_evidence: self
                .rows
                .iter()
                .filter(|row| row.has_expired_evidence())
                .count(),
            entries_missing_backport_policy: self
                .rows
                .iter()
                .filter(|row| row.has_active_reason(AutomationGapReason::BackportPolicyMissing))
                .count(),
            entries_backport_window_closed: self
                .rows
                .iter()
                .filter(|row| row.has_active_reason(AutomationGapReason::BackportWindowClosed))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the register that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> M5DepthTrainAutomationExportProjection {
        M5DepthTrainAutomationExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5DepthTrainAutomationExportRow {
                    entry_id: row.entry_id.clone(),
                    lane_kind: row.lane_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    automation_state: row.automation_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                    has_expired_evidence: row.has_expired_evidence(),
                    backport_kind: row.backport_eligibility.backport_kind,
                    backport_eligible: row.backport_eligibility.eligible,
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<M5DepthTrainAutomationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5DepthTrainAutomationViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5DepthTrainAutomationViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5DepthTrainAutomationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5DepthTrainAutomationViolation>) {
        if self.schema_version
            != GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_SCHEMA_VERSION
        {
            violations.push(M5DepthTrainAutomationViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind
            != GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_RECORD_KIND
        {
            violations.push(M5DepthTrainAutomationViolation::UnsupportedRecordKind {
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
                violations.push(M5DepthTrainAutomationViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5DepthTrainAutomationViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.lane_kinds != M5LaneKind::ALL.to_vec() {
            violations.push(M5DepthTrainAutomationViolation::ClosedVocabularyMismatch {
                field: "lane_kinds",
            });
        }
        if self.automation_states != AutomationState::ALL.to_vec() {
            violations.push(M5DepthTrainAutomationViolation::ClosedVocabularyMismatch {
                field: "automation_states",
            });
        }
        if self.gap_reasons != AutomationGapReason::ALL.to_vec() {
            violations.push(M5DepthTrainAutomationViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.stop_rule_actions != AutomationAction::ALL.to_vec() {
            violations.push(M5DepthTrainAutomationViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }
        if self.backport_kinds != BackportKind::ALL.to_vec() {
            violations.push(M5DepthTrainAutomationViolation::ClosedVocabularyMismatch {
                field: "backport_kinds",
            });
        }
        if self.evidence_kinds != EvidenceKind::ALL.to_vec() {
            violations.push(M5DepthTrainAutomationViolation::ClosedVocabularyMismatch {
                field: "evidence_kinds",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5DepthTrainAutomationViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5DepthTrainAutomationViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5DepthTrainAutomationViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5DepthTrainAutomationViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5DepthTrainAutomationViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5DepthTrainAutomationViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5DepthTrainAutomationViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5DepthTrainAutomationViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5DepthTrainAutomationViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in AutomationGapReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(M5DepthTrainAutomationViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &M5DepthTrainRow,
        violations: &mut Vec<M5DepthTrainAutomationViolation>,
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
                "backport_eligibility.policy_ref",
                &row.backport_eligibility.policy_ref,
            ),
            (
                "backport_eligibility.rationale",
                &row.backport_eligibility.rationale,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(M5DepthTrainAutomationViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no lane may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5DepthTrainAutomationViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5DepthTrainAutomationViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5DepthTrainAutomationViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A held lane must have a non-empty backport policy ref.
        if row.holds_label() && !row.backport_eligibility.has_policy() {
            violations.push(
                M5DepthTrainAutomationViolation::HeldWithMissingBackportPolicy {
                    entry_id: row.entry_id.clone(),
                },
            );
        }

        // A public claim whose canonical label is below the cutline forces the lane
        // to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(M5DepthTrainAutomationViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5DepthTrainAutomationViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.automation_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed row carries exactly the public claim's canonical label,
            // carries no active gap reason, rides a captured within-SLO packet,
            // and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(M5DepthTrainAutomationViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(M5DepthTrainAutomationViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(M5DepthTrainAutomationViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(M5DepthTrainAutomationViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(M5DepthTrainAutomationViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            // A held row must not have expired evidence.
            if row.has_expired_evidence() {
                violations.push(M5DepthTrainAutomationViolation::HeldWithExpiredEvidence {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(M5DepthTrainAutomationViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.automation_state,
                    published: row.published_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5DepthTrainAutomationViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.automation_state,
                });
            }
            // A narrowing row whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(AutomationGapReason::ProofPacketStale)
            {
                violations.push(
                    M5DepthTrainAutomationViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(AutomationGapReason::ProofPacketMissing)
            {
                violations.push(
                    M5DepthTrainAutomationViolation::MissingPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_state_reason_coherence(
        &self,
        row: &M5DepthTrainRow,
        violations: &mut Vec<M5DepthTrainAutomationViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<M5DepthTrainAutomationViolation>,
                               expected: AutomationGapReason| {
            violations.push(M5DepthTrainAutomationViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.automation_state,
                expected_reason: expected,
            });
        };

        match row.automation_state {
            AutomationState::Incomplete => {
                if !row.has_active_reason(AutomationGapReason::ProofPacketMissing)
                    && !row.has_active_reason(AutomationGapReason::EvidenceMissing)
                    && !row.has_active_reason(AutomationGapReason::BackportPolicyMissing)
                {
                    push_incoherent(violations, AutomationGapReason::ProofPacketMissing);
                }
            }
            AutomationState::Stale => {
                if !(row.has_active_reason(AutomationGapReason::ProofPacketStale)
                    || row.has_active_reason(AutomationGapReason::CompatibilityReportStale)
                    || row.has_active_reason(AutomationGapReason::RollbackPathStale)
                    || row.has_active_reason(AutomationGapReason::EvidenceExpired))
                {
                    push_incoherent(violations, AutomationGapReason::ProofPacketStale);
                }
            }
            AutomationState::Blocked => {
                if !row.has_active_reason(AutomationGapReason::BackportPolicyMissing)
                    && !row.has_active_reason(AutomationGapReason::BackportWindowClosed)
                    && !row.has_active_reason(AutomationGapReason::EvidenceExpired)
                {
                    push_incoherent(violations, AutomationGapReason::BackportPolicyMissing);
                }
            }
            AutomationState::RuleMissing => {
                if !row.has_active_reason(AutomationGapReason::BackportPolicyMissing)
                    && !row.has_active_reason(AutomationGapReason::EvidenceMissing)
                {
                    push_incoherent(violations, AutomationGapReason::BackportPolicyMissing);
                }
            }
            AutomationState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(M5DepthTrainAutomationViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.automation_state,
                    });
                }
            }
            AutomationState::Complete => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5DepthTrainAutomationViolation>) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.surface_ref.clone())
            .collect();
        for declared in &self.release_blocking_lane_refs {
            if !covered.contains(declared) {
                violations.push(
                    M5DepthTrainAutomationViolation::ReleaseBlockingSurfaceUncovered {
                        surface_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_lane_refs.contains(&row.surface_ref) {
                violations.push(
                    M5DepthTrainAutomationViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<M5DepthTrainAutomationViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(M5DepthTrainAutomationViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(M5DepthTrainAutomationViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(
                M5DepthTrainAutomationViolation::PromotionDecisionInconsistent {
                    declared: self.promotion.decision,
                    computed,
                },
            );
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                M5DepthTrainAutomationViolation::PromotionBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(
                M5DepthTrainAutomationViolation::PromotionBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// A validation violation for the M5 depth-train automation register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DepthTrainAutomationViolation {
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
        reason: AutomationGapReason,
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
        /// Automation state.
        state: AutomationState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Automation state.
        state: AutomationState,
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
    /// A held row lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A held row has expired evidence.
    HeldWithExpiredEvidence {
        /// Row id.
        entry_id: String,
    },
    /// A held row has a missing backport policy.
    HeldWithMissingBackportPolicy {
        /// Row id.
        entry_id: String,
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
    /// A state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Automation state.
        state: AutomationState,
        /// Reason the state requires.
        expected_reason: AutomationGapReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// Automation state.
        state: AutomationState,
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

impl fmt::Display for M5DepthTrainAutomationViolation {
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
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "row {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "row {entry_id} state {state:?} must narrow but publishes {published:?}"
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
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "row {entry_id} holds stable on stale packet {slo_state:?}"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} holds stable without owner signoff")
            }
            Self::HeldWithExpiredEvidence { entry_id } => {
                write!(f, "row {entry_id} holds stable with expired evidence")
            }
            Self::HeldWithMissingBackportPolicy { entry_id } => {
                write!(
                    f,
                    "row {entry_id} holds stable with missing backport policy"
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
                state,
                expected_reason,
            } => write!(
                f,
                "row {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "row {entry_id} state {state:?} names no waiver")
            }
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

impl Error for M5DepthTrainAutomationViolation {}

/// Loads the embedded M5 depth-train automation register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`M5DepthTrainAutomationRegister`].
pub fn current_m5_depth_train_automation_register(
) -> Result<M5DepthTrainAutomationRegister, serde_json::Error> {
    serde_json::from_str(
        GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_JSON,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn register() -> M5DepthTrainAutomationRegister {
        current_m5_depth_train_automation_register().expect("register parses")
    }

    #[test]
    fn embedded_register_parses_and_validates() {
        let r = register();
        assert_eq!(
            r.schema_version,
            GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_SCHEMA_VERSION
        );
        assert_eq!(
            r.record_kind,
            GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_RECORD_KIND
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
            .into_iter()
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
        let covered: BTreeSet<AutomationGapReason> = r
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in AutomationGapReason::ALL {
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
            .push(AutomationGapReason::ProofPacketMissing);
        r.summary = r.computed_summary();
        assert!(r
            .validate()
            .iter()
            .any(|v| matches!(v, M5DepthTrainAutomationViolation::HeldWithActiveGap { .. })));
    }

    #[test]
    fn validate_flags_a_narrowing_row_that_does_not_narrow() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.automation_state = AutomationState::Incomplete;
        row.active_gap_reasons
            .push(AutomationGapReason::ProofPacketMissing);
        row.published_label = StableClaimLevel::Stable;
        r.summary = r.computed_summary();
        r.promotion.decision = r.computed_promotion_decision();
        r.promotion.blocking_rule_ids = r.computed_blocking_rule_ids();
        r.promotion.blocking_claim_ids = r.computed_blocking_entry_ids();
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5DepthTrainAutomationViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_promotion_decision() {
        let mut r = register();
        r.promotion.decision = PromotionDecision::Proceed;
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5DepthTrainAutomationViolation::PromotionDecisionInconsistent { .. }
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
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5DepthTrainAutomationViolation::HeldWithoutSignoff { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_row_with_expired_evidence() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.evidence_expiry_records.push(EvidenceExpiryRecord {
            evidence_kind: EvidenceKind::ProofPacket,
            evidence_ref: "ref".to_owned(),
            captured_at: "2026-01-01".to_owned(),
            expires_at: "2026-02-01".to_owned(),
            days_remaining: -10,
            expired: true,
            rationale: "Expired".to_owned(),
        });
        r.summary = r.computed_summary();
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5DepthTrainAutomationViolation::HeldWithExpiredEvidence { .. }
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
