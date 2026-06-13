//! Timezone-aware evidence chronology, actor lineage, and source/live/imported
//! labels for the durable M5 evidence families.
//!
//! This module is the canonical *evidence-time* contract for the M5 evidence
//! families: incident support packets, offboarding exit packets, AI retained
//! evidence, managed sync/mirror ledgers, and provider-linked work items. Each
//! family contributes one [`M5EvidenceChronologyRow`] that carries an absolute
//! timestamp, a local time-zone basis, a source class, a live/imported class,
//! and the full [`ActorLineage`] of who originated, relayed, imported, or now
//! holds the evidence. The packet exposes product, admin, and support/export
//! projections that all read the same chronology and lineage vocabulary so
//! support and admin reviewers can reconstruct *what happened, in what order,
//! and from which source class* without guessing from inconsistent timestamps
//! or missing source labels.
//!
//! The packet is metadata-only: lineage steps carry opaque actor refs and
//! review-safe labels but no credential bodies, raw provider payloads, or raw
//! user identifiers. Local-only evidence never claims a remote hold, remote
//! export, or remote delete; those boundaries are stated explicitly and
//! preserved verbatim into every projection.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::stabilize_chronology_grammar_and_history_row_truth::{
    action_label, ActionVerb, ActorKind, ChronologyFreshnessClass, ChronologyImportedClass,
    ChronologySourceClass, ProvenanceBadge, TimePosture,
};

#[cfg(test)]
mod tests;

/// Schema version for the M5 evidence-chronology packet.
pub const M5_EVIDENCE_CHRONOLOGY_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for the top-level packet.
pub const M5_EVIDENCE_CHRONOLOGY_PACKET_RECORD_KIND: &str = "m5_evidence_chronology_packet";

/// Stable record kind for a chronology row.
pub const M5_EVIDENCE_CHRONOLOGY_ROW_RECORD_KIND: &str = "m5_evidence_chronology_row";

/// Shared evidence-time contract reference for new M5 rows.
pub const M5_EVIDENCE_CHRONOLOGY_SHARED_CONTRACT_REF: &str =
    "chronology:m5_evidence_time_lineage:v1";

/// Repo-relative doc reference for the evidence-chronology contract.
pub const M5_EVIDENCE_CHRONOLOGY_DOC_REF: &str =
    "docs/governance/m5_evidence_chronology_lineage.md";

/// Repo-relative artifact summary for the evidence-chronology contract.
pub const M5_EVIDENCE_CHRONOLOGY_ARTIFACT_REF: &str =
    "artifacts/governance/m5_evidence_chronology_lineage.md";

/// Repo-relative schema reference for the evidence-chronology contract.
pub const M5_EVIDENCE_CHRONOLOGY_SCHEMA_REF: &str =
    "schemas/governance/m5_evidence_chronology_lineage.schema.json";

/// Repo-relative fixture directory for the canonical packet.
pub const M5_EVIDENCE_CHRONOLOGY_FIXTURE_DIR: &str =
    "fixtures/governance/m5_evidence_chronology_lineage";

/// M5 evidence family whose chronology this contract governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceWorkflowClass {
    /// Incident workspace and support handoff packets.
    IncidentSupport,
    /// Offboarding and access-end export packets.
    Offboarding,
    /// AI evidence packets retained under managed policy.
    AiEvidence,
    /// Managed sync and mirror ledger records.
    ManagedSync,
    /// Provider-linked work-item mutation and handoff records.
    ProviderLinked,
}

impl M5EvidenceWorkflowClass {
    /// Every governed workflow class, in deterministic order.
    pub const ALL: [Self; 5] = [
        Self::IncidentSupport,
        Self::Offboarding,
        Self::AiEvidence,
        Self::ManagedSync,
        Self::ProviderLinked,
    ];

    /// Returns the stable snake_case token for the workflow class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncidentSupport => "incident_support",
            Self::Offboarding => "offboarding",
            Self::AiEvidence => "ai_evidence",
            Self::ManagedSync => "managed_sync",
            Self::ProviderLinked => "provider_linked",
        }
    }
}

/// Role an actor plays in an evidence lineage chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorLineageRole {
    /// First actor that produced or observed the evidence.
    Originator,
    /// Actor that relayed or forwarded the evidence in place.
    Relay,
    /// Actor that imported the evidence from an external source.
    Importer,
    /// Actor that exported or handed the evidence onward.
    Exporter,
    /// Actor that reviewed the evidence.
    Reviewer,
    /// Actor that approved a state change on the evidence.
    Approver,
    /// Actor that currently holds custody of the evidence.
    Custodian,
}

impl ActorLineageRole {
    /// Returns the stable snake_case token for the role.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Originator => "originator",
            Self::Relay => "relay",
            Self::Importer => "importer",
            Self::Exporter => "exporter",
            Self::Reviewer => "reviewer",
            Self::Approver => "approver",
            Self::Custodian => "custodian",
        }
    }
}

/// Where the authoritative copy of an evidence row lives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceResidencyClass {
    /// The platform only ever possessed a local copy.
    LocalOnly,
    /// The managed service copy is authoritative.
    ManagedCopy,
    /// Local and managed copies both carry distinct truth.
    LocalAndManaged,
}

impl EvidenceResidencyClass {
    /// Returns the stable snake_case token for the residency class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ManagedCopy => "managed_copy",
            Self::LocalAndManaged => "local_and_managed",
        }
    }

    /// Returns true when the platform only knows a local copy of the evidence.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::LocalOnly)
    }
}

/// Absolute time and local context preserved for one lineage hop.
///
/// Lighter than a full [`TimePosture`]: a lineage step preserves the absolute
/// timestamp, the time-zone basis, and the live/imported class for the hop,
/// while the row's [`TimePosture`] carries the relative-age presentation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageStepTime {
    /// Absolute UTC timestamp for the hop.
    pub absolute_timestamp: String,
    /// IANA time zone used by the source or display context.
    pub timezone_iana: String,
    /// UTC offset displayed for the hop.
    pub utc_offset: String,
    /// Local display label for the hop.
    pub local_time_label: String,
    /// Live/imported/cache posture for the hop timestamp.
    pub imported_class: ChronologyImportedClass,
}

/// One actor hop in an evidence lineage chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorLineageStep {
    /// Zero-based position of the hop in the chain.
    pub step_index: u32,
    /// Role this actor played.
    pub role: ActorLineageRole,
    /// Actor class for the hop.
    pub actor_kind: ActorKind,
    /// Stable opaque actor ref when known.
    pub actor_ref: Option<String>,
    /// Privacy-safe actor label.
    pub actor_label: String,
    /// Stable action verb for the hop.
    pub action: ActionVerb,
    /// Source/provenance class for the hop.
    pub source_class: ChronologySourceClass,
    /// Absolute time and local context for the hop.
    pub step_time: LineageStepTime,
    /// Optional review-safe note for the hop.
    pub note: Option<String>,
}

/// Ordered chain of actors that produced, relayed, and now hold the evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorLineage {
    /// Stable lineage id.
    pub lineage_id: String,
    /// Originating actor ref when known.
    pub originator_actor_ref: Option<String>,
    /// Ref for the actor currently holding custody.
    pub current_custodian_ref: String,
    /// Lineage hops in deterministic chronological order.
    pub steps: Vec<ActorLineageStep>,
}

impl ActorLineage {
    /// Returns the originating (first) lineage step.
    pub fn originator(&self) -> Option<&ActorLineageStep> {
        self.steps.first()
    }

    /// Returns the ordered actor refs across the lineage chain.
    pub fn ordered_actor_refs(&self) -> Vec<String> {
        self.steps
            .iter()
            .map(|step| {
                step.actor_ref
                    .clone()
                    .unwrap_or_else(|| format!("unresolved:{}", step.actor_label))
            })
            .collect()
    }
}

/// Canonical timezone-aware evidence chronology row for one M5 family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceChronologyRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Stable event id shared across surfaces.
    pub canonical_event_id: String,
    /// Governed evidence workflow class.
    pub workflow_class: M5EvidenceWorkflowClass,
    /// Primary (originating) actor class.
    pub actor_kind: ActorKind,
    /// Primary (originating) actor label.
    pub actor_label: String,
    /// Stable action verb.
    pub action: ActionVerb,
    /// Privacy-safe object label.
    pub object_label: String,
    /// Privacy-safe outcome label.
    pub outcome_label: String,
    /// Deterministic actor/action/object/outcome sentence.
    pub grammar_sentence: String,
    /// Source/provenance class for the row.
    pub source_class: ChronologySourceClass,
    /// Live/imported class for the row.
    pub imported_class: ChronologyImportedClass,
    /// Source/provenance badges preserved by UI, export, and admin paths.
    pub provenance_badges: Vec<ProvenanceBadge>,
    /// Absolute and relative time posture.
    pub time_posture: TimePosture,
    /// Full actor lineage chain.
    pub actor_lineage: ActorLineage,
    /// Where the authoritative copy lives.
    pub residency: EvidenceResidencyClass,
    /// Whether the row claims a remote (managed) legal hold.
    pub claims_remote_hold: bool,
    /// Whether the row claims a remote (managed) export.
    pub claims_remote_export: bool,
    /// Whether the row claims a remote (managed) delete.
    pub claims_remote_delete: bool,
    /// Explicit boundary note when the evidence is local-only or off-platform.
    pub local_only_boundary_note: Option<String>,
    /// Proof reference backing the row.
    pub proof_ref: String,
}

impl M5EvidenceChronologyRow {
    /// Returns the deterministic actor/action/object/outcome sentence.
    pub fn canonical_sentence(&self) -> String {
        format!(
            "{} {} {}: {}",
            self.actor_label,
            action_label(self.action),
            self.object_label,
            self.outcome_label
        )
    }
}

/// Product-surface projection of an evidence chronology row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductEvidenceRow {
    /// Governed workflow class.
    pub workflow_class: M5EvidenceWorkflowClass,
    /// Canonical event id.
    pub canonical_event_id: String,
    /// Preserved grammar sentence.
    pub grammar_sentence: String,
    /// Source class shown inline.
    pub source_class: ChronologySourceClass,
    /// Live/imported class shown inline.
    pub imported_class: ChronologyImportedClass,
    /// Absolute timestamp.
    pub absolute_timestamp: String,
    /// Relative-age label.
    pub relative_age_label: String,
    /// Residency badge.
    pub residency: EvidenceResidencyClass,
    /// Number of lineage hops.
    pub lineage_step_count: usize,
}

/// Admin-surface projection of an evidence chronology row.
///
/// Carries the full lineage so an admin reviewer can reconstruct actor lineage
/// and chronology without re-deriving it from inconsistent timestamps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminEvidenceRow {
    /// Governed workflow class.
    pub workflow_class: M5EvidenceWorkflowClass,
    /// Canonical event id.
    pub canonical_event_id: String,
    /// Preserved grammar sentence.
    pub grammar_sentence: String,
    /// Source class.
    pub source_class: ChronologySourceClass,
    /// Live/imported class.
    pub imported_class: ChronologyImportedClass,
    /// Absolute timestamp.
    pub absolute_timestamp: String,
    /// IANA time zone basis.
    pub timezone_iana: String,
    /// UTC offset displayed with the row.
    pub utc_offset: String,
    /// Full reconstructed actor lineage.
    pub actor_lineage: ActorLineage,
    /// Residency posture.
    pub residency: EvidenceResidencyClass,
    /// Local-only or off-platform boundary note when present.
    pub local_only_boundary_note: Option<String>,
}

/// Support/export projection of an evidence chronology row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportEvidenceRow {
    /// Governed workflow class.
    pub workflow_class: M5EvidenceWorkflowClass,
    /// Canonical event id.
    pub canonical_event_id: String,
    /// Preserved grammar sentence.
    pub grammar_sentence: String,
    /// Exported provenance markers.
    pub provenance_marker_labels: Vec<String>,
    /// Absolute timestamp.
    pub absolute_timestamp: String,
    /// IANA time zone basis.
    pub timezone_iana: String,
    /// Source class.
    pub source_class: ChronologySourceClass,
    /// Live/imported class.
    pub imported_class: ChronologyImportedClass,
    /// Ordered actor refs across the lineage chain.
    pub lineage_actor_refs: Vec<String>,
    /// Number of lineage hops.
    pub lineage_step_count: usize,
    /// Residency posture.
    pub residency: EvidenceResidencyClass,
    /// Local-only or off-platform boundary note when present.
    pub local_only_boundary_note: Option<String>,
}

/// Top-level canonical M5 evidence-chronology packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceChronologyPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Stable record kind.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Shared evidence-time contract ref.
    pub shared_contract_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// UTC packet timestamp.
    pub as_of: String,
    /// Overview doc ref.
    pub overview_doc_ref: String,
    /// Artifact summary ref.
    pub artifact_summary_ref: String,
    /// Governed family rows.
    pub rows: Vec<M5EvidenceChronologyRow>,
    /// Review-safe summary.
    pub summary: String,
}

/// Validation issue emitted by the M5 evidence-chronology packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code", content = "detail")]
pub enum M5EvidenceChronologyViolation {
    /// Schema version mismatch.
    SchemaVersionMismatch { found: u32 },
    /// Packet record kind mismatch.
    PacketRecordKindMismatch { found: String },
    /// Row record kind mismatch.
    RowRecordKindMismatch { row_id: String, found: String },
    /// Row schema version mismatch.
    RowSchemaVersionMismatch { row_id: String, found: u32 },
    /// A required chronology field was empty.
    RequiredFieldEmpty { row_id: String, field: String },
    /// The grammar sentence was not generated from actor/action/object/outcome.
    GrammarSentenceDrift { row_id: String },
    /// The row carries no provenance badge.
    MissingProvenanceBadge { row_id: String },
    /// A provenance badge dropped its imported/cached/AI/policy marker text.
    ProvenanceExportMarkerDrift { row_id: String },
    /// A stale or expired row failed to explain why it stays visible.
    StaleReasonMissing { row_id: String },
    /// The actor lineage chain was empty.
    EmptyActorLineage { row_id: String },
    /// Lineage steps were not contiguously ordered from zero.
    LineageStepOrderBroken { row_id: String, step_index: u32 },
    /// The first lineage step was not the originator.
    LineageMissingOriginator { row_id: String },
    /// The row's primary actor label disagreed with the originating hop.
    OriginatorActorMismatch { row_id: String },
    /// A local-only row claimed a remote legal hold.
    LocalOnlyClaimsRemoteHold { row_id: String },
    /// A local-only row claimed a remote export.
    LocalOnlyClaimsRemoteExport { row_id: String },
    /// A local-only row claimed a remote delete.
    LocalOnlyClaimsRemoteDelete { row_id: String },
    /// Duplicate row id in the packet.
    DuplicateRowId { row_id: String },
    /// Duplicate canonical event id in the packet.
    DuplicateCanonicalEventId { canonical_event_id: String },
    /// A required workflow class is missing from the packet.
    WorkflowCoverageMissing {
        workflow_class: M5EvidenceWorkflowClass,
    },
}

impl M5EvidenceChronologyPacket {
    /// Validates the packet against the evidence-time honesty contract.
    pub fn validate(&self) -> Vec<M5EvidenceChronologyViolation> {
        let mut violations = Vec::new();

        if self.schema_version != M5_EVIDENCE_CHRONOLOGY_SCHEMA_VERSION {
            violations.push(M5EvidenceChronologyViolation::SchemaVersionMismatch {
                found: self.schema_version,
            });
        }
        if self.record_kind != M5_EVIDENCE_CHRONOLOGY_PACKET_RECORD_KIND {
            violations.push(M5EvidenceChronologyViolation::PacketRecordKindMismatch {
                found: self.record_kind.clone(),
            });
        }

        let mut seen_row_ids = BTreeSet::new();
        let mut seen_event_ids = BTreeSet::new();
        for row in &self.rows {
            Self::validate_row(row, &mut violations);
            if !seen_row_ids.insert(row.row_id.clone()) {
                violations.push(M5EvidenceChronologyViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            if !seen_event_ids.insert(row.canonical_event_id.clone()) {
                violations.push(M5EvidenceChronologyViolation::DuplicateCanonicalEventId {
                    canonical_event_id: row.canonical_event_id.clone(),
                });
            }
        }

        for workflow_class in M5EvidenceWorkflowClass::ALL {
            if !self
                .rows
                .iter()
                .any(|row| row.workflow_class == workflow_class)
            {
                violations.push(M5EvidenceChronologyViolation::WorkflowCoverageMissing {
                    workflow_class,
                });
            }
        }

        violations
    }

    fn validate_row(
        row: &M5EvidenceChronologyRow,
        violations: &mut Vec<M5EvidenceChronologyViolation>,
    ) {
        if row.record_kind != M5_EVIDENCE_CHRONOLOGY_ROW_RECORD_KIND {
            violations.push(M5EvidenceChronologyViolation::RowRecordKindMismatch {
                row_id: row.row_id.clone(),
                found: row.record_kind.clone(),
            });
        }
        if row.schema_version != M5_EVIDENCE_CHRONOLOGY_SCHEMA_VERSION {
            violations.push(M5EvidenceChronologyViolation::RowSchemaVersionMismatch {
                row_id: row.row_id.clone(),
                found: row.schema_version,
            });
        }

        for (field, value) in [
            ("row_id", row.row_id.as_str()),
            ("canonical_event_id", row.canonical_event_id.as_str()),
            ("actor_label", row.actor_label.as_str()),
            ("object_label", row.object_label.as_str()),
            ("outcome_label", row.outcome_label.as_str()),
            (
                "absolute_timestamp",
                row.time_posture.absolute_timestamp.as_str(),
            ),
            ("timezone_iana", row.time_posture.timezone_iana.as_str()),
            ("utc_offset", row.time_posture.utc_offset.as_str()),
            (
                "relative_label",
                row.time_posture.relative_age.relative_label.as_str(),
            ),
            ("proof_ref", row.proof_ref.as_str()),
            (
                "current_custodian_ref",
                row.actor_lineage.current_custodian_ref.as_str(),
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(M5EvidenceChronologyViolation::RequiredFieldEmpty {
                    row_id: row.row_id.clone(),
                    field: field.to_owned(),
                });
            }
        }

        if row.grammar_sentence != row.canonical_sentence() {
            violations.push(M5EvidenceChronologyViolation::GrammarSentenceDrift {
                row_id: row.row_id.clone(),
            });
        }

        if row.provenance_badges.is_empty() {
            violations.push(M5EvidenceChronologyViolation::MissingProvenanceBadge {
                row_id: row.row_id.clone(),
            });
        }
        for badge in &row.provenance_badges {
            if badge.source_class.requires_export_marker()
                && !badge
                    .export_marker_label
                    .contains(badge.badge_label.as_str())
            {
                violations.push(M5EvidenceChronologyViolation::ProvenanceExportMarkerDrift {
                    row_id: row.row_id.clone(),
                });
            }
        }

        if matches!(
            row.time_posture.relative_age.freshness_class,
            ChronologyFreshnessClass::Stale | ChronologyFreshnessClass::Expired
        ) && row.time_posture.relative_age.stale_reason_label.is_none()
        {
            violations.push(M5EvidenceChronologyViolation::StaleReasonMissing {
                row_id: row.row_id.clone(),
            });
        }

        Self::validate_lineage(row, violations);

        if row.residency.is_local_only() {
            if row.claims_remote_hold {
                violations.push(M5EvidenceChronologyViolation::LocalOnlyClaimsRemoteHold {
                    row_id: row.row_id.clone(),
                });
            }
            if row.claims_remote_export {
                violations.push(M5EvidenceChronologyViolation::LocalOnlyClaimsRemoteExport {
                    row_id: row.row_id.clone(),
                });
            }
            if row.claims_remote_delete {
                violations.push(M5EvidenceChronologyViolation::LocalOnlyClaimsRemoteDelete {
                    row_id: row.row_id.clone(),
                });
            }
        }
    }

    fn validate_lineage(
        row: &M5EvidenceChronologyRow,
        violations: &mut Vec<M5EvidenceChronologyViolation>,
    ) {
        let steps = &row.actor_lineage.steps;
        if steps.is_empty() {
            violations.push(M5EvidenceChronologyViolation::EmptyActorLineage {
                row_id: row.row_id.clone(),
            });
            return;
        }

        for (position, step) in steps.iter().enumerate() {
            if step.step_index as usize != position {
                violations.push(M5EvidenceChronologyViolation::LineageStepOrderBroken {
                    row_id: row.row_id.clone(),
                    step_index: step.step_index,
                });
            }
            for (field, value) in [
                ("actor_label", step.actor_label.as_str()),
                (
                    "step_absolute_timestamp",
                    step.step_time.absolute_timestamp.as_str(),
                ),
                ("step_timezone_iana", step.step_time.timezone_iana.as_str()),
                ("step_utc_offset", step.step_time.utc_offset.as_str()),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5EvidenceChronologyViolation::RequiredFieldEmpty {
                        row_id: row.row_id.clone(),
                        field: field.to_owned(),
                    });
                }
            }
        }

        let originator = &steps[0];
        if originator.role != ActorLineageRole::Originator {
            violations.push(M5EvidenceChronologyViolation::LineageMissingOriginator {
                row_id: row.row_id.clone(),
            });
        }
        if originator.actor_label != row.actor_label {
            violations.push(M5EvidenceChronologyViolation::OriginatorActorMismatch {
                row_id: row.row_id.clone(),
            });
        }
    }

    /// Projects the product-surface rows.
    pub fn product_projection(&self) -> Vec<ProductEvidenceRow> {
        self.rows
            .iter()
            .map(|row| ProductEvidenceRow {
                workflow_class: row.workflow_class,
                canonical_event_id: row.canonical_event_id.clone(),
                grammar_sentence: row.grammar_sentence.clone(),
                source_class: row.source_class,
                imported_class: row.imported_class,
                absolute_timestamp: row.time_posture.absolute_timestamp.clone(),
                relative_age_label: row.time_posture.relative_age.relative_label.clone(),
                residency: row.residency,
                lineage_step_count: row.actor_lineage.steps.len(),
            })
            .collect()
    }

    /// Projects the admin-surface rows with full lineage reconstruction.
    pub fn admin_projection(&self) -> Vec<AdminEvidenceRow> {
        self.rows
            .iter()
            .map(|row| AdminEvidenceRow {
                workflow_class: row.workflow_class,
                canonical_event_id: row.canonical_event_id.clone(),
                grammar_sentence: row.grammar_sentence.clone(),
                source_class: row.source_class,
                imported_class: row.imported_class,
                absolute_timestamp: row.time_posture.absolute_timestamp.clone(),
                timezone_iana: row.time_posture.timezone_iana.clone(),
                utc_offset: row.time_posture.utc_offset.clone(),
                actor_lineage: row.actor_lineage.clone(),
                residency: row.residency,
                local_only_boundary_note: row.local_only_boundary_note.clone(),
            })
            .collect()
    }

    /// Projects the support/export rows.
    pub fn support_export_projection(&self) -> Vec<SupportExportEvidenceRow> {
        self.rows
            .iter()
            .map(|row| SupportExportEvidenceRow {
                workflow_class: row.workflow_class,
                canonical_event_id: row.canonical_event_id.clone(),
                grammar_sentence: row.grammar_sentence.clone(),
                provenance_marker_labels: row
                    .provenance_badges
                    .iter()
                    .map(|badge| badge.export_marker_label.clone())
                    .collect(),
                absolute_timestamp: row.time_posture.absolute_timestamp.clone(),
                timezone_iana: row.time_posture.timezone_iana.clone(),
                source_class: row.source_class,
                imported_class: row.imported_class,
                lineage_actor_refs: row.actor_lineage.ordered_actor_refs(),
                lineage_step_count: row.actor_lineage.steps.len(),
                residency: row.residency,
                local_only_boundary_note: row.local_only_boundary_note.clone(),
            })
            .collect()
    }
}

/// Returns the canonical seeded M5 evidence-chronology packet.
pub fn seeded_m5_evidence_chronology_packet() -> M5EvidenceChronologyPacket {
    let rows = vec![
        incident_row(),
        offboarding_row(),
        ai_evidence_row(),
        managed_sync_row(),
        provider_linked_row(),
    ];

    M5EvidenceChronologyPacket {
        schema_version: M5_EVIDENCE_CHRONOLOGY_SCHEMA_VERSION,
        record_kind: M5_EVIDENCE_CHRONOLOGY_PACKET_RECORD_KIND.to_owned(),
        packet_id: "m5-evidence-chronology:lineage:0001".to_owned(),
        shared_contract_ref: M5_EVIDENCE_CHRONOLOGY_SHARED_CONTRACT_REF.to_owned(),
        schema_ref: M5_EVIDENCE_CHRONOLOGY_SCHEMA_REF.to_owned(),
        as_of: "2026-06-13T16:00:00Z".to_owned(),
        overview_doc_ref: M5_EVIDENCE_CHRONOLOGY_DOC_REF.to_owned(),
        artifact_summary_ref: M5_EVIDENCE_CHRONOLOGY_ARTIFACT_REF.to_owned(),
        rows,
        summary: "Canonical timezone-aware evidence chronology, actor lineage, and \
                  source/live/imported labels for the M5 incident, offboarding, AI, sync, and \
                  provider evidence families."
            .to_owned(),
    }
}

fn badge(source: ChronologySourceClass, label: &str) -> ProvenanceBadge {
    ProvenanceBadge {
        source_class: source,
        badge_label: label.to_owned(),
        export_marker_label: format!("{label} source"),
    }
}

fn finalize(mut row: M5EvidenceChronologyRow) -> M5EvidenceChronologyRow {
    row.grammar_sentence = row.canonical_sentence();
    row
}

fn incident_row() -> M5EvidenceChronologyRow {
    finalize(M5EvidenceChronologyRow {
        record_kind: M5_EVIDENCE_CHRONOLOGY_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_EVIDENCE_CHRONOLOGY_SCHEMA_VERSION,
        row_id: "m5-evidence:incident".to_owned(),
        canonical_event_id: "event:incident:timeline:2026-06-13T15:40:00Z".to_owned(),
        workflow_class: M5EvidenceWorkflowClass::IncidentSupport,
        actor_kind: ActorKind::UserActor,
        actor_label: "On-call engineer".to_owned(),
        action: ActionVerb::Published,
        object_label: "incident evidence timeline".to_owned(),
        outcome_label: "timeline shared to the support case".to_owned(),
        grammar_sentence: String::new(),
        source_class: ChronologySourceClass::FirstPartyDirectObservation,
        imported_class: ChronologyImportedClass::Live,
        provenance_badges: vec![badge(
            ChronologySourceClass::FirstPartyDirectObservation,
            "Direct",
        )],
        time_posture: TimePosture {
            absolute_timestamp: "2026-06-13T15:40:00Z".to_owned(),
            timezone_iana: "America/Los_Angeles".to_owned(),
            utc_offset: "-07:00".to_owned(),
            local_time_label: "2026-06-13 08:40 America/Los_Angeles".to_owned(),
            imported_class: ChronologyImportedClass::Live,
            relative_age: rel("20 min ago", ChronologyFreshnessClass::Fresh, None),
        },
        actor_lineage: ActorLineage {
            lineage_id: "lineage:incident".to_owned(),
            originator_actor_ref: Some("actor:on-call-engineer".to_owned()),
            current_custodian_ref: "owner:support-governance".to_owned(),
            steps: vec![
                step(
                    0,
                    ActorLineageRole::Originator,
                    ActorKind::UserActor,
                    Some("actor:on-call-engineer"),
                    "On-call engineer",
                    ActionVerb::Published,
                    ChronologySourceClass::FirstPartyDirectObservation,
                    "2026-06-13T15:40:00Z",
                    "2026-06-13 08:40 America/Los_Angeles",
                    ChronologyImportedClass::Live,
                    None,
                ),
                step(
                    1,
                    ActorLineageRole::Custodian,
                    ActorKind::AdminPolicyActor,
                    Some("owner:support-governance"),
                    "Support governance",
                    ActionVerb::Acknowledged,
                    ChronologySourceClass::FirstPartySynthesizedSummary,
                    "2026-06-13T15:45:00Z",
                    "2026-06-13 08:45 America/Los_Angeles",
                    ChronologyImportedClass::Live,
                    Some("Managed support copy retained for the case window.".to_owned()),
                ),
            ],
        },
        residency: EvidenceResidencyClass::LocalAndManaged,
        claims_remote_hold: true,
        claims_remote_export: true,
        claims_remote_delete: true,
        local_only_boundary_note: None,
        proof_ref: "proof:m5-evidence-chronology:incident".to_owned(),
    })
}

fn offboarding_row() -> M5EvidenceChronologyRow {
    finalize(M5EvidenceChronologyRow {
        record_kind: M5_EVIDENCE_CHRONOLOGY_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_EVIDENCE_CHRONOLOGY_SCHEMA_VERSION,
        row_id: "m5-evidence:offboarding".to_owned(),
        canonical_event_id: "event:offboarding:exit-packet:2026-06-13T14:30:00Z".to_owned(),
        workflow_class: M5EvidenceWorkflowClass::Offboarding,
        actor_kind: ActorKind::AdminPolicyActor,
        actor_label: "Org admin".to_owned(),
        action: ActionVerb::Exported,
        object_label: "offboarding exit packet".to_owned(),
        outcome_label: "managed export retained at the retention floor".to_owned(),
        grammar_sentence: String::new(),
        source_class: ChronologySourceClass::PolicyAuthored,
        imported_class: ChronologyImportedClass::Live,
        provenance_badges: vec![badge(ChronologySourceClass::PolicyAuthored, "Policy")],
        time_posture: TimePosture {
            absolute_timestamp: "2026-06-13T14:30:00Z".to_owned(),
            timezone_iana: "Europe/Berlin".to_owned(),
            utc_offset: "+02:00".to_owned(),
            local_time_label: "2026-06-13 16:30 Europe/Berlin".to_owned(),
            imported_class: ChronologyImportedClass::Live,
            relative_age: rel("1 h 30 min ago", ChronologyFreshnessClass::Current, None),
        },
        actor_lineage: ActorLineage {
            lineage_id: "lineage:offboarding".to_owned(),
            originator_actor_ref: Some("owner:org-admin".to_owned()),
            current_custodian_ref: "owner:org-admin".to_owned(),
            steps: vec![
                step(
                    0,
                    ActorLineageRole::Originator,
                    ActorKind::AdminPolicyActor,
                    Some("owner:org-admin"),
                    "Org admin",
                    ActionVerb::Exported,
                    ChronologySourceClass::PolicyAuthored,
                    "2026-06-13T14:30:00Z",
                    "2026-06-13 16:30 Europe/Berlin",
                    ChronologyImportedClass::Live,
                    None,
                ),
                step(
                    1,
                    ActorLineageRole::Approver,
                    ActorKind::AdminPolicyActor,
                    Some("owner:records-governance"),
                    "Records governance",
                    ActionVerb::Accepted,
                    ChronologySourceClass::PolicyAuthored,
                    "2026-06-13T14:35:00Z",
                    "2026-06-13 16:35 Europe/Berlin",
                    ChronologyImportedClass::Live,
                    Some("Retention-floor approval recorded.".to_owned()),
                ),
            ],
        },
        residency: EvidenceResidencyClass::ManagedCopy,
        claims_remote_hold: true,
        claims_remote_export: true,
        claims_remote_delete: true,
        local_only_boundary_note: Some(
            "Local downloaded exports remain user-controlled on device.".to_owned(),
        ),
        proof_ref: "proof:m5-evidence-chronology:offboarding".to_owned(),
    })
}

fn ai_evidence_row() -> M5EvidenceChronologyRow {
    finalize(M5EvidenceChronologyRow {
        record_kind: M5_EVIDENCE_CHRONOLOGY_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_EVIDENCE_CHRONOLOGY_SCHEMA_VERSION,
        row_id: "m5-evidence:ai-evidence".to_owned(),
        canonical_event_id: "event:ai:evidence-packet:2026-06-13T15:10:00Z".to_owned(),
        workflow_class: M5EvidenceWorkflowClass::AiEvidence,
        actor_kind: ActorKind::AiAgentActor,
        actor_label: "AI run".to_owned(),
        action: ActionVerb::Proposed,
        object_label: "patch review evidence packet".to_owned(),
        outcome_label: "review required before apply".to_owned(),
        grammar_sentence: String::new(),
        source_class: ChronologySourceClass::AiAssisted,
        imported_class: ChronologyImportedClass::Live,
        provenance_badges: vec![badge(ChronologySourceClass::AiAssisted, "AI assisted")],
        time_posture: TimePosture {
            absolute_timestamp: "2026-06-13T15:10:00Z".to_owned(),
            timezone_iana: "America/Los_Angeles".to_owned(),
            utc_offset: "-07:00".to_owned(),
            local_time_label: "2026-06-13 08:10 America/Los_Angeles".to_owned(),
            imported_class: ChronologyImportedClass::Live,
            relative_age: rel("50 min ago", ChronologyFreshnessClass::Current, None),
        },
        actor_lineage: ActorLineage {
            lineage_id: "lineage:ai-evidence".to_owned(),
            originator_actor_ref: Some("ai-run:2048".to_owned()),
            current_custodian_ref: "owner:records-governance".to_owned(),
            steps: vec![
                step(
                    0,
                    ActorLineageRole::Originator,
                    ActorKind::AiAgentActor,
                    Some("ai-run:2048"),
                    "AI run",
                    ActionVerb::Proposed,
                    ChronologySourceClass::AiAssisted,
                    "2026-06-13T15:10:00Z",
                    "2026-06-13 08:10 America/Los_Angeles",
                    ChronologyImportedClass::Live,
                    None,
                ),
                step(
                    1,
                    ActorLineageRole::Reviewer,
                    ActorKind::UserActor,
                    Some("actor:local-user"),
                    "You",
                    ActionVerb::Presented,
                    ChronologySourceClass::FirstPartyDirectObservation,
                    "2026-06-13T15:12:00Z",
                    "2026-06-13 08:12 America/Los_Angeles",
                    ChronologyImportedClass::Live,
                    Some("Local reviewer opened the evidence packet.".to_owned()),
                ),
                step(
                    2,
                    ActorLineageRole::Custodian,
                    ActorKind::AdminPolicyActor,
                    Some("owner:records-governance"),
                    "Records governance",
                    ActionVerb::Held,
                    ChronologySourceClass::PolicyAuthored,
                    "2026-06-13T15:14:00Z",
                    "2026-06-13 08:14 America/Los_Angeles",
                    ChronologyImportedClass::Live,
                    Some("Managed evidence copy retained under policy.".to_owned()),
                ),
            ],
        },
        residency: EvidenceResidencyClass::LocalAndManaged,
        claims_remote_hold: true,
        claims_remote_export: true,
        claims_remote_delete: true,
        local_only_boundary_note: Some("Local prompt and result caches stay on device.".to_owned()),
        proof_ref: "proof:m5-evidence-chronology:ai-evidence".to_owned(),
    })
}

fn managed_sync_row() -> M5EvidenceChronologyRow {
    finalize(M5EvidenceChronologyRow {
        record_kind: M5_EVIDENCE_CHRONOLOGY_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_EVIDENCE_CHRONOLOGY_SCHEMA_VERSION,
        row_id: "m5-evidence:managed-sync".to_owned(),
        canonical_event_id: "event:sync:mirror-ledger:2026-06-13T12:05:00Z".to_owned(),
        workflow_class: M5EvidenceWorkflowClass::ManagedSync,
        actor_kind: ActorKind::SystemActor,
        actor_label: "Sync engine".to_owned(),
        action: ActionVerb::Imported,
        object_label: "managed mirror ledger snapshot".to_owned(),
        outcome_label: "mirror state cached from the managed service".to_owned(),
        grammar_sentence: String::new(),
        source_class: ChronologySourceClass::ProviderStaleCached,
        imported_class: ChronologyImportedClass::StaleCached,
        provenance_badges: vec![badge(
            ChronologySourceClass::ProviderStaleCached,
            "Stale cached",
        )],
        time_posture: TimePosture {
            absolute_timestamp: "2026-06-13T12:05:00Z".to_owned(),
            timezone_iana: "America/Los_Angeles".to_owned(),
            utc_offset: "-07:00".to_owned(),
            local_time_label: "2026-06-13 05:05 America/Los_Angeles".to_owned(),
            imported_class: ChronologyImportedClass::StaleCached,
            relative_age: rel(
                "3 h 55 min ago",
                ChronologyFreshnessClass::Stale,
                Some("Mirror snapshot is older than the managed service truth."),
            ),
        },
        actor_lineage: ActorLineage {
            lineage_id: "lineage:managed-sync".to_owned(),
            originator_actor_ref: Some("service:managed-sync".to_owned()),
            current_custodian_ref: "owner:org-admin".to_owned(),
            steps: vec![
                step(
                    0,
                    ActorLineageRole::Originator,
                    ActorKind::SystemActor,
                    Some("service:managed-sync"),
                    "Sync engine",
                    ActionVerb::Imported,
                    ChronologySourceClass::ProviderStaleCached,
                    "2026-06-13T12:05:00Z",
                    "2026-06-13 05:05 America/Los_Angeles",
                    ChronologyImportedClass::StaleCached,
                    Some("Last successful mirror import before the link degraded.".to_owned()),
                ),
                step(
                    1,
                    ActorLineageRole::Custodian,
                    ActorKind::AdminPolicyActor,
                    Some("owner:org-admin"),
                    "Org admin",
                    ActionVerb::Held,
                    ChronologySourceClass::PolicyAuthored,
                    "2026-06-13T13:00:00Z",
                    "2026-06-13 06:00 America/Los_Angeles",
                    ChronologyImportedClass::Live,
                    Some("Managed mirror held under an active legal hold.".to_owned()),
                ),
            ],
        },
        residency: EvidenceResidencyClass::LocalAndManaged,
        claims_remote_hold: true,
        claims_remote_export: false,
        claims_remote_delete: true,
        local_only_boundary_note: Some(
            "Local per-device snapshots are outside the managed mirror.".to_owned(),
        ),
        proof_ref: "proof:m5-evidence-chronology:managed-sync".to_owned(),
    })
}

fn provider_linked_row() -> M5EvidenceChronologyRow {
    finalize(M5EvidenceChronologyRow {
        record_kind: M5_EVIDENCE_CHRONOLOGY_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_EVIDENCE_CHRONOLOGY_SCHEMA_VERSION,
        row_id: "m5-evidence:provider-linked".to_owned(),
        canonical_event_id: "event:provider:work-item:2026-06-13T15:25:00Z".to_owned(),
        workflow_class: M5EvidenceWorkflowClass::ProviderLinked,
        actor_kind: ActorKind::RemoteServiceActor,
        actor_label: "GitHub Enterprise".to_owned(),
        action: ActionVerb::Imported,
        object_label: "linked work-item review status".to_owned(),
        outcome_label: "provider state mirrored as local linkage metadata".to_owned(),
        grammar_sentence: String::new(),
        source_class: ChronologySourceClass::ProviderImported,
        imported_class: ChronologyImportedClass::Imported,
        provenance_badges: vec![badge(ChronologySourceClass::ProviderImported, "Imported")],
        time_posture: TimePosture {
            absolute_timestamp: "2026-06-13T15:25:00Z".to_owned(),
            timezone_iana: "America/Los_Angeles".to_owned(),
            utc_offset: "-07:00".to_owned(),
            local_time_label: "2026-06-13 08:25 America/Los_Angeles".to_owned(),
            imported_class: ChronologyImportedClass::Imported,
            relative_age: rel("35 min ago", ChronologyFreshnessClass::Fresh, None),
        },
        actor_lineage: ActorLineage {
            lineage_id: "lineage:provider-linked".to_owned(),
            originator_actor_ref: Some("provider:github-enterprise".to_owned()),
            current_custodian_ref: "owner:local-user".to_owned(),
            steps: vec![
                step(
                    0,
                    ActorLineageRole::Originator,
                    ActorKind::RemoteServiceActor,
                    Some("provider:github-enterprise"),
                    "GitHub Enterprise",
                    ActionVerb::Imported,
                    ChronologySourceClass::ProviderImported,
                    "2026-06-13T15:25:00Z",
                    "2026-06-13 08:25 America/Los_Angeles",
                    ChronologyImportedClass::Imported,
                    None,
                ),
                step(
                    1,
                    ActorLineageRole::Custodian,
                    ActorKind::UserActor,
                    Some("actor:local-user"),
                    "You",
                    ActionVerb::Acknowledged,
                    ChronologySourceClass::FirstPartyDirectObservation,
                    "2026-06-13T15:26:00Z",
                    "2026-06-13 08:26 America/Los_Angeles",
                    ChronologyImportedClass::Live,
                    Some("Only local linkage metadata is held on device.".to_owned()),
                ),
            ],
        },
        residency: EvidenceResidencyClass::LocalOnly,
        claims_remote_hold: false,
        claims_remote_export: false,
        claims_remote_delete: false,
        local_only_boundary_note: Some(
            "The provider-side record is outside this platform's hold, export, and delete scope."
                .to_owned(),
        ),
        proof_ref: "proof:m5-evidence-chronology:provider-linked".to_owned(),
    })
}

fn rel(
    relative_label: &str,
    freshness_class: ChronologyFreshnessClass,
    stale_reason: Option<&str>,
) -> crate::stabilize_chronology_grammar_and_history_row_truth::RelativeAgeHint {
    crate::stabilize_chronology_grammar_and_history_row_truth::RelativeAgeHint {
        rendered_at: "2026-06-13T16:00:00Z".to_owned(),
        relative_label: relative_label.to_owned(),
        freshness_class,
        visible_reason_label:
            "evidence row remains visible with its absolute time and source class".to_owned(),
        stale_reason_label: stale_reason.map(str::to_owned),
    }
}

#[allow(clippy::too_many_arguments)]
fn step(
    step_index: u32,
    role: ActorLineageRole,
    actor_kind: ActorKind,
    actor_ref: Option<&str>,
    actor_label: &str,
    action: ActionVerb,
    source_class: ChronologySourceClass,
    absolute: &str,
    local: &str,
    imported_class: ChronologyImportedClass,
    note: Option<String>,
) -> ActorLineageStep {
    let utc_offset = if local.contains("Europe/Berlin") {
        "+02:00"
    } else {
        "-07:00"
    };
    let timezone_iana = if local.contains("Europe/Berlin") {
        "Europe/Berlin"
    } else {
        "America/Los_Angeles"
    };
    ActorLineageStep {
        step_index,
        role,
        actor_kind,
        actor_ref: actor_ref.map(str::to_owned),
        actor_label: actor_label.to_owned(),
        action,
        source_class,
        step_time: LineageStepTime {
            absolute_timestamp: absolute.to_owned(),
            timezone_iana: timezone_iana.to_owned(),
            utc_offset: utc_offset.to_owned(),
            local_time_label: local.to_owned(),
            imported_class,
        },
        note,
    }
}
