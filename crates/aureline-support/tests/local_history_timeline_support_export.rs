//! Support-side parity checks for local-history timeline fidelity labels.

use std::collections::BTreeSet;

use aureline_history::local_history::timeline::{
    current_local_history_timeline_corpus, LocalHistoryTimelineEvaluator,
    LocalHistoryTimelineFidelityLabel,
};
use aureline_support::local_history_timeline::{
    compile_support_export_envelope, LocalHistoryTimelineSupportExportEnvelope,
    LOCAL_HISTORY_TIMELINE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND,
    LOCAL_HISTORY_TIMELINE_SUPPORT_EXPORT_ROW_RECORD_KIND,
};

#[test]
fn support_envelope_quotes_the_same_fidelity_labels_as_timeline_report() {
    let envelope = compile_support_export_envelope(
        "envelope:local-history-timeline:test",
        "2026-05-17T07:05:00Z",
    )
    .expect("support envelope compiles");
    assert_eq!(
        envelope.record_kind,
        LOCAL_HISTORY_TIMELINE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND
    );
    assert!(envelope.is_export_safe());

    let corpus = current_local_history_timeline_corpus().expect("corpus parses");
    let report = LocalHistoryTimelineEvaluator::new()
        .report(
            "report:local-history-timeline:test",
            "2026-05-17T07:05:00Z",
            &corpus,
        )
        .expect("report builds");

    let support_labels = envelope
        .rows
        .iter()
        .map(|row| row.fidelity_label)
        .collect::<BTreeSet<_>>();
    let report_labels = report
        .matrix_rows
        .iter()
        .map(|row| row.fidelity_label)
        .collect::<BTreeSet<_>>();
    assert_eq!(support_labels, report_labels);
    assert_eq!(
        support_labels,
        [
            LocalHistoryTimelineFidelityLabel::Exact,
            LocalHistoryTimelineFidelityLabel::Compatible,
            LocalHistoryTimelineFidelityLabel::LayoutOnly,
            LocalHistoryTimelineFidelityLabel::EvidenceOnly,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
    );
}

#[test]
fn support_rows_keep_compare_restore_and_export_action_labels_aligned() {
    let envelope = compile_support_export_envelope(
        "envelope:local-history-timeline:test",
        "2026-05-17T07:05:00Z",
    )
    .expect("support envelope compiles");

    for row in &envelope.rows {
        assert_eq!(
            row.record_kind,
            LOCAL_HISTORY_TIMELINE_SUPPORT_EXPORT_ROW_RECORD_KIND
        );
        assert_eq!(row.compare_fidelity_label, row.fidelity_label);
        assert_eq!(row.restore_fidelity_label, row.fidelity_label);
        assert_eq!(row.export_fidelity_label, row.fidelity_label);
        assert!(!row.live_session_resumed);
        assert!(!row.privileged_run_resumed);
    }
}

#[test]
fn support_envelope_round_trips_through_json() {
    let envelope = compile_support_export_envelope(
        "envelope:local-history-timeline:test",
        "2026-05-17T07:05:00Z",
    )
    .expect("support envelope compiles");
    let json = serde_json::to_string(&envelope).expect("envelope serializes");
    let parsed: LocalHistoryTimelineSupportExportEnvelope =
        serde_json::from_str(&json).expect("envelope round-trips");
    assert_eq!(parsed, envelope);
}
