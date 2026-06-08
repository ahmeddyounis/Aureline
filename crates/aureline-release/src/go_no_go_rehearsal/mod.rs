//! Typed final go/no-go rehearsal for the stable launch line, with an explicit launch
//! cutline, exception packets, and rollback checkpoints.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes; the
//! [`stable_proof_index`](crate::stable_proof_index) decides whether each
//! launch-blocking *requirement* is proven; the
//! [`stable_version_windows`](crate::stable_version_windows) freezes each public
//! interface surface's version window; and the
//! [`open_paid_boundary_audit`](crate::open_paid_boundary_audit) audits the governance
//! facts the launch rests on. None of them answer the question this module answers: **was
//! the release train actually rehearsed end-to-end before the go/no-go — the explicit
//! cutline signed off, the promotion step dry-run, each rollback checkpoint verified to a
//! restore point, and every open exception packet reviewed — behind a fresh rehearsal
//! packet and an owner sign-off, and is each stage narrowed to a No-Go the moment its
//! backing thins out?** This module is the **final go/no-go rehearsal**. For every
//! rehearsal stage it records one row that binds the stage to the
//! [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose lifecycle label it
//! backs, the rehearsal packet that grounds it, the rollback checkpoints it must verify,
//! the exception packet (if any) holding it provisionally, and the owner sign-off.
//!
//! Each [`RehearsalStageRow`] is one `(rehearsal stage, public claim)` binding. It:
//!
//! - names the stage kind it exercises ([`RehearsalStageRow::stage_kind`],
//!   [`RehearsalStageRow::subject_ref`], [`RehearsalStageRow::subject_summary`]) and
//!   whether that stage is part of the release-blocking rehearsal set
//!   ([`RehearsalStageRow::release_blocking`]);
//! - pins the rehearsal packet ([`ProofPacket`]) with its packet-freshness SLO and the
//!   [`RollbackCheckpoint`] set whose every member must be verified for the stage to
//!   return Go;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose public
//!   claim it backs ([`RehearsalStageRow::claim_ref`]) and the canonical lifecycle label
//!   that entry publishes ([`RehearsalStageRow::claim_label`]). That label is a hard
//!   **ceiling**: a stage may carry the claim's label or narrow below it, but it may never
//!   assert a public claim wider than the public claim it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-rehearsal labels,
//!   so docs, Help/About, the release center, and support exports ingest one verdict per
//!   stage instead of cloning their own;
//! - records the rehearsal state earned ([`RehearsalState`]), the active gap reasons
//!   ([`RehearsalGapReason`]), and the label it *effectively* publishes after narrowing
//!   ([`RehearsalStageRow::effective_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary between
//! a stage whose backing supports a Stable public claim and one narrowed below it. A stage
//! that is not rehearsed clean — because its rehearsal packet aged out or is missing,
//! because a required rollback checkpoint is unverified, because its evidence is
//! incomplete, because its owner sign-off is missing, because its exception packet
//! expired, or because the public claim it backs is itself below the cutline — is
//! structurally required to drop below the cutline rather than inherit an adjacent
//! rehearsed stage. The [`RehearsalRule`] set names the closed conditions that gate the
//! go/no-go, and [`GoNoGoRehearsal::publication`] records the proceed/hold verdict.
//!
//! The rehearsal is checked in at `artifacts/release/go_no_go_rehearsal.json` and embedded
//! here, so this typed consumer and the CI gate agree on every row without a cargo build in
//! CI.
//!
//! The model is metadata-only: every field is a typed state, a boolean control flag, or an
//! opaque ref. It carries no raw artifacts, raw logs, signatures, or credential material.
//! Two classes of check live outside this model because they need more than the rehearsal
//! sees: date arithmetic (recomputing the packet-freshness state and exception expiry
//! against an `as_of` date) and the cross-artifact ceiling check (whether each row's
//! `claim_label` still equals the label the stable claim manifest publishes for the entry
//! named by `claim_ref`). Those live in the CI gate. This model enforces the structural and
//! logical invariants that hold regardless of the clock and the neighbouring artifact — the
//! ceiling/no-widening rule, rollback-checkpoint completeness, narrowing consistency,
//! packet/state coherence, owner sign-off on Go stages, stage-kind and release-line
//! coverage, rehearsal-rule wiring, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported go/no-go-rehearsal schema version.
pub const GO_NO_GO_REHEARSAL_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the rehearsal.
pub const GO_NO_GO_REHEARSAL_RECORD_KIND: &str = "go_no_go_rehearsal";

/// Repo-relative path to the checked-in rehearsal.
pub const GO_NO_GO_REHEARSAL_PATH: &str = "artifacts/release/go_no_go_rehearsal.json";

/// Embedded checked-in rehearsal JSON.
pub const GO_NO_GO_REHEARSAL_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/go_no_go_rehearsal.json"
));

/// The rehearsal stage a row exercises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageKind {
    /// The explicit launch-cutline review and go/no-go signoff.
    CutlineReview,
    /// The promotion publish-step dry run, exercised against the live train.
    PromotionStep,
    /// A rollback drill that restores from a verified rollback checkpoint.
    RollbackCheckpoint,
    /// The review of open launch exception packets before the go/no-go.
    ExceptionReview,
}

impl StageKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CutlineReview,
        Self::PromotionStep,
        Self::RollbackCheckpoint,
        Self::ExceptionReview,
    ];

    /// Stable token recorded in the rehearsal.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CutlineReview => "cutline_review",
            Self::PromotionStep => "promotion_step",
            Self::RollbackCheckpoint => "rollback_checkpoint",
            Self::ExceptionReview => "exception_review",
        }
    }
}

/// Go/no-go state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RehearsalState {
    /// The stage was rehearsed end-to-end: a captured, within-SLO rehearsal packet, every
    /// required rollback checkpoint verified, and an owner sign-off back the public claim
    /// at its full canonical lifecycle label.
    GoRehearsed,
    /// The stage returns Go at the claim's full label only because an active, unexpired
    /// exception packet covers a recorded residual gap.
    GoOnException,
    /// A required rollback checkpoint is unverified, the stage evidence is incomplete, or
    /// the owner sign-off is absent; the stage is a No-Go and the label must narrow.
    NoGoUnrehearsed,
    /// The public claim this stage backs is itself below the cutline, so the stage inherits
    /// that ceiling and narrows.
    NoGoClaimNarrowed,
    /// The rehearsal packet breached its freshness SLO (or is missing); the stage is a
    /// No-Go and the label must narrow.
    NoGoStale,
    /// The stage relied on an exception packet that has expired; the label must narrow.
    NoGoExceptionExpired,
}

impl RehearsalState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GoRehearsed,
        Self::GoOnException,
        Self::NoGoUnrehearsed,
        Self::NoGoClaimNarrowed,
        Self::NoGoStale,
        Self::NoGoExceptionExpired,
    ];

    /// Stable token recorded in the rehearsal.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GoRehearsed => "go_rehearsed",
            Self::GoOnException => "go_on_exception",
            Self::NoGoUnrehearsed => "no_go_unrehearsed",
            Self::NoGoClaimNarrowed => "no_go_claim_narrowed",
            Self::NoGoStale => "no_go_stale",
            Self::NoGoExceptionExpired => "no_go_exception_expired",
        }
    }

    /// Whether the state lets the stage carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::GoRehearsed | Self::GoOnException)
    }

    /// Whether the state forces the stage below the public claim label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a stage narrows or a rehearsal rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RehearsalGapReason {
    /// The backing public claim narrowed below the cutline.
    ClaimLabelNarrowed,
    /// Required rehearsal evidence is incomplete.
    RehearsalEvidenceIncomplete,
    /// The rehearsal packet breached its freshness SLO.
    RehearsalPacketFreshnessBreached,
    /// No rehearsal packet has been captured.
    RehearsalPacketMissing,
    /// An exception packet the stage relied on has expired.
    ExceptionExpired,
    /// The required stage owner sign-off is missing.
    OwnerSignoffMissing,
    /// One or more required rollback checkpoints are unverified.
    RollbackCheckpointUnverified,
}

impl RehearsalGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ClaimLabelNarrowed,
        Self::RehearsalEvidenceIncomplete,
        Self::RehearsalPacketFreshnessBreached,
        Self::RehearsalPacketMissing,
        Self::ExceptionExpired,
        Self::OwnerSignoffMissing,
        Self::RollbackCheckpointUnverified,
    ];

    /// Stable token recorded in the rehearsal.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::RehearsalEvidenceIncomplete => "rehearsal_evidence_incomplete",
            Self::RehearsalPacketFreshnessBreached => "rehearsal_packet_freshness_breached",
            Self::RehearsalPacketMissing => "rehearsal_packet_missing",
            Self::ExceptionExpired => "exception_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::RollbackCheckpointUnverified => "rollback_checkpoint_unverified",
        }
    }
}

/// Default action a rehearsal rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RehearsalAction {
    /// Hold the go/no-go until the condition clears.
    HoldGoNoGo,
    /// Narrow the stage's published lifecycle label below the cutline.
    NarrowRehearsalLabel,
    /// Refresh the rehearsal packet so it re-enters its freshness SLO.
    RefreshRehearsalPacket,
    /// Verify the unverified required rollback checkpoint.
    VerifyRollbackCheckpoint,
    /// Recapture the rehearsal evidence the packet depends on.
    RecaptureRehearsalEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl RehearsalAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldGoNoGo,
        Self::NarrowRehearsalLabel,
        Self::RefreshRehearsalPacket,
        Self::VerifyRollbackCheckpoint,
        Self::RecaptureRehearsalEvidence,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the rehearsal.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldGoNoGo => "hold_go_no_go",
            Self::NarrowRehearsalLabel => "narrow_rehearsal_label",
            Self::RefreshRehearsalPacket => "refresh_rehearsal_packet",
            Self::VerifyRollbackCheckpoint => "verify_rollback_checkpoint",
            Self::RecaptureRehearsalEvidence => "recapture_rehearsal_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One required rollback checkpoint on a stage: a concrete restore point that must be
/// verified for the stage to return Go (a frozen cutline, a clean promotion dry run, a
/// previous-build restore, a preserved-state rollback, …).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackCheckpoint {
    /// Stable checkpoint id.
    pub checkpoint_id: String,
    /// Human-readable title.
    pub title: String,
    /// Ref to the restore point or artifact the checkpoint verifies.
    pub restore_point_ref: String,
    /// Whether the checkpoint was verified to its restore point.
    pub verified: bool,
}

/// One rehearsal rule: a closed condition that narrows a stage and may gate the go/no-go.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RehearsalRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched stage fires this rule.
    pub trigger_reason: RehearsalGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: RehearsalAction,
    /// Whether firing this rule blocks the go/no-go.
    pub blocks_go_no_go: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One rehearsal row: a `(rehearsal stage, public claim)` binding bound to its rehearsal
/// packet, required rollback checkpoints, canonical ceiling label, and packet-freshness SLO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RehearsalStageRow {
    /// Stable rehearsal-row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The rehearsal stage this row exercises.
    pub stage_kind: StageKind,
    /// Ref to the rehearsal-stage subject this row covers.
    pub subject_ref: String,
    /// Reviewable one-line statement of the rehearsed stage.
    pub subject_summary: String,
    /// Whether the stage is part of the release-blocking rehearsal set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: a row may
    /// never carry a label wider than this.
    pub claim_label: StableClaimLevel,
    /// Rehearsal state earned for the row.
    pub rehearsal_state: RehearsalState,
    /// The rehearsal packet and its freshness SLO.
    pub rehearsal_packet: ProofPacket,
    /// Exception packet authorizing a provisional Go, when present.
    #[serde(default)]
    pub exception_packet: Option<QualificationWaiver>,
    /// Stage owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Required rollback checkpoints the stage must verify.
    #[serde(default)]
    pub rollback_checkpoints: Vec<RollbackCheckpoint>,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<RehearsalGapReason>,
    /// The lifecycle label the row effectively carries after narrowing.
    pub effective_label: StableClaimLevel,
    /// Publication destinations that render this row's verdict.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl RehearsalStageRow {
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
        self.rehearsal_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: RehearsalGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// True when every required rollback checkpoint is verified.
    pub fn all_checkpoints_verified(&self) -> bool {
        !self.rollback_checkpoints.is_empty()
            && self
                .rollback_checkpoints
                .iter()
                .all(|checkpoint| checkpoint.verified)
    }

    /// Count of required rollback checkpoints that are unverified.
    pub fn unverified_checkpoint_count(&self) -> usize {
        self.rollback_checkpoints
            .iter()
            .filter(|checkpoint| !checkpoint.verified)
            .count()
    }
}

/// The recorded go/no-go verdict for the final rehearsal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RehearsalPublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Rehearsal-rule ids that block the go/no-go, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Rehearsal-row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the rehearsal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoNoGoRehearsalSummary {
    /// Total number of rehearsal rows.
    pub total_rows: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows carrying a label at or above the cutline (Go).
    pub rows_go: usize,
    /// Rows narrowed below the cutline (No-Go).
    pub rows_no_go_below_cutline: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows carrying a label at or above the cutline.
    pub release_blocking_go: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_no_go: usize,
    /// Rows holding their label via an active exception packet.
    pub rows_on_active_exception: usize,
    /// Rehearsal packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Rehearsal packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Rehearsal packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Rehearsal packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total required rollback checkpoints across all rows.
    pub total_checkpoints: usize,
    /// Required rollback checkpoints that are verified.
    pub checkpoints_verified: usize,
    /// Required rollback checkpoints that are unverified.
    pub checkpoints_unverified: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of rehearsal rules currently firing.
    pub rules_firing: usize,
}

/// The typed final go/no-go rehearsal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoNoGoRehearsal {
    /// Rehearsal schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable rehearsal identifier.
    pub rehearsal_id: String,
    /// Lifecycle status of this rehearsal artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this rehearsal ingests as its public-claim source
    /// and ceiling.
    pub claim_manifest_ref: String,
    /// Ref to the stable proof-index row proving this rehearsal.
    pub stable_proof_index_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed stage-kind vocabulary.
    pub stage_kinds: Vec<StageKind>,
    /// Closed rehearsal-state vocabulary.
    pub rehearsal_states: Vec<RehearsalState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<RehearsalGapReason>,
    /// Closed rehearsal-action vocabulary.
    pub rehearsal_actions: Vec<RehearsalAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking rehearsal refs this rehearsal must cover.
    pub release_blocking_stage_refs: Vec<String>,
    /// Rehearsal rules.
    pub rules: Vec<RehearsalRule>,
    /// Rehearsal rows.
    pub rows: Vec<RehearsalStageRow>,
    /// Recorded go/no-go verdict.
    pub publication: RehearsalPublicationRecord,
    /// Summary counts.
    pub summary: GoNoGoRehearsalSummary,
}

impl GoNoGoRehearsal {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&RehearsalStageRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows carrying a label at or above the cutline (Go).
    pub fn rows_go(&self) -> Vec<&RehearsalStageRow> {
        self.rows.iter().filter(|row| row.holds_stable()).collect()
    }

    /// Returns the rows narrowed below the cutline (No-Go).
    pub fn rows_no_go(&self) -> Vec<&RehearsalStageRow> {
        self.rows.iter().filter(|row| !row.holds_stable()).collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&RehearsalStageRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one rehearsal stage kind.
    pub fn rows_for_kind(&self, kind: StageKind) -> Vec<&RehearsalStageRow> {
        self.rows
            .iter()
            .filter(|row| row.stage_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the rehearsal covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn rule_fires(&self, rule: &RehearsalRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the go/no-go verdict from the rows and rehearsal rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .rules
            .iter()
            .any(|rule| rule.blocks_go_no_go && self.rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block the go/no-go and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_go_no_go && self.rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Rehearsal-row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose claim is
    /// already canonically narrowed is not a *go/no-go* blocker, it merely inherits the
    /// upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<RehearsalGapReason> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_go_no_go && self.rule_fires(rule))
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

    /// Recomputes the summary block from the rows and rehearsal rules.
    pub fn computed_summary(&self) -> GoNoGoRehearsalSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.rehearsal_packet.slo_state == state)
                .count()
        };
        let release_blocking: Vec<&RehearsalStageRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        let total_checkpoints: usize = self
            .rows
            .iter()
            .map(|row| row.rollback_checkpoints.len())
            .sum();
        let checkpoints_unverified: usize = self
            .rows
            .iter()
            .map(RehearsalStageRow::unverified_checkpoint_count)
            .sum();
        GoNoGoRehearsalSummary {
            total_rows: self.rows.len(),
            total_claims: self.claims().len(),
            rows_go: self.rows.iter().filter(|row| row.holds_stable()).count(),
            rows_no_go_below_cutline: self.rows.iter().filter(|row| !row.holds_stable()).count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_go: release_blocking
                .iter()
                .filter(|row| row.holds_stable())
                .count(),
            release_blocking_no_go: release_blocking
                .iter()
                .filter(|row| !row.holds_stable())
                .count(),
            rows_on_active_exception: self
                .rows
                .iter()
                .filter(|row| row.rehearsal_state == RehearsalState::GoOnException)
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_checkpoints,
            checkpoints_verified: total_checkpoints - checkpoints_unverified,
            checkpoints_unverified,
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

    /// Produces an export/Help-About-safe projection of the rehearsal that downstream
    /// surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> RehearsalExportProjection {
        RehearsalExportProjection {
            rehearsal_id: self.rehearsal_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| RehearsalExportRow {
                    entry_id: row.entry_id.clone(),
                    stage_kind: row.stage_kind,
                    subject_ref: row.subject_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    effective_label: row.effective_label,
                    holds_stable: row.holds_stable(),
                    rehearsal_state: row.rehearsal_state,
                    slo_state: row.rehearsal_packet.slo_state,
                    checkpoint_total: row.rollback_checkpoints.len(),
                    checkpoint_unverified: row.unverified_checkpoint_count(),
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the rehearsal, returning every violation found.
    pub fn validate(&self) -> Vec<GoNoGoRehearsalViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(GoNoGoRehearsalViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(GoNoGoRehearsalViolation::EmptyRehearsal);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(GoNoGoRehearsalViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<GoNoGoRehearsalViolation>) {
        if self.schema_version != GO_NO_GO_REHEARSAL_SCHEMA_VERSION {
            violations.push(GoNoGoRehearsalViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != GO_NO_GO_REHEARSAL_RECORD_KIND {
            violations.push(GoNoGoRehearsalViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("rehearsal_id", &self.rehearsal_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("stable_proof_index_ref", &self.stable_proof_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(GoNoGoRehearsalViolation::EmptyField {
                    entry_id: "<rehearsal>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(GoNoGoRehearsalViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.stage_kinds != StageKind::ALL.to_vec() {
            violations.push(GoNoGoRehearsalViolation::ClosedVocabularyMismatch {
                field: "stage_kinds",
            });
        }
        if self.rehearsal_states != RehearsalState::ALL.to_vec() {
            violations.push(GoNoGoRehearsalViolation::ClosedVocabularyMismatch {
                field: "rehearsal_states",
            });
        }
        if self.gap_reasons != RehearsalGapReason::ALL.to_vec() {
            violations.push(GoNoGoRehearsalViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.rehearsal_actions != RehearsalAction::ALL.to_vec() {
            violations.push(GoNoGoRehearsalViolation::ClosedVocabularyMismatch {
                field: "rehearsal_actions",
            });
        }
        if self.release_blocking_stage_refs.is_empty() {
            violations.push(GoNoGoRehearsalViolation::EmptyField {
                entry_id: "<rehearsal>".to_owned(),
                field_name: "release_blocking_stage_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(GoNoGoRehearsalViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(GoNoGoRehearsalViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(GoNoGoRehearsalViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(GoNoGoRehearsalViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<GoNoGoRehearsalViolation>) {
        if self.rules.is_empty() {
            violations.push(GoNoGoRehearsalViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(GoNoGoRehearsalViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(GoNoGoRehearsalViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(GoNoGoRehearsalViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every gap reason must have a rule, so a gap reason cannot fire without a
        // corresponding go/no-go gate.
        for reason in RehearsalGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(GoNoGoRehearsalViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &RehearsalStageRow,
        violations: &mut Vec<GoNoGoRehearsalViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("subject_ref", &row.subject_ref),
            ("subject_summary", &row.subject_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            (
                "rehearsal_packet.packet_id",
                &row.rehearsal_packet.packet_id,
            ),
            (
                "rehearsal_packet.packet_ref",
                &row.rehearsal_packet.packet_ref,
            ),
            (
                "rehearsal_packet.proof_index_ref",
                &row.rehearsal_packet.proof_index_ref,
            ),
            (
                "rehearsal_packet.freshness_slo.slo_register_ref",
                &row.rehearsal_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(GoNoGoRehearsalViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no row may carry a label wider than the public claim's canonical
        // label.
        if row.effective_label.rank() > row.claim_label.rank() {
            violations.push(GoNoGoRehearsalViolation::EffectiveWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                effective: row.effective_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn window
        // may not exceed it.
        if row.rehearsal_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(GoNoGoRehearsalViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "rehearsal_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.rehearsal_packet.freshness_slo.window_is_consistent() {
            violations.push(GoNoGoRehearsalViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        self.validate_checkpoints(row, violations);

        // A public claim whose canonical label is below the cutline forces the row to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(GoNoGoRehearsalViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if !row.has_active_reason(RehearsalGapReason::ClaimLabelNarrowed) {
                violations.push(GoNoGoRehearsalViolation::ClaimNarrowedWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        let slo_state = row.rehearsal_packet.slo_state;

        if row.holds_label() {
            // A Go row carries exactly the public claim's canonical label, carries no
            // active gap reason, rides a captured within-SLO packet, verifies every
            // rollback checkpoint, is owner-signed, and (for an on-exception row) relies on
            // an unexpired exception packet.
            if row.effective_label != row.claim_label {
                violations.push(GoNoGoRehearsalViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    effective: row.effective_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(GoNoGoRehearsalViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.rehearsal_packet.has_capture() {
                violations.push(GoNoGoRehearsalViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(GoNoGoRehearsalViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !row.all_checkpoints_verified() {
                violations.push(GoNoGoRehearsalViolation::HeldWithUnverifiedCheckpoint {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(GoNoGoRehearsalViolation::HeldWithoutOwnerSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            if row
                .exception_packet
                .as_ref()
                .is_some_and(|packet| packet.expires_at.as_str() <= self.as_of.as_str())
            {
                violations.push(GoNoGoRehearsalViolation::HeldOnExpiredException {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the effective label below the cutline and name at
            // least one active reason.
            if row.holds_stable() {
                violations.push(GoNoGoRehearsalViolation::EffectiveLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.rehearsal_state,
                    effective: row.effective_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(GoNoGoRehearsalViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.rehearsal_state,
                });
            }
            // A narrowing row whose packet is breached or missing must name the matching
            // freshness reason, so the freshness automation stays honest.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(RehearsalGapReason::RehearsalPacketFreshnessBreached)
            {
                violations.push(GoNoGoRehearsalViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(RehearsalGapReason::RehearsalPacketMissing)
            {
                violations.push(GoNoGoRehearsalViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_checkpoints(
        &self,
        row: &RehearsalStageRow,
        violations: &mut Vec<GoNoGoRehearsalViolation>,
    ) {
        if row.rollback_checkpoints.is_empty() {
            violations.push(GoNoGoRehearsalViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "rollback_checkpoints",
            });
        }
        let mut seen = BTreeSet::new();
        for checkpoint in &row.rollback_checkpoints {
            if !seen.insert(checkpoint.checkpoint_id.clone()) {
                violations.push(GoNoGoRehearsalViolation::DuplicateCheckpointId {
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
                    violations.push(GoNoGoRehearsalViolation::EmptyField {
                        entry_id: row.entry_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
        if row.unverified_checkpoint_count() > 0
            && !row.has_active_reason(RehearsalGapReason::RollbackCheckpointUnverified)
        {
            violations.push(
                GoNoGoRehearsalViolation::UnverifiedCheckpointWithoutReason {
                    entry_id: row.entry_id.clone(),
                },
            );
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &RehearsalStageRow,
        violations: &mut Vec<GoNoGoRehearsalViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<GoNoGoRehearsalViolation>,
                               expected: RehearsalGapReason| {
            violations.push(GoNoGoRehearsalViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.rehearsal_state,
                expected_reason: expected,
            });
        };

        match row.rehearsal_state {
            RehearsalState::NoGoUnrehearsed => {
                const ALLOWED: [RehearsalGapReason; 3] = [
                    RehearsalGapReason::RehearsalEvidenceIncomplete,
                    RehearsalGapReason::OwnerSignoffMissing,
                    RehearsalGapReason::RollbackCheckpointUnverified,
                ];
                if !ALLOWED.iter().any(|reason| row.has_active_reason(*reason)) {
                    push_incoherent(violations, RehearsalGapReason::RehearsalEvidenceIncomplete);
                }
            }
            RehearsalState::NoGoClaimNarrowed => {
                if !row.has_active_reason(RehearsalGapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, RehearsalGapReason::ClaimLabelNarrowed);
                }
            }
            RehearsalState::NoGoStale => {
                if !(row.has_active_reason(RehearsalGapReason::RehearsalPacketFreshnessBreached)
                    || row.has_active_reason(RehearsalGapReason::RehearsalPacketMissing))
                {
                    push_incoherent(
                        violations,
                        RehearsalGapReason::RehearsalPacketFreshnessBreached,
                    );
                }
            }
            RehearsalState::NoGoExceptionExpired => {
                if !row.has_active_reason(RehearsalGapReason::ExceptionExpired) {
                    push_incoherent(violations, RehearsalGapReason::ExceptionExpired);
                }
                if row.exception_packet.is_none() {
                    violations.push(GoNoGoRehearsalViolation::ExceptionStateWithoutPacket {
                        entry_id: row.entry_id.clone(),
                        state: row.rehearsal_state,
                    });
                }
            }
            RehearsalState::GoOnException => {
                if row
                    .exception_packet
                    .as_ref()
                    .map(|packet| {
                        packet.waiver_ref.trim().is_empty() || packet.expires_at.trim().is_empty()
                    })
                    .unwrap_or(true)
                {
                    violations.push(GoNoGoRehearsalViolation::ExceptionStateWithoutPacket {
                        entry_id: row.entry_id.clone(),
                        state: row.rehearsal_state,
                    });
                }
            }
            RehearsalState::GoRehearsed => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<GoNoGoRehearsalViolation>) {
        // Each subject ref appears at most once: a stage has one canonical row.
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.subject_ref.as_str()) {
                violations.push(GoNoGoRehearsalViolation::DuplicateSubjectRef {
                    subject_ref: row.subject_ref.clone(),
                });
            }
        }

        // The release line must cover every declared release-blocking stage with exactly
        // one release-blocking row, and every release-blocking row must be declared, so a
        // stage cannot quietly drop out of the rehearsal.
        let declared: BTreeSet<&str> = self
            .release_blocking_stage_refs
            .iter()
            .map(String::as_str)
            .collect();
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.entry_id.as_str())
            .collect();
        for declared_ref in &declared {
            if !covered.contains(declared_ref) {
                violations.push(GoNoGoRehearsalViolation::ReleaseBlockingRefWithoutRow {
                    entry_id: (*declared_ref).to_owned(),
                });
            }
        }
        for row in &self.rows {
            if row.release_blocking && !declared.contains(row.entry_id.as_str()) {
                violations.push(GoNoGoRehearsalViolation::ReleaseBlockingRowNotInSet {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        // The rehearsal must cover all four stage kinds — cutline review, promotion step,
        // rollback checkpoint, and exception review — so the release line cannot rehearse
        // some stages and silently leave a whole stage kind unexercised.
        for kind in StageKind::ALL {
            if self.rows_for_kind(kind).is_empty() {
                violations.push(GoNoGoRehearsalViolation::StageKindAbsent { kind });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<GoNoGoRehearsalViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(GoNoGoRehearsalViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(GoNoGoRehearsalViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(GoNoGoRehearsalViolation::PublicationDecisionInconsistent {
                declared: self.publication.decision,
                computed,
            });
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(GoNoGoRehearsalViolation::PublicationBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.publication.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(GoNoGoRehearsalViolation::PublicationBlockingSetMismatch {
                field: "blocking_entry_ids",
            });
        }
    }
}

/// A redaction-safe export row projected from the rehearsal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RehearsalExportRow {
    /// Stable rehearsal-row id.
    pub entry_id: String,
    /// Rehearsal stage kind.
    pub stage_kind: StageKind,
    /// Rehearsed stage subject ref.
    pub subject_ref: String,
    /// Whether the stage is part of the release-blocking set.
    pub release_blocking: bool,
    /// The public-claim entry ref the row backs.
    pub claim_ref: String,
    /// The public claim's canonical ceiling label.
    pub claim_label: StableClaimLevel,
    /// Lifecycle label the row carries.
    pub effective_label: StableClaimLevel,
    /// Whether the row carries a label at or above the cutline.
    pub holds_stable: bool,
    /// Rehearsal state.
    pub rehearsal_state: RehearsalState,
    /// Rehearsal-packet freshness-SLO state.
    pub slo_state: FreshnessSloState,
    /// Total required rollback checkpoints.
    pub checkpoint_total: usize,
    /// Required rollback checkpoints that are unverified.
    pub checkpoint_unverified: usize,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<RehearsalGapReason>,
}

/// A redaction-safe export projection of the rehearsal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RehearsalExportProjection {
    /// Rehearsal id this projection was produced from.
    pub rehearsal_id: String,
    /// Rehearsal as-of date.
    pub as_of: String,
    /// Go/no-go verdict.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<RehearsalExportRow>,
}

/// A validation violation for the final go/no-go rehearsal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoNoGoRehearsalViolation {
    /// The rehearsal carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the rehearsal.
        actual: u32,
    },
    /// The rehearsal carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the rehearsal.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The rehearsal has no rows.
    EmptyRehearsal,
    /// The rehearsal has no rules.
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
    /// A checkpoint id appears more than once in a row.
    DuplicateCheckpointId {
        /// Row id.
        entry_id: String,
        /// Duplicate checkpoint id.
        checkpoint_id: String,
    },
    /// A subject ref appears on more than one row.
    DuplicateSubjectRef {
        /// Duplicate subject ref.
        subject_ref: String,
    },
    /// A rehearsal rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: RehearsalGapReason,
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
    /// A Go row is backed by a public claim already below the cutline.
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
    /// A Go row carries a label different from its claim ceiling.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Claim ceiling.
        claim: StableClaimLevel,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A Go row carries an active gap reason.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A Go row has no captured, evidence-backed packet.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A Go row rides a stale or missing packet.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// SLO state.
        slo_state: FreshnessSloState,
    },
    /// A Go row carries an unverified rollback checkpoint.
    HeldWithUnverifiedCheckpoint {
        /// Row id.
        entry_id: String,
    },
    /// A Go row lacks owner sign-off.
    HeldWithoutOwnerSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A Go row relies on an expired exception packet.
    HeldOnExpiredException {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing state did not narrow below the cutline.
    EffectiveLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: RehearsalState,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A narrowing state carries no reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: RehearsalState,
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
    /// An unverified rollback checkpoint does not name the unverified-checkpoint reason.
    UnverifiedCheckpointWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A row state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: RehearsalState,
        /// Expected reason.
        expected_reason: RehearsalGapReason,
    },
    /// An exception-bearing state names no exception packet.
    ExceptionStateWithoutPacket {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: RehearsalState,
    },
    /// A declared release-blocking stage has no row.
    ReleaseBlockingRefWithoutRow {
        /// Missing row id.
        entry_id: String,
    },
    /// A release-blocking row was not declared.
    ReleaseBlockingRowNotInSet {
        /// Row id.
        entry_id: String,
    },
    /// A stage kind is absent from the rehearsal.
    StageKindAbsent {
        /// Missing stage kind.
        kind: StageKind,
    },
    /// The declared go/no-go decision disagrees with the computed decision.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared go/no-go blocking set disagrees with computed rules.
    PublicationBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// Summary counts disagree with rows.
    SummaryMismatch,
}

impl fmt::Display for GoNoGoRehearsalViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported rehearsal schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported rehearsal record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "rehearsal {field} is not the canonical value")
            }
            Self::EmptyRehearsal => write!(f, "rehearsal has no rows"),
            Self::NoRules => write!(f, "rehearsal has no rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate rehearsal row id {entry_id}")
            }
            Self::DuplicateRuleId { rule_id } => write!(f, "duplicate rule id {rule_id}"),
            Self::DuplicateCheckpointId {
                entry_id,
                checkpoint_id,
            } => write!(f, "rehearsal row {entry_id} repeats checkpoint {checkpoint_id}"),
            Self::DuplicateSubjectRef { subject_ref } => {
                write!(f, "subject ref {subject_ref} appears on more than one row")
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
                "rehearsal row {entry_id} effective label {} is wider than claim ceiling {}",
                effective.as_str(),
                claim.as_str()
            ),
            Self::FreshnessSloInconsistent { entry_id } => write!(
                f,
                "rehearsal row {entry_id} freshness SLO warn window exceeds target age"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "rehearsal row {entry_id} returns Go while claim label {} is below the cutline",
                claim.as_str()
            ),
            Self::ClaimNarrowedWithoutReason { entry_id } => write!(
                f,
                "rehearsal row {entry_id} backs a narrowed claim without naming claim_label_narrowed"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                effective,
            } => write!(
                f,
                "rehearsal row {entry_id} returns Go at {} but claim ceiling is {}",
                effective.as_str(),
                claim.as_str()
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(
                    f,
                    "rehearsal row {entry_id} returns Go while a gap reason is active"
                )
            }
            Self::HeldWithoutFreshPacket { entry_id } => write!(
                f,
                "rehearsal row {entry_id} returns Go with no captured, evidence-backed packet"
            ),
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => write!(
                f,
                "rehearsal row {entry_id} returns Go while packet is {}",
                slo_state.as_str()
            ),
            Self::HeldWithUnverifiedCheckpoint { entry_id } => write!(
                f,
                "rehearsal row {entry_id} returns Go with an unverified rollback checkpoint"
            ),
            Self::HeldWithoutOwnerSignoff { entry_id } => {
                write!(f, "rehearsal row {entry_id} returns Go without owner sign-off")
            }
            Self::HeldOnExpiredException { entry_id } => {
                write!(
                    f,
                    "rehearsal row {entry_id} returns Go on an expired exception packet"
                )
            }
            Self::EffectiveLabelNotNarrowed {
                entry_id,
                state,
                effective,
            } => write!(
                f,
                "rehearsal row {entry_id} state {} must narrow but holds {}",
                state.as_str(),
                effective.as_str()
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "rehearsal row {entry_id} state {} narrows without a reason",
                state.as_str()
            ),
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "rehearsal row {entry_id} has a breached packet without the freshness reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "rehearsal row {entry_id} has a missing packet without the missing-packet reason"
            ),
            Self::UnverifiedCheckpointWithoutReason { entry_id } => write!(
                f,
                "rehearsal row {entry_id} has an unverified checkpoint without rollback_checkpoint_unverified"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "rehearsal row {entry_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::ExceptionStateWithoutPacket { entry_id, state } => write!(
                f,
                "rehearsal row {entry_id} state {} names no exception packet",
                state.as_str()
            ),
            Self::ReleaseBlockingRefWithoutRow { entry_id } => {
                write!(f, "declared release-blocking stage {entry_id} has no row")
            }
            Self::ReleaseBlockingRowNotInSet { entry_id } => {
                write!(f, "release-blocking stage {entry_id} is not declared")
            }
            Self::StageKindAbsent { kind } => {
                write!(f, "stage kind {} is covered by no row", kind.as_str())
            }
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "go/no-go decision {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "go/no-go {field} disagrees with firing rules")
            }
            Self::SummaryMismatch => write!(f, "rehearsal summary counts disagree with rows"),
        }
    }
}

impl Error for GoNoGoRehearsalViolation {}

/// Loads the embedded final go/no-go rehearsal.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in rehearsal no longer matches
/// [`GoNoGoRehearsal`].
pub fn current_go_no_go_rehearsal() -> Result<GoNoGoRehearsal, serde_json::Error> {
    serde_json::from_str(GO_NO_GO_REHEARSAL_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rehearsal() -> GoNoGoRehearsal {
        current_go_no_go_rehearsal().expect("go/no-go rehearsal parse")
    }

    #[test]
    fn embedded_rehearsal_parses_and_validates() {
        let rehearsal = rehearsal();
        assert_eq!(rehearsal.schema_version, GO_NO_GO_REHEARSAL_SCHEMA_VERSION);
        assert_eq!(rehearsal.record_kind, GO_NO_GO_REHEARSAL_RECORD_KIND);
        assert_eq!(rehearsal.validate(), Vec::new());
    }

    #[test]
    fn every_stage_kind_is_covered() {
        let rehearsal = rehearsal();
        for kind in StageKind::ALL {
            assert!(
                !rehearsal.rows_for_kind(kind).is_empty(),
                "{} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let rehearsal = rehearsal();
        assert_eq!(rehearsal.summary, rehearsal.computed_summary());
        assert_eq!(
            rehearsal.summary.rows_go + rehearsal.summary.rows_no_go_below_cutline,
            rehearsal.rows.len()
        );
    }

    #[test]
    fn go_no_go_proceeds_without_blocking_rules() {
        let rehearsal = rehearsal();
        assert_eq!(rehearsal.publication.decision, PromotionDecision::Proceed);
        assert_eq!(
            rehearsal.publication.decision,
            rehearsal.computed_publication_decision()
        );
        assert!(rehearsal.publication.blocking_rule_ids.is_empty());
        assert!(rehearsal.publication.blocking_entry_ids.is_empty());
    }

    #[test]
    fn go_row_on_breached_packet_fails() {
        let mut rehearsal = rehearsal();
        let row = rehearsal
            .rows
            .iter_mut()
            .find(|row| row.holds_label())
            .expect("a Go row exists");
        row.rehearsal_packet.slo_state = FreshnessSloState::Breached;
        rehearsal.summary = rehearsal.computed_summary();
        assert!(rehearsal.validate().iter().any(|violation| matches!(
            violation,
            GoNoGoRehearsalViolation::HeldOnStalePacket { .. }
        )));
    }

    #[test]
    fn go_row_with_unverified_checkpoint_fails() {
        let mut rehearsal = rehearsal();
        let row = rehearsal
            .rows
            .iter_mut()
            .find(|row| row.holds_label())
            .expect("a Go row exists");
        row.rollback_checkpoints[0].verified = false;
        rehearsal.summary = rehearsal.computed_summary();
        assert!(rehearsal.validate().iter().any(|violation| matches!(
            violation,
            GoNoGoRehearsalViolation::HeldWithUnverifiedCheckpoint { .. }
        )));
    }

    #[test]
    fn rehearsal_rows_go_without_narrowing() {
        let rehearsal = rehearsal();
        assert!(
            rehearsal.rows.iter().all(RehearsalStageRow::holds_stable),
            "clean rehearsal must not narrow a row"
        );
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let rehearsal = rehearsal();
        let projection = rehearsal.support_export_projection();
        assert_eq!(projection.rows.len(), rehearsal.rows.len());
        assert_eq!(
            projection.publication_decision,
            rehearsal.publication.decision
        );
        for (row, projected) in rehearsal.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, projected.entry_id);
            assert_eq!(row.holds_stable(), projected.holds_stable);
            assert_eq!(
                row.unverified_checkpoint_count(),
                projected.checkpoint_unverified
            );
        }
    }
}
