//! Typed promotion-ledger register binding each M5 artifact family to an
//! immutable promotion timeline, the affected artifact-graph node set those
//! promotions touch, the release-center/headless reconstruction parity that
//! proves both flows replay the same history, and the break-glass events that
//! ride the same timeline object model as ordinary promotions.
//!
//! Where the per-family release graph
//! (`implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family`)
//! speaks for the *release candidate* every M5 artifact family ships and the
//! publication review-sheet register
//! (`ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes`)
//! speaks for the *review sheet* every publication lane exposes, this register
//! speaks for the *promotion history* every M5 artifact graph accumulates — the
//! single inspectable ledger a release-center operator scrolls, a headless
//! automation flow reconstructs, and an audit or postmortem export replays. Each
//! [`FamilyPromotionLedger`] binds one artifact family to:
//!
//! - the [`ArtifactGraphNode`] set the family's promotions touch, every node
//!   carrying a canonical [`ImmutableDigest`](crate::release_center_model::ImmutableDigest)
//!   so a promotion step joins to immutable graph material rather than a mutable
//!   "latest" pointer,
//! - an ordered [`PromotionTimelineStep`](crate::release_center_model::PromotionTimelineStep)
//!   timeline carrying, for every promotion, the source stage, destination stage,
//!   approving actors, evidence bundle refs, immutable digest refs, reversible
//!   window, and the affected node set — and capturing break-glass freezes,
//!   emergency publications, and out-of-band corrections in the *same* step model
//!   via each step's [`BreakGlassDisclosure`](crate::release_center_model::BreakGlassDisclosure),
//! - a [`HistoryReconstructionParity`] record proving the release-center UI and
//!   the headless plan reconstruct the same ordered step set under the same
//!   history digest, and that an audit/postmortem export can replay the same
//!   history,
//! - an owner manifest ([`FamilyPromotionLedger::owner_signoff`]), a
//!   [`ProofPacket`] and its freshness SLO, and an optional waiver,
//! - the overall ledger state earned ([`LedgerState`]), the active narrowing
//!   reasons ([`NarrowingReason`]), and the effective label after narrowing
//!   ([`FamilyPromotionLedger::published_label`]).
//!
//! The [`LaunchCutline`] fixes the boundary between a ledger that may publish a
//! Stable claim and one that must narrow below it. The
//! [`M5ArtifactGraphPromotionStopRule`] set names the closed conditions that gate
//! publication — one per [`NarrowingReason`] — and
//! [`M5ArtifactGraphPromotionRegister::publication`] records the proceed/hold
//! verdict.
//!
//! Two guardrails are encoded directly in [`validate`](M5ArtifactGraphPromotionRegister::validate):
//! an emergency (break-glass) step may not bypass timeline capture or digest
//! binding — it must still record its stages, approving actors, evidence, and
//! immutable digests like an ordinary promotion — and a ledger may not let a
//! mutable "latest" pointer stand in for immutable graph history.
//!
//! The register is checked in at
//! `artifacts/release/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs.json`
//! and embedded here, so this typed consumer and the CI gate agree on every
//! artifact graph without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no credential bodies, raw artifact payloads, signatures, or provider
//! material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix::M5ArtifactFamilyKind;
use crate::release_center_model::{
    AuthSourceClass, BreakGlassDisclosure, BreakGlassStateClass, ImmutableDigest,
    PromotionEventClass, PromotionStage, PromotionTimelineStep, SemanticChangeClass,
};
use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

mod builder;
pub use builder::build_m5_artifact_graph_promotion_ledger;

/// Supported register schema version.
pub const M5_ARTIFACT_GRAPH_PROMOTION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_ARTIFACT_GRAPH_PROMOTION_RECORD_KIND: &str =
    "implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs";

/// Repo-relative path to the checked-in register.
pub const M5_ARTIFACT_GRAPH_PROMOTION_PATH: &str =
    "artifacts/release/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs.json";

/// Embedded checked-in register JSON.
pub const M5_ARTIFACT_GRAPH_PROMOTION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs.json"
));

/// Whether a ledger drives publication from immutable graph history or a mutable
/// "latest" pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryPointerClass {
    /// Promotion history is reconstructed from immutable graph nodes and digests.
    ImmutableGraphHistory,
    /// A mutable "latest" pointer stands in for immutable graph history.
    MutableLatestPointer,
}

impl HistoryPointerClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 2] = [Self::ImmutableGraphHistory, Self::MutableLatestPointer];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImmutableGraphHistory => "immutable_graph_history",
            Self::MutableLatestPointer => "mutable_latest_pointer",
        }
    }

    /// Whether the class lets a ledger hold its label: only immutable history
    /// clears the gate.
    pub const fn holds(self) -> bool {
        matches!(self, Self::ImmutableGraphHistory)
    }
}

/// Parity of the reconstructed promotion history across release-center, headless,
/// and audit/postmortem flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityState {
    /// Release-center and headless flows reconstruct the same history digest.
    Matched,
    /// Release-center and headless flows reconstruct diverging histories.
    Divergent,
    /// One flow has no reconstructable history to compare.
    Missing,
}

impl ParityState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Matched, Self::Divergent, Self::Missing];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Matched => "matched",
            Self::Divergent => "divergent",
            Self::Missing => "missing",
        }
    }

    /// Whether the state lets a ledger publish: only matched parity clears it.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Matched)
    }
}

/// Overall state a promotion ledger earned for its claimed label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerState {
    /// Every promotion is timeline-captured and digest-bound, release-center and
    /// headless reconstruct the same history, audit replay is available,
    /// break-glass events are reconciled, the proof packet is within SLO, and the
    /// owner has signed; the family publishes its claim.
    Reconstructable,
    /// A history gap (timeline capture, digest binding, node-set, reconstruction,
    /// audit-replay, break-glass, reversible-window, mutable-pointer, or stale
    /// evidence gap) narrows the family below the cutline.
    HistoryGap,
    /// The proof packet has gone stale or is missing.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers a
    /// recorded gap.
    OnWaiver,
    /// The owner manifest is unsigned.
    OwnerUnsigned,
}

impl LedgerState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Reconstructable,
        Self::HistoryGap,
        Self::Stale,
        Self::OnWaiver,
        Self::OwnerUnsigned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reconstructable => "reconstructable",
            Self::HistoryGap => "history_gap",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::OwnerUnsigned => "owner_unsigned",
        }
    }

    /// Whether the state lets a ledger carry its claimed label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Reconstructable | Self::OnWaiver)
    }
}

/// Closed reason a promotion ledger narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// A promotion or emergency action did not record a complete timeline step:
    /// it omits its stages, approving actors, or evidence (capture was bypassed).
    TimelineCaptureBypassed,
    /// A timeline step records no immutable digest refs (its material is not
    /// digest-bound).
    DigestBindingMissing,
    /// The affected node set is empty, or a step cites a digest that resolves to
    /// no node in the set (the immutable-digest join is incomplete).
    AffectedNodeSetIncomplete,
    /// A mutable "latest" pointer stands in for immutable graph history.
    MutableLatestPointer,
    /// Release-center and headless flows reconstruct diverging histories.
    ReconstructionDivergent,
    /// No audit/postmortem export can replay who promoted what, when, on which
    /// evidence, and with which reversible window.
    AuditReplayUnavailable,
    /// A break-glass step is active past its reconciliation window, or names no
    /// reconciliation follow-up.
    BreakGlassUnreconciled,
    /// A timeline step discloses neither a reversible window nor a rollback
    /// target.
    ReversibleWindowUndisclosed,
    /// A timeline step rides evidence that is stale or missing and blocks
    /// promotion.
    EvidenceStale,
    /// The proof packet is stale.
    ProofPacketStale,
    /// The proof packet is missing.
    ProofPacketMissing,
    /// The owner manifest is unsigned.
    OwnerManifestUnsigned,
    /// A waiver the ledger relied on has expired.
    WaiverExpired,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::TimelineCaptureBypassed,
        Self::DigestBindingMissing,
        Self::AffectedNodeSetIncomplete,
        Self::MutableLatestPointer,
        Self::ReconstructionDivergent,
        Self::AuditReplayUnavailable,
        Self::BreakGlassUnreconciled,
        Self::ReversibleWindowUndisclosed,
        Self::EvidenceStale,
        Self::ProofPacketStale,
        Self::ProofPacketMissing,
        Self::OwnerManifestUnsigned,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TimelineCaptureBypassed => "timeline_capture_bypassed",
            Self::DigestBindingMissing => "digest_binding_missing",
            Self::AffectedNodeSetIncomplete => "affected_node_set_incomplete",
            Self::MutableLatestPointer => "mutable_latest_pointer",
            Self::ReconstructionDivergent => "reconstruction_divergent",
            Self::AuditReplayUnavailable => "audit_replay_unavailable",
            Self::BreakGlassUnreconciled => "break_glass_unreconciled",
            Self::ReversibleWindowUndisclosed => "reversible_window_undisclosed",
            Self::EvidenceStale => "evidence_stale",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::OwnerManifestUnsigned => "owner_manifest_unsigned",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether this reason is a history gap (rather than a proof-packet or
    /// owner-manifest gap). The [`LedgerState::HistoryGap`] state must name at
    /// least one of these.
    pub const fn is_history_gap(self) -> bool {
        matches!(
            self,
            Self::TimelineCaptureBypassed
                | Self::DigestBindingMissing
                | Self::AffectedNodeSetIncomplete
                | Self::MutableLatestPointer
                | Self::ReconstructionDivergent
                | Self::AuditReplayUnavailable
                | Self::BreakGlassUnreconciled
                | Self::ReversibleWindowUndisclosed
                | Self::EvidenceStale
                | Self::WaiverExpired
        )
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the claim below the cutline.
    NarrowLabel,
    /// Capture the missing promotion as a complete timeline step.
    CaptureTimelineStep,
    /// Bind the step to its immutable digests.
    BindImmutableDigest,
    /// Complete the affected node set so every cited digest resolves.
    CompleteAffectedNodeSet,
    /// Pin the ledger to immutable graph history instead of a mutable pointer.
    PinImmutableHistory,
    /// Reconcile the release-center and headless reconstruction.
    ReconcileReconstruction,
    /// Restore the audit/postmortem replay export.
    RestoreAuditReplay,
    /// Reconcile the open break-glass action.
    ReconcileBreakGlass,
    /// Disclose the reversible window or rollback target.
    DiscloseReversibleWindow,
    /// Recapture the stale or missing step evidence.
    RecaptureEvidence,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Obtain the required owner-manifest sign-off.
    RequestOwnerSignoff,
    /// Renew the expired waiver.
    RenewWaiver,
}

impl StopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::CaptureTimelineStep,
        Self::BindImmutableDigest,
        Self::CompleteAffectedNodeSet,
        Self::PinImmutableHistory,
        Self::ReconcileReconstruction,
        Self::RestoreAuditReplay,
        Self::ReconcileBreakGlass,
        Self::DiscloseReversibleWindow,
        Self::RecaptureEvidence,
        Self::RefreshProofPacket,
        Self::RequestOwnerSignoff,
        Self::RenewWaiver,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::CaptureTimelineStep => "capture_timeline_step",
            Self::BindImmutableDigest => "bind_immutable_digest",
            Self::CompleteAffectedNodeSet => "complete_affected_node_set",
            Self::PinImmutableHistory => "pin_immutable_history",
            Self::ReconcileReconstruction => "reconcile_reconstruction",
            Self::RestoreAuditReplay => "restore_audit_replay",
            Self::ReconcileBreakGlass => "reconcile_break_glass",
            Self::DiscloseReversibleWindow => "disclose_reversible_window",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::RenewWaiver => "renew_waiver",
        }
    }
}

/// One artifact-graph node in a family's affected node set.
///
/// Each node carries a canonical [`ImmutableDigest`](crate::release_center_model::ImmutableDigest);
/// a promotion timeline step joins to the node set by citing the node's digest
/// id, so promotion history is anchored to immutable graph material rather than a
/// mutable pointer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactGraphNode {
    /// Stable node id, equal to the node's immutable digest id.
    pub node_id: String,
    /// Artifact ref this node represents in the graph.
    pub artifact_ref: String,
    /// The immutable digest of the node's material.
    pub digest: ImmutableDigest,
    /// Exact-build identity ref the node was produced under.
    pub exact_build_identity_ref: String,
    /// Reviewable one-line statement of the node.
    pub summary: String,
}

/// Proof that release-center, headless, and audit/postmortem flows reconstruct
/// the same promotion history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryReconstructionParity {
    /// Ref to the release-center reconstruction of the timeline.
    pub release_center_history_ref: String,
    /// Ref to the headless reconstruction of the timeline.
    pub headless_history_ref: String,
    /// History digest the release-center flow reconstructs.
    pub release_center_history_digest: String,
    /// History digest the headless flow reconstructs.
    pub headless_history_digest: String,
    /// Ref to the audit/postmortem replay export.
    pub audit_export_ref: String,
    /// History digest the audit/postmortem export replays.
    pub audit_export_digest: String,
    /// The ordered timeline step ids both flows reconstruct.
    #[serde(default)]
    pub reconstructed_step_ids: Vec<String>,
    /// The parity state earned.
    pub parity_state: ParityState,
}

impl HistoryReconstructionParity {
    /// Whether release-center and headless reconstruct the same non-empty history
    /// digest.
    pub fn history_digests_match(&self) -> bool {
        !self.release_center_history_digest.trim().is_empty()
            && self.release_center_history_digest == self.headless_history_digest
    }

    /// Whether an audit/postmortem export can replay the same history.
    pub fn audit_replay_available(&self) -> bool {
        !self.audit_export_ref.trim().is_empty()
            && !self.audit_export_digest.trim().is_empty()
            && self.audit_export_digest == self.release_center_history_digest
            && !self.reconstructed_step_ids.is_empty()
    }
}

/// One promotion-ledger stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ArtifactGraphPromotionStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched family fires this rule.
    pub trigger_reason: NarrowingReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: StopAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One promotion ledger for one M5 artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyPromotionLedger {
    /// Stable ledger id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The M5 artifact family this ledger tracks.
    pub family_kind: M5ArtifactFamilyKind,
    /// The artifact graph ref this family's nodes belong to.
    pub artifact_graph_ref: String,
    /// The release-candidate ref whose promotions this ledger records.
    pub candidate_ref: String,
    /// Reviewable one-line statement of the family.
    pub family_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes.
    pub claim_label: StableClaimLevel,
    /// Overall ledger state earned for the family.
    pub ledger_state: LedgerState,
    /// Whether promotion history is immutable or driven by a mutable pointer.
    pub history_pointer_class: HistoryPointerClass,
    /// The affected artifact-graph node set the family's promotions touch.
    pub affected_node_set: Vec<ArtifactGraphNode>,
    /// The ordered promotion timeline, including break-glass steps.
    pub timeline: Vec<PromotionTimelineStep>,
    /// The release-center/headless/audit reconstruction parity.
    pub reconstruction: HistoryReconstructionParity,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner manifest sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the family below its claim label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The lifecycle label the family effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this ledger.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the family carries this posture.
    pub rationale: String,
}

impl FamilyPromotionLedger {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the ledger's state lets it carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.ledger_state.holds_label()
    }

    /// True when a narrowing reason is active on the family.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// The ordered timeline step ids.
    pub fn timeline_step_ids(&self) -> Vec<String> {
        self.timeline
            .iter()
            .map(|step| step.timeline_step_id.clone())
            .collect()
    }

    /// The digest ids in the affected node set.
    pub fn node_digest_ids(&self) -> BTreeSet<String> {
        self.affected_node_set
            .iter()
            .map(|node| node.digest.digest_id.clone())
            .collect()
    }

    /// True when `step` was driven by a break-glass action that the timeline must
    /// reconcile (an emergency publication, freeze, or out-of-band correction).
    pub fn is_break_glass_step(step: &PromotionTimelineStep) -> bool {
        BreakGlassDisclosure::state_requires_reconciliation(step.break_glass.state_class)
    }

    /// The break-glass steps in the timeline.
    pub fn break_glass_steps(&self) -> Vec<&PromotionTimelineStep> {
        self.timeline
            .iter()
            .filter(|step| Self::is_break_glass_step(step))
            .collect()
    }

    /// True when `step` records a complete promotion: approving actors, evidence,
    /// and immutable digests — the capture an emergency flow may not skip.
    pub fn step_capture_complete(step: &PromotionTimelineStep) -> bool {
        !step.approving_actor_refs.is_empty()
            && !step.evidence_refs.is_empty()
            && !step.digest_refs.is_empty()
            && !step.timeline_step_id.trim().is_empty()
    }

    /// True when every digest ref the step cites resolves to a node in the set.
    pub fn step_digests_resolve(&self, step: &PromotionTimelineStep) -> bool {
        let nodes = self.node_digest_ids();
        !step.digest_refs.is_empty() && step.digest_refs.iter().all(|d| nodes.contains(d))
    }

    /// True when the step discloses a reversible window or a rollback target.
    pub fn step_reversible_disclosed(step: &PromotionTimelineStep) -> bool {
        step.reversible_window
            .as_ref()
            .map(|window| !window.trim().is_empty())
            .unwrap_or(false)
            || !step.rollback_target_ref.trim().is_empty()
    }

    /// True when `step` rides evidence that is stale or missing and blocks
    /// promotion.
    pub fn step_evidence_blocks(step: &PromotionTimelineStep) -> bool {
        step.evidence_refs
            .iter()
            .any(|e| e.required_for_promotion && e.freshness_class.blocks_promotion())
    }

    /// True when `step` is a reconciled or properly pending break-glass step.
    pub fn break_glass_step_reconciled(step: &PromotionTimelineStep) -> bool {
        match step.break_glass.state_class {
            BreakGlassStateClass::Reconciled | BreakGlassStateClass::SupersededBySignedAction => {
                true
            }
            BreakGlassStateClass::ActivePendingReconciliation => {
                step.break_glass.reconcile_by.is_some()
                    && !step.break_glass.follow_up_refs.is_empty()
            }
            BreakGlassStateClass::ExpiredWithoutReconciliation
            | BreakGlassStateClass::ForbiddenForAction => false,
            BreakGlassStateClass::NotUsed | BreakGlassStateClass::EligibleButNotUsed => true,
        }
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ArtifactGraphPromotionSummary {
    /// Total number of promotion ledgers.
    pub total_entries: usize,
    /// Distinct artifact graphs covered.
    pub total_artifact_graphs: usize,
    /// Ledgers publishing a label at or above the cutline.
    pub entries_reconstructable: usize,
    /// Ledgers narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Ledgers holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Ledgers carrying a timeline-capture or digest-binding reason.
    pub entries_with_timeline_gap: usize,
    /// Ledgers carrying an affected-node-set reason.
    pub entries_with_node_set_gap: usize,
    /// Ledgers carrying a reconstruction or audit-replay reason.
    pub entries_with_reconstruction_gap: usize,
    /// Ledgers carrying a break-glass reason.
    pub entries_with_break_glass_gap: usize,
    /// Ledgers carrying a mutable-latest-pointer reason.
    pub entries_with_mutable_pointer_gap: usize,
    /// Total release-blocking families.
    pub release_blocking_total: usize,
    /// Release-blocking families publishing a label at or above the cutline.
    pub release_blocking_reconstructable: usize,
    /// Release-blocking families narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook-pack families.
    pub notebook_pack_entries: usize,
    /// Request/data-asset families.
    pub request_data_asset_entries: usize,
    /// Profiler/replay-artifact families.
    pub profiler_replay_entries: usize,
    /// Framework/template-pack families.
    pub framework_template_entries: usize,
    /// Docs-pack families.
    pub docs_pack_entries: usize,
    /// Model-pack families.
    pub model_pack_entries: usize,
    /// Companion/offboarding-packet families.
    pub companion_offboarding_entries: usize,
    /// Managed-output families.
    pub managed_output_entries: usize,
    /// Ledgers whose reconstruction parity is `matched`.
    pub parity_matched: usize,
    /// Ledgers whose reconstruction parity is `divergent`.
    pub parity_divergent: usize,
    /// Ledgers whose reconstruction parity is `missing`.
    pub parity_missing: usize,
    /// Ledgers driven by immutable graph history.
    pub history_immutable: usize,
    /// Ledgers driven by a mutable latest pointer.
    pub history_mutable: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total timeline steps across all ledgers.
    pub total_timeline_steps: usize,
    /// Total break-glass steps across all ledgers.
    pub total_break_glass_steps: usize,
    /// Total active narrowing reasons across all ledgers.
    pub total_active_narrowing_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One replay entry — the audit/postmortem view of a single promotion step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromotionReplayEntry {
    /// Stable timeline step id.
    pub timeline_step_id: String,
    /// Source stage.
    pub source_stage: PromotionStage,
    /// Destination stage.
    pub destination_stage: PromotionStage,
    /// Timeline event class.
    pub event_class: PromotionEventClass,
    /// Semantic change class.
    pub semantic_change_class: SemanticChangeClass,
    /// Approving actor refs (who promoted).
    pub approving_actor_refs: Vec<String>,
    /// Auth source class used by the step.
    pub auth_source_class: AuthSourceClass,
    /// Evidence refs the step rode (on which evidence).
    pub evidence_refs: Vec<String>,
    /// Immutable digest refs bound to the step.
    pub digest_refs: Vec<String>,
    /// Reversible window (with which reversible window).
    pub reversible_window: Option<String>,
    /// Rollback target ref available for the step.
    pub rollback_target_ref: String,
    /// Break-glass state of the step.
    pub break_glass_state: BreakGlassStateClass,
    /// Whether the step was a break-glass action.
    pub is_break_glass: bool,
}

/// One export row for downstream Help/About, support, audit, and diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactGraphPromotionExportRow {
    /// Stable ledger id.
    pub entry_id: String,
    /// The M5 artifact family this ledger tracks.
    pub family_kind: M5ArtifactFamilyKind,
    /// The artifact graph ref.
    pub artifact_graph_ref: String,
    /// The release-candidate ref.
    pub candidate_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the family publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall ledger state earned.
    pub ledger_state: LedgerState,
    /// Whether promotion history is immutable or mutable-pointer-driven.
    pub history_pointer_class: HistoryPointerClass,
    /// The reconstruction parity state.
    pub parity_state: ParityState,
    /// Whether an audit/postmortem export can replay the history.
    pub audit_replay_available: bool,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Number of timeline steps.
    pub timeline_step_count: usize,
    /// Number of break-glass steps.
    pub break_glass_step_count: usize,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The per-step audit/postmortem replay.
    pub replay: Vec<PromotionReplayEntry>,
}

/// Export projection for Help/About, support, audit, and diagnostics surfaces.
///
/// Each row carries a `replay` that reconstructs who promoted what, when, on
/// which evidence, and with which reversible window — the audit and postmortem
/// view of the family's promotion history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactGraphPromotionExportProjection {
    /// Register identifier.
    pub manifest_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5ArtifactGraphPromotionExportRow>,
}

/// The typed M5 artifact-graph promotion-ledger register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ArtifactGraphPromotionRegister {
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
    /// Ref to the M5 publication matrix this register publishes against.
    pub publication_matrix_ref: String,
    /// Ref to the per-family release graph this register joins to.
    pub family_release_graph_ref: String,
    /// Ref to the shared release-center object model.
    pub release_center_model_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed artifact-family-kind vocabulary.
    pub family_kinds: Vec<M5ArtifactFamilyKind>,
    /// Closed ledger-state vocabulary.
    pub ledger_states: Vec<LedgerState>,
    /// Closed history-pointer-class vocabulary.
    pub history_pointer_classes: Vec<HistoryPointerClass>,
    /// Closed parity-state vocabulary.
    pub parity_states: Vec<ParityState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking candidate refs this register covers.
    pub release_blocking_candidate_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<M5ArtifactGraphPromotionStopRule>,
    /// Promotion ledgers.
    pub rows: Vec<FamilyPromotionLedger>,
    /// Recorded publication verdict.
    pub publication: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5ArtifactGraphPromotionSummary,
}

impl M5ArtifactGraphPromotionRegister {
    /// Returns the ledger registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&FamilyPromotionLedger> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the ledgers publishing a label at or above the cutline.
    pub fn rows_reconstructable(&self) -> Vec<&FamilyPromotionLedger> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the ledgers narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&FamilyPromotionLedger> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking ledgers.
    pub fn release_blocking_rows(&self) -> Vec<&FamilyPromotionLedger> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the ledgers for one artifact-family kind.
    pub fn rows_for_kind(&self, kind: M5ArtifactFamilyKind) -> Vec<&FamilyPromotionLedger> {
        self.rows
            .iter()
            .filter(|row| row.family_kind == kind)
            .collect()
    }

    /// Distinct artifact graphs (by graph ref) the register covers.
    pub fn artifact_graphs(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.artifact_graph_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched family carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &M5ArtifactGraphPromotionStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the ledgers and stop rules.
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

    /// Ledger ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only ledgers whose claim is at or above the cutline count: a ledger whose
    /// claim is already canonically narrowed is not a *publication* blocker, it
    /// merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<NarrowingReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
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

    /// Recomputes the summary block from the ledgers and stop rules.
    pub fn computed_summary(&self) -> M5ArtifactGraphPromotionSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: M5ArtifactFamilyKind| self.rows_for_kind(kind).len();
        let parity = |state: ParityState| {
            self.rows
                .iter()
                .filter(|row| row.reconstruction.parity_state == state)
                .count()
        };
        let pointer = |class: HistoryPointerClass| {
            self.rows
                .iter()
                .filter(|row| row.history_pointer_class == class)
                .count()
        };
        let with_any = |reasons: &[NarrowingReason]| {
            self.rows
                .iter()
                .filter(|row| reasons.iter().any(|r| row.has_active_reason(*r)))
                .count()
        };
        let release_blocking: Vec<&FamilyPromotionLedger> = self.release_blocking_rows();
        M5ArtifactGraphPromotionSummary {
            total_entries: self.rows.len(),
            total_artifact_graphs: self.artifact_graphs().len(),
            entries_reconstructable: self
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
                .filter(|row| row.ledger_state == LedgerState::OnWaiver)
                .count(),
            entries_with_timeline_gap: with_any(&[
                NarrowingReason::TimelineCaptureBypassed,
                NarrowingReason::DigestBindingMissing,
            ]),
            entries_with_node_set_gap: with_any(&[NarrowingReason::AffectedNodeSetIncomplete]),
            entries_with_reconstruction_gap: with_any(&[
                NarrowingReason::ReconstructionDivergent,
                NarrowingReason::AuditReplayUnavailable,
            ]),
            entries_with_break_glass_gap: with_any(&[NarrowingReason::BreakGlassUnreconciled]),
            entries_with_mutable_pointer_gap: with_any(&[NarrowingReason::MutableLatestPointer]),
            release_blocking_total: release_blocking.len(),
            release_blocking_reconstructable: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            notebook_pack_entries: kind(M5ArtifactFamilyKind::NotebookPack),
            request_data_asset_entries: kind(M5ArtifactFamilyKind::RequestDataAsset),
            profiler_replay_entries: kind(M5ArtifactFamilyKind::ProfilerReplayArtifact),
            framework_template_entries: kind(M5ArtifactFamilyKind::FrameworkTemplatePack),
            docs_pack_entries: kind(M5ArtifactFamilyKind::DocsPack),
            model_pack_entries: kind(M5ArtifactFamilyKind::ModelPack),
            companion_offboarding_entries: kind(M5ArtifactFamilyKind::CompanionOffboardingPacket),
            managed_output_entries: kind(M5ArtifactFamilyKind::ManagedOutput),
            parity_matched: parity(ParityState::Matched),
            parity_divergent: parity(ParityState::Divergent),
            parity_missing: parity(ParityState::Missing),
            history_immutable: pointer(HistoryPointerClass::ImmutableGraphHistory),
            history_mutable: pointer(HistoryPointerClass::MutableLatestPointer),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_timeline_steps: self.rows.iter().map(|row| row.timeline.len()).sum(),
            total_break_glass_steps: self
                .rows
                .iter()
                .map(|row| row.break_glass_steps().len())
                .sum(),
            total_active_narrowing_reasons: self
                .rows
                .iter()
                .map(|row| row.active_narrowing_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/audit-safe projection that downstream surfaces render
    /// instead of cloning status text. Each row carries a per-step replay.
    pub fn support_export_projection(&self) -> M5ArtifactGraphPromotionExportProjection {
        M5ArtifactGraphPromotionExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5ArtifactGraphPromotionExportRow {
                    entry_id: row.entry_id.clone(),
                    family_kind: row.family_kind,
                    artifact_graph_ref: row.artifact_graph_ref.clone(),
                    candidate_ref: row.candidate_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    ledger_state: row.ledger_state,
                    history_pointer_class: row.history_pointer_class,
                    parity_state: row.reconstruction.parity_state,
                    audit_replay_available: row.reconstruction.audit_replay_available(),
                    slo_state: row.proof_packet.slo_state,
                    timeline_step_count: row.timeline.len(),
                    break_glass_step_count: row.break_glass_steps().len(),
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                    replay: row
                        .timeline
                        .iter()
                        .map(|step| PromotionReplayEntry {
                            timeline_step_id: step.timeline_step_id.clone(),
                            source_stage: step.source_stage,
                            destination_stage: step.destination_stage,
                            event_class: step.event_class,
                            semantic_change_class: step.semantic_change_class,
                            approving_actor_refs: step.approving_actor_refs.clone(),
                            auth_source_class: step.auth_source_class,
                            evidence_refs: step
                                .evidence_refs
                                .iter()
                                .map(|e| e.evidence_ref.clone())
                                .collect(),
                            digest_refs: step.digest_refs.clone(),
                            reversible_window: step.reversible_window.clone(),
                            rollback_target_ref: step.rollback_target_ref.clone(),
                            break_glass_state: step.break_glass.state_class,
                            is_break_glass: FamilyPromotionLedger::is_break_glass_step(step),
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<M5ArtifactGraphPromotionViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5ArtifactGraphPromotionViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5ArtifactGraphPromotionViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5ArtifactGraphPromotionViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ArtifactGraphPromotionViolation>) {
        if self.schema_version != M5_ARTIFACT_GRAPH_PROMOTION_SCHEMA_VERSION {
            violations.push(
                M5ArtifactGraphPromotionViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_ARTIFACT_GRAPH_PROMOTION_RECORD_KIND {
            violations.push(M5ArtifactGraphPromotionViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("manifest_id", &self.manifest_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("publication_matrix_ref", &self.publication_matrix_ref),
            ("family_release_graph_ref", &self.family_release_graph_ref),
            ("release_center_model_ref", &self.release_center_model_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ArtifactGraphPromotionViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        let closed = |violations: &mut Vec<M5ArtifactGraphPromotionViolation>,
                      ok: bool,
                      field: &'static str| {
            if !ok {
                violations
                    .push(M5ArtifactGraphPromotionViolation::ClosedVocabularyMismatch { field });
            }
        };
        closed(
            violations,
            self.lifecycle_labels == StableClaimLevel::ALL.to_vec(),
            "lifecycle_labels",
        );
        closed(
            violations,
            self.family_kinds == M5ArtifactFamilyKind::ALL.to_vec(),
            "family_kinds",
        );
        closed(
            violations,
            self.ledger_states == LedgerState::ALL.to_vec(),
            "ledger_states",
        );
        closed(
            violations,
            self.history_pointer_classes == HistoryPointerClass::ALL.to_vec(),
            "history_pointer_classes",
        );
        closed(
            violations,
            self.parity_states == ParityState::ALL.to_vec(),
            "parity_states",
        );
        closed(
            violations,
            self.narrowing_reasons == NarrowingReason::ALL.to_vec(),
            "narrowing_reasons",
        );
        closed(
            violations,
            self.stop_rule_actions == StopAction::ALL.to_vec(),
            "stop_rule_actions",
        );

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(
                M5ArtifactGraphPromotionViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.cutline_level",
                },
            );
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(
                M5ArtifactGraphPromotionViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.above_cutline_levels",
                },
            );
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(
                M5ArtifactGraphPromotionViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.below_cutline_levels",
                },
            );
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5ArtifactGraphPromotionViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5ArtifactGraphPromotionViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5ArtifactGraphPromotionViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5ArtifactGraphPromotionViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ArtifactGraphPromotionViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5ArtifactGraphPromotionViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations.push(
                    M5ArtifactGraphPromotionViolation::NarrowingReasonWithoutStopRule { reason },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &FamilyPromotionLedger,
        violations: &mut Vec<M5ArtifactGraphPromotionViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("artifact_graph_ref", &row.artifact_graph_ref),
            ("candidate_ref", &row.candidate_ref),
            ("family_summary", &row.family_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            (
                "reconstruction.release_center_history_ref",
                &row.reconstruction.release_center_history_ref,
            ),
            (
                "reconstruction.headless_history_ref",
                &row.reconstruction.headless_history_ref,
            ),
            (
                "reconstruction.audit_export_ref",
                &row.reconstruction.audit_export_ref,
            ),
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
                violations.push(M5ArtifactGraphPromotionViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_history(row, violations);

        // The ceiling: no family may publish a label wider than the claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5ArtifactGraphPromotionViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window consistent.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5ArtifactGraphPromotionViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(
                M5ArtifactGraphPromotionViolation::FreshnessSloInconsistent {
                    entry_id: row.entry_id.clone(),
                },
            );
        }

        // A claim whose canonical label is below the cutline forces the family to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(M5ArtifactGraphPromotionViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(M5ArtifactGraphPromotionViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.ledger_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A reconstructable/on-waiver family publishes exactly the claim's
            // canonical label, carries no active reason, rides a captured
            // within-SLO packet, and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(M5ArtifactGraphPromotionViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(M5ArtifactGraphPromotionViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(M5ArtifactGraphPromotionViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(M5ArtifactGraphPromotionViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(M5ArtifactGraphPromotionViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            // The held invariants on the ledger itself.
            if !row.history_pointer_class.holds() {
                violations.push(M5ArtifactGraphPromotionViolation::HeldOnMutablePointer {
                    entry_id: row.entry_id.clone(),
                });
            }
            if row.timeline.is_empty() {
                violations.push(M5ArtifactGraphPromotionViolation::HeldWithoutTimeline {
                    entry_id: row.entry_id.clone(),
                });
            }
            if row.affected_node_set.is_empty() {
                violations.push(M5ArtifactGraphPromotionViolation::HeldWithoutNodeSet {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.reconstruction.parity_state.holds()
                || !row.reconstruction.history_digests_match()
            {
                violations.push(
                    M5ArtifactGraphPromotionViolation::HeldWithoutReconstruction {
                        entry_id: row.entry_id.clone(),
                        state: row.reconstruction.parity_state,
                    },
                );
            }
            if !row.reconstruction.audit_replay_available() {
                violations.push(M5ArtifactGraphPromotionViolation::HeldWithoutAuditReplay {
                    entry_id: row.entry_id.clone(),
                });
            }
            if row.reconstruction.reconstructed_step_ids != row.timeline_step_ids() {
                violations.push(
                    M5ArtifactGraphPromotionViolation::ReconstructionStepMismatch {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            // A reconstructable family carries no waiver; an on-waiver family
            // carries a valid one.
            match row.ledger_state {
                LedgerState::Reconstructable => {
                    if row.waiver.is_some() {
                        violations.push(M5ArtifactGraphPromotionViolation::ClearedWithWaiver {
                            entry_id: row.entry_id.clone(),
                        });
                    }
                }
                LedgerState::OnWaiver => {
                    if row
                        .waiver
                        .as_ref()
                        .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                        .unwrap_or(true)
                    {
                        violations.push(
                            M5ArtifactGraphPromotionViolation::WaiverStateWithoutWaiver {
                                entry_id: row.entry_id.clone(),
                                state: row.ledger_state,
                            },
                        );
                    }
                }
                _ => {}
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(
                    M5ArtifactGraphPromotionViolation::PublishedLabelNotNarrowed {
                        entry_id: row.entry_id.clone(),
                        state: row.ledger_state,
                        published: row.published_label,
                    },
                );
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(M5ArtifactGraphPromotionViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.ledger_state,
                });
            }
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::ProofPacketStale)
            {
                violations.push(
                    M5ArtifactGraphPromotionViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
            {
                violations.push(
                    M5ArtifactGraphPromotionViolation::MissingPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    /// Every "if this aspect is bad, the matching reason must be active" rule.
    /// These apply to every family regardless of held/narrowing state, and encode
    /// the two guardrails: an emergency step may not bypass timeline capture or
    /// digest binding, and a mutable pointer may not stand in for immutable
    /// history.
    fn validate_history(
        &self,
        row: &FamilyPromotionLedger,
        violations: &mut Vec<M5ArtifactGraphPromotionViolation>,
    ) {
        let require = |violations: &mut Vec<M5ArtifactGraphPromotionViolation>,
                       bad: bool,
                       reason: NarrowingReason| {
            if bad && !row.has_active_reason(reason) {
                violations.push(M5ArtifactGraphPromotionViolation::HistoryGapWithoutReason {
                    entry_id: row.entry_id.clone(),
                    reason,
                });
            }
        };

        let capture_bypassed = row.timeline.is_empty()
            || row
                .timeline
                .iter()
                .any(|step| !FamilyPromotionLedger::step_capture_complete(step));
        let digest_missing = row.timeline.iter().any(|step| step.digest_refs.is_empty());
        let node_set_incomplete = row.affected_node_set.is_empty()
            || row
                .affected_node_set
                .iter()
                .any(|node| node.digest.digest_id.trim().is_empty())
            || row
                .timeline
                .iter()
                .any(|step| !row.step_digests_resolve(step));
        let reconstruction_divergent = !row.reconstruction.history_digests_match();
        let audit_unavailable = !row.reconstruction.audit_replay_available()
            || row.reconstruction.reconstructed_step_ids != row.timeline_step_ids();
        let break_glass_unreconciled = row
            .break_glass_steps()
            .iter()
            .any(|step| !FamilyPromotionLedger::break_glass_step_reconciled(step));
        let reversible_undisclosed = row
            .timeline
            .iter()
            .any(|step| !FamilyPromotionLedger::step_reversible_disclosed(step));
        let evidence_stale = row
            .timeline
            .iter()
            .any(FamilyPromotionLedger::step_evidence_blocks);

        require(
            violations,
            capture_bypassed,
            NarrowingReason::TimelineCaptureBypassed,
        );
        require(
            violations,
            digest_missing,
            NarrowingReason::DigestBindingMissing,
        );
        require(
            violations,
            node_set_incomplete,
            NarrowingReason::AffectedNodeSetIncomplete,
        );
        require(
            violations,
            row.history_pointer_class == HistoryPointerClass::MutableLatestPointer,
            NarrowingReason::MutableLatestPointer,
        );
        require(
            violations,
            reconstruction_divergent,
            NarrowingReason::ReconstructionDivergent,
        );
        require(
            violations,
            audit_unavailable,
            NarrowingReason::AuditReplayUnavailable,
        );
        require(
            violations,
            break_glass_unreconciled,
            NarrowingReason::BreakGlassUnreconciled,
        );
        require(
            violations,
            reversible_undisclosed,
            NarrowingReason::ReversibleWindowUndisclosed,
        );
        require(violations, evidence_stale, NarrowingReason::EvidenceStale);

        // Guardrail: a break-glass step must still be digest-bound and capture its
        // stages, actors, and evidence — an emergency may not bypass the timeline.
        for step in row.break_glass_steps() {
            if !FamilyPromotionLedger::step_capture_complete(step)
                || !row.step_digests_resolve(step)
            {
                violations.push(
                    M5ArtifactGraphPromotionViolation::BreakGlassBypassedCapture {
                        entry_id: row.entry_id.clone(),
                        timeline_step_id: step.timeline_step_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &FamilyPromotionLedger,
        violations: &mut Vec<M5ArtifactGraphPromotionViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<M5ArtifactGraphPromotionViolation>,
                               expected: NarrowingReason| {
            violations.push(M5ArtifactGraphPromotionViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.ledger_state,
                expected_reason: expected,
            });
        };

        match row.ledger_state {
            LedgerState::HistoryGap => {
                if !row
                    .active_narrowing_reasons
                    .iter()
                    .any(|reason| reason.is_history_gap())
                {
                    push_incoherent(violations, NarrowingReason::TimelineCaptureBypassed);
                }
            }
            LedgerState::Stale => {
                if !row.has_active_reason(NarrowingReason::ProofPacketStale)
                    && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
                {
                    push_incoherent(violations, NarrowingReason::ProofPacketStale);
                }
            }
            LedgerState::OwnerUnsigned => {
                if !row.has_active_reason(NarrowingReason::OwnerManifestUnsigned) {
                    push_incoherent(violations, NarrowingReason::OwnerManifestUnsigned);
                }
            }
            LedgerState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(
                        M5ArtifactGraphPromotionViolation::WaiverStateWithoutWaiver {
                            entry_id: row.entry_id.clone(),
                            state: row.ledger_state,
                        },
                    );
                }
            }
            LedgerState::Reconstructable => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5ArtifactGraphPromotionViolation>) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.candidate_ref.clone())
            .collect();
        for declared in &self.release_blocking_candidate_refs {
            if !covered.contains(declared) {
                violations.push(
                    M5ArtifactGraphPromotionViolation::ReleaseBlockingCandidateUncovered {
                        candidate_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking
                && !self
                    .release_blocking_candidate_refs
                    .contains(&row.candidate_ref)
            {
                violations.push(
                    M5ArtifactGraphPromotionViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<M5ArtifactGraphPromotionViolation>) {
        if self.publication.promotion_gate.trim().is_empty() {
            violations.push(M5ArtifactGraphPromotionViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(M5ArtifactGraphPromotionViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                M5ArtifactGraphPromotionViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                M5ArtifactGraphPromotionViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(
                M5ArtifactGraphPromotionViolation::PublicationBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// A validation violation for the promotion-ledger register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ArtifactGraphPromotionViolation {
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
    /// The register has no ledgers.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Ledger or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A ledger id appears more than once.
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
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Ledger id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A ledger holds a label while the claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Ledger id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing state carries no active reason.
    NarrowingWithoutReason {
        /// Ledger id.
        entry_id: String,
        /// Ledger state.
        state: LedgerState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Ledger id.
        entry_id: String,
        /// Ledger state.
        state: LedgerState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held ledger carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Ledger id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held ledger has active narrowing reasons.
    HeldWithActiveGap {
        /// Ledger id.
        entry_id: String,
    },
    /// A held ledger has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Ledger id.
        entry_id: String,
    },
    /// A held ledger rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Ledger id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held ledger lacks owner-manifest sign-off.
    HeldWithoutSignoff {
        /// Ledger id.
        entry_id: String,
    },
    /// A held ledger lets a mutable latest pointer stand in for immutable history.
    HeldOnMutablePointer {
        /// Ledger id.
        entry_id: String,
    },
    /// A held ledger records no promotion timeline.
    HeldWithoutTimeline {
        /// Ledger id.
        entry_id: String,
    },
    /// A held ledger records no affected node set.
    HeldWithoutNodeSet {
        /// Ledger id.
        entry_id: String,
    },
    /// A held ledger does not reconstruct the same history across flows.
    HeldWithoutReconstruction {
        /// Ledger id.
        entry_id: String,
        /// Parity state.
        state: ParityState,
    },
    /// A held ledger has no audit/postmortem replay.
    HeldWithoutAuditReplay {
        /// Ledger id.
        entry_id: String,
    },
    /// The reconstructed step ids disagree with the timeline order.
    ReconstructionStepMismatch {
        /// Ledger id.
        entry_id: String,
    },
    /// A reconstructable ledger carries a waiver.
    ClearedWithWaiver {
        /// Ledger id.
        entry_id: String,
    },
    /// A bad history aspect did not name its narrowing reason.
    HistoryGapWithoutReason {
        /// Ledger id.
        entry_id: String,
        /// The reason the aspect requires.
        reason: NarrowingReason,
    },
    /// A break-glass step bypassed timeline capture or digest binding.
    BreakGlassBypassedCapture {
        /// Ledger id.
        entry_id: String,
        /// Offending timeline step id.
        timeline_step_id: String,
    },
    /// A ledger state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Ledger id.
        entry_id: String,
        /// Ledger state.
        state: LedgerState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Ledger id.
        entry_id: String,
        /// Ledger state.
        state: LedgerState,
    },
    /// A narrowing ledger with a breached proof packet does not name the stale
    /// reason.
    BreachedPacketWithoutReason {
        /// Ledger id.
        entry_id: String,
    },
    /// A narrowing ledger with a missing proof packet does not name the missing
    /// reason.
    MissingPacketWithoutReason {
        /// Ledger id.
        entry_id: String,
    },
    /// A release-blocking candidate ref has no covering ledger.
    ReleaseBlockingCandidateUncovered {
        /// Candidate ref.
        candidate_ref: String,
    },
    /// A release-blocking ledger is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Ledger id.
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
    /// The summary counts disagree with the ledgers.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Ledger id.
        entry_id: String,
    },
}

impl fmt::Display for M5ArtifactGraphPromotionViolation {
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
            Self::EmptyRegister => write!(f, "register has no ledgers"),
            Self::NoStopRules => write!(f, "register has no stop rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => write!(f, "duplicate entry id {entry_id}"),
            Self::DuplicateStopRuleId { rule_id } => write!(f, "duplicate stop rule id {rule_id}"),
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::NarrowingReasonWithoutStopRule { reason } => write!(
                f,
                "narrowing reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "ledger {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "ledger {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "ledger {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "ledger {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "ledger {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "ledger {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "ledger {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => write!(
                f,
                "ledger {entry_id} holds stable on stale packet {slo_state:?}"
            ),
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "ledger {entry_id} holds stable without owner signoff")
            }
            Self::HeldOnMutablePointer { entry_id } => write!(
                f,
                "ledger {entry_id} holds stable on a mutable latest pointer"
            ),
            Self::HeldWithoutTimeline { entry_id } => {
                write!(f, "ledger {entry_id} holds stable without a timeline")
            }
            Self::HeldWithoutNodeSet { entry_id } => {
                write!(f, "ledger {entry_id} holds stable without an affected node set")
            }
            Self::HeldWithoutReconstruction { entry_id, state } => write!(
                f,
                "ledger {entry_id} holds stable without history reconstruction parity ({state:?})"
            ),
            Self::HeldWithoutAuditReplay { entry_id } => {
                write!(f, "ledger {entry_id} holds stable without audit replay")
            }
            Self::ReconstructionStepMismatch { entry_id } => write!(
                f,
                "ledger {entry_id} reconstructed step ids disagree with the timeline order"
            ),
            Self::ClearedWithWaiver { entry_id } => {
                write!(f, "reconstructable ledger {entry_id} carries a waiver")
            }
            Self::HistoryGapWithoutReason { entry_id, reason } => write!(
                f,
                "ledger {entry_id} history gap requires active reason {}",
                reason.as_str()
            ),
            Self::BreakGlassBypassedCapture {
                entry_id,
                timeline_step_id,
            } => write!(
                f,
                "ledger {entry_id} break-glass step {timeline_step_id} bypassed timeline capture or digest binding"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "ledger {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "ledger {entry_id} state {state:?} names no waiver")
            }
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "ledger {entry_id} breached packet without proof_packet_stale reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "ledger {entry_id} missing packet without proof_packet_missing reason"
            ),
            Self::ReleaseBlockingCandidateUncovered { candidate_ref } => write!(
                f,
                "release-blocking candidate {candidate_ref} has no covering ledger"
            ),
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking ledger {entry_id} is not declared in release_blocking_candidate_refs"
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "publication {declared:?} disagrees with computed {computed:?}"
            ),
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with ledgers"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "ledger {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for M5ArtifactGraphPromotionViolation {}

/// Loads the embedded promotion-ledger register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`M5ArtifactGraphPromotionRegister`].
pub fn current_m5_artifact_graph_promotion_ledger(
) -> Result<M5ArtifactGraphPromotionRegister, serde_json::Error> {
    serde_json::from_str(M5_ARTIFACT_GRAPH_PROMOTION_JSON)
}

#[cfg(test)]
mod tests;
