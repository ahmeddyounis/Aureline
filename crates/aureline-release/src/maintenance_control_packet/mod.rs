//! Typed maintenance-control packet for the release line's hotfix, backport,
//! correction-train, and support-window lanes.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes; the
//! [`stable_proof_index`](crate::stable_proof_index) decides whether each
//! launch-blocking *requirement* is proven; and the
//! [`stable_version_windows`](crate::stable_version_windows) freezes each public
//! interface surface's version window. None of them answer the question this module
//! answers: **for each post-release maintenance lane — an emergency hotfix lane, a
//! supported-line backport lane, a planned correction-train lane, or a support-window
//! commitment — is that lane actually governed, backed by a fresh control packet, a
//! complete and unexpired support window, and an owner sign-off?** This module is the
//! **maintenance-control packet**. For every maintenance lane it records one row that
//! binds the lane to the [`stable_claim_manifest`](crate::stable_claim_manifest) entry
//! whose lifecycle label it backs, the control packet that proves the lane is staffed,
//! the support window the lane commits to, the shared correction-train packet form it
//! rides, the waiver (if any) holding it provisionally, and the owner sign-off.
//!
//! Each [`MaintenanceRow`] is one `(lane, public claim)` binding. It:
//!
//! - names the maintenance lane it governs ([`MaintenanceRow::lane_kind`],
//!   [`MaintenanceRow::lane_ref`], [`MaintenanceRow::lane_summary`]) and whether that
//!   lane is part of the release-blocking maintenance set
//!   ([`MaintenanceRow::release_blocking`]);
//! - pins the support window ([`SupportWindow`]) and the control packet
//!   ([`ProofPacket`]) with its packet-freshness SLO;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose
//!   public claim it backs ([`MaintenanceRow::claim_ref`]) and the canonical lifecycle
//!   label that entry publishes ([`MaintenanceRow::claim_label`]). That label is a hard
//!   **ceiling**: a lane may govern at the claim's label or narrow below it, but it may
//!   never assert a maintenance promise wider than the public claim it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-lane labels, so
//!   docs, Help/About, the release center, and support exports ingest one label per
//!   lane instead of cloning their own;
//! - records the control state earned ([`ControlState`]), the active gap reasons
//!   ([`GapReason`]), and the label it *effectively* governs after narrowing
//!   ([`MaintenanceRow::controlled_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a lane whose control backs a Stable public claim and one narrowed below it.
//! A lane that is not governed — because its control packet aged out or is missing,
//! because its support window is incomplete or has passed its end-of-support date,
//! because its waiver expired, because its lane evidence is incomplete, or because the
//! public claim it backs is itself below the cutline — is structurally required to drop
//! below the cutline rather than inherit an adjacent governed lane. The [`ControlRule`]
//! set names the closed conditions that gate publication, and
//! [`MaintenanceControlPacket::publication`] records the proceed/hold verdict.
//!
//! The packet is checked in at `artifacts/release/maintenance_control_packet.json` and
//! embedded here, so this typed consumer and the CI gate agree on every row without a
//! cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Three
//! classes of check live outside this model because they need more than the packet
//! sees: date arithmetic (recomputing the packet-freshness state, waiver expiry, and
//! support-window-expiry against an `as_of` date) and the cross-artifact ceiling check
//! (whether each row's `claim_label` still equals the label the stable claim manifest
//! publishes for the entry named by `claim_ref`). Those live in the CI gate. This model
//! enforces the structural and logical invariants that hold regardless of the clock and
//! the neighbouring artifact — the ceiling/no-widening rule, support-window ordering,
//! support-window completeness, narrowing consistency, packet/state coherence, owner
//! sign-off on governed rows, lane-kind and release-line coverage, publication-rule
//! wiring, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported maintenance-control packet schema version.
pub const MAINTENANCE_CONTROL_PACKET_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const MAINTENANCE_CONTROL_PACKET_RECORD_KIND: &str = "maintenance_control_packet";

/// Repo-relative path to the checked-in packet.
pub const MAINTENANCE_CONTROL_PACKET_PATH: &str =
    "artifacts/release/maintenance_control_packet.json";

/// Embedded checked-in packet JSON.
pub const MAINTENANCE_CONTROL_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/maintenance_control_packet.json"
));

/// The maintenance lane kind a control row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneKind {
    /// An emergency hotfix lane carrying the smallest viable change set.
    Hotfix,
    /// A supported-line backport lane.
    Backport,
    /// A planned correction-train lane.
    CorrectionTrain,
    /// A support-window commitment for a supported line.
    SupportWindow,
}

impl LaneKind {
    /// Every lane kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Hotfix,
        Self::Backport,
        Self::CorrectionTrain,
        Self::SupportWindow,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hotfix => "hotfix",
            Self::Backport => "backport",
            Self::CorrectionTrain => "correction_train",
            Self::SupportWindow => "support_window",
        }
    }
}

/// The support posture a maintenance lane's support window commits to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportPosture {
    /// Security, correctness, and compatibility fixes are promised.
    FullSupport,
    /// Only security and critical-severity fixes are promised.
    SecurityAndCriticalOnly,
    /// Only security fixes are promised.
    SecurityOnly,
    /// No new fixes are promised; the window is winding down to end-of-life.
    EndOfLifeScheduled,
}

impl SupportPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullSupport,
        Self::SecurityAndCriticalOnly,
        Self::SecurityOnly,
        Self::EndOfLifeScheduled,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSupport => "full_support",
            Self::SecurityAndCriticalOnly => "security_and_critical_only",
            Self::SecurityOnly => "security_only",
            Self::EndOfLifeScheduled => "end_of_life_scheduled",
        }
    }

    /// Whether the support window still claims active support (not wound down to
    /// end-of-life). A wound-down window passing its end-of-support date is expected,
    /// not a defect; an active window passing it is expired.
    pub const fn claims_active_support(self) -> bool {
        !matches!(self, Self::EndOfLifeScheduled)
    }
}

/// Control state a lane earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlState {
    /// The lane is governed: a captured, within-SLO control packet and a complete,
    /// unexpired support window back the public claim at its full canonical lifecycle
    /// label, owner-signed.
    Governed,
    /// The lane governs the claim's full label only because an active, unexpired
    /// waiver covers a recorded control gap.
    GovernedOnWaiver,
    /// The control packet or lane evidence is incomplete, the support window is
    /// incomplete, or owner sign-off is absent; the lane is not governed and the label
    /// must narrow.
    UngovernedUnbacked,
    /// The public claim this lane backs is itself below the cutline, so the control
    /// inherits that ceiling and narrows.
    UngovernedClaimNarrowed,
    /// The control packet breached its freshness SLO (or is missing); the lane is not
    /// governed and the label must narrow.
    UngovernedStale,
    /// The lane relied on a waiver that has expired; the label must narrow.
    UngovernedWaiverExpired,
    /// The support window passed its end-of-support date without renewal; the lane
    /// cannot govern until the window is renewed or formally moved to end-of-life and
    /// the label must narrow.
    UngovernedSupportExpired,
}

impl ControlState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Governed,
        Self::GovernedOnWaiver,
        Self::UngovernedUnbacked,
        Self::UngovernedClaimNarrowed,
        Self::UngovernedStale,
        Self::UngovernedWaiverExpired,
        Self::UngovernedSupportExpired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Governed => "governed",
            Self::GovernedOnWaiver => "governed_on_waiver",
            Self::UngovernedUnbacked => "ungoverned_unbacked",
            Self::UngovernedClaimNarrowed => "ungoverned_claim_narrowed",
            Self::UngovernedStale => "ungoverned_stale",
            Self::UngovernedWaiverExpired => "ungoverned_waiver_expired",
            Self::UngovernedSupportExpired => "ungoverned_support_expired",
        }
    }

    /// Whether the state lets a lane govern the public claim at its label.
    pub const fn holds_control(self) -> bool {
        matches!(self, Self::Governed | Self::GovernedOnWaiver)
    }

    /// Whether the state forces the lane below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_control()
    }
}

/// Closed reason a lane narrows or a control rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The public claim this lane backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The lane names a maintenance capability the build does not yet implement.
    LaneCapabilityAbsent,
    /// The control packet's lane-level evidence (drill, staffing, rollback target) is
    /// incomplete.
    ControlEvidenceIncomplete,
    /// The support window is missing a required date field.
    SupportWindowIncomplete,
    /// The support window passed its end-of-support date without renewal.
    SupportWindowExpired,
    /// The control packet breached its freshness SLO.
    ControlPacketFreshnessBreached,
    /// No control packet has been captured for the lane.
    ControlPacketMissing,
    /// A waiver the lane relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ClaimLabelNarrowed,
        Self::LaneCapabilityAbsent,
        Self::ControlEvidenceIncomplete,
        Self::SupportWindowIncomplete,
        Self::SupportWindowExpired,
        Self::ControlPacketFreshnessBreached,
        Self::ControlPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::LaneCapabilityAbsent => "lane_capability_absent",
            Self::ControlEvidenceIncomplete => "control_evidence_incomplete",
            Self::SupportWindowIncomplete => "support_window_incomplete",
            Self::SupportWindowExpired => "support_window_expired",
            Self::ControlPacketFreshnessBreached => "control_packet_freshness_breached",
            Self::ControlPacketMissing => "control_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a control rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the lane's controlled lifecycle label below the cutline.
    NarrowControlLabel,
    /// Refresh the control packet so it re-enters its freshness SLO.
    RefreshControlPacket,
    /// Complete the support window (open, review-through, and end-of-support dates) or
    /// renew it.
    CompleteSupportWindow,
    /// Recapture the lane-level control evidence the control packet depends on.
    RecaptureControlEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl ControlAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPublication,
        Self::NarrowControlLabel,
        Self::RefreshControlPacket,
        Self::CompleteSupportWindow,
        Self::RecaptureControlEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowControlLabel => "narrow_control_label",
            Self::RefreshControlPacket => "refresh_control_packet",
            Self::CompleteSupportWindow => "complete_support_window",
            Self::RecaptureControlEvidence => "recapture_control_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// The support window a maintenance lane commits to: the open date, the current
/// review-through horizon, the end-of-support date, and the support posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportWindow {
    /// UTC date the support window opened.
    pub opened_at: String,
    /// UTC date the current support commitment is reviewed through.
    pub review_through_date: String,
    /// UTC date support ends for the line.
    pub end_of_support_date: String,
    /// The support posture the window commits to.
    pub support_posture: SupportPosture,
}

impl SupportWindow {
    /// True when every required date field is present, so the window gives a complete
    /// support story (open, review-through, and end-of-support).
    pub fn is_complete(&self) -> bool {
        !self.opened_at.trim().is_empty()
            && !self.review_through_date.trim().is_empty()
            && !self.end_of_support_date.trim().is_empty()
    }

    /// True when `opened_at <= review_through_date <= end_of_support_date` under ISO
    /// date ordering.
    ///
    /// Dates that are not parseable cannot be ordered structurally, so the check passes
    /// for them and the gate is responsible for any registry-specific ordering rule.
    pub fn is_ordered(&self) -> bool {
        match (
            parse_date(&self.opened_at),
            parse_date(&self.review_through_date),
            parse_date(&self.end_of_support_date),
        ) {
            (Some(opened), Some(review), Some(end)) => opened <= review && review <= end,
            _ => true,
        }
    }

    /// True when the window still claims active support (posture is not end-of-life).
    pub const fn claims_active_support(&self) -> bool {
        self.support_posture.claims_active_support()
    }
}

/// One control rule: a closed condition that narrows a lane label and may gate
/// publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: GapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: ControlAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One maintenance-control row: a `(lane, public claim)` binding bound to its support
/// window, control packet, correction-train packet form, canonical ceiling label, and
/// packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintenanceRow {
    /// Stable control-row id.
    pub control_id: String,
    /// Human-readable title.
    pub title: String,
    /// The maintenance lane kind this row governs.
    pub lane_kind: LaneKind,
    /// The maintenance lane id this row governs.
    pub lane_ref: String,
    /// Reviewable one-line statement of the lane.
    pub lane_summary: String,
    /// Whether the lane is part of the release-blocking maintenance set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this control backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a lane
    /// may never govern a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Control state earned for the lane.
    pub control_state: ControlState,
    /// The support window the lane commits to.
    pub support_window: SupportWindow,
    /// The control packet and its freshness SLO.
    pub control_packet: ProofPacket,
    /// Ref to the shared correction-train packet form this lane rides.
    pub correction_packet_ref: String,
    /// Waiver authorizing a provisional control, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// The lifecycle label the control effectively backs after narrowing.
    pub controlled_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl MaintenanceRow {
    /// True when the controlled label is at or above the cutline.
    pub fn governs_stable(&self) -> bool {
        self.controlled_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the lane govern its claimed label.
    pub fn holds_control(&self) -> bool {
        self.control_state.holds_control()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: GapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// The recorded publication verdict for the maintenance-control packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlPublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Control-rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Control-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_lane_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintenanceControlPacketSummary {
    /// Total number of control rows.
    pub total_lanes: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows governing a label at or above the cutline.
    pub lanes_governed_stable: usize,
    /// Rows narrowed below the cutline.
    pub lanes_narrowed_below_cutline: usize,
    /// Rows holding control via an active waiver.
    pub lanes_on_active_waiver: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows governing a label at or above the cutline.
    pub release_blocking_governed_stable: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_ungoverned: usize,
    /// Hotfix lanes.
    pub hotfix_lanes: usize,
    /// Backport lanes.
    pub backport_lanes: usize,
    /// Correction-train lanes.
    pub correction_train_lanes: usize,
    /// Support-window lanes.
    pub support_window_lanes: usize,
    /// Control packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Control packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Control packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Control packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Rows whose state is `ungoverned_support_expired`.
    pub lanes_support_expired: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of control rules currently firing.
    pub control_rules_firing: usize,
}

/// The typed maintenance-control packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintenanceControlPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this packet ingests as its public-claim source
    /// and ceiling.
    pub claim_manifest_ref: String,
    /// Ref to the shared correction-train packet template every correction lane rides.
    pub correction_train_template_ref: String,
    /// Ref to the support-window contract every support window rides.
    pub support_window_contract_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed lane-kind vocabulary.
    pub lane_kinds: Vec<LaneKind>,
    /// Closed support-posture vocabulary.
    pub support_postures: Vec<SupportPosture>,
    /// Closed control-state vocabulary.
    pub control_states: Vec<ControlState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Closed control-action vocabulary.
    pub control_actions: Vec<ControlAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking lane refs this packet must cover.
    pub release_blocking_lane_refs: Vec<String>,
    /// Control rules.
    pub control_rules: Vec<ControlRule>,
    /// Control rows.
    pub rows: Vec<MaintenanceRow>,
    /// Recorded publication verdict.
    pub publication: ControlPublicationRecord,
    /// Summary counts.
    pub summary: MaintenanceControlPacketSummary,
}

impl MaintenanceControlPacket {
    /// Returns the row registered for `control_id`.
    pub fn row(&self, control_id: &str) -> Option<&MaintenanceRow> {
        self.rows.iter().find(|row| row.control_id == control_id)
    }

    /// Returns the rows governing a label at or above the cutline.
    pub fn rows_governed_stable(&self) -> Vec<&MaintenanceRow> {
        self.rows
            .iter()
            .filter(|row| row.governs_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&MaintenanceRow> {
        self.rows
            .iter()
            .filter(|row| !row.governs_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&MaintenanceRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one lane kind.
    pub fn rows_for_kind(&self, kind: LaneKind) -> Vec<&MaintenanceRow> {
        self.rows
            .iter()
            .filter(|row| row.lane_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the packet covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn control_rule_fires(&self, rule: &ControlRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and control rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .control_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.control_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .control_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.control_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Control-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose claim
    /// is already canonically narrowed is not a *control* blocker, it merely inherits
    /// the upstream ceiling.
    pub fn computed_blocking_lane_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<GapReason> = self
            .control_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.control_rule_fires(rule))
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
                ids.insert(row.control_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and control rules.
    pub fn computed_summary(&self) -> MaintenanceControlPacketSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.control_packet.slo_state == state)
                .count()
        };
        let kind = |kind: LaneKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&MaintenanceRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        MaintenanceControlPacketSummary {
            total_lanes: self.rows.len(),
            total_claims: self.claims().len(),
            lanes_governed_stable: self.rows.iter().filter(|row| row.governs_stable()).count(),
            lanes_narrowed_below_cutline: self
                .rows
                .iter()
                .filter(|row| !row.governs_stable())
                .count(),
            lanes_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.control_state == ControlState::GovernedOnWaiver)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_governed_stable: release_blocking
                .iter()
                .filter(|row| row.governs_stable())
                .count(),
            release_blocking_ungoverned: release_blocking
                .iter()
                .filter(|row| !row.governs_stable())
                .count(),
            hotfix_lanes: kind(LaneKind::Hotfix),
            backport_lanes: kind(LaneKind::Backport),
            correction_train_lanes: kind(LaneKind::CorrectionTrain),
            support_window_lanes: kind(LaneKind::SupportWindow),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            lanes_support_expired: self
                .rows
                .iter()
                .filter(|row| row.control_state == ControlState::UngovernedSupportExpired)
                .count(),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            control_rules_firing: self
                .control_rules
                .iter()
                .filter(|rule| self.control_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the packet that downstream
    /// surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> MaintenanceExportProjection {
        MaintenanceExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| MaintenanceExportRow {
                    control_id: row.control_id.clone(),
                    lane_kind: row.lane_kind,
                    lane_ref: row.lane_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    controlled_label: row.controlled_label,
                    governs_stable: row.governs_stable(),
                    control_state: row.control_state,
                    support_posture: row.support_window.support_posture,
                    end_of_support_date: row.support_window.end_of_support_date.clone(),
                    slo_state: row.control_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<MaintenanceControlPacketViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.control_id.clone()) {
                violations.push(MaintenanceControlPacketViolation::DuplicateControlId {
                    control_id: row.control_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(MaintenanceControlPacketViolation::EmptyPacket);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(MaintenanceControlPacketViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<MaintenanceControlPacketViolation>) {
        if self.schema_version != MAINTENANCE_CONTROL_PACKET_SCHEMA_VERSION {
            violations.push(
                MaintenanceControlPacketViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != MAINTENANCE_CONTROL_PACKET_RECORD_KIND {
            violations.push(MaintenanceControlPacketViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            (
                "correction_train_template_ref",
                &self.correction_train_template_ref,
            ),
            (
                "support_window_contract_ref",
                &self.support_window_contract_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(MaintenanceControlPacketViolation::EmptyField {
                    control_id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(
                MaintenanceControlPacketViolation::ClosedVocabularyMismatch {
                    field: "lifecycle_labels",
                },
            );
        }
        if self.lane_kinds != LaneKind::ALL.to_vec() {
            violations.push(
                MaintenanceControlPacketViolation::ClosedVocabularyMismatch {
                    field: "lane_kinds",
                },
            );
        }
        if self.support_postures != SupportPosture::ALL.to_vec() {
            violations.push(
                MaintenanceControlPacketViolation::ClosedVocabularyMismatch {
                    field: "support_postures",
                },
            );
        }
        if self.control_states != ControlState::ALL.to_vec() {
            violations.push(
                MaintenanceControlPacketViolation::ClosedVocabularyMismatch {
                    field: "control_states",
                },
            );
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(
                MaintenanceControlPacketViolation::ClosedVocabularyMismatch {
                    field: "gap_reasons",
                },
            );
        }
        if self.control_actions != ControlAction::ALL.to_vec() {
            violations.push(
                MaintenanceControlPacketViolation::ClosedVocabularyMismatch {
                    field: "control_actions",
                },
            );
        }
        if self.release_blocking_lane_refs.is_empty() {
            violations.push(MaintenanceControlPacketViolation::EmptyField {
                control_id: "<packet>".to_owned(),
                field_name: "release_blocking_lane_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(
                MaintenanceControlPacketViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.cutline_level",
                },
            );
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(
                MaintenanceControlPacketViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.above_cutline_levels",
                },
            );
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(
                MaintenanceControlPacketViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.below_cutline_levels",
                },
            );
        }
        if cutline.description.trim().is_empty() {
            violations.push(MaintenanceControlPacketViolation::EmptyField {
                control_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<MaintenanceControlPacketViolation>) {
        if self.control_rules.is_empty() {
            violations.push(MaintenanceControlPacketViolation::NoControlRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.control_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(MaintenanceControlPacketViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(MaintenanceControlPacketViolation::EmptyField {
                        control_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(MaintenanceControlPacketViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every gap reason must have a rule, so a gap reason cannot fire without a
        // corresponding publication gate.
        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(MaintenanceControlPacketViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &MaintenanceRow,
        violations: &mut Vec<MaintenanceControlPacketViolation>,
    ) {
        for (field, value) in [
            ("control_id", &row.control_id),
            ("title", &row.title),
            ("lane_ref", &row.lane_ref),
            ("lane_summary", &row.lane_summary),
            ("claim_ref", &row.claim_ref),
            ("correction_packet_ref", &row.correction_packet_ref),
            ("rationale", &row.rationale),
            ("support_window.opened_at", &row.support_window.opened_at),
            (
                "support_window.end_of_support_date",
                &row.support_window.end_of_support_date,
            ),
            ("control_packet.packet_id", &row.control_packet.packet_id),
            ("control_packet.packet_ref", &row.control_packet.packet_ref),
            (
                "control_packet.proof_index_ref",
                &row.control_packet.proof_index_ref,
            ),
            (
                "control_packet.freshness_slo.slo_register_ref",
                &row.control_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(MaintenanceControlPacketViolation::EmptyField {
                    control_id: row.control_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no control may back a label wider than the public claim's
        // canonical label.
        if row.controlled_label.rank() > row.claim_label.rank() {
            violations.push(
                MaintenanceControlPacketViolation::ControlledWiderThanClaim {
                    control_id: row.control_id.clone(),
                    claim: row.claim_label,
                    controlled: row.controlled_label,
                },
            );
        }

        // The support window must be ordered opened <= review_through <= end_of_support.
        if !row.support_window.is_ordered() {
            violations.push(MaintenanceControlPacketViolation::SupportWindowDisordered {
                control_id: row.control_id.clone(),
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.control_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(MaintenanceControlPacketViolation::EmptyField {
                control_id: row.control_id.clone(),
                field_name: "control_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.control_packet.freshness_slo.window_is_consistent() {
            violations.push(
                MaintenanceControlPacketViolation::FreshnessSloInconsistent {
                    control_id: row.control_id.clone(),
                },
            );
        }

        self.validate_support_window(row, violations);

        // A public claim whose canonical label is below the cutline forces the control
        // to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_control() {
                violations.push(MaintenanceControlPacketViolation::HeldOnNarrowedClaim {
                    control_id: row.control_id.clone(),
                    claim: row.claim_label,
                });
            }
            if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                violations.push(
                    MaintenanceControlPacketViolation::ClaimNarrowedWithoutReason {
                        control_id: row.control_id.clone(),
                    },
                );
            }
        }

        let slo_state = row.control_packet.slo_state;

        if row.holds_control() {
            // A governed row backs exactly the public claim's canonical label, carries
            // no active gap reason, rides a captured within-SLO packet, has a complete
            // support window, and is owner-signed.
            if row.controlled_label != row.claim_label {
                violations.push(MaintenanceControlPacketViolation::HeldLabelNotEqualClaim {
                    control_id: row.control_id.clone(),
                    claim: row.claim_label,
                    controlled: row.controlled_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(MaintenanceControlPacketViolation::HeldWithActiveGap {
                    control_id: row.control_id.clone(),
                });
            }
            if !row.control_packet.has_capture() {
                violations.push(MaintenanceControlPacketViolation::HeldWithoutFreshPacket {
                    control_id: row.control_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(MaintenanceControlPacketViolation::HeldOnStalePacket {
                    control_id: row.control_id.clone(),
                    slo_state,
                });
            }
            if !row.support_window.is_complete() {
                violations.push(
                    MaintenanceControlPacketViolation::HeldWithIncompleteSupportWindow {
                        control_id: row.control_id.clone(),
                    },
                );
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(MaintenanceControlPacketViolation::HeldWithoutSignoff {
                    control_id: row.control_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the controlled label below the cutline and
            // name at least one active reason.
            if row.governs_stable() {
                violations.push(
                    MaintenanceControlPacketViolation::ControlledLabelNotNarrowed {
                        control_id: row.control_id.clone(),
                        state: row.control_state,
                        controlled: row.controlled_label,
                    },
                );
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(MaintenanceControlPacketViolation::NarrowingWithoutReason {
                    control_id: row.control_id.clone(),
                    state: row.control_state,
                });
            }
            // A narrowing row whose packet is breached or missing must name the matching
            // freshness reason, so the freshness automation stays honest.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(GapReason::ControlPacketFreshnessBreached)
            {
                violations.push(
                    MaintenanceControlPacketViolation::BreachedPacketWithoutReason {
                        control_id: row.control_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(GapReason::ControlPacketMissing)
            {
                violations.push(
                    MaintenanceControlPacketViolation::MissingPacketWithoutReason {
                        control_id: row.control_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_support_window(
        &self,
        row: &MaintenanceRow,
        violations: &mut Vec<MaintenanceControlPacketViolation>,
    ) {
        let incomplete = !row.support_window.is_complete();
        // A row carrying the incomplete-support-window reason must actually have an
        // incomplete window, and a row with an incomplete window must name the reason
        // (so the support-window-completeness automation stays honest).
        if row.has_active_reason(GapReason::SupportWindowIncomplete) && !incomplete {
            violations.push(
                MaintenanceControlPacketViolation::SupportWindowReasonWithoutIncomplete {
                    control_id: row.control_id.clone(),
                },
            );
        }
        if incomplete && !row.has_active_reason(GapReason::SupportWindowIncomplete) {
            violations.push(
                MaintenanceControlPacketViolation::IncompleteSupportWindowWithoutReason {
                    control_id: row.control_id.clone(),
                },
            );
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &MaintenanceRow,
        violations: &mut Vec<MaintenanceControlPacketViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<MaintenanceControlPacketViolation>,
                               expected: GapReason| {
            violations.push(MaintenanceControlPacketViolation::StateReasonIncoherent {
                control_id: row.control_id.clone(),
                state: row.control_state,
                expected_reason: expected,
            });
        };

        match row.control_state {
            ControlState::UngovernedUnbacked => {
                const ALLOWED: [GapReason; 4] = [
                    GapReason::LaneCapabilityAbsent,
                    GapReason::ControlEvidenceIncomplete,
                    GapReason::SupportWindowIncomplete,
                    GapReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| row.has_active_reason(*r)) {
                    push_incoherent(violations, GapReason::ControlEvidenceIncomplete);
                }
            }
            ControlState::UngovernedClaimNarrowed => {
                if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, GapReason::ClaimLabelNarrowed);
                }
            }
            ControlState::UngovernedStale => {
                if !(row.has_active_reason(GapReason::ControlPacketFreshnessBreached)
                    || row.has_active_reason(GapReason::ControlPacketMissing))
                {
                    push_incoherent(violations, GapReason::ControlPacketFreshnessBreached);
                }
            }
            ControlState::UngovernedWaiverExpired => {
                if !row.has_active_reason(GapReason::WaiverExpired) {
                    push_incoherent(violations, GapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(
                        MaintenanceControlPacketViolation::WaiverStateWithoutWaiver {
                            control_id: row.control_id.clone(),
                            state: row.control_state,
                        },
                    );
                }
            }
            ControlState::UngovernedSupportExpired => {
                if !row.has_active_reason(GapReason::SupportWindowExpired) {
                    push_incoherent(violations, GapReason::SupportWindowExpired);
                }
                // A support window can only be *expired* if it still claims active
                // support; a window formally moved to end-of-life is expected to pass
                // its end-of-support date.
                if !row.support_window.claims_active_support() {
                    violations.push(
                        MaintenanceControlPacketViolation::ExpiredStateOnEndOfLifeWindow {
                            control_id: row.control_id.clone(),
                        },
                    );
                }
            }
            ControlState::GovernedOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(
                        MaintenanceControlPacketViolation::WaiverStateWithoutWaiver {
                            control_id: row.control_id.clone(),
                            state: row.control_state,
                        },
                    );
                }
            }
            ControlState::Governed => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<MaintenanceControlPacketViolation>) {
        // Each lane ref appears at most once: a lane has one canonical control row.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.lane_ref.as_str()) {
                violations.push(MaintenanceControlPacketViolation::DuplicateLaneRef {
                    lane_ref: row.lane_ref.clone(),
                });
            }
        }

        // The release line must govern every declared release-blocking lane with
        // exactly one release-blocking row, and every release-blocking row must be
        // declared, so a lane cannot quietly drop out of the packet.
        let declared: BTreeSet<&str> = self
            .release_blocking_lane_refs
            .iter()
            .map(String::as_str)
            .collect();
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.lane_ref.as_str())
            .collect();
        for declared_ref in &declared {
            if !covered.contains(declared_ref) {
                violations.push(
                    MaintenanceControlPacketViolation::ReleaseBlockingRefWithoutRow {
                        lane_ref: (*declared_ref).to_owned(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !declared.contains(row.lane_ref.as_str()) {
                violations.push(
                    MaintenanceControlPacketViolation::ReleaseBlockingRowNotInSet {
                        control_id: row.control_id.clone(),
                        lane_ref: row.lane_ref.clone(),
                    },
                );
            }
        }

        // The packet must cover all four lane kinds — hotfix, backport, correction
        // train, and support window — so the release line cannot govern some lanes and
        // silently leave a whole maintenance kind ungoverned.
        for kind in LaneKind::ALL {
            if self.rows_for_kind(kind).is_empty() {
                violations.push(MaintenanceControlPacketViolation::LaneKindAbsent { kind });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<MaintenanceControlPacketViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(MaintenanceControlPacketViolation::EmptyField {
                control_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(MaintenanceControlPacketViolation::EmptyField {
                control_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                MaintenanceControlPacketViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                MaintenanceControlPacketViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_lane_ids != self.computed_blocking_lane_ids() {
            violations.push(
                MaintenanceControlPacketViolation::PublicationBlockingSetMismatch {
                    field: "blocking_lane_ids",
                },
            );
        }
    }
}

/// Parses an ISO `YYYY-MM-DD` date into comparable components.
///
/// Returns `None` when the value is empty or not a three-part dotted/dashed numeric
/// date, so callers can fall back to a registry-specific rule.
fn parse_date(value: &str) -> Option<(u64, u64, u64)> {
    if value.trim().is_empty() {
        return None;
    }
    let mut parts = value.split('-');
    let year = parts.next()?.parse::<u64>().ok()?;
    let month = parts.next()?.parse::<u64>().ok()?;
    let day = parts.next()?.parse::<u64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((year, month, day))
}

/// A redaction-safe export row projected from the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceExportRow {
    /// Stable control-row id.
    pub control_id: String,
    /// Maintenance lane kind.
    pub lane_kind: LaneKind,
    /// Maintenance lane ref.
    pub lane_ref: String,
    /// Whether the lane is part of the release-blocking maintenance set.
    pub release_blocking: bool,
    /// The public-claim entry ref the control backs.
    pub claim_ref: String,
    /// The public claim's canonical ceiling label.
    pub claim_label: StableClaimLevel,
    /// Lifecycle label the control backs.
    pub controlled_label: StableClaimLevel,
    /// Whether the row governs a label at or above the cutline.
    pub governs_stable: bool,
    /// Control state.
    pub control_state: ControlState,
    /// Support posture of the window.
    pub support_posture: SupportPosture,
    /// End-of-support date of the window.
    pub end_of_support_date: String,
    /// Control-packet freshness-SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<GapReason>,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<MaintenanceExportRow>,
}

/// A validation violation for the maintenance-control packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaintenanceControlPacketViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The packet has no rows.
    EmptyPacket,
    /// The packet has no control rules.
    NoControlRules,
    /// A required field is empty.
    EmptyField {
        /// Row, rule, or section id.
        control_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A control id appears more than once.
    DuplicateControlId {
        /// Duplicate control id.
        control_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A control rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: GapReason,
    },
    /// A controlled label is wider than the public claim's canonical label.
    ControlledWiderThanClaim {
        /// Row id.
        control_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
        /// Controlled label.
        controlled: StableClaimLevel,
    },
    /// A support window is not ordered opened <= review_through <= end_of_support.
    SupportWindowDisordered {
        /// Row id.
        control_id: String,
    },
    /// A freshness SLO's warn window exceeds its target age.
    FreshnessSloInconsistent {
        /// Row id.
        control_id: String,
    },
    /// A row carries the incomplete-support-window reason but the window is complete.
    SupportWindowReasonWithoutIncomplete {
        /// Row id.
        control_id: String,
    },
    /// A row has an incomplete support window but does not name the reason.
    IncompleteSupportWindowWithoutReason {
        /// Row id.
        control_id: String,
    },
    /// A support-expired state names an end-of-life window (expected to pass its date).
    ExpiredStateOnEndOfLifeWindow {
        /// Row id.
        control_id: String,
    },
    /// A row holds control while the public claim's canonical label is narrowed.
    HeldOnNarrowedClaim {
        /// Row id.
        control_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
    },
    /// A row whose claim is narrowed does not carry the claim-narrowed reason.
    ClaimNarrowedWithoutReason {
        /// Row id.
        control_id: String,
    },
    /// A narrowing state did not drop the controlled label below the cutline.
    ControlledLabelNotNarrowed {
        /// Row id.
        control_id: String,
        /// Control state.
        state: ControlState,
        /// Controlled label.
        controlled: StableClaimLevel,
    },
    /// A narrowing state carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        control_id: String,
        /// Control state.
        state: ControlState,
    },
    /// A governed row's controlled label is not equal to its claim ceiling label.
    HeldLabelNotEqualClaim {
        /// Row id.
        control_id: String,
        /// Claim ceiling label.
        claim: StableClaimLevel,
        /// Controlled label.
        controlled: StableClaimLevel,
    },
    /// A governed row carries an active gap reason.
    HeldWithActiveGap {
        /// Row id.
        control_id: String,
    },
    /// A governed row rides a control packet with no capture or evidence.
    HeldWithoutFreshPacket {
        /// Row id.
        control_id: String,
    },
    /// A governed row rides a control packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        control_id: String,
        /// The packet's freshness-SLO state.
        slo_state: FreshnessSloState,
    },
    /// A governed row carries an incomplete support window.
    HeldWithIncompleteSupportWindow {
        /// Row id.
        control_id: String,
    },
    /// A governed row has no owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        control_id: String,
    },
    /// A narrowing row with a breached packet does not name the breach reason.
    BreachedPacketWithoutReason {
        /// Row id.
        control_id: String,
    },
    /// A narrowing row with a missing packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Row id.
        control_id: String,
    },
    /// A control state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        control_id: String,
        /// Control state.
        state: ControlState,
        /// Reason the state requires.
        expected_reason: GapReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        control_id: String,
        /// Control state.
        state: ControlState,
    },
    /// A lane ref appears on more than one row.
    DuplicateLaneRef {
        /// Duplicate lane ref.
        lane_ref: String,
    },
    /// A declared release-blocking lane ref has no covering row.
    ReleaseBlockingRefWithoutRow {
        /// Uncovered lane ref.
        lane_ref: String,
    },
    /// A release-blocking row's lane ref is not in the declared set.
    ReleaseBlockingRowNotInSet {
        /// Row id.
        control_id: String,
        /// The row's lane ref.
        lane_ref: String,
    },
    /// A lane kind is not covered by any row.
    LaneKindAbsent {
        /// The uncovered lane kind.
        kind: LaneKind,
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
}

impl fmt::Display for MaintenanceControlPacketViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyPacket => write!(f, "packet has no rows"),
            Self::NoControlRules => write!(f, "packet has no control rules"),
            Self::EmptyField {
                control_id,
                field_name,
            } => write!(f, "{control_id} has empty field {field_name}"),
            Self::DuplicateControlId { control_id } => {
                write!(f, "duplicate control row id {control_id}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate control rule id {rule_id}")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "control rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutRule { reason } => write!(
                f,
                "gap reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::ControlledWiderThanClaim {
                control_id,
                claim,
                controlled,
            } => write!(
                f,
                "lane {control_id} controlled label {} is wider than the claim ceiling {}",
                controlled.as_str(),
                claim.as_str()
            ),
            Self::SupportWindowDisordered { control_id } => write!(
                f,
                "lane {control_id} support window is not ordered opened <= review_through <= end_of_support"
            ),
            Self::FreshnessSloInconsistent { control_id } => write!(
                f,
                "lane {control_id} freshness SLO warn window exceeds its target age"
            ),
            Self::SupportWindowReasonWithoutIncomplete { control_id } => write!(
                f,
                "lane {control_id} names support_window_incomplete but the window is complete"
            ),
            Self::IncompleteSupportWindowWithoutReason { control_id } => write!(
                f,
                "lane {control_id} has an incomplete support window but does not name support_window_incomplete"
            ),
            Self::ExpiredStateOnEndOfLifeWindow { control_id } => write!(
                f,
                "lane {control_id} is support-expired but its window is formally end-of-life"
            ),
            Self::HeldOnNarrowedClaim { control_id, claim } => write!(
                f,
                "lane {control_id} holds control while the public claim label {} is below the cutline",
                claim.as_str()
            ),
            Self::ClaimNarrowedWithoutReason { control_id } => write!(
                f,
                "lane {control_id} backs a claim that is narrowed but does not name claim_label_narrowed"
            ),
            Self::ControlledLabelNotNarrowed {
                control_id,
                state,
                controlled,
            } => write!(
                f,
                "lane {control_id} state {} must narrow below the cutline but governs {}",
                state.as_str(),
                controlled.as_str()
            ),
            Self::NarrowingWithoutReason { control_id, state } => write!(
                f,
                "lane {control_id} state {} narrows without naming an active gap reason",
                state.as_str()
            ),
            Self::HeldLabelNotEqualClaim {
                control_id,
                claim,
                controlled,
            } => write!(
                f,
                "lane {control_id} governs {} but its public claim label is {}",
                controlled.as_str(),
                claim.as_str()
            ),
            Self::HeldWithActiveGap { control_id } => write!(
                f,
                "lane {control_id} governs its label while a gap reason is active"
            ),
            Self::HeldWithoutFreshPacket { control_id } => write!(
                f,
                "lane {control_id} governs its label with no captured, evidence-backed control packet"
            ),
            Self::HeldOnStalePacket {
                control_id,
                slo_state,
            } => write!(
                f,
                "lane {control_id} governs its label while its packet is {} (outside its freshness SLO)",
                slo_state.as_str()
            ),
            Self::HeldWithIncompleteSupportWindow { control_id } => write!(
                f,
                "lane {control_id} governs its label with an incomplete support window"
            ),
            Self::HeldWithoutSignoff { control_id } => {
                write!(f, "lane {control_id} governs its label without owner sign-off")
            }
            Self::BreachedPacketWithoutReason { control_id } => write!(
                f,
                "lane {control_id} has a breached packet but does not name control_packet_freshness_breached"
            ),
            Self::MissingPacketWithoutReason { control_id } => write!(
                f,
                "lane {control_id} has a missing packet but does not name control_packet_missing"
            ),
            Self::StateReasonIncoherent {
                control_id,
                state,
                expected_reason,
            } => write!(
                f,
                "lane {control_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { control_id, state } => write!(
                f,
                "lane {control_id} state {} names no waiver",
                state.as_str()
            ),
            Self::DuplicateLaneRef { lane_ref } => {
                write!(f, "duplicate lane ref {lane_ref}")
            }
            Self::ReleaseBlockingRefWithoutRow { lane_ref } => write!(
                f,
                "declared release-blocking lane {lane_ref} has no covering row"
            ),
            Self::ReleaseBlockingRowNotInSet {
                control_id,
                lane_ref,
            } => write!(
                f,
                "lane {control_id} is release-blocking but its lane {lane_ref} is not in release_blocking_lane_refs"
            ),
            Self::LaneKindAbsent { kind } => write!(
                f,
                "lane kind {} is not covered by any control row",
                kind.as_str()
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "publication decision {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with the firing rules")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the rows"),
        }
    }
}

impl Error for MaintenanceControlPacketViolation {}

/// Loads the embedded maintenance-control packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`MaintenanceControlPacket`] — including when a row carries a lifecycle label, lane
/// kind, support posture, control state, freshness-SLO state, gap reason, or control
/// action outside the closed vocabularies.
pub fn current_maintenance_control_packet() -> Result<MaintenanceControlPacket, serde_json::Error> {
    serde_json::from_str(MAINTENANCE_CONTROL_PACKET_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packet() -> MaintenanceControlPacket {
        current_maintenance_control_packet().expect("packet parses")
    }

    #[test]
    fn embedded_packet_parses_and_validates() {
        let packet = packet();
        assert_eq!(
            packet.schema_version,
            MAINTENANCE_CONTROL_PACKET_SCHEMA_VERSION
        );
        assert_eq!(packet.record_kind, MAINTENANCE_CONTROL_PACKET_RECORD_KIND);
        assert_eq!(packet.validate(), Vec::new());
        assert!(!packet.rows.is_empty());
    }

    #[test]
    fn every_lane_kind_is_covered() {
        let packet = packet();
        for kind in LaneKind::ALL {
            assert!(
                !packet.rows_for_kind(kind).is_empty(),
                "lane kind {} must have at least one control row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn every_release_blocking_lane_is_covered() {
        let packet = packet();
        let covered: BTreeSet<&str> = packet
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.lane_ref.as_str())
            .collect();
        assert!(!packet.release_blocking_lane_refs.is_empty());
        for declared in &packet.release_blocking_lane_refs {
            assert!(
                covered.contains(declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn packet_exercises_governed_and_narrowed_rows() {
        let packet = packet();
        assert!(
            !packet.rows_governed_stable().is_empty(),
            "packet must show at least one governed-stable lane"
        );
        assert!(
            !packet.rows_narrowed().is_empty(),
            "packet must show at least one narrowed lane"
        );
    }

    #[test]
    fn summary_counts_match_rows() {
        let packet = packet();
        assert_eq!(packet.summary, packet.computed_summary());
        assert_eq!(
            packet.summary.lanes_governed_stable + packet.summary.lanes_narrowed_below_cutline,
            packet.rows.len()
        );
        assert_eq!(
            packet.summary.packets_current
                + packet.summary.packets_due_for_refresh
                + packet.summary.packets_breached
                + packet.summary.packets_missing,
            packet.rows.len()
        );
        assert_eq!(
            packet.summary.hotfix_lanes
                + packet.summary.backport_lanes
                + packet.summary.correction_train_lanes
                + packet.summary.support_window_lanes,
            packet.rows.len()
        );
    }

    #[test]
    fn publication_holds_when_a_blocking_rule_fires() {
        let packet = packet();
        assert_eq!(packet.publication.decision, PromotionDecision::Hold);
        assert_eq!(
            packet.publication.decision,
            packet.computed_publication_decision()
        );
        assert!(!packet.publication.blocking_rule_ids.is_empty());
        assert!(!packet.publication.blocking_lane_ids.is_empty());
    }

    #[test]
    fn every_gap_reason_has_a_rule() {
        let packet = packet();
        let covered: BTreeSet<GapReason> = packet
            .control_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in GapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_row_governs_wider_than_its_claim_ceiling() {
        let packet = packet();
        for row in &packet.rows {
            assert!(
                row.controlled_label.rank() <= row.claim_label.rank(),
                "{} governs wider than its ceiling",
                row.control_id
            );
        }
    }

    #[test]
    fn every_support_window_is_ordered() {
        let packet = packet();
        for row in &packet.rows {
            assert!(
                row.support_window.is_ordered(),
                "{} support window is disordered",
                row.control_id
            );
        }
    }

    #[test]
    fn validate_flags_a_control_wider_than_ceiling() {
        let mut packet = packet();
        let row = packet
            .rows
            .iter_mut()
            .find(|row| !row.governs_stable() && row.claim_label == StableClaimLevel::Beta)
            .expect("a narrowed row under a beta ceiling exists");
        row.controlled_label = StableClaimLevel::Stable;
        let control_id = row.control_id.clone();
        packet.summary = packet.computed_summary();
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            MaintenanceControlPacketViolation::ControlledWiderThanClaim { control_id: id, .. } if *id == control_id
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut packet = packet();
        let row = packet
            .rows
            .iter_mut()
            .find(|row| row.control_state == ControlState::UngovernedStale)
            .expect("an ungoverned-stale row exists");
        row.controlled_label = row.claim_label;
        packet.summary = packet.computed_summary();
        packet.publication.decision = packet.computed_publication_decision();
        packet.publication.blocking_rule_ids = packet.computed_blocking_rule_ids();
        packet.publication.blocking_lane_ids = packet.computed_blocking_lane_ids();
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            MaintenanceControlPacketViolation::ControlledLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_a_disordered_support_window() {
        let mut packet = packet();
        let row = packet.rows.first_mut().expect("packet has a row");
        row.support_window.opened_at = "2099-01-01".to_owned();
        let control_id = row.control_id.clone();
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            MaintenanceControlPacketViolation::SupportWindowDisordered { control_id: id } if *id == control_id
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut packet = packet();
        packet.publication.decision = PromotionDecision::Proceed;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            MaintenanceControlPacketViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_governed_row_without_signoff() {
        let mut packet = packet();
        let row = packet
            .rows
            .iter_mut()
            .find(|row| row.holds_control())
            .expect("a governed row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        let control_id = row.control_id.clone();
        packet.summary = packet.computed_summary();
        assert!(packet
            .validate()
            .contains(&MaintenanceControlPacketViolation::HeldWithoutSignoff { control_id }));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let packet = packet();
        let projection = packet.support_export_projection();
        assert_eq!(projection.rows.len(), packet.rows.len());
        assert_eq!(projection.publication_decision, packet.publication.decision);
        for (row, projected) in packet.rows.iter().zip(&projection.rows) {
            assert_eq!(row.control_id, projected.control_id);
            assert_eq!(row.lane_ref, projected.lane_ref);
            assert_eq!(row.governs_stable(), projected.governs_stable);
            assert_eq!(row.controlled_label, projected.controlled_label);
            assert_eq!(row.control_packet.slo_state, projected.slo_state);
        }
    }
}
