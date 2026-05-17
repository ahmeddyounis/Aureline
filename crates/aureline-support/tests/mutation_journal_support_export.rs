//! Support-side parity drill for the mutation-journal beta projection.
//!
//! Compiles the metadata-safe support-export envelope from the
//! checked-in mutation-journal corpus and asserts that it mirrors the
//! report matrix row-for-row, so incident, support-bundle, doctor,
//! crash, and recovery-ladder surfaces stay aligned on one controlled
//! grouped-write vocabulary.

use aureline_reactive_state::mutation_journal::{
    current_mutation_journal_corpus, MutationJournalEvaluator,
};
use aureline_support::mutation_journal::{
    compile_support_export_envelope, MutationJournalSupportExportEnvelope,
    MUTATION_JOURNAL_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND,
    MUTATION_JOURNAL_SUPPORT_EXPORT_ROW_RECORD_KIND,
};

#[test]
fn envelope_mirrors_report_matrix() {
    let envelope =
        compile_support_export_envelope("envelope:mutation_journal:test", "2026-05-16T10:30:00Z")
            .expect("envelope compiles");
    assert_eq!(
        envelope.record_kind,
        MUTATION_JOURNAL_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND
    );
    assert!(envelope.is_export_safe());

    let corpus = current_mutation_journal_corpus().expect("corpus loads");
    let report = MutationJournalEvaluator::new()
        .report(
            "envelope:mutation_journal:test:report",
            "2026-05-16T10:30:00Z",
            &corpus,
        )
        .expect("report builds");

    assert_eq!(envelope.rows.len(), report.matrix_rows.len());

    for (row, matrix) in envelope.rows.iter().zip(report.matrix_rows.iter()) {
        assert_eq!(
            row.record_kind,
            MUTATION_JOURNAL_SUPPORT_EXPORT_ROW_RECORD_KIND
        );
        assert_eq!(row.entry_id, matrix.entry_id);
        assert_eq!(row.consumer_surface, matrix.consumer_surface);
        assert_eq!(row.source_lane, matrix.source_lane);
        assert_eq!(row.actor_class, matrix.actor_class);
        assert_eq!(row.authority_class, matrix.authority_class);
        assert_eq!(row.entry_kind, matrix.entry_kind);
        assert_eq!(row.group_size, matrix.group_size);
        assert_eq!(row.recovery_class, matrix.recovery_class);
        assert_eq!(row.attribution_state, matrix.attribution_state);
        assert_eq!(row.replayability_state, matrix.replayability_state);
        assert_eq!(row.downgrade_label, matrix.downgrade_label);
        assert_eq!(row.open_gap_classes, matrix.open_gap_classes);
        assert_eq!(row.affected_paths.len() as u32, matrix.affected_path_count);
    }
}

#[test]
fn envelope_round_trips_through_json() {
    let envelope =
        compile_support_export_envelope("envelope:mutation_journal:test", "2026-05-16T10:30:00Z")
            .expect("envelope compiles");
    let json = serde_json::to_string(&envelope).expect("envelope serializes");
    let parsed: MutationJournalSupportExportEnvelope =
        serde_json::from_str(&json).expect("envelope round-trips");
    assert_eq!(parsed, envelope);
}

#[test]
fn envelope_preserves_affected_paths_and_safety_baseline() {
    let envelope =
        compile_support_export_envelope("envelope:mutation_journal:test", "2026-05-16T10:30:00Z")
            .expect("envelope compiles");
    assert!(envelope.raw_payload_excluded);
    assert!(envelope.raw_private_material_excluded);
    assert!(envelope.ambient_authority_excluded);
    for row in &envelope.rows {
        assert!(
            !row.affected_paths.is_empty(),
            "row {} must preserve at least one affected_path",
            row.entry_id
        );
        for path in &row.affected_paths {
            assert!(
                !path.trim().is_empty(),
                "row {} affected_path entries must be non-empty",
                row.entry_id
            );
        }
        assert!(
            !row.open_gap_classes.is_empty(),
            "row {} must declare at least one open_gap class",
            row.entry_id
        );
    }
}
