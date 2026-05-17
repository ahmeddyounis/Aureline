use std::collections::BTreeSet;

use aureline_history::local_history::timeline::{
    current_local_history_timeline_corpus, current_local_history_timeline_fixture_refs,
    LocalHistoryTimelineActionAvailability, LocalHistoryTimelineActionClass,
    LocalHistoryTimelineEvaluator, LocalHistoryTimelineFidelityLabel,
};

#[test]
fn protected_timeline_corpus_validates_and_covers_required_fidelity_labels() {
    let corpus = current_local_history_timeline_corpus().expect("checked-in corpus parses");
    let evaluator = LocalHistoryTimelineEvaluator::new();
    evaluator
        .validate_corpus(&corpus)
        .expect("timeline corpus validates");

    let labels = corpus
        .entries
        .iter()
        .map(|entry| entry.case.timeline_row.fidelity_label)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        labels,
        [
            LocalHistoryTimelineFidelityLabel::Exact,
            LocalHistoryTimelineFidelityLabel::Compatible,
            LocalHistoryTimelineFidelityLabel::LayoutOnly,
            LocalHistoryTimelineFidelityLabel::EvidenceOnly,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
    );

    let fixture_refs = current_local_history_timeline_fixture_refs().collect::<Vec<_>>();
    assert_eq!(fixture_refs.len(), corpus.entries.len());
}

#[test]
fn packet_keeps_compare_restore_and_export_labels_in_lockstep() {
    let corpus = current_local_history_timeline_corpus().expect("checked-in corpus parses");
    let packet = LocalHistoryTimelineEvaluator::new()
        .packet(
            "packet:local-history-timeline:test",
            "2026-05-17T07:00:00Z",
            aureline_history::LocalHistoryTimelineConsumerSurface::SupportExport,
            &corpus,
        )
        .expect("packet builds");

    packet.validate().expect("packet validates");
    for row in &packet.rows {
        let action_classes = row
            .actions
            .iter()
            .map(|action| action.action_class)
            .collect::<BTreeSet<_>>();
        assert_eq!(
            action_classes,
            [
                LocalHistoryTimelineActionClass::Compare,
                LocalHistoryTimelineActionClass::Restore,
                LocalHistoryTimelineActionClass::Export,
            ]
            .into_iter()
            .collect::<BTreeSet<_>>()
        );
        assert!(row
            .actions
            .iter()
            .all(|action| action.fidelity_label == row.fidelity_label));
    }
}

#[test]
fn evidence_only_rows_disable_restore_and_never_claim_live_resumption() {
    let corpus = current_local_history_timeline_corpus().expect("checked-in corpus parses");
    let evidence = corpus
        .entries
        .iter()
        .find(|entry| {
            entry.case.timeline_row.fidelity_label
                == LocalHistoryTimelineFidelityLabel::EvidenceOnly
        })
        .expect("evidence-only fixture exists");
    let restore_action = evidence
        .case
        .timeline_row
        .actions
        .iter()
        .find(|action| action.action_class == LocalHistoryTimelineActionClass::Restore)
        .expect("restore action exists");

    assert_eq!(
        restore_action.availability_class,
        LocalHistoryTimelineActionAvailability::DisabledExportOnly
    );
    assert!(!restore_action.writes_new_checkpoint);
    assert!(
        !evidence
            .case
            .timeline_row
            .no_rerun_guard
            .live_session_resumed
    );
    assert!(
        !evidence
            .case
            .timeline_row
            .no_rerun_guard
            .privileged_run_resumed
    );
    assert!(
        evidence
            .case
            .timeline_row
            .no_rerun_guard
            .evidence_only_label_visible
    );
}

#[test]
fn validator_rejects_evidence_only_live_session_claims() {
    let corpus = current_local_history_timeline_corpus().expect("checked-in corpus parses");
    let mut case = corpus
        .entries
        .iter()
        .find(|entry| {
            entry.case.timeline_row.fidelity_label
                == LocalHistoryTimelineFidelityLabel::EvidenceOnly
        })
        .expect("evidence-only fixture exists")
        .case
        .clone();
    case.timeline_row.no_rerun_guard.live_session_resumed = true;

    let report = LocalHistoryTimelineEvaluator::new()
        .validate_case(&case)
        .expect_err("live evidence-only claim must fail");
    assert!(report
        .violations
        .iter()
        .any(|violation| violation.check_id == "row.no_rerun.evidence_resumed_live_state"));
}

#[test]
fn timeline_report_is_export_safe_and_round_trips() {
    let corpus = current_local_history_timeline_corpus().expect("checked-in corpus parses");
    let report = LocalHistoryTimelineEvaluator::new()
        .report(
            "report:local-history-timeline:test",
            "2026-05-17T07:00:00Z",
            &corpus,
        )
        .expect("report builds");
    assert!(report.is_export_safe());

    let json = serde_json::to_string(&report).expect("report serializes");
    let parsed: aureline_history::LocalHistoryTimelineReport =
        serde_json::from_str(&json).expect("report round-trips");
    assert_eq!(parsed, report);
}
