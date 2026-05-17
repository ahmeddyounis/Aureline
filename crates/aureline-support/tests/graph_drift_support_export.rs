//! Integration drill for the support-export consumer of graph drift
//! packets.
//!
//! The drill re-proves that the support-export envelope folded from
//! the checked-in graph drift corpus preserves the readiness, freshness,
//! scope, lineage, and consumer-surface tokens the in-product chrome
//! renders, without re-running graph producers and without inferring
//! freshness from timing alone.

use aureline_graph::{
    current_graph_drift_corpus, GraphDriftPacketEvaluator, GRAPH_DRIFT_PACKET_DOC_REF,
    GRAPH_DRIFT_PACKET_REPORT_REF, GRAPH_DRIFT_PACKET_SCHEMA_REF,
};
use aureline_support::graph_drift::{
    compile_support_export_envelope, GraphDriftSupportExportEnvelope,
    GRAPH_DRIFT_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND, GRAPH_DRIFT_SUPPORT_EXPORT_ROW_RECORD_KIND,
};

#[test]
fn support_export_envelope_compiles_from_checked_in_corpus() {
    let envelope =
        compile_support_export_envelope("envelope:graph_drift:test", "2026-05-16T10:00:00Z")
            .expect("envelope compiles from checked-in drift corpus");
    assert_eq!(
        envelope.record_kind,
        GRAPH_DRIFT_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND
    );
    assert!(envelope.is_export_safe());
    assert_eq!(envelope.doc_ref, GRAPH_DRIFT_PACKET_DOC_REF);
    assert_eq!(envelope.schema_ref, GRAPH_DRIFT_PACKET_SCHEMA_REF);
    assert_eq!(envelope.report_ref, GRAPH_DRIFT_PACKET_REPORT_REF);

    let corpus = current_graph_drift_corpus().expect("corpus loads");
    assert_eq!(envelope.rows.len(), corpus.entries.len());
    for row in &envelope.rows {
        assert_eq!(row.record_kind, GRAPH_DRIFT_SUPPORT_EXPORT_ROW_RECORD_KIND);
        // Every row keeps the packet ref so the support pipeline can
        // re-join to alpha evidence without re-running graph producers.
        assert!(!row.envelope_packet_ref.trim().is_empty());
        // Open-gap classes never empty — aligned rows carry the
        // sentinel `none`.
        assert!(!row.open_gap_classes.is_empty());
    }
}

#[test]
fn support_export_envelope_round_trips_through_serde() {
    let envelope =
        compile_support_export_envelope("envelope:graph_drift:test", "2026-05-16T10:00:00Z")
            .expect("envelope compiles");
    let json = serde_json::to_string(&envelope).expect("envelope serializes");
    let parsed: GraphDriftSupportExportEnvelope =
        serde_json::from_str(&json).expect("envelope round-trips");
    assert_eq!(parsed, envelope);
}

#[test]
fn support_export_envelope_parity_with_graph_report() {
    // The support-export envelope is folded from the same drift
    // corpus the graph report reads. Folded rows must mirror the
    // matrix-row truth (readiness, freshness, scope, lineage, drift,
    // downgrade) so support and chrome agree on graph truth.
    let envelope =
        compile_support_export_envelope("envelope:graph_drift:parity", "2026-05-16T10:00:00Z")
            .expect("envelope compiles");
    let corpus = current_graph_drift_corpus().expect("corpus loads");
    let report = GraphDriftPacketEvaluator::new()
        .report("report:graph_drift:parity", "2026-05-16T10:00:00Z", &corpus)
        .expect("report builds");

    assert_eq!(envelope.rows.len(), report.matrix_rows.len());
    for matrix_row in &report.matrix_rows {
        let envelope_row = envelope
            .rows
            .iter()
            .find(|row| row.packet_id == matrix_row.packet_id)
            .expect("matrix row has a folded envelope row");
        assert_eq!(envelope_row.consumer_surface, matrix_row.consumer_surface);
        assert_eq!(envelope_row.readiness_state, matrix_row.readiness_state);
        assert_eq!(envelope_row.freshness_class, matrix_row.freshness_class);
        assert_eq!(envelope_row.scope_class, matrix_row.scope_class);
        assert_eq!(envelope_row.data_lane_lineage, matrix_row.data_lane_lineage);
        assert_eq!(envelope_row.drift_indicator, matrix_row.drift_indicator);
        assert_eq!(envelope_row.downgrade_label, matrix_row.downgrade_label);
        assert_eq!(envelope_row.open_gap_classes, matrix_row.open_gap_classes);
    }
}
