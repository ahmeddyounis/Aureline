//! Unit tests for the badge-aggregate engine, dedupe and coalescing rules, cross-surface
//! parity, security-never-silenced rule, telemetry, and export-safety.

use super::*;

fn bundle() -> BadgeAggregatesBundle {
    badge_aggregates_bundle()
}

fn agg(scope: AttentionScopeClass) -> BadgeAggregate {
    bundle()
        .aggregate(scope)
        .expect("aggregate present")
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
    assert_eq!(badge_aggregates_bundle(), badge_aggregates_bundle());
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(!bundle.telemetry.captures_message_text);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn count_class_classifies_every_bucket() {
    assert_eq!(CountClass::classify(0), CountClass::None);
    assert_eq!(CountClass::classify(1), CountClass::Single);
    assert_eq!(CountClass::classify(2), CountClass::Few);
    assert_eq!(CountClass::classify(9), CountClass::Few);
    assert_eq!(CountClass::classify(10), CountClass::Many);
    assert_eq!(CountClass::classify(98), CountClass::Many);
    assert_eq!(CountClass::classify(99), CountClass::Saturated);
    assert_eq!(CountClass::classify(1_000), CountClass::Saturated);
    assert_eq!(count_display(5), "5");
    assert_eq!(count_display(150), "99+");
}

#[test]
fn repeats_dedupe_into_one_object_not_a_raw_tally() {
    // The session scope has a failure repeated three times plus a progress and an AI object:
    // five raw counted events collapse to three distinct durable objects.
    let session = agg(AttentionScopeClass::Session);
    assert_eq!(session.count, 3);
    assert_eq!(session.deduped_count, 3);
    assert_eq!(session.raw_event_count, 5);
    assert!(session.raw_event_count > session.count);
    assert_eq!(session.count_class, CountClass::Few);
    // The repeated failure appears exactly once among the deduped objects.
    let failures = session
        .deduped_objects
        .iter()
        .filter(|o| o.object_key == "session:save_conflict:doc7")
        .count();
    assert_eq!(failures, 1);
}

#[test]
fn many_class_is_exercised_with_dedupe() {
    // Ten distinct diagnostics plus one repeat: count ten, raw eleven.
    let workspace = agg(AttentionScopeClass::Workspace);
    assert_eq!(workspace.count, 10);
    assert_eq!(workspace.raw_event_count, 11);
    assert_eq!(workspace.count_class, CountClass::Many);
}

#[test]
fn zero_count_names_its_exclusion_reasons() {
    // Every app-global item is muted, suppressed, or settled: count zero, reasons named.
    let app = agg(AttentionScopeClass::AppGlobal);
    assert_eq!(app.count, 0);
    assert_eq!(app.count_class, CountClass::None);
    assert_eq!(app.count_display, "0");
    assert!(app.deduped_objects.is_empty());
    assert_eq!(app.freshness, BadgeFreshnessClass::None);
    assert!(app
        .muted_reasons
        .contains(&BadgeMuteReasonClass::MutedByFocusMode));
    assert!(app
        .suppressed_reasons
        .contains(&BadgeSuppressionReasonClass::RateLimited));
}

#[test]
fn quiet_hours_and_policy_exclusions_are_named() {
    // Tenant scope counts one managed alert; a policy-suppressed and a deferred item are
    // excluded but named.
    let tenant = agg(AttentionScopeClass::TenantOrg);
    assert_eq!(tenant.count, 1);
    assert_eq!(tenant.count_class, CountClass::Single);
    assert!(tenant
        .suppressed_reasons
        .contains(&BadgeSuppressionReasonClass::PolicySuppressed));
    assert!(tenant
        .active_quiet_hours_modes
        .contains(&QuietHoursModeClass::FollowAdminPolicy));
}

#[test]
fn security_advisory_is_never_silenced() {
    // The collaboration scope has a security advisory the user muted and quiet hours deferred;
    // it still counts.
    let collab = agg(AttentionScopeClass::Collaboration);
    assert!(collab
        .deduped_objects
        .iter()
        .any(|o| o.object_key == "collab:security:token:1"));
    let advisory = bundle()
        .items
        .iter()
        .find(|i| i.item_id == "durable_item:collab.security_advisory")
        .expect("advisory present")
        .clone();
    assert!(advisory.mute_reason.is_named());
    assert!(advisory.quiet_hours_mode.is_deferring());
    assert_eq!(advisory.contribution(), BadgeContributionClass::Counted);
}

#[test]
fn single_object_badge_reopens_the_exact_object() {
    let tenant = agg(AttentionScopeClass::TenantOrg);
    assert_eq!(tenant.count, 1);
    assert_eq!(
        tenant.reopen_anchor_ref,
        tenant.deduped_objects[0].reopen_anchor_ref
    );
}

#[test]
fn multi_object_badge_reopens_the_scope_pending_list() {
    let session = agg(AttentionScopeClass::Session);
    assert!(session.count > 1);
    assert_eq!(
        session.reopen_anchor_ref,
        "aureline://activity/session/pending"
    );
    assert_eq!(session.reopen_target, ReopenTargetClass::ActivityJobRow);
}

#[test]
fn every_surface_shows_the_same_count_per_scope() {
    let bundle = bundle();
    for aggregate in &bundle.aggregates {
        for surface in GOVERNED_BADGE_SURFACES {
            let badge = bundle
                .surface_badge(surface, aggregate.scope)
                .expect("surface badge present");
            assert_eq!(badge.count, aggregate.count, "count parity for {surface:?}");
            assert_eq!(badge.count_class, aggregate.count_class);
            assert_eq!(badge.reopen_anchor_ref, aggregate.reopen_anchor_ref);
        }
    }
}

#[test]
fn dock_badge_is_always_count_only() {
    let bundle = bundle();
    for badge in &bundle.surface_badges {
        if badge.surface == FanoutChannelClass::DockTaskbarBadge {
            assert_eq!(badge.applied_redaction, AttentionRedactionClass::CountOnly);
        }
    }
}

#[test]
fn no_surface_widens_privacy_below_the_floor() {
    let bundle = bundle();
    for badge in &bundle.surface_badges {
        let floor = bundle.aggregate(badge.scope).unwrap().privacy_floor;
        assert!(
            badge.applied_redaction >= floor,
            "surface {:?} widened privacy",
            badge.surface
        );
    }
}

#[test]
fn repeated_failures_coalesce_into_one_object() {
    let bundle = bundle();
    let failure = bundle
        .coalesced_failures
        .iter()
        .find(|f| f.root_cause_key == "session:save_conflict:doc7")
        .expect("coalesced failure present");
    assert_eq!(failure.occurrence_count, 3);
    assert!(failure.spam_prevented);
    assert_eq!(failure.scope, AttentionScopeClass::Session);
    assert!(failure.durable_record_present);
    // It reopens the representative item's exact authoritative object.
    let rep = bundle
        .items
        .iter()
        .find(|i| i.item_id == failure.representative_item_id)
        .expect("representative present");
    assert_eq!(failure.reopen_anchor_ref, rep.reopen_anchor_ref);
    assert_eq!(failure.reopen_target, rep.reopen_target);
    // The badge counts the coalesced object exactly once.
    let session = bundle.aggregate(AttentionScopeClass::Session).unwrap();
    let counted = session
        .deduped_objects
        .iter()
        .filter(|o| o.object_key == "session:save_conflict:doc7")
        .count();
    assert_eq!(counted, 1);
}

#[test]
fn telemetry_reconciles_and_captures_no_text() {
    let bundle = bundle();
    let t = &bundle.telemetry;
    assert!(!t.captures_message_text);
    assert_eq!(t.total_items, bundle.items.len());
    let outcome_sum: usize = t.outcome_rollup.iter().map(|r| r.count).sum();
    assert_eq!(outcome_sum, t.total_items);
    assert_eq!(
        t.total_raw_counted,
        t.total_counted + t.total_deduped_repeats
    );
    assert_eq!(
        t.total_items,
        t.total_raw_counted
            + t.total_muted
            + t.total_suppressed
            + t.total_deferred
            + t.total_settled
    );
    let badge_sum: usize = bundle.aggregates.iter().map(|a| a.count).sum();
    assert_eq!(t.total_counted, badge_sum);
    // Every route shows the same total — parity at the telemetry level.
    for row in &t.route_rollup {
        assert_eq!(row.badge_count_total, t.total_counted);
    }
    assert_eq!(t.total_failure_occurrences, 3);
}

#[test]
fn aggregates_reproduce_from_corpus() {
    let bundle = bundle();
    assert_eq!(aggregate_badges(&bundle.items), bundle.aggregates);
    assert_eq!(surface_badges(&bundle.aggregates), bundle.surface_badges);
    assert_eq!(coalesce_failures(&bundle.items), bundle.coalesced_failures);
    assert_eq!(
        badge_telemetry(
            &bundle.items,
            &bundle.aggregates,
            &bundle.coalesced_failures
        ),
        bundle.telemetry
    );
}

#[test]
fn bundle_round_trips_through_json() {
    let bundle = bundle();
    let json = serde_json::to_string(&bundle).expect("serializes");
    let back: BadgeAggregatesBundle = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(back, bundle);
}
