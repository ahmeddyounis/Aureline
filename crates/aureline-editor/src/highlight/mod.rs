//! Highlight overlay primitives for editor surfaces.
//!
//! This module provides a lightweight, view-independent highlight vocabulary
//! used to paint match overlays (for example in-file find/replace matches)
//! without inventing a parallel text representation or coordinate system.
//!
//! The highlight spans use [`crate::TextPoint`] coordinates so they remain
//! compatible with grapheme-aware navigation and selection semantics.

mod syntax;

use serde::{Deserialize, Serialize};

use crate::viewport::TextPoint;

pub use syntax::{
    EditorTextRange, SyntaxHighlightKind, SyntaxHighlightSourceClass, SyntaxHighlightSpan,
};

/// One highlighted span expressed in `(line, grapheme)` coordinates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HighlightSpan {
    /// Inclusive start point for the highlight.
    pub start: TextPoint,
    /// Exclusive end point for the highlight.
    pub end: TextPoint,
}

/// Highlight overlays that can be painted over editor text.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HighlightOverlaySet {
    /// All match spans to paint.
    pub matches: Vec<HighlightSpan>,
    /// The active match, when one exists.
    pub active_match: Option<HighlightSpan>,
}

impl HighlightOverlaySet {
    /// Returns true when there are no highlights to paint.
    pub fn is_empty(&self) -> bool {
        self.matches.is_empty() && self.active_match.is_none()
    }
}
