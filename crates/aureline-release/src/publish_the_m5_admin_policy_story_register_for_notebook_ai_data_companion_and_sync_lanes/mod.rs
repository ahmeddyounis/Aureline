//! Typed M5 admin/policy story register for notebook, AI, data, companion, and sync lanes.
//!
//! This module publishes the canonical admin/policy story register that governs
//! the privacy, trust, access-control, audit, consent, and rollback posture for
//! every M5 depth lane. Each [`M5AdminPolicyLaneRow`] binds one lane to:
//!
//! - an [`AdminPolicyStory`] composed of required story items
//!   ([`AdminPolicyStoryItem`]), each with its kind, state, and artifact ref,
//! - the register state earned ([`AdminPolicyLaneState`]), the active gap reasons
//!   ([`AdminPolicyGapReason`]), and the effective label after narrowing
//!   ([`M5AdminPolicyLaneRow::published_label`]),
//! - a proof packet with freshness SLO ([`ProofPacket`]),
//! - owner sign-off ([`OwnerSignoff`]),
//! - stop rules that gate publication when story items are missing, stale, or
//!   unsigned.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between a lane that may publish as Stable and one that must narrow
//! below it. The [`AdminPolicyStopRule`] set names the closed conditions that
//! gate publication, and [`M5AdminPolicyStoryRegister::publication`] records the
//! proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/m5/publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes.json`
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
pub const PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_RECORD_KIND: &str =
    "publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes";

/// Repo-relative path to the checked-in register.
pub const PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_PATH: &str =
    "artifacts/release/m5/publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes.json";

/// Embedded checked-in register JSON.
pub const PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_JSON: &str =
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5/publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes.json"
    ));

/// M5 admin/policy lane a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdminPolicyLaneKind {
    /// Notebook and data-rich promoted surfaces.
    Notebook,
    /// AI-adjacent surfaces and language intelligence.
    AiAdjacent,
    /// Data-heavy surfaces (result grids, variable explorers).
    DataRich,
    /// Browser/mobile companion surfaces.
    Companion,
    /// Sync and device-registry surfaces.
    Sync,
}

impl M5AdminPolicyLaneKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Notebook,
        Self::AiAdjacent,
        Self::DataRich,
        Self::Companion,
        Self::Sync,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::AiAdjacent => "ai_adjacent",
            Self::DataRich => "data_rich",
            Self::Companion => "companion",
            Self::Sync => "sync",
        }
    }
}

/// Kind of item in an admin/policy story.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminPolicyStoryItemKind {
    /// Privacy disclosure and data-handling transparency.
    PrivacyDisclosure,
    /// Data retention and deletion policy.
    DataRetention,
    /// Access control and authorization policy.
    AccessControl,
    /// Audit trail and logging policy.
    AuditTrail,
    /// Consent management and user-choice policy.
    ConsentManagement,
    /// Rollback and downgrade policy.
    RollbackPolicy,
}

impl AdminPolicyStoryItemKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PrivacyDisclosure,
        Self::DataRetention,
        Self::AccessControl,
        Self::AuditTrail,
        Self::ConsentManagement,
        Self::RollbackPolicy,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrivacyDisclosure => "privacy_disclosure",
            Self::DataRetention => "data_retention",
            Self::AccessControl => "access_control",
            Self::AuditTrail => "audit_trail",
            Self::ConsentManagement => "consent_management",
            Self::RollbackPolicy => "rollback_policy",
        }
    }
}

/// State of an admin/policy story item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminPolicyStoryItemState {
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

impl AdminPolicyStoryItemState {
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
pub enum AdminPolicyLaneState {
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

impl AdminPolicyLaneState {
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

/// Closed reason an admin/policy lane narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminPolicyGapReason {
    /// Admin/policy story is incomplete.
    AdminPolicyStoryIncomplete,
    /// Admin/policy story is stale.
    AdminPolicyStoryStale,
    /// Proof packet is missing.
    ProofPacketMissing,
    /// Proof packet is stale.
    ProofPacketStale,
    /// Owner sign-off is missing.
    OwnerSignoffMissing,
    /// A required waiver has expired.
    WaiverExpired,
}

impl AdminPolicyGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AdminPolicyStoryIncomplete,
        Self::AdminPolicyStoryStale,
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdminPolicyStoryIncomplete => "admin_policy_story_incomplete",
            Self::AdminPolicyStoryStale => "admin_policy_story_stale",
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
pub enum AdminPolicyAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the public claim below the cutline.
    NarrowLabel,
    /// Staff the admin/policy story.
    StaffAdminPolicy,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl AdminPolicyAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::StaffAdminPolicy,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::StaffAdminPolicy => "staff_admin_policy",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One item in an admin/policy story.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdminPolicyStoryItem {
    /// Stable item id.
    pub item_id: String,
    /// Human-readable title.
    pub title: String,
    /// The kind of admin/policy story item.
    pub item_kind: AdminPolicyStoryItemKind,
    /// Ref to the artifact that fulfills this item.
    pub artifact_ref: String,
    /// Current state of the item.
    pub item_state: AdminPolicyStoryItemState,
    /// The owner responsible for this item.
    pub owner_ref: String,
    /// Reviewable reason this item carries this state.
    pub rationale: String,
}

/// The admin/policy story for a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdminPolicyStory {
    /// Stable story id.
    pub story_id: String,
    /// Story version.
    pub story_version: u32,
    /// Story items.
    pub items: Vec<AdminPolicyStoryItem>,
}

impl AdminPolicyStory {
    /// True when every required [`AdminPolicyStoryItemKind`] is present and complete.
    pub fn is_complete(&self) -> bool {
        let present: BTreeSet<AdminPolicyStoryItemKind> =
            self.items.iter().map(|e| e.item_kind).collect();
        if present.len() != AdminPolicyStoryItemKind::ALL.len() {
            return false;
        }
        self.items.iter().all(|e| e.item_state.is_complete())
    }

    /// True when every required item kind is present.
    pub fn has_all_required_items(&self) -> bool {
        let present: BTreeSet<AdminPolicyStoryItemKind> =
            self.items.iter().map(|e| e.item_kind).collect();
        present.len() == AdminPolicyStoryItemKind::ALL.len()
    }

    /// Returns items whose state is stale.
    pub fn stale_items(&self) -> Vec<&AdminPolicyStoryItem> {
        self.items
            .iter()
            .filter(|e| e.item_state == AdminPolicyStoryItemState::Stale)
            .collect()
    }

    /// Returns items whose state is missing.
    pub fn missing_items(&self) -> Vec<&AdminPolicyStoryItem> {
        self.items
            .iter()
            .filter(|e| e.item_state == AdminPolicyStoryItemState::Missing)
            .collect()
    }
}

/// One admin/policy stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdminPolicyStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: AdminPolicyGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: AdminPolicyAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One M5 admin/policy lane row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AdminPolicyLaneRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The lane kind this row governs.
    pub lane_kind: M5AdminPolicyLaneKind,
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
    /// The admin/policy story for this lane.
    pub admin_policy_story: AdminPolicyStory,
    /// Register state earned for the row.
    pub lane_state: AdminPolicyLaneState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<AdminPolicyGapReason>,
    /// The lifecycle label the lane effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl M5AdminPolicyLaneRow {
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
    pub fn has_active_reason(&self, reason: AdminPolicyGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AdminPolicyRegisterSummary {
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
    /// Notebook rows.
    pub notebook_entries: usize,
    /// AI-adjacent rows.
    pub ai_adjacent_entries: usize,
    /// Data-rich rows.
    pub data_rich_entries: usize,
    /// Companion rows.
    pub companion_entries: usize,
    /// Sync rows.
    pub sync_entries: usize,
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
pub struct M5AdminPolicyRegisterExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The lane kind this row governs.
    pub lane_kind: M5AdminPolicyLaneKind,
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
    pub lane_state: AdminPolicyLaneState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<AdminPolicyGapReason>,
    /// Owner ref.
    pub owner_ref: String,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdminPolicyRegisterExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5AdminPolicyRegisterExportRow>,
}

/// The typed M5 admin/policy story register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AdminPolicyStoryRegister {
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
    pub lane_kinds: Vec<M5AdminPolicyLaneKind>,
    /// Closed story-item-kind vocabulary.
    pub story_item_kinds: Vec<AdminPolicyStoryItemKind>,
    /// Closed story-item-state vocabulary.
    pub story_item_states: Vec<AdminPolicyStoryItemState>,
    /// Closed lane-state vocabulary.
    pub lane_states: Vec<AdminPolicyLaneState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<AdminPolicyGapReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<AdminPolicyAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_lane_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<AdminPolicyStopRule>,
    /// Lane rows.
    pub rows: Vec<M5AdminPolicyLaneRow>,
    /// Recorded publication verdict.
    pub publication: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5AdminPolicyRegisterSummary,
}

impl M5AdminPolicyStoryRegister {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&M5AdminPolicyLaneRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&M5AdminPolicyLaneRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&M5AdminPolicyLaneRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&M5AdminPolicyLaneRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one lane kind.
    pub fn rows_for_kind(&self, kind: M5AdminPolicyLaneKind) -> Vec<&M5AdminPolicyLaneRow> {
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
    pub fn stop_rule_fires(&self, rule: &AdminPolicyStopRule) -> bool {
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
        let blocking_triggers: BTreeSet<AdminPolicyGapReason> = self
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
    pub fn computed_summary(&self) -> M5AdminPolicyRegisterSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: M5AdminPolicyLaneKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&M5AdminPolicyLaneRow> = self.release_blocking_rows();
        let story_items = |state: AdminPolicyStoryItemState| {
            self.rows
                .iter()
                .flat_map(|row| &row.admin_policy_story.items)
                .filter(|e| e.item_state == state)
                .count()
        };
        M5AdminPolicyRegisterSummary {
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
                .filter(|row| row.lane_state == AdminPolicyLaneState::OnWaiver)
                .count(),
            entries_owner_blocked: self
                .rows
                .iter()
                .filter(|row| row.lane_state == AdminPolicyLaneState::OwnerBlocked)
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
            notebook_entries: kind(M5AdminPolicyLaneKind::Notebook),
            ai_adjacent_entries: kind(M5AdminPolicyLaneKind::AiAdjacent),
            data_rich_entries: kind(M5AdminPolicyLaneKind::DataRich),
            companion_entries: kind(M5AdminPolicyLaneKind::Companion),
            sync_entries: kind(M5AdminPolicyLaneKind::Sync),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            story_items_published: story_items(AdminPolicyStoryItemState::Published),
            story_items_stale: story_items(AdminPolicyStoryItemState::Stale),
            story_items_missing: story_items(AdminPolicyStoryItemState::Missing),
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
    pub fn support_export_projection(&self) -> M5AdminPolicyRegisterExportProjection {
        M5AdminPolicyRegisterExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5AdminPolicyRegisterExportRow {
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
    pub fn validate(&self) -> Vec<M5AdminPolicyRegisterViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5AdminPolicyRegisterViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5AdminPolicyRegisterViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5AdminPolicyRegisterViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5AdminPolicyRegisterViolation>) {
        if self.schema_version
            != PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_SCHEMA_VERSION
        {
            violations.push(M5AdminPolicyRegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind
            != PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_RECORD_KIND
        {
            violations.push(M5AdminPolicyRegisterViolation::UnsupportedRecordKind {
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
                violations.push(M5AdminPolicyRegisterViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5AdminPolicyRegisterViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.lane_kinds != M5AdminPolicyLaneKind::ALL.to_vec() {
            violations.push(M5AdminPolicyRegisterViolation::ClosedVocabularyMismatch {
                field: "lane_kinds",
            });
        }
        if self.story_item_kinds != AdminPolicyStoryItemKind::ALL.to_vec() {
            violations.push(M5AdminPolicyRegisterViolation::ClosedVocabularyMismatch {
                field: "story_item_kinds",
            });
        }
        if self.story_item_states != AdminPolicyStoryItemState::ALL.to_vec() {
            violations.push(M5AdminPolicyRegisterViolation::ClosedVocabularyMismatch {
                field: "story_item_states",
            });
        }
        if self.lane_states != AdminPolicyLaneState::ALL.to_vec() {
            violations.push(M5AdminPolicyRegisterViolation::ClosedVocabularyMismatch {
                field: "lane_states",
            });
        }
        if self.gap_reasons != AdminPolicyGapReason::ALL.to_vec() {
            violations.push(M5AdminPolicyRegisterViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.stop_rule_actions != AdminPolicyAction::ALL.to_vec() {
            violations.push(M5AdminPolicyRegisterViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5AdminPolicyRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5AdminPolicyRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5AdminPolicyRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5AdminPolicyRegisterViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5AdminPolicyRegisterViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5AdminPolicyRegisterViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5AdminPolicyRegisterViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5AdminPolicyRegisterViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5AdminPolicyRegisterViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in AdminPolicyGapReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(M5AdminPolicyRegisterViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &M5AdminPolicyLaneRow,
        violations: &mut Vec<M5AdminPolicyRegisterViolation>,
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
                violations.push(M5AdminPolicyRegisterViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no row may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5AdminPolicyRegisterViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5AdminPolicyRegisterViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5AdminPolicyRegisterViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A row that holds its label must have owner sign-off.
        if row.holds_label() && !row.owner_signoff.signed_off {
            violations.push(M5AdminPolicyRegisterViolation::HeldWithoutSignoff {
                entry_id: row.entry_id.clone(),
            });
        }

        // A row that holds its label must not have active gap reasons.
        if row.holds_label() && !row.active_gap_reasons.is_empty() {
            violations.push(M5AdminPolicyRegisterViolation::HeldWithActiveGap {
                entry_id: row.entry_id.clone(),
                reasons: row.active_gap_reasons.clone(),
            });
        }

        // A held row must ride a packet within its freshness SLO.
        let slo_state = row.proof_packet.slo_state;
        if row.holds_label() && !slo_state.is_within_slo() {
            violations.push(M5AdminPolicyRegisterViolation::HeldOnStalePacket {
                entry_id: row.entry_id.clone(),
                slo_state,
            });
        }

        // A row whose state forces narrowing must actually narrow.
        if row.lane_state.forces_narrowing() && row.published_label.rank() >= row.claim_label.rank()
        {
            violations.push(M5AdminPolicyRegisterViolation::PublishedLabelNotNarrowed {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
                state: row.lane_state,
            });
        }

        // Admin/policy story must contain all required item kinds.
        if !row.admin_policy_story.has_all_required_items() {
            violations.push(M5AdminPolicyRegisterViolation::IncompleteAdminPolicyStory {
                entry_id: row.entry_id.clone(),
            });
        }

        // A held row must have a complete admin/policy story.
        if row.holds_label() && !row.admin_policy_story.is_complete() {
            violations.push(M5AdminPolicyRegisterViolation::IncompleteAdminPolicyStory {
                entry_id: row.entry_id.clone(),
            });
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5AdminPolicyRegisterViolation>) {
        let covered_refs: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.surface_ref.clone())
            .collect();
        for surface_ref in &self.release_blocking_lane_refs {
            if !covered_refs.contains(surface_ref) {
                violations.push(
                    M5AdminPolicyRegisterViolation::ReleaseBlockingSurfaceUncovered {
                        surface_ref: surface_ref.clone(),
                    },
                );
            }
        }
        let rb_set: BTreeSet<String> = self.release_blocking_lane_refs.iter().cloned().collect();
        for row in &self.rows {
            if row.release_blocking && !rb_set.contains(&row.surface_ref) {
                violations.push(
                    M5AdminPolicyRegisterViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<M5AdminPolicyRegisterViolation>) {
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                M5AdminPolicyRegisterViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        let blocking_rules = self.computed_blocking_rule_ids();
        if self.publication.blocking_rule_ids != blocking_rules {
            violations.push(
                M5AdminPolicyRegisterViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        let blocking_entries = self.computed_blocking_entry_ids();
        if self.publication.blocking_claim_ids != blocking_entries {
            violations.push(
                M5AdminPolicyRegisterViolation::PublicationBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// Validation error for the M5 admin/policy story register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AdminPolicyRegisterViolation {
    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the register.
        actual: u32,
    },
    /// Record kind does not match the expected kind.
    UnsupportedRecordKind {
        /// Record kind found in the register.
        actual: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A closed vocabulary field does not match the canonical set.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// No stop rules are defined.
    NoStopRules,
    /// A stop-rule id appears more than once.
    DuplicateStopRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A stop rule watches no labels.
    StopRuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no covering stop rule.
    GapReasonWithoutStopRule {
        /// Uncovered reason.
        reason: AdminPolicyGapReason,
    },
    /// A row id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// The register contains no rows.
    EmptyRegister,
    /// The published label is wider than the canonical claim label.
    PublishedWiderThanClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
    /// A row that holds its label lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A row that holds its label has active gap reasons.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
        /// Active gap reasons.
        reasons: Vec<AdminPolicyGapReason>,
    },
    /// A held row rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A row whose state forces narrowing did not narrow.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
        /// Lane state.
        state: AdminPolicyLaneState,
    },
    /// A release-blocking surface has no covering row.
    ReleaseBlockingSurfaceUncovered {
        /// Surface ref.
        surface_ref: String,
    },
    /// A release-blocking row is not declared in release_blocking_lane_refs.
    ReleaseBlockingRowNotDeclared {
        /// Row id.
        entry_id: String,
    },
    /// The declared publication decision disagrees with the computed decision.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// A publication blocking set field disagrees with firing stop rules.
    PublicationBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// Summary counts disagree with row state.
    SummaryMismatch,
    /// Admin/policy story is missing required items.
    IncompleteAdminPolicyStory {
        /// Row id.
        entry_id: String,
    },
}

impl fmt::Display for M5AdminPolicyRegisterViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported schema version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported record kind {actual}")
            }
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id}: empty field {field_name}"),
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch: {field}")
            }
            Self::NoStopRules => write!(f, "no stop rules defined"),
            Self::DuplicateStopRuleId { rule_id } => {
                write!(f, "duplicate stop-rule id {rule_id}")
            }
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutStopRule { reason } => {
                write!(f, "gap reason {} has no stop rule", reason.as_str())
            }
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id {entry_id}")
            }
            Self::EmptyRegister => write!(f, "register has no rows"),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "{entry_id}: published label {published:?} is wider than claim {claim:?}"
            ),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "row {entry_id} freshness SLO window is inconsistent")
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(
                    f,
                    "row {entry_id} holds its label but lacks owner sign-off"
                )
            }
            Self::HeldWithActiveGap { entry_id, reasons } => {
                write!(
                    f,
                    "row {entry_id} holds its label but has active gaps: {reasons:?}"
                )
            }
            Self::HeldOnStalePacket { entry_id, slo_state } => {
                write!(
                    f,
                    "row {entry_id} holds stable on stale packet {slo_state:?}"
                )
            }
            Self::PublishedLabelNotNarrowed {
                entry_id,
                claim,
                published,
                state,
            } => write!(
                f,
                "row {entry_id} state {state:?} forces narrowing but claim {claim:?} / published {published:?} does not narrow"
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
            Self::IncompleteAdminPolicyStory { entry_id } => {
                write!(f, "row {entry_id} admin/policy story is incomplete")
            }
        }
    }
}

impl Error for M5AdminPolicyRegisterViolation {}

/// Loads the embedded M5 admin/policy story register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`M5AdminPolicyStoryRegister`].
pub fn current_m5_admin_policy_story_register(
) -> Result<M5AdminPolicyStoryRegister, serde_json::Error> {
    serde_json::from_str(
        PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_JSON,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn register() -> M5AdminPolicyStoryRegister {
        current_m5_admin_policy_story_register().expect("register parses")
    }

    #[test]
    fn embedded_register_parses_and_validates() {
        let r = register();
        assert_eq!(
            r.schema_version,
            PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_SCHEMA_VERSION
        );
        assert_eq!(
            r.record_kind,
            PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_RECORD_KIND
        );
        assert_eq!(r.validate(), Vec::new());
        assert!(!r.rows.is_empty());
    }

    #[test]
    fn covers_every_lane_kind() {
        let r = register();
        for kind in M5AdminPolicyLaneKind::ALL {
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
    fn publication_decision_matches_computed() {
        let r = register();
        assert_eq!(r.publication.decision, r.computed_publication_decision());
        assert_eq!(
            r.publication.blocking_rule_ids,
            r.computed_blocking_rule_ids()
        );
        assert_eq!(
            r.publication.blocking_claim_ids,
            r.computed_blocking_entry_ids()
        );
    }

    #[test]
    fn every_gap_reason_has_a_stop_rule() {
        let r = register();
        let covered: BTreeSet<AdminPolicyGapReason> = r
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in AdminPolicyGapReason::ALL {
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
            .push(AdminPolicyGapReason::ProofPacketMissing);
        r.summary = r.computed_summary();
        assert!(r
            .validate()
            .iter()
            .any(|v| matches!(v, M5AdminPolicyRegisterViolation::HeldWithActiveGap { .. })));
    }

    #[test]
    fn validate_flags_a_narrowing_row_that_does_not_narrow() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.lane_state = AdminPolicyLaneState::Incomplete;
        row.active_gap_reasons
            .push(AdminPolicyGapReason::ProofPacketMissing);
        row.published_label = StableClaimLevel::Stable;
        r.summary = r.computed_summary();
        r.publication.decision = r.computed_publication_decision();
        r.publication.blocking_rule_ids = r.computed_blocking_rule_ids();
        r.publication.blocking_claim_ids = r.computed_blocking_entry_ids();
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5AdminPolicyRegisterViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_row_on_stale_packet() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.proof_packet.slo_state = FreshnessSloState::Breached;
        r.summary = r.computed_summary();
        assert!(r
            .validate()
            .iter()
            .any(|v| matches!(v, M5AdminPolicyRegisterViolation::HeldOnStalePacket { .. })));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut r = register();
        r.publication.decision = PromotionDecision::Proceed;
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5AdminPolicyRegisterViolation::PublicationDecisionInconsistent { .. }
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
            .any(|v| matches!(v, M5AdminPolicyRegisterViolation::HeldWithoutSignoff { .. })));
    }

    #[test]
    fn validate_flags_an_incomplete_admin_policy_story() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.admin_policy_story.items.clear();
        r.summary = r.computed_summary();
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5AdminPolicyRegisterViolation::IncompleteAdminPolicyStory { .. }
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
