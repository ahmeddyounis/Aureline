use super::seed::seeded_voice_support_export_packet;
use super::*;

fn packet() -> VoiceSupportExportPacket {
    seeded_voice_support_export_packet()
}

#[test]
fn seeded_packet_validates() {
    let violations = packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_session_row_minimizes_data() {
    for row in &packet().sessions {
        assert!(
            row.data_minimization_held(),
            "row {} retained raw audio/transcript",
            row.session_id
        );
        assert!(row.is_consistent(), "row {} inconsistent", row.session_id);
        assert!(row.raw_audio_excluded && row.raw_transcript_excluded);
        assert!(row.keyboard_fallback_available);
    }
}

#[test]
fn telemetry_posture_excludes_raw_content() {
    let posture = packet().telemetry_posture;
    assert!(!posture.raw_audio_in_telemetry);
    assert!(!posture.raw_transcript_in_telemetry);
    assert!(!posture.raw_audio_in_crash_packets);
    assert!(!posture.sensitive_transcript_in_logs);
    assert!(posture.data_minimization_held());
    // Metadata is still captured so support can explain behavior.
    assert!(posture.captured_metadata_classes.contains("failure_class"));
    assert!(posture
        .captured_metadata_classes
        .contains("provider_drift_class"));
}

#[test]
fn diagnostics_cover_failure_blocked_and_drift() {
    let p = packet();
    assert!(p.failure_session_count() >= 1, "no failure case");
    assert!(
        p.blocked_action_session_count() >= 1,
        "no blocked-action case"
    );
    assert!(p.drift_session_count() >= 1, "no provider-drift case");
}

#[test]
fn transcript_export_default_is_exclusion() {
    let p = packet();
    let default = p
        .transcript_export_decisions
        .iter()
        .find(|d| {
            matches!(
                d.inclusion_state,
                TranscriptInclusionState::ExcludedByDefault
            )
        })
        .expect("default-exclusion decision present");
    assert!(!default.redaction_applied);
    assert_eq!(default.bounded_segment_count, 0);
    assert!(default.redaction_summary.is_none());
    assert!(default.is_consistent());
}

#[test]
fn explicit_transcript_export_is_reviewed_redacted_and_bounded() {
    let p = packet();
    let explicit = p
        .transcript_export_decisions
        .iter()
        .find(|d| {
            matches!(
                d.inclusion_state,
                TranscriptInclusionState::RedactedIncludedAfterExplicitReview
            )
        })
        .expect("explicit-export decision present");
    assert!(explicit.reviewed_by_user, "export must be user-reviewed");
    assert!(explicit.redaction_applied, "export must be redacted");
    assert!(explicit.bounded_segment_count > 0, "export must be bounded");
    assert!(
        !explicit.user_visible_label.trim().is_empty(),
        "export must be labeled"
    );
    let summary = explicit
        .redaction_summary
        .as_ref()
        .expect("redaction summary present");
    assert!(summary.is_well_formed());
    assert!(summary.residual_text_excluded_from_support);
    assert!(summary.redacted_span_count >= 1);
}

#[test]
fn redaction_masks_sensitive_classes_and_leaves_no_raw() {
    let raw = "email jordan.doe@example.com call 415-555-0148 token sk-AB12cd34EF56gh78ij90 \
               path /Users/jordan/secret/config.env host https://internal.example.com/x?y=1 \
               ip 10.0.12.34";
    let result = redact_transcript(raw);

    // None of the sensitive literals survive into the redacted text.
    for needle in [
        "jordan.doe@example.com",
        "sk-AB12cd34EF56gh78ij90",
        "/Users/jordan/secret/config.env",
        "https://internal.example.com/x?y=1",
        "10.0.12.34",
        "5550148",
    ] {
        assert!(
            !result.redacted_text.contains(needle),
            "redacted text still contains {needle}: {}",
            result.redacted_text
        );
    }

    // Every class we planted is recorded.
    for class in [
        RedactionClass::EmailAddress,
        RedactionClass::CredentialToken,
        RedactionClass::AbsolutePath,
        RedactionClass::Url,
        RedactionClass::IpAddress,
        RedactionClass::LongNumericSequence,
    ] {
        assert!(
            result.summary.classes_redacted.contains(&class),
            "missing class {}",
            class.as_str()
        );
    }
    assert!(result.summary.is_well_formed());
    // Non-sensitive words are preserved.
    assert!(result.redacted_text.contains("email"));
    assert!(result.redacted_text.contains("path"));
}

#[test]
fn redaction_summary_carries_no_text() {
    let result = redact_transcript("secret token sk-ABCDEFGH12345678ZZZZ here");
    let json = serde_json::to_string(&result.summary).expect("serialize");
    assert!(!json.contains("sk-ABCDEFGH12345678ZZZZ"));
    assert!(json.contains("credential_token"));
}

#[test]
fn raw_audio_retention_rejected() {
    let mut p = packet();
    p.sessions[0].raw_audio_excluded = false;
    let violations = p.validate();
    assert!(violations.contains(&VoiceSupportExportViolation::RawAudioNotExcluded));
}

#[test]
fn raw_transcript_retention_rejected() {
    let mut p = packet();
    p.sessions[0].raw_transcript_excluded = false;
    let violations = p.validate();
    assert!(violations.contains(&VoiceSupportExportViolation::RawTranscriptNotExcluded));
}

#[test]
fn telemetry_carrying_raw_audio_rejected() {
    let mut p = packet();
    p.telemetry_posture.raw_audio_in_telemetry = true;
    let violations = p.validate();
    assert!(violations.contains(&VoiceSupportExportViolation::TelemetryCarriesRawAudio));
}

#[test]
fn crash_packet_carrying_raw_audio_rejected() {
    let mut p = packet();
    p.telemetry_posture.raw_audio_in_crash_packets = true;
    let violations = p.validate();
    assert!(violations.contains(&VoiceSupportExportViolation::CrashPacketCarriesRawAudio));
}

#[test]
fn unreviewed_transcript_inclusion_rejected() {
    let mut p = packet();
    for d in &mut p.transcript_export_decisions {
        if matches!(
            d.inclusion_state,
            TranscriptInclusionState::RedactedIncludedAfterExplicitReview
        ) {
            d.reviewed_by_user = false;
        }
    }
    let violations = p.validate();
    assert!(violations.contains(&VoiceSupportExportViolation::TranscriptExportNotExplicit));
}

#[test]
fn unredacted_transcript_inclusion_rejected() {
    let mut p = packet();
    for d in &mut p.transcript_export_decisions {
        if matches!(
            d.inclusion_state,
            TranscriptInclusionState::RedactedIncludedAfterExplicitReview
        ) {
            d.redaction_applied = false;
        }
    }
    let violations = p.validate();
    assert!(violations.contains(&VoiceSupportExportViolation::TranscriptExportNotRedacted));
}

#[test]
fn dropping_explicit_export_case_is_flagged() {
    let mut p = packet();
    p.transcript_export_decisions.retain(|d| {
        !matches!(
            d.inclusion_state,
            TranscriptInclusionState::RedactedIncludedAfterExplicitReview
        )
    });
    let violations = p.validate();
    assert!(violations.contains(&VoiceSupportExportViolation::ExplicitRedactedExportCaseMissing));
}

#[test]
fn local_provider_with_provider_retained_audio_rejected() {
    let mut p = packet();
    // Force an inconsistent row: on-device but provider-retained audio.
    p.sessions[0].audio_retention = AudioRetentionClass::AudioRetainedProviderPerContract;
    let violations = p.validate();
    assert!(violations.contains(&VoiceSupportExportViolation::SessionRowIncomplete));
}

#[test]
fn json_round_trips() {
    let p = packet();
    let json = p.export_safe_json();
    let parsed: VoiceSupportExportPacket = serde_json::from_str(&json).expect("round-trip");
    assert_eq!(parsed, p);
}

#[test]
fn compact_lines_and_markdown_render() {
    let p = packet();
    let compact = p.compact_lines();
    assert_eq!(
        compact.len(),
        p.sessions.len() + p.transcript_export_decisions.len() + 1
    );
    let md = p.render_markdown();
    assert!(md.contains("Voice Session Support Export"));
    assert!(md.contains("Transcript export decisions"));
}
