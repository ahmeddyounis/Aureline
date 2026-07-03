//! The M5 assurance / governance / capability-boundary / route-and-event-provenance governance
//! matrix — one frozen baseline every claimed assurance-center, governance-dashboard,
//! capability-boundary, route-hop, approval-ticket, and event-provenance surface qualifies against
//! before deeper implementation widens a regulated, sovereign, self-hosted, or managed claim.
//!
//! The current release sheet already covers locality / tenant truth, policy / admin packets,
//! runtime approvals, descriptor / badge publication, and some operator dashboards, but it leaves
//! the assurance center, governance / fitness dashboard, capability-boundary inspector, route /
//! transport-route timeline, approval-ticket, and event-provenance inspector too implicit to
//! implement directly. The source set treats assurance claims, control-proof rows, exception /
//! waiver disclosure, governance freshness, service ownership, decision-right visibility, route
//! timelines, and provenance inspectors as first-class product truth rather than procurement-only
//! reports or scattered debug panels. This lane closes that gap with one governed matrix rather
//! than parallel assurance prose, admin panels, and hidden debug state.
//!
//! The matrix has three parts:
//!
//! - The canonical [state families](AssuranceStateFamily): one ordered, gate-bound vocabulary each
//!   for [assurance-claim](AssuranceClaimState), [governance](GovernanceState),
//!   [capability-boundary](CapabilityBoundaryState), [route-hop](RouteHopState),
//!   [approval](ApprovalState), and [provenance](ProvenanceState) state. Every token binds to a
//!   shared [gate posture](crate::m5_descriptor_badge::DescriptorGate) and an effective
//!   [qualification floor](crate::m5_descriptor_badge::QualificationClass) so the states are reused
//!   across surfaces instead of restated as ad hoc labels — and so a governance dashboard's `pass`,
//!   `stale`, `waived`, and `blocked` postures resolve to one frozen vocabulary.
//! - The governed [assurance facets](AssuranceFacet): the nine product surfaces the source set
//!   treats as governed truth (assurance claim, control proof, exception / waiver, governance
//!   freshness, service ownership, capability boundary, route hop, approval ticket, event
//!   provenance). Each facet names its [dimension](AssuranceDimension), the state family that
//!   governs it, the evidence classes it binds, the claimed posture lines it scopes to, the trust
//!   boundaries it covers, its degraded-data behavior, an owner role, and the proof path plus
//!   [freshness](crate::m5_descriptor_badge::FreshnessState) that keeps it current.
//! - The claimed [consumer surfaces](AssuranceConsumer): assurance center, governance dashboard,
//!   capability-boundary inspector, route / event-provenance inspector, admin console, About / help,
//!   procurement / evaluation, and support exports. Each binds the facets it reads, and the matrix
//!   *derives* its coverage gaps, gate decision, and effective qualification from those facets'
//!   proof freshness and current assurance state.
//!
//! Gaps in *proof* (a stale, expired, or missing facet proof, or a facet a consumer reads that the
//! matrix does not govern) and gaps in *assurance coverage* (a facet whose current canonical state
//! itself narrows or blocks) both fail the matrix rather than remaining implied: a stale facet
//! deterministically narrows every consumer that reads it below Stable, and an expired / missing /
//! ungoverned facet — or a facet in a blocking assurance state — blocks that consumer from Stable
//! promotion, with the gap named per consumer and its drifted dimension.
//!
//! The [`M5AssuranceRouteGovernance`] packet is the one inspectable, serde-serializable governance
//! truth assurance, governance, route, admin, help, procurement, and support surfaces consume rather
//! than maintaining parallel assurance / route inventories; it carries metadata and refs only — no
//! credential bodies or raw provider payloads, and so an exported packet preserves route / evidence
//! lineage without leaking secrets.
//!
//! - Packet schema:
//!   [`schemas/release/m5-assurance-route-governance.schema.json`](../../../../../schemas/release/m5-assurance-route-governance.schema.json)
//! - Contract doc:
//!   [`docs/release/m5-assurance-route-governance-contract.md`](../../../../../docs/release/m5-assurance-route-governance-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_assurance_route_governance,
    seeded_m5_assurance_route_governance_missing_proof_blocked,
    seeded_m5_assurance_route_governance_stale_proof_narrowed, M5_ASSURANCE_ROUTE_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The matrix reuses the descriptor / badge runtime's frozen gate vocabulary so the assurance
// governance layer and the public-truth descriptor layer can never drift to different gate tokens.
use crate::m5_descriptor_badge::{
    ConsumerStatus, DescriptorGate, DescriptorSignal, FreshnessState, QualificationClass,
};

/// Record-kind tag carried by [`M5AssuranceRouteGovernance`].
pub const M5_ASSURANCE_ROUTE_RECORD_KIND: &str = "m5_assurance_route_governance";

/// Schema version for the governance packet.
pub const M5_ASSURANCE_ROUTE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the governance packet schema.
pub const M5_ASSURANCE_ROUTE_SCHEMA_REF: &str =
    "schemas/release/m5-assurance-route-governance.schema.json";

/// Repo-relative path of the published governance inventory.
pub const M5_ASSURANCE_ROUTE_REF: &str =
    "artifacts/release/m5-assurance-route-governance-summary.json";

/// Repo-relative path of the rendered governance matrix document.
pub const M5_ASSURANCE_ROUTE_GOVERNANCE_REF: &str =
    "artifacts/release/m5-assurance-route-governance.md";

/// Repo-relative path of the machine-readable matrix export.
pub const M5_ASSURANCE_ROUTE_MATRIX_CSV_REF: &str =
    "artifacts/release/m5-assurance-route-matrix.csv";

/// Repo-relative path of the release-grade governance parity proof.
pub const M5_ASSURANCE_ROUTE_PROOF_REF: &str =
    "artifacts/release-proof/m5-assurance-route-governance/assurance-route-matrix.json";

/// Repo-relative path of the governance contract doc.
pub const M5_ASSURANCE_ROUTE_DOC_REF: &str =
    "docs/release/m5-assurance-route-governance-contract.md";

/// Repo-relative directory of the per-state governance fixtures.
pub const M5_ASSURANCE_ROUTE_FIXTURE_DIR: &str = "fixtures/release/m5-assurance-route/";

/// Prefix every assurance / route governance message id carries so consumers can route it.
pub const M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX: &str = "release_assurance_route_governance.";

/// One of the three governance dimensions a [facet](AssuranceFacet) belongs to. Naming the
/// dimension on every drift is what lets the matrix say *which* of claim assurance, governance
/// posture, or route / event provenance drifted rather than collapsing the cause into one flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceDimension {
    /// What a claim asserts and the proof / waiver disclosure backing it.
    ClaimAssurance,
    /// How current and owned the governing surface is: freshness, ownership, capability boundary.
    GovernancePosture,
    /// Where work went and who approved it: route hop, approval ticket, event provenance.
    RouteProvenance,
}

impl AssuranceDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ClaimAssurance,
        Self::GovernancePosture,
        Self::RouteProvenance,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimAssurance => "claim_assurance",
            Self::GovernancePosture => "governance_posture",
            Self::RouteProvenance => "route_provenance",
        }
    }

    /// Reviewer-facing dimension label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ClaimAssurance => "Claim assurance",
            Self::GovernancePosture => "Governance posture",
            Self::RouteProvenance => "Route / event provenance",
        }
    }
}

/// One governed product surface the source set treats as assurance / governance / route truth. Each
/// facet owns one proof path; binding a consumer to a facet is what makes that consumer's claim
/// depend on the facet's proof staying current and its assurance state staying governed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceFacet {
    /// The assurance card: what posture a surface claims and the evidence it binds.
    AssuranceClaim,
    /// The control-proof rows: the controls a claim asserts are proven.
    ControlProof,
    /// The exception / waiver disclosure: which controls are held under an accepted waiver.
    ExceptionWaiver,
    /// The governance / fitness dashboard freshness: how current the governing evidence is.
    GovernanceFreshness,
    /// The service-ownership / decision-right register: who owns and decides for the surface.
    ServiceOwnership,
    /// The capability-boundary inspector: the exact boundary facts a claim stays within.
    CapabilityBoundary,
    /// The route / transport-route timeline: where work went, hop by hop.
    RouteHop,
    /// The approval-ticket record: who approved a high-risk route or action.
    ApprovalTicket,
    /// The event-provenance inspector: the lineage an emitted event can be traced to.
    EventProvenance,
}

impl AssuranceFacet {
    /// Every facet, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::AssuranceClaim,
        Self::ControlProof,
        Self::ExceptionWaiver,
        Self::GovernanceFreshness,
        Self::ServiceOwnership,
        Self::CapabilityBoundary,
        Self::RouteHop,
        Self::ApprovalTicket,
        Self::EventProvenance,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssuranceClaim => "assurance_claim",
            Self::ControlProof => "control_proof",
            Self::ExceptionWaiver => "exception_waiver",
            Self::GovernanceFreshness => "governance_freshness",
            Self::ServiceOwnership => "service_ownership",
            Self::CapabilityBoundary => "capability_boundary",
            Self::RouteHop => "route_hop",
            Self::ApprovalTicket => "approval_ticket",
            Self::EventProvenance => "event_provenance",
        }
    }

    /// Reviewer-facing facet label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AssuranceClaim => "Assurance claim",
            Self::ControlProof => "Control proof",
            Self::ExceptionWaiver => "Exception / waiver",
            Self::GovernanceFreshness => "Governance freshness",
            Self::ServiceOwnership => "Service ownership",
            Self::CapabilityBoundary => "Capability boundary",
            Self::RouteHop => "Route hop",
            Self::ApprovalTicket => "Approval ticket",
            Self::EventProvenance => "Event provenance",
        }
    }

    /// The dimension this facet belongs to.
    pub const fn dimension(self) -> AssuranceDimension {
        match self {
            Self::AssuranceClaim | Self::ControlProof | Self::ExceptionWaiver => {
                AssuranceDimension::ClaimAssurance
            }
            Self::GovernanceFreshness | Self::ServiceOwnership | Self::CapabilityBoundary => {
                AssuranceDimension::GovernancePosture
            }
            Self::RouteHop | Self::ApprovalTicket | Self::EventProvenance => {
                AssuranceDimension::RouteProvenance
            }
        }
    }

    /// The canonical state family that governs this facet.
    pub const fn state_family(self) -> AssuranceStateFamily {
        match self {
            Self::AssuranceClaim | Self::ControlProof => AssuranceStateFamily::AssuranceClaim,
            Self::ExceptionWaiver | Self::GovernanceFreshness | Self::ServiceOwnership => {
                AssuranceStateFamily::Governance
            }
            Self::CapabilityBoundary => AssuranceStateFamily::CapabilityBoundary,
            Self::RouteHop => AssuranceStateFamily::RouteHop,
            Self::ApprovalTicket => AssuranceStateFamily::Approval,
            Self::EventProvenance => AssuranceStateFamily::Provenance,
        }
    }

    /// Repo-relative release-grade proof path that keeps this facet current.
    pub const fn proof_ref(self) -> &'static str {
        match self {
            Self::AssuranceClaim => {
                "artifacts/release-proof/m5-assurance-route-governance/assurance-claim.json"
            }
            Self::ControlProof => {
                "artifacts/release-proof/m5-assurance-route-governance/control-proof.json"
            }
            Self::ExceptionWaiver => {
                "artifacts/release-proof/m5-assurance-route-governance/exception-waiver.json"
            }
            Self::GovernanceFreshness => {
                "artifacts/release-proof/m5-assurance-route-governance/governance-freshness.json"
            }
            Self::ServiceOwnership => {
                "artifacts/release-proof/m5-assurance-route-governance/service-ownership.json"
            }
            Self::CapabilityBoundary => {
                "artifacts/release-proof/m5-assurance-route-governance/capability-boundary.json"
            }
            Self::RouteHop => {
                "artifacts/release-proof/m5-assurance-route-governance/route-hop.json"
            }
            Self::ApprovalTicket => {
                "artifacts/release-proof/m5-assurance-route-governance/approval-ticket.json"
            }
            Self::EventProvenance => {
                "artifacts/release-proof/m5-assurance-route-governance/event-provenance.json"
            }
        }
    }

    /// Owner role accountable for keeping this facet's proof current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::AssuranceClaim => "assurance_center_owner",
            Self::ControlProof => "control_proof_owner",
            Self::ExceptionWaiver => "exception_waiver_owner",
            Self::GovernanceFreshness => "governance_dashboard_owner",
            Self::ServiceOwnership => "service_ownership_owner",
            Self::CapabilityBoundary => "capability_boundary_owner",
            Self::RouteHop => "route_explainability_owner",
            Self::ApprovalTicket => "approval_authority_owner",
            Self::EventProvenance => "event_provenance_owner",
        }
    }
}

/// One canonical assurance state family. Each family is an ordered, gate-bound vocabulary reused
/// across the governed surfaces so an assurance / governance / route state is never restated as an
/// ad hoc label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceStateFamily {
    /// How strongly a claim or control is proven.
    AssuranceClaim,
    /// How a governance / fitness dashboard cell reads: pass, stale, waived, blocked.
    Governance,
    /// Where a claim sits relative to its declared capability boundary.
    CapabilityBoundary,
    /// How attributable a route / transport hop is.
    RouteHop,
    /// How authorized a high-risk route or action is.
    Approval,
    /// How traceable an emitted event's provenance is.
    Provenance,
}

impl AssuranceStateFamily {
    /// Every family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AssuranceClaim,
        Self::Governance,
        Self::CapabilityBoundary,
        Self::RouteHop,
        Self::Approval,
        Self::Provenance,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssuranceClaim => "assurance_claim",
            Self::Governance => "governance",
            Self::CapabilityBoundary => "capability_boundary",
            Self::RouteHop => "route_hop",
            Self::Approval => "approval",
            Self::Provenance => "provenance",
        }
    }

    /// Reviewer-facing family label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AssuranceClaim => "Assurance-claim state",
            Self::Governance => "Governance state",
            Self::CapabilityBoundary => "Capability-boundary state",
            Self::RouteHop => "Route-hop state",
            Self::Approval => "Approval state",
            Self::Provenance => "Provenance state",
        }
    }

    /// The ordered canonical state tokens of this family, each bound to a gate posture and floor.
    pub fn state_defs(self) -> Vec<AssuranceStateTokenDef> {
        match self {
            Self::AssuranceClaim => AssuranceClaimState::ALL
                .iter()
                .map(|s| s.to_def())
                .collect(),
            Self::Governance => GovernanceState::ALL.iter().map(|s| s.to_def()).collect(),
            Self::CapabilityBoundary => CapabilityBoundaryState::ALL
                .iter()
                .map(|s| s.to_def())
                .collect(),
            Self::RouteHop => RouteHopState::ALL.iter().map(|s| s.to_def()).collect(),
            Self::Approval => ApprovalState::ALL.iter().map(|s| s.to_def()).collect(),
            Self::Provenance => ProvenanceState::ALL.iter().map(|s| s.to_def()).collect(),
        }
    }

    /// True when `token` is a member of this family.
    pub fn contains_token(self, token: &str) -> bool {
        self.state_defs().iter().any(|d| d.token == token)
    }
}

/// Maps a gate posture to the effective qualification floor it implies, so a state's posture and
/// floor can never disagree: governed stands at Stable, narrowed floors at Beta, blocked at
/// Unavailable.
const fn floor_for_posture(posture: DescriptorGate) -> QualificationClass {
    match posture {
        DescriptorGate::Governed => QualificationClass::Stable,
        DescriptorGate::Narrowed => QualificationClass::Beta,
        DescriptorGate::Blocked => QualificationClass::Unavailable,
    }
}

/// Builds a state token definition from a stable token, label, and gate posture.
fn state_def(token: &str, label: &str, posture: DescriptorGate) -> AssuranceStateTokenDef {
    AssuranceStateTokenDef {
        token: token.to_owned(),
        label: label.to_owned(),
        gate_posture: posture,
        effective_floor: floor_for_posture(posture),
        message_id: format!("{M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX}state.{token}"),
    }
}

/// Canonical assurance-claim state vocabulary (most→least assured).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceClaimState {
    /// The claim is proven by fresh, bound evidence.
    Proven,
    /// The claim is attested by signed evidence; still governed.
    Attested,
    /// The claim is under review; it narrows until review completes.
    UnderReview,
    /// A control is held by a pending exception; the claim narrows.
    ExceptionPending,
    /// The claim is unproven; Stable promotion is held.
    Unproven,
}

impl AssuranceClaimState {
    /// Every assurance-claim state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Proven,
        Self::Attested,
        Self::UnderReview,
        Self::ExceptionPending,
        Self::Unproven,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proven => "proven",
            Self::Attested => "attested",
            Self::UnderReview => "under_review",
            Self::ExceptionPending => "exception_pending",
            Self::Unproven => "unproven",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Proven => "Proven",
            Self::Attested => "Attested",
            Self::UnderReview => "Under review",
            Self::ExceptionPending => "Exception pending",
            Self::Unproven => "Unproven",
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::Proven | Self::Attested => DescriptorGate::Governed,
            Self::UnderReview | Self::ExceptionPending => DescriptorGate::Narrowed,
            Self::Unproven => DescriptorGate::Blocked,
        }
    }

    fn to_def(self) -> AssuranceStateTokenDef {
        state_def(self.as_str(), self.label(), self.gate_posture())
    }
}

/// Canonical governance / fitness-dashboard state vocabulary (most→least healthy). The frozen
/// `pass` / `stale` / `waived` / `blocked` postures a governance dashboard distinguishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceState {
    /// The fitness function passes on fresh evidence.
    Pass,
    /// The fitness function passes and is actively monitored; still governed.
    Monitored,
    /// The governing evidence aged out; the claim narrows.
    Stale,
    /// The cell is held by an accepted waiver; the claim narrows.
    Waived,
    /// The fitness function failed; Stable promotion is held.
    Blocked,
}

impl GovernanceState {
    /// Every governance state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Pass,
        Self::Monitored,
        Self::Stale,
        Self::Waived,
        Self::Blocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Monitored => "monitored",
            Self::Stale => "stale",
            Self::Waived => "waived",
            Self::Blocked => "blocked",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pass => "Pass",
            Self::Monitored => "Monitored",
            Self::Stale => "Stale",
            Self::Waived => "Waived",
            Self::Blocked => "Blocked",
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::Pass | Self::Monitored => DescriptorGate::Governed,
            Self::Stale | Self::Waived => DescriptorGate::Narrowed,
            Self::Blocked => DescriptorGate::Blocked,
        }
    }

    fn to_def(self) -> AssuranceStateTokenDef {
        state_def(self.as_str(), self.label(), self.gate_posture())
    }
}

/// Canonical capability-boundary state vocabulary (most→least within boundary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityBoundaryState {
    /// The claim sits well within its declared boundary.
    WithinBoundary,
    /// The claim sits within a documented boundary edge; still governed.
    BoundaryDocumented,
    /// The claim sits at the boundary edge; it narrows.
    AtBoundaryEdge,
    /// The boundary has been narrowed below the claim; it narrows.
    BoundaryNarrowed,
    /// The claim falls outside its boundary; Stable promotion is held.
    OutsideBoundary,
}

impl CapabilityBoundaryState {
    /// Every capability-boundary state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::WithinBoundary,
        Self::BoundaryDocumented,
        Self::AtBoundaryEdge,
        Self::BoundaryNarrowed,
        Self::OutsideBoundary,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinBoundary => "within_boundary",
            Self::BoundaryDocumented => "boundary_documented",
            Self::AtBoundaryEdge => "at_boundary_edge",
            Self::BoundaryNarrowed => "boundary_narrowed",
            Self::OutsideBoundary => "outside_boundary",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WithinBoundary => "Within boundary",
            Self::BoundaryDocumented => "Boundary documented",
            Self::AtBoundaryEdge => "At boundary edge",
            Self::BoundaryNarrowed => "Boundary narrowed",
            Self::OutsideBoundary => "Outside boundary",
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::WithinBoundary | Self::BoundaryDocumented => DescriptorGate::Governed,
            Self::AtBoundaryEdge | Self::BoundaryNarrowed => DescriptorGate::Narrowed,
            Self::OutsideBoundary => DescriptorGate::Blocked,
        }
    }

    fn to_def(self) -> AssuranceStateTokenDef {
        state_def(self.as_str(), self.label(), self.gate_posture())
    }
}

/// Canonical route-hop state vocabulary (most→least attributable).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteHopState {
    /// Work stayed on the local machine; fully explainable.
    LocalOnly,
    /// Work went to a named remote target, fully attributed; still governed.
    AttributedRemote,
    /// Work was served via a labelled mirror; the claim narrows.
    MirroredRoute,
    /// The route is degraded / partially attributed; the claim narrows.
    RouteDegraded,
    /// The route cannot be attributed; Stable promotion is held.
    UnattributedRoute,
}

impl RouteHopState {
    /// Every route-hop state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOnly,
        Self::AttributedRemote,
        Self::MirroredRoute,
        Self::RouteDegraded,
        Self::UnattributedRoute,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::AttributedRemote => "attributed_remote",
            Self::MirroredRoute => "mirrored_route",
            Self::RouteDegraded => "route_degraded",
            Self::UnattributedRoute => "unattributed_route",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalOnly => "Local only",
            Self::AttributedRemote => "Attributed remote",
            Self::MirroredRoute => "Mirrored route",
            Self::RouteDegraded => "Route degraded",
            Self::UnattributedRoute => "Unattributed route",
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::LocalOnly | Self::AttributedRemote => DescriptorGate::Governed,
            Self::MirroredRoute | Self::RouteDegraded => DescriptorGate::Narrowed,
            Self::UnattributedRoute => DescriptorGate::Blocked,
        }
    }

    fn to_def(self) -> AssuranceStateTokenDef {
        state_def(self.as_str(), self.label(), self.gate_posture())
    }
}

/// Canonical approval state vocabulary (most→least authorized).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalState {
    /// The action is pre-authorized by standing policy.
    PreAuthorized,
    /// The action was approved by a named approver; still governed.
    Approved,
    /// Approval is pending; the claim narrows until it lands.
    ApprovalPending,
    /// Approval is required before the action proceeds; the claim narrows.
    ApprovalRequired,
    /// Approval was denied; Stable promotion is held.
    ApprovalDenied,
}

impl ApprovalState {
    /// Every approval state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PreAuthorized,
        Self::Approved,
        Self::ApprovalPending,
        Self::ApprovalRequired,
        Self::ApprovalDenied,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreAuthorized => "pre_authorized",
            Self::Approved => "approved",
            Self::ApprovalPending => "approval_pending",
            Self::ApprovalRequired => "approval_required",
            Self::ApprovalDenied => "approval_denied",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PreAuthorized => "Pre-authorized",
            Self::Approved => "Approved",
            Self::ApprovalPending => "Approval pending",
            Self::ApprovalRequired => "Approval required",
            Self::ApprovalDenied => "Approval denied",
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::PreAuthorized | Self::Approved => DescriptorGate::Governed,
            Self::ApprovalPending | Self::ApprovalRequired => DescriptorGate::Narrowed,
            Self::ApprovalDenied => DescriptorGate::Blocked,
        }
    }

    fn to_def(self) -> AssuranceStateTokenDef {
        state_def(self.as_str(), self.label(), self.gate_posture())
    }
}

/// Canonical provenance state vocabulary (most→least traceable).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceState {
    /// The event traces to a fully recorded lineage.
    FullyTraced,
    /// The event is a derived output that traces to its source; still governed.
    DerivedTraced,
    /// Only partial provenance is recorded; the claim narrows.
    PartialProvenance,
    /// The provenance ledger aged out; the claim narrows.
    ProvenanceStale,
    /// No provenance is recorded; Stable promotion is held.
    ProvenanceMissing,
}

impl ProvenanceState {
    /// Every provenance state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullyTraced,
        Self::DerivedTraced,
        Self::PartialProvenance,
        Self::ProvenanceStale,
        Self::ProvenanceMissing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyTraced => "fully_traced",
            Self::DerivedTraced => "derived_traced",
            Self::PartialProvenance => "partial_provenance",
            Self::ProvenanceStale => "provenance_stale",
            Self::ProvenanceMissing => "provenance_missing",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullyTraced => "Fully traced",
            Self::DerivedTraced => "Derived, traced",
            Self::PartialProvenance => "Partial provenance",
            Self::ProvenanceStale => "Provenance stale",
            Self::ProvenanceMissing => "Provenance missing",
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::FullyTraced | Self::DerivedTraced => DescriptorGate::Governed,
            Self::PartialProvenance | Self::ProvenanceStale => DescriptorGate::Narrowed,
            Self::ProvenanceMissing => DescriptorGate::Blocked,
        }
    }

    fn to_def(self) -> AssuranceStateTokenDef {
        state_def(self.as_str(), self.label(), self.gate_posture())
    }
}

/// A typed current assurance state assigned to a facet — one value drawn from the facet's state
/// family. The matrix uses it to bind the facet to a gate posture and qualification floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalState {
    /// An assurance-claim state.
    AssuranceClaim(AssuranceClaimState),
    /// A governance state.
    Governance(GovernanceState),
    /// A capability-boundary state.
    CapabilityBoundary(CapabilityBoundaryState),
    /// A route-hop state.
    RouteHop(RouteHopState),
    /// An approval state.
    Approval(ApprovalState),
    /// A provenance state.
    Provenance(ProvenanceState),
}

impl CanonicalState {
    /// The family this state belongs to.
    pub const fn family(self) -> AssuranceStateFamily {
        match self {
            Self::AssuranceClaim(_) => AssuranceStateFamily::AssuranceClaim,
            Self::Governance(_) => AssuranceStateFamily::Governance,
            Self::CapabilityBoundary(_) => AssuranceStateFamily::CapabilityBoundary,
            Self::RouteHop(_) => AssuranceStateFamily::RouteHop,
            Self::Approval(_) => AssuranceStateFamily::Approval,
            Self::Provenance(_) => AssuranceStateFamily::Provenance,
        }
    }

    /// Stable token recorded in the packet.
    pub const fn token(self) -> &'static str {
        match self {
            Self::AssuranceClaim(s) => s.as_str(),
            Self::Governance(s) => s.as_str(),
            Self::CapabilityBoundary(s) => s.as_str(),
            Self::RouteHop(s) => s.as_str(),
            Self::Approval(s) => s.as_str(),
            Self::Provenance(s) => s.as_str(),
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AssuranceClaim(s) => s.label(),
            Self::Governance(s) => s.label(),
            Self::CapabilityBoundary(s) => s.label(),
            Self::RouteHop(s) => s.label(),
            Self::Approval(s) => s.label(),
            Self::Provenance(s) => s.label(),
        }
    }

    /// Gate posture this state binds to.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::AssuranceClaim(s) => s.gate_posture(),
            Self::Governance(s) => s.gate_posture(),
            Self::CapabilityBoundary(s) => s.gate_posture(),
            Self::RouteHop(s) => s.gate_posture(),
            Self::Approval(s) => s.gate_posture(),
            Self::Provenance(s) => s.gate_posture(),
        }
    }
}

/// One evidence class a facet binds its assurance to. The set reuses the existing lower-level
/// publication / approval / provenance lanes rather than inventing new evidence families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    /// A signed control attestation.
    ControlAttestation,
    /// A policy / governance bundle.
    PolicyBundle,
    /// A runtime-approval record.
    RuntimeApprovalRecord,
    /// A route / transport-route timeline.
    RouteTimeline,
    /// An event-provenance ledger.
    ProvenanceLedger,
    /// A service-ownership / decision-right register.
    OwnershipRegister,
    /// An exception / waiver record.
    WaiverRecord,
    /// A capability-boundary manifest.
    BoundaryManifest,
}

impl EvidenceClass {
    /// Every evidence class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ControlAttestation,
        Self::PolicyBundle,
        Self::RuntimeApprovalRecord,
        Self::RouteTimeline,
        Self::ProvenanceLedger,
        Self::OwnershipRegister,
        Self::WaiverRecord,
        Self::BoundaryManifest,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlAttestation => "control_attestation",
            Self::PolicyBundle => "policy_bundle",
            Self::RuntimeApprovalRecord => "runtime_approval_record",
            Self::RouteTimeline => "route_timeline",
            Self::ProvenanceLedger => "provenance_ledger",
            Self::OwnershipRegister => "ownership_register",
            Self::WaiverRecord => "waiver_record",
            Self::BoundaryManifest => "boundary_manifest",
        }
    }
}

/// One claimed M5 posture line the matrix scopes to. The set is the regulated / sovereign /
/// self-hosted / managed lines the exit-gate anchor names; this lane does not invent new posture
/// families or compliance frameworks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedPosture {
    /// The managed (vendor-operated control plane) posture.
    Managed,
    /// The self-hosted posture.
    SelfHosted,
    /// The regulated posture.
    Regulated,
    /// The sovereign / air-gapped posture.
    Sovereign,
}

impl ClaimedPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Managed,
        Self::SelfHosted,
        Self::Regulated,
        Self::Sovereign,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Managed => "managed",
            Self::SelfHosted => "self_hosted",
            Self::Regulated => "regulated",
            Self::Sovereign => "sovereign",
        }
    }
}

/// One trust boundary a facet covers. Keeping `local_first` and `control_plane` distinct is what
/// lets the matrix state local-only continuity explicitly so a managed or vendor outage never
/// implies local inspection is unsafe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustBoundary {
    /// The local-first boundary: evidence that stays on the local machine.
    LocalFirst,
    /// The control-plane boundary: evidence that crosses to a managed / remote control plane.
    ControlPlane,
}

impl TrustBoundary {
    /// Every boundary, in declaration order.
    pub const ALL: [Self; 2] = [Self::LocalFirst, Self::ControlPlane];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFirst => "local_first",
            Self::ControlPlane => "control_plane",
        }
    }
}

/// How a facet behaves under stale, mirrored, or no-live-data conditions. Every behavior keeps the
/// surface local-safe: it labels the weaker state rather than dropping it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedDataBehavior {
    /// The surface is showing live, attested data.
    LiveAttested,
    /// The surface is showing mirrored data, labelled as mirrored.
    MirroredLabelled,
    /// The surface is showing offline-cached data, labelled as offline.
    OfflineCached,
    /// The surface is showing data behind a stale banner.
    StaleBannerShown,
    /// No live data is reachable; the surface shows only the locally recorded lineage.
    LocalLineageOnly,
}

impl DegradedDataBehavior {
    /// Every behavior, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveAttested,
        Self::MirroredLabelled,
        Self::OfflineCached,
        Self::StaleBannerShown,
        Self::LocalLineageOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveAttested => "live_attested",
            Self::MirroredLabelled => "mirrored_labelled",
            Self::OfflineCached => "offline_cached",
            Self::StaleBannerShown => "stale_banner_shown",
            Self::LocalLineageOnly => "local_lineage_only",
        }
    }
}

/// The kind of coverage gap on a consumer's read facet: a proof-currency gap or an assurance-state
/// gap. Naming the kind is what lets the matrix fail proof *or* assurance coverage rather than
/// leaving it implied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceGapKind {
    /// The consumer reads a facet the matrix does not govern.
    FacetUngoverned,
    /// A read facet's proof is stale (narrows).
    ProofStale,
    /// A read facet's proof is expired (blocks).
    ProofExpired,
    /// A read facet's proof is missing (blocks).
    ProofMissing,
    /// A read facet's current assurance state itself narrows the claim.
    AssuranceStateNarrowed,
    /// A read facet's current assurance state itself blocks the claim.
    AssuranceStateBlocked,
}

impl AssuranceGapKind {
    /// Every gap kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FacetUngoverned,
        Self::ProofStale,
        Self::ProofExpired,
        Self::ProofMissing,
        Self::AssuranceStateNarrowed,
        Self::AssuranceStateBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetUngoverned => "facet_ungoverned",
            Self::ProofStale => "proof_stale",
            Self::ProofExpired => "proof_expired",
            Self::ProofMissing => "proof_missing",
            Self::AssuranceStateNarrowed => "assurance_state_narrowed",
            Self::AssuranceStateBlocked => "assurance_state_blocked",
        }
    }

    /// True when this gap blocks Stable promotion (vs only narrowing it).
    pub const fn is_blocking(self) -> bool {
        matches!(
            self,
            Self::FacetUngoverned
                | Self::ProofExpired
                | Self::ProofMissing
                | Self::AssuranceStateBlocked
        )
    }
}

/// One canonical assurance state token definition, bound to a gate posture and a qualification
/// floor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceStateTokenDef {
    /// Stable token.
    pub token: String,
    /// Reviewer-facing label.
    pub label: String,
    /// Gate posture this state binds to.
    pub gate_posture: DescriptorGate,
    /// Effective qualification floor implied by the posture.
    pub effective_floor: QualificationClass,
    /// Stable message id; prefixed [`M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX`].
    pub message_id: String,
}

/// One canonical assurance state family with its ordered, gate-bound token set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceStateFamilyDef {
    /// The family.
    pub family: AssuranceStateFamily,
    /// Reviewer-facing family label.
    pub family_label: String,
    /// The ordered state tokens.
    pub states: Vec<AssuranceStateTokenDef>,
}

impl AssuranceStateFamilyDef {
    /// Builds the family definition from the typed family.
    pub fn for_family(family: AssuranceStateFamily) -> Self {
        Self {
            family,
            family_label: family.label().to_owned(),
            states: family.state_defs(),
        }
    }

    /// Validates the family's internal invariants: every token binds a posture-consistent floor and
    /// carries a prefixed message id.
    fn validate(&self) -> Vec<M5AssuranceRouteViolation> {
        let mut out = Vec::new();
        if self.family_label != self.family.label() || self.states != self.family.state_defs() {
            out.push(M5AssuranceRouteViolation::StateFamilyDrift);
        }
        for state in &self.states {
            if state.effective_floor != floor_for_posture(state.gate_posture)
                || !state
                    .message_id
                    .starts_with(M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX)
            {
                out.push(M5AssuranceRouteViolation::StateBindingInvalid);
            }
        }
        out
    }
}

/// The canonical state-family definitions, in family order.
pub fn canonical_state_families() -> Vec<AssuranceStateFamilyDef> {
    AssuranceStateFamily::ALL
        .iter()
        .map(|f| AssuranceStateFamilyDef::for_family(*f))
        .collect()
}

/// One governed assurance facet row: its dimension, the state family that governs it, its current
/// canonical state, the evidence classes / postures / trust boundaries it discloses, its
/// degraded-data behavior, the proof path and freshness backing it, and the status that proof
/// implies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceFacetRow {
    /// The governed facet.
    pub facet: AssuranceFacet,
    /// Reviewer-facing facet label.
    pub facet_label: String,
    /// The dimension this facet belongs to.
    pub dimension: AssuranceDimension,
    /// The state family that governs this facet.
    pub state_family: AssuranceStateFamily,
    /// The facet's current canonical state token (a member of [`Self::state_family`]).
    pub current_state_token: String,
    /// Reviewer-facing current-state label.
    pub current_state_label: String,
    /// Gate posture the current state binds to.
    pub state_gate: DescriptorGate,
    /// Effective qualification floor implied by the current state.
    pub state_floor: QualificationClass,
    /// The evidence classes this facet binds.
    pub evidence_classes: Vec<EvidenceClass>,
    /// The claimed posture lines this facet scopes to.
    pub claimed_postures: Vec<ClaimedPosture>,
    /// The trust boundaries this facet covers.
    pub trust_boundaries: Vec<TrustBoundary>,
    /// How this facet behaves under stale / mirrored / no-live-data conditions.
    pub degraded_data_behavior: DegradedDataBehavior,
    /// Owner role accountable for keeping the proof current.
    pub owner_role: String,
    /// Repo-relative release-grade proof path.
    pub proof_ref: String,
    /// Freshness of the facet's proof.
    pub proof_freshness: FreshnessState,
    /// Coverage status implied by the proof freshness.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl AssuranceFacetRow {
    /// Builds a facet row at a given proof freshness and current state, deriving every field from
    /// the facet so a row can never cite a field that drifts from it.
    pub fn new(
        facet: AssuranceFacet,
        state: CanonicalState,
        proof_freshness: FreshnessState,
        evidence_classes: &[EvidenceClass],
        claimed_postures: &[ClaimedPosture],
        trust_boundaries: &[TrustBoundary],
        degraded_data_behavior: DegradedDataBehavior,
    ) -> Self {
        let status = proof_status(proof_freshness);
        Self {
            facet,
            facet_label: facet.label().to_owned(),
            dimension: facet.dimension(),
            state_family: facet.state_family(),
            current_state_token: state.token().to_owned(),
            current_state_label: state.label().to_owned(),
            state_gate: state.gate_posture(),
            state_floor: floor_for_posture(state.gate_posture()),
            evidence_classes: evidence_classes.to_vec(),
            claimed_postures: claimed_postures.to_vec(),
            trust_boundaries: trust_boundaries.to_vec(),
            degraded_data_behavior,
            owner_role: facet.owner_role().to_owned(),
            proof_ref: facet.proof_ref().to_owned(),
            proof_freshness,
            status,
            signal: status.signal(),
            detail_message_id: format!(
                "{}facet.{}",
                M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX,
                facet.as_str()
            ),
        }
    }

    /// The proof-currency gap kind a consumer that reads this facet inherits, if any.
    fn proof_gap_kind(&self) -> Option<AssuranceGapKind> {
        match self.proof_freshness {
            FreshnessState::Current => None,
            FreshnessState::Stale => Some(AssuranceGapKind::ProofStale),
            FreshnessState::Expired => Some(AssuranceGapKind::ProofExpired),
            FreshnessState::Missing => Some(AssuranceGapKind::ProofMissing),
        }
    }

    /// The assurance-state gap kind a consumer that reads this facet inherits, if any.
    fn state_gap_kind(&self) -> Option<AssuranceGapKind> {
        match self.state_gate {
            DescriptorGate::Governed => None,
            DescriptorGate::Narrowed => Some(AssuranceGapKind::AssuranceStateNarrowed),
            DescriptorGate::Blocked => Some(AssuranceGapKind::AssuranceStateBlocked),
        }
    }

    /// Validates the row's invariants: every derived field matches the facet, the current state is
    /// a member of its family, the status mirrors the proof freshness, and the message id carries
    /// the lane prefix.
    fn validate(&self) -> Vec<M5AssuranceRouteViolation> {
        let mut out = Vec::new();
        if self.facet_label != self.facet.label()
            || self.dimension != self.facet.dimension()
            || self.state_family != self.facet.state_family()
            || self.owner_role != self.facet.owner_role()
            || self.proof_ref != self.facet.proof_ref()
        {
            out.push(M5AssuranceRouteViolation::FacetFieldMismatch);
        }
        if !self.state_family.contains_token(&self.current_state_token)
            || self.state_floor != floor_for_posture(self.state_gate)
        {
            out.push(M5AssuranceRouteViolation::FacetStateInvalid);
        }
        if self.evidence_classes.is_empty()
            || self.claimed_postures.is_empty()
            || self.trust_boundaries.is_empty()
        {
            out.push(M5AssuranceRouteViolation::FacetDisclosureEmpty);
        }
        let status = proof_status(self.proof_freshness);
        if self.status != status || self.signal != status.signal() {
            out.push(M5AssuranceRouteViolation::FacetStatusDrift);
        }
        if !self
            .detail_message_id
            .starts_with(M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX)
        {
            out.push(M5AssuranceRouteViolation::UnprefixedMessageId);
        }
        out
    }
}

/// Maps a proof freshness to the coverage status it implies: current is mapped, stale is
/// provisional (narrowed), expired / missing is unmapped (blocked).
fn proof_status(freshness: FreshnessState) -> ConsumerStatus {
    match freshness {
        FreshnessState::Current => ConsumerStatus::Mapped,
        FreshnessState::Stale => ConsumerStatus::Provisional,
        FreshnessState::Expired | FreshnessState::Missing => ConsumerStatus::Unmapped,
    }
}

/// One coverage gap on a claimed consumer: a facet it reads whose proof drifted, whose assurance
/// state narrows or blocks, or that the matrix does not govern at all.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceGap {
    /// Consumer this gap applies to.
    pub consumer: AssuranceConsumer,
    /// The facet the gap concerns.
    pub facet: AssuranceFacet,
    /// The dimension that drifted.
    pub dimension: AssuranceDimension,
    /// The kind of gap.
    pub gap_kind: AssuranceGapKind,
    /// Stable message id; prefixed [`M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

/// One claimed M5 assurance / governance / route-inspection consumer surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceConsumer {
    /// The assurance center.
    AssuranceCenter,
    /// The governance / fitness dashboard.
    GovernanceDashboard,
    /// The capability-boundary inspector.
    CapabilityInspector,
    /// The route / event-provenance inspector.
    RouteInspector,
    /// The admin console / decision-right governance.
    AdminConsole,
    /// The About / help surface.
    HelpAbout,
    /// The procurement / evaluation pack.
    ProcurementEvaluation,
    /// Support exports / bundles.
    SupportExport,
}

impl AssuranceConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::AssuranceCenter,
        Self::GovernanceDashboard,
        Self::CapabilityInspector,
        Self::RouteInspector,
        Self::AdminConsole,
        Self::HelpAbout,
        Self::ProcurementEvaluation,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssuranceCenter => "assurance_center",
            Self::GovernanceDashboard => "governance_dashboard",
            Self::CapabilityInspector => "capability_inspector",
            Self::RouteInspector => "route_inspector",
            Self::AdminConsole => "admin_console",
            Self::HelpAbout => "help_about",
            Self::ProcurementEvaluation => "procurement_evaluation",
            Self::SupportExport => "support_export",
        }
    }

    /// Reviewer-facing consumer label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AssuranceCenter => "Assurance center",
            Self::GovernanceDashboard => "Governance dashboard",
            Self::CapabilityInspector => "Capability-boundary inspector",
            Self::RouteInspector => "Route / provenance inspector",
            Self::AdminConsole => "Admin console",
            Self::HelpAbout => "About / help",
            Self::ProcurementEvaluation => "Procurement / evaluation",
            Self::SupportExport => "Support export",
        }
    }

    /// Owner role accountable for keeping this consumer's binding current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::AssuranceCenter => "assurance_center_owner",
            Self::GovernanceDashboard => "governance_dashboard_owner",
            Self::CapabilityInspector => "capability_inspector_owner",
            Self::RouteInspector => "route_inspector_owner",
            Self::AdminConsole => "admin_console_owner",
            Self::HelpAbout => "help_about_owner",
            Self::ProcurementEvaluation => "procurement_owner",
            Self::SupportExport => "support_export_owner",
        }
    }
}

/// Derived verdict for a consumer, computed from its gaps.
struct ConsumerVerdict {
    status: ConsumerStatus,
    signal: DescriptorSignal,
    gate: DescriptorGate,
    effective_qualification: QualificationClass,
}

/// Restrictiveness rank of a qualification class (least restrictive first).
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

/// Derives a consumer's verdict from its gaps: any blocking gap blocks Stable; any narrowing gap
/// narrows to at least Beta; an ungapped consumer stands at its claim.
fn derive_consumer_verdict(claimed: QualificationClass, gaps: &[AssuranceGap]) -> ConsumerVerdict {
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

/// One claimed consumer surface certified against the governed facets: the facets it reads, the
/// union of evidence classes / postures / trust boundaries those facets disclose, the proof paths
/// backing them, the per-consumer gaps, and the verdict derived from those facets' proof freshness
/// and assurance state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceConsumerRow {
    /// The consumer surface.
    pub consumer: AssuranceConsumer,
    /// Reviewer-facing consumer label.
    pub consumer_label: String,
    /// Owner role accountable for keeping this consumer's binding current.
    pub owner_role: String,
    /// Public qualification the consumer wants to keep.
    pub claimed_qualification: QualificationClass,
    /// The facets this consumer reads.
    pub read_facets: Vec<AssuranceFacet>,
    /// The dimensions this consumer's read facets cover, in dimension order.
    pub covered_dimensions: Vec<AssuranceDimension>,
    /// The union of evidence classes the read facets disclose, in class order.
    pub disclosed_evidence_classes: Vec<EvidenceClass>,
    /// The union of claimed postures the read facets scope to, in posture order.
    pub claimed_postures: Vec<ClaimedPosture>,
    /// The union of trust boundaries the read facets cover, in boundary order.
    pub trust_boundaries: Vec<TrustBoundary>,
    /// The proof paths backing the read facets — refs only.
    pub proof_refs: Vec<String>,
    /// Effective qualification after the gate applies.
    pub effective_qualification: QualificationClass,
    /// Coverage status.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Gate decision the release automation reads.
    pub gate_decision: DescriptorGate,
    /// Exact coverage gaps for this consumer.
    pub gaps: Vec<AssuranceGap>,
    /// Stable message id for the status; prefixed [`M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
    /// Stable message id for the gate; prefixed [`M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

impl AssuranceConsumerRow {
    /// Builds a consumer row from its claimed qualification and the facets it reads; the resolved
    /// unions, gaps, and verdict are recomputed against the packet's facet rows when the packet is
    /// assembled.
    pub fn new(
        consumer: AssuranceConsumer,
        claimed_qualification: QualificationClass,
        read_facets: &[AssuranceFacet],
    ) -> Self {
        Self {
            consumer,
            consumer_label: consumer.label().to_owned(),
            owner_role: consumer.owner_role().to_owned(),
            claimed_qualification,
            read_facets: read_facets.to_vec(),
            covered_dimensions: Vec::new(),
            disclosed_evidence_classes: Vec::new(),
            claimed_postures: Vec::new(),
            trust_boundaries: Vec::new(),
            proof_refs: Vec::new(),
            effective_qualification: claimed_qualification,
            status: ConsumerStatus::Mapped,
            signal: DescriptorSignal::Green,
            gate_decision: DescriptorGate::Governed,
            gaps: Vec::new(),
            status_message_id: format!(
                "{}consumer.{}.status",
                M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
            gate_message_id: format!(
                "{}consumer.{}.gate",
                M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
        }
    }

    /// Recomputes the resolved unions, gaps, and verdict from the packet's facet rows, so a
    /// consumer's claim is always generated from the same checked-in proofs and assurance states
    /// the packet ships rather than a hand-maintained status.
    pub fn recompute(&mut self, facets: &[AssuranceFacetRow]) {
        let mut read = self.read_facets.clone();
        read.sort_by_key(facet_rank);
        read.dedup();
        self.read_facets = read.clone();

        let row_for = |facet: AssuranceFacet| -> Option<&AssuranceFacetRow> {
            facets.iter().find(|r| r.facet == facet)
        };

        // Union the disclosures across the read facets, canonically ordered and deduped.
        let mut dimensions: Vec<AssuranceDimension> = Vec::new();
        let mut evidence_classes: Vec<EvidenceClass> = Vec::new();
        let mut postures: Vec<ClaimedPosture> = Vec::new();
        let mut boundaries: Vec<TrustBoundary> = Vec::new();
        let mut proof_refs: Vec<String> = Vec::new();
        for &facet in &read {
            dimensions.push(facet.dimension());
            proof_refs.push(
                row_for(facet)
                    .map(|r| r.proof_ref.clone())
                    .unwrap_or_else(|| facet.proof_ref().to_owned()),
            );
            if let Some(row) = row_for(facet) {
                evidence_classes.extend(row.evidence_classes.iter().copied());
                postures.extend(row.claimed_postures.iter().copied());
                boundaries.extend(row.trust_boundaries.iter().copied());
            }
        }
        dimensions.sort_by_key(|d| dimension_rank(*d));
        dimensions.dedup();
        evidence_classes.sort_by_key(|c| evidence_rank(*c));
        evidence_classes.dedup();
        postures.sort_by_key(|p| posture_rank(*p));
        postures.dedup();
        boundaries.sort_by_key(|b| boundary_rank(*b));
        boundaries.dedup();
        self.covered_dimensions = dimensions;
        self.disclosed_evidence_classes = evidence_classes;
        self.claimed_postures = postures;
        self.trust_boundaries = boundaries;
        self.proof_refs = proof_refs;

        // Derive the coverage gaps from each read facet's proof currency and assurance state.
        let consumer = self.consumer;
        let mut gaps = Vec::new();
        let mut push_gap = |facet: AssuranceFacet, kind: AssuranceGapKind| {
            gaps.push(AssuranceGap {
                consumer,
                facet,
                dimension: facet.dimension(),
                gap_kind: kind,
                cause_message_id: format!(
                    "{}consumer.{}.{}.{}.gap",
                    M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX,
                    consumer.as_str(),
                    facet.as_str(),
                    kind.as_str()
                ),
            });
        };
        for &facet in &read {
            match row_for(facet) {
                None => push_gap(facet, AssuranceGapKind::FacetUngoverned),
                Some(row) => {
                    if let Some(kind) = row.proof_gap_kind() {
                        push_gap(facet, kind);
                    }
                    if let Some(kind) = row.state_gap_kind() {
                        push_gap(facet, kind);
                    }
                }
            }
        }
        gaps.sort_by(|a, b| {
            a.facet
                .as_str()
                .cmp(b.facet.as_str())
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

    /// Validates the consumer's static invariants.
    fn validate_static(&self) -> Vec<M5AssuranceRouteViolation> {
        let mut out = Vec::new();
        if self.consumer_label != self.consumer.label()
            || self.owner_role != self.consumer.owner_role()
        {
            out.push(M5AssuranceRouteViolation::MissingIdentity);
        }
        if self.read_facets.is_empty() {
            out.push(M5AssuranceRouteViolation::ConsumerReadsNoFacets);
        }
        if !self
            .status_message_id
            .starts_with(M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX)
            || !self
                .gate_message_id
                .starts_with(M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX)
        {
            out.push(M5AssuranceRouteViolation::UnprefixedMessageId);
        }
        for gap in &self.gaps {
            if !gap
                .cause_message_id
                .starts_with(M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX)
                || gap.consumer != self.consumer
                || gap.dimension != gap.facet.dimension()
            {
                out.push(M5AssuranceRouteViolation::CoverageGapInvalid);
            }
        }
        out
    }
}

/// Position of a facet in the canonical ordering.
fn facet_rank(facet: &AssuranceFacet) -> usize {
    AssuranceFacet::ALL
        .iter()
        .position(|f| f == facet)
        .unwrap_or(AssuranceFacet::ALL.len())
}

/// Position of a dimension in the canonical ordering.
fn dimension_rank(dimension: AssuranceDimension) -> usize {
    AssuranceDimension::ALL
        .iter()
        .position(|d| *d == dimension)
        .unwrap_or(AssuranceDimension::ALL.len())
}

/// Position of an evidence class in the canonical ordering.
fn evidence_rank(class: EvidenceClass) -> usize {
    EvidenceClass::ALL
        .iter()
        .position(|c| *c == class)
        .unwrap_or(EvidenceClass::ALL.len())
}

/// Position of a posture in the canonical ordering.
fn posture_rank(posture: ClaimedPosture) -> usize {
    ClaimedPosture::ALL
        .iter()
        .position(|p| *p == posture)
        .unwrap_or(ClaimedPosture::ALL.len())
}

/// Position of a trust boundary in the canonical ordering.
fn boundary_rank(boundary: TrustBoundary) -> usize {
    TrustBoundary::ALL
        .iter()
        .position(|b| *b == boundary)
        .unwrap_or(TrustBoundary::ALL.len())
}

/// Which surfaces consume the one governance matrix. Every flag must hold so no surface keeps a
/// parallel assurance / route inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceDisclosure {
    /// The assurance center consumes the matrix.
    pub assurance_center_consumes_matrix: bool,
    /// The governance dashboard consumes the matrix.
    pub governance_dashboard_consumes_matrix: bool,
    /// The capability-boundary inspector consumes the matrix.
    pub capability_inspector_consumes_matrix: bool,
    /// The route / provenance inspector consumes the matrix.
    pub route_inspector_consumes_matrix: bool,
    /// The admin console consumes the matrix.
    pub admin_console_consumes_matrix: bool,
    /// The About / help surface consumes the matrix.
    pub help_about_consumes_matrix: bool,
    /// Procurement / evaluation consumes the matrix.
    pub procurement_evaluation_consumes_matrix: bool,
    /// Support exports consume the matrix.
    pub support_export_consumes_matrix: bool,
}

impl AssuranceDisclosure {
    /// The canonical disclosure: every surface consumes the matrix.
    pub const fn all_surfaces() -> Self {
        Self {
            assurance_center_consumes_matrix: true,
            governance_dashboard_consumes_matrix: true,
            capability_inspector_consumes_matrix: true,
            route_inspector_consumes_matrix: true,
            admin_console_consumes_matrix: true,
            help_about_consumes_matrix: true,
            procurement_evaluation_consumes_matrix: true,
            support_export_consumes_matrix: true,
        }
    }

    /// True when every surface consumes the matrix.
    pub const fn all_consume(&self) -> bool {
        self.assurance_center_consumes_matrix
            && self.governance_dashboard_consumes_matrix
            && self.capability_inspector_consumes_matrix
            && self.route_inspector_consumes_matrix
            && self.admin_console_consumes_matrix
            && self.help_about_consumes_matrix
            && self.procurement_evaluation_consumes_matrix
            && self.support_export_consumes_matrix
    }
}

/// Self-describing controlled-vocabulary set so the packet resolves every token it carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceVocabulary {
    /// Dimension tokens.
    pub dimensions: Vec<String>,
    /// Facet tokens.
    pub facets: Vec<String>,
    /// State-family tokens.
    pub state_families: Vec<String>,
    /// Evidence-class tokens.
    pub evidence_classes: Vec<String>,
    /// Claimed-posture tokens.
    pub claimed_postures: Vec<String>,
    /// Trust-boundary tokens.
    pub trust_boundaries: Vec<String>,
    /// Degraded-data-behavior tokens.
    pub degraded_data_behaviors: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Qualification-class tokens.
    pub qualification_classes: Vec<String>,
}

impl AssuranceVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dimensions: tokens(&AssuranceDimension::ALL, |d| d.as_str()),
            facets: tokens(&AssuranceFacet::ALL, |f| f.as_str()),
            state_families: tokens(&AssuranceStateFamily::ALL, |f| f.as_str()),
            evidence_classes: tokens(&EvidenceClass::ALL, |c| c.as_str()),
            claimed_postures: tokens(&ClaimedPosture::ALL, |p| p.as_str()),
            trust_boundaries: tokens(&TrustBoundary::ALL, |b| b.as_str()),
            degraded_data_behaviors: tokens(&DegradedDataBehavior::ALL, |b| b.as_str()),
            consumers: tokens(&AssuranceConsumer::ALL, |c| c.as_str()),
            gap_kinds: tokens(&AssuranceGapKind::ALL, |k| k.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |g| g.as_str()),
            freshness_states: tokens(&FreshnessState::ALL, |f| f.as_str()),
            qualification_classes: tokens(&QualificationClass::ALL, |q| q.as_str()),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Maps a typed `ALL` array to its stable tokens.
fn tokens<T: Copy>(all: &[T], f: impl Fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|t| f(*t).to_owned()).collect()
}

/// Compact governance summary — the scoreboard assurance / governance / support surfaces read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceSummary {
    /// Total governed facets.
    pub total_facets: u32,
    /// Facets whose proof is current.
    pub current_facets: u32,
    /// Facets whose proof is stale.
    pub stale_facets: u32,
    /// Facets whose proof is expired.
    pub expired_facets: u32,
    /// Facets whose proof is missing.
    pub missing_facets: u32,
    /// Total canonical state families.
    pub total_state_families: u32,
    /// Total claimed consumers.
    pub total_consumers: u32,
    /// Consumers certified at their full claim.
    pub certified_consumer_count: u32,
    /// Consumers that auto-narrowed below their claim.
    pub narrowed_consumer_count: u32,
    /// Consumers blocked from Stable promotion.
    pub blocked_consumer_count: u32,
    /// True when at least one consumer is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
}

/// Packet-level release gate aggregating the per-consumer gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceReleaseGate {
    /// True when at least one consumer is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Sorted consumer tokens blocked from Stable promotion.
    pub blocked_consumers: Vec<String>,
    /// Sorted consumer tokens that auto-narrowed below their claim.
    pub narrowed_consumers: Vec<String>,
    /// Sorted consumer tokens fully certified for Stable promotion.
    pub certified_consumers: Vec<String>,
    /// Sorted dimension tokens whose proof or assurance state drifted.
    pub drifted_dimensions: Vec<String>,
    /// Stable message id; prefixed [`M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

/// Conformance review. Every flag is a hard invariant; all must hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceConformance {
    /// Every facet is governed exactly once with a proof path.
    pub every_facet_governed_with_proof: bool,
    /// Every dimension is covered by at least one governed facet.
    pub every_dimension_covered: bool,
    /// Every state family is referenced by at least one governed facet.
    pub every_state_family_referenced: bool,
    /// Every claimed consumer maps to facets, disclosures, and proof paths.
    pub every_consumer_maps_to_facets_and_proof: bool,
    /// Every claimed consumer reads at least one governed facet.
    pub every_consumer_reads_at_least_one_facet: bool,
    /// A stale facet proof narrows the consumers that read it deterministically.
    pub stale_proof_narrows_deterministically: bool,
    /// An expired / missing / ungoverned facet proof blocks the consumers that read it.
    pub missing_proof_blocks_stable_promotion: bool,
    /// Exact coverage gaps are named per consumer with their drifted dimension.
    pub exact_gaps_named_per_consumer: bool,
    /// Every canonical assurance state binds to a gate posture and a consistent floor.
    pub state_vocabulary_bound_to_gate: bool,
    /// Assurance, governance, route, admin, help, procurement, and support surfaces consume one
    /// matrix.
    pub surfaces_consume_one_matrix: bool,
    /// The matrix is generated from the same checked-in proofs and assurance states.
    pub generated_from_checked_in_proofs: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// The export carries no raw provider material — route / evidence lineage stays refs-only.
    pub export_carries_no_raw_material: bool,
}

impl AssuranceConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_facet_governed_with_proof
            && self.every_dimension_covered
            && self.every_state_family_referenced
            && self.every_consumer_maps_to_facets_and_proof
            && self.every_consumer_reads_at_least_one_facet
            && self.stale_proof_narrows_deterministically
            && self.missing_proof_blocks_stable_promotion
            && self.exact_gaps_named_per_consumer
            && self.state_vocabulary_bound_to_gate
            && self.surfaces_consume_one_matrix
            && self.generated_from_checked_in_proofs
            && self.controlled_enums_frozen
            && self.export_carries_no_raw_material
    }
}

/// Constructor input for [`M5AssuranceRouteGovernance::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AssuranceRouteInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The governed facet rows.
    pub facets: Vec<AssuranceFacetRow>,
    /// The claimed consumer rows (unions / gaps / verdict are recomputed from the facets).
    pub consumers: Vec<AssuranceConsumerRow>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable governance truth packet assurance, governance, route,
/// admin, help, procurement, and support surfaces consume: the canonical state families, the
/// governed facets, the per-consumer matrix, the controlled vocabulary, a conformance review, a
/// summary, and the aggregate release gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AssuranceRouteGovernance {
    /// Record kind; must equal [`M5_ASSURANCE_ROUTE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ASSURANCE_ROUTE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The canonical assurance state families.
    pub state_families: Vec<AssuranceStateFamilyDef>,
    /// The governed facet rows.
    pub facets: Vec<AssuranceFacetRow>,
    /// The claimed consumer rows with their derived verdicts.
    pub consumers: Vec<AssuranceConsumerRow>,
    /// The consumer tokens that read this matrix.
    pub consumer_tokens: Vec<String>,
    /// Which surfaces consume the matrix.
    pub disclosure: AssuranceDisclosure,
    /// Compact governance summary.
    pub summary: AssuranceSummary,
    /// Packet-level release gate.
    pub release_gate: AssuranceReleaseGate,
    /// Controlled-vocabulary set.
    pub vocabulary: AssuranceVocabulary,
    /// Conformance review block.
    pub conformance: AssuranceConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AssuranceRouteGovernance {
    /// Builds a governance packet from seed input, recomputing each consumer's verdict and deriving
    /// the state families, summary, release gate, and conformance review from the facets.
    pub fn new(input: M5AssuranceRouteInput) -> Self {
        let facets = input.facets;
        let state_families = canonical_state_families();
        let mut consumers = input.consumers;
        for consumer in &mut consumers {
            consumer.recompute(&facets);
        }
        let consumer_tokens = tokens(&AssuranceConsumer::ALL, |c| c.as_str());
        let summary = derive_summary(&facets, &state_families, &consumers);
        let release_gate = derive_release_gate(&facets, &consumers);
        let conformance = derive_conformance(&facets, &state_families, &consumers);
        Self {
            record_kind: M5_ASSURANCE_ROUTE_RECORD_KIND.to_owned(),
            schema_version: M5_ASSURANCE_ROUTE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            state_families,
            facets,
            consumers,
            consumer_tokens,
            disclosure: AssuranceDisclosure::all_surfaces(),
            summary,
            release_gate,
            vocabulary: AssuranceVocabulary::canonical(),
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release automation must hold Stable promotion.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.release_gate.blocks_stable_promotion
    }

    /// Finds a governed facet by facet.
    pub fn facet(&self, facet: AssuranceFacet) -> Option<&AssuranceFacetRow> {
        self.facets.iter().find(|r| r.facet == facet)
    }

    /// Finds a consumer row by consumer.
    pub fn consumer(&self, consumer: AssuranceConsumer) -> Option<&AssuranceConsumerRow> {
        self.consumers.iter().find(|c| c.consumer == consumer)
    }

    /// Renders the packet for a generation channel. The output is identical for every channel —
    /// the channel parameter exists only to prove desktop, CLI/headless, and offline / mirror
    /// generation produce byte-identical output.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn render_for_channel(&self, _channel: AssuranceChannel) -> String {
        self.export_safe_json()
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 assurance route governance serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per (consumer, read facet) join, naming
    /// the consumer, its owner, the facet, the facet owner, the proof path, and any gap.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,consumer_owner,claimed_qualification,effective_qualification,gate_decision,facet,dimension,state_family,current_state,facet_owner,proof_ref,proof_freshness,facet_status,evidence_classes,claimed_postures,trust_boundaries,degraded_data_behavior,gap_kind\n",
        );
        for c in &self.consumers {
            for &facet in &c.read_facets {
                let row = self.facet(facet);
                let gap_kind = c
                    .gaps
                    .iter()
                    .find(|g| g.facet == facet)
                    .map(|g| g.gap_kind.as_str())
                    .unwrap_or("");
                let (
                    current_state,
                    facet_owner,
                    proof_ref,
                    proof_freshness,
                    facet_status,
                    evidence_classes,
                    postures,
                    boundaries,
                    degraded,
                ) = match row {
                    Some(r) => (
                        r.current_state_token.clone(),
                        r.owner_role.clone(),
                        r.proof_ref.clone(),
                        r.proof_freshness.as_str().to_owned(),
                        r.status.as_str().to_owned(),
                        join_tokens(&r.evidence_classes, |x| x.as_str()),
                        join_tokens(&r.claimed_postures, |x| x.as_str()),
                        join_tokens(&r.trust_boundaries, |x| x.as_str()),
                        r.degraded_data_behavior.as_str().to_owned(),
                    ),
                    None => (
                        String::new(),
                        String::new(),
                        facet.proof_ref().to_owned(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                    ),
                };
                out.push_str(&format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                    c.consumer.as_str(),
                    c.owner_role,
                    c.claimed_qualification.as_str(),
                    c.effective_qualification.as_str(),
                    c.gate_decision.as_str(),
                    facet.as_str(),
                    facet.dimension().as_str(),
                    facet.state_family().as_str(),
                    current_state,
                    facet_owner,
                    proof_ref,
                    proof_freshness,
                    facet_status,
                    evidence_classes,
                    postures,
                    boundaries,
                    degraded,
                    gap_kind,
                ));
            }
        }
        out
    }

    /// Deterministic governance matrix document for review, support, docs, or shiproom handoff.
    pub fn render_governance_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Assurance / Governance / Route-Provenance Governance Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Facets: {} ({} current, {} stale, {} expired, {} missing)\n",
            self.summary.total_facets,
            self.summary.current_facets,
            self.summary.stale_facets,
            self.summary.expired_facets,
            self.summary.missing_facets
        ));
        out.push_str(&format!(
            "- Consumers: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_consumers,
            self.summary.certified_consumer_count,
            self.summary.narrowed_consumer_count,
            self.summary.blocked_consumer_count
        ));
        out.push_str(&format!(
            "- Release gate: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));
        out.push_str(
            "- Consumed by: assurance center, governance dashboard, capability inspector, route inspector, admin, About/help, procurement, support\n",
        );

        out.push_str("\n## Canonical assurance state families\n\n");
        out.push_str("| Family | State | Gate posture | Effective floor |\n");
        out.push_str("|--------|-------|--------------|-----------------|\n");
        for family in &self.state_families {
            for state in &family.states {
                out.push_str(&format!(
                    "| `{}` | `{}` | `{}` | `{}` |\n",
                    family.family.as_str(),
                    state.token,
                    state.gate_posture.as_str(),
                    state.effective_floor.as_str()
                ));
            }
        }

        out.push_str("\n## Governed facets\n\n");
        out.push_str(
            "| Facet | Dimension | State family | Current state | Postures | Boundaries | Degraded-data | Owner | Proof | Freshness | Status |\n",
        );
        out.push_str(
            "|-------|-----------|--------------|---------------|----------|------------|---------------|-------|-------|-----------|--------|\n",
        );
        for f in &self.facets {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                f.facet.as_str(),
                f.dimension.as_str(),
                f.state_family.as_str(),
                f.current_state_token,
                join_tokens(&f.claimed_postures, |x| x.as_str()),
                join_tokens(&f.trust_boundaries, |x| x.as_str()),
                f.degraded_data_behavior.as_str(),
                f.owner_role,
                f.proof_ref,
                f.proof_freshness.as_str(),
                f.status.as_str()
            ));
        }

        out.push_str("\n## Claimed consumers\n\n");
        out.push_str(
            "| Consumer | Owner | Status | Claim → effective | Gate | Reads | Evidence classes |\n",
        );
        out.push_str(
            "|----------|-------|--------|-------------------|------|-------|------------------|\n",
        );
        for c in &self.consumers {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` → `{}` | `{}` | {} | {} |\n",
                c.consumer.as_str(),
                c.owner_role,
                c.status.as_str(),
                c.claimed_qualification.as_str(),
                c.effective_qualification.as_str(),
                c.gate_decision.as_str(),
                join_tokens(&c.read_facets, |x| x.as_str()),
                join_tokens(&c.disclosed_evidence_classes, |x| x.as_str())
            ));
            for gap in &c.gaps {
                out.push_str(&format!(
                    "| | | gap: `{}` on `{}` (`{}`) | | | | |\n",
                    gap.gap_kind.as_str(),
                    gap.facet.as_str(),
                    gap.dimension.as_str()
                ));
            }
        }
        out
    }

    /// Compact Markdown report for the release-grade parity proof.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Assurance / Governance / Route-Provenance Governance — Proof\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Facets: {} ({} current, {} stale, {} expired, {} missing)\n",
            self.summary.total_facets,
            self.summary.current_facets,
            self.summary.stale_facets,
            self.summary.expired_facets,
            self.summary.missing_facets
        ));
        out.push_str(&format!(
            "- Consumers: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_consumers,
            self.summary.certified_consumer_count,
            self.summary.narrowed_consumer_count,
            self.summary.blocked_consumer_count
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
        out.push_str(&format!(
            "- Matrix CSV: `{}`\n",
            M5_ASSURANCE_ROUTE_MATRIX_CSV_REF
        ));
        out
    }

    /// Validates the packet's invariants.
    pub fn validate(&self) -> Vec<M5AssuranceRouteViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_ASSURANCE_ROUTE_RECORD_KIND {
            out.push(M5AssuranceRouteViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ASSURANCE_ROUTE_SCHEMA_VERSION {
            out.push(M5AssuranceRouteViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5AssuranceRouteViolation::MissingIdentity);
        }

        // Canonical state families.
        if self.state_families != canonical_state_families() {
            out.push(M5AssuranceRouteViolation::StateFamilyDrift);
        }
        for family in &self.state_families {
            out.extend(family.validate());
        }

        // Every facet governed exactly once and self-consistent.
        let mut seen_facets = std::collections::BTreeSet::new();
        for facet in &self.facets {
            if !seen_facets.insert(facet.facet) {
                out.push(M5AssuranceRouteViolation::DuplicateFacet);
            }
            out.extend(facet.validate());
        }
        for facet in AssuranceFacet::ALL {
            if !self.facets.iter().any(|r| r.facet == facet) {
                out.push(M5AssuranceRouteViolation::FacetNotGoverned);
            }
        }

        if self.consumers.is_empty() {
            out.push(M5AssuranceRouteViolation::PacketHasNoConsumers);
        }
        let mut seen_consumers = std::collections::BTreeSet::new();
        for consumer in &self.consumers {
            if !seen_consumers.insert(consumer.consumer) {
                out.push(M5AssuranceRouteViolation::DuplicateConsumer);
            }
            out.extend(consumer.validate_static());
            // The stored verdict must match a fresh recompute from the facets.
            let mut probe = consumer.clone();
            probe.recompute(&self.facets);
            if probe != *consumer {
                out.push(M5AssuranceRouteViolation::ConsumerVerdictDrift);
            }
        }

        let expected_tokens = tokens(&AssuranceConsumer::ALL, |c| c.as_str());
        if self.consumer_tokens != expected_tokens {
            out.push(M5AssuranceRouteViolation::ConsumerSetMismatch);
        }
        if !self.disclosure.all_consume() {
            out.push(M5AssuranceRouteViolation::DisclosureIncomplete);
        }
        if self.summary != derive_summary(&self.facets, &self.state_families, &self.consumers) {
            out.push(M5AssuranceRouteViolation::SummaryDrift);
        }
        if self.release_gate != derive_release_gate(&self.facets, &self.consumers) {
            out.push(M5AssuranceRouteViolation::ReleaseGateAggregateMismatch);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5AssuranceRouteViolation::VocabularyMismatch);
        }
        if self.conformance
            != derive_conformance(&self.facets, &self.state_families, &self.consumers)
            || !self.conformance.all_hold()
        {
            out.push(M5AssuranceRouteViolation::ConformanceReviewFailed);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 assurance route governance serializes"),
        ) {
            out.push(M5AssuranceRouteViolation::RawMaterialInExport);
        }
        out
    }
}

/// The generation channel a governance packet is produced on. Every channel produces byte-identical
/// output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceChannel {
    /// The desktop product UI.
    DesktopUi,
    /// The CLI / headless structured-output path.
    CliHeadless,
    /// Offline / mirror-safe packet generation.
    OfflineMirror,
}

impl AssuranceChannel {
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

/// Joins a token list for table / CSV rendering, comma-space separated.
fn join_tokens<T: Copy>(items: &[T], f: impl Fn(T) -> &'static str) -> String {
    items
        .iter()
        .map(|t| f(*t))
        .collect::<Vec<_>>()
        .join(if items.len() > 8 { "," } else { " " })
}

/// Derives the governance summary from the facets, state families, and consumers.
fn derive_summary(
    facets: &[AssuranceFacetRow],
    state_families: &[AssuranceStateFamilyDef],
    consumers: &[AssuranceConsumerRow],
) -> AssuranceSummary {
    let facet_count = |state: FreshnessState| -> u32 {
        facets.iter().filter(|f| f.proof_freshness == state).count() as u32
    };
    let blocked = consumers.iter().filter(|c| c.is_blocked()).count() as u32;
    AssuranceSummary {
        total_facets: facets.len() as u32,
        current_facets: facet_count(FreshnessState::Current),
        stale_facets: facet_count(FreshnessState::Stale),
        expired_facets: facet_count(FreshnessState::Expired),
        missing_facets: facet_count(FreshnessState::Missing),
        total_state_families: state_families.len() as u32,
        total_consumers: consumers.len() as u32,
        certified_consumer_count: consumers.iter().filter(|c| c.is_certified()).count() as u32,
        narrowed_consumer_count: consumers.iter().filter(|c| c.is_narrowed()).count() as u32,
        blocked_consumer_count: blocked,
        blocks_stable_promotion: blocked > 0,
    }
}

/// Derives the aggregate release gate from the per-consumer gates and the drifted facets.
fn derive_release_gate(
    facets: &[AssuranceFacetRow],
    consumers: &[AssuranceConsumerRow],
) -> AssuranceReleaseGate {
    let pick = |f: &dyn Fn(&AssuranceConsumerRow) -> bool| -> Vec<String> {
        let mut t: Vec<String> = consumers
            .iter()
            .filter(|c| f(c))
            .map(|c| c.consumer.as_str().to_owned())
            .collect();
        t.sort();
        t
    };
    let mut drifted_dimensions: Vec<String> = facets
        .iter()
        .filter(|f| {
            !matches!(f.proof_freshness, FreshnessState::Current)
                || !matches!(f.state_gate, DescriptorGate::Governed)
        })
        .map(|f| f.dimension.as_str().to_owned())
        .collect();
    drifted_dimensions.sort();
    drifted_dimensions.dedup();
    let blocked = pick(&|c| c.is_blocked());
    AssuranceReleaseGate {
        blocks_stable_promotion: !blocked.is_empty(),
        blocked_consumers: blocked,
        narrowed_consumers: pick(&|c| c.is_narrowed()),
        certified_consumers: pick(&|c| c.is_certified()),
        drifted_dimensions,
        gate_message_id: format!("{M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX}release_gate"),
    }
}

/// Derives the conformance review, so the stored block reflects the actual packet.
fn derive_conformance(
    facets: &[AssuranceFacetRow],
    state_families: &[AssuranceStateFamilyDef],
    consumers: &[AssuranceConsumerRow],
) -> AssuranceConformance {
    let every_facet = AssuranceFacet::ALL.iter().all(|f| {
        facets
            .iter()
            .filter(|r| r.facet == *f)
            .filter(|r| !r.proof_ref.trim().is_empty())
            .count()
            == 1
    });

    let every_dimension = AssuranceDimension::ALL
        .iter()
        .all(|d| facets.iter().any(|r| r.dimension == *d));

    let every_family = AssuranceStateFamily::ALL
        .iter()
        .all(|fam| facets.iter().any(|r| r.state_family == *fam));

    let maps_to_proof = !consumers.is_empty()
        && consumers.iter().all(|c| {
            !c.read_facets.is_empty()
                && c.proof_refs.len() == c.read_facets.len()
                && !c.disclosed_evidence_classes.is_empty()
                && !c.claimed_postures.is_empty()
                && !c.trust_boundaries.is_empty()
        });

    let every_reads_facet = consumers.iter().all(|c| !c.read_facets.is_empty());

    let posture_of = |facet: AssuranceFacet| -> Option<(FreshnessState, DescriptorGate)> {
        facets
            .iter()
            .find(|r| r.facet == facet)
            .map(|r| (r.proof_freshness, r.state_gate))
    };

    // A facet that only narrows (stale proof or narrowing state) narrows every consumer that reads
    // it, unless a blocking facet already blocks that consumer.
    let stale_narrows = consumers.iter().all(|c| {
        let reads_narrowing = c.read_facets.iter().any(|f| {
            matches!(
                posture_of(*f),
                Some((FreshnessState::Stale, _)) | Some((_, DescriptorGate::Narrowed))
            )
        });
        let reads_blocking = c.read_facets.iter().any(|f| {
            matches!(
                posture_of(*f),
                Some((FreshnessState::Expired, _))
                    | Some((FreshnessState::Missing, _))
                    | Some((_, DescriptorGate::Blocked))
                    | None
            )
        });
        !reads_narrowing || reads_blocking || c.is_narrowed()
    });

    // A blocking facet (expired / missing / ungoverned proof or blocking state) blocks every
    // consumer that reads it.
    let missing_blocks = consumers.iter().all(|c| {
        let reads_blocking = c.read_facets.iter().any(|f| {
            matches!(
                posture_of(*f),
                Some((FreshnessState::Expired, _))
                    | Some((FreshnessState::Missing, _))
                    | Some((_, DescriptorGate::Blocked))
                    | None
            )
        });
        !reads_blocking || c.is_blocked()
    });

    let gaps_named = consumers.iter().all(|c| {
        c.gaps.iter().all(|g| {
            g.cause_message_id
                .starts_with(M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX)
                && g.consumer == c.consumer
                && g.dimension == g.facet.dimension()
        })
    });

    let state_bound = state_families == canonical_state_families()
        && state_families.iter().all(|fam| {
            fam.states
                .iter()
                .all(|s| s.effective_floor == floor_for_posture(s.gate_posture))
        });

    let generated = consumers.iter().all(|c| {
        let mut probe = c.clone();
        probe.recompute(facets);
        probe == *c
    });

    let export_clean =
        !json_contains_forbidden_material(&serde_json::to_value(facets).expect("facets serialize"))
            && !json_contains_forbidden_material(
                &serde_json::to_value(consumers).expect("consumers serialize"),
            );

    AssuranceConformance {
        every_facet_governed_with_proof: every_facet,
        every_dimension_covered: every_dimension,
        every_state_family_referenced: every_family,
        every_consumer_maps_to_facets_and_proof: maps_to_proof,
        every_consumer_reads_at_least_one_facet: every_reads_facet,
        stale_proof_narrows_deterministically: stale_narrows,
        missing_proof_blocks_stable_promotion: missing_blocks,
        exact_gaps_named_per_consumer: gaps_named,
        state_vocabulary_bound_to_gate: state_bound,
        surfaces_consume_one_matrix: true,
        generated_from_checked_in_proofs: generated,
        controlled_enums_frozen: AssuranceVocabulary::canonical().matches_canonical(),
        export_carries_no_raw_material: export_clean,
    }
}

/// Validation failures for the assurance / route governance lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AssuranceRouteViolation {
    /// The packet record kind is wrong.
    WrongRecordKind,
    /// The packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The canonical state families drifted.
    StateFamilyDrift,
    /// A state token binds an inconsistent floor or unprefixed message id.
    StateBindingInvalid,
    /// A facet cites a field that does not match its facet.
    FacetFieldMismatch,
    /// A facet's current state is not a member of its family, or its floor is inconsistent.
    FacetStateInvalid,
    /// A facet discloses no evidence classes, postures, or trust boundaries.
    FacetDisclosureEmpty,
    /// A facet's status drifted from its proof freshness.
    FacetStatusDrift,
    /// Two facets name the same facet.
    DuplicateFacet,
    /// A facet has no governed entry.
    FacetNotGoverned,
    /// The packet declares no claimed consumers.
    PacketHasNoConsumers,
    /// Two consumers share a consumer token.
    DuplicateConsumer,
    /// A claimed consumer reads no facets.
    ConsumerReadsNoFacets,
    /// A consumer's stored verdict drifted from a fresh recompute.
    ConsumerVerdictDrift,
    /// A coverage gap is malformed (wrong consumer, dimension, or unprefixed message id).
    CoverageGapInvalid,
    /// The consumer-token set does not match the canonical consumers.
    ConsumerSetMismatch,
    /// A disclosure surface does not consume the matrix.
    DisclosureIncomplete,
    /// The summary disagrees with the facets / consumers.
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

impl M5AssuranceRouteViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::StateFamilyDrift => "state_family_drift",
            Self::StateBindingInvalid => "state_binding_invalid",
            Self::FacetFieldMismatch => "facet_field_mismatch",
            Self::FacetStateInvalid => "facet_state_invalid",
            Self::FacetDisclosureEmpty => "facet_disclosure_empty",
            Self::FacetStatusDrift => "facet_status_drift",
            Self::DuplicateFacet => "duplicate_facet",
            Self::FacetNotGoverned => "facet_not_governed",
            Self::PacketHasNoConsumers => "packet_has_no_consumers",
            Self::DuplicateConsumer => "duplicate_consumer",
            Self::ConsumerReadsNoFacets => "consumer_reads_no_facets",
            Self::ConsumerVerdictDrift => "consumer_verdict_drift",
            Self::CoverageGapInvalid => "coverage_gap_invalid",
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
