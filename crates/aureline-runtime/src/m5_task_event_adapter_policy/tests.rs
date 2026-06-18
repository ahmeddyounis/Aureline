//! Unit coverage for the M5 task-event adapter-policy baseline.

use super::*;

fn stable() -> TaskEventAdapterPolicyBaseline {
    seeded_task_event_adapter_policy_baseline()
}

#[test]
fn seed_baseline_validates_clean_and_is_stable() {
    let baseline = stable();
    assert_eq!(baseline.record_kind, TASK_EVENT_ADAPTER_POLICY_RECORD_KIND);
    assert_eq!(
        baseline.schema_version,
        TASK_EVENT_ADAPTER_POLICY_SCHEMA_VERSION
    );
    assert!(
        baseline.validate().is_empty(),
        "seed baseline must validate clean: {:?}",
        baseline.validate()
    );
    assert!(baseline.is_stable());
    assert_eq!(baseline.promotion_state.as_str(), "stable");
}

#[test]
fn priority_ladder_is_the_canonical_native_first_order() {
    let baseline = stable();
    let ranks: Vec<(&str, u8)> = baseline
        .priority_ladder
        .iter()
        .map(|rung| (rung.source_kind.as_str(), rung.priority_rank))
        .collect();
    assert_eq!(
        ranks,
        vec![
            ("native", 1),
            ("bsp", 2),
            ("bazel-bep", 3),
            ("structured-output", 4),
            ("heuristic-parser", 5),
        ]
    );
    for rung in &baseline.priority_ladder {
        assert_eq!(
            rung.authoritative,
            source_is_authoritative(rung.source_kind)
        );
        assert_eq!(rung.masquerade_blocked, !rung.authoritative);
    }
}

#[test]
fn downgrade_vocabulary_is_the_closed_four_set() {
    let baseline = stable();
    // Tokens are returned in enum declaration order.
    assert_eq!(
        baseline.downgrade_reason_tokens(),
        vec![
            "partial_support",
            "heuristic_fallback",
            "replay_gap",
            "unsupported_adapter_capability",
        ]
    );
}

#[test]
fn retention_matrix_covers_every_source_and_class() {
    let baseline = stable();
    assert_eq!(baseline.retention_matrix.len(), 5 * 3);
    for source_kind in BuildTestEventSourceKind::ALL {
        let defaults = baseline
            .retention_matrix
            .iter()
            .filter(|cell| cell.source_kind == source_kind && cell.is_default)
            .count();
        assert_eq!(defaults, 1, "exactly one default per source");
    }
}

#[test]
fn all_six_consumers_are_bound() {
    let baseline = stable();
    // Tokens are returned in enum declaration order.
    assert_eq!(
        baseline.consumer_tokens(),
        vec![
            "pipeline",
            "coverage",
            "snapshot_flaky",
            "notebook_run",
            "cli_headless",
            "support_export",
        ]
    );
}

#[test]
fn out_of_order_ladder_blocks_stable() {
    let mut input = current_stable_task_event_adapter_policy_input();
    // Swap native and heuristic ranks: native pretends to be lowest priority.
    for rung in &mut input.priority_ladder {
        if rung.source_kind == BuildTestEventSourceKind::Native {
            rung.priority_rank = 5;
        } else if rung.source_kind == BuildTestEventSourceKind::HeuristicParser {
            rung.priority_rank = 1;
        }
    }
    let baseline = TaskEventAdapterPolicyBaseline::materialize(input);
    assert_eq!(baseline.promotion_state.as_str(), "blocks_stable");
    assert!(baseline
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PolicyFindingKind::PriorityRankMismatch));
}

#[test]
fn heuristic_ceiling_overclaim_blocks_stable() {
    let mut input = current_stable_task_event_adapter_policy_input();
    for rung in &mut input.priority_ladder {
        if rung.source_kind == BuildTestEventSourceKind::HeuristicParser {
            rung.confidence_ceiling = BuildTestEventConfidence::High;
        }
    }
    let baseline = TaskEventAdapterPolicyBaseline::materialize(input);
    assert!(baseline
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PolicyFindingKind::ConfidenceCeilingMismatch));
    assert!(!baseline.is_stable());
}

#[test]
fn missing_consumer_binding_blocks_stable() {
    let mut input = current_stable_task_event_adapter_policy_input();
    input
        .consumer_bindings
        .retain(|b| b.consumer != TaskEventConsumer::NotebookRun);
    let baseline = TaskEventAdapterPolicyBaseline::materialize(input);
    assert!(baseline
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PolicyFindingKind::ConsumerBindingMissing));
}

#[test]
fn shadow_that_is_not_downgraded_blocks_stable() {
    let mut input = current_stable_task_event_adapter_policy_input();
    let row = input
        .arbitration_rows
        .iter_mut()
        .find(|r| r.arbitration_id == "arbitration:native-over-heuristic")
        .expect("seed has the native-over-heuristic arbitration");
    let shadow = &mut row.shadow_events[0];
    shadow.downgraded = false;
    shadow.downgrade_reason = None;
    let baseline = TaskEventAdapterPolicyBaseline::materialize(input);
    assert!(baseline
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PolicyFindingKind::ArbitrationShadowNotDowngraded));
}

#[test]
fn lower_priority_winner_blocks_stable() {
    let mut input = current_stable_task_event_adapter_policy_input();
    // Swap so the structured-output emission claims to win over BSP truth.
    let row = input
        .arbitration_rows
        .iter_mut()
        .find(|r| r.arbitration_id == "arbitration:bsp-over-structured")
        .expect("seed has the bsp-over-structured arbitration");
    std::mem::swap(&mut row.winning_event, &mut row.shadow_events[0]);
    let baseline = TaskEventAdapterPolicyBaseline::materialize(input);
    assert!(!baseline.is_stable());
    assert!(baseline.validation_findings.iter().any(|f| f.finding_kind
        == PolicyFindingKind::ArbitrationWinnerNotHighestPriority
        || f.finding_kind == PolicyFindingKind::ArbitrationShadowNotDowngraded));
}

#[test]
fn support_export_round_trips_and_is_safe() {
    let baseline = stable();
    let export = baseline.support_export(
        TASK_EVENT_ADAPTER_POLICY_SUPPORT_EXPORT_ID,
        "2026-06-17T00:01:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.baseline_id_ref, baseline.baseline_id);
    let json = serde_json::to_string(&export).expect("serialize export");
    let round: TaskEventAdapterPolicySupportExport =
        serde_json::from_str(&json).expect("deserialize export");
    assert_eq!(round, export);
}

#[test]
fn finding_tokens_are_pinned() {
    assert_eq!(
        PolicyFindingKind::ArbitrationWinnerNotHighestPriority.as_str(),
        "arbitration_winner_not_highest_priority"
    );
    assert_eq!(
        PolicyFindingKind::RetentionDefaultInvalid.as_str(),
        "retention_default_invalid"
    );
    assert_eq!(
        PolicyFindingKind::DowngradeVocabularyDrift.as_str(),
        "downgrade_vocabulary_drift"
    );
}
