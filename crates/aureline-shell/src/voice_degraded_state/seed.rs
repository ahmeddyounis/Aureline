//! Deterministic seed for the voice degraded-state surface.
//!
//! The seed is the single mint-from-truth source for the checked-in fixtures
//! under [`super::VOICE_DEGRADED_STATE_FIXTURES_DIR_REF`], the published help doc,
//! and the degraded-state matrix. Every id, ref, and label is stable so the
//! fixtures stay bit-for-bit equal across regenerations.

use super::{
    DegradedBanner, DegradedBannerSeverity, DegradedNarration, DegradedRecoveryPosture,
    KeyboardFirstFallback, NarrationPoliteness, RecoveryAction, RecoveryActionKind,
    VoiceClaimPosture, VoiceDegradedCause, VoiceDegradedFlow, VoiceDegradedStatePacket,
    VoicePolicyState, VoiceShellLifecycleState, REDACTION_CLASS, VOICE_DEGRADED_FLOW_RECORD_KIND,
    VOICE_DEGRADED_STATE_SCHEMA_VERSION, VOICE_DEGRADED_STATE_SHARED_CONTRACT_REF,
};

/// Canonical command id of the keyboard-first fallback (open command palette).
const KEYBOARD_FALLBACK_COMMAND_ID: &str = "cmd:command_palette.open";
/// Canonical command id to open microphone / input-device settings.
const OPEN_MIC_SETTINGS_COMMAND_ID: &str = "cmd:settings.open_microphone";
/// Canonical command id to switch capture to an on-device engine.
const SWITCH_TO_LOCAL_COMMAND_ID: &str = "cmd:voice.switch_to_on_device_engine";
/// Canonical command id to retry voice once the condition clears.
const RETRY_CAPTURE_COMMAND_ID: &str = "cmd:voice.retry_capture";
/// Canonical command id to confirm or correct a held result.
const CONFIRM_RESULT_COMMAND_ID: &str = "cmd:voice.confirm_result";
/// Canonical command id to install or switch the speech language pack.
const MANAGE_LANGUAGE_PACKS_COMMAND_ID: &str = "cmd:voice.manage_language_packs";
/// Canonical command id to open the voice policy detail surface.
const OPEN_POLICY_DETAILS_COMMAND_ID: &str = "cmd:policy.open_voice_details";

/// Focus target returned to after a command-mode fallback.
const COMMAND_FOCUS_TARGET: &str = "target:command_overlay.invoker";
/// Focus target returned to after a dictation-mode fallback.
const DICTATION_FOCUS_TARGET: &str = "target:editor.active_insertion_point";

fn action(
    action_id: &str,
    kind: RecoveryActionKind,
    command_id: &str,
    label_ref: &str,
) -> RecoveryAction {
    RecoveryAction {
        action_id: action_id.to_owned(),
        kind,
        command_id: command_id.to_owned(),
        label_ref: label_ref.to_owned(),
        keyboard_accessible: true,
    }
}

fn keyboard_fallback(focus_return_target_ref: &str) -> KeyboardFirstFallback {
    KeyboardFirstFallback {
        keyboard_fallback_command_id: KEYBOARD_FALLBACK_COMMAND_ID.to_owned(),
        preserves_focus_and_work: true,
        focus_return_target_ref: focus_return_target_ref.to_owned(),
        preserves_uncommitted_work: true,
        fallback_hint_label_ref: "label:voice:fallback_hint_keyboard".to_owned(),
    }
}

fn banner(
    banner_id: &str,
    severity: DegradedBannerSeverity,
    title_label_ref: &str,
    cause_detail_label_ref: &str,
    consequence_label_ref: &str,
) -> DegradedBanner {
    DegradedBanner {
        banner_id: banner_id.to_owned(),
        severity,
        durable: true,
        title_label_ref: title_label_ref.to_owned(),
        cause_detail_label_ref: cause_detail_label_ref.to_owned(),
        consequence_label_ref: consequence_label_ref.to_owned(),
        names_specific_cause: true,
        placement_class: "banner_workspace_voice".to_owned(),
    }
}

fn narration(announcement_label_ref: &str, politeness: NarrationPoliteness) -> DegradedNarration {
    DegradedNarration {
        announcement_label_ref: announcement_label_ref.to_owned(),
        politeness,
        announced_once_per_transition: true,
        names_cause_and_recovery: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn flow(
    flow_id: &str,
    surface_label_ref: &str,
    claim_posture: VoiceClaimPosture,
    cause: VoiceDegradedCause,
    lifecycle_state: VoiceShellLifecycleState,
    policy_state: VoicePolicyState,
    recovery_posture: DegradedRecoveryPosture,
    banner: DegradedBanner,
    recovery_actions: Vec<RecoveryAction>,
    keyboard_fallback: KeyboardFirstFallback,
    narration: DegradedNarration,
) -> VoiceDegradedFlow {
    VoiceDegradedFlow {
        record_kind: VOICE_DEGRADED_FLOW_RECORD_KIND.to_owned(),
        schema_version: VOICE_DEGRADED_STATE_SCHEMA_VERSION,
        shared_contract_ref: VOICE_DEGRADED_STATE_SHARED_CONTRACT_REF.to_owned(),
        flow_id: flow_id.to_owned(),
        surface_label_ref: surface_label_ref.to_owned(),
        claim_posture,
        cause,
        canonical_unavailable_reason: cause.canonical_unavailable_reason(),
        lifecycle_state,
        policy_state,
        recovery_posture,
        banner,
        recovery_actions,
        keyboard_fallback,
        narration,
        enters_controlled_state: true,
        preserves_nonvoice_recovery: true,
        screen_reader_announces_state: true,
        keyboard_reachable: true,
        redaction_class: REDACTION_CLASS.to_owned(),
    }
}

/// No microphone hardware: voice is unavailable; the banner points the user at
/// input settings and the keyboard / command palette keep working.
fn missing_microphone_flow() -> VoiceDegradedFlow {
    flow(
        "voice-degraded:missing-microphone-hardware",
        "label:voice:command_overlay",
        VoiceClaimPosture::ClaimedBeta,
        VoiceDegradedCause::MissingMicrophoneHardware,
        VoiceShellLifecycleState::Unavailable,
        VoicePolicyState::UserControlled,
        DegradedRecoveryPosture::FellBackToKeyboardFirst,
        banner(
            "voice-degraded:banner:no-microphone",
            DegradedBannerSeverity::Warning,
            "label:voice:banner_title_no_microphone",
            "label:voice:cause_no_microphone_detail",
            "label:voice:consequence_capture_unavailable",
        ),
        vec![
            action(
                "voice-degraded:action:open-mic-settings",
                RecoveryActionKind::OpenMicrophoneSettings,
                OPEN_MIC_SETTINGS_COMMAND_ID,
                "label:voice:action_open_mic_settings",
            ),
            action(
                "voice-degraded:action:keyboard-fallback:no-microphone",
                RecoveryActionKind::KeyboardFallback,
                KEYBOARD_FALLBACK_COMMAND_ID,
                "label:voice:action_keyboard_fallback",
            ),
        ],
        keyboard_fallback(COMMAND_FOCUS_TARGET),
        narration(
            "a11y:voice:degraded_no_microphone",
            NarrationPoliteness::Polite,
        ),
    )
}

/// Noisy environment: voice stays usable but every recognized result is held for
/// explicit confirmation; the user can confirm, retry quieter, or type.
fn noisy_environment_flow() -> VoiceDegradedFlow {
    flow(
        "voice-degraded:noisy-environment",
        "label:voice:dictation_input",
        VoiceClaimPosture::ClaimedPreview,
        VoiceDegradedCause::NoisyEnvironment,
        VoiceShellLifecycleState::NeedsConfirmation,
        VoicePolicyState::UserControlled,
        DegradedRecoveryPosture::HeldForConfirmation,
        banner(
            "voice-degraded:banner:noisy-environment",
            DegradedBannerSeverity::Informational,
            "label:voice:banner_title_noisy_environment",
            "label:voice:cause_noisy_environment_detail",
            "label:voice:consequence_results_held_for_confirmation",
        ),
        vec![
            action(
                "voice-degraded:action:confirm-held-result",
                RecoveryActionKind::ConfirmHeldResult,
                CONFIRM_RESULT_COMMAND_ID,
                "label:voice:action_confirm_held_result",
            ),
            action(
                "voice-degraded:action:retry-quieter",
                RecoveryActionKind::RetryWhenConditionClears,
                RETRY_CAPTURE_COMMAND_ID,
                "label:voice:action_retry_when_quieter",
            ),
            action(
                "voice-degraded:action:keyboard-fallback:noisy-environment",
                RecoveryActionKind::KeyboardFallback,
                KEYBOARD_FALLBACK_COMMAND_ID,
                "label:voice:action_keyboard_fallback",
            ),
        ],
        keyboard_fallback(DICTATION_FOCUS_TARGET),
        narration(
            "a11y:voice:degraded_noisy_environment",
            NarrationPoliteness::Polite,
        ),
    )
}

/// Provider offline: the hosted engine is unreachable; offer the on-device
/// engine, a retry, and the keyboard fallback.
fn provider_offline_flow() -> VoiceDegradedFlow {
    flow(
        "voice-degraded:provider-offline",
        "label:voice:dictation_input",
        VoiceClaimPosture::ClaimedPreview,
        VoiceDegradedCause::ProviderOffline,
        VoiceShellLifecycleState::Unavailable,
        VoicePolicyState::UserControlled,
        DegradedRecoveryPosture::OfferedOnDeviceEngineFallback,
        banner(
            "voice-degraded:banner:provider-offline",
            DegradedBannerSeverity::Warning,
            "label:voice:banner_title_provider_offline",
            "label:voice:cause_provider_offline_detail",
            "label:voice:consequence_hosted_recognition_unavailable",
        ),
        vec![
            action(
                "voice-degraded:action:switch-to-on-device",
                RecoveryActionKind::SwitchToOnDeviceEngine,
                SWITCH_TO_LOCAL_COMMAND_ID,
                "label:voice:action_switch_to_on_device",
            ),
            action(
                "voice-degraded:action:retry-provider",
                RecoveryActionKind::RetryWhenConditionClears,
                RETRY_CAPTURE_COMMAND_ID,
                "label:voice:action_retry_provider",
            ),
            action(
                "voice-degraded:action:keyboard-fallback:provider-offline",
                RecoveryActionKind::KeyboardFallback,
                KEYBOARD_FALLBACK_COMMAND_ID,
                "label:voice:action_keyboard_fallback",
            ),
        ],
        keyboard_fallback(DICTATION_FOCUS_TARGET),
        narration(
            "a11y:voice:degraded_provider_offline",
            NarrationPoliteness::Polite,
        ),
    )
}

/// Language pack missing: recognition for the requested language is unavailable;
/// offer to install / switch the pack, and keep the keyboard fallback.
fn language_pack_missing_flow() -> VoiceDegradedFlow {
    flow(
        "voice-degraded:language-pack-missing",
        "label:voice:dictation_input",
        VoiceClaimPosture::ClaimedBeta,
        VoiceDegradedCause::LanguagePackMissing,
        VoiceShellLifecycleState::Unavailable,
        VoicePolicyState::UserControlled,
        DegradedRecoveryPosture::HeldUntilConditionClears,
        banner(
            "voice-degraded:banner:language-pack-missing",
            DegradedBannerSeverity::Warning,
            "label:voice:banner_title_language_pack_missing",
            "label:voice:cause_language_pack_missing_detail",
            "label:voice:consequence_language_recognition_unavailable",
        ),
        vec![
            action(
                "voice-degraded:action:manage-language-packs",
                RecoveryActionKind::InstallOrSwitchLanguagePack,
                MANAGE_LANGUAGE_PACKS_COMMAND_ID,
                "label:voice:action_manage_language_packs",
            ),
            action(
                "voice-degraded:action:keyboard-fallback:language-pack-missing",
                RecoveryActionKind::KeyboardFallback,
                KEYBOARD_FALLBACK_COMMAND_ID,
                "label:voice:action_keyboard_fallback",
            ),
        ],
        keyboard_fallback(DICTATION_FOCUS_TARGET),
        narration(
            "a11y:voice:degraded_language_pack_missing",
            NarrationPoliteness::Polite,
        ),
    )
}

/// Policy blocked: voice is disabled by managed policy; offer the policy detail
/// surface and the keyboard fallback, narrated assertively as a hard stop.
fn policy_blocked_flow() -> VoiceDegradedFlow {
    flow(
        "voice-degraded:policy-blocked",
        "label:voice:command_overlay",
        VoiceClaimPosture::ClaimedBeta,
        VoiceDegradedCause::PolicyBlocked,
        VoiceShellLifecycleState::PolicyBlocked,
        VoicePolicyState::PolicyBlocked,
        DegradedRecoveryPosture::FellBackToKeyboardFirst,
        banner(
            "voice-degraded:banner:policy-blocked",
            DegradedBannerSeverity::Blocked,
            "label:voice:banner_title_policy_blocked",
            "label:voice:cause_policy_blocked_detail",
            "label:voice:consequence_voice_disabled_by_policy",
        ),
        vec![
            action(
                "voice-degraded:action:open-policy-details",
                RecoveryActionKind::OpenPolicyDetails,
                OPEN_POLICY_DETAILS_COMMAND_ID,
                "label:voice:action_open_policy_details",
            ),
            action(
                "voice-degraded:action:keyboard-fallback:policy-blocked",
                RecoveryActionKind::KeyboardFallback,
                KEYBOARD_FALLBACK_COMMAND_ID,
                "label:voice:action_keyboard_fallback",
            ),
        ],
        keyboard_fallback(COMMAND_FOCUS_TARGET),
        narration(
            "a11y:voice:degraded_policy_blocked",
            NarrationPoliteness::Assertive,
        ),
    )
}

/// The canonical, fully qualified voice degraded-state packet. Covers every
/// major voice failure class in canonical order.
pub fn seeded_voice_degraded_state_packet() -> VoiceDegradedStatePacket {
    VoiceDegradedStatePacket::new(vec![
        missing_microphone_flow(),
        noisy_environment_flow(),
        provider_offline_flow(),
        language_pack_missing_flow(),
        policy_blocked_flow(),
    ])
}
