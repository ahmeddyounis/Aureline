//! Dictated-text editor bridge: route speech into the shared edit model.
//!
//! Voice is an explicit, privacy-bounded input mode. A dictated word, a spoken
//! punctuation mark, a formatting intent, or a correction gesture is only ever
//! *recognized* — once recognized it must reach the **same** editor transactions
//! and grouped undo model that typing, paste, and structural edits use, never a
//! hidden speech-only buffer and never a side path that bypasses the buffer
//! journal.
//!
//! This module is that bridge. [`DictationCaptureSession`] drives one capture
//! against a [`Buffer`](aureline_buffer::Buffer) and the view-local
//! [`SelectionState`](crate::selection::SelectionState):
//!
//! - every finalized [`DictationIntent`] is lowered to a concrete edit and
//!   applied through [`SelectionState::apply_insert_text`] /
//!   [`SelectionState::apply_delete_backward`] or the shared undo/redo stack, so
//!   dictated edits land in ordinary [`UndoClass::TextEdit`](aureline_buffer::UndoClass)
//!   history groups and undo/redo predictably;
//! - an in-flight interim hypothesis is display-only ([`InterimDictation`]) — it
//!   never writes to the buffer — and cancelling a capture discards it and
//!   restores the prior insertion point;
//! - claimed text-entry surfaces beyond the main editor are covered explicitly
//!   through [`DictationSurface`] support classes, so an unsupported surface
//!   surfaces an explicit error instead of half-working.
//!
//! The capture also projects its committed edits onto the shared history
//! vocabulary through [`aureline_history::voice_groups`], proving grouped-history
//! parity. [`DictationEditParityPacket`] is the inspectable truth packet built
//! from a deterministic seed; it carries only typed class tokens, opaque ids,
//! byte counts, and redaction-aware label refs — never raw audio bytes, raw
//! transcript text, or raw provider payloads.

use aureline_buffer::{Buffer, BufferError, CompensationPosture, Snapshot};
use aureline_history::voice_groups::{
    VoiceHistoryGroupInput, VoiceHistoryGroupMember, VoiceHistoryGroupRecord,
    DICTATION_CAPTURE_COMMAND_ID,
};
use serde::{Deserialize, Serialize};

use crate::selection::{SelectionState, TextEditScope};
use crate::viewport::TextPoint;

pub use aureline_history::voice_groups::{DictationIntentClass, DictationRecognitionLocality};

/// Schema version stamped on every dictation-edit-parity record.
pub const DICTATION_EDIT_PARITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`DictationEditParityPacket`].
pub const DICTATION_EDIT_PARITY_PACKET_RECORD_KIND: &str = "dictation_edit_parity_packet_record";

/// Stable packet id quoted across surfaces.
pub const DICTATION_EDIT_PARITY_PACKET_ID: &str = "editor:dictation_edit_parity:packet:v1";

/// Repo-relative path of the published companion doc.
pub const DICTATION_EDIT_PARITY_DOC_REF: &str = "docs/ux/dictation-edit-contract.md";

/// Repo-relative directory of the checked-in mint-from-truth fixtures.
pub const DICTATION_EDIT_PARITY_FIXTURES_DIR_REF: &str = "fixtures/voice/dictation-edit-parity";

/// Cross-surface voice / dictation / speech-privacy contract this bridge rides.
pub const VOICE_AND_DICTATION_CONTRACT_REF: &str = "docs/ux/voice_and_dictation_contract.md";

/// Redaction class stamped on every record; the packet carries metadata only.
pub const REDACTION_CLASS: &str = "metadata_safe_default";

/// Stable originator identifiers for dictation-owned undo groups.
///
/// Dictated edits ride the **same** [`UndoClass::TextEdit`](aureline_buffer::UndoClass)
/// as typing; the originator only attributes the lane for lineage and audit, it
/// never forks a separate buffer or undo path. These strings are part of the
/// history/lineage contract and should remain stable over time.
pub mod originator {
    /// Plain dictated words.
    pub const DICTATION_TEXT: &str = "voice:dictation.text";

    /// Spoken punctuation marks ("period", "comma", ...).
    pub const DICTATION_PUNCTUATION: &str = "voice:dictation.punctuation";

    /// Spoken formatting intents ("new line", "new paragraph", "tab").
    pub const DICTATION_FORMATTING: &str = "voice:dictation.formatting";

    /// Spoken correction gestures ("scratch that", "delete that", ...).
    pub const DICTATION_CORRECTION: &str = "voice:dictation.correction";
}

// ---------------------------------------------------------------------------
// Intent vocabulary.
// ---------------------------------------------------------------------------

/// A spoken punctuation mark, lowered to the literal text it inserts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PunctuationMark {
    /// `.`
    Period,
    /// `,`
    Comma,
    /// `?`
    QuestionMark,
    /// `!`
    ExclamationMark,
    /// `:`
    Colon,
    /// `;`
    Semicolon,
}

impl PunctuationMark {
    /// The literal text this mark inserts.
    pub const fn literal(self) -> &'static str {
        match self {
            Self::Period => ".",
            Self::Comma => ",",
            Self::QuestionMark => "?",
            Self::ExclamationMark => "!",
            Self::Colon => ":",
            Self::Semicolon => ";",
        }
    }
}

/// A spoken formatting intent, lowered to the literal whitespace it inserts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormattingIntent {
    /// A single line break.
    NewLine,
    /// A blank-line paragraph break.
    NewParagraph,
    /// A tab character.
    Tab,
}

impl FormattingIntent {
    /// The literal text this intent inserts.
    pub const fn literal(self) -> &'static str {
        match self {
            Self::NewLine => "\n",
            Self::NewParagraph => "\n\n",
            Self::Tab => "\t",
        }
    }
}

/// A spoken correction gesture.
///
/// Every gesture routes through the shared edit model or the shared undo/redo
/// stack — there is no bespoke voice correction buffer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "gesture", rename_all = "snake_case")]
pub enum CorrectionGesture {
    /// Undo the last dictated group ("scratch that").
    ScratchThat,
    /// Redo the last undone dictated group.
    RedoLast,
    /// Delete the active selection ("delete that").
    DeleteSelection,
    /// Replace the active selection with new text.
    ReplaceSelection {
        /// The replacement text.
        replacement: String,
    },
}

/// One finalized dictation intent ready to apply through the edit model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "intent", rename_all = "snake_case")]
pub enum DictationIntent {
    /// Plain dictated words inserted at the caret.
    InsertText {
        /// The recognized words.
        text: String,
    },
    /// A spoken punctuation mark.
    Punctuation(PunctuationMark),
    /// A spoken formatting intent.
    Formatting(FormattingIntent),
    /// A spoken correction gesture.
    Correction(CorrectionGesture),
}

impl DictationIntent {
    /// Coarse intent class shared with the history projection.
    pub fn class(&self) -> DictationIntentClass {
        match self {
            Self::InsertText { .. } => DictationIntentClass::Text,
            Self::Punctuation(_) => DictationIntentClass::Punctuation,
            Self::Formatting(_) => DictationIntentClass::Formatting,
            Self::Correction(_) => DictationIntentClass::Correction,
        }
    }

    /// Stable undo-group originator for the intent's lane.
    pub fn originator(&self) -> &'static str {
        match self {
            Self::InsertText { .. } => originator::DICTATION_TEXT,
            Self::Punctuation(_) => originator::DICTATION_PUNCTUATION,
            Self::Formatting(_) => originator::DICTATION_FORMATTING,
            Self::Correction(_) => originator::DICTATION_CORRECTION,
        }
    }

    /// `true` when the intent is plain dictated text (the only intent a
    /// text-only degraded surface accepts).
    pub fn is_plain_text(&self) -> bool {
        matches!(self, Self::InsertText { .. })
    }

    /// Lowers the intent to the concrete edit-model effect it produces.
    fn resolve(&self) -> ResolvedEffect {
        match self {
            Self::InsertText { text } => ResolvedEffect::Insert(text.clone()),
            Self::Punctuation(mark) => ResolvedEffect::Insert(mark.literal().to_owned()),
            Self::Formatting(intent) => ResolvedEffect::Insert(intent.literal().to_owned()),
            Self::Correction(CorrectionGesture::ScratchThat) => ResolvedEffect::Undo,
            Self::Correction(CorrectionGesture::RedoLast) => ResolvedEffect::Redo,
            Self::Correction(CorrectionGesture::DeleteSelection) => ResolvedEffect::DeleteSelection,
            Self::Correction(CorrectionGesture::ReplaceSelection { replacement }) => {
                ResolvedEffect::ReplaceSelection(replacement.clone())
            }
        }
    }
}

/// Internal lowering of an intent to a shared-edit-model effect.
enum ResolvedEffect {
    /// Insert text at the caret (replacing any active selection).
    Insert(String),
    /// Replace the active selection with text; a no-op without a selection.
    ReplaceSelection(String),
    /// Delete the active selection; a no-op without a selection.
    DeleteSelection,
    /// Undo the last committed group through the shared undo stack.
    Undo,
    /// Redo the last undone group through the shared redo stack.
    Redo,
}

/// The concrete effect an applied intent had on the buffer, as a stable token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DictationEffectClass {
    /// Text was inserted or a selection replaced.
    InsertText,
    /// Text was deleted.
    DeleteText,
    /// The last committed group was undone through the shared stack.
    UndoLastGroup,
    /// The last undone group was redone through the shared stack.
    RedoLastGroup,
    /// The intent resolved to no change.
    NoOp,
}

impl DictationEffectClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InsertText => "insert_text",
            Self::DeleteText => "delete_text",
            Self::UndoLastGroup => "undo_last_group",
            Self::RedoLastGroup => "redo_last_group",
            Self::NoOp => "no_op",
        }
    }

    /// `true` when the effect committed a content-bearing edit transaction (an
    /// insert or delete) that becomes a member of the capture's history group.
    pub const fn is_content_edit(self) -> bool {
        matches!(self, Self::InsertText | Self::DeleteText)
    }
}

// ---------------------------------------------------------------------------
// Surface coverage.
// ---------------------------------------------------------------------------

/// A focused text-entry surface dictation may target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DictationSurfaceClass {
    /// The main code editor viewport.
    MainEditor,
    /// A focused single-line text field (rename input, find field, ...).
    SingleLineTextField,
    /// A focused multi-line text area (commit message, comment, ...).
    MultiLineTextArea,
    /// An integrated terminal surface.
    Terminal,
    /// A notebook cell surface.
    NotebookCell,
    /// A custom extension-owned widget.
    CustomWidget,
}

impl DictationSurfaceClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MainEditor => "main_editor",
            Self::SingleLineTextField => "single_line_text_field",
            Self::MultiLineTextArea => "multi_line_text_area",
            Self::Terminal => "terminal",
            Self::NotebookCell => "notebook_cell",
            Self::CustomWidget => "custom_widget",
        }
    }
}

/// How honestly a surface supports dictation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DictationSurfaceSupport {
    /// Full dictation: text, punctuation, formatting, and corrections.
    Supported,
    /// Plain dictated text only; punctuation / formatting / correction intents
    /// are explicitly rejected rather than half-applied.
    DegradedTextOnly,
    /// Dictation is not wired here; every intent is explicitly rejected.
    Unsupported,
}

impl DictationSurfaceSupport {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::DegradedTextOnly => "degraded_text_only",
            Self::Unsupported => "unsupported",
        }
    }
}

/// A claimed text-entry surface and its dictation support posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DictationSurface {
    /// The surface class.
    pub surface_class: DictationSurfaceClass,
    /// Stable id of the concrete surface instance.
    pub surface_id: String,
    /// Honest support posture for the surface.
    pub support: DictationSurfaceSupport,
    /// Representation label explaining the support posture (never raw text).
    pub support_reason_ref: String,
    /// Accessibility label ref narrated by the screen reader for the surface.
    pub accessibility_label_ref: String,
}

// ---------------------------------------------------------------------------
// Errors.
// ---------------------------------------------------------------------------

/// A reason a dictation intent could not be applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DictationError {
    /// The capture has already ended or been cancelled.
    NotCapturing {
        /// Offending capture id.
        capture_id: String,
    },
    /// The surface does not support dictation; nothing was applied.
    SurfaceUnsupported {
        /// Offending surface id.
        surface_id: String,
    },
    /// The surface only supports plain text; a richer intent was rejected.
    IntentUnsupportedOnSurface {
        /// Offending surface id.
        surface_id: String,
        /// The rejected intent class.
        intent_class: DictationIntentClass,
    },
    /// A buffer transaction failed.
    Buffer(BufferError),
}

impl std::fmt::Display for DictationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotCapturing { capture_id } => {
                write!(f, "dictation capture {capture_id} is not active")
            }
            Self::SurfaceUnsupported { surface_id } => {
                write!(f, "surface {surface_id} does not support dictation")
            }
            Self::IntentUnsupportedOnSurface {
                surface_id,
                intent_class,
            } => write!(
                f,
                "surface {surface_id} does not support {} dictation intents",
                intent_class.as_str()
            ),
            Self::Buffer(err) => write!(f, "buffer edit failed: {err}"),
        }
    }
}

impl std::error::Error for DictationError {}

impl From<BufferError> for DictationError {
    fn from(err: BufferError) -> Self {
        Self::Buffer(err)
    }
}

// ---------------------------------------------------------------------------
// Per-edit record.
// ---------------------------------------------------------------------------

/// One applied dictation intent, as an inspectable, export-safe record.
///
/// The record carries the lineage proving the edit rode the shared model — its
/// originator, the frozen undo class the buffer reported, the undo-group id, and
/// caret movement — plus a representation label, never the raw spoken text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DictationEditRecord {
    /// Stable per-edit id (`<capture_id>:edit:<nn>`).
    pub edit_id: String,
    /// Coarse intent class.
    pub intent_class: DictationIntentClass,
    /// Stable undo-group originator the edit committed under.
    pub originator: String,
    /// Concrete effect the edit had.
    pub effect: DictationEffectClass,
    /// Frozen editor undo-class id the buffer reported (`text_edit`, ...).
    pub undo_class_id: String,
    /// Undo-group id the edit committed under or reversed, when one applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undo_group_id: Option<u64>,
    /// Bytes inserted by the edit.
    pub inserted_bytes: u64,
    /// Bytes removed by the edit.
    pub removed_bytes: u64,
    /// `true` when the committed group reverses cleanly through undo.
    pub reversible: bool,
    /// Primary caret position before the edit.
    pub caret_before: TextPoint,
    /// Primary caret position after the edit.
    pub caret_after: TextPoint,
    /// Representation label for the affected text (never raw spoken bytes).
    pub text_label_ref: String,
}

/// Outcome of applying one dictation intent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictationApplyOutcome {
    /// The recorded edit.
    pub record: DictationEditRecord,
    /// `true` when the buffer changed (a transaction committed or the shared
    /// undo/redo stack moved).
    pub mutated_buffer: bool,
}

// ---------------------------------------------------------------------------
// Capture session.
// ---------------------------------------------------------------------------

/// In-flight, display-only interim dictation hypothesis.
///
/// The interim is never written to the buffer — it mirrors IME preedit. It is
/// committed through the shared edit model on [`DictationCaptureSession::end`]
/// or discarded on [`DictationCaptureSession::cancel`], which also restores the
/// insertion point captured when the interim began.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterimDictation {
    text: String,
    insertion_point: SelectionState,
}

impl InterimDictation {
    /// The pending hypothesis text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Byte length of the pending hypothesis.
    pub fn len_bytes(&self) -> usize {
        self.text.len()
    }

    /// The insertion point a cancel restores to.
    pub fn insertion_point(&self) -> &SelectionState {
        &self.insertion_point
    }
}

/// Lifecycle status of a dictation capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureStatus {
    /// The capture is active and accepting intents.
    Capturing,
    /// The capture finalized; committed edits remain in history.
    Ended,
    /// The capture was cancelled; the in-flight interim was discarded.
    Cancelled,
}

impl CaptureStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Capturing => "capturing",
            Self::Ended => "ended",
            Self::Cancelled => "cancelled",
        }
    }
}

/// Drives one dictation capture against a buffer and view-local selection.
#[derive(Debug, Clone)]
pub struct DictationCaptureSession {
    capture_id: String,
    surface: DictationSurface,
    recognition_locality: DictationRecognitionLocality,
    interim: Option<InterimDictation>,
    members: Vec<VoiceHistoryGroupMember>,
    records: Vec<DictationEditRecord>,
    status: CaptureStatus,
}

impl DictationCaptureSession {
    /// Opens a capture for `surface` recognized at `recognition_locality`.
    pub fn begin(
        capture_id: impl Into<String>,
        surface: DictationSurface,
        recognition_locality: DictationRecognitionLocality,
    ) -> Self {
        Self {
            capture_id: capture_id.into(),
            surface,
            recognition_locality,
            interim: None,
            members: Vec::new(),
            records: Vec::new(),
            status: CaptureStatus::Capturing,
        }
    }

    /// The capture id.
    pub fn capture_id(&self) -> &str {
        &self.capture_id
    }

    /// The target surface.
    pub fn surface(&self) -> &DictationSurface {
        &self.surface
    }

    /// The capture status.
    pub fn status(&self) -> CaptureStatus {
        self.status
    }

    /// The recorded edits, in apply order.
    pub fn records(&self) -> &[DictationEditRecord] {
        &self.records
    }

    /// The in-flight interim hypothesis, if any.
    pub fn interim(&self) -> Option<&InterimDictation> {
        self.interim.as_ref()
    }

    /// Sets a display-only interim hypothesis and captures the insertion point a
    /// later cancel restores. This never writes to the buffer.
    pub fn set_interim(&mut self, text: impl Into<String>, selection: &SelectionState) {
        self.interim = Some(InterimDictation {
            text: text.into(),
            insertion_point: selection.clone(),
        });
    }

    /// Clears the interim hypothesis without restoring the insertion point.
    pub fn clear_interim(&mut self) {
        self.interim = None;
    }

    /// Applies one finalized intent through the shared edit model.
    ///
    /// Insert / punctuation / formatting / replace intents commit ordinary
    /// text-edit transactions; scratch / redo gestures move the shared
    /// undo/redo stack. Unsupported surfaces and intents are rejected
    /// explicitly so a claimed surface never half-works.
    pub fn apply(
        &mut self,
        buffer: &mut Buffer,
        selection: &mut SelectionState,
        intent: &DictationIntent,
    ) -> Result<DictationApplyOutcome, DictationError> {
        if self.status != CaptureStatus::Capturing {
            return Err(DictationError::NotCapturing {
                capture_id: self.capture_id.clone(),
            });
        }
        match self.surface.support {
            DictationSurfaceSupport::Unsupported => {
                return Err(DictationError::SurfaceUnsupported {
                    surface_id: self.surface.surface_id.clone(),
                });
            }
            DictationSurfaceSupport::DegradedTextOnly if !intent.is_plain_text() => {
                return Err(DictationError::IntentUnsupportedOnSurface {
                    surface_id: self.surface.surface_id.clone(),
                    intent_class: intent.class(),
                });
            }
            _ => {}
        }

        let class = intent.class();
        let originator = intent.originator();
        let caret_before = selection.primary_caret();

        let mut undo_class_id = "text_edit".to_owned();
        let mut undo_group_id: Option<u64> = None;
        let mut inserted_bytes = 0u64;
        let mut removed_bytes = 0u64;
        let mut reversible = false;
        let effect: DictationEffectClass;

        match intent.resolve() {
            ResolvedEffect::Insert(text) => {
                let snapshot = buffer.snapshot();
                match selection.apply_insert_text(
                    buffer,
                    &snapshot,
                    &text,
                    originator,
                    TextEditScope::PrimaryOnly,
                )? {
                    Some(outcome) => {
                        undo_class_id = outcome.committed.class_id.to_owned();
                        undo_group_id = Some(outcome.committed.undo_group_id.0);
                        inserted_bytes = outcome.committed.inserted_bytes as u64;
                        removed_bytes = outcome.committed.removed_bytes as u64;
                        reversible = is_reversible(outcome.committed.compensation_posture);
                        effect = DictationEffectClass::InsertText;
                    }
                    None => effect = DictationEffectClass::NoOp,
                }
            }
            ResolvedEffect::ReplaceSelection(text) => {
                if selection.ordered_selection_ranges().is_empty() {
                    effect = DictationEffectClass::NoOp;
                } else {
                    let snapshot = buffer.snapshot();
                    match selection.apply_insert_text(
                        buffer,
                        &snapshot,
                        &text,
                        originator,
                        TextEditScope::PrimaryOnly,
                    )? {
                        Some(outcome) => {
                            undo_class_id = outcome.committed.class_id.to_owned();
                            undo_group_id = Some(outcome.committed.undo_group_id.0);
                            inserted_bytes = outcome.committed.inserted_bytes as u64;
                            removed_bytes = outcome.committed.removed_bytes as u64;
                            reversible = is_reversible(outcome.committed.compensation_posture);
                            effect = DictationEffectClass::InsertText;
                        }
                        None => effect = DictationEffectClass::NoOp,
                    }
                }
            }
            ResolvedEffect::DeleteSelection => {
                if selection.ordered_selection_ranges().is_empty() {
                    effect = DictationEffectClass::NoOp;
                } else {
                    let snapshot = buffer.snapshot();
                    match selection.apply_delete_backward(
                        buffer,
                        &snapshot,
                        originator,
                        TextEditScope::PrimaryOnly,
                    )? {
                        Some(outcome) => {
                            undo_class_id = outcome.committed.class_id.to_owned();
                            undo_group_id = Some(outcome.committed.undo_group_id.0);
                            inserted_bytes = outcome.committed.inserted_bytes as u64;
                            removed_bytes = outcome.committed.removed_bytes as u64;
                            reversible = is_reversible(outcome.committed.compensation_posture);
                            effect = DictationEffectClass::DeleteText;
                        }
                        None => effect = DictationEffectClass::NoOp,
                    }
                }
            }
            ResolvedEffect::Undo => match buffer.undo() {
                Some(outcome) => {
                    undo_class_id = outcome.class_id.to_owned();
                    undo_group_id = Some(outcome.undo_group_id.0);
                    reversible = is_reversible(outcome.compensation_posture);
                    clamp_selection(buffer, selection);
                    effect = DictationEffectClass::UndoLastGroup;
                }
                None => effect = DictationEffectClass::NoOp,
            },
            ResolvedEffect::Redo => match buffer.redo() {
                Some(outcome) => {
                    undo_class_id = outcome.class_id.to_owned();
                    undo_group_id = Some(outcome.undo_group_id.0);
                    reversible = is_reversible(outcome.compensation_posture);
                    clamp_selection(buffer, selection);
                    effect = DictationEffectClass::RedoLastGroup;
                }
                None => effect = DictationEffectClass::NoOp,
            },
        }

        let caret_after = selection.primary_caret();
        let record = DictationEditRecord {
            edit_id: format!("{}:edit:{:02}", self.capture_id, self.records.len()),
            intent_class: class,
            originator: originator.to_owned(),
            effect,
            undo_class_id,
            undo_group_id,
            inserted_bytes,
            removed_bytes,
            reversible,
            caret_before,
            caret_after,
            text_label_ref: format!("label:dictation:{}", class.as_str()),
        };

        if effect.is_content_edit() {
            self.members.push(VoiceHistoryGroupMember::new(
                record.edit_id.clone(),
                class,
                record.undo_class_id.clone(),
                record.reversible,
                record.inserted_bytes,
                record.removed_bytes,
            ));
        }
        self.records.push(record.clone());

        Ok(DictationApplyOutcome {
            record,
            mutated_buffer: effect != DictationEffectClass::NoOp,
        })
    }

    /// Ends the capture, finalizing any in-flight interim as an ordinary edit
    /// and keeping committed text. The caret stays at the live insertion point.
    pub fn end(
        &mut self,
        buffer: &mut Buffer,
        selection: &mut SelectionState,
    ) -> Result<DictationCaptureSummary, DictationError> {
        if self.status != CaptureStatus::Capturing {
            return Err(DictationError::NotCapturing {
                capture_id: self.capture_id.clone(),
            });
        }
        if let Some(interim) = self.interim.take() {
            let intent = DictationIntent::InsertText { text: interim.text };
            self.apply(buffer, selection, &intent)?;
        }
        self.status = CaptureStatus::Ended;
        Ok(self.summary())
    }

    /// Cancels the capture, discarding any in-flight interim and restoring the
    /// prior insertion point. Committed edits remain undoable in shared history.
    pub fn cancel(&mut self, selection: &mut SelectionState) -> DictationCaptureSummary {
        if let Some(interim) = self.interim.take() {
            *selection = interim.insertion_point;
        }
        self.status = CaptureStatus::Cancelled;
        self.summary()
    }

    /// Projects the capture's committed content edits onto the shared history
    /// vocabulary. Returns `None` when nothing content-bearing committed.
    pub fn history_group(&self) -> Option<VoiceHistoryGroupRecord> {
        if self.members.is_empty() {
            return None;
        }
        Some(VoiceHistoryGroupRecord::from_input(
            VoiceHistoryGroupInput {
                group_id: self.capture_id.clone(),
                surface_id: self.surface.surface_id.clone(),
                command_id: DICTATION_CAPTURE_COMMAND_ID.to_owned(),
                recognition_locality: self.recognition_locality,
                members: self.members.clone(),
            },
        ))
    }

    fn summary(&self) -> DictationCaptureSummary {
        DictationCaptureSummary {
            capture_id: self.capture_id.clone(),
            surface_id: self.surface.surface_id.clone(),
            status: self.status,
            content_edits: self.members.len() as u32,
            history_group: self.history_group(),
        }
    }
}

/// Result of ending or cancelling a capture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictationCaptureSummary {
    /// The capture id.
    pub capture_id: String,
    /// The target surface id.
    pub surface_id: String,
    /// Final lifecycle status.
    pub status: CaptureStatus,
    /// Number of content-bearing edits committed.
    pub content_edits: u32,
    /// The capture's history group, when content committed.
    pub history_group: Option<VoiceHistoryGroupRecord>,
}

fn is_reversible(posture: CompensationPosture) -> bool {
    matches!(posture, CompensationPosture::Compensatable)
}

fn line_grapheme_counts(snapshot: &Snapshot) -> Vec<usize> {
    (0..snapshot.line_count())
        .map(|line| snapshot.grapheme_count_in_line(line).unwrap_or(0))
        .collect()
}

fn clamp_selection(buffer: &mut Buffer, selection: &mut SelectionState) {
    let snapshot = buffer.snapshot();
    let counts = line_grapheme_counts(&snapshot);
    selection.clamp_to_document(&counts);
}

// ---------------------------------------------------------------------------
// Truth packet.
// ---------------------------------------------------------------------------

/// One claimed text-entry surface row in the parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DictationSurfaceCoverageRow {
    /// Surface class.
    pub surface_class: DictationSurfaceClass,
    /// Stable surface id.
    pub surface_id: String,
    /// Honest support posture.
    pub support: DictationSurfaceSupport,
    /// Representation label explaining the posture.
    pub support_reason_ref: String,
    /// Accessibility label ref narrated for the surface.
    pub accessibility_label_ref: String,
}

/// Predictable-undo/redo proof captured by replaying a scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoRedoRoundtrip {
    /// Content-bearing edits committed during the scenario.
    pub content_edits: u32,
    /// `true` when draining undo returns the buffer to its seed text.
    pub returns_to_seed_after_full_undo: bool,
    /// `true` when draining redo returns the buffer to its final text.
    pub returns_to_final_after_full_redo: bool,
    /// `true` when every committed group rode an ordinary text-edit class.
    pub all_groups_text_edit_class: bool,
}

/// How a parity scenario resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DictationScenarioOutcomeClass {
    /// Dictation applied and committed through the shared edit model.
    Applied,
    /// A capture was cancelled and the insertion point restored.
    CancelledRestored,
    /// Dictation was explicitly rejected on an unsupported surface.
    RejectedUnsupported,
    /// A richer intent was explicitly rejected on a text-only surface.
    RejectedDegraded,
}

impl DictationScenarioOutcomeClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::CancelledRestored => "cancelled_restored",
            Self::RejectedUnsupported => "rejected_unsupported",
            Self::RejectedDegraded => "rejected_degraded",
        }
    }
}

/// One replayed parity scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DictationParityScenario {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Surface id the scenario ran against.
    pub surface_id: String,
    /// Surface class the scenario ran against.
    pub surface_class: DictationSurfaceClass,
    /// Support posture of the surface.
    pub support: DictationSurfaceSupport,
    /// How the scenario resolved.
    pub outcome_class: DictationScenarioOutcomeClass,
    /// Recorded edits, in apply order.
    pub edits: Vec<DictationEditRecord>,
    /// Grouped-history projection, when content committed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_group: Option<VoiceHistoryGroupRecord>,
    /// Undo/redo round-trip proof, when content committed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undo_redo: Option<UndoRedoRoundtrip>,
    /// Whether a cancelled capture restored the insertion point.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_point_preserved: Option<bool>,
    /// `true` when an unsupported / degraded intent was explicitly rejected.
    pub explicit_rejection: bool,
    /// Representation label describing the scenario.
    pub note_ref: String,
}

/// Cross-scenario invariant manifest. Every field is `true` exactly when the
/// packet's scenarios uphold the dictation-edit contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DictationParityInvariantManifest {
    /// Every committed / reversed edit rode an ordinary text-edit undo class.
    pub every_edit_uses_ordinary_text_edit_class: bool,
    /// Every content edit committed a real buffer transaction (shared model).
    pub every_edit_routes_through_shared_edit_model: bool,
    /// Undo and redo round-trip predictably in every applied scenario.
    pub undo_redo_round_trips_predictably: bool,
    /// Cancelling a capture restores the prior insertion point.
    pub capture_cancel_restores_insertion_point: bool,
    /// Unsupported / degraded surfaces reject explicitly, never half-work.
    pub unsupported_surfaces_are_explicit: bool,
    /// Every projected history group is parity-clean.
    pub history_groups_are_parity_clean: bool,
    /// No capture created a hidden speech-only buffer.
    pub no_hidden_speech_buffer: bool,
}

impl DictationParityInvariantManifest {
    /// The all-satisfied manifest.
    pub const fn all_true() -> Self {
        Self {
            every_edit_uses_ordinary_text_edit_class: true,
            every_edit_routes_through_shared_edit_model: true,
            undo_redo_round_trips_predictably: true,
            capture_cancel_restores_insertion_point: true,
            unsupported_surfaces_are_explicit: true,
            history_groups_are_parity_clean: true,
            no_hidden_speech_buffer: true,
        }
    }

    /// Recomputes the manifest from the packet's surfaces and scenarios.
    pub fn from_parts(
        surfaces: &[DictationSurfaceCoverageRow],
        scenarios: &[DictationParityScenario],
    ) -> Self {
        let mut manifest = Self::all_true();

        for surface in surfaces {
            if surface.support != DictationSurfaceSupport::Supported
                && surface.support_reason_ref.trim().is_empty()
            {
                manifest.unsupported_surfaces_are_explicit = false;
            }
        }

        for scenario in scenarios {
            for edit in &scenario.edits {
                if edit.undo_group_id.is_some()
                    && !aureline_history::voice_groups::ORDINARY_TEXT_EDIT_UNDO_CLASS_IDS
                        .contains(&edit.undo_class_id.as_str())
                {
                    manifest.every_edit_uses_ordinary_text_edit_class = false;
                }
                if edit.effect.is_content_edit() && edit.undo_group_id.is_none() {
                    manifest.every_edit_routes_through_shared_edit_model = false;
                    manifest.no_hidden_speech_buffer = false;
                }
            }

            if let Some(roundtrip) = scenario.undo_redo {
                if !roundtrip.returns_to_seed_after_full_undo
                    || !roundtrip.returns_to_final_after_full_redo
                    || !roundtrip.all_groups_text_edit_class
                {
                    manifest.undo_redo_round_trips_predictably = false;
                }
            }

            if let Some(restored) = scenario.restore_point_preserved {
                if !restored {
                    manifest.capture_cancel_restores_insertion_point = false;
                }
            }

            match scenario.outcome_class {
                // An unsupported surface must reject explicitly and apply nothing.
                DictationScenarioOutcomeClass::RejectedUnsupported => {
                    if !scenario.explicit_rejection || !scenario.edits.is_empty() {
                        manifest.unsupported_surfaces_are_explicit = false;
                    }
                }
                // A degraded surface may apply plain text but must reject the
                // richer intent explicitly rather than half-applying it.
                DictationScenarioOutcomeClass::RejectedDegraded => {
                    if !scenario.explicit_rejection {
                        manifest.unsupported_surfaces_are_explicit = false;
                    }
                }
                _ => {}
            }

            if let Some(group) = &scenario.history_group {
                if !group.is_well_formed() {
                    manifest.history_groups_are_parity_clean = false;
                }
            }
        }

        manifest
    }

    /// `true` when every invariant holds.
    pub fn all_satisfied(&self) -> bool {
        *self == Self::all_true()
    }
}

/// Inspectable truth packet for the dictation-edit-parity lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DictationEditParityPacket {
    /// Record discriminator; equals [`DICTATION_EDIT_PARITY_PACKET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; equals [`DICTATION_EDIT_PARITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Ref to the published companion doc.
    pub doc_ref: String,
    /// Ref to the checked-in fixtures directory.
    pub fixtures_dir_ref: String,
    /// Ref to the cross-surface voice / dictation contract this bridge rides.
    pub voice_and_dictation_contract_ref: String,
    /// Claimed text-entry surfaces and their support postures.
    pub surfaces: Vec<DictationSurfaceCoverageRow>,
    /// Replayed parity scenarios, in canonical order.
    pub scenarios: Vec<DictationParityScenario>,
    /// Cross-scenario invariant manifest.
    pub invariants: DictationParityInvariantManifest,
    /// `true` — dictated edits route through the shared editor edit model.
    pub routes_through_shared_edit_model: bool,
    /// `true` — no capture creates a hidden speech-only buffer.
    pub no_hidden_speech_buffer: bool,
    /// `true` — no raw audio / transcript bytes ever cross this boundary.
    pub raw_audio_or_transcript_bytes_excluded: bool,
    /// Redaction class stamped on the packet.
    pub redaction_class: String,
}

impl DictationEditParityPacket {
    /// Builds a packet from `surfaces` and `scenarios`, stamping the canonical
    /// envelope and recomputing the invariant manifest.
    pub fn new(
        surfaces: Vec<DictationSurfaceCoverageRow>,
        scenarios: Vec<DictationParityScenario>,
    ) -> Self {
        let invariants = DictationParityInvariantManifest::from_parts(&surfaces, &scenarios);
        Self {
            record_kind: DICTATION_EDIT_PARITY_PACKET_RECORD_KIND.to_owned(),
            schema_version: DICTATION_EDIT_PARITY_SCHEMA_VERSION,
            packet_id: DICTATION_EDIT_PARITY_PACKET_ID.to_owned(),
            doc_ref: DICTATION_EDIT_PARITY_DOC_REF.to_owned(),
            fixtures_dir_ref: DICTATION_EDIT_PARITY_FIXTURES_DIR_REF.to_owned(),
            voice_and_dictation_contract_ref: VOICE_AND_DICTATION_CONTRACT_REF.to_owned(),
            surfaces,
            scenarios,
            invariants,
            routes_through_shared_edit_model: true,
            no_hidden_speech_buffer: true,
            raw_audio_or_transcript_bytes_excluded: true,
            redaction_class: REDACTION_CLASS.to_owned(),
        }
    }

    /// Returns the scenario with `scenario_id`, if present.
    pub fn scenario(&self, scenario_id: &str) -> Option<&DictationParityScenario> {
        self.scenarios.iter().find(|s| s.scenario_id == scenario_id)
    }

    /// Collects every invariant violation. An empty result means every dictated
    /// edit stays inside the shared edit model with predictable grouped undo and
    /// honest surface coverage.
    pub fn validate(&self) -> Vec<String> {
        let mut out = Vec::new();
        let recomputed =
            DictationParityInvariantManifest::from_parts(&self.surfaces, &self.scenarios);
        if recomputed != self.invariants {
            out.push("invariant manifest drifted from scenarios".to_owned());
        }
        if !recomputed.all_satisfied() {
            out.push("one or more dictation-edit invariants are unsatisfied".to_owned());
        }
        for scenario in &self.scenarios {
            if let Some(group) = &scenario.history_group {
                for violation in group.check() {
                    out.push(format!(
                        "{}: history group {}",
                        scenario.scenario_id,
                        violation.class_token()
                    ));
                }
            }
        }
        out
    }

    /// `true` when the packet validates.
    pub fn is_well_formed(&self) -> bool {
        self.validate().is_empty()
    }

    /// Support-safe compact lines, one per scenario, plus a header.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(self.scenarios.len() + 1);
        lines.push(format!(
            "{} | surfaces={} | scenarios={} | invariants_ok={}",
            self.packet_id,
            self.surfaces.len(),
            self.scenarios.len(),
            self.invariants.all_satisfied(),
        ));
        for scenario in &self.scenarios {
            lines.push(format!(
                "{} | surface={} | support={} | outcome={} | edits={}",
                scenario.scenario_id,
                scenario.surface_id,
                scenario.support.as_str(),
                scenario.outcome_class.as_str(),
                scenario.edits.len(),
            ));
        }
        lines
    }

    /// Renders the published Markdown companion summary.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Dictation edit contract\n\n");
        out.push_str(
            "Generated from the `voice_input` seed. Do not edit by hand; regenerate with \
             `cargo run -p aureline-editor --bin aureline_dictation_edit_parity -- write`.\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Voice/dictation contract: `{}`\n",
            self.voice_and_dictation_contract_ref
        ));
        out.push_str(&format!("- Fixtures: `{}`\n\n", self.fixtures_dir_ref));

        out.push_str("## Claimed text-entry surfaces\n\n");
        out.push_str("| Surface | Class | Support | Reason |\n");
        out.push_str("| --- | --- | --- | --- |\n");
        for surface in &self.surfaces {
            out.push_str(&format!(
                "| `{}` | {} | {} | `{}` |\n",
                surface.surface_id,
                surface.surface_class.as_str(),
                surface.support.as_str(),
                surface.support_reason_ref,
            ));
        }
        out.push('\n');

        out.push_str("## Parity scenarios\n\n");
        out.push_str(
            "| Scenario | Surface | Outcome | Edits | Undo/redo | History group | Restored |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for scenario in &self.scenarios {
            let roundtrip = match scenario.undo_redo {
                Some(rt) => {
                    if rt.returns_to_seed_after_full_undo
                        && rt.returns_to_final_after_full_redo
                        && rt.all_groups_text_edit_class
                    {
                        "round-trips"
                    } else {
                        "drift"
                    }
                }
                None => "-",
            };
            let group = match &scenario.history_group {
                Some(g) => {
                    if g.is_well_formed() {
                        "parity-clean"
                    } else {
                        "drift"
                    }
                }
                None => "-",
            };
            let restored = match scenario.restore_point_preserved {
                Some(true) => "yes",
                Some(false) => "no",
                None => "-",
            };
            out.push_str(&format!(
                "| `{}` | `{}` | {} | {} | {} | {} | {} |\n",
                scenario.scenario_id,
                scenario.surface_id,
                scenario.outcome_class.as_str(),
                scenario.edits.len(),
                roundtrip,
                group,
                restored,
            ));
        }
        out.push('\n');

        out.push_str("## Invariants\n\n");
        let inv = &self.invariants;
        for (label, value) in [
            (
                "Every dictated edit rides an ordinary text-edit undo class",
                inv.every_edit_uses_ordinary_text_edit_class,
            ),
            (
                "Every content edit routes through the shared edit model",
                inv.every_edit_routes_through_shared_edit_model,
            ),
            (
                "Undo and redo round-trip predictably",
                inv.undo_redo_round_trips_predictably,
            ),
            (
                "Cancelling a capture restores the prior insertion point",
                inv.capture_cancel_restores_insertion_point,
            ),
            (
                "Unsupported / degraded surfaces reject explicitly",
                inv.unsupported_surfaces_are_explicit,
            ),
            (
                "History groups are parity-clean",
                inv.history_groups_are_parity_clean,
            ),
            ("No hidden speech-only buffer", inv.no_hidden_speech_buffer),
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
        serde_json::to_string_pretty(self).expect("dictation edit-parity packet serializes")
    }
}

/// Serializes a value as pretty JSON with a trailing newline (the on-disk
/// fixture form).
pub fn fixture_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let mut json = serde_json::to_string_pretty(value)?;
    json.push('\n');
    Ok(json)
}

/// Stable per-scenario fixture file name (the slug after the last `:`).
pub fn scenario_fixture_file_name(scenario: &DictationParityScenario) -> String {
    let slug = scenario
        .scenario_id
        .rsplit(':')
        .next()
        .unwrap_or(&scenario.scenario_id);
    format!("{slug}.json")
}

/// Writes the seeded packet, per-scenario fixtures, and the compact summary to
/// `dir`. This is the single mint path the bin and the equality test share, so
/// the checked-in fixtures can never drift silently.
pub fn write_fixtures(
    dir: &std::path::Path,
    packet: &DictationEditParityPacket,
) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;

    let packet_json =
        fixture_json(packet).map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    std::fs::write(dir.join("packet.json"), packet_json)?;

    for scenario in &packet.scenarios {
        let json = fixture_json(scenario)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        std::fs::write(dir.join(scenario_fixture_file_name(scenario)), json)?;
    }

    let mut compact = packet.compact_lines().join("\n");
    compact.push('\n');
    std::fs::write(dir.join("compact.txt"), compact)?;

    Ok(())
}

mod seed;
pub use seed::seeded_dictation_edit_parity_packet;

#[cfg(test)]
mod tests;
