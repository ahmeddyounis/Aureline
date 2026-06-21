//! Explicit voice command-mode / dictation-mode shell-state surface.
//!
//! Voice is an explicit, privacy-bounded input mode in Aureline. This module
//! owns the *always-visible shell state* a user reads to answer one question at
//! a glance: **is Aureline in command mode, dictation mode, idle, or blocked,
//! and where is my speech being processed?** It is the legible-shell complement
//! to the two existing voice lanes:
//!
//! - [`crate::voice`] models the bounded M3 preview/beta surface (transcript
//!   strips, disambiguation sheets, per-command resolution proof); and
//! - [`crate::freeze_the_m5_voice_mode_provider_transcript_retention_and_command_parity_matrix`]
//!   freezes the M5 provider / retention / command-parity *qualification*
//!   matrix.
//!
//! This lane does not mint a second interaction model. It reuses the canonical
//! mode, activation, mic-indicator, processing-locality, retention, and policy
//! vocabulary from those lanes and projects it into the persistent shell
//! affordances the spec requires:
//!
//! - a [`VoiceModeStrip`] that keeps command mode and dictation mode separate
//!   and visible at all times (never one collapsing silently into the other);
//! - a [`PushToTalkControl`] that makes push-to-talk (or an equivalent
//!   explicit activation) the default and refuses to let capture become
//!   silently continuous;
//! - a [`VoiceMicIndicator`] (mic-state pill) that stays visible whenever
//!   capture is active and always carries the active mode and the
//!   local-versus-hosted processing cue;
//! - a [`ProviderLocalityDisclosure`] that surfaces provider / local-engine
//!   identity, processing locality, and retention posture inline — without a
//!   deep settings dive; and
//! - a [`VoiceRecoveryAffordance`] that, whenever voice is unavailable or
//!   policy-blocked, surfaces an immediate keyboard-first recovery rather than
//!   a dead end.
//!
//! Each [`VoiceShellStateRow`] carries exactly one
//! [`VoiceShellLifecycleState`] drawn from a controlled lifecycle vocabulary —
//! idle, listening, processing, needs confirmation, unavailable, policy blocked
//! — never provider-specific prose. The top-level [`VoiceShellStatePacket`] is
//! the inspectable truth packet consumed by the live shell, Help/About,
//! diagnostics, and metadata-only support export.
//!
//! [`VoiceShellStatePacket::validate`] refuses any packet that lets the mode
//! become implicit, hides the mic indicator while capturing, defaults to
//! continuous listening without an explicit opt-in, buries provider/locality
//! behind a settings dive, or leaves a blocked/unavailable row without a
//! keyboard-first recovery. Raw audio bytes, raw transcript text, raw provider
//! payloads, private paths, and credentials never cross this boundary; the
//! packet carries only typed class tokens, booleans, opaque ids, and
//! redaction-aware label refs.
//!
//! The seed in [`seed`] is the single mint-from-truth source for the checked-in
//! fixtures under [`VOICE_SHELL_STATE_FIXTURES_DIR_REF`] and the published
//! companion doc [`VOICE_SHELL_STATE_DOC_REF`].

#[cfg(test)]
mod tests;

pub mod seed;

use std::path::Path;
use std::{fs, io};

use serde::{Deserialize, Serialize};

pub use crate::freeze_the_m5_voice_mode_provider_transcript_retention_and_command_parity_matrix::VoicePolicyState;
pub use crate::voice::{
    BackgroundListeningState, MicIndicatorClass, ProcessingLocalityCue, RetentionMode,
    VoiceActivationClass, VoiceClaimPosture, VoiceModeClass, VoiceUnavailableReason,
};

pub use seed::seeded_voice_shell_state_packet;

/// Schema version exported with every voice-shell-state record.
pub const VOICE_SHELL_STATE_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref quoted by every voice-shell-state record.
pub const VOICE_SHELL_STATE_SHARED_CONTRACT_REF: &str = "shell:voice_shell_state:v1";

/// Stable record kind for [`VoiceShellStatePacket`] payloads.
pub const VOICE_SHELL_STATE_PACKET_RECORD_KIND: &str = "shell_voice_shell_state_packet_record";

/// Stable record kind for [`VoiceShellStateRow`] payloads.
pub const VOICE_SHELL_STATE_ROW_RECORD_KIND: &str = "shell_voice_shell_state_row_record";

/// Stable packet id quoted across surfaces.
pub const VOICE_SHELL_STATE_PACKET_ID: &str = "shell:voice_shell_state:packet:v1";

/// Repo-relative path of the published companion doc.
pub const VOICE_SHELL_STATE_DOC_REF: &str = "docs/ux/voice-shell-states.md";

/// Repo-relative directory of the checked-in mint-from-truth fixtures.
pub const VOICE_SHELL_STATE_FIXTURES_DIR_REF: &str = "fixtures/voice/mode-and-mic-state";

/// Repo-relative path of the cross-surface voice / dictation contract this lane
/// keeps legible.
pub const VOICE_AND_DICTATION_CONTRACT_REF: &str = "docs/ux/voice_and_dictation_contract.md";

/// Repo-relative path of the M5 voice-qualification matrix summary whose
/// provider / retention / parity truth this lane surfaces.
pub const VOICE_QUALIFICATION_MATRIX_REF: &str = "artifacts/voice/m5-voice-qualification-matrix.md";

/// Redaction class stamped on every record; the packet carries metadata only.
pub const REDACTION_CLASS: &str = "metadata_safe_default";

/// Controlled lifecycle vocabulary for the voice shell state.
///
/// Surfaces project these states verbatim — they never substitute
/// provider-specific prose for one of these tokens. The vocabulary is
/// deliberately small so a user can always tell, at a glance, whether Aureline
/// is capturing, deciding, waiting on them, or stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceShellLifecycleState {
    /// Microphone is off; nothing is being captured.
    Idle,
    /// Capture is active and audio is being received.
    Listening,
    /// Capture finished; the utterance is being recognized/resolved.
    Processing,
    /// A resolved high-impact command is waiting on explicit confirmation
    /// before it runs.
    NeedsConfirmation,
    /// Voice is unavailable (no microphone, offline with no local engine,
    /// provider down, …); a keyboard-first recovery is offered.
    Unavailable,
    /// Voice is blocked by policy or the deployment envelope; a keyboard-first
    /// recovery is offered.
    PolicyBlocked,
}

impl VoiceShellLifecycleState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Listening => "listening",
            Self::Processing => "processing",
            Self::NeedsConfirmation => "needs_confirmation",
            Self::Unavailable => "unavailable",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// `true` while audio is actively being captured or recognized, so the mic
    /// indicator MUST be visible.
    pub const fn capture_active(self) -> bool {
        matches!(self, Self::Listening | Self::Processing)
    }

    /// `true` for the blocked/unavailable states that MUST surface an immediate
    /// keyboard-first recovery instead of a dead end.
    pub const fn is_blocked_or_unavailable(self) -> bool {
        matches!(self, Self::Unavailable | Self::PolicyBlocked)
    }

    /// `true` when a resolved command is waiting on explicit confirmation.
    pub const fn awaits_confirmation(self) -> bool {
        matches!(self, Self::NeedsConfirmation)
    }
}

/// Mode strip keeping command and dictation modes separate and visible.
///
/// Both labels are always present so a user can read which mode is active
/// without inferring it from the absence of the other; the strip never lets one
/// mode silently stand in for the other.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceModeStrip {
    /// Active mode (command vs dictation made explicit; idle/blocked carried as
    /// their own [`VoiceModeClass`] variants).
    pub active_mode: VoiceModeClass,
    /// Label ref for the command-mode segment (always shown).
    pub command_mode_label_ref: String,
    /// Label ref for the dictation-mode segment (always shown).
    pub dictation_mode_label_ref: String,
    /// Canonical command id that toggles between command and dictation modes.
    pub mode_toggle_command_id: String,
    /// `true` when both mode segments are rendered (never one collapsing into
    /// the other).
    pub both_modes_visible: bool,
    /// `true` when the strip and its toggle are reachable by keyboard.
    pub keyboard_reachable: bool,
}

/// Push-to-talk (or equivalent explicit) activation control.
///
/// Push-to-talk is the default. Continuous/wake activation is only ever a
/// non-default path gated by an explicit opt-in, so capture can never become
/// silently always-on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PushToTalkControl {
    /// Default activation class for the row (push-to-talk by default).
    pub default_activation_class: VoiceActivationClass,
    /// Canonical command id for the press-and-hold activation.
    pub hold_command_id: String,
    /// Canonical command id for the toggle activation.
    pub toggle_command_id: String,
    /// `true` when any continuous/wake activation is gated behind an explicit
    /// opt-in.
    pub continuous_requires_opt_in: bool,
    /// Background-listening state (off by default; only an explicit opt-in turns
    /// it on).
    pub background_listening_state: BackgroundListeningState,
    /// Label ref describing how to activate capture.
    pub activation_hint_label_ref: String,
    /// `true` when the activation control is reachable by keyboard.
    pub keyboard_reachable: bool,
}

/// Mic-state pill: the persistent cue rendered whenever capture is active.
///
/// The pill always carries the active mode and the local-versus-hosted
/// processing cue, so decorative mic chrome can never obscure the explicit mode
/// or locality state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceMicIndicator {
    /// Persistent mic-indicator class.
    pub indicator_class: MicIndicatorClass,
    /// `true` when audio is actively being captured.
    pub capture_active: bool,
    /// Active voice mode disclosed on the pill.
    pub active_mode: VoiceModeClass,
    /// Local-or-hosted processing cue disclosed on the pill.
    pub processing_locality: ProcessingLocalityCue,
    /// Canonical command id for the mute action.
    pub mute_command_id: String,
    /// Canonical command id for the stop action.
    pub stop_command_id: String,
    /// Accessibility label ref narrated by the screen reader.
    pub accessibility_label_ref: String,
    /// `true` when the pill and its actions are reachable by keyboard.
    pub keyboard_reachable: bool,
    /// Layout target the pill renders in (status zone, editor adjunct, …).
    pub layout_target_class: String,
}

/// Inline provider / locality / retention disclosure.
///
/// Surfaced on the shell row, not behind a deep settings dive, so a claim-
/// bearing voice surface always discloses where speech is processed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLocalityDisclosure {
    /// Provider or local-engine label ref.
    pub provider_or_local_engine_label_ref: String,
    /// Local-or-hosted processing cue.
    pub processing_locality: ProcessingLocalityCue,
    /// Retention posture disclosed alongside locality.
    pub retention_mode: RetentionMode,
    /// `true` when the disclosure is visible inline (no settings dive needed).
    pub visible_without_settings_dive: bool,
    /// Label ref for the inline disclosure text.
    pub disclosure_label_ref: String,
}

/// Keyboard-first recovery affordance for unavailable / blocked rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceRecoveryAffordance {
    /// `true` when a keyboard-first recovery is offered immediately (not buried
    /// behind a retry-only dialog).
    pub keyboard_first_recovery_immediate: bool,
    /// Canonical command id of the keyboard fallback.
    pub keyboard_fallback_command_id: String,
    /// Label ref for the recovery affordance.
    pub recovery_label_ref: String,
    /// Typed unavailable reason, when the row is unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<VoiceUnavailableReason>,
    /// Policy lock/block note ref, when the row is policy-blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_block_note_ref: Option<String>,
}

/// One claimed voice surface's explicit shell state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceShellStateRow {
    /// Record discriminator; equals [`VOICE_SHELL_STATE_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; equals [`VOICE_SHELL_STATE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Surface label ref.
    pub surface_label_ref: String,
    /// Claim posture against the claimed-surface rules.
    pub claim_posture: VoiceClaimPosture,
    /// Controlled lifecycle state for this row.
    pub lifecycle_state: VoiceShellLifecycleState,
    /// Mode strip (command vs dictation always visible).
    pub mode_strip: VoiceModeStrip,
    /// Push-to-talk / explicit activation control.
    pub activation: PushToTalkControl,
    /// Mic-state pill.
    pub mic_indicator: VoiceMicIndicator,
    /// Inline provider / locality / retention disclosure.
    pub provider_locality: ProviderLocalityDisclosure,
    /// Keyboard-first recovery affordance.
    pub recovery: VoiceRecoveryAffordance,
    /// Policy state of the session backing this row.
    pub policy_state: VoicePolicyState,
    /// `true` when the mode and lifecycle state are announced to assistive tech.
    pub screen_reader_announces_state: bool,
    /// `true` when the whole row is reachable and operable by keyboard.
    pub keyboard_reachable: bool,
    /// Redaction class.
    pub redaction_class: String,
}

impl VoiceShellStateRow {
    /// `true` when command and dictation modes are both visible and the active
    /// mode reads as a definite command/dictation/idle/blocked state.
    pub fn mode_is_explicit(&self) -> bool {
        self.mode_strip.both_modes_visible
            && !self.mode_strip.command_mode_label_ref.is_empty()
            && !self.mode_strip.dictation_mode_label_ref.is_empty()
    }

    /// `true` when activation defaults to an explicit (push-to-talk / manual)
    /// class, or a continuous/wake class backed by an explicit opt-in.
    pub fn activation_default_ok(&self) -> bool {
        if self.activation.default_activation_class.is_explicit() {
            return true;
        }
        self.activation.continuous_requires_opt_in
            && self.activation.background_listening_state == BackgroundListeningState::OnUserOptedIn
    }

    /// Collects every invariant this row violates. An empty result means the
    /// row keeps voice legible, push-to-talk-default, disclosed, and
    /// keyboard-recoverable.
    pub fn check(&self) -> Vec<VoiceShellStateViolation> {
        let mut out = Vec::new();
        let id = || self.row_id.clone();

        if !self.mode_is_explicit() {
            out.push(VoiceShellStateViolation::ModeNotExplicitlyVisible { row_id: id() });
        }

        // The mic indicator must be visible — and carry the live mode and
        // locality — whenever capture is active.
        if self.lifecycle_state.capture_active() {
            if !self.mic_indicator.capture_active
                || self.mic_indicator.indicator_class
                    != MicIndicatorClass::PersistentIndicatorVisibleCaptureActive
            {
                out.push(VoiceShellStateViolation::MicIndicatorHiddenDuringCapture {
                    row_id: id(),
                });
            }
            if self.mic_indicator.accessibility_label_ref.is_empty() {
                out.push(
                    VoiceShellStateViolation::CaptureWithoutAccessibilityAnnouncement {
                        row_id: id(),
                    },
                );
            }
        }

        if !self.activation_default_ok() {
            out.push(VoiceShellStateViolation::ContinuousListeningWithoutOptIn { row_id: id() });
        }

        // A claimed surface must disclose provider/locality inline.
        if self.claim_posture.is_claimed() && !self.provider_locality.visible_without_settings_dive
        {
            out.push(
                VoiceShellStateViolation::ProviderLocalityRequiresSettingsDive { row_id: id() },
            );
        }

        // Blocked/unavailable rows must offer an immediate keyboard-first
        // recovery with a typed reason.
        if self.lifecycle_state.is_blocked_or_unavailable() {
            if !self.recovery.keyboard_first_recovery_immediate
                || self.recovery.keyboard_fallback_command_id.is_empty()
            {
                out.push(
                    VoiceShellStateViolation::BlockedStateMissingKeyboardRecovery { row_id: id() },
                );
            }
            let has_reason = self.recovery.unavailable_reason.is_some()
                || self.recovery.policy_block_note_ref.is_some();
            if !has_reason {
                out.push(VoiceShellStateViolation::BlockedStateMissingReason { row_id: id() });
            }
        }

        if !self.screen_reader_announces_state {
            out.push(VoiceShellStateViolation::StateNotAnnounced { row_id: id() });
        }

        if !self.keyboard_reachable {
            out.push(VoiceShellStateViolation::KeyboardUnreachable { row_id: id() });
        }

        out
    }

    /// One compact, support-safe summary line for the row.
    pub fn compact_line(&self) -> String {
        format!(
            "{} | {} | mode={} | activation={} | locality={} | policy={}",
            self.row_id,
            self.lifecycle_state.as_str(),
            self.mode_strip.active_mode.as_str(),
            self.activation.default_activation_class.as_str(),
            self.provider_locality.processing_locality.as_str(),
            self.policy_state.as_str(),
        )
    }
}

/// Cross-row invariant manifest. Every field is `true` exactly when the packet
/// validates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceShellStateInvariantManifest {
    /// Every row keeps command and dictation mode explicit and visible.
    pub every_row_mode_explicit: bool,
    /// The mic indicator is visible whenever a row is capturing.
    pub mic_visible_whenever_capturing: bool,
    /// Activation defaults to push-to-talk / explicit, or opt-in continuous.
    pub push_to_talk_or_opt_in_default: bool,
    /// No claimed row defaults to hidden continuous listening.
    pub no_hidden_continuous_listening: bool,
    /// Provider / locality is disclosed inline on every claimed row.
    pub provider_locality_inline: bool,
    /// Blocked / unavailable rows offer an immediate keyboard-first recovery.
    pub blocked_states_offer_keyboard_first_recovery: bool,
    /// Capture is always announced to assistive tech.
    pub capture_always_announced: bool,
}

impl VoiceShellStateInvariantManifest {
    /// The all-satisfied manifest.
    pub const fn all_true() -> Self {
        Self {
            every_row_mode_explicit: true,
            mic_visible_whenever_capturing: true,
            push_to_talk_or_opt_in_default: true,
            no_hidden_continuous_listening: true,
            provider_locality_inline: true,
            blocked_states_offer_keyboard_first_recovery: true,
            capture_always_announced: true,
        }
    }

    /// Recomputes the manifest from a row set by lowering each row's violations
    /// onto the matching invariant.
    pub fn from_rows(rows: &[VoiceShellStateRow]) -> Self {
        let mut manifest = Self::all_true();
        for row in rows {
            for violation in row.check() {
                match violation {
                    VoiceShellStateViolation::ModeNotExplicitlyVisible { .. } => {
                        manifest.every_row_mode_explicit = false;
                    }
                    VoiceShellStateViolation::MicIndicatorHiddenDuringCapture { .. } => {
                        manifest.mic_visible_whenever_capturing = false;
                    }
                    VoiceShellStateViolation::ContinuousListeningWithoutOptIn { .. } => {
                        manifest.push_to_talk_or_opt_in_default = false;
                        manifest.no_hidden_continuous_listening = false;
                    }
                    VoiceShellStateViolation::ProviderLocalityRequiresSettingsDive { .. } => {
                        manifest.provider_locality_inline = false;
                    }
                    VoiceShellStateViolation::BlockedStateMissingKeyboardRecovery { .. }
                    | VoiceShellStateViolation::BlockedStateMissingReason { .. } => {
                        manifest.blocked_states_offer_keyboard_first_recovery = false;
                    }
                    VoiceShellStateViolation::CaptureWithoutAccessibilityAnnouncement {
                        ..
                    }
                    | VoiceShellStateViolation::StateNotAnnounced { .. } => {
                        manifest.capture_always_announced = false;
                    }
                    VoiceShellStateViolation::KeyboardUnreachable { .. } => {}
                }
            }
        }
        manifest
    }
}

/// One way a [`VoiceShellStateRow`] can break the voice-legibility contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "violation_kind", rename_all = "snake_case")]
pub enum VoiceShellStateViolation {
    /// Command and dictation modes are not both explicitly visible.
    ModeNotExplicitlyVisible {
        /// Offending row id.
        row_id: String,
    },
    /// The mic indicator is hidden while capture is active.
    MicIndicatorHiddenDuringCapture {
        /// Offending row id.
        row_id: String,
    },
    /// Capture defaults to continuous/wake without an explicit opt-in.
    ContinuousListeningWithoutOptIn {
        /// Offending row id.
        row_id: String,
    },
    /// A claimed row hides provider/locality behind a settings dive.
    ProviderLocalityRequiresSettingsDive {
        /// Offending row id.
        row_id: String,
    },
    /// A blocked/unavailable row offers no immediate keyboard-first recovery.
    BlockedStateMissingKeyboardRecovery {
        /// Offending row id.
        row_id: String,
    },
    /// A blocked/unavailable row carries no typed reason.
    BlockedStateMissingReason {
        /// Offending row id.
        row_id: String,
    },
    /// Capture is active but no accessibility announcement is present.
    CaptureWithoutAccessibilityAnnouncement {
        /// Offending row id.
        row_id: String,
    },
    /// The row's mode/lifecycle state is not announced to assistive tech.
    StateNotAnnounced {
        /// Offending row id.
        row_id: String,
    },
    /// The row is not reachable by keyboard.
    KeyboardUnreachable {
        /// Offending row id.
        row_id: String,
    },
}

impl VoiceShellStateViolation {
    /// Stable class token for the violation kind.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ModeNotExplicitlyVisible { .. } => "mode_not_explicitly_visible",
            Self::MicIndicatorHiddenDuringCapture { .. } => "mic_indicator_hidden_during_capture",
            Self::ContinuousListeningWithoutOptIn { .. } => "continuous_listening_without_opt_in",
            Self::ProviderLocalityRequiresSettingsDive { .. } => {
                "provider_locality_requires_settings_dive"
            }
            Self::BlockedStateMissingKeyboardRecovery { .. } => {
                "blocked_state_missing_keyboard_recovery"
            }
            Self::BlockedStateMissingReason { .. } => "blocked_state_missing_reason",
            Self::CaptureWithoutAccessibilityAnnouncement { .. } => {
                "capture_without_accessibility_announcement"
            }
            Self::StateNotAnnounced { .. } => "state_not_announced",
            Self::KeyboardUnreachable { .. } => "keyboard_unreachable",
        }
    }

    /// Offending row id.
    pub fn row_id(&self) -> &str {
        match self {
            Self::ModeNotExplicitlyVisible { row_id }
            | Self::MicIndicatorHiddenDuringCapture { row_id }
            | Self::ContinuousListeningWithoutOptIn { row_id }
            | Self::ProviderLocalityRequiresSettingsDive { row_id }
            | Self::BlockedStateMissingKeyboardRecovery { row_id }
            | Self::BlockedStateMissingReason { row_id }
            | Self::CaptureWithoutAccessibilityAnnouncement { row_id }
            | Self::StateNotAnnounced { row_id }
            | Self::KeyboardUnreachable { row_id } => row_id,
        }
    }
}

/// Inspectable truth packet for the voice shell-state lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceShellStatePacket {
    /// Record discriminator; equals [`VOICE_SHELL_STATE_PACKET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; equals [`VOICE_SHELL_STATE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Ref to the cross-surface voice / dictation contract this lane keeps legible.
    pub voice_and_dictation_contract_ref: String,
    /// Ref to the M5 voice-qualification matrix this lane surfaces.
    pub qualification_matrix_ref: String,
    /// Ref to the published companion doc.
    pub doc_ref: String,
    /// Ref to the checked-in fixtures directory.
    pub fixtures_dir_ref: String,
    /// Shell-state rows, in canonical order.
    pub rows: Vec<VoiceShellStateRow>,
    /// Cross-row invariant manifest.
    pub invariants: VoiceShellStateInvariantManifest,
    /// `true` — no raw audio/transcript bytes ever cross this boundary.
    pub raw_audio_or_transcript_bytes_excluded: bool,
}

impl VoiceShellStatePacket {
    /// Builds a packet from `rows`, stamping the canonical envelope and
    /// recomputing the invariant manifest from the rows.
    pub fn new(rows: Vec<VoiceShellStateRow>) -> Self {
        let invariants = VoiceShellStateInvariantManifest::from_rows(&rows);
        Self {
            record_kind: VOICE_SHELL_STATE_PACKET_RECORD_KIND.to_owned(),
            schema_version: VOICE_SHELL_STATE_SCHEMA_VERSION,
            shared_contract_ref: VOICE_SHELL_STATE_SHARED_CONTRACT_REF.to_owned(),
            packet_id: VOICE_SHELL_STATE_PACKET_ID.to_owned(),
            voice_and_dictation_contract_ref: VOICE_AND_DICTATION_CONTRACT_REF.to_owned(),
            qualification_matrix_ref: VOICE_QUALIFICATION_MATRIX_REF.to_owned(),
            doc_ref: VOICE_SHELL_STATE_DOC_REF.to_owned(),
            fixtures_dir_ref: VOICE_SHELL_STATE_FIXTURES_DIR_REF.to_owned(),
            rows,
            invariants,
            raw_audio_or_transcript_bytes_excluded: true,
        }
    }

    /// Returns the row with `row_id`, if present.
    pub fn row(&self, row_id: &str) -> Option<&VoiceShellStateRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// Collects every invariant violation across all rows. An empty result
    /// means the packet keeps voice mode legible, push-to-talk-default,
    /// disclosed, and keyboard-recoverable.
    pub fn validate(&self) -> Vec<VoiceShellStateViolation> {
        self.rows
            .iter()
            .flat_map(VoiceShellStateRow::check)
            .collect()
    }

    /// `true` when no row violates an invariant.
    pub fn is_well_formed(&self) -> bool {
        self.validate().is_empty()
    }

    /// Support-safe compact lines, one per row, plus a header.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(self.rows.len() + 1);
        lines.push(format!(
            "{} | rows={} | invariants_ok={}",
            self.packet_id,
            self.rows.len(),
            self.is_well_formed(),
        ));
        lines.extend(self.rows.iter().map(VoiceShellStateRow::compact_line));
        lines
    }

    /// Renders the published Markdown companion summary.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Voice shell states\n\n");
        out.push_str(
            "Generated from the `voice_shell_state` seed. Do not edit by hand; \
             regenerate with `cargo run -p aureline-shell --example dump_voice_shell_state -- write`.\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Contract: `{}`\n",
            self.voice_and_dictation_contract_ref
        ));
        out.push_str(&format!(
            "- Qualification truth: `{}`\n",
            self.qualification_matrix_ref
        ));
        out.push_str(&format!("- Fixtures: `{}`\n\n", self.fixtures_dir_ref));

        out.push_str(
            "| Row | Lifecycle | Active mode | Activation | Locality | Retention | Policy |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.rows {
            out.push_str(&format!(
                "| `{}` | {} | {} | {} | {} | {} | {} |\n",
                row.row_id,
                row.lifecycle_state.as_str(),
                row.mode_strip.active_mode.as_str(),
                row.activation.default_activation_class.as_str(),
                row.provider_locality.processing_locality.as_str(),
                row.provider_locality.retention_mode.as_str(),
                row.policy_state.as_str(),
            ));
        }
        out.push('\n');
        out.push_str("## Invariants\n\n");
        let inv = &self.invariants;
        for (label, value) in [
            (
                "Every row keeps command/dictation mode explicit",
                inv.every_row_mode_explicit,
            ),
            (
                "Mic indicator visible whenever capturing",
                inv.mic_visible_whenever_capturing,
            ),
            (
                "Push-to-talk or opt-in continuous default",
                inv.push_to_talk_or_opt_in_default,
            ),
            (
                "No hidden continuous listening",
                inv.no_hidden_continuous_listening,
            ),
            (
                "Provider/locality disclosed inline",
                inv.provider_locality_inline,
            ),
            (
                "Blocked states offer keyboard-first recovery",
                inv.blocked_states_offer_keyboard_first_recovery,
            ),
            ("Capture always announced", inv.capture_always_announced),
        ] {
            out.push_str(&format!(
                "- [{}] {}\n",
                if value { "x" } else { " " },
                label
            ));
        }
        out
    }

    /// Serializes the packet as the canonical export-safe pretty JSON (no
    /// trailing newline).
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("voice shell-state packet serializes")
    }
}

/// Serializes a value as pretty JSON with a trailing newline (the on-disk
/// fixture form).
pub fn fixture_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let mut json = serde_json::to_string_pretty(value)?;
    json.push('\n');
    Ok(json)
}

/// Stable fixture file name for a row's lifecycle state.
pub const fn row_fixture_file_name(state: VoiceShellLifecycleState) -> &'static str {
    match state {
        VoiceShellLifecycleState::Idle => "idle.json",
        VoiceShellLifecycleState::Listening => "listening.json",
        VoiceShellLifecycleState::Processing => "processing.json",
        VoiceShellLifecycleState::NeedsConfirmation => "needs_confirmation.json",
        VoiceShellLifecycleState::Unavailable => "unavailable.json",
        VoiceShellLifecycleState::PolicyBlocked => "policy_blocked.json",
    }
}

/// Writes the seeded packet, the per-row fixtures, and the compact summary to
/// `dir`. This is the single mint path the example dump and the equality test
/// share, so the checked-in fixtures can never drift silently.
pub fn write_fixtures(dir: &Path, packet: &VoiceShellStatePacket) -> io::Result<()> {
    fs::create_dir_all(dir)?;

    let packet_json =
        fixture_json(packet).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    fs::write(dir.join("packet.json"), packet_json)?;

    for row in &packet.rows {
        let json = fixture_json(row).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        fs::write(dir.join(row_fixture_file_name(row.lifecycle_state)), json)?;
    }

    let mut compact = packet.compact_lines().join("\n");
    compact.push('\n');
    fs::write(dir.join("compact.txt"), compact)?;

    Ok(())
}
