//! Certification capstone for every claimed runbook-backed incident/operator row.
//!
//! The six lanes in this crate each freeze one slice of runbook truth and ship one
//! proof artifact: the [governance matrix](crate::m5_runbook_governance), the
//! [source register](crate::m5_runbook_sources), the
//! [executable step library](crate::m5_runbook_steps), the
//! [execution history](crate::m5_runbook_executions), the
//! [handoff register](crate::m5_runbook_handoffs), and the
//! [companion register](crate::m5_runbook_companion). This module is the qualification
//! capstone that binds those lane proofs to the *product rows* that claim
//! runbook-backed behavior, so a claim only stands while the proof under it is current.
//!
//! Each [`RunbookProofLane`] declares the [certification facet](CertificationFacet) it
//! covers — source truth, step lineage, boundary honesty, or export proof — its
//! source-of-truth schema, its published register, its release proof artifact, and a
//! [proof-freshness state](crate::m5_runbook_governance::ProofFreshnessState). Each
//! [claimed incident/operator row](IncidentOperatorRow) binds the proof lanes it
//! depends on, and the certification *derives*, per row:
//!
//! - the exact [coverage gaps](RunbookCertificationGap) — a bound lane the packet does
//!   not govern, or whose proof is stale or missing;
//! - a [`RunbookGate`](crate::m5_runbook_governance::RunbookGate) the
//!   release/public-truth automation reads — a row whose bound lane is unmapped or
//!   whose proof is missing is *blocked* from Stable promotion (and named, never
//!   hidden), while a row whose bound lane's proof is stale *auto-narrows* below Stable;
//! - an effective
//!   [`RunbookClaimClass`](crate::m5_runbook_governance::RunbookClaimClass) after the
//!   gate applies, floored at Beta for any narrowing gap and held for any blocking gap.
//!
//! The narrowing is deterministic: a stale or missing lane proof always narrows or
//! blocks the rows that bind it, so an aged proof never leaves a claim standing as
//! implied stable behavior. The [`M5RunbookCertificationPacket`] is the one
//! inspectable, serde-serializable truth packet the consuming surfaces read, and the
//! [`CertificationDisclosure`] records that Help/About, the shiproom, support exports,
//! and the incident/operator surfaces all consume *this* qualification rather than a
//! private spreadsheet. The packet carries metadata and refs only: no credential
//! bodies or raw provider/console payloads.
//!
//! - Packet schema:
//!   [`schemas/runbooks/m5-runbook-certification.schema.json`](../../../../../schemas/runbooks/m5-runbook-certification.schema.json)
//! - Contract doc:
//!   [`docs/runbooks/m5-runbook-certification.md`](../../../../../docs/runbooks/m5-runbook-certification.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_runbook_certification_packet,
    seeded_m5_runbook_certification_packet_missing_proof_blocked,
    seeded_m5_runbook_certification_packet_stale_proof_narrowed,
    M5_RUNBOOK_CERTIFICATION_PACKET_ID,
};

use serde::{Deserialize, Serialize};

use crate::m5_runbook_governance::{
    ProofFreshnessState, RunbookClaimClass, RunbookConsumer, RunbookGapKind, RunbookGate,
    RunbookSignal, RunbookSurfaceStatus,
};

/// Record-kind tag carried by [`M5RunbookCertificationPacket`].
pub const M5_RUNBOOK_CERTIFICATION_RECORD_KIND: &str = "m5_runbook_certification";

/// Schema version for the certification packet.
pub const M5_RUNBOOK_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the certification packet schema.
pub const M5_RUNBOOK_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/runbooks/m5-runbook-certification.schema.json";

/// Repo-relative path of the published certification inventory.
pub const M5_RUNBOOK_CERTIFICATION_REF: &str = "artifacts/runbooks/m5-runbook-certification.json";

/// Repo-relative path of the release-grade certification support export.
pub const M5_RUNBOOK_CERTIFICATION_PROOF_REF: &str =
    "artifacts/release/m5-runbook-proof/runbook-certification.json";

/// Repo-relative path of the certification contract doc.
pub const M5_RUNBOOK_CERTIFICATION_DOC_REF: &str = "docs/runbooks/m5-runbook-certification.md";

/// Repo-relative directory of the certification drill fixtures.
pub const M5_RUNBOOK_CERTIFICATION_FIXTURE_DIR: &str = "fixtures/runbooks/m5-certification-drills/";

/// Prefix every certification-lane message id carries so consumers can route it.
pub const M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX: &str = "runbooks_certification.";

/// One slice of runbook truth that ships a proof artifact. Each lane is one of the
/// six governed lanes in this crate; binding a claimed row to a lane is what makes the
/// row's claim depend on that lane's proof staying current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookProofLane {
    /// The governance matrix: the object contracts and release gate.
    Governance,
    /// The source register: where each runbook's authority comes from.
    Sources,
    /// The executable step library: the step taxonomy.
    Steps,
    /// The execution history: executed-step lineage, deviation, and archival export.
    Executions,
    /// The handoff register: browser/vendor-console boundary honesty.
    Handoffs,
    /// The companion register: companion-scoped authority and its export.
    Companion,
}

impl RunbookProofLane {
    /// Every proof lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Governance,
        Self::Sources,
        Self::Steps,
        Self::Executions,
        Self::Handoffs,
        Self::Companion,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Governance => "governance",
            Self::Sources => "sources",
            Self::Steps => "steps",
            Self::Executions => "executions",
            Self::Handoffs => "handoffs",
            Self::Companion => "companion",
        }
    }

    /// The certification facet this lane covers.
    pub const fn facet(self) -> CertificationFacet {
        match self {
            Self::Governance | Self::Sources => CertificationFacet::SourceTruth,
            Self::Steps | Self::Executions => CertificationFacet::StepLineage,
            Self::Handoffs => CertificationFacet::BoundaryHonesty,
            Self::Companion => CertificationFacet::ExportProof,
        }
    }

    /// Repo-relative schema that is this lane's source of truth.
    pub const fn schema_ref(self) -> &'static str {
        match self {
            Self::Governance => "schemas/runbooks/m5-runbook-governance.schema.json",
            Self::Sources => "schemas/runbooks/m5-runbook-source-register.schema.json",
            Self::Steps => "schemas/runbooks/m5-runbook-step-library.schema.json",
            Self::Executions => "schemas/runbooks/m5-runbook-execution-history.schema.json",
            Self::Handoffs => "schemas/runbooks/m5-runbook-handoff-register.schema.json",
            Self::Companion => "schemas/runbooks/m5-runbook-companion-register.schema.json",
        }
    }

    /// Repo-relative published register/inventory for this lane.
    pub const fn register_ref(self) -> &'static str {
        match self {
            Self::Governance => "artifacts/runbooks/m5-runbook-governance.json",
            Self::Sources => "artifacts/runbooks/m5-runbook-source-register.json",
            Self::Steps => "artifacts/runbooks/m5-runbook-step-library.json",
            Self::Executions => "artifacts/runbooks/m5-runbook-execution-history.json",
            Self::Handoffs => "artifacts/runbooks/m5-runbook-handoff-register.json",
            Self::Companion => "artifacts/runbooks/m5-runbook-companion-register.json",
        }
    }

    /// Repo-relative release proof artifact for this lane.
    pub const fn proof_ref(self) -> &'static str {
        match self {
            Self::Governance => "artifacts/release/m5-runbook-proof/runbook-governance.json",
            Self::Sources => "artifacts/release/m5-runbook-proof/runbook-source-register.json",
            Self::Steps => "artifacts/release/m5-runbook-proof/runbook-step-library.json",
            Self::Executions => "artifacts/release/m5-runbook-proof/runbook-execution-history.json",
            Self::Handoffs => "artifacts/release/m5-runbook-proof/runbook-handoff-register.json",
            Self::Companion => "artifacts/release/m5-runbook-proof/runbook-companion-register.json",
        }
    }

    /// The lane's canonical register/packet id, for cross-referencing the source lane.
    pub const fn register_id(self) -> &'static str {
        match self {
            Self::Governance => "m5-runbook-governance:stable:0001",
            Self::Sources => "m5-runbook-source-register:stable:0001",
            Self::Steps => "m5-runbook-step-library:stable:0001",
            Self::Executions => "m5-runbook-execution-history:stable:0001",
            Self::Handoffs => "m5-runbook-handoff-register:stable:0001",
            Self::Companion => "m5-runbook-companion-register:stable:0001",
        }
    }

    /// Reviewer-facing lane label.
    pub const fn lane_label(self) -> &'static str {
        match self {
            Self::Governance => "Runbook governance matrix",
            Self::Sources => "Runbook source register",
            Self::Steps => "Runbook executable step library",
            Self::Executions => "Runbook execution history",
            Self::Handoffs => "Runbook control-plane handoff register",
            Self::Companion => "Runbook companion-scoped surface register",
        }
    }

    /// Owner role accountable for keeping this lane's proof current.
    pub const fn owner_role(self) -> &'static str {
        match self {
            Self::Governance => "runbook_governance_owner",
            Self::Sources => "runbook_authoring_owner",
            Self::Steps => "runbook_authoring_owner",
            Self::Executions => "incident_operations_owner",
            Self::Handoffs => "control_plane_boundary_owner",
            Self::Companion => "companion_owner",
        }
    }
}

/// The aspect of runbook truth a proof lane certifies. The exit gate narrows a claim
/// when any of these facets goes stale or failing: runbook source truth, step lineage,
/// boundary honesty, or export proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationFacet {
    /// Where a runbook's authority comes from (governance + source register).
    SourceTruth,
    /// The executable step taxonomy and its execution/deviation lineage.
    StepLineage,
    /// Browser/vendor-console pivots stay attributable handoffs, never hidden escapes.
    BoundaryHonesty,
    /// Archived execution history and companion-scoped surfaces export truthfully.
    ExportProof,
}

impl CertificationFacet {
    /// Every facet, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SourceTruth,
        Self::StepLineage,
        Self::BoundaryHonesty,
        Self::ExportProof,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceTruth => "source_truth",
            Self::StepLineage => "step_lineage",
            Self::BoundaryHonesty => "boundary_honesty",
            Self::ExportProof => "export_proof",
        }
    }
}

/// One certification surface that exposes the qualification output. Naming each
/// surface is what proves Help/About, the shiproom, and support exports all read the
/// same machine-readable qualification rather than a private spreadsheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationSurface {
    /// The Help/About panel.
    HelpAbout,
    /// The release shiproom dashboard.
    Shiproom,
    /// Support exports / bundles.
    SupportExport,
    /// The incident workspace and operator dashboard surfaces.
    IncidentOperator,
}

impl QualificationSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::HelpAbout,
        Self::Shiproom,
        Self::SupportExport,
        Self::IncidentOperator,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::Shiproom => "shiproom",
            Self::SupportExport => "support_export",
            Self::IncidentOperator => "incident_operator",
        }
    }
}

/// One governed proof lane's certification contract: the facet it covers, its owner,
/// its source-of-truth schema, its published register, its release proof, and the
/// freshness of that proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookProofLaneContract {
    /// The proof lane.
    pub lane: RunbookProofLane,
    /// Reviewer-facing lane label.
    pub lane_label: String,
    /// The certification facet this lane covers.
    pub facet: CertificationFacet,
    /// Owner role accountable for keeping the lane proof current.
    pub owner_role: String,
    /// Cross-ref to the lane's canonical register/packet id.
    pub register_id: String,
    /// Repo-relative source-of-truth schema.
    pub schema_ref: String,
    /// Repo-relative published register/inventory.
    pub register_ref: String,
    /// Repo-relative release proof artifact.
    pub proof_ref: String,
    /// Freshness of the lane's proof.
    pub proof_freshness: ProofFreshnessState,
    /// Stable message id; prefixed [`M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl RunbookProofLaneContract {
    /// Builds a lane contract for a lane at a given proof freshness, deriving every
    /// ref from the lane so the contract can never cite a stale ref.
    pub fn for_lane(lane: RunbookProofLane, proof_freshness: ProofFreshnessState) -> Self {
        Self {
            lane,
            lane_label: lane.lane_label().to_owned(),
            facet: lane.facet(),
            owner_role: lane.owner_role().to_owned(),
            register_id: lane.register_id().to_owned(),
            schema_ref: lane.schema_ref().to_owned(),
            register_ref: lane.register_ref().to_owned(),
            proof_ref: lane.proof_ref().to_owned(),
            proof_freshness,
            detail_message_id: format!(
                "{}lane.{}",
                M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX,
                lane.as_str()
            ),
        }
    }

    /// Validates the lane contract's invariants: every derived ref matches the lane,
    /// every identity field is present, and the message id carries the lane prefix.
    pub fn validate(&self) -> Vec<M5RunbookCertificationViolation> {
        let mut out = Vec::new();
        if self.facet != self.lane.facet()
            || self.schema_ref != self.lane.schema_ref()
            || self.register_ref != self.lane.register_ref()
            || self.proof_ref != self.lane.proof_ref()
            || self.register_id != self.lane.register_id()
        {
            out.push(M5RunbookCertificationViolation::LaneContractRefMismatch);
        }
        if self.lane_label.trim().is_empty() || self.owner_role.trim().is_empty() {
            out.push(M5RunbookCertificationViolation::MissingIdentity);
        }
        if !self
            .detail_message_id
            .starts_with(M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX)
        {
            out.push(M5RunbookCertificationViolation::UnprefixedMessageId);
        }
        out
    }
}

/// One coverage gap on a claimed incident/operator row: a bound lane the packet does
/// not govern, or a bound lane whose proof is stale or missing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookCertificationGap {
    /// Row this gap applies to.
    pub row_id: String,
    /// The bound proof lane the gap concerns.
    pub lane: RunbookProofLane,
    /// The kind of gap.
    pub gap_kind: RunbookGapKind,
    /// Stable message id; prefixed [`M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

/// Derived verdict for a row, computed from its gaps.
struct RowVerdict {
    status: RunbookSurfaceStatus,
    signal: RunbookSignal,
    gate: RunbookGate,
    effective_class: RunbookClaimClass,
}

/// Restrictiveness rank of a claim class, computed from the canonical `ALL` ordering
/// (least restrictive first) so the certification reuses the shipped claim taxonomy.
fn claim_rank(class: RunbookClaimClass) -> usize {
    RunbookClaimClass::ALL
        .iter()
        .position(|c| *c == class)
        .unwrap_or(RunbookClaimClass::ALL.len())
}

/// The more restrictive of two claim classes.
fn more_restrictive(a: RunbookClaimClass, b: RunbookClaimClass) -> RunbookClaimClass {
    if claim_rank(a) >= claim_rank(b) {
        a
    } else {
        b
    }
}

fn derive_row_verdict(claimed: RunbookClaimClass, gaps: &[RunbookCertificationGap]) -> RowVerdict {
    let any_blocking = gaps.iter().any(|g| g.gap_kind.is_blocking());
    let any_narrowing = gaps.iter().any(|g| !g.gap_kind.is_blocking());

    let status = if any_blocking {
        RunbookSurfaceStatus::Unmapped
    } else if any_narrowing {
        RunbookSurfaceStatus::Provisional
    } else {
        RunbookSurfaceStatus::Mapped
    };

    let gate = if any_blocking {
        RunbookGate::Blocked
    } else if any_narrowing {
        RunbookGate::Narrowed
    } else {
        RunbookGate::Governed
    };

    let effective_class = match gate {
        RunbookGate::Governed => claimed,
        RunbookGate::Blocked => RunbookClaimClass::Held,
        // A stale lane proof always narrows the claim to at least Beta — deterministic
        // and never a quiet stable claim over an aged proof.
        RunbookGate::Narrowed => more_restrictive(claimed, RunbookClaimClass::Beta),
    };

    RowVerdict {
        status,
        signal: status.signal(),
        gate,
        effective_class,
    }
}

/// One claimed runbook-backed incident/operator row: the consumer it serves, the proof
/// lanes it binds, and the verdict derived from those lanes' proof freshness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentOperatorRow {
    /// Stable row id, unique within the packet.
    pub row_id: String,
    /// Reviewer-facing row label.
    pub row_label: String,
    /// The consumer family this row belongs to.
    pub consumer: RunbookConsumer,
    /// Owner role accountable for keeping this row's claim current.
    pub owner_role: String,
    /// Public claim the row wants to keep.
    pub claimed_class: RunbookClaimClass,
    /// The proof lanes this row depends on.
    pub bound_lanes: Vec<RunbookProofLane>,
    /// The certification facets this row's bound lanes cover, in facet order.
    pub covered_facets: Vec<CertificationFacet>,
    /// Effective claim after the gate applies.
    pub effective_class: RunbookClaimClass,
    /// Green/yellow/red coverage status.
    pub status: RunbookSurfaceStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: RunbookSignal,
    /// Release-gate decision the release/public-truth automation reads.
    pub gate_decision: RunbookGate,
    /// Exact coverage gaps for this row.
    pub gaps: Vec<RunbookCertificationGap>,
    /// Stable message id for the status; prefixed
    /// [`M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
    /// Stable message id for the gate; prefixed
    /// [`M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

impl IncidentOperatorRow {
    /// Recomputes the gaps, covered facets, and verdict from the lane contracts, so a
    /// row's claim is always generated from the same checked-in lane proofs Aureline
    /// ships rather than a hand-maintained status.
    pub fn recompute(&mut self, lanes: &[RunbookProofLaneContract]) {
        let mut gaps = Vec::new();
        let mut push_gap = |lane: RunbookProofLane, kind: RunbookGapKind| {
            gaps.push(RunbookCertificationGap {
                row_id: self.row_id.clone(),
                lane,
                gap_kind: kind,
                cause_message_id: format!(
                    "{}{}.{}.{}.gap",
                    M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX,
                    self.row_id,
                    lane.as_str(),
                    kind.as_str()
                ),
            });
        };

        for &lane in &self.bound_lanes {
            match lanes.iter().find(|c| c.lane == lane) {
                None => push_gap(lane, RunbookGapKind::ObjectMappingMissing),
                Some(contract) => match contract.proof_freshness {
                    ProofFreshnessState::Current => {}
                    ProofFreshnessState::Stale => push_gap(lane, RunbookGapKind::ProofStale),
                    ProofFreshnessState::Missing => push_gap(lane, RunbookGapKind::ProofMissing),
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

        let mut facets: Vec<CertificationFacet> =
            self.bound_lanes.iter().map(|l| l.facet()).collect();
        facets.sort_by_key(|f| {
            CertificationFacet::ALL
                .iter()
                .position(|c| c == f)
                .unwrap_or(CertificationFacet::ALL.len())
        });
        facets.dedup();
        self.covered_facets = facets;

        let verdict = derive_row_verdict(self.claimed_class, &self.gaps);
        self.status = verdict.status;
        self.signal = verdict.signal;
        self.gate_decision = verdict.gate;
        self.effective_class = verdict.effective_class;
    }

    /// True when the row is blocked from Stable promotion.
    pub fn is_blocked(&self) -> bool {
        self.gate_decision.blocks()
    }

    /// True when the row auto-narrowed below its claim.
    pub fn is_narrowed(&self) -> bool {
        matches!(self.gate_decision, RunbookGate::Narrowed)
    }

    /// True when the row is fully certified for Stable promotion.
    pub fn is_certified(&self) -> bool {
        matches!(self.gate_decision, RunbookGate::Governed)
    }

    /// Validates the row's static invariants (identity, bound lanes, message ids).
    fn validate_static(&self) -> Vec<M5RunbookCertificationViolation> {
        let mut out = Vec::new();
        if self.row_id.trim().is_empty() || self.row_label.trim().is_empty() {
            out.push(M5RunbookCertificationViolation::MissingIdentity);
        }
        if self.bound_lanes.is_empty() {
            out.push(M5RunbookCertificationViolation::RowBindsNoLanes);
        }
        if !self
            .status_message_id
            .starts_with(M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX)
            || !self
                .gate_message_id
                .starts_with(M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX)
        {
            out.push(M5RunbookCertificationViolation::UnprefixedMessageId);
        }
        for gap in &self.gaps {
            if !gap
                .cause_message_id
                .starts_with(M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX)
            {
                out.push(M5RunbookCertificationViolation::UnprefixedMessageId);
            }
        }
        out
    }
}

/// Compact qualification summary — the scoreboard the Help/About, shiproom, and
/// support surfaces all read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSummary {
    /// Total governed proof lanes.
    pub total_lanes: u32,
    /// Total claimed incident/operator rows.
    pub total_rows: u32,
    /// Rows certified at their full claim (governed).
    pub certified_row_count: u32,
    /// Rows that auto-narrowed below their claim.
    pub narrowed_row_count: u32,
    /// Rows blocked from Stable promotion.
    pub blocked_row_count: u32,
    /// Lanes whose proof is current.
    pub current_lane_count: u32,
    /// Lanes whose proof is stale.
    pub stale_lane_count: u32,
    /// Lanes whose proof is missing.
    pub missing_lane_count: u32,
    /// True when at least one row is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
}

/// Packet-level release gate aggregating the per-row gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookCertificationReleaseGate {
    /// True when at least one row is blocked from Stable promotion.
    pub blocks_stable_promotion: bool,
    /// Sorted row ids blocked from Stable promotion.
    pub blocked_row_ids: Vec<String>,
    /// Sorted row ids that auto-narrowed below their claim.
    pub narrowed_row_ids: Vec<String>,
    /// Sorted row ids fully certified for Stable promotion.
    pub certified_row_ids: Vec<String>,
    /// Stable message id; prefixed [`M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX`].
    pub gate_message_id: String,
}

/// Which certification surfaces consume the one qualification output. Every flag must
/// hold so Help/About, the shiproom, support exports, and the incident/operator
/// surfaces all read the same machine-readable qualification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDisclosure {
    /// The Help/About panel exposes the current qualification.
    pub help_about_exposes_qualification: bool,
    /// The release shiproom exposes the current qualification.
    pub shiproom_exposes_qualification: bool,
    /// Support exports expose the current qualification.
    pub support_export_exposes_qualification: bool,
    /// The incident workspace and operator dashboard consume the qualification.
    pub incident_operator_surfaces_consume_qualification: bool,
}

impl CertificationDisclosure {
    /// The canonical disclosure: every surface consumes the qualification.
    pub const fn all_surfaces() -> Self {
        Self {
            help_about_exposes_qualification: true,
            shiproom_exposes_qualification: true,
            support_export_exposes_qualification: true,
            incident_operator_surfaces_consume_qualification: true,
        }
    }

    /// True when every surface consumes the qualification.
    pub const fn all_expose(&self) -> bool {
        self.help_about_exposes_qualification
            && self.shiproom_exposes_qualification
            && self.support_export_exposes_qualification
            && self.incident_operator_surfaces_consume_qualification
    }
}

/// Self-describing controlled-vocabulary set so the packet resolves every token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationVocabulary {
    /// Proof-lane tokens.
    pub proof_lanes: Vec<String>,
    /// Certification-facet tokens.
    pub certification_facets: Vec<String>,
    /// Qualification-surface tokens.
    pub qualification_surfaces: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
    /// Proof-freshness tokens.
    pub proof_freshness_states: Vec<String>,
    /// Surface-status tokens.
    pub surface_statuses: Vec<String>,
    /// Signal tokens.
    pub signals: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
    /// Gap-kind tokens.
    pub gap_kinds: Vec<String>,
    /// Claim-class tokens.
    pub claim_classes: Vec<String>,
}

impl CertificationVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            proof_lanes: RunbookProofLane::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            certification_facets: CertificationFacet::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            qualification_surfaces: QualificationSurface::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            consumers: RunbookConsumer::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            proof_freshness_states: ProofFreshnessState::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            surface_statuses: RunbookSurfaceStatus::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            signals: RunbookSignal::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            gate_decisions: RunbookGate::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            gap_kinds: RunbookGapKind::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            claim_classes: RunbookClaimClass::ALL
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

/// Certification conformance review. Every flag is a hard invariant; all must hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationConformance {
    /// Every certification facet is covered by at least one governed proof lane.
    pub every_facet_covered_by_a_proof_lane: bool,
    /// Every claimed row binds at least one proof lane.
    pub every_row_binds_at_least_one_proof_lane: bool,
    /// Every claimed row maps to current lane contracts or auto-narrows/blocks.
    pub every_row_maps_to_contracts_or_narrows: bool,
    /// A stale lane proof narrows the rows that bind it deterministically.
    pub stale_proof_narrows_deterministically: bool,
    /// A missing/unmapped lane proof blocks Stable promotion for the rows that bind it.
    pub missing_proof_blocks_stable_promotion: bool,
    /// Exact coverage gaps are named per row.
    pub exact_gaps_named_per_row: bool,
    /// Help/About, the shiproom, support, and incident/operator read one qualification.
    pub surfaces_consume_one_qualification: bool,
    /// The packet is generated from the same checked-in lane proofs.
    pub generated_from_checked_in_lane_proofs: bool,
    /// The export carries no raw boundary material.
    pub export_carries_no_raw_boundary_material: bool,
}

impl CertificationConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_facet_covered_by_a_proof_lane
            && self.every_row_binds_at_least_one_proof_lane
            && self.every_row_maps_to_contracts_or_narrows
            && self.stale_proof_narrows_deterministically
            && self.missing_proof_blocks_stable_promotion
            && self.exact_gaps_named_per_row
            && self.surfaces_consume_one_qualification
            && self.generated_from_checked_in_lane_proofs
            && self.export_carries_no_raw_boundary_material
    }
}

/// Constructor input for [`M5RunbookCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RunbookCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The governed proof-lane contracts.
    pub proof_lanes: Vec<RunbookProofLaneContract>,
    /// The claimed incident/operator rows (gaps/verdict are recomputed from the lanes).
    pub rows: Vec<IncidentOperatorRow>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 runbook certification packet: the qualification of every claimed
/// incident/operator row against the six runbook proof lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunbookCertificationPacket {
    /// Record kind; must equal [`M5_RUNBOOK_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The governed proof-lane contracts.
    pub proof_lanes: Vec<RunbookProofLaneContract>,
    /// The claimed incident/operator rows with their derived verdicts.
    pub rows: Vec<IncidentOperatorRow>,
    /// Compact qualification summary.
    pub summary: CertificationSummary,
    /// Which surfaces consume the qualification.
    pub disclosure: CertificationDisclosure,
    /// Packet-level release gate.
    pub release_gate: RunbookCertificationReleaseGate,
    /// Controlled-vocabulary set.
    pub vocabulary: CertificationVocabulary,
    /// Conformance review block.
    pub conformance: CertificationConformance,
    /// Cross-refs to the lane proof artifacts this packet certifies against.
    pub source_proof_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RunbookCertificationPacket {
    /// Builds a certification packet from seed input, recomputing each row's verdict and
    /// deriving the summary, release gate, and conformance review from the lane proofs.
    pub fn new(input: M5RunbookCertificationPacketInput) -> Self {
        let proof_lanes = input.proof_lanes;
        let mut rows = input.rows;
        for row in &mut rows {
            row.recompute(&proof_lanes);
        }
        let summary = derive_summary(&proof_lanes, &rows);
        let release_gate = derive_release_gate(&rows);
        let conformance = derive_conformance(&proof_lanes, &rows);
        let mut source_proof_refs: Vec<String> =
            proof_lanes.iter().map(|l| l.proof_ref.clone()).collect();
        source_proof_refs.sort();
        Self {
            record_kind: M5_RUNBOOK_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_RUNBOOK_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            proof_lanes,
            rows,
            summary,
            disclosure: CertificationDisclosure::all_surfaces(),
            release_gate,
            vocabulary: CertificationVocabulary::canonical(),
            conformance,
            source_proof_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release/public-truth automation must hold Stable promotion.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.release_gate.blocks_stable_promotion
    }

    /// Finds a proof-lane contract by lane.
    pub fn lane_contract(&self, lane: RunbookProofLane) -> Option<&RunbookProofLaneContract> {
        self.proof_lanes.iter().find(|c| c.lane == lane)
    }

    /// Finds a row by id.
    pub fn row(&self, row_id: &str) -> Option<&IncidentOperatorRow> {
        self.rows.iter().find(|r| r.row_id == row_id)
    }

    /// Validates the certification packet's invariants.
    pub fn validate(&self) -> Vec<M5RunbookCertificationViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_CERTIFICATION_RECORD_KIND {
            out.push(M5RunbookCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RUNBOOK_CERTIFICATION_SCHEMA_VERSION {
            out.push(M5RunbookCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5RunbookCertificationViolation::MissingIdentity);
        }

        // Every facet must be covered by at least one governed lane, and every lane
        // contract must be self-consistent. Duplicate lanes are rejected.
        let mut seen_lanes = std::collections::BTreeSet::new();
        for contract in &self.proof_lanes {
            if !seen_lanes.insert(contract.lane) {
                out.push(M5RunbookCertificationViolation::DuplicateLane);
            }
            out.extend(contract.validate());
        }
        for facet in CertificationFacet::ALL {
            if !self.proof_lanes.iter().any(|c| c.facet == facet) {
                out.push(M5RunbookCertificationViolation::FacetNotCovered);
            }
        }

        if self.rows.is_empty() {
            out.push(M5RunbookCertificationViolation::PacketHasNoRows);
        }
        let mut seen_rows = std::collections::BTreeSet::new();
        for row in &self.rows {
            if !seen_rows.insert(row.row_id.as_str()) {
                out.push(M5RunbookCertificationViolation::DuplicateRowId);
            }
            out.extend(row.validate_static());
            // The stored verdict must match a fresh recompute from the lane proofs.
            let mut probe = row.clone();
            probe.recompute(&self.proof_lanes);
            if probe.gaps != row.gaps
                || probe.covered_facets != row.covered_facets
                || probe.status != row.status
                || probe.signal != row.signal
                || probe.gate_decision != row.gate_decision
                || probe.effective_class != row.effective_class
            {
                out.push(M5RunbookCertificationViolation::RowVerdictDrift);
            }
        }

        if self.summary != derive_summary(&self.proof_lanes, &self.rows) {
            out.push(M5RunbookCertificationViolation::SummaryDrift);
        }
        if self.release_gate != derive_release_gate(&self.rows) {
            out.push(M5RunbookCertificationViolation::ReleaseGateAggregateMismatch);
        }
        if !self.disclosure.all_expose() {
            out.push(M5RunbookCertificationViolation::DisclosureIncomplete);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5RunbookCertificationViolation::VocabularyMismatch);
        }
        if self.conformance != derive_conformance(&self.proof_lanes, &self.rows)
            || !self.conformance.all_hold()
        {
            out.push(M5RunbookCertificationViolation::ConformanceReviewFailed);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 runbook certification serializes"),
        ) {
            out.push(M5RunbookCertificationViolation::RawBoundaryMaterialInExport);
        }

        out
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 runbook certification serializes")
    }

    /// Deterministic Markdown proof for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Runbook Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Proof lanes: {} ({} current, {} stale, {} missing)\n",
            self.summary.total_lanes,
            self.summary.current_lane_count,
            self.summary.stale_lane_count,
            self.summary.missing_lane_count
        ));
        out.push_str(&format!(
            "- Rows: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_rows,
            self.summary.certified_row_count,
            self.summary.narrowed_row_count,
            self.summary.blocked_row_count
        ));
        out.push_str(&format!(
            "- Release gate: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));
        out.push_str("- Exposed on: Help/About, shiproom, support exports, incident/operator\n");

        out.push_str("\n## Runbook proof lanes\n\n");
        out.push_str("| Lane | Facet | Owner | Source of truth | Proof | Freshness |\n");
        out.push_str("|------|-------|-------|-----------------|-------|-----------|\n");
        for lane in &self.proof_lanes {
            out.push_str(&format!(
                "| `{}` | `{}` | {} | `{}` | `{}` | `{}` |\n",
                lane.lane.as_str(),
                lane.facet.as_str(),
                lane.owner_role,
                lane.schema_ref,
                lane.proof_ref,
                lane.proof_freshness.as_str()
            ));
        }

        out.push_str("\n## Claimed incident/operator rows\n\n");
        out.push_str("| Row | Consumer | Status | Claim → effective | Gate | Binds |\n");
        out.push_str("|-----|----------|--------|-------------------|------|-------|\n");
        for row in &self.rows {
            let bound: Vec<&str> = row.bound_lanes.iter().map(|l| l.as_str()).collect();
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` → `{}` | `{}` | {} |\n",
                row.row_id,
                row.consumer.as_str(),
                row.status.as_str(),
                row.claimed_class.as_str(),
                row.effective_class.as_str(),
                row.gate_decision.as_str(),
                bound.join(", ")
            ));
            for gap in &row.gaps {
                out.push_str(&format!(
                    "| | | | gap: `{}` on `{}` | | |\n",
                    gap.gap_kind.as_str(),
                    gap.lane.as_str()
                ));
            }
        }
        out
    }
}

/// Derives the qualification summary from the lane proofs and rows.
fn derive_summary(
    lanes: &[RunbookProofLaneContract],
    rows: &[IncidentOperatorRow],
) -> CertificationSummary {
    let lane_count = |state: ProofFreshnessState| -> u32 {
        lanes.iter().filter(|l| l.proof_freshness == state).count() as u32
    };
    let blocked = rows.iter().filter(|r| r.is_blocked()).count() as u32;
    CertificationSummary {
        total_lanes: lanes.len() as u32,
        total_rows: rows.len() as u32,
        certified_row_count: rows.iter().filter(|r| r.is_certified()).count() as u32,
        narrowed_row_count: rows.iter().filter(|r| r.is_narrowed()).count() as u32,
        blocked_row_count: blocked,
        current_lane_count: lane_count(ProofFreshnessState::Current),
        stale_lane_count: lane_count(ProofFreshnessState::Stale),
        missing_lane_count: lane_count(ProofFreshnessState::Missing),
        blocks_stable_promotion: blocked > 0,
    }
}

/// Derives the aggregate release gate from the per-row gates.
fn derive_release_gate(rows: &[IncidentOperatorRow]) -> RunbookCertificationReleaseGate {
    let pick = |f: &dyn Fn(&IncidentOperatorRow) -> bool| -> Vec<String> {
        let mut ids: Vec<String> = rows
            .iter()
            .filter(|r| f(r))
            .map(|r| r.row_id.clone())
            .collect();
        ids.sort();
        ids
    };
    let blocked = pick(&|r| r.is_blocked());
    RunbookCertificationReleaseGate {
        blocks_stable_promotion: !blocked.is_empty(),
        blocked_row_ids: blocked,
        narrowed_row_ids: pick(&|r| r.is_narrowed()),
        certified_row_ids: pick(&|r| r.is_certified()),
        gate_message_id: format!("{}release_gate", M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX),
    }
}

/// Derives the conformance review from the lane proofs and rows, so the stored block
/// reflects the actual packet rather than an assertion.
fn derive_conformance(
    lanes: &[RunbookProofLaneContract],
    rows: &[IncidentOperatorRow],
) -> CertificationConformance {
    let every_facet_covered = CertificationFacet::ALL
        .iter()
        .all(|facet| lanes.iter().any(|l| l.facet == *facet));

    let every_row_binds = !rows.is_empty() && rows.iter().all(|r| !r.bound_lanes.is_empty());

    // Every row maps to current contracts (governed) or auto-narrows/blocks via a named
    // gap — there is never a row with a stale/missing bound lane that stays governed.
    let maps_or_narrows = rows.iter().all(|r| match r.gate_decision {
        RunbookGate::Governed => r.gaps.is_empty(),
        RunbookGate::Narrowed | RunbookGate::Blocked => !r.gaps.is_empty(),
    });

    // A stale lane proof narrows every row that binds it (to at least Beta), and never
    // blocks unless a proof is missing/unmapped.
    let stale_narrows = rows.iter().all(|r| {
        let binds_stale = r.bound_lanes.iter().any(|lane| {
            lanes
                .iter()
                .find(|c| c.lane == *lane)
                .map(|c| c.proof_freshness == ProofFreshnessState::Stale)
                .unwrap_or(false)
        });
        let binds_failing = r.bound_lanes.iter().any(|lane| {
            lanes
                .iter()
                .find(|c| c.lane == *lane)
                .map(|c| c.proof_freshness == ProofFreshnessState::Missing)
                .unwrap_or(true)
        });
        // When a row binds a stale lane but no failing lane, it must auto-narrow.
        !binds_stale || binds_failing || r.is_narrowed()
    });

    // A missing/unmapped lane proof blocks every row that binds it from Stable.
    let missing_blocks = rows.iter().all(|r| {
        let binds_failing = r.bound_lanes.iter().any(|lane| {
            lanes
                .iter()
                .find(|c| c.lane == *lane)
                .map(|c| c.proof_freshness == ProofFreshnessState::Missing)
                .unwrap_or(true)
        });
        !binds_failing || r.is_blocked()
    });

    // Every gap names its lane and kind via a prefixed cause message id.
    let gaps_named = rows.iter().all(|r| {
        r.gaps.iter().all(|g| {
            g.cause_message_id
                .starts_with(M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX)
                && g.row_id == r.row_id
        })
    });

    // Every row recomputes from the same checked-in lane proofs.
    let generated_from_lanes = rows.iter().all(|r| {
        let mut probe = r.clone();
        probe.recompute(lanes);
        probe.gaps == r.gaps
            && probe.status == r.status
            && probe.gate_decision == r.gate_decision
            && probe.effective_class == r.effective_class
    });

    CertificationConformance {
        every_facet_covered_by_a_proof_lane: every_facet_covered,
        every_row_binds_at_least_one_proof_lane: every_row_binds,
        every_row_maps_to_contracts_or_narrows: maps_or_narrows,
        stale_proof_narrows_deterministically: stale_narrows,
        missing_proof_blocks_stable_promotion: missing_blocks,
        exact_gaps_named_per_row: gaps_named,
        surfaces_consume_one_qualification: true,
        generated_from_checked_in_lane_proofs: generated_from_lanes,
        export_carries_no_raw_boundary_material: true,
    }
}

/// Validation failures for the certification lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunbookCertificationViolation {
    /// The packet record kind is wrong.
    WrongRecordKind,
    /// The packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// A lane contract cites a ref that does not match its lane.
    LaneContractRefMismatch,
    /// Two lane contracts name the same lane.
    DuplicateLane,
    /// A certification facet has no governed proof lane.
    FacetNotCovered,
    /// The packet declares no claimed rows.
    PacketHasNoRows,
    /// Two rows share a row id.
    DuplicateRowId,
    /// A claimed row binds no proof lanes.
    RowBindsNoLanes,
    /// A row's stored verdict drifted from a fresh recompute.
    RowVerdictDrift,
    /// The qualification summary disagrees with the rows/lanes.
    SummaryDrift,
    /// The aggregate release gate disagrees with the per-row gates.
    ReleaseGateAggregateMismatch,
    /// A disclosure surface does not consume the qualification.
    DisclosureIncomplete,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5RunbookCertificationViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::LaneContractRefMismatch => "lane_contract_ref_mismatch",
            Self::DuplicateLane => "duplicate_lane",
            Self::FacetNotCovered => "facet_not_covered",
            Self::PacketHasNoRows => "packet_has_no_rows",
            Self::DuplicateRowId => "duplicate_row_id",
            Self::RowBindsNoLanes => "row_binds_no_lanes",
            Self::RowVerdictDrift => "row_verdict_drift",
            Self::SummaryDrift => "summary_drift",
            Self::ReleaseGateAggregateMismatch => "release_gate_aggregate_mismatch",
            Self::DisclosureIncomplete => "disclosure_incomplete",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked boundary material. Mirrors the
/// redaction posture of the source, step, handoff, and companion lanes.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized packet for forbidden boundary material. Returns true when a
/// key (case-insensitive) contains a forbidden substring.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, child)| {
            let lower = key.to_ascii_lowercase();
            FORBIDDEN_KEY_SUBSTRINGS
                .iter()
                .any(|needle| lower.contains(needle))
                || json_contains_forbidden_boundary_material(child)
        }),
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
