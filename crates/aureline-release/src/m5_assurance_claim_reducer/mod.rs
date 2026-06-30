//! The M5 assurance-claim reducer — the one source of truth that *automatically narrows* Aureline's
//! regulated / self-hosted / sovereign / no-vendor / no-telemetry / customer-managed-key claims the
//! moment a precondition behind them drifts, and drives every consumer of those claims from that one
//! output.
//!
//! The [assurance center](crate::m5_assurance_center) derives each claim card's active state from the
//! control proofs backing it; the [governance matrix](crate::m5_assurance_route_governance) freezes
//! the gate-bound claim-state grammar both lanes share. This lane closes the loop between the
//! *condition* and the *consumers*: it takes the four trust preconditions a regulated claim depends on
//! — fresh supporting evidence, an unchanged hosted-dependency boundary, pinned key / data residency,
//! and an intact policy / control path — and reduces each claim to the single weakest state those
//! preconditions allow today. When a precondition drifts, the claim narrows; when one is invalidated,
//! the claim blocks. The reduction is never a manual copy edit: every weaker state is *derived* from a
//! named precondition drift, so a claim can never read stronger than the trust facts behind it.
//!
//! The packet has three derived parts, all computed from one input — each precondition's current
//! status:
//!
//! - [`ReducedClaim`]s. One per [claim subject](crate::m5_assurance_center::AssuranceClaimSubject). A
//!   reduced claim never asserts a fixed state; it derives its [reduced
//!   state](crate::m5_assurance_route_governance::AssuranceClaimState) from the worst gate among its
//!   required [preconditions](ClaimPrecondition). A drifted precondition narrows the claim to
//!   `under_review`; an invalidated precondition blocks it to `unproven`. Each drift is recorded as a
//!   [`PreconditionDrift`] naming *which* precondition drifted and the [restoration
//!   action](RestorationAction) that would lift it, and when the claim is not fully governed it
//!   carries a [`NearestTruthful`] statement — the weaker posture that is still true — so the product
//!   can state the nearest truthful current claim instead of the one that no longer holds.
//! - [`ConsumerProjection`]s. One per [consumer surface](ReducerConsumer): the About / help panel, the
//!   assurance center, the exported evaluation packet, the procurement export, and the
//!   release / public-truth manifest. Every consumer reads the *same* reduced state for the same
//!   claim; the projections are derived from the one reduced claim, so they always converge. This is
//!   the guardrail the spec names: a claim narrowed in one consumer can never read stronger in another.
//! - An [`AssuranceNarrowingExportPreview`]. A redaction-safe, refs-only projection of the reduced
//!   claims for offline procurement / evaluator review that preserves the drift lineage and evidence
//!   refs without leaking any raw material.
//!
//! The [`M5AssuranceClaimReducer`] packet is the one inspectable, serde-serializable truth record this
//! lane produces: it preserves proof lineage as refs only and carries no credential bodies or raw
//! provider payloads.
//!
//! - Packet schema:
//!   [`schemas/public-truth/m5-assurance-claim-reducer.schema.json`](../../../../../schemas/public-truth/m5-assurance-claim-reducer.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-assurance-claim-reducer-contract.md`](../../../../../docs/public-truth/m5-assurance-claim-reducer-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_assurance_claim_reducer,
    seeded_m5_assurance_claim_reducer_hosted_dependency_drift_narrowed,
    seeded_m5_assurance_claim_reducer_key_residency_mismatch_blocked,
    seeded_m5_assurance_claim_reducer_policy_path_regression_blocked,
    seeded_m5_assurance_claim_reducer_stale_evidence_narrowed,
    M5_ASSURANCE_CLAIM_REDUCER_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The reducer reuses the assurance center's claim subjects and the governance matrix's frozen
// claim-state / posture / boundary / evidence vocabulary plus the descriptor / badge gate runtime, so
// the reduced state and every consumer projection can never drift to a different state grammar.
use crate::m5_assurance_center::AssuranceClaimSubject;
use crate::m5_assurance_route_governance::{
    AssuranceClaimState, ClaimedPosture, EvidenceClass, TrustBoundary,
};
use crate::m5_descriptor_badge::{
    ConsumerStatus, DescriptorGate, DescriptorSignal, QualificationClass,
};

/// Record-kind tag carried by [`M5AssuranceClaimReducer`].
pub const M5_ASSURANCE_CLAIM_REDUCER_RECORD_KIND: &str = "m5_assurance_claim_reducer";

/// Record-kind tag carried by the embedded [`AssuranceNarrowingExportPreview`].
pub const M5_ASSURANCE_NARROWING_EXPORT_RECORD_KIND: &str = "m5_assurance_narrowing_export_preview";

/// Schema version for the assurance-claim-reducer packet.
pub const M5_ASSURANCE_CLAIM_REDUCER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the assurance-claim-reducer packet schema.
pub const M5_ASSURANCE_CLAIM_REDUCER_SCHEMA_REF: &str =
    "schemas/public-truth/m5-assurance-claim-reducer.schema.json";

/// Repo-relative path of the published assurance-claim-reducer inventory.
pub const M5_ASSURANCE_CLAIM_REDUCER_REGISTRY_REF: &str =
    "artifacts/public-truth/m5-assurance-claim-reducer.json";

/// Repo-relative path of the rendered narrowing overview document.
pub const M5_ASSURANCE_CLAIM_REDUCER_OVERVIEW_REF: &str =
    "artifacts/public-truth/m5-assurance-claim-reducer.md";

/// Repo-relative path of the machine-readable claim / precondition matrix export.
pub const M5_ASSURANCE_CLAIM_REDUCER_CLAIMS_CSV_REF: &str =
    "artifacts/public-truth/m5-assurance-claim-reducer-claims.csv";

/// Repo-relative path of the release-grade narrowing parity proof.
pub const M5_ASSURANCE_NARROWING_PROOF_REF: &str =
    "artifacts/public-truth/m5-assurance-narrowing-proof/assurance-claim-reducer.json";

/// Repo-relative path of the exported redaction-safe narrowing preview.
pub const M5_ASSURANCE_NARROWING_EXPORT_PREVIEW_REF: &str =
    "artifacts/public-truth/m5-assurance-narrowing-proof/export-preview.json";

/// Repo-relative path of the assurance-claim-reducer contract doc.
pub const M5_ASSURANCE_CLAIM_REDUCER_DOC_REF: &str =
    "docs/public-truth/m5-assurance-claim-reducer-contract.md";

/// Repo-relative directory of the per-state narrowing fixtures.
pub const M5_ASSURANCE_CLAIM_REDUCER_FIXTURE_DIR: &str =
    "fixtures/public-truth/assurance-claim-narrowing/";

/// Prefix every assurance-claim-reducer message id carries so consumers can route it.
pub const M5_ASSURANCE_CLAIM_REDUCER_MESSAGE_ID_PREFIX: &str =
    "public_truth.assurance_claim_reducer.";

// ---------------------------------------------------------------------------------------------
// Preconditions
// ---------------------------------------------------------------------------------------------

/// One trust precondition a regulated claim depends on. The set is exactly the four drift dimensions
/// the exit-gate names — stale evidence, hosted-dependency drift, key / residency mismatch, and
/// policy-path regression — framed as the *condition that must hold* for the claim to keep its full
/// strength. This lane invents no new compliance frameworks; it names the trust facts the existing
/// proof, route, boundary, and policy lanes already record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimPrecondition {
    /// The supporting evidence behind the claim is fresh (not stale / expired).
    EvidenceFreshness,
    /// No new hosted / vendor dependency has crossed the claim's declared trust boundary.
    HostedDependencyBoundary,
    /// Encryption keys and data stay pinned to the customer-owned residency.
    KeyResidency,
    /// The required policy / control path backing the claim is intact.
    PolicyControlPath,
}

impl ClaimPrecondition {
    /// Every precondition, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::EvidenceFreshness,
        Self::HostedDependencyBoundary,
        Self::KeyResidency,
        Self::PolicyControlPath,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceFreshness => "evidence_freshness",
            Self::HostedDependencyBoundary => "hosted_dependency_boundary",
            Self::KeyResidency => "key_residency",
            Self::PolicyControlPath => "policy_control_path",
        }
    }

    /// Reader-facing precondition label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EvidenceFreshness => "Evidence freshness",
            Self::HostedDependencyBoundary => "Hosted-dependency boundary",
            Self::KeyResidency => "Key / data residency",
            Self::PolicyControlPath => "Policy / control path",
        }
    }

    /// The evidence class that proves this precondition holds.
    pub const fn evidence_class(self) -> EvidenceClass {
        match self {
            Self::EvidenceFreshness => EvidenceClass::ControlAttestation,
            Self::HostedDependencyBoundary => EvidenceClass::RouteTimeline,
            Self::KeyResidency => EvidenceClass::BoundaryManifest,
            Self::PolicyControlPath => EvidenceClass::PolicyBundle,
        }
    }

    /// Owner role accountable for keeping this precondition's proof current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::EvidenceFreshness => "control_proof_owner",
            Self::HostedDependencyBoundary => "route_explainability_owner",
            Self::KeyResidency => "key_custody_owner",
            Self::PolicyControlPath => "policy_governance_owner",
        }
    }

    /// Repo-relative proof ref backing this precondition — drawn from the governance-matrix proofs, so
    /// the reducer reuses the existing proof lanes rather than minting a parallel evidence family.
    pub const fn proof_ref(self) -> &'static str {
        match self {
            Self::EvidenceFreshness => {
                "artifacts/release-proof/m5-assurance-route-governance/control-proof.json"
            }
            Self::HostedDependencyBoundary => {
                "artifacts/release-proof/m5-assurance-route-governance/route-hop.json"
            }
            Self::KeyResidency => {
                "artifacts/release-proof/m5-assurance-route-governance/capability-boundary.json"
            }
            Self::PolicyControlPath => {
                "artifacts/release-proof/m5-assurance-route-governance/governance-freshness.json"
            }
        }
    }

    /// The drift token this precondition produces at the given status. A satisfied precondition has no
    /// drift token.
    const fn drift_token(self, status: PreconditionStatus) -> Option<DriftToken> {
        match (self, status) {
            (_, PreconditionStatus::Satisfied) => None,
            (Self::EvidenceFreshness, PreconditionStatus::Drifted) => {
                Some(DriftToken::StaleEvidence)
            }
            (Self::EvidenceFreshness, PreconditionStatus::Invalidated) => {
                Some(DriftToken::EvidenceExpired)
            }
            (Self::HostedDependencyBoundary, PreconditionStatus::Drifted) => {
                Some(DriftToken::HostedDependencyDrift)
            }
            (Self::HostedDependencyBoundary, PreconditionStatus::Invalidated) => {
                Some(DriftToken::BoundaryDependencyAdded)
            }
            (Self::KeyResidency, PreconditionStatus::Drifted) => {
                Some(DriftToken::KeyResidencyDrift)
            }
            (Self::KeyResidency, PreconditionStatus::Invalidated) => {
                Some(DriftToken::KeyResidencyMismatch)
            }
            (Self::PolicyControlPath, PreconditionStatus::Drifted) => {
                Some(DriftToken::PolicyPathDegraded)
            }
            (Self::PolicyControlPath, PreconditionStatus::Invalidated) => {
                Some(DriftToken::PolicyPathRegression)
            }
        }
    }

    /// The action that would restore this precondition.
    const fn restoration_action(self) -> RestorationAction {
        match self {
            Self::EvidenceFreshness => RestorationAction::RefreshEvidence,
            Self::HostedDependencyBoundary => RestorationAction::RestoreBoundary,
            Self::KeyResidency => RestorationAction::RepinKeyResidency,
            Self::PolicyControlPath => RestorationAction::RestorePolicyPath,
        }
    }
}

/// The current status of a precondition. Declaration order is most→least satisfied so the worst
/// applicable status wins; each status binds to one gate posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreconditionStatus {
    /// The precondition holds; the claim keeps its full strength.
    Satisfied,
    /// The precondition is drifting; the claim narrows.
    Drifted,
    /// The precondition is invalidated; the claim blocks.
    Invalidated,
}

impl PreconditionStatus {
    /// Every status, in declaration order.
    pub const ALL: [Self; 3] = [Self::Satisfied, Self::Drifted, Self::Invalidated];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Drifted => "drifted",
            Self::Invalidated => "invalidated",
        }
    }

    /// Reader-facing status label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Satisfied => "Satisfied",
            Self::Drifted => "Drifted",
            Self::Invalidated => "Invalidated",
        }
    }

    /// The gate posture this status implies: satisfied stays governed, drifted narrows, invalidated
    /// blocks.
    pub const fn gate(self) -> DescriptorGate {
        match self {
            Self::Satisfied => DescriptorGate::Governed,
            Self::Drifted => DescriptorGate::Narrowed,
            Self::Invalidated => DescriptorGate::Blocked,
        }
    }
}

/// The named drift a precondition inflicts on a claim. The set is the four exit-gate conditions plus
/// their blocking escalations; each maps one-to-one to a (precondition, status) pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftToken {
    /// The supporting evidence is stale (narrows).
    StaleEvidence,
    /// The supporting evidence is expired / missing (blocks).
    EvidenceExpired,
    /// A hosted dependency drifted toward the claim's boundary (narrows).
    HostedDependencyDrift,
    /// A hosted / vendor dependency crossed the claim's boundary (blocks).
    BoundaryDependencyAdded,
    /// Key / data residency is drifting from its pin (narrows).
    KeyResidencyDrift,
    /// Key / data residency no longer matches the claim (blocks).
    KeyResidencyMismatch,
    /// The required policy / control path is degraded (narrows).
    PolicyPathDegraded,
    /// The required policy / control path regressed (blocks).
    PolicyPathRegression,
}

impl DriftToken {
    /// Every drift token, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::StaleEvidence,
        Self::EvidenceExpired,
        Self::HostedDependencyDrift,
        Self::BoundaryDependencyAdded,
        Self::KeyResidencyDrift,
        Self::KeyResidencyMismatch,
        Self::PolicyPathDegraded,
        Self::PolicyPathRegression,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleEvidence => "stale_evidence",
            Self::EvidenceExpired => "evidence_expired",
            Self::HostedDependencyDrift => "hosted_dependency_drift",
            Self::BoundaryDependencyAdded => "boundary_dependency_added",
            Self::KeyResidencyDrift => "key_residency_drift",
            Self::KeyResidencyMismatch => "key_residency_mismatch",
            Self::PolicyPathDegraded => "policy_path_degraded",
            Self::PolicyPathRegression => "policy_path_regression",
        }
    }

    /// Reader-facing drift label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::StaleEvidence => "Stale evidence",
            Self::EvidenceExpired => "Evidence expired",
            Self::HostedDependencyDrift => "Hosted-dependency drift",
            Self::BoundaryDependencyAdded => "Boundary dependency added",
            Self::KeyResidencyDrift => "Key / residency drift",
            Self::KeyResidencyMismatch => "Key / residency mismatch",
            Self::PolicyPathDegraded => "Policy-path degraded",
            Self::PolicyPathRegression => "Policy-path regression",
        }
    }
}

/// The action that would restore a drifted precondition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestorationAction {
    /// Refresh and re-attest the supporting evidence.
    RefreshEvidence,
    /// Restore the declared trust boundary by removing the hosted dependency.
    RestoreBoundary,
    /// Re-pin key / data residency to the customer-owned region.
    RepinKeyResidency,
    /// Restore the required policy / control path.
    RestorePolicyPath,
}

impl RestorationAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RefreshEvidence,
        Self::RestoreBoundary,
        Self::RepinKeyResidency,
        Self::RestorePolicyPath,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshEvidence => "refresh_evidence",
            Self::RestoreBoundary => "restore_boundary",
            Self::RepinKeyResidency => "repin_key_residency",
            Self::RestorePolicyPath => "restore_policy_path",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Consumers
// ---------------------------------------------------------------------------------------------

/// One consumer surface the reducer's output governs. The set is the surfaces the spec names: the
/// About / help panel, the assurance center, the exported evaluation packet, the procurement export,
/// and the release / public-truth manifest. Every consumer reads the *same* reduced state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReducerConsumer {
    /// The About / help panel.
    HelpAbout,
    /// The in-product assurance center.
    AssuranceCenter,
    /// The exported evaluation packet.
    EvaluationPacket,
    /// The procurement export.
    ProcurementExport,
    /// The release / public-truth manifest.
    ReleasePublicTruth,
}

impl ReducerConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HelpAbout,
        Self::AssuranceCenter,
        Self::EvaluationPacket,
        Self::ProcurementExport,
        Self::ReleasePublicTruth,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::AssuranceCenter => "assurance_center",
            Self::EvaluationPacket => "evaluation_packet",
            Self::ProcurementExport => "procurement_export",
            Self::ReleasePublicTruth => "release_public_truth",
        }
    }

    /// Reviewer-facing consumer label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::HelpAbout => "About / help",
            Self::AssuranceCenter => "Assurance center",
            Self::EvaluationPacket => "Evaluation packet",
            Self::ProcurementExport => "Procurement export",
            Self::ReleasePublicTruth => "Release / public-truth manifest",
        }
    }

    /// Owner role accountable for keeping this consumer bound to the reducer output.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about_owner",
            Self::AssuranceCenter => "assurance_center_owner",
            Self::EvaluationPacket => "evaluation_packet_owner",
            Self::ProcurementExport => "procurement_export_owner",
            Self::ReleasePublicTruth => "release_truth_owner",
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

/// Maps a gate posture to the coverage status it implies.
const fn gate_status(gate: DescriptorGate) -> ConsumerStatus {
    match gate {
        DescriptorGate::Governed => ConsumerStatus::Mapped,
        DescriptorGate::Narrowed => ConsumerStatus::Provisional,
        DescriptorGate::Blocked => ConsumerStatus::Unmapped,
    }
}

/// The reduced claim state a gate posture implies: governed proves the claim, narrowed puts it under
/// review, blocked leaves it unproven.
const fn state_for_gate(gate: DescriptorGate) -> AssuranceClaimState {
    match gate {
        DescriptorGate::Governed => AssuranceClaimState::Proven,
        DescriptorGate::Narrowed => AssuranceClaimState::UnderReview,
        DescriptorGate::Blocked => AssuranceClaimState::Unproven,
    }
}

/// Position of a posture in the canonical (weakest→strongest) ordering.
fn posture_rank(posture: ClaimedPosture) -> usize {
    ClaimedPosture::ALL
        .iter()
        .position(|p| *p == posture)
        .unwrap_or(ClaimedPosture::ALL.len())
}

/// The nearest weaker posture below `posture`; the weakest posture maps to itself.
fn weaker_posture(posture: ClaimedPosture) -> ClaimedPosture {
    let rank = posture_rank(posture);
    if rank == 0 {
        ClaimedPosture::ALL[0]
    } else {
        ClaimedPosture::ALL[rank - 1]
    }
}

/// Position of a precondition in the canonical ordering.
fn precondition_rank(precondition: ClaimPrecondition) -> usize {
    ClaimPrecondition::ALL
        .iter()
        .position(|p| *p == precondition)
        .unwrap_or(ClaimPrecondition::ALL.len())
}

/// The trust preconditions a claim subject depends on, in canonical order. Every claim depends on
/// fresh supporting evidence; the boundary / residency / policy preconditions apply to the claims
/// whose trust story they govern.
fn required_preconditions(subject: AssuranceClaimSubject) -> Vec<ClaimPrecondition> {
    use ClaimPrecondition::*;
    let mut out = match subject {
        AssuranceClaimSubject::LocalFirstContinuity => vec![EvidenceFreshness, PolicyControlPath],
        AssuranceClaimSubject::TelemetryControl => {
            vec![
                EvidenceFreshness,
                HostedDependencyBoundary,
                PolicyControlPath,
            ]
        }
        AssuranceClaimSubject::KeyOwnership => vec![EvidenceFreshness, KeyResidency],
        AssuranceClaimSubject::DataResidency => {
            vec![EvidenceFreshness, HostedDependencyBoundary, KeyResidency]
        }
        AssuranceClaimSubject::RegulatedOperation => vec![
            EvidenceFreshness,
            HostedDependencyBoundary,
            KeyResidency,
            PolicyControlPath,
        ],
        AssuranceClaimSubject::AirGapContainment => {
            vec![
                EvidenceFreshness,
                HostedDependencyBoundary,
                PolicyControlPath,
            ]
        }
        AssuranceClaimSubject::SovereignDeployment => vec![
            EvidenceFreshness,
            HostedDependencyBoundary,
            KeyResidency,
            PolicyControlPath,
        ],
    };
    out.sort_by_key(|p| precondition_rank(*p));
    out.dedup();
    out
}

// ---------------------------------------------------------------------------------------------
// Precondition readings & drift
// ---------------------------------------------------------------------------------------------

/// One precondition reading on a reduced claim: the precondition, its current status, the gate that
/// status implies, and the evidence class / owner / proof ref backing it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreconditionReading {
    /// The precondition.
    pub precondition: ClaimPrecondition,
    /// Reader-facing precondition label.
    pub precondition_label: String,
    /// The precondition's current status.
    pub status: PreconditionStatus,
    /// Reader-facing status label.
    pub status_label: String,
    /// The gate this status implies.
    pub gate: DescriptorGate,
    /// The evidence class that proves the precondition.
    pub evidence_class: EvidenceClass,
    /// Owner role accountable for the precondition's proof.
    pub owner_role: String,
    /// Repo-relative proof ref backing the precondition.
    pub proof_ref: String,
}

impl PreconditionReading {
    fn new(precondition: ClaimPrecondition, status: PreconditionStatus) -> Self {
        Self {
            precondition,
            precondition_label: precondition.label().to_owned(),
            status,
            status_label: status.label().to_owned(),
            gate: status.gate(),
            evidence_class: precondition.evidence_class(),
            owner_role: precondition.owner_role().to_owned(),
            proof_ref: precondition.proof_ref().to_owned(),
        }
    }
}

/// One recorded precondition drift on a reduced claim: which precondition drifted, the named drift,
/// the status, the gate it inflicts, the action that restores it, and the evidence ref that would
/// prove the restoration. This is the attribution the spec requires — the product can name the
/// precondition that changed instead of silently restating a weaker claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreconditionDrift {
    /// The precondition that drifted.
    pub precondition: ClaimPrecondition,
    /// The named drift this (precondition, status) produces.
    pub drift: DriftToken,
    /// Reader-facing drift label.
    pub drift_label: String,
    /// The precondition's status.
    pub status: PreconditionStatus,
    /// The gate the drift inflicts.
    pub gate: DescriptorGate,
    /// True when this drift blocks the claim (vs only narrowing it).
    pub blocking: bool,
    /// The action that would restore the precondition.
    pub restoration_action: RestorationAction,
    /// Repo-relative proof ref that would prove the restoration.
    pub evidence_ref: String,
    /// Stable message id; prefixed [`M5_ASSURANCE_CLAIM_REDUCER_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

impl PreconditionDrift {
    /// Builds a drift row from a precondition reading, or `None` when the precondition is satisfied.
    fn from_reading(subject: AssuranceClaimSubject, reading: &PreconditionReading) -> Option<Self> {
        let drift = reading.precondition.drift_token(reading.status)?;
        Some(Self {
            precondition: reading.precondition,
            drift,
            drift_label: drift.label().to_owned(),
            status: reading.status,
            gate: reading.gate,
            blocking: matches!(reading.gate, DescriptorGate::Blocked),
            restoration_action: reading.precondition.restoration_action(),
            evidence_ref: reading.proof_ref.clone(),
            cause_message_id: format!(
                "{}claim.{}.{}.{}.drift",
                M5_ASSURANCE_CLAIM_REDUCER_MESSAGE_ID_PREFIX,
                subject.as_str(),
                reading.precondition.as_str(),
                drift.as_str()
            ),
        })
    }
}

// ---------------------------------------------------------------------------------------------
// Nearest truthful fallback
// ---------------------------------------------------------------------------------------------

/// The nearest truthful current statement a claim falls back to when a precondition drifts: the
/// weaker posture that is still true, the strongest state that weaker statement can carry, and the
/// preconditions still satisfied versus those that drifted. The fallback never overstates: its state
/// is never `proven`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NearestTruthful {
    /// The nearest weaker posture the claim can still truthfully assert.
    pub fallback_posture: ClaimedPosture,
    /// The strongest state the weaker statement can carry — never `proven`.
    pub fallback_state: AssuranceClaimState,
    /// Preconditions still satisfied, in canonical order.
    pub still_satisfied: Vec<ClaimPrecondition>,
    /// Preconditions that drifted, in canonical order.
    pub drifted_preconditions: Vec<ClaimPrecondition>,
    /// Stable message id; prefixed [`M5_ASSURANCE_CLAIM_REDUCER_MESSAGE_ID_PREFIX`].
    pub message_id: String,
}

// ---------------------------------------------------------------------------------------------
// Consumer projections
// ---------------------------------------------------------------------------------------------

/// One consumer's projection of a reduced claim. Every consumer reads the same reduced state for the
/// same claim; the `converges_with_reduced` flag is the convergence proof — and the guardrail that no
/// consumer can read the claim stronger than the reduced output after a narrowing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerProjection {
    /// The consumer surface.
    pub consumer: ReducerConsumer,
    /// Reviewer-facing consumer label.
    pub consumer_label: String,
    /// The claim state this consumer reads — the reduced state, by construction.
    pub claim_state: AssuranceClaimState,
    /// Effective qualification this consumer reads.
    pub effective_qualification: QualificationClass,
    /// Coverage status implied by the reduced gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Owner role accountable for keeping this consumer bound to the reducer output.
    pub owner_role: String,
    /// True when this projection reads the same state and qualification as the reduced claim.
    pub converges_with_reduced: bool,
    /// Stable message id; prefixed [`M5_ASSURANCE_CLAIM_REDUCER_MESSAGE_ID_PREFIX`].
    pub message_id: String,
}

// ---------------------------------------------------------------------------------------------
// Reduced claims
// ---------------------------------------------------------------------------------------------

/// One reduced claim: the subject, the posture it claims, the preconditions it depends on, the reduced
/// state derived from the worst gate among them, the recorded precondition drifts, the nearest
/// truthful fallback when not governed, and the per-consumer projections that all read the reduced
/// state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedClaim {
    /// The claim subject.
    pub subject: AssuranceClaimSubject,
    /// Reader-facing claim label.
    pub subject_label: String,
    /// The deployment posture this claim asserts.
    pub claimed_posture: ClaimedPosture,
    /// The trust boundaries this claim spans.
    pub trust_boundaries: Vec<TrustBoundary>,
    /// Owner role accountable for the claim.
    pub owner_role: String,
    /// The precondition readings, in canonical order.
    pub preconditions: Vec<PreconditionReading>,
    /// The reduced claim state, derived from the worst gate among the preconditions.
    pub reduced_state: AssuranceClaimState,
    /// Reader-facing reduced-state label.
    pub reduced_state_label: String,
    /// Gate posture the reduced state binds to.
    pub reduced_gate: DescriptorGate,
    /// Effective qualification implied by the reduced gate.
    pub effective_qualification: QualificationClass,
    /// The recorded precondition drifts, in canonical order.
    pub drifts: Vec<PreconditionDrift>,
    /// The repo-relative proof refs of the required preconditions — evidence links, refs only.
    pub evidence_refs: Vec<String>,
    /// The nearest truthful fallback when not fully governed; absent when governed.
    pub nearest_truthful: Option<NearestTruthful>,
    /// Coverage status implied by the reduced gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// The per-consumer projections, in consumer order.
    pub consumer_projections: Vec<ConsumerProjection>,
    /// Stable message id for the reduced state; prefixed
    /// [`M5_ASSURANCE_CLAIM_REDUCER_MESSAGE_ID_PREFIX`].
    pub state_message_id: String,
}

impl ReducedClaim {
    /// Reduces a claim from the global precondition statuses, taking the worst gate among the claim's
    /// required preconditions so the reduced state can never read stronger than the trust facts behind
    /// it.
    fn reduce(
        subject: AssuranceClaimSubject,
        statuses: &[(ClaimPrecondition, PreconditionStatus)],
    ) -> Self {
        let required = required_preconditions(subject);
        let preconditions: Vec<PreconditionReading> = required
            .iter()
            .map(|p| {
                let status = statuses
                    .iter()
                    .find(|(precondition, _)| precondition == p)
                    .map(|(_, status)| *status)
                    .unwrap_or(PreconditionStatus::Satisfied);
                PreconditionReading::new(*p, status)
            })
            .collect();

        let reduced_gate = preconditions
            .iter()
            .map(|r| r.gate)
            .fold(DescriptorGate::Governed, worse_gate);
        let reduced_state = state_for_gate(reduced_gate);
        let effective_qualification = floor_for_gate(reduced_gate);

        let drifts: Vec<PreconditionDrift> = preconditions
            .iter()
            .filter_map(|r| PreconditionDrift::from_reading(subject, r))
            .collect();

        let evidence_refs: Vec<String> =
            preconditions.iter().map(|r| r.proof_ref.clone()).collect();

        let nearest_truthful = (reduced_gate != DescriptorGate::Governed).then(|| {
            let still_satisfied: Vec<ClaimPrecondition> = preconditions
                .iter()
                .filter(|r| r.gate == DescriptorGate::Governed)
                .map(|r| r.precondition)
                .collect();
            let drifted_preconditions: Vec<ClaimPrecondition> = preconditions
                .iter()
                .filter(|r| r.gate != DescriptorGate::Governed)
                .map(|r| r.precondition)
                .collect();
            // The fallback never overstates: if any precondition still holds the claim can be
            // attested at a weaker posture; if none holds it is unproven.
            let fallback_state = if still_satisfied.is_empty() {
                AssuranceClaimState::Unproven
            } else {
                AssuranceClaimState::Attested
            };
            NearestTruthful {
                fallback_posture: weaker_posture(subject.claimed_posture()),
                fallback_state,
                still_satisfied,
                drifted_preconditions,
                message_id: format!(
                    "{}claim.{}.nearest_truthful",
                    M5_ASSURANCE_CLAIM_REDUCER_MESSAGE_ID_PREFIX,
                    subject.as_str()
                ),
            }
        });

        let status = gate_status(reduced_gate);
        let consumer_projections = ReducerConsumer::ALL
            .iter()
            .map(|consumer| ConsumerProjection {
                consumer: *consumer,
                consumer_label: consumer.label().to_owned(),
                claim_state: reduced_state,
                effective_qualification,
                status,
                signal: status.signal(),
                owner_role: consumer.owner_role().to_owned(),
                converges_with_reduced: true,
                message_id: format!(
                    "{}claim.{}.consumer.{}",
                    M5_ASSURANCE_CLAIM_REDUCER_MESSAGE_ID_PREFIX,
                    subject.as_str(),
                    consumer.as_str()
                ),
            })
            .collect();

        Self {
            subject,
            subject_label: subject.label().to_owned(),
            claimed_posture: subject.claimed_posture(),
            trust_boundaries: subject.trust_boundaries(),
            owner_role: subject.owner_role().to_owned(),
            preconditions,
            reduced_state,
            reduced_state_label: reduced_state.label().to_owned(),
            reduced_gate,
            effective_qualification,
            drifts,
            evidence_refs,
            nearest_truthful,
            status,
            signal: status.signal(),
            consumer_projections,
            state_message_id: format!(
                "{}claim.{}.state",
                M5_ASSURANCE_CLAIM_REDUCER_MESSAGE_ID_PREFIX,
                subject.as_str()
            ),
        }
    }

    /// True when the claim stands fully proven at its claimed posture.
    pub fn is_governed(&self) -> bool {
        matches!(self.reduced_gate, DescriptorGate::Governed)
    }

    /// True when the claim auto-narrowed below its claimed posture.
    pub fn is_narrowed(&self) -> bool {
        matches!(self.reduced_gate, DescriptorGate::Narrowed)
    }

    /// True when the claim is blocked from its claimed posture.
    pub fn is_blocked(&self) -> bool {
        matches!(self.reduced_gate, DescriptorGate::Blocked)
    }

    /// Validates the reduced claim's invariants.
    fn validate(
        &self,
        statuses: &[(ClaimPrecondition, PreconditionStatus)],
    ) -> Vec<M5AssuranceClaimReducerViolation> {
        let mut out = Vec::new();
        let probe = Self::reduce(self.subject, statuses);
        if probe != *self {
            out.push(M5AssuranceClaimReducerViolation::ReducedClaimDrift);
        }
        if self.subject_label != self.subject.label()
            || self.owner_role != self.subject.owner_role()
            || self.claimed_posture != self.subject.claimed_posture()
            || self.trust_boundaries != self.subject.trust_boundaries()
        {
            out.push(M5AssuranceClaimReducerViolation::ClaimFieldMismatch);
        }
        // The reduced gate is the worst gate among the preconditions; the state and qualification
        // mirror it. A claim can never read stronger than its preconditions allow.
        let worst = self
            .preconditions
            .iter()
            .map(|r| r.gate)
            .fold(DescriptorGate::Governed, worse_gate);
        if self.reduced_gate != worst
            || self.reduced_state != state_for_gate(worst)
            || self.effective_qualification != floor_for_gate(worst)
            || self.status != gate_status(worst)
            || self.signal != self.status.signal()
        {
            out.push(M5AssuranceClaimReducerViolation::ClaimOverstatesPreconditions);
        }
        // Every drift is attributed to a non-satisfied precondition, and every non-satisfied
        // precondition is recorded as a drift.
        let drifted_count = self
            .preconditions
            .iter()
            .filter(|r| r.status != PreconditionStatus::Satisfied)
            .count();
        if self.drifts.len() != drifted_count
            || self.drifts.iter().any(|d| {
                !self
                    .preconditions
                    .iter()
                    .any(|r| r.precondition == d.precondition && r.status == d.status)
            })
        {
            out.push(M5AssuranceClaimReducerViolation::DriftAttributionInvalid);
        }
        // The nearest-truthful fallback is present exactly when not governed and never overstates.
        match (&self.nearest_truthful, self.is_governed()) {
            (Some(_), true) | (None, false) => {
                out.push(M5AssuranceClaimReducerViolation::FallbackPresenceInvalid);
            }
            (Some(f), false) => {
                if matches!(f.fallback_state, AssuranceClaimState::Proven)
                    || posture_rank(f.fallback_posture) > posture_rank(self.claimed_posture)
                {
                    out.push(M5AssuranceClaimReducerViolation::FallbackOverstates);
                }
            }
            (None, true) => {}
        }
        // Every consumer projection converges on the reduced state and qualification.
        if self.consumer_projections.len() != ReducerConsumer::ALL.len()
            || self.consumer_projections.iter().any(|p| {
                p.claim_state != self.reduced_state
                    || p.effective_qualification != self.effective_qualification
                    || p.status != self.status
                    || !p.converges_with_reduced
            })
        {
            out.push(M5AssuranceClaimReducerViolation::ConsumerDivergence);
        }
        for d in &self.drifts {
            if !d
                .cause_message_id
                .starts_with(M5_ASSURANCE_CLAIM_REDUCER_MESSAGE_ID_PREFIX)
            {
                out.push(M5AssuranceClaimReducerViolation::UnprefixedMessageId);
            }
        }
        if !self
            .state_message_id
            .starts_with(M5_ASSURANCE_CLAIM_REDUCER_MESSAGE_ID_PREFIX)
        {
            out.push(M5AssuranceClaimReducerViolation::UnprefixedMessageId);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Export preview
// ---------------------------------------------------------------------------------------------

/// One refs-only export entry: a reduced claim reduced to its subject, reduced state, qualification,
/// the drift tokens that narrowed it, the nearest truthful fallback, and the evidence refs — no raw
/// material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducerExportEntry {
    /// The claim subject.
    pub subject: AssuranceClaimSubject,
    /// The deployment posture the claim asserts.
    pub claimed_posture: ClaimedPosture,
    /// The reduced claim state.
    pub reduced_state: AssuranceClaimState,
    /// Effective qualification implied by the reduced gate.
    pub effective_qualification: QualificationClass,
    /// The named drift tokens that narrowed the claim, in canonical order.
    pub drift_tokens: Vec<DriftToken>,
    /// The nearest truthful posture when not governed.
    pub nearest_posture: Option<ClaimedPosture>,
    /// The nearest truthful state when not governed.
    pub nearest_state: Option<AssuranceClaimState>,
    /// The evidence refs backing the claim's preconditions.
    pub evidence_refs: Vec<String>,
}

/// The exported redaction-safe narrowing preview: the reduced claims reduced to refs and drift
/// tokens, plus the consumer set, so an offline procurement / evaluator review reads the same reduced
/// states the live surfaces show without leaking any raw material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceNarrowingExportPreview {
    /// Record kind; must equal [`M5_ASSURANCE_NARROWING_EXPORT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; mirrors the parent packet.
    pub schema_version: u32,
    /// Stable export-preview id.
    pub preview_id: String,
    /// The reducer packet this export was generated from.
    pub generated_from: String,
    /// The evaluation date the preview was computed as-of.
    pub evaluated_at: String,
    /// The reduced-claim export entries, in subject order.
    pub entries: Vec<ReducerExportEntry>,
    /// The consumer surfaces the reducer output governs.
    pub consumers: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
}

impl AssuranceNarrowingExportPreview {
    fn derive(
        preview_id: &str,
        generated_from: &str,
        evaluated_at: &str,
        redaction_class_token: &str,
        claims: &[ReducedClaim],
    ) -> Self {
        let entries = claims
            .iter()
            .map(|c| ReducerExportEntry {
                subject: c.subject,
                claimed_posture: c.claimed_posture,
                reduced_state: c.reduced_state,
                effective_qualification: c.effective_qualification,
                drift_tokens: c.drifts.iter().map(|d| d.drift).collect(),
                nearest_posture: c.nearest_truthful.as_ref().map(|f| f.fallback_posture),
                nearest_state: c.nearest_truthful.as_ref().map(|f| f.fallback_state),
                evidence_refs: c.evidence_refs.clone(),
            })
            .collect();
        Self {
            record_kind: M5_ASSURANCE_NARROWING_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_ASSURANCE_CLAIM_REDUCER_SCHEMA_VERSION,
            preview_id: preview_id.to_owned(),
            generated_from: generated_from.to_owned(),
            evaluated_at: evaluated_at.to_owned(),
            entries,
            consumers: tokens(&ReducerConsumer::ALL, |c| c.as_str()),
            redaction_class_token: redaction_class_token.to_owned(),
        }
    }

    /// Deterministic export-safe JSON for the preview.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only preview fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 assurance narrowing export preview serializes")
    }
}

// ---------------------------------------------------------------------------------------------
// Vocabulary / summary / conformance
// ---------------------------------------------------------------------------------------------

/// Self-describing controlled-vocabulary set so the packet resolves every token it carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceClaimReducerVocabulary {
    /// Claim-subject tokens.
    pub claim_subjects: Vec<String>,
    /// Claim-state tokens (the assurance-claim family).
    pub claim_states: Vec<String>,
    /// Precondition tokens.
    pub preconditions: Vec<String>,
    /// Precondition-status tokens.
    pub precondition_statuses: Vec<String>,
    /// Drift tokens.
    pub drift_tokens: Vec<String>,
    /// Restoration-action tokens.
    pub restoration_actions: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Deployment-profile (claimed-posture) tokens.
    pub deployment_profiles: Vec<String>,
    /// Trust-boundary tokens.
    pub trust_boundaries: Vec<String>,
    /// Evidence-class tokens.
    pub evidence_classes: Vec<String>,
    /// Qualification-class tokens.
    pub qualification_classes: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
}

impl AssuranceClaimReducerVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            claim_subjects: tokens(&AssuranceClaimSubject::ALL, |s| s.as_str()),
            claim_states: tokens(&AssuranceClaimState::ALL, |s| s.as_str()),
            preconditions: tokens(&ClaimPrecondition::ALL, |p| p.as_str()),
            precondition_statuses: tokens(&PreconditionStatus::ALL, |s| s.as_str()),
            drift_tokens: tokens(&DriftToken::ALL, |d| d.as_str()),
            restoration_actions: tokens(&RestorationAction::ALL, |a| a.as_str()),
            consumers: tokens(&ReducerConsumer::ALL, |c| c.as_str()),
            deployment_profiles: tokens(&ClaimedPosture::ALL, |p| p.as_str()),
            trust_boundaries: tokens(&TrustBoundary::ALL, |b| b.as_str()),
            evidence_classes: tokens(&EvidenceClass::ALL, |c| c.as_str()),
            qualification_classes: tokens(&QualificationClass::ALL, |q| q.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |g| g.as_str()),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Compact reducer summary — the scoreboard the overview and exports read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceClaimReducerSummary {
    /// Total reduced claims.
    pub total_claims: u32,
    /// Claims that stay proven.
    pub proven_claims: u32,
    /// Claims narrowed (under review).
    pub narrowed_claims: u32,
    /// Claims blocked (unproven).
    pub blocked_claims: u32,
    /// Total recorded precondition drifts across all claims.
    pub total_drifts: u32,
    /// Total consumer projections.
    pub total_projections: u32,
    /// Consumer projections that converge on their claim's reduced state.
    pub converged_projections: u32,
    /// True when at least one claim is blocked from its claimed posture.
    pub blocks_stable_promotion: bool,
}

/// Conformance review. Every flag is a hard invariant; all must hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceClaimReducerConformance {
    /// Every reduced claim derives its state from its preconditions.
    pub reduced_state_derived_from_preconditions: bool,
    /// No reduced claim reads stronger than its preconditions allow.
    pub no_claim_overstates_preconditions: bool,
    /// Stale evidence narrows the claims that depend on it.
    pub stale_evidence_narrows: bool,
    /// Hosted-dependency drift narrows the claims that depend on it.
    pub hosted_dependency_drift_narrows: bool,
    /// Key / residency mismatch blocks the claims that depend on it.
    pub key_residency_mismatch_blocks: bool,
    /// Policy-path regression blocks the claims that depend on it.
    pub policy_path_regression_blocks: bool,
    /// Every narrowed / blocked claim records which precondition drifted.
    pub drift_attributed_to_precondition: bool,
    /// Every consumer projects the same reduced state — one output governs all surfaces.
    pub consumers_converge_on_reduced_state: bool,
    /// No consumer reads a claim stronger than the reduced output.
    pub no_consumer_strengthens_after_narrowing: bool,
    /// Every not-fully-governed claim carries a nearest truthful fallback.
    pub nearest_truthful_present_when_not_governed: bool,
    /// No nearest-truthful fallback overstates its posture or state.
    pub nearest_truthful_never_overstates: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// The export carries no raw provider material — drift lineage stays refs-only.
    pub export_carries_no_raw_material: bool,
}

impl AssuranceClaimReducerConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.reduced_state_derived_from_preconditions
            && self.no_claim_overstates_preconditions
            && self.stale_evidence_narrows
            && self.hosted_dependency_drift_narrows
            && self.key_residency_mismatch_blocks
            && self.policy_path_regression_blocks
            && self.drift_attributed_to_precondition
            && self.consumers_converge_on_reduced_state
            && self.no_consumer_strengthens_after_narrowing
            && self.nearest_truthful_present_when_not_governed
            && self.nearest_truthful_never_overstates
            && self.controlled_enums_frozen
            && self.export_carries_no_raw_material
    }
}

// ---------------------------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------------------------

/// Constructor input for [`M5AssuranceClaimReducer::new`]. The only raw input is each precondition's
/// current status; everything else is derived. A precondition not listed defaults to satisfied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AssuranceClaimReducerInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// Each precondition's current status.
    pub precondition_states: Vec<(ClaimPrecondition, PreconditionStatus)>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable assurance-claim-reducer truth packet: the reduced claims,
/// the per-consumer projections, the exported redaction-safe preview, the controlled vocabulary, a
/// summary, and a conformance review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AssuranceClaimReducer {
    /// Record kind; must equal [`M5_ASSURANCE_CLAIM_REDUCER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ASSURANCE_CLAIM_REDUCER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The precondition statuses the reduction was computed from, in canonical order.
    pub precondition_states: Vec<PreconditionReading>,
    /// The reduced claims, in subject order.
    pub reduced_claims: Vec<ReducedClaim>,
    /// The exported redaction-safe narrowing preview.
    pub export_preview: AssuranceNarrowingExportPreview,
    /// Controlled-vocabulary set.
    pub vocabulary: AssuranceClaimReducerVocabulary,
    /// Compact summary.
    pub summary: AssuranceClaimReducerSummary,
    /// Conformance review block.
    pub conformance: AssuranceClaimReducerConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AssuranceClaimReducer {
    /// Builds a reducer packet from seed input, reducing each claim from the precondition statuses and
    /// deriving the summary / conformance / export preview from the reduced claims.
    pub fn new(input: M5AssuranceClaimReducerInput) -> Self {
        // Normalise the precondition statuses to one per kind, in canonical order, defaulting to
        // satisfied.
        let statuses: Vec<(ClaimPrecondition, PreconditionStatus)> = ClaimPrecondition::ALL
            .iter()
            .map(|p| {
                let status = input
                    .precondition_states
                    .iter()
                    .find(|(precondition, _)| precondition == p)
                    .map(|(_, status)| *status)
                    .unwrap_or(PreconditionStatus::Satisfied);
                (*p, status)
            })
            .collect();

        let precondition_states: Vec<PreconditionReading> = statuses
            .iter()
            .map(|(p, s)| PreconditionReading::new(*p, *s))
            .collect();

        let reduced_claims: Vec<ReducedClaim> = AssuranceClaimSubject::ALL
            .iter()
            .map(|s| ReducedClaim::reduce(*s, &statuses))
            .collect();

        let export_preview = AssuranceNarrowingExportPreview::derive(
            &format!("{}:export", input.packet_id),
            &input.packet_id,
            &input.evaluated_at,
            &input.redaction_class_token,
            &reduced_claims,
        );

        let summary = derive_summary(&reduced_claims);
        let conformance = derive_conformance(&reduced_claims, &export_preview);

        Self {
            record_kind: M5_ASSURANCE_CLAIM_REDUCER_RECORD_KIND.to_owned(),
            schema_version: M5_ASSURANCE_CLAIM_REDUCER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            precondition_states,
            reduced_claims,
            export_preview,
            vocabulary: AssuranceClaimReducerVocabulary::canonical(),
            summary,
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release automation must hold Stable promotion — at least one claim is blocked.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.summary.blocks_stable_promotion
    }

    /// Finds a reduced claim by subject.
    pub fn claim(&self, subject: AssuranceClaimSubject) -> Option<&ReducedClaim> {
        self.reduced_claims.iter().find(|c| c.subject == subject)
    }

    /// The global status of a precondition.
    pub fn precondition_status(
        &self,
        precondition: ClaimPrecondition,
    ) -> Option<PreconditionStatus> {
        self.precondition_states
            .iter()
            .find(|r| r.precondition == precondition)
            .map(|r| r.status)
    }

    /// The global precondition statuses as the reduction read them.
    fn statuses(&self) -> Vec<(ClaimPrecondition, PreconditionStatus)> {
        self.precondition_states
            .iter()
            .map(|r| (r.precondition, r.status))
            .collect()
    }

    /// Renders the packet for a generation channel. The output is identical for every channel — the
    /// channel parameter exists only to prove desktop, CLI / headless, and offline / mirror
    /// generation produce byte-identical output.
    pub fn render_for_channel(&self, _channel: AssuranceClaimReducerChannel) -> String {
        self.export_safe_json()
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 assurance claim reducer serializes")
    }

    /// The exported redaction-safe narrowing preview's JSON.
    pub fn render_export_preview(&self) -> String {
        self.export_preview.export_safe_json()
    }

    /// Deterministic, machine-readable claim / precondition matrix CSV: one row per (claim, required
    /// precondition) join, naming the claim, its reduced state, the precondition, its status, the gate
    /// it inflicts, the proof ref, and any drift.
    pub fn render_claims_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "claim,claimed_posture,reduced_state,effective_qualification,claim_owner,nearest_posture,nearest_state,precondition,status,gate,evidence_class,precondition_owner,proof_ref,drift\n",
        );
        for claim in &self.reduced_claims {
            let nearest_posture = claim
                .nearest_truthful
                .as_ref()
                .map(|f| f.fallback_posture.as_str())
                .unwrap_or("");
            let nearest_state = claim
                .nearest_truthful
                .as_ref()
                .map(|f| f.fallback_state.as_str())
                .unwrap_or("");
            for reading in &claim.preconditions {
                let drift = claim
                    .drifts
                    .iter()
                    .find(|d| d.precondition == reading.precondition)
                    .map(|d| d.drift.as_str())
                    .unwrap_or("");
                out.push_str(&format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                    claim.subject.as_str(),
                    claim.claimed_posture.as_str(),
                    claim.reduced_state.as_str(),
                    claim.effective_qualification.as_str(),
                    claim.owner_role,
                    nearest_posture,
                    nearest_state,
                    reading.precondition.as_str(),
                    reading.status.as_str(),
                    reading.gate.as_str(),
                    reading.evidence_class.as_str(),
                    reading.owner_role,
                    reading.proof_ref,
                    drift,
                ));
            }
        }
        out
    }

    /// Deterministic narrowing overview document for review, support, docs, or evaluator handoff.
    pub fn render_overview_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Assurance-Claim Reducer\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Claims: {} ({} proven, {} narrowed, {} blocked)\n",
            self.summary.total_claims,
            self.summary.proven_claims,
            self.summary.narrowed_claims,
            self.summary.blocked_claims
        ));
        out.push_str(&format!(
            "- Recorded drifts: {}\n",
            self.summary.total_drifts
        ));
        out.push_str(&format!(
            "- Consumer projections: {} ({} converged)\n",
            self.summary.total_projections, self.summary.converged_projections
        ));
        out.push_str(&format!(
            "- Stable promotion: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));

        out.push_str("\n## Precondition status\n\n");
        out.push_str("| Precondition | Status | Gate | Evidence class | Owner | Proof |\n");
        out.push_str("|--------------|--------|------|----------------|-------|-------|\n");
        for reading in &self.precondition_states {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                reading.precondition.as_str(),
                reading.status.as_str(),
                reading.gate.as_str(),
                reading.evidence_class.as_str(),
                reading.owner_role,
                reading.proof_ref
            ));
        }

        out.push_str("\n## Reduced claims\n\n");
        out.push_str(
            "| Claim | Claimed posture | Reduced state | Qualification | Drifts | Nearest truthful |\n",
        );
        out.push_str(
            "|-------|-----------------|---------------|---------------|--------|------------------|\n",
        );
        for claim in &self.reduced_claims {
            let nearest = claim
                .nearest_truthful
                .as_ref()
                .map(|f| {
                    format!(
                        "`{}` / `{}`",
                        f.fallback_posture.as_str(),
                        f.fallback_state.as_str()
                    )
                })
                .unwrap_or_else(|| "—".to_owned());
            let drifts = if claim.drifts.is_empty() {
                "—".to_owned()
            } else {
                join_tokens(
                    &claim.drifts.iter().map(|d| d.drift).collect::<Vec<_>>(),
                    |d| d.as_str(),
                )
            };
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} | {} |\n",
                claim.subject.as_str(),
                claim.claimed_posture.as_str(),
                claim.reduced_state.as_str(),
                claim.effective_qualification.as_str(),
                drifts,
                nearest
            ));
        }

        out.push_str("\n## Consumer convergence\n\n");
        out.push_str("Every consumer reads the same reduced state per claim — one reducer output governs all surfaces.\n\n");
        out.push_str("| Claim |");
        for consumer in ReducerConsumer::ALL {
            out.push_str(&format!(" {} |", consumer.label()));
        }
        out.push('\n');
        out.push_str("|-------|");
        for _ in ReducerConsumer::ALL {
            out.push_str("------|");
        }
        out.push('\n');
        for claim in &self.reduced_claims {
            out.push_str(&format!("| `{}` |", claim.subject.as_str()));
            for projection in &claim.consumer_projections {
                out.push_str(&format!(" `{}` |", projection.claim_state.as_str()));
            }
            out.push('\n');
        }
        out
    }

    /// Compact Markdown report for the release-grade narrowing proof.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Assurance-Claim Reducer — Proof\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Claims: {} ({} proven, {} narrowed, {} blocked)\n",
            self.summary.total_claims,
            self.summary.proven_claims,
            self.summary.narrowed_claims,
            self.summary.blocked_claims
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
            M5_ASSURANCE_NARROWING_EXPORT_PREVIEW_REF
        ));
        out.push_str(&format!(
            "- Claims CSV: `{}`\n",
            M5_ASSURANCE_CLAIM_REDUCER_CLAIMS_CSV_REF
        ));
        out
    }

    /// Validates the packet's invariants.
    pub fn validate(&self) -> Vec<M5AssuranceClaimReducerViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_ASSURANCE_CLAIM_REDUCER_RECORD_KIND {
            out.push(M5AssuranceClaimReducerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ASSURANCE_CLAIM_REDUCER_SCHEMA_VERSION {
            out.push(M5AssuranceClaimReducerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5AssuranceClaimReducerViolation::MissingIdentity);
        }

        // Precondition statuses: one per kind, in canonical order.
        let expected_preconditions: Vec<ClaimPrecondition> = ClaimPrecondition::ALL.to_vec();
        let seen: Vec<ClaimPrecondition> = self
            .precondition_states
            .iter()
            .map(|r| r.precondition)
            .collect();
        if seen != expected_preconditions {
            out.push(M5AssuranceClaimReducerViolation::PreconditionSetInvalid);
        }

        let statuses = self.statuses();

        // Every subject reduced exactly once, in canonical order, and self-consistent.
        let expected_subjects: Vec<AssuranceClaimSubject> = AssuranceClaimSubject::ALL.to_vec();
        let claim_subjects: Vec<AssuranceClaimSubject> =
            self.reduced_claims.iter().map(|c| c.subject).collect();
        if claim_subjects != expected_subjects {
            out.push(M5AssuranceClaimReducerViolation::ClaimSetInvalid);
        }
        for claim in &self.reduced_claims {
            out.extend(claim.validate(&statuses));
        }

        // Export preview re-derives from the claims and carries no raw material.
        let expected_preview = AssuranceNarrowingExportPreview::derive(
            &format!("{}:export", self.packet_id),
            &self.packet_id,
            &self.evaluated_at,
            &self.redaction_class_token,
            &self.reduced_claims,
        );
        if expected_preview != self.export_preview {
            out.push(M5AssuranceClaimReducerViolation::ExportPreviewDrift);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("reducer packet serializes"),
        ) {
            out.push(M5AssuranceClaimReducerViolation::RawMaterialInExport);
        }

        // Vocabulary, summary, conformance.
        if !self.vocabulary.matches_canonical() {
            out.push(M5AssuranceClaimReducerViolation::VocabularyMismatch);
        }
        if derive_summary(&self.reduced_claims) != self.summary {
            out.push(M5AssuranceClaimReducerViolation::SummaryDrift);
        }
        let conformance = derive_conformance(&self.reduced_claims, &self.export_preview);
        if conformance != self.conformance || !self.conformance.all_hold() {
            out.push(M5AssuranceClaimReducerViolation::ConformanceReviewFailed);
        }

        out.sort();
        out.dedup();
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Summary / conformance derivation
// ---------------------------------------------------------------------------------------------

fn derive_summary(claims: &[ReducedClaim]) -> AssuranceClaimReducerSummary {
    let proven = claims.iter().filter(|c| c.is_governed()).count() as u32;
    let narrowed = claims.iter().filter(|c| c.is_narrowed()).count() as u32;
    let blocked = claims.iter().filter(|c| c.is_blocked()).count() as u32;
    let total_drifts = claims.iter().map(|c| c.drifts.len() as u32).sum();
    let total_projections = claims
        .iter()
        .map(|c| c.consumer_projections.len() as u32)
        .sum();
    let converged_projections = claims
        .iter()
        .flat_map(|c| c.consumer_projections.iter())
        .filter(|p| p.converges_with_reduced)
        .count() as u32;
    AssuranceClaimReducerSummary {
        total_claims: claims.len() as u32,
        proven_claims: proven,
        narrowed_claims: narrowed,
        blocked_claims: blocked,
        total_drifts,
        total_projections,
        converged_projections,
        blocks_stable_promotion: blocked > 0,
    }
}

/// Probes the reduced gate of a claim that depends on `precondition` when that precondition takes
/// `status`, leaving every other precondition satisfied. Returns `None` when no claim depends on the
/// precondition.
fn probe_gate(
    precondition: ClaimPrecondition,
    status: PreconditionStatus,
) -> Option<DescriptorGate> {
    let subject = AssuranceClaimSubject::ALL
        .iter()
        .copied()
        .find(|s| required_preconditions(*s).contains(&precondition))?;
    let statuses = vec![(precondition, status)];
    Some(ReducedClaim::reduce(subject, &statuses).reduced_gate)
}

fn derive_conformance(
    claims: &[ReducedClaim],
    export_preview: &AssuranceNarrowingExportPreview,
) -> AssuranceClaimReducerConformance {
    let no_overstate = claims.iter().all(|c| {
        let worst = c
            .preconditions
            .iter()
            .map(|r| r.gate)
            .fold(DescriptorGate::Governed, worse_gate);
        c.reduced_gate == worst && c.reduced_state == state_for_gate(worst)
    });

    let drift_attributed = claims.iter().all(|c| {
        let drifted = c
            .preconditions
            .iter()
            .filter(|r| r.status != PreconditionStatus::Satisfied)
            .count();
        c.drifts.len() == drifted && (c.is_governed() == c.drifts.is_empty())
    });

    let consumers_converge = claims.iter().all(|c| {
        c.consumer_projections.len() == ReducerConsumer::ALL.len()
            && c.consumer_projections.iter().all(|p| {
                p.claim_state == c.reduced_state
                    && p.effective_qualification == c.effective_qualification
                    && p.converges_with_reduced
            })
    });

    // No consumer reads a claim stronger than the reduced output: every projection's gate-rank is at
    // least the reduced claim's gate-rank.
    let no_strengthen = claims.iter().all(|c| {
        c.consumer_projections.iter().all(|p| {
            gate_rank(qualification_gate(p.effective_qualification)) >= gate_rank(c.reduced_gate)
        })
    });

    let fallback_present = claims
        .iter()
        .all(|c| c.is_governed() == c.nearest_truthful.is_none());

    let fallback_never_overstates = claims.iter().all(|c| {
        c.nearest_truthful.as_ref().map_or(true, |f| {
            !matches!(f.fallback_state, AssuranceClaimState::Proven)
                && posture_rank(f.fallback_posture) <= posture_rank(c.claimed_posture)
        })
    });

    let export_clean = !json_contains_forbidden_material(
        &serde_json::to_value(export_preview).expect("export preview serializes"),
    );

    AssuranceClaimReducerConformance {
        reduced_state_derived_from_preconditions: claims
            .iter()
            .all(|c| c.reduced_state == state_for_gate(c.reduced_gate)),
        no_claim_overstates_preconditions: no_overstate,
        stale_evidence_narrows: probe_gate(
            ClaimPrecondition::EvidenceFreshness,
            PreconditionStatus::Drifted,
        ) == Some(DescriptorGate::Narrowed),
        hosted_dependency_drift_narrows: probe_gate(
            ClaimPrecondition::HostedDependencyBoundary,
            PreconditionStatus::Drifted,
        ) == Some(DescriptorGate::Narrowed),
        key_residency_mismatch_blocks: probe_gate(
            ClaimPrecondition::KeyResidency,
            PreconditionStatus::Invalidated,
        ) == Some(DescriptorGate::Blocked),
        policy_path_regression_blocks: probe_gate(
            ClaimPrecondition::PolicyControlPath,
            PreconditionStatus::Invalidated,
        ) == Some(DescriptorGate::Blocked),
        drift_attributed_to_precondition: drift_attributed,
        consumers_converge_on_reduced_state: consumers_converge,
        no_consumer_strengthens_after_narrowing: no_strengthen,
        nearest_truthful_present_when_not_governed: fallback_present,
        nearest_truthful_never_overstates: fallback_never_overstates,
        controlled_enums_frozen: AssuranceClaimReducerVocabulary::canonical().matches_canonical(),
        export_carries_no_raw_material: export_clean,
    }
}

/// The gate posture a qualification floor implies (the inverse of [`floor_for_gate`]).
const fn qualification_gate(qualification: QualificationClass) -> DescriptorGate {
    match qualification {
        QualificationClass::Stable => DescriptorGate::Governed,
        QualificationClass::Unavailable => DescriptorGate::Blocked,
        _ => DescriptorGate::Narrowed,
    }
}

// ---------------------------------------------------------------------------------------------
// Channels
// ---------------------------------------------------------------------------------------------

/// A generation channel the packet can be rendered for. Every channel produces byte-identical output;
/// the type exists to prove desktop, CLI / headless, and offline / mirror parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceClaimReducerChannel {
    /// The desktop surface.
    Desktop,
    /// The CLI / headless emitter.
    Headless,
    /// The offline / mirror generation path.
    OfflineMirror,
}

impl AssuranceClaimReducerChannel {
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
// Shared helpers
// ---------------------------------------------------------------------------------------------

/// Builds a token list from a typed `ALL` array.
fn tokens<T: Copy>(all: &[T], f: impl Fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|t| f(*t).to_owned()).collect()
}

/// Joins a token list for table / CSV rendering, space separated.
fn join_tokens<T: Copy>(items: &[T], f: impl Fn(T) -> &'static str) -> String {
    items
        .iter()
        .map(|t| f(*t))
        .collect::<Vec<_>>()
        .join(if items.len() > 8 { "," } else { " " })
}

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
// Violations
// ---------------------------------------------------------------------------------------------

/// Validation failures for the assurance-claim-reducer lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AssuranceClaimReducerViolation {
    /// The packet record kind is wrong.
    WrongRecordKind,
    /// The packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The precondition set is not the canonical set in canonical order.
    PreconditionSetInvalid,
    /// The claim set is not the canonical set in canonical order.
    ClaimSetInvalid,
    /// A reduced claim drifted from a fresh reduction of the preconditions.
    ReducedClaimDrift,
    /// A reduced claim cites a field that does not match its subject.
    ClaimFieldMismatch,
    /// A reduced claim reads stronger than its preconditions allow.
    ClaimOverstatesPreconditions,
    /// A claim's drift attribution does not match its precondition statuses.
    DriftAttributionInvalid,
    /// A claim's nearest-truthful fallback is present when governed or absent when not.
    FallbackPresenceInvalid,
    /// A claim's nearest-truthful fallback overstates its posture or state.
    FallbackOverstates,
    /// A consumer projection diverges from the reduced state.
    ConsumerDivergence,
    /// The export preview drifted from the reduced claims.
    ExportPreviewDrift,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// The summary disagrees with the reduced claims.
    SummaryDrift,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The export contains raw provider material.
    RawMaterialInExport,
}

impl M5AssuranceClaimReducerViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::PreconditionSetInvalid => "precondition_set_invalid",
            Self::ClaimSetInvalid => "claim_set_invalid",
            Self::ReducedClaimDrift => "reduced_claim_drift",
            Self::ClaimFieldMismatch => "claim_field_mismatch",
            Self::ClaimOverstatesPreconditions => "claim_overstates_preconditions",
            Self::DriftAttributionInvalid => "drift_attribution_invalid",
            Self::FallbackPresenceInvalid => "fallback_presence_invalid",
            Self::FallbackOverstates => "fallback_overstates",
            Self::ConsumerDivergence => "consumer_divergence",
            Self::ExportPreviewDrift => "export_preview_drift",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::SummaryDrift => "summary_drift",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}
