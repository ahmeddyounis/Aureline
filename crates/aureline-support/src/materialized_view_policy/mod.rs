//! Support-export consumer for the canonical materialized-view-class
//! policy.
//!
//! This module folds the checked-in materialized-view-class policy into a
//! metadata-safe support-export envelope so support, clear-data,
//! offboarding, restore, and release tooling can quote one table for how
//! each view class persists, retains, exports, deletes, holds, and
//! contributes to a support bundle — instead of inferring behavior from a
//! storage location.
//!
//! [`bundle_disposition`] and [`clear_data_disposition`] expose the two
//! dispositions support flows reach for most often; the full grid is
//! available through [`aureline_reactive_state::materialized_view_disposition_for`].

use std::fmt;

use aureline_reactive_state::{
    materialized_view_disposition_for, seeded_materialized_view_policy,
    validate_materialized_view_policy, MaterializedViewClearDataSemantics,
    MaterializedViewDisposition, MaterializedViewExportClass,
    MaterializedViewHoldOffboardingSemantics, MaterializedViewLifecycleOperation,
    MaterializedViewPersistenceClass, MaterializedViewPolicyValidationReport,
    MaterializedViewPolicyViewClass, MaterializedViewRetentionClass,
    MaterializedViewSupportBundleSemantics, MATERIALIZED_VIEW_POLICY_DOC_REF,
    MATERIALIZED_VIEW_POLICY_REPORT_REF, MATERIALIZED_VIEW_POLICY_SCHEMA_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one support-export row.
pub const MATERIALIZED_VIEW_POLICY_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "materialized_view_policy_support_export_row";

/// Stable record-kind tag for the support-export envelope.
pub const MATERIALIZED_VIEW_POLICY_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "materialized_view_policy_support_export_envelope";

/// One support-export row copied from the canonical policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedViewPolicySupportExportRow {
    /// Stable row record kind.
    pub record_kind: String,
    /// The materialized-view class.
    pub view_class: MaterializedViewPolicyViewClass,
    /// Where the class lives.
    pub persistence: MaterializedViewPersistenceClass,
    /// How long the class is retained.
    pub retention: MaterializedViewRetentionClass,
    /// Whether / how the class may be exported.
    pub export: MaterializedViewExportClass,
    /// What clear-data does to the class.
    pub delete_semantics: MaterializedViewClearDataSemantics,
    /// How the class behaves under hold / offboarding.
    pub hold_offboarding: MaterializedViewHoldOffboardingSemantics,
    /// What the class contributes to a support bundle.
    pub support_bundle: MaterializedViewSupportBundleSemantics,
    /// Disposition of the class when added to a support bundle.
    pub bundle_disposition: MaterializedViewDisposition,
    /// Disposition of the class under a clear-data sweep.
    pub clear_data_disposition: MaterializedViewDisposition,
    /// Whether persisted state survives a clear-data sweep.
    pub survives_clear_data: bool,
    /// Metadata-safe export invariant.
    pub raw_payload_excluded: bool,
    /// Metadata-safe export invariant.
    pub raw_private_material_excluded: bool,
    /// Metadata-safe export invariant.
    pub ambient_authority_excluded: bool,
}

impl MaterializedViewPolicySupportExportRow {
    /// Whether the row preserves the metadata-safe export invariants.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
    }
}

/// Metadata-safe support-export envelope for the policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedViewPolicySupportExport {
    /// Stable envelope record kind.
    pub record_kind: String,
    /// Source policy packet id.
    pub packet_id: String,
    /// Source policy schema version.
    pub schema_version: u32,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Report ref.
    pub report_ref: String,
    /// One row per materialized-view class.
    pub rows: Vec<MaterializedViewPolicySupportExportRow>,
    /// Invariant summary copied from the policy.
    pub invariants: Vec<String>,
}

impl MaterializedViewPolicySupportExport {
    /// Whether every row preserves the metadata-safe export invariants.
    pub fn is_export_safe(&self) -> bool {
        self.rows
            .iter()
            .all(MaterializedViewPolicySupportExportRow::is_export_safe)
    }
}

/// Error returned when the support export cannot be compiled.
#[derive(Debug)]
pub enum MaterializedViewPolicySupportExportError {
    /// The canonical policy failed validation.
    PacketValidation(MaterializedViewPolicyValidationReport),
}

impl fmt::Display for MaterializedViewPolicySupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => {
                write!(f, "materialized view policy invalid: {report}")
            }
        }
    }
}

impl std::error::Error for MaterializedViewPolicySupportExportError {}

impl From<MaterializedViewPolicyValidationReport> for MaterializedViewPolicySupportExportError {
    fn from(report: MaterializedViewPolicyValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

/// Returns the support-bundle disposition for a materialized-view class.
pub fn bundle_disposition(
    view_class: MaterializedViewPolicyViewClass,
) -> MaterializedViewDisposition {
    materialized_view_disposition_for(
        view_class,
        MaterializedViewLifecycleOperation::SupportBundle,
    )
}

/// Returns the clear-data disposition for a materialized-view class.
pub fn clear_data_disposition(
    view_class: MaterializedViewPolicyViewClass,
) -> MaterializedViewDisposition {
    materialized_view_disposition_for(view_class, MaterializedViewLifecycleOperation::ClearData)
}

/// Compiles the metadata-safe support-export envelope from the canonical
/// policy.
///
/// # Errors
///
/// Returns [`MaterializedViewPolicySupportExportError`] when the policy
/// fails validation.
pub fn compile_support_export_envelope(
) -> Result<MaterializedViewPolicySupportExport, MaterializedViewPolicySupportExportError> {
    let packet = seeded_materialized_view_policy();
    validate_materialized_view_policy(&packet)?;

    let rows = packet
        .classes
        .iter()
        .map(|row| MaterializedViewPolicySupportExportRow {
            record_kind: MATERIALIZED_VIEW_POLICY_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            view_class: row.view_class,
            persistence: row.persistence,
            retention: row.retention,
            export: row.export,
            delete_semantics: row.delete_semantics,
            hold_offboarding: row.hold_offboarding,
            support_bundle: row.support_bundle,
            bundle_disposition: bundle_disposition(row.view_class),
            clear_data_disposition: clear_data_disposition(row.view_class),
            survives_clear_data: row.survives_clear_data,
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        })
        .collect();

    Ok(MaterializedViewPolicySupportExport {
        record_kind: MATERIALIZED_VIEW_POLICY_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
        packet_id: packet.packet_id.clone(),
        schema_version: packet.schema_version,
        doc_ref: MATERIALIZED_VIEW_POLICY_DOC_REF.to_owned(),
        schema_ref: MATERIALIZED_VIEW_POLICY_SCHEMA_REF.to_owned(),
        report_ref: MATERIALIZED_VIEW_POLICY_REPORT_REF.to_owned(),
        rows,
        invariants: packet.invariants.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_is_export_safe_and_complete() {
        let envelope = compile_support_export_envelope().expect("envelope compiles");
        assert!(envelope.is_export_safe());
        assert_eq!(envelope.rows.len(), 4);
        assert_eq!(
            envelope.record_kind,
            MATERIALIZED_VIEW_POLICY_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND
        );
    }

    #[test]
    fn exportable_snapshot_survives_clear_data_in_export() {
        let envelope = compile_support_export_envelope().expect("envelope compiles");
        let exportable = envelope
            .rows
            .iter()
            .find(|r| r.view_class == MaterializedViewPolicyViewClass::ExportableSnapshot)
            .expect("exportable class present");
        assert!(exportable.survives_clear_data);
        assert_eq!(
            exportable.clear_data_disposition,
            MaterializedViewDisposition::SavedArtifactPreserved
        );
    }

    #[test]
    fn ephemeral_is_excluded_from_bundle() {
        assert_eq!(
            bundle_disposition(MaterializedViewPolicyViewClass::EphemeralProjection),
            MaterializedViewDisposition::ExcludedFromBundle
        );
    }

    #[test]
    fn envelope_serializes_without_raw_payloads() {
        let envelope = compile_support_export_envelope().expect("envelope compiles");
        let json = serde_json::to_string(&envelope).expect("serializes");
        assert!(json.contains("\"raw_payload_excluded\":true"));
        assert!(json.contains("managed_replicated_view"));
        assert!(json.contains("preserve_saved_artifact"));
    }
}
