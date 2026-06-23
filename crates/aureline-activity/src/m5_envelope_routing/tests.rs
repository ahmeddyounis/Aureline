//! Unit tests for the envelope-routing engine, producer registry, invariants, and
//! export-safety rules.

use super::*;

fn ai_envelope() -> NotificationEnvelope {
    let bundle = envelope_routing_bundle();
    bundle
        .envelope("notification_envelope:ai.awaiting_approval:0001")
        .expect("ai envelope present")
        .clone()
}

fn security_envelope() -> NotificationEnvelope {
    let bundle = envelope_routing_bundle();
    bundle
        .envelope("notification_envelope:security.credential_revoked:0001")
        .expect("security envelope present")
        .clone()
}

fn context(id: &str) -> RoutingContext {
    let bundle = envelope_routing_bundle();
    bundle.context(id).expect("context present").clone()
}

#[test]
fn bundle_validates_and_all_invariants_hold() {
    let bundle = envelope_routing_bundle();
    bundle.validate().expect("canonical bundle validates");
    assert!(bundle.all_invariants_hold());
    assert!(!bundle.invariants.is_empty());
}

#[test]
fn bundle_is_deterministic() {
    assert_eq!(envelope_routing_bundle(), envelope_routing_bundle());
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = envelope_routing_bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn every_subsystem_has_exactly_one_producer() {
    let bundle = envelope_routing_bundle();
    for subsystem in SourceSubsystemClass::ALL {
        let count = bundle
            .producers
            .iter()
            .filter(|p| p.source_subsystem == subsystem)
            .count();
        assert_eq!(count, 1, "subsystem {} producer count", subsystem.as_str());
    }
}

#[test]
fn every_producer_routes_the_typed_path() {
    let bundle = envelope_routing_bundle();
    for producer in &bundle.producers {
        assert!(producer.routes_through_typed_envelope);
        assert!(!producer.retains_surface_local_logic);
        assert!(
            bundle.envelope(&producer.emits_envelope_id).is_some(),
            "producer {} emits a known envelope",
            producer.producer_id
        );
    }
}

#[test]
fn every_envelope_carries_the_contract_fields() {
    let bundle = envelope_routing_bundle();
    for envelope in &bundle.envelopes {
        assert!(!envelope.dedupe_key.is_empty());
        assert!(!envelope.recommended_surfaces.is_empty());
        assert!(envelope.recommends(FanoutChannelClass::InAppActivityCenter));
        assert!(!envelope.action_target.action_target_id.is_empty());
        assert!(!envelope.reopen_targets.is_empty());
        assert!(envelope.carries_durable_record);
        assert!(envelope.carries_localizable_copy);
        assert!(envelope
            .reopen_targets
            .contains(&envelope.action_target.reopen_target));
    }
}

#[test]
fn routing_is_reproducible() {
    let envelope = ai_envelope();
    let ctx = context("context:default_focused");
    assert_eq!(
        route_envelope(&envelope, &ctx),
        route_envelope(&envelope, &ctx)
    );
}

#[test]
fn in_app_is_always_a_durable_delivery() {
    let bundle = envelope_routing_bundle();
    for decision in &bundle.decisions {
        let in_app = decision
            .outcome(FanoutChannelClass::InAppActivityCenter)
            .expect("in-app outcome present");
        assert_eq!(in_app.disposition, RouteDispositionClass::Deliver);
        assert!(in_app.delivers_durable_record);
        assert!(decision.durable_record_present);
    }
}

#[test]
fn quiet_hours_defers_fanout_but_keeps_durable_record() {
    let envelope = ai_envelope();
    let ctx = context("context:background_quiet_hours");
    let decision = route_envelope(&envelope, &ctx);
    assert!(decision.durable_record_present);
    let os = decision
        .outcome(FanoutChannelClass::OsNativeNotification)
        .expect("os outcome");
    assert_eq!(os.disposition, RouteDispositionClass::DeferQuietHours);
}

#[test]
fn focus_and_dnd_defer_fanout() {
    let envelope = ai_envelope();
    let ctx = context("context:presenting_dnd");
    let decision = route_envelope(&envelope, &ctx);
    let os = decision
        .outcome(FanoutChannelClass::OsNativeNotification)
        .expect("os outcome");
    assert_eq!(os.disposition, RouteDispositionClass::DeferFocus);
}

#[test]
fn managed_locked_suppresses_companion_fanout() {
    let envelope = ai_envelope();
    let ctx = context("context:managed_locked_owner");
    let decision = route_envelope(&envelope, &ctx);
    for surface in [
        FanoutChannelClass::BrowserCompanion,
        FanoutChannelClass::MobileCompanion,
    ] {
        let outcome = decision.outcome(surface).expect("companion outcome");
        assert_eq!(
            outcome.disposition,
            RouteDispositionClass::SuppressedByAdminPolicy
        );
    }
    // The durable record is still delivered.
    assert!(decision.durable_record_present);
}

#[test]
fn security_advisory_breaks_through_every_restriction() {
    let envelope = security_envelope();
    for ctx_id in [
        "context:background_quiet_hours",
        "context:presenting_dnd",
        "context:guest_muted",
    ] {
        let ctx = context(ctx_id);
        let decision = route_envelope(&envelope, &ctx);
        let os = decision
            .outcome(FanoutChannelClass::OsNativeNotification)
            .expect("os outcome");
        assert!(
            os.disposition.is_delivered(),
            "security advisory must break through in {ctx_id}"
        );
    }
}

#[test]
fn user_mute_suppresses_non_security_fanout() {
    let envelope = ai_envelope();
    let ctx = context("context:guest_muted");
    let decision = route_envelope(&envelope, &ctx);
    let os = decision
        .outcome(FanoutChannelClass::OsNativeNotification)
        .expect("os outcome");
    assert_eq!(
        os.disposition,
        RouteDispositionClass::SuppressedByUserPolicy
    );
    assert!(decision.durable_record_present);
}

#[test]
fn privacy_never_widens_on_out_of_window_surfaces() {
    let bundle = envelope_routing_bundle();
    for decision in &bundle.decisions {
        let envelope = bundle.envelope(&decision.envelope_id).expect("envelope");
        for outcome in &decision.outcomes {
            if outcome.surface == FanoutChannelClass::InAppActivityCenter {
                continue;
            }
            let profile = channel_profile(outcome.surface);
            assert!(
                redaction_rank(outcome.applied_redaction)
                    >= redaction_rank(envelope.default_redaction),
                "{} widened past the envelope default",
                outcome.surface.as_str()
            );
            assert!(
                redaction_rank(outcome.applied_redaction)
                    >= redaction_rank(profile.default_redaction),
                "{} widened past the channel default",
                outcome.surface.as_str()
            );
        }
    }
}

#[test]
fn preview_approval_actions_never_execute_inline_on_fanout() {
    let bundle = envelope_routing_bundle();
    for decision in &bundle.decisions {
        let envelope = bundle.envelope(&decision.envelope_id).expect("envelope");
        if !envelope.action_target.routes_through_preview_approval {
            continue;
        }
        for outcome in &decision.outcomes {
            if outcome.surface == FanoutChannelClass::InAppActivityCenter {
                continue;
            }
            assert!(
                outcome.dangerous_action_handoff_to_in_product,
                "out-of-window {} must hand a gated action to the in-product surface",
                outcome.surface.as_str()
            );
        }
    }
}

#[test]
fn every_surface_outcome_shares_one_action_target() {
    let bundle = envelope_routing_bundle();
    for decision in &bundle.decisions {
        for outcome in &decision.outcomes {
            assert_eq!(outcome.action_target_id, decision.action_target_id);
        }
        let envelope = bundle.envelope(&decision.envelope_id).expect("envelope");
        assert_eq!(
            decision.action_target_id,
            envelope.action_target.action_target_id
        );
    }
}

#[test]
fn limited_collaboration_role_keeps_collab_handoffs_in_product() {
    let bundle = envelope_routing_bundle();
    let collab = bundle
        .envelope("notification_envelope:collab.review_requested:0001")
        .expect("collab envelope")
        .clone();
    let ctx = context("context:guest_muted");
    let decision = route_envelope(&collab, &ctx);
    let browser = decision
        .outcome(FanoutChannelClass::BrowserCompanion)
        .expect("browser outcome");
    assert_eq!(browser.disposition, RouteDispositionClass::RouteToInProduct);
}

#[test]
fn screen_reader_posture_is_recorded_on_every_outcome() {
    let envelope = ai_envelope();
    let ctx = context("context:screen_reader_reviewer");
    let decision = route_envelope(&envelope, &ctx);
    for outcome in &decision.outcomes {
        assert!(outcome.requires_accessible_affordance);
    }
}

#[test]
fn validate_rejects_a_raw_payload_flag_flip() {
    let mut bundle = envelope_routing_bundle();
    bundle.raw_payload_excluded = false;
    assert!(bundle.validate().is_err());
    assert!(!bundle.is_support_export_safe());
}

#[test]
fn validate_rejects_an_unsafe_ref() {
    let mut bundle = envelope_routing_bundle();
    bundle.envelopes[0].scope_ref = "https://internal.example.com/scope".to_owned();
    assert!(!bundle.is_support_export_safe());
    assert!(bundle.validate().is_err());
}

#[test]
fn validate_rejects_a_producer_with_surface_local_logic() {
    let mut bundle = envelope_routing_bundle();
    bundle.producers[0].retains_surface_local_logic = true;
    assert!(bundle.validate().is_err());
}

#[test]
fn validate_rejects_a_non_reproducible_decision() {
    let mut bundle = envelope_routing_bundle();
    bundle.decisions[0]
        .outcomes
        .retain(|o| o.surface == FanoutChannelClass::InAppActivityCenter);
    assert!(bundle.validate().is_err());
}

#[test]
fn human_readable_projection_renders() {
    let bundle = envelope_routing_bundle();
    let lines = envelope_routing_lines(&bundle);
    assert!(lines.iter().any(|l| l.contains("Envelope-routing bundle")));
    assert!(lines.iter().any(|l| l.contains("Producers:")));
    assert!(lines.iter().any(|l| l.contains("Decisions:")));
    for subsystem in SourceSubsystemClass::ALL {
        assert!(
            lines.iter().any(|l| l.contains(subsystem.as_str())),
            "projection must mention subsystem {}",
            subsystem.as_str()
        );
    }
}
