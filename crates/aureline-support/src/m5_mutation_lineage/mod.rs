//! Support-export consumer for the canonical M5 mutation-lineage packet.
//!
//! This module folds the checked-in M5 mutation-lineage packet into a
//! metadata-safe support-export envelope so support, incident, and handoff
//! flows can quote the same cross-surface reversal classes, checkpoint
//! lineage, file counts, artifact classes, and automation or policy influence
//! that the shell history inspector renders.

use std::fmt;

use aureline_reactive_state::{
    seeded_m5_mutation_lineage_packet, validate_m5_mutation_lineage_packet, M5MutationArtifactClass,
    M5MutationAutomationInfluence, M5MutationLineagePacket,
    M5MutationLineageValidationReport, M5MutationPolicyInfluence,
    M5MutationReversalClass, M5MutationSupportExportManifestRow,
    M5_MUTATION_LINEAGE_DOC_REF, M5_MUTATION_LINEAGE_REPORT_REF, M5_MUTATION_LINEAGE_SCHEMA_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one support-export row.
pub const M5_MUTATION_LINEAGE_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "m5_mutation_lineage_support_export_row";

/// Stable record-kind tag for the support-export envelope.
pub const M5_MUTATION_LINEAGE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "m5_mutation_lineage_support_export_envelope";

/// One support-export row copied from the canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MutationLineageSupportExportRow {
    /// Stable row record kind.
    pub record_kind: String,
    /// Stable row id.
    pub row_id: String,
    /// Shared lineage root id.
    pub lineage_root_id: String,
    /// Related group ids.
    pub group_ids: Vec<String>,
    /// Related mutation ids.
    pub mutation_ids: Vec<String>,
    /// Highest-risk reversal class preserved on the export row.
    pub highest_risk_reversal_class: M5MutationReversalClass,
    /// Every reversal class preserved on the export row.
    pub reversal_classes: Vec<M5MutationReversalClass>,
    /// Aggregated file count.
    pub total_file_count: u32,
    /// Artifact classes preserved on the export row.
    pub artifact_classes: Vec<M5MutationArtifactClass>,
    /// Automation influences preserved on the export row.
    pub automation_influences: Vec<M5MutationAutomationInfluence>,
    /// Policy influences preserved on the export row.
    pub policy_influences: Vec<M5MutationPolicyInfluence>,
    /// Raw payloads remain excluded.
    pub raw_payload_excluded: bool,
    /// Raw private material remains excluded.
    pub raw_private_material_excluded: bool,
    /// Ambient authority remains excluded.
    pub ambient_authority_excluded: bool,
    /// The export still points to one lineage thread.
    pub single_lineage_thread_preserved: bool,
    /// Short reviewer note.
    pub notes: String,
}

impl M5MutationLineageSupportExportRow {
    fn from_manifest_row(row: &M5MutationSupportExportManifestRow) -> Self {
        Self {
            record_kind: M5_MUTATION_LINEAGE_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            row_id: row.row_id.clone(),
            lineage_root_id: row.lineage_root_id.clone(),
            group_ids: row.group_ids.clone(),
            mutation_ids: row.mutation_ids.clone(),
            highest_risk_reversal_class: row.highest_risk_reversal_class,
            reversal_classes: row.reversal_classes.clone(),
            total_file_count: row.total_file_count,
            artifact_classes: row.artifact_classes.clone(),
            automation_influences: row.automation_influences.clone(),
            policy_influences: row.policy_influences.clone(),
            raw_payload_excluded: row.raw_payload_excluded,
            raw_private_material_excluded: row.raw_private_material_excluded,
            ambient_authority_excluded: row.ambient_authority_excluded,
            single_lineage_thread_preserved: row.single_lineage_thread_preserved,
            notes: row.notes.clone(),
        }
    }

    /// Returns true when the export row remains metadata-safe.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.single_lineage_thread_preserved
            && !self.group_ids.is_empty()
            && !self.mutation_ids.is_empty()
            && self.total_file_count > 0
            && !self.artifact_classes.is_empty()
            && !self.reversal_classes.is_empty()
    }
}

/// Metadata-safe support-export envelope for M5 mutation lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MutationLineageSupportExportEnvelope {
    /// Stable envelope record kind.
    pub record_kind: String,
    /// Stable envelope id.
    pub envelope_id: String,
    /// Capture time supplied by the caller.
    pub captured_at: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Reviewer report ref.
    pub report_ref: String,
    /// Raw payloads remain excluded.
    pub raw_payload_excluded: bool,
    /// Raw private material remains excluded.
    pub raw_private_material_excluded: bool,
    /// Ambient authority remains excluded.
    pub ambient_authority_excluded: bool,
    /// Export rows.
    pub rows: Vec<M5MutationLineageSupportExportRow>,
}

impl M5MutationLineageSupportExportEnvelope {
    /// Builds an envelope from a validated packet.
    pub fn from_packet(
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
        packet: &M5MutationLineagePacket,
    ) -> Self {
        let mut rows: Vec<_> = packet
            .support_export_rows
            .iter()
            .map(M5MutationLineageSupportExportRow::from_manifest_row)
            .collect();
        rows.sort_by(|a, b| a.row_id.cmp(&b.row_id));
        Self {
            record_kind: M5_MUTATION_LINEAGE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            doc_ref: M5_MUTATION_LINEAGE_DOC_REF.to_owned(),
            schema_ref: M5_MUTATION_LINEAGE_SCHEMA_REF.to_owned(),
            report_ref: M5_MUTATION_LINEAGE_REPORT_REF.to_owned(),
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            rows,
        }
    }

    /// Returns true when the envelope remains metadata-safe and in sync with
    /// the canonical packet refs.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.doc_ref == M5_MUTATION_LINEAGE_DOC_REF
            && self.schema_ref == M5_MUTATION_LINEAGE_SCHEMA_REF
            && self.report_ref == M5_MUTATION_LINEAGE_REPORT_REF
            && !self.rows.is_empty()
            && self.rows.iter().all(M5MutationLineageSupportExportRow::is_export_safe)
    }
}

/// Error returned when the envelope cannot be compiled.
#[derive(Debug)]
pub enum M5MutationLineageSupportExportError {
    /// The canonical packet failed validation.
    PacketValidation(M5MutationLineageValidationReport),
}

impl fmt::Display for M5MutationLineageSupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => write!(f, "m5 mutation lineage invalid: {report}"),
        }
    }
}

impl std::error::Error for M5MutationLineageSupportExportError {}

impl From<M5MutationLineageValidationReport> for M5MutationLineageSupportExportError {
    fn from(report: M5MutationLineageValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

/// Compiles the metadata-safe support-export envelope from the canonical M5
/// packet.
pub fn compile_support_export_envelope(
    envelope_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> Result<M5MutationLineageSupportExportEnvelope, M5MutationLineageSupportExportError> {
    let packet = seeded_m5_mutation_lineage_packet();
    validate_m5_mutation_lineage_packet(&packet)?;
    Ok(M5MutationLineageSupportExportEnvelope::from_packet(
        envelope_id,
        captured_at,
        &packet,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_envelope_round_trip() {
        let envelope = compile_support_export_envelope(
            "envelope:m5_mutation_lineage:test",
            "2026-06-13T08:00:00Z",
        )
        .expect("envelope compiles");
        assert!(envelope.is_export_safe());
        assert_eq!(envelope.rows.len(), 5);

        let json = serde_json::to_string(&envelope).expect("envelope serializes");
        let parsed: M5MutationLineageSupportExportEnvelope =
            serde_json::from_str(&json).expect("envelope round-trips");
        assert_eq!(parsed, envelope);
    }

    #[test]
    fn provider_sync_row_preserves_compensate_and_manual_classes() {
        let envelope = compile_support_export_envelope(
            "envelope:m5_mutation_lineage:provider_sync",
            "2026-06-13T08:05:00Z",
        )
        .expect("envelope compiles");
        let row = envelope
            .rows
            .iter()
            .find(|row| row.lineage_root_id == "lineage:m5:provider_sync:0001")
            .expect("provider sync row exists");
        assert_eq!(
            row.highest_risk_reversal_class,
            M5MutationReversalClass::Manual
        );
        assert!(
            row.reversal_classes
                .contains(&M5MutationReversalClass::Compensate)
        );
        assert!(
            row.reversal_classes
                .contains(&M5MutationReversalClass::Manual)
        );
        assert!(row.single_lineage_thread_preserved);
    }
}
