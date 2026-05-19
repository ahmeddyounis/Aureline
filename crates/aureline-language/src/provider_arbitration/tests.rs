use super::records::{CompletenessClass, HealthState};
use super::*;

#[test]
fn current_corpus_loads_and_validates() {
    let corpus = current_provider_arbitration_corpus().expect("corpus parses");
    assert_eq!(corpus.entries.len(), 6);
    let report = ArbitrationInspector::new()
        .report(
            "language:provider-arbitration:report:unit",
            "2026-05-19T10:00:00Z",
            &corpus,
        )
        .expect("corpus validates");
    assert!(report.is_export_safe());
    assert!(report.aggregate_counts.conflict_rows >= 1);
    assert!(report.aggregate_counts.quarantined_provider_rows >= 1);
}

#[test]
fn missing_lane_is_flagged() {
    let mut corpus = current_provider_arbitration_corpus().expect("corpus parses");
    corpus.entries.retain(|entry| {
        entry.arbitration_decision.language_action_lane_class != LanguageActionLaneClass::Formatting
    });
    let report = ArbitrationInspector::new().validate(&corpus);
    assert!(report
        .defects
        .iter()
        .any(|defect| defect.check_id == "corpus.lane_missing"));
}

#[test]
fn exact_outcome_rejects_fallback_label() {
    let mut corpus = current_provider_arbitration_corpus().expect("corpus parses");
    let entry = corpus
        .entries
        .iter_mut()
        .find(|entry| {
            entry.arbitration_decision.confidence_outcome_class == ConfidenceOutcomeClass::Exact
        })
        .expect("at least one exact row");
    entry.arbitration_decision.fallback_label_class = FallbackLabelClass::TextFallback;
    let report = ArbitrationInspector::new().validate(&corpus);
    assert!(report
        .defects
        .iter()
        .any(|defect| defect.check_id == "decision.exact_must_not_label_fallback"));
}

#[test]
fn conflict_blocks_ready_apply() {
    let mut corpus = current_provider_arbitration_corpus().expect("corpus parses");
    let entry = corpus
        .entries
        .iter_mut()
        .find(|entry| {
            entry.arbitration_decision.disagreement_block.conflict_class != ConflictClass::None
        })
        .expect("at least one conflict row");
    entry.arbitration_decision.apply_gate_class = ApplyGateClass::ReadyToApply;
    let report = ArbitrationInspector::new().validate(&corpus);
    assert!(report
        .defects
        .iter()
        .any(|defect| defect.check_id == "decision.conflict_blocks_ready_apply"));
}

#[test]
fn quarantine_row_must_disclose_actions() {
    let mut corpus = current_provider_arbitration_corpus().expect("corpus parses");
    let entry = corpus
        .entries
        .iter_mut()
        .find(|entry| {
            entry
                .provider_health_states
                .iter()
                .any(|row| row.health_state == HealthState::CrashLoopQuarantined)
        })
        .expect("at least one quarantined provider");
    let row = entry
        .provider_health_states
        .iter_mut()
        .find(|row| row.health_state == HealthState::CrashLoopQuarantined)
        .expect("quarantined row");
    row.retry_isolate_controls.retry_action_class = RetryActionClass::NotAvailable;
    row.retry_isolate_controls.isolate_action_class = IsolateActionClass::NotAvailable;
    let report = ArbitrationInspector::new().validate(&corpus);
    assert!(report
        .defects
        .iter()
        .any(|defect| defect.check_id == "provider_health_state.quarantine_actions_missing"));
}

#[test]
fn wide_scope_rename_partial_must_route_through_review() {
    let mut corpus = current_provider_arbitration_corpus().expect("corpus parses");
    let entry = corpus
        .entries
        .iter_mut()
        .find(|entry| {
            entry.arbitration_decision.language_action_lane_class == LanguageActionLaneClass::Rename
                && entry.arbitration_decision.negotiated_completeness_class
                    == CompletenessClass::PartialForClaimedScope
        })
        .expect("at least one partial rename row");
    entry.arbitration_decision.apply_gate_class = ApplyGateClass::ReadyToApply;
    let report = ArbitrationInspector::new().validate(&corpus);
    let has_specific_check = report.defects.iter().any(|defect| {
        defect.check_id == "decision.wide_scope_rename_requires_preview_gate"
            || defect.check_id == "decision.conflict_blocks_ready_apply"
            || defect.check_id == "decision.partial_requires_partial_completeness"
    });
    assert!(has_specific_check);
}

#[test]
fn fixture_refs_round_trip() {
    let refs: Vec<&str> = current_provider_arbitration_fixture_refs().collect();
    assert_eq!(refs.len(), 6);
    for fixture_ref in refs {
        assert!(fixture_ref.starts_with(PROVIDER_ARBITRATION_CORPUS_DIR));
    }
}
