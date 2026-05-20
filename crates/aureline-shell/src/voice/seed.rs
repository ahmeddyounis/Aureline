//! Deterministic seed for the bounded voice-preview surface.
//!
//! The seed is the single mint-from-truth source for the checked-in
//! fixtures, the published markdown artifact, and the support-export
//! wrapper. Every id, ref, and label is stable so the fixtures stay
//! bit-for-bit equal across regenerations.

use super::{
    build_voice_preview_beta_page, BackgroundListeningState, CommandDisambiguationCandidate,
    CommandDisambiguationSheet, ConfidenceCue, DisabledReasonCode, EnablementDecisionClass,
    MicIndicatorClass, MicStatePill, NoBypassGuards, ProcessingLocalityCue, ProviderPrivacyRow,
    RetentionMode, TranscriptCorrectionPosture, TranscriptStrip, VoiceCapabilityScope,
    VoiceClaimPosture, VoiceCommandResolution, VoiceCommandResolutionClass, VoiceLifecycleLabel,
    VoiceModeClass, VoicePreviewBetaPage, VoicePreviewRow, VoiceUnavailableBanner,
    VoiceUnavailableReason, VoiceActivationClass, COMMAND_RESULT_PACKET_SCHEMA_REF,
    REDACTION_CLASS, VOICE_PREVIEW_ROW_RECORD_KIND, VOICE_PREVIEW_SCHEMA_VERSION,
    VOICE_PREVIEW_SHARED_CONTRACT_REF,
};

fn mic_pill(
    id: &str,
    capture_active: bool,
    voice_mode_class: VoiceModeClass,
    mic_indicator_class: MicIndicatorClass,
    processing_locality_cue: ProcessingLocalityCue,
    layout_target_class: &str,
    label: &str,
) -> MicStatePill {
    MicStatePill {
        record_kind: "shell_voice_mic_state_pill_record".to_owned(),
        schema_version: VOICE_PREVIEW_SCHEMA_VERSION,
        shared_contract_ref: VOICE_PREVIEW_SHARED_CONTRACT_REF.to_owned(),
        pill_id: id.to_owned(),
        capture_active,
        voice_mode_class,
        mic_indicator_class,
        processing_locality_cue,
        mute_action_command_id: "cmd:voice.mute_microphone".to_owned(),
        stop_action_command_id: "cmd:voice.stop_capture".to_owned(),
        accessibility_label_ref: label.to_owned(),
        keyboard_reachable: true,
        layout_target_class: layout_target_class.to_owned(),
        redaction_class: REDACTION_CLASS.to_owned(),
    }
}

fn transcript_strip(
    id: &str,
    confidence_cue: ConfidenceCue,
    posture: TranscriptCorrectionPosture,
    label: &str,
) -> TranscriptStrip {
    TranscriptStrip {
        record_kind: "shell_voice_transcript_strip_record".to_owned(),
        schema_version: VOICE_PREVIEW_SCHEMA_VERSION,
        shared_contract_ref: VOICE_PREVIEW_SHARED_CONTRACT_REF.to_owned(),
        strip_id: id.to_owned(),
        transcript_text_label_ref: label.to_owned(),
        confidence_cue,
        transcript_correction_posture: posture,
        edit_command_id: "cmd:voice.edit_transcript".to_owned(),
        correct_command_id: "cmd:voice.correct_transcript".to_owned(),
        confirm_command_id: "cmd:voice.confirm_transcript".to_owned(),
        cancel_command_id: "cmd:voice.cancel_transcript".to_owned(),
        accessibility_label_ref: "a11y:voice:transcript_strip".to_owned(),
        redaction_class: REDACTION_CLASS.to_owned(),
    }
}

fn provider_privacy_row(
    id: &str,
    provider_label: &str,
    processing_locality_cue: ProcessingLocalityCue,
    retention_mode: RetentionMode,
    background_listening_state: BackgroundListeningState,
    policy_note: Option<&str>,
    unavailable_reason: Option<VoiceUnavailableReason>,
    keyboard_fallback_command_id: &str,
) -> ProviderPrivacyRow {
    ProviderPrivacyRow {
        record_kind: "shell_voice_provider_privacy_row_record".to_owned(),
        schema_version: VOICE_PREVIEW_SCHEMA_VERSION,
        shared_contract_ref: VOICE_PREVIEW_SHARED_CONTRACT_REF.to_owned(),
        row_id: id.to_owned(),
        provider_or_local_engine_label_ref: provider_label.to_owned(),
        processing_locality_cue,
        retention_mode,
        background_listening_state,
        policy_lock_or_block_note_ref: policy_note.map(str::to_owned),
        unavailable_reason,
        keyboard_fallback_available: true,
        keyboard_fallback_command_id: keyboard_fallback_command_id.to_owned(),
        accessibility_label_ref: "a11y:voice:provider_privacy_row".to_owned(),
        redaction_class: REDACTION_CLASS.to_owned(),
    }
}

fn resolution(
    id: &str,
    phrase_label: &str,
    confidence_cue: ConfidenceCue,
    resolution_class: VoiceCommandResolutionClass,
    canonical_command_id: Option<&str>,
    canonical_verb: Option<&str>,
    lifecycle_label: Option<VoiceLifecycleLabel>,
    capability_scope_class: VoiceCapabilityScope,
    preview_class_declared: &str,
    approval_posture_class_declared: &str,
    enablement_decision_class: EnablementDecisionClass,
    disabled_reason_code: Option<DisabledReasonCode>,
    preview_required: bool,
    approval_required: bool,
) -> VoiceCommandResolution {
    VoiceCommandResolution {
        record_kind: "shell_voice_command_resolution_record".to_owned(),
        schema_version: VOICE_PREVIEW_SCHEMA_VERSION,
        shared_contract_ref: VOICE_PREVIEW_SHARED_CONTRACT_REF.to_owned(),
        resolution_id: id.to_owned(),
        spoken_phrase_label_ref: phrase_label.to_owned(),
        confidence_cue,
        resolution_class,
        canonical_command_id: canonical_command_id.map(str::to_owned),
        command_revision_ref: canonical_command_id
            .map(|id| format!("cmd-rev:{}:2026.05.20-01", id.trim_start_matches("cmd:"))),
        canonical_verb: canonical_verb.map(str::to_owned),
        lifecycle_label,
        capability_scope_class,
        preview_class_declared: preview_class_declared.to_owned(),
        approval_posture_class_declared: approval_posture_class_declared.to_owned(),
        enablement_decision_class,
        disabled_reason_code,
        preview_required,
        approval_required,
        result_packet_schema_ref: COMMAND_RESULT_PACKET_SCHEMA_REF.to_owned(),
        parity_expectation_ref: "schemas/commands/parity_expectation.schema.json".to_owned(),
        no_bypass_guards: NoBypassGuards::strict(),
        docs_help_anchor_ref: "docs:anchor:voice:command_resolution_overview".to_owned(),
        keyboard_equivalent_command_id: canonical_command_id.map(str::to_owned),
        redaction_class: REDACTION_CLASS.to_owned(),
    }
}

fn base_row(
    row_id: &str,
    surface_label_ref: &str,
    claim_posture: VoiceClaimPosture,
    command_mode_explicit: bool,
    dictation_mode_explicit: bool,
    keyboard_reachable: bool,
    screen_reader_narratable: bool,
    default_activation_class: VoiceActivationClass,
    provider_privacy: ProviderPrivacyRow,
    docs_help_anchor_ref: &str,
) -> VoicePreviewRow {
    VoicePreviewRow {
        record_kind: VOICE_PREVIEW_ROW_RECORD_KIND.to_owned(),
        schema_version: VOICE_PREVIEW_SCHEMA_VERSION,
        shared_contract_ref: VOICE_PREVIEW_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        surface_label_ref: surface_label_ref.to_owned(),
        claim_posture,
        command_mode_explicit,
        dictation_mode_explicit,
        keyboard_reachable,
        screen_reader_narratable,
        default_activation_class,
        background_listening_state: BackgroundListeningState::OffDefault,
        mic_pill: None,
        transcript_strip: None,
        disambiguation_sheet: None,
        provider_privacy_row: provider_privacy,
        unavailable_banner: None,
        command_resolutions: Vec::new(),
        docs_help_anchor_ref: docs_help_anchor_ref.to_owned(),
        blocking_findings: Vec::new(),
    }
}

/// Builds the deterministic seeded voice-preview page.
pub fn seeded_voice_preview_beta_page() -> VoicePreviewBetaPage {
    // 1. Claimed beta command-mode row: local, push-to-talk, resolving a
    //    privileged refactor through the canonical command graph with a
    //    required preview and strict no-bypass guards.
    let mut command_row = base_row(
        "voice:row:command_mode_local_beta",
        "label:voice:command_mode_local",
        VoiceClaimPosture::ClaimedBeta,
        true,
        true,
        true,
        true,
        VoiceActivationClass::PushToTalkHeld,
        provider_privacy_row(
            "voice:privacy:command_mode_local_beta",
            "label:voice:provider:local_in_process_engine",
            ProcessingLocalityCue::LocalOnDevice,
            RetentionMode::NoAudioNoTranscriptRetained,
            BackgroundListeningState::OffDefault,
            None,
            None,
            "cmd:command_palette.open",
        ),
        "docs:anchor:voice:command_mode_overview",
    );
    command_row.mic_pill = Some(mic_pill(
        "voice:pill:command_mode_local_beta",
        true,
        VoiceModeClass::CommandModeActive,
        MicIndicatorClass::PersistentIndicatorVisibleCaptureActive,
        ProcessingLocalityCue::LocalOnDevice,
        "command_palette",
        "a11y:voice:mic_active_command_mode_local",
    ));
    command_row.transcript_strip = Some(transcript_strip(
        "voice:strip:command_mode_local_beta",
        ConfidenceCue::High,
        TranscriptCorrectionPosture::CorrectionRequiredBeforeCommit,
        "label:voice:transcript:rename_symbol_phrase",
    ));
    command_row.disambiguation_sheet = Some(CommandDisambiguationSheet {
        record_kind: "shell_voice_command_disambiguation_sheet_record".to_owned(),
        schema_version: VOICE_PREVIEW_SCHEMA_VERSION,
        shared_contract_ref: VOICE_PREVIEW_SHARED_CONTRACT_REF.to_owned(),
        sheet_id: "voice:sheet:command_mode_local_beta".to_owned(),
        candidates: vec![
            CommandDisambiguationCandidate {
                candidate_command_id: "cmd:edit.rename_symbol_across_project".to_owned(),
                candidate_label_ref: "label:edit.rename_symbol_across_project".to_owned(),
                confidence_cue: ConfidenceCue::High,
                capability_scope_class: VoiceCapabilityScope::RecoverableDurableMutation,
                preview_required: true,
            },
            CommandDisambiguationCandidate {
                candidate_command_id: "cmd:edit.rename_symbol_in_file".to_owned(),
                candidate_label_ref: "label:edit.rename_symbol_in_file".to_owned(),
                confidence_cue: ConfidenceCue::Medium,
                capability_scope_class: VoiceCapabilityScope::ReversibleLocalMutation,
                preview_required: false,
            },
        ],
        confirm_command_id: "cmd:voice.confirm_disambiguation".to_owned(),
        cancel_command_id: "cmd:voice.cancel_disambiguation".to_owned(),
        preview_state_for_risky: "preview_required_shown".to_owned(),
        accessibility_label_ref: "a11y:voice:disambiguation_sheet".to_owned(),
        redaction_class: REDACTION_CLASS.to_owned(),
    });
    command_row.command_resolutions = vec![
        resolution(
            "voice:resolution:rename_symbol_across_project",
            "label:voice:phrase:rename_symbol_across_project",
            ConfidenceCue::High,
            VoiceCommandResolutionClass::ResolvesToCanonicalCommandId,
            Some("cmd:edit.rename_symbol_across_project"),
            Some("edit.rename_symbol_across_project"),
            Some(VoiceLifecycleLabel::Beta),
            VoiceCapabilityScope::RecoverableDurableMutation,
            "structured_diff_preview",
            "no_approval_required",
            EnablementDecisionClass::Enabled,
            None,
            true,
            false,
        ),
        resolution(
            "voice:resolution:go_to_definition",
            "label:voice:phrase:go_to_definition",
            ConfidenceCue::High,
            VoiceCommandResolutionClass::ResolvesToCanonicalCommandId,
            Some("cmd:navigation.go_to_definition"),
            Some("navigation.go_to_definition"),
            Some(VoiceLifecycleLabel::Stable),
            VoiceCapabilityScope::ReversibleLocalRead,
            "no_preview_required",
            "no_approval_required",
            EnablementDecisionClass::Enabled,
            None,
            false,
            false,
        ),
    ];

    // 2. Claimed beta dictation-mode row: local, push-to-talk, text into the
    //    editor on the shared undo stack.
    let mut dictation_row = base_row(
        "voice:row:dictation_local_beta",
        "label:voice:dictation_local",
        VoiceClaimPosture::ClaimedBeta,
        true,
        true,
        true,
        true,
        VoiceActivationClass::PushToTalkHeld,
        provider_privacy_row(
            "voice:privacy:dictation_local_beta",
            "label:voice:provider:local_in_process_engine",
            ProcessingLocalityCue::LocalOnDevice,
            RetentionMode::EphemeralAudioLocalOnlyNoTranscriptRetained,
            BackgroundListeningState::OffDefault,
            None,
            None,
            "cmd:editor.focus",
        ),
        "docs:anchor:voice:dictation_overview",
    );
    dictation_row.mic_pill = Some(mic_pill(
        "voice:pill:dictation_local_beta",
        true,
        VoiceModeClass::DictationModeActive,
        MicIndicatorClass::PersistentIndicatorVisibleCaptureActive,
        ProcessingLocalityCue::LocalOnDevice,
        "primary_editor",
        "a11y:voice:mic_active_dictation_local",
    ));
    dictation_row.transcript_strip = Some(transcript_strip(
        "voice:strip:dictation_local_beta",
        ConfidenceCue::Medium,
        TranscriptCorrectionPosture::CorrectionOptionalBeforeCommit,
        "label:voice:transcript:dictated_segment",
    ));
    dictation_row.command_resolutions = vec![resolution(
        "voice:resolution:insert_dictated_text",
        "label:voice:phrase:dictated_segment",
        ConfidenceCue::Medium,
        VoiceCommandResolutionClass::ResolvesToDictationTextOnly,
        Some("cmd:editor.insert_dictated_text"),
        Some("editor.insert_dictated_text"),
        Some(VoiceLifecycleLabel::Beta),
        VoiceCapabilityScope::ReversibleLocalMutation,
        "no_preview_required",
        "no_approval_required",
        EnablementDecisionClass::Enabled,
        None,
        false,
        false,
    )];

    // 3. Claimed preview hosted-command row: a remote engine with disclosed
    //    handoff and explicit consent, resolving a high-impact publish that
    //    still rides the required preview and approval path.
    let mut hosted_row = base_row(
        "voice:row:hosted_command_preview",
        "label:voice:hosted_command_preview",
        VoiceClaimPosture::ClaimedPreview,
        true,
        true,
        true,
        true,
        VoiceActivationClass::ManualCommandActivation,
        provider_privacy_row(
            "voice:privacy:hosted_command_preview",
            "label:voice:provider:byok_remote_vendor",
            ProcessingLocalityCue::HostedRemoteDisclosed,
            RetentionMode::TranscriptRetainedProviderPerContract,
            BackgroundListeningState::OffDefault,
            None,
            None,
            "cmd:command_palette.open",
        ),
        "docs:anchor:voice:hosted_provider_overview",
    );
    hosted_row.mic_pill = Some(mic_pill(
        "voice:pill:hosted_command_preview",
        true,
        VoiceModeClass::CommandModeActive,
        MicIndicatorClass::PersistentIndicatorVisibleCaptureActive,
        ProcessingLocalityCue::HostedRemoteDisclosed,
        "command_palette",
        "a11y:voice:mic_active_command_mode_hosted",
    ));
    hosted_row.transcript_strip = Some(transcript_strip(
        "voice:strip:hosted_command_preview",
        ConfidenceCue::High,
        TranscriptCorrectionPosture::CorrectionRequiredBeforeCommit,
        "label:voice:transcript:push_branch_phrase",
    ));
    hosted_row.disambiguation_sheet = Some(CommandDisambiguationSheet {
        record_kind: "shell_voice_command_disambiguation_sheet_record".to_owned(),
        schema_version: VOICE_PREVIEW_SCHEMA_VERSION,
        shared_contract_ref: VOICE_PREVIEW_SHARED_CONTRACT_REF.to_owned(),
        sheet_id: "voice:sheet:hosted_command_preview".to_owned(),
        candidates: vec![CommandDisambiguationCandidate {
            candidate_command_id: "cmd:git.push_current_branch".to_owned(),
            candidate_label_ref: "label:git.push_current_branch".to_owned(),
            confidence_cue: ConfidenceCue::High,
            capability_scope_class: VoiceCapabilityScope::IrreversiblePublish,
            preview_required: true,
        }],
        confirm_command_id: "cmd:voice.confirm_disambiguation".to_owned(),
        cancel_command_id: "cmd:voice.cancel_disambiguation".to_owned(),
        preview_state_for_risky: "preview_and_approval_required_shown".to_owned(),
        accessibility_label_ref: "a11y:voice:disambiguation_sheet".to_owned(),
        redaction_class: REDACTION_CLASS.to_owned(),
    });
    hosted_row.command_resolutions = vec![resolution(
        "voice:resolution:push_current_branch",
        "label:voice:phrase:push_current_branch",
        ConfidenceCue::High,
        VoiceCommandResolutionClass::ResolvesToCanonicalCommandId,
        Some("cmd:git.push_current_branch"),
        Some("git.push_current_branch"),
        Some(VoiceLifecycleLabel::Preview),
        VoiceCapabilityScope::IrreversiblePublish,
        "irreversible_publish_preview",
        "approval_required",
        EnablementDecisionClass::Enabled,
        None,
        true,
        true,
    )];

    // 4. Claimed beta row that is currently unavailable: provider/no-mic, with
    //    a typed unavailable reason, a banner, and a keyboard fallback. The
    //    one resolution shows the shared disabled-reason vocabulary.
    let mut unavailable_row = base_row(
        "voice:row:provider_unavailable_fallback",
        "label:voice:provider_unavailable",
        VoiceClaimPosture::ClaimedBeta,
        true,
        true,
        true,
        true,
        VoiceActivationClass::PushToTalkHeld,
        provider_privacy_row(
            "voice:privacy:provider_unavailable_fallback",
            "label:voice:provider:none_available",
            ProcessingLocalityCue::ProcessingUnavailable,
            RetentionMode::NoAudioNoTranscriptRetained,
            BackgroundListeningState::OffDefault,
            None,
            Some(VoiceUnavailableReason::NoMicrophone),
            "cmd:command_palette.open",
        ),
        "docs:anchor:voice:unavailable_states_overview",
    );
    unavailable_row.mic_pill = Some(mic_pill(
        "voice:pill:provider_unavailable_fallback",
        false,
        VoiceModeClass::IdleMicrophoneOff,
        MicIndicatorClass::PersistentIndicatorUnavailableDegraded,
        ProcessingLocalityCue::ProcessingUnavailable,
        "status_zone",
        "a11y:voice:mic_unavailable_no_microphone",
    ));
    unavailable_row.unavailable_banner = Some(VoiceUnavailableBanner {
        record_kind: "shell_voice_unavailable_banner_record".to_owned(),
        schema_version: VOICE_PREVIEW_SCHEMA_VERSION,
        shared_contract_ref: VOICE_PREVIEW_SHARED_CONTRACT_REF.to_owned(),
        banner_id: "voice:banner:provider_unavailable_fallback".to_owned(),
        unavailable_reason: VoiceUnavailableReason::NoMicrophone,
        message_ref: "label:voice:banner:no_microphone_use_keyboard".to_owned(),
        keyboard_fallback_command_id: "cmd:command_palette.open".to_owned(),
        accessibility_label_ref: "a11y:voice:unavailable_banner".to_owned(),
        redaction_class: REDACTION_CLASS.to_owned(),
    });
    unavailable_row.command_resolutions = vec![resolution(
        "voice:resolution:blocked_no_microphone",
        "label:voice:phrase:rename_symbol_blocked",
        ConfidenceCue::Low,
        VoiceCommandResolutionClass::ResolutionBlockedByEnvelope,
        Some("cmd:edit.rename_symbol_across_project"),
        Some("edit.rename_symbol_across_project"),
        Some(VoiceLifecycleLabel::Beta),
        VoiceCapabilityScope::InertMetadataOnly,
        "structured_diff_preview",
        "no_approval_required",
        EnablementDecisionClass::DisabledWithReason,
        Some(DisabledReasonCode::ExecutionContextUnavailable),
        false,
        false,
    )];

    // 5. Unclaimed Labs row: voice stays suppressed, hidden, and unadvertised.
    let mut labs_row = base_row(
        "voice:row:labs_unadvertised_continuous",
        "label:voice:labs_unadvertised",
        VoiceClaimPosture::LabsUnadvertised,
        false,
        false,
        true,
        true,
        VoiceActivationClass::ActivationUnavailableInEnvelope,
        provider_privacy_row(
            "voice:privacy:labs_unadvertised_continuous",
            "label:voice:provider:labs_disabled",
            ProcessingLocalityCue::ProcessingUnavailable,
            RetentionMode::RetentionUnavailableInEnvelope,
            BackgroundListeningState::OffDefault,
            Some("label:voice:policy:continuous_listening_labs_only"),
            Some(VoiceUnavailableReason::PolicyLockedOrBlocked),
            "cmd:command_palette.open",
        ),
        "docs:anchor:voice:labs_unadvertised_overview",
    );
    labs_row.unavailable_banner = Some(VoiceUnavailableBanner {
        record_kind: "shell_voice_unavailable_banner_record".to_owned(),
        schema_version: VOICE_PREVIEW_SCHEMA_VERSION,
        shared_contract_ref: VOICE_PREVIEW_SHARED_CONTRACT_REF.to_owned(),
        banner_id: "voice:banner:labs_unadvertised_continuous".to_owned(),
        unavailable_reason: VoiceUnavailableReason::PolicyLockedOrBlocked,
        message_ref: "label:voice:banner:labs_only_not_advertised".to_owned(),
        keyboard_fallback_command_id: "cmd:command_palette.open".to_owned(),
        accessibility_label_ref: "a11y:voice:labs_banner".to_owned(),
        redaction_class: REDACTION_CLASS.to_owned(),
    });

    build_voice_preview_beta_page(vec![
        command_row,
        dictation_row,
        hosted_row,
        unavailable_row,
        labs_row,
    ])
}
