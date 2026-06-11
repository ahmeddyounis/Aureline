//! Typed M5 depth-claim manifest, feature-family packets, and qualification matrix.
//!
//! This module freezes the canonical M5 depth-claim control surface. Where the
//! feature-train matrix speaks for lanes, scorecards, and the dependency graph,
//! this manifest speaks for the *depth claim* each M5 feature family publishes
//! and the *qualification matrix* that grounds it. Each [`FamilyPacket`] binds
//! one M5 feature family to:
//!
//! - the stable claim it backs ([`FamilyPacket::claim_ref`],
//!   [`FamilyPacket::claim_label`]),
//! - a qualification matrix ([`FamilyPacket::qualification_matrix`]) of one
//!   [`QualificationCell`] per [`QualificationDimension`], so scorecard,
//!   compatibility, proof freshness, generated-artifact lineage, locale parity,
//!   support-packet currency, accessibility, and downgrade automation are each
//!   an explicit, inspectable truth,
//! - the overall packet state earned ([`PacketState`]), the active narrowing
//!   reasons ([`NarrowingReason`]), and the effective label after narrowing
//!   ([`FamilyPacket::published_label`]),
//! - a [`ProofPacket`] (reused from the stable claim manifest) and its freshness
//!   SLO, an owner sign-off, and an optional waiver.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between a family that may publish a Stable depth claim and one that
//! must narrow below it. The [`DepthStopRule`] set names the closed conditions
//! that gate M5 depth promotion — one per [`NarrowingReason`] — and
//! [`DepthClaimManifest::promotion`] records the proceed/hold verdict.
//!
//! The manifest is checked in at
//! `artifacts/release/m5/freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix.json`
//! and embedded here, so this typed consumer and the CI gate agree on every
//! packet without a cargo build in CI.
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

/// Supported manifest schema version.
pub const FREEZE_M5_DEPTH_CLAIM_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the manifest.
pub const FREEZE_M5_DEPTH_CLAIM_MANIFEST_RECORD_KIND: &str =
    "freeze_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix";

/// Repo-relative path to the checked-in manifest.
pub const FREEZE_M5_DEPTH_CLAIM_MANIFEST_PATH: &str =
    "artifacts/release/m5/freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix.json";

/// Embedded checked-in manifest JSON.
pub const FREEZE_M5_DEPTH_CLAIM_MANIFEST_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix.json"
));

/// M5 feature family a packet governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FamilyKind {
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

impl FamilyKind {
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

    /// Stable token recorded in the manifest.
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

/// One column of the per-family qualification matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationDimension {
    /// The feature scorecard is published and complete.
    Scorecard,
    /// The compatibility packet is present and current.
    Compatibility,
    /// The proof packet is captured and within its freshness SLO.
    ProofFreshness,
    /// Generated-artifact lineage is recorded for every generated asset.
    Lineage,
    /// Locale packs hold parity with the source strings.
    LocaleParity,
    /// The support packet matches shipped behavior.
    SupportPacket,
    /// Accessibility signoff is recorded.
    Accessibility,
    /// Downgrade automation that narrows the claim is defined.
    DowngradeAutomation,
}

impl QualificationDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Scorecard,
        Self::Compatibility,
        Self::ProofFreshness,
        Self::Lineage,
        Self::LocaleParity,
        Self::SupportPacket,
        Self::Accessibility,
        Self::DowngradeAutomation,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scorecard => "scorecard",
            Self::Compatibility => "compatibility",
            Self::ProofFreshness => "proof_freshness",
            Self::Lineage => "lineage",
            Self::LocaleParity => "locale_parity",
            Self::SupportPacket => "support_packet",
            Self::Accessibility => "accessibility",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// The narrowing reason a non-qualified, non-waived cell in this dimension
    /// must name, given the cell's [`QualificationState`].
    pub const fn reason_for_state(self, state: QualificationState) -> Option<NarrowingReason> {
        match self {
            Self::Scorecard => Some(NarrowingReason::ScorecardIncomplete),
            Self::Compatibility => Some(NarrowingReason::CompatibilityMissing),
            Self::ProofFreshness => match state {
                QualificationState::Stale => Some(NarrowingReason::ProofPacketStale),
                _ => Some(NarrowingReason::ProofPacketMissing),
            },
            Self::Lineage => Some(NarrowingReason::LineageMissing),
            Self::LocaleParity => Some(NarrowingReason::LocaleParityDrifted),
            Self::SupportPacket => Some(NarrowingReason::SupportPacketLagging),
            Self::Accessibility => Some(NarrowingReason::AccessibilityUnsigned),
            Self::DowngradeAutomation => Some(NarrowingReason::DowngradeAutomationMissing),
        }
    }
}

/// The state of one qualification-matrix cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationState {
    /// The dimension is fully qualified.
    Qualified,
    /// Required qualification work for the dimension is incomplete.
    Incomplete,
    /// Qualification existed but has gone stale.
    Stale,
    /// Held provisionally under an active, unexpired waiver.
    Waived,
    /// The dimension has no qualification evidence at all.
    Missing,
}

impl QualificationState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Qualified,
        Self::Incomplete,
        Self::Stale,
        Self::Waived,
        Self::Missing,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Incomplete => "incomplete",
            Self::Stale => "stale",
            Self::Waived => "waived",
            Self::Missing => "missing",
        }
    }

    /// Whether a cell in this state lets the family hold its depth claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Qualified | Self::Waived)
    }
}

/// Overall qualification state a feature-family packet earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketState {
    /// Every dimension is qualified and current.
    Qualified,
    /// One or more required dimensions are incomplete or missing.
    Incomplete,
    /// A dimension has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers a
    /// recorded gap.
    OnWaiver,
    /// Generated-artifact lineage is missing.
    LineageMissing,
    /// Locale parity has drifted.
    LocaleDrifted,
    /// The support packet lags shipped behavior.
    SupportLagging,
}

impl PacketState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Qualified,
        Self::Incomplete,
        Self::Stale,
        Self::OnWaiver,
        Self::LineageMissing,
        Self::LocaleDrifted,
        Self::SupportLagging,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Incomplete => "incomplete",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::LineageMissing => "lineage_missing",
            Self::LocaleDrifted => "locale_drifted",
            Self::SupportLagging => "support_lagging",
        }
    }

    /// Whether the state lets a family carry the depth claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Qualified | Self::OnWaiver)
    }

    /// Whether the state forces the family below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason an M5 depth claim narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// The feature scorecard is incomplete.
    ScorecardIncomplete,
    /// The compatibility packet is missing.
    CompatibilityMissing,
    /// The proof packet is missing.
    ProofPacketMissing,
    /// The proof packet is stale.
    ProofPacketStale,
    /// Generated-artifact lineage is missing.
    LineageMissing,
    /// Locale parity has drifted.
    LocaleParityDrifted,
    /// The support packet lags shipped behavior.
    SupportPacketLagging,
    /// Accessibility signoff is missing.
    AccessibilityUnsigned,
    /// Downgrade automation is undefined.
    DowngradeAutomationMissing,
    /// A waiver the family relied on has expired.
    WaiverExpired,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ScorecardIncomplete,
        Self::CompatibilityMissing,
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::LineageMissing,
        Self::LocaleParityDrifted,
        Self::SupportPacketLagging,
        Self::AccessibilityUnsigned,
        Self::DowngradeAutomationMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScorecardIncomplete => "scorecard_incomplete",
            Self::CompatibilityMissing => "compatibility_missing",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::LineageMissing => "lineage_missing",
            Self::LocaleParityDrifted => "locale_parity_drifted",
            Self::SupportPacketLagging => "support_packet_lagging",
            Self::AccessibilityUnsigned => "accessibility_unsigned",
            Self::DowngradeAutomationMissing => "downgrade_automation_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepthStopAction {
    /// Hold promotion until the condition clears.
    HoldPromotion,
    /// Narrow the depth claim below the cutline.
    NarrowLabel,
    /// Complete the feature scorecard.
    CompleteScorecard,
    /// Refresh the compatibility packet.
    RefreshCompatibilityPacket,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Refresh the generated-artifact lineage record.
    RefreshLineage,
    /// Refresh the locale pack to restore parity.
    RefreshLocalePack,
    /// Refresh the support packet to match shipped behavior.
    RefreshSupportPacket,
    /// Record the accessibility signoff.
    SignAccessibility,
    /// Define the downgrade automation.
    DefineDowngradeAutomation,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl DepthStopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::HoldPromotion,
        Self::NarrowLabel,
        Self::CompleteScorecard,
        Self::RefreshCompatibilityPacket,
        Self::RefreshProofPacket,
        Self::RefreshLineage,
        Self::RefreshLocalePack,
        Self::RefreshSupportPacket,
        Self::SignAccessibility,
        Self::DefineDowngradeAutomation,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the manifest.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowLabel => "narrow_label",
            Self::CompleteScorecard => "complete_scorecard",
            Self::RefreshCompatibilityPacket => "refresh_compatibility_packet",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RefreshLineage => "refresh_lineage",
            Self::RefreshLocalePack => "refresh_locale_pack",
            Self::RefreshSupportPacket => "refresh_support_packet",
            Self::SignAccessibility => "sign_accessibility",
            Self::DefineDowngradeAutomation => "define_downgrade_automation",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One cell of the per-family qualification matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationCell {
    /// The qualification dimension this cell speaks for.
    pub dimension: QualificationDimension,
    /// The qualification state earned for the dimension.
    pub state: QualificationState,
    /// Ref to the dimension's evidence. Empty only on a missing cell.
    pub evidence_ref: String,
}

/// One M5 depth-claim stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DepthStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched packet fires this rule.
    pub trigger_reason: NarrowingReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: DepthStopAction,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One M5 feature-family packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyPacket {
    /// Stable packet id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The feature family this packet governs.
    pub family_kind: FamilyKind,
    /// The family ref this packet speaks about.
    pub family_ref: String,
    /// Reviewable one-line statement of the family.
    pub family_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose depth claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the depth claim publishes.
    pub claim_label: StableClaimLevel,
    /// Overall qualification state earned for the packet.
    pub packet_state: PacketState,
    /// The qualification matrix: one cell per [`QualificationDimension`].
    pub qualification_matrix: Vec<QualificationCell>,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the packet below its claim label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The lifecycle label the family effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this packet's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the packet carries this posture.
    pub rationale: String,
}

impl FamilyPacket {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the depth claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the packet's state lets the family carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.packet_state.holds_label()
    }

    /// True when a narrowing reason is active on the packet.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// Returns the cell registered for `dimension`, if any.
    pub fn cell(&self, dimension: QualificationDimension) -> Option<&QualificationCell> {
        self.qualification_matrix
            .iter()
            .find(|cell| cell.dimension == dimension)
    }
}

/// Summary counts carried by the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DepthClaimManifestSummary {
    /// Total number of family packets.
    pub total_entries: usize,
    /// Distinct depth claims covered.
    pub total_claims: usize,
    /// Packets publishing a label at or above the cutline.
    pub entries_qualified: usize,
    /// Packets narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Packets holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Packets carrying a lineage-missing reason.
    pub entries_with_lineage_gap: usize,
    /// Packets carrying a locale-parity-drifted reason.
    pub entries_with_locale_gap: usize,
    /// Packets carrying a support-packet-lagging reason.
    pub entries_with_support_gap: usize,
    /// Total release-blocking packets.
    pub release_blocking_total: usize,
    /// Release-blocking packets publishing a label at or above the cutline.
    pub release_blocking_qualified: usize,
    /// Release-blocking packets narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook packets.
    pub notebook_entries: usize,
    /// Data-rich packets.
    pub data_rich_entries: usize,
    /// AI-adjacent packets.
    pub ai_adjacent_entries: usize,
    /// Framework packets.
    pub framework_entries: usize,
    /// Review packets.
    pub review_entries: usize,
    /// Companion packets.
    pub companion_entries: usize,
    /// Managed-depth packets.
    pub managed_depth_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all packets.
    pub total_active_narrowing_reasons: usize,
    /// Total qualification cells across all packets.
    pub total_qualification_cells: usize,
    /// Cells in the `qualified` state.
    pub cells_qualified: usize,
    /// Cells in the `incomplete` state.
    pub cells_incomplete: usize,
    /// Cells in the `stale` state.
    pub cells_stale: usize,
    /// Cells in the `waived` state.
    pub cells_waived: usize,
    /// Cells in the `missing` state.
    pub cells_missing: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepthClaimExportRow {
    /// Stable packet id.
    pub entry_id: String,
    /// The feature family this packet governs.
    pub family_kind: FamilyKind,
    /// The family ref this packet speaks about.
    pub family_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose depth claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the packet publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall packet state earned.
    pub packet_state: PacketState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepthClaimExportProjection {
    /// Manifest identifier.
    pub manifest_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<DepthClaimExportRow>,
}

/// The typed M5 depth-claim manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DepthClaimManifest {
    /// Manifest schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable manifest identifier.
    pub manifest_id: String,
    /// Lifecycle status of this manifest artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this manifest ingests.
    pub claim_manifest_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed family-kind vocabulary.
    pub family_kinds: Vec<FamilyKind>,
    /// Closed qualification-dimension vocabulary.
    pub qualification_dimensions: Vec<QualificationDimension>,
    /// Closed qualification-state vocabulary.
    pub qualification_states: Vec<QualificationState>,
    /// Closed packet-state vocabulary.
    pub packet_states: Vec<PacketState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<DepthStopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking family refs this manifest must cover.
    pub release_blocking_family_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<DepthStopRule>,
    /// Family packets.
    pub rows: Vec<FamilyPacket>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: DepthClaimManifestSummary,
}

impl DepthClaimManifest {
    /// Returns the packet registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&FamilyPacket> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the packets publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&FamilyPacket> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the packets narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&FamilyPacket> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking packets.
    pub fn release_blocking_rows(&self) -> Vec<&FamilyPacket> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the packets for one family kind.
    pub fn rows_for_kind(&self, kind: FamilyKind) -> Vec<&FamilyPacket> {
        self.rows
            .iter()
            .filter(|row| row.family_kind == kind)
            .collect()
    }

    /// Distinct depth claims (by claim ref) the manifest covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched packet carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &DepthStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the packets and stop rules.
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

    /// Packet ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only packets whose depth claim is at or above the cutline count: a packet
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

    /// Recomputes the summary block from the packets and stop rules.
    pub fn computed_summary(&self) -> DepthClaimManifestSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: FamilyKind| self.rows_for_kind(kind).len();
        let with_reason = |reason: NarrowingReason| {
            self.rows
                .iter()
                .filter(|row| row.has_active_reason(reason))
                .count()
        };
        let cell_state = |state: QualificationState| {
            self.rows
                .iter()
                .flat_map(|row| row.qualification_matrix.iter())
                .filter(|cell| cell.state == state)
                .count()
        };
        let release_blocking: Vec<&FamilyPacket> = self.release_blocking_rows();
        DepthClaimManifestSummary {
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
                .filter(|row| row.packet_state == PacketState::OnWaiver)
                .count(),
            entries_with_lineage_gap: with_reason(NarrowingReason::LineageMissing),
            entries_with_locale_gap: with_reason(NarrowingReason::LocaleParityDrifted),
            entries_with_support_gap: with_reason(NarrowingReason::SupportPacketLagging),
            release_blocking_total: release_blocking.len(),
            release_blocking_qualified: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            notebook_entries: kind(FamilyKind::Notebook),
            data_rich_entries: kind(FamilyKind::DataRich),
            ai_adjacent_entries: kind(FamilyKind::AiAdjacent),
            framework_entries: kind(FamilyKind::Framework),
            review_entries: kind(FamilyKind::Review),
            companion_entries: kind(FamilyKind::Companion),
            managed_depth_entries: kind(FamilyKind::ManagedDepth),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_narrowing_reasons: self
                .rows
                .iter()
                .map(|row| row.active_narrowing_reasons.len())
                .sum(),
            total_qualification_cells: self
                .rows
                .iter()
                .map(|row| row.qualification_matrix.len())
                .sum(),
            cells_qualified: cell_state(QualificationState::Qualified),
            cells_incomplete: cell_state(QualificationState::Incomplete),
            cells_stale: cell_state(QualificationState::Stale),
            cells_waived: cell_state(QualificationState::Waived),
            cells_missing: cell_state(QualificationState::Missing),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection that downstream surfaces
    /// render instead of cloning status text.
    pub fn support_export_projection(&self) -> DepthClaimExportProjection {
        DepthClaimExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| DepthClaimExportRow {
                    entry_id: row.entry_id.clone(),
                    family_kind: row.family_kind,
                    family_ref: row.family_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    packet_state: row.packet_state,
                    slo_state: row.proof_packet.slo_state,
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the manifest, returning every violation found.
    pub fn validate(&self) -> Vec<DepthClaimManifestViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(DepthClaimManifestViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(DepthClaimManifestViolation::EmptyManifest);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(DepthClaimManifestViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<DepthClaimManifestViolation>) {
        if self.schema_version != FREEZE_M5_DEPTH_CLAIM_MANIFEST_SCHEMA_VERSION {
            violations.push(DepthClaimManifestViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != FREEZE_M5_DEPTH_CLAIM_MANIFEST_RECORD_KIND {
            violations.push(DepthClaimManifestViolation::UnsupportedRecordKind {
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
                violations.push(DepthClaimManifestViolation::EmptyField {
                    entry_id: "<manifest>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(DepthClaimManifestViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.family_kinds != FamilyKind::ALL.to_vec() {
            violations.push(DepthClaimManifestViolation::ClosedVocabularyMismatch {
                field: "family_kinds",
            });
        }
        if self.qualification_dimensions != QualificationDimension::ALL.to_vec() {
            violations.push(DepthClaimManifestViolation::ClosedVocabularyMismatch {
                field: "qualification_dimensions",
            });
        }
        if self.qualification_states != QualificationState::ALL.to_vec() {
            violations.push(DepthClaimManifestViolation::ClosedVocabularyMismatch {
                field: "qualification_states",
            });
        }
        if self.packet_states != PacketState::ALL.to_vec() {
            violations.push(DepthClaimManifestViolation::ClosedVocabularyMismatch {
                field: "packet_states",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(DepthClaimManifestViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.stop_rule_actions != DepthStopAction::ALL.to_vec() {
            violations.push(DepthClaimManifestViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(DepthClaimManifestViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(DepthClaimManifestViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(DepthClaimManifestViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(DepthClaimManifestViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<DepthClaimManifestViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(DepthClaimManifestViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(DepthClaimManifestViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(DepthClaimManifestViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(DepthClaimManifestViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(DepthClaimManifestViolation::NarrowingReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(&self, row: &FamilyPacket, violations: &mut Vec<DepthClaimManifestViolation>) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("family_ref", &row.family_ref),
            ("family_summary", &row.family_summary),
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
                violations.push(DepthClaimManifestViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_qualification_matrix(row, violations);

        // The ceiling: no family may carry a label wider than the depth claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(DepthClaimManifestViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(DepthClaimManifestViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(DepthClaimManifestViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A depth claim whose canonical label is below the cutline forces the
        // family to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(DepthClaimManifestViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(DepthClaimManifestViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.packet_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed family carries exactly the depth claim's canonical label,
            // carries no active reason, rides a captured within-SLO packet, and is
            // owner-signed.
            if row.published_label != row.claim_label {
                violations.push(DepthClaimManifestViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(DepthClaimManifestViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(DepthClaimManifestViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(DepthClaimManifestViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(DepthClaimManifestViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(DepthClaimManifestViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.packet_state,
                    published: row.published_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(DepthClaimManifestViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.packet_state,
                });
            }
            // A narrowing family whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::ProofPacketStale)
            {
                violations.push(DepthClaimManifestViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
            {
                violations.push(DepthClaimManifestViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_qualification_matrix(
        &self,
        row: &FamilyPacket,
        violations: &mut Vec<DepthClaimManifestViolation>,
    ) {
        let mut seen: BTreeSet<QualificationDimension> = BTreeSet::new();
        for cell in &row.qualification_matrix {
            if !seen.insert(cell.dimension) {
                violations.push(DepthClaimManifestViolation::DuplicateDimension {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A missing cell carries no evidence ref; every other state must.
            if cell.state != QualificationState::Missing && cell.evidence_ref.trim().is_empty() {
                violations.push(DepthClaimManifestViolation::CellEvidenceMissing {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A waived cell only holds under an unexpired waiver.
            if cell.state == QualificationState::Waived && row.waiver.is_none() {
                violations.push(DepthClaimManifestViolation::WaivedCellWithoutWaiver {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A non-qualified, non-waived cell must name its narrowing reason.
            if !cell.state.holds() {
                if let Some(reason) = cell.dimension.reason_for_state(cell.state) {
                    if !row.has_active_reason(reason) {
                        violations.push(DepthClaimManifestViolation::CellReasonNotActive {
                            entry_id: row.entry_id.clone(),
                            dimension: cell.dimension,
                            reason,
                        });
                    }
                }
            }
        }
        // The qualification matrix must carry exactly one cell per dimension.
        for dimension in QualificationDimension::ALL {
            if !seen.contains(&dimension) {
                violations.push(
                    DepthClaimManifestViolation::QualificationMatrixIncompleteCoverage {
                        entry_id: row.entry_id.clone(),
                        dimension,
                    },
                );
            }
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &FamilyPacket,
        violations: &mut Vec<DepthClaimManifestViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<DepthClaimManifestViolation>,
                               expected: NarrowingReason| {
            violations.push(DepthClaimManifestViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.packet_state,
                expected_reason: expected,
            });
        };

        match row.packet_state {
            PacketState::Incomplete => {
                if !row.has_active_reason(NarrowingReason::ScorecardIncomplete)
                    && !row.has_active_reason(NarrowingReason::CompatibilityMissing)
                    && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
                    && !row.has_active_reason(NarrowingReason::AccessibilityUnsigned)
                    && !row.has_active_reason(NarrowingReason::DowngradeAutomationMissing)
                {
                    push_incoherent(violations, NarrowingReason::ScorecardIncomplete);
                }
            }
            PacketState::Stale => {
                if !row.has_active_reason(NarrowingReason::ProofPacketStale) {
                    push_incoherent(violations, NarrowingReason::ProofPacketStale);
                }
            }
            PacketState::LineageMissing => {
                if !row.has_active_reason(NarrowingReason::LineageMissing) {
                    push_incoherent(violations, NarrowingReason::LineageMissing);
                }
            }
            PacketState::LocaleDrifted => {
                if !row.has_active_reason(NarrowingReason::LocaleParityDrifted) {
                    push_incoherent(violations, NarrowingReason::LocaleParityDrifted);
                }
            }
            PacketState::SupportLagging => {
                if !row.has_active_reason(NarrowingReason::SupportPacketLagging) {
                    push_incoherent(violations, NarrowingReason::SupportPacketLagging);
                }
            }
            PacketState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(DepthClaimManifestViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.packet_state,
                    });
                }
            }
            PacketState::Qualified => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<DepthClaimManifestViolation>) {
        let covered: BTreeSet<String> =
            self.rows.iter().map(|row| row.family_ref.clone()).collect();
        for declared in &self.release_blocking_family_refs {
            if !covered.contains(declared) {
                violations.push(
                    DepthClaimManifestViolation::ReleaseBlockingFamilyUncovered {
                        family_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_family_refs.contains(&row.family_ref)
            {
                violations.push(DepthClaimManifestViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<DepthClaimManifestViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(DepthClaimManifestViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(DepthClaimManifestViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(DepthClaimManifestViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(DepthClaimManifestViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(DepthClaimManifestViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the M5 depth-claim manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepthClaimManifestViolation {
    /// The manifest carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the manifest.
        actual: u32,
    },
    /// The manifest carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the manifest.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The manifest has no packets.
    EmptyManifest,
    /// The manifest has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Packet or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A packet id appears more than once.
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
    /// A qualification matrix has two cells for one dimension.
    DuplicateDimension {
        /// Packet id.
        entry_id: String,
        /// Duplicated dimension.
        dimension: QualificationDimension,
    },
    /// A qualification matrix is missing a dimension cell.
    QualificationMatrixIncompleteCoverage {
        /// Packet id.
        entry_id: String,
        /// Uncovered dimension.
        dimension: QualificationDimension,
    },
    /// A non-missing cell has no evidence ref.
    CellEvidenceMissing {
        /// Packet id.
        entry_id: String,
        /// Dimension.
        dimension: QualificationDimension,
    },
    /// A waived cell is carried without a waiver.
    WaivedCellWithoutWaiver {
        /// Packet id.
        entry_id: String,
        /// Dimension.
        dimension: QualificationDimension,
    },
    /// A non-qualified cell does not name its narrowing reason.
    CellReasonNotActive {
        /// Packet id.
        entry_id: String,
        /// Dimension.
        dimension: QualificationDimension,
        /// The reason the cell requires.
        reason: NarrowingReason,
    },
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Packet id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A packet holds a label while the depth claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Packet id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing state carries no active reason.
    NarrowingWithoutReason {
        /// Packet id.
        entry_id: String,
        /// Packet state.
        state: PacketState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Packet id.
        entry_id: String,
        /// Packet state.
        state: PacketState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held packet carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Packet id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held packet has active narrowing reasons.
    HeldWithActiveGap {
        /// Packet id.
        entry_id: String,
    },
    /// A held packet has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Packet id.
        entry_id: String,
    },
    /// A held packet rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Packet id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held packet lacks owner sign-off.
    HeldWithoutSignoff {
        /// Packet id.
        entry_id: String,
    },
    /// A narrowing packet with a breached proof packet does not name the stale reason.
    BreachedPacketWithoutReason {
        /// Packet id.
        entry_id: String,
    },
    /// A narrowing packet with a missing proof packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Packet id.
        entry_id: String,
    },
    /// A packet state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Packet id.
        entry_id: String,
        /// Packet state.
        state: PacketState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Packet id.
        entry_id: String,
        /// Packet state.
        state: PacketState,
    },
    /// A release-blocking family ref has no covering packet.
    ReleaseBlockingFamilyUncovered {
        /// Family ref.
        family_ref: String,
    },
    /// A release-blocking packet is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Packet id.
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
    /// The summary counts disagree with the packets.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Packet id.
        entry_id: String,
    },
}

impl fmt::Display for DepthClaimManifestViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported manifest schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported manifest record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "manifest {field} is not the canonical value")
            }
            Self::EmptyManifest => write!(f, "manifest has no packets"),
            Self::NoStopRules => write!(f, "manifest has no stop rules"),
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
                "packet {entry_id} has duplicate dimension {}",
                dimension.as_str()
            ),
            Self::QualificationMatrixIncompleteCoverage {
                entry_id,
                dimension,
            } => write!(
                f,
                "packet {entry_id} qualification matrix is missing dimension {}",
                dimension.as_str()
            ),
            Self::CellEvidenceMissing {
                entry_id,
                dimension,
            } => write!(
                f,
                "packet {entry_id} dimension {} has no evidence ref",
                dimension.as_str()
            ),
            Self::WaivedCellWithoutWaiver {
                entry_id,
                dimension,
            } => write!(
                f,
                "packet {entry_id} dimension {} is waived without a waiver",
                dimension.as_str()
            ),
            Self::CellReasonNotActive {
                entry_id,
                dimension,
                reason,
            } => write!(
                f,
                "packet {entry_id} dimension {} requires active reason {}",
                dimension.as_str(),
                reason.as_str()
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "packet {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "packet {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "packet {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "packet {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "packet {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "packet {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "packet {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "packet {entry_id} holds stable on stale packet {slo_state:?}"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "packet {entry_id} holds stable without owner signoff")
            }
            Self::BreachedPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "packet {entry_id} breached packet without proof_packet_stale reason"
                )
            }
            Self::MissingPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "packet {entry_id} missing packet without proof_packet_missing reason"
                )
            }
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "packet {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "packet {entry_id} state {state:?} names no waiver")
            }
            Self::ReleaseBlockingFamilyUncovered { family_ref } => {
                write!(
                    f,
                    "release-blocking family {family_ref} has no covering packet"
                )
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => {
                write!(
                    f,
                    "release-blocking packet {entry_id} is not declared in release_blocking_family_refs"
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
            Self::SummaryMismatch => write!(f, "summary counts disagree with packets"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "packet {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for DepthClaimManifestViolation {}

/// Loads the embedded M5 depth-claim manifest.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in manifest no longer matches
/// [`DepthClaimManifest`].
pub fn current_m5_depth_claim_manifest() -> Result<DepthClaimManifest, serde_json::Error> {
    serde_json::from_str(FREEZE_M5_DEPTH_CLAIM_MANIFEST_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> DepthClaimManifest {
        current_m5_depth_claim_manifest().expect("manifest parses")
    }

    #[test]
    fn embedded_manifest_parses_and_validates() {
        let m = manifest();
        assert_eq!(
            m.schema_version,
            FREEZE_M5_DEPTH_CLAIM_MANIFEST_SCHEMA_VERSION
        );
        assert_eq!(m.record_kind, FREEZE_M5_DEPTH_CLAIM_MANIFEST_RECORD_KIND);
        assert_eq!(m.validate(), Vec::new());
        assert!(!m.rows.is_empty());
    }

    #[test]
    fn covers_every_family_kind() {
        let m = manifest();
        for kind in FamilyKind::ALL {
            assert!(
                !m.rows_for_kind(kind).is_empty(),
                "family kind {} must have at least one packet",
                kind.as_str()
            );
        }
    }

    #[test]
    fn every_packet_covers_every_dimension() {
        let m = manifest();
        for row in &m.rows {
            for dimension in QualificationDimension::ALL {
                assert!(
                    row.cell(dimension).is_some(),
                    "packet {} must cover dimension {}",
                    row.entry_id,
                    dimension.as_str()
                );
            }
        }
    }

    #[test]
    fn covers_every_declared_release_blocking_family() {
        let m = manifest();
        assert!(!m.release_blocking_family_refs.is_empty());
        let covered: Vec<&str> = m
            .release_blocking_rows()
            .iter()
            .map(|row| row.family_ref.as_str())
            .collect();
        for declared in &m.release_blocking_family_refs {
            assert!(
                covered.contains(&declared.as_str()),
                "{declared} has no covering release-blocking packet"
            );
        }
    }

    #[test]
    fn summary_counts_match_packets() {
        let m = manifest();
        assert_eq!(m.summary, m.computed_summary());
        assert_eq!(
            m.summary.entries_qualified + m.summary.entries_narrowed,
            m.rows.len()
        );
    }

    #[test]
    fn promotion_decision_matches_computed() {
        let m = manifest();
        assert_eq!(m.promotion.decision, m.computed_promotion_decision());
        assert_eq!(
            m.promotion.blocking_rule_ids,
            m.computed_blocking_rule_ids()
        );
        assert_eq!(
            m.promotion.blocking_claim_ids,
            m.computed_blocking_entry_ids()
        );
    }

    #[test]
    fn every_narrowing_reason_has_a_stop_rule() {
        let m = manifest();
        let covered: BTreeSet<NarrowingReason> = m
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in NarrowingReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn validate_flags_a_held_packet_with_active_gap() {
        let mut m = manifest();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held packet exists");
        row.active_narrowing_reasons
            .push(NarrowingReason::LocaleParityDrifted);
        m.summary = m.computed_summary();
        assert!(m
            .validate()
            .iter()
            .any(|v| matches!(v, DepthClaimManifestViolation::HeldWithActiveGap { .. })));
    }

    #[test]
    fn validate_flags_a_missing_dimension_cell() {
        let mut m = manifest();
        m.rows[0]
            .qualification_matrix
            .retain(|cell| cell.dimension != QualificationDimension::Lineage);
        assert!(m.validate().iter().any(|v| matches!(
            v,
            DepthClaimManifestViolation::QualificationMatrixIncompleteCoverage { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_promotion_decision() {
        let mut m = manifest();
        m.promotion.decision = PromotionDecision::Proceed;
        assert!(m.validate().iter().any(|v| matches!(
            v,
            DepthClaimManifestViolation::PromotionDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn export_projection_mirrors_packets() {
        let m = manifest();
        let projection = m.support_export_projection();
        assert_eq!(projection.rows.len(), m.rows.len());
        for (row, proj) in m.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, proj.entry_id);
            assert_eq!(row.publishes_stable(), proj.publishes_stable);
        }
    }
}
