//! Voice provider routing, language-profile switching, local-versus-hosted
//! processing, transcript retention/export controls, and policy/entitlement
//! gating for claimed voice profiles.
//!
//! The qualification matrix
//! ([`crate::freeze_the_m5_voice_mode_provider_transcript_retention_and_command_parity_matrix`])
//! freezes *which* claimed voice profiles rest on a disclosed provider and a
//! bounded retention posture. This lane models the **runtime resolution** that
//! sits underneath those claims: given a requested provider, a requested
//! language/acoustic profile, the active policy state, and the entitlement held
//! for the request, it resolves an explicit, inspectable routing outcome.
//!
//! The contract is privacy-first and never silent:
//!
//! * **Local-first defaults stay visible.** With no specific request the
//!   resolver routes to the designated on-device default and discloses it.
//! * **Switching never hides a retention or export change.** A routed or
//!   downgraded outcome records whether the active provider, locality, retention
//!   mode, or export posture differs from what was requested, so the chrome can
//!   surface the delta rather than swapping it silently.
//! * **Denials are explicit, not a quiet widening.** A policy or entitlement
//!   denial of a hosted/broader provider never falls back to a broader or less
//!   private provider. It either blocks explicitly (keyboard fallback intact) or
//!   downgrades to a strictly more-private on-device provider, with a precise,
//!   non-generic reason. The resolver only ever moves *toward* local, never
//!   toward a more remote provider.
//!
//! [`VoiceProviderRoutingOutcome`] is the resolution result, computed once by
//! [`resolve_voice_routing`] and recorded on each [`VoiceProviderRoutingRow`].
//! [`VoiceProviderRoutingPacket::validate`] re-derives every row's outcome from
//! its inputs, so a recorded outcome can never drift from the resolver, and it
//! refuses any packet whose routing widens authority, reduces privacy on a
//! denial, drops the keyboard fallback, or carries raw boundary material.
//!
//! Raw audio bytes, raw transcript text, raw provider payloads, private paths,
//! and credentials never cross this boundary; the packet carries only typed
//! class tokens, booleans, opaque ids, fingerprint digests, and redaction-aware
//! reviewable labels.
//!
//! The boundary schemas are
//! [`schemas/voice/retention-and-export.schema.json`](../../../../schemas/voice/retention-and-export.schema.json),
//! [`schemas/voice/voice-provider-descriptor.schema.json`](../../../../schemas/voice/voice-provider-descriptor.schema.json),
//! and
//! [`schemas/voice/voice-session.schema.json`](../../../../schemas/voice/voice-session.schema.json).
//! The truth doc is
//! [`docs/privacy/voice-processing-and-retention.md`](../../../../docs/privacy/voice-processing-and-retention.md).

#[cfg(test)]
mod tests;

pub mod seed;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub use crate::freeze_the_m5_voice_mode_provider_transcript_retention_and_command_parity_matrix::{
    AudioRetentionClass, TranscriptExportPosture, VoicePolicyState, VoiceProviderClass,
    VoiceTransportClass,
};
pub use crate::voice::{ProcessingLocalityCue, RetentionMode, VoiceClaimPosture};

pub use seed::{seeded_voice_provider_routing_packet, write_fixtures};

/// Record kind carried by [`VoiceProviderRoutingPacket`].
pub const VOICE_PROVIDER_ROUTING_PACKET_RECORD_KIND: &str = "voice_provider_routing_packet";

/// Record kind carried by a standalone [`VoiceRetentionExportControls`].
pub const VOICE_RETENTION_EXPORT_CONTROLS_RECORD_KIND: &str = "voice_retention_and_export_controls";

/// Schema version shared by the routing packet and its retention/export object.
pub const VOICE_PROVIDER_ROUTING_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the retention/export boundary schema.
pub const VOICE_RETENTION_EXPORT_SCHEMA_REF: &str =
    "schemas/voice/retention-and-export.schema.json";

/// Repo-relative path of the provider-descriptor boundary schema this lane reuses.
pub const VOICE_PROVIDER_DESCRIPTOR_SCHEMA_REF: &str =
    "schemas/voice/voice-provider-descriptor.schema.json";

/// Repo-relative path of the voice-session boundary schema this lane reuses.
pub const VOICE_SESSION_STATE_SCHEMA_REF: &str = "schemas/voice/voice-session.schema.json";

/// Repo-relative path of the privacy truth doc.
pub const VOICE_PROCESSING_AND_RETENTION_DOC_REF: &str =
    "docs/privacy/voice-processing-and-retention.md";

/// Repo-relative path of the checked-in support-export artifact.
pub const VOICE_PROVIDER_ROUTING_ARTIFACT_REF: &str =
    "artifacts/voice/voice-provider-routing/support_export.json";

/// Repo-relative directory of the checked-in routing fixtures.
pub const VOICE_PROVIDER_ROUTING_FIXTURES_DIR_REF: &str =
    "fixtures/voice/provider-locality-and-policy";

/// Privacy rank of a processing-locality cue. Higher is *more* private: an
/// on-device engine keeps audio local, a disclosed hosted engine is less
/// private, and an unavailable-processing state routes nothing at all.
///
/// The resolver uses this so a denial can only ever move the active locality
/// toward a strictly more-private (or equally private) engine — never toward a
/// more remote one.
pub const fn locality_privacy_rank(locality: ProcessingLocalityCue) -> u8 {
    match locality {
        ProcessingLocalityCue::LocalOnDevice => 2,
        ProcessingLocalityCue::HostedRemoteDisclosed => 1,
        ProcessingLocalityCue::ProcessingUnavailable => 0,
    }
}

/// Acoustic-profile class layered on top of a language pack. Distinguishes a
/// default model from environment- or speaker-adapted profiles so switching
/// profiles is explicit rather than silent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceAcousticProfileClass {
    /// The default acoustic model for the language pack.
    DefaultAcoustic,
    /// A noise-adapted acoustic profile.
    NoiseAdapted,
    /// An accent-adapted acoustic profile.
    AccentAdapted,
    /// A near-field/headset acoustic profile.
    HeadsetNearField,
}

impl VoiceAcousticProfileClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultAcoustic => "default_acoustic",
            Self::NoiseAdapted => "noise_adapted",
            Self::AccentAdapted => "accent_adapted",
            Self::HeadsetNearField => "headset_near_field",
        }
    }
}

/// Availability of a requested language pack. Drives whether a local provider can
/// serve the requested language profile or must fall back to its baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceLanguagePackAvailability {
    /// Bundled with the app and present on-device.
    BundledLocal,
    /// User-downloaded and present on-device.
    DownloadedLocal,
    /// Available to download but not yet present on-device.
    AvailableForDownload,
    /// Only available through a hosted provider.
    HostedOnly,
    /// Not available in the current envelope.
    Unavailable,
}

impl VoiceLanguagePackAvailability {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundledLocal => "bundled_local",
            Self::DownloadedLocal => "downloaded_local",
            Self::AvailableForDownload => "available_for_download",
            Self::HostedOnly => "hosted_only",
            Self::Unavailable => "unavailable",
        }
    }

    /// Whether the pack is present on-device and ready for local processing.
    pub const fn is_local_ready(self) -> bool {
        matches!(self, Self::BundledLocal | Self::DownloadedLocal)
    }
}

/// Entitlement state held for a requested provider/locality/language path. A
/// denied entitlement never silently routes to a broader or less private
/// provider; it blocks or downgrades to local.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceEntitlementState {
    /// No entitlement is required for this path.
    NotRequired,
    /// The required entitlement is held.
    Granted,
    /// An entitlement is required but not held; an explicit upgrade is offered.
    RequiresUpgrade,
    /// A previously held entitlement was revoked.
    Revoked,
    /// The entitlement could not be verified; fails closed.
    Unverified,
}

impl VoiceEntitlementState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Granted => "granted",
            Self::RequiresUpgrade => "requires_upgrade",
            Self::Revoked => "revoked",
            Self::Unverified => "unverified",
        }
    }

    /// Whether the entitlement permits this path.
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::NotRequired | Self::Granted)
    }
}

/// A requested or active language/acoustic profile. Switching profiles is
/// always explicit product state, never inferred.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceLanguageProfile {
    /// Language tag (BCP-47-style, e.g. `en-US`).
    pub language_tag: String,
    /// Acoustic-profile class layered on the language pack.
    pub acoustic_profile_class: VoiceAcousticProfileClass,
    /// Availability of the backing language pack.
    pub pack_availability: VoiceLanguagePackAvailability,
    /// Export-safe label shown when the profile is active.
    pub profile_label: String,
}

impl VoiceLanguageProfile {
    /// Whether the backing language pack is present on-device.
    pub fn is_local_ready(&self) -> bool {
        self.pack_availability.is_local_ready()
    }

    /// Whether the profile is structurally well-formed.
    pub fn is_well_formed(&self) -> bool {
        !self.language_tag.trim().is_empty() && !self.profile_label.trim().is_empty()
    }
}

/// Explicit transcript-retention and export control state for a provider path.
/// This is the user/admin-facing control object that the retention/export schema
/// describes: it pins the retention mode, the audio-retention class, and the
/// export posture, declares whether the user can change them, and carries a
/// precise disclosure label. Raw transcripts are excluded by default and any
/// support export is redacted first.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceRetentionExportControls {
    /// Record kind; must equal [`VOICE_RETENTION_EXPORT_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VOICE_PROVIDER_ROUTING_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Canonical retention mode.
    pub retention_mode: RetentionMode,
    /// Audio-retention class.
    pub audio_retention: AudioRetentionClass,
    /// Transcript-export posture.
    pub transcript_export: TranscriptExportPosture,
    /// Whether the user can change these controls in this context.
    pub user_changeable: bool,
    /// Whether raw transcripts are excluded from support/telemetry by default
    /// (must be true).
    pub raw_transcripts_excluded_by_default: bool,
    /// Whether transcripts are redacted before any support export.
    pub redaction_before_support_export: bool,
    /// Export-safe disclosure summary shown alongside the controls.
    pub disclosure_label: String,
}

impl VoiceRetentionExportControls {
    /// Builds a retention/export control object with the canonical record kind
    /// and schema version.
    pub fn new(
        retention_mode: RetentionMode,
        audio_retention: AudioRetentionClass,
        transcript_export: TranscriptExportPosture,
        user_changeable: bool,
        disclosure_label: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: VOICE_RETENTION_EXPORT_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: VOICE_PROVIDER_ROUTING_SCHEMA_VERSION,
            retention_mode,
            audio_retention,
            transcript_export,
            user_changeable,
            raw_transcripts_excluded_by_default: true,
            redaction_before_support_export: true,
            disclosure_label: disclosure_label.into(),
        }
    }

    /// Whether the controls keep all transcript/audio handling local-only.
    pub fn is_local_only(&self) -> bool {
        matches!(
            self.retention_mode,
            RetentionMode::NoAudioNoTranscriptRetained
                | RetentionMode::EphemeralAudioLocalOnlyNoTranscriptRetained
                | RetentionMode::TranscriptRetainedLocalOnly
        ) && !matches!(
            self.audio_retention,
            AudioRetentionClass::AudioRetainedProviderPerContract
        ) && !matches!(
            self.transcript_export,
            TranscriptExportPosture::ProviderContractRetained
        )
    }

    /// Whether raw transcripts stay out of support/telemetry by default.
    pub const fn raw_excluded(&self) -> bool {
        self.raw_transcripts_excluded_by_default
    }

    /// Whether the controls are internally consistent: raw transcripts excluded
    /// by default, and a support-bundle retention mode carries redaction.
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

    /// Whether the controls object is structurally well-formed.
    pub fn is_well_formed(&self) -> bool {
        self.record_kind == VOICE_RETENTION_EXPORT_CONTROLS_RECORD_KIND
            && self.schema_version == VOICE_PROVIDER_ROUTING_SCHEMA_VERSION
            && !self.disclosure_label.trim().is_empty()
            && self.raw_excluded()
            && self.posture_consistent()
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only object fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("voice retention/export controls serialize")
    }
}

/// A provider candidate the resolver can route to. Each candidate declares its
/// class, processing locality, transport, retention/export controls, the
/// language tags it can serve, the entitlement it requires, and whether it is
/// the designated on-device local-first default.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceProviderRoutingCandidate {
    /// Durable candidate (provider) id.
    pub candidate_id: String,
    /// Export-safe candidate label.
    pub candidate_label: String,
    /// Non-display fingerprint token; must differ from `candidate_id`.
    pub fingerprint_token: String,
    /// Provider class.
    pub provider_class: VoiceProviderClass,
    /// Processing-locality cue.
    pub processing_locality: ProcessingLocalityCue,
    /// Transport class.
    pub transport_class: VoiceTransportClass,
    /// Retention/export controls bound to this candidate.
    pub retention_export: VoiceRetentionExportControls,
    /// Language tags this candidate can serve.
    pub supported_language_tags: Vec<String>,
    /// The always-available baseline language tag for this candidate.
    pub baseline_language_tag: String,
    /// Whether routing to this candidate requires an entitlement.
    pub requires_entitlement: bool,
    /// Whether routing to this candidate requires a hosted-permitting policy.
    pub requires_hosted_policy: bool,
    /// Whether this candidate is the designated on-device local-first default.
    pub is_local_first_default: bool,
    /// Whether the candidate is currently available (not disabled/unavailable).
    pub available: bool,
    /// Whether a keyboard fallback remains complete via this candidate.
    pub keyboard_fallback_available: bool,
}

impl VoiceProviderRoutingCandidate {
    /// Whether the candidate is a usable on-device local engine.
    pub fn is_usable_local(&self) -> bool {
        self.available
            && self.processing_locality == ProcessingLocalityCue::LocalOnDevice
            && !self.provider_class.is_disabled()
    }

    /// Whether the candidate can serve the requested language profile: the tag is
    /// supported, and for a local engine the pack is present on-device (a hosted
    /// engine serves any supported tag server-side).
    pub fn serves_language(&self, profile: &VoiceLanguageProfile) -> bool {
        if !self
            .supported_language_tags
            .iter()
            .any(|tag| tag == &profile.language_tag)
        {
            return false;
        }
        if self.processing_locality == ProcessingLocalityCue::LocalOnDevice {
            profile.is_local_ready() || profile.language_tag == self.baseline_language_tag
        } else {
            true
        }
    }

    /// The candidate's baseline language profile, always available on-device.
    pub fn baseline_language_profile(&self) -> VoiceLanguageProfile {
        VoiceLanguageProfile {
            language_tag: self.baseline_language_tag.clone(),
            acoustic_profile_class: VoiceAcousticProfileClass::DefaultAcoustic,
            pack_availability: VoiceLanguagePackAvailability::BundledLocal,
            profile_label: format!("Baseline {} model", self.baseline_language_tag),
        }
    }

    /// Whether the candidate is structurally well-formed.
    pub fn is_well_formed(&self) -> bool {
        !self.candidate_id.trim().is_empty()
            && !self.candidate_label.trim().is_empty()
            && self.fingerprint_token.trim() != self.candidate_id.trim()
            && !self.fingerprint_token.trim().is_empty()
            && !self.baseline_language_tag.trim().is_empty()
            && self
                .supported_language_tags
                .iter()
                .any(|tag| tag == &self.baseline_language_tag)
            && self.retention_export.is_well_formed()
            && self.keyboard_fallback_available
    }
}

/// What the user/profile requested before resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceProviderRoutingRequest {
    /// Requested provider id; `None` requests the local-first default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_provider_id: Option<String>,
    /// Requested processing locality.
    pub requested_locality: ProcessingLocalityCue,
    /// Requested language/acoustic profile.
    pub requested_language_profile: VoiceLanguageProfile,
    /// Requested retention/export controls.
    pub requested_retention_export: VoiceRetentionExportControls,
    /// Active voice policy state.
    pub policy_state: VoicePolicyState,
    /// Whether org policy permits hosted/remote processing in this context.
    pub hosted_permitted_by_policy: bool,
    /// Entitlement state held for the requested path.
    pub entitlement_state: VoiceEntitlementState,
}

/// The resolved routing decision class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceRoutingDecision {
    /// The specifically requested provider was permitted and routed.
    RoutedAsRequested,
    /// No specific provider was requested; the local-first default was routed.
    RoutedLocalFirstDefault,
    /// The requested language pack was unavailable; the same provider's baseline
    /// language profile was routed instead.
    LanguageProfileDowngraded,
    /// The requested path was denied/unavailable; routing was held at a strictly
    /// more-private on-device provider, disclosed explicitly.
    DowngradedToMorePrivate,
    /// The requested path was denied/unavailable and no more-private fallback
    /// exists; voice is blocked explicitly and the keyboard path remains.
    BlockedExplicit,
}

impl VoiceRoutingDecision {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoutedAsRequested => "routed_as_requested",
            Self::RoutedLocalFirstDefault => "routed_local_first_default",
            Self::LanguageProfileDowngraded => "language_profile_downgraded",
            Self::DowngradedToMorePrivate => "downgraded_to_more_private",
            Self::BlockedExplicit => "blocked_explicit",
        }
    }

    /// Whether the decision routes voice to an active provider.
    pub const fn is_routed(self) -> bool {
        !matches!(self, Self::BlockedExplicit)
    }

    /// Whether the decision reflects a denial or unavailability.
    pub const fn is_denial(self) -> bool {
        matches!(
            self,
            Self::LanguageProfileDowngraded | Self::DowngradedToMorePrivate | Self::BlockedExplicit
        )
    }
}

/// Reason a routing outcome downgraded or blocked. The chrome quotes the precise
/// reason rather than a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceRoutingDenialReason {
    /// Policy blocks voice entirely in this context.
    PolicyBlocksVoice,
    /// Policy permits on-device processing only; hosted was not routed.
    PolicyRequiresLocalOnly,
    /// An entitlement upgrade is required and was not held.
    EntitlementRequiresUpgrade,
    /// A previously held entitlement was revoked.
    EntitlementRevoked,
    /// The entitlement could not be verified.
    EntitlementUnverified,
    /// The requested provider is unavailable.
    RequestedProviderUnavailable,
    /// The requested language pack is unavailable.
    LanguagePackUnavailable,
    /// No more-private on-device fallback exists, so voice is blocked.
    NoLocalFallbackAvailable,
}

impl VoiceRoutingDenialReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyBlocksVoice => "policy_blocks_voice",
            Self::PolicyRequiresLocalOnly => "policy_requires_local_only",
            Self::EntitlementRequiresUpgrade => "entitlement_requires_upgrade",
            Self::EntitlementRevoked => "entitlement_revoked",
            Self::EntitlementUnverified => "entitlement_unverified",
            Self::RequestedProviderUnavailable => "requested_provider_unavailable",
            Self::LanguagePackUnavailable => "language_pack_unavailable",
            Self::NoLocalFallbackAvailable => "no_local_fallback_available",
        }
    }
}

/// The resolved routing outcome: which provider and locality class are active,
/// the active language profile and retention/export controls, whether anything
/// changed from the request, and — on a denial — a precise reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceProviderRoutingOutcome {
    /// The routing decision class.
    pub decision: VoiceRoutingDecision,
    /// Active provider id; `None` only when blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_provider_id: Option<String>,
    /// Active provider class; `None` only when blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_provider_class: Option<VoiceProviderClass>,
    /// Active processing locality; `processing_unavailable` when blocked.
    pub active_locality: ProcessingLocalityCue,
    /// Active language profile; `None` only when blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_language_profile: Option<VoiceLanguageProfile>,
    /// Active retention/export controls; `None` only when blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_retention_export: Option<VoiceRetentionExportControls>,
    /// Reason for a downgrade/block; `None` for a clean routed outcome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason: Option<VoiceRoutingDenialReason>,
    /// Whether the active provider differs from the requested one.
    pub provider_changed_from_request: bool,
    /// Whether the active locality differs from the requested one.
    pub locality_changed_from_request: bool,
    /// Whether the active retention mode/audio class differs from the request.
    pub retention_changed_from_request: bool,
    /// Whether the active export posture differs from the request.
    pub export_changed_from_request: bool,
    /// Whether the active language profile differs from the requested one.
    pub language_changed_from_request: bool,
    /// Precise, non-generic disclosure label for the outcome.
    pub disclosure_label: String,
    /// Whether a keyboard fallback remains available (must be true).
    pub keyboard_fallback_available: bool,
}

impl VoiceProviderRoutingOutcome {
    /// Whether the outcome never widens privacy: a routed/downgraded outcome's
    /// active locality is at least as private as the requested locality, and a
    /// blocked outcome routes nothing.
    pub fn privacy_never_widened(&self, request: &VoiceProviderRoutingRequest) -> bool {
        if self.decision == VoiceRoutingDecision::BlockedExplicit {
            return self.active_provider_id.is_none()
                && self.active_locality == ProcessingLocalityCue::ProcessingUnavailable;
        }
        locality_privacy_rank(self.active_locality)
            >= locality_privacy_rank(request.requested_locality)
    }

    /// Whether a denial outcome resolves to a strictly more-private engine or a
    /// block — never a broader/less private provider. A clean routed outcome is
    /// exempt (it routed exactly what was requested).
    pub fn denial_never_widens(&self, request: &VoiceProviderRoutingRequest) -> bool {
        match self.decision {
            VoiceRoutingDecision::DowngradedToMorePrivate => {
                self.active_locality == ProcessingLocalityCue::LocalOnDevice
                    && locality_privacy_rank(self.active_locality)
                        >= locality_privacy_rank(request.requested_locality)
            }
            VoiceRoutingDecision::BlockedExplicit => self.active_provider_id.is_none(),
            _ => true,
        }
    }

    /// Whether the outcome is structurally well-formed for its decision class.
    pub fn is_well_formed(&self) -> bool {
        if !self.keyboard_fallback_available || self.disclosure_label.trim().is_empty() {
            return false;
        }
        match self.decision {
            VoiceRoutingDecision::BlockedExplicit => {
                self.active_provider_id.is_none()
                    && self.active_provider_class.is_none()
                    && self.active_language_profile.is_none()
                    && self.active_retention_export.is_none()
                    && self.active_locality == ProcessingLocalityCue::ProcessingUnavailable
                    && self.denial_reason.is_some()
            }
            _ => {
                let active_ok = self.active_provider_id.is_some()
                    && self.active_provider_class.is_some()
                    && self
                        .active_language_profile
                        .as_ref()
                        .is_some_and(VoiceLanguageProfile::is_well_formed)
                    && self
                        .active_retention_export
                        .as_ref()
                        .is_some_and(VoiceRetentionExportControls::is_well_formed)
                    && self.active_locality != ProcessingLocalityCue::ProcessingUnavailable;
                let denial_ok = if self.decision.is_denial() {
                    self.denial_reason.is_some()
                } else {
                    self.denial_reason.is_none()
                };
                active_ok && denial_ok
            }
        }
    }
}

/// Whether a disclosure label is a generic non-answer rather than a precise one.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    matches!(
        trimmed.to_lowercase().as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "blocked"
            | "downgraded"
    )
}

/// Resolves a voice routing request against the available candidates.
///
/// The resolution is deterministic and privacy-first: a denial never falls back
/// to a broader or less private provider. It blocks explicitly or downgrades to a
/// strictly more-private on-device default, and it records every retention,
/// export, provider, locality, and language change relative to the request.
pub fn resolve_voice_routing(
    request: &VoiceProviderRoutingRequest,
    candidates: &[VoiceProviderRoutingCandidate],
) -> VoiceProviderRoutingOutcome {
    // 1. Policy can block voice entirely.
    if request.policy_state == VoicePolicyState::PolicyBlocked {
        return blocked_outcome(
            VoiceRoutingDenialReason::PolicyBlocksVoice,
            "Voice is blocked by policy in this context; the keyboard and command palette remain available",
        );
    }

    // 2. Locate the requested candidate (or the local-first default).
    let requested = match &request.requested_provider_id {
        Some(id) => candidates.iter().find(|c| &c.candidate_id == id),
        None => candidates.iter().find(|c| c.is_local_first_default),
    };
    let specific = request.requested_provider_id.is_some();

    let Some(candidate) = requested else {
        return downgrade_or_block(
            request,
            candidates,
            None,
            VoiceRoutingDenialReason::RequestedProviderUnavailable,
        );
    };

    // 3. Gate checks, in priority order.
    if !candidate.available
        || candidate.provider_class.is_disabled()
        || candidate.processing_locality == ProcessingLocalityCue::ProcessingUnavailable
    {
        return downgrade_or_block(
            request,
            candidates,
            Some(&candidate.candidate_id),
            VoiceRoutingDenialReason::RequestedProviderUnavailable,
        );
    }
    if candidate.requires_hosted_policy && !request.hosted_permitted_by_policy {
        return downgrade_or_block(
            request,
            candidates,
            Some(&candidate.candidate_id),
            VoiceRoutingDenialReason::PolicyRequiresLocalOnly,
        );
    }
    if candidate.requires_entitlement && !request.entitlement_state.is_satisfied() {
        let reason = match request.entitlement_state {
            VoiceEntitlementState::Revoked => VoiceRoutingDenialReason::EntitlementRevoked,
            VoiceEntitlementState::Unverified => VoiceRoutingDenialReason::EntitlementUnverified,
            _ => VoiceRoutingDenialReason::EntitlementRequiresUpgrade,
        };
        return downgrade_or_block(request, candidates, Some(&candidate.candidate_id), reason);
    }
    if !candidate.serves_language(&request.requested_language_profile) {
        // The candidate can still serve its baseline on-device; otherwise fall
        // back to a more-private default or block.
        if candidate
            .supported_language_tags
            .iter()
            .any(|tag| tag == &candidate.baseline_language_tag)
        {
            return language_downgraded_outcome(request, candidate);
        }
        return downgrade_or_block(
            request,
            candidates,
            Some(&candidate.candidate_id),
            VoiceRoutingDenialReason::LanguagePackUnavailable,
        );
    }

    // 4. All gates pass: route the requested candidate.
    routed_outcome(request, candidate, specific)
}

fn blocked_outcome(reason: VoiceRoutingDenialReason, label: &str) -> VoiceProviderRoutingOutcome {
    VoiceProviderRoutingOutcome {
        decision: VoiceRoutingDecision::BlockedExplicit,
        active_provider_id: None,
        active_provider_class: None,
        active_locality: ProcessingLocalityCue::ProcessingUnavailable,
        active_language_profile: None,
        active_retention_export: None,
        denial_reason: Some(reason),
        provider_changed_from_request: true,
        locality_changed_from_request: true,
        retention_changed_from_request: false,
        export_changed_from_request: false,
        language_changed_from_request: false,
        disclosure_label: label.to_owned(),
        keyboard_fallback_available: true,
    }
}

fn routed_outcome(
    request: &VoiceProviderRoutingRequest,
    candidate: &VoiceProviderRoutingCandidate,
    specific: bool,
) -> VoiceProviderRoutingOutcome {
    let decision = if specific {
        VoiceRoutingDecision::RoutedAsRequested
    } else {
        VoiceRoutingDecision::RoutedLocalFirstDefault
    };
    let label = if specific {
        format!(
            "Routing voice to {} ({} processing); retention and export are disclosed before capture",
            candidate.candidate_label,
            candidate.processing_locality.as_str()
        )
    } else {
        format!(
            "Voice defaults to the on-device {} ({} processing); no audio leaves the device",
            candidate.candidate_label,
            candidate.processing_locality.as_str()
        )
    };
    build_active_outcome(
        request,
        candidate,
        request.requested_language_profile.clone(),
        decision,
        None,
        label,
    )
}

fn language_downgraded_outcome(
    request: &VoiceProviderRoutingRequest,
    candidate: &VoiceProviderRoutingCandidate,
) -> VoiceProviderRoutingOutcome {
    let baseline = candidate.baseline_language_profile();
    let label = format!(
        "Requested language pack '{}' is unavailable; voice continues on {} with the baseline '{}' profile",
        request.requested_language_profile.language_tag,
        candidate.candidate_label,
        baseline.language_tag
    );
    build_active_outcome(
        request,
        candidate,
        baseline,
        VoiceRoutingDecision::LanguageProfileDowngraded,
        Some(VoiceRoutingDenialReason::LanguagePackUnavailable),
        label,
    )
}

/// Resolves a denied/unavailable request: downgrade to a strictly more-private
/// on-device default when one exists, otherwise block explicitly. Never routes
/// to a broader or less private provider.
fn downgrade_or_block(
    request: &VoiceProviderRoutingRequest,
    candidates: &[VoiceProviderRoutingCandidate],
    exclude_id: Option<&str>,
    reason: VoiceRoutingDenialReason,
) -> VoiceProviderRoutingOutcome {
    let fallback = candidates.iter().find(|c| {
        c.is_local_first_default
            && c.is_usable_local()
            && !c.requires_entitlement
            && !c.requires_hosted_policy
            && exclude_id != Some(c.candidate_id.as_str())
    });

    match fallback {
        Some(local)
            if locality_privacy_rank(local.processing_locality)
                >= locality_privacy_rank(request.requested_locality) =>
        {
            // Serve the requested profile if the local default can; otherwise its
            // baseline. Either way the fallback is strictly more private.
            let language = if local.serves_language(&request.requested_language_profile) {
                request.requested_language_profile.clone()
            } else {
                local.baseline_language_profile()
            };
            let label = format!(
                "{}; voice was held on the on-device {} instead of a broader provider",
                denial_reason_phrase(reason),
                local.candidate_label
            );
            build_active_outcome(
                request,
                local,
                language,
                VoiceRoutingDecision::DowngradedToMorePrivate,
                Some(reason),
                label,
            )
        }
        _ => {
            let label = format!(
                "{}; no on-device fallback is available, so voice is blocked and the keyboard path remains",
                denial_reason_phrase(reason)
            );
            blocked_outcome(reason, &label)
        }
    }
}

fn denial_reason_phrase(reason: VoiceRoutingDenialReason) -> &'static str {
    match reason {
        VoiceRoutingDenialReason::PolicyBlocksVoice => "Voice is blocked by policy",
        VoiceRoutingDenialReason::PolicyRequiresLocalOnly => {
            "Policy permits on-device processing only"
        }
        VoiceRoutingDenialReason::EntitlementRequiresUpgrade => {
            "The requested provider needs an entitlement upgrade"
        }
        VoiceRoutingDenialReason::EntitlementRevoked => {
            "The entitlement for the requested provider was revoked"
        }
        VoiceRoutingDenialReason::EntitlementUnverified => {
            "The entitlement for the requested provider could not be verified"
        }
        VoiceRoutingDenialReason::RequestedProviderUnavailable => {
            "The requested speech provider is unavailable"
        }
        VoiceRoutingDenialReason::LanguagePackUnavailable => {
            "The requested language pack is unavailable"
        }
        VoiceRoutingDenialReason::NoLocalFallbackAvailable => "No on-device fallback is available",
    }
}

fn build_active_outcome(
    request: &VoiceProviderRoutingRequest,
    candidate: &VoiceProviderRoutingCandidate,
    language: VoiceLanguageProfile,
    decision: VoiceRoutingDecision,
    denial_reason: Option<VoiceRoutingDenialReason>,
    disclosure_label: String,
) -> VoiceProviderRoutingOutcome {
    let active = &candidate.retention_export;
    let requested = &request.requested_retention_export;
    let provider_changed = request
        .requested_provider_id
        .as_deref()
        .is_some_and(|id| id != candidate.candidate_id);
    let retention_changed = active.retention_mode != requested.retention_mode
        || active.audio_retention != requested.audio_retention;
    let export_changed = active.transcript_export != requested.transcript_export;
    let language_changed = language.language_tag != request.requested_language_profile.language_tag
        || language.acoustic_profile_class
            != request.requested_language_profile.acoustic_profile_class;

    VoiceProviderRoutingOutcome {
        decision,
        active_provider_id: Some(candidate.candidate_id.clone()),
        active_provider_class: Some(candidate.provider_class),
        active_locality: candidate.processing_locality,
        active_language_profile: Some(language),
        active_retention_export: Some(active.clone()),
        denial_reason,
        provider_changed_from_request: provider_changed,
        locality_changed_from_request: candidate.processing_locality != request.requested_locality,
        retention_changed_from_request: retention_changed,
        export_changed_from_request: export_changed,
        language_changed_from_request: language_changed,
        disclosure_label,
        keyboard_fallback_available: true,
    }
}

/// One resolved routing scenario row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceProviderRoutingRow {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Export-safe scenario label.
    pub scenario_label: String,
    /// Non-display fingerprint token; must differ from `scenario_id`.
    pub fingerprint_token: String,
    /// Claim posture of the scenario.
    pub claim_posture: VoiceClaimPosture,
    /// The routing request.
    pub request: VoiceProviderRoutingRequest,
    /// The available candidates considered by the resolver.
    pub candidates: Vec<VoiceProviderRoutingCandidate>,
    /// The recorded routing outcome (must equal the resolver's output).
    pub outcome: VoiceProviderRoutingOutcome,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl VoiceProviderRoutingRow {
    /// Re-derives the outcome from the recorded inputs.
    pub fn recompute_outcome(&self) -> VoiceProviderRoutingOutcome {
        resolve_voice_routing(&self.request, &self.candidates)
    }

    /// Whether the recorded outcome matches a fresh resolution of the inputs.
    pub fn outcome_is_honest(&self) -> bool {
        self.recompute_outcome() == self.outcome
    }

    /// Whether the fingerprint is a real non-display basis distinct from the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.fingerprint_token.trim();
        !token.is_empty() && token != self.scenario_id.trim()
    }

    /// Whether the row is complete and its invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.scenario_id.trim().is_empty()
            && !self.scenario_label.trim().is_empty()
            && self.fingerprint_independent_of_id()
            && !self.candidates.is_empty()
            && self
                .candidates
                .iter()
                .all(VoiceProviderRoutingCandidate::is_well_formed)
            && self.request.requested_language_profile.is_well_formed()
            && self.request.requested_retention_export.is_well_formed()
            && self.outcome.is_well_formed()
            && self.outcome_is_honest()
            && self.outcome.privacy_never_widened(&self.request)
            && self.outcome.denial_never_widens(&self.request)
            && !label_is_generic(&self.outcome.disclosure_label)
            && !self.source_contract_refs.is_empty()
            && self
                .source_contract_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceProviderRoutingGuardrails {
    /// Provider and locality class are always inspectable on a routed outcome.
    pub provider_and_locality_inspectable: bool,
    /// Local-first defaults stay visible and disclosed.
    pub local_first_default_visible: bool,
    /// Switching provider or language never hides a retention or export change.
    pub switching_never_hides_retention_or_export: bool,
    /// Policy/entitlement denials block explicitly instead of widening.
    pub denials_block_instead_of_widening: bool,
    /// A denial never falls back to a broader or less private provider.
    pub no_silent_fallback_to_less_private: bool,
    /// Audio/transcript never route outside the declared locality/retention model.
    pub audio_transcript_never_leave_declared_model: bool,
    /// A keyboard fallback is always available; voice is never a dead end.
    pub keyboard_fallback_always_available: bool,
}

impl VoiceProviderRoutingGuardrails {
    /// Whether every guardrail invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.provider_and_locality_inspectable
            && self.local_first_default_visible
            && self.switching_never_hides_retention_or_export
            && self.denials_block_instead_of_widening
            && self.no_silent_fallback_to_less_private
            && self.audio_transcript_never_leave_declared_model
            && self.keyboard_fallback_always_available
    }
}

/// Consumer projection block: the surfaces that read this routing truth instead
/// of cloning provider/locality/retention text by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceProviderRoutingConsumerProjection {
    /// Settings/provider surfaces ingest the routing truth.
    pub settings_ingests_routing: bool,
    /// Admin/policy surfaces ingest the routing truth.
    pub admin_ingests_routing: bool,
    /// Diagnostics surfaces ingest the routing truth.
    pub diagnostics_ingests_routing: bool,
    /// Support-export surfaces ingest the routing truth.
    pub support_export_ingests_routing: bool,
    /// Active provider and locality are shown without a settings dive.
    pub active_provider_visible_without_settings_dive: bool,
}

impl VoiceProviderRoutingConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.settings_ingests_routing
            && self.admin_ingests_routing
            && self.diagnostics_ingests_routing
            && self.support_export_ingests_routing
            && self.active_provider_visible_without_settings_dive
    }
}

/// Constructor input for [`VoiceProviderRoutingPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceProviderRoutingPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Export-safe packet label.
    pub label: String,
    /// Per-scenario rows.
    pub rows: Vec<VoiceProviderRoutingRow>,
    /// Guardrail invariants block.
    pub guardrails: VoiceProviderRoutingGuardrails,
    /// Consumer projection block.
    pub consumer_projection: VoiceProviderRoutingConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe voice provider routing / privacy-gating packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceProviderRoutingPacket {
    /// Record kind; must equal [`VOICE_PROVIDER_ROUTING_PACKET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VOICE_PROVIDER_ROUTING_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Export-safe packet label.
    pub label: String,
    /// Per-scenario rows.
    pub rows: Vec<VoiceProviderRoutingRow>,
    /// Guardrail invariants block.
    pub guardrails: VoiceProviderRoutingGuardrails,
    /// Consumer projection block.
    pub consumer_projection: VoiceProviderRoutingConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl VoiceProviderRoutingPacket {
    /// Builds a voice provider routing packet.
    pub fn new(input: VoiceProviderRoutingPacketInput) -> Self {
        Self {
            record_kind: VOICE_PROVIDER_ROUTING_PACKET_RECORD_KIND.to_owned(),
            schema_version: VOICE_PROVIDER_ROUTING_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Resolves a row by its scenario id.
    pub fn row(&self, scenario_id: &str) -> Option<&VoiceProviderRoutingRow> {
        self.rows.iter().find(|row| row.scenario_id == scenario_id)
    }

    /// Decisions represented across rows.
    pub fn represented_decisions(&self) -> BTreeSet<VoiceRoutingDecision> {
        self.rows.iter().map(|row| row.outcome.decision).collect()
    }

    /// Denial reasons represented across rows.
    pub fn represented_denial_reasons(&self) -> BTreeSet<VoiceRoutingDenialReason> {
        self.rows
            .iter()
            .filter_map(|row| row.outcome.denial_reason)
            .collect()
    }

    /// Count of rows whose outcome routed to an active provider.
    pub fn routed_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.outcome.decision.is_routed())
            .count()
    }

    /// Count of rows whose outcome blocked voice explicitly.
    pub fn blocked_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.outcome.decision == VoiceRoutingDecision::BlockedExplicit)
            .count()
    }

    /// Validates the routing packet invariants.
    pub fn validate(&self) -> Vec<VoiceRoutingViolation> {
        let mut violations = Vec::new();

        if self.record_kind != VOICE_PROVIDER_ROUTING_PACKET_RECORD_KIND {
            violations.push(VoiceRoutingViolation::WrongRecordKind);
        }
        if self.schema_version != VOICE_PROVIDER_ROUTING_SCHEMA_VERSION {
            violations.push(VoiceRoutingViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(VoiceRoutingViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(VoiceRoutingViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(VoiceRoutingViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("voice routing packet serializes"),
        ) {
            violations.push(VoiceRoutingViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("voice routing packet serializes")
    }

    /// Compact one-line-per-row summary for diagnostics and support handoff.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(self.rows.len() + 1);
        lines.push(format!(
            "{} | rows={} | routed={} | blocked={} | invariants_ok={}",
            self.packet_id,
            self.rows.len(),
            self.routed_row_count(),
            self.blocked_row_count(),
            self.validate().is_empty()
        ));
        for row in &self.rows {
            let locality = row
                .outcome
                .active_provider_id
                .as_deref()
                .map_or("none", |_| row.outcome.active_locality.as_str());
            lines.push(format!(
                "{} | decision={} | active_provider={} | locality={} | denial={} | retention_changed={} | export_changed={}",
                row.scenario_id,
                row.outcome.decision.as_str(),
                row.outcome.active_provider_id.as_deref().unwrap_or("none"),
                locality,
                row.outcome
                    .denial_reason
                    .map_or("none", VoiceRoutingDenialReason::as_str),
                row.outcome.retention_changed_from_request,
                row.outcome.export_changed_from_request,
            ));
        }
        lines
    }

    /// Deterministic Markdown summary for docs, support, or release handoff.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Voice Provider Routing & Privacy-Gating\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Rows: {} ({} routed, {} blocked)\n",
            self.rows.len(),
            self.routed_row_count(),
            self.blocked_row_count()
        ));
        out.push_str("\n## Scenarios\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): `{}`\n",
                row.scenario_id,
                row.claim_posture.as_str(),
                row.outcome.decision.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.scenario_label));
            out.push_str(&format!(
                "  - requested locality = `{}`, active locality = `{}`\n",
                row.request.requested_locality.as_str(),
                row.outcome.active_locality.as_str()
            ));
            out.push_str(&format!(
                "  - active provider = `{}`, language = `{}`\n",
                row.outcome.active_provider_id.as_deref().unwrap_or("none"),
                row.outcome
                    .active_language_profile
                    .as_ref()
                    .map_or("none", |p| p.language_tag.as_str())
            ));
            out.push_str(&format!(
                "  - retention changed = {}, export changed = {}, provider changed = {}, language changed = {}\n",
                row.outcome.retention_changed_from_request,
                row.outcome.export_changed_from_request,
                row.outcome.provider_changed_from_request,
                row.outcome.language_changed_from_request,
            ));
            if let Some(reason) = row.outcome.denial_reason {
                out.push_str(&format!("  - denial reason = `{}`\n", reason.as_str()));
            }
            out.push_str(&format!(
                "  - disclosure: {}\n",
                row.outcome.disclosure_label
            ));
        }
        out
    }
}

/// Errors reading the checked-in routing artifact.
#[derive(Debug)]
pub enum VoiceProviderRoutingArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<VoiceRoutingViolation>),
}

impl fmt::Display for VoiceProviderRoutingArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "voice routing export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "voice routing export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for VoiceProviderRoutingArtifactError {}

/// Validation failures emitted by [`VoiceProviderRoutingPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoiceRoutingViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// A required base source contract ref is missing.
    MissingSourceContracts,
    /// A row is incomplete.
    RowIncomplete,
    /// A recorded outcome does not match a fresh resolution of its inputs.
    OutcomeDoesNotMatchResolver,
    /// A denial/downgrade routed to a broader or less private provider.
    DenialWidenedPrivacy,
    /// A routed/downgraded outcome reduced privacy relative to the request.
    RoutingReducedPrivacy,
    /// A row dropped the keyboard fallback.
    KeyboardFallbackMissing,
    /// A row retained raw transcripts in support/telemetry by default.
    RawTranscriptRetainedByDefault,
    /// A downgrade/block carries a generic, imprecise disclosure label.
    DisclosureLabelGeneric,
    /// A scenario fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// No local-first default routed-as-default case is present.
    LocalFirstDefaultCaseMissing,
    /// No routed hosted case discloses a retention/export change on switch.
    HostedSwitchDisclosureCaseMissing,
    /// No language-profile switch case is present.
    LanguageSwitchCaseMissing,
    /// No policy-blocked explicit-block case is present.
    PolicyBlockedCaseMissing,
    /// No entitlement-denied case is present.
    EntitlementDeniedCaseMissing,
    /// No provider-unavailable downgrade/block case is present.
    ProviderUnavailableCaseMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl VoiceRoutingViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RowIncomplete => "row_incomplete",
            Self::OutcomeDoesNotMatchResolver => "outcome_does_not_match_resolver",
            Self::DenialWidenedPrivacy => "denial_widened_privacy",
            Self::RoutingReducedPrivacy => "routing_reduced_privacy",
            Self::KeyboardFallbackMissing => "keyboard_fallback_missing",
            Self::RawTranscriptRetainedByDefault => "raw_transcript_retained_by_default",
            Self::DisclosureLabelGeneric => "disclosure_label_generic",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::LocalFirstDefaultCaseMissing => "local_first_default_case_missing",
            Self::HostedSwitchDisclosureCaseMissing => "hosted_switch_disclosure_case_missing",
            Self::LanguageSwitchCaseMissing => "language_switch_case_missing",
            Self::PolicyBlockedCaseMissing => "policy_blocked_case_missing",
            Self::EntitlementDeniedCaseMissing => "entitlement_denied_case_missing",
            Self::ProviderUnavailableCaseMissing => "provider_unavailable_case_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable routing artifact.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_voice_provider_routing_export(
) -> Result<VoiceProviderRoutingPacket, VoiceProviderRoutingArtifactError> {
    let packet: VoiceProviderRoutingPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/voice/voice-provider-routing/support_export.json"
    )))
    .map_err(VoiceProviderRoutingArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(VoiceProviderRoutingArtifactError::Validation(violations))
    }
}

/// Serializes a value as pretty JSON with a trailing newline (on-disk form).
pub fn fixture_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let mut json = serde_json::to_string_pretty(value)?;
    json.push('\n');
    Ok(json)
}

/// Writes the checked-in support-export artifact to `path`.
pub fn write_support_export(path: &Path, packet: &VoiceProviderRoutingPacket) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = fixture_json(packet).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    fs::write(path, json)
}

fn validate_source_contracts(
    packet: &VoiceProviderRoutingPacket,
    violations: &mut Vec<VoiceRoutingViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        VOICE_RETENTION_EXPORT_SCHEMA_REF,
        VOICE_PROVIDER_DESCRIPTOR_SCHEMA_REF,
        VOICE_SESSION_STATE_SCHEMA_REF,
        VOICE_PROCESSING_AND_RETENTION_DOC_REF,
        VOICE_PROVIDER_ROUTING_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(VoiceRoutingViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &VoiceProviderRoutingPacket,
    violations: &mut Vec<VoiceRoutingViolation>,
) {
    // A local-first default routed cleanly with no audio leaving the device.
    if !packet.rows.iter().any(|row| {
        row.outcome.decision == VoiceRoutingDecision::RoutedLocalFirstDefault
            && row.outcome.active_locality == ProcessingLocalityCue::LocalOnDevice
    }) {
        violations.push(VoiceRoutingViolation::LocalFirstDefaultCaseMissing);
    }

    // A hosted switch that surfaces a retention/export change (criterion 2).
    if !packet.rows.iter().any(|row| {
        row.outcome.active_locality == ProcessingLocalityCue::HostedRemoteDisclosed
            && row.outcome.decision.is_routed()
            && (row.outcome.retention_changed_from_request
                || row.outcome.export_changed_from_request)
    }) {
        violations.push(VoiceRoutingViolation::HostedSwitchDisclosureCaseMissing);
    }

    // A language-profile switch case (criterion 2/requirement 3).
    if !packet.rows.iter().any(|row| {
        row.outcome.language_changed_from_request
            || row.outcome.decision == VoiceRoutingDecision::LanguageProfileDowngraded
    }) {
        violations.push(VoiceRoutingViolation::LanguageSwitchCaseMissing);
    }

    // A policy-blocked explicit-block case (criterion 3).
    if !packet.rows.iter().any(|row| {
        row.outcome.decision == VoiceRoutingDecision::BlockedExplicit
            && row.outcome.denial_reason == Some(VoiceRoutingDenialReason::PolicyBlocksVoice)
    }) {
        violations.push(VoiceRoutingViolation::PolicyBlockedCaseMissing);
    }

    // An entitlement-denied case that blocks or downgrades to local (criterion 3).
    if !packet.rows.iter().any(|row| {
        matches!(
            row.outcome.denial_reason,
            Some(VoiceRoutingDenialReason::EntitlementRequiresUpgrade)
                | Some(VoiceRoutingDenialReason::EntitlementRevoked)
                | Some(VoiceRoutingDenialReason::EntitlementUnverified)
        )
    }) {
        violations.push(VoiceRoutingViolation::EntitlementDeniedCaseMissing);
    }

    // A provider-unavailable downgrade/block case (requirement 3).
    if !packet.rows.iter().any(|row| {
        row.outcome.denial_reason == Some(VoiceRoutingDenialReason::RequestedProviderUnavailable)
    }) {
        violations.push(VoiceRoutingViolation::ProviderUnavailableCaseMissing);
    }
}

fn validate_rows(packet: &VoiceProviderRoutingPacket, violations: &mut Vec<VoiceRoutingViolation>) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(VoiceRoutingViolation::RowIncomplete);
        }
        if !row.outcome_is_honest() {
            violations.push(VoiceRoutingViolation::OutcomeDoesNotMatchResolver);
        }
        if !row.outcome.denial_never_widens(&row.request) {
            violations.push(VoiceRoutingViolation::DenialWidenedPrivacy);
        }
        if !row.outcome.privacy_never_widened(&row.request) {
            violations.push(VoiceRoutingViolation::RoutingReducedPrivacy);
        }
        if !row.outcome.keyboard_fallback_available {
            violations.push(VoiceRoutingViolation::KeyboardFallbackMissing);
        }
        if let Some(controls) = &row.outcome.active_retention_export {
            if !controls.raw_excluded() {
                violations.push(VoiceRoutingViolation::RawTranscriptRetainedByDefault);
            }
        }
        for candidate in &row.candidates {
            if !candidate.retention_export.raw_excluded() {
                violations.push(VoiceRoutingViolation::RawTranscriptRetainedByDefault);
            }
        }
        if row.outcome.decision.is_denial() && label_is_generic(&row.outcome.disclosure_label) {
            violations.push(VoiceRoutingViolation::DisclosureLabelGeneric);
        }
        if !row.fingerprint_independent_of_id() {
            violations.push(VoiceRoutingViolation::FingerprintSubstitutesIdentity);
        }
    }
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
