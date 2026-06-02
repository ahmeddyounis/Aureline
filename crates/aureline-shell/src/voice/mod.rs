//! Bounded voice-command and dictation preview surface (beta / preview).
//!
//! This module implements only the slice of speech input Aureline can
//! honestly support today: a bounded, inspectable preview/beta surface
//! that rides the frozen voice / dictation / speech-privacy contract in
//! [`docs/ux/voice_and_dictation_contract.md`](../../../../docs/ux/voice_and_dictation_contract.md)
//! and the canonical command graph owned by `aureline-commands`. It does
//! **not** mint a parallel command system, a hidden general assistant, or
//! an always-listening wake-word path.
//!
//! Every claimed voice row carries:
//!
//! - a **mic-state pill** ([`MicStatePill`]) that discloses active /
//!   inactive capture, command vs dictation mode, a local-or-hosted
//!   processing cue, mute/stop actions, and an accessibility label;
//! - a **transcript strip** ([`TranscriptStrip`]) and a
//!   **command-disambiguation sheet** ([`CommandDisambiguationSheet`])
//!   with confidence cues, edit/correct and confirm/cancel actions, and
//!   a preview state for risky commands;
//! - a **provider/privacy row** ([`ProviderPrivacyRow`]) (and, when a
//!   capability is degraded, a [`VoiceUnavailableBanner`]) that discloses
//!   the provider or local engine, the retention mode, the
//!   background-listening state, any policy lock/block note, the typed
//!   unavailable reason, and the keyboard fallback; and
//! - one [`VoiceCommandResolution`] per spoken command, which routes
//!   through the same stable `command_id`, capability scope, lifecycle
//!   label, disabled reason, preview/approval posture, result-packet
//!   schema, and [`NoBypassGuards`] the keyboard and command-palette
//!   lanes use.
//!
//! The surface is consumed by:
//!
//! - the live shell voice preview inspector;
//! - the headless inspector (`aureline_shell_voice_preview`), the only
//!   mint-from-truth path for the JSON fixtures checked in under
//!   `fixtures/ux/m3/voice_preview_and_privacy/`;
//! - the support-export wrapper a reviewer pivots from a support case
//!   to the row that flagged a finding;
//! - the markdown artifact under `artifacts/ux/m3/voice_preview_beta.md`
//!   (rendered from the same seed); and
//! - the CI gate `tools/ci/m3/voice_preview_check.py`.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures are bit-for-bit equal to the seed returned by
//! [`seeded_voice_preview_beta_page`].

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

pub use aureline_commands::invocation::NoBypassGuards;
pub use aureline_commands::{DisabledReasonCode, EnablementDecisionClass};

/// Schema version exported with every voice-preview record.
pub const VOICE_PREVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every voice-preview record.
pub const VOICE_PREVIEW_SHARED_CONTRACT_REF: &str = "shell:voice_preview_beta:v1";

/// Stable record kind for [`VoicePreviewBetaPage`] payloads.
pub const VOICE_PREVIEW_PAGE_RECORD_KIND: &str = "shell_voice_preview_beta_page_record";

/// Stable record kind for [`VoicePreviewRow`] payloads.
pub const VOICE_PREVIEW_ROW_RECORD_KIND: &str = "shell_voice_preview_beta_row_record";

/// Stable record kind for [`VoicePreviewSupportExport`] payloads.
pub const VOICE_PREVIEW_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_voice_preview_beta_support_export_record";

/// Stable page id quoted across surfaces.
pub const VOICE_PREVIEW_PAGE_ID: &str = "shell:voice_preview_beta:page:v1";

/// Stable support-export id quoted in the published wrapper.
pub const VOICE_PREVIEW_SUPPORT_EXPORT_ID: &str = "support-export:voice-preview:beta:001";

/// Path of the published markdown report.
pub const VOICE_PREVIEW_PUBLISHED_REPORT_REF: &str = "artifacts/ux/m3/voice_preview_beta.md";

/// Path of the published companion doc.
pub const VOICE_PREVIEW_PUBLISHED_DOC_REF: &str = "docs/ux/m3/voice_preview_beta.md";

/// Boundary schema ref for the voice session-state records.
pub const VOICE_SESSION_STATE_SCHEMA_REF: &str = "schemas/ux/voice_session_state.schema.json";

/// Boundary schema ref for the voice command-resolution records.
pub const VOICE_COMMAND_RESOLUTION_SCHEMA_REF: &str =
    "schemas/ux/voice_command_resolution.schema.json";

/// Canonical command result-packet schema voice resolutions reuse.
pub const COMMAND_RESULT_PACKET_SCHEMA_REF: &str =
    "schemas/commands/command_result_packet.schema.json";

/// Frozen voice/dictation/speech-privacy contract this surface rides.
pub const VOICE_AND_DICTATION_CONTRACT_REF: &str = "docs/ux/voice_and_dictation_contract.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-05-20T00:00:00Z";

/// Default redaction class used for representation-labeled voice rows.
const REDACTION_CLASS: &str = "metadata_safe_default";

/// Voice mode the surface is in. Mirrors `voice_mode_class` on the frozen
/// contract; command and dictation are always separate, explicit states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceModeClass {
    /// Microphone is off; no capture.
    IdleMicrophoneOff,
    /// Dictation mode: words become editor text on the shared undo stack.
    DictationModeActive,
    /// Command mode: words resolve to canonical command ids.
    CommandModeActive,
    /// Continuous listening; only valid with an explicit opt-in.
    ContinuousListeningActiveUserOptedIn,
    /// Voice mode is blocked by policy.
    VoiceModeBlockedByPolicy,
    /// Voice mode is blocked by the deployment envelope.
    VoiceModeBlockedByEnvelope,
}

impl VoiceModeClass {
    /// Stable schema token.
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

    /// `true` while audio is actively being captured.
    pub const fn is_capturing(self) -> bool {
        matches!(
            self,
            Self::DictationModeActive
                | Self::CommandModeActive
                | Self::ContinuousListeningActiveUserOptedIn
        )
    }
}

/// How a voice session is activated. Mirrors `activation_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceActivationClass {
    /// Push-to-talk hold (default explicit activation).
    PushToTalkHeld,
    /// Push-to-talk toggle.
    PushToTalkToggle,
    /// Wake-phrase continuous; only valid with an explicit opt-in.
    WakePhraseContinuousUserOptedIn,
    /// Manual command activation (e.g. command-mode button).
    ManualCommandActivation,
    /// Activation blocked by policy.
    ActivationBlockedByPolicy,
    /// Activation unavailable in the envelope.
    ActivationUnavailableInEnvelope,
}

impl VoiceActivationClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PushToTalkHeld => "push_to_talk_held",
            Self::PushToTalkToggle => "push_to_talk_toggle",
            Self::WakePhraseContinuousUserOptedIn => "wake_phrase_continuous_user_opted_in",
            Self::ManualCommandActivation => "manual_command_activation",
            Self::ActivationBlockedByPolicy => "activation_blocked_by_policy",
            Self::ActivationUnavailableInEnvelope => "activation_unavailable_in_envelope",
        }
    }

    /// `true` for activation classes that are explicit, user-initiated, and
    /// not implicitly always-on.
    pub const fn is_explicit(self) -> bool {
        matches!(
            self,
            Self::PushToTalkHeld | Self::PushToTalkToggle | Self::ManualCommandActivation
        )
    }
}

/// Persistent mic-indicator state. Mirrors `mic_indicator_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MicIndicatorClass {
    /// Indicator visible while capture is active.
    PersistentIndicatorVisibleCaptureActive,
    /// Indicator visible while capture is idle.
    PersistentIndicatorVisibleCaptureIdle,
    /// Indicator hidden because capture is disabled.
    PersistentIndicatorHiddenCaptureDisabled,
    /// Indicator unavailable/degraded.
    PersistentIndicatorUnavailableDegraded,
}

impl MicIndicatorClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersistentIndicatorVisibleCaptureActive => {
                "persistent_indicator_visible_capture_active"
            }
            Self::PersistentIndicatorVisibleCaptureIdle => {
                "persistent_indicator_visible_capture_idle"
            }
            Self::PersistentIndicatorHiddenCaptureDisabled => {
                "persistent_indicator_hidden_capture_disabled"
            }
            Self::PersistentIndicatorUnavailableDegraded => {
                "persistent_indicator_unavailable_degraded"
            }
        }
    }
}

/// Local-or-hosted processing cue shown on the mic pill and privacy row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessingLocalityCue {
    /// Audio processed on-device; no handoff.
    LocalOnDevice,
    /// Audio handed off to a hosted/remote engine, disclosed before use.
    HostedRemoteDisclosed,
    /// Processing unavailable in the current state.
    ProcessingUnavailable,
}

impl ProcessingLocalityCue {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnDevice => "local_on_device",
            Self::HostedRemoteDisclosed => "hosted_remote_disclosed",
            Self::ProcessingUnavailable => "processing_unavailable",
        }
    }
}

/// Command-preview posture. Mirrors `command_preview_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandPreviewClass {
    /// Preview required before applying privileged actions.
    PreviewRequiredForPrivilegedActions,
    /// Preview optional for reversible local actions.
    PreviewOptionalForReversibleLocalActions,
    /// Preview skipped for inert metadata-only actions.
    PreviewSkippedForInertMetadataOnly,
    /// Preview blocked by the envelope.
    PreviewBlockedByEnvelope,
}

impl CommandPreviewClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewRequiredForPrivilegedActions => "preview_required_for_privileged_actions",
            Self::PreviewOptionalForReversibleLocalActions => {
                "preview_optional_for_reversible_local_actions"
            }
            Self::PreviewSkippedForInertMetadataOnly => "preview_skipped_for_inert_metadata_only",
            Self::PreviewBlockedByEnvelope => "preview_blocked_by_envelope",
        }
    }
}

/// Transcript-correction posture. Mirrors `transcript_correction_posture`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptCorrectionPosture {
    /// Correction window required before commit.
    CorrectionRequiredBeforeCommit,
    /// Correction window optional before commit.
    CorrectionOptionalBeforeCommit,
    /// Correction unavailable; capture only.
    CorrectionUnavailableCaptureOnly,
    /// Correction blocked by the envelope.
    CorrectionBlockedByEnvelope,
}

impl TranscriptCorrectionPosture {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorrectionRequiredBeforeCommit => "correction_required_before_commit",
            Self::CorrectionOptionalBeforeCommit => "correction_optional_before_commit",
            Self::CorrectionUnavailableCaptureOnly => "correction_unavailable_capture_only",
            Self::CorrectionBlockedByEnvelope => "correction_blocked_by_envelope",
        }
    }
}

/// Confidence cue shown next to a transcript segment or candidate.
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
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Whether background listening is on. Default is off; only an explicit
/// opt-in can turn it on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackgroundListeningState {
    /// Off (default). Capture only happens during explicit activation.
    OffDefault,
    /// On, with an explicit per-profile or per-session opt-in.
    OnUserOptedIn,
    /// Blocked by policy.
    BlockedByPolicy,
}

impl BackgroundListeningState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OffDefault => "off_default",
            Self::OnUserOptedIn => "on_user_opted_in",
            Self::BlockedByPolicy => "blocked_by_policy",
        }
    }
}

/// Retention mode disclosed on the privacy row. Mirrors
/// `retention_stance_class`.
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
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAudioNoTranscriptRetained => "no_audio_retained_no_transcript_retained",
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
}

/// Typed reason a voice capability is unavailable. Drives the unavailable
/// banner and the privacy row's unavailable note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceUnavailableReason {
    /// Offline and no local engine is available.
    OfflineNoLocalEngine,
    /// Speech provider unavailable.
    ProviderUnavailable,
    /// No microphone device present or permitted.
    NoMicrophone,
    /// Environment too noisy for reliable capture.
    NoisyEnvironment,
    /// Policy lock or block disables voice in this context.
    PolicyLockedOrBlocked,
    /// Local speech pack is present but unverified.
    LocalPackUnverified,
    /// Workspace trust is restricted.
    TrustRestricted,
}

impl VoiceUnavailableReason {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfflineNoLocalEngine => "offline_no_local_engine",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::NoMicrophone => "no_microphone",
            Self::NoisyEnvironment => "noisy_environment",
            Self::PolicyLockedOrBlocked => "policy_locked_or_blocked",
            Self::LocalPackUnverified => "local_pack_unverified",
            Self::TrustRestricted => "trust_restricted",
        }
    }
}

/// Claim posture of a voice row against the M3 claimed-surface rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceClaimPosture {
    /// Aureline claims this row as a beta voice surface.
    ClaimedBeta,
    /// Aureline claims this row as a preview voice surface.
    ClaimedPreview,
    /// Unclaimed: voice stays Labs/unadvertised, suppressed by default.
    LabsUnadvertised,
}

impl VoiceClaimPosture {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimedBeta => "claimed_beta",
            Self::ClaimedPreview => "claimed_preview",
            Self::LabsUnadvertised => "labs_unadvertised",
        }
    }

    /// `true` for postures that claim a first-class beta/preview surface.
    pub const fn is_claimed(self) -> bool {
        matches!(self, Self::ClaimedBeta | Self::ClaimedPreview)
    }
}

/// Capability scope of a resolved command. Re-exports the canonical
/// command capability vocabulary so high-impact actions are recognized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceCapabilityScope {
    /// Inert metadata route (no state change).
    InertMetadataOnly,
    /// Reversible local read.
    ReversibleLocalRead,
    /// Reversible local mutation (undoable without rollback).
    ReversibleLocalMutation,
    /// Recoverable durable mutation (requires a rollback handle).
    RecoverableDurableMutation,
    /// Destructive bulk mutation (multi-file, multi-record).
    DestructiveBulkMutation,
    /// Irreversible publish / network mutation.
    IrreversiblePublish,
}

impl VoiceCapabilityScope {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InertMetadataOnly => "inert_metadata_only",
            Self::ReversibleLocalRead => "reversible_local_read",
            Self::ReversibleLocalMutation => "reversible_local_mutation",
            Self::RecoverableDurableMutation => "recoverable_durable_mutation",
            Self::DestructiveBulkMutation => "destructive_bulk_mutation",
            Self::IrreversiblePublish => "irreversible_publish",
        }
    }

    /// `true` for high-impact scopes that MUST preserve the preview /
    /// approval / trust / audit path a keyboard invocation rides.
    pub const fn is_high_impact(self) -> bool {
        matches!(
            self,
            Self::RecoverableDurableMutation
                | Self::DestructiveBulkMutation
                | Self::IrreversiblePublish
        )
    }
}

/// How a spoken phrase resolves. Mirrors `voice_intent_resolution_class`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceCommandResolutionClass {
    /// Resolves to exactly one canonical command id.
    ResolvesToCanonicalCommandId,
    /// Resolves to dictation text only (no command).
    ResolvesToDictationTextOnly,
    /// Ambiguous: a disambiguation sheet is required.
    ResolvesToDisambiguationRequired,
    /// Denied: the verb is not on the registry.
    ResolutionDeniedUncanonicalVerb,
    /// Denied: confidence is below the floor.
    ResolutionDeniedLowConfidence,
    /// Blocked by the envelope (policy/trust/offline).
    ResolutionBlockedByEnvelope,
}

impl VoiceCommandResolutionClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvesToCanonicalCommandId => "resolves_to_canonical_command_id",
            Self::ResolvesToDictationTextOnly => "resolves_to_dictation_text_only",
            Self::ResolvesToDisambiguationRequired => "resolves_to_disambiguation_required",
            Self::ResolutionDeniedUncanonicalVerb => "resolution_denied_uncanonical_verb",
            Self::ResolutionDeniedLowConfidence => "resolution_denied_low_confidence",
            Self::ResolutionBlockedByEnvelope => "resolution_blocked_by_envelope",
        }
    }

    /// `true` when this class is expected to resolve to a canonical command.
    pub const fn binds_command_id(self) -> bool {
        matches!(self, Self::ResolvesToCanonicalCommandId)
    }
}

/// Lifecycle label of a resolved command. Surfaces project the same label
/// across keyboard, palette, and voice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceLifecycleLabel {
    /// Generally available.
    Stable,
    /// Beta lane.
    Beta,
    /// Preview lane.
    Preview,
    /// Deprecated; surfaces show the replacement command id.
    Deprecated,
}

impl VoiceLifecycleLabel {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Mic-state pill: the persistent cue rendered whenever capture is active.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MicStatePill {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable pill id.
    pub pill_id: String,
    /// `true` when audio is actively being captured.
    pub capture_active: bool,
    /// Active voice mode (command vs dictation made explicit).
    pub voice_mode_class: VoiceModeClass,
    /// Persistent mic-indicator state.
    pub mic_indicator_class: MicIndicatorClass,
    /// Local-or-hosted processing cue.
    pub processing_locality_cue: ProcessingLocalityCue,
    /// Canonical command id for the mute action.
    pub mute_action_command_id: String,
    /// Canonical command id for the stop action.
    pub stop_action_command_id: String,
    /// Accessibility label ref narrated by the screen reader.
    pub accessibility_label_ref: String,
    /// `true` when the pill and its actions are reachable by keyboard.
    pub keyboard_reachable: bool,
    /// Layout target the pill renders in (status zone, editor adjunct, …).
    pub layout_target_class: String,
    /// Redaction class.
    pub redaction_class: String,
}

/// Transcript strip with confidence and correction actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptStrip {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable strip id.
    pub strip_id: String,
    /// Representation label for the transcript (never raw spoken bytes).
    pub transcript_text_label_ref: String,
    /// Confidence cue for the segment.
    pub confidence_cue: ConfidenceCue,
    /// Transcript-correction posture.
    pub transcript_correction_posture: TranscriptCorrectionPosture,
    /// Canonical command id for the edit action.
    pub edit_command_id: String,
    /// Canonical command id for the correct action.
    pub correct_command_id: String,
    /// Canonical command id for the confirm action.
    pub confirm_command_id: String,
    /// Canonical command id for the cancel action.
    pub cancel_command_id: String,
    /// Accessibility label ref.
    pub accessibility_label_ref: String,
    /// Redaction class.
    pub redaction_class: String,
}

/// One candidate row on a disambiguation sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDisambiguationCandidate {
    /// Canonical command id this candidate would invoke.
    pub candidate_command_id: String,
    /// Candidate primary label ref.
    pub candidate_label_ref: String,
    /// Confidence cue for the candidate.
    pub confidence_cue: ConfidenceCue,
    /// Capability scope of the candidate command.
    pub capability_scope_class: VoiceCapabilityScope,
    /// `true` when this candidate would require a preview before apply.
    pub preview_required: bool,
}

/// Command-disambiguation sheet with confidence cues and a preview state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDisambiguationSheet {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable sheet id.
    pub sheet_id: String,
    /// Candidate rows, in canonical order.
    pub candidates: Vec<CommandDisambiguationCandidate>,
    /// Canonical command id for the confirm action.
    pub confirm_command_id: String,
    /// Canonical command id for the cancel action.
    pub cancel_command_id: String,
    /// Preview state shown for risky candidates.
    pub preview_state_for_risky: String,
    /// Accessibility label ref.
    pub accessibility_label_ref: String,
    /// Redaction class.
    pub redaction_class: String,
}

/// Provider/privacy row disclosing locality, retention, and fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderPrivacyRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Provider or local-engine label ref.
    pub provider_or_local_engine_label_ref: String,
    /// Local-or-hosted processing cue.
    pub processing_locality_cue: ProcessingLocalityCue,
    /// Retention mode.
    pub retention_mode: RetentionMode,
    /// Background-listening state.
    pub background_listening_state: BackgroundListeningState,
    /// Policy lock/block note ref, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_lock_or_block_note_ref: Option<String>,
    /// Typed unavailable reason, when the capability is degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<VoiceUnavailableReason>,
    /// `true` when a keyboard fallback is available.
    pub keyboard_fallback_available: bool,
    /// Canonical command id of the keyboard fallback.
    pub keyboard_fallback_command_id: String,
    /// Accessibility label ref.
    pub accessibility_label_ref: String,
    /// Redaction class.
    pub redaction_class: String,
}

/// Unavailable banner shown when a voice capability is degraded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceUnavailableBanner {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable banner id.
    pub banner_id: String,
    /// Typed unavailable reason.
    pub unavailable_reason: VoiceUnavailableReason,
    /// Message label ref.
    pub message_ref: String,
    /// Canonical command id of the keyboard fallback.
    pub keyboard_fallback_command_id: String,
    /// Accessibility label ref.
    pub accessibility_label_ref: String,
    /// Redaction class.
    pub redaction_class: String,
}

/// One spoken-command resolution routed through the canonical command
/// graph. This is the parity-proof object: every field that a keyboard or
/// palette invocation would carry is carried here, verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceCommandResolution {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable resolution id.
    pub resolution_id: String,
    /// Representation label for the spoken phrase (never raw bytes).
    pub spoken_phrase_label_ref: String,
    /// Confidence cue.
    pub confidence_cue: ConfidenceCue,
    /// Resolution class.
    pub resolution_class: VoiceCommandResolutionClass,
    /// Canonical command id (present when the class binds one).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_command_id: Option<String>,
    /// Descriptor revision the resolution was produced against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_revision_ref: Option<String>,
    /// Dotted canonical verb resolved from the descriptor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_verb: Option<String>,
    /// Lifecycle label projected from the descriptor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_label: Option<VoiceLifecycleLabel>,
    /// Capability scope of the resolved command.
    pub capability_scope_class: VoiceCapabilityScope,
    /// Preview class declared on the descriptor.
    pub preview_class_declared: String,
    /// Approval posture class declared on the descriptor.
    pub approval_posture_class_declared: String,
    /// Enablement decision (shared with keyboard/palette).
    pub enablement_decision_class: EnablementDecisionClass,
    /// Disabled reason code (shared vocabulary), when disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_code: Option<DisabledReasonCode>,
    /// `true` when this resolution requires a preview before apply.
    pub preview_required: bool,
    /// `true` when this resolution requires approval before apply.
    pub approval_required: bool,
    /// Result-packet schema ref reused from the command graph.
    pub result_packet_schema_ref: String,
    /// Parity-expectation ref the result packet asserts.
    pub parity_expectation_ref: String,
    /// No-bypass guards asserted on this resolution.
    pub no_bypass_guards: NoBypassGuards,
    /// Canonical docs/help anchor ref.
    pub docs_help_anchor_ref: String,
    /// The same command id reachable from the keyboard lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyboard_equivalent_command_id: Option<String>,
    /// Redaction class.
    pub redaction_class: String,
}

impl VoiceCommandResolution {
    /// `true` when this resolution drives a high-impact mutation.
    pub const fn is_high_impact(&self) -> bool {
        self.capability_scope_class.is_high_impact()
    }
}

/// Blocking finding emitted by the validator against a voice row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum VoiceBlockingFinding {
    /// A claimed row does not make command mode explicit.
    CommandModeNotExplicit { row_id: String },
    /// A claimed row does not make dictation mode explicit.
    DictationModeNotExplicit { row_id: String },
    /// A claimed row is not keyboard reachable.
    NotKeyboardReachable { row_id: String },
    /// A claimed row is not screen-reader narratable.
    NotScreenReaderNarratable { row_id: String },
    /// A claimed, capturing row is missing its mic-state pill.
    MicPillMissingWhileClaimed { row_id: String },
    /// A claimed, capturing row hides the mic indicator during capture.
    MicIndicatorHiddenDuringCapture { row_id: String },
    /// Capture is implicitly always-on (non-explicit default activation).
    ImplicitAlwaysOnCapture { row_id: String },
    /// Background listening is on without an explicit opt-in.
    BackgroundListeningWithoutOptIn { row_id: String },
    /// Provider/privacy state is not disclosed.
    ProviderPrivacyStateHidden { row_id: String },
    /// An unavailable state offers no keyboard fallback.
    UnavailableWithoutKeyboardFallback { row_id: String },
    /// A high-impact resolution skips the required preview.
    HighImpactPreviewBypassed {
        row_id: String,
        resolution_id: String,
    },
    /// A high-impact resolution weakens a strict no-bypass guard.
    NoBypassGuardWeakened {
        row_id: String,
        resolution_id: String,
        guard: String,
    },
    /// A mutating resolution does not bind a canonical command id.
    ResolutionMissingCommandId {
        row_id: String,
        resolution_id: String,
    },
    /// A resolution resolves to a verb outside the registry.
    ResolutionUncanonicalVerb {
        row_id: String,
        resolution_id: String,
    },
    /// A disabled resolution drops the typed disabled reason.
    DisabledResolutionMissingReason {
        row_id: String,
        resolution_id: String,
    },
    /// A Labs/unadvertised row advertises broad support.
    LabsRowAdvertisesBroadSupport { row_id: String },
}

impl VoiceBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::CommandModeNotExplicit { .. } => "command_mode_not_explicit",
            Self::DictationModeNotExplicit { .. } => "dictation_mode_not_explicit",
            Self::NotKeyboardReachable { .. } => "not_keyboard_reachable",
            Self::NotScreenReaderNarratable { .. } => "not_screen_reader_narratable",
            Self::MicPillMissingWhileClaimed { .. } => "mic_pill_missing_while_claimed",
            Self::MicIndicatorHiddenDuringCapture { .. } => "mic_indicator_hidden_during_capture",
            Self::ImplicitAlwaysOnCapture { .. } => "implicit_always_on_capture",
            Self::BackgroundListeningWithoutOptIn { .. } => "background_listening_without_opt_in",
            Self::ProviderPrivacyStateHidden { .. } => "provider_privacy_state_hidden",
            Self::UnavailableWithoutKeyboardFallback { .. } => {
                "unavailable_without_keyboard_fallback"
            }
            Self::HighImpactPreviewBypassed { .. } => "high_impact_preview_bypassed",
            Self::NoBypassGuardWeakened { .. } => "no_bypass_guard_weakened",
            Self::ResolutionMissingCommandId { .. } => "resolution_missing_command_id",
            Self::ResolutionUncanonicalVerb { .. } => "resolution_uncanonical_verb",
            Self::DisabledResolutionMissingReason { .. } => "disabled_resolution_missing_reason",
            Self::LabsRowAdvertisesBroadSupport { .. } => "labs_row_advertises_broad_support",
        }
    }

    /// Returns the row id this finding is attached to.
    pub fn row_id(&self) -> &str {
        match self {
            Self::CommandModeNotExplicit { row_id }
            | Self::DictationModeNotExplicit { row_id }
            | Self::NotKeyboardReachable { row_id }
            | Self::NotScreenReaderNarratable { row_id }
            | Self::MicPillMissingWhileClaimed { row_id }
            | Self::MicIndicatorHiddenDuringCapture { row_id }
            | Self::ImplicitAlwaysOnCapture { row_id }
            | Self::BackgroundListeningWithoutOptIn { row_id }
            | Self::ProviderPrivacyStateHidden { row_id }
            | Self::UnavailableWithoutKeyboardFallback { row_id }
            | Self::HighImpactPreviewBypassed { row_id, .. }
            | Self::NoBypassGuardWeakened { row_id, .. }
            | Self::ResolutionMissingCommandId { row_id, .. }
            | Self::ResolutionUncanonicalVerb { row_id, .. }
            | Self::DisabledResolutionMissingReason { row_id, .. }
            | Self::LabsRowAdvertisesBroadSupport { row_id } => row_id,
        }
    }
}

/// One per-surface voice preview row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoicePreviewRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Surface label ref.
    pub surface_label_ref: String,
    /// Claim posture (claimed beta/preview vs labs/unadvertised).
    pub claim_posture: VoiceClaimPosture,
    /// `true` when command mode is an explicit, reachable state.
    pub command_mode_explicit: bool,
    /// `true` when dictation mode is an explicit, reachable state.
    pub dictation_mode_explicit: bool,
    /// `true` when the row and its actions are keyboard reachable.
    pub keyboard_reachable: bool,
    /// `true` when the row is screen-reader narratable.
    pub screen_reader_narratable: bool,
    /// Default activation class (explicit push-to-talk by default).
    pub default_activation_class: VoiceActivationClass,
    /// Background-listening state.
    pub background_listening_state: BackgroundListeningState,
    /// Mic-state pill, present on claimed capturing rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mic_pill: Option<MicStatePill>,
    /// Transcript strip, present on claimed rows that surface transcripts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript_strip: Option<TranscriptStrip>,
    /// Command-disambiguation sheet, present on claimed command rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disambiguation_sheet: Option<CommandDisambiguationSheet>,
    /// Provider/privacy row (always disclosed).
    pub provider_privacy_row: ProviderPrivacyRow,
    /// Unavailable banner, present when a capability is degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_banner: Option<VoiceUnavailableBanner>,
    /// Spoken-command resolutions on this row, in canonical order.
    pub command_resolutions: Vec<VoiceCommandResolution>,
    /// Canonical docs/help anchor ref for the row.
    pub docs_help_anchor_ref: String,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<VoiceBlockingFinding>,
}

/// Const-true invariants every voice-preview page declares.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoicePreviewInvariantManifest {
    /// Command and dictation modes are separate, explicit states.
    pub command_and_dictation_modes_are_explicit: bool,
    /// Claimed voice rows are keyboard reachable and screen-reader narratable.
    pub claimed_rows_keyboard_reachable_and_screen_reader_narratable: bool,
    /// Capture is never implicitly always-on.
    pub capture_is_never_implicitly_always_on: bool,
    /// The persistent mic indicator is visible during active capture.
    pub persistent_mic_indicator_visible_during_active_capture: bool,
    /// Provider/privacy state is disclosed before or during capture.
    pub provider_privacy_state_disclosed_before_or_during_capture: bool,
    /// High-impact voice actions cannot bypass preview/approval/trust/audit.
    pub high_impact_actions_cannot_bypass_preview_approval_trust_audit: bool,
    /// Spoken commands resolve through the same canonical command ids.
    pub spoken_commands_resolve_through_canonical_command_ids: bool,
    /// Unavailable states always provide a keyboard fallback.
    pub unavailable_states_always_provide_keyboard_fallback: bool,
    /// Unclaimed rows stay Labs/unadvertised and imply no broad support.
    pub unclaimed_rows_stay_labs_unadvertised: bool,
}

impl VoicePreviewInvariantManifest {
    /// The const-true manifest declared on a conforming page.
    pub const fn all_true() -> Self {
        Self {
            command_and_dictation_modes_are_explicit: true,
            claimed_rows_keyboard_reachable_and_screen_reader_narratable: true,
            capture_is_never_implicitly_always_on: true,
            persistent_mic_indicator_visible_during_active_capture: true,
            provider_privacy_state_disclosed_before_or_during_capture: true,
            high_impact_actions_cannot_bypass_preview_approval_trust_audit: true,
            spoken_commands_resolve_through_canonical_command_ids: true,
            unavailable_states_always_provide_keyboard_fallback: true,
            unclaimed_rows_stay_labs_unadvertised: true,
        }
    }
}

/// Page-level summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoicePreviewSummary {
    /// Number of rows on the page.
    pub row_count: usize,
    /// Number of claimed (beta/preview) rows.
    pub claimed_row_count: usize,
    /// Number of Labs/unadvertised rows.
    pub labs_row_count: usize,
    /// Number of rows currently capturing audio.
    pub capturing_row_count: usize,
    /// Number of rows in an unavailable state with a keyboard fallback.
    pub unavailable_with_fallback_count: usize,
    /// Number of spoken-command resolutions across all rows.
    pub resolution_count: usize,
    /// Number of high-impact resolutions across all rows.
    pub high_impact_resolution_count: usize,
    /// Total blocking findings across the page.
    pub total_blocking_findings: usize,
    /// Claim-posture tokens present on the page.
    pub claim_postures_present: Vec<String>,
    /// Voice-mode tokens present on the page.
    pub voice_modes_present: Vec<String>,
}

/// Bounded voice-command and dictation preview page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoicePreviewBetaPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Frozen voice/dictation/speech-privacy contract this page rides.
    pub voice_and_dictation_contract_ref: String,
    /// Session-state boundary schema ref.
    pub session_state_schema_ref: String,
    /// Command-resolution boundary schema ref.
    pub command_resolution_schema_ref: String,
    /// Per-surface rows, sorted by `row_id`.
    pub rows: Vec<VoicePreviewRow>,
    /// Const-true invariants.
    pub invariants: VoicePreviewInvariantManifest,
    /// Page-level summary.
    pub summary: VoicePreviewSummary,
    /// `true` when there are zero blocking findings.
    pub page_clean: bool,
    /// Markdown publication ref.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the page can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the page can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the page was generated.
    pub generated_at: String,
}

impl VoicePreviewBetaPage {
    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "page: rows={}, claimed={}, labs={}, capturing={}, resolutions={}, high_impact={}, blocking={}, clean={}",
            self.summary.row_count,
            self.summary.claimed_row_count,
            self.summary.labs_row_count,
            self.summary.capturing_row_count,
            self.summary.resolution_count,
            self.summary.high_impact_resolution_count,
            self.summary.total_blocking_findings,
            self.page_clean,
        ));
        for row in &self.rows {
            lines.push(format!(
                "{}: posture={}, command_mode={}, dictation_mode={}, default_activation={}, background_listening={}, resolutions={}",
                row.row_id,
                row.claim_posture.as_str(),
                row.command_mode_explicit,
                row.dictation_mode_explicit,
                row.default_activation_class.as_str(),
                row.background_listening_state.as_str(),
                row.command_resolutions.len(),
            ));
            for finding in &row.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {}",
                    finding.class_token(),
                    finding.row_id(),
                ));
            }
        }
        lines
    }

    /// Renders the markdown report artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Bounded voice preview and privacy (beta)\n\n");
        out.push_str(
            "Generated from the seeded voice-preview projection in\n\
             [`crate::voice`](../../../crates/aureline-shell/src/voice/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- report-md > \\\n  artifacts/ux/m3/voice_preview_beta.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Page id: `{}`\n", self.page_id));
        out.push_str(&format!(
            "- Contract: `{}`\n",
            self.voice_and_dictation_contract_ref
        ));
        out.push_str(&format!(
            "- Session-state schema: `{}`\n",
            self.session_state_schema_ref
        ));
        out.push_str(&format!(
            "- Command-resolution schema: `{}`\n",
            self.command_resolution_schema_ref
        ));
        out.push_str(&format!("- Rows: `{}`\n", self.summary.row_count));
        out.push_str(&format!(
            "- Claimed beta/preview rows: `{}`\n",
            self.summary.claimed_row_count
        ));
        out.push_str(&format!(
            "- Labs/unadvertised rows: `{}`\n",
            self.summary.labs_row_count
        ));
        out.push_str(&format!(
            "- Spoken-command resolutions: `{}` (high-impact: `{}`)\n",
            self.summary.resolution_count, self.summary.high_impact_resolution_count
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.page_clean { "clean" } else { "blocked" }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Rows\n\n");
        out.push_str(
            "| Row | Posture | Command mode | Dictation mode | Default activation | Background listening | Capturing | Resolutions |\n\
             | --- | ------- | ------------ | -------------- | ------------------ | -------------------- | --------- | ----------: |\n",
        );
        for row in &self.rows {
            let capturing = row
                .mic_pill
                .as_ref()
                .map(|pill| pill.capture_active)
                .unwrap_or(false);
            out.push_str(&format!(
                "| `{}` | `{}` | {} | {} | `{}` | `{}` | {} | {} |\n",
                row.row_id,
                row.claim_posture.as_str(),
                row.command_mode_explicit,
                row.dictation_mode_explicit,
                row.default_activation_class.as_str(),
                row.background_listening_state.as_str(),
                capturing,
                row.command_resolutions.len(),
            ));
        }
        out.push('\n');

        out.push_str("## Command-graph parity\n\n");
        out.push_str(
            "Every spoken command resolves through the same canonical command id, capability \
             scope, lifecycle label, disabled reason, preview/approval posture, and result-packet \
             schema as the keyboard and command-palette lanes.\n\n",
        );
        out.push_str(
            "| Resolution | Command id | Scope | Preview required | Approval required | Enablement | Disabled reason |\n\
             | ---------- | ---------- | ----- | ---------------- | ----------------- | ---------- | --------------- |\n",
        );
        for row in &self.rows {
            for resolution in &row.command_resolutions {
                let command_id = resolution.canonical_command_id.as_deref().unwrap_or("-");
                let disabled = resolution
                    .disabled_reason_code
                    .map(|code| code.as_str())
                    .unwrap_or("-");
                out.push_str(&format!(
                    "| `{}` | `{}` | `{}` | {} | {} | `{}` | `{}` |\n",
                    resolution.resolution_id,
                    command_id,
                    resolution.capability_scope_class.as_str(),
                    resolution.preview_required,
                    resolution.approval_required,
                    resolution.enablement_decision_class.as_str(),
                    disabled,
                ));
            }
        }
        out.push('\n');

        out.push_str("## Privacy and availability\n\n");
        out.push_str(
            "| Row | Processing | Retention | Background listening | Unavailable reason | Keyboard fallback |\n\
             | --- | ---------- | --------- | -------------------- | ------------------ | ----------------- |\n",
        );
        for row in &self.rows {
            let privacy = &row.provider_privacy_row;
            let unavailable = privacy
                .unavailable_reason
                .map(|reason| reason.as_str())
                .unwrap_or("-");
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.row_id,
                privacy.processing_locality_cue.as_str(),
                privacy.retention_mode.as_str(),
                privacy.background_listening_state.as_str(),
                unavailable,
                privacy.keyboard_fallback_available,
            ));
        }
        out.push('\n');

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test voice_preview_beta_fixtures\n");
        out.push_str("python3 tools/ci/m3/voice_preview_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the voice-preview page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoicePreviewSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// `true` when no raw audio or raw transcript bytes cross this boundary.
    pub raw_audio_or_transcript_bytes_excluded: bool,
    /// Voice-preview page quoted in full.
    pub page: VoicePreviewBetaPage,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl VoicePreviewSupportExport {
    /// Builds the support-export wrapper for a voice-preview page.
    pub fn from_page(support_export_id: impl Into<String>, page: VoicePreviewBetaPage) -> Self {
        let mut case_ids = vec![page.page_id.clone()];
        for row in &page.rows {
            case_ids.push(row.row_id.clone());
            for resolution in &row.command_resolutions {
                if let Some(command_id) = &resolution.canonical_command_id {
                    case_ids.push(command_id.clone());
                }
            }
        }
        Self {
            record_kind: VOICE_PREVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: VOICE_PREVIEW_SCHEMA_VERSION,
            shared_contract_ref: VOICE_PREVIEW_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            raw_audio_or_transcript_bytes_excluded: true,
            page,
            case_ids,
        }
    }
}

/// Computes the per-row blocking findings.
fn compute_row_findings(row: &VoicePreviewRow) -> Vec<VoiceBlockingFinding> {
    let mut findings = Vec::new();
    let row_id = row.row_id.clone();
    let claimed = row.claim_posture.is_claimed();
    let capturing = row
        .mic_pill
        .as_ref()
        .map(|pill| pill.capture_active)
        .unwrap_or(false);

    if claimed {
        if !row.command_mode_explicit {
            findings.push(VoiceBlockingFinding::CommandModeNotExplicit {
                row_id: row_id.clone(),
            });
        }
        if !row.dictation_mode_explicit {
            findings.push(VoiceBlockingFinding::DictationModeNotExplicit {
                row_id: row_id.clone(),
            });
        }
        if !row.keyboard_reachable {
            findings.push(VoiceBlockingFinding::NotKeyboardReachable {
                row_id: row_id.clone(),
            });
        }
        if !row.screen_reader_narratable {
            findings.push(VoiceBlockingFinding::NotScreenReaderNarratable {
                row_id: row_id.clone(),
            });
        }
        // Capture must be explicitly activated, never implicitly always-on.
        if !row.default_activation_class.is_explicit() {
            findings.push(VoiceBlockingFinding::ImplicitAlwaysOnCapture {
                row_id: row_id.clone(),
            });
        }
        match &row.mic_pill {
            None => {
                findings.push(VoiceBlockingFinding::MicPillMissingWhileClaimed {
                    row_id: row_id.clone(),
                });
            }
            Some(pill) => {
                if pill.capture_active
                    && pill.mic_indicator_class
                        != MicIndicatorClass::PersistentIndicatorVisibleCaptureActive
                {
                    findings.push(VoiceBlockingFinding::MicIndicatorHiddenDuringCapture {
                        row_id: row_id.clone(),
                    });
                }
            }
        }
    } else {
        // Labs/unadvertised rows must not advertise broad support: no
        // active capture, no claimed resolutions, indicator hidden.
        let advertises = capturing
            || !row.command_resolutions.is_empty()
            || row.background_listening_state == BackgroundListeningState::OnUserOptedIn
            || row
                .mic_pill
                .as_ref()
                .map(|pill| {
                    pill.mic_indicator_class
                        != MicIndicatorClass::PersistentIndicatorHiddenCaptureDisabled
                })
                .unwrap_or(false);
        if advertises {
            findings.push(VoiceBlockingFinding::LabsRowAdvertisesBroadSupport {
                row_id: row_id.clone(),
            });
        }
    }

    // Background listening on without an explicit opt-in is implicit capture.
    if row.background_listening_state == BackgroundListeningState::OnUserOptedIn
        && row.default_activation_class != VoiceActivationClass::WakePhraseContinuousUserOptedIn
    {
        findings.push(VoiceBlockingFinding::BackgroundListeningWithoutOptIn {
            row_id: row_id.clone(),
        });
    }

    // Provider/privacy state must always be disclosed.
    let privacy = &row.provider_privacy_row;
    if privacy.provider_or_local_engine_label_ref.trim().is_empty() {
        findings.push(VoiceBlockingFinding::ProviderPrivacyStateHidden {
            row_id: row_id.clone(),
        });
    }

    // An unavailable state must always offer a keyboard fallback.
    let unavailable = privacy.unavailable_reason.is_some() || row.unavailable_banner.is_some();
    if unavailable && !privacy.keyboard_fallback_available {
        findings.push(VoiceBlockingFinding::UnavailableWithoutKeyboardFallback {
            row_id: row_id.clone(),
        });
    }

    // Per-resolution parity checks.
    for resolution in &row.command_resolutions {
        if resolution.is_high_impact() {
            if !resolution.preview_required {
                findings.push(VoiceBlockingFinding::HighImpactPreviewBypassed {
                    row_id: row_id.clone(),
                    resolution_id: resolution.resolution_id.clone(),
                });
            }
            let guards = &resolution.no_bypass_guards;
            for (name, value) in [
                (
                    "trust_revalidation_required",
                    guards.trust_revalidation_required,
                ),
                (
                    "policy_revalidation_required",
                    guards.policy_revalidation_required,
                ),
                ("preview_path_preserved", guards.preview_path_preserved),
                ("approval_path_preserved", guards.approval_path_preserved),
                (
                    "permission_prompt_revalidation_required",
                    guards.permission_prompt_revalidation_required,
                ),
                (
                    "capability_class_may_not_widen",
                    guards.capability_class_may_not_widen,
                ),
                (
                    "result_schema_may_not_be_replaced",
                    guards.result_schema_may_not_be_replaced,
                ),
            ] {
                if !value {
                    findings.push(VoiceBlockingFinding::NoBypassGuardWeakened {
                        row_id: row_id.clone(),
                        resolution_id: resolution.resolution_id.clone(),
                        guard: name.to_owned(),
                    });
                }
            }
        }

        match resolution.resolution_class {
            VoiceCommandResolutionClass::ResolvesToCanonicalCommandId => {
                if resolution.canonical_command_id.is_none() {
                    findings.push(VoiceBlockingFinding::ResolutionMissingCommandId {
                        row_id: row_id.clone(),
                        resolution_id: resolution.resolution_id.clone(),
                    });
                }
            }
            VoiceCommandResolutionClass::ResolutionDeniedUncanonicalVerb => {
                findings.push(VoiceBlockingFinding::ResolutionUncanonicalVerb {
                    row_id: row_id.clone(),
                    resolution_id: resolution.resolution_id.clone(),
                });
            }
            _ => {
                // A mutating resolution must bind a canonical command id; it
                // cannot mutate the workspace through a private path.
                if resolution.capability_scope_class != VoiceCapabilityScope::InertMetadataOnly
                    && resolution.capability_scope_class
                        != VoiceCapabilityScope::ReversibleLocalRead
                    && resolution.canonical_command_id.is_none()
                    && resolution.resolution_class
                        != VoiceCommandResolutionClass::ResolvesToDictationTextOnly
                {
                    findings.push(VoiceBlockingFinding::ResolutionMissingCommandId {
                        row_id: row_id.clone(),
                        resolution_id: resolution.resolution_id.clone(),
                    });
                }
            }
        }

        if resolution.enablement_decision_class != EnablementDecisionClass::Enabled
            && resolution.disabled_reason_code.is_none()
        {
            findings.push(VoiceBlockingFinding::DisabledResolutionMissingReason {
                row_id: row_id.clone(),
                resolution_id: resolution.resolution_id.clone(),
            });
        }
    }

    findings
}

/// Builds a [`VoicePreviewRow`] computing its blocking findings.
pub fn build_voice_preview_row(mut row: VoicePreviewRow) -> VoicePreviewRow {
    row.record_kind = VOICE_PREVIEW_ROW_RECORD_KIND.to_owned();
    row.schema_version = VOICE_PREVIEW_SCHEMA_VERSION;
    row.shared_contract_ref = VOICE_PREVIEW_SHARED_CONTRACT_REF.to_owned();
    row.blocking_findings = compute_row_findings(&row);
    row
}

/// Builds a full [`VoicePreviewBetaPage`] from per-surface rows.
pub fn build_voice_preview_beta_page(rows: Vec<VoicePreviewRow>) -> VoicePreviewBetaPage {
    let mut rows: Vec<VoicePreviewRow> = rows.into_iter().map(build_voice_preview_row).collect();
    rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));

    let row_count = rows.len();
    let claimed_row_count = rows
        .iter()
        .filter(|row| row.claim_posture.is_claimed())
        .count();
    let labs_row_count = row_count - claimed_row_count;
    let capturing_row_count = rows
        .iter()
        .filter(|row| {
            row.mic_pill
                .as_ref()
                .map(|pill| pill.capture_active)
                .unwrap_or(false)
        })
        .count();
    let unavailable_with_fallback_count = rows
        .iter()
        .filter(|row| {
            row.provider_privacy_row.unavailable_reason.is_some()
                && row.provider_privacy_row.keyboard_fallback_available
        })
        .count();
    let resolution_count = rows
        .iter()
        .map(|row| row.command_resolutions.len())
        .sum::<usize>();
    let high_impact_resolution_count = rows
        .iter()
        .flat_map(|row| row.command_resolutions.iter())
        .filter(|resolution| resolution.is_high_impact())
        .count();
    let total_blocking_findings = rows
        .iter()
        .map(|row| row.blocking_findings.len())
        .sum::<usize>();

    let mut claim_postures: BTreeSet<&str> = BTreeSet::new();
    let mut voice_modes: BTreeSet<&str> = BTreeSet::new();
    for row in &rows {
        claim_postures.insert(row.claim_posture.as_str());
        if let Some(pill) = &row.mic_pill {
            voice_modes.insert(pill.voice_mode_class.as_str());
        }
    }

    let summary = VoicePreviewSummary {
        row_count,
        claimed_row_count,
        labs_row_count,
        capturing_row_count,
        unavailable_with_fallback_count,
        resolution_count,
        high_impact_resolution_count,
        total_blocking_findings,
        claim_postures_present: claim_postures.into_iter().map(str::to_owned).collect(),
        voice_modes_present: voice_modes.into_iter().map(str::to_owned).collect(),
    };

    let page_clean = total_blocking_findings == 0;

    VoicePreviewBetaPage {
        record_kind: VOICE_PREVIEW_PAGE_RECORD_KIND.to_owned(),
        schema_version: VOICE_PREVIEW_SCHEMA_VERSION,
        shared_contract_ref: VOICE_PREVIEW_SHARED_CONTRACT_REF.to_owned(),
        page_id: VOICE_PREVIEW_PAGE_ID.to_owned(),
        voice_and_dictation_contract_ref: VOICE_AND_DICTATION_CONTRACT_REF.to_owned(),
        session_state_schema_ref: VOICE_SESSION_STATE_SCHEMA_REF.to_owned(),
        command_resolution_schema_ref: VOICE_COMMAND_RESOLUTION_SCHEMA_REF.to_owned(),
        rows,
        invariants: VoicePreviewInvariantManifest::all_true(),
        summary,
        page_clean,
        published_report_ref: VOICE_PREVIEW_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: VOICE_PREVIEW_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            VOICE_PREVIEW_PUBLISHED_DOC_REF.to_owned(),
            VOICE_AND_DICTATION_CONTRACT_REF.to_owned(),
        ],
        support_export_refs: vec!["support:voice-preview:beta".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_voice_preview_beta_page`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum VoicePreviewValidationError {
    /// The page has no rows.
    NoRows,
    /// No claimed beta/preview row is present.
    NoClaimedRow,
    /// The invariant manifest is not all-true.
    InvariantManifestNotAllTrue,
    /// A blocking finding remains on a row.
    BlockingFindingPresent { row_id: String, class: String },
    /// A row's docs/help anchor is empty.
    MissingDocsHelpAnchor { row_id: String },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a voice-preview page against the M3 acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_voice_preview_beta_page(
    page: &VoicePreviewBetaPage,
) -> Result<(), Vec<VoicePreviewValidationError>> {
    let mut errors = Vec::new();

    if page.rows.is_empty() {
        errors.push(VoicePreviewValidationError::NoRows);
    }

    if !page.rows.iter().any(|row| row.claim_posture.is_claimed()) {
        errors.push(VoicePreviewValidationError::NoClaimedRow);
    }

    if page.invariants != VoicePreviewInvariantManifest::all_true() {
        errors.push(VoicePreviewValidationError::InvariantManifestNotAllTrue);
    }

    for row in &page.rows {
        if row.docs_help_anchor_ref.trim().is_empty() {
            errors.push(VoicePreviewValidationError::MissingDocsHelpAnchor {
                row_id: row.row_id.clone(),
            });
        }
        for finding in &row.blocking_findings {
            errors.push(VoicePreviewValidationError::BlockingFindingPresent {
                row_id: finding.row_id().to_owned(),
                class: finding.class_token().to_owned(),
            });
        }
    }

    if page.published_report_ref.trim().is_empty() {
        errors.push(VoicePreviewValidationError::PublishedReportRefMissing);
    }
    if page.published_doc_ref.trim().is_empty() {
        errors.push(VoicePreviewValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

mod seed;
pub use seed::seeded_voice_preview_beta_page;

pub mod conformance;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_is_clean() {
        let page = seeded_voice_preview_beta_page();
        validate_voice_preview_beta_page(&page).expect("seeded page must validate");
        assert!(page.page_clean);
        assert_eq!(page.summary.total_blocking_findings, 0);
    }

    #[test]
    fn seed_proves_both_command_and_dictation_modes() {
        let page = seeded_voice_preview_beta_page();
        assert!(page
            .summary
            .voice_modes_present
            .iter()
            .any(|mode| mode == "command_mode_active"));
        assert!(page
            .summary
            .voice_modes_present
            .iter()
            .any(|mode| mode == "dictation_mode_active"));
    }

    #[test]
    fn seed_covers_claimed_and_labs_postures() {
        let page = seeded_voice_preview_beta_page();
        assert!(page.summary.claimed_row_count >= 1);
        assert!(page.summary.labs_row_count >= 1);
        assert!(page
            .summary
            .claim_postures_present
            .iter()
            .any(|posture| posture == "labs_unadvertised"));
    }

    #[test]
    fn high_impact_resolutions_keep_preview_and_strict_guards() {
        let page = seeded_voice_preview_beta_page();
        let mut saw_high_impact = false;
        for row in &page.rows {
            for resolution in &row.command_resolutions {
                if resolution.is_high_impact() {
                    saw_high_impact = true;
                    assert!(resolution.preview_required, "{}", resolution.resolution_id);
                    assert_eq!(resolution.no_bypass_guards, NoBypassGuards::strict());
                    assert!(resolution.canonical_command_id.is_some());
                }
            }
        }
        assert!(
            saw_high_impact,
            "seed must exercise a high-impact resolution"
        );
    }

    #[test]
    fn unavailable_rows_offer_keyboard_fallback() {
        let page = seeded_voice_preview_beta_page();
        let mut saw_unavailable = false;
        for row in &page.rows {
            if row.provider_privacy_row.unavailable_reason.is_some() {
                saw_unavailable = true;
                assert!(row.provider_privacy_row.keyboard_fallback_available);
                assert!(!row
                    .provider_privacy_row
                    .keyboard_fallback_command_id
                    .trim()
                    .is_empty());
            }
        }
        assert!(saw_unavailable, "seed must exercise an unavailable row");
    }

    #[test]
    fn injected_high_impact_bypass_is_caught() {
        let mut page = seeded_voice_preview_beta_page();
        // Find a high-impact resolution and strip its preview requirement.
        let row = page
            .rows
            .iter_mut()
            .find(|row| row.command_resolutions.iter().any(|r| r.is_high_impact()))
            .expect("seed must have a high-impact resolution");
        let resolution = row
            .command_resolutions
            .iter_mut()
            .find(|r| r.is_high_impact())
            .expect("resolution");
        resolution.preview_required = false;
        let rebuilt = build_voice_preview_row(row.clone());
        assert!(rebuilt.blocking_findings.iter().any(|finding| matches!(
            finding,
            VoiceBlockingFinding::HighImpactPreviewBypassed { .. }
        )));
    }

    #[test]
    fn injected_labs_advertisement_is_caught() {
        let mut page = seeded_voice_preview_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|row| !row.claim_posture.is_claimed())
            .expect("seed must have a labs row");
        // A Labs row that starts advertising spoken commands must be caught.
        row.background_listening_state = BackgroundListeningState::OnUserOptedIn;
        row.default_activation_class = VoiceActivationClass::WakePhraseContinuousUserOptedIn;
        let rebuilt = build_voice_preview_row(row.clone());
        assert!(rebuilt.blocking_findings.iter().any(|finding| matches!(
            finding,
            VoiceBlockingFinding::LabsRowAdvertisesBroadSupport { .. }
        )));
    }

    #[test]
    fn support_export_excludes_raw_bytes_and_quotes_case_ids() {
        let page = seeded_voice_preview_beta_page();
        let export =
            VoicePreviewSupportExport::from_page(VOICE_PREVIEW_SUPPORT_EXPORT_ID, page.clone());
        assert!(export.raw_audio_or_transcript_bytes_excluded);
        assert!(export.case_ids.contains(&page.page_id));
        for row in &page.rows {
            assert!(export.case_ids.contains(&row.row_id));
        }
    }
}
