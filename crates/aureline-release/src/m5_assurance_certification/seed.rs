//! Canonical seed builders for the M5 assurance certification.
//!
//! These builders are the single producer of the checked-in certification packet, the published
//! inventory, the rendered certification document, the machine-readable grid CSV, the release-grade
//! parity proof (and its Markdown report), and the per-state drill fixtures. The headless emitter and
//! the inline tests both call them so the in-code packet, the artifacts, and the fixtures never
//! drift.
//!
//! The certification is a pure projection of the [governance
//! matrix](crate::m5_assurance_route_governance): every variant is built by qualifying the same
//! claimed profile grid against one of the three seeded governance packets. The canonical packet
//! qualifies against the all-current matrix, so every claimed profile stands at Stable; the drills
//! qualify against the matrix's stale-proof and missing-proof variants and let the projection
//! recompute each profile's outcome, gate, and effective qualification.

use super::*;

use crate::m5_assurance_route_governance::{
    seeded_m5_assurance_route_governance,
    seeded_m5_assurance_route_governance_missing_proof_blocked,
    seeded_m5_assurance_route_governance_stale_proof_narrowed,
};

/// Stable packet id for the canonical certification packet.
pub const M5_ASSURANCE_CERTIFICATION_PACKET_ID: &str = "m5-assurance-certification:stable:0001";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The claimed M5 deployment profiles, each claiming a Stable assurance posture. Every claimed
/// profile is qualified against the governed facets that scope to it; a facet that does not scope to
/// a profile (an exception / waiver or approval ticket that does not apply to a sovereign / air-gapped
/// deployment) is simply dropped from that profile's backing set, never overstated.
fn claimed_grid() -> Vec<(ClaimedPosture, QualificationClass)> {
    ClaimedPosture::ALL
        .iter()
        .map(|profile| (*profile, QualificationClass::Stable))
        .collect()
}

/// Assembles a certification packet by projecting a governance matrix onto the claimed grid.
fn assemble(
    packet_id: &str,
    report_label: &str,
    governance: &M5AssuranceRouteGovernance,
) -> M5AssuranceCertification {
    M5AssuranceCertification::from_governance(
        governance,
        M5AssuranceCertificationInput {
            packet_id: packet_id.to_owned(),
            report_label: report_label.to_owned(),
            profiles: claimed_grid(),
            redaction_class_token: REDACTION_CLASS.to_owned(),
            minted_at: governance.minted_at.clone(),
        },
    )
}

/// The canonical certification: every claimed profile qualifies against the all-current governance
/// matrix, so every applicable dimension is certified and every profile stands at its claimed Stable
/// qualification.
pub fn seeded_m5_assurance_certification() -> M5AssuranceCertification {
    assemble(
        M5_ASSURANCE_CERTIFICATION_PACKET_ID,
        "M5 assurance / governance / route-provenance certification",
        &seeded_m5_assurance_route_governance(),
    )
}

/// Drill: the governance matrix's route-hop proof is stale, so every claimed profile narrows on the
/// boundary / route dimension (the dimension the route-hop facet backs) while the assurance,
/// governance, and event-provenance dimensions stay certified — proving narrowing is per dimension,
/// not behind a generic stable badge.
pub fn seeded_m5_assurance_certification_stale_proof_narrowed() -> M5AssuranceCertification {
    assemble(
        "m5-assurance-certification:drill-stale:0001",
        "M5 assurance certification — stale-proof drill",
        &seeded_m5_assurance_route_governance_stale_proof_narrowed(),
    )
}

/// Drill: the governance matrix's event-provenance proof is missing, so every claimed profile blocks
/// on the event-provenance dimension and the consumers that surface it block from Stable promotion,
/// while About / help — which does not surface event provenance — stays certified.
pub fn seeded_m5_assurance_certification_missing_proof_blocked() -> M5AssuranceCertification {
    assemble(
        "m5-assurance-certification:drill-missing:0001",
        "M5 assurance certification — missing-proof drill",
        &seeded_m5_assurance_route_governance_missing_proof_blocked(),
    )
}
