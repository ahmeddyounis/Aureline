//! Unit tests for the M5 evaluate/REPL sheet set.

use super::*;

#[test]
fn canonical_set_validates_and_is_export_safe() {
    let set = m5_evaluate_repl_sheet_set();
    set.validate().expect("canonical set validates");
    assert!(set.is_support_export_safe());
    assert!(set.all_invariants_hold());
    assert!(set.raw_payload_excluded);
}

#[test]
fn canonical_set_round_trips_through_serde() {
    let set = m5_evaluate_repl_sheet_set();
    let json = serde_json::to_string(&set).expect("serializes");
    let back: EvaluateReplSheetSet = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(set, back);
}

#[test]
fn every_purity_disposition_and_direction_is_materialized() {
    let set = m5_evaluate_repl_sheet_set();
    for purity in EvaluatePurityClass::ALL {
        assert!(
            set.evaluation_in_purity(purity).is_some(),
            "missing evaluation for purity {}",
            purity.as_str()
        );
    }
    for disposition in ApprovalDisposition::ALL {
        assert!(
            set.evaluation_in_disposition(disposition).is_some(),
            "missing evaluation for disposition {}",
            disposition.as_str()
        );
    }
    for direction in ConsoleDirection::ALL {
        assert!(
            set.emission_in_direction(direction).is_some(),
            "missing emission for direction {}",
            direction.as_str()
        );
    }
}

#[test]
fn purity_is_classified_before_dispatch_and_discloses_risk() {
    let set = m5_evaluate_repl_sheet_set();
    for ev in &set.evaluations {
        assert_eq!(
            ev.posture.discloses_side_effect_risk,
            ev.purity.discloses_side_effect_risk(),
            "evaluation {} risk disclosure must equal its purity",
            ev.evaluate_id
        );
        assert_eq!(ev.posture.approval_required, ev.purity.requires_approval());
        // A pure expression never claims a side-effect risk; an effectful one always does.
        assert_eq!(ev.posture.discloses_side_effect_risk, !ev.purity.is_pure());
    }
}

#[test]
fn unknown_or_mutating_evaluation_never_runs_unless_approved() {
    let set = m5_evaluate_repl_sheet_set();
    for ev in &set.evaluations {
        if ev.posture.approval_required && ev.disposition != ApprovalDisposition::Approved {
            assert!(
                !ev.posture.permits_dispatch,
                "evaluation {} requires approval and is not approved but permits dispatch",
                ev.evaluate_id
            );
            assert!(
                ev.result.is_none(),
                "evaluation {} is not approved but carries a result",
                ev.evaluate_id
            );
        }
    }
    // At least one effectful, approved evaluation actually runs.
    assert!(set
        .evaluations
        .iter()
        .any(|e| e.posture.approval_required && e.posture.permits_dispatch && e.result.is_some()));
}

#[test]
fn blocked_denied_and_expired_states_are_preserved() {
    let set = m5_evaluate_repl_sheet_set();
    for disposition in [
        ApprovalDisposition::Blocked,
        ApprovalDisposition::Denied,
        ApprovalDisposition::Expired,
    ] {
        let ev = set
            .evaluation_in_disposition(disposition)
            .unwrap_or_else(|| panic!("missing disposition {}", disposition.as_str()));
        assert!(!ev.posture.permits_dispatch);
        assert!(ev.result.is_none());
    }
}

#[test]
fn inspect_only_context_blocks_effectful_evaluation() {
    let set = m5_evaluate_repl_sheet_set();
    let blocked: Vec<_> = set
        .evaluations
        .iter()
        .filter(|e| !e.context.authority.allows_mutation() && e.purity.requires_approval())
        .collect();
    assert!(
        !blocked.is_empty(),
        "an inspect-only effectful evaluation must exist"
    );
    for ev in blocked {
        assert!(
            !ev.posture.permits_dispatch,
            "effectful evaluation {} against an inspect-only context must not permit dispatch",
            ev.evaluate_id
        );
        assert!(ev.posture.blocked_by_inspect_only);
    }
}

#[test]
fn approved_effectful_evaluations_name_a_reviewer() {
    let set = m5_evaluate_repl_sheet_set();
    for ev in &set.evaluations {
        if ev.purity.requires_approval() && ev.disposition == ApprovalDisposition::Approved {
            assert!(
                ev.actor.reviewed_by_ref.is_some(),
                "approved evaluation {} must name a reviewer",
                ev.evaluate_id
            );
        }
    }
}

#[test]
fn no_raw_expression_text_and_redacted_results_withhold_bodies() {
    let set = m5_evaluate_repl_sheet_set();
    for ev in &set.evaluations {
        assert!(!ev.expression_digest.is_empty());
        if let Some(result) = &ev.result {
            if result.is_redacted {
                assert!(result.result_repr_digest.is_none());
            }
            assert_eq!(
                result.result_body_present,
                result.outcome.carries_value() && !result.is_redacted
            );
            assert_eq!(
                result.result_repr_digest.is_some(),
                result.result_body_present
            );
        }
    }
    // A redacted result is materialized.
    assert!(set
        .evaluations
        .iter()
        .filter_map(|e| e.result.as_ref())
        .any(|r| r.is_redacted));
}

#[test]
fn console_separates_user_input_from_target_output() {
    let set = m5_evaluate_repl_sheet_set();
    assert!(set
        .console
        .iter()
        .any(|c| c.direction() == ConsoleDirection::UserInput));
    assert!(set
        .console
        .iter()
        .any(|c| c.direction() == ConsoleDirection::TargetOutput));
    for em in &set.console {
        assert_eq!(em.direction, em.stream.direction());
        assert_eq!(em.pill.is_user_input, em.direction.is_user_input());
        assert_eq!(em.pill.is_target_output, !em.direction.is_user_input());
    }
}

#[test]
fn replayed_console_lines_are_never_shown_as_live() {
    let set = m5_evaluate_repl_sheet_set();
    let replayed: Vec<_> = set.console.iter().filter(|c| c.pill.is_replayed).collect();
    assert!(!replayed.is_empty(), "a replayed emission must exist");
    for em in replayed {
        assert_eq!(em.liveness, ConsoleLiveness::ReplayedCapture);
        assert!(!em.pill.is_live);
        assert!(em.pill.requires_disclosure);
    }
    for em in &set.console {
        if em.liveness == ConsoleLiveness::Live {
            assert!(!em.pill.is_replayed);
        }
    }
}

#[test]
fn redacted_console_emissions_withhold_their_bodies() {
    let set = m5_evaluate_repl_sheet_set();
    let redacted: Vec<_> = set
        .console
        .iter()
        .filter(|c| c.redaction.is_redacted())
        .collect();
    assert!(!redacted.is_empty(), "a redacted emission must exist");
    for em in redacted {
        assert!(em.pill.is_redacted);
        assert!(!em.pill.body_present);
        assert!(em.body_digest.is_none());
    }
}

#[test]
fn linked_console_emissions_resolve_to_an_evaluation() {
    let set = m5_evaluate_repl_sheet_set();
    let mut linked = 0usize;
    for em in &set.console {
        if let Some(id) = &em.linked_evaluate_id {
            assert!(
                set.evaluation(id).is_some(),
                "emission {} links to missing evaluate {id}",
                em.emission_id
            );
            linked += 1;
        }
    }
    assert!(linked > 0, "an evaluate-linked emission must exist");
}

#[test]
fn the_full_redaction_vocabulary_is_materialized() {
    let set = m5_evaluate_repl_sheet_set();
    for class in EvaluateRedactionClass::ALL {
        let in_eval = set.evaluations.iter().any(|e| {
            e.expression_redaction == class
                || e.result
                    .as_ref()
                    .map(|r| r.redaction == class)
                    .unwrap_or(false)
        });
        let in_console = set.console.iter().any(|c| c.redaction == class);
        assert!(
            in_eval || in_console,
            "redaction class {} is not materialized",
            class.as_str()
        );
    }
}

#[test]
fn tampering_with_a_posture_pill_fails_validation() {
    let mut set = m5_evaluate_repl_sheet_set();
    // Force a denied may-mutate evaluation to claim it permits dispatch.
    let ev = set
        .evaluations
        .iter_mut()
        .find(|e| e.disposition == ApprovalDisposition::Denied)
        .expect("a denied evaluation exists");
    ev.posture.permits_dispatch = true;
    assert!(
        set.validate().is_err(),
        "a denied evaluation claiming it permits dispatch must fail validation"
    );
}

#[test]
fn attaching_a_result_to_a_withheld_request_fails_validation() {
    let mut set = m5_evaluate_repl_sheet_set();
    let ev = set
        .evaluations
        .iter_mut()
        .find(|e| e.disposition == ApprovalDisposition::Pending)
        .expect("a pending evaluation exists");
    ev.result = Some(EvaluateResult::build(
        "debug.evaluate.result:tampered:9999",
        EvaluateOutcome::Completed,
        "tampered result",
        Some("value:digest:tamper"),
        "tampered",
        false,
        EvaluateRedactionClass::NotRedacted,
        M5_EVALUATE_REPL_SHEETS_AS_OF,
    ));
    assert!(
        set.validate().is_err(),
        "a pending request carrying a result must fail validation"
    );
}

#[test]
fn running_an_effectful_eval_on_an_inspect_only_context_fails_validation() {
    let mut set = m5_evaluate_repl_sheet_set();
    let ev = set
        .evaluations
        .iter_mut()
        .find(|e| !e.context.authority.allows_mutation() && e.purity.requires_approval())
        .expect("an inspect-only effectful evaluation exists");
    // Force it to look approved-and-dispatchable against the recording.
    ev.disposition = ApprovalDisposition::Approved;
    ev.disposition_token = ApprovalDisposition::Approved.as_str().to_owned();
    ev.posture = EvaluatePosturePill::derive(
        ev.purity,
        ApprovalDisposition::Approved,
        ev.context.authority,
    );
    ev.actor.reviewed_by_ref = Some("actor:user:0002".to_owned());
    assert!(
        set.validate().is_err(),
        "an effectful evaluation that would run against an inspect-only context must fail"
    );
}

#[test]
fn unlinking_redaction_from_a_console_body_fails_validation() {
    let mut set = m5_evaluate_repl_sheet_set();
    let em = set
        .console
        .iter_mut()
        .find(|c| c.redaction.is_redacted())
        .expect("a redacted emission exists");
    // Force a redacted emission to carry a body.
    em.body_digest = Some("console:digest:leak".to_owned());
    assert!(
        set.validate().is_err(),
        "a redacted console emission carrying a body must fail validation"
    );
}

#[test]
fn lines_projection_covers_evaluations_console_and_invariants() {
    let set = m5_evaluate_repl_sheet_set();
    let lines = m5_evaluate_repl_sheet_lines(&set);
    assert!(lines.iter().any(|l| l.contains("Evaluations:")));
    assert!(lines.iter().any(|l| l.contains("Console:")));
    assert!(lines.iter().any(|l| l.contains("Invariants:")));
    assert!(lines.iter().any(|l| l.contains("purity=pure")));
    assert!(lines.iter().any(|l| l.contains("purity=may_mutate")));
    assert!(lines.iter().any(|l| l.contains("disposition=blocked")));
    assert!(lines.iter().any(|l| l.contains("result=<withheld")));
    assert!(lines.iter().any(|l| l.contains("user_input")));
    assert!(lines.iter().any(|l| l.contains("target_output")));
}

#[test]
fn enum_tokens_are_stable_and_unique() {
    fn unique<const N: usize>(tokens: [&str; N]) -> bool {
        tokens
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            == N
    }
    assert!(unique(EvaluatePurityClass::ALL.map(|p| p.as_str())));
    assert!(unique(ApprovalDisposition::ALL.map(|d| d.as_str())));
    assert!(unique(EvaluateContextScope::ALL.map(|s| s.as_str())));
    assert!(unique(EvaluateContextAuthority::ALL.map(|a| a.as_str())));
    assert!(unique(EvaluateActorClass::ALL.map(|a| a.as_str())));
    assert!(unique(EvaluateOutcome::ALL.map(|o| o.as_str())));
    assert!(unique(EvaluateRedactionClass::ALL.map(|r| r.as_str())));
    assert!(unique(ConsoleDirection::ALL.map(|d| d.as_str())));
    assert!(unique(ConsoleStreamClass::ALL.map(|s| s.as_str())));
    assert!(unique(ConsoleLiveness::ALL.map(|l| l.as_str())));
}
