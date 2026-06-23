//! Unit tests for the attention-action engine, action definitions, invariants, and
//! export-safety rules.

use super::*;

fn item_named(item_id: &str) -> AttentionItem {
    let bundle = attention_actions_bundle();
    bundle.item(item_id).expect("item present").clone()
}

#[test]
fn bundle_validates_and_all_invariants_hold() {
    let bundle = attention_actions_bundle();
    bundle.validate().expect("canonical bundle validates");
    assert!(bundle.all_invariants_hold());
    assert!(!bundle.invariants.is_empty());
}

#[test]
fn bundle_is_deterministic() {
    assert_eq!(attention_actions_bundle(), attention_actions_bundle());
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = attention_actions_bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn every_action_is_defined_exactly_once() {
    let bundle = attention_actions_bundle();
    assert_eq!(
        bundle.action_definitions.len(),
        AttentionActionClass::ALL.len()
    );
    for action in AttentionActionClass::ALL {
        assert_eq!(
            bundle
                .action_definitions
                .iter()
                .filter(|d| d.action == action)
                .count(),
            1,
            "action {} defined once",
            action.as_str()
        );
    }
}

#[test]
fn the_five_actions_carry_distinct_state_and_badge() {
    let states: std::collections::BTreeSet<&str> = AttentionActionClass::ALL
        .iter()
        .map(|a| action_definition(*a).resulting_state.as_str())
        .collect();
    let badges: std::collections::BTreeSet<&str> = AttentionActionClass::ALL
        .iter()
        .map(|a| action_definition(*a).badge_effect.as_str())
        .collect();
    assert_eq!(
        states.len(),
        5,
        "each action has a distinct resulting state"
    );
    assert_eq!(badges.len(), 5, "each action has a distinct badge effect");
}

#[test]
fn retention_classes_differentiate_actions() {
    use AttentionActionClass::*;
    assert_eq!(
        action_definition(Dismiss).retention_class,
        "durable_until_archived"
    );
    assert_eq!(
        action_definition(Acknowledge).retention_class,
        "durable_until_resolved"
    );
    assert_eq!(
        action_definition(Snooze).retention_class,
        "suppression_state_separate"
    );
    assert_eq!(
        action_definition(Mute).retention_class,
        "suppression_state_separate"
    );
    assert_eq!(
        action_definition(Resolve).retention_class,
        "durable_until_archived"
    );
}

#[test]
fn apply_is_deterministic_and_reproducible() {
    let item = item_named("attention_item:collab.review_requested:0001");
    for action in &item.supported_actions {
        let a = apply_attention_action(&item, *action);
        let b = apply_attention_action(&item, *action);
        assert_eq!(a, b, "applying {} is deterministic", action.as_str());
    }
}

#[test]
fn every_action_keeps_the_record_and_clears_the_badge() {
    let item = item_named("attention_item:collab.review_requested:0001");
    for action in &item.supported_actions {
        let outcome = apply_attention_action(&item, *action);
        assert!(
            outcome.keeps_underlying_record,
            "{} keeps record",
            action.as_str()
        );
        assert!(
            outcome.badge_count_after < outcome.badge_count_before,
            "{} clears the badge",
            action.as_str()
        );
        assert!(outcome.badge_delta <= 0);
    }
}

#[test]
fn exact_reopen_continuity_survives_every_action() {
    let item = item_named("attention_item:ai.awaiting_approval:0001");
    for action in &item.supported_actions {
        let outcome = apply_attention_action(&item, *action);
        assert!(outcome.reopen_continuity_preserved);
        assert_eq!(outcome.reopen_target, item.reopen_target);
        assert_eq!(outcome.reopen_anchor_ref, item.reopen_anchor_ref);
        assert_eq!(outcome.action_target_id, item.action_target_id);
        assert!(!outcome.replays_side_effects);
    }
}

#[test]
fn snooze_and_mute_carry_resume_conditions_others_do_not() {
    let item = item_named("attention_item:collab.review_requested:0001");
    assert!(apply_attention_action(&item, AttentionActionClass::Snooze)
        .resume_condition
        .is_some());
    assert!(apply_attention_action(&item, AttentionActionClass::Mute)
        .resume_condition
        .is_some());
    assert!(apply_attention_action(&item, AttentionActionClass::Dismiss)
        .resume_condition
        .is_none());
    assert!(
        apply_attention_action(&item, AttentionActionClass::Acknowledge)
            .resume_condition
            .is_none()
    );
    assert!(apply_attention_action(&item, AttentionActionClass::Resolve)
        .resume_condition
        .is_none());
}

#[test]
fn only_snooze_and_mute_create_separate_suppression_state() {
    use AttentionActionClass::*;
    assert!(action_definition(Snooze).creates_separate_suppression_state);
    assert!(action_definition(Mute).creates_separate_suppression_state);
    assert!(!action_definition(Dismiss).creates_separate_suppression_state);
    assert!(!action_definition(Acknowledge).creates_separate_suppression_state);
    assert!(!action_definition(Resolve).creates_separate_suppression_state);
    for action in AttentionActionClass::ALL {
        assert!(action_definition(action).audit_append_only);
    }
}

#[test]
fn security_advisory_cannot_be_silenced() {
    let bundle = attention_actions_bundle();
    let security = bundle
        .item("attention_item:security.credential_revoked:0001")
        .expect("security item present");
    assert!(!security.supports(AttentionActionClass::Mute));
    assert!(!security.supports(AttentionActionClass::Dismiss));
    assert!(!security.supports(AttentionActionClass::Snooze));
    assert!(security.supports(AttentionActionClass::Acknowledge));
    assert!(security.supports(AttentionActionClass::Resolve));
}

#[test]
fn every_outcome_propagates_authoritatively_to_the_in_app_center_and_badge() {
    let bundle = attention_actions_bundle();
    for outcome in &bundle.outcomes {
        let in_app = outcome
            .propagation(FanoutChannelClass::InAppActivityCenter)
            .expect("in-app propagation present");
        assert_eq!(
            in_app.propagation,
            SurfaceActionPropagationClass::ApplyAuthoritative
        );
        assert!(outcome
            .propagation(FanoutChannelClass::DockTaskbarBadge)
            .is_some());
        for p in &outcome.surface_propagation {
            assert_eq!(p.reflects_action_target_id, outcome.action_target_id);
            assert!(!p.replays_side_effect);
        }
    }
}

#[test]
fn all_actions_are_exercised_in_the_corpus() {
    let bundle = attention_actions_bundle();
    for action in AttentionActionClass::ALL {
        assert!(
            bundle.outcomes.iter().any(|o| o.action == action),
            "action {} exercised",
            action.as_str()
        );
    }
}

#[test]
fn bundle_round_trips_through_json() {
    let bundle = attention_actions_bundle();
    let json = serde_json::to_string(&bundle).expect("serializes");
    let back: AttentionActionsBundle = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(back, bundle);
}
