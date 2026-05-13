use std::collections::BTreeMap;

use crate::lsp_router::{RedactionClass, RouterDecisionRecord};

use super::records::{
    DiagnosticBusAggregateCounts, DiagnosticBusSnapshot, DiagnosticEnvelope,
    DiagnosticProviderAvailabilityRow, DIAGNOSTIC_BUS_SCHEMA_VERSION,
};

/// Snapshot request for a diagnostic bus collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticBusSnapshotRequest {
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Workspace id covered by the snapshot.
    pub workspace_id: String,
    /// Diagnostic collection id.
    pub collection_id: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// In-memory diagnostic bus for one workspace or diagnostic collection.
#[derive(Debug, Clone, Default)]
pub struct DiagnosticBus {
    diagnostics: BTreeMap<String, DiagnosticEnvelope>,
    provider_rows: BTreeMap<String, DiagnosticProviderAvailabilityRow>,
}

impl DiagnosticBus {
    /// Builds an empty diagnostic bus.
    pub fn new() -> Self {
        Self::default()
    }

    /// Publishes or replaces one normalized diagnostic envelope.
    pub fn publish(&mut self, diagnostic: DiagnosticEnvelope) {
        self.diagnostics
            .insert(diagnostic.diagnostic_id.clone(), diagnostic);
    }

    /// Publishes or replaces one provider availability row.
    pub fn ingest_provider_availability(&mut self, row: DiagnosticProviderAvailabilityRow) {
        self.provider_rows.insert(row.provider_id.clone(), row);
    }

    /// Ingests provider availability rows from an LSP router decision.
    pub fn ingest_router_decision(&mut self, decision: &RouterDecisionRecord) {
        for row in &decision.provider_stack_rows {
            self.ingest_provider_availability(
                DiagnosticProviderAvailabilityRow::from_provider_stack_row(row, decision),
            );
        }
    }

    /// Returns true when the bus has no diagnostics and no provider state.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty() && self.provider_rows.is_empty()
    }

    /// Builds a deterministic diagnostic bus snapshot.
    pub fn snapshot(&self, request: DiagnosticBusSnapshotRequest) -> DiagnosticBusSnapshot {
        let diagnostics = self.diagnostics.values().cloned().collect::<Vec<_>>();
        let provider_availability_rows = self.provider_rows.values().cloned().collect::<Vec<_>>();
        let aggregate_counts =
            DiagnosticBusAggregateCounts::from_rows(&diagnostics, &provider_availability_rows);
        let total_count = aggregate_counts.total_count;
        let provider_count = provider_availability_rows.len();

        DiagnosticBusSnapshot {
            record_kind: DiagnosticBusSnapshot::RECORD_KIND.into(),
            diagnostic_bus_schema_version: DIAGNOSTIC_BUS_SCHEMA_VERSION,
            snapshot_id: request.snapshot_id,
            workspace_id: request.workspace_id,
            collection_id: request.collection_id,
            diagnostics,
            provider_availability_rows,
            aggregate_counts,
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: request.captured_at,
            export_safe_summary: format!(
                "Diagnostic bus snapshot contains {total_count} diagnostics and {provider_count} provider availability rows."
            ),
        }
    }
}
