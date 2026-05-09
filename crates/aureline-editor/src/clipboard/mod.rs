//! Clipboard planning helpers for editor surfaces.
//!
//! The editor copy path defaults to the [`RepresentationClass::Raw`] view of the
//! underlying buffer snapshot, matching the clipboard representation contract in
//! `docs/ux/clipboard_history_contract.md` (§5). The editor-facing overview and
//! consumer notes live in `docs/editor/copy_contract.md`.
//!
//! This module provides a minimal payload structure so shell consumers can wire
//! copy/cut/paste on live editor surfaces without collapsing future
//! representation-aware variants into an ambiguous plain-text-only path.

use std::ops::Range;

use aureline_buffer::Snapshot;
use serde::{Deserialize, Serialize};

use crate::selection::SelectionState;

/// Representation class describing how clipboard text is produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationClass {
    /// Exact selected bytes (lossless UTF-8).
    Raw,
    /// Rendered surface representation (HTML/styled text); never the only flavour.
    Rendered,
    /// Escaped representation intended for contexts like logs or chat.
    Escaped,
    /// Sanitized representation for suspicious or high-risk content.
    Sanitized,
    /// Metadata-only representation for blocked exports (for example secrets).
    BlockedMetadataOnly,
    /// Generated representation that must carry citation anchors.
    Generated,
}

/// Closed-set copy variant ids used by editor surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyVariantId {
    /// `copy.variant.selection_raw`
    SelectionRaw,
    /// `copy.variant.line`
    Line,
}

impl CopyVariantId {
    /// Returns the stable id string from the clipboard history contract.
    pub const fn id(self) -> &'static str {
        match self {
            Self::SelectionRaw => "copy.variant.selection_raw",
            Self::Line => "copy.variant.line",
        }
    }

    /// Returns the default representation class for the variant.
    pub const fn default_representation_class(self) -> RepresentationClass {
        match self {
            Self::SelectionRaw | Self::Line => RepresentationClass::Raw,
        }
    }
}

/// Clipboard payload ready to project into the system clipboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyPayload {
    /// Stable copy variant id (see [`CopyVariantId::id`]).
    pub copy_variant_id: CopyVariantId,
    /// Representation class applied to `text`.
    pub representation_class: RepresentationClass,
    /// Textual payload for the system clipboard.
    pub text: String,
}

/// Planned cut operation over the editor buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CutPayload {
    /// Clipboard payload to place on the system clipboard.
    pub payload: CopyPayload,
    /// Byte ranges to remove from the buffer snapshot.
    pub delete_ranges: Vec<Range<usize>>,
}

/// Clipboard planning errors for editor surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardError {
    /// The buffer snapshot is not lossless UTF-8, so grapheme-based selection
    /// coordinates cannot be mapped to byte offsets safely.
    NonUtf8Snapshot,
    /// The selection mapped outside the snapshot byte bounds.
    SelectionOutOfBounds,
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonUtf8Snapshot => write!(f, "snapshot is not lossless UTF-8"),
            Self::SelectionOutOfBounds => write!(f, "selection mapped outside snapshot bounds"),
        }
    }
}

impl std::error::Error for ClipboardError {}

/// Plans a default copy for the given selection state.
///
/// When any caret has an active selection range, the planned payload uses
/// [`CopyVariantId::SelectionRaw`]. Otherwise, it falls back to
/// [`CopyVariantId::Line`].
pub fn plan_copy_default(
    snapshot: &Snapshot,
    selections: &SelectionState,
) -> Result<CopyPayload, ClipboardError> {
    if has_any_non_empty_selection(snapshot, selections)? {
        plan_copy_variant(snapshot, selections, CopyVariantId::SelectionRaw)
    } else {
        plan_copy_variant(snapshot, selections, CopyVariantId::Line)
    }
}

/// Plans a default cut for the given selection state.
///
/// When any caret has an active selection range, the planned cut deletes the
/// selection ranges. Otherwise, it deletes the entire line(s) covered by the
/// active caret set (including line terminators when present).
pub fn plan_cut_default(
    snapshot: &Snapshot,
    selections: &SelectionState,
) -> Result<CutPayload, ClipboardError> {
    if has_any_non_empty_selection(snapshot, selections)? {
        plan_cut_variant(snapshot, selections, CopyVariantId::SelectionRaw)
    } else {
        plan_cut_variant(snapshot, selections, CopyVariantId::Line)
    }
}

/// Plans a copy payload for a specific clipboard variant.
pub fn plan_copy_variant(
    snapshot: &Snapshot,
    selections: &SelectionState,
    variant: CopyVariantId,
) -> Result<CopyPayload, ClipboardError> {
    let text = snapshot.as_str().ok_or(ClipboardError::NonUtf8Snapshot)?;
    let representation_class = variant.default_representation_class();

    let text_payload = match variant {
        CopyVariantId::SelectionRaw => {
            let ranges = selection_ranges(snapshot, selections)?;
            join_segments(text, &ranges, SegmentJoin::LineFeedSeparated)?
        }
        CopyVariantId::Line => {
            let ranges = line_ranges(snapshot, selections)?;
            join_segments(text, &ranges, SegmentJoin::Concatenate)?
        }
    };

    Ok(CopyPayload {
        copy_variant_id: variant,
        representation_class,
        text: text_payload,
    })
}

/// Plans a cut payload and the associated buffer deletions for a variant.
pub fn plan_cut_variant(
    snapshot: &Snapshot,
    selections: &SelectionState,
    variant: CopyVariantId,
) -> Result<CutPayload, ClipboardError> {
    let payload = plan_copy_variant(snapshot, selections, variant)?;
    let delete_ranges = match variant {
        CopyVariantId::SelectionRaw => selection_ranges(snapshot, selections)?,
        CopyVariantId::Line => line_ranges(snapshot, selections)?,
    };

    Ok(CutPayload {
        payload,
        delete_ranges,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SegmentJoin {
    Concatenate,
    LineFeedSeparated,
}

fn join_segments(
    text: &str,
    ranges: &[Range<usize>],
    join: SegmentJoin,
) -> Result<String, ClipboardError> {
    let mut out = String::new();
    for (idx, range) in ranges.iter().enumerate() {
        if idx > 0 {
            match join {
                SegmentJoin::Concatenate => {}
                SegmentJoin::LineFeedSeparated => out.push('\n'),
            }
        }
        let segment = text
            .get(range.clone())
            .ok_or(ClipboardError::SelectionOutOfBounds)?;
        out.push_str(segment);
    }
    Ok(out)
}

fn has_any_non_empty_selection(
    snapshot: &Snapshot,
    selections: &SelectionState,
) -> Result<bool, ClipboardError> {
    let ranges = selection_ranges(snapshot, selections)?;
    Ok(!ranges.is_empty())
}

fn selection_ranges(
    snapshot: &Snapshot,
    selections: &SelectionState,
) -> Result<Vec<Range<usize>>, ClipboardError> {
    let text = snapshot.as_str().ok_or(ClipboardError::NonUtf8Snapshot)?;

    let mut ranges: Vec<Range<usize>> = Vec::new();
    if let Some((start, end)) = selections.primary_selection_range() {
        let range = byte_range_for_points(snapshot, text, start, end)?;
        if range.start < range.end {
            ranges.push(range);
        }
    }
    for caret in selections.secondary() {
        if let Some((start, end)) = caret.ordered_range() {
            let range = byte_range_for_points(snapshot, text, start, end)?;
            if range.start < range.end {
                ranges.push(range);
            }
        }
    }

    ranges.sort_by_key(|range| (range.start, range.end));
    Ok(merge_overlapping_ranges(ranges))
}

fn line_ranges(
    snapshot: &Snapshot,
    selections: &SelectionState,
) -> Result<Vec<Range<usize>>, ClipboardError> {
    let _ = snapshot.as_str().ok_or(ClipboardError::NonUtf8Snapshot)?;

    let mut lines: Vec<usize> = Vec::new();
    lines.push(selections.primary_caret().line);
    for caret in selections.secondary() {
        lines.push(caret.caret().line);
    }
    lines.sort_unstable();
    lines.dedup();

    let mut ranges: Vec<Range<usize>> = Vec::new();
    let line_count = snapshot.line_count().max(1);
    for line in lines {
        let line = line.min(line_count.saturating_sub(1));
        let Some(span) = snapshot.line_span(line) else {
            continue;
        };
        let end = if line + 1 < line_count {
            snapshot
                .line_span(line + 1)
                .map(|next| next.start)
                .unwrap_or(snapshot.len())
        } else {
            snapshot.len()
        };
        let start = span.start.min(end);
        ranges.push(start..end.max(start));
    }

    ranges.sort_by_key(|range| (range.start, range.end));
    Ok(merge_overlapping_ranges(ranges))
}

fn byte_range_for_points(
    snapshot: &Snapshot,
    text: &str,
    start: crate::viewport::TextPoint,
    end: crate::viewport::TextPoint,
) -> Result<Range<usize>, ClipboardError> {
    let start_offset = byte_offset_for_point(snapshot, start)?;
    let end_offset = byte_offset_for_point(snapshot, end)?;
    let start_offset = start_offset.min(end_offset);
    let end_offset = end_offset.max(start_offset);
    if end_offset > text.len() {
        return Err(ClipboardError::SelectionOutOfBounds);
    }
    Ok(start_offset..end_offset)
}

fn byte_offset_for_point(
    snapshot: &Snapshot,
    point: crate::viewport::TextPoint,
) -> Result<usize, ClipboardError> {
    snapshot
        .byte_offset_for_line_grapheme(point.line, point.grapheme)
        .ok_or(ClipboardError::SelectionOutOfBounds)
}

fn merge_overlapping_ranges(mut ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    if ranges.is_empty() {
        return ranges;
    }

    let mut out: Vec<Range<usize>> = Vec::with_capacity(ranges.len());
    for range in ranges.drain(..) {
        if let Some(last) = out.last_mut() {
            if range.start < last.end {
                last.end = last.end.max(range.end);
                continue;
            }
        }
        out.push(range);
    }
    out
}
