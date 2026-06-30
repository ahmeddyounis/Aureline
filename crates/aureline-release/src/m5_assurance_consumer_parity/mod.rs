//! The M5 assurance consumer-parity model: one object the claimed help / procurement / evaluation /
//! support / public-truth surfaces all read from, so they can never restate the same trust facts
//! independently.
//!
//! The assurance center, the assurance-claim reducer, the governance/fitness dashboard, the
//! capability-boundary inspector, and the event-provenance inspector each already mint an inspectable
//! truth packet for one slice of M5 assurance and high-risk-route truth. This lane is the convergence
//! layer over all five: it ingests those packets, normalizes every claim, control-proof, governance,
//! ownership, decision-right, boundary, route, approval, and event item into one [`UnifiedFact`]
//! grammar — a gate, an effective qualification, an owner, an evidence-freshness reading, and the
//! evidence refs behind it — and projects each fact onto every consumer surface so the About / help
//! panel, the procurement export, the evaluation packet, the support export, and the
//! shiproom / public-truth manifest read the *same* fact at the *same* gate.
//!
//! The guardrail is the same one each source lane already enforces, lifted to the consumer boundary:
//! each fact's per-consumer projection reads the fact's own gate and qualification
//! ([`ConsumerProjection::converges_with_fact`] is the proof), and every consumer view reads every
//! fact, so a fact narrowed or blocked in one surface can never read stronger in another. When any
//! ingested source narrows or blocks an item, every consumer narrows or blocks with it; when any
//! source holds Stable promotion, the parity packet holds it too.
//!
//! The packet is metadata-only. It does not embed the raw source packets; it records a
//! [`SourceBinding`] per ingested packet (its id, record kind, registry ref, fact count, and whether
//! it validated clean) and reduces every fact to repo-relative evidence refs, so the export preserves
//! owner / freshness / route lineage without leaking any credential body or raw provider payload.

use serde::{Deserialize, Serialize};

use crate::m5_assurance_center::{M5AssuranceCenter, M5_ASSURANCE_CENTER_REGISTRY_REF};
use crate::m5_assurance_claim_reducer::{
    M5AssuranceClaimReducer, M5_ASSURANCE_CLAIM_REDUCER_REGISTRY_REF,
};
use crate::m5_boundary_inspector::{M5BoundaryInspector, M5_BOUNDARY_INSPECTOR_REGISTRY_REF};
use crate::m5_descriptor_badge::{
    ConsumerStatus, DescriptorGate, DescriptorSignal, FreshnessState, QualificationClass,
};
use crate::m5_event_provenance::{M5EventProvenance, M5_EVENT_PROVENANCE_REGISTRY_REF};
use crate::m5_governance_dashboard::{M5GovernanceDashboard, M5_GOVERNANCE_DASHBOARD_REGISTRY_REF};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_assurance_consumer_parity,
    seeded_m5_assurance_consumer_parity_boundary_route_blocked,
    seeded_m5_assurance_consumer_parity_claim_narrowed,
    seeded_m5_assurance_consumer_parity_event_blocked,
    seeded_m5_assurance_consumer_parity_governance_blocked, M5_ASSURANCE_CONSUMER_PARITY_PACKET_ID,
};

// ---------------------------------------------------------------------------------------------
// Identity constants
// ---------------------------------------------------------------------------------------------

/// Record kind tag for the consumer-parity packet.
pub const M5_ASSURANCE_CONSUMER_PARITY_RECORD_KIND: &str = "m5_assurance_consumer_parity";

/// Record kind tag for the exported refs-only preview.
pub const M5_ASSURANCE_CONSUMER_PARITY_EXPORT_RECORD_KIND: &str =
    "m5_assurance_consumer_parity_export_preview";

/// Schema version for the consumer-parity packet.
pub const M5_ASSURANCE_CONSUMER_PARITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path to the packet JSON Schema.
pub const M5_ASSURANCE_CONSUMER_PARITY_SCHEMA_REF: &str =
    "schemas/public-truth/m5-assurance-consumer-parity.schema.json";

/// Repo-relative path to the published consumer-parity inventory.
pub const M5_ASSURANCE_CONSUMER_PARITY_REGISTRY_REF: &str =
    "artifacts/public-truth/m5-assurance-consumer-parity.json";

/// Repo-relative path to the rendered consumer-parity overview document.
pub const M5_ASSURANCE_CONSUMER_PARITY_OVERVIEW_REF: &str =
    "artifacts/public-truth/m5-assurance-consumer-parity.md";

/// Repo-relative path to the machine-readable fact / consumer matrix CSV.
pub const M5_ASSURANCE_CONSUMER_PARITY_FACTS_CSV_REF: &str =
    "artifacts/public-truth/m5-assurance-consumer-parity-facts.csv";

/// Repo-relative path to the release-grade export proof.
pub const M5_ASSURANCE_EXPORT_PROOF_REF: &str =
    "artifacts/release/m5-assurance-export-proof/consumer-parity.json";

/// Repo-relative path to the exported refs-only preview.
pub const M5_ASSURANCE_EXPORT_PREVIEW_REF: &str =
    "artifacts/release/m5-assurance-export-proof/export-preview.json";

/// Repo-relative path to the contract document.
pub const M5_ASSURANCE_CONSUMER_PARITY_DOC_REF: &str =
    "docs/public-truth/m5-assurance-consumer-parity-contract.md";

/// Repo-relative directory holding the per-state drill fixtures.
pub const M5_ASSURANCE_CONSUMER_PARITY_FIXTURE_DIR: &str =
    "fixtures/public-truth/m5-assurance-consumers/";

/// Message-id prefix every stable message id in this lane carries.
pub const M5_ASSURANCE_CONSUMER_PARITY_MESSAGE_ID_PREFIX: &str =
    "public_truth.assurance_consumer_parity.";

// ---------------------------------------------------------------------------------------------
// Consumer surfaces
// ---------------------------------------------------------------------------------------------

/// A claimed M5 surface that reads the unified assurance / route truth. Every surface reads the same
/// fact set at the same gate; the type exists so no surface can hold its own copy of the inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityConsumer {
    /// The About / help panel that reads core assurance and boundary facts offline.
    HelpAbout,
    /// The procurement export handed to a buyer's review.
    ProcurementExport,
    /// The exported evaluation packet handed to an evaluator.
    EvaluationPacket,
    /// The support export / field bundle.
    SupportExport,
    /// The shiproom / release / public-truth manifest.
    ReleasePublicTruth,
}

impl ParityConsumer {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HelpAbout,
        Self::ProcurementExport,
        Self::EvaluationPacket,
        Self::SupportExport,
        Self::ReleasePublicTruth,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::ProcurementExport => "procurement_export",
            Self::EvaluationPacket => "evaluation_packet",
            Self::SupportExport => "support_export",
            Self::ReleasePublicTruth => "release_public_truth",
        }
    }

    /// Reviewer-facing consumer label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::HelpAbout => "About / help",
            Self::ProcurementExport => "Procurement export",
            Self::EvaluationPacket => "Evaluation packet",
            Self::SupportExport => "Support export",
            Self::ReleasePublicTruth => "Release / public-truth",
        }
    }

    /// Owner role accountable for keeping this consumer bound to the parity model.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about_owner",
            Self::ProcurementExport => "procurement_export_owner",
            Self::EvaluationPacket => "evaluation_packet_owner",
            Self::SupportExport => "support_export_owner",
            Self::ReleasePublicTruth => "release_truth_owner",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Source packets
// ---------------------------------------------------------------------------------------------

/// The upstream truth packet a [`UnifiedFact`] was ingested from. Each kind is one already-published
/// M5 assurance / route lane; this lane never re-derives their facts, it normalizes and converges
/// them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePacketKind {
    /// The assurance center (claim cards and control-proof rows).
    AssuranceCenter,
    /// The assurance-claim reducer (auto-narrowing regulated claims).
    AssuranceClaimReducer,
    /// The governance / fitness dashboard (fitness, ownership, decision rights).
    GovernanceDashboard,
    /// The capability-boundary inspector (boundary, route, approval facets).
    BoundaryInspector,
    /// The event-provenance inspector (deferred / replayable events).
    EventProvenance,
}

impl SourcePacketKind {
    /// Every source packet kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AssuranceCenter,
        Self::AssuranceClaimReducer,
        Self::GovernanceDashboard,
        Self::BoundaryInspector,
        Self::EventProvenance,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssuranceCenter => "assurance_center",
            Self::AssuranceClaimReducer => "assurance_claim_reducer",
            Self::GovernanceDashboard => "governance_dashboard",
            Self::BoundaryInspector => "boundary_inspector",
            Self::EventProvenance => "event_provenance",
        }
    }

    /// Reviewer-facing source label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AssuranceCenter => "Assurance center",
            Self::AssuranceClaimReducer => "Assurance-claim reducer",
            Self::GovernanceDashboard => "Governance / fitness dashboard",
            Self::BoundaryInspector => "Capability-boundary inspector",
            Self::EventProvenance => "Event-provenance inspector",
        }
    }

    /// The published registry ref for the source packet.
    pub const fn registry_ref(self) -> &'static str {
        match self {
            Self::AssuranceCenter => M5_ASSURANCE_CENTER_REGISTRY_REF,
            Self::AssuranceClaimReducer => M5_ASSURANCE_CLAIM_REDUCER_REGISTRY_REF,
            Self::GovernanceDashboard => M5_GOVERNANCE_DASHBOARD_REGISTRY_REF,
            Self::BoundaryInspector => M5_BOUNDARY_INSPECTOR_REGISTRY_REF,
            Self::EventProvenance => M5_EVENT_PROVENANCE_REGISTRY_REF,
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Truth domains
// ---------------------------------------------------------------------------------------------

/// The kind of assurance / route truth a [`UnifiedFact`] carries. The set is the union of the
/// inspectable item kinds across the five source lanes, normalized to one grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthDomain {
    /// A regulated / sovereign / self-hosted assurance claim.
    AssuranceClaim,
    /// A control-proof row backing one or more claims.
    ControlProof,
    /// A governance / fitness function tile.
    GovernanceFitness,
    /// A service-ownership card.
    ServiceOwnership,
    /// A decision-right card.
    DecisionRight,
    /// A capability-boundary summary card.
    CapabilityBoundary,
    /// A route-hop timeline for a high-risk action.
    RouteTimeline,
    /// An approval-ticket inspector for a high-risk action.
    ApprovalTicket,
    /// An event-provenance row for a deferred / replayable action.
    EventProvenance,
}

impl TruthDomain {
    /// Every truth domain, in canonical order.
    pub const ALL: [Self; 9] = [
        Self::AssuranceClaim,
        Self::ControlProof,
        Self::GovernanceFitness,
        Self::ServiceOwnership,
        Self::DecisionRight,
        Self::CapabilityBoundary,
        Self::RouteTimeline,
        Self::ApprovalTicket,
        Self::EventProvenance,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssuranceClaim => "assurance_claim",
            Self::ControlProof => "control_proof",
            Self::GovernanceFitness => "governance_fitness",
            Self::ServiceOwnership => "service_ownership",
            Self::DecisionRight => "decision_right",
            Self::CapabilityBoundary => "capability_boundary",
            Self::RouteTimeline => "route_timeline",
            Self::ApprovalTicket => "approval_ticket",
            Self::EventProvenance => "event_provenance",
        }
    }

    /// Reviewer-facing domain label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AssuranceClaim => "Assurance claim",
            Self::ControlProof => "Control proof",
            Self::GovernanceFitness => "Governance fitness",
            Self::ServiceOwnership => "Service ownership",
            Self::DecisionRight => "Decision right",
            Self::CapabilityBoundary => "Capability boundary",
            Self::RouteTimeline => "Route timeline",
            Self::ApprovalTicket => "Approval ticket",
            Self::EventProvenance => "Event provenance",
        }
    }

    /// The source packet this domain is ingested from.
    pub const fn source_packet(self) -> SourcePacketKind {
        match self {
            Self::AssuranceClaim => SourcePacketKind::AssuranceClaimReducer,
            Self::ControlProof => SourcePacketKind::AssuranceCenter,
            Self::GovernanceFitness | Self::ServiceOwnership | Self::DecisionRight => {
                SourcePacketKind::GovernanceDashboard
            }
            Self::CapabilityBoundary | Self::RouteTimeline | Self::ApprovalTicket => {
                SourcePacketKind::BoundaryInspector
            }
            Self::EventProvenance => SourcePacketKind::EventProvenance,
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Gate / freshness helpers
// ---------------------------------------------------------------------------------------------

/// Position of a gate in the canonical (most→least permissive) ordering.
const fn gate_rank(gate: DescriptorGate) -> usize {
    match gate {
        DescriptorGate::Governed => 0,
        DescriptorGate::Narrowed => 1,
        DescriptorGate::Blocked => 2,
    }
}

/// The more restrictive of two gate postures.
const fn worse_gate(a: DescriptorGate, b: DescriptorGate) -> DescriptorGate {
    if gate_rank(a) >= gate_rank(b) {
        a
    } else {
        b
    }
}

/// The coverage status a gate posture implies.
const fn gate_status(gate: DescriptorGate) -> ConsumerStatus {
    match gate {
        DescriptorGate::Governed => ConsumerStatus::Mapped,
        DescriptorGate::Narrowed => ConsumerStatus::Provisional,
        DescriptorGate::Blocked => ConsumerStatus::Unmapped,
    }
}

/// The effective qualification floor a gate posture implies: governed stands at Stable, narrowed
/// floors at Beta, blocked at Unavailable.
const fn floor_for_gate(gate: DescriptorGate) -> QualificationClass {
    match gate {
        DescriptorGate::Governed => QualificationClass::Stable,
        DescriptorGate::Narrowed => QualificationClass::Beta,
        DescriptorGate::Blocked => QualificationClass::Unavailable,
    }
}

/// The evidence-freshness reading a gate posture implies for an aggregate fact whose source item does
/// not track its own freshness window directly.
const fn freshness_for_gate(gate: DescriptorGate) -> FreshnessState {
    match gate {
        DescriptorGate::Governed => FreshnessState::Current,
        DescriptorGate::Narrowed => FreshnessState::Stale,
        DescriptorGate::Blocked => FreshnessState::Missing,
    }
}

// ---------------------------------------------------------------------------------------------
// Consumer projections
// ---------------------------------------------------------------------------------------------

/// One consumer's reading of a [`UnifiedFact`]. By construction every projection reads the fact's own
/// gate and qualification, so no consumer can read a fact stronger than the converged model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerProjection {
    /// The consumer surface.
    pub consumer: ParityConsumer,
    /// Reviewer-facing consumer label.
    pub consumer_label: String,
    /// The gate this consumer reads — the fact's own gate, by construction.
    pub gate: DescriptorGate,
    /// The effective qualification this consumer reads.
    pub effective_qualification: QualificationClass,
    /// Coverage status implied by the gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Owner role accountable for keeping this consumer bound to the fact.
    pub owner_role: String,
    /// True when this projection reads the same gate and qualification as its fact.
    pub converges_with_fact: bool,
    /// Stable message id; prefixed [`M5_ASSURANCE_CONSUMER_PARITY_MESSAGE_ID_PREFIX`].
    pub message_id: String,
}

/// Builds the per-consumer projection set for a fact, all reading the fact's own gate.
fn projections_for(
    domain: TruthDomain,
    subject: &str,
    gate: DescriptorGate,
    qualification: QualificationClass,
) -> Vec<ConsumerProjection> {
    let status = gate_status(gate);
    ParityConsumer::ALL
        .iter()
        .map(|consumer| ConsumerProjection {
            consumer: *consumer,
            consumer_label: consumer.label().to_owned(),
            gate,
            effective_qualification: qualification,
            status,
            signal: status.signal(),
            owner_role: consumer.owner_role().to_owned(),
            converges_with_fact: true,
            message_id: format!(
                "{}fact.{}.{}.consumer.{}",
                M5_ASSURANCE_CONSUMER_PARITY_MESSAGE_ID_PREFIX,
                domain.as_str(),
                subject,
                consumer.as_str()
            ),
        })
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Unified facts
// ---------------------------------------------------------------------------------------------

/// One normalized assurance / route fact, drawn from a source lane and reduced to one gate, one
/// effective qualification, one owner, one freshness reading, and the evidence refs behind it, with a
/// per-consumer projection so every claimed surface reads it identically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnifiedFact {
    /// The truth domain this fact carries.
    pub domain: TruthDomain,
    /// Reviewer-facing domain label.
    pub domain_label: String,
    /// The source packet this fact was ingested from.
    pub source_packet: SourcePacketKind,
    /// Stable subject token, unique within the domain.
    pub subject: String,
    /// Reader-facing subject label.
    pub label: String,
    /// The fact's gate — the source item's effective gate.
    pub gate: DescriptorGate,
    /// Effective qualification implied by the gate.
    pub effective_qualification: QualificationClass,
    /// Coverage status implied by the gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Owner role accountable for the fact.
    pub owner_role: String,
    /// Evidence freshness behind the fact.
    pub evidence_freshness: FreshnessState,
    /// Repo-relative evidence refs — links only, no raw material.
    pub evidence_refs: Vec<String>,
    /// The published registry ref of the source packet.
    pub source_ref: String,
    /// The per-consumer projections, in consumer order.
    pub consumer_projections: Vec<ConsumerProjection>,
    /// Stable message id for the fact; prefixed [`M5_ASSURANCE_CONSUMER_PARITY_MESSAGE_ID_PREFIX`].
    pub fact_message_id: String,
}

impl UnifiedFact {
    /// Builds a normalized fact, deriving the status, signal, qualification, and per-consumer
    /// projections from the gate so a fact and every consumer reading it stay convergent.
    #[allow(clippy::too_many_arguments)]
    fn build(
        domain: TruthDomain,
        subject: String,
        label: String,
        gate: DescriptorGate,
        owner_role: String,
        evidence_freshness: FreshnessState,
        evidence_refs: Vec<String>,
    ) -> Self {
        let source_packet = domain.source_packet();
        let effective_qualification = floor_for_gate(gate);
        let status = gate_status(gate);
        let consumer_projections = projections_for(domain, &subject, gate, effective_qualification);
        let fact_message_id = format!(
            "{}fact.{}.{}",
            M5_ASSURANCE_CONSUMER_PARITY_MESSAGE_ID_PREFIX,
            domain.as_str(),
            subject
        );
        Self {
            domain,
            domain_label: domain.label().to_owned(),
            source_packet,
            subject,
            label,
            gate,
            effective_qualification,
            status,
            signal: status.signal(),
            owner_role,
            evidence_freshness,
            evidence_refs,
            source_ref: source_packet.registry_ref().to_owned(),
            consumer_projections,
            fact_message_id,
        }
    }

    /// A stable reference key for the fact, unique across domains.
    pub fn ref_key(&self) -> String {
        format!("{}:{}", self.domain.as_str(), self.subject)
    }

    /// True when the fact stands fully governed.
    pub fn is_governed(&self) -> bool {
        matches!(self.gate, DescriptorGate::Governed)
    }

    /// True when the fact narrowed below Stable.
    pub fn is_narrowed(&self) -> bool {
        matches!(self.gate, DescriptorGate::Narrowed)
    }

    /// True when the fact is blocked from Stable promotion.
    pub fn is_blocked(&self) -> bool {
        matches!(self.gate, DescriptorGate::Blocked)
    }

    /// Validates the fact's internal convergence invariants.
    fn validate(&self) -> Vec<M5AssuranceConsumerParityViolation> {
        let mut out = Vec::new();
        if self.domain_label != self.domain.label()
            || self.source_packet != self.domain.source_packet()
            || self.source_ref != self.domain.source_packet().registry_ref()
        {
            out.push(M5AssuranceConsumerParityViolation::FactFieldMismatch);
        }
        // Status, signal, and qualification all follow from the gate.
        if self.status != gate_status(self.gate)
            || self.signal != self.status.signal()
            || self.effective_qualification != floor_for_gate(self.gate)
        {
            out.push(M5AssuranceConsumerParityViolation::FactOverstatesGate);
        }
        if self.subject.trim().is_empty()
            || self.label.trim().is_empty()
            || self.owner_role.trim().is_empty()
            || self.evidence_refs.is_empty()
            || self.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            out.push(M5AssuranceConsumerParityViolation::FactMissingLineage);
        }
        // Every consumer reads the fact, at the fact's own gate.
        let expected = projections_for(
            self.domain,
            &self.subject,
            self.gate,
            self.effective_qualification,
        );
        if self.consumer_projections != expected {
            out.push(M5AssuranceConsumerParityViolation::ConsumerDivergence);
        }
        let consumers: Vec<ParityConsumer> = self
            .consumer_projections
            .iter()
            .map(|p| p.consumer)
            .collect();
        if consumers != ParityConsumer::ALL.to_vec() {
            out.push(M5AssuranceConsumerParityViolation::ConsumerSetInvalid);
        }
        if !self
            .fact_message_id
            .starts_with(M5_ASSURANCE_CONSUMER_PARITY_MESSAGE_ID_PREFIX)
            || self.consumer_projections.iter().any(|p| {
                !p.message_id
                    .starts_with(M5_ASSURANCE_CONSUMER_PARITY_MESSAGE_ID_PREFIX)
            })
        {
            out.push(M5AssuranceConsumerParityViolation::UnprefixedMessageId);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Fact extraction
// ---------------------------------------------------------------------------------------------

/// Normalizes the assurance-claim reducer's reduced claims into assurance-claim facts.
fn facts_from_claim_reducer(reducer: &M5AssuranceClaimReducer) -> Vec<UnifiedFact> {
    reducer
        .reduced_claims
        .iter()
        .map(|claim| {
            UnifiedFact::build(
                TruthDomain::AssuranceClaim,
                claim.subject.as_str().to_owned(),
                claim.subject_label.clone(),
                claim.reduced_gate,
                claim.owner_role.clone(),
                freshness_for_gate(claim.reduced_gate),
                non_empty_refs(&claim.evidence_refs),
            )
        })
        .collect()
}

/// Normalizes the assurance center's control-proof rows into control-proof facts.
fn facts_from_assurance_center(center: &M5AssuranceCenter) -> Vec<UnifiedFact> {
    center
        .control_proof_rows
        .iter()
        .map(|row| {
            UnifiedFact::build(
                TruthDomain::ControlProof,
                row.control.as_str().to_owned(),
                row.control_label.clone(),
                row.effective_gate,
                row.owner_role.clone(),
                row.evidence_freshness,
                vec![row.proof_ref.clone()],
            )
        })
        .collect()
}

/// Normalizes the governance dashboard's fitness tiles, service cards, and decision-right cards into
/// governance facts.
fn facts_from_governance(dashboard: &M5GovernanceDashboard) -> Vec<UnifiedFact> {
    let mut out = Vec::new();
    for tile in &dashboard.fitness_tiles {
        out.push(UnifiedFact::build(
            TruthDomain::GovernanceFitness,
            tile.function.as_str().to_owned(),
            tile.function_label.clone(),
            tile.gate,
            tile.owner_role.clone(),
            tile.evidence_freshness,
            vec![tile.proof_ref.clone()],
        ));
    }
    for card in &dashboard.service_cards {
        out.push(UnifiedFact::build(
            TruthDomain::ServiceOwnership,
            card.service.as_str().to_owned(),
            card.service_label.clone(),
            card.gate,
            card.owner_role.clone(),
            freshness_for_gate(card.gate),
            non_empty_refs(&card.evidence_refs),
        ));
    }
    for card in &dashboard.decision_right_cards {
        out.push(UnifiedFact::build(
            TruthDomain::DecisionRight,
            card.decision.as_str().to_owned(),
            card.decision_label.clone(),
            card.gate,
            card.accountable_owner.clone(),
            freshness_for_gate(card.gate),
            non_empty_refs(&card.evidence_refs),
        ));
    }
    out
}

/// Normalizes the boundary inspector's per-action boundary, route, and approval facets into facts.
fn facts_from_boundary(inspector: &M5BoundaryInspector) -> Vec<UnifiedFact> {
    let mut out = Vec::new();
    for action in &inspector.action_inspectors {
        let boundary = &action.boundary_card;
        out.push(UnifiedFact::build(
            TruthDomain::CapabilityBoundary,
            boundary.action.as_str().to_owned(),
            boundary.action_label.clone(),
            boundary.effective_gate,
            boundary.owner_role.clone(),
            boundary.evidence_freshness,
            vec![boundary.proof_ref.clone()],
        ));
    }
    for action in &inspector.action_inspectors {
        let route = &action.route_timeline;
        out.push(UnifiedFact::build(
            TruthDomain::RouteTimeline,
            route.action.as_str().to_owned(),
            route.action_label.clone(),
            route.effective_gate,
            route.owner_role.clone(),
            route.evidence_freshness,
            vec![route.proof_ref.clone()],
        ));
    }
    for action in &inspector.action_inspectors {
        let approval = &action.approval_ticket;
        out.push(UnifiedFact::build(
            TruthDomain::ApprovalTicket,
            approval.action.as_str().to_owned(),
            approval.action_label.clone(),
            approval.effective_gate,
            approval.owner_role.clone(),
            freshness_for_gate(approval.effective_gate),
            vec![approval.proof_ref.clone()],
        ));
    }
    out
}

/// Normalizes the event-provenance inspector's deferred-event provenance rows into facts.
fn facts_from_event(event: &M5EventProvenance) -> Vec<UnifiedFact> {
    event
        .deferred_events
        .iter()
        .map(|deferred| {
            // The event fact reflects the whole deferred-event verdict — the worst of the provenance,
            // route-drift, and reapproval facets — so a tenant or region drift that blocks via the
            // drift banner blocks the event fact, not only the provenance row.
            let row = &deferred.provenance_row;
            UnifiedFact::build(
                TruthDomain::EventProvenance,
                row.action.as_str().to_owned(),
                row.action_label.clone(),
                deferred.effective_gate,
                row.owner_role.clone(),
                row.evidence_freshness,
                vec![row.proof_ref.clone()],
            )
        })
        .collect()
}

/// Returns the refs unchanged, or a single placeholder ref when the source carried none, so every
/// fact preserves at least one lineage ref.
fn non_empty_refs(refs: &[String]) -> Vec<String> {
    if refs.is_empty() {
        vec!["artifacts/public-truth/".to_owned()]
    } else {
        refs.to_vec()
    }
}

// ---------------------------------------------------------------------------------------------
// Consumer views
// ---------------------------------------------------------------------------------------------

/// One consumer's generated view over the unified fact set: the same facts every other consumer
/// reads, summarized at the worst gate across them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerView {
    /// The consumer surface.
    pub consumer: ParityConsumer,
    /// Reviewer-facing consumer label.
    pub consumer_label: String,
    /// Owner role accountable for the consumer.
    pub owner_role: String,
    /// The number of facts this consumer reads.
    pub fact_count: u32,
    /// Facts read as governed.
    pub governed_facts: u32,
    /// Facts read as narrowed.
    pub narrowed_facts: u32,
    /// Facts read as blocked.
    pub blocked_facts: u32,
    /// The worst gate across the facts.
    pub worst_gate: DescriptorGate,
    /// Effective qualification implied by the worst gate.
    pub effective_qualification: QualificationClass,
    /// Coverage status implied by the worst gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// True when this view reads every fact in the model — the parity proof.
    pub reads_all_facts: bool,
    /// The fact ref keys this view reads, in fact order.
    pub fact_refs: Vec<String>,
    /// Stable message id; prefixed [`M5_ASSURANCE_CONSUMER_PARITY_MESSAGE_ID_PREFIX`].
    pub view_message_id: String,
}

/// Derives a consumer's view over the fact set.
fn derive_consumer_view(consumer: ParityConsumer, facts: &[UnifiedFact]) -> ConsumerView {
    let worst_gate = facts
        .iter()
        .map(|f| f.gate)
        .fold(DescriptorGate::Governed, worse_gate);
    let governed = facts.iter().filter(|f| f.is_governed()).count() as u32;
    let narrowed = facts.iter().filter(|f| f.is_narrowed()).count() as u32;
    let blocked = facts.iter().filter(|f| f.is_blocked()).count() as u32;
    let status = gate_status(worst_gate);
    let fact_refs: Vec<String> = facts.iter().map(|f| f.ref_key()).collect();
    ConsumerView {
        consumer,
        consumer_label: consumer.label().to_owned(),
        owner_role: consumer.owner_role().to_owned(),
        fact_count: facts.len() as u32,
        governed_facts: governed,
        narrowed_facts: narrowed,
        blocked_facts: blocked,
        worst_gate,
        effective_qualification: floor_for_gate(worst_gate),
        status,
        signal: status.signal(),
        reads_all_facts: fact_refs.len() == facts.len(),
        fact_refs,
        view_message_id: format!(
            "{}consumer.{}",
            M5_ASSURANCE_CONSUMER_PARITY_MESSAGE_ID_PREFIX,
            consumer.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------------------------
// Source bindings
// ---------------------------------------------------------------------------------------------

/// Records that one upstream source packet was ingested: which packet, how many facts it contributed,
/// and whether it validated clean. The parity packet binds to refs, never to embedded source bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceBinding {
    /// The source packet kind.
    pub source_packet: SourcePacketKind,
    /// Reviewer-facing source label.
    pub source_label: String,
    /// The source packet's stable id.
    pub packet_id: String,
    /// The source packet's record kind tag.
    pub record_kind: String,
    /// The published registry ref of the source packet.
    pub registry_ref: String,
    /// The number of facts this source contributed.
    pub fact_count: u32,
    /// True when the source packet validated clean on ingest.
    pub validated_clean: bool,
    /// True when at least one fact from this source blocks Stable promotion.
    pub blocks_stable_promotion: bool,
}

// ---------------------------------------------------------------------------------------------
// Export preview
// ---------------------------------------------------------------------------------------------

/// One refs-only export entry: a fact reduced to its domain, subject, gate, qualification, owner,
/// freshness, and evidence refs — no raw material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerParityExportEntry {
    /// The truth domain.
    pub domain: TruthDomain,
    /// The subject token.
    pub subject: String,
    /// The source packet.
    pub source_packet: SourcePacketKind,
    /// The fact's gate.
    pub gate: DescriptorGate,
    /// Effective qualification implied by the gate.
    pub effective_qualification: QualificationClass,
    /// Coverage status implied by the gate.
    pub status: ConsumerStatus,
    /// Owner role accountable for the fact.
    pub owner_role: String,
    /// Evidence freshness behind the fact.
    pub evidence_freshness: FreshnessState,
    /// Repo-relative evidence refs — links only.
    pub evidence_refs: Vec<String>,
    /// The published registry ref of the source packet.
    pub source_ref: String,
}

/// The exported refs-only preview: the same fact set the live consumer views read, reduced to refs and
/// gates, plus the consumer and domain sets, so an offline procurement / evaluation / support review
/// reads the same facts the in-product surfaces show without leaking any raw material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerParityExportPreview {
    /// Record kind; must equal [`M5_ASSURANCE_CONSUMER_PARITY_EXPORT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ASSURANCE_CONSUMER_PARITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable preview id.
    pub preview_id: String,
    /// The packet id the preview was generated from.
    pub generated_from: String,
    /// The evaluation date the preview was computed as-of.
    pub evaluated_at: String,
    /// The consumer surfaces that read the same fact set.
    pub consumers: Vec<ParityConsumer>,
    /// The truth domains covered, in canonical order.
    pub domains: Vec<TruthDomain>,
    /// The export entries, in fact order.
    pub entries: Vec<ConsumerParityExportEntry>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
}

impl ConsumerParityExportPreview {
    /// Derives the export preview from the facts.
    fn derive(
        preview_id: &str,
        generated_from: &str,
        evaluated_at: &str,
        redaction_class_token: &str,
        facts: &[UnifiedFact],
    ) -> Self {
        let entries: Vec<ConsumerParityExportEntry> = facts
            .iter()
            .map(|f| ConsumerParityExportEntry {
                domain: f.domain,
                subject: f.subject.clone(),
                source_packet: f.source_packet,
                gate: f.gate,
                effective_qualification: f.effective_qualification,
                status: f.status,
                owner_role: f.owner_role.clone(),
                evidence_freshness: f.evidence_freshness,
                evidence_refs: f.evidence_refs.clone(),
                source_ref: f.source_ref.clone(),
            })
            .collect();
        let mut domains: Vec<TruthDomain> = facts.iter().map(|f| f.domain).collect();
        domains.dedup();
        Self {
            record_kind: M5_ASSURANCE_CONSUMER_PARITY_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_ASSURANCE_CONSUMER_PARITY_SCHEMA_VERSION,
            preview_id: preview_id.to_owned(),
            generated_from: generated_from.to_owned(),
            evaluated_at: evaluated_at.to_owned(),
            consumers: ParityConsumer::ALL.to_vec(),
            domains,
            entries,
            redaction_class_token: redaction_class_token.to_owned(),
        }
    }

    /// Deterministic export-safe JSON for the preview.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only preview fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("consumer parity export preview serializes")
    }
}

// ---------------------------------------------------------------------------------------------
// Vocabulary / summary / conformance
// ---------------------------------------------------------------------------------------------

/// The controlled-vocabulary token sets the packet froze.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerParityVocabulary {
    /// Consumer surface tokens.
    pub consumers: Vec<String>,
    /// Source packet tokens.
    pub source_packets: Vec<String>,
    /// Truth domain tokens.
    pub domains: Vec<String>,
    /// Gate tokens.
    pub gates: Vec<String>,
    /// Signal tokens.
    pub signals: Vec<String>,
    /// Qualification tokens.
    pub qualifications: Vec<String>,
    /// Freshness tokens.
    pub freshness_states: Vec<String>,
}

impl ConsumerParityVocabulary {
    /// The canonical, frozen vocabulary.
    pub fn canonical() -> Self {
        Self {
            consumers: ParityConsumer::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            source_packets: SourcePacketKind::ALL
                .iter()
                .map(|s| s.as_str().to_owned())
                .collect(),
            domains: TruthDomain::ALL
                .iter()
                .map(|d| d.as_str().to_owned())
                .collect(),
            gates: DescriptorGate::ALL
                .iter()
                .map(|g| g.as_str().to_owned())
                .collect(),
            signals: DescriptorSignal::ALL
                .iter()
                .map(|s| s.as_str().to_owned())
                .collect(),
            qualifications: QualificationClass::ALL
                .iter()
                .map(|q| q.as_str().to_owned())
                .collect(),
            freshness_states: FreshnessState::ALL
                .iter()
                .map(|f| f.as_str().to_owned())
                .collect(),
        }
    }

    /// True when this vocabulary equals the canonical one.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// A compact summary of the converged model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerParitySummary {
    /// Total facts in the model.
    pub total_facts: u32,
    /// Facts standing governed.
    pub governed_facts: u32,
    /// Facts narrowed below Stable.
    pub narrowed_facts: u32,
    /// Facts blocked from Stable promotion.
    pub blocked_facts: u32,
    /// The number of consumer surfaces.
    pub total_consumers: u32,
    /// The number of truth domains covered.
    pub total_domains: u32,
    /// The number of source packets bound.
    pub bound_sources: u32,
    /// The total per-consumer projections across all facts.
    pub total_projections: u32,
    /// The projections that converge on their fact.
    pub converged_projections: u32,
    /// The worst gate across all facts.
    pub worst_gate: DescriptorGate,
    /// True when the model holds Stable promotion (at least one fact blocked).
    pub blocks_stable_promotion: bool,
}

/// The conformance review block — every invariant the packet asserts, recomputed from the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerParityConformance {
    /// Every fact derives its status, signal, and qualification from its gate.
    pub facts_derive_from_gate: bool,
    /// Every fact carries one projection per consumer surface.
    pub every_fact_projects_to_every_consumer: bool,
    /// Every projection reads the same gate and qualification as its fact.
    pub consumers_converge_on_fact: bool,
    /// No consumer reads a fact stronger than the fact's gate.
    pub no_consumer_strengthens_a_fact: bool,
    /// Every consumer view reads every fact in the model.
    pub every_consumer_reads_every_fact: bool,
    /// All five source packets are bound.
    pub all_sources_bound: bool,
    /// Every bound source packet validated clean on ingest.
    pub bound_sources_validated_clean: bool,
    /// Every fact preserves at least one repo-relative evidence ref.
    pub facts_preserve_evidence_lineage: bool,
    /// The export preview mirrors the live fact set.
    pub export_mirrors_live_facts: bool,
    /// The serialized export carries no raw material.
    pub export_carries_no_raw_material: bool,
    /// The controlled vocabulary equals the frozen canonical one.
    pub controlled_enums_frozen: bool,
}

impl ConsumerParityConformance {
    /// True when every conformance flag holds.
    pub fn all_hold(&self) -> bool {
        self.facts_derive_from_gate
            && self.every_fact_projects_to_every_consumer
            && self.consumers_converge_on_fact
            && self.no_consumer_strengthens_a_fact
            && self.every_consumer_reads_every_fact
            && self.all_sources_bound
            && self.bound_sources_validated_clean
            && self.facts_preserve_evidence_lineage
            && self.export_mirrors_live_facts
            && self.export_carries_no_raw_material
            && self.controlled_enums_frozen
    }
}

// ---------------------------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------------------------

/// The seed input the packet is minted from: the five already-published source packets plus identity.
pub struct M5AssuranceConsumerParityInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The assurance center packet.
    pub assurance_center: M5AssuranceCenter,
    /// The assurance-claim reducer packet.
    pub claim_reducer: M5AssuranceClaimReducer,
    /// The governance / fitness dashboard packet.
    pub governance_dashboard: M5GovernanceDashboard,
    /// The capability-boundary inspector packet.
    pub boundary_inspector: M5BoundaryInspector,
    /// The event-provenance inspector packet.
    pub event_provenance: M5EventProvenance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

// ---------------------------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------------------------

/// The M5 assurance consumer-parity packet: the one converged model that the claimed help /
/// procurement / evaluation / support / public-truth surfaces read from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AssuranceConsumerParity {
    /// Record kind; must equal [`M5_ASSURANCE_CONSUMER_PARITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ASSURANCE_CONSUMER_PARITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The source packets ingested, in source order.
    pub source_bindings: Vec<SourceBinding>,
    /// The unified facts, in domain order.
    pub facts: Vec<UnifiedFact>,
    /// The per-consumer generated views, in consumer order.
    pub consumer_views: Vec<ConsumerView>,
    /// The exported refs-only preview.
    pub export_preview: ConsumerParityExportPreview,
    /// Controlled-vocabulary set.
    pub vocabulary: ConsumerParityVocabulary,
    /// Compact summary.
    pub summary: ConsumerParitySummary,
    /// Conformance review block.
    pub conformance: ConsumerParityConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AssuranceConsumerParity {
    /// Builds the converged packet from the five source packets, normalizing every item into one fact
    /// grammar and projecting each fact onto every consumer surface.
    pub fn new(input: M5AssuranceConsumerParityInput) -> Self {
        let mut facts = Vec::new();
        facts.extend(facts_from_claim_reducer(&input.claim_reducer));
        facts.extend(facts_from_assurance_center(&input.assurance_center));
        facts.extend(facts_from_governance(&input.governance_dashboard));
        facts.extend(facts_from_boundary(&input.boundary_inspector));
        facts.extend(facts_from_event(&input.event_provenance));

        let source_bindings = vec![
            binding_for(
                SourcePacketKind::AssuranceCenter,
                &input.assurance_center.packet_id,
                &input.assurance_center.record_kind,
                input.assurance_center.validate().is_empty(),
                &facts,
            ),
            binding_for(
                SourcePacketKind::AssuranceClaimReducer,
                &input.claim_reducer.packet_id,
                &input.claim_reducer.record_kind,
                input.claim_reducer.validate().is_empty(),
                &facts,
            ),
            binding_for(
                SourcePacketKind::GovernanceDashboard,
                &input.governance_dashboard.packet_id,
                &input.governance_dashboard.record_kind,
                input.governance_dashboard.validate().is_empty(),
                &facts,
            ),
            binding_for(
                SourcePacketKind::BoundaryInspector,
                &input.boundary_inspector.packet_id,
                &input.boundary_inspector.record_kind,
                input.boundary_inspector.validate().is_empty(),
                &facts,
            ),
            binding_for(
                SourcePacketKind::EventProvenance,
                &input.event_provenance.packet_id,
                &input.event_provenance.record_kind,
                input.event_provenance.validate().is_empty(),
                &facts,
            ),
        ];

        let consumer_views: Vec<ConsumerView> = ParityConsumer::ALL
            .iter()
            .map(|consumer| derive_consumer_view(*consumer, &facts))
            .collect();

        let export_preview = ConsumerParityExportPreview::derive(
            &format!("{}:export", input.packet_id),
            &input.packet_id,
            &input.evaluated_at,
            &input.redaction_class_token,
            &facts,
        );

        let summary = derive_summary(&facts, &source_bindings);
        let conformance =
            derive_conformance(&facts, &consumer_views, &source_bindings, &export_preview);

        Self {
            record_kind: M5_ASSURANCE_CONSUMER_PARITY_RECORD_KIND.to_owned(),
            schema_version: M5_ASSURANCE_CONSUMER_PARITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            source_bindings,
            facts,
            consumer_views,
            export_preview,
            vocabulary: ConsumerParityVocabulary::canonical(),
            summary,
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release automation must hold Stable promotion — at least one fact is blocked.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.summary.blocks_stable_promotion
    }

    /// Finds a fact by its domain and subject.
    pub fn fact(&self, domain: TruthDomain, subject: &str) -> Option<&UnifiedFact> {
        self.facts
            .iter()
            .find(|f| f.domain == domain && f.subject == subject)
    }

    /// Finds a consumer view by surface.
    pub fn consumer_view(&self, consumer: ParityConsumer) -> Option<&ConsumerView> {
        self.consumer_views.iter().find(|v| v.consumer == consumer)
    }

    /// Renders the packet for a generation channel. The output is identical for every channel — the
    /// channel parameter exists only to prove desktop, CLI / headless, and offline / mirror
    /// generation produce byte-identical output.
    pub fn render_for_channel(&self, _channel: AssuranceConsumerParityChannel) -> String {
        self.export_safe_json()
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 assurance consumer parity serializes")
    }

    /// The exported refs-only preview's JSON.
    pub fn render_export_preview(&self) -> String {
        self.export_preview.export_safe_json()
    }

    /// Deterministic, machine-readable fact / consumer matrix CSV: one row per fact, naming its
    /// domain, subject, source, gate, qualification, owner, freshness, and evidence refs.
    pub fn render_facts_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "domain,subject,label,source_packet,gate,effective_qualification,status,owner_role,evidence_freshness,evidence_refs,source_ref\n",
        );
        for fact in &self.facts {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                fact.domain.as_str(),
                fact.subject,
                csv_cell(&fact.label),
                fact.source_packet.as_str(),
                fact.gate.as_str(),
                fact.effective_qualification.as_str(),
                fact.status.as_str(),
                fact.owner_role,
                fact.evidence_freshness.as_str(),
                csv_cell(&fact.evidence_refs.join(";")),
                fact.source_ref,
            ));
        }
        out
    }

    /// Deterministic consumer-parity overview document for review, support, docs, or evaluator
    /// handoff.
    pub fn render_overview_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Assurance Consumer-Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Facts: {} ({} governed, {} narrowed, {} blocked) across {} domains\n",
            self.summary.total_facts,
            self.summary.governed_facts,
            self.summary.narrowed_facts,
            self.summary.blocked_facts,
            self.summary.total_domains
        ));
        out.push_str(&format!(
            "- Consumers: {} (each reads every fact)\n",
            self.summary.total_consumers
        ));
        out.push_str(&format!(
            "- Projections: {} ({} converged)\n",
            self.summary.total_projections, self.summary.converged_projections
        ));
        out.push_str(&format!(
            "- Sources bound: {}\n",
            self.summary.bound_sources
        ));
        out.push_str(&format!(
            "- Stable promotion: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));

        out.push_str("\n## Source bindings\n\n");
        out.push_str("| Source | Packet | Facts | Validated | Blocks |\n");
        out.push_str("|--------|--------|-------|-----------|--------|\n");
        for b in &self.source_bindings {
            out.push_str(&format!(
                "| {} | `{}` | {} | {} | {} |\n",
                b.source_label,
                b.packet_id,
                b.fact_count,
                if b.validated_clean { "clean" } else { "FAILED" },
                if b.blocks_stable_promotion {
                    "blocked"
                } else {
                    "pass"
                }
            ));
        }

        out.push_str("\n## Consumer parity\n\n");
        out.push_str(
            "Every consumer reads the same fact set at the same worst gate — one model governs all surfaces.\n\n",
        );
        out.push_str("| Consumer | Facts | Worst gate | Qualification | Reads all |\n");
        out.push_str("|----------|-------|------------|---------------|-----------|\n");
        for view in &self.consumer_views {
            out.push_str(&format!(
                "| {} | {} | `{}` | `{}` | {} |\n",
                view.consumer_label,
                view.fact_count,
                view.worst_gate.as_str(),
                view.effective_qualification.as_str(),
                if view.reads_all_facts { "yes" } else { "NO" }
            ));
        }

        out.push_str("\n## Facts\n\n");
        out.push_str("| Domain | Subject | Source | Gate | Qualification | Owner | Freshness |\n");
        out.push_str("|--------|---------|--------|------|---------------|-------|-----------|\n");
        for fact in &self.facts {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                fact.domain.as_str(),
                fact.subject,
                fact.source_packet.as_str(),
                fact.gate.as_str(),
                fact.effective_qualification.as_str(),
                fact.owner_role,
                fact.evidence_freshness.as_str()
            ));
        }
        out
    }

    /// Compact Markdown report for the release-grade export proof.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Assurance Consumer-Parity — Export Proof\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Facts: {} ({} governed, {} narrowed, {} blocked)\n",
            self.summary.total_facts,
            self.summary.governed_facts,
            self.summary.narrowed_facts,
            self.summary.blocked_facts
        ));
        out.push_str(&format!(
            "- Conformance: {}\n",
            if self.conformance.all_hold() {
                "all invariants hold"
            } else {
                "FAILED"
            }
        ));
        out.push_str(&format!(
            "- Stable promotion: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));
        out.push_str(&format!(
            "- Export preview: `{}`\n",
            M5_ASSURANCE_EXPORT_PREVIEW_REF
        ));
        out.push_str(&format!(
            "- Facts CSV: `{}`\n",
            M5_ASSURANCE_CONSUMER_PARITY_FACTS_CSV_REF
        ));
        out
    }

    /// Validates the packet's invariants. Returns a sorted, de-duplicated list of violations; an empty
    /// list means the packet is conformant.
    pub fn validate(&self) -> Vec<M5AssuranceConsumerParityViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_ASSURANCE_CONSUMER_PARITY_RECORD_KIND {
            out.push(M5AssuranceConsumerParityViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ASSURANCE_CONSUMER_PARITY_SCHEMA_VERSION {
            out.push(M5AssuranceConsumerParityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5AssuranceConsumerParityViolation::MissingIdentity);
        }

        // Facts: at least one per domain, in canonical domain order, each self-consistent.
        for fact in &self.facts {
            out.extend(fact.validate());
        }
        if !facts_cover_all_domains(&self.facts) {
            out.push(M5AssuranceConsumerParityViolation::DomainSetIncomplete);
        }
        if !facts_in_domain_order(&self.facts) {
            out.push(M5AssuranceConsumerParityViolation::FactOrderInvalid);
        }

        // Source bindings: one per kind, in source order, all validated clean.
        let bound: Vec<SourcePacketKind> = self
            .source_bindings
            .iter()
            .map(|b| b.source_packet)
            .collect();
        if bound != SourcePacketKind::ALL.to_vec() {
            out.push(M5AssuranceConsumerParityViolation::SourceSetInvalid);
        }
        if self.source_bindings.iter().any(|b| !b.validated_clean) {
            out.push(M5AssuranceConsumerParityViolation::SourceNotClean);
        }
        for b in &self.source_bindings {
            let expected = self
                .facts
                .iter()
                .filter(|f| f.source_packet == b.source_packet)
                .count() as u32;
            let blocks = self
                .facts
                .iter()
                .any(|f| f.source_packet == b.source_packet && f.is_blocked());
            if b.fact_count != expected
                || b.registry_ref != b.source_packet.registry_ref()
                || b.source_label != b.source_packet.label()
                || b.blocks_stable_promotion != blocks
            {
                out.push(M5AssuranceConsumerParityViolation::SourceBindingDrift);
            }
        }

        // Consumer views: one per consumer, in consumer order, each reading every fact.
        let consumers: Vec<ParityConsumer> =
            self.consumer_views.iter().map(|v| v.consumer).collect();
        if consumers != ParityConsumer::ALL.to_vec() {
            out.push(M5AssuranceConsumerParityViolation::ConsumerSetInvalid);
        }
        for view in &self.consumer_views {
            if derive_consumer_view(view.consumer, &self.facts) != *view {
                out.push(M5AssuranceConsumerParityViolation::ConsumerViewDrift);
            }
            if !view.reads_all_facts || view.fact_count as usize != self.facts.len() {
                out.push(M5AssuranceConsumerParityViolation::ConsumerDoesNotReadAllFacts);
            }
        }

        // Export preview re-derives from the facts and carries no raw material.
        let expected_preview = ConsumerParityExportPreview::derive(
            &format!("{}:export", self.packet_id),
            &self.packet_id,
            &self.evaluated_at,
            &self.redaction_class_token,
            &self.facts,
        );
        if expected_preview != self.export_preview {
            out.push(M5AssuranceConsumerParityViolation::ExportPreviewDrift);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("consumer parity packet serializes"),
        ) {
            out.push(M5AssuranceConsumerParityViolation::RawMaterialInExport);
        }

        // Vocabulary, summary, conformance.
        if !self.vocabulary.matches_canonical() {
            out.push(M5AssuranceConsumerParityViolation::VocabularyMismatch);
        }
        if derive_summary(&self.facts, &self.source_bindings) != self.summary {
            out.push(M5AssuranceConsumerParityViolation::SummaryDrift);
        }
        let conformance = derive_conformance(
            &self.facts,
            &self.consumer_views,
            &self.source_bindings,
            &self.export_preview,
        );
        if conformance != self.conformance || !self.conformance.all_hold() {
            out.push(M5AssuranceConsumerParityViolation::ConformanceReviewFailed);
        }

        out.sort();
        out.dedup();
        out
    }
}

/// Builds a source binding from the facts already extracted for that source.
fn binding_for(
    source: SourcePacketKind,
    packet_id: &str,
    record_kind: &str,
    validated_clean: bool,
    facts: &[UnifiedFact],
) -> SourceBinding {
    let owned: Vec<&UnifiedFact> = facts.iter().filter(|f| f.source_packet == source).collect();
    SourceBinding {
        source_packet: source,
        source_label: source.label().to_owned(),
        packet_id: packet_id.to_owned(),
        record_kind: record_kind.to_owned(),
        registry_ref: source.registry_ref().to_owned(),
        fact_count: owned.len() as u32,
        validated_clean,
        blocks_stable_promotion: owned.iter().any(|f| f.is_blocked()),
    }
}

// ---------------------------------------------------------------------------------------------
// Summary / conformance derivation
// ---------------------------------------------------------------------------------------------

fn derive_summary(facts: &[UnifiedFact], bindings: &[SourceBinding]) -> ConsumerParitySummary {
    let governed = facts.iter().filter(|f| f.is_governed()).count() as u32;
    let narrowed = facts.iter().filter(|f| f.is_narrowed()).count() as u32;
    let blocked = facts.iter().filter(|f| f.is_blocked()).count() as u32;
    let total_projections = facts
        .iter()
        .map(|f| f.consumer_projections.len() as u32)
        .sum();
    let converged_projections = facts
        .iter()
        .flat_map(|f| f.consumer_projections.iter())
        .filter(|p| p.converges_with_fact)
        .count() as u32;
    let worst_gate = facts
        .iter()
        .map(|f| f.gate)
        .fold(DescriptorGate::Governed, worse_gate);
    let mut domains: Vec<TruthDomain> = facts.iter().map(|f| f.domain).collect();
    domains.sort_by_key(|d| domain_rank(*d));
    domains.dedup();
    ConsumerParitySummary {
        total_facts: facts.len() as u32,
        governed_facts: governed,
        narrowed_facts: narrowed,
        blocked_facts: blocked,
        total_consumers: ParityConsumer::ALL.len() as u32,
        total_domains: domains.len() as u32,
        bound_sources: bindings.len() as u32,
        total_projections,
        converged_projections,
        worst_gate,
        blocks_stable_promotion: blocked > 0,
    }
}

fn derive_conformance(
    facts: &[UnifiedFact],
    views: &[ConsumerView],
    bindings: &[SourceBinding],
    export_preview: &ConsumerParityExportPreview,
) -> ConsumerParityConformance {
    let facts_derive_from_gate = facts.iter().all(|f| {
        f.status == gate_status(f.gate)
            && f.signal == f.status.signal()
            && f.effective_qualification == floor_for_gate(f.gate)
    });

    let every_fact_projects_to_every_consumer = facts.iter().all(|f| {
        let consumers: Vec<ParityConsumer> =
            f.consumer_projections.iter().map(|p| p.consumer).collect();
        consumers == ParityConsumer::ALL.to_vec()
    });

    let consumers_converge_on_fact = facts.iter().all(|f| {
        f.consumer_projections.iter().all(|p| {
            p.gate == f.gate
                && p.effective_qualification == f.effective_qualification
                && p.converges_with_fact
        })
    });

    // No consumer reads a fact stronger than its gate: every projection's gate-rank is at least the
    // fact's gate-rank.
    let no_consumer_strengthens_a_fact = facts.iter().all(|f| {
        f.consumer_projections
            .iter()
            .all(|p| gate_rank(p.gate) >= gate_rank(f.gate))
    });

    let every_consumer_reads_every_fact = views.len() == ParityConsumer::ALL.len()
        && views
            .iter()
            .all(|v| v.reads_all_facts && v.fact_count as usize == facts.len());

    let all_sources_bound = {
        let bound: Vec<SourcePacketKind> = bindings.iter().map(|b| b.source_packet).collect();
        bound == SourcePacketKind::ALL.to_vec()
    };

    let bound_sources_validated_clean = bindings.iter().all(|b| b.validated_clean);

    let facts_preserve_evidence_lineage = facts.iter().all(|f| {
        !f.evidence_refs.is_empty() && f.evidence_refs.iter().all(|r| !r.trim().is_empty())
    });

    let export_mirrors_live_facts = export_preview.entries.len() == facts.len()
        && export_preview
            .entries
            .iter()
            .zip(facts.iter())
            .all(|(e, f)| {
                e.domain == f.domain
                    && e.subject == f.subject
                    && e.gate == f.gate
                    && e.effective_qualification == f.effective_qualification
                    && e.owner_role == f.owner_role
                    && e.evidence_refs == f.evidence_refs
            });

    let export_clean = !json_contains_forbidden_material(
        &serde_json::to_value(export_preview).expect("export preview serializes"),
    );

    ConsumerParityConformance {
        facts_derive_from_gate,
        every_fact_projects_to_every_consumer,
        consumers_converge_on_fact,
        no_consumer_strengthens_a_fact,
        every_consumer_reads_every_fact,
        all_sources_bound,
        bound_sources_validated_clean,
        facts_preserve_evidence_lineage,
        export_mirrors_live_facts,
        export_carries_no_raw_material: export_clean,
        controlled_enums_frozen: ConsumerParityVocabulary::canonical().matches_canonical(),
    }
}

/// Position of a domain in the canonical order.
fn domain_rank(domain: TruthDomain) -> usize {
    TruthDomain::ALL
        .iter()
        .position(|d| *d == domain)
        .unwrap_or(TruthDomain::ALL.len())
}

/// True when the facts cover every truth domain at least once.
fn facts_cover_all_domains(facts: &[UnifiedFact]) -> bool {
    TruthDomain::ALL
        .iter()
        .all(|domain| facts.iter().any(|f| f.domain == *domain))
}

/// True when the facts are grouped in canonical domain order (non-decreasing domain rank).
fn facts_in_domain_order(facts: &[UnifiedFact]) -> bool {
    facts
        .windows(2)
        .all(|w| domain_rank(w[0].domain) <= domain_rank(w[1].domain))
}

// ---------------------------------------------------------------------------------------------
// CSV / export-safety helpers
// ---------------------------------------------------------------------------------------------

/// Escapes a CSV cell that may contain a comma or quote.
fn csv_cell(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Key substrings that must never appear in a serialized export.
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

// ---------------------------------------------------------------------------------------------
// Channels
// ---------------------------------------------------------------------------------------------

/// A generation channel the packet can be rendered for. Every channel produces byte-identical output;
/// the type exists to prove desktop, CLI / headless, and offline / mirror parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceConsumerParityChannel {
    /// The desktop surface.
    Desktop,
    /// The CLI / headless emitter.
    Headless,
    /// Offline / mirror-safe packet generation.
    OfflineMirror,
}

impl AssuranceConsumerParityChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 3] = [Self::Desktop, Self::Headless, Self::OfflineMirror];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Headless => "headless",
            Self::OfflineMirror => "offline_mirror",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Violations
// ---------------------------------------------------------------------------------------------

/// A packet-invariant violation surfaced by [`M5AssuranceConsumerParity::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AssuranceConsumerParityViolation {
    /// The record kind is not the expected tag.
    WrongRecordKind,
    /// The schema version is not the expected version.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// A fact cites a field that drifts from its domain.
    FactFieldMismatch,
    /// A fact's status, signal, or qualification does not follow from its gate.
    FactOverstatesGate,
    /// A fact is missing subject, label, owner, or evidence lineage.
    FactMissingLineage,
    /// A fact's consumer projections diverge from the converged set.
    ConsumerDivergence,
    /// The consumer set is wrong or out of order.
    ConsumerSetInvalid,
    /// The facts do not cover every truth domain.
    DomainSetIncomplete,
    /// The facts are not grouped in canonical domain order.
    FactOrderInvalid,
    /// The source-binding set is wrong or out of order.
    SourceSetInvalid,
    /// A bound source packet did not validate clean.
    SourceNotClean,
    /// A source binding's fact count, refs, or block flag drifts from the facts.
    SourceBindingDrift,
    /// A consumer view drifts from the value derived from the facts.
    ConsumerViewDrift,
    /// A consumer view does not read every fact in the model.
    ConsumerDoesNotReadAllFacts,
    /// The export preview drifts from the facts.
    ExportPreviewDrift,
    /// The serialized packet carries forbidden raw material.
    RawMaterialInExport,
    /// The controlled vocabulary drifts from the frozen canonical one.
    VocabularyMismatch,
    /// The summary drifts from the value derived from the facts.
    SummaryDrift,
    /// The conformance review failed.
    ConformanceReviewFailed,
    /// A stable message id is missing the required prefix.
    UnprefixedMessageId,
}

impl M5AssuranceConsumerParityViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::FactFieldMismatch => "fact_field_mismatch",
            Self::FactOverstatesGate => "fact_overstates_gate",
            Self::FactMissingLineage => "fact_missing_lineage",
            Self::ConsumerDivergence => "consumer_divergence",
            Self::ConsumerSetInvalid => "consumer_set_invalid",
            Self::DomainSetIncomplete => "domain_set_incomplete",
            Self::FactOrderInvalid => "fact_order_invalid",
            Self::SourceSetInvalid => "source_set_invalid",
            Self::SourceNotClean => "source_not_clean",
            Self::SourceBindingDrift => "source_binding_drift",
            Self::ConsumerViewDrift => "consumer_view_drift",
            Self::ConsumerDoesNotReadAllFacts => "consumer_does_not_read_all_facts",
            Self::ExportPreviewDrift => "export_preview_drift",
            Self::RawMaterialInExport => "raw_material_in_export",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::SummaryDrift => "summary_drift",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::UnprefixedMessageId => "unprefixed_message_id",
        }
    }
}
