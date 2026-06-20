//! Support-export consumer for the canonical reactive-recovery packet.
//!
//! This module folds the checked-in recovery packet into a metadata-safe
//! support-export envelope so support and diagnostics flows can quote the same
//! lag condition, recovery strategy, epoch posture, action posture, preserved
//! context, and rationale that the state packet freezes — without inventing
//! local stale-state wording.

use std::fmt;

use aureline_reactive_state::{
    seeded_reactive_recovery_packet, validate_reactive_recovery_packet,
    ReactiveRecoveryActionPosture, ReactiveRecoveryConsumerSurface, ReactiveRecoveryEpochPosture,
    ReactiveRecoveryFlowRow, ReactiveRecoveryLagCondition, ReactiveRecoveryPacket,
    ReactiveRecoveryPreservedContextClass, ReactiveRecoveryStrategy,
    ReactiveRecoveryValidationReport, REACTIVE_RECOVERY_DOC_REF, REACTIVE_RECOVERY_REPORT_REF,
    REACTIVE_RECOVERY_SCHEMA_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one support-export row.
pub const REACTIVE_RECOVERY_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "reactive_recovery_support_export_row";

/// Stable record-kind tag for the support-export envelope.
pub const REACTIVE_RECOVERY_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "reactive_recovery_support_export_envelope";

/// One support-export row copied from the canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveRecoverySupportExportRow {
    /// Stable row record kind.
    pub record_kind: String,
    /// Stable flow id.
    pub flow_id: String,
    /// Consumer surface that fell behind.
    pub consumer_surface: ReactiveRecoveryConsumerSurface,
    /// Condition that put the consumer behind.
    pub lag_condition: ReactiveRecoveryLagCondition,
    /// Primary recovery strategy.
    pub primary_strategy: ReactiveRecoveryStrategy,
    /// Fallback strategies that remain available.
    pub fallback_strategies: Vec<ReactiveRecoveryStrategy>,
    /// Epoch posture while recovering.
    pub epoch_posture: ReactiveRecoveryEpochPosture,
    /// Action posture while recovering.
    pub action_posture: ReactiveRecoveryActionPosture,
    /// Whether exact-truth actions are offered.
    pub offers_exact_truth_action: bool,
    /// Whether a silent retry is allowed.
    pub silent_retry_allowed: bool,
    /// Context kept visible and honest during recovery.
    pub preserved_context: Vec<ReactiveRecoveryPreservedContextClass>,
    /// Support-safe summary of how the consumer recovers.
    pub recovery_summary: String,
    /// Support-safe summary of why the truth posture is honest.
    pub truth_posture_rationale: String,
    /// Raw payloads remain excluded.
    pub raw_payload_excluded: bool,
    /// Ambient authority remains excluded.
    pub ambient_authority_excluded: bool,
}

impl ReactiveRecoverySupportExportRow {
    fn from_flow(row: &ReactiveRecoveryFlowRow) -> Self {
        Self {
            record_kind: REACTIVE_RECOVERY_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            flow_id: row.flow_id.clone(),
            consumer_surface: row.consumer_surface,
            lag_condition: row.lag_condition,
            primary_strategy: row.primary_strategy,
            fallback_strategies: row.fallback_strategies.clone(),
            epoch_posture: row.epoch_posture,
            action_posture: row.action_posture,
            offers_exact_truth_action: row.offers_exact_truth_action,
            silent_retry_allowed: row.silent_retry_allowed,
            preserved_context: row.preserved_context.clone(),
            recovery_summary: row.recovery_summary.clone(),
            truth_posture_rationale: row.truth_posture_rationale.clone(),
            raw_payload_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    /// Returns true when the row remains metadata-safe and support-usable.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.ambient_authority_excluded
            && !self.fallback_strategies.is_empty()
            && !self.preserved_context.is_empty()
            && !self.recovery_summary.trim().is_empty()
            && !self.truth_posture_rationale.trim().is_empty()
            // A support row must never imply an exact-truth action survived recovery.
            && (!self.offers_exact_truth_action || self.epoch_posture.is_current())
    }
}

/// Metadata-safe support-export envelope for reactive recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveRecoverySupportExportEnvelope {
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
    /// Ambient authority remains excluded.
    pub ambient_authority_excluded: bool,
    /// Export rows.
    pub rows: Vec<ReactiveRecoverySupportExportRow>,
}

impl ReactiveRecoverySupportExportEnvelope {
    /// Builds an envelope from a validated packet.
    pub fn from_packet(
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
        packet: &ReactiveRecoveryPacket,
    ) -> Self {
        let mut rows: Vec<_> = packet
            .flows
            .iter()
            .map(ReactiveRecoverySupportExportRow::from_flow)
            .collect();
        rows.sort_by(|a, b| a.flow_id.cmp(&b.flow_id));
        Self {
            record_kind: REACTIVE_RECOVERY_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            doc_ref: REACTIVE_RECOVERY_DOC_REF.to_owned(),
            schema_ref: REACTIVE_RECOVERY_SCHEMA_REF.to_owned(),
            report_ref: REACTIVE_RECOVERY_REPORT_REF.to_owned(),
            raw_payload_excluded: true,
            ambient_authority_excluded: true,
            rows,
        }
    }

    /// Returns true when the envelope remains metadata-safe and in sync with
    /// the canonical packet refs.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.ambient_authority_excluded
            && self.doc_ref == REACTIVE_RECOVERY_DOC_REF
            && self.schema_ref == REACTIVE_RECOVERY_SCHEMA_REF
            && self.report_ref == REACTIVE_RECOVERY_REPORT_REF
            && !self.rows.is_empty()
            && self
                .rows
                .iter()
                .all(ReactiveRecoverySupportExportRow::is_export_safe)
    }
}

/// Error returned when the support envelope cannot be compiled.
#[derive(Debug)]
pub enum ReactiveRecoverySupportExportError {
    /// The canonical packet failed validation.
    PacketValidation(ReactiveRecoveryValidationReport),
}

impl fmt::Display for ReactiveRecoverySupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => {
                write!(f, "reactive-recovery packet invalid: {report}")
            }
        }
    }
}

impl std::error::Error for ReactiveRecoverySupportExportError {}

impl From<ReactiveRecoveryValidationReport> for ReactiveRecoverySupportExportError {
    fn from(report: ReactiveRecoveryValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

/// Compiles the metadata-safe support-export envelope from the canonical
/// reactive-recovery packet.
pub fn compile_support_export_envelope(
    envelope_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> Result<ReactiveRecoverySupportExportEnvelope, ReactiveRecoverySupportExportError> {
    let packet = seeded_reactive_recovery_packet();
    validate_reactive_recovery_packet(&packet)?;
    Ok(ReactiveRecoverySupportExportEnvelope::from_packet(
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
            "envelope:reactive_recovery:test",
            "2026-06-19T08:30:00Z",
        )
        .expect("envelope compiles");
        assert!(envelope.is_export_safe());
        assert_eq!(envelope.rows.len(), 9);

        let json = serde_json::to_string(&envelope).expect("envelope serializes");
        let parsed: ReactiveRecoverySupportExportEnvelope =
            serde_json::from_str(&json).expect("envelope round-trips");
        assert_eq!(parsed, envelope);
    }

    #[test]
    fn no_exported_row_offers_exact_truth_while_behind() {
        let envelope = compile_support_export_envelope(
            "envelope:reactive_recovery:exact_truth",
            "2026-06-19T08:35:00Z",
        )
        .expect("envelope compiles");
        for row in &envelope.rows {
            assert!(
                !row.offers_exact_truth_action,
                "support row {} must not export an exact-truth action while behind",
                row.flow_id
            );
            assert!(
                !row.silent_retry_allowed,
                "support row {} must not export a silent retry",
                row.flow_id
            );
        }
    }

    #[test]
    fn provider_overlay_row_stays_blocked() {
        let envelope = compile_support_export_envelope(
            "envelope:reactive_recovery:provider_overlay",
            "2026-06-19T08:40:00Z",
        )
        .expect("envelope compiles");
        let row = envelope
            .rows
            .iter()
            .find(|row| row.flow_id == "review_workspace_provider_overlay_disappeared")
            .expect("provider overlay row exists");
        assert_eq!(row.action_posture, ReactiveRecoveryActionPosture::Blocked);
        assert_eq!(row.epoch_posture, ReactiveRecoveryEpochPosture::StaleEpoch);
    }
}
