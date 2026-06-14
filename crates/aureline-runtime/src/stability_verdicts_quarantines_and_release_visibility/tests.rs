use super::*;

use crate::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use crate::testing_identity::TestItemIdentityClass;

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn subject(subject_id: &str, node_kind: DurableTestNodeKind) -> VerdictSubject {
    VerdictSubject {
        subject_id: subject_id.to_owned(),
        node_kind,
        subject_fingerprint_token: format!("fingerprint:{subject_id}"),
        identity_class: TestItemIdentityClass::Stable,
    }
}

fn imported_subject(subject_id: &str) -> VerdictSubject {
    VerdictSubject {
        subject_id: subject_id.to_owned(),
        node_kind: DurableTestNodeKind::ConcreteInvocation,
        subject_fingerprint_token: format!("fingerprint:{subject_id}"),
        identity_class: TestItemIdentityClass::ImportedReadOnly,
    }
}

fn window(observed: u32, passed: u32, failed: u32, inconclusive: u32) -> EvidenceWindow {
    EvidenceWindow {
        observed_attempts: observed,
        passed_attempts: passed,
        failed_attempts: failed,
        inconclusive_attempts: inconclusive,
        first_attempt_ref: "attempt:first".to_owned(),
        latest_attempt_ref: "attempt:latest".to_owned(),
        evidence_attempt_refs: (1..=observed)
            .map(|index| format!("attempt:{index}"))
            .collect(),
        window_opened_at: "2026-06-01T00:00:00Z".to_owned(),
        window_closed_at: "2026-06-10T00:00:00Z".to_owned(),
    }
}

fn stable_verdict() -> StabilityVerdictRecord {
    StabilityVerdictRecord {
        verdict_id: "verdict:stable".to_owned(),
        subject: subject("test:stable", DurableTestNodeKind::ConcreteInvocation),
        session_ref: "session:stable".to_owned(),
        state: StabilityVerdictState::Stable,
        confidence: StabilityConfidenceClass::High,
        evidence_window: window(8, 8, 0, 0),
        evidence_provenance: VerdictEvidenceProvenance::LocalAuthoritative,
        owner_ref: "owner:a".to_owned(),
        imported: false,
        origin_provider_ref: None,
        quarantine_ref: None,
        release_visibility: ReleaseVisibilityClass::InformationalRecovered,
        readiness_impact: ReadinessImpactClass::NoImpact,
        evidence_refs: refs(&["evidence:stable"]),
        support_summary: "stable".to_owned(),
    }
}

fn flaky_verdict() -> StabilityVerdictRecord {
    StabilityVerdictRecord {
        verdict_id: "verdict:flaky".to_owned(),
        subject: subject("test:flaky[*]", DurableTestNodeKind::ParameterizedTemplate),
        session_ref: "session:flaky".to_owned(),
        state: StabilityVerdictState::ConfirmedFlaky,
        confidence: StabilityConfidenceClass::Moderate,
        evidence_window: window(10, 7, 3, 0),
        evidence_provenance: VerdictEvidenceProvenance::LocalAuthoritative,
        owner_ref: "owner:b".to_owned(),
        imported: false,
        origin_provider_ref: None,
        quarantine_ref: None,
        release_visibility: ReleaseVisibilityClass::ClaimNarrowingRequired,
        readiness_impact: ReadinessImpactClass::NarrowsClaim,
        evidence_refs: refs(&["evidence:flaky"]),
        support_summary: "flaky".to_owned(),
    }
}

fn quarantined_verdict() -> StabilityVerdictRecord {
    StabilityVerdictRecord {
        verdict_id: "verdict:quarantined".to_owned(),
        subject: subject("test:quarantined", DurableTestNodeKind::ConcreteInvocation),
        session_ref: "session:quarantined".to_owned(),
        state: StabilityVerdictState::Quarantined,
        confidence: StabilityConfidenceClass::Moderate,
        evidence_window: window(12, 5, 5, 2),
        evidence_provenance: VerdictEvidenceProvenance::LocalAuthoritative,
        owner_ref: "owner:c".to_owned(),
        imported: false,
        origin_provider_ref: None,
        quarantine_ref: Some("quarantine:q".to_owned()),
        release_visibility: ReleaseVisibilityClass::ReleaseBlocking,
        readiness_impact: ReadinessImpactClass::FailsReadiness,
        evidence_refs: refs(&["evidence:quarantined"]),
        support_summary: "quarantined".to_owned(),
    }
}

fn imported_verdict() -> StabilityVerdictRecord {
    StabilityVerdictRecord {
        verdict_id: "verdict:imported".to_owned(),
        subject: imported_subject("test:imported"),
        session_ref: "session:imported".to_owned(),
        state: StabilityVerdictState::ImportedOnlyUnverified,
        confidence: StabilityConfidenceClass::Low,
        evidence_window: window(3, 0, 0, 3),
        evidence_provenance: VerdictEvidenceProvenance::ImportedReadOnly,
        owner_ref: "owner:d".to_owned(),
        imported: true,
        origin_provider_ref: Some("provider:ci".to_owned()),
        quarantine_ref: None,
        release_visibility: ReleaseVisibilityClass::ClaimNarrowingRequired,
        readiness_impact: ReadinessImpactClass::NarrowsClaim,
        evidence_refs: refs(&["evidence:imported"]),
        support_summary: "imported".to_owned(),
    }
}

fn active_quarantine() -> QuarantineRecord {
    QuarantineRecord {
        quarantine_id: "quarantine:q".to_owned(),
        subject: subject("test:quarantined", DurableTestNodeKind::ConcreteInvocation),
        verdict_ref: "verdict:quarantined".to_owned(),
        treatment_kind: QuarantineTreatmentKind::Quarantine,
        state: QuarantineState::Active,
        reason: QuarantineReason::ReproducedFlaky,
        owner_ref: "owner:c".to_owned(),
        created_at: "2026-06-01T00:00:00Z".to_owned(),
        expires_at: "2026-12-01T00:00:00Z".to_owned(),
        restore_condition: RestoreConditionClass::StableWindowRequired,
        release_visibility: ReleaseVisibilityClass::ReleaseBlocking,
        readiness_impact: ReadinessImpactClass::FailsReadiness,
        imported: false,
        reopened_attempt_ref: None,
        evidence_refs: refs(&["evidence:quarantined"]),
        support_summary: "active".to_owned(),
    }
}

fn expired_quarantine() -> QuarantineRecord {
    QuarantineRecord {
        quarantine_id: "quarantine:expired".to_owned(),
        subject: subject("test:flaky[*]", DurableTestNodeKind::ParameterizedTemplate),
        verdict_ref: "verdict:flaky".to_owned(),
        treatment_kind: QuarantineTreatmentKind::Quarantine,
        state: QuarantineState::ExpiredReopened,
        reason: QuarantineReason::KnownFailing,
        owner_ref: "owner:b".to_owned(),
        created_at: "2026-01-01T00:00:00Z".to_owned(),
        expires_at: "2026-03-01T00:00:00Z".to_owned(),
        restore_condition: RestoreConditionClass::ManualReviewOnly,
        release_visibility: ReleaseVisibilityClass::ReleaseBlocking,
        readiness_impact: ReadinessImpactClass::FailsReadiness,
        imported: false,
        reopened_attempt_ref: Some("attempt:reopen".to_owned()),
        evidence_refs: refs(&["evidence:flaky"]),
        support_summary: "expired".to_owned(),
    }
}

fn guardrails() -> StabilityGuardrails {
    StabilityGuardrails {
        templates_distinct_from_invocations: true,
        imported_never_reads_as_local: true,
        flaky_badges_evidence_based: true,
        quarantines_visible_and_countable: true,
        expiry_fails_readiness: true,
        no_green_over_stale_or_quarantine: true,
    }
}

fn consumer_projection() -> StabilityConsumerProjection {
    StabilityConsumerProjection {
        flaky_badges_normalized: true,
        quarantine_ui_normalized: true,
        imported_join_normalized: true,
        release_support_export_normalized: true,
        readiness_gate_reads_packet: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        STABILITY_VERDICT_QUARANTINE_SCHEMA_REF,
        STABILITY_VERDICT_QUARANTINE_DOC_REF,
        STABILITY_VERDICT_QUARANTINE_ARTIFACT_REF,
    ])
}

fn valid_packet() -> StabilityVerdictQuarantinePacket {
    StabilityVerdictQuarantinePacket::new(StabilityVerdictQuarantinePacketInput {
        packet_id: "packet:test".to_owned(),
        label: "test".to_owned(),
        verdicts: vec![
            stable_verdict(),
            flaky_verdict(),
            quarantined_verdict(),
            imported_verdict(),
        ],
        quarantines: vec![active_quarantine(), expired_quarantine()],
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-13T00:00:00Z".to_owned(),
    })
}

#[test]
fn valid_packet_validates() {
    let packet = valid_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn export_round_trips() {
    let packet = valid_packet();
    let json = packet.export_safe_json();
    let parsed: StabilityVerdictQuarantinePacket = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, packet);
}

#[test]
fn coverage_requires_each_state_family() {
    let mut packet = valid_packet();
    packet
        .verdicts
        .retain(|v| v.state != StabilityVerdictState::ImportedOnlyUnverified);
    assert!(packet
        .validate()
        .contains(&StabilityVerdictQuarantineViolation::ImportedVerdictCaseMissing));
}

#[test]
fn templates_and_invocations_must_both_appear() {
    let mut packet = valid_packet();
    for verdict in &mut packet.verdicts {
        if verdict.subject.node_kind == DurableTestNodeKind::ParameterizedTemplate {
            verdict.subject.node_kind = DurableTestNodeKind::ConcreteInvocation;
        }
    }
    for quarantine in &mut packet.quarantines {
        if quarantine.subject.node_kind == DurableTestNodeKind::ParameterizedTemplate {
            quarantine.subject.node_kind = DurableTestNodeKind::ConcreteInvocation;
        }
    }
    assert!(packet
        .validate()
        .contains(&StabilityVerdictQuarantineViolation::TemplateCollapsedWithInvocation));
}

#[test]
fn imported_verdict_cannot_read_as_local() {
    let mut packet = valid_packet();
    // Force an imported verdict to drop its imported markers: it now reads local.
    let imported = packet
        .verdicts
        .iter_mut()
        .find(|v| v.verdict_id == "verdict:imported")
        .unwrap();
    imported.imported = false;
    assert!(packet
        .validate()
        .contains(&StabilityVerdictQuarantineViolation::ImportedVerdictReadsAsLocal));
}

#[test]
fn imported_verdict_can_never_be_stable() {
    let imported = imported_verdict();
    assert!(!imported.state.permits_no_readiness_impact());
    // An imported verdict that claims Stable is window/marker-inconsistent.
    let mut bad = imported;
    bad.state = StabilityVerdictState::Stable;
    assert!(!bad.imported_markers_consistent());
}

#[test]
fn green_over_stale_is_rejected() {
    let mut packet = valid_packet();
    // A flaky verdict claiming no readiness impact would hide debt behind green.
    let flaky = packet
        .verdicts
        .iter_mut()
        .find(|v| v.verdict_id == "verdict:flaky")
        .unwrap();
    flaky.readiness_impact = ReadinessImpactClass::NoImpact;
    assert!(packet
        .validate()
        .contains(&StabilityVerdictQuarantineViolation::GreenOverStaleOrQuarantine));
}

#[test]
fn state_must_match_evidence_window() {
    let mut packet = valid_packet();
    // A "stable" verdict with failures in its window is not evidence-based.
    let stable = packet
        .verdicts
        .iter_mut()
        .find(|v| v.verdict_id == "verdict:stable")
        .unwrap();
    stable.evidence_window = window(8, 6, 2, 0);
    assert!(packet
        .validate()
        .contains(&StabilityVerdictQuarantineViolation::VerdictStateWindowMismatch));
}

#[test]
fn quarantined_verdict_must_bind_resolvable_record() {
    let mut packet = valid_packet();
    let quarantined = packet
        .verdicts
        .iter_mut()
        .find(|v| v.verdict_id == "verdict:quarantined")
        .unwrap();
    quarantined.quarantine_ref = Some("quarantine:missing".to_owned());
    assert!(packet
        .validate()
        .contains(&StabilityVerdictQuarantineViolation::QuarantineBindingUnresolved));
}

#[test]
fn quarantine_must_reference_a_present_verdict() {
    let mut packet = valid_packet();
    packet.quarantines[0].verdict_ref = "verdict:missing".to_owned();
    assert!(packet
        .validate()
        .contains(&StabilityVerdictQuarantineViolation::QuarantineVerdictUnresolved));
}

#[test]
fn suppressing_quarantine_cannot_be_silently_muted() {
    let mut packet = valid_packet();
    // An active record dropping to no readiness impact + recovered visibility is a
    // silent mute hiding debt behind green.
    packet.quarantines[0].readiness_impact = ReadinessImpactClass::NoImpact;
    packet.quarantines[0].release_visibility = ReleaseVisibilityClass::InformationalRecovered;
    let violations = packet.validate();
    assert!(violations.contains(&StabilityVerdictQuarantineViolation::QuarantineSilentlyMuted));
}

#[test]
fn coverage_requires_an_expired_quarantine() {
    let mut packet = valid_packet();
    packet
        .quarantines
        .retain(|q| q.state != QuarantineState::ExpiredReopened);
    assert!(packet
        .validate()
        .contains(&StabilityVerdictQuarantineViolation::ExpiredQuarantineCaseMissing));
}

#[test]
fn evaluated_at_flips_expired_active_record() {
    let active = active_quarantine();
    assert_eq!(active.state, QuarantineState::Active);

    let reopened = active.evaluated_at("2027-01-01T00:00:00Z", Some("attempt:x".to_owned()));
    assert_eq!(reopened.state, QuarantineState::ExpiredReopened);
    assert_eq!(
        reopened.readiness_impact,
        ReadinessImpactClass::FailsReadiness
    );
    assert_eq!(
        reopened.release_visibility,
        ReleaseVisibilityClass::ReleaseBlocking
    );
    assert_eq!(reopened.reopened_attempt_ref.as_deref(), Some("attempt:x"));
    assert!(reopened.expiry_markers_consistent());

    // A record still inside its window is unchanged.
    let still_active = active.evaluated_at("2026-06-15T00:00:00Z", None);
    assert_eq!(still_active.state, QuarantineState::Active);
}

#[test]
fn readiness_gate_reflects_failing_rows() {
    let packet = valid_packet();
    assert!(packet.readiness_blocked());
    assert_eq!(packet.expired_reopened_count(), 1);
    assert_eq!(packet.active_quarantine_count(), 1);
    assert_eq!(packet.imported_verdict_count(), 1);
}

#[test]
fn fingerprint_cannot_substitute_for_id() {
    let mut packet = valid_packet();
    let verdict = &mut packet.verdicts[0];
    verdict.subject.subject_fingerprint_token = verdict.subject.subject_id.clone();
    assert!(packet
        .validate()
        .contains(&StabilityVerdictQuarantineViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn missing_source_contract_is_flagged() {
    let mut packet = valid_packet();
    packet.source_contract_refs = refs(&[STABILITY_VERDICT_QUARANTINE_SCHEMA_REF]);
    assert!(packet
        .validate()
        .contains(&StabilityVerdictQuarantineViolation::MissingSourceContracts));
}

#[test]
fn markdown_summary_renders_rows() {
    let packet = valid_packet();
    let summary = packet.render_markdown_summary();
    assert!(summary.contains("Stability Verdicts And Quarantine Records"));
    assert!(summary.contains("verdict:quarantined"));
    assert!(summary.contains("quarantine:expired"));
}
