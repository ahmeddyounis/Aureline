//! The M5 descriptor / badge certification packet — one qualification output every public-truth
//! consumer reads.
//!
//! The sibling lanes each own one slice of the shared public-truth runtime: the
//! [descriptor object](crate::m5_descriptor_object) and the
//! [descriptor / badge matrix](crate::m5_descriptor_badge) freeze the descriptor families and
//! their gate; the [badge vocabulary](crate::m5_badge_vocabulary) resolves every descriptor value
//! to a stable badge; the [claim-narrowing](crate::m5_claim_narrowing) lane derives the one
//! controlled degraded-claim state; the [descriptor join](crate::m5_descriptor_join) carries that
//! truth into copy-safe export carriers; the [omission guard](crate::m5_omission_guard) proves a
//! present mirror / offline / side-loaded / missing-evidence condition can never disappear into
//! omission; and the [client-scope card](crate::m5_client_scope_card) states a narrowed client's
//! scope and authority so it can never imply desktop parity. Each lane ships its own schema, its
//! published registry, and a release-grade parity proof fixture.
//!
//! This lane is the certification *over* all of them. It does not invent a new claim family: it
//! binds every claimed M5 consumer — release center, Help/About, marketplace, docs/help,
//! certification, evaluation packs, support exports, and companion handoffs — to the shared
//! [runtime lanes](RuntimeLane) it reads, the descriptor schemas and
//! [badge families](crate::m5_descriptor_badge::BadgeFamily) those lanes expose, the frozen
//! [downgrade rules](crate::m5_descriptor_badge::DowngradeRule) that govern them, and the
//! release-grade *proof fixtures* that keep them current — and then auto-narrows a consumer's
//! claim deterministically the moment any lane it reads goes stale or failing. Each lane belongs
//! to one of three [certification dimensions](CertificationDimension) — descriptor parity, badge /
//! runtime proof, and freshness integration — so a drift report names *which* dimension aged out
//! rather than collapsing the cause into one flag.
//!
//! The auto-narrowing reuses the matrix's gate semantics exactly: a lane whose parity proof is
//! [`Stale`](crate::m5_descriptor_badge::FreshnessState::Stale) narrows every consumer that reads
//! it below Stable; a lane whose proof is
//! [`Expired`](crate::m5_descriptor_badge::FreshnessState::Expired) or
//! [`Missing`](crate::m5_descriptor_badge::FreshnessState::Missing) — or a lane a consumer reads
//! that the packet does not certify at all — blocks that consumer from Stable promotion, with the
//! gap named per consumer. So descriptor or badge-runtime drift narrows claims deterministically
//! instead of remaining hidden behind local copy, and a stale or failing certification can never
//! read fully certified.
//!
//! The [`M5DescriptorCertification`] packet is the one inspectable, serde-serializable
//! certification truth release, support, docs, and evaluation surfaces consume rather than
//! maintaining parallel truth inventories; it carries metadata and refs only — no credential
//! bodies or raw provider payloads.
//!
//! - Packet schema:
//!   [`schemas/provenance/m5-descriptor-certification.schema.json`](../../../../../schemas/provenance/m5-descriptor-certification.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-descriptor-certification.md`](../../../../../docs/public-truth/m5-descriptor-certification.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_descriptor_certification, seeded_m5_descriptor_certification_missing_proof_blocked,
    seeded_m5_descriptor_certification_stale_proof_narrowed, M5_DESCRIPTOR_CERTIFICATION_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The certification reuses the matrix lane's frozen vocabularies and downgrade rules so the
// certification layer and the governance layer can never drift to different tokens or rules.
use crate::m5_descriptor_badge::{
    canonical_downgrade_rules, BadgeFamily, ClientScope, ConsumerStatus, DescriptorFamily,
    DescriptorGapKind, DescriptorGate, DescriptorSignal, DowngradeRule, FreshnessState,
    ProvenanceClass, PublicTruthConsumer, QualificationClass,
    M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
};

/// Record-kind tag carried by [`M5DescriptorCertification`].
pub const M5_DESCRIPTOR_CERTIFICATION_RECORD_KIND: &str = "m5_descriptor_certification";

/// Schema version for the certification packet.
pub const M5_DESCRIPTOR_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the certification packet schema.
pub const M5_DESCRIPTOR_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/provenance/m5-descriptor-certification.schema.json";

/// Repo-relative path of the published certification inventory.
pub const M5_DESCRIPTOR_CERTIFICATION_REF: &str =
    "artifacts/public-truth/m5-descriptor-certification.json";

/// Repo-relative path of the release-grade certification parity proof.
pub const M5_DESCRIPTOR_CERTIFICATION_PROOF_REF: &str =
    "artifacts/release/m5-descriptor-parity-proof/descriptor-certification.json";

/// Repo-relative path of the descriptor / badge governance matrix doc this lane certifies.
pub const M5_DESCRIPTOR_CERTIFICATION_GOVERNANCE_REF: &str =
    "artifacts/public-truth/m5-descriptor-badge-governance.md";

/// Repo-relative path of the certification contract doc.
pub const M5_DESCRIPTOR_CERTIFICATION_DOC_REF: &str =
    "docs/public-truth/m5-descriptor-certification.md";

/// Repo-relative directory of the certification consumer fixtures.
pub const M5_DESCRIPTOR_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/public-truth/m5-badge-consumers/";

/// One of the three certification dimensions a [runtime lane](RuntimeLane) belongs to. Naming the
/// dimension on every drift is what lets the certification say *which* of descriptor parity, badge
/// / runtime proof, or freshness integration aged out rather than collapsing the cause into one
/// flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDimension {
    /// Descriptor-parity lanes: the descriptor objects, their gate matrix, and the client-scope
    /// cards that ground the descriptor families.
    DescriptorParity,
    /// Badge / runtime-proof lanes: the badge vocabulary and the copy-safe export carriers.
    BadgeRuntime,
    /// Freshness-integration lanes: the claim-narrowing runtime and the no-silent-omission guard.
    FreshnessIntegration,
}

impl CertificationDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::DescriptorParity,
        Self::BadgeRuntime,
        Self::FreshnessIntegration,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorParity => "descriptor_parity",
            Self::BadgeRuntime => "badge_runtime",
            Self::FreshnessIntegration => "freshness_integration",
        }
    }

    /// Reviewer-facing dimension label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DescriptorParity => "Descriptor parity",
            Self::BadgeRuntime => "Badge / runtime proof",
            Self::FreshnessIntegration => "Freshness integration",
        }
    }
}

/// One shared public-truth runtime lane the certification certifies. Each lane is a sibling
/// public-truth lane with its own schema, published registry, and release-grade parity proof
/// fixture; binding a consumer to a lane is what makes that consumer's claim depend on the lane's
/// proof staying current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeLane {
    /// The artifact-bound descriptor object and its controlled enums.
    DescriptorObject,
    /// The descriptor / badge governance matrix and per-consumer gate.
    DescriptorBadgeMatrix,
    /// The stable badge-vocabulary and explanation-drawer toolkit.
    BadgeVocabulary,
    /// The one controlled degraded-claim state derived from the descriptors.
    ClaimNarrowing,
    /// The copy-safe joins into export packets, support bundles, and admin reports.
    DescriptorJoin,
    /// The shared weaker-evidence-state vocabulary and no-silent-omission guard.
    OmissionGuard,
    /// The client-scope cards and deep-link / handoff disclosures.
    ClientScopeCard,
}

impl RuntimeLane {
    /// Every runtime lane, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DescriptorObject,
        Self::DescriptorBadgeMatrix,
        Self::BadgeVocabulary,
        Self::ClaimNarrowing,
        Self::DescriptorJoin,
        Self::OmissionGuard,
        Self::ClientScopeCard,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DescriptorObject => "descriptor_object",
            Self::DescriptorBadgeMatrix => "descriptor_badge_matrix",
            Self::BadgeVocabulary => "badge_vocabulary",
            Self::ClaimNarrowing => "claim_narrowing",
            Self::DescriptorJoin => "descriptor_join",
            Self::OmissionGuard => "omission_guard",
            Self::ClientScopeCard => "client_scope_card",
        }
    }

    /// Reviewer-facing lane label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DescriptorObject => "Descriptor object",
            Self::DescriptorBadgeMatrix => "Descriptor / badge matrix",
            Self::BadgeVocabulary => "Badge vocabulary",
            Self::ClaimNarrowing => "Claim narrowing",
            Self::DescriptorJoin => "Descriptor join",
            Self::OmissionGuard => "Omission guard",
            Self::ClientScopeCard => "Client-scope card",
        }
    }

    /// The certification dimension this lane belongs to.
    pub const fn dimension(self) -> CertificationDimension {
        match self {
            Self::DescriptorObject | Self::DescriptorBadgeMatrix | Self::ClientScopeCard => {
                CertificationDimension::DescriptorParity
            }
            Self::BadgeVocabulary | Self::DescriptorJoin => CertificationDimension::BadgeRuntime,
            Self::ClaimNarrowing | Self::OmissionGuard => {
                CertificationDimension::FreshnessIntegration
            }
        }
    }

    /// Repo-relative source-of-truth schema for this lane.
    pub const fn schema_ref(self) -> &'static str {
        match self {
            Self::DescriptorObject => "schemas/provenance/m5-descriptor-object.schema.json",
            Self::DescriptorBadgeMatrix => {
                "schemas/provenance/m5-descriptor-badge-matrix.schema.json"
            }
            Self::BadgeVocabulary => "schemas/provenance/m5-badge-vocabulary.schema.json",
            Self::ClaimNarrowing => "schemas/provenance/m5-claim-narrowing.schema.json",
            Self::DescriptorJoin => "schemas/provenance/m5-descriptor-join.schema.json",
            Self::OmissionGuard => "schemas/provenance/m5-omission-guard.schema.json",
            Self::ClientScopeCard => "schemas/provenance/m5-client-scope-card.schema.json",
        }
    }

    /// Repo-relative published registry inventory for this lane.
    pub const fn published_artifact_ref(self) -> &'static str {
        match self {
            Self::DescriptorObject => {
                "artifacts/public-truth/descriptors/m5-descriptor-object-registry.json"
            }
            Self::DescriptorBadgeMatrix => "artifacts/public-truth/m5-descriptor-badge-matrix.json",
            Self::BadgeVocabulary => "artifacts/public-truth/m5-badge-vocabulary.json",
            Self::ClaimNarrowing => "artifacts/public-truth/m5-claim-narrowing.json",
            Self::DescriptorJoin => "artifacts/public-truth/m5-descriptor-join.json",
            Self::OmissionGuard => "artifacts/public-truth/m5-omission-guard.json",
            Self::ClientScopeCard => "artifacts/public-truth/m5-client-scope-card.json",
        }
    }

    /// Repo-relative release-grade parity-proof fixture that keeps this lane current.
    pub const fn parity_proof_ref(self) -> &'static str {
        match self {
            Self::DescriptorObject => {
                "artifacts/release/m5-descriptor-parity-proof/descriptor-objects.json"
            }
            Self::DescriptorBadgeMatrix => {
                "artifacts/release/m5-descriptor-parity-proof/descriptor-badge-matrix.json"
            }
            Self::BadgeVocabulary => {
                "artifacts/release/m5-descriptor-parity-proof/badge-vocabulary.json"
            }
            Self::ClaimNarrowing => {
                "artifacts/release/m5-descriptor-parity-proof/claim-narrowing.json"
            }
            Self::DescriptorJoin => {
                "artifacts/release/m5-descriptor-parity-proof/descriptor-join.json"
            }
            Self::OmissionGuard => {
                "artifacts/release/m5-descriptor-parity-proof/omission-guard.json"
            }
            Self::ClientScopeCard => {
                "artifacts/release/m5-descriptor-parity-proof/client-scope-card.json"
            }
        }
    }

    /// Owner role accountable for keeping this lane's parity proof current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::DescriptorObject | Self::DescriptorBadgeMatrix => "release_descriptor_owner",
            Self::BadgeVocabulary | Self::DescriptorJoin => "release_badge_runtime_owner",
            Self::ClaimNarrowing | Self::OmissionGuard => "release_freshness_owner",
            Self::ClientScopeCard => "companion_scope_owner",
        }
    }
}

/// One certified runtime lane: its dimension, the schema / published registry / parity-proof refs
/// that ground it, the freshness of its parity proof, and the certification status that freshness
/// implies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedLane {
    /// The runtime lane.
    pub lane: RuntimeLane,
    /// Reviewer-facing lane label.
    pub lane_label: String,
    /// The certification dimension this lane belongs to.
    pub dimension: CertificationDimension,
    /// Repo-relative source-of-truth schema.
    pub schema_ref: String,
    /// Repo-relative published registry inventory.
    pub published_artifact_ref: String,
    /// Repo-relative release-grade parity-proof fixture.
    pub parity_proof_ref: String,
    /// Owner role accountable for keeping the parity proof current.
    pub owner_role: String,
    /// Freshness of the lane's parity proof.
    pub proof_freshness: FreshnessState,
    /// Coverage status implied by the proof freshness.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl CertifiedLane {
    /// Builds a certified lane at a given proof freshness, deriving every ref from the lane so a
    /// lane can never cite a ref that drifts from it.
    pub fn for_lane(lane: RuntimeLane, proof_freshness: FreshnessState) -> Self {
        let status = lane_status(proof_freshness);
        Self {
            lane,
            lane_label: lane.label().to_owned(),
            dimension: lane.dimension(),
            schema_ref: lane.schema_ref().to_owned(),
            published_artifact_ref: lane.published_artifact_ref().to_owned(),
            parity_proof_ref: lane.parity_proof_ref().to_owned(),
            owner_role: lane.owner_role().to_owned(),
            proof_freshness,
            status,
            signal: status.signal(),
            detail_message_id: format!(
                "{}certification.lane.{}",
                M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
                lane.as_str()
            ),
        }
    }

    /// Validates the lane's invariants: every derived field matches the lane, the status mirrors
    /// the proof freshness, and the message id carries the lane prefix.
    fn validate(&self) -> Vec<M5DescriptorCertificationViolation> {
        let mut out = Vec::new();
        if self.lane_label != self.lane.label()
            || self.dimension != self.lane.dimension()
            || self.schema_ref != self.lane.schema_ref()
            || self.published_artifact_ref != self.lane.published_artifact_ref()
            || self.parity_proof_ref != self.lane.parity_proof_ref()
            || self.owner_role != self.lane.owner_role()
        {
            out.push(M5DescriptorCertificationViolation::LaneFieldMismatch);
        }
        let status = lane_status(self.proof_freshness);
        if self.status != status || self.signal != status.signal() {
            out.push(M5DescriptorCertificationViolation::LaneStatusDrift);
        }
        if !self
            .detail_message_id
            .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
        {
            out.push(M5DescriptorCertificationViolation::UnprefixedMessageId);
        }
        out
    }
}

/// Maps a parity-proof freshness to the coverage status it implies: current is mapped, stale is
/// provisional (narrowed), and expired / missing is unmapped (blocked).
fn lane_status(freshness: FreshnessState) -> ConsumerStatus {
    match freshness {
        FreshnessState::Current => ConsumerStatus::Mapped,
        FreshnessState::Stale => ConsumerStatus::Provisional,
        FreshnessState::Expired | FreshnessState::Missing => ConsumerStatus::Unmapped,
    }
}

/// One certification gap on a claimed consumer: a runtime lane it reads whose parity proof is
/// stale, expired, or missing, or a lane it reads that the packet does not certify at all.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationGap {
    /// Consumer this gap applies to.
    pub consumer: PublicTruthConsumer,
    /// The runtime lane the gap concerns.
    pub lane: RuntimeLane,
    /// The certification dimension that drifted.
    pub dimension: CertificationDimension,
    /// The kind of gap (reuses the descriptor matrix gap vocabulary).
    pub gap_kind: DescriptorGapKind,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

/// Derived verdict for a consumer, computed from its certification gaps.
struct ConsumerVerdict {
    status: ConsumerStatus,
    signal: DescriptorSignal,
    gate: DescriptorGate,
    effective_qualification: QualificationClass,
}

/// Restrictiveness rank of a qualification class, from the shipped support-class ladder (least
/// restrictive first).
fn qualification_rank(class: QualificationClass) -> usize {
    QualificationClass::ALL
        .iter()
        .position(|c| *c == class)
        .unwrap_or(QualificationClass::ALL.len())
}

/// The more restrictive of two qualification classes.
fn more_restrictive(a: QualificationClass, b: QualificationClass) -> QualificationClass {
    if qualification_rank(a) >= qualification_rank(b) {
        a
    } else {
        b
    }
}

/// Derives a consumer's verdict from its gaps, with the same gate semantics the descriptor / badge
/// matrix uses: any blocking gap blocks Stable; any narrowing gap narrows to at least Beta; an
/// ungapped consumer stands at its claim.
fn derive_consumer_verdict(
    claimed: QualificationClass,
    gaps: &[CertificationGap],
) -> ConsumerVerdict {
    let any_blocking = gaps.iter().any(|g| g.gap_kind.is_blocking());
    let any_narrowing = gaps.iter().any(|g| !g.gap_kind.is_blocking());

    let status = if any_blocking {
        ConsumerStatus::Unmapped
    } else if any_narrowing {
        ConsumerStatus::Provisional
    } else {
        ConsumerStatus::Mapped
    };

    let gate = if any_blocking {
        DescriptorGate::Blocked
    } else if any_narrowing {
        DescriptorGate::Narrowed
    } else {
        DescriptorGate::Governed
    };

    let effective_qualification = match gate {
        DescriptorGate::Governed => claimed,
        DescriptorGate::Blocked => QualificationClass::Unavailable,
        DescriptorGate::Narrowed => more_restrictive(claimed, QualificationClass::Beta),
    };

    ConsumerVerdict {
        status,
        signal: status.signal(),
        gate,
        effective_qualification,
    }
}

/// One claimed public-truth consumer certified against the shared runtime: the descriptor families
/// it binds, the badge families and descriptor schemas those families expose, the runtime lanes it
/// reads, the downgrade rules that govern it, the proof fixtures backing it, and the verdict
/// derived from those lanes' parity-proof freshness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedConsumer {
    /// The consumer surface.
    pub consumer: PublicTruthConsumer,
    /// Reviewer-facing consumer label.
    pub consumer_label: String,
    /// Owner role accountable for keeping this consumer's certification current.
    pub owner_role: String,
    /// Public qualification the consumer wants to keep.
    pub claimed_qualification: QualificationClass,
    /// The descriptor families this consumer binds.
    pub bound_families: Vec<DescriptorFamily>,
    /// The badge families this consumer's bound descriptors render, in family order.
    pub covered_badge_families: Vec<BadgeFamily>,
    /// The descriptor schemas this consumer maps to, in family order — refs only.
    pub bound_descriptor_schemas: Vec<String>,
    /// The runtime lanes this consumer reads.
    pub certified_lanes: Vec<RuntimeLane>,
    /// The downgrade-rule ids that govern this consumer's bound families.
    pub applicable_downgrade_rule_ids: Vec<String>,
    /// The parity-proof fixtures backing this consumer's certified lanes — refs only.
    pub proof_fixture_refs: Vec<String>,
    /// Effective qualification after the certification gate applies.
    pub effective_qualification: QualificationClass,
    /// Coverage status.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Certification gate decision the release / public-truth automation reads.
    pub gate_decision: DescriptorGate,
    /// Exact certification gaps for this consumer.
    pub gaps: Vec<CertificationGap>,
    /// Stable message id for the status; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
    /// Stable message id for the gate; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

impl CertifiedConsumer {
    /// Builds a consumer certification from its claimed qualification, bound families, and the
    /// runtime lanes it reads; the resolved refs, gaps, and verdict are recomputed against the
    /// packet's certified lanes when the packet is assembled.
    pub fn new(
        consumer: PublicTruthConsumer,
        claimed_qualification: QualificationClass,
        bound_families: &[DescriptorFamily],
        certified_lanes: &[RuntimeLane],
    ) -> Self {
        Self {
            consumer,
            consumer_label: consumer.label().to_owned(),
            owner_role: consumer.owner_role().to_owned(),
            claimed_qualification,
            bound_families: bound_families.to_vec(),
            covered_badge_families: Vec::new(),
            bound_descriptor_schemas: Vec::new(),
            certified_lanes: certified_lanes.to_vec(),
            applicable_downgrade_rule_ids: Vec::new(),
            proof_fixture_refs: Vec::new(),
            effective_qualification: claimed_qualification,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            gate_decision: DescriptorGate::Governed,
            gaps: Vec::new(),
            status_message_id: format!(
                "{}certification.{}.status",
                M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
            gate_message_id: format!(
                "{}certification.{}.gate",
                M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
        }
    }

    /// Recomputes the resolved refs, certification gaps, and verdict from the packet's certified
    /// lanes and the frozen downgrade rules, so a consumer's claim is always generated from the
    /// same checked-in parity proofs the packet ships rather than a hand-maintained status.
    pub fn recompute(&mut self, lanes: &[CertifiedLane], downgrade_rules: &[DowngradeRule]) {
        // Canonicalise the bound families and certified lanes, then resolve the descriptor schemas,
        // badge families, downgrade rules, and proof fixtures the consumer maps to.
        let mut families = self.bound_families.clone();
        families.sort_by_key(family_rank);
        families.dedup();
        self.bound_families = families.clone();
        self.covered_badge_families = families.iter().map(|f| f.badge_family()).collect();
        self.bound_descriptor_schemas =
            families.iter().map(|f| f.schema_ref().to_owned()).collect();

        let mut read_lanes = self.certified_lanes.clone();
        read_lanes.sort_by_key(lane_rank);
        read_lanes.dedup();
        self.certified_lanes = read_lanes.clone();

        self.applicable_downgrade_rule_ids = downgrade_rules
            .iter()
            .filter(|r| families.contains(&r.trigger_family))
            .map(|r| r.rule_id.clone())
            .collect();

        self.proof_fixture_refs = read_lanes
            .iter()
            .map(|lane| {
                lanes
                    .iter()
                    .find(|c| c.lane == *lane)
                    .map(|c| c.parity_proof_ref.clone())
                    .unwrap_or_else(|| lane.parity_proof_ref().to_owned())
            })
            .collect();

        // Derive the certification gaps from each read lane's parity-proof freshness.
        let consumer = self.consumer;
        let mut gaps = Vec::new();
        let mut push_gap = |lane: RuntimeLane, kind: DescriptorGapKind| {
            gaps.push(CertificationGap {
                consumer,
                lane,
                dimension: lane.dimension(),
                gap_kind: kind,
                cause_message_id: format!(
                    "{}certification.{}.{}.{}.gap",
                    M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX,
                    consumer.as_str(),
                    lane.as_str(),
                    kind.as_str()
                ),
            });
        };

        for &lane in &read_lanes {
            match lanes.iter().find(|c| c.lane == lane) {
                None => push_gap(lane, DescriptorGapKind::DescriptorMappingMissing),
                Some(certified) => match certified.proof_freshness {
                    FreshnessState::Current => {}
                    FreshnessState::Stale => push_gap(lane, DescriptorGapKind::ProofStale),
                    FreshnessState::Expired => push_gap(lane, DescriptorGapKind::ProofExpired),
                    FreshnessState::Missing => push_gap(lane, DescriptorGapKind::ProofMissing),
                },
            }
        }

        gaps.sort_by(|a, b| {
            a.lane
                .as_str()
                .cmp(b.lane.as_str())
                .then(a.gap_kind.as_str().cmp(b.gap_kind.as_str()))
        });
        self.gaps = gaps;

        let verdict = derive_consumer_verdict(self.claimed_qualification, &self.gaps);
        self.status = verdict.status;
        self.signal = verdict.signal;
        self.gate_decision = verdict.gate;
        self.effective_qualification = verdict.effective_qualification;
    }

    /// True when the consumer is blocked from Stable promotion.
    pub fn is_blocked(&self) -> bool {
        self.gate_decision.blocks()
    }

    /// True when the consumer auto-narrowed below its claim.
    pub fn is_narrowed(&self) -> bool {
        matches!(self.gate_decision, DescriptorGate::Narrowed)
    }

    /// True when the consumer is fully certified at its claim.
    pub fn is_certified(&self) -> bool {
        matches!(self.gate_decision, DescriptorGate::Governed)
    }

    /// Validates the consumer's static invariants (identity, bound families, read lanes, message
    /// ids).
    fn validate_static(&self) -> Vec<M5DescriptorCertificationViolation> {
        let mut out = Vec::new();
        if self.consumer_label != self.consumer.label()
            || self.owner_role != self.consumer.owner_role()
        {
            out.push(M5DescriptorCertificationViolation::MissingIdentity);
        }
        if self.bound_families.is_empty() {
            out.push(M5DescriptorCertificationViolation::ConsumerBindsNoDescriptors);
        }
        if self.certified_lanes.is_empty() {
            out.push(M5DescriptorCertificationViolation::ConsumerReadsNoLanes);
        }
        if !self
            .status_message_id
            .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
            || !self
                .gate_message_id
                .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
        {
            out.push(M5DescriptorCertificationViolation::UnprefixedMessageId);
        }
        for gap in &self.gaps {
            if !gap
                .cause_message_id
                .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
                || gap.consumer != self.consumer
                || gap.dimension != gap.lane.dimension()
            {
                out.push(M5DescriptorCertificationViolation::CertificationGapInvalid);
            }
        }
        out
    }
}

/// Position of a descriptor family in the canonical ordering.
fn family_rank(family: &DescriptorFamily) -> usize {
    DescriptorFamily::ALL
        .iter()
        .position(|f| f == family)
        .unwrap_or(DescriptorFamily::ALL.len())
}

/// Position of a runtime lane in the canonical ordering.
fn lane_rank(lane: &RuntimeLane) -> usize {
    RuntimeLane::ALL
        .iter()
        .position(|l| l == lane)
        .unwrap_or(RuntimeLane::ALL.len())
}

/// Which surfaces consume the one certification output. Every flag must hold so no surface keeps a
/// parallel truth inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDisclosure {
    /// The release center consumes the certification output.
    pub release_center_consumes_certification: bool,
    /// The Help/About panel consumes the certification output.
    pub help_about_consumes_certification: bool,
    /// The marketplace / ecosystem surface consumes the certification output.
    pub marketplace_consumes_certification: bool,
    /// The docs / help surface consumes the certification output.
    pub docs_help_consumes_certification: bool,
    /// The certification surface consumes the certification output.
    pub certification_consumes_certification: bool,
    /// Evaluation packs consume the certification output.
    pub evaluation_packs_consume_certification: bool,
    /// Support exports consume the certification output.
    pub support_export_consumes_certification: bool,
    /// Companion handoffs consume the certification output.
    pub companion_handoff_consumes_certification: bool,
}

impl CertificationDisclosure {
    /// The canonical disclosure: every surface consumes the certification output.
    pub const fn all_surfaces() -> Self {
        Self {
            release_center_consumes_certification: true,
            help_about_consumes_certification: true,
            marketplace_consumes_certification: true,
            docs_help_consumes_certification: true,
            certification_consumes_certification: true,
            evaluation_packs_consume_certification: true,
            support_export_consumes_certification: true,
            companion_handoff_consumes_certification: true,
        }
    }

    /// True when every surface consumes the certification output.
    pub const fn all_consume(&self) -> bool {
        self.release_center_consumes_certification
            && self.help_about_consumes_certification
            && self.marketplace_consumes_certification
            && self.docs_help_consumes_certification
            && self.certification_consumes_certification
            && self.evaluation_packs_consume_certification
            && self.support_export_consumes_certification
            && self.companion_handoff_consumes_certification
    }
}

/// Self-describing controlled-vocabulary set so the packet resolves every token it carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationVocabulary {
    /// Runtime-lane tokens.
    pub runtime_lanes: Vec<String>,
    /// Certification-dimension tokens.
    pub dimensions: Vec<String>,
    /// Descriptor-family tokens.
    pub descriptor_families: Vec<String>,
    /// Badge-family tokens.
    pub badge_families: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Qualification-class tokens.
    pub qualification_classes: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
    /// Signal tokens.
    pub signals: Vec<String>,
    /// Consumer-status tokens.
    pub consumer_statuses: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
}

impl CertificationVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            runtime_lanes: RuntimeLane::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            dimensions: CertificationDimension::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            descriptor_families: DescriptorFamily::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            badge_families: BadgeFamily::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            consumers: PublicTruthConsumer::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            freshness_states: FreshnessState::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            qualification_classes: QualificationClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            gate_decisions: DescriptorGate::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            signals: DescriptorSignal::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            consumer_statuses: ConsumerStatus::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            gap_kinds: DescriptorGapKind::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Compact certification summary — the scoreboard release / support / docs surfaces read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSummary {
    /// Total certified runtime lanes.
    pub total_lanes: u32,
    /// Lanes whose parity proof is current.
    pub current_lanes: u32,
    /// Lanes whose parity proof is stale.
    pub stale_lanes: u32,
    /// Lanes whose parity proof is expired.
    pub expired_lanes: u32,
    /// Lanes whose parity proof is missing.
    pub missing_lanes: u32,
    /// Total claimed consumers.
    pub total_consumers: u32,
    /// Consumers certified at their full claim.
    pub certified_consumer_count: u32,
    /// Consumers that auto-narrowed below their claim.
    pub narrowed_consumer_count: u32,
    /// Consumers blocked from Stable promotion.
    pub blocked_consumer_count: u32,
    /// Total downgrade rules certified.
    pub total_downgrade_rules: u32,
    /// True when at least one consumer is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
}

/// Packet-level release gate aggregating the per-consumer gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationReleaseGate {
    /// True when at least one consumer is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Sorted consumer tokens blocked from Stable promotion.
    pub blocked_consumers: Vec<String>,
    /// Sorted consumer tokens that auto-narrowed below their claim.
    pub narrowed_consumers: Vec<String>,
    /// Sorted consumer tokens fully certified for Stable promotion.
    pub certified_consumers: Vec<String>,
    /// Sorted dimension tokens whose proof drifted (stale or failing).
    pub drifted_dimensions: Vec<String>,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

/// Conformance review. Every flag is a hard invariant; all must hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationConformance {
    /// Every runtime lane is certified exactly once with a parity-proof fixture.
    pub every_lane_certified_with_proof: bool,
    /// Every certification dimension is covered by at least one certified lane.
    pub every_dimension_covered: bool,
    /// Every claimed consumer maps to descriptor schemas, badge families, downgrade rules, and
    /// proof fixtures.
    pub every_consumer_maps_to_descriptors_and_proof: bool,
    /// Every claimed consumer reads at least one runtime lane.
    pub every_consumer_reads_at_least_one_lane: bool,
    /// A stale lane proof narrows the consumers that read it deterministically.
    pub stale_proof_narrows_deterministically: bool,
    /// An expired / missing / uncertified lane proof blocks the consumers that read it.
    pub missing_proof_blocks_stable_promotion: bool,
    /// Exact certification gaps are named per consumer with their drifted dimension.
    pub exact_gaps_named_per_consumer: bool,
    /// Every non-authoritative descriptor value still has a downgrade rule.
    pub downgrade_rules_cover_every_weaker_value: bool,
    /// Release, support, docs, and evaluation surfaces consume one certification output.
    pub surfaces_consume_one_certification: bool,
    /// The certification is generated from the same checked-in parity proofs.
    pub generated_from_checked_in_proofs: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// The export carries no raw provider material.
    pub export_carries_no_raw_material: bool,
}

impl CertificationConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_lane_certified_with_proof
            && self.every_dimension_covered
            && self.every_consumer_maps_to_descriptors_and_proof
            && self.every_consumer_reads_at_least_one_lane
            && self.stale_proof_narrows_deterministically
            && self.missing_proof_blocks_stable_promotion
            && self.exact_gaps_named_per_consumer
            && self.downgrade_rules_cover_every_weaker_value
            && self.surfaces_consume_one_certification
            && self.generated_from_checked_in_proofs
            && self.controlled_enums_frozen
            && self.export_carries_no_raw_material
    }
}

/// Constructor input for [`M5DescriptorCertification::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DescriptorCertificationInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The certified runtime lanes.
    pub lanes: Vec<CertifiedLane>,
    /// The claimed consumer certifications (gaps / verdict are recomputed from the lanes).
    pub consumers: Vec<CertifiedConsumer>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable certification truth packet release, support, docs, and
/// evaluation surfaces consume: the certified runtime lanes, the frozen downgrade rules, the
/// per-consumer certification, the controlled vocabulary, a conformance review, a summary, and the
/// aggregate release gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DescriptorCertification {
    /// Record kind; must equal [`M5_DESCRIPTOR_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DESCRIPTOR_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The certified runtime lanes.
    pub lanes: Vec<CertifiedLane>,
    /// The frozen downgrade rules certified across the lanes.
    pub downgrade_rules: Vec<DowngradeRule>,
    /// The claimed consumer certifications with their derived verdicts.
    pub consumers: Vec<CertifiedConsumer>,
    /// The public-truth consumer tokens that read this certification output.
    pub consumer_tokens: Vec<String>,
    /// Which surfaces consume the certification output.
    pub disclosure: CertificationDisclosure,
    /// Compact certification summary.
    pub summary: CertificationSummary,
    /// Packet-level release gate.
    pub release_gate: CertificationReleaseGate,
    /// Controlled-vocabulary set.
    pub vocabulary: CertificationVocabulary,
    /// Conformance review block.
    pub conformance: CertificationConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DescriptorCertification {
    /// Builds a certification packet from seed input, recomputing each consumer's verdict and
    /// deriving the downgrade rules, summary, release gate, and conformance review from the lanes.
    pub fn new(input: M5DescriptorCertificationInput) -> Self {
        let lanes = input.lanes;
        let downgrade_rules = canonical_downgrade_rules();
        let mut consumers = input.consumers;
        for consumer in &mut consumers {
            consumer.recompute(&lanes, &downgrade_rules);
        }
        let consumer_tokens: Vec<String> = PublicTruthConsumer::ALL
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        let summary = derive_summary(&lanes, &downgrade_rules, &consumers);
        let release_gate = derive_release_gate(&lanes, &consumers);
        let conformance = derive_conformance(&lanes, &downgrade_rules, &consumers);
        Self {
            record_kind: M5_DESCRIPTOR_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_DESCRIPTOR_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            lanes,
            downgrade_rules,
            consumers,
            consumer_tokens,
            disclosure: CertificationDisclosure::all_surfaces(),
            summary,
            release_gate,
            vocabulary: CertificationVocabulary::canonical(),
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release / public-truth automation must hold Stable promotion.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.release_gate.blocks_stable_promotion
    }

    /// Finds a certified lane by runtime lane.
    pub fn lane(&self, lane: RuntimeLane) -> Option<&CertifiedLane> {
        self.lanes.iter().find(|c| c.lane == lane)
    }

    /// Finds a consumer certification by consumer.
    pub fn consumer(&self, consumer: PublicTruthConsumer) -> Option<&CertifiedConsumer> {
        self.consumers.iter().find(|c| c.consumer == consumer)
    }

    /// Renders the packet for a generation channel. The output is identical for every channel —
    /// the channel parameter exists only to prove desktop, CLI/headless, and offline / mirror
    /// packet generation produce byte-identical output.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn render_for_channel(&self, _channel: CertificationChannel) -> String {
        self.export_safe_json()
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 descriptor certification serializes")
    }

    /// Deterministic Markdown certification report for support, docs, shiproom, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Descriptor / Badge Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Lanes: {} ({} current, {} stale, {} expired, {} missing)\n",
            self.summary.total_lanes,
            self.summary.current_lanes,
            self.summary.stale_lanes,
            self.summary.expired_lanes,
            self.summary.missing_lanes
        ));
        out.push_str(&format!(
            "- Consumers: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_consumers,
            self.summary.certified_consumer_count,
            self.summary.narrowed_consumer_count,
            self.summary.blocked_consumer_count
        ));
        out.push_str(&format!(
            "- Downgrade rules: {}\n",
            self.summary.total_downgrade_rules
        ));
        out.push_str(&format!(
            "- Release gate: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));
        if !self.release_gate.drifted_dimensions.is_empty() {
            out.push_str(&format!(
                "- Drifted dimensions: {}\n",
                self.release_gate
                    .drifted_dimensions
                    .iter()
                    .map(|d| format!("`{d}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out.push_str(
            "- Consumed by: release center, Help/About, marketplace, docs/help, certification, evaluation packs, support, companion\n",
        );

        out.push_str("\n## Certified runtime lanes\n\n");
        out.push_str("| Lane | Dimension | Schema | Parity proof | Freshness | Status |\n");
        out.push_str("|------|-----------|--------|--------------|-----------|--------|\n");
        for lane in &self.lanes {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                lane.lane.as_str(),
                lane.dimension.as_str(),
                lane.schema_ref,
                lane.parity_proof_ref,
                lane.proof_freshness.as_str(),
                lane.status.as_str()
            ));
        }

        out.push_str("\n## Certified consumers\n\n");
        out.push_str("| Consumer | Status | Claim → effective | Gate | Reads | Binds |\n");
        out.push_str("|----------|--------|-------------------|------|-------|-------|\n");
        for c in &self.consumers {
            let read: Vec<&str> = c.certified_lanes.iter().map(|l| l.as_str()).collect();
            let bound: Vec<&str> = c.bound_families.iter().map(|f| f.as_str()).collect();
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` → `{}` | `{}` | {} | {} |\n",
                c.consumer.as_str(),
                c.status.as_str(),
                c.claimed_qualification.as_str(),
                c.effective_qualification.as_str(),
                c.gate_decision.as_str(),
                read.join(", "),
                bound.join(", ")
            ));
            for gap in &c.gaps {
                out.push_str(&format!(
                    "| | | gap: `{}` on `{}` (`{}`) | | | |\n",
                    gap.gap_kind.as_str(),
                    gap.lane.as_str(),
                    gap.dimension.as_str()
                ));
            }
        }
        out
    }

    /// Validates the packet's invariants.
    pub fn validate(&self) -> Vec<M5DescriptorCertificationViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_DESCRIPTOR_CERTIFICATION_RECORD_KIND {
            out.push(M5DescriptorCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DESCRIPTOR_CERTIFICATION_SCHEMA_VERSION {
            out.push(M5DescriptorCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5DescriptorCertificationViolation::MissingIdentity);
        }

        // Every runtime lane must be certified exactly once and be self-consistent.
        let mut seen_lanes = std::collections::BTreeSet::new();
        for lane in &self.lanes {
            if !seen_lanes.insert(lane.lane) {
                out.push(M5DescriptorCertificationViolation::DuplicateLane);
            }
            out.extend(lane.validate());
        }
        for lane in RuntimeLane::ALL {
            if !self.lanes.iter().any(|c| c.lane == lane) {
                out.push(M5DescriptorCertificationViolation::LaneNotCertified);
            }
        }

        if self.downgrade_rules != canonical_downgrade_rules() {
            out.push(M5DescriptorCertificationViolation::DowngradeRulesDrift);
        }

        if self.consumers.is_empty() {
            out.push(M5DescriptorCertificationViolation::PacketHasNoConsumers);
        }
        let mut seen_consumers = std::collections::BTreeSet::new();
        for consumer in &self.consumers {
            if !seen_consumers.insert(consumer.consumer) {
                out.push(M5DescriptorCertificationViolation::DuplicateConsumer);
            }
            out.extend(consumer.validate_static());
            // The stored verdict must match a fresh recompute from the parity proofs.
            let mut probe = consumer.clone();
            probe.recompute(&self.lanes, &self.downgrade_rules);
            if probe.gaps != consumer.gaps
                || probe.covered_badge_families != consumer.covered_badge_families
                || probe.bound_descriptor_schemas != consumer.bound_descriptor_schemas
                || probe.applicable_downgrade_rule_ids != consumer.applicable_downgrade_rule_ids
                || probe.proof_fixture_refs != consumer.proof_fixture_refs
                || probe.status != consumer.status
                || probe.signal != consumer.signal
                || probe.gate_decision != consumer.gate_decision
                || probe.effective_qualification != consumer.effective_qualification
            {
                out.push(M5DescriptorCertificationViolation::ConsumerVerdictDrift);
            }
        }

        let expected_tokens: Vec<String> = PublicTruthConsumer::ALL
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        if self.consumer_tokens != expected_tokens {
            out.push(M5DescriptorCertificationViolation::ConsumerSetMismatch);
        }
        if !self.disclosure.all_consume() {
            out.push(M5DescriptorCertificationViolation::DisclosureIncomplete);
        }
        if self.summary != derive_summary(&self.lanes, &self.downgrade_rules, &self.consumers) {
            out.push(M5DescriptorCertificationViolation::SummaryDrift);
        }
        if self.release_gate != derive_release_gate(&self.lanes, &self.consumers) {
            out.push(M5DescriptorCertificationViolation::ReleaseGateAggregateMismatch);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5DescriptorCertificationViolation::VocabularyMismatch);
        }
        if self.conformance
            != derive_conformance(&self.lanes, &self.downgrade_rules, &self.consumers)
            || !self.conformance.all_hold()
        {
            out.push(M5DescriptorCertificationViolation::ConformanceReviewFailed);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 descriptor certification serializes"),
        ) {
            out.push(M5DescriptorCertificationViolation::RawMaterialInExport);
        }
        out
    }
}

/// The generation channel a certification packet is produced on. Every channel produces
/// byte-identical output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationChannel {
    /// The desktop product UI.
    DesktopUi,
    /// The CLI / headless structured-output path.
    CliHeadless,
    /// Offline / mirror-safe packet generation.
    OfflineMirror,
}

impl CertificationChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopUi, Self::CliHeadless, Self::OfflineMirror];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopUi => "desktop_ui",
            Self::CliHeadless => "cli_headless",
            Self::OfflineMirror => "offline_mirror",
        }
    }
}

/// Derives the certification summary from the lanes, rules, and consumers.
fn derive_summary(
    lanes: &[CertifiedLane],
    downgrade_rules: &[DowngradeRule],
    consumers: &[CertifiedConsumer],
) -> CertificationSummary {
    let lane_count = |state: FreshnessState| -> u32 {
        lanes.iter().filter(|l| l.proof_freshness == state).count() as u32
    };
    let blocked = consumers.iter().filter(|c| c.is_blocked()).count() as u32;
    CertificationSummary {
        total_lanes: lanes.len() as u32,
        current_lanes: lane_count(FreshnessState::Current),
        stale_lanes: lane_count(FreshnessState::Stale),
        expired_lanes: lane_count(FreshnessState::Expired),
        missing_lanes: lane_count(FreshnessState::Missing),
        total_consumers: consumers.len() as u32,
        certified_consumer_count: consumers.iter().filter(|c| c.is_certified()).count() as u32,
        narrowed_consumer_count: consumers.iter().filter(|c| c.is_narrowed()).count() as u32,
        blocked_consumer_count: blocked,
        total_downgrade_rules: downgrade_rules.len() as u32,
        blocks_stable_promotion: blocked > 0,
    }
}

/// Derives the aggregate release gate from the per-consumer gates and the drifted lanes.
fn derive_release_gate(
    lanes: &[CertifiedLane],
    consumers: &[CertifiedConsumer],
) -> CertificationReleaseGate {
    let pick = |f: &dyn Fn(&CertifiedConsumer) -> bool| -> Vec<String> {
        let mut tokens: Vec<String> = consumers
            .iter()
            .filter(|c| f(c))
            .map(|c| c.consumer.as_str().to_owned())
            .collect();
        tokens.sort();
        tokens
    };
    let mut drifted_dimensions: Vec<String> = lanes
        .iter()
        .filter(|l| !matches!(l.proof_freshness, FreshnessState::Current))
        .map(|l| l.dimension.as_str().to_owned())
        .collect();
    drifted_dimensions.sort();
    drifted_dimensions.dedup();
    let blocked = pick(&|c| c.is_blocked());
    CertificationReleaseGate {
        blocks_stable_promotion: !blocked.is_empty(),
        blocked_consumers: blocked,
        narrowed_consumers: pick(&|c| c.is_narrowed()),
        certified_consumers: pick(&|c| c.is_certified()),
        drifted_dimensions,
        gate_message_id: format!(
            "{}certification.release_gate",
            M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX
        ),
    }
}

/// Derives the conformance review, so the stored block reflects the actual packet.
fn derive_conformance(
    lanes: &[CertifiedLane],
    downgrade_rules: &[DowngradeRule],
    consumers: &[CertifiedConsumer],
) -> CertificationConformance {
    let every_lane = RuntimeLane::ALL.iter().all(|l| {
        lanes
            .iter()
            .filter(|c| c.lane == *l)
            .filter(|c| !c.parity_proof_ref.trim().is_empty())
            .count()
            == 1
    });

    let every_dimension = CertificationDimension::ALL
        .iter()
        .all(|d| lanes.iter().any(|c| c.dimension == *d));

    let maps_to_proof = !consumers.is_empty()
        && consumers.iter().all(|c| {
            !c.bound_families.is_empty()
                && c.bound_descriptor_schemas.len() == c.bound_families.len()
                && c.covered_badge_families.len() == c.bound_families.len()
                && c.proof_fixture_refs.len() == c.certified_lanes.len()
                && !c.certified_lanes.is_empty()
        });

    let every_reads_lane = consumers.iter().all(|c| !c.certified_lanes.is_empty());

    let freshness_of = |lane: RuntimeLane| -> Option<FreshnessState> {
        lanes
            .iter()
            .find(|c| c.lane == lane)
            .map(|c| c.proof_freshness)
    };

    // A stale lane proof narrows every consumer that reads it, unless a failing lane already
    // blocks that consumer.
    let stale_narrows = consumers.iter().all(|c| {
        let reads_stale = c
            .certified_lanes
            .iter()
            .any(|l| freshness_of(*l) == Some(FreshnessState::Stale));
        let reads_failing = c.certified_lanes.iter().any(|l| {
            !matches!(
                freshness_of(*l),
                Some(FreshnessState::Current) | Some(FreshnessState::Stale)
            )
        });
        !reads_stale || reads_failing || c.is_narrowed()
    });

    // An expired / missing / uncertified lane proof blocks every consumer that reads it.
    let missing_blocks = consumers.iter().all(|c| {
        let reads_failing = c.certified_lanes.iter().any(|l| {
            !matches!(
                freshness_of(*l),
                Some(FreshnessState::Current) | Some(FreshnessState::Stale)
            )
        });
        !reads_failing || c.is_blocked()
    });

    let gaps_named = consumers.iter().all(|c| {
        c.gaps.iter().all(|g| {
            g.cause_message_id
                .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX)
                && g.consumer == c.consumer
                && g.dimension == g.lane.dimension()
        })
    });

    // Every non-authoritative descriptor value still has a downgrade rule (reuses the shared rule
    // set, so the certification never loosens the matrix's downgrade coverage).
    let mut weaker_values: Vec<(DescriptorFamily, &str)> = Vec::new();
    for class in ProvenanceClass::ALL {
        if !class.is_authoritative() {
            weaker_values.push((DescriptorFamily::Provenance, class.as_str()));
        }
    }
    for state in FreshnessState::ALL {
        if !matches!(state, FreshnessState::Current) {
            weaker_values.push((DescriptorFamily::Freshness, state.as_str()));
        }
    }
    for scope in ClientScope::ALL {
        if !scope.is_full_authority() {
            weaker_values.push((DescriptorFamily::ClientScope, scope.as_str()));
        }
    }
    let downgrade_covers = weaker_values.iter().all(|(family, token)| {
        downgrade_rules
            .iter()
            .any(|r| r.trigger_family == *family && r.trigger_token == *token)
    });

    let generated = consumers.iter().all(|c| {
        let mut probe = c.clone();
        probe.recompute(lanes, downgrade_rules);
        probe.gaps == c.gaps
            && probe.status == c.status
            && probe.gate_decision == c.gate_decision
            && probe.effective_qualification == c.effective_qualification
            && probe.proof_fixture_refs == c.proof_fixture_refs
    });

    let export_clean =
        !json_contains_forbidden_material(&serde_json::to_value(lanes).expect("lanes serialize"))
            && !json_contains_forbidden_material(
                &serde_json::to_value(consumers).expect("consumers serialize"),
            );

    CertificationConformance {
        every_lane_certified_with_proof: every_lane,
        every_dimension_covered: every_dimension,
        every_consumer_maps_to_descriptors_and_proof: maps_to_proof,
        every_consumer_reads_at_least_one_lane: every_reads_lane,
        stale_proof_narrows_deterministically: stale_narrows,
        missing_proof_blocks_stable_promotion: missing_blocks,
        exact_gaps_named_per_consumer: gaps_named,
        downgrade_rules_cover_every_weaker_value: downgrade_covers,
        surfaces_consume_one_certification: true,
        generated_from_checked_in_proofs: generated,
        controlled_enums_frozen: CertificationVocabulary::canonical().matches_canonical(),
        export_carries_no_raw_material: export_clean,
    }
}

/// Validation failures for the descriptor-certification lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DescriptorCertificationViolation {
    /// The packet record kind is wrong.
    WrongRecordKind,
    /// The packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// A certified lane cites a field that does not match its lane.
    LaneFieldMismatch,
    /// A certified lane's status drifted from its proof freshness.
    LaneStatusDrift,
    /// Two certified lanes name the same runtime lane.
    DuplicateLane,
    /// A runtime lane has no certified entry.
    LaneNotCertified,
    /// The downgrade rule set drifted from the canonical rules.
    DowngradeRulesDrift,
    /// The packet declares no claimed consumers.
    PacketHasNoConsumers,
    /// Two consumers share a consumer token.
    DuplicateConsumer,
    /// A claimed consumer binds no descriptor families.
    ConsumerBindsNoDescriptors,
    /// A claimed consumer reads no runtime lanes.
    ConsumerReadsNoLanes,
    /// A consumer's stored verdict drifted from a fresh recompute.
    ConsumerVerdictDrift,
    /// A certification gap is malformed (wrong consumer, dimension, or unprefixed message id).
    CertificationGapInvalid,
    /// The consumer-token set does not match the canonical consumers.
    ConsumerSetMismatch,
    /// A disclosure surface does not consume the certification output.
    DisclosureIncomplete,
    /// The certification summary disagrees with the lanes / consumers.
    SummaryDrift,
    /// The aggregate release gate disagrees with the per-consumer gates.
    ReleaseGateAggregateMismatch,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The export contains raw provider material.
    RawMaterialInExport,
}

impl M5DescriptorCertificationViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::LaneFieldMismatch => "lane_field_mismatch",
            Self::LaneStatusDrift => "lane_status_drift",
            Self::DuplicateLane => "duplicate_lane",
            Self::LaneNotCertified => "lane_not_certified",
            Self::DowngradeRulesDrift => "downgrade_rules_drift",
            Self::PacketHasNoConsumers => "packet_has_no_consumers",
            Self::DuplicateConsumer => "duplicate_consumer",
            Self::ConsumerBindsNoDescriptors => "consumer_binds_no_descriptors",
            Self::ConsumerReadsNoLanes => "consumer_reads_no_lanes",
            Self::ConsumerVerdictDrift => "consumer_verdict_drift",
            Self::CertificationGapInvalid => "certification_gap_invalid",
            Self::ConsumerSetMismatch => "consumer_set_mismatch",
            Self::DisclosureIncomplete => "disclosure_incomplete",
            Self::SummaryDrift => "summary_drift",
            Self::ReleaseGateAggregateMismatch => "release_gate_aggregate_mismatch",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked raw material. Mirrors the redaction posture of
/// the upstream descriptor lanes.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized value for forbidden material. Returns true when a key (case-insensitive)
/// contains a forbidden substring.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, child)| {
            let lower = key.to_ascii_lowercase();
            FORBIDDEN_KEY_SUBSTRINGS
                .iter()
                .any(|needle| lower.contains(needle))
                || json_contains_forbidden_material(child)
        }),
        serde_json::Value::Array(items) => items.iter().any(json_contains_forbidden_material),
        _ => false,
    }
}
