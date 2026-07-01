//! Canonical seed for the M5 device-permission set, plus the two narrowed
//! scenario records used as protected fixtures.
//!
//! The seed builder is the single mint-from-truth path: the checked-in support
//! export, governance summary, matrix CSV, and fixtures are projections of these
//! functions, and the module tests prove the on-disk artifacts deserialize back
//! to exactly these values.

use super::{
    CaptureClass, CaptureExportReview, CaptureRedactionState, CaptureReviewActionClass,
    ConfidenceCue, DataExitBoundary, DeviceClass, DevicePermissionRow, M5DevicePermissionSet,
    MicPillState, MicStatePill, PermissionActionClass, PermissionActor, PermissionState,
    ProcessingLocalityCue, RetentionMode, TranscriptCorrectionPosture, VoiceCapabilityScope,
    VoiceUnavailableReason, DEVICE_PERMISSION_ROW_RECORD_KIND,
    DEVICE_PERMISSION_ROW_SCHEMA_VERSION, M5_DEVICE_PERMISSION_CONTRACT_DOC_REF,
    M5_DEVICE_PERMISSION_ROW_SCHEMA_REF, M5_DEVICE_PERMISSION_SET_RECORD_KIND,
    M5_DEVICE_PERMISSION_SET_SCHEMA_VERSION, M5_DEVICE_PERMISSION_VOICE_MATRIX_REF,
    M5_MIC_STATE_PILL_SCHEMA_REF,
};

/// Stable id of the canonical device-permission set.
pub const M5_DEVICE_PERMISSION_SET_ID: &str = "m5_device_permission_set:default";

#[allow(clippy::too_many_arguments)]
fn permission_row(
    row_id: &str,
    device_class: DeviceClass,
    permission_state: PermissionState,
    controlling_actor: PermissionActor,
    processing_locality: ProcessingLocalityCue,
    retention_mode: RetentionMode,
    data_exit_boundary: DataExitBoundary,
    capture_active: bool,
    available_actions: Vec<PermissionActionClass>,
    device_label: &str,
    storage_retention_note: &str,
    actor_note: &str,
) -> DevicePermissionRow {
    DevicePermissionRow {
        device_permission_row_schema_version: DEVICE_PERMISSION_ROW_SCHEMA_VERSION,
        record_kind: DEVICE_PERMISSION_ROW_RECORD_KIND.to_owned(),
        row_id: row_id.to_owned(),
        device_class,
        permission_state,
        controlling_actor,
        processing_locality,
        retention_mode,
        data_exit_boundary,
        capture_active,
        available_actions,
        device_label: device_label.to_owned(),
        storage_retention_note: storage_retention_note.to_owned(),
        actor_note: actor_note.to_owned(),
        contract_doc_ref: M5_DEVICE_PERMISSION_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

fn permission_rows() -> Vec<DevicePermissionRow> {
    use PermissionActionClass as A;
    vec![
        permission_row(
            "device_permission_row:microphone",
            DeviceClass::Microphone,
            PermissionState::GrantedIdle,
            PermissionActor::User,
            ProcessingLocalityCue::LocalOnDevice,
            RetentionMode::EphemeralAudioLocalOnlyNoTranscriptRetained,
            DataExitBoundary::NoPayloadLeavesProduct,
            false,
            vec![A::RevokeInApp, A::OpenSystemSettings, A::ReviewCapture, A::MuteNow],
            "Microphone",
            "Audio is processed on-device and held only ephemerally; no transcript is retained and nothing leaves the machine.",
            "You granted microphone access; you can revoke it here or in system settings at any time.",
        ),
        permission_row(
            "device_permission_row:camera",
            DeviceClass::Camera,
            PermissionState::NotYetRequested,
            PermissionActor::OperatingSystem,
            ProcessingLocalityCue::ProcessingUnavailable,
            RetentionMode::NoAudioNoTranscriptRetained,
            DataExitBoundary::NoPayloadLeavesProduct,
            false,
            vec![A::OpenSystemSettings, A::RequestAccess],
            "Camera",
            "Camera access has not been requested; no video is captured or retained.",
            "The operating system owns this grant; Aureline will prompt only when you request access.",
        ),
        permission_row(
            "device_permission_row:screen_capture",
            DeviceClass::ScreenCapture,
            PermissionState::GrantedInUse,
            PermissionActor::User,
            ProcessingLocalityCue::LocalOnDevice,
            RetentionMode::NoAudioNoTranscriptRetained,
            DataExitBoundary::NoPayloadLeavesProduct,
            true,
            vec![A::RevokeInApp, A::OpenSystemSettings, A::ReviewCapture, A::MuteNow],
            "Screen capture",
            "Screen capture is live and processed on-device; frames are not retained and never leave the machine.",
            "You granted screen capture; revoke it here or mute it now to stop capture immediately.",
        ),
        permission_row(
            "device_permission_row:system_audio_capture",
            DeviceClass::SystemAudioCapture,
            PermissionState::GrantedIdle,
            PermissionActor::ConnectedProvider,
            ProcessingLocalityCue::HostedRemoteDisclosed,
            RetentionMode::TranscriptRetainedProviderPerContract,
            DataExitBoundary::VendorOrThirdPartyOutbound,
            false,
            vec![A::RevokeInApp, A::OpenSystemSettings, A::ReviewCapture, A::MuteNow],
            "System audio",
            "System audio is handed to a connected provider that retains a transcript per its contract; this is not local processing.",
            "A connected provider is in the capture path; revoke the grant to stop sending audio to it.",
        ),
        permission_row(
            "device_permission_row:clipboard",
            DeviceClass::Clipboard,
            PermissionState::BlockedByPolicy,
            PermissionActor::AdministratorPolicy,
            ProcessingLocalityCue::ProcessingUnavailable,
            RetentionMode::NoAudioNoTranscriptRetained,
            DataExitBoundary::NoPayloadLeavesProduct,
            false,
            vec![A::OpenSystemSettings],
            "Clipboard",
            "Clipboard access is blocked by policy; nothing is read or retained.",
            "An administrator policy blocks this grant; you cannot loosen it from here.",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn mic_pill(
    pill_id: &str,
    pill_state: MicPillState,
    processing_locality: ProcessingLocalityCue,
    correction_posture: TranscriptCorrectionPosture,
    confidence_cue: Option<ConfidenceCue>,
    command_capability_scope: VoiceCapabilityScope,
    preview_required_before_commit: bool,
    indicator_visible: bool,
    unavailable_reason: Option<VoiceUnavailableReason>,
    pill_label: &str,
    state_summary: &str,
) -> MicStatePill {
    MicStatePill {
        pill_id: pill_id.to_owned(),
        pill_state,
        processing_locality,
        correction_posture,
        confidence_cue,
        command_capability_scope,
        preview_required_before_commit,
        indicator_visible,
        unavailable_reason,
        pill_label: pill_label.to_owned(),
        state_summary: state_summary.to_owned(),
    }
}

fn mic_pills() -> Vec<MicStatePill> {
    use MicPillState as S;
    use ProcessingLocalityCue as P;
    use TranscriptCorrectionPosture as C;
    vec![
        mic_pill(
            "mic_state_pill:idle",
            S::Idle,
            P::LocalOnDevice,
            C::CorrectionOptionalBeforeCommit,
            None,
            VoiceCapabilityScope::InertMetadataOnly,
            false,
            true,
            None,
            "Idle",
            "Microphone is available but not capturing; processing would be on-device.",
        ),
        mic_pill(
            "mic_state_pill:listening",
            S::Listening,
            P::LocalOnDevice,
            C::CorrectionOptionalBeforeCommit,
            Some(ConfidenceCue::High),
            VoiceCapabilityScope::ReversibleLocalMutation,
            false,
            true,
            None,
            "Listening",
            "Capturing audio on-device with a visible indicator; a reversible edit needs no confirmation.",
        ),
        mic_pill(
            "mic_state_pill:muted",
            S::Muted,
            P::LocalOnDevice,
            C::CorrectionOptionalBeforeCommit,
            None,
            VoiceCapabilityScope::InertMetadataOnly,
            false,
            true,
            None,
            "Muted",
            "Access exists but capture is suppressed; unmute to resume.",
        ),
        mic_pill(
            "mic_state_pill:processing",
            S::Processing,
            P::HostedRemoteDisclosed,
            C::CorrectionRequiredBeforeCommit,
            Some(ConfidenceCue::Medium),
            VoiceCapabilityScope::ReversibleLocalRead,
            false,
            true,
            None,
            "Processing",
            "Finishing recognition through a disclosed hosted engine; the transcript can be corrected before use.",
        ),
        mic_pill(
            "mic_state_pill:needs_confirmation",
            S::NeedsConfirmation,
            P::LocalOnDevice,
            C::CorrectionRequiredBeforeCommit,
            Some(ConfidenceCue::Low),
            VoiceCapabilityScope::RecoverableDurableMutation,
            true,
            true,
            None,
            "Needs confirmation",
            "A high-impact spoken command awaits explicit confirmation; correct the transcript, then confirm the preview to commit.",
        ),
        mic_pill(
            "mic_state_pill:unavailable",
            S::Unavailable,
            P::ProcessingUnavailable,
            C::CorrectionUnavailableCaptureOnly,
            None,
            VoiceCapabilityScope::InertMetadataOnly,
            false,
            false,
            Some(VoiceUnavailableReason::NoMicrophone),
            "Unavailable",
            "Voice capture is unavailable because no microphone is present.",
        ),
        mic_pill(
            "mic_state_pill:policy_blocked",
            S::PolicyBlocked,
            P::ProcessingUnavailable,
            C::CorrectionBlockedByEnvelope,
            None,
            VoiceCapabilityScope::InertMetadataOnly,
            false,
            false,
            Some(VoiceUnavailableReason::PolicyLockedOrBlocked),
            "Policy blocked",
            "Voice capture is blocked by policy in this context.",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn capture_review(
    review_id: &str,
    included_capture_classes: Vec<CaptureClass>,
    retention_mode: RetentionMode,
    redaction_state: CaptureRedactionState,
    processing_locality: ProcessingLocalityCue,
    data_exit_boundary: DataExitBoundary,
    delete_available: bool,
    export_available: bool,
    available_actions: Vec<CaptureReviewActionClass>,
    review_label: &str,
    review_summary: &str,
) -> CaptureExportReview {
    CaptureExportReview {
        review_id: review_id.to_owned(),
        included_capture_classes,
        retention_mode,
        redaction_state,
        processing_locality,
        data_exit_boundary,
        delete_available,
        export_available,
        available_actions,
        review_label: review_label.to_owned(),
        review_summary: review_summary.to_owned(),
    }
}

fn capture_reviews() -> Vec<CaptureExportReview> {
    use CaptureReviewActionClass as A;
    vec![
        capture_review(
            "capture_export_review:voice_session",
            vec![CaptureClass::LiveAudioStream, CaptureClass::Transcript],
            RetentionMode::EphemeralAudioLocalOnlyNoTranscriptRetained,
            CaptureRedactionState::RawNeverExported,
            ProcessingLocalityCue::LocalOnDevice,
            DataExitBoundary::NoPayloadLeavesProduct,
            true,
            false,
            vec![A::DeleteNow, A::ReviewInline, A::RevokeAndPurge],
            "Voice session capture",
            "Live audio and transcript stay on-device; the raw capture is never exported and can be deleted or purged.",
        ),
        capture_review(
            "capture_export_review:screen_and_clipboard",
            vec![
                CaptureClass::Screenshot,
                CaptureClass::ScreenRecording,
                CaptureClass::ClipboardSnapshot,
            ],
            RetentionMode::TranscriptRetainedRedactedInSupportBundle,
            CaptureRedactionState::RedactedBeforeExport,
            ProcessingLocalityCue::HostedRemoteDisclosed,
            DataExitBoundary::RedactedSupportPacket,
            true,
            true,
            vec![A::DeleteNow, A::ExportRedactedCopy, A::ReviewInline, A::RevokeAndPurge],
            "Screen & clipboard capture",
            "Screenshots, recordings, and clipboard snapshots are redacted before any export; delete or export a redaction-safe copy.",
        ),
        capture_review(
            "capture_export_review:device_inventory",
            vec![CaptureClass::DeviceInventory],
            RetentionMode::NoAudioNoTranscriptRetained,
            CaptureRedactionState::MetadataRefsOnly,
            ProcessingLocalityCue::LocalOnDevice,
            DataExitBoundary::NoPayloadLeavesProduct,
            false,
            true,
            vec![A::ExportRedactedCopy, A::ReviewInline],
            "Device inventory",
            "Only redaction-safe device-class metadata is carried; you can export the metadata-only refs.",
        ),
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_DEVICE_PERMISSION_ROW_SCHEMA_REF.to_owned(),
        M5_MIC_STATE_PILL_SCHEMA_REF.to_owned(),
        M5_DEVICE_PERMISSION_CONTRACT_DOC_REF.to_owned(),
        M5_DEVICE_PERMISSION_VOICE_MATRIX_REF.to_owned(),
    ]
}

/// Build the canonical M5 device-permission set.
pub fn seeded_m5_device_permission_set() -> M5DevicePermissionSet {
    M5DevicePermissionSet {
        schema_version: M5_DEVICE_PERMISSION_SET_SCHEMA_VERSION,
        record_kind: M5_DEVICE_PERMISSION_SET_RECORD_KIND.to_owned(),
        set_id: M5_DEVICE_PERMISSION_SET_ID.to_owned(),
        set_label: "M5 device-permission & capture review".to_owned(),
        permission_rows: permission_rows(),
        mic_pills: mic_pills(),
        capture_reviews: capture_reviews(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "mint.m5_device_permission_set".to_owned(),
        contract_doc_ref: M5_DEVICE_PERMISSION_CONTRACT_DOC_REF.to_owned(),
    }
}

/// A standalone mic-state pill for an irreversible high-impact spoken command:
/// it must sit in needs-confirmation with a required transcript-correction strip
/// and a required preview before commit.
pub fn seeded_high_impact_confirmation_pill() -> MicStatePill {
    mic_pill(
        "mic_state_pill:publish_confirmation",
        MicPillState::NeedsConfirmation,
        ProcessingLocalityCue::LocalOnDevice,
        TranscriptCorrectionPosture::CorrectionRequiredBeforeCommit,
        Some(ConfidenceCue::Low),
        VoiceCapabilityScope::IrreversiblePublish,
        true,
        true,
        None,
        "Confirm publish",
        "An irreversible publish was heard; correct the transcript and confirm the preview before it commits.",
    )
}

/// A standalone capture/export review whose audio and transcript are retained by
/// a connected provider per contract: processing is disclosed as hosted, never
/// claimed local, and the export stays redaction-bounded.
pub fn seeded_provider_backed_capture_review() -> CaptureExportReview {
    use CaptureReviewActionClass as A;
    capture_review(
        "capture_export_review:provider_backed_voice",
        vec![CaptureClass::LiveAudioStream, CaptureClass::Transcript],
        RetentionMode::TranscriptRetainedProviderPerContract,
        CaptureRedactionState::RedactedBeforeExport,
        ProcessingLocalityCue::HostedRemoteDisclosed,
        DataExitBoundary::RedactedSupportPacket,
        true,
        true,
        vec![A::DeleteNow, A::ExportRedactedCopy, A::ReviewInline, A::RevokeAndPurge],
        "Provider-backed voice capture",
        "A connected provider retains audio and transcript per contract; processing is disclosed as hosted and any export is redacted first.",
    )
}
