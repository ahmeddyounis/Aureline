//! Canonical seed builders for the M5 assurance / governance / route-provenance governance matrix.
//!
//! These builders are the single producer of the checked-in governance packet, the published
//! inventory, the rendered governance document, the machine-readable matrix CSV, the release-grade
//! parity proof (and its Markdown report), and the stale / missing drill fixtures. The headless
//! emitter and the inline tests both call them so the in-code packet, the artifacts, and the
//! fixtures never drift. Every consumer's verdict is derived from the same governed facets: the
//! canonical packet certifies every facet current and every assurance state governed, so every
//! consumer stands fully certified at Stable; the drills perturb one facet's proof freshness and
//! let the derivation recompute each consumer's status, gate, effective qualification, and gaps.

use super::*;

/// Stable packet id for the canonical (all-current) governance packet.
pub const M5_ASSURANCE_ROUTE_PACKET_ID: &str = "m5-assurance-route-governance:stable:0001";

/// Evaluation / mint timestamp for the canonical packet — a date at which every facet's proof is
/// current.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The facet the stale drill perturbs. It is read by the route / provenance inspectors and
/// capability inspector but not the assurance center, governance dashboard, admin, or About/help, so
/// the drill narrows exactly the consumers that depend on the route timeline.
const STALE_DRILL_FACET: AssuranceFacet = AssuranceFacet::RouteHop;

/// The facet the missing drill perturbs. It is read by the route inspector, procurement, and
/// support exports, so the drill blocks exactly the consumers that depend on the event-provenance
/// inspector.
const MISSING_DRILL_FACET: AssuranceFacet = AssuranceFacet::EventProvenance;

/// The governed facet definitions: each facet's current canonical state, the evidence classes it
/// binds, the claimed postures and trust boundaries it scopes to, and its degraded-data behavior.
/// Every baseline state is governed, so the canonical packet certifies every consumer at Stable.
#[allow(clippy::type_complexity)]
const FACET_DEFS: [(
    AssuranceFacet,
    CanonicalState,
    &[EvidenceClass],
    &[ClaimedPosture],
    &[TrustBoundary],
    DegradedDataBehavior,
); 9] = [
    (
        AssuranceFacet::AssuranceClaim,
        CanonicalState::AssuranceClaim(AssuranceClaimState::Proven),
        &[
            EvidenceClass::ControlAttestation,
            EvidenceClass::PolicyBundle,
            EvidenceClass::BoundaryManifest,
        ],
        &[
            ClaimedPosture::Managed,
            ClaimedPosture::SelfHosted,
            ClaimedPosture::Regulated,
            ClaimedPosture::Sovereign,
        ],
        &[TrustBoundary::LocalFirst, TrustBoundary::ControlPlane],
        DegradedDataBehavior::MirroredLabelled,
    ),
    (
        AssuranceFacet::ControlProof,
        CanonicalState::AssuranceClaim(AssuranceClaimState::Attested),
        &[
            EvidenceClass::ControlAttestation,
            EvidenceClass::PolicyBundle,
        ],
        &[
            ClaimedPosture::Managed,
            ClaimedPosture::SelfHosted,
            ClaimedPosture::Regulated,
            ClaimedPosture::Sovereign,
        ],
        &[TrustBoundary::LocalFirst, TrustBoundary::ControlPlane],
        DegradedDataBehavior::OfflineCached,
    ),
    (
        AssuranceFacet::ExceptionWaiver,
        CanonicalState::Governance(GovernanceState::Pass),
        &[EvidenceClass::WaiverRecord, EvidenceClass::PolicyBundle],
        &[
            ClaimedPosture::Managed,
            ClaimedPosture::SelfHosted,
            ClaimedPosture::Regulated,
        ],
        &[TrustBoundary::LocalFirst, TrustBoundary::ControlPlane],
        DegradedDataBehavior::StaleBannerShown,
    ),
    (
        AssuranceFacet::GovernanceFreshness,
        CanonicalState::Governance(GovernanceState::Monitored),
        &[
            EvidenceClass::PolicyBundle,
            EvidenceClass::ControlAttestation,
        ],
        &[
            ClaimedPosture::Managed,
            ClaimedPosture::SelfHosted,
            ClaimedPosture::Regulated,
            ClaimedPosture::Sovereign,
        ],
        &[TrustBoundary::LocalFirst, TrustBoundary::ControlPlane],
        DegradedDataBehavior::StaleBannerShown,
    ),
    (
        AssuranceFacet::ServiceOwnership,
        CanonicalState::Governance(GovernanceState::Pass),
        &[EvidenceClass::OwnershipRegister, EvidenceClass::PolicyBundle],
        &[
            ClaimedPosture::Managed,
            ClaimedPosture::SelfHosted,
            ClaimedPosture::Regulated,
            ClaimedPosture::Sovereign,
        ],
        &[TrustBoundary::LocalFirst, TrustBoundary::ControlPlane],
        DegradedDataBehavior::MirroredLabelled,
    ),
    (
        AssuranceFacet::CapabilityBoundary,
        CanonicalState::CapabilityBoundary(CapabilityBoundaryState::WithinBoundary),
        &[
            EvidenceClass::BoundaryManifest,
            EvidenceClass::ControlAttestation,
        ],
        &[
            ClaimedPosture::Managed,
            ClaimedPosture::SelfHosted,
            ClaimedPosture::Regulated,
            ClaimedPosture::Sovereign,
        ],
        &[TrustBoundary::LocalFirst, TrustBoundary::ControlPlane],
        DegradedDataBehavior::LocalLineageOnly,
    ),
    (
        AssuranceFacet::RouteHop,
        CanonicalState::RouteHop(RouteHopState::AttributedRemote),
        &[EvidenceClass::RouteTimeline, EvidenceClass::ProvenanceLedger],
        &[
            ClaimedPosture::Managed,
            ClaimedPosture::SelfHosted,
            ClaimedPosture::Regulated,
            ClaimedPosture::Sovereign,
        ],
        &[TrustBoundary::LocalFirst, TrustBoundary::ControlPlane],
        DegradedDataBehavior::LocalLineageOnly,
    ),
    (
        AssuranceFacet::ApprovalTicket,
        CanonicalState::Approval(ApprovalState::Approved),
        &[
            EvidenceClass::RuntimeApprovalRecord,
            EvidenceClass::PolicyBundle,
        ],
        &[
            ClaimedPosture::Managed,
            ClaimedPosture::SelfHosted,
            ClaimedPosture::Regulated,
        ],
        &[TrustBoundary::LocalFirst, TrustBoundary::ControlPlane],
        DegradedDataBehavior::OfflineCached,
    ),
    (
        AssuranceFacet::EventProvenance,
        CanonicalState::Provenance(ProvenanceState::FullyTraced),
        &[
            EvidenceClass::ProvenanceLedger,
            EvidenceClass::RouteTimeline,
        ],
        &[
            ClaimedPosture::Managed,
            ClaimedPosture::SelfHosted,
            ClaimedPosture::Regulated,
            ClaimedPosture::Sovereign,
        ],
        &[TrustBoundary::LocalFirst, TrustBoundary::ControlPlane],
        DegradedDataBehavior::LocalLineageOnly,
    ),
];

/// The claimed consumer surfaces and the facets each reads. Together the reads cover every facet.
const CONSUMER_DEFS: [(AssuranceConsumer, QualificationClass, &[AssuranceFacet]); 8] = [
    (
        AssuranceConsumer::AssuranceCenter,
        QualificationClass::Stable,
        &[
            AssuranceFacet::AssuranceClaim,
            AssuranceFacet::ControlProof,
            AssuranceFacet::ExceptionWaiver,
            AssuranceFacet::CapabilityBoundary,
        ],
    ),
    (
        AssuranceConsumer::GovernanceDashboard,
        QualificationClass::Stable,
        &[
            AssuranceFacet::AssuranceClaim,
            AssuranceFacet::ControlProof,
            AssuranceFacet::ExceptionWaiver,
            AssuranceFacet::GovernanceFreshness,
            AssuranceFacet::ServiceOwnership,
        ],
    ),
    (
        AssuranceConsumer::CapabilityInspector,
        QualificationClass::Stable,
        &[
            AssuranceFacet::CapabilityBoundary,
            AssuranceFacet::ServiceOwnership,
            AssuranceFacet::RouteHop,
        ],
    ),
    (
        AssuranceConsumer::RouteInspector,
        QualificationClass::Stable,
        &[
            AssuranceFacet::RouteHop,
            AssuranceFacet::ApprovalTicket,
            AssuranceFacet::EventProvenance,
        ],
    ),
    (
        AssuranceConsumer::AdminConsole,
        QualificationClass::Stable,
        &[
            AssuranceFacet::ServiceOwnership,
            AssuranceFacet::ApprovalTicket,
            AssuranceFacet::ExceptionWaiver,
            AssuranceFacet::GovernanceFreshness,
        ],
    ),
    (
        AssuranceConsumer::HelpAbout,
        QualificationClass::Stable,
        &[
            AssuranceFacet::AssuranceClaim,
            AssuranceFacet::CapabilityBoundary,
            AssuranceFacet::ServiceOwnership,
        ],
    ),
    (
        AssuranceConsumer::ProcurementEvaluation,
        QualificationClass::Stable,
        &[
            AssuranceFacet::AssuranceClaim,
            AssuranceFacet::ControlProof,
            AssuranceFacet::ExceptionWaiver,
            AssuranceFacet::GovernanceFreshness,
            AssuranceFacet::ServiceOwnership,
            AssuranceFacet::CapabilityBoundary,
            AssuranceFacet::RouteHop,
            AssuranceFacet::ApprovalTicket,
            AssuranceFacet::EventProvenance,
        ],
    ),
    (
        AssuranceConsumer::SupportExport,
        QualificationClass::Stable,
        &[
            AssuranceFacet::ControlProof,
            AssuranceFacet::GovernanceFreshness,
            AssuranceFacet::RouteHop,
            AssuranceFacet::ApprovalTicket,
            AssuranceFacet::EventProvenance,
        ],
    ),
];

/// Builds the canonical governed facets with every proof current.
fn canonical_facets() -> Vec<AssuranceFacetRow> {
    FACET_DEFS
        .iter()
        .map(|(facet, state, classes, postures, boundaries, degraded)| {
            AssuranceFacetRow::new(
                *facet,
                *state,
                FreshnessState::Current,
                classes,
                postures,
                boundaries,
                *degraded,
            )
        })
        .collect()
}

/// Marks one facet's proof at the given freshness state.
fn with_facet_state(
    mut facets: Vec<AssuranceFacetRow>,
    facet: AssuranceFacet,
    state: FreshnessState,
) -> Vec<AssuranceFacetRow> {
    for row in &mut facets {
        if row.facet == facet {
            let def = FACET_DEFS
                .iter()
                .find(|(f, ..)| *f == facet)
                .expect("facet has a definition");
            *row = AssuranceFacetRow::new(facet, def.1, state, def.2, def.3, def.4, def.5);
        }
    }
    facets
}

/// Builds the claimed consumer rows; unions, gaps, and verdict are recomputed in the packet.
fn consumer_rows() -> Vec<AssuranceConsumerRow> {
    CONSUMER_DEFS
        .iter()
        .map(|(consumer, claimed, facets)| AssuranceConsumerRow::new(*consumer, *claimed, facets))
        .collect()
}

/// Assembles a packet from the given governed facets.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    facets: Vec<AssuranceFacetRow>,
) -> M5AssuranceRouteGovernance {
    M5AssuranceRouteGovernance::new(M5AssuranceRouteInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        facets,
        consumers: consumer_rows(),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-current assurance / governance / route-provenance governance packet: every
/// facet governed at a current proof and a governed assurance state, so every consumer stands fully
/// certified at Stable.
pub fn seeded_m5_assurance_route_governance() -> M5AssuranceRouteGovernance {
    assemble_packet(
        M5_ASSURANCE_ROUTE_PACKET_ID,
        "M5 assurance / governance / route-provenance governance matrix",
        canonical_facets(),
    )
}

/// Drill: one facet's proof is stale, so the consumers that read it auto-narrow below Stable.
pub fn seeded_m5_assurance_route_governance_stale_proof_narrowed() -> M5AssuranceRouteGovernance {
    let facets = with_facet_state(canonical_facets(), STALE_DRILL_FACET, FreshnessState::Stale);
    assemble_packet(
        "m5-assurance-route-governance:drill-stale:0001",
        "M5 assurance / governance / route-provenance governance — stale-proof drill",
        facets,
    )
}

/// Drill: one facet's proof is missing, so the consumers that read it are blocked from Stable
/// promotion.
pub fn seeded_m5_assurance_route_governance_missing_proof_blocked() -> M5AssuranceRouteGovernance {
    let facets = with_facet_state(
        canonical_facets(),
        MISSING_DRILL_FACET,
        FreshnessState::Missing,
    );
    assemble_packet(
        "m5-assurance-route-governance:drill-missing:0001",
        "M5 assurance / governance / route-provenance governance — missing-proof drill",
        facets,
    )
}
