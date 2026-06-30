//! Canonical seed builders for the M5 assurance consumer-parity model.
//!
//! These builders are the single producer of the checked-in parity packet, the published inventory,
//! the rendered overview document, the machine-readable fact / consumer matrix CSV, the release-grade
//! export proof (and its Markdown report), the exported refs-only preview, and the per-state drill
//! fixtures. The headless emitter and the inline tests both call them so the in-code packet, the
//! artifacts, and the fixtures never drift. The canonical packet ingests every source lane at its
//! all-governed seed, so every fact stays governed and every consumer reads the same governed fact;
//! each drill swaps exactly one source lane for one of its own narrowing / blocking drills and lets
//! the parity model carry that one narrowing through to every consumer at once.

use super::*;

use crate::m5_assurance_center::seeded_m5_assurance_center;
use crate::m5_assurance_claim_reducer::{
    seeded_m5_assurance_claim_reducer, seeded_m5_assurance_claim_reducer_stale_evidence_narrowed,
};
use crate::m5_boundary_inspector::{
    seeded_m5_boundary_inspector, seeded_m5_boundary_inspector_route_unattributed_blocked,
};
use crate::m5_event_provenance::{
    seeded_m5_event_provenance, seeded_m5_event_provenance_drift_tenant_blocked,
};
use crate::m5_governance_dashboard::{
    seeded_m5_governance_dashboard, seeded_m5_governance_dashboard_missing_evidence_blocked,
};

/// Stable packet id for the canonical (all-governed) parity packet.
pub const M5_ASSURANCE_CONSUMER_PARITY_PACKET_ID: &str = "m5-assurance-consumer-parity:stable:0001";

/// Evaluation / mint timestamp for the canonical packet.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

/// Default redaction class for the seeded packets.
const REDACTION_CLASS: &str = "metadata_safe_default";

/// Assembles a parity packet from the five source packets.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    assurance_center: M5AssuranceCenter,
    claim_reducer: M5AssuranceClaimReducer,
    governance_dashboard: M5GovernanceDashboard,
    boundary_inspector: M5BoundaryInspector,
    event_provenance: M5EventProvenance,
) -> M5AssuranceConsumerParity {
    M5AssuranceConsumerParity::new(M5AssuranceConsumerParityInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        assurance_center,
        claim_reducer,
        governance_dashboard,
        boundary_inspector,
        event_provenance,
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-governed parity packet: every source lane stands fully governed, so every fact
/// stays governed and every consumer reads the same governed fact set.
pub fn seeded_m5_assurance_consumer_parity() -> M5AssuranceConsumerParity {
    assemble_packet(
        M5_ASSURANCE_CONSUMER_PARITY_PACKET_ID,
        "M5 assurance consumer-parity",
        seeded_m5_assurance_center(),
        seeded_m5_assurance_claim_reducer(),
        seeded_m5_governance_dashboard(),
        seeded_m5_boundary_inspector(),
        seeded_m5_event_provenance(),
    )
}

/// Drill: the assurance-claim reducer narrows on stale evidence, so the assurance-claim facts narrow
/// to Beta and every consumer reads them narrowed — no surface can still read those claims as Stable.
pub fn seeded_m5_assurance_consumer_parity_claim_narrowed() -> M5AssuranceConsumerParity {
    assemble_packet(
        "m5-assurance-consumer-parity:drill-claim-narrowed:0001",
        "M5 assurance consumer-parity — claim-narrowed drill",
        seeded_m5_assurance_center(),
        seeded_m5_assurance_claim_reducer_stale_evidence_narrowed(),
        seeded_m5_governance_dashboard(),
        seeded_m5_boundary_inspector(),
        seeded_m5_event_provenance(),
    )
}

/// Drill: the governance / fitness dashboard blocks on missing evidence, so the governance facts block
/// to Unavailable, every consumer reads them blocked, and the parity packet holds Stable promotion.
pub fn seeded_m5_assurance_consumer_parity_governance_blocked() -> M5AssuranceConsumerParity {
    assemble_packet(
        "m5-assurance-consumer-parity:drill-governance-blocked:0001",
        "M5 assurance consumer-parity — governance-blocked drill",
        seeded_m5_assurance_center(),
        seeded_m5_assurance_claim_reducer(),
        seeded_m5_governance_dashboard_missing_evidence_blocked(),
        seeded_m5_boundary_inspector(),
        seeded_m5_event_provenance(),
    )
}

/// Drill: the capability-boundary inspector blocks on an unattributed route hop, so the route facts
/// block to Unavailable, every consumer reads them blocked, and the parity packet holds Stable
/// promotion.
pub fn seeded_m5_assurance_consumer_parity_boundary_route_blocked() -> M5AssuranceConsumerParity {
    assemble_packet(
        "m5-assurance-consumer-parity:drill-boundary-route-blocked:0001",
        "M5 assurance consumer-parity — boundary-route-blocked drill",
        seeded_m5_assurance_center(),
        seeded_m5_assurance_claim_reducer(),
        seeded_m5_governance_dashboard(),
        seeded_m5_boundary_inspector_route_unattributed_blocked(),
        seeded_m5_event_provenance(),
    )
}

/// Drill: the event-provenance inspector blocks on a changed tenant, so the event facts block to
/// Unavailable, every consumer reads them blocked, and the parity packet holds Stable promotion.
pub fn seeded_m5_assurance_consumer_parity_event_blocked() -> M5AssuranceConsumerParity {
    assemble_packet(
        "m5-assurance-consumer-parity:drill-event-blocked:0001",
        "M5 assurance consumer-parity — event-blocked drill",
        seeded_m5_assurance_center(),
        seeded_m5_assurance_claim_reducer(),
        seeded_m5_governance_dashboard(),
        seeded_m5_boundary_inspector(),
        seeded_m5_event_provenance_drift_tenant_blocked(),
    )
}
