//! The M5 governance / fitness dashboard — the operator / admin / evaluator-facing surface that
//! turns Aureline's protected fitness functions, nightly governance runs, accepted waivers, service
//! ownership, and decision rights into inspectable product truth, bound to the same gate-bound state
//! grammar the [governance matrix](crate::m5_assurance_route_governance) and the
//! [assurance center](crate::m5_assurance_center) froze.
//!
//! The assurance center reads "what does Aureline claim and what proves it"; this lane reads the
//! *governance* layer beside it: "which protected fitness functions are passing right now, what is
//! held under an accepted waiver, when does that waiver expire, who owns each governed service, and
//! who decides each governed change?" Like the assurance center, every surface is **derived** from
//! one input — each fitness function's measured run result, evidence freshness, and (when present) an
//! accepted waiver and its expiry standing — so a tile can never read greener than its proof.
//!
//! The packet has five product parts, all derived from that one input:
//!
//! - [`FitnessTile`]s. One per protected [fitness function](FitnessFunction). A tile never asserts a
//!   fixed colour; it derives its [state](FitnessState) from its measured result, evidence freshness,
//!   and waiver standing, distinguishing `passing`, `warning`, `evidence_stale`, `waived`,
//!   `waiver_expired`, and `blocked` rather than flattening them into one pass / fail colour. Stale
//!   evidence narrows a tile to `evidence_stale`; an accepted waiver narrows it to `waived`; an
//!   expired waiver and missing / expired evidence block it.
//! - [`NightlyGovernanceRow`]s. The nightly run record for each fitness function — its last run, run
//!   state, measured result, freshness, and passing streak — bound to the same proof ref the tile
//!   reads.
//! - [`WaiverQueueRow`]s. The accepted waivers, ordered by expiry urgency (expired first), each
//!   disclosing its expiry, rationale, responsible party, the action that clears it, and the
//!   governance ticket it rides.
//! - [`ServiceOwnershipCard`]s. One per governed [service](Service): its accountable owner, decision
//!   forum, the fitness functions it owns, its worst tile state, and its open / expired waiver
//!   counts.
//! - [`DecisionRightCard`]s. One per governed [decision right](DecisionRight): the forum that decides
//!   it, the accountable owner, the services it governs, and whether the decision is currently
//!   exercisable or held because its scope is blocked.
//!
//! [`GovernanceOverview`]s summarise the board per [deployment profile](crate::m5_assurance_route_governance::ClaimedPosture),
//! and the whole packet is stamped with one corpus identity so a pass measured against one corpus /
//! profile can never be read as a pass in another context. Finally the packet carries a
//! [`GovernanceEvaluationPacket`] export that reuses the exact state and proof vocabulary the tiles
//! show. The [`M5GovernanceDashboard`] packet is the one inspectable, serde-serializable truth record
//! this lane produces: it preserves proof lineage as refs only and carries no credential bodies or
//! raw provider payloads.
//!
//! - Packet schema:
//!   [`schemas/public-truth/m5-governance-dashboard.schema.json`](../../../../../schemas/public-truth/m5-governance-dashboard.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-governance-dashboard-contract.md`](../../../../../docs/public-truth/m5-governance-dashboard-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_governance_dashboard, seeded_m5_governance_dashboard_evidence_stale_narrowed,
    seeded_m5_governance_dashboard_missing_evidence_blocked,
    seeded_m5_governance_dashboard_waiver_active_narrowed,
    seeded_m5_governance_dashboard_waiver_expired_blocked, seeded_m5_governance_dashboard_warning,
    M5_GOVERNANCE_DASHBOARD_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The governance dashboard reuses the governance matrix's frozen posture / evidence vocabulary and
// the descriptor / badge gate runtime, so the in-product tiles and the exported evaluation packet can
// never drift to a different state grammar.
use crate::m5_assurance_route_governance::{ClaimedPosture, EvidenceClass};
use crate::m5_descriptor_badge::{
    ConsumerStatus, DescriptorGate, DescriptorSignal, FreshnessState, QualificationClass,
};

/// Record-kind tag carried by [`M5GovernanceDashboard`].
pub const M5_GOVERNANCE_DASHBOARD_RECORD_KIND: &str = "m5_governance_dashboard";

/// Record-kind tag carried by the embedded [`GovernanceEvaluationPacket`].
pub const M5_GOVERNANCE_EVALUATION_RECORD_KIND: &str = "m5_governance_dashboard_evaluation_packet";

/// Schema version for the governance-dashboard packet.
pub const M5_GOVERNANCE_DASHBOARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the governance-dashboard packet schema.
pub const M5_GOVERNANCE_DASHBOARD_SCHEMA_REF: &str =
    "schemas/public-truth/m5-governance-dashboard.schema.json";

/// Repo-relative path of the published governance-dashboard inventory.
pub const M5_GOVERNANCE_DASHBOARD_REGISTRY_REF: &str =
    "artifacts/public-truth/m5-governance-dashboard.json";

/// Repo-relative path of the rendered governance-dashboard overview document.
pub const M5_GOVERNANCE_DASHBOARD_OVERVIEW_REF: &str =
    "artifacts/public-truth/m5-governance-dashboard.md";

/// Repo-relative path of the machine-readable fitness-tile matrix export.
pub const M5_GOVERNANCE_DASHBOARD_TILES_CSV_REF: &str =
    "artifacts/public-truth/m5-governance-dashboard-tiles.csv";

/// Repo-relative path of the release-grade governance-dashboard parity proof.
pub const M5_GOVERNANCE_DASHBOARD_PROOF_REF: &str =
    "artifacts/public-truth/m5-governance-dashboard-proof/governance-dashboard.json";

/// Repo-relative path of the exported evaluation packet.
pub const M5_GOVERNANCE_DASHBOARD_EVALUATION_PACKET_REF: &str =
    "artifacts/public-truth/m5-governance-dashboard-proof/evaluation-packet.json";

/// Repo-relative path of the governance-dashboard contract doc.
pub const M5_GOVERNANCE_DASHBOARD_DOC_REF: &str =
    "docs/public-truth/m5-governance-dashboard-contract.md";

/// Repo-relative directory of the per-state governance-dashboard fixtures.
pub const M5_GOVERNANCE_DASHBOARD_FIXTURE_DIR: &str =
    "fixtures/public-truth/m5-governance-dashboard/";

/// Prefix every governance-dashboard message id carries so consumers can route it.
pub const M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX: &str = "public_truth.governance_dashboard.";

// ---------------------------------------------------------------------------------------------
// Fitness functions
// ---------------------------------------------------------------------------------------------

/// One protected fitness function the dashboard watches. The set is the package-boundary, evidence,
/// claim, and route / provenance fitness functions the protected-fitness lane names; this dashboard
/// invents no new fitness functions and owns no metric logic — it reads the nightly results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FitnessFunction {
    /// No production crate depends on a spike / benchmark crate or a forbidden edge.
    PackageBoundaryIntegrity,
    /// Every protected path carries a current decision record and owner.
    ProtectedPathReview,
    /// Schemas and their example artifacts / fixtures do not drift.
    SchemaExampleParity,
    /// Every proof packet stays inside its freshness SLO.
    EvidenceFreshnessSlo,
    /// No published claim reads wider than the proof backing it.
    ClaimNoOverclaim,
    /// Every high-risk route explains where work went and who approved it.
    RouteExplainability,
    /// The event-provenance ledger is complete for governed events.
    ProvenanceCompleteness,
}

impl FitnessFunction {
    /// Every fitness function, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::PackageBoundaryIntegrity,
        Self::ProtectedPathReview,
        Self::SchemaExampleParity,
        Self::EvidenceFreshnessSlo,
        Self::ClaimNoOverclaim,
        Self::RouteExplainability,
        Self::ProvenanceCompleteness,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageBoundaryIntegrity => "package_boundary_integrity",
            Self::ProtectedPathReview => "protected_path_review",
            Self::SchemaExampleParity => "schema_example_parity",
            Self::EvidenceFreshnessSlo => "evidence_freshness_slo",
            Self::ClaimNoOverclaim => "claim_no_overclaim",
            Self::RouteExplainability => "route_explainability",
            Self::ProvenanceCompleteness => "provenance_completeness",
        }
    }

    /// Reader-facing fitness-function label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PackageBoundaryIntegrity => "Package boundary integrity",
            Self::ProtectedPathReview => "Protected-path review",
            Self::SchemaExampleParity => "Schema / example parity",
            Self::EvidenceFreshnessSlo => "Evidence freshness SLO",
            Self::ClaimNoOverclaim => "Claim no-overclaim",
            Self::RouteExplainability => "Route explainability",
            Self::ProvenanceCompleteness => "Provenance completeness",
        }
    }

    /// The service that owns this fitness function.
    pub const fn service(self) -> Service {
        match self {
            Self::PackageBoundaryIntegrity | Self::ProtectedPathReview => {
                Service::PackageGovernance
            }
            Self::SchemaExampleParity | Self::EvidenceFreshnessSlo => Service::EvidencePipeline,
            Self::ClaimNoOverclaim => Service::ClaimPublication,
            Self::RouteExplainability | Self::ProvenanceCompleteness => Service::RouteProvenance,
        }
    }

    /// The weakest deployment profile this fitness function is required under; it applies to that
    /// profile and every stronger one, so a profile's overview narrows when one of its required
    /// functions narrows but stronger-only functions never narrow a weaker profile.
    pub const fn scope_profile(self) -> ClaimedPosture {
        match self {
            Self::PackageBoundaryIntegrity | Self::SchemaExampleParity => ClaimedPosture::Managed,
            Self::ProtectedPathReview => ClaimedPosture::SelfHosted,
            Self::EvidenceFreshnessSlo | Self::ClaimNoOverclaim => ClaimedPosture::Regulated,
            Self::RouteExplainability | Self::ProvenanceCompleteness => ClaimedPosture::Sovereign,
        }
    }

    /// The evidence class that grounds this fitness function's nightly result.
    pub const fn evidence_class(self) -> EvidenceClass {
        match self {
            Self::PackageBoundaryIntegrity => EvidenceClass::BoundaryManifest,
            Self::ProtectedPathReview => EvidenceClass::OwnershipRegister,
            Self::SchemaExampleParity => EvidenceClass::ControlAttestation,
            Self::EvidenceFreshnessSlo => EvidenceClass::PolicyBundle,
            Self::ClaimNoOverclaim => EvidenceClass::ControlAttestation,
            Self::RouteExplainability => EvidenceClass::RouteTimeline,
            Self::ProvenanceCompleteness => EvidenceClass::ProvenanceLedger,
        }
    }

    /// Owner role accountable for keeping this function passing — the role, never a person.
    pub const fn owner_role(self) -> &'static str {
        self.service().owner_role()
    }

    /// Repo-relative proof ref backing this function — drawn from the governance-matrix proofs, so
    /// the dashboard reuses the existing nightly governance feed rather than minting a parallel one.
    pub const fn proof_ref(self) -> &'static str {
        match self {
            Self::PackageBoundaryIntegrity => {
                "artifacts/release-proof/m5-assurance-route-governance/capability-boundary.json"
            }
            Self::ProtectedPathReview => {
                "artifacts/release-proof/m5-assurance-route-governance/control-proof.json"
            }
            Self::SchemaExampleParity | Self::EvidenceFreshnessSlo => {
                "artifacts/release-proof/m5-assurance-route-governance/governance-freshness.json"
            }
            Self::ClaimNoOverclaim => {
                "artifacts/release-proof/m5-assurance-route-governance/assurance-claim.json"
            }
            Self::RouteExplainability => {
                "artifacts/release-proof/m5-assurance-route-governance/route-hop.json"
            }
            Self::ProvenanceCompleteness => {
                "artifacts/release-proof/m5-assurance-route-governance/event-provenance.json"
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Services / forums / decision rights
// ---------------------------------------------------------------------------------------------

/// One governed service whose fitness functions the dashboard watches. Each service names one
/// accountable owner role and one decision forum, so the board makes ownership first-class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Service {
    /// Owns package-boundary and protected-path fitness functions.
    PackageGovernance,
    /// Owns schema / example parity and evidence-freshness fitness functions.
    EvidencePipeline,
    /// Owns the claim no-overclaim fitness function.
    ClaimPublication,
    /// Owns route-explainability and provenance-completeness fitness functions.
    RouteProvenance,
}

impl Service {
    /// Every service, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PackageGovernance,
        Self::EvidencePipeline,
        Self::ClaimPublication,
        Self::RouteProvenance,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageGovernance => "package_governance",
            Self::EvidencePipeline => "evidence_pipeline",
            Self::ClaimPublication => "claim_publication",
            Self::RouteProvenance => "route_provenance",
        }
    }

    /// Reader-facing service label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PackageGovernance => "Package governance",
            Self::EvidencePipeline => "Evidence pipeline",
            Self::ClaimPublication => "Claim publication",
            Self::RouteProvenance => "Route / provenance",
        }
    }

    /// Owner role accountable for the service — the role, never a person.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::PackageGovernance => "package_governance_owner",
            Self::EvidencePipeline => "evidence_pipeline_owner",
            Self::ClaimPublication => "claim_publication_owner",
            Self::RouteProvenance => "route_provenance_owner",
        }
    }

    /// The decision forum that governs the service's changes.
    pub const fn forum(self) -> Forum {
        match self {
            Self::PackageGovernance => Forum::ArchitectureForum,
            Self::EvidencePipeline | Self::RouteProvenance => Forum::GovernanceCouncil,
            Self::ClaimPublication => Forum::ShiproomForum,
        }
    }

    /// The fitness functions this service owns, in canonical order.
    fn owned_functions(self) -> Vec<FitnessFunction> {
        FitnessFunction::ALL
            .iter()
            .copied()
            .filter(|f| f.service() == self)
            .collect()
    }
}

/// One decision forum that governs a class of changes. A forum is a named body, never an individual,
/// so the dashboard discloses decision rights without leaking private membership.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Forum {
    /// The release shiproom.
    ShiproomForum,
    /// The governance council.
    GovernanceCouncil,
    /// The architecture forum.
    ArchitectureForum,
}

impl Forum {
    /// Every forum, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ShiproomForum,
        Self::GovernanceCouncil,
        Self::ArchitectureForum,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShiproomForum => "shiproom_forum",
            Self::GovernanceCouncil => "governance_council",
            Self::ArchitectureForum => "architecture_forum",
        }
    }

    /// Reader-facing forum label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ShiproomForum => "Shiproom",
            Self::GovernanceCouncil => "Governance council",
            Self::ArchitectureForum => "Architecture forum",
        }
    }
}

/// One governed decision right the dashboard makes visible: who decides a class of change and whether
/// the decision is currently exercisable. The set is the promotion, waiver, boundary, and exception
/// decisions the governance model names; this lane invents no new authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionRight {
    /// Whether a surface may promote to Stable.
    StablePromotion,
    /// Whether a control may be held under an accepted waiver.
    WaiverAcceptance,
    /// Whether a package-boundary change may land.
    BoundaryChange,
    /// Whether an accepted exception may be renewed.
    ExceptionRenewal,
}

impl DecisionRight {
    /// Every decision right, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StablePromotion,
        Self::WaiverAcceptance,
        Self::BoundaryChange,
        Self::ExceptionRenewal,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StablePromotion => "stable_promotion",
            Self::WaiverAcceptance => "waiver_acceptance",
            Self::BoundaryChange => "boundary_change",
            Self::ExceptionRenewal => "exception_renewal",
        }
    }

    /// Reader-facing decision label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::StablePromotion => "Stable promotion",
            Self::WaiverAcceptance => "Waiver acceptance",
            Self::BoundaryChange => "Boundary change",
            Self::ExceptionRenewal => "Exception renewal",
        }
    }

    /// The forum that exercises this decision right.
    pub const fn forum(self) -> Forum {
        match self {
            Self::StablePromotion => Forum::ShiproomForum,
            Self::WaiverAcceptance | Self::ExceptionRenewal => Forum::GovernanceCouncil,
            Self::BoundaryChange => Forum::ArchitectureForum,
        }
    }

    /// Owner role accountable for the decision — the role, never a person.
    pub const fn accountable_owner(self) -> &'static str {
        match self {
            Self::StablePromotion => "release_owner",
            Self::WaiverAcceptance | Self::ExceptionRenewal => "governance_owner",
            Self::BoundaryChange => "architecture_owner",
        }
    }

    /// The services this decision right governs, in canonical order.
    fn governed_services(self) -> Vec<Service> {
        match self {
            // Promotion and waiver / exception governance read the whole board.
            Self::StablePromotion | Self::WaiverAcceptance | Self::ExceptionRenewal => {
                Service::ALL.to_vec()
            }
            // A boundary change is scoped to package governance.
            Self::BoundaryChange => vec![Service::PackageGovernance],
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Measured result / fitness state
// ---------------------------------------------------------------------------------------------

/// The measured result of a nightly fitness run, before freshness and waiver overlays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FitnessMeasure {
    /// The fitness function passed.
    Pass,
    /// The fitness function passed with a warning threshold breach.
    Warn,
    /// The fitness function failed.
    Fail,
}

impl FitnessMeasure {
    /// Every measure, in declaration order.
    pub const ALL: [Self; 3] = [Self::Pass, Self::Warn, Self::Fail];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Warn => "warn",
            Self::Fail => "fail",
        }
    }

    /// Reader-facing measure label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pass => "Pass",
            Self::Warn => "Warn",
            Self::Fail => "Fail",
        }
    }

    /// The gate the measure alone implies.
    const fn gate(self) -> DescriptorGate {
        match self {
            Self::Pass => DescriptorGate::Governed,
            Self::Warn => DescriptorGate::Narrowed,
            Self::Fail => DescriptorGate::Blocked,
        }
    }
}

/// The colour-distinct state of a fitness tile, derived from its measured result, evidence freshness,
/// and waiver standing. The six states stay distinct so a waived or stale item never renders as a
/// clean pass, and an expired waiver reads worse than an active one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FitnessState {
    /// Fresh evidence, the function passed, no waiver — a clean pass.
    Passing,
    /// The function breached a warning threshold but did not fail.
    Warning,
    /// The function's evidence is stale; the result is no longer trusted.
    EvidenceStale,
    /// A failing / warning function is held under an accepted, in-date waiver.
    Waived,
    /// The waiver that held a failing function has expired; the failure is no longer covered.
    WaiverExpired,
    /// The function failed, or its evidence is expired / missing — a hard block.
    Blocked,
}

impl FitnessState {
    /// Every fitness state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Passing,
        Self::Warning,
        Self::EvidenceStale,
        Self::Waived,
        Self::WaiverExpired,
        Self::Blocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passing => "passing",
            Self::Warning => "warning",
            Self::EvidenceStale => "evidence_stale",
            Self::Waived => "waived",
            Self::WaiverExpired => "waiver_expired",
            Self::Blocked => "blocked",
        }
    }

    /// Reader-facing state label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Passing => "Passing",
            Self::Warning => "Warning",
            Self::EvidenceStale => "Evidence stale",
            Self::Waived => "Waived",
            Self::WaiverExpired => "Waiver expired",
            Self::Blocked => "Blocked",
        }
    }

    /// The release gate this state binds to. `passing` is governed; `warning`, `evidence_stale`, and
    /// `waived` narrow; `waiver_expired` and `blocked` block.
    pub const fn gate(self) -> DescriptorGate {
        match self {
            Self::Passing => DescriptorGate::Governed,
            Self::Warning | Self::EvidenceStale | Self::Waived => DescriptorGate::Narrowed,
            Self::WaiverExpired | Self::Blocked => DescriptorGate::Blocked,
        }
    }

    /// Severity rank (best→worst) used to pick the single worst representative state across a set of
    /// tiles. The order is gate-consistent: a more severe state never reads a weaker gate.
    const fn severity_rank(self) -> usize {
        match self {
            Self::Passing => 0,
            Self::Waived => 1,
            Self::EvidenceStale => 2,
            Self::Warning => 3,
            Self::WaiverExpired => 4,
            Self::Blocked => 5,
        }
    }

    /// True when this state never reads as a clean pass — every narrowed / blocked state.
    pub const fn is_clean_pass(self) -> bool {
        matches!(self, Self::Passing)
    }
}

/// Derives a tile's state from its measured result, evidence freshness, and waiver standing — the
/// single rule every tile, nightly row, service card, and decision card reads, so the board can never
/// disagree with itself. A waiver overlays the measured / freshness gate: an in-date waiver narrows a
/// failing function to `waived`, an expired waiver blocks it to `waiver_expired`.
fn derive_fitness_state(
    measure: FitnessMeasure,
    freshness: FreshnessState,
    waiver: Option<WaiverStanding>,
) -> FitnessState {
    if let Some(standing) = waiver {
        return match standing {
            WaiverStanding::Active | WaiverStanding::ExpiringSoon => FitnessState::Waived,
            WaiverStanding::Expired => FitnessState::WaiverExpired,
        };
    }
    match worse_gate(measure.gate(), freshness_gate(freshness)) {
        DescriptorGate::Governed => FitnessState::Passing,
        DescriptorGate::Narrowed => {
            if matches!(freshness, FreshnessState::Stale) {
                FitnessState::EvidenceStale
            } else {
                FitnessState::Warning
            }
        }
        DescriptorGate::Blocked => FitnessState::Blocked,
    }
}

// ---------------------------------------------------------------------------------------------
// Waivers
// ---------------------------------------------------------------------------------------------

/// The expiry standing of an accepted waiver. The dashboard reads the standing the governance
/// pipeline computed; it does not compute expiry from a clock of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaiverStanding {
    /// In date with headroom.
    Active,
    /// In date but inside its expiry-warning window.
    ExpiringSoon,
    /// Past its expiry; the waiver no longer holds.
    Expired,
}

impl WaiverStanding {
    /// Every standing, in declaration order.
    pub const ALL: [Self; 3] = [Self::Active, Self::ExpiringSoon, Self::Expired];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ExpiringSoon => "expiring_soon",
            Self::Expired => "expired",
        }
    }

    /// Reader-facing standing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::ExpiringSoon => "Expiring soon",
            Self::Expired => "Expired",
        }
    }

    /// Queue priority (most urgent first): expired waivers head the queue.
    const fn queue_priority(self) -> usize {
        match self {
            Self::Expired => 0,
            Self::ExpiringSoon => 1,
            Self::Active => 2,
        }
    }
}

/// The party responsible for clearing a waiver — a role, never a person.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaiverParty {
    /// The service owner must act.
    ServiceOwner,
    /// The governance owner must act.
    GovernanceOwner,
    /// The release owner must act.
    ReleaseOwner,
}

impl WaiverParty {
    /// Every party, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ServiceOwner,
        Self::GovernanceOwner,
        Self::ReleaseOwner,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServiceOwner => "service_owner",
            Self::GovernanceOwner => "governance_owner",
            Self::ReleaseOwner => "release_owner",
        }
    }
}

/// The action that clears an accepted waiver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaiverClearingAction {
    /// Fix the underlying regression and re-run the fitness function.
    RemediateAndReverify,
    /// Renew the waiver before its expiry.
    RenewWaiver,
    /// Accept the residual risk and close the waiver.
    AcceptResidualRisk,
    /// Enable a compensating control that removes the dependency.
    EnableCompensatingControl,
}

impl WaiverClearingAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RemediateAndReverify,
        Self::RenewWaiver,
        Self::AcceptResidualRisk,
        Self::EnableCompensatingControl,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemediateAndReverify => "remediate_and_reverify",
            Self::RenewWaiver => "renew_waiver",
            Self::AcceptResidualRisk => "accept_residual_risk",
            Self::EnableCompensatingControl => "enable_compensating_control",
        }
    }
}

/// The posture a decision-right card reads: whether the decision is currently exercisable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionPosture {
    /// The governed scope is clean; the decision is exercisable.
    Clear,
    /// The governed scope is narrowed; the decision needs review.
    Watch,
    /// The governed scope is blocked; the decision is held.
    Held,
}

impl DecisionPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 3] = [Self::Clear, Self::Watch, Self::Held];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Watch => "watch",
            Self::Held => "held",
        }
    }

    /// Reader-facing posture label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Clear => "Clear",
            Self::Watch => "Watch",
            Self::Held => "Held",
        }
    }

    /// The posture a gate decision implies.
    const fn from_gate(gate: DescriptorGate) -> Self {
        match gate {
            DescriptorGate::Governed => Self::Clear,
            DescriptorGate::Narrowed => Self::Watch,
            DescriptorGate::Blocked => Self::Held,
        }
    }
}

/// An evaluation / export action a governance overview offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DashboardAction {
    /// Inspect a single fitness tile, its measure, freshness, and waiver.
    InspectTile,
    /// Review the waiver-expiry queue.
    ReviewWaiverQueue,
    /// Open a service-ownership card.
    OpenServiceCard,
    /// Export the evaluation packet for offline review.
    ExportEvaluationPacket,
}

impl DashboardAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InspectTile,
        Self::ReviewWaiverQueue,
        Self::OpenServiceCard,
        Self::ExportEvaluationPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectTile => "inspect_tile",
            Self::ReviewWaiverQueue => "review_waiver_queue",
            Self::OpenServiceCard => "open_service_card",
            Self::ExportEvaluationPacket => "export_evaluation_packet",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Gate / posture helpers
// ---------------------------------------------------------------------------------------------

/// Maps a gate posture to the qualification floor it implies: governed stands at Stable, narrowed
/// floors at Beta, blocked at Unavailable.
const fn floor_for_gate(gate: DescriptorGate) -> QualificationClass {
    match gate {
        DescriptorGate::Governed => QualificationClass::Stable,
        DescriptorGate::Narrowed => QualificationClass::Beta,
        DescriptorGate::Blocked => QualificationClass::Unavailable,
    }
}

/// Restrictiveness rank of a gate posture (least restrictive first).
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

/// The gate a control's evidence freshness implies on its own: current keeps it governed, stale
/// narrows, expired / missing block.
const fn freshness_gate(freshness: FreshnessState) -> DescriptorGate {
    match freshness {
        FreshnessState::Current => DescriptorGate::Governed,
        FreshnessState::Stale => DescriptorGate::Narrowed,
        FreshnessState::Expired | FreshnessState::Missing => DescriptorGate::Blocked,
    }
}

/// Maps a gate posture to the coverage status it implies.
const fn gate_status(gate: DescriptorGate) -> ConsumerStatus {
    match gate {
        DescriptorGate::Governed => ConsumerStatus::Mapped,
        DescriptorGate::Narrowed => ConsumerStatus::Provisional,
        DescriptorGate::Blocked => ConsumerStatus::Unmapped,
    }
}

/// Position of a posture in the canonical (weakest→strongest) ordering.
fn posture_rank(posture: ClaimedPosture) -> usize {
    ClaimedPosture::ALL
        .iter()
        .position(|p| *p == posture)
        .unwrap_or(ClaimedPosture::ALL.len())
}

/// True when `function`'s scope is honored at or below `profile` — i.e. the profile is at least as
/// strong as the profile the function is required under.
fn function_applies(function: FitnessFunction, profile: ClaimedPosture) -> bool {
    posture_rank(profile) >= posture_rank(function.scope_profile())
}

/// Position of a fitness function in the canonical ordering.
fn function_rank(function: FitnessFunction) -> usize {
    FitnessFunction::ALL
        .iter()
        .position(|f| *f == function)
        .unwrap_or(FitnessFunction::ALL.len())
}

/// The single worst representative state across a set of tiles, or `passing` when empty.
fn worst_state(tiles: &[&FitnessTile]) -> FitnessState {
    tiles
        .iter()
        .map(|t| t.state)
        .max_by_key(|s| s.severity_rank())
        .unwrap_or(FitnessState::Passing)
}

/// The worst gate across a set of tiles, or `governed` when empty.
fn worst_gate(tiles: &[&FitnessTile]) -> DescriptorGate {
    tiles
        .iter()
        .map(|t| t.gate)
        .fold(DescriptorGate::Governed, worse_gate)
}

// ---------------------------------------------------------------------------------------------
// Fitness tiles
// ---------------------------------------------------------------------------------------------

/// One freshness-aware fitness tile: a protected fitness function, the corpus it was measured
/// against, its measured result, evidence freshness, waiver standing, and the colour-distinct state
/// the three together imply. The tile is stamped with the corpus and scope profile so an exported
/// tile can never be read as a pass in another context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FitnessTile {
    /// The fitness function.
    pub function: FitnessFunction,
    /// Reader-facing function label.
    pub function_label: String,
    /// The service that owns the function.
    pub service: Service,
    /// The weakest profile the function is required under.
    pub scope_profile: ClaimedPosture,
    /// The corpus the function was measured against — bound so the tile cannot be overgeneralised.
    pub corpus_id: String,
    /// The measured nightly result.
    pub measure: FitnessMeasure,
    /// Reader-facing measure label.
    pub measure_label: String,
    /// Freshness of the function's evidence.
    pub evidence_freshness: FreshnessState,
    /// Waiver standing, when the function is held under an accepted waiver.
    pub waiver_standing: Option<WaiverStanding>,
    /// The colour-distinct tile state, derived from measure, freshness, and waiver.
    pub state: FitnessState,
    /// Reader-facing state label.
    pub state_label: String,
    /// The release gate the state binds to.
    pub gate: DescriptorGate,
    /// Effective qualification implied by the gate.
    pub effective_qualification: QualificationClass,
    /// The evidence class that grounds the function.
    pub evidence_class: EvidenceClass,
    /// Owner role accountable for the function.
    pub owner_role: String,
    /// Decision forum that governs the function's service.
    pub forum: Forum,
    /// Repo-relative proof ref backing the function.
    pub proof_ref: String,
    /// Coverage status implied by the gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl FitnessTile {
    /// Builds a fitness tile, deriving every field from the function so a tile can never cite a field
    /// that drifts from it.
    fn new(
        function: FitnessFunction,
        measure: FitnessMeasure,
        freshness: FreshnessState,
        waiver_standing: Option<WaiverStanding>,
        corpus_id: &str,
    ) -> Self {
        let state = derive_fitness_state(measure, freshness, waiver_standing);
        let gate = state.gate();
        let status = gate_status(gate);
        Self {
            function,
            function_label: function.label().to_owned(),
            service: function.service(),
            scope_profile: function.scope_profile(),
            corpus_id: corpus_id.to_owned(),
            measure,
            measure_label: measure.label().to_owned(),
            evidence_freshness: freshness,
            waiver_standing,
            state,
            state_label: state.label().to_owned(),
            gate,
            effective_qualification: floor_for_gate(gate),
            evidence_class: function.evidence_class(),
            owner_role: function.owner_role().to_owned(),
            forum: function.service().forum(),
            proof_ref: function.proof_ref().to_owned(),
            status,
            signal: status.signal(),
            detail_message_id: format!(
                "{}tile.{}",
                M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX,
                function.as_str()
            ),
        }
    }

    /// The underlying gate the tile would read without any waiver overlay — used to prove a waiver is
    /// never attached to a passing function.
    fn underlying_gate(&self) -> DescriptorGate {
        worse_gate(self.measure.gate(), freshness_gate(self.evidence_freshness))
    }

    /// Validates the tile's invariants: every derived field matches the function, the state matches a
    /// fresh derivation, a waiver only sits on a non-passing function, and the message id carries the
    /// lane prefix.
    fn validate(&self) -> Vec<M5GovernanceDashboardViolation> {
        let mut out = Vec::new();
        if self.function_label != self.function.label()
            || self.service != self.function.service()
            || self.scope_profile != self.function.scope_profile()
            || self.evidence_class != self.function.evidence_class()
            || self.owner_role != self.function.owner_role()
            || self.forum != self.function.service().forum()
            || self.proof_ref != self.function.proof_ref()
            || self.measure_label != self.measure.label()
        {
            out.push(M5GovernanceDashboardViolation::TileFieldMismatch);
        }
        let expected =
            derive_fitness_state(self.measure, self.evidence_freshness, self.waiver_standing);
        if self.state != expected
            || self.state_label != expected.label()
            || self.gate != expected.gate()
            || self.effective_qualification != floor_for_gate(expected.gate())
            || self.status != gate_status(expected.gate())
            || self.signal != self.status.signal()
        {
            out.push(M5GovernanceDashboardViolation::TileStateDrift);
        }
        // A waiver may only hold a function that is not already a clean pass.
        if self.waiver_standing.is_some()
            && matches!(self.underlying_gate(), DescriptorGate::Governed)
        {
            out.push(M5GovernanceDashboardViolation::WaiverOnPassingFunction);
        }
        if self.proof_ref.trim().is_empty() || self.corpus_id.trim().is_empty() {
            out.push(M5GovernanceDashboardViolation::TileEvidenceMissing);
        }
        if !self
            .detail_message_id
            .starts_with(M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX)
        {
            out.push(M5GovernanceDashboardViolation::UnprefixedMessageId);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Nightly governance rows
// ---------------------------------------------------------------------------------------------

/// One nightly governance run record: the fitness function, its last run, the state that run read,
/// the measured result and freshness, and the passing streak. The row reads the same proof ref and
/// derived state the tile reads, so the run log and the board never disagree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NightlyGovernanceRow {
    /// The fitness function.
    pub function: FitnessFunction,
    /// Reader-facing function label.
    pub function_label: String,
    /// The corpus the run was measured against.
    pub corpus_id: String,
    /// The timestamp of the last nightly run.
    pub last_run_at: String,
    /// The state the last run read.
    pub run_state: FitnessState,
    /// Reader-facing run-state label.
    pub run_state_label: String,
    /// The measured result.
    pub measure: FitnessMeasure,
    /// Freshness of the run's evidence.
    pub evidence_freshness: FreshnessState,
    /// Consecutive passing nightly runs.
    pub consecutive_passing_runs: u32,
    /// The release gate the run state binds to.
    pub gate: DescriptorGate,
    /// Repo-relative proof ref backing the run.
    pub proof_ref: String,
    /// Coverage status implied by the gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX`].
    pub run_message_id: String,
}

impl NightlyGovernanceRow {
    /// Builds a nightly row from the tile and its run metadata.
    fn from_tile(tile: &FitnessTile, last_run_at: &str, consecutive_passing_runs: u32) -> Self {
        Self {
            function: tile.function,
            function_label: tile.function.label().to_owned(),
            corpus_id: tile.corpus_id.clone(),
            last_run_at: last_run_at.to_owned(),
            run_state: tile.state,
            run_state_label: tile.state.label().to_owned(),
            measure: tile.measure,
            evidence_freshness: tile.evidence_freshness,
            consecutive_passing_runs,
            gate: tile.gate,
            proof_ref: tile.proof_ref.clone(),
            status: tile.status,
            signal: tile.signal,
            run_message_id: format!(
                "{}nightly.{}",
                M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX,
                tile.function.as_str()
            ),
        }
    }

    /// Validates the row's invariants: it agrees with the tile it mirrors and carries the lane prefix.
    fn validate(&self, tile: &FitnessTile) -> Vec<M5GovernanceDashboardViolation> {
        let mut out = Vec::new();
        if self.function != tile.function
            || self.run_state != tile.state
            || self.run_state_label != tile.state.label()
            || self.measure != tile.measure
            || self.evidence_freshness != tile.evidence_freshness
            || self.gate != tile.gate
            || self.proof_ref != tile.proof_ref
            || self.corpus_id != tile.corpus_id
            || self.status != tile.status
            || self.signal != tile.signal
        {
            out.push(M5GovernanceDashboardViolation::NightlyRowDrift);
        }
        if self.last_run_at.trim().is_empty() {
            out.push(M5GovernanceDashboardViolation::NightlyRunMissing);
        }
        if !self
            .run_message_id
            .starts_with(M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX)
        {
            out.push(M5GovernanceDashboardViolation::UnprefixedMessageId);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Waiver-expiry queue rows
// ---------------------------------------------------------------------------------------------

/// Seed input for one accepted waiver, attached to a fitness function in the dashboard input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaiverSeed {
    /// The waiver's expiry standing.
    pub standing: WaiverStanding,
    /// The waiver's expiry date.
    pub expiry: String,
    /// Short rationale (no secrets / no private incident content).
    pub rationale: String,
    /// The party responsible for clearing the waiver.
    pub responsible_party: WaiverParty,
    /// The action that clears the waiver.
    pub action: WaiverClearingAction,
    /// The governance ticket the waiver rides.
    pub ticket_ref: String,
}

/// One waiver-expiry-queue row: a fitness function held under an accepted waiver, ordered by expiry
/// urgency, disclosing its standing, expiry, rationale, responsible party, the action that clears it,
/// and the governance ticket it rides.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaiverQueueRow {
    /// The waived fitness function.
    pub function: FitnessFunction,
    /// Reader-facing function label.
    pub function_label: String,
    /// The service that owns the function.
    pub service: Service,
    /// The corpus the waiver applies to.
    pub corpus_id: String,
    /// The waiver's expiry standing.
    pub queue_state: WaiverStanding,
    /// Reader-facing standing label.
    pub queue_state_label: String,
    /// The waiver's expiry date.
    pub expiry: String,
    /// Short rationale (no secrets).
    pub rationale: String,
    /// The party responsible for clearing the waiver.
    pub responsible_party: WaiverParty,
    /// The action that clears the waiver.
    pub action: WaiverClearingAction,
    /// The governance ticket the waiver rides.
    pub ticket_ref: String,
    /// The tile state the waiver produces — `waived` while in date, `waiver_expired` once expired.
    pub tile_state: FitnessState,
    /// Owner role accountable for the function.
    pub owner_role: String,
    /// Decision forum that governs the function's service.
    pub forum: Forum,
    /// Coverage status (waivers narrow while in date, block once expired).
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl WaiverQueueRow {
    /// Builds a queue row from a fitness function, its waiver seed, and the corpus.
    fn new(function: FitnessFunction, seed: &WaiverSeed, corpus_id: &str) -> Self {
        let tile_state = match seed.standing {
            WaiverStanding::Active | WaiverStanding::ExpiringSoon => FitnessState::Waived,
            WaiverStanding::Expired => FitnessState::WaiverExpired,
        };
        let status = gate_status(tile_state.gate());
        Self {
            function,
            function_label: function.label().to_owned(),
            service: function.service(),
            corpus_id: corpus_id.to_owned(),
            queue_state: seed.standing,
            queue_state_label: seed.standing.label().to_owned(),
            expiry: seed.expiry.clone(),
            rationale: seed.rationale.clone(),
            responsible_party: seed.responsible_party,
            action: seed.action,
            ticket_ref: seed.ticket_ref.clone(),
            tile_state,
            owner_role: function.owner_role().to_owned(),
            forum: function.service().forum(),
            status,
            signal: status.signal(),
            detail_message_id: format!(
                "{}waiver.{}",
                M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX,
                function.as_str()
            ),
        }
    }

    /// Validates the row's invariants: the disclosure fields are present and the message id carries
    /// the lane prefix.
    fn validate(&self) -> Vec<M5GovernanceDashboardViolation> {
        let mut out = Vec::new();
        if self.function_label != self.function.label()
            || self.service != self.function.service()
            || self.owner_role != self.function.owner_role()
            || self.forum != self.function.service().forum()
            || self.queue_state_label != self.queue_state.label()
        {
            out.push(M5GovernanceDashboardViolation::WaiverFieldMismatch);
        }
        let expected_state = match self.queue_state {
            WaiverStanding::Active | WaiverStanding::ExpiringSoon => FitnessState::Waived,
            WaiverStanding::Expired => FitnessState::WaiverExpired,
        };
        if self.tile_state != expected_state
            || self.status != gate_status(expected_state.gate())
            || self.signal != self.status.signal()
        {
            out.push(M5GovernanceDashboardViolation::WaiverStateDrift);
        }
        if self.expiry.trim().is_empty()
            || self.rationale.trim().is_empty()
            || self.ticket_ref.trim().is_empty()
        {
            out.push(M5GovernanceDashboardViolation::WaiverDisclosureIncomplete);
        }
        if !self
            .detail_message_id
            .starts_with(M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX)
        {
            out.push(M5GovernanceDashboardViolation::UnprefixedMessageId);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Service-ownership cards
// ---------------------------------------------------------------------------------------------

/// One service-ownership card: a governed service, its accountable owner and decision forum, the
/// fitness functions it owns, its worst tile state, and its open / expired waiver counts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceOwnershipCard {
    /// The service.
    pub service: Service,
    /// Reader-facing service label.
    pub service_label: String,
    /// Owner role accountable for the service.
    pub owner_role: String,
    /// Decision forum that governs the service.
    pub forum: Forum,
    /// Reader-facing forum label.
    pub forum_label: String,
    /// The fitness functions the service owns, in canonical order.
    pub governed_functions: Vec<FitnessFunction>,
    /// The worst tile state across the service's functions.
    pub worst_state: FitnessState,
    /// Reader-facing worst-state label.
    pub worst_state_label: String,
    /// Open (in-date) waivers on the service's functions.
    pub open_waiver_count: u32,
    /// Expired waivers on the service's functions.
    pub expired_waiver_count: u32,
    /// The release gate (worst gate among the service's functions).
    pub gate: DescriptorGate,
    /// Effective qualification implied by the gate.
    pub effective_qualification: QualificationClass,
    /// The proof refs of the service's functions, refs only.
    pub evidence_refs: Vec<String>,
    /// Coverage status implied by the gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX`].
    pub card_message_id: String,
}

impl ServiceOwnershipCard {
    /// Derives a service card from the tiles and waiver rows.
    fn derive(service: Service, tiles: &[FitnessTile], waivers: &[WaiverQueueRow]) -> Self {
        let functions = service.owned_functions();
        let owned: Vec<&FitnessTile> = tiles.iter().filter(|t| t.service == service).collect();
        let gate = worst_gate(&owned);
        let worst = worst_state(&owned);
        let open = waivers
            .iter()
            .filter(|w| w.service == service && w.queue_state != WaiverStanding::Expired)
            .count() as u32;
        let expired = waivers
            .iter()
            .filter(|w| w.service == service && w.queue_state == WaiverStanding::Expired)
            .count() as u32;
        let status = gate_status(gate);
        Self {
            service,
            service_label: service.label().to_owned(),
            owner_role: service.owner_role().to_owned(),
            forum: service.forum(),
            forum_label: service.forum().label().to_owned(),
            governed_functions: functions.clone(),
            worst_state: worst,
            worst_state_label: worst.label().to_owned(),
            open_waiver_count: open,
            expired_waiver_count: expired,
            gate,
            effective_qualification: floor_for_gate(gate),
            evidence_refs: functions.iter().map(|f| f.proof_ref().to_owned()).collect(),
            status,
            signal: status.signal(),
            card_message_id: format!(
                "{}service.{}",
                M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX,
                service.as_str()
            ),
        }
    }

    /// True when every owned function is a clean pass.
    pub fn is_clean(&self) -> bool {
        matches!(self.gate, DescriptorGate::Governed)
    }

    /// Validates the card against a fresh derivation.
    fn validate(
        &self,
        tiles: &[FitnessTile],
        waivers: &[WaiverQueueRow],
    ) -> Vec<M5GovernanceDashboardViolation> {
        let mut out = Vec::new();
        if Self::derive(self.service, tiles, waivers) != *self {
            out.push(M5GovernanceDashboardViolation::ServiceCardDrift);
        }
        if self.owner_role.trim().is_empty() {
            out.push(M5GovernanceDashboardViolation::ServiceOwnerMissing);
        }
        if !self
            .card_message_id
            .starts_with(M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX)
        {
            out.push(M5GovernanceDashboardViolation::UnprefixedMessageId);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Decision-right cards
// ---------------------------------------------------------------------------------------------

/// One decision-right card: a governed decision, the forum that decides it, the accountable owner,
/// the services it governs, and whether the decision is currently exercisable or held because its
/// scope is blocked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionRightCard {
    /// The decision right.
    pub decision: DecisionRight,
    /// Reader-facing decision label.
    pub decision_label: String,
    /// The forum that exercises the decision.
    pub forum: Forum,
    /// Reader-facing forum label.
    pub forum_label: String,
    /// Owner role accountable for the decision.
    pub accountable_owner: String,
    /// The services the decision governs, in canonical order.
    pub governed_services: Vec<Service>,
    /// The number of fitness functions in the governed scope.
    pub governed_function_count: u32,
    /// The worst tile state across the governed scope.
    pub worst_state: FitnessState,
    /// Reader-facing worst-state label.
    pub worst_state_label: String,
    /// Whether the decision is currently exercisable.
    pub posture: DecisionPosture,
    /// Reader-facing posture label.
    pub posture_label: String,
    /// The number of blocked functions in the governed scope.
    pub blocking_function_count: u32,
    /// Open (in-date) waivers in the governed scope.
    pub open_waiver_count: u32,
    /// The release gate (worst gate across the governed scope).
    pub gate: DescriptorGate,
    /// Effective qualification implied by the gate.
    pub effective_qualification: QualificationClass,
    /// The proof refs in the governed scope, refs only.
    pub evidence_refs: Vec<String>,
    /// Coverage status implied by the gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX`].
    pub card_message_id: String,
}

impl DecisionRightCard {
    /// Derives a decision-right card from the tiles and waiver rows.
    fn derive(decision: DecisionRight, tiles: &[FitnessTile], waivers: &[WaiverQueueRow]) -> Self {
        let services = decision.governed_services();
        let scope: Vec<&FitnessTile> = tiles
            .iter()
            .filter(|t| services.contains(&t.service))
            .collect();
        let gate = worst_gate(&scope);
        let worst = worst_state(&scope);
        let blocking = scope
            .iter()
            .filter(|t| matches!(t.gate, DescriptorGate::Blocked))
            .count() as u32;
        let open = waivers
            .iter()
            .filter(|w| services.contains(&w.service) && w.queue_state != WaiverStanding::Expired)
            .count() as u32;
        let posture = DecisionPosture::from_gate(gate);
        let status = gate_status(gate);
        let mut evidence_refs: Vec<String> = scope.iter().map(|t| t.proof_ref.clone()).collect();
        evidence_refs.sort();
        evidence_refs.dedup();
        Self {
            decision,
            decision_label: decision.label().to_owned(),
            forum: decision.forum(),
            forum_label: decision.forum().label().to_owned(),
            accountable_owner: decision.accountable_owner().to_owned(),
            governed_services: services,
            governed_function_count: scope.len() as u32,
            worst_state: worst,
            worst_state_label: worst.label().to_owned(),
            posture,
            posture_label: posture.label().to_owned(),
            blocking_function_count: blocking,
            open_waiver_count: open,
            gate,
            effective_qualification: floor_for_gate(gate),
            evidence_refs,
            status,
            signal: status.signal(),
            card_message_id: format!(
                "{}decision.{}",
                M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX,
                decision.as_str()
            ),
        }
    }

    /// True when the decision is currently exercisable.
    pub fn is_exercisable(&self) -> bool {
        matches!(self.posture, DecisionPosture::Clear)
    }

    /// Validates the card against a fresh derivation.
    fn validate(
        &self,
        tiles: &[FitnessTile],
        waivers: &[WaiverQueueRow],
    ) -> Vec<M5GovernanceDashboardViolation> {
        let mut out = Vec::new();
        if Self::derive(self.decision, tiles, waivers) != *self {
            out.push(M5GovernanceDashboardViolation::DecisionCardDrift);
        }
        if self.accountable_owner.trim().is_empty() {
            out.push(M5GovernanceDashboardViolation::DecisionOwnerMissing);
        }
        if self.posture != DecisionPosture::from_gate(self.gate) {
            out.push(M5GovernanceDashboardViolation::DecisionPostureDrift);
        }
        if !self
            .card_message_id
            .starts_with(M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX)
        {
            out.push(M5GovernanceDashboardViolation::UnprefixedMessageId);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Per-profile overviews
// ---------------------------------------------------------------------------------------------

/// Count of fitness tiles by state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TileStateCounts {
    /// Tiles passing.
    pub passing: u32,
    /// Tiles warning.
    pub warning: u32,
    /// Tiles whose evidence is stale.
    pub evidence_stale: u32,
    /// Tiles held under an in-date waiver.
    pub waived: u32,
    /// Tiles whose waiver expired.
    pub waiver_expired: u32,
    /// Tiles blocked.
    pub blocked: u32,
}

/// Count of evidence by freshness across a set of functions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessCounts {
    /// Evidence current.
    pub current: u32,
    /// Evidence stale.
    pub stale: u32,
    /// Evidence expired.
    pub expired: u32,
    /// Evidence missing.
    pub missing: u32,
}

/// One per-profile governance overview: the applicable fitness functions, the tile-state and
/// evidence-freshness summaries, the open / expired waiver counts, the strongest honored posture, and
/// the actions an evaluator can take from this profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceOverview {
    /// The deployment profile this overview is shown under.
    pub profile: ClaimedPosture,
    /// The corpus the overview is scoped to.
    pub corpus_id: String,
    /// The fitness functions applicable to this profile, in canonical order.
    pub applicable_functions: Vec<FitnessFunction>,
    /// Tile-state counts over the applicable functions.
    pub tile_state_counts: TileStateCounts,
    /// Evidence-freshness counts over the applicable functions.
    pub evidence_freshness_counts: FreshnessCounts,
    /// Open (in-date) waivers affecting an applicable function.
    pub open_waiver_count: u32,
    /// Expired waivers affecting an applicable function.
    pub expired_waiver_count: u32,
    /// The strongest posture every applicable function is passing at — never above the profile.
    pub effective_posture: ClaimedPosture,
    /// Gate decision for the profile (worst gate among the applicable functions).
    pub gate_decision: DescriptorGate,
    /// Effective qualification implied by the gate decision.
    pub effective_qualification: QualificationClass,
    /// Coverage status (mirrors [`Self::gate_decision`]).
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// The actions offered from this profile.
    pub dashboard_actions: Vec<DashboardAction>,
    /// Stable message id; prefixed [`M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
}

impl GovernanceOverview {
    /// Derives a per-profile overview from the tiles and waiver rows.
    fn derive(
        profile: ClaimedPosture,
        corpus_id: &str,
        tiles: &[FitnessTile],
        waivers: &[WaiverQueueRow],
    ) -> Self {
        let applicable: Vec<FitnessFunction> = FitnessFunction::ALL
            .iter()
            .copied()
            .filter(|f| function_applies(*f, profile))
            .collect();
        let applicable_tiles: Vec<&FitnessTile> = applicable
            .iter()
            .filter_map(|f| tiles.iter().find(|t| t.function == *f))
            .collect();

        let mut counts = TileStateCounts {
            passing: 0,
            warning: 0,
            evidence_stale: 0,
            waived: 0,
            waiver_expired: 0,
            blocked: 0,
        };
        let mut fresh = FreshnessCounts {
            current: 0,
            stale: 0,
            expired: 0,
            missing: 0,
        };
        for tile in &applicable_tiles {
            match tile.state {
                FitnessState::Passing => counts.passing += 1,
                FitnessState::Warning => counts.warning += 1,
                FitnessState::EvidenceStale => counts.evidence_stale += 1,
                FitnessState::Waived => counts.waived += 1,
                FitnessState::WaiverExpired => counts.waiver_expired += 1,
                FitnessState::Blocked => counts.blocked += 1,
            }
            match tile.evidence_freshness {
                FreshnessState::Current => fresh.current += 1,
                FreshnessState::Stale => fresh.stale += 1,
                FreshnessState::Expired => fresh.expired += 1,
                FreshnessState::Missing => fresh.missing += 1,
            }
        }

        let open = waivers
            .iter()
            .filter(|w| {
                applicable.contains(&w.function) && w.queue_state != WaiverStanding::Expired
            })
            .count() as u32;
        let expired = waivers
            .iter()
            .filter(|w| {
                applicable.contains(&w.function) && w.queue_state == WaiverStanding::Expired
            })
            .count() as u32;

        let gate_decision = worst_gate(&applicable_tiles);
        let effective_posture = strongest_honored_posture(profile, tiles);
        let status = gate_status(gate_decision);
        Self {
            profile,
            corpus_id: corpus_id.to_owned(),
            applicable_functions: applicable,
            tile_state_counts: counts,
            evidence_freshness_counts: fresh,
            open_waiver_count: open,
            expired_waiver_count: expired,
            effective_posture,
            gate_decision,
            effective_qualification: floor_for_gate(gate_decision),
            status,
            signal: status.signal(),
            dashboard_actions: DashboardAction::ALL.to_vec(),
            summary_message_id: format!(
                "{}overview.{}",
                M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX,
                profile.as_str()
            ),
        }
    }

    /// Validates the overview against a fresh derivation and its non-overstatement invariant.
    fn validate(
        &self,
        corpus_id: &str,
        tiles: &[FitnessTile],
        waivers: &[WaiverQueueRow],
    ) -> Vec<M5GovernanceDashboardViolation> {
        let mut out = Vec::new();
        if Self::derive(self.profile, corpus_id, tiles, waivers) != *self {
            out.push(M5GovernanceDashboardViolation::OverviewDrift);
        }
        if posture_rank(self.effective_posture) > posture_rank(self.profile) {
            out.push(M5GovernanceDashboardViolation::OverviewOverstatesPosture);
        }
        if self.corpus_id != corpus_id {
            out.push(M5GovernanceDashboardViolation::CorpusIdentityMissing);
        }
        if self.effective_qualification != floor_for_gate(self.gate_decision)
            || self.status != gate_status(self.gate_decision)
            || self.signal != self.status.signal()
        {
            out.push(M5GovernanceDashboardViolation::OverviewGateDrift);
        }
        if !self
            .summary_message_id
            .starts_with(M5_GOVERNANCE_DASHBOARD_MESSAGE_ID_PREFIX)
        {
            out.push(M5GovernanceDashboardViolation::UnprefixedMessageId);
        }
        out
    }
}

/// The strongest posture at or below `profile` whose every applicable function tile is passing. This
/// is the overview's honest effective posture: it auto-narrows below the profile the moment a
/// function it would imply is not passing, and never reads above the profile.
fn strongest_honored_posture(profile: ClaimedPosture, tiles: &[FitnessTile]) -> ClaimedPosture {
    let mut best = ClaimedPosture::ALL[0];
    for &candidate in ClaimedPosture::ALL.iter() {
        if posture_rank(candidate) > posture_rank(profile) {
            continue;
        }
        let all_passing = FitnessFunction::ALL
            .iter()
            .copied()
            .filter(|f| function_applies(*f, candidate))
            .all(|f| {
                tiles
                    .iter()
                    .find(|t| t.function == f)
                    .is_some_and(|t| t.state.is_clean_pass())
            });
        if all_passing && posture_rank(candidate) >= posture_rank(best) {
            best = candidate;
        }
    }
    best
}

// ---------------------------------------------------------------------------------------------
// Evaluation packet (export)
// ---------------------------------------------------------------------------------------------

/// One tile entry in the exported evaluation packet — the same state vocabulary the tile shows,
/// reduced to refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationTileEntry {
    /// The fitness function token.
    pub function: FitnessFunction,
    /// The owning service.
    pub service: Service,
    /// The scope profile.
    pub scope_profile: ClaimedPosture,
    /// The tile state token.
    pub state: FitnessState,
    /// The release gate.
    pub gate: DescriptorGate,
    /// Effective qualification.
    pub effective_qualification: QualificationClass,
    /// Evidence freshness.
    pub evidence_freshness: FreshnessState,
    /// Owner role.
    pub owner_role: String,
    /// Decision forum.
    pub forum: Forum,
    /// Proof ref (refs only).
    pub proof_ref: String,
}

/// One waiver entry in the exported evaluation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationWaiverEntry {
    /// The waived function token.
    pub function: FitnessFunction,
    /// The expiry standing.
    pub queue_state: WaiverStanding,
    /// The expiry date.
    pub expiry: String,
    /// The responsible party.
    pub responsible_party: WaiverParty,
    /// The clearing action.
    pub action: WaiverClearingAction,
    /// The tile state the waiver produces.
    pub tile_state: FitnessState,
}

/// One service entry in the exported evaluation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationServiceEntry {
    /// The service token.
    pub service: Service,
    /// Owner role.
    pub owner_role: String,
    /// Decision forum.
    pub forum: Forum,
    /// The worst tile state.
    pub worst_state: FitnessState,
    /// The release gate.
    pub gate: DescriptorGate,
    /// Open (in-date) waivers.
    pub open_waiver_count: u32,
    /// Expired waivers.
    pub expired_waiver_count: u32,
}

/// One decision entry in the exported evaluation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationDecisionEntry {
    /// The decision token.
    pub decision: DecisionRight,
    /// The forum.
    pub forum: Forum,
    /// Owner role.
    pub accountable_owner: String,
    /// The posture.
    pub posture: DecisionPosture,
    /// The worst tile state.
    pub worst_state: FitnessState,
    /// The release gate.
    pub gate: DescriptorGate,
    /// The number of blocked functions in scope.
    pub blocking_function_count: u32,
}

/// The exported evaluation packet: the tiles, waivers, service cards, and decision cards reduced to
/// the exact state and proof vocabulary the in-product dashboard shows, so an exported pack and the
/// live UI can never read differently. It is stamped with the corpus so it cannot be overgeneralised.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceEvaluationPacket {
    /// Record kind; must equal [`M5_GOVERNANCE_EVALUATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; mirrors the parent packet.
    pub schema_version: u32,
    /// Stable evaluation-packet id.
    pub packet_id: String,
    /// The dashboard packet this export was generated from.
    pub generated_from: String,
    /// The corpus the export is scoped to.
    pub corpus_id: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The tile entries.
    pub tiles: Vec<EvaluationTileEntry>,
    /// The waiver entries.
    pub waivers: Vec<EvaluationWaiverEntry>,
    /// The service entries.
    pub services: Vec<EvaluationServiceEntry>,
    /// The decision entries.
    pub decision_rights: Vec<EvaluationDecisionEntry>,
    /// The controlled vocabulary the entries draw from.
    pub vocabulary: GovernanceDashboardVocabulary,
    /// Packet redaction class token.
    pub redaction_class_token: String,
}

impl GovernanceEvaluationPacket {
    /// Builds the evaluation packet from the dashboard parts.
    #[allow(clippy::too_many_arguments)]
    fn derive(
        packet_id: &str,
        generated_from: &str,
        corpus_id: &str,
        evaluated_at: &str,
        redaction_class_token: &str,
        tiles: &[FitnessTile],
        waivers: &[WaiverQueueRow],
        services: &[ServiceOwnershipCard],
        decisions: &[DecisionRightCard],
    ) -> Self {
        Self {
            record_kind: M5_GOVERNANCE_EVALUATION_RECORD_KIND.to_owned(),
            schema_version: M5_GOVERNANCE_DASHBOARD_SCHEMA_VERSION,
            packet_id: packet_id.to_owned(),
            generated_from: generated_from.to_owned(),
            corpus_id: corpus_id.to_owned(),
            evaluated_at: evaluated_at.to_owned(),
            tiles: tiles
                .iter()
                .map(|t| EvaluationTileEntry {
                    function: t.function,
                    service: t.service,
                    scope_profile: t.scope_profile,
                    state: t.state,
                    gate: t.gate,
                    effective_qualification: t.effective_qualification,
                    evidence_freshness: t.evidence_freshness,
                    owner_role: t.owner_role.clone(),
                    forum: t.forum,
                    proof_ref: t.proof_ref.clone(),
                })
                .collect(),
            waivers: waivers
                .iter()
                .map(|w| EvaluationWaiverEntry {
                    function: w.function,
                    queue_state: w.queue_state,
                    expiry: w.expiry.clone(),
                    responsible_party: w.responsible_party,
                    action: w.action,
                    tile_state: w.tile_state,
                })
                .collect(),
            services: services
                .iter()
                .map(|s| EvaluationServiceEntry {
                    service: s.service,
                    owner_role: s.owner_role.clone(),
                    forum: s.forum,
                    worst_state: s.worst_state,
                    gate: s.gate,
                    open_waiver_count: s.open_waiver_count,
                    expired_waiver_count: s.expired_waiver_count,
                })
                .collect(),
            decision_rights: decisions
                .iter()
                .map(|d| EvaluationDecisionEntry {
                    decision: d.decision,
                    forum: d.forum,
                    accountable_owner: d.accountable_owner.clone(),
                    posture: d.posture,
                    worst_state: d.worst_state,
                    gate: d.gate,
                    blocking_function_count: d.blocking_function_count,
                })
                .collect(),
            vocabulary: GovernanceDashboardVocabulary::canonical(),
            redaction_class_token: redaction_class_token.to_owned(),
        }
    }

    /// Deterministic export-safe JSON for the evaluation packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 governance evaluation packet serializes")
    }

    /// True when every token the packet carries is a member of the canonical vocabulary.
    fn reuses_canonical_vocabulary(&self) -> bool {
        if !self.vocabulary.matches_canonical() {
            return false;
        }
        let vocab = &self.vocabulary;
        self.tiles.iter().all(|t| {
            vocab
                .fitness_functions
                .contains(&t.function.as_str().to_owned())
                && vocab.fitness_states.contains(&t.state.as_str().to_owned())
        }) && self.waivers.iter().all(|w| {
            vocab
                .waiver_standings
                .contains(&w.queue_state.as_str().to_owned())
                && vocab
                    .fitness_states
                    .contains(&w.tile_state.as_str().to_owned())
        }) && self.services.iter().all(|s| {
            vocab.services.contains(&s.service.as_str().to_owned())
                && vocab
                    .fitness_states
                    .contains(&s.worst_state.as_str().to_owned())
        }) && self.decision_rights.iter().all(|d| {
            vocab
                .decision_rights
                .contains(&d.decision.as_str().to_owned())
                && vocab
                    .decision_postures
                    .contains(&d.posture.as_str().to_owned())
        })
    }
}

// ---------------------------------------------------------------------------------------------
// Vocabulary / summary / conformance
// ---------------------------------------------------------------------------------------------

/// Self-describing controlled-vocabulary set so the packet resolves every token it carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceDashboardVocabulary {
    /// Fitness-function tokens.
    pub fitness_functions: Vec<String>,
    /// Service tokens.
    pub services: Vec<String>,
    /// Forum tokens.
    pub forums: Vec<String>,
    /// Decision-right tokens.
    pub decision_rights: Vec<String>,
    /// Fitness-measure tokens.
    pub fitness_measures: Vec<String>,
    /// Fitness-state tokens.
    pub fitness_states: Vec<String>,
    /// Waiver-standing tokens.
    pub waiver_standings: Vec<String>,
    /// Waiver-party tokens.
    pub waiver_parties: Vec<String>,
    /// Waiver-action tokens.
    pub waiver_actions: Vec<String>,
    /// Decision-posture tokens.
    pub decision_postures: Vec<String>,
    /// Dashboard-action tokens.
    pub dashboard_actions: Vec<String>,
    /// Deployment-profile tokens.
    pub deployment_profiles: Vec<String>,
    /// Evidence-class tokens.
    pub evidence_classes: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Qualification-class tokens.
    pub qualification_classes: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
}

impl GovernanceDashboardVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            fitness_functions: tokens(&FitnessFunction::ALL, |f| f.as_str()),
            services: tokens(&Service::ALL, |s| s.as_str()),
            forums: tokens(&Forum::ALL, |f| f.as_str()),
            decision_rights: tokens(&DecisionRight::ALL, |d| d.as_str()),
            fitness_measures: tokens(&FitnessMeasure::ALL, |m| m.as_str()),
            fitness_states: tokens(&FitnessState::ALL, |s| s.as_str()),
            waiver_standings: tokens(&WaiverStanding::ALL, |s| s.as_str()),
            waiver_parties: tokens(&WaiverParty::ALL, |p| p.as_str()),
            waiver_actions: tokens(&WaiverClearingAction::ALL, |a| a.as_str()),
            decision_postures: tokens(&DecisionPosture::ALL, |p| p.as_str()),
            dashboard_actions: tokens(&DashboardAction::ALL, |a| a.as_str()),
            deployment_profiles: tokens(&ClaimedPosture::ALL, |p| p.as_str()),
            evidence_classes: tokens(&EvidenceClass::ALL, |c| c.as_str()),
            freshness_states: tokens(&FreshnessState::ALL, |f| f.as_str()),
            qualification_classes: tokens(&QualificationClass::ALL, |q| q.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |g| g.as_str()),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Compact governance-dashboard summary — the scoreboard the overview and exports read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceDashboardSummary {
    /// Total fitness tiles.
    pub total_functions: u32,
    /// Tiles passing.
    pub passing: u32,
    /// Tiles warning.
    pub warning: u32,
    /// Tiles whose evidence is stale.
    pub evidence_stale: u32,
    /// Tiles waived (in date).
    pub waived: u32,
    /// Tiles whose waiver expired.
    pub waiver_expired: u32,
    /// Tiles blocked.
    pub blocked: u32,
    /// Total services.
    pub total_services: u32,
    /// Services whose every function is a clean pass.
    pub clean_services: u32,
    /// Total decision rights.
    pub total_decision_rights: u32,
    /// Decision rights currently exercisable.
    pub exercisable_decisions: u32,
    /// Total waivers (open + expired).
    pub total_waivers: u32,
    /// Open (in-date) waivers.
    pub open_waivers: u32,
    /// Expired waivers.
    pub expired_waivers: u32,
    /// Total deployment profiles.
    pub total_profiles: u32,
    /// Profiles whose claimed posture is fully honored.
    pub honored_profiles: u32,
    /// True when at least one tile is blocked (a hard fail or an expired waiver).
    pub blocks_stable_promotion: bool,
}

/// Conformance review. Every flag is a hard invariant; all must hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceDashboardConformance {
    /// Every tile derives its state from measure, freshness, and waiver.
    pub tile_state_derived_from_inputs: bool,
    /// Freshness and ownership are first-class on every tile and card.
    pub freshness_and_ownership_first_class: bool,
    /// No waived or stale tile renders as a clean pass.
    pub waived_or_stale_never_clean_pass: bool,
    /// Stale evidence narrows the tiles that read it deterministically.
    pub stale_evidence_narrows_deterministically: bool,
    /// Missing / expired evidence and expired waivers block Stable promotion.
    pub missing_or_expired_blocks_stable_promotion: bool,
    /// Every waiver-queue row discloses expiry, party, and a clearing action.
    pub waiver_queue_discloses_expiry_and_action: bool,
    /// Every service card binds an owner and a forum.
    pub service_cards_bind_owner_and_forum: bool,
    /// Every decision card binds an owner and a forum.
    pub decision_cards_bind_owner_and_forum: bool,
    /// No overview reads a posture above its profile.
    pub overview_effective_posture_never_overstated: bool,
    /// Every dashboard surface is bound to the packet's corpus identity.
    pub corpus_identity_bound: bool,
    /// The exported evaluation packet reuses the in-product state and proof vocabulary.
    pub evaluation_packet_reuses_ui_vocabulary: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// The export carries no raw provider material — proof lineage stays refs-only.
    pub export_carries_no_raw_material: bool,
}

impl GovernanceDashboardConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.tile_state_derived_from_inputs
            && self.freshness_and_ownership_first_class
            && self.waived_or_stale_never_clean_pass
            && self.stale_evidence_narrows_deterministically
            && self.missing_or_expired_blocks_stable_promotion
            && self.waiver_queue_discloses_expiry_and_action
            && self.service_cards_bind_owner_and_forum
            && self.decision_cards_bind_owner_and_forum
            && self.overview_effective_posture_never_overstated
            && self.corpus_identity_bound
            && self.evaluation_packet_reuses_ui_vocabulary
            && self.controlled_enums_frozen
            && self.export_carries_no_raw_material
    }
}

// ---------------------------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------------------------

/// Seed input for one fitness function's nightly result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FitnessFunctionState {
    /// The fitness function.
    pub function: FitnessFunction,
    /// The measured nightly result.
    pub measure: FitnessMeasure,
    /// Freshness of the function's evidence.
    pub freshness: FreshnessState,
    /// The timestamp of the last nightly run.
    pub last_run_at: String,
    /// Consecutive passing nightly runs.
    pub consecutive_passing_runs: u32,
    /// An accepted waiver holding the function, when present.
    pub waiver: Option<WaiverSeed>,
}

/// Constructor input for [`M5GovernanceDashboard::new`]. The only raw inputs are each function's
/// measured result, evidence freshness, run metadata, and accepted waiver; everything else is derived.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5GovernanceDashboardInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The corpus identity the dashboard was measured against.
    pub corpus_id: String,
    /// Human-readable corpus label.
    pub corpus_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// Each fitness function's nightly result.
    pub function_states: Vec<FitnessFunctionState>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable governance-dashboard truth packet: the per-profile
/// overviews, the freshness-aware fitness tiles, the nightly governance rows, the waiver-expiry
/// queue, the service-ownership cards, the decision-right cards, the exported evaluation packet, the
/// controlled vocabulary, a summary, and a conformance review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GovernanceDashboard {
    /// Record kind; must equal [`M5_GOVERNANCE_DASHBOARD_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_GOVERNANCE_DASHBOARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The corpus identity the dashboard was measured against.
    pub corpus_id: String,
    /// Human-readable corpus label.
    pub corpus_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The per-profile overviews, in profile order.
    pub overviews: Vec<GovernanceOverview>,
    /// The freshness-aware fitness tiles, in function order.
    pub fitness_tiles: Vec<FitnessTile>,
    /// The nightly governance rows, in function order.
    pub nightly_rows: Vec<NightlyGovernanceRow>,
    /// The waiver-expiry queue, ordered by expiry urgency.
    pub waiver_queue: Vec<WaiverQueueRow>,
    /// The service-ownership cards, in service order.
    pub service_cards: Vec<ServiceOwnershipCard>,
    /// The decision-right cards, in decision order.
    pub decision_right_cards: Vec<DecisionRightCard>,
    /// The exported evaluation packet.
    pub evaluation_packet: GovernanceEvaluationPacket,
    /// Controlled-vocabulary set.
    pub vocabulary: GovernanceDashboardVocabulary,
    /// Compact summary.
    pub summary: GovernanceDashboardSummary,
    /// Conformance review block.
    pub conformance: GovernanceDashboardConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5GovernanceDashboard {
    /// Builds a governance-dashboard packet from seed input, deriving the tiles from the nightly
    /// results, the cards and overviews from the tiles, and the summary / conformance / evaluation
    /// packet from all of them.
    pub fn new(input: M5GovernanceDashboardInput) -> Self {
        // Fitness tiles and nightly rows, in canonical function order.
        let mut states = input.function_states.clone();
        states.sort_by_key(|s| function_rank(s.function));
        states.dedup_by_key(|s| s.function);

        let fitness_tiles: Vec<FitnessTile> = states
            .iter()
            .map(|s| {
                FitnessTile::new(
                    s.function,
                    s.measure,
                    s.freshness,
                    s.waiver.as_ref().map(|w| w.standing),
                    &input.corpus_id,
                )
            })
            .collect();

        let nightly_rows: Vec<NightlyGovernanceRow> = states
            .iter()
            .filter_map(|s| {
                fitness_tiles
                    .iter()
                    .find(|t| t.function == s.function)
                    .map(|tile| {
                        NightlyGovernanceRow::from_tile(
                            tile,
                            &s.last_run_at,
                            s.consecutive_passing_runs,
                        )
                    })
            })
            .collect();

        // Waiver queue, ordered by expiry urgency (expired first), then function order.
        let mut waiver_queue: Vec<WaiverQueueRow> = states
            .iter()
            .filter_map(|s| {
                s.waiver
                    .as_ref()
                    .map(|w| WaiverQueueRow::new(s.function, w, &input.corpus_id))
            })
            .collect();
        waiver_queue.sort_by(|a, b| {
            a.queue_state
                .queue_priority()
                .cmp(&b.queue_state.queue_priority())
                .then(function_rank(a.function).cmp(&function_rank(b.function)))
        });

        let service_cards: Vec<ServiceOwnershipCard> = Service::ALL
            .iter()
            .map(|s| ServiceOwnershipCard::derive(*s, &fitness_tiles, &waiver_queue))
            .collect();

        let decision_right_cards: Vec<DecisionRightCard> = DecisionRight::ALL
            .iter()
            .map(|d| DecisionRightCard::derive(*d, &fitness_tiles, &waiver_queue))
            .collect();

        let overviews: Vec<GovernanceOverview> = ClaimedPosture::ALL
            .iter()
            .map(|p| {
                GovernanceOverview::derive(*p, &input.corpus_id, &fitness_tiles, &waiver_queue)
            })
            .collect();

        let evaluation_packet = GovernanceEvaluationPacket::derive(
            &format!("{}:eval", input.packet_id),
            &input.packet_id,
            &input.corpus_id,
            &input.evaluated_at,
            &input.redaction_class_token,
            &fitness_tiles,
            &waiver_queue,
            &service_cards,
            &decision_right_cards,
        );

        let summary = derive_summary(
            &fitness_tiles,
            &waiver_queue,
            &service_cards,
            &decision_right_cards,
        );
        let conformance = derive_conformance(
            &input.corpus_id,
            &fitness_tiles,
            &nightly_rows,
            &waiver_queue,
            &service_cards,
            &decision_right_cards,
            &overviews,
            &evaluation_packet,
        );

        Self {
            record_kind: M5_GOVERNANCE_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_GOVERNANCE_DASHBOARD_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            corpus_id: input.corpus_id,
            corpus_label: input.corpus_label,
            evaluated_at: input.evaluated_at,
            overviews,
            fitness_tiles,
            nightly_rows,
            waiver_queue,
            service_cards,
            decision_right_cards,
            evaluation_packet,
            vocabulary: GovernanceDashboardVocabulary::canonical(),
            summary,
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release automation must hold Stable promotion — at least one tile is blocked.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.summary.blocks_stable_promotion
    }

    /// Finds a fitness tile by function.
    pub fn tile(&self, function: FitnessFunction) -> Option<&FitnessTile> {
        self.fitness_tiles.iter().find(|t| t.function == function)
    }

    /// Finds a service card by service.
    pub fn service_card(&self, service: Service) -> Option<&ServiceOwnershipCard> {
        self.service_cards.iter().find(|c| c.service == service)
    }

    /// Finds a decision-right card by decision.
    pub fn decision_card(&self, decision: DecisionRight) -> Option<&DecisionRightCard> {
        self.decision_right_cards
            .iter()
            .find(|c| c.decision == decision)
    }

    /// Finds a per-profile overview by profile.
    pub fn overview(&self, profile: ClaimedPosture) -> Option<&GovernanceOverview> {
        self.overviews.iter().find(|o| o.profile == profile)
    }

    /// Renders the packet for a generation channel. The output is identical for every channel — the
    /// channel parameter exists only to prove desktop, CLI / headless, and offline / mirror
    /// generation produce byte-identical output.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn render_for_channel(&self, _channel: GovernanceDashboardChannel) -> String {
        self.export_safe_json()
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 governance dashboard serializes")
    }

    /// The exported evaluation packet's JSON.
    pub fn render_evaluation_packet(&self) -> String {
        self.evaluation_packet.export_safe_json()
    }

    /// Deterministic, machine-readable fitness-tile matrix CSV: one row per fitness tile, naming the
    /// function, service, corpus, scope profile, measure, freshness, waiver standing, derived state,
    /// gate, owner, forum, and proof ref.
    pub fn render_tiles_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "function,service,corpus_id,scope_profile,measure,evidence_freshness,waiver_standing,state,gate,effective_qualification,owner_role,forum,proof_ref\n",
        );
        for tile in &self.fitness_tiles {
            let waiver = tile.waiver_standing.map(|s| s.as_str()).unwrap_or("");
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                tile.function.as_str(),
                tile.service.as_str(),
                tile.corpus_id,
                tile.scope_profile.as_str(),
                tile.measure.as_str(),
                tile.evidence_freshness.as_str(),
                waiver,
                tile.state.as_str(),
                tile.gate.as_str(),
                tile.effective_qualification.as_str(),
                tile.owner_role,
                tile.forum.as_str(),
                tile.proof_ref,
            ));
        }
        out
    }

    /// Deterministic governance-dashboard overview document for review, support, docs, or evaluator
    /// handoff.
    pub fn render_overview_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Governance Dashboard\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!(
            "- Corpus: `{}` ({})\n",
            self.corpus_id, self.corpus_label
        ));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        let s = &self.summary;
        out.push_str(&format!(
            "- Fitness: {} ({} passing, {} warning, {} stale, {} waived, {} waiver-expired, {} blocked)\n",
            s.total_functions, s.passing, s.warning, s.evidence_stale, s.waived, s.waiver_expired, s.blocked
        ));
        out.push_str(&format!(
            "- Waivers: {} ({} open, {} expired)\n",
            s.total_waivers, s.open_waivers, s.expired_waivers
        ));
        out.push_str(&format!(
            "- Services: {} ({} clean) — Decisions: {} ({} exercisable)\n",
            s.total_services, s.clean_services, s.total_decision_rights, s.exercisable_decisions
        ));
        out.push_str(&format!(
            "- Stable promotion: {}\n",
            if s.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));

        out.push_str("\n## Deployment-profile overviews\n\n");
        out.push_str(
            "| Profile | Effective posture | Gate | Qualification | Passing | Warning | Stale | Waived | Waiver-expired | Blocked | Open waivers |\n",
        );
        out.push_str(
            "|---------|-------------------|------|---------------|---------|---------|-------|--------|----------------|---------|--------------|\n",
        );
        for o in &self.overviews {
            let c = &o.tile_state_counts;
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} | {} | {} | {} | {} | {} | {} |\n",
                o.profile.as_str(),
                o.effective_posture.as_str(),
                o.gate_decision.as_str(),
                o.effective_qualification.as_str(),
                c.passing,
                c.warning,
                c.evidence_stale,
                c.waived,
                c.waiver_expired,
                c.blocked,
                o.open_waiver_count,
            ));
        }

        out.push_str("\n## Fitness tiles\n\n");
        out.push_str(
            "| Function | Service | Scope | Measure | Freshness | State | Gate | Owner | Forum |\n",
        );
        out.push_str(
            "|----------|---------|-------|---------|-----------|-------|------|-------|-------|\n",
        );
        for tile in &self.fitness_tiles {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                tile.function.as_str(),
                tile.service.as_str(),
                tile.scope_profile.as_str(),
                tile.measure.as_str(),
                tile.evidence_freshness.as_str(),
                tile.state.as_str(),
                tile.gate.as_str(),
                tile.owner_role,
                tile.forum.as_str(),
            ));
        }

        if !self.waiver_queue.is_empty() {
            out.push_str("\n## Waiver-expiry queue\n\n");
            out.push_str(
                "| Function | Standing | Expiry | Party | Action | Ticket | Tile state |\n",
            );
            out.push_str(
                "|----------|----------|--------|-------|--------|--------|------------|\n",
            );
            for w in &self.waiver_queue {
                out.push_str(&format!(
                    "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    w.function.as_str(),
                    w.queue_state.as_str(),
                    w.expiry,
                    w.responsible_party.as_str(),
                    w.action.as_str(),
                    w.ticket_ref,
                    w.tile_state.as_str(),
                ));
            }
        }

        out.push_str("\n## Service ownership\n\n");
        out.push_str("| Service | Owner | Forum | Worst state | Gate | Open waivers | Expired |\n");
        out.push_str("|---------|-------|-------|-------------|------|--------------|---------|\n");
        for c in &self.service_cards {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | {} | {} |\n",
                c.service.as_str(),
                c.owner_role,
                c.forum.as_str(),
                c.worst_state.as_str(),
                c.gate.as_str(),
                c.open_waiver_count,
                c.expired_waiver_count,
            ));
        }

        out.push_str("\n## Decision rights\n\n");
        out.push_str("| Decision | Forum | Owner | Posture | Worst state | Gate | Blocking |\n");
        out.push_str("|----------|-------|-------|---------|-------------|------|----------|\n");
        for c in &self.decision_right_cards {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                c.decision.as_str(),
                c.forum.as_str(),
                c.accountable_owner,
                c.posture.as_str(),
                c.worst_state.as_str(),
                c.gate.as_str(),
                c.blocking_function_count,
            ));
        }
        out
    }

    /// Compact Markdown report for the release-grade parity proof.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Governance Dashboard — Proof\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!(
            "- Corpus: `{}` ({})\n",
            self.corpus_id, self.corpus_label
        ));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        let s = &self.summary;
        out.push_str(&format!(
            "- Fitness: {} ({} passing, {} warning, {} stale, {} waived, {} waiver-expired, {} blocked)\n",
            s.total_functions, s.passing, s.warning, s.evidence_stale, s.waived, s.waiver_expired, s.blocked
        ));
        out.push_str(&format!(
            "- Stable promotion: {}\n",
            if s.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));
        out.push_str(&format!(
            "- Evaluation packet: `{}`\n",
            M5_GOVERNANCE_DASHBOARD_EVALUATION_PACKET_REF
        ));
        out.push_str(&format!(
            "- Fitness-tile CSV: `{}`\n",
            M5_GOVERNANCE_DASHBOARD_TILES_CSV_REF
        ));
        out
    }

    /// Validates the packet's invariants.
    pub fn validate(&self) -> Vec<M5GovernanceDashboardViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_GOVERNANCE_DASHBOARD_RECORD_KIND {
            out.push(M5GovernanceDashboardViolation::WrongRecordKind);
        }
        if self.schema_version != M5_GOVERNANCE_DASHBOARD_SCHEMA_VERSION {
            out.push(M5GovernanceDashboardViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.corpus_id.trim().is_empty()
            || self.corpus_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5GovernanceDashboardViolation::MissingIdentity);
        }

        // Every function tiled exactly once and self-consistent.
        let mut seen = std::collections::BTreeSet::new();
        for tile in &self.fitness_tiles {
            if !seen.insert(tile.function) {
                out.push(M5GovernanceDashboardViolation::DuplicateFunction);
            }
            if tile.corpus_id != self.corpus_id {
                out.push(M5GovernanceDashboardViolation::CorpusIdentityMissing);
            }
            out.extend(tile.validate());
        }
        for function in FitnessFunction::ALL {
            if !self.fitness_tiles.iter().any(|t| t.function == function) {
                out.push(M5GovernanceDashboardViolation::FunctionNotTiled);
            }
        }

        // Every nightly row mirrors its tile.
        for row in &self.nightly_rows {
            match self.tile(row.function) {
                Some(tile) => out.extend(row.validate(tile)),
                None => out.push(M5GovernanceDashboardViolation::NightlyRowDrift),
            }
        }
        for function in FitnessFunction::ALL {
            if !self.nightly_rows.iter().any(|r| r.function == function) {
                out.push(M5GovernanceDashboardViolation::FunctionNotTiled);
            }
        }

        // Waiver queue: each row discloses fully, sits on a tile that carries the same standing, and
        // the queue is ordered by expiry urgency.
        for row in &self.waiver_queue {
            out.extend(row.validate());
            match self.tile(row.function) {
                Some(tile) => {
                    if tile.waiver_standing != Some(row.queue_state) {
                        out.push(M5GovernanceDashboardViolation::WaiverWithoutWaivedTile);
                    }
                }
                None => out.push(M5GovernanceDashboardViolation::WaiverWithoutWaivedTile),
            }
        }
        if !is_sorted_by_key(&self.waiver_queue, |w| {
            (w.queue_state.queue_priority(), function_rank(w.function))
        }) {
            out.push(M5GovernanceDashboardViolation::WaiverQueueUnordered);
        }
        // Every waived tile must surface a queue row.
        for tile in &self.fitness_tiles {
            if tile.waiver_standing.is_some()
                && !self
                    .waiver_queue
                    .iter()
                    .any(|w| w.function == tile.function)
            {
                out.push(M5GovernanceDashboardViolation::WaiverWithoutWaivedTile);
            }
        }

        // Every service and decision card self-consistent.
        for card in &self.service_cards {
            out.extend(card.validate(&self.fitness_tiles, &self.waiver_queue));
        }
        for service in Service::ALL {
            if !self.service_cards.iter().any(|c| c.service == service) {
                out.push(M5GovernanceDashboardViolation::ServiceNotCarded);
            }
        }
        for card in &self.decision_right_cards {
            out.extend(card.validate(&self.fitness_tiles, &self.waiver_queue));
        }
        for decision in DecisionRight::ALL {
            if !self
                .decision_right_cards
                .iter()
                .any(|c| c.decision == decision)
            {
                out.push(M5GovernanceDashboardViolation::DecisionNotCarded);
            }
        }

        // Every profile has an overview.
        for o in &self.overviews {
            out.extend(o.validate(&self.corpus_id, &self.fitness_tiles, &self.waiver_queue));
        }
        for profile in ClaimedPosture::ALL {
            if !self.overviews.iter().any(|o| o.profile == profile) {
                out.push(M5GovernanceDashboardViolation::ProfileNotCovered);
            }
        }

        let expected_eval = GovernanceEvaluationPacket::derive(
            &self.evaluation_packet.packet_id,
            &self.packet_id,
            &self.corpus_id,
            &self.evaluated_at,
            &self.redaction_class_token,
            &self.fitness_tiles,
            &self.waiver_queue,
            &self.service_cards,
            &self.decision_right_cards,
        );
        if self.evaluation_packet != expected_eval
            || !self.evaluation_packet.reuses_canonical_vocabulary()
        {
            out.push(M5GovernanceDashboardViolation::EvaluationPacketDrift);
        }

        if !self.vocabulary.matches_canonical() {
            out.push(M5GovernanceDashboardViolation::VocabularyMismatch);
        }
        if self.summary
            != derive_summary(
                &self.fitness_tiles,
                &self.waiver_queue,
                &self.service_cards,
                &self.decision_right_cards,
            )
        {
            out.push(M5GovernanceDashboardViolation::SummaryDrift);
        }
        if self.conformance
            != derive_conformance(
                &self.corpus_id,
                &self.fitness_tiles,
                &self.nightly_rows,
                &self.waiver_queue,
                &self.service_cards,
                &self.decision_right_cards,
                &self.overviews,
                &self.evaluation_packet,
            )
            || !self.conformance.all_hold()
        {
            out.push(M5GovernanceDashboardViolation::ConformanceReviewFailed);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 governance dashboard serializes"),
        ) {
            out.push(M5GovernanceDashboardViolation::RawMaterialInExport);
        }
        out
    }
}

/// The generation channel a governance-dashboard packet is produced on. Every channel produces
/// byte-identical output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDashboardChannel {
    /// The desktop product UI.
    DesktopUi,
    /// The CLI / headless structured-output path.
    CliHeadless,
    /// Offline / mirror-safe packet generation.
    OfflineMirror,
}

impl GovernanceDashboardChannel {
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

// ---------------------------------------------------------------------------------------------
// Derivations
// ---------------------------------------------------------------------------------------------

/// Derives the summary from the tiles, waivers, service cards, and decision cards.
fn derive_summary(
    tiles: &[FitnessTile],
    waivers: &[WaiverQueueRow],
    services: &[ServiceOwnershipCard],
    decisions: &[DecisionRightCard],
) -> GovernanceDashboardSummary {
    let tile_count =
        |state: FitnessState| -> u32 { tiles.iter().filter(|t| t.state == state).count() as u32 };
    let open_waivers = waivers
        .iter()
        .filter(|w| w.queue_state != WaiverStanding::Expired)
        .count() as u32;
    let expired_waivers = waivers
        .iter()
        .filter(|w| w.queue_state == WaiverStanding::Expired)
        .count() as u32;
    let blocked = tiles
        .iter()
        .filter(|t| matches!(t.gate, DescriptorGate::Blocked))
        .count() as u32;
    let honored = ClaimedPosture::ALL
        .iter()
        .filter(|p| strongest_honored_posture(**p, tiles) == **p)
        .count() as u32;
    GovernanceDashboardSummary {
        total_functions: tiles.len() as u32,
        passing: tile_count(FitnessState::Passing),
        warning: tile_count(FitnessState::Warning),
        evidence_stale: tile_count(FitnessState::EvidenceStale),
        waived: tile_count(FitnessState::Waived),
        waiver_expired: tile_count(FitnessState::WaiverExpired),
        blocked: tile_count(FitnessState::Blocked),
        total_services: services.len() as u32,
        clean_services: services.iter().filter(|c| c.is_clean()).count() as u32,
        total_decision_rights: decisions.len() as u32,
        exercisable_decisions: decisions.iter().filter(|c| c.is_exercisable()).count() as u32,
        total_waivers: waivers.len() as u32,
        open_waivers,
        expired_waivers,
        total_profiles: ClaimedPosture::ALL.len() as u32,
        honored_profiles: honored,
        blocks_stable_promotion: blocked > 0,
    }
}

/// Derives the conformance review, so the stored block reflects the actual packet.
#[allow(clippy::too_many_arguments)]
fn derive_conformance(
    corpus_id: &str,
    tiles: &[FitnessTile],
    nightly: &[NightlyGovernanceRow],
    waivers: &[WaiverQueueRow],
    services: &[ServiceOwnershipCard],
    decisions: &[DecisionRightCard],
    overviews: &[GovernanceOverview],
    evaluation_packet: &GovernanceEvaluationPacket,
) -> GovernanceDashboardConformance {
    let derived = tiles.iter().all(|t| {
        t.state == derive_fitness_state(t.measure, t.evidence_freshness, t.waiver_standing)
    });

    let first_class = tiles
        .iter()
        .all(|t| !t.owner_role.trim().is_empty() && !t.proof_ref.trim().is_empty())
        && services.iter().all(|c| !c.owner_role.trim().is_empty());

    let never_clean = tiles.iter().all(|t| {
        if matches!(
            t.state,
            FitnessState::Waived | FitnessState::EvidenceStale | FitnessState::WaiverExpired
        ) {
            !matches!(t.gate, DescriptorGate::Governed)
                && !matches!(t.signal, DescriptorSignal::Green)
        } else {
            true
        }
    });

    let stale_narrows = tiles.iter().all(|t| {
        if matches!(t.evidence_freshness, FreshnessState::Stale) && t.waiver_standing.is_none() {
            matches!(t.gate, DescriptorGate::Narrowed | DescriptorGate::Blocked)
        } else {
            true
        }
    });

    let missing_blocks = tiles.iter().all(|t| {
        let hard = matches!(
            t.evidence_freshness,
            FreshnessState::Expired | FreshnessState::Missing
        ) && t.waiver_standing.is_none();
        let expired_waiver = matches!(t.waiver_standing, Some(WaiverStanding::Expired));
        if hard || expired_waiver {
            matches!(t.gate, DescriptorGate::Blocked)
        } else {
            true
        }
    });

    let waiver_discloses = waivers.iter().all(|w| {
        !w.expiry.trim().is_empty()
            && !w.rationale.trim().is_empty()
            && !w.ticket_ref.trim().is_empty()
    });

    let service_owner = services
        .iter()
        .all(|c| !c.owner_role.trim().is_empty() && Forum::ALL.contains(&c.forum));
    let decision_owner = decisions
        .iter()
        .all(|c| !c.accountable_owner.trim().is_empty() && Forum::ALL.contains(&c.forum));

    let overview_ok = overviews
        .iter()
        .all(|o| posture_rank(o.effective_posture) <= posture_rank(o.profile));

    let corpus_bound = !corpus_id.trim().is_empty()
        && tiles.iter().all(|t| t.corpus_id == corpus_id)
        && nightly.iter().all(|r| r.corpus_id == corpus_id)
        && overviews.iter().all(|o| o.corpus_id == corpus_id)
        && evaluation_packet.corpus_id == corpus_id;

    let export_clean = ![
        serde_json::to_value(tiles).expect("tiles serialize"),
        serde_json::to_value(waivers).expect("waivers serialize"),
        serde_json::to_value(services).expect("services serialize"),
        serde_json::to_value(decisions).expect("decisions serialize"),
    ]
    .iter()
    .any(json_contains_forbidden_material);

    GovernanceDashboardConformance {
        tile_state_derived_from_inputs: derived,
        freshness_and_ownership_first_class: first_class,
        waived_or_stale_never_clean_pass: never_clean,
        stale_evidence_narrows_deterministically: stale_narrows,
        missing_or_expired_blocks_stable_promotion: missing_blocks,
        waiver_queue_discloses_expiry_and_action: waiver_discloses,
        service_cards_bind_owner_and_forum: service_owner,
        decision_cards_bind_owner_and_forum: decision_owner,
        overview_effective_posture_never_overstated: overview_ok,
        corpus_identity_bound: corpus_bound,
        evaluation_packet_reuses_ui_vocabulary: evaluation_packet.reuses_canonical_vocabulary(),
        controlled_enums_frozen: GovernanceDashboardVocabulary::canonical().matches_canonical(),
        export_carries_no_raw_material: export_clean,
    }
}

// ---------------------------------------------------------------------------------------------
// Token helpers
// ---------------------------------------------------------------------------------------------

/// Maps a typed `ALL` array to its stable tokens.
fn tokens<T: Copy>(all: &[T], f: impl Fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|t| f(*t).to_owned()).collect()
}

/// True when `items` is non-decreasing by `key`.
fn is_sorted_by_key<T, K: Ord>(items: &[T], key: impl Fn(&T) -> K) -> bool {
    items.windows(2).all(|w| key(&w[0]) <= key(&w[1]))
}

// ---------------------------------------------------------------------------------------------
// Violations
// ---------------------------------------------------------------------------------------------

/// Validation failures for the governance-dashboard lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceDashboardViolation {
    /// The packet record kind is wrong.
    WrongRecordKind,
    /// The packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// A tile cites a field that does not match its function.
    TileFieldMismatch,
    /// A tile's state drifted from its measure / freshness / waiver.
    TileStateDrift,
    /// A tile binds no proof ref or corpus.
    TileEvidenceMissing,
    /// A waiver is attached to a function that is already a clean pass.
    WaiverOnPassingFunction,
    /// Two tiles name the same function.
    DuplicateFunction,
    /// A function has no tile / nightly row.
    FunctionNotTiled,
    /// A nightly row drifted from its tile.
    NightlyRowDrift,
    /// A nightly row records no run timestamp.
    NightlyRunMissing,
    /// A waiver-queue row cites a field that does not match its function.
    WaiverFieldMismatch,
    /// A waiver-queue row's state drifted from its standing.
    WaiverStateDrift,
    /// A waiver-queue row omits expiry, rationale, or ticket.
    WaiverDisclosureIncomplete,
    /// A waiver-queue row has no tile carrying the same waiver standing.
    WaiverWithoutWaivedTile,
    /// The waiver queue is not ordered by expiry urgency.
    WaiverQueueUnordered,
    /// A service card drifted from a fresh derivation.
    ServiceCardDrift,
    /// A service card binds no owner.
    ServiceOwnerMissing,
    /// A service has no card.
    ServiceNotCarded,
    /// A decision card drifted from a fresh derivation.
    DecisionCardDrift,
    /// A decision card binds no owner.
    DecisionOwnerMissing,
    /// A decision card's posture drifted from its gate.
    DecisionPostureDrift,
    /// A decision has no card.
    DecisionNotCarded,
    /// An overview drifted from a fresh derivation.
    OverviewDrift,
    /// An overview reads a posture above its profile.
    OverviewOverstatesPosture,
    /// An overview's gate or qualification drifted.
    OverviewGateDrift,
    /// A profile has no overview.
    ProfileNotCovered,
    /// A surface is not stamped with the packet's corpus identity.
    CorpusIdentityMissing,
    /// The evaluation packet drifted from the tiles / waivers / cards or its vocabulary.
    EvaluationPacketDrift,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// The summary disagrees with the tiles / waivers / cards.
    SummaryDrift,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The export contains raw provider material.
    RawMaterialInExport,
}

impl M5GovernanceDashboardViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::TileFieldMismatch => "tile_field_mismatch",
            Self::TileStateDrift => "tile_state_drift",
            Self::TileEvidenceMissing => "tile_evidence_missing",
            Self::WaiverOnPassingFunction => "waiver_on_passing_function",
            Self::DuplicateFunction => "duplicate_function",
            Self::FunctionNotTiled => "function_not_tiled",
            Self::NightlyRowDrift => "nightly_row_drift",
            Self::NightlyRunMissing => "nightly_run_missing",
            Self::WaiverFieldMismatch => "waiver_field_mismatch",
            Self::WaiverStateDrift => "waiver_state_drift",
            Self::WaiverDisclosureIncomplete => "waiver_disclosure_incomplete",
            Self::WaiverWithoutWaivedTile => "waiver_without_waived_tile",
            Self::WaiverQueueUnordered => "waiver_queue_unordered",
            Self::ServiceCardDrift => "service_card_drift",
            Self::ServiceOwnerMissing => "service_owner_missing",
            Self::ServiceNotCarded => "service_not_carded",
            Self::DecisionCardDrift => "decision_card_drift",
            Self::DecisionOwnerMissing => "decision_owner_missing",
            Self::DecisionPostureDrift => "decision_posture_drift",
            Self::DecisionNotCarded => "decision_not_carded",
            Self::OverviewDrift => "overview_drift",
            Self::OverviewOverstatesPosture => "overview_overstates_posture",
            Self::OverviewGateDrift => "overview_gate_drift",
            Self::ProfileNotCovered => "profile_not_covered",
            Self::CorpusIdentityMissing => "corpus_identity_missing",
            Self::EvaluationPacketDrift => "evaluation_packet_drift",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::SummaryDrift => "summary_drift",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked raw material. Mirrors the redaction posture of the
/// upstream descriptor / governance lanes.
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
