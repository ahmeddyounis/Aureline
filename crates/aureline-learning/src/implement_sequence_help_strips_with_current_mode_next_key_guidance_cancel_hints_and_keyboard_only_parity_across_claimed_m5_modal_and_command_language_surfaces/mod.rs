//! One reusable M5 command-language primitive — the sequence-help strip — so a keyboard-first
//! user can see, from the strip alone, exactly where a partial or ambiguous key sequence stands
//! before it fails or surprises them: the current mode or leader key in effect, the valid next
//! keys they can press, the cancel key that always backs them out, an example command the
//! sequence resolves to, and a way to open the full cheat sheet — all reachable by keyboard,
//! announced to a screen reader, and never requiring pointer hover.
//!
//! Aureline's frozen contextual-teaching / migration-bridge component matrix
//! ([`crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix`])
//! names the sequence-help strip as one governed component family and freezes its controlled
//! vocabulary — the sequence-help states (`ready`, `awaiting_next_key`, `partial_match`,
//! `no_binding`, `conflicting_binding`, `disabled_in_context`), the sequence step kinds
//! (`leader_key`, `chord`, `prefix_argument`, `motion`, `operator`, `terminal_action`), and the
//! command-backing states (`bound_command`, `unbound_hint`, `deep_link_command`,
//! `palette_entry`, `keybinding_route`, `no_command_backing`) — plus the surface families, the
//! deployment lines, the consumer surfaces, the accessibility routes, the qualification classes,
//! and the downgrade triggers. This module *implements* that contract as one reusable resolver
//! so a user can tell — from the strip alone — the current mode or leader key, the valid next
//! keys, the cancel key, whether the sequence is still open, a dead end, ambiguous, or disabled,
//! and how to reach the full cheat sheet, without ever masking the current mode or next keys,
//! failing silently on a partial or ambiguous sequence, requiring pointer hover, or severing the
//! command backing or cheat-sheet route.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_sequence_help_strip`] — takes one sequence's help state, current step kind,
//!    command-backing state, opaque current-mode-or-leader reference, valid next keys, cancel
//!    key, optional opaque example-command reference, screen-reader announcement text, opaque
//!    full-cheat-sheet reference, and opaque stable strip identity, and produces one
//!    [`M5ResolvedSequenceHelpStrip`] carrying the derived help posture (ready-for-input,
//!    awaiting-next-key, partial-sequence, unbound-dead-end, conflicting-binding, or
//!    disabled-in-context), the bounded show-valid-next-keys / run-example-command /
//!    resolve-conflicting-binding / cancel-sequence / open-full-cheat-sheet actions, and whether
//!    the sequence is still awaiting more keys, a dead end, ambiguous, or disabled. It never
//!    masks the current mode or the next keys, never lets a partial or ambiguous sequence fail
//!    silently (cancel and the full cheat sheet always stay reachable), never requires pointer
//!    hover, always carries a screen-reader announcement, and always preserves the command
//!    backing honestly.
//!
//! A single parity matrix — [`M5SequenceHelpStripPacket`] — binds one row per claimed M5 modal /
//! command-language consumer (the leader-sequence overlay, the modal-operator strip, the
//! partial-command hint, the command-palette sequence hint, and the support sequence export) to
//! the shared strip anatomy, the same sequence-help states, step kinds, command-backing states,
//! help postures, bounded actions, export fields, and non-visual accessibility routes, so the
//! current-mode / next-keys / cancel-key / example-command / cheat-sheet vocabulary stays
//! identical across desktop, headless/export, and support consumers.
//!
//! The sequence-help state ([`M5SequenceHelpState`]), sequence step kind
//! ([`M5SequenceStepKind`]), command-backing state ([`M5CommandBackingState`]), teaching surface
//! family ([`M5TeachingSurfaceFamily`]), deployment line ([`M5TeachingDeploymentLine`]),
//! teaching consumer surface ([`M5TeachingConsumerSurface`]), accessibility route
//! ([`M5TeachingAccessibilityRoute`]), qualification class ([`M5TeachingQualificationClass`]),
//! and downgrade trigger ([`M5TeachingDowngradeTrigger`]) are reused verbatim from the frozen
//! matrix. This module mints new vocabulary only for what that matrix left implicit about the
//! strip itself: its modal / command-language consumers, its anatomy parts, its derived help
//! posture, its bounded actions, and its export fields. No M5 command-language surface invents a
//! second sequence-help grammar.
//!
//! Raw keystroke logs, pasted buffers, credentials, and private endpoints stay outside the
//! export boundary; every current-mode reference, command reference, cheat-sheet reference, and
//! strip identity is carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_sequence_help_strip_command_palette_sequence_hint_beta_narrowed,
    seeded_m5_sequence_help_strip_packet,
    seeded_m5_sequence_help_strip_support_sequence_export_preview_narrowed,
    M5_SEQUENCE_HELP_STRIP_PACKET_ID,
};

// The sequence-help state, sequence step kind, command-backing state, surface family, deployment
// line, consumer surface, accessibility route, qualification class, and downgrade triggers are
// frozen once, in the contextual-teaching / migration-bridge component matrix. This primitive
// reuses them verbatim so it never invents a parallel command-language vocabulary.
pub use crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix::{
    M5CommandBackingState, M5SequenceHelpState, M5SequenceStepKind, M5TeachingAccessibilityRoute,
    M5TeachingConsumerSurface, M5TeachingDeploymentLine, M5TeachingDowngradeTrigger,
    M5TeachingQualificationClass, M5TeachingSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5SequenceHelpStripPacket`].
pub const M5_SEQUENCE_HELP_STRIP_RECORD_KIND: &str =
    "implement_sequence_help_strips_with_current_mode_next_key_guidance_cancel_hints_and_keyboard_only_parity_across_claimed_m5_modal_and_command_language_surfaces";

/// Schema version for M5 sequence-help-strip records.
pub const M5_SEQUENCE_HELP_STRIP_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the sequence-help-strip boundary schema.
pub const M5_SEQUENCE_HELP_STRIP_SCHEMA_REF: &str = "schemas/ui/m5-sequence-help-strip.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SEQUENCE_HELP_STRIP_DOC_REF: &str = "docs/help/m5_sequence_help_strip_primitive.md";

/// Repo-relative path of the frozen contextual-teaching / migration-bridge component matrix this
/// primitive narrows from.
pub const M5_SEQUENCE_HELP_STRIP_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-contextual-teaching-migration-bridge-component-matrix.schema.json";

/// Repo-relative path of the keybinding-resolver contract the strip's next-key guidance binds
/// against.
pub const M5_SEQUENCE_HELP_STRIP_KEYBINDING_RESOLVER_REF: &str =
    "schemas/commands/keybinding_resolver.schema.json";

/// Repo-relative path of the command-descriptor contract the strip's example-command backing
/// binds against.
pub const M5_SEQUENCE_HELP_STRIP_COMMAND_DESCRIPTOR_REF: &str =
    "schemas/commands/command_descriptor.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SEQUENCE_HELP_STRIP_FIXTURE_DIR: &str = "fixtures/ui/m5-sequence-help-strip-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SEQUENCE_HELP_STRIP_ARTIFACT_REF: &str =
    "artifacts/release/m5-sequence-help-strip-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SEQUENCE_HELP_STRIP_CSV_REF: &str =
    "artifacts/release/m5-sequence-help-strip-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SEQUENCE_HELP_STRIP_REPORT_REF: &str =
    "artifacts/design/m5-sequence-help-strip-primitive.md";

/// One claimed M5 modal / command-language consumer that renders the shared sequence-help strip.
/// These are the consumers the acceptance criteria name — the leader-sequence overlay, the
/// modal-operator strip, the partial-command hint, the command-palette sequence hint, and the
/// support sequence export — so the same strip grammar works across leader sequences, modal
/// operators, partial keyboard commands, and every related command-language teaching moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SequenceHelpConsumerSurface {
    /// The leader-sequence overlay surface.
    LeaderSequenceOverlay,
    /// The modal-operator strip surface.
    ModalOperatorStrip,
    /// The partial-command hint surface.
    PartialCommandHint,
    /// The command-palette sequence hint surface.
    CommandPaletteSequenceHint,
    /// The support sequence-export surface.
    SupportSequenceExport,
}

impl M5SequenceHelpConsumerSurface {
    /// Every claimed modal / command-language consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LeaderSequenceOverlay,
        Self::ModalOperatorStrip,
        Self::PartialCommandHint,
        Self::CommandPaletteSequenceHint,
        Self::SupportSequenceExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LeaderSequenceOverlay => "leader_sequence_overlay",
            Self::ModalOperatorStrip => "modal_operator_strip",
            Self::PartialCommandHint => "partial_command_hint",
            Self::CommandPaletteSequenceHint => "command_palette_sequence_hint",
            Self::SupportSequenceExport => "support_sequence_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LeaderSequenceOverlay => "Leader-Sequence Overlay",
            Self::ModalOperatorStrip => "Modal-Operator Strip",
            Self::PartialCommandHint => "Partial-Command Hint",
            Self::CommandPaletteSequenceHint => "Command-Palette Sequence Hint",
            Self::SupportSequenceExport => "Support Sequence Export",
        }
    }
}

/// The derived help posture of a sequence-help strip — the resolver's honest verdict about where
/// a key sequence stands. Derived one-to-one from the frozen sequence-help state so a partial,
/// dead-end, ambiguous, or disabled sequence is always named for exactly what it is and never
/// left to fail silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SequenceHelpPosture {
    /// Ready to accept the first key of a sequence.
    ReadyForInput,
    /// Awaiting the next key of a multi-key sequence.
    AwaitingNextKey,
    /// A partial match so far — the sequence can still continue.
    PartialSequence,
    /// No binding for the entered keys — a dead end, but never a silent one.
    UnboundDeadEnd,
    /// A conflicting binding needs resolution before the sequence can complete.
    ConflictingBinding,
    /// The sequence is disabled in the current context.
    DisabledInContext,
}

impl M5SequenceHelpPosture {
    /// Every help posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadyForInput,
        Self::AwaitingNextKey,
        Self::PartialSequence,
        Self::UnboundDeadEnd,
        Self::ConflictingBinding,
        Self::DisabledInContext,
    ];

    /// The help posture that honestly reflects a sequence-help state — one-to-one, never
    /// upgrading a dead end or an ambiguous match into a resolvable sequence.
    pub const fn from_state(state: M5SequenceHelpState) -> Self {
        match state {
            M5SequenceHelpState::Ready => Self::ReadyForInput,
            M5SequenceHelpState::AwaitingNextKey => Self::AwaitingNextKey,
            M5SequenceHelpState::PartialMatch => Self::PartialSequence,
            M5SequenceHelpState::NoBinding => Self::UnboundDeadEnd,
            M5SequenceHelpState::ConflictingBinding => Self::ConflictingBinding,
            M5SequenceHelpState::DisabledInContext => Self::DisabledInContext,
        }
    }

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyForInput => "ready_for_input",
            Self::AwaitingNextKey => "awaiting_next_key",
            Self::PartialSequence => "partial_sequence",
            Self::UnboundDeadEnd => "unbound_dead_end",
            Self::ConflictingBinding => "conflicting_binding",
            Self::DisabledInContext => "disabled_in_context",
        }
    }

    /// True when the sequence is still open and expects further keys (ready, awaiting a next
    /// key, or a partial match). These are the postures that must always show valid next keys.
    pub const fn is_awaiting_more(self) -> bool {
        matches!(
            self,
            Self::ReadyForInput | Self::AwaitingNextKey | Self::PartialSequence
        )
    }

    /// True when the entered keys resolve to no binding — a dead end. The strip still explains
    /// it; it never fails silently.
    pub const fn is_dead_end(self) -> bool {
        matches!(self, Self::UnboundDeadEnd)
    }

    /// True when the entered keys are ambiguous and need a conflict resolved.
    pub const fn is_ambiguous(self) -> bool {
        matches!(self, Self::ConflictingBinding)
    }

    /// True when the sequence is disabled in the current context.
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::DisabledInContext)
    }
}

/// One bounded action a sequence-help strip offers, so a keyboard-first user can always see the
/// valid next keys, run the example command, resolve an ambiguous binding, cancel out of the
/// sequence, or open the full cheat sheet — never trapped in a partial or ambiguous sequence
/// they cannot interpret or escape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SequenceHelpAction {
    /// Show the valid next keys for the current sequence.
    ShowValidNextKeys,
    /// Run the example command this sequence resolves to.
    RunExampleCommand,
    /// Resolve the conflicting binding blocking this sequence.
    ResolveConflictingBinding,
    /// Cancel out of the current sequence.
    CancelSequence,
    /// Open the full cheat sheet.
    OpenFullCheatSheet,
}

impl M5SequenceHelpAction {
    /// Every sequence-help action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ShowValidNextKeys,
        Self::RunExampleCommand,
        Self::ResolveConflictingBinding,
        Self::CancelSequence,
        Self::OpenFullCheatSheet,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShowValidNextKeys => "show_valid_next_keys",
            Self::RunExampleCommand => "run_example_command",
            Self::ResolveConflictingBinding => "resolve_conflicting_binding",
            Self::CancelSequence => "cancel_sequence",
            Self::OpenFullCheatSheet => "open_full_cheat_sheet",
        }
    }
}

/// Controlled sequence-help-strip anatomy part the shared strip surfaces. The parts in
/// [`M5SequenceHelpAnatomyPart::MANDATORY`] are required on every strip so the current mode or
/// leader key, the valid next keys, the cancel key, the example command, and the
/// open-full-cheat-sheet action are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SequenceHelpAnatomyPart {
    /// The current-mode-or-leader cue.
    CurrentModeOrLeaderCue,
    /// The valid-next-keys cue.
    ValidNextKeysCue,
    /// The cancel-key cue.
    CancelKeyCue,
    /// The example-command cue.
    ExampleCommandCue,
    /// The open-full-cheat-sheet action cue.
    OpenCheatSheetActionCue,
    /// The screen-reader-announcement cue.
    ScreenReaderAnnouncementCue,
    /// The step-kind cue.
    StepKindCue,
    /// The command-backing cue.
    CommandBackingCue,
}

impl M5SequenceHelpAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CurrentModeOrLeaderCue,
        Self::ValidNextKeysCue,
        Self::CancelKeyCue,
        Self::ExampleCommandCue,
        Self::OpenCheatSheetActionCue,
        Self::ScreenReaderAnnouncementCue,
        Self::StepKindCue,
        Self::CommandBackingCue,
    ];

    /// The anatomy parts every strip must render.
    pub const MANDATORY: [Self; 5] = [
        Self::CurrentModeOrLeaderCue,
        Self::ValidNextKeysCue,
        Self::CancelKeyCue,
        Self::ExampleCommandCue,
        Self::OpenCheatSheetActionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentModeOrLeaderCue => "current_mode_or_leader_cue",
            Self::ValidNextKeysCue => "valid_next_keys_cue",
            Self::CancelKeyCue => "cancel_key_cue",
            Self::ExampleCommandCue => "example_command_cue",
            Self::OpenCheatSheetActionCue => "open_cheat_sheet_action_cue",
            Self::ScreenReaderAnnouncementCue => "screen_reader_announcement_cue",
            Self::StepKindCue => "step_kind_cue",
            Self::CommandBackingCue => "command_backing_cue",
        }
    }
}

/// A field the strip export carries so sequence-help-strip truth is reconstructable. The fields
/// in [`M5SequenceHelpExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SequenceHelpExportField {
    /// The sequence-help state.
    HelpState,
    /// The sequence step kind.
    StepKind,
    /// The current-mode-or-leader reference.
    CurrentModeOrLeaderRef,
    /// The valid next keys.
    ValidNextKeys,
    /// The cancel key.
    CancelKey,
    /// The example-command reference.
    ExampleCommandRef,
    /// The screen-reader announcement.
    ScreenReaderAnnouncement,
    /// The full-cheat-sheet reference.
    CheatSheetRef,
}

impl M5SequenceHelpExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::HelpState,
        Self::StepKind,
        Self::CurrentModeOrLeaderRef,
        Self::ValidNextKeys,
        Self::CancelKey,
        Self::ExampleCommandRef,
        Self::ScreenReaderAnnouncement,
        Self::CheatSheetRef,
    ];

    /// The export fields every strip must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::HelpState,
        Self::StepKind,
        Self::CurrentModeOrLeaderRef,
        Self::ValidNextKeys,
        Self::CancelKey,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpState => "help_state",
            Self::StepKind => "step_kind",
            Self::CurrentModeOrLeaderRef => "current_mode_or_leader_ref",
            Self::ValidNextKeys => "valid_next_keys",
            Self::CancelKey => "cancel_key",
            Self::ExampleCommandRef => "example_command_ref",
            Self::ScreenReaderAnnouncement => "screen_reader_announcement",
            Self::CheatSheetRef => "cheat_sheet_ref",
        }
    }
}

// ---- sequence-help-strip resolver ---------------------------------------

/// The full input to the sequence-help-strip resolver for one key sequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SequenceHelpStripResolutionInput {
    /// Where the sequence currently stands.
    pub help_state: M5SequenceHelpState,
    /// The kind of step the current sequence position expects.
    pub step_kind: M5SequenceStepKind,
    /// How the sequence's terminal action ties to a stable command.
    pub command_backing: M5CommandBackingState,
    /// The opaque current mode or leader key in effect (must be non-empty).
    pub current_mode_or_leader_ref: String,
    /// The valid next keys the user can press. Required to be non-empty for an open sequence
    /// (ready, awaiting a next key, or a partial match).
    pub valid_next_keys: Vec<String>,
    /// The opaque cancel key that always backs out of the sequence (must be non-empty).
    pub cancel_key: String,
    /// The opaque example command this sequence resolves to. `None` only for a sequence with no
    /// command backing; `Some(non-empty)` for every command-backed sequence.
    pub example_command_ref: Option<String>,
    /// The screen-reader announcement text (must be non-empty) so the strip is never hover- or
    /// sight-only.
    pub screen_reader_announcement: String,
    /// The opaque full-cheat-sheet reference the open-full-cheat-sheet action targets (must be
    /// non-empty).
    pub cheat_sheet_ref: String,
    /// The opaque stable strip identity (must be non-empty).
    pub strip_identity_ref: String,
}

/// The resolved sequence-help-strip truth for one key sequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSequenceHelpStrip {
    /// The sequence-help state.
    pub help_state: M5SequenceHelpState,
    /// The sequence step kind.
    pub step_kind: M5SequenceStepKind,
    /// The command-backing state.
    pub command_backing: M5CommandBackingState,
    /// The opaque current-mode-or-leader reference, preserved exactly from the input.
    pub current_mode_or_leader_ref: String,
    /// The valid next keys, preserved exactly from the input.
    pub valid_next_keys: Vec<String>,
    /// The opaque cancel key, preserved exactly from the input.
    pub cancel_key: String,
    /// The opaque example-command reference, preserved exactly from the input.
    pub example_command_ref: Option<String>,
    /// The screen-reader announcement, preserved exactly from the input.
    pub screen_reader_announcement: String,
    /// The opaque full-cheat-sheet reference, preserved exactly from the input.
    pub cheat_sheet_ref: String,
    /// The opaque stable strip identity, preserved exactly from the input.
    pub strip_identity_ref: String,
    /// The derived help posture.
    pub help_posture: M5SequenceHelpPosture,
    /// The bounded actions this strip offers.
    pub available_actions: Vec<M5SequenceHelpAction>,
    /// True when the sequence is still open and awaiting more keys.
    pub is_awaiting_more: bool,
    /// True when the entered keys resolve to no binding.
    pub is_dead_end: bool,
    /// True when the entered keys are ambiguous.
    pub is_ambiguous: bool,
    /// True when the sequence is disabled in the current context.
    pub is_disabled: bool,
    /// True when the sequence's terminal action is command-backed.
    pub is_command_backed: bool,
    /// True when the strip shows one or more valid next keys.
    pub shows_next_keys: bool,
    /// True when the strip offers a run-example-command action.
    pub example_command_available: bool,
    /// True when the strip offers a cancel-sequence action. ALWAYS `true`.
    pub cancel_available: bool,
    /// True when the strip offers an open-full-cheat-sheet action. ALWAYS `true`.
    pub cheat_sheet_available: bool,
    /// The strip always shows the current mode or leader key. ALWAYS `true`.
    pub shows_current_mode_or_leader: bool,
    /// The strip always explains the next keys or, on a dead end, the reason there are none —
    /// never failing silently. ALWAYS `true`.
    pub explains_next_keys_or_dead_end: bool,
    /// The strip always shows the cancel key. ALWAYS `true`.
    pub shows_cancel_key: bool,
    /// The strip never requires pointer hover. ALWAYS `true`.
    pub never_requires_pointer_hover: bool,
    /// The strip always carries a screen-reader announcement. ALWAYS `true`.
    pub provides_screen_reader_announcement: bool,
    /// The strip always keeps the full cheat sheet reachable. ALWAYS `true`.
    pub keeps_full_cheat_sheet_reachable: bool,
    /// The strip always preserves the command backing honestly. ALWAYS `true`.
    pub preserves_command_backing_honestly: bool,
}

/// Errors returned by [`resolve_sequence_help_strip`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SequenceHelpStripResolutionError {
    /// The current-mode-or-leader reference was empty.
    EmptyCurrentModeOrLeader,
    /// The cancel key was empty.
    EmptyCancelKey,
    /// The screen-reader announcement was empty.
    EmptyScreenReaderAnnouncement,
    /// The cheat-sheet reference was empty.
    EmptyCheatSheetRef,
    /// The strip identity ref was empty.
    EmptyStripIdentity,
    /// An open sequence (ready, awaiting a next key, or a partial match) named no valid next
    /// keys — it would fail silently.
    MissingNextKeysForOpenSequence,
    /// A sequence with no command backing wrongly declared an example command.
    ExampleCommandOnUnbackedState,
    /// A command-backed sequence declared no example command.
    MissingExampleForBackedState,
    /// A strip descriptor carried forbidden material.
    ForbiddenSequenceMaterial,
}

impl M5SequenceHelpStripResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyCurrentModeOrLeader => "empty_current_mode_or_leader",
            Self::EmptyCancelKey => "empty_cancel_key",
            Self::EmptyScreenReaderAnnouncement => "empty_screen_reader_announcement",
            Self::EmptyCheatSheetRef => "empty_cheat_sheet_ref",
            Self::EmptyStripIdentity => "empty_strip_identity",
            Self::MissingNextKeysForOpenSequence => "missing_next_keys_for_open_sequence",
            Self::ExampleCommandOnUnbackedState => "example_command_on_unbacked_state",
            Self::MissingExampleForBackedState => "missing_example_for_backed_state",
            Self::ForbiddenSequenceMaterial => "forbidden_sequence_material",
        }
    }
}

impl fmt::Display for M5SequenceHelpStripResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "sequence help strip resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SequenceHelpStripResolutionError {}

/// Resolves one sequence-help strip from its declared help state, step kind, command backing,
/// current-mode reference, valid next keys, cancel key, example command, and cheat-sheet route.
///
/// The help posture is derived one-to-one from the frozen sequence-help state so a partial,
/// dead-end, ambiguous, or disabled sequence is always named for exactly what it is. The action
/// set offers show-valid-next-keys whenever there are next keys to show, run-example-command
/// whenever the sequence is command-backed with an example, resolve-conflicting-binding whenever
/// the sequence is ambiguous, and always offers cancel-sequence and open-full-cheat-sheet so a
/// keyboard-first user can never be trapped in a sequence that fails silently. An open sequence
/// with no valid next keys is rejected outright, so an ambiguous or partial sequence never
/// leaves the user with nothing to press and no way to interpret it.
pub fn resolve_sequence_help_strip(
    input: &M5SequenceHelpStripResolutionInput,
) -> Result<M5ResolvedSequenceHelpStrip, M5SequenceHelpStripResolutionError> {
    if input.current_mode_or_leader_ref.trim().is_empty() {
        return Err(M5SequenceHelpStripResolutionError::EmptyCurrentModeOrLeader);
    }
    if input.cancel_key.trim().is_empty() {
        return Err(M5SequenceHelpStripResolutionError::EmptyCancelKey);
    }
    if input.screen_reader_announcement.trim().is_empty() {
        return Err(M5SequenceHelpStripResolutionError::EmptyScreenReaderAnnouncement);
    }
    if input.cheat_sheet_ref.trim().is_empty() {
        return Err(M5SequenceHelpStripResolutionError::EmptyCheatSheetRef);
    }
    if input.strip_identity_ref.trim().is_empty() {
        return Err(M5SequenceHelpStripResolutionError::EmptyStripIdentity);
    }
    if sequence_input_has_forbidden_material(input) {
        return Err(M5SequenceHelpStripResolutionError::ForbiddenSequenceMaterial);
    }

    let help_posture = M5SequenceHelpPosture::from_state(input.help_state);
    let shows_next_keys = input
        .valid_next_keys
        .iter()
        .any(|key| !key.trim().is_empty());
    if help_posture.is_awaiting_more() && !shows_next_keys {
        return Err(M5SequenceHelpStripResolutionError::MissingNextKeysForOpenSequence);
    }

    let is_command_backed = !matches!(
        input.command_backing,
        M5CommandBackingState::NoCommandBacking
    );
    let has_example = input
        .example_command_ref
        .as_ref()
        .is_some_and(|value| !value.trim().is_empty());
    if is_command_backed {
        if !has_example {
            return Err(M5SequenceHelpStripResolutionError::MissingExampleForBackedState);
        }
    } else if has_example {
        return Err(M5SequenceHelpStripResolutionError::ExampleCommandOnUnbackedState);
    }

    let available_actions = derive_sequence_help_actions(
        shows_next_keys,
        is_command_backed && has_example,
        help_posture.is_ambiguous(),
    );

    Ok(M5ResolvedSequenceHelpStrip {
        help_state: input.help_state,
        step_kind: input.step_kind,
        command_backing: input.command_backing,
        current_mode_or_leader_ref: input.current_mode_or_leader_ref.clone(),
        valid_next_keys: input.valid_next_keys.clone(),
        cancel_key: input.cancel_key.clone(),
        example_command_ref: input.example_command_ref.clone(),
        screen_reader_announcement: input.screen_reader_announcement.clone(),
        cheat_sheet_ref: input.cheat_sheet_ref.clone(),
        strip_identity_ref: input.strip_identity_ref.clone(),
        help_posture,
        available_actions,
        is_awaiting_more: help_posture.is_awaiting_more(),
        is_dead_end: help_posture.is_dead_end(),
        is_ambiguous: help_posture.is_ambiguous(),
        is_disabled: help_posture.is_disabled(),
        is_command_backed,
        shows_next_keys,
        example_command_available: is_command_backed && has_example,
        cancel_available: true,
        cheat_sheet_available: true,
        // The acceptance criteria: sequence-help strips always show the current mode or leader,
        // always explain the next keys or the reason a dead end has none (never failing
        // silently), always show the cancel key, never require pointer hover, always carry a
        // screen-reader announcement, always keep the full cheat sheet reachable, and always
        // preserve the command backing honestly.
        shows_current_mode_or_leader: true,
        explains_next_keys_or_dead_end: true,
        shows_cancel_key: true,
        never_requires_pointer_hover: true,
        provides_screen_reader_announcement: true,
        keeps_full_cheat_sheet_reachable: true,
        preserves_command_backing_honestly: true,
    })
}

/// Derives the bounded action set from whether valid next keys exist, whether the sequence is
/// command-backed with an example, and whether the sequence is ambiguous.
///
/// Every strip offers cancel-sequence and open-full-cheat-sheet so a keyboard-first user can
/// always back out or reach the full cheat sheet. A strip with valid next keys additionally
/// offers show-valid-next-keys; a command-backed sequence with an example offers
/// run-example-command; and an ambiguous sequence offers resolve-conflicting-binding.
fn derive_sequence_help_actions(
    shows_next_keys: bool,
    example_command_available: bool,
    is_ambiguous: bool,
) -> Vec<M5SequenceHelpAction> {
    use M5SequenceHelpAction as Action;

    let mut actions = Vec::new();
    if shows_next_keys {
        actions.push(Action::ShowValidNextKeys);
    }
    if example_command_available {
        actions.push(Action::RunExampleCommand);
    }
    if is_ambiguous {
        actions.push(Action::ResolveConflictingBinding);
    }
    actions.push(Action::CancelSequence);
    actions.push(Action::OpenFullCheatSheet);
    actions
}

/// True when any opaque descriptor on the input carries obviously forbidden material.
fn sequence_input_has_forbidden_material(input: &M5SequenceHelpStripResolutionInput) -> bool {
    if value_repr_is_forbidden(&input.current_mode_or_leader_ref)
        || value_repr_is_forbidden(&input.cancel_key)
        || value_repr_is_forbidden(&input.screen_reader_announcement)
        || value_repr_is_forbidden(&input.cheat_sheet_ref)
        || value_repr_is_forbidden(&input.strip_identity_ref)
    {
        return true;
    }
    if let Some(command) = &input.example_command_ref {
        if value_repr_is_forbidden(command) {
            return true;
        }
    }
    input
        .valid_next_keys
        .iter()
        .any(|key| value_repr_is_forbidden(key))
}

// ---- worked cases -------------------------------------------------------

/// One worked sequence-help-strip resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SequenceHelpStripResolutionCase {
    /// The resolver input.
    pub input: M5SequenceHelpStripResolutionInput,
    /// The resolved truth. Must equal `resolve_sequence_help_strip(&input)`.
    pub resolved: M5ResolvedSequenceHelpStrip,
}

impl M5SequenceHelpStripResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5SequenceHelpStripResolutionInput) -> Self {
        let resolved =
            resolve_sequence_help_strip(&input).expect("seed sequence help strip case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_sequence_help_strip(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved case preserves the input strip identity, current-mode reference,
    /// valid next keys, cancel key, example command, screen-reader announcement, and cheat-sheet
    /// reference exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.strip_identity_ref == self.input.strip_identity_ref
            && self.resolved.current_mode_or_leader_ref == self.input.current_mode_or_leader_ref
            && self.resolved.valid_next_keys == self.input.valid_next_keys
            && self.resolved.cancel_key == self.input.cancel_key
            && self.resolved.example_command_ref == self.input.example_command_ref
            && self.resolved.screen_reader_announcement == self.input.screen_reader_announcement
            && self.resolved.cheat_sheet_ref == self.input.cheat_sheet_ref
    }

    /// True when the resolved case shows the current mode, explains next keys or the dead end,
    /// shows the cancel key, never requires pointer hover, carries a screen-reader announcement,
    /// keeps the cheat sheet reachable, preserves command backing honestly, and — concretely —
    /// shows next keys wherever the sequence is still open and always keeps cancel and the cheat
    /// sheet reachable so it never fails silently.
    pub fn preserves_keyboard_parity(&self) -> bool {
        self.resolved.shows_current_mode_or_leader
            && self.resolved.explains_next_keys_or_dead_end
            && self.resolved.shows_cancel_key
            && self.resolved.never_requires_pointer_hover
            && self.resolved.provides_screen_reader_announcement
            && self.resolved.keeps_full_cheat_sheet_reachable
            && self.resolved.preserves_command_backing_honestly
            // The concrete AC1 guarantee: an open sequence always shows valid next keys.
            && (!self.resolved.is_awaiting_more || self.resolved.shows_next_keys)
            // The concrete AC1 guarantee: cancel and the full cheat sheet are always reachable,
            // so a partial or ambiguous sequence never becomes a silent dead end.
            && self.resolved.cancel_available
            && self.resolved.cheat_sheet_available
            && self
                .resolved
                .available_actions
                .contains(&M5SequenceHelpAction::CancelSequence)
            && self
                .resolved
                .available_actions
                .contains(&M5SequenceHelpAction::OpenFullCheatSheet)
    }
}

/// One row in the primitive matrix: one modal / command-language consumer bound to the shared
/// strip anatomy, sequence-help states, step kinds, command-backing states, help postures,
/// bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SequenceHelpConsumerRow {
    /// Modal / command-language consumer family.
    pub consumer_surface: M5SequenceHelpConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5TeachingQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 modal / command-language surface families that render / consume this strip.
    pub surface_families: Vec<M5TeachingSurfaceFamily>,
    /// Deployment lines this strip keeps the same truth across.
    pub deployment_lines: Vec<M5TeachingDeploymentLine>,
    /// Anatomy parts this strip renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5SequenceHelpAnatomyPart>,
    /// Sequence-help states this consumer distinguishes.
    pub help_states: Vec<M5SequenceHelpState>,
    /// Sequence step kinds this consumer distinguishes.
    pub step_kinds: Vec<M5SequenceStepKind>,
    /// Command-backing states this consumer distinguishes.
    pub command_backing_states: Vec<M5CommandBackingState>,
    /// Help postures this consumer distinguishes.
    pub help_postures: Vec<M5SequenceHelpPosture>,
    /// Bounded sequence-help actions this consumer offers.
    pub help_actions: Vec<M5SequenceHelpAction>,
    /// Export fields this strip carries (must include the mandatory fields).
    pub export_fields: Vec<M5SequenceHelpExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5TeachingAccessibilityRoute>,
    /// Teaching subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5TeachingConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5TeachingDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked strip resolutions proving the resolver on this consumer.
    pub sequence_examples: Vec<M5SequenceHelpStripResolutionCase>,
    /// Hard invariant: this consumer never masks its current mode or valid next keys. MUST be
    /// `false`.
    pub masks_current_mode_or_next_keys: bool,
    /// Hard invariant: this consumer never lets a partial or ambiguous sequence fail silently.
    /// MUST be `false`.
    pub fails_silently_on_partial_or_ambiguous: bool,
    /// Hard invariant: this consumer never requires pointer hover. MUST be `false`.
    pub requires_pointer_hover: bool,
    /// Hard invariant: this consumer never severs the command backing or cheat-sheet route. MUST
    /// be `false`.
    pub severs_command_backing_or_cheat_sheet: bool,
}

impl M5SequenceHelpConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5SequenceHelpAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5SequenceHelpAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5SequenceHelpExportField> =
            self.export_fields.iter().copied().collect();
        M5SequenceHelpExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_current_mode_or_next_keys
            && !self.fails_silently_on_partial_or_ambiguous
            && !self.requires_pointer_hover
            && !self.severs_command_backing_or_cheat_sheet
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SequenceHelpStripVocabularySet {
    /// Modal / command-language consumer tokens.
    pub help_consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Help-posture tokens.
    pub help_postures: Vec<String>,
    /// Help-action tokens.
    pub help_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Sequence-help-state tokens (reused from the frozen matrix).
    pub help_states: Vec<String>,
    /// Sequence-step-kind tokens (reused from the frozen matrix).
    pub step_kinds: Vec<String>,
    /// Command-backing-state tokens (reused from the frozen matrix).
    pub command_backing_states: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Teaching-consumer-surface tokens (reused from the frozen matrix).
    pub teaching_consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5SequenceHelpStripVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            help_consumer_surfaces: tokens(&M5SequenceHelpConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5SequenceHelpAnatomyPart::ALL, |v| v.as_str()),
            help_postures: tokens(&M5SequenceHelpPosture::ALL, |v| v.as_str()),
            help_actions: tokens(&M5SequenceHelpAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5SequenceHelpExportField::ALL, |v| v.as_str()),
            help_states: tokens(&M5SequenceHelpState::ALL, |v| v.as_str()),
            step_kinds: tokens(&M5SequenceStepKind::ALL, |v| v.as_str()),
            command_backing_states: tokens(&M5CommandBackingState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TeachingSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TeachingDeploymentLine::ALL, |v| v.as_str()),
            teaching_consumer_surfaces: tokens(&M5TeachingConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TeachingAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SequenceHelpStripGovernanceReview {
    /// The strip shows the current mode or leader key.
    pub sequence_strip_shows_current_mode_or_leader: bool,
    /// The strip shows the valid next keys.
    pub sequence_strip_shows_valid_next_keys: bool,
    /// The strip shows the cancel key.
    pub sequence_strip_shows_cancel_key: bool,
    /// The strip shows an example command.
    pub sequence_strip_shows_example_command: bool,
    /// The strip can open the full cheat sheet.
    pub sequence_strip_opens_full_cheat_sheet: bool,
    /// Partial or ambiguous sequences never fail silently.
    pub partial_or_ambiguous_sequences_never_fail_silently: bool,
    /// Keyboard-first users learn command-language pathways entirely in-product.
    pub keyboard_first_users_learn_pathways_in_product: bool,
    /// The strip never requires pointer hover.
    pub sequence_strip_never_requires_pointer_hover: bool,
    /// The strip provides a screen-reader announcement.
    pub sequence_strip_provides_screen_reader_announcement: bool,
    /// The strip preserves the command backing behind the sequence.
    pub sequence_strip_preserves_command_backing: bool,
    /// Strips keep the same truth across every deployment line.
    pub sequence_strips_stable_across_deployment_lines: bool,
    /// Strips keep the same truth across desktop, headless/export, and support consumers.
    pub sequence_strips_stable_across_consumer_surfaces: bool,
    /// Every strip declares a non-visual accessibility route.
    pub every_sequence_strip_declares_accessibility_route: bool,
    /// The support / export packet reconstructs sequence-help-strip truth.
    pub support_export_reconstructs_sequence_truth: bool,
    /// Later M5 rows cannot invent parallel sequence-help vocabulary.
    pub later_rows_cannot_invent_parallel_sequence_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SequenceHelpStripConsumerProjection {
    /// Modal / command-language surfaces consume the shared sequence-help vocabulary.
    pub command_language_surfaces_consume_sequence_vocabulary: bool,
    /// The help-posture resolver reads a single canonical source.
    pub help_posture_reads_single_source: bool,
    /// The action-set derivation reads a single canonical source.
    pub action_set_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop strips read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SequenceHelpStripProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the sequence-help strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SequenceHelpStripReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting sequence-help audit.
    pub sequence_help_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SequenceHelpStripPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SequenceHelpStripPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Modal / command-language consumer rows.
    pub rows: Vec<M5SequenceHelpConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SequenceHelpStripVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SequenceHelpStripGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SequenceHelpStripConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SequenceHelpStripProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SequenceHelpStripReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 sequence-help-strip primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SequenceHelpStripPacket {
    /// Record kind; must equal [`M5_SEQUENCE_HELP_STRIP_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SEQUENCE_HELP_STRIP_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Modal / command-language consumer rows.
    pub rows: Vec<M5SequenceHelpConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SequenceHelpStripVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SequenceHelpStripGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SequenceHelpStripConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SequenceHelpStripProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SequenceHelpStripReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SequenceHelpStripPacket {
    /// Builds an M5 sequence-help-strip-primitive packet from stable-lane input.
    pub fn new(input: M5SequenceHelpStripPacketInput) -> Self {
        Self {
            record_kind: M5_SEQUENCE_HELP_STRIP_RECORD_KIND.to_owned(),
            schema_version: M5_SEQUENCE_HELP_STRIP_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 sequence-help-strip-primitive invariants.
    pub fn validate(&self) -> Vec<M5SequenceHelpStripViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SEQUENCE_HELP_STRIP_RECORD_KIND {
            violations.push(M5SequenceHelpStripViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SEQUENCE_HELP_STRIP_SCHEMA_VERSION {
            violations.push(M5SequenceHelpStripViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SequenceHelpStripViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_help_state_coverage(self, &mut violations);
        validate_step_kind_coverage(self, &mut violations);
        validate_posture_coverage(self, &mut violations);
        validate_action_coverage(self, &mut violations);
        validate_non_silent_coverage(self, &mut violations);
        validate_keyboard_parity_coverage(self, &mut violations);
        validate_reversibility(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 sequence help strip primitive packet serializes"),
        ) {
            violations.push(M5SequenceHelpStripViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 sequence help strip primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per modal / command-language consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy,help_states,step_kinds,help_postures,help_actions,sequence_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.help_states, |v| v.as_str()),
                join_tokens(&row.step_kinds, |v| v.as_str()),
                join_tokens(&row.help_postures, |v| v.as_str()),
                join_tokens(&row.help_actions, |v| v.as_str()),
                row.sequence_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Sequence-Help-Strip Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Modal / command-language consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Help postures: {}\n",
            self.vocabulary_set.help_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Help actions: {}\n",
            self.vocabulary_set.help_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Help states: {}\n",
            self.vocabulary_set.help_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Modal / command-language consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked sequences: {}\n",
                row.sequence_examples.len()
            ));
            for case in &row.sequence_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` (awaiting `{}`, dead-end `{}`, ambiguous `{}`, backed `{}`)\n",
                    case.resolved.strip_identity_ref,
                    case.resolved.help_state.as_str(),
                    case.resolved.step_kind.as_str(),
                    case.resolved.help_posture.as_str(),
                    case.resolved.is_awaiting_more,
                    case.resolved.is_dead_end,
                    case.resolved.is_ambiguous,
                    case.resolved.is_command_backed,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 sequence-help-strip-primitive export.
#[derive(Debug)]
pub enum M5SequenceHelpStripArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SequenceHelpStripViolation>),
}

impl fmt::Display for M5SequenceHelpStripArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 sequence help strip primitive export parse failed: {error}"
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
                    "m5 sequence help strip primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SequenceHelpStripArtifactError {}

/// Validation failures emitted by [`M5SequenceHelpStripPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SequenceHelpStripViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required modal / command-language consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A modal / command-language consumer row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked sequence resolutions.
    SequenceExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every sequence-help state.
    HelpStateCoverageUnproven,
    /// The worked resolutions do not exercise every sequence step kind.
    StepKindCoverageUnproven,
    /// The worked resolutions do not prove an awaiting-more, a dead-end, a conflicting, and a
    /// disabled help posture.
    PostureCoverageUnproven,
    /// The worked resolutions do not prove the show-valid-next-keys, run-example-command,
    /// resolve-conflicting-binding, cancel-sequence, and open-full-cheat-sheet actions.
    ActionCoverageUnproven,
    /// No worked resolution proves a dead-end or ambiguous sequence still keeps cancel and the
    /// full cheat sheet reachable — that a partial or ambiguous sequence never fails silently.
    NonSilentParityUnproven,
    /// A worked resolution does not keep keyboard-only parity (screen reader + non-hover) for
    /// every consumer.
    KeyboardParityUnproven,
    /// A worked resolution does not preserve the current mode, next keys, cancel key,
    /// screen-reader announcement, or cheat-sheet route.
    ReversibilityUnproven,
    /// A worked resolution does not preserve its exact strip identity, current mode, next keys,
    /// cancel key, example command, announcement, or cheat-sheet reference.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SequenceHelpStripViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::SequenceExampleMissing => "sequence_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::HelpStateCoverageUnproven => "help_state_coverage_unproven",
            Self::StepKindCoverageUnproven => "step_kind_coverage_unproven",
            Self::PostureCoverageUnproven => "posture_coverage_unproven",
            Self::ActionCoverageUnproven => "action_coverage_unproven",
            Self::NonSilentParityUnproven => "non_silent_parity_unproven",
            Self::KeyboardParityUnproven => "keyboard_parity_unproven",
            Self::ReversibilityUnproven => "reversibility_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 sequence-help-strip-primitive export.
pub fn current_stable_m5_sequence_help_strip_export(
) -> Result<M5SequenceHelpStripPacket, M5SequenceHelpStripArtifactError> {
    let packet: M5SequenceHelpStripPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-sequence-help-strip-primitive-proof/support_export.json"
    )))
    .map_err(M5SequenceHelpStripArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SequenceHelpStripArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SEQUENCE_HELP_STRIP_SCHEMA_REF,
        M5_SEQUENCE_HELP_STRIP_DOC_REF,
        M5_SEQUENCE_HELP_STRIP_COMPONENT_MATRIX_REF,
        M5_SEQUENCE_HELP_STRIP_KEYBINDING_RESOLVER_REF,
        M5_SEQUENCE_HELP_STRIP_COMMAND_DESCRIPTOR_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SequenceHelpStripViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5SequenceHelpStripViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let present: BTreeSet<M5SequenceHelpConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5SequenceHelpConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5SequenceHelpStripViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.help_states.is_empty()
            || row.step_kinds.is_empty()
            || row.command_backing_states.is_empty()
            || row.help_postures.is_empty()
            || row.help_actions.is_empty()
            || row.export_fields.is_empty()
        {
            violations.push(M5SequenceHelpStripViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5SequenceHelpStripViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5SequenceHelpStripViolation::MandatoryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TeachingAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5SequenceHelpStripViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5SequenceHelpStripViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5SequenceHelpStripViolation::DowngradeTriggersMissing);
        }
        if row.sequence_examples.is_empty() {
            violations.push(M5SequenceHelpStripViolation::SequenceExampleMissing);
        }
        if row
            .sequence_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5SequenceHelpStripViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5SequenceHelpStripViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5SequenceHelpStripViolation::RowInvariantViolated);
        }
    }
}

/// Every sequence-help state must be exercised by some worked resolution — the implementation
/// requirement that the strip works across ready, awaiting, partial, no-binding, conflicting, and
/// disabled sequences.
fn validate_help_state_coverage(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let exercised: BTreeSet<M5SequenceHelpState> = packet
        .rows
        .iter()
        .flat_map(|row| row.sequence_examples.iter())
        .map(|case| case.resolved.help_state)
        .collect();
    let covered = M5SequenceHelpState::ALL
        .iter()
        .all(|state| exercised.contains(state));
    if !covered {
        violations.push(M5SequenceHelpStripViolation::HelpStateCoverageUnproven);
    }
}

/// Every sequence step kind must be exercised — the implementation requirement that the same
/// strip works for leader sequences, modal operators, partial keyboard commands, and every
/// related command-language teaching moment.
fn validate_step_kind_coverage(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let exercised: BTreeSet<M5SequenceStepKind> = packet
        .rows
        .iter()
        .flat_map(|row| row.sequence_examples.iter())
        .map(|case| case.resolved.step_kind)
        .collect();
    let covered = M5SequenceStepKind::ALL
        .iter()
        .all(|kind| exercised.contains(kind));
    if !covered {
        violations.push(M5SequenceHelpStripViolation::StepKindCoverageUnproven);
    }
}

/// At least one worked resolution must prove an awaiting-more, a dead-end, a conflicting, and a
/// disabled posture — the acceptance criterion that ambiguous or partial sequences are always
/// named for exactly what they are.
fn validate_posture_coverage(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.sequence_examples.iter())
    };
    let has_awaiting = cases().any(|case| case.resolved.help_posture.is_awaiting_more());
    let has_dead_end = cases().any(|case| case.resolved.help_posture.is_dead_end());
    let has_conflicting = cases().any(|case| case.resolved.help_posture.is_ambiguous());
    let has_disabled = cases().any(|case| case.resolved.help_posture.is_disabled());
    if !(has_awaiting && has_dead_end && has_conflicting && has_disabled) {
        violations.push(M5SequenceHelpStripViolation::PostureCoverageUnproven);
    }
}

/// At least one worked resolution must prove each of the show-valid-next-keys,
/// run-example-command, resolve-conflicting-binding, cancel-sequence, and open-full-cheat-sheet
/// actions — the implementation requirement that the strip offers next-key guidance, an example
/// command, and a full-cheat-sheet route.
fn validate_action_coverage(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.sequence_examples.iter())
    };
    let covered = M5SequenceHelpAction::ALL
        .iter()
        .all(|action| cases().any(|case| case.resolved.available_actions.contains(action)));
    if !covered {
        violations.push(M5SequenceHelpStripViolation::ActionCoverageUnproven);
    }
}

/// At least one worked resolution must prove that a dead-end or ambiguous sequence still keeps
/// cancel and the full cheat sheet reachable — the acceptance criterion that ambiguous or partial
/// sequences never fail silently or require external docs to interpret.
fn validate_non_silent_coverage(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let proven = packet
        .rows
        .iter()
        .flat_map(|row| row.sequence_examples.iter())
        .any(|case| {
            (case.resolved.is_dead_end || case.resolved.is_ambiguous)
                && case.resolved.cancel_available
                && case.resolved.cheat_sheet_available
                && case
                    .resolved
                    .available_actions
                    .contains(&M5SequenceHelpAction::CancelSequence)
                && case
                    .resolved
                    .available_actions
                    .contains(&M5SequenceHelpAction::OpenFullCheatSheet)
        });
    if !proven {
        violations.push(M5SequenceHelpStripViolation::NonSilentParityUnproven);
    }
}

/// Every worked resolution must keep keyboard-only parity — never requiring pointer hover and
/// always carrying a screen-reader announcement — the acceptance criterion that keyboard-first
/// users can learn command-language pathways entirely in-product.
fn validate_keyboard_parity_coverage(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let parity = packet
        .rows
        .iter()
        .flat_map(|row| row.sequence_examples.iter())
        .all(|case| {
            case.resolved.never_requires_pointer_hover
                && case.resolved.provides_screen_reader_announcement
        });
    if !parity {
        violations.push(M5SequenceHelpStripViolation::KeyboardParityUnproven);
    }
}

/// Every worked resolution must show the current mode, explain next keys or the dead end, show
/// the cancel key, and keep cancel / cheat sheet reachable — the acceptance criteria that a
/// sequence is always inspectable before it fails or surprises the user.
fn validate_reversibility(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.sequence_examples.iter())
        .all(|case| case.preserves_keyboard_parity());
    if !preserved {
        violations.push(M5SequenceHelpStripViolation::ReversibilityUnproven);
    }
}

/// Every worked resolution must preserve its exact strip identity, current mode, next keys,
/// cancel key, example command, announcement, and cheat-sheet reference — the invariant that the
/// strip never rewrites what it explains.
fn validate_identity_preservation(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.sequence_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5SequenceHelpStripViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.sequence_strip_shows_current_mode_or_leader,
        review.sequence_strip_shows_valid_next_keys,
        review.sequence_strip_shows_cancel_key,
        review.sequence_strip_shows_example_command,
        review.sequence_strip_opens_full_cheat_sheet,
        review.partial_or_ambiguous_sequences_never_fail_silently,
        review.keyboard_first_users_learn_pathways_in_product,
        review.sequence_strip_never_requires_pointer_hover,
        review.sequence_strip_provides_screen_reader_announcement,
        review.sequence_strip_preserves_command_backing,
        review.sequence_strips_stable_across_deployment_lines,
        review.sequence_strips_stable_across_consumer_surfaces,
        review.every_sequence_strip_declares_accessibility_route,
        review.support_export_reconstructs_sequence_truth,
        review.later_rows_cannot_invent_parallel_sequence_vocabulary,
    ] {
        if !ok {
            violations.push(M5SequenceHelpStripViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.command_language_surfaces_consume_sequence_vocabulary,
        projection.help_posture_reads_single_source,
        projection.action_set_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5SequenceHelpStripViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SequenceHelpStripViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SequenceHelpStripPacket,
    violations: &mut Vec<M5SequenceHelpStripViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.sequence_help_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SequenceHelpStripViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
