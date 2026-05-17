//! Support-side parity drill for the generated-artifact lineage beta
//! projection.
//!
//! Compiles the metadata-safe support-export envelope from the
//! checked-in lineage corpus and asserts that it mirrors the report
//! matrix row-for-row, so navigation, AI context, review, and support
//! export surfaces stay aligned on one controlled lineage vocabulary.

use aureline_reactive_state::generated_lineage::{
    current_generated_artifact_lineage_corpus, GeneratedArtifactLineageEvaluator,
};
use aureline_support::generated_lineage::{
    compile_support_export_envelope, GeneratedLineageSupportExportEnvelope,
    GENERATED_LINEAGE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND,
    GENERATED_LINEAGE_SUPPORT_EXPORT_ROW_RECORD_KIND,
};

#[test]
fn envelope_mirrors_report_matrix() {
    let envelope =
        compile_support_export_envelope("envelope:generated_lineage:test", "2026-05-16T10:00:00Z")
            .expect("envelope compiles");
    assert_eq!(
        envelope.record_kind,
        GENERATED_LINEAGE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND
    );
    assert!(envelope.is_export_safe());

    let corpus = current_generated_artifact_lineage_corpus().expect("corpus loads");
    let report = GeneratedArtifactLineageEvaluator::new()
        .report(
            "envelope:generated_lineage:test:report",
            "2026-05-16T10:00:00Z",
            &corpus,
        )
        .expect("report builds");

    assert_eq!(envelope.rows.len(), report.matrix_rows.len());

    for (row, matrix) in envelope.rows.iter().zip(report.matrix_rows.iter()) {
        assert_eq!(
            row.record_kind,
            GENERATED_LINEAGE_SUPPORT_EXPORT_ROW_RECORD_KIND
        );
        assert_eq!(row.packet_id, matrix.packet_id);
        assert_eq!(row.consumer_surface, matrix.consumer_surface);
        assert_eq!(row.artifact_family, matrix.artifact_family);
        assert_eq!(row.artifact_ref, matrix.artifact_ref);
        assert_eq!(row.lineage_class, matrix.lineage_class);
        assert_eq!(row.drift_state, matrix.drift_state);
        assert_eq!(row.default_edit_posture, matrix.default_edit_posture);
        assert_eq!(row.downgrade_label, matrix.downgrade_label);
        assert_eq!(row.open_gap_classes, matrix.open_gap_classes);
    }
}

#[test]
fn envelope_round_trips_through_json() {
    let envelope =
        compile_support_export_envelope("envelope:generated_lineage:test", "2026-05-16T10:00:00Z")
            .expect("envelope compiles");
    let json = serde_json::to_string(&envelope).expect("envelope serializes");
    let parsed: GeneratedLineageSupportExportEnvelope =
        serde_json::from_str(&json).expect("envelope round-trips");
    assert_eq!(parsed, envelope);
}

#[test]
fn envelope_preserves_generator_identity_and_source_refs() {
    let envelope =
        compile_support_export_envelope("envelope:generated_lineage:test", "2026-05-16T10:00:00Z")
            .expect("envelope compiles");
    for row in &envelope.rows {
        assert!(
            !row.generator_identity.generator_ref.trim().is_empty(),
            "row {} must preserve generator_ref",
            row.packet_id
        );
        assert!(
            !row.source_refs.is_empty(),
            "row {} must preserve at least one source_ref",
            row.packet_id
        );
    }
}
