//! The single mint-from-truth source for the voice support-export packet, its
//! checked-in JSON/Markdown artifacts, and the redaction/export fixtures.
//!
//! Every id, ref, and label is stable so the artifacts and fixtures stay
//! byte-aligned with the in-crate builder, and the seeded packet validates by
//! construction.

use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::Path;

use super::{
    fixture_json, redact_transcript, AudioRetentionClass, ConfidenceCue, ProcessingLocality,
    RetentionMode, TranscriptExportPosture, TranscriptInclusionState, VoiceBlockedActionClass,
    VoicePolicyState, VoiceProviderClass, VoiceProviderDriftClass, VoiceSessionDiagnosticsRow,
    VoiceSessionFailureClass, VoiceSessionMode, VoiceSupportExportConsumerProjection,
    VoiceSupportExportGuardrails, VoiceSupportExportPacket, VoiceSupportExportPacketInput,
    VoiceTelemetryPosture, VoiceTranscriptExportDecision, VOICE_PROCESSING_AND_RETENTION_DOC_REF,
    VOICE_RETENTION_EXPORT_SCHEMA_REF, VOICE_SESSION_DIAGNOSTICS_ROW_RECORD_KIND,
    VOICE_SESSION_STATE_SCHEMA_REF, VOICE_SUPPORT_EXPORT_DOC_REF,
    VOICE_SUPPORT_EXPORT_FIXTURES_DIR_REF, VOICE_SUPPORT_EXPORT_PACKET_REF,
    VOICE_SUPPORT_EXPORT_REPORT_REF, VOICE_SUPPORT_EXPORT_SCHEMA_REF,
    VOICE_SUPPORT_EXPORT_SCHEMA_VERSION, VOICE_TELEMETRY_POSTURE_RECORD_KIND,
    VOICE_TRANSCRIPT_EXPORT_DECISION_RECORD_KIND,
};

/// Stable packet id minted by [`seeded_voice_support_export_packet`].
pub const SEED_VOICE_SUPPORT_EXPORT_PACKET_ID: &str = "voice-support-export:stable:0001";

/// Mint timestamp used by [`seeded_voice_support_export_packet`].
pub const SEED_VOICE_SUPPORT_EXPORT_MINTED_AT: &str = "2026-06-20T00:00:00Z";

/// Sample raw transcript used only transiently to mint the redaction summary for
/// the explicit-export fixture. It contains an email, a phone number, a token,
/// and an absolute path so the redactor exercises multiple classes. It is never
/// stored — only the content-free summary survives into the packet.
const SAMPLE_RAW_TRANSCRIPT: &str =
    "rename the function to processPayment and email me at jordan.doe@example.com or call \
     415-555-0148, the token is sk-AB12cd34EF56gh78ij90 and the config lives in \
     /Users/jordan/secret/config.env";

#[allow(clippy::too_many_arguments)]
fn row(
    session_id: &str,
    session_label: &str,
    mode: VoiceSessionMode,
    provider_class: VoiceProviderClass,
    processing_locality: ProcessingLocality,
    retention_mode: RetentionMode,
    audio_retention: AudioRetentionClass,
    transcript_export: TranscriptExportPosture,
    policy_state: VoicePolicyState,
    aggregate_confidence: Option<ConfidenceCue>,
    failure_class: Option<VoiceSessionFailureClass>,
    blocked_action_class: Option<VoiceBlockedActionClass>,
    provider_drift_class: VoiceProviderDriftClass,
) -> VoiceSessionDiagnosticsRow {
    VoiceSessionDiagnosticsRow {
        record_kind: VOICE_SESSION_DIAGNOSTICS_ROW_RECORD_KIND.to_owned(),
        schema_version: VOICE_SUPPORT_EXPORT_SCHEMA_VERSION,
        session_id: session_id.to_owned(),
        session_label: session_label.to_owned(),
        mode,
        provider_class,
        processing_locality,
        retention_mode,
        audio_retention,
        transcript_export,
        policy_state,
        aggregate_confidence,
        failure_class,
        blocked_action_class,
        provider_drift_class,
        keyboard_fallback_available: true,
        raw_audio_excluded: true,
        raw_transcript_excluded: true,
    }
}

/// Stable fixture file name for a session row (derived from its session id slug).
pub fn session_fixture_file_name(session_id: &str) -> String {
    let slug = session_id.split(':').nth(1).unwrap_or(session_id);
    format!("session-{slug}.json")
}

/// Stable fixture file name for a transcript export decision.
pub fn decision_fixture_file_name(decision: &VoiceTranscriptExportDecision) -> String {
    format!("export-{}.json", decision.inclusion_state.as_str())
}

fn seeded_sessions() -> Vec<VoiceSessionDiagnosticsRow> {
    vec![
        // Clean on-device dictation: nothing retained, nothing failed.
        row(
            "voice-session:local-dictation-clean:0001",
            "On-device dictation in the editor, nothing retained",
            VoiceSessionMode::DictationModeActive,
            VoiceProviderClass::OnDeviceLocal,
            ProcessingLocality::LocalOnDevice,
            RetentionMode::NoAudioNoTranscriptRetained,
            AudioRetentionClass::NoAudioRetained,
            TranscriptExportPosture::NoTranscriptExport,
            VoicePolicyState::UserControlled,
            Some(ConfidenceCue::High),
            None,
            None,
            VoiceProviderDriftClass::NoDriftObserved,
        ),
        // Hosted command session that held a high-impact command for confirmation.
        row(
            "voice-session:hosted-command-confirm-held:0001",
            "Disclosed hosted command session; a high-impact command was held for confirmation",
            VoiceSessionMode::CommandModeActive,
            VoiceProviderClass::ApprovedRemoteDisclosed,
            ProcessingLocality::HostedRemoteDisclosed,
            RetentionMode::TranscriptRetainedRedactedInSupportBundle,
            AudioRetentionClass::EphemeralAudioLocalOnly,
            TranscriptExportPosture::ExplicitUserExportRedacted,
            VoicePolicyState::EnterprisePolicyManaged,
            Some(ConfidenceCue::Medium),
            None,
            Some(VoiceBlockedActionClass::HighImpactCommandHeldForConfirmation),
            VoiceProviderDriftClass::NoDriftObserved,
        ),
        // Hosted provider unreachable, fell back to the local engine (drift + failure).
        row(
            "voice-session:provider-unreachable-fell-back-local:0001",
            "A hosted provider became unreachable; the session fell back to the on-device engine",
            VoiceSessionMode::CommandModeActive,
            VoiceProviderClass::OnDeviceLocal,
            ProcessingLocality::LocalOnDevice,
            RetentionMode::NoAudioNoTranscriptRetained,
            AudioRetentionClass::NoAudioRetained,
            TranscriptExportPosture::NoTranscriptExport,
            VoicePolicyState::UserControlled,
            Some(ConfidenceCue::Medium),
            Some(VoiceSessionFailureClass::HostedProviderUnreachableFellBackLocal),
            None,
            VoiceProviderDriftClass::ProviderDowngradedToLocal,
        ),
        // Recognition aborted on low confidence.
        row(
            "voice-session:low-confidence-aborted:0001",
            "Dictation aborted because recognition confidence stayed below the usable threshold",
            VoiceSessionMode::DictationModeActive,
            VoiceProviderClass::OnDeviceLocal,
            ProcessingLocality::LocalOnDevice,
            RetentionMode::EphemeralAudioLocalOnlyNoTranscriptRetained,
            AudioRetentionClass::EphemeralAudioLocalOnly,
            TranscriptExportPosture::NoTranscriptExport,
            VoicePolicyState::UserControlled,
            Some(ConfidenceCue::Low),
            Some(VoiceSessionFailureClass::RecognitionLowConfidenceAborted),
            None,
            VoiceProviderDriftClass::NoDriftObserved,
        ),
        // Policy blocks voice entirely; keyboard stays available.
        row(
            "voice-session:policy-blocked:0001",
            "Policy blocks voice in this context; the keyboard path stays available",
            VoiceSessionMode::VoiceModeBlockedByPolicy,
            VoiceProviderClass::ProviderDisabled,
            ProcessingLocality::ProcessingUnavailable,
            RetentionMode::RetentionBlockedByPolicy,
            AudioRetentionClass::AudioCaptureBlocked,
            TranscriptExportPosture::ExportBlockedByPolicy,
            VoicePolicyState::PolicyBlocked,
            None,
            Some(VoiceSessionFailureClass::PolicyBlockedCapture),
            Some(VoiceBlockedActionClass::ContinuousListeningBlockedByPolicy),
            VoiceProviderDriftClass::NoDriftObserved,
        ),
        // Enterprise relay with provider-contract retention — still raw-excluded
        // from our support bundle.
        row(
            "voice-session:enterprise-relay-managed:0001",
            "Enterprise relay session with provider-contract retention; raw content still stays out of support",
            VoiceSessionMode::CommandModeActive,
            VoiceProviderClass::EnterpriseRelayManaged,
            ProcessingLocality::HostedRemoteDisclosed,
            RetentionMode::TranscriptRetainedProviderPerContract,
            AudioRetentionClass::AudioRetainedProviderPerContract,
            TranscriptExportPosture::ProviderContractRetained,
            VoicePolicyState::EnterprisePolicyManaged,
            Some(ConfidenceCue::High),
            None,
            None,
            VoiceProviderDriftClass::NoDriftObserved,
        ),
        // Dictation into a surface that does not accept dictated text.
        row(
            "voice-session:dictation-target-unsupported:0001",
            "Dictation targeted a surface that does not accept dictated text; the action was held",
            VoiceSessionMode::DictationModeActive,
            VoiceProviderClass::OnDeviceLocal,
            ProcessingLocality::LocalOnDevice,
            RetentionMode::NoAudioNoTranscriptRetained,
            AudioRetentionClass::NoAudioRetained,
            TranscriptExportPosture::NoTranscriptExport,
            VoicePolicyState::UserControlled,
            Some(ConfidenceCue::Medium),
            None,
            Some(VoiceBlockedActionClass::DictationTargetSurfaceUnsupported),
            VoiceProviderDriftClass::NoDriftObserved,
        ),
    ]
}

fn decision(
    inclusion_state: TranscriptInclusionState,
    transcript_export_posture: TranscriptExportPosture,
    redaction_applied: bool,
    reviewed_by_user: bool,
    user_visible_label: &str,
    bounded_segment_count: u32,
    redaction_summary: Option<super::TranscriptRedactionSummary>,
) -> VoiceTranscriptExportDecision {
    VoiceTranscriptExportDecision {
        record_kind: VOICE_TRANSCRIPT_EXPORT_DECISION_RECORD_KIND.to_owned(),
        schema_version: VOICE_SUPPORT_EXPORT_SCHEMA_VERSION,
        inclusion_state,
        transcript_export_posture,
        redaction_applied,
        reviewed_by_user,
        user_visible_label: user_visible_label.to_owned(),
        bounded_segment_count,
        redaction_summary,
    }
}

fn seeded_decisions() -> Vec<VoiceTranscriptExportDecision> {
    // The redaction summary is minted from the sample raw transcript, which lives
    // only inside this function call and is dropped immediately. Only the
    // content-free summary survives.
    let redaction = redact_transcript(SAMPLE_RAW_TRANSCRIPT);

    vec![
        decision(
            TranscriptInclusionState::ExcludedByDefault,
            TranscriptExportPosture::MetadataOnlySupportExport,
            false,
            false,
            "Transcripts are excluded from support exports by default — only metadata classes are captured",
            0,
            None,
        ),
        decision(
            TranscriptInclusionState::RedactedIncludedAfterExplicitReview,
            TranscriptExportPosture::ExplicitUserExportRedacted,
            true,
            true,
            "User reviewed and exported 3 transcript segments with redaction applied",
            3,
            Some(redaction.summary),
        ),
        decision(
            TranscriptInclusionState::BlockedByPolicy,
            TranscriptExportPosture::ExportBlockedByPolicy,
            false,
            false,
            "Transcript export is blocked by policy in this context",
            0,
            None,
        ),
        decision(
            TranscriptInclusionState::NoTranscriptAvailable,
            TranscriptExportPosture::NoTranscriptExport,
            false,
            false,
            "No transcript was produced for this session",
            0,
            None,
        ),
    ]
}

fn seeded_telemetry_posture() -> VoiceTelemetryPosture {
    let mut captured: BTreeSet<String> = BTreeSet::new();
    for class in [
        "mode",
        "provider_class",
        "processing_locality",
        "retention_mode",
        "audio_retention",
        "transcript_export",
        "policy_state",
        "aggregate_confidence",
        "failure_class",
        "blocked_action_class",
        "provider_drift_class",
    ] {
        captured.insert(class.to_owned());
    }
    VoiceTelemetryPosture {
        record_kind: VOICE_TELEMETRY_POSTURE_RECORD_KIND.to_owned(),
        schema_version: VOICE_SUPPORT_EXPORT_SCHEMA_VERSION,
        raw_audio_in_telemetry: false,
        raw_transcript_in_telemetry: false,
        raw_audio_in_crash_packets: false,
        sensitive_transcript_in_logs: false,
        captured_metadata_classes: captured,
    }
}

/// The canonical, validating voice support-export packet that the checked-in
/// JSON/Markdown artifacts, the fixtures, and the conformance tests all share.
///
/// It covers a clean on-device dictation, a hosted command session that held a
/// high-impact command, a hosted-provider fallback to local (failure + drift), a
/// low-confidence abort, a policy block, an enterprise relay with provider
/// contract retention, and a dictation-into-unsupported-surface block — plus the
/// four transcript inclusion states and a no-audio-default telemetry posture.
pub fn seeded_voice_support_export_packet() -> VoiceSupportExportPacket {
    VoiceSupportExportPacket::new(VoiceSupportExportPacketInput {
        packet_id: SEED_VOICE_SUPPORT_EXPORT_PACKET_ID.to_owned(),
        label: "Voice Session Support Export & Redaction".to_owned(),
        sessions: seeded_sessions(),
        transcript_export_decisions: seeded_decisions(),
        telemetry_posture: seeded_telemetry_posture(),
        guardrails: VoiceSupportExportGuardrails {
            raw_audio_excluded_by_default: true,
            raw_transcript_excluded_by_default: true,
            transcript_export_explicit_and_redacted: true,
            failures_blocked_actions_and_drift_diagnosable: true,
            supportability_does_not_widen_retention: true,
            keyboard_fallback_always_available: true,
        },
        consumer_projection: VoiceSupportExportConsumerProjection {
            diagnostics_ingests: true,
            support_export_ingests: true,
            help_about_ingests: true,
            release_center_ingests: true,
            telemetry_schema_ingests: true,
        },
        source_contract_refs: vec![
            VOICE_SUPPORT_EXPORT_SCHEMA_REF.to_owned(),
            VOICE_SUPPORT_EXPORT_DOC_REF.to_owned(),
            VOICE_SESSION_STATE_SCHEMA_REF.to_owned(),
            VOICE_RETENTION_EXPORT_SCHEMA_REF.to_owned(),
            VOICE_PROCESSING_AND_RETENTION_DOC_REF.to_owned(),
            VOICE_SUPPORT_EXPORT_PACKET_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_VOICE_SUPPORT_EXPORT_MINTED_AT.to_owned(),
    })
}

/// Writes the checked-in support-export JSON artifact to `path`.
///
/// # Errors
///
/// Returns the IO error if the artifact cannot be written.
pub fn write_support_export(path: &Path, packet: &VoiceSupportExportPacket) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = fixture_json(packet).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    fs::write(path, json)
}

/// Writes the seeded packet, the per-session fixtures, the per-decision fixtures,
/// and the compact summary to `dir`. This is the single mint path the example
/// dump and the conformance test share, so the checked-in fixtures can never
/// drift silently from the in-crate builder.
///
/// # Errors
///
/// Returns the IO error if any fixture cannot be written.
pub fn write_fixtures(dir: &Path, packet: &VoiceSupportExportPacket) -> io::Result<()> {
    fs::create_dir_all(dir)?;

    let packet_json =
        fixture_json(packet).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    fs::write(dir.join("packet.json"), packet_json)?;

    for session in &packet.sessions {
        let json =
            fixture_json(session).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        fs::write(
            dir.join(session_fixture_file_name(&session.session_id)),
            json,
        )?;
    }

    for decision in &packet.transcript_export_decisions {
        let json =
            fixture_json(decision).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        fs::write(dir.join(decision_fixture_file_name(decision)), json)?;
    }

    let json = fixture_json(&packet.telemetry_posture)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    fs::write(dir.join("telemetry-posture.json"), json)?;

    let mut compact = packet.compact_lines().join("\n");
    compact.push('\n');
    fs::write(dir.join("compact.txt"), compact)?;

    Ok(())
}

/// Repo-relative fixtures dir, re-exported for the example dump and tests.
pub const FIXTURES_DIR_REF: &str = VOICE_SUPPORT_EXPORT_FIXTURES_DIR_REF;

/// Repo-relative support-export JSON artifact ref, re-exported for the example.
pub const SUPPORT_EXPORT_REF: &str = VOICE_SUPPORT_EXPORT_PACKET_REF;

/// Repo-relative support-export Markdown report ref, re-exported for the example.
pub const SUPPORT_REPORT_REF: &str = VOICE_SUPPORT_EXPORT_REPORT_REF;
