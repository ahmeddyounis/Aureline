//! Typed M5 local-model, provider-graduation, and spend-governance control packet.
//!
//! This module publishes the canonical control packet that governs the
//! local-model, provider-graduation, and spend-governance depth lanes for
//! Milestone 5. Each [`M5ControlPacketLaneRow`] binds one lane to:
//!
//! - a [`ControlPacketStory`] composed of required story items
//!   ([`ControlPacketItem`]), each with its kind, state, and artifact ref,
//! - the register state earned ([`ControlPacketLaneState`]), the active gap reasons
//!   ([`ControlPacketGapReason`]), and the effective label after narrowing
//!   ([`M5ControlPacketLaneRow::published_label`]),
//! - a proof packet with freshness SLO ([`ProofPacket`]),
//! - owner sign-off ([`OwnerSignoff`]),
//! - stop rules that gate publication when story items are missing, stale, or
//!   unsigned.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between a lane that may publish as Stable and one that must narrow
//! below it. The [`ControlPacketStopRule`] set names the closed conditions that
//! gate publication, and [`M5ControlPacketRegister::publication`] records the
//! proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/m5/publish_the_m5_local_model_provider_graduation_and_spend_governance_control_packet.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
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
pub const PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_RECORD_KIND: &str =
    "publish_the_m5_local_model_provider_graduation_and_spend_governance_control_packet";

/// Repo-relative path to the checked-in register.
pub const PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_PATH: &str =
    "artifacts/release/m5/publish_the_m5_local_model_provider_graduation_and_spend_governance_control_packet.json";

/// Embedded checked-in register JSON.
pub const PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_JSON: &str =
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5/publish_the_m5_local_model_provider_graduation_and_spend_governance_control_packet.json"
    ));

/// M5 control-packet lane a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ControlPacketLaneKind {
    /// Local and on-device model execution surfaces.
    LocalModel,
    /// Provider graduation from preview to stable.
    ProviderGraduation,
    /// Spend governance and budget control surfaces.
    SpendGovernance,
}

impl M5ControlPacketLaneKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::LocalModel,
        Self::ProviderGraduation,
        Self::SpendGovernance,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalModel => "local_model",
            Self::ProviderGraduation => "provider_graduation",
            Self::SpendGovernance => "spend_governance",
        }
    }
}

/// Kind of item in a control-packet story.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlPacketItemKind {
    /// Local-model capability and on-device execution posture.
    LocalModelCapability,
    /// Provider graduation path and criteria.
    ProviderGraduationPath,
    /// Spend-governance policy and budget controls.
    SpendGovernancePolicy,
    /// Privacy and trust posture for the lane.
    PrivacyTrustPosture,
    /// Rollback and downgrade path definition.
    RollbackDowngradePath,
    /// Compatibility and interoperability requirements.
    CompatibilityInterop,
}

impl ControlPacketItemKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalModelCapability,
        Self::ProviderGraduationPath,
        Self::SpendGovernancePolicy,
        Self::PrivacyTrustPosture,
        Self::RollbackDowngradePath,
        Self::CompatibilityInterop,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalModelCapability => "local_model_capability",
            Self::ProviderGraduationPath => "provider_graduation_path",
            Self::SpendGovernancePolicy => "spend_governance_policy",
            Self::PrivacyTrustPosture => "privacy_trust_posture",
            Self::RollbackDowngradePath => "rollback_downgrade_path",
            Self::CompatibilityInterop => "compatibility_interop",
        }
    }
}

/// State of a control-packet story item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlPacketItemState {
    /// Item is planned but not yet drafted.
    Planned,
    /// Item is drafted but not yet reviewed.
    Drafted,
    /// Item has been reviewed.
    Reviewed,
    /// Item is published and current.
    Published,
    /// Item is published but has gone stale.
    Stale,
    /// Item is missing.
    Missing,
}

impl ControlPacketItemState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Planned,
        Self::Drafted,
        Self::Reviewed,
        Self::Published,
        Self::Stale,
        Self::Missing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Drafted => "drafted",
            Self::Reviewed => "reviewed",
            Self::Published => "published",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }

    /// Whether the state counts as complete for a story item.
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Reviewed | Self::Published)
    }
}

/// Register state a lane earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlPacketLaneState {
    /// All story items are complete and the owner is signed off.
    Complete,
    /// One or more required story items are missing or incomplete.
    Incomplete,
    /// A story item has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers
    /// a recorded gap.
    OnWaiver,
    /// Blocked by a missing owner or incomplete owner sign-off.
    OwnerBlocked,
}

impl ControlPacketLaneState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Complete,
        Self::Incomplete,
        Self::Stale,
        Self::OnWaiver,
        Self::OwnerBlocked,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::OwnerBlocked => "owner_blocked",
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

/// Closed reason a control-packet lane narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlPacketGapReason {
    /// Control-packet story is incomplete.
    ControlPacketIncomplete,
    /// Control-packet story is stale.
    ControlPacketStale,
    /// Proof packet is missing.
    ProofPacketMissing,
    /// Proof packet is stale.
    ProofPacketStale,
    /// Owner sign-off is missing.
    OwnerSignoffMissing,
    /// A required waiver has expired.
    WaiverExpired,
}

impl ControlPacketGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ControlPacketIncomplete,
        Self::ControlPacketStale,
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlPacketIncomplete => "control_packet_incomplete",
            Self::ControlPacketStale => "control_packet_stale",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlPacketAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the public claim below the cutline.
    NarrowLabel,
    /// Staff the control-packet story.
    StaffControlPacket,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl ControlPacketAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::StaffControlPacket,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::StaffControlPacket => "staff_control_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One item in a control-packet story.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlPacketItem {
    /// Stable item id.
    pub item_id: String,
    /// Human-readable title.
    pub title: String,
    /// The kind of control-packet story item.
    pub item_kind: ControlPacketItemKind,
    /// Ref to the artifact that fulfills this item.
    pub artifact_ref: String,
    /// Current state of the item.
    pub item_state: ControlPacketItemState,
    /// The owner responsible for this item.
    pub owner_ref: String,
    /// Reviewable reason this item carries this state.
    pub rationale: String,
}

/// The control-packet story for a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlPacketStory {
    /// Stable story id.
    pub story_id: String,
    /// Story version.
    pub story_version: u32,
    /// Story items.
    pub items: Vec<ControlPacketItem>,
}

impl ControlPacketStory {
    /// True when every required [`ControlPacketItemKind`] is present and complete.
    pub fn is_complete(&self) -> bool {
        let present: BTreeSet<ControlPacketItemKind> =
            self.items.iter().map(|e| e.item_kind).collect();
        if present.len() != ControlPacketItemKind::ALL.len() {
            return false;
        }
        self.items.iter().all(|e| e.item_state.is_complete())
    }

    /// True when every required item kind is present.
    pub fn has_all_required_items(&self) -> bool {
        let present: BTreeSet<ControlPacketItemKind> =
            self.items.iter().map(|e| e.item_kind).collect();
        present.len() == ControlPacketItemKind::ALL.len()
    }

    /// Returns items whose state is stale.
    pub fn stale_items(&self) -> Vec<&ControlPacketItem> {
        self.items
            .iter()
            .filter(|e| e.item_state == ControlPacketItemState::Stale)
            .collect()
    }

    /// Returns items whose state is missing.
    pub fn missing_items(&self) -> Vec<&ControlPacketItem> {
        self.items
            .iter()
            .filter(|e| e.item_state == ControlPacketItemState::Missing)
            .collect()
    }
}

/// One control-packet stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlPacketStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: ControlPacketGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: ControlPacketAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One M5 control-packet lane row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ControlPacketLaneRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The lane kind this row governs.
    pub lane_kind: M5ControlPacketLaneKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the row.
    pub surface_summary: String,
    /// Whether the lane is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// The control-packet story for this lane.
    pub control_packet_story: ControlPacketStory,
    /// Register state earned for the row.
    pub lane_state: ControlPacketLaneState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<ControlPacketGapReason>,
    /// The lifecycle label the lane effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl M5ControlPacketLaneRow {
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
        self.lane_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: ControlPacketGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ControlPacketRegisterSummary {
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
    /// Rows blocked by a missing owner sign-off.
    pub entries_owner_blocked: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_holding: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Local-model rows.
    pub local_model_entries: usize,
    /// Provider-graduation rows.
    pub provider_graduation_entries: usize,
    /// Spend-governance rows.
    pub spend_governance_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Story items whose state is `published`.
    pub story_items_published: usize,
    /// Story items whose state is `stale`.
    pub story_items_stale: usize,
    /// Story items whose state is `missing`.
    pub story_items_missing: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ControlPacketRegisterExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The lane kind this row governs.
    pub lane_kind: M5ControlPacketLaneKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Whether the lane is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Register state earned.
    pub lane_state: ControlPacketLaneState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<ControlPacketGapReason>,
    /// Owner ref.
    pub owner_ref: String,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ControlPacketRegisterExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5ControlPacketRegisterExportRow>,
}

/// The typed M5 control-packet register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ControlPacketRegister {
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
    /// Ref to the M5 feature-train matrix this register builds on.
    pub feature_train_matrix_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed lane-kind vocabulary.
    pub lane_kinds: Vec<M5ControlPacketLaneKind>,
    /// Closed story-item-kind vocabulary.
    pub story_item_kinds: Vec<ControlPacketItemKind>,
    /// Closed story-item-state vocabulary.
    pub story_item_states: Vec<ControlPacketItemState>,
    /// Closed lane-state vocabulary.
    pub lane_states: Vec<ControlPacketLaneState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<ControlPacketGapReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<ControlPacketAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_lane_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<ControlPacketStopRule>,
    /// Lane rows.
    pub rows: Vec<M5ControlPacketLaneRow>,
    /// Recorded publication verdict.
    pub publication: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5ControlPacketRegisterSummary,
}

impl M5ControlPacketRegister {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&M5ControlPacketLaneRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&M5ControlPacketLaneRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&M5ControlPacketLaneRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&M5ControlPacketLaneRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one lane kind.
    pub fn rows_for_kind(&self, kind: M5ControlPacketLaneKind) -> Vec<&M5ControlPacketLaneRow> {
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
    pub fn stop_rule_fires(&self, rule: &ControlPacketStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and stop rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Stop-rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Lane-row ids that trigger a blocking, firing rule, sorted and unique.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<ControlPacketGapReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
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
    pub fn computed_summary(&self) -> M5ControlPacketRegisterSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: M5ControlPacketLaneKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&M5ControlPacketLaneRow> = self.release_blocking_rows();
        let story_items = |state: ControlPacketItemState| {
            self.rows
                .iter()
                .flat_map(|row| &row.control_packet_story.items)
                .filter(|e| e.item_state == state)
                .count()
        };
        M5ControlPacketRegisterSummary {
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
                .filter(|row| row.lane_state == ControlPacketLaneState::OnWaiver)
                .count(),
            entries_owner_blocked: self
                .rows
                .iter()
                .filter(|row| row.lane_state == ControlPacketLaneState::OwnerBlocked)
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
            local_model_entries: kind(M5ControlPacketLaneKind::LocalModel),
            provider_graduation_entries: kind(M5ControlPacketLaneKind::ProviderGraduation),
            spend_governance_entries: kind(M5ControlPacketLaneKind::SpendGovernance),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            story_items_published: story_items(ControlPacketItemState::Published),
            story_items_stale: story_items(ControlPacketItemState::Stale),
            story_items_missing: story_items(ControlPacketItemState::Missing),
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
        }
    }

    /// Produces an export/Help-About-safe projection of the register that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> M5ControlPacketRegisterExportProjection {
        M5ControlPacketRegisterExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5ControlPacketRegisterExportRow {
                    entry_id: row.entry_id.clone(),
                    lane_kind: row.lane_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    lane_state: row.lane_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                    owner_ref: row.owner_signoff.owner_ref.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<M5ControlPacketRegisterViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5ControlPacketRegisterViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5ControlPacketRegisterViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5ControlPacketRegisterViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ControlPacketRegisterViolation>) {
        if self.schema_version
            != PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_SCHEMA_VERSION
        {
            violations.push(M5ControlPacketRegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind
            != PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_RECORD_KIND
        {
            violations.push(M5ControlPacketRegisterViolation::UnsupportedRecordKind {
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
                violations.push(M5ControlPacketRegisterViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5ControlPacketRegisterViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.lane_kinds != M5ControlPacketLaneKind::ALL.to_vec() {
            violations.push(M5ControlPacketRegisterViolation::ClosedVocabularyMismatch {
                field: "lane_kinds",
            });
        }
        if self.story_item_kinds != ControlPacketItemKind::ALL.to_vec() {
            violations.push(M5ControlPacketRegisterViolation::ClosedVocabularyMismatch {
                field: "story_item_kinds",
            });
        }
        if self.story_item_states != ControlPacketItemState::ALL.to_vec() {
            violations.push(M5ControlPacketRegisterViolation::ClosedVocabularyMismatch {
                field: "story_item_states",
            });
        }
        if self.lane_states != ControlPacketLaneState::ALL.to_vec() {
            violations.push(M5ControlPacketRegisterViolation::ClosedVocabularyMismatch {
                field: "lane_states",
            });
        }
        if self.gap_reasons != ControlPacketGapReason::ALL.to_vec() {
            violations.push(M5ControlPacketRegisterViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.stop_rule_actions != ControlPacketAction::ALL.to_vec() {
            violations.push(M5ControlPacketRegisterViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5ControlPacketRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5ControlPacketRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5ControlPacketRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5ControlPacketRegisterViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5ControlPacketRegisterViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5ControlPacketRegisterViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5ControlPacketRegisterViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ControlPacketRegisterViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5ControlPacketRegisterViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in ControlPacketGapReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(M5ControlPacketRegisterViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &M5ControlPacketLaneRow,
        violations: &mut Vec<M5ControlPacketRegisterViolation>,
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
                violations.push(M5ControlPacketRegisterViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no lane may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5ControlPacketRegisterViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5ControlPacketRegisterViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5ControlPacketRegisterViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A held lane must have a complete control-packet story.
        if row.holds_label() && !row.control_packet_story.is_complete() {
            violations.push(
                M5ControlPacketRegisterViolation::HeldWithIncompleteControlPacketStory {
                    entry_id: row.entry_id.clone(),
                },
            );
        }

        // A public claim whose canonical label is below the cutline forces the
        // lane to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(M5ControlPacketRegisterViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5ControlPacketRegisterViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.lane_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed row carries exactly the public claim's canonical label,
            // carries no active gap reason, rides a captured within-SLO packet,
            // and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(M5ControlPacketRegisterViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(M5ControlPacketRegisterViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(M5ControlPacketRegisterViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(M5ControlPacketRegisterViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(M5ControlPacketRegisterViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(
                    M5ControlPacketRegisterViolation::PublishedLabelNotNarrowed {
                        entry_id: row.entry_id.clone(),
                        state: row.lane_state,
                        published: row.published_label,
                    },
                );
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5ControlPacketRegisterViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.lane_state,
                });
            }
            // A narrowing row whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(ControlPacketGapReason::ProofPacketStale)
            {
                violations.push(
                    M5ControlPacketRegisterViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(ControlPacketGapReason::ProofPacketMissing)
            {
                violations.push(
                    M5ControlPacketRegisterViolation::MissingPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);

        // Control-packet-story completeness: every required item kind must be present.
        let present_kinds: BTreeSet<ControlPacketItemKind> = row
            .control_packet_story
            .items
            .iter()
            .map(|e| e.item_kind)
            .collect();
        for kind in ControlPacketItemKind::ALL {
            if !present_kinds.contains(&kind) {
                violations.push(
                    M5ControlPacketRegisterViolation::ControlPacketStoryMissingRequiredKind {
                        entry_id: row.entry_id.clone(),
                        item_kind: kind,
                    },
                );
            }
        }

        // Every story item must have non-empty required fields.
        for item in &row.control_packet_story.items {
            for (field, value) in [
                ("item_id", &item.item_id),
                ("title", &item.title),
                ("artifact_ref", &item.artifact_ref),
                ("owner_ref", &item.owner_ref),
                ("rationale", &item.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ControlPacketRegisterViolation::EmptyField {
                        entry_id: format!("{}:{}", row.entry_id, item.item_id),
                        field_name: field,
                    });
                }
            }
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &M5ControlPacketLaneRow,
        violations: &mut Vec<M5ControlPacketRegisterViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<M5ControlPacketRegisterViolation>,
                               expected: ControlPacketGapReason| {
            violations.push(M5ControlPacketRegisterViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.lane_state,
                expected_reason: expected,
            });
        };

        match row.lane_state {
            ControlPacketLaneState::Incomplete => {
                if !row.has_active_reason(ControlPacketGapReason::ControlPacketIncomplete)
                    && !row.has_active_reason(ControlPacketGapReason::ProofPacketMissing)
                {
                    push_incoherent(violations, ControlPacketGapReason::ControlPacketIncomplete);
                }
            }
            ControlPacketLaneState::Stale => {
                if !(row.has_active_reason(ControlPacketGapReason::ControlPacketStale)
                    || row.has_active_reason(ControlPacketGapReason::ProofPacketStale))
                {
                    push_incoherent(violations, ControlPacketGapReason::ControlPacketStale);
                }
            }
            ControlPacketLaneState::OwnerBlocked => {
                if !row.has_active_reason(ControlPacketGapReason::OwnerSignoffMissing) {
                    push_incoherent(violations, ControlPacketGapReason::OwnerSignoffMissing);
                }
            }
            ControlPacketLaneState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(M5ControlPacketRegisterViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.lane_state,
                    });
                }
            }
            ControlPacketLaneState::Complete => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5ControlPacketRegisterViolation>) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.surface_ref.clone())
            .collect();
        for declared in &self.release_blocking_lane_refs {
            if !covered.contains(declared) {
                violations.push(
                    M5ControlPacketRegisterViolation::ReleaseBlockingSurfaceUncovered {
                        surface_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_lane_refs.contains(&row.surface_ref) {
                violations.push(
                    M5ControlPacketRegisterViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<M5ControlPacketRegisterViolation>) {
        if self.publication.promotion_gate.trim().is_empty() {
            violations.push(M5ControlPacketRegisterViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(M5ControlPacketRegisterViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                M5ControlPacketRegisterViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                M5ControlPacketRegisterViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(
                M5ControlPacketRegisterViolation::PublicationBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// A validation violation for the M5 control-packet register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ControlPacketRegisterViolation {
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
        reason: ControlPacketGapReason,
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
        /// Lane state.
        state: ControlPacketLaneState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Lane state.
        state: ControlPacketLaneState,
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
    /// A held row has an incomplete control-packet story.
    HeldWithIncompleteControlPacketStory {
        /// Row id.
        entry_id: String,
    },
    /// A held row lacks owner sign-off.
    HeldWithoutSignoff {
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
    /// A lane state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Lane state.
        state: ControlPacketLaneState,
        /// Reason the state requires.
        expected_reason: ControlPacketGapReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// Lane state.
        state: ControlPacketLaneState,
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
    /// The declared publication decision disagrees with the computed one.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared publication blocking set disagrees with the computed one.
    PublicationBlockingSetMismatch {
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
    /// The control-packet story is missing a required item kind.
    ControlPacketStoryMissingRequiredKind {
        /// Row id.
        entry_id: String,
        /// Missing item kind.
        item_kind: ControlPacketItemKind,
    },
}

impl fmt::Display for M5ControlPacketRegisterViolation {
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
            Self::HeldWithIncompleteControlPacketStory { entry_id } => {
                write!(
                    f,
                    "row {entry_id} holds stable with incomplete control-packet story"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} holds stable without owner signoff")
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
            Self::PublicationDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "publication {declared:?} disagrees with computed {computed:?}"
                )
            }
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with rows"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "row {entry_id} freshness SLO window is inconsistent")
            }
            Self::ControlPacketStoryMissingRequiredKind {
                entry_id,
                item_kind,
            } => {
                write!(
                    f,
                    "row {entry_id} control-packet story missing required kind {item_kind:?}"
                )
            }
        }
    }
}

impl Error for M5ControlPacketRegisterViolation {}

/// Loads the embedded M5 control-packet register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`M5ControlPacketRegister`].
pub fn current_m5_control_packet_register() -> Result<M5ControlPacketRegister, serde_json::Error> {
    serde_json::from_str(
        PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_JSON,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn register() -> M5ControlPacketRegister {
        current_m5_control_packet_register().expect("register parses")
    }

    #[test]
    fn embedded_register_parses_and_validates() {
        let reg = register();
        assert_eq!(
            reg.schema_version,
            PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_SCHEMA_VERSION
        );
        assert_eq!(
            reg.record_kind,
            PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_RECORD_KIND
        );
        assert_eq!(reg.validate(), Vec::new());
        assert!(!reg.rows.is_empty());
    }

    #[test]
    fn covers_every_lane_kind() {
        let reg = register();
        for kind in M5ControlPacketLaneKind::ALL {
            assert!(
                !reg.rows_for_kind(kind).is_empty(),
                "lane kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn covers_every_declared_release_blocking_surface() {
        let reg = register();
        assert!(!reg.release_blocking_lane_refs.is_empty());
        let covered: Vec<&str> = reg
            .release_blocking_rows()
            .iter()
            .map(|row| row.surface_ref.as_str())
            .collect();
        for declared in &reg.release_blocking_lane_refs {
            assert!(
                covered.contains(&declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let reg = register();
        assert_eq!(reg.summary, reg.computed_summary());
        assert_eq!(
            reg.summary.entries_holding_stable + reg.summary.entries_narrowed,
            reg.rows.len()
        );
    }

    #[test]
    fn publication_decision_matches_computed() {
        let reg = register();
        assert_eq!(
            reg.publication.decision,
            reg.computed_publication_decision()
        );
        assert_eq!(
            reg.publication.blocking_rule_ids,
            reg.computed_blocking_rule_ids()
        );
        assert_eq!(
            reg.publication.blocking_claim_ids,
            reg.computed_blocking_entry_ids()
        );
    }

    #[test]
    fn every_gap_reason_has_a_stop_rule() {
        let reg = register();
        let covered: BTreeSet<ControlPacketGapReason> = reg
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in ControlPacketGapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn validate_flags_a_held_row_with_active_gap() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.active_gap_reasons
            .push(ControlPacketGapReason::ProofPacketMissing);
        reg.summary = reg.computed_summary();
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            M5ControlPacketRegisterViolation::HeldWithActiveGap { .. }
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_row_that_does_not_narrow() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.lane_state = ControlPacketLaneState::Incomplete;
        row.active_gap_reasons
            .push(ControlPacketGapReason::ControlPacketIncomplete);
        row.published_label = StableClaimLevel::Stable;
        reg.summary = reg.computed_summary();
        reg.publication.decision = reg.computed_publication_decision();
        reg.publication.blocking_rule_ids = reg.computed_blocking_rule_ids();
        reg.publication.blocking_claim_ids = reg.computed_blocking_entry_ids();
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            M5ControlPacketRegisterViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut reg = register();
        reg.publication.decision = PromotionDecision::Proceed;
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            M5ControlPacketRegisterViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_claim_without_signoff() {
        let mut reg = register();
        let row = reg
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        reg.summary = reg.computed_summary();
        assert!(reg.validate().iter().any(|v| matches!(
            v,
            M5ControlPacketRegisterViolation::HeldWithoutSignoff { .. }
        )));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let reg = register();
        let projection = reg.support_export_projection();
        assert_eq!(projection.rows.len(), reg.rows.len());
        for (row, proj) in reg.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, proj.entry_id);
            assert_eq!(row.publishes_stable(), proj.publishes_stable);
        }
    }
}
