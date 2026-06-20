//! Unit tests for the reactive-diagnostics packet, fixtures, and export.

use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_reactive_diagnostics_packet();
    validate_reactive_diagnostics_packet(&packet).expect("seeded packet must validate");
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_reactive_diagnostics_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let parsed: ReactiveDiagnosticsPacket =
        serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn required_reason_codes_have_probes() {
    let packet = seeded_reactive_diagnostics_packet();
    for required in ReactiveStateReasonCode::required_named() {
        assert!(
            packet
                .doctor_probes
                .iter()
                .any(|probe| probe.reason_code == required),
            "missing doctor probe for required reason code {}",
            required.as_token()
        );
    }
}

#[test]
fn every_reason_code_is_probed() {
    let packet = seeded_reactive_diagnostics_packet();
    for code in ReactiveStateReasonCode::all() {
        assert!(
            packet
                .doctor_probes
                .iter()
                .any(|probe| probe.reason_code == code),
            "missing doctor probe for reason code {}",
            code.as_token()
        );
    }
}

#[test]
fn no_slow_consumer_offers_exact_truth_while_behind() {
    let packet = seeded_reactive_diagnostics_packet();
    for row in &packet.slow_consumers {
        assert!(
            row.honors_truth_gate(),
            "slow consumer {} broke the truth gate",
            row.consumer_surface.as_str()
        );
        if !row.epoch_posture.is_current() {
            assert!(!row.offers_exact_truth_action);
            assert!(!row.silent_retry_allowed);
        }
    }
}

#[test]
fn provider_unavailable_consumer_is_blocked() {
    let packet = seeded_reactive_diagnostics_packet();
    let row = packet
        .slow_consumers
        .iter()
        .find(|row| row.reason_code == ReactiveStateReasonCode::ProviderOverlayUnavailable)
        .expect("provider-unavailable slow consumer exists");
    assert_eq!(row.action_posture, ActionPosture::Blocked);
    assert_eq!(row.epoch_posture, EpochPosture::StaleEpoch);
}

#[test]
fn epoch_drift_flag_matches_epochs() {
    let packet = seeded_reactive_diagnostics_packet();
    for row in &packet.active_subscriptions {
        assert_eq!(
            row.epoch_drift,
            row.snapshot_epoch < row.authority_epoch,
            "subscription {} epoch_drift flag is inconsistent",
            row.subscription_id
        );
        assert_ne!(
            row.truth_claim,
            TruthClaim::ExactCurrentTruth,
            "derived subscription {} must not claim exact current truth",
            row.subscription_id
        );
    }
}

#[test]
fn stale_materializations_are_never_authoritative() {
    let packet = seeded_reactive_diagnostics_packet();
    assert!(!packet.stale_materializations.is_empty());
    for row in &packet.stale_materializations {
        assert!(
            row.is_stale(),
            "materialization {} is marked stale but authoritative",
            row.binding_id
        );
    }
}

#[test]
fn invalidation_history_is_ordered_and_forward() {
    let packet = seeded_reactive_diagnostics_packet();
    let mut last = None;
    for row in &packet.invalidation_history {
        if let Some(prev) = last {
            assert!(
                row.sequence > prev,
                "invalidation history is not increasing"
            );
        }
        last = Some(row.sequence);
        assert!(
            row.to_epoch >= row.from_epoch,
            "invalidation rolled epoch backward"
        );
    }
}

#[test]
fn fixtures_validate_against_packet() {
    let packet = seeded_reactive_diagnostics_packet();
    let fixtures = seeded_reactive_diagnostics_fixtures();
    assert_eq!(fixtures.len(), 8);
    for fixture in &fixtures {
        validate_reactive_diagnostics_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn reason_code_finding_codes_are_stable() {
    assert_eq!(
        ReactiveStateReasonCode::ConsumerStale.finding_code(),
        "reactive.consumer_stale"
    );
    assert_eq!(
        ReactiveStateReasonCode::ProviderOverlayUnavailable.finding_code(),
        "reactive.provider_overlay_unavailable"
    );
}

#[test]
fn support_export_is_metadata_safe() {
    let envelope = compile_support_export_envelope(
        "envelope:reactive_diagnostics:test",
        "2026-06-19T09:00:00Z",
    )
    .expect("support export compiles");
    assert!(envelope.is_export_safe());
    assert_eq!(envelope.rows.len(), 8);

    // Rows are sorted by finding code for deterministic review.
    let mut sorted = envelope.rows.clone();
    sorted.sort_by(|a, b| a.finding_code.cmp(&b.finding_code));
    assert_eq!(sorted, envelope.rows);

    let json = serde_json::to_string(&envelope).expect("envelope serializes");
    let parsed: ReactiveDiagnosticsSupportExportEnvelope =
        serde_json::from_str(&json).expect("envelope round-trips");
    assert_eq!(parsed, envelope);
}

#[test]
fn support_export_preserves_product_vocabulary() {
    // Every export row's finding code must match the product reason-code
    // vocabulary so support and product never diverge.
    let envelope = compile_support_export_envelope(
        "envelope:reactive_diagnostics:vocab",
        "2026-06-19T09:05:00Z",
    )
    .expect("support export compiles");
    for row in &envelope.rows {
        assert_eq!(row.finding_code, row.reason_code.finding_code());
        assert_eq!(row.severity, row.reason_code.severity());
        assert_eq!(row.safe_next_step, row.reason_code.safe_next_step());
    }
}
