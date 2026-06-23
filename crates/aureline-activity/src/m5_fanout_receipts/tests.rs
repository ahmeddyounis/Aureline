//! Unit tests for the fanout-receipt minting engine, source/condition corpus,
//! invariants, and export-safety rules.

use super::*;

fn bundle() -> FanoutReceiptsBundle {
    fanout_receipts_bundle()
}

fn source_named(slug: &str) -> FanoutSource {
    bundle()
        .source(&format!("notification_envelope:{slug}:0001"))
        .expect("source present")
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
    assert_eq!(fanout_receipts_bundle(), fanout_receipts_bundle());
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn mint_is_deterministic_and_reproducible() {
    let source = source_named("ai.awaiting_approval");
    let a = mint_dispatch(&source, FanoutConditionClass::LockedScreen);
    let b = mint_dispatch(&source, FanoutConditionClass::LockedScreen);
    assert_eq!(a, b);
}

#[test]
fn all_delivered_counts_every_destination_as_delivered() {
    let source = source_named("task.completed");
    let dispatch = mint_dispatch(&source, FanoutConditionClass::AllDelivered);
    assert_eq!(dispatch.delivered_count, GOVERNED_DESTINATIONS.len());
    assert_eq!(dispatch.undelivered_count, 0);
    for receipt in &dispatch.receipts {
        assert_eq!(receipt.delivery_state, FanoutDeliveryStateClass::Delivered);
        assert_eq!(
            receipt.resulting_state,
            AttentionStateClass::Shown,
            "delivered maps to the matrix shown state"
        );
        assert_eq!(
            receipt.stale_or_undelivered_reason,
            StaleUndeliveredReasonClass::None
        );
    }
}

#[test]
fn stale_copy_is_labeled_not_counted_delivered() {
    let source = source_named("task.completed");
    let dispatch = mint_dispatch(&source, FanoutConditionClass::MobileStale);
    let mobile = dispatch
        .receipt(FanoutChannelClass::MobileCompanion)
        .expect("mobile receipt present");
    assert_eq!(mobile.delivery_state, FanoutDeliveryStateClass::Stale);
    assert_eq!(mobile.resulting_state, AttentionStateClass::FanoutStale);
    assert_eq!(
        mobile.stale_or_undelivered_reason,
        StaleUndeliveredReasonClass::SupersededByNewerState
    );
    // The stale copy is not counted as delivered, and the others are.
    assert_eq!(dispatch.stale_count, 1);
    assert_eq!(dispatch.delivered_count, GOVERNED_DESTINATIONS.len() - 1);
    assert!(!dispatch
        .delivered_destinations()
        .contains(&FanoutChannelClass::MobileCompanion));
    assert!(dispatch.durable_record_present);
}

#[test]
fn undelivered_copy_is_labeled_with_a_reason() {
    let source = source_named("incident.flagged");
    let dispatch = mint_dispatch(&source, FanoutConditionClass::CompanionUndelivered);
    let browser = dispatch
        .receipt(FanoutChannelClass::BrowserCompanion)
        .expect("browser receipt present");
    assert_eq!(
        browser.delivery_state,
        FanoutDeliveryStateClass::Undelivered
    );
    assert_eq!(
        browser.resulting_state,
        AttentionStateClass::FanoutUndelivered
    );
    assert_eq!(
        browser.stale_or_undelivered_reason,
        StaleUndeliveredReasonClass::ClientUnreachable
    );
    assert_eq!(
        browser.summary_posture,
        FanoutSummaryPostureClass::NoSummary
    );
    assert!(dispatch.all_failures_labeled);
    assert!(dispatch.durable_record_present);
}

#[test]
fn managed_endpoint_blocks_payload_on_every_destination() {
    let source = source_named("ai.awaiting_approval");
    let dispatch = mint_dispatch(&source, FanoutConditionClass::ManagedEndpointBlocked);
    for receipt in &dispatch.receipts {
        assert_eq!(
            receipt.delivery_state,
            FanoutDeliveryStateClass::Undelivered
        );
        assert_eq!(
            receipt.stale_or_undelivered_reason,
            StaleUndeliveredReasonClass::ManagedEndpointBlocked
        );
        assert_eq!(
            receipt.summary_posture,
            FanoutSummaryPostureClass::NoSummary
        );
    }
    assert_eq!(dispatch.undelivered_count, GOVERNED_DESTINATIONS.len());
    // The authoritative record survives even when no copy is delivered.
    assert!(dispatch.durable_record_present);
}

#[test]
fn locked_screen_reduces_sensitive_copies_to_count_only() {
    // A workspace-sensitive source is reduced to a lock-screen-safe count-only affordance.
    let sensitive = source_named("ai.awaiting_approval");
    let dispatch = mint_dispatch(&sensitive, FanoutConditionClass::LockedScreen);
    for receipt in &dispatch.receipts {
        assert_eq!(receipt.delivery_state, FanoutDeliveryStateClass::Delivered);
        assert_eq!(
            receipt.summary_posture,
            FanoutSummaryPostureClass::LockScreenSafe
        );
        assert_eq!(
            receipt.applied_redaction,
            AttentionRedactionClass::CountOnly
        );
    }

    // A summary-safe source keeps a clear summary even on a locked screen.
    let routine = source_named("task.completed");
    let routine_dispatch = mint_dispatch(&routine, FanoutConditionClass::LockedScreen);
    for receipt in &routine_dispatch.receipts {
        assert_eq!(
            receipt.summary_posture,
            FanoutSummaryPostureClass::ClearSummary
        );
    }
}

#[test]
fn policy_withheld_is_suppressed_not_failed() {
    let source = source_named("incident.flagged");
    let dispatch = mint_dispatch(&source, FanoutConditionClass::PolicyWithheld);
    for receipt in &dispatch.receipts {
        assert_eq!(receipt.delivery_state, FanoutDeliveryStateClass::Suppressed);
        assert_eq!(receipt.resulting_state, AttentionStateClass::Suppressed);
        assert!(receipt.suppression_reason.is_named());
        // A suppression is not a delivery gap.
        assert!(!receipt.stale_or_undelivered_reason.is_named());
    }
    // Suppression is never counted as an undelivered transport failure.
    assert_eq!(dispatch.suppressed_count, GOVERNED_DESTINATIONS.len());
    assert_eq!(dispatch.undelivered_count, 0);
}

#[test]
fn unknown_transport_requires_review() {
    let source = source_named("task.completed");
    let dispatch = mint_dispatch(&source, FanoutConditionClass::TransportUnknown);
    let os = dispatch
        .receipt(FanoutChannelClass::OsNativeNotification)
        .expect("os receipt present");
    assert_eq!(os.delivery_state, FanoutDeliveryStateClass::Unknown);
    assert_eq!(
        os.resulting_state,
        AttentionStateClass::UnknownRequiresReview
    );
    assert_eq!(
        os.stale_or_undelivered_reason,
        StaleUndeliveredReasonClass::TransportIndeterminate
    );
    assert_eq!(dispatch.unknown_count, 1);
    // The unknown copy is not counted as delivered.
    assert_eq!(dispatch.delivered_count, GOVERNED_DESTINATIONS.len() - 1);
}

#[test]
fn preview_approval_sources_never_act_inline() {
    // An approval-gated source hands off; an ungated source may act inline.
    let gated = source_named("ai.awaiting_approval");
    assert!(gated.routes_through_preview_approval);
    let gated_dispatch = mint_dispatch(&gated, FanoutConditionClass::AllDelivered);
    for receipt in &gated_dispatch.receipts {
        assert!(receipt.routes_through_preview_approval);
        assert!(!receipt.inline_action_allowed);
    }

    let ungated = source_named("task.completed");
    assert!(!ungated.routes_through_preview_approval);
    let ungated_dispatch = mint_dispatch(&ungated, FanoutConditionClass::AllDelivered);
    for receipt in &ungated_dispatch.receipts {
        assert!(receipt.inline_action_allowed);
    }
}

#[test]
fn every_receipt_reopens_the_source_exact_object() {
    let bundle = bundle();
    for dispatch in &bundle.dispatches {
        let source = bundle
            .source(&dispatch.source_envelope_id)
            .expect("source present");
        for receipt in &dispatch.receipts {
            assert_eq!(receipt.reopen_target, source.reopen_target);
            assert_eq!(receipt.reopen_anchor_ref, source.reopen_anchor_ref);
            assert!(receipt.reopen_is_exact);
            assert!(is_export_safe_ref(&receipt.reopen_anchor_ref));
        }
    }
}

#[test]
fn privacy_is_never_widened_on_a_delivered_copy() {
    let bundle = bundle();
    for dispatch in &bundle.dispatches {
        let source = bundle
            .source(&dispatch.source_envelope_id)
            .expect("source present");
        for receipt in &dispatch.receipts {
            if matches!(
                receipt.delivery_state,
                FanoutDeliveryStateClass::Delivered | FanoutDeliveryStateClass::Stale
            ) {
                assert_ne!(
                    receipt.summary_posture,
                    FanoutSummaryPostureClass::NoSummary
                );
                assert!(
                    redaction_rank(receipt.applied_redaction)
                        >= redaction_rank(source.privacy_floor()),
                    "destination {} widened privacy",
                    receipt.destination.as_str()
                );
            }
        }
    }
}

#[test]
fn every_state_and_posture_is_exercised() {
    let bundle = bundle();
    for state in FanoutDeliveryStateClass::ALL {
        assert!(
            bundle
                .dispatches
                .iter()
                .any(|d| d.receipts.iter().any(|r| r.delivery_state == state)),
            "state {} exercised",
            state.as_str()
        );
    }
    for posture in FanoutSummaryPostureClass::ALL {
        assert!(
            bundle
                .dispatches
                .iter()
                .any(|d| d.receipts.iter().any(|r| r.summary_posture == posture)),
            "posture {} exercised",
            posture.as_str()
        );
    }
}

#[test]
fn bundle_round_trips_through_json() {
    let bundle = bundle();
    let json = serde_json::to_string(&bundle).expect("serializes");
    let back: FanoutReceiptsBundle = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(back, bundle);
}
