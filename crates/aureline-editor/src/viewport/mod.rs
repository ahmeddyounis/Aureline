//! Editor viewport state and damage classification.
//!
//! The editor viewport is the canonical owner of scroll, caret, and selection
//! state for one visible editor surface. It also caches the line-layout data
//! needed to paint overlays (caret, selection, IME) without re-shaping or
//! re-rasterizing glyphs.

use std::cmp::Ordering;

use aureline_render::draw_queue::{CompositionLayerId, DamageClassId, DamageEvent, DamageRegion};
use aureline_render::hooks::Hook;
use aureline_render::PixelRect;
use serde::{Deserialize, Serialize};

/// A caret/selection position expressed in line + grapheme coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextPoint {
    /// Zero-based line index.
    pub line: usize,
    /// Zero-based grapheme column within `line`.
    pub grapheme: usize,
}

impl TextPoint {
    /// Returns a `(line, grapheme)` ordering key.
    pub const fn key(self) -> (usize, usize) {
        (self.line, self.grapheme)
    }
}

/// Directional caret movement within a viewport.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaretMove {
    Left,
    Right,
    Up,
    Down,
    LineStart,
    LineEnd,
    PageUp,
    PageDown,
}

/// Selection mutations issued by input dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionDelta {
    /// Clears selection and collapses to the caret.
    Cleared,
    /// Extends the selection by one grapheme to the left.
    ExtendedLeft,
    /// Extends the selection by one grapheme to the right.
    ExtendedRight,
}

/// IME composition metadata for overlay rendering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImeComposition {
    /// Preedit text.
    pub text: String,
    /// Caret offset within `text`, in bytes.
    pub caret_byte_offset: usize,
}

/// Editor input action vocabulary used by the viewport compositor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EditorAction {
    /// Inserts the provided text at the caret.
    InsertText { text: String },
    /// Deletes one grapheme to the left of the caret (backspace semantics).
    DeleteBackward,
    /// Moves the caret.
    MoveCaret {
        movement: CaretMove,
        /// When true, preserves/extends the selection while moving.
        #[serde(default)]
        extend_selection: bool,
    },
    /// Changes selection state.
    ChangeSelection { delta: SelectionDelta },
    /// Updates the active IME preedit composition.
    UpdateComposition { composition: ImeComposition },
    /// Clears IME preedit state (for example when IME is disabled).
    ClearComposition,
    /// Scrolls the viewport by a line delta.
    ScrollLines { dy_lines: i32 },
    /// Signals a multi-monitor scale change.
    ScaleChange,
}

/// Output of classifying and applying an editor action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewportDamage {
    /// The damage event to enqueue.
    pub event: DamageEvent,
    /// The protected-path hook associated with the action.
    pub hook: Hook,
}

/// Cached layout information for one shaped line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineLayout {
    /// Document line index.
    pub line_index: usize,
    /// Top-edge y coordinate within the viewport, in pixels.
    pub y_top_px: i32,
    /// Grapheme-boundary x positions (length = grapheme_count + 1).
    pub grapheme_x_px: Vec<u32>,
}

/// Layout cache for the visible viewport region.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ViewportLayout {
    pub first_visible_line: usize,
    pub line_height_px: u32,
    pub viewport_width_px: u32,
    pub viewport_height_px: u32,
    pub lines: Vec<LineLayout>,
}

impl ViewportLayout {
    /// Returns the layout entry for `line_index` when cached.
    pub fn line(&self, line_index: usize) -> Option<&LineLayout> {
        self.lines.iter().find(|row| row.line_index == line_index)
    }
}

/// Snapshot of viewport state suitable for structured logging.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorViewportSnapshot {
    pub scroll_line: usize,
    pub caret: TextPoint,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_anchor: Option<TextPoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ime_composition: Option<ImeComposition>,
}

/// Canonical editor viewport state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorViewport {
    scroll_line: usize,
    caret: TextPoint,
    selection_anchor: Option<TextPoint>,
    ime_composition: Option<ImeComposition>,
    layout: ViewportLayout,
    /// Column preference captured when moving vertically.
    preferred_grapheme_column: usize,
}

impl EditorViewport {
    /// Creates a new viewport with its caret at the origin.
    pub fn new() -> Self {
        Self {
            scroll_line: 0,
            caret: TextPoint { line: 0, grapheme: 0 },
            selection_anchor: None,
            ime_composition: None,
            layout: ViewportLayout::default(),
            preferred_grapheme_column: 0,
        }
    }

    /// Returns the current scroll line.
    pub const fn scroll_line(&self) -> usize {
        self.scroll_line
    }

    /// Returns the caret position.
    pub const fn caret(&self) -> TextPoint {
        self.caret
    }

    /// Replaces the caret position.
    pub fn set_caret(&mut self, caret: TextPoint) {
        self.caret = caret;
        self.preferred_grapheme_column = caret.grapheme;
    }

    /// Clears any active selection.
    pub fn clear_selection(&mut self) {
        self.selection_anchor = None;
    }

    /// Replaces the selection anchor.
    pub fn set_selection_anchor(&mut self, anchor: Option<TextPoint>) {
        self.selection_anchor = anchor;
    }

    /// Returns the active selection anchor, when present.
    pub const fn selection_anchor(&self) -> Option<TextPoint> {
        self.selection_anchor
    }

    /// Returns the cached viewport layout.
    pub const fn layout(&self) -> &ViewportLayout {
        &self.layout
    }

    /// Returns a structured snapshot suitable for JSON serialization.
    pub fn snapshot(&self) -> EditorViewportSnapshot {
        EditorViewportSnapshot {
            scroll_line: self.scroll_line,
            caret: self.caret,
            selection_anchor: self.selection_anchor,
            ime_composition: self.ime_composition.clone(),
        }
    }

    /// Replaces the cached layout.
    pub fn set_layout(&mut self, layout: ViewportLayout) {
        self.layout = layout;
    }

    /// Updates scroll position by `dy_lines`, clamping to `max_scroll_line`.
    pub fn scroll_by_lines(&mut self, dy_lines: i32, max_scroll_line: usize) -> bool {
        let before = self.scroll_line;
        let new_scroll = if dy_lines.is_negative() {
            before.saturating_sub(dy_lines.wrapping_abs() as usize)
        } else {
            before.saturating_add(dy_lines as usize)
        };
        self.scroll_line = new_scroll.min(max_scroll_line);
        self.scroll_line != before
    }

    /// Applies an editor action and returns its damage classification.
    ///
    /// The returned [`ViewportDamage`] always targets `viewport_rect` or a
    /// subset of it; callers should enqueue the event into the shared draw
    /// queue so dirty-region planning stays aligned.
    pub fn apply_action(
        &mut self,
        action: &EditorAction,
        viewport_rect: PixelRect,
        max_scroll_line: usize,
    ) -> Option<ViewportDamage> {
        if viewport_rect.is_empty() {
            return None;
        }

        match action {
            EditorAction::InsertText { .. } | EditorAction::DeleteBackward => {
                Some(ViewportDamage {
                    event: DamageEvent::with_region(
                        CompositionLayerId::TextAndDecoration,
                        DamageClassId::TextReflowLocal,
                        DamageRegion::Rect(viewport_rect),
                    ),
                    hook: Hook::ReflowLineRange,
                })
            }
            EditorAction::MoveCaret {
                extend_selection, ..
            } => {
                if *extend_selection {
                    Some(ViewportDamage {
                        event: DamageEvent::with_region(
                            CompositionLayerId::OverlayEphemera,
                            DamageClassId::SelectionOverlayOnly,
                            DamageRegion::Rect(viewport_rect),
                        ),
                        hook: Hook::SelectionChange,
                    })
                } else {
                    Some(ViewportDamage {
                        event: DamageEvent::with_region(
                            CompositionLayerId::OverlayEphemera,
                            DamageClassId::CaretOverlayOnly,
                            DamageRegion::Rect(viewport_rect),
                        ),
                        hook: Hook::CaretMove,
                    })
                }
            }
            EditorAction::ChangeSelection { .. } => Some(ViewportDamage {
                event: DamageEvent::with_region(
                    CompositionLayerId::OverlayEphemera,
                    DamageClassId::SelectionOverlayOnly,
                    DamageRegion::Rect(viewport_rect),
                ),
                hook: Hook::SelectionChange,
            }),
            EditorAction::UpdateComposition { .. } | EditorAction::ClearComposition => {
                Some(ViewportDamage {
                    event: DamageEvent::with_region(
                        CompositionLayerId::OverlayEphemera,
                        DamageClassId::ImeMarkedTextOverlay,
                        DamageRegion::Rect(viewport_rect),
                    ),
                    hook: Hook::ImeCompositionUpdate,
                })
            }
            EditorAction::ScrollLines { dy_lines } => {
                if self.scroll_by_lines(*dy_lines, max_scroll_line) {
                    Some(ViewportDamage {
                        event: DamageEvent::with_region(
                            CompositionLayerId::TextAndDecoration,
                            DamageClassId::ViewportScrollTranslate,
                            DamageRegion::Rect(viewport_rect),
                        ),
                        hook: Hook::ScrollFrame,
                    })
                } else {
                    None
                }
            }
            EditorAction::ScaleChange => Some(ViewportDamage {
                event: DamageEvent::with_region(
                    CompositionLayerId::TextAndDecoration,
                    DamageClassId::ViewportResizeOrScaleChange,
                    DamageRegion::Rect(viewport_rect),
                ),
                hook: Hook::MultiMonitorScaleChange,
            }),
        }
    }

    /// Moves the caret according to `movement`, clamping to `line_graphemes`.
    pub fn move_caret(
        &mut self,
        movement: CaretMove,
        line_graphemes: &[usize],
        extend_selection: bool,
    ) -> bool {
        let before = self.caret;
        if !extend_selection {
            self.selection_anchor = None;
        } else if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.caret);
        }

        let line_count = line_graphemes.len().max(1);
        let mut line = self.caret.line.min(line_count.saturating_sub(1));
        let max_col = line_graphemes.get(line).copied().unwrap_or(0);
        let mut col = self.caret.grapheme.min(max_col);

        match movement {
            CaretMove::Left => {
                if col > 0 {
                    col = col.saturating_sub(1);
                } else if line > 0 {
                    line = line.saturating_sub(1);
                    col = line_graphemes.get(line).copied().unwrap_or(0);
                }
                self.preferred_grapheme_column = col;
            }
            CaretMove::Right => {
                if col < max_col {
                    col = col.saturating_add(1);
                } else if line + 1 < line_count {
                    line = line.saturating_add(1);
                    col = 0;
                }
                self.preferred_grapheme_column = col;
            }
            CaretMove::Up => {
                if line > 0 {
                    line = line.saturating_sub(1);
                }
                let max_col = line_graphemes.get(line).copied().unwrap_or(0);
                col = self.preferred_grapheme_column.min(max_col);
            }
            CaretMove::Down => {
                if line + 1 < line_count {
                    line = line.saturating_add(1);
                }
                let max_col = line_graphemes.get(line).copied().unwrap_or(0);
                col = self.preferred_grapheme_column.min(max_col);
            }
            CaretMove::LineStart => {
                col = 0;
                self.preferred_grapheme_column = col;
            }
            CaretMove::LineEnd => {
                col = max_col;
                self.preferred_grapheme_column = col;
            }
            CaretMove::PageUp => {
                line = line.saturating_sub(10);
                let max_col = line_graphemes.get(line).copied().unwrap_or(0);
                col = self.preferred_grapheme_column.min(max_col);
            }
            CaretMove::PageDown => {
                line = (line + 10).min(line_count.saturating_sub(1));
                let max_col = line_graphemes.get(line).copied().unwrap_or(0);
                col = self.preferred_grapheme_column.min(max_col);
            }
        }

        self.caret = TextPoint { line, grapheme: col };
        before != self.caret
    }

    /// Applies a selection delta.
    pub fn apply_selection_delta(&mut self, delta: SelectionDelta, line_graphemes: &[usize]) {
        match delta {
            SelectionDelta::Cleared => {
                self.selection_anchor = None;
            }
            SelectionDelta::ExtendedLeft => {
                let _ = self.move_caret(CaretMove::Left, line_graphemes, true);
            }
            SelectionDelta::ExtendedRight => {
                let _ = self.move_caret(CaretMove::Right, line_graphemes, true);
            }
        }
    }

    /// Replaces the current IME composition state.
    pub fn set_ime_composition(&mut self, composition: Option<ImeComposition>) {
        self.ime_composition = composition;
    }

    /// Returns an ordered `(start, end)` selection range when one exists.
    pub fn selection_range(&self) -> Option<(TextPoint, TextPoint)> {
        let anchor = self.selection_anchor?;
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

impl Default for EditorViewport {
    fn default() -> Self {
        Self::new()
    }
}
