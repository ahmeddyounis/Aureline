//! Line indexing for piece-tree snapshots.
//!
//! The buffer core owns byte→(line, column) translation so downstream
//! consumers (views, save, search, history) do not invent competing notions of
//! where lines begin. This module provides a minimal, newline-aware index over
//! a snapshot's immutable bytes.
//!
//! Newline handling:
//! - `\n` (LF) terminates a line.
//! - `\r\n` (CRLF) terminates a line as one unit.
//! - `\r` (CR) terminates a line.
//!
//! Line spans exclude the terminator bytes. A trailing terminator therefore
//! produces a final empty line, matching typical editor semantics.

/// One line's byte range within a snapshot's content, excluding its line
/// terminator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineSpan {
    /// Inclusive start byte offset.
    pub start: usize,
    /// Exclusive end byte offset.
    pub end: usize,
}

impl LineSpan {
    /// Returns the span length in bytes.
    pub const fn len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Returns true when the span has no bytes.
    pub const fn is_empty(self) -> bool {
        self.start >= self.end
    }
}

/// Immutable line index over a snapshot's bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineIndex {
    lines: Vec<LineSpan>,
}

impl LineIndex {
    /// Builds a line index over `bytes`.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut lines: Vec<LineSpan> = Vec::new();
        let mut line_start = 0usize;
        let mut i = 0usize;
        while i < bytes.len() {
            match bytes[i] {
                b'\n' => {
                    lines.push(LineSpan {
                        start: line_start,
                        end: i,
                    });
                    i += 1;
                    line_start = i;
                }
                b'\r' => {
                    lines.push(LineSpan {
                        start: line_start,
                        end: i,
                    });
                    i += 1;
                    if i < bytes.len() && bytes[i] == b'\n' {
                        i += 1;
                    }
                    line_start = i;
                }
                _ => i += 1,
            }
        }
        lines.push(LineSpan {
            start: line_start,
            end: bytes.len(),
        });
        Self { lines }
    }

    /// Returns the number of lines in the indexed content.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Returns the byte span for `line`, excluding line terminators.
    pub fn line_span(&self, line: usize) -> Option<LineSpan> {
        self.lines.get(line).copied()
    }

    /// Returns the line index that contains `offset`.
    ///
    /// `offset == content_len` maps to the final line.
    pub fn line_for_byte_offset(&self, offset: usize) -> Option<usize> {
        if self.lines.is_empty() {
            return None;
        }
        let idx = self
            .lines
            .partition_point(|span| span.start <= offset)
            .saturating_sub(1);
        Some(idx.min(self.lines.len().saturating_sub(1)))
    }
}
