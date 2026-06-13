//! Canonical records, deletion-honesty, chronology, and policy-simulation matrix.
//!
//! This module loads the checked-in governance matrix that freezes how durable
//! managed/provider/support artifact families disclose record class, managed
//! versus local authority, delete/export honesty, chronology, and policy
//! simulation. The packet is metadata-only and is intended to be the first
//! authoritative input for product, CLI/headless, help/docs, support-export,
//! and release-evidence consumers.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::stabilize_record_class_registry_legal_hold_delete_honesty::RecordOperationOutcome;
use crate::{
    current_registry, validate_typed, LocalVsManagedCopy, ManagedCopyPosture, RecordClassId,
    RecordClassScope, RecordRegistryError,
};

#[cfg(test)]
mod tests;

/// Supported schema version for the checked-in matrix.
pub const RECORDS_POLICY_SIMULATION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for the checked-in matrix.
pub const RECORDS_POLICY_SIMULATION_MATRIX_RECORD_KIND: &str = "records_policy_simulation_matrix";

/// Repo-relative path to the checked-in matrix.
pub const RECORDS_POLICY_SIMULATION_MATRIX_PATH: &str =
    "artifacts/governance/records_policy_simulation_matrix.yaml";

const RECORDS_POLICY_SIMULATION_MATRIX_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/records_policy_simulation_matrix.yaml"
));

/// The durable artifact family a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernedArtifactFamily {
    /// Retained AI evidence, review packets, and provider result lineage.
    AiEvidencePacket,
    /// Provider-linked work-item and mutation-review records.
    ProviderLinkedWorkItem,
    /// Companion continuity, staged handoff, or mirrored session packets.
    CompanionContinuityPacket,
    /// Incident, runbook, and support handoff packets.
    IncidentSupportPacket,
    /// Managed sync continuity, mirror ledger, or device-registry records.
    SyncMirrorLedger,
    /// Offboarding and access-end records.
    OffboardingRecord,
    /// Browser or external-browser handoff manifests and audit envelopes.
    BrowserHandoffManifest,
    /// Support-export packets and their managed retained copies.
    SupportExportPacket,
}

impl GovernedArtifactFamily {
    /// Every governed family in canonical order.
    pub const ALL: [Self; 8] = [
        Self::AiEvidencePacket,
        Self::ProviderLinkedWorkItem,
        Self::CompanionContinuityPacket,
        Self::IncidentSupportPacket,
        Self::SyncMirrorLedger,
        Self::OffboardingRecord,
        Self::BrowserHandoffManifest,
        Self::SupportExportPacket,
    ];

    /// Stable token recorded by the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiEvidencePacket => "ai_evidence_packet",
            Self::ProviderLinkedWorkItem => "provider_linked_work_item",
            Self::CompanionContinuityPacket => "companion_continuity_packet",
            Self::IncidentSupportPacket => "incident_support_packet",
            Self::SyncMirrorLedger => "sync_mirror_ledger",
            Self::OffboardingRecord => "offboarding_record",
            Self::BrowserHandoffManifest => "browser_handoff_manifest",
            Self::SupportExportPacket => "support_export_packet",
        }
    }
}

/// Consumer surface that must ingest the checked-in matrix directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceClass {
    /// Product-facing rows, cards, or status surfaces.
    Product,
    /// CLI or headless explain output.
    CliHeadless,
    /// Help, docs, or operator-facing guidance.
    HelpDocs,
    /// Support-export packets and incident handoff.
    SupportExport,
    /// Release evidence and claim publication.
    ReleaseEvidence,
}

impl ConsumerSurfaceClass {
    /// Every required consumer surface.
    pub const ALL: [Self; 5] = [
        Self::Product,
        Self::CliHeadless,
        Self::HelpDocs,
        Self::SupportExport,
        Self::ReleaseEvidence,
    ];
}

/// Boundary class describing where the platform has authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityBoundaryClass {
    /// The platform only has local-device truth.
    LocalOnly,
    /// The platform only has managed retained truth.
    ManagedOnly,
    /// Local and managed copies both exist and must stay distinct.
    LocalAndManaged,
}

/// Chronology evidence class a governed row may surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyEvidenceClass {
    /// Live in-product chronology.
    Live,
    /// Imported or offline evidence chronology.
    Imported,
    /// Reconstructed chronology derived from logs or receipts.
    Reconstructed,
}

impl ChronologyEvidenceClass {
    /// Every chronology evidence class.
    pub const ALL: [Self; 3] = [Self::Live, Self::Imported, Self::Reconstructed];
}

/// Freshness state of the proof backing one governed row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofFreshnessClass {
    /// Backing proof is current.
    Current,
    /// Backing proof exists but is stale.
    Stale,
    /// Backing proof is absent.
    Missing,
}

impl ProofFreshnessClass {
    /// Returns the stable token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }
}

/// Published qualification carried by one governed row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordsPolicyQualificationClass {
    /// The row is fully backed and may hold its published claim.
    Stable,
    /// The row is narrowed and must be reviewed before any widening.
    NeedsReview,
}

impl RecordsPolicyQualificationClass {
    /// Returns the stable token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NeedsReview => "needs_review",
        }
    }

    /// Returns `true` when the row is fully backed.
    pub const fn holds_claim(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Closed reason a governed row is narrowed or blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordsPolicyGapReason {
    /// The row overclaims a managed delete for local-only material.
    ManagedDeleteOverclaimed,
    /// The row overclaims a managed export for local-only material.
    ManagedExportOverclaimed,
    /// The row overclaims a managed hold for local-only material.
    ManagedHoldOverclaimed,
    /// The row omits required chronology truth.
    ChronologyContractIncomplete,
    /// The row omits required policy-simulation coverage.
    PolicySimulationMissing,
    /// The row omits required remembered-decision revalidation coverage.
    RememberedDecisionRevalidationMissing,
    /// The row's backing proof packet is stale.
    ProofStale,
    /// The row's backing proof packet is missing.
    ProofMissing,
    /// Required consumer coverage is missing.
    ConsumerCoverageMissing,
}

impl RecordsPolicyGapReason {
    /// Every gap reason in canonical order.
    pub const ALL: [Self; 9] = [
        Self::ManagedDeleteOverclaimed,
        Self::ManagedExportOverclaimed,
        Self::ManagedHoldOverclaimed,
        Self::ChronologyContractIncomplete,
        Self::PolicySimulationMissing,
        Self::RememberedDecisionRevalidationMissing,
        Self::ProofStale,
        Self::ProofMissing,
        Self::ConsumerCoverageMissing,
    ];

    /// Returns the stable token for this gap.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedDeleteOverclaimed => "managed_delete_overclaimed",
            Self::ManagedExportOverclaimed => "managed_export_overclaimed",
            Self::ManagedHoldOverclaimed => "managed_hold_overclaimed",
            Self::ChronologyContractIncomplete => "chronology_contract_incomplete",
            Self::PolicySimulationMissing => "policy_simulation_missing",
            Self::RememberedDecisionRevalidationMissing => {
                "remembered_decision_revalidation_missing"
            }
            Self::ProofStale => "proof_stale",
            Self::ProofMissing => "proof_missing",
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
        }
    }
}

/// Overall publication verdict for the matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordsPolicyPublicationDecision {
    /// No release-blocking row is narrowed.
    Proceed,
    /// One or more release-blocking rows are narrowed.
    Hold,
}

impl RecordsPolicyPublicationDecision {
    /// Returns the stable token for this decision.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Hold => "hold",
        }
    }
}

/// One consumer binding that must ingest the packet directly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsumerBinding {
    /// The consumer surface.
    pub surface_class: ConsumerSurfaceClass,
    /// Stable consumer reference.
    pub consumer_ref: String,
    /// Reviewable explanation of how the consumer ingests the packet.
    pub projection_rule: String,
}

/// Record-class descriptor exported for one governed row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordClassDescriptor {
    /// Stable record class governing the row.
    pub record_class_id: RecordClassId,
    /// Registry row reference that defines the class.
    pub registry_row_ref: String,
    /// Scope family declared by the registry.
    pub class_scope: RecordClassScope,
    /// Residency scope disclosed for this class.
    pub residency_scope: String,
    /// Whether the class may participate in hold semantics.
    pub hold_eligible: bool,
    /// Redaction profile disclosed for the class.
    pub redaction_profile: String,
    /// Reviewable managed-copy disclosure copied from the registry row.
    pub managed_copy_label: String,
}

/// Retention-policy assignment projected for one governed row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetentionPolicyAssignment {
    /// Stable assignment id.
    pub assignment_id: String,
    /// Policy family governing the row.
    pub policy_id: String,
    /// Policy version or epoch applied to the row.
    pub policy_version: String,
    /// Scope selector naming the retained object family.
    pub scope_selector: String,
    /// Local retention owner reference.
    pub local_owner_ref: String,
    /// Managed retention owner reference.
    pub managed_owner_ref: String,
    /// Default retention trigger or duration rule.
    pub retention_rule: String,
    /// Delete, tombstone, or archive action taken when retention ends.
    pub delete_action: String,
    /// Grace or queueing rule before terminal destruction.
    pub grace_rule: String,
}

/// Chronology contract a governed row must honor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologyContract {
    /// Stable row id for the chronology surface.
    pub row_id: String,
    /// Absolute time must always be preserved.
    pub absolute_time_required: bool,
    /// Local timezone context must be preserved.
    pub local_context_required: bool,
    /// Source/origin context must be preserved.
    pub source_required: bool,
    /// Actor lineage must be preserved.
    pub actor_lineage_required: bool,
    /// Field token used to disclose imported/live chronology class.
    pub imported_live_field: String,
    /// Chronology evidence classes the row can disclose.
    pub chronology_classes: Vec<ChronologyEvidenceClass>,
}

impl ChronologyContract {
    /// Returns `true` when the contract preserves the required chronology fields.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && self.absolute_time_required
            && self.local_context_required
            && self.source_required
            && self.actor_lineage_required
            && !self.imported_live_field.trim().is_empty()
            && self
                .chronology_classes
                .contains(&ChronologyEvidenceClass::Live)
            && self
                .chronology_classes
                .contains(&ChronologyEvidenceClass::Imported)
    }
}

/// Policy-simulation contract attached to a governed row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyContract {
    /// Owner controlling the next policy step for this row.
    pub policy_owner_ref: String,
    /// `true` when policy simulation must exist for this row.
    pub policy_simulation_required: bool,
    /// `true` when exception preview must exist for this row.
    pub exception_preview_required: bool,
    /// `true` when remembered-decision revalidation must exist for this row.
    pub remembered_decision_revalidation_required: bool,
    /// Stable trigger tokens that force reapproval.
    pub required_reapproval_trigger_tokens: Vec<String>,
}

/// One governed row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordsPolicyMatrixRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The governed artifact family.
    pub artifact_family: GovernedArtifactFamily,
    /// Whether this row is release-blocking.
    pub release_blocking: bool,
    /// Stable producer record kinds that must validate against the class.
    pub producer_record_kinds: Vec<String>,
    /// Record class governing the row.
    pub record_class_id: RecordClassId,
    /// Full record-class descriptor exported for this row.
    pub record_class_descriptor: RecordClassDescriptor,
    /// Retention-policy assignment exported for this row.
    pub retention_policy_assignment: RetentionPolicyAssignment,
    /// Where the platform's destructive/export authority lives.
    pub authority_boundary: AuthorityBoundaryClass,
    /// Local versus managed truth relationship.
    pub local_truth_authority: LocalVsManagedCopy,
    /// Managed-copy posture.
    pub managed_copy_posture: ManagedCopyPosture,
    /// Local retention owner.
    pub local_owner_ref: String,
    /// Managed retention owner.
    pub managed_owner_ref: String,
    /// `true` when a managed hold may be claimed for this row.
    pub claims_managed_hold: bool,
    /// `true` when a managed export may be claimed for this row.
    pub claims_managed_export: bool,
    /// `true` when a managed delete may be claimed for this row.
    pub claims_managed_delete: bool,
    /// Primary delete-honesty posture projected for this row.
    pub delete_state: RecordOperationOutcome,
    /// Primary export-honesty posture projected for this row.
    pub export_state: RecordOperationOutcome,
    /// Chronology contract for this row.
    pub chronology_contract: ChronologyContract,
    /// Policy-simulation contract for this row.
    pub policy_contract: PolicyContract,
    /// Proof packet or artifact backing this row.
    pub proof_ref: String,
    /// Freshness state of the backing proof.
    pub proof_freshness: ProofFreshnessClass,
    /// Published qualification after narrowing.
    pub published_qualification: RecordsPolicyQualificationClass,
    /// Active gap reasons.
    #[serde(default)]
    pub active_gap_reasons: Vec<RecordsPolicyGapReason>,
    /// Help/docs references consumers should cite.
    #[serde(default)]
    pub docs_refs: Vec<String>,
    /// Reviewable rationale for the row.
    pub rationale: String,
}

impl RecordsPolicyMatrixRow {
    /// Returns `true` when the row is narrowed below its claimed posture.
    pub const fn needs_review(&self) -> bool {
        !self.published_qualification.holds_claim()
    }
}

/// Publication record for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordsPolicyPublicationRecord {
    /// Publication gate name.
    pub publication_gate: String,
    /// Proceed or hold decision.
    pub decision: RecordsPolicyPublicationDecision,
    /// Release-blocking rows that currently block publication.
    pub blocking_row_ids: Vec<String>,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Roll-up counts for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordsPolicySummary {
    /// Total governed rows.
    pub total_rows: usize,
    /// Distinct governed families covered.
    pub total_families: usize,
    /// Release-blocking rows.
    pub release_blocking_rows: usize,
    /// Stable rows.
    pub stable_rows: usize,
    /// Narrowed rows.
    pub needs_review_rows: usize,
    /// Rows whose proof is current.
    pub proof_current_rows: usize,
    /// Rows whose proof is stale.
    pub proof_stale_rows: usize,
    /// Rows whose proof is missing.
    pub proof_missing_rows: usize,
    /// Active gap reasons across the matrix.
    pub total_active_gap_reasons: usize,
    /// Consumer bindings declared by the matrix.
    pub consumer_binding_count: usize,
}

/// Checked-in records/policy governance matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordsPolicySimulationMatrix {
    /// Schema version.
    pub schema_version: u32,
    /// Stable record kind.
    pub record_kind: String,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// UTC date the packet reflects.
    pub as_of: String,
    /// Human-readable status.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Companion artifact summary.
    pub artifact_summary_ref: String,
    /// Required consumer bindings.
    pub consumer_bindings: Vec<ConsumerBinding>,
    /// Governed family vocabulary.
    pub artifact_families: Vec<GovernedArtifactFamily>,
    /// Consumer-surface vocabulary.
    pub consumer_surfaces: Vec<ConsumerSurfaceClass>,
    /// Chronology vocabulary.
    pub chronology_classes: Vec<ChronologyEvidenceClass>,
    /// Proof-freshness vocabulary.
    pub proof_freshness_states: Vec<ProofFreshnessClass>,
    /// Qualification vocabulary.
    pub qualification_states: Vec<RecordsPolicyQualificationClass>,
    /// Gap-reason vocabulary.
    pub gap_reasons: Vec<RecordsPolicyGapReason>,
    /// Release-blocking family refs.
    pub release_blocking_family_refs: Vec<String>,
    /// Governed rows.
    pub rows: Vec<RecordsPolicyMatrixRow>,
    /// Publication verdict.
    pub publication: RecordsPolicyPublicationRecord,
    /// Roll-up counts.
    pub summary: RecordsPolicySummary,
}

/// Narrow projection used by CLI/headless surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliHeadlessProjectionRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Governing record class.
    pub record_class_id: RecordClassId,
    /// Retention assignment id.
    pub retention_assignment_id: String,
    /// Chronology row id.
    pub chronology_row_id: String,
    /// Where destructive/export authority lives.
    pub authority_boundary: AuthorityBoundaryClass,
    /// Published qualification.
    pub qualification: RecordsPolicyQualificationClass,
    /// Delete-honesty posture.
    pub delete_state: RecordOperationOutcome,
    /// Export-honesty posture.
    pub export_state: RecordOperationOutcome,
    /// Proof freshness state.
    pub proof_freshness: ProofFreshnessClass,
    /// Active narrowing reasons.
    pub active_gap_reasons: Vec<RecordsPolicyGapReason>,
}

/// Help/docs projection over one governed row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpDocsProjectionRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Title shown in docs/help.
    pub title: String,
    /// Governing record class.
    pub record_class_id: RecordClassId,
    /// Reviewable managed-copy disclosure.
    pub managed_copy_label: String,
    /// Help/docs references that should cite the row.
    pub docs_refs: Vec<String>,
    /// Reviewable rationale for the row.
    pub rationale: String,
}

/// Release-evidence projection over one governed row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseEvidenceProjectionRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Governing record class.
    pub record_class_id: RecordClassId,
    /// Whether the row is release-blocking.
    pub release_blocking: bool,
    /// Published qualification.
    pub qualification: RecordsPolicyQualificationClass,
    /// Proof freshness state.
    pub proof_freshness: ProofFreshnessClass,
    /// Proof reference surfaced into release evidence.
    pub proof_ref: String,
}

/// Product-facing projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductProjectionRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// Governing record class.
    pub record_class_id: RecordClassId,
    /// Retention assignment id.
    pub retention_assignment_id: String,
    /// Chronology row id.
    pub chronology_row_id: String,
    /// Local versus managed authority boundary.
    pub authority_boundary: AuthorityBoundaryClass,
    /// Primary delete-honesty posture.
    pub delete_state: RecordOperationOutcome,
    /// Primary export-honesty posture.
    pub export_state: RecordOperationOutcome,
}

/// Support/export projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportProjectionRow {
    /// Stable entry id.
    pub entry_id: String,
    /// Governing record class.
    pub record_class_id: RecordClassId,
    /// Stable producer record kinds covered by the row.
    pub producer_record_kinds: Vec<String>,
    /// Retention assignment id.
    pub retention_assignment_id: String,
    /// Local retention owner reference.
    pub local_owner_ref: String,
    /// Managed retention owner reference.
    pub managed_owner_ref: String,
    /// Chronology row id.
    pub chronology_row_id: String,
    /// Imported/live disclosure field.
    pub imported_live_field: String,
    /// Where destructive/export authority lives.
    pub authority_boundary: AuthorityBoundaryClass,
    /// Local versus managed truth relationship.
    pub local_truth_authority: LocalVsManagedCopy,
    /// Managed-copy posture.
    pub managed_copy_posture: ManagedCopyPosture,
    /// Delete-honesty posture.
    pub delete_state: RecordOperationOutcome,
    /// Export-honesty posture.
    pub export_state: RecordOperationOutcome,
}

/// Validation violation emitted by the typed model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code", content = "detail")]
pub enum RecordsPolicyMatrixViolation {
    /// Schema version mismatch.
    SchemaVersionMismatch { found: u32 },
    /// Record kind mismatch.
    RecordKindMismatch { found: String },
    /// A required family is uncovered.
    ArtifactFamilyUncovered {
        artifact_family: GovernedArtifactFamily,
    },
    /// A required consumer surface is not bound.
    ConsumerSurfaceUnbound { surface_class: ConsumerSurfaceClass },
    /// A row names an unknown record class.
    UnknownRecordClass {
        entry_id: String,
        record_class_id: RecordClassId,
    },
    /// A governed family is mapped to the wrong record class.
    WrongRecordClassForFamily {
        entry_id: String,
        expected: RecordClassId,
        found: RecordClassId,
    },
    /// A descriptor field drifts from the registry row.
    RecordDescriptorMismatch { entry_id: String, field: String },
    /// A producer record kind is missing.
    ProducerRecordKindsMissing { entry_id: String },
    /// A producer record kind is not registered to the row's class.
    ProducerRecordKindMismatch {
        entry_id: String,
        record_kind: String,
        message: String,
    },
    /// A local-only row overclaims a managed control.
    ManagedControlOverclaimed { entry_id: String, control: String },
    /// A row omits required chronology truth.
    ChronologyContractIncomplete { entry_id: String },
    /// A retention assignment drifts from the row's declared owners.
    RetentionAssignmentMismatch { entry_id: String, field: String },
    /// A row requires policy governance but omits its owner.
    PolicyOwnerMissing { entry_id: String },
    /// A row requires remembered-decision revalidation but omits a trigger.
    ReapprovalTriggerMissing { entry_id: String, trigger: String },
    /// A stable row still carries active gap reasons.
    StableRowHasGap { entry_id: String },
    /// A narrowed row carries no active gap reasons.
    NarrowedRowWithoutGap { entry_id: String },
    /// Proof freshness and gap reasons disagree.
    ProofFreshnessMismatch { entry_id: String },
    /// Publication decision disagrees with the computed decision.
    PublicationDecisionMismatch {
        found: RecordsPolicyPublicationDecision,
        expected: RecordsPolicyPublicationDecision,
    },
    /// Summary roll-up disagrees with the computed roll-up.
    SummaryMismatch { field: String },
}

impl RecordsPolicySimulationMatrix {
    /// Validates structural and logical invariants of the checked-in matrix.
    pub fn validate(&self) -> Vec<RecordsPolicyMatrixViolation> {
        let mut violations = Vec::new();

        if self.schema_version != RECORDS_POLICY_SIMULATION_MATRIX_SCHEMA_VERSION {
            violations.push(RecordsPolicyMatrixViolation::SchemaVersionMismatch {
                found: self.schema_version,
            });
        }
        if self.record_kind != RECORDS_POLICY_SIMULATION_MATRIX_RECORD_KIND {
            violations.push(RecordsPolicyMatrixViolation::RecordKindMismatch {
                found: self.record_kind.clone(),
            });
        }

        let registry = current_registry();

        let covered_families: BTreeSet<GovernedArtifactFamily> =
            self.rows.iter().map(|row| row.artifact_family).collect();
        for family in GovernedArtifactFamily::ALL {
            if !covered_families.contains(&family) {
                violations.push(RecordsPolicyMatrixViolation::ArtifactFamilyUncovered {
                    artifact_family: family,
                });
            }
        }

        let bound_surfaces: BTreeSet<ConsumerSurfaceClass> = self
            .consumer_bindings
            .iter()
            .map(|binding| binding.surface_class)
            .collect();
        for surface in ConsumerSurfaceClass::ALL {
            if !bound_surfaces.contains(&surface) {
                violations.push(RecordsPolicyMatrixViolation::ConsumerSurfaceUnbound {
                    surface_class: surface,
                });
            }
        }

        for row in &self.rows {
            if let Ok(registry) = &registry {
                let expected_record_class = canonical_record_class_for_family(row.artifact_family);
                if row.record_class_id != expected_record_class {
                    violations.push(RecordsPolicyMatrixViolation::WrongRecordClassForFamily {
                        entry_id: row.entry_id.clone(),
                        expected: expected_record_class,
                        found: row.record_class_id,
                    });
                }

                let Some(registry_row) = registry.row(row.record_class_id) else {
                    violations.push(RecordsPolicyMatrixViolation::UnknownRecordClass {
                        entry_id: row.entry_id.clone(),
                        record_class_id: row.record_class_id,
                    });
                    continue;
                };

                if row.record_class_descriptor.record_class_id != row.record_class_id {
                    violations.push(RecordsPolicyMatrixViolation::RecordDescriptorMismatch {
                        entry_id: row.entry_id.clone(),
                        field: "record_class_id".to_owned(),
                    });
                }
                if row.record_class_descriptor.class_scope != registry_row.class_scope {
                    violations.push(RecordsPolicyMatrixViolation::RecordDescriptorMismatch {
                        entry_id: row.entry_id.clone(),
                        field: "class_scope".to_owned(),
                    });
                }
                if row
                    .record_class_descriptor
                    .residency_scope
                    .trim()
                    .is_empty()
                    || registry_row.residency_scope.as_deref()
                        != Some(row.record_class_descriptor.residency_scope.as_str())
                {
                    violations.push(RecordsPolicyMatrixViolation::RecordDescriptorMismatch {
                        entry_id: row.entry_id.clone(),
                        field: "residency_scope".to_owned(),
                    });
                }
                if row.record_class_descriptor.hold_eligible
                    != registry_row.hold_semantics.eligible.as_bool()
                {
                    violations.push(RecordsPolicyMatrixViolation::RecordDescriptorMismatch {
                        entry_id: row.entry_id.clone(),
                        field: "hold_eligible".to_owned(),
                    });
                }
                if row
                    .record_class_descriptor
                    .redaction_profile
                    .trim()
                    .is_empty()
                    || registry_row.redaction_profile.as_deref()
                        != Some(row.record_class_descriptor.redaction_profile.as_str())
                {
                    violations.push(RecordsPolicyMatrixViolation::RecordDescriptorMismatch {
                        entry_id: row.entry_id.clone(),
                        field: "redaction_profile".to_owned(),
                    });
                }
                if row
                    .record_class_descriptor
                    .managed_copy_label
                    .trim()
                    .is_empty()
                    || normalized_whitespace(&row.record_class_descriptor.managed_copy_label)
                        != normalized_whitespace(&registry_row.local_truth.managed_copy_label)
                {
                    violations.push(RecordsPolicyMatrixViolation::RecordDescriptorMismatch {
                        entry_id: row.entry_id.clone(),
                        field: "managed_copy_label".to_owned(),
                    });
                }

                if row.producer_record_kinds.is_empty() {
                    violations.push(RecordsPolicyMatrixViolation::ProducerRecordKindsMissing {
                        entry_id: row.entry_id.clone(),
                    });
                } else {
                    for record_kind in &row.producer_record_kinds {
                        if let Err(error) = validate_typed(record_kind, row.record_class_id) {
                            violations.push(
                                RecordsPolicyMatrixViolation::ProducerRecordKindMismatch {
                                    entry_id: row.entry_id.clone(),
                                    record_kind: record_kind.clone(),
                                    message: error.to_string(),
                                },
                            );
                        }
                    }
                }
            }

            if row.retention_policy_assignment.local_owner_ref != row.local_owner_ref {
                violations.push(RecordsPolicyMatrixViolation::RetentionAssignmentMismatch {
                    entry_id: row.entry_id.clone(),
                    field: "local_owner_ref".to_owned(),
                });
            }
            if row.retention_policy_assignment.managed_owner_ref != row.managed_owner_ref {
                violations.push(RecordsPolicyMatrixViolation::RetentionAssignmentMismatch {
                    entry_id: row.entry_id.clone(),
                    field: "managed_owner_ref".to_owned(),
                });
            }

            if row.authority_boundary == AuthorityBoundaryClass::LocalOnly {
                if row.claims_managed_delete {
                    violations.push(RecordsPolicyMatrixViolation::ManagedControlOverclaimed {
                        entry_id: row.entry_id.clone(),
                        control: "managed_delete".to_owned(),
                    });
                }
                if row.claims_managed_export {
                    violations.push(RecordsPolicyMatrixViolation::ManagedControlOverclaimed {
                        entry_id: row.entry_id.clone(),
                        control: "managed_export".to_owned(),
                    });
                }
                if row.claims_managed_hold {
                    violations.push(RecordsPolicyMatrixViolation::ManagedControlOverclaimed {
                        entry_id: row.entry_id.clone(),
                        control: "managed_hold".to_owned(),
                    });
                }
            }

            if !row.chronology_contract.is_complete() {
                violations.push(RecordsPolicyMatrixViolation::ChronologyContractIncomplete {
                    entry_id: row.entry_id.clone(),
                });
            }

            let policy_required = row.policy_contract.policy_simulation_required
                || row.policy_contract.exception_preview_required
                || row
                    .policy_contract
                    .remembered_decision_revalidation_required;
            if policy_required && row.policy_contract.policy_owner_ref.trim().is_empty() {
                violations.push(RecordsPolicyMatrixViolation::PolicyOwnerMissing {
                    entry_id: row.entry_id.clone(),
                });
            }

            if row
                .policy_contract
                .remembered_decision_revalidation_required
            {
                for trigger in REQUIRED_REAPPROVAL_TRIGGER_TOKENS {
                    if !row
                        .policy_contract
                        .required_reapproval_trigger_tokens
                        .iter()
                        .any(|candidate| candidate == trigger)
                    {
                        violations.push(RecordsPolicyMatrixViolation::ReapprovalTriggerMissing {
                            entry_id: row.entry_id.clone(),
                            trigger: (*trigger).to_string(),
                        });
                    }
                }
            }

            match row.published_qualification {
                RecordsPolicyQualificationClass::Stable => {
                    if !row.active_gap_reasons.is_empty() {
                        violations.push(RecordsPolicyMatrixViolation::StableRowHasGap {
                            entry_id: row.entry_id.clone(),
                        });
                    }
                }
                RecordsPolicyQualificationClass::NeedsReview => {
                    if row.active_gap_reasons.is_empty() {
                        violations.push(RecordsPolicyMatrixViolation::NarrowedRowWithoutGap {
                            entry_id: row.entry_id.clone(),
                        });
                    }
                }
            }

            let proof_gap_expected = match row.proof_freshness {
                ProofFreshnessClass::Current => None,
                ProofFreshnessClass::Stale => Some(RecordsPolicyGapReason::ProofStale),
                ProofFreshnessClass::Missing => Some(RecordsPolicyGapReason::ProofMissing),
            };
            let proof_gap_present = row.active_gap_reasons.iter().copied().find(|gap| {
                matches!(
                    gap,
                    RecordsPolicyGapReason::ProofStale | RecordsPolicyGapReason::ProofMissing
                )
            });
            if proof_gap_expected != proof_gap_present {
                violations.push(RecordsPolicyMatrixViolation::ProofFreshnessMismatch {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        let expected_decision = self.computed_publication_decision();
        if self.publication.decision != expected_decision {
            violations.push(RecordsPolicyMatrixViolation::PublicationDecisionMismatch {
                found: self.publication.decision,
                expected: expected_decision,
            });
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
                "stable_rows",
                self.summary.stable_rows == computed_summary.stable_rows,
            ),
            (
                "needs_review_rows",
                self.summary.needs_review_rows == computed_summary.needs_review_rows,
            ),
            (
                "proof_current_rows",
                self.summary.proof_current_rows == computed_summary.proof_current_rows,
            ),
            (
                "proof_stale_rows",
                self.summary.proof_stale_rows == computed_summary.proof_stale_rows,
            ),
            (
                "proof_missing_rows",
                self.summary.proof_missing_rows == computed_summary.proof_missing_rows,
            ),
            (
                "total_active_gap_reasons",
                self.summary.total_active_gap_reasons == computed_summary.total_active_gap_reasons,
            ),
            (
                "consumer_binding_count",
                self.summary.consumer_binding_count == computed_summary.consumer_binding_count,
            ),
        ] {
            if !ok {
                violations.push(RecordsPolicyMatrixViolation::SummaryMismatch {
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
    ) -> Option<&RecordsPolicyMatrixRow> {
        self.rows
            .iter()
            .find(|row| row.artifact_family == artifact_family)
    }

    /// Recomputes the publication decision from the rows alone.
    pub fn computed_publication_decision(&self) -> RecordsPolicyPublicationDecision {
        if self
            .rows
            .iter()
            .any(|row| row.release_blocking && row.needs_review())
        {
            RecordsPolicyPublicationDecision::Hold
        } else {
            RecordsPolicyPublicationDecision::Proceed
        }
    }

    /// Recomputes the summary from the rows and consumer bindings.
    pub fn computed_summary(&self) -> RecordsPolicySummary {
        RecordsPolicySummary {
            total_rows: self.rows.len(),
            total_families: self
                .rows
                .iter()
                .map(|row| row.artifact_family)
                .collect::<BTreeSet<_>>()
                .len(),
            release_blocking_rows: self.rows.iter().filter(|row| row.release_blocking).count(),
            stable_rows: self
                .rows
                .iter()
                .filter(|row| {
                    row.published_qualification == RecordsPolicyQualificationClass::Stable
                })
                .count(),
            needs_review_rows: self
                .rows
                .iter()
                .filter(|row| {
                    row.published_qualification == RecordsPolicyQualificationClass::NeedsReview
                })
                .count(),
            proof_current_rows: self
                .rows
                .iter()
                .filter(|row| row.proof_freshness == ProofFreshnessClass::Current)
                .count(),
            proof_stale_rows: self
                .rows
                .iter()
                .filter(|row| row.proof_freshness == ProofFreshnessClass::Stale)
                .count(),
            proof_missing_rows: self
                .rows
                .iter()
                .filter(|row| row.proof_freshness == ProofFreshnessClass::Missing)
                .count(),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            consumer_binding_count: self.consumer_bindings.len(),
        }
    }

    /// Returns the CLI/headless projection rows.
    pub fn cli_headless_projection(&self) -> Vec<CliHeadlessProjectionRow> {
        self.rows
            .iter()
            .map(|row| CliHeadlessProjectionRow {
                entry_id: row.entry_id.clone(),
                artifact_family: row.artifact_family,
                record_class_id: row.record_class_id,
                retention_assignment_id: row.retention_policy_assignment.assignment_id.clone(),
                chronology_row_id: row.chronology_contract.row_id.clone(),
                authority_boundary: row.authority_boundary,
                qualification: row.published_qualification,
                delete_state: row.delete_state,
                export_state: row.export_state,
                proof_freshness: row.proof_freshness,
                active_gap_reasons: row.active_gap_reasons.clone(),
            })
            .collect()
    }

    /// Returns the help/docs projection rows.
    pub fn help_docs_projection(&self) -> Vec<HelpDocsProjectionRow> {
        self.rows
            .iter()
            .map(|row| HelpDocsProjectionRow {
                entry_id: row.entry_id.clone(),
                title: row.title.clone(),
                record_class_id: row.record_class_id,
                managed_copy_label: row.record_class_descriptor.managed_copy_label.clone(),
                docs_refs: row.docs_refs.clone(),
                rationale: row.rationale.clone(),
            })
            .collect()
    }

    /// Returns the release-evidence projection rows.
    pub fn release_evidence_projection(&self) -> Vec<ReleaseEvidenceProjectionRow> {
        self.rows
            .iter()
            .map(|row| ReleaseEvidenceProjectionRow {
                entry_id: row.entry_id.clone(),
                artifact_family: row.artifact_family,
                record_class_id: row.record_class_id,
                release_blocking: row.release_blocking,
                qualification: row.published_qualification,
                proof_freshness: row.proof_freshness,
                proof_ref: row.proof_ref.clone(),
            })
            .collect()
    }

    /// Returns the product-facing projection rows.
    pub fn product_projection(&self) -> Vec<ProductProjectionRow> {
        self.rows
            .iter()
            .map(|row| ProductProjectionRow {
                entry_id: row.entry_id.clone(),
                title: row.title.clone(),
                record_class_id: row.record_class_id,
                retention_assignment_id: row.retention_policy_assignment.assignment_id.clone(),
                chronology_row_id: row.chronology_contract.row_id.clone(),
                authority_boundary: row.authority_boundary,
                delete_state: row.delete_state,
                export_state: row.export_state,
            })
            .collect()
    }

    /// Returns the support/export projection rows.
    pub fn support_export_projection(&self) -> Vec<SupportExportProjectionRow> {
        self.rows
            .iter()
            .map(|row| SupportExportProjectionRow {
                entry_id: row.entry_id.clone(),
                record_class_id: row.record_class_id,
                producer_record_kinds: row.producer_record_kinds.clone(),
                retention_assignment_id: row.retention_policy_assignment.assignment_id.clone(),
                local_owner_ref: row.retention_policy_assignment.local_owner_ref.clone(),
                managed_owner_ref: row.retention_policy_assignment.managed_owner_ref.clone(),
                chronology_row_id: row.chronology_contract.row_id.clone(),
                imported_live_field: row.chronology_contract.imported_live_field.clone(),
                authority_boundary: row.authority_boundary,
                local_truth_authority: row.local_truth_authority,
                managed_copy_posture: row.managed_copy_posture,
                delete_state: row.delete_state,
                export_state: row.export_state,
            })
            .collect()
    }
}

/// Loads the checked-in records/policy governance matrix.
pub fn current_records_policy_matrix() -> Result<RecordsPolicySimulationMatrix, serde_yaml::Error> {
    serde_yaml::from_str(RECORDS_POLICY_SIMULATION_MATRIX_YAML)
}

/// Required remembered-decision reapproval triggers.
pub const REQUIRED_REAPPROVAL_TRIGGER_TOKENS: &[&str] = &[
    "target_drift",
    "policy_drift",
    "version_drift",
    "authority_drift",
];

/// Returns the current registry or forwards the typed load error.
pub fn records_policy_matrix_registry(
) -> Result<&'static crate::RecordClassRegistry, RecordRegistryError> {
    current_registry()
}

const fn canonical_record_class_for_family(family: GovernedArtifactFamily) -> RecordClassId {
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

fn normalized_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}
