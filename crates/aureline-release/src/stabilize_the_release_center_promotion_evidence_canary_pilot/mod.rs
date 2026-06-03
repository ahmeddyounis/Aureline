//! Typed release-center promotion evidence with canary→pilot→broad ring control,
//! soak windows, rollback-stop triggers, and kill-switch posture.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes; the
//! [`stable_qualification_matrix`](crate::stable_qualification_matrix) decides
//! whether each per-lane qualification row holds its claimed level. None of them
//! answer the question this module answers: **for each promotion subject — binary
//! artifact graph or non-binary AI/provider/model/prompt/tool pack — which ring
//! does it currently occupy, has it completed the required soak to widen to the
//! next ring, and what rollback-stop or kill-switch posture protects the cohort
//! if proof ages out or a regression appears mid-soak?** This module is the
//! **ring promotion control register**. For every promotion subject it records one
//! row that binds the subject to the [`stable_claim_manifest`](crate::stable_claim_manifest)
//! entry whose lifecycle label it backs, the ring it currently occupies, the soak
//! windows and widening criteria that govern the next ring, the rollback-stop
//! triggers that may fire, the kill-switch posture, the proof packet that grounds
//! it, and the waiver (if any) holding it provisionally.
//!
//! Each [`PromotionSubjectRow`] is one `(promotion subject, public claim, ring)`
//! binding. It:
//!
//! - names the subject it governs ([`PromotionSubjectRow::subject_kind`],
//!   [`PromotionSubjectRow::subject_ref`], [`PromotionSubjectRow::subject_summary`])
//!   and whether that subject is part of the release-blocking set
//!   ([`PromotionSubjectRow::release_blocking`]);
//! - pins the current ring ([`PromotionSubjectRow::current_ring`]), the target
//!   ring ([`PromotionSubjectRow::target_ring`]), and the promotion state earned
//!   ([`PromotionSubjectRow::promotion_state`]);
//! - records the soak windows ([`SoakWindow`]) that must complete before the
//!   subject may widen from internal→canary, canary→pilot, or pilot→broad, each
//!   with minimum duration, required evidence refs, and completion status;
//! - records the rollback-stop triggers ([`RollbackStopTrigger`]) that may fire
//!   per ring, each with a trigger kind, ring scope, and whether auto-rollback
//!   is enabled;
//! - records the kill-switch posture ([`KillSwitchPosture`]) and the rollback
//!   target ring so shiproom can reconstruct the exact fallback chain;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry
//!   whose public claim it backs ([`PromotionSubjectRow::claim_ref`]) and the
//!   canonical lifecycle label that entry publishes
//!   ([`PromotionSubjectRow::claim_label`]);
//! - records the effective label after narrowing
//!   ([`PromotionSubjectRow::effective_label`]), the active gap reasons
//!   ([`GapReason`]), the proof packet, owner sign-off, and optional waiver.
//!
//! The [`PromotionRule`] set names the closed conditions that gate promotion,
//! and [`RingPromotionControl::publication`] records the proceed/hold verdict.
//!
//! The artifact is checked in at
//! `artifacts/release/stabilize_the_release_center_promotion_evidence_canary_pilot.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.
//! Date arithmetic (packet-freshness recomputation and waiver expiry against an
//! `as_of` date) lives in the CI gate; this model enforces the structural and
//! logical invariants that hold regardless of the clock — narrowing consistency,
//! the no-widening rule, ring ordering, soak completion before widening,
//! packet/state coherence, owner sign-off on current rows, rule wiring, and the
//! publication verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::ProofPacket;
use crate::stable_claim_matrix::{OwnerSignoff, QualificationWaiver, StableClaimLevel};

/// Supported ring promotion control schema version.
pub const RING_PROMOTION_CONTROL_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the artifact.
pub const RING_PROMOTION_CONTROL_RECORD_KIND: &str = "ring_promotion_control";

/// Repo-relative path to the checked-in artifact.
pub const RING_PROMOTION_CONTROL_PATH: &str =
    "artifacts/release/stabilize_the_release_center_promotion_evidence_canary_pilot.json";

/// Embedded checked-in artifact JSON.
pub const RING_PROMOTION_CONTROL_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stabilize_the_release_center_promotion_evidence_canary_pilot.json"
));

/// Promotion ring a subject may occupy, in widening order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Ring {
    /// Internal validation ring before canary.
    Internal,
    /// Canary cohort — smallest exposure, fastest signal.
    Canary,
    /// Pilot cohort — managed partner or expanded internal exposure.
    Pilot,
    /// Broad stable rollout.
    Broad,
}

impl Ring {
    /// Every ring, in widening order.
    pub const ALL: [Self; 4] = [Self::Internal, Self::Canary, Self::Pilot, Self::Broad];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::Canary => "canary",
            Self::Pilot => "pilot",
            Self::Broad => "broad",
        }
    }

    /// Strength rank; higher is a wider ring.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Internal => 0,
            Self::Canary => 1,
            Self::Pilot => 2,
            Self::Broad => 3,
        }
    }

    /// Returns the next wider ring, if any.
    pub const fn next(self) -> Option<Self> {
        match self {
            Self::Internal => Some(Self::Canary),
            Self::Canary => Some(Self::Pilot),
            Self::Pilot => Some(Self::Broad),
            Self::Broad => None,
        }
    }
}

/// Kind of promotion subject governed by this register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionSubjectKind {
    /// Binary artifact graph (IDE, CLI, remote agent, symbols, etc.).
    BinaryArtifactGraph,
    /// AI provider pack (provider configuration, routing, fallback).
    AiProviderPack,
    /// AI model pack (model weights, quantization, runtime binding).
    AiModelPack,
    /// AI prompt pack (system prompts, prompt templates, guardrails).
    AiPromptPack,
    /// AI tool pack (tool definitions, tool schemas, execution bindings).
    AiToolPack,
}

impl PromotionSubjectKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::BinaryArtifactGraph,
        Self::AiProviderPack,
        Self::AiModelPack,
        Self::AiPromptPack,
        Self::AiToolPack,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BinaryArtifactGraph => "binary_artifact_graph",
            Self::AiProviderPack => "ai_provider_pack",
            Self::AiModelPack => "ai_model_pack",
            Self::AiPromptPack => "ai_prompt_pack",
            Self::AiToolPack => "ai_tool_pack",
        }
    }
}

/// Promotion state a subject earned for its current ring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// The subject is currently soaking in its ring; widening to the next ring
    /// is blocked until the soak window completes.
    Soaking,
    /// The subject has completed soak and holds its ring with current evidence.
    Qualified,
    /// The subject carries its ring only because an active, unexpired waiver
    /// covers a recorded residual gap.
    ProvisionalOnWaiver,
    /// The subject is blocked from widening by an active gap; it stays in its
    /// current ring until the gap clears.
    Blocked,
    /// The proof packet breached its freshness SLO; the effective label must
    /// narrow and the subject may need to roll back to a narrower ring.
    NarrowedStale,
    /// No proof packet has been captured; the effective label must narrow.
    NarrowedMissing,
    /// A regression was detected mid-soak or in the current ring; the effective
    /// label must narrow.
    NarrowedRegressed,
    /// The row relied on a waiver that has expired; the effective label must
    /// narrow.
    NarrowedWaiverExpired,
    /// The subject has been explicitly rolled back to its rollback target ring.
    RolledBack,
}

impl PromotionState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Soaking,
        Self::Qualified,
        Self::ProvisionalOnWaiver,
        Self::Blocked,
        Self::NarrowedStale,
        Self::NarrowedMissing,
        Self::NarrowedRegressed,
        Self::NarrowedWaiverExpired,
        Self::RolledBack,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Soaking => "soaking",
            Self::Qualified => "qualified",
            Self::ProvisionalOnWaiver => "provisional_on_waiver",
            Self::Blocked => "blocked",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedMissing => "narrowed_missing",
            Self::NarrowedRegressed => "narrowed_regressed",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
            Self::RolledBack => "rolled_back",
        }
    }

    /// Whether the state lets the subject carry its claimed ring.
    pub const fn holds_ring(self) -> bool {
        matches!(self, Self::Qualified | Self::ProvisionalOnWaiver | Self::Soaking)
    }

    /// Whether the state forces narrowing or rollback.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_ring()
    }
}

/// Closed reason a subject narrows, blocks, or a rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The backing public claim narrowed below the cutline.
    ClaimLabelNarrowed,
    /// The required soak window is incomplete.
    SoakIncomplete,
    /// The proof packet breached its freshness SLO.
    EvidenceStale,
    /// No proof packet has been captured.
    EvidenceMissing,
    /// A regression was detected mid-soak or in the current ring.
    RegressionDetected,
    /// A waiver the row relied on has expired.
    WaiverExpired,
    /// The required row owner sign-off is missing.
    OwnerSignoffMissing,
    /// A rollback-stop trigger fired.
    RollbackStopTriggered,
    /// The kill switch is armed and blocks widening.
    KillSwitchArmed,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ClaimLabelNarrowed,
        Self::SoakIncomplete,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::RegressionDetected,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
        Self::RollbackStopTriggered,
        Self::KillSwitchArmed,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::SoakIncomplete => "soak_incomplete",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceMissing => "evidence_missing",
            Self::RegressionDetected => "regression_detected",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::RollbackStopTriggered => "rollback_stop_triggered",
            Self::KillSwitchArmed => "kill_switch_armed",
        }
    }
}

/// Default action a rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    /// Hold widening to the next ring until the condition clears.
    HoldWidening,
    /// Narrow the public claim below the stable cutline.
    NarrowClaim,
    /// Refresh the proof packet.
    RefreshPacket,
    /// Recapture the subject evidence.
    RecaptureEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
    /// Trigger rollback to the rollback target ring.
    TriggerRollback,
    /// Arm the kill switch for this subject.
    ArmKillSwitch,
}

impl Action {
    /// Every action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::HoldWidening,
        Self::NarrowClaim,
        Self::RefreshPacket,
        Self::RecaptureEvidence,
        Self::RequestOwnerSignoff,
        Self::TriggerRollback,
        Self::ArmKillSwitch,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldWidening => "hold_widening",
            Self::NarrowClaim => "narrow_claim",
            Self::RefreshPacket => "refresh_packet",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::TriggerRollback => "trigger_rollback",
            Self::ArmKillSwitch => "arm_kill_switch",
        }
    }
}

/// Kill-switch posture for a promotion subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KillSwitchPosture {
    /// Kill switch is disabled; normal promotion flow applies.
    Disabled,
    /// Kill switch is armed; it will engage if a rollback-stop trigger fires.
    Armed,
    /// Kill switch is enabled; the subject is actively blocked from widening.
    Enabled,
}

impl KillSwitchPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 3] = [Self::Disabled, Self::Armed, Self::Enabled];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Armed => "armed",
            Self::Enabled => "enabled",
        }
    }
}

/// Kind of rollback-stop trigger that may fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackTriggerKind {
    /// Crash or exception rate regression.
    Crash,
    /// Data loss or corruption signal.
    DataLoss,
    /// Trust boundary regression (auth, attestation, signing).
    TrustRegression,
    /// Protected-path regression (update, install, recovery).
    ProtectedPathRegression,
    /// Performance regression beyond published budget.
    PerformanceRegression,
    /// Compatibility failure with claimed matrix rows.
    CompatibilityFailure,
    /// Update or install failure.
    UpdateFailure,
}

impl RollbackTriggerKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Crash,
        Self::DataLoss,
        Self::TrustRegression,
        Self::ProtectedPathRegression,
        Self::PerformanceRegression,
        Self::CompatibilityFailure,
        Self::UpdateFailure,
    ];

    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Crash => "crash",
            Self::DataLoss => "data_loss",
            Self::TrustRegression => "trust_regression",
            Self::ProtectedPathRegression => "protected_path_regression",
            Self::PerformanceRegression => "performance_regression",
            Self::CompatibilityFailure => "compatibility_failure",
            Self::UpdateFailure => "update_failure",
        }
    }
}

/// One soak window governing a ring transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoakWindow {
    /// The ring transition this soak window governs (e.g. "canary_to_pilot").
    pub transition: String,
    /// Minimum duration in hours before the transition may complete.
    pub minimum_duration_hours: u32,
    /// Evidence refs required before the soak may complete.
    #[serde(default)]
    pub required_evidence_refs: Vec<String>,
    /// Fitness-check refs required before the soak may complete.
    #[serde(default)]
    pub required_fitness_checks: Vec<String>,
    /// UTC date the soak window started.
    pub started_at: String,
    /// UTC date the soak window completed, if it has.
    #[serde(default)]
    pub completed_at: Option<String>,
    /// Whether the soak window is complete.
    pub is_complete: bool,
}

/// One rollback-stop trigger attached to a promotion subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackStopTrigger {
    /// Stable trigger id.
    pub trigger_id: String,
    /// Human-readable title.
    pub title: String,
    /// The kind of trigger.
    pub trigger_kind: RollbackTriggerKind,
    /// Rings where this trigger is active.
    pub ring_scope: Vec<Ring>,
    /// Whether this trigger automatically rolls back when fired.
    pub auto_rollback: bool,
    /// Reviewable reason this trigger exists.
    pub rationale: String,
}

/// One promotion subject row in the ring promotion control register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionSubjectRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The kind of promotion subject this row governs.
    pub subject_kind: PromotionSubjectKind,
    /// Ref to the subject (manifest, package, or channel family) this row speaks for.
    pub subject_ref: String,
    /// Human-readable summary of the subject.
    pub subject_summary: String,
    /// Whether this subject is part of the release-blocking set.
    pub release_blocking: bool,
    /// Ref to the stable claim manifest entry this row backs.
    pub claim_ref: String,
    /// Canonical lifecycle label the claim entry publishes.
    pub claim_label: StableClaimLevel,
    /// Current ring the subject occupies.
    pub current_ring: Ring,
    /// Target ring the subject is widening toward.
    pub target_ring: Ring,
    /// Promotion state earned.
    pub promotion_state: PromotionState,
    /// Soak windows governing ring transitions.
    #[serde(default)]
    pub soak_windows: Vec<SoakWindow>,
    /// Rollback-stop triggers active for this subject.
    #[serde(default)]
    pub rollback_stop_triggers: Vec<RollbackStopTrigger>,
    /// Kill-switch posture.
    pub kill_switch_posture: KillSwitchPosture,
    /// Rollback target ring when a trigger fires or kill switch engages.
    pub rollback_target_ring: Ring,
    /// Proof packet grounding this row.
    pub proof_packet: ProofPacket,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Waiver authorizing a provisional hold, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Active gap reasons narrowing or blocking the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// The label the row effectively publishes after narrowing.
    pub effective_label: StableClaimLevel,
    /// Publication destinations that render this row.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl PromotionSubjectRow {
    /// True when the row's effective label is not a narrowing label relative to
    /// the claim.
    pub fn holds_claim(&self) -> bool {
        self.effective_label.rank() >= self.claim_label.rank()
            && self.promotion_state.holds_ring()
    }

    /// True when the subject is currently soaking.
    pub fn is_soaking(&self) -> bool {
        self.promotion_state == PromotionState::Soaking
    }

    /// True when the subject has completed all required soak windows up to its
    /// target ring.
    pub fn soak_complete_for_target(&self) -> bool {
        self.soak_windows.iter().all(|w| {
            if self.transition_is_active(&w.transition) {
                w.is_complete
            } else {
                true
            }
        })
    }

    /// True when the transition is part of the path from current to target ring.
    fn transition_is_active(&self, transition: &str) -> bool {
        let expected = format!("{}_to_{}", self.current_ring.as_str(), self.target_ring.as_str());
        transition == expected
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: GapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// One rule: a closed condition that narrows a claim or blocks widening.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a row fires this rule.
    pub trigger_reason: GapReason,
    /// Promotion states this rule watches.
    pub applies_to_states: Vec<PromotionState>,
    /// Default action prescribed when the rule fires.
    pub default_action: Action,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// The recorded publication verdict for the ring promotion control lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionDecision {
    /// The ring promotion line may publish.
    Proceed,
    /// Promotion is blocked by one or more firing rules.
    Hold,
}

impl PromotionDecision {
    /// Stable token recorded in the artifact.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Hold => "hold",
        }
    }
}

/// The recorded publication verdict for the ring promotion control lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionPublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Rule ids that block promotion, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_row_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RingPromotionControlSummary {
    /// Total number of promotion subject rows.
    pub total_rows: usize,
    /// Rows effectively holding their claim.
    pub rows_holding_claim: usize,
    /// Rows narrowed below their claim.
    pub rows_narrowed: usize,
    /// Rows holding claim via an active waiver.
    pub rows_on_active_waiver: usize,
    /// Rows currently soaking.
    pub rows_soaking: usize,
    /// Rows blocked from widening.
    pub rows_blocked: usize,
    /// Rows rolled back.
    pub rows_rolled_back: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of rules currently firing.
    pub rules_firing: usize,
    /// Subjects at canary ring.
    pub canary_subjects: usize,
    /// Subjects at pilot ring.
    pub pilot_subjects: usize,
    /// Subjects at broad ring.
    pub broad_subjects: usize,
    /// Subjects with armed or enabled kill switch.
    pub kill_switch_active: usize,
}

/// The typed ring promotion control artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RingPromotionControl {
    /// Artifact schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable artifact identifier.
    pub artifact_id: String,
    /// Lifecycle status of this artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this artifact ingests.
    pub claim_manifest_ref: String,
    /// Ref to the stable proof index this artifact ingests.
    pub stable_proof_index_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed ring vocabulary.
    pub rings: Vec<Ring>,
    /// Closed subject-kind vocabulary.
    pub subject_kinds: Vec<PromotionSubjectKind>,
    /// Closed promotion-state vocabulary.
    pub promotion_states: Vec<PromotionState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Closed action vocabulary.
    pub actions: Vec<Action>,
    /// Closed kill-switch posture vocabulary.
    pub kill_switch_postures: Vec<KillSwitchPosture>,
    /// Closed rollback-trigger-kind vocabulary.
    pub rollback_trigger_kinds: Vec<RollbackTriggerKind>,
    /// Release-blocking subject refs.
    pub release_blocking_subject_refs: Vec<String>,
    /// Promotion subject rows.
    pub rows: Vec<PromotionSubjectRow>,
    /// Promotion rules.
    pub rules: Vec<PromotionRule>,
    /// Recorded publication verdict.
    pub publication: PromotionPublicationRecord,
    /// Summary counts.
    pub summary: RingPromotionControlSummary,
}

impl RingPromotionControl {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&PromotionSubjectRow> {
        self.rows.iter().find(|r| r.entry_id == entry_id)
    }

    /// Returns the rows effectively holding their claim.
    pub fn rows_holding_claim(&self) -> Vec<&PromotionSubjectRow> {
        self.rows.iter().filter(|r| r.holds_claim()).collect()
    }

    /// Returns the rows narrowed below their claim.
    pub fn rows_narrowed(&self) -> Vec<&PromotionSubjectRow> {
        self.rows.iter().filter(|r| !r.holds_claim()).collect()
    }

    /// Returns the rows currently soaking.
    pub fn rows_soaking(&self) -> Vec<&PromotionSubjectRow> {
        self.rows.iter().filter(|r| r.is_soaking()).collect()
    }

    /// Returns the rows blocked from widening.
    pub fn rows_blocked(&self) -> Vec<&PromotionSubjectRow> {
        self.rows
            .iter()
            .filter(|r| r.promotion_state == PromotionState::Blocked)
            .collect()
    }

    /// Returns the rows rolled back.
    pub fn rows_rolled_back(&self) -> Vec<&PromotionSubjectRow> {
        self.rows
            .iter()
            .filter(|r| r.promotion_state == PromotionState::RolledBack)
            .collect()
    }

    /// Returns rows at a given ring.
    pub fn rows_at_ring(&self, ring: Ring) -> Vec<&PromotionSubjectRow> {
        self.rows.iter().filter(|r| r.current_ring == ring).collect()
    }

    /// True when `rule` fires: a row in its watch set carries its trigger reason.
    pub fn rule_fires(&self, rule: &PromotionRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_states.contains(&row.promotion_state)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .rules
            .iter()
            .any(|rule| rule.blocks_promotion && self.rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block promotion and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Row ids that trigger a blocking, firing rule, sorted and unique.
    pub fn computed_blocking_row_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<GapReason> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row
                .active_gap_reasons
                .iter()
                .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and rules.
    pub fn computed_summary(&self) -> RingPromotionControlSummary {
        RingPromotionControlSummary {
            total_rows: self.rows.len(),
            rows_holding_claim: self.rows.iter().filter(|r| r.holds_claim()).count(),
            rows_narrowed: self.rows.iter().filter(|r| !r.holds_claim()).count(),
            rows_on_active_waiver: self
                .rows
                .iter()
                .filter(|r| r.promotion_state == PromotionState::ProvisionalOnWaiver)
                .count(),
            rows_soaking: self.rows.iter().filter(|r| r.is_soaking()).count(),
            rows_blocked: self.rows.iter().filter(|r| r.promotion_state == PromotionState::Blocked).count(),
            rows_rolled_back: self
                .rows
                .iter()
                .filter(|r| r.promotion_state == PromotionState::RolledBack)
                .count(),
            total_active_gap_reasons: self.rows.iter().map(|r| r.active_gap_reasons.len()).sum(),
            rules_firing: self.rules.iter().filter(|rule| self.rule_fires(rule)).count(),
            canary_subjects: self.rows_at_ring(Ring::Canary).len(),
            pilot_subjects: self.rows_at_ring(Ring::Pilot).len(),
            broad_subjects: self.rows_at_ring(Ring::Broad).len(),
            kill_switch_active: self
                .rows
                .iter()
                .filter(|r| {
                    r.kill_switch_posture == KillSwitchPosture::Armed
                        || r.kill_switch_posture == KillSwitchPosture::Enabled
                })
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the artifact that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> RingPromotionControlExportProjection {
        RingPromotionControlExportProjection {
            artifact_id: self.artifact_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| PromotionSubjectExportRow {
                    entry_id: row.entry_id.clone(),
                    subject_kind: row.subject_kind,
                    claim_label: row.claim_label,
                    effective_label: row.effective_label,
                    holds_claim: row.holds_claim(),
                    current_ring: row.current_ring,
                    target_ring: row.target_ring,
                    promotion_state: row.promotion_state,
                    kill_switch_posture: row.kill_switch_posture,
                    rollback_target_ring: row.rollback_target_ring,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                    soak_complete_for_target: row.soak_complete_for_target(),
                })
                .collect(),
        }
    }

    /// Validates the artifact, returning every violation found.
    pub fn validate(&self) -> Vec<RingPromotionControlViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(RingPromotionControlViolation::DuplicateRowId {
                    row_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(RingPromotionControlViolation::EmptyRows);
        }

        // Every subject kind must appear at least once.
        let present_kinds: BTreeSet<PromotionSubjectKind> =
            self.rows.iter().map(|r| r.subject_kind).collect();
        for kind in PromotionSubjectKind::ALL {
            if !present_kinds.contains(&kind) {
                violations.push(RingPromotionControlViolation::SubjectKindMissing { kind });
            }
        }

        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(RingPromotionControlViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<RingPromotionControlViolation>) {
        if self.schema_version != RING_PROMOTION_CONTROL_SCHEMA_VERSION {
            violations.push(RingPromotionControlViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != RING_PROMOTION_CONTROL_RECORD_KIND {
            violations.push(RingPromotionControlViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("artifact_id", &self.artifact_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("stable_proof_index_ref", &self.stable_proof_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(RingPromotionControlViolation::EmptyField {
                    id: "<artifact>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.rings != Ring::ALL.to_vec() {
            violations.push(RingPromotionControlViolation::ClosedVocabularyMismatch {
                field: "rings",
            });
        }
        if self.subject_kinds != PromotionSubjectKind::ALL.to_vec() {
            violations.push(RingPromotionControlViolation::ClosedVocabularyMismatch {
                field: "subject_kinds",
            });
        }
        if self.promotion_states != PromotionState::ALL.to_vec() {
            violations.push(RingPromotionControlViolation::ClosedVocabularyMismatch {
                field: "promotion_states",
            });
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(RingPromotionControlViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.actions != Action::ALL.to_vec() {
            violations.push(RingPromotionControlViolation::ClosedVocabularyMismatch {
                field: "actions",
            });
        }
        if self.kill_switch_postures != KillSwitchPosture::ALL.to_vec() {
            violations.push(RingPromotionControlViolation::ClosedVocabularyMismatch {
                field: "kill_switch_postures",
            });
        }
        if self.rollback_trigger_kinds != RollbackTriggerKind::ALL.to_vec() {
            violations.push(RingPromotionControlViolation::ClosedVocabularyMismatch {
                field: "rollback_trigger_kinds",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<RingPromotionControlViolation>) {
        if self.rules.is_empty() {
            violations.push(RingPromotionControlViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(RingPromotionControlViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(RingPromotionControlViolation::EmptyField {
                        id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_states.is_empty() {
                violations.push(RingPromotionControlViolation::RuleWithoutStates {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(RingPromotionControlViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &PromotionSubjectRow,
        violations: &mut Vec<RingPromotionControlViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("subject_ref", &row.subject_ref),
            ("subject_summary", &row.subject_summary),
            ("rationale", &row.rationale),
            ("claim_ref", &row.claim_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(RingPromotionControlViolation::EmptyField {
                    id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // Target ring must be at or wider than current ring.
        if row.target_ring.rank() < row.current_ring.rank() {
            violations.push(RingPromotionControlViolation::TargetRingNarrowerThanCurrent {
                row_id: row.entry_id.clone(),
                current: row.current_ring,
                target: row.target_ring,
            });
        }

        // No widening: a narrowing state must drop the effective label below the
        // claim label.
        if row.promotion_state.forces_narrowing() {
            if row.effective_label.rank() >= row.claim_label.rank() {
                violations.push(
                    RingPromotionControlViolation::EffectiveLabelNotNarrowed {
                        row_id: row.entry_id.clone(),
                        state: row.promotion_state,
                        claim_label: row.claim_label,
                        effective_label: row.effective_label,
                    },
                );
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(RingPromotionControlViolation::NarrowingWithoutReason {
                    row_id: row.entry_id.clone(),
                    state: row.promotion_state,
                });
            }
        }

        // A held row must have owner sign-off. Soaking rows may carry
        // SoakIncomplete as their only active gap reason; all other held states
        // must carry no active gap reasons.
        if row.promotion_state.holds_ring() {
            let allowed_gaps: &[GapReason] = match row.promotion_state {
                PromotionState::Soaking => &[GapReason::SoakIncomplete],
                _ => &[],
            };
            let disallowed: Vec<&GapReason> = row
                .active_gap_reasons
                .iter()
                .filter(|g| !allowed_gaps.contains(g))
                .collect();
            if !disallowed.is_empty() {
                violations.push(RingPromotionControlViolation::HeldRowWithActiveGap {
                    row_id: row.entry_id.clone(),
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(RingPromotionControlViolation::HeldRowWithoutSignoff {
                    row_id: row.entry_id.clone(),
                });
            }
        }

        // Rollback target must be at or narrower than current ring.
        if row.rollback_target_ring.rank() > row.current_ring.rank() {
            violations.push(RingPromotionControlViolation::RollbackTargetWiderThanCurrent {
                row_id: row.entry_id.clone(),
                current: row.current_ring,
                rollback_target: row.rollback_target_ring,
            });
        }

        self.validate_row_state_reason_coherence(row, violations);
    }

    fn validate_row_state_reason_coherence(
        &self,
        row: &PromotionSubjectRow,
        violations: &mut Vec<RingPromotionControlViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<RingPromotionControlViolation>,
                               expected: GapReason| {
            violations.push(RingPromotionControlViolation::StateReasonIncoherent {
                row_id: row.entry_id.clone(),
                state: row.promotion_state,
                expected_reason: expected,
            });
        };

        match row.promotion_state {
            PromotionState::Soaking => {
                if !row.has_active_reason(GapReason::SoakIncomplete) {
                    push_incoherent(violations, GapReason::SoakIncomplete);
                }
            }
            PromotionState::Blocked => {
                const ALLOWED: [GapReason; 4] = [
                    GapReason::EvidenceMissing,
                    GapReason::EvidenceStale,
                    GapReason::OwnerSignoffMissing,
                    GapReason::KillSwitchArmed,
                ];
                if !ALLOWED.iter().any(|r| row.has_active_reason(*r)) {
                    push_incoherent(violations, GapReason::EvidenceMissing);
                }
            }
            PromotionState::NarrowedStale => {
                if !row.has_active_reason(GapReason::EvidenceStale) {
                    push_incoherent(violations, GapReason::EvidenceStale);
                }
            }
            PromotionState::NarrowedMissing => {
                if !row.has_active_reason(GapReason::EvidenceMissing) {
                    push_incoherent(violations, GapReason::EvidenceMissing);
                }
            }
            PromotionState::NarrowedRegressed => {
                if !(row.has_active_reason(GapReason::RegressionDetected)
                    || row.has_active_reason(GapReason::RollbackStopTriggered))
                {
                    push_incoherent(violations, GapReason::RegressionDetected);
                }
            }
            PromotionState::NarrowedWaiverExpired => {
                if !row.has_active_reason(GapReason::WaiverExpired) {
                    push_incoherent(violations, GapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(
                        RingPromotionControlViolation::WaiverStateWithoutWaiver {
                            row_id: row.entry_id.clone(),
                            state: row.promotion_state,
                        },
                    );
                }
            }
            PromotionState::RolledBack => {
                if !row.has_active_reason(GapReason::RollbackStopTriggered) {
                    push_incoherent(violations, GapReason::RollbackStopTriggered);
                }
            }
            PromotionState::ProvisionalOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(
                        RingPromotionControlViolation::WaiverStateWithoutWaiver {
                            row_id: row.entry_id.clone(),
                            state: row.promotion_state,
                        },
                    );
                }
            }
            PromotionState::Qualified => {}
        }
    }

    fn validate_publication(&self, violations: &mut Vec<RingPromotionControlViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(RingPromotionControlViolation::EmptyField {
                id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(RingPromotionControlViolation::EmptyField {
                id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                RingPromotionControlViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                RingPromotionControlViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_row_ids != self.computed_blocking_row_ids() {
            violations.push(
                RingPromotionControlViolation::PublicationBlockingSetMismatch {
                    field: "blocking_row_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from a promotion subject row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromotionSubjectExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The promotion subject kind.
    pub subject_kind: PromotionSubjectKind,
    /// Canonical lifecycle label the claim entry publishes.
    pub claim_label: StableClaimLevel,
    /// Effective label after narrowing.
    pub effective_label: StableClaimLevel,
    /// Whether the row holds its claim.
    pub holds_claim: bool,
    /// Current ring.
    pub current_ring: Ring,
    /// Target ring.
    pub target_ring: Ring,
    /// Promotion state.
    pub promotion_state: PromotionState,
    /// Kill-switch posture.
    pub kill_switch_posture: KillSwitchPosture,
    /// Rollback target ring.
    pub rollback_target_ring: Ring,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<GapReason>,
    /// Whether soak is complete for the target ring.
    pub soak_complete_for_target: bool,
}

/// A redaction-safe export projection of the artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RingPromotionControlExportProjection {
    /// Artifact id this projection was produced from.
    pub artifact_id: String,
    /// Artifact as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<PromotionSubjectExportRow>,
}

/// A validation violation for the ring promotion control artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RingPromotionControlViolation {
    /// The artifact carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the artifact.
        actual: u32,
    },
    /// The artifact carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the artifact.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The artifact has no rows.
    EmptyRows,
    /// The artifact has no rules.
    NoRules,
    /// A required subject kind is missing from the rows.
    SubjectKindMissing {
        /// Missing kind.
        kind: PromotionSubjectKind,
    },
    /// A required field is empty.
    EmptyField {
        /// Row, rule, or section id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A rule names no states to watch.
    RuleWithoutStates {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: GapReason,
    },
    /// The target ring is narrower than the current ring.
    TargetRingNarrowerThanCurrent {
        /// Row id.
        row_id: String,
        /// Current ring.
        current: Ring,
        /// Target ring.
        target: Ring,
    },
    /// A narrowing state did not drop the effective label below the claim label.
    EffectiveLabelNotNarrowed {
        /// Row id.
        row_id: String,
        /// Promotion state.
        state: PromotionState,
        /// Claim label.
        claim_label: StableClaimLevel,
        /// Effective label.
        effective_label: StableClaimLevel,
    },
    /// A narrowing row state carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        row_id: String,
        /// State.
        state: PromotionState,
    },
    /// A held row carries an active gap reason.
    HeldRowWithActiveGap {
        /// Row id.
        row_id: String,
    },
    /// A held row has no owner sign-off.
    HeldRowWithoutSignoff {
        /// Row id.
        row_id: String,
    },
    /// A matrix row state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        row_id: String,
        /// State.
        state: PromotionState,
        /// Reason the state requires.
        expected_reason: GapReason,
    },
    /// A waiver-bearing row state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        row_id: String,
        /// State.
        state: PromotionState,
    },
    /// The rollback target ring is wider than the current ring.
    RollbackTargetWiderThanCurrent {
        /// Row id.
        row_id: String,
        /// Current ring.
        current: Ring,
        /// Rollback target ring.
        rollback_target: Ring,
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

impl fmt::Display for RingPromotionControlViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported artifact schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported artifact record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "artifact {field} is not the canonical value")
            }
            Self::EmptyRows => write!(f, "artifact has no promotion subject rows"),
            Self::NoRules => write!(f, "artifact has no rules"),
            Self::SubjectKindMissing { kind } => {
                write!(f, "missing row for subject kind {}", kind.as_str())
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => {
                write!(f, "duplicate row id {row_id}")
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate rule id {rule_id}")
            }
            Self::RuleWithoutStates { rule_id } => {
                write!(f, "rule {rule_id} watches no states")
            }
            Self::GapReasonWithoutRule { reason } => write!(
                f,
                "gap reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::TargetRingNarrowerThanCurrent { row_id, current, target } => write!(
                f,
                "row {row_id} target ring {} is narrower than current ring {}",
                target.as_str(),
                current.as_str()
            ),
            Self::EffectiveLabelNotNarrowed {
                row_id,
                state,
                claim_label,
                effective_label,
            } => write!(
                f,
                "row {row_id} state {} must narrow below claim label {} but effective is {}",
                state.as_str(),
                claim_label.as_str(),
                effective_label.as_str()
            ),
            Self::NarrowingWithoutReason { row_id, state } => write!(
                f,
                "row {row_id} state {} narrows without naming an active gap reason",
                state.as_str()
            ),
            Self::HeldRowWithActiveGap { row_id } => write!(
                f,
                "row {row_id} holds claim while a gap reason is active"
            ),
            Self::HeldRowWithoutSignoff { row_id } => {
                write!(f, "row {row_id} holds claim without owner sign-off")
            }
            Self::StateReasonIncoherent {
                row_id,
                state,
                expected_reason,
            } => write!(
                f,
                "row {row_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { row_id, state } => write!(
                f,
                "row {row_id} state {} names no waiver",
                state.as_str()
            ),
            Self::RollbackTargetWiderThanCurrent {
                row_id,
                current,
                rollback_target,
            } => write!(
                f,
                "row {row_id} rollback target {} is wider than current ring {}",
                rollback_target.as_str(),
                current.as_str()
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "publication decision {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PublicationBlockingSetMismatch { field } => write!(
                f,
                "publication {field} disagrees with the firing rules"
            ),
            Self::SummaryMismatch => write!(
                f,
                "artifact summary counts disagree with the rows"
            ),
        }
    }
}

impl Error for RingPromotionControlViolation {}

/// Loads the embedded ring promotion control artifact.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in artifact no longer matches
/// [`RingPromotionControl`].
pub fn current_ring_promotion_control() -> Result<RingPromotionControl, serde_json::Error> {
    serde_json::from_str(RING_PROMOTION_CONTROL_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn artifact() -> RingPromotionControl {
        current_ring_promotion_control().expect("artifact parses")
    }

    #[test]
    fn embedded_artifact_parses_and_validates() {
        let artifact = artifact();
        assert_eq!(
            artifact.schema_version,
            RING_PROMOTION_CONTROL_SCHEMA_VERSION
        );
        assert_eq!(
            artifact.record_kind,
            RING_PROMOTION_CONTROL_RECORD_KIND
        );
        assert_eq!(artifact.validate(), Vec::new());
        assert!(!artifact.rows.is_empty());
    }

    #[test]
    fn artifact_exercises_holding_and_narrowed_rows() {
        let artifact = artifact();
        assert!(
            !artifact.rows_holding_claim().is_empty(),
            "artifact must hold at least one claim"
        );
        assert!(
            !artifact.rows_narrowed().is_empty(),
            "artifact must narrow at least one claim"
        );
    }

    #[test]
    fn every_subject_kind_is_present() {
        let artifact = artifact();
        let present: BTreeSet<PromotionSubjectKind> =
            artifact.rows.iter().map(|r| r.subject_kind).collect();
        for kind in PromotionSubjectKind::ALL {
            assert!(
                present.contains(&kind),
                "missing subject kind {}",
                kind.as_str()
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let artifact = artifact();
        assert_eq!(artifact.summary, artifact.computed_summary());
        assert_eq!(
            artifact.summary.rows_holding_claim + artifact.summary.rows_narrowed,
            artifact.rows.len()
        );
    }

    #[test]
    fn publication_matches_computed() {
        let artifact = artifact();
        assert_eq!(
            artifact.publication.decision,
            artifact.computed_publication_decision()
        );
        assert_eq!(
            artifact.publication.blocking_rule_ids,
            artifact.computed_blocking_rule_ids()
        );
        assert_eq!(
            artifact.publication.blocking_row_ids,
            artifact.computed_blocking_row_ids()
        );
    }

    #[test]
    fn every_gap_reason_has_a_rule() {
        let artifact = artifact();
        let covered: BTreeSet<GapReason> = artifact
            .rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in GapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn no_row_renders_wider_than_its_claim_ceiling() {
        let artifact = artifact();
        for row in &artifact.rows {
            assert!(
                row.effective_label.rank() <= row.claim_label.rank(),
                "{} renders wider than its ceiling",
                row.entry_id
            );
        }
    }

    #[test]
    fn validate_flags_a_target_narrower_than_current() {
        let mut artifact = artifact();
        let row = artifact
            .rows
            .iter_mut()
            .find(|r| r.current_ring == Ring::Canary)
            .expect("a canary row exists");
        row.target_ring = Ring::Internal;
        let entry_id = row.entry_id.clone();
        assert!(artifact.validate().iter().any(|v| matches!(
            v,
            RingPromotionControlViolation::TargetRingNarrowerThanCurrent { row_id, .. } if *row_id == entry_id
        )));
    }

    #[test]
    fn validate_flags_a_rollback_target_wider_than_current() {
        let mut artifact = artifact();
        let row = artifact
            .rows
            .iter_mut()
            .find(|r| r.current_ring == Ring::Canary)
            .expect("a canary row exists");
        row.rollback_target_ring = Ring::Pilot;
        let entry_id = row.entry_id.clone();
        assert!(artifact.validate().iter().any(|v| matches!(
            v,
            RingPromotionControlViolation::RollbackTargetWiderThanCurrent { row_id, .. } if *row_id == entry_id
        )));
    }

    #[test]
    fn validate_flags_a_narrowing_state_that_does_not_narrow() {
        let mut artifact = artifact();
        let row = artifact
            .rows
            .iter_mut()
            .find(|r| r.promotion_state == PromotionState::NarrowedStale)
            .expect("a narrowed-stale row exists");
        row.effective_label = row.claim_label;
        let entry_id = row.entry_id.clone();
        assert!(artifact.validate().iter().any(|v| matches!(
            v,
            RingPromotionControlViolation::EffectiveLabelNotNarrowed { row_id, .. } if *row_id == entry_id
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut artifact = artifact();
        artifact.publication.decision = PromotionDecision::Proceed;
        assert!(artifact.validate().iter().any(|v| matches!(
            v,
            RingPromotionControlViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_green_row_without_signoff() {
        let mut artifact = artifact();
        let row = artifact
            .rows
            .iter_mut()
            .find(|r| r.promotion_state.holds_ring())
            .expect("a held row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        let entry_id = row.entry_id.clone();
        assert!(artifact
            .validate()
            .contains(&RingPromotionControlViolation::HeldRowWithoutSignoff {
                row_id: entry_id,
            }));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let artifact = artifact();
        let projection = artifact.support_export_projection();
        assert_eq!(projection.rows.len(), artifact.rows.len());
        assert_eq!(
            projection.publication_decision,
            artifact.publication.decision
        );
        for (row, projected) in artifact.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, projected.entry_id);
            assert_eq!(row.holds_claim(), projected.holds_claim);
            assert_eq!(row.current_ring, projected.current_ring);
            assert_eq!(row.effective_label, projected.effective_label);
        }
    }
}
