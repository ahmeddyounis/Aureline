use aureline_runtime::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use aureline_runtime::stability_verdicts_quarantines_and_release_visibility::{
    current_stability_verdict_quarantine_export, QuarantineState, ReadinessImpactClass,
    ReleaseVisibilityClass, StabilityVerdictQuarantinePacket, StabilityVerdictState,
    VerdictEvidenceProvenance,
};

fn fixture(name: &str) -> StabilityVerdictQuarantinePacket {
    let path = format!(
        "{}/../../fixtures/testing/m5/stability-verdicts-quarantines-and-release-visibility/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_stability_verdict_quarantine_export()
        .expect("checked-in stability verdict quarantine export should validate");
    assert!(packet.validate().is_empty());

    // The badge vocabulary is exercised, not merely declared.
    for state in [
        StabilityVerdictState::Stable,
        StabilityVerdictState::ConfirmedFlaky,
        StabilityVerdictState::Quarantined,
        StabilityVerdictState::ImportedOnlyUnverified,
    ] {
        assert!(
            packet.represented_verdict_states().contains(&state),
            "missing verdict state {}",
            state.as_str()
        );
    }
}

#[test]
fn template_and_invocation_identities_stay_distinct() {
    let packet = current_stability_verdict_quarantine_export().expect("export validates");
    let kinds = packet.represented_subject_kinds();
    assert!(kinds.contains(&DurableTestNodeKind::ParameterizedTemplate));
    assert!(kinds.contains(&DurableTestNodeKind::ConcreteInvocation));
}

#[test]
fn imported_verdict_never_reads_as_a_locally_verified_pass() {
    let packet = current_stability_verdict_quarantine_export().expect("export validates");
    let imported = packet
        .verdicts
        .iter()
        .find(|v| v.imported)
        .expect("an imported verdict");
    assert_eq!(
        imported.state,
        StabilityVerdictState::ImportedOnlyUnverified
    );
    assert_eq!(
        imported.evidence_provenance,
        VerdictEvidenceProvenance::ImportedReadOnly
    );
    assert!(imported.origin_provider_ref.is_some());
    assert_ne!(imported.readiness_impact, ReadinessImpactClass::NoImpact);
}

#[test]
fn quarantines_stay_visible_and_tied_to_owners_and_expiry() {
    let packet = current_stability_verdict_quarantine_export().expect("export validates");
    assert!(!packet.quarantines.is_empty());
    for quarantine in &packet.quarantines {
        assert!(!quarantine.owner_ref.trim().is_empty());
        assert!(!quarantine.expires_at.trim().is_empty());
        assert!(quarantine.release_visibility.is_visible_debt());
        if quarantine.state.is_suppressing() {
            assert_ne!(quarantine.readiness_impact, ReadinessImpactClass::NoImpact);
        }
    }
    // An expired quarantine reopened its scope and blocks readiness.
    let expired = packet
        .quarantines
        .iter()
        .find(|q| q.state == QuarantineState::ExpiredReopened)
        .expect("an expired-reopened quarantine");
    assert_eq!(
        expired.readiness_impact,
        ReadinessImpactClass::FailsReadiness
    );
    assert_eq!(
        expired.release_visibility,
        ReleaseVisibilityClass::ReleaseBlocking
    );
    assert!(expired.reopened_attempt_ref.is_some());
}

#[test]
fn release_readiness_gate_reads_the_same_truth() {
    let packet = current_stability_verdict_quarantine_export().expect("export validates");
    // Quarantined and expired rows fail readiness; only stable rolls up green.
    assert!(packet.readiness_blocked());
    assert!(packet.fails_readiness_count() >= 1);
}

#[test]
fn fixture_expired_quarantine_reopens_and_fails_readiness() {
    let packet = fixture("expired_quarantine_reopens_and_fails_readiness.json");
    assert!(packet.validate().is_empty());

    let reopened = packet
        .quarantines
        .iter()
        .find(|q| q.state == QuarantineState::ExpiredReopened)
        .expect("an expired-reopened quarantine");
    assert_eq!(
        reopened.readiness_impact,
        ReadinessImpactClass::FailsReadiness
    );
    assert!(reopened.reopened_attempt_ref.is_some());
    assert!(packet.readiness_blocked());
}
