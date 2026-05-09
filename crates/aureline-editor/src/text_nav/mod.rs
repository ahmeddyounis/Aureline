//! Grapheme-aware text navigation helpers.
//!
//! Viewports store caret positions in `(line, grapheme)` coordinates, but some
//! movement commands (for example "move by word") require consulting the
//! underlying buffer contents. This module hosts deterministic navigation
//! helpers that operate on [`aureline_buffer::Snapshot`] and return updated
//! [`crate::TextPoint`] positions without introducing protocol-specific offset
//! semantics into the editor core.

use aureline_buffer::Snapshot;
use unicode_segmentation::UnicodeSegmentation as _;

use crate::TextPoint;

/// Word-movement direction used by [`move_point_by_word`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordMotion {
    Left,
    Right,
}

/// Moves `point` to the next/previous word boundary.
///
/// The returned position is expressed in `(line, grapheme)` coordinates and is
/// safe for cursoring and deletion operations that must not split grapheme
/// clusters.
pub fn move_point_by_word(
    snapshot: &Snapshot,
    point: TextPoint,
    direction: WordMotion,
) -> Option<TextPoint> {
    match direction {
        WordMotion::Left => move_word_left(snapshot, point),
        WordMotion::Right => move_word_right(snapshot, point),
    }
}

/// Converts `point` into a byte offset within `snapshot`.
///
/// The translation is grapheme-aware: `point.grapheme` is interpreted as an
/// extended grapheme cluster index inside `point.line`.
pub fn byte_offset_for_text_point(snapshot: &Snapshot, point: TextPoint) -> Option<usize> {
    let line = point.line.min(snapshot.line_count().saturating_sub(1));
    snapshot.byte_offset_for_line_grapheme(line, point.grapheme)
}

/// Converts a byte `offset` within `snapshot` into a `(line, grapheme)` point.
///
/// The returned position clamps to `snapshot` bounds and never lands on a
/// grapheme interior.
pub fn text_point_for_byte_offset(snapshot: &Snapshot, offset: usize) -> Option<TextPoint> {
    let offset = offset.min(snapshot.len());
    let (line, grapheme) = snapshot.line_grapheme_for_byte_offset(offset)?;
    Some(TextPoint { line, grapheme })
}

fn move_word_left(snapshot: &Snapshot, point: TextPoint) -> Option<TextPoint> {
    if snapshot.line_count() == 0 {
        return Some(TextPoint {
            line: 0,
            grapheme: 0,
        });
    }

    let mut line = point.line.min(snapshot.line_count().saturating_sub(1));
    let mut local = caret_local_byte(snapshot, line, point.grapheme)?;

    loop {
        let span = snapshot.line_span(line)?;
        let text = snapshot.line_str(line)?;

        if let Some(target) = prev_word_boundary(text, local) {
            return point_for_line_local_byte(snapshot, line, span.start, target);
        }

        if line == 0 {
            return Some(TextPoint {
                line: 0,
                grapheme: 0,
            });
        }

        line = line.saturating_sub(1);
        local = snapshot.line_str(line).map(|row| row.len()).unwrap_or(0);
    }
}

fn move_word_right(snapshot: &Snapshot, point: TextPoint) -> Option<TextPoint> {
    if snapshot.line_count() == 0 {
        return Some(TextPoint {
            line: 0,
            grapheme: 0,
        });
    }

    let mut line = point.line.min(snapshot.line_count().saturating_sub(1));
    let mut local = caret_local_byte(snapshot, line, point.grapheme)?;
    let mut inclusive = false;

    loop {
        let span = snapshot.line_span(line)?;
        let text = snapshot.line_str(line)?;

        let target = if inclusive {
            next_word_boundary_inclusive(text, local)
        } else {
            next_word_boundary(text, local)
        };

        if let Some(target) = target {
            return point_for_line_local_byte(snapshot, line, span.start, target);
        }

        if line + 1 >= snapshot.line_count() {
            return point_for_line_local_byte(snapshot, line, span.start, text.len());
        }

        line = line.saturating_add(1);
        local = 0;
        inclusive = true;
    }
}

fn caret_local_byte(snapshot: &Snapshot, line: usize, grapheme: usize) -> Option<usize> {
    let span = snapshot.line_span(line)?;
    let text = snapshot.line_str(line)?;
    let current_byte = snapshot.byte_offset_for_line_grapheme(line, grapheme)?;
    let local = current_byte.saturating_sub(span.start).min(span.len());
    Some(local.min(text.len()))
}

fn point_for_line_local_byte(
    snapshot: &Snapshot,
    line: usize,
    line_start: usize,
    local: usize,
) -> Option<TextPoint> {
    let global = line_start.saturating_add(local).min(snapshot.len());
    let (mapped_line, grapheme) = snapshot.line_grapheme_for_byte_offset(global)?;
    if mapped_line != line {
        return Some(TextPoint { line, grapheme: 0 });
    }
    Some(TextPoint {
        line: mapped_line,
        grapheme,
    })
}

fn segment_is_word_like(segment: &str) -> bool {
    segment.chars().any(|ch| ch.is_alphanumeric() || ch == '_')
}

fn prev_word_boundary(text: &str, from: usize) -> Option<usize> {
    if from == 0 {
        return None;
    }

    let mut current_word_start: Option<usize> = None;
    let mut prior_word_start: Option<usize> = None;

    for (idx, segment) in text.split_word_bound_indices() {
        if !segment_is_word_like(segment) {
            continue;
        }

        if idx < from {
            prior_word_start = Some(idx);
        }

        let end = idx.saturating_add(segment.len());
        if idx <= from && from < end {
            current_word_start = Some(idx);
        }
    }

    if let Some(start) = current_word_start {
        if from > start {
            return Some(start);
        }
    }

    prior_word_start
}

fn next_word_boundary(text: &str, from: usize) -> Option<usize> {
    if from >= text.len() {
        return None;
    }

    text.split_word_bound_indices()
        .filter_map(|(idx, segment)| {
            if idx > from && segment_is_word_like(segment) {
                Some(idx)
            } else {
                None
            }
        })
        .next()
}

fn next_word_boundary_inclusive(text: &str, from: usize) -> Option<usize> {
    if from > text.len() {
        return None;
    }

    text.split_word_bound_indices()
        .filter_map(|(idx, segment)| {
            if idx >= from && segment_is_word_like(segment) {
                Some(idx)
            } else {
                None
            }
        })
        .next()
}
