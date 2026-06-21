//! Deterministic seed for the voice shell-state surface.
//!
//! The seed is the single mint-from-truth source for the checked-in fixtures
//! under [`super::VOICE_SHELL_STATE_FIXTURES_DIR_REF`] and the published
//! companion doc. Every id, ref, and label is stable so the fixtures stay
//! bit-for-bit equal across regenerations.

use super::{
    BackgroundListeningState, MicIndicatorClass, ProcessingLocalityCue, ProviderLocalityDisclosure,
    PushToTalkControl, RetentionMode, VoiceActivationClass, VoiceClaimPosture, VoiceMicIndicator,
    VoiceModeClass, VoiceModeStrip, VoicePolicyState, VoiceRecoveryAffordance,
    VoiceShellLifecycleState, VoiceShellStatePacket, VoiceShellStateRow, VoiceUnavailableReason,
    REDACTION_CLASS, VOICE_SHELL_STATE_ROW_RECORD_KIND, VOICE_SHELL_STATE_SCHEMA_VERSION,
    VOICE_SHELL_STATE_SHARED_CONTRACT_REF,
};

/// Canonical command id that toggles between command and dictation mode.
const MODE_TOGGLE_COMMAND_ID: &str = "cmd:voice.toggle_mode";
/// Canonical command id for press-and-hold activation.
const HOLD_COMMAND_ID: &str = "cmd:voice.push_to_talk_hold";
/// Canonical command id for toggle activation.
const TOGGLE_COMMAND_ID: &str = "cmd:voice.push_to_talk_toggle";
/// Canonical command id for the mute action.
const MUTE_COMMAND_ID: &str = "cmd:voice.mute_microphone";
/// Canonical command id for the stop action.
const STOP_COMMAND_ID: &str = "cmd:voice.stop_capture";
/// Canonical command id of the keyboard-first fallback (open command palette).
const KEYBOARD_FALLBACK_COMMAND_ID: &str = "cmd:command_palette.open";

fn mode_strip(active_mode: VoiceModeClass) -> VoiceModeStrip {
    VoiceModeStrip {
        active_mode,
        command_mode_label_ref: "label:voice:mode_command".to_owned(),
        dictation_mode_label_ref: "label:voice:mode_dictation".to_owned(),
        mode_toggle_command_id: MODE_TOGGLE_COMMAND_ID.to_owned(),
        both_modes_visible: true,
        keyboard_reachable: true,
    }
}

fn activation(
    default_activation_class: VoiceActivationClass,
    background_listening_state: BackgroundListeningState,
) -> PushToTalkControl {
    PushToTalkControl {
        default_activation_class,
        hold_command_id: HOLD_COMMAND_ID.to_owned(),
        toggle_command_id: TOGGLE_COMMAND_ID.to_owned(),
        continuous_requires_opt_in: true,
        background_listening_state,
        activation_hint_label_ref: "label:voice:activation_push_to_talk".to_owned(),
        keyboard_reachable: true,
    }
}

fn mic_indicator(
    indicator_class: MicIndicatorClass,
    capture_active: bool,
    active_mode: VoiceModeClass,
    processing_locality: ProcessingLocalityCue,
    accessibility_label_ref: &str,
) -> VoiceMicIndicator {
    VoiceMicIndicator {
        indicator_class,
        capture_active,
        active_mode,
        processing_locality,
        mute_command_id: MUTE_COMMAND_ID.to_owned(),
        stop_command_id: STOP_COMMAND_ID.to_owned(),
        accessibility_label_ref: accessibility_label_ref.to_owned(),
        keyboard_reachable: true,
        layout_target_class: "status_zone_voice_pill".to_owned(),
    }
}

fn provider_locality(
    provider_label: &str,
    processing_locality: ProcessingLocalityCue,
    retention_mode: RetentionMode,
    disclosure_label_ref: &str,
) -> ProviderLocalityDisclosure {
    ProviderLocalityDisclosure {
        provider_or_local_engine_label_ref: provider_label.to_owned(),
        processing_locality,
        retention_mode,
        visible_without_settings_dive: true,
        disclosure_label_ref: disclosure_label_ref.to_owned(),
    }
}

fn recovery(
    recovery_label_ref: &str,
    unavailable_reason: Option<VoiceUnavailableReason>,
    policy_block_note_ref: Option<&str>,
) -> VoiceRecoveryAffordance {
    VoiceRecoveryAffordance {
        keyboard_first_recovery_immediate: true,
        keyboard_fallback_command_id: KEYBOARD_FALLBACK_COMMAND_ID.to_owned(),
        recovery_label_ref: recovery_label_ref.to_owned(),
        unavailable_reason,
        policy_block_note_ref: policy_block_note_ref.map(str::to_owned),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    surface_label_ref: &str,
    claim_posture: VoiceClaimPosture,
    lifecycle_state: VoiceShellLifecycleState,
    mode_strip: VoiceModeStrip,
    activation: PushToTalkControl,
    mic_indicator: VoiceMicIndicator,
    provider_locality: ProviderLocalityDisclosure,
    recovery: VoiceRecoveryAffordance,
    policy_state: VoicePolicyState,
) -> VoiceShellStateRow {
    VoiceShellStateRow {
        record_kind: VOICE_SHELL_STATE_ROW_RECORD_KIND.to_owned(),
        schema_version: VOICE_SHELL_STATE_SCHEMA_VERSION,
        shared_contract_ref: VOICE_SHELL_STATE_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        surface_label_ref: surface_label_ref.to_owned(),
        claim_posture,
        lifecycle_state,
        mode_strip,
        activation,
        mic_indicator,
        provider_locality,
        recovery,
        policy_state,
        screen_reader_announces_state: true,
        keyboard_reachable: true,
        redaction_class: REDACTION_CLASS.to_owned(),
    }
}

/// Idle: mic off, push-to-talk default, local processing, no retention.
fn idle_row() -> VoiceShellStateRow {
    row(
        "voice-shell:command-overlay:local:idle",
        "label:voice:command_overlay",
        VoiceClaimPosture::ClaimedBeta,
        VoiceShellLifecycleState::Idle,
        mode_strip(VoiceModeClass::IdleMicrophoneOff),
        activation(
            VoiceActivationClass::PushToTalkHeld,
            BackgroundListeningState::OffDefault,
        ),
        mic_indicator(
            MicIndicatorClass::PersistentIndicatorVisibleCaptureIdle,
            false,
            VoiceModeClass::IdleMicrophoneOff,
            ProcessingLocalityCue::LocalOnDevice,
            "a11y:voice:idle_local",
        ),
        provider_locality(
            "label:voice:local_engine",
            ProcessingLocalityCue::LocalOnDevice,
            RetentionMode::NoAudioNoTranscriptRetained,
            "label:voice:disclosure_local_no_retention",
        ),
        recovery("label:voice:keyboard_fallback", None, None),
        VoicePolicyState::UserControlled,
    )
}

/// Listening: command mode capturing on-device; mic indicator active.
fn listening_row() -> VoiceShellStateRow {
    row(
        "voice-shell:command-overlay:local:listening",
        "label:voice:command_overlay",
        VoiceClaimPosture::ClaimedBeta,
        VoiceShellLifecycleState::Listening,
        mode_strip(VoiceModeClass::CommandModeActive),
        activation(
            VoiceActivationClass::PushToTalkHeld,
            BackgroundListeningState::OffDefault,
        ),
        mic_indicator(
            MicIndicatorClass::PersistentIndicatorVisibleCaptureActive,
            true,
            VoiceModeClass::CommandModeActive,
            ProcessingLocalityCue::LocalOnDevice,
            "a11y:voice:listening_command_local",
        ),
        provider_locality(
            "label:voice:local_engine",
            ProcessingLocalityCue::LocalOnDevice,
            RetentionMode::EphemeralAudioLocalOnlyNoTranscriptRetained,
            "label:voice:disclosure_local_ephemeral",
        ),
        recovery("label:voice:keyboard_fallback", None, None),
        VoicePolicyState::UserControlled,
    )
}

/// Processing: dictation mode recognizing on a disclosed hosted engine.
fn processing_row() -> VoiceShellStateRow {
    row(
        "voice-shell:dictation-input:hosted:processing",
        "label:voice:dictation_input",
        VoiceClaimPosture::ClaimedPreview,
        VoiceShellLifecycleState::Processing,
        mode_strip(VoiceModeClass::DictationModeActive),
        activation(
            VoiceActivationClass::PushToTalkToggle,
            BackgroundListeningState::OffDefault,
        ),
        mic_indicator(
            MicIndicatorClass::PersistentIndicatorVisibleCaptureActive,
            true,
            VoiceModeClass::DictationModeActive,
            ProcessingLocalityCue::HostedRemoteDisclosed,
            "a11y:voice:processing_dictation_hosted",
        ),
        provider_locality(
            "label:voice:hosted_provider",
            ProcessingLocalityCue::HostedRemoteDisclosed,
            RetentionMode::TranscriptRetainedRedactedInSupportBundle,
            "label:voice:disclosure_hosted_redacted",
        ),
        recovery("label:voice:keyboard_fallback", None, None),
        VoicePolicyState::UserControlled,
    )
}

/// Needs confirmation: a resolved high-impact command awaits explicit confirm.
fn needs_confirmation_row() -> VoiceShellStateRow {
    row(
        "voice-shell:command-overlay:local:needs-confirmation",
        "label:voice:command_overlay",
        VoiceClaimPosture::ClaimedBeta,
        VoiceShellLifecycleState::NeedsConfirmation,
        mode_strip(VoiceModeClass::CommandModeActive),
        activation(
            VoiceActivationClass::PushToTalkHeld,
            BackgroundListeningState::OffDefault,
        ),
        mic_indicator(
            MicIndicatorClass::PersistentIndicatorVisibleCaptureIdle,
            false,
            VoiceModeClass::CommandModeActive,
            ProcessingLocalityCue::LocalOnDevice,
            "a11y:voice:needs_confirmation_command_local",
        ),
        provider_locality(
            "label:voice:local_engine",
            ProcessingLocalityCue::LocalOnDevice,
            RetentionMode::TranscriptRetainedLocalOnly,
            "label:voice:disclosure_local_transcript",
        ),
        recovery("label:voice:keyboard_confirm_or_cancel", None, None),
        VoicePolicyState::UserControlled,
    )
}

/// Unavailable: no microphone present; keyboard-first recovery offered.
fn unavailable_row() -> VoiceShellStateRow {
    row(
        "voice-shell:command-overlay:local:unavailable",
        "label:voice:command_overlay",
        VoiceClaimPosture::ClaimedBeta,
        VoiceShellLifecycleState::Unavailable,
        mode_strip(VoiceModeClass::IdleMicrophoneOff),
        activation(
            VoiceActivationClass::PushToTalkHeld,
            BackgroundListeningState::OffDefault,
        ),
        mic_indicator(
            MicIndicatorClass::PersistentIndicatorUnavailableDegraded,
            false,
            VoiceModeClass::IdleMicrophoneOff,
            ProcessingLocalityCue::ProcessingUnavailable,
            "a11y:voice:unavailable_no_microphone",
        ),
        provider_locality(
            "label:voice:local_engine",
            ProcessingLocalityCue::ProcessingUnavailable,
            RetentionMode::RetentionUnavailableInEnvelope,
            "label:voice:disclosure_unavailable",
        ),
        recovery(
            "label:voice:keyboard_recovery_no_microphone",
            Some(VoiceUnavailableReason::NoMicrophone),
            None,
        ),
        VoicePolicyState::UserControlled,
    )
}

/// Policy blocked: voice disabled by managed policy; keyboard-first recovery.
fn policy_blocked_row() -> VoiceShellStateRow {
    row(
        "voice-shell:command-overlay:managed:policy-blocked",
        "label:voice:command_overlay",
        VoiceClaimPosture::ClaimedBeta,
        VoiceShellLifecycleState::PolicyBlocked,
        mode_strip(VoiceModeClass::VoiceModeBlockedByPolicy),
        activation(
            VoiceActivationClass::PushToTalkHeld,
            BackgroundListeningState::BlockedByPolicy,
        ),
        mic_indicator(
            MicIndicatorClass::PersistentIndicatorHiddenCaptureDisabled,
            false,
            VoiceModeClass::VoiceModeBlockedByPolicy,
            ProcessingLocalityCue::ProcessingUnavailable,
            "a11y:voice:policy_blocked",
        ),
        provider_locality(
            "label:voice:managed_policy",
            ProcessingLocalityCue::ProcessingUnavailable,
            RetentionMode::RetentionBlockedByPolicy,
            "label:voice:disclosure_policy_blocked",
        ),
        recovery(
            "label:voice:keyboard_recovery_policy_blocked",
            None,
            Some("label:voice:policy_block_note"),
        ),
        VoicePolicyState::PolicyBlocked,
    )
}

/// The canonical, fully qualified voice shell-state packet. Covers every
/// lifecycle state across local and hosted processing localities.
pub fn seeded_voice_shell_state_packet() -> VoiceShellStatePacket {
    VoiceShellStatePacket::new(vec![
        idle_row(),
        listening_row(),
        processing_row(),
        needs_confirmation_row(),
        unavailable_row(),
        policy_blocked_row(),
    ])
}
