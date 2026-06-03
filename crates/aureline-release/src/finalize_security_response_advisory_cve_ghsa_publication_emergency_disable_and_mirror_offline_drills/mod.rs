//! Typed security-response, advisory, CVE/GHSA publication, emergency disable,
//! and mirror/offline drill packet for the stable launch line.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes; the
//! [`stable_proof_index`](crate::stable_proof_index) decides whether each
//! launch-blocking *requirement* is proven; the
//! [`stable_version_windows`](crate::stable_version_windows) freezes each public
//! interface surface's version window; and the
//! [`maintenance_control_packet`](crate::maintenance_control_packet) governs the
//! post-release maintenance lanes. None of them answer the question this module
//! answers: **for each security-response lane — the core security response
//! process, security advisory publication, CVE/GHSA publication, emergency
//! disable capability, and mirror/offline publication drills — is that lane
//! actually ready, backed by a fresh response packet, with its required
//! emergency controls satisfied or its mirror drills verified, and an owner
//! sign-off, and is it narrowed below the cutline the moment its backing thins
//! out?** This module is the **security-response packet**. For every response
//! lane it records one row that binds the lane to the
//! [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose lifecycle
//! label it backs, the response packet that grounds it, the emergency controls
//! or mirror drill checkpoints it must satisfy, the waiver (if any) holding it
//! provisionally, and the owner sign-off.
//!
//! Each [`ResponseRow`] is one `(response lane, public claim)` binding. It:
//!
//! - names the response lane it governs ([`ResponseRow::response_kind`],
//!   [`ResponseRow::lane_ref`], [`ResponseRow::lane_summary`]) and whether that
//!   lane is part of the release-blocking response set
//!   ([`ResponseRow::release_blocking`]);
//! - pins the response packet ([`ProofPacket`]) with its packet-freshness SLO;
//! - for emergency-disable lanes, pins the [`EmergencyControl`] set whose every
//!   member must be satisfied for the lane to return Ready;
//! - for mirror/offline-drill lanes, pins the [`MirrorDrillCheckpoint`] set
//!   whose every member must be verified for the lane to return Ready;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry
//!   whose public claim it backs ([`ResponseRow::claim_ref`]) and the canonical
//!   lifecycle label that entry publishes ([`ResponseRow::claim_label`]). That
//!   label is a hard **ceiling**: a lane may carry the claim's label or narrow
//!   below it, but it may never assert a public claim wider than the public claim
//!   it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-response
//!   labels, so docs, Help/About, the release center, and support exports ingest
//!   one label per lane instead of cloning their own;
//! - records the response state earned ([`ResponseState`]), the active gap
//!   reasons ([`GapReason`]), and the label it *effectively* publishes after
//!   narrowing ([`ResponseRow::effective_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between a lane whose backing supports a Stable public claim and one
//! narrowed below it. A lane that is not ready — because its response packet aged
//! out or is missing, because a required emergency control is unsatisfied, because
//! a required mirror drill checkpoint is unverified, because its evidence is
//! incomplete, because its owner sign-off is missing, because its waiver expired,
//! or because the public claim it backs is itself below the cutline — is
//! structurally required to drop below the cutline rather than inherit an
//! adjacent ready lane. The [`ResponseRule`] set names the closed conditions that
//! gate publication, and [`SecurityResponsePacket::publication`] records the
//! proceed/hold verdict.
//!
//! The packet is checked in at
//! `artifacts/release/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state, a boolean control
//! flag, or an opaque ref. It carries no raw artifacts, raw logs, signatures, or
//! credential material. Two classes of check live outside this model because they
//! need more than the packet sees: date arithmetic (recomputing the
//! packet-freshness state and waiver expiry against an `as_of` date) and the
//! cross-artifact ceiling check (whether each row's `claim_label` still equals
//! the label the stable claim manifest publishes for the entry named by
//! `claim_ref`). Those live in the CI gate. This model enforces the structural
//! and logical invariants that hold regardless of the clock and the neighbouring
//! artifact — the ceiling/no-widening rule, emergency-control completeness,
//! mirror-drill-checkpoint completeness, narrowing consistency, packet/state
//! coherence, owner sign-off on ready lanes, response-kind and release-line
//! coverage, response-rule wiring, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported security-response packet schema version.
pub const SECURITY_RESPONSE_PACKET_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const SECURITY_RESPONSE_PACKET_RECORD_KIND: &str = "security_response_packet";

/// Repo-relative path to the checked-in packet.
pub const SECURITY_RESPONSE_PACKET_PATH: &str =
    "artifacts/release/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills.json";

/// Embedded checked-in packet JSON.
pub const SECURITY_RESPONSE_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills.json"
));

/// The response lane kind a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseKind {
    /// The core security response process: triage, incident declaration,
    /// evidence preservation, and coordination.
    SecurityResponse,
    /// Security advisory publication: drafting, review, and release of
    /// customer-facing security advisories.
    AdvisoryPublication,
    /// CVE/GHSA publication: assignment and publication to external
    /// vulnerability databases.
    CveGhsaPublication,
    /// Emergency disable capability: kill-switch, feature flag disable,
    /// and rapid-response circuit breakers.
    EmergencyDisable,
    /// Mirror and offline publication drills: verification that approved
    /// mirrors and offline bundles can be imported and verified.
    MirrorOfflineDrill,
}

impl ResponseKind {
    /// Every response kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SecurityResponse,
        Self::AdvisoryPublication,
        Self::CveGhsaPublication,
        Self::EmergencyDisable,
        Self::MirrorOfflineDrill,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SecurityResponse => "security_response",
            Self::AdvisoryPublication => "advisory_publication",
            Self::CveGhsaPublication => "cve_ghsa_publication",
            Self::EmergencyDisable => "emergency_disable",
            Self::MirrorOfflineDrill => "mirror_offline_drill",
        }
    }
}

/// Response state a lane earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseState {
    /// The lane is ready: a captured, within-SLO response packet, every required
    /// emergency control satisfied or mirror drill checkpoint verified, and an
    /// owner sign-off back the public claim at its full canonical lifecycle
    /// label.
    Ready,
    /// The lane carries the claim's full label only because an active, unexpired
    /// waiver covers a recorded residual gap.
    ReadyOnWaiver,
    /// A required emergency control is unsatisfied, a mirror drill checkpoint is
    /// unverified, the row evidence is incomplete, or the owner sign-off is
    /// absent; the lane is not ready and the label must narrow.
    NotReadyUnbacked,
    /// The public claim this lane backs is itself below the cutline, so the lane
    /// inherits that ceiling and narrows.
    NotReadyClaimNarrowed,
    /// The response packet breached its freshness SLO (or is missing); the lane
    /// is not ready and the label must narrow.
    NotReadyStale,
    /// The lane relied on a waiver that has expired; the label must narrow.
    NotReadyWaiverExpired,
}

impl ResponseState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Ready,
        Self::ReadyOnWaiver,
        Self::NotReadyUnbacked,
        Self::NotReadyClaimNarrowed,
        Self::NotReadyStale,
        Self::NotReadyWaiverExpired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::ReadyOnWaiver => "ready_on_waiver",
            Self::NotReadyUnbacked => "not_ready_unbacked",
            Self::NotReadyClaimNarrowed => "not_ready_claim_narrowed",
            Self::NotReadyStale => "not_ready_stale",
            Self::NotReadyWaiverExpired => "not_ready_waiver_expired",
        }
    }

    /// Whether the state lets a lane carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Ready | Self::ReadyOnWaiver)
    }

    /// Whether the state forces the lane below the public claim label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a lane narrows or a response rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The backing public claim narrowed below the cutline.
    ClaimLabelNarrowed,
    /// Required response evidence is incomplete.
    ResponseEvidenceIncomplete,
    /// The response packet breached its freshness SLO.
    ResponsePacketFreshnessBreached,
    /// No response packet has been captured.
    ResponsePacketMissing,
    /// A waiver the lane relied on has expired.
    WaiverExpired,
    /// The required lane owner sign-off is missing.
    OwnerSignoffMissing,
    /// One or more required emergency controls are unsatisfied.
    EmergencyControlUnsatisfied,
    /// One or more required mirror drill checkpoints are unverified.
    MirrorDrillUnverified,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ClaimLabelNarrowed,
        Self::ResponseEvidenceIncomplete,
        Self::ResponsePacketFreshnessBreached,
        Self::ResponsePacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
        Self::EmergencyControlUnsatisfied,
        Self::MirrorDrillUnverified,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::ResponseEvidenceIncomplete => "response_evidence_incomplete",
            Self::ResponsePacketFreshnessBreached => "response_packet_freshness_breached",
            Self::ResponsePacketMissing => "response_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::EmergencyControlUnsatisfied => "emergency_control_unsatisfied",
            Self::MirrorDrillUnverified => "mirror_drill_unverified",
        }
    }
}

/// Default action a response rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the lane's published lifecycle label below the cutline.
    NarrowResponseLabel,
    /// Refresh the response packet so it re-enters its freshness SLO.
    RefreshResponsePacket,
    /// Satisfy the unsatisfied required emergency control.
    SatisfyEmergencyControl,
    /// Verify the unverified required mirror drill checkpoint.
    VerifyMirrorDrill,
    /// Recapture the response evidence the packet depends on.
    RecaptureResponseEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl ResponseAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::HoldPublication,
        Self::NarrowResponseLabel,
        Self::RefreshResponsePacket,
        Self::SatisfyEmergencyControl,
        Self::VerifyMirrorDrill,
        Self::RecaptureResponseEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowResponseLabel => "narrow_response_label",
            Self::RefreshResponsePacket => "refresh_response_packet",
            Self::SatisfyEmergencyControl => "satisfy_emergency_control",
            Self::VerifyMirrorDrill => "verify_mirror_drill",
            Self::RecaptureResponseEvidence => "recapture_response_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One required emergency control on a row: a concrete emergency-readiness check
/// that must be satisfied for the lane to return Ready (a kill-switch test, a
/// feature-flag disable drill, a circuit-breaker rehearsal, …).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmergencyControl {
    /// Stable control id.
    pub control_id: String,
    /// Human-readable title.
    pub title: String,
    /// Ref to the artifact or policy the control checks.
    pub control_ref: String,
    /// Whether the control is satisfied.
    pub satisfied: bool,
}

/// One required mirror drill checkpoint on a row: a concrete restore point or
/// verification step that must be verified for the lane to return Ready (a
/// mirror-only import, a deny-all bundle verification, an offline bundle
/// validation, …).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MirrorDrillCheckpoint {
    /// Stable checkpoint id.
    pub checkpoint_id: String,
    /// Human-readable title.
    pub title: String,
    /// Ref to the restore point or artifact the checkpoint verifies.
    pub restore_point_ref: String,
    /// Whether the checkpoint was verified to its restore point.
    pub verified: bool,
}

/// One response rule: a closed condition that narrows a lane and may gate
/// publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: GapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: ResponseAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One response row: a `(response lane, public claim)` binding bound to its
/// response packet, required emergency controls or mirror drill checkpoints,
/// canonical ceiling label, and packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseRow {
    /// Stable response-row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The response lane kind this row governs.
    pub response_kind: ResponseKind,
    /// Ref to the response-lane subject this row covers.
    pub lane_ref: String,
    /// Reviewable one-line statement of the response lane.
    pub lane_summary: String,
    /// Whether the lane is part of the release-blocking response set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a
    /// row may never carry a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Response state earned for the row.
    pub response_state: ResponseState,
    /// The response packet and its freshness SLO.
    pub response_packet: ProofPacket,
    /// Waiver authorizing a provisional readiness, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Lane owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Required emergency controls the row must satisfy.
    #[serde(default)]
    pub emergency_controls: Vec<EmergencyControl>,
    /// Required mirror drill checkpoints the row must verify.
    #[serde(default)]
    pub mirror_drill_checkpoints: Vec<MirrorDrillCheckpoint>,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// The lifecycle label the row effectively carries after narrowing.
    pub effective_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl ResponseRow {
    /// True when the effective label is at or above the cutline.
    pub fn holds_stable(&self) -> bool {
        self.effective_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets it carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.response_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: GapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// True when every required emergency control is satisfied.
    pub fn all_emergency_controls_satisfied(&self) -> bool {
        !self.emergency_controls.is_empty()
            && self
                .emergency_controls
                .iter()
                .all(|control| control.satisfied)
    }

    /// Count of required emergency controls that are unsatisfied.
    pub fn unsatisfied_emergency_control_count(&self) -> usize {
        self.emergency_controls
            .iter()
            .filter(|control| !control.satisfied)
            .count()
    }

    /// True when every required mirror drill checkpoint is verified.
    pub fn all_mirror_checkpoints_verified(&self) -> bool {
        !self.mirror_drill_checkpoints.is_empty()
            && self
                .mirror_drill_checkpoints
                .iter()
                .all(|checkpoint| checkpoint.verified)
    }

    /// Count of required mirror drill checkpoints that are unverified.
    pub fn unverified_mirror_checkpoint_count(&self) -> usize {
        self.mirror_drill_checkpoints
            .iter()
            .filter(|checkpoint| !checkpoint.verified)
            .count()
    }
}

/// The recorded publication verdict for the security-response packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponsePublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Response-rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Response-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecurityResponsePacketSummary {
    /// Total number of response rows.
    pub total_rows: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows carrying a label at or above the cutline.
    pub rows_ready: usize,
    /// Rows narrowed below the cutline.
    pub rows_narrowed_below_cutline: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows carrying a label at or above the cutline.
    pub release_blocking_ready: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Rows holding their label via an active waiver.
    pub rows_on_active_waiver: usize,
    /// Response packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Response packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Response packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Response packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total required emergency controls across all rows.
    pub total_emergency_controls: usize,
    /// Required emergency controls that are satisfied.
    pub emergency_controls_satisfied: usize,
    /// Required emergency controls that are unsatisfied.
    pub emergency_controls_unsatisfied: usize,
    /// Total required mirror drill checkpoints across all rows.
    pub total_mirror_checkpoints: usize,
    /// Required mirror drill checkpoints that are verified.
    pub mirror_checkpoints_verified: usize,
    /// Required mirror drill checkpoints that are unverified.
    pub mirror_checkpoints_unverified: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of response rules currently firing.
    pub rules_firing: usize,
}

/// The typed security-response packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecurityResponsePacket {
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
    /// Ref to the stable claim manifest this packet ingests as its public-claim
    /// source and ceiling.
    pub claim_manifest_ref: String,
    /// Ref to the stable proof-index row proving this packet.
    pub stable_proof_index_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed response-kind vocabulary.
    pub response_kinds: Vec<ResponseKind>,
    /// Closed response-state vocabulary.
    pub response_states: Vec<ResponseState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Closed response-action vocabulary.
    pub response_actions: Vec<ResponseAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking lane refs this packet must cover.
    pub release_blocking_lane_refs: Vec<String>,
    /// Response rules.
    pub rules: Vec<ResponseRule>,
    /// Response rows.
    pub rows: Vec<ResponseRow>,
    /// Recorded publication verdict.
    pub publication: ResponsePublicationRecord,
    /// Summary counts.
    pub summary: SecurityResponsePacketSummary,
}

impl SecurityResponsePacket {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&ResponseRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows carrying a label at or above the cutline.
    pub fn rows_ready(&self) -> Vec<&ResponseRow> {
        self.rows.iter().filter(|row| row.holds_stable()).collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&ResponseRow> {
        self.rows.iter().filter(|row| !row.holds_stable()).collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&ResponseRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one response kind.
    pub fn rows_for_kind(&self, kind: ResponseKind) -> Vec<&ResponseRow> {
        self.rows
            .iter()
            .filter(|row| row.response_kind == kind)
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
    pub fn rule_fires(&self, rule: &ResponseRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and response rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .rules
            .iter()
            .any(|rule| rule.blocks_publication && self.rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Response-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose
    /// claim is already canonically narrowed is not a *publication* blocker, it
    /// merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<GapReason> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.rule_fires(rule))
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

    /// Recomputes the summary block from the rows and response rules.
    pub fn computed_summary(&self) -> SecurityResponsePacketSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.response_packet.slo_state == state)
                .count()
        };
        let release_blocking: Vec<&ResponseRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        let total_emergency_controls: usize = self
            .rows
            .iter()
            .map(|row| row.emergency_controls.len())
            .sum();
        let emergency_controls_unsatisfied: usize = self
            .rows
            .iter()
            .map(ResponseRow::unsatisfied_emergency_control_count)
            .sum();
        let total_mirror_checkpoints: usize = self
            .rows
            .iter()
            .map(|row| row.mirror_drill_checkpoints.len())
            .sum();
        let mirror_checkpoints_unverified: usize = self
            .rows
            .iter()
            .map(ResponseRow::unverified_mirror_checkpoint_count)
            .sum();
        SecurityResponsePacketSummary {
            total_rows: self.rows.len(),
            total_claims: self.claims().len(),
            rows_ready: self.rows.iter().filter(|row| row.holds_stable()).count(),
            rows_narrowed_below_cutline: self.rows.iter().filter(|row| !row.holds_stable()).count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_ready: release_blocking
                .iter()
                .filter(|row| row.holds_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.holds_stable())
                .count(),
            rows_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.response_state == ResponseState::ReadyOnWaiver)
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_emergency_controls,
            emergency_controls_satisfied: total_emergency_controls - emergency_controls_unsatisfied,
            emergency_controls_unsatisfied,
            total_mirror_checkpoints,
            mirror_checkpoints_verified: total_mirror_checkpoints - mirror_checkpoints_unverified,
            mirror_checkpoints_unverified,
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            rules_firing: self
                .rules
                .iter()
                .filter(|rule| self.rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the packet that downstream
    /// surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> ResponseExportProjection {
        ResponseExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| ResponseExportRow {
                    entry_id: row.entry_id.clone(),
                    response_kind: row.response_kind,
                    lane_ref: row.lane_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    effective_label: row.effective_label,
                    holds_stable: row.holds_stable(),
                    response_state: row.response_state,
                    slo_state: row.response_packet.slo_state,
                    emergency_control_total: row.emergency_controls.len(),
                    emergency_control_unsatisfied: row.unsatisfied_emergency_control_count(),
                    mirror_checkpoint_total: row.mirror_drill_checkpoints.len(),
                    mirror_checkpoint_unverified: row.unverified_mirror_checkpoint_count(),
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<SecurityResponsePacketViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(SecurityResponsePacketViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(SecurityResponsePacketViolation::EmptyPacket);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(SecurityResponsePacketViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<SecurityResponsePacketViolation>) {
        if self.schema_version != SECURITY_RESPONSE_PACKET_SCHEMA_VERSION {
            violations.push(SecurityResponsePacketViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != SECURITY_RESPONSE_PACKET_RECORD_KIND {
            violations.push(SecurityResponsePacketViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("stable_proof_index_ref", &self.stable_proof_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(SecurityResponsePacketViolation::EmptyField {
                    entry_id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(SecurityResponsePacketViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.response_kinds != ResponseKind::ALL.to_vec() {
            violations.push(SecurityResponsePacketViolation::ClosedVocabularyMismatch {
                field: "response_kinds",
            });
        }
        if self.response_states != ResponseState::ALL.to_vec() {
            violations.push(SecurityResponsePacketViolation::ClosedVocabularyMismatch {
                field: "response_states",
            });
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(SecurityResponsePacketViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.response_actions != ResponseAction::ALL.to_vec() {
            violations.push(SecurityResponsePacketViolation::ClosedVocabularyMismatch {
                field: "response_actions",
            });
        }
        if self.release_blocking_lane_refs.is_empty() {
            violations.push(SecurityResponsePacketViolation::EmptyField {
                entry_id: "<packet>".to_owned(),
                field_name: "release_blocking_lane_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(SecurityResponsePacketViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(SecurityResponsePacketViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(SecurityResponsePacketViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(SecurityResponsePacketViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<SecurityResponsePacketViolation>) {
        if self.rules.is_empty() {
            violations.push(SecurityResponsePacketViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(SecurityResponsePacketViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(SecurityResponsePacketViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(SecurityResponsePacketViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every gap reason must have a rule, so a gap reason cannot fire without a
        // corresponding publication gate.
        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(SecurityResponsePacketViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &ResponseRow,
        violations: &mut Vec<SecurityResponsePacketViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("lane_ref", &row.lane_ref),
            ("lane_summary", &row.lane_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("response_packet.packet_id", &row.response_packet.packet_id),
            (
                "response_packet.packet_ref",
                &row.response_packet.packet_ref,
            ),
            (
                "response_packet.proof_index_ref",
                &row.response_packet.proof_index_ref,
            ),
            (
                "response_packet.freshness_slo.slo_register_ref",
                &row.response_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(SecurityResponsePacketViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no row may carry a label wider than the public claim's
        // canonical label.
        if row.effective_label.rank() > row.claim_label.rank() {
            violations.push(SecurityResponsePacketViolation::EffectiveWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                effective: row.effective_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.response_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(SecurityResponsePacketViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "response_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.response_packet.freshness_slo.window_is_consistent() {
            violations.push(SecurityResponsePacketViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        self.validate_emergency_controls(row, violations);
        self.validate_mirror_checkpoints(row, violations);

        // A public claim whose canonical label is below the cutline forces the row
        // to inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(SecurityResponsePacketViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                violations.push(
                    SecurityResponsePacketViolation::ClaimNarrowedWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        let slo_state = row.response_packet.slo_state;

        if row.holds_label() {
            // A ready row carries exactly the public claim's canonical label,
            // carries no active gap reason, rides a captured within-SLO packet,
            // satisfies every emergency control (if any), verifies every mirror
            // drill checkpoint (if any), is owner-signed, and (for an on-waiver
            // row) relies on an unexpired waiver.
            if row.effective_label != row.claim_label {
                violations.push(SecurityResponsePacketViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    effective: row.effective_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(SecurityResponsePacketViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.response_packet.has_capture() {
                violations.push(SecurityResponsePacketViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(SecurityResponsePacketViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !row.emergency_controls.is_empty() && !row.all_emergency_controls_satisfied() {
                violations.push(
                    SecurityResponsePacketViolation::HeldWithUnsatisfiedEmergencyControl {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !row.mirror_drill_checkpoints.is_empty() && !row.all_mirror_checkpoints_verified() {
                violations.push(
                    SecurityResponsePacketViolation::HeldWithUnverifiedMirrorCheckpoint {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(SecurityResponsePacketViolation::HeldWithoutOwnerSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            if row
                .waiver
                .as_ref()
                .is_some_and(|waiver| waiver.expires_at.as_str() <= self.as_of.as_str())
            {
                violations.push(SecurityResponsePacketViolation::HeldOnExpiredWaiver {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the effective label below the cutline
            // and name at least one active reason.
            if row.holds_stable() {
                violations.push(SecurityResponsePacketViolation::EffectiveLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.response_state,
                    effective: row.effective_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(SecurityResponsePacketViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.response_state,
                });
            }
            // A narrowing row whose packet is breached or missing must name the
            // matching freshness reason, so the freshness automation stays honest.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(GapReason::ResponsePacketFreshnessBreached)
            {
                violations.push(
                    SecurityResponsePacketViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(GapReason::ResponsePacketMissing)
            {
                violations.push(
                    SecurityResponsePacketViolation::MissingPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_emergency_controls(
        &self,
        row: &ResponseRow,
        violations: &mut Vec<SecurityResponsePacketViolation>,
    ) {
        if row.response_kind == ResponseKind::EmergencyDisable && row.emergency_controls.is_empty()
        {
            violations.push(SecurityResponsePacketViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "emergency_controls",
            });
        }
        let mut seen = BTreeSet::new();
        for control in &row.emergency_controls {
            if !seen.insert(control.control_id.clone()) {
                violations.push(SecurityResponsePacketViolation::DuplicateControlId {
                    entry_id: row.entry_id.clone(),
                    control_id: control.control_id.clone(),
                });
            }
            for (field, value) in [
                ("control_id", &control.control_id),
                ("title", &control.title),
                ("control_ref", &control.control_ref),
            ] {
                if value.trim().is_empty() {
                    violations.push(SecurityResponsePacketViolation::EmptyField {
                        entry_id: row.entry_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
        if row.unsatisfied_emergency_control_count() > 0
            && !row.has_active_reason(GapReason::EmergencyControlUnsatisfied)
        {
            violations.push(
                SecurityResponsePacketViolation::UnsatisfiedControlWithoutReason {
                    entry_id: row.entry_id.clone(),
                },
            );
        }
    }

    fn validate_mirror_checkpoints(
        &self,
        row: &ResponseRow,
        violations: &mut Vec<SecurityResponsePacketViolation>,
    ) {
        if row.response_kind == ResponseKind::MirrorOfflineDrill
            && row.mirror_drill_checkpoints.is_empty()
        {
            violations.push(SecurityResponsePacketViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "mirror_drill_checkpoints",
            });
        }
        let mut seen = BTreeSet::new();
        for checkpoint in &row.mirror_drill_checkpoints {
            if !seen.insert(checkpoint.checkpoint_id.clone()) {
                violations.push(SecurityResponsePacketViolation::DuplicateCheckpointId {
                    entry_id: row.entry_id.clone(),
                    checkpoint_id: checkpoint.checkpoint_id.clone(),
                });
            }
            for (field, value) in [
                ("checkpoint_id", &checkpoint.checkpoint_id),
                ("title", &checkpoint.title),
                ("restore_point_ref", &checkpoint.restore_point_ref),
            ] {
                if value.trim().is_empty() {
                    violations.push(SecurityResponsePacketViolation::EmptyField {
                        entry_id: row.entry_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
        if row.unverified_mirror_checkpoint_count() > 0
            && !row.has_active_reason(GapReason::MirrorDrillUnverified)
        {
            violations.push(
                SecurityResponsePacketViolation::UnverifiedCheckpointWithoutReason {
                    entry_id: row.entry_id.clone(),
                },
            );
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &ResponseRow,
        violations: &mut Vec<SecurityResponsePacketViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<SecurityResponsePacketViolation>,
                               expected: GapReason| {
            violations.push(SecurityResponsePacketViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.response_state,
                expected_reason: expected,
            });
        };

        match row.response_state {
            ResponseState::NotReadyUnbacked => {
                const ALLOWED: [GapReason; 4] = [
                    GapReason::ResponseEvidenceIncomplete,
                    GapReason::OwnerSignoffMissing,
                    GapReason::EmergencyControlUnsatisfied,
                    GapReason::MirrorDrillUnverified,
                ];
                if !ALLOWED.iter().any(|reason| row.has_active_reason(*reason)) {
                    push_incoherent(violations, GapReason::ResponseEvidenceIncomplete);
                }
            }
            ResponseState::NotReadyClaimNarrowed => {
                if !row.has_active_reason(GapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, GapReason::ClaimLabelNarrowed);
                }
            }
            ResponseState::NotReadyStale => {
                if !(row.has_active_reason(GapReason::ResponsePacketFreshnessBreached)
                    || row.has_active_reason(GapReason::ResponsePacketMissing))
                {
                    push_incoherent(violations, GapReason::ResponsePacketFreshnessBreached);
                }
            }
            ResponseState::NotReadyWaiverExpired => {
                if !row.has_active_reason(GapReason::WaiverExpired) {
                    push_incoherent(violations, GapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(SecurityResponsePacketViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.response_state,
                    });
                }
            }
            ResponseState::ReadyOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|waiver| {
                        waiver.waiver_ref.trim().is_empty() || waiver.expires_at.trim().is_empty()
                    })
                    .unwrap_or(true)
                {
                    violations.push(SecurityResponsePacketViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.response_state,
                    });
                }
            }
            ResponseState::Ready => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<SecurityResponsePacketViolation>) {
        // Each lane ref appears at most once: a lane has one canonical response row.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.lane_ref.as_str()) {
                violations.push(SecurityResponsePacketViolation::DuplicateLaneRef {
                    lane_ref: row.lane_ref.clone(),
                });
            }
        }

        // The release line must cover every declared release-blocking lane with
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
                    SecurityResponsePacketViolation::ReleaseBlockingRefWithoutRow {
                        lane_ref: (*declared_ref).to_owned(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !declared.contains(row.lane_ref.as_str()) {
                violations.push(
                    SecurityResponsePacketViolation::ReleaseBlockingRowNotInSet {
                        entry_id: row.entry_id.clone(),
                        lane_ref: row.lane_ref.clone(),
                    },
                );
            }
        }

        // The packet must cover all five response kinds — security_response,
        // advisory_publication, cve_ghsa_publication, emergency_disable, and
        // mirror_offline_drill — so the release line cannot govern some lanes and
        // silently leave a whole response kind ungoverned.
        for kind in ResponseKind::ALL {
            if self.rows_for_kind(kind).is_empty() {
                violations.push(SecurityResponsePacketViolation::ResponseKindAbsent { kind });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<SecurityResponsePacketViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(SecurityResponsePacketViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(SecurityResponsePacketViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                SecurityResponsePacketViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                SecurityResponsePacketViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(
                SecurityResponsePacketViolation::PublicationBlockingSetMismatch {
                    field: "blocking_entry_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseExportRow {
    /// Stable response-row id.
    pub entry_id: String,
    /// Response lane kind.
    pub response_kind: ResponseKind,
    /// Response lane ref.
    pub lane_ref: String,
    /// Whether the lane is part of the release-blocking response set.
    pub release_blocking: bool,
    /// The public-claim entry ref the row backs.
    pub claim_ref: String,
    /// The public claim's canonical ceiling label.
    pub claim_label: StableClaimLevel,
    /// Lifecycle label the row carries.
    pub effective_label: StableClaimLevel,
    /// Whether the row carries a label at or above the cutline.
    pub holds_stable: bool,
    /// Response state.
    pub response_state: ResponseState,
    /// Response-packet freshness-SLO state.
    pub slo_state: FreshnessSloState,
    /// Total required emergency controls.
    pub emergency_control_total: usize,
    /// Required emergency controls that are unsatisfied.
    pub emergency_control_unsatisfied: usize,
    /// Total required mirror drill checkpoints.
    pub mirror_checkpoint_total: usize,
    /// Required mirror drill checkpoints that are unverified.
    pub mirror_checkpoint_unverified: usize,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<GapReason>,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<ResponseExportRow>,
}

/// A validation violation for the security-response packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityResponsePacketViolation {
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
    /// The packet has no rules.
    NoRules,
    /// A required field is empty.
    EmptyField {
        /// Row, rule, or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// An entry id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A control id appears more than once in a row.
    DuplicateControlId {
        /// Row id.
        entry_id: String,
        /// Duplicate control id.
        control_id: String,
    },
    /// A checkpoint id appears more than once in a row.
    DuplicateCheckpointId {
        /// Row id.
        entry_id: String,
        /// Duplicate checkpoint id.
        checkpoint_id: String,
    },
    /// A lane ref appears on more than one row.
    DuplicateLaneRef {
        /// Duplicate lane ref.
        lane_ref: String,
    },
    /// A response rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: GapReason,
    },
    /// A row's effective label is wider than its claim ceiling.
    EffectiveWiderThanClaim {
        /// Row id.
        entry_id: String,
        /// Claim ceiling.
        claim: StableClaimLevel,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A freshness SLO's warn window exceeds its target age.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
    /// A ready row is backed by a public claim already below the cutline.
    HeldOnNarrowedClaim {
        /// Row id.
        entry_id: String,
        /// Claim ceiling.
        claim: StableClaimLevel,
    },
    /// A row whose claim narrowed does not name the claim-narrowed reason.
    ClaimNarrowedWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A ready row carries a label different from its claim ceiling.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Claim ceiling.
        claim: StableClaimLevel,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A ready row carries an active gap reason.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A ready row has no captured, evidence-backed packet.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A ready row rides a stale or missing packet.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// SLO state.
        slo_state: FreshnessSloState,
    },
    /// A ready row carries an unsatisfied emergency control.
    HeldWithUnsatisfiedEmergencyControl {
        /// Row id.
        entry_id: String,
    },
    /// A ready row has an unverified mirror drill checkpoint.
    HeldWithUnverifiedMirrorCheckpoint {
        /// Row id.
        entry_id: String,
    },
    /// A ready row lacks owner sign-off.
    HeldWithoutOwnerSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A ready row relies on an expired waiver.
    HeldOnExpiredWaiver {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing state did not narrow below the cutline.
    EffectiveLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: ResponseState,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A narrowing state carries no reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: ResponseState,
    },
    /// A breached packet does not name the freshness breach reason.
    BreachedPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A missing packet does not name the missing packet reason.
    MissingPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// An unsatisfied emergency control does not name the unsatisfied-control reason.
    UnsatisfiedControlWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// An unverified mirror checkpoint does not name the unverified-checkpoint reason.
    UnverifiedCheckpointWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A row state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: ResponseState,
        /// Expected reason.
        expected_reason: GapReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: ResponseState,
    },
    /// A declared release-blocking lane ref has no row.
    ReleaseBlockingRefWithoutRow {
        /// Missing lane ref.
        lane_ref: String,
    },
    /// A release-blocking row was not declared.
    ReleaseBlockingRowNotInSet {
        /// Row id.
        entry_id: String,
        /// The row's lane ref.
        lane_ref: String,
    },
    /// A response kind is absent from the packet.
    ResponseKindAbsent {
        /// Missing kind.
        kind: ResponseKind,
    },
    /// The declared publication decision disagrees with the computed decision.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared publication blocking set disagrees with computed rules.
    PublicationBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// Summary counts disagree with rows.
    SummaryMismatch,
}

impl fmt::Display for SecurityResponsePacketViolation {
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
            Self::NoRules => write!(f, "packet has no rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate response row id {entry_id}")
            }
            Self::DuplicateRuleId { rule_id } => write!(f, "duplicate rule id {rule_id}"),
            Self::DuplicateControlId {
                entry_id,
                control_id,
            } => write!(f, "response row {entry_id} repeats control {control_id}"),
            Self::DuplicateCheckpointId {
                entry_id,
                checkpoint_id,
            } => write!(
                f,
                "response row {entry_id} repeats checkpoint {checkpoint_id}"
            ),
            Self::DuplicateLaneRef { lane_ref } => {
                write!(f, "lane ref {lane_ref} appears on more than one row")
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "rule {rule_id} watches no lifecycle labels")
            }
            Self::GapReasonWithoutRule { reason } => write!(
                f,
                "gap reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::EffectiveWiderThanClaim {
                entry_id,
                claim,
                effective,
            } => write!(
                f,
                "response row {entry_id} effective label {} is wider than claim ceiling {}",
                effective.as_str(),
                claim.as_str()
            ),
            Self::FreshnessSloInconsistent { entry_id } => write!(
                f,
                "response row {entry_id} freshness SLO warn window exceeds target age"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "response row {entry_id} holds ready while claim label {} is below the cutline",
                claim.as_str()
            ),
            Self::ClaimNarrowedWithoutReason { entry_id } => write!(
                f,
                "response row {entry_id} backs a narrowed claim without naming claim_label_narrowed"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                effective,
            } => write!(
                f,
                "response row {entry_id} holds ready {} but claim ceiling is {}",
                effective.as_str(),
                claim.as_str()
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "response row {entry_id} holds ready while a gap reason is active")
            }
            Self::HeldWithoutFreshPacket { entry_id } => write!(
                f,
                "response row {entry_id} holds ready with no captured, evidence-backed packet"
            ),
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => write!(
                f,
                "response row {entry_id} holds ready while packet is {}",
                slo_state.as_str()
            ),
            Self::HeldWithUnsatisfiedEmergencyControl { entry_id } => write!(
                f,
                "response row {entry_id} holds ready with an unsatisfied emergency control"
            ),
            Self::HeldWithUnverifiedMirrorCheckpoint { entry_id } => write!(
                f,
                "response row {entry_id} holds ready with an unverified mirror drill checkpoint"
            ),
            Self::HeldWithoutOwnerSignoff { entry_id } => {
                write!(f, "response row {entry_id} holds ready without owner sign-off")
            }
            Self::HeldOnExpiredWaiver { entry_id } => {
                write!(f, "response row {entry_id} holds ready on an expired waiver")
            }
            Self::EffectiveLabelNotNarrowed {
                entry_id,
                state,
                effective,
            } => write!(
                f,
                "response row {entry_id} state {} must narrow but holds {}",
                state.as_str(),
                effective.as_str()
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "response row {entry_id} state {} narrows without a reason",
                state.as_str()
            ),
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "response row {entry_id} has a breached packet without the freshness reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "response row {entry_id} has a missing packet without the missing-packet reason"
            ),
            Self::UnsatisfiedControlWithoutReason { entry_id } => write!(
                f,
                "response row {entry_id} has an unsatisfied control without emergency_control_unsatisfied"
            ),
            Self::UnverifiedCheckpointWithoutReason { entry_id } => write!(
                f,
                "response row {entry_id} has an unverified checkpoint without mirror_drill_unverified"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "response row {entry_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => write!(
                f,
                "response row {entry_id} state {} names no waiver",
                state.as_str()
            ),
            Self::ReleaseBlockingRefWithoutRow { lane_ref } => {
                write!(f, "declared release-blocking lane {lane_ref} has no row")
            }
            Self::ReleaseBlockingRowNotInSet {
                entry_id,
                lane_ref,
            } => write!(
                f,
                "release-blocking row {entry_id} lane {lane_ref} is not declared"
            ),
            Self::ResponseKindAbsent { kind } => {
                write!(f, "response kind {} is covered by no row", kind.as_str())
            }
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "publication decision {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with firing rules")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with rows"),
        }
    }
}

impl Error for SecurityResponsePacketViolation {}

/// Loads the embedded security-response packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`SecurityResponsePacket`].
pub fn current_security_response_packet() -> Result<SecurityResponsePacket, serde_json::Error> {
    serde_json::from_str(SECURITY_RESPONSE_PACKET_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packet() -> SecurityResponsePacket {
        current_security_response_packet().expect("packet parses")
    }

    #[test]
    fn embedded_packet_parses_and_validates() {
        let packet = packet();
        assert_eq!(
            packet.schema_version,
            SECURITY_RESPONSE_PACKET_SCHEMA_VERSION
        );
        assert_eq!(packet.record_kind, SECURITY_RESPONSE_PACKET_RECORD_KIND);
        assert_eq!(packet.validate(), Vec::new());
        assert!(!packet.rows.is_empty());
    }

    #[test]
    fn every_response_kind_is_covered() {
        let packet = packet();
        for kind in ResponseKind::ALL {
            assert!(
                !packet.rows_for_kind(kind).is_empty(),
                "response kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn every_release_blocking_lane_is_covered() {
        let packet = packet();
        assert!(!packet.release_blocking_lane_refs.is_empty());
        let covered: Vec<&str> = packet
            .release_blocking_rows()
            .into_iter()
            .map(|row| row.lane_ref.as_str())
            .collect();
        for declared in &packet.release_blocking_lane_refs {
            assert!(
                covered.contains(&declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn packet_exercises_ready_and_narrowed_rows() {
        let packet = packet();
        assert!(
            !packet.rows_ready().is_empty(),
            "packet must show at least one ready row"
        );
        assert!(
            !packet.rows_narrowed().is_empty(),
            "packet must show at least one narrowed row"
        );
    }

    #[test]
    fn summary_counts_match_rows() {
        let packet = packet();
        assert_eq!(packet.summary, packet.computed_summary());
        assert_eq!(
            packet.summary.rows_ready + packet.summary.rows_narrowed_below_cutline,
            packet.rows.len()
        );
        assert_eq!(
            packet.summary.packets_current
                + packet.summary.packets_due_for_refresh
                + packet.summary.packets_breached
                + packet.summary.packets_missing,
            packet.rows.len()
        );
    }

    #[test]
    fn publication_holds_when_blocking_rules_fire() {
        let packet = packet();
        assert_eq!(packet.publication.decision, PromotionDecision::Hold);
        assert_eq!(
            packet.publication.decision,
            packet.computed_publication_decision()
        );
        assert!(!packet.publication.blocking_rule_ids.is_empty());
        assert!(!packet.publication.blocking_entry_ids.is_empty());
    }

    #[test]
    fn every_gap_reason_has_a_rule() {
        let packet = packet();
        let covered: BTreeSet<GapReason> = packet
            .rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in GapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_row_carries_wider_than_its_claim_ceiling() {
        let packet = packet();
        for row in &packet.rows {
            assert!(
                row.effective_label.rank() <= row.claim_label.rank(),
                "{} carries wider than its ceiling",
                row.entry_id
            );
        }
    }

    #[test]
    fn validate_flags_a_row_wider_than_ceiling() {
        let mut packet = packet();
        let row = packet
            .rows
            .iter_mut()
            .find(|row| !row.holds_stable() && row.claim_label == StableClaimLevel::Beta)
            .expect("a narrowed row under a beta ceiling exists");
        row.effective_label = StableClaimLevel::Stable;
        let entry_id = row.entry_id.clone();
        packet.summary = packet.computed_summary();
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            SecurityResponsePacketViolation::EffectiveWiderThanClaim { entry_id: id, .. } if *id == entry_id
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut packet = packet();
        let row = packet
            .rows
            .iter_mut()
            .find(|row| row.response_state == ResponseState::NotReadyStale)
            .expect("a not-ready-stale row exists");
        row.effective_label = row.claim_label;
        packet.summary = packet.computed_summary();
        packet.publication.decision = packet.computed_publication_decision();
        packet.publication.blocking_rule_ids = packet.computed_blocking_rule_ids();
        packet.publication.blocking_entry_ids = packet.computed_blocking_entry_ids();
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            SecurityResponsePacketViolation::EffectiveLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut packet = packet();
        packet.publication.decision = PromotionDecision::Proceed;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            SecurityResponsePacketViolation::PublicationDecisionInconsistent { .. }
        )));
    }
}
