//! Canonical M5 graph-depth certification report: the single qualification packet that
//! graduates the workset-scope, topology-identity, impact-query, ownership-source, and
//! architecture-explainer code-understanding rows only where their evidence is current and
//! provable, and automatically narrows the rest to a smaller label before publication.
//!
//! This packet is the certification layer above the
//! [`crate::m5_graph_governance`] matrix. It does not re-derive each lane's truth — it
//! ingests the governance packet's published claim for the row
//! ([`CertificationRow::governance_claim`]), runs the per-row qualification drills
//! ([`CertificationDrill`]) the certification suite owns, scores how fresh the certification
//! evidence is ([`EvidenceFreshness`]), and publishes the certification label
//! ([`crate::m5_graph_governance::GraphDepthClaim`]) no input can exceed.
//!
//! The certification gate is non-inheriting and fail-closed. The published label is the
//! weakest ceiling implied by the governance claim, the certification evidence freshness, and
//! the drill outcomes, so a governance-narrowed row, stale or missing certification evidence,
//! or an unproven, narrowed, or failed drill all narrow or withhold the certified label
//! automatically rather than leaving a row green by inertia. A row that declared a stronger
//! label than the gate permits has its published label lowered and its
//! [`CertificationDowngradeReason`]s and [`CertificationDowngradePath`] recomputed; all are
//! validated against the gate so a downgrade can never be asserted or hidden by hand.
//!
//! Because every required consumer surface — release evidence, docs/help, onboarding, review,
//! AI context, and support export — binds to this one packet via a
//! [`CertificationConsumerBinding`] that must ingest the packet, preserve its labels, and
//! narrow with it, a row narrowed here cannot stay authoritative on a marketed row, a docs
//! badge, or a support export. Each binding is stamped with the active scope snapshot so
//! support and evidence packets can reconstruct the scope the certification answered.
//!
//! The packet is checked in at `artifacts/graph/m5/m5-graph-certification.json` and embedded
//! here. It is metadata-only: every field is a typed state, a count, or an opaque ref, and it
//! carries no credential bodies, raw provider payloads, or graph node contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_graph_governance::{GraphDepthClaim, GraphDepthLane, GraphGovernanceDecision};

/// Supported M5 graph-certification report schema version.
pub const M5_GRAPH_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_GRAPH_CERTIFICATION_RECORD_KIND: &str = "m5_graph_certification_report";

/// Repo-relative path to the checked-in packet.
pub const M5_GRAPH_CERTIFICATION_PATH: &str = "artifacts/graph/m5/m5-graph-certification.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_GRAPH_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/graph/m5-graph-certification.schema.json";

/// Repo-relative path to the companion document.
pub const M5_GRAPH_CERTIFICATION_DOC_REF: &str = "docs/graph/m5/m5-graph-certification.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_GRAPH_CERTIFICATION_ARTIFACT_DOC_REF: &str =
    "artifacts/graph/m5/m5-graph-certification.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_GRAPH_CERTIFICATION_FIXTURE_DIR: &str = "fixtures/graph/m5/m5-graph-certification";

/// Repo-relative path to the upstream graph-governance matrix this packet certifies.
pub const M5_GRAPH_CERTIFICATION_GOVERNANCE_PACKET_REF: &str =
    "artifacts/graph/m5/m5-graph-governance.json";

/// Embedded checked-in packet JSON.
pub const M5_GRAPH_CERTIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/graph/m5/m5-graph-certification.json"
));

/// A qualification drill the certification suite runs for every claimed M5 graph row.
///
/// A row is never certified above [`GraphDepthClaim::Withheld`] unless every required drill
/// ran and passed cleanly; an unproven, narrowed, or failed drill narrows or withholds the
/// certified label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDrill {
    /// Workset and sparse-scope honesty drill.
    WorksetScope,
    /// Topology node and edge identity-stability drill.
    TopologyIdentity,
    /// Impact-query result-class distinctness drill.
    ImpactQuery,
    /// Ownership-source classification drill.
    OwnershipSource,
    /// Generated-versus-curated explainer-citation drill.
    ExplainerCitation,
    /// Keyboard, list/table, and screen-reader accessibility drill.
    Accessibility,
    /// Support and evidence export-join drill.
    ExportJoin,
}

impl CertificationDrill {
    /// Every required drill, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::WorksetScope,
        Self::TopologyIdentity,
        Self::ImpactQuery,
        Self::OwnershipSource,
        Self::ExplainerCitation,
        Self::Accessibility,
        Self::ExportJoin,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorksetScope => "workset_scope",
            Self::TopologyIdentity => "topology_identity",
            Self::ImpactQuery => "impact_query",
            Self::OwnershipSource => "ownership_source",
            Self::ExplainerCitation => "explainer_citation",
            Self::Accessibility => "accessibility",
            Self::ExportJoin => "export_join",
        }
    }
}

/// The outcome of one qualification drill.
///
/// Ordered by [`DrillOutcome::claim_ceiling`]: a passed drill backs an authoritative label, a
/// narrowed drill caps at scope-qualified, and a failed or not-run drill withholds the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillOutcome {
    /// The drill ran and passed cleanly.
    Passed,
    /// The drill ran but only proved the row for a narrower slice.
    Narrowed,
    /// The drill ran and failed.
    Failed,
    /// The drill did not run; the row is unproven.
    NotRun,
}

impl DrillOutcome {
    /// Every drill outcome, in declaration order.
    pub const ALL: [Self; 4] = [Self::Passed, Self::Narrowed, Self::Failed, Self::NotRun];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Narrowed => "narrowed",
            Self::Failed => "failed",
            Self::NotRun => "not_run",
        }
    }

    /// Highest certification label this outcome permits a row to publish.
    pub const fn claim_ceiling(self) -> GraphDepthClaim {
        match self {
            Self::Passed => GraphDepthClaim::Authoritative,
            Self::Narrowed => GraphDepthClaim::ScopeQualified,
            Self::Failed | Self::NotRun => GraphDepthClaim::Withheld,
        }
    }

    /// Whether the outcome narrows the row to a slice.
    pub const fn is_narrowed(self) -> bool {
        matches!(self, Self::Narrowed)
    }

    /// Whether the outcome leaves the row unproven (failed or never run).
    pub const fn is_unproven(self) -> bool {
        matches!(self, Self::Failed | Self::NotRun)
    }

    /// Whether the drill ran at all, so it must carry an evidence ref.
    pub const fn was_run(self) -> bool {
        !matches!(self, Self::NotRun)
    }
}

/// How fresh the certification evidence backing a row is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// The certification evidence is current.
    Current,
    /// The certification evidence is aging but in tolerance; caps at scope-qualified.
    Aging,
    /// The certification evidence is expired; caps at provisional.
    Expired,
    /// The certification evidence is missing; caps at withheld.
    Missing,
}

impl EvidenceFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Aging, Self::Expired, Self::Missing];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Aging => "aging",
            Self::Expired => "expired",
            Self::Missing => "missing",
        }
    }

    /// Highest certification label this freshness state permits a row to publish.
    pub const fn claim_ceiling(self) -> GraphDepthClaim {
        match self {
            Self::Current => GraphDepthClaim::Authoritative,
            Self::Aging => GraphDepthClaim::ScopeQualified,
            Self::Expired => GraphDepthClaim::Provisional,
            Self::Missing => GraphDepthClaim::Withheld,
        }
    }

    /// Whether this state raises the [`CertificationDowngradeReason::EvidenceStale`] trigger.
    pub const fn is_stale_trigger(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// A headline reason the certification gate narrows a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDowngradeReason {
    /// The upstream governance matrix already narrowed the row below authoritative.
    GovernanceNarrowed,
    /// The certification evidence is aging, expired, or missing.
    EvidenceStale,
    /// At least one qualification drill proved the row only for a narrower slice.
    DrillNarrowed,
    /// At least one qualification drill failed or never ran.
    DrillFailed,
}

impl CertificationDowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::GovernanceNarrowed,
        Self::EvidenceStale,
        Self::DrillNarrowed,
        Self::DrillFailed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GovernanceNarrowed => "governance_narrowed",
            Self::EvidenceStale => "evidence_stale",
            Self::DrillNarrowed => "drill_narrowed",
            Self::DrillFailed => "drill_failed",
        }
    }
}

/// The exact recovery path surfaced when a row's certified label is narrowed or withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDowngradePath {
    /// Rerun the failed, not-run, or narrowed qualification drills.
    RerunDrills,
    /// Refresh the aging, expired, or missing certification evidence.
    RefreshEvidence,
    /// Adopt the governance matrix's narrowing rather than re-asserting a broader label.
    AdoptGovernanceNarrowing,
    /// Withhold the row from publication.
    WithholdRow,
    /// No downgrade is needed; only valid when the row is certified authoritative.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl CertificationDowngradePath {
    /// Every downgrade path, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RerunDrills,
        Self::RefreshEvidence,
        Self::AdoptGovernanceNarrowing,
        Self::WithholdRow,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RerunDrills => "rerun_drills",
            Self::RefreshEvidence => "refresh_evidence",
            Self::AdoptGovernanceNarrowing => "adopt_governance_narrowing",
            Self::WithholdRow => "withhold_row",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real recovery path the row owner can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// A downstream surface that must ingest this certification packet and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationConsumerSurface {
    /// Release evidence and proof index.
    ReleaseEvidence,
    /// Docs and help/service-health surface.
    DocsHelp,
    /// Onboarding tours and first-run context.
    Onboarding,
    /// Review explanation surface.
    Review,
    /// AI context-assembly surface.
    AiContext,
    /// Support export bundle.
    SupportExport,
}

impl CertificationConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::ReleaseEvidence,
        Self::DocsHelp,
        Self::Onboarding,
        Self::Review,
        Self::AiContext,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseEvidence => "release_evidence",
            Self::DocsHelp => "docs_help",
            Self::Onboarding => "onboarding",
            Self::Review => "review",
            Self::AiContext => "ai_context",
            Self::SupportExport => "support_export",
        }
    }
}

/// The outcome of one qualification drill, with its evidence ref and capture time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationDrillResult {
    /// Drill this result records.
    pub drill: CertificationDrill,
    /// Outcome of the drill.
    pub outcome: DrillOutcome,
    /// Ref to the drill's evidence; required whenever the drill ran.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    /// Capture timestamp for the drill run.
    pub checked_at: String,
}

impl CertificationDrillResult {
    /// Whether the result carries the evidence ref its outcome requires.
    pub fn has_required_evidence(&self) -> bool {
        if self.outcome.was_run() {
            self.evidence_ref
                .as_ref()
                .is_some_and(|r| !r.trim().is_empty())
        } else {
            true
        }
    }
}

/// One certification row for a claimed M5 graph code-understanding subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationRow {
    /// Stable certification-row id.
    pub row_id: String,
    /// Graph code-understanding subject this row certifies.
    pub subject: GraphDepthLane,
    /// Owner accountable for the row's evidence and conformance.
    pub owner: String,
    /// Depth claim the upstream governance matrix published for this subject.
    ///
    /// The certification gate can only narrow from here; it never re-broadens a
    /// governance-narrowed row.
    pub governance_claim: GraphDepthClaim,
    /// How fresh the certification evidence backing this row is.
    pub evidence_freshness: EvidenceFreshness,
    /// Per-drill outcomes; one result per required drill.
    #[serde(default)]
    pub drill_results: Vec<CertificationDrillResult>,
    /// Label the row's own evidence asserts, before the gate.
    pub declared_label: GraphDepthClaim,
    /// Label actually published after the gate narrows the row.
    ///
    /// Must equal [`CertificationRow::effective_label`].
    pub published_label: GraphDepthClaim,
    /// Decision the gate takes; must equal the recomputed decision.
    pub certification_decision: GraphGovernanceDecision,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<CertificationDowngradeReason>,
    /// Recovery path surfaced when the label is narrowed or withheld.
    pub downgrade_path: CertificationDowngradePath,
    /// Profiles or slices this row still certifies.
    #[serde(default)]
    pub supported_profiles: Vec<String>,
    /// Caveats attached to the published label.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale, missing, or narrowing the label.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Ref to the upstream governance packet this row certifies.
    pub governance_packet_ref: String,
    /// Ref to the governance row this certification row narrows from.
    pub governance_row_ref: String,
    /// Ref to the graph-conformance suite backing the row.
    pub conformance_ref: String,
    /// Ref to the row's supporting evidence.
    pub evidence_ref: String,
    /// Active scope snapshot the certification answered, stamped for replay.
    pub scope_snapshot_ref: String,
    /// Ref to the machine-readable certification receipt.
    pub certification_receipt_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl CertificationRow {
    /// The label the row's own evidence asserted, before gate narrowing.
    pub fn capability_floor(&self) -> GraphDepthClaim {
        self.declared_label
    }

    /// Highest label the drills permit, the weakest ceiling across every required drill.
    ///
    /// A missing required drill caps the row at withheld, so an incompletely drilled row can
    /// never read as certified.
    pub fn drill_ceiling(&self) -> GraphDepthClaim {
        let mut ceiling = GraphDepthClaim::Authoritative;
        for drill in CertificationDrill::ALL {
            let outcome = self
                .drill_results
                .iter()
                .find(|r| r.drill == drill)
                .map(|r| r.outcome.claim_ceiling())
                .unwrap_or(GraphDepthClaim::Withheld);
            ceiling = ceiling.min(outcome);
        }
        ceiling
    }

    /// The label the gate permits this row to publish.
    ///
    /// Lowers the declared label to the weakest ceiling implied by the governance claim, the
    /// certification evidence freshness, and the drill outcomes, so a governance-narrowed row,
    /// stale evidence, or an unproven, narrowed, or failed drill can never publish an
    /// authoritative label.
    pub fn effective_label(&self) -> GraphDepthClaim {
        self.capability_floor()
            .min(self.governance_claim)
            .min(self.evidence_freshness.claim_ceiling())
            .min(self.drill_ceiling())
    }

    /// Whether any required drill proved the row only for a narrower slice.
    pub fn has_narrowed_drill(&self) -> bool {
        self.drill_results.iter().any(|r| r.outcome.is_narrowed())
    }

    /// Whether any required drill failed or never ran.
    pub fn has_unproven_drill(&self) -> bool {
        CertificationDrill::ALL.iter().any(|&drill| {
            self.drill_results
                .iter()
                .find(|r| r.drill == drill)
                .map(|r| r.outcome.is_unproven())
                .unwrap_or(true)
        })
    }

    /// The headline downgrade reasons recomputed from the row's observed states.
    pub fn computed_downgrade_reasons(&self) -> Vec<CertificationDowngradeReason> {
        let mut reasons = Vec::new();
        if self.governance_claim.rank() < GraphDepthClaim::Authoritative.rank() {
            reasons.push(CertificationDowngradeReason::GovernanceNarrowed);
        }
        if self.evidence_freshness.is_stale_trigger() {
            reasons.push(CertificationDowngradeReason::EvidenceStale);
        }
        if self.has_narrowed_drill() {
            reasons.push(CertificationDowngradeReason::DrillNarrowed);
        }
        if self.has_unproven_drill() {
            reasons.push(CertificationDowngradeReason::DrillFailed);
        }
        reasons
    }

    /// The recovery path the gate must record, derived from the row's observed states.
    ///
    /// Ordered by severity: a withheld row points at withhold, an unproven or narrowed drill
    /// points at a drill rerun, stale evidence points at a refresh, a governance-only
    /// narrowing points at adopting that narrowing, and a clean row needs nothing.
    pub fn computed_downgrade_path(&self) -> CertificationDowngradePath {
        if self.effective_label() == GraphDepthClaim::Withheld {
            CertificationDowngradePath::WithholdRow
        } else if self.has_unproven_drill() || self.has_narrowed_drill() {
            CertificationDowngradePath::RerunDrills
        } else if self.evidence_freshness.is_stale_trigger() {
            CertificationDowngradePath::RefreshEvidence
        } else if self.governance_claim.rank() < GraphDepthClaim::Authoritative.rank() {
            CertificationDowngradePath::AdoptGovernanceNarrowing
        } else {
            CertificationDowngradePath::NoneNeeded
        }
    }

    /// The decision the gate must record, derived from the effective label.
    pub fn required_decision(&self) -> GraphGovernanceDecision {
        GraphGovernanceDecision::for_claim(self.effective_label())
    }

    /// Whether the row publishes a clean authoritative certification.
    pub fn is_certified(&self) -> bool {
        self.effective_label() == GraphDepthClaim::Authoritative
    }

    /// Whether the gate narrowed the published label below what the row declared.
    pub fn is_downgraded(&self) -> bool {
        self.effective_label().rank() < self.capability_floor().rank()
    }

    /// Whether the row covers every required drill exactly once.
    pub fn covers_all_drills(&self) -> bool {
        let mut seen = BTreeSet::new();
        for result in &self.drill_results {
            seen.insert(result.drill);
        }
        CertificationDrill::ALL.iter().all(|d| seen.contains(d))
            && self.drill_results.len() == CertificationDrill::ALL.len()
    }

    /// Whether the row carries its own non-empty governance, conformance, evidence, scope,
    /// and receipt refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.governance_packet_ref.trim().is_empty()
            && !self.governance_row_ref.trim().is_empty()
            && !self.conformance_ref.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
            && !self.certification_receipt_ref.trim().is_empty()
    }

    /// Whether the stored published label, decision, reasons, and path all agree with the
    /// recomputed gate.
    pub fn gate_consistent(&self) -> bool {
        self.published_label == self.effective_label()
            && self.certification_decision == self.required_decision()
            && self.downgrade_reasons == self.computed_downgrade_reasons()
            && self.downgrade_path == self.computed_downgrade_path()
    }
}

/// One binding wiring a downstream surface to this certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer_surface: CertificationConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Certification packet id this surface ingests.
    pub certification_packet_id_ref: String,
    /// Active scope snapshot stamped on the binding for replay.
    pub scope_snapshot_ref: String,
    /// True when the surface ingests this certification packet rather than a parallel sheet.
    pub ingests_certification_packet: bool,
    /// True when the surface preserves the published labels verbatim.
    pub preserves_published_labels: bool,
    /// True when the surface preserves the recovery paths verbatim.
    pub preserves_downgrade_paths: bool,
    /// True when the surface narrows automatically as rows are downgraded.
    pub narrows_on_downgrade: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl CertificationConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.certification_packet_id_ref == packet_id
            && self.ingests_certification_packet
            && self.preserves_published_labels
            && self.preserves_downgrade_paths
            && self.narrows_on_downgrade
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5GraphCertificationSummary {
    /// Total certification rows.
    pub total_rows: usize,
    /// Number of claimed subjects.
    pub subject_count: usize,
    /// Rows certified authoritative.
    pub certified_rows: usize,
    /// Rows narrowed to a scope-qualified label.
    pub scope_qualified_rows: usize,
    /// Rows narrowed to a provisional label.
    pub provisional_rows: usize,
    /// Rows withheld from publication.
    pub withheld_rows: usize,
    /// Rows whose published label was downgraded below what they declared.
    pub downgraded_rows: usize,
    /// Rows carrying at least one downgrade reason.
    pub rows_with_downgrade_reasons: usize,
    /// Rows whose certification evidence is aging, expired, or missing.
    pub stale_evidence_rows: usize,
    /// Rows with at least one narrowed, failed, or not-run drill.
    pub rows_with_imperfect_drills: usize,
}

/// A redaction-safe export row projected from a certification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GraphCertificationExportRow {
    /// Certification-row id.
    pub row_id: String,
    /// Subject token.
    pub subject: String,
    /// Owner accountable for the row.
    pub owner: String,
    /// Governance-claim token the row narrows from.
    pub governance_claim: String,
    /// Evidence-freshness token.
    pub evidence_freshness: String,
    /// Declared-label token.
    pub declared_label: String,
    /// Published-label token.
    pub published_label: String,
    /// Certification-decision token.
    pub certification_decision: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Downgrade-path token.
    pub downgrade_path: String,
    /// Supported profiles or slices.
    pub supported_profiles: Vec<String>,
    /// Caveats attached to the published label.
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale or missing.
    pub stale_or_missing_fields: Vec<String>,
    /// Governance-packet ref this row certifies.
    pub governance_packet_ref: String,
    /// Scope snapshot the certification answered.
    pub scope_snapshot_ref: String,
    /// Certification-receipt ref.
    pub certification_receipt_ref: String,
    /// Whether the row publishes an authoritative certification.
    pub certified: bool,
    /// Whether the published label was downgraded below the declared label.
    pub downgraded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet — the canonical certification index
/// downstream surfaces render instead of restating each row's label by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GraphCertificationExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5GraphCertificationExportRow>,
    /// Whether every row's published label and decision agree with the gate.
    pub all_rows_gate_consistent: bool,
    /// Rows that publish an authoritative certification.
    pub certified_count: usize,
    /// Rows the gate narrowed or withheld.
    pub narrowed_count: usize,
    /// Rows the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed M5 graph-depth certification report packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5GraphCertificationReport {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the upstream graph-governance matrix this report certifies.
    pub governance_packet_ref: String,
    /// Claimed subjects; one row per subject.
    pub subjects: Vec<GraphDepthLane>,
    /// Closed certification-label vocabulary.
    pub certification_labels: Vec<GraphDepthClaim>,
    /// Closed drill vocabulary.
    pub drills: Vec<CertificationDrill>,
    /// Closed drill-outcome vocabulary.
    pub drill_outcomes: Vec<DrillOutcome>,
    /// Closed evidence-freshness vocabulary.
    pub evidence_freshness_states: Vec<EvidenceFreshness>,
    /// Closed certification-decision vocabulary.
    pub certification_decisions: Vec<GraphGovernanceDecision>,
    /// Closed downgrade-path vocabulary.
    pub downgrade_paths: Vec<CertificationDowngradePath>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<CertificationDowngradeReason>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<CertificationConsumerSurface>,
    /// Certification rows, one per claimed subject.
    #[serde(default)]
    pub rows: Vec<CertificationRow>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<CertificationConsumerBinding>,
    /// Summary counts.
    pub summary: M5GraphCertificationSummary,
}

impl M5GraphCertificationReport {
    /// Returns the row for a claimed subject.
    pub fn row(&self, subject: GraphDepthLane) -> Option<&CertificationRow> {
        self.rows.iter().find(|r| r.subject == subject)
    }

    /// Rows that publish an authoritative certification.
    pub fn certified_rows(&self) -> impl Iterator<Item = &CertificationRow> {
        self.rows.iter().filter(|r| r.is_certified())
    }

    /// Rows the gate narrowed or withheld in any way.
    pub fn narrowed_rows(&self) -> impl Iterator<Item = &CertificationRow> {
        self.rows
            .iter()
            .filter(|r| r.required_decision().is_narrowed())
    }

    /// Rows the gate withheld entirely.
    pub fn withheld_rows(&self) -> impl Iterator<Item = &CertificationRow> {
        self.rows
            .iter()
            .filter(|r| r.required_decision() == GraphGovernanceDecision::Withhold)
    }

    /// Whether a consumer binding preserves this packet for the given surface.
    pub fn has_binding_for(&self, surface: CertificationConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every row's stored published label, decision, reasons, and path agree with the
    /// recomputed gate.
    pub fn all_rows_gate_consistent(&self) -> bool {
        self.rows.iter().all(|r| r.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5GraphCertificationSummary {
        let count_published = |label: GraphDepthClaim| {
            self.rows
                .iter()
                .filter(|r| r.published_label == label)
                .count()
        };
        M5GraphCertificationSummary {
            total_rows: self.rows.len(),
            subject_count: self.subjects.len(),
            certified_rows: count_published(GraphDepthClaim::Authoritative),
            scope_qualified_rows: count_published(GraphDepthClaim::ScopeQualified),
            provisional_rows: count_published(GraphDepthClaim::Provisional),
            withheld_rows: count_published(GraphDepthClaim::Withheld),
            downgraded_rows: self.rows.iter().filter(|r| r.is_downgraded()).count(),
            rows_with_downgrade_reasons: self
                .rows
                .iter()
                .filter(|r| !r.downgrade_reasons.is_empty())
                .count(),
            stale_evidence_rows: self
                .rows
                .iter()
                .filter(|r| r.evidence_freshness.is_stale_trigger())
                .count(),
            rows_with_imperfect_drills: self
                .rows
                .iter()
                .filter(|r| r.has_narrowed_drill() || r.has_unproven_drill())
                .count(),
        }
    }

    /// Produces the certification index downstream surfaces — release evidence, docs/help,
    /// onboarding, review, AI context, and support exports — render instead of restating each
    /// row's certification by hand.
    pub fn export_projection(&self) -> M5GraphCertificationExportProjection {
        let rows = self
            .rows
            .iter()
            .map(|r| M5GraphCertificationExportRow {
                row_id: r.row_id.clone(),
                subject: r.subject.as_str().to_owned(),
                owner: r.owner.clone(),
                governance_claim: r.governance_claim.as_str().to_owned(),
                evidence_freshness: r.evidence_freshness.as_str().to_owned(),
                declared_label: r.declared_label.as_str().to_owned(),
                published_label: r.published_label.as_str().to_owned(),
                certification_decision: r.certification_decision.as_str().to_owned(),
                downgrade_reasons: r
                    .downgrade_reasons
                    .iter()
                    .map(|x| x.as_str().to_owned())
                    .collect(),
                downgrade_path: r.downgrade_path.as_str().to_owned(),
                supported_profiles: r.supported_profiles.clone(),
                caveats: r.caveats.clone(),
                stale_or_missing_fields: r.stale_or_missing_fields.clone(),
                governance_packet_ref: r.governance_packet_ref.clone(),
                scope_snapshot_ref: r.scope_snapshot_ref.clone(),
                certification_receipt_ref: r.certification_receipt_ref.clone(),
                certified: r.is_certified(),
                downgraded: r.is_downgraded(),
                summary: format!(
                    "{}: governance {}, evidence {}, declared {}, published {} ({}), recovery {}",
                    r.subject.as_str(),
                    r.governance_claim.as_str(),
                    r.evidence_freshness.as_str(),
                    r.declared_label.as_str(),
                    r.published_label.as_str(),
                    r.certification_decision.as_str(),
                    r.downgrade_path.as_str()
                ),
            })
            .collect();
        M5GraphCertificationExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_rows_gate_consistent: self.all_rows_gate_consistent(),
            certified_count: self.certified_rows().count(),
            narrowed_count: self.narrowed_rows().count(),
            withheld_count: self.withheld_rows().count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact certification report.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5GraphCertificationSupportExport {
        M5GraphCertificationSupportExport {
            record_kind: M5_GRAPH_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_GRAPH_CERTIFICATION_SCHEMA_VERSION,
            export_id: export_id.into(),
            certification_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            certification_report: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5GraphCertificationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<GraphDepthLane> = self.subjects.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_subjects = BTreeSet::new();
        for row in &self.rows {
            if !seen_ids.insert(row.row_id.clone()) {
                violations.push(M5GraphCertificationViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            if !seen_subjects.insert(row.subject) {
                violations.push(M5GraphCertificationViolation::DuplicateSubjectRow {
                    subject: row.subject.as_str(),
                });
            }
            if !claimed.contains(&row.subject) {
                violations.push(M5GraphCertificationViolation::UnclaimedSubjectRow {
                    row_id: row.row_id.clone(),
                    subject: row.subject.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed subject must carry its own row, so a subject never inherits a
        // certification from an adjacent one.
        for &subject in &self.subjects {
            if !seen_subjects.contains(&subject) {
                violations.push(M5GraphCertificationViolation::MissingSubjectRow {
                    subject: subject.as_str(),
                });
            }
        }

        // Every required consumer surface must bind to this packet and narrow with it, so a
        // narrowed row cannot stay green on a downstream surface by inertia.
        for surface in CertificationConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(M5GraphCertificationViolation::MissingConsumerBinding {
                    surface: surface.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5GraphCertificationViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5GraphCertificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5GraphCertificationViolation>) {
        if self.schema_version != M5_GRAPH_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5GraphCertificationViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_GRAPH_CERTIFICATION_RECORD_KIND {
            violations.push(M5GraphCertificationViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("governance_packet_ref", &self.governance_packet_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5GraphCertificationViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.governance_packet_ref != M5_GRAPH_CERTIFICATION_GOVERNANCE_PACKET_REF {
            violations.push(M5GraphCertificationViolation::GovernancePacketMismatch {
                expected: M5_GRAPH_CERTIFICATION_GOVERNANCE_PACKET_REF,
            });
        }
        for (field, ok) in [
            ("subjects", self.subjects == GraphDepthLane::ALL.to_vec()),
            (
                "certification_labels",
                self.certification_labels == GraphDepthClaim::ALL.to_vec(),
            ),
            ("drills", self.drills == CertificationDrill::ALL.to_vec()),
            (
                "drill_outcomes",
                self.drill_outcomes == DrillOutcome::ALL.to_vec(),
            ),
            (
                "evidence_freshness_states",
                self.evidence_freshness_states == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "certification_decisions",
                self.certification_decisions == GraphGovernanceDecision::ALL.to_vec(),
            ),
            (
                "downgrade_paths",
                self.downgrade_paths == CertificationDowngradePath::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == CertificationDowngradeReason::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == CertificationConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5GraphCertificationViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &CertificationRow,
        violations: &mut Vec<M5GraphCertificationViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("owner", &row.owner),
            ("governance_packet_ref", &row.governance_packet_ref),
            ("governance_row_ref", &row.governance_row_ref),
            ("conformance_ref", &row.conformance_ref),
            ("evidence_ref", &row.evidence_ref),
            ("scope_snapshot_ref", &row.scope_snapshot_ref),
            ("certification_receipt_ref", &row.certification_receipt_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5GraphCertificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        // The row must certify the canonical governance packet, so a certification never
        // narrows from a packet other than the matrix it claims to gate.
        if row.governance_packet_ref != M5_GRAPH_CERTIFICATION_GOVERNANCE_PACKET_REF {
            violations.push(M5GraphCertificationViolation::GovernancePacketMismatch {
                expected: M5_GRAPH_CERTIFICATION_GOVERNANCE_PACKET_REF,
            });
        }

        // The row must cover every required drill exactly once, so an incompletely drilled row
        // is never certified by omission.
        if !row.covers_all_drills() {
            violations.push(M5GraphCertificationViolation::IncompleteDrillCoverage {
                row_id: row.row_id.clone(),
            });
        }
        for result in &row.drill_results {
            if result.checked_at.trim().is_empty() {
                violations.push(M5GraphCertificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "drill_results.checked_at",
                });
            }
            if !result.has_required_evidence() {
                violations.push(M5GraphCertificationViolation::DrillMissingEvidence {
                    row_id: row.row_id.clone(),
                    drill: result.drill.as_str(),
                });
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5GraphCertificationViolation::DuplicateDowngradeReason {
                    row_id: row.row_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published label must equal the gate's recomputed ceiling, so a
        // governance-narrowed, stale, or under-drilled row can never read as certified.
        let effective = row.effective_label();
        if row.published_label != effective {
            violations.push(M5GraphCertificationViolation::OverstatedLabel {
                row_id: row.row_id.clone(),
                published: row.published_label.as_str(),
                computed: effective.as_str(),
            });
        }

        // The published label may never exceed the governance claim, the cornerstone of the
        // non-inheritance guarantee: a certification never re-broadens a governance-narrowed
        // row.
        if row.published_label.rank() > row.governance_claim.rank() {
            violations.push(M5GraphCertificationViolation::ExceedsGovernance {
                row_id: row.row_id.clone(),
                published: row.published_label.as_str(),
                governance: row.governance_claim.as_str(),
            });
        }

        let required = row.required_decision();
        if row.certification_decision != required {
            violations.push(M5GraphCertificationViolation::DecisionMismatch {
                row_id: row.row_id.clone(),
                declared: row.certification_decision.as_str(),
                required: required.as_str(),
            });
        }

        let computed = row.computed_downgrade_reasons();
        if row.downgrade_reasons != computed {
            violations.push(M5GraphCertificationViolation::DowngradeReasonsMismatch {
                row_id: row.row_id.clone(),
            });
        }

        let computed_path = row.computed_downgrade_path();
        if row.downgrade_path != computed_path {
            violations.push(M5GraphCertificationViolation::DowngradePathMismatch {
                row_id: row.row_id.clone(),
                declared: row.downgrade_path.as_str(),
                required: computed_path.as_str(),
            });
        }

        // A narrowed or withheld row must offer a real recovery path, list a caveat, and name
        // what is stale, so a degraded row never drops its recovery semantics or hides why it
        // narrowed.
        if row.certification_decision.is_narrowed() {
            if !row.downgrade_path.is_offered() {
                violations.push(M5GraphCertificationViolation::MissingDowngradePath {
                    row_id: row.row_id.clone(),
                });
            }
            if row.caveats.is_empty() {
                violations.push(M5GraphCertificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "caveats",
                });
            }
            if row.stale_or_missing_fields.is_empty() {
                violations.push(M5GraphCertificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "stale_or_missing_fields",
                });
            }
        }

        // A row that still certifies a publishable label must name at least one supported
        // profile or slice.
        if row.published_label != GraphDepthClaim::Withheld && row.supported_profiles.is_empty() {
            violations.push(M5GraphCertificationViolation::EmptyField {
                id: row.row_id.clone(),
                field_name: "supported_profiles",
            });
        }

        // A certified row must be genuinely whole-provable: the governance claim is
        // authoritative, the evidence is current, every drill passed, the declared label is
        // authoritative, and nothing narrows it. This is the guardrail against a blanket
        // 'codebase understanding complete' badge over an unproven row.
        if row.is_certified()
            && (row.governance_claim != GraphDepthClaim::Authoritative
                || row.evidence_freshness != EvidenceFreshness::Current
                || row.drill_ceiling() != GraphDepthClaim::Authoritative
                || row.capability_floor() != GraphDepthClaim::Authoritative
                || !row.downgrade_reasons.is_empty()
                || !row.caveats.is_empty()
                || !row.stale_or_missing_fields.is_empty()
                || row.downgrade_path.is_offered())
        {
            violations.push(M5GraphCertificationViolation::CertifiedRowNotWhole {
                row_id: row.row_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5 graph-certification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5GraphCertificationViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A certification-row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A claimed subject carries more than one row.
    DuplicateSubjectRow {
        /// Subject token.
        subject: &'static str,
    },
    /// A claimed subject has no row.
    MissingSubjectRow {
        /// Subject token.
        subject: &'static str,
    },
    /// A row covers a subject the packet does not claim.
    UnclaimedSubjectRow {
        /// Row id.
        row_id: String,
        /// Subject token.
        subject: &'static str,
    },
    /// A row or the packet binds to a governance packet other than the canonical one.
    GovernancePacketMismatch {
        /// Expected governance-packet path.
        expected: &'static str,
    },
    /// A row does not cover every required drill exactly once.
    IncompleteDrillCoverage {
        /// Row id.
        row_id: String,
    },
    /// A drill that ran carries no evidence ref.
    DrillMissingEvidence {
        /// Row id.
        row_id: String,
        /// Drill token.
        drill: &'static str,
    },
    /// A row lists a downgrade reason more than once.
    DuplicateDowngradeReason {
        /// Row id.
        row_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A row publishes a label beyond what the gate computes.
    OverstatedLabel {
        /// Row id.
        row_id: String,
        /// Published label token.
        published: &'static str,
        /// Computed effective label token.
        computed: &'static str,
    },
    /// A row publishes a label above the upstream governance claim.
    ExceedsGovernance {
        /// Row id.
        row_id: String,
        /// Published label token.
        published: &'static str,
        /// Governance claim token.
        governance: &'static str,
    },
    /// A row's decision disagrees with its gate decision.
    DecisionMismatch {
        /// Row id.
        row_id: String,
        /// Declared decision token.
        declared: &'static str,
        /// Required decision token.
        required: &'static str,
    },
    /// A row's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Row id.
        row_id: String,
    },
    /// A row's downgrade path disagrees with the recomputed path.
    DowngradePathMismatch {
        /// Row id.
        row_id: String,
        /// Declared path token.
        declared: &'static str,
        /// Required path token.
        required: &'static str,
    },
    /// A narrowed or withheld row offers no recovery path.
    MissingDowngradePath {
        /// Row id.
        row_id: String,
    },
    /// A certified row still narrows a state or carries a downgrade reason.
    CertifiedRowNotWhole {
        /// Row id.
        row_id: String,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer binding drops or remints certification truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5GraphCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => write!(f, "duplicate row id {row_id}"),
            Self::DuplicateSubjectRow { subject } => {
                write!(f, "duplicate row for subject {subject}")
            }
            Self::MissingSubjectRow { subject } => {
                write!(f, "missing row for claimed subject {subject}")
            }
            Self::UnclaimedSubjectRow { row_id, subject } => {
                write!(f, "row {row_id} covers unclaimed subject {subject}")
            }
            Self::GovernancePacketMismatch { expected } => {
                write!(
                    f,
                    "governance_packet_ref must be the canonical governance packet {expected}"
                )
            }
            Self::IncompleteDrillCoverage { row_id } => {
                write!(f, "row {row_id} does not cover every required drill once")
            }
            Self::DrillMissingEvidence { row_id, drill } => {
                write!(f, "row {row_id} drill {drill} ran without an evidence ref")
            }
            Self::DuplicateDowngradeReason { row_id, reason } => {
                write!(f, "row {row_id} repeats downgrade reason {reason}")
            }
            Self::OverstatedLabel {
                row_id,
                published,
                computed,
            } => write!(
                f,
                "row {row_id} publishes label {published} but the gate computes {computed}"
            ),
            Self::ExceedsGovernance {
                row_id,
                published,
                governance,
            } => write!(
                f,
                "row {row_id} publishes label {published} above governance claim {governance}"
            ),
            Self::DecisionMismatch {
                row_id,
                declared,
                required,
            } => write!(
                f,
                "row {row_id} records decision {declared} but the gate requires {required}"
            ),
            Self::DowngradeReasonsMismatch { row_id } => {
                write!(f, "row {row_id} downgrade reasons disagree with the gate")
            }
            Self::DowngradePathMismatch {
                row_id,
                declared,
                required,
            } => write!(
                f,
                "row {row_id} records recovery {declared} but the gate requires {required}"
            ),
            Self::MissingDowngradePath { row_id } => {
                write!(
                    f,
                    "row {row_id} is narrowed or withheld but offers no recovery path"
                )
            }
            Self::CertifiedRowNotWhole { row_id } => {
                write!(
                    f,
                    "row {row_id} is certified but narrows a state or carries a downgrade reason"
                )
            }
            Self::MissingConsumerBinding { surface } => {
                write!(f, "missing consumer binding for surface {surface}")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(
                    f,
                    "binding {binding_ref} does not preserve certification truth"
                )
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the rows"),
        }
    }
}

impl Error for M5GraphCertificationViolation {}

/// Stable record-kind tag for [`M5GraphCertificationSupportExport`].
pub const M5_GRAPH_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_graph_certification_support_export";

/// Support-export wrapper preserving the certification report verbatim for support and
/// evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GraphCertificationSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub certification_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact certification report preserved by the export.
    pub certification_report: M5GraphCertificationReport,
}

impl M5GraphCertificationSupportExport {
    /// Whether the export preserves the same packet id and a clean report.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_GRAPH_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_GRAPH_CERTIFICATION_SCHEMA_VERSION
            && self.certification_packet_id_ref == self.certification_report.packet_id
            && self.raw_private_material_excluded
            && self.certification_report.validate().is_empty()
    }
}

/// Loads the embedded M5 graph-certification report packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5GraphCertificationReport`].
pub fn current_m5_graph_certification_report(
) -> Result<M5GraphCertificationReport, serde_json::Error> {
    serde_json::from_str(M5_GRAPH_CERTIFICATION_JSON)
}

#[cfg(test)]
mod tests;
