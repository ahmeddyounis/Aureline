//! Conformance dump for the M5 stability-verdict / quarantine packet.
//!
//! Prints the canonical support export (default), the Markdown summary
//! (`summary` argument), or the expiry-drill fixture (`fixture` argument) so the
//! checked-in artifacts stay byte-aligned with the in-crate builder.

use aureline_runtime::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use aureline_runtime::stability_verdicts_quarantines_and_release_visibility::*;
use aureline_runtime::testing_identity::TestItemIdentityClass;

const PACKET_ID: &str = "stability-verdict-quarantine:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn subject(
    subject_id: &str,
    node_kind: DurableTestNodeKind,
    identity_class: TestItemIdentityClass,
) -> VerdictSubject {
    VerdictSubject {
        subject_id: subject_id.to_owned(),
        node_kind,
        subject_fingerprint_token: format!("fingerprint:{subject_id}"),
        identity_class,
    }
}

fn window(
    observed: u32,
    passed: u32,
    failed: u32,
    inconclusive: u32,
    first: &str,
    latest: &str,
) -> EvidenceWindow {
    EvidenceWindow {
        observed_attempts: observed,
        passed_attempts: passed,
        failed_attempts: failed,
        inconclusive_attempts: inconclusive,
        first_attempt_ref: first.to_owned(),
        latest_attempt_ref: latest.to_owned(),
        evidence_attempt_refs: (1..=observed)
            .map(|index| format!("{first}:obs:{index}"))
            .collect(),
        window_opened_at: "2026-06-01T00:00:00Z".to_owned(),
        window_closed_at: "2026-06-12T00:00:00Z".to_owned(),
    }
}

fn stable_verdict() -> StabilityVerdictRecord {
    StabilityVerdictRecord {
        verdict_id: "verdict:stable:auth".to_owned(),
        subject: subject(
            "test:auth::login_ok",
            DurableTestNodeKind::ConcreteInvocation,
            TestItemIdentityClass::Stable,
        ),
        session_ref: "session:local:auth".to_owned(),
        state: StabilityVerdictState::Stable,
        confidence: StabilityConfidenceClass::High,
        evidence_window: window(8, 8, 0, 0, "attempt:auth", "attempt:auth:8"),
        evidence_provenance: VerdictEvidenceProvenance::LocalAuthoritative,
        owner_ref: "owner:auth-team".to_owned(),
        imported: false,
        origin_provider_ref: None,
        quarantine_ref: None,
        release_visibility: ReleaseVisibilityClass::InformationalRecovered,
        readiness_impact: ReadinessImpactClass::NoImpact,
        evidence_refs: refs(&["evidence:auth:window"]),
        support_summary: "Login suite stable across eight clean local attempts.".to_owned(),
    }
}

fn flaky_verdict() -> StabilityVerdictRecord {
    StabilityVerdictRecord {
        verdict_id: "verdict:flaky:checkout".to_owned(),
        subject: subject(
            "test:checkout::pay[*]",
            DurableTestNodeKind::ParameterizedTemplate,
            TestItemIdentityClass::Stable,
        ),
        session_ref: "session:local:checkout".to_owned(),
        state: StabilityVerdictState::ConfirmedFlaky,
        confidence: StabilityConfidenceClass::Moderate,
        evidence_window: window(10, 7, 3, 0, "attempt:checkout", "attempt:checkout:10"),
        evidence_provenance: VerdictEvidenceProvenance::LocalAuthoritative,
        owner_ref: "owner:checkout-team".to_owned(),
        imported: false,
        origin_provider_ref: None,
        quarantine_ref: None,
        release_visibility: ReleaseVisibilityClass::ClaimNarrowingRequired,
        readiness_impact: ReadinessImpactClass::NarrowsClaim,
        evidence_refs: refs(&["evidence:checkout:window"]),
        support_summary: "Checkout template intermittently fails (3/10); claim narrowed."
            .to_owned(),
    }
}

fn quarantined_integration_verdict() -> StabilityVerdictRecord {
    StabilityVerdictRecord {
        verdict_id: "verdict:quarantined:integration".to_owned(),
        subject: subject(
            "test:integration::sync[case-3]",
            DurableTestNodeKind::ConcreteInvocation,
            TestItemIdentityClass::Stable,
        ),
        session_ref: "session:remote:integration".to_owned(),
        state: StabilityVerdictState::Quarantined,
        confidence: StabilityConfidenceClass::Moderate,
        evidence_window: window(12, 5, 5, 2, "attempt:integration", "attempt:integration:12"),
        evidence_provenance: VerdictEvidenceProvenance::RemoteAuthoritative,
        owner_ref: "owner:integration-team".to_owned(),
        imported: false,
        origin_provider_ref: None,
        quarantine_ref: Some("quarantine:active:integration".to_owned()),
        release_visibility: ReleaseVisibilityClass::ReleaseBlocking,
        readiness_impact: ReadinessImpactClass::FailsReadiness,
        evidence_refs: refs(&["evidence:integration:window"]),
        support_summary: "Integration case quarantined under an active record.".to_owned(),
    }
}

fn quarantined_legacy_verdict() -> StabilityVerdictRecord {
    StabilityVerdictRecord {
        verdict_id: "verdict:quarantined:legacy".to_owned(),
        subject: subject(
            "test:legacy::migrate",
            DurableTestNodeKind::ConcreteInvocation,
            TestItemIdentityClass::Stable,
        ),
        session_ref: "session:local:legacy".to_owned(),
        state: StabilityVerdictState::Quarantined,
        confidence: StabilityConfidenceClass::Low,
        evidence_window: window(9, 2, 6, 1, "attempt:legacy", "attempt:legacy:9"),
        evidence_provenance: VerdictEvidenceProvenance::LocalAuthoritative,
        owner_ref: "owner:platform-team".to_owned(),
        imported: false,
        origin_provider_ref: None,
        quarantine_ref: Some("quarantine:expired:legacy".to_owned()),
        release_visibility: ReleaseVisibilityClass::ReleaseBlocking,
        readiness_impact: ReadinessImpactClass::FailsReadiness,
        evidence_refs: refs(&["evidence:legacy:window"]),
        support_summary: "Legacy migration quarantine expired and reopened for review.".to_owned(),
    }
}

fn imported_verdict() -> StabilityVerdictRecord {
    StabilityVerdictRecord {
        verdict_id: "verdict:imported:smoke".to_owned(),
        subject: subject(
            "test:imported::smoke",
            DurableTestNodeKind::ConcreteInvocation,
            TestItemIdentityClass::ImportedReadOnly,
        ),
        session_ref: "session:imported:smoke".to_owned(),
        state: StabilityVerdictState::ImportedOnlyUnverified,
        confidence: StabilityConfidenceClass::Low,
        evidence_window: window(3, 0, 0, 3, "attempt:imported", "attempt:imported:3"),
        evidence_provenance: VerdictEvidenceProvenance::ImportedReadOnly,
        owner_ref: "owner:release-eng".to_owned(),
        imported: true,
        origin_provider_ref: Some("provider:ci-smoke".to_owned()),
        quarantine_ref: None,
        release_visibility: ReleaseVisibilityClass::ClaimNarrowingRequired,
        readiness_impact: ReadinessImpactClass::NarrowsClaim,
        evidence_refs: refs(&["evidence:imported:smoke"]),
        support_summary: "Imported CI smoke evidence; not locally verified, claim narrowed."
            .to_owned(),
    }
}

fn stale_verdict() -> StabilityVerdictRecord {
    StabilityVerdictRecord {
        verdict_id: "verdict:stale:nightly".to_owned(),
        subject: subject(
            "test:nightly::soak",
            DurableTestNodeKind::ConcreteInvocation,
            TestItemIdentityClass::Stable,
        ),
        session_ref: "session:local:nightly".to_owned(),
        state: StabilityVerdictState::StaleEvidence,
        confidence: StabilityConfidenceClass::InsufficientEvidence,
        evidence_window: window(4, 4, 0, 0, "attempt:nightly", "attempt:nightly:4"),
        evidence_provenance: VerdictEvidenceProvenance::LocalAuthoritative,
        owner_ref: "owner:qa-team".to_owned(),
        imported: false,
        origin_provider_ref: None,
        quarantine_ref: None,
        release_visibility: ReleaseVisibilityClass::ClaimNarrowingRequired,
        readiness_impact: ReadinessImpactClass::NarrowsClaim,
        evidence_refs: refs(&["evidence:nightly:window"]),
        support_summary: "Nightly soak evidence is stale; claim narrowed pending a fresh run."
            .to_owned(),
    }
}

fn active_integration_quarantine() -> QuarantineRecord {
    QuarantineRecord {
        quarantine_id: "quarantine:active:integration".to_owned(),
        subject: subject(
            "test:integration::sync[case-3]",
            DurableTestNodeKind::ConcreteInvocation,
            TestItemIdentityClass::Stable,
        ),
        verdict_ref: "verdict:quarantined:integration".to_owned(),
        treatment_kind: QuarantineTreatmentKind::Quarantine,
        state: QuarantineState::Active,
        reason: QuarantineReason::ReproducedFlaky,
        owner_ref: "owner:integration-team".to_owned(),
        created_at: "2026-06-01T00:00:00Z".to_owned(),
        expires_at: "2026-12-01T00:00:00Z".to_owned(),
        restore_condition: RestoreConditionClass::StableWindowRequired,
        release_visibility: ReleaseVisibilityClass::ReleaseBlocking,
        readiness_impact: ReadinessImpactClass::FailsReadiness,
        imported: false,
        reopened_attempt_ref: None,
        evidence_refs: refs(&["evidence:integration:window"]),
        support_summary: "Active quarantine; restore needs a stable window.".to_owned(),
    }
}

fn expired_legacy_quarantine() -> QuarantineRecord {
    QuarantineRecord {
        quarantine_id: "quarantine:expired:legacy".to_owned(),
        subject: subject(
            "test:legacy::migrate",
            DurableTestNodeKind::ConcreteInvocation,
            TestItemIdentityClass::Stable,
        ),
        verdict_ref: "verdict:quarantined:legacy".to_owned(),
        treatment_kind: QuarantineTreatmentKind::Quarantine,
        state: QuarantineState::ExpiredReopened,
        reason: QuarantineReason::KnownFailing,
        owner_ref: "owner:platform-team".to_owned(),
        created_at: "2026-01-01T00:00:00Z".to_owned(),
        expires_at: "2026-03-01T00:00:00Z".to_owned(),
        restore_condition: RestoreConditionClass::ManualReviewOnly,
        release_visibility: ReleaseVisibilityClass::ReleaseBlocking,
        readiness_impact: ReadinessImpactClass::FailsReadiness,
        imported: false,
        reopened_attempt_ref: Some("attempt:legacy:reopen".to_owned()),
        evidence_refs: refs(&["evidence:legacy:window"]),
        support_summary: "Expired quarantine reopened for owner review.".to_owned(),
    }
}

fn imported_mute_record() -> QuarantineRecord {
    QuarantineRecord {
        quarantine_id: "quarantine:mute:imported".to_owned(),
        subject: subject(
            "test:imported::smoke",
            DurableTestNodeKind::ConcreteInvocation,
            TestItemIdentityClass::ImportedReadOnly,
        ),
        verdict_ref: "verdict:imported:smoke".to_owned(),
        treatment_kind: QuarantineTreatmentKind::Mute,
        state: QuarantineState::Active,
        reason: QuarantineReason::ImportedIncomparable,
        owner_ref: "owner:release-eng".to_owned(),
        created_at: "2026-06-01T00:00:00Z".to_owned(),
        expires_at: "2026-09-01T00:00:00Z".to_owned(),
        restore_condition: RestoreConditionClass::OwnerSignOff,
        release_visibility: ReleaseVisibilityClass::ClaimNarrowingRequired,
        readiness_impact: ReadinessImpactClass::NarrowsClaim,
        imported: true,
        reopened_attempt_ref: None,
        evidence_refs: refs(&["evidence:imported:smoke"]),
        support_summary: "Imported-incomparable mute; stays visible as narrowing debt.".to_owned(),
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
        STABILITY_VERDICT_QUARANTINE_SUMMARY_REF,
    ])
}

fn packet() -> StabilityVerdictQuarantinePacket {
    StabilityVerdictQuarantinePacket::new(StabilityVerdictQuarantinePacketInput {
        packet_id: PACKET_ID.to_owned(),
        label: "M5 Stability Verdicts And Quarantine Records".to_owned(),
        verdicts: vec![
            stable_verdict(),
            flaky_verdict(),
            quarantined_integration_verdict(),
            quarantined_legacy_verdict(),
            imported_verdict(),
            stale_verdict(),
        ],
        quarantines: vec![
            active_integration_quarantine(),
            expired_legacy_quarantine(),
            imported_mute_record(),
        ],
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

/// Builds the expiry-drill fixture: an active quarantine evaluated past its expiry
/// flips to an expired-reopened record that fails readiness, proving an expired
/// quarantine never silently persists as local state.
fn expiry_drill_fixture() -> StabilityVerdictQuarantinePacket {
    let evaluated = active_integration_quarantine().evaluated_at(
        "2027-01-01T00:00:00Z",
        Some("attempt:integration:reopen".to_owned()),
    );

    StabilityVerdictQuarantinePacket::new(StabilityVerdictQuarantinePacketInput {
        packet_id: "stability-verdict-quarantine:fixture:expiry-drill".to_owned(),
        label: "Expired quarantine reopens and fails readiness".to_owned(),
        verdicts: vec![
            stable_verdict(),
            flaky_verdict(),
            quarantined_integration_verdict(),
            imported_verdict(),
        ],
        quarantines: vec![evaluated],
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());

    let packet = if which == "fixture" {
        expiry_drill_fixture()
    } else {
        packet()
    };

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
