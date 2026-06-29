//! The M5 assurance center — the user / admin / evaluator-facing surface that turns Aureline's
//! regulated, sovereign, air-gapped, telemetry, residency, key-ownership, and local-first
//! continuity claims into inspectable product truth, bound to the same gate-bound state grammar the
//! [governance matrix](crate::m5_assurance_route_governance) froze.
//!
//! The governance matrix certifies *whether* each claimed surface (assurance center, governance
//! dashboard, route inspector, …) is mapped, narrowed, or blocked from its facets' proof freshness.
//! This lane is the layer above it: the assurance center a person actually reads. It answers, for a
//! given deployment profile, "what does Aureline claim, what proves it right now, what is held under
//! an accepted exception, and — when a stronger posture is not satisfied today — what is the nearest
//! statement that is still true?"
//!
//! The packet has four product parts, all *derived* from one input — each control's current proof
//! state and evidence freshness — so a card can never read stronger than its proof:
//!
//! - [`ClaimCard`]s. One per [claim subject](AssuranceClaimSubject). A card never asserts a fixed
//!   state; it *derives* its [active state](AssuranceClaimState) from the [control-proof
//!   rows](ControlProofRow) backing it, taking the worst gate among them. A claim with a stale
//!   control narrows to `under_review`; a claim with a waived control narrows to `exception_pending`;
//!   a claim with a missing control blocks to `unproven`. When the active state is not fully
//!   governed the card carries a [nearest truthful fallback](ClaimFallback) naming the weaker posture
//!   that *is* still proven, so the product never implies a posture the active path does not satisfy.
//! - [`ControlProofRow`]s. The controls a claim asserts are proven, each bound to one evidence
//!   class, freshness, owner, and a repo-relative proof ref drawn from the governance-matrix proofs —
//!   never a parallel evidence family.
//! - [`ExceptionWaiverRow`]s. The controls held under an accepted waiver, each disclosing its
//!   mitigation, expiry, compensating control, the responsible party, and the action that clears it.
//! - [`AssuranceOverview`]s. One per [deployment profile](crate::m5_assurance_route_governance::ClaimedPosture):
//!   the claim-state summary, evidence-freshness summary, known-exception count, the strongest
//!   *honored* posture (which auto-narrows below the profile when a claim cannot be proven), and the
//!   evaluation / export actions an evaluator can take without leaving the product.
//!
//! Finally the packet carries an [`EvaluationPacket`] export that reuses the exact claim-state and
//! proof vocabulary the cards show, so an exported evaluation pack and the in-product assurance
//! center can never drift to different copy. The [`M5AssuranceCenter`] packet is the one
//! inspectable, serde-serializable truth record this lane produces: it preserves proof lineage as
//! refs only and carries no credential bodies or raw provider payloads.
//!
//! - Packet schema:
//!   [`schemas/public-truth/m5-assurance-center.schema.json`](../../../../../schemas/public-truth/m5-assurance-center.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-assurance-center-contract.md`](../../../../../docs/public-truth/m5-assurance-center-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_assurance_center, seeded_m5_assurance_center_missing_evidence_blocked,
    seeded_m5_assurance_center_stale_evidence_narrowed, seeded_m5_assurance_center_waiver_narrowed,
    M5_ASSURANCE_CENTER_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The assurance center reuses the governance matrix's frozen claim-state / governance / posture /
// boundary vocabulary and the descriptor / badge gate runtime, so the in-product cards and the
// exported evaluation packet can never drift to a different state grammar.
use crate::m5_assurance_route_governance::{
    AssuranceClaimState, ClaimedPosture, EvidenceClass, GovernanceState, TrustBoundary,
};
use crate::m5_descriptor_badge::{
    ConsumerStatus, DescriptorGate, DescriptorSignal, FreshnessState, QualificationClass,
};

/// Record-kind tag carried by [`M5AssuranceCenter`].
pub const M5_ASSURANCE_CENTER_RECORD_KIND: &str = "m5_assurance_center";

/// Record-kind tag carried by the embedded [`EvaluationPacket`].
pub const M5_ASSURANCE_EVALUATION_RECORD_KIND: &str = "m5_assurance_evaluation_packet";

/// Schema version for the assurance-center packet.
pub const M5_ASSURANCE_CENTER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the assurance-center packet schema.
pub const M5_ASSURANCE_CENTER_SCHEMA_REF: &str =
    "schemas/public-truth/m5-assurance-center.schema.json";

/// Repo-relative path of the published assurance-center inventory.
pub const M5_ASSURANCE_CENTER_REGISTRY_REF: &str =
    "artifacts/public-truth/m5-assurance-center.json";

/// Repo-relative path of the rendered assurance-center overview document.
pub const M5_ASSURANCE_CENTER_OVERVIEW_REF: &str = "artifacts/public-truth/m5-assurance-center.md";

/// Repo-relative path of the machine-readable claim / control matrix export.
pub const M5_ASSURANCE_CENTER_CLAIMS_CSV_REF: &str =
    "artifacts/public-truth/m5-assurance-center-claims.csv";

/// Repo-relative path of the release-grade assurance-center parity proof.
pub const M5_ASSURANCE_CENTER_PROOF_REF: &str =
    "artifacts/public-truth/m5-assurance-center-proof/assurance-center.json";

/// Repo-relative path of the exported evaluation packet.
pub const M5_ASSURANCE_CENTER_EVALUATION_PACKET_REF: &str =
    "artifacts/public-truth/m5-assurance-center-proof/evaluation-packet.json";

/// Repo-relative path of the assurance-center contract doc.
pub const M5_ASSURANCE_CENTER_DOC_REF: &str = "docs/public-truth/m5-assurance-center-contract.md";

/// Repo-relative directory of the per-state assurance-center fixtures.
pub const M5_ASSURANCE_CENTER_FIXTURE_DIR: &str = "fixtures/public-truth/m5-assurance-center/";

/// Prefix every assurance-center message id carries so consumers can route it.
pub const M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX: &str = "public_truth.assurance_center.";

// ---------------------------------------------------------------------------------------------
// Claim subjects
// ---------------------------------------------------------------------------------------------

/// One assurance claim the center makes — the *subject* of a claim, distinct from its
/// [proof state](AssuranceClaimState). The set is the regulated / sovereign / air-gapped /
/// telemetry / residency / key-ownership lines the goal names, plus the local-first continuity
/// claim that holds under every profile; this lane invents no new compliance frameworks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceClaimSubject {
    /// Local editing continues regardless of control-plane availability.
    LocalFirstContinuity,
    /// Telemetry egress is controlled and inspectable.
    TelemetryControl,
    /// Encryption keys stay under customer ownership.
    KeyOwnership,
    /// Data stays pinned to its declared residency region.
    DataResidency,
    /// The deployment satisfies the regulated-operation posture.
    RegulatedOperation,
    /// The deployment runs air-gapped with no live vendor path.
    AirGapContainment,
    /// The deployment runs sovereign on a customer-owned control plane.
    SovereignDeployment,
}

impl AssuranceClaimSubject {
    /// Every claim subject, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LocalFirstContinuity,
        Self::TelemetryControl,
        Self::KeyOwnership,
        Self::DataResidency,
        Self::RegulatedOperation,
        Self::AirGapContainment,
        Self::SovereignDeployment,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFirstContinuity => "local_first_continuity",
            Self::TelemetryControl => "telemetry_control",
            Self::KeyOwnership => "key_ownership",
            Self::DataResidency => "data_residency",
            Self::RegulatedOperation => "regulated_operation",
            Self::AirGapContainment => "air_gap_containment",
            Self::SovereignDeployment => "sovereign_deployment",
        }
    }

    /// Reader-facing claim label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalFirstContinuity => "Local-first continuity",
            Self::TelemetryControl => "Telemetry control",
            Self::KeyOwnership => "Customer key ownership",
            Self::DataResidency => "Data residency",
            Self::RegulatedOperation => "Regulated operation",
            Self::AirGapContainment => "Air-gap containment",
            Self::SovereignDeployment => "Sovereign deployment",
        }
    }

    /// The deployment profile this claim is strongest under — the posture it asserts.
    pub const fn claimed_posture(self) -> ClaimedPosture {
        match self {
            Self::LocalFirstContinuity => ClaimedPosture::Managed,
            Self::TelemetryControl | Self::KeyOwnership => ClaimedPosture::SelfHosted,
            Self::DataResidency | Self::RegulatedOperation => ClaimedPosture::Regulated,
            Self::AirGapContainment | Self::SovereignDeployment => ClaimedPosture::Sovereign,
        }
    }

    /// Owner role accountable for keeping this claim's proof current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::LocalFirstContinuity => "assurance_center_owner",
            Self::TelemetryControl => "telemetry_governance_owner",
            Self::KeyOwnership => "key_custody_owner",
            Self::DataResidency => "data_residency_owner",
            Self::RegulatedOperation => "regulated_assurance_owner",
            Self::AirGapContainment => "air_gap_assurance_owner",
            Self::SovereignDeployment => "sovereign_assurance_owner",
        }
    }

    /// The trust boundaries this claim spans.
    pub fn trust_boundaries(self) -> Vec<TrustBoundary> {
        match self {
            Self::LocalFirstContinuity
            | Self::KeyOwnership
            | Self::AirGapContainment
            | Self::SovereignDeployment => vec![TrustBoundary::LocalFirst],
            Self::TelemetryControl | Self::DataResidency | Self::RegulatedOperation => {
                vec![TrustBoundary::LocalFirst, TrustBoundary::ControlPlane]
            }
        }
    }

    /// The controls this claim asserts are proven, in canonical order.
    pub fn required_controls(self) -> Vec<ControlId> {
        match self {
            Self::LocalFirstContinuity => vec![ControlId::LocalEditContinuity],
            Self::TelemetryControl => vec![ControlId::TelemetryEgressGate],
            Self::KeyOwnership => vec![
                ControlId::CustomerManagedKeyCustody,
                ControlId::LocalKeyEscrow,
            ],
            Self::DataResidency => vec![ControlId::DataResidencyPin],
            Self::RegulatedOperation => {
                vec![ControlId::RegulatedAuditTrail, ControlId::DataResidencyPin]
            }
            Self::AirGapContainment => {
                vec![ControlId::VendorPathSevered, ControlId::OfflineUpdatePath]
            }
            Self::SovereignDeployment => vec![
                ControlId::SovereignControlPlane,
                ControlId::CustomerManagedKeyCustody,
                ControlId::VendorPathSevered,
            ],
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Controls
// ---------------------------------------------------------------------------------------------

/// One control a claim asserts is proven. Each control owns one evidence class, owner role, and a
/// repo-relative proof ref pointing at the governance-matrix proof that backs it, so the assurance
/// center reuses the existing proof lanes rather than minting a parallel evidence family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlId {
    /// Local editing stays available without the control plane.
    LocalEditContinuity,
    /// Telemetry egress is gated and inspectable.
    TelemetryEgressGate,
    /// Encryption keys are customer-managed.
    CustomerManagedKeyCustody,
    /// A local key escrow keeps keys recoverable on-prem.
    LocalKeyEscrow,
    /// Data is pinned to a residency region.
    DataResidencyPin,
    /// A regulated-grade audit trail is captured.
    RegulatedAuditTrail,
    /// The live vendor path is severed.
    VendorPathSevered,
    /// An offline update path is available.
    OfflineUpdatePath,
    /// The control plane runs on customer-owned infrastructure.
    SovereignControlPlane,
}

impl ControlId {
    /// Every control, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::LocalEditContinuity,
        Self::TelemetryEgressGate,
        Self::CustomerManagedKeyCustody,
        Self::LocalKeyEscrow,
        Self::DataResidencyPin,
        Self::RegulatedAuditTrail,
        Self::VendorPathSevered,
        Self::OfflineUpdatePath,
        Self::SovereignControlPlane,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalEditContinuity => "local_edit_continuity",
            Self::TelemetryEgressGate => "telemetry_egress_gate",
            Self::CustomerManagedKeyCustody => "customer_managed_key_custody",
            Self::LocalKeyEscrow => "local_key_escrow",
            Self::DataResidencyPin => "data_residency_pin",
            Self::RegulatedAuditTrail => "regulated_audit_trail",
            Self::VendorPathSevered => "vendor_path_severed",
            Self::OfflineUpdatePath => "offline_update_path",
            Self::SovereignControlPlane => "sovereign_control_plane",
        }
    }

    /// Reader-facing control label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalEditContinuity => "Local edit continuity",
            Self::TelemetryEgressGate => "Telemetry egress gate",
            Self::CustomerManagedKeyCustody => "Customer-managed key custody",
            Self::LocalKeyEscrow => "Local key escrow",
            Self::DataResidencyPin => "Data residency pin",
            Self::RegulatedAuditTrail => "Regulated audit trail",
            Self::VendorPathSevered => "Vendor path severed",
            Self::OfflineUpdatePath => "Offline update path",
            Self::SovereignControlPlane => "Sovereign control plane",
        }
    }

    /// The evidence class that proves this control.
    pub const fn evidence_class(self) -> EvidenceClass {
        match self {
            Self::LocalEditContinuity | Self::SovereignControlPlane => {
                EvidenceClass::BoundaryManifest
            }
            Self::TelemetryEgressGate => EvidenceClass::PolicyBundle,
            Self::CustomerManagedKeyCustody | Self::LocalKeyEscrow | Self::RegulatedAuditTrail => {
                EvidenceClass::ControlAttestation
            }
            Self::DataResidencyPin => EvidenceClass::PolicyBundle,
            Self::VendorPathSevered => EvidenceClass::RouteTimeline,
            Self::OfflineUpdatePath => EvidenceClass::ProvenanceLedger,
        }
    }

    /// Owner role accountable for keeping this control's proof current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::LocalEditContinuity => "assurance_center_owner",
            Self::TelemetryEgressGate => "telemetry_governance_owner",
            Self::CustomerManagedKeyCustody | Self::LocalKeyEscrow => "key_custody_owner",
            Self::DataResidencyPin => "data_residency_owner",
            Self::RegulatedAuditTrail => "control_proof_owner",
            Self::VendorPathSevered => "route_explainability_owner",
            Self::OfflineUpdatePath => "event_provenance_owner",
            Self::SovereignControlPlane => "capability_boundary_owner",
        }
    }

    /// Repo-relative proof ref backing this control — drawn from the governance-matrix proofs.
    pub const fn proof_ref(self) -> &'static str {
        match self {
            Self::LocalEditContinuity | Self::SovereignControlPlane => {
                "artifacts/release-proof/m5-assurance-route-governance/capability-boundary.json"
            }
            Self::TelemetryEgressGate => {
                "artifacts/release-proof/m5-assurance-route-governance/governance-freshness.json"
            }
            Self::CustomerManagedKeyCustody | Self::LocalKeyEscrow => {
                "artifacts/release-proof/m5-assurance-route-governance/control-proof.json"
            }
            Self::DataResidencyPin | Self::RegulatedAuditTrail => {
                "artifacts/release-proof/m5-assurance-route-governance/assurance-claim.json"
            }
            Self::VendorPathSevered => {
                "artifacts/release-proof/m5-assurance-route-governance/route-hop.json"
            }
            Self::OfflineUpdatePath => {
                "artifacts/release-proof/m5-assurance-route-governance/event-provenance.json"
            }
        }
    }

    /// The claims this control backs, in canonical order.
    fn backs_claims(self) -> Vec<AssuranceClaimSubject> {
        AssuranceClaimSubject::ALL
            .iter()
            .copied()
            .filter(|s| s.required_controls().contains(&self))
            .collect()
    }
}

/// The responsible party for an accepted waiver's clearing action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponsibleParty {
    /// The deploying customer must act.
    Customer,
    /// The workspace / deployment admin must act.
    Admin,
    /// The vendor must act.
    Vendor,
}

impl ResponsibleParty {
    /// Every party, in declaration order.
    pub const ALL: [Self; 3] = [Self::Customer, Self::Admin, Self::Vendor];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Customer => "customer",
            Self::Admin => "admin",
            Self::Vendor => "vendor",
        }
    }
}

/// The action that clears an accepted waiver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaiverAction {
    /// Provide the missing proof and re-attest the control.
    ProvideProofAndReattest,
    /// Rotate the affected key or credential.
    RotateKeyMaterial,
    /// Review the waiver and renew or remove it before expiry.
    ReviewAndRenew,
    /// Enable the compensating control to remove the dependency.
    EnableCompensatingControl,
}

impl WaiverAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProvideProofAndReattest,
        Self::RotateKeyMaterial,
        Self::ReviewAndRenew,
        Self::EnableCompensatingControl,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvideProofAndReattest => "provide_proof_and_reattest",
            Self::RotateKeyMaterial => "rotate_key_material",
            Self::ReviewAndRenew => "review_and_renew",
            Self::EnableCompensatingControl => "enable_compensating_control",
        }
    }
}

/// An evaluation / export action an assurance overview offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationAction {
    /// Inspect a single claim, its controls, and its fallback.
    InspectClaim,
    /// Download the control-proof rows backing the claims.
    DownloadControlProofs,
    /// Review the open exception / waiver rows.
    ReviewExceptions,
    /// Export the evaluation packet for offline review.
    ExportEvaluationPacket,
}

impl EvaluationAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InspectClaim,
        Self::DownloadControlProofs,
        Self::ReviewExceptions,
        Self::ExportEvaluationPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectClaim => "inspect_claim",
            Self::DownloadControlProofs => "download_control_proofs",
            Self::ReviewExceptions => "review_exceptions",
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

/// The gate posture a control's evidence freshness implies on its own: current keeps it governed,
/// stale narrows, expired / missing block.
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

/// The nearest weaker posture below `posture`; the weakest posture maps to itself.
fn weaker_posture(posture: ClaimedPosture) -> ClaimedPosture {
    let rank = posture_rank(posture);
    if rank == 0 {
        ClaimedPosture::ALL[0]
    } else {
        ClaimedPosture::ALL[rank - 1]
    }
}

/// True when `subject`'s claimed posture is honored at or below `profile` — i.e. the profile is at
/// least as strong as the posture the claim asserts.
fn subject_applies(subject: AssuranceClaimSubject, profile: ClaimedPosture) -> bool {
    posture_rank(profile) >= posture_rank(subject.claimed_posture())
}

// ---------------------------------------------------------------------------------------------
// Control-proof rows
// ---------------------------------------------------------------------------------------------

/// One control-proof row: the control a claim asserts is proven, its current proof state, the
/// evidence class / freshness / owner / proof ref backing it, and the effective gate the proof
/// state and freshness together imply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlProofRow {
    /// The control.
    pub control: ControlId,
    /// Reader-facing control label.
    pub control_label: String,
    /// The claims this control backs, in canonical order.
    pub backs_claims: Vec<AssuranceClaimSubject>,
    /// The control's current proof state.
    pub proof_state: AssuranceClaimState,
    /// Reader-facing proof-state label.
    pub proof_state_label: String,
    /// The evidence class that proves the control.
    pub evidence_class: EvidenceClass,
    /// Freshness of the control's evidence.
    pub evidence_freshness: FreshnessState,
    /// Owner role accountable for the proof.
    pub owner_role: String,
    /// Repo-relative proof ref backing the control.
    pub proof_ref: String,
    /// The gate the proof state and freshness together imply (the more restrictive of the two).
    pub effective_gate: DescriptorGate,
    /// Coverage status implied by the effective gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl ControlProofRow {
    /// Builds a control-proof row, deriving every field from the control so a row can never cite a
    /// field that drifts from it. The effective gate is the more restrictive of the proof state's
    /// gate and the freshness gate, so a `proven` control with stale evidence still narrows.
    pub fn new(
        control: ControlId,
        proof_state: AssuranceClaimState,
        evidence_freshness: FreshnessState,
    ) -> Self {
        let effective_gate = worse_gate(
            proof_state.gate_posture(),
            freshness_gate(evidence_freshness),
        );
        let status = gate_status(effective_gate);
        Self {
            control,
            control_label: control.label().to_owned(),
            backs_claims: control.backs_claims(),
            proof_state,
            proof_state_label: proof_state.label().to_owned(),
            evidence_class: control.evidence_class(),
            evidence_freshness,
            owner_role: control.owner_role().to_owned(),
            proof_ref: control.proof_ref().to_owned(),
            effective_gate,
            status,
            signal: status.signal(),
            detail_message_id: format!(
                "{}control.{}",
                M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX,
                control.as_str()
            ),
        }
    }

    /// Validates the row's invariants: every derived field matches the control, the effective gate
    /// matches the proof state and freshness, the status mirrors the gate, and the message id
    /// carries the lane prefix.
    fn validate(&self) -> Vec<M5AssuranceCenterViolation> {
        let mut out = Vec::new();
        if self.control_label != self.control.label()
            || self.backs_claims != self.control.backs_claims()
            || self.evidence_class != self.control.evidence_class()
            || self.owner_role != self.control.owner_role()
            || self.proof_ref != self.control.proof_ref()
            || self.proof_state_label != self.proof_state.label()
        {
            out.push(M5AssuranceCenterViolation::ControlFieldMismatch);
        }
        let expected_gate = worse_gate(
            self.proof_state.gate_posture(),
            freshness_gate(self.evidence_freshness),
        );
        if self.effective_gate != expected_gate
            || self.status != gate_status(expected_gate)
            || self.signal != self.status.signal()
        {
            out.push(M5AssuranceCenterViolation::ControlGateDrift);
        }
        if self.proof_ref.trim().is_empty() {
            out.push(M5AssuranceCenterViolation::ControlEvidenceMissing);
        }
        if !self
            .detail_message_id
            .starts_with(M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX)
        {
            out.push(M5AssuranceCenterViolation::UnprefixedMessageId);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Claim cards
// ---------------------------------------------------------------------------------------------

/// The nearest truthful statement a claim falls back to when its full posture is not satisfied now:
/// the weaker posture that is still proven, the strongest state that weaker statement can carry, and
/// the controls that are still proven versus those that are not.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimFallback {
    /// The nearest weaker posture the claim can still truthfully assert.
    pub fallback_posture: ClaimedPosture,
    /// The strongest state the weaker statement can carry — never `proven`, so a fallback never
    /// overstates.
    pub fallback_state: AssuranceClaimState,
    /// Controls still proven (governed), in canonical order.
    pub still_proven_controls: Vec<ControlId>,
    /// Controls that narrowed or blocked the claim, in canonical order.
    pub unmet_controls: Vec<ControlId>,
    /// Stable message id; prefixed [`M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX`].
    pub message_id: String,
}

/// The kind of drift a claim's control inflicts on the claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimGapKind {
    /// A required control narrowed the claim below its full posture.
    ControlNarrowed,
    /// A required control blocked the claim.
    ControlBlocked,
}

impl ClaimGapKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlNarrowed => "control_narrowed",
            Self::ControlBlocked => "control_blocked",
        }
    }

    /// True when this gap blocks the claim (vs only narrowing it).
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::ControlBlocked)
    }
}

/// One coverage gap on a claim card: a required control that narrowed or blocked the claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimGap {
    /// The claim this gap applies to.
    pub claim: AssuranceClaimSubject,
    /// The control that drifted.
    pub control: ControlId,
    /// The kind of gap.
    pub gap_kind: ClaimGapKind,
    /// Stable message id; prefixed [`M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

/// One claim card: the subject, the posture it claims, the controls it requires, the active state
/// *derived* from those controls' proofs, the evidence links and owner backing it, and — when the
/// active state is not fully governed — the nearest truthful fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimCard {
    /// The claim subject.
    pub subject: AssuranceClaimSubject,
    /// Reader-facing claim label.
    pub subject_label: String,
    /// The deployment posture this claim asserts.
    pub claimed_posture: ClaimedPosture,
    /// The controls this claim requires, in canonical order.
    pub required_controls: Vec<ControlId>,
    /// The active claim state, derived from the worst gate among the required controls.
    pub active_state: AssuranceClaimState,
    /// Reader-facing active-state label.
    pub active_state_label: String,
    /// Gate posture the active state binds to.
    pub active_gate: DescriptorGate,
    /// Effective qualification implied by the active gate.
    pub effective_qualification: QualificationClass,
    /// The repo-relative proof refs of the required controls — evidence links, refs only.
    pub evidence_refs: Vec<String>,
    /// The evidence classes the required controls bind, in canonical order.
    pub evidence_classes: Vec<EvidenceClass>,
    /// The trust boundaries this claim spans.
    pub trust_boundaries: Vec<TrustBoundary>,
    /// Owner role accountable for the claim.
    pub owner_role: String,
    /// The nearest truthful fallback when the full posture is not satisfied; absent when governed.
    pub fallback: Option<ClaimFallback>,
    /// Coverage status implied by the active gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// The exact per-control gaps for this claim.
    pub gaps: Vec<ClaimGap>,
    /// Stable message id for the active state; prefixed [`M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX`].
    pub state_message_id: String,
}

impl ClaimCard {
    /// Derives a claim card from the control-proof rows, taking the worst gate among the required
    /// controls so the card can never read stronger than its proof.
    fn derive(subject: AssuranceClaimSubject, controls: &[ControlProofRow]) -> Self {
        let required = subject.required_controls();
        let rows: Vec<&ControlProofRow> = required
            .iter()
            .filter_map(|c| controls.iter().find(|r| r.control == *c))
            .collect();

        let worst_gate = rows
            .iter()
            .map(|r| r.effective_gate)
            .fold(DescriptorGate::Governed, worse_gate);

        let active_state = derive_claim_state(&rows, worst_gate);
        let effective_qualification = floor_for_gate(worst_gate);

        let evidence_refs: Vec<String> =
            required.iter().map(|c| c.proof_ref().to_owned()).collect();
        let mut evidence_classes: Vec<EvidenceClass> =
            required.iter().map(|c| c.evidence_class()).collect();
        evidence_classes.sort_by_key(|c| evidence_class_rank(*c));
        evidence_classes.dedup();

        // Gaps: one per required control whose effective gate is not governed.
        let mut gaps = Vec::new();
        for row in &rows {
            let kind = match row.effective_gate {
                DescriptorGate::Governed => continue,
                DescriptorGate::Narrowed => ClaimGapKind::ControlNarrowed,
                DescriptorGate::Blocked => ClaimGapKind::ControlBlocked,
            };
            gaps.push(ClaimGap {
                claim: subject,
                control: row.control,
                gap_kind: kind,
                cause_message_id: format!(
                    "{}claim.{}.{}.{}.gap",
                    M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX,
                    subject.as_str(),
                    row.control.as_str(),
                    kind.as_str()
                ),
            });
        }
        gaps.sort_by(|a, b| {
            control_rank(a.control)
                .cmp(&control_rank(b.control))
                .then(a.gap_kind.as_str().cmp(b.gap_kind.as_str()))
        });

        // Fallback: present whenever the claim is not fully governed.
        let fallback = (worst_gate != DescriptorGate::Governed).then(|| {
            let still_proven: Vec<ControlId> = rows
                .iter()
                .filter(|r| r.effective_gate == DescriptorGate::Governed)
                .map(|r| r.control)
                .collect();
            let unmet: Vec<ControlId> = rows
                .iter()
                .filter(|r| r.effective_gate != DescriptorGate::Governed)
                .map(|r| r.control)
                .collect();
            let fallback_state = if still_proven.is_empty() {
                AssuranceClaimState::Unproven
            } else {
                AssuranceClaimState::Attested
            };
            ClaimFallback {
                fallback_posture: weaker_posture(subject.claimed_posture()),
                fallback_state,
                still_proven_controls: still_proven,
                unmet_controls: unmet,
                message_id: format!(
                    "{}claim.{}.fallback",
                    M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX,
                    subject.as_str()
                ),
            }
        });

        let status = gate_status(worst_gate);
        Self {
            subject,
            subject_label: subject.label().to_owned(),
            claimed_posture: subject.claimed_posture(),
            required_controls: required,
            active_state,
            active_state_label: active_state.label().to_owned(),
            active_gate: worst_gate,
            effective_qualification,
            evidence_refs,
            evidence_classes,
            trust_boundaries: subject.trust_boundaries(),
            owner_role: subject.owner_role().to_owned(),
            fallback,
            status,
            signal: status.signal(),
            gaps,
            state_message_id: format!(
                "{}claim.{}.state",
                M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX,
                subject.as_str()
            ),
        }
    }

    /// True when the claim stands fully proven / attested at its claimed posture.
    pub fn is_governed(&self) -> bool {
        matches!(self.active_gate, DescriptorGate::Governed)
    }

    /// True when the claim auto-narrowed below its claimed posture.
    pub fn is_narrowed(&self) -> bool {
        matches!(self.active_gate, DescriptorGate::Narrowed)
    }

    /// True when the claim is blocked from its claimed posture.
    pub fn is_blocked(&self) -> bool {
        matches!(self.active_gate, DescriptorGate::Blocked)
    }

    /// Validates the card's invariants: the active state matches a fresh derivation from the
    /// controls, the fallback is present exactly when not governed and never overstates, and the
    /// message ids carry the lane prefix.
    fn validate(&self, controls: &[ControlProofRow]) -> Vec<M5AssuranceCenterViolation> {
        let mut out = Vec::new();
        let probe = Self::derive(self.subject, controls);
        if probe != *self {
            out.push(M5AssuranceCenterViolation::ClaimCardDrift);
        }
        if self.subject_label != self.subject.label()
            || self.owner_role != self.subject.owner_role()
            || self.required_controls != self.subject.required_controls()
            || self.claimed_posture != self.subject.claimed_posture()
        {
            out.push(M5AssuranceCenterViolation::ClaimFieldMismatch);
        }
        if self.effective_qualification != floor_for_gate(self.active_gate)
            || self.status != gate_status(self.active_gate)
            || self.signal != self.status.signal()
        {
            out.push(M5AssuranceCenterViolation::ClaimGateDrift);
        }
        // A card must never read stronger than its controls allow.
        let worst = self
            .required_controls
            .iter()
            .filter_map(|c| controls.iter().find(|r| r.control == *c))
            .map(|r| r.effective_gate)
            .fold(DescriptorGate::Governed, worse_gate);
        if self.active_gate != worst {
            out.push(M5AssuranceCenterViolation::ClaimOverstatesControls);
        }
        match (&self.fallback, self.active_gate) {
            (Some(f), DescriptorGate::Narrowed | DescriptorGate::Blocked) => {
                if matches!(f.fallback_state, AssuranceClaimState::Proven)
                    || posture_rank(f.fallback_posture) > posture_rank(self.claimed_posture)
                    || !f
                        .message_id
                        .starts_with(M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX)
                {
                    out.push(M5AssuranceCenterViolation::FallbackOverstates);
                }
            }
            (None, DescriptorGate::Governed) => {}
            _ => out.push(M5AssuranceCenterViolation::FallbackPresenceInvalid),
        }
        if !self
            .state_message_id
            .starts_with(M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX)
        {
            out.push(M5AssuranceCenterViolation::UnprefixedMessageId);
        }
        out
    }
}

/// Maps the worst gate among a claim's controls to the active claim state, never overstating:
/// fully governed reads `proven` only when every control is `proven`, else `attested`; a narrowed
/// claim reads `exception_pending` when a control is waived, else `under_review`; a blocked claim
/// reads `unproven`.
fn derive_claim_state(
    rows: &[&ControlProofRow],
    worst_gate: DescriptorGate,
) -> AssuranceClaimState {
    match worst_gate {
        DescriptorGate::Governed => {
            if rows
                .iter()
                .all(|r| matches!(r.proof_state, AssuranceClaimState::Proven))
            {
                AssuranceClaimState::Proven
            } else {
                AssuranceClaimState::Attested
            }
        }
        DescriptorGate::Narrowed => {
            if rows
                .iter()
                .any(|r| matches!(r.proof_state, AssuranceClaimState::ExceptionPending))
            {
                AssuranceClaimState::ExceptionPending
            } else {
                AssuranceClaimState::UnderReview
            }
        }
        DescriptorGate::Blocked => AssuranceClaimState::Unproven,
    }
}

// ---------------------------------------------------------------------------------------------
// Exception / waiver rows
// ---------------------------------------------------------------------------------------------

/// Seed input for one accepted waiver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionWaiverSeed {
    /// The control held under the waiver.
    pub control: ControlId,
    /// Short mitigation summary (no secrets).
    pub mitigation: String,
    /// The waiver's expiry date.
    pub expiry: String,
    /// The compensating control standing in for the waived one.
    pub compensating_control: ControlId,
    /// The party responsible for clearing the waiver.
    pub responsible_party: ResponsibleParty,
    /// The action that clears the waiver.
    pub action: WaiverAction,
}

/// One exception / waiver row: a control held under an accepted waiver, disclosing its mitigation,
/// expiry, compensating control, the responsible party, the action that clears it, and the claims
/// it narrows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExceptionWaiverRow {
    /// The waived control.
    pub control: ControlId,
    /// Reader-facing control label.
    pub control_label: String,
    /// The governance state this row reads — always `waived`.
    pub governance_state: GovernanceState,
    /// The claims this waiver narrows, in canonical order.
    pub affected_claims: Vec<AssuranceClaimSubject>,
    /// Short mitigation summary (no secrets).
    pub mitigation: String,
    /// The waiver's expiry date.
    pub expiry: String,
    /// The compensating control standing in for the waived one.
    pub compensating_control: ControlId,
    /// The party responsible for clearing the waiver.
    pub responsible_party: ResponsibleParty,
    /// The action that clears the waiver.
    pub action: WaiverAction,
    /// Coverage status — waivers narrow, so always provisional.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl ExceptionWaiverRow {
    /// Builds an exception row from its seed, deriving the affected claims and status.
    fn from_seed(seed: &ExceptionWaiverSeed) -> Self {
        let status = ConsumerStatus::Provisional;
        Self {
            control: seed.control,
            control_label: seed.control.label().to_owned(),
            governance_state: GovernanceState::Waived,
            affected_claims: seed.control.backs_claims(),
            mitigation: seed.mitigation.clone(),
            expiry: seed.expiry.clone(),
            compensating_control: seed.compensating_control,
            responsible_party: seed.responsible_party,
            action: seed.action,
            status,
            signal: status.signal(),
            detail_message_id: format!(
                "{}exception.{}",
                M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX,
                seed.control.as_str()
            ),
        }
    }

    /// Validates the row's invariants: the disclosure fields are present and the message id carries
    /// the lane prefix.
    fn validate(&self) -> Vec<M5AssuranceCenterViolation> {
        let mut out = Vec::new();
        if self.control_label != self.control.label()
            || self.affected_claims != self.control.backs_claims()
            || self.governance_state != GovernanceState::Waived
        {
            out.push(M5AssuranceCenterViolation::ExceptionFieldMismatch);
        }
        if self.mitigation.trim().is_empty()
            || self.expiry.trim().is_empty()
            || self.compensating_control == self.control
        {
            out.push(M5AssuranceCenterViolation::ExceptionDisclosureIncomplete);
        }
        if self.status != ConsumerStatus::Provisional || self.signal != self.status.signal() {
            out.push(M5AssuranceCenterViolation::ExceptionStatusDrift);
        }
        if !self
            .detail_message_id
            .starts_with(M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX)
        {
            out.push(M5AssuranceCenterViolation::UnprefixedMessageId);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Per-profile overviews
// ---------------------------------------------------------------------------------------------

/// Count of claim cards by active state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimStateCounts {
    /// Claims fully proven.
    pub proven: u32,
    /// Claims attested.
    pub attested: u32,
    /// Claims under review.
    pub under_review: u32,
    /// Claims held under an exception.
    pub exception_pending: u32,
    /// Claims unproven / blocked.
    pub unproven: u32,
}

/// Count of evidence by freshness across a set of controls.
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

/// One per-profile assurance overview: the applicable claims, the claim-state and evidence-freshness
/// summaries, the known-exception count, the strongest *honored* posture, and the evaluation /
/// export actions an evaluator can take from this profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceOverview {
    /// The deployment profile this overview is shown under.
    pub profile: ClaimedPosture,
    /// The claims applicable to this profile, in canonical order.
    pub applicable_claims: Vec<AssuranceClaimSubject>,
    /// Claim-state counts over the applicable claims.
    pub claim_state_counts: ClaimStateCounts,
    /// Evidence-freshness counts over the applicable claims' controls.
    pub evidence_freshness_counts: FreshnessCounts,
    /// Open exceptions affecting an applicable claim.
    pub known_exception_count: u32,
    /// The strongest posture every applicable claim is governed at — never above the profile.
    pub effective_posture: ClaimedPosture,
    /// Gate decision for the profile (worst gate among the applicable claims).
    pub gate_decision: DescriptorGate,
    /// Effective qualification implied by the gate decision.
    pub effective_qualification: QualificationClass,
    /// Coverage status (mirrors [`Self::gate_decision`]).
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// The evaluation / export actions offered from this profile.
    pub evaluation_actions: Vec<EvaluationAction>,
    /// Stable message id; prefixed [`M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
}

impl AssuranceOverview {
    /// Derives a per-profile overview from the claim cards, control rows, and exception rows.
    fn derive(
        profile: ClaimedPosture,
        cards: &[ClaimCard],
        controls: &[ControlProofRow],
        exceptions: &[ExceptionWaiverRow],
    ) -> Self {
        let applicable: Vec<AssuranceClaimSubject> = AssuranceClaimSubject::ALL
            .iter()
            .copied()
            .filter(|s| subject_applies(*s, profile))
            .collect();
        let applicable_cards: Vec<&ClaimCard> = applicable
            .iter()
            .filter_map(|s| cards.iter().find(|c| c.subject == *s))
            .collect();

        let mut counts = ClaimStateCounts {
            proven: 0,
            attested: 0,
            under_review: 0,
            exception_pending: 0,
            unproven: 0,
        };
        for card in &applicable_cards {
            match card.active_state {
                AssuranceClaimState::Proven => counts.proven += 1,
                AssuranceClaimState::Attested => counts.attested += 1,
                AssuranceClaimState::UnderReview => counts.under_review += 1,
                AssuranceClaimState::ExceptionPending => counts.exception_pending += 1,
                AssuranceClaimState::Unproven => counts.unproven += 1,
            }
        }

        // Freshness across the applicable claims' controls (deduped).
        let mut control_set: Vec<ControlId> = applicable
            .iter()
            .flat_map(|s| s.required_controls())
            .collect();
        control_set.sort_by_key(|c| control_rank(*c));
        control_set.dedup();
        let mut fresh = FreshnessCounts {
            current: 0,
            stale: 0,
            expired: 0,
            missing: 0,
        };
        for control in &control_set {
            if let Some(row) = controls.iter().find(|r| r.control == *control) {
                match row.evidence_freshness {
                    FreshnessState::Current => fresh.current += 1,
                    FreshnessState::Stale => fresh.stale += 1,
                    FreshnessState::Expired => fresh.expired += 1,
                    FreshnessState::Missing => fresh.missing += 1,
                }
            }
        }

        let known_exception_count = exceptions
            .iter()
            .filter(|e| control_set.contains(&e.control))
            .count() as u32;

        let gate_decision = applicable_cards
            .iter()
            .map(|c| c.active_gate)
            .fold(DescriptorGate::Governed, worse_gate);

        let effective_posture = strongest_honored_posture(profile, cards);

        let status = gate_status(gate_decision);
        Self {
            profile,
            applicable_claims: applicable,
            claim_state_counts: counts,
            evidence_freshness_counts: fresh,
            known_exception_count,
            effective_posture,
            gate_decision,
            effective_qualification: floor_for_gate(gate_decision),
            status,
            signal: status.signal(),
            evaluation_actions: EvaluationAction::ALL.to_vec(),
            summary_message_id: format!(
                "{}overview.{}",
                M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX,
                profile.as_str()
            ),
        }
    }

    /// Validates the overview's invariants: the effective posture never exceeds the profile, the
    /// gate and qualification agree, and the message id carries the lane prefix.
    fn validate(
        &self,
        cards: &[ClaimCard],
        controls: &[ControlProofRow],
        exceptions: &[ExceptionWaiverRow],
    ) -> Vec<M5AssuranceCenterViolation> {
        let mut out = Vec::new();
        let probe = Self::derive(self.profile, cards, controls, exceptions);
        if probe != *self {
            out.push(M5AssuranceCenterViolation::OverviewDrift);
        }
        if posture_rank(self.effective_posture) > posture_rank(self.profile) {
            out.push(M5AssuranceCenterViolation::OverviewOverstatesPosture);
        }
        if self.effective_qualification != floor_for_gate(self.gate_decision)
            || self.status != gate_status(self.gate_decision)
            || self.signal != self.status.signal()
        {
            out.push(M5AssuranceCenterViolation::OverviewGateDrift);
        }
        if !self
            .summary_message_id
            .starts_with(M5_ASSURANCE_CENTER_MESSAGE_ID_PREFIX)
        {
            out.push(M5AssuranceCenterViolation::UnprefixedMessageId);
        }
        out
    }
}

/// The strongest posture at or below `profile` whose every applicable claim is governed. This is the
/// overview's honest "effective posture": it auto-narrows below the profile the moment a claim it
/// would imply cannot be proven, and never reads above the profile.
fn strongest_honored_posture(profile: ClaimedPosture, cards: &[ClaimCard]) -> ClaimedPosture {
    let mut best = ClaimedPosture::ALL[0];
    for &candidate in ClaimedPosture::ALL.iter() {
        if posture_rank(candidate) > posture_rank(profile) {
            continue;
        }
        let all_governed = AssuranceClaimSubject::ALL
            .iter()
            .copied()
            .filter(|s| subject_applies(*s, candidate))
            .all(|s| {
                cards
                    .iter()
                    .find(|c| c.subject == s)
                    .is_some_and(ClaimCard::is_governed)
            });
        if all_governed && posture_rank(candidate) >= posture_rank(best) {
            best = candidate;
        }
    }
    best
}

// ---------------------------------------------------------------------------------------------
// Evaluation packet (export)
// ---------------------------------------------------------------------------------------------

/// One claim entry in the exported evaluation packet — the same claim-state vocabulary the card
/// shows, reduced to refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationClaimEntry {
    /// The claim subject token.
    pub subject: AssuranceClaimSubject,
    /// The claimed posture.
    pub claimed_posture: ClaimedPosture,
    /// The active state token.
    pub active_state: AssuranceClaimState,
    /// Effective qualification.
    pub effective_qualification: QualificationClass,
    /// Owner role.
    pub owner_role: String,
    /// Proof refs (refs only).
    pub evidence_refs: Vec<String>,
    /// The fallback posture, when present.
    pub fallback_posture: Option<ClaimedPosture>,
    /// The fallback state token, when present.
    pub fallback_state: Option<AssuranceClaimState>,
}

/// One control entry in the exported evaluation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationControlEntry {
    /// The control token.
    pub control: ControlId,
    /// The proof-state token.
    pub proof_state: AssuranceClaimState,
    /// The evidence class.
    pub evidence_class: EvidenceClass,
    /// The evidence freshness.
    pub evidence_freshness: FreshnessState,
    /// The proof ref (refs only).
    pub proof_ref: String,
    /// Owner role.
    pub owner_role: String,
}

/// One exception entry in the exported evaluation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationExceptionEntry {
    /// The waived control token.
    pub control: ControlId,
    /// The governance-state token.
    pub governance_state: GovernanceState,
    /// The waiver expiry.
    pub expiry: String,
    /// The compensating control token.
    pub compensating_control: ControlId,
    /// The responsible party.
    pub responsible_party: ResponsibleParty,
    /// The claims affected.
    pub affected_claims: Vec<AssuranceClaimSubject>,
}

/// The exported evaluation packet: the claim cards, control proofs, and exception rows reduced to
/// the exact claim-state and proof vocabulary the in-product assurance center shows, so an exported
/// pack and the live UI can never read differently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationPacket {
    /// Record kind; must equal [`M5_ASSURANCE_EVALUATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; mirrors the parent packet.
    pub schema_version: u32,
    /// Stable evaluation-packet id.
    pub packet_id: String,
    /// The assurance-center packet this export was generated from.
    pub generated_from: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The claim entries.
    pub claims: Vec<EvaluationClaimEntry>,
    /// The control entries.
    pub controls: Vec<EvaluationControlEntry>,
    /// The exception entries.
    pub exceptions: Vec<EvaluationExceptionEntry>,
    /// The controlled vocabulary the entries draw from.
    pub vocabulary: AssuranceCenterVocabulary,
    /// Packet redaction class token.
    pub redaction_class_token: String,
}

impl EvaluationPacket {
    /// Builds the evaluation packet from the assurance-center parts.
    fn derive(
        packet_id: &str,
        generated_from: &str,
        evaluated_at: &str,
        redaction_class_token: &str,
        cards: &[ClaimCard],
        controls: &[ControlProofRow],
        exceptions: &[ExceptionWaiverRow],
    ) -> Self {
        let claims = cards
            .iter()
            .map(|c| EvaluationClaimEntry {
                subject: c.subject,
                claimed_posture: c.claimed_posture,
                active_state: c.active_state,
                effective_qualification: c.effective_qualification,
                owner_role: c.owner_role.clone(),
                evidence_refs: c.evidence_refs.clone(),
                fallback_posture: c.fallback.as_ref().map(|f| f.fallback_posture),
                fallback_state: c.fallback.as_ref().map(|f| f.fallback_state),
            })
            .collect();
        let control_entries = controls
            .iter()
            .map(|r| EvaluationControlEntry {
                control: r.control,
                proof_state: r.proof_state,
                evidence_class: r.evidence_class,
                evidence_freshness: r.evidence_freshness,
                proof_ref: r.proof_ref.clone(),
                owner_role: r.owner_role.clone(),
            })
            .collect();
        let exception_entries = exceptions
            .iter()
            .map(|e| EvaluationExceptionEntry {
                control: e.control,
                governance_state: e.governance_state,
                expiry: e.expiry.clone(),
                compensating_control: e.compensating_control,
                responsible_party: e.responsible_party,
                affected_claims: e.affected_claims.clone(),
            })
            .collect();
        Self {
            record_kind: M5_ASSURANCE_EVALUATION_RECORD_KIND.to_owned(),
            schema_version: M5_ASSURANCE_CENTER_SCHEMA_VERSION,
            packet_id: packet_id.to_owned(),
            generated_from: generated_from.to_owned(),
            evaluated_at: evaluated_at.to_owned(),
            claims,
            controls: control_entries,
            exceptions: exception_entries,
            vocabulary: AssuranceCenterVocabulary::canonical(),
            redaction_class_token: redaction_class_token.to_owned(),
        }
    }

    /// Deterministic export-safe JSON for the evaluation packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 assurance evaluation packet serializes")
    }

    /// True when every token the packet carries is a member of the canonical vocabulary, so the
    /// export reuses the same grammar the UI shows.
    fn reuses_canonical_vocabulary(&self) -> bool {
        if !self.vocabulary.matches_canonical() {
            return false;
        }
        let vocab = &self.vocabulary;
        self.claims.iter().all(|c| {
            vocab
                .claim_subjects
                .contains(&c.subject.as_str().to_owned())
                && vocab
                    .claim_states
                    .contains(&c.active_state.as_str().to_owned())
                && c.fallback_state
                    .map_or(true, |s| vocab.claim_states.contains(&s.as_str().to_owned()))
        }) && self.controls.iter().all(|c| {
            vocab.controls.contains(&c.control.as_str().to_owned())
                && vocab
                    .claim_states
                    .contains(&c.proof_state.as_str().to_owned())
        }) && self.exceptions.iter().all(|e| {
            vocab
                .governance_states
                .contains(&e.governance_state.as_str().to_owned())
        })
    }
}

// ---------------------------------------------------------------------------------------------
// Vocabulary / summary / conformance
// ---------------------------------------------------------------------------------------------

/// Self-describing controlled-vocabulary set so the packet resolves every token it carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceCenterVocabulary {
    /// Claim-subject tokens.
    pub claim_subjects: Vec<String>,
    /// Claim-state tokens (the assurance-claim family).
    pub claim_states: Vec<String>,
    /// Control tokens.
    pub controls: Vec<String>,
    /// Governance-state tokens.
    pub governance_states: Vec<String>,
    /// Deployment-profile (claimed-posture) tokens.
    pub deployment_profiles: Vec<String>,
    /// Trust-boundary tokens.
    pub trust_boundaries: Vec<String>,
    /// Evidence-class tokens.
    pub evidence_classes: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Qualification-class tokens.
    pub qualification_classes: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
    /// Responsible-party tokens.
    pub responsible_parties: Vec<String>,
    /// Waiver-action tokens.
    pub waiver_actions: Vec<String>,
    /// Evaluation-action tokens.
    pub evaluation_actions: Vec<String>,
}

impl AssuranceCenterVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            claim_subjects: tokens(&AssuranceClaimSubject::ALL, |s| s.as_str()),
            claim_states: tokens(&AssuranceClaimState::ALL, |s| s.as_str()),
            controls: tokens(&ControlId::ALL, |c| c.as_str()),
            governance_states: tokens(&GovernanceState::ALL, |g| g.as_str()),
            deployment_profiles: tokens(&ClaimedPosture::ALL, |p| p.as_str()),
            trust_boundaries: tokens(&TrustBoundary::ALL, |b| b.as_str()),
            evidence_classes: tokens(&EvidenceClass::ALL, |c| c.as_str()),
            freshness_states: tokens(&FreshnessState::ALL, |f| f.as_str()),
            qualification_classes: tokens(&QualificationClass::ALL, |q| q.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |g| g.as_str()),
            responsible_parties: tokens(&ResponsibleParty::ALL, |p| p.as_str()),
            waiver_actions: tokens(&WaiverAction::ALL, |a| a.as_str()),
            evaluation_actions: tokens(&EvaluationAction::ALL, |a| a.as_str()),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Compact assurance-center summary — the scoreboard the overview and exports read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceCenterSummary {
    /// Total claim cards.
    pub total_claims: u32,
    /// Claims proven.
    pub proven_claims: u32,
    /// Claims attested.
    pub attested_claims: u32,
    /// Claims narrowed (under review or exception pending).
    pub narrowed_claims: u32,
    /// Claims unproven / blocked.
    pub blocked_claims: u32,
    /// Total controls.
    pub total_controls: u32,
    /// Controls governed.
    pub governed_controls: u32,
    /// Controls narrowed.
    pub narrowed_controls: u32,
    /// Controls blocked.
    pub blocked_controls: u32,
    /// Total exceptions.
    pub total_exceptions: u32,
    /// Total deployment profiles.
    pub total_profiles: u32,
    /// Profiles whose claimed posture is fully honored.
    pub honored_profiles: u32,
    /// True when at least one claim is blocked from its claimed posture.
    pub blocks_stable_promotion: bool,
}

/// Conformance review. Every flag is a hard invariant; all must hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssuranceCenterConformance {
    /// Every claim card derives its state from its controls.
    pub claim_state_derived_from_controls: bool,
    /// No claim card reads stronger than its controls allow.
    pub no_claim_overstates_controls: bool,
    /// Every control binds an evidence class, owner, and proof ref.
    pub every_control_bound_to_evidence_and_owner: bool,
    /// Stale evidence narrows the claims that read it deterministically.
    pub stale_evidence_narrows_deterministically: bool,
    /// Missing / expired evidence blocks the claims that read it.
    pub missing_evidence_blocks_stable_promotion: bool,
    /// Every exception discloses mitigation, expiry, compensating control, and an action.
    pub exceptions_disclose_mitigation_expiry_and_action: bool,
    /// Every not-fully-governed claim carries a nearest truthful fallback.
    pub fallback_present_when_not_governed: bool,
    /// No overview reads a posture above its profile.
    pub overview_effective_posture_never_overstated: bool,
    /// The exported evaluation packet reuses the in-product claim-state and proof vocabulary.
    pub evaluation_packet_reuses_ui_vocabulary: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// The export carries no raw provider material — proof lineage stays refs-only.
    pub export_carries_no_raw_material: bool,
}

impl AssuranceCenterConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.claim_state_derived_from_controls
            && self.no_claim_overstates_controls
            && self.every_control_bound_to_evidence_and_owner
            && self.stale_evidence_narrows_deterministically
            && self.missing_evidence_blocks_stable_promotion
            && self.exceptions_disclose_mitigation_expiry_and_action
            && self.fallback_present_when_not_governed
            && self.overview_effective_posture_never_overstated
            && self.evaluation_packet_reuses_ui_vocabulary
            && self.controlled_enums_frozen
            && self.export_carries_no_raw_material
    }
}

// ---------------------------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------------------------

/// Constructor input for [`M5AssuranceCenter::new`]. The only raw inputs are each control's proof
/// state and evidence freshness plus the accepted waivers; everything else is derived.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AssuranceCenterInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// Each control's current proof state and evidence freshness.
    pub control_states: Vec<(ControlId, AssuranceClaimState, FreshnessState)>,
    /// The accepted waivers.
    pub exceptions: Vec<ExceptionWaiverSeed>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable assurance-center truth packet: the per-profile overviews,
/// the claim cards, the control-proof rows, the exception / waiver rows, the exported evaluation
/// packet, the controlled vocabulary, a summary, and a conformance review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AssuranceCenter {
    /// Record kind; must equal [`M5_ASSURANCE_CENTER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ASSURANCE_CENTER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The per-profile overviews, in profile order.
    pub overviews: Vec<AssuranceOverview>,
    /// The claim cards, in subject order.
    pub claim_cards: Vec<ClaimCard>,
    /// The control-proof rows, in control order.
    pub control_proof_rows: Vec<ControlProofRow>,
    /// The exception / waiver rows, in control order.
    pub exception_waiver_rows: Vec<ExceptionWaiverRow>,
    /// The exported evaluation packet.
    pub evaluation_packet: EvaluationPacket,
    /// Controlled-vocabulary set.
    pub vocabulary: AssuranceCenterVocabulary,
    /// Compact summary.
    pub summary: AssuranceCenterSummary,
    /// Conformance review block.
    pub conformance: AssuranceCenterConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AssuranceCenter {
    /// Builds an assurance-center packet from seed input, deriving the claim cards from the control
    /// proofs, the overviews from the cards, and the summary / conformance / evaluation packet from
    /// all of them.
    pub fn new(input: M5AssuranceCenterInput) -> Self {
        // Control-proof rows, in canonical order.
        let mut control_proof_rows: Vec<ControlProofRow> = input
            .control_states
            .iter()
            .map(|(control, state, freshness)| ControlProofRow::new(*control, *state, *freshness))
            .collect();
        control_proof_rows.sort_by_key(|r| control_rank(r.control));
        control_proof_rows.dedup_by_key(|r| r.control);

        // Exception rows, in canonical order.
        let mut exception_waiver_rows: Vec<ExceptionWaiverRow> = input
            .exceptions
            .iter()
            .map(ExceptionWaiverRow::from_seed)
            .collect();
        exception_waiver_rows.sort_by_key(|r| control_rank(r.control));

        // Claim cards derived from the control proofs.
        let claim_cards: Vec<ClaimCard> = AssuranceClaimSubject::ALL
            .iter()
            .map(|s| ClaimCard::derive(*s, &control_proof_rows))
            .collect();

        // Per-profile overviews derived from the cards.
        let overviews: Vec<AssuranceOverview> = ClaimedPosture::ALL
            .iter()
            .map(|p| {
                AssuranceOverview::derive(
                    *p,
                    &claim_cards,
                    &control_proof_rows,
                    &exception_waiver_rows,
                )
            })
            .collect();

        let evaluation_packet = EvaluationPacket::derive(
            &format!("{}:eval", input.packet_id),
            &input.packet_id,
            &input.evaluated_at,
            &input.redaction_class_token,
            &claim_cards,
            &control_proof_rows,
            &exception_waiver_rows,
        );

        let summary = derive_summary(&claim_cards, &control_proof_rows, &exception_waiver_rows);
        let conformance = derive_conformance(
            &claim_cards,
            &control_proof_rows,
            &exception_waiver_rows,
            &overviews,
            &evaluation_packet,
        );

        Self {
            record_kind: M5_ASSURANCE_CENTER_RECORD_KIND.to_owned(),
            schema_version: M5_ASSURANCE_CENTER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            overviews,
            claim_cards,
            control_proof_rows,
            exception_waiver_rows,
            evaluation_packet,
            vocabulary: AssuranceCenterVocabulary::canonical(),
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

    /// Finds a claim card by subject.
    pub fn claim(&self, subject: AssuranceClaimSubject) -> Option<&ClaimCard> {
        self.claim_cards.iter().find(|c| c.subject == subject)
    }

    /// Finds a control-proof row by control.
    pub fn control(&self, control: ControlId) -> Option<&ControlProofRow> {
        self.control_proof_rows
            .iter()
            .find(|r| r.control == control)
    }

    /// Finds a per-profile overview by profile.
    pub fn overview(&self, profile: ClaimedPosture) -> Option<&AssuranceOverview> {
        self.overviews.iter().find(|o| o.profile == profile)
    }

    /// Renders the packet for a generation channel. The output is identical for every channel — the
    /// channel parameter exists only to prove desktop, CLI / headless, and offline / mirror
    /// generation produce byte-identical output.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn render_for_channel(&self, _channel: AssuranceCenterChannel) -> String {
        self.export_safe_json()
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 assurance center serializes")
    }

    /// The exported evaluation packet's JSON — the same claim-state and proof vocabulary the UI
    /// shows, reduced to refs.
    pub fn render_evaluation_packet(&self) -> String {
        self.evaluation_packet.export_safe_json()
    }

    /// Deterministic, machine-readable claim / control matrix CSV: one row per (claim, required
    /// control) join, naming the claim, its active state, the control, its proof state and freshness,
    /// the proof ref, and any gap.
    pub fn render_claims_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "claim,claimed_posture,active_state,effective_qualification,claim_owner,fallback_posture,fallback_state,control,proof_state,evidence_class,evidence_freshness,control_owner,proof_ref,gap_kind\n",
        );
        for card in &self.claim_cards {
            let fallback_posture = card
                .fallback
                .as_ref()
                .map(|f| f.fallback_posture.as_str())
                .unwrap_or("");
            let fallback_state = card
                .fallback
                .as_ref()
                .map(|f| f.fallback_state.as_str())
                .unwrap_or("");
            for control in &card.required_controls {
                let row = self.control(*control);
                let gap = card
                    .gaps
                    .iter()
                    .find(|g| g.control == *control)
                    .map(|g| g.gap_kind.as_str())
                    .unwrap_or("");
                let (proof_state, evidence_class, freshness, control_owner, proof_ref) = match row {
                    Some(r) => (
                        r.proof_state.as_str().to_owned(),
                        r.evidence_class.as_str().to_owned(),
                        r.evidence_freshness.as_str().to_owned(),
                        r.owner_role.clone(),
                        r.proof_ref.clone(),
                    ),
                    None => (
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        control.proof_ref().to_owned(),
                    ),
                };
                out.push_str(&format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                    card.subject.as_str(),
                    card.claimed_posture.as_str(),
                    card.active_state.as_str(),
                    card.effective_qualification.as_str(),
                    card.owner_role,
                    fallback_posture,
                    fallback_state,
                    control.as_str(),
                    proof_state,
                    evidence_class,
                    freshness,
                    control_owner,
                    proof_ref,
                    gap,
                ));
            }
        }
        out
    }

    /// Deterministic assurance-center overview document for review, support, docs, or evaluator
    /// handoff.
    pub fn render_overview_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Assurance Center\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Claims: {} ({} proven, {} attested, {} narrowed, {} blocked)\n",
            self.summary.total_claims,
            self.summary.proven_claims,
            self.summary.attested_claims,
            self.summary.narrowed_claims,
            self.summary.blocked_claims
        ));
        out.push_str(&format!(
            "- Controls: {} ({} governed, {} narrowed, {} blocked)\n",
            self.summary.total_controls,
            self.summary.governed_controls,
            self.summary.narrowed_controls,
            self.summary.blocked_controls
        ));
        out.push_str(&format!(
            "- Open exceptions: {}\n",
            self.summary.total_exceptions
        ));
        out.push_str(&format!(
            "- Stable promotion: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));

        out.push_str("\n## Deployment-profile overviews\n\n");
        out.push_str(
            "| Profile | Effective posture | Gate | Qualification | Proven | Attested | Under review | Exception | Unproven | Exceptions |\n",
        );
        out.push_str(
            "|---------|-------------------|------|---------------|--------|----------|--------------|-----------|----------|------------|\n",
        );
        for o in &self.overviews {
            let c = &o.claim_state_counts;
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} | {} | {} | {} | {} | {} |\n",
                o.profile.as_str(),
                o.effective_posture.as_str(),
                o.gate_decision.as_str(),
                o.effective_qualification.as_str(),
                c.proven,
                c.attested,
                c.under_review,
                c.exception_pending,
                c.unproven,
                o.known_exception_count
            ));
        }

        out.push_str("\n## Claim cards\n\n");
        out.push_str(
            "| Claim | Claimed posture | Active state | Qualification | Owner | Fallback |\n",
        );
        out.push_str(
            "|-------|-----------------|--------------|---------------|-------|----------|\n",
        );
        for card in &self.claim_cards {
            let fallback = card
                .fallback
                .as_ref()
                .map(|f| {
                    format!(
                        "`{}` / `{}`",
                        f.fallback_posture.as_str(),
                        f.fallback_state.as_str()
                    )
                })
                .unwrap_or_else(|| "—".to_owned());
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                card.subject.as_str(),
                card.claimed_posture.as_str(),
                card.active_state.as_str(),
                card.effective_qualification.as_str(),
                card.owner_role,
                fallback
            ));
        }

        out.push_str("\n## Control proof\n\n");
        out.push_str("| Control | Backs | Proof state | Evidence class | Freshness | Gate | Owner | Proof |\n");
        out.push_str("|---------|-------|-------------|----------------|-----------|------|-------|-------|\n");
        for row in &self.control_proof_rows {
            out.push_str(&format!(
                "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                row.control.as_str(),
                join_tokens(&row.backs_claims, |c| c.as_str()),
                row.proof_state.as_str(),
                row.evidence_class.as_str(),
                row.evidence_freshness.as_str(),
                row.effective_gate.as_str(),
                row.owner_role,
                row.proof_ref
            ));
        }

        if !self.exception_waiver_rows.is_empty() {
            out.push_str("\n## Exceptions / waivers\n\n");
            out.push_str(
                "| Control | State | Expiry | Compensating control | Party | Action | Affects |\n",
            );
            out.push_str(
                "|---------|-------|--------|----------------------|-------|--------|---------|\n",
            );
            for row in &self.exception_waiver_rows {
                out.push_str(&format!(
                    "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                    row.control.as_str(),
                    row.governance_state.as_str(),
                    row.expiry,
                    row.compensating_control.as_str(),
                    row.responsible_party.as_str(),
                    row.action.as_str(),
                    join_tokens(&row.affected_claims, |c| c.as_str())
                ));
            }
        }
        out
    }

    /// Compact Markdown report for the release-grade parity proof.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Assurance Center — Proof\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Claims: {} ({} proven, {} attested, {} narrowed, {} blocked)\n",
            self.summary.total_claims,
            self.summary.proven_claims,
            self.summary.attested_claims,
            self.summary.narrowed_claims,
            self.summary.blocked_claims
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
            "- Evaluation packet: `{}`\n",
            M5_ASSURANCE_CENTER_EVALUATION_PACKET_REF
        ));
        out.push_str(&format!(
            "- Claims CSV: `{}`\n",
            M5_ASSURANCE_CENTER_CLAIMS_CSV_REF
        ));
        out
    }

    /// Validates the packet's invariants.
    pub fn validate(&self) -> Vec<M5AssuranceCenterViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_ASSURANCE_CENTER_RECORD_KIND {
            out.push(M5AssuranceCenterViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ASSURANCE_CENTER_SCHEMA_VERSION {
            out.push(M5AssuranceCenterViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5AssuranceCenterViolation::MissingIdentity);
        }

        // Every control governed exactly once and self-consistent.
        let mut seen_controls = std::collections::BTreeSet::new();
        for row in &self.control_proof_rows {
            if !seen_controls.insert(row.control) {
                out.push(M5AssuranceCenterViolation::DuplicateControl);
            }
            out.extend(row.validate());
        }
        for control in ControlId::ALL {
            if !self.control_proof_rows.iter().any(|r| r.control == control) {
                out.push(M5AssuranceCenterViolation::ControlNotGoverned);
            }
        }

        // Every claim governed exactly once and self-consistent.
        let mut seen_claims = std::collections::BTreeSet::new();
        for card in &self.claim_cards {
            if !seen_claims.insert(card.subject) {
                out.push(M5AssuranceCenterViolation::DuplicateClaim);
            }
            out.extend(card.validate(&self.control_proof_rows));
        }
        for subject in AssuranceClaimSubject::ALL {
            if !self.claim_cards.iter().any(|c| c.subject == subject) {
                out.push(M5AssuranceCenterViolation::ClaimNotGoverned);
            }
        }

        // Every profile has an overview.
        for o in &self.overviews {
            out.extend(o.validate(
                &self.claim_cards,
                &self.control_proof_rows,
                &self.exception_waiver_rows,
            ));
        }
        for profile in ClaimedPosture::ALL {
            if !self.overviews.iter().any(|o| o.profile == profile) {
                out.push(M5AssuranceCenterViolation::ProfileNotCovered);
            }
        }

        for row in &self.exception_waiver_rows {
            out.extend(row.validate());
            // An exception must correspond to a control held in an exception_pending proof state.
            if let Some(control_row) = self.control(row.control) {
                if !matches!(
                    control_row.proof_state,
                    AssuranceClaimState::ExceptionPending
                ) {
                    out.push(M5AssuranceCenterViolation::ExceptionWithoutWaivedControl);
                }
            } else {
                out.push(M5AssuranceCenterViolation::ExceptionWithoutWaivedControl);
            }
        }

        let expected_eval = EvaluationPacket::derive(
            &self.evaluation_packet.packet_id,
            &self.packet_id,
            &self.evaluated_at,
            &self.redaction_class_token,
            &self.claim_cards,
            &self.control_proof_rows,
            &self.exception_waiver_rows,
        );
        if self.evaluation_packet != expected_eval
            || !self.evaluation_packet.reuses_canonical_vocabulary()
        {
            out.push(M5AssuranceCenterViolation::EvaluationPacketDrift);
        }

        if !self.vocabulary.matches_canonical() {
            out.push(M5AssuranceCenterViolation::VocabularyMismatch);
        }
        if self.summary
            != derive_summary(
                &self.claim_cards,
                &self.control_proof_rows,
                &self.exception_waiver_rows,
            )
        {
            out.push(M5AssuranceCenterViolation::SummaryDrift);
        }
        if self.conformance
            != derive_conformance(
                &self.claim_cards,
                &self.control_proof_rows,
                &self.exception_waiver_rows,
                &self.overviews,
                &self.evaluation_packet,
            )
            || !self.conformance.all_hold()
        {
            out.push(M5AssuranceCenterViolation::ConformanceReviewFailed);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 assurance center serializes"),
        ) {
            out.push(M5AssuranceCenterViolation::RawMaterialInExport);
        }
        out
    }
}

/// The generation channel an assurance-center packet is produced on. Every channel produces
/// byte-identical output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceCenterChannel {
    /// The desktop product UI.
    DesktopUi,
    /// The CLI / headless structured-output path.
    CliHeadless,
    /// Offline / mirror-safe packet generation.
    OfflineMirror,
}

impl AssuranceCenterChannel {
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

/// Derives the summary from the cards, controls, and exceptions.
fn derive_summary(
    cards: &[ClaimCard],
    controls: &[ControlProofRow],
    exceptions: &[ExceptionWaiverRow],
) -> AssuranceCenterSummary {
    let claim_state_count = |state: AssuranceClaimState| -> u32 {
        cards.iter().filter(|c| c.active_state == state).count() as u32
    };
    let control_gate_count = |gate: DescriptorGate| -> u32 {
        controls.iter().filter(|r| r.effective_gate == gate).count() as u32
    };
    let blocked_claims = cards.iter().filter(|c| c.is_blocked()).count() as u32;
    let honored = ClaimedPosture::ALL
        .iter()
        .filter(|p| strongest_honored_posture(**p, cards) == **p)
        .count() as u32;
    AssuranceCenterSummary {
        total_claims: cards.len() as u32,
        proven_claims: claim_state_count(AssuranceClaimState::Proven),
        attested_claims: claim_state_count(AssuranceClaimState::Attested),
        narrowed_claims: cards.iter().filter(|c| c.is_narrowed()).count() as u32,
        blocked_claims,
        total_controls: controls.len() as u32,
        governed_controls: control_gate_count(DescriptorGate::Governed),
        narrowed_controls: control_gate_count(DescriptorGate::Narrowed),
        blocked_controls: control_gate_count(DescriptorGate::Blocked),
        total_exceptions: exceptions.len() as u32,
        total_profiles: ClaimedPosture::ALL.len() as u32,
        honored_profiles: honored,
        blocks_stable_promotion: blocked_claims > 0,
    }
}

/// Derives the conformance review, so the stored block reflects the actual packet.
fn derive_conformance(
    cards: &[ClaimCard],
    controls: &[ControlProofRow],
    exceptions: &[ExceptionWaiverRow],
    overviews: &[AssuranceOverview],
    evaluation_packet: &EvaluationPacket,
) -> AssuranceCenterConformance {
    let derived = cards
        .iter()
        .all(|c| ClaimCard::derive(c.subject, controls) == *c);

    let no_overstate = cards.iter().all(|c| {
        let worst = c
            .required_controls
            .iter()
            .filter_map(|id| controls.iter().find(|r| r.control == *id))
            .map(|r| r.effective_gate)
            .fold(DescriptorGate::Governed, worse_gate);
        c.active_gate == worst
    });

    let bound = ControlId::ALL.iter().all(|id| {
        controls
            .iter()
            .filter(|r| r.control == *id)
            .filter(|r| !r.proof_ref.trim().is_empty() && !r.owner_role.trim().is_empty())
            .count()
            == 1
    });

    // A control that only narrows (stale freshness or a narrowing proof state, but not blocking)
    // narrows every claim that requires it, unless a blocking control already blocks that claim.
    let stale_narrows = cards.iter().all(|c| {
        let reads_narrowing = c.required_controls.iter().any(|id| {
            controls
                .iter()
                .find(|r| r.control == *id)
                .is_some_and(|r| matches!(r.effective_gate, DescriptorGate::Narrowed))
        });
        let reads_blocking = c.required_controls.iter().any(|id| {
            controls
                .iter()
                .find(|r| r.control == *id)
                .map_or(true, |r| matches!(r.effective_gate, DescriptorGate::Blocked))
        });
        !reads_narrowing || reads_blocking || c.is_narrowed()
    });

    let missing_blocks = cards.iter().all(|c| {
        let reads_blocking = c.required_controls.iter().any(|id| {
            controls
                .iter()
                .find(|r| r.control == *id)
                .map_or(true, |r| matches!(r.effective_gate, DescriptorGate::Blocked))
        });
        !reads_blocking || c.is_blocked()
    });

    let exceptions_disclose = exceptions.iter().all(|e| {
        !e.mitigation.trim().is_empty()
            && !e.expiry.trim().is_empty()
            && e.compensating_control != e.control
    });

    let fallback_present = cards
        .iter()
        .all(|c| c.is_governed() == c.fallback.is_none());

    let overview_ok = overviews
        .iter()
        .all(|o| posture_rank(o.effective_posture) <= posture_rank(o.profile));

    let export_clean =
        !json_contains_forbidden_material(&serde_json::to_value(cards).expect("cards serialize"))
            && !json_contains_forbidden_material(
                &serde_json::to_value(controls).expect("controls serialize"),
            )
            && !json_contains_forbidden_material(
                &serde_json::to_value(exceptions).expect("exceptions serialize"),
            );

    AssuranceCenterConformance {
        claim_state_derived_from_controls: derived,
        no_claim_overstates_controls: no_overstate,
        every_control_bound_to_evidence_and_owner: bound,
        stale_evidence_narrows_deterministically: stale_narrows,
        missing_evidence_blocks_stable_promotion: missing_blocks,
        exceptions_disclose_mitigation_expiry_and_action: exceptions_disclose,
        fallback_present_when_not_governed: fallback_present,
        overview_effective_posture_never_overstated: overview_ok,
        evaluation_packet_reuses_ui_vocabulary: evaluation_packet.reuses_canonical_vocabulary(),
        controlled_enums_frozen: AssuranceCenterVocabulary::canonical().matches_canonical(),
        export_carries_no_raw_material: export_clean,
    }
}

// ---------------------------------------------------------------------------------------------
// Ranking / token helpers
// ---------------------------------------------------------------------------------------------

/// Position of a control in the canonical ordering.
fn control_rank(control: ControlId) -> usize {
    ControlId::ALL
        .iter()
        .position(|c| *c == control)
        .unwrap_or(ControlId::ALL.len())
}

/// Position of an evidence class in the canonical ordering.
fn evidence_class_rank(class: EvidenceClass) -> usize {
    EvidenceClass::ALL
        .iter()
        .position(|c| *c == class)
        .unwrap_or(EvidenceClass::ALL.len())
}

/// Maps a typed `ALL` array to its stable tokens.
fn tokens<T: Copy>(all: &[T], f: impl Fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|t| f(*t).to_owned()).collect()
}

/// Joins a token list for table / CSV rendering, comma-space separated.
fn join_tokens<T: Copy>(items: &[T], f: impl Fn(T) -> &'static str) -> String {
    items
        .iter()
        .map(|t| f(*t))
        .collect::<Vec<_>>()
        .join(if items.len() > 8 { "," } else { " " })
}

// ---------------------------------------------------------------------------------------------
// Violations
// ---------------------------------------------------------------------------------------------

/// Validation failures for the assurance-center lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AssuranceCenterViolation {
    /// The packet record kind is wrong.
    WrongRecordKind,
    /// The packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// A control row cites a field that does not match its control.
    ControlFieldMismatch,
    /// A control row's effective gate drifted from its proof state and freshness.
    ControlGateDrift,
    /// A control row binds no evidence proof ref.
    ControlEvidenceMissing,
    /// Two control rows name the same control.
    DuplicateControl,
    /// A control has no governed row.
    ControlNotGoverned,
    /// A claim card drifted from a fresh derivation of its controls.
    ClaimCardDrift,
    /// A claim card cites a field that does not match its subject.
    ClaimFieldMismatch,
    /// A claim card's gate or qualification drifted.
    ClaimGateDrift,
    /// A claim card reads stronger than its controls allow.
    ClaimOverstatesControls,
    /// A claim's fallback overstates its posture or state.
    FallbackOverstates,
    /// A claim's fallback is present when governed or absent when not.
    FallbackPresenceInvalid,
    /// Two claim cards name the same subject.
    DuplicateClaim,
    /// A claim has no card.
    ClaimNotGoverned,
    /// An overview drifted from a fresh derivation.
    OverviewDrift,
    /// An overview reads a posture above its profile.
    OverviewOverstatesPosture,
    /// An overview's gate or qualification drifted.
    OverviewGateDrift,
    /// A profile has no overview.
    ProfileNotCovered,
    /// An exception row cites a field that does not match its control.
    ExceptionFieldMismatch,
    /// An exception row omits mitigation, expiry, or a distinct compensating control.
    ExceptionDisclosureIncomplete,
    /// An exception row's status drifted.
    ExceptionStatusDrift,
    /// An exception row has no control held in an exception-pending proof state.
    ExceptionWithoutWaivedControl,
    /// The evaluation packet drifted from the cards / controls / exceptions or its vocabulary.
    EvaluationPacketDrift,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// The summary disagrees with the cards / controls / exceptions.
    SummaryDrift,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The export contains raw provider material.
    RawMaterialInExport,
}

impl M5AssuranceCenterViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::ControlFieldMismatch => "control_field_mismatch",
            Self::ControlGateDrift => "control_gate_drift",
            Self::ControlEvidenceMissing => "control_evidence_missing",
            Self::DuplicateControl => "duplicate_control",
            Self::ControlNotGoverned => "control_not_governed",
            Self::ClaimCardDrift => "claim_card_drift",
            Self::ClaimFieldMismatch => "claim_field_mismatch",
            Self::ClaimGateDrift => "claim_gate_drift",
            Self::ClaimOverstatesControls => "claim_overstates_controls",
            Self::FallbackOverstates => "fallback_overstates",
            Self::FallbackPresenceInvalid => "fallback_presence_invalid",
            Self::DuplicateClaim => "duplicate_claim",
            Self::ClaimNotGoverned => "claim_not_governed",
            Self::OverviewDrift => "overview_drift",
            Self::OverviewOverstatesPosture => "overview_overstates_posture",
            Self::OverviewGateDrift => "overview_gate_drift",
            Self::ProfileNotCovered => "profile_not_covered",
            Self::ExceptionFieldMismatch => "exception_field_mismatch",
            Self::ExceptionDisclosureIncomplete => "exception_disclosure_incomplete",
            Self::ExceptionStatusDrift => "exception_status_drift",
            Self::ExceptionWithoutWaivedControl => "exception_without_waived_control",
            Self::EvaluationPacketDrift => "evaluation_packet_drift",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::SummaryDrift => "summary_drift",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked raw material. Mirrors the redaction posture of
/// the upstream descriptor / governance lanes.
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
