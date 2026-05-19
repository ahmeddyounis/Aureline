use std::collections::BTreeSet;

use aureline_language::provider_arbitration::{
    current_provider_arbitration_corpus, current_provider_arbitration_fixture_refs, ApplyGateClass,
    ArbitrationCompletenessClass, ArbitrationHealthState, ArbitrationInspector,
    ConfidenceOutcomeClass, ConflictClass, ConsumerSurfaceClass, DisagreementVisibilityClass,
    DowngradedPromiseReasonClass, FallbackLabelClass, IsolateActionClass, LanguageActionLaneClass,
    RetryActionClass, PROVIDER_ARBITRATION_CORPUS_DIR,
};

#[test]
fn corpus_covers_required_lanes_and_outcomes() {
    let corpus = current_provider_arbitration_corpus().expect("corpus parses");
    let report = ArbitrationInspector::new()
        .report(
            "language:provider-arbitration:report:beta",
            "2026-05-19T18:00:00Z",
            &corpus,
        )
        .expect("corpus validates");

    assert!(report.is_export_safe());
    assert_eq!(
        report.aggregate_counts.total_rows,
        corpus.entries.len() as u32
    );
    assert!(report.aggregate_counts.exact_rows >= 1);
    assert!(report.aggregate_counts.heuristic_rows >= 1);
    assert!(report.aggregate_counts.partial_rows >= 1);
    assert!(report.aggregate_counts.stale_rows >= 1);
    assert!(report.aggregate_counts.unavailable_rows >= 1);
    assert!(report.aggregate_counts.conflict_rows >= 1);
    assert!(report.aggregate_counts.quarantined_provider_rows >= 1);
    assert!(report.aggregate_counts.preview_gated_rows >= 1);
    assert!(report.aggregate_counts.side_branch_gated_rows >= 1);

    let lanes = corpus
        .entries
        .iter()
        .map(|entry| entry.arbitration_decision.language_action_lane_class)
        .collect::<BTreeSet<_>>();
    for required in [
        LanguageActionLaneClass::Definition,
        LanguageActionLaneClass::References,
        LanguageActionLaneClass::Rename,
        LanguageActionLaneClass::Formatting,
        LanguageActionLaneClass::OrganizeImports,
        LanguageActionLaneClass::CodeAction,
    ] {
        assert!(lanes.contains(&required), "missing lane {required:?}");
    }
}

#[test]
fn non_exact_outcomes_carry_fallback_labels() {
    let corpus = current_provider_arbitration_corpus().expect("corpus parses");
    for entry in corpus.entries {
        let decision = &entry.arbitration_decision;
        match decision.confidence_outcome_class {
            ConfidenceOutcomeClass::Exact => {
                assert_eq!(decision.fallback_label_class, FallbackLabelClass::None);
                assert_eq!(
                    decision
                        .downgraded_promise_block
                        .downgraded_promise_reason_class,
                    DowngradedPromiseReasonClass::None
                );
            }
            ConfidenceOutcomeClass::Heuristic
            | ConfidenceOutcomeClass::Partial
            | ConfidenceOutcomeClass::Stale => {
                assert_ne!(
                    decision.fallback_label_class,
                    FallbackLabelClass::None,
                    "{} non-exact outcomes must carry a fallback label",
                    decision.arbitration_decision_id
                );
            }
            ConfidenceOutcomeClass::Unavailable => {
                assert!(decision.chosen_provider_id.is_none());
                assert!(matches!(
                    decision.apply_gate_class,
                    ApplyGateClass::BlockedForHealth
                        | ApplyGateClass::BlockedForPartialScope
                        | ApplyGateClass::BlockedForDisagreement
                        | ApplyGateClass::InspectOnly
                ));
            }
        }
    }
}

#[test]
fn conflict_rows_route_through_visible_disagreement_panels() {
    let corpus = current_provider_arbitration_corpus().expect("corpus parses");
    let conflict_rows = corpus
        .entries
        .iter()
        .filter(|entry| {
            entry.arbitration_decision.disagreement_block.conflict_class != ConflictClass::None
        })
        .collect::<Vec<_>>();
    assert!(!conflict_rows.is_empty());
    for entry in conflict_rows {
        let block = &entry.arbitration_decision.disagreement_block;
        assert_ne!(
            block.disagreement_visibility_class,
            DisagreementVisibilityClass::None,
            "{} must surface disagreement through a visible panel",
            entry.arbitration_decision.arbitration_decision_id
        );
        assert_ne!(
            entry.arbitration_decision.apply_gate_class,
            ApplyGateClass::ReadyToApply,
            "{} must not allow ready-to-apply with a conflict",
            entry.arbitration_decision.arbitration_decision_id
        );
    }
}

#[test]
fn wide_scope_rename_with_partial_truth_requires_preview_or_side_branch() {
    let corpus = current_provider_arbitration_corpus().expect("corpus parses");
    let rename_rows = corpus
        .entries
        .iter()
        .filter(|entry| {
            entry.arbitration_decision.language_action_lane_class == LanguageActionLaneClass::Rename
        })
        .collect::<Vec<_>>();
    assert!(!rename_rows.is_empty());
    for entry in rename_rows {
        let decision = &entry.arbitration_decision;
        if decision.negotiated_completeness_class
            == ArbitrationCompletenessClass::PartialForClaimedScope
        {
            assert!(
                matches!(
                    decision.apply_gate_class,
                    ApplyGateClass::PreviewRequired
                        | ApplyGateClass::SideBranchRequired
                        | ApplyGateClass::BlockedForPartialScope
                        | ApplyGateClass::InspectOnly
                ),
                "{} must gate apply when rename is partial",
                decision.arbitration_decision_id
            );
        }
    }
}

#[test]
fn quarantined_provider_remains_inspectable_with_retry_or_isolate() {
    let corpus = current_provider_arbitration_corpus().expect("corpus parses");
    let mut saw_quarantine = false;
    for entry in &corpus.entries {
        for health in &entry.provider_health_states {
            if health.health_state == ArbitrationHealthState::CrashLoopQuarantined {
                saw_quarantine = true;
                assert!(
                    health.quarantine_ref.is_some(),
                    "quarantined provider {} must carry quarantine ref",
                    health.provider_id
                );
                assert_eq!(
                    health
                        .downgraded_promise_block
                        .downgraded_promise_reason_class,
                    DowngradedPromiseReasonClass::CrashLoopExcluded
                );
                assert!(
                    health.retry_isolate_controls.retry_action_class
                        != RetryActionClass::NotAvailable
                        || health.retry_isolate_controls.isolate_action_class
                            != IsolateActionClass::NotAvailable,
                    "quarantined provider {} must expose at least one retry or isolate action",
                    health.provider_id
                );
            }
        }
    }
    assert!(
        saw_quarantine,
        "corpus must include a quarantined provider row"
    );
}

#[test]
fn every_decision_routes_to_required_consumers() {
    let corpus = current_provider_arbitration_corpus().expect("corpus parses");
    for entry in &corpus.entries {
        let routed: BTreeSet<ConsumerSurfaceClass> = entry
            .arbitration_decision
            .consumer_routing_rows
            .iter()
            .map(|row| row.consumer_surface_class)
            .collect();
        for required in [
            ConsumerSurfaceClass::EditorChrome,
            ConsumerSurfaceClass::CommandResult,
            ConsumerSurfaceClass::CliHeadlessInspect,
            ConsumerSurfaceClass::SupportExport,
        ] {
            assert!(
                routed.contains(&required),
                "{} must route to {required:?}",
                entry.arbitration_decision.arbitration_decision_id
            );
        }
    }
}

#[test]
fn fixture_refs_match_corpus_directory() {
    let refs: Vec<&str> = current_provider_arbitration_fixture_refs().collect();
    assert_eq!(refs.len(), 6);
    for fixture_ref in refs {
        assert!(
            fixture_ref.starts_with(PROVIDER_ARBITRATION_CORPUS_DIR),
            "fixture ref {fixture_ref} must live in the corpus directory"
        );
    }
}
