//! Typed rollback/revocation register binding each M5 artifact family to scoped
//! recovery records that target the smallest affected node set, to the
//! hosted/mirrored/offline delivery parity that proves every customer receives the
//! same recovery truth, and to the emergency-disable and advisory routing that
//! rides the same auditable record model as an ordinary rollback.
//!
//! Where the per-family release graph
//! (`implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family`)
//! speaks for the *release candidate* every M5 artifact family ships and the
//! promotion-ledger register
//! (`implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs`)
//! speaks for the *promotion history* every M5 artifact graph accumulates, this
//! register speaks for the *recovery posture* every M5 artifact graph must carry —
//! the inspectable rollback, revocation, yank, repin, and emergency-disable records
//! a release-center operator triggers, a headless automation flow replays, and a
//! security advisory routes to hosted, mirrored, and offline customers alike. Each
//! [`FamilyRecoveryLedger`] binds one artifact family to:
//!
//! - the [`RecoveryGraphNode`] set the family's recovery actions target, every node
//!   carrying a canonical [`ImmutableDigest`](crate::release_center_model::ImmutableDigest)
//!   and an [`RecoveryGraphNode::installable_after_action`] flag so a record joins to
//!   immutable graph material and so an unaffected node stays explicitly installable,
//! - one or more canonical
//!   [`RollbackOrRevocationRecord`](crate::release_center_model::RollbackOrRevocationRecord)
//!   recovery records, each carrying the affected and explicitly-preserved artifact
//!   refs, the blast-radius class, the last-known-good target, the linked advisory and
//!   revocation refs, the artifact-graph consistency after the action, and the
//!   break-glass disclosure for an emergency-disable,
//! - a [`ChannelDeliveryParity`] record proving the hosted, mirrored, and offline
//!   channels each received the same recovery record set and advisories, so an
//!   offline or mirrored customer is never a second-class citizen for emergency
//!   evidence,
//! - an owner manifest ([`FamilyRecoveryLedger::owner_signoff`]), a [`ProofPacket`]
//!   and its freshness SLO, and an optional waiver,
//! - the overall ledger state earned ([`RecoveryLedgerState`]), the active narrowing
//!   reasons ([`NarrowingReason`]), and the effective label after narrowing
//!   ([`FamilyRecoveryLedger::published_label`]).
//!
//! The [`LaunchCutline`] fixes the boundary between a ledger that may publish a
//! Stable claim and one that must narrow below it. The
//! [`M5ArtifactGraphRecoveryStopRule`] set names the closed conditions that gate
//! publication — one per [`NarrowingReason`] — and
//! [`M5ArtifactGraphRecoveryRegister::publication`] records the proceed/hold verdict.
//!
//! Two guardrails are encoded directly in
//! [`validate`](M5ArtifactGraphRecoveryRegister::validate): a recovery record may not
//! over-revoke — it may never list a node the graph model marks installable in its
//! affected (revoked) set when a smaller node-set action would preserve it — and a
//! family may not withhold emergency truth from the mirrored or offline channel while
//! the hosted channel already has it.
//!
//! The register is checked in at
//! `artifacts/release/m5/implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs.json`
//! and embedded here, so this typed consumer and the CI gate agree on every artifact
//! graph without a cargo build in CI.
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
    ArtifactGraphConsistency, AuthSourceClass, BlastRadiusClass, BreakGlassDisclosure,
    BreakGlassStateClass, ImmutableDigest, RolloutRing, RollbackOrRevocationKind,
    RollbackOrRevocationRecord,
};
use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

mod builder;
pub use builder::build_m5_artifact_graph_recovery_register;

/// Supported register schema version.
pub const M5_ARTIFACT_GRAPH_RECOVERY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_ARTIFACT_GRAPH_RECOVERY_RECORD_KIND: &str =
    "implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs";

/// Repo-relative path to the checked-in register.
pub const M5_ARTIFACT_GRAPH_RECOVERY_PATH: &str =
    "artifacts/release/m5/implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs.json";

/// Embedded checked-in register JSON.
pub const M5_ARTIFACT_GRAPH_RECOVERY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs.json"
));

/// A customer delivery channel that must receive current recovery truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryChannel {
    /// Hosted/managed customers served from the primary feed.
    Hosted,
    /// Mirrored/self-hosted customers served from a mirror feed.
    Mirrored,
    /// Offline/air-gapped customers served from an offline import path.
    Offline,
}

impl DeliveryChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 3] = [Self::Hosted, Self::Mirrored, Self::Offline];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hosted => "hosted",
            Self::Mirrored => "mirrored",
            Self::Offline => "offline",
        }
    }
}

/// Whether a delivery channel has received current recovery truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelDeliveryState {
    /// The channel received the family's current recovery record set and advisories.
    Current,
    /// Delivery is in flight and within its propagation window.
    Pending,
    /// The channel's recovery truth is stale and blocks the claim.
    Stale,
    /// The channel has no delivery path for the recovery truth at all.
    Undelivered,
}

impl ChannelDeliveryState {
    /// Every state, freshest to absent.
    pub const ALL: [Self; 4] = [Self::Current, Self::Pending, Self::Stale, Self::Undelivered];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Pending => "pending",
            Self::Stale => "stale",
            Self::Undelivered => "undelivered",
        }
    }

    /// Whether the state lets a family hold its label: only a current channel clears
    /// the gate.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Overall state a recovery ledger earned for its claimed label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLedgerState {
    /// Every recovery record is blast-radius-minimized and preserves unaffected
    /// nodes, the artifact graph stays consistent, the hosted/mirrored/offline
    /// channels are at parity, every emergency-disable is advisory-routed and
    /// reconciled, the proof packet is within SLO, and the owner has signed; the
    /// family publishes its claim.
    Contained,
    /// A recovery gap (blast-radius, unaffected-preservation, graph-consistency,
    /// last-known-good, channel-parity, advisory-routing, emergency-reconciliation,
    /// or stale-evidence gap) narrows the family below the cutline.
    RecoveryGap,
    /// The proof packet has gone stale or is missing.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers a
    /// recorded gap.
    OnWaiver,
    /// The owner manifest is unsigned.
    OwnerUnsigned,
}

impl RecoveryLedgerState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Contained,
        Self::RecoveryGap,
        Self::Stale,
        Self::OnWaiver,
        Self::OwnerUnsigned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Contained => "contained",
            Self::RecoveryGap => "recovery_gap",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::OwnerUnsigned => "owner_unsigned",
        }
    }

    /// Whether the state lets a ledger carry its claimed label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Contained | Self::OnWaiver)
    }
}

/// Closed reason a recovery ledger narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// A recovery record names no affected artifact ref, or does not classify every
    /// node in the affected set as affected or preserved (its blast radius is
    /// unscoped).
    BlastRadiusUnscoped,
    /// A recovery record does not list a node the graph model marks installable in
    /// its explicitly-preserved (unaffected) set.
    UnaffectedNodesNotPreserved,
    /// A recovery record leaves the artifact graph broken after the action.
    GraphConsistencyBroken,
    /// A rollback or repin record cites no last-known-good target.
    LastKnownGoodMissing,
    /// The mirrored channel has no delivery path for the recovery truth.
    MirrorParityMissing,
    /// The offline channel has no delivery path for the recovery truth.
    OfflineParityMissing,
    /// A delivery channel's recovery truth is stale.
    ChannelDeliveryStale,
    /// A revocation, yank, or emergency-disable record routes no security advisory.
    AdvisoryRoutingMissing,
    /// An emergency-disable record is active past its reconciliation window or names
    /// no reconciliation follow-up.
    EmergencyDisableUnreconciled,
    /// A recovery record rides evidence that is stale or missing and blocks the
    /// claim.
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
    pub const ALL: [Self; 14] = [
        Self::BlastRadiusUnscoped,
        Self::UnaffectedNodesNotPreserved,
        Self::GraphConsistencyBroken,
        Self::LastKnownGoodMissing,
        Self::MirrorParityMissing,
        Self::OfflineParityMissing,
        Self::ChannelDeliveryStale,
        Self::AdvisoryRoutingMissing,
        Self::EmergencyDisableUnreconciled,
        Self::EvidenceStale,
        Self::ProofPacketStale,
        Self::ProofPacketMissing,
        Self::OwnerManifestUnsigned,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlastRadiusUnscoped => "blast_radius_unscoped",
            Self::UnaffectedNodesNotPreserved => "unaffected_nodes_not_preserved",
            Self::GraphConsistencyBroken => "graph_consistency_broken",
            Self::LastKnownGoodMissing => "last_known_good_missing",
            Self::MirrorParityMissing => "mirror_parity_missing",
            Self::OfflineParityMissing => "offline_parity_missing",
            Self::ChannelDeliveryStale => "channel_delivery_stale",
            Self::AdvisoryRoutingMissing => "advisory_routing_missing",
            Self::EmergencyDisableUnreconciled => "emergency_disable_unreconciled",
            Self::EvidenceStale => "evidence_stale",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::OwnerManifestUnsigned => "owner_manifest_unsigned",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether this reason is a recovery gap (rather than a proof-packet or
    /// owner-manifest gap). The [`RecoveryLedgerState::RecoveryGap`] state must name
    /// at least one of these.
    pub const fn is_recovery_gap(self) -> bool {
        matches!(
            self,
            Self::BlastRadiusUnscoped
                | Self::UnaffectedNodesNotPreserved
                | Self::GraphConsistencyBroken
                | Self::LastKnownGoodMissing
                | Self::MirrorParityMissing
                | Self::OfflineParityMissing
                | Self::ChannelDeliveryStale
                | Self::AdvisoryRoutingMissing
                | Self::EmergencyDisableUnreconciled
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
    /// Re-target the record to the smallest affected node set.
    MinimizeBlastRadius,
    /// Explicitly preserve every unaffected node.
    PreserveUnaffectedNodes,
    /// Restore artifact-graph consistency after the action.
    RestoreGraphConsistency,
    /// Bind the record to a last-known-good target.
    BindLastKnownGood,
    /// Deliver the recovery truth to the mirrored channel.
    DeliverMirrorTruth,
    /// Deliver the recovery truth to the offline channel.
    DeliverOfflineTruth,
    /// Refresh the stale channel delivery.
    RefreshChannelDelivery,
    /// Route the security advisory for the action.
    RouteAdvisory,
    /// Reconcile the open emergency-disable action.
    ReconcileEmergencyDisable,
    /// Recapture the stale or missing record evidence.
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
    pub const ALL: [Self; 15] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::MinimizeBlastRadius,
        Self::PreserveUnaffectedNodes,
        Self::RestoreGraphConsistency,
        Self::BindLastKnownGood,
        Self::DeliverMirrorTruth,
        Self::DeliverOfflineTruth,
        Self::RefreshChannelDelivery,
        Self::RouteAdvisory,
        Self::ReconcileEmergencyDisable,
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
            Self::MinimizeBlastRadius => "minimize_blast_radius",
            Self::PreserveUnaffectedNodes => "preserve_unaffected_nodes",
            Self::RestoreGraphConsistency => "restore_graph_consistency",
            Self::BindLastKnownGood => "bind_last_known_good",
            Self::DeliverMirrorTruth => "deliver_mirror_truth",
            Self::DeliverOfflineTruth => "deliver_offline_truth",
            Self::RefreshChannelDelivery => "refresh_channel_delivery",
            Self::RouteAdvisory => "route_advisory",
            Self::ReconcileEmergencyDisable => "reconcile_emergency_disable",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::RenewWaiver => "renew_waiver",
        }
    }
}

/// True when a record kind must route a security advisory (a withdrawal action).
pub const fn kind_needs_advisory(kind: RollbackOrRevocationKind) -> bool {
    matches!(
        kind,
        RollbackOrRevocationKind::Revoke
            | RollbackOrRevocationKind::Yank
            | RollbackOrRevocationKind::EmergencyDisable
    )
}

/// True when a record kind must bind a last-known-good target (a restore action).
pub const fn kind_needs_last_known_good(kind: RollbackOrRevocationKind) -> bool {
    matches!(
        kind,
        RollbackOrRevocationKind::Rollback | RollbackOrRevocationKind::Repin
    )
}

/// One artifact-graph node in a family's affected node set.
///
/// Each node carries a canonical [`ImmutableDigest`](crate::release_center_model::ImmutableDigest)
/// and an [`installable_after_action`](RecoveryGraphNode::installable_after_action)
/// flag: a recovery record joins to the set by citing the node's artifact ref, and a
/// node that remains installable after the action is the explicit promise that an
/// unaffected node is preserved rather than swept up in an over-broad revocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryGraphNode {
    /// Stable node id, equal to the node's immutable digest id.
    pub node_id: String,
    /// Artifact ref this node represents in the graph.
    pub artifact_ref: String,
    /// The immutable digest of the node's material.
    pub digest: ImmutableDigest,
    /// Exact-build identity ref the node was produced under.
    pub exact_build_identity_ref: String,
    /// Whether the node remains installable after the family's recovery action.
    pub installable_after_action: bool,
    /// Reviewable one-line statement of the node.
    pub summary: String,
}

/// One channel's delivery of the family's recovery truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelDelivery {
    /// The customer delivery channel.
    pub channel: DeliveryChannel,
    /// Whether the channel received the current recovery truth.
    pub delivery_state: ChannelDeliveryState,
    /// Ref to the feed or import path the channel is served from.
    pub feed_ref: String,
    /// Recovery record ids the channel has received.
    #[serde(default)]
    pub delivered_record_ids: Vec<String>,
    /// Advisory refs the channel has received.
    #[serde(default)]
    pub advisory_refs: Vec<String>,
    /// UTC date the channel was last delivered to, or null when undelivered.
    #[serde(default)]
    pub delivered_at: Option<String>,
    /// Reviewable one-line statement of the channel delivery.
    pub summary: String,
}

/// Proof that the hosted, mirrored, and offline channels are at recovery parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelDeliveryParity {
    /// One delivery row per customer channel; must cover every [`DeliveryChannel`].
    pub channels: Vec<ChannelDelivery>,
    /// Reviewable one-line statement of the parity posture.
    pub summary: String,
}

impl ChannelDeliveryParity {
    /// The delivery row for `channel`, if present.
    pub fn channel(&self, channel: DeliveryChannel) -> Option<&ChannelDelivery> {
        self.channels.iter().find(|c| c.channel == channel)
    }

    /// The delivery state for `channel`, if present.
    pub fn channel_state(&self, channel: DeliveryChannel) -> Option<ChannelDeliveryState> {
        self.channel(channel).map(|c| c.delivery_state)
    }

    /// True when every channel is present and current.
    pub fn all_channels_current(&self) -> bool {
        DeliveryChannel::ALL
            .iter()
            .all(|channel| self.channel_state(*channel) == Some(ChannelDeliveryState::Current))
    }

    /// True when every channel delivers exactly `record_ids` (no channel is missing
    /// the recovery truth that another channel already has).
    pub fn channels_at_parity(&self, record_ids: &BTreeSet<String>) -> bool {
        DeliveryChannel::ALL.iter().all(|channel| {
            self.channel(*channel)
                .map(|c| {
                    let delivered: BTreeSet<String> =
                        c.delivered_record_ids.iter().cloned().collect();
                    &delivered == record_ids
                })
                .unwrap_or(false)
        })
    }
}

/// One recovery-ledger stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ArtifactGraphRecoveryStopRule {
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

/// One recovery ledger for one M5 artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyRecoveryLedger {
    /// Stable ledger id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The M5 artifact family this ledger tracks.
    pub family_kind: M5ArtifactFamilyKind,
    /// The artifact graph ref this family's nodes belong to.
    pub artifact_graph_ref: String,
    /// The release-candidate ref whose recovery this ledger records.
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
    pub ledger_state: RecoveryLedgerState,
    /// The artifact-graph node set the family's recovery actions target.
    pub affected_node_set: Vec<RecoveryGraphNode>,
    /// The scoped rollback/revocation records for the family.
    pub recovery_records: Vec<RollbackOrRevocationRecord>,
    /// The hosted/mirrored/offline channel delivery parity.
    pub channel_parity: ChannelDeliveryParity,
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

impl FamilyRecoveryLedger {
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

    /// The recovery record ids, as a set.
    pub fn record_ids(&self) -> BTreeSet<String> {
        self.recovery_records
            .iter()
            .map(|record| record.record_id.clone())
            .collect()
    }

    /// Every node's artifact ref.
    pub fn node_artifact_refs(&self) -> BTreeSet<String> {
        self.affected_node_set
            .iter()
            .map(|node| node.artifact_ref.clone())
            .collect()
    }

    /// Artifact refs of nodes that remain installable after the action.
    pub fn preserved_node_refs(&self) -> BTreeSet<String> {
        self.affected_node_set
            .iter()
            .filter(|node| node.installable_after_action)
            .map(|node| node.artifact_ref.clone())
            .collect()
    }

    /// True when `record` names at least one affected node ref and classifies every
    /// node in the family's node set as affected or explicitly preserved.
    pub fn record_blast_radius_scoped(&self, record: &RollbackOrRevocationRecord) -> bool {
        if record.affected_artifact_refs.is_empty() {
            return false;
        }
        let classified: BTreeSet<String> = record
            .affected_artifact_refs
            .iter()
            .chain(record.unaffected_artifact_refs.iter())
            .cloned()
            .collect();
        self.node_artifact_refs()
            .iter()
            .all(|node_ref| classified.contains(node_ref))
    }

    /// True when every installable node ref is in `record`'s explicitly-preserved
    /// (unaffected) set.
    pub fn record_preserves_unaffected(&self, record: &RollbackOrRevocationRecord) -> bool {
        let preserved: BTreeSet<String> = record.unaffected_artifact_refs.iter().cloned().collect();
        self.preserved_node_refs()
            .iter()
            .all(|node_ref| preserved.contains(node_ref))
    }

    /// True when `record` lists a node the graph model marks installable in its
    /// affected (revoked) set — the over-revoke the guardrail forbids.
    pub fn record_overrevokes(&self, record: &RollbackOrRevocationRecord) -> bool {
        let affected: BTreeSet<String> = record.affected_artifact_refs.iter().cloned().collect();
        self.preserved_node_refs()
            .iter()
            .any(|node_ref| affected.contains(node_ref))
    }

    /// True when `record` leaves the artifact graph broken after the action.
    pub fn record_graph_broken(record: &RollbackOrRevocationRecord) -> bool {
        matches!(
            record.artifact_graph_consistency,
            ArtifactGraphConsistency::Broken
        )
    }

    /// True when `record` keeps the artifact graph consistent (full or scoped).
    pub fn record_graph_consistent(record: &RollbackOrRevocationRecord) -> bool {
        matches!(
            record.artifact_graph_consistency,
            ArtifactGraphConsistency::ConsistentFullGraph
                | ArtifactGraphConsistency::ConsistentScopedException
        )
    }

    /// True when `record` is a restore action that cites no last-known-good target.
    pub fn record_last_known_good_missing(record: &RollbackOrRevocationRecord) -> bool {
        kind_needs_last_known_good(record.kind) && record.last_known_good_ref.trim().is_empty()
    }

    /// True when `record` is a withdrawal action that routes no advisory.
    pub fn record_advisory_missing(record: &RollbackOrRevocationRecord) -> bool {
        kind_needs_advisory(record.kind)
            && record
                .advisory_refs
                .iter()
                .all(|advisory| advisory.trim().is_empty())
    }

    /// True when `record` rides evidence that is stale or missing and blocks the
    /// claim.
    pub fn record_evidence_blocks(record: &RollbackOrRevocationRecord) -> bool {
        record
            .evidence_refs
            .iter()
            .any(|e| e.required_for_promotion && e.freshness_class.blocks_promotion())
    }

    /// True when `record` is an emergency-disable action.
    pub fn record_is_emergency_disable(record: &RollbackOrRevocationRecord) -> bool {
        matches!(record.kind, RollbackOrRevocationKind::EmergencyDisable)
            || BreakGlassDisclosure::state_requires_reconciliation(record.break_glass.state_class)
    }

    /// True when an emergency-disable record is reconciled or properly pending.
    pub fn record_emergency_reconciled(record: &RollbackOrRevocationRecord) -> bool {
        match record.break_glass.state_class {
            BreakGlassStateClass::Reconciled | BreakGlassStateClass::SupersededBySignedAction => {
                true
            }
            BreakGlassStateClass::ActivePendingReconciliation => {
                record.break_glass.reconcile_by.is_some()
                    && !record.break_glass.follow_up_refs.is_empty()
            }
            BreakGlassStateClass::ExpiredWithoutReconciliation
            | BreakGlassStateClass::ForbiddenForAction => false,
            BreakGlassStateClass::NotUsed | BreakGlassStateClass::EligibleButNotUsed => true,
        }
    }

    /// The emergency-disable records in the ledger.
    pub fn emergency_disable_records(&self) -> Vec<&RollbackOrRevocationRecord> {
        self.recovery_records
            .iter()
            .filter(|record| Self::record_is_emergency_disable(record))
            .collect()
    }

    /// True when the family carries a withdrawal record that routes an advisory: the
    /// emergency truth that the mirrored and offline channels must also receive.
    pub fn carries_routed_advisory(&self) -> bool {
        self.recovery_records.iter().any(|record| {
            kind_needs_advisory(record.kind)
                && record.advisory_refs.iter().any(|a| !a.trim().is_empty())
        })
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ArtifactGraphRecoverySummary {
    /// Total number of recovery ledgers.
    pub total_entries: usize,
    /// Distinct artifact graphs covered.
    pub total_artifact_graphs: usize,
    /// Ledgers publishing a label at or above the cutline.
    pub entries_contained: usize,
    /// Ledgers narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Ledgers holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Ledgers carrying a blast-radius or unaffected-preservation reason.
    pub entries_with_blast_radius_gap: usize,
    /// Ledgers carrying a graph-consistency reason.
    pub entries_with_graph_consistency_gap: usize,
    /// Ledgers carrying a channel-parity reason.
    pub entries_with_channel_parity_gap: usize,
    /// Ledgers carrying an advisory-routing reason.
    pub entries_with_advisory_gap: usize,
    /// Ledgers carrying an emergency-disable reason.
    pub entries_with_emergency_gap: usize,
    /// Total release-blocking families.
    pub release_blocking_total: usize,
    /// Release-blocking families publishing a label at or above the cutline.
    pub release_blocking_contained: usize,
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
    /// Channel deliveries whose state is `current`.
    pub channels_current: usize,
    /// Channel deliveries whose state is `pending`.
    pub channels_pending: usize,
    /// Channel deliveries whose state is `stale`.
    pub channels_stale: usize,
    /// Channel deliveries whose state is `undelivered`.
    pub channels_undelivered: usize,
    /// Recovery records whose kind is `rollback`.
    pub records_rollback: usize,
    /// Recovery records whose kind is `revoke`.
    pub records_revoke: usize,
    /// Recovery records whose kind is `yank`.
    pub records_yank: usize,
    /// Recovery records whose kind is `repin`.
    pub records_repin: usize,
    /// Recovery records whose kind is `emergency_disable`.
    pub records_emergency_disable: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total recovery records across all ledgers.
    pub total_recovery_records: usize,
    /// Total emergency-disable records across all ledgers.
    pub total_emergency_disable_records: usize,
    /// Total active narrowing reasons across all ledgers.
    pub total_active_narrowing_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One replay entry — the audit/advisory view of a single recovery record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryReplayEntry {
    /// Stable recovery record id.
    pub record_id: String,
    /// Recovery record kind.
    pub kind: RollbackOrRevocationKind,
    /// Blast-radius class.
    pub blast_radius_class: BlastRadiusClass,
    /// Number of affected artifact refs.
    pub affected_artifact_count: usize,
    /// Number of explicitly-preserved (unaffected) artifact refs.
    pub unaffected_artifact_count: usize,
    /// Artifact-graph consistency after the action.
    pub artifact_graph_consistency: ArtifactGraphConsistency,
    /// Last-known-good target ref.
    pub last_known_good_ref: String,
    /// Advisory refs routed for the action.
    pub advisory_refs: Vec<String>,
    /// Revocation record refs linked to the action.
    pub revocation_record_refs: Vec<String>,
    /// Auth-source class used by the action.
    pub auth_source_class: AuthSourceClass,
    /// Rollout ring or target scope.
    pub rollout_ring: RolloutRing,
    /// Break-glass state of the action.
    pub break_glass_state: BreakGlassStateClass,
    /// Whether the action is an emergency-disable.
    pub is_emergency_disable: bool,
}

/// One export row for downstream Help/About, support, advisory, and diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactGraphRecoveryExportRow {
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
    pub ledger_state: RecoveryLedgerState,
    /// Per-channel delivery states (hosted, mirrored, offline).
    pub channel_states: Vec<(DeliveryChannel, ChannelDeliveryState)>,
    /// Whether every channel is current and at parity.
    pub channels_at_parity: bool,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Number of recovery records.
    pub recovery_record_count: usize,
    /// Number of emergency-disable records.
    pub emergency_disable_count: usize,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The per-record audit/advisory replay.
    pub replay: Vec<RecoveryReplayEntry>,
}

/// Export projection for Help/About, support, advisory, and diagnostics surfaces.
///
/// Each row carries a `replay` that reconstructs every recovery action — its kind,
/// blast radius, affected and preserved node counts, last-known-good target, routed
/// advisories, and emergency state — and the hosted/mirrored/offline delivery states
/// so a support or advisory surface can prove every customer received the same
/// recovery truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactGraphRecoveryExportProjection {
    /// Register identifier.
    pub manifest_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5ArtifactGraphRecoveryExportRow>,
}

/// The typed M5 artifact-graph rollback/revocation register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ArtifactGraphRecoveryRegister {
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
    /// Ref to the promotion-ledger register this register sits beside.
    pub promotion_ledger_ref: String,
    /// Ref to the shared release-center object model.
    pub release_center_model_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed artifact-family-kind vocabulary.
    pub family_kinds: Vec<M5ArtifactFamilyKind>,
    /// Closed recovery-record-kind vocabulary.
    pub recovery_record_kinds: Vec<RollbackOrRevocationKind>,
    /// Closed delivery-channel vocabulary.
    pub delivery_channels: Vec<DeliveryChannel>,
    /// Closed channel-delivery-state vocabulary.
    pub channel_delivery_states: Vec<ChannelDeliveryState>,
    /// Closed ledger-state vocabulary.
    pub ledger_states: Vec<RecoveryLedgerState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking candidate refs this register covers.
    pub release_blocking_candidate_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<M5ArtifactGraphRecoveryStopRule>,
    /// Recovery ledgers.
    pub rows: Vec<FamilyRecoveryLedger>,
    /// Recorded publication verdict.
    pub publication: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5ArtifactGraphRecoverySummary,
}

impl M5ArtifactGraphRecoveryRegister {
    /// Returns the ledger registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&FamilyRecoveryLedger> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the ledgers publishing a label at or above the cutline.
    pub fn rows_contained(&self) -> Vec<&FamilyRecoveryLedger> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the ledgers narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&FamilyRecoveryLedger> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking ledgers.
    pub fn release_blocking_rows(&self) -> Vec<&FamilyRecoveryLedger> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the ledgers for one artifact-family kind.
    pub fn rows_for_kind(&self, kind: M5ArtifactFamilyKind) -> Vec<&FamilyRecoveryLedger> {
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
    pub fn stop_rule_fires(&self, rule: &M5ArtifactGraphRecoveryStopRule) -> bool {
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
    pub fn computed_summary(&self) -> M5ArtifactGraphRecoverySummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: M5ArtifactFamilyKind| self.rows_for_kind(kind).len();
        let channels = |state: ChannelDeliveryState| {
            self.rows
                .iter()
                .flat_map(|row| row.channel_parity.channels.iter())
                .filter(|c| c.delivery_state == state)
                .count()
        };
        let records = |record_kind: RollbackOrRevocationKind| {
            self.rows
                .iter()
                .flat_map(|row| row.recovery_records.iter())
                .filter(|r| r.kind == record_kind)
                .count()
        };
        let with_any = |reasons: &[NarrowingReason]| {
            self.rows
                .iter()
                .filter(|row| reasons.iter().any(|r| row.has_active_reason(*r)))
                .count()
        };
        let release_blocking: Vec<&FamilyRecoveryLedger> = self.release_blocking_rows();
        M5ArtifactGraphRecoverySummary {
            total_entries: self.rows.len(),
            total_artifact_graphs: self.artifact_graphs().len(),
            entries_contained: self
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
                .filter(|row| row.ledger_state == RecoveryLedgerState::OnWaiver)
                .count(),
            entries_with_blast_radius_gap: with_any(&[
                NarrowingReason::BlastRadiusUnscoped,
                NarrowingReason::UnaffectedNodesNotPreserved,
            ]),
            entries_with_graph_consistency_gap: with_any(&[
                NarrowingReason::GraphConsistencyBroken,
            ]),
            entries_with_channel_parity_gap: with_any(&[
                NarrowingReason::MirrorParityMissing,
                NarrowingReason::OfflineParityMissing,
                NarrowingReason::ChannelDeliveryStale,
            ]),
            entries_with_advisory_gap: with_any(&[NarrowingReason::AdvisoryRoutingMissing]),
            entries_with_emergency_gap: with_any(&[NarrowingReason::EmergencyDisableUnreconciled]),
            release_blocking_total: release_blocking.len(),
            release_blocking_contained: release_blocking
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
            channels_current: channels(ChannelDeliveryState::Current),
            channels_pending: channels(ChannelDeliveryState::Pending),
            channels_stale: channels(ChannelDeliveryState::Stale),
            channels_undelivered: channels(ChannelDeliveryState::Undelivered),
            records_rollback: records(RollbackOrRevocationKind::Rollback),
            records_revoke: records(RollbackOrRevocationKind::Revoke),
            records_yank: records(RollbackOrRevocationKind::Yank),
            records_repin: records(RollbackOrRevocationKind::Repin),
            records_emergency_disable: records(RollbackOrRevocationKind::EmergencyDisable),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_recovery_records: self.rows.iter().map(|row| row.recovery_records.len()).sum(),
            total_emergency_disable_records: self
                .rows
                .iter()
                .map(|row| row.emergency_disable_records().len())
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

    /// Produces an export/advisory-safe projection that downstream surfaces render
    /// instead of cloning status text. Each row carries a per-record replay and the
    /// hosted/mirrored/offline delivery states.
    pub fn support_export_projection(&self) -> M5ArtifactGraphRecoveryExportProjection {
        M5ArtifactGraphRecoveryExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5ArtifactGraphRecoveryExportRow {
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
                    channel_states: DeliveryChannel::ALL
                        .iter()
                        .filter_map(|channel| {
                            row.channel_parity
                                .channel_state(*channel)
                                .map(|state| (*channel, state))
                        })
                        .collect(),
                    channels_at_parity: row.channel_parity.all_channels_current()
                        && row.channel_parity.channels_at_parity(&row.record_ids()),
                    slo_state: row.proof_packet.slo_state,
                    recovery_record_count: row.recovery_records.len(),
                    emergency_disable_count: row.emergency_disable_records().len(),
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                    replay: row
                        .recovery_records
                        .iter()
                        .map(|record| RecoveryReplayEntry {
                            record_id: record.record_id.clone(),
                            kind: record.kind,
                            blast_radius_class: record.blast_radius_class,
                            affected_artifact_count: record.affected_artifact_refs.len(),
                            unaffected_artifact_count: record.unaffected_artifact_refs.len(),
                            artifact_graph_consistency: record.artifact_graph_consistency,
                            last_known_good_ref: record.last_known_good_ref.clone(),
                            advisory_refs: record.advisory_refs.clone(),
                            revocation_record_refs: record.revocation_record_refs.clone(),
                            auth_source_class: record.auth_source_class,
                            rollout_ring: record.rollout_ring,
                            break_glass_state: record.break_glass.state_class,
                            is_emergency_disable: FamilyRecoveryLedger::record_is_emergency_disable(
                                record,
                            ),
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<M5ArtifactGraphRecoveryViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5ArtifactGraphRecoveryViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5ArtifactGraphRecoveryViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5ArtifactGraphRecoveryViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ArtifactGraphRecoveryViolation>) {
        if self.schema_version != M5_ARTIFACT_GRAPH_RECOVERY_SCHEMA_VERSION {
            violations.push(M5ArtifactGraphRecoveryViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_ARTIFACT_GRAPH_RECOVERY_RECORD_KIND {
            violations.push(M5ArtifactGraphRecoveryViolation::UnsupportedRecordKind {
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
            ("promotion_ledger_ref", &self.promotion_ledger_ref),
            ("release_center_model_ref", &self.release_center_model_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ArtifactGraphRecoveryViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        let closed = |violations: &mut Vec<M5ArtifactGraphRecoveryViolation>,
                      ok: bool,
                      field: &'static str| {
            if !ok {
                violations
                    .push(M5ArtifactGraphRecoveryViolation::ClosedVocabularyMismatch { field });
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
            self.recovery_record_kinds == recovery_record_kinds_all(),
            "recovery_record_kinds",
        );
        closed(
            violations,
            self.delivery_channels == DeliveryChannel::ALL.to_vec(),
            "delivery_channels",
        );
        closed(
            violations,
            self.channel_delivery_states == ChannelDeliveryState::ALL.to_vec(),
            "channel_delivery_states",
        );
        closed(
            violations,
            self.ledger_states == RecoveryLedgerState::ALL.to_vec(),
            "ledger_states",
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
            violations.push(M5ArtifactGraphRecoveryViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5ArtifactGraphRecoveryViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5ArtifactGraphRecoveryViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5ArtifactGraphRecoveryViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5ArtifactGraphRecoveryViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5ArtifactGraphRecoveryViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5ArtifactGraphRecoveryViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ArtifactGraphRecoveryViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5ArtifactGraphRecoveryViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations.push(
                    M5ArtifactGraphRecoveryViolation::NarrowingReasonWithoutStopRule { reason },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &FamilyRecoveryLedger,
        violations: &mut Vec<M5ArtifactGraphRecoveryViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("artifact_graph_ref", &row.artifact_graph_ref),
            ("candidate_ref", &row.candidate_ref),
            ("family_summary", &row.family_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("channel_parity.summary", &row.channel_parity.summary),
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
                violations.push(M5ArtifactGraphRecoveryViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_recovery(row, violations);

        // The ceiling: no family may publish a label wider than the claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5ArtifactGraphRecoveryViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window consistent.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5ArtifactGraphRecoveryViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5ArtifactGraphRecoveryViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A claim whose canonical label is below the cutline forces the family to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(M5ArtifactGraphRecoveryViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(M5ArtifactGraphRecoveryViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.ledger_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A contained/on-waiver family publishes exactly the claim's canonical
            // label, carries no active reason, rides a captured within-SLO packet,
            // and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(M5ArtifactGraphRecoveryViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(M5ArtifactGraphRecoveryViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(M5ArtifactGraphRecoveryViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(M5ArtifactGraphRecoveryViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(M5ArtifactGraphRecoveryViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            // The held invariants on the ledger itself.
            if row.recovery_records.is_empty() {
                violations.push(
                    M5ArtifactGraphRecoveryViolation::HeldWithoutRecoveryRecords {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if row.affected_node_set.is_empty() {
                violations.push(M5ArtifactGraphRecoveryViolation::HeldWithoutNodeSet {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !(row.channel_parity.all_channels_current()
                && row.channel_parity.channels_at_parity(&row.record_ids()))
            {
                violations.push(M5ArtifactGraphRecoveryViolation::HeldWithoutChannelParity {
                    entry_id: row.entry_id.clone(),
                });
            }
            // A contained family carries no waiver; an on-waiver family carries a
            // valid one.
            match row.ledger_state {
                RecoveryLedgerState::Contained => {
                    if row.waiver.is_some() {
                        violations.push(M5ArtifactGraphRecoveryViolation::ClearedWithWaiver {
                            entry_id: row.entry_id.clone(),
                        });
                    }
                }
                RecoveryLedgerState::OnWaiver => {
                    if row
                        .waiver
                        .as_ref()
                        .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                        .unwrap_or(true)
                    {
                        violations.push(
                            M5ArtifactGraphRecoveryViolation::WaiverStateWithoutWaiver {
                                entry_id: row.entry_id.clone(),
                                state: row.ledger_state,
                            },
                        );
                    }
                }
                _ => {}
            }
        } else {
            // A narrowing state must drop the published label below the cutline and
            // name at least one active reason.
            if row.publishes_stable() {
                violations.push(
                    M5ArtifactGraphRecoveryViolation::PublishedLabelNotNarrowed {
                        entry_id: row.entry_id.clone(),
                        state: row.ledger_state,
                        published: row.published_label,
                    },
                );
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(M5ArtifactGraphRecoveryViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.ledger_state,
                });
            }
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::ProofPacketStale)
            {
                violations.push(
                    M5ArtifactGraphRecoveryViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
            {
                violations.push(
                    M5ArtifactGraphRecoveryViolation::MissingPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    /// Every "if this aspect is bad, the matching reason must be active" rule. These
    /// apply to every family regardless of held/narrowing state, and encode the two
    /// guardrails: a record may not over-revoke a preservable node, and an
    /// emergency-bearing family may not withhold the truth from the mirrored or
    /// offline channel while the hosted channel already has it.
    fn validate_recovery(
        &self,
        row: &FamilyRecoveryLedger,
        violations: &mut Vec<M5ArtifactGraphRecoveryViolation>,
    ) {
        let require = |violations: &mut Vec<M5ArtifactGraphRecoveryViolation>,
                       bad: bool,
                       reason: NarrowingReason| {
            if bad && !row.has_active_reason(reason) {
                violations.push(M5ArtifactGraphRecoveryViolation::RecoveryGapWithoutReason {
                    entry_id: row.entry_id.clone(),
                    reason,
                });
            }
        };

        let no_records = row.recovery_records.is_empty();
        let no_nodes = row.affected_node_set.is_empty();
        let blast_radius_unscoped = no_records
            || no_nodes
            || row
                .recovery_records
                .iter()
                .any(|record| !row.record_blast_radius_scoped(record));
        let unaffected_not_preserved = row
            .recovery_records
            .iter()
            .any(|record| !row.record_preserves_unaffected(record));
        let graph_broken = row
            .recovery_records
            .iter()
            .any(FamilyRecoveryLedger::record_graph_broken);
        let last_known_good_missing = row
            .recovery_records
            .iter()
            .any(FamilyRecoveryLedger::record_last_known_good_missing);
        let mirror_missing = row.channel_parity.channel_state(DeliveryChannel::Mirrored)
            == Some(ChannelDeliveryState::Undelivered);
        let offline_missing = row.channel_parity.channel_state(DeliveryChannel::Offline)
            == Some(ChannelDeliveryState::Undelivered);
        let channel_stale = row
            .channel_parity
            .channels
            .iter()
            .any(|c| c.delivery_state == ChannelDeliveryState::Stale);
        let advisory_missing = row
            .recovery_records
            .iter()
            .any(FamilyRecoveryLedger::record_advisory_missing);
        let emergency_unreconciled = row
            .emergency_disable_records()
            .iter()
            .any(|record| !FamilyRecoveryLedger::record_emergency_reconciled(record));
        let evidence_stale = row
            .recovery_records
            .iter()
            .any(FamilyRecoveryLedger::record_evidence_blocks);

        require(
            violations,
            blast_radius_unscoped,
            NarrowingReason::BlastRadiusUnscoped,
        );
        require(
            violations,
            unaffected_not_preserved,
            NarrowingReason::UnaffectedNodesNotPreserved,
        );
        require(
            violations,
            graph_broken,
            NarrowingReason::GraphConsistencyBroken,
        );
        require(
            violations,
            last_known_good_missing,
            NarrowingReason::LastKnownGoodMissing,
        );
        require(
            violations,
            mirror_missing,
            NarrowingReason::MirrorParityMissing,
        );
        require(
            violations,
            offline_missing,
            NarrowingReason::OfflineParityMissing,
        );
        require(
            violations,
            channel_stale,
            NarrowingReason::ChannelDeliveryStale,
        );
        require(
            violations,
            advisory_missing,
            NarrowingReason::AdvisoryRoutingMissing,
        );
        require(
            violations,
            emergency_unreconciled,
            NarrowingReason::EmergencyDisableUnreconciled,
        );
        require(violations, evidence_stale, NarrowingReason::EvidenceStale);

        // Guardrail: a record may not over-revoke — list a node the graph model marks
        // installable in its affected (revoked) set when a smaller node-set action
        // would preserve it.
        for record in &row.recovery_records {
            if row.record_overrevokes(record) {
                violations.push(
                    M5ArtifactGraphRecoveryViolation::OverRevokedPreservableNode {
                        entry_id: row.entry_id.clone(),
                        record_id: record.record_id.clone(),
                    },
                );
            }
        }

        // Guardrail: an emergency-bearing family may not withhold the truth from the
        // mirrored or offline channel while the hosted channel already has it.
        if row.carries_routed_advisory()
            && row.channel_parity.channel_state(DeliveryChannel::Hosted)
                == Some(ChannelDeliveryState::Current)
        {
            for channel in [DeliveryChannel::Mirrored, DeliveryChannel::Offline] {
                if row.channel_parity.channel_state(channel)
                    == Some(ChannelDeliveryState::Undelivered)
                {
                    violations.push(
                        M5ArtifactGraphRecoveryViolation::EmergencyTruthWithheldFromChannel {
                            entry_id: row.entry_id.clone(),
                            channel,
                        },
                    );
                }
            }
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &FamilyRecoveryLedger,
        violations: &mut Vec<M5ArtifactGraphRecoveryViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<M5ArtifactGraphRecoveryViolation>,
                               expected: NarrowingReason| {
            violations.push(M5ArtifactGraphRecoveryViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.ledger_state,
                expected_reason: expected,
            });
        };

        match row.ledger_state {
            RecoveryLedgerState::RecoveryGap => {
                if !row
                    .active_narrowing_reasons
                    .iter()
                    .any(|reason| reason.is_recovery_gap())
                {
                    push_incoherent(violations, NarrowingReason::BlastRadiusUnscoped);
                }
            }
            RecoveryLedgerState::Stale => {
                if !row.has_active_reason(NarrowingReason::ProofPacketStale)
                    && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
                {
                    push_incoherent(violations, NarrowingReason::ProofPacketStale);
                }
            }
            RecoveryLedgerState::OwnerUnsigned => {
                if !row.has_active_reason(NarrowingReason::OwnerManifestUnsigned) {
                    push_incoherent(violations, NarrowingReason::OwnerManifestUnsigned);
                }
            }
            RecoveryLedgerState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(M5ArtifactGraphRecoveryViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.ledger_state,
                    });
                }
            }
            RecoveryLedgerState::Contained => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5ArtifactGraphRecoveryViolation>) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.candidate_ref.clone())
            .collect();
        for declared in &self.release_blocking_candidate_refs {
            if !covered.contains(declared) {
                violations.push(
                    M5ArtifactGraphRecoveryViolation::ReleaseBlockingCandidateUncovered {
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
                    M5ArtifactGraphRecoveryViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<M5ArtifactGraphRecoveryViolation>) {
        if self.publication.promotion_gate.trim().is_empty() {
            violations.push(M5ArtifactGraphRecoveryViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(M5ArtifactGraphRecoveryViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                M5ArtifactGraphRecoveryViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                M5ArtifactGraphRecoveryViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(
                M5ArtifactGraphRecoveryViolation::PublicationBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// The closed recovery-record-kind vocabulary, in declaration order.
fn recovery_record_kinds_all() -> Vec<RollbackOrRevocationKind> {
    vec![
        RollbackOrRevocationKind::Rollback,
        RollbackOrRevocationKind::Revoke,
        RollbackOrRevocationKind::Yank,
        RollbackOrRevocationKind::Repin,
        RollbackOrRevocationKind::EmergencyDisable,
    ]
}

/// A validation violation for the rollback/revocation register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ArtifactGraphRecoveryViolation {
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
        state: RecoveryLedgerState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Ledger id.
        entry_id: String,
        /// Ledger state.
        state: RecoveryLedgerState,
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
    /// A held ledger records no recovery records.
    HeldWithoutRecoveryRecords {
        /// Ledger id.
        entry_id: String,
    },
    /// A held ledger records no affected node set.
    HeldWithoutNodeSet {
        /// Ledger id.
        entry_id: String,
    },
    /// A held ledger does not have all delivery channels at recovery parity.
    HeldWithoutChannelParity {
        /// Ledger id.
        entry_id: String,
    },
    /// A contained ledger carries a waiver.
    ClearedWithWaiver {
        /// Ledger id.
        entry_id: String,
    },
    /// A bad recovery aspect did not name its narrowing reason.
    RecoveryGapWithoutReason {
        /// Ledger id.
        entry_id: String,
        /// The reason the aspect requires.
        reason: NarrowingReason,
    },
    /// A recovery record over-revoked a node the graph model marks installable.
    OverRevokedPreservableNode {
        /// Ledger id.
        entry_id: String,
        /// Offending recovery record id.
        record_id: String,
    },
    /// An emergency-bearing family withheld the truth from a channel the hosted
    /// channel already has.
    EmergencyTruthWithheldFromChannel {
        /// Ledger id.
        entry_id: String,
        /// The under-served channel.
        channel: DeliveryChannel,
    },
    /// A ledger state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Ledger id.
        entry_id: String,
        /// Ledger state.
        state: RecoveryLedgerState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Ledger id.
        entry_id: String,
        /// Ledger state.
        state: RecoveryLedgerState,
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

impl fmt::Display for M5ArtifactGraphRecoveryViolation {
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
            Self::HeldWithoutRecoveryRecords { entry_id } => {
                write!(f, "ledger {entry_id} holds stable without a recovery record")
            }
            Self::HeldWithoutNodeSet { entry_id } => {
                write!(f, "ledger {entry_id} holds stable without an affected node set")
            }
            Self::HeldWithoutChannelParity { entry_id } => write!(
                f,
                "ledger {entry_id} holds stable without hosted/mirrored/offline channel parity"
            ),
            Self::ClearedWithWaiver { entry_id } => {
                write!(f, "contained ledger {entry_id} carries a waiver")
            }
            Self::RecoveryGapWithoutReason { entry_id, reason } => write!(
                f,
                "ledger {entry_id} recovery gap requires active reason {}",
                reason.as_str()
            ),
            Self::OverRevokedPreservableNode {
                entry_id,
                record_id,
            } => write!(
                f,
                "ledger {entry_id} record {record_id} over-revoked a node the graph model marks installable"
            ),
            Self::EmergencyTruthWithheldFromChannel { entry_id, channel } => write!(
                f,
                "ledger {entry_id} withheld emergency truth from the {} channel that the hosted channel already has",
                channel.as_str()
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

impl Error for M5ArtifactGraphRecoveryViolation {}

/// Loads the embedded rollback/revocation register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`M5ArtifactGraphRecoveryRegister`].
pub fn current_m5_artifact_graph_recovery_register(
) -> Result<M5ArtifactGraphRecoveryRegister, serde_json::Error> {
    serde_json::from_str(M5_ARTIFACT_GRAPH_RECOVERY_JSON)
}

#[cfg(test)]
mod tests;
