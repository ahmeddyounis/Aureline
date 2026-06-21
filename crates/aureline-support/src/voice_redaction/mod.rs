//! Support-safe transcript/audio boundaries, no-audio-default telemetry and
//! support posture, redacted transcript export, and voice-session diagnostics
//! for claimed voice profiles.
//!
//! Voice is an explicit, privacy-bounded input mode. When something goes wrong
//! in a voice session — a hosted provider falls back to the local engine, a
//! high-impact command is held for confirmation, recognition aborts on low
//! confidence — support needs to *explain* what happened without a hidden debug
//! switch and without ever ingesting raw audio or raw transcript text.
//!
//! This module is the support-side boundary for that. It owns:
//!
//! - [`VoiceSessionDiagnosticsRow`] — one metadata-only row per voice session.
//!   It carries the session's mode, provider class, processing locality,
//!   retention/audio/export postures, policy state, an aggregate recognition
//!   confidence *class* (never content), and the typed failure, blocked-action,
//!   and provider-drift classes that let support narrate the session. Both
//!   `raw_audio_excluded` and `raw_transcript_excluded` are pinned true — raw
//!   audio bytes and raw transcript text never enter this row.
//! - [`VoiceTranscriptExportDecision`] — the explicit, reviewed transcript
//!   export decision. Transcript text is excluded by default; the only path that
//!   includes any is an explicit, user-reviewed, redacted, bounded export with a
//!   user-visible label. Even then the row carries only a
//!   [`TranscriptRedactionSummary`] (counts and class tokens), never the text.
//! - [`VoiceTelemetryPosture`] — the no-audio-default telemetry/crash/log
//!   posture: raw audio and sensitive transcript content stay out of telemetry,
//!   crash packets, and logs; only metadata class tokens are captured.
//! - [`redact_transcript`] — the redaction primitive used at the explicit export
//!   moment. It masks emails, long numeric sequences, absolute paths, URLs,
//!   IP addresses, and credential-like tokens, and returns the redacted text
//!   alongside a content-free [`TranscriptRedactionSummary`].
//! - [`VoiceSupportExportPacket`] — the top-level packet that folds the
//!   diagnostics rows, the export decisions, the telemetry posture, the
//!   guardrails, and the consumer projection into one inspectable, export-safe
//!   record. [`VoiceSupportExportPacket::validate`] refuses any packet that
//!   retains raw audio/transcript by default, exports transcripts without an
//!   explicit redacted review, lets telemetry/crash/logs carry sensitive
//!   content, or lets supportability convenience widen retention.
//!
//! This crate cannot depend on the shell where the live voice state lives, so
//! the class tokens here mirror the canonical boundary schemas
//! [`schemas/voice/voice-session.schema.json`](../../../../schemas/voice/voice-session.schema.json)
//! and
//! [`schemas/voice/retention-and-export.schema.json`](../../../../schemas/voice/retention-and-export.schema.json)
//! exactly, so Help/About, diagnostics, support export, and release surfaces
//! ingest the same vocabulary the shell emits. The boundary schema for this
//! packet is
//! [`schemas/voice/transcript-redaction-and-support-export.schema.json`](../../../../schemas/voice/transcript-redaction-and-support-export.schema.json)
//! and the truth doc is
//! [`docs/privacy/voice-support-export.md`](../../../../docs/privacy/voice-support-export.md).

#[cfg(test)]
mod tests;

pub mod seed;

use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub use seed::{seeded_voice_support_export_packet, write_fixtures, write_support_export};

/// Schema version stamped onto the packet and every record it carries.
pub const VOICE_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;

/// Record kind carried by [`VoiceSupportExportPacket`].
pub const VOICE_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str = "voice_support_export_packet";

/// Record kind carried by a [`VoiceSessionDiagnosticsRow`].
pub const VOICE_SESSION_DIAGNOSTICS_ROW_RECORD_KIND: &str = "voice_session_diagnostics_row";

/// Record kind carried by a [`VoiceTranscriptExportDecision`].
pub const VOICE_TRANSCRIPT_EXPORT_DECISION_RECORD_KIND: &str = "voice_transcript_export_decision";

/// Record kind carried by a [`VoiceTelemetryPosture`].
pub const VOICE_TELEMETRY_POSTURE_RECORD_KIND: &str = "voice_telemetry_posture";

/// Record kind carried by a [`TranscriptRedactionSummary`].
pub const VOICE_TRANSCRIPT_REDACTION_SUMMARY_RECORD_KIND: &str =
    "voice_transcript_redaction_summary";

/// Repo-relative boundary schema for this packet.
pub const VOICE_SUPPORT_EXPORT_SCHEMA_REF: &str =
    "schemas/voice/transcript-redaction-and-support-export.schema.json";

/// Repo-relative privacy truth doc for this lane.
pub const VOICE_SUPPORT_EXPORT_DOC_REF: &str = "docs/privacy/voice-support-export.md";

/// Repo-relative checked-in support-export packet (machine truth).
pub const VOICE_SUPPORT_EXPORT_PACKET_REF: &str = "artifacts/support/voice-session-export.json";

/// Repo-relative checked-in rendered support-export report (human truth).
pub const VOICE_SUPPORT_EXPORT_REPORT_REF: &str = "artifacts/support/voice-session-export.md";

/// Repo-relative directory of the checked-in redaction/export fixtures.
pub const VOICE_SUPPORT_EXPORT_FIXTURES_DIR_REF: &str = "fixtures/voice/redaction-and-export";

/// Repo-relative voice-session boundary schema this lane mirrors.
pub const VOICE_SESSION_STATE_SCHEMA_REF: &str = "schemas/voice/voice-session.schema.json";

/// Repo-relative retention/export boundary schema this lane mirrors.
pub const VOICE_RETENTION_EXPORT_SCHEMA_REF: &str =
    "schemas/voice/retention-and-export.schema.json";

/// Repo-relative privacy doc the broader voice lane shares.
pub const VOICE_PROCESSING_AND_RETENTION_DOC_REF: &str =
    "docs/privacy/voice-processing-and-retention.md";

// ---------------------------------------------------------------------------
// Mirrored canonical class tokens
// ---------------------------------------------------------------------------

/// Voice session mode. Mirrors `mode_class` in the voice-session schema: command
/// mode and dictation mode are always distinct, explicit states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceSessionMode {
    /// Microphone off; no capture.
    IdleMicrophoneOff,
    /// Dictation mode active (speech becomes editable text).
    DictationModeActive,
    /// Command mode active (speech resolves to canonical commands).
    CommandModeActive,
    /// Continuous listening active after an explicit user opt-in.
    ContinuousListeningActiveUserOptedIn,
    /// Voice mode blocked by policy.
    VoiceModeBlockedByPolicy,
    /// Voice mode blocked by the runtime envelope.
    VoiceModeBlockedByEnvelope,
}

impl VoiceSessionMode {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdleMicrophoneOff => "idle_microphone_off",
            Self::DictationModeActive => "dictation_mode_active",
            Self::CommandModeActive => "command_mode_active",
            Self::ContinuousListeningActiveUserOptedIn => {
                "continuous_listening_active_user_opted_in"
            }
            Self::VoiceModeBlockedByPolicy => "voice_mode_blocked_by_policy",
            Self::VoiceModeBlockedByEnvelope => "voice_mode_blocked_by_envelope",
        }
    }

    /// Whether the mode is a blocked state (no capture is possible).
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::VoiceModeBlockedByPolicy | Self::VoiceModeBlockedByEnvelope
        )
    }
}

/// Processing-locality cue. Mirrors `processing_locality` in the voice-session
/// schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingLocality {
    /// Audio processed on-device; no handoff.
    LocalOnDevice,
    /// Audio handed to a disclosed hosted/remote engine.
    HostedRemoteDisclosed,
    /// Processing unavailable in the current state.
    ProcessingUnavailable,
}

impl ProcessingLocality {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnDevice => "local_on_device",
            Self::HostedRemoteDisclosed => "hosted_remote_disclosed",
            Self::ProcessingUnavailable => "processing_unavailable",
        }
    }

    /// Whether the locality is on-device (the most private path).
    pub const fn is_local(self) -> bool {
        matches!(self, Self::LocalOnDevice)
    }
}

/// Speech-provider class. Mirrors the qualification-matrix provider class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceProviderClass {
    /// On-device, in-process speech engine.
    OnDeviceLocal,
    /// Approved remote provider, opt-in and disclosed.
    ApprovedRemoteDisclosed,
    /// Enterprise-managed relay/provider, policy-controlled and audited.
    EnterpriseRelayManaged,
    /// A mocked provider used only for conformance fixtures.
    MockedTestProvider,
    /// Provider disabled; voice is unavailable and falls back to keyboard.
    ProviderDisabled,
}

impl VoiceProviderClass {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnDeviceLocal => "on_device_local",
            Self::ApprovedRemoteDisclosed => "approved_remote_disclosed",
            Self::EnterpriseRelayManaged => "enterprise_relay_managed",
            Self::MockedTestProvider => "mocked_test_provider",
            Self::ProviderDisabled => "provider_disabled",
        }
    }

    /// Whether the provider processes audio off-device (remote or relay).
    pub const fn is_hosted(self) -> bool {
        matches!(
            self,
            Self::ApprovedRemoteDisclosed | Self::EnterpriseRelayManaged
        )
    }
}

/// Audio-retention posture. Mirrors `audio_retention` in the retention/export
/// schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioRetentionClass {
    /// No audio retained at all.
    NoAudioRetained,
    /// Ephemeral audio buffered local-only during capture, then dropped.
    EphemeralAudioLocalOnly,
    /// Audio retained in a bounded local window.
    BoundedAudioLocalWindow,
    /// Audio retained by the provider per an enterprise contract.
    AudioRetainedProviderPerContract,
    /// Audio capture blocked entirely.
    AudioCaptureBlocked,
}

impl AudioRetentionClass {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAudioRetained => "no_audio_retained",
            Self::EphemeralAudioLocalOnly => "ephemeral_audio_local_only",
            Self::BoundedAudioLocalWindow => "bounded_audio_local_window",
            Self::AudioRetainedProviderPerContract => "audio_retained_provider_per_contract",
            Self::AudioCaptureBlocked => "audio_capture_blocked",
        }
    }

    /// Whether audio is retained by a provider off-device (per contract).
    pub const fn is_provider_retained(self) -> bool {
        matches!(self, Self::AudioRetainedProviderPerContract)
    }
}

/// Transcript-export posture. Mirrors `transcript_export` in the retention/export
/// schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptExportPosture {
    /// No transcript export path.
    NoTranscriptExport,
    /// Explicit user-initiated export, redacted before it leaves.
    ExplicitUserExportRedacted,
    /// Only metadata (no transcript text) enters the support export.
    MetadataOnlySupportExport,
    /// Transcript retained/exported by the provider per an enterprise contract.
    ProviderContractRetained,
    /// Export blocked by policy.
    ExportBlockedByPolicy,
}

impl TranscriptExportPosture {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoTranscriptExport => "no_transcript_export",
            Self::ExplicitUserExportRedacted => "explicit_user_export_redacted",
            Self::MetadataOnlySupportExport => "metadata_only_support_export",
            Self::ProviderContractRetained => "provider_contract_retained",
            Self::ExportBlockedByPolicy => "export_blocked_by_policy",
        }
    }
}

/// Retention mode. Mirrors `retention_mode` in the retention/export schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionMode {
    /// No audio retained, no transcript retained.
    NoAudioNoTranscriptRetained,
    /// Ephemeral audio local-only, no transcript retained.
    EphemeralAudioLocalOnlyNoTranscriptRetained,
    /// Transcript retained local-only.
    TranscriptRetainedLocalOnly,
    /// Transcript retained, redacted in support bundle.
    TranscriptRetainedRedactedInSupportBundle,
    /// Transcript retained by provider per contract.
    TranscriptRetainedProviderPerContract,
    /// Retention blocked by policy.
    RetentionBlockedByPolicy,
    /// Retention unavailable in the envelope.
    RetentionUnavailableInEnvelope,
}

impl RetentionMode {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAudioNoTranscriptRetained => "no_audio_no_transcript_retained",
            Self::EphemeralAudioLocalOnlyNoTranscriptRetained => {
                "ephemeral_audio_local_only_no_transcript_retained"
            }
            Self::TranscriptRetainedLocalOnly => "transcript_retained_local_only",
            Self::TranscriptRetainedRedactedInSupportBundle => {
                "transcript_retained_redacted_in_support_bundle"
            }
            Self::TranscriptRetainedProviderPerContract => {
                "transcript_retained_provider_per_contract"
            }
            Self::RetentionBlockedByPolicy => "retention_blocked_by_policy",
            Self::RetentionUnavailableInEnvelope => "retention_unavailable_in_envelope",
        }
    }

    /// Whether the mode keeps all transcript/audio handling local-only.
    pub const fn is_local_only(self) -> bool {
        matches!(
            self,
            Self::NoAudioNoTranscriptRetained
                | Self::EphemeralAudioLocalOnlyNoTranscriptRetained
                | Self::TranscriptRetainedLocalOnly
        )
    }
}

/// Policy state of a voice session. Mirrors `policy_state` in the voice-session
/// schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoicePolicyState {
    /// Voice is user-controlled in this context.
    UserControlled,
    /// Voice is governed by enterprise policy.
    EnterprisePolicyManaged,
    /// Voice is blocked by policy; capture cannot start.
    PolicyBlocked,
}

impl VoicePolicyState {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserControlled => "user_controlled",
            Self::EnterprisePolicyManaged => "enterprise_policy_managed",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Aggregate recognition-confidence *class* for a session. This is a coarse
/// quality cue, never transcript content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceCue {
    /// High recognition confidence.
    High,
    /// Medium recognition confidence.
    Medium,
    /// Low recognition confidence (correction strongly suggested).
    Low,
}

impl ConfidenceCue {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

// ---------------------------------------------------------------------------
// Diagnostics class tokens introduced by this lane
// ---------------------------------------------------------------------------

/// Typed voice-session failure class. These let support explain *why* a session
/// did not complete normally without any transcript content or raw audio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceSessionFailureClass {
    /// Capture could not start: no microphone device present or permitted.
    CaptureStartFailedNoMicrophone,
    /// Capture was interrupted because the input device was lost mid-session.
    CaptureInterruptedDeviceLost,
    /// Recognition aborted because confidence stayed below the usable threshold.
    RecognitionLowConfidenceAborted,
    /// A hosted provider was unreachable; the session fell back to the local engine.
    HostedProviderUnreachableFellBackLocal,
    /// A hosted provider was unreachable and no local fallback existed; blocked.
    HostedProviderUnreachableBlocked,
    /// A requested language pack was unavailable; the baseline pack was used.
    LanguagePackUnavailableUsedBaseline,
    /// The transcript-correction window timed out before commit.
    TranscriptCorrectionTimedOut,
    /// A spoken command could not be disambiguated to a canonical command.
    CommandDisambiguationUnresolved,
    /// Capture was blocked by policy.
    PolicyBlockedCapture,
    /// Capture was blocked by the runtime envelope.
    EnvelopeBlockedCapture,
}

impl VoiceSessionFailureClass {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CaptureStartFailedNoMicrophone => "capture_start_failed_no_microphone",
            Self::CaptureInterruptedDeviceLost => "capture_interrupted_device_lost",
            Self::RecognitionLowConfidenceAborted => "recognition_low_confidence_aborted",
            Self::HostedProviderUnreachableFellBackLocal => {
                "hosted_provider_unreachable_fell_back_local"
            }
            Self::HostedProviderUnreachableBlocked => "hosted_provider_unreachable_blocked",
            Self::LanguagePackUnavailableUsedBaseline => "language_pack_unavailable_used_baseline",
            Self::TranscriptCorrectionTimedOut => "transcript_correction_timed_out",
            Self::CommandDisambiguationUnresolved => "command_disambiguation_unresolved",
            Self::PolicyBlockedCapture => "policy_blocked_capture",
            Self::EnvelopeBlockedCapture => "envelope_blocked_capture",
        }
    }
}

/// Typed blocked-action class. Records that a spoken action was intentionally
/// held or denied (rather than silently applied), so support can explain the
/// behavior without a debug switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceBlockedActionClass {
    /// A high-impact command was held for explicit confirmation.
    HighImpactCommandHeldForConfirmation,
    /// A privileged action was held for a preview before applying.
    PrivilegedActionHeldForPreview,
    /// A spoken command fell outside the active capability envelope.
    CommandOutsideCapabilityEnvelope,
    /// A spoken command was denied by policy.
    CommandDeniedByPolicy,
    /// Dictation targeted a surface that does not accept dictated text.
    DictationTargetSurfaceUnsupported,
    /// Continuous listening was requested but is blocked by policy.
    ContinuousListeningBlockedByPolicy,
}

impl VoiceBlockedActionClass {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighImpactCommandHeldForConfirmation => {
                "high_impact_command_held_for_confirmation"
            }
            Self::PrivilegedActionHeldForPreview => "privileged_action_held_for_preview",
            Self::CommandOutsideCapabilityEnvelope => "command_outside_capability_envelope",
            Self::CommandDeniedByPolicy => "command_denied_by_policy",
            Self::DictationTargetSurfaceUnsupported => "dictation_target_surface_unsupported",
            Self::ContinuousListeningBlockedByPolicy => "continuous_listening_blocked_by_policy",
        }
    }
}

/// Typed provider-drift class: the observed difference between the requested
/// provider/locality/profile and the active one. By contract drift only ever
/// moves toward a more-private (on-device, narrower) posture, never toward a
/// broader or more remote one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceProviderDriftClass {
    /// The active provider/locality/profile matched the request.
    NoDriftObserved,
    /// The active provider was downgraded to the on-device engine.
    ProviderDowngradedToLocal,
    /// The active locality moved to on-device processing.
    LocalityMovedToLocal,
    /// A requested language pack fell back to the on-device baseline profile.
    LanguagePackFellBackToBaseline,
    /// The active retention posture was narrowed (more private) than requested.
    RetentionPostureNarrowed,
    /// The active export posture was narrowed (more private) than requested.
    ExportPostureNarrowed,
    /// The requested provider became unavailable in this session.
    ProviderBecameUnavailable,
}

impl VoiceProviderDriftClass {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDriftObserved => "no_drift_observed",
            Self::ProviderDowngradedToLocal => "provider_downgraded_to_local",
            Self::LocalityMovedToLocal => "locality_moved_to_local",
            Self::LanguagePackFellBackToBaseline => "language_pack_fell_back_to_baseline",
            Self::RetentionPostureNarrowed => "retention_posture_narrowed",
            Self::ExportPostureNarrowed => "export_posture_narrowed",
            Self::ProviderBecameUnavailable => "provider_became_unavailable",
        }
    }

    /// Whether observed drift kept the session at least as private as requested.
    /// Every variant here is privacy-preserving by construction, so this is a
    /// structural guarantee the validator can rely on.
    pub const fn preserves_privacy(self) -> bool {
        true
    }
}

/// User-visible inclusion state of a transcript export decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptInclusionState {
    /// Transcript text is excluded by default; only metadata is captured.
    ExcludedByDefault,
    /// Transcript text was redacted and included after an explicit user review.
    RedactedIncludedAfterExplicitReview,
    /// Transcript export is blocked by policy in this context.
    BlockedByPolicy,
    /// No transcript was produced for the session.
    NoTranscriptAvailable,
}

impl TranscriptInclusionState {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExcludedByDefault => "excluded_by_default",
            Self::RedactedIncludedAfterExplicitReview => "redacted_included_after_explicit_review",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::NoTranscriptAvailable => "no_transcript_available",
        }
    }

    /// Whether this state means transcript text is included in the (user-owned)
    /// export.
    pub const fn includes_transcript(self) -> bool {
        matches!(self, Self::RedactedIncludedAfterExplicitReview)
    }
}

// ---------------------------------------------------------------------------
// Redaction primitive
// ---------------------------------------------------------------------------

/// A class of sensitive span the redactor masks. The summary carries only the
/// set of classes that were hit, never the spans themselves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// An email address.
    EmailAddress,
    /// A long numeric sequence (phone, card, account, long id).
    LongNumericSequence,
    /// An absolute filesystem path (POSIX or Windows drive form).
    AbsolutePath,
    /// A URL.
    Url,
    /// An IPv4 address.
    IpAddress,
    /// A credential-like token (key prefix or long opaque string).
    CredentialToken,
}

impl RedactionClass {
    /// Stable token, equal to the serialized form.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmailAddress => "email_address",
            Self::LongNumericSequence => "long_numeric_sequence",
            Self::AbsolutePath => "absolute_path",
            Self::Url => "url",
            Self::IpAddress => "ip_address",
            Self::CredentialToken => "credential_token",
        }
    }

    /// The placeholder substituted for a span of this class.
    pub const fn placeholder(self) -> &'static str {
        match self {
            Self::EmailAddress => "[redacted-email]",
            Self::LongNumericSequence => "[redacted-number]",
            Self::AbsolutePath => "[redacted-path]",
            Self::Url => "[redacted-url]",
            Self::IpAddress => "[redacted-ip]",
            Self::CredentialToken => "[redacted-token]",
        }
    }
}

/// Content-free summary of a redaction pass. This is the only redaction artifact
/// that ever enters the support packet: it proves redaction happened (how many
/// spans, of which classes) without carrying any transcript text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptRedactionSummary {
    /// Record kind; must equal [`VOICE_TRANSCRIPT_REDACTION_SUMMARY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VOICE_SUPPORT_EXPORT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Number of sensitive spans masked.
    pub redacted_span_count: u32,
    /// The classes of span that were masked.
    pub classes_redacted: BTreeSet<RedactionClass>,
    /// Whether the residual (redacted) text is still kept out of the support
    /// export by default (it is — only this summary travels into support).
    pub residual_text_excluded_from_support: bool,
}

impl TranscriptRedactionSummary {
    /// Whether the summary is structurally well-formed and content-free.
    pub fn is_well_formed(&self) -> bool {
        self.record_kind == VOICE_TRANSCRIPT_REDACTION_SUMMARY_RECORD_KIND
            && self.schema_version == VOICE_SUPPORT_EXPORT_SCHEMA_VERSION
            && self.residual_text_excluded_from_support
            && (self.redacted_span_count as usize) >= self.classes_redacted.len()
    }
}

/// Result of redacting a transcript for an explicit, user-reviewed export. The
/// `redacted_text` is what flows into the user's own export destination; only
/// the [`TranscriptRedactionSummary`] ever enters the support packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranscriptRedactionResult {
    /// The transcript with every sensitive span masked by a placeholder.
    pub redacted_text: String,
    /// The content-free summary of the pass.
    pub summary: TranscriptRedactionSummary,
}

/// Redacts a raw transcript for an explicit, user-reviewed export.
///
/// Masks emails, URLs, IPv4 addresses, absolute paths, credential-like tokens,
/// and long numeric sequences. The function is deterministic and allocates a
/// fresh string; it never logs, retains, or returns the raw input, and the
/// summary it returns carries only counts and class tokens.
pub fn redact_transcript(raw: &str) -> TranscriptRedactionResult {
    let mut classes: BTreeSet<RedactionClass> = BTreeSet::new();
    let mut span_count: u32 = 0;
    let mut out_tokens: Vec<String> = Vec::new();

    for token in raw.split_whitespace() {
        let (redacted, hits) = redact_token(token);
        for class in hits {
            classes.insert(class);
            span_count = span_count.saturating_add(1);
        }
        out_tokens.push(redacted);
    }

    let summary = TranscriptRedactionSummary {
        record_kind: VOICE_TRANSCRIPT_REDACTION_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: VOICE_SUPPORT_EXPORT_SCHEMA_VERSION,
        redacted_span_count: span_count,
        classes_redacted: classes,
        residual_text_excluded_from_support: true,
    };

    TranscriptRedactionResult {
        redacted_text: out_tokens.join(" "),
        summary,
    }
}

/// Redacts a single whitespace-delimited token, returning the masked token and
/// the classes it triggered. Whole-token structured types win over an in-token
/// digit-run pass; punctuation around a structured value is preserved.
fn redact_token(token: &str) -> (String, Vec<RedactionClass>) {
    let trimmed = token.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '/' && c != '\\');
    if trimmed.is_empty() {
        return (token.to_owned(), Vec::new());
    }

    if let Some(class) = whole_token_class(trimmed) {
        let replaced = token.replacen(trimmed, class.placeholder(), 1);
        return (replaced, vec![class]);
    }

    redact_digit_runs(token)
}

/// Classifies a token as a single structured sensitive value, if it is one.
fn whole_token_class(token: &str) -> Option<RedactionClass> {
    if token.starts_with("http://") || token.starts_with("https://") {
        return Some(RedactionClass::Url);
    }
    if is_email(token) {
        return Some(RedactionClass::EmailAddress);
    }
    if is_ipv4(token) {
        return Some(RedactionClass::IpAddress);
    }
    if is_absolute_path(token) {
        return Some(RedactionClass::AbsolutePath);
    }
    if is_credential_token(token) {
        return Some(RedactionClass::CredentialToken);
    }
    None
}

/// Whether a token looks like an email address (`local@domain.tld`).
fn is_email(token: &str) -> bool {
    let mut parts = token.split('@');
    let (Some(local), Some(domain), None) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    !local.is_empty()
        && domain.contains('.')
        && domain.split('.').all(|label| {
            !label.is_empty() && label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        })
}

/// Whether a token is a dotted-quad IPv4 address.
fn is_ipv4(token: &str) -> bool {
    let groups: Vec<&str> = token.split('.').collect();
    groups.len() == 4
        && groups.iter().all(|group| {
            !group.is_empty()
                && group.len() <= 3
                && group.chars().all(|c| c.is_ascii_digit())
                && group.parse::<u16>().map(|n| n <= 255).unwrap_or(false)
        })
}

/// Whether a token is an absolute filesystem path.
fn is_absolute_path(token: &str) -> bool {
    if token.starts_with('/') && token.len() > 1 {
        return true;
    }
    let bytes = token.as_bytes();
    bytes.len() >= 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'\\'
}

/// Whether a token looks like a credential: a known key prefix or a long opaque
/// run of token characters.
fn is_credential_token(token: &str) -> bool {
    const PREFIXES: [&str; 4] = ["sk-", "ghp_", "pat_", "xoxb-"];
    if PREFIXES.iter().any(|prefix| token.starts_with(prefix)) {
        return true;
    }
    let opaque = token
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    let has_letter = token.chars().any(|c| c.is_ascii_alphabetic());
    let has_digit = token.chars().any(|c| c.is_ascii_digit());
    opaque && has_letter && has_digit && token.len() >= 20
}

/// Replaces maximal runs of 4+ digits in a token with the numeric placeholder.
fn redact_digit_runs(token: &str) -> (String, Vec<RedactionClass>) {
    let mut out = String::with_capacity(token.len());
    let mut run = String::new();
    let mut hit = false;

    let flush = |run: &mut String, out: &mut String, hit: &mut bool| {
        if run.chars().filter(|c| c.is_ascii_digit()).count() >= 4 {
            out.push_str(RedactionClass::LongNumericSequence.placeholder());
            *hit = true;
        } else {
            out.push_str(run);
        }
        run.clear();
    };

    for c in token.chars() {
        if c.is_ascii_digit() {
            run.push(c);
        } else {
            flush(&mut run, &mut out, &mut hit);
            out.push(c);
        }
    }
    flush(&mut run, &mut out, &mut hit);

    let classes = if hit {
        vec![RedactionClass::LongNumericSequence]
    } else {
        Vec::new()
    };
    (out, classes)
}

// ---------------------------------------------------------------------------
// Diagnostics row
// ---------------------------------------------------------------------------

/// One metadata-only diagnostic row per voice session. It carries every class
/// token support needs to explain the session — mode, provider, locality,
/// retention/audio/export posture, policy, an aggregate confidence *class*, and
/// the failure/blocked-action/drift classes — and pins both raw-exclusion flags
/// true. No raw audio bytes, raw transcript text, private paths, or credentials
/// ever appear here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceSessionDiagnosticsRow {
    /// Record kind; must equal [`VOICE_SESSION_DIAGNOSTICS_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VOICE_SUPPORT_EXPORT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Opaque, non-empty session id (no path or user content).
    pub session_id: String,
    /// Export-safe human label describing the session at a glance.
    pub session_label: String,
    /// Session mode (command vs dictation vs idle vs blocked).
    pub mode: VoiceSessionMode,
    /// Active speech-provider class.
    pub provider_class: VoiceProviderClass,
    /// Active processing locality.
    pub processing_locality: ProcessingLocality,
    /// Active retention mode.
    pub retention_mode: RetentionMode,
    /// Active audio-retention class.
    pub audio_retention: AudioRetentionClass,
    /// Active transcript-export posture.
    pub transcript_export: TranscriptExportPosture,
    /// Active policy state.
    pub policy_state: VoicePolicyState,
    /// Aggregate recognition-confidence class, when recognition ran.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub aggregate_confidence: Option<ConfidenceCue>,
    /// Failure class, when the session did not complete normally.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub failure_class: Option<VoiceSessionFailureClass>,
    /// Blocked-action class, when a spoken action was held or denied.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub blocked_action_class: Option<VoiceBlockedActionClass>,
    /// Observed provider/locality/profile drift class.
    pub provider_drift_class: VoiceProviderDriftClass,
    /// Whether the keyboard/command-palette fallback stayed available.
    pub keyboard_fallback_available: bool,
    /// Whether raw audio is excluded from this row (must be true).
    pub raw_audio_excluded: bool,
    /// Whether raw transcript text is excluded from this row (must be true).
    pub raw_transcript_excluded: bool,
}

impl VoiceSessionDiagnosticsRow {
    /// Whether the row keeps raw audio and raw transcript out (data minimization).
    pub const fn data_minimization_held(&self) -> bool {
        self.raw_audio_excluded && self.raw_transcript_excluded
    }

    /// Whether the row's cross-field state is internally consistent. This is the
    /// "diagnostics can explain the session without widening retention" check.
    pub fn is_consistent(&self) -> bool {
        if self.record_kind != VOICE_SESSION_DIAGNOSTICS_ROW_RECORD_KIND
            || self.schema_version != VOICE_SUPPORT_EXPORT_SCHEMA_VERSION
            || self.session_id.trim().is_empty()
            || self.session_label.trim().is_empty()
        {
            return false;
        }
        // Data minimization is non-negotiable.
        if !self.data_minimization_held() {
            return false;
        }
        // The keyboard path is never dropped by a voice session.
        if !self.keyboard_fallback_available {
            return false;
        }
        // Provider and locality must agree on hosted-vs-local.
        match (self.provider_class, self.processing_locality) {
            (VoiceProviderClass::OnDeviceLocal, ProcessingLocality::HostedRemoteDisclosed) => {
                return false;
            }
            (
                VoiceProviderClass::ApprovedRemoteDisclosed
                | VoiceProviderClass::EnterpriseRelayManaged,
                ProcessingLocality::LocalOnDevice,
            ) => return false,
            _ => {}
        }
        // Provider-retained audio/transcript can only ride a hosted provider.
        if self.audio_retention.is_provider_retained() && !self.provider_class.is_hosted() {
            return false;
        }
        if matches!(
            self.transcript_export,
            TranscriptExportPosture::ProviderContractRetained
        ) && !self.provider_class.is_hosted()
        {
            return false;
        }
        // On-device processing can never carry provider-retained audio/transcript.
        if self.processing_locality.is_local()
            && (self.audio_retention.is_provider_retained()
                || matches!(
                    self.transcript_export,
                    TranscriptExportPosture::ProviderContractRetained
                ))
        {
            return false;
        }
        // A blocked mode must reflect a blocked/unavailable posture.
        if self.mode.is_blocked()
            && !matches!(
                self.processing_locality,
                ProcessingLocality::ProcessingUnavailable
            )
        {
            return false;
        }
        true
    }
}

// ---------------------------------------------------------------------------
// Transcript export decision
// ---------------------------------------------------------------------------

/// An explicit, reviewed transcript-export decision. Transcript text is excluded
/// by default; the only inclusion path is an explicit, user-reviewed, redacted,
/// bounded export, and even then only the [`TranscriptRedactionSummary`] travels
/// into support — never the text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceTranscriptExportDecision {
    /// Record kind; must equal [`VOICE_TRANSCRIPT_EXPORT_DECISION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VOICE_SUPPORT_EXPORT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// User-visible inclusion state.
    pub inclusion_state: TranscriptInclusionState,
    /// The export posture backing this decision.
    pub transcript_export_posture: TranscriptExportPosture,
    /// Whether redaction was applied before any text left the device.
    pub redaction_applied: bool,
    /// Whether the user explicitly reviewed the export.
    pub reviewed_by_user: bool,
    /// Export-safe, user-visible label shown wherever this export is referenced.
    pub user_visible_label: String,
    /// Number of transcript segments the user explicitly chose to export.
    pub bounded_segment_count: u32,
    /// Content-free redaction summary, present only when text was included.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub redaction_summary: Option<TranscriptRedactionSummary>,
}

impl VoiceTranscriptExportDecision {
    /// Whether the decision is internally consistent with its inclusion state.
    pub fn is_consistent(&self) -> bool {
        if self.record_kind != VOICE_TRANSCRIPT_EXPORT_DECISION_RECORD_KIND
            || self.schema_version != VOICE_SUPPORT_EXPORT_SCHEMA_VERSION
            || self.user_visible_label.trim().is_empty()
        {
            return false;
        }
        match self.inclusion_state {
            TranscriptInclusionState::RedactedIncludedAfterExplicitReview => {
                self.redaction_applied
                    && self.reviewed_by_user
                    && self.bounded_segment_count > 0
                    && matches!(
                        self.transcript_export_posture,
                        TranscriptExportPosture::ExplicitUserExportRedacted
                    )
                    && self
                        .redaction_summary
                        .as_ref()
                        .is_some_and(TranscriptRedactionSummary::is_well_formed)
            }
            TranscriptInclusionState::ExcludedByDefault => {
                !self.redaction_applied
                    && self.bounded_segment_count == 0
                    && self.redaction_summary.is_none()
                    && matches!(
                        self.transcript_export_posture,
                        TranscriptExportPosture::NoTranscriptExport
                            | TranscriptExportPosture::MetadataOnlySupportExport
                    )
            }
            TranscriptInclusionState::BlockedByPolicy => {
                !self.redaction_applied
                    && self.bounded_segment_count == 0
                    && self.redaction_summary.is_none()
                    && matches!(
                        self.transcript_export_posture,
                        TranscriptExportPosture::ExportBlockedByPolicy
                    )
            }
            TranscriptInclusionState::NoTranscriptAvailable => {
                !self.redaction_applied
                    && self.bounded_segment_count == 0
                    && self.redaction_summary.is_none()
                    && matches!(
                        self.transcript_export_posture,
                        TranscriptExportPosture::NoTranscriptExport
                    )
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Telemetry posture
// ---------------------------------------------------------------------------

/// The no-audio-default telemetry/crash/log posture. Raw audio and sensitive
/// transcript content stay out of telemetry, crash packets, and logs; only
/// metadata class tokens are captured.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceTelemetryPosture {
    /// Record kind; must equal [`VOICE_TELEMETRY_POSTURE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VOICE_SUPPORT_EXPORT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Whether raw audio enters telemetry (must be false).
    pub raw_audio_in_telemetry: bool,
    /// Whether raw transcript text enters telemetry (must be false).
    pub raw_transcript_in_telemetry: bool,
    /// Whether raw audio enters crash packets (must be false).
    pub raw_audio_in_crash_packets: bool,
    /// Whether sensitive transcript content enters logs (must be false).
    pub sensitive_transcript_in_logs: bool,
    /// The metadata class-token field names telemetry/diagnostics capture.
    pub captured_metadata_classes: BTreeSet<String>,
}

impl VoiceTelemetryPosture {
    /// Whether the posture holds data-minimization: no raw audio/transcript in
    /// telemetry, crash packets, or logs, and at least one metadata class is
    /// still captured so behavior stays explainable.
    pub fn data_minimization_held(&self) -> bool {
        self.record_kind == VOICE_TELEMETRY_POSTURE_RECORD_KIND
            && self.schema_version == VOICE_SUPPORT_EXPORT_SCHEMA_VERSION
            && !self.raw_audio_in_telemetry
            && !self.raw_transcript_in_telemetry
            && !self.raw_audio_in_crash_packets
            && !self.sensitive_transcript_in_logs
            && !self.captured_metadata_classes.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Guardrails and consumer projection
// ---------------------------------------------------------------------------

/// The invariants this lane guarantees. Every field must be true on a valid
/// packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceSupportExportGuardrails {
    /// Raw audio is excluded from support/telemetry by default.
    pub raw_audio_excluded_by_default: bool,
    /// Raw transcript content is excluded from support/telemetry by default.
    pub raw_transcript_excluded_by_default: bool,
    /// Transcript export is explicit, reviewed, redacted, and bounded.
    pub transcript_export_explicit_and_redacted: bool,
    /// Failures, blocked actions, and provider drift are diagnosable.
    pub failures_blocked_actions_and_drift_diagnosable: bool,
    /// Supportability convenience never widens retention by default.
    pub supportability_does_not_widen_retention: bool,
    /// The keyboard fallback is always available.
    pub keyboard_fallback_always_available: bool,
}

impl VoiceSupportExportGuardrails {
    /// Whether every guardrail holds.
    pub const fn all_true(&self) -> bool {
        self.raw_audio_excluded_by_default
            && self.raw_transcript_excluded_by_default
            && self.transcript_export_explicit_and_redacted
            && self.failures_blocked_actions_and_drift_diagnosable
            && self.supportability_does_not_widen_retention
            && self.keyboard_fallback_always_available
    }
}

/// Which downstream surfaces ingest this packet rather than cloning voice state
/// text by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceSupportExportConsumerProjection {
    /// Diagnostics ingests this packet.
    pub diagnostics_ingests: bool,
    /// Support export ingests this packet.
    pub support_export_ingests: bool,
    /// Help/About ingests this packet.
    pub help_about_ingests: bool,
    /// Release center ingests this packet.
    pub release_center_ingests: bool,
    /// The telemetry schema ingests the captured metadata classes.
    pub telemetry_schema_ingests: bool,
}

impl VoiceSupportExportConsumerProjection {
    /// Whether every consumer ingests the shared packet.
    pub const fn all_true(&self) -> bool {
        self.diagnostics_ingests
            && self.support_export_ingests
            && self.help_about_ingests
            && self.release_center_ingests
            && self.telemetry_schema_ingests
    }
}

// ---------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------

/// Constructor input for [`VoiceSupportExportPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceSupportExportPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human label for the packet.
    pub label: String,
    /// One diagnostics row per session.
    pub sessions: Vec<VoiceSessionDiagnosticsRow>,
    /// The transcript-export decisions covered by this packet.
    pub transcript_export_decisions: Vec<VoiceTranscriptExportDecision>,
    /// The telemetry/crash/log posture.
    pub telemetry_posture: VoiceTelemetryPosture,
    /// The guardrail block.
    pub guardrails: VoiceSupportExportGuardrails,
    /// The consumer projection.
    pub consumer_projection: VoiceSupportExportConsumerProjection,
    /// Source contract refs this packet quotes.
    pub source_contract_refs: Vec<String>,
    /// Redaction-class token for the packet as a whole.
    pub redaction_class_token: String,
    /// Mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// Top-level, export-safe voice support packet. It folds the per-session
/// diagnostics rows, the transcript-export decisions, the telemetry posture, the
/// guardrails, and the consumer projection into one inspectable record consumed
/// by diagnostics, support export, Help/About, and release surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceSupportExportPacket {
    /// Record kind; must equal [`VOICE_SUPPORT_EXPORT_PACKET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VOICE_SUPPORT_EXPORT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human label for the packet.
    pub label: String,
    /// One diagnostics row per session.
    pub sessions: Vec<VoiceSessionDiagnosticsRow>,
    /// The transcript-export decisions covered by this packet.
    pub transcript_export_decisions: Vec<VoiceTranscriptExportDecision>,
    /// The telemetry/crash/log posture.
    pub telemetry_posture: VoiceTelemetryPosture,
    /// The guardrail block.
    pub guardrails: VoiceSupportExportGuardrails,
    /// The consumer projection.
    pub consumer_projection: VoiceSupportExportConsumerProjection,
    /// Source contract refs this packet quotes.
    pub source_contract_refs: Vec<String>,
    /// Redaction-class token for the packet as a whole.
    pub redaction_class_token: String,
    /// Mint timestamp (RFC 3339).
    pub minted_at: String,
}

impl VoiceSupportExportPacket {
    /// Builds a packet with the canonical record kind and schema version.
    pub fn new(input: VoiceSupportExportPacketInput) -> Self {
        Self {
            record_kind: VOICE_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: VOICE_SUPPORT_EXPORT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            sessions: input.sessions,
            transcript_export_decisions: input.transcript_export_decisions,
            telemetry_posture: input.telemetry_posture,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Number of sessions that recorded a failure class.
    pub fn failure_session_count(&self) -> usize {
        self.sessions
            .iter()
            .filter(|row| row.failure_class.is_some())
            .count()
    }

    /// Number of sessions that recorded a blocked-action class.
    pub fn blocked_action_session_count(&self) -> usize {
        self.sessions
            .iter()
            .filter(|row| row.blocked_action_class.is_some())
            .count()
    }

    /// Number of sessions that recorded observed provider drift.
    pub fn drift_session_count(&self) -> usize {
        self.sessions
            .iter()
            .filter(|row| {
                !matches!(
                    row.provider_drift_class,
                    VoiceProviderDriftClass::NoDriftObserved
                )
            })
            .count()
    }

    /// Looks up a diagnostics row by session id.
    pub fn session(&self, session_id: &str) -> Option<&VoiceSessionDiagnosticsRow> {
        self.sessions
            .iter()
            .find(|row| row.session_id == session_id)
    }

    /// Validates every privacy and supportability invariant. An empty result
    /// means the packet is export-safe.
    pub fn validate(&self) -> Vec<VoiceSupportExportViolation> {
        let mut violations: BTreeSet<VoiceSupportExportViolation> = BTreeSet::new();

        if self.record_kind != VOICE_SUPPORT_EXPORT_PACKET_RECORD_KIND {
            violations.insert(VoiceSupportExportViolation::WrongRecordKind);
        }
        if self.schema_version != VOICE_SUPPORT_EXPORT_SCHEMA_VERSION {
            violations.insert(VoiceSupportExportViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            violations.insert(VoiceSupportExportViolation::MissingIdentity);
        }
        for required in [
            VOICE_SUPPORT_EXPORT_SCHEMA_REF,
            VOICE_SUPPORT_EXPORT_DOC_REF,
        ] {
            if !self.source_contract_refs.iter().any(|r| r == required) {
                violations.insert(VoiceSupportExportViolation::MissingSourceContracts);
            }
        }

        if self.sessions.is_empty() {
            violations.insert(VoiceSupportExportViolation::SessionRowIncomplete);
        }
        for row in &self.sessions {
            if !row.is_consistent() {
                violations.insert(VoiceSupportExportViolation::SessionRowIncomplete);
            }
            if !row.raw_audio_excluded {
                violations.insert(VoiceSupportExportViolation::RawAudioNotExcluded);
            }
            if !row.raw_transcript_excluded {
                violations.insert(VoiceSupportExportViolation::RawTranscriptNotExcluded);
            }
            if !row.keyboard_fallback_available {
                violations.insert(VoiceSupportExportViolation::KeyboardFallbackMissing);
            }
            if !row.provider_drift_class.preserves_privacy() {
                violations.insert(VoiceSupportExportViolation::ProviderDriftNotTowardLocal);
            }
            // Supportability must not widen retention: a metadata-only support
            // posture can never coincide with a row that leaks raw transcript.
            if matches!(
                row.transcript_export,
                TranscriptExportPosture::MetadataOnlySupportExport
            ) && !row.raw_transcript_excluded
            {
                violations.insert(VoiceSupportExportViolation::SupportabilityWidenedRetention);
            }
        }

        // Telemetry posture.
        if self.telemetry_posture.raw_audio_in_telemetry {
            violations.insert(VoiceSupportExportViolation::TelemetryCarriesRawAudio);
        }
        if self.telemetry_posture.raw_transcript_in_telemetry {
            violations.insert(VoiceSupportExportViolation::TelemetryCarriesRawTranscript);
        }
        if self.telemetry_posture.raw_audio_in_crash_packets {
            violations.insert(VoiceSupportExportViolation::CrashPacketCarriesRawAudio);
        }
        if self.telemetry_posture.sensitive_transcript_in_logs {
            violations.insert(VoiceSupportExportViolation::LogsCarrySensitiveTranscript);
        }
        if !self.telemetry_posture.data_minimization_held() {
            violations.insert(VoiceSupportExportViolation::TelemetryMetadataMissing);
        }

        // Transcript export decisions.
        if self.transcript_export_decisions.is_empty() {
            violations.insert(VoiceSupportExportViolation::TranscriptInclusionInconsistent);
        }
        for decision in &self.transcript_export_decisions {
            if !decision.is_consistent() {
                violations.insert(VoiceSupportExportViolation::TranscriptInclusionInconsistent);
            }
            if decision.user_visible_label.trim().is_empty() {
                violations.insert(VoiceSupportExportViolation::TranscriptExportLabelMissing);
            }
            if decision.inclusion_state.includes_transcript() {
                if !decision.reviewed_by_user {
                    violations.insert(VoiceSupportExportViolation::TranscriptExportNotExplicit);
                }
                if !decision.redaction_applied {
                    violations.insert(VoiceSupportExportViolation::TranscriptExportNotRedacted);
                }
            }
        }

        // Coverage: the seed must prove all three acceptance criteria.
        if self.failure_session_count() == 0 {
            violations.insert(VoiceSupportExportViolation::FailureDiagnosabilityCaseMissing);
        }
        if self.blocked_action_session_count() == 0 {
            violations.insert(VoiceSupportExportViolation::BlockedActionDiagnosabilityCaseMissing);
        }
        if self.drift_session_count() == 0 {
            violations.insert(VoiceSupportExportViolation::ProviderDriftDiagnosabilityCaseMissing);
        }
        let has_default_exclusion = self.transcript_export_decisions.iter().any(|d| {
            matches!(
                d.inclusion_state,
                TranscriptInclusionState::ExcludedByDefault
            )
        });
        if !has_default_exclusion {
            violations.insert(VoiceSupportExportViolation::DefaultExclusionCaseMissing);
        }
        let has_explicit_export = self.transcript_export_decisions.iter().any(|d| {
            matches!(
                d.inclusion_state,
                TranscriptInclusionState::RedactedIncludedAfterExplicitReview
            )
        });
        if !has_explicit_export {
            violations.insert(VoiceSupportExportViolation::ExplicitRedactedExportCaseMissing);
        }

        if !self.guardrails.all_true() {
            violations.insert(VoiceSupportExportViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_true() {
            violations.insert(VoiceSupportExportViolation::ConsumerProjectionIncomplete);
        }

        violations.into_iter().collect()
    }

    /// Deterministic export-safe JSON (no trailing newline).
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("voice support export packet serializes")
    }

    /// Compact one-line-per-record summary for diagnostics and support handoff.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines =
            Vec::with_capacity(self.sessions.len() + self.transcript_export_decisions.len() + 1);
        lines.push(format!(
            "{} | sessions={} | failures={} | blocked={} | drift={} | data_minimization_ok={}",
            self.packet_id,
            self.sessions.len(),
            self.failure_session_count(),
            self.blocked_action_session_count(),
            self.drift_session_count(),
            self.validate().is_empty(),
        ));
        for row in &self.sessions {
            lines.push(format!(
                "{} | mode={} | provider={} | locality={} | retention={} | audio={} | export={} | confidence={} | failure={} | blocked={} | drift={} | raw_audio_excluded={} | raw_transcript_excluded={}",
                row.session_id,
                row.mode.as_str(),
                row.provider_class.as_str(),
                row.processing_locality.as_str(),
                row.retention_mode.as_str(),
                row.audio_retention.as_str(),
                row.transcript_export.as_str(),
                row.aggregate_confidence.map_or("none", ConfidenceCue::as_str),
                row.failure_class.map_or("none", VoiceSessionFailureClass::as_str),
                row.blocked_action_class.map_or("none", VoiceBlockedActionClass::as_str),
                row.provider_drift_class.as_str(),
                row.raw_audio_excluded,
                row.raw_transcript_excluded,
            ));
        }
        for decision in &self.transcript_export_decisions {
            lines.push(format!(
                "transcript_export | state={} | posture={} | redacted={} | reviewed={} | segments={} | label=\"{}\"",
                decision.inclusion_state.as_str(),
                decision.transcript_export_posture.as_str(),
                decision.redaction_applied,
                decision.reviewed_by_user,
                decision.bounded_segment_count,
                decision.user_visible_label,
            ));
        }
        lines
    }

    /// Deterministic Markdown summary for docs, support, or release handoff.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Voice Session Support Export & Redaction\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str(&format!(
            "- Sessions: {} ({} with a failure, {} with a blocked action, {} with provider drift)\n",
            self.sessions.len(),
            self.failure_session_count(),
            self.blocked_action_session_count(),
            self.drift_session_count(),
        ));
        out.push_str(&format!(
            "- Data minimization: raw audio in telemetry = {}, raw transcript in telemetry = {}, raw audio in crash packets = {}, sensitive transcript in logs = {}\n",
            self.telemetry_posture.raw_audio_in_telemetry,
            self.telemetry_posture.raw_transcript_in_telemetry,
            self.telemetry_posture.raw_audio_in_crash_packets,
            self.telemetry_posture.sensitive_transcript_in_logs,
        ));

        out.push_str("\n## Session diagnostics\n\n");
        for row in &self.sessions {
            out.push_str(&format!(
                "- **{}** — {}\n",
                row.session_id, row.session_label
            ));
            out.push_str(&format!(
                "  - mode = `{}`, provider = `{}`, locality = `{}`\n",
                row.mode.as_str(),
                row.provider_class.as_str(),
                row.processing_locality.as_str(),
            ));
            out.push_str(&format!(
                "  - retention = `{}`, audio = `{}`, export = `{}`, policy = `{}`\n",
                row.retention_mode.as_str(),
                row.audio_retention.as_str(),
                row.transcript_export.as_str(),
                row.policy_state.as_str(),
            ));
            out.push_str(&format!(
                "  - confidence = `{}`, failure = `{}`, blocked = `{}`, drift = `{}`\n",
                row.aggregate_confidence
                    .map_or("none", ConfidenceCue::as_str),
                row.failure_class
                    .map_or("none", VoiceSessionFailureClass::as_str),
                row.blocked_action_class
                    .map_or("none", VoiceBlockedActionClass::as_str),
                row.provider_drift_class.as_str(),
            ));
            out.push_str(&format!(
                "  - raw audio excluded = {}, raw transcript excluded = {}, keyboard fallback = {}\n",
                row.raw_audio_excluded, row.raw_transcript_excluded, row.keyboard_fallback_available,
            ));
        }

        out.push_str("\n## Transcript export decisions\n\n");
        for decision in &self.transcript_export_decisions {
            out.push_str(&format!(
                "- `{}` ({}): {}\n",
                decision.inclusion_state.as_str(),
                decision.transcript_export_posture.as_str(),
                decision.user_visible_label,
            ));
            out.push_str(&format!(
                "  - redaction applied = {}, reviewed by user = {}, segments = {}\n",
                decision.redaction_applied,
                decision.reviewed_by_user,
                decision.bounded_segment_count,
            ));
            if let Some(summary) = &decision.redaction_summary {
                let classes = summary
                    .classes_redacted
                    .iter()
                    .map(|c| c.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!(
                    "  - redacted spans = {} ({})\n",
                    summary.redacted_span_count, classes,
                ));
            }
        }

        out
    }
}

/// A single validation failure emitted by [`VoiceSupportExportPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VoiceSupportExportViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// A required source contract ref is missing.
    MissingSourceContracts,
    /// A session row is missing or internally inconsistent.
    SessionRowIncomplete,
    /// A session row did not exclude raw audio.
    RawAudioNotExcluded,
    /// A session row did not exclude raw transcript text.
    RawTranscriptNotExcluded,
    /// A session row dropped the keyboard fallback.
    KeyboardFallbackMissing,
    /// Telemetry carries raw audio.
    TelemetryCarriesRawAudio,
    /// Telemetry carries raw transcript text.
    TelemetryCarriesRawTranscript,
    /// A crash packet carries raw audio.
    CrashPacketCarriesRawAudio,
    /// Logs carry sensitive transcript content.
    LogsCarrySensitiveTranscript,
    /// Telemetry captured no metadata classes, so behavior is not explainable.
    TelemetryMetadataMissing,
    /// A transcript export decision is internally inconsistent.
    TranscriptInclusionInconsistent,
    /// An included transcript export was not user-reviewed.
    TranscriptExportNotExplicit,
    /// An included transcript export was not redacted.
    TranscriptExportNotRedacted,
    /// A transcript export decision is missing its user-visible label.
    TranscriptExportLabelMissing,
    /// Supportability convenience widened retention by default.
    SupportabilityWidenedRetention,
    /// A recorded drift moved toward a less-private posture.
    ProviderDriftNotTowardLocal,
    /// No session demonstrates failure diagnosability.
    FailureDiagnosabilityCaseMissing,
    /// No session demonstrates blocked-action diagnosability.
    BlockedActionDiagnosabilityCaseMissing,
    /// No session demonstrates provider-drift diagnosability.
    ProviderDriftDiagnosabilityCaseMissing,
    /// No transcript decision demonstrates the default-exclusion path.
    DefaultExclusionCaseMissing,
    /// No transcript decision demonstrates the explicit, redacted export path.
    ExplicitRedactedExportCaseMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
}

impl VoiceSupportExportViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::SessionRowIncomplete => "session_row_incomplete",
            Self::RawAudioNotExcluded => "raw_audio_not_excluded",
            Self::RawTranscriptNotExcluded => "raw_transcript_not_excluded",
            Self::KeyboardFallbackMissing => "keyboard_fallback_missing",
            Self::TelemetryCarriesRawAudio => "telemetry_carries_raw_audio",
            Self::TelemetryCarriesRawTranscript => "telemetry_carries_raw_transcript",
            Self::CrashPacketCarriesRawAudio => "crash_packet_carries_raw_audio",
            Self::LogsCarrySensitiveTranscript => "logs_carry_sensitive_transcript",
            Self::TelemetryMetadataMissing => "telemetry_metadata_missing",
            Self::TranscriptInclusionInconsistent => "transcript_inclusion_inconsistent",
            Self::TranscriptExportNotExplicit => "transcript_export_not_explicit",
            Self::TranscriptExportNotRedacted => "transcript_export_not_redacted",
            Self::TranscriptExportLabelMissing => "transcript_export_label_missing",
            Self::SupportabilityWidenedRetention => "supportability_widened_retention",
            Self::ProviderDriftNotTowardLocal => "provider_drift_not_toward_local",
            Self::FailureDiagnosabilityCaseMissing => "failure_diagnosability_case_missing",
            Self::BlockedActionDiagnosabilityCaseMissing => {
                "blocked_action_diagnosability_case_missing"
            }
            Self::ProviderDriftDiagnosabilityCaseMissing => {
                "provider_drift_diagnosability_case_missing"
            }
            Self::DefaultExclusionCaseMissing => "default_exclusion_case_missing",
            Self::ExplicitRedactedExportCaseMissing => "explicit_redacted_export_case_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
        }
    }
}

// ---------------------------------------------------------------------------
// Artifact helpers
// ---------------------------------------------------------------------------

/// Serializes a value as pretty JSON with a trailing newline (on-disk form).
///
/// # Errors
///
/// Returns the serializer error if the value cannot be serialized.
pub fn fixture_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let mut json = serde_json::to_string_pretty(value)?;
    json.push('\n');
    Ok(json)
}

/// Reads and validates the checked-in support-export packet artifact.
///
/// # Errors
///
/// Returns a list of violations if the checked-in artifact does not validate.
pub fn current_voice_support_export(
) -> Result<VoiceSupportExportPacket, Vec<VoiceSupportExportViolation>> {
    let packet: VoiceSupportExportPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/support/voice-session-export.json"
    )))
    .expect("checked-in voice support export parses");
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(violations)
    }
}

/// Writes the rendered Markdown report to `path`.
///
/// # Errors
///
/// Returns the IO error if the report cannot be written.
pub fn write_report(path: &Path, packet: &VoiceSupportExportPacket) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, packet.render_markdown())
}
