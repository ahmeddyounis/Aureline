//! Support/export projection for canonical export/delete lifecycle records.
//!
//! This module consumes the authoritative export-job, request-case, and
//! delete-case packet from `aureline-records` and exposes a metadata-only
//! support/export object. It gives support surfaces one place to inspect
//! manifests, receipts, typed blockers, and local-only boundary notes across
//! the AI evidence, provider, sync, incident, and offboarding families.

use serde::{Deserialize, Serialize};

use aureline_records::export_delete_lifecycle::{
    seeded_records_export_delete_lifecycle_packet, RecordsExportDeleteLifecyclePacket,
    RecordsExportDeleteLifecycleViolation, SupportExportLifecycleRow,
    RECORDS_EXPORT_DELETE_LIFECYCLE_ARTIFACT_REF, RECORDS_EXPORT_DELETE_LIFECYCLE_DOC_REF,
};

#[cfg(test)]
mod tests;

/// Schema version for the support-side lifecycle export.
pub const RECORDS_EXPORT_DELETE_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for the support-side lifecycle export.
pub const RECORDS_EXPORT_DELETE_GOVERNANCE_RECORD_KIND: &str =
    "records_export_delete_governance_support_export";

/// Support-side packet joining lifecycle truth into one metadata-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordsExportDeleteGovernanceSupportExport {
    /// Schema version.
    pub schema_version: u32,
    /// Stable record kind.
    pub record_kind: String,
    /// Stable export id.
    pub export_id: String,
    /// Generated-at timestamp.
    pub generated_at: String,
    /// Lifecycle contract doc ref.
    pub lifecycle_doc_ref: String,
    /// Lifecycle artifact summary ref.
    pub lifecycle_artifact_ref: String,
    /// Embedded authoritative lifecycle packet.
    pub lifecycle_packet: RecordsExportDeleteLifecyclePacket,
    /// Support/export projection rows.
    pub projection_rows: Vec<SupportExportLifecycleRow>,
    /// Validation issues inherited from the authoritative packet.
    pub violations: Vec<RecordsExportDeleteLifecycleViolation>,
    /// Metadata-only redaction invariant.
    pub raw_private_material_excluded: bool,
    /// Review-safe summary.
    pub summary: String,
}

impl RecordsExportDeleteGovernanceSupportExport {
    /// Returns the current support-side export from the seeded authoritative packet.
    pub fn current() -> Self {
        let lifecycle_packet = seeded_records_export_delete_lifecycle_packet();
        let projection_rows = lifecycle_packet.support_export_projection();
        let violations = lifecycle_packet.validate();
        let summary = format!(
            "Support export covering {} lifecycle families, {} export jobs, and {} delete cases.",
            projection_rows.len(),
            lifecycle_packet.export_jobs.len(),
            lifecycle_packet.delete_cases.len()
        );

        Self {
            schema_version: RECORDS_EXPORT_DELETE_GOVERNANCE_SCHEMA_VERSION,
            record_kind: RECORDS_EXPORT_DELETE_GOVERNANCE_RECORD_KIND.to_owned(),
            export_id: "support-export:records-export-delete-governance:v1".to_owned(),
            generated_at: lifecycle_packet.as_of.clone(),
            lifecycle_doc_ref: RECORDS_EXPORT_DELETE_LIFECYCLE_DOC_REF.to_owned(),
            lifecycle_artifact_ref: RECORDS_EXPORT_DELETE_LIFECYCLE_ARTIFACT_REF.to_owned(),
            lifecycle_packet,
            projection_rows,
            violations,
            raw_private_material_excluded: true,
            summary,
        }
    }
}
