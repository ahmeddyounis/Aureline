use std::collections::BTreeSet;

use aureline_language::refactor_preview::RefactorCorpusRowState;
use aureline_support::refactor_preview::{
    compile_support_export_envelope, RefactorPreviewSupportExportEnvelope,
};

#[test]
fn support_export_compiles_from_language_corpus() {
    let envelope = compile_support_export_envelope(
        "envelope:language-refactor-preview:test",
        "2026-05-18T10:15:00Z",
    )
    .expect("support export compiles");

    assert!(envelope.is_export_safe());
    assert_eq!(envelope.rows.len(), envelope.report.rows.len());
    assert!(envelope.raw_payload_excluded);
    assert!(envelope.raw_private_material_excluded);

    let states = envelope
        .rows
        .iter()
        .map(|row| row.corpus_row_state)
        .collect::<BTreeSet<_>>();
    assert!(states.contains(&RefactorCorpusRowState::Green));
    assert!(states.contains(&RefactorCorpusRowState::Downgraded));
    assert!(states.contains(&RefactorCorpusRowState::Unsupported));
}

#[test]
fn support_export_preserves_fallback_and_grouped_rollback_refs() {
    let envelope = compile_support_export_envelope(
        "envelope:language-refactor-preview:test",
        "2026-05-18T10:20:00Z",
    )
    .expect("support export compiles");

    assert!(envelope.rows.iter().any(|row| {
        row.fallback_label.contains("Text-only fallback")
            && row.corpus_row_state == RefactorCorpusRowState::Unsupported
            && row.rollback_handle_ref.is_none()
    }));
    assert!(envelope.rows.iter().any(|row| {
        row.rollback_handle_ref.is_some()
            && row.local_history_group_ref.is_some()
            && row.mutation_journal_ref.is_some()
    }));
}

#[test]
fn support_export_round_trips_through_json() {
    let envelope = compile_support_export_envelope(
        "envelope:language-refactor-preview:test",
        "2026-05-18T10:25:00Z",
    )
    .expect("support export compiles");

    let json = serde_json::to_string(&envelope).expect("support export serializes");
    let parsed: RefactorPreviewSupportExportEnvelope =
        serde_json::from_str(&json).expect("support export deserializes");
    assert_eq!(parsed, envelope);
}
