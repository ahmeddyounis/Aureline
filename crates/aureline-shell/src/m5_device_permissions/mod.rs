//! Device-permission rows, mic-state pills, transcript-correction strips, and
//! capture/export reviews for the M5 voice, help, and support surfaces.
//!
//! This module is the in-product producer of the durable
//! [`M5DevicePermissionSet`] that makes device and capture state an explicit,
//! reversible product surface rather than a hidden OS side effect. It answers, at
//! a glance and field by field, three questions a voice- or capture-capable
//! surface must never leave implicit:
//!
//! - **What device class is accessible, and who controls it?** Each
//!   [`DevicePermissionRow`] names a [`DeviceClass`], its [`PermissionState`], the
//!   [`PermissionActor`] that controls the grant, the storage/retention note
//!   ([`RetentionMode`]), and the reversible [`PermissionActionClass`] actions —
//!   always including *open system settings*, and *revoke in app* whenever the
//!   grant is live. Capture is never implicitly always-on: a row may only report
//!   `capture_active` while its permission state is [`PermissionState::GrantedInUse`].
//! - **Is capture active, local, provider-backed, blocked, or unavailable?** Each
//!   [`MicStatePill`] pins one of the seven [`MicPillState`] states — idle,
//!   listening, muted, processing, needs-confirmation, unavailable, policy-blocked
//!   — with a [`ProcessingLocalityCue`] that never claims local processing when a
//!   provider is in the path. A high-impact spoken command
//!   ([`VoiceCapabilityScope::is_high_impact`]) is forced through the same
//!   preview/confirmation gate as any other mutating action: the pill must sit in
//!   [`MicPillState::NeedsConfirmation`] with transcript correction *required*
//!   before commit.
//! - **What capture will be retained or exported, and can I delete it?** Each
//!   [`CaptureExportReview`] names the included [`CaptureClass`] set, the retention
//!   mode, the [`CaptureRedactionState`], the [`DataExitBoundary`], and whether
//!   delete and redacted export are available — so a review stays privacy-bounded
//!   and reusable in support/help flows.
//!
//! The [`ProcessingLocalityCue`], [`RetentionMode`], [`TranscriptCorrectionPosture`],
//! [`ConfidenceCue`], [`VoiceCapabilityScope`], and [`VoiceUnavailableReason`]
//! vocabulary is reused from the frozen voice contract ([`crate::voice`]) and the
//! [`DataExitBoundary`] from the About/help destination contract
//! ([`crate::public_truth`]), so a device-permission surface declares the same
//! versioned, redaction-safe vocabulary the voice and community-handoff lanes
//! already publish.
//!
//! Raw URLs, raw email addresses, raw local paths, raw usernames, raw hostnames,
//! tokens, and raw secret material never cross this boundary; the records carry
//! opaque refs, controlled-vocabulary tokens, and bounded reviewable sentences
//! only.
//!
//! The row schema is
//! [`schemas/help/m5-device-permission-row.schema.json`](../../../../schemas/help/m5-device-permission-row.schema.json)
//! and the mic-pill schema is
//! [`schemas/help/m5-mic-state-pill.schema.json`](../../../../schemas/help/m5-mic-state-pill.schema.json).
//! The contract doc is
//! [`docs/help/m5_device_permissions_contract.md`](../../../../docs/help/m5_device_permissions_contract.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_high_impact_confirmation_pill, seeded_m5_device_permission_set,
    seeded_provider_backed_capture_review, M5_DEVICE_PERMISSION_SET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::public_truth::DataExitBoundary;
pub use crate::voice::{
    ConfidenceCue, ProcessingLocalityCue, RetentionMode, TranscriptCorrectionPosture,
    VoiceCapabilityScope, VoiceUnavailableReason,
};

/// Stable record-kind tag carried by [`DevicePermissionRow`].
pub const DEVICE_PERMISSION_ROW_RECORD_KIND: &str = "device_permission_row";

/// Stable record-kind tag carried by [`M5DevicePermissionSet`].
pub const M5_DEVICE_PERMISSION_SET_RECORD_KIND: &str = "m5_device_permission_set";

/// Schema version for a single device-permission row.
pub const DEVICE_PERMISSION_ROW_SCHEMA_VERSION: u32 = 1;

/// Schema version for the bundled device-permission set.
pub const M5_DEVICE_PERMISSION_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the device-permission-row schema this producer projects.
pub const M5_DEVICE_PERMISSION_ROW_SCHEMA_REF: &str =
    "schemas/help/m5-device-permission-row.schema.json";

/// Repo-relative path of the mic-state-pill schema this producer projects.
pub const M5_MIC_STATE_PILL_SCHEMA_REF: &str = "schemas/help/m5-mic-state-pill.schema.json";

/// Repo-relative path of the contract doc all records point at.
pub const M5_DEVICE_PERMISSION_CONTRACT_DOC_REF: &str =
    "docs/help/m5_device_permissions_contract.md";

/// Repo-relative path of the frozen voice mode/provider/transcript/retention
/// matrix this lane's mic-state and retention vocabulary builds on.
pub const M5_DEVICE_PERMISSION_VOICE_MATRIX_REF: &str =
    "schemas/help/m5-public-handoff-matrix.schema.json";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DEVICE_PERMISSION_ARTIFACT_REF: &str =
    "artifacts/help/m5-device-permission-proof/permission_set.json";

/// The closed set of device classes whose access Aureline exposes as an explicit,
/// reversible permission row rather than a hidden OS side effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceClass {
    /// The microphone / audio input device.
    Microphone,
    /// The camera / video input device.
    Camera,
    /// Screen-capture / screen-share access.
    ScreenCapture,
    /// System-audio (loopback) capture.
    SystemAudioCapture,
    /// Clipboard read access.
    Clipboard,
}

impl DeviceClass {
    /// Every device class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Microphone,
        Self::Camera,
        Self::ScreenCapture,
        Self::SystemAudioCapture,
        Self::Clipboard,
    ];

    /// Stable token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Microphone => "microphone",
            Self::Camera => "camera",
            Self::ScreenCapture => "screen_capture",
            Self::SystemAudioCapture => "system_audio_capture",
            Self::Clipboard => "clipboard",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Microphone => "Microphone",
            Self::Camera => "Camera",
            Self::ScreenCapture => "Screen capture",
            Self::SystemAudioCapture => "System audio",
            Self::Clipboard => "Clipboard",
        }
    }
}

/// The closed permission-state vocabulary for a device-permission row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionState {
    /// Granted and actively in use — capture is live.
    GrantedInUse,
    /// Granted but idle — access exists, no capture right now.
    GrantedIdle,
    /// Explicitly denied.
    Denied,
    /// Never requested; the default resting state.
    NotYetRequested,
    /// Previously granted, then revoked by the user.
    RevokedByUser,
    /// Blocked by policy / the deployment envelope.
    BlockedByPolicy,
    /// No device present or the OS reports it unavailable.
    UnavailableNoDevice,
}

impl PermissionState {
    /// Every permission state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::GrantedInUse,
        Self::GrantedIdle,
        Self::Denied,
        Self::NotYetRequested,
        Self::RevokedByUser,
        Self::BlockedByPolicy,
        Self::UnavailableNoDevice,
    ];

    /// Stable token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GrantedInUse => "granted_in_use",
            Self::GrantedIdle => "granted_idle",
            Self::Denied => "denied",
            Self::NotYetRequested => "not_yet_requested",
            Self::RevokedByUser => "revoked_by_user",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::UnavailableNoDevice => "unavailable_no_device",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::GrantedInUse => "Granted — in use",
            Self::GrantedIdle => "Granted — idle",
            Self::Denied => "Denied",
            Self::NotYetRequested => "Not yet requested",
            Self::RevokedByUser => "Revoked by you",
            Self::BlockedByPolicy => "Blocked by policy",
            Self::UnavailableNoDevice => "Unavailable — no device",
        }
    }

    /// True when access exists (granted, idle or in use).
    pub const fn is_granted(self) -> bool {
        matches!(self, Self::GrantedInUse | Self::GrantedIdle)
    }

    /// True only while capture may legitimately be live.
    pub const fn permits_active_capture(self) -> bool {
        matches!(self, Self::GrantedInUse)
    }

    /// True when the user may still request (or re-request) access.
    pub const fn is_requestable(self) -> bool {
        matches!(self, Self::NotYetRequested | Self::Denied | Self::RevokedByUser)
    }
}

/// Who controls the grant behind a device-permission row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionActor {
    /// The user granted or revoked access directly.
    User,
    /// The operating system owns the grant (OS-level prompt / setting).
    OperatingSystem,
    /// An administrator policy owns the grant; the user cannot loosen it.
    AdministratorPolicy,
    /// A connected provider is in the capture path.
    ConnectedProvider,
}

impl PermissionActor {
    /// Every actor, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::User,
        Self::OperatingSystem,
        Self::AdministratorPolicy,
        Self::ConnectedProvider,
    ];

    /// Stable token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::OperatingSystem => "operating_system",
            Self::AdministratorPolicy => "administrator_policy",
            Self::ConnectedProvider => "connected_provider",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::User => "You",
            Self::OperatingSystem => "Operating system",
            Self::AdministratorPolicy => "Administrator policy",
            Self::ConnectedProvider => "Connected provider",
        }
    }

    /// True when a provider or connector is in the capture path, so a row may not
    /// imply purely local processing.
    pub const fn is_provider_backed(self) -> bool {
        matches!(self, Self::ConnectedProvider)
    }
}

/// The reversible actions offered on a device-permission row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionActionClass {
    /// Revoke the grant from within Aureline.
    RevokeInApp,
    /// Open the operating-system settings for this device class.
    OpenSystemSettings,
    /// Request (or re-request) access.
    RequestAccess,
    /// Review what was captured under this grant.
    ReviewCapture,
    /// Mute capture immediately without revoking the grant.
    MuteNow,
}

impl PermissionActionClass {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevokeInApp,
        Self::OpenSystemSettings,
        Self::RequestAccess,
        Self::ReviewCapture,
        Self::MuteNow,
    ];

    /// Stable token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevokeInApp => "revoke_in_app",
            Self::OpenSystemSettings => "open_system_settings",
            Self::RequestAccess => "request_access",
            Self::ReviewCapture => "review_capture",
            Self::MuteNow => "mute_now",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RevokeInApp => "Revoke in app",
            Self::OpenSystemSettings => "Open system settings",
            Self::RequestAccess => "Request access",
            Self::ReviewCapture => "Review capture",
            Self::MuteNow => "Mute now",
        }
    }
}

/// The seven mic-state pill states a voice-capable surface exposes so capture is
/// legible at a glance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MicPillState {
    /// Microphone available but not capturing.
    Idle,
    /// Actively capturing audio.
    Listening,
    /// Muted: access exists but capture is suppressed.
    Muted,
    /// Finishing recognition / handing a transcript to processing.
    Processing,
    /// A resolved command is awaiting explicit confirmation before commit.
    NeedsConfirmation,
    /// Voice capture is unavailable in the current state.
    Unavailable,
    /// Voice capture is blocked by policy.
    PolicyBlocked,
}

impl MicPillState {
    /// Every pill state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Idle,
        Self::Listening,
        Self::Muted,
        Self::Processing,
        Self::NeedsConfirmation,
        Self::Unavailable,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded on the pill.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Listening => "listening",
            Self::Muted => "muted",
            Self::Processing => "processing",
            Self::NeedsConfirmation => "needs_confirmation",
            Self::Unavailable => "unavailable",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::Listening => "Listening",
            Self::Muted => "Muted",
            Self::Processing => "Processing",
            Self::NeedsConfirmation => "Needs confirmation",
            Self::Unavailable => "Unavailable",
            Self::PolicyBlocked => "Policy blocked",
        }
    }

    /// True while audio is actively being captured or drained.
    pub const fn is_capturing(self) -> bool {
        matches!(self, Self::Listening | Self::Processing)
    }

    /// True for states where voice capability is off (blocked or unavailable).
    pub const fn is_off(self) -> bool {
        matches!(self, Self::Unavailable | Self::PolicyBlocked)
    }
}

/// The closed set of capture classes a capture/export review may name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureClass {
    /// A live audio stream.
    LiveAudioStream,
    /// A recognized transcript.
    Transcript,
    /// A single screenshot.
    Screenshot,
    /// A screen recording.
    ScreenRecording,
    /// A clipboard snapshot.
    ClipboardSnapshot,
    /// A device inventory (class facts about available devices).
    DeviceInventory,
}

impl CaptureClass {
    /// Every capture class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiveAudioStream,
        Self::Transcript,
        Self::Screenshot,
        Self::ScreenRecording,
        Self::ClipboardSnapshot,
        Self::DeviceInventory,
    ];

    /// Stable token recorded on the review.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveAudioStream => "live_audio_stream",
            Self::Transcript => "transcript",
            Self::Screenshot => "screenshot",
            Self::ScreenRecording => "screen_recording",
            Self::ClipboardSnapshot => "clipboard_snapshot",
            Self::DeviceInventory => "device_inventory",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LiveAudioStream => "Live audio stream",
            Self::Transcript => "Transcript",
            Self::Screenshot => "Screenshot",
            Self::ScreenRecording => "Screen recording",
            Self::ClipboardSnapshot => "Clipboard snapshot",
            Self::DeviceInventory => "Device inventory",
        }
    }
}

/// The redaction state applied to a capture/export review before anything can be
/// exported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureRedactionState {
    /// The capture is redacted before it may be exported.
    RedactedBeforeExport,
    /// The raw capture stays local and is never exported in any form.
    RawNeverExported,
    /// Only redaction-safe metadata / object refs may leave, no payload bodies.
    MetadataRefsOnly,
}

impl CaptureRedactionState {
    /// Every redaction state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::RedactedBeforeExport,
        Self::RawNeverExported,
        Self::MetadataRefsOnly,
    ];

    /// Stable token recorded on the review.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedactedBeforeExport => "redacted_before_export",
            Self::RawNeverExported => "raw_never_exported",
            Self::MetadataRefsOnly => "metadata_refs_only",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RedactedBeforeExport => "Redacted before export",
            Self::RawNeverExported => "Raw never exported",
            Self::MetadataRefsOnly => "Metadata refs only",
        }
    }

    /// True when this redaction state allows a (redaction-safe) export at all.
    pub const fn allows_export(self) -> bool {
        matches!(self, Self::RedactedBeforeExport | Self::MetadataRefsOnly)
    }

    /// Whether this redaction state permits the given data-exit boundary.
    pub fn allows_data_exit(self, data_exit: DataExitBoundary) -> bool {
        use DataExitBoundary as D;
        match self {
            // A raw capture that never leaves the machine allows no payload exit.
            Self::RawNeverExported => matches!(data_exit, D::NoPayloadLeavesProduct),
            // Metadata-only reviews carry refs, never payload bodies.
            Self::MetadataRefsOnly => {
                matches!(data_exit, D::NoPayloadLeavesProduct | D::MetadataSafeObjectRefs)
            }
            // A redacted capture may ride a redaction-safe support/metadata boundary.
            Self::RedactedBeforeExport => matches!(
                data_exit,
                D::NoPayloadLeavesProduct | D::MetadataSafeObjectRefs | D::RedactedSupportPacket
            ),
        }
    }
}

/// The reversible actions offered on a capture/export review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureReviewActionClass {
    /// Delete the captured data now.
    DeleteNow,
    /// Export a redaction-safe copy.
    ExportRedactedCopy,
    /// Review the capture inline before deciding.
    ReviewInline,
    /// Revoke the grant and purge everything captured under it.
    RevokeAndPurge,
}

impl CaptureReviewActionClass {
    /// Every review action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DeleteNow,
        Self::ExportRedactedCopy,
        Self::ReviewInline,
        Self::RevokeAndPurge,
    ];

    /// Stable token recorded on the review.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeleteNow => "delete_now",
            Self::ExportRedactedCopy => "export_redacted_copy",
            Self::ReviewInline => "review_inline",
            Self::RevokeAndPurge => "revoke_and_purge",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DeleteNow => "Delete now",
            Self::ExportRedactedCopy => "Export redacted copy",
            Self::ReviewInline => "Review inline",
            Self::RevokeAndPurge => "Revoke and purge",
        }
    }
}

/// True when a retention mode keeps a transcript with a provider per contract, so
/// a row may not claim purely local processing.
pub fn retention_is_provider_backed(retention: RetentionMode) -> bool {
    matches!(retention, RetentionMode::TranscriptRetainedProviderPerContract)
}

/// One device-permission row: a device class, its permission state, the actor
/// that controls it, its storage/retention note, and the reversible actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DevicePermissionRow {
    /// Schema version for this row shape.
    pub device_permission_row_schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable row id; prefixed `device_permission_row:`.
    pub row_id: String,
    /// The device class this row governs.
    pub device_class: DeviceClass,
    /// The current permission state.
    pub permission_state: PermissionState,
    /// The actor that controls the grant.
    pub controlling_actor: PermissionActor,
    /// The local-or-hosted processing cue for capture under this grant.
    pub processing_locality: ProcessingLocalityCue,
    /// The storage/retention mode disclosed for capture under this grant.
    pub retention_mode: RetentionMode,
    /// The data-exit boundary capture under this grant obeys.
    pub data_exit_boundary: DataExitBoundary,
    /// Whether capture is live right now; only true in the in-use state.
    pub capture_active: bool,
    /// The reversible actions offered on this row.
    pub available_actions: Vec<PermissionActionClass>,
    /// Reviewer-facing device label.
    pub device_label: String,
    /// A bounded reviewable sentence describing storage/retention.
    pub storage_retention_note: String,
    /// A bounded reviewable sentence describing who controls the grant.
    pub actor_note: String,
    /// Frozen contract-doc ref.
    pub contract_doc_ref: String,
    /// Optional reviewer note.
    pub notes: Option<String>,
}

impl DevicePermissionRow {
    /// Validate the row against the device-permission contract.
    pub fn validate(&self) -> Result<(), DevicePermissionError> {
        if self.device_permission_row_schema_version != DEVICE_PERMISSION_ROW_SCHEMA_VERSION {
            return Err(DevicePermissionError::WrongRowSchemaVersion {
                row_id: self.row_id.clone(),
                actual: self.device_permission_row_schema_version,
            });
        }
        if self.record_kind != DEVICE_PERMISSION_ROW_RECORD_KIND {
            return Err(DevicePermissionError::WrongRowRecordKind {
                row_id: self.row_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if !self.row_id.starts_with("device_permission_row:") {
            return Err(DevicePermissionError::MalformedRowId {
                row_id: self.row_id.clone(),
            });
        }
        if self.contract_doc_ref != M5_DEVICE_PERMISSION_CONTRACT_DOC_REF {
            return Err(DevicePermissionError::WrongContractDocRef {
                record_id: self.row_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }
        for (field, value) in [
            ("device_label", &self.device_label),
            ("storage_retention_note", &self.storage_retention_note),
            ("actor_note", &self.actor_note),
        ] {
            if non_empty(value).is_none() {
                return Err(DevicePermissionError::EmptyRequiredField {
                    record_id: self.row_id.clone(),
                    field,
                });
            }
        }

        // Capture may be live only while the grant is in use — never implicitly
        // always-on.
        if self.capture_active && !self.permission_state.permits_active_capture() {
            return Err(DevicePermissionError::CaptureActiveWithoutGrant {
                row_id: self.row_id.clone(),
                state: self.permission_state,
            });
        }
        if !self.capture_active && self.permission_state.permits_active_capture() {
            return Err(DevicePermissionError::InUseButNotCapturing {
                row_id: self.row_id.clone(),
            });
        }

        // Local-processing honesty: never claim local processing when a provider
        // is in the path (by actor or by provider-backed retention).
        let provider_in_path = self.controlling_actor.is_provider_backed()
            || retention_is_provider_backed(self.retention_mode);
        if provider_in_path && self.processing_locality == ProcessingLocalityCue::LocalOnDevice {
            return Err(DevicePermissionError::LocalProcessingClaimedWithProvider {
                record_id: self.row_id.clone(),
            });
        }
        if self.processing_locality == ProcessingLocalityCue::LocalOnDevice
            && self.data_exit_boundary != DataExitBoundary::NoPayloadLeavesProduct
        {
            return Err(DevicePermissionError::LocalProcessingLeaksPayload {
                record_id: self.row_id.clone(),
            });
        }

        // Actions: always offer a route to system settings; offer revoke while
        // granted, and request-access while (re-)requestable.
        if self.available_actions.is_empty() {
            return Err(DevicePermissionError::NoActions {
                row_id: self.row_id.clone(),
            });
        }
        let mut seen: BTreeSet<PermissionActionClass> = BTreeSet::new();
        for action in &self.available_actions {
            if !seen.insert(*action) {
                return Err(DevicePermissionError::DuplicateAction {
                    row_id: self.row_id.clone(),
                    action: *action,
                });
            }
        }
        if !seen.contains(&PermissionActionClass::OpenSystemSettings) {
            return Err(DevicePermissionError::MissingSystemSettingsAction {
                row_id: self.row_id.clone(),
            });
        }
        if self.permission_state.is_granted()
            && !seen.contains(&PermissionActionClass::RevokeInApp)
        {
            return Err(DevicePermissionError::MissingRevokeAction {
                row_id: self.row_id.clone(),
            });
        }
        if self.permission_state.is_requestable()
            && !seen.contains(&PermissionActionClass::RequestAccess)
        {
            return Err(DevicePermissionError::MissingRequestAction {
                row_id: self.row_id.clone(),
            });
        }
        Ok(())
    }
}

/// One mic-state pill plus its transcript-correction strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MicStatePill {
    /// Stable pill id; prefixed `mic_state_pill:`.
    pub pill_id: String,
    /// The pill state.
    pub pill_state: MicPillState,
    /// The local-or-hosted processing cue shown on the pill.
    pub processing_locality: ProcessingLocalityCue,
    /// The transcript-correction posture on the correction strip.
    pub correction_posture: TranscriptCorrectionPosture,
    /// The confidence cue on the current transcript segment, when there is one.
    pub confidence_cue: Option<ConfidenceCue>,
    /// The capability scope of the resolved spoken command.
    pub command_capability_scope: VoiceCapabilityScope,
    /// Whether a preview/confirmation is required before commit.
    pub preview_required_before_commit: bool,
    /// Whether the persistent mic indicator is visible.
    pub indicator_visible: bool,
    /// The typed reason capture is off, when the pill is unavailable/blocked.
    pub unavailable_reason: Option<VoiceUnavailableReason>,
    /// Reviewer-facing pill label.
    pub pill_label: String,
    /// A bounded reviewable sentence summarizing the pill state.
    pub state_summary: String,
}

impl MicStatePill {
    /// Validate the pill against the device-permission contract.
    pub fn validate(&self) -> Result<(), DevicePermissionError> {
        if !self.pill_id.starts_with("mic_state_pill:") {
            return Err(DevicePermissionError::MalformedPillId {
                pill_id: self.pill_id.clone(),
            });
        }
        for (field, value) in [
            ("pill_label", &self.pill_label),
            ("state_summary", &self.state_summary),
        ] {
            if non_empty(value).is_none() {
                return Err(DevicePermissionError::EmptyRequiredField {
                    record_id: self.pill_id.clone(),
                    field,
                });
            }
        }

        // Capturing states expose a live indicator and a real processing cue.
        if self.pill_state.is_capturing() {
            if !self.indicator_visible {
                return Err(DevicePermissionError::CapturingWithoutIndicator {
                    pill_id: self.pill_id.clone(),
                });
            }
            if self.processing_locality == ProcessingLocalityCue::ProcessingUnavailable {
                return Err(DevicePermissionError::CapturingWithoutProcessing {
                    pill_id: self.pill_id.clone(),
                });
            }
        }

        // Off states carry a typed reason and no active processing; other states
        // carry no unavailable reason.
        if self.pill_state.is_off() {
            match self.unavailable_reason {
                None => {
                    return Err(DevicePermissionError::OffWithoutReason {
                        pill_id: self.pill_id.clone(),
                    });
                }
                Some(reason) => {
                    if self.pill_state == MicPillState::PolicyBlocked
                        && reason != VoiceUnavailableReason::PolicyLockedOrBlocked
                    {
                        return Err(DevicePermissionError::PolicyBlockedWrongReason {
                            pill_id: self.pill_id.clone(),
                        });
                    }
                }
            }
            if self.processing_locality != ProcessingLocalityCue::ProcessingUnavailable {
                return Err(DevicePermissionError::OffWithActiveProcessing {
                    pill_id: self.pill_id.clone(),
                });
            }
        } else if self.unavailable_reason.is_some() {
            return Err(DevicePermissionError::ReasonOnAvailablePill {
                pill_id: self.pill_id.clone(),
            });
        }

        // High-impact spoken commands ride the same preview/confirmation gate as
        // any other mutating action, with transcript correction required first.
        if self.command_capability_scope.is_high_impact() {
            if self.pill_state != MicPillState::NeedsConfirmation {
                return Err(DevicePermissionError::HighImpactWithoutConfirmation {
                    pill_id: self.pill_id.clone(),
                });
            }
            if self.correction_posture != TranscriptCorrectionPosture::CorrectionRequiredBeforeCommit
            {
                return Err(DevicePermissionError::HighImpactWithoutCorrection {
                    pill_id: self.pill_id.clone(),
                });
            }
            if !self.preview_required_before_commit {
                return Err(DevicePermissionError::HighImpactWithoutPreview {
                    pill_id: self.pill_id.clone(),
                });
            }
        }

        // A needs-confirmation pill always gates commit behind preview + required
        // correction, regardless of scope.
        if self.pill_state == MicPillState::NeedsConfirmation
            && (!self.preview_required_before_commit
                || self.correction_posture
                    != TranscriptCorrectionPosture::CorrectionRequiredBeforeCommit)
        {
            return Err(DevicePermissionError::ConfirmationWithoutGate {
                pill_id: self.pill_id.clone(),
            });
        }

        // A confidence cue only makes sense while there is a transcript segment.
        if self.confidence_cue.is_some()
            && matches!(
                self.pill_state,
                MicPillState::Idle
                    | MicPillState::Muted
                    | MicPillState::Unavailable
                    | MicPillState::PolicyBlocked
            )
        {
            return Err(DevicePermissionError::ConfidenceWithoutTranscript {
                pill_id: self.pill_id.clone(),
            });
        }
        Ok(())
    }
}

/// One capture/export review naming the included capture classes, retention,
/// redaction, and delete/export availability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureExportReview {
    /// Stable review id; prefixed `capture_export_review:`.
    pub review_id: String,
    /// The capture classes included in this review.
    pub included_capture_classes: Vec<CaptureClass>,
    /// The retention mode covering the included capture.
    pub retention_mode: RetentionMode,
    /// The redaction state applied before export.
    pub redaction_state: CaptureRedactionState,
    /// The local-or-hosted processing cue for the captured data.
    pub processing_locality: ProcessingLocalityCue,
    /// The data-exit boundary an export obeys.
    pub data_exit_boundary: DataExitBoundary,
    /// Whether the captured data can be deleted.
    pub delete_available: bool,
    /// Whether a redaction-safe export is available.
    pub export_available: bool,
    /// The reversible actions offered on this review.
    pub available_actions: Vec<CaptureReviewActionClass>,
    /// Reviewer-facing review label.
    pub review_label: String,
    /// A bounded reviewable sentence summarizing the review.
    pub review_summary: String,
}

impl CaptureExportReview {
    /// Validate the review against the device-permission contract.
    pub fn validate(&self) -> Result<(), DevicePermissionError> {
        if !self.review_id.starts_with("capture_export_review:") {
            return Err(DevicePermissionError::MalformedReviewId {
                review_id: self.review_id.clone(),
            });
        }
        for (field, value) in [
            ("review_label", &self.review_label),
            ("review_summary", &self.review_summary),
        ] {
            if non_empty(value).is_none() {
                return Err(DevicePermissionError::EmptyRequiredField {
                    record_id: self.review_id.clone(),
                    field,
                });
            }
        }
        if self.included_capture_classes.is_empty() {
            return Err(DevicePermissionError::EmptyCaptureClasses {
                review_id: self.review_id.clone(),
            });
        }
        let mut seen_classes: BTreeSet<CaptureClass> = BTreeSet::new();
        for class in &self.included_capture_classes {
            if !seen_classes.insert(*class) {
                return Err(DevicePermissionError::DuplicateCaptureClass {
                    review_id: self.review_id.clone(),
                    class: *class,
                });
            }
        }
        if self.available_actions.is_empty() {
            return Err(DevicePermissionError::NoReviewActions {
                review_id: self.review_id.clone(),
            });
        }
        let mut seen_actions: BTreeSet<CaptureReviewActionClass> = BTreeSet::new();
        for action in &self.available_actions {
            if !seen_actions.insert(*action) {
                return Err(DevicePermissionError::DuplicateReviewAction {
                    review_id: self.review_id.clone(),
                    action: *action,
                });
            }
        }

        // Export honesty: a review may only advertise export when its redaction
        // state permits a redaction-safe export, and it must offer the action.
        if self.export_available {
            if !self.redaction_state.allows_export() {
                return Err(DevicePermissionError::ExportWithoutRedaction {
                    review_id: self.review_id.clone(),
                });
            }
            if !seen_actions.contains(&CaptureReviewActionClass::ExportRedactedCopy) {
                return Err(DevicePermissionError::ExportWithoutExportAction {
                    review_id: self.review_id.clone(),
                });
            }
        }
        if self.delete_available && !seen_actions.contains(&CaptureReviewActionClass::DeleteNow) {
            return Err(DevicePermissionError::DeleteWithoutDeleteAction {
                review_id: self.review_id.clone(),
            });
        }

        // Redaction / data-exit consistency.
        if !self.redaction_state.allows_data_exit(self.data_exit_boundary) {
            return Err(DevicePermissionError::RedactionDataExitMismatch {
                review_id: self.review_id.clone(),
                redaction: self.redaction_state,
                data_exit: self.data_exit_boundary,
            });
        }

        // Local-processing honesty on the review, too.
        if retention_is_provider_backed(self.retention_mode)
            && self.processing_locality == ProcessingLocalityCue::LocalOnDevice
        {
            return Err(DevicePermissionError::LocalProcessingClaimedWithProvider {
                record_id: self.review_id.clone(),
            });
        }
        if self.processing_locality == ProcessingLocalityCue::LocalOnDevice
            && self.data_exit_boundary != DataExitBoundary::NoPayloadLeavesProduct
        {
            return Err(DevicePermissionError::LocalProcessingLeaksPayload {
                record_id: self.review_id.clone(),
            });
        }
        Ok(())
    }
}

/// A bundled set of device-permission rows, mic-state pills, and capture/export
/// reviews, checked in as the canonical M5 source for device/capture-boundary
/// truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DevicePermissionSet {
    /// Schema version for the set shape.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable id for the set.
    pub set_id: String,
    /// Reviewer-facing label for the set.
    pub set_label: String,
    /// One device-permission row per device class.
    pub permission_rows: Vec<DevicePermissionRow>,
    /// The mic-state pills, one per pill state.
    pub mic_pills: Vec<MicStatePill>,
    /// The capture/export reviews.
    pub capture_reviews: Vec<CaptureExportReview>,
    /// Source contracts this set binds to by id.
    pub source_contract_refs: Vec<String>,
    /// Redaction-class token covering the export boundary.
    pub redaction_class_token: String,
    /// Opaque mint timestamp ref.
    pub minted_at: String,
    /// Frozen contract-doc ref.
    pub contract_doc_ref: String,
}

impl M5DevicePermissionSet {
    /// Validate the set: every record validates, every device class / pill state /
    /// capture class is covered, capture is never all-on, both local and
    /// provider-backed processing are represented, at least one high-impact pill
    /// proves the confirmation gate, and the source contracts are present.
    pub fn validate(&self) -> Result<(), DevicePermissionError> {
        if self.schema_version != M5_DEVICE_PERMISSION_SET_SCHEMA_VERSION {
            return Err(DevicePermissionError::WrongSetSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_DEVICE_PERMISSION_SET_RECORD_KIND {
            return Err(DevicePermissionError::WrongSetRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        if non_empty(&self.set_id).is_none()
            || non_empty(&self.set_label).is_none()
            || non_empty(&self.redaction_class_token).is_none()
            || non_empty(&self.minted_at).is_none()
        {
            return Err(DevicePermissionError::SetIdentityIncomplete);
        }
        if !ref_is_opaque(&self.minted_at) {
            return Err(DevicePermissionError::RawRefLeak {
                record_id: self.set_id.clone(),
                field: "minted_at",
            });
        }
        if self.contract_doc_ref != M5_DEVICE_PERMISSION_CONTRACT_DOC_REF {
            return Err(DevicePermissionError::WrongContractDocRef {
                record_id: self.set_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }

        // Every record validates and has a unique id.
        let mut ids: BTreeSet<&str> = BTreeSet::new();
        for row in &self.permission_rows {
            row.validate()?;
            if !ids.insert(row.row_id.as_str()) {
                return Err(DevicePermissionError::DuplicateRecordId {
                    record_id: row.row_id.clone(),
                });
            }
        }
        for pill in &self.mic_pills {
            pill.validate()?;
            if !ids.insert(pill.pill_id.as_str()) {
                return Err(DevicePermissionError::DuplicateRecordId {
                    record_id: pill.pill_id.clone(),
                });
            }
        }
        for review in &self.capture_reviews {
            review.validate()?;
            if !ids.insert(review.review_id.as_str()) {
                return Err(DevicePermissionError::DuplicateRecordId {
                    record_id: review.review_id.clone(),
                });
            }
        }

        // Every device class is named exactly once.
        for device in DeviceClass::ALL {
            let count = self
                .permission_rows
                .iter()
                .filter(|r| r.device_class == device)
                .count();
            if count != 1 {
                return Err(DevicePermissionError::DeviceClassNotNamedOnce { device, count });
            }
        }

        // Every pill state is represented.
        for state in MicPillState::ALL {
            if !self.mic_pills.iter().any(|p| p.pill_state == state) {
                return Err(DevicePermissionError::PillStateMissing { state });
            }
        }

        // Every capture class is covered by some review.
        for class in CaptureClass::ALL {
            let covered = self
                .capture_reviews
                .iter()
                .any(|r| r.included_capture_classes.contains(&class));
            if !covered {
                return Err(DevicePermissionError::CaptureClassUncovered { class });
            }
        }

        // Guardrail: capture is not always-on — at least one permission row rests
        // in a non-capturing default.
        if self.permission_rows.iter().all(|r| r.capture_active) {
            return Err(DevicePermissionError::CaptureAlwaysOn);
        }

        // Both local and provider-backed processing must be represented so users
        // can tell them apart.
        if !self
            .permission_rows
            .iter()
            .any(|r| r.processing_locality == ProcessingLocalityCue::LocalOnDevice)
        {
            return Err(DevicePermissionError::LocalProcessingUnrepresented);
        }
        if !self
            .permission_rows
            .iter()
            .any(|r| r.processing_locality == ProcessingLocalityCue::HostedRemoteDisclosed)
        {
            return Err(DevicePermissionError::ProviderProcessingUnrepresented);
        }

        // At least one high-impact pill proves the confirmation/preview gate.
        if !self
            .mic_pills
            .iter()
            .any(|p| p.command_capability_scope.is_high_impact())
        {
            return Err(DevicePermissionError::NoHighImpactGate);
        }

        // At least one capture review proves privacy-bounded delete + export.
        if !self.capture_reviews.iter().any(|r| r.delete_available) {
            return Err(DevicePermissionError::NoDeletableReview);
        }

        // Source contracts bound by id.
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        for required in [
            M5_DEVICE_PERMISSION_ROW_SCHEMA_REF,
            M5_MIC_STATE_PILL_SCHEMA_REF,
            M5_DEVICE_PERMISSION_CONTRACT_DOC_REF,
            M5_DEVICE_PERMISSION_VOICE_MATRIX_REF,
        ] {
            if !refs.contains(required) {
                return Err(DevicePermissionError::MissingSourceContracts);
            }
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("device permission set serializes"),
        ) {
            return Err(DevicePermissionError::RawMaterialInExport);
        }
        Ok(())
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only set fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("device permission set serializes")
    }

    /// Deterministic, machine-readable CSV: one row per record across all three
    /// families, naming its kind, identity, primary state, processing locality,
    /// retention/exit, and the active/gate flag.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("record_kind,identity,primary_state,processing_locality,retention_or_exit,active_or_gate\n");
        for row in &self.permission_rows {
            out.push_str(&format!(
                "device_permission_row,{},{},{},{},{}\n",
                row.row_id,
                row.permission_state.as_str(),
                row.processing_locality.as_str(),
                row.retention_mode.as_str(),
                row.capture_active,
            ));
        }
        for pill in &self.mic_pills {
            out.push_str(&format!(
                "mic_state_pill,{},{},{},{},{}\n",
                pill.pill_id,
                pill.pill_state.as_str(),
                pill.processing_locality.as_str(),
                pill.correction_posture.as_str(),
                pill.preview_required_before_commit,
            ));
        }
        for review in &self.capture_reviews {
            out.push_str(&format!(
                "capture_export_review,{},{},{},{},{}\n",
                review.review_id,
                review.redaction_state.as_str(),
                review.processing_locality.as_str(),
                review.data_exit_boundary.as_str(),
                review.export_available,
            ));
        }
        out
    }

    /// Deterministic Markdown governance summary for support, docs, or review
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 device-permission & capture review\n\n");
        out.push_str(&format!("Set: `{}`\n\n", self.set_id));

        out.push_str("## Device-permission rows\n\n");
        out.push_str("| Device | State | Actor | Processing | Retention | Capture active? |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- |\n");
        for row in &self.permission_rows {
            out.push_str(&format!(
                "| {} | {} | {} | `{}` | `{}` | {} |\n",
                row.device_class.label(),
                row.permission_state.label(),
                row.controlling_actor.label(),
                row.processing_locality.as_str(),
                row.retention_mode.as_str(),
                row.capture_active,
            ));
        }

        out.push_str("\n## Mic-state pills\n\n");
        out.push_str("| Pill | State | Processing | Correction | Scope | Preview required? |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- |\n");
        for pill in &self.mic_pills {
            out.push_str(&format!(
                "| `{}` | {} | `{}` | `{}` | `{}` | {} |\n",
                pill.pill_id,
                pill.pill_state.label(),
                pill.processing_locality.as_str(),
                pill.correction_posture.as_str(),
                pill.command_capability_scope.as_str(),
                pill.preview_required_before_commit,
            ));
        }

        out.push_str("\n## Capture/export reviews\n\n");
        out.push_str("| Review | Redaction | Data exit | Delete? | Export? |\n");
        out.push_str("| --- | --- | --- | --- | --- |\n");
        for review in &self.capture_reviews {
            out.push_str(&format!(
                "| `{}` | {} | `{}` | {} | {} |\n",
                review.review_id,
                review.redaction_state.label(),
                review.data_exit_boundary.as_str(),
                review.delete_available,
                review.export_available,
            ));
        }
        out.push('\n');
        out.push_str("Capture is never always-on by default, local processing is never claimed when a provider is in the path, ");
        out.push_str("and high-impact spoken commands ride the same preview/confirmation gate with transcript correction required before commit.\n");
        out
    }
}

/// True when a ref is an opaque token rather than a raw URL, email, or blank.
fn ref_is_opaque(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty()
        && trimmed == value
        && !trimmed.contains("://")
        && !trimmed.contains('@')
        && !trimmed.contains(char::is_whitespace)
}

fn non_empty(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Closed validation-error vocabulary for the device-permission contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DevicePermissionError {
    WrongRowSchemaVersion { row_id: String, actual: u32 },
    WrongRowRecordKind { row_id: String, actual: String },
    MalformedRowId { row_id: String },
    MalformedPillId { pill_id: String },
    MalformedReviewId { review_id: String },
    CaptureActiveWithoutGrant { row_id: String, state: PermissionState },
    InUseButNotCapturing { row_id: String },
    LocalProcessingClaimedWithProvider { record_id: String },
    LocalProcessingLeaksPayload { record_id: String },
    NoActions { row_id: String },
    DuplicateAction { row_id: String, action: PermissionActionClass },
    MissingSystemSettingsAction { row_id: String },
    MissingRevokeAction { row_id: String },
    MissingRequestAction { row_id: String },
    CapturingWithoutIndicator { pill_id: String },
    CapturingWithoutProcessing { pill_id: String },
    OffWithoutReason { pill_id: String },
    OffWithActiveProcessing { pill_id: String },
    PolicyBlockedWrongReason { pill_id: String },
    ReasonOnAvailablePill { pill_id: String },
    HighImpactWithoutConfirmation { pill_id: String },
    HighImpactWithoutCorrection { pill_id: String },
    HighImpactWithoutPreview { pill_id: String },
    ConfirmationWithoutGate { pill_id: String },
    ConfidenceWithoutTranscript { pill_id: String },
    EmptyCaptureClasses { review_id: String },
    DuplicateCaptureClass { review_id: String, class: CaptureClass },
    NoReviewActions { review_id: String },
    DuplicateReviewAction { review_id: String, action: CaptureReviewActionClass },
    ExportWithoutRedaction { review_id: String },
    ExportWithoutExportAction { review_id: String },
    DeleteWithoutDeleteAction { review_id: String },
    RedactionDataExitMismatch {
        review_id: String,
        redaction: CaptureRedactionState,
        data_exit: DataExitBoundary,
    },
    WrongSetSchemaVersion { actual: u32 },
    WrongSetRecordKind { actual: String },
    SetIdentityIncomplete,
    DuplicateRecordId { record_id: String },
    DeviceClassNotNamedOnce { device: DeviceClass, count: usize },
    PillStateMissing { state: MicPillState },
    CaptureClassUncovered { class: CaptureClass },
    CaptureAlwaysOn,
    LocalProcessingUnrepresented,
    ProviderProcessingUnrepresented,
    NoHighImpactGate,
    NoDeletableReview,
    MissingSourceContracts,
    RawMaterialInExport,
    WrongContractDocRef { record_id: String, actual: String },
    EmptyRequiredField { record_id: String, field: &'static str },
    RawRefLeak { record_id: String, field: &'static str },
}

impl fmt::Display for DevicePermissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRowSchemaVersion { row_id, actual } => write!(
                f,
                "row {row_id} has unsupported device_permission_row_schema_version {actual}"
            ),
            Self::WrongRowRecordKind { row_id, actual } => {
                write!(f, "row {row_id} has unsupported record kind {actual}")
            }
            Self::MalformedRowId { row_id } => {
                write!(f, "row id {row_id} must start with device_permission_row:")
            }
            Self::MalformedPillId { pill_id } => {
                write!(f, "pill id {pill_id} must start with mic_state_pill:")
            }
            Self::MalformedReviewId { review_id } => {
                write!(f, "review id {review_id} must start with capture_export_review:")
            }
            Self::CaptureActiveWithoutGrant { row_id, state } => write!(
                f,
                "row {row_id} reports capture active while state is {}; capture is never always-on",
                state.as_str()
            ),
            Self::InUseButNotCapturing { row_id } => {
                write!(f, "row {row_id} is granted-in-use but reports capture inactive")
            }
            Self::LocalProcessingClaimedWithProvider { record_id } => write!(
                f,
                "record {record_id} claims local processing while a provider is in the path"
            ),
            Self::LocalProcessingLeaksPayload { record_id } => write!(
                f,
                "record {record_id} claims local processing but its data exit lets payload leave"
            ),
            Self::NoActions { row_id } => write!(f, "row {row_id} offers no reversible actions"),
            Self::DuplicateAction { row_id, action } => {
                write!(f, "row {row_id} repeats action {}", action.as_str())
            }
            Self::MissingSystemSettingsAction { row_id } => {
                write!(f, "row {row_id} must offer open_system_settings")
            }
            Self::MissingRevokeAction { row_id } => {
                write!(f, "row {row_id} is granted but offers no revoke_in_app")
            }
            Self::MissingRequestAction { row_id } => {
                write!(f, "row {row_id} is requestable but offers no request_access")
            }
            Self::CapturingWithoutIndicator { pill_id } => {
                write!(f, "pill {pill_id} is capturing without a visible indicator")
            }
            Self::CapturingWithoutProcessing { pill_id } => {
                write!(f, "pill {pill_id} is capturing but reports processing unavailable")
            }
            Self::OffWithoutReason { pill_id } => {
                write!(f, "pill {pill_id} is off but names no unavailable reason")
            }
            Self::OffWithActiveProcessing { pill_id } => {
                write!(f, "pill {pill_id} is off but still reports active processing")
            }
            Self::PolicyBlockedWrongReason { pill_id } => {
                write!(f, "pill {pill_id} is policy-blocked but its reason is not policy")
            }
            Self::ReasonOnAvailablePill { pill_id } => {
                write!(f, "pill {pill_id} names an unavailable reason while available")
            }
            Self::HighImpactWithoutConfirmation { pill_id } => write!(
                f,
                "pill {pill_id} routes a high-impact command without needs_confirmation"
            ),
            Self::HighImpactWithoutCorrection { pill_id } => write!(
                f,
                "pill {pill_id} routes a high-impact command without required transcript correction"
            ),
            Self::HighImpactWithoutPreview { pill_id } => write!(
                f,
                "pill {pill_id} routes a high-impact command without a required preview"
            ),
            Self::ConfirmationWithoutGate { pill_id } => write!(
                f,
                "pill {pill_id} is needs-confirmation without a preview + required-correction gate"
            ),
            Self::ConfidenceWithoutTranscript { pill_id } => {
                write!(f, "pill {pill_id} shows a confidence cue with no transcript segment")
            }
            Self::EmptyCaptureClasses { review_id } => {
                write!(f, "review {review_id} names no capture classes")
            }
            Self::DuplicateCaptureClass { review_id, class } => {
                write!(f, "review {review_id} repeats capture class {}", class.as_str())
            }
            Self::NoReviewActions { review_id } => {
                write!(f, "review {review_id} offers no reversible actions")
            }
            Self::DuplicateReviewAction { review_id, action } => {
                write!(f, "review {review_id} repeats action {}", action.as_str())
            }
            Self::ExportWithoutRedaction { review_id } => write!(
                f,
                "review {review_id} advertises export but its redaction state forbids it"
            ),
            Self::ExportWithoutExportAction { review_id } => write!(
                f,
                "review {review_id} advertises export but offers no export_redacted_copy action"
            ),
            Self::DeleteWithoutDeleteAction { review_id } => write!(
                f,
                "review {review_id} advertises delete but offers no delete_now action"
            ),
            Self::RedactionDataExitMismatch {
                review_id,
                redaction,
                data_exit,
            } => write!(
                f,
                "review {review_id} redaction {} cannot use data exit {}",
                redaction.as_str(),
                data_exit.as_str()
            ),
            Self::WrongSetSchemaVersion { actual } => {
                write!(f, "set has unsupported schema_version {actual}")
            }
            Self::WrongSetRecordKind { actual } => {
                write!(f, "set has unsupported record kind {actual}")
            }
            Self::SetIdentityIncomplete => write!(f, "set is missing required identity fields"),
            Self::DuplicateRecordId { record_id } => {
                write!(f, "set has duplicate record id {record_id}")
            }
            Self::DeviceClassNotNamedOnce { device, count } => write!(
                f,
                "set names device class {} {count} times; expected exactly once",
                device.as_str()
            ),
            Self::PillStateMissing { state } => {
                write!(f, "set is missing mic pill state {}", state.as_str())
            }
            Self::CaptureClassUncovered { class } => {
                write!(f, "set never reviews capture class {}", class.as_str())
            }
            Self::CaptureAlwaysOn => {
                write!(f, "set makes capture always-on; at least one row must rest inactive")
            }
            Self::LocalProcessingUnrepresented => {
                write!(f, "set never represents local on-device processing")
            }
            Self::ProviderProcessingUnrepresented => {
                write!(f, "set never represents provider-backed processing")
            }
            Self::NoHighImpactGate => {
                write!(f, "set has no high-impact pill proving the confirmation gate")
            }
            Self::NoDeletableReview => {
                write!(f, "set has no capture review offering delete")
            }
            Self::MissingSourceContracts => {
                write!(f, "set is missing a required source contract ref")
            }
            Self::RawMaterialInExport => {
                write!(f, "set export carries forbidden raw material")
            }
            Self::WrongContractDocRef { record_id, actual } => {
                write!(f, "record {record_id} cites wrong contract doc {actual}")
            }
            Self::EmptyRequiredField { record_id, field } => {
                write!(f, "record {record_id} is missing required field {field}")
            }
            Self::RawRefLeak { record_id, field } => write!(
                f,
                "record {record_id} field {field} contains a raw URL, email, or whitespace; opaque refs only"
            ),
        }
    }
}

impl Error for DevicePermissionError {}

/// Reads and validates the checked-in stable device-permission set.
pub fn current_stable_m5_device_permission_set() -> Result<M5DevicePermissionSet, Box<dyn Error>> {
    let set: M5DevicePermissionSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/help/m5-device-permission-proof/permission_set.json"
    )))?;
    set.validate()?;
    Ok(set)
}
