//! In-file find/replace state and match computation.
//!
//! This module provides the lexical-only foundation for in-buffer search and
//! replace. The match model uses byte offsets over the buffer snapshot as the
//! source of truth and projects highlight spans in grapheme coordinates so
//! viewport overlays can paint matches without losing source fidelity.
//!
//! Notes:
//! - M1 is intentionally lexical-only: no semantic awareness and no language
//!   provider integration.
//! - The implementation avoids inventing a parallel text engine. Every match is
//!   computed against the live [`aureline_buffer::Snapshot`] text.

use std::ops::Range;

use aureline_buffer::{Buffer, BufferError, CommittedInfo, RevisionId, Snapshot, SnapshotId};
use aureline_buffer::{TransactionSpec, UndoClass};
use serde::{Deserialize, Serialize};

use crate::highlight::{HighlightOverlaySet, HighlightSpan};
use crate::viewport::TextPoint;

/// Find/replace overlay mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindReplaceMode {
    /// Find/replace is inactive and paints no highlights.
    Hidden,
    /// Find UI is active.
    Find,
    /// Replace UI is active.
    Replace,
}

/// Options that control lexical match discovery.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindOptions {
    /// When false, matches ignore ASCII case differences.
    pub case_sensitive: bool,
    /// When true, matches must be bounded by ASCII word boundaries.
    pub whole_word: bool,
}

/// Degraded-state vocabulary for in-file find/replace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FindReplaceDegradedReason {
    /// Snapshot was not lossless UTF-8, so string matching cannot run safely.
    NonUtf8Snapshot,
    /// Match discovery was capped by the scan budget.
    ScanBudgetExceeded {
        scanned_bytes: usize,
        total_bytes: usize,
    },
    /// Match discovery was capped by the match-count budget.
    MatchBudgetExceeded { match_cap: usize },
}

/// Errors returned by in-file find/replace helpers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindReplaceError {
    /// The snapshot is not lossless UTF-8.
    NonUtf8Snapshot,
    /// Replace was attempted without an active match.
    NoActiveMatch,
    /// Replace-all was blocked because the match set is known to be partial.
    LimitedMatchSet,
    /// Buffer transaction failed.
    Buffer(BufferError),
}

impl std::fmt::Display for FindReplaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonUtf8Snapshot => write!(f, "snapshot is not lossless UTF-8"),
            Self::NoActiveMatch => write!(f, "no active match to replace"),
            Self::LimitedMatchSet => write!(f, "match set is limited; replace-all is blocked"),
            Self::Buffer(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for FindReplaceError {}

impl From<BufferError> for FindReplaceError {
    fn from(value: BufferError) -> Self {
        Self::Buffer(value)
    }
}

/// Result of applying a replace operation as one buffer transaction.
#[derive(Debug, Clone)]
pub struct ReplaceOutcome {
    /// Information about the committed buffer transaction.
    pub committed: CommittedInfo,
    /// Snapshot produced by the committed edit.
    pub snapshot: Snapshot,
    /// Buffer revision observed after committing.
    pub revision: RevisionId,
    /// Number of replaced matches.
    pub replaced_count: usize,
}

/// Canonical in-file find/replace state bound to one buffer.
#[derive(Debug, Clone)]
pub struct FindReplaceState {
    mode: FindReplaceMode,
    query: String,
    replacement: String,
    options: FindOptions,

    computed_snapshot_id: Option<SnapshotId>,
    matches: Vec<Range<usize>>,
    active_match_index: Option<usize>,

    max_scan_bytes: usize,
    max_match_count: usize,
    degraded_reason: Option<FindReplaceDegradedReason>,

    highlight_cache_snapshot_id: Option<SnapshotId>,
    highlight_cache: HighlightOverlaySet,
}

impl Default for FindReplaceState {
    fn default() -> Self {
        Self::new()
    }
}

impl FindReplaceState {
    /// Creates a new find/replace state with conservative budgets.
    pub fn new() -> Self {
        Self {
            mode: FindReplaceMode::Hidden,
            query: String::new(),
            replacement: String::new(),
            options: FindOptions::default(),
            computed_snapshot_id: None,
            matches: Vec::new(),
            active_match_index: None,
            max_scan_bytes: 2_000_000,
            max_match_count: 2048,
            degraded_reason: None,
            highlight_cache_snapshot_id: None,
            highlight_cache: HighlightOverlaySet::default(),
        }
    }

    /// Configures the scan and match budgets used for match discovery.
    ///
    /// When either budget is exceeded, the state reports a degraded reason and
    /// truncates match enumeration accordingly.
    pub fn configure_budgets(&mut self, max_scan_bytes: usize, max_match_count: usize) {
        let max_scan_bytes = max_scan_bytes.max(1);
        let max_match_count = max_match_count.max(1);
        if self.max_scan_bytes == max_scan_bytes && self.max_match_count == max_match_count {
            return;
        }
        self.max_scan_bytes = max_scan_bytes;
        self.max_match_count = max_match_count;
        self.invalidate_matches();
    }

    /// Returns the active mode for the state.
    pub const fn mode(&self) -> FindReplaceMode {
        self.mode
    }

    /// Returns the current query string.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Returns the current replacement string.
    pub fn replacement(&self) -> &str {
        &self.replacement
    }

    /// Returns the configured find options.
    pub const fn options(&self) -> FindOptions {
        self.options
    }

    /// Returns the active degraded reason, when one exists.
    pub fn degraded_reason(&self) -> Option<&FindReplaceDegradedReason> {
        self.degraded_reason.as_ref()
    }

    /// Returns the number of matches in the current match set.
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Returns the active match index (zero-based), when one exists.
    pub fn active_match_index(&self) -> Option<usize> {
        self.active_match_index
    }

    /// Returns the active match byte range, when one exists.
    pub fn active_match_range(&self) -> Option<Range<usize>> {
        let idx = self.active_match_index?;
        self.matches.get(idx).cloned()
    }

    /// Returns highlight overlays for the last synced snapshot, when active.
    pub fn highlight_overlays(&self) -> Option<&HighlightOverlaySet> {
        if self.mode == FindReplaceMode::Hidden || self.query.trim().is_empty() {
            return None;
        }
        if self.highlight_cache_snapshot_id.is_some() && !self.highlight_cache.is_empty() {
            Some(&self.highlight_cache)
        } else {
            None
        }
    }

    /// Sets the active mode.
    pub fn set_mode(&mut self, mode: FindReplaceMode) {
        if mode == self.mode {
            return;
        }
        self.mode = mode;
        if mode == FindReplaceMode::Hidden {
            self.clear_runtime_state();
        }
    }

    /// Closes the active find/replace mode (hides highlights).
    pub fn close(&mut self) {
        self.set_mode(FindReplaceMode::Hidden);
    }

    /// Replaces the query string and invalidates the current match cache.
    pub fn set_query(&mut self, query: impl Into<String>) {
        let query = query.into();
        if query == self.query {
            return;
        }
        self.query = query;
        self.invalidate_matches();
    }

    /// Replaces the replacement string.
    pub fn set_replacement(&mut self, replacement: impl Into<String>) {
        self.replacement = replacement.into();
    }

    /// Toggles ASCII-only case sensitivity.
    pub fn toggle_case_sensitive(&mut self) {
        self.options.case_sensitive = !self.options.case_sensitive;
        self.invalidate_matches();
    }

    /// Toggles ASCII whole-word matching.
    pub fn toggle_whole_word(&mut self) {
        self.options.whole_word = !self.options.whole_word;
        self.invalidate_matches();
    }

    /// Ensures the match set and highlight overlays are current for `snapshot`.
    pub fn sync_for_view(
        &mut self,
        snapshot: &Snapshot,
        caret: TextPoint,
    ) -> Result<(), FindReplaceError> {
        if self.mode == FindReplaceMode::Hidden || self.query.trim().is_empty() {
            self.clear_runtime_state();
            self.computed_snapshot_id = Some(snapshot.id());
            return Ok(());
        }

        self.ensure_matches_current(snapshot)?;
        self.select_active_for_caret(snapshot, caret);
        self.refresh_highlight_cache(snapshot);
        Ok(())
    }

    /// Advances the active match to the next match in the set.
    pub fn select_next(
        &mut self,
        snapshot: &Snapshot,
        caret: TextPoint,
    ) -> Result<Option<Range<usize>>, FindReplaceError> {
        self.sync_for_view(snapshot, caret)?;
        if self.matches.is_empty() {
            return Ok(None);
        }
        let next = match self.active_match_index {
            Some(idx) => (idx + 1) % self.matches.len(),
            None => 0,
        };
        self.active_match_index = Some(next);
        self.refresh_highlight_cache(snapshot);
        Ok(self.active_match_range())
    }

    /// Moves the active match to the previous match in the set.
    pub fn select_prev(
        &mut self,
        snapshot: &Snapshot,
        caret: TextPoint,
    ) -> Result<Option<Range<usize>>, FindReplaceError> {
        self.sync_for_view(snapshot, caret)?;
        if self.matches.is_empty() {
            return Ok(None);
        }
        let len = self.matches.len();
        let next = match self.active_match_index {
            Some(idx) => (idx + len - 1) % len,
            None => 0,
        };
        self.active_match_index = Some(next);
        self.refresh_highlight_cache(snapshot);
        Ok(self.active_match_range())
    }

    /// Applies a replace for the currently active match.
    ///
    /// The replace is executed as one buffer transaction so undo/redo can treat
    /// the operation as an attributable unit.
    pub fn replace_active(
        &mut self,
        buffer: &mut Buffer,
        snapshot: &Snapshot,
        caret: TextPoint,
        originator: &str,
    ) -> Result<Option<ReplaceOutcome>, FindReplaceError> {
        self.sync_for_view(snapshot, caret)?;
        let Some(active) = self.active_match_range() else {
            return Ok(None);
        };

        let mut tx = buffer.begin(TransactionSpec::new(
            UndoClass::StructuralEdit,
            originator.to_string(),
        ))?;
        tx.replace(active.clone(), &self.replacement)?;
        let committed = tx.commit()?;
        let revision = buffer.revision_id();
        let next_snapshot = buffer.snapshot();

        self.invalidate_matches();

        Ok(Some(ReplaceOutcome {
            committed,
            snapshot: next_snapshot,
            revision,
            replaced_count: 1,
        }))
    }

    /// Applies a replace over every match in the current match set.
    pub fn replace_all(
        &mut self,
        buffer: &mut Buffer,
        snapshot: &Snapshot,
        caret: TextPoint,
        originator: &str,
    ) -> Result<Option<ReplaceOutcome>, FindReplaceError> {
        self.sync_for_view(snapshot, caret)?;
        if self.matches.is_empty() {
            return Ok(None);
        }
        if matches!(
            self.degraded_reason,
            Some(FindReplaceDegradedReason::ScanBudgetExceeded { .. })
                | Some(FindReplaceDegradedReason::MatchBudgetExceeded { .. })
        ) {
            return Err(FindReplaceError::LimitedMatchSet);
        }

        let replaced_count = self.matches.len();
        let mut tx = buffer.begin(TransactionSpec::new(
            UndoClass::StructuralEdit,
            originator.to_string(),
        ))?;
        for range in self.matches.iter().rev() {
            tx.replace(range.clone(), &self.replacement)?;
        }
        let committed = tx.commit()?;
        let revision = buffer.revision_id();
        let next_snapshot = buffer.snapshot();

        self.invalidate_matches();

        Ok(Some(ReplaceOutcome {
            committed,
            snapshot: next_snapshot,
            revision,
            replaced_count,
        }))
    }

    fn invalidate_matches(&mut self) {
        self.computed_snapshot_id = None;
        self.highlight_cache_snapshot_id = None;
        self.matches.clear();
        self.active_match_index = None;
        self.highlight_cache = HighlightOverlaySet::default();
        self.degraded_reason = None;
    }

    fn clear_runtime_state(&mut self) {
        self.computed_snapshot_id = None;
        self.highlight_cache_snapshot_id = None;
        self.matches.clear();
        self.active_match_index = None;
        self.highlight_cache = HighlightOverlaySet::default();
        self.degraded_reason = None;
    }

    fn ensure_matches_current(&mut self, snapshot: &Snapshot) -> Result<(), FindReplaceError> {
        if self.computed_snapshot_id == Some(snapshot.id()) {
            return Ok(());
        }
        let Some(text) = snapshot.as_str() else {
            self.matches.clear();
            self.active_match_index = None;
            self.computed_snapshot_id = Some(snapshot.id());
            self.degraded_reason = Some(FindReplaceDegradedReason::NonUtf8Snapshot);
            self.highlight_cache_snapshot_id = None;
            self.highlight_cache = HighlightOverlaySet::default();
            return Err(FindReplaceError::NonUtf8Snapshot);
        };
        let needle = self.query.as_str();
        let (matches, degraded_reason) = find_matches_with_budget(
            text,
            needle,
            self.options,
            self.max_scan_bytes,
            self.max_match_count,
        );

        self.matches = matches;
        self.computed_snapshot_id = Some(snapshot.id());
        self.degraded_reason = degraded_reason;
        Ok(())
    }

    fn select_active_for_caret(&mut self, snapshot: &Snapshot, caret: TextPoint) {
        if self.matches.is_empty() {
            self.active_match_index = None;
            return;
        }

        let caret_offset = snapshot
            .byte_offset_for_line_grapheme(caret.line, caret.grapheme)
            .unwrap_or(snapshot.len());

        if let Some((idx, _)) = self
            .matches
            .iter()
            .enumerate()
            .find(|(_, range)| range.start <= caret_offset && caret_offset < range.end)
        {
            self.active_match_index = Some(idx);
            return;
        }

        if let Some((idx, _)) = self
            .matches
            .iter()
            .enumerate()
            .find(|(_, range)| range.start >= caret_offset)
        {
            self.active_match_index = Some(idx);
            return;
        }

        self.active_match_index = Some(0);
    }

    fn refresh_highlight_cache(&mut self, snapshot: &Snapshot) {
        if self.mode == FindReplaceMode::Hidden
            || self.query.trim().is_empty()
            || self.matches.is_empty()
        {
            self.highlight_cache_snapshot_id = None;
            self.highlight_cache = HighlightOverlaySet::default();
            return;
        }

        if self.highlight_cache_snapshot_id != Some(snapshot.id()) {
            let overlays = HighlightOverlaySet {
                matches: self
                    .matches
                    .iter()
                    .filter_map(|range| highlight_span_for_byte_range(snapshot, range))
                    .collect(),
                ..HighlightOverlaySet::default()
            };
            self.highlight_cache_snapshot_id = Some(snapshot.id());
            self.highlight_cache = overlays;
        }

        self.highlight_cache.active_match = self
            .active_match_range()
            .and_then(|range| highlight_span_for_byte_range(snapshot, &range));
    }
}

fn find_matches_with_budget(
    haystack: &str,
    needle: &str,
    options: FindOptions,
    max_scan_bytes: usize,
    max_match_count: usize,
) -> (Vec<Range<usize>>, Option<FindReplaceDegradedReason>) {
    if needle.is_empty() {
        return (Vec::new(), None);
    }

    let mut scan_limit = haystack.len().min(max_scan_bytes);
    while scan_limit > 0 && !haystack.is_char_boundary(scan_limit) {
        scan_limit = scan_limit.saturating_sub(1);
    }
    let limited_haystack = &haystack[..scan_limit];

    let mut matches_truncated = false;
    let mut out: Vec<Range<usize>> = Vec::new();

    if !options.case_sensitive && needle.is_ascii() {
        let needle_bytes = needle.as_bytes();
        let hay_bytes = limited_haystack.as_bytes();
        let mut pos = 0usize;
        while pos <= hay_bytes.len().saturating_sub(needle_bytes.len()) {
            let Some(found) = find_next_ascii_case_insensitive(hay_bytes, needle_bytes, pos) else {
                break;
            };
            let start = found;
            let end = start.saturating_add(needle_bytes.len());
            if options.whole_word && !is_ascii_whole_word_boundary(hay_bytes, start, end) {
                pos = start.saturating_add(1);
                continue;
            }
            out.push(start..end);
            if out.len() >= max_match_count {
                matches_truncated = true;
                break;
            }
            pos = end;
        }
    } else {
        for (start, matched) in limited_haystack.match_indices(needle) {
            let end = start.saturating_add(matched.len());
            if options.whole_word
                && !is_ascii_whole_word_boundary(limited_haystack.as_bytes(), start, end)
            {
                continue;
            }
            out.push(start..end);
            if out.len() >= max_match_count {
                matches_truncated = true;
                break;
            }
        }
    }

    let degraded_reason = if scan_limit < haystack.len() {
        Some(FindReplaceDegradedReason::ScanBudgetExceeded {
            scanned_bytes: scan_limit,
            total_bytes: haystack.len(),
        })
    } else if matches_truncated {
        Some(FindReplaceDegradedReason::MatchBudgetExceeded {
            match_cap: max_match_count,
        })
    } else {
        None
    };

    (out, degraded_reason)
}

fn find_next_ascii_case_insensitive(hay: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if needle.is_empty() || hay.len() < needle.len() || from >= hay.len() {
        return None;
    }
    let max_start = hay.len().saturating_sub(needle.len());
    (from..=max_start)
        .find(|&start| eq_ascii_case_insensitive(&hay[start..start + needle.len()], needle))
}

fn eq_ascii_case_insensitive(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .all(|(left, right)| left.eq_ignore_ascii_case(right))
}

fn is_ascii_whole_word_boundary(bytes: &[u8], start: usize, end: usize) -> bool {
    let prev = start.checked_sub(1).and_then(|idx| bytes.get(idx).copied());
    let next = bytes.get(end).copied();

    let prev_is_word = prev.is_some_and(is_ascii_word_byte);
    let next_is_word = next.is_some_and(is_ascii_word_byte);
    !prev_is_word && !next_is_word
}

fn is_ascii_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn highlight_span_for_byte_range(
    snapshot: &Snapshot,
    range: &Range<usize>,
) -> Option<HighlightSpan> {
    if range.start >= range.end {
        return None;
    }
    let start_offset = range.start.min(snapshot.len());
    let end_offset = range.end.min(snapshot.len());
    if start_offset >= end_offset {
        return None;
    }

    let start = floor_point_for_offset(snapshot, start_offset)?;
    let (end_line, end_grapheme) = snapshot.line_grapheme_for_byte_offset(end_offset)?;
    let end = TextPoint {
        line: end_line,
        grapheme: end_grapheme,
    };

    Some(HighlightSpan { start, end })
}

fn floor_point_for_offset(snapshot: &Snapshot, offset: usize) -> Option<TextPoint> {
    let (line, grapheme) = snapshot.line_grapheme_for_byte_offset(offset)?;
    let boundary = snapshot.byte_offset_for_line_grapheme(line, grapheme)?;
    if boundary > offset && grapheme > 0 {
        Some(TextPoint {
            line,
            grapheme: grapheme.saturating_sub(1),
        })
    } else {
        Some(TextPoint { line, grapheme })
    }
}
