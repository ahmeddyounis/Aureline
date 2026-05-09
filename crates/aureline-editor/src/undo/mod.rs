//! Undo/redo lineage helpers for editor surfaces.
//!
//! The buffer journal is the single source of truth for undo/redo. Each
//! committed transaction carries:
//! - an [`UndoClass`] (frozen taxonomy);
//! - an `originator` string describing the actor/source lane; and
//! - an optional human-readable `label` for named groups.
//!
//! This module projects that journal metadata into lightweight summaries so
//! editor and shell surfaces can:
//! - use stable originator identifiers for common actions (typing, paste,
//!   external reload); and
//! - report the next undo/redo action name/class without reaching into buffer
//!   internals.

use aureline_buffer::{Buffer, CompensationPosture, UndoClass, UndoGroupId};

/// Stable originator identifiers for editor-owned undo groups.
///
/// These strings are part of the history/lineage contract and should remain
/// stable over time.
pub mod originator {
    /// Keystroke-driven edits (typing, backspace/delete).
    pub const USER_KEYSTROKE: &str = "user_keystroke";

    /// Paste-driven edits sourced from the clipboard.
    pub const PASTE: &str = "paste";

    /// Cut-driven edits (clipboard + delete).
    pub const CUT: &str = "command:editor.cut";

    /// Clean external-change reload that adopts on-disk bytes.
    pub const EXTERNAL_CHANGE_RELOAD: &str = "command:editor.externalChange.reload";
}

/// Summary of one undoable group suitable for UI and structured logging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UndoGroupSummary {
    pub class: UndoClass,
    pub class_id: &'static str,
    pub compensation_posture: CompensationPosture,
    pub undo_group_id: UndoGroupId,
    pub originator: String,
    pub label: Option<String>,
}

impl UndoGroupSummary {
    /// Returns the label when present, otherwise the undo-class id.
    pub fn label_or_class_id(&self) -> &str {
        self.label.as_deref().unwrap_or(self.class_id)
    }
}

/// Returns the next undo group summary, if any.
pub fn next_undo(buffer: &Buffer) -> Option<UndoGroupSummary> {
    summary_from_entry(buffer.peek_undo()?)
}

/// Returns the next redo group summary, if any.
pub fn next_redo(buffer: &Buffer) -> Option<UndoGroupSummary> {
    summary_from_entry(buffer.peek_redo()?)
}

fn summary_from_entry(entry: aureline_buffer::JournalEntry<'_>) -> Option<UndoGroupSummary> {
    Some(UndoGroupSummary {
        class: entry.class(),
        class_id: entry.class_id(),
        compensation_posture: entry.compensation_posture(),
        undo_group_id: entry.undo_group_id(),
        originator: entry.originator().to_string(),
        label: entry.label().map(ToString::to_string),
    })
}
