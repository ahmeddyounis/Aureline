//! Support-export consumer for graph drift packets.
//!
//! This module is the first non-graph consumer of the
//! [`aureline_graph::drift_packets`] projection. It folds the
//! checked-in drift corpus into a typed support-export envelope so the
//! local-first support pipeline can quote the same readiness,
//! freshness, scope, and data-lane lineage fields the in-product
//! chrome renders, without re-running graph producers and without
//! inferring freshness from timing alone.
//!
//! The envelope is metadata-safe by construction: it only ever holds
//! tokens drawn from the closed drift-packet vocabularies, the
//! `envelope_packet_ref` back to the alpha
//! [`aureline_graph::GraphFactCuePacket`], and the safety baseline the
//! drift packet declared.

use std::error::Error;
use std::fmt;

use aureline_graph::{
    current_graph_drift_corpus, DataLaneLineage, DriftConsumerSurface, DriftDowngradeLabel,
    DriftIndicator, DriftOpenGapClass, FreshnessClass, GraphDriftCorpus, GraphDriftPacket,
    GraphDriftPacketEvaluator, GraphDriftReport, ReadinessState, ScopeClass,
    GRAPH_DRIFT_PACKET_DOC_REF, GRAPH_DRIFT_PACKET_REPORT_REF, GRAPH_DRIFT_PACKET_SCHEMA_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for the support-export envelope row.
pub const GRAPH_DRIFT_SUPPORT_EXPORT_ROW_RECORD_KIND: &str = "graph_drift_support_export_row";

/// Stable record-kind tag for the support-export envelope itself.
pub const GRAPH_DRIFT_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "graph_drift_support_export_envelope";

/// One row of the support-export envelope. Quotes the closed-vocabulary
/// tokens the drift packet declared so the support pipeline can audit
/// graph drift without re-reading alpha envelopes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphDriftSupportExportRow {
    pub record_kind: String,
    pub packet_id: String,
    pub consumer_surface: DriftConsumerSurface,
    pub subject_ref: String,
    pub envelope_packet_ref: String,
    pub workspace_id: String,
    pub workspace_graph_id: String,
    pub readiness_state: ReadinessState,
    pub freshness_class: FreshnessClass,
    pub scope_class: ScopeClass,
    pub data_lane_lineage: DataLaneLineage,
    pub drift_indicator: DriftIndicator,
    pub downgrade_label: DriftDowngradeLabel,
    pub open_gap_classes: Vec<DriftOpenGapClass>,
}

impl GraphDriftSupportExportRow {
    fn from_packet(packet: &GraphDriftPacket) -> Self {
        let mut open_gap_classes: Vec<DriftOpenGapClass> =
            packet.open_gaps.iter().map(|gap| gap.gap_class).collect();
        if open_gap_classes.is_empty() {
            open_gap_classes.push(DriftOpenGapClass::None);
        }
        Self {
            record_kind: GRAPH_DRIFT_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            packet_id: packet.packet_id.clone(),
            consumer_surface: packet.consumer_surface,
            subject_ref: packet.subject_ref.clone(),
            envelope_packet_ref: packet.envelope_packet_ref.clone(),
            workspace_id: packet.workspace_id.clone(),
            workspace_graph_id: packet.workspace_graph_id.clone(),
            readiness_state: packet.readiness_state,
            freshness_class: packet.freshness_class,
            scope_class: packet.scope_class,
            data_lane_lineage: packet.data_lane_lineage,
            drift_indicator: packet.drift_indicator,
            downgrade_label: packet.downgrade_label,
            open_gap_classes,
        }
    }
}

/// Support-export envelope folded from the checked-in drift corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphDriftSupportExportEnvelope {
    pub record_kind: String,
    pub envelope_id: String,
    pub captured_at: String,
    pub doc_ref: String,
    pub schema_ref: String,
    pub report_ref: String,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub rows: Vec<GraphDriftSupportExportRow>,
}

impl GraphDriftSupportExportEnvelope {
    /// Builds the envelope by folding the [`GraphDriftReport`] matrix
    /// rows into one support-export row per packet.
    pub fn from_report(
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
        report: &GraphDriftReport,
        corpus: &GraphDriftCorpus,
    ) -> Self {
        let mut rows: Vec<GraphDriftSupportExportRow> = corpus
            .entries
            .iter()
            .map(|entry| GraphDriftSupportExportRow::from_packet(&entry.packet))
            .collect();
        rows.sort_by(|a, b| a.packet_id.cmp(&b.packet_id));
        Self {
            record_kind: GRAPH_DRIFT_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            doc_ref: report.doc_ref.clone(),
            schema_ref: report.schema_ref.clone(),
            report_ref: GRAPH_DRIFT_PACKET_REPORT_REF.to_owned(),
            raw_private_material_excluded: report.raw_private_material_excluded,
            ambient_authority_excluded: report.ambient_authority_excluded,
            rows,
        }
    }

    /// Returns true when the envelope is metadata-safe.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.doc_ref == GRAPH_DRIFT_PACKET_DOC_REF
            && self.schema_ref == GRAPH_DRIFT_PACKET_SCHEMA_REF
            && self.report_ref == GRAPH_DRIFT_PACKET_REPORT_REF
            && !self.rows.is_empty()
    }
}

/// Error returned when the support-export pipeline cannot compile a
/// drift envelope from the checked-in corpus.
#[derive(Debug)]
pub enum GraphDriftSupportExportError {
    CorpusParse(serde_yaml::Error),
    CorpusValidation(aureline_graph::GraphDriftValidationReport),
}

impl fmt::Display for GraphDriftSupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CorpusParse(err) => write!(f, "graph drift corpus parse: {err}"),
            Self::CorpusValidation(report) => write!(f, "graph drift corpus invalid: {report}"),
        }
    }
}

impl Error for GraphDriftSupportExportError {}

impl From<serde_yaml::Error> for GraphDriftSupportExportError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::CorpusParse(err)
    }
}

impl From<aureline_graph::GraphDriftValidationReport> for GraphDriftSupportExportError {
    fn from(err: aureline_graph::GraphDriftValidationReport) -> Self {
        Self::CorpusValidation(err)
    }
}

/// Compiles the support-export envelope from the checked-in drift
/// corpus. Support pipelines call this to load drift-truth into a
/// metadata-safe envelope without re-running graph producers.
pub fn compile_support_export_envelope(
    envelope_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> Result<GraphDriftSupportExportEnvelope, GraphDriftSupportExportError> {
    let envelope_id = envelope_id.into();
    let captured_at = captured_at.into();
    let corpus = current_graph_drift_corpus()?;
    let report = GraphDriftPacketEvaluator::new().report(
        format!("{envelope_id}:report"),
        captured_at.clone(),
        &corpus,
    )?;
    Ok(GraphDriftSupportExportEnvelope::from_report(
        envelope_id,
        captured_at,
        &report,
        &corpus,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_envelope_round_trip() {
        let envelope = compile_support_export_envelope(
            "envelope:graph_drift:test",
            "2026-05-16T10:00:00Z",
        )
        .expect("envelope compiles");
        assert!(envelope.is_export_safe());
        assert!(!envelope.rows.is_empty());

        let json = serde_json::to_string(&envelope).expect("envelope serializes");
        let parsed: GraphDriftSupportExportEnvelope =
            serde_json::from_str(&json).expect("envelope round-trips");
        assert_eq!(parsed, envelope);
    }
}
