//! Frozen M5 voice-mode provider / transcript-retention / command-parity
//! qualification matrix for every claimed voice-capable surface.
//!
//! Voice stays an explicit, privacy-bounded input mode in Aureline: command
//! mode and dictation mode are always separate and visible, push-to-talk (or an
//! equivalent explicit activation) is the default, provider locality and
//! transcript retention are inspectable, dictated edits ride the same edit
//! model and grouped undo/history as keyboard input, and no voice path widens
//! authority, bypasses preview/confirmation, or creates hidden transcript/audio
//! retention. The M3 preview ([`crate::voice`]) and the M4 surface
//! qualification already model the per-surface session, mode-state, and command
//! resolution objects. This module turns the remaining implicit promise — that
//! every *claimed* voice profile rests on a disclosed provider, a bounded
//! transcript-retention posture, and full command parity — into one
//! machine-readable, verification-bound matrix.
//!
//! * a [`VoiceClaimedProfileRow`] ties a durable claimed voice profile (keyed by
//!   a [`VoiceSurfaceKind`], a [`VoiceProfileOriginClass`], and a non-display
//!   fingerprint) to a versioned [`VoiceProviderDescriptor`] (provider class,
//!   processing locality, transport class, capability disclosure, and a
//!   [`VoiceTranscriptRetentionPosture`]), a versioned [`VoiceSessionState`]
//!   (mode, activation, mic indicator, policy state), a [`VoiceCommandParity`]
//!   block, and a claimed and effective [`VoiceQualificationGrade`];
//! * each row is **verification-bound, not asserted**: its [`VoiceVerification`]
//!   names a [`VoiceProofCurrency`] and, unless the proof is missing, a
//!   reopenable proof ref keyed by a non-display fingerprint, so command,
//!   accessibility, diagnostics, support-export, and release surfaces can reopen
//!   the same evidence object that backs the qualification claim;
//! * the row **auto-downgrades**: [`VoiceClaimedProfileRow::needs_downgrade`] is
//!   true whenever a *claimed* profile conflates or blocks its mode, defaults to
//!   continuous/wake activation without an opt-in, leaves hosted/enterprise
//!   processing undisclosed, ships incomplete command parity, or carries stale,
//!   missing, or imported-on-local proof. A downgraded row must carry an
//!   effective grade strictly below its claim, a recorded
//!   [`VoiceQualificationDowngradeTrigger`], and a precise label — never a
//!   generic non-answer. Unclaimed (Labs/unadvertised) profiles make no claim to
//!   downgrade from and are kept clearly separate from claimed scope.
//!
//! [`M5VoiceQualificationMatrixPacket::validate`] additionally refuses any
//! packet that lets a voice profile become a dead end (no keyboard fallback),
//! retain raw transcripts by default, enable background listening by default,
//! claim opt-in capabilities without opt-in guards, or let a provider-linked
//! profile read as a locally verified one.
//!
//! Raw audio bytes, raw transcript text, raw provider payloads, private paths,
//! and credentials never cross this boundary; the packet carries only typed
//! class tokens, booleans, opaque ids, fingerprint digests, and redaction-aware
//! reviewable labels.
//!
//! The boundary schemas are
//! [`schemas/voice/m5-voice-qualification-matrix.schema.json`](../../../../schemas/voice/m5-voice-qualification-matrix.schema.json),
//! [`schemas/voice/voice-provider-descriptor.schema.json`](../../../../schemas/voice/voice-provider-descriptor.schema.json),
//! and
//! [`schemas/voice/voice-session.schema.json`](../../../../schemas/voice/voice-session.schema.json).
//! The truth doc is
//! [`docs/ux/voice-mode-and-privacy-truth.md`](../../../../docs/ux/voice-mode-and-privacy-truth.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::voice::{
    BackgroundListeningState, CommandPreviewClass, MicIndicatorClass, ProcessingLocalityCue,
    RetentionMode, TranscriptCorrectionPosture, VoiceActivationClass, VoiceCapabilityScope,
    VoiceClaimPosture, VoiceModeClass, VoiceUnavailableReason,
};

/// Stable record-kind tag carried by [`M5VoiceQualificationMatrixPacket`].
pub const VOICE_QUALIFICATION_MATRIX_RECORD_KIND: &str =
    "freeze_m5_voice_mode_provider_transcript_retention_command_parity_matrix_packet";

/// Stable record-kind tag carried by a standalone [`VoiceProviderDescriptor`].
pub const VOICE_PROVIDER_DESCRIPTOR_RECORD_KIND: &str = "voice_provider_descriptor";

/// Stable record-kind tag carried by a standalone [`VoiceSessionState`].
pub const VOICE_SESSION_STATE_RECORD_KIND: &str = "voice_session_state";

/// Schema version shared by the matrix packet and its component objects.
pub const VOICE_QUALIFICATION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the matrix boundary schema.
pub const VOICE_QUALIFICATION_MATRIX_SCHEMA_REF: &str =
    "schemas/voice/m5-voice-qualification-matrix.schema.json";

/// Repo-relative path of the provider-descriptor boundary schema.
pub const VOICE_PROVIDER_DESCRIPTOR_SCHEMA_REF: &str =
    "schemas/voice/voice-provider-descriptor.schema.json";

/// Repo-relative path of the voice-session boundary schema.
pub const VOICE_SESSION_STATE_SCHEMA_REF: &str = "schemas/voice/voice-session.schema.json";

/// Repo-relative path of the truth doc.
pub const VOICE_QUALIFICATION_MATRIX_DOC_REF: &str = "docs/ux/voice-mode-and-privacy-truth.md";

/// Repo-relative path of the checked support-export artifact.
pub const VOICE_QUALIFICATION_MATRIX_ARTIFACT_REF: &str =
    "artifacts/voice/m5-voice-qualification-matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const VOICE_QUALIFICATION_MATRIX_SUMMARY_REF: &str =
    "artifacts/voice/m5-voice-qualification-matrix.md";

/// Repo-relative path of the M3 cross-surface voice/dictation contract this lane
/// extends.
pub const VOICE_AND_DICTATION_CONTRACT_REF: &str = "docs/ux/voice_and_dictation_contract.md";

/// Repo-relative path of the M4 voice/dictation surface qualification packet this
/// lane builds on.
pub const VOICE_M4_SURFACE_QUALIFICATION_REF: &str =
    "artifacts/release/m4/voice-and-dictation-surface-qualification.json";

/// Kind of claimed voice surface a profile row covers. Mirrors the M4 voice
/// surface taxonomy so command, accessibility, and release surfaces read one set
/// of surface ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceSurfaceKind {
    /// Spoken command overlay routed through the canonical command graph.
    CommandOverlay,
    /// Dictation input that inserts text on the shared editor undo stack.
    DictationInput,
    /// Transcript correction / review surface before a privileged commit.
    TranscriptCorrection,
    /// Provider / privacy settings surface that discloses locality and retention.
    ProviderPrivacySettings,
    /// Unavailable / fallback surface that always offers a keyboard path.
    UnavailableFallback,
    /// High-impact action review surface for destructive or publishing voice
    /// actions.
    HighImpactActionReview,
}

impl VoiceSurfaceKind {
    /// Every claimed voice surface kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CommandOverlay,
        Self::DictationInput,
        Self::TranscriptCorrection,
        Self::ProviderPrivacySettings,
        Self::UnavailableFallback,
        Self::HighImpactActionReview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandOverlay => "command_overlay",
            Self::DictationInput => "dictation_input",
            Self::TranscriptCorrection => "transcript_correction",
            Self::ProviderPrivacySettings => "provider_privacy_settings",
            Self::UnavailableFallback => "unavailable_fallback",
            Self::HighImpactActionReview => "high_impact_action_review",
        }
    }
}

/// Origin of a claimed voice profile. A provider-linked or imported profile must
/// never read as a locally verified first-party profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceProfileOriginClass {
    /// A first-party, locally verified voice profile.
    FirstPartyLocalProfile,
    /// An enterprise-managed voice profile governed by org policy.
    EnterpriseManagedProfile,
    /// A provider-linked profile whose qualification is provider-backed.
    ProviderLinkedProfile,
    /// An imported, read-only profile record.
    ImportedReadOnlyProfile,
}

impl VoiceProfileOriginClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyLocalProfile => "first_party_local_profile",
            Self::EnterpriseManagedProfile => "enterprise_managed_profile",
            Self::ProviderLinkedProfile => "provider_linked_profile",
            Self::ImportedReadOnlyProfile => "imported_read_only_profile",
        }
    }

    /// Whether qualification for this origin is provider-backed / imported rather
    /// than locally verified, so a current claim rests on imported proof.
    pub const fn is_provider_or_imported(self) -> bool {
        matches!(
            self,
            Self::ProviderLinkedProfile | Self::ImportedReadOnlyProfile
        )
    }
}

/// Class of speech provider backing a voice profile. The class is always
/// disclosed before capture; a disabled provider keeps the keyboard path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceProviderClass {
    /// On-device, in-process speech engine (preferred where available).
    OnDeviceLocal,
    /// Approved remote ASR/TTS, opt-in and disclosed.
    ApprovedRemoteDisclosed,
    /// Enterprise-managed relay/provider, policy-controlled and audited.
    EnterpriseRelayManaged,
    /// A mocked provider used only for conformance fixtures.
    MockedTestProvider,
    /// Provider disabled; voice is unavailable and falls back to keyboard.
    ProviderDisabled,
}

impl VoiceProviderClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnDeviceLocal => "on_device_local",
            Self::ApprovedRemoteDisclosed => "approved_remote_disclosed",
            Self::EnterpriseRelayManaged => "enterprise_relay_managed",
            Self::MockedTestProvider => "mocked_test_provider",
            Self::ProviderDisabled => "provider_disabled",
        }
    }

    /// Whether the provider processes audio off-device (remote or enterprise
    /// relay), so transport disclosure and audit are required.
    pub const fn is_hosted(self) -> bool {
        matches!(
            self,
            Self::ApprovedRemoteDisclosed | Self::EnterpriseRelayManaged
        )
    }

    /// Whether the provider is disabled.
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::ProviderDisabled)
    }
}

/// How audio/transcripts move to the provider. A hosted provider must declare a
/// disclosed, policy-bounded or explicit-opt-in endpoint; a blocked transport
/// keeps capture local or denied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceTransportClass {
    /// Audio stays in-process; no handoff.
    LocalInProcessOnly,
    /// Audio handed to a policy-bounded, disclosed org endpoint.
    PolicyBoundedDisclosedEndpoint,
    /// Audio handed to a disclosed endpoint only after an explicit opt-in.
    ExplicitOptInDisclosedEndpoint,
    /// Transport blocked; no audio leaves the device.
    TransportBlocked,
}

impl VoiceTransportClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalInProcessOnly => "local_in_process_only",
            Self::PolicyBoundedDisclosedEndpoint => "policy_bounded_disclosed_endpoint",
            Self::ExplicitOptInDisclosedEndpoint => "explicit_opt_in_disclosed_endpoint",
            Self::TransportBlocked => "transport_blocked",
        }
    }

    /// Whether this transport discloses a remote handoff (policy-bounded or
    /// explicit opt-in), as a hosted provider requires.
    pub const fn discloses_remote_handoff(self) -> bool {
        matches!(
            self,
            Self::PolicyBoundedDisclosedEndpoint | Self::ExplicitOptInDisclosedEndpoint
        )
    }
}

/// Audio-retention posture of a provider. Audio is never retained by default;
/// any retention is local, ephemeral, or per an enterprise contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioRetentionClass {
    /// No audio retained at all.
    NoAudioRetained,
    /// Ephemeral audio buffered local-only during capture, then dropped.
    EphemeralAudioLocalOnly,
    /// Audio retained in a bounded local window (e.g. for replay/correction).
    BoundedAudioLocalWindow,
    /// Audio retained by the provider per an enterprise contract.
    AudioRetainedProviderPerContract,
    /// Audio capture blocked entirely.
    AudioCaptureBlocked,
}

impl AudioRetentionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAudioRetained => "no_audio_retained",
            Self::EphemeralAudioLocalOnly => "ephemeral_audio_local_only",
            Self::BoundedAudioLocalWindow => "bounded_audio_local_window",
            Self::AudioRetainedProviderPerContract => "audio_retained_provider_per_contract",
            Self::AudioCaptureBlocked => "audio_capture_blocked",
        }
    }

    /// Whether audio leaves the device by default (provider-retained), which a
    /// generally-claimed profile must not do without an enterprise contract.
    pub const fn is_provider_retained(self) -> bool {
        matches!(self, Self::AudioRetainedProviderPerContract)
    }
}

/// Transcript-export posture. Transcript export is explicit, bounded, and
/// user-visible; raw transcripts never enter support bundles or telemetry by
/// default.
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
    /// Stable token recorded in the packet.
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

/// Policy state of a voice session. Drives whether voice is user-controlled,
/// enterprise-managed, or policy-blocked.
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
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserControlled => "user_controlled",
            Self::EnterprisePolicyManaged => "enterprise_policy_managed",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Qualification grade a claimed voice profile holds. Higher [`Self::rank`] is a
/// stronger claim, so a downgraded row must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceQualificationGrade {
    /// A fully qualified claimed voice profile (disclosed, bounded, parity-complete).
    QualifiedClaimedProfile,
    /// A qualified but deliberately narrowed profile (e.g. hosted/enterprise,
    /// preview-class).
    QualifiedNarrowedProfile,
    /// An unclaimed Labs/unadvertised profile, kept out of public scope.
    LabsUnadvertisedProfile,
    /// A profile whose qualification was withdrawn.
    QualificationWithdrawn,
    /// Qualification does not apply to this row.
    NotApplicable,
}

impl VoiceQualificationGrade {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualifiedClaimedProfile => "qualified_claimed_profile",
            Self::QualifiedNarrowedProfile => "qualified_narrowed_profile",
            Self::LabsUnadvertisedProfile => "labs_unadvertised_profile",
            Self::QualificationWithdrawn => "qualification_withdrawn",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this grade carries a public, qualified claim.
    pub const fn is_qualified_claim(self) -> bool {
        matches!(
            self,
            Self::QualifiedClaimedProfile | Self::QualifiedNarrowedProfile
        )
    }

    /// Ordinal rank; higher is a stronger claim, so a downgrade must move
    /// strictly lower.
    pub const fn rank(self) -> u8 {
        match self {
            Self::NotApplicable => 0,
            Self::QualificationWithdrawn => 1,
            Self::LabsUnadvertisedProfile => 2,
            Self::QualifiedNarrowedProfile => 3,
            Self::QualifiedClaimedProfile => 4,
        }
    }
}

/// Currency of the proof backing a row's verification. Only a current, reopenable
/// proof backs a claim; a stale, missing, review-pending, or imported-on-local
/// proof auto-downgrades the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceProofCurrency {
    /// A fresh local proof verified inside its freshness window.
    VerifiedCurrent,
    /// A cached local proof still inside its freshness window.
    CachedWithinWindow,
    /// A current proof imported / provider-backed and read-only locally.
    ImportedCurrent,
    /// A proof that exists but has aged outside its freshness window.
    StaleExpired,
    /// A proof that still requires review and fails closed.
    RequiresReview,
    /// No proof object exists for this row.
    MissingProof,
}

impl VoiceProofCurrency {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCurrent => "verified_current",
            Self::CachedWithinWindow => "cached_within_window",
            Self::ImportedCurrent => "imported_current",
            Self::StaleExpired => "stale_expired",
            Self::RequiresReview => "requires_review",
            Self::MissingProof => "missing_proof",
        }
    }

    /// Whether this is a current, locally verified or cached proof.
    pub const fn is_current_local(self) -> bool {
        matches!(self, Self::VerifiedCurrent | Self::CachedWithinWindow)
    }

    /// Whether this is a current imported / provider-backed proof.
    pub const fn is_imported_current(self) -> bool {
        matches!(self, Self::ImportedCurrent)
    }

    /// Whether this currency carries no proof object.
    pub const fn is_absent(self) -> bool {
        matches!(self, Self::MissingProof)
    }
}

/// Reason a claimed profile auto-downgraded below its claim. The chrome quotes
/// the trigger verbatim instead of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceQualificationDowngradeTrigger {
    /// The session mode could not be kept explicitly command-vs-dictation.
    ModeSeparationUnverified,
    /// A claimed profile defaulted to continuous/wake activation without opt-in.
    PushToTalkDefaultMissing,
    /// Hosted/enterprise processing or retention was not disclosed.
    ProviderLocalityUndisclosed,
    /// Transcript retention was not bounded for a generally-claimed profile.
    TranscriptRetentionUnbounded,
    /// Command parity (stable ids, preview, approval, undo, audit) was incomplete.
    CommandParityIncomplete,
    /// The keyboard fallback path was not complete.
    KeyboardFallbackMissing,
    /// The provider became unavailable or capture was blocked.
    ProviderUnavailableDowngraded,
    /// The verification proof aged outside its freshness window.
    StaleVerificationProof,
    /// Imported / provider proof stood in for a local-profile claim.
    ImportedProofOnLocalProfile,
}

impl VoiceQualificationDowngradeTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModeSeparationUnverified => "mode_separation_unverified",
            Self::PushToTalkDefaultMissing => "push_to_talk_default_missing",
            Self::ProviderLocalityUndisclosed => "provider_locality_undisclosed",
            Self::TranscriptRetentionUnbounded => "transcript_retention_unbounded",
            Self::CommandParityIncomplete => "command_parity_incomplete",
            Self::KeyboardFallbackMissing => "keyboard_fallback_missing",
            Self::ProviderUnavailableDowngraded => "provider_unavailable_downgraded",
            Self::StaleVerificationProof => "stale_verification_proof",
            Self::ImportedProofOnLocalProfile => "imported_proof_on_local_profile",
        }
    }
}

/// Capability disclosure a provider advertises. Opt-in guards are invariant:
/// continuous listening and wake-word require an explicit opt-in, and background
/// listening is off by default.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceProviderCapabilityDisclosure {
    /// Whether the provider supports command mode.
    pub supports_command_mode: bool,
    /// Whether the provider supports dictation mode.
    pub supports_dictation_mode: bool,
    /// Whether the provider supports a transcript correction/review pass.
    pub supports_correction_review: bool,
    /// Whether continuous listening requires an explicit opt-in (must be true).
    pub continuous_listening_requires_opt_in: bool,
    /// Whether wake-word activation requires an explicit opt-in (must be true).
    pub wake_word_requires_opt_in: bool,
    /// Whether background listening is off by default (must be true).
    pub background_listening_default_off: bool,
}

impl VoiceProviderCapabilityDisclosure {
    /// Whether the opt-in guards hold: continuous listening and wake-word are
    /// opt-in and background listening is off by default.
    pub const fn opt_in_guards_hold(&self) -> bool {
        self.continuous_listening_requires_opt_in
            && self.wake_word_requires_opt_in
            && self.background_listening_default_off
    }

    /// Whether the provider keeps command mode and dictation mode separable.
    pub const fn modes_separable(&self) -> bool {
        self.supports_command_mode && self.supports_dictation_mode
    }
}

/// Transcript-retention posture disclosed by a provider. The posture pins the
/// retention mode, the audio-retention class, and the export posture so the
/// privacy row, diagnostics, and support export read one bounded contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceTranscriptRetentionPosture {
    /// Canonical retention mode (mirrors [`RetentionMode`]).
    pub retention_mode: RetentionMode,
    /// Audio-retention class.
    pub audio_retention: AudioRetentionClass,
    /// Transcript-export posture.
    pub transcript_export: TranscriptExportPosture,
    /// Whether a bounded correction buffer holds recent transcript locally.
    pub bounded_correction_buffer: bool,
    /// Whether raw transcripts are excluded from support/telemetry by default
    /// (must be true).
    pub raw_transcripts_excluded_by_default: bool,
    /// Whether transcripts are redacted before any support export.
    pub redaction_before_support_export: bool,
}

impl VoiceTranscriptRetentionPosture {
    /// Whether raw transcripts stay out of support/telemetry by default.
    pub const fn raw_excluded(&self) -> bool {
        self.raw_transcripts_excluded_by_default
    }

    /// Whether the retention posture is internally consistent: raw transcripts
    /// are excluded by default, and a support-bundle retention mode carries
    /// redaction.
    pub fn posture_consistent(&self) -> bool {
        if !self.raw_transcripts_excluded_by_default {
            return false;
        }
        match self.retention_mode {
            RetentionMode::TranscriptRetainedRedactedInSupportBundle => {
                self.redaction_before_support_export
            }
            _ => true,
        }
    }

    /// Whether transcript retention is bounded for a generally-claimed profile:
    /// retention stays local/ephemeral/redacted and audio is not provider
    /// retained. Provider-per-contract retention is a deliberate narrowing, not a
    /// generally-claimed posture.
    pub fn bounded_for_general_claim(&self) -> bool {
        if self.audio_retention.is_provider_retained() {
            return false;
        }
        matches!(
            self.transcript_export,
            TranscriptExportPosture::NoTranscriptExport
                | TranscriptExportPosture::ExplicitUserExportRedacted
                | TranscriptExportPosture::MetadataOnlySupportExport
                | TranscriptExportPosture::ExportBlockedByPolicy
        ) && !matches!(
            self.retention_mode,
            RetentionMode::TranscriptRetainedProviderPerContract
        )
    }
}

/// Command-parity guarantees a voice profile must satisfy to be claimed: spoken
/// actions ride the same stable command ids, disabled reasons, preview/apply/
/// revert, approval, grouped undo, and audit/support lineage as the keyboard
/// path, and high-impact actions are reviewed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceCommandParity {
    /// Spoken actions resolve to stable command ids.
    pub stable_command_ids: bool,
    /// Disabled voice actions carry a typed reason.
    pub disabled_with_reason: bool,
    /// Voice actions ride the same preview/apply/revert path.
    pub preview_apply_revert: bool,
    /// Voice actions honor the same approval requirements.
    pub approval_requirements: bool,
    /// Dictated edits and voice actions group into the shared undo stack.
    pub undo_grouping: bool,
    /// Voice actions carry audit/support lineage.
    pub audit_support_lineage: bool,
    /// High-impact voice actions require transcript confirmation/review.
    pub high_impact_review: bool,
    /// The keyboard equivalent remains complete for every voice action.
    pub keyboard_fallback_parity: bool,
}

impl VoiceCommandParity {
    /// Whether every command-parity guarantee holds.
    pub const fn parity_complete(&self) -> bool {
        self.stable_command_ids
            && self.disabled_with_reason
            && self.preview_apply_revert
            && self.approval_requirements
            && self.undo_grouping
            && self.audit_support_lineage
            && self.high_impact_review
            && self.keyboard_fallback_parity
    }
}

/// A versioned, export-safe speech-provider descriptor: the canonical truth for a
/// voice provider's locality, transport, capability disclosure, and
/// transcript-retention posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceProviderDescriptor {
    /// Record kind; must equal [`VOICE_PROVIDER_DESCRIPTOR_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VOICE_QUALIFICATION_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Durable provider id referenced by sessions and rows.
    pub provider_id: String,
    /// Non-display fingerprint token. Must differ from
    /// [`provider_id`](VoiceProviderDescriptor::provider_id).
    pub provider_fingerprint_token: String,
    /// Provider class.
    pub provider_class: VoiceProviderClass,
    /// Processing-locality cue.
    pub processing_locality: ProcessingLocalityCue,
    /// Transport class.
    pub transport_class: VoiceTransportClass,
    /// Background-listening state.
    pub background_listening_state: BackgroundListeningState,
    /// Capability disclosure.
    pub capability_disclosure: VoiceProviderCapabilityDisclosure,
    /// Transcript-retention posture.
    pub retention_posture: VoiceTranscriptRetentionPosture,
    /// Export-safe data-class label.
    pub data_class_label: String,
    /// Whether the provider supports an audit trail (required for hosted/enterprise).
    pub audit_capable: bool,
    /// Whether a keyboard fallback remains complete when this provider fails.
    pub keyboard_fallback_available: bool,
    /// Optional fallback provider id (an alternate provider or local engine).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_provider_id: Option<String>,
}

impl VoiceProviderDescriptor {
    /// Whether the descriptor's fingerprint is a real non-display basis distinct
    /// from the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.provider_fingerprint_token.trim();
        !token.is_empty() && token != self.provider_id.trim()
    }

    /// Whether hosted/enterprise processing is disclosed: a hosted provider
    /// declares a remote-handoff transport, discloses hosted locality, and is
    /// audit-capable.
    pub fn locality_disclosed(&self) -> bool {
        if !self.provider_class.is_hosted() {
            return true;
        }
        self.transport_class.discloses_remote_handoff()
            && self.processing_locality == ProcessingLocalityCue::HostedRemoteDisclosed
            && self.audit_capable
    }

    /// Whether the descriptor satisfies the absolute provider invariants: a
    /// reopenable fingerprint, opt-in guards, a consistent retention posture, raw
    /// transcripts excluded by default, and a complete keyboard fallback.
    pub fn invariants_hold(&self) -> bool {
        self.fingerprint_independent_of_id()
            && self.capability_disclosure.opt_in_guards_hold()
            && self.retention_posture.posture_consistent()
            && self.retention_posture.raw_excluded()
            && self.keyboard_fallback_available
    }

    /// Whether the descriptor is structurally well-formed and carries its
    /// canonical record kind and schema version.
    pub fn is_well_formed(&self) -> bool {
        self.record_kind == VOICE_PROVIDER_DESCRIPTOR_RECORD_KIND
            && self.schema_version == VOICE_QUALIFICATION_MATRIX_SCHEMA_VERSION
            && !self.provider_id.trim().is_empty()
            && !self.data_class_label.trim().is_empty()
            && self.invariants_hold()
    }
}

/// A versioned, export-safe voice-session state object: the canonical truth for a
/// session's mode, activation, mic indicator, processing locality, provider
/// reference, and policy state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceSessionState {
    /// Record kind; must equal [`VOICE_SESSION_STATE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VOICE_QUALIFICATION_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Durable session id.
    pub session_id: String,
    /// Voice mode class (command vs dictation vs idle vs blocked).
    pub mode_class: VoiceModeClass,
    /// Activation class (push-to-talk default vs opt-in continuous/wake).
    pub activation_class: VoiceActivationClass,
    /// Persistent mic-indicator class.
    pub mic_indicator_class: MicIndicatorClass,
    /// Processing-locality cue shown during capture.
    pub processing_locality: ProcessingLocalityCue,
    /// Provider id this session binds to (matches a [`VoiceProviderDescriptor`]).
    pub provider_id: String,
    /// Background-listening state.
    pub background_listening_state: BackgroundListeningState,
    /// Command-preview posture for privileged actions.
    pub command_preview_class: CommandPreviewClass,
    /// Transcript-correction posture before a privileged commit.
    pub transcript_correction_posture: TranscriptCorrectionPosture,
    /// Policy state.
    pub policy_state: VoicePolicyState,
    /// Whether a keyboard fallback is available from this session.
    pub keyboard_fallback_available: bool,
    /// Whether mode and mic state are announced to assistive tech.
    pub accessibility_announced: bool,
}

impl VoiceSessionState {
    /// Whether command mode and dictation mode stay explicit: the mode is a
    /// definite command/dictation/idle state, not a policy/envelope block standing
    /// in for a live mode.
    pub const fn mode_is_explicit(&self) -> bool {
        matches!(
            self.mode_class,
            VoiceModeClass::IdleMicrophoneOff
                | VoiceModeClass::DictationModeActive
                | VoiceModeClass::CommandModeActive
                | VoiceModeClass::ContinuousListeningActiveUserOptedIn
        )
    }

    /// Whether activation is push-to-talk / explicit by default, or a
    /// continuous/wake activation backed by an explicit opt-in.
    pub fn activation_default_ok(&self) -> bool {
        if self.activation_class.is_explicit() {
            return true;
        }
        matches!(
            self.activation_class,
            VoiceActivationClass::WakePhraseContinuousUserOptedIn
        ) && self.background_listening_state == BackgroundListeningState::OnUserOptedIn
    }

    /// Whether background listening is consistent with activation: background-on
    /// requires an opt-in continuous/wake activation; off/blocked are always fine.
    pub fn background_consistent(&self) -> bool {
        match self.background_listening_state {
            BackgroundListeningState::OnUserOptedIn => matches!(
                self.activation_class,
                VoiceActivationClass::WakePhraseContinuousUserOptedIn
            ),
            BackgroundListeningState::OffDefault | BackgroundListeningState::BlockedByPolicy => {
                true
            }
        }
    }

    /// Whether the session is structurally well-formed and carries its canonical
    /// record kind and schema version.
    pub fn is_well_formed(&self) -> bool {
        self.record_kind == VOICE_SESSION_STATE_RECORD_KIND
            && self.schema_version == VOICE_QUALIFICATION_MATRIX_SCHEMA_VERSION
            && !self.session_id.trim().is_empty()
            && !self.provider_id.trim().is_empty()
            && self.background_consistent()
    }
}

/// A row's verification proof: the proof currency plus a reopenable evidence
/// object, so a qualification grade is backed by an object a reviewer can reopen
/// rather than an asserted claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceVerification {
    /// Currency of the proof backing this row.
    pub proof_currency: VoiceProofCurrency,
    /// Reopenable ref of the proof object. Present unless the proof is missing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_ref: Option<String>,
    /// Non-display fingerprint token of the proof object. Present iff `proof_ref`
    /// is present, and must differ from it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_fingerprint_token: Option<String>,
    /// Export-safe reviewable summary of the proof.
    pub summary: String,
}

impl VoiceVerification {
    /// Whether the proof object is reopenable: a present ref carries a distinct
    /// non-display fingerprint and a non-empty summary.
    pub fn proof_reopenable(&self) -> bool {
        match (&self.proof_ref, &self.proof_fingerprint_token) {
            (Some(reference), Some(fingerprint)) => {
                let reference = reference.trim();
                let fingerprint = fingerprint.trim();
                !reference.is_empty() && !fingerprint.is_empty() && fingerprint != reference
            }
            _ => false,
        }
    }

    /// Whether this verification is well-formed: a missing proof carries no ref,
    /// any other currency carries a reopenable proof, and the summary is present.
    pub fn is_well_formed(&self) -> bool {
        if self.summary.trim().is_empty() {
            return false;
        }
        if self.proof_currency.is_absent() {
            self.proof_ref.is_none() && self.proof_fingerprint_token.is_none()
        } else {
            self.proof_reopenable()
        }
    }

    /// Whether this verification backs a current claim for the given origin
    /// posture. A local profile needs locally verified or cached proof; a
    /// provider/imported profile needs current imported proof. Either way the
    /// proof must be reopenable.
    pub fn backs_claim(&self, provider_or_imported: bool) -> bool {
        if !self.proof_reopenable() {
            return false;
        }
        if provider_or_imported {
            self.proof_currency.is_imported_current()
        } else {
            self.proof_currency.is_current_local()
        }
    }
}

/// One claimed (or Labs/unadvertised) voice profile row in the qualification
/// matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceClaimedProfileRow {
    /// Stable profile id.
    pub profile_id: String,
    /// Kind of claimed voice surface.
    pub surface_kind: VoiceSurfaceKind,
    /// Origin class of the profile.
    pub origin_class: VoiceProfileOriginClass,
    /// Non-display fingerprint token. Must differ from
    /// [`profile_id`](VoiceClaimedProfileRow::profile_id).
    pub profile_fingerprint_token: String,
    /// Human-readable row label.
    pub label_summary: String,
    /// Claim posture (claimed beta/preview vs Labs/unadvertised).
    pub claim_posture: VoiceClaimPosture,
    /// Versioned provider descriptor backing this profile.
    pub provider: VoiceProviderDescriptor,
    /// Versioned session state for this profile.
    pub session: VoiceSessionState,
    /// Command-parity guarantees for this profile.
    pub command_parity: VoiceCommandParity,
    /// Reopenable verification proof backing the qualification claim.
    pub verification: VoiceVerification,
    /// Headline qualification grade publicly claimed for this row.
    pub claimed_grade: VoiceQualificationGrade,
    /// Effective grade after auto-downgrading; equals the claim when every axis is
    /// honest and the proof is current, and ranks strictly below it otherwise.
    pub effective_grade: VoiceQualificationGrade,
    /// Trigger that fired the downgrade, required when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<VoiceQualificationDowngradeTrigger>,
    /// Precise downgraded label, required when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgraded_label: Option<String>,
    /// Optional unavailable reason for fallback rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<VoiceUnavailableReason>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl VoiceClaimedProfileRow {
    /// Whether qualification for this row is provider-backed / imported.
    pub fn provider_or_imported(&self) -> bool {
        self.origin_class.is_provider_or_imported()
    }

    /// Whether the row carries a public claim (claimed beta/preview).
    pub fn is_claimed(&self) -> bool {
        self.claim_posture.is_claimed()
    }

    /// Whether the session keeps command vs dictation mode explicit.
    pub fn mode_separation_ok(&self) -> bool {
        self.session.mode_is_explicit()
    }

    /// Whether activation defaults to push-to-talk / explicit, or an opted-in
    /// continuous/wake activation.
    pub fn push_to_talk_default_ok(&self) -> bool {
        self.session.activation_default_ok()
    }

    /// Whether the provider discloses hosted/enterprise processing and retention.
    pub fn locality_disclosed_ok(&self) -> bool {
        self.provider.locality_disclosed()
    }

    /// Whether transcript retention is bounded for the row's claim level. A
    /// narrowed profile may carry provider-per-contract retention; a fully
    /// claimed profile may not.
    pub fn retention_bounded_for_claim(&self) -> bool {
        if self.claimed_grade == VoiceQualificationGrade::QualifiedClaimedProfile {
            self.provider.retention_posture.bounded_for_general_claim()
        } else {
            true
        }
    }

    /// Whether the provider is unavailable or capture is blocked.
    pub fn provider_unavailable(&self) -> bool {
        self.provider.provider_class.is_disabled()
            || self.session.processing_locality == ProcessingLocalityCue::ProcessingUnavailable
            || matches!(
                self.session.mode_class,
                VoiceModeClass::VoiceModeBlockedByPolicy
                    | VoiceModeClass::VoiceModeBlockedByEnvelope
            )
    }

    /// Whether the verification proof backs a current claim for this row's origin
    /// posture.
    pub fn verification_current(&self) -> bool {
        self.verification.backs_claim(self.provider_or_imported())
    }

    /// Whether a claimed row must downgrade below its claim because an axis is
    /// denied or the verification proof is not current. Unclaimed profiles make no
    /// claim to downgrade from.
    pub fn needs_downgrade(&self) -> bool {
        if !self.is_claimed() {
            return false;
        }
        !self.verification_current()
            || !self.mode_separation_ok()
            || !self.push_to_talk_default_ok()
            || !self.locality_disclosed_ok()
            || !self.retention_bounded_for_claim()
            || !self.command_parity.parity_complete()
            || self.provider_unavailable()
    }

    /// Whether the effective grade ranks strictly below the claim.
    pub fn properly_downgraded(&self) -> bool {
        self.effective_grade.rank() < self.claimed_grade.rank()
    }

    /// Whether the effective grade and downgrade evidence are consistent.
    ///
    /// When the row does not need downgrade the effective grade equals the claim;
    /// otherwise it must rank strictly below the claim and carry both a recorded
    /// trigger and a precise downgraded label.
    pub fn downgrade_consistent(&self) -> bool {
        if self.needs_downgrade() {
            self.properly_downgraded()
                && self.downgrade_trigger.is_some()
                && self
                    .downgraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_grade == self.claimed_grade
        }
    }

    /// Whether the imported posture is consistent: a provider/imported profile
    /// never reads as a locally verified profile, and a local profile never leans
    /// on imported proof.
    pub fn imported_posture_consistent(&self) -> bool {
        if self.provider_or_imported() {
            !self.verification.proof_currency.is_current_local()
        } else {
            !self.verification.proof_currency.is_imported_current()
        }
    }

    /// Whether the keyboard fallback is complete on both the provider and the
    /// session, so voice never becomes a dead end.
    pub fn keyboard_fallback_ok(&self) -> bool {
        self.provider.keyboard_fallback_available && self.session.keyboard_fallback_available
    }

    /// Whether the row's session binds the row's provider descriptor.
    pub fn session_binds_provider(&self) -> bool {
        self.session.provider_id == self.provider.provider_id
    }

    /// Whether the profile fingerprint is a real non-display basis distinct from
    /// the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.profile_fingerprint_token.trim();
        !token.is_empty() && token != self.profile_id.trim()
    }

    /// Whether every field required to record this row is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.profile_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.fingerprint_independent_of_id()
            && self.provider.is_well_formed()
            && self.session.is_well_formed()
            && self.session_binds_provider()
            && self.verification.is_well_formed()
            && self.downgrade_consistent()
            && self.imported_posture_consistent()
            && self.keyboard_fallback_ok()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.source_contract_refs.is_empty()
            && self
                .source_contract_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceQualificationGuardrails {
    /// Command mode and dictation mode are never conflated or inferred silently.
    pub command_and_dictation_modes_never_conflated: bool,
    /// Push-to-talk (or equivalent explicit activation) is the default.
    pub push_to_talk_or_explicit_activation_is_default: bool,
    /// Provider locality and transcript retention are always disclosed.
    pub provider_locality_and_retention_always_disclosed: bool,
    /// Transcripts are bounded and raw transcripts are excluded by default.
    pub transcripts_bounded_and_raw_excluded_by_default: bool,
    /// Voice actions reuse the command/undo/policy parity of other inputs.
    pub voice_actions_reuse_command_undo_policy_parity: bool,
    /// A keyboard fallback is always available; voice is never a dead end.
    pub keyboard_fallback_always_available: bool,
    /// Background listening is never on by default.
    pub background_listening_never_default_on: bool,
    /// Claimed voice profiles are kept separate from broader future ambitions.
    pub claimed_profiles_separated_from_future_voice_ambitions: bool,
    /// Any claimed profile lacking current proof auto-downgrades below its claim.
    pub rows_auto_downgrade_without_current_proof: bool,
}

impl VoiceQualificationGuardrails {
    /// Whether every guardrail invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.command_and_dictation_modes_never_conflated
            && self.push_to_talk_or_explicit_activation_is_default
            && self.provider_locality_and_retention_always_disclosed
            && self.transcripts_bounded_and_raw_excluded_by_default
            && self.voice_actions_reuse_command_undo_policy_parity
            && self.keyboard_fallback_always_available
            && self.background_listening_never_default_on
            && self.claimed_profiles_separated_from_future_voice_ambitions
            && self.rows_auto_downgrade_without_current_proof
    }
}

/// Consumer projection block: the surfaces that read this matrix without cloning
/// voice state text by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceQualificationConsumerProjection {
    /// Product surfaces ingest this matrix.
    pub product_ingests_matrix: bool,
    /// Command / help surfaces ingest the same matrix.
    pub command_help_ingests_matrix: bool,
    /// Accessibility surfaces ingest the same matrix.
    pub accessibility_ingests_matrix: bool,
    /// Diagnostics surfaces ingest the same matrix.
    pub diagnostics_ingests_matrix: bool,
    /// Support-export surfaces ingest the same matrix.
    pub support_export_ingests_matrix: bool,
    /// Release-control surfaces ingest the same matrix.
    pub release_control_ingests_matrix: bool,
    /// Downgraded profiles are visibly labeled below their claim in every surface.
    pub downgraded_profiles_labeled_below_claim: bool,
}

impl VoiceQualificationConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.product_ingests_matrix
            && self.command_help_ingests_matrix
            && self.accessibility_ingests_matrix
            && self.diagnostics_ingests_matrix
            && self.support_export_ingests_matrix
            && self.release_control_ingests_matrix
            && self.downgraded_profiles_labeled_below_claim
    }
}

/// Verification freshness block for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceQualificationFreshness {
    /// Verification-freshness SLO in hours.
    pub verification_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last verification refresh.
    pub last_verification_refresh: String,
    /// True when stale verification automatically downgrades claimed rows.
    pub auto_downgrade_on_stale: bool,
}

impl VoiceQualificationFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.verification_freshness_slo_hours > 0
            && !self.last_verification_refresh.trim().is_empty()
    }
}

/// Constructor input for [`M5VoiceQualificationMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5VoiceQualificationMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub label: String,
    /// Per-profile rows.
    pub rows: Vec<VoiceClaimedProfileRow>,
    /// Guardrail invariants block.
    pub guardrails: VoiceQualificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: VoiceQualificationConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: VoiceQualificationFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 voice-mode provider / transcript-retention / command-parity
/// qualification matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VoiceQualificationMatrixPacket {
    /// Record kind; must equal [`VOICE_QUALIFICATION_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VOICE_QUALIFICATION_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub label: String,
    /// Per-profile rows.
    pub rows: Vec<VoiceClaimedProfileRow>,
    /// Guardrail invariants block.
    pub guardrails: VoiceQualificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: VoiceQualificationConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: VoiceQualificationFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5VoiceQualificationMatrixPacket {
    /// Builds a voice-qualification matrix packet.
    pub fn new(input: M5VoiceQualificationMatrixPacketInput) -> Self {
        Self {
            record_kind: VOICE_QUALIFICATION_MATRIX_RECORD_KIND.to_owned(),
            schema_version: VOICE_QUALIFICATION_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            verification_freshness: input.verification_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surface kinds represented by some row in this packet.
    pub fn represented_surface_kinds(&self) -> BTreeSet<VoiceSurfaceKind> {
        self.rows.iter().map(|row| row.surface_kind).collect()
    }

    /// Voice modes represented across rows.
    pub fn represented_modes(&self) -> BTreeSet<VoiceModeClass> {
        self.rows.iter().map(|row| row.session.mode_class).collect()
    }

    /// Provider classes represented across rows.
    pub fn represented_provider_classes(&self) -> BTreeSet<VoiceProviderClass> {
        self.rows
            .iter()
            .map(|row| row.provider.provider_class)
            .collect()
    }

    /// Count of rows that auto-downgraded below their claim.
    pub fn downgraded_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.needs_downgrade()).count()
    }

    /// Count of rows holding a public claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Count of Labs/unadvertised rows.
    pub fn labs_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.claim_posture == VoiceClaimPosture::LabsUnadvertised)
            .count()
    }

    /// Count of provider-linked / imported rows.
    pub fn provider_or_imported_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.provider_or_imported())
            .count()
    }

    /// Resolves a row by its id.
    pub fn row(&self, profile_id: &str) -> Option<&VoiceClaimedProfileRow> {
        self.rows.iter().find(|row| row.profile_id == profile_id)
    }

    /// Validates the voice-qualification matrix invariants.
    pub fn validate(&self) -> Vec<VoiceMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != VOICE_QUALIFICATION_MATRIX_RECORD_KIND {
            violations.push(VoiceMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != VOICE_QUALIFICATION_MATRIX_SCHEMA_VERSION {
            violations.push(VoiceMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(VoiceMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(VoiceMatrixViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(VoiceMatrixViolation::ConsumerProjectionIncomplete);
        }
        if !self.verification_freshness.is_valid() {
            violations.push(VoiceMatrixViolation::VerificationFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("voice qualification matrix packet serializes"),
        ) {
            violations.push(VoiceMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("voice qualification matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, accessibility, or release
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Voice-Mode Provider / Transcript-Retention / Command-Parity Qualification Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Rows: {} ({} claimed, {} Labs/unadvertised, {} provider/imported, {} downgraded)\n",
            self.rows.len(),
            self.claimed_row_count(),
            self.labs_row_count(),
            self.provider_or_imported_row_count(),
            self.downgraded_row_count()
        ));
        out.push_str(&format!(
            "- Surface kinds: {} / {}\n",
            self.represented_surface_kinds().len(),
            VoiceSurfaceKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Verification freshness SLO: {} hours (last refresh: {})\n",
            self.verification_freshness.verification_freshness_slo_hours,
            self.verification_freshness.last_verification_refresh
        ));
        out.push_str("\n## Profiles\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.profile_id,
                row.surface_kind.as_str(),
                row.claimed_grade.as_str(),
                row.effective_grade.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!(
                "  - posture `{}`, origin `{}`\n",
                row.claim_posture.as_str(),
                row.origin_class.as_str()
            ));
            out.push_str(&format!(
                "  - session mode = `{}`, activation = `{}`, policy = `{}`\n",
                row.session.mode_class.as_str(),
                row.session.activation_class.as_str(),
                row.session.policy_state.as_str()
            ));
            out.push_str(&format!(
                "  - provider `{}` ({}), locality = `{}`, transport = `{}`\n",
                row.provider.provider_id,
                row.provider.provider_class.as_str(),
                row.provider.processing_locality.as_str(),
                row.provider.transport_class.as_str()
            ));
            out.push_str(&format!(
                "  - retention = `{}`, audio = `{}`, export = `{}`, raw_excluded = {}\n",
                row.provider.retention_posture.retention_mode.as_str(),
                row.provider.retention_posture.audio_retention.as_str(),
                row.provider.retention_posture.transcript_export.as_str(),
                row.provider
                    .retention_posture
                    .raw_transcripts_excluded_by_default
            ));
            out.push_str(&format!(
                "  - command parity complete = {}, keyboard fallback = {}\n",
                row.command_parity.parity_complete(),
                row.keyboard_fallback_ok()
            ));
            out.push_str(&format!(
                "  - verification = `{}`\n",
                row.verification.proof_currency.as_str()
            ));
            if let Some(reason) = &row.unavailable_reason {
                out.push_str(&format!("  - unavailable reason = `{}`\n", reason.as_str()));
            }
            if let Some(label) = &row.downgraded_label {
                out.push_str(&format!("  - Downgraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum VoiceQualificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<VoiceMatrixViolation>),
}

impl fmt::Display for VoiceQualificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "voice qualification matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "voice qualification matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for VoiceQualificationArtifactError {}

/// Validation failures emitted by [`M5VoiceQualificationMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoiceMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed surface kind is represented by no row.
    RequiredSurfaceKindMissing,
    /// Command mode is represented by no row.
    CommandModeCoverageMissing,
    /// Dictation mode is represented by no row.
    DictationModeCoverageMissing,
    /// Hosted-remote processing is represented by no row.
    HostedLocalityCoverageMissing,
    /// No Labs/unadvertised row separates claimed scope from future ambitions.
    LabsProfileCaseMissing,
    /// No row demonstrates honest auto-downgrade on a denied axis.
    DowngradedRowCaseMissing,
    /// No clean, current, claimed row anchors a fully qualified claim.
    CleanClaimedCaseMissing,
    /// No provider-linked / imported row is present.
    ProviderOrImportedCaseMissing,
    /// No unavailable / fallback row proves a complete keyboard fallback.
    UnavailableFallbackCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A claimed row was not downgraded below its claim despite a denied axis or
    /// uncurrent proof.
    RowNotDowngradedOnDeniedAxis,
    /// A downgraded row lacks a precise downgraded label or trigger.
    DowngradedRowMissingLabelOrTrigger,
    /// A row's profile fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A row's keyboard fallback is missing on the provider or session.
    KeyboardFallbackMissing,
    /// A row retains raw transcripts in support/telemetry by default.
    RawTranscriptRetainedByDefault,
    /// A provider has background listening on by default.
    BackgroundListeningDefaultOn,
    /// A session's background listening is inconsistent with its activation.
    BackgroundListeningInconsistentWithActivation,
    /// A provider does not gate continuous/wake capabilities behind an opt-in.
    ProviderOptInGuardsMissing,
    /// A claimed row conflates or blocks its mode without downgrading.
    ModeSeparationDeniedNotDowngraded,
    /// A claimed row defaults to continuous/wake activation without downgrading.
    ActivationDefaultDeniedNotDowngraded,
    /// A claimed row leaves hosted/enterprise locality undisclosed without
    /// downgrading.
    ProviderLocalityDeniedNotDowngraded,
    /// A claimed row ships incomplete command parity without downgrading.
    CommandParityDeniedNotDowngraded,
    /// A provider-linked / imported row reads as a locally verified profile.
    ImportedReadsAsLocal,
    /// A row's verification proof is not reopenable.
    VerificationProofNotReopenable,
    /// A row's session does not bind the row's provider descriptor.
    SessionProviderRefMismatch,
    /// A provider descriptor is structurally inconsistent.
    ProviderDescriptorInconsistent,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Verification freshness block is incomplete.
    VerificationFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl VoiceMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceKindMissing => "required_surface_kind_missing",
            Self::CommandModeCoverageMissing => "command_mode_coverage_missing",
            Self::DictationModeCoverageMissing => "dictation_mode_coverage_missing",
            Self::HostedLocalityCoverageMissing => "hosted_locality_coverage_missing",
            Self::LabsProfileCaseMissing => "labs_profile_case_missing",
            Self::DowngradedRowCaseMissing => "downgraded_row_case_missing",
            Self::CleanClaimedCaseMissing => "clean_claimed_case_missing",
            Self::ProviderOrImportedCaseMissing => "provider_or_imported_case_missing",
            Self::UnavailableFallbackCaseMissing => "unavailable_fallback_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowNotDowngradedOnDeniedAxis => "row_not_downgraded_on_denied_axis",
            Self::DowngradedRowMissingLabelOrTrigger => "downgraded_row_missing_label_or_trigger",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::KeyboardFallbackMissing => "keyboard_fallback_missing",
            Self::RawTranscriptRetainedByDefault => "raw_transcript_retained_by_default",
            Self::BackgroundListeningDefaultOn => "background_listening_default_on",
            Self::BackgroundListeningInconsistentWithActivation => {
                "background_listening_inconsistent_with_activation"
            }
            Self::ProviderOptInGuardsMissing => "provider_opt_in_guards_missing",
            Self::ModeSeparationDeniedNotDowngraded => "mode_separation_denied_not_downgraded",
            Self::ActivationDefaultDeniedNotDowngraded => {
                "activation_default_denied_not_downgraded"
            }
            Self::ProviderLocalityDeniedNotDowngraded => "provider_locality_denied_not_downgraded",
            Self::CommandParityDeniedNotDowngraded => "command_parity_denied_not_downgraded",
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::VerificationProofNotReopenable => "verification_proof_not_reopenable",
            Self::SessionProviderRefMismatch => "session_provider_ref_mismatch",
            Self::ProviderDescriptorInconsistent => "provider_descriptor_inconsistent",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::VerificationFreshnessIncomplete => "verification_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_voice_qualification_matrix_export(
) -> Result<M5VoiceQualificationMatrixPacket, VoiceQualificationArtifactError> {
    let packet: M5VoiceQualificationMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/voice/m5-voice-qualification-matrix/support_export.json"
    )))
    .map_err(VoiceQualificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(VoiceQualificationArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5VoiceQualificationMatrixPacket,
    violations: &mut Vec<VoiceMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        VOICE_QUALIFICATION_MATRIX_SCHEMA_REF,
        VOICE_PROVIDER_DESCRIPTOR_SCHEMA_REF,
        VOICE_SESSION_STATE_SCHEMA_REF,
        VOICE_QUALIFICATION_MATRIX_DOC_REF,
        VOICE_QUALIFICATION_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(VoiceMatrixViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &M5VoiceQualificationMatrixPacket,
    violations: &mut Vec<VoiceMatrixViolation>,
) {
    let surface_kinds = packet.represented_surface_kinds();
    for required in VoiceSurfaceKind::ALL {
        if !surface_kinds.contains(&required) {
            violations.push(VoiceMatrixViolation::RequiredSurfaceKindMissing);
            break;
        }
    }

    let modes = packet.represented_modes();
    if !modes.contains(&VoiceModeClass::CommandModeActive) {
        violations.push(VoiceMatrixViolation::CommandModeCoverageMissing);
    }
    if !modes.contains(&VoiceModeClass::DictationModeActive) {
        violations.push(VoiceMatrixViolation::DictationModeCoverageMissing);
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.provider.processing_locality == ProcessingLocalityCue::HostedRemoteDisclosed)
    {
        violations.push(VoiceMatrixViolation::HostedLocalityCoverageMissing);
    }

    if packet.labs_row_count() == 0 {
        violations.push(VoiceMatrixViolation::LabsProfileCaseMissing);
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.needs_downgrade() && row.downgrade_consistent())
    {
        violations.push(VoiceMatrixViolation::DowngradedRowCaseMissing);
    }

    if !packet.rows.iter().any(|row| {
        !row.needs_downgrade()
            && row.is_claimed()
            && row.claimed_grade == VoiceQualificationGrade::QualifiedClaimedProfile
            && row.verification_current()
    }) {
        violations.push(VoiceMatrixViolation::CleanClaimedCaseMissing);
    }

    if packet.provider_or_imported_row_count() == 0 {
        violations.push(VoiceMatrixViolation::ProviderOrImportedCaseMissing);
    }

    if !packet.rows.iter().any(|row| {
        row.surface_kind == VoiceSurfaceKind::UnavailableFallback && row.keyboard_fallback_ok()
    }) {
        violations.push(VoiceMatrixViolation::UnavailableFallbackCaseMissing);
    }
}

fn validate_rows(
    packet: &M5VoiceQualificationMatrixPacket,
    violations: &mut Vec<VoiceMatrixViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(VoiceMatrixViolation::RowIncomplete);
        }
        if row.needs_downgrade() && !row.properly_downgraded() {
            violations.push(VoiceMatrixViolation::RowNotDowngradedOnDeniedAxis);
        }
        if row.needs_downgrade()
            && (row.downgrade_trigger.is_none()
                || !row
                    .downgraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations.push(VoiceMatrixViolation::DowngradedRowMissingLabelOrTrigger);
        }
        if !row.fingerprint_independent_of_id() {
            violations.push(VoiceMatrixViolation::FingerprintSubstitutesIdentity);
        }

        // Absolute invariants — never allowed even on a downgraded or Labs row.
        if !row.keyboard_fallback_ok() {
            violations.push(VoiceMatrixViolation::KeyboardFallbackMissing);
        }
        if !row.provider.retention_posture.raw_excluded() {
            violations.push(VoiceMatrixViolation::RawTranscriptRetainedByDefault);
        }
        if !row
            .provider
            .capability_disclosure
            .background_listening_default_off
        {
            violations.push(VoiceMatrixViolation::BackgroundListeningDefaultOn);
        }
        if !row.provider.capability_disclosure.opt_in_guards_hold() {
            violations.push(VoiceMatrixViolation::ProviderOptInGuardsMissing);
        }
        if !row.session.background_consistent() {
            violations.push(VoiceMatrixViolation::BackgroundListeningInconsistentWithActivation);
        }
        if !row.provider.is_well_formed() {
            violations.push(VoiceMatrixViolation::ProviderDescriptorInconsistent);
        }
        if !row.session_binds_provider() {
            violations.push(VoiceMatrixViolation::SessionProviderRefMismatch);
        }

        // Denied-axis conditions — for a claimed row they must be reflected by a
        // strict downgrade rather than left standing at the claim.
        if row.is_claimed() && !row.properly_downgraded() {
            if !row.mode_separation_ok() {
                violations.push(VoiceMatrixViolation::ModeSeparationDeniedNotDowngraded);
            }
            if !row.push_to_talk_default_ok() {
                violations.push(VoiceMatrixViolation::ActivationDefaultDeniedNotDowngraded);
            }
            if !row.locality_disclosed_ok() {
                violations.push(VoiceMatrixViolation::ProviderLocalityDeniedNotDowngraded);
            }
            if !row.command_parity.parity_complete() {
                violations.push(VoiceMatrixViolation::CommandParityDeniedNotDowngraded);
            }
        }

        if !row.imported_posture_consistent() {
            violations.push(VoiceMatrixViolation::ImportedReadsAsLocal);
        }
        if !row.verification.is_well_formed() {
            violations.push(VoiceMatrixViolation::VerificationProofNotReopenable);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(VoiceMatrixViolation::RowEvidenceMissing);
        }
    }
}

/// Whether a downgraded label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise downgrade truth.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "downgraded"
            | "unverified"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Stable packet id minted by [`seeded_voice_qualification_matrix_packet`].
pub const SEED_VOICE_QUALIFICATION_PACKET_ID: &str = "m5-voice-qualification-matrix:stable:0001";

/// Mint timestamp used by [`seeded_voice_qualification_matrix_packet`].
pub const SEED_VOICE_QUALIFICATION_MINTED_AT: &str = "2026-06-14T00:00:00Z";

/// Builds the canonical, validating voice-qualification matrix packet that the
/// checked-in support export, the Markdown summary, and the conformance tests all
/// share, so the in-crate builder stays byte-aligned with the artifact.
///
/// The seed covers every claimed voice surface kind, both command and dictation
/// modes, on-device / hosted / enterprise / disabled provider classes, anchors a
/// clean local push-to-talk claim, holds an enterprise-managed narrowed profile,
/// a provider-linked profile that never reads as a local rerun, a Labs profile
/// that keeps voice out of public scope, an unavailable-fallback profile, and one
/// claimed profile that auto-downgrades because its hosted provider went
/// unavailable.
pub fn seeded_voice_qualification_matrix_packet() -> M5VoiceQualificationMatrixPacket {
    M5VoiceQualificationMatrixPacket::new(M5VoiceQualificationMatrixPacketInput {
        packet_id: SEED_VOICE_QUALIFICATION_PACKET_ID.to_owned(),
        label:
            "M5 Voice-Mode Provider / Transcript-Retention / Command-Parity Qualification Matrix"
                .to_owned(),
        rows: seeded_rows(),
        guardrails: VoiceQualificationGuardrails {
            command_and_dictation_modes_never_conflated: true,
            push_to_talk_or_explicit_activation_is_default: true,
            provider_locality_and_retention_always_disclosed: true,
            transcripts_bounded_and_raw_excluded_by_default: true,
            voice_actions_reuse_command_undo_policy_parity: true,
            keyboard_fallback_always_available: true,
            background_listening_never_default_on: true,
            claimed_profiles_separated_from_future_voice_ambitions: true,
            rows_auto_downgrade_without_current_proof: true,
        },
        consumer_projection: VoiceQualificationConsumerProjection {
            product_ingests_matrix: true,
            command_help_ingests_matrix: true,
            accessibility_ingests_matrix: true,
            diagnostics_ingests_matrix: true,
            support_export_ingests_matrix: true,
            release_control_ingests_matrix: true,
            downgraded_profiles_labeled_below_claim: true,
        },
        verification_freshness: VoiceQualificationFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_VOICE_QUALIFICATION_MINTED_AT.to_owned(),
            auto_downgrade_on_stale: true,
        },
        source_contract_refs: vec![
            VOICE_QUALIFICATION_MATRIX_SCHEMA_REF.to_owned(),
            VOICE_PROVIDER_DESCRIPTOR_SCHEMA_REF.to_owned(),
            VOICE_SESSION_STATE_SCHEMA_REF.to_owned(),
            VOICE_QUALIFICATION_MATRIX_DOC_REF.to_owned(),
            VOICE_QUALIFICATION_MATRIX_ARTIFACT_REF.to_owned(),
            VOICE_AND_DICTATION_CONTRACT_REF.to_owned(),
            VOICE_M4_SURFACE_QUALIFICATION_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_VOICE_QUALIFICATION_MINTED_AT.to_owned(),
    })
}

fn seeded_rows() -> Vec<VoiceClaimedProfileRow> {
    vec![
        local_command_overlay_row(),
        local_dictation_row(),
        transcript_correction_row(),
        enterprise_provider_privacy_row(),
        provider_linked_high_impact_row(),
        unavailable_fallback_row(),
        labs_continuous_row(),
        hosted_provider_unavailable_downgraded_row(),
    ]
}

/// On-device push-to-talk command overlay: the clean, fully-claimed local profile.
fn local_command_overlay_row() -> VoiceClaimedProfileRow {
    let provider = local_provider("voice.provider.on_device_command");
    let session = session(
        "voice.session.local_command_overlay",
        VoiceModeClass::CommandModeActive,
        VoiceActivationClass::PushToTalkHeld,
        &provider,
    );
    base_row(BaseRow {
        profile_id: "voice-qual:command-overlay:local:0001",
        surface_kind: VoiceSurfaceKind::CommandOverlay,
        origin_class: VoiceProfileOriginClass::FirstPartyLocalProfile,
        label: "On-device push-to-talk command overlay routed through the canonical command graph",
        claim_posture: VoiceClaimPosture::ClaimedBeta,
        provider,
        session,
        command_parity: full_parity(),
        currency: VoiceProofCurrency::VerifiedCurrent,
        claimed: VoiceQualificationGrade::QualifiedClaimedProfile,
    })
}

/// On-device dictation input on the shared editor undo stack.
fn local_dictation_row() -> VoiceClaimedProfileRow {
    let provider = local_provider("voice.provider.on_device_dictation");
    let session = session(
        "voice.session.local_dictation",
        VoiceModeClass::DictationModeActive,
        VoiceActivationClass::PushToTalkToggle,
        &provider,
    );
    base_row(BaseRow {
        profile_id: "voice-qual:dictation-input:local:0001",
        surface_kind: VoiceSurfaceKind::DictationInput,
        origin_class: VoiceProfileOriginClass::FirstPartyLocalProfile,
        label: "On-device dictation that inserts text on the shared editor undo stack",
        claim_posture: VoiceClaimPosture::ClaimedBeta,
        provider,
        session,
        command_parity: full_parity(),
        currency: VoiceProofCurrency::VerifiedCurrent,
        claimed: VoiceQualificationGrade::QualifiedClaimedProfile,
    })
}

/// Transcript correction / review surface before a privileged commit.
fn transcript_correction_row() -> VoiceClaimedProfileRow {
    let provider = local_provider("voice.provider.on_device_correction");
    let mut session = session(
        "voice.session.transcript_correction",
        VoiceModeClass::DictationModeActive,
        VoiceActivationClass::ManualCommandActivation,
        &provider,
    );
    session.transcript_correction_posture =
        TranscriptCorrectionPosture::CorrectionRequiredBeforeCommit;
    base_row(BaseRow {
        profile_id: "voice-qual:transcript-correction:local:0001",
        surface_kind: VoiceSurfaceKind::TranscriptCorrection,
        origin_class: VoiceProfileOriginClass::FirstPartyLocalProfile,
        label: "Transcript correction surface that requires a correction window before any privileged commit",
        claim_posture: VoiceClaimPosture::ClaimedBeta,
        provider,
        session,
        command_parity: full_parity(),
        currency: VoiceProofCurrency::CachedWithinWindow,
        claimed: VoiceQualificationGrade::QualifiedClaimedProfile,
    })
}

/// Enterprise-managed provider/privacy settings: a deliberately narrowed profile
/// with disclosed hosted processing and per-contract retention.
fn enterprise_provider_privacy_row() -> VoiceClaimedProfileRow {
    let provider = enterprise_provider("voice.provider.enterprise_relay");
    let mut session = session(
        "voice.session.enterprise_privacy",
        VoiceModeClass::CommandModeActive,
        VoiceActivationClass::PushToTalkHeld,
        &provider,
    );
    session.policy_state = VoicePolicyState::EnterprisePolicyManaged;
    session.processing_locality = ProcessingLocalityCue::HostedRemoteDisclosed;
    base_row(BaseRow {
        profile_id: "voice-qual:provider-privacy:enterprise:0001",
        surface_kind: VoiceSurfaceKind::ProviderPrivacySettings,
        origin_class: VoiceProfileOriginClass::EnterpriseManagedProfile,
        label: "Enterprise-managed provider/privacy settings with disclosed hosted relay and audited per-contract retention",
        claim_posture: VoiceClaimPosture::ClaimedPreview,
        provider,
        session,
        command_parity: full_parity(),
        currency: VoiceProofCurrency::VerifiedCurrent,
        claimed: VoiceQualificationGrade::QualifiedNarrowedProfile,
    })
}

/// Provider-linked high-impact action review whose qualification is provider
/// backed and never reads as a local rerun.
fn provider_linked_high_impact_row() -> VoiceClaimedProfileRow {
    let provider = enterprise_provider("voice.provider.linked_high_impact");
    let mut session = session(
        "voice.session.high_impact_review",
        VoiceModeClass::CommandModeActive,
        VoiceActivationClass::PushToTalkHeld,
        &provider,
    );
    session.processing_locality = ProcessingLocalityCue::HostedRemoteDisclosed;
    session.command_preview_class = CommandPreviewClass::PreviewRequiredForPrivilegedActions;
    base_row(BaseRow {
        profile_id: "voice-qual:high-impact-review:provider:0001",
        surface_kind: VoiceSurfaceKind::HighImpactActionReview,
        origin_class: VoiceProfileOriginClass::ProviderLinkedProfile,
        label: "Provider-linked high-impact action review where spoken destructive/publish actions require transcript confirmation",
        claim_posture: VoiceClaimPosture::ClaimedPreview,
        provider,
        session,
        command_parity: full_parity(),
        currency: VoiceProofCurrency::ImportedCurrent,
        claimed: VoiceQualificationGrade::QualifiedNarrowedProfile,
    })
}

/// Unavailable / fallback profile that always offers a complete keyboard path.
fn unavailable_fallback_row() -> VoiceClaimedProfileRow {
    let provider = disabled_provider("voice.provider.disabled_fallback");
    let mut session = session(
        "voice.session.unavailable_fallback",
        VoiceModeClass::IdleMicrophoneOff,
        VoiceActivationClass::ManualCommandActivation,
        &provider,
    );
    session.mic_indicator_class = MicIndicatorClass::PersistentIndicatorHiddenCaptureDisabled;
    session.processing_locality = ProcessingLocalityCue::ProcessingUnavailable;
    let mut row = base_row(BaseRow {
        profile_id: "voice-qual:unavailable-fallback:local:0001",
        surface_kind: VoiceSurfaceKind::UnavailableFallback,
        origin_class: VoiceProfileOriginClass::FirstPartyLocalProfile,
        label: "Unavailable voice surface (no microphone) that always falls back to the keyboard / command palette",
        claim_posture: VoiceClaimPosture::ClaimedBeta,
        provider,
        session,
        command_parity: full_parity(),
        currency: VoiceProofCurrency::VerifiedCurrent,
        claimed: VoiceQualificationGrade::QualifiedNarrowedProfile,
    });
    // Provider unavailable: the claimed row downgrades honestly to Labs scope.
    row.effective_grade = VoiceQualificationGrade::LabsUnadvertisedProfile;
    row.downgrade_trigger = Some(VoiceQualificationDowngradeTrigger::ProviderUnavailableDowngraded);
    row.downgraded_label = Some(
        "Microphone unavailable; voice held at fallback scope with a complete keyboard / command-palette path rather than claiming live capture"
            .to_owned(),
    );
    row.unavailable_reason = Some(VoiceUnavailableReason::NoMicrophone);
    row
}

/// Labs / unadvertised continuous-listening profile, kept out of public scope.
fn labs_continuous_row() -> VoiceClaimedProfileRow {
    let provider = local_provider("voice.provider.labs_continuous");
    let mut session = session(
        "voice.session.labs_continuous",
        VoiceModeClass::ContinuousListeningActiveUserOptedIn,
        VoiceActivationClass::WakePhraseContinuousUserOptedIn,
        &provider,
    );
    session.background_listening_state = BackgroundListeningState::OnUserOptedIn;
    let mut provider = provider;
    provider.background_listening_state = BackgroundListeningState::OnUserOptedIn;
    let mut row = base_row(BaseRow {
        profile_id: "voice-qual:command-overlay:labs-continuous:0001",
        surface_kind: VoiceSurfaceKind::CommandOverlay,
        origin_class: VoiceProfileOriginClass::FirstPartyLocalProfile,
        label: "Labs/unadvertised wake-phrase continuous listening, opted-in and explicitly out of stable scope",
        claim_posture: VoiceClaimPosture::LabsUnadvertised,
        provider,
        session,
        command_parity: full_parity(),
        currency: VoiceProofCurrency::VerifiedCurrent,
        claimed: VoiceQualificationGrade::LabsUnadvertisedProfile,
    });
    row.session.provider_id = row.provider.provider_id.clone();
    row
}

/// Hosted command profile claiming a full stable scope whose provider went
/// unavailable: auto-downgrades below its claim with a precise label.
fn hosted_provider_unavailable_downgraded_row() -> VoiceClaimedProfileRow {
    let mut provider = enterprise_provider("voice.provider.hosted_unavailable");
    provider.provider_class = VoiceProviderClass::ApprovedRemoteDisclosed;
    let mut session = session(
        "voice.session.hosted_unavailable",
        VoiceModeClass::CommandModeActive,
        VoiceActivationClass::PushToTalkHeld,
        &provider,
    );
    session.processing_locality = ProcessingLocalityCue::ProcessingUnavailable;
    let mut row = base_row(BaseRow {
        profile_id: "voice-qual:command-overlay:hosted-unavailable:0001",
        surface_kind: VoiceSurfaceKind::CommandOverlay,
        origin_class: VoiceProfileOriginClass::FirstPartyLocalProfile,
        label: "Hosted command overlay that claimed full scope but whose remote provider is unavailable",
        claim_posture: VoiceClaimPosture::ClaimedBeta,
        provider,
        session,
        command_parity: full_parity(),
        currency: VoiceProofCurrency::VerifiedCurrent,
        claimed: VoiceQualificationGrade::QualifiedClaimedProfile,
    });
    row.effective_grade = VoiceQualificationGrade::QualifiedNarrowedProfile;
    row.downgrade_trigger = Some(VoiceQualificationDowngradeTrigger::ProviderUnavailableDowngraded);
    row.downgraded_label = Some(
        "Remote speech provider is unavailable; held narrowed with a complete keyboard fallback rather than claiming full hosted command scope"
            .to_owned(),
    );
    row.unavailable_reason = Some(VoiceUnavailableReason::ProviderUnavailable);
    row
}

fn full_parity() -> VoiceCommandParity {
    VoiceCommandParity {
        stable_command_ids: true,
        disabled_with_reason: true,
        preview_apply_revert: true,
        approval_requirements: true,
        undo_grouping: true,
        audit_support_lineage: true,
        high_impact_review: true,
        keyboard_fallback_parity: true,
    }
}

fn bounded_local_retention() -> VoiceTranscriptRetentionPosture {
    VoiceTranscriptRetentionPosture {
        retention_mode: RetentionMode::EphemeralAudioLocalOnlyNoTranscriptRetained,
        audio_retention: AudioRetentionClass::EphemeralAudioLocalOnly,
        transcript_export: TranscriptExportPosture::MetadataOnlySupportExport,
        bounded_correction_buffer: true,
        raw_transcripts_excluded_by_default: true,
        redaction_before_support_export: true,
    }
}

fn enterprise_retention() -> VoiceTranscriptRetentionPosture {
    VoiceTranscriptRetentionPosture {
        retention_mode: RetentionMode::TranscriptRetainedProviderPerContract,
        audio_retention: AudioRetentionClass::AudioRetainedProviderPerContract,
        transcript_export: TranscriptExportPosture::ProviderContractRetained,
        bounded_correction_buffer: true,
        raw_transcripts_excluded_by_default: true,
        redaction_before_support_export: true,
    }
}

fn disclosure(command: bool, dictation: bool) -> VoiceProviderCapabilityDisclosure {
    VoiceProviderCapabilityDisclosure {
        supports_command_mode: command,
        supports_dictation_mode: dictation,
        supports_correction_review: true,
        continuous_listening_requires_opt_in: true,
        wake_word_requires_opt_in: true,
        background_listening_default_off: true,
    }
}

fn local_provider(provider_id: &str) -> VoiceProviderDescriptor {
    VoiceProviderDescriptor {
        record_kind: VOICE_PROVIDER_DESCRIPTOR_RECORD_KIND.to_owned(),
        schema_version: VOICE_QUALIFICATION_MATRIX_SCHEMA_VERSION,
        provider_id: provider_id.to_owned(),
        provider_fingerprint_token: format!("fp:provider:{provider_id}"),
        provider_class: VoiceProviderClass::OnDeviceLocal,
        processing_locality: ProcessingLocalityCue::LocalOnDevice,
        transport_class: VoiceTransportClass::LocalInProcessOnly,
        background_listening_state: BackgroundListeningState::OffDefault,
        capability_disclosure: disclosure(true, true),
        retention_posture: bounded_local_retention(),
        data_class_label: "voice_transcript_local_only".to_owned(),
        audit_capable: true,
        keyboard_fallback_available: true,
        fallback_provider_id: None,
    }
}

fn enterprise_provider(provider_id: &str) -> VoiceProviderDescriptor {
    VoiceProviderDescriptor {
        record_kind: VOICE_PROVIDER_DESCRIPTOR_RECORD_KIND.to_owned(),
        schema_version: VOICE_QUALIFICATION_MATRIX_SCHEMA_VERSION,
        provider_id: provider_id.to_owned(),
        provider_fingerprint_token: format!("fp:provider:{provider_id}"),
        provider_class: VoiceProviderClass::EnterpriseRelayManaged,
        processing_locality: ProcessingLocalityCue::HostedRemoteDisclosed,
        transport_class: VoiceTransportClass::PolicyBoundedDisclosedEndpoint,
        background_listening_state: BackgroundListeningState::OffDefault,
        capability_disclosure: disclosure(true, false),
        retention_posture: enterprise_retention(),
        data_class_label: "voice_transcript_enterprise_contract".to_owned(),
        audit_capable: true,
        keyboard_fallback_available: true,
        fallback_provider_id: Some("voice.provider.on_device_command".to_owned()),
    }
}

fn disabled_provider(provider_id: &str) -> VoiceProviderDescriptor {
    VoiceProviderDescriptor {
        record_kind: VOICE_PROVIDER_DESCRIPTOR_RECORD_KIND.to_owned(),
        schema_version: VOICE_QUALIFICATION_MATRIX_SCHEMA_VERSION,
        provider_id: provider_id.to_owned(),
        provider_fingerprint_token: format!("fp:provider:{provider_id}"),
        provider_class: VoiceProviderClass::ProviderDisabled,
        processing_locality: ProcessingLocalityCue::ProcessingUnavailable,
        transport_class: VoiceTransportClass::TransportBlocked,
        background_listening_state: BackgroundListeningState::OffDefault,
        capability_disclosure: disclosure(true, true),
        retention_posture: VoiceTranscriptRetentionPosture {
            retention_mode: RetentionMode::NoAudioNoTranscriptRetained,
            audio_retention: AudioRetentionClass::AudioCaptureBlocked,
            transcript_export: TranscriptExportPosture::NoTranscriptExport,
            bounded_correction_buffer: false,
            raw_transcripts_excluded_by_default: true,
            redaction_before_support_export: true,
        },
        data_class_label: "voice_disabled_no_capture".to_owned(),
        audit_capable: true,
        keyboard_fallback_available: true,
        fallback_provider_id: Some("voice.provider.on_device_command".to_owned()),
    }
}

fn session(
    session_id: &str,
    mode_class: VoiceModeClass,
    activation_class: VoiceActivationClass,
    provider: &VoiceProviderDescriptor,
) -> VoiceSessionState {
    VoiceSessionState {
        record_kind: VOICE_SESSION_STATE_RECORD_KIND.to_owned(),
        schema_version: VOICE_QUALIFICATION_MATRIX_SCHEMA_VERSION,
        session_id: session_id.to_owned(),
        mode_class,
        activation_class,
        mic_indicator_class: if mode_class.is_capturing() {
            MicIndicatorClass::PersistentIndicatorVisibleCaptureActive
        } else {
            MicIndicatorClass::PersistentIndicatorVisibleCaptureIdle
        },
        processing_locality: provider.processing_locality,
        provider_id: provider.provider_id.clone(),
        background_listening_state: provider.background_listening_state,
        command_preview_class: CommandPreviewClass::PreviewRequiredForPrivilegedActions,
        transcript_correction_posture: TranscriptCorrectionPosture::CorrectionOptionalBeforeCommit,
        policy_state: VoicePolicyState::UserControlled,
        keyboard_fallback_available: true,
        accessibility_announced: true,
    }
}

/// Inline constructor input for one seeded profile row.
struct BaseRow {
    profile_id: &'static str,
    surface_kind: VoiceSurfaceKind,
    origin_class: VoiceProfileOriginClass,
    label: &'static str,
    claim_posture: VoiceClaimPosture,
    provider: VoiceProviderDescriptor,
    session: VoiceSessionState,
    command_parity: VoiceCommandParity,
    currency: VoiceProofCurrency,
    claimed: VoiceQualificationGrade,
}

fn base_row(base: BaseRow) -> VoiceClaimedProfileRow {
    let (proof_ref, proof_fingerprint_token) = if base.currency.is_absent() {
        (None, None)
    } else {
        (
            Some(format!("evidence:{}", base.profile_id)),
            Some(format!("fp:proof:{}", base.profile_id)),
        )
    };
    VoiceClaimedProfileRow {
        profile_id: base.profile_id.to_owned(),
        surface_kind: base.surface_kind,
        origin_class: base.origin_class,
        profile_fingerprint_token: format!("fp:profile:{}", base.profile_id),
        label_summary: base.label.to_owned(),
        claim_posture: base.claim_posture,
        provider: base.provider,
        session: base.session,
        command_parity: base.command_parity,
        verification: VoiceVerification {
            proof_currency: base.currency,
            proof_ref,
            proof_fingerprint_token,
            summary: format!(
                "{} qualification verified with {} proof",
                base.surface_kind.as_str(),
                base.currency.as_str()
            ),
        },
        claimed_grade: base.claimed,
        effective_grade: base.claimed,
        downgrade_trigger: None,
        downgraded_label: None,
        unavailable_reason: None,
        evidence_refs: vec![format!("evidence:row:{}", base.profile_id)],
        source_contract_refs: vec![VOICE_QUALIFICATION_MATRIX_DOC_REF.to_owned()],
    }
}
