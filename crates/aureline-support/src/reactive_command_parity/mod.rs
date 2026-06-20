//! Support-export consumer for the canonical reactive-command-parity packet.
//!
//! This module folds the checked-in command-parity packet into a metadata-safe
//! support-export envelope so support and diagnostics flows can quote the same
//! mutating surface, optimistic posture, divergence resolution, preserved
//! lineage, and rationale that the state packet freezes — without inventing
//! local optimistic-state wording. The export keeps the central guardrail
//! visible: no exported row claims success before the canonical command and
//! mutation journal publish.

use std::fmt;

use aureline_reactive_state::{
    seeded_reactive_command_parity_packet, validate_reactive_command_parity_packet, ParityFlowRow,
    ReactiveCommandParityDivergenceResolution, ReactiveCommandParityLineageField,
    ReactiveCommandParityMutatingSurface, ReactiveCommandParityMutationKind,
    ReactiveCommandParityOptimisticPosture, ReactiveCommandParityPacket,
    ReactiveCommandParityStateVisibility, ReactiveCommandParityValidationReport,
    REACTIVE_COMMAND_PARITY_DOC_REF, REACTIVE_COMMAND_PARITY_REPORT_REF,
    REACTIVE_COMMAND_PARITY_SCHEMA_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one support-export row.
pub const REACTIVE_COMMAND_PARITY_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "reactive_command_parity_support_export_row";

/// Stable record-kind tag for the support-export envelope.
pub const REACTIVE_COMMAND_PARITY_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "reactive_command_parity_support_export_envelope";

/// One support-export row copied from the canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveCommandParitySupportExportRow {
    /// Stable row record kind.
    pub record_kind: String,
    /// Stable flow id.
    pub flow_id: String,
    /// Mutating surface that published the change.
    pub mutating_surface: ReactiveCommandParityMutatingSurface,
    /// Kind of mutation the surface performed.
    pub mutation_kind: ReactiveCommandParityMutationKind,
    /// How the surface's optimistic path is handled.
    pub optimistic_posture: ReactiveCommandParityOptimisticPosture,
    /// Visibility shown before the canonical path publishes.
    pub state_before_publish: ReactiveCommandParityStateVisibility,
    /// Whether publication waits for the command graph to commit.
    pub publishes_after_command_commit: bool,
    /// Whether publication waits for the mutation journal to commit.
    pub publishes_after_journal_commit: bool,
    /// Whether the change is published via the reactive graph.
    pub publishes_via_reactive_graph: bool,
    /// Whether the surface claims success before publication.
    pub claims_success_before_publish: bool,
    /// How a canonical divergence is resolved.
    pub divergence_resolution: ReactiveCommandParityDivergenceResolution,
    /// Lineage the published mutation preserves.
    pub preserved_lineage: Vec<ReactiveCommandParityLineageField>,
    /// Support-safe summary of how the surface publishes.
    pub publication_summary: String,
    /// Support-safe summary of why the parity posture is honest.
    pub parity_rationale: String,
    /// Raw payloads remain excluded.
    pub raw_payload_excluded: bool,
    /// Ambient authority remains excluded.
    pub ambient_authority_excluded: bool,
}

impl ReactiveCommandParitySupportExportRow {
    fn from_flow(row: &ParityFlowRow) -> Self {
        Self {
            record_kind: REACTIVE_COMMAND_PARITY_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            flow_id: row.flow_id.clone(),
            mutating_surface: row.mutating_surface,
            mutation_kind: row.mutation_kind,
            optimistic_posture: row.optimistic_posture,
            state_before_publish: row.state_before_publish,
            publishes_after_command_commit: row.publishes_after_command_commit,
            publishes_after_journal_commit: row.publishes_after_journal_commit,
            publishes_via_reactive_graph: row.publishes_via_reactive_graph,
            claims_success_before_publish: row.claims_success_before_publish,
            divergence_resolution: row.divergence_resolution,
            preserved_lineage: row.preserved_lineage.clone(),
            publication_summary: row.publication_summary.clone(),
            parity_rationale: row.parity_rationale.clone(),
            raw_payload_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    /// Returns true when the row remains metadata-safe and support-usable.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.ambient_authority_excluded
            && self.publishes_after_command_commit
            && self.publishes_after_journal_commit
            && self.publishes_via_reactive_graph
            // The guardrail must survive export: no success before publish.
            && !self.claims_success_before_publish
            && !self.state_before_publish.claims_current_truth()
            && !self.preserved_lineage.is_empty()
            && !self.publication_summary.trim().is_empty()
            && !self.parity_rationale.trim().is_empty()
    }
}

/// Metadata-safe support-export envelope for reactive command parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveCommandParitySupportExportEnvelope {
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
    pub rows: Vec<ReactiveCommandParitySupportExportRow>,
}

impl ReactiveCommandParitySupportExportEnvelope {
    /// Builds an envelope from a validated packet.
    pub fn from_packet(
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
        packet: &ReactiveCommandParityPacket,
    ) -> Self {
        let mut rows: Vec<_> = packet
            .flows
            .iter()
            .map(ReactiveCommandParitySupportExportRow::from_flow)
            .collect();
        rows.sort_by(|a, b| a.flow_id.cmp(&b.flow_id));
        Self {
            record_kind: REACTIVE_COMMAND_PARITY_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            doc_ref: REACTIVE_COMMAND_PARITY_DOC_REF.to_owned(),
            schema_ref: REACTIVE_COMMAND_PARITY_SCHEMA_REF.to_owned(),
            report_ref: REACTIVE_COMMAND_PARITY_REPORT_REF.to_owned(),
            raw_payload_excluded: true,
            ambient_authority_excluded: true,
            rows,
        }
    }

    /// Returns true when the envelope remains metadata-safe and in sync with the
    /// canonical packet refs.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.ambient_authority_excluded
            && self.doc_ref == REACTIVE_COMMAND_PARITY_DOC_REF
            && self.schema_ref == REACTIVE_COMMAND_PARITY_SCHEMA_REF
            && self.report_ref == REACTIVE_COMMAND_PARITY_REPORT_REF
            && !self.rows.is_empty()
            && self
                .rows
                .iter()
                .all(ReactiveCommandParitySupportExportRow::is_export_safe)
    }
}

/// Error returned when the support envelope cannot be compiled.
#[derive(Debug)]
pub enum ReactiveCommandParitySupportExportError {
    /// The canonical packet failed validation.
    PacketValidation(ReactiveCommandParityValidationReport),
}

impl fmt::Display for ReactiveCommandParitySupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => {
                write!(f, "reactive-command-parity packet invalid: {report}")
            }
        }
    }
}

impl std::error::Error for ReactiveCommandParitySupportExportError {}

impl From<ReactiveCommandParityValidationReport> for ReactiveCommandParitySupportExportError {
    fn from(report: ReactiveCommandParityValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

/// Compiles the metadata-safe support-export envelope from the canonical
/// reactive-command-parity packet.
pub fn compile_support_export_envelope(
    envelope_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> Result<ReactiveCommandParitySupportExportEnvelope, ReactiveCommandParitySupportExportError> {
    let packet = seeded_reactive_command_parity_packet();
    validate_reactive_command_parity_packet(&packet)?;
    Ok(ReactiveCommandParitySupportExportEnvelope::from_packet(
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
            "envelope:reactive_command_parity:test",
            "2026-06-19T08:30:00Z",
        )
        .expect("envelope compiles");
        assert!(envelope.is_export_safe());
        assert_eq!(envelope.rows.len(), 6);

        let json = serde_json::to_string(&envelope).expect("envelope serializes");
        let parsed: ReactiveCommandParitySupportExportEnvelope =
            serde_json::from_str(&json).expect("envelope round-trips");
        assert_eq!(parsed, envelope);
    }

    #[test]
    fn no_exported_row_claims_success_before_publish() {
        let envelope = compile_support_export_envelope(
            "envelope:reactive_command_parity:guardrail",
            "2026-06-19T08:35:00Z",
        )
        .expect("envelope compiles");
        for row in &envelope.rows {
            assert!(
                !row.claims_success_before_publish,
                "support row {} must not export a pre-publish success claim",
                row.flow_id
            );
            assert!(
                row.publishes_after_command_commit && row.publishes_after_journal_commit,
                "support row {} must export the command and journal publication gate",
                row.flow_id
            );
            assert!(
                !row.state_before_publish.claims_current_truth(),
                "support row {} must not export published truth before publish",
                row.flow_id
            );
        }
    }

    #[test]
    fn provider_mutation_row_keeps_degrade_resolution() {
        let envelope = compile_support_export_envelope(
            "envelope:reactive_command_parity:provider",
            "2026-06-19T08:40:00Z",
        )
        .expect("envelope compiles");
        let row = envelope
            .rows
            .iter()
            .find(|row| row.flow_id == "provider_config_mutation")
            .expect("provider mutation row exists");
        assert_eq!(
            row.divergence_resolution,
            ReactiveCommandParityDivergenceResolution::DegradeSurface
        );
        assert!(
            row.preserved_lineage
                .contains(&ReactiveCommandParityLineageField::Command),
            "support row must keep command lineage for correlation"
        );
    }
}
