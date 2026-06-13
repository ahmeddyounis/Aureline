//! Canonical certification packet for the M5 record-governance lane.
//!
//! This module aggregates the five lane proofs — record class, hold/delete
//! honesty, chronology, policy simulation, and exception/expiry — into one
//! per-family certification verdict. Each governed M5 managed/enterprise family
//! carries a proof cell per dimension; a family certifies only when every cell
//! is observed and current. Any missing, stale, or unproven managed-control cell
//! auto-narrows the family's published claim and, when the family is
//! release-blocking, holds the shiproom promotion gate.
//!
//! The packet is the canonical shiproom and public-claim proof for the lane:
//! shiproom and public surfaces ingest [`M5RecordsPolicyCertificationPacket::shiproom_projection`]
//! and [`M5RecordsPolicyCertificationPacket::public_claim_projection`] rather
//! than cloning their own status text. The packet is metadata-only and carries
//! no credential bodies, raw provider payloads, or durable content.
//!
//! The certification is bound to live truth: it is seeded from the checked-in
//! [`records_policy_simulation_matrix`](crate::records_policy_simulation_matrix)
//! and [`M5RecordsPolicyCertificationPacket::verify_against_live_packets`]
//! cross-checks the same-crate hold/retention and policy-simulation packets so a
//! regression in any upstream packet narrows the affected claim.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::m5_policy_simulation::{
    seeded_m5_policy_simulation_packet, M5_POLICY_SIMULATION_ARTIFACT_REF,
    M5_POLICY_SIMULATION_RECORD_KIND, M5_POLICY_SIMULATION_SHARED_CONTRACT_REF,
};
use crate::m5_records_policy::{
    seeded_m5_records_policy_packet, M5_RECORDS_POLICY_ARTIFACT_REF, M5_RECORDS_POLICY_RECORD_KIND,
    M5_RECORDS_POLICY_SHARED_CONTRACT_REF,
};
use crate::records_policy_simulation_matrix::{
    current_records_policy_matrix, AuthorityBoundaryClass, GovernedArtifactFamily,
    ProofFreshnessClass, RecordsPolicySimulationMatrix,
};
use crate::{current_registry, RecordClassId};

#[cfg(test)]
mod tests;

/// Supported schema version for the certification packet.
pub const M5_RECORDS_POLICY_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for the certification packet.
pub const M5_RECORDS_POLICY_CERTIFICATION_RECORD_KIND: &str =
    "m5_records_policy_certification_packet";

/// Shared contract reference for the certification packet.
pub const M5_RECORDS_POLICY_CERTIFICATION_SHARED_CONTRACT_REF: &str =
    "governance:m5_records_policy_certification:v1";

/// Repo-relative human-readable companion document.
pub const M5_RECORDS_POLICY_CERTIFICATION_DOC_REF: &str =
    "docs/governance/m5_records_policy_certification.md";

/// Repo-relative artifact summary for the certification packet.
pub const M5_RECORDS_POLICY_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/governance/m5_records_policy_certification.md";

/// Repo-relative JSON Schema for the certification packet.
pub const M5_RECORDS_POLICY_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/governance/m5_records_policy_certification.schema.json";

/// Repo-relative fixture directory for the certification packet.
pub const M5_RECORDS_POLICY_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/governance/m5_records_policy_certification";

/// Pinned record kind of the upstream chronology packet (in `aureline-chronology`).
pub const M5_CERT_CHRONOLOGY_PACKET_RECORD_KIND: &str = "m5_evidence_chronology_packet";

/// Pinned shared-contract reference of the upstream chronology packet.
pub const M5_CERT_CHRONOLOGY_CONTRACT_REF: &str = "chronology:m5_evidence_time_lineage:v1";

/// Pinned artifact summary of the upstream chronology packet.
pub const M5_CERT_CHRONOLOGY_ARTIFACT_REF: &str =
    "artifacts/governance/m5_evidence_chronology_lineage.md";

/// Pinned record kind of the upstream exception/expiry packet (in `aureline-policy`).
pub const M5_CERT_EXCEPTION_PACKET_RECORD_KIND: &str = "m5_exception_expiry_packet";

/// Pinned shared-contract reference of the upstream exception/expiry packet.
pub const M5_CERT_EXCEPTION_CONTRACT_REF: &str = "policy:m5_exception_expiry_truth:v1";

/// Pinned artifact summary of the upstream exception/expiry packet.
pub const M5_CERT_EXCEPTION_ARTIFACT_REF: &str = "artifacts/governance/m5_exception_expiry.md";

/// One lane proof dimension a governed family must satisfy to certify.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationProofDimension {
    /// Governed record-class descriptor proof.
    RecordClass,
    /// Legal-hold and delete/export honesty proof.
    HoldDelete,
    /// Timezone-aware chronology and lineage proof.
    Chronology,
    /// Pre-apply policy-simulation proof.
    PolicySimulation,
    /// Time-bounded exception and remembered-decision revalidation proof.
    ExceptionExpiry,
}

impl CertificationProofDimension {
    /// Every proof dimension in canonical order.
    pub const ALL: [Self; 5] = [
        Self::RecordClass,
        Self::HoldDelete,
        Self::Chronology,
        Self::PolicySimulation,
        Self::ExceptionExpiry,
    ];

    /// Returns the stable token for this dimension.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordClass => "record_class",
            Self::HoldDelete => "hold_delete",
            Self::Chronology => "chronology",
            Self::PolicySimulation => "policy_simulation",
            Self::ExceptionExpiry => "exception_expiry",
        }
    }
}

/// Published certification verdict for one governed family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationVerdict {
    /// Every proof dimension is observed and current; the claim holds.
    Certified,
    /// At least one proof dimension is missing, stale, or unproven; the claim narrows.
    Narrowed,
}

impl CertificationVerdict {
    /// Returns the stable token for this verdict.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Narrowed => "narrowed",
        }
    }

    /// Returns `true` when the family holds its published claim.
    pub const fn holds_claim(self) -> bool {
        matches!(self, Self::Certified)
    }
}

/// Closed kind explaining why a proof dimension narrows a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationNarrowKind {
    /// The dimension's backing proof is absent.
    ProofMissing,
    /// The dimension's backing proof exists but is stale.
    ProofStale,
    /// The dimension's live upstream packet failed validation.
    UpstreamValidationFailed,
    /// The family claims a managed control its boundary cannot prove.
    ManagedClaimUnproven,
}

impl CertificationNarrowKind {
    /// Every narrow kind in canonical order.
    pub const ALL: [Self; 4] = [
        Self::ProofMissing,
        Self::ProofStale,
        Self::UpstreamValidationFailed,
        Self::ManagedClaimUnproven,
    ];

    /// Returns the stable token for this kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofMissing => "proof_missing",
            Self::ProofStale => "proof_stale",
            Self::UpstreamValidationFailed => "upstream_validation_failed",
            Self::ManagedClaimUnproven => "managed_claim_unproven",
        }
    }
}

/// Shiproom-level decision rolled up from the certified families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationPromotionDecision {
    /// No release-blocking family is narrowed; promotion may proceed.
    Promote,
    /// One or more release-blocking families are narrowed; promotion holds.
    Hold,
}

impl CertificationPromotionDecision {
    /// Returns the stable token for this decision.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Promote => "promote",
            Self::Hold => "hold",
        }
    }
}

/// Consumer surface that ingests the certification result directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationConsumerSurface {
    /// Shiproom promotion and gate review.
    Shiproom,
    /// Public claim and status surfaces.
    PublicClaim,
    /// CLI or headless explain output.
    CliHeadless,
    /// Support-export packets and incident handoff.
    SupportExport,
}

impl CertificationConsumerSurface {
    /// Every required consumer surface.
    pub const ALL: [Self; 4] = [
        Self::Shiproom,
        Self::PublicClaim,
        Self::CliHeadless,
        Self::SupportExport,
    ];
}

/// Upstream contract references the certification binds together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationUpstreamContracts {
    /// Record-class registry contract backing the record-class dimension.
    pub record_class_matrix_ref: String,
    /// Hold/retention contract backing the hold/delete dimension.
    pub hold_retention_contract_ref: String,
    /// Policy-simulation contract backing the policy-simulation dimension.
    pub policy_simulation_contract_ref: String,
    /// Exception/expiry contract backing the exception/expiry dimension.
    pub exception_expiry_contract_ref: String,
    /// Chronology contract backing the chronology dimension.
    pub chronology_contract_ref: String,
}

/// One proof cell binding a dimension to its upstream truth source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationProofCell {
    /// The proof dimension this cell covers.
    pub dimension: CertificationProofDimension,
    /// Upstream packet record kind that proves the dimension.
    pub source_record_kind: String,
    /// Upstream shared-contract reference the cell pins.
    pub contract_ref: String,
    /// Artifact, document, or row reference carrying the proof.
    pub proof_ref: String,
    /// `true` when the backing proof is observed for this family.
    pub observed: bool,
    /// Freshness of the backing proof.
    pub freshness: ProofFreshnessClass,
}

impl CertificationProofCell {
    /// Returns `true` when the proof is both observed and current.
    pub fn is_current(&self) -> bool {
        self.observed && self.freshness == ProofFreshnessClass::Current
    }
}

/// One narrowing reason pinning a dimension to its narrow kind.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationNarrowReason {
    /// The proof dimension that triggered the narrowing.
    pub dimension: CertificationProofDimension,
    /// Why the dimension narrows the family.
    pub kind: CertificationNarrowKind,
}

/// One governed M5 managed/enterprise family certified by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RecordsPolicyCertificationRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The governed artifact family.
    pub artifact_family: GovernedArtifactFamily,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// Record class governing the family.
    pub record_class_id: RecordClassId,
    /// Where the platform's destructive/export authority lives.
    pub authority_boundary: AuthorityBoundaryClass,
    /// `true` when a managed hold may be claimed for this family.
    pub claims_managed_hold: bool,
    /// `true` when a managed export may be claimed for this family.
    pub claims_managed_export: bool,
    /// `true` when a managed delete may be claimed for this family.
    pub claims_managed_delete: bool,
    /// Reference to the governing records/policy matrix row.
    pub matrix_row_ref: String,
    /// Proof cells, one per dimension.
    pub proof_cells: Vec<CertificationProofCell>,
    /// Published verdict after narrowing.
    pub verdict: CertificationVerdict,
    /// Active narrow reasons backing the verdict.
    #[serde(default)]
    pub narrow_reasons: Vec<CertificationNarrowReason>,
    /// Single label shiproom surfaces ingest verbatim.
    pub shiproom_label: String,
    /// Single label public claim surfaces ingest verbatim.
    pub public_claim_label: String,
    /// Reviewable rationale for the family.
    pub rationale: String,
}

impl M5RecordsPolicyCertificationRow {
    /// Returns the proof cell for `dimension`, if present.
    pub fn cell(&self, dimension: CertificationProofDimension) -> Option<&CertificationProofCell> {
        self.proof_cells
            .iter()
            .find(|cell| cell.dimension == dimension)
    }

    /// Returns `true` when the family is narrowed below its claimed posture.
    pub fn needs_review(&self) -> bool {
        !self.verdict.holds_claim()
    }

    /// Recomputes the verdict and narrow reasons from the proof cells alone.
    ///
    /// A dimension narrows when its cell is missing/unobserved ([`CertificationNarrowKind::ProofMissing`]),
    /// stale ([`CertificationNarrowKind::ProofStale`]), or — for the hold/delete
    /// dimension on a local-only family that claims a managed control —
    /// [`CertificationNarrowKind::ManagedClaimUnproven`]. The verdict is
    /// [`CertificationVerdict::Certified`] only when no reason is raised.
    pub fn computed_verdict(&self) -> (CertificationVerdict, Vec<CertificationNarrowReason>) {
        let mut reasons = Vec::new();

        for dimension in CertificationProofDimension::ALL {
            match self.cell(dimension) {
                None => reasons.push(CertificationNarrowReason {
                    dimension,
                    kind: CertificationNarrowKind::ProofMissing,
                }),
                Some(cell) if !cell.observed => reasons.push(CertificationNarrowReason {
                    dimension,
                    kind: CertificationNarrowKind::ProofMissing,
                }),
                Some(cell) => match cell.freshness {
                    ProofFreshnessClass::Current => {}
                    ProofFreshnessClass::Stale => reasons.push(CertificationNarrowReason {
                        dimension,
                        kind: CertificationNarrowKind::ProofStale,
                    }),
                    ProofFreshnessClass::Missing => reasons.push(CertificationNarrowReason {
                        dimension,
                        kind: CertificationNarrowKind::ProofMissing,
                    }),
                },
            }
        }

        if self.authority_boundary == AuthorityBoundaryClass::LocalOnly
            && (self.claims_managed_hold
                || self.claims_managed_export
                || self.claims_managed_delete)
        {
            reasons.push(CertificationNarrowReason {
                dimension: CertificationProofDimension::HoldDelete,
                kind: CertificationNarrowKind::ManagedClaimUnproven,
            });
        }

        reasons.sort();
        reasons.dedup();

        let verdict = if reasons.is_empty() {
            CertificationVerdict::Certified
        } else {
            CertificationVerdict::Narrowed
        };
        (verdict, reasons)
    }
}

/// Consumer binding that must ingest the certification packet directly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationConsumerBinding {
    /// The consumer surface.
    pub surface: CertificationConsumerSurface,
    /// Stable consumer reference.
    pub consumer_ref: String,
    /// Reviewable explanation of how the consumer ingests the packet.
    pub projection_rule: String,
}

/// Shiproom promotion record rolled up from the certified families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationPromotionRecord {
    /// Promotion gate name.
    pub promotion_gate: String,
    /// Promote or hold decision.
    pub decision: CertificationPromotionDecision,
    /// Release-blocking entry ids currently holding promotion.
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Roll-up counts for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationSummary {
    /// Total governed families.
    pub total_rows: usize,
    /// Distinct governed families covered.
    pub total_families: usize,
    /// Release-blocking families.
    pub release_blocking_rows: usize,
    /// Certified families.
    pub certified_rows: usize,
    /// Narrowed families.
    pub narrowed_rows: usize,
    /// Proof cells that are current.
    pub proof_current_cells: usize,
    /// Proof cells that are stale.
    pub proof_stale_cells: usize,
    /// Proof cells that are missing.
    pub proof_missing_cells: usize,
    /// Active narrow reasons across the packet.
    pub total_narrow_reasons: usize,
    /// Consumer bindings declared by the packet.
    pub consumer_binding_count: usize,
}

/// Canonical certification packet for the M5 record-governance lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RecordsPolicyCertificationPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Stable record kind.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// UTC instant the packet reflects.
    pub as_of: String,
    /// Human-readable companion document.
    pub overview_doc_ref: String,
    /// Companion artifact summary.
    pub artifact_summary_ref: String,
    /// JSON Schema reference.
    pub schema_ref: String,
    /// Upstream contract references the packet binds together.
    pub upstream_contracts: CertificationUpstreamContracts,
    /// Proof-dimension vocabulary.
    pub proof_dimensions: Vec<CertificationProofDimension>,
    /// Freshness vocabulary.
    pub freshness_states: Vec<ProofFreshnessClass>,
    /// Verdict vocabulary.
    pub verdict_states: Vec<CertificationVerdict>,
    /// Narrow-kind vocabulary.
    pub narrow_kinds: Vec<CertificationNarrowKind>,
    /// Required consumer bindings.
    pub consumer_bindings: Vec<CertificationConsumerBinding>,
    /// Governed families.
    pub rows: Vec<M5RecordsPolicyCertificationRow>,
    /// Shiproom promotion record.
    pub promotion: CertificationPromotionRecord,
    /// Roll-up counts.
    pub summary: CertificationSummary,
}

/// Narrow projection ingested by shiproom and public claim surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationClaimProjectionRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// Published verdict.
    pub verdict: CertificationVerdict,
    /// Single label the surface renders verbatim.
    pub label: String,
    /// Active narrow reasons.
    pub narrow_reasons: Vec<CertificationNarrowReason>,
}

/// CLI/headless projection over one governed family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationCliHeadlessRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Governing record class.
    pub record_class_id: RecordClassId,
    /// Published verdict.
    pub verdict: CertificationVerdict,
    /// Per-dimension freshness, one entry per dimension in canonical order.
    pub dimension_freshness: Vec<(CertificationProofDimension, ProofFreshnessClass)>,
    /// Active narrow reasons.
    pub narrow_reasons: Vec<CertificationNarrowReason>,
}

/// Support/export projection over one governed family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSupportExportRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Governing record class.
    pub record_class_id: RecordClassId,
    /// Where destructive/export authority lives.
    pub authority_boundary: AuthorityBoundaryClass,
    /// Published verdict.
    pub verdict: CertificationVerdict,
    /// Proof refs disclosed per dimension.
    pub proof_refs: Vec<(CertificationProofDimension, String)>,
    /// Active narrow reasons.
    pub narrow_reasons: Vec<CertificationNarrowReason>,
}

/// Cross-check finding raised when live truth disagrees with the certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code", content = "detail")]
pub enum CertificationCrossCheckFinding {
    /// The records/policy matrix could not be loaded.
    MatrixUnreadable {
        /// Parser error message.
        message: String,
    },
    /// A live upstream packet failed its own validation.
    UpstreamPacketInvalid {
        /// The dimension whose upstream packet regressed.
        dimension: CertificationProofDimension,
    },
    /// A certified family is absent from the live matrix.
    MatrixRowMissing {
        /// The certification entry id.
        entry_id: String,
        /// The governed family missing from the matrix.
        artifact_family: GovernedArtifactFamily,
    },
    /// A proof cell names a source record kind that does not match live truth.
    ProofSourceMismatch {
        /// The certification entry id.
        entry_id: String,
        /// The proof dimension.
        dimension: CertificationProofDimension,
        /// The source record kind named by the cell.
        found: String,
        /// The record kind live truth expects.
        expected: String,
    },
}

/// Validation violation emitted by the typed certification model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code", content = "detail")]
pub enum M5RecordsPolicyCertificationViolation {
    /// Schema version mismatch.
    SchemaVersionMismatch {
        /// Version found in the packet.
        found: u32,
    },
    /// Record kind mismatch.
    RecordKindMismatch {
        /// Record kind found in the packet.
        found: String,
    },
    /// A required family is uncovered.
    ArtifactFamilyUncovered {
        /// The uncovered family.
        artifact_family: GovernedArtifactFamily,
    },
    /// A required consumer surface is not bound.
    ConsumerSurfaceUnbound {
        /// The unbound surface.
        surface: CertificationConsumerSurface,
    },
    /// A family is covered more than once.
    DuplicateFamily {
        /// The duplicated family.
        artifact_family: GovernedArtifactFamily,
    },
    /// A row names an unknown record class.
    UnknownRecordClass {
        /// The certification entry id.
        entry_id: String,
        /// The unknown record class.
        record_class_id: RecordClassId,
    },
    /// A row omits a required proof dimension.
    ProofDimensionMissing {
        /// The certification entry id.
        entry_id: String,
        /// The missing dimension.
        dimension: CertificationProofDimension,
    },
    /// A row declares a proof dimension more than once.
    ProofDimensionDuplicated {
        /// The certification entry id.
        entry_id: String,
        /// The duplicated dimension.
        dimension: CertificationProofDimension,
    },
    /// A proof cell omits its source record kind or proof reference.
    ProofCellIncomplete {
        /// The certification entry id.
        entry_id: String,
        /// The incomplete dimension.
        dimension: CertificationProofDimension,
    },
    /// A local-only family claims a managed control it cannot prove.
    ManagedControlOverclaimed {
        /// The certification entry id.
        entry_id: String,
        /// The overclaimed control.
        control: String,
    },
    /// The stored verdict disagrees with the computed verdict.
    VerdictMismatch {
        /// The certification entry id.
        entry_id: String,
        /// The stored verdict.
        found: CertificationVerdict,
        /// The computed verdict.
        expected: CertificationVerdict,
    },
    /// The stored narrow reasons disagree with the computed reasons.
    NarrowReasonsMismatch {
        /// The certification entry id.
        entry_id: String,
    },
    /// A shiproom or public claim label is empty or contradicts the verdict.
    ClaimLabelInconsistent {
        /// The certification entry id.
        entry_id: String,
        /// The inconsistent surface label.
        surface: CertificationConsumerSurface,
    },
    /// The promotion decision disagrees with the computed decision.
    PromotionDecisionMismatch {
        /// The stored decision.
        found: CertificationPromotionDecision,
        /// The computed decision.
        expected: CertificationPromotionDecision,
    },
    /// The promotion blocking list disagrees with the narrowed release-blocking families.
    PromotionBlockingMismatch,
    /// A summary roll-up field disagrees with the computed roll-up.
    SummaryMismatch {
        /// The mismatched field name.
        field: String,
    },
}

impl M5RecordsPolicyCertificationPacket {
    /// Validates structural and logical invariants of the certification packet.
    pub fn validate(&self) -> Vec<M5RecordsPolicyCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != M5_RECORDS_POLICY_CERTIFICATION_SCHEMA_VERSION {
            violations.push(
                M5RecordsPolicyCertificationViolation::SchemaVersionMismatch {
                    found: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_RECORDS_POLICY_CERTIFICATION_RECORD_KIND {
            violations.push(M5RecordsPolicyCertificationViolation::RecordKindMismatch {
                found: self.record_kind.clone(),
            });
        }

        let registry = current_registry();

        let mut seen_families: BTreeSet<GovernedArtifactFamily> = BTreeSet::new();
        for row in &self.rows {
            if !seen_families.insert(row.artifact_family) {
                violations.push(M5RecordsPolicyCertificationViolation::DuplicateFamily {
                    artifact_family: row.artifact_family,
                });
            }
        }
        for family in GovernedArtifactFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    M5RecordsPolicyCertificationViolation::ArtifactFamilyUncovered {
                        artifact_family: family,
                    },
                );
            }
        }

        let bound_surfaces: BTreeSet<CertificationConsumerSurface> = self
            .consumer_bindings
            .iter()
            .map(|binding| binding.surface)
            .collect();
        for surface in CertificationConsumerSurface::ALL {
            if !bound_surfaces.contains(&surface) {
                violations.push(
                    M5RecordsPolicyCertificationViolation::ConsumerSurfaceUnbound { surface },
                );
            }
        }

        for row in &self.rows {
            if let Ok(registry) = &registry {
                if registry.row(row.record_class_id).is_none() {
                    violations.push(M5RecordsPolicyCertificationViolation::UnknownRecordClass {
                        entry_id: row.entry_id.clone(),
                        record_class_id: row.record_class_id,
                    });
                }
            }

            let mut seen_dimensions: BTreeSet<CertificationProofDimension> = BTreeSet::new();
            for cell in &row.proof_cells {
                if !seen_dimensions.insert(cell.dimension) {
                    violations.push(
                        M5RecordsPolicyCertificationViolation::ProofDimensionDuplicated {
                            entry_id: row.entry_id.clone(),
                            dimension: cell.dimension,
                        },
                    );
                }
                if cell.source_record_kind.trim().is_empty() || cell.proof_ref.trim().is_empty() {
                    violations.push(M5RecordsPolicyCertificationViolation::ProofCellIncomplete {
                        entry_id: row.entry_id.clone(),
                        dimension: cell.dimension,
                    });
                }
            }
            for dimension in CertificationProofDimension::ALL {
                if !seen_dimensions.contains(&dimension) {
                    violations.push(
                        M5RecordsPolicyCertificationViolation::ProofDimensionMissing {
                            entry_id: row.entry_id.clone(),
                            dimension,
                        },
                    );
                }
            }

            if row.authority_boundary == AuthorityBoundaryClass::LocalOnly {
                for (claimed, control) in [
                    (row.claims_managed_hold, "managed_hold"),
                    (row.claims_managed_export, "managed_export"),
                    (row.claims_managed_delete, "managed_delete"),
                ] {
                    if claimed {
                        violations.push(
                            M5RecordsPolicyCertificationViolation::ManagedControlOverclaimed {
                                entry_id: row.entry_id.clone(),
                                control: control.to_owned(),
                            },
                        );
                    }
                }
            }

            let (expected_verdict, expected_reasons) = row.computed_verdict();
            if row.verdict != expected_verdict {
                violations.push(M5RecordsPolicyCertificationViolation::VerdictMismatch {
                    entry_id: row.entry_id.clone(),
                    found: row.verdict,
                    expected: expected_verdict,
                });
            }
            let mut stored_reasons = row.narrow_reasons.clone();
            stored_reasons.sort();
            if stored_reasons != expected_reasons {
                violations.push(
                    M5RecordsPolicyCertificationViolation::NarrowReasonsMismatch {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }

            if !label_matches_verdict(&row.shiproom_label, row.verdict) {
                violations.push(
                    M5RecordsPolicyCertificationViolation::ClaimLabelInconsistent {
                        entry_id: row.entry_id.clone(),
                        surface: CertificationConsumerSurface::Shiproom,
                    },
                );
            }
            if !label_matches_verdict(&row.public_claim_label, row.verdict) {
                violations.push(
                    M5RecordsPolicyCertificationViolation::ClaimLabelInconsistent {
                        entry_id: row.entry_id.clone(),
                        surface: CertificationConsumerSurface::PublicClaim,
                    },
                );
            }
        }

        let expected_decision = self.computed_promotion_decision();
        if self.promotion.decision != expected_decision {
            violations.push(
                M5RecordsPolicyCertificationViolation::PromotionDecisionMismatch {
                    found: self.promotion.decision,
                    expected: expected_decision,
                },
            );
        }
        let mut expected_blocking: Vec<String> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking && row.needs_review())
            .map(|row| row.entry_id.clone())
            .collect();
        expected_blocking.sort();
        let mut found_blocking = self.promotion.blocking_entry_ids.clone();
        found_blocking.sort();
        if found_blocking != expected_blocking {
            violations.push(M5RecordsPolicyCertificationViolation::PromotionBlockingMismatch);
        }

        let computed_summary = self.computed_summary();
        for (field, ok) in [
            (
                "total_rows",
                self.summary.total_rows == computed_summary.total_rows,
            ),
            (
                "total_families",
                self.summary.total_families == computed_summary.total_families,
            ),
            (
                "release_blocking_rows",
                self.summary.release_blocking_rows == computed_summary.release_blocking_rows,
            ),
            (
                "certified_rows",
                self.summary.certified_rows == computed_summary.certified_rows,
            ),
            (
                "narrowed_rows",
                self.summary.narrowed_rows == computed_summary.narrowed_rows,
            ),
            (
                "proof_current_cells",
                self.summary.proof_current_cells == computed_summary.proof_current_cells,
            ),
            (
                "proof_stale_cells",
                self.summary.proof_stale_cells == computed_summary.proof_stale_cells,
            ),
            (
                "proof_missing_cells",
                self.summary.proof_missing_cells == computed_summary.proof_missing_cells,
            ),
            (
                "total_narrow_reasons",
                self.summary.total_narrow_reasons == computed_summary.total_narrow_reasons,
            ),
            (
                "consumer_binding_count",
                self.summary.consumer_binding_count == computed_summary.consumer_binding_count,
            ),
        ] {
            if !ok {
                violations.push(M5RecordsPolicyCertificationViolation::SummaryMismatch {
                    field: field.to_owned(),
                });
            }
        }

        violations
    }

    /// Returns the row for `artifact_family`, if present.
    pub fn row_for_family(
        &self,
        artifact_family: GovernedArtifactFamily,
    ) -> Option<&M5RecordsPolicyCertificationRow> {
        self.rows
            .iter()
            .find(|row| row.artifact_family == artifact_family)
    }

    /// Recomputes the promotion decision from the rows alone.
    pub fn computed_promotion_decision(&self) -> CertificationPromotionDecision {
        if self
            .rows
            .iter()
            .any(|row| row.release_blocking && row.needs_review())
        {
            CertificationPromotionDecision::Hold
        } else {
            CertificationPromotionDecision::Promote
        }
    }

    /// Recomputes the summary from the rows and consumer bindings.
    pub fn computed_summary(&self) -> CertificationSummary {
        let mut proof_current_cells = 0;
        let mut proof_stale_cells = 0;
        let mut proof_missing_cells = 0;
        for row in &self.rows {
            for cell in &row.proof_cells {
                if !cell.observed {
                    proof_missing_cells += 1;
                    continue;
                }
                match cell.freshness {
                    ProofFreshnessClass::Current => proof_current_cells += 1,
                    ProofFreshnessClass::Stale => proof_stale_cells += 1,
                    ProofFreshnessClass::Missing => proof_missing_cells += 1,
                }
            }
        }

        CertificationSummary {
            total_rows: self.rows.len(),
            total_families: self
                .rows
                .iter()
                .map(|row| row.artifact_family)
                .collect::<BTreeSet<_>>()
                .len(),
            release_blocking_rows: self.rows.iter().filter(|row| row.release_blocking).count(),
            certified_rows: self
                .rows
                .iter()
                .filter(|row| row.verdict == CertificationVerdict::Certified)
                .count(),
            narrowed_rows: self
                .rows
                .iter()
                .filter(|row| row.verdict == CertificationVerdict::Narrowed)
                .count(),
            proof_current_cells,
            proof_stale_cells,
            proof_missing_cells,
            total_narrow_reasons: self.rows.iter().map(|row| row.narrow_reasons.len()).sum(),
            consumer_binding_count: self.consumer_bindings.len(),
        }
    }

    /// Returns the shiproom projection rows.
    ///
    /// Shiproom gate review ingests these rows verbatim instead of cloning its
    /// own status text.
    pub fn shiproom_projection(&self) -> Vec<CertificationClaimProjectionRow> {
        self.claim_projection(|row| row.shiproom_label.clone())
    }

    /// Returns the public-claim projection rows.
    ///
    /// Public status surfaces ingest these rows verbatim instead of cloning
    /// their own status text.
    pub fn public_claim_projection(&self) -> Vec<CertificationClaimProjectionRow> {
        self.claim_projection(|row| row.public_claim_label.clone())
    }

    fn claim_projection(
        &self,
        label: impl Fn(&M5RecordsPolicyCertificationRow) -> String,
    ) -> Vec<CertificationClaimProjectionRow> {
        self.rows
            .iter()
            .map(|row| CertificationClaimProjectionRow {
                entry_id: row.entry_id.clone(),
                artifact_family: row.artifact_family,
                release_blocking: row.release_blocking,
                verdict: row.verdict,
                label: label(row),
                narrow_reasons: row.narrow_reasons.clone(),
            })
            .collect()
    }

    /// Returns the CLI/headless projection rows.
    pub fn cli_headless_projection(&self) -> Vec<CertificationCliHeadlessRow> {
        self.rows
            .iter()
            .map(|row| CertificationCliHeadlessRow {
                entry_id: row.entry_id.clone(),
                artifact_family: row.artifact_family,
                record_class_id: row.record_class_id,
                verdict: row.verdict,
                dimension_freshness: CertificationProofDimension::ALL
                    .into_iter()
                    .map(|dimension| {
                        let freshness = row
                            .cell(dimension)
                            .map(|cell| {
                                if cell.observed {
                                    cell.freshness
                                } else {
                                    ProofFreshnessClass::Missing
                                }
                            })
                            .unwrap_or(ProofFreshnessClass::Missing);
                        (dimension, freshness)
                    })
                    .collect(),
                narrow_reasons: row.narrow_reasons.clone(),
            })
            .collect()
    }

    /// Returns the support/export projection rows.
    pub fn support_export_projection(&self) -> Vec<CertificationSupportExportRow> {
        self.rows
            .iter()
            .map(|row| CertificationSupportExportRow {
                entry_id: row.entry_id.clone(),
                record_class_id: row.record_class_id,
                authority_boundary: row.authority_boundary,
                verdict: row.verdict,
                proof_refs: row
                    .proof_cells
                    .iter()
                    .map(|cell| (cell.dimension, cell.proof_ref.clone()))
                    .collect(),
                narrow_reasons: row.narrow_reasons.clone(),
            })
            .collect()
    }

    /// Cross-checks the certification against live same-crate truth.
    ///
    /// Returns the findings that would force a re-certification: the records/policy
    /// matrix or a same-crate upstream packet failing its own validation, a
    /// certified family missing from the live matrix, or a proof cell naming a
    /// source record kind that disagrees with live truth. An empty result means
    /// the certification is consistent with current truth.
    pub fn verify_against_live_packets(&self) -> Vec<CertificationCrossCheckFinding> {
        let mut findings = Vec::new();

        match current_records_policy_matrix() {
            Ok(matrix) => {
                if !matrix.validate().is_empty() {
                    findings.push(CertificationCrossCheckFinding::UpstreamPacketInvalid {
                        dimension: CertificationProofDimension::RecordClass,
                    });
                }
                for row in &self.rows {
                    if matrix.row_for_family(row.artifact_family).is_none() {
                        findings.push(CertificationCrossCheckFinding::MatrixRowMissing {
                            entry_id: row.entry_id.clone(),
                            artifact_family: row.artifact_family,
                        });
                    }
                }
                self.check_chronology_sources(&matrix, &mut findings);
            }
            Err(error) => findings.push(CertificationCrossCheckFinding::MatrixUnreadable {
                message: error.to_string(),
            }),
        }

        let hold_retention = seeded_m5_records_policy_packet();
        if !hold_retention.validate().is_empty() {
            findings.push(CertificationCrossCheckFinding::UpstreamPacketInvalid {
                dimension: CertificationProofDimension::HoldDelete,
            });
        }
        self.check_cell_source(
            CertificationProofDimension::HoldDelete,
            &hold_retention.record_kind,
            &mut findings,
        );

        let policy_simulation = seeded_m5_policy_simulation_packet();
        if !policy_simulation.validate().is_empty() {
            findings.push(CertificationCrossCheckFinding::UpstreamPacketInvalid {
                dimension: CertificationProofDimension::PolicySimulation,
            });
        }
        self.check_cell_source(
            CertificationProofDimension::PolicySimulation,
            &policy_simulation.record_kind,
            &mut findings,
        );

        findings
    }

    fn check_cell_source(
        &self,
        dimension: CertificationProofDimension,
        expected: &str,
        findings: &mut Vec<CertificationCrossCheckFinding>,
    ) {
        for row in &self.rows {
            if let Some(cell) = row.cell(dimension) {
                if cell.source_record_kind != expected {
                    findings.push(CertificationCrossCheckFinding::ProofSourceMismatch {
                        entry_id: row.entry_id.clone(),
                        dimension,
                        found: cell.source_record_kind.clone(),
                        expected: expected.to_owned(),
                    });
                }
            }
        }
    }

    fn check_chronology_sources(
        &self,
        matrix: &RecordsPolicySimulationMatrix,
        findings: &mut Vec<CertificationCrossCheckFinding>,
    ) {
        for row in &self.rows {
            let Some(matrix_row) = matrix.row_for_family(row.artifact_family) else {
                continue;
            };
            if let Some(cell) = row.cell(CertificationProofDimension::Chronology) {
                let expected = matrix_row.chronology_contract.row_id.clone();
                if cell.proof_ref != expected {
                    findings.push(CertificationCrossCheckFinding::ProofSourceMismatch {
                        entry_id: row.entry_id.clone(),
                        dimension: CertificationProofDimension::Chronology,
                        found: cell.proof_ref.clone(),
                        expected,
                    });
                }
            }
        }
    }
}

/// Returns `true` when a published label is non-empty and agrees with the verdict.
///
/// A certified row must carry a label that reads "certified" and a narrowed row
/// must read "narrowed" so a surface can never render cosmetically simple "done"
/// copy over a narrowed claim.
fn label_matches_verdict(label: &str, verdict: CertificationVerdict) -> bool {
    let trimmed = label.trim().to_ascii_lowercase();
    if trimmed.is_empty() {
        return false;
    }
    match verdict {
        CertificationVerdict::Certified => {
            trimmed.contains("certified") && !trimmed.contains("narrowed")
        }
        CertificationVerdict::Narrowed => trimmed.contains("narrowed"),
    }
}

fn shiproom_label_for(row: &M5RecordsPolicyCertificationRow) -> String {
    match row.verdict {
        CertificationVerdict::Certified => format!(
            "M5 {} record governance certified — all proofs current",
            row.artifact_family.as_str()
        ),
        CertificationVerdict::Narrowed => format!(
            "M5 {} record governance narrowed — {} open proof gap(s); promotion held",
            row.artifact_family.as_str(),
            row.narrow_reasons.len()
        ),
    }
}

fn public_claim_label_for(row: &M5RecordsPolicyCertificationRow) -> String {
    match row.verdict {
        CertificationVerdict::Certified => format!(
            "{}: record-governance claim certified",
            row.artifact_family.as_str()
        ),
        CertificationVerdict::Narrowed => format!(
            "{}: record-governance claim narrowed pending proof",
            row.artifact_family.as_str()
        ),
    }
}

/// Builds the canonical certification packet from live same-crate truth.
///
/// The packet binds one row per governed family in the checked-in records/policy
/// matrix, attaches a proof cell per dimension, and computes each verdict, label,
/// promotion decision, and summary so the result is internally consistent and
/// equal to the checked-in fixture.
pub fn seeded_m5_records_policy_certification_packet() -> M5RecordsPolicyCertificationPacket {
    let matrix = current_records_policy_matrix().expect("records/policy matrix parses");

    let rows: Vec<M5RecordsPolicyCertificationRow> = matrix
        .rows
        .iter()
        .map(|matrix_row| {
            let record_class_proof_kind = matrix_row
                .producer_record_kinds
                .first()
                .cloned()
                .unwrap_or_else(|| matrix_row.record_class_id.as_str().to_owned());

            let proof_cells = vec![
                CertificationProofCell {
                    dimension: CertificationProofDimension::RecordClass,
                    source_record_kind: record_class_proof_kind,
                    contract_ref: matrix.shared_contract_ref.clone(),
                    proof_ref: matrix_row.proof_ref.clone(),
                    observed: true,
                    freshness: ProofFreshnessClass::Current,
                },
                CertificationProofCell {
                    dimension: CertificationProofDimension::HoldDelete,
                    source_record_kind: M5_RECORDS_POLICY_RECORD_KIND.to_owned(),
                    contract_ref: M5_RECORDS_POLICY_SHARED_CONTRACT_REF.to_owned(),
                    proof_ref: M5_RECORDS_POLICY_ARTIFACT_REF.to_owned(),
                    observed: true,
                    freshness: ProofFreshnessClass::Current,
                },
                CertificationProofCell {
                    dimension: CertificationProofDimension::Chronology,
                    source_record_kind: M5_CERT_CHRONOLOGY_PACKET_RECORD_KIND.to_owned(),
                    contract_ref: M5_CERT_CHRONOLOGY_CONTRACT_REF.to_owned(),
                    proof_ref: matrix_row.chronology_contract.row_id.clone(),
                    observed: true,
                    freshness: ProofFreshnessClass::Current,
                },
                CertificationProofCell {
                    dimension: CertificationProofDimension::PolicySimulation,
                    source_record_kind: M5_POLICY_SIMULATION_RECORD_KIND.to_owned(),
                    contract_ref: M5_POLICY_SIMULATION_SHARED_CONTRACT_REF.to_owned(),
                    proof_ref: M5_POLICY_SIMULATION_ARTIFACT_REF.to_owned(),
                    observed: true,
                    freshness: ProofFreshnessClass::Current,
                },
                CertificationProofCell {
                    dimension: CertificationProofDimension::ExceptionExpiry,
                    source_record_kind: M5_CERT_EXCEPTION_PACKET_RECORD_KIND.to_owned(),
                    contract_ref: M5_CERT_EXCEPTION_CONTRACT_REF.to_owned(),
                    proof_ref: M5_CERT_EXCEPTION_ARTIFACT_REF.to_owned(),
                    observed: true,
                    freshness: ProofFreshnessClass::Current,
                },
            ];

            let mut row = M5RecordsPolicyCertificationRow {
                entry_id: format!("cert:{}", matrix_row.artifact_family.as_str()),
                title: format!("{} record-governance certification", matrix_row.title),
                artifact_family: matrix_row.artifact_family,
                release_blocking: matrix_row.release_blocking,
                record_class_id: matrix_row.record_class_id,
                authority_boundary: matrix_row.authority_boundary,
                claims_managed_hold: matrix_row.claims_managed_hold,
                claims_managed_export: matrix_row.claims_managed_export,
                claims_managed_delete: matrix_row.claims_managed_delete,
                matrix_row_ref: matrix_row.entry_id.clone(),
                proof_cells,
                verdict: CertificationVerdict::Certified,
                narrow_reasons: Vec::new(),
                shiproom_label: String::new(),
                public_claim_label: String::new(),
                rationale: format!(
                    "All five lane proofs (record class, hold/delete, chronology, policy \
                     simulation, exception/expiry) are current for {}.",
                    matrix_row.artifact_family.as_str()
                ),
            };
            let (verdict, narrow_reasons) = row.computed_verdict();
            row.verdict = verdict;
            row.narrow_reasons = narrow_reasons;
            row.shiproom_label = shiproom_label_for(&row);
            row.public_claim_label = public_claim_label_for(&row);
            row
        })
        .collect();

    let consumer_bindings = vec![
        CertificationConsumerBinding {
            surface: CertificationConsumerSurface::Shiproom,
            consumer_ref: "shiproom:m5_record_governance_gate".to_owned(),
            projection_rule: "Ingest shiproom_projection(); a Hold promotion decision blocks \
                              the M5 record-governance gate."
                .to_owned(),
        },
        CertificationConsumerBinding {
            surface: CertificationConsumerSurface::PublicClaim,
            consumer_ref: "public_claim:m5_record_governance".to_owned(),
            projection_rule: "Ingest public_claim_projection(); render each label verbatim and \
                              never widen a narrowed claim."
                .to_owned(),
        },
        CertificationConsumerBinding {
            surface: CertificationConsumerSurface::CliHeadless,
            consumer_ref: "cli:records explain --certification".to_owned(),
            projection_rule: "Ingest cli_headless_projection() for per-dimension freshness and \
                              narrow reasons."
                .to_owned(),
        },
        CertificationConsumerBinding {
            surface: CertificationConsumerSurface::SupportExport,
            consumer_ref: "support_export:m5_record_governance_certification".to_owned(),
            projection_rule: "Ingest support_export_projection() for metadata-safe proof refs \
                              and verdicts."
                .to_owned(),
        },
    ];

    let mut packet = M5RecordsPolicyCertificationPacket {
        schema_version: M5_RECORDS_POLICY_CERTIFICATION_SCHEMA_VERSION,
        record_kind: M5_RECORDS_POLICY_CERTIFICATION_RECORD_KIND.to_owned(),
        packet_id: "m5-records-policy-certification:0001".to_owned(),
        shared_contract_ref: M5_RECORDS_POLICY_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
        as_of: "2026-06-13T16:00:00Z".to_owned(),
        overview_doc_ref: M5_RECORDS_POLICY_CERTIFICATION_DOC_REF.to_owned(),
        artifact_summary_ref: M5_RECORDS_POLICY_CERTIFICATION_ARTIFACT_REF.to_owned(),
        schema_ref: M5_RECORDS_POLICY_CERTIFICATION_SCHEMA_REF.to_owned(),
        upstream_contracts: CertificationUpstreamContracts {
            record_class_matrix_ref: matrix.shared_contract_ref.clone(),
            hold_retention_contract_ref: M5_RECORDS_POLICY_SHARED_CONTRACT_REF.to_owned(),
            policy_simulation_contract_ref: M5_POLICY_SIMULATION_SHARED_CONTRACT_REF.to_owned(),
            exception_expiry_contract_ref: M5_CERT_EXCEPTION_CONTRACT_REF.to_owned(),
            chronology_contract_ref: M5_CERT_CHRONOLOGY_CONTRACT_REF.to_owned(),
        },
        proof_dimensions: CertificationProofDimension::ALL.to_vec(),
        freshness_states: vec![
            ProofFreshnessClass::Current,
            ProofFreshnessClass::Stale,
            ProofFreshnessClass::Missing,
        ],
        verdict_states: vec![
            CertificationVerdict::Certified,
            CertificationVerdict::Narrowed,
        ],
        narrow_kinds: CertificationNarrowKind::ALL.to_vec(),
        consumer_bindings,
        rows,
        promotion: CertificationPromotionRecord {
            promotion_gate: "m5_record_governance_promotion".to_owned(),
            decision: CertificationPromotionDecision::Promote,
            blocking_entry_ids: Vec::new(),
            rationale: String::new(),
        },
        summary: CertificationSummary {
            total_rows: 0,
            total_families: 0,
            release_blocking_rows: 0,
            certified_rows: 0,
            narrowed_rows: 0,
            proof_current_cells: 0,
            proof_stale_cells: 0,
            proof_missing_cells: 0,
            total_narrow_reasons: 0,
            consumer_binding_count: 0,
        },
    };

    let decision = packet.computed_promotion_decision();
    let blocking: Vec<String> = packet
        .rows
        .iter()
        .filter(|row| row.release_blocking && row.needs_review())
        .map(|row| row.entry_id.clone())
        .collect();
    packet.promotion = CertificationPromotionRecord {
        promotion_gate: "m5_record_governance_promotion".to_owned(),
        decision,
        rationale: match decision {
            CertificationPromotionDecision::Promote => {
                "Every release-blocking M5 record-governance family is certified with current \
                 proof across all five lane dimensions."
                    .to_owned()
            }
            CertificationPromotionDecision::Hold => format!(
                "{} release-blocking family/families narrowed; promotion holds until proof is \
                 refreshed.",
                blocking.len()
            ),
        },
        blocking_entry_ids: blocking,
    };
    packet.summary = packet.computed_summary();
    packet
}

/// Returns the canonical record class expected for a governed family.
///
/// Mirrors the records/policy matrix mapping so the certification stays aligned
/// with the registry without importing the matrix's private helper.
pub const fn certified_record_class_for_family(family: GovernedArtifactFamily) -> RecordClassId {
    match family {
        GovernedArtifactFamily::AiEvidencePacket => RecordClassId::AiRetainedEvidencePacket,
        GovernedArtifactFamily::ProviderLinkedWorkItem => {
            RecordClassId::ProviderLinkedWorkItemRecord
        }
        GovernedArtifactFamily::CompanionContinuityPacket => {
            RecordClassId::CompanionContinuityPacket
        }
        GovernedArtifactFamily::IncidentSupportPacket => RecordClassId::IncidentSupportPacket,
        GovernedArtifactFamily::SyncMirrorLedger => RecordClassId::SyncMirrorLedger,
        GovernedArtifactFamily::OffboardingRecord => RecordClassId::OffboardingExitPacket,
        GovernedArtifactFamily::BrowserHandoffManifest => RecordClassId::BrowserHandoffManifest,
        GovernedArtifactFamily::SupportExportPacket => RecordClassId::SupportExportPacket,
    }
}
