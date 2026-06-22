//! Canonical signature-help and snippet-session truth model: signature-help
//! cards and snippet-session strips bound into one inspectable, low-latency
//! typing-loop contract across the claimed editor families.
//!
//! Where the [completion-row model](crate::m5_completion_rows) freezes the one
//! shared *suggestion row*, and the [editor-assist matrix](crate::m5_editor_assist)
//! freezes the per-surface degraded-state *policy*, this module freezes the two
//! protected mid-typing surfaces that materially change input meaning while the
//! user is composing: the **signature-help card** (active overload and active
//! parameter) and the **snippet-session strip** (placeholder traversal, exit
//! path, and multi-cursor / IME safety). Before it, signature help and snippet
//! sessions were scattered across provider-specific panes: one pane let snippet
//! mode silently hijack Tab, another let a stale signature card sit over the
//! active line without a limited cue, a third dropped a multi-cursor snippet to
//! a single composition target with no visible explanation. This module folds
//! both into one governed session model that carries, for every surface:
//!
//! 1. **Source and provenance** — every card and strip embeds the canonical
//!    [`AssistSourceDescriptor`] so provider identity, support posture, freshness,
//!    locality, and degraded state travel with the surface.
//! 2. **The input-meaning truth** — a [`SignatureCard`] always exposes its active
//!    overload and active parameter; a [`SnippetStrip`] always exposes its
//!    placeholder count, active placeholder index, and a visible
//!    [`SnippetExitPath`], so anything that changes what the next keystroke means
//!    is visible rather than inferred.
//! 3. **IME / multi-cursor coherence** — each strip either stays coherent for the
//!    whole selection set or degrades *explicitly* to one composition target with
//!    a non-empty primary caret and a disclosed, screen-reader-announced cue.
//! 4. **No-hidden-side-effects truth** — when a snippet or signature-derived
//!    accept path also adds an import, generated scaffolding, a dependency, or a
//!    config edit, an [`AcceptSideEffectClass`] discloses it before commit and
//!    requires a preview where the effect is broad.
//! 5. **Blocked / degraded reasons** — large-file suppression, partial-index
//!    pending, restricted read-only routing, provider-unavailable, and
//!    stale-awaiting-refresh are named with an [`AssistBlockReason`] rather than
//!    appearing as a silent regression.
//!
//! Each claimed editor family resolves into a [`SignatureSnippetSnapshot`] that
//! pins its [`AssistDegradeClass`] posture and a visible label. Every card and
//! strip derives the canonical [`SignatureHelpRecord`] / [`SnippetSessionRecord`]
//! from the *same* data, proving the surface binding and the shared session model
//! cannot drift.
//!
//! The build is static and deterministic: [`signature_snippet_model`] assembles
//! the one canonical record, the checked-in fixture plus the replay gate freeze
//! it byte-for-byte, and the model proves its own honesty invariants over its
//! data. It carries no file contents, credential bodies, or raw provider
//! payloads, so support, AI, and migration surfaces can consume it directly.

use serde::{Deserialize, Serialize};

use aureline_language::{
    RouterCompletenessClass, RouterDegradedStateClass, RouterFreshnessClass, RouterLocalityClass,
    RouterScopeClaimClass, RouterSupportClass, ScopeLimitClass,
};

use crate::assist::{
    AssistSourceDescriptor, AssistSourceFamily, AssistSourceLabelClass, SignatureHelpInit,
    SignatureHelpRecord, SignaturePlacementClass, SnippetCursorPostureClass,
    SnippetImePostureClass, SnippetSessionInit, SnippetSessionRecord, SnippetSessionStateClass,
    SnippetTabBehaviorClass,
};
use crate::m5_editor_assist::{
    AssistDegradeClass, ClassDescriptor, EditorSurfaceClass, SignatureHelpStateClass,
};

/// Schema version for the signature-snippet model record.
pub const M5_SIGNATURE_SNIPPET_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the signature-snippet model record.
pub const M5_SIGNATURE_SNIPPET_SCHEMA_REF: &str = "schemas/editor/m5-signature-snippet.schema.json";

/// Stable record-kind tag for the signature-snippet model record.
pub const M5_SIGNATURE_SNIPPET_RECORD_KIND: &str = "m5_signature_snippet_model";

/// Stable id for the canonical signature-snippet model.
pub const M5_SIGNATURE_SNIPPET_MODEL_ID: &str = "m5-signature-snippet:model:0001";

/// Capture stamp for the canonical model. Held as a constant so the projection
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_SIGNATURE_SNIPPET_AS_OF: &str = "2026-06-22T00:00:00Z";

const SIGNATURE_DISMISS_COMMAND: &str = "command.editor.signature_help.dismiss";
const SNIPPET_NEXT_COMMAND: &str = "command.editor.snippet.next_placeholder";
const SNIPPET_PREV_COMMAND: &str = "command.editor.snippet.previous_placeholder";
const SNIPPET_EXIT_COMMAND: &str = "command.editor.snippet.exit";
const SNIPPET_ESCAPE_COMMAND: &str = "command.editor.snippet.cancel";

// ---------------------------------------------------------------------------
// Accept-side-effect cue — the no-hidden-side-effects truth.
// ---------------------------------------------------------------------------

/// What accepting a snippet expansion or a signature-derived completion does
/// beyond the current insertion range, disclosed before commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcceptSideEffectClass {
    /// Accept edits only the snippet / signature target range.
    EditsTargetRangeOnly,
    /// Accept also adds or rewrites an import.
    AddsImport,
    /// Accept also writes generated scaffolding routed through its generator.
    AddsGeneratedScaffolding,
    /// Accept also adds or changes a dependency.
    AddsDependency,
    /// Accept also edits configuration elsewhere.
    AddsConfigEdit,
}

impl AcceptSideEffectClass {
    /// All side-effect cues, in catalog order.
    pub const ALL: [Self; 5] = [
        Self::EditsTargetRangeOnly,
        Self::AddsImport,
        Self::AddsGeneratedScaffolding,
        Self::AddsDependency,
        Self::AddsConfigEdit,
    ];

    /// Returns the stable schema token for this cue.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditsTargetRangeOnly => "edits_target_range_only",
            Self::AddsImport => "adds_import",
            Self::AddsGeneratedScaffolding => "adds_generated_scaffolding",
            Self::AddsDependency => "adds_dependency",
            Self::AddsConfigEdit => "adds_config_edit",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EditsTargetRangeOnly => "Edits target range only",
            Self::AddsImport => "Adds an import",
            Self::AddsGeneratedScaffolding => "Adds generated scaffolding",
            Self::AddsDependency => "Adds a dependency",
            Self::AddsConfigEdit => "Edits configuration",
        }
    }

    /// Returns true when the effect must be disclosed before commit.
    pub const fn requires_pre_commit_disclosure(self) -> bool {
        !matches!(self, Self::EditsTargetRangeOnly)
    }

    /// Returns true when the broader effect requires a preview before applying.
    pub const fn requires_preview(self) -> bool {
        matches!(self, Self::AddsGeneratedScaffolding | Self::AddsDependency)
    }
}

// ---------------------------------------------------------------------------
// Blocked / degraded reason.
// ---------------------------------------------------------------------------

/// Named reason a signature card or snippet strip cannot run at full fidelity,
/// kept explicit so a degraded surface never reads as a silent regression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistBlockReason {
    /// The file is in large-file / restricted mode and assist is suppressed.
    LargeFileSuppressed,
    /// The semantic index is still building, so results are narrowed and labeled.
    PartialIndexPending,
    /// Reading is allowed but apply routes through a generator or review gate.
    RestrictedReadOnly,
    /// The provider is unavailable, so a labeled fallback is shown.
    ProviderUnavailable,
    /// A previous result is shown while a refresh is pending.
    StaleAwaitingRefresh,
}

impl AssistBlockReason {
    /// All block reasons, in catalog order.
    pub const ALL: [Self; 5] = [
        Self::LargeFileSuppressed,
        Self::PartialIndexPending,
        Self::RestrictedReadOnly,
        Self::ProviderUnavailable,
        Self::StaleAwaitingRefresh,
    ];

    /// Returns the stable schema token for this reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LargeFileSuppressed => "large_file_suppressed",
            Self::PartialIndexPending => "partial_index_pending",
            Self::RestrictedReadOnly => "restricted_read_only",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::StaleAwaitingRefresh => "stale_awaiting_refresh",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LargeFileSuppressed => "Suppressed in large-file mode",
            Self::PartialIndexPending => "Index still building",
            Self::RestrictedReadOnly => "Read-only — apply routes elsewhere",
            Self::ProviderUnavailable => "Provider unavailable — fallback shown",
            Self::StaleAwaitingRefresh => "Showing previous result — refresh pending",
        }
    }

    /// Returns true when the reason routes apply through a preview / review gate.
    pub const fn requires_preview(self) -> bool {
        matches!(self, Self::RestrictedReadOnly)
    }
}

// ---------------------------------------------------------------------------
// Snippet exit path.
// ---------------------------------------------------------------------------

/// The visible exit path for a snippet session, so the user always sees how to
/// leave and Tab is never captured invisibly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnippetExitPath {
    /// Command id that advances to the next placeholder.
    pub next_placeholder_command_id_ref: String,
    /// Command id that returns to the previous placeholder.
    pub previous_placeholder_command_id_ref: String,
    /// Command id that exits the session after the final placeholder.
    pub exit_command_id_ref: String,
    /// Command id that cancels the session immediately.
    pub escape_command_id_ref: String,
    /// Whether Tab on the final placeholder exits the session.
    pub exits_on_final_placeholder: bool,
    /// Human-readable description of how to leave the session.
    pub exit_label: String,
}

impl SnippetExitPath {
    /// Builds a standard exit path with the canonical command ids.
    fn standard(exits_on_final_placeholder: bool) -> Self {
        Self {
            next_placeholder_command_id_ref: SNIPPET_NEXT_COMMAND.to_owned(),
            previous_placeholder_command_id_ref: SNIPPET_PREV_COMMAND.to_owned(),
            exit_command_id_ref: SNIPPET_EXIT_COMMAND.to_owned(),
            escape_command_id_ref: SNIPPET_ESCAPE_COMMAND.to_owned(),
            exits_on_final_placeholder,
            exit_label: "Tab to advance, Shift+Tab to go back, Esc to leave the snippet".to_owned(),
        }
    }

    /// Returns true when the exit path is visible and actionable.
    pub fn is_visible(&self) -> bool {
        !self.exit_label.trim().is_empty()
            && !self.escape_command_id_ref.trim().is_empty()
            && !self.exit_command_id_ref.trim().is_empty()
    }
}

// ---------------------------------------------------------------------------
// Signature-help card.
// ---------------------------------------------------------------------------

/// One signature-help card bound for an editor surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignatureCard {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable card id.
    pub card_id: String,
    /// Lifecycle / visibility state of the card.
    pub state_class: SignatureHelpStateClass,
    /// Source and provenance descriptor.
    pub source: AssistSourceDescriptor,
    /// One-based active signature (overload) index.
    pub active_signature_index: u32,
    /// Total signature overload count.
    pub signature_count: u32,
    /// One-based active parameter index in the active signature.
    pub active_parameter_index: u32,
    /// Total parameter count in the active signature.
    pub parameter_count: u32,
    /// Placement posture for the card.
    pub placement_class: SignaturePlacementClass,
    /// Whether the card avoids focus capture.
    pub non_blocking: bool,
    /// Whether the card remains valid during IME composition.
    pub ime_composition_safe: bool,
    /// Whether the card overlaps the active editor line (must be false).
    pub obscures_active_line: bool,
    /// Whether a stale card discloses its limited / source cue.
    pub stale_disclosed: bool,
    /// Side effect of a signature-derived accept path.
    pub accept_side_effect: AcceptSideEffectClass,
    /// Plain-language summary of any additional edit on accept.
    pub side_effect_summary: Option<String>,
    /// Whether the additional edit must be disclosed before commit.
    pub commit_disclosure_required: bool,
    /// Whether a preview is required before applying.
    pub preview_required: bool,
    /// Named blocked / degraded reason, when the card is not full fidelity.
    pub blocked_reason: Option<AssistBlockReason>,
    /// Whether the card is fully keyboard reachable.
    pub keyboard_reachable: bool,
    /// Command id that dismisses the card.
    pub dismiss_command_id_ref: String,
    /// Non-color differentiator for stale / unavailable / fallback states.
    pub non_color_differentiator: String,
    /// Accessible summary for screen readers.
    pub accessibility_label: String,
    /// Canonical signature-help record derived from the same data.
    pub canonical_record: SignatureHelpRecord,
}

/// Initialization data for a [`SignatureCard`].
#[derive(Debug, Clone, PartialEq, Eq)]
struct SignatureCardInit {
    surface: EditorSurfaceClass,
    card_id: String,
    state_class: SignatureHelpStateClass,
    source: AssistSourceDescriptor,
    active_signature_index: u32,
    signature_count: u32,
    active_parameter_index: u32,
    parameter_count: u32,
    placement_class: SignaturePlacementClass,
    accept_side_effect: AcceptSideEffectClass,
    side_effect_summary: Option<String>,
    blocked_reason: Option<AssistBlockReason>,
}

impl SignatureCard {
    /// Stable record-kind tag for signature cards.
    pub const RECORD_KIND: &'static str = "m5_signature_card";

    fn new(init: SignatureCardInit) -> Self {
        let visible = signature_state_is_visible(init.state_class);
        let preview_required = init.accept_side_effect.requires_preview();
        let commit_disclosure_required =
            init.accept_side_effect.requires_pre_commit_disclosure() || preview_required;
        let stale_disclosed = matches!(
            init.state_class,
            SignatureHelpStateClass::StalePendingRefresh
        );

        let non_color_differentiator = match init.state_class {
            SignatureHelpStateClass::StalePendingRefresh => {
                "stale badge + \"refresh pending\" text".to_owned()
            }
            SignatureHelpStateClass::Unavailable => "dimmed + \"unavailable\" text".to_owned(),
            _ if init.source.requires_degraded_disclosure() => {
                "fallback badge + source label".to_owned()
            }
            _ => "source label text".to_owned(),
        };

        let accessibility_label = build_signature_accessibility_label(&init, visible);

        let canonical_record = SignatureHelpRecord::new(SignatureHelpInit {
            signature_help_id: init.card_id.clone(),
            assist_session_id: format!("signature-session:{}", init.surface.as_str()),
            document_ref: document_ref_for(init.surface),
            language_id: language_id_for(init.surface).to_owned(),
            invocation_anchor_ref: format!("anchor:{}:call_site", init.surface.as_str()),
            source: init.source.clone(),
            active_signature_index: init.active_signature_index,
            signature_count: init.signature_count,
            active_parameter_index: init.active_parameter_index,
            parameter_count: init.parameter_count,
            placement_class: init.placement_class,
            non_blocking: true,
            ime_composition_safe: true,
            dismiss_command_id_ref: SIGNATURE_DISMISS_COMMAND.to_owned(),
            captured_at: M5_SIGNATURE_SNIPPET_AS_OF.to_owned(),
        });

        Self {
            record_kind: Self::RECORD_KIND.to_owned(),
            card_id: init.card_id,
            state_class: init.state_class,
            source: init.source,
            active_signature_index: init.active_signature_index,
            signature_count: init.signature_count,
            active_parameter_index: init.active_parameter_index,
            parameter_count: init.parameter_count,
            placement_class: init.placement_class,
            non_blocking: true,
            ime_composition_safe: true,
            obscures_active_line: false,
            stale_disclosed,
            accept_side_effect: init.accept_side_effect,
            side_effect_summary: init.side_effect_summary,
            commit_disclosure_required,
            preview_required,
            blocked_reason: init.blocked_reason,
            keyboard_reachable: true,
            dismiss_command_id_ref: SIGNATURE_DISMISS_COMMAND.to_owned(),
            non_color_differentiator,
            accessibility_label,
            canonical_record,
        }
    }

    /// Returns true when the card is materially visible to the user.
    pub fn is_visible(&self) -> bool {
        signature_state_is_visible(self.state_class)
    }

    /// Returns true when the card exposes its active parameter coherently.
    pub fn exposes_active_parameter(&self) -> bool {
        self.parameter_count >= 1
            && self.active_parameter_index >= 1
            && self.active_parameter_index <= self.parameter_count
    }

    /// Returns true when an overloaded card exposes its active overload.
    pub fn exposes_active_overload(&self) -> bool {
        self.signature_count >= 2
            && self.active_signature_index >= 1
            && self.active_signature_index <= self.signature_count
    }

    /// Returns true when the card stays subordinate and non-blocking while typing.
    pub fn is_typing_loop_safe(&self) -> bool {
        self.non_blocking && self.ime_composition_safe && !self.obscures_active_line
    }
}

/// Returns true when a signature-help state is materially visible to the user.
fn signature_state_is_visible(state: SignatureHelpStateClass) -> bool {
    matches!(
        state,
        SignatureHelpStateClass::VisibleSingle
            | SignatureHelpStateClass::VisibleOverloaded
            | SignatureHelpStateClass::StalePendingRefresh
    )
}

fn build_signature_accessibility_label(init: &SignatureCardInit, visible: bool) -> String {
    let source = &init.source.source_label;
    if !visible {
        return format!(
            "Signature help unavailable on this surface ({}); source {source}.",
            init.blocked_reason
                .map(AssistBlockReason::label)
                .unwrap_or("not offered"),
        );
    }
    let overload = if init.signature_count >= 2 {
        format!(
            "overload {} of {}, ",
            init.active_signature_index, init.signature_count
        )
    } else {
        String::new()
    };
    let stale = if matches!(
        init.state_class,
        SignatureHelpStateClass::StalePendingRefresh
    ) {
        " Showing previous signature while a refresh is pending."
    } else {
        ""
    };
    format!(
        "Signature help from {source}; {overload}parameter {} of {}.{stale}",
        init.active_parameter_index, init.parameter_count,
    )
}

// ---------------------------------------------------------------------------
// Snippet-session strip.
// ---------------------------------------------------------------------------

/// One snippet-session strip bound for an editor surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnippetStrip {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable strip id.
    pub strip_id: String,
    /// Lifecycle state of the snippet session.
    pub state_class: SnippetSessionStateClass,
    /// Source and provenance descriptor.
    pub source: AssistSourceDescriptor,
    /// One-based active placeholder index.
    pub active_placeholder_index: Option<u32>,
    /// Total placeholder count in the session.
    pub placeholder_count: u32,
    /// Number of carets participating in the insertion.
    pub selection_count: u32,
    /// Whether multi-cursor traversal is supported.
    pub multi_cursor_compatible: bool,
    /// Tab-key behavior.
    pub tab_behavior_class: SnippetTabBehaviorClass,
    /// IME composition posture applied to traversal.
    pub ime_posture_class: SnippetImePostureClass,
    /// Cursor movement posture used to keep the session recoverable.
    pub cursor_posture_class: SnippetCursorPostureClass,
    /// Primary caret used when multi-cursor composition narrows to one target.
    pub primary_caret_ref: Option<String>,
    /// Whether the narrowed-composition cue must be announced.
    pub composition_disclosure_required: bool,
    /// Whether the snippet strip must be visible while active.
    pub visible_strip_required: bool,
    /// Whether Tab capture is disclosed rather than silent.
    pub tab_capture_disclosed: bool,
    /// Visible exit path for the session.
    pub exit_path: SnippetExitPath,
    /// Side effect of accepting / expanding the snippet.
    pub accept_side_effect: AcceptSideEffectClass,
    /// Plain-language summary of any additional edit on accept.
    pub side_effect_summary: Option<String>,
    /// Whether the additional edit must be disclosed before commit.
    pub commit_disclosure_required: bool,
    /// Whether a preview is required before applying.
    pub preview_required: bool,
    /// Named blocked / degraded reason, when the strip is not full fidelity.
    pub blocked_reason: Option<AssistBlockReason>,
    /// Whether the strip is fully keyboard reachable.
    pub keyboard_reachable: bool,
    /// Non-color differentiator for narrowed / blocked states.
    pub non_color_differentiator: String,
    /// Accessible summary for screen readers.
    pub accessibility_label: String,
    /// Canonical snippet-session record derived from the same data.
    pub canonical_record: SnippetSessionRecord,
}

/// Initialization data for a [`SnippetStrip`].
#[derive(Debug, Clone, PartialEq, Eq)]
struct SnippetStripInit {
    surface: EditorSurfaceClass,
    strip_id: String,
    state_class: SnippetSessionStateClass,
    source: AssistSourceDescriptor,
    active_placeholder_index: Option<u32>,
    placeholder_count: u32,
    selection_count: u32,
    multi_cursor_compatible: bool,
    tab_behavior_class: SnippetTabBehaviorClass,
    ime_posture_class: SnippetImePostureClass,
    cursor_posture_class: SnippetCursorPostureClass,
    primary_caret_ref: Option<String>,
    accept_side_effect: AcceptSideEffectClass,
    side_effect_summary: Option<String>,
    blocked_reason: Option<AssistBlockReason>,
}

impl SnippetStrip {
    /// Stable record-kind tag for snippet strips.
    pub const RECORD_KIND: &'static str = "m5_snippet_strip";

    fn new(init: SnippetStripInit) -> Self {
        let active = matches!(init.state_class, SnippetSessionStateClass::Active);
        let preview_required = init.accept_side_effect.requires_preview()
            || init
                .blocked_reason
                .is_some_and(AssistBlockReason::requires_preview);
        let commit_disclosure_required =
            init.accept_side_effect.requires_pre_commit_disclosure() || preview_required;
        let composition_disclosure_required = matches!(
            init.ime_posture_class,
            SnippetImePostureClass::CompositionPrimaryCaretOnly
                | SnippetImePostureClass::CompositionBlocked
        );
        let visible_strip_required = active;
        let tab_capture_disclosed = active;
        let exit_path = SnippetExitPath::standard(
            init.tab_behavior_class != SnippetTabBehaviorClass::PassThroughOutsideSession,
        );

        let non_color_differentiator = if composition_disclosure_required {
            "narrowed-composition badge + caret marker".to_owned()
        } else if init.blocked_reason.is_some() {
            "blocked badge + reason text".to_owned()
        } else {
            "snippet outline + placeholder markers".to_owned()
        };

        let accessibility_label = build_snippet_accessibility_label(&init, active);

        let canonical_record = SnippetSessionRecord::new(SnippetSessionInit {
            snippet_session_id: init.strip_id.clone(),
            document_ref: document_ref_for(init.surface),
            language_id: language_id_for(init.surface).to_owned(),
            source: init.source.clone(),
            state_class: init.state_class,
            active_placeholder_index: init.active_placeholder_index,
            placeholder_count: init.placeholder_count,
            selection_count: init.selection_count,
            multi_cursor_compatible: init.multi_cursor_compatible,
            tab_behavior_class: init.tab_behavior_class,
            ime_posture_class: init.ime_posture_class,
            cursor_posture_class: init.cursor_posture_class,
            primary_caret_ref: init.primary_caret_ref.clone(),
            next_placeholder_command_id_ref: SNIPPET_NEXT_COMMAND.to_owned(),
            previous_placeholder_command_id_ref: SNIPPET_PREV_COMMAND.to_owned(),
            exit_command_id_ref: SNIPPET_EXIT_COMMAND.to_owned(),
            escape_command_id_ref: SNIPPET_ESCAPE_COMMAND.to_owned(),
            visible_strip_required,
            captured_at: M5_SIGNATURE_SNIPPET_AS_OF.to_owned(),
        });

        Self {
            record_kind: Self::RECORD_KIND.to_owned(),
            strip_id: init.strip_id,
            state_class: init.state_class,
            source: init.source,
            active_placeholder_index: init.active_placeholder_index,
            placeholder_count: init.placeholder_count,
            selection_count: init.selection_count,
            multi_cursor_compatible: init.multi_cursor_compatible,
            tab_behavior_class: init.tab_behavior_class,
            ime_posture_class: init.ime_posture_class,
            cursor_posture_class: init.cursor_posture_class,
            primary_caret_ref: init.primary_caret_ref,
            composition_disclosure_required,
            visible_strip_required,
            tab_capture_disclosed,
            exit_path,
            accept_side_effect: init.accept_side_effect,
            side_effect_summary: init.side_effect_summary,
            commit_disclosure_required,
            preview_required,
            blocked_reason: init.blocked_reason,
            keyboard_reachable: true,
            non_color_differentiator,
            accessibility_label,
            canonical_record,
        }
    }

    /// Returns true when placeholder traversal is active.
    pub fn is_active(&self) -> bool {
        matches!(self.state_class, SnippetSessionStateClass::Active)
    }

    /// Returns true when snippet traversal owns the Tab key.
    pub fn captures_tab(&self) -> bool {
        self.canonical_record.captures_tab()
    }

    /// Returns true when the active session exposes a visible exit path and a
    /// coherent active placeholder index.
    pub fn exposes_exit_path(&self) -> bool {
        if !self.is_active() {
            return true;
        }
        self.exit_path.is_visible()
            && self.placeholder_count >= 1
            && self
                .active_placeholder_index
                .is_some_and(|index| index >= 1 && index <= self.placeholder_count)
    }

    /// Returns true when the session is coherent for the whole selection set or
    /// degrades explicitly to one disclosed composition target.
    pub fn ime_and_multicursor_coherent_or_degraded(&self) -> bool {
        let base = self.canonical_record.is_keyboard_and_ime_safe();
        if matches!(
            self.ime_posture_class,
            SnippetImePostureClass::CompositionPrimaryCaretOnly
        ) {
            base && self.composition_disclosure_required
                && self
                    .primary_caret_ref
                    .as_ref()
                    .is_some_and(|caret| !caret.trim().is_empty())
        } else {
            base
        }
    }

    /// Returns true when an active Tab-capturing session discloses its capture.
    pub fn does_not_hijack_tab(&self) -> bool {
        if self.captures_tab() {
            self.visible_strip_required && self.tab_capture_disclosed
        } else {
            true
        }
    }
}

fn build_snippet_accessibility_label(init: &SnippetStripInit, active: bool) -> String {
    if !active {
        return format!(
            "Snippet assist not active on this surface ({}); source {}.",
            init.blocked_reason
                .map(AssistBlockReason::label)
                .unwrap_or("inactive"),
            init.source.source_label,
        );
    }
    let placeholder = format!(
        "placeholder {} of {}",
        init.active_placeholder_index.unwrap_or(1),
        init.placeholder_count,
    );
    let composition = match init.ime_posture_class {
        SnippetImePostureClass::CompositionActivePassThrough => {
            " IME composition active; Tab passes through until composition ends."
        }
        SnippetImePostureClass::CompositionPrimaryCaretOnly => {
            " IME composition narrowed to one primary caret; other carets pause until composition ends."
        }
        SnippetImePostureClass::CompositionBlocked => {
            " IME composition cannot continue; snippet traversal paused."
        }
        SnippetImePostureClass::NoComposition => "",
    };
    format!(
        "Snippet session from {source}; {placeholder}. Tab advances, Esc leaves.{composition}",
        source = init.source.source_label,
    )
}

// ---------------------------------------------------------------------------
// Surface snapshot.
// ---------------------------------------------------------------------------

/// One claimed editor family resolved into its signature card and snippet strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignatureSnippetSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub m5_signature_snippet_schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Editor surface family covered by this snapshot.
    pub surface_class: EditorSurfaceClass,
    /// Workspace id covered by the snapshot.
    pub workspace_id: String,
    /// Document ref covered by the snapshot.
    pub document_ref: String,
    /// Language id resolved for the document.
    pub language_id: String,
    /// Degraded-state posture for the surface.
    pub degrade_class: AssistDegradeClass,
    /// Visible degrade label.
    pub degrade_label: String,
    /// Signature-help card, when this surface offers one.
    pub signature_card: Option<SignatureCard>,
    /// Snippet-session strip, when this surface offers one.
    pub snippet_strip: Option<SnippetStrip>,
    /// Whether the snapshot needs source / fallback / commit disclosure.
    pub disclosure_required: bool,
    /// Accessible summary for screen readers.
    pub accessibility_summary: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl SignatureSnippetSnapshot {
    /// Stable record-kind tag for signature-snippet snapshots.
    pub const RECORD_KIND: &'static str = "m5_signature_snippet_snapshot";
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

/// One frozen honesty invariant the model must satisfy, with the result of
/// evaluating it over the model's own data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignatureSnippetInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// Human-readable statement of the invariant.
    pub statement: String,
    /// Whether the invariant holds on the built model.
    pub holds: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// The canonical, frozen, export-safe signature-help and snippet-session model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignatureSnippetModel {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_signature_snippet_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable model id.
    pub model_id: String,
    /// Capture stamp.
    pub as_of: String,
    /// Signature-state catalog.
    pub signature_state_classes: Vec<ClassDescriptor>,
    /// Snippet-state catalog.
    pub snippet_state_classes: Vec<ClassDescriptor>,
    /// IME-posture catalog.
    pub ime_posture_classes: Vec<ClassDescriptor>,
    /// Cursor-posture catalog.
    pub cursor_posture_classes: Vec<ClassDescriptor>,
    /// Accept-side-effect catalog.
    pub accept_side_effect_classes: Vec<ClassDescriptor>,
    /// Block-reason catalog.
    pub block_reason_classes: Vec<ClassDescriptor>,
    /// One snapshot per claimed editor family.
    pub surface_snapshots: Vec<SignatureSnippetSnapshot>,
    /// Frozen invariants and whether each holds on this model.
    pub invariants: Vec<SignatureSnippetInvariant>,
    /// Whether the model is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl SignatureSnippetModel {
    /// Returns true when every frozen invariant holds on this model.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the model is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == M5_SIGNATURE_SNIPPET_SCHEMA_REF
            && self.record_kind == M5_SIGNATURE_SNIPPET_RECORD_KIND
    }

    /// Returns the snapshot for the given surface, when present.
    pub fn snapshot(&self, surface: EditorSurfaceClass) -> Option<&SignatureSnippetSnapshot> {
        self.surface_snapshots
            .iter()
            .find(|snapshot| snapshot.surface_class == surface)
    }

    /// Returns every signature card across every snapshot.
    pub fn all_cards(&self) -> impl Iterator<Item = &SignatureCard> {
        self.surface_snapshots
            .iter()
            .filter_map(|snapshot| snapshot.signature_card.as_ref())
    }

    /// Returns every snippet strip across every snapshot.
    pub fn all_strips(&self) -> impl Iterator<Item = &SnippetStrip> {
        self.surface_snapshots
            .iter()
            .filter_map(|snapshot| snapshot.snippet_strip.as_ref())
    }
}

// ---------------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------------

fn document_ref_for(surface: EditorSurfaceClass) -> String {
    let path = match surface {
        EditorSurfaceClass::CodeFile => "src/render.rs",
        EditorSurfaceClass::ConfigFile => "Cargo.toml",
        EditorSurfaceClass::NotebookCell => "analysis.ipynb#cell-3",
        EditorSurfaceClass::RequestEditor => "requests/list_users.http",
        EditorSurfaceClass::SqlEditor => "queries/active_users.sql",
        EditorSurfaceClass::DocsCodeBlock => "docs/guide.md#example-2",
        EditorSurfaceClass::GeneratedFile => "target/generated/schema.rs",
        EditorSurfaceClass::ProtectedFile => "infra/policy.toml",
        EditorSurfaceClass::PartialIndexState => "src/pipeline.rs",
        EditorSurfaceClass::LargeFileRestricted => "logs/trace.log",
    };
    format!("doc:{path}")
}

const fn language_id_for(surface: EditorSurfaceClass) -> &'static str {
    match surface {
        EditorSurfaceClass::CodeFile
        | EditorSurfaceClass::GeneratedFile
        | EditorSurfaceClass::PartialIndexState => "rust",
        EditorSurfaceClass::ConfigFile | EditorSurfaceClass::ProtectedFile => "toml",
        EditorSurfaceClass::NotebookCell => "python",
        EditorSurfaceClass::RequestEditor => "http",
        EditorSurfaceClass::SqlEditor => "sql",
        EditorSurfaceClass::DocsCodeBlock => "markdown",
        EditorSurfaceClass::LargeFileRestricted => "log",
    }
}

#[allow(clippy::too_many_arguments)]
fn source(
    surface: EditorSurfaceClass,
    family: AssistSourceFamily,
    provider_id: Option<&str>,
    provider_label: &str,
    support: RouterSupportClass,
    freshness: RouterFreshnessClass,
    scope: RouterScopeClaimClass,
    completeness: RouterCompletenessClass,
    locality: RouterLocalityClass,
    degraded: RouterDegradedStateClass,
    scope_limits: Vec<ScopeLimitClass>,
    summary: &str,
) -> AssistSourceDescriptor {
    AssistSourceDescriptor {
        source_descriptor_id: format!(
            "signature-snippet-source:{}:{}",
            surface.as_str(),
            family.as_str()
        ),
        source_family: family,
        source_label_class: AssistSourceLabelClass::from_source_family(family),
        source_label: provider_label.to_owned(),
        provider_id: provider_id.map(str::to_owned),
        router_decision_ref: provider_id
            .map(|id| format!("router-decision:{}:{id}", surface.as_str())),
        source_ref: None,
        support_class: support,
        freshness_class: freshness,
        scope_claim_class: scope,
        completeness_class: completeness,
        scope_limit_classes: scope_limits,
        locality_class: locality,
        degraded_state_class: degraded,
        summary: summary.to_owned(),
    }
}

fn class_descriptor(token: &str, label: &str, note: &str) -> ClassDescriptor {
    ClassDescriptor {
        class_token: token.to_owned(),
        label: label.to_owned(),
        note: note.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Builder.
// ---------------------------------------------------------------------------

/// Builds the one canonical signature-help and snippet-session model.
///
/// The build is deterministic and self-contained: it materializes one
/// [`SignatureSnippetSnapshot`] per claimed editor family, derives the canonical
/// shared [`SignatureHelpRecord`] / [`SnippetSessionRecord`] from the same data,
/// and evaluates every frozen honesty invariant over the assembled data so the
/// record's `invariants[].holds` reflect real checks.
pub fn signature_snippet_model() -> SignatureSnippetModel {
    let surface_snapshots = build_surface_snapshots();
    let invariants = evaluate_invariants(&surface_snapshots);

    let qualified = invariants.iter().all(|invariant| invariant.holds);
    let cards = surface_snapshots
        .iter()
        .filter(|snapshot| snapshot.signature_card.is_some())
        .count();
    let strips = surface_snapshots
        .iter()
        .filter(|snapshot| snapshot.snippet_strip.is_some())
        .count();
    let summary = if qualified {
        format!(
            "Signature-snippet model frozen: {cards} signature cards and {strips} snippet strips \
             across {surfaces} editor families. Every card exposes its active overload and \
             parameter, every active strip exposes its placeholder count and exit path, IME and \
             multi-cursor either stay coherent or degrade to one disclosed target, snippet mode \
             never hijacks Tab invisibly, and every accept side effect discloses before commit. \
             All {invariants} invariants hold.",
            cards = cards,
            strips = strips,
            surfaces = surface_snapshots.len(),
            invariants = invariants.len(),
        )
    } else {
        format!(
            "Signature-snippet model INVALID: {failing} of {total} invariants do not hold.",
            failing = invariants.iter().filter(|i| !i.holds).count(),
            total = invariants.len(),
        )
    };

    SignatureSnippetModel {
        record_kind: M5_SIGNATURE_SNIPPET_RECORD_KIND.to_owned(),
        m5_signature_snippet_schema_version: M5_SIGNATURE_SNIPPET_SCHEMA_VERSION,
        schema_ref: M5_SIGNATURE_SNIPPET_SCHEMA_REF.to_owned(),
        model_id: M5_SIGNATURE_SNIPPET_MODEL_ID.to_owned(),
        as_of: M5_SIGNATURE_SNIPPET_AS_OF.to_owned(),
        signature_state_classes: build_signature_state_catalog(),
        snippet_state_classes: build_snippet_state_catalog(),
        ime_posture_classes: build_ime_posture_catalog(),
        cursor_posture_classes: build_cursor_posture_catalog(),
        accept_side_effect_classes: build_accept_side_effect_catalog(),
        block_reason_classes: build_block_reason_catalog(),
        surface_snapshots,
        invariants,
        raw_payload_excluded: true,
        summary,
    }
}

/// Builds the human-readable projection of the model for support and headless use.
pub fn signature_snippet_model_lines(model: &SignatureSnippetModel) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Signature-snippet model — {} ({})",
        model.model_id, model.as_of
    ));
    lines.push(format!(
        "schema_ref={} version={}",
        model.schema_ref, model.m5_signature_snippet_schema_version
    ));

    lines.push("Surface snapshots:".to_owned());
    for snapshot in &model.surface_snapshots {
        lines.push(format!(
            "  {surface}: {degrade} ({label}) — disclosure={disclosure}",
            surface = snapshot.surface_class.as_str(),
            degrade = snapshot.degrade_class.as_str(),
            label = snapshot.degrade_label,
            disclosure = snapshot.disclosure_required,
        ));
        if let Some(card) = &snapshot.signature_card {
            lines.push(format!(
                "    signature: state={state} overload={sig}/{sigc} parameter={par}/{parc} \
                 side_effect={effect} blocked={blocked}",
                state = card.state_class.as_str(),
                sig = card.active_signature_index,
                sigc = card.signature_count,
                par = card.active_parameter_index,
                parc = card.parameter_count,
                effect = card.accept_side_effect.as_str(),
                blocked = card
                    .blocked_reason
                    .map(AssistBlockReason::as_str)
                    .unwrap_or("none"),
            ));
        }
        if let Some(strip) = &snapshot.snippet_strip {
            lines.push(format!(
                "    snippet: state={state} placeholder={ph:?}/{phc} ime={ime} cursor={cursor} \
                 captures_tab={tab} side_effect={effect} preview={preview} blocked={blocked}",
                state = snippet_state_token(strip.state_class),
                ph = strip.active_placeholder_index,
                phc = strip.placeholder_count,
                ime = ime_posture_token(strip.ime_posture_class),
                cursor = cursor_posture_token(strip.cursor_posture_class),
                tab = strip.captures_tab(),
                effect = strip.accept_side_effect.as_str(),
                preview = strip.preview_required,
                blocked = strip
                    .blocked_reason
                    .map(AssistBlockReason::as_str)
                    .unwrap_or("none"),
            ));
        }
    }

    lines.push("Invariants:".to_owned());
    for invariant in &model.invariants {
        lines.push(format!(
            "  {id} holds={holds}",
            id = invariant.invariant_id,
            holds = invariant.holds,
        ));
    }

    lines.push(model.summary.clone());
    lines
}

// ---------------------------------------------------------------------------
// Catalog builders.
// ---------------------------------------------------------------------------

fn build_signature_state_catalog() -> Vec<ClassDescriptor> {
    SignatureHelpStateClass::ALL
        .iter()
        .map(|state| {
            let note = match state {
                SignatureHelpStateClass::Hidden => "No signature help is shown.",
                SignatureHelpStateClass::VisibleSingle => {
                    "One signature; active parameter must be visible."
                }
                SignatureHelpStateClass::VisibleOverloaded => {
                    "Overload set; active overload and active parameter must both be visible."
                }
                SignatureHelpStateClass::StalePendingRefresh => {
                    "Previous signature shown with a limited / refresh-pending cue."
                }
                SignatureHelpStateClass::Unavailable => {
                    "Not offered on this surface; reason is named."
                }
            };
            class_descriptor(state.as_str(), state.label(), note)
        })
        .collect()
}

fn build_snippet_state_catalog() -> Vec<ClassDescriptor> {
    [
        (
            SnippetSessionStateClass::Inactive,
            "Inactive",
            "No snippet session owns placeholder traversal.",
        ),
        (
            SnippetSessionStateClass::Active,
            "Active",
            "Session owns placeholder traversal; strip and exit path are visible.",
        ),
        (
            SnippetSessionStateClass::Exited,
            "Exited",
            "Session ended after the final placeholder.",
        ),
        (
            SnippetSessionStateClass::Cancelled,
            "Cancelled",
            "Session was cancelled by the user.",
        ),
    ]
    .into_iter()
    .map(|(state, label, note)| class_descriptor(snippet_state_token(state), label, note))
    .collect()
}

fn build_ime_posture_catalog() -> Vec<ClassDescriptor> {
    [
        (
            SnippetImePostureClass::NoComposition,
            "No composition",
            "No IME composition; snippet navigation may own Tab.",
        ),
        (
            SnippetImePostureClass::CompositionActivePassThrough,
            "Composition active (pass through)",
            "Composition active; composition keys pass through coherently for the selection set.",
        ),
        (
            SnippetImePostureClass::CompositionPrimaryCaretOnly,
            "Composition primary caret only",
            "Narrowed to one primary caret with a disclosed, announced cue.",
        ),
        (
            SnippetImePostureClass::CompositionBlocked,
            "Composition blocked",
            "Composition cannot safely continue; traversal pauses with a cue.",
        ),
    ]
    .into_iter()
    .map(|(posture, label, note)| class_descriptor(ime_posture_token(posture), label, note))
    .collect()
}

fn build_cursor_posture_catalog() -> Vec<ClassDescriptor> {
    [
        (
            SnippetCursorPostureClass::Stable,
            "Stable",
            "Active caret stays anchored to the current placeholder.",
        ),
        (
            SnippetCursorPostureClass::Remapped,
            "Remapped",
            "Session remapped the caret to the equivalent placeholder.",
        ),
        (
            SnippetCursorPostureClass::RecoverableAfterMovement,
            "Recoverable after movement",
            "Rapid movement left a recoverable visible strip.",
        ),
        (
            SnippetCursorPostureClass::LostOutsideSession,
            "Lost outside session",
            "Caret moved outside a recoverable snippet range.",
        ),
    ]
    .into_iter()
    .map(|(posture, label, note)| class_descriptor(cursor_posture_token(posture), label, note))
    .collect()
}

fn build_accept_side_effect_catalog() -> Vec<ClassDescriptor> {
    AcceptSideEffectClass::ALL
        .iter()
        .map(|effect| {
            let note = format!(
                "{}{}",
                if effect.requires_pre_commit_disclosure() {
                    "Discloses before commit"
                } else {
                    "No additional edit"
                },
                if effect.requires_preview() {
                    "; preview required."
                } else {
                    "."
                },
            );
            class_descriptor(effect.as_str(), effect.label(), &note)
        })
        .collect()
}

fn build_block_reason_catalog() -> Vec<ClassDescriptor> {
    AssistBlockReason::ALL
        .iter()
        .map(|reason| {
            let note = if reason.requires_preview() {
                "Disclosed; apply routes through a preview / review gate."
            } else {
                "Disclosed with a visible labeled cue."
            };
            class_descriptor(reason.as_str(), reason.label(), note)
        })
        .collect()
}

const fn snippet_state_token(state: SnippetSessionStateClass) -> &'static str {
    match state {
        SnippetSessionStateClass::Inactive => "inactive",
        SnippetSessionStateClass::Active => "active",
        SnippetSessionStateClass::Exited => "exited",
        SnippetSessionStateClass::Cancelled => "cancelled",
    }
}

const fn ime_posture_token(posture: SnippetImePostureClass) -> &'static str {
    match posture {
        SnippetImePostureClass::NoComposition => "no_composition",
        SnippetImePostureClass::CompositionActivePassThrough => "composition_active_pass_through",
        SnippetImePostureClass::CompositionPrimaryCaretOnly => "composition_primary_caret_only",
        SnippetImePostureClass::CompositionBlocked => "composition_blocked",
    }
}

const fn cursor_posture_token(posture: SnippetCursorPostureClass) -> &'static str {
    match posture {
        SnippetCursorPostureClass::Stable => "stable",
        SnippetCursorPostureClass::Remapped => "remapped",
        SnippetCursorPostureClass::RecoverableAfterMovement => "recoverable_after_movement",
        SnippetCursorPostureClass::LostOutsideSession => "lost_outside_session",
    }
}

// ---------------------------------------------------------------------------
// Snapshot assembly.
// ---------------------------------------------------------------------------

struct SnapshotSpec {
    surface: EditorSurfaceClass,
    workspace_id: &'static str,
    degrade_class: AssistDegradeClass,
    degrade_label: &'static str,
    signature_card: Option<SignatureCard>,
    snippet_strip: Option<SnippetStrip>,
}

fn assemble_snapshot(spec: SnapshotSpec) -> SignatureSnippetSnapshot {
    let surface = spec.surface;
    let card_blocked = spec
        .signature_card
        .as_ref()
        .is_some_and(|card| card.blocked_reason.is_some());
    let strip_blocked = spec
        .snippet_strip
        .as_ref()
        .is_some_and(|strip| strip.blocked_reason.is_some());
    let card_discloses = spec
        .signature_card
        .as_ref()
        .is_some_and(|card| card.commit_disclosure_required);
    let strip_discloses = spec
        .snippet_strip
        .as_ref()
        .is_some_and(|strip| strip.commit_disclosure_required);
    let disclosure_required = spec.degrade_class != AssistDegradeClass::FullFidelity
        || card_blocked
        || strip_blocked
        || card_discloses
        || strip_discloses;

    let mut summary_parts: Vec<String> = Vec::new();
    if let Some(card) = &spec.signature_card {
        summary_parts.push(format!(
            "signature card {} ({} overload(s), parameter {}/{})",
            card.state_class.as_str(),
            card.signature_count,
            card.active_parameter_index,
            card.parameter_count,
        ));
    }
    if let Some(strip) = &spec.snippet_strip {
        summary_parts.push(format!(
            "snippet strip {} ({} placeholder(s))",
            snippet_state_token(strip.state_class),
            strip.placeholder_count,
        ));
    }
    if summary_parts.is_empty() {
        summary_parts.push("no signature or snippet surface".to_owned());
    }
    let joined = summary_parts.join("; ");
    let accessibility_summary = format!("{surface}: {joined}.", surface = surface.label(),);
    let export_safe_summary = format!(
        "{surface} resolves {joined}; posture {posture}.",
        surface = surface.as_str(),
        posture = spec.degrade_class.as_str(),
    );

    SignatureSnippetSnapshot {
        record_kind: SignatureSnippetSnapshot::RECORD_KIND.to_owned(),
        m5_signature_snippet_schema_version: M5_SIGNATURE_SNIPPET_SCHEMA_VERSION,
        snapshot_id: format!("signature-snippet:{}", surface.as_str()),
        surface_class: surface,
        workspace_id: spec.workspace_id.to_owned(),
        document_ref: document_ref_for(surface),
        language_id: language_id_for(surface).to_owned(),
        degrade_class: spec.degrade_class,
        degrade_label: spec.degrade_label.to_owned(),
        signature_card: spec.signature_card,
        snippet_strip: spec.snippet_strip,
        disclosure_required,
        accessibility_summary,
        export_safe_summary,
    }
}

fn build_surface_snapshots() -> Vec<SignatureSnippetSnapshot> {
    vec![
        build_code_file_snapshot(),
        build_config_file_snapshot(),
        build_notebook_cell_snapshot(),
        build_request_editor_snapshot(),
        build_sql_editor_snapshot(),
        build_docs_code_block_snapshot(),
        build_generated_file_snapshot(),
        build_protected_file_snapshot(),
        build_partial_index_snapshot(),
        build_large_file_snapshot(),
    ]
}

fn build_code_file_snapshot() -> SignatureSnippetSnapshot {
    let surface = EditorSurfaceClass::CodeFile;
    let card = SignatureCard::new(SignatureCardInit {
        surface,
        card_id: "code-signature".to_owned(),
        state_class: SignatureHelpStateClass::VisibleOverloaded,
        source: source(
            surface,
            AssistSourceFamily::LanguageServer,
            Some("rust-analyzer"),
            "rust-analyzer",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::WholeWorkspace,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalSidecar,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Overloaded function signatures from the language server.",
        ),
        active_signature_index: 2,
        signature_count: 3,
        active_parameter_index: 2,
        parameter_count: 3,
        placement_class: SignaturePlacementClass::NonBlockingNearCaret,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: None,
    });
    let strip = SnippetStrip::new(SnippetStripInit {
        surface,
        strip_id: "code-snippet".to_owned(),
        state_class: SnippetSessionStateClass::Active,
        source: source(
            surface,
            AssistSourceFamily::Snippet,
            None,
            "Snippet pack",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Multi-cursor function-scaffold snippet that adds a use import on accept.",
        ),
        active_placeholder_index: Some(1),
        placeholder_count: 3,
        selection_count: 3,
        multi_cursor_compatible: true,
        tab_behavior_class: SnippetTabBehaviorClass::TraversePlaceholdersWhileActive,
        ime_posture_class: SnippetImePostureClass::NoComposition,
        cursor_posture_class: SnippetCursorPostureClass::Stable,
        primary_caret_ref: None,
        accept_side_effect: AcceptSideEffectClass::AddsImport,
        side_effect_summary: Some(
            "Adds `use std::time::Duration;` to the import block.".to_owned(),
        ),
        blocked_reason: None,
    });
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        signature_card: Some(card),
        snippet_strip: Some(strip),
    })
}

fn build_config_file_snapshot() -> SignatureSnippetSnapshot {
    let surface = EditorSurfaceClass::ConfigFile;
    let strip = SnippetStrip::new(SnippetStripInit {
        surface,
        strip_id: "config-snippet".to_owned(),
        state_class: SnippetSessionStateClass::Active,
        source: source(
            surface,
            AssistSourceFamily::FrameworkPack,
            Some("schema-pack:cargo"),
            "Schema pack",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Schema-backed dependency-entry snippet; accepting adds a dependency.",
        ),
        active_placeholder_index: Some(1),
        placeholder_count: 2,
        selection_count: 1,
        multi_cursor_compatible: false,
        tab_behavior_class: SnippetTabBehaviorClass::TraversePlaceholdersWhileActive,
        ime_posture_class: SnippetImePostureClass::NoComposition,
        cursor_posture_class: SnippetCursorPostureClass::Stable,
        primary_caret_ref: None,
        accept_side_effect: AcceptSideEffectClass::AddsDependency,
        side_effect_summary: Some(
            "Adds a `[dependencies]` entry and updates the lockfile; preview first.".to_owned(),
        ),
        blocked_reason: None,
    });
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        signature_card: None,
        snippet_strip: Some(strip),
    })
}

fn build_notebook_cell_snapshot() -> SignatureSnippetSnapshot {
    let surface = EditorSurfaceClass::NotebookCell;
    let card = SignatureCard::new(SignatureCardInit {
        surface,
        card_id: "notebook-signature".to_owned(),
        state_class: SignatureHelpStateClass::VisibleSingle,
        source: source(
            surface,
            AssistSourceFamily::LanguageServer,
            Some("pyright"),
            "Pyright",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::NotebookCell,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Notebook-cell-scoped function signature.",
        ),
        active_signature_index: 1,
        signature_count: 1,
        active_parameter_index: 1,
        parameter_count: 2,
        placement_class: SignaturePlacementClass::NonBlockingNearCaret,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: None,
    });
    // Multi-cursor IME composition narrows to one primary caret with a disclosed cue.
    let strip = SnippetStrip::new(SnippetStripInit {
        surface,
        strip_id: "notebook-snippet".to_owned(),
        state_class: SnippetSessionStateClass::Active,
        source: source(
            surface,
            AssistSourceFamily::Snippet,
            None,
            "Snippet pack",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::NotebookCell,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Multi-cursor snippet under active IME composition, narrowed to one caret.",
        ),
        active_placeholder_index: Some(1),
        placeholder_count: 2,
        selection_count: 3,
        multi_cursor_compatible: true,
        tab_behavior_class: SnippetTabBehaviorClass::TraversePlaceholdersWhileActive,
        ime_posture_class: SnippetImePostureClass::CompositionPrimaryCaretOnly,
        cursor_posture_class: SnippetCursorPostureClass::Remapped,
        primary_caret_ref: Some("caret:notebook:primary".to_owned()),
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: None,
    });
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        signature_card: Some(card),
        snippet_strip: Some(strip),
    })
}

fn build_request_editor_snapshot() -> SignatureSnippetSnapshot {
    let surface = EditorSurfaceClass::RequestEditor;
    let card = SignatureCard::new(SignatureCardInit {
        surface,
        card_id: "request-signature".to_owned(),
        state_class: SignatureHelpStateClass::VisibleSingle,
        source: source(
            surface,
            AssistSourceFamily::FrameworkPack,
            Some("http-template"),
            "Request template helpers",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Template-function signature for a request variable helper.",
        ),
        active_signature_index: 1,
        signature_count: 1,
        active_parameter_index: 1,
        parameter_count: 2,
        placement_class: SignaturePlacementClass::NonBlockingNearCaret,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: None,
    });
    let strip = SnippetStrip::new(SnippetStripInit {
        surface,
        strip_id: "request-snippet".to_owned(),
        state_class: SnippetSessionStateClass::Active,
        source: source(
            surface,
            AssistSourceFamily::Snippet,
            None,
            "Request snippet pack",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Authorization-header scaffold snippet within the request body.",
        ),
        active_placeholder_index: Some(1),
        placeholder_count: 2,
        selection_count: 1,
        multi_cursor_compatible: false,
        tab_behavior_class: SnippetTabBehaviorClass::TraversePlaceholdersWhileActive,
        ime_posture_class: SnippetImePostureClass::NoComposition,
        cursor_posture_class: SnippetCursorPostureClass::Stable,
        primary_caret_ref: None,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: None,
    });
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        signature_card: Some(card),
        snippet_strip: Some(strip),
    })
}

fn build_sql_editor_snapshot() -> SignatureSnippetSnapshot {
    let surface = EditorSurfaceClass::SqlEditor;
    let card = SignatureCard::new(SignatureCardInit {
        surface,
        card_id: "sql-signature".to_owned(),
        state_class: SignatureHelpStateClass::VisibleSingle,
        source: source(
            surface,
            AssistSourceFamily::FallbackLexical,
            None,
            "SQL dialect fallback",
            RouterSupportClass::FallbackOnly,
            RouterFreshnessClass::WarmCached,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::PartialForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::DegradedProviderUnavailable,
            vec![ScopeLimitClass::SingleFileOnly],
            "No live database connection; signature derived from dialect fallback.",
        ),
        active_signature_index: 1,
        signature_count: 1,
        active_parameter_index: 1,
        parameter_count: 2,
        placement_class: SignaturePlacementClass::NonBlockingNearCaret,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: Some(AssistBlockReason::ProviderUnavailable),
    });
    let strip = SnippetStrip::new(SnippetStripInit {
        surface,
        strip_id: "sql-snippet".to_owned(),
        state_class: SnippetSessionStateClass::Active,
        source: source(
            surface,
            AssistSourceFamily::Snippet,
            None,
            "SQL snippet pack",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::None,
            Vec::new(),
            "SELECT scaffold snippet; snippet packs stay available under provider fallback.",
        ),
        active_placeholder_index: Some(1),
        placeholder_count: 3,
        selection_count: 1,
        multi_cursor_compatible: false,
        tab_behavior_class: SnippetTabBehaviorClass::TraversePlaceholdersWhileActive,
        ime_posture_class: SnippetImePostureClass::NoComposition,
        cursor_posture_class: SnippetCursorPostureClass::Stable,
        primary_caret_ref: None,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: None,
    });
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::SourceLabeledFallback,
        degrade_label: "Source-labeled fallback — no live connection",
        signature_card: Some(card),
        snippet_strip: Some(strip),
    })
}

fn build_docs_code_block_snapshot() -> SignatureSnippetSnapshot {
    let surface = EditorSurfaceClass::DocsCodeBlock;
    let card = SignatureCard::new(SignatureCardInit {
        surface,
        card_id: "docs-signature".to_owned(),
        state_class: SignatureHelpStateClass::StalePendingRefresh,
        source: source(
            surface,
            AssistSourceFamily::FallbackLexical,
            None,
            "Detected-language best effort",
            RouterSupportClass::FallbackOnly,
            RouterFreshnessClass::Stale,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::PartialForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::DegradedScopeNarrowed,
            vec![ScopeLimitClass::SingleFileOnly],
            "Best-effort signature for a fenced block while a refresh is pending.",
        ),
        active_signature_index: 1,
        signature_count: 1,
        active_parameter_index: 1,
        parameter_count: 2,
        placement_class: SignaturePlacementClass::NonBlockingNearCaret,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: Some(AssistBlockReason::StaleAwaitingRefresh),
    });
    let strip = SnippetStrip::new(SnippetStripInit {
        surface,
        strip_id: "docs-snippet".to_owned(),
        state_class: SnippetSessionStateClass::Active,
        source: source(
            surface,
            AssistSourceFamily::Snippet,
            None,
            "Docs snippet pack",
            RouterSupportClass::FallbackOnly,
            RouterFreshnessClass::WarmCached,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::PartialForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::None,
            vec![ScopeLimitClass::SingleFileOnly],
            "Fenced-block snippet by detected language; edits stay inside the block.",
        ),
        active_placeholder_index: Some(1),
        placeholder_count: 2,
        selection_count: 1,
        multi_cursor_compatible: false,
        tab_behavior_class: SnippetTabBehaviorClass::TraversePlaceholdersWhileActive,
        ime_posture_class: SnippetImePostureClass::NoComposition,
        cursor_posture_class: SnippetCursorPostureClass::Stable,
        primary_caret_ref: None,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: None,
    });
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::SourceLabeledFallback,
        degrade_label: "Source-labeled fallback — best effort by detected language",
        signature_card: Some(card),
        snippet_strip: Some(strip),
    })
}

fn build_generated_file_snapshot() -> SignatureSnippetSnapshot {
    let surface = EditorSurfaceClass::GeneratedFile;
    let card = SignatureCard::new(SignatureCardInit {
        surface,
        card_id: "generated-signature".to_owned(),
        state_class: SignatureHelpStateClass::VisibleSingle,
        source: source(
            surface,
            AssistSourceFamily::FrameworkPack,
            Some("generated-source-bridge"),
            "Generated-source bridge",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalSidecar,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Signature is read-only; edits route through the generator.",
        ),
        active_signature_index: 1,
        signature_count: 1,
        active_parameter_index: 1,
        parameter_count: 2,
        placement_class: SignaturePlacementClass::NonBlockingNearCaret,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: Some(AssistBlockReason::RestrictedReadOnly),
    });
    let strip = SnippetStrip::new(SnippetStripInit {
        surface,
        strip_id: "generated-snippet".to_owned(),
        state_class: SnippetSessionStateClass::Active,
        source: source(
            surface,
            AssistSourceFamily::FrameworkPack,
            Some("generated-source-bridge"),
            "Generated-source bridge",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalSidecar,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Snippet previews generated scaffolding; apply routes through the generator.",
        ),
        active_placeholder_index: Some(1),
        placeholder_count: 2,
        selection_count: 1,
        multi_cursor_compatible: false,
        tab_behavior_class: SnippetTabBehaviorClass::TraversePlaceholdersWhileActive,
        ime_posture_class: SnippetImePostureClass::NoComposition,
        cursor_posture_class: SnippetCursorPostureClass::Stable,
        primary_caret_ref: None,
        accept_side_effect: AcceptSideEffectClass::AddsGeneratedScaffolding,
        side_effect_summary: Some(
            "Regenerates the file via its generator; preview the generated output first."
                .to_owned(),
        ),
        blocked_reason: Some(AssistBlockReason::RestrictedReadOnly),
    });
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::ReadOnlyNoApply,
        degrade_label: "Read-only — apply routes through the generator",
        signature_card: Some(card),
        snippet_strip: Some(strip),
    })
}

fn build_protected_file_snapshot() -> SignatureSnippetSnapshot {
    let surface = EditorSurfaceClass::ProtectedFile;
    let card = SignatureCard::new(SignatureCardInit {
        surface,
        card_id: "protected-signature".to_owned(),
        state_class: SignatureHelpStateClass::VisibleSingle,
        source: source(
            surface,
            AssistSourceFamily::FrameworkPack,
            Some("schema-pack:policy"),
            "Policy schema pack",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::WholeWorkspace,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalSidecar,
            RouterDegradedStateClass::DegradedPolicyNarrowed,
            Vec::new(),
            "Signature is read-only; writes require staged review.",
        ),
        active_signature_index: 1,
        signature_count: 1,
        active_parameter_index: 1,
        parameter_count: 2,
        placement_class: SignaturePlacementClass::NonBlockingNearCaret,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: Some(AssistBlockReason::RestrictedReadOnly),
    });
    let strip = SnippetStrip::new(SnippetStripInit {
        surface,
        strip_id: "protected-snippet".to_owned(),
        state_class: SnippetSessionStateClass::Active,
        source: source(
            surface,
            AssistSourceFamily::Snippet,
            None,
            "Policy snippet pack",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::DegradedPolicyNarrowed,
            vec![ScopeLimitClass::SingleFileOnly],
            "Policy-block snippet; accepting edits protected config behind staged review.",
        ),
        active_placeholder_index: Some(1),
        placeholder_count: 2,
        selection_count: 1,
        multi_cursor_compatible: false,
        tab_behavior_class: SnippetTabBehaviorClass::TraversePlaceholdersWhileActive,
        ime_posture_class: SnippetImePostureClass::NoComposition,
        cursor_posture_class: SnippetCursorPostureClass::Stable,
        primary_caret_ref: None,
        accept_side_effect: AcceptSideEffectClass::AddsConfigEdit,
        side_effect_summary: Some(
            "Edits a protected config block; the change enters staged review before commit."
                .to_owned(),
        ),
        blocked_reason: Some(AssistBlockReason::RestrictedReadOnly),
    });
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::ReadOnlyNoApply,
        degrade_label: "Read-only — writes require staged review",
        signature_card: Some(card),
        snippet_strip: Some(strip),
    })
}

fn build_partial_index_snapshot() -> SignatureSnippetSnapshot {
    let surface = EditorSurfaceClass::PartialIndexState;
    let card = SignatureCard::new(SignatureCardInit {
        surface,
        card_id: "partial-index-signature".to_owned(),
        state_class: SignatureHelpStateClass::StalePendingRefresh,
        source: source(
            surface,
            AssistSourceFamily::LanguageServer,
            Some("rust-analyzer"),
            "rust-analyzer (indexing)",
            RouterSupportClass::Advisory,
            RouterFreshnessClass::Unverified,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::PartialForClaimedScope,
            RouterLocalityClass::LocalSidecar,
            RouterDegradedStateClass::DegradedScopeNarrowed,
            vec![ScopeLimitClass::SingleFileOnly],
            "Index still building; previous signature shown while it refreshes.",
        ),
        active_signature_index: 1,
        signature_count: 1,
        active_parameter_index: 1,
        parameter_count: 2,
        placement_class: SignaturePlacementClass::NonBlockingNearCaret,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: Some(AssistBlockReason::PartialIndexPending),
    });
    let strip = SnippetStrip::new(SnippetStripInit {
        surface,
        strip_id: "partial-index-snippet".to_owned(),
        state_class: SnippetSessionStateClass::Active,
        source: source(
            surface,
            AssistSourceFamily::Snippet,
            None,
            "Snippet pack",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Snippet packs stay fully available while the semantic index builds.",
        ),
        active_placeholder_index: Some(1),
        placeholder_count: 2,
        selection_count: 1,
        multi_cursor_compatible: false,
        tab_behavior_class: SnippetTabBehaviorClass::TraversePlaceholdersWhileActive,
        ime_posture_class: SnippetImePostureClass::NoComposition,
        cursor_posture_class: SnippetCursorPostureClass::Stable,
        primary_caret_ref: None,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: None,
    });
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::PendingPartialIndex,
        degrade_label: "Pending — index still building",
        signature_card: Some(card),
        snippet_strip: Some(strip),
    })
}

fn build_large_file_snapshot() -> SignatureSnippetSnapshot {
    let surface = EditorSurfaceClass::LargeFileRestricted;
    let card = SignatureCard::new(SignatureCardInit {
        surface,
        card_id: "large-file-signature".to_owned(),
        state_class: SignatureHelpStateClass::Unavailable,
        source: source(
            surface,
            AssistSourceFamily::FallbackLexical,
            None,
            "Large-file mode",
            RouterSupportClass::Unsupported,
            RouterFreshnessClass::Unverified,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::UnavailableForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::DegradedScopeNarrowed,
            vec![ScopeLimitClass::SingleFileOnly],
            "Signature help is suppressed in large-file / restricted mode.",
        ),
        active_signature_index: 0,
        signature_count: 0,
        active_parameter_index: 0,
        parameter_count: 0,
        placement_class: SignaturePlacementClass::HiddenUnavailable,
        accept_side_effect: AcceptSideEffectClass::EditsTargetRangeOnly,
        side_effect_summary: None,
        blocked_reason: Some(AssistBlockReason::LargeFileSuppressed),
    });
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::SuppressedLargeFile,
        degrade_label: "Suppressed — large-file mode",
        signature_card: Some(card),
        snippet_strip: None,
    })
}

// ---------------------------------------------------------------------------
// Invariant evaluation.
// ---------------------------------------------------------------------------

fn evaluate_invariants(snapshots: &[SignatureSnippetSnapshot]) -> Vec<SignatureSnippetInvariant> {
    let cards: Vec<&SignatureCard> = snapshots
        .iter()
        .filter_map(|snapshot| snapshot.signature_card.as_ref())
        .collect();
    let strips: Vec<&SnippetStrip> = snapshots
        .iter()
        .filter_map(|snapshot| snapshot.snippet_strip.as_ref())
        .collect();

    let mut invariants = Vec::new();

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "every_surface_has_card_or_strip".into(),
        statement: "Each claimed editor family resolves at least a signature card or a snippet \
                    strip."
            .into(),
        holds: !snapshots.is_empty()
            && snapshots
                .iter()
                .all(|s| s.signature_card.is_some() || s.snippet_strip.is_some()),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "signature_never_obscures_active_line".into(),
        statement: "No signature card overlaps the active editor line.".into(),
        holds: cards.iter().all(|card| !card.obscures_active_line),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "visible_signature_is_typing_loop_safe".into(),
        statement: "Every visible signature card stays non-blocking and IME-safe during typing."
            .into(),
        holds: cards
            .iter()
            .filter(|card| card.is_visible())
            .all(|card| card.is_typing_loop_safe()),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "visible_signature_exposes_active_parameter".into(),
        statement:
            "Every visible signature card exposes its active parameter within the parameter \
                    count."
                .into(),
        holds: cards
            .iter()
            .filter(|card| card.is_visible())
            .all(|card| card.exposes_active_parameter()),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "overloaded_signature_exposes_active_overload".into(),
        statement: "Every overloaded signature card exposes its active overload within the \
                    signature count."
            .into(),
        holds: cards
            .iter()
            .filter(|card| matches!(card.state_class, SignatureHelpStateClass::VisibleOverloaded))
            .all(|card| card.exposes_active_overload()),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "stale_signature_discloses_limited_cue".into(),
        statement: "Every stale signature card discloses a limited / refresh-pending cue with a \
                    non-color differentiator."
            .into(),
        holds: cards
            .iter()
            .filter(|card| {
                matches!(
                    card.state_class,
                    SignatureHelpStateClass::StalePendingRefresh
                )
            })
            .all(|card| card.stale_disclosed && !card.non_color_differentiator.trim().is_empty()),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "snippet_never_hijacks_tab_invisibly".into(),
        statement: "Every snippet strip that captures Tab keeps a visible strip and discloses the \
                    capture."
            .into(),
        holds: strips.iter().all(|strip| strip.does_not_hijack_tab()),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "active_snippet_exposes_exit_path".into(),
        statement: "Every active snippet strip exposes a visible exit path and a coherent active \
                    placeholder index within the placeholder count."
            .into(),
        holds: strips.iter().all(|strip| strip.exposes_exit_path()),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "ime_multicursor_coherent_or_degraded".into(),
        statement: "Every snippet strip stays coherent for the whole selection set or degrades to \
                    one disclosed composition target."
            .into(),
        holds: strips
            .iter()
            .all(|strip| strip.ime_and_multicursor_coherent_or_degraded()),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "accept_side_effects_disclose_before_commit".into(),
        statement:
            "Every card or strip whose accept adds an edit beyond the target range discloses \
                    it before commit with a summary or preview."
                .into(),
        holds: cards
            .iter()
            .map(|card| {
                (
                    card.accept_side_effect,
                    card.commit_disclosure_required,
                    card.side_effect_summary.is_some(),
                    card.preview_required,
                )
            })
            .chain(strips.iter().map(|strip| {
                (
                    strip.accept_side_effect,
                    strip.commit_disclosure_required,
                    strip.side_effect_summary.is_some(),
                    strip.preview_required,
                )
            }))
            .filter(|(effect, _, _, _)| effect.requires_pre_commit_disclosure())
            .all(|(_, disclose, has_summary, preview)| disclose && (has_summary || preview)),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "generated_and_dependency_effects_require_preview".into(),
        statement: "Every accept that changes generated output or a dependency requires preview."
            .into(),
        holds: cards
            .iter()
            .map(|card| (card.accept_side_effect, card.preview_required))
            .chain(
                strips
                    .iter()
                    .map(|strip| (strip.accept_side_effect, strip.preview_required)),
            )
            .filter(|(effect, _)| {
                matches!(
                    effect,
                    AcceptSideEffectClass::AddsGeneratedScaffolding
                        | AcceptSideEffectClass::AddsDependency
                )
            })
            .all(|(_, preview)| preview),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "blocked_items_carry_reason_and_disclose".into(),
        statement: "Every snapshot containing a blocked card or strip flags disclosure.".into(),
        holds: snapshots.iter().all(|s| {
            let blocked = s
                .signature_card
                .as_ref()
                .is_some_and(|card| card.blocked_reason.is_some())
                || s.snippet_strip
                    .as_ref()
                    .is_some_and(|strip| strip.blocked_reason.is_some());
            !blocked || s.disclosure_required
        }),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "degraded_surfaces_label_and_disclose".into(),
        statement: "Every surface that is not full fidelity carries a visible degrade label and \
                    flags disclosure."
            .into(),
        holds: snapshots
            .iter()
            .filter(|s| s.degrade_class != AssistDegradeClass::FullFidelity)
            .all(|s| !s.degrade_label.trim().is_empty() && s.disclosure_required),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "every_card_and_strip_source_labeled".into(),
        statement: "Every card and strip carries a non-empty source label so provider / source is \
                    visible."
            .into(),
        holds: cards
            .iter()
            .all(|card| !card.source.source_label.trim().is_empty())
            && strips
                .iter()
                .all(|strip| !strip.source.source_label.trim().is_empty()),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "every_card_and_strip_keyboard_reachable".into(),
        statement: "Every card and strip is fully keyboard reachable.".into(),
        holds: cards.iter().all(|card| card.keyboard_reachable)
            && strips.iter().all(|strip| strip.keyboard_reachable),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "every_card_and_strip_screen_reader_meaningful".into(),
        statement: "Every card and strip carries a non-empty screen-reader label.".into(),
        holds: cards
            .iter()
            .all(|card| !card.accessibility_label.trim().is_empty())
            && strips
                .iter()
                .all(|strip| !strip.accessibility_label.trim().is_empty()),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "cards_mirror_canonical_record".into(),
        statement: "Each signature card's canonical record mirrors its overload, parameter, \
                    placement, and source."
            .into(),
        holds: cards.iter().all(|card| {
            let record = &card.canonical_record;
            record.active_signature_index == card.active_signature_index
                && record.signature_count == card.signature_count
                && record.active_parameter_index == card.active_parameter_index
                && record.parameter_count == card.parameter_count
                && record.placement_class == card.placement_class
                && record.source == card.source
        }),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "strips_mirror_canonical_record".into(),
        statement: "Each snippet strip's canonical record mirrors its state, placeholder, IME / \
                    cursor posture, and source."
            .into(),
        holds: strips.iter().all(|strip| {
            let record = &strip.canonical_record;
            record.state_class == strip.state_class
                && record.active_placeholder_index == strip.active_placeholder_index
                && record.placeholder_count == strip.placeholder_count
                && record.selection_count == strip.selection_count
                && record.tab_behavior_class == strip.tab_behavior_class
                && record.ime_posture_class == strip.ime_posture_class
                && record.cursor_posture_class == strip.cursor_posture_class
                && record.source == strip.source
        }),
    });

    invariants.push(SignatureSnippetInvariant {
        invariant_id: "first_consumers_prove_shared_model".into(),
        statement: "The notebook, request, SQL, and docs-code surfaces each prove the shared \
                    session model by resolving a card or strip."
            .into(),
        holds: [
            EditorSurfaceClass::NotebookCell,
            EditorSurfaceClass::RequestEditor,
            EditorSurfaceClass::SqlEditor,
            EditorSurfaceClass::DocsCodeBlock,
        ]
        .iter()
        .all(|surface| {
            snapshots
                .iter()
                .find(|s| s.surface_class == *surface)
                .is_some_and(|s| s.signature_card.is_some() || s.snippet_strip.is_some())
        }),
    });

    invariants
}

#[cfg(test)]
mod tests;
