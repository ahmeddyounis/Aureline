//! Unit tests for the suppression engine, policy/signal corpus, invariants, and
//! export-safety rules.

use super::*;

fn bundle() -> QuietHoursSuppressionBundle {
    quiet_hours_suppression_bundle()
}

fn signal_named(slug: &str) -> AttentionSignal {
    bundle()
        .signal(&format!("attention_signal:{slug}:0001"))
        .expect("signal present")
        .clone()
}

fn policy_named(slug: &str) -> SuppressionPolicy {
    bundle()
        .policy(&format!("suppression_policy:{slug}:0001"))
        .expect("policy present")
        .clone()
}

#[test]
fn bundle_validates_and_all_invariants_hold() {
    let bundle = bundle();
    bundle.validate().expect("canonical bundle validates");
    assert!(bundle.all_invariants_hold());
    assert!(!bundle.invariants.is_empty());
}

#[test]
fn bundle_is_deterministic() {
    assert_eq!(
        quiet_hours_suppression_bundle(),
        quiet_hours_suppression_bundle()
    );
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn evaluate_is_deterministic_and_reproducible() {
    let signal = signal_named("ai.awaiting_approval");
    let policy = policy_named("quiet_hours");
    let a = evaluate_suppression(&signal, &policy);
    let b = evaluate_suppression(&signal, &policy);
    assert_eq!(a, b);
}

#[test]
fn in_app_always_shows_the_durable_record() {
    let bundle = bundle();
    for decision in &bundle.decisions {
        let in_app = decision
            .outcome(FanoutChannelClass::InAppActivityCenter)
            .expect("in-app outcome present");
        assert_eq!(in_app.disposition, SuppressionDispositionClass::Shown);
        assert!(in_app.delivers_durable_record);
        assert_eq!(in_app.suppression_source, SuppressionSourceClass::None);
        assert!(decision.durable_record_present);
    }
}

#[test]
fn clear_policy_shows_everything_out_of_window() {
    let policy = policy_named("clear");
    // A routine, summary-safe event is shown on every surface under a clear policy.
    let signal = signal_named("task.completed");
    let decision = evaluate_suppression(&signal, &policy);
    for outcome in &decision.outcomes {
        assert_eq!(
            outcome.disposition,
            SuppressionDispositionClass::Shown,
            "surface {} should show under a clear policy",
            outcome.surface.as_str()
        );
    }
    assert!(decision.ledger_entries.is_empty());
}

#[test]
fn quiet_hours_withholds_routine_but_keeps_durable_record() {
    let signal = signal_named("task.completed");
    let policy = policy_named("quiet_hours");
    let decision = evaluate_suppression(&signal, &policy);

    // Out-of-window surfaces are withheld; the in-app record still shows.
    for surface in [
        FanoutChannelClass::OsNativeNotification,
        FanoutChannelClass::BrowserCompanion,
        FanoutChannelClass::MobileCompanion,
    ] {
        let outcome = decision.outcome(surface).expect("outcome present");
        assert_eq!(outcome.disposition, SuppressionDispositionClass::Withheld);
        assert_eq!(
            outcome.suppression_source,
            SuppressionSourceClass::QuietHours
        );
        assert_eq!(
            outcome.resulting_state,
            Some(AttentionStateClass::QuietHoursDeferred)
        );
    }
    assert!(decision.durable_record_present);
    // A withheld out-of-window surface produces a ledger entry separate from audit
    // history that does not imply the underlying object disappeared.
    assert_eq!(decision.ledger_entries.len(), 3);
    for entry in &decision.ledger_entries {
        assert!(entry.separate_from_audit_history);
        assert!(!entry.implies_underlying_disappeared);
    }
}

#[test]
fn high_importance_escapes_quiet_hours_only_when_named() {
    let policy = policy_named("quiet_hours");

    // Named approval handoff escapes (downgraded) on out-of-window surfaces.
    let named = signal_named("ai.awaiting_approval");
    let named_decision = evaluate_suppression(&named, &policy);
    let os_named = named_decision
        .outcome(FanoutChannelClass::OsNativeNotification)
        .expect("os outcome present");
    assert_eq!(
        os_named.disposition,
        SuppressionDispositionClass::Downgraded
    );
    assert!(os_named.escaped_suppression);
    assert!(os_named.names_scope_and_consequence);
    assert!(named_decision.high_importance_escaped);

    // An unnamed high-importance handoff is withheld out-of-window.
    let unnamed = signal_named("collab.review_requested");
    assert!(unnamed.is_high_importance());
    assert!(!unnamed.names_scope_and_consequence());
    let unnamed_decision = evaluate_suppression(&unnamed, &policy);
    let os_unnamed = unnamed_decision
        .outcome(FanoutChannelClass::OsNativeNotification)
        .expect("os outcome present");
    assert_eq!(
        os_unnamed.disposition,
        SuppressionDispositionClass::Withheld
    );
    assert!(!os_unnamed.escaped_suppression);
    assert!(!unnamed_decision.high_importance_escaped);
    // But it remains accountable: the blocked high-importance event requires an audit
    // trail and a ledger entry.
    assert!(os_unnamed.audit_trail_required);
}

#[test]
fn security_advisory_is_never_silenced() {
    let signal = signal_named("security.credential_revoked");
    for policy in &bundle().policies {
        let decision = evaluate_suppression(&signal, policy);
        assert!(!decision.security_silenced);
        // The in-app surface always shows it.
        assert_eq!(
            decision
                .outcome(FanoutChannelClass::InAppActivityCenter)
                .expect("in-app present")
                .disposition,
            SuppressionDispositionClass::Shown
        );
        // At least one surface delivered it.
        assert!(decision
            .outcomes
            .iter()
            .any(|o| o.disposition.is_delivered()));
    }
}

#[test]
fn security_advisory_escapes_quiet_hours_downgraded() {
    let signal = signal_named("security.credential_revoked");
    let policy = policy_named("quiet_hours");
    let decision = evaluate_suppression(&signal, &policy);
    let os = decision
        .outcome(FanoutChannelClass::OsNativeNotification)
        .expect("os present");
    assert_eq!(os.disposition, SuppressionDispositionClass::Downgraded);
    assert!(os.escaped_suppression);
    assert_eq!(os.suppression_source, SuppressionSourceClass::QuietHours);
}

#[test]
fn managed_locked_withholds_companions_but_not_os() {
    let signal = signal_named("collab.review_requested");
    let policy = policy_named("managed_locked");
    let decision = evaluate_suppression(&signal, &policy);
    for surface in [
        FanoutChannelClass::BrowserCompanion,
        FanoutChannelClass::MobileCompanion,
    ] {
        let outcome = decision
            .outcome(surface)
            .expect("companion outcome present");
        assert_eq!(outcome.disposition, SuppressionDispositionClass::Withheld);
        assert_eq!(
            outcome.suppression_source,
            SuppressionSourceClass::AdminSuppression
        );
    }
    // The OS notification is not a companion; admin-lock does not withhold it.
    let os = decision
        .outcome(FanoutChannelClass::OsNativeNotification)
        .expect("os present");
    assert_ne!(
        os.suppression_source,
        SuppressionSourceClass::AdminSuppression
    );
}

#[test]
fn managed_endpoint_noncompliant_withholds_out_of_window() {
    let signal = signal_named("ai.awaiting_approval");
    let policy = policy_named("managed_endpoint_noncompliant");
    let decision = evaluate_suppression(&signal, &policy);
    for surface in [
        FanoutChannelClass::OsNativeNotification,
        FanoutChannelClass::BrowserCompanion,
        FanoutChannelClass::MobileCompanion,
    ] {
        let outcome = decision.outcome(surface).expect("outcome present");
        assert_eq!(outcome.disposition, SuppressionDispositionClass::Withheld);
        assert_eq!(
            outcome.suppression_source,
            SuppressionSourceClass::ManagedEndpointPolicy
        );
    }
    assert!(decision.durable_record_present);
}

#[test]
fn lock_screen_downgrades_sensitive_content() {
    // A security-critical event is downgraded to count-only on a locked screen, but a
    // summary-safe event is shown.
    let policy = policy_named("lock_screen");
    let sensitive = signal_named("trust.provider_changed");
    let decision = evaluate_suppression(&sensitive, &policy);
    let os = decision
        .outcome(FanoutChannelClass::OsNativeNotification)
        .expect("os present");
    assert_eq!(os.disposition, SuppressionDispositionClass::Downgraded);
    assert_eq!(
        os.suppression_source,
        SuppressionSourceClass::LockScreenPrivacy
    );
    assert_eq!(os.applied_redaction, AttentionRedactionClass::CountOnly);

    let routine = signal_named("task.completed");
    let routine_decision = evaluate_suppression(&routine, &policy);
    assert_eq!(
        routine_decision
            .outcome(FanoutChannelClass::OsNativeNotification)
            .expect("os present")
            .disposition,
        SuppressionDispositionClass::Shown
    );
}

#[test]
fn downgrade_never_widens_privacy() {
    let bundle = bundle();
    for decision in &bundle.decisions {
        let signal = bundle.signal(&decision.signal_id).expect("signal present");
        for outcome in &decision.outcomes {
            if outcome.surface == FanoutChannelClass::InAppActivityCenter {
                continue;
            }
            if outcome.disposition.is_delivered() {
                assert!(
                    redaction_rank(outcome.applied_redaction)
                        >= redaction_rank(surface_normal_redaction(signal, outcome.surface)),
                    "surface {} widened privacy",
                    outcome.surface.as_str()
                );
            }
        }
    }
}

#[test]
fn every_disposition_and_source_is_exercised() {
    let bundle = bundle();
    for disp in SuppressionDispositionClass::ALL {
        assert!(
            bundle
                .decisions
                .iter()
                .any(|d| d.outcomes.iter().any(|o| o.disposition == disp)),
            "disposition {} exercised",
            disp.as_str()
        );
    }
    for source in SuppressionSourceClass::ALL {
        if source == SuppressionSourceClass::None {
            continue;
        }
        assert!(
            bundle
                .decisions
                .iter()
                .any(|d| d.outcomes.iter().any(|o| o.suppression_source == source)),
            "source {} exercised",
            source.as_str()
        );
    }
}

#[test]
fn ledger_entries_only_for_non_shown_out_of_window() {
    let bundle = bundle();
    for decision in &bundle.decisions {
        let expected: usize = decision
            .outcomes
            .iter()
            .filter(|o| o.surface != FanoutChannelClass::InAppActivityCenter)
            .filter(|o| o.disposition != SuppressionDispositionClass::Shown)
            .count();
        assert_eq!(decision.ledger_entries.len(), expected);
        for entry in &decision.ledger_entries {
            assert_ne!(entry.surface, FanoutChannelClass::InAppActivityCenter);
            assert!(entry.separate_from_audit_history);
            assert!(!entry.implies_underlying_disappeared);
            assert!(is_export_safe_ref(&entry.reopen_anchor_ref));
        }
    }
}

#[test]
fn bundle_round_trips_through_json() {
    let bundle = bundle();
    let json = serde_json::to_string(&bundle).expect("serializes");
    let back: QuietHoursSuppressionBundle = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(back, bundle);
}
