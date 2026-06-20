//! Support-export consumer for the canonical M5 reactive-governance matrix.
//!
//! This module folds the checked-in reactive-governance matrix into a
//! metadata-safe support-export envelope so support, release, and
//! procurement readers can quote the same authority classes, view
//! classes, healthy claims, and automatic claim-narrowing behavior the
//! shell explainer renders — without embedding raw payloads, private
//! material, or ambient authority.
//!
//! Release and support tooling can also call
//! [`narrow_exported_claim`] to detect underqualified reactive-state
//! rows and downgrade a claim from any observed state through the one
//! canonical engine.

use std::fmt;

use aureline_reactive_state::{
    narrow_m5_reactive_truth_claim, seeded_m5_reactive_governance_packet,
    validate_m5_reactive_governance_packet, M5ReactiveAuthorityClass, M5ReactiveBackpressureMode,
    M5ReactiveCompleteness, M5ReactiveDerivationClass, M5ReactiveFreshness,
    M5ReactiveGovernancePacket, M5ReactiveGovernanceValidationReport, M5ReactiveInvalidationReason,
    M5ReactiveObservedState, M5ReactiveScopeClass, M5ReactiveSurfaceClass, M5ReactiveTruthClaim,
    M5ReactiveViewClass, M5_REACTIVE_GOVERNANCE_DOC_REF, M5_REACTIVE_GOVERNANCE_REPORT_REF,
    M5_REACTIVE_GOVERNANCE_SCHEMA_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one support-export row.
pub const M5_REACTIVE_GOVERNANCE_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "m5_reactive_governance_support_export_row";

/// Stable record-kind tag for the support-export envelope.
pub const M5_REACTIVE_GOVERNANCE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "m5_reactive_governance_support_export_envelope";

/// One support-export row copied from the canonical matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReactiveGovernanceSupportExportRow {
    /// Stable row record kind.
    pub record_kind: String,
    /// Reactive surface class.
    pub surface_class: M5ReactiveSurfaceClass,
    /// Authority that owns the canonical truth.
    pub authority_class: M5ReactiveAuthorityClass,
    /// Whether the surface is authoritative or derived.
    pub derivation_class: M5ReactiveDerivationClass,
    /// Subscription scope class.
    pub scope_class: M5ReactiveScopeClass,
    /// Materialized-view class.
    pub view_class: M5ReactiveViewClass,
    /// Canonical query family.
    pub query_family: String,
    /// Strongest claim the surface presents when healthy.
    pub healthy_claim: M5ReactiveTruthClaim,
    /// Degraded freshness states the surface can present.
    pub supported_freshness: Vec<M5ReactiveFreshness>,
    /// Degraded completeness states the surface can present.
    pub supported_completeness: Vec<M5ReactiveCompleteness>,
    /// Non-realtime backpressure modes the surface can experience.
    pub supported_backpressure: Vec<M5ReactiveBackpressureMode>,
    /// Invalidation reasons the surface honors.
    pub honored_invalidation_reasons: Vec<M5ReactiveInvalidationReason>,
    /// Count of computed narrowing rules carried by the matrix row.
    pub narrowing_rule_count: usize,
    /// Metadata-safe export invariant.
    pub raw_payload_excluded: bool,
    /// Metadata-safe export invariant.
    pub raw_private_material_excluded: bool,
    /// Metadata-safe export invariant.
    pub ambient_authority_excluded: bool,
}

impl M5ReactiveGovernanceSupportExportRow {
    /// Whether the row preserves the metadata-safe export invariants.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
    }
}

/// Metadata-safe support-export envelope for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReactiveGovernanceSupportExport {
    /// Stable envelope record kind.
    pub record_kind: String,
    /// Source matrix packet id.
    pub packet_id: String,
    /// Source matrix schema version.
    pub schema_version: u32,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Report ref.
    pub report_ref: String,
    /// One row per reactive surface.
    pub rows: Vec<M5ReactiveGovernanceSupportExportRow>,
    /// Invariant summary copied from the matrix.
    pub invariants: Vec<String>,
}

impl M5ReactiveGovernanceSupportExport {
    /// Whether every row preserves the metadata-safe export invariants.
    pub fn is_export_safe(&self) -> bool {
        self.rows
            .iter()
            .all(M5ReactiveGovernanceSupportExportRow::is_export_safe)
    }
}

/// Error returned when the support export cannot be compiled.
#[derive(Debug)]
pub enum M5ReactiveGovernanceSupportExportError {
    /// The canonical matrix failed validation.
    PacketValidation(M5ReactiveGovernanceValidationReport),
    /// A surface is missing from the matrix.
    UnknownSurface(M5ReactiveSurfaceClass),
}

impl fmt::Display for M5ReactiveGovernanceSupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => {
                write!(f, "m5 reactive governance invalid: {report}")
            }
            Self::UnknownSurface(surface) => {
                write!(f, "unknown reactive surface: {}", surface.as_str())
            }
        }
    }
}

impl std::error::Error for M5ReactiveGovernanceSupportExportError {}

impl From<M5ReactiveGovernanceValidationReport> for M5ReactiveGovernanceSupportExportError {
    fn from(report: M5ReactiveGovernanceValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

/// Compiles the metadata-safe support-export envelope from the canonical
/// matrix.
///
/// # Errors
///
/// Returns [`M5ReactiveGovernanceSupportExportError`] when the matrix
/// fails validation.
pub fn compile_support_export_envelope(
) -> Result<M5ReactiveGovernanceSupportExport, M5ReactiveGovernanceSupportExportError> {
    let packet = seeded_m5_reactive_governance_packet();
    validate_m5_reactive_governance_packet(&packet)?;
    Ok(envelope_from_packet(&packet))
}

/// Downgrades a claim from an observed subscription state through the
/// canonical narrowing engine. Release and support tooling use this to
/// detect underqualified reactive-state rows automatically.
pub fn narrow_exported_claim(observed: &M5ReactiveObservedState) -> M5ReactiveTruthClaim {
    narrow_m5_reactive_truth_claim(M5ReactiveDerivationClass::Derived, observed).claim
}

fn envelope_from_packet(packet: &M5ReactiveGovernancePacket) -> M5ReactiveGovernanceSupportExport {
    let mut rows: Vec<_> = packet
        .surfaces
        .iter()
        .map(|row| M5ReactiveGovernanceSupportExportRow {
            record_kind: M5_REACTIVE_GOVERNANCE_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            surface_class: row.surface_class,
            authority_class: row.authority_class,
            derivation_class: row.derivation_class,
            scope_class: row.scope_class,
            view_class: row.view_class,
            query_family: row.query_family.clone(),
            healthy_claim: row.healthy_claim,
            supported_freshness: row.supported_freshness.clone(),
            supported_completeness: row.supported_completeness.clone(),
            supported_backpressure: row.supported_backpressure.clone(),
            honored_invalidation_reasons: row.honored_invalidation_reasons.clone(),
            narrowing_rule_count: row.claim_narrowing_rules.len(),
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        })
        .collect();
    rows.sort_by(|a, b| a.surface_class.as_str().cmp(b.surface_class.as_str()));
    M5ReactiveGovernanceSupportExport {
        record_kind: M5_REACTIVE_GOVERNANCE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
        packet_id: packet.packet_id.clone(),
        schema_version: packet.schema_version,
        doc_ref: M5_REACTIVE_GOVERNANCE_DOC_REF.to_owned(),
        schema_ref: M5_REACTIVE_GOVERNANCE_SCHEMA_REF.to_owned(),
        report_ref: M5_REACTIVE_GOVERNANCE_REPORT_REF.to_owned(),
        rows,
        invariants: packet.invariants.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_is_export_safe_and_complete() {
        let envelope = compile_support_export_envelope().expect("envelope compiles");
        assert!(envelope.is_export_safe());
        assert_eq!(envelope.rows.len(), 13);
        assert_eq!(
            envelope.record_kind,
            M5_REACTIVE_GOVERNANCE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND
        );
        for row in &envelope.rows {
            assert_eq!(row.derivation_class, M5ReactiveDerivationClass::Derived);
            assert_ne!(row.healthy_claim, M5ReactiveTruthClaim::ExactCurrentTruth);
        }
    }

    #[test]
    fn envelope_serializes_without_raw_payloads() {
        let envelope = compile_support_export_envelope().expect("envelope compiles");
        let json = serde_json::to_string(&envelope).expect("serializes");
        assert!(json.contains("\"raw_payload_excluded\":true"));
        assert!(json.contains("provider_overlay"));
        assert!(json.contains("managed_replicated_view"));
    }

    #[test]
    fn narrowing_downgrades_an_imported_snapshot() {
        let observed = M5ReactiveObservedState {
            freshness: M5ReactiveFreshness::Imported,
            completeness: M5ReactiveCompleteness::Partial,
            backpressure_mode: M5ReactiveBackpressureMode::Realtime,
            terminal_reason: None,
            policy_limited: false,
        };
        assert_eq!(
            narrow_exported_claim(&observed),
            M5ReactiveTruthClaim::ImportedSnapshot
        );
    }
}
