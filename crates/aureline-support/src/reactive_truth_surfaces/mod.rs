//! Support-export consumer for the reactive-truth cue layer.
//!
//! This module folds the canonical reactive-truth-surfaces audit into a
//! metadata-safe support-export envelope so support, release, and
//! procurement readers can tell apart warming, cached, stale, partial,
//! coalesced, policy-limited, and provider-unavailable states — and see
//! how dangerous derived actions narrow under each — using the same
//! tokens the shell renders, without embedding raw payloads, private
//! material, or ambient authority.
//!
//! Release and support tooling can also call [`narrow_exported_cue`] to
//! render the cue for any observed state through the one canonical engine,
//! so an underqualified reactive-state row downgrades automatically.

use std::fmt;

use aureline_reactive_state::{
    build_reactive_truth_cue, seeded_reactive_truth_surfaces_packet,
    validate_reactive_truth_surfaces_packet, M5ReactiveAuthorityClass, M5ReactiveObservedState,
    M5ReactiveSurfaceClass, M5ReactiveTruthClaim, M5ReactiveViewClass, ReactiveTruthActionGate,
    ReactiveTruthCue, ReactiveTruthSurfacesError, ReactiveTruthSurfacesPacket,
    ReactiveTruthSurfacesValidationReport, REACTIVE_TRUTH_SURFACES_DOC_REF,
    REACTIVE_TRUTH_SURFACES_REPORT_REF, REACTIVE_TRUTH_SURFACES_SCHEMA_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one support-export row.
pub const REACTIVE_TRUTH_SURFACES_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "reactive_truth_surfaces_support_export_row";

/// Stable record-kind tag for the support-export envelope.
pub const REACTIVE_TRUTH_SURFACES_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "reactive_truth_surfaces_support_export_envelope";

/// One support-export row copied from the canonical audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveTruthSurfacesSupportExportRow {
    /// Stable row record kind.
    pub record_kind: String,
    /// Reactive surface class.
    pub surface_class: M5ReactiveSurfaceClass,
    /// Authority that owns the canonical truth.
    pub authority_class: M5ReactiveAuthorityClass,
    /// Materialized-view class.
    pub view_class: M5ReactiveViewClass,
    /// Epoch-parity group the surface must stay level with.
    pub epoch_parity_group_id: String,
    /// Strongest claim the surface presents when healthy.
    pub healthy_claim: M5ReactiveTruthClaim,
    /// Action gate at the healthy ceiling.
    pub healthy_action_gate: ReactiveTruthActionGate,
    /// Count of gated narrowing rules carried by the audit row.
    pub gated_rule_count: usize,
    /// Metadata-safe export invariant.
    pub raw_payload_excluded: bool,
    /// Metadata-safe export invariant.
    pub raw_private_material_excluded: bool,
    /// Metadata-safe export invariant.
    pub ambient_authority_excluded: bool,
}

impl ReactiveTruthSurfacesSupportExportRow {
    /// Whether the row preserves the metadata-safe export invariants.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
    }
}

/// Metadata-safe support-export envelope for the cue layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveTruthSurfacesSupportExport {
    /// Stable envelope record kind.
    pub record_kind: String,
    /// Source audit packet id.
    pub packet_id: String,
    /// Source audit schema version.
    pub schema_version: u32,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Report ref.
    pub report_ref: String,
    /// One row per reactive surface.
    pub rows: Vec<ReactiveTruthSurfacesSupportExportRow>,
    /// Invariant summary copied from the audit.
    pub invariants: Vec<String>,
}

impl ReactiveTruthSurfacesSupportExport {
    /// Whether every row preserves the metadata-safe export invariants.
    pub fn is_export_safe(&self) -> bool {
        self.rows
            .iter()
            .all(ReactiveTruthSurfacesSupportExportRow::is_export_safe)
    }
}

/// Error returned when the support export cannot be compiled.
#[derive(Debug)]
pub enum ReactiveTruthSurfacesSupportExportError {
    /// The canonical audit failed validation.
    PacketValidation(ReactiveTruthSurfacesValidationReport),
    /// A cue could not be rendered.
    Cue(ReactiveTruthSurfacesError),
}

impl fmt::Display for ReactiveTruthSurfacesSupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => {
                write!(f, "reactive truth surfaces invalid: {report}")
            }
            Self::Cue(err) => write!(f, "reactive truth cue unavailable: {err}"),
        }
    }
}

impl std::error::Error for ReactiveTruthSurfacesSupportExportError {}

impl From<ReactiveTruthSurfacesValidationReport> for ReactiveTruthSurfacesSupportExportError {
    fn from(report: ReactiveTruthSurfacesValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

impl From<ReactiveTruthSurfacesError> for ReactiveTruthSurfacesSupportExportError {
    fn from(err: ReactiveTruthSurfacesError) -> Self {
        Self::Cue(err)
    }
}

/// Compiles the metadata-safe support-export envelope from the canonical
/// audit.
///
/// # Errors
///
/// Returns [`ReactiveTruthSurfacesSupportExportError`] when the audit fails
/// validation.
pub fn compile_support_export_envelope(
) -> Result<ReactiveTruthSurfacesSupportExport, ReactiveTruthSurfacesSupportExportError> {
    let packet = seeded_reactive_truth_surfaces_packet();
    validate_reactive_truth_surfaces_packet(&packet)?;
    Ok(envelope_from_packet(&packet))
}

/// Renders the cue for a surface and observed state through the canonical
/// engine. Release and support tooling use this to downgrade underqualified
/// reactive-state rows automatically.
///
/// # Errors
///
/// Returns [`ReactiveTruthSurfacesSupportExportError`] when the surface is
/// unknown or the matrix fails validation.
pub fn narrow_exported_cue(
    surface_class: M5ReactiveSurfaceClass,
    observed: &M5ReactiveObservedState,
) -> Result<ReactiveTruthCue, ReactiveTruthSurfacesSupportExportError> {
    Ok(build_reactive_truth_cue(surface_class, *observed)?)
}

fn envelope_from_packet(
    packet: &ReactiveTruthSurfacesPacket,
) -> ReactiveTruthSurfacesSupportExport {
    let mut rows: Vec<_> = packet
        .surfaces
        .iter()
        .map(|audit| ReactiveTruthSurfacesSupportExportRow {
            record_kind: REACTIVE_TRUTH_SURFACES_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            surface_class: audit.surface_class,
            authority_class: audit.authority_class,
            view_class: audit.view_class,
            epoch_parity_group_id: audit.epoch_parity_group_id.clone(),
            healthy_claim: audit.healthy_claim,
            healthy_action_gate: audit.healthy_action_gate,
            gated_rule_count: audit.gated_narrowing_rules.len(),
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        })
        .collect();
    rows.sort_by(|a, b| a.surface_class.as_str().cmp(b.surface_class.as_str()));
    ReactiveTruthSurfacesSupportExport {
        record_kind: REACTIVE_TRUTH_SURFACES_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
        packet_id: packet.packet_id.clone(),
        schema_version: packet.schema_version,
        doc_ref: REACTIVE_TRUTH_SURFACES_DOC_REF.to_owned(),
        schema_ref: REACTIVE_TRUTH_SURFACES_SCHEMA_REF.to_owned(),
        report_ref: REACTIVE_TRUTH_SURFACES_REPORT_REF.to_owned(),
        rows,
        invariants: packet.invariants.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_reactive_state::{
        M5ReactiveBackpressureMode, M5ReactiveCompleteness, M5ReactiveFreshness,
    };

    #[test]
    fn envelope_is_export_safe_and_complete() {
        let envelope = compile_support_export_envelope().expect("envelope compiles");
        assert!(envelope.is_export_safe());
        assert_eq!(envelope.rows.len(), 13);
        assert_eq!(
            envelope.record_kind,
            REACTIVE_TRUTH_SURFACES_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND
        );
        for row in &envelope.rows {
            assert_ne!(row.healthy_claim, M5ReactiveTruthClaim::ExactCurrentTruth);
            assert_eq!(row.healthy_action_gate, ReactiveTruthActionGate::Enabled);
        }
    }

    #[test]
    fn envelope_serializes_without_raw_payloads() {
        let envelope = compile_support_export_envelope().expect("envelope compiles");
        let json = serde_json::to_string(&envelope).expect("serializes");
        assert!(json.contains("\"raw_payload_excluded\":true"));
        assert!(json.contains("provider_overlay"));
        assert!(json.contains("epoch_parity:"));
    }

    #[test]
    fn release_reader_sees_a_blocked_stale_cue() {
        let observed = M5ReactiveObservedState {
            freshness: M5ReactiveFreshness::Stale,
            completeness: M5ReactiveCompleteness::Full,
            backpressure_mode: M5ReactiveBackpressureMode::Realtime,
            terminal_reason: None,
            policy_limited: false,
        };
        let cue =
            narrow_exported_cue(M5ReactiveSurfaceClass::SupportExportView, &observed).expect("cue");
        assert_eq!(cue.narrowed_claim, M5ReactiveTruthClaim::StaleSnapshot);
        assert_eq!(cue.action_gate, ReactiveTruthActionGate::Blocked);
        assert!(!cue.dangerous_action_enabled);
    }
}
