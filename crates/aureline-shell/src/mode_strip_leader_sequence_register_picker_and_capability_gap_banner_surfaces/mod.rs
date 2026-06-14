//! Live mode-strip, leader-sequence-guide, register/clipboard-picker, and
//! capability-gap-banner state for every claimed M5 keyboard-first surface.
//!
//! Aureline's switching wedge only holds if the frozen keyboard-mode taxonomy is
//! bound to *real* surfaces rather than asserted in the abstract. The companion
//! continuity matrix in
//! [`crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix`]
//! freezes the canonical vocabulary (mode-strip, sequence-guide, clipboard-route,
//! continuity-parity-grade, proof-currency, downgrade-trigger). This module is the
//! first consumer that renders that vocabulary as the live posture a user actually
//! sees on a notebook, data/API, preview, docs, or companion-adjacent M5 surface:
//! a [`KeymapSourcePreset`], the visible [`ModeIndicator`], a [`PendingSequenceState`]
//! with its operator/sequence tokens and [`SequenceTimeoutPosture`], a
//! [`RegisterClipboardPicker`], a [`ModeStripAccessibility`] block, and any
//! [`CapabilityGapBanner`] that explains why the surface narrowed or rejected a
//! sequence.
//!
//! * a [`SurfaceModeStrip`] ties a durable surface subject (reusing the frozen
//!   [`KeyboardSurfaceSubject`]) to its live keyboard posture and a claimed and
//!   effective [`ContinuityParityGrade`];
//! * each strip is **verification-bound, not asserted**: its reused
//!   [`AxisVerification`] names an [`AxisProofCurrency`] and, unless the proof is
//!   missing, a reopenable proof ref keyed by a non-display fingerprint, so help,
//!   accessibility, and support tooling can reopen the same evidence that backs the
//!   posture;
//! * the strip **auto-downgrades**: [`SurfaceModeStrip::needs_downgrade`] is true
//!   whenever the visible mode cannot be identified, a claimed leader sequence is
//!   unsupported, a clipboard route drops plain text, the surface stops being
//!   keyboard-complete or its macro replay stops being explicit, or its
//!   verification proof is stale, missing, review-pending, or imported proof
//!   standing in for a local claim. A downgraded strip must carry an effective
//!   grade strictly below its claim, a recorded [`ParityDowngradeTrigger`], a
//!   precise label, and at least one [`CapabilityGapBanner`] explaining the
//!   narrowing — never a silent approximation.
//!
//! [`ModeStripSurfacePacket::validate`] also refuses a packet that lets an
//! unsupported modal sequence read as silently approximated, lets a rich-only copy
//! become the only clipboard representation, hides a capability gap behind
//! hover-only UI, or leaves mode changes and sequence ambiguity unreachable by
//! keyboard or screen reader.
//!
//! Raw clipboard bodies, raw key buffers, raw provider payloads, file contents,
//! private paths, and credentials never cross this boundary; the packet carries
//! only typed class tokens, booleans, opaque ids, fingerprint digests, redaction
//! counts, and reviewable labels.
//!
//! The boundary schema is
//! [`schemas/interaction/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners.schema.json`](../../../../schemas/interaction/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners.schema.json).
//! The contract doc is
//! [`docs/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners.md`](../../../../docs/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners.md).
//! The protected fixture directory is
//! [`fixtures/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners/`](../../../../fixtures/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Re-export the frozen taxonomy this consumer binds, so product, accessibility,
// and support surfaces can name those types through this module rather than
// reaching into the matrix crate by hand.
pub use crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix::{
    AxisProofCurrency, AxisVerification, ClipboardRouteClass, ContinuityParityGrade,
    KeyboardSurfaceKind, KeyboardSurfaceSubject, ModeStripClass, ParityDowngradeTrigger,
    SequenceGuideClass, SurfaceOriginClass,
};

/// Stable record-kind tag carried by [`ModeStripSurfacePacket`].
pub const MODE_STRIP_SURFACE_RECORD_KIND: &str =
    "m5_mode_strip_leader_sequence_register_picker_capability_gap_banner_packet";

/// Schema version for the mode-strip surface packet.
pub const MODE_STRIP_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MODE_STRIP_SURFACE_SCHEMA_REF: &str =
    "schemas/interaction/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners.schema.json";

/// Repo-relative path of the contract doc.
pub const MODE_STRIP_SURFACE_DOC_REF: &str =
    "docs/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners.md";

/// Repo-relative path of the checked support-export artifact.
pub const MODE_STRIP_SURFACE_ARTIFACT_REF: &str =
    "artifacts/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const MODE_STRIP_SURFACE_SUMMARY_REF: &str =
    "artifacts/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners.md";

/// Repo-relative path of the protected fixture directory.
pub const MODE_STRIP_SURFACE_FIXTURE_DIR: &str =
    "fixtures/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners";

/// Repo-relative path of the frozen keyboard-continuity matrix this consumer binds.
pub const FROZEN_KEYBOARD_CONTINUITY_MATRIX_REF: &str =
    "schemas/interaction/freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.schema.json";

/// Keymap source preset a surface advertises. A switching user imports one of
/// these; the strip names it explicitly so keys never silently change meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeymapSourcePreset {
    /// A Vim-style modal preset.
    VimPreset,
    /// A Neovim-style modal preset.
    NeovimPreset,
    /// An Emacs-style chorded preset.
    EmacsPreset,
    /// A Helix-style selection-first modal preset.
    HelixPreset,
    /// A non-modal default keymap with no imported modal layer.
    NonModalDefault,
    /// An imported custom keymap that is read-only locally.
    ImportedCustomPreset,
}

impl KeymapSourcePreset {
    /// Every source preset, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::VimPreset,
        Self::NeovimPreset,
        Self::EmacsPreset,
        Self::HelixPreset,
        Self::NonModalDefault,
        Self::ImportedCustomPreset,
    ];

    /// The imported modal presets the switching wedge must demonstrably support.
    pub const MODAL_PRESETS: [Self; 4] = [
        Self::VimPreset,
        Self::NeovimPreset,
        Self::EmacsPreset,
        Self::HelixPreset,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VimPreset => "vim_preset",
            Self::NeovimPreset => "neovim_preset",
            Self::EmacsPreset => "emacs_preset",
            Self::HelixPreset => "helix_preset",
            Self::NonModalDefault => "non_modal_default",
            Self::ImportedCustomPreset => "imported_custom_preset",
        }
    }
}

/// The visible current mode a surface advertises in its mode strip. A surface that
/// cannot identify its active mode reports [`Self::ModeUnknownDowngraded`] rather
/// than guessing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModeIndicator {
    /// Normal / command-navigation mode.
    NormalMode,
    /// Insert / text-entry mode.
    InsertMode,
    /// Visual / selection mode.
    VisualMode,
    /// Visual-line selection mode.
    VisualLineMode,
    /// Visual-block selection mode.
    VisualBlockMode,
    /// Replace / overwrite mode.
    ReplaceMode,
    /// Operator-pending mode (an operator is awaiting its motion / text object).
    OperatorPendingMode,
    /// Command-line / ex-command mode.
    CommandMode,
    /// Read-only modal navigation, no edits.
    ReadOnlyNavigationMode,
    /// A non-modal, fully keyboard-driven editing posture.
    NonModalEditingMode,
    /// The active mode could not be identified — a downgrade, never a silent guess.
    ModeUnknownDowngraded,
}

impl ModeIndicator {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NormalMode => "normal_mode",
            Self::InsertMode => "insert_mode",
            Self::VisualMode => "visual_mode",
            Self::VisualLineMode => "visual_line_mode",
            Self::VisualBlockMode => "visual_block_mode",
            Self::ReplaceMode => "replace_mode",
            Self::OperatorPendingMode => "operator_pending_mode",
            Self::CommandMode => "command_mode",
            Self::ReadOnlyNavigationMode => "read_only_navigation_mode",
            Self::NonModalEditingMode => "non_modal_editing_mode",
            Self::ModeUnknownDowngraded => "mode_unknown_downgraded",
        }
    }

    /// Whether the visible mode is honestly downgraded rather than identified.
    pub const fn is_downgraded(self) -> bool {
        matches!(self, Self::ModeUnknownDowngraded)
    }
}

/// How a surface resolves a pending multi-key / leader sequence when ambiguity or a
/// timeout is in play.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SequenceTimeoutPosture {
    /// No sequence is pending; nothing is being held.
    NoPendingSequence,
    /// The surface waits for explicit completion with no timeout.
    WaitsForExplicitCompletion,
    /// A timeout discards the pending sequence rather than guessing.
    TimeoutCancelsPending,
    /// A timeout commits the longest unambiguous prefix and discards the rest.
    TimeoutCommitsLongestPrefix,
    /// Ambiguity is held open for an explicit user choice (a discovery overlay).
    AmbiguityHeldForExplicitChoice,
}

impl SequenceTimeoutPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPendingSequence => "no_pending_sequence",
            Self::WaitsForExplicitCompletion => "waits_for_explicit_completion",
            Self::TimeoutCancelsPending => "timeout_cancels_pending",
            Self::TimeoutCommitsLongestPrefix => "timeout_commits_longest_prefix",
            Self::AmbiguityHeldForExplicitChoice => "ambiguity_held_for_explicit_choice",
        }
    }
}

/// Resolution state of a pending sequence. An unsupported sequence is downgraded
/// explicitly rather than silently approximated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SequenceResolution {
    /// No sequence is pending.
    NoPendingSequence,
    /// The pending input maps to exactly one unambiguous binding.
    Unambiguous,
    /// More than one binding matches; awaiting disambiguation input or choice.
    AwaitingDisambiguation,
    /// A timeout / longest-match rule resolved the sequence.
    ResolvedLongestMatch,
    /// The claimed sequence is unsupported here — a downgrade, never an approximation.
    UnsupportedDowngraded,
}

impl SequenceResolution {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPendingSequence => "no_pending_sequence",
            Self::Unambiguous => "unambiguous",
            Self::AwaitingDisambiguation => "awaiting_disambiguation",
            Self::ResolvedLongestMatch => "resolved_longest_match",
            Self::UnsupportedDowngraded => "unsupported_downgraded",
        }
    }

    /// Whether this resolution is an honest unsupported downgrade.
    pub const fn is_downgraded(self) -> bool {
        matches!(self, Self::UnsupportedDowngraded)
    }
}

/// Why a surface narrowed or rejected a claimed keyboard affordance. The banner
/// names a specific reason so support and migration tooling can explain it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityGapKind {
    /// A claimed modal mode is unsupported on this surface.
    ModalModeUnsupported,
    /// A claimed multi-key / leader sequence is unsupported on this surface.
    ModalSequenceUnsupported,
    /// Named-register routing is unavailable; copy narrows to a single clipboard.
    NamedRegisterUnsupported,
    /// A useful plain-text clipboard representation is unavailable.
    ClipboardPlainTextUnavailable,
    /// A drag/drop verb narrowed (e.g. move unavailable, copy/link only).
    DragDropVerbNarrowed,
    /// The sequence timeout posture narrowed from the imported preset's default.
    TimeoutPostureNarrowed,
    /// Orientation aids reduced honestly (fewer aids than the imported preset).
    OrientationAidReduced,
    /// Macro replay is unavailable on this surface.
    MacroReplayUnavailable,
}

impl CapabilityGapKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModalModeUnsupported => "modal_mode_unsupported",
            Self::ModalSequenceUnsupported => "modal_sequence_unsupported",
            Self::NamedRegisterUnsupported => "named_register_unsupported",
            Self::ClipboardPlainTextUnavailable => "clipboard_plain_text_unavailable",
            Self::DragDropVerbNarrowed => "drag_drop_verb_narrowed",
            Self::TimeoutPostureNarrowed => "timeout_posture_narrowed",
            Self::OrientationAidReduced => "orientation_aid_reduced",
            Self::MacroReplayUnavailable => "macro_replay_unavailable",
        }
    }
}

/// Whether a capability gap narrowed to a supported subset or rejected the input
/// outright. Either disposition is explicit; neither is a silent approximation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapDisposition {
    /// The surface fell back to a clearly stated supported subset.
    NarrowedToSupportedSubset,
    /// The surface rejected the input rather than approximating it.
    RejectedOutright,
}

impl GapDisposition {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NarrowedToSupportedSubset => "narrowed_to_supported_subset",
            Self::RejectedOutright => "rejected_outright",
        }
    }
}

/// A live pending-sequence state: the operator / sequence tokens in flight, the
/// numeric count prefix, the timeout posture, and the resolution. Tokens are
/// reviewable guide labels, never raw key buffers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingSequenceState {
    /// Frozen sequence-guide class this surface realizes.
    pub sequence_guide: SequenceGuideClass,
    /// Reviewable operator token in flight (e.g. `operator:change`), if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_operator_token: Option<String>,
    /// Reviewable pending-sequence guide token (e.g. `guide:leader>g>d`), if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_sequence_token: Option<String>,
    /// Numeric count prefix accumulated so far, if any. A bare number, never text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count_prefix: Option<u32>,
    /// Whether the surface is awaiting further input to complete the sequence.
    pub awaiting_more_input: bool,
    /// Timeout posture governing the pending sequence.
    pub timeout_posture: SequenceTimeoutPosture,
    /// Resolution state of the pending sequence.
    pub resolution: SequenceResolution,
    /// Whether the pending sequence is announced to assistive technology.
    pub sequence_guide_announced: bool,
}

impl PendingSequenceState {
    /// Whether the sequence axis is honestly downgraded rather than supported.
    pub fn is_downgraded(&self) -> bool {
        self.sequence_guide.is_downgraded() || self.resolution.is_downgraded()
    }

    /// Whether any pending operator, sequence, or count is in flight.
    pub fn has_pending_input(&self) -> bool {
        self.pending_operator_token.is_some()
            || self.pending_sequence_token.is_some()
            || self.count_prefix.is_some()
    }

    /// Whether the pending-sequence state is internally consistent: a downgrade is
    /// explicit and aligned, supported state never claims the unsupported token, and
    /// pending input pairs with a real timeout/resolution posture.
    pub fn is_well_formed(&self) -> bool {
        if self
            .pending_operator_token
            .as_ref()
            .is_some_and(|token| token.trim().is_empty())
        {
            return false;
        }
        if self
            .pending_sequence_token
            .as_ref()
            .is_some_and(|token| token.trim().is_empty())
        {
            return false;
        }

        if self.is_downgraded() {
            // An unsupported sequence downgrades both the guide class and the
            // resolution together, so it can never read as a partial approximation.
            return self.sequence_guide == SequenceGuideClass::SequenceUnsupportedDowngraded
                && self.resolution == SequenceResolution::UnsupportedDowngraded;
        }

        if self.resolution == SequenceResolution::UnsupportedDowngraded {
            return false;
        }

        if self.has_pending_input() {
            self.awaiting_more_input
                && self.timeout_posture != SequenceTimeoutPosture::NoPendingSequence
                && self.resolution != SequenceResolution::NoPendingSequence
        } else {
            !self.awaiting_more_input
                && self.timeout_posture == SequenceTimeoutPosture::NoPendingSequence
                && matches!(
                    self.resolution,
                    SequenceResolution::NoPendingSequence | SequenceResolution::Unambiguous
                )
        }
    }
}

/// A live register / clipboard-route picker: the active and selectable register
/// tokens and the plain-text / sensitive-copy posture. Register tokens are typed
/// labels, never clipboard bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterClipboardPicker {
    /// Frozen clipboard-route class this surface realizes.
    pub clipboard_route: ClipboardRouteClass,
    /// Active register token (e.g. `register:unnamed`), if a register is selected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_register_token: Option<String>,
    /// Selectable register tokens offered by the picker.
    pub available_register_tokens: Vec<String>,
    /// Whether a useful plain-text representation is available to paste/export.
    pub plain_text_representation_available: bool,
    /// Whether a sensitive-copy warning is active for the current selection.
    pub sensitive_copy_warning_active: bool,
    /// Whether the picker is reachable by keyboard (not hover/pointer-only).
    pub picker_keyboard_reachable: bool,
}

impl RegisterClipboardPicker {
    /// Whether the route denies a useful plain-text representation.
    pub fn is_denied(&self) -> bool {
        self.clipboard_route.is_denied()
    }

    /// Whether plain-text availability matches the route: every route except the
    /// denied rich-only route preserves a plain-text representation.
    pub fn plain_text_consistent(&self) -> bool {
        self.plain_text_representation_available != self.clipboard_route.is_denied()
    }

    /// Whether the picker is well-formed: plain-text availability matches the route,
    /// the picker is keyboard-reachable, register tokens are non-empty, a named-
    /// register route offers at least one register, and any active register is one
    /// of the offered tokens.
    pub fn is_well_formed(&self) -> bool {
        if !self.plain_text_consistent() || !self.picker_keyboard_reachable {
            return false;
        }
        if self
            .available_register_tokens
            .iter()
            .any(|token| token.trim().is_empty())
        {
            return false;
        }
        if self.clipboard_route == ClipboardRouteClass::NamedRegisterRouted
            && self.available_register_tokens.is_empty()
        {
            return false;
        }
        if let Some(active) = &self.active_register_token {
            if active.trim().is_empty()
                || !self.available_register_tokens.iter().any(|t| t == active)
            {
                return false;
            }
        }
        true
    }
}

/// A capability-gap banner: a precise, reachable explanation of why a surface
/// narrowed or rejected a claimed keyboard affordance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityGapBanner {
    /// Stable gap id.
    pub gap_id: String,
    /// Which affordance narrowed or was rejected.
    pub gap_kind: CapabilityGapKind,
    /// Whether the surface narrowed to a supported subset or rejected the input.
    pub disposition: GapDisposition,
    /// Precise, non-generic explanation shown to the user.
    pub explanation: String,
    /// Export-safe reason token for support / migration tooling.
    pub export_safe_reason_token: String,
    /// Whether the banner is reachable by keyboard.
    pub keyboard_reachable: bool,
    /// Whether the banner is announced to assistive technology.
    pub screen_reader_announced: bool,
    /// Whether the banner is shown only on hover / pointer focus — must be false.
    pub hover_only: bool,
}

impl CapabilityGapBanner {
    /// Whether the banner is reachable without relying on hover-only UI.
    pub fn is_reachable(&self) -> bool {
        self.keyboard_reachable && self.screen_reader_announced && !self.hover_only
    }

    /// Whether the banner is well-formed: ids and reason token present, explanation
    /// precise (not a generic non-answer), and reachable by keyboard and screen
    /// reader without hover-only UI.
    pub fn is_well_formed(&self) -> bool {
        !self.gap_id.trim().is_empty()
            && !self.export_safe_reason_token.trim().is_empty()
            && !label_is_generic(&self.explanation)
            && self.is_reachable()
    }
}

/// Accessibility posture of a surface's mode strip and sequence guide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeStripAccessibility {
    /// Whether mode changes are announced to assistive technology.
    pub mode_change_announced: bool,
    /// Whether the pending sequence is announced to assistive technology.
    pub pending_sequence_announced: bool,
    /// Whether the mode strip and sequence guide are reachable by keyboard.
    pub keyboard_reachable: bool,
    /// Whether the strip is shown only on hover / pointer focus — must be false.
    pub hover_only: bool,
}

impl ModeStripAccessibility {
    /// Whether mode changes and sequence ambiguity are keyboard- and screen-reader-
    /// reachable without relying on hover-only UI.
    pub fn all_reachable(&self) -> bool {
        self.mode_change_announced
            && self.pending_sequence_announced
            && self.keyboard_reachable
            && !self.hover_only
    }
}

/// One claimed M5 surface's live keyboard posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceModeStrip {
    /// Stable strip id.
    pub strip_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the strip covers (reused frozen subject).
    pub subject: KeyboardSurfaceSubject,
    /// Human-readable strip label.
    pub label_summary: String,
    /// Keymap source preset the surface advertises.
    pub source_preset: KeymapSourcePreset,
    /// The visible current mode.
    pub current_mode: ModeIndicator,
    /// Frozen mode-strip class this surface realizes.
    pub mode_strip_class: ModeStripClass,
    /// Whether the surface remains keyboard-complete (no mouse-only fallback).
    pub keyboard_complete: bool,
    /// Whether mode changes and macro replay are explicit on this surface.
    pub macro_replay_explicit: bool,
    /// Live pending-sequence state.
    pub pending_sequence: PendingSequenceState,
    /// Live register / clipboard-route picker.
    pub register_picker: RegisterClipboardPicker,
    /// Accessibility posture of the mode strip and sequence guide.
    pub accessibility: ModeStripAccessibility,
    /// Capability-gap banners explaining any narrowing or rejection.
    pub capability_gaps: Vec<CapabilityGapBanner>,
    /// Reopenable verification proof backing the posture (reused frozen proof).
    pub verification: AxisVerification,
    /// Headline parity grade publicly claimed for this surface.
    pub claimed_grade: ContinuityParityGrade,
    /// Effective grade after auto-downgrading; equals the claim when every axis is
    /// supported and the proof is current, and ranks strictly below it otherwise.
    pub effective_grade: ContinuityParityGrade,
    /// Trigger that fired the downgrade, required when the strip is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<ParityDowngradeTrigger>,
    /// Precise downgraded label, required when the strip is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgraded_label: Option<String>,
    /// Evidence packet refs backing this strip.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this strip.
    pub source_contract_refs: Vec<String>,
}

impl SurfaceModeStrip {
    /// Whether parity for this strip is provider-backed / imported.
    pub fn provider_or_imported(&self) -> bool {
        self.subject.is_provider_or_imported()
    }

    /// Whether the strip carries a public parity claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_grade.is_claimed()
    }

    /// Whether the visible mode axis is honestly downgraded rather than identified.
    pub fn mode_is_downgraded(&self) -> bool {
        self.current_mode.is_downgraded() || self.mode_strip_class.is_downgraded()
    }

    /// Whether any interaction axis is in an honestly-downgraded or denied state.
    pub fn any_axis_downgraded(&self) -> bool {
        self.mode_is_downgraded()
            || self.pending_sequence.is_downgraded()
            || self.register_picker.is_denied()
    }

    /// Whether the verification proof backs a current claim for this strip's origin.
    pub fn verification_current(&self) -> bool {
        self.verification.backs_claim(self.provider_or_imported())
    }

    /// Whether the strip must downgrade below its claim because an axis is denied or
    /// downgraded, the surface stopped being keyboard-complete or macro-explicit, or
    /// the verification proof is not current.
    pub fn needs_downgrade(&self) -> bool {
        self.any_axis_downgraded()
            || !self.verification_current()
            || !self.keyboard_complete
            || !self.macro_replay_explicit
    }

    /// Whether the effective grade ranks strictly below the claim.
    pub fn properly_downgraded(&self) -> bool {
        self.effective_grade.rank() < self.claimed_grade.rank()
    }

    /// Whether a capability-gap banner explains every narrowed or rejected
    /// affordance: a surface that narrows or rejects a mode, sequence, or clipboard
    /// route never does so silently. Verification-freshness downgrades are governed
    /// separately and do not require an affordance banner.
    pub fn gap_explains_downgrade(&self) -> bool {
        !self.any_axis_downgraded() || !self.capability_gaps.is_empty()
    }

    /// Whether the effective grade and downgrade evidence are consistent.
    ///
    /// When the strip does not need downgrade the effective grade equals the claim;
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

    /// Whether the imported posture is consistent: a provider/imported surface never
    /// reads as a locally verified surface, and a local surface never leans on
    /// imported proof.
    pub fn imported_posture_consistent(&self) -> bool {
        if self.provider_or_imported() {
            !self.verification.proof_currency.is_current_local()
        } else {
            !self.verification.proof_currency.is_imported_current()
        }
    }

    /// Whether every field required to record this strip is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.strip_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.subject.is_valid()
            && self.verification.is_well_formed()
            && self.pending_sequence.is_well_formed()
            && self.register_picker.is_well_formed()
            && self.accessibility.all_reachable()
            && self.capability_gaps.iter().all(|gap| gap.is_well_formed())
            && self.gap_explains_downgrade()
            && self.downgrade_consistent()
            && self.imported_posture_consistent()
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
pub struct ModeStripGuardrails {
    /// The visible current mode is always preserved, never silently guessed.
    pub visible_current_mode_preserved: bool,
    /// A pending sequence is never silently approximated.
    pub pending_sequence_never_silently_approximated: bool,
    /// The register / clipboard route is always visible and keyboard-reachable.
    pub register_clipboard_route_visible: bool,
    /// Capability gaps are explained, never hidden behind hover-only UI.
    pub capability_gaps_explained_not_hidden: bool,
    /// Mode changes and sequence ambiguity are keyboard- and screen-reader-reachable.
    pub mode_changes_keyboard_and_screen_reader_reachable: bool,
    /// Any claimed surface lacking identified behavior or current proof
    /// auto-downgrades below its claim.
    pub rows_auto_downgrade_without_identified_behavior: bool,
}

impl ModeStripGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.visible_current_mode_preserved
            && self.pending_sequence_never_silently_approximated
            && self.register_clipboard_route_visible
            && self.capability_gaps_explained_not_hidden
            && self.mode_changes_keyboard_and_screen_reader_reachable
            && self.rows_auto_downgrade_without_identified_behavior
    }
}

/// Consumer projection block: the surfaces that read these strips without cloning
/// switching-wedge language by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeStripConsumerProjection {
    /// Product surfaces ingest these strips.
    pub product_ingests_strips: bool,
    /// Help / migration guidance reconstructs the exact posture from these strips.
    pub help_migration_reconstructs_posture: bool,
    /// Accessibility surfaces ingest these strips.
    pub accessibility_ingests_strips: bool,
    /// Diagnostics surfaces ingest these strips.
    pub diagnostics_ingests_strips: bool,
    /// Support exports reconstruct the capability-gap posture from these strips.
    pub support_export_reconstructs_capability_gaps: bool,
    /// Downgraded strips are visibly labeled below their claim in every surface.
    pub downgraded_strips_labeled_below_claim: bool,
}

impl ModeStripConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_strips
            && self.help_migration_reconstructs_posture
            && self.accessibility_ingests_strips
            && self.diagnostics_ingests_strips
            && self.support_export_reconstructs_capability_gaps
            && self.downgraded_strips_labeled_below_claim
    }
}

/// Verification freshness block for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeStripFreshness {
    /// Verification-freshness SLO in hours.
    pub verification_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last verification refresh.
    pub last_verification_refresh: String,
    /// True when stale verification automatically downgrades claimed strips.
    pub auto_downgrade_on_stale: bool,
}

impl ModeStripFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.verification_freshness_slo_hours > 0
            && !self.last_verification_refresh.trim().is_empty()
    }
}

/// Constructor input for [`ModeStripSurfacePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModeStripSurfacePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-surface mode strips.
    pub strips: Vec<SurfaceModeStrip>,
    /// Guardrail invariants block.
    pub guardrails: ModeStripGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ModeStripConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: ModeStripFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe mode-strip surface packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeStripSurfacePacket {
    /// Record kind; must equal [`MODE_STRIP_SURFACE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MODE_STRIP_SURFACE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-surface mode strips.
    pub strips: Vec<SurfaceModeStrip>,
    /// Guardrail invariants block.
    pub guardrails: ModeStripGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ModeStripConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: ModeStripFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ModeStripSurfacePacket {
    /// Builds a mode-strip surface packet.
    pub fn new(input: ModeStripSurfacePacketInput) -> Self {
        Self {
            record_kind: MODE_STRIP_SURFACE_RECORD_KIND.to_owned(),
            schema_version: MODE_STRIP_SURFACE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            strips: input.strips,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            verification_freshness: input.verification_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surface kinds represented by some strip in this packet.
    pub fn represented_surface_kinds(&self) -> BTreeSet<KeyboardSurfaceKind> {
        self.strips.iter().map(|strip| strip.surface_kind).collect()
    }

    /// Source presets represented across strips.
    pub fn represented_source_presets(&self) -> BTreeSet<KeymapSourcePreset> {
        self.strips
            .iter()
            .map(|strip| strip.source_preset)
            .collect()
    }

    /// Capability-gap kinds represented across strips.
    pub fn represented_gap_kinds(&self) -> BTreeSet<CapabilityGapKind> {
        self.strips
            .iter()
            .flat_map(|strip| strip.capability_gaps.iter().map(|gap| gap.gap_kind))
            .collect()
    }

    /// Count of strips that auto-downgraded below their claim.
    pub fn downgraded_strip_count(&self) -> usize {
        self.strips
            .iter()
            .filter(|strip| strip.needs_downgrade())
            .count()
    }

    /// Count of strips holding a public parity claim.
    pub fn claimed_strip_count(&self) -> usize {
        self.strips
            .iter()
            .filter(|strip| strip.is_claimed())
            .count()
    }

    /// Count of provider-linked / imported strips.
    pub fn provider_or_imported_strip_count(&self) -> usize {
        self.strips
            .iter()
            .filter(|strip| strip.provider_or_imported())
            .count()
    }

    /// Resolves a strip by its id.
    pub fn strip(&self, strip_id: &str) -> Option<&SurfaceModeStrip> {
        self.strips.iter().find(|strip| strip.strip_id == strip_id)
    }

    /// Validates the mode-strip surface invariants.
    pub fn validate(&self) -> Vec<ModeStripViolation> {
        let mut violations = Vec::new();

        if self.record_kind != MODE_STRIP_SURFACE_RECORD_KIND {
            violations.push(ModeStripViolation::WrongRecordKind);
        }
        if self.schema_version != MODE_STRIP_SURFACE_SCHEMA_VERSION {
            violations.push(ModeStripViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ModeStripViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_strips(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(ModeStripViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ModeStripViolation::ConsumerProjectionIncomplete);
        }
        if !self.verification_freshness.is_valid() {
            violations.push(ModeStripViolation::VerificationFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("mode strip surface packet serializes"),
        ) {
            violations.push(ModeStripViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("mode strip surface packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, accessibility, or release
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Mode-Strip / Leader-Sequence-Guide / Register-Picker / Capability-Gap Surfaces\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Strips: {} ({} claimed, {} provider/imported, {} downgraded)\n",
            self.strips.len(),
            self.claimed_strip_count(),
            self.provider_or_imported_strip_count(),
            self.downgraded_strip_count()
        ));
        out.push_str(&format!(
            "- Surface kinds: {} / {}\n",
            self.represented_surface_kinds().len(),
            KeyboardSurfaceKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Source presets: {} / {}\n",
            self.represented_source_presets().len(),
            KeymapSourcePreset::ALL.len()
        ));
        out.push_str(&format!(
            "- Verification freshness SLO: {} hours (last refresh: {})\n",
            self.verification_freshness.verification_freshness_slo_hours,
            self.verification_freshness.last_verification_refresh
        ));
        out.push_str("\n## Strips\n\n");
        for strip in &self.strips {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                strip.strip_id,
                strip.surface_kind.as_str(),
                strip.claimed_grade.as_str(),
                strip.effective_grade.as_str()
            ));
            out.push_str(&format!("  - {}\n", strip.label_summary));
            out.push_str(&format!(
                "  - subject `{}` ({}), keyboard_complete={}, macro_replay_explicit={}\n",
                strip.subject.surface_id,
                strip.subject.origin_class.as_str(),
                strip.keyboard_complete,
                strip.macro_replay_explicit
            ));
            out.push_str(&format!(
                "  - source_preset = `{}`, current_mode = `{}`, mode_strip = `{}`\n",
                strip.source_preset.as_str(),
                strip.current_mode.as_str(),
                strip.mode_strip_class.as_str()
            ));
            out.push_str(&format!(
                "  - sequence_guide = `{}`, timeout = `{}`, resolution = `{}`\n",
                strip.pending_sequence.sequence_guide.as_str(),
                strip.pending_sequence.timeout_posture.as_str(),
                strip.pending_sequence.resolution.as_str()
            ));
            out.push_str(&format!(
                "  - clipboard_route = `{}`, plain_text_available={}, registers={}\n",
                strip.register_picker.clipboard_route.as_str(),
                strip.register_picker.plain_text_representation_available,
                strip.register_picker.available_register_tokens.len()
            ));
            out.push_str(&format!(
                "  - verification = `{}`\n",
                strip.verification.proof_currency.as_str()
            ));
            for gap in &strip.capability_gaps {
                out.push_str(&format!(
                    "  - capability gap `{}` ({}): {}\n",
                    gap.gap_kind.as_str(),
                    gap.disposition.as_str(),
                    gap.explanation
                ));
            }
            if let Some(label) = &strip.downgraded_label {
                out.push_str(&format!("  - Downgraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum ModeStripArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ModeStripViolation>),
}

impl fmt::Display for ModeStripArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "mode strip surface export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "mode strip surface export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ModeStripArtifactError {}

/// Validation failures emitted by [`ModeStripSurfacePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModeStripViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed surface kind is represented by no strip.
    RequiredSurfaceKindMissing,
    /// The imported modal source presets are not all represented, so the matrix
    /// cannot prove it supports the switching wedge.
    SourcePresetCoverageMissing,
    /// No strip demonstrates an honest capability-gap downgrade.
    CapabilityGapCaseMissing,
    /// No fully keyboard-complete, current strip anchors a clean claim.
    KeyboardCompleteCaseMissing,
    /// No provider-linked / imported strip is present.
    ProviderOrImportedCaseMissing,
    /// A strip is incomplete.
    StripIncomplete,
    /// A claimed strip was not downgraded below its claim despite unidentified
    /// behavior or uncurrent proof.
    StripNotDowngradedOnUnidentifiedBehavior,
    /// A downgraded strip lacks a precise downgraded label or trigger.
    DowngradedStripMissingLabelOrTrigger,
    /// A strip's subject fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A claimed surface is not keyboard-complete.
    SurfaceNotKeyboardComplete,
    /// A surface's mode changes / macro replay are not explicit.
    MacroReplayNotExplicit,
    /// The visible current mode is unidentified without an honest downgrade.
    CurrentModeNotVisible,
    /// A pending sequence was approximated rather than downgraded honestly.
    SequenceSilentlyApproximated,
    /// A clipboard route dropped the plain-text representation without downgrading.
    ClipboardPlainTextLost,
    /// A downgraded strip narrows or rejects a sequence with no capability-gap banner.
    CapabilityGapMissingForNarrowing,
    /// A capability-gap banner is not keyboard- and screen-reader-reachable.
    CapabilityGapNotReachable,
    /// Mode changes / sequence ambiguity are not keyboard- and screen-reader-reachable.
    ModeChangesNotReachable,
    /// A provider/imported strip reads as a locally verified surface.
    ImportedReadsAsLocal,
    /// A strip's verification proof is not reopenable.
    VerificationProofNotReopenable,
    /// A strip lacks evidence refs.
    StripEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Verification freshness block is incomplete.
    VerificationFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ModeStripViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceKindMissing => "required_surface_kind_missing",
            Self::SourcePresetCoverageMissing => "source_preset_coverage_missing",
            Self::CapabilityGapCaseMissing => "capability_gap_case_missing",
            Self::KeyboardCompleteCaseMissing => "keyboard_complete_case_missing",
            Self::ProviderOrImportedCaseMissing => "provider_or_imported_case_missing",
            Self::StripIncomplete => "strip_incomplete",
            Self::StripNotDowngradedOnUnidentifiedBehavior => {
                "strip_not_downgraded_on_unidentified_behavior"
            }
            Self::DowngradedStripMissingLabelOrTrigger => {
                "downgraded_strip_missing_label_or_trigger"
            }
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::SurfaceNotKeyboardComplete => "surface_not_keyboard_complete",
            Self::MacroReplayNotExplicit => "macro_replay_not_explicit",
            Self::CurrentModeNotVisible => "current_mode_not_visible",
            Self::SequenceSilentlyApproximated => "sequence_silently_approximated",
            Self::ClipboardPlainTextLost => "clipboard_plain_text_lost",
            Self::CapabilityGapMissingForNarrowing => "capability_gap_missing_for_narrowing",
            Self::CapabilityGapNotReachable => "capability_gap_not_reachable",
            Self::ModeChangesNotReachable => "mode_changes_not_reachable",
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::VerificationProofNotReopenable => "verification_proof_not_reopenable",
            Self::StripEvidenceMissing => "strip_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::VerificationFreshnessIncomplete => "verification_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Surface kinds this consumer must bind to make keyboard-mode parity authoritative
/// on the new M5 surfaces named by the spec.
const REQUIRED_SURFACE_KINDS: [KeyboardSurfaceKind; 5] = [
    KeyboardSurfaceKind::NotebookSurface,
    KeyboardSurfaceKind::DataApiSurface,
    KeyboardSurfaceKind::PreviewSurface,
    KeyboardSurfaceKind::DocsSurface,
    KeyboardSurfaceKind::CompanionSurface,
];

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_mode_strip_surface_export() -> Result<ModeStripSurfacePacket, ModeStripArtifactError>
{
    let packet: ModeStripSurfacePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners/support_export.json"
    )))
    .map_err(ModeStripArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ModeStripArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ModeStripSurfacePacket,
    violations: &mut Vec<ModeStripViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        MODE_STRIP_SURFACE_SCHEMA_REF,
        MODE_STRIP_SURFACE_DOC_REF,
        MODE_STRIP_SURFACE_ARTIFACT_REF,
        FROZEN_KEYBOARD_CONTINUITY_MATRIX_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ModeStripViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(packet: &ModeStripSurfacePacket, violations: &mut Vec<ModeStripViolation>) {
    let surface_kinds = packet.represented_surface_kinds();
    for required in REQUIRED_SURFACE_KINDS {
        if !surface_kinds.contains(&required) {
            violations.push(ModeStripViolation::RequiredSurfaceKindMissing);
            break;
        }
    }

    let presets = packet.represented_source_presets();
    for required in KeymapSourcePreset::MODAL_PRESETS {
        if !presets.contains(&required) {
            violations.push(ModeStripViolation::SourcePresetCoverageMissing);
            break;
        }
    }

    if !packet.strips.iter().any(|strip| {
        strip.needs_downgrade() && strip.downgrade_consistent() && !strip.capability_gaps.is_empty()
    }) {
        violations.push(ModeStripViolation::CapabilityGapCaseMissing);
    }

    if !packet.strips.iter().any(|strip| {
        !strip.needs_downgrade()
            && strip.is_claimed()
            && strip.keyboard_complete
            && strip.verification_current()
    }) {
        violations.push(ModeStripViolation::KeyboardCompleteCaseMissing);
    }

    if packet.provider_or_imported_strip_count() == 0 {
        violations.push(ModeStripViolation::ProviderOrImportedCaseMissing);
    }
}

fn validate_strips(packet: &ModeStripSurfacePacket, violations: &mut Vec<ModeStripViolation>) {
    for strip in &packet.strips {
        if !strip.is_complete() {
            violations.push(ModeStripViolation::StripIncomplete);
        }
        if strip.needs_downgrade() && !strip.properly_downgraded() {
            violations.push(ModeStripViolation::StripNotDowngradedOnUnidentifiedBehavior);
        }
        if strip.needs_downgrade()
            && (strip.downgrade_trigger.is_none()
                || !strip
                    .downgraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations.push(ModeStripViolation::DowngradedStripMissingLabelOrTrigger);
        }
        if !strip.subject.fingerprint_independent_of_id() {
            violations.push(ModeStripViolation::FingerprintSubstitutesIdentity);
        }
        if !strip.keyboard_complete {
            violations.push(ModeStripViolation::SurfaceNotKeyboardComplete);
        }
        if !strip.macro_replay_explicit {
            violations.push(ModeStripViolation::MacroReplayNotExplicit);
        }
        if strip.mode_is_downgraded() && !strip.properly_downgraded() {
            violations.push(ModeStripViolation::CurrentModeNotVisible);
        }
        if strip.pending_sequence.is_downgraded() && !strip.properly_downgraded() {
            violations.push(ModeStripViolation::SequenceSilentlyApproximated);
        }
        if strip.register_picker.is_denied() && !strip.properly_downgraded() {
            violations.push(ModeStripViolation::ClipboardPlainTextLost);
        }
        if !strip.gap_explains_downgrade() {
            violations.push(ModeStripViolation::CapabilityGapMissingForNarrowing);
        }
        if strip.capability_gaps.iter().any(|gap| !gap.is_reachable()) {
            violations.push(ModeStripViolation::CapabilityGapNotReachable);
        }
        if !strip.accessibility.all_reachable() {
            violations.push(ModeStripViolation::ModeChangesNotReachable);
        }
        if !strip.imported_posture_consistent() {
            violations.push(ModeStripViolation::ImportedReadsAsLocal);
        }
        if !strip.verification.is_well_formed() {
            violations.push(ModeStripViolation::VerificationProofNotReopenable);
        }
        if strip.evidence_refs.is_empty() || strip.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(ModeStripViolation::StripEvidenceMissing);
        }
    }
}

/// Whether a downgraded / capability-gap label is a generic non-answer rather than a
/// precise explanation. A generic provider error must never stand in for a precise
/// downgrade truth.
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
            | "unsupported"
    )
}

/// Stable packet id minted by [`seeded_mode_strip_surface_packet`].
pub const SEED_MODE_STRIP_PACKET_ID: &str = "m5-mode-strip-surfaces:stable:0001";

/// Mint timestamp used by [`seeded_mode_strip_surface_packet`].
pub const SEED_MODE_STRIP_MINTED_AT: &str = "2026-06-14T00:00:00Z";

/// Builds the canonical, validating mode-strip surface packet that the checked-in
/// support export, the Markdown summary, and the conformance tests all share, so the
/// in-crate builder stays byte-aligned with the artifact.
///
/// The seed binds every named M5 surface (notebook, data/API, preview, docs,
/// companion) plus editor anchors to live mode-strip, leader-sequence, register/
/// clipboard, and capability-gap state; covers every imported modal preset
/// (Vim/Neovim/Emacs/Helix); holds one provider-linked surface that never reads as a
/// local rerun; carries one surface mid-sequence with a live pending operator and
/// count prefix; and includes one surface that auto-downgrades honestly because a
/// claimed leader sequence is unsupported, with a capability-gap banner explaining
/// the narrowing rather than silently approximating the sequence.
pub fn seeded_mode_strip_surface_packet() -> ModeStripSurfacePacket {
    ModeStripSurfacePacket::new(ModeStripSurfacePacketInput {
        packet_id: SEED_MODE_STRIP_PACKET_ID.to_owned(),
        label: "M5 Mode-Strip / Leader-Sequence / Register-Picker / Capability-Gap Surfaces"
            .to_owned(),
        strips: seeded_strips(),
        guardrails: ModeStripGuardrails {
            visible_current_mode_preserved: true,
            pending_sequence_never_silently_approximated: true,
            register_clipboard_route_visible: true,
            capability_gaps_explained_not_hidden: true,
            mode_changes_keyboard_and_screen_reader_reachable: true,
            rows_auto_downgrade_without_identified_behavior: true,
        },
        consumer_projection: ModeStripConsumerProjection {
            product_ingests_strips: true,
            help_migration_reconstructs_posture: true,
            accessibility_ingests_strips: true,
            diagnostics_ingests_strips: true,
            support_export_reconstructs_capability_gaps: true,
            downgraded_strips_labeled_below_claim: true,
        },
        verification_freshness: ModeStripFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_MODE_STRIP_MINTED_AT.to_owned(),
            auto_downgrade_on_stale: true,
        },
        source_contract_refs: vec![
            MODE_STRIP_SURFACE_SCHEMA_REF.to_owned(),
            MODE_STRIP_SURFACE_DOC_REF.to_owned(),
            MODE_STRIP_SURFACE_ARTIFACT_REF.to_owned(),
            FROZEN_KEYBOARD_CONTINUITY_MATRIX_REF.to_owned(),
            "shell:interaction_transfer_beta:v1".to_owned(),
            "shell:interaction_integrity_beta:v1".to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_MODE_STRIP_MINTED_AT.to_owned(),
    })
}

fn seeded_strips() -> Vec<SurfaceModeStrip> {
    vec![
        editor_core_strip(),
        notebook_strip(),
        data_api_strip(),
        preview_strip(),
        docs_pending_sequence_strip(),
        companion_provider_strip(),
        data_api_unsupported_sequence_strip(),
    ]
}

/// Shared idle-sequence state for a surface with nothing pending.
fn idle_sequence(guide: SequenceGuideClass) -> PendingSequenceState {
    PendingSequenceState {
        sequence_guide: guide,
        pending_operator_token: None,
        pending_sequence_token: None,
        count_prefix: None,
        awaiting_more_input: false,
        timeout_posture: SequenceTimeoutPosture::NoPendingSequence,
        resolution: SequenceResolution::NoPendingSequence,
        sequence_guide_announced: true,
    }
}

/// Reachable accessibility posture used by every clean seed strip.
fn full_accessibility() -> ModeStripAccessibility {
    ModeStripAccessibility {
        mode_change_announced: true,
        pending_sequence_announced: true,
        keyboard_reachable: true,
        hover_only: false,
    }
}

/// Reopenable verification proof for a strip keyed by its id.
fn proof_for(strip_id: &str, currency: AxisProofCurrency, surface: &str) -> AxisVerification {
    let (proof_ref, proof_fingerprint_token) = if currency.is_absent() {
        (None, None)
    } else {
        (
            Some(format!("evidence:{strip_id}")),
            Some(format!("fp:proof:{strip_id}")),
        )
    };
    AxisVerification {
        proof_currency: currency,
        proof_ref,
        proof_fingerprint_token,
        summary: format!(
            "{surface} live mode-strip posture verified with {} proof",
            currency.as_str()
        ),
    }
}

/// Durable subject keyed by a strip id, with a fingerprint distinct from the id.
fn subject_for(strip_id: &str, origin: SurfaceOriginClass) -> KeyboardSurfaceSubject {
    KeyboardSurfaceSubject {
        surface_id: format!("surface:{strip_id}"),
        origin_class: origin,
        surface_fingerprint_token: format!("fp:surface:{strip_id}"),
    }
}

fn editor_core_strip() -> SurfaceModeStrip {
    let strip_id = "mode-strip:editor-core:0001";
    SurfaceModeStrip {
        strip_id: strip_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::EditorCore,
        subject: subject_for(strip_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Editor / diff core under a Vim preset: Normal mode visible, full leader-sequence guide, plain-text clipboard"
            .to_owned(),
        source_preset: KeymapSourcePreset::VimPreset,
        current_mode: ModeIndicator::NormalMode,
        mode_strip_class: ModeStripClass::ModalParityComplete,
        keyboard_complete: true,
        macro_replay_explicit: true,
        pending_sequence: idle_sequence(SequenceGuideClass::LeaderSequenceComplete),
        register_picker: RegisterClipboardPicker {
            clipboard_route: ClipboardRouteClass::PlainTextPreserved,
            active_register_token: None,
            available_register_tokens: Vec::new(),
            plain_text_representation_available: true,
            sensitive_copy_warning_active: false,
            picker_keyboard_reachable: true,
        },
        accessibility: full_accessibility(),
        capability_gaps: Vec::new(),
        verification: proof_for(strip_id, AxisProofCurrency::VerifiedCurrent, "editor core"),
        claimed_grade: ContinuityParityGrade::SwitchingCertified,
        effective_grade: ContinuityParityGrade::SwitchingCertified,
        downgrade_trigger: None,
        downgraded_label: None,
        evidence_refs: vec![format!("evidence:strip:{strip_id}")],
        source_contract_refs: vec![MODE_STRIP_SURFACE_DOC_REF.to_owned()],
    }
}

fn notebook_strip() -> SurfaceModeStrip {
    let strip_id = "mode-strip:notebook:0001";
    SurfaceModeStrip {
        strip_id: strip_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::NotebookSurface,
        subject: subject_for(strip_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Notebook cell under a Neovim preset: Insert mode visible, named-register picker, prefix-discoverable sequences"
            .to_owned(),
        source_preset: KeymapSourcePreset::NeovimPreset,
        current_mode: ModeIndicator::InsertMode,
        mode_strip_class: ModeStripClass::ModalParityComplete,
        keyboard_complete: true,
        macro_replay_explicit: true,
        pending_sequence: idle_sequence(SequenceGuideClass::PrefixDiscoverable),
        register_picker: RegisterClipboardPicker {
            clipboard_route: ClipboardRouteClass::NamedRegisterRouted,
            active_register_token: Some("register:unnamed".to_owned()),
            available_register_tokens: vec![
                "register:unnamed".to_owned(),
                "register:a".to_owned(),
                "register:plus".to_owned(),
            ],
            plain_text_representation_available: true,
            sensitive_copy_warning_active: false,
            picker_keyboard_reachable: true,
        },
        accessibility: full_accessibility(),
        capability_gaps: Vec::new(),
        verification: proof_for(strip_id, AxisProofCurrency::VerifiedCurrent, "notebook"),
        claimed_grade: ContinuityParityGrade::ParityComplete,
        effective_grade: ContinuityParityGrade::ParityComplete,
        downgrade_trigger: None,
        downgraded_label: None,
        evidence_refs: vec![format!("evidence:strip:{strip_id}")],
        source_contract_refs: vec![MODE_STRIP_SURFACE_DOC_REF.to_owned()],
    }
}

fn data_api_strip() -> SurfaceModeStrip {
    let strip_id = "mode-strip:data-api:0001";
    SurfaceModeStrip {
        strip_id: strip_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DataApiSurface,
        subject: subject_for(strip_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Data-grid / API result under a non-modal default: keyboard-complete navigation with sensitive-copy warning"
            .to_owned(),
        source_preset: KeymapSourcePreset::NonModalDefault,
        current_mode: ModeIndicator::NonModalEditingMode,
        mode_strip_class: ModeStripClass::NonModalKeyboardComplete,
        keyboard_complete: true,
        macro_replay_explicit: true,
        pending_sequence: idle_sequence(SequenceGuideClass::SingleStrokeOnly),
        register_picker: RegisterClipboardPicker {
            clipboard_route: ClipboardRouteClass::SensitiveCopyWarned,
            active_register_token: None,
            available_register_tokens: Vec::new(),
            plain_text_representation_available: true,
            sensitive_copy_warning_active: true,
            picker_keyboard_reachable: true,
        },
        accessibility: full_accessibility(),
        capability_gaps: Vec::new(),
        verification: proof_for(strip_id, AxisProofCurrency::CachedWithinWindow, "data/API"),
        claimed_grade: ContinuityParityGrade::ParityComplete,
        effective_grade: ContinuityParityGrade::ParityComplete,
        downgrade_trigger: None,
        downgraded_label: None,
        evidence_refs: vec![format!("evidence:strip:{strip_id}")],
        source_contract_refs: vec![MODE_STRIP_SURFACE_DOC_REF.to_owned()],
    }
}

fn preview_strip() -> SurfaceModeStrip {
    let strip_id = "mode-strip:preview:0001";
    SurfaceModeStrip {
        strip_id: strip_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::PreviewSurface,
        subject: subject_for(strip_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Source-first preview under an Emacs preset: read-only modal navigation, plain-text copy, prefix discovery"
            .to_owned(),
        source_preset: KeymapSourcePreset::EmacsPreset,
        current_mode: ModeIndicator::ReadOnlyNavigationMode,
        mode_strip_class: ModeStripClass::ModalReadOnlyNavigation,
        keyboard_complete: true,
        macro_replay_explicit: true,
        pending_sequence: idle_sequence(SequenceGuideClass::PrefixDiscoverable),
        register_picker: RegisterClipboardPicker {
            clipboard_route: ClipboardRouteClass::PlainTextPreserved,
            active_register_token: None,
            available_register_tokens: Vec::new(),
            plain_text_representation_available: true,
            sensitive_copy_warning_active: false,
            picker_keyboard_reachable: true,
        },
        accessibility: full_accessibility(),
        capability_gaps: Vec::new(),
        verification: proof_for(strip_id, AxisProofCurrency::VerifiedCurrent, "preview"),
        claimed_grade: ContinuityParityGrade::ParityComplete,
        effective_grade: ContinuityParityGrade::ParityComplete,
        downgrade_trigger: None,
        downgraded_label: None,
        evidence_refs: vec![format!("evidence:strip:{strip_id}")],
        source_contract_refs: vec![MODE_STRIP_SURFACE_DOC_REF.to_owned()],
    }
}

/// A docs surface mid-sequence: an operator and count prefix are in flight and the
/// surface is awaiting the motion / text object that completes the change.
fn docs_pending_sequence_strip() -> SurfaceModeStrip {
    let strip_id = "mode-strip:docs:0001";
    SurfaceModeStrip {
        strip_id: strip_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DocsSurface,
        subject: subject_for(strip_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Docs authoring under a Helix preset: operator-pending mode with a live count prefix and leader-sequence guide"
            .to_owned(),
        source_preset: KeymapSourcePreset::HelixPreset,
        current_mode: ModeIndicator::OperatorPendingMode,
        mode_strip_class: ModeStripClass::ModalParityComplete,
        keyboard_complete: true,
        macro_replay_explicit: true,
        pending_sequence: PendingSequenceState {
            sequence_guide: SequenceGuideClass::LeaderSequenceComplete,
            pending_operator_token: Some("operator:change".to_owned()),
            pending_sequence_token: Some("guide:change>text-object".to_owned()),
            count_prefix: Some(2),
            awaiting_more_input: true,
            timeout_posture: SequenceTimeoutPosture::WaitsForExplicitCompletion,
            resolution: SequenceResolution::AwaitingDisambiguation,
            sequence_guide_announced: true,
        },
        register_picker: RegisterClipboardPicker {
            clipboard_route: ClipboardRouteClass::RichWithPlainFallback,
            active_register_token: None,
            available_register_tokens: Vec::new(),
            plain_text_representation_available: true,
            sensitive_copy_warning_active: false,
            picker_keyboard_reachable: true,
        },
        accessibility: full_accessibility(),
        capability_gaps: Vec::new(),
        verification: proof_for(strip_id, AxisProofCurrency::VerifiedCurrent, "docs"),
        claimed_grade: ContinuityParityGrade::ParityComplete,
        effective_grade: ContinuityParityGrade::ParityComplete,
        downgrade_trigger: None,
        downgraded_label: None,
        evidence_refs: vec![format!("evidence:strip:{strip_id}")],
        source_contract_refs: vec![MODE_STRIP_SURFACE_DOC_REF.to_owned()],
    }
}

/// A provider-linked companion surface whose posture is provider-backed and never
/// reads as a local rerun. It carries a non-downgrading capability-gap banner that
/// honestly explains that named-register routing narrows to a single clipboard.
fn companion_provider_strip() -> SurfaceModeStrip {
    let strip_id = "mode-strip:companion:0001";
    SurfaceModeStrip {
        strip_id: strip_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::CompanionSurface,
        subject: subject_for(strip_id, SurfaceOriginClass::ProviderLinkedSurface),
        label_summary: "Provider-linked companion under an Emacs preset: read-only modal navigation, provider-backed posture, register narrowed honestly"
            .to_owned(),
        source_preset: KeymapSourcePreset::EmacsPreset,
        current_mode: ModeIndicator::ReadOnlyNavigationMode,
        mode_strip_class: ModeStripClass::ModalReadOnlyNavigation,
        keyboard_complete: true,
        macro_replay_explicit: true,
        pending_sequence: idle_sequence(SequenceGuideClass::PrefixDiscoverable),
        register_picker: RegisterClipboardPicker {
            clipboard_route: ClipboardRouteClass::RichWithPlainFallback,
            active_register_token: None,
            available_register_tokens: Vec::new(),
            plain_text_representation_available: true,
            sensitive_copy_warning_active: false,
            picker_keyboard_reachable: true,
        },
        accessibility: full_accessibility(),
        capability_gaps: vec![CapabilityGapBanner {
            gap_id: format!("gap:{strip_id}:register"),
            gap_kind: CapabilityGapKind::NamedRegisterUnsupported,
            disposition: GapDisposition::NarrowedToSupportedSubset,
            explanation: "Named-register routing is provider-backed and read-only here; copy narrows to the single system clipboard with a plain-text fallback"
                .to_owned(),
            export_safe_reason_token: "named_register_unsupported_on_provider_surface".to_owned(),
            keyboard_reachable: true,
            screen_reader_announced: true,
            hover_only: false,
        }],
        verification: proof_for(strip_id, AxisProofCurrency::ImportedCurrent, "companion"),
        claimed_grade: ContinuityParityGrade::ParityPartial,
        effective_grade: ContinuityParityGrade::ParityPartial,
        downgrade_trigger: None,
        downgraded_label: None,
        evidence_refs: vec![format!("evidence:strip:{strip_id}")],
        source_contract_refs: vec![MODE_STRIP_SURFACE_DOC_REF.to_owned()],
    }
}

/// A data/API surface that imported a leader-key workflow it cannot host. Rather than
/// silently approximating the multi-key sequence, the surface downgrades the sequence
/// axis, drops the effective grade below its claim, and raises a capability-gap banner
/// that names the rejection.
fn data_api_unsupported_sequence_strip() -> SurfaceModeStrip {
    let strip_id = "mode-strip:data-api:unsupported-sequence:0001";
    SurfaceModeStrip {
        strip_id: strip_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DataApiSurface,
        subject: subject_for(strip_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Data-grid that imported a leader-key workflow it cannot host: sequence downgraded honestly with a capability-gap banner"
            .to_owned(),
        source_preset: KeymapSourcePreset::NonModalDefault,
        current_mode: ModeIndicator::NonModalEditingMode,
        mode_strip_class: ModeStripClass::NonModalKeyboardComplete,
        keyboard_complete: true,
        macro_replay_explicit: true,
        pending_sequence: PendingSequenceState {
            sequence_guide: SequenceGuideClass::SequenceUnsupportedDowngraded,
            pending_operator_token: None,
            pending_sequence_token: Some("guide:leader>g>d (unsupported here)".to_owned()),
            count_prefix: None,
            awaiting_more_input: false,
            timeout_posture: SequenceTimeoutPosture::NoPendingSequence,
            resolution: SequenceResolution::UnsupportedDowngraded,
            sequence_guide_announced: true,
        },
        register_picker: RegisterClipboardPicker {
            clipboard_route: ClipboardRouteClass::PlainTextPreserved,
            active_register_token: None,
            available_register_tokens: Vec::new(),
            plain_text_representation_available: true,
            sensitive_copy_warning_active: false,
            picker_keyboard_reachable: true,
        },
        accessibility: full_accessibility(),
        capability_gaps: vec![CapabilityGapBanner {
            gap_id: format!("gap:{strip_id}:sequence"),
            gap_kind: CapabilityGapKind::ModalSequenceUnsupported,
            disposition: GapDisposition::RejectedOutright,
            explanation: "The imported leader-key sequence has no binding on this data-grid surface; it is rejected rather than approximated, and single-stroke navigation stays available"
                .to_owned(),
            export_safe_reason_token: "leader_sequence_unsupported_on_data_grid".to_owned(),
            keyboard_reachable: true,
            screen_reader_announced: true,
            hover_only: false,
        }],
        verification: proof_for(strip_id, AxisProofCurrency::VerifiedCurrent, "data/API"),
        claimed_grade: ContinuityParityGrade::ParityComplete,
        effective_grade: ContinuityParityGrade::ParityUnverified,
        downgrade_trigger: Some(ParityDowngradeTrigger::UnsupportedSequenceDowngraded),
        downgraded_label: Some(
            "Imported leader-key sequence is unsupported on this data-grid surface; held parity-unverified rather than silently approximating the sequence"
                .to_owned(),
        ),
        evidence_refs: vec![format!("evidence:strip:{strip_id}")],
        source_contract_refs: vec![MODE_STRIP_SURFACE_DOC_REF.to_owned()],
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
