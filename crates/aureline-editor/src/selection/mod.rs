//! Editor selection state and multi-cursor edit application.
//!
//! This module owns the view-local selection model (primary caret, optional
//! selection anchor, and any secondary carets) plus the logic for applying text
//! edits across the active selection set as one grouped buffer transaction.
//!
//! The buffer remains the source of truth for text storage, snapshots, and the
//! undo journal. Selection state is view-local and is updated by mapping prior
//! caret positions through committed edit operations.

use std::cmp::Ordering;

use aureline_buffer::{Buffer, BufferError, CommittedInfo, RevisionId, Snapshot, TransactionSpec};
use aureline_buffer::UndoClass;

use crate::viewport::TextPoint;

/// The selection scope a text edit is applied against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEditScope {
    /// Apply the edit only at the primary caret (and its active selection).
    PrimaryOnly,
    /// Apply the edit once for every caret in the view.
    AllCarets,
}

/// One caret plus its optional selection anchor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaretSelection {
    caret: TextPoint,
    anchor: Option<TextPoint>,
}

impl CaretSelection {
    /// Creates a caret selection with no active range.
    pub const fn new(caret: TextPoint) -> Self {
        Self { caret, anchor: None }
    }

    /// Returns the caret position.
    pub const fn caret(&self) -> TextPoint {
        self.caret
    }

    /// Sets the caret position.
    pub fn set_caret(&mut self, caret: TextPoint) {
        self.caret = caret;
    }

    /// Returns the selection anchor, when one exists.
    pub const fn anchor(&self) -> Option<TextPoint> {
        self.anchor
    }

    /// Sets the selection anchor.
    pub fn set_anchor(&mut self, anchor: Option<TextPoint>) {
        self.anchor = anchor;
    }

    /// Returns an ordered `(start, end)` selection range when non-empty.
    pub fn ordered_range(&self) -> Option<(TextPoint, TextPoint)> {
        let anchor = self.anchor?;
        if anchor == self.caret {
            return None;
        }
        match anchor.key().cmp(&self.caret.key()) {
            Ordering::Less => Some((anchor, self.caret)),
            Ordering::Equal => None,
            Ordering::Greater => Some((self.caret, anchor)),
        }
    }
}

/// View-local selection state for one editor viewport.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionState {
    primary: CaretSelection,
    secondary: Vec<CaretSelection>,
}

impl SelectionState {
    /// Creates a selection state containing exactly one primary caret.
    pub fn new(primary_caret: TextPoint) -> Self {
        Self {
            primary: CaretSelection::new(primary_caret),
            secondary: Vec::new(),
        }
    }

    /// Returns the primary caret position.
    pub const fn primary_caret(&self) -> TextPoint {
        self.primary.caret()
    }

    /// Sets the primary caret position.
    pub fn set_primary_caret(&mut self, caret: TextPoint) {
        self.primary.set_caret(caret);
        self.normalize();
    }

    /// Returns the primary selection anchor, when one exists.
    pub const fn primary_anchor(&self) -> Option<TextPoint> {
        self.primary.anchor()
    }

    /// Sets the primary selection anchor.
    pub fn set_primary_anchor(&mut self, anchor: Option<TextPoint>) {
        self.primary.set_anchor(anchor);
    }

    /// Clears the primary selection anchor.
    pub fn clear_primary_selection(&mut self) {
        self.primary.set_anchor(None);
    }

    /// Returns the number of carets (primary + secondary).
    pub fn caret_count(&self) -> usize {
        1usize.saturating_add(self.secondary.len())
    }

    /// Returns the secondary caret selections.
    pub fn secondary(&self) -> &[CaretSelection] {
        &self.secondary
    }

    /// Returns mutable access to secondary caret selections.
    pub fn secondary_mut(&mut self) -> &mut Vec<CaretSelection> {
        &mut self.secondary
    }

    /// Removes all secondary carets and their anchors.
    pub fn clear_secondary(&mut self) {
        self.secondary.clear();
    }

    /// Adds a secondary caret at `caret` when it is not already present.
    pub fn add_secondary_caret(&mut self, caret: TextPoint) {
        if caret == self.primary.caret() {
            return;
        }
        if self.secondary.iter().any(|item| item.caret() == caret) {
            return;
        }
        self.secondary.push(CaretSelection::new(caret));
        self.normalize();
    }

    /// Returns ordered `(start, end)` ranges for every non-empty selection.
    pub fn ordered_selection_ranges(&self) -> Vec<(TextPoint, TextPoint)> {
        let mut out = Vec::new();
        if let Some(range) = self.primary.ordered_range() {
            out.push(range);
        }
        for caret in &self.secondary {
            if let Some(range) = caret.ordered_range() {
                out.push(range);
            }
        }
        out
    }

    /// Returns the primary ordered selection range when non-empty.
    pub fn primary_selection_range(&self) -> Option<(TextPoint, TextPoint)> {
        self.primary.ordered_range()
    }

    /// Clamps every caret and anchor to `line_graphemes`.
    pub fn clamp_to_document(&mut self, line_graphemes: &[usize]) {
        clamp_selection_to_document(&mut self.primary, line_graphemes);
        for caret in &mut self.secondary {
            clamp_selection_to_document(caret, line_graphemes);
        }
        self.normalize();
    }

    fn normalize(&mut self) {
        self.secondary.retain(|row| row.caret() != self.primary.caret());
        self.secondary.sort_by_key(|row| row.caret().key());
        self.secondary.dedup_by(|a, b| a.caret().key() == b.caret().key());
    }

    /// Applies an insert/replace operation against `scope`.
    ///
    /// Returns `Ok(None)` when `text` is empty or the edit is a no-op.
    pub fn apply_insert_text(
        &mut self,
        buffer: &mut Buffer,
        snapshot: &Snapshot,
        text: &str,
        originator: &str,
        scope: TextEditScope,
    ) -> Result<Option<TextEditOutcome>, BufferError> {
        if text.is_empty() {
            return Ok(None);
        }

        self.normalize();
        let inserted_len = text.as_bytes().len();
        let mut ops = build_insert_ops(snapshot, self, scope, inserted_len);
        if ops.is_empty() {
            return Ok(None);
        }
        ops = normalize_ops(ops);

        let multi_caret = matches!(scope, TextEditScope::AllCarets) && self.caret_count() > 1;
        let undo_class = if multi_caret {
            UndoClass::MultiCursorTextEdit
        } else {
            UndoClass::TextEdit
        };
        let originator = if multi_caret {
            format!("{originator}:multi_cursor")
        } else {
            originator.to_string()
        };

        let mut tx = buffer.begin(TransactionSpec::new(undo_class, originator))?;
        for op in ops.iter().rev() {
            tx.replace(op.start..op.end, text)?;
        }
        let committed = tx.commit()?;
        let revision = buffer.revision_id();
        let next_snapshot = buffer.snapshot();

        remap_carets_after_ops(self, snapshot, &next_snapshot, &ops);
        clear_all_anchors(self);

        Ok(Some(TextEditOutcome {
            committed,
            snapshot: next_snapshot,
            revision,
        }))
    }

    /// Applies a backward-delete (backspace) against `scope`.
    ///
    /// Returns `Ok(None)` when the edit is a no-op (for example at the start of
    /// the document).
    pub fn apply_delete_backward(
        &mut self,
        buffer: &mut Buffer,
        snapshot: &Snapshot,
        originator: &str,
        scope: TextEditScope,
    ) -> Result<Option<TextEditOutcome>, BufferError> {
        self.normalize();
        let mut ops = build_backward_delete_ops(snapshot, self, scope);
        if ops.is_empty() {
            return Ok(None);
        }
        ops = normalize_ops(ops);

        let multi_caret = matches!(scope, TextEditScope::AllCarets) && self.caret_count() > 1;
        let undo_class = if multi_caret {
            UndoClass::MultiCursorTextEdit
        } else {
            UndoClass::TextEdit
        };
        let originator = if multi_caret {
            format!("{originator}:multi_cursor")
        } else {
            originator.to_string()
        };

        let mut tx = buffer.begin(TransactionSpec::new(undo_class, originator))?;
        for op in ops.iter().rev() {
            tx.replace(op.start..op.end, "")?;
        }
        let committed = tx.commit()?;
        let revision = buffer.revision_id();
        let next_snapshot = buffer.snapshot();

        remap_carets_after_ops(self, snapshot, &next_snapshot, &ops);
        clear_all_anchors(self);

        Ok(Some(TextEditOutcome {
            committed,
            snapshot: next_snapshot,
            revision,
        }))
    }
}

/// Result of applying one grouped edit transaction.
#[derive(Debug, Clone)]
pub struct TextEditOutcome {
    /// Information about the committed buffer transaction.
    pub committed: CommittedInfo,
    /// Snapshot produced by the committed edit.
    pub snapshot: Snapshot,
    /// Buffer revision observed after committing.
    pub revision: RevisionId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EditOp {
    start: usize,
    end: usize,
    inserted_len: usize,
}

impl EditOp {
    fn removed_len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    fn delta(self) -> i128 {
        self.inserted_len as i128 - self.removed_len() as i128
    }
}

fn clamp_selection_to_document(selection: &mut CaretSelection, line_graphemes: &[usize]) {
    let line_count = line_graphemes.len().max(1);
    let line = selection.caret.line.min(line_count.saturating_sub(1));
    let max_col = line_graphemes.get(line).copied().unwrap_or(0);
    selection.caret.line = line;
    selection.caret.grapheme = selection.caret.grapheme.min(max_col);

    if let Some(anchor) = selection.anchor {
        let line = anchor.line.min(line_count.saturating_sub(1));
        let max_col = line_graphemes.get(line).copied().unwrap_or(0);
        selection.anchor = Some(TextPoint {
            line,
            grapheme: anchor.grapheme.min(max_col),
        });
    }
}

fn clear_all_anchors(state: &mut SelectionState) {
    state.primary.set_anchor(None);
    for caret in &mut state.secondary {
        caret.set_anchor(None);
    }
}

fn build_insert_ops(
    snapshot: &Snapshot,
    state: &SelectionState,
    scope: TextEditScope,
    inserted_len: usize,
) -> Vec<EditOp> {
    let mut out = Vec::new();

    let mut push_for = |caret: &CaretSelection| {
        if let Some((start, end)) = caret.ordered_range() {
            let start = byte_offset_for_point(snapshot, start);
            let end = byte_offset_for_point(snapshot, end);
            out.push(EditOp {
                start,
                end: end.max(start),
                inserted_len,
            });
        } else {
            let offset = byte_offset_for_point(snapshot, caret.caret());
            out.push(EditOp {
                start: offset,
                end: offset,
                inserted_len,
            });
        }
    };

    match scope {
        TextEditScope::PrimaryOnly => push_for(&state.primary),
        TextEditScope::AllCarets => {
            push_for(&state.primary);
            for caret in &state.secondary {
                push_for(caret);
            }
        }
    }

    out
}

fn build_backward_delete_ops(
    snapshot: &Snapshot,
    state: &SelectionState,
    scope: TextEditScope,
) -> Vec<EditOp> {
    let mut out = Vec::new();
    let mut push_for = |caret: &CaretSelection| {
        if let Some((start, end)) = caret.ordered_range() {
            let start = byte_offset_for_point(snapshot, start);
            let end = byte_offset_for_point(snapshot, end);
            if start < end {
                out.push(EditOp {
                    start,
                    end,
                    inserted_len: 0,
                });
            }
            return;
        }

        let point = caret.caret();
        if point.line == 0 && point.grapheme == 0 {
            return;
        }

        if point.grapheme > 0 {
            let start = byte_offset_for_point(
                snapshot,
                TextPoint {
                    line: point.line,
                    grapheme: point.grapheme.saturating_sub(1),
                },
            );
            let end = byte_offset_for_point(snapshot, point);
            if start < end {
                out.push(EditOp {
                    start,
                    end,
                    inserted_len: 0,
                });
            }
            return;
        }

        let Some(span) = snapshot.line_span(point.line) else {
            return;
        };
        if span.start == 0 {
            return;
        }
        let bytes = snapshot.as_bytes();
        let (start, end) = if span.start >= 2
            && bytes.get(span.start - 2) == Some(&b'\r')
            && bytes.get(span.start - 1) == Some(&b'\n')
        {
            (span.start - 2, span.start)
        } else if bytes.get(span.start - 1) == Some(&b'\n') || bytes.get(span.start - 1) == Some(&b'\r')
        {
            (span.start - 1, span.start)
        } else {
            (span.start.saturating_sub(1), span.start)
        };
        if start < end {
            out.push(EditOp {
                start,
                end,
                inserted_len: 0,
            });
        }
    };

    match scope {
        TextEditScope::PrimaryOnly => push_for(&state.primary),
        TextEditScope::AllCarets => {
            push_for(&state.primary);
            for caret in &state.secondary {
                push_for(caret);
            }
        }
    }

    out
}

fn byte_offset_for_point(snapshot: &Snapshot, point: TextPoint) -> usize {
    snapshot
        .byte_offset_for_line_grapheme(point.line, point.grapheme)
        .unwrap_or(snapshot.len())
}

fn normalize_ops(mut ops: Vec<EditOp>) -> Vec<EditOp> {
    ops.sort_by(|a, b| (a.start, a.end).cmp(&(b.start, b.end)));
    let mut out: Vec<EditOp> = Vec::with_capacity(ops.len());
    for mut op in ops {
        if let Some(last) = out.last_mut() {
            if op.start < last.end {
                if op.inserted_len != last.inserted_len {
                    op.inserted_len = last.inserted_len;
                }
                last.end = last.end.max(op.end);
                continue;
            }
        }
        out.push(op);
    }
    out
}

fn remap_carets_after_ops(
    state: &mut SelectionState,
    prior: &Snapshot,
    next: &Snapshot,
    ops: &[EditOp],
) {
    let primary_offset = byte_offset_for_point(prior, state.primary.caret());
    let primary_mapped = map_offset(primary_offset, ops);
    state.primary.set_caret(point_for_offset(next, primary_mapped));

    for caret in &mut state.secondary {
        let offset = byte_offset_for_point(prior, caret.caret());
        let mapped = map_offset(offset, ops);
        caret.set_caret(point_for_offset(next, mapped));
    }

    state.normalize();
}

fn point_for_offset(snapshot: &Snapshot, offset: usize) -> TextPoint {
    match snapshot.line_grapheme_for_byte_offset(offset) {
        Some((line, grapheme)) => TextPoint { line, grapheme },
        None => TextPoint { line: 0, grapheme: 0 },
    }
}

fn map_offset(mut pos: usize, ops: &[EditOp]) -> usize {
    let mut shift: i128 = 0;
    for op in ops {
        let start = (op.start as i128 + shift).max(0) as usize;
        let end = (op.end as i128 + shift).max(0) as usize;
        let delta = op.delta();
        if pos < start {
            // No change.
        } else if pos >= end {
            pos = ((pos as i128) + delta).max(0) as usize;
        } else {
            pos = start.saturating_add(op.inserted_len);
        }
        shift += delta;
    }
    pos
}
