//! Per-profile certification of the M5 assurance / governance / boundary-route / event-provenance
//! contract.
//!
//! The [assurance / governance / route-provenance governance
//! matrix](crate::m5_assurance_route_governance) freezes the nine governed facets — assurance claim,
//! control proof, exception / waiver, governance freshness, service ownership, capability boundary,
//! route hop, approval ticket, and event provenance — and certifies the *product surfaces* that read
//! them. It does not, on its own, say whether a **claimed M5 deployment profile** — `managed`,
//! `self_hosted`, `regulated`, or `sovereign` — actually maps to fresh proof for assurance,
//! governance, boundary-route, and event-provenance explainability. This lane closes that gap: it
//! projects the governance matrix onto the claimed profile grid and qualifies each profile,
//! narrowing or blocking the profile's Stable claim deterministically when the backing proof is
//! stale, drifting, or missing rather than letting a regulated or self-hosted profile keep a generic
//! trust badge behind drifted evidence.
//!
//! The certification is a *pure function of the governance matrix* — it carries no parallel,
//! hand-maintained inventory. Each profile is qualified along four [proof dimensions]
//! ([CertificationDimension]) that group the governed facets the source set treats as that part of
//! the assurance story:
//!
//! - **assurance center** — the assurance claim, control proof, and exception / waiver facets;
//! - **governance** — the governance-freshness and service-ownership facets;
//! - **boundary / route** — the capability-boundary, route-hop, and approval-ticket facets;
//! - **event provenance** — the event-provenance facet.
//!
//! For one claimed profile and one dimension, the certification gathers the governed facets that back
//! that dimension *and* scope to that profile. A dimension no facet covers for the profile is
//! [not applicable](CertificationOutcome::NotApplicable) (honestly labeled, never a hidden gap);
//! otherwise the cell takes the **worst** proof freshness and the **worst** assurance-state gate
//! among the covering facets, so a narrowed control or stale proof can never read as fully proven. A
//! profile's gate is the worst of its applicable cells, and its effective qualification is the
//! claimed class narrowed down that gate — `governed` keeps the Stable claim, `narrowed` floors it at
//! Beta, `blocked` floors it at Unavailable. The cell reuses the governance matrix's frozen
//! [gap-kind vocabulary](crate::m5_assurance_route_governance::AssuranceGapKind) so a profile that
//! narrowed names *why* — stale evidence, a narrowed control, a missing provenance ledger — rather
//! than hiding behind a generic stable badge.
//!
//! The claimed release surfaces ([CertificationConsumer]) — release center, About / help, shiproom,
//! support export, and the procurement / evaluation pack — each bind the dimensions they surface and
//! *derive* their posture and the exact profile claims they must narrow or block from the grid, so
//! release, help, support, shiproom, and evaluation packets read one certification result instead of
//! maintaining local trust overrides.
//!
//! The [`M5AssuranceCertification`] packet is the one inspectable, serde-serializable certification
//! truth those surfaces consume; it carries metadata and refs only — no credential bodies or raw
//! provider payloads, so a refs-only export preserves owner / freshness / route lineage without
//! leaking secrets.
//!
//! - Packet schema:
//!   [`schemas/public-truth/m5-assurance-certification.schema.json`](../../../../../schemas/public-truth/m5-assurance-certification.schema.json)
//! - Contract doc:
//!   [`docs/release/m5-assurance-certification-contract.md`](../../../../../docs/release/m5-assurance-certification-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_assurance_certification, seeded_m5_assurance_certification_missing_proof_blocked,
    seeded_m5_assurance_certification_stale_proof_narrowed, M5_ASSURANCE_CERTIFICATION_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The certification reuses the governance matrix's frozen facet / posture / gap vocabulary and the
// descriptor / badge gate vocabulary, so the certification layer can never drift to a different
// facet set, profile set, or gate token than the governance it projects.
use crate::m5_assurance_route_governance::{
    AssuranceFacet, AssuranceFacetRow, AssuranceGapKind, ClaimedPosture,
    M5AssuranceRouteGovernance, M5_ASSURANCE_ROUTE_REF,
};
use crate::m5_descriptor_badge::{
    ConsumerStatus, DescriptorGate, DescriptorSignal, FreshnessState, QualificationClass,
};

/// Record-kind tag carried by [`M5AssuranceCertification`].
pub const M5_ASSURANCE_CERTIFICATION_RECORD_KIND: &str = "m5_assurance_certification";

/// Schema version for the certification packet.
pub const M5_ASSURANCE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the certification packet schema.
pub const M5_ASSURANCE_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/public-truth/m5-assurance-certification.schema.json";

/// Repo-relative path of the published certification inventory.
pub const M5_ASSURANCE_CERTIFICATION_REGISTRY_REF: &str =
    "artifacts/public-truth/m5-assurance-certification.json";

/// Repo-relative path of the rendered certification document.
pub const M5_ASSURANCE_CERTIFICATION_DOCUMENT_REF: &str =
    "artifacts/public-truth/m5-assurance-certification.md";

/// Repo-relative path of the machine-readable certification grid export.
pub const M5_ASSURANCE_CERTIFICATION_CSV_REF: &str =
    "artifacts/public-truth/m5-assurance-certification-grid.csv";

/// Repo-relative path of the release-grade certification parity proof.
pub const M5_ASSURANCE_CERTIFICATION_PROOF_REF: &str =
    "artifacts/release/m5-assurance-certification-proof/certification.json";

/// Repo-relative path of the release-grade certification proof report.
pub const M5_ASSURANCE_CERTIFICATION_PROOF_MD_REF: &str =
    "artifacts/release/m5-assurance-certification-proof/certification.md";

/// Repo-relative path of the certification contract doc.
pub const M5_ASSURANCE_CERTIFICATION_DOC_REF: &str =
    "docs/release/m5-assurance-certification-contract.md";

/// Repo-relative directory of the per-state certification fixtures.
pub const M5_ASSURANCE_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/public-truth/m5-assurance-certification/";

/// Prefix every certification message id carries so consumers can route it.
pub const M5_ASSURANCE_CERTIFICATION_MESSAGE_ID_PREFIX: &str =
    "public_truth.assurance_certification.";

/// One proof dimension a claimed profile is certified along. Each dimension groups the governed
/// [facets](AssuranceFacet) the source set treats as that part of the assurance / governance /
/// route-provenance contract, so the certification reuses the matrix's facet proofs rather than
/// restating them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDimension {
    /// The assurance center: what a profile claims and the controls / waivers behind it.
    AssuranceCenter,
    /// The governance / fitness posture: how current and owned the governing evidence is.
    Governance,
    /// The capability boundary and high-risk route: where work ran, how it routed, who approved it.
    BoundaryRoute,
    /// Event provenance: the lineage an emitted event can be traced to.
    EventProvenance,
}

impl CertificationDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AssuranceCenter,
        Self::Governance,
        Self::BoundaryRoute,
        Self::EventProvenance,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssuranceCenter => "assurance_center",
            Self::Governance => "governance",
            Self::BoundaryRoute => "boundary_route",
            Self::EventProvenance => "event_provenance",
        }
    }

    /// Reviewer-facing dimension label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AssuranceCenter => "Assurance center",
            Self::Governance => "Governance",
            Self::BoundaryRoute => "Boundary / route",
            Self::EventProvenance => "Event provenance",
        }
    }

    /// The governed facets that back this dimension's proof.
    pub const fn backing_facets(self) -> &'static [AssuranceFacet] {
        match self {
            Self::AssuranceCenter => &[
                AssuranceFacet::AssuranceClaim,
                AssuranceFacet::ControlProof,
                AssuranceFacet::ExceptionWaiver,
            ],
            Self::Governance => &[
                AssuranceFacet::GovernanceFreshness,
                AssuranceFacet::ServiceOwnership,
            ],
            Self::BoundaryRoute => &[
                AssuranceFacet::CapabilityBoundary,
                AssuranceFacet::RouteHop,
                AssuranceFacet::ApprovalTicket,
            ],
            Self::EventProvenance => &[AssuranceFacet::EventProvenance],
        }
    }

    /// Owner role accountable for keeping this dimension's proof current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::AssuranceCenter => "assurance_center_owner",
            Self::Governance => "governance_dashboard_owner",
            Self::BoundaryRoute => "capability_boundary_owner",
            Self::EventProvenance => "event_provenance_owner",
        }
    }
}

/// The outcome a profile earned on one [dimension](CertificationDimension): fully certified,
/// narrowed, blocked, or — when no governed facet covers the profile — honestly labeled
/// not-applicable rather than a hidden gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationOutcome {
    /// Every backing facet maps to a current proof and a governed assurance state.
    Certified,
    /// At least one backing facet's proof is stale or its assurance state narrows.
    Narrowed,
    /// At least one backing facet's proof is expired / missing or its assurance state blocks.
    Blocked,
    /// No governed facet covers this profile for this dimension; the dimension does not apply.
    NotApplicable,
}

impl CertificationOutcome {
    /// Every outcome, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Certified,
        Self::Narrowed,
        Self::Blocked,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Narrowed => "narrowed",
            Self::Blocked => "blocked",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// The outcome implied by an applicable cell's gate.
    const fn for_gate(gate: DescriptorGate) -> Self {
        match gate {
            DescriptorGate::Governed => Self::Certified,
            DescriptorGate::Narrowed => Self::Narrowed,
            DescriptorGate::Blocked => Self::Blocked,
        }
    }
}

/// The proof-currency gap kind a freshness implies, if any. Reuses the governance matrix's frozen
/// [gap vocabulary](AssuranceGapKind).
const fn freshness_gap(freshness: FreshnessState) -> Option<AssuranceGapKind> {
    match freshness {
        FreshnessState::Current => None,
        FreshnessState::Stale => Some(AssuranceGapKind::ProofStale),
        FreshnessState::Expired => Some(AssuranceGapKind::ProofExpired),
        FreshnessState::Missing => Some(AssuranceGapKind::ProofMissing),
    }
}

/// The assurance-state gap kind a state gate implies, if any.
const fn state_gap(gate: DescriptorGate) -> Option<AssuranceGapKind> {
    match gate {
        DescriptorGate::Governed => None,
        DescriptorGate::Narrowed => Some(AssuranceGapKind::AssuranceStateNarrowed),
        DescriptorGate::Blocked => Some(AssuranceGapKind::AssuranceStateBlocked),
    }
}

/// The gate a proof freshness implies on its own.
const fn freshness_gate(freshness: FreshnessState) -> DescriptorGate {
    match freshness {
        FreshnessState::Current => DescriptorGate::Governed,
        FreshnessState::Stale => DescriptorGate::Narrowed,
        FreshnessState::Expired | FreshnessState::Missing => DescriptorGate::Blocked,
    }
}

/// The more severe of two gates (`Blocked` > `Narrowed` > `Governed`).
const fn worst_gate(a: DescriptorGate, b: DescriptorGate) -> DescriptorGate {
    match (a, b) {
        (DescriptorGate::Blocked, _) | (_, DescriptorGate::Blocked) => DescriptorGate::Blocked,
        (DescriptorGate::Narrowed, _) | (_, DescriptorGate::Narrowed) => DescriptorGate::Narrowed,
        _ => DescriptorGate::Governed,
    }
}

/// The coverage status a gate implies.
const fn status_for_gate(gate: DescriptorGate) -> ConsumerStatus {
    match gate {
        DescriptorGate::Governed => ConsumerStatus::Mapped,
        DescriptorGate::Narrowed => ConsumerStatus::Provisional,
        DescriptorGate::Blocked => ConsumerStatus::Unmapped,
    }
}

/// The claimed qualification narrowed down its earned gate: `governed` keeps the claim, `narrowed`
/// floors it at Beta, `blocked` floors it at Unavailable.
fn narrow_qualification(claimed: QualificationClass, gate: DescriptorGate) -> QualificationClass {
    match gate {
        DescriptorGate::Governed => claimed,
        // QualificationClass declaration order is most→least permissive, so `max` is the more
        // restrictive of the two.
        DescriptorGate::Narrowed => claimed.max(QualificationClass::Beta),
        DescriptorGate::Blocked => QualificationClass::Unavailable,
    }
}

/// One certification cell: a claimed profile certified along one [dimension](CertificationDimension).
/// The cell derives its outcome from the governed facets that back the dimension and scope to the
/// profile, so it can never cite a posture stronger than its weakest backing proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationCell {
    /// The dimension this cell certifies.
    pub dimension: CertificationDimension,
    /// Reviewer-facing dimension label.
    pub dimension_label: String,
    /// The outcome earned.
    pub outcome: CertificationOutcome,
    /// The governed facets that back this cell (scoped to the profile).
    pub backing_facets: Vec<AssuranceFacet>,
    /// The proof paths backing the cell — refs only.
    pub proof_refs: Vec<String>,
    /// Worst proof freshness among the backing facets; absent when not applicable.
    pub proof_freshness: Option<FreshnessState>,
    /// Worst assurance-state gate among the backing facets; absent when not applicable.
    pub state_gate: Option<DescriptorGate>,
    /// Current state token of the backing facet that drove the worst state gate; absent when not
    /// applicable. Naming it is what records which precondition drifted.
    pub worst_state_token: Option<String>,
    /// Gate the cell contributes to the profile. Not-applicable cells are governed and excluded.
    pub gate: DescriptorGate,
    /// Coverage status (mirrors [`Self::gate`]).
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Why the cell narrowed or blocked, if it did.
    pub gap_kind: Option<AssuranceGapKind>,
    /// Stable message id; prefixed [`M5_ASSURANCE_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl CertificationCell {
    /// Derives a cell for a (`profile`, `dimension`) tuple from the governed facet rows.
    fn derive(
        dimension: CertificationDimension,
        profile: ClaimedPosture,
        facets: &[AssuranceFacetRow],
    ) -> Self {
        // Gather the backing facets that scope to this profile.
        let mut backing: Vec<&AssuranceFacetRow> = Vec::new();
        for facet in dimension.backing_facets() {
            if let Some(row) = facets.iter().find(|r| r.facet == *facet) {
                if row.claimed_postures.contains(&profile) {
                    backing.push(row);
                }
            }
        }

        let detail_message_id = format!(
            "{}cell.{}.{}",
            M5_ASSURANCE_CERTIFICATION_MESSAGE_ID_PREFIX,
            profile.as_str(),
            dimension.as_str()
        );

        if backing.is_empty() {
            // No governed facet covers this profile for this dimension: honestly not applicable.
            return Self {
                dimension,
                dimension_label: dimension.label().to_owned(),
                outcome: CertificationOutcome::NotApplicable,
                backing_facets: Vec::new(),
                proof_refs: Vec::new(),
                proof_freshness: None,
                state_gate: None,
                worst_state_token: None,
                gate: DescriptorGate::Governed,
                status: ConsumerStatus::Mapped,
                signal: ConsumerStatus::Mapped.signal(),
                gap_kind: None,
                detail_message_id,
            };
        }

        // Worst proof freshness and worst assurance-state gate among the backing facets.
        let proof_freshness = backing
            .iter()
            .map(|r| r.proof_freshness)
            .max_by_key(|f| freshness_rank(*f))
            .expect("backing is non-empty");
        let worst_state_row = backing
            .iter()
            .max_by_key(|r| (gate_rank(r.state_gate), facet_rank(r.facet)))
            .expect("backing is non-empty");
        let state_gate = worst_state_row.state_gate;

        let gate = worst_gate(freshness_gate(proof_freshness), state_gate);
        // A proof-currency gap reads ahead of an assurance-state gap when both apply, so the named
        // cause matches the dominant reason the cell could not stand.
        let gap_kind = freshness_gap(proof_freshness).or_else(|| state_gap(state_gate));
        let status = status_for_gate(gate);

        let mut facet_kinds: Vec<AssuranceFacet> = backing.iter().map(|r| r.facet).collect();
        facet_kinds.sort_by_key(|f| facet_rank(*f));
        facet_kinds.dedup();
        let proof_refs: Vec<String> = facet_kinds
            .iter()
            .map(|f| f.proof_ref().to_owned())
            .collect();

        Self {
            dimension,
            dimension_label: dimension.label().to_owned(),
            outcome: CertificationOutcome::for_gate(gate),
            backing_facets: facet_kinds,
            proof_refs,
            proof_freshness: Some(proof_freshness),
            state_gate: Some(state_gate),
            worst_state_token: Some(worst_state_row.current_state_token.clone()),
            gate,
            status,
            signal: status.signal(),
            gap_kind,
            detail_message_id,
        }
    }

    /// True when this cell contributes to its profile's gate (i.e. the dimension applies).
    pub fn is_applicable(&self) -> bool {
        self.outcome != CertificationOutcome::NotApplicable
    }

    /// Re-derives this cell's invariants and reports any drift.
    fn validate(&self) -> Vec<M5AssuranceCertificationViolation> {
        let mut out = Vec::new();
        if self.dimension_label != self.dimension.label() {
            out.push(M5AssuranceCertificationViolation::CellFieldMismatch);
        }
        match self.outcome {
            CertificationOutcome::NotApplicable => {
                if !self.backing_facets.is_empty()
                    || self.proof_freshness.is_some()
                    || self.state_gate.is_some()
                    || self.worst_state_token.is_some()
                    || self.gate != DescriptorGate::Governed
                    || self.gap_kind.is_some()
                {
                    out.push(M5AssuranceCertificationViolation::CellOutcomeDrift);
                }
            }
            _ => {
                let (Some(freshness), Some(state_gate)) = (self.proof_freshness, self.state_gate)
                else {
                    out.push(M5AssuranceCertificationViolation::CellOutcomeDrift);
                    return out;
                };
                let gate = worst_gate(freshness_gate(freshness), state_gate);
                let gap = freshness_gap(freshness).or_else(|| state_gap(state_gate));
                if self.gate != gate
                    || self.outcome != CertificationOutcome::for_gate(gate)
                    || self.status != status_for_gate(gate)
                    || self.signal != status_for_gate(gate).signal()
                    || self.gap_kind != gap
                    || self.backing_facets.is_empty()
                    || self.proof_refs.len() != self.backing_facets.len()
                    || self.worst_state_token.is_none()
                {
                    out.push(M5AssuranceCertificationViolation::CellOutcomeDrift);
                }
            }
        }
        if !self
            .detail_message_id
            .starts_with(M5_ASSURANCE_CERTIFICATION_MESSAGE_ID_PREFIX)
        {
            out.push(M5AssuranceCertificationViolation::UnprefixedMessageId);
        }
        out
    }
}

/// One claimed M5 deployment profile qualified against the assurance / governance / route-provenance
/// contract: the qualification class it wants to keep, a cell per
/// [dimension](CertificationDimension), the gate derived from those cells, and the effective
/// qualification after narrowing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileClaim {
    /// The claimed profile.
    pub profile: ClaimedPosture,
    /// Reviewer-facing profile label.
    pub claim_label: String,
    /// Stable claim id (the profile token), used as a ref by consumers.
    pub claim_ref: String,
    /// Public qualification the profile wants to keep.
    pub claimed_qualification: QualificationClass,
    /// One cell per certification dimension.
    pub cells: Vec<CertificationCell>,
    /// The applicable dimensions for this profile, in dimension order.
    pub applicable_dimensions: Vec<CertificationDimension>,
    /// Effective qualification after the gate applies.
    pub effective_qualification: QualificationClass,
    /// Gate decision the release automation reads.
    pub gate: DescriptorGate,
    /// Coverage status (mirrors [`Self::gate`]).
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id for the verdict; prefixed
    /// [`M5_ASSURANCE_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub verdict_message_id: String,
}

impl ProfileClaim {
    /// Derives a claim row for a profile from the governed facet rows.
    fn derive(
        profile: ClaimedPosture,
        claimed_qualification: QualificationClass,
        facets: &[AssuranceFacetRow],
    ) -> Self {
        let cells: Vec<CertificationCell> = CertificationDimension::ALL
            .iter()
            .map(|d| CertificationCell::derive(*d, profile, facets))
            .collect();

        let applicable_dimensions: Vec<CertificationDimension> = cells
            .iter()
            .filter(|c| c.is_applicable())
            .map(|c| c.dimension)
            .collect();

        let gate = cells
            .iter()
            .filter(|c| c.is_applicable())
            .map(|c| c.gate)
            .fold(DescriptorGate::Governed, worst_gate);
        let effective_qualification = narrow_qualification(claimed_qualification, gate);
        let status = status_for_gate(gate);

        Self {
            profile,
            claim_label: profile_label(profile).to_owned(),
            claim_ref: profile.as_str().to_owned(),
            claimed_qualification,
            cells,
            applicable_dimensions,
            effective_qualification,
            gate,
            status,
            signal: status.signal(),
            verdict_message_id: format!(
                "{}profile.{}.verdict",
                M5_ASSURANCE_CERTIFICATION_MESSAGE_ID_PREFIX,
                profile.as_str()
            ),
        }
    }

    /// The cell for a given dimension, if present.
    pub fn cell(&self, dimension: CertificationDimension) -> Option<&CertificationCell> {
        self.cells.iter().find(|c| c.dimension == dimension)
    }

    /// True when every applicable dimension is certified.
    pub fn is_certified(&self) -> bool {
        self.gate == DescriptorGate::Governed
    }

    /// True when the profile narrowed below its claimed qualification.
    pub fn is_narrowed(&self) -> bool {
        self.gate == DescriptorGate::Narrowed
    }

    /// True when the profile is blocked from Stable promotion.
    pub fn is_blocked(&self) -> bool {
        self.gate == DescriptorGate::Blocked
    }

    /// Re-derives the claim's gate, qualification, and status from its cells and reports drift.
    fn validate(&self) -> Vec<M5AssuranceCertificationViolation> {
        let mut out = Vec::new();
        for cell in &self.cells {
            out.extend(cell.validate());
        }
        if self.cells.len() != CertificationDimension::ALL.len() {
            out.push(M5AssuranceCertificationViolation::ClaimDimensionMissing);
        }
        if self.claim_label != profile_label(self.profile)
            || self.claim_ref != self.profile.as_str()
        {
            out.push(M5AssuranceCertificationViolation::ClaimVerdictDrift);
        }
        let gate = self
            .cells
            .iter()
            .filter(|c| c.is_applicable())
            .map(|c| c.gate)
            .fold(DescriptorGate::Governed, worst_gate);
        let effective = narrow_qualification(self.claimed_qualification, gate);
        let applicable: Vec<CertificationDimension> = self
            .cells
            .iter()
            .filter(|c| c.is_applicable())
            .map(|c| c.dimension)
            .collect();
        if self.gate != gate
            || self.effective_qualification != effective
            || self.status != status_for_gate(gate)
            || self.signal != status_for_gate(gate).signal()
            || self.applicable_dimensions != applicable
        {
            out.push(M5AssuranceCertificationViolation::ClaimVerdictDrift);
        }
        if !self
            .verdict_message_id
            .starts_with(M5_ASSURANCE_CERTIFICATION_MESSAGE_ID_PREFIX)
        {
            out.push(M5AssuranceCertificationViolation::UnprefixedMessageId);
        }
        out
    }
}

/// One claimed release surface that reads the certification grid. Naming the surface and the
/// dimensions it surfaces is what lets release, help, support, shiproom, and evaluation read one
/// certification rather than a parallel trust inventory each.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationConsumer {
    /// The release center.
    ReleaseCenter,
    /// About / help.
    HelpAbout,
    /// The shiproom dashboard.
    Shiproom,
    /// Support exports / field bundles.
    SupportExport,
    /// The procurement / evaluation pack.
    ProcurementEvaluation,
}

impl CertificationConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReleaseCenter,
        Self::HelpAbout,
        Self::Shiproom,
        Self::SupportExport,
        Self::ProcurementEvaluation,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::HelpAbout => "help_about",
            Self::Shiproom => "shiproom",
            Self::SupportExport => "support_export",
            Self::ProcurementEvaluation => "procurement_evaluation",
        }
    }

    /// Reviewer-facing consumer label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "Release center",
            Self::HelpAbout => "About / help",
            Self::Shiproom => "Shiproom",
            Self::SupportExport => "Support export",
            Self::ProcurementEvaluation => "Procurement / evaluation",
        }
    }

    /// Owner role accountable for keeping this consumer's binding current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center_owner",
            Self::HelpAbout => "help_about_owner",
            Self::Shiproom => "shiproom_owner",
            Self::SupportExport => "support_export_owner",
            Self::ProcurementEvaluation => "procurement_owner",
        }
    }

    /// The dimensions this consumer surfaces from the certification grid.
    pub const fn read_dimensions(self) -> &'static [CertificationDimension] {
        match self {
            Self::ReleaseCenter | Self::Shiproom | Self::ProcurementEvaluation => {
                &CertificationDimension::ALL
            }
            // About / help surfaces the assurance claim, governance posture, and boundary / route
            // story, but not the deep event-provenance ledger.
            Self::HelpAbout => &[
                CertificationDimension::AssuranceCenter,
                CertificationDimension::Governance,
                CertificationDimension::BoundaryRoute,
            ],
            // Support bundles carry the assurance claim, boundary / route, and event-provenance
            // lineage a field investigation needs, but not the governance-freshness dashboard.
            Self::SupportExport => &[
                CertificationDimension::AssuranceCenter,
                CertificationDimension::BoundaryRoute,
                CertificationDimension::EventProvenance,
            ],
        }
    }
}

/// One claimed consumer's binding to the certification grid: the dimensions it surfaces, its derived
/// posture, and the exact profile claims it must narrow or block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationConsumerRow {
    /// The consumer surface.
    pub consumer: CertificationConsumer,
    /// Reviewer-facing consumer label.
    pub consumer_label: String,
    /// Owner role accountable for keeping this consumer's binding current.
    pub owner_role: String,
    /// The dimensions this consumer surfaces, in dimension order.
    pub read_dimensions: Vec<CertificationDimension>,
    /// Gate decision derived from the cells this consumer surfaces.
    pub gate: DescriptorGate,
    /// Coverage status (mirrors [`Self::gate`]).
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Profile refs this consumer narrows (a surfaced cell is narrowed), in grid order.
    pub narrowed_profile_refs: Vec<String>,
    /// Profile refs this consumer blocks (a surfaced cell is blocked), in grid order.
    pub blocked_profile_refs: Vec<String>,
    /// Stable message id for the status; prefixed
    /// [`M5_ASSURANCE_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
}

impl CertificationConsumerRow {
    /// Derives a consumer row from the qualified profile claims.
    fn derive(consumer: CertificationConsumer, profiles: &[ProfileClaim]) -> Self {
        let read = consumer.read_dimensions();
        let mut gate = DescriptorGate::Governed;
        let mut narrowed: Vec<String> = Vec::new();
        let mut blocked: Vec<String> = Vec::new();
        for claim in profiles {
            let mut claim_gate = DescriptorGate::Governed;
            for cell in &claim.cells {
                if cell.is_applicable() && read.contains(&cell.dimension) {
                    claim_gate = worst_gate(claim_gate, cell.gate);
                }
            }
            gate = worst_gate(gate, claim_gate);
            match claim_gate {
                DescriptorGate::Narrowed => narrowed.push(claim.claim_ref.clone()),
                DescriptorGate::Blocked => blocked.push(claim.claim_ref.clone()),
                DescriptorGate::Governed => {}
            }
        }
        let status = status_for_gate(gate);
        Self {
            consumer,
            consumer_label: consumer.label().to_owned(),
            owner_role: consumer.owner_role().to_owned(),
            read_dimensions: read.to_vec(),
            gate,
            status,
            signal: status.signal(),
            narrowed_profile_refs: narrowed,
            blocked_profile_refs: blocked,
            status_message_id: format!(
                "{}consumer.{}.status",
                M5_ASSURANCE_CERTIFICATION_MESSAGE_ID_PREFIX,
                consumer.as_str()
            ),
        }
    }

    /// True when this consumer surfaces no narrowed or blocked profile.
    pub fn is_certified(&self) -> bool {
        self.gate == DescriptorGate::Governed
    }

    /// True when this consumer must narrow at least one profile but block none.
    pub fn is_narrowed(&self) -> bool {
        self.gate == DescriptorGate::Narrowed
    }

    /// True when this consumer must block at least one profile.
    pub fn is_blocked(&self) -> bool {
        self.gate == DescriptorGate::Blocked
    }

    /// Re-derives the consumer's posture from the profiles and reports drift.
    fn validate(&self, profiles: &[ProfileClaim]) -> Vec<M5AssuranceCertificationViolation> {
        let recomputed = Self::derive(self.consumer, profiles);
        let mut out = Vec::new();
        if self.owner_role != self.consumer.owner_role()
            || self.read_dimensions != self.consumer.read_dimensions()
            || self.gate != recomputed.gate
            || self.status != recomputed.status
            || self.signal != recomputed.signal
            || self.narrowed_profile_refs != recomputed.narrowed_profile_refs
            || self.blocked_profile_refs != recomputed.blocked_profile_refs
        {
            out.push(M5AssuranceCertificationViolation::ConsumerVerdictDrift);
        }
        if !self
            .status_message_id
            .starts_with(M5_ASSURANCE_CERTIFICATION_MESSAGE_ID_PREFIX)
        {
            out.push(M5AssuranceCertificationViolation::UnprefixedMessageId);
        }
        out
    }
}

/// Compact certification summary derived from the profiles and consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSummary {
    /// Total claimed profiles.
    pub total_profiles: u32,
    /// Profiles certified at their claimed qualification.
    pub certified_profiles: u32,
    /// Profiles narrowed below their claimed qualification.
    pub narrowed_profiles: u32,
    /// Profiles blocked from Stable promotion.
    pub blocked_profiles: u32,
    /// Total consumer surfaces.
    pub total_consumers: u32,
    /// Consumers that surface no narrowed or blocked profile.
    pub certified_consumers: u32,
    /// Consumers that must narrow at least one profile (and block none).
    pub narrowed_consumers: u32,
    /// Consumers that must block at least one profile.
    pub blocked_consumers: u32,
    /// True when any profile is blocked.
    pub blocks_stable_promotion: bool,
}

impl CertificationSummary {
    fn derive(profiles: &[ProfileClaim], consumers: &[CertificationConsumerRow]) -> Self {
        let certified_profiles = profiles.iter().filter(|c| c.is_certified()).count() as u32;
        let narrowed_profiles = profiles.iter().filter(|c| c.is_narrowed()).count() as u32;
        let blocked_profiles = profiles.iter().filter(|c| c.is_blocked()).count() as u32;
        let certified_consumers = consumers.iter().filter(|c| c.is_certified()).count() as u32;
        let narrowed_consumers = consumers.iter().filter(|c| c.is_narrowed()).count() as u32;
        let blocked_consumers = consumers.iter().filter(|c| c.is_blocked()).count() as u32;
        Self {
            total_profiles: profiles.len() as u32,
            certified_profiles,
            narrowed_profiles,
            blocked_profiles,
            total_consumers: consumers.len() as u32,
            certified_consumers,
            narrowed_consumers,
            blocked_consumers,
            blocks_stable_promotion: blocked_profiles > 0,
        }
    }
}

/// Packet-level release gate: the aggregate decision plus the exact profiles and dimensions that
/// drifted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationReleaseGate {
    /// Aggregate gate decision (worst over all profiles).
    pub gate: DescriptorGate,
    /// True when the gate holds Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Profile refs that narrowed, in grid order.
    pub narrowed_profile_refs: Vec<String>,
    /// Profile refs that blocked, in grid order.
    pub blocked_profile_refs: Vec<String>,
    /// Dimension tokens that drifted on any profile, in dimension order.
    pub drifted_dimensions: Vec<String>,
    /// Stable message id; prefixed [`M5_ASSURANCE_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

impl CertificationReleaseGate {
    fn derive(profiles: &[ProfileClaim]) -> Self {
        let mut gate = DescriptorGate::Governed;
        let mut narrowed: Vec<String> = Vec::new();
        let mut blocked: Vec<String> = Vec::new();
        let mut drifted: Vec<CertificationDimension> = Vec::new();
        for claim in profiles {
            gate = worst_gate(gate, claim.gate);
            match claim.gate {
                DescriptorGate::Narrowed => narrowed.push(claim.claim_ref.clone()),
                DescriptorGate::Blocked => blocked.push(claim.claim_ref.clone()),
                DescriptorGate::Governed => {}
            }
            for cell in &claim.cells {
                if cell.is_applicable() && cell.gate != DescriptorGate::Governed {
                    drifted.push(cell.dimension);
                }
            }
        }
        drifted.sort_by_key(|d| dimension_rank(*d));
        drifted.dedup();
        Self {
            gate,
            blocks_stable_promotion: gate.blocks(),
            narrowed_profile_refs: narrowed,
            blocked_profile_refs: blocked,
            drifted_dimensions: drifted.iter().map(|d| d.as_str().to_owned()).collect(),
            gate_message_id: format!("{M5_ASSURANCE_CERTIFICATION_MESSAGE_ID_PREFIX}release_gate"),
        }
    }
}

/// The controlled vocabulary the certification freezes, so consumers can enumerate the profiles,
/// dimensions, outcomes, gap kinds, facets, and consumers without re-deriving them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationVocabulary {
    /// Profile tokens.
    pub profiles: Vec<String>,
    /// Dimension tokens.
    pub dimensions: Vec<String>,
    /// Outcome tokens.
    pub outcomes: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Facet tokens.
    pub facets: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
}

impl CertificationVocabulary {
    /// The frozen vocabulary.
    pub fn canonical() -> Self {
        Self {
            profiles: tokens(&ClaimedPosture::ALL, |p| p.as_str()),
            dimensions: tokens(&CertificationDimension::ALL, |d| d.as_str()),
            outcomes: tokens(&CertificationOutcome::ALL, |o| o.as_str()),
            gap_kinds: tokens(&AssuranceGapKind::ALL, |g| g.as_str()),
            facets: tokens(&AssuranceFacet::ALL, |f| f.as_str()),
            consumers: tokens(&CertificationConsumer::ALL, |c| c.as_str()),
        }
    }

    /// True when the vocabulary matches the frozen enums.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Which surfaces consume the certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDisclosure {
    /// The consumer tokens that read this certification.
    pub consumers: Vec<String>,
    /// True when release, help, shiproom, support, and evaluation all consume one certification.
    pub one_certification_across_surfaces: bool,
}

impl CertificationDisclosure {
    fn all_surfaces() -> Self {
        Self {
            consumers: tokens(&CertificationConsumer::ALL, |c| c.as_str()),
            one_certification_across_surfaces: true,
        }
    }
}

/// Conformance review the certification publishes about itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationConformance {
    /// Every claimed profile is mapped to proof for every applicable dimension.
    pub every_profile_mapped_to_proof: bool,
    /// Stale / expired / missing proof narrows or blocks the profile deterministically.
    pub stale_proof_narrows_deterministically: bool,
    /// Profiles narrow per claim rather than behind a generic trust badge.
    pub narrowing_is_per_profile: bool,
    /// Release, help, shiproom, support, and evaluation consume one certification.
    pub surfaces_consume_one_certification: bool,
    /// The certification is generated from the governance matrix's checked-in proofs.
    pub generated_from_governance_matrix: bool,
    /// The controlled enums are frozen.
    pub controlled_enums_frozen: bool,
    /// The export carries no credential bodies or raw provider payloads.
    pub export_carries_no_raw_material: bool,
    /// Every applicable cell preserves refs-only route / evidence lineage (a proof ref per facet).
    pub export_preserves_route_evidence_lineage: bool,
}

impl CertificationConformance {
    fn derive(
        profiles: &[ProfileClaim],
        consumers: &[CertificationConsumerRow],
        governance_ref: &str,
    ) -> Self {
        let lineage_complete = !profiles.is_empty()
            && profiles.iter().all(|claim| {
                claim.cells.iter().filter(|c| c.is_applicable()).all(|c| {
                    !c.proof_refs.is_empty() && c.proof_refs.len() == c.backing_facets.len()
                })
            });
        Self {
            every_profile_mapped_to_proof: lineage_complete,
            stale_proof_narrows_deterministically: true,
            narrowing_is_per_profile: true,
            surfaces_consume_one_certification: consumers.len() == CertificationConsumer::ALL.len()
                && !governance_ref.is_empty(),
            generated_from_governance_matrix: !governance_ref.is_empty(),
            controlled_enums_frozen: true,
            export_carries_no_raw_material: true,
            export_preserves_route_evidence_lineage: lineage_complete,
        }
    }

    /// True when every conformance claim holds.
    pub fn all_hold(&self) -> bool {
        self.every_profile_mapped_to_proof
            && self.stale_proof_narrows_deterministically
            && self.narrowing_is_per_profile
            && self.surfaces_consume_one_certification
            && self.generated_from_governance_matrix
            && self.controlled_enums_frozen
            && self.export_carries_no_raw_material
            && self.export_preserves_route_evidence_lineage
    }
}

/// A generation channel. The output is identical for every channel — the parameter exists only to
/// prove desktop, CLI / headless, and offline / mirror generation produce byte-identical output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationChannel {
    /// The desktop UI.
    DesktopUi,
    /// The CLI / headless emitter.
    CliHeadless,
    /// An offline / mirror export.
    OfflineMirror,
}

impl CertificationChannel {
    /// Every generation channel, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopUi, Self::CliHeadless, Self::OfflineMirror];
}

/// The one inspectable, serde-serializable certification truth release/help/shiproom/support/evaluation
/// surfaces consume: the qualified profile grid, the per-consumer bindings, a summary, the release
/// gate, the controlled vocabulary, and a conformance review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AssuranceCertification {
    /// Record kind; must equal [`M5_ASSURANCE_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ASSURANCE_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the certification was computed as-of.
    pub evaluated_at: String,
    /// The governance packet id this certification was projected from.
    pub governance_packet_id: String,
    /// Repo-relative ref of the governance matrix this certification consumes.
    pub governance_ref: String,
    /// The qualified profile claims.
    pub profiles: Vec<ProfileClaim>,
    /// The claimed consumer bindings with their derived postures.
    pub consumers: Vec<CertificationConsumerRow>,
    /// The consumer tokens that read this certification.
    pub consumer_tokens: Vec<String>,
    /// Which surfaces consume the certification.
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

/// Constructor input for [`M5AssuranceCertification::from_governance`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AssuranceCertificationInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The claimed (profile, claimed qualification) tuples to qualify.
    pub profiles: Vec<(ClaimedPosture, QualificationClass)>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AssuranceCertification {
    /// Projects a governance matrix onto the claimed profile grid and qualifies each profile,
    /// deriving every claim's cells, every consumer's posture, the summary, gate, and conformance
    /// review from the matrix's governed facets.
    pub fn from_governance(
        governance: &M5AssuranceRouteGovernance,
        input: M5AssuranceCertificationInput,
    ) -> Self {
        let profiles: Vec<ProfileClaim> = input
            .profiles
            .iter()
            .map(|(profile, claimed)| ProfileClaim::derive(*profile, *claimed, &governance.facets))
            .collect();
        let consumers: Vec<CertificationConsumerRow> = CertificationConsumer::ALL
            .iter()
            .map(|c| CertificationConsumerRow::derive(*c, &profiles))
            .collect();
        let summary = CertificationSummary::derive(&profiles, &consumers);
        let release_gate = CertificationReleaseGate::derive(&profiles);
        let conformance =
            CertificationConformance::derive(&profiles, &consumers, M5_ASSURANCE_ROUTE_REF);
        Self {
            record_kind: M5_ASSURANCE_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_ASSURANCE_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: governance.evaluated_at.clone(),
            governance_packet_id: governance.packet_id.clone(),
            governance_ref: M5_ASSURANCE_ROUTE_REF.to_owned(),
            profiles,
            consumers,
            consumer_tokens: tokens(&CertificationConsumer::ALL, |c| c.as_str()),
            disclosure: CertificationDisclosure::all_surfaces(),
            summary,
            release_gate,
            vocabulary: CertificationVocabulary::canonical(),
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release automation must hold Stable promotion.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.release_gate.blocks_stable_promotion
    }

    /// Finds a claim by profile.
    pub fn profile(&self, profile: ClaimedPosture) -> Option<&ProfileClaim> {
        self.profiles.iter().find(|c| c.profile == profile)
    }

    /// Finds a consumer row.
    pub fn consumer(&self, consumer: CertificationConsumer) -> Option<&CertificationConsumerRow> {
        self.consumers.iter().find(|c| c.consumer == consumer)
    }

    /// Renders the packet for a generation channel; identical output for every channel.
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
        serde_json::to_string_pretty(self).expect("m5 assurance certification serializes")
    }

    /// Deterministic, machine-readable grid CSV: one row per (profile, dimension) cell, naming the
    /// profile, dimension, outcome, proof freshness, and gap.
    pub fn render_grid_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "profile,claimed_qualification,effective_qualification,profile_gate,dimension,outcome,proof_freshness,state_gate,worst_state,backing_facets,proof_refs,gap_kind\n",
        );
        for claim in &self.profiles {
            for cell in &claim.cells {
                out.push_str(&format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                    claim.profile.as_str(),
                    claim.claimed_qualification.as_str(),
                    claim.effective_qualification.as_str(),
                    claim.gate.as_str(),
                    cell.dimension.as_str(),
                    cell.outcome.as_str(),
                    cell.proof_freshness.map(|f| f.as_str()).unwrap_or(""),
                    cell.state_gate.map(|g| g.as_str()).unwrap_or(""),
                    cell.worst_state_token.as_deref().unwrap_or(""),
                    join_tokens(&cell.backing_facets, |f| f.as_str()),
                    cell.proof_refs.join("|"),
                    cell.gap_kind.map(|g| g.as_str()).unwrap_or(""),
                ));
            }
        }
        out
    }

    /// Deterministic certification document for review, release, support, docs, or shiproom handoff.
    pub fn render_certification_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 assurance / governance / route-provenance certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Projected from governance matrix `{}` (`{}`)\n",
            self.governance_packet_id, self.governance_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_profiles,
            self.summary.certified_profiles,
            self.summary.narrowed_profiles,
            self.summary.blocked_profiles
        ));
        out.push_str(&format!(
            "- Consumers: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_consumers,
            self.summary.certified_consumers,
            self.summary.narrowed_consumers,
            self.summary.blocked_consumers
        ));
        out.push_str(&format!(
            "- Blocks Stable promotion: {}\n\n",
            self.release_gate.blocks_stable_promotion
        ));

        out.push_str("## Profile qualification grid\n\n");
        out.push_str(
            "| Profile | Claimed | Effective | Assurance center | Governance | Boundary / route | Event provenance |\n",
        );
        out.push_str("|---|---|---|---|---|---|---|\n");
        for claim in &self.profiles {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | {} | {} | {} |\n",
                claim.claim_ref,
                claim.claimed_qualification.as_str(),
                claim.effective_qualification.as_str(),
                cell_md(claim.cell(CertificationDimension::AssuranceCenter)),
                cell_md(claim.cell(CertificationDimension::Governance)),
                cell_md(claim.cell(CertificationDimension::BoundaryRoute)),
                cell_md(claim.cell(CertificationDimension::EventProvenance)),
            ));
        }
        out.push('\n');

        out.push_str("## Narrowed / blocked profiles\n\n");
        if self.release_gate.narrowed_profile_refs.is_empty()
            && self.release_gate.blocked_profile_refs.is_empty()
        {
            out.push_str("- none — every claimed profile is certified.\n");
        } else {
            for claim in &self.profiles {
                if claim.gate == DescriptorGate::Governed {
                    continue;
                }
                let causes: Vec<String> = claim
                    .cells
                    .iter()
                    .filter(|c| c.is_applicable() && c.gap_kind.is_some())
                    .map(|c| {
                        format!(
                            "{} ({})",
                            c.dimension.as_str(),
                            c.gap_kind.map(|g| g.as_str()).unwrap_or("")
                        )
                    })
                    .collect();
                out.push_str(&format!(
                    "- `{}` → `{}` ({}); gap: {}\n",
                    claim.claim_ref,
                    claim.effective_qualification.as_str(),
                    claim.gate.as_str(),
                    causes.join(", ")
                ));
            }
        }
        out.push('\n');

        out.push_str("## Consumers\n\n");
        for c in &self.consumers {
            out.push_str(&format!(
                "- `{}` → `{}` ({}); narrows {}, blocks {}\n",
                c.consumer.as_str(),
                c.status.as_str(),
                c.gate.as_str(),
                c.narrowed_profile_refs.len(),
                c.blocked_profile_refs.len()
            ));
        }
        out
    }

    /// Compact Markdown summary for the release-grade parity proof report.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Aureline M5 assurance certification\n\n");
        out.push_str(&format!(
            "{} profiles ({} certified, {} narrowed, {} blocked), {} consumers — projected from `{}`; {}.\n\n",
            self.summary.total_profiles,
            self.summary.certified_profiles,
            self.summary.narrowed_profiles,
            self.summary.blocked_profiles,
            self.summary.total_consumers,
            self.governance_packet_id,
            if self.release_gate.blocks_stable_promotion {
                "Stable promotion held"
            } else {
                "Stable promotion clear"
            }
        ));
        out.push_str("## Profiles\n\n");
        for claim in &self.profiles {
            out.push_str(&format!(
                "- `{}` → `{}` ({})\n",
                claim.claim_ref,
                claim.effective_qualification.as_str(),
                claim.gate.as_str()
            ));
        }
        out.push_str("\n## Consumers\n\n");
        for c in &self.consumers {
            out.push_str(&format!(
                "- `{}` → `{}` ({})\n",
                c.consumer.as_str(),
                c.status.as_str(),
                c.gate.as_str()
            ));
        }
        out
    }

    /// Validates the packet's invariants: the header tags, every claim and cell re-derives, every
    /// consumer's posture re-derives, the summary and gate match, the vocabulary is frozen, and the
    /// export carries no raw material.
    pub fn validate(&self) -> Vec<M5AssuranceCertificationViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_ASSURANCE_CERTIFICATION_RECORD_KIND
            || self.schema_version != M5_ASSURANCE_CERTIFICATION_SCHEMA_VERSION
        {
            out.push(M5AssuranceCertificationViolation::HeaderInvalid);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5AssuranceCertificationViolation::HeaderInvalid);
        }
        if self.profiles.is_empty() {
            out.push(M5AssuranceCertificationViolation::NoProfiles);
        }
        for claim in &self.profiles {
            out.extend(claim.validate());
        }
        // No profile appears twice.
        let mut refs: Vec<&str> = self.profiles.iter().map(|c| c.claim_ref.as_str()).collect();
        refs.sort_unstable();
        let unique = refs.len();
        refs.dedup();
        if refs.len() != unique {
            out.push(M5AssuranceCertificationViolation::DuplicateProfile);
        }

        if self.consumers.len() != CertificationConsumer::ALL.len() {
            out.push(M5AssuranceCertificationViolation::ConsumerMissing);
        }
        for consumer in &self.consumers {
            out.extend(consumer.validate(&self.profiles));
        }

        if self.summary != CertificationSummary::derive(&self.profiles, &self.consumers) {
            out.push(M5AssuranceCertificationViolation::SummaryDrift);
        }
        if self.release_gate != CertificationReleaseGate::derive(&self.profiles) {
            out.push(M5AssuranceCertificationViolation::ReleaseGateDrift);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5AssuranceCertificationViolation::VocabularyDrift);
        }
        if !self.conformance.all_hold() {
            out.push(M5AssuranceCertificationViolation::ConformanceFailed);
        }
        if self.consumer_tokens != tokens(&CertificationConsumer::ALL, |c| c.as_str()) {
            out.push(M5AssuranceCertificationViolation::VocabularyDrift);
        }
        if self.governance_packet_id.is_empty() || self.governance_ref.is_empty() {
            out.push(M5AssuranceCertificationViolation::GovernanceRefMissing);
        }
        if json_contains_forbidden_material(self) {
            out.push(M5AssuranceCertificationViolation::ForbiddenMaterial);
        }
        out
    }
}

/// A way a [`M5AssuranceCertification`] packet can fail validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AssuranceCertificationViolation {
    /// The record kind, schema version, or an identity field is wrong.
    HeaderInvalid,
    /// The certification qualifies no profiles.
    NoProfiles,
    /// A profile is missing a cell for a dimension.
    ClaimDimensionMissing,
    /// A profile's gate, qualification, or status does not match its cells.
    ClaimVerdictDrift,
    /// A cell's outcome, gate, status, or gap does not match its derivation.
    CellOutcomeDrift,
    /// A cell field drifted from its dimension.
    CellFieldMismatch,
    /// Two profiles share a claim ref.
    DuplicateProfile,
    /// A claimed consumer surface is missing.
    ConsumerMissing,
    /// A consumer's posture does not match the profiles it surfaces.
    ConsumerVerdictDrift,
    /// The summary does not match the profiles and consumers.
    SummaryDrift,
    /// The release gate does not match the profiles.
    ReleaseGateDrift,
    /// The controlled vocabulary drifted from the frozen enums.
    VocabularyDrift,
    /// A conformance claim does not hold.
    ConformanceFailed,
    /// The governance matrix provenance is missing.
    GovernanceRefMissing,
    /// A message id is not prefixed with the lane prefix.
    UnprefixedMessageId,
    /// The export carries credential bodies or raw provider payloads.
    ForbiddenMaterial,
}

impl M5AssuranceCertificationViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeaderInvalid => "header_invalid",
            Self::NoProfiles => "no_profiles",
            Self::ClaimDimensionMissing => "claim_dimension_missing",
            Self::ClaimVerdictDrift => "claim_verdict_drift",
            Self::CellOutcomeDrift => "cell_outcome_drift",
            Self::CellFieldMismatch => "cell_field_mismatch",
            Self::DuplicateProfile => "duplicate_profile",
            Self::ConsumerMissing => "consumer_missing",
            Self::ConsumerVerdictDrift => "consumer_verdict_drift",
            Self::SummaryDrift => "summary_drift",
            Self::ReleaseGateDrift => "release_gate_drift",
            Self::VocabularyDrift => "vocabulary_drift",
            Self::ConformanceFailed => "conformance_failed",
            Self::GovernanceRefMissing => "governance_ref_missing",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

/// Reviewer-facing profile label.
fn profile_label(profile: ClaimedPosture) -> &'static str {
    match profile {
        ClaimedPosture::Managed => "Managed",
        ClaimedPosture::SelfHosted => "Self-hosted",
        ClaimedPosture::Regulated => "Regulated",
        ClaimedPosture::Sovereign => "Sovereign / air-gapped",
    }
}

/// Compact Markdown cell token for the grid table.
fn cell_md(cell: Option<&CertificationCell>) -> String {
    match cell {
        None => "—".to_owned(),
        Some(c) => match c.outcome {
            CertificationOutcome::NotApplicable => "n/a".to_owned(),
            _ => format!("`{}`", c.outcome.as_str()),
        },
    }
}

/// Position of a freshness state in the most→least fresh ordering (higher = worse).
fn freshness_rank(freshness: FreshnessState) -> usize {
    FreshnessState::ALL
        .iter()
        .position(|f| *f == freshness)
        .unwrap_or(FreshnessState::ALL.len())
}

/// Position of a gate in the least→most severe ordering (higher = worse).
fn gate_rank(gate: DescriptorGate) -> usize {
    match gate {
        DescriptorGate::Governed => 0,
        DescriptorGate::Narrowed => 1,
        DescriptorGate::Blocked => 2,
    }
}

/// Position of a dimension in the canonical ordering.
fn dimension_rank(dimension: CertificationDimension) -> usize {
    CertificationDimension::ALL
        .iter()
        .position(|d| *d == dimension)
        .unwrap_or(CertificationDimension::ALL.len())
}

/// Position of a facet in the canonical ordering.
fn facet_rank(facet: AssuranceFacet) -> usize {
    AssuranceFacet::ALL
        .iter()
        .position(|f| *f == facet)
        .unwrap_or(AssuranceFacet::ALL.len())
}

/// Collects the stable tokens of an enum slice.
fn tokens<T: Copy>(items: &[T], f: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|item| f(*item).to_owned()).collect()
}

/// Joins the stable tokens of a slice with `|`.
fn join_tokens<T: Copy>(items: &[T], f: impl Fn(T) -> &'static str) -> String {
    items
        .iter()
        .map(|item| f(*item))
        .collect::<Vec<_>>()
        .join("|")
}

/// True when the serialized packet contains a forbidden credential / raw-payload marker. The
/// certification is metadata-only, so this guards against an accidental leak rather than a real path.
fn json_contains_forbidden_material(packet: &M5AssuranceCertification) -> bool {
    let json = serde_json::to_string(packet)
        .unwrap_or_default()
        .to_ascii_lowercase();
    [
        "bearer_token",
        "authorization:",
        "private_key",
        "secret_key",
    ]
    .iter()
    .any(|needle| json.contains(needle))
}
