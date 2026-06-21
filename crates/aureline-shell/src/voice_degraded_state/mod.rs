//! Voice fallback and degraded-mode flows with keyboard-first recovery.
//!
//! Voice is an explicit, privacy-bounded **optional** input mode in Aureline.
//! When the conditions a claimed voice surface depends on are not met — there is
//! no microphone, the room is too noisy, the speech provider is offline, the
//! requested language pack is missing, or policy blocks voice — that surface
//! MUST enter a *controlled, named, recoverable* degraded state rather than
//! collapse into a generic error, fail silently, or oscillate. This lane owns
//! that contract.
//!
//! It is the degraded-path complement to the three normal-path voice lanes:
//!
//! - [`crate::voice`] models the bounded preview/beta capture surface;
//! - [`crate::voice_shell_state`] owns the always-visible mode / mic / locality
//!   shell state and its controlled lifecycle vocabulary; and
//! - [`crate::freeze_the_m5_voice_mode_provider_transcript_retention_and_command_parity_matrix`]
//!   freezes the M5 provider / retention / command-parity qualification matrix.
//!
//! This lane does not mint a second interaction model. It reuses the controlled
//! [`VoiceShellLifecycleState`] (a degraded flow always lands on `unavailable`,
//! `policy_blocked`, or `needs_confirmation`), the canonical
//! [`VoiceUnavailableReason`] vocabulary, and the policy / claim vocabularies the
//! sibling lanes own, and projects them into the concrete degraded affordances
//! the spec requires:
//!
//! - a durable [`DegradedBanner`] that names the *specific* failed capability,
//!   the cause, and the consequence — never a generic "Voice unavailable" line;
//! - one or more concrete inline [`RecoveryAction`]s, each bound to a canonical
//!   command id, so every failure class has a real next step;
//! - a [`KeyboardFirstFallback`] that returns the user to the keyboard /
//!   command palette **without losing focus or uncommitted work**; and
//! - a narration-safe [`DegradedNarration`] that announces the cause and the
//!   recovery exactly once per controlled transition (no oscillation, no spam).
//!
//! Each [`VoiceDegradedFlow`] covers exactly one [`VoiceDegradedCause`]. The
//! top-level [`VoiceDegradedStatePacket`] is the inspectable truth packet
//! consumed by the live shell, Help/About, diagnostics, and metadata-only
//! support export. [`VoiceDegradedStatePacket::validate`] refuses any flow that
//! hides the cause behind generic copy, omits a concrete recovery, drops the
//! keyboard-first fallback, fails to enter a controlled state, narrates unsafely,
//! or suppresses an existing non-voice recovery affordance. Raw audio bytes, raw
//! transcript text, raw provider payloads, private paths, and credentials never
//! cross this boundary; the packet carries only typed class tokens, booleans,
//! opaque ids, and redaction-aware label refs.
//!
//! The seed in [`seed`] is the single mint-from-truth source for the checked-in
//! fixtures under [`VOICE_DEGRADED_STATE_FIXTURES_DIR_REF`], the published help
//! doc [`VOICE_DEGRADED_STATE_DOC_REF`], and the degraded-state matrix
//! [`VOICE_DEGRADED_STATE_MATRIX_REF`].

#[cfg(test)]
mod tests;

pub mod seed;

use std::path::Path;
use std::{fs, io};

use serde::{Deserialize, Serialize};

pub use crate::freeze_the_m5_voice_mode_provider_transcript_retention_and_command_parity_matrix::VoicePolicyState;
pub use crate::voice::{VoiceClaimPosture, VoiceUnavailableReason};
pub use crate::voice_shell_state::VoiceShellLifecycleState;

pub use seed::seeded_voice_degraded_state_packet;

/// Schema version exported with every voice-degraded-state record.
pub const VOICE_DEGRADED_STATE_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref quoted by every voice-degraded-state record.
pub const VOICE_DEGRADED_STATE_SHARED_CONTRACT_REF: &str = "shell:voice_degraded_state:v1";

/// Stable record kind for [`VoiceDegradedStatePacket`] payloads.
pub const VOICE_DEGRADED_STATE_PACKET_RECORD_KIND: &str =
    "shell_voice_degraded_state_packet_record";

/// Stable record kind for [`VoiceDegradedFlow`] payloads.
pub const VOICE_DEGRADED_FLOW_RECORD_KIND: &str = "shell_voice_degraded_flow_record";

/// Stable packet id quoted across surfaces.
pub const VOICE_DEGRADED_STATE_PACKET_ID: &str = "shell:voice_degraded_state:packet:v1";

/// Repo-relative path of the published help / recovery doc.
pub const VOICE_DEGRADED_STATE_DOC_REF: &str = "docs/help/voice-fallback-and-recovery.md";

/// Repo-relative path of the published degraded-state matrix.
pub const VOICE_DEGRADED_STATE_MATRIX_REF: &str = "artifacts/voice/degraded-state-matrix.md";

/// Repo-relative directory of the checked-in mint-from-truth fixtures.
pub const VOICE_DEGRADED_STATE_FIXTURES_DIR_REF: &str = "fixtures/voice/fallback-and-noisy-env";

/// Contract ref of the sibling shell-state lane whose controlled lifecycle and
/// mic/mode/locality vocabulary this lane builds on.
pub const VOICE_SHELL_STATE_CONTRACT_REF: &str = "shell:voice_shell_state:v1";

/// Redaction class stamped on every record; the packet carries metadata only.
pub const REDACTION_CLASS: &str = "metadata_safe_default";

/// The major voice failure classes this lane gives a controlled degraded flow.
///
/// Surfaces project these causes verbatim — they never substitute a generic
/// "Voice unavailable" line for one of these tokens. Each cause maps onto a
/// controlled [`VoiceShellLifecycleState`] and, where one exists, a canonical
/// [`VoiceUnavailableReason`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceDegradedCause {
    /// No microphone device is present or permitted.
    MissingMicrophoneHardware,
    /// The environment is too noisy for reliable recognition; capture continues
    /// but every result is held for confirmation.
    NoisyEnvironment,
    /// The speech provider is unreachable and no usable engine is available.
    ProviderOffline,
    /// The requested language / locale speech pack or profile is missing.
    LanguagePackMissing,
    /// Voice is blocked by managed policy or the deployment envelope.
    PolicyBlocked,
}

impl VoiceDegradedCause {
    /// Every degraded cause, in canonical order.
    pub const ALL: [VoiceDegradedCause; 5] = [
        Self::MissingMicrophoneHardware,
        Self::NoisyEnvironment,
        Self::ProviderOffline,
        Self::LanguagePackMissing,
        Self::PolicyBlocked,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingMicrophoneHardware => "missing_microphone_hardware",
            Self::NoisyEnvironment => "noisy_environment",
            Self::ProviderOffline => "provider_offline",
            Self::LanguagePackMissing => "language_pack_missing",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// Canonical cross-surface unavailable reason for this cause, when one
    /// exists. Language-pack-missing has no canonical reason token, so the
    /// lane-local cause is authoritative.
    pub const fn canonical_unavailable_reason(self) -> Option<VoiceUnavailableReason> {
        match self {
            Self::MissingMicrophoneHardware => Some(VoiceUnavailableReason::NoMicrophone),
            Self::NoisyEnvironment => Some(VoiceUnavailableReason::NoisyEnvironment),
            Self::ProviderOffline => Some(VoiceUnavailableReason::ProviderUnavailable),
            Self::LanguagePackMissing => None,
            Self::PolicyBlocked => Some(VoiceUnavailableReason::PolicyLockedOrBlocked),
        }
    }

    /// The controlled lifecycle state a surface enters for this cause. A noisy
    /// environment keeps voice usable but confirmation-gated; every other cause
    /// makes voice unavailable (or policy-blocked) rather than silently broken.
    pub const fn controlled_state(self) -> VoiceShellLifecycleState {
        match self {
            Self::NoisyEnvironment => VoiceShellLifecycleState::NeedsConfirmation,
            Self::PolicyBlocked => VoiceShellLifecycleState::PolicyBlocked,
            Self::MissingMicrophoneHardware | Self::ProviderOffline | Self::LanguagePackMissing => {
                VoiceShellLifecycleState::Unavailable
            }
        }
    }
}

/// Severity tier of a degraded banner. Maps onto the error taxonomy: a degraded
/// state is a contextual banner the user can usually work around, while a
/// policy block is a harder stop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedBannerSeverity {
    /// Informational: voice is degraded but largely usable.
    Informational,
    /// Warning: voice cannot capture reliably until the user acts.
    Warning,
    /// Blocked: voice is stopped by policy or the envelope.
    Blocked,
}

impl DegradedBannerSeverity {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::Warning => "warning",
            Self::Blocked => "blocked",
        }
    }
}

/// What the recovery does for a degraded flow. Every flow always also exposes a
/// keyboard-first fallback; this names the *primary* posture of the flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedRecoveryPosture {
    /// Hand off to the keyboard / command palette as the primary path.
    FellBackToKeyboardFirst,
    /// Offer an on-device engine when the hosted provider is offline.
    OfferedOnDeviceEngineFallback,
    /// Keep recognizing but hold every result for explicit confirmation.
    HeldForConfirmation,
    /// Hold voice until the blocking condition clears, then retry.
    HeldUntilConditionClears,
}

impl DegradedRecoveryPosture {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FellBackToKeyboardFirst => "fell_back_to_keyboard_first",
            Self::OfferedOnDeviceEngineFallback => "offered_on_device_engine_fallback",
            Self::HeldForConfirmation => "held_for_confirmation",
            Self::HeldUntilConditionClears => "held_until_condition_clears",
        }
    }
}

/// Kind of a concrete inline recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionKind {
    /// Fall back to keyboard / command-palette input.
    KeyboardFallback,
    /// Switch capture to an on-device engine.
    SwitchToOnDeviceEngine,
    /// Open microphone / input-device settings.
    OpenMicrophoneSettings,
    /// Install or switch the speech language / locale pack.
    InstallOrSwitchLanguagePack,
    /// Retry voice once the blocking condition clears.
    RetryWhenConditionClears,
    /// Confirm or correct a held, low-confidence result.
    ConfirmHeldResult,
    /// Open the policy detail / access-request surface.
    OpenPolicyDetails,
}

impl RecoveryActionKind {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFallback => "keyboard_fallback",
            Self::SwitchToOnDeviceEngine => "switch_to_on_device_engine",
            Self::OpenMicrophoneSettings => "open_microphone_settings",
            Self::InstallOrSwitchLanguagePack => "install_or_switch_language_pack",
            Self::RetryWhenConditionClears => "retry_when_condition_clears",
            Self::ConfirmHeldResult => "confirm_held_result",
            Self::OpenPolicyDetails => "open_policy_details",
        }
    }
}

/// Politeness of the assistive-tech announcement raised on a degraded transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrationPoliteness {
    /// Announced politely; does not interrupt the user mid-utterance.
    Polite,
    /// Announced assertively; used for a hard stop the user must hear.
    Assertive,
}

impl NarrationPoliteness {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Polite => "polite",
            Self::Assertive => "assertive",
        }
    }
}

/// Durable degraded banner that names the failed capability, the cause, and the
/// consequence — never generic "Voice unavailable" copy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// Severity tier.
    pub severity: DegradedBannerSeverity,
    /// `true` when the banner persists until the condition clears (not a
    /// transient toast that disappears before the user can act).
    pub durable: bool,
    /// Label ref naming the failed voice capability.
    pub title_label_ref: String,
    /// Label ref naming the *specific* cause (no generic copy).
    pub cause_detail_label_ref: String,
    /// Label ref naming the consequence (what voice can no longer do).
    pub consequence_label_ref: String,
    /// `true` when the banner names the specific cause rather than a generic
    /// failure — guards the precise-failure-cause requirement.
    pub names_specific_cause: bool,
    /// Layout placement class (banner, status item, …) per the error taxonomy.
    pub placement_class: String,
}

impl DegradedBanner {
    /// `true` when the banner is durable and names the specific failed
    /// capability, cause, and consequence.
    pub fn is_precise_and_durable(&self) -> bool {
        self.durable
            && self.names_specific_cause
            && !self.title_label_ref.is_empty()
            && !self.cause_detail_label_ref.is_empty()
            && !self.consequence_label_ref.is_empty()
    }
}

/// One concrete inline recovery action bound to a canonical command id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryAction {
    /// Stable action id.
    pub action_id: String,
    /// Recovery action kind.
    pub kind: RecoveryActionKind,
    /// Canonical command id the action invokes.
    pub command_id: String,
    /// Label ref for the action.
    pub label_ref: String,
    /// `true` when the action is reachable and operable by keyboard.
    pub keyboard_accessible: bool,
}

/// Keyboard-first fallback that preserves task continuity and focus return.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardFirstFallback {
    /// Canonical command id of the keyboard fallback (e.g. open command palette).
    pub keyboard_fallback_command_id: String,
    /// `true` when falling back keeps current focus and in-progress work.
    pub preserves_focus_and_work: bool,
    /// Label ref of the focus target returned to after the fallback.
    pub focus_return_target_ref: String,
    /// `true` when uncommitted work (dictated text, pending edits) is preserved.
    pub preserves_uncommitted_work: bool,
    /// Label ref for the fallback hint.
    pub fallback_hint_label_ref: String,
}

impl KeyboardFirstFallback {
    /// `true` when the fallback keeps focus and work and is wired to a command.
    pub fn preserves_continuity(&self) -> bool {
        self.preserves_focus_and_work
            && self.preserves_uncommitted_work
            && !self.keyboard_fallback_command_id.is_empty()
            && !self.focus_return_target_ref.is_empty()
    }
}

/// Narration-safe state change announced to assistive tech.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedNarration {
    /// Label ref announced when the surface enters this degraded state.
    pub announcement_label_ref: String,
    /// Announcement politeness.
    pub politeness: NarrationPoliteness,
    /// `true` when the announcement fires exactly once per controlled
    /// transition (no oscillation / repeated chatter).
    pub announced_once_per_transition: bool,
    /// `true` when the announcement names both the cause and the recovery.
    pub names_cause_and_recovery: bool,
}

impl DegradedNarration {
    /// `true` when the narration is safe: present, single-shot, and naming both
    /// the cause and the recovery.
    pub fn is_safe(&self) -> bool {
        !self.announcement_label_ref.is_empty()
            && self.announced_once_per_transition
            && self.names_cause_and_recovery
    }
}

/// One major voice failure class's controlled degraded flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceDegradedFlow {
    /// Record discriminator; equals [`VOICE_DEGRADED_FLOW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; equals [`VOICE_DEGRADED_STATE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable flow id.
    pub flow_id: String,
    /// Surface label ref.
    pub surface_label_ref: String,
    /// Claim posture of the affected surface.
    pub claim_posture: VoiceClaimPosture,
    /// The failure class this flow covers.
    pub cause: VoiceDegradedCause,
    /// Canonical cross-surface unavailable reason, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_unavailable_reason: Option<VoiceUnavailableReason>,
    /// Controlled lifecycle state the surface enters.
    pub lifecycle_state: VoiceShellLifecycleState,
    /// Policy state backing the flow.
    pub policy_state: VoicePolicyState,
    /// Primary recovery posture.
    pub recovery_posture: DegradedRecoveryPosture,
    /// Durable, precise degraded banner.
    pub banner: DegradedBanner,
    /// Concrete inline recovery actions (at least one, keyboard-first).
    pub recovery_actions: Vec<RecoveryAction>,
    /// Keyboard-first fallback preserving focus and work.
    pub keyboard_fallback: KeyboardFirstFallback,
    /// Narration-safe state change.
    pub narration: DegradedNarration,
    /// `true` when the surface enters a controlled state rather than failing
    /// silently or oscillating.
    pub enters_controlled_state: bool,
    /// `true` when the degraded flow leaves existing non-voice recovery
    /// affordances intact (does not suppress or overwrite them).
    pub preserves_nonvoice_recovery: bool,
    /// `true` when the mode / lifecycle change is announced to assistive tech.
    pub screen_reader_announces_state: bool,
    /// `true` when the whole flow is reachable and operable by keyboard.
    pub keyboard_reachable: bool,
    /// Redaction class.
    pub redaction_class: String,
}

impl VoiceDegradedFlow {
    /// `true` when the lifecycle state is one of the controlled degraded states
    /// (unavailable, policy-blocked, needs-confirmation) — never a normal
    /// capturing/idle state and never silent.
    pub fn lands_on_controlled_state(&self) -> bool {
        matches!(
            self.lifecycle_state,
            VoiceShellLifecycleState::Unavailable
                | VoiceShellLifecycleState::PolicyBlocked
                | VoiceShellLifecycleState::NeedsConfirmation
        )
    }

    /// `true` when at least one recovery action is a keyboard fallback bound to a
    /// command id.
    pub fn has_keyboard_first_recovery_action(&self) -> bool {
        self.recovery_actions.iter().any(|action| {
            action.kind == RecoveryActionKind::KeyboardFallback && !action.command_id.is_empty()
        })
    }

    /// Collects every invariant this flow violates. An empty result means the
    /// flow is named, recoverable, keyboard-first, controlled, narration-safe,
    /// and non-destructive to non-voice recovery.
    pub fn check(&self) -> Vec<VoiceDegradedStateViolation> {
        let mut out = Vec::new();
        let id = || self.flow_id.clone();

        if !self.banner.is_precise_and_durable() {
            out.push(VoiceDegradedStateViolation::GenericOrTransientBanner { flow_id: id() });
        }

        let has_concrete_action = !self.recovery_actions.is_empty()
            && self
                .recovery_actions
                .iter()
                .all(|action| !action.command_id.is_empty() && !action.label_ref.is_empty());
        if !has_concrete_action {
            out.push(VoiceDegradedStateViolation::NoConcreteRecoveryAction { flow_id: id() });
        }

        if !self.has_keyboard_first_recovery_action() {
            out.push(VoiceDegradedStateViolation::MissingKeyboardFirstFallback { flow_id: id() });
        }

        if !self.keyboard_fallback.preserves_continuity() {
            out.push(
                VoiceDegradedStateViolation::KeyboardFallbackLosesContinuity { flow_id: id() },
            );
        }

        if !self.enters_controlled_state || !self.lands_on_controlled_state() {
            out.push(VoiceDegradedStateViolation::NotAControlledState { flow_id: id() });
        }

        // Policy cause, policy-blocked state, and policy_state must agree.
        let policy_consistent = match self.cause {
            VoiceDegradedCause::PolicyBlocked => {
                self.lifecycle_state == VoiceShellLifecycleState::PolicyBlocked
                    && self.policy_state == VoicePolicyState::PolicyBlocked
            }
            _ => {
                self.lifecycle_state != VoiceShellLifecycleState::PolicyBlocked
                    && self.policy_state != VoicePolicyState::PolicyBlocked
            }
        };
        if !policy_consistent {
            out.push(VoiceDegradedStateViolation::PolicyStateInconsistent { flow_id: id() });
        }

        if self.canonical_unavailable_reason != self.cause.canonical_unavailable_reason() {
            out.push(VoiceDegradedStateViolation::CanonicalReasonMismatch { flow_id: id() });
        }

        if !self.narration.is_safe() || !self.screen_reader_announces_state {
            out.push(VoiceDegradedStateViolation::NarrationUnsafe { flow_id: id() });
        }

        if !self.preserves_nonvoice_recovery {
            out.push(VoiceDegradedStateViolation::SuppressesNonVoiceRecovery { flow_id: id() });
        }

        let keyboard_ok = self.keyboard_reachable
            && self
                .recovery_actions
                .iter()
                .all(|action| action.keyboard_accessible);
        if !keyboard_ok {
            out.push(VoiceDegradedStateViolation::KeyboardUnreachable { flow_id: id() });
        }

        out
    }

    /// One compact, support-safe summary line for the flow.
    pub fn compact_line(&self) -> String {
        format!(
            "{} | cause={} | state={} | posture={} | actions={} | policy={}",
            self.flow_id,
            self.cause.as_str(),
            self.lifecycle_state.as_str(),
            self.recovery_posture.as_str(),
            self.recovery_actions.len(),
            self.policy_state.as_str(),
        )
    }
}

/// Cross-flow invariant manifest. Every field is `true` exactly when the packet
/// validates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceDegradedStateInvariantManifest {
    /// Every flow shows a durable banner naming the specific cause.
    pub every_cause_has_durable_banner: bool,
    /// Every flow offers at least one concrete recovery action.
    pub every_cause_has_concrete_recovery: bool,
    /// The keyboard-first fallback preserves focus and work everywhere.
    pub keyboard_fallback_preserves_continuity: bool,
    /// Every flow lands on a controlled state — never silent, never oscillating.
    pub every_state_is_controlled: bool,
    /// No flow hides its cause behind generic failure copy.
    pub no_generic_failure_copy: bool,
    /// Every state change is narration-safe.
    pub state_changes_are_narration_safe: bool,
    /// No flow suppresses an existing non-voice recovery affordance.
    pub nonvoice_recovery_preserved: bool,
    /// All five major failure classes are covered.
    pub all_failure_classes_covered: bool,
}

impl VoiceDegradedStateInvariantManifest {
    /// The all-satisfied manifest.
    pub const fn all_true() -> Self {
        Self {
            every_cause_has_durable_banner: true,
            every_cause_has_concrete_recovery: true,
            keyboard_fallback_preserves_continuity: true,
            every_state_is_controlled: true,
            no_generic_failure_copy: true,
            state_changes_are_narration_safe: true,
            nonvoice_recovery_preserved: true,
            all_failure_classes_covered: true,
        }
    }

    /// Recomputes the manifest from a flow set by lowering each flow's
    /// violations onto the matching invariant and confirming coverage of every
    /// failure class.
    pub fn from_flows(flows: &[VoiceDegradedFlow]) -> Self {
        let mut manifest = Self::all_true();
        for flow in flows {
            for violation in flow.check() {
                match violation {
                    VoiceDegradedStateViolation::GenericOrTransientBanner { .. } => {
                        manifest.every_cause_has_durable_banner = false;
                        manifest.no_generic_failure_copy = false;
                    }
                    VoiceDegradedStateViolation::NoConcreteRecoveryAction { .. }
                    | VoiceDegradedStateViolation::MissingKeyboardFirstFallback { .. } => {
                        manifest.every_cause_has_concrete_recovery = false;
                    }
                    VoiceDegradedStateViolation::KeyboardFallbackLosesContinuity { .. } => {
                        manifest.keyboard_fallback_preserves_continuity = false;
                    }
                    VoiceDegradedStateViolation::NotAControlledState { .. }
                    | VoiceDegradedStateViolation::PolicyStateInconsistent { .. }
                    | VoiceDegradedStateViolation::CanonicalReasonMismatch { .. } => {
                        manifest.every_state_is_controlled = false;
                    }
                    VoiceDegradedStateViolation::NarrationUnsafe { .. } => {
                        manifest.state_changes_are_narration_safe = false;
                    }
                    VoiceDegradedStateViolation::SuppressesNonVoiceRecovery { .. } => {
                        manifest.nonvoice_recovery_preserved = false;
                    }
                    VoiceDegradedStateViolation::KeyboardUnreachable { .. } => {}
                }
            }
        }

        let covered: std::collections::BTreeSet<VoiceDegradedCause> =
            flows.iter().map(|flow| flow.cause).collect();
        manifest.all_failure_classes_covered = VoiceDegradedCause::ALL
            .iter()
            .all(|cause| covered.contains(cause));

        manifest
    }
}

/// One way a [`VoiceDegradedFlow`] can break the degraded-recovery contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "violation_kind", rename_all = "snake_case")]
pub enum VoiceDegradedStateViolation {
    /// The banner is transient or hides the cause behind generic copy.
    GenericOrTransientBanner {
        /// Offending flow id.
        flow_id: String,
    },
    /// The flow offers no concrete recovery action.
    NoConcreteRecoveryAction {
        /// Offending flow id.
        flow_id: String,
    },
    /// The flow has no keyboard-first fallback action.
    MissingKeyboardFirstFallback {
        /// Offending flow id.
        flow_id: String,
    },
    /// Falling back to the keyboard loses focus or in-progress work.
    KeyboardFallbackLosesContinuity {
        /// Offending flow id.
        flow_id: String,
    },
    /// The flow does not enter a controlled degraded state.
    NotAControlledState {
        /// Offending flow id.
        flow_id: String,
    },
    /// The cause, lifecycle state, and policy state disagree.
    PolicyStateInconsistent {
        /// Offending flow id.
        flow_id: String,
    },
    /// The declared canonical reason does not match the cause's canonical reason.
    CanonicalReasonMismatch {
        /// Offending flow id.
        flow_id: String,
    },
    /// The state change is not narration-safe.
    NarrationUnsafe {
        /// Offending flow id.
        flow_id: String,
    },
    /// The degraded flow suppresses an existing non-voice recovery affordance.
    SuppressesNonVoiceRecovery {
        /// Offending flow id.
        flow_id: String,
    },
    /// The flow or one of its actions is not reachable by keyboard.
    KeyboardUnreachable {
        /// Offending flow id.
        flow_id: String,
    },
}

impl VoiceDegradedStateViolation {
    /// Stable class token for the violation kind.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::GenericOrTransientBanner { .. } => "generic_or_transient_banner",
            Self::NoConcreteRecoveryAction { .. } => "no_concrete_recovery_action",
            Self::MissingKeyboardFirstFallback { .. } => "missing_keyboard_first_fallback",
            Self::KeyboardFallbackLosesContinuity { .. } => "keyboard_fallback_loses_continuity",
            Self::NotAControlledState { .. } => "not_a_controlled_state",
            Self::PolicyStateInconsistent { .. } => "policy_state_inconsistent",
            Self::CanonicalReasonMismatch { .. } => "canonical_reason_mismatch",
            Self::NarrationUnsafe { .. } => "narration_unsafe",
            Self::SuppressesNonVoiceRecovery { .. } => "suppresses_non_voice_recovery",
            Self::KeyboardUnreachable { .. } => "keyboard_unreachable",
        }
    }

    /// Offending flow id.
    pub fn flow_id(&self) -> &str {
        match self {
            Self::GenericOrTransientBanner { flow_id }
            | Self::NoConcreteRecoveryAction { flow_id }
            | Self::MissingKeyboardFirstFallback { flow_id }
            | Self::KeyboardFallbackLosesContinuity { flow_id }
            | Self::NotAControlledState { flow_id }
            | Self::PolicyStateInconsistent { flow_id }
            | Self::CanonicalReasonMismatch { flow_id }
            | Self::NarrationUnsafe { flow_id }
            | Self::SuppressesNonVoiceRecovery { flow_id }
            | Self::KeyboardUnreachable { flow_id } => flow_id,
        }
    }
}

/// Inspectable truth packet for the voice degraded-state lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceDegradedStatePacket {
    /// Record discriminator; equals [`VOICE_DEGRADED_STATE_PACKET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; equals [`VOICE_DEGRADED_STATE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Contract ref of the sibling shell-state lane this lane builds on.
    pub voice_shell_state_contract_ref: String,
    /// Ref to the published help / recovery doc.
    pub doc_ref: String,
    /// Ref to the published degraded-state matrix.
    pub matrix_ref: String,
    /// Ref to the checked-in fixtures directory.
    pub fixtures_dir_ref: String,
    /// Degraded flows, in canonical cause order.
    pub flows: Vec<VoiceDegradedFlow>,
    /// Cross-flow invariant manifest.
    pub invariants: VoiceDegradedStateInvariantManifest,
    /// `true` — no raw audio/transcript bytes ever cross this boundary.
    pub raw_audio_or_transcript_bytes_excluded: bool,
}

impl VoiceDegradedStatePacket {
    /// Builds a packet from `flows`, stamping the canonical envelope and
    /// recomputing the invariant manifest from the flows.
    pub fn new(flows: Vec<VoiceDegradedFlow>) -> Self {
        let invariants = VoiceDegradedStateInvariantManifest::from_flows(&flows);
        Self {
            record_kind: VOICE_DEGRADED_STATE_PACKET_RECORD_KIND.to_owned(),
            schema_version: VOICE_DEGRADED_STATE_SCHEMA_VERSION,
            shared_contract_ref: VOICE_DEGRADED_STATE_SHARED_CONTRACT_REF.to_owned(),
            packet_id: VOICE_DEGRADED_STATE_PACKET_ID.to_owned(),
            voice_shell_state_contract_ref: VOICE_SHELL_STATE_CONTRACT_REF.to_owned(),
            doc_ref: VOICE_DEGRADED_STATE_DOC_REF.to_owned(),
            matrix_ref: VOICE_DEGRADED_STATE_MATRIX_REF.to_owned(),
            fixtures_dir_ref: VOICE_DEGRADED_STATE_FIXTURES_DIR_REF.to_owned(),
            flows,
            invariants,
            raw_audio_or_transcript_bytes_excluded: true,
        }
    }

    /// Returns the flow covering `cause`, if present.
    pub fn flow(&self, cause: VoiceDegradedCause) -> Option<&VoiceDegradedFlow> {
        self.flows.iter().find(|flow| flow.cause == cause)
    }

    /// Collects every invariant violation across all flows. An empty result
    /// means every failure class is named, recoverable, keyboard-first,
    /// controlled, narration-safe, and non-destructive to non-voice recovery.
    pub fn validate(&self) -> Vec<VoiceDegradedStateViolation> {
        self.flows
            .iter()
            .flat_map(VoiceDegradedFlow::check)
            .collect()
    }

    /// `true` when no flow violates an invariant and every failure class is
    /// covered.
    pub fn is_well_formed(&self) -> bool {
        self.validate().is_empty() && self.invariants.all_failure_classes_covered
    }

    /// Support-safe compact lines, one per flow, plus a header.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(self.flows.len() + 1);
        lines.push(format!(
            "{} | flows={} | invariants_ok={}",
            self.packet_id,
            self.flows.len(),
            self.is_well_formed(),
        ));
        lines.extend(self.flows.iter().map(VoiceDegradedFlow::compact_line));
        lines
    }

    /// Renders the published Markdown matrix summary.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Voice degraded-state and recovery matrix\n\n");
        out.push_str(
            "Generated from the `voice_degraded_state` seed. Do not edit by hand; \
             regenerate with `cargo run -p aureline-shell --example dump_voice_degraded_state -- write`.\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Builds on: `{}`\n",
            self.voice_shell_state_contract_ref
        ));
        out.push_str(&format!("- Help / recovery doc: `{}`\n", self.doc_ref));
        out.push_str(&format!("- Fixtures: `{}`\n\n", self.fixtures_dir_ref));

        out.push_str(
            "| Cause | Controlled state | Recovery posture | Severity | Recovery actions |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- |\n");
        for flow in &self.flows {
            let actions = flow
                .recovery_actions
                .iter()
                .map(|action| action.kind.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "| `{}` | {} | {} | {} | {} |\n",
                flow.cause.as_str(),
                flow.lifecycle_state.as_str(),
                flow.recovery_posture.as_str(),
                flow.banner.severity.as_str(),
                actions,
            ));
        }
        out.push('\n');
        out.push_str("## Invariants\n\n");
        let inv = &self.invariants;
        for (label, value) in [
            (
                "Every cause shows a durable banner naming the cause",
                inv.every_cause_has_durable_banner,
            ),
            (
                "Every cause offers a concrete recovery action",
                inv.every_cause_has_concrete_recovery,
            ),
            (
                "Keyboard fallback preserves focus and work",
                inv.keyboard_fallback_preserves_continuity,
            ),
            (
                "Every state is controlled (no silent / oscillating failure)",
                inv.every_state_is_controlled,
            ),
            ("No generic failure copy", inv.no_generic_failure_copy),
            (
                "State changes are narration-safe",
                inv.state_changes_are_narration_safe,
            ),
            (
                "Non-voice recovery affordances preserved",
                inv.nonvoice_recovery_preserved,
            ),
            (
                "All five failure classes covered",
                inv.all_failure_classes_covered,
            ),
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
        serde_json::to_string_pretty(self).expect("voice degraded-state packet serializes")
    }
}

/// Serializes a value as pretty JSON with a trailing newline (the on-disk
/// fixture form).
pub fn fixture_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let mut json = serde_json::to_string_pretty(value)?;
    json.push('\n');
    Ok(json)
}

/// Stable fixture file name for a degraded cause.
pub const fn flow_fixture_file_name(cause: VoiceDegradedCause) -> &'static str {
    match cause {
        VoiceDegradedCause::MissingMicrophoneHardware => "missing-microphone-hardware.json",
        VoiceDegradedCause::NoisyEnvironment => "noisy-environment.json",
        VoiceDegradedCause::ProviderOffline => "provider-offline.json",
        VoiceDegradedCause::LanguagePackMissing => "language-pack-missing.json",
        VoiceDegradedCause::PolicyBlocked => "policy-blocked.json",
    }
}

/// Writes the seeded packet, the per-cause flow fixtures, and the compact
/// summary to `dir`. This is the single mint path the example dump and the
/// equality test share, so the checked-in fixtures can never drift silently.
pub fn write_fixtures(dir: &Path, packet: &VoiceDegradedStatePacket) -> io::Result<()> {
    fs::create_dir_all(dir)?;

    let packet_json =
        fixture_json(packet).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    fs::write(dir.join("packet.json"), packet_json)?;

    for flow in &packet.flows {
        let json = fixture_json(flow).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        fs::write(dir.join(flow_fixture_file_name(flow.cause)), json)?;
    }

    let mut compact = packet.compact_lines().join("\n");
    compact.push('\n');
    fs::write(dir.join("compact.txt"), compact)?;

    Ok(())
}
